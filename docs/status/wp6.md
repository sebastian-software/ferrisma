# WP6: VP8 decoder core

- Status: every unit of the WP6 inventory is translated, except `VP8Delete`,
  which only clears and frees and is the owner's drop, and the `Init` calls
  that only assign portable pointers (ADR 0003). Each is listed under
  not-applicable checks or recorded deviations. The release, asan-ubsan and
  debug oracles agree with the translation on every compared case.
- Sub-wave: 2b
- Model: Claude Opus 5.5 (`claude-opus-5-5`)
- Depends on: WP1, WP2, WP4, WP7
- Probes: `tools/oracle/probes/wp6/`; tests: `tests/wp6_*.rs`

Written by the package session; the integrator aggregates it into
[PORT_STATUS.md](../PORT_STATUS.md). See the package's row in
[WORKPLAN.md](../WORKPLAN.md) for its verification inputs. The inventory
comes from `tools/units.json`, including the function- and type-level
exceptions inside a module.

## Decisions

The brief fixed the thread split; WP6 stopped at a checkpoint before
`frame_dec.c` with four questions, and the integrator decided on 2026-11-07:

- **Threads (Q1, option T2).** Nothing borrowed for the caller's lifetime enters
  `WebPWorker`. The worker job is `VP8RowJob` (`vp8i_dec.rs`): the thread
  context, the cache rows with their extra rows, `yuv_b`, `yuv_t` and the
  dithering generator, with copies of the decoder's scalars that `FinishRow`
  reads. It runs `ReconstructRow` for `mt_method == 2`, the deblocking, the
  dithering and the rotation of the top samples. The alpha-row decoding, the
  crop and `put()` run on the calling thread at the next `Sync` (in
  `VP8ProcessRow`, `ParseFrame` and `VP8ExitCritical`), for the row that the job
  finished, because the alpha decoding reads the caller's input and `put()`
  writes the caller's output. The `ALPHDecoder` and the alpha plane stay in
  `VP8Decoder`. ADR 0003's "Threads" paragraph records the rule (amended
  2026-11-07, WP6 and WP11).
- **Alpha input (Q2, option A).** `dec->alpha_data` points into the caller's
  input outside the VP8 payload, so it is a per-call argument
  `alpha_data: Option<&[u8]>` (C's `alpha_data[..alpha_data_size]`, `None` for
  `NULL`) of `vp8_decode`, `vp8_init_frame`, `vp8_process_row`,
  `vp8_exit_critical` and `vp8_decompress_alpha_rows`. The decoder stores
  neither field.
- **Alpha rows (Q3, option R1).** The RGB alpha emitters with fancy upsampling
  step back one row from `io->a`. WP7's `VP8IoRows` gained `a_offset`: `a` is the
  alpha plane from its first row and `a_offset` the index of C's `io->a`. Its own
  commit.
- **Flipped, scaled YUV output (Q4, option S1).** `io_dec.c` gives the
  rescalers the output planes with their strides, which are negative for a
  flipped buffer. WP4's `webp_rescaler_export_row` now advances `dst_pos` by the
  signed stride, and `dst_pos` is public, so that the owner can start at the
  first row; WP4's rescaler test covers flipped destinations. Its own commit,
  titled as a WP4 fix.
- **`VP8GetThreadMethod`** drops its `WebPHeaderStructure` parameter (WP12's
  type), which C reads only in `assert(headers == NULL || !headers->is_lossless)`;
  WP12 keeps the assertion at its call site.

## Inventory

Every row names what is translated; each item carries an `// upstream:` marker.
Names follow ADR 0001 (`tools/rust_name.py`). No module uses `unsafe`.

| Rust module | Upstream units | Translated |
| --- | --- | --- |
| `src/dsp/dec.rs` | `src/dsp/dec.c` | yes: `clip_8b`, `STORE`, `STORE2`, `TransformOne_C`, `TransformAC3_C`, `TransformTwo_C`, `TransformUV_C`, `TransformDC_C`, `TransformDCUV_C`, `TransformWHT_C`, `DST`, `TrueMotion`, `TM4_C`, `TM8uv_C`, `TM16_C`, `VE16_C`, `HE16_C`, `Put16`, `DC16_C`, `DC16NoTop_C`, `DC16NoLeft_C`, `DC16NoTopLeft_C`, `AVG3`, `AVG2`, `VE4_C`, `HE4_C`, `DC4_C`, `RD4_C`, `LD4_C`, `VR4_C`, `VL4_C`, `HU4_C`, `HD4_C`, `VE8uv_C`, `HE8uv_C`, `Put8x8uv`, `DC8uv_C`, `DC8uvNoLeft_C`, `DC8uvNoTop_C`, `DC8uvNoTopLeft_C`, `DoFilter2_C`, `DoFilter4_C`, `DoFilter6_C`, `Hev`, `NeedsFilter_C`, `NeedsFilter2_C`, `SimpleVFilter16_C`, `SimpleHFilter16_C`, `SimpleVFilter16i_C`, `SimpleHFilter16i_C`, `FilterLoop26_C`, `FilterLoop24_C`, `VFilter16_C`, `HFilter16_C`, `VFilter16i_C`, `HFilter16i_C`, `VFilter8_C`, `HFilter8_C`, `VFilter8i_C`, `HFilter8i_C`, `DitherCombine8x8_C`; the dispatch pointers `VP8Transform`, `VP8TransformAC3`, `VP8TransformUV`, `VP8TransformDC`, `VP8TransformDCUV`, `VP8TransformWHT`, `VP8PredLuma4`, `VP8PredLuma16`, `VP8PredChroma8`, `VP8VFilter16`, `VP8HFilter16`, `VP8VFilter8`, `VP8HFilter8`, `VP8VFilter16i`, `VP8HFilter16i`, `VP8VFilter8i`, `VP8HFilter8i`, `VP8SimpleVFilter16`, `VP8SimpleHFilter16`, `VP8SimpleVFilter16i`, `VP8SimpleHFilter16i`, `VP8DitherCombine8x8` as functions; the `dsp.h` types `VP8DecIdct`, `VP8DecIdct2`, `VP8PredFunc`, `VP8SimpleFilterFunc`, `VP8LumaFilterFunc`, `VP8ChromaFilterFunc`. `VP8DspInit` has no counterpart (not-applicable check 2). |
| `src/dsp/dec_clip_tables.rs` | `src/dsp/dec_clip_tables.c` | yes: `abs0`, `sclip1`, `sclip2`, `clip1` and the pointers `VP8ksclip1`, `VP8ksclip2`, `VP8kclip1`, `VP8kabs0` (deviation 3). `USE_STATIC_TABLES` is 1, so `VP8InitClipTables` has nothing to do (not-applicable check 2). |
| `src/dec/common_dec.rs` | `src/dec/common_dec.h` | yes: the intra mode enumerators `B_DC_PRED` to `NUM_B_DC_MODES` and the constants `MB_FEATURE_TREE_PROBS` to `NUM_PROBAS`, as `i32` constants (the enums are anonymous). `IsValidColorspace` is declared here and defined in `buffer_dec.c` (WP12). |
| `src/dec/vp8_dec.rs` | `src/dec/vp8_dec.c`, `src/dec/vp8_dec.h`; except `VP8Io`, `VP8IoPutHook`, `VP8IoSetupHook`, `VP8IoTeardownHook` (WP7) | yes, except `VP8Delete` (deviation 12): `WebPGetDecoderVersion`, `GetCoeffsFunc`, `GetCoeffs`, `InitGetCoeffs`, `SetOk`, `VP8InitIoInternal`, `VP8InitIo`, `VP8New`, `VP8Status`, `VP8StatusMessage`, `VP8SetError`, `VP8CheckSignature`, `VP8GetInfo`, `ResetSegmentHeader`, `ParseSegmentHeader`, `ParsePartitions`, `ParseFilterHeader`, `VP8GetHeaders`, `kCat3` to `kCat6`, `kCat3456`, `kZigzag`, `GetLargeValue`, `GetCoeffsFast`, `GetCoeffsAlt`, `NzCodeBits`, `ParseResiduals`, `VP8DecodeMB`, `VP8InitScanline`, `ParseFrame`, `VP8Decode`, `VP8Clear`. WP7's items gained `VP8IoRows::a_offset` (decision Q3). `VP8LCheckSignature` and `VP8LGetInfo`, which the header declares, are WP7's in `vp8l_dec.rs`. |
| `src/dec/vp8i_dec.rs` | `src/dec/vp8i_dec.h` | yes: `DEC_MAJ_VERSION`, `DEC_MIN_VERSION`, `DEC_REV_VERSION`, `YUV_SIZE`, `Y_OFF`, `U_OFF`, `V_OFF`, `MIN_WIDTH_FOR_THREADS`, `VP8FrameHeader`, `VP8PictureHeader`, `VP8SegmentHeader`, `VP8ProbaArray`, `VP8BandProbas`, `VP8Proba`, `VP8FilterHeader`, `VP8FInfo`, `VP8MB`, `quant_t`, `VP8QuantMatrix`, `VP8MBData`, `VP8ThreadContext`, `VP8TopSamples`, `VP8Decoder`, and the Rust-only `VP8RowJob` and `VP8AlphaPlane` (deviations 1 and 9). The functions it declares are defined in the `.c` files and translated there. |
| `src/dec/tree_dec.rs` | `src/dec/tree_dec.c` | yes: `USE_GENERIC_TREE` (chosen by target), `kYModesIntra4`, `CoeffsProba0`, `kBModesProba`, `CoeffsUpdateProba`, `kBands`, `VP8ResetProba`, `ParseIntraMode`, `VP8ParseIntraModeRow`, `VP8ParseProba`. |
| `src/dec/quant_dec.rs` | `src/dec/quant_dec.c` | yes: `clip`, `kDcTable`, `kAcTable`, `VP8ParseQuant`. |
| `src/dec/frame_dec.rs` | `src/dec/frame_dec.c` | yes: `kScan`, `CheckMode`, `Copy32b`, `DoTransform`, `DoUVTransform`, `ReconstructRow`, `kFilterExtraRows`, `DoFilter`, `FilterRow`, `PrecomputeFilterStrengths`, `MIN_DITHER_AMP`, `DITHER_AMP_TAB_SIZE`, `kQuantToDitherAmp`, `VP8InitDithering`, `Dither8x8`, `DitherRow`, `MACROBLOCK_VPOS`, `FinishRow` (split, decision Q1), `VP8ProcessRow`, `VP8EnterCritical`, `VP8ExitCritical`, `MT_CACHE_LINES`, `ST_CACHE_LINES`, `InitThreadContext`, `VP8GetThreadMethod`, `AllocateMemory`, `InitIo`, `VP8InitFrame`. |
| `src/dec/io_dec.rs` | `src/dec/io_dec.c` | yes: `EmitYUV`, `EmitSampledRGB`, `EmitFancyRGB`, `FillAlphaPlane`, `EmitAlphaYUV`, `GetAlphaSourceRow`, `EmitAlphaRGB`, `EmitAlphaRGBA4444`, `Rescale`, `EmitRescaledYUV`, `EmitRescaledAlphaYUV`, `InitYUVRescaler`, `ExportRGB`, `EmitRescaledRGB`, `ExportAlpha`, `ExportAlphaRGBA4444`, `EmitRescaledAlphaRGB`, `InitRGBRescaler`, `CustomSetup`, `CustomPut`, `CustomTeardown`, `WebPInitCustomIo`, and the Rust-only `CustomIo` (deviation 14). |
| `src/dec/alpha_dec.rs` | `src/dec/alpha_dec.c`; `WebPDeallocateAlphaMemory` declared in `alphai_dec.h` | yes: `ALPHNew`, `ALPHDelete`, `ALPHInit`, `ALPHDecode`, `AllocateAlphaPlane`, `WebPDeallocateAlphaMemory`, `VP8DecompressAlphaRows`. |

## Evidence

Every differential test runs the C probe in a separate process, against the
oracle built from the pinned `v1.6.0` source with `python3 tools/build_oracle.py`.
The tests generate a seeded script, compute the expected output with the Rust
translation, and compare it with the probe's output line by line.
`python3 tools/check.py --oracle` runs them with the `oracle` feature, so a
missing oracle fails instead of skipping. The counts are those of one run on
this host (macOS ARM64, Apple clang 21).

| Test file | Test | Cases | What it compares |
| --- | --- | ---: | --- |
| `tests/wp6_dsp.rs` | `decoder_dsp_matches_the_c_functions` | 8,241 | The four clip tables over their whole ranges (the implicit zeros included); `VP8Transform` (one and two blocks), `VP8TransformAC3`, `VP8TransformUV`, `VP8TransformDC`, `VP8TransformDCUV` on random blocks with coefficients up to ±15000, and `VP8TransformWHT` on any `int16_t` input; every mode of `VP8PredLuma4`, `VP8PredLuma16` and `VP8PredChroma8` on random and nearly flat neighbors; the four simple and eight complex loop filters on planes with noise, flat areas and steps, strides 16 to 64, thresholds up to 1200; `VP8DitherCombine8x8`. Every byte of the updated buffers. |
| `tests/wp6_headers.rs` | `frame_headers_match_the_c_parser` | 21,321 | `VP8GetHeaders` on the `VP8 ` payload of all 88 lossy vectors (the 56 `vp80-*` vectors and 32 others), in normal and incremental mode, in full, at every size up to 40 bytes, around the end of the first partition and of the partition size table and at random sizes; 24 mutations per vector (bit flips in the frame and picture headers, random bytes, the VP8 signature of the decoder fuzz dictionary inserted); `VP8GetInfo` at every cut and against chunk sizes around the partition length; `VP8CheckSignature`. After each call: the result, the status and message, every header field, `filter_type`, the quantizers of the four segments, the segment and skip probabilities, a digest of the coefficient probabilities, the band indexes, the first-partition and token-partition bit readers (value, range, bits, end flag and positions) and the io. Statuses OK, `BITSTREAM_ERROR`, `UNSUPPORTED_FEATURE`, `SUSPENDED` and `NOT_ENOUGH_DATA`. |
| `tests/wp6_io.rs` | `custom_io_matches_the_c_output_functions` | 1,560 | `WebPInitCustomIo`'s setup, put and teardown on seeded pictures of 1 to 40 by 1 to 40 pixels with random or smooth Y, U, V and optional alpha planes, fed in the row chunks that `FinishRow` computes for 0, 2 and 8 held-back rows, into output buffers that the test lays out (the C probe builds them the same way, since `WebPFlipBuffer` is WP12's): all 13 colorspaces, padded strides, flipped buffers, one allocation or one per plane; no options, options with point or fancy upsampling, valid and invalid crop windows, explicit, derived and invalid scaled sizes. The result of each call, the io after setup, `last_y` and every byte of the output allocations, which start filled with 0xa5. |
| `tests/wp6_decode.rs` | `vp8_decoder_matches_the_c_decoder` | 3,738 decodes, 19,926 puts | `VP8GetHeaders` and `VP8Decode` through `VP8Io` hooks that print each `put()` (position, size and the visible Y, U, V and alpha rows, in hex for small pictures and as 64-bit FNV-1a digests otherwise), on all 88 lossy vectors, with the ALPH chunk where there is one: `mt_method` 0, 1 and 2; bypassed filtering; dithering strengths (including 7 and 14, which give the smallest dithering amplitude) and alpha dithering strengths through `VP8InitDithering`; crop windows through `WebPIoInitFromOptions` in the setup hook; a failing setup hook; a `put()` that fails at a random row and at the last row (in the threaded path C then fails without recording a status); truncated ALPH chunks; truncations at and around the end of the frame header, of the first partition, of the size table and of every token partition; 6 mutations per vector in the partitions, screened as below. The header state before the decoding, the dithering flags, the setup and teardown calls, the result, the status and message, `ready`, `is_alpha_decoded` and the io. The 11158 by 2156 `bryce.webp` is decoded once, threaded and cropped. Statuses OK (1,410), `OUT_OF_MEMORY` from failed alpha headers (15), `BITSTREAM_ERROR` (20), `USER_ABORT` (305) and `NOT_ENOUGH_DATA` (598). |
| `tests/wp6_decode.rs` | `alpha_rows_match_the_c_decoder` | 8,460 calls | `VP8DecompressAlphaRows` in steps of 1, 7, 16 and 64 rows on the ALPH chunks of the 18 vectors that have one (uncompressed and lossless, every filter, pre-processed levels), with crop windows, alpha dithering 0, 30 and 100, requests outside the picture, and truncated chunks. The returned row position, the status, `is_alpha_decoded`, `alpha_dithering` and a digest of the decoded rows of the plane. |
| `tests/wp4_rescaler.rs` | `rescaler_matches_the_c_functions` | (WP4's test) | Extended for decision Q4: every rescaler case also writes into a flipped destination with a negative stride. |

Local results on this branch:

- `python3 tools/check.py --oracle --msrv` passes on the final tree: formatting,
  Clippy with `-D warnings` on all targets and features, the upstream pins, the
  module skeleton, the documentation links, `tools/unit_dependencies.py --check`,
  and the tests with the `oracle` feature in the dev, release and `checked`
  profiles and with Rust 1.95.
- The WP6 tests and WP4's rescaler test pass against the asan-ubsan oracle, which
  aborts on the first sanitizer report (`-fno-sanitize-recover=all`), and
  against the debug oracle, which keeps upstream's assertions; neither fired.
- Mutations of the Rust code, each run against the test that covers it and
  then reverted: 16 in all, 15 caught. They covered the 4x4 average rounding, the
  high-edge-variance test, the rotation of the top samples, the filter-strength
  thresholds, the sticky `put()` error of the threaded path, the DC mode at the
  left border, the chroma dithering mask, the large-value categories, the alpha
  row of the fancy upsampler, the fancy upsampler's held-back row, the alpha
  pre-processing switch, the chroma mode tree, the second-order AC quantizer
  floor, the partition size clamp and the top-left filter limit. The survivor,
  `data.dither >= MIN_DITHER_AMP` as `>`, is equivalent on the vectors: a block
  dithered with amplitude 4 changes no sample (the deltas are -2 to 1, and
  `(delta + 8) >> 4` is 0) and only advances the generator, which shows only when
  a block with a larger amplitude follows, and no vector has such a pair.

## Not-applicable checks

These checks do not apply to WP6, or cannot be made on this host, with the reason:

1. **`NULL` arguments.** `VP8GetHeaders` and `VP8Decode` with a `NULL` decoder or
   io, `VP8Status` and `VP8StatusMessage` on `NULL`, `VP8InitIoInternal` with a
   `NULL` io, `VP8GetInfo` with `NULL` data or out-parameters exist only at the C
   ABI (PORTING rule 8).
2. **`Init` functions.** `VP8DspInit`, `VP8InitClipTables` (static tables),
   `VP8FiltersInit` (in `ALPHInit`), `WebPInitUpsamplers`, `WebPInitSamplers`,
   `WebPInitYUV444Converters` and `WebPInitAlphaProcessing` (in `io_dec.c`) only
   assign portable pointers in the reference configuration, and ADR 0003 omits
   them with an `// upstream:` comment. `InitGetCoeffs` makes a CPU-dependent
   choice and is kept as a `LazyLock` (deviation 5).
3. **SIMD and NEON variants** (`dec_sse2.c`, `dec_sse41.c`, `dec_neon.c`) are
   wave 4. `WEBP_NEON_OMIT_C_CODE` is 0, so every `_C` function is compiled.
4. **`USE_GENERIC_TREE`.** The table-driven 4x4 mode parser is compiled on x86
   and the coded tree on ARM, AArch64 and WebAssembly, as in C; both read the
   same bits. This host runs the coded tree; the Linux x86-64 CI runs the table.
5. **`GetCoeffsAlt`** is selected only on x86 CPUs with slow SSSE3; this host
   and the CI hosts run `GetCoeffsFast`. Both decode the same values.
6. **Host refusals of an allocation.** The size tests of `WebPSafeMalloc` are
   applied with the C totals, but a refusal by the host of an accepted size
   cannot be provoked here; the Rust code returns `VP8_STATUS_OUT_OF_MEMORY`
   where C does. `VP8New` and `ALPHNew` allocate fixed-size objects, whose size
   test always passes; a refused host allocation aborts.
7. **Thread start failure.** `WebPWorker::reset` fails only when the host cannot
   start a thread; `InitThreadContext`'s `VP8_STATUS_OUT_OF_MEMORY` path is
   translated but not provoked.
8. **`WEBP_REDUCE_SIZE`** is unset and `FANCY_UPSAMPLING` defined in the reference
   configuration, so the rescaling and fancy upsampling paths are translated and
   their alternatives are not compiled. `WEBP_SWAP_16BIT_CSP` is 0.
9. **Big-endian hosts.** `HE4_C` stores a byte replicated four times, which has
   the same bytes in both orders; no other WP6 function depends on byte order.
10. **Floating point.** `VP8InitRandom(&dec->dithering_rg, 1.0f)` is WP1's; no
    WP6 function computes in floating point.

### Inputs excluded from the oracle (PORTING rule 8)

The C code is undefined or asserts on these inputs, so no test issues them:

1. **Coefficients that overflow `TransformOne`.** Its second pass multiplies the
   first-pass values by 35468 in `int`; a bitstream can code coefficients for
   which that overflows (the comment in `dec.c` assumes inputs in
   [-2048, 2047]). The DSP test keeps coefficients within ±15000, and the decode
   test screens each mutated stream: it parses every macroblock with the
   translation's public parser and simulates `TransformOne`'s arithmetic on every
   block in 64 bits. With an earlier seed the screen excluded one of 528
   mutations, and the asan-ubsan oracle confirmed the overflow there
   (`signed integer overflow: 62027 * 35468`); with the final seed it excludes
   none. The Rust code panics on such input in the dev and `checked` profiles
   and wraps in release.
2. **`mt_method == 1` with dithering.** `VP8GetThreadMethod` returns only 0 or 2.
   With 1, C dithers on the worker from `ctx->mb_data`, which is `dec->mb_data`
   while the parser overwrites it: a data race. The test runs `mt_method` 1 only
   without dithering (deviation 2).
3. **A second `VP8Decode` on a decoder whose alpha is decoded.** `AllocateMemory`
   points `alpha_plane` at the alpha part of the reused `dec->mem`, and with
   `is_alpha_decoded` set, `VP8DecompressAlphaRows` returns rows of it that were
   never written. The Rust frame memory is zeroed (deviation 9).
4. **The rows of the alpha plane that a decoding has not reached.** The plane is
   `malloc`ed; the alpha test prints only the decoded rows, from the crop top on
   (the lossless path may skip the rows above it).
5. **Output buffers smaller than the output, overlapping planes, or a
   `WebPDecBufferPlanes` variant that does not match the colorspace**, as in WP7;
   and a YUVA output without an A plane while the picture has alpha, which C
   writes through `NULL`.
6. **Parse failures after a successful `VP8Decode` without new headers**, and
   calls of `VP8ProcessRow` and `VP8ExitCritical` outside the pairing that
   `VP8EnterCritical` documents.

## Recorded deviations

Deviations are changes of representation that the ADRs allow, or behavior that
the Rust side cannot reproduce exactly. Each records the reason.

1. **Threaded row work (ADR 0003, "Threads", decision Q1).** `VP8RowJob` is the
   worker's data and lives in the worker, which is its home also without a
   thread; `VP8New` sets it with the hook once (C sets the hook in
   `InitThreadContext`). It holds the thread context, the cache rows, `yuv_b`,
   `yuv_t`, `dithering_rg` and copies of `mt_method`, `filter_type`, `dither`,
   `mb_w`, `mb_h`, `tl_mb_x`, `br_mb_x`, `br_mb_y`, `num_caches` and the cache
   strides, refreshed before each row. `intra_t` and `intra_l` are parse state on
   the calling thread; the worker mutates `yuv_t`. The hook runs `FinishRow`
   without its `put()` branch; that branch (the alpha rows, the crop and `put()`)
   runs on the calling thread at the next `Sync` with the thread context's copy
   of the io. Outputs, status codes, the order of hook calls and the point where
   a failure surfaces are C's; the overlap of the alpha decoding and `put()` with
   the next row's parsing is lost. WP1's worker cannot set `had_error` from
   outside, so a failed `put()` sets `put_error` on the decoder, which the later
   `Sync`s report as `had_error` does until `InitThreadContext` resets the
   worker. In the threaded path the rotation of the top samples runs in the job,
   before the `put()` that C runs first: with filtering there are 3 cache rows,
   the rotation happens only for cache row 2 and copies rows `48-e..48` to
   `-e..0` (e = 2 or 8), while the `put()` of that row reads `32-e..48-e`;
   without filtering e = 0. The single-threaded path keeps C's order, where the
   regions overlap. The alpha decoding's `VP8SetError` runs on the calling thread
   (C: on the worker, before the `Sync` that reports it).
2. **`mt_method == 1`** gives the thread context its own macroblock data and
   swaps it with the parser's, as `mt_method == 2` does; C shares one buffer
   between them (excluded input 2). In the single-threaded path the decoder's
   `mb_data` and `f_info` move into the thread context for the row, where C
   points both at the same memory.
3. **Clip tables.** The tables are static arrays with C's values, and each
   exported pointer into their middle is a `CenteredTable` indexed with C's signed
   offsets. `sclip1` and `sclip2` hold the `int8_t` values that C reads through
   the cast pointers. C initializes 2040 of the 2041 entries of `sclip1`, 224 of
   225 of `sclip2` and 766 of 767 of `clip1`; the last entry is an implicit 0 in
   C and an explicit 0 here, and the DSP test compares it. No decoded input
   reaches those entries.
4. **Pointers into blocks and planes (ADR 0003).** A pointer that the C code only
   moves forward is a slice from it; one from which it also reads backwards (the
   predictors' `dst[-1]` and `dst[-BPS]`, the filters' `p[-2 * step]`) is a slice
   and a position. Positions outside the slice panic, where C reads or writes
   outside the buffer (WP4, deviation 6). The bit readers of the first partition
   and of the token partitions hold positions in `data`, the VP8 payload:
   `VP8InitBitReader(br, buf + start, size)` is WP2's init on the bytes followed
   by `VP8RemapBitReader` by `start`.
5. **`GetCoeffs`** is a `LazyLock` of the function that `InitGetCoeffs` selects
   from the CPU query (C: the `volatile` pointer that `VP8New` sets). The
   coefficient probabilities are the bands of one type and the indexes of
   `bands_ptr` into them, where C passes an array of pointers; `bands_ptr` starts
   at 0 where C has `NULL`, and nothing reads it before `VP8ParseProba` sets it
   (the probes print it only when the decoder is ready).
6. **Per-call context (ADR 0003).** `VP8GetHeaders`, `VP8Decode`,
   `VP8ParseProba`, `VP8ParseIntraModeRow`, `VP8ParseQuant` and `VP8DecodeMB`
   take `data`, C's `io->data[..io->data_size]`; the functions that emit rows take
   `alpha_data` and `Option<&mut (dyn VP8IoHooks + '_)>` (decision Q2). C's
   `VP8ParseProba(br, dec)` and `VP8ParseIntraModeRow(br, dec)` receive
   `&dec->br` from every caller and read `dec.br` here; `VP8DecodeMB` takes the
   index of the token partition, which every caller computes as
   `mb_y & num_parts_minus_one`.
7. **Errors (ADR 0003).** `VP8SetError` keeps its precedence and returns the
   status that the decoder holds afterwards. `VP8GetHeaders` and
   `VP8EnterCritical` return `Result<(), VP8StatusCode>` with the recorded status.
   `VP8Decode`, `VP8InitFrame`, `VP8ProcessRow`, `VP8ExitCritical`, `ParseFrame`
   and `VP8DecodeMB` return `bool`, because C can fail there without recording a
   status (a failed `put()` of the last threaded row, a size test that does not
   fit), so the status field is the only record, as in C. `VP8GetInfo` returns
   `Option<(width, height)>`, since C writes them only on success.
   `VP8DecompressAlphaRows` returns `Option<usize>`, the position of the row in the
   alpha plane, `None` where C returns `NULL`. `error_msg` is
   `Option<&'static str>`.
8. **Frame memory (ADR 0003, "Allocation limits").** `AllocateMemory` applies the
   size test of C's total, alignment slack included, when C would allocate, then
   allocates the parts separately and zeroed: `intra_t`, `mb_info` (`mb_w + 1`
   entries, index 0 is C's `mb_info[-1]`), `f_info`, `mb_data`, the alpha part,
   and the job's `yuv_t`, `yuv_b`, the thread context's halves and the three cache
   planes with their extra rows. C reuses its block when it is large enough;
   every part that C does not initialize is written before it is read, so fresh
   parts decode the same. `mem_size` keeps C's size for that decision. The C sizes
   of the element types are `C_SIZE` constants (`VP8MBData` 800, `VP8TopSamples`
   32, `VP8FInfo` 4, `VP8MB` 2).
9. **Alpha plane.** `dec->alpha_plane` is `VP8AlphaPlane`: none, the alpha part
   of the frame memory (`frame_alpha`, which `AllocateMemory` carves and nothing
   decodes into) or `alpha_plane_mem`.
10. **`VP8IoRows`** (decision Q3) carries the whole alpha plane and the position
    of C's `io->a`. `y` is the cache from the row on; `u` and `v` likewise.
11. **Hooks.** C tests `io->put != NULL` and the others; the Rust calls test
    `Option` and `has_put`, `has_setup` and `has_teardown` (WP7's trait). The
    trait objects take their own lifetime (`dyn VP8IoHooks + '_`), so that the
    calls can reborrow them.
12. **`VP8New` and `VP8Delete`.** `VP8New` returns a `Box`; `VP8Delete` only
    clears and frees, so the owner drops the box, and dropping the worker ends its
    thread. `VP8Clear` drops a `put()` that was not synced, which no caller leaves
    (they call `VP8ExitCritical` first).
13. **Flipped output in `io_dec.c`.** WP4's helpers that take a stride accept
    only non-negative strides (WP4, deviation 6). Where C passes the negative
    stride of a flipped buffer to `WebPDispatchAlpha`, `WebPApplyAlphaMultiply`,
    `WebPApplyAlphaMultiply4444`, `WebPMultRows` or `WebPSamplerProcessPlane`,
    `io_dec.rs` calls them one row at a time with C's row arithmetic. Row
    positions are computed per row from the plane's offset, so no position is
    formed past the last row. The rescalers receive the plane's first row as
    `dst_pos` and the signed stride (decision Q4). In `EmitRescaledAlphaYUV`, C
    reads the alpha rows from the output while it writes the luma rows; the
    planes may share one region here, so the alpha rows are copied first.
14. **`WebPInitCustomIo`** returns `CustomIo`, which borrows the parameters and the
    output buffer and implements `VP8IoHooks`; `VP8Io` has no hook fields, so `io`
    is left as it is. `ALPHInit`'s `WebPInitCustomIo(NULL, io)` has nothing to
    set. `p->memory` holds only what C carves besides the rescalers: the fancy
    upsampler's rows, or the RGB rescalers' output rows (the rescalers own their
    work rows and point at their row through `dst_pos`). The size test applies to
    C's total, rescaler structs and alignment included. `CustomTeardown` drops the
    rescalers with the memory, where C leaves their pointers dangling.
15. **Lint expectations.** C names (`similar_names`, `many_single_char_names`),
    long C functions (`too_many_lines`), C parameter lists (`too_many_arguments`),
    the casts that C performs implicitly (`cast_possible_truncation`,
    `cast_sign_loss`, `cast_possible_wrap`), `v += v + bit`
    (`misrefactored_assign_op`), the decoder's `int` flags
    (`struct_excessive_bools`), the condition order of C (`if_not_else`) and the
    boxed alpha decoder (`unnecessary_box_returns`). Each names the C construct
    (ADR 0005).

## Notes for the packages that build on WP6

- **WP12 (decoder integration).** `DecodeInto` calls
  `vp8_get_headers(dec, &mut io, data)` with `data` from the frame offset, sets
  `dec.mt_method = vp8_get_thread_method(params.options, io.width, io.height)`
  (keep `assert(!headers.is_lossless)` there), calls
  `vp8_init_dithering(params.options, dec)` and
  `vp8_decode(dec, &mut io, data, alpha_data, Some(&mut hooks))`, where
  `alpha_data` is the ALPH payload slice (`None` without ALPH) and `hooks` is
  `webp_init_custom_io(&mut params, &mut output, &mut io)`. On failure read
  `dec.status`, as C does; it can be OK after a failed `put()` of the last
  threaded row. `WebPIDecoder` keeps `io`, `params`, `dec`, `mem` and the output
  side by side and passes disjoint borrows: `vp8_enter_critical(dec, io, hooks)`,
  `vp8_init_frame(dec, io, alpha_data)`, then per row `vp8_parse_intra_mode_row`,
  `vp8_decode_mb(dec, token_br, data)` for each macroblock, `vp8_init_scanline`
  and `vp8_process_row(dec, io, alpha_data, hooks)`, and `vp8_exit_critical`. The
  same `data` and `alpha_data` must reach every call of one decoding; `alpha_data`
  is the bytes in `mem`, so `DoRemap` needs no alpha update (C moves
  `dec->alpha_data`), and `NeedCompressedAlpha` tests `alpha_data.is_some()` and
  `!dec.is_alpha_decoded`. Remap `dec.br` and `dec.parts` with index deltas (WP2).
  `RestoreContext` copies `dec.br` and `dec.parts[token_br]` field by field
  (WP2's reader has no `Clone`). `VP8Clear` before an `End` with a pending row is
  never needed: call `vp8_exit_critical` first, as idec's `IDecError` does.
  `IsValidColorspace` is declared in `common_dec.h` and defined in `buffer_dec.c`.
- **WP10 (VP8 encoder).** The DSP of `dec.c` is in `src/dsp/dec.rs`: transforms
  take a slice from the block, predictors and filters a slice and a position. The
  chroma filters take the U and V planes separately; where the encoder holds both
  in one buffer, split it with `split_at_mut`. The clip tables are
  `CenteredTable`s indexed with C's signed offsets.
- **WP16 (safe API).** A decoder borrows nothing between calls; the hooks, the
  output and the input are per-call arguments. `VP8IoRows::a` is the alpha plane
  from its first row with the position `a_offset`, so a user `put()` reads
  `a[a_offset..]` where C reads `io->a`.
