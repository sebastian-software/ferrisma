# ADR-0032: Read JPEG XL through the workspace decoder with a full-frame fallback

- **Status:** Accepted
- **Date:** 2026-11-11
- **Last reviewed:** 2026-11-27
- **Supersedes:** None; adds a backend alongside the interim reader in [0018](0018-optional-codec-and-cms-dependencies.md). It became the default on 2026-11-12, was confirmed by the owner on 2026-11-13, and is the only reader since 2026-11-15.
- **Superseded in part by:** [0041](0041-v1-quality-gate-equivalence-with-the-ported-original.md) (the default is confirmed; comparisons with jxl-oxide are context, not a gate); the owner decision of 2026-11-15 recorded [below](#the-only-reader-2026-11-15-owner-decision) (the workspace decoder is the only JPEG XL reader; jxl-oxide leaves the runtime)
- **Related:** [0015](0015-typed-output-color-signals.md), [0019](0019-source-side-orientation.md), [0025](0025-codec-selection-reference-conformance-and-image-scope.md), [0026](0026-codecs-in-one-repository-and-workspace.md)

## Context

The local libjxl-rs port implements the pinned reference decoder and retained C++-oracle captures. Its public decoder API has no crop or ROI request. The interim jxl-oxide adapter requests a crop; that capability cannot be transferred to this decoder. JPEG XL remains an experimental input; the full external JPEG XL conformance corpus and codec-selection measurements remain open.

## Decision

*The default selection in this paragraph was replaced on 2026-11-12; see [Default switch](#default-switch-2026-11-12-pending-owner-confirmation). The wrapper, rejection, color and memory rules stand.*

Use a separate, private `ferrisma-jxl-decoder` crate to own libjxl's raw decoder pointer and input/output lifetimes. The workspace decoder is the only JPEG XL reader (`jpeg-xl-read`); the interim `jxl-oxide` backend and its selectors (`JpegXlDecoder`, `--jxl-decoder`) were dropped on 2026-11-15, and `jxl-oxide` remains a test cross-check. The selection text that follows is historical. The selection does not change when both features compile, including `--all-features`; requesting a backend whose feature is missing yields `FeatureDisabled`. The engine remains `unsafe`-free and holds only a safe slice API. Inspect basic info and the `DATA` output ICC before allocating pixels; reject animation, unknown layouts, other extra channels, float/HDR exponent channels, zero or excessive dimensions, and profiles above 16 MiB. Preserve the decoder's default upright orientation. Report the emitted F32 color samples using the `DATA` ICC, not the codestream's `ORIGINAL` profile. Report alpha association from basic info. Existing color/HDR gates decide whether those samples can enter a requested transform; no ICC header is treated as a conversion.

*The retained-buffer rule in this paragraph was replaced on 2026-11-12; see [Region-sized retained buffer](#region-sized-retained-buffer-2026-11-12). Full-frame reconstruction, the fallback report and opaque codec memory stand.*

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

The crop gap has an additional structural cause that neither issue removes: the public decoder has no region request, so every crop reconstructs and budgets the full frame. Closing it would require a codec-local region API (#197); until then crops are full-frame fallbacks.

Recommendation: keep jxl-oxide as the default until a timing phase on an otherwise idle host, after #96 and #100, measures full and crop decode against jxl-oxide on the ADR-0025 corpus. Switch the default for v1.0 if the full-decode gap is within a bound the maintainer accepts and the full-frame crop cost is accepted as a documented JPEG XL limitation. Otherwise, record JPEG XL input as the v1.0 exception in #196 and #67. In either case, keep jxl-oxide available as a fallback until ADR-0025 point 4 is satisfied. Still open beside timing: conformance on x86-64 and Linux, binary size, and process-level peak memory on the selection corpus.

## Default switch (2026-11-12, confirmed 2026-11-13; superseded by the only-reader decision)

The owner's direction for v1.0 (2026-11-12) is to ship the engine on the project's own four codec ports by default, to keep external backends selectable, and to treat known performance gaps as follow-up work rather than release blockers. Under that direction the assessment above is resolved in favor of the switch, and the recommendation to wait for the timing phase is not followed for the default:

- `JpegXlDecoder::default()` is `LibjxlRs` whenever `jpeg-xl-own-read` is compiled in. `JxlOxide` is the default only in a build that enables `jpeg-xl-read` without `jpeg-xl-own-read`; with neither feature the default names `LibjxlRs` and reports `FeatureDisabled`. The `cli` feature enables both, so `--jxl-decoder=jxl-oxide` and `JpegXlDecoder::JxlOxide` remain explicit, tested backends. Cargo feature unification can therefore change the default of a build that enabled only `jpeg-xl-read`; callers that need a fixed backend select it explicitly.
- **Evidence relied on:** the reference-port origin (ADR-0025 point 1); the pinned external conformance corpus, 27/27 level-10 and 24/24 level-5 list entries (51/51) on macOS ARM64 ([record](../../codecs/libjxl-rs/docs/CONFORMANCE.md)); the wrapper's application-mode run (32 accepted cases pass, 8 rejected by design); and the wrapper ownership/error review above.
- **Accepted trade-off:** on the only recorded comparison ([photo-sized measurement](../benchmarks/M2-jxl-input-switch.md), one thread, Apple M1 Ultra) the workspace reader is about 3.6× slower than jxl-oxide for a full 2268×1512 decode and about 9.5× slower for a center crop. Every crop reconstructs and budgets the full upright F32 frame (no region API), and the report says so: `SourceAccess::FullFrameFallback`, one known full-frame buffer, and opaque, unbounded codec memory. A JPEG XL crop therefore costs a full decode in time and engine-managed memory. These numbers are older, single-host measurements and are not re-measured here.
- **Gates:** the conformance run on x86-64 Linux was met on 2026-11-24; the timing gate was dropped by the 2026-11-13 decision; a codec-local region API is not implemented (#197).
- **Pending owner confirmation:** this switch implements the stated direction on a best estimate. The owner has not yet reviewed the specific trade-off above; it is reversible by restoring the `Default` implementation of `JpegXlDecoder` and removing `jpeg-xl-own-read` from `cli`.

Tests: `pipeline_tests::default_jpeg_xl_decoder_follows_the_compiled_backends` checks the default end to end per feature set; `hardening_tests::missing_jpeg_xl_backends_report_feature_disabled` checks each backend's missing feature; jxl-oxide-specific tests (codec region request, animation, hostile color, decoder work) now select it explicitly.

## Region-sized retained buffer (2026-11-12)

**Problem.** The decoder has no region request, and the adapter also kept the whole reconstructed frame as an engine-managed interleaved F32 buffer: 12 B/px for RGB, 288 MB at 24 MP, for any crop. That made every JPEG XL crop cost a full frame in engine-managed memory and failed 24 MP crops under the CLI's default 256 MB budget.

**Decision.** The wrapper's `decode_region_into` registers the port's image-out callback (`jxl_decoder_set_image_out_callback`) instead of an output buffer and copies only the rows and columns of the requested upright region into a region-sized, engine-budgeted F32 buffer. The adapter's contract is unchanged: the callback receives upright coordinates (the port applies orientation before it, one pixel per call for transposing orientations), so the engine's upright crop maps to the same samples. The samples are bit-identical to the same rectangle of the buffer path, which the wrapper keeps as `decode_into` and uses as the test reference. The adapter uses the callback for every request smaller than the frame and keeps `decode_into` for full-frame requests, which then retain exactly what they did before without a callback per run (one per pixel for transposing orientations measured about 4 % slower on a 24 MP full read). The conformance wrapper example emits through the callback with the full region, so the external corpus exercises the callback path.

**What stays true.** The codec still reconstructs the full upright frame and allocates its own full-frame state outside the budget. Reports keep `FullFrameFallback`, `CodecWork::full_frame` (codec output = whole image, reconstructed pixels and entropy rows at least the full frame) and `opaque_codec_memory_unbounded`. A crop saves no decode time. `known_full_frame_buffers` is now 1 only for a full-frame request, because only then is the retained buffer a full frame.

**Callback safety.** The callback is an `unsafe extern "C"` function, the port's documented contract is that callbacks never unwind or reenter the decoder, and unwinding out of an `extern "C"` function aborts. The body therefore runs under `catch_unwind`; a panic or a run outside the upright image marks the sink failed, later calls are ignored, and the decode returns `Decode` once control is back in Rust. No parallel runner is registered, so the port calls the callback synchronously on the decoding thread, one call at a time, and the sink state lives in `Cell`s read between `process_input` calls through the same shared reference. The sink is declared before the decoder and the decoder is destroyed before the sink and the caller's buffer borrow end. Success requires FULL_IMAGE, SUCCESS and a written-pixel count equal to the region's area. The engine crate stays free of `unsafe`.

**UINT8 output for 8-bit sources (considered, not adopted).** Requesting `JXL_TYPE_UINT8` for 8-bit sources would cut the retained buffer from 12 to 3 B/px, but it is not provably the same as the current path, for three reasons. (1) The port's 8-bit write stage adds libjxl's ordered dither (`stage_write.rs`, `make_unsigned`, amplitude up to ±0.49 LSB) before rounding, so for any sample that is not an exact multiple of 1/255 it emits dithered values the engine never computes; that includes every XYB or VarDCT file and any modular frame with restoration filters, patches, splines, noise, blending or a DATA encoding that differs from the coded one. (2) The public decoder API cannot identify the exact subset: basic info exposes `uses_original_profile` and `bits_per_sample`, but not the frame encoding, restoration filters, or the presence of patches, splines and noise, so "lossless modular 8-bit enum sRGB" cannot be established before choosing the output type. (3) A `U8` source changes the engine path, not just the storage: ADR-0014's linear-sRGB resize domain admits only `U8` sources, and downstream stages would convert `k` back to F32 with their own arithmetic rather than the decoder's `k × (1/255)`. Results could therefore change beyond what ADR-0015/0016/0021 describe as the same emitted samples. The adapter keeps F32 for every source.

**Evidence.** Wrapper tests compare the callback output bit for bit with the buffer path on 24 fixtures (all eight orientations, lossless and lossy, gray, gray+alpha, straight and associated alpha, XYB) for full regions, 1-pixel corners, odd offsets, single rows and columns and far-edge crops; reject invalid regions before decoding; reject truncation, foreign `ImageInfo` and the pixel limit; and inject a panic inside the callback, which fails the decode without aborting. Adapter tests cover cancellation in the middle of row emission, a truncated codestream after the buffer is charged and a budget failure, each leaving no live reservation. Through the release CLI the WebP outputs are byte-identical before and after on the lossless 3.4, 12 and 24 MP photos and on an orientation-6 24 MP JPEG-reconstruction file. Indicative loaded-host measurements ([analysis, section 8](../benchmarks/v1.0-jxl-performance-analysis-2026-11-12.md#8-crop-sized-buffering-through-the-image-out-callback-2026-11-12)): a 24 MP center crop's managed peak falls from 307 to 91 MB and its process RSS from 342 to 126 MB; full-frame reads are unchanged.

**What the port would need to go further.** A codec-local region request (#197, solution 4 of the analysis) to save decode time; a public signal that a frame's output is integer-exact (encoding, restoration filters, patches/splines/noise, blending) and an option to disable dithering for 8-bit output, before UINT8 output could be considered again.

## The only reader (2026-11-15, owner decision)

**Decision.** jxl-oxide leaves the engine's runtime; the workspace decoder is
the only JPEG XL reader. Removed: the jxl-oxide adapter
(`src/codecs/jxl_oxide.rs`), `JpegXlDecoder` and
`TransformOptions::jxl_decoder`, the CLI flag `--jxl-decoder=`, the fallback
"jxl-oxide is the default without the workspace feature", the report value
`decoder_backend = "jxl-oxide"`, the internal error
`UnrenderableColorEncoding` that only that adapter raised, and the direct
dependency on `jxl-grid`. jxl-oxide stays in the repository as an
independent cross-check only: a `[dev-dependencies]` entry of the engine
(no feature enables it) for the tests that compare the workspace decoder's
samples or the encoder's output with an independent decoder, and the
existing dev-dependency of `ferrisma-jxl-decoder` for the benchmark's
context column (`scripts/bench/ci-perf/codec_jxl.sh`).

**Why.** Every JPEG XL robustness finding of the week lay in jxl-oxide and
not in the workspace port: a 1 GiB reservation from an 11-byte file
([fuzzing](../fuzzing.md#hosted-smoke-run-43690971661-2026-11-15)), an ICC
preallocation and render memory the engine could not bound (#200), an
overflow on 31-bit samples (#213), and extra channels dropped by the adapter
(ADR-0042, audit X6 to X9). It is an independent implementation of the
standard, while the v1.0 bar is bit identity with the ported original
(ADR-0041). The condition the owner set, the x86-64 speed of the workspace
decoder, is met: since `75c31f0d` the hosted run 43697684326 measured lossy
decode at 1.38× to 1.46× libjxl's AVX2 build, where it had been 4.5×.

**Feature name.** _Working decision 2026-11-15 (delegated, revisable by the
owner):_ the workspace reader takes the shorter name `jpeg-xl-read`, as
`jpeg-read`, `webp-read` and `avif-read` name the only reader of their
format; `jpeg-xl-own-read` is removed without an alias (pre-1.0). The
stable feature promised the capability, not a backend (ADR-0040), so a
consumer that enabled `jpeg-xl-read` keeps JPEG XL input and now gets the
workspace decoder. The name changed in: the root `Cargo.toml` (`features`,
`cli`, docs.rs metadata), `crates/ferrisma-libavif/Cargo.toml` (`cli`),
`fuzz/Cargo.toml` (the removed `own-jxl` feature), `src/` and `tests/` cfg
gates, `src/cli.rs` (`--version` feature list),
`.github/workflows/ci.yml` (`rust` feature loop, `minimum-rust` set),
`.github/workflows/bench-v1-perf.yml`, `examples/cli_subset.rs`, the
README, `docs/status.md`, `docs/compatibility.md`,
`docs/supported-matrix.md`, `docs/architecture.md`,
`docs/getting-started.md`, `docs/publishing.md` and
`crates/ferrisma-jxl-decoder/README.md`. Dated records (benchmarks,
verification records, the license and unsafe audits) keep the name they
were measured with.

**Tests.** Tests of the adapter itself were removed with it:
`codecs::jxl_oxide::tests` (five: region requests skipping pass groups,
single-TOC crops, patch references, noise and spline crops, animation),
`hardening_tests::interim_truncated_and_mutated_jxl_headers_never_panic_in_adapter_setup`,
`jxl_oxide_unknown_colour_space_is_a_controlled_error`,
`jxl_oxide_oversized_toc_section_is_resource_exhaustion`,
`decoder_tests::interim_jpeg_xl_requests_codec_region_and_delivers_float_scanlines`,
`interim_jpeg_xl_region_samples_match_full_render`,
`decoded_resize_tests::interim_real_jxl_float_roi_flows_through_resize`,
`orientation_tests::interim_jpeg_xl_orientations_are_rendered_upright_for_every_crop`,
`lossy_jpeg_xl_orientations_and_crops_through_jxl_oxide`,
`tests/animated_input.rs::animated_jpeg_xl_is_rejected_by_jxl_oxide`,
`tests/decoder_work.rs::jxl_oxide_crop_reports_only_a_lower_bound`,
`tests/report_backends.rs::explicitly_selected_jxl_oxide_is_reported`,
`pipeline_tests::selecting_uncompiled_local_jpeg_xl_decoder_reports_feature_disabled`,
`default_jpeg_xl_decoder_follows_the_compiled_backends`, and
`silent_drops::jpeg_xl_reader_selection_is_ignored_for_jpeg_input_and_reported`;
each has a workspace-reader twin or tested only the removed selection.
Properties that guard the workspace reader were re-pointed to it, among
them the oversized-TOC and unknown-colour-space regressions through the
public API (`hardening_tests::public_transcode_ends_an_oversized_jxl_toc_section_in_a_controlled_error`,
`public_transcode_rejects_unknown_jxl_colour_space`), the refusals of
ADR-0042 D1 (`silent_drops::jpeg_xl_reader_*`), and the XYB cross-check
(`jxl_color_tests::the_reader_agrees_with_jxl_oxide_on_xyb_rendered_into_display_p3`,
now raw samples of the reader against jxl-oxide). The cross-checks that
decode the encoder's output with jxl-oxide (`jxl_output_tests`,
`jxl_lossy_tests`, `supported_matrix::jpeg_xl_output_is_a_raw_codestream_that_reads_back`,
`tests/jxl_cli.rs`) keep running in every test build, including CI's
`--features jpeg-xl-write` line.

**Consequences.** One JPEG XL reader means ADR-0042 D4 holds by construction
and ADR-0043 D5 has one cell per format. A crop always costs a full decode
(no region request; this section does not change that). Two outcomes change
for the files that only jxl-oxide read differently: the unknown-colour-space
regression is `MalformedInput` without a reason (as libjxl), not
`UnsupportedColor`/`UnsupportedColorEncoding`; the oversized TOC section is
`MalformedInput` under `UseRenderedIcc` (as libjxl), not
`ResourceExhausted`. 21 packages left the fuzz crate's lockfile (the
jxl-oxide family and its dependencies); the root lockfile keeps them for the
dev-dependency.

## Review when

Review after a change to the port's image-out callback contract or write stage, the timing phase after #96 and #100, realistic-image time/RSS comparisons, a libjxl region-decode API, a color or alpha mismatch, or a supported input that requires additional extra channels. Re-evaluate the default switch if the owner rejects the trade-off, if the idle-host timing shows a full-decode gap the owner does not accept, or if x86-64 conformance fails.

## Open questions

- A codec-local region API for the JPEG XL decoder: #197.

## Amendment log

- 2026-11-11 — Added the independent safe wrapper and explicit full-frame experimental input path. The photo-sized time/RSS comparison retained jxl-oxide as the default under ADR-0025.
- 2026-11-12 — Recorded the external conformance corpus (all level-10 and level-5 cases pass on macOS ARM64) and the wrapper's application-mode run (32 accepted cases pass, 8 rejected by design). Added the v1.0 assessment: correctness is no longer blocking; the measured performance gap blocks the switch pending #96 and #100. The default is unchanged.
- 2026-11-12 — Under the owner's v1.0 direction (own ports by default, performance gaps as follow-ups), the workspace reader becomes the default whenever `jpeg-xl-own-read` is compiled in, and `cli` enables it. jxl-oxide stays an explicit backend. The 3.6× full / 9.5× crop regression and full-frame crops are recorded as the accepted trade-off, pending owner confirmation; x86-64 conformance, idle-host timing and the region API stay open. The earlier default rationale above is kept for history.
- 2026-11-12 — Indicative loaded-host re-measurement in the [v1.0 baseline](../benchmarks/v1.0-baseline-2026-11-12.md#1-jpeg-xl-decode-libjxl-rs-versus-jxl-oxide-55-93): the VarDCT flower gap reproduces (3.2× full, 8.8× crop on one thread); on lossless 3.4–24 MP photos the workspace decoder is about 17–19 % faster for a full frame but uses about 1.8× the RSS (618 versus 353 MB at 24 MP), and crops cost 2.2–2.6× jxl-oxide. No lossy 12–48 MP JXL and no C++ reference timing were available. The default is unchanged; the idle-host timing gate stays open.
- 2026-11-12 — [Performance analysis](../benchmarks/v1.0-jxl-performance-analysis-2026-11-12.md) (indicative; loaded host) with the pinned C++ libjxl built scalar (the oracle's configuration) and with SIMD: on cjxl `-e 7 -d 1` lossy files the reader is 8.1–8.2× slower than jxl-oxide (the 3.2× JPEG-reconstruction figure understates it) and about 7× the same-arithmetic scalar C++; SIMD and threads explain only a minor part of the gap. Two exact port changes landed with it: row spans resolved once per row in Gaborish, EPF, YCbCr and chroma upsampling (−20 % lossy, −25 % JPEG-reconstruction decode), and lazily zeroed default allocations, which bring lossless decode RSS to the C++ figure (24 MP: 618 → 323 MB; engine CLI 640 → 346 MB). The 1.8× RSS finding above is superseded; the time and crop gaps remain, with ranked fixes in that record.
- 2026-11-12 — The adapter retains only the requested upright region through the decoder's image-out callback: managed memory is proportional to the request (24 MP center crop 307 → 91 MB managed, 342 → 126 MB RSS; indicative). Full-frame reconstruction, `FullFrameFallback`, full-frame `CodecWork` and opaque codec memory are unchanged; `known_full_frame_buffers` is 1 only for a full-frame request. UINT8 output for 8-bit sources was evaluated and not adopted (dithered output, exact subset not identifiable through the public API, different engine path).
- 2026-11-13 — Owner decision D1, recorded in [ADR-0041](0041-v1-quality-gate-equivalence-with-the-ported-original.md): the workspace JPEG XL decoder stays the default reader. The 2026-11-12 default switch is no longer pending owner confirmation, and the comparison with jxl-oxide (time, crop cost, RSS) is context, not a criterion; jxl-oxide stays an explicit backend. The idle-host timing phase is no longer a gate for the default (D3: no performance gate). Speed is reported against the original in ADR-0041's three columns; the [JPEG XL analysis](../benchmarks/v1.0-jxl-performance-analysis-2026-11-12.md) has column (a) for AArch64. Correctness gates stay: the x86-64 conformance run, and the port's equivalence with libjxl under ADR-0041 (scalar on both targets; SIMD, #96, before 1.0). The historical rationale above is unchanged.
- 2026-11-13 — Straight alpha reaches the rendered-ICC stage (#222, ADR-0015 2026-11-13). This reader already reported association from basic info; the jxl-oxide backend now does as well (`alpha_associated == false` is straight, anything else stays unspecified and is refused). `src/jxl_alpha_tests.rs` reads the engine's own transparent lossy output back with both readers (alpha exact) and checks that a codestream rewritten to associated alpha is declared `Premultiplied` here and refused.
- 2026-11-15 — Owner decision: jxl-oxide leaves the runtime and the workspace decoder is the only JPEG XL reader; jxl-oxide remains a test-only cross-check. `jpeg-xl-own-read` is renamed `jpeg-xl-read` (working decision, delegated). Recorded in [The only reader](#the-only-reader-2026-11-15-owner-decision); the sections above are kept for history.
- 2026-11-24 — The x86-64 conformance gate is settled. The external corpus passes on hosted Linux x86-64 hardware (`ubuntu-24.04`) with the corpus's own runner, on the default AVX2 path and on the forced scalar path: 27 of 27 level-10 and 24 of 24 level-5 cases each, `original.icc` and reconstructed JPEGs identical (workflow `JXL: Conformance corpus (Linux x86-64)`, run [45343416525](https://github.com/sebastian-software/ferrisma/actions/runs/45343416525) at `01dfdc80`; an earlier run, 43690766156 at `b3f29a71`, gave the same counts). The gate named under "Gates that stay open" and in the review triggers is met. See the [record](../../codecs/libjxl-rs/docs/CONFORMANCE.md).
- 2026-11-26 — Finalized as a living record, owner decision: status Accepted; stale passages corrected in place, dropped decisions marked, open questions limited to open issues.
- 2026-11-27 — **Multithreaded decode wiring and TSan gate (issue #100).** `ferrisma-jxl-decoder` adds `decode_into_with_threads` and `decode_region_into_with_threads` (keeping `decode_into`/`decode_region_into` as single-threaded helpers). When `threads <= 1`, no runner is registered (zero overhead on single-threaded calls and batch workers). When `threads > 1`, `Decoder` creates a lazy `ThreadParallelRunner` (`jxl_thread_parallel_runner_create_lazy` in `codecs/libjxl-rs`, clamped to `256` max workers) and registers it via `jxl_decoder_set_lazy_thread_parallel_runner`, which updates the runner's active worker cap at `FrameStage::Toc` for every frame (including hidden reference frames and blending frames, lock-free and clamped to `[0, construction_max_workers]`) before section decoding begins: zero OS threads are spawned at creation time, single-group or sub-`65,536`-pixel frames (`num_groups <= 1 || frame_pixels < 65_536`, including `7×5`, `128×128` VarDCT, `320×64` 2-group VarDCT, hidden single-group reference frames, and `300×300` modular frames with `512×512` groups) cap at `0` workers, and larger multi-group frames grow the pool on demand up to `min(construction_max_workers, num_groups)` using fallible `std::thread::Builder::new().spawn`. On allocation or OS thread-creation failure, already-started workers are signalled to exit and joined, and `RunnerError::{Allocation, ThreadSpawn}` maps distinctly to `Error::Allocation` (`AllocationFailed`) and `Error::ThreadSpawn` (`ThreadSpawnFailed`). The runner is held as a field of `Decoder` dropped strictly after `Decoder::drop` calls `jxl_decoder_destroy`. `RegionSink` replaces `Cell<u64>`/`Cell<bool>` with `AtomicU64`/`AtomicBool` (`Acquire`/`Release` on `failed`, `Relaxed` `fetch_add` on `written`) and relies on `libjxl-rs`'s disjoint upright group-coordinate dispatch for race-free concurrent byte copies into the caller buffer (verified including multi-group transposing-orientation crops where `region_callback` executes 1 pixel per call on worker threads); callback panics on any worker thread are caught via `catch_unwind` and returned as `Error::Decode`. `JxlSource` implements `DecodeSource::set_decode_threads`, forwarding the pipeline's resolved `transform_threads`. `parallel_execute::run` starts its resize workers lazily on the first needed row emitted by `source.read_region`, so full-frame multithreaded decoders like `JxlSource` finish and join their decode pool before any resize worker exists (peak live OS threads stay bounded by `threads` rather than `2 × threads`). ThreadSanitizer passes with zero warnings locally on macOS ARM64 (`ferrisma-jxl-decoder --lib`, `ferrisma-libjxl --lib thread_parallel_runner_internal`, `ferrisma-libjxl --test thread_runners`, `ferrisma-libjxl --test dispatch_scope`) and is gated in CI (`.github/workflows/jxl-check.yml`, `tsan` job, also running `thread_parallel_runner_internal` and `simd_threads`).
  Small-image check (`target/release/examples/jxl_decode_threads_bench`, 100 reps, prebuilt release binary, Apple M1 Ultra): `7×5` (`tests/fixtures/sample.jxl`) stays flat at **0.08 ms** median across 1T, 8T, and 20T (`best 0.07–0.08 ms`, digest `15df4410af317070`), `128×128` VarDCT d1.0 stays flat at **0.78–0.79 ms** median across 1T, 8T, and 20T (`best 0.74 ms`, digest `d4b18bdbc9dc4692`), and the small 2-group `320×64` VarDCT d1.0 case (`small-multigroup`, 20,480 px < 65,536, 0 workers spawned) stays flat at **0.90 / 0.92 / 0.93 ms** median across 1T / 8T / 20T (`best 0.87–0.88 ms`, digest `6bf3d8343b3a4332`).
  Reproducible large multi-group release measurements (prebuilt `cargo build --release --features jpeg-xl-read,jpeg-xl-write,webp-read --example jxl_decode_threads_bench`, then `/usr/bin/time -l target/release/examples/jxl_decode_threads_bench bench ...` directly; Apple M1 Ultra 20 cores, synthetic 4096×3072 / 12.6 MP / 192 groups, median of 5 runs; indicative loaded host):

  | input & mode | 1T median ms (RSS) | 2T median ms (RSS) | 4T median ms (RSS) | 8T median ms (RSS) | 8T speedup | bit-identical digest |
  | --- | ---: | ---: | ---: | ---: | ---: | --- |
  | VarDCT d1.0 `full` (`decode_into`) | 392.2 (190.6 MiB) | 237.9 (198.1 MiB) | 124.4 (195.3 MiB) | 83.9 (210.2 MiB) | 4.67× | `2a7b4b0b908b1f4b` |
  | VarDCT d1.0 `crop` 50% (`decode_region_into`) | 424.9 (86.0 MiB) | 235.9 (93.0 MiB) | 127.5 (94.0 MiB) | 73.3 (95.8 MiB) | 5.79× | `954a0341f619a3fc` |
  | VarDCT d1.0 `pipeline-full` (`transcode_with`) | 571.0 (195.5 MiB) | 343.5 (201.2 MiB) | 248.9 (201.0 MiB) | 192.0 (210.2 MiB) | 2.97× | `dc4c9404754cae55` |
  | Lossless modular `full` (`decode_into`) | 756.5 (327.9 MiB) | 370.1 (338.4 MiB) | 216.7 (348.9 MiB) | 120.9 (370.6 MiB) | 6.25× | `28d267c713a3d608` |
  | Lossless modular `crop` 50% (`decode_region_into`) | 710.1 (219.2 MiB) | 481.0 (226.9 MiB) | 226.3 (246.6 MiB) | 116.6 (253.4 MiB) | 6.09× | `2af5042039a9ef94` |
  | Lossless modular `pipeline-full` (`transcode_with`) | 737.0 (328.3 MiB) | 517.1 (336.9 MiB) | 325.1 (349.2 MiB) | 220.5 (361.3 MiB) | 3.34× | `f4aef5bd58e28090` |
