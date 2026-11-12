# ADR-0032: Read JPEG XL through the workspace decoder with a full-frame fallback

- **Status:** Experimental
- **Date:** 2026-11-11
- **Last reviewed:** 2026-11-11
- **Supersedes:** None; adds an explicit opt-in backend alongside the interim reader in [0018](0018-optional-codec-and-cms-dependencies.md)
- **Related:** [0015](0015-typed-output-color-signals.md), [0019](0019-source-side-orientation.md), [0025](0025-codec-selection-reference-conformance-and-image-scope.md), [0026](0026-codecs-in-one-repository-and-workspace.md)

## Context

The local libjxl-rs port implements the pinned reference decoder and retained C++-oracle captures. Its public decoder API has no crop or ROI request. The interim jxl-oxide adapter requests a crop; that capability cannot be transferred to this decoder. JPEG XL remains an experimental input; the full external JPEG XL conformance corpus and codec-selection measurements remain open.

## Decision

Use a separate, private `ferrisma-jxl-decoder` crate to own libjxl's raw decoder pointer and input/output lifetimes. Keep `jxl-oxide` as the default `jpeg-xl-read` backend. The new `jpeg-xl-own-read` feature compiles the local decoder; `TransformOptions::jxl_decoder = JpegXlDecoder::LibjxlRs` or `--jxl-decoder=libjxl-rs` selects it explicitly. The selection does not change when both features compile, including `--all-features`; requesting a backend whose feature is missing yields `FeatureDisabled`. The engine remains `unsafe`-free and holds only a safe slice API. Inspect basic info and the `DATA` output ICC before allocating pixels; reject animation, unknown layouts, other extra channels, float/HDR exponent channels, zero or excessive dimensions, and profiles above 16 MiB. Preserve the decoder's default upright orientation. Report the emitted F32 color samples using the `DATA` ICC, not the codestream's `ORIGINAL` profile. Report alpha association from basic info. Existing color/HDR gates decide whether those samples can enter a requested transform; no ICC header is treated as a conversion.

Each read reconstructs the full upright frame into an engine-budgeted F32 buffer, then lends cropped rows. It reports `FullFrameFallback`, one known full-frame buffer and unbounded opaque codec memory. A crop does not avoid any known codec reconstruction. Buffer allocation is preceded by the configured pixel limit and checked sample/byte arithmetic. The exact pixel buffer is charged to the shared managed budget; libjxl internal allocations remain outside that budget. Decoder destruction releases registered pointers before borrowed input and caller output storage can die. A failed or truncated decode never emits rows.

## Consequences

The reference-origin decoder can enter the experimental pipeline behind its own safety boundary. The [photo-sized comparison](../benchmarks/M2-jxl-input-switch.md) found the local decoder about 3.6× slower for full input and 9.5× slower for a center crop, so it remains opt-in while jxl-oxide stays the default. The fallback and budget failure are visible. The wrapper retains jxl-oxide as a development-only comparison dependency; the engine retains it separately for the default runtime path.

## Alternatives considered

- Switch the default immediately: rejected after the photo-sized timing regression and because ADR-0025's complete selection gates remain open.
- Pretend libjxl's output buffer accepts a crop: its API has no such parameter, and slicing a full output cannot be described as avoided reconstruction.
- Add a codec-local crop API: a separate port change requiring independent oracle and quality evidence.

## Validation

The existing eight lossless, independently libjxl-encoded orientation fixtures match upright expected RGB samples on full and cropped reads. Adapter tests compare crop samples against a full read, verify full-frame accounting and an ROI-sized budget failure, and exercise malformed header handling, grayscale and alpha association. The port's 14 retained public-decoder captures pass the Rust test and were replayed directly against the prebuilt pinned C++ oracle with byte-identical observations on 2026-11-11. The broader scoped C++ oracle remains a separate gate. The [comparison record](../benchmarks/M2-jxl-input-switch.md) measures release-mode full/crop decoding of the same input on one thread and records elapsed time plus process max RSS. The complete external conformance corpus is not yet recorded.

## Review when

Review after a full conformance run, realistic-image time/RSS comparisons, a libjxl region-decode API, a color or alpha mismatch, or a supported input that requires additional extra channels.

## Amendment log

- 2026-11-11 — Added the independent safe wrapper and explicit full-frame experimental input path. The photo-sized time/RSS comparison retained jxl-oxide as the default under ADR-0025.
