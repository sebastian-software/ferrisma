# ADR-0016: Model a composable working-color transform boundary

- **Status:** Proposed
- **Date:** 2026-10-31
- **Last reviewed:** 2026-11-12
- **Supersedes:** None
- **Related:** [0002](0002-semantic-graph-and-planner.md), [0004](0004-buffer-ownership-and-budget.md), [0005](0005-codec-capabilities-and-pixel-contracts.md), [0014](0014-linear-srgb-straight-alpha.md), [0015](0015-typed-output-color-signals.md), [M1](../milestones/M1-complete-image-pipeline.md)

## Context

The current scalar `LinearSrgb` reference handles only explicitly tagged SDR sRGB samples and straight alpha. The typed color-signalling contract now distinguishes P3, PQ/HLG and ICC evidence but deliberately cannot execute conversions. The next several milestones need a coherent place for profile transforms, higher-quality resampling and encoder input negotiation.

Neither always-converting to RGBA8 nor making every codec emit a full-frame floating-point RGB buffer is consistent with Ferrisma's pipeline-first, memory-conscious design.

## Proposed decision

Investigate a **composable, semantic color-transform stage**, in which:

1. Each image edge carries independent descriptors of **pixel layout**, **color encoding or linear-light domain**, **alpha association**, **precision**, **residency** and **ownership**.
2. A transform node declares its **source and destination color meaning** (including profile/gamut, transfer, white point, and intent where relevant). It is not a side-effect or unchecked metadata rewrite.
3. The planner can choose small CPU working regions (e.g., `f32` or high-precision reference scalar pixels) while preserving the option of codec-native planar work and future GPU-resident kernels. A universal eager `RGBAf32` frame is **not** mandatory.
4. Resampling on color channels follows the declared working domain; when alpha is straight, it is made premultiplied **in linear light before filtering**. The planner makes linearization, association and final quantization explicit.
5. Out-of-gamut linear samples are **not silently clipped** in intermediate transforms. Gamut mapping/clamping and target output encoding are separate, visible decisions with tests and output metadata.
6. ICC and CICP conflicts do not get silently prioritized. Decoder adapters must declare the profile of their **rendered output**, and a future CMS backend must be supplied only after its license, safety and precision are audited.
7. PQ/HLG and HDR tone mapping require distinct reference-white/scene-display decisions, and stay unsupported by default through the SDR M1 milestone.

Initial implementation should start with a scalar SDR Display P3 (D65, sRGB transfer) to linear-sRGB reference, independently verified color matrices, and a minimal region-based transform contract. The first native ICC backend can then be evaluated (e.g. `moxcms`), but its API must not dictate graph semantics.

This ADR is **not Accepted** until the reference and at least one real codec's output profile are validated.

## Consequences

+ A sound foundation for multiple codecs and output color spaces without recreating eager full-frame buffers.
+ Color management and resampling become composable stages, leaving room for fusion without exposing backend details in the graph.
+ The planner can explain exact transforms and output profile changes.
- More explicit metadata and test fixtures; source metadata from an arbitrary codec may be inadequate.
- Floating-point working colors require precision, gamut and quantization policy.
- A conservative initial implementation will reject many real-world HDR and ICC workflows rather than guessing.

## Alternatives considered

- **Normalize all input to full-frame RGBA8:** simplest API, but discards precision and adds expensive copies.
- **Make `f32`/linear RGB the only internal pixel format:** elegant kernels but unnecessary conversions for format-native or copy-free steps.
- **Let the encoder/decoder decide all color handling:** prevents the planner from reasoning about transfer, work and quality.
- **Add ICC transforms as an invisible effect of `resize`:** hides semantic changes and cannot be reliably tested.
- **GPU-first universal shader IR:** speculative and unnecessary for the first working SDR pipeline.

## Validation

- Reference D65 Display P3-to-linear-sRGB matrix tests, gray/white invariance, out-of-gamut preservation and near-reversible transforms.
- Alpha association tests through conversion, filtering and encoding, including fully transparent pixels.
- Working-row/region memory and copy accounting, cancellation, and explicit output color metadata.
- Cross-check with an independent ICC/CMS oracle where profiles are supported.
- Exercise real JPEG and JPEG XL emitted data without ungrounded assumptions about sRGB.
- Reject missing/conflicting ICC/CICP and PQ/HLG when no explicit policy is supplied.

## Review when

Real codec and encoder dataflow makes the proposed format contract too expensive, if a CMS exposes a better typed interface, or when HDR and GPU residency require distinct working-space semantics.

## Amendment log

- 2026-10-31 — Proposed an explicit, region-capable working-color transform boundary for M1.
- 2026-11-12 — Independent resize oracle (#18), CICP checks beside the rendered ICC (#17, #41), and real-photograph LittleCMS evidence.

## 2026-10-31: narrow reference experiment (not acceptance)

A dependency-free scalar `src/working_color.rs` proof now decodes tagged sRGB or Display P3 (D65, sRGB transfer) samples into unclipped linear sRGB triples using the W3C CSS Color 4 rational reference matrices. A separate helper converts explicitly straight alpha into linear-light premultiplied samples. These values are not tied to a graph, decoder or full-frame buffer; ICC and PQ/HLG still fail closed. Independent unit fixtures verify D65 neutral preservation, a known out-of-gamut P3 red, extended signed transfer, alpha and profile rejection. **This does not yet implement the proposed composable graph transform or an encoder-ready color-managed path.** The ADR remains Proposed until real source and sink integration is proven.

## 2026-10-31: pipeline implementation, still under review

PR #11 experimentally connects `ToLinearSrgb → WorkingLinear(Area/Lanczos3) → ToSrgb8` with real decoder rows and an explicit AVIF frame-materialization sink. The scalar reference preserves out-of-gamut intermediate colors and alpha association until final output quantization. A contract test uncovered **nonmonotonic minimum input indices at exact Lanczos zero crossings**, requiring the scanline cache to retain any row needed by *any remaining* output row. No general planner optimization, unlimited-memory guarantee, full orientation handling or arbitrary profile conversion is implied. The ADR stays **Proposed** pending independent quality and resource validation.

### Region footprint correction

Differential testing exposed that the minimum *nonzero* tap of Lanczos3 can move backward at exact sinc zero crossings. Besides delaying row-cache eviction, the region planner must take the union of **every output sample's** filter support, not assume that the first and last output points bound all interior taps. The current reference planner now does so. For large regions, the O(number of output positions) planning overhead is an explicit optimization opportunity; a future analytical conservative bound must prove it contains the reference tap union.
- 2026-11-06 — The rendered-ICC adapter converts directly into the linear-light working space (moxcms with an identity-TRC sRGB destination), so no transfer curve is encoded and decoded again between CMS and resampler (ADR-0023). Known limitation: moxcms' matrix-shaper path clamps to `[0, 1]`, so out-of-sRGB-gamut colors are clipped before resampling, contrary to this ADR's "no premature clipping"; a test documents the clamping on a Display P3 profile. The ICC stage still runs serially in front of the row pipeline; moving it into the workers is the next step for this boundary.
- 2026-11-10 — The ICC adapter now opts into moxcms 0.9.1's floating-point extended-range matrix-shaper path (ADR-0023). Its input remains the decoder's rendered RGB ICC, and its output remains linear-light sRGB with signed values until final output quantization. The prior clamping test now requires preservation and fails on the old path. Selected samples from CC0 Display P3 and Adobe-compatible ICC files, plus independently generated variants, are checked against a LittleCMS 2.19 float transform; provenance and hashes are in `tests/fixtures/icc/README.md`. LUT-profile and intent-specific gamut behavior, P3/ICC photographs, and HDR rejection corpus remain open. This corrects the 2026-11-06 limitation without claiming that the proposed general graph transform is complete.

## 2026-11-12: independent oracle evidence for the working-linear stage

**Resize (#18).** `tests/resize_oracle.rs` compares the public facade's Area and Lanczos3 output (lossless WebP input, captured final rows, `F32` and `F64Reference`) with two oracles that share no code with `src/quality.rs` or the executors: retained ImageMagick 7.1.2-35 Q16-HDRI linear-light results and a separately written direct 2D summation (`tests/support/resize_oracle.rs`). ImageMagick's Lanczos is forced to the exact sinc; Ferrisma's Area equals ImageMagick `-scale` on downscaled axes and `-filter Triangle` on upscaled ones (a unit box integrated over unit pixels is a tent), so the fixtures apply the matching operator per axis. Cases: 2.1x and 2.2x upscale, 80x and 60x downscale, odd crops, mixed axis scales, a one-pixel column, opaque edges, a checkerboard, straight-alpha stripes, fully transparent pixels with saturated hidden color, and random alpha. Results: oracle and ImageMagick agree within `1.7e-7` linear (bound `1e-6`, derived from float32 storage); the engine matches both on all 73,119 compared RGB samples, alpha differs by one code only at exact rounding ties (asserted); a broader sweep (identity, 1.5x, 2.37x, 16x, 120x/160x, 1 x 1, anisotropic, far-corner crop, one- and two-pixel sources) matches the in-tree oracle on all 138,606 RGB samples. Every sRGB8 code round-trips at identity geometry. Tolerances are ADR-0021 rule 4, adopted unchanged. One intentional output-step difference is recorded in ADR-0014. Display P3 input through the rendered-ICC policy stays within a derived `1.66e-3` linear bound around the oracle on 108,888 samples, while an oracle that clips before filtering leaves that bound on 24,884, so out-of-gamut values provably survive until the output step. Provenance and regeneration: `tests/fixtures/resize/README.md`.

**ICC and CICP (#17, #41).** The rendered-ICC adapter no longer ignores CICP beside the profile. Decoder CICP, and a well-formed RGB `cicp` tag inside the profile, must be sRGB, linear sRGB or Display P3 and must agree with the profile transform on seven probe colors within `2e-3` linear; PQ, HLG, unknown and contradicting signals are a controlled `UnsupportedColor`. Before this change a PQ or HLG JPEG XL file was converted through the decoder's synthesized ICC as if it were SDR. Miscoded profile tags (moxcms 0.9.1 writes `[11, 13, 1, 0]` into its own Display P3 profile) are not trusted as descriptions, but a PQ/HLG transfer in them still rejects. The bounded ICC header check of ADR-0015 now runs before moxcms, which accepted a declared size beyond the data. A real photograph (the libavif Paris JPEG with its compact sRGB ICC v4 profile) and the same crop tagged as Display P3 match a LittleCMS 2.19 float transform within ADR-0023's `8e-4` on all samples (`tests/icc_photo.rs`). LUT-based profiles, intents other than relative colorimetric and a photograph corpus remain open; this ADR stays Proposed.
