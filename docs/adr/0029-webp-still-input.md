# ADR-0029: Read still WebP through the workspace reference port

- **Status:** Experimental
- **Date:** 2026-11-11
- **Last reviewed:** 2026-11-12
- **Supersedes:** The unimplemented `webp-read` target in [0018](0018-optional-codec-and-cms-dependencies.md); no prior implemented behavior
- **Related:** [0012](0012-codec-source-contract.md), [0015](0015-typed-output-color-signals.md), [0018](0018-optional-codec-and-cms-dependencies.md), [0019](0019-source-side-orientation.md), [0025](0025-codec-selection-reference-conformance-and-image-scope.md), [0026](0026-codecs-in-one-repository-and-workspace.md), [WebP port plan](../research/2026-11-05-libwebp-port-plan.md)

## Context

Ferrisma has no WebP input. The workspace now contains `libwebp-rs`, a BSD-3-Clause port of the libwebp reference implementation. Its safe API exposes image features, a still-image crop decoder, and container ICC and EXIF. The prior plan required a pinnable external revision (issue #86); ADR-0026's committed monorepo path and compatible package version now provide the same reproducible dependency identity through the shared lockfile. The old external-tag gate is therefore obsolete for this workspace adoption, while independent publication remains separate.

## Decision

Add optional `webp-read` and enable it in the CLI. `ImageFormat::WebP` binds a private still-image source. `Features::parse` checks canvas size and animation before container parsing or output allocation. Animated WebP is explicitly unsupported. A complete still image must expose a lossy or lossless bitstream format; `format: None` alone never identifies animation.

The source emits unsigned 8-bit RGB or straight-alpha RGBA using the port's non-premultiplied modes. Its ICC, if present, describes those emitted RGB samples; no CICP is invented from the coded YUV matrix. Without ICC, the output color is unspecified and the existing explicit `--assume-srgb` policy applies. EXIF orientation is reported to the source-side wrapper, so public crops use upright coordinates. Malformed EXIF and unsupported orientation values fail closed.

The adapter requests a decoder crop into an engine-budgeted buffer. Lossless crops expand odd origins to the preceding even coordinate, then slice away the extra row or column. For lossy RGB, a differential test found that cropping at the requested top or horizontal edges changes rendered pixels relative to a full decode. The adapter therefore requests full width from the top through the requested bottom plus up to eight filter-context rows, then slices the requested region. `Config::output_size` validates the expanded request before allocation. The source lends each requested row to the sink; cancellation stops subsequent delivery. Lossy input reports `CodecRegionRequest` only because it can avoid rows below that expanded bottom; it saves no top or horizontal work. Lossless input reports `FullFrameFallback` because its internal decoder builds a full ARGB image. A caller-owned buffer equal to the entire frame is counted as a known full-frame buffer; both paths report unbounded opaque codec memory. The budget covers the decoded output buffer, including expansion, but not libwebp scratch or its lossless full-frame allocation.

## Consequences

- Still lossy and lossless WebP can feed the existing AVIF pipeline without changing its output contract.
- Alpha is described accurately; the current facade's alpha compatibility gates still decide which transforms can consume it.
- A crop bounds the caller-owned sample buffer, but does not imply whole-process memory bounds or general region decode savings.
- The engine remains `unsafe`-free; the port's implementation has its own audited boundary and validation tiers.

## Alternatives

- `image-webp`: an additional interim implementation without this port's documented crop behavior; it would duplicate maintenance.
- A universal full-frame RGBA buffer: simpler but forces alpha and hides avoidable lossy output allocation.
- An incremental decoder for in-memory input: useful for streamed compressed input, but this facade already owns the whole compressed slice and `decode_into` supplies the crop and budget boundary.
- WebP output or animation in this change: each needs separate output semantics, budget and validation decisions.

## Validation

Focused adapter tests compare odd and interior crops with a full decode for lossy RGB and lossless RGB/RGBA, and compare a retained lossy fixture exactly with C libwebp 1.6.0 `dwebp` RGB output; cover managed-buffer budget exhaustion and release, cancellation, malformed input, animation, canvas pixel limits, ICC, and EXIF orientation. A WebP-to-AVIF roundtrip through the independent AVIF reader confirms straight alpha survives with a gradient. The feature-only build, default build, CLI combination, relevant engine tests, formatting, and strict Clippy are the local gate. The port's existing scoped codec-oracle and provenance gates remain its own validation and are not replaced by adapter tests. This adoption makes no new speed or peak-RSS claim.

## Review when

Review if the port changes crop memory or row-work behavior, its color conversion or metadata semantics change, alpha formats or association rules change, animation enters scope, or measured crop cost justifies incremental compressed-input streaming.

## Amendment log

- 2026-11-11 — Adopted still-image input through the safe, local `libwebp-rs` API under the existing private source contract; retained explicit color, orientation, budget and opaque-memory gates.
- 2026-11-12 — Recorded the measured row work of lossy and lossless crops (issue #19).


## 2026-11-12: measured crop row work

Both WebP bitstreams decode from the first row at full width: VP8 needs intra
prediction and loop-filter context, VP8L its predictors and backward
references. The adapter's reports now state this (ADR-0012). For lossy input it
asks the codec for full-width rows from the top to eight rows below the
request; that whole buffer is engine-managed. For lossless input it asks for an
even-aligned crop, but VP8L still reconstructs every row above it internally at
full width, outside the budget.

Measured counts: a 30×40 display crop of a 96×72 orientation-5 lossy file
(stored request 40×30 at (21, 11)) produced a 96×49 codec output, 3,504 pixels
of overfetch; the lossless file produced a 41×31 output (71 pixels of
overfetch) and at least 96 × 41 reconstructed pixels. On a generated 6000×4000
lossy file with orientation 6, a 2000×1500 display crop requested 1500×2000
stored pixels at (1000, 1000) and the codec wrote 6000×3008 pixels, 15,048,000
pixels of overfetch held as 54,144,000 managed bytes. A full read reports one
known full frame. The previous text says only that crop output is budgeted; it
did not say how large the lossy crop buffer becomes for a low crop. A
row-callback decode that keeps only the requested rows would remove most of that
buffer; it is not implemented.
