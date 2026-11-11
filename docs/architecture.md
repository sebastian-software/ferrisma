# Architecture sketch

*Status: exploratory. This page summarizes design intent and the current implementation, not a stable API contract. Start with the [documentation guide](README.md) and [current status](status.md).*

## Core separation

```text
                       Declarative image graph
            source ─── transform ─── transform ─── sink
                                   |
                                   v
                      Capability-aware planner
             format  ·  color  ·  geometry  ·  cost
                                   |
                                   v
                       Executable region plan
               task DAG  ·  ownership  ·  memory budget
                          /                 \
                         v                   v
                CPU execution           Future GPU execution
               scalar / SIMD          device tasks / textures
                          \                 /
                           v               v
                            Output consumer
```

The architecture has four conceptual layers:

1. **Graph:** immutable semantic image operations, independent of particular hardware and codec implementations. Nodes describe *what image they produce* rather than a buffer they own.
2. **Planner:** combines requested outputs with operation dependencies, codec capabilities, color semantics, and resource constraints. Produces an explainable execution plan; an optimization is valid only if it respects observable semantics.
3. **Executor:** executes work on regions/scanlines or device workloads under explicit lifetime, concurrency, and memory constraints. The CPU backend comes first; GPU execution is conditional on end-to-end benefit.
4. **Adapters:** decoders, encoders, sources, output sinks and device interfaces. Adapters publish real capabilities. The format specification alone is not proof that an implementation supports a fast path.

## Demand-driven example

As a design example, consider cropping an AVIF image, resizing it, and encoding
a JPEG XL thumbnail. JPEG XL output is not implemented in the current pipeline;
the example illustrates dependency propagation:

```text
required output region
  <- resampling source footprint, plus filter halo
  <- crop coordinate mapping (respecting orientation semantics)
  <- decoder region, rounded to decoder's supported granularity
  <- encoded source
```

The engine must distinguish a decoder that can *avoid reconstructing* irrelevant regions from a decoder that only hides them after full reconstruction. A plan may need full-frame decode for some images. This is an explicit cost, not an invisible violation of the contract.

## A first-class pixel contract

A surface needs more than width and height:

- dimensions and coordinate space, pixel/plane strides and alignment;
- planar or interleaved channel layout and sample representation;
- bit depth and sample range, including float where required;
- color primaries, transfer characteristics, color matrix, chroma subsampling/siting;
- alpha mode (straight/premultiplied) and orientation/metadata semantics;
- residency (CPU / device), ownership, lifetime and synchronization requirements.

These are semantic requirements, not an invitation to implement every possible layout in the first version. Add representations as a real codec-to-operation path requires them.

## Memory and scheduling

- **No full-frame intermediate by default.** Region views and borrowed input should be normal; explicit materialization is permitted when unavoidable or measurably beneficial.
- Use a **memory budget** for engine-managed allocations, including concurrent in-flight tasks. Estimate or constrain opaque decoder/encoder memory separately; never claim total-process bounds that cannot be enforced.
- Working-linear resize reserves a conservative allowance for tap construction and execution tables before planning. The allowance stays live through execution and thread joins; pixel buffers are charged separately. Graph nodes, allocator/runtime internals and worker stacks remain outside this model ([ADR-0004](adr/0004-buffer-ownership-and-budget.md)).
- Represent filter **halos**, alignment and access patterns in dependencies; no single physical tile size is mandated.
- Make cache and intermediate reuse evidence-driven. A region cache is helpful for overlapping consumers, but can hurt one-shot streaming workflows.
- CPU/GPU transfers are explicit plan edges with costs. Avoid backend switches that erase gains through copying and synchronization.

## Open questions

1. What is the minimum *semantic* node contract that supports region dependencies without committing to a giant public DAG API?
2. How do we represent decoder capability granularity and quality (exact ROI versus preview versus full-resolution)?
3. How do we preserve color correctness while deciding between planar native fast paths and RGB/linear-light paths?
4. Can executor-owned buffers be borrowed safely across asynchronous tasks without overconstraining scheduling?
5. When does the additional complexity of a GPU path pay for itself over a well-planned CPU path?

These questions are tracked in [the ADRs](adr/README.md), not decided by this sketch.

## First executable experiment (not a stable contract)

The internal Rust implementation in `src/` tests the four layers using a **single-chain Gray8** graph with synthetic random-access source, crop, scalar bilinear resize, and a borrowed scanline sink:

```text
caller's requested output rectangle
    → schedule one output row
    → backward region footprint through resize and crop
    → synthetic source generates only that footprint
    → crop rebases the region without copying
    → resize allocates just its required output row
    → sink borrows that row; both input/output reservations are released
    → repeat with the next output row
```

**Why one row?** With extreme downsampling, two consecutive target rows can sample source rows millions of pixels apart. Taking their union as one source rectangle materializes unnecessary intervening scanlines. Planning each requested row avoids that specific trap; future planners may merge adjacent rows only when doing so is worthwhile.

This experiment has no codec integration, advanced color model, multi-input branching, parallel worker scheduler, or GPU support. The budget currently tracks explicitly allocated **pixel bytes only**, not metadata, codec internals, or sink-owned buffers. This limitation must remain visible when comparing real workloads. See [the experiment's contracts](design/region-experiment.md) and ADRs [0002](adr/0002-semantic-graph-and-planner.md), [0003](adr/0003-demand-driven-regions.md), and [0004](adr/0004-buffer-ownership-and-budget.md).


## Borrowed planar source experiment (2026-10-31)

The [second executable experiment](design/planar-surface-experiment.md) introduces a `Planar10Source` node with format and CICP-like color metadata but **no owned buffer**. Execution binds borrowed byte-strided `Yuv420P10<'a>` planes to that node, then forwards crop-only requests through the existing region planner. This directly tests the graph/storage lifetime boundary. The graph is not lifetime-parametrized by decoder buffers.

Only even-origin YUV 4:2:0 crops are representable as direct views: shifting the chroma phase requires a real resampling policy. Planar resize is intentionally rejected rather than using the existing Gray8 bilinear filter on subsampled/HDR data. All source memory remains caller-owned, and the executor reports no managed *pixel* allocations. This does not assert zero process allocations, universal format support, or a stable API; the experiment itself does not validate an actual decoder.

## Decoder source seam (experimental)

Read-only JPEG, WebP, JPEG XL and AVIF codecs are optional source adapters, bound to graph nodes only at execution. Their internal ownership remains outside graph semantics; a source reports either a known full-frame decode fallback or a codec-level ROI request. ROI requests do not guarantee avoided reconstruction. See [ADR-0012](adr/0012-codec-source-contract.md). The managed-memory budget does not cover codec-internal allocations. A full color-profile graph contract is still open.

## Rendered color descriptor and first codec resize (experimental)

Graph nodes now share exact rendered ICC and CICP metadata and alpha-association status, independent of codec-owned pixel buffers. Absent color metadata is explicitly **Unspecified**, never implicitly sRGB. The first decoder-backed resize supports only Gray/RGB U8 or F32 with no alpha, and must be explicitly requested as **encoded-sample-domain bilinear**. It is not a linear-light, color-managed, HDR-tonemapping or alpha-premultiplied resize. A single decoder request streams rows using one budgeted previous scanline and one output scanline. The JPEG adapter asks the codec for an iMCU-expanded horizontal crop with one chroma halo column on each available side, skips leading rows, and finishes the remaining stream for error validation; it does not claim that entropy or coefficient work was avoided. The planner's single bounding ROI may overfetch on large downscales. See [ADR-0013](adr/0013-sample-domain-resize.md) and [the experiment](design/decoded-resize-experiment.md).

## Strict linear-light sRGB reference (experimental)

[ADR-0014](adr/0014-linear-srgb-straight-alpha.md) and [the reference experiment](design/color-correctness-experiment.md) add an explicitly tagged `LinearSrgb` resize domain to the existing semantic graph. It only accepts RGB8 opaque and RGBA8 straight-alpha samples with sole, unambiguous full-range RGB/sRGB CICP. RGB is transfer-decoded before interpolation; straight alpha is premultiplied in linear light and unpremultiplied for output. ICC/HDR/unknown alpha is deliberately rejected until a real color-management contract exists. This does not change codec ROI semantics, decoder-owned memory, or the CPU-first execution model.

## Typed rendered-color contract (experimental)

[ADR-0015](adr/0015-typed-output-color-signals.md) and [the color contract](design/color-management-contract.md) establish a no-pixel-allocation inspection stage for **decoder output evidence**: typed H.273/CICP code points, bounded ICC header and tag-table inspection, and explicit sRGB/P3/PQ/HLG or unresolved-profile classifications. The planner distinguishes a supported sRGB transfer reference from work that would require gamut conversion, HDR reference-white/tone-mapping policy, an ICC CMM, or profile precedence. ICC plus CICP are not silently merged; a valid header is not a validated ICC color transform. This does not add new codec support or a full ICC engine.

## M1 pipeline, facade and dependencies (experimental)

The current experimental M1 path runs `decoder source → crop → ToLinearSrgb → WorkingLinear(Area | Lanczos3) → ToSrgb8 → AVIF or WebP sink` through a bounded row-cache executor ([ADR-0016](adr/0016-working-color-space-and-transform-boundaries.md), [ADR-0030](adr/0030-still-webp-output-and-format-neutral-encoder-result.md)). Two executors implement it: the scalar `f64` reference (`quality_execute`) and the default `f32` executor (`fast_execute`), with runtime-dispatched CPU kernels, which linearizes each sample once through exact tables and keeps filtered rows in an O(1) ring; a differential test holds the second to the first within [ADR-0021](adr/0021-working-precision-policy.md)'s bound. The AVIF and WebP sinks explicitly materialize output frames and report them. One public entry point, `pipeline::transcode`, and a `cli`-gated binary exercise this path; graph, planner and executor stay private ([ADR-0017](adr/0017-minimal-public-facade-and-cli.md)).

Codecs and the ICC CMS are optional features: workspace `libjpeg-turbo-rs` and `libwebp-rs`, default JPEG XL reader `jxl-oxide`, workspace libavif/AOM through the independent safe `ferrixel-avif-read` input wrapper, `rav1e` with `avif-serialize` for default AVIF output, and `moxcms` for rendered-ICC conversion. The local `libjxl-rs` decoder is an explicit opt-in input backend through the independent `ferrixel-jxl-decoder` wrapper. The ICC source adapter emits linear-light sRGB and retains signed, out-of-sRGB-gamut samples for supported RGB matrix-shaper profiles until final output quantization; the LUT path retains its prior behavior ([ADR-0023](adr/0023-decode-side-work-and-scaling-policy.md)). Sources describe their rendered color before decoding. For AVIF, Ferrixel's bounded parser reads primary-item `colr` and orientation properties, while a small same-module libavif seam reads AV1 sequence headers from the selected color samples before pixel decode; contradictory signals fail closed ([ADR-0031](adr/0031-workspace-libavif-input-experiment.md)). The core's only unconditional dependency is the first-party kernel crate `crates/ferrixel-kernels` (ADR-0020): `std::arch` SIMD kernels with runtime dispatch behind a safe API, so the engine keeps `#![forbid(unsafe_code)]`. The codec-free core and features without `avif-read`/`cli` retain stable Rust 1.98 support; `avif-read` requires the pinned nightly on native macOS ARM64 or GNU Linux x86-64. The opt-in JPEG XL decoder reconstructs the full frame for every crop ([ADR-0032](adr/0032-jpeg-xl-workspace-decoder-input.md)); complete external conformance and selection gates remain open. Still WebP input and output use the workspace port through [ADR-0029](adr/0029-webp-still-input.md) and [ADR-0030](adr/0030-still-webp-output-and-format-neutral-encoder-result.md). JPEG stays input-only.

WebP and AVIF use the small safe `libsharpyuv-core` conversion crate through distinct package adapters ([ADR-0033](adr/0033-shared-sharpyuv-conversion-core.md)); the AVIF C ABI retains its isolated unsafe boundary.

## Orientation (experimental)

JPEG and WebP sources report EXIF orientation; AVIF reports its `irot`/`imir`
item transforms. A non-identity source is wrapped in `OrientedSource`, which
maps each display rectangle back to its stored rectangle, reads it once into a
budgeted buffer and emits upright rows. The graph therefore stays
orientation-free and crops are display coordinates; the buffer is reported as
`orientation_buffer_bytes`. Both JPEG XL input backends apply orientation by
default and return upright dimensions and samples, verified against the eight
independent fixtures. See [ADR-0019](adr/0019-source-side-orientation.md),
[ADR-0031](adr/0031-workspace-libavif-input-experiment.md) and
[ADR-0032](adr/0032-jpeg-xl-workspace-decoder-input.md).

## Parallel execution (experimental)

[ADR-0022](adr/0022-parallel-row-pipeline-and-engine.md): with `transform_threads > 1` the f32 executor runs a row pipeline. The thread that drives the decoder copies each needed source row and submits it to workers, which linearize it and run the horizontal kernel; output rows whose horizontal inputs all exist are submitted for the vertical pass and quantization; a reorder buffer hands finished rows to the sink in order. A bounded job queue applies backpressure to the decoder, horizontal rows are released by the same rule as the sequential ring, and every buffer is charged to the shared budget. The output is byte-identical to the sequential path. `Engine::transcode_batch` schedules whole images across scoped workers instead, each with one transform and one encoder thread.

## Repository and verification boundaries

[ADR-0026](adr/0026-codecs-in-one-repository-and-workspace.md) places the four
codec ports under `codecs/` in the engine's Cargo workspace. The AVIF input
wrapper and optional encoder adapter both use the local libavif package; the
default engine AVIF encoder remains rav1e. The engine, adapters, codec APIs,
licenses and unsafe boundaries remain distinct; membership alone does not
select a pipeline backend. The root owns the lockfile, development toolchain, profiles and active
CI workflows. Component checks retain their independent C/C++ references,
upstream pins and comparison rules, with a common entry point in
`cargo xtask codecs`. See [codec development](codecs.md) for supported
feature combinations, reference setup, and independent publication.

[ADR-0027](adr/0027-tiered-validation-and-rust-test-orchestration.md) defines scoped PR validation and deeper scheduled and manual reference checks. Rust owns the shared orchestration; existing Python oracle harnesses remain until evidence-preserving ports replace them.

## Documentation delivery

The Ardo website in `homepage/` presents the engine and ports through the shared
Ferramenta theme. Technical pages are generated from this repository's Markdown;
the website has a separate Node/pnpm dependency graph and build, outside Cargo.
[ADR-0028](adr/0028-ardo-documentation-site.md) records the content ownership and
presentation boundary. It adds no engine API or codec capability.

## Thumbnail requests and encoder frame reservations (2026-11-11)

[ADR-0034](adr/0034-thumbnail-geometry.md) adds a narrow thumbnail request to
that same pipeline: orient the source, apply an explicit upright crop, resolve
inside/centered-cover/fill geometry with integer rounding, then construct the
existing private graph. One supplied dimension preserves aspect ratio;
no-enlargement behavior is explicit, and the result reports actual dimensions.
The old exact-size calls retain their pixel behavior. This unpublished API adds
fields to `Output`; it does not promise exhaustive-struct source compatibility.
JPEG/WebP header inspection reads emitted-sample evidence without requesting
pixels, applying profile assumptions or establishing full-image validity.

The external encoder seam also accepts a known pixel-frame requirement before
`begin`. The engine reserves those bytes before backend allocation and keeps
the reservation through `finish` or partial-sink destruction. The libavif
adapter declares concurrent RGBA and tightly packed YUV(A) planes, including
rounded-up chroma dimensions. This is conservative reservation accounting;
allocator overhead, conversion scratch, encoded output and AOM internals remain
opaque. Backends that do not declare frames retain a zero requirement. See the
amendment to [ADR-0024](adr/0024-pluggable-encoders-and-libavif-rs-backend.md).
