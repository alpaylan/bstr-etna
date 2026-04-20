//! ETNA framework-neutral property functions for bstr.
//!
//! Each `property_<name>` is a pure function taking concrete, owned inputs and
//! returning `PropertyResult`. Framework adapters (proptest/quickcheck/crabcheck/hegel)
//! in `src/bin/etna.rs` and deterministic witness tests in `tests/etna_witnesses.rs`
//! both call these functions directly — there is no re-implementation of the
//! invariant inside any adapter.

#![allow(missing_docs)]

use crate::ByteSlice;
use alloc::borrow::Cow;
use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;

pub enum PropertyResult {
    Pass,
    Fail(String),
    Discard,
}

/// `splitn_str(n, needle).count() == min(n, split_str(needle).count())` and
/// the same holds for `rsplitn_str` vs `split_str` counting. Detects the
/// `aed424a` bug where `splitn_str(2, ":")` on `b"ab"` yielded `["ab", ""]`
/// (count 2) even though the input has no matches and `split_str` yields
/// `["ab"]` (count 1).
pub fn property_splitn_bounded(
    haystack: Vec<u8>,
    needle: Vec<u8>,
    n: u32,
) -> PropertyResult {
    if needle.is_empty() {
        return PropertyResult::Discard;
    }
    let n = n as usize;
    let full = haystack.split_str(&needle).count();
    let expected = core::cmp::min(n, full);
    let forward = haystack.splitn_str(n, &needle).count();
    if forward != expected {
        return PropertyResult::Fail(format!(
            "splitn_str({n}, {:?}) on {:?}: got {forward} items, expected {expected} (split_str count = {full})",
            needle.as_bstr(),
            haystack.as_bstr()
        ));
    }
    let reverse = haystack.rsplitn_str(n, &needle).count();
    if reverse != expected {
        return PropertyResult::Fail(format!(
            "rsplitn_str({n}, {:?}) on {:?}: got {reverse} items, expected {expected} (split_str count = {full})",
            needle.as_bstr(),
            haystack.as_bstr()
        ));
    }
    PropertyResult::Pass
}

/// `&BStr == Cow<BStr>` and `Cow<BStr> == &BStr` must both agree with
/// the underlying byte equality. Detects the `b2111b6` bug where the
/// macro body of `impl PartialEq<&BStr> for Cow<BStr>` compared `other`
/// with itself, so `cow == &bstr` returned `true` for any pair. We test
/// both directions so the witness catches either half of the symmetric
/// pair being broken.
pub fn property_cow_partialeq_bstr(a: Vec<u8>, b: Vec<u8>) -> PropertyResult {
    let bs_a = a.as_bstr();
    let bs_b = b.as_bstr();
    let cow_b: Cow<'_, crate::BStr> = Cow::Borrowed(bs_b);
    let cow_a: Cow<'_, crate::BStr> = Cow::Borrowed(bs_a);
    let expected = a == b;
    let forward = bs_a == cow_b;
    if forward != expected {
        return PropertyResult::Fail(format!(
            "&BStr == Cow<BStr>: {:?} == Cow::Borrowed({:?}) returned {forward}, but bytes equal? {expected}",
            bs_a, bs_b
        ));
    }
    let reverse = cow_a == bs_b;
    if reverse != expected {
        return PropertyResult::Fail(format!(
            "Cow<BStr> == &BStr: Cow::Borrowed({:?}) == {:?} returned {reverse}, but bytes equal? {expected}",
            bs_a, bs_b
        ));
    }
    PropertyResult::Pass
}

/// Debug output of `BStr` containing the valid UTF-8 encoding of `U+FFFD`
/// must emit the literal replacement character, not three `\xNN` escapes.
/// Detects the `eafb495` bug where the FFFD branch always rewrote the
/// three bytes as individual hex escapes.
pub fn property_debug_valid_fffd_preserved(
    prefix: Vec<u8>,
    suffix: Vec<u8>,
) -> PropertyResult {
    let mut input = prefix;
    input.extend_from_slice(b"\xEF\xBF\xBD");
    input.extend_from_slice(&suffix);
    let rendered = format!("{:?}", input.as_bstr());
    let fffd_bytes = b"\xEF\xBF\xBD";
    // The valid FFFD encoding must survive verbatim inside the quoted output.
    if rendered.as_bytes().windows(3).any(|w| w == fffd_bytes) {
        PropertyResult::Pass
    } else {
        PropertyResult::Fail(format!(
            "Debug of {:?} dropped raw FFFD: {rendered:?}",
            input.as_bstr()
        ))
    }
}

/// Debug output of any ASCII control byte (0x01..=0x1f or 0x7f, excluding
/// `\0`, `\t`, `\n`, `\r`) must use `\xNN` escapes, not `\u{NN}` escapes.
/// Detects both `8e2041e` (pre-fix, everything fell through to `escape_debug`)
/// and `732fc99` (pre-fix, the 0x1a..=0x1f slice still fell through).
pub fn property_debug_ascii_control_hex(byte: u8) -> PropertyResult {
    match byte {
        b'\0' | b'\t' | b'\n' | b'\r' => return PropertyResult::Discard,
        0x01..=0x7f => {}
        _ => return PropertyResult::Discard,
    }
    // 0x20..=0x7e are printable — escape_ascii keeps them literal.
    if (0x20..=0x7e).contains(&byte) {
        return PropertyResult::Discard;
    }
    let rendered = format!("{:?}", [byte].as_bstr());
    let expected = format!("\"\\x{:02x}\"", byte);
    if rendered == expected {
        PropertyResult::Pass
    } else {
        PropertyResult::Fail(format!(
            "Debug of [{byte:#04x}] = {rendered:?}, expected {expected:?}"
        ))
    }
}

/// Debug output of an invalid UTF-8 byte must use lowercase hex in its
/// `\xNN` escape. Detects the `af99a6e` bug where the FFFD-else branch
/// wrote `\x{:02X}` (uppercase).
pub fn property_debug_hex_lowercase(byte: u8) -> PropertyResult {
    if byte < 0x80 {
        return PropertyResult::Discard;
    }
    let rendered = format!("{:?}", [byte].as_bstr());
    // The byte is invalid UTF-8 on its own, so BStr emits it through the
    // FFFD-replacement branch as a single `\xNN` escape.
    let expected = format!("\"\\x{:02x}\"", byte);
    if rendered == expected {
        PropertyResult::Pass
    } else {
        PropertyResult::Fail(format!(
            "Debug of [{byte:#04x}] = {rendered:?}, expected {expected:?}"
        ))
    }
}

