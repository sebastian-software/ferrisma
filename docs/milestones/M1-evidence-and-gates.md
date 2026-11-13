# M1 evidence, open risks and release decision gates

**Status:** Implementation under validation, not M1-RC. This file records reproducible observations and outstanding work, not completion claims. See [current status](../status.md) and the [documentation guide](../README.md). Tracking: [M1 issue #8](https://github.com/sebastian-software/ferrisma/issues/8) and [PR #11](https://github.com/sebastian-software/ferrisma/pull/11).

> **2026-11-13:** The v1.0 release gate is now [ADR-0041](../adr/0041-v1-quality-gate-equivalence-with-the-ported-original.md) (owner decision): bit identity of each codec port with its ported original, scalar and SIMD, on AArch64 and x86-64, and no performance gate. The M1 criteria below are unchanged and still describe M1's own acceptance; where gate 6 or the release decision ask for performance or Sharp/libvips comparisons, those are reported context for v1.0, not a release condition.

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
| 2. Color fidelity | Known sRGB/P3 numeric oracles, alpha and crop tests; selected CC0 P3/Adobe ICC samples checked against LittleCMS 2.19, including out-of-gamut values (ADR-0023); Area/Lanczos3 checked against ImageMagick and an independent oracle, one real ICC photograph against LittleCMS, PQ/HLG and contradicting CICP rejected beside a rendered ICC, both JPEG XL decoders' emitted-color declarations (2026-11-12 below) | Photograph corpus, LUT/intent coverage, ICC-signalled JPEG XL fixtures, AVIF/HEIF HDR rejection corpus |
| 3. Codec capabilities | Reports ROI versus full-frame fallback, plus the codec request, codec output, overfetch and lower bounds of reconstruction and entropy work per adapter; pinned counts for JPEG, WebP, both JPEG XL backends and AVIF; jxl-oxide group-skipping probes ([2026-11-12](#2026-11-12-decoder-work-and-orientation-evidence), ADR-0012, ADR-0023) | Codec-internal work beyond the lower bounds, JPEG XL photographs at camera size, wall time and clean-host RSS |
| 4. Engine memory | RAII frame, row and scratch reservations; conservative resize-plan admission before footprint construction; cancellation tests (ADR-0004) | Graph/allocator/runtime memory and opaque codec allocations remain outside the model; test representative process RSS and bombs |
| 5. Correctness and errors | Geometry, invalid options, truncated headers and JPEG tails below a crop, AVIF container/bitstream color conflicts; JPEG EXIF, WebP EXIF, lossless and lossy JPEG XL and AVIF `irot`/`imir` orientation applied and checked against ImageMagick/libheif/libjxl fixtures; animated input rejected for every format that carries it (ADR-0019) | Larger malicious corpus, AVIF `clap`, alpha and ICC conflict cases |
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
- The first hosted ASan campaign ([run 43117625746](https://github.com/sebastian-software/ferrisma/actions/runs/43117625746)) found a JPEG XL colour-encoding panic in the default Linux lane and macOS AVIF-port link failures. Both are fixed (2026-11-12); bounded 600-second local ASan runs of all three lanes on macOS ARM64 found no further failure. Results, hostile AVIF/ICC corpora and the preflight audit are in [fuzzing](../fuzzing.md#campaign-results); the sustained coverage-reviewed campaign remains open.
- [PR #188](https://github.com/sebastian-software/ferrisma/pull/188) makes row, orientation-gather and batch worker creation fallible, closes partially started queues before joining, and returns resource errors. A batch start gate keeps images from starting until all workers exist; batch worker counts are capped at 256. Injected partial-start failures, cleanup and preserved backend-panic propagation are tested; [CI run 43074579659](https://github.com/sebastian-software/ferrisma/actions/runs/43074579659) passed. OS thread stacks remain outside the managed budget.
- [PR #189](https://github.com/sebastian-software/ferrisma/pull/189) binds encoder evidence to freshly built tools and separate per-thread quality targets. The [dated smoke](../benchmarks/M2-encoder-smoke-2026-11-11.md) retains 60 samples for one quality-80 photograph on x86-64, with normal/masked AOM dispatch. A quality-60 integer gap failed closed; the successful subset does not complete the multi-quality, multi-image, two-architecture selection gate.

These changes improve the experimental path's usability and failure behavior.
They do not change the release decision below or establish general Sharp parity.

## 2026-11-12 decoder work and orientation evidence

Issues #19 and #68. Counts only; no timing was recorded because the host was
under heavy parallel load. Maximum RSS below is an **indicative** observation
from `/usr/bin/time -l` on that loaded host, not a measurement; a clean
re-measurement of RSS and time is deferred.

**Reporting.** `ProcessingReport::decoder_work` (CLI `decoder_work:` line) gives the
upright crop, the executor request, the codec request in stored coordinates,
the codec output rectangle, the overfetch, and `Exact`/`AtLeast` counts of
reconstructed pixels and entropy-decoded rows (ADR-0012). The executor request
equals the crop and each row is delivered once: there is no filter halo beyond
the crop and no duplicated row.

**Defects fixed.**

- JPEG tail validation did not run: skipping to the last row marked EOI as
  reached, so a top crop of a file truncated below it succeeded. Fixed and
  covered by `codecs::jpeg::tests::a_top_crop_of_a_file_truncated_below_it_fails_closed`.
- The jxl-oxide adapter rendered the first keyframe of an animated JPEG XL file.
  It now rejects animation (`tests/animated_input.rs`).
- The orientation gather rejected gray+alpha rows (latent; no current adapter
  emits them with an orientation).

**Deterministic tests.** `tests/decoder_work.rs` (JPEG 4:2:0 crop: 46×20 output
for a 40×20 request, 120 pixels of overfetch, 302 of 302 rows entropy-decoded;
EXIF-6 crop mapping and 1,080 copied bytes; lossy WebP: 96×49 output for a
40×30 stored request; lossless WebP: 41×31 output, ≥ 3,936 reconstructed
pixels; jxl-oxide: output equals request, lower bound only; libjxl-rs and AVIF:
full frame), `tests/animated_input.rs`, the jxl-oxide probes in
`codecs::jxl_oxide::tests`, `orientation::tests::lossy_jpeg_xl_*`,
`every_orientation_keeps_channels_alpha_and_color_for_every_crop` and
`camera_size_orientation_costs_are_reported_exactly`.

**jxl-oxide 0.12.6 probes.** Inverting every byte of pass group 1 leaves a crop
away from it bit-identical on two 257×129 multi-group VarDCT files, while the
full frame and a crop over that group change or fail. A single-TOC frame and a
patch reference frame are read for any crop. Noise and splines crops equal the
full render. This is evidence for those files only.

**Large inputs.** Release CLI at this branch, Apple M1 Ultra, macOS 27.0.1,
`rustc 1.100.0-nightly (5a2be9f5f 2026-09-06)`, `--assume-srgb --budget-mb=512`,
WebP output, one transform thread unless noted. The inputs were generated from
the libavif photograph `paris_exif_xmp_icc.jpg`:
`magick paris_exif_xmp_icc.jpg -strip -filter Lanczos -resize 6000x4000! -attenuate 0.3 +noise Gaussian base24.png`
(likewise 8000x6000), then encoded with `magick base.png -quality 90 -sampling-factor 2x2 out.jpg`
and an EXIF APP1 (big-endian IFD0 `Orientation`) inserted after SOI;
`magick base24.png -quality 85 webp24.webp` with `webpmux -set exif` for
orientation 6; `magick base24.png -quality 60 -define heic:speed=9 [-orient RightTop] avif24.avif`
(libheif 1.23.7, `irot`).

| input (SHA-256 prefix) | request | codec output | overfetch px | entropy rows | buffer / copied bytes | managed peak | max RSS |
| --- | --- | --- | ---: | --- | --- | ---: | ---: |
| JPEG 6000×4000 o1 (`8987b123`) | full | full | 0 | 4000 | 0 / 0 | 21,430,112 | 13.5 MB |
| JPEG 6000×4000 o2 (`90417369`) | full | full | 0 | 4000 | 72,000,000 / 144,000,000 | 93,430,112 | 87.5 MB |
| JPEG 6000×4000 o6 (`276418bc`) | full | full | 0 | 4000 | 72,000,000 / 144,000,000 | 93,202,112 | 86.3 MB |
| JPEG 8000×6000 o1 (`a6592acd`) | full | full | 0 | 6000 | 0 / 0 | 30,531,312 | 20.0 MB |
| JPEG 8000×6000 o2 (`3e0c3646`) | full | full | 0 | 6000 | 144,000,000 / 288,000,000 | 174,531,312 | 166.1 MB |
| JPEG 8000×6000 o6 (`39e05889`) | full | full | 0 | 6000 | 144,000,000 / 288,000,000 | 174,303,312 | 166.2 MB |
| JPEG 8000×6000 o6, crop 2000×1500+1000+1000 | 1500×2000+1000+3000 | 1509×2000+992+3000 | 18,000 | 6000 | 9,000,000 / 18,000,000 | 17,081,312 | 26.3 MB |
| JPEG 8000×6000 o2, same crop | 2000×1500+5000+1000 | 2009×1500+4992+1000 | 13,500 | 6000 | 9,000,000 / 18,000,000 | 17,081,339 | 26.2 MB |
| WebP lossy 6000×4000 o1 (`51cf8137`) | full | full | 0 | ≥ 4000 | 0 / 0 | 93,412,112 | 84.2 MB |
| WebP lossy 6000×4000 o6 (`d87eacaa`) | full | full | 0 | ≥ 4000 | 72,000,000 / 144,000,000 | 165,184,112 | 156.0 MB |
| WebP lossy o6, same crop | 1500×2000+1000+1000 | 6000×3008+0+0 | 15,048,000 | ≥ 3008 | 9,000,000 / 18,000,000 | 71,219,312 | 74.8 MB |
| AVIF 6000×4000 (`8c6d7944`) | full | full | 0 | ≥ 4000 | 0 / 0 | 93,412,112 | 415.1 MB |
| AVIF 6000×4000 `irot` 3 (`54ac73d4`) | full | full | 0 | ≥ 4000 | 72,000,000 / 144,000,000 | 165,184,112 | 486.3 MB |
| AVIF `irot` 3, same crop | 1500×2000+1000+1000 | full | 21,000,000 | ≥ 4000 | 9,000,000 / 18,000,000 | 89,075,312 | 422.1 MB |

Full SHA-256 values: `8987b123822e30c8408a41a73070d6e0cf7df946dcc5bc0b36b8028cd8ffe2f5`,
`904173690fe714505c2a749e7508014e55d6335104c35b6bfe7b8e132d94a4dc`,
`276418bc349a8ec017eab1bb9e407b10b24f87b7597495de17a3da86473add9e`,
`a6592acd5f4a36607da15aedc0ff880404f93bb5819bb846088c907f0256d96c`,
`3e0c3646de7eb545aa8b896811fb2626b380b0619e79ec73951eb9768b7b5278`,
`39e05889f03837e2450ac29b662892e4b66a9eedb8ade4eac34f78cb59e39d5b`,
`51cf813740204bc5bc9eebc2e05c78ffe76cc2d4969ae511ec1ba7fb748a3a75`,
`d87eacaaf13a08a3200b3d2b24e4218ed8450098b16a0a4c36176873dfe6160d`,
`8c6d7944f6734bc7a5890f22cc15f99e5a952aa5e1643be7a82b504742fd5860`,
`54ac73d4961915e7474c16108a0fc3c30b565b4b79bb354c059e5ae65971f602`.
The magick-generated files are not retained; regenerating them with another
ImageMagick or libheif build may change their hashes but not the geometry
counts. Reconstructed-pixel lower bounds equal the codec-output area, plus one
tail row for JPEG crops that end above the bottom (for example ≥ 3,019,509 for
the 48 Mpx orientation-6 crop).

Observations for follow-up: AVIF decode memory is mostly opaque (indicative RSS
4–5× the managed peak); a low lossy-WebP crop holds most of the frame in
managed memory; mirror-only orientation 2 costs as much as a rotation (ADR-0019
keeps the materialization).

**Re-measurement, 2026-11-12 (indicative; loaded host).** The
[v1.0 baseline](../benchmarks/v1.0-baseline-2026-11-12.md#3-decode-stage-times-rss-and-orientation-cost-at-12-and-24-mp-19-68)
supersedes the 24 MP RSS entries above for JPEG, WebP and AVIF with interleaved
7-sample runs (load average 6.5–8.9) on real 12.2 MP and 24 MP photographs
instead of resized derivatives, and adds stage times. Max RSS, 24 MP, one
transform thread: JPEG o1 35.3 MB, o6 107.0 MB, o2 107.3 MB; WebP 102.3 MB;
AVIF 434.3 MB against 102.7 MB managed; lossless JPEG XL 639.7 MB (libjxl-rs)
and 371.2 MB (jxl-oxide). Orientation gather at 24 MP: +87 ms (o6) and +30 ms
(o2) wall on one thread, about +20 ms with all cores, plus the 72 MB stored-order
buffer in both cases. These numbers stay indicative. The 48 MP rows above are
not re-measured. The same run found that the `mountain` and `deer` corpus
photographs are rejected because their EXIF `Orientation` is stored as LONG.
## 2026-11-12 independent color and resize evidence

- **Resize (#18):** `tests/resize_oracle.rs` checks the facade's Area and Lanczos3 output against retained ImageMagick 7.1.2-35 Q16-HDRI linear-light results and a separately written direct 2D oracle: upscaling, 80x downscaling, odd crops, mixed axis scales, straight-alpha edges and fully transparent pixels, and a broader geometry sweep. The oracles agree within `1.7e-7` linear; the engine (`F32` and `F64Reference`) matches them on every compared RGB sample, with one-code alpha differences only at asserted rounding ties (ADR-0021 rule 4). One intentional difference, unassociation where Lanczos overshoot lifts alpha above one, is recorded in ADR-0014. Display P3 through the rendered-ICC policy keeps out-of-gamut values until the output step. Fixtures, commands and hashes: `tests/fixtures/resize/README.md`; evidence summary in ADR-0016.
- **JPEG XL emitted color (#41):** Under `NullCms`, jxl-oxide 0.12.6 renders into the declared enum encoding and describes it with `rendered_icc`, but its `rendered_cicp` reported D65 Display P3 as CICP primaries 11 (DCI white). The adapter now derives CICP itself. Under the rendered-ICC policy, PQ/HLG JPEG XL was previously converted as SDR through the decoder's synthesized ICC; CICP beside a rendered ICC is now checked and HDR, unknown and contradicting signals are rejected. Tests run against both JPEG XL decoders (ADR-0015, `tests/fixtures/jxl-color/README.md`). ICC-signalled JPEG XL fixtures remain missing because no encoder is available.
- **ICC (#17):** A real photograph's compact sRGB ICC v4 profile, and the same crop as Display P3 with out-of-sRGB samples, match LittleCMS 2.19 within `8e-4` linear; hostile profiles are controlled errors; the bounded header check now runs before moxcms (ADR-0023). LUT profiles, other intents and a corpus remain open.

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
