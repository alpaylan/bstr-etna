// Deterministic witness tests for ETNA variants.
//
// Each `witness_<name>_case_<tag>` passes on the base commit and fails under
// the corresponding `etna/<variant>` branch. Witnesses call `property_<name>`
// directly with frozen inputs; they do not touch framework machinery (no
// proptest, no quickcheck, no RNG, no clocks).

use bstr::etna::{
    property_cow_partialeq_bstr, property_debug_ascii_control_hex,
    property_debug_hex_lowercase, property_debug_valid_fffd_preserved,
    property_splitn_bounded, PropertyResult,
};

fn expect_pass(r: PropertyResult, what: &str) {
    match r {
        PropertyResult::Pass => (),
        PropertyResult::Fail(m) => panic!("{what}: property failed: {m}"),
        PropertyResult::Discard => panic!("{what}: unexpected discard"),
    }
}

// Variant: splitn_trailing_empty_aed424a_1
//
// Mutation reverts the `|| self.split.done` short-circuit in SplitN /
// SplitNReverse, so `splitn_str(n, needle)` can emit a spurious empty tail
// when the limit exceeds the actual number of splits.
#[test]
fn witness_splitn_bounded_case_no_needle_in_haystack() {
    expect_pass(
        property_splitn_bounded(b"ab".to_vec(), b":".to_vec(), 2),
        "splitn_str(2, \":\") on b\"ab\"",
    );
}

#[test]
fn witness_splitn_bounded_case_needle_once_extra_limit() {
    // Limit 3 on an input with exactly 2 splits: when count reaches 3 the
    // buggy iterator takes the `count == limit` branch and returns
    // haystack[haystack.len()..] = b"" as a spurious trailing empty slice.
    expect_pass(
        property_splitn_bounded(b"a:b".to_vec(), b":".to_vec(), 3),
        "splitn_str(3, \":\") on b\"a:b\"",
    );
}

// Variant: cow_partialeq_self_compare_b2111b6_1
//
// Mutation reverts `impl_partial_eq_cow`'s first impl to compare `other`
// against itself, so `&BStr == Cow<BStr>` returns `true` for any pair.
#[test]
fn witness_cow_partialeq_bstr_case_distinct_bytes() {
    expect_pass(
        property_cow_partialeq_bstr(b"hello".to_vec(), b"goodbye".to_vec()),
        "&BStr(hello) vs Cow(goodbye)",
    );
}

#[test]
fn witness_cow_partialeq_bstr_case_distinct_lengths() {
    expect_pass(
        property_cow_partialeq_bstr(b"abc".to_vec(), b"ab".to_vec()),
        "&BStr(abc) vs Cow(ab)",
    );
}

// Variant: debug_fffd_not_escaped_eafb495_1
//
// Mutation removes the `if bytes == b"\xEF\xBF\xBD"` short-circuit so every
// FFFD character (including those decoded from valid UTF-8) is printed as
// three `\xNN` escapes, losing the literal replacement char.
#[test]
fn witness_debug_valid_fffd_preserved_case_bare() {
    expect_pass(
        property_debug_valid_fffd_preserved(Vec::new(), Vec::new()),
        "Debug of valid FFFD alone",
    );
}

#[test]
fn witness_debug_valid_fffd_preserved_case_sandwiched() {
    expect_pass(
        property_debug_valid_fffd_preserved(b"\xFF".to_vec(), b"\xFF".to_vec()),
        "Debug of 0xFF, FFFD, 0xFF",
    );
}

// Variant: debug_non_ascii_control_unicode_escape_8e2041e_1
//
// Mutation reverts the entire ASCII control handling: everything below FFFD
// goes through `escape_debug`, so byte 0x02 prints as `\u{2}` instead of
// `\x02`.
#[test]
fn witness_debug_ascii_control_hex_case_low_stx() {
    expect_pass(
        property_debug_ascii_control_hex(0x02),
        "Debug of byte 0x02",
    );
}

// Variant: debug_control_chars_x1a_732fc99_1
//
// Mutation narrows the control-char branch to the pre-732fc99 ranges, so
// bytes 0x1a..=0x1f fall through to `escape_debug` and print as
// `\u{1a}..\u{1f}` instead of `\x1a..\x1f`.
#[test]
fn witness_debug_ascii_control_hex_case_high_fs() {
    expect_pass(
        property_debug_ascii_control_hex(0x1c),
        "Debug of byte 0x1c",
    );
}

// Variant: debug_hex_uppercase_af99a6e_1
//
// Mutation reverts the FFFD-else hex format to `{:02X}`, so invalid UTF-8
// bytes print with uppercase hex digits.
#[test]
fn witness_debug_hex_lowercase_case_ff() {
    expect_pass(
        property_debug_hex_lowercase(0xff),
        "Debug of byte 0xff",
    );
}

#[test]
fn witness_debug_hex_lowercase_case_ed() {
    expect_pass(
        property_debug_hex_lowercase(0xed),
        "Debug of byte 0xed",
    );
}
