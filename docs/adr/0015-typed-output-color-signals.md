# ADR-0015: Inspect rendered color signals before selecting a color operation

- **Status:** Accepted
- **Date:** 2026-10-31
- **Last reviewed:** 2026-11-26
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

## Current state (finalized 2026-11-26)

This record is a living record. Where the text above differs from the code on `main`, this section states the current rule.

- Out of date and corrected here: a CMS exists (moxcms, ADR-0016), so this ADR no longer excludes a CMM or an ICC transform; `ColorSignal`, `RefusalReason` and `ProcessingReport::color_signal` are public. ICC/CICP precedence and the XYB enum-sRGB question were decided by the owner on 2026-11-15 (see the amendment log). The default JPEG XL decoder is the libjxl-rs port. ICC-signalled JPEG XL fixtures exist under `tests/fixtures/parity`.
- Still not implemented: HDR reference white (PQ/HLG are refused).

### Open questions

- A cap on the ICC payload held in `Arc` storage (only AVIF has one, 16 MiB): not implemented.

## Amendment log

- 2026-10-31 — Added explicit, non-transforming color evidence classification.
- 2026-11-06 — CICP `[1, 8, 0, 1]` (sRGB primaries, linear transfer, full range) is classified as its own family, `LinearSrgb`, and is transfer-ready: it is the graph's working-space signal and what the ICC adapter now emits (ADR-0023). Limited range or other matrices with transfer 8 stay `UnknownSignal`.
- 2026-11-12 — Verified what jxl-oxide emits without a CMS and corrected its reported CICP; CICP beside a rendered ICC is checked, not ignored (#41).
- 2026-11-12 — Review of the v1.0 integration: PQ/HLG JPEG XL is refused from the transfer function, also without any CICP; AVIF `nclx` beside ICC is refused unless it agrees, with a named reason (pending owner confirmation).
- 2026-11-14 — The report now names the color signal a conversion used (`ProcessingReport::color_signal`), and the CICP-beside-ICC refusal has a machine-readable reason (`RefusalReason::CicpBesideIcc`); issue #212, designed in ADR-0017's amendment of that date. Observation only: rule 4 and the pending AVIF `nclx` outcome are unchanged.
- 2026-11-15 — The color refusals of this ADR's rules now carry a machine-readable reason (`RefusalReason::HdrTransfer`, `UnsupportedColorEncoding`, `RenderedIccMissing`, `ContradictoryColorSignals`; issue #212, ADR-0017 amendment of that date). Observation only: no rule, kind or message changed.
- 2026-11-13 — Straight alpha through the rendered-ICC stage (#222): RGBA with explicitly straight alpha converts its color channels and carries alpha unchanged; premultiplied and unspecified alpha stay refused; jxl-oxide declares straight alpha only from `alpha_associated == false`. The strict policy is unchanged; the question whether XYB-encoded enum sRGB should pass it is recorded below for the owner.
- 2026-11-15 — Owner decision (#222): a JPEG XL enum color encoding is a typed signal. Option 2 of the 2026-11-13 entry is implemented: the reader reports an enum DATA encoding with an H.273 code point as that CICP and no synthesized profile, so the strict policy reads the engine's own JPEG XL output; a file with an embedded profile still needs `UseRenderedIcc`. With it, a typed sRGB, Display P3 or linear sRGB signal without a profile takes the same route under `UseRenderedIcc` as under `Strict` (working decision, delegated, revisable by the owner). See the section of that date below.
- 2026-11-15 — Owner decision (O5, third item): AVIF with an ICC profile and an `nclx` beside it follows MIAF. The profile governs color primaries and transfer; matrix coefficients and range come from `nclx`. The adapter reports the profile alone; `CicpBesideIcc` remains for a `cicp` tag inside a profile. See the last section.

## 2026-11-12: JPEG XL emitted-sample evidence (#41)

**What jxl-oxide 0.12.6 emits under `NullCms`** (the engine's build, `default-features = false`), from `jxl-render` 0.12.4 `RenderContextBuilder::build` and `jxl-oxide` `rendered_icc`/`rendered_cicp`: an enum-signalled image is rendered in exactly that enum encoding (XYB is converted to it; non-XYB samples are passed through). An ICC-signalled non-XYB image is passed through in the embedded profile, or in an enum encoding jxl-color recognizes in it. An ICC-signalled XYB image is rendered in that recognized enum encoding or, if the profile is not recognized, in sRGB; the embedded profile is then not used. `rendered_icc` always describes the result: the embedded bytes, or an ICC synthesized from the enum (with a `cicp` tag for PQ/HLG). `rendered_cicp`, however, maps primaries without the white point, so D65 Display P3 is reported as CICP primaries 11, which H.273 defines with the DCI white point.

**Change.** The jxl-oxide adapter now derives CICP from the header's enum encoding (the renderer uses it unchanged, because the adapter never requests another encoding), maps white point and primaries together (`D65 + P3 -> 12`, `DCI + P3 -> 11`), and reports no CICP when the encoding came from an ICC profile or has no exact H.273 code. The libjxl-rs adapter reports only its rendered ICC, whose `cicp` tag carries the code points. Under the rendered-ICC policy, CICP beside the profile must agree with it (ADR-0016, 2026-11-12); the strict policy still refuses every JPEG XL input, because both adapters always describe their output with an ICC profile.

**Evidence.** No JPEG XL encoder is available on the host, so `tests/fixtures/jxl-color` respells only the enum color encoding of two independently encoded files (lossless modular and lossy XYB) with a documented header rewriter. `src/jxl_color_tests.rs` runs every check against each compiled decoder: declared signals (P3, PQ, HLG, linear sRGB, DCI-P3; the P3 case failed before the adapter change); exact known lossless samples reproduced through each decoder's ICC within `2e-3` linear of the independent oracle (P3 and linear sRGB); both decoders identical on XYB rendered into Display P3; PQ, HLG and DCI-P3 rejected under both policies (PQ was converted before the CICP check). ICC-signalled JPEG XL files are covered by the source review above only: writing a fixture needs a JPEG XL ICC-stream encoder. **The default JPEG XL decoder may change for v1.0 (#55)**; the tests take the decoder as a parameter, so the same expectations apply to either. Re-evaluate when jxl-oxide changes `cicp()` or its default output encoding, when the adapter requests an output encoding, or when ICC-signalled fixtures become available.

## 2026-11-12: review of the v1.0 integration

**PQ/HLG without a CICP code point (correction).** The entry above says PQ
and HLG are rejected, but that rejection only fired when a CICP code point
existed: the jxl-oxide adapter's `rendered_cicp`, or a `cicp` tag in the
rendered ICC. An enum colour encoding with custom primaries (and, for the
libjxl-rs backend, a white point other than D65 or DCI) has neither, so its
PQ or HLG transfer passed and `UseRenderedIcc` converted the synthesized
PQ/HLG approximation profile as SDR sRGB. Both JPEG XL adapters now refuse
PQ and HLG from the declared transfer function itself, before any profile is
declared: jxl-oxide from the header's enum encoding (and from the encoding
jxl-render recognized in an embedded ICC), libjxl-rs from the DATA encoding,
which the safe wrapper now exposes as `ImageInfo::data_transfer`. Under every
input profile policy these files are a controlled `UnsupportedColor`.
Evidence: four respelled fixtures (`tests/fixtures/jxl-color`, custom
primaries with PQ and HLG, white point E with PQ, XYB with custom primaries
and PQ) and `jxl_color_tests::hdr_transfer_is_rejected_without_any_cicp_equivalent`,
which runs both decoders under all three policies. Before the change it
failed for seven of eight decoder/fixture pairs under `UseRenderedIcc`
(the jxl-oxide adapter already rejected the white point E file through the
existing CICP check). An ICC-signalled JPEG XL file with an unrecognized PQ/HLG
profile is still covered only by the profile's own `cicp` tag; no fixture
exists for it.

**AVIF `nclx` beside an ICC `colr` (decision pending owner confirmation;
superseded by the owner's decision of 2026-11-15, see the section of that
date at the end).**
MIAF gives an ICC `colr` precedence over `nclx` for color interpretation.
Rule 4 above forbids a default precedence, so the engine keeps both signals
and applies MIAF's rule only where the two agree: under `UseRenderedIcc` an
`nclx` beside the profile must be sRGB, linear sRGB or Display P3 and must
match the profile transform on the probe colors. An sRGB profile with
`nclx` 1/13/6 therefore converts. The same profile with `nclx` 1/1/6
(BT.709 transfer) is refused rather than resolved in the profile's favor.
The error is now a dedicated `CicpBesideIcc` whose message names the
conflict and says that the ICC profile is not given precedence
automatically. The same error now covers every refusal in the CICP-beside-ICC
check, including HDR and contradicting code points. The alternative, applying
MIAF precedence and reporting that `nclx` was ignored, needs a report field
for the signal used and an owner decision; until then, refusal is the
fail-closed choice. Evidence: `tests/avif_icc_nclx.rs` with the two fixtures in
`tests/fixtures/avif-color` (provenance in its README). Strict and
`AssumeSrgbIfMissing` refuse both files, because neither treats an ICC-tagged
file as sRGB. Re-evaluate when the owner confirms or changes this outcome, or
when the report can name the color signal it used.

## 2026-11-13: straight alpha through the rendered-ICC stage (#222)

**Problem.** The engine writes lossy JPEG XL with straight alpha (ADR-0039)
but could not read such a file back with either decoder. The rendered-ICC
stage (`IccSrgbSource`, `src/cms.rs`) accepted only opaque RGB, so
`UseRenderedIcc` failed with `UnsupportedInput`; the jxl-oxide adapter
declared every alpha channel `Unspecified`; and without `UseRenderedIcc`
both adapters describe XYB output with a rendered ICC profile, which the
strict policy refuses.

**Change.** The rendered-ICC stage accepts opaque RGB and RGBA whose alpha
the decoder declares `Straight`. Only the three color channels go through
the profile transform (the same per-channel table or moxcms evaluation as
opaque RGB, bit for bit); the alpha sample bypasses it unchanged (8-bit
alpha normalized to `[0, 1]` like the color channels, float alpha copied).
The stage still emits linear-light sRGB `[1, 8, 0, 1]` with straight alpha,
which the executors associate in linear light (ADR-0014). `Premultiplied`
and `Unspecified` alpha are refused with `UnsupportedAlphaMode`: dividing
premultiplied color before a nonlinear profile curve, or guessing an
association, would be an unrecorded semantic decision. Other layouts stay
`UnsupportedCodecLayout`. The jxl-oxide adapter now reads the association
of the first alpha extra channel, the one jxl-oxide 0.12.6 renders (it
neither premultiplies nor divides): `alpha_associated == false` is
`Straight`, everything else stays `Unspecified`. The libjxl-rs adapter
already reported association from basic info (ADR-0032).

**Evidence.** `src/jxl_alpha_tests.rs`, every check with each compiled
decoder: the engine writes translucent lossy JPEG XL (9x17, 300x260 and the
streaming 800x700) and reads it back at identity geometry under
`UseRenderedIcc` on one thread and in parallel (deferred conversion).
Alpha equals the written alpha exactly; color over visible pixels lies
within this ADR's `2e-3` linear bound around each decoder's own sRGB
output (measured: at most 2 codes from the oracle encoding), and within
ADR-0039's lossy pins against the written rows (measured max / mean 17 /
3.74, 19 / 2.22, 18 / 2.11 with both decoders, the figures ADR-0039
recorded for the direct jxl-oxide decode). The test failed before the change. Rejections: the strict
policy refuses the engine's own opaque and transparent output with
`UnsupportedColor` for both decoders, unchanged; a codestream whose alpha
is rewritten to associated (a test helper rewrites only that header
bundle) is declared `Unspecified` by jxl-oxide and `Premultiplied` by
libjxl-rs and refused under every policy, under `UseRenderedIcc` as
`UnsupportedInput`. Unit tests in `cms::tests` cover the 8-bit table, 8-bit
moxcms and float paths against the opaque conversion bit for bit, alpha
bit for bit, deferral and the refusals. The former
`icc_tagged_alpha_input_is_rejected_before_output` in
`tests/resize_oracle.rs` asserted the old refusal; it is replaced by
`icc_tagged_straight_alpha_input_converts_color_and_keeps_alpha`, which
checks Display P3 WebP with alpha through both filters and both precisions
against the independent oracle: alpha equal except at exact rounding ties,
color within the existing derived `1.66e-3` linear bound scaled by
`1 / alpha` for the output division (41,814 samples, 8,396 translucent
pixels).

**Open: should XYB-encoded enum sRGB pass the strict policy?** Not
decided here; the strict policy is unchanged. The engine still needs
`UseRenderedIcc` to read its own JPEG XL output, opaque or transparent.
Options:

1. *Keep the refusal* (status quo). Fail-closed and consistent with rule 4,
   but the default policy cannot read the engine's own output.
2. *Describe enum-signalled output by its enum, not by a synthesized
   profile.* When the codestream signals an enum encoding (no embedded ICC)
   and the decoder renders into exactly that encoding, the adapter reports
   the CICP of that encoding and no `rendered_icc`; the strict policy then
   accepts exact sRGB `[1, 13, 0, 1]` (and the other ready families) as for
   any CICP-tagged input. The synthesized profile is derived from the enum,
   so it adds no independent evidence; reporting both is what turns one
   signal into the refused ICC+CICP pair. Needs: per-decoder evidence that
   the emitted samples equal the enum encoding (jxl-oxide by the source
   review of 2026-11-12 above; libjxl-rs would need its DATA primaries and
   white point exposed like `data_transfer`), a check that the strict float
   path keeps XYB's slightly out-of-range values as the rendered-ICC path
   does, and the PQ/HLG refusals kept in front.
3. *Accept XYB enum sRGB as a special case of the strict policy* without
   changing what the adapters report. Smaller, but it would make the
   policy depend on codec internals rather than on the declared signal.

Recommendation: option 2, limited to enum-signalled files, because it keeps
the rule that the adapter declares what it emits and removes the artificial
dual signal instead of adding an exception. It changes what `Strict`
accepts, so it needs the owner's decision. Re-evaluate this entry with that
decision, or if a decoder renders an enum-signalled file into another
encoding.

**Decided 2026-11-15 (owner): option 2.** See the next section.

## 2026-11-15: a JPEG XL enum encoding is a typed signal (owner decision, #222)

**Decision (owner, 2026-11-15).** A JPEG XL file whose color encoding is
signaled structurally, not as an ICC profile, and maps to a color space the
engine converts itself (sRGB, Display P3, linear sRGB: the set the AVIF path
accepts for `nclx`) is accepted under `Strict` and reported with the same
`ColorSignal` an equivalent AVIF gets (`Cicp`). A JPEG XL file with a real
embedded ICC profile still needs `UseRenderedIcc`. The engine reads its own
default JPEG XL output, lossy and lossless, with default settings. Parity
with AVIF follows ADR-0043.

**Implementation.** The reader bridge exposes which of the two the file
has, from libjxl's own API, without parsing a synthesized profile:
`ImageInfo::original_icc` is true when `JxlDecoderGetColorAsEncodedProfile`
fails for the ORIGINAL target (the codestream's `want_icc`), and
`ImageInfo::data_encoding` carries the DATA target's enum encoding (color
space, white point, primaries, transfer). The adapter maps an enum to its
H.273 code point (primaries 1, 9, 12 with D65 and 11 with the DCI white;
transfers 1, 8, 13, 16, 17, 18; identity matrix, full range) and reports it
as `rendered_cicp` without `rendered_icc`; the code point equals the `cicp`
tag libjxl writes into the profile it synthesizes for the same encoding
(`jxl_color_tests::adapters_declare_the_emitted_encoding`). An encoding
without a code point (custom primaries or white point, a gamma exponent)
keeps the synthesized DATA profile: it is exact, AVIF `nclx` cannot express
such an encoding, so it has no AVIF counterpart (format reason, ADR-0043
D1), and it still needs `UseRenderedIcc`. A file with an embedded profile
keeps its DATA profile whatever its DATA encoding (for XYB the decoder
renders into linear sRGB then), and `Discarded::IccProfile` is now keyed on
that signal. PQ and HLG stay refused in front, from the DATA transfer.

**The policies, per typed signal.** Under every policy the engine converts
a typed sRGB, Display P3 or linear sRGB signal without a profile through
its own transfer and matrix (`working_color`, extended range, so XYB's
slightly out-of-range float values are kept as on the rendered-ICC path).
*Working decision 2026-11-15 (delegated, revisable by the owner):* this
includes `UseRenderedIcc`, which permits a profile conversion and no longer
requires one for these signals; before, a signalled sRGB AVIF was refused
under it with `RenderedIccMissing`, and an enum JPEG XL file went through
the synthesized profile (`ColorSignal::RenderedIcc`). Without this, JPEG XL
and AVIF would differ under that policy (or every caller who passed
`--use-rendered-icc` to read JPEG XL would be refused). A typed signal
outside the three families is `UnsupportedColorEncoding` under `Strict` and
`AssumeSrgbIfMissing` and `RenderedIccMissing` under `UseRenderedIcc`, in
both formats (JPEG XL DCI-P3, which was `CicpBesideIcc` through the
synthesized profile's `cicp` tag, now gets the AVIF reasons).

**Evidence.** `tests/format_parity.rs`:
`the_engine_reads_its_own_jpeg_xl_output_with_default_settings` (lossy
d1.0 and lossless effort 2, read back under the default options; failed
before with `RenderedIccRequiresOptIn`), `typed_srgb_is_read_under_every_policy`,
`typed_signal_outside_the_supported_families_is_refused_alike`,
`embedded_icc_needs_the_rendered_icc_policy_in_every_format` (a cjxl file
with an embedded profile, `tests/fixtures/parity`). `jxl_alpha_tests::strict_policy_reads_engine_written_jpeg_xl`
reads translucent and opaque lossy output under `Strict` and finds the
same rows, bit for bit, as under `UseRenderedIcc`.

## 2026-11-15: AVIF ICC beside `nclx` follows MIAF (owner decision)

**Decision (owner, 2026-11-15).** An AVIF file with an ICC `colr` and an
`nclx` `colr` follows the standard (MIAF, ISO/IEC 23000-22 7.3.6.4): the
ICC profile governs color primaries and transfer; matrix coefficients and
range come from `nclx`. With `UseRenderedIcc` such a file is converted
through its profile; without the opt-in it is refused like any ICC file,
with `RenderedIccRequiresOptIn`. This supersedes the refusal recorded on
2026-11-12 (rule 4 allows a precedence that the format's standard defines).

**What was refused before, on real files.** avifenc 1.4.2 writes `nclx`
with primaries and transfer unspecified (2/2) and its default matrix 6
whenever the source has an ICC profile (`apps/avifenc.c`: it sets sRGB
code points only for a source without a profile; libavif always writes an
`nclx` beside a `prof`, `src/write.c`); libavif's own test files
`paris_icc_exif_xmp.avif` and `seine_sdr_gainmap_srgb_icc.avif` carry 2/2/6.
Those converted before (2/2 is no CICP), as did the heif-enc file with an
agreeing 1/13/6 (`RenderedIccWithAgreeingCicp`). Refused with
`CicpBesideIcc` were the heif-enc file with 1/1/6 (BT.709 transfer:
"cannot be checked") and `paris_icc_exif_xmp.avif` with its `nclx` patched
to 12/13/6 (Display P3: "disagrees with the profile transform";
`tests/fixtures/parity/icc-nclx-12-13-6.avif`).

**Implementation.** The AVIF adapter reports no rendered CICP when the item
has an ICC profile: libavif's YUV-to-RGB step has already used the `nclx`
matrix and range, and the profile describes the RGB samples. The
container-against-bitstream check (`ContradictoryColorSignals`) still runs.
All four files now convert under `UseRenderedIcc` as `RenderedIcc`; the
1/13 and 1/1 files give identical rows, as do 2/2 and 12/13
(`tests/avif_icc_nclx.rs`). The `nclx` primaries and transfer are not
reported as discarded: the standard defines them as superseded by the
profile, not as information the output leaves out (working decision
2026-11-15, delegated, revisable by the owner).

**`CicpBesideIcc` remains reachable and keeps its name**: it is the
refusal of a `cicp` tag inside the profile (checked as part of the
profile, ADR-0016) that is HDR, cannot be checked or contradicts the
profile, and of a decoder's CICP beside a profile, which no built-in reader
reports any more. Evidence: `tests/fixtures/parity/rgb-icc-cicp-p3.jpg`
(an sRGB profile with a Display P3 `cicp` tag) in
`refusal_reason_tests` and `format_parity::a_contradicting_cicp_tag_inside_the_profile_is_refused`.
`ColorSignal::RenderedIccWithAgreeingCicp` is no longer produced by a
built-in reader and is kept for one that reports CICP beside a profile.
- 2026-11-26 — Finalized as a living record, owner decision: status Accepted.
