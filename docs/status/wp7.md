# WP7: VP8L decoder

- Status: every unit of the WP7 inventory is translated, except
  `VP8LDelete`, which only clears and frees and is the owner's drop, and the
  `Init` calls that only assign portable pointers (ADR 0003). Each is listed
  under not-applicable checks or recorded deviations. The release, asan-ubsan
  and debug oracles agree with the translation on every compared case.
- Sub-wave: 2a
- Model: Claude Opus 5.5 (`claude-opus-5-5`)
- Depends on: WP1, WP2, WP3, WP4, and the rescaler change of WP4 that the
  integrator decided on 2026-11-07 (merged into `main` with PR #9)
- Probes: `tools/oracle/probes/wp7/`; tests: `tests/wp7_*.rs`

Written by the package session; the integrator aggregates it into
[PORT_STATUS.md](../PORT_STATUS.md). See the package's row in
[WORKPLAN.md](../WORKPLAN.md) for its verification inputs. The inventory
comes from `tools/units.json`, including the function- and type-level
exceptions inside a module.

## Decision: decoding context per call

`VP8Io` with its hooks and `opaque`, and `WebPDecParams` with its output
callbacks, are the interface that WP6, WP12 and WP16 build on. In libwebp their
pointers refer to siblings or owners of the same object: `WebPIDecoder` holds
`io`, `params`, `dec`, `mem` and `output` side by side, with `io.opaque =
&params`, `io.data` inside `mem` and `params.output = &output`; the VP8L
decoder stores `dec->io` and `dec->output`; `VP8Decoder` owns `ALPHDecoder`,
whose `io.opaque` points back at it; and the rescalers borrow memory that their
owner also holds. ADR 0003 did not settle this, so WP7 stopped at the checkpoint
with a proposal, and the integrator decided on 2026-11-07:

- Option A, "context per call": a decoder structure holds no pointer to its
  input, its `VP8Io`, its output, its hook context or its owner; the owner passes
  them to each call, and pointers into them become positions. ADR 0003 records
  the rule as "Decoding context per call (decided 2026-11-07, WP7)", with the
  consequences for WP6 and WP12.
- `output` is passed per call next to `params`, not held in `WebPDecParams`.
- R1: `WebPRescaler` owns its two work rows and takes its destination per call.
  The WP4 session made this change in a separate branch with
  `WebPRescaler::C_SIZE`; WP7 is written against it.

## Inventory

Every row names what is translated; each item carries an `// upstream:` marker.
Names follow ADR 0001 (`tools/rust_name.py`). No module uses `unsafe`.

| Rust module | Upstream units | Translated |
| --- | --- | --- |
| `src/dec/vp8l_dec.rs` | `src/dec/vp8l_dec.c` | yes, except `VP8LDelete` (deviation 9): `NUM_ARGB_CACHE_ROWS`, `kCodeLengthLiterals` (`CODE_LENGTH_LITERALS`), `kCodeLengthRepeatCode`, `kCodeLengthExtraBits`, `kCodeLengthRepeatOffsets`, `HuffIndex` with `GREEN`, `RED`, `BLUE`, `ALPHA`, `DIST`, `kAlphabetSize`, `kLiteralMap`, `NUM_CODE_LENGTH_CODES`, `kCodeLengthCodeOrder`, `CODE_TO_PLANE_CODES`, `kCodeToPlane`, `FIXED_TABLE_SIZE`, `kTableSize`, `VP8LSetError`, `VP8LCheckSignature`, `ReadImageInfo`, `VP8LGetInfo`, `GetCopyDistance`, `GetCopyLength`, `PlaneCodeToDistance`, `ReadSymbol`, `BITS_SPECIAL_MARKER`, `PACKED_NON_LITERAL_CODE`, `ReadPackedSymbols`, `AccumulateHCode`, `BuildPackedTable`, `ReadHuffmanCodeLengths`, `ReadHuffmanCode`, `ReadHuffmanCodes`, `ReadHuffmanCodesHelper`, `AllocateAndInitRescaler`, `Export`, `EmitRescaledRowsRGBA`, `EmitRows`, `ConvertToYUVA`, `ExportYUVA`, `EmitRescaledRowsYUVA`, `EmitRowsYUVA`, `SetCropWindow`, `GetMetaIndex`, `GetHtreeGroupForPos`, `ProcessRowsFunc`, `ApplyInverseTransforms`, `ProcessRows`, `Is8bOptimizable`, `AlphaApplyFilter`, `ExtractPalettedAlphaRows`, `Rotate8b`, `CopySmallPattern8b`, `CopyBlock8b`, `CopySmallPattern32b`, `CopyBlock32b`, `DecodeAlphaData`, `SaveState`, `RestoreState`, `SYNC_EVERY_N_ROWS`, `DecodeImageData`, `ClearTransform`, `ExpandColorMap`, `ReadTransform`, `InitMetadata`, `ClearMetadata`, `VP8LNew`, `VP8LClear`, `UpdateDecoder`, `DecodeImageStream`, `AllocateInternalBuffers32b`, `AllocateInternalBuffers8b`, `ExtractAlphaRows`, `VP8LDecodeAlphaHeader`, `VP8LDecodeAlphaImageStream`, `VP8LDecodeHeader`, `VP8LDecodeImage`. |
| `src/dec/vp8li_dec.rs` | `src/dec/vp8li_dec.h`; except `VP8LTransform` (WP3) | yes: `VP8LDecodeState` with `READ_DATA`, `READ_HDR`, `READ_DIM`, `VP8LMetadata`, `VP8LDecoder` (deviations 1 to 3). The functions it declares are defined in `vp8l_dec.c` and translated there. WP3's `VP8LTransform` is unchanged. |
| `src/dec/alphai_dec.rs` | `src/dec/alphai_dec.h` | yes: `ALPHDecoder` (deviation 1). The header also declares `WebPDeallocateAlphaMemory`, which `src/dec/alpha_dec.c` defines; it belongs to WP6. |
| `src/dec/vp8_dec.rs` (exception) | types `VP8Io`, `VP8IoPutHook`, `VP8IoSetupHook`, `VP8IoTeardownHook` from `src/dec/vp8_dec.h` | yes: `VP8Io` with its scalar fields; the trait `VP8IoHooks`, whose methods `setup`, `put` and `teardown` are the three hook types and whose `has_setup`, `has_put` and `has_teardown` keep C's `NULL` tests; and the Rust-only `VP8IoRows`, the rows that C passes through `io->y`, `io->u`, `io->v` and `io->a` (deviation 1). Appended below the module header. |
| `src/dec/webpi_dec.rs` (exception) | types `WebPDecParams`, `OutputFunc`, `OutputAlphaFunc`, `OutputRowFunc` from `src/dec/webpi_dec.h` | yes (deviation 1). Appended below the module header. |
| `src/dec/webp_dec.rs` (exception) | functions `WebPIoInitFromOptions`, `WebPCheckCropDimensions`, `WebPCheckCropDimensionsBasic` from `src/dec/webp_dec.c` | yes. Appended below the module header. |

## Evidence

Every differential test runs the C probe in a separate process, against the
oracle built from the pinned `v1.6.0` source with `python3 tools/build_oracle.py`.
The tests generate a seeded script, compute the expected output with the Rust
translation, and compare it with the probe's output line by line. After the
header and after every step, the decode and alpha probes print the status, the
decoder's scalar fields and bit reader (value, length, position, bit position,
end of stream), its transforms with a digest of their data, the color cache, the
Huffman image, every tree group (each tree as segment and offset, the trivial
literal and code, the packed table) and every table segment with a digest of
its codes, all `VP8Io` fields, and every output plane in hex or as a 64-bit
FNV-1a digest of all its bytes, including the bytes outside the decoded area
(filled with `0xa5`). `python3 tools/check.py --oracle` runs the tests with the
`oracle` feature, so a missing oracle fails instead of skipping. The counts are
those of one run on this host (macOS ARM64, Apple clang 21).

| Test file | Test | Cases | What it compares |
| --- | --- | ---: | --- |
| `tests/wp7_info.rs` | `header_info_matches_the_c_functions` | 3,802 | `VP8LCheckSignature` and `VP8LGetInfo` on the first chunk of every test vector at sizes 0 to 12 and in full, 12 mutated headers per vector (magic byte, size and alpha bits, version), and 400 random headers. |
| `tests/wp7_io_options.rs` | `io_init_from_options_matches_the_c_function` | 7,000 | 3,000 `WebPCheckCropDimensions` calls around the bounds (zero, negative, exact fit, one past, `INT_MAX`, `INT_MIN`); 4,000 `WebPIoInitFromOptions` calls without options and with them, for every source colorspace (the YUV crop snapping), crops inside and outside the picture, scaled sizes of 0 (derived), negative and above `INT_MAX / 2`, bypass and fancy flags, from a zeroed io and from one whose fields hold 777, so the fields that a call leaves alone, also after a failure, are visible. |
| `tests/wp7_decode.rs` | `lossless_vec_vectors_match_the_c_decoder` | 4,003 | The 32 `lossless_vec_*` vectors. |
| `tests/wp7_decode.rs` | `color_transform_vector_matches_the_c_decoder` | 133 | `lossless_color_transform.webp`. |
| `tests/wp7_decode.rs` | `other_lossless_vectors_match_the_c_decoder` | 1,141 | `bad_palette_index`, `color_cache_bits_11`, `dual_transform`, `one_color_no_palette`, `near_lossless_75` and `lossless1` to `lossless4`. |
| `tests/wp7_decode.rs` | `large_random_alpha_vector_matches_the_c_decoder` | 2 | `lossless_big_random_alpha.webp` (2048 by 2048): RGBA, and YUVA cropped and scaled. |
| `tests/wp7_decode.rs` | `generated_pictures_match_the_c_decoder` | 4,913 | 120 seeded pictures, 1 to 64 by 1 to 40 pixels (most narrower than 8), with few colors, gradients, repeated patterns, noise and sparse alpha, encoded by the oracle's own `cwebp -lossless` with every method and quality. |
| `tests/wp7_decode.rs` | `crafted_distances_match_the_c_decoder` | 1,200 | 600 hand-built streams of 1 to 40 by 1 to 24 pixels with a two-symbol green code, random lengths, distance symbols and extra bits: plane codes whose distance falls below 1 (clamped), references before the start and past the end. |
| `tests/wp7_alpha.rs` | `alpha_planes_match_the_c_decoder` | 946 | The 13 lossless ALPH chunks (`alpha_filter_*`, `alpha_color_cache`, `big_endian_bug_393`, `lossy_alpha1` to `4`; 12 on the 8-bit path, one on the 32-bit path), every filter, in row steps of 16, 1, 7, irregular and all rows, with crop windows, up to 38 truncations each and 24 mutations each. |
| `tests/wp7_huffman_codes.rs` | `huffman_codes_match_the_c_reader` | 1,483 | `ReadHuffmanCodesHelper` on generated codes for every color cache size: simple codes of one and two symbols (inside and outside the alphabet), complete 8-bit codes through a code-length code with and without `max_symbol`, random bits, truncations and flipped bits, with and without a mapping, and the arguments that fail before reading; 806 succeed, with 934 packed tables and 958 trivial codes. |

Each vector of the decode tests runs: a BGRA decode dumped in hex; all 13
colorspaces with tight strides and with padded strides, half of them flipped;
options without cropping or scaling; the whole picture and the last pixel as
crop windows; six random crops; eight scales (down, up, one side derived),
alone and after a crop, in RGB and YUV modes; two invalid options (a crop past
the edge, a negative scaled width); truncations at up to 58 sizes, including
around the smallest prefix that holds the header; four incremental decodings
from that prefix in 1 to 64 steps (plain, flipped YUVA, cropped, scaled); one
that suspends and then fails without the incremental flag; and 24 mutations, of
which half run incrementally. Over all decode tests: 11,392 decodes, 4,828
failed headers, and after the steps 4,490 `VP8_STATUS_OK`, 84
`VP8_STATUS_INVALID_PARAM`, 1,990 `VP8_STATUS_BITSTREAM_ERROR` and 11,834
`VP8_STATUS_SUSPENDED`.

Local results on this branch:

- `python3 tools/check.py --oracle --msrv` passes on the final tree: formatting,
  Clippy with `-D warnings` on all targets and features, the upstream pins, the
  module skeleton, the documentation links, `tools/unit_dependencies.py --check`,
  and the tests with the `oracle` feature in the dev, release and `checked`
  profiles and with Rust 1.95.
- The WP7 tests pass against the asan-ubsan oracle, which aborts on the first
  sanitizer report (`-fno-sanitize-recover=all`), and against the debug oracle,
  which keeps upstream's assertions; neither fired.
- LeakSanitizer does not run on macOS. macOS `leaks --atExit` on each of the five
  probes, fed with scripts that reach success, invalid options, bitstream errors,
  suspensions and failed headers, reports 0 leaks for 0 bytes. Linux CI runs
  LeakSanitizer.
- Mutations of the Rust code, each run against the tests and then reverted: 23
  in all, 20 caught. They covered the error precedence of `VP8LSetError`, the
  distance clamp, the 8-bit and 32-bit pattern copies, the previous code length,
  the mapping threshold, the trivial code test, the packed alpha shift, the
  check-point interval, the color cache restore, the alpha top row and row
  blocks, the color map tail, the unpremultiplication of scaled rows, the U and
  V store flag, the crop snapping, the alpha `prev_line`, the copy distance,
  the bypass flag and the color cache size. The error precedence and the
  distance clamp first survived; the incremental decodings that fail after a
  suspension and the crafted streams were added to catch them. Three mutations
  are equivalent and survive by design: `dist >= 1` as `dist > 1` returns the
  same value, VP8L never passes a row past `crop_bottom` to `SetCropWindow`, and
  one more top-prediction pixel only moves `argb_cache`.

## Not-applicable checks

These checks do not apply to WP7, or cannot be made on this host, with the reason:

1. **No `put` hook in VP8L.** `VP8LDecodeImage` writes the rows into the output
   buffer itself and calls none of the `VP8Io` hooks; the work plan's "ARGB rows
   dumped through a put callback" are therefore compared in the output buffer,
   after the header and after every step, including the bytes outside the
   decoded area. The hooks are called by WP6's `frame_dec.c`, and WP6 tests them.
2. **`Init` functions.** `VP8LDspInit` (in `VP8LNew`), `WebPInitAlphaProcessing`
   and `WebPInitConvertARGBToYUV` (in `VP8LDecodeImage` and
   `VP8LDecodeAlphaImageStream`) only assign portable pointers, and ADR 0003
   omits them with an `// upstream:` comment. The alpha probe calls
   `VP8FiltersInit`, as `ALPHInit` does before the VP8L alpha calls.
3. **`WEBP_REDUCE_SIZE`** is unset in the reference configuration, so the
   rescaling branches are translated and its `else` branches (which reject
   scaling with `VP8_STATUS_INVALID_PARAM`) are not compiled.
4. **Host refusals of an allocation.** The size test of `WebPSafeMalloc` is
   applied with the C sizes, but a refusal by the host of an accepted size
   cannot be provoked here; the Rust code returns `VP8_STATUS_OUT_OF_MEMORY`
   where C does. `VP8LNew` allocates a fixed-size structure, whose size test
   always passes (deviation 9).
5. **Big-endian hosts.** `Rotate8b`, the two-byte pattern of `CopyBlock8b`, the
   alpha byte of `ConvertToYUVA` and the native byte order of the casts keep both
   branches; the host is little-endian, so the tests compare the little-endian
   branches only.
6. **32-bit ARM and MIPS branches of `CopyBlock8b`** (`__arm__`,
   `WEBP_USE_MIPS_DSP_R2`) compute the same pattern values as the multiplication;
   neither target is built.
7. **`NULL` arguments.** `VP8LDecodeHeader(NULL, ...)`, `VP8LDecodeHeader(dec,
   NULL)` (which records `VP8_STATUS_INVALID_PARAM`), `VP8LDecodeImage(NULL)`,
   `VP8LGetInfo(NULL, ...)` and its `NULL` out-parameters exist only at the C ABI
   (PORTING rule 8); these headers are not public.
8. **Uninitialized C memory.** `VP8LHtreeGroupsNew` does not initialize the
   groups, and C sets `literal_arb` only for a trivial literal and the packed
   table only when it is used; the probes print them only then. The pixel and
   transform buffers come from `WebPSafeMalloc` and are written before they are
   read; Rust zero-initializes them (ADR 0003).
9. **Threads.** The VP8L decoder starts no worker. The constraint that WP6 meets
   with `use_threads` is recorded under the notes below.
10. **Floating point.** No translated WP7 function computes in floating point.
11. **Type sizes.** WP7 allocates no array of a type of its own; the C sizes it
    uses are those of `HuffmanCode`, `HTreeGroup` (WP3) and `WebPRescaler` (WP4),
    which their packages compare with the C `sizeof`, and of `int` and
    `uint32_t`.

### Inputs excluded from the oracle (PORTING rule 8)

The C code is undefined or asserts on these inputs, so no test issues them:

1. `VP8LDecodeImage` after it failed or completed: C asserts that the Huffman
   tables exist and that `last_row` grows. The tests stop after a failure or
   when the status is no longer `VP8_STATUS_SUSPENDED`, as the incremental
   decoder does.
2. `VP8LDecodeAlphaImageStream` with `last_row` above the height or the crop
   bottom (C asserts), or without a decoded alpha header.
3. Output buffers smaller than the decoded area, or whose planes overlap, or a
   `WebPDecBufferPlanes` variant that does not match the colorspace: C writes
   outside the buffer or reads the other union member; Rust panics.
4. `WebPIoInitFromOptions` on an io whose `width * 3` or `height * 3` overflows
   an `int`, with scaling enabled.
5. `ReadHuffmanCodesHelper` with a mapping that leaves groups unfilled or maps
   outside them, or with color cache bits outside 0 to 11: C reads uninitialized
   groups or indexes `kTableSize` outside its bounds.
6. `WebPISetIOHooks` combined with a lossless image: C reads the user data as
   `WebPDecParams` in `VP8LDecodeImage`. The Rust decoder takes the parameters
   per call, so this case is defined there; it belongs to WP12's tests.

## Recorded deviations

Deviations are changes of representation that the ADRs allow, or behavior that
the Rust side cannot reproduce exactly. Each records the reason.

1. **Decoding context per call (ADR 0003).** `VP8Io` keeps its scalar fields;
   its `int` flags are `bool`. The input of `io->data` is a `data: &[u8]`
   argument of `vp8l_decode_header`, `vp8l_decode_image`,
   `vp8l_decode_alpha_header` and `vp8l_decode_alpha_image_stream`, and of every
   internal function that reads bits; the image call must receive the same bytes
   at the same positions as the header call, as the bit reader of WP2 keeps
   positions. `VP8LDecoder` stores no `io` and no `output`: `vp8l_decode_image`
   takes the `VP8Io`, the parameters and the output buffer; C stores `output` at
   the first call and reads it later, and the Rust caller passes the same buffer
   each time. `ALPHDecoder` stores no `output`: the VP8 decoder's alpha plane is
   an argument of `vp8l_decode_alpha_image_stream`, and `prev_line` is a position
   in it. `alph_dec->io.opaque` and `dec->io` are not stored; the alpha
   functions take `alph_dec`. `WebPDecParams` holds `options` as a borrow,
   `tmp_y`, `tmp_u` and `tmp_v` as positions in `memory`, the rescalers by value,
   and the emit functions as `Option` plain function pointers.
2. **Internal pixel buffers.** C's `pixels` is `uint32_t*` for the 32-bit path
   and `uint8_t*` for the paletted alpha path; Rust has `pixels: Vec<u32>` and
   `pixels_8b: Vec<u8>`. The 32-bit buffer holds the pixels, the top-prediction
   row and `argb_cache` in one allocation, as C does and as WP3's in-place
   transforms require (WP3 deviation 14); `argb_cache` is an `Option<usize>`
   position, `None` where C has `NULL`. `VP8LClear` leaves it, as C leaves the
   stale pointer. The decode loop borrows the pixel buffer out of the decoder for
   its duration, and the row-processing function receives it.
3. **Rescaler memory.** C carves the rescaler, its work rows and the scaled row
   from one block (`rescaler_memory`). The size test of `WebPSafeMalloc` is
   applied to that total with `WebPRescaler::C_SIZE` (ADR 0003) by reserving and
   dropping it; then the rescaler receives its work rows (WP4's R1 form) and the
   decoder holds the scaled row as bytes (`rescaled_row`).
4. **ARGB rows and the byte rescaler.** C casts the `uint32_t` rows to
   `uint8_t*` for `WebPRescalerImport` and the rescaler's byte row back to
   `uint32_t*` for `WebPMultARGBRow` and `VP8LConvertFromBGRA`. Rust converts the
   rows with `to_ne_bytes` and `from_ne_bytes`, one copy per row with the same
   bytes. The premultiplication before the import happens in place on the pixel
   buffer, as in C; the unpremultiplication after the export happens on the
   converted row, where C changes the rescaler's row in place, which the next
   export overwrites before anything reads it. `WebPExtractAlpha` reads a byte
   copy of the row.
5. **`ProcessRowsFunc`** is a `&mut dyn FnMut(&mut VP8LDecoder, &mut [u32], i32)`
   closure that carries its context: the `VP8Io` and the output buffer for
   `ProcessRows`, and the alpha parts for `ExtractAlphaRows` (C reads both
   through `dec->io` and `io->opaque`). `None` is C's `NULL`.
6. **Errors (ADR 0003).** `VP8LSetError` keeps its precedence and returns the
   status that the decoder holds afterwards. `vp8l_decode_header`,
   `vp8l_decode_image` and `vp8l_decode_alpha_image_stream` return
   `Result<(), VP8StatusCode>` with the recorded status; a suspended incremental
   decoding is `Ok` with `dec.status == VP8_STATUS_SUSPENDED`, as C returns 1.
   The internal functions keep their `int` results as `bool` or sizes, because
   several of them fail without recording an error (a repeated transform, the
   end of the stream in `ReadHuffmanCodes`, a mismatched mapping), so the status
   field is the only record, as in C. `vp8l_decode_alpha_header` returns `bool`:
   its decoder is dropped on failure, as C deletes it.
7. **`VP8LGetInfo`** returns `Option<(width, height, has_alpha)>`, because C
   writes its out-parameters only on success; `has_alpha` is a `bool`.
   `VP8LCheckSignature` and `VP8LGetInfo` take a slice whose length is C's size.
   `ReadImageInfo` keeps its out-parameters, which C writes before the version
   check.
8. **`ReadHuffmanCodesHelper`** takes the tables and the groups separately from
   the decoder, as C does; the decoder's own tables are moved out of `dec.hdr`
   for the call, because C passes `&dec->hdr.huffman_tables` next to `dec` and the
   helper reaches the decoder only for its bit reader and status.
   `htree_groups` is `Option<Vec<HTreeGroup>>`, `None` where C has `NULL`, and
   becomes `None` on failure. Tree pointers are `HuffmanTablesOffset` positions
   (WP3 deviation 1).
9. **`VP8LNew` and `VP8LDelete`.** `VP8LNew` returns a `Box`; its fixed-size
   allocation cannot fail the size test, and a host refusal aborts. `VP8LDelete`
   only clears and frees, so the owner drops the box instead, as for
   `WebPSafeFree` (WP1, deviation 5).
10. **Bit reader copies.** `SaveState` and `RestoreState` copy the bit reader
    field by field, as the C struct assignment does; WP2's `VP8LBitReader` has no
    `Clone`.
11. **Output rows.** Row `y` of a plane starts at `offset + y * stride`, computed
    per row with the signed stride, so a flipped buffer never forms the position
    past its last row that the C pointer arithmetic forms. `ConvertToYUVA`
    borrows the U and V rows together, from one region (internal storage) or two
    (external storage).
12. **`ExpandColorMap`** works on the bytes of the color map in native order, as
    C does through its `uint8_t*` casts.
13. **`AlphaApplyFilter`** unfilters each row in place by passing a copy of the
    row as the input, as WP4 deviation 15 allows; the previous row is a position
    in the alpha plane.
14. **Pattern copies.** `CopySmallPattern8b` and `CopySmallPattern32b` test the
    address of the destination for alignment, as C does, and store the pattern
    through its native bytes; the 64-bit swap is `rotate_left(32)`. The `goto
    Copy` of `CopyBlock8b` is a fallthrough when no pattern applies.
15. **`DecodeImageData`'s jumps.** The `goto AdvanceByOne` and `goto Error` of
    the decode loop are a labeled block that yields whether to advance, and a
    flag; the order of every read, write, check-point and row callback is C's.
16. **Lint expectations.** `VP8Io` keeps its flags (`struct_excessive_bools`);
    the C names `dst` and `dist`, `huffman_xsize` and `huffman_ysize`,
    `transform_xsize` and `transform_ysize` stay (`similar_names`); the
    pointer-alignment tests keep their masks (`verbose_bit_mask`); the two `None`
    branches of `VP8LGetInfo` stay apart (`if_same_then_else`); the debug
    assertion of `DecodeImageData` keeps its C form (`nonminimal_bool`);
    `ReadHuffmanCodesHelper` and `DecodeImageData` keep their length
    (`too_many_lines`), and functions with the C parameters plus the per-call
    context keep them (`too_many_arguments`). Each names the C construct
    (ADR 0005).

## Notes for the packages that build on WP7

- **WP6.** `VP8Io`, `VP8IoRows` and `VP8IoHooks` are in `src/dec/vp8_dec.rs`;
  WP6 added `VP8IoRows::a_offset` (`a` is the alpha plane from its first row, and
  `a_offset` the index of C's `io->a`) and appended its items below them. The VP8 entry points take the input
  and `Option<&mut dyn VP8IoHooks>` per call; `FinishRow` builds `VP8IoRows`
  from the cache and the alpha plane; `io_dec.c`'s `CustomSetup`, `CustomPut` and
  `CustomTeardown` implement `VP8IoHooks` for a short-lived pair of `params` and
  `output`; `alpha_dec.c` passes the alpha chunk after its header byte and the
  alpha plane to `vp8l_decode_alpha_header` and `vp8l_decode_alpha_image_stream`.
  `ALPHInit` calls `VP8FiltersInit`, which has no Rust counterpart. With
  `use_threads`, `FinishRow` (and with it `put` and the emit functions that write
  the caller's output) runs on the worker. WP1's `WebPWorker::reset` requires
  `D1, D2: Send + 'static`, and the output borrows the caller's memory for `'a`
  (ADR 0003, buffers and planes), so WP6 cannot move that context into the worker
  as it is; it needs a scoped thread or an amendment of ADR 0003's thread rule.
  This is noted for WP6's brief; it is not a WP7 decision.
- **WP12.** `DecodeInto` passes the input from the frame offset, `params` and
  `config.output`. `WebPIDecoder` keeps `io`, `params`, `dec`, `mem` and its
  output buffers as plain fields and passes disjoint borrows. For VP8L,
  `DoRemap` calls only `vp8l_bit_reader_set_buffer(&mut dec.br, len)`, and the
  next `vp8l_decode_image` receives the moved bytes at the same positions; the
  tests of WP7 drive the decoder exactly that way. `WebPResetDecParams` is
  `WebPDecParams::default()`, and `WebPAllocateDecBuffer` sizes the output that
  `vp8l_decode_image` writes.
