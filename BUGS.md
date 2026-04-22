# bstr — Injected Bugs

Total mutations: 6

## Bug Index

| # | Variant | Name | Location | Injection | Fix Commit |
|---|---------|------|----------|-----------|------------|
| 1 | `cow_partialeq_self_compare_b2111b6_1` | `cow_partialeq_self_compare` | `src/impls.rs` | `patch` | `b2111b6bbf2c9a819fb1338aa81bd099874106a1` |
| 2 | `debug_control_chars_x1a_732fc99_1` | `debug_control_chars_x1a` | `src/impls.rs` | `patch` | `732fc99f3844d88dc40f33d95a7bc8f3f6bd2e5b` |
| 3 | `debug_fffd_not_escaped_eafb495_1` | `debug_fffd_not_escaped` | `src/impls.rs` | `patch` | `eafb4951c651c4b4eab94621c259f80b217803ee` |
| 4 | `debug_hex_uppercase_af99a6e_1` | `debug_hex_uppercase` | `src/impls.rs` | `patch` | `af99a6ecb4723d0ea03982797a1becd8437d3f7d` |
| 5 | `debug_non_ascii_control_unicode_escape_8e2041e_1` | `debug_non_ascii_control_unicode_escape` | `src/impls.rs` | `patch` | `8e2041ed5481078f25635dd7989a96abd87721ce` |
| 6 | `splitn_trailing_empty_aed424a_1` | `splitn_trailing_empty` | `src/ext_slice.rs` | `patch` | `aed424a778f43373824232e242e4f7894ba221f1` |

## Property Mapping

| Variant | Property | Witness(es) |
|---------|----------|-------------|
| `cow_partialeq_self_compare_b2111b6_1` | `CowPartialeqBstr` | `witness_cow_partialeq_bstr_case_distinct_bytes`, `witness_cow_partialeq_bstr_case_distinct_lengths` |
| `debug_control_chars_x1a_732fc99_1` | `DebugAsciiControlHex` | `witness_debug_ascii_control_hex_case_high_fs` |
| `debug_fffd_not_escaped_eafb495_1` | `DebugValidFffdPreserved` | `witness_debug_valid_fffd_preserved_case_bare`, `witness_debug_valid_fffd_preserved_case_sandwiched` |
| `debug_hex_uppercase_af99a6e_1` | `DebugHexLowercase` | `witness_debug_hex_lowercase_case_ff`, `witness_debug_hex_lowercase_case_ed` |
| `debug_non_ascii_control_unicode_escape_8e2041e_1` | `DebugAsciiControlHex` | `witness_debug_ascii_control_hex_case_low_stx` |
| `splitn_trailing_empty_aed424a_1` | `SplitnBounded` | `witness_splitn_bounded_case_no_needle_in_haystack`, `witness_splitn_bounded_case_needle_once_extra_limit` |

## Framework Coverage

| Property | proptest | quickcheck | crabcheck | hegel |
|----------|---------:|-----------:|----------:|------:|
| `CowPartialeqBstr` | ✓ | ✓ | ✓ | ✓ |
| `DebugAsciiControlHex` | ✓ | ✓ | ✓ | ✓ |
| `DebugValidFffdPreserved` | ✓ | ✓ | ✓ | ✓ |
| `DebugHexLowercase` | ✓ | ✓ | ✓ | ✓ |
| `SplitnBounded` | ✓ | ✓ | ✓ | ✓ |

## Bug Details

### 1. cow_partialeq_self_compare

- **Variant**: `cow_partialeq_self_compare_b2111b6_1`
- **Location**: `src/impls.rs`
- **Property**: `CowPartialeqBstr`
- **Witness(es)**:
  - `witness_cow_partialeq_bstr_case_distinct_bytes`
  - `witness_cow_partialeq_bstr_case_distinct_lengths`
- **Source**: impl: fix cow partialeq impl
  > One arm of the `impl_partial_eq_cow!` macro compared the right-hand operand with itself instead of with the left, so `cow == &bstr` (and the mirrored direction) returned `true` for every pair. The fix swaps the wrong argument for `this` so both directions actually compare against the caller's bytes.
- **Fix commit**: `b2111b6bbf2c9a819fb1338aa81bd099874106a1` — impl: fix cow partialeq impl
- **Invariant violated**: `&BStr == Cow<BStr>` and `Cow<BStr> == &BStr` must both agree with underlying byte equality, in both directions.
- **How the mutation triggers**: The buggy second impl of the macro expanded to `PartialEq::eq(this, other.as_bytes())` against the wrong operand, effectively comparing `other` with itself. So `cow == &bstr` would always return `true` regardless of content. The witness compares distinct byte strings and expects `false` via either direction of the macro-generated `impl`s.

### 2. debug_control_chars_x1a

- **Variant**: `debug_control_chars_x1a_732fc99_1`
- **Location**: `src/impls.rs`
- **Property**: `DebugAsciiControlHex`
- **Witness(es)**:
  - `witness_debug_ascii_control_hex_case_high_fs`
- **Source**: impl: fix formatting of control characters \\x1a through \\x1f in Debug impl
  > Following 8e2041e, the ASCII-control escape arm in `Debug for BStr` was written as two disjoint ranges `0x01..=0x19` and `0x20..=0x7f`, accidentally omitting `0x1a..=0x1f`. Those bytes fell through to `escape_debug` and printed as `\u{1a}..\u{1f}`. The fix merges the ranges into `'\x01'..='\x7f'`.
- **Fix commit**: `732fc99f3844d88dc40f33d95a7bc8f3f6bd2e5b` — impl: fix formatting of control characters \\x1a through \\x1f in Debug impl
- **Invariant violated**: Same as variant 4, for the 0x1a..=0x1f subrange.
- **How the mutation triggers**: After 8e2041e the escape branch covered `0x01..=0x19` and `0x20..=0x7f` but omitted `0x1a..=0x1f` (a Rust range subtlety around `SUB` / `ESC` / `FS`..`US`); those bytes still fell through to `escape_debug` and printed `\u{1a}..\u{1f}` instead of `\x1a..\x1f`. The 732fc99 fix extends the range to `'\x01'..='\x7f'`. The mutation narrows it back.

### 3. debug_fffd_not_escaped

- **Variant**: `debug_fffd_not_escaped_eafb495_1`
- **Location**: `src/impls.rs`
- **Property**: `DebugValidFffdPreserved`
- **Witness(es)**:
  - `witness_debug_valid_fffd_preserved_case_bare`
  - `witness_debug_valid_fffd_preserved_case_sandwiched`
- **Source**: impl: fix replacement codepoint handling in Debug impl
  > `Debug for BStr` unconditionally rewrote the three bytes of a well-formed U+FFFD (`0xEF 0xBF 0xBD`) as `\xef\xbf\xbd`, erasing valid replacement characters from the output. The fix special-cases the exact byte sequence and renders it via `ch.escape_debug()` so the codepoint survives verbatim.
- **Fix commit**: `eafb4951c651c4b4eab94621c259f80b217803ee` — impl: fix replacement codepoint handling in Debug impl
- **Invariant violated**: Debug output of a `BStr` containing the valid UTF-8 encoding of `U+FFFD` (`0xEF 0xBF 0xBD`) must render the literal replacement character, not three `\xNN` escapes.
- **How the mutation triggers**: The fix checks `if bytes == b"\xEF\xBF\xBD"` and, when true, renders `ch.escape_debug()` so the character survives verbatim. The buggy code unconditionally rewrites the three FFFD bytes as `\xef\xbf\xbd`, so a well-formed FFFD in the input never appears literally in the output.

### 4. debug_hex_uppercase

- **Variant**: `debug_hex_uppercase_af99a6e_1`
- **Location**: `src/impls.rs`
- **Property**: `DebugHexLowercase`
- **Witness(es)**:
  - `witness_debug_hex_lowercase_case_ff`
  - `witness_debug_hex_lowercase_case_ed`
- **Source**: impl: fix discrepancy in upper/lower case in `impl fmt::Debug for BStr`
  > The invalid-UTF-8 branch of `Debug for BStr` formatted each byte with `{:02X}` (uppercase), diverging from the lowercase hex the rest of the impl (and the analogous `str` debug output) uses. The fix is a one-character change to `{:02x}`.
- **Fix commit**: `af99a6ecb4723d0ea03982797a1becd8437d3f7d` — impl: fix discrepancy in upper/lower case in `impl fmt::Debug for BStr`
- **Invariant violated**: Invalid UTF-8 bytes rendered through the FFFD-else branch must use lowercase hex in the `\xNN` escape.
- **How the mutation triggers**: The pre-af99a6e code wrote `write!(f, "\\x{:02X}", b)?` (uppercase). The fix is the one-character change to `{:02x}`. The witness formats byte `0xff` and asserts the rendering is `"\xff"`, not `"\xFF"`.

### 5. debug_non_ascii_control_unicode_escape

- **Variant**: `debug_non_ascii_control_unicode_escape_8e2041e_1`
- **Location**: `src/impls.rs`
- **Property**: `DebugAsciiControlHex`
- **Witness(es)**:
  - `witness_debug_ascii_control_hex_case_low_stx`
- **Source**: impl: improve Debug impl for BStr
  > Prior to this commit, `Debug for BStr` only special-cased `U+FFFD` and delegated everything else to `ch.escape_debug()`, which renders ASCII control bytes as `\u{NN}` instead of the shorter `\xNN`. The fix introduces dedicated arms for `\0`, `\x01..=\x7f`, and the FFFD case so control bytes use the hex-escape form.
- **Fix commit**: `8e2041ed5481078f25635dd7989a96abd87721ce` — impl: improve Debug impl for BStr
- **Invariant violated**: Debug output of any ASCII control byte (0x01..=0x1f or 0x7f, excluding `\0`, `\t`, `\n`, `\r`) must use `\xNN` escapes.
- **How the mutation triggers**: The fix introduces per-byte arms (`'\0' => "\\0"`, `'\x01'..='\x7f' => escape_ascii()`, `'\u{FFFD}' => ...`). The pre-8e2041e buggy code only special-cased FFFD and fell through to `ch.escape_debug()` for everything else, so byte 0x02 prints as `\u{2}` instead of `\x02`.

### 6. splitn_trailing_empty

- **Variant**: `splitn_trailing_empty_aed424a_1`
- **Location**: `src/ext_slice.rs`
- **Property**: `SplitnBounded`
- **Witness(es)**:
  - `witness_splitn_bounded_case_no_needle_in_haystack`
  - `witness_splitn_bounded_case_needle_once_extra_limit`
- **Source**: api: fix splitn_str and rsplitn_str
  > `SplitN::next` short-circuited on `count == limit` without checking whether the underlying split had already produced its final chunk, so `splitn_str(n, needle)` emitted a spurious trailing empty slice whenever the haystack contained fewer than `n-1` needle occurrences. The fix gates the early return on `self.split.done` as well.
- **Fix commit**: `aed424a778f43373824232e242e4f7894ba221f1` — api: fix splitn_str and rsplitn_str
- **Invariant violated**: `haystack.splitn_str(n, needle).count() == min(n, haystack.split_str(needle).count())` (and the same for `rsplitn_str`).
- **How the mutation triggers**: The fix adds `|| self.split.done` to the `count == limit` short-circuit so the splitn iterator stops emitting once the underlying split is exhausted. The buggy version still returns `haystack[haystack.len()..]` (an empty slice) as a spurious trailing element when the limit exceeds the number of matches — so `splitn_str(2, ":")` on `b"ab"` yields `["ab", ""]` instead of `["ab"]`.
