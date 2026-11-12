# ADR-0032: Read JPEG XL through the workspace decoder with a full-frame fallback

- **Status:** Experimental
- **Date:** 2026-11-11
- **Last reviewed:** 2026-11-12
- **Supersedes:** None; adds a backend alongside the interim reader in [0018](0018-optional-codec-and-cms-dependencies.md), opt-in on 2026-11-11 and the default when compiled in since 2026-11-12 (pending owner confirmation)
- **Related:** [0015](0015-typed-output-color-signals.md), [0019](0019-source-side-orientation.md), [0025](0025-codec-selection-reference-conformance-and-image-scope.md), [0026](0026-codecs-in-one-repository-and-workspace.md)

## Context

The local libjxl-rs port implements the pinned reference decoder and retained C++-oracle captures. Its public decoder API has no crop or ROI request. The interim jxl-oxide adapter requests a crop; that capability cannot be transferred to this decoder. JPEG XL remains an experimental input; the full external JPEG XL conformance corpus and codec-selection measurements remain open.

## Decision

*The default selection in this paragraph was replaced on 2026-11-12; see [Default switch](#default-switch-2026-11-12-pending-owner-confirmation). The wrapper, rejection, color and memory rules stand.*

Use a separate, private `ferrisma-jxl-decoder` crate to own libjxl's raw decoder pointer and input/output lifetimes. Keep `jxl-oxide` as the default `jpeg-xl-read` backend. The new `jpeg-xl-own-read` feature compiles the local decoder; `TransformOptions::jxl_decoder = JpegXlDecoder::LibjxlRs` or `--jxl-decoder=libjxl-rs` selects it explicitly. The selection does not change when both features compile, including `--all-features`; requesting a backend whose feature is missing yields `FeatureDisabled`. The engine remains `unsafe`-free and holds only a safe slice API. Inspect basic info and the `DATA` output ICC before allocating pixels; reject animation, unknown layouts, other extra channels, float/HDR exponent channels, zero or excessive dimensions, and profiles above 16 MiB. Preserve the decoder's default upright orientation. Report the emitted F32 color samples using the `DATA` ICC, not the codestream's `ORIGINAL` profile. Report alpha association from basic info. Existing color/HDR gates decide whether those samples can enter a requested transform; no ICC header is treated as a conversion.

Each read reconstructs the full upright frame into an engine-budgeted F32 buffer, then lends cropped rows. It reports `FullFrameFallback`, one known full-frame buffer and unbounded opaque codec memory. A crop does not avoid any known codec reconstruction. Buffer allocation is preceded by the configured pixel limit and checked sample/byte arithmetic. The exact pixel buffer is charged to the shared managed budget; libjxl internal allocations remain outside that budget. Decoder destruction releases registered pointers before borrowed input and caller output storage can die. A failed or truncated decode never emits rows.

## Consequences

The reference-origin decoder can enter the experimental pipeline behind its own safety boundary. The [photo-sized comparison](../benchmarks/M2-jxl-input-switch.md) found the local decoder about 3.6× slower for full input and 9.5× slower for a center crop, so it remains opt-in while jxl-oxide stays the default. The fallback and budget failure are visible. The wrapper retains jxl-oxide as a development-only comparison dependency; the engine retains it separately for the default runtime path.

## Alternatives considered

- Switch the default immediately: rejected after the photo-sized timing regression and because ADR-0025's complete selection gates remain open.
- Pretend libjxl's output buffer accepts a crop: its API has no such parameter, and slicing a full output cannot be described as avoided reconstruction.
- Add a codec-local crop API: a separate port change requiring independent oracle and quality evidence.

## Validation

The existing eight lossless, independently libjxl-encoded orientation fixtures match upright expected RGB samples on full and cropped reads. Adapter tests compare crop samples against a full read, verify full-frame accounting and an ROI-sized budget failure, and exercise malformed header handling, grayscale and alpha association. The port's 14 retained public-decoder captures pass the Rust test and were replayed directly against the prebuilt pinned C++ oracle with byte-identical observations on 2026-11-11. The broader scoped C++ oracle remains a separate gate. The [comparison record](../benchmarks/M2-jxl-input-switch.md) measures release-mode full/crop decoding of the same input on one thread and records elapsed time plus process max RSS. On 2026-11-12 the pinned external JPEG XL conformance corpus (`libjxl/conformance` `b1d0f990`) passed under its own runner and tolerances on macOS ARM64: 27 of 27 level-10 and 24 of 24 level-5 cases through a djxl-equivalent harness, with byte-identical original profiles and reconstructed JPEGs ([record](../../codecs/libjxl-rs/docs/CONFORMANCE.md)). Through this wrapper, built as the engine builds it, all 32 cases it accepts pass in the runner's application mode after the runner converts the emitted samples from the wrapper's `DATA` ICC to the reference profile. This covers straight and premultiplied alpha, gray, XYB fallback profiles and orientation. The other eight (animation, CMYK, spot colors, float samples) are rejected with `Unsupported` before pixel allocation. Wrapper unit tests also cover float, spot and multi-channel rejection, and an `ImageInfo` from another image being refused before any output buffer is registered.

## v1.0 adoption assessment (2026-11-12)

Correctness no longer blocks a default switch. The decoder is the reference port (ADR-0025 point 1), matches the pinned C++ decoder on every `decode_test.cc` history, and passes the external conformance corpus; the wrapper review found no ownership or error-mapping defect. It owns the decoder pointer, ties the input lifetime to the decoder, drops the decoder before the caller's buffer is released, checks pixel limits, sample arithmetic and the 16 MiB profile bound before allocation, and maps every non-success event to an error before rows are emitted. It does not catch panics from the port; panic freedom on malformed input remains fuzzing work (#65, #94).

Performance blocks the switch. The only recorded comparison is the [photo-sized measurement](../benchmarks/M2-jxl-input-switch.md): about 3.6× slower than jxl-oxide for a full 2268×1512 decode and about 9.5× slower for a center crop, on one thread on Apple M1 Ultra. Two known port limitations are the likely contributors to the full-decode gap; neither has been measured in isolation. Both are release work if v1.0 keeps its "workspace port by default" scope:

- #96: the decoder runs only scalar paths; libjxl's Highway SIMD specializations are not ported.
- #100: RunOnPool task bodies run one at a time under a runner, so threads bring no decode speedup.

The crop gap has an additional structural cause that neither issue removes: the public decoder has no region request, so every crop reconstructs and budgets the full frame. Closing it would require a codec-local region API (a separate port change, see Alternatives) or accepting full-frame crops for JPEG XL.

Recommendation: keep jxl-oxide as the default until a timing phase on an otherwise idle host, after #96 and #100, measures full and crop decode against jxl-oxide on the ADR-0025 corpus. Switch the default for v1.0 if the full-decode gap is within a bound the maintainer accepts and the full-frame crop cost is accepted as a documented JPEG XL limitation. Otherwise, record JPEG XL input as the v1.0 exception in #196 and #67. In either case, keep jxl-oxide available as a fallback until ADR-0025 point 4 is satisfied. Still open beside timing: conformance on x86-64 and Linux, binary size, and process-level peak memory on the selection corpus.

## Default switch (2026-11-12, pending owner confirmation)

The owner's direction for v1.0 (2026-11-12) is to ship the engine on the project's own four codec ports by default, to keep external backends selectable, and to treat known performance gaps as follow-up work rather than release blockers. Under that direction the assessment above is resolved in favor of the switch, and the recommendation to wait for the timing phase is not followed for the default:

- `JpegXlDecoder::default()` is `LibjxlRs` whenever `jpeg-xl-own-read` is compiled in. `JxlOxide` is the default only in a build that enables `jpeg-xl-read` without `jpeg-xl-own-read`; with neither feature the default names `LibjxlRs` and reports `FeatureDisabled`. The `cli` feature enables both, so `--jxl-decoder=jxl-oxide` and `JpegXlDecoder::JxlOxide` remain explicit, tested backends. Cargo feature unification can therefore change the default of a build that enabled only `jpeg-xl-read`; callers that need a fixed backend select it explicitly.
- **Evidence relied on:** the reference-port origin (ADR-0025 point 1); the pinned external conformance corpus, 27/27 level-10 and 24/24 level-5 list entries (51/51) on macOS ARM64 ([record](../../codecs/libjxl-rs/docs/CONFORMANCE.md)); the wrapper's application-mode run (32 accepted cases pass, 8 rejected by design); and the wrapper ownership/error review above.
- **Accepted trade-off:** on the only recorded comparison ([photo-sized measurement](../benchmarks/M2-jxl-input-switch.md), one thread, Apple M1 Ultra) the workspace reader is about 3.6× slower than jxl-oxide for a full 2268×1512 decode and about 9.5× slower for a center crop. Every crop reconstructs and budgets the full upright F32 frame (no region API), and the report says so: `SourceAccess::FullFrameFallback`, one known full-frame buffer, and opaque, unbounded codec memory. A JPEG XL crop therefore costs a full decode in time and engine-managed memory. These numbers are older, single-host measurements and are not re-measured here.
- **Gates that stay open:** the conformance run on x86-64 Linux; a timing phase on an otherwise idle host after #96 (SIMD) and #100 (pool tasks), on the ADR-0025 corpus; binary size and process-level peak memory; a codec-local region API. jxl-oxide is not removed (ADR-0025 point 4).
- **Pending owner confirmation:** this switch implements the stated direction on a best estimate. The owner has not yet reviewed the specific trade-off above; it is reversible by restoring the `Default` implementation of `JpegXlDecoder` and removing `jpeg-xl-own-read` from `cli`.

Tests: `pipeline_tests::default_jpeg_xl_decoder_follows_the_compiled_backends` checks the default end to end per feature set; `hardening_tests::missing_jpeg_xl_backends_report_feature_disabled` checks each backend's missing feature; jxl-oxide-specific tests (codec region request, animation, hostile color, decoder work) now select it explicitly.

## Review when

Review after the conformance run on x86-64 and Linux, the timing phase after #96 and #100, realistic-image time/RSS comparisons, a libjxl region-decode API, a color or alpha mismatch, or a supported input that requires additional extra channels. Re-evaluate the default switch if the owner rejects the trade-off, if the idle-host timing shows a full-decode gap the owner does not accept, or if x86-64 conformance fails.

## Amendment log

- 2026-11-11 — Added the independent safe wrapper and explicit full-frame experimental input path. The photo-sized time/RSS comparison retained jxl-oxide as the default under ADR-0025.
- 2026-11-12 — Recorded the external conformance corpus (all level-10 and level-5 cases pass on macOS ARM64) and the wrapper's application-mode run (32 accepted cases pass, 8 rejected by design). Added the v1.0 assessment: correctness is no longer blocking; the measured performance gap blocks the switch pending #96 and #100. The default is unchanged.
- 2026-11-12 — Under the owner's v1.0 direction (own ports by default, performance gaps as follow-ups), the workspace reader becomes the default whenever `jpeg-xl-own-read` is compiled in, and `cli` enables it. jxl-oxide stays an explicit backend. The 3.6× full / 9.5× crop regression and full-frame crops are recorded as the accepted trade-off, pending owner confirmation; x86-64 conformance, idle-host timing and the region API stay open. The earlier default rationale above is kept for history.
- 2026-11-12 — Indicative loaded-host re-measurement in the [v1.0 baseline](../benchmarks/v1.0-baseline-2026-11-12.md#1-jpeg-xl-decode-libjxl-rs-versus-jxl-oxide-55-93): the VarDCT flower gap reproduces (3.2× full, 8.8× crop on one thread); on lossless 3.4–24 MP photos the workspace decoder is about 17–19 % faster for a full frame but uses about 1.8× the RSS (618 versus 353 MB at 24 MP), and crops cost 2.2–2.6× jxl-oxide. No lossy 12–48 MP JXL and no C++ reference timing were available. The default is unchanged; the idle-host timing gate stays open.
- 2026-11-12 — [Performance analysis](../benchmarks/v1.0-jxl-performance-analysis-2026-11-12.md) (indicative; loaded host) with the pinned C++ libjxl built scalar (the oracle's configuration) and with SIMD: on cjxl `-e 7 -d 1` lossy files the reader is 8.1–8.2× slower than jxl-oxide (the 3.2× JPEG-reconstruction figure understates it) and about 7× the same-arithmetic scalar C++; SIMD and threads explain only a minor part of the gap. Two exact port changes landed with it: row spans resolved once per row in Gaborish, EPF, YCbCr and chroma upsampling (−20 % lossy, −25 % JPEG-reconstruction decode), and lazily zeroed default allocations, which bring lossless decode RSS to the C++ figure (24 MP: 618 → 323 MB; engine CLI 640 → 346 MB). The 1.8× RSS finding above is superseded; the time and crop gaps remain, with ranked fixes in that record.
