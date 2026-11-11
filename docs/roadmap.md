# Implementation roadmap

For the current implementation and navigation, see [status](status.md) and the [documentation guide](README.md).

**Current acceptance milestone:** [M1 — First complete image pipeline](milestones/M1-complete-image-pipeline.md), with an experimental SDR JPEG/JPEG XL/AVIF → AVIF implementation under validation. [M2 — Performance architecture](milestones/M2-performance-architecture.md) remains a proposed milestone, although its `f32` executor, CPU dispatch, parallel execution, and initial decode improvements have landed. Broader benchmark and release evidence remains open. The milestone plans and their dated amendments own acceptance criteria; the earlier phases below remain research history.

[Encoder/CMS candidates and licensing](research/2026-10-31-encoder-and-cms-candidates.md) identify a permissively licensed AVIF path, a candidate ICC backend, and a JPEG XL encoder license gate. See [ADR-0016](adr/0016-working-color-space-and-transform-boundaries.md) (Proposed) for the working-color design contract.

*Sequence, not a deadline or a commitment to a fixed API.* Progress depends on tests and architectural evidence, not feature count.

## Historical phases and experiments

The dated descriptions below record what each experiment established at the time; later implementations do not erase those limitations or imply milestone acceptance.

### 0. The smallest correct pipeline — experimental implementation

- Typed geometry: rectangles, bounds, coordinate transforms, input footprints and filter halos.
- Minimal image graph with synthetic sources (no codecs required).
- A region-requesting consumer and a simple CPU executor.
- Safe view/borrow contracts and an observable allocation counter.
- Golden tests for crop and one reference resize algorithm; no throughput claims.
- A readable plan report showing actual work, materializations, peak engine-managed memory, and unsupported fast paths.

**Progress (2026-10-31):** The scalar Gray8 synthetic → crop → bilinear resize → borrowed scanline sink implementation validates region planning and bounded managed pixel buffers. A subsequent [borrowed planar source probe](design/planar-surface-experiment.md) integrates 10-bit YUV 4:2:0 planes with byte strides, explicit color metadata and even-origin zero-copy crop forwarding. Neither experiment validates real decoder APIs or phase-aware/HDR resizing. At that point, a real codec lifetime and output-format contract still needed validation.

**Exit criterion:** a synthetic pipeline requests only necessary upstream regions and passes strict semantic/ownership tests without allocating full-frame intermediate images.

### 1. Real format-informed vertical slice

- Evaluate **one** modern decoder and **one** output sink from JPEG XL, AVIF and WebP according to usable APIs, licensing, correctness and accessible partial-decode capabilities.
- Measure full decode versus limited decode where the implementation genuinely supports it.
- Validate pixel contracts, orientation, alpha and color handling.
- Connect bounded scheduling and backpressure to concrete codec behavior.

**Exit criterion:** a real input-to-output pipeline with trustworthy resource accounting and no unjustified copies; graceful fallback where codec-specific optimizations are unavailable.

### 2. Pipeline optimization by evidence

- Native-layout negotiation and conversion avoidance where quality is maintained.
- Correct, reproducible resampling benchmarks on several image classes and sizes.
- Limited SIMD variants only for proven hotspots with a reference implementation.
- Task scheduling and buffer reuse tuned with both latency and multi-image throughput.

**Exit criterion:** measured improvement without quality or memory regressions, with planner explanations for every fast path.

### 3. Conditional GPU experiments

- Model device residency, upload/readback cost and synchronization explicitly.
- Prototype one large, repeated operation (e.g., resampling plus color transform).
- Compare end-to-end time and resource use against CPU on representative hardware.
- Preserve the CPU-only configuration as the baseline.

**Exit criterion:** an independently measured workload where device execution wins enough to justify maintenance and portability costs.

## Not prioritized initially

Historical JPEG output tuning, compatibility with the complete libvips or Sharp surface, hundreds of image operations, generalized image editing, custom codecs, FFI-heavy dependencies, and a universal compute DSL.

## Historical decoder adapter experiment

Read-only JPEG and JPEG XL source adapters are introduced behind Cargo features. They validate cropped source-to-sink execution and managed vs. opaque memory boundaries; they do **not** complete phase 1's color, orientation, malformed-input and full encoder requirements. See [ADR-0012](adr/0012-codec-source-contract.md).

## Historical 2026-10-31 real decoder resize experiment

An explicitly selected sample-domain bilinear `JPEG/JPEG XL decoder → crop* → resize → crop* → borrowed row sink` path probed real codec output and end-to-end row lifetimes. Rendered ICC/CICP and alpha association are tracked through the graph and verified on binding. No color-managed filtering, alpha-aware compositing or HDR conversion is claimed. The experiment identified linear-light, premultiplied-alpha filtering and region overfetch as follow-up work. See [ADR-0013](adr/0013-sample-domain-resize.md).

## Historical color fidelity experiment (2026-10-31)

The [first opt-in linear-light sRGB path](design/color-correctness-experiment.md) introduces a correct transfer-function and straight-alpha bilinear reference for strictly tagged RGB8/RGBA8. That experiment did not provide ICC parsing, gamut conversion, HDR transfer processing or an encoder path. Its proposed next step was an explicit trusted **decoder output profile / color transform** contract, without treating untagged JPEG or JPEG XL float samples as sRGB. See [ADR-0014](adr/0014-linear-srgb-straight-alpha.md).

## Historical explicit color evidence (2026-10-31)

[ADR-0015](adr/0015-typed-output-color-signals.md) records the next experiment: classify **rendered** decoder color signals without changing pixels. The shared contract identifies sRGB, Display P3, BT.2100 PQ and HLG, preserves unsupported code points, inspects only ICC header/tag-table bounds, and refuses to guess precedence when ICC and CICP are both present. The linear-sRGB path delegates eligibility to that contract; the experiment itself did not provide other conversions. Before admitting ICC or HDR transforms, specify the CMM's supported profile classes, pixel and reference-white domains, output metadata, black-point/intent policy, memory accounting, licensing and differential color fixtures.

## Historical codec decisions (2026-11-05)

JPEG remains input-only through `zune-jpeg`; no JPEG encoder is planned, and `mozjpeg-rs` (BSD-3-Clause, pure Rust) is the recorded candidate should a consumer need one. The decision proposed WebP input and output through a mechanical libwebp-rs port, with no interim WebP dependency and the decoder as its first milestone. This resolves the WebP part of the phase 1 evaluation item. See the [JPEG and WebP evaluation](research/2026-11-05-jpeg-and-webp-codec-evaluation.md), the [libwebp port plan](research/2026-11-05-libwebp-port-plan.md), the [work plan for parallel sessions](research/2026-11-05-libwebp-port-workplan.md) and [ADR-0018](adr/0018-optional-codec-and-cms-dependencies.md).

The JPEG, WebP, JPEG XL and AVIF ports now reside in this workspace. Workspace membership does not select a port as a pipeline backend; see [codec development](codecs.md) and [ADR-0025](adr/0025-codec-selection-reference-conformance-and-image-scope.md) for adoption gates.

As of 2026-11-11, `jpeg-read` selects the workspace libjpeg-turbo-rs port;
the `zune-jpeg` references in the dated research above describe the historical
adapter and its measurements.
