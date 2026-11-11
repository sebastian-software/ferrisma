# Architecture Decision Records

Ferrixel keeps **living ADRs**: decisions are durable and reviewable, not immutable historical monuments. A decision can change when evidence changes, but its previous rationale and the reason for revision must remain discoverable.

For implemented capabilities and package status, use [current status](../status.md).
The [documentation guide](../README.md) also links the codec-local ADR indexes.

## Lifecycle

- **Proposed:** hypothesis/design under review, not binding implementation policy.
- **Experimental:** temporarily implemented or benchmarked to collect evidence.
- **Accepted:** current architectural policy; amendments require recorded rationale.
- **Superseded:** replaced by a linked ADR.
- **Rejected:** considered and declined, with an explanation.

Every ADR contains **Status**, **Date**, **Last reviewed**, **Context**, **Decision**, **Consequences**, **Alternatives**, **Validation**, **Review when**, and an **Amendment log**. Update this index whenever status or topic changes. Separate measured facts from hypotheses; avoid declaring architecture decisions from attractive diagrams alone. Use [TEMPLATE.md](TEMPLATE.md) for new ADRs.

## Current decisions

| ADR | Topic | Status |
| --- | --- | --- |
| [0001](0001-project-thesis.md) | Pipeline-first project principles and modern-format scope | Accepted |
| [0002](0002-semantic-graph-and-planner.md) | Semantic graph separated from capability-aware planner | Experimental |
| [0003](0003-demand-driven-regions.md) | Demand propagation, region footprints and fallback | Experimental |
| [0004](0004-buffer-ownership-and-budget.md) | Ownership, allocation and realistic memory budgets | Experimental |
| [0005](0005-codec-capabilities-and-pixel-contracts.md) | Modern codec adapters, color and layout contracts | Proposed |
| [0006](0006-cpu-first-gpu-capable-execution.md) | Execution scheduling, SIMD and conditional GPU | Proposed |
| [0007](0007-correctness-and-performance-evidence.md) | Correctness and benchmark evidence before optimization | Accepted |
| [0008](0008-initial-vertical-slice.md) | Minimal architecture-validating implementation | Experimental |
| [0009](0009-living-adr-process.md) | Decision lifecycle and amendment policy | Accepted |
| [0010](0010-dual-licensing.md) | Dual-license Ferrixel under MIT OR Apache-2.0 | Accepted |
| [0011](0011-borrowed-planar-surface-contract.md) | Borrowed, byte-strided YUV420P10 source contract | Experimental |
| [0012](0012-codec-source-contract.md) | Real decoder sources: explicit ROI capabilities and workspace JPEG scanlines | Experimental |
| [0013](0013-sample-domain-resize.md) | Preserve rendered metadata and validate sample-domain codec resize | Experimental |
| [0014](0014-linear-srgb-straight-alpha.md) | Explicit sRGB linear-light resize and premultiplied filtering | Experimental |
| [0015](0015-typed-output-color-signals.md) | Inspect CICP and ICC output evidence; describe required conversions | Experimental |
| [0016](0016-working-color-space-and-transform-boundaries.md) | Region-capable working-color transform and output semantics | Proposed |
| [0017](0017-minimal-public-facade-and-cli.md) | Minimal `pipeline::transcode` facade and feature-gated CLI; partially supersedes 0008 | Experimental |
| [0018](0018-optional-codec-and-cms-dependencies.md) | Interim codec/CMS dependencies and port adoption gates; JPEG input-only; MSRV 1.98; committed lockfile | Experimental |
| [0019](0019-source-side-orientation.md) | Apply EXIF/AVIF orientation through an explicit, budgeted source-side materialization | Experimental |
| [0020](0020-kernel-crate-and-isa-dispatch.md) | Separate `ferrixel-kernels` crate, `std::arch` intrinsics behind a safe API, runtime ISA dispatch reported in the plan, scalar/SIMD differential tests | Experimental |
| [0021](0021-working-precision-policy.md) | Working precision: `f32` with exact LUTs and a testable error bound against the `f64` oracle; the Q1.14 path was measured and dropped | Experimental |
| [0022](0022-parallel-row-pipeline-and-engine.md) | Parallel row pipeline with ordered emission (byte-identical to the sequential executor, standard library only) and an `Engine` for batches | Experimental |
| [0023](0023-decode-side-work-and-scaling-policy.md) | Full-resolution decode by default; decoder SIMD, extended-range linear ICC matrix-shaper output and cheaper orientation; shrink-on-load only as an explicit, measured quality policy | Experimental |
| [0024](0024-pluggable-encoders-and-libavif-rs-backend.md) | Pluggable row-level encoder backends; libavif-rs (Rust AOM) as an isolated adapter crate (workspace placement superseded by 0026), `rav1e` stays the default | Experimental |
| [0025](0025-codec-selection-reference-conformance-and-image-scope.md) | Codec selection: reference implementation or proven conformance, ports reduced to the still-image scope, deviations and removals only after measurement, mechanical oracle for later optimization; JPEG decode before encode | Accepted |
| [0026](0026-codecs-in-one-repository-and-workspace.md) | One repository and Cargo workspace for the engine, adapter and four codec ports; imported PRs, common verification entry point and early SharpYUV consolidation review | Accepted |
| [0027](0027-tiered-validation-and-rust-test-orchestration.md) | Scoped PR checks, deep validation cadence and Rust orchestration | Accepted |
| [0028](0028-ardo-documentation-site.md) | Ardo website with the shared Ferramenta theme and canonical repository Markdown | Accepted |
| [0029](0029-webp-still-input.md) | Still WebP input through the workspace port with explicit crop cost, color, orientation and memory limits | Experimental |
| [0030](0030-still-webp-output-and-format-neutral-encoder-result.md) | Still WebP output, format-tagged encoder results, and backend use without `avif-write` | Experimental |
| [0032](0032-jpeg-xl-workspace-decoder-input.md) | JPEG XL input through the workspace reference decoder, with a budgeted full-frame fallback | Experimental |

## Changing an ADR

Edit the original if refining or correcting the same decision; update **Last reviewed** and append the key change to the amendment log. Write a new ADR when fundamentally replacing a decision or choosing a materially different direction, and cross-link `Supersedes`/`Superseded by`. Significant changes should include a small experiment or empirical motivation.
