//! Fault-localization integration tests for bstr.
//!
//! One `#[test]` per property in src/bin/etna-faultloc.rs's dispatch.

use bstr::etna::{
    property_cow_partialeq_bstr, property_debug_ascii_control_hex,
    property_debug_hex_lowercase, property_debug_valid_fffd_preserved,
    property_splitn_bounded, PropertyResult,
};
use crabcheck::quickcheck::{Arbitrary, Mutate};
use rand::Rng;
use std::fmt;

#[derive(Clone)]
struct Bytes(Vec<u8>);
impl fmt::Debug for Bytes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

#[derive(Clone)]
struct TwoBytes {
    a: Vec<u8>,
    b: Vec<u8>,
}
impl fmt::Debug for TwoBytes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "a={:?} b={:?}", self.a, self.b)
    }
}

#[derive(Clone)]
struct SplitnInput {
    haystack: Vec<u8>,
    needle: Vec<u8>,
    n: u32,
}
impl fmt::Debug for SplitnInput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "h={:?} n={:?} k={}", self.haystack, self.needle, self.n)
    }
}

#[derive(Clone, Copy)]
struct ByteWrap(u8);
impl fmt::Debug for ByteWrap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "0x{:02x}", self.0)
    }
}

fn gen_bytes<R: Rng>(rng: &mut R, max_len: usize) -> Vec<u8> {
    let len = rng.random_range(0usize..=max_len);
    (0..len)
        .map(|_| rng.random_range(0u16..=255) as u8)
        .collect()
}

impl<R: Rng> Arbitrary<R> for Bytes {
    fn generate(rng: &mut R, _n: usize) -> Self {
        Bytes(gen_bytes(rng, 32))
    }
}
impl<R: Rng> Arbitrary<R> for TwoBytes {
    fn generate(rng: &mut R, _n: usize) -> Self {
        TwoBytes {
            a: gen_bytes(rng, 16),
            b: gen_bytes(rng, 16),
        }
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
    fn generate(rng: &mut R, _n: usize) -> Self {
        ByteWrap(rng.random())
    }
}

fn mutate_bytes<R: Rng>(rng: &mut R, v: &[u8], max_len: usize) -> Vec<u8> {
    let mut out = v.to_vec();
    match rng.random_range(0u8..3) {
        0 if !out.is_empty() => {
            let i = rng.random_range(0..out.len());
            let bit = rng.random_range(0u32..8);
            out[i] ^= 1u8 << bit;
        }
        1 if out.len() < max_len => out.push(rng.random_range(0u16..=255) as u8),
        _ if !out.is_empty() => {
            out.pop();
        }
        _ => {}
    }
    out
}

impl<R: Rng> Mutate<R> for Bytes {
    fn mutate(&self, rng: &mut R, _n: usize) -> Self {
        Bytes(mutate_bytes(rng, &self.0, 32))
    }
}
impl<R: Rng> Mutate<R> for TwoBytes {
    fn mutate(&self, rng: &mut R, _n: usize) -> Self {
        let mut out = self.clone();
        if rng.random_bool(0.5) {
            out.a = mutate_bytes(rng, &out.a, 16);
        } else {
            out.b = mutate_bytes(rng, &out.b, 16);
        }
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

fn property_splitn_bounded_test(i: SplitnInput) -> Option<bool> {
    to_opt(property_splitn_bounded(i.haystack, i.needle, i.n))
}

fn property_cow_partialeq_bstr_test(i: TwoBytes) -> Option<bool> {
    to_opt(property_cow_partialeq_bstr(i.a, i.b))
}

fn property_debug_valid_fffd_preserved_test(i: TwoBytes) -> Option<bool> {
    to_opt(property_debug_valid_fffd_preserved(i.a, i.b))
}

fn property_debug_ascii_control_hex_test(i: ByteWrap) -> Option<bool> {
    to_opt(property_debug_ascii_control_hex(i.0))
}

fn property_debug_hex_lowercase_test(i: ByteWrap) -> Option<bool> {
    to_opt(property_debug_hex_lowercase(i.0))
}

// Manual JSON emitter (we don't depend on serde_json in dev-deps).
fn json_escape(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 2);
    out.push('"');
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if (c as u32) < 0x20 => out.push_str(&format!("\\u{:04x}", c as u32)),
            c => out.push(c),
        }
    }
    out.push('"');
    out
}

fn json_f64(x: f64) -> String {
    if x.is_finite() {
        format!("{}", x)
    } else {
        "null".to_string()
    }
}

fn emit_locate_json(r: &crabcheck::profiling::LocateResult) {
    use crabcheck::quickcheck::ResultStatus;
    let status = match &r.run.status {
        ResultStatus::Failed { .. } => "Failed",
        ResultStatus::Finished => "Finished",
        ResultStatus::GaveUp => "GaveUp",
        ResultStatus::TimedOut => "TimedOut",
        ResultStatus::Aborted { .. } => "Aborted",
    };
    let top = if let Some(s) = r.top() {
        format!(
            "{{\"rank\":{},\"file\":{},\"function\":{},\"start_line\":{},\"end_line\":{},\"ochiai\":{},\"delta\":{},\"panic_overlap\":{},\"confidence\":{},\"confidence_rule\":{}}}",
            s.rank,
            json_escape(&s.region.file),
            json_escape(&s.region.function),
            s.region.start_line,
            s.region.end_line,
            json_f64(s.region.suspiciousness.ochiai as f64),
            json_f64(s.region.delta as f64),
            s.panic_overlap,
            json_escape(&format!("{}", s.confidence)),
            json_escape(s.confidence_rule),
        )
    } else {
        "null".to_string()
    };
    let top_5_items: Vec<String> = r
        .suspects
        .iter()
        .take(5)
        .map(|s| {
            format!(
                "{{\"rank\":{},\"file\":{},\"function\":{},\"start_line\":{},\"end_line\":{},\"confidence\":{},\"confidence_rule\":{},\"panic_overlap\":{}}}",
                s.rank,
                json_escape(&s.region.file),
                json_escape(&s.region.function),
                s.region.start_line,
                s.region.end_line,
                json_escape(&format!("{}", s.confidence)),
                json_escape(s.confidence_rule),
                s.panic_overlap,
            )
        })
        .collect();
    let top_5 = format!("[{}]", top_5_items.join(","));
    let diag_items: Vec<String> = r.diagnostics.iter().map(|d| json_escape(d.tag())).collect();
    let diags = format!("[{}]", diag_items.join(","));
    let out = format!(
        "{{\"status\":{},\"passed\":{},\"discarded\":{},\"n_panics\":{},\"n_suspects\":{},\"top\":{},\"top_5\":{},\"diagnostics\":{}}}",
        json_escape(status),
        r.run.passed,
        r.run.discarded,
        r.n_panics,
        r.suspects.len(),
        top,
        top_5,
        diags,
    );
    println!("@@LOCATE@@ {}", out);
}

#[test]
fn locate_splitn_bounded() {
    let report = crabcheck::quickcheck_with_locate!(property_splitn_bounded_test, "bstr");
    eprintln!("{report}");
    emit_locate_json(&report);
}

#[test]
fn locate_cow_partialeq_bstr() {
    let report = crabcheck::quickcheck_with_locate!(property_cow_partialeq_bstr_test, "bstr");
    eprintln!("{report}");
    emit_locate_json(&report);
}

#[test]
fn locate_debug_valid_fffd_preserved() {
    let report =
        crabcheck::quickcheck_with_locate!(property_debug_valid_fffd_preserved_test, "bstr");
    eprintln!("{report}");
    emit_locate_json(&report);
}

#[test]
fn locate_debug_ascii_control_hex() {
    let report =
        crabcheck::quickcheck_with_locate!(property_debug_ascii_control_hex_test, "bstr");
    eprintln!("{report}");
    emit_locate_json(&report);
}

#[test]
fn locate_debug_hex_lowercase() {
    let report = crabcheck::quickcheck_with_locate!(property_debug_hex_lowercase_test, "bstr");
    eprintln!("{report}");
    emit_locate_json(&report);
}
