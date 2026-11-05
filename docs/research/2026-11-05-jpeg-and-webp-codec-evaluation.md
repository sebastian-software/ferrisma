# JPEG and WebP codec evaluation (5 November 2026)

**Status:** Research notes and two recorded decisions; no dependency added. Crate facts were read from crates.io, lib.rs and docs.rs on 2026-11-05. Performance and parity figures quoted from crate READMEs are the authors' own and were not reproduced here.

## Decisions

1. **JPEG stays input-only.** `zune-jpeg` remains the `jpeg-read` adapter ([ADR-0012](../adr/0012-codec-source-contract.md)). No JPEG encoder is planned; the permissive candidate is recorded below for the day a consumer needs one.
2. **WebP input and output come from a mechanical Rust port of libwebp.** Ferrixel takes no interim WebP dependency. The port is planned in [the libwebp port plan](2026-11-05-libwebp-port-plan.md) and recorded as the WebP target in [ADR-0018](../adr/0018-optional-codec-and-cms-dependencies.md).

## JPEG

### Decoding (kept)

`zune-jpeg` 0.5 (MIT OR Apache-2.0 OR Zlib, Rust 1.77) stays. The `image` crate made it its default JPEG decoder in 0.25 with libjpeg-turbo-class speed; the jpegli-rs README lists its throughput at roughly eight times that of jpegli-rs's own decoder. Lossless JPEG (medical imaging) is not supported, which Ferrixel does not need.

### Encoding candidates (not adopted)

| Crate | Version | License | `unsafe` | Verified interface and evidence |
| --- | --- | --- | --- | --- |
| `mozjpeg-rs` | 0.9.2 (2026-04-17) | BSD-3-Clause | `#![forbid(unsafe_code)]`, safe SIMD via `archmage` | Encoder only; no decoder, no decode feature. Baseline and progressive output byte-identical to C mozjpeg per FFI comparison tests; trellis on by default. Streaming encoder holds one MCU row; `Limits` checked before allocation; cancellation token; ICC/EXIF/XMP embedding. Rust 1.89. |
| `jpeg-encoder` | 0.7.1 (2026-07-27) | (MIT OR Apache-2.0) AND IJG | none by default; optional AVX2 feature adds `unsafe` | Baseline and progressive, optimized Huffman, subsampling, `no_std`; no runtime dependencies. One independent comparison measured roughly 2.6x larger files than mozjpeg at nominal quality 90; nominal qualities are not calibrated, so this is not a quality-matched result. |
| `zenjpeg` (formerly `jpegli-rs`) | 0.8.4 (2026-06-01) | AGPL-3.0-only OR commercial | `#![forbid(unsafe_code)]` | Encoder and decoder with jpegli-class compression (author-reported). Blocked by the license gate in ADR-0018; the decoder API is declared prerelease. |
| `jpegli-rs` | 0.12.0 (2026-01-24) | AGPL-3.0-or-later | opt-in `unsafe_simd` | Being renamed to `zenjpeg`; same gate. |

C and C++ encoders (mozjpeg, libjpeg-turbo, jpegli in libjxl's `lib/jpegli`, all permissive) would require FFI or a port. jpegli lives in the libjxl tree, so a Rust jpegli would belong to libjxl-rs, not to Ferrixel. Not pursued.

**Why no encoder now.** M1 ends in AVIF; the roadmap lists historical JPEG output tuning under "Not prioritized initially". If a consumer needs JPEG output, `mozjpeg-rs` is the candidate to run through the adapter checklist in [the encoder research](2026-10-31-encoder-and-cms-candidates.md); its one-MCU-row streaming encoder and pre-allocation limits match Ferrixel's memory contract on paper. The README states that not all code was manually reviewed.

## WebP

### Rust crates

| Crate | Version | License | `unsafe` | Verified interface and evidence |
| --- | --- | --- | --- | --- |
| `image-webp` | 0.2.4 (2025-08-27) | MIT OR Apache-2.0 | `#![forbid(unsafe_code)]`, no unsafe dependencies | Decoder: VP8 lossy, VP8L lossless, alpha, animation, ICC/EXIF/XMP chunks; reads from `BufRead + Seek` into a caller buffer; `set_memory_limit` is documented as not covering every allocation. No decode-side crop or scale. Encoder: lossless only. Decode speed 70–100 % of libwebp (author-reported). Rust 1.80. |
| `zenwebp` | 0.4.6 (2026-10-24) | AGPL-3.0-only OR commercial | `#![forbid(unsafe_code)]` | Fork of `image-webp` plus a lossy encoder that follows libwebp's algorithms; bit-exact lossy decode and sizes within 0.02 % of libwebp at method 5 (author-reported). Blocked by the license gate. Demonstrates that a Rust port of libwebp's encoder is feasible. |
| `webpx` | 0.4.1 (2026-10-25) | MIT OR Apache-2.0 | FFI | Typed wrapper around libwebp via `libwebp-sys` 0.14; needs a C toolchain; offers decode-side crop and scale and incremental decoding because libwebp does. Outside the pure-Rust preference. |
| `webp` | 0.3.1 (2025-08-29) | MIT OR Apache-2.0 | FFI | Older `libwebp-sys` 0.9 wrapper. |
| `webp-rust`, `webpkit`, `gamut-webp`, `oxideav-webp`, `vaam-image-webp` | 0.1–0.3 | MIT or MIT OR Apache-2.0 | varies | Early lossy VP8 encoders without parity evidence or corpus results. Not candidates. |

No permissively licensed, pure-Rust lossy WebP encoder with parity evidence exists.

### C and C++ implementations

| Implementation | License | Lossy encoder | Decoder | Notes |
| --- | --- | --- | --- | --- |
| libwebp 1.6.0 (Google) | BSD-3-Clause + WebM patent grant | yes, the reference | yes: crop and scale at decode, incremental decoder, planar YUVA output | Plain C, no C++ in the libraries; SIMD in separate files per architecture |
| libvpx VP8 (WebM) | BSD-3-Clause + WebM patent grant | yes, as a video encoder | yes | No WebP container, no VP8L, no alpha; `vp8/encoder` + `vp8/common` + `vpx_dsp` are several times libwebp's size |
| FFmpeg `webp.c`, `vp8.c` | LGPL-2.1-or-later | no; `ff_libwebp_encoder` wraps libwebp | yes | License incompatible with the default dependency policy |
| Wuffs `std/webp`, `std/vp8` (Google) | Apache-2.0 | no | yes | Written in the Wuffs language and transpiled to C; decoder only |

No independent lossy WebP encoder exists in C or C++. Independent lossy encoders exist only in Go (`webp-go`, MIT, written from RFC 6386) and in Rust (`zenwebp`, AGPL).

### Why port libwebp

- It is the only complete lossy WebP encoder with a permissive license, and it is the reference implementation every other tool wraps.
- Its decoder exposes what [format-capabilities.md](format-capabilities.md) lists as open for WebP: cropping and scaling at decode (`WebPDecoderConfig.options`), incremental decoding (`WebPIDecoder`) and planar YUVA output (`WebPDecodeYUV`, `WebPINewYUVA`). `image-webp` offers none of these.
- Size is within what the sister ports already handled. Non-blank, non-comment lines in libwebp 1.6.0: `src/enc` 11,744; `src/dec` 6,073; portable `src/dsp` 8,270; `src/utils` 3,276; public headers 928; `sharpyuv` 1,462; `src/mux` and `src/demux` 3,837; SIMD variants 19,607 (of which 10,402 MIPS/MSA). The libavif source tree ported by libavif-rs has about 21,000 lines.
- libavif-rs already pins libwebp 1.6.0 at `4fa21912338357f89e4fd51cf2368325b59e9bd9` under `upstream/libwebp`, records its license in `docs/NATIVE-LICENSES.md` and ships a translated SharpYUV crate with a 1,271-group C-versus-Rust comparison inventory.
- License: BSD-3-Clause plus the separate WebM patent grant in `PATENTS`, the same terms libavif-rs already carries for SharpYUV. A BSD-3-Clause port enters the MIT OR Apache-2.0 dependency tree like `ravif` does.

### Options kept open

- **`image-webp` as an interim read-only adapter.** Permissive and safe, but without lossy encoding or decode-side cropping, and it would be replaced by the port's decoder, which is the first port milestone. Revisit only if WebP input is needed before that milestone.
- **`webpx` (libwebp through FFI).** Rejected for the same reasons as libavif through FFI in ADR-0018: native toolchain and an unsafe boundary in the core build.

## Sources

- [docs.rs: mozjpeg_rs](https://docs.rs/mozjpeg-rs/latest/mozjpeg_rs/), [GitHub: imazen/mozjpeg-rs](https://github.com/imazen/mozjpeg-rs)
- [lib.rs: jpeg-encoder](https://lib.rs/crates/jpeg-encoder), [jpeg-encoder versus mozjpeg comparison](https://github.com/jordanfelle/nicti/issues/223)
- [lib.rs: zenjpeg](https://lib.rs/crates/zenjpeg), [lib.rs: jpegli-rs](https://lib.rs/crates/jpegli-rs)
- [lib.rs: image-webp](https://lib.rs/crates/image-webp), [docs.rs: image_webp::WebPDecoder](https://docs.rs/image-webp/latest/image_webp/struct.WebPDecoder.html)
- [lib.rs: zenwebp](https://lib.rs/crates/zenwebp), [lib.rs: webpx](https://lib.rs/crates/webpx), [lib.rs: webp](https://lib.rs/crates/webp)
- [libwebp](https://chromium.googlesource.com/webm/libwebp), [libwebp-test-data](https://chromium.googlesource.com/webm/libwebp-test-data), [libvpx](https://chromium.googlesource.com/webm/libvpx)
- [FFmpeg libavcodec/allcodecs.c](https://github.com/FFmpeg/FFmpeg/blob/master/libavcodec/allcodecs.c)
- [Wuffs std/webp](https://github.com/google/wuffs/blob/main/std/webp/README.md), [Wuffs std/vp8](https://github.com/google/wuffs/tree/main/std/vp8)
- [webp-go](https://libraries.io/go/github.com%2Flinzeyan%2Fwebp-go)
- [RFC 6386: VP8 Data Format and Decoding Guide](https://www.rfc-editor.org/rfc/rfc6386), [RFC 9649: WebP Image Format](https://www.rfc-editor.org/rfc/rfc9649)
