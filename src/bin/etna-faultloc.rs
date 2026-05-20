use std::fmt;

use bstr::etna::{
    property_cow_partialeq_bstr, property_debug_ascii_control_hex,
    property_debug_hex_lowercase, property_debug_valid_fffd_preserved,
    property_splitn_bounded, PropertyResult,
};
use crabcheck::profiling::quickcheck;
use crabcheck::quickcheck::{Arbitrary, Mutate};
use rand::Rng;

#[derive(Clone)]
struct Bytes(Vec<u8>);
impl fmt::Debug for Bytes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { self.0.fmt(f) }
}

#[derive(Clone)]
struct TwoBytes { a: Vec<u8>, b: Vec<u8> }
impl fmt::Debug for TwoBytes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "a={:?} b={:?}", self.a, self.b)
    }
}

#[derive(Clone)]
struct SplitnInput { haystack: Vec<u8>, needle: Vec<u8>, n: u32 }
impl fmt::Debug for SplitnInput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "h={:?} n={:?} k={}", self.haystack, self.needle, self.n)
    }
}

#[derive(Clone, Copy)]
struct ByteWrap(u8);
impl fmt::Debug for ByteWrap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { write!(f, "0x{:02x}", self.0) }
}

fn gen_bytes<R: Rng>(rng: &mut R, max_len: usize) -> Vec<u8> {
    let len = rng.random_range(0usize..=max_len);
    (0..len).map(|_| rng.random_range(0u16..=255) as u8).collect()
}

impl<R: Rng> Arbitrary<R> for Bytes {
    fn generate(rng: &mut R, _n: usize) -> Self { Bytes(gen_bytes(rng, 32)) }
}
impl<R: Rng> Arbitrary<R> for TwoBytes {
    fn generate(rng: &mut R, _n: usize) -> Self {
        TwoBytes { a: gen_bytes(rng, 16), b: gen_bytes(rng, 16) }
    }
}
impl<R: Rng> Arbitrary<R> for SplitnInput {
    fn generate(rng: &mut R, _n: usize) -> Self {
        SplitnInput {
            haystack: gen_bytes(rng, 32),
            needle: gen_bytes(rng, 4),
            n: rng.random_range(0u32..=8),
        }
    }
}
impl<R: Rng> Arbitrary<R> for ByteWrap {
    fn generate(rng: &mut R, _n: usize) -> Self { ByteWrap(rng.random()) }
}

fn mutate_bytes<R: Rng>(rng: &mut R, v: &[u8], max_len: usize) -> Vec<u8> {
    let mut out = v.to_vec();
    match rng.random_range(0u8..3) {
        0 if !out.is_empty() => {
            let i = rng.random_range(0..out.len());
            let bit = rng.random_range(0u32..8);
            out[i] ^= 1u8 << bit;
        },
        1 if out.len() < max_len => out.push(rng.random_range(0u16..=255) as u8),
        _ if !out.is_empty() => { out.pop(); },
        _ => {},
    }
    out
}

impl<R: Rng> Mutate<R> for Bytes {
    fn mutate(&self, rng: &mut R, _n: usize) -> Self { Bytes(mutate_bytes(rng, &self.0, 32)) }
}
impl<R: Rng> Mutate<R> for TwoBytes {
    fn mutate(&self, rng: &mut R, _n: usize) -> Self {
        let mut out = self.clone();
        if rng.random_bool(0.5) { out.a = mutate_bytes(rng, &out.a, 16); }
        else { out.b = mutate_bytes(rng, &out.b, 16); }
        out
    }
}
impl<R: Rng> Mutate<R> for SplitnInput {
    fn mutate(&self, rng: &mut R, _n: usize) -> Self {
        let mut out = self.clone();
        match rng.random_range(0u8..3) {
            0 => out.haystack = mutate_bytes(rng, &out.haystack, 32),
            1 => out.needle = mutate_bytes(rng, &out.needle, 4),
            _ => out.n = (out.n.wrapping_add(1)) % 9,
        }
        out
    }
}
impl<R: Rng> Mutate<R> for ByteWrap {
    fn mutate(&self, rng: &mut R, _n: usize) -> Self {
        let bit = rng.random_range(0u32..8);
        ByteWrap(self.0 ^ (1u8 << bit))
    }
}

fn to_opt(r: PropertyResult) -> Option<bool> {
    match r {
        PropertyResult::Pass => Some(true),
        PropertyResult::Fail(_) => Some(false),
        PropertyResult::Discard => None,
    }
}


fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    if args.len() < 3 { return; }
    let tool = args[1].as_str();
    let property = args[2].as_str();
    let result = match (tool, property) {
        ("crabcheck", "SplitnBounded") => {
            quickcheck(|i: SplitnInput| {
                to_opt(property_splitn_bounded(i.haystack, i.needle, i.n))
            })
        },
        ("crabcheck", "CowPartialeqBstr") => {
            quickcheck(|i: TwoBytes| {
                to_opt(property_cow_partialeq_bstr(i.a, i.b))
            })
        },
        ("crabcheck", "DebugValidFffdPreserved") => {
            quickcheck(|i: TwoBytes| {
                to_opt(property_debug_valid_fffd_preserved(i.a, i.b))
            })
        },
        ("crabcheck", "DebugAsciiControlHex") => {
            quickcheck(|ByteWrap(b)| to_opt(property_debug_ascii_control_hex(b)))
        },
        ("crabcheck", "DebugHexLowercase") => {
            quickcheck(|ByteWrap(b)| to_opt(property_debug_hex_lowercase(b)))
        },
        _ => panic!("Unknown: {tool} {property}"),
    };
    println!("Result: {:?}", result);
}
