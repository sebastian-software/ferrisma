# Decode-side scaling and encoder capabilities (5 November 2026)

**Status:** Research notes for [M2](../milestones/M2-performance-architecture.md) work packages 2 (encoder throughput) and 5 (decode-side scaling and native layouts); no code or dependency changed. Crate facts were read from the exact sources in the local Cargo registry for the versions pinned in `Cargo.lock`: `zune-jpeg` 0.5.15 (`zune-core` 0.5.3), `jxl-oxide` 0.12.6 (`jxl-render` 0.12.4, `jxl-frame` 0.13.3, `jxl-threadpool` 1.0.0), `avif-decode` 3.0.0, `rav1d` 1.1.0, `ravif` 0.13.0, `rav1e` 0.8.1, `avif-serialize` 0.8.9, `maybe-rayon` 0.1.1, `v_frame` 0.3.9. The sister ports were read at their `main` HEADs of 2026-11-05 (libavif-rs `d025dca`, libjxl-rs `b5994bd`). libvips, libjpeg-turbo and libjxl were read from GitHub `master`/`main` on 2026-11-05; their line numbers drift. File references are `crate/path:line`. **Nothing was timed**; every statement about cost is structural, not measured.

## Summary

- **The M1 codec stack runs without threads and without SIMD**, except `rav1d` on x86-64. Ferrisma's `default-features = false` turns off `zune-jpeg`'s intrinsics, `jxl-oxide`'s Rayon pool, and `ravif`'s `threading` and `asm` features; the latter makes `rav1e` link the single-threaded `maybe-rayon` shim, so `with_num_threads(Some(1))` in `src/avif_sink.rs` is not what limits the encoder. Package 0's baseline must say which configuration it measured.
- **JPEG:** `zune-jpeg` has no scaled, region or row-streaming decode. Its 4x4 and 1x1 IDCTs are sparse-coefficient shortcuts that still emit 8x8 samples. Baseline decoding is internally MCU-row streaming into a caller-provided full-frame buffer; progressive decoding buffers every coefficient. DCT-domain scaling is an upstream contribution with six touchpoints, listed below, and even the reference implementation saves no entropy decoding.
- **JPEG XL:** the region request does skip entropy decoding and transforms of groups outside the group-aligned, filter-padded region for VarDCT and plain modular frames; modular frames with squeeze or palette, LF frames and reference frames fall back to the full frame, and the whole codestream stays in memory. No public reduced-resolution (1/2, 1/4, 1/8, LF-only) rendering exists; the LF image is computed internally but not exposed.
- **AVIF decode:** `avif-decode` runs `rav1d` with up to 32 threads and converts to a full RGB(A) frame; the YUV planes, chroma sampling and bit depth exist behind `pub(crate)` and are not reachable. AV1 has no reduced-resolution decode; neither dav1d nor libavif offers one.
- **AVIF encode:** `ravif` accepts 10-bit input but hard-codes 4:4:4 and sRGB/BT.709 signalling and feeds `rav1e` a complete padded frame. `rav1e` itself takes 4:2:0 and 4:2:2 planes at 8, 10 or 12 bits, a tile count and a thread pool, and `avif-serialize` can write the matching `av1C`/`colr`; a direct `rav1e` adapter is the route to a native YUV path. Neither offers row or tile input.
- **Sister ports:** libavif-rs exposes libavif's complete C API (planes, bit depth, `maxThreads`, tiles, YUV input to the encoder) as `unsafe extern "C"` on nightly Rust, guarded to macOS ARM64, with libvmaf linked natively; libjxl-rs builds on stable Rust 1.95 with no dependencies but exports libjxl's C-style API with raw pointers, serializes its parallel-runner tasks, and has no lossy encoder. Neither meets ADR-0018's adoption gates yet.
- **Reference:** libvips' shrink-on-load sets `scale_denom` on libjpeg and reads scanlines in 8-row strips; libjpeg-turbo scales by choosing a smaller IDCT per component after decoding every coefficient. libjxl's "downsampled" decode is a progressive flush into a full-size buffer plus skipping the rest of the frame, and only when the encoder wrote passes.

## 0. What Ferrisma actually builds

Resolved with `cargo tree --offline -e features` for each Ferrisma feature on 2026-11-05.

| Ferrisma feature | Crate features enabled | Consequence | Evidence |
| --- | --- | --- | --- |
| `jpeg-read` | `zune-jpeg/std` only (`x86`, `neon` off) | Scalar IDCT, upsampling and color conversion; the AVX2/NEON paths are compiled out | `Cargo.toml:28`; `zune-jpeg/Cargo.toml` features; `zune-jpeg/src/idct.rs:51-76` |
| `jpeg-xl-read` | `jxl-oxide` without `rayon` | `jxl-threadpool` is built with `default = []`, so `default_pool()` returns `JxlThreadPool::none()`: every group job runs inline | `Cargo.toml:29`; `jxl-oxide/Cargo.toml:50`; `jxl-oxide/src/lib.rs:192-195`; `jxl-threadpool/Cargo.toml:30-35`; `Cargo.lock` lists only `tracing` under `jxl-threadpool` |
| `avif-write` | `ravif` without `threading` and `asm` | `ravif` substitutes a `rayoff` module whose `join` runs both closures sequentially; `rav1e` is built without `threading = ["rayon/threads"]`, so its `rayon` alias is `maybe-rayon` 0.1.1 with the `threads` feature off, whose `ThreadPool::install` calls the closure in place; no assembly | `Cargo.toml:24`; `ravif/Cargo.toml:56`; `ravif/src/lib.rs:32-41`; `ravif/src/av1encoder.rs:5-6`; `rav1e/Cargo.toml:87, 230-233`; `maybe-rayon/src/lib.rs:4-30`; `Cargo.lock` lists only `cfg-if` under `maybe-rayon` |
| `avif-read` | `avif-decode` without `binary`; `rav1d` with `default` (`asm`) on x86/x86-64, with only `bitdepth_8`/`bitdepth_16` elsewhere | On the Apple Silicon dev machine `rav1d` runs without its arm64 assembly; on x86-64 it needs NASM and runs the assembly | `Cargo.toml:25`; `avif-decode/Cargo.toml:82-95`; `cargo tree --target aarch64-apple-darwin` vs `--target x86_64-unknown-linux-gnu` |

`rayon` 1.12.0 is still in the lockfile, pulled by `av-scenechange` through `rav1e`, not by any codec path Ferrisma calls.

## 1. `zune-jpeg` 0.5.15

### Public interface

`JpegDecoder::{new, new_with_options, decode_headers, decode, decode_into, output_buffer_size, info, dimensions, icc_profile, exif, xmp, iptc, input_colorspace, output_colorspace, options, set_options}` (`src/decoder.rs:215-880`). `decode_into` requires a buffer of at least `width * height * components` bytes and returns `TooSmallOutput` otherwise (`src/decoder.rs:827-848`, `:265-276`). The options type comes from `zune-core`: `set_max_width`, `set_max_height`, `set_use_unsafe`, `set_strict_mode`, `set_byte_endian`, `jpeg_set_max_scans`, `jpeg_set_out_colorspace` (`zune-core/src/options/decoder.rs:231-453`). There is no scale factor, no region, no row callback and no incremental output. The 0.5.16 release candidate in the registry adds arithmetic decoding and cancellation (`bitstream_arith.rs`, `cancel.rs`) and no scaling (grep only; not reviewed).

### Internal structure

- **Baseline:** per component a scratch buffer of one MCU row, `width_stride * vertical_sample * 8` samples (`src/mcu.rs:156-166`); for each MCU row the decoder entropy-decodes and inverse-transforms all MCUs (`decode_mcu_width`, `:215-248`), then `post_process` upsamples and color-converts that row band straight into the caller's buffer at `pixels[pixels_written..]` (`:252-260`, `:788-930`). Output is therefore produced top-down in bands of 8 or 16 rows while the entropy decoder is at most one band ahead. Exception: when the components are not all in the first scan, every coefficient of the image is buffered first (`:171-186`, comment on issue #277).
- **Progressive:** all coefficients of all components are buffered (`src/mcu_prog.rs:59`), refined scan by scan, and transformed at the end in `finish_progressive_decoding` (`:473`). This mirrors libjpeg's `coef->whole_image` for multi-scan files.
- **Reduced IDCTs are not scaling.** After entropy decoding a block, the decoder counts the decoded coefficients and picks a 1x1 kernel for DC-only blocks, a 4x4-input kernel when at most ten zigzag coefficients are non-zero, else the full kernel (`src/mcu.rs:584-591`). The 4x4 kernel's contract is "only the first 4x4 block of coefficients are non-zero" (`src/idct.rs:81-84`); it still writes eight rows of eight samples (`src/idct/scalar.rs:203-260`). They reduce arithmetic on sparse blocks, not output size.
- **Threads:** none. The README lists "Single-threaded decoding" as a feature (`README.md:58`); no thread or Rayon dependency.
- **`unsafe`:** only in the SIMD files (`src/idct/{avx2,neon}.rs`, `src/color_convert/{avx,neon64}.rs`, `src/upsampler/{avx2,neon}.rs`, `src/unsafe_utils*.rs`), compile-time gated by the `x86`/`neon` features and runtime-gated by `DecoderOptions::use_unsafe` (`README.md:36`; `src/idct.rs:51-76`). Ferrisma compiles them out.

### What adding DCT-domain scaled decode would involve

libjpeg's model (section 6) is the template: a per-component output block size of 1, 2 or 4 instead of 8, chosen so that subsampled chroma gets a larger IDCT instead of an upsampling pass. In `zune-jpeg` that touches:

1. **Options.** A scale denominator in `zune-core`'s `DecoderOptions` (a separate crate with its own release cycle) or a `zune-jpeg`-local setter; `output_buffer_size`, `dimensions` and `ImageInfo` must report the scaled size while EXIF and ICC stay as they are (`src/decoder.rs:245-276`, `:954-1060`).
2. **Kernels.** New reduced-output kernels, 8x8 to 4x4, 2x2 and 1x1, equivalent to libjpeg-turbo's `jidctred.c`, keeping the `IDCTPtr` signature `(&mut [i32; 64], &mut [i16], stride)`; scalar first, then AVX2 and NEON variants, since the dispatch is per ISA (`src/idct.rs:51-103`). The existing `idct4x4` cannot be reused: it has a different contract.
3. **Geometry.** Block placement (`idct_position`, `src/mcu.rs:572-580`), per-component `width_stride` and the upsampler selection (`set_upsampling`, `src/decoder.rs:883-925`; `setup_upsample_scanline`, `src/components.rs:190`) assume eight output samples per block. At 1/2 with 4:2:0 chroma the chroma IDCT would run at 8x8 while luma runs at 4x4, so no upsampling is needed; that interplay is the part libjpeg's `jdmaster.c` handles.
4. **Post-processing.** `post_process` and `color_convert` take `width` and `padded_width` (`src/mcu.rs:788-800`; `src/worker.rs:31, 375`); the band height per MCU row shrinks accordingly.
5. **Progressive.** `finish_progressive_decoding` shares the same kernels and geometry (`src/mcu_prog.rs:473-620`), so it would scale too, but only after buffering all coefficients.
6. **Entropy decoding stays complete.** Every coefficient is Huffman-decoded to find block boundaries, as in libjpeg (`jdcoefct.c:114`). The saving is in the IDCT, upsampling, color conversion and output volume; how much that is for `zune-jpeg` is unmeasured.

A smaller upstream contribution is **row-band output** for baseline images: `post_process` already writes sequential bands, so a callback or `decode_rows` API would remove the full-frame output buffer for baseline JPEG without changing any kernel. Progressive files would still buffer coefficients (not pixels), which is the ceiling for that path.

Scaled IDCT output is a DCT-domain low-pass in gamma-encoded YCbCr, not Ferrisma's linear-light filter; libvips disables it in linear mode for that reason (section 6). ADR-0023 has to declare it as a separate quality policy before the planner may pick it.

## 2. `jxl-oxide` 0.12.6

### How a region request flows

`JxlImage::builder().read(reader)` reads the whole stream and feeds it to the frames (`src/lib.rs:237-282`); each frame stores the bytes of every TOC group (`jxl-frame/src/lib.rs:279-294`), so the complete codestream is held in memory and entropy decoding is deferred per group through `pass_group_bitstream` (`:563-608`). Frames with a single TOC entry (one group, one pass) have no per-group access (`:568-590`).

`set_image_region` stores the request (`src/lib.rs:791`; `jxl-render/src/lib.rs:232`). `render_frame_cropped` renders the keyframe with that region and crops the output stream to it (`src/lib.rs:715-740`). The renderer maps the display region to frame coordinates (`jxl-render/src/util.rs:19-40`), pads it for LF frames, upsampling, the edge-preserving filter (+2, +5 or +6 samples by iteration count), the Gabor filter (+1), chroma upsampling (rounding to even) and EPF's 8x8 blocks (`:51-120`).

### What is skipped

- **VarDCT:** the region is aligned to the group dimension (`128 << group_size_shift`, 256 by default; `jxl-render/src/vardct/mod.rs:98`, `jxl-frame/src/header.rs:31, 299-301`). Buffers are allocated for that aligned region only (`vardct/mod.rs:206-222`), LF groups are loaded for the corresponding LF region (`:165-171`), pass groups are entropy-decoded only for groups that cover the buffer (`:229-300`, `jxl-render/src/image.rs:670-700`), and dequantization, chroma-from-luma and the inverse transform are skipped for groups outside the region (`:325-341`).
- **Modular:** pass groups whose rectangle does not intersect the region are skipped (`jxl-render/src/modular.rs:81`).
- **Not skipped:** modular images whose global tree uses a palette or a squeeze transform expand the region to the whole frame (`modular.rs:156-170`); `ReferenceOnly` frames render whole (`util.rs:28-30`), reference frames used for blending render at their full size (`render.rs:190-193`), and the codestream bytes of every group remain resident. Patches, splines and noise were not traced (unverified).

The granularity is therefore one group, plus halos, per requested rectangle: a one-pixel crop still decodes at least one 256x256 group per channel. ADR-0012's "no guarantee" wording is correct in the general case and conservative for VarDCT photographs.

### Reduced-resolution rendering

There is no public LF-only, 1/2, 1/4 or 1/8 render. The only `downsample` helpers in `jxl-oxide`, `jxl-render` and `jxl-frame` are internal region arithmetic (`jxl-render/src/region.rs:141-180`). The 1/8 LF image is computed (`lf_xyb`, `vardct/mod.rs:165-205`, including adaptive LF smoothing and LF chroma-from-luma) and then consumed by the HF transform; it is never returned. Progressive partial rendering exists (`render_loading_frame`, `src/lib.rs:743-748`; `jxl-render/src/lib.rs:643-760`) and the VarDCT path falls back to an LF-only transform when HF data is absent (`vardct/mod.rs:338-341`), but the output buffer is still the full-size aligned region. LF frames (`lf_level > 0`) are upsampled before use (`jxl-render/src/lib.rs:882-884`). A reduced-resolution output needs an upstream API: either a render target for the LF image, or a "maximum downsampling" mode that stops after the pass whose `downsample` hint satisfies the request (`jxl-frame/src/header.rs:141` carries the per-pass 1, 2, 4, 8 hints when the encoder wrote several passes).

### Threading

`JxlThreadPool` wraps the global Rayon pool, a caller-supplied `Arc<rayon_core::ThreadPool>`, a new pool of N threads, or nothing (`jxl-threadpool/src/lib.rs:36-100`); it is passed through `JxlImageBuilder::pool` (`src/lib.rs:207`). With a pool, pass groups are decoded as one spawned job per group (`vardct/mod.rs:253-300`; `modular.rs:36, 94`), dequantization and transforms run with `for_each_vec` (`vardct/mod.rs:319`), as do EPF and Gabor (`filter/epf.rs:178`, `filter/gabor.rs:90`). `with_rayon_thread_pool` is the hook for an engine-owned pool (ADR-0022).

### Output and color

Rendering produces `f32` buffers per channel (`src/fb.rs:7-146`); `Render::stream()` writes interleaved rows cropped to the request as `f32`, `u16` or `u8` (`src/fb.rs:354`, `:454-463`), and `image_planar` returns per-channel planes (`src/lib.rs:1175`). The default output encoding is the image's enum color encoding, or its embedded ICC profile when parseable, else sRGB (`jxl-render/src/lib.rs:101-150`); XYB frames are converted to the requested encoding by `jxl-color`. The CMS is `NullCms` unless `set_cms` is called (`jxl-render/src/lib.rs:184, 200`); whether ICC-to-ICC requests fail or pass through under `NullCms` was not traced (unverified). Ferrisma reads `rendered_icc` and `rendered_cicp` and requests nothing, so it receives the image's own encoding, as `src/codecs/jxl.rs` assumes.

## 3. `avif-decode` 3.0.0 and `rav1d` 1.1.0

### Public interface and threading

`Decoder::from_avif` / `from_reader` parse the container with `avif-parse` and open one `rav1d` context with `n_threads = available_parallelism().min(32)`, `max_frame_delay = 1` and film grain applied (`src/lib.rs:72-86`; `src/rav1d.rs:24-46`). None of these are configurable. `to_image` decodes the color item, converts it to RGB, then decodes the alpha item as a second AV1 picture and merges it, unpremultiplying when the container says so (`src/lib.rs:88-129`; `src/image.rs:98-116`). Layered items are decoded layer by layer and the last picture kept (`src/rav1d.rs:72-140`).

### Planes exist but are private

`FrameTempRef::planes()` returns `Planes::{YuvPlanes8, YuvPlanes16, Mono8, Mono16}` with strided `ImgRef` views of Y, U and V, the chroma sampling (4:2:0, 4:2:2, 4:4:4, monochrome) and the bit depth from `Dav1dPicture.p.bpc` (`src/image.rs:12-36`, `:138-165`; `src/rav1d.rs:169-200`). The function and `decode_frame` are `pub(crate)` in private modules (`src/lib.rs:16-18`), so a consumer only sees `Image::{Rgb8, Rgba8, Rgb16, Rgba16, Gray8, Gray16}`. Exposing the planes is a small upstream change or a fork; nothing structural prevents it.

### Memory and output granularity

Known full-frame allocations per decode: the AVIF item copies made by `avif-parse` (`AvifData::primary_item`, `alpha_item`), the `rav1d` picture (planes with stride and padding, 16-bit storage above 8 bits) from `rav1d`'s default allocator, and the RGB(A) `ImgVec` that `yuv` 0.8.20 writes into (`src/image.rs:268-290`, `:190-204`); 16-bit planes are copied once more when unaligned (`:122-134`). `Dav1dSettings` has an `allocator` with `alloc_picture_callback` (`rav1d/include/dav1d/dav1d.rs:127-141`; `picture.rs:551-640`) that would let an engine place the picture in budgeted memory; `avif-decode` uses the default. There is no per-row or per-tile output: `dav1d_get_picture` returns whole pictures (`rav1d/src/lib.rs:584`). Ferrisma adds its own `avif-parse` preflight and `avif_meta` pass (ADR-0018) and, for more than 8 bits, an `f32` row conversion.

### `rav1d` itself

`rav1d` exposes dav1d's C API as `pub unsafe extern "C" fn dav1d_{default_settings, open, send_data, get_picture, close}` (`src/lib.rs:138-742`) and says "A Rust API is planned" (`README.md:145-156`). It builds on stable Rust for x86, x86-64 and aarch64 although `rust-toolchain.toml` pins `nightly-2025-05-01` (`README.md:11-14`); x86 needs NASM (`:16`); `src/*.rs` contains 565 `unsafe` tokens (grep). Its task model splits a frame into `TileEntropy`, `TileReconstruction`, `DeblockCols`, `DeblockRows`, `Cdef`, `SuperResolution`, `LoopRestoration` and film-grain tasks (`src/internal.rs:230-244`), so worker threads can overlap entropy decoding, reconstruction and post-filters of a single-tile still image; the speedup for one picture with `max_frame_delay = 1` is unmeasured. AV1 has no reduced-resolution decode mode, and neither dav1d nor libavif adds one (confirmed on libavif-rs in section 5).

## 4. `ravif` 0.13.0 and `rav1e` 0.8.1

### Threads and tiles

`with_num_threads(None)` uses the global Rayon pool; `Some(n)` makes `rav1e` build a dedicated pool of `n` threads (`src/av1encoder.rs:177-185`; `rav1e/src/api/config/mod.rs:172, 261-268`). The tile count is `threads.min(width * height / min_tile_size^2)`, where `min_tile_size` is 4096, 2048, 1024, 512, 256 or 128 pixels for speeds 0 to 5 and above, doubled above quality 80 (`src/av1encoder.rs:584-591`, `:652-656`), so tiles follow threads. Color and alpha are encoded as two concurrent `rayon::join` branches (`:441`). In Ferrisma's build all of this collapses to one thread (section 0), and the `tiles` value becomes 1.

### Speed presets

`with_speed(1..=10)`: "1 = very very slow, but max compression", "10 = quick, but larger file sizes and lower quality" (`src/av1encoder.rs:147-158`). Each value maps to `rav1e`'s `SpeedSettings::from_preset` (`rav1e/src/api/config/speedsettings.rs:115-195`) plus `ravif`'s overrides: multiref and scene detection off, one lookahead frame, partition range by speed and quality, complex prediction modes only at speed 1, bottom-up encoding up to 2, RDO transform decision up to 4 (not at high quality), fine directional intra up to 6, fast deblock from 7, loop restoration and CDEF only for low quality, inter transform split from 9, transform-domain rate from 10 (`src/av1encoder.rs:542-640`). The crate's own comments estimate some costs (for example "2x-3x slower, 2% better" for complex prediction modes); these are the author's notes, not measurements made here. Quality maps to a quantizer through a piecewise curve (`:513-517`).

### Input: 10-bit yes, 4:2:0 no, planes no, rows no

- `encode_rgba` / `encode_rgb` convert each pixel to BT.601 YCbCr at 8 or 10 bits; `BitDepth::Auto` (the default) means 10-bit, with 8-bit values expanded by bit replication (`src/av1encoder.rs:43-52`, `:240-262`, `:472-475`).
- `encode_raw_planes_8_bit` / `encode_raw_planes_10_bit` take `width`, `height`, an iterator of per-pixel `[P; 3]` triples, an optional alpha iterator, a pixel range and matrix coefficients (`:353-384`). Despite the name they are not planar, and the documentation states "Chroma subsampling is not supported" (`:342`); the internal config hard-codes `ChromaSampling::Cs444` and sRGB transfer with BT.709 primaries (`:391-395`, `:414`).
- `init_frame_3` copies the iterator row by row into a `rav1e` `Frame` (`:697-725`), which `Context::send_frame` requires complete (`rav1e/src/api/context.rs:51-57`, `:108-130`); the frame's planes carry `LUMA_PADDING = SB_SIZE + FRAME_MARGIN` on each side (`rav1e/src/frame/mod.rs:23, 69`). The encoder's working set is therefore the padded 4:4:4 planes (three 16-bit planes at 10 bits) plus `rav1e`'s opaque state, on top of Ferrisma's RGBA8 frame.
- The container writer gets only matrix coefficients and the premultiplied flag from `ravif` (`:443-452`); primaries and transfer are left at `avif-serialize`'s defaults, so any non-sRGB output Ferrisma might later produce would be mislabeled through `ravif`.

### Direct `rav1e` plus `avif-serialize`

`EncoderConfig` exposes `bit_depth`, `chroma_sampling` (`Cs420`, `Cs422`, `Cs444`, `Cs400`), `chroma_sample_position`, `pixel_range`, `color_description`, `tiles` or `tile_cols`/`tile_rows`, `still_picture` and the speed settings (`rav1e/src/api/config/encoder.rs:27-117`); `Config::with_threads` and `with_thread_pool` accept an engine-owned pool (`config/mod.rs:172-191`). `Context::new_frame()` allocates planes for the configured subsampling, and `Plane::copy_from_raw_u8` or `rows_iter_mut` fill them (`v_frame/src/plane.rs:352, 594`). `avif-serialize` 0.8.9 (BSD-3-Clause) writes `set_chroma_subsampling`, `set_monochrome`, `color_primaries`, `transfer_characteristics`, `matrix_coefficients` and `premultiplied_alpha` (`src/lib.rs:95-125`, `:390-400`), while its documentation discourages subsampling (`:43-44`, `:388`). So a Ferrisma adapter on `rav1e` directly can take 10-bit 4:2:0 planes and label the output correctly, using crates already in the tree; it still has to hand over a complete frame.

## 5. libavif-rs and libjxl-rs

Both checkouts were read at their 2026-11-05 HEADs; paths are repository-relative.

| | libavif-rs (`d025dca`) | libjxl-rs (`b5994bd`) |
| --- | --- | --- |
| Toolchain | `nightly-2026-09-07`; `#![feature(extern_types)]` in `src/lib.rs:7`, `core_intrinsics`, `c_variadic` in the aom and dav1d crates; `publish = false` on all four crates | `stable`, edition 2024, `rust-version = "1.95"`, no dependencies, `publish = false` |
| Target guard | `compile_error!` unless `aarch64-apple-darwin` (`src/lib.rs:13-14`); Linux x86-64 validated through the C harness but not admitted | Linux x86-64 and macOS AArch64 in CI; parity is against the scalar, non-vectorized oracle build |
| API shape | libavif's C ABI: `pub unsafe extern "C" fn avifDecoderParse` (`src/read.rs:9393`), `avifImageYUVToRGB` (`src/reformat.rs:3431`), `avifEncoderAddImage` (`src/write.rs:5264`); README row "Safe Rust API and stable toolchain: Deferred" (`README.md:67`) | Rust mirror of libjxl's C API with raw-pointer buffers: `jxl_decoder_set_image_out_buffer(dec, format, *mut c_void, size)` is `unsafe`; safe slice entry points exist only for fixed profiles; 1,513 `unsafe` tokens in `src` (grep) |
| Decoder backends | Rust ports of dav1d and libaom always linked; `libgav1`, `avm` as feature-gated native FFI | Full `decode.cc` port; C ABI exports all 52 `decode.h` functions plus the nine runner functions |
| Encoder backends | Rust libaom always linked; `rav1e`, `svt` (SVT-AV1) as feature-gated native FFI through `pkg-config`; SharpYUV ported | Only a fixed lossless profile (`LosslessThunderConfig::encode_pixels`, `src/jxl/enc_frame.rs:155`); no VarDCT encoder, no encoder C ABI, no container writer (`docs/ENCODER_EXPANSION.md`) |
| Planes and bit depth before RGB | `avifImage { depth, yuvFormat, yuvRange, yuvPlanes, yuvRowBytes, ... }` (`src/avif.rs:170-199`) | Not in libjxl's API; pixel output through buffers or row callbacks; `JxlBasicInfo` bit depth, `JxlPixelFormat` UINT8/UINT16/FLOAT16/FLOAT |
| Threads | `avifDecoder.maxThreads` (`src/read.rs:815`), `avifRGBImage.maxThreads`, `avifEncoder.maxThreads` (`src/avif.rs:297`) | `jxl_decoder_set_parallel_runner` merged 2026-11-05 (`src/jxl/decode.rs:1097`), but task bodies are serialized under one lock, "a runner brings no parallel speedup yet" (`docs/PORT_STATUS.md:51, 86`) |
| Reduced resolution | None (libavif has none) | `jxl_decoder_set_progressive_detail` (`:3723`, accepts `kDC`, `kLastPasses`, `kPasses`), `jxl_decoder_get_intended_downsampling_ratio` (`:2968`), `jxl_decoder_flush_image` (`:2976`), row callbacks (`:3310`): the libjxl progressive path, see section 6 |
| Encoder input | `avifEncoderAddImage(*const avifImage)` takes YUV planes at any depth 1..16 directly; `tileRowsLog2`, `tileColsLog2`, `autoTiling`, `speed`, `codecChoice` (`src/avif.rs:296-311`); RGB input through `avifImageRGBToYUV` with SharpYUV | n/a |
| Native build requirements | libvmaf through `pkg-config` even in the default build, plus libc++ (`crates/aom/build.rs:8-10`) | None for ordinary builds; the C++ oracle only behind the `cpp-oracle` test feature |
| Validation | Upstream GoogleTest/CTest suites driven through the C ABI (9,473 cases default, byte-exact AVIF/PNG output with `-ffp-contract=off`); 12 CI workflows | 961 Rust tests, 24 oracle-differential files; 53,050 recorded histories reproduce exactly through the C ABI drivers (`docs/PORT_STATUS.md:140-157`) |
| Distance to ADR-0018 gates | Missing: safe API, stable toolchain, publishable or taggable release, any target but macOS ARM64, a build without libvmaf | Missing: safe API, publishable release, lossy encoder, working parallelism, SIMD (`.cargo/config.toml` disables auto-vectorization in-repo) |

ADR-0018's description of both ports holds except for one point: libjxl-rs's thread runners are merged, not "in review", and libjxl-rs builds on stable Rust, which the ADR does not say.

### Update 2026-11-06: libavif-rs supports GNU Linux x86-64

The table above describes libavif-rs at `d025dca`. Its `main` at `f3b2981` (2026-11-06) admits a second target: the guard in `src/lib.rs:15-19` now accepts `aarch64-apple-darwin` and `x86_64-unknown-linux-gnu`, and Linux compiles generated native profiles (`src/lib.rs:29-31`, `include!("linux_profiles.rs")`) for six Cargo feature sets (README "Linux whole-pipeline support", ADR 0054). Unchanged: `nightly-2026-09-07` with `#![feature(extern_types)]`, the `unsafe` C ABI (README row "Safe Rust API and stable toolchain: Deferred"), the libvmaf requirement in `crates/aom/build.rs:8-10`, and `publish = false`. For ADR-0018's gates this removes "any target but macOS ARM64" from the missing list: Ferrisma's CI target (Linux x86-64) is now covered; a safe API, a stable toolchain and a build without libvmaf remain open.

## 6. Reference: libvips shrink-on-load and libjxl downsampling

### libvips with libjpeg-turbo

- `vips_thumbnail_find_jpegshrink` picks 8, 4, 2 or 1 for a common shrink of at least 16, 8 or 4, leaving "at least a factor of two for the final resize step", and returns 1 in linear mode because "libjpeg shrinks in Y (of YCbCr), not linear space" (`libvips/resample/thumbnail.c:547-575`). The loader is opened with `access = VIPS_ACCESS_SEQUENTIAL` and `shrink = factor`, and the residual shrink runs through `vips_resize`.
- `jpeg2vips.c` sets `cinfo->scale_denom = jpeg->shrink; cinfo->scale_num = 1;` and calls `jpeg_calc_output_dimensions` (`libvips/foreign/jpeg2vips.c:537-539`), trusts `image_width / shrink` rather than libjpeg's rounded-up size, runs a `FATSTRIP` pipeline (`:627`) whose generate function calls `jpeg_read_scanlines` one row at a time inside 8-row `vips_sequential` tiles (`:879`, `:933`). libvips never buffers the decoded frame in this path; libjpeg buffers all coefficients for progressive files, which libvips flags as `jpeg-multiscan`.
- libjpeg-turbo's `jpeg_core_output_dimensions` picks a DCT scaled size of 1 to 16 per component from `scale_num / scale_denom` (M/8, with 1/4 and 1/2 SIMD-accelerated; `README.md:171`, `libjpeg.txt:1380`), and `jpeg_calc_output_dimensions` lets chroma use a larger IDCT instead of upsampling (`jdmaster.c:99, 267`). `jinit_inverse_dct` selects `jpeg_idct_1x1`, `2x2`, `4x4` (`jidctred.c`) or `NxN` (`jidctint.c`) accordingly (`jddctmgr.c:113`). `decompress_onepass` still calls `decode_mcu` for every MCU and only skips the IDCT for blocks outside a horizontal crop (`jdcoefct.c:114, 134-164`). The TurboJPEG API says "scaling factors are generally limited to multiples of 1/8" (`turbojpeg.h`). The 6b documentation's sentence that smaller ratios "permit significantly faster decoding" is not in libjpeg-turbo's text; no speed figure is quoted here.
- Other loaders: WebP uses libwebp's `use_scaling` / `scaled_width` (post-decode rescaling, `webp2vips.c:616`); HEIF/AVIF through libheif has no shrink, only the embedded thumbnail item (`heifload.c:1187`); JPEG XL has an unimplemented `shrink` field with a TODO "read out 8x shrunk image?" and decodes whole frames with a parallel runner (`jxlload.c:69, 85, 204, 585`).

### libjxl

- `JXL_DEC_FRAME_PROGRESSION` fires when the 1:8 DC image is ready, and "the image data is still returned at full resolution, giving upscaled DC" (`lib/include/jxl/decode.h:306-316`). `JxlProgressiveDetail` offers `kDC`, `kLastPasses`, `kPasses` (implemented) and `kDCProgressive`, `kDCGroups`, `kGroups` (declared, not implemented) (`:340-360`); `JxlDecoderSetProgressiveDetail` (`:1424`), `JxlDecoderGetIntendedDownsamplingRatio` returning 1, 2, 4 or 8 (`:1434`), `JxlDecoderFlushImage` into the full-size output buffer (`:1452`) and `JxlDecoderSkipCurrentFrame` (`:426`) are the whole mechanism. There is no reduced-size output buffer and no region-of-interest decode in the public API; `JxlDecoderSetImageOutBitDepth` and `JxlDecoderSetParallelRunner` exist.
- `djxl --downsampling=1|2|4|8` sets `max_downsampling`; the extras decoder then subscribes to frame progression with `kLastPasses`, flushes when the intended ratio satisfies the request and skips the rest of the frame (`tools/djxl_main.cc:112`; `lib/extras/dec/jxl.cc:188-193, 484-496`). It only triggers when the encoder wrote passes with downsampling hints; decode.h warns the event "is not guaranteed to trigger".
- Format facts (`doc/format_overview.md`, `lib/jxl/frame_header.h`): VarDCT always carries a 1:8 LF image, optionally as a separate DC frame (`kDCFrame`, `dc_level` 1 to 4, dimensions `DivCeil(xsize, 1 << (3 * dc_level))`); the TOC enables "partial decoding of a region of interest or a progressive preview"; modular squeeze provides 1:8, 1:4, 1:2 stages. The format permits what the libjxl API does not expose; `jxl-oxide` exposes the region part.

## Capability matrix

Columns are the crates Ferrisma ships (`zune-jpeg`, `jxl-oxide`, `avif-decode` on `rav1d`) and the reference implementations. "Internal" means present in the code but not reachable through the public API.

| Capability | `zune-jpeg` 0.5.15 | `jxl-oxide` 0.12.6 | `avif-decode` 3.0.0 / `rav1d` 1.1.0 | libjpeg-turbo / libjxl / libavif-rs |
| --- | --- | --- | --- | --- |
| Reduced-resolution decode | No (`decoder.rs:265`, `mcu.rs:584-591`) | No public API; LF image internal (`vardct/mod.rs:165-205`) | No; AV1 has none | libjpeg-turbo M/8 IDCT scaling (`jdmaster.c:99`); libjxl DC flush into a full-size buffer (`decode.h:306-316`); libavif none |
| Region decode that skips work | No | Yes for VarDCT and plain modular at group granularity plus halos; full frame for squeeze/palette, LF and reference frames (`vardct/mod.rs:98-341`, `modular.rs:81, 156`) | No | libjpeg-turbo horizontal iMCU crop in `jdcoefct.c:134`; libjxl none; libavif none |
| Row or band output | Internal: baseline writes bands of 8/16 rows into a full-frame caller buffer (`mcu.rs:252-260`); progressive buffers all coefficients (`mcu_prog.rs:59`) | Rows streamed from a region-sized rendered buffer (`fb.rs:354`) | No; whole picture (`rav1d/src/lib.rs:584`) | libjpeg scanlines (`jpeg2vips.c:879`); libjxl row callbacks (`decode.h:1130`); libavif `avifDecoderDecodedRowCount` for incremental input |
| Native planar / YUV output | No (RGB, BGR, Luma, CMYK/YCCK interleaved) | `image_planar` per channel, XYB already converted (`lib.rs:1175`) | Internal: `Planes` with strides, 4:2:0/4:2:2/4:4:4, 8 to 16 bits (`image.rs:12-36`, `:138`), `pub(crate)` | libavif-rs `avifImage.yuvPlanes` (`src/avif.rs:177`); libjxl none |
| Bit depth before conversion | 8-bit | f32 everywhere; `color_bit_depth` reported | Internal `bpc` 8/10/12/16 (`rav1d.rs:169`) | libavif-rs `avifImage.depth` |
| Threads | None (`README.md:58`) | Rayon pool, off in Ferrisma (`lib.rs:192-195`) | `rav1d` `n_threads = min(cores, 32)`, not configurable (`lib.rs:80`) | libjpeg none; libjxl runners; libjxl-rs serialized; libavif-rs `maxThreads` |
| Known full-frame buffers | Caller output buffer; whole coefficients for progressive | Whole codestream; group-aligned region buffers | Item copies, `rav1d` picture, RGB(A) frame (`image.rs:268`) | libvips none for baseline JPEG |
| `unsafe` | SIMD only, compiled out by Ferrisma | None in `jxl-oxide` (not audited here) | `rav1d` throughout, C-ABI calls in `avif-decode/src/rav1d.rs` | libavif-rs and libjxl-rs: C-ABI mirrors |

| Encoder capability | `ravif` 0.13.0 | `rav1e` 0.8.1 direct | libavif-rs |
| --- | --- | --- | --- |
| Threads | `with_num_threads` (`av1encoder.rs:181`); shim in Ferrisma's build | `with_threads`, `with_thread_pool` (`config/mod.rs:172-191`) | `avifEncoder.maxThreads` |
| Tiles | Derived from threads and speed (`:652-656`) | `tiles`, `tile_cols`, `tile_rows` (`encoder.rs:96-113`) | `tileRowsLog2`, `tileColsLog2`, `autoTiling` |
| Speed presets | 1 to 10 with overrides (`:542-640`) | 0 to 10 (`speedsettings.rs:115`) | `speed` per codec |
| 4:2:0 input | No, `Cs444` hard-coded (`:414`) | Yes, `ChromaSampling::Cs420` (`encoder.rs:42`) | Yes, `avifImage` planes |
| 10-bit input | Yes, per-pixel triples 0..=1023 (`:377`) | Yes, `Context<u16>` with `bit_depth` 10 or 12 | Yes, depth 1..16 |
| Planar input | No, pixel iterator (`:353-384`) | Yes, `Plane` rows (`v_frame/src/plane.rs:352`) | Yes |
| Row or tile input | No; complete `Frame` (`:697-725`) | No; complete `Frame` (`context.rs:108`) | No; complete `avifImage` |
| Color signalling | BT.601 matrix only; primaries/transfer fixed at defaults (`:391-395`, `:443-452`) | `color_description` in the bitstream; `avif-serialize` writes `colr` (`avif-serialize/src/lib.rs:95-125`) | Full `colr`, `clli`, `mdcv` |
| Alternative codecs | None | None | Rust libaom; SVT-AV1 and rav1e through native FFI features |

## Implications for work package 2 (encoder throughput)

**Usable now, with Ferrisma-side changes only.**

- Enable `ravif`'s `threading` feature (and measure `asm` separately) before any thread-count experiment; otherwise `with_num_threads` is a no-op. `asm` pulls NASM on x86 and assembly into the `unsafe` boundary that ADR-0018 still has under review; measure it as a separate configuration.
- Sweep `with_num_threads` and `with_speed` on the package-0 corpus at matched SSIMULACRA2, recording bytes and time; tiles follow threads automatically, so a tile sweep is a thread sweep unless `rav1e` is driven directly.
- Pass a Ferrisma-owned pool: `ravif` cannot take one, `rav1e`'s `Config::with_thread_pool` can. That is the first reason to bypass `ravif`.

**Needs an adapter on `rav1e` directly (no upstream work).** 4:2:0 input, 10-bit planes from Ferrisma's working values without the 8-bit sRGB quantization step, correct `colr` for non-sRGB output, tile counts independent of threads, and an engine-owned pool. `avif-serialize` already writes the container. The adapter still materializes one padded frame per encode, which is what the AVIF sink does today in RGBA8; the plan must keep reporting it.

**Ceiling.** `rav1e` has no row or tile input, so the encoder cannot consume the decoder-ahead ring of package 4 directly; the best achievable shape is "resample into the encoder's planes, then encode". Encoder alternatives through libavif-rs are not adoptable until the port offers a safe API on stable Rust off macOS-only and without libvmaf; SVT-AV1 and rav1e there are native FFI anyway. Hardware encode (package 8) stays the escape hatch.

## Implications for work package 5 (decode-side scaling and native layouts)

**JPEG.** No decode-side scale is available from any Rust decoder in the tree. The options, in increasing cost: (1) a row-band output API in `zune-jpeg` (baseline only), which gives package 4 a streaming source and removes the full-frame output buffer; (2) a scaled-IDCT contribution with the six touchpoints of section 1, scalar first, with the quality policy of ADR-0023 declared before the planner uses it; (3) a different adapter, which today means libjpeg-turbo through FFI or a port, both outside the current dependency policy. The reference saves IDCT, upsampling, color conversion and output volume, never entropy decoding, so even option 2 has a floor that package 0 should measure (time spent in Huffman decoding versus the rest).

**JPEG XL.** The region request is real for photographs: a crop to one group skips the other groups' entropy decoding and transforms. The achieved decode region should enter the plan as the group-aligned, halo-padded rectangle rather than the requested crop. Reduced resolution needs an upstream API in `jxl-oxide` (an LF render target or a maximum-downsampling stop); both are contributions, not adapter work. Enable the `rayon` feature with `JxlImageBuilder::pool(JxlThreadPool::with_rayon_thread_pool(engine_pool))` so the decoder joins the engine's pool instead of the global one (ADR-0022). The whole codestream in memory is a ceiling for very large inputs unless `jxl-oxide` grows a streaming group store.

**AVIF.** No reduced-resolution decode exists at the format level; the decode-side gain is the native 10-bit 4:2:0 path. That path needs the planes, which `avif-decode` has but does not export; the shortest route is an upstream change or fork exposing `Planes`, the next is driving `rav1d` directly behind an isolated `unsafe` adapter crate (the boundary ADR-0020 is defining), and the long route is libavif-rs. On the encoder side the same path needs the direct `rav1e` adapter from package 2. Chroma siting (`chroma_sample_position`), range and matrix must travel with the planes, and ADR-0011's even-origin crop and no-planar-resize rules apply until the gamma-domain native policy exists. `rav1d` spawns its own threads outside the engine's pool and `avif-decode` cannot change that, so package 4's budget and scheduler must account for an uncontrolled worker set when AVIF input is present.

**Reporting.** None of the three decoders can report a scale factor other than 1 today; the plan should print "requested scale, achieved scale 1, reason" per adapter rather than omit the field.

## Open questions

1. How much of `zune-jpeg`'s baseline time is entropy decoding? That number bounds the benefit of a scaled IDCT before anyone writes it.
2. Does `jxl-oxide`'s group skipping hold with patches, splines and noise enabled, and for images with a single TOC entry? Trace or measure with the package-0 corpus.
3. What does `jxl-oxide` emit for an ICC-tagged image under `NullCms` when a different encoding is requested: an error or untransformed samples?
4. What speedup does `rav1d`'s task model give a single-tile still image at `max_frame_delay = 1`, and does it collide with an engine pool of the same size?
5. Would `kornelski/avif-decode` accept exposing `Planes` and a thread setting? If not, is a `rav1d` adapter crate acceptable under ADR-0020's boundary?
6. `ravif`'s `encode_raw_planes_*` documentation calls subsampling "a bad idea for AVIF"; is 4:2:0 a size win at matched SSIMULACRA2 for Ferrisma's corpus, or only a memory win?
7. When libjxl-rs gains a lossy encoder, will its runner serialization be lifted, and will the decoder's row callbacks be usable from a safe wrapper?
8. Which reduced-resolution API would `jxl-oxide` upstream prefer: an LF render target or a libjxl-style maximum-downsampling stop?

## Sources

- Crate sources under `~/.cargo/registry/src/index.crates.io-*/` for the versions in `Cargo.lock`; feature resolution with `cargo tree --offline -e features`.
- Ferrisma: `Cargo.toml`, `Cargo.lock`, `src/codecs/{jpeg,jxl,avif}.rs`, `src/avif_sink.rs`, [ADR-0011](../adr/0011-borrowed-planar-surface-contract.md), [ADR-0012](../adr/0012-codec-source-contract.md), [ADR-0018](../adr/0018-optional-codec-and-cms-dependencies.md), [M2 plan](../milestones/M2-performance-architecture.md), [format capabilities](format-capabilities.md), [codec adapter audit](codec-adapter-audit.md).
- Sister ports: `/Users/sebastian/Workspace/libavif-rs` (`README.md`, `docs/PLATFORMS.md`, `docs/FUTURE.md`, `src/{avif,read,write,reformat}.rs`, `crates/aom/build.rs`) and `/Users/sebastian/Workspace/libjxl-rs` (`README.md`, `docs/PORT_STATUS.md`, `docs/ENCODER_EXPANSION.md`, `src/jxl/decode.rs`).
- [libvips thumbnail.c](https://github.com/libvips/libvips/blob/master/libvips/resample/thumbnail.c), [jpeg2vips.c](https://github.com/libvips/libvips/blob/master/libvips/foreign/jpeg2vips.c), [jxlload.c](https://github.com/libvips/libvips/blob/master/libvips/foreign/jxlload.c), [webp2vips.c](https://github.com/libvips/libvips/blob/master/libvips/foreign/webp2vips.c), [heifload.c](https://github.com/libvips/libvips/blob/master/libvips/foreign/heifload.c)
- [libjpeg-turbo libjpeg.txt](https://github.com/libjpeg-turbo/libjpeg-turbo/blob/main/doc/libjpeg.txt), [jdmaster.c](https://github.com/libjpeg-turbo/libjpeg-turbo/blob/main/src/jdmaster.c), [jddctmgr.c](https://github.com/libjpeg-turbo/libjpeg-turbo/blob/main/src/jddctmgr.c), [jidctred.c](https://github.com/libjpeg-turbo/libjpeg-turbo/blob/main/src/jidctred.c), [jdcoefct.c](https://github.com/libjpeg-turbo/libjpeg-turbo/blob/main/src/jdcoefct.c), [turbojpeg.h](https://github.com/libjpeg-turbo/libjpeg-turbo/blob/main/src/turbojpeg.h)
- [libjxl decode.h](https://github.com/libjxl/libjxl/blob/main/lib/include/jxl/decode.h), [frame_header.h](https://github.com/libjxl/libjxl/blob/main/lib/jxl/frame_header.h), [format_overview.md](https://github.com/libjxl/libjxl/blob/main/doc/format_overview.md), [djxl_main.cc](https://github.com/libjxl/libjxl/blob/main/tools/djxl_main.cc), [extras/dec/jxl.cc](https://github.com/libjxl/libjxl/blob/main/lib/extras/dec/jxl.cc)
- [rav1d issue #1252: Rust API](https://github.com/memorysafety/rav1d/issues/1252)
