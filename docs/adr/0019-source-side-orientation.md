# ADR-0019: Apply orientation as an explicit source-side materialization

- **Status:** Experimental
- **Date:** 2026-11-05
- **Last reviewed:** 2026-11-11
- **Supersedes:** None
- **Related:** [0003](0003-demand-driven-regions.md), [0004](0004-buffer-ownership-and-budget.md), [0012](0012-codec-source-contract.md), [0016](0016-working-color-space-and-transform-boundaries.md), [0017](0017-minimal-public-facade-and-cli.md), [0018](0018-optional-codec-and-cms-dependencies.md)

## Context

Decoders emit samples in *stored* order. JPEG EXIF `Orientation`, JPEG XL's header orientation and AVIF `irot`/`imir` say how those samples are displayed. Ignoring them silently produces sideways or mirrored output; until 2026-11-05 every adapter rejected non-upright input instead. Phone photos are commonly stored rotated, so rejection makes the facade impractical.

Orientation is the dihedral group of eight transforms. Flips can be streamed row by row; any 90° rotation turns output rows into source columns, so no row-streaming implementation can avoid holding the needed source region. The project's guardrails allow an explicit, observable materialization where an operation requires one (M1 plan, ADR-0004), but not a silent full-frame intermediate.

## Decision

1. **Adapters report, the engine applies.** `DecodeSource::orientation()` (default identity) returns one of the eight EXIF orientations for the emitted samples. JPEG maps its IFD0 `Orientation` 1–8; AVIF maps `irot` followed by `imir`, where `imir` axis 0 exchanges top and bottom and axis 1 exchanges left and right (ISO/IEC 23008-12:2022, as documented by libavif). Invalid values, `clap`, and an `imir` associated before `irot` still fail closed.
2. **Upright view at the source boundary.** The facade wraps a non-identity source in `OrientedSource`, which reports the upright size and identity orientation. The graph, planner and color stages only ever see an upright image, so crop rectangles are display coordinates and no graph node type is added.
3. **Explicit, region-sized materialization.** For each requested display rectangle, `OrientedSource` computes the exact stored rectangle, requests it once from the decoder, copies it into one engine-budgeted buffer and emits upright rows from it. The buffer is the stored footprint of the request, not necessarily the whole frame, and is reported as `DecodeReport::engine_buffer_bytes` and `ProcessingReport::orientation_buffer_bytes`. Upright input takes no extra path.
4. **JPEG XL is oriented by its decoder.** The default `jxl-oxide` adapter and the opt-in local libjxl-rs adapter both report upright dimensions and apply header orientation in emitted samples. Either adapter therefore reports identity orientation and needs no separate orientation buffer. The local decoder reconstructs a full frame even for crops (ADR-0032). Lossless fixtures written with the pinned libjxl encoder verify the output for all eight orientations and several crops.

## Consequences

+ EXIF-rotated photos and HEIF-style AVIF transforms work through the facade, with crops in the coordinates users see.
+ Orientation logic is codec-independent and testable without codecs; the planner and executor are unchanged.
+ Memory cost is explicit, budgeted and reported.
- Oriented input costs one extra region copy and up to one region of engine memory, even for pure flips that could stream.
- Very large rotated images without a crop need the whole stored frame in managed memory; the budget rejects them instead of degrading.
- Two mechanisms exist: engine-side for JPEG/AVIF, decoder-side for JPEG XL. The opt-in local libjxl-rs adapter re-verifies JPEG XL behavior against the same independent fixtures (ADR-0032).

## Alternatives considered

- **Orientation graph node with planner support:** more general (e.g. orientation after crop), but adds a node kind and region mapping to the planner for a transform that belongs at the source. Revisit if non-source orientation is needed.
- **Let decoders orient:** the JPEG XL decoders offer it; JPEG and AVIF adapters do not, so a universal decoder-side policy would leave their orientation unhandled.
- **Streaming flips, buffering only rotations:** saves a copy for orientations 2 and 4; deferred until measurements show it matters.
- **Keep rejecting:** safe but makes common camera images unusable.

## Validation

- Every EXIF orientation against independent grid flips/rotations, for U8 and F32 rows and every display sub-rectangle; every `irot`×`imir` combination against explicit rotate-then-mirror composition.
- Real files: eight JPEGs with inserted EXIF orientation and eight AVIFs written by ImageMagick 7.1.2/libheif 1.23.7 with `irot`/`imir`, compared with ImageMagick's `-auto-orient` rendering (mean error < 2 on distinct 4×4 color blocks); eight lossless JPEG XL files written by libjxl a16d32d with `JxlBasicInfo::orientation`, compared exactly with the transformed pattern for four crops each. See `tests/fixtures/orientation/README.md`.
- Budget exhaustion, buffer release, reported buffer bytes, out-of-bounds regions, `imir`-before-`irot` detection, and a facade test that crops an EXIF-rotated JPEG in display coordinates.

Before **Accepted**: measure the copy cost on large images; decide whether flips should stream; cover lossy and animated JPEG XL orientation.

## Review when

The JPEG XL decoder changes again; a workload shows the region copy is significant; orientation must be applied somewhere other than the source; or `clap` support is added.

## Amendment log

- 2026-11-05 — Initial decision: apply EXIF and AVIF orientation through `OrientedSource`; JPEG XL stays rejected.
- 2026-11-05 — Verified `jxl-oxide`'s own orientation and display-coordinate crops with libjxl-written fixtures; JPEG XL orientation is accepted.
- 2026-11-06 — The display-order gather copies pixels as fixed-size arrays per channel count instead of a variable-length slice copy, which compiled to one `memmove` call per pixel. The 12 Mpx EXIF-oriented phone photo's decode stage fell from 152 ms to 112 ms (ADR-0023). Buffer semantics and the reported `orientation_buffer_bytes` are unchanged.
- 2026-11-06 — With more than one transform thread and at least 64 display rows, the gather runs in blocks of 16 rows on scoped worker threads and the rows reach the sink in order on the calling thread (ADR-0023). At most twice the thread count of blocks are in flight, budgeted as engine-managed memory on top of the stored-order buffer; a budget that fits the buffer but no block fails with `BudgetExceeded`. Rows are identical for every thread count. `orientation_buffer_bytes` still reports only the stored-order buffer.
- 2026-11-11 — The opt-in local libjxl-rs decoder's default upright output matched the existing eight independent orientation fixtures for full frames and crops. The decoder's full-frame output cost is recorded in ADR-0032; jxl-oxide remains the default input backend.
