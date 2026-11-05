# libwebp port work plan for parallel sessions (5 November 2026)

**Status:** WP0 (bootstrap) done on 5 November 2026; waves 1 to 5 not started. This plan decomposes the [libwebp port plan](PORT_PLAN.md) into work packages that independent Claude Code sessions can execute in parallel, each in its own worktree branch of this repository. It moved here from the Ferrixel repository (`docs/research/2026-11-05-libwebp-port-workplan.md` at commit `2aa4f3d1bb226d9541a4e9270168b03d17cac2ec`) during WP0, with links and paths made concrete; the package catalog is unchanged. [Findings from WP0](#findings-from-wp0) records what the bootstrap found and proposes adjustments that the maintainer decides. Line counts are non-blank, non-comment lines of libwebp 1.6.0; they size the packages and say nothing about duration.

## How the plan works

- **One package, one session, one branch, one pull request.** Every package lists the upstream units it translates and the Rust modules it owns. Two packages never own the same file, so sessions of the same wave run without coordination.
- **Waves order the dependencies.** A wave starts when the packages it depends on are merged. Inside a wave every package is independent. Fewer sessions than packages is fine; the critical path below says what to start first.
- **Shared ground rules replace coordination.** Each brief carries the same rules (below), the representation decisions from the bootstrap package and the differential-oracle harness. A session that needs something another package owns stops and reports instead of editing it.
- **An integrator merges by wave.** The integrator (maintainer or one session) rebases, runs `tools/check.py`, reviews source correspondence and merges in wave order, then aggregates `docs/PORT_STATUS.md` from the per-package status files.
- **Two model tiers.** Each package runs on Claude Opus 5.5 (`claude-opus-5-5`) when it designs, integrates, exports a C ABI or translates a codec core, and on Claude Haiku 5.5 (`claude-haiku-5-5`) when it is a mechanical, fully specified translation that the differential tests check exhaustively. The assignment is in the catalog below and in every brief.

## Ground rules for every session

1. Mechanical port of the pinned `upstream/libwebp` (v1.6.0, `4fa21912338357f89e4fd51cf2368325b59e9bd9`): preserve algorithms, tables, constants, numeric widths, operation order, status codes and partial state on error. No redesign, no tuning, no new optimizations, no SIMD.
2. One Rust module per C file with the C name (`src/dec/vp8_dec.c` becomes `src/dec/vp8_dec.rs`). Every translated function carries a source reference (`// upstream: src/dec/vp8_dec.c VP8GetHeaders`). Keep Google's copyright notice in each module.
3. Edit only the files the package owns plus `docs/status/<wp-id>.md`. Module files for every unit exist from the bootstrap with their provenance header only, so no package edits `mod.rs` or `lib.rs`.
4. No stubs, no placeholder APIs, no C fallback, no skipped parity cases, no changed expected outputs, no blanket warning suppressions. Unported functions stay absent.
5. `#![deny(unsafe_code)]` applies everywhere except the `src/abi` modules, which carry a local safety argument per operation. This is stricter than libjxl-rs, which uses `unsafe` with local safety arguments inside the codec; a unit that cannot meet it stops and asks for an ADR instead of adding `unsafe`. Ordinary `cargo build` compiles and links no C.
6. Verification is differential: a C probe compiled against the oracle build runs in a separate process and the Rust test compares exact bytes and status codes. A Rust round trip proves nothing. Debug, Release and the `checked` profile (overflow checks on) must pass.
7. No new Cargo dependencies. Development dependencies only if the bootstrap ADR lists them.
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
  tools/units.json              upstream unit -> module -> package, waves, declared dependencies
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
  fuzz/                         cargo-fuzz targets, registered by the integration packages
                                (not created by WP0; see the findings)
  docs/PORTING.md  PORT_STATUS.md  PROVENANCE.md  WORKPLAN.md  PORT_PLAN.md  adr/
  docs/status/<wp-id>.md        one status page per package
```

A package's `<wp-id>` is its number in lower case: WP6 is `wp6`, with the status
page `docs/status/wp6.md`, probes in `tools/oracle/probes/wp6/`, tests in
`tests/wp6_*.rs` and the branch `<prefix>/wp6`. `tools/units.json` lists every
module with its owning package. WP21 and WP22 live in the Ferrixel repository.

## Waves

| Wave | Packages | Parallel sessions | Unblocks |
| --- | --- | --- | --- |
| 0 | WP0 bootstrap | 1 | everything |
| 1 | WP1 types and workers, WP2 bit I/O, WP3 lossless shared, WP4 pixel ops, WP5 SharpYUV | up to 5 | wave 2 |
| 2 | WP6 VP8 decoder, WP7 VP8L decoder, WP8 demux, WP9 encoder picture, WP10 VP8 encoder, WP11 VP8L encoder | up to 6 | wave 3 |
| 3 | WP12 decoder integration, WP13 encoder integration, WP14 animation decode | up to 3 | wave 4 |
| 4 | WP15 mux, WP16 safe decoder API, WP17 safe encoder API, WP18 to WP20 SIMD | as wanted | Ferrixel adapters |
| 5 | WP21 Ferrixel `webp-read`, WP22 Ferrixel `webp-write` (in the Ferrixel repository) | 2 | Ferrixel WebP support |

**Critical path to Ferrixel's first WebP milestone (reading):** WP0 → WP1, WP2, WP4 (and WP3 for lossless) → WP6, WP7 → WP12 → WP16 → WP21.

**Start order with three sessions:** WP1, WP2, WP4; then WP3, WP5, WP6; then WP7, WP9, WP10; then WP8, WP11, WP12; then WP13, WP14, WP16; then WP15, WP17, WP21.

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

| Package | Upstream units (lines) | Provides to later packages | Verification |
| --- | --- | --- | --- |
| **WP1 types, memory, workers** (about 1,550) | `src/webp/types.h`, `format_constants.h`, `decode.h`, `encode.h`, `demux.h`, `mux_types.h` as Rust types and enums with original field order (773); `src/utils/utils.c/.h` (324), `endian_inl_utils.h` (73), `random_utils` (59), `thread_utils` (319) | `WebPDecoderConfig`, `WebPDecBuffer`, `WebPBitstreamFeatures`, `WebPConfig`, `WebPPicture`, `WebPAuxStats`, `WebPMemoryWriter` and the demux/mux enums; the allocation helper with `WEBP_MAX_ALLOCABLE_MEMORY`; `VP8Random`; the worker | Probes for `WebPSafeMalloc` limit decisions on boundary sizes, `VP8Random` sequences for seeded states, worker state transitions including `Sync` after failure; struct size and field offsets compared with the C `sizeof`/`offsetof` for the ABI mirror |
| **WP2 bit I/O** (about 870) | `bit_reader_utils.c/.h`, `bit_reader_inl_utils.h` (501), `bit_writer_utils.c/.h` (372) | `VP8BitReader`, `VP8LBitReader`, `VP8BitWriter`, `VP8LBitWriter` | Probes reading seeded and truncated streams with recorded bit positions, `eos` behavior and 64-bit refill boundaries; writers compared on byte output and overflow handling |
| **WP3 lossless shared** (about 1,930) | `huffman_utils` (283), `huffman_encode_utils` (351), `color_cache_utils` (82), `palette` (342), `dsp/lossless.c/.h`, `lossless_common.h` (876) | Huffman table construction and lookup, Huffman tree building for the encoder, `VP8LColorCache`, palette sorting and search, predictors, color transforms and the shared `VP8LDspInit` functions | Probes on fuzzed code-length sets including rejected ones, cache operations, palette orders, and every predictor and transform on seeded ARGB rows |
| **WP4 pixel ops** (about 2,850) | `dsp/dsp.h`, `cpu.c/.h` (697), `yuv.c/.h` (352), `upsampling.c` (280), `alpha_processing.c` (415), `filters.c` plus `utils/filters_utils` (266), `rescaler.c` plus `utils/rescaler_utils` (385), `utils/quant_levels_utils` and `quant_levels_dec_utils` (328), `ssim.c` (124) | YUV to RGB conversions for every `WEBP_CSP_MODE`, upsamplers, premultiply and alpha extraction, alpha filters and filter estimation, `WebPRescaler`, `QuantizeLevels` and `WebPDequantizeLevels`, `VP8SSIM*`, `VP8GetCPUInfo` | Probes on seeded planes with padded strides, odd sizes and all colorspace modes; rescaler on seeded scale factors including 1:1 and extreme downscales; filter estimation on seeded images |
| **WP5 SharpYUV** (about 1,140) | `sharpyuv/sharpyuv.c` (460), `sharpyuv_csp.c` (83), `sharpyuv_dsp.c` (76), `sharpyuv_gamma.c` (358), `sharpyuv_cpu.c` (2), headers (164) | `SharpYuvConvert`, `SharpYuvConvertWithOptions`, matrices and transfer functions | The 1,271-group inventory of libavif-rs `SHARPYUV-VALIDATION.md` reproduced as probes; `libavif-sharpyuv` optionally as a second oracle |

### Wave 2

| Package | Upstream units (lines) | Depends on | Verification |
| --- | --- | --- | --- |
| **WP6 VP8 decoder core** (about 3,170) | `src/dec/vp8_dec.c` (591), `tree_dec.c` (500), `quant_dec.c` (87), `frame_dec.c` (624), `vp8_dec.h`, `vp8i_dec.h`, `common_dec.h` (273), `dsp/dec.c` (752), `dec_clip_tables.c` (342) | WP1, WP2, WP4 | A probe that drives `VP8GetHeaders`/`VP8Decode` through `VP8Io` with a put callback dumping reconstructed Y, U, V rows, with and without in-loop filtering bypass, with dithering strengths, over the 56 `vp80-*` vectors, truncations at every partition boundary and the decoder fuzz dictionary; exact plane equality and status codes |
| **WP7 VP8L decoder** (about 1,520) | `vp8l_dec.c` (1,436), `vp8li_dec.h` (82) | WP1, WP2, WP3, WP4 | Probe dumping decoded ARGB rows for the 32 `lossless_vec_*` vectors, `lossless_color_transform.webp` and the alpha, color-cache and `bad_palette_index` vectors; truncations; `VP8LGetInfo` on headers |
| **WP8 demux** (about 760) | `src/demux/demux.c` (761) | WP1 | Probe iterating frames and chunks (`ICCP`, `EXIF`, `XMP`, unknown) for every vector and for synthetic VP8X files with each flag combination, partial data states (`WEBP_DEMUX_PARSING_HEADER` onward) and malformed chunk sizes; exact iterator results |
| **WP9 encoder picture and config** (about 1,730) | `src/enc/config_enc.c` (123), `picture_enc.c` (229), `picture_csp_enc.c` (696), `picture_rescale_enc.c` (238), `picture_tools_enc.c` (236), `picture_psnr_enc.c` (204) | WP1, WP4, WP5 | Probes for `WebPConfigInit` presets and `WebPValidateConfig` on boundary values, `WebPPictureImport*` from RGB/RGBA/BGRA with strides, `WebPPictureARGBToYUVA` with and without sharp YUV and dithering, `YUVAToARGB`, rescale and crop, `WebPPictureDistortion` metrics, `WebPMemoryWriter`; exact planes and metrics |
| **WP10 VP8 encoder core** (about 5,720) | `analysis_enc.c` (364), `iterator_enc.c` (326), `cost_enc.c/.h` (327), `quant_enc.c` (1,051), `tree_enc.c` (474), `token_enc.c` (211), `syntax_enc.c` (297), `frame_enc.c` (741), `filter_enc.c` (178), `webp_enc.c` without the alpha branch (297), `vp8i_enc.h` (298), `dsp/enc.c` (722), `dsp/cost.c` (372), `dsp/quant.h` (64) | WP1, WP2, WP4 | Probe calling `WebPEncode` on YUV pictures without alpha produced by the oracle, over quality, method, presets, segments, partitions, SNS and filter settings, `pass`, `target_size`, `target_PSNR`, `emulate_jpeg_size`, `low_memory`, `thread_level`; byte-identical files and `WebPAuxStats` |
| **WP11 VP8L encoder** (about 5,830) | `vp8l_enc.c` (1,566), `predictor_enc.c` (854), `histogram_enc.c/.h` (951), `backward_references_enc.c/.h` (980), `backward_references_cost_enc.c` (573), `near_lossless_enc.c` (115), `vp8li_enc.h` (74), `dsp/lossless_enc.c` (712) | WP1, WP2, WP3, WP4 | Probe calling `VP8LEncodeImage` (writes the complete RIFF file itself) on ARGB pictures over quality, method, `exact`, `near_lossless`, `use_delta_palette` and `thread_level`; byte-identical files |

### Wave 3

| Package | Upstream units (lines) | Takes over | Verification and gates |
| --- | --- | --- | --- |
| **WP12 decoder integration** (about 2,480 plus ABI) | `webp_dec.c` (657), `buffer_dec.c` (258), `io_dec.c` (557), `alpha_dec.c` (183), `idec_dec.c` (735), `alphai_dec.h`, `webpi_dec.h` (90); `src/abi/decode.rs` exporting `decode.h` | WP6, WP7 modules for integration fixes; `tools/oracle` consumer build of `dwebp`, `webpinfo` and `imageio` against the Rust static library | `test_dwebp.sh` 480 checksums with Rust-linked `dwebp`; `test_lossless.sh` (33 vectors, 330 comparisons); advanced-API matrix (cropping, scaling, `bypass_filtering`, `no_fancy_upsampling`, `flip`, `use_threads`, all `WEBP_CSP_MODE`, `WebPDecodeYUVInto`); incremental decoder at chunk sizes 1, 7, 64, 4,096; `dec_fuzzer`, `simple_api_fuzzer`, `advanced_api_fuzzer`, `huffman_fuzzer` as `fuzz/` targets; `webpinfo` output identical; row counters for the Ferrixel cropping measurement |
| **WP13 encoder integration** (about 340 plus ABI) | `alpha_enc.c` (341), the alpha branch of `webp_enc.rs`; `src/abi/encode.rs` exporting `encode.h` | WP9, WP10, WP11 modules; consumer build of `cwebp` | Full encoder matrix from the port plan with alpha (`alpha_quality`, `alpha_filtering`, `alpha_compression`, `exact`), `test_cwebp.sh` with Rust-linked `cwebp`, progress hook and abort, custom writer; `enc_fuzzer`, `enc_dec_fuzzer` as `fuzz/` targets |
| **WP14 animation decode and demux ABI** (about 360 plus ABI) | `src/demux/anim_decode.c` (360); `src/abi/demux.rs` exporting `demux.h` | WP8 | `WebPAnimDecoder` frames compared with the oracle on the animated vectors and synthetic animations (blend and dispose combinations, odd offsets); `animdecoder_fuzzer`; `anim_dump` as consumer |

### Wave 4

| Package | Scope | Depends on |
| --- | --- | --- |
| **WP15 mux and animation encode** (about 2,870) | `src/mux/muxedit.c`, `muxinternal.c`, `muxread.c`, `anim_encode.c`, headers; `src/abi/mux.rs` for `mux.h`; consumers `webpmux`, `img2webp`, `anim_diff`; `mux_demux_api_fuzzer`, `animencoder_fuzzer`, `animation_api_fuzzer` | WP13, WP14 |
| **WP16 safe decoder API** | Features probe, decoder configuration with cropping, scaling and colorspace into a caller `&mut [u8]` or planar slices, incremental decoder with `append`, demux view for metadata chunks; API ADR; tests against the C ABI path on the vectors | WP12, WP14 |
| **WP17 safe encoder API** | Config presets, picture import from strided RGB/RGBA rows and YUVA planes, output through `Vec<u8>` or `impl FnMut(&[u8]) -> bool`, progress and abort, allocation errors; API ADR | WP13 |
| **WP18 SIMD x86 decoder** (`dec_sse2`, `dec_sse41`, `upsampling_sse2/sse41`, `yuv_sse2/sse41`, `alpha_processing_sse2/sse41`, `filters_sse2`, `rescaler_sse2`, `lossless_sse2/sse41/avx2`) | Isolated `unsafe` `std::arch` modules behind runtime selection; exact parity with the scalar port and the SIMD-on oracle; reproducible benchmark per module | WP12 |
| **WP19 SIMD x86 encoder** (`enc_sse2`, `enc_sse41`, `cost_sse2`, `ssim_sse2`, `lossless_enc_sse2/sse41/avx2`) | Same rules | WP13 |
| **WP20 NEON** (all `*_neon.c`) | Same rules, macOS ARM64 is the primary host | WP12, WP13 |

MIPS and MSA variants are not planned.

### Wave 5: Ferrixel adapters (in the Ferrixel repository)

| Package | Scope | Depends on |
| --- | --- | --- |
| **WP21 `webp-read`** | Source adapter under [ADR-0012](https://github.com/sebastian-software/ferrixel/blob/main/docs/adr/0012-codec-source-contract.md): header probe, RGB rows or planar YUV, cropping only with the measured semantics from WP12, ICC through the demux view, EXIF orientation through [ADR-0019](https://github.com/sebastian-software/ferrixel/blob/main/docs/adr/0019-source-side-orientation.md); amend [ADR-0018](https://github.com/sebastian-software/ferrixel/blob/main/docs/adr/0018-optional-codec-and-cms-dependencies.md) with the adoption evidence | WP16 and a pinnable libwebp-rs revision |
| **WP22 `webp-write`** | Sink for lossy and lossless output with one declared, budgeted output frame like the AVIF sink; output color signaling; CLI options | WP17 |

## Integration rules

- Merge order is wave order. Inside a wave, merge in the order pull requests become green; rebase each branch on `main` before its checks.
- A pull request is mergeable when `tools/check.py` passes on CI for both targets, the status file is complete, and a reviewer has spot-checked source correspondence for at least the largest unit.
- Wave 3 packages take over the modules of the wave 2 packages they integrate. Until then, a wave 2 session that finds a defect in another package's module reports it in its status file and in the pull request instead of fixing it.
- `docs/PORT_STATUS.md` is aggregated by the integrator from `docs/status/*.md` after each wave; packages never edit it.
- The nightly workflow runs the full gates of all merged integration packages (`test_dwebp.sh`, `test_cwebp.sh`, matrices, fuzz smoke) on both targets.

## Brief template for a session

```text
Port <package id and name> of libwebp-rs. Model: <claude-opus-5-5 | claude-haiku-5-5 per the model assignment>.

Repository: <path or clone URL>, branch <prefix>/<wp-id>. Read AGENTS.md, docs/PORTING.md,
docs/WORKPLAN.md (this package's row) and the ADRs in docs/adr before writing code.

Translate these upstream units from upstream/libwebp at the pinned revision into the
module files that already exist with their provenance headers: <list of C files -> Rust modules>.
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

The bootstrap left the package catalog above unchanged. These findings either changed WP0's own deliverables (1 to 5) or need the maintainer's decision before the fan-out (6 and 7).

1. **The test data was pinned at `260804d` instead of `06ddd96`** (decided after WP0's first report). Test-data commits `05cd58b` and `06ddd96` (26 August 2025) add no vectors; they regenerate seven PGM checksums in `libwebp_tests.md5` and the `grid.pgm`, `peak.pgm` and `lossless_color_transform.pgm` references for libwebp `0d14d84b` ("Have lossless use ImportYUVAFromRGB", 24 September 2025, after v1.6.0). Against them, the v1.6.0 oracle's `dwebp` matches 473 of 480 checksums and no lossless vector's PGM output. At `260804d`, the last revision before that update, with the same 131 vectors, both scripts pass completely, so the pinned references match the pinned library. `tools/oracle_baseline.py --diagnose-post-v1.6.0` still reports the differences against `06ddd96`.
2. **`test_lossless.sh` covers 33 vectors, not 46:** the 32 `lossless_vec_*` files and `lossless_color_transform.webp`, each in five formats with and without `-noasm`, 330 comparisons. The test data holds 38 `lossless*.webp` files in all.
3. **The reference configuration does not build on x86-64.** Upstream's `WEBP_ENABLE_SIMD=OFF` appends `-mno-sse2`, and Clang then rejects functions that return a `double`. The oracle appends `-msse2` on x86-64 only ([ADR 0002](adr/0002-differential-oracle.md)); the Linux CI job is the first native evidence.
4. **`forbid(unsafe_code)` is not possible** with an ABI module that needs `unsafe`; the crate denies it instead ([ADR 0005](adr/0005-toolchain-and-unsafe-policy.md)).
5. **Assignments the catalog left open**, now in `tools/units.json`: `src/webp/mux.rs` (`mux.h`) to WP15; `src/abi/types.rs` (`WebPMalloc`, `WebPFree`, needed by the decoder tools) to WP12; `src/dsp/common_sse2.rs` and `common_sse41.rs` to WP18, so WP19 depends on WP18; `src/sharpyuv/sharpyuv_sse2.rs` to WP19; `src/dsp/neon.rs` and `src/sharpyuv/sharpyuv_neon.rs` to WP20. SIMD modules exist only on their architectures, and NEON only on AArch64, because Rust's 32-bit ARM NEON intrinsics are unstable. The 25 MIPS and MSA files are listed as excluded.
6. **The `fuzz/` crate needs a decision before wave 3.** cargo-fuzz targets need `libfuzzer-sys`, a development dependency that ADR 0005 must name first, and WP12, WP13 and WP14 would all edit `fuzz/Cargo.toml` in the same wave. WP0 did not create the crate.
7. **Packages within a wave are not independent.** `tools/unit_dependencies.py` compares the declared dependencies with the oracle's actual cross-unit references (`nm` on the oracle objects plus `#include` lines). Calls that the catalog's dependencies do not cover:

   | Package | Calls into | Functions |
   | --- | --- | --- |
   | WP2, WP3, WP4 | WP1, same wave | `WebPSafeMalloc`, `WebPSafeCalloc`, `WebPSafeFree` |
   | WP1 | WP3, same wave | `WebPGetColorPalette` in `utils.c` calls `GetColorPalette` in `palette.c` |
   | WP3 | WP4, same wave | `WebPApplyAlphaMultiply`, `WebPApplyAlphaMultiply4444`, `VP8GetCPUInfo` from `dsp/lossless.c` |
   | WP6 | WP12, later wave | `VP8DecompressAlphaRows` (`frame_dec.c`), `WebPDeallocateAlphaMemory` (`vp8_dec.c`) |
   | WP7 | WP12, later wave | `WebPIoInitFromOptions` (`vp8l_dec.c`) |
   | WP8 | WP12, later wave | `WebPGetFeaturesInternal` (`demux.c`) |
   | WP9 | WP10, same wave | `WebPEncodingSetError`, `WebPEncode` |
   | WP9 | WP3, undeclared | `VP8LConvertBGRAToRGBA`, `VP8LDspInit` |
   | WP10 | WP6, same wave | `VP8DspInit`, the loop filters and `VP8TransformWHT` from `dsp/dec.c` |
   | WP10 | WP9 and WP11, same wave | `WebPValidateConfig`, `WebPValidatePicture`, picture conversion and cleanup, `VP8LEncodeImage` (all from `webp_enc.c`) |
   | WP10 | WP13, later wave | `VP8EncInitAlpha`, `VP8EncStartAlpha`, `VP8EncFinishAlpha`, `VP8EncDeleteAlpha` |
   | WP11 | WP9 and WP10, same wave | `WebPPictureView`, `WebPPictureHasTransparency`, `WebPPictureInitInternal`, `WebPEncodingSetError`, `WebPReportProgress` |
   | WP14 | WP12, same wave | `WebPDecode`, `WebPGetFeaturesInternal`, `WebPInitDecoderConfigInternal` |
   | WP15 | WP6, WP7, WP12, undeclared | `VP8GetInfo`, `VP8LGetInfo`, `VP8LCheckSignature`, `WebPDecode` and the config initializers |

   Some of these are cycles at file level (`vp8l_dec.c` and `webp_dec.c` call each other; `webp_enc.c` and `picture_enc.c` call each other), so no order of whole files resolves them. Because the ground rules forbid stubs and partial functions, a package cannot translate a function whose callee is still absent; WP6, for example, could not translate `FinishRow`, and so neither `VP8Decode`, which its verification row needs.

   **Proposal, not applied:** keep file ownership, order the packages by the call graph, and assign the few cycle-closing functions to the package that needs them first, recorded as function-level exceptions in `tools/units.json`:

   - Wave 1: WP1 first; then WP2, WP4 and WP5 in parallel; then WP3. WP3 also translates `WebPGetColorPalette` into `src/utils/utils.rs`.
   - Decoder: move `alpha_dec.c` and `alphai_dec.h` from WP12 to WP7 (the alpha plane is VP8L-coded), and let WP7 also translate `WebPIoInitFromOptions` into `src/dec/webp_dec.rs`; run WP6 after WP7. WP8 needs `WebPGetFeaturesInternal`, so it runs after WP12, followed by WP14.
   - Encoder: WP9 also translates `WebPEncodingSetError` and `WebPReportProgress` into `src/enc/webp_enc.rs`; WP13 translates the rest of `webp_enc.c` (`WebPEncode` and its helpers) and the simple-encoding functions of `picture_enc.c` that call it; run WP11 after WP9 and WP10 after WP6.
   - Declare WP3 for WP9, and WP6, WP7 and WP12 for WP15.

   The critical path to Ferrixel's first reading milestone becomes WP0 → WP1 → WP2, WP4 → WP3 → WP7 → WP6 → WP12 → WP8 → WP14 → WP16 → WP21.
