# M1 evidence, open risks and release decision gates

**Status:** Implementation under validation, not M1-RC. This file records reproducible observations and outstanding work, not completion claims. See [current status](../status.md) and the [documentation guide](../README.md). Tracking: [M1 issue #8](https://github.com/sebastian-software/ferrisma/issues/8) and [PR #11](https://github.com/sebastian-software/ferrisma/pull/11).

## Implemented experimental pipeline

- Semantic color-conversion, quality-resample and output-encoding nodes, with region planning and bounded cache/row-buffer ownership.
- CPU scalar SDR sRGB / Display P3 transfer and gamut conversion, explicit straight-alpha association, Area and Lanczos3 filters, and a separate final sRGB8 quantization step.
- Optional JPEG, still WebP, JPEG XL and AVIF sources and optional AVIF and still WebP writers. The writers declare and budget their retained output surfaces. WebP crop output is budgeted, while its decoder-internal memory is opaque ([ADR-0029](../adr/0029-webp-still-input.md), [ADR-0030](../adr/0030-still-webp-output-and-format-neutral-encoder-result.md)).
- AVIF color evidence comes from the primary item's `colr` (`nclx` or ICC) property and the AV1 sequence header, read by a bounds-checked parser before any pixel decode. The adapter reports the emitted full-range RGB as CICP `[primaries, transfer, 0, 1]`, rejects `nclx` that contradicts the matrix, range, primaries or transfer the decoder actually uses, applies supported `irot`/`imir` orientation before cropping, and fails closed on `clap`, unsupported transform ordering, and unknown essential properties.
- A small Rust facade and CLI using the same graph/executor. Untagged inputs require an **explicit** sRGB assumption or an ICC path when supported.
- Optional `moxcms` rendered-ICC-to-sRGB source adapter with an explicit user policy. It is not arbitrary ICC/CICP equivalence or HDR handling.
- Property/golden tests, malformed-input smoke tests, a libFuzzer target, an MSRV 1.98 CI job, single-feature compilation checks and an SPDX metadata inventory.
- A reproducible synthetic RSS/time smoke and a libvips feasibility comparison. See [benchmark protocol](../benchmarks/M1-protocol.md).

## Acceptance gates (not yet fully satisfied)

| Gate | Current evidence | Remaining validation |
| --- | --- | --- |
| 1. Real JPEG/JXL → AVIF | End-to-end test fixtures with optional features | Broad real-image corpus, output profile and orientation cases |
| 2. Color fidelity | Known sRGB/P3 numeric oracles, alpha and crop tests; selected CC0 P3/Adobe ICC samples checked against LittleCMS 2.19, including out-of-gamut values (ADR-0023) | P3/ICC real-image quality, broader out-of-gamut and HDR rejection corpus, LUT/intent coverage |
| 3. Codec capabilities | Reports ROI versus full-frame fallback | Measure internal decoder work and source overfetch on real formats |
| 4. Engine memory | RAII frame, row and scratch reservations; conservative resize-plan admission before footprint construction; cancellation tests (ADR-0004) | Graph/allocator/runtime memory and opaque codec allocations remain outside the model; test representative process RSS and bombs |
| 5. Correctness and errors | Geometry, invalid options, truncated headers, AVIF container/bitstream color conflicts; JPEG EXIF, JPEG XL and AVIF `irot`/`imir` orientation applied and checked against ImageMagick/libheif/libjxl fixtures (ADR-0019) | Larger malicious corpus, AVIF `clap`, alpha and ICC conflict cases |
| 6. Quality/performance baseline | Initial synthetic CI smoke; subsequent [quality-matched Sharp comparison](../benchmarks/M2-sharp-comparison.md) and [libavif backend comparison](../benchmarks/M2-libavif-backend.md), each with a recorded corpus and host | Independent libvips CLI and Butteraugli evidence, larger photographs, broader color/corpus coverage, and representative latency distributions |
| 7. Release/audit | MSRV and feature checks, metadata screening, and a bounded four-input/two-output fuzz smoke ([record](../fuzzing.md)) | Extended coverage-reviewed fuzz campaign, native/unsafe review, third-party license notices and release decision |

## Crucial engineering findings

1. **Lanczos tap minima are not guaranteed monotone** because sinc zero crossings can remove a low-index tap for one output line and reintroduce it later. The row scheduler must keep data until its **last** consumer. A deterministic property test exposed the error and the cache retention policy was corrected in a separate commit.
2. **Historical MPL-2.0 transitive parser:** The initial `avif-decode` reader brought in MPL-2.0 `avif-parse` 2.1.0. ADR-0031 removed both from the live engine graph on 2026-11-11. Ferrisma itself stays MIT OR Apache-2.0; delivery of the replacement graph still needs third-party license, patent and notice/source audit. See [license audit](../license-audit.md).
3. **Memory accounting boundary:** Working-linear resize now reserves its modeled tap/scheduling storage before planning. Internal codec allocations, graph nodes, allocator/runtime internals and OS stacks remain outside `MemoryBudget`. Do not advertise a strict process-wide cap or compare new managed peaks directly with older pixel-only peaks.
4. **Portability:** Own AVIF input/output require the pinned nightly and a supported native host (macOS ARM64 or GNU Linux x86-64). Features without `avif-read`, `avif-own-write`, or `cli` retain stable Rust 1.98 checks. Broad cross-platform validation is not complete; the dated checks below describe the earlier reader.

## 2026-11-11 operational follow-up

- [PR #185](https://github.com/sebastian-software/ferrisma/pull/185) closes a reproduced planning-budget gap: a 10,000×1 Area downscale previously admitted a 160,000-byte tap vector under a 64-byte budget. The request now fails before decoder work. Conservative admission, fallible table allocation, cancellation release and the existing precision/parallel comparisons are covered; [CI run 43071572537](https://github.com/sebastian-software/ferrisma/actions/runs/43071572537) passed. This is not a total-process memory bound.
- [PR #186](https://github.com/sebastian-software/ferrisma/pull/186) exposes the existing workspace AOM writer through the normal facade and CLI, sharing one isolated writer boundary with the compatibility adapter. Tests cover AVIF input/output together, 8/10-bit, 4:2:0/4:4:4, alpha, frame admission and independent decode checks; [CI run 43072824580](https://github.com/sebastian-software/ferrisma/actions/runs/43072824580) passed. Rav1e stays the default; final backend selection remains subject to ADR-0025.
- [PR #187](https://github.com/sebastian-software/ferrisma/pull/187) retains the one-byte fuzz framing while making four input formats and both existing output formats reachable under explicit color policies and resource limits. [Native run 43072724334](https://github.com/sebastian-software/ferrisma/actions/runs/43072724334) passed the eight deterministic seed cases and 3,000 seeded/mutated calls. The exact revision, observed RSS, retained corpus and limits are recorded in [fuzzing](../fuzzing.md); this shallow smoke does not close the sustained-fuzz gate.
- [PR #188](https://github.com/sebastian-software/ferrisma/pull/188) makes row, orientation-gather and batch worker creation fallible, closes partially started queues before joining, and returns resource errors. A batch start gate keeps images from starting until all workers exist; batch worker counts are capped at 256. Injected partial-start failures, cleanup and preserved backend-panic propagation are tested; [CI run 43074579659](https://github.com/sebastian-software/ferrisma/actions/runs/43074579659) passed. OS thread stacks remain outside the managed budget.
- [PR #189](https://github.com/sebastian-software/ferrisma/pull/189) binds encoder evidence to freshly built tools and separate per-thread quality targets. The [dated smoke](../benchmarks/M2-encoder-smoke-2026-11-11.md) retains 60 samples for one quality-80 photograph on x86-64, with normal/masked AOM dispatch. A quality-60 integer gap failed closed; the successful subset does not complete the multi-quality, multi-image, two-architecture selection gate.

These changes improve the experimental path's usability and failure behavior.
They do not change the release decision below or establish general Sharp parity.

## Decision

**Do not merge as an M1 release-candidate** until the independent oracle, qualitative encoding equivalence, security, total resource consumption and license gates above are addressed. A green CI confirms the current tested contracts, not the entire release specification.

The ten work packages remain tracked in [the canonical milestone plan](M1-complete-image-pipeline.md). Do not silently mark them completed based on existence of implementation files.

## 2026-10-31 validation updates

- **Core CI:** [run 41235323262](https://github.com/sebastian-software/ferrisma/actions/runs/41235323262) passed 61/61 codec-free tests and 78/78 all-feature tests, both Clippy jobs, rustfmt, the single-feature matrix, the Rust 1.98 MSRV check and provisional dependency metadata screening. These results precede the additional SPDX slash-syntax unit checks and documentation changes; verify CI again for the final PR head.
- **Fuzz smoke:** [run 41235323267](https://github.com/sebastian-software/ferrisma/actions/runs/41235323267) completed **3,000 libFuzzer executions**, seeded by two real JPEG/JPEG XL fixtures, with no crash reported. The final corpus contained 141 items (~23 KiB) and the sanitizer-instrumented runner reported peak RSS of ~139 MiB. This does **not** certify memory safety or replace sustained coverage-directed fuzzing.
- **Reference quality:** A regression suite found two independent assumptions broken by Lanczos3 sinc zero crossings. Both row-cache eviction and requested-region footprint calculation now consider *all future consumers/all requested destination samples* instead of only adjacent or endpoint locations.
- **Benchmark:** [run 41235323253](https://github.com/sebastian-software/ferrisma/actions/runs/41235323253) obtained Sharp AVIF and Ferrisma AVIF time/RSS/output-size observations. Sharp 0.34.5 was faster in this tiny **unmatched-quality** case (0.11 s vs. 0.69 s); Ferrisma used lower process RSS (~24 MiB vs. ~84 MiB). No inference of equivalent picture quality or end-to-end superiority is permitted.
- **Early decompression guard:** The experimental API rejects compressed inputs beyond 64 MiB by default and has a default 100-megapixel input limit. AVIF inspects its coded sequence dimensions before full-frame pixel decode. This is not a strict cap on opaque codec RSS, and AVIF currently parses compressed metadata twice.
- **Licensing:** The dependency preflight explicitly handles modern SPDX alternative/combined expressions and legacy Cargo dual-license slash syntax. `avif-parse` remains MPL-2.0 and requires a real notice/source/distribution review; a passing metadata screen is not an authorization to distribute as wholly permissive.

**Release decision remains open:** real-photo image quality parity and representative large-image benchmarks, source-code/NOTICE audit, decoder orientation and color profile standards compliance, input memory resource hardening, cross-platform confirmation and a substantially longer fuzz campaign remain unmet.
