# ADR-0019: Apply orientation as an explicit source-side materialization

- **Status:** Experimental
- **Date:** 2026-11-05
- **Last reviewed:** 2026-11-12
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
- 2026-11-12 — Recorded camera-size copy costs, lossy JPEG XL orientation, channel/alpha/color preservation and the animation policy; fixed gray+alpha gathering; kept the materialization for mirror-only orientations (issue #68). Status stays Experimental.
- 2026-11-12 — Tiled gather for transposing orientations (5–8; issue #68). A display row of a rotation reads a stored *column*, so the one-row gather touched one cache line per pixel. Transposing orientations now gather 16 display rows at a time in tiles of 64 display columns, on the sequential path as on the parallel one; every sample is the one the row gather copies, only the visiting order differs, and the exhaustive orientation, crop, channel-layout and sample-type tests pass unchanged. **Intended accounting change:** the sequential managed peak of a transposing orientation is now the stored-order buffer plus one block of 16 upright rows instead of one row (+15 rows: +180,000 bytes for a 6000×4000 RGB8 rotation, +270,000 bytes for 8000×6000). `camera_size_orientation_costs_are_reported_exactly` now derives that number; mirrors and orientations without a transpose keep one row and the exact old peak. The buffer and copied bytes are unchanged. Interleaved A/B on a synthetic 6000×4000 4:2:0 JPEG to WebP 1067×1600, one transform thread, 7 repetitions, **indicative; loaded host** (load 11.5–11.8): decode stage 168.4 → 119.0 ms for orientation 6 (orientation 1: 80.1 → 80.8 ms, unchanged path), identical output bytes, CLI managed peak 102,474,468 → 102,648,468 bytes; with all cores, whose gather already held 16-row blocks and now tiles them too, the decode stage did not change (89.5 → 89.1 ms). Evidence: the [performance analysis](../benchmarks/v1.0-engine-performance-analysis-2026-11-12.md#applied-fixes-2026-11-12-evening).
- 2026-11-12 — The JPEG and WebP EXIF readers accept an Orientation tag stored as LONG as well as SHORT. Two verified corpus photographs (`mountain`, `deer`) store it as LONG and failed closed with `MalformedInput`. Other types, counts, and out-of-range values still fail.


## 2026-11-12: large, lossy and animated input (issue #68)

**Copy and materialization costs, in bytes.** A reoriented request materializes
its stored rectangle once (`orientation_buffer_bytes`) and copies every sample
twice, once into that buffer and once into upright rows
(`ProcessingReport::decoder_work.wrapper_copied_bytes`). Synthetic sources pin
the exact accounting for full 6000×4000 and 8000×6000 RGB8 frames in a mirror
and a rotation, sequential and with eight gather threads, without decoding
pixels (`camera_size_orientation_costs_are_reported_exactly`): 72,000,000 and
144,000,000 buffer bytes, twice that copied; the sequential managed peak is the
buffer plus one upright row; the parallel gather adds at most 2 × threads + 1
blocks of 16 rows. A budget one byte short of the buffer fails before decoding.

Generated 4:2:0 JPEGs (quality 90) with injected EXIF orientation, a lossy WebP
with orientation 6 and an AVIF with `irot` 3, all from one photograph,
went through the release CLI on 2026-11-12 (Apple M1 Ultra, macOS 27.0.1,
`rustc 1.100.0-nightly 2026-09-06`, one transform thread, WebP output). Counts
are exact and repeat; maximum RSS comes from `/usr/bin/time -l` on a machine
under heavy parallel load from other jobs, so it is **indicative only**:

| input | orientation | buffer bytes | copied bytes | managed peak | max RSS (indicative) |
| --- | --- | ---: | ---: | ---: | ---: |
| JPEG 6000×4000 | 1 | 0 | 0 | 21,430,112 | 13.5 MB |
| JPEG 6000×4000 | 2 (mirror) | 72,000,000 | 144,000,000 | 93,430,112 | 87.5 MB |
| JPEG 6000×4000 | 6 (rotation) | 72,000,000 | 144,000,000 | 93,202,112 | 86.3 MB |
| JPEG 8000×6000 | 1 | 0 | 0 | 30,531,312 | 20.0 MB |
| JPEG 8000×6000 | 2 | 144,000,000 | 288,000,000 | 174,531,312 | 166.1 MB |
| JPEG 8000×6000 | 6 | 144,000,000 | 288,000,000 | 174,303,312 | 166.2 MB |
| WebP 6000×4000 | 6 | 72,000,000 | 144,000,000 | 165,184,112 | 156.0 MB |
| AVIF 6000×4000 | 6 | 72,000,000 | 144,000,000 | 165,184,112 | 486.3 MB |
| JPEG 8000×6000, display crop 2000×1500 | 6 | 9,000,000 | 18,000,000 | 17,081,312 | 26.3 MB |

The managed peak includes the conservative resize-plan allowance (ADR-0004),
which is why it can exceed RSS for upright JPEG. With 20 transform threads the
oriented 24 Mpx and 48 Mpx JPEG peaks rose by 10.2–15.3 MB and 15.3–20.4 MB
(gather blocks and row-pipeline queue; upright input rose by 2.4–3.3 MB).
Source hashes and the regeneration commands are in the
[M1 evidence register](../milestones/M1-evidence-and-gates.md#2026-11-12-decoder-work-and-orientation-evidence).
No wall-clock time was recorded: the machine load makes it invalid evidence.
A clean re-measurement of time and RSS is deferred.

**Mirror-only streaming: retained materialization (decision).** Orientation 2
costs exactly as much as a rotation (the table above). A streamed horizontal
mirror would drop the buffer and one of the two copies. Orientation 4 cannot
stream from a top-down decoder: its first upright row is the last stored row,
so it needs the same buffer. Only orientation 2 would benefit. Cameras record a
rotation, not a mirror, and the project has no corpus showing orientation 2 in
practice. The materialization therefore stays for both mirrors. Re-evaluate
when a corpus or a memory-limited workload shows orientation-2 input, or when
decoders emit rows bottom-up.

**Lossy and animated JPEG XL.** Eight lossy VarDCT files with header
orientations 1–8, written by libjxl a16d32d and captured upright by it
(`codecs/libjxl-rs/tests/data/decoder-geometry/orientation-N.*`), match the
captures exactly at 8 bits through both backends. All eight carry the same
stored codestream, so each upright frame is checked exactly against the EXIF
transform of the orientation-1 frame, and three crops per orientation exactly
against slices of the upright frame. The earlier evidence was lossless only.
Animated input is rejected as unsupported by every adapter that can carry
animation; there is no first-frame policy. Before this change, the jxl-oxide
adapter rendered the first keyframe silently. `tests/animated_input.rs` covers WebP
(`img2webp` two-frame file), JPEG XL through both backends and AVIF (`avis`).
JPEG has no animation; MPF secondary images are not read.

**Crops, alpha and color after orientation.** For gray, gray+alpha, RGB and
RGBA rows, U8 and F32, all eight orientations and every display crop of a 5×3
image, each upright pixel keeps its channels together, alpha included. The
alpha mode, ICC and CICP pass through unchanged. Gray+alpha rows previously failed with
`UnsupportedCodecLayout` because the gather handled 1, 3 and 4 channels only.
No current adapter emits gray+alpha with an orientation, so the defect was
latent. Real-file crops are covered for JPEG, WebP and AVIF through the facade
(`tests/decoder_work.rs`) and for JPEG XL in the adapter tests.

Remaining before **Accepted**: a clean re-measurement of gather time and
process RSS on an unloaded host, and a lossy JPEG XL photograph at camera size
(no JPEG XL encoder for lossy output is available in the workspace).
