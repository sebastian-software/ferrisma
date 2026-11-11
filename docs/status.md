# Current project status

**Reviewed: 2026-11-10.** Ferrixel is an experimental, unpublished image engine
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
quantize → encode AVIF
```

| Format | Current engine path | Workspace port relationship |
| --- | --- | --- |
| JPEG | Input through `zune-jpeg` (`jpeg-read`); full-frame decode | `libjpeg-turbo-rs` is a candidate replacement, not yet connected |
| JPEG XL | Input through `jxl-oxide` (`jpeg-xl-read`); codec-level ROI requests | `libjxl-rs` is not yet connected; ROI requests do not prove avoided internal work |
| AVIF | Input through `avif-decode` (`avif-read`); output through `rav1e` (`avif-write`) | `ferrixel-libavif` provides an explicitly selected alternative encoder; it is not the default or the input backend |
| WebP | Still input through workspace `libwebp-rs` (`webp-read`); lossy crop requests, lossless full-frame internal work | No engine output or animation adapter; opaque decoder memory is unbounded |

The `cli` feature enables the current engine adapters and ICC path together.
The authoritative feature selection is in [Cargo.toml](../Cargo.toml); the
[public facade](../src/pipeline.rs) and [CLI](../src/cli.rs) expose the supported
operations. JPEG output and JPEG XL output are not implemented engine features.

Current behavior and boundaries:

- **Color and geometry:** SDR sRGB/Display P3 reference math, Area and Lanczos3
  resizing, and opt-in RGB ICC conversion through `moxcms`. Input color must be
  explicit. HDR, contradictory metadata, and unsupported alpha cases fail
  closed. Supported JPEG, WebP, JPEG XL, and AVIF orientation is applied before crop;
  AVIF `clap` and unsupported transform orderings are rejected.
- **Execution:** the default `f32` executor is checked against an `f64`
  reference. Runtime CPU dispatch, a parallel row pipeline, and batch execution
  exist. GPU execution and a general multi-output DAG remain design work.
- **Memory:** engine-managed frame, row, and scratch allocations are budgeted
  and reported. Codec allocations and some planner metadata are outside that
  budget. JPEG/AVIF decoding, lossless WebP decoder internals, and AVIF encoding may require full frames; there
  is no process-wide memory-cap guarantee.
- **Planar experiments:** borrowed YUV420P10 supports even-origin crop-only
  views. It is not a general high-bit-depth pipeline, and has no planar resize
  or HDR tone-mapping policy.

See [architecture](architecture.md), [M1 evidence and gates](milestones/M1-evidence-and-gates.md),
and [the color contract](design/color-management-contract.md) for the tested
contracts and their limits.

## Components in the workspace

There are **11 Cargo packages**. The four packages in the AVIF row below are
separate workspace members; they are grouped here by their shared port.

| Component | Implemented role | Remaining boundary or adoption gate |
| --- | --- | --- |
| [`ferrixel`](../Cargo.toml) | Feature-gated facade/CLI and the SDR pipeline above | M1 quality, resource, security, license, and release acceptance |
| [`ferrixel-kernels`](../crates/ferrixel-kernels) | Safe API over scalar, NEON, AVX2, and AVX-512 resampling; runtime selection | Host-specific evidence still applies; no SVE2 kernel or GPU backend |
| [`ferrixel-libavif`](../crates/ferrixel-libavif/README.md) | Explicit alternative AVIF encoder using the local libavif/AOM port | Nightly and an audited unsafe boundary; cannot link with the interim `avif-read` backend |
| [`libjpeg-turbo-rs`](../codecs/libjpeg-turbo-rs/README.md) | 8-bit decoder, scaled decode, crop/skip, metadata, safe scanlines, and SIMD | Connect and validate the engine adapter; encoding, lossless JPEG, and 12/16-bit samples are outside the port's scope |
| [`libwebp-rs`](../codecs/libwebp-rs/README.md) | Safe decode/encode APIs, incremental decode, containers/animation, and SharpYUV | Still-image input adopted through `webp-read`; output remains separate; NEON rescaler export kernels retain the portable Rust translation |
| [`libjxl-rs`](../codecs/libjxl-rs/docs/PORT_STATUS.md) | Incremental decoder, C-ABI surface, and reference comparison suites | Public APIs are provisional; encoder incomplete; engine integration and deeper acceptance remain open |
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
- Extract a shared SharpYUV core only after the recorded ABI, callback,
  allocation, parity, and measurement gates in
  [ADR-0026](adr/0026-codecs-in-one-repository-and-workspace.md#sharpyuv-consolidation-review-2026-11-10)
  and [issue #75](https://github.com/sebastian-software/ferrixel/issues/75) pass.
- Continue verified Rust/TypeScript tooling consolidation. The shared runner
  is Rust already; retained Python harnesses are compatibility assets, not a
  new orchestration layer.

The [roadmap](roadmap.md) connects these gates to the milestone plans.
Historical benchmark results have their own commits, corpora, hardware, and
quality settings; they are not current universal speed claims. Update this
overview when integration or acceptance changes, with a link to the owning
evidence rather than a percentage-complete estimate.
