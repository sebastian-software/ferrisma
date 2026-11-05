# libwebp port work plan for parallel sessions (5 November 2026)

**Status:** WP0 (bootstrap) done on 5 November 2026; waves 1 to 5 not started. This plan decomposes the [libwebp port plan](PORT_PLAN.md) into work packages that independent Claude Code sessions can execute in parallel, each in its own worktree branch of this repository. It moved here from the Ferrixel repository (`docs/research/2026-11-05-libwebp-port-workplan.md` at commit `2aa4f3d1bb226d9541a4e9270168b03d17cac2ec`) during WP0, with links and paths made concrete. After WP0's first report the maintainer adjusted the plan: the test data is pinned at `260804d`, WP12 owns the `fuzz/` crate, and the packages run in sub-waves ordered by the call graph, with a few function- and type-level exceptions; [Findings from WP0](#findings-from-wp0) records why. Line counts are non-blank, non-comment lines of libwebp 1.6.0; they size the packages and say nothing about duration.

## How the plan works

- **One package, one session, one branch, one pull request.** Every package lists the upstream units it translates and the Rust modules it owns. Two packages never own the same file, apart from the function- and type-level exceptions in `tools/units.json`, so sessions of the same sub-wave run without coordination.
- **Sub-waves order the dependencies.** A package starts when the packages it depends on are merged; the sub-waves below are the order in which that happens, derived from the oracle's actual call graph (`tools/unit_dependencies.py`). Inside a sub-wave the packages are independent, except that WP5 merges after WP4. Fewer sessions than packages is fine; the critical path below says what to start first.
- **Shared ground rules replace coordination.** Each brief carries the same rules (below), the representation decisions from the bootstrap package and the differential-oracle harness. A session that needs something another package owns stops and reports instead of editing it.
- **An integrator merges by wave.** The integrator (maintainer or one session) rebases, runs `tools/check.py`, reviews source correspondence and merges in wave order, then aggregates `docs/PORT_STATUS.md` from the per-package status files.
- **Two model tiers.** Each package runs on Claude Opus 5.5 (`claude-opus-5-5`) when it designs, integrates, exports a C ABI or translates a codec core, and on Claude Haiku 5.5 (`claude-haiku-5-5`) when it is a mechanical, fully specified translation that the differential tests check exhaustively. The assignment is in the catalog below and in every brief.

## Ground rules for every session

1. Mechanical port of the pinned `upstream/libwebp` (v1.6.0, `4fa21912338357f89e4fd51cf2368325b59e9bd9`): preserve algorithms, tables, constants, numeric widths, operation order, status codes and partial state on error. No redesign, no tuning, no new optimizations, no SIMD.
2. One Rust module per C file with the C name (`src/dec/vp8_dec.c` becomes `src/dec/vp8_dec.rs`). Every translated function carries a source reference (`// upstream: src/dec/vp8_dec.c VP8GetHeaders`). Keep Google's copyright notice in each module.
3. Edit only the files the package owns plus `docs/status/<wp-id>.md`. Module files for every unit exist from the bootstrap with their provenance header only, so no package edits `mod.rs` or `lib.rs`. Where `tools/units.json` lists an exception, the named package appends exactly those functions or types to another package's module, and every later owner of that module keeps them unchanged.
4. No stubs, no placeholder APIs, no C fallback, no skipped parity cases, no changed expected outputs, no blanket warning suppressions. Unported functions stay absent.
5. `#![deny(unsafe_code)]` applies everywhere except the `src/abi` modules, which carry a local safety argument per operation. This is stricter than libjxl-rs, which uses `unsafe` with local safety arguments inside the codec; a unit that cannot meet it stops and asks for an ADR instead of adding `unsafe`. Ordinary `cargo build` compiles and links no C.
6. Verification is differential: a C probe compiled against the oracle build runs in a separate process and the Rust test compares exact bytes and status codes. A Rust round trip proves nothing. Debug, Release and the `checked` profile (overflow checks on) must pass.
7. No new Cargo dependencies. Development dependencies only if the bootstrap ADR lists them. The one planned exception is the separate `fuzz/` crate: WP12 creates it with `libfuzzer-sys` as its only dependency and amends ADR 0005 to name it; it is never a dependency of the library. WP13 and WP14 add their fuzz targets only after WP12 is merged.
8. Undefined or ambiguous C behavior is recorded in the status file and excluded from the oracle explicitly, never silently defined.
9. Conventional Commits in US English, unsigned (`--no-gpg-sign`), small coherent commits. Do not push to `main`, do not publish, do not advance submodules, do not create remote repositories.
10. Done means: all listed units translated, differential tests green on the host in all three profiles, `tools/check.py` green, `docs/status/<wp-id>.md` written with inventory, evidence, not-applicable checks and recorded deviations, pull request opened with that summary.

## Repository layout fixed by the bootstrap

```text
libwebp-rs/
  Cargo.toml                    package libwebp-rs, lib name webp, rlib + staticlib + cdylib,
                                checked profile, oracle feature; no dependencies
  rust-toolchain.toml           stable with rustfmt and Clippy (rust-version 1.95)
  LICENSE  PATENTS  AUTHORS     unchanged copies of upstream COPYING, PATENTS and AUTHORS
  AGENTS.md  CONTRIBUTING.md  README.md
  upstream.json                 pins of both submodules
  upstream/libwebp              submodule at v1.6.0
  upstream/libwebp-test-data    submodule at 260804d686f0491e0326a50eef491829dae09e9e
  src/lib.rs                    fixed by WP0: lint policy and top-level modules
  src/webp/                     public types: types, format_constants, decode, encode, demux,
                                mux_types (WP1) and mux (WP15)
  src/utils/  src/dsp/  src/dec/  src/enc/  src/demux/  src/mux/  src/sharpyuv/
                                one module per upstream unit, header only until its package lands
  src/abi/                      C ABI export per header: types, decode, encode, demux, mux;
                                the only unsafe code
  tools/units.json              upstream unit -> module -> package, sub-waves, dependencies,
                                function- and type-level exceptions, reviewed includes
  tools/check.py                fmt, Clippy, tests in three profiles, provenance, skeleton,
                                links, MSRV and oracle modes
  tools/verify_upstream.py      submodule pins and license copies
  tools/units.py                module skeleton and provenance headers
  tools/rust_name.py            Rust names under the ADR 0001 naming rule
  tools/unit_dependencies.py    declared versus actual cross-package references
  tools/build_oracle.py         oracle build per variant into build/oracle/<variant>/
  tools/oracle_baseline.py      test_dwebp.sh and test_lossless.sh against the oracle's dwebp
  tools/oracle/CMakeLists.txt   oracle library from upstream in the reference configuration
  tools/oracle/probe_support/   shared probe helpers (WP0)
  tools/oracle/probes/<wp-id>/  C probes per package; optional support/ and oracle.cmake
  tests/support/oracle.rs       locates the oracle, runs a probe, parses its key=value output
  tests/<wp-id>_*.rs            differential tests per package
  fuzz/                         cargo-fuzz targets in a separate crate, created by WP12;
                                WP13, WP14 and WP15 add theirs after WP12 is merged
  docs/PORTING.md  PORT_STATUS.md  PROVENANCE.md  WORKPLAN.md  PORT_PLAN.md  adr/
  docs/status/<wp-id>.md        one status page per package
```

A package's `<wp-id>` is its number in lower case: WP6 is `wp6`, with the status
page `docs/status/wp6.md`, probes in `tools/oracle/probes/wp6/`, tests in
`tests/wp6_*.rs` and the branch `<prefix>/wp6`. `tools/units.json` lists every
module with its owning package. WP21 and WP22 live in the Ferrixel repository.

## Waves

| Sub-wave | Packages | Parallel sessions | Unblocks |
| --- | --- | --- | --- |
| 0 | WP0 bootstrap | 1 | everything |
| 1a | WP1 types, memory and workers | 1 | 1b |
| 1b | WP2 bit I/O, WP4 pixel ops, WP5 SharpYUV (merges after WP4) | up to 3 | 1c, 2a |
| 1c | WP3 lossless shared | 1 | 2a |
| 2a | WP7 VP8L decoder, WP9 encoder picture and config | up to 2 | 2b |
| 2b | WP6 VP8 decoder core, WP11 VP8L encoder | up to 2 | 2c, 3a |
| 2c | WP10 VP8 encoder core | 1 | 3b |
| 3a | WP12 decoder integration | 1 | 3b, 3c, 4 |
| 3b | WP8 demux, WP13 encoder integration | up to 2 | 3c, 4 |
| 3c | WP14 animation decode and demux ABI | 1 | 4 |
| 4 | WP15 mux, WP16 safe decoder API, WP17 safe encoder API, WP18 to WP20 SIMD | as wanted | Ferrixel adapters |
| 5 | WP21 Ferrixel `webp-read`, WP22 Ferrixel `webp-write` (in the Ferrixel repository) | 2 | Ferrixel WebP support |

Each package's dependencies are in its catalog row and in `tools/units.json`; a package may start as soon as they are merged, so WP12 can run next to WP10.

**Critical path to Ferrixel's first WebP milestone (reading):** WP0 → WP1 → WP2, WP4 → WP3 → WP7 → WP6 → WP12 → WP8 → WP14 → WP16 → WP21.

**Start order with three sessions:** WP1; then WP2, WP4, WP5; then WP3; then WP7, WP9; then WP6, WP11; then WP10, WP12; then WP8, WP13; then WP14, WP17; then WP16, WP15; then WP21, WP22 and the SIMD packages as wanted.

## Model assignment

| Tier | Packages | Why |
| --- | --- | --- |
| Claude Opus 5.5 (`claude-opus-5-5`) | WP0, WP3, WP6, WP7, WP10, WP11, WP12, WP13, WP14, WP15, WP16, WP17, WP18, WP19, WP20, WP21, WP22, the integrator | Design decisions and ADRs (WP0, WP16, WP17, WP21, WP22); codec cores with entropy coding, prediction, trellis and rate loops (WP6, WP7, WP10, WP11); Huffman table construction that both VP8L paths depend on (WP3); integration with `unsafe` C ABI export and original consumers (WP12, WP13, WP14, WP15); `unsafe` intrinsics with parity gates (WP18 to WP20); review of source correspondence |
| Claude Haiku 5.5 (`claude-haiku-5-5`) | WP1, WP2, WP4, WP5, WP8, WP9 | Mechanical transcription of types, bit I/O, pixel operations, SharpYUV, chunk parsing and picture import; each is small, has no design freedom and is checked exhaustively by its probes, so a slip surfaces in the differential tests rather than downstream |

A Haiku package that meets a design question (for example an allocation-limit representation not covered by the WP0 ADRs) stops and reports instead of deciding; the integrator or an Opus session answers it.

## Package catalog

### WP0: bootstrap (sequential, about 2 days of a single session's work before any fan-out)

- **Creates:** the repository as laid out above, modeled on libjxl-rs: `AGENTS.md`, `CONTRIBUTING.md`, `docs/PORTING.md`, `docs/PROVENANCE.md`, `docs/PORT_STATUS.md` (links to `docs/status/*.md`), `docs/WORKPLAN.md` (this document), both submodules, `upstream.json`, `LICENSE`, `PATENTS`, CI (Linux x86-64 and macOS ARM64; ordinary, sanitizer-oracle and MSRV jobs), `tools/check.py` (fmt, clippy with `-D warnings`, tests in three profiles, provenance check of the submodules).
- **Module skeleton:** every `mod.rs` and every unit's `.rs` file with its provenance header and nothing else, so wave 1 and 2 sessions never touch shared files.
- **Oracle harness:** `tools/build_oracle.py` builds the oracle in the reference configuration from the port plan (`WEBP_ENABLE_SIMD=OFF`, threads on, `-ffp-contract=off`, Release plus ASan/UBSan variants) and the probe executables under `tools/oracle/probes`; `tests/support/oracle.rs` locates them through `LIBWEBP_ORACLE_DIR`, runs them and compares. One example probe and one example differential test document the pattern. Baseline recorded: `test_dwebp.sh` 480 checksums and `test_lossless.sh` 33 vectors against the oracle's own `dwebp`.
- **ADRs to record (recommended answers):** (1) representation: byte offsets and slices instead of pointers, owned buffers instead of `malloc`/`free` pairs, `Result<T, VP8StatusCode>` and `Result<T, WebPEncodingError>` with the original numeric codes, a fallible allocation helper that mirrors `WebPSafeMalloc` limits; (2) dispatch: portable DSP functions are plain functions behind the C names, `VP8GetCPUInfo` stays a query, SIMD arrives in wave 4 behind runtime selection; (3) ABI: `src/abi` mirrors the C structs with `repr(C)` and converts to the safe representations at the boundary; (4) threads: `WebPWorker` on `std::thread` with the same state machine; (5) toolchain: `rust-version = "1.95"`, edition 2024, `#![deny(unsafe_code)]`.
- **Done:** `cargo build`, `tools/check.py` and the example differential test pass on both CI targets; the oracle baseline is recorded in `docs/PORT_STATUS.md`.
- **Result (5 November 2026):** done on the macOS ARM64 host; the CI workflow exists but has not run, because the repository has no remote yet. The ADRs are [0001](adr/0001-mechanical-port.md) to [0005](adr/0005-toolchain-and-unsafe-policy.md); the record is [docs/status/wp0.md](status/wp0.md).

### Wave 1

| Package | Upstream units (lines) | Depends on (sub-wave) | Provides to later packages | Verification |
| --- | --- | --- | --- | --- |
| **WP1 types, memory, workers** (about 1,550) | `src/webp/types.h`, `format_constants.h`, `decode.h`, `encode.h`, `demux.h`, `mux_types.h` as Rust types and enums with original field order (773); `src/utils/utils.c/.h` (324) except `WebPGetColorPalette` (WP3), `endian_inl_utils.h` (73), `random_utils` (59), `thread_utils` (319) | WP0 (1a) | `WebPDecoderConfig`, `WebPDecBuffer`, `WebPBitstreamFeatures`, `WebPConfig`, `WebPPicture`, `WebPAuxStats`, `WebPMemoryWriter` and the demux/mux enums; the allocation helper with `WEBP_MAX_ALLOCABLE_MEMORY`; `VP8Random`; the worker | Probes for `WebPSafeMalloc` limit decisions on boundary sizes, `VP8Random` sequences for seeded states, worker state transitions including `Sync` after failure; struct size and field offsets compared with the C `sizeof`/`offsetof` for the ABI mirror |
| **WP2 bit I/O** (about 870) | `bit_reader_utils.c/.h`, `bit_reader_inl_utils.h` (501), `bit_writer_utils.c/.h` (372) | WP1 (1b) | `VP8BitReader`, `VP8LBitReader`, `VP8BitWriter`, `VP8LBitWriter` | Probes reading seeded and truncated streams with recorded bit positions, `eos` behavior and 64-bit refill boundaries; writers compared on byte output and overflow handling |
| **WP3 lossless shared** (about 1,940) | `huffman_utils` (283), `huffman_encode_utils` (351), `color_cache_utils` (82), `palette` (342), `dsp/lossless.c/.h`, `lossless_common.h` (876); exceptions: `WebPGetColorPalette` in `src/utils/utils.rs`, the `VP8LTransform` type in `src/dec/vp8li_dec.rs` | WP1, WP4 (1c) | Huffman table construction and lookup, Huffman tree building for the encoder, `VP8LColorCache`, palette sorting and search, predictors, color transforms and the shared `VP8LDspInit` functions | Probes on fuzzed code-length sets including rejected ones, cache operations, palette orders, and every predictor and transform on seeded ARGB rows |
| **WP4 pixel ops** (about 2,850) | `dsp/dsp.h`, `cpu.c/.h` (697), `yuv.c/.h` (352), `upsampling.c` (280), `alpha_processing.c` (415), `filters.c` plus `utils/filters_utils` (266), `rescaler.c` plus `utils/rescaler_utils` (385), `utils/quant_levels_utils` and `quant_levels_dec_utils` (328), `ssim.c` (124) | WP1 (1b) | YUV to RGB conversions for every `WEBP_CSP_MODE`, upsamplers, premultiply and alpha extraction, alpha filters and filter estimation, `WebPRescaler`, `QuantizeLevels` and `WebPDequantizeLevels`, `VP8SSIM*`, `VP8GetCPUInfo` | Probes on seeded planes with padded strides, odd sizes and all colorspace modes; rescaler on seeded scale factors including 1:1 and extreme downscales; filter estimation on seeded images |
| **WP5 SharpYUV** (about 1,140) | `sharpyuv/sharpyuv.c` (460), `sharpyuv_csp.c` (83), `sharpyuv_dsp.c` (76), `sharpyuv_gamma.c` (358), `sharpyuv_cpu.c` (2), headers (164) | WP1; WP4 for `sharpyuv_cpu.c`, which includes `src/dsp/cpu.c` to create `SharpYuvGetCPUInfo`, and for `SharpYuvInit`, which assigns it (1b, merges after WP4) | `SharpYuvConvert`, `SharpYuvConvertWithOptions`, matrices and transfer functions | The 1,271-group inventory of libavif-rs `SHARPYUV-VALIDATION.md` reproduced as probes; `libavif-sharpyuv` optionally as a second oracle |

### Wave 2

WP8 is listed here by subject; it runs in sub-wave 3b because the demuxer validates frames with `WebPGetFeaturesInternal` from WP12.

| Package | Upstream units (lines) | Depends on (sub-wave) | Verification |
| --- | --- | --- | --- |
| **WP6 VP8 decoder core** (about 3,910) | `src/dec/vp8_dec.c` (591), `tree_dec.c` (500), `quant_dec.c` (87), `frame_dec.c` (624), `vp8_dec.h` except `VP8Io` and its hooks (WP7), `vp8i_dec.h`, `common_dec.h` (273), `alpha_dec.c` (183), `io_dec.c` (557), `dsp/dec.c` (752), `dec_clip_tables.c` (342) | WP1, WP2, WP4, WP7 (2b) | A probe that drives `VP8GetHeaders`/`VP8Decode` through `VP8Io` with a put callback dumping reconstructed Y, U, V rows, with and without in-loop filtering bypass, with dithering strengths, over the 56 `vp80-*` vectors, truncations at every partition boundary and the decoder fuzz dictionary; the decoded alpha planes of the `alpha_*` and `lossy_alpha*` vectors through `VP8DecompressAlphaRows`; `io_dec.c` through `WebPInitCustomIo` into output buffers that the test allocates, for every `WEBP_CSP_MODE`; exact plane equality and status codes |
| **WP7 VP8L decoder** (about 1,640) | `vp8l_dec.c` (1,436), `vp8li_dec.h` (82) except `VP8LTransform` (WP3), `alphai_dec.h` (29); exceptions: `VP8Io` and its hook types in `src/dec/vp8_dec.rs`, `WebPDecParams` and its output callback types in `src/dec/webpi_dec.rs`, `WebPIoInitFromOptions` with `WebPCheckCropDimensions` and `WebPCheckCropDimensionsBasic` in `src/dec/webp_dec.rs` (55) | WP1, WP2, WP3, WP4 (2a) | Probe dumping decoded ARGB rows for the 32 `lossless_vec_*` vectors, `lossless_color_transform.webp` and the alpha, color-cache and `bad_palette_index` vectors; truncations; `VP8LGetInfo` on headers; `WebPIoInitFromOptions` on cropping and scaling options |
| **WP8 demux** (about 760) | `src/demux/demux.c` (761) | WP1, WP12 (3b) | Probe iterating frames and chunks (`ICCP`, `EXIF`, `XMP`, unknown) for every vector and for synthetic VP8X files with each flag combination, partial data states (`WEBP_DEMUX_PARSING_HEADER` onward) and malformed chunk sizes; exact iterator results |
| **WP9 encoder picture and config** (about 1,700) | `src/enc/config_enc.c` (123), `picture_enc.c` (229) except the simple encoding API (WP13), `picture_csp_enc.c` (696), `picture_rescale_enc.c` (238), `picture_tools_enc.c` (236), `picture_psnr_enc.c` (204); exception: `WebPEncodingSetError` and `WebPReportProgress` in `src/enc/webp_enc.rs` | WP1, WP3, WP4, WP5 (2a) | Probes for `WebPConfigInit` presets and `WebPValidateConfig` on boundary values, `WebPPictureImport*` from RGB/RGBA/BGRA with strides, `WebPPictureARGBToYUVA` with and without sharp YUV and dithering, `YUVAToARGB`, rescale and crop, `WebPPictureDistortion` metrics, `WebPMemoryWriter`, the first-error precedence of `WebPEncodingSetError`; exact planes and metrics |
| **WP10 VP8 encoder core** (about 5,960) | `analysis_enc.c` (364), `iterator_enc.c` (326), `cost_enc.c/.h` (327), `quant_enc.c` (1,051), `tree_enc.c` (474), `token_enc.c` (211), `syntax_enc.c` (297), `frame_enc.c` (741), `filter_enc.c` (178), `alpha_enc.c` (341), `webp_enc.c` except the WP9 and WP13 exceptions (the encoder setup: `InitVP8Encoder`, `DeleteVP8Encoder`, `MapConfigToTools`, the `Reset*` helpers, `WebPGetEncoderVersion`), `vp8i_enc.h` (298), `dsp/enc.c` (722), `dsp/cost.c` (372), `dsp/quant.h` (64) | WP1, WP2, WP4, WP6, WP9, WP11 (2c) | `WebPEncode` arrives with WP13, so the differential test drives the encoder steps from test code in the order `WebPEncode` calls them (`InitVP8Encoder`, `VP8EncAnalyze`, `VP8EncStartAlpha`, `VP8EncLoop` or `VP8EncTokenLoop`, `VP8EncFinishAlpha`, `VP8EncWrite`, `DeleteVP8Encoder`) on YUV pictures without alpha produced by the oracle, and compares the written bytes with the oracle's `WebPEncode` output over quality, method, presets, segments, partitions, SNS and filter settings, `pass`, `target_size`, `target_PSNR`, `emulate_jpeg_size`, `low_memory`, `thread_level`; this is test code, not a library stub; `WebPAuxStats` follow with WP13 |
| **WP11 VP8L encoder** (about 5,830) | `vp8l_enc.c` (1,566), `predictor_enc.c` (854), `histogram_enc.c/.h` (951), `backward_references_enc.c/.h` (980), `backward_references_cost_enc.c` (573), `near_lossless_enc.c` (115), `vp8li_enc.h` (74), `dsp/lossless_enc.c` (712) | WP1, WP2, WP3, WP4, WP9 (2b) | Probe calling `VP8LEncodeImage` (writes the complete RIFF file itself) on ARGB pictures over quality, method, `exact`, `near_lossless`, `use_delta_palette` and `thread_level`; byte-identical files |

### Wave 3

| Package | Upstream units (lines) | Depends on (sub-wave); takes over | Verification and gates |
| --- | --- | --- | --- |
| **WP12 decoder integration** (about 1,650 plus ABI) | `webp_dec.c` (657) except the WP7 exceptions, `buffer_dec.c` (258), `idec_dec.c` (735), `webpi_dec.h` (61) except the WP7 types; `src/abi/decode.rs` exporting `decode.h` and `src/abi/types.rs` exporting `WebPMalloc` and `WebPFree` | WP6, WP7 (3a); takes over their modules for integration fixes; `tools/oracle` consumer build of `dwebp`, `webpinfo` and `imageio` against the Rust static library | `test_dwebp.sh` 480 checksums with Rust-linked `dwebp`; `test_lossless.sh` (33 vectors, 330 comparisons); advanced-API matrix (cropping, scaling, `bypass_filtering`, `no_fancy_upsampling`, `flip`, `use_threads`, all `WEBP_CSP_MODE`, `WebPDecodeYUVInto`); incremental decoder at chunk sizes 1, 7, 64, 4,096; creates `fuzz/` as a separate crate whose only dependency is `libfuzzer-sys`, amends ADR 0005 to name it, and adds `dec_fuzzer`, `simple_api_fuzzer`, `advanced_api_fuzzer`, `huffman_fuzzer` as its targets; `webpinfo` output identical; row counters for the Ferrixel cropping measurement |
| **WP13 encoder integration** (about 150 plus ABI) | exceptions: `WebPEncode` with `GetPSNR`, `FinalizePSNR` and `StoreStats` in `src/enc/webp_enc.rs`, the simple encoding API (`Encode`, `WebPEncodeRGB`, `WebPEncodeBGR`, `WebPEncodeRGBA`, `WebPEncodeBGRA` and their `WebPEncodeLossless*` counterparts) in `src/enc/picture_enc.rs`; `src/abi/encode.rs` exporting `encode.h` | WP9, WP10, WP11, WP12 for the `fuzz/` crate (3b); takes over the WP9, WP10, WP11 modules; consumer build of `cwebp` | Full encoder matrix from the port plan with alpha (`alpha_quality`, `alpha_filtering`, `alpha_compression`, `exact`), `test_cwebp.sh` with Rust-linked `cwebp`, progress hook and abort, custom writer; `enc_fuzzer`, `enc_dec_fuzzer` as `fuzz/` targets, added only after WP12 is merged |
| **WP14 animation decode and demux ABI** (about 360 plus ABI) | `src/demux/anim_decode.c` (360); `src/abi/demux.rs` exporting `demux.h` | WP8, WP12 (3c); takes over the WP8 module | `WebPAnimDecoder` frames compared with the oracle on the animated vectors and synthetic animations (blend and dispose combinations, odd offsets); `animdecoder_fuzzer` as a `fuzz/` target after WP12 is merged; `anim_dump` as consumer |

### Wave 4

| Package | Scope | Depends on |
| --- | --- | --- |
| **WP15 mux and animation encode** (about 2,870) | `src/mux/muxedit.c`, `muxinternal.c`, `muxread.c`, `anim_encode.c`, headers; `src/abi/mux.rs` for `mux.h`; consumers `webpmux`, `img2webp`, `anim_diff`; `mux_demux_api_fuzzer`, `animencoder_fuzzer`, `animation_api_fuzzer` | WP6, WP7, WP12, WP13, WP14 |
| **WP16 safe decoder API** | Features probe, decoder configuration with cropping, scaling and colorspace into a caller `&mut [u8]` or planar slices, incremental decoder with `append`, demux view for metadata chunks; API ADR; tests against the C ABI path on the vectors | WP12, WP14 |
| **WP17 safe encoder API** | Config presets, picture import from strided RGB/RGBA rows and YUVA planes, output through `Vec<u8>` or `impl FnMut(&[u8]) -> bool`, progress and abort, allocation errors; API ADR | WP13 |
| **WP18 SIMD x86 decoder** (`dec_sse2`, `dec_sse41`, `upsampling_sse2/sse41`, `yuv_sse2/sse41`, `alpha_processing_sse2/sse41`, `filters_sse2`, `rescaler_sse2`, `lossless_sse2/sse41/avx2`) | Isolated `unsafe` `std::arch` modules behind runtime selection; exact parity with the scalar port and the SIMD-on oracle; reproducible benchmark per module | WP12 |
| **WP19 SIMD x86 encoder** (`enc_sse2`, `enc_sse41`, `cost_sse2`, `ssim_sse2`, `lossless_enc_sse2/sse41/avx2`, `sharpyuv_sse2`) | Same rules | WP13, WP18 (`common_sse2.h`, `common_sse41.h`) |
| **WP20 NEON** (all `*_neon.c` and `neon.h`) | Same rules, macOS ARM64 is the primary host | WP12, WP13 |

MIPS and MSA variants are not planned.

### Wave 5: Ferrixel adapters (in the Ferrixel repository)

| Package | Scope | Depends on |
| --- | --- | --- |
| **WP21 `webp-read`** | Source adapter under [ADR-0012](https://github.com/sebastian-software/ferrixel/blob/main/docs/adr/0012-codec-source-contract.md): header probe, RGB rows or planar YUV, cropping only with the measured semantics from WP12, ICC through the demux view, EXIF orientation through [ADR-0019](https://github.com/sebastian-software/ferrixel/blob/main/docs/adr/0019-source-side-orientation.md); amend [ADR-0018](https://github.com/sebastian-software/ferrixel/blob/main/docs/adr/0018-optional-codec-and-cms-dependencies.md) with the adoption evidence | WP16 and a pinnable libwebp-rs revision |
| **WP22 `webp-write`** | Sink for lossy and lossless output with one declared, budgeted output frame like the AVIF sink; output color signaling; CLI options | WP17 |

## Integration rules

- Merge order is sub-wave order. Inside a sub-wave, merge in the order pull requests become green (WP5 after WP4); rebase each branch on `main` before its checks.
- A package appends its function- and type-level exceptions to the other package's module; whoever owns or later takes over that module keeps them unchanged. `tools/check.py --oracle` fails when the declared dependencies, exceptions and reviewed includes in `tools/units.json` no longer cover the oracle's cross-unit references.
- A pull request is mergeable when `tools/check.py` passes on CI for both targets, the status file is complete, and a reviewer has spot-checked source correspondence for at least the largest unit.
- Wave 3 packages take over the modules of the packages they integrate (WP12: WP6, WP7; WP13: WP9, WP10, WP11; WP14: WP8). Until then, a session that finds a defect in another package's module reports it in its status file and in the pull request instead of fixing it.
- `docs/PORT_STATUS.md` is aggregated by the integrator from `docs/status/*.md` after each wave; packages never edit it.
- The nightly workflow runs the full gates of all merged integration packages (`test_dwebp.sh`, `test_cwebp.sh`, matrices, fuzz smoke) on both targets.

## Brief template for a session

```text
Port <package id and name> of libwebp-rs. Model: <claude-opus-5-5 | claude-haiku-5-5 per the model assignment>.

Repository: <path or clone URL>, branch <prefix>/<wp-id>. Read AGENTS.md, docs/PORTING.md,
docs/WORKPLAN.md (this package's row) and the ADRs in docs/adr before writing code.

Translate these upstream units from upstream/libwebp at the pinned revision into the
module files that already exist with their provenance headers: <list of C files -> Rust modules>,
plus the functions and types that tools/units.json assigns to this package as exceptions.
Depend only on these merged packages: <list>. Do not edit any other module.

Verification: write C probes under tools/oracle/probes/<wp-id>/ and differential tests under
tests/<wp-id>_*.rs that cover <the package's verification row>. Run tools/check.py.

Done: every listed unit translated with source references; tests green in debug, release and
checked profiles; docs/status/<wp-id>.md written (inventory, evidence, not-applicable checks,
recorded deviations); pull request opened against main with that summary. Do not push to main,
publish, advance submodules or add dependencies.
```

## Decisions taken before WP0

1. The four decisions of the port plan stand as recommended: the libjxl-rs method with `#![deny(unsafe_code)]` outside the C ABI module, a new repository, `rust-version = "1.95"` with edition 2024, and SharpYUV translated again in the safe style (WP5).
2. The maintainer integrates.
3. The GitHub repository visibility is still open. WP0 created nothing remote and asks for it when the local bootstrap is complete.

## Findings from WP0

Findings 2 to 5 changed WP0's own deliverables. The maintainer decided findings 1, 6 and 7 after WP0's first report; the catalog above reflects those decisions.

1. **The test data was pinned at `260804d` instead of `06ddd96`** (decided after WP0's first report). Test-data commits `05cd58b` and `06ddd96` (26 August 2025) add no vectors; they regenerate seven PGM checksums in `libwebp_tests.md5` and the `grid.pgm`, `peak.pgm` and `lossless_color_transform.pgm` references for libwebp `0d14d84b` ("Have lossless use ImportYUVAFromRGB", 24 September 2025, after v1.6.0). Against them, the v1.6.0 oracle's `dwebp` matches 473 of 480 checksums and no lossless vector's PGM output. At `260804d`, the last revision before that update, with the same 131 vectors, both scripts pass completely, so the pinned references match the pinned library. `tools/oracle_baseline.py --diagnose-post-v1.6.0` still reports the differences against `06ddd96`.
2. **`test_lossless.sh` covers 33 vectors, not 46:** the 32 `lossless_vec_*` files and `lossless_color_transform.webp`, each in five formats with and without `-noasm`, 330 comparisons. The test data holds 38 `lossless*.webp` files in all.
3. **The reference configuration does not build on x86-64.** Upstream's `WEBP_ENABLE_SIMD=OFF` appends `-mno-sse2`, and Clang then rejects functions that return a `double`. The oracle appends `-msse2` on x86-64 only ([ADR 0002](adr/0002-differential-oracle.md)); the Linux CI job is the first native evidence.
4. **`forbid(unsafe_code)` is not possible** with an ABI module that needs `unsafe`; the crate denies it instead ([ADR 0005](adr/0005-toolchain-and-unsafe-policy.md)).
5. **Assignments the catalog left open**, now in `tools/units.json`: `src/webp/mux.rs` (`mux.h`) to WP15; `src/abi/types.rs` (`WebPMalloc`, `WebPFree`, needed by the decoder tools) to WP12; `src/dsp/common_sse2.rs` and `common_sse41.rs` to WP18, so WP19 depends on WP18; `src/sharpyuv/sharpyuv_sse2.rs` to WP19; `src/dsp/neon.rs` and `src/sharpyuv/sharpyuv_neon.rs` to WP20. SIMD modules exist only on their architectures, and NEON only on AArch64, because Rust's 32-bit ARM NEON intrinsics are unstable. The 25 MIPS and MSA files are listed as excluded.
6. **The `fuzz/` crate belongs to WP12** (decided after WP0's first report). cargo-fuzz targets need `libfuzzer-sys`, and WP12, WP13 and WP14 would otherwise all edit `fuzz/Cargo.toml` in the same wave. WP12 creates `fuzz/` as a separate crate whose only dependency is `libfuzzer-sys` and amends ADR 0005 to name it; it is never a dependency of the library. WP13 and WP14 add their targets only after WP12 is merged. WP0 did not create the crate.
7. **Packages within a wave were not independent** (decided after WP0's first report). `tools/unit_dependencies.py` compares the declared dependencies with the oracle's actual cross-unit references (`nm` on the oracle objects plus `#include` lines). For the catalog as moved from Ferrixel it found these calls, among others: WP2, WP3 and WP4 call WP1's `WebPSafeMalloc`; WP1's `WebPGetColorPalette` calls WP3's `GetColorPalette`; WP3 calls WP4's alpha multiplication; WP6 calls WP12's alpha decoding; WP7 calls WP12's `WebPIoInitFromOptions`; WP8 calls WP12's `WebPGetFeaturesInternal`; WP9, WP10 and WP11 call each other through `webp_enc.c`; WP10 calls WP6's `dsp/dec.c` and WP13's alpha encoder. Some are file-level cycles (`vp8l_dec.c` and `webp_dec.c`, `webp_enc.c` and `picture_enc.c`), and because the ground rules forbid stubs and partial functions, no order of whole files resolves them.

   The maintainer adopted WP0's proposal: order the packages by the call graph into the sub-waves above, declare the dependencies the code needs, and assign the few cycle-closing functions to the package that needs them first, as function-level exceptions in `tools/units.json`. Checking the proposal against the types, which Rust needs before any code that uses them compiles, required four corrections, applied with it:

   - `alpha_dec.c` moves to WP6, not to WP7: it needs `VP8Decoder`, `VP8SetError` and `VP8InitIoInternal` from WP6 and `WebPInitCustomIo` from `io_dec.c`, so `io_dec.c` moves to WP6 as well (it needs only WP1, WP4 and WP7's `WebPIoInitFromOptions`). `alphai_dec.h` moves to WP7 as decided, because `vp8l_dec.c` fills `ALPHDecoder`.
   - `alpha_enc.c` moves from WP13 to WP10: `InitVP8Encoder` and `DeleteVP8Encoder`, which WP10's test drives, call `VP8EncInitAlpha` and `VP8EncDeleteAlpha`, and `VP8EncInitAlpha` installs the alpha worker hook that needs the whole file. Everything it calls (WP4's filters and quantizer, WP9's picture functions, WP11's `VP8LEncodeStream`) is merged before sub-wave 2c.
   - Three type-level exceptions: WP3 defines `VP8LTransform` in `src/dec/vp8li_dec.rs`, because the inverse transforms in `dsp/lossless.c` take it; WP7 defines `VP8Io` with its hook types in `src/dec/vp8_dec.rs` and `WebPDecParams` with its output callback types in `src/dec/webpi_dec.rs`.
   - WP7's exception in `src/dec/webp_dec.rs` also covers `WebPCheckCropDimensions` and `WebPCheckCropDimensionsBasic`, which `WebPIoInitFromOptions` calls; WP13's exception in `src/enc/webp_enc.rs` covers `GetPSNR`, `FinalizePSNR` and `StoreStats`, which only `WebPEncode` uses; WP5 merges after WP4, because `sharpyuv_cpu.c` includes `src/dsp/cpu.c`.

   With these, `tools/unit_dependencies.py --check` reports no undeclared reference; 16 header includes that use only macros, prototypes of the including package's own functions, or nothing, are recorded as reviewed in `tools/units.json`.
