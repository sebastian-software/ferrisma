# ADR-0030: Still WebP output and explicit encoder format

- **Status:** Accepted
- **Date:** 2026-11-11
- **Last reviewed:** 2026-11-26
- **Partially supersedes:** [0017](0017-minimal-public-facade-and-cli.md), AVIF-only result and feature gating; [0024](0024-pluggable-encoders-and-libavif-rs-backend.md), backend feature coupling and result naming
- **Related:** [0015](0015-typed-output-color-signals.md), [0017](0017-minimal-public-facade-and-cli.md), [0018](0018-optional-codec-and-cms-dependencies.md), [0024](0024-pluggable-encoders-and-libavif-rs-backend.md), [0026](0026-codecs-in-one-repository-and-workspace.md), [0029](0029-webp-still-input.md), [M1](../milestones/M1-complete-image-pipeline.md), issues [#16](https://github.com/sebastian-software/ferrisma/issues/16) and [#60](https://github.com/sebastian-software/ferrisma/issues/60)

## Context

The implemented pipeline only exposed an AVIF byte field and compiled `rav1e` even for an external encoder. The workspace WebP port now provides a safe still encoder, muxer, and decoder. It requires a complete input picture before analysis. Its lossy ARGB import allocates YUV420 planes; its other internal state is opaque. WebP has no native CICP signal, so the output's ICC must describe the final sRGB samples rather than copy an input profile.

## Decision

Keep the existing ordered, straight-alpha sRGB8 `RowEncoder` boundary. `pipeline` and `EncoderBackend` compile without `avif-write`. `Output` carries `format` and `bytes`; `EncoderBackend::format` and `EncodedImage::format` must agree with the requested format. `EncodeRequest` carries the `OutputSettings` (format and typed settings) to external backends too (ADR-0040, 2026-11-16). Built-in output chooses AVIF or WebP explicitly. AVIF retains its existing quality, speed, chroma, and threading behavior. WebP uses separate `WebpOptions`: lossy quality/speed or lossless level, which has no quality setting. WebP rejects explicit AVIF chroma and multiple encoder threads.

The WebP sink fills one budgeted ARGB frame, then reserves the predictable YUV420(A) plane size before lossy encoding. It sets `exact(true)` to preserve the filtered RGB under zero alpha. The codec may still allocate opaque scratch. The bounded writer stops before the configured encoded-output limit; AVIF and external backends are checked after they return because their APIs materialize their result. The sink wraps the image with the committed sRGB2014 ICC profile through the port's muxer. It does not copy source ICC, EXIF orientation, or XMP. The final encoded result is a caller-owned buffer outside the managed pixel-memory budget; the report names the known encoder frame materialization and flags opaque memory.

Not implemented: animation, metadata passthrough, WebP SharpYUV, direct YUV-to-AOM (#60).

## Alternatives

- A generic output plugin registry would add a public discovery and capability contract before a use case requires it. The two built-in formats and the existing backend trait suffice.
- Reuse AVIF `quality` for lossless WebP would silently ignore a caller option. A format-specific lossless level makes this impossible.
- Import each RGBA row directly into the codec would imply streaming encode despite libwebp's full-picture analysis. The explicit frame preserves truthful memory accounting.
- Copy input metadata would misdescribe transformed pixels and retain a stale orientation. A fixed output sRGB ICC matches the quantized samples.

## Consequences

The public experimental result field changes from `avif_bytes` to `bytes` and includes `format`; external backends add a format declaration and tag their result. A backend-only build no longer links `rav1e`. WebP output has an explicit full-frame materialization and bounded retained file, but no total-process-memory bound because libwebp's encoder and muxer allocate internally. The embedded, unchanged `sRGB2014.icc` asset is the committed libavif test fixture, SHA-256 `384b832de3412066743b52a75ee906b6fb9fb8d9e09e936fc2c43223815c6e0a`. [Upstream provenance](../../codecs/libavif-rs/upstream/libavif/tests/data/README.md) attributes it to the International Color Consortium (2015); the [ICC profile license](https://registry.color.org/profile-library/#licensing) permits unchanged redistribution. It is not covered by the libwebp BSD notice.

## Validation

Focused tests exercise WebP lossy and lossless output, ICC payload equality, metadata omission, straight alpha, exact lossless decode against the pipeline's finished RGBA rows, frame and file limits, and a backend-only build without `avif-write`. The same bytes are also decoded with the independent C libwebp 1.6.0 `dwebp` tool in local verification. No speed or peak-RSS claim is made.

## Review when

Review if WebP's API exposes bounded scratch, a lower-copy mux path, a real streaming encode, or a supported richer color-output contract; if the committed ICC's provenance is unsuitable for distribution; or when direct AOM YUV measurement resolves the remaining part of issue #60.

## Open questions

- Direct YUV-to-AOM encoding: #60.

## Amendment log

- 2026-11-11 — Initial experimental still WebP output and encoder-result decision.
- 2026-11-26 — Finalized as a living record, owner decision: status Accepted; stale passages corrected in place, dropped decisions marked, open questions limited to open issues.
- 2026-11-26 — Pre-reserve checked padded RIFF size on the first WebP output chunk (#282). `WebpSink::finish` previously called `bytes.try_reserve_exact(chunk.len())` on every `libwebp-rs` callback, reallocating and copying on each chunk (including a trailing 1-byte RIFF pad byte). It now parses the 12-byte `RIFF`/`WEBP` header on the first callback, checks the padded total against `max_output_bytes`, reserves that exact total once, and falls back to `try_reserve(chunk.len())` for any unexpected tail. Per-callback capacity/pointer stability is pinned by `webp_output_tests::webp_chunk_writer_reserves_padded_riff_size_without_pad_reallocation`; before/after writer and peak-RSS measurements are recorded in [2026-11-26: WebP chunk writer pre-reservation measurement (#282)](#2026-11-26-webp-chunk-writer-pre-reservation-measurement-282).

## 2026-11-26: WebP chunk writer pre-reservation measurement (#282)

### Multi-chunk writer growth, pointer moves, and time

Run the committed `#[ignore]` harness in `src/webp_output_tests.rs` (collecting capacity-growth and pointer-move counters outside the timing loops, then timing 1 warmup + 7 interleaved repetitions comparing pre-#282 per-chunk `try_reserve_exact(chunk.len())` against the production `WebpChunkWriter::push_chunk` on multi-chunk odd-padded lossless and lossy WebP encodes):

```sh
cargo test --release -p ferrisma --lib --no-default-features --features webp-read,webp-write \
  -- --ignored --nocapture --exact webp_output_tests::bench_webp_chunk_writer_growth_and_peak
```

Results (**indicative; loaded host**, Apple M1 Ultra, `arm64`, 20 cores, load averages 20.37 / 16.98 / 17.48; identical output bytes across baseline and candidate; "moved bytes" counts `before_len` only when `as_ptr()` changed, and "copy upper bound" sums `before_len` across all capacity-growth requests in case another allocator cannot grow in place):

| Case | Encoded bytes | Chunks | Baseline growths / ptr moves (moved B; copy upper bound) | Candidate growths / ptr moves | Peak capacity (baseline / candidate) | Writer median (baseline / candidate) |
| --- | ---: | ---: | --- | ---: | ---: | ---: |
| `lossless-alpha-1024x768-padded` | 1,716,960 B | 3 (`21, 1716938, 1`) | 2 / 1 (21 B; upper bound 1,716,980 B) | 0 / 0 | 1,716,960 B / 1,716,960 B | 38.1 µs / 37.1 µs |
| `lossy-alpha-2048x1536` | 1,024,278 B | 9 (`12, 18, 8, 10647, 1, 8, 10, 28609, 984965`) | 8 / 4 (10,784 B; upper bound 82,162 B) | 0 / 0 | 1,024,278 B / 1,024,278 B | 23.7 µs / 22.0 µs |
| `lossy-opaque-2048x1536` | 1,003,422 B | 5 (`12, 8, 10, 28607, 974785`) | 4 / 3 (28,679 B; upper bound 28,699 B) | 0 / 0 | 1,003,422 B / 1,003,422 B | 22.1 µs / 20.9 µs |

### End-to-end 24 Mpx CLI peak RSS (`6000×4000`)

To check whether upfront RIFF reservation increases peak process memory while `libwebp-rs` still holds its bit-writer partition buffers, build both the baseline (`cd5c63a6`, saved as `/tmp/ferrisma-webp-before`) and the candidate (`/tmp/ferrisma-webp-after`) with:

```sh
cargo build --release -p ferrisma --example cli_subset \
  --no-default-features --features jpeg-read,webp-read,webp-write,icc
```

Measure `encode_us`, `total_us`, and `/usr/bin/time -l` peak RSS across 1 warmup + 7 interleaved repetitions on `codecs/libavif-rs/upstream/libavif/tests/data/dog_exif_extended_xmp_icc.jpg`:

```sh
# Lossless 24 Mpx (20,115,998 B output)
/usr/bin/time -l /tmp/ferrisma-webp-before codecs/libavif-rs/upstream/libavif/tests/data/dog_exif_extended_xmp_icc.jpg /tmp/webp-b.webp 6000 4000 --webp-lossless --webp-level=0 --use-rendered-icc --transform-threads=1
/usr/bin/time -l /tmp/ferrisma-webp-after  codecs/libavif-rs/upstream/libavif/tests/data/dog_exif_extended_xmp_icc.jpg /tmp/webp-a.webp 6000 4000 --webp-lossless --webp-level=0 --use-rendered-icc --transform-threads=1

# Lossy 24 Mpx (1,856,856 B output)
/usr/bin/time -l /tmp/ferrisma-webp-before codecs/libavif-rs/upstream/libavif/tests/data/dog_exif_extended_xmp_icc.jpg /tmp/webp-b.webp 6000 4000 --webp-quality=80 --webp-method=0 --use-rendered-icc --transform-threads=1
/usr/bin/time -l /tmp/ferrisma-webp-after  codecs/libavif-rs/upstream/libavif/tests/data/dog_exif_extended_xmp_icc.jpg /tmp/webp-a.webp 6000 4000 --webp-quality=80 --webp-method=0 --use-rendered-icc --transform-threads=1
```

Results (**indicative; loaded host**, Apple M1 Ultra, `arm64`, 20 cores, load averages 32.35 / 23.49 / 19.09; output SHA-256 byte-identical across all runs):

| Workload (`6000×4000`, 24 Mpx) | Configuration | Encode median | Total median | Peak RSS median (max) | Output SHA-256 |
| --- | --- | ---: | ---: | ---: | --- |
| Lossless WebP (`20,115,998` B) | baseline (`cd5c63a6`) | 1158.47 ms | 1597.49 ms | 582.88 MiB (583.66 MiB) | `d0282dc537bde632…` |
| Lossless WebP (`20,115,998` B) | candidate (`#282`) | 1162.46 ms | 1585.34 ms | 582.97 MiB (584.78 MiB) | `d0282dc537bde632…` |
| Lossy WebP (`1,856,856` B) | baseline (`cd5c63a6`) | 595.52 ms | 1043.32 ms | 235.64 MiB (237.11 MiB) | `b678f2690f1653fb…` |
| Lossy WebP (`1,856,856` B) | candidate (`#282`) | 608.89 ms | 1062.24 ms | 235.64 MiB (238.00 MiB) | `b678f2690f1653fb…` |

Reserving the checked padded RIFF size on the first header callback eliminates all subsequent reallocations (0 vs 2–8) with no peak RSS regression (`+0.09 MiB` lossless, `+0.00 MiB` lossy), because `libwebp-rs` emits the RIFF header only after its partition bit-writers are finalized.
