# WP3: lossless shared

- Status: every unit of the WP3 inventory is translated, except the four inline
  functions of `src/dsp/lossless_common.h` that the integrator gave to WP11 (see
  the decision below), the `Init` function that only assigns portable pointers
  (ADR 0003), the free-only `VP8LHtreeGroupsFree`, and the SIMD variants
  (wave 4). Each is listed under the decision, not-applicable checks or recorded
  deviations. The release, asan-ubsan and debug oracles agree with the
  translation on every compared case.
- Sub-wave: 1c
- Model: Claude Opus 5.5 (`claude-opus-5-5`)
- Depends on: WP1, WP4
- Probes: `tools/oracle/probes/wp3/`; tests: `tests/wp3_*.rs`

Written by the package session; the integrator aggregates it into
[PORT_STATUS.md](../PORT_STATUS.md). See the package's row in
[WORKPLAN.md](../WORKPLAN.md) for its verification inputs. The inventory
comes from `tools/units.json`, including the function- and type-level
exceptions inside a module.

## Decision: four `lossless_common.h` functions belong to WP11

`src/dsp/lossless_common.h` defines four inline functions whose bodies read
data that `src/dsp/lossless_enc.c` defines: `VP8LFastLog2` and `VP8LFastSLog2`
read `kLog2Table`, `kSLog2Table`, `VP8LFastLog2Slow` and `VP8LFastSLog2Slow`;
`VP8LPrefixEncodeBits` and `VP8LPrefixEncode` read `kPrefixEncodeCode` and
`kPrefixEncodeExtraBitsValue`. Only WP11's units call them. WP3 reported that
neither package could translate them alone, and the integrator decided on
2026-11-06:

- `tools/units.json` has a function-level exception that gives the four
  functions in `src/dsp/lossless_common.rs` to WP11, which defines the tables and
  slow functions in `src/dsp/lossless_enc.rs` and appends the four functions to
  `lossless_common.rs`. The module header names the exception, and
  `docs/status/wp11.md` lists it in its inventory.
- `VP8LPrefixEncodeBitsNoLUT` and `VP8LPrefixEncodeNoLUT` read no table and stay
  with WP3.
- `tools/unit_dependencies.py --check` still reports no undeclared reference;
  the inline functions leave no symbol of their own in the oracle objects.

## Inventory

Every row names what is translated; each item carries an `// upstream:` marker.
Names follow ADR 0001 (`tools/rust_name.py`). No module uses `unsafe`.

| Rust module | Upstream units | Translated |
| --- | --- | --- |
| `src/utils/huffman_utils.rs` | `src/utils/huffman_utils.c`, `src/utils/huffman_utils.h` | yes, except `VP8LHtreeGroupsFree` (deviation 6): `HUFFMAN_TABLE_BITS`, `HUFFMAN_TABLE_MASK`, `LENGTHS_TABLE_BITS`, `LENGTHS_TABLE_MASK`, `HUFFMAN_PACKED_BITS`, `HUFFMAN_PACKED_TABLE_SIZE`, `HuffmanCode`, `HuffmanCode32`, `HuffmanTablesSegment`, `HuffmanTables`, `HTreeGroup`, `MAX_HTREE_GROUPS`, `MAX_CODE_LENGTHS_SIZE`, `SORTED_SIZE_CUTOFF`, `VP8LHtreeGroupsNew`, `GetNextKey`, `ReplicateValue`, `NextTableBitSize`, `BuildHuffmanTable`, `VP8LBuildHuffmanTable` (with the `NULL` root table), `VP8LHuffmanTablesAllocate`, `VP8LHuffmanTablesDeallocate`; and the Rust-only position type `HuffmanTablesOffset` (deviation 1). |
| `src/utils/huffman_encode_utils.rs` | `src/utils/huffman_encode_utils.c`, `src/utils/huffman_encode_utils.h` | yes: `HuffmanTreeToken`, `HuffmanTreeCode`, `HuffmanTree`, `ValuesShouldBeCollapsedToStrideAverage`, `OptimizeHuffmanForRle`, `CompareHuffmanTrees`, `SetBitDepths`, `GenerateOptimalTree`, `CodeRepeatedValues`, `CodeRepeatedZeros`, `VP8LCreateCompressedHuffmanTree`, `kReversedBits` (`REVERSED_BITS`), `ReverseBits`, `ConvertBitDepthsToSymbols`, `VP8LCreateHuffmanTree`. |
| `src/utils/color_cache_utils.rs` | `src/utils/color_cache_utils.c`, `src/utils/color_cache_utils.h` | yes: `VP8LColorCache`, `kHashMul` (`HASH_MUL`), `VP8LHashPix`, `VP8LColorCacheLookup`, `VP8LColorCacheSet`, `VP8LColorCacheInsert`, `VP8LColorCacheGetIndex`, `VP8LColorCacheContains`, `VP8LColorCacheInit`, `VP8LColorCacheClear`, `VP8LColorCacheCopy`. |
| `src/utils/palette.rs` | `src/utils/palette.c`, `src/utils/palette.h` | yes: `PaletteSorting` with its five enumerators, `PaletteCompareColorsForQsort`, `PaletteComponentDistance`, `PaletteColorDistance`, `SwapColor`, `SearchColorNoIdx`, `PrepareMapToPalette`, `COLOR_HASH_SIZE`, `COLOR_HASH_RIGHT_SHIFT`, `GetColorPalette`, `PaletteHasNonMonotonousDeltas`, `PaletteSortMinimizeDeltas`, `CoOccurrenceFindMax`, `CoOccurrenceBuild`, `Sum`, `PaletteSortModifiedZeng`, `PaletteSort`. |
| `src/dsp/lossless.rs` | `src/dsp/lossless.c`, `src/dsp/lossless.h` | yes, portable, without `VP8LDspInit` (not-applicable check 2) and without the encoder declarations of `lossless.h`, which `lossless_enc.c` defines (WP11): `VP8LPredictorFunc`, `VP8LPredictorAddSubFunc`, `VP8LMultipliers`, `VP8LStreaks`, `VP8LBitEntropy`; `Average2`, `Average3`, `Average4`, `Clip255`, `AddSubtractComponentFull`, `ClampedAddSubtractFull`, `AddSubtractComponentHalf`, `ClampedAddSubtractHalf`, `Sub3`, `Select`; `VP8LPredictor0_C` to `VP8LPredictor13_C`; `PredictorAdd0_C` to `PredictorAdd13_C`; the tables `VP8LPredictors`, `VP8LPredictorsAdd` and `VP8LPredictorsAdd_C` behind `vp8l_predictors`, `vp8l_predictors_add` and `vp8l_predictors_add_c`; `PredictorInverseTransform_C`, `VP8LAddGreenToBlueAndRed_C` and its dispatch, `ColorTransformDelta`, `ColorCodeToMultipliers`, `VP8LTransformColorInverse_C` and its dispatch, `ColorSpaceInverseTransform_C`, `MapARGB_C`, `MapAlpha_C` with `VP8LMapColor32b` and `VP8LMapColor8b`, `ColorIndexInverseTransform_C` and `VP8LColorIndexInverseTransformAlpha` (`COLOR_INDEX_INVERSE`), `VP8LInverseTransform`, `is_big_endian`, the five `VP8LConvertBGRATo*_C` converters and their dispatch, `CopyOrSwap`, `VP8LConvertFromBGRA`. |
| `src/dsp/lossless_common.rs` | `src/dsp/lossless_common.h` | partly: `VP8GetARGBIndex`, `VP8GetAlphaIndex`, `VP8GetARGBValue`, `VP8GetAlphaValue`, `VP8LSubSampleSize`, `VP8LNearLosslessBits`, `APPROX_LOG_WITH_CORRECTION_MAX`, `APPROX_LOG_MAX`, `LOG_2_PRECISION_BITS`, `LOG_2_RECIPROCAL`, `LOG_2_RECIPROCAL_FIXED_DOUBLE`, `LOG_2_RECIPROCAL_FIXED`, `LOG_LOOKUP_IDX_MAX`, `RightShiftRound`, `DivRound`, `WEBP_INT64_MAX`, `WEBP_UINT64_MAX`, `VP8LPrefixEncodeBitsNoLUT`, `VP8LPrefixEncodeNoLUT`, `PREFIX_LOOKUP_IDX_MAX`, `VP8LPrefixCode`, `VP8LAddPixels`, `VP8LSubPixels`, `GENERATE_PREDICTOR_ADD`. Not WP3's: `VP8LFastLog2`, `VP8LFastSLog2`, `VP8LPrefixEncodeBits`, `VP8LPrefixEncode` (WP11's exception, see the decision above); the declarations of `kLog2Table`, `kSLog2Table`, `VP8LFastLog2Slow`, `VP8LFastSLog2Slow`, `kPrefixEncodeCode` and `kPrefixEncodeExtraBitsValue` belong with their definitions in `lossless_enc.c` (WP11). |
| `src/utils/utils.rs` (exception) | function `WebPGetColorPalette` from `src/utils/utils.c` | yes, appended below WP1's items, which are unchanged. |
| `src/dec/vp8li_dec.rs` (exception) | type `VP8LTransform` from `src/dec/vp8li_dec.h` | yes, appended below the module header. |

## Evidence

Every differential test runs the C probe in a separate process, against the
oracle built from the pinned `v1.6.0` source with `python3 tools/build_oracle.py`.
The tests generate a seeded script, compute the expected output with the Rust
translation, and compare it with the probe's output line by line.
`python3 tools/check.py --oracle` runs them with the `oracle` feature, so a
missing oracle fails instead of skipping. The counts are those of one run on this
host (macOS ARM64, Apple clang 21).

| Test file | Test | Commands / observations | What it compares |
| --- | --- | ---: | --- |
| `tests/wp3_constants.rs` | `constants_and_sizes_match_the_c_headers` | 28 | The C `sizeof` of `HuffmanCode`, `HuffmanTablesSegment`, `HTreeGroup`, `HuffmanTreeToken`, `HuffmanTreeCode` and `HuffmanTree`, which are the `size` arguments of their allocations; the table and packing constants, `kHashMul`, the `PaletteSorting` enumerators, and the `lossless_common.h` constants, doubles as their 64-bit patterns. |
| `tests/wp3_huffman.rs` | `huffman_tables_match_the_c_construction` | 1,183 / 4,620 | 552 builds into tables and 165 validations with a `NULL` root table, 351 of the 717 rejected, 140,660 table entries compared. Seeded code-length sets: complete codes from random trees (up to 2,328 symbols and 15 bits), single-symbol codes of every length, incomplete and over-subscribed variants, lengths above 15, all-zero and uniformly random sets; alphabets of 19, 40, 256, 280 and 280 plus every cache size, and random ones; root bits 7, 8 and 1 to 12. Root segments too small, exact (`curr_table + total_size == size` needs a new segment) and one entry larger, negative sizes that fail the allocation, chained segments, the heap-sorted path above 512 symbols, and `Deallocate`. After every command: the return value, the codes at the current table, the segment chain with each segment's size, position and allocation. `VP8LHtreeGroupsNew` decisions for 7 counts. |
| `tests/wp3_huffman_encode.rs` | `encoder_huffman_trees_match_the_c_functions` | 901 / 2,604 | 401 `VP8LCreateHuffmanTree` calls with depth limits 7 (19 symbols) and 15 (40, 256, 280, 280 plus every cache size, random sizes) on sparse, dense, Fibonacci, geometric, single- and two-symbol, empty and run-structured histograms, including exact runs of 3 to 9 symbols around the RLE thresholds; some start with stale code lengths. The modified histogram, the RLE marks, the code lengths and the codes. 500 `VP8LCreateCompressedHuffmanTree` calls on the built codes and on code lengths with long runs, up to 2,400 symbols: the token count and every token. |
| `tests/wp3_color_cache.rs` | `color_cache_matches_the_c_cache` | 14,436 / 6,683 | `VP8LHashPix` for shifts 1 to 31; for every hash size 1 to 11, two caches with about 4,000 inserts, 2,000 sets, lookups, index and containment queries (of seen and unseen colors) and copies, a full dump of both caches after each round, `Init` with its shift and bits, and `Clear`. |
| `tests/wp3_palette.rs` | `palette_analysis_matches_the_c_functions` | 2,700 / 5,274 | 343 `GetColorPalette` and `WebPGetColorPalette` calls, with and without a palette buffer, on seeded pictures up to 70 by 40 with 1 to 356 colors (66 with more than 256), padded and negative strides; 242 `PrepareMapToPalette` and 726 `SearchColorNoIdx` calls; 1,389 `PaletteSort` calls, 463 for each of `kSortedDefault`, `kMinimizeDelta` and `kModifiedZeng`, on the pictures' palettes and on larger palettes, with black first, gray and green ramps (monotonous deltas) and one-color pictures. Whole 256-entry output buffers are compared, so untouched entries count too. |
| `tests/wp3_lossless.rs` | `lossless_dsp_matches_the_c_functions` | 2,257 / 2,268 | All 16 entries of `VP8LPredictors` on 1,212 pixel quadruples each, and the exported `VP8LPredictor2_C` to `13_C`; 640 calls of `VP8LPredictorsAdd` and `VP8LPredictorsAdd_C` over all 16 modes in the decoder's layout (row above, in place or from a separate row); `VP8LAddGreenToBlueAndRed` and `VP8LTransformColorInverse` with their `_C` functions, `VP8LMapColor32b` (in place and separate) and `VP8LMapColor8b`; 480 `VP8LInverseTransform` calls, 120 per transform type, widths 1 to 67 (mostly odd), every predictor and cross-color tile size (bits 2 to 9), every color-indexing depth (bits 0 to 3), from row 0 and later rows, in place (including the packed move of color indexing) and from a separate input, with the copy of the last row; 40 `VP8LColorIndexInverseTransformAlpha` calls; 440 `VP8LConvertFromBGRA` calls over the 11 RGB modes and 400 calls of the five converters and their dispatch functions. Whole buffers are compared, including the entries a call must not touch. |
| `tests/wp3_lossless_common.rs` | `lossless_common_helpers_match_the_c_header` | 3,161 / 5,061 | `VP8LSubSampleSize` at the wrap of `uint32_t`, `VP8LNearLosslessBits` for -20 to 120, `RightShiftRound` with sums that wrap, `DivRound` with every sign combination, both prefix encodings for distances 3 to 1,199 and 303 large ones, `VP8LAddPixels`, `VP8LSubPixels` and the four color-map accessors. |

Local results on this branch:

- `python3 tools/check.py --oracle --msrv` passes on the final tree: formatting,
  Clippy with `-D warnings` on all targets and features, the upstream pins, the
  module skeleton, the documentation links, `tools/unit_dependencies.py --check`
  (no undeclared references), and the tests with the `oracle` feature in the dev,
  release and `checked` profiles and with Rust 1.95. Each of the four runs reports
  46 tests, 7 of them WP3, none failed or skipped.
- `python3 tools/check.py --oracle-variant asan-ubsan --stage test` and
  `--oracle-variant debug --stage test` pass with the same 46 tests in each
  profile. The asan-ubsan oracle aborts on the first sanitizer report
  (`-fno-sanitize-recover=all`) and the debug oracle keeps upstream's
  assertions; neither fired.
- LeakSanitizer does not run on macOS. macOS `leaks --atExit` on each of the seven
  probes, fed with the scripts the tests generate, reports 0 leaks for 0 bytes.
  Linux CI runs LeakSanitizer.
- Mutations of the Rust code, each caught by the tests and then reverted: the
  exact-fit test of the segment allocation (`>=` to `>`), the tree-fullness check,
  the depth-limit doubling of `GenerateOptimalTree`, the three RLE thresholds of
  `OptimizeHuffmanForRle`, the prepend test of the modified Zeng sort, both
  black-first thresholds of the palette sorts, the predictor sentinels, the copy of
  the last predicted row, the packed move of color indexing, the shift of
  `ColorTransformDelta`, the byte swap of `CopyOrSwap`, the alpha order of
  `MODE_Argb`, the tie of `Select`, the tile row advance of the predictor and
  cross-color transforms, a mask of the RGB565 converter, the halving of
  `AddSubtractComponentHalf`, and the alpha-channel mask of `Average2`. Two
  mutations first survived, the exact segment fit and the RLE thresholds; the
  scripts gained the cases that catch them. A mutation of the lowest bit of the
  `Average2` mask is equivalent (the bit is shifted out) and survives by design.

## Not-applicable checks

These checks do not apply to WP3, or cannot be made on this host, with the reason:

1. **SIMD variants.** `lossless_sse2.c`, `lossless_sse41.c`, `lossless_avx2.c`,
   `lossless_neon.c` and the `_SSE` pointers that `lossless.h` declares
   (`VP8LAddGreenToBlueAndRed_SSE`, `VP8LPredictorsAdd_SSE`,
   `VP8LTransformColorInverse_SSE`, `VP8LConvertBGRAToRGB_SSE`,
   `VP8LConvertBGRAToRGBA_SSE`) are wave 4 (WP18 to WP20).
2. **`VP8LDspInit`** only assigns portable pointers (`COPY_PREDICTOR_ARRAY` and
   the converters), and ADR 0003 omits it with `WEBP_DSP_INIT_FUNC`. The tables are
   static arrays with the same entries, and the dispatch functions call the
   portable code. Its SIMD and MIPS branches are compiled out in the reference
   configuration.
3. **MIPS and MSA** (`lossless_mips_dsp_r2.c`, `lossless_msa.c`) are excluded in
   `tools/units.json`.
4. **`WEBP_SWAP_16BIT_CSP == 1`.** The swapped byte order of the RGBA4444 and
   RGB565 converters is off in the reference configuration; the standard order is
   translated.
5. **Big-endian hosts.** `is_big_endian` is the target's byte order, and
   `CopyOrSwap` keeps both branches. The host is little-endian, so the tests
   compare the little-endian branches only.
6. **Floating point.** No translated WP3 function computes in floating point.
   `LOG_2_RECIPROCAL` and `LOG_2_RECIPROCAL_FIXED_DOUBLE` are constants and are
   compared as bit patterns.
7. **Host refusals of an allocation.** The size tests of `WebPSafeMalloc` are
   compared through negative sizes (`alloc`, `groups`) and counts above the limit.
   A refusal by the host of an accepted size (the new segment, the heap-sorted
   array, the co-occurrence matrix) cannot be provoked here; the Rust code returns
   the same failure where C does (`try_reserve`, `webp_safe_malloc`).
8. **`VP8LColorCacheInit` failure.** The format allows 1 to 11 hash bits, which
   always pass the size test; the failure branch keeps C's partial state (deviation
   12) but is not reached.
9. **Uninitialized C memory.** `VP8LHuffmanTablesAllocate`, the new segments and
   `VP8LHtreeGroupsNew` use `WebPSafeMalloc` without initialization; Rust
   initializes (ADR 0003). `BuildHuffmanTable` writes every entry of the range it
   returns for an accepted code, and only that range is compared. The lines of
   `CoOccurrenceBuild` are written before they are read.
10. **Type sizes on other hosts.** The `C_SIZE` constants follow the pointer
    width; this host (arm64) checks the 64-bit values, and the x86-64 CI host
    checks them again. 32-bit targets are not built.
11. **Compiler plumbing.** `WEBP_INLINE`, `LOCAL_INLINE` (a workaround for GCC
    4.9 on ARM), `WEBP_RESTRICT`, `WEBP_NODISCARD` (`#[must_use]`) and
    `WEBP_UBSAN_IGNORE_UNSIGNED_OVERFLOW` (the wrapping operations) have no other
    counterpart.
12. **`NULL` arguments.** `VP8LHtreeGroupsFree(NULL)`,
    `VP8LHuffmanTablesDeallocate(NULL)` and `VP8LColorCacheClear(NULL)` exist only
    at the C ABI (PORTING rule 8); none of these headers is public.

### Inputs excluded from the oracle (PORTING rule 8)

The C code is undefined, asserts, or does not terminate on these inputs, so no
test issues them:

1. `VP8LBuildHuffmanTable` with a negative code length (C writes outside its
   histogram; Rust panics), with no code lengths (C asserts), or with `root_bits`
   above `MAX_ALLOWED_CODE_LENGTH` (C reads its histogram past the end). The
   decoder passes 7 or 8; the tests use 1 to 12.
2. `VP8LHuffmanTablesAllocate(0)` and `VP8LHtreeGroupsNew(0)`: the zero-size
   request of `WebPSafeMalloc` (WP1, not-applicable check 12).
   `VP8LHtreeGroupsNew` above `MAX_HTREE_GROUPS` with a successful allocation
   asserts in C.
3. `OptimizeHuffmanForRle` with counts of 2^31 or more: the conversion to `int`
   is implementation-defined and `abs(a - b)` can overflow. The histograms stay
   below 2^26.
4. `GenerateOptimalTree` with stale code lengths above the depth limit for unused
   symbols, which never satisfy the limit (C loops forever), or with more used
   symbols than `1 << (limit - 1)` (C asserts). The encoder starts from zeros.
5. `SearchColorNoIdx` with a color that is not in the palette (C loops forever),
   and `PrepareMapToPalette` with repeated colors (C asserts in its comparator).
6. `PaletteSort` with `kUnusedPalette`, `kPaletteSortingNum` or another value (C
   asserts and returns 0; Rust debug-asserts and returns `false`), and
   `kModifiedZeng` on a picture with one color but a palette of two or more,
   where `CoOccurrenceFindMax` asserts `c1 != c2`. The signed `delta` sums of
   the modified Zeng sort overflow only for co-occurrence counts beyond the test
   pictures.
7. `VP8LInverseTransform` with `row_start >= row_end` or `row_end > ysize` (C
   asserts), and with input and output rows that overlap without being equal.
   The batch predictors take a `WEBP_RESTRICT` output, so only the two layouts
   the decoder uses, equal or disjoint, have a defined result; both are tested.
8. `VP8LConvertFromBGRA` with `MODE_YUV`, `MODE_YUVA` or `MODE_LAST` (C asserts).
9. `VP8LHashPix` with a shift of 32 or more, `VP8LColorCacheInit` with hash
   bits outside 1 to 30, `VP8LSubSampleSize` with 31 or more sampling bits,
   `RightShiftRound` with a shift of 64 or more, `DivRound` by zero or with an
   overflowing quotient, and the prefix encodings for distances below 3: shifts
   out of range, a zero argument to `BitsLog2Floor` or signed overflow in C.

## Recorded deviations

Deviations are changes of representation that the ADRs allow, or behavior that
the Rust side cannot reproduce exactly. Each records the reason.

1. **`HuffmanTables` stores positions (ADR 0003, "Readers over input").** The
   tables own their segments: `root`, then `next_segments` in the order of the C
   `next` chain, so `HuffmanTablesSegment` has no `next` field. `curr_segment` is
   the segment's index (0 is `root`), and `curr_table` is a position in the
   segment's `start`. A C `HuffmanCode*` into the tables is a
   `HuffmanTablesOffset { segment, offset }`, and `HTreeGroup::htrees` holds five
   of them; the owner (the decoder's `VP8LMetadata`, WP7) passes the tables to
   whatever reads an entry. `segment()` and `segment_mut()` resolve an index. A
   second-level table is addressed as in C, relative to its root entry in the same
   segment.
2. **Segment lifetimes.** `VP8LHuffmanTablesAllocate` drops the old chain where C
   cuts it with `root->next = NULL` without freeing; `Deallocate` empties
   `root.start` and drops the chain. As in C, `Deallocate` leaves `curr_segment`,
   `root.curr_table` and `root.size` alone; a stale `curr_segment` panics on use
   where the C pointer dangles. A new segment replaces the chain after the current
   one, as `curr_segment->next = next` does; the current segment is always the last.
3. **Allocation of a new segment.** C allocates the segment struct with
   `WebPSafeMalloc(1, sizeof(*next))`, whose size test always passes; the Rust
   code reserves the slot in `next_segments` with `try_reserve`, so a refusal by
   the host fails the build as in C. The codes go through `webp_safe_malloc` with
   `HuffmanCode::C_SIZE`. The `C_SIZE` constants of the six allocated types are
   compared with the C `sizeof` (`tests/wp3_constants.rs`).
4. **`VP8LBuildHuffmanTable`** takes `Option<&mut HuffmanTables>`, where `None` is
   the `NULL` root table that only validates the code lengths (kept for WP7), and
   the code lengths as a slice of `code_lengths_size` entries. Like C, it builds at
   `curr_table` and does not advance it; the decoder adds the returned size.
   `BuildHuffmanTable` takes the root table and the sorted array as `Option`s,
   `None` together, and the C `table` pointer is a position in the root table.
5. **Sorted symbols.** The 512-entry stack array of C is a Rust array on the
   stack; longer code-length sets use `webp_safe_malloc::<u16>` with the C size 2,
   and a refusal returns 0 as in C.
6. **Tree groups.** `VP8LHtreeGroupsNew` returns `Option<Vec<HTreeGroup>>` with
   default groups (C leaves them uninitialized). `VP8LHtreeGroupsFree` only frees,
   so the owner drops the vector instead, as for `WebPSafeFree` (WP1, deviation 5).
   The flags `is_trivial_literal`, `is_trivial_code` and `use_packed_table` are
   `bool`, because C uses them only as predicates.
7. **`HuffmanTreeCode` owns its arrays.** `code_lengths: Vec<u8>` and
   `codes: Vec<u16>` replace the two pointers. The encoder carves both arrays of
   every code from one allocation (`GetHuffBitLengthsAndCodes` in
   `src/enc/vp8l_enc.c`); ADR 0003 lets the Rust caller check that total and
   allocate separate buffers. `StoreFullHuffmanCode` builds its 19-entry code the
   same way. WP11 consumes this form, which the integrator confirmed on
   2026-11-06.
8. **Token output.** `VP8LCreateCompressedHuffmanTree` takes the `max_tokens`
   tokens as a slice, the pointer and length pair of ADR 0003; a write past them
   panics where C asserts. `CodeRepeatedValues` and `CodeRepeatedZeros` return
   the next position instead of the next pointer.
9. **Sorting (PORTING rule 8).** `qsort` becomes `sort_unstable_by` with the C
   comparators. `GenerateOptimalTree` sorts leaves whose symbol values differ
   (the comparator asserts it), `GetColorPalette` sorts distinct colors, and
   `PrepareMapToPalette` asserts distinct colors. No two keys tie, so the order is
   the one total order and equals the C order; there is nothing implementation-
   defined to exclude. The comparators keep their debug assertions.
10. **Truncations kept.** `SetBitDepths` stores the depth as `uint8_t`, which wraps
    above 255 in the first rounds of a deep tree, as in C; `ReverseBits` results are
    truncated to `uint16_t`; the C `int` to `uint8_t` and `uint16_t` conversions of
    the table builder and the palette indexes are explicit casts. The unsigned
    sums of the RLE heuristic, the co-occurrence counters and the pixel arithmetic
    wrap.
11. **`VP8LColorCache`.** `colors` is a `Vec<u32>`, empty where C has `NULL`.
    `Set` and `Insert` take `&mut`, because C writes through the cache's pointer
    from a `const` struct pointer. Keys index after a checked conversion.
12. **Partial state of `VP8LColorCacheInit`.** A failed allocation leaves
    `colors` empty and `hash_shift` and `hash_bits` unchanged, as in C.
13. **Pictures in the palette code.** The ARGB plane is read through
    `WebPPicture::argb` and `argb_storage` (ADR 0003, buffers and planes). Row `y`
    starts at `offset + y * argb_stride`, computed per row, so a negative stride
    never forms the position past the last row that the C pointer increment forms.
    `CoOccurrenceFindMax` returns `(c1, c2)` instead of writing two out-parameters;
    the `best_sum` pointer of the modified Zeng sort is an index. `PaletteSort`,
    `PaletteSortModifiedZeng` and `CoOccurrenceBuild` return `bool`.
14. **One buffer for the decoder transforms.** The inverse transforms read and
    write one pixel buffer: input and output may be the same rows, and the
    predictors read the row above and the pixel to the left of the output.
    `VP8LInverseTransform`, the batch predictors, `VP8LAddGreenToBlueAndRed`,
    `VP8LTransformColorInverse` and `VP8LMapColor32b` therefore take the buffer once
    (`data`) with the C pointers as positions (ADR 0003, pointers), so every access
    happens at the C index and in the C order whatever the overlap. WP4's unfilters
    copy the input instead (WP4, deviation 15); here the predictors read output
    written by the same call (the top-right pixel of the last column is the first
    pixel of the current row), which a copy would not reproduce. WP7 keeps
    `pixels` and `argb_cache` in one buffer, as the C allocation does, or passes
    the buffer that holds both. The 8-bit alpha path and the converters take
    distinct slices, as their callers have distinct buffers. The integrator
    confirmed this form on 2026-11-06.
15. **Predictor signatures.** A one-pixel predictor takes `(left, left_pos, top,
    top_pos)`, where `left[left_pos]` is `*left` and `top[top_pos + k]` is `top[k]`
    for `k` in -1 to 1. `VP8LPredictorAddSubFunc` is the decoder's form,
    `(data, in, upper, num_pixels, out)`. `VP8LPredictorsSub` (WP11) reads its input
    and the row above from one buffer and writes residuals into another, so its
    dispatch function will have its own signature, as WP4 deviation 1 allows.
16. **Dispatch (ADR 0003).** `VP8LPredictors`, `VP8LPredictorsAdd` and
    `VP8LPredictorsAdd_C` are static arrays with the entries of
    `COPY_PREDICTOR_ARRAY`, including the sentinels at 14 and 15, behind functions
    that take the mode first. The pointers `VP8LAddGreenToBlueAndRed`,
    `VP8LTransformColorInverse`, `VP8LConvertBGRATo*`, `VP8LMapColor32b` and
    `VP8LMapColor8b` are functions that call the portable code.
17. **`GENERATE_PREDICTOR_ADD`** is a `macro_rules!` macro in
    `src/dsp/lossless_common.rs`, re-exported to the crate, which generates
    functions of the decoder form. The C `assert(upper != NULL)` has no
    counterpart, because positions cannot be `NULL`; the left position is formed
    with a wrapping subtraction, as the C `&out[x - 1]` is formed, and only read by
    the predictors that read it.
18. **Out-parameters.** `VP8LPrefixEncodeBitsNoLUT` returns `(code, extra_bits)`
    and `VP8LPrefixEncodeNoLUT` returns `(code, extra_bits, extra_bits_value)`;
    C writes every out-parameter on every call. `ColorCodeToMultipliers` keeps its
    `&mut VP8LMultipliers`, which the caller reuses across tiles.
19. **Converters keep their element types.** `VP8LConvertFromBGRA` and the
    converters take `&[u32]` and write `&mut [u8]`, as the C pointers are typed
    (the untyped-sample-buffer rule of ADR 0003 covers only `void*`), as WP4's
    `WebPMultARGBRows` keeps `&mut [u32]` (WP4, deviation 7); the integrator
    confirmed this form on 2026-11-06. WP9's `picture_csp_enc.c` casts an RGBA
    byte buffer to `uint32_t*` for `VP8LConvertBGRAToRGBA`; WP9 builds the `u32`
    samples from the RGBA bytes in native byte order, which is what the C cast
    reads. A mode without a conversion debug-asserts and writes nothing, where C
    asserts.
20. **`VP8LTransform`** owns `data` as a `Vec<u32>` (empty where C has `NULL`);
    the decoder allocates, replaces and frees it. The field `type` is the raw
    identifier `r#type`.
21. **Lint expectations.** `LOG_2_RECIPROCAL` keeps the C literal under an
    item-level `clippy::approx_constant` expectation, the comparators keep their
    `if` chains (`clippy::comparison_chain`), the palette comparator takes
    references as the sort requires (`clippy::trivially_copy_pass_by_ref`), and
    `PaletteSortMinimizeDeltas` keeps its index loops
    (`clippy::needless_range_loop`). Each names the C construct (ADR 0005).

## Notes for the packages that build on WP3

- **WP7.** Build tables with `vp8l_build_huffman_table(Some(&mut tables), ...)`,
  record `HuffmanTablesOffset { segment: tables.curr_segment, offset:
  tables.segment(tables.curr_segment).curr_table }` in `htrees[j]`, then add the
  size to `curr_table`; `ReadSymbol` reads `tables.segment(pos.segment).start`
  from `pos.offset`. Pass `None` for the skipped tree groups. Call
  `vp8l_inverse_transform` with the decoder's pixel buffer and the positions of
  `rows` and `argb_cache` in it, with a row before `argb_cache`.
- **WP11.** `HuffmanTreeCode` owns its arrays (deviation 7); check the carved
  total with the C arithmetic before allocating the codes. `VP8LPredictorsSub`
  needs its own signature (deviation 15). `VP8LFastLog2`, `VP8LFastSLog2`,
  `VP8LPrefixEncodeBits` and `VP8LPrefixEncode` are WP11's exception in
  `lossless_common.rs`; append them below WP3's items.
- **WP9.** `vp8l_convert_bgra_to_rgba` takes `u32` pixels and writes bytes;
  build the pixels from the RGBA bytes in native byte order (deviation 19). `webp_get_color_palette` is in `src/utils/utils.rs`.
