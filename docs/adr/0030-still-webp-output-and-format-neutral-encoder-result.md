# ADR-0030: Still WebP output and explicit encoder format

- **Status:** Experimental
- **Date:** 2026-11-11
- **Last reviewed:** 2026-11-11
- **Partially supersedes:** [0017](0017-minimal-public-facade-and-cli.md), AVIF-only result and feature gating; [0024](0024-pluggable-encoders-and-libavif-rs-backend.md), backend feature coupling and result naming
- **Related:** [0015](0015-typed-output-color-signals.md), [0017](0017-minimal-public-facade-and-cli.md), [0018](0018-optional-codec-and-cms-dependencies.md), [0024](0024-pluggable-encoders-and-libavif-rs-backend.md), [0026](0026-codecs-in-one-repository-and-workspace.md), [0029](0029-webp-still-input.md), [M1](../milestones/M1-complete-image-pipeline.md), issues [#16](https://github.com/sebastian-software/ferrisma/issues/16) and [#60](https://github.com/sebastian-software/ferrisma/issues/60)

## Context

The implemented pipeline only exposed an AVIF byte field and compiled `rav1e` even for an external encoder. The workspace WebP port now provides a safe still encoder, muxer, and decoder. It requires a complete input picture before analysis. Its lossy ARGB import allocates YUV420 planes; its other internal state is opaque. WebP has no native CICP signal, so the output's ICC must describe the final sRGB samples rather than copy an input profile.

## Decision

Keep the existing ordered, straight-alpha sRGB8 `RowEncoder` boundary. `pipeline` and `EncoderBackend` compile without `avif-write`. `Output` carries `format` and `bytes`; `EncoderBackend::format` and `EncodedImage::format` must agree with the requested format. `EncodeRequest` carries the format and typed WebP settings to external backends too. Built-in output chooses AVIF or WebP explicitly. AVIF retains its existing quality, speed, chroma, and threading behavior. WebP uses separate `WebpOptions`: lossy quality/speed or lossless level, which has no quality setting. WebP rejects explicit AVIF chroma and multiple encoder threads.

The WebP sink fills one budgeted ARGB frame, then reserves the predictable YUV420(A) plane size before lossy encoding. It sets `exact(true)` to preserve the filtered RGB under zero alpha. The codec may still allocate opaque scratch. The bounded writer stops before the configured encoded-output limit; AVIF and external backends are checked after they return because their APIs materialize their result. The sink wraps the image with the committed sRGB2014 ICC profile through the port's muxer. It does not copy source ICC, EXIF orientation, or XMP. The final encoded result is a caller-owned buffer outside the managed pixel-memory budget; the report names the known encoder frame materialization and flags opaque memory.

This decision does not implement animation, metadata passthrough, WebP SharpYUV, or direct YUV-to-AOM. Issue #60's measured choice on direct AOM encoding remains open.

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

## Amendment log

- 2026-11-11 — Initial experimental still WebP output and encoder-result decision.
