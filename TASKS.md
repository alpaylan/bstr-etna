# bstr — ETNA Tasks

Total tasks: 24

## Task Index

| Task | Variant | Framework | Property | Witness |
|------|---------|-----------|----------|---------|
| 001 | `cow_partialeq_self_compare_b2111b6_1` | proptest | `CowPartialeqBstr` | `witness_cow_partialeq_bstr_case_distinct_bytes` |
| 002 | `cow_partialeq_self_compare_b2111b6_1` | quickcheck | `CowPartialeqBstr` | `witness_cow_partialeq_bstr_case_distinct_bytes` |
| 003 | `cow_partialeq_self_compare_b2111b6_1` | crabcheck | `CowPartialeqBstr` | `witness_cow_partialeq_bstr_case_distinct_bytes` |
| 004 | `cow_partialeq_self_compare_b2111b6_1` | hegel | `CowPartialeqBstr` | `witness_cow_partialeq_bstr_case_distinct_bytes` |
| 005 | `debug_control_chars_x1a_732fc99_1` | proptest | `DebugAsciiControlHex` | `witness_debug_ascii_control_hex_case_high_fs` |
| 006 | `debug_control_chars_x1a_732fc99_1` | quickcheck | `DebugAsciiControlHex` | `witness_debug_ascii_control_hex_case_high_fs` |
| 007 | `debug_control_chars_x1a_732fc99_1` | crabcheck | `DebugAsciiControlHex` | `witness_debug_ascii_control_hex_case_high_fs` |
| 008 | `debug_control_chars_x1a_732fc99_1` | hegel | `DebugAsciiControlHex` | `witness_debug_ascii_control_hex_case_high_fs` |
| 009 | `debug_fffd_not_escaped_eafb495_1` | proptest | `DebugValidFffdPreserved` | `witness_debug_valid_fffd_preserved_case_bare` |
| 010 | `debug_fffd_not_escaped_eafb495_1` | quickcheck | `DebugValidFffdPreserved` | `witness_debug_valid_fffd_preserved_case_bare` |
| 011 | `debug_fffd_not_escaped_eafb495_1` | crabcheck | `DebugValidFffdPreserved` | `witness_debug_valid_fffd_preserved_case_bare` |
| 012 | `debug_fffd_not_escaped_eafb495_1` | hegel | `DebugValidFffdPreserved` | `witness_debug_valid_fffd_preserved_case_bare` |
| 013 | `debug_hex_uppercase_af99a6e_1` | proptest | `DebugHexLowercase` | `witness_debug_hex_lowercase_case_ff` |
| 014 | `debug_hex_uppercase_af99a6e_1` | quickcheck | `DebugHexLowercase` | `witness_debug_hex_lowercase_case_ff` |
| 015 | `debug_hex_uppercase_af99a6e_1` | crabcheck | `DebugHexLowercase` | `witness_debug_hex_lowercase_case_ff` |
| 016 | `debug_hex_uppercase_af99a6e_1` | hegel | `DebugHexLowercase` | `witness_debug_hex_lowercase_case_ff` |
| 017 | `debug_non_ascii_control_unicode_escape_8e2041e_1` | proptest | `DebugAsciiControlHex` | `witness_debug_ascii_control_hex_case_low_stx` |
| 018 | `debug_non_ascii_control_unicode_escape_8e2041e_1` | quickcheck | `DebugAsciiControlHex` | `witness_debug_ascii_control_hex_case_low_stx` |
| 019 | `debug_non_ascii_control_unicode_escape_8e2041e_1` | crabcheck | `DebugAsciiControlHex` | `witness_debug_ascii_control_hex_case_low_stx` |
| 020 | `debug_non_ascii_control_unicode_escape_8e2041e_1` | hegel | `DebugAsciiControlHex` | `witness_debug_ascii_control_hex_case_low_stx` |
| 021 | `splitn_trailing_empty_aed424a_1` | proptest | `SplitnBounded` | `witness_splitn_bounded_case_no_needle_in_haystack` |
| 022 | `splitn_trailing_empty_aed424a_1` | quickcheck | `SplitnBounded` | `witness_splitn_bounded_case_no_needle_in_haystack` |
| 023 | `splitn_trailing_empty_aed424a_1` | crabcheck | `SplitnBounded` | `witness_splitn_bounded_case_no_needle_in_haystack` |
| 024 | `splitn_trailing_empty_aed424a_1` | hegel | `SplitnBounded` | `witness_splitn_bounded_case_no_needle_in_haystack` |

## Witness Catalog

- `witness_cow_partialeq_bstr_case_distinct_bytes` — base passes, variant fails
- `witness_cow_partialeq_bstr_case_distinct_lengths` — base passes, variant fails
- `witness_debug_ascii_control_hex_case_high_fs` — base passes, variant fails
- `witness_debug_valid_fffd_preserved_case_bare` — base passes, variant fails
- `witness_debug_valid_fffd_preserved_case_sandwiched` — base passes, variant fails
- `witness_debug_hex_lowercase_case_ff` — base passes, variant fails
- `witness_debug_hex_lowercase_case_ed` — base passes, variant fails
- `witness_debug_ascii_control_hex_case_low_stx` — base passes, variant fails
- `witness_splitn_bounded_case_no_needle_in_haystack` — base passes, variant fails
- `witness_splitn_bounded_case_needle_once_extra_limit` — base passes, variant fails
