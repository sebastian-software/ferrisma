# ADR-0043: Formats behave alike where the format allows

- **Status:** Accepted (the principle and decisions D1–D5). The [open question](#open-questions-pending-owner-decision) is not decided.
- **Date:** 2026-11-15
- **Last reviewed:** 2026-11-15
- **Source:** owner statement of 2026-11-15, made with his decision on premultiplied alpha
- **Supersedes:** None
- **Related:** [0015](0015-typed-output-color-signals.md), [0017](0017-minimal-public-facade-and-cli.md), [0040](0040-stable-public-contract-and-compatibility-policy.md), [0042](0042-no-silent-loss-refuse-or-report.md), [supported matrix](../supported-matrix.md), [silent-drops audit](../silent-drops-audit-2026-11-15.md)

## Context

Ferrisma reads JPEG, WebP, JPEG XL and AVIF and writes AVIF, WebP and JPEG
XL through one facade. Each format is read by its own adapter, and each
adapter was written against its own decoder. Where the decoders differ in
what they hand over, the adapters came to differ in what the engine does
with the same property of an image, without a reason in the format itself:

- **Premultiplied alpha.** AVIF with premultiplied alpha was accepted and
  un-premultiplied (silently, see ADR-0042); JPEG XL with premultiplied
  alpha was refused.
- **Grayscale.** A grayscale JPEG is accepted and expanded to RGB; a
  grayscale JPEG XL and a monochrome AVIF are refused, although all three
  formats represent gray and both JPEG XL readers deliver it.
- **Error vocabulary.** Until 2026-11-15 the same cause could arrive under
  different kinds per format, and the two JPEG XL readers disagreed on
  which files they refuse (ADR-0042, D3 and D4).

A caller who processes a mixed set of files cannot be expected to know
these differences. When the owner decided the premultiplied-alpha question
on 2026-11-15 (both formats accept it and convert it automatically, since
it is not an error in the file), he stated the rule behind it: an important
principle of working with this library is that the formats behave
identically, as far as a format can represent the thing in question.

## Decision

The principle: **the same property of an image leads to the same outcome
whichever format carries it, unless the format cannot represent the
property or its standard defines it differently.**

**D1. Same outcome class.** For a property that several formats can carry
(alpha and its association, gray or color layout, bit depth, orientation
and geometric transforms, color signaling, animation, auxiliary images,
metadata), the engine does the same thing in every format: it accepts and
carries it, accepts and converts it, refuses it, or discards and reports it
(the classes of ADR-0042). A difference between formats needs a reason in
the format or its standard, and that reason is written next to the
difference in the supported matrix.

**D2. Same conversion.** Where a property is accepted through a conversion
(un-premultiplying alpha, expanding gray to RGB, applying an orientation),
it is the same operation for every format: the same arithmetic at the same
stage and precision of the pipeline, and the same entry in the processing
report. An adapter does not apply its decoder's own variant when the
pipeline has one.

**D3. Same words for the same cause.** The same cause is reported with the
same error kind and the same `RefusalReason` in every format, and the same
discarded class in the report (ADR-0042, D2 and D3).

**D4. What this does not cover: encoder controls.** How hard an encoder
works and how it trades size against quality is specific to the encoder.
Those settings are not made alike by mapping one scale onto all encoders:
each output has its encoder's own scales (owner decision of 2026-11-15 on
the WebP speed aliases), and a setting that belongs to another format is
refused. Parity applies to what an image means and to what happens to it,
not to tuning.

**D5. The matrix is where parity is checked.** Each row of the supported
matrix describes one property across the formats. A row whose cells differ
names the format reason or is a defect. Properties that several formats
share get tests that run the same case through every format that can carry
it and assert the same outcome class, kind, reason and report entry.

### Decided applications

- **Premultiplied alpha** (owner decision, 2026-11-15): AVIF and JPEG XL
  both accept it, the pipeline converts it to straight alpha, and the
  report names the conversion. JPEG XL uses the un-premultiply step of the
  ported original, so the port stays identical with it (ADR-0041). The
  earlier refusal for JPEG XL is replaced. It counts as supported once a
  fixture exists for each format.
- **JPEG XL enum color encoding is a typed signal** (owner decision,
  2026-11-15, #222; ADR-0015 section of that date): an enum encoding with
  an H.273 code point is reported as CICP, like AVIF `nclx`, so both get the
  same outcome, `ColorSignal` and `RefusalReason` under every policy; an
  embedded profile needs `UseRenderedIcc` in every format. An enum encoding
  without a code point (custom primaries or white point, gamma) has no
  AVIF counterpart: `nclx` cannot express it (format reason).

## Consequences

- Some refusals become conversions (premultiplied JPEG XL alpha). Where the
  open question below is answered in favor of accepting, grayscale JPEG XL
  and monochrome AVIF follow.
- Adapters lose freedom: a new reader or format has to be fitted to the
  existing outcome for each shared property, or its difference justified.
- The supported matrix becomes a checked artifact, and the parity tests
  grow with every shared property.
- Parity can be bought only where it is correct. A conversion that is exact
  in one format and lossy in another is not the same operation; such a case
  is a format reason under D1 and is written down, not hidden behind equal
  behavior.

## Alternatives considered

- **Each adapter follows its decoder.** The state before this ADR. Simple
  to build; the caller pays with behavior that depends on the container.
- **The lowest common denominator: refuse whatever one format's adapter
  cannot do.** Uniform, but it would refuse valid files that every other
  tool reads (grayscale JPEG, for example).
- **One unified scale for every encoder setting.** Rejected by D4: the
  WebP speed scale showed that a mapped scale produces values without an
  effect of their own, and comparable libraries expose each encoder's
  scale.

## Validation

- The parity tests of D5: one table of shared properties, each run through
  every format that carries it, asserting the same outcome.
- A review of every row of the supported matrix against D1, recorded in the
  matrix (format reason or defect) and repeated when a format, reader or
  output is added.
- This decision is falsified by two files in different formats that carry
  the same property and get different outcomes without a format reason in
  the matrix.

State on 2026-11-15: the principle is recorded; the premultiplied-alpha
change, the parity tests and the matrix review are not implemented yet.

## Open questions (pending owner decision)

1. **Grayscale.** JPEG expands gray to RGB; JPEG XL and AVIF refuse it.
   Under D1 this difference has no format reason. Options: accept gray in
   all three and expand it to RGB (or RGBA with alpha), which loses nothing;
   refuse it in all three; or keep the difference and record why. A gray
   image may carry a gray ICC profile, which must enter color handling
   correctly or be refused with a reason, not guessed.

## Review when

- A format, reader or output is added.
- A row of the supported matrix gets cells that differ.
- A caller reports two files with the same property and different outcomes.

## Amendment log

- 2026-11-15 — Initial record of the owner's principle, with the
  premultiplied-alpha decision as its first application.
- 2026-11-15 — The JPEG XL enum encoding decision (#222) added as an
  application; `tests/format_parity.rs` started (D5) with color
  signaling, straight alpha, orientation and animation.
