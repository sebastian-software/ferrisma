# libwebp port plan (5 November 2026)

> Copied from the Ferrixel repository (`docs/research/2026-11-05-libwebp-port-plan.md`
> at commit `2aa4f3d1bb226d9541a4e9270168b03d17cac2ec`) by WP0 so that sessions in this
> repository have its gates and matrices at hand. Only links were changed: links to
> Ferrixel documents point to its `main` branch. The text is otherwise the original
> record. WP0 found four statements that do not hold; [WORKPLAN.md](WORKPLAN.md#findings-from-wp0)
> records them: test-data `06ddd96` postdates v1.6.0, so the test data is pinned at
> `260804d` instead (same 131 vectors), where `test_dwebp.sh` passes all 480 checksums;
> `test_lossless.sh` covers 33 vectors (32 `lossless_vec_*` and
> `lossless_color_transform.webp`), not 46; the crate uses `#![deny(unsafe_code)]`,
> because `forbid` cannot be relaxed for the C ABI module; and on x86-64 the reference
> configuration needs `-msse2` to build.

**Status:** Plan. Nothing has been ported and the repository does not exist yet. This document scopes a third sister port, libwebp-rs, next to [libavif-rs](https://github.com/sebastian-software/libavif-rs) and [libjxl-rs](https://github.com/sebastian-software/libjxl-rs). It records the upstream pin, the method to confirm, the phase order with the exact source units, the parity gates and the Ferrixel adoption step. Line counts are non-blank, non-comment lines of libwebp 1.6.0 and describe size, not effort. See [the codec evaluation](https://github.com/sebastian-software/ferrixel/blob/main/docs/research/2026-11-05-jpeg-and-webp-codec-evaluation.md) for why libwebp was chosen.

## Goal

The [work plan for parallel sessions](WORKPLAN.md) splits this plan into work packages with disjoint file ownership, waves and briefs.


A mechanical, behavior-preserving Rust port of libwebp 1.6.0: decoder, demux, encoder and later mux, with a safe Rust API that Ferrixel can consume behind `webp-read` and `webp-write` features under the adoption gates of [ADR-0018](https://github.com/sebastian-software/ferrixel/blob/main/docs/adr/0018-optional-codec-and-cms-dependencies.md). The original C tools (`dwebp`, `cwebp`, `webpinfo`) remain unmodified consumers that link the Rust library through the original headers, as both sister ports do.

## Upstream pin

| Item | Value |
| --- | --- |
| Source | <https://chromium.googlesource.com/webm/libwebp>, tag `v1.6.0`, commit `4fa21912338357f89e4fd51cf2368325b59e9bd9` (2025-06-30); the same revision libavif-rs pins under `upstream/libwebp` |
| Test vectors | <https://chromium.googlesource.com/webm/libwebp-test-data>, commit `06ddd96e276c2c638a72d39d3c0f340afd61978c` (2025-08-26): 131 `.webp` vectors (56 `vp80-*` lossy conformance files, 46 `lossless_vec_*`, alpha, color-cache and regression cases), `libwebp_tests.md5` with 480 output checksums for `bmp`, `pam`, `pgm`, `ppm` and `tiff` outputs, `test_dwebp.sh`, `test_cwebp.sh`, `test_lossless.sh` and `grid.pam` as the lossless reference image |
| Specifications | RFC 6386 (VP8) and RFC 9649 (WebP container, VP8L) as reading aids only; the pinned source is the authority |
| License | `COPYING` (BSD-3-Clause) and `PATENTS` (WebM patent grant) copied unchanged to the repository root; crate license `BSD-3-Clause`; translated modules keep Google's copyright notice and name their source file |

## Method to confirm before starting

The two sister ports use different methods:

| | libavif-rs | libjxl-rs |
| --- | --- | --- |
| Translation | c2rust 0.21.0 / LLVM 16.0.6 transpile with recorded corrections | hand-written mechanical translation |
| Result | C-ABI-preserving `unsafe` Rust throughout (the SharpYUV crate has 89 `unsafe` sites in 2,362 lines) | idiomatic Rust with `unsafe` where the translation needs it, each with a local safety argument (1340 `unsafe` blocks, functions or impls in 73 of 238 source files, 71,549 lines, on 2026-11-05) |
| Toolchain | nightly (`nightly-2026-09-07`) | stable, `rust-version = 1.95` |
| Reference | original C/C++ test suites linked as consumers | C++ oracle executable used only by differential tests; ordinary builds compile no C or C++ |
| C ABI | preserved by construction | exported from Rust for the original headers |

**Recommendation: the libjxl-rs method, with a stricter `unsafe` policy.** ADR-0018 gates (a) and (b) require a safe Rust API and a stable toolchain, which the c2rust output does not give without a second pass. libwebp is plain C without templates, and its SIMD lives in separate per-architecture files, so the portable core of about 30,000 lines is a tractable hand translation. Unlike libjxl-rs, the port should start with `#![deny(unsafe_code)]` outside the C ABI export module: `image-webp` and `zenwebp` show that VP8 and VP8L decode and encode translate into `unsafe`-free Rust, and the port plan's SIMD phase is the only foreseeable exception. Relaxing this for a unit needs an ADR with the local safety argument, as libjxl-rs requires. Representation rules follow libjxl-rs's accepted ADR on ownership and storage: byte offsets instead of pointers, `Result` instead of status integers with preserved codes, explicit partial state on error, deliberate integer widths and wrapping.

Decisions to confirm with the maintainer before Phase 0:

1. libjxl-rs method (recommended) or c2rust method.
2. New repository `sebastian-software/libwebp-rs` (recommended) or a workspace crate inside libavif-rs. libavif-rs keeps its `libavif-sharpyuv` crate either way.
3. MSRV: align with libjxl-rs (1.95) or with Ferrixel (1.98).
4. SharpYUV: translate again in the safe style (recommended, about 1,100 portable lines) or depend on `libavif-sharpyuv` (unsafe, nightly). The existing crate stays useful as a second oracle in both cases.

## Reference configuration for the C oracle

Build the pinned source once as the comparison oracle:

```sh
cmake -S upstream/libwebp -B build/oracle -DCMAKE_BUILD_TYPE=Release \
  -DWEBP_ENABLE_SIMD=OFF -DWEBP_USE_THREAD=ON -DWEBP_NEAR_LOSSLESS=ON \
  -DWEBP_ENABLE_SWAP_16BIT_CSP=OFF -DWEBP_BUILD_DWEBP=ON -DWEBP_BUILD_CWEBP=ON \
  -DWEBP_BUILD_WEBPINFO=ON -DWEBP_BUILD_LIBWEBPMUX=ON -DWEBP_BUILD_ANIM_UTILS=OFF \
  -DWEBP_BUILD_GIF2WEBP=OFF -DWEBP_BUILD_IMG2WEBP=OFF -DWEBP_BUILD_VWEBP=OFF \
  -DWEBP_BUILD_WEBPMUX=OFF -DWEBP_BUILD_EXTRAS=OFF -DCMAKE_C_FLAGS=-ffp-contract=off
```

- SIMD off and `-ffp-contract=off` follow libavif-rs ADR 0024 (non-contracting C references for portable ports). A second, SIMD-on oracle is diagnostic only; upstream's own `test_cwebp.sh` compares optimized against `-noasm` output by checksum, so any SIMD-versus-scalar difference is upstream's to explain and not an acceptance input.
- Debug, Release and AddressSanitizer/UndefinedBehaviorSanitizer oracle builds, as in libjxl-rs. The Rust side adds a `checked` profile with overflow checks and debug assertions.
- Floating point appears in `src/enc/quant_enc.c` (`pow`), `src/enc/picture_csp_enc.c` (gamma tables via `pow`), `src/enc/picture_psnr_enc.c` (`log`), `src/dsp/lossless_enc.c` (`log` folded into fixed-point tables), `src/enc/frame_enc.c`, `src/enc/filter_enc.c`, `src/enc/config_enc.c`, `src/enc/webp_enc.c`, `src/enc/picture_enc.c`, `src/dec/io_dec.c`, `src/dsp/ssim.c`, `src/utils/random_utils.c`, `src/utils/quant_levels_utils.c`, `sharpyuv/sharpyuv_gamma.c` (`pow`, `expf`, `logf`, `sqrtf`) and `sharpyuv/sharpyuv_csp.c` (`floor`). Rust's `f64::powf` and friends call the platform libm, so host-local equality with the oracle is the expectation; cross-host bit equality is not assumed and is recorded per platform, as libjxl-rs does.

## Scope and phases

### Phase 0: bootstrap, no codec code

- Repository modeled on libjxl-rs: `AGENTS.md`, `CONTRIBUTING.md`, `docs/PORTING.md`, `docs/PORT_STATUS.md`, `docs/PROVENANCE.md`, `docs/adr/` with the three starting decisions (mechanical port, differential oracle, Rust representation), `upstream/libwebp` as a Git submodule at the pin, `upstream.json`, `LICENSE` and `PATENTS` copies, `tools/check.py`, CI on Linux x86-64 and macOS ARM64 with ordinary, sanitizer-oracle and MSRV jobs.
- `upstream/libwebp-test-data` as a second submodule at its pin.
- Cargo: package `libwebp-rs`, library name `webp`, `crate-type = ["rlib", "staticlib", "cdylib"]`, `publish = false` until the API settles, `#![forbid(unsafe_code)]` crate-wide with the C ABI export module as the only audited exception.
- Build the oracle in the reference configuration and record the baseline: `test_dwebp.sh` passes all 480 checksums, `test_lossless.sh` passes all 46 vectors.

### Phase 1: foundations (about 6,500 lines)

- Public headers as Rust types with original field order and defaults: `types.h` (62), `format_constants.h` (53), `decode.h` (218), `encode.h` (234), `demux.h` (151), `mux_types.h` (55), including `WebPInitDecoderConfig`, `WebPConfigInit` with presets, `WebPValidateConfig` and the `WebPValidateDecoderConfig` added in 1.6.0.
- `src/utils`: `bit_reader_utils.c/.h` with `bit_reader_inl_utils.h` (501), `bit_writer_utils` (372), `huffman_utils` (283), `huffman_encode_utils` (351), `color_cache_utils` (82), `palette` (342), `rescaler_utils` (175), `quant_levels_utils` (104), `quant_levels_dec_utils` (224), `filters_utils` (67), `random_utils` (59), `endian_inl_utils.h` (73), `utils.c/.h` (324: `WebPSafeMalloc`/`WebPSafeCalloc` size checks and `WEBP_MAX_ALLOCABLE_MEMORY` become fallible allocations with the same limits), `thread_utils` (319: the `WebPWorker` interface and its `Reset`/`Sync`/`Launch`/`Execute`/`End` state machine on `std::thread`).
- Shared portable `src/dsp`: `dsp.h` (303), `cpu.c/.h` (394: dispatch tables select the portable implementations; `VP8GetCPUInfo` stays a query), `yuv.c/.h` (352), `upsampling.c` (280), `filters.c` (199), `rescaler.c` (210), `alpha_processing.c` (415), `lossless.c/.h` with `lossless_common.h` (876), `dec_clip_tables.c` (342), `quant.h` (64).
- Gates: per-unit differential tests against the oracle in separate processes with exact outputs: bit reader on random and truncated streams including the end-of-stream and `eos` behavior, Huffman table construction on fuzzed code-length sets including rejected sets, rescaler on seeded sizes and scale factors, filters, upsampling and YUV conversion on seeded planes with padded strides, allocation limits on oversized requests.

### Phase 2: decoder and demux (about 7,600 lines), first Ferrixel-relevant milestone

- `src/dec`: `webp_dec.c` (657: RIFF, VP8X and `WebPGetFeatures`), `vp8_dec.c` (591), `tree_dec.c` (500), `quant_dec.c` (87), `frame_dec.c` (624), `io_dec.c` (557: output modes, cropping, scaling, upsampling), `buffer_dec.c` (258), `alpha_dec.c` (183), `vp8l_dec.c` (1,436), `idec_dec.c` (735: incremental decoder); headers (445); `src/dsp/dec.c` (752).
- `src/demux/demux.c` (761) belongs to this phase: `ICCP`, `EXIF` and `XMP` chunks are stored and exposed only through the demux API (`WebPDemuxGetChunk`), not through the decoder, and Ferrixel needs the ICC profile before it decodes pixels. `anim_decode.c` (360) may wait for Phase 4.
- Export the `decode.h` and `demux.h` C ABI from Rust so the unmodified `examples/dwebp.c` (378) and `examples/webpinfo.c` (1,077) with `imageio` link against the Rust library.
- Gates:
  1. `test_dwebp.sh` against the Rust-linked `dwebp`: all 480 checksums in all five output formats, including `-pgm` (planar YUV dump) and `-pam` (RGBA).
  2. `test_lossless.sh`: all 46 lossless vectors equal to `grid.pam`.
  3. Advanced-API differential over all 131 vectors: `use_cropping` with odd and even origins and sizes, `use_scaling` to several targets, `bypass_filtering`, `no_fancy_upsampling`, `flip`, `use_threads`, every `WEBP_CSP_MODE` including the premultiplied modes, and `WebPDecodeYUVInto`; exact buffer equality with the oracle, including guard bytes around caller buffers.
  4. Incremental decoder: feed each vector in chunk sizes 1, 7, 64 and 4,096 bytes and compare each intermediate `WebPIDecGetRGB`/`WebPIDecGetYUVA` state and status with the oracle at the same prefixes.
  5. Malformed input: `dec_fuzzer`, `simple_api_fuzzer`, `advanced_api_fuzzer` and `huffman_fuzzer` from `tests/fuzzer` reproduced as Rust fuzz targets; status-code parity on a seeded corpus plus every vector truncated at every chunk boundary; the oracle side runs under the sanitizers.
  6. `webpinfo` linked against Rust prints identical output for every vector.
- Ferrixel measurement, recorded in [format-capabilities.md](https://github.com/sebastian-software/ferrixel/blob/main/docs/research/format-capabilities.md): instrument the port to count parsed macroblock rows, filtered rows and converted rows under cropping and scaling. The source shows what to expect (`src/dec/frame_dec.c`, `vp8_dec.c`, `io_dec.c`): VP8 parses macroblock rows up to the crop bottom plus the filter margin, filters only from the crop top, emits only cropped rows, and saves nothing for horizontal cropping; scaling runs after full-resolution reconstruction. Do not claim region savings before the measurement confirms this.

### Phase 3: encoder (about 14,800 lines)

- `sharpyuv` portable: `sharpyuv.c` (460), `sharpyuv_csp.c` (83), `sharpyuv_dsp.c` (76), `sharpyuv_gamma.c` (358), `sharpyuv_cpu.c` (2), headers (164). The 1,271-group inventory of libavif-rs's `SHARPYUV-VALIDATION.md` (RGB 8/10/12/16 by YUV 8/10/12 by sixteen transfer functions by five matrices, padded rows, planar and interleaved input, concurrent conversions) is the ready-made differential design; `libavif-sharpyuv` is a second oracle.
- `src/enc`, all 23 implementation files (11,744) and headers (626): `config_enc`, `picture_enc`, `picture_csp_enc` (RGB to YUV with and without sharp YUV), `picture_rescale_enc`, `picture_psnr_enc`, `picture_tools_enc`, `analysis_enc` (segments, uses `WebPWorker`), `iterator_enc`, `cost_enc`, `quant_enc` (trellis), `tree_enc`, `token_enc`, `syntax_enc`, `frame_enc` (passes, target size and PSNR loops), `filter_enc`, `alpha_enc` (uses `WebPWorker`), `near_lossless_enc`, `predictor_enc`, `histogram_enc`, `backward_references_enc`, `backward_references_cost_enc`, `vp8l_enc` (uses `WebPWorker`), `webp_enc`.
- Portable encoder-side `src/dsp`: `enc.c` (722), `cost.c` (372), `ssim.c` (124), `lossless_enc.c` (712). `lossless_avx2.c` and `lossless_enc_avx2.c` are SIMD and wait for Phase 5.
- Export the `encode.h` C ABI; the unmodified `examples/cwebp.c` (1,127) links against Rust.
- Gates: byte-identical output and identical `WebPAuxStats` against the SIMD-off oracle over a recorded matrix. Inputs: the 131 vectors decoded to `ppm`/`pam` by the oracle, `grid.png`, and seeded synthetic images with odd sizes, 1 by 1, the 16,383-pixel dimension limit and alpha patterns. Lossy: quality {0, 10, 50, 75, 90, 100} by method {0..6} by every `WebPPreset`, `segments` {1, 4}, `partitions` {0, 3}, `sns_strength` and `filter_strength` defaults and extremes, `filter_type`, `autofilter`, `alpha_quality` {0, 50, 100}, `alpha_filtering` and `alpha_compression`, `use_sharp_yuv`, `exact`, `emulate_jpeg_size`, `low_memory`, `thread_level` {0, 1}. Lossless: quality {0, 50, 100} by method {0..6}, `exact`, `near_lossless` {0, 40, 60, 80, 100}, `use_delta_palette`. Rate control: `target_size` and `target_PSNR` with `pass` {1, 3, 10}. Also `show_compressed`, the progress hook with user abort, `WebPMemoryWriter` and a custom `WebPWriterFunction`. `test_cwebp.sh` runs with the Rust-linked `cwebp`; `enc_fuzzer` and `enc_dec_fuzzer` become Rust fuzz targets.
- Upstream 1.6.0 fixed a regression where multi-threaded lossless output differed from single-threaded output; equality across `thread_level` is therefore an explicit test, not an assumption.

### Phase 4: container utilities (about 4,200 lines)

`src/mux`: `anim_encode.c` (1,277), `muxedit.c` (484), `muxinternal.c` (391), `muxread.c` (432), headers (132); `src/demux/anim_decode.c` (360); `mux.h` ABI. Consumers: `webpmux`, `img2webp`, `anim_diff`. Needed for animated WebP only. Ferrixel's still-image scope does not depend on it, so this phase can follow adoption.

### Phase 5: SIMD, deferred and measured

`src/dsp` SSE2, SSE4.1 and AVX2 variants (7,309 lines) and NEON variants (3,794) as isolated `unsafe` `std::arch` modules or portable SIMD, each admitted only with exact parity against the scalar port and the SIMD-on oracle, and with a reproducible benchmark. MIPS and MSA variants (10,402) have no target and are not planned. Ferrixel's rule applies: correctness and a scalar path first.

### Phase 6: safe Rust API and Ferrixel adoption

- Safe API over the port, separate from the C ABI module: a features probe; a decoder configuration with cropping, scaling and colorspace that decodes into a caller `&mut [u8]` or planar YUVA slices; an incremental decoder with `append(&[u8])` and partial-row access; a demux view for `ICCP`, `EXIF` and `XMP` chunks; an encoder with presets, picture import from RGB/RGBA rows with strides and from YUVA planes, output through `Vec<u8>` or `impl FnMut(&[u8]) -> bool` (the `WebPWriterFunction` contract), progress and abort hooks; allocation failures as errors.
- ADR-0018 gates: (a) the ABI module is the only `unsafe`; (b) stable toolchain; (c) a tagged release or pinned Git revision; (d) CI evidence on Ferrixel's targets; (e) exposed semantics: lossy output is full-range BT.601 YCbCr converted with the constants in `src/dsp/yuv.h`, alpha is straight unless a premultiplied `WEBP_CSP_MODE` is selected, ICC comes from the demux API, and WebP carries no orientation of its own beyond an optional `EXIF` chunk, which Ferrixel's orientation policy must treat as it treats JPEG EXIF ([ADR-0019](https://github.com/sebastian-software/ferrixel/blob/main/docs/adr/0019-source-side-orientation.md)).
- Ferrixel adapters, each with its own ADR when adopted: a `webp-read` source under the [ADR-0012](https://github.com/sebastian-software/ferrixel/blob/main/docs/adr/0012-codec-source-contract.md) contract (header probe, RGB rows or planar YUV, cropping only with the measured semantics from Phase 2, ICC through demux) and a `webp-write` sink for lossy and lossless output. libwebp's encoder needs the complete picture before analysis, so the sink declares and budgets one output frame like the AVIF sink does; this materialization is reported, not hidden.

## Verification principles carried over from the sister ports

- Compare source and target independently and in separate processes; a Rust round trip is not evidence.
- No stubs, no C fallback, no changed expected outputs, no skipped parity cases, no blanket warning suppressions.
- Record undefined or ambiguous source behavior, such as invalid-argument cases that are C undefined behavior, and exclude them from the equivalence oracle explicitly, as `SHARPYUV-VALIDATION.md` does.
- `PORT_STATUS.md` names exact translated units and open gaps. Passing a phase is not "libwebp is ported".
- Measure performance and memory separately after parity; timing and allocation addresses are not expected to match.

## Risks

- **libm equivalence.** `pow`, `log`, `expf`, `logf` and `sqrtf` results must match the oracle's libm bit for bit on the same host; record cross-host status instead of assuming it.
- **Threading.** `WebPWorker` semantics on `std::thread` must produce identical output with threads on and off; the decoder's `use_threads` and the encoder's `thread_level` are matrix dimensions.
- **Allocation limits.** `WebPSafeMalloc` size checks and `WEBP_MAX_ALLOCABLE_MEMORY` decide which malformed inputs fail; they must be preserved for status-code parity.
- **Build options.** `WEBP_ENABLE_SWAP_16BIT_CSP`, `WEBP_NEAR_LOSSLESS`, `WEBP_USE_THREAD` and `WEBP_REDUCE_SIZE`/`WEBP_REDUCE_CSP` change behavior; the pinned configuration is recorded and others are not claimed.
- **API design pressure.** Keep the C ABI export verbatim and design the safe API after Phase 2 against Ferrixel's actual needs, not before.
