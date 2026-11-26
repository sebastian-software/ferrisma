# Architecture Decision Records

Ferrisma keeps **living ADRs**: decisions are durable and reviewable, not immutable historical monuments. A decision can change when evidence changes, but its previous rationale and the reason for revision must remain discoverable.

For implemented capabilities and package status, use [current status](../status.md).
The [documentation guide](../README.md) also links the codec-local ADR indexes.

## Lifecycle

- **Accepted:** the rule that holds now. ADRs are living records (owner decision, 2026-11-26): none stays Proposed or Experimental; when reality changes, the ADR is amended with a dated log line. Parts not implemented yet are stated as such, and genuinely open owner questions are kept as an "Open questions" list inside the record.
- **Proposed** and **Experimental:** no longer used for ADRs in this directory; the vocabulary remains only for [TEMPLATE.md](TEMPLATE.md) drafts.
- **Superseded:** replaced by a linked ADR.
- **Rejected:** considered and declined, with an explanation.

Every ADR contains **Status**, **Date**, **Last reviewed**, **Context**, **Decision**, **Consequences**, **Alternatives**, **Validation**, **Review when**, and an **Amendment log**. Update this index whenever status or topic changes. Separate measured facts from hypotheses; avoid declaring architecture decisions from attractive diagrams alone. Use [TEMPLATE.md](TEMPLATE.md) for new ADRs.

## Current decisions

| ADR | Topic | Status |
| --- | --- | --- |
| [0001](0001-project-thesis.md) | Pipeline-first project principles and modern-format scope | Accepted |
| [0002](0002-semantic-graph-and-planner.md) | Semantic graph separated from capability-aware planner | Accepted |
| [0003](0003-demand-driven-regions.md) | Demand propagation, region footprints and fallback | Accepted |
| [0004](0004-buffer-ownership-and-budget.md) | Ownership, allocation, conservative resize-plan admission and realistic memory budgets | Accepted |
| [0005](0005-codec-capabilities-and-pixel-contracts.md) | Modern codec adapters, color and layout contracts | Accepted |
| [0006](0006-cpu-first-gpu-capable-execution.md) | Execution scheduling, SIMD and conditional GPU | Accepted |
| [0007](0007-correctness-and-performance-evidence.md) | Correctness, reproducible benchmark evidence and bounded quality calibration before optimization | Accepted |
| [0008](0008-initial-vertical-slice.md) | Minimal architecture-validating implementation | Accepted (partly superseded by 0017 and 0026) |
| [0009](0009-living-adr-process.md) | Decision lifecycle and amendment policy | Accepted |
| [0010](0010-dual-licensing.md) | Dual-license Ferrisma under MIT OR Apache-2.0 | Accepted |
| [0011](0011-borrowed-planar-surface-contract.md) | Borrowed, byte-strided YUV420P10 source contract | Accepted |
| [0012](0012-codec-source-contract.md) | Real decoder sources: explicit ROI capabilities and workspace JPEG scanlines | Accepted |
| [0013](0013-sample-domain-resize.md) | Preserve rendered metadata and validate sample-domain codec resize | Accepted |
| [0014](0014-linear-srgb-straight-alpha.md) | Explicit sRGB linear-light resize and premultiplied filtering | Accepted |
| [0015](0015-typed-output-color-signals.md) | Inspect CICP and ICC output evidence; describe required conversions | Accepted |
| [0016](0016-working-color-space-and-transform-boundaries.md) | Region-capable working-color transform and output semantics | Accepted |
| [0017](0017-minimal-public-facade-and-cli.md) | Minimal `pipeline::transcode` facade and feature-gated CLI; partially supersedes 0008 | Accepted |
| [0018](0018-optional-codec-and-cms-dependencies.md) | Interim codec/CMS dependencies and port adoption gates; JPEG input-only; MSRV 1.98; committed lockfile | Accepted |
| [0019](0019-source-side-orientation.md) | Apply EXIF/AVIF orientation through an explicit, budgeted source-side materialization | Accepted |
| [0020](0020-kernel-crate-and-isa-dispatch.md) | Separate `ferrisma-kernels` crate, `std::arch` intrinsics behind a safe API, runtime ISA dispatch reported in the plan, scalar/SIMD differential tests | Accepted |
| [0021](0021-working-precision-policy.md) | Working precision: `f32` with exact LUTs, 1:1 sRGB callback-thread identity pass-through, single-pass tap planning and a testable error bound against the `f64` oracle; the Q1.14 path was measured and dropped | Accepted |
| [0022](0022-parallel-row-pipeline-and-engine.md) | Parallel row pipeline with ordered emission (byte-identical to the sequential executor, standard library only) and an `Engine` for batches | Accepted |
| [0023](0023-decode-side-work-and-scaling-policy.md) | Full-resolution decode by default; decoder SIMD, extended-range linear ICC matrix-shaper output and cheaper orientation; shrink-on-load only as an explicit, measured quality policy | Accepted (decisions 3 and 4 not implemented) |
| [0024](0024-pluggable-encoders-and-libavif-rs-backend.md) | Pluggable row-level encoder backends; libavif-rs (Rust AOM) as an isolated adapter crate (workspace placement superseded by 0026), `rav1e` stays the default | Accepted |
| [0025](0025-codec-selection-reference-conformance-and-image-scope.md) | Codec selection: reference implementation or proven conformance, ports reduced to the still-image scope, deviations and removals only after measurement, mechanical oracle for later optimization; JPEG decode before encode | Accepted |
| [0026](0026-codecs-in-one-repository-and-workspace.md) | One repository and Cargo workspace for the engine, adapter and four codec ports; imported PRs, common verification entry point and early SharpYUV consolidation review | Accepted |
| [0027](0027-tiered-validation-and-rust-test-orchestration.md) | Scoped PR checks, deep validation cadence and Rust orchestration | Accepted |
| [0028](0028-ardo-documentation-site.md) | Ardo website with the shared Ferramenta theme and canonical repository Markdown | Accepted |
| [0029](0029-webp-still-input.md) | Still WebP input through the workspace port with explicit crop cost, color, orientation and memory limits | Accepted |
| [0030](0030-still-webp-output-and-format-neutral-encoder-result.md) | Still WebP output, format-tagged encoder results, and backend use without `avif-write` | Accepted |
| [0031](0031-workspace-libavif-input-experiment.md) | Experimental AVIF input through an isolated safe reader: the workspace dav1d port by default since 2026-11-22, the libaom reference decoder selectable | Accepted |
| [0032](0032-jpeg-xl-workspace-decoder-input.md) | JPEG XL input through the workspace reference decoder, with a budgeted full-frame fallback | Accepted |
| [0033](0033-shared-sharpyuv-conversion-core.md) | One safe SharpYUV conversion core with WebP and AVIF boundary adapters | Accepted |
| [0034](0034-thumbnail-geometry.md) | Upright thumbnail geometry, centered cover, explicit no-enlargement behavior and bounded JPEG/WebP header inspection | Accepted |
| [0037](0037-selectable-workspace-avif-encoder.md) | Explicit workspace libavif/AOM output selection through an independent writer boundary; Rav1e remains default | Accepted |
| [0038](0038-ferrisma-project-name.md) | Ferrisma project identity, technical rename, ferrisma.dev, and published placeholder packages | Accepted |
| [0039](0039-jpeg-xl-output-through-the-workspace-encoder.md) | JPEG XL output through the workspace libjxl-rs encoder: lossy VarDCT by default (straight alpha, patches off) and the verified lossless profile; explicit output semantics and license gate; `jxl-encoder` stays excluded | Accepted |
| [0040](0040-stable-public-contract-and-compatibility-policy.md) | Stable 1.0 public contract tiers, SemVer, Rust version, deprecation and upgrade policy; facade review recommendations | Accepted (tiers bind from 1.0.0; undecided tiers are experimental) |
| [0041](0041-v1-quality-gate-equivalence-with-the-ported-original.md) | v1.0 quality gate: bit identity with the ported original per execution kind and target, NaN payloads excluded, NEON/AVX2 SIMD scope with AVX-512 off by default, local-first validation, three-column performance reporting without a performance gate, 0.x until the gate is met; partially supersedes 0007, 0020, 0025, 0027, 0032, 0037, 0040 | Accepted (open questions O1–O9 decided 2026-11-15) |
| [0042](0042-no-silent-loss-refuse-or-report.md) | No silent loss: the pipeline carries a part of the input over, refuses the input, or names in the result what it left out; errors keep their true class; readers of one format agree on refusals | Accepted (open questions pending) |
| [0043](0043-formats-behave-alike.md) | Formats behave alike where the format allows: the same property of an image gets the same outcome, conversion and wording in every format; encoder controls stay native per format; the supported matrix is where parity is checked | Accepted |
| [0044](0044-no-assembly-acceleration-through-rust-intrinsics.md) | No assembly in the project: acceleration only through Rust intrinsics with narrowly bounded `unsafe`; assembly-only kernels of an original run scalar until rebuilt with intrinsics; vendor-dependent instructions stay as in the original; linking an original's assembly is excluded | Accepted (one open question: the ten inline assembly sites left in libjxl-rs; `cargo xtask assembly` guards the rest) |
| [0045](0045-recipe-api.md) | Recipe API: one source, several crops, several outputs; one quality scale calibrated on SSIMULACRA 2, quality as displayed for dense screens; encoder settings only through configuration | Accepted |

## Changing an ADR

Edit the original if refining or correcting the same decision; update **Last reviewed** and append the key change to the amendment log. Write a new ADR when fundamentally replacing a decision or choosing a materially different direction, and cross-link `Supersedes`/`Superseded by`. Significant changes should include a small experiment or empirical motivation.
