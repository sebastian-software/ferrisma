# ADR-0034: Resolve thumbnail geometry at the facade

- **Status:** Accepted
- **Date:** 2026-11-11
- **Last reviewed:** 2026-11-11
- **Supersedes:** None
- **Related:** [0015](0015-typed-output-color-signals.md), [0019](0019-source-side-orientation.md), [0025](0025-codec-selection-reference-conformance-and-image-scope.md)

## Context

A consumer needs an upright JPEG or WebP thumbnail without first decoding a
frame or calculating its display dimensions. The existing facade requires two
exact output dimensions, making aspect-preserving resizing and centered cover
the caller's responsibility. An explicit crop already uses upright display
coordinates. The supported end-to-end consumer is JPEG/WebP to WebP/AVIF;
this decision does not select another codec or promise a stable API.

## Decision

Keep existing exact-size calls unchanged. Add an explicit resize request with
one or two nonzero dimensions and inside, centered cover, and fill modes.
Resolve it after orientation and an optional explicit crop, before constructing
the private graph. One dimension preserves aspect ratio. Inside preserves aspect
ratio within both dimensions; cover crops the centered source rectangle and
produces the requested dimensions; fill stretches to both dimensions. With no
enlargement, aspect-preserving modes clamp the scale to one, and fill clamps
each axis independently. Report actual output dimensions.

Calculations use integer arithmetic. Derived dimensions round to nearest with
ties upward and a minimum of one pixel. Cover uses the largest integer source
rectangle within the selected crop that fits the requested aspect ratio;
its derived extent rounds down, with a minimum of one pixel. The center origin
rounds down, so an odd excess pixel is removed from the right or bottom. The
integer crop can differ slightly from the exact target aspect ratio, especially
for tiny inputs. An explicit crop is resolved first, and cover coordinates are
relative to it; all refer to upright pixels.

Offer a compressed-byte- and pixel-limited header inspection for JPEG and WebP
that does not request pixel decode. It reports the source adapter's rendered
sample metadata, not an assumed color profile. Codec allocations are not
covered by the engine budget. Inspection does not establish that the entire
image is decodable or that its color is supported for transformation.

## Alternatives and trade-offs

Require every consumer to inspect and calculate geometry: duplicates tricky
orientation, rounding, and overflow handling. Replace the existing options or
expose the graph: breaks callers or turns internal design into public API.
Fractional crop coordinates would be more exact but need a new resampling
contract; integer crop preserves the existing graph behavior. Bounds do not
guarantee bounded opaque codec scratch memory.

## Validation and review

Validate real JPEG/WebP consumer paths, upright orientation, explicit crop and
fit interaction, extremes and limits, no enlargement, and exact-size backward
compatibility. Existing color and materialization policies continue to apply.
Revisit when consumer evidence requires fractional crop coordinates, other
inspection formats, or a different rounding policy.

## Amendment log

- 2026-11-11 — Accepted the narrow thumbnail facade and header inspection for
  implementation; quality, portability, and release gates remain open.
