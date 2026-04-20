# bstr — Injected Bugs

Total mutations: 6

## Bug Index

| # | Name | Variant | File | Injection | Fix Commit |
|---|------|---------|------|-----------|------------|
| 1 | `splitn_trailing_empty` | `splitn_trailing_empty_aed424a_1` | `patches/splitn_trailing_empty_aed424a_1.patch` | `patch` | `aed424a778f43373824232e242e4f7894ba221f1` |
| 2 | `cow_partialeq_self_compare` | `cow_partialeq_self_compare_b2111b6_1` | `patches/cow_partialeq_self_compare_b2111b6_1.patch` | `patch` | `b2111b6bbf2c9a819fb1338aa81bd099874106a1` |
| 3 | `debug_fffd_not_escaped` | `debug_fffd_not_escaped_eafb495_1` | `patches/debug_fffd_not_escaped_eafb495_1.patch` | `patch` | `eafb4951c651c4b4eab94621c259f80b217803ee` |
| 4 | `debug_non_ascii_control_unicode_escape` | `debug_non_ascii_control_unicode_escape_8e2041e_1` | `patches/debug_non_ascii_control_unicode_escape_8e2041e_1.patch` | `patch` | `8e2041ed5481078f25635dd7989a96abd87721ce` |
| 5 | `debug_control_chars_x1a` | `debug_control_chars_x1a_732fc99_1` | `patches/debug_control_chars_x1a_732fc99_1.patch` | `patch` | `732fc99f3844d88dc40f33d95a7bc8f3f6bd2e5b` |
| 6 | `debug_hex_uppercase` | `debug_hex_uppercase_af99a6e_1` | `patches/debug_hex_uppercase_af99a6e_1.patch` | `patch` | `af99a6ecb4723d0ea03982797a1becd8437d3f7d` |

## Property Mapping

| Variant | Property | Witness(es) |
|---------|----------|-------------|
| `splitn_trailing_empty_aed424a_1` | `property_splitn_bounded` | `witness_splitn_bounded_case_no_needle_in_haystack`, `witness_splitn_bounded_case_needle_once_extra_limit` |
| `cow_partialeq_self_compare_b2111b6_1` | `property_cow_partialeq_bstr` | `witness_cow_partialeq_bstr_case_distinct_bytes`, `witness_cow_partialeq_bstr_case_distinct_lengths` |
| `debug_fffd_not_escaped_eafb495_1` | `property_debug_valid_fffd_preserved` | `witness_debug_valid_fffd_preserved_case_bare`, `witness_debug_valid_fffd_preserved_case_sandwiched` |
| `debug_non_ascii_control_unicode_escape_8e2041e_1` | `property_debug_ascii_control_hex` | `witness_debug_ascii_control_hex_case_low_stx` |
| `debug_control_chars_x1a_732fc99_1` | `property_debug_ascii_control_hex` | `witness_debug_ascii_control_hex_case_high_fs` |
| `debug_hex_uppercase_af99a6e_1` | `property_debug_hex_lowercase` | `witness_debug_hex_lowercase_case_ff`, `witness_debug_hex_lowercase_case_ed` |

## Framework Coverage

| Property | proptest | quickcheck | crabcheck | hegel |
|----------|---------:|-----------:|----------:|------:|
| `property_splitn_bounded` | OK | OK | OK | OK |
| `property_cow_partialeq_bstr` | OK | OK | OK | OK |
| `property_debug_valid_fffd_preserved` | OK | OK | OK | OK |
| `property_debug_ascii_control_hex` | OK | OK | OK | OK |
| `property_debug_hex_lowercase` | OK | OK | OK | OK |

## Bug Details

### 1. splitn_trailing_empty (aed424a_1)
- **Variant**: `splitn_trailing_empty_aed424a_1`
- **Location**: `src/ext_slice.rs`, `SplitN::next` and `SplitNReverse::next`
- **Property**: `property_splitn_bounded`
- **Witnesses**: `witness_splitn_bounded_case_no_needle_in_haystack`, `witness_splitn_bounded_case_needle_once_extra_limit`
- **Fix commit**: `aed424a778f43373824232e242e4f7894ba221f1` — "api: fix splitn_str and rsplitn_str"
- **Invariant violated**: `haystack.splitn_str(n, needle).count() == min(n, haystack.split_str(needle).count())` (and the same for `rsplitn_str`).
- **How the mutation triggers**: The fix adds `|| self.split.done` to the `count == limit` short-circuit so the splitn iterator stops emitting once the underlying split is exhausted. The buggy version still returns `haystack[haystack.len()..]` (an empty slice) as a spurious trailing element when the limit exceeds the number of matches — so `splitn_str(2, ":")` on `b"ab"` yields `["ab", ""]` instead of `["ab"]`.

### 2. cow_partialeq_self_compare (b2111b6_1)
- **Variant**: `cow_partialeq_self_compare_b2111b6_1`
- **Location**: `src/impls.rs`, `impl_partial_eq_cow!` macro second impl
- **Property**: `property_cow_partialeq_bstr`
- **Witnesses**: `witness_cow_partialeq_bstr_case_distinct_bytes`, `witness_cow_partialeq_bstr_case_distinct_lengths`
- **Fix commit**: `b2111b6bbf2c9a819fb1338aa81bd099874106a1` — "impl: fix cow partialeq impl"
- **Invariant violated**: `&BStr == Cow<BStr>` and `Cow<BStr> == &BStr` must both agree with underlying byte equality, in both directions.
- **How the mutation triggers**: The buggy second impl of the macro expanded to `PartialEq::eq(this, other.as_bytes())` against the wrong operand, effectively comparing `other` with itself. So `cow == &bstr` would always return `true` regardless of content. The witness compares distinct byte strings and expects `false` via either direction of the macro-generated `impl`s.

### 3. debug_fffd_not_escaped (eafb495_1)
- **Variant**: `debug_fffd_not_escaped_eafb495_1`
- **Location**: `src/impls.rs`, `impl fmt::Debug for BStr`, `'\u{FFFD}'` arm
- **Property**: `property_debug_valid_fffd_preserved`
- **Witnesses**: `witness_debug_valid_fffd_preserved_case_bare`, `witness_debug_valid_fffd_preserved_case_sandwiched`
- **Fix commit**: `eafb4951c651c4b4eab94621c259f80b217803ee` — "impl: fix replacement codepoint handling in Debug impl"
- **Invariant violated**: Debug output of a `BStr` containing the valid UTF-8 encoding of `U+FFFD` (`0xEF 0xBF 0xBD`) must render the literal replacement character, not three `\xNN` escapes.
- **How the mutation triggers**: The fix checks `if bytes == b"\xEF\xBF\xBD"` and, when true, renders `ch.escape_debug()` so the character survives verbatim. The buggy code unconditionally rewrites the three FFFD bytes as `\xef\xbf\xbd`, so a well-formed FFFD in the input never appears literally in the output.

### 4. debug_non_ascii_control_unicode_escape (8e2041e_1)
- **Variant**: `debug_non_ascii_control_unicode_escape_8e2041e_1`
- **Location**: `src/impls.rs`, `impl fmt::Debug for BStr`, main `match ch` block
- **Property**: `property_debug_ascii_control_hex`
- **Witness**: `witness_debug_ascii_control_hex_case_low_stx`
- **Fix commit**: `8e2041ed5481078f25635dd7989a96abd87721ce` — "impl: improve Debug impl for BStr"
- **Invariant violated**: Debug output of any ASCII control byte (0x01..=0x1f or 0x7f, excluding `\0`, `\t`, `\n`, `\r`) must use `\xNN` escapes.
- **How the mutation triggers**: The fix introduces per-byte arms (`'\0' => "\\0"`, `'\x01'..='\x7f' => escape_ascii()`, `'\u{FFFD}' => ...`). The pre-8e2041e buggy code only special-cased FFFD and fell through to `ch.escape_debug()` for everything else, so byte 0x02 prints as `\u{2}` instead of `\x02`.

### 5. debug_control_chars_x1a (732fc99_1)
- **Variant**: `debug_control_chars_x1a_732fc99_1`
- **Location**: `src/impls.rs`, `impl fmt::Debug for BStr`, ASCII control range
- **Property**: `property_debug_ascii_control_hex`
- **Witness**: `witness_debug_ascii_control_hex_case_high_fs`
- **Fix commit**: `732fc99f3844d88dc40f33d95a7bc8f3f6bd2e5b` — "impl: fix formatting of control characters \\x1a through \\x1f in Debug impl"
- **Invariant violated**: Same as variant 4, for the 0x1a..=0x1f subrange.
- **How the mutation triggers**: After 8e2041e the escape branch covered `0x01..=0x19` and `0x20..=0x7f` but omitted `0x1a..=0x1f` (a Rust range subtlety around `SUB` / `ESC` / `FS`..`US`); those bytes still fell through to `escape_debug` and printed `\u{1a}..\u{1f}` instead of `\x1a..\x1f`. The 732fc99 fix extends the range to `'\x01'..='\x7f'`. The mutation narrows it back.

### 6. debug_hex_uppercase (af99a6e_1)
- **Variant**: `debug_hex_uppercase_af99a6e_1`
- **Location**: `src/impls.rs`, `impl fmt::Debug for BStr`, FFFD-else branch hex format
- **Property**: `property_debug_hex_lowercase`
- **Witnesses**: `witness_debug_hex_lowercase_case_ff`, `witness_debug_hex_lowercase_case_ed`
- **Fix commit**: `af99a6ecb4723d0ea03982797a1becd8437d3f7d` — "impl: fix discrepancy in upper/lower case in `impl fmt::Debug for BStr`"
- **Invariant violated**: Invalid UTF-8 bytes rendered through the FFFD-else branch must use lowercase hex in the `\xNN` escape.
- **How the mutation triggers**: The pre-af99a6e code wrote `write!(f, "\\x{:02X}", b)?` (uppercase). The fix is the one-character change to `{:02x}`. The witness formats byte `0xff` and asserts the rendering is `"\xff"`, not `"\xFF"`.
