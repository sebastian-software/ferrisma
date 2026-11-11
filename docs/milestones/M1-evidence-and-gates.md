# M1 evidence, open risks and release decision gates

**Status:** Implementation under validation, not M1-RC. This file records reproducible observations and outstanding work, not completion claims. See [current status](../status.md) and the [documentation guide](../README.md). Tracking: [M1 issue #8](https://github.com/sebastian-software/ferrixel/issues/8) and [PR #11](https://github.com/sebastian-software/ferrixel/pull/11).

## Implemented experimental pipeline

- Semantic color-conversion, quality-resample and output-encoding nodes, with region planning and bounded cache/row-buffer ownership.
- CPU scalar SDR sRGB / Display P3 transfer and gamut conversion, explicit straight-alpha association, Area and Lanczos3 filters, and a separate final sRGB8 quantization step.
- Optional read-only JPEG, still WebP, JPEG XL and AVIF sources and optional AVIF writer; the AVIF writer declares and budgets its required full-frame output surface. WebP crop output is budgeted, while its decoder-internal memory is opaque ([ADR-0029](../adr/0029-webp-still-input.md)).
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
| 4. Engine memory | RAII frame, row and scratch reservations; cancellation tests | Planner metadata is not fully charged; AVIF decoder and encoders have opaque allocations; test peak process RSS and bombs |
| 5. Correctness and errors | Geometry, invalid options, truncated headers, AVIF container/bitstream color conflicts; JPEG EXIF, JPEG XL and AVIF `irot`/`imir` orientation applied and checked against ImageMagick/libheif/libjxl fixtures (ADR-0019) | Larger malicious corpus, AVIF `clap`, alpha and ICC conflict cases |
| 6. Quality/performance baseline | Initial synthetic CI smoke; subsequent [quality-matched Sharp comparison](../benchmarks/M2-sharp-comparison.md) and [libavif backend comparison](../benchmarks/M2-libavif-backend.md), each with a recorded corpus and host | Independent libvips CLI and Butteraugli evidence, larger photographs, broader color/corpus coverage, and representative latency distributions |
| 7. Release/audit | MSRV check, Cargo feature checks, metadata audit and fuzz harness | Extended fuzz campaign, native/unsafe review, third-party license notices and release decision |

## Crucial engineering findings

1. **Lanczos tap minima are not guaranteed monotone** because sinc zero crossings can remove a low-index tap for one output line and reintroduce it later. The row scheduler must keep data until its **last** consumer. A deterministic property test exposed the error and the cache retention policy was corrected in a separate commit.
2. **MPL-2.0 transitive parser:** `avif-parse` 2.1.0 is MPL-2.0 via `avif-decode`. Ferrixel itself stays MIT OR Apache-2.0; delivery of the complete dependency graph still needs third-party license and notice/source audit. See [license audit](../license-audit.md).
3. **Memory accounting boundary:** Internal codec decode/encode allocations and currently some planner metadata structures are not included in `MemoryBudget`. Do not advertise a strict process-wide cap.
4. **Portability:** The active all-feature AVIF path requires Rust 1.98 and may require NASM on x86. CPU-only/no-codec remains optional; broad cross-platform validation is not complete.

## Decision

**Do not merge as an M1 release-candidate** until the independent oracle, qualitative encoding equivalence, security, total resource consumption and license gates above are addressed. A green CI confirms the current tested contracts, not the entire release specification.

The ten work packages remain tracked in [the canonical milestone plan](M1-complete-image-pipeline.md). Do not silently mark them completed based on existence of implementation files.

## 2026-10-31 validation updates

- **Core CI:** [run 41235323262](https://github.com/sebastian-software/ferrixel/actions/runs/41235323262) passed 61/61 codec-free tests and 78/78 all-feature tests, both Clippy jobs, rustfmt, the single-feature matrix, the Rust 1.98 MSRV check and provisional dependency metadata screening. These results precede the additional SPDX slash-syntax unit checks and documentation changes; verify CI again for the final PR head.
- **Fuzz smoke:** [run 41235323267](https://github.com/sebastian-software/ferrixel/actions/runs/41235323267) completed **3,000 libFuzzer executions**, seeded by two real JPEG/JPEG XL fixtures, with no crash reported. The final corpus contained 141 items (~23 KiB) and the sanitizer-instrumented runner reported peak RSS of ~139 MiB. This does **not** certify memory safety or replace sustained coverage-directed fuzzing.
- **Reference quality:** A regression suite found two independent assumptions broken by Lanczos3 sinc zero crossings. Both row-cache eviction and requested-region footprint calculation now consider *all future consumers/all requested destination samples* instead of only adjacent or endpoint locations.
- **Benchmark:** [run 41235323253](https://github.com/sebastian-software/ferrixel/actions/runs/41235323253) obtained Sharp AVIF and Ferrixel AVIF time/RSS/output-size observations. Sharp 0.34.5 was faster in this tiny **unmatched-quality** case (0.11 s vs. 0.69 s); Ferrixel used lower process RSS (~24 MiB vs. ~84 MiB). No inference of equivalent picture quality or end-to-end superiority is permitted.
- **Early decompression guard:** The experimental API rejects compressed inputs beyond 64 MiB by default and has a default 100-megapixel input limit. AVIF inspects its coded sequence dimensions before full-frame pixel decode. This is not a strict cap on opaque codec RSS, and AVIF currently parses compressed metadata twice.
- **Licensing:** The dependency preflight explicitly handles modern SPDX alternative/combined expressions and legacy Cargo dual-license slash syntax. `avif-parse` remains MPL-2.0 and requires a real notice/source/distribution review; a passing metadata screen is not an authorization to distribute as wholly permissive.

**Release decision remains open:** real-photo image quality parity and representative large-image benchmarks, source-code/NOTICE audit, decoder orientation and color profile standards compliance, input memory resource hardening, cross-platform confirmation and a substantially longer fuzz campaign remain unmet.
