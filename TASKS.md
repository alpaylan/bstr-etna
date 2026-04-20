# bstr — ETNA Tasks

Total tasks: 24

ETNA tasks are **mutation/property/witness triplets**. Each row below is one runnable task. The `<PropertyKey>` token in the command column uses the PascalCase key recognised by `src/bin/etna.rs`; passing `All` runs every property for the named framework in a single invocation.

## Property keys

| Property | PropertyKey |
|----------|-------------|
| `property_splitn_bounded` | `SplitnBounded` |
| `property_cow_partialeq_bstr` | `CowPartialEqBstr` |
| `property_debug_valid_fffd_preserved` | `DebugValidFfFdPreserved` |
| `property_debug_ascii_control_hex` | `DebugAsciiControlHex` |
| `property_debug_hex_lowercase` | `DebugHexLowercase` |

## Task Index

| Task | Variant | Framework | Property | Witness | Command |
|------|---------|-----------|----------|---------|---------|
| 001 | `splitn_trailing_empty_aed424a_1` | proptest | `property_splitn_bounded` | `witness_splitn_bounded_case_no_needle_in_haystack` | `cargo run --release --bin etna -- proptest SplitnBounded` |
| 002 | `splitn_trailing_empty_aed424a_1` | quickcheck | `property_splitn_bounded` | `witness_splitn_bounded_case_no_needle_in_haystack` | `cargo run --release --bin etna -- quickcheck SplitnBounded` |
| 003 | `splitn_trailing_empty_aed424a_1` | crabcheck | `property_splitn_bounded` | `witness_splitn_bounded_case_no_needle_in_haystack` | `cargo run --release --bin etna -- crabcheck SplitnBounded` |
| 004 | `splitn_trailing_empty_aed424a_1` | hegel | `property_splitn_bounded` | `witness_splitn_bounded_case_no_needle_in_haystack` | `cargo run --release --bin etna -- hegel SplitnBounded` |
| 005 | `cow_partialeq_self_compare_b2111b6_1` | proptest | `property_cow_partialeq_bstr` | `witness_cow_partialeq_bstr_case_distinct_bytes` | `cargo run --release --bin etna -- proptest CowPartialEqBstr` |
| 006 | `cow_partialeq_self_compare_b2111b6_1` | quickcheck | `property_cow_partialeq_bstr` | `witness_cow_partialeq_bstr_case_distinct_bytes` | `cargo run --release --bin etna -- quickcheck CowPartialEqBstr` |
| 007 | `cow_partialeq_self_compare_b2111b6_1` | crabcheck | `property_cow_partialeq_bstr` | `witness_cow_partialeq_bstr_case_distinct_bytes` | `cargo run --release --bin etna -- crabcheck CowPartialEqBstr` |
| 008 | `cow_partialeq_self_compare_b2111b6_1` | hegel | `property_cow_partialeq_bstr` | `witness_cow_partialeq_bstr_case_distinct_bytes` | `cargo run --release --bin etna -- hegel CowPartialEqBstr` |
| 009 | `debug_fffd_not_escaped_eafb495_1` | proptest | `property_debug_valid_fffd_preserved` | `witness_debug_valid_fffd_preserved_case_bare` | `cargo run --release --bin etna -- proptest DebugValidFfFdPreserved` |
| 010 | `debug_fffd_not_escaped_eafb495_1` | quickcheck | `property_debug_valid_fffd_preserved` | `witness_debug_valid_fffd_preserved_case_bare` | `cargo run --release --bin etna -- quickcheck DebugValidFfFdPreserved` |
| 011 | `debug_fffd_not_escaped_eafb495_1` | crabcheck | `property_debug_valid_fffd_preserved` | `witness_debug_valid_fffd_preserved_case_bare` | `cargo run --release --bin etna -- crabcheck DebugValidFfFdPreserved` |
| 012 | `debug_fffd_not_escaped_eafb495_1` | hegel | `property_debug_valid_fffd_preserved` | `witness_debug_valid_fffd_preserved_case_bare` | `cargo run --release --bin etna -- hegel DebugValidFfFdPreserved` |
| 013 | `debug_non_ascii_control_unicode_escape_8e2041e_1` | proptest | `property_debug_ascii_control_hex` | `witness_debug_ascii_control_hex_case_low_stx` | `cargo run --release --bin etna -- proptest DebugAsciiControlHex` |
| 014 | `debug_non_ascii_control_unicode_escape_8e2041e_1` | quickcheck | `property_debug_ascii_control_hex` | `witness_debug_ascii_control_hex_case_low_stx` | `cargo run --release --bin etna -- quickcheck DebugAsciiControlHex` |
| 015 | `debug_non_ascii_control_unicode_escape_8e2041e_1` | crabcheck | `property_debug_ascii_control_hex` | `witness_debug_ascii_control_hex_case_low_stx` | `cargo run --release --bin etna -- crabcheck DebugAsciiControlHex` |
| 016 | `debug_non_ascii_control_unicode_escape_8e2041e_1` | hegel | `property_debug_ascii_control_hex` | `witness_debug_ascii_control_hex_case_low_stx` | `cargo run --release --bin etna -- hegel DebugAsciiControlHex` |
| 017 | `debug_control_chars_x1a_732fc99_1` | proptest | `property_debug_ascii_control_hex` | `witness_debug_ascii_control_hex_case_high_fs` | `cargo run --release --bin etna -- proptest DebugAsciiControlHex` |
| 018 | `debug_control_chars_x1a_732fc99_1` | quickcheck | `property_debug_ascii_control_hex` | `witness_debug_ascii_control_hex_case_high_fs` | `cargo run --release --bin etna -- quickcheck DebugAsciiControlHex` |
| 019 | `debug_control_chars_x1a_732fc99_1` | crabcheck | `property_debug_ascii_control_hex` | `witness_debug_ascii_control_hex_case_high_fs` | `cargo run --release --bin etna -- crabcheck DebugAsciiControlHex` |
| 020 | `debug_control_chars_x1a_732fc99_1` | hegel | `property_debug_ascii_control_hex` | `witness_debug_ascii_control_hex_case_high_fs` | `cargo run --release --bin etna -- hegel DebugAsciiControlHex` |
| 021 | `debug_hex_uppercase_af99a6e_1` | proptest | `property_debug_hex_lowercase` | `witness_debug_hex_lowercase_case_ff` | `cargo run --release --bin etna -- proptest DebugHexLowercase` |
| 022 | `debug_hex_uppercase_af99a6e_1` | quickcheck | `property_debug_hex_lowercase` | `witness_debug_hex_lowercase_case_ff` | `cargo run --release --bin etna -- quickcheck DebugHexLowercase` |
| 023 | `debug_hex_uppercase_af99a6e_1` | crabcheck | `property_debug_hex_lowercase` | `witness_debug_hex_lowercase_case_ff` | `cargo run --release --bin etna -- crabcheck DebugHexLowercase` |
| 024 | `debug_hex_uppercase_af99a6e_1` | hegel | `property_debug_hex_lowercase` | `witness_debug_hex_lowercase_case_ff` | `cargo run --release --bin etna -- hegel DebugHexLowercase` |

## Witness catalog

Each witness is a deterministic concrete test. Base build: passes. Variant-active build: fails.

- `witness_splitn_bounded_case_no_needle_in_haystack` — `property_splitn_bounded(b"ab", b":", 2)` → `Pass` on base. Limit (2) exceeds the number of splits (1); the buggy iterator emits a spurious empty trailing slice so `count()` is 2 instead of 1.
- `witness_splitn_bounded_case_needle_once_extra_limit` — `property_splitn_bounded(b"a:b", b":", 3)` → `Pass` on base. Limit (3) exceeds the number of splits (2); when `count == limit` the buggy iterator returns `haystack[haystack.len()..]` = `b""` as a spurious third element.
- `witness_cow_partialeq_bstr_case_distinct_bytes` — `property_cow_partialeq_bstr(b"hello", b"goodbye")` → `Pass` on base. The buggy `Cow == &BStr` impl compares the RHS with itself and returns `true` even for unequal inputs.
- `witness_cow_partialeq_bstr_case_distinct_lengths` — `property_cow_partialeq_bstr(b"abc", b"ab")` → `Pass` on base. Same buggy self-compare; different length makes the failure visible under any comparison order.
- `witness_debug_valid_fffd_preserved_case_bare` — `property_debug_valid_fffd_preserved([], [])` → `Pass` on base. Bare valid-UTF-8 FFFD; the buggy branch rewrites to `\xef\xbf\xbd` instead of preserving the raw character.
- `witness_debug_valid_fffd_preserved_case_sandwiched` — `property_debug_valid_fffd_preserved([0xFF], [0xFF])` → `Pass` on base. FFFD surrounded by invalid UTF-8 to keep the surrounding bytes on the escape path and only test the middle codepoint.
- `witness_debug_ascii_control_hex_case_low_stx` — `property_debug_ascii_control_hex(0x02)` → `Pass` on base. Byte 0x02 is in `0x01..=0x19` and must use `\x02`. The 8e2041e-reverted variant falls straight through to `escape_debug`, emitting `\u{2}`.
- `witness_debug_ascii_control_hex_case_high_fs` — `property_debug_ascii_control_hex(0x1c)` → `Pass` on base. Byte 0x1c is in the 732fc99-extended range `0x1a..=0x1f`; the narrowed variant prints `\u{1c}` instead of `\x1c`.
- `witness_debug_hex_lowercase_case_ff` — `property_debug_hex_lowercase(0xff)` → `Pass` on base. The FFFD-else branch of a single invalid byte must produce lowercase `\xff`. The uppercase-hex variant emits `\xFF`.
- `witness_debug_hex_lowercase_case_ed` — `property_debug_hex_lowercase(0xed)` → `Pass` on base. `0xed` is an invalid UTF-8 lead-only byte, so like 0xff it reaches the FFFD-else branch directly; lowercase vs uppercase is observable independent of digit content.
