# Current project status

**Reviewed: 2026-11-11.** Ferrixel is an experimental, unpublished image engine
and codec workspace. M1 is implemented in part and under validation, not a
release candidate. This page owns the current cross-component overview;
the linked guides and evidence records own the details.

The repository contains both the pipeline a caller can exercise today and the
codec ports intended for its future backends. A port's coverage, its Rust API,
its integration into the engine, and release acceptance are separate questions.

## What works through the engine

The optional Rust facade and CLI run this supported SDR path:

```text
JPEG / WebP / JPEG XL / AVIF
        ↓
decode and interpret rendered color
        ↓
apply orientation → crop → resize in linear light
        ↓
quantize → encode AVIF or still WebP
```

| Format | Current engine path | Workspace port relationship |
| --- | --- | --- |
| JPEG | Input through workspace `libjpeg-turbo-rs` (`jpeg-read`); codec-level crop request and budgeted RGB row | Scaled IDCT remains unused by the adapter; codec memory and reconstruction are opaque |
| JPEG XL | Default input through `jxl-oxide` (`jpeg-xl-read`); explicit opt-in local `libjxl-rs` via `ferrixel-jxl-decoder` (`jpeg-xl-own-read`) | `JpegXlDecoder::LibjxlRs` or `--jxl-decoder=libjxl-rs` selects the local decoder; it has no ROI request and budgets a full F32 frame. Complete external conformance and selection gates remain open |
| AVIF | Workspace libavif/AOM input (`avif-read`); Rav1e output by default (`avif-write`), explicitly selectable workspace libavif/AOM output (`avif-own-write`) | Independent read/write wrappers isolate the unsafe ABI; the facade and CLI select the own writer through `AvifEncoder::Libavif` / `--avif-encoder=libavif`. Native/nightly requirements and backend-selection gates remain ([ADR-0037](adr/0037-selectable-workspace-avif-encoder.md)) |
| WebP | Still input and output through workspace `libwebp-rs` (`webp-read`, `webp-write`); lossy crop requests, lossless full-frame internal work | The output sink budgets an ARGB frame and predictable lossy planes, tags sRGB ICC, and reports opaque codec memory; animation is unsupported |

The `cli` feature enables the current engine adapters and ICC path together.
The authoritative feature selection is in [Cargo.toml](../Cargo.toml); the
[public facade](../src/pipeline.rs) and [CLI](../src/cli.rs) expose the supported
operations. JPEG output and JPEG XL output are not implemented engine features.

Current behavior and boundaries:

- **Color and geometry:** SDR sRGB/Display P3 reference math, Area and Lanczos3
  resizing, and opt-in RGB ICC conversion through `moxcms`. Input color must be
  explicit. HDR, contradictory metadata, and unsupported alpha cases fail
  closed. The JPEG input adapter emits RGB8 and rejects CMYK/YCCK files;
  supported JPEG, WebP, JPEG XL, and AVIF orientation is applied before crop;
  AVIF `clap` and unsupported transform orderings are rejected.
- **Execution:** the default `f32` executor is checked against an `f64`
  reference. Runtime CPU dispatch, a parallel row pipeline, and batch execution
  exist. Per-image thread requests above 256 fail before decoding; batch worker
  counts are capped at 256. OS worker-start failure returns a resource error
  after joining partially started workers ([ADR-0022](adr/0022-parallel-row-pipeline-and-engine.md)).
  GPU execution and a general multi-output DAG remain design work.
- **Thumbnail facade:** `transcode_resized` resolves one or two dimensions,
  inside/centered-cover/fill geometry, and explicit no-enlargement behavior
  after orientation and an optional upright crop. Existing exact-size calls
  retain their pixel behavior. Results report actual dimensions; the public
  `Output` type gains fields in this unpublished experimental API. JPEG/WebP
  header inspection has byte/pixel limits and does not validate complete pixel
  data or assume a color profile ([ADR-0034](adr/0034-thumbnail-geometry.md)).
- **Memory:** engine-managed frame, row, and scratch allocations are budgeted
  and reported. Working-linear resize also reserves a conservative tap-table
  and scheduling allowance before footprint construction; reported peaks now
  include it ([ADR-0004](adr/0004-buffer-ownership-and-budget.md)). Codec
  allocations, graph nodes, allocator/runtime internals, and worker stacks
  remain outside that budget. JPEG and lossless WebP codec internals, AVIF decoding, and AVIF
  encoding may require full frames; there is no process-wide memory-cap guarantee.
  External encoders can declare known simultaneous pixel-frame bytes before
  allocation; the engine holds that reservation through encoder completion.
  The libavif backend declares its RGBA and YUV(A) planes, while codec scratch,
  allocator overhead and encoded bytes remain outside that reservation.
  The AVIF reader charges its full RGB output to the engine budget but retains
  opaque libavif/AOM scratch. On one measured photo it took 8–14% longer and
  about 2× peak RSS versus the former interim decoder; this does not rank
  decoders on other workloads ([evidence](benchmarks/M2-avif-input.md)).
- **Planar experiments:** borrowed YUV420P10 supports even-origin crop-only
  views. It is not a general high-bit-depth pipeline, and has no planar resize
  or HDR tone-mapping policy.

See [architecture](architecture.md), [M1 evidence and gates](milestones/M1-evidence-and-gates.md),
and [the color contract](design/color-management-contract.md) for the tested
contracts and their limits.

## Components in the workspace

There are **15 Cargo packages**. The four packages in the AVIF row below are
separate workspace members; they are grouped here by their shared port.

| Component | Implemented role | Remaining boundary or adoption gate |
| --- | --- | --- |
| [`ferrixel`](../Cargo.toml) | Feature-gated facade/CLI and the SDR pipeline above | M1 quality, resource, security, license, and release acceptance |
| [`ferrixel-kernels`](../crates/ferrixel-kernels) | Safe API over scalar, NEON, AVX2, and AVX-512 resampling; runtime selection | Host-specific evidence still applies; no SVE2 kernel or GPU backend |
| [`libsharpyuv-core`](../crates/libsharpyuv-core) | One safe SharpYUV numeric conversion shared by WebP and AVIF | Separate adapters preserve WebP slices/SIMD and AVIF C ABI/allocation behavior; measured AVIF kernel cost and native oracle gates are recorded in [ADR-0033](adr/0033-shared-sharpyuv-conversion-core.md) |
| [`ferrixel-libavif`](../crates/ferrixel-libavif/README.md) | Compatibility adapter and CLI re-exporting the engine's libavif backend | Delegates to the independent writer; native/nightly and codec acceptance gates remain |
| [`ferrixel-avif-read`](../crates/ferrixel-avif-read) | Safe, independent read wrapper over libavif's AOM decoder; selected by `avif-read` | Pinned nightly and supported macOS ARM64/GNU Linux x86-64; full Argon/dav1d adoption evidence remains open |
| [`ferrixel-avif-write`](../crates/ferrixel-avif-write) | Independent safe row writer over the workspace AOM encoder; selected by `avif-own-write` | Isolates unsafe ABI calls; known RGBA/YUV(A) bytes are reserved, codec scratch and encoded output remain opaque |
| [`ferrixel-jxl-decoder`](../crates/ferrixel-jxl-decoder) | Safe, independent wrapper for the opt-in local JPEG XL decoder | Full-frame fallback and measured photo regression keep `jxl-oxide` as the default |
| [`libjpeg-turbo-rs`](../codecs/libjpeg-turbo-rs/README.md) | 8-bit decoder, scaled decode, crop/skip, metadata, safe scanlines, and SIMD; selected for engine JPEG input | Scaled-IDCT quality policy and deeper cross-platform adoption evidence remain open; encoding, lossless JPEG, and 12/16-bit samples are outside the port's scope |
| [`libwebp-rs`](../codecs/libwebp-rs/README.md) | Safe decode/encode APIs, incremental decode, containers/animation, and SharpYUV | Still-image input and output adopted through `webp-read` and `webp-write`; NEON rescaler export kernels retain the portable Rust translation |
| [`libjxl-rs`](../codecs/libjxl-rs/docs/PORT_STATUS.md) | Incremental decoder, C-ABI surface, and reference comparison suites | Opt-in engine integration exists; public APIs are provisional, encoder incomplete, and deeper acceptance remains open |
| [`libavif-rs`](../codecs/libavif-rs/README.md), `libavif-aom`, `libavif-dav1d`, `libavif-sharpyuv` | AVIF container and codec ports, using generated and owned Rust with C-ABI boundaries | No stable safe Rust API claim; validated native hosts are macOS ARM64 and GNU Linux x86-64; package-specific deep gates remain |
| [`ferrixel-xtask`](../tools/xtask) | Standard-library Rust control plane, invoked as `cargo xtask`, for affected components and validation stages | Existing Python oracle harnesses migrate incrementally without losing evidence |

The engine forbids unsafe code. The kernel, adapter, and codec boundaries have
their own policies; that engine rule does not describe the entire dependency
graph. Package versions, licenses, and eventual publication remain independent.
All four original codec repositories are retired; work and issue tracking now
live here. [Codec development](codecs.md) owns setup, package selection,
provenance, toolchains, and release boundaries.

## How validation is organized

| Tier | Purpose | What it does not establish |
| --- | --- | --- |
| Scoped PR checks | Affected-component Rust tests, retained reference fixtures, provenance, linting, and MSRV checks | A fresh comparison with every C/C++ oracle on every change |
| Nightly / selected deep branch runs | Live oracle comparisons, sanitizers, profiles, and native architecture checks where supported | Completion of manual exhaustive suites or the M1 release gates |
| Manual exhaustive checks | AOM original-consumer, recovery, translation, and other specifically selected acceptance work | A blanket guarantee for untested platforms or feature combinations |

Required live oracles fail their jobs if unavailable. Changes to oracle pins,
goldens, unsafe ABI, SIMD, or floating-point behavior require the relevant deep
evidence before acceptance. No expected output is refreshed merely to turn a
failure green. [ADR-0027](adr/0027-tiered-validation-and-rust-test-orchestration.md)
owns this policy; [GitHub Actions](https://github.com/sebastian-software/ferrixel/actions)
shows current run results. A green PR check is not full codec certification.

## Next integration and release gates

- Complete the independent image-quality, hardening, resource, and dependency
  evidence in the [M1 gate register](milestones/M1-evidence-and-gates.md).
- Adopt workspace codecs through real end-to-end cases under
  [ADR-0025](adr/0025-codec-selection-reference-conformance-and-image-scope.md),
  preserving rendered color, orientation, buffer ownership, and conformance.
  Workspace membership alone does not satisfy those gates.
- Validate the extracted shared SharpYUV core with native Linux and macOS
  API, ABI, allocation, callback, and full original-consumer oracles. Preserve
  the independent C reference and review the measured AVIF kernel cost in
  [ADR-0033](adr/0033-shared-sharpyuv-conversion-core.md) and
  [issue #75](https://github.com/sebastian-software/ferrixel/issues/75).
- Continue verified Rust/TypeScript tooling consolidation. The shared runner
  is Rust already; retained Python harnesses are compatibility assets, not a
  new orchestration layer.

The [roadmap](roadmap.md) connects these gates to the milestone plans.
Historical benchmark results have their own commits, corpora, hardware, and
quality settings; they are not current universal speed claims. Update this
overview when integration or acceptance changes, with a link to the owning
evidence rather than a percentage-complete estimate.
