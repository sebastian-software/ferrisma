# ADR-0017: Expose one minimal transcode facade and a feature-gated CLI

- **Status:** Experimental
- **Date:** 2026-11-05
- **Last reviewed:** 2026-11-05
- **Supersedes:** The "no public fluent API or CLI as initial prerequisites" clause of [ADR-0008](0008-initial-vertical-slice.md), for M1 only
- **Related:** [0008](0008-initial-vertical-slice.md), [0012](0012-codec-source-contract.md), [0016](0016-working-color-space-and-transform-boundaries.md), [M1 plan](../milestones/M1-complete-image-pipeline.md) work package 8

## Context

ADR-0008 deliberately kept a public API and CLI out of the first vertical slice so that execution contracts were validated before interface work. M1 work package 8 requires a small Rust API and CLI that exercise the *same* core as the tests (`input → crop → resize → AVIF`). PR #11 added `ferrixel::pipeline` and `src/bin/ferrixel.rs` without recording that decision. This ADR records the boundary that now exists and what it deliberately does not promise.

Verified facts (2026-11-05, `main` at 9291c33):

- The only public module is `pipeline`, compiled only with the `avif-write` feature because AVIF is its sole output; without it the library exposes no public API. Graph, planner, executor, codec adapters, color contract and memory types stay `pub(crate)`.
- The facade has one entry point, `transcode(encoded, format, options) -> Result<Output, ProcessingError>`, plus plain option, report and enum types.
- The binary requires the `cli` Cargo feature, which enables the JPEG, JPEG XL and AVIF readers, the AVIF writer and `icc`. The library builds without any codec.
- The package is `publish = false`, version 0.1.0.

## Decision

Keep exactly one public, intentionally small facade for M1:

1. **Surface.** `pipeline::{transcode, TransformOptions, Crop, ResizeFilter, InputProfilePolicy, ImageFormat, Output, ProcessingReport, StageTimings, WorkingPrecision, ChromaSubsampling, KernelLevel, KernelLevelReason, Engine, BatchItem, SourceAccess, ProcessingError, ProcessingErrorKind}`. No builder/fluent DAG API, plugin registry, async interface, streaming input, Node.js bindings or GPU options.
2. **Color intent is explicit.** `InputProfilePolicy::Strict` is the default. `AssumeSrgbIfMissing` applies only when the decoder reports neither rendered ICC nor CICP; `UseRenderedIcc` requires the `icc` feature. The facade never guesses a profile.
3. **Resources are explicit options.** Managed-memory budget, compressed-input byte limit and decoded-pixel limit are caller options with conservative defaults. The report keeps engine-managed memory, known codec materializations and opaque codec/encoder memory separate.
4. **No backend type leaks.** Codec errors are converted at the boundary; codec crates never appear in public signatures. `ProcessingError::kind()` returns a `#[non_exhaustive]` `ProcessingErrorKind` (invalid options, input limit, disabled feature, malformed input, unsupported input, unsupported color, resource exhaustion, cancellation, encoder failure, internal) so callers can match without parsing messages; the message keeps the details.
5. **CLI is an example consumer.** `src/bin/ferrixel.rs` only parses arguments and calls `transcode`; it adds no processing logic. It stays behind `cli`.
6. **No stability promise.** Until a first publication decision, any part of the facade may change without deprecation.

## Consequences

+ M1 can be exercised end to end by a real consumer without exposing the graph or executor.
+ Codec and CMS features remain optional; the core crate's only unconditional dependency is the first-party `ferrixel-kernels` (ADR-0020).
- Error kinds are deliberately coarse; finer-grained variants (for example which limit or which color signal) are a later, additive change.
- The facade decodes one image to one AVIF output. Multi-output DAGs, other output formats and streaming input are out of scope (M2+).
- EXIF, JPEG XL and AVIF orientation are applied with crops in display coordinates (ADR-0019); AVIF `clap` is still rejected with `ProcessingErrorKind::UnsupportedInput`.
- A public facade makes later internal renames cheaper than API changes; keeping the surface small is the mitigation.

## Alternatives considered

- **Keep everything private until M1-RC:** satisfies ADR-0008 literally, but leaves work package 8 and its acceptance test without a real consumer.
- **Expose the graph and planner:** gives flexibility, but freezes contracts that ADR-0002/0003/0016 still treat as experimental or proposed.
- **Fluent builder API:** convenient, but presumes an operation set and composition model that M1 does not need.
- **Separate CLI crate now:** ADR-0008 asks to split crates only after a demonstrable boundary; a feature-gated binary suffices.

## Validation

- `pipeline_tests` run real JPEG → AVIF, JPEG XL + ICC → AVIF and strict sRGB AVIF → AVIF through the facade.
- `hardening_tests` cover compressed-input and pixel limits, invalid options and malformed inputs through `transcode`, and assert the matching `ProcessingErrorKind`.
- CI compiles each codec feature alone and the `cli` feature.

Before **Accepted**: documented supported-input matrix, and at least one consumer outside the test suite.

## Review when

A first publication is decided; a consumer needs multiple outputs, streaming input or another output format; error matching becomes necessary; or the facade starts duplicating planner logic.

## Amendment log

- 2026-11-05 — Recorded the facade and CLI introduced by PR #11 as an experimental M1 boundary.
- 2026-11-05 — Added `ProcessingErrorKind`; replaced the crate-wide dead-code allowance by scoped allowances on test-only experiments.
- 2026-11-05 — Added `StageTimings` (open, decode, transform, encode, total wall-clock) to `ProcessingReport` for the M2 baseline harness; the CLI prints them and a stable `report:` key=value line. Observations, not claims.
- 2026-11-05 — Added `WorkingPrecision` (`F32` default, `F64Reference`) as `TransformOptions::precision` and `ProcessingReport::working_precision`, and the CLI flag `--reference`, so the M1 oracle stays reachable for verification and before/after measurements (ADR-0021).
- 2026-11-06 — Added `TransformOptions::encoder_threads` (default 1, 0 = all cores, at most 256), `ProcessingReport::encoder_threads` and the CLI flag `--encoder-threads`. The default keeps output bytes independent of the machine, because `ravif` derives AV1 tiles from the thread count.
- 2026-11-06 — Added `ChromaSubsampling` (`Yuv444` default, `Yuv420`) as `TransformOptions::chroma_subsampling` and the CLI flag `--chroma=444|420`. `encoder_threads` changed meaning slightly: 1 encodes one AV1 tile, any other value uses tiles derived from the output size, so the bytes no longer depend on the thread count or the machine. `encoder_output_frame_bytes` now reports the encoder's YUV input frame instead of an RGBA copy.
- 2026-11-06 — Kept `ChromaSubsampling::Yuv444` as the default after the 4:2:0 measurements (maintainer decision): full-resolution chroma matches Sharp's AVIF default and avoids a silent output change; callers opt into 4:2:0 explicitly.
- 2026-11-06 — Added `KernelLevel` and `KernelLevelReason` (re-exported from `ferrixel-kernels`), `TransformOptions::kernel_level_ceiling` (`None` picks the best supported level), `ProcessingReport::kernel_level` and `kernel_level_reason`, and the CLI flag `--kernel-level=scalar|neon|avx2`, as ADR-0020 decision 5 requires.
- 2026-11-06 — Added `TransformOptions::transform_threads` (default 0 = all cores; output byte-identical for every value), `ProcessingReport::transform_threads`, the CLI flag `--transform-threads`, and `Engine` with `BatchItem` for in-process batches (ADR-0022). The facade's surface grows by one type with two methods; a long-lived pool, async and streaming input remain out of scope.
