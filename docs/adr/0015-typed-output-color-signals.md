# ADR-0015: Inspect rendered color signals before selecting a color operation

- **Status:** Experimental
- **Date:** 2026-10-31
- **Last reviewed:** 2026-11-12
- **Supersedes:** None
- **Related:** [0005](0005-codec-capabilities-and-pixel-contracts.md), [0012](0012-codec-source-contract.md), [0013](0013-sample-domain-resize.md), [0014](0014-linear-srgb-straight-alpha.md)

## Context

The decoder contract carries raw `rendered_icc` and `rendered_cicp` values and the graph preserves both. The first linear-light reference is restricted to explicit sRGB CICP, but a byte comparison alone cannot tell the planner why Display P3, Rec.2020 PQ/HLG, ICC or conflicting metadata need different work. Worse, a decoder may already have converted its output colors, so using original-file metadata in place of the profile of **emitted samples** can silently corrupt results.

ICC profile parsing and ICC color transformation are not equivalent. A profile with a plausible 128-byte header is not automatically safe, complete, or equivalent to a named color space.

## Decision

Introduce a small, dependency-free **inspection layer**, separate from pixel conversion, the semantic image graph and codec storage.

1. Parse H.273/CICP as typed primary, transfer, matrix and Boolean range fields. Preserve unknown code points, including unsupported matrices or limited-range sRGB-like values; do not "normalize" them to sRGB.
2. Classify the exact signaling combinations currently relevant to our pipeline: RGB full-range sRGB `[1,13,0,1]`, Display P3 `[12,13,0,1]`, and BT.2100 PQ/HLG with recognized RGB or BT.2020 YCbCr matrices. These are *descriptors*, not implementations.
3. Inspect ICC data with checked lengths, the `acsp` signature, declared size and tag-table boundary; extract only header fields. **Do not claim this validates a whole ICC profile or allows any ICC conversion.** Full parsing and transform correctness belong to a future color management engine.
4. When both ICC and CICP are present, preserve both. No default precedence or equivalence is assumed; the selected decoder and its output-profile semantics must define that policy.
5. Return a typed *linear-sRGB readiness/requirement* decision: reference ready, gamut conversion needed, HDR handling needed, ICC engine needed, dual-signal resolution needed, missing metadata, unsupported signal, or malformed metadata.
6. Use that decision as the **single source of truth** for the existing strict `LinearSrgb` graph operation and executor guard. All non-ready cases remain rejected. Existing `EncodedSamples` execution is unchanged and still does not imply color fidelity.

`DecoderInfo` must describe *rendered output pixel values*, not merely original encoded tags. `DecoderInfo::rendered_color_signal` avoids copying profile payloads when inspecting. For example, if a JPEG XL adapter requests a converted output profile through a CMS, the resulting emitted pixel metadata must match that output profile.

This is an **experimental internal contract**, not a supported API, and does not bring a CMM, tone mapper, ICC transform, SIMD/GPU implementation, or new dependency.

## Consequences

+ Establishes actionable, observable reasons for color conversions without prematurely choosing a CMM or backend.
+ Prevents silent gamut/transfer errors and preserves metadata across cropped regions.
+ Distinguishes code-point classification from actual decoder work and from color-managed pixel transformation.
+ ICC inspection is bounded and does not materialize or convert pixels.
- Header-level ICC inspection cannot check all tags, profile validity, rendering intent or color-equivalence.
- The current source/graph still clone ICC payload into `Arc` storage, which must be capped and accounted for in a future resource-safety review.
- ICC/CICP precedence, arbitrary ICC, HDR reference white, out-of-gamut handling, and alpha in linear working space remain unsolved.

## Alternatives considered

- **Assume sRGB if tags are absent:** rejected; not grounded in rendered pixel semantics.
- **Treat ICC and CICP as automatically equivalent:** rejected; profile precedence depends on format and decoder configuration.
- **Implement arbitrary ICC parsing or GPU shaders now:** rejected; unrelated complexity without correctness or licensing validation.
- **Normalize all decoder output to RGBA8:** rejected; loses high-bit-depth/HDR information and increases memory traffic.

## Validation

- Characterize sRGB, P3, PQ, HLG, limited-range and unsupported matrix code points independently.
- Reject malformed CICP range flags, truncated ICC headers, invalid signatures, impossible declared sizes, and tag-table overflow.
- Verify that ICC-only and ICC+CICP cannot qualify for the scalar sRGB reference merely because their bytes appear to match a tag.
- Verify color evidence is preserved over crop and that the graph refuses unsupported gamut conversion before invoking any codec.
- Preserve tests for the sRGB linear-light reference, planar YUV, JPEG/JPEG XL source lifetimes and encoded-sample resize.

## Review when

A real decoder exposes a usable output-profile transform API; an ICC parser/CMM is selected; a reference-white and output-intent model for PQ/HLG is defined; metadata sizes cause large allocations; or evidence demonstrates that an explicitly supported profile precedence policy can be standardized per adapter.

## Amendment log

- 2026-10-31 — Added explicit, non-transforming color evidence classification.
- 2026-11-06 — CICP `[1, 8, 0, 1]` (sRGB primaries, linear transfer, full range) is classified as its own family, `LinearSrgb`, and is transfer-ready: it is the graph's working-space signal and what the ICC adapter now emits (ADR-0023). Limited range or other matrices with transfer 8 stay `UnknownSignal`.
- 2026-11-12 — Verified what jxl-oxide emits without a CMS and corrected its reported CICP; CICP beside a rendered ICC is checked, not ignored (#41).

## 2026-11-12: JPEG XL emitted-sample evidence (#41)

**What jxl-oxide 0.12.6 emits under `NullCms`** (the engine's build, `default-features = false`), from `jxl-render` 0.12.4 `RenderContextBuilder::build` and `jxl-oxide` `rendered_icc`/`rendered_cicp`: an enum-signalled image is rendered in exactly that enum encoding (XYB is converted to it; non-XYB samples are passed through). An ICC-signalled non-XYB image is passed through in the embedded profile, or in an enum encoding jxl-color recognizes in it. An ICC-signalled XYB image is rendered in that recognized enum encoding or, if the profile is not recognized, in sRGB; the embedded profile is then not used. `rendered_icc` always describes the result: the embedded bytes, or an ICC synthesized from the enum (with a `cicp` tag for PQ/HLG). `rendered_cicp`, however, maps primaries without the white point, so D65 Display P3 is reported as CICP primaries 11, which H.273 defines with the DCI white point.

**Change.** The jxl-oxide adapter now derives CICP from the header's enum encoding (the renderer uses it unchanged, because the adapter never requests another encoding), maps white point and primaries together (`D65 + P3 -> 12`, `DCI + P3 -> 11`), and reports no CICP when the encoding came from an ICC profile or has no exact H.273 code. The libjxl-rs adapter reports only its rendered ICC, whose `cicp` tag carries the code points. Under the rendered-ICC policy, CICP beside the profile must agree with it (ADR-0016, 2026-11-12); the strict policy still refuses every JPEG XL input, because both adapters always describe their output with an ICC profile.

**Evidence.** No JPEG XL encoder is available on the host, so `tests/fixtures/jxl-color` respells only the enum color encoding of two independently encoded files (lossless modular and lossy XYB) with a documented header rewriter. `src/jxl_color_tests.rs` runs every check against each compiled decoder: declared signals (P3, PQ, HLG, linear sRGB, DCI-P3; the P3 case failed before the adapter change); exact known lossless samples reproduced through each decoder's ICC within `2e-3` linear of the independent oracle (P3 and linear sRGB); both decoders identical on XYB rendered into Display P3; PQ, HLG and DCI-P3 rejected under both policies (PQ was converted before the CICP check). ICC-signalled JPEG XL files are covered by the source review above only: writing a fixture needs a JPEG XL ICC-stream encoder. **The default JPEG XL decoder may change for v1.0 (#55)**; the tests take the decoder as a parameter, so the same expectations apply to either. Re-evaluate when jxl-oxide changes `cicp()` or its default output encoding, when the adapter requests an output encoding, or when ICC-signalled fixtures become available.
