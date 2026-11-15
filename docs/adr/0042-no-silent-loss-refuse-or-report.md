# ADR-0042: No silent loss: refuse or report

- **Status:** Accepted (the principle and decisions D1–D6). The mechanisms that implement D2 are working decisions, marked as such, and the [open questions](#open-questions-pending-owner-decision) are not decided.
- **Date:** 2026-11-15
- **Last reviewed:** 2026-11-15
- **Source:** owner statement of 2026-11-15, made when he accepted the refusals recorded in the [supported matrix](../supported-matrix.md); the audit of the same day
- **Supersedes:** None
- **Related:** [0015](0015-typed-output-color-signals.md), [0017](0017-minimal-public-facade-and-cli.md), [0040](0040-stable-public-contract-and-compatibility-policy.md), [0041](0041-v1-quality-gate-equivalence-with-the-ported-original.md), [0043](0043-formats-behave-alike.md), [silent-drops audit](../silent-drops-audit-2026-11-15.md), [supported matrix](../supported-matrix.md), [compatibility](../compatibility.md)

## Context

Ferrisma refuses many inputs on purpose: animation, HDR transfer functions,
grayscale JPEG XL, premultiplied JPEG XL alpha, `clap` and others. Each
refusal is an error with a kind and, since 2026-11-15, a typed
`RefusalReason` (ADR-0017). When the owner reviewed three of these refusals
on 2026-11-15 he accepted them and stated the rule behind his acceptance:
which inputs are supported is negotiable, but nothing may disappear
implicitly, and no error may be hidden or swallowed.

A refusal satisfies that rule. What was not known was the opposite case:
what a successful `transcode` leaves out or changes without telling the
caller. The [audit](../silent-drops-audit-2026-11-15.md) of the same day
answered it for `main` at `64284177`, with 19 tests through the public API
(`tests/silent_drops.rs`):

- 25 items are discarded silently and were not documented, 29 are discarded
  silently but documented, 21 are reported to the caller, 11 are preserved.
- Two of the undocumented items deliver a wrong picture without an error. An
  AVIF whose alpha item carries an unknown property marked essential comes
  out opaque. A JPEG XL file with depth, mask, black or other extra channels,
  spot colors, or more than one alpha channel is accepted by the `jxl-oxide`
  reader, which drops the channels, composites the spot colors and keeps the
  first alpha, while the `libjxl-rs` reader refuses the same file.
- One error was misreported: an AVIF with an unknown essential property on
  its primary item was reported as malformed although it is well-formed and
  merely unsupported.
- The rest is information that the output does not carry and the result did
  not mention: gain maps, EXIF, XMP, IPTC and comments, hidden color under
  zero alpha, sample precision above what reaches the encoder, and others.

The rule existed in parts before. ADR-0015 forbids reinterpreting color
silently, and the hostile-input policy requires a controlled error instead
of a panic. No record stated it for the whole pipeline, so each adapter
decided for itself.

## Decision

The principle, in the owner's sense: **a caller can always tell what
happened to their image. The pipeline either carries a part of the input
into the output, or refuses the input, or says in the result that it left
the part out. It never does a fourth thing.**

**D1. Refuse what would make the picture wrong.** If the pipeline cannot
honor something that determines the picture the caller receives (its
samples, alpha, geometry, or the meaning of its colors), it refuses the
input with an error and a `RefusalReason`. It does not deliver a picture
without that part. This covers properties and boxes a format marks as
required for correct presentation, channels and layers that contribute to
the image, and transformations the pipeline does not apply.

**D2. Report what is left out of a correct picture.** Information that the
pipeline does not carry into the output, but whose absence leaves the
delivered picture correct under the documented contract, may be discarded.
The result then names it: every class of discarded information that
actually applied to this input is listed in the processing report, and the
supported matrix documents it. Examples: metadata (EXIF, XMP, IPTC,
comments), auxiliary images such as gain maps when the base image is read,
hidden color under zero alpha, sample precision above what the output
keeps. _Working decision 2026-11-15 (delegated, revisable by the owner):_
the mechanism is an additive field `ProcessingReport::discarded` with a
`#[non_exhaustive]` enum of classes, in the experimental tier of ADR-0040,
and a `discarded=` key on the CLI report line.

**D3. Return errors with their true class.** An error from a decoder,
encoder or color-management step is returned to the caller. It is not
replaced by a fallback result, a default, or a partial image, and it is not
reported under a kind that misstates it: unsupported is not malformed, and
exhausted resources are not a decode failure. Where a codec itself cannot
detect damage (corrupted JPEG scan data or WebP payloads that still decode),
nothing is swallowed by the pipeline; the supported matrix says so.

**D4. Readers of one format agree on what they refuse.** Where a format has
more than one reader, the readers may differ in what they support, but none
may accept a file by dropping what another refuses for the reasons of D1.
The stricter result wins.

**D5. The inventory is maintained.** The audit document is the inventory of
every discarded, changed or ignored item, with its class and the test that
proves it. A change that adds an input feature, a format, a reader or an
output updates the inventory and its tests in the same change. A test that
pins today's behavior of an item this ADR wants changed says so in its name
and comment, so that it is not read as an endorsement.

**D6. Documentation alone is not a report.** An item that is documented but
not reported at run time does not satisfy D2. It stays in the inventory as
open until the result names it or the input is refused.

This ADR does not decide which inputs are supported. Supporting grayscale
JPEG XL or premultiplied alpha, for example, remains a separate question;
this ADR only requires that an unsupported case ends as D1 says.

## Consequences

- Some files that were accepted are refused: AVIF files with an unknown
  essential property on an item the reader uses, and JPEG XL files with
  contributing extra channels on the `jxl-oxide` reader (which has since left
  the runtime). For a caller that
  relied on the old result this is a behavior change; the old result was a
  wrong picture.
- `ProcessingReport` grows a field, and the CLI report line a key. Both are
  additive under ADR-0040.
- A caller that needs strictness can check the report and reject a result
  whose `discarded` list is not empty; a caller that does not care ignores
  it. No option is needed for either.
- Adapters carry more bookkeeping: each must know what it skipped. Where a
  reader cannot tell cheaply and exactly whether something applied (for
  example whether any sample was clipped), the item stays documented and
  open under D6 until a way exists; it is not reported on a guess.
- The misreported AVIF error changed kind (`MalformedInput` to
  `UnsupportedInput`, reason `UnsupportedEssentialProperty`). A file that
  really is malformed in that box still reports `MalformedInput`.

## Alternatives considered

- **Document only.** Keep discarding silently and list everything in the
  supported matrix. Rejected by D6: a caller processing a file cannot know
  from a document whether this file had a gain map.
- **Refuse everything that is not carried over.** The strictest reading:
  an input with EXIF would be refused unless the caller opted in to dropping
  it. Rejected as the default, because nearly every photograph carries
  metadata and the delivered picture is correct without it; it remains
  possible as an opt-in (see the open questions).
- **Warnings on a log or on standard error.** Rejected: a library has no
  log the caller is sure to read, and text is not something a program can
  act on. The report is typed and belongs to the result.
- **Carry everything over.** Preserving metadata, gain maps and extra
  channels is feature work per format and output with its own color and
  privacy questions. It can be done item by item later; each item then moves
  from "reported" to "preserved" in the inventory.

## Validation

- `tests/silent_drops.rs` pins every audited item through the public API on
  real fixtures; `tests/supported_matrix.rs` pins every refusal with kind,
  message and reason.
- For D1 and D4: the two wrong-picture cases have tests that assert the
  refusal and, while JPEG XL had two readers, that both returned the same
  kind and reason for the same file. Since 2026-11-15 JPEG XL has one
  reader, and the tests pin its refusals.
- For D2: each reported class has a test with a fixture that has the thing
  and one that does not, so a class is neither missed nor reported
  unconditionally.
- This decision is falsified by any input for which a successful result
  differs from the input in a way that is neither in the report nor in the
  inventory. The fuzz lanes do not detect that; it needs the inventory to be
  kept (D5).

State on 2026-11-15: the audit, its tests and the corrected AVIF error kind
are on branch `v1/drops-batch`. Branch `v1/engine-discarded` implements D1
(an unknown essential property on any AVIF item the reader uses, alpha
items and grid cells included, is refused; the jxl-oxide reader refuses
extra channels other than one alpha), D4 for JPEG XL (both readers return
the same kind and reason for the audited files, asserted in
`tests/silent_drops.rs`), and D2 (`ProcessingReport::discarded` with eleven
classes and the CLI key `discarded=`, each proven by
`tests/discarded_report.rs` with a fixture that has it and one that does
not). D4 for JPEG XL is also met by removal: branch `v1/remove-alt-backends`
removes the jxl-oxide runtime reader (owner decision of 2026-11-15, taken for
after the libjxl-rs AVX2 speed fix, which landed in `75c31f0d`), so every
JPEG XL file has one reader and the inventory's jxl-oxide rows X6 to X9 are
resolved by removal ([ADR-0032](0032-jpeg-xl-workspace-decoder-input.md#the-only-reader-2026-11-15-owner-decision)). The [inventory](../silent-drops-audit-2026-11-15.md) moves 48 items to
"reported"; open under D6 remain the documented-only items and six
undocumented ones; among the documented ones is out-of-gamut clipping, which cannot be
detected cheaply and exactly apart from resampling overshoot and is
therefore not reported. Until the branch is on `main`, the inventory's
first column of counts is the state of `main`. Since branch `v1/engine-depth`
(owner decision of 2026-11-15: the output depth follows the input, ADR-0021
amendment) sample precision is preserved wherever the output holds it, and
`PrecisionReduced` is reported by the pipeline exactly where it does not;
the inventory moves X17, V14, O2, P1 and P6 to "preserved".

## Open questions (pending owner decision)

None: the open questions of the initial record were decided on
2026-11-15 (below).

### Decided since 2026-11-15

- **Rendering intent** (formerly open question 1): relative colorimetric,
  set explicitly; owner decision of 2026-11-15, recorded in the
  [ADR-0016 amendment](0016-working-color-space-and-transform-boundaries.md#2026-11-15-rendering-intent-is-relative-colorimetric-owner-decision).
  The report does not name it: it is a fixed part of the contract.
- **Premultiplied AVIF alpha** (formerly open question 3): accepted and
  converted, not refused; owner decision of 2026-11-15, which also makes
  premultiplied JPEG XL alpha accepted instead of refused (ADR-0043,
  decided applications). Both readers convert with their original's own
  step and the report names it (`Discarded::Unpremultiplied`); the two
  steps' arithmetic differs and is written into the supported matrix.
- **Repeated CLI flags** (formerly the open question of that name):
  refused; owner decision of 2026-11-15 ("everything else is mostly a
  silent error"). A flag given twice, or a pair of mutually exclusive
  flags, is an error naming the flags, before any input is read
  (`src/cli.rs`; inventory Q7).
- **A strict mode** (formerly the open question of that name): none for
  now; owner decision of 2026-11-15. No option turns
  `ProcessingReport::discarded` into a refusal: a caller checks the set,
  and a script checks the CLI's `discarded=` key, the last of the
  `report:` line, which is `none` when nothing was left out. An option can
  be added later without breaking anything; preserving metadata is a
  separate feature question.
- **Options without effect** (formerly the open question of that name):
  rejected where the caller set a value other than the default; owner
  decision of 2026-11-15. A thread count above 256 in a batch, an exact
  output size with the resize entry points and an AVIF depth with an
  external backend are `InvalidOptions` before decoding (inventory Q1, Q2,
  Q5); a value the report states as replaced stays where it is documented
  (Q4, Q6, valid batch thread counts). Encoder scales are native per
  format instead of aliased (ADR-0043 D4; inventory O8): WebP lossy takes
  libwebp's `method` 0–6, and the mapped `speed` is gone.

## Review when

- An input feature, format, reader or output is added (D5).
- A reported class becomes preserved, or an open item of D6 gets a way to be
  detected exactly.
- A caller reports a result that differs from its input in a way the report
  did not name.
- The owner decides one of the open questions.

## Amendment log

- 2026-11-15 — Initial record of the owner's principle and of the decisions
  derived from it after the audit of the same day.
- 2026-11-15 — Implementation of D1, D2 and D4 on `v1/engine-discarded`
  recorded in the state paragraph under Validation; clipping recorded as
  open under D6 because it cannot be detected exactly. No decision changed.
- 2026-11-15 — Open question 1 (rendering intent) decided by the owner:
  relative colorimetric, set explicitly (ADR-0016 amendment); moved to
  "Decided since 2026-11-15", the remaining questions renumbered.
- 2026-11-15 — Open question 3 (premultiplied AVIF alpha) decided by the
  owner: accept and convert in both formats (ADR-0043); moved to "Decided
  since 2026-11-15", the remaining question renumbered.
- 2026-11-15 — D4 for JPEG XL met by removal: the jxl-oxide runtime reader
  is removed (ADR-0032, owner decision), so JPEG XL has one reader; the
  state paragraph under Validation and the inventory (X6 to X9 and Q3
  resolved by removal) are updated. No decision changed.
- 2026-11-15 — Open question on repeated CLI flags decided by the owner:
  a repeated or contradictory flag is an error; implemented on
  `v1/engine-options` and moved to "Decided since 2026-11-15".
- 2026-11-15 — Open question on a strict mode decided by the owner: no
  option for now; moved to "Decided since 2026-11-15". The CLI test now
  reads `discarded=` as a script would.
- 2026-11-15 — Open question on options without effect decided by the
  owner: refused where the caller set a nondefault value; implemented on
  `v1/engine-options` (inventory Q1, Q2, Q5) and moved to "Decided since
  2026-11-15"; no question remains open.
- 2026-11-15 — The WebP part of the decision on options without effect is
  implemented with ADR-0043 D4: libwebp's own `method` replaces the mapped
  `speed` (inventory O8), and the AVIF quality is libavif's integer scale
  (O3). No decision changed.
