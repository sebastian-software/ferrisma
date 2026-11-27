# ADR-0019: Apply orientation as an explicit source-side materialization

- **Status:** Accepted
- **Date:** 2026-11-05
- **Last reviewed:** 2026-11-27
- **Supersedes:** None
- **Related:** [0003](0003-demand-driven-regions.md), [0004](0004-buffer-ownership-and-budget.md), [0012](0012-codec-source-contract.md), [0016](0016-working-color-space-and-transform-boundaries.md), [0017](0017-minimal-public-facade-and-cli.md), [0018](0018-optional-codec-and-cms-dependencies.md)

## Context

Decoders emit samples in *stored* order. JPEG EXIF `Orientation`, JPEG XL's header orientation and AVIF `irot`/`imir` say how those samples are displayed. Ignoring them silently produces sideways or mirrored output; until 2026-11-05 every adapter rejected non-upright input instead. Phone photos are commonly stored rotated, so rejection makes the facade impractical.

Orientation is the dihedral group of eight transforms. Flips can be streamed row by row; any 90° rotation turns output rows into source columns, so no row-streaming implementation can avoid holding the needed source region. The project's guardrails allow an explicit, observable materialization where an operation requires one (M1 plan, ADR-0004), but not a silent full-frame intermediate.

## Decision

1. **Adapters report, the engine applies.** `DecodeSource::orientation()` (default identity) returns one of the eight EXIF orientations for the emitted samples. JPEG maps its IFD0 `Orientation` 1–8; AVIF maps `irot` followed by `imir`, where `imir` axis 0 exchanges top and bottom and axis 1 exchanges left and right (ISO/IEC 23008-12:2022, as documented by libavif). Invalid values, `clap`, and an `imir` associated before `irot` still fail closed.
2. **Upright view at the source boundary.** The facade wraps a non-identity source in `OrientedSource`, which reports the upright size and identity orientation. The graph, planner and color stages only ever see an upright image, so crop rectangles are display coordinates and no graph node type is added.
3. **Explicit, region-sized materialization.** For each requested display rectangle, `OrientedSource` computes the exact stored rectangle, requests it once from the decoder, copies it into one engine-budgeted buffer and emits upright rows from it. The buffer is the stored footprint of the request, not necessarily the whole frame, and is reported as `DecodeReport::engine_buffer_bytes` and `ProcessingReport::orientation_buffer_bytes`. Upright input takes no extra path.
4. **JPEG XL is oriented by its decoder.** The workspace libjxl-rs adapter, the only JPEG XL reader (ADR-0032), reports upright dimensions and applies header orientation in emitted samples, so it reports identity orientation and needs no separate orientation buffer. It reconstructs a full frame even for crops (ADR-0032). Lossless fixtures written with the pinned libjxl encoder verify the output for all eight orientations and several crops.

## Consequences

+ EXIF-rotated photos and HEIF-style AVIF transforms work through the facade, with crops in the coordinates users see.
+ Orientation logic is codec-independent and testable without codecs; the planner and executor are unchanged.
+ Memory cost is explicit, budgeted and reported.
- Oriented input costs one extra region copy and up to one region of engine memory, even for pure flips that could stream.
- Very large rotated images without a crop need the whole stored frame in managed memory; the budget rejects them instead of degrading.
- Two mechanisms exist: engine-side for JPEG/AVIF, decoder-side for JPEG XL. The libjxl-rs adapter re-verifies JPEG XL behavior against the same independent fixtures (ADR-0032).

## Alternatives considered

- **Orientation graph node with planner support:** more general (e.g. orientation after crop), but adds a node kind and region mapping to the planner for a transform that belongs at the source. Revisit if non-source orientation is needed.
- **Let decoders orient:** the JPEG XL decoders offer it; JPEG and AVIF adapters do not, so a universal decoder-side policy would leave their orientation unhandled.
- **Streaming flips, buffering only rotations:** saves a copy for orientations 2 and 4; deferred until measurements show it matters.
- **Keep rejecting:** safe but makes common camera images unusable.

## Validation

- Every EXIF orientation against independent grid flips/rotations, for U8 and F32 rows and every display sub-rectangle; every `irot`×`imir` combination against explicit rotate-then-mirror composition.
- Real files: eight JPEGs with inserted EXIF orientation and eight AVIFs written by ImageMagick 7.1.2/libheif 1.23.7 with `irot`/`imir`, compared with ImageMagick's `-auto-orient` rendering (mean error < 2 on distinct 4×4 color blocks); eight lossless JPEG XL files written by libjxl a16d32d with `JxlBasicInfo::orientation`, compared exactly with the transformed pattern for four crops each. See `tests/fixtures/orientation/README.md`.
- Budget exhaustion, buffer release, reported buffer bytes, out-of-bounds regions, `imir`-before-`irot` detection, and a facade test that crops an EXIF-rotated JPEG in display coordinates.

Done: copy cost measured on large images, flips keep the materialization, lossy JPEG XL orientation covered and animated JPEG XL refused. A clean re-measurement on an unloaded host has not been made.

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
- 2026-11-26 — Finalized as a living record, owner decision: status Accepted; stale passages corrected in place, dropped decisions marked, open questions limited to open issues.
- 2026-11-27 — Streaming EXIF orientation 2 (`flip_x` only) in O(1 row) memory, fast whole-row copy/reverse for orientations 3 and 4, hoisted-bounds transpose for orientations 5–8, and zero-oversubscription thread partitioning with `parallel_execute` (issue #68; reverses the 2026-11-12 decision to retain full-region materialization for orientation 2; see the section below).

## 2026-11-27: streaming orientation 2, fast non-transpose gather, thread partitioning, and 24–48 MP A/B measurements (issue #68)

**Revised decision — orientation 2 streams row by row for row-streaming sources above one scanline.** On 2026-11-12 the record retained full-region materialization for mirror-only orientation 2 (`flip_x` only) because it had not yet been justified by workload evidence. On 24 MP and 48 MP RGB8 JPEG inputs, orientation 2 previously allocated 72 MB and 144 MB of engine-managed memory and copied the full frame twice — identical to a 90° rotation despite requiring no vertical reordering. `OrientedSource` now streams orientation 2 row by row through a single budgeted row buffer (`O(1 row)` memory: 18 KB at 24 MP, 24 KB at 48 MP) whenever the underlying decoder reports `DecodeSource::streams_rows() == true` (currently scanline JPEG) **and** the requested stored region exceeds one full-width source scanline (`stored.width * stored.height > source.info().size.width`), reversing each incoming row in place and emitting it immediately. Accounting reflects the actual work: `orientation_buffer_bytes = 0` (no stored-region buffer) and `wrapper_copied_bytes = 1 × region_bytes` (one copy instead of two). Tiny crops whose region is no larger than one source scanline (e.g. a `1×1` crop of a JPEG) and frame-backed decoders (`streams_rows() == false`, i.e., WebP, AVIF, and JPEG XL — including lossy WebP, which holds a multi-row decoded frame in `MemoryBudget` across `sink` calls) retain the region-buffered copy (using the fast fixed-channel row reverse) so the decoder releases its scanline or decoded frame before downstream resize buffers are allocated, avoiding overlapping decode + resize peak budgets on small crops. Orientations 3 (`180°`) and 4 (`flip_y`) still require the stored region because top-down decoders emit the bottom display row last, so they retain full-region materialization.

**Fast non-transpose gather and hoisted transpose bounds.** Orientation 4 (`flip_y`) now copies whole stored rows with `copy_from_slice`; orientation 3 (`180°`) reverses whole stored rows with fixed-channel `as_chunks::<N>()` reverse iteration. Transposing orientations (5–8) keep the 16×64 cache-tiled gather with per-row bounds checks hoisted once per tile span instead of per-pixel `stored_point`/`try_from`/`get` lookups.

**Eliminating nested thread-pool oversubscription.** Previously `OrientedSource::with_threads(transform_threads)` spawned `N` gather threads with a `2 * N`-block (`32 * N` rows) queue *inside* `parallel_execute::run`'s `N` worker threads (2`N` concurrent threads and 10–20 MB of extra gather blocks). Now:
- Non-transposing orientations (3 and 4) never spawn parallel gather threads (single-row sequential copy/reverse keeps managed peak at `frame + 1 row`).
- For transposing orientations (5–8), `emit_parallel` drops each emitting block before queuing the next job so at most `threads + 1` blocks are ever live simultaneously. When `parallel_execute::run` runs with `used_threads > 1`, `fast_execute::partition_transform_threads` only splits off gather threads when `OrientedSource` actually spawns a pool (`source_height >= PARALLEL_MIN_ROWS` and `used_threads >= 6`: `gather = (used_threads / 3).clamp(2, 4)`, `workers = used_threads - gather`, so `gather + workers == used_threads`); whenever gathering stays serial (`gather == 1`), all `used_threads` go to `parallel_execute`. `QualityReport::transform_threads` reports the actual row-pipeline `worker_threads` spawned. `1:1` identity pass-through runs without `parallel_execute` workers and retains the full gather thread allowance.

**24 MP and 48 MP before/after measurements.** Interleaved A/B (5 repetitions each, median reported, `/usr/bin/time -l` maximum RSS) comparing `503007eb` (before) vs `perf/epic-287-icc-and-orientation` (after) on Apple M1 Ultra (20 cores, 64 GiB RAM, macOS 27.2.1, `rustc 1.100.0-nightly 2026-09-06`, load average 9.0–11.0 during run — indicative under background load; interleaved A/B pairs control for drift). Inputs: deterministic `magick -seed 42 -size WxH plasma:fractal -strip -quality 90 -sampling-factor 4:2:0` with identical JPEG payloads and minimal 36-byte EXIF APP1 orientation tags (`24mp-o1`: 4,322,110 B `sha256:4b5c68fe2138a060`; `24mp-o2`: `0cc9a3cc03d9fcbf`; `24mp-o3`: `e401d492cb2872f7`; `24mp-o4`: `eab4b41f17fc0f95`; `24mp-o6`: `55f3589f81aa5c21`; `48mp-o1`: 8,019,996 B `bfbd0945d334ab90`; `48mp-o2`: `eff1f3d6a9bc8f82`; `48mp-o3`: `ca71b292510adce8`; `48mp-o4`: `3a19b9b32f230f0b`; `48mp-o6`: `3a4265b3331081fd`), transcoded to WebP (`--webp-method=0`, `1600×1067` / `1067×1600` for 24 MP, `1600×1200` / `1200×1600` for 48 MP). Every case produced **byte-identical** WebP output between before and after (`EQ`):

| case | decode ms (B → A) | transform ms (B → A) | total ms (B → A) | orient buf MB (B → A) | managed peak MB (B → A) | max RSS MB (B → A) |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| 24 MP o1, t=1 | 69.91 → 69.47 | 79.83 → 78.82 | 188.67 → 187.66 | 0.00 → 0.00 | 30.12 → 30.12 | 27.79 → 28.05 |
| 24 MP o2, t=1 | 99.30 → 78.64 | 79.48 → 79.31 | 217.27 → 198.04 | 72.00 → 0.00 | 102.12 → 30.13 | 100.01 → 28.18 |
| 24 MP o3, t=1 | 97.29 → 85.69 | 79.26 → 79.04 | 213.81 → 205.01 | 72.00 → 72.00 | 102.12 → 102.12 | 100.01 → 100.29 |
| 24 MP o4, t=1 | 97.21 → 79.57 | 78.12 → 78.00 | 213.01 → 196.62 | 72.00 → 72.00 | 102.12 → 102.12 | 99.89 → 99.83 |
| 24 MP o6, t=1 | 107.99 → 101.57 | 81.14 → 79.25 | 227.53 → 220.10 | 72.00 → 72.00 | 102.05 → 102.05 | 100.07 → 99.78 |
| 24 MP o1, t=8 | 70.14 → 70.06 | 11.68 → 11.44 | 119.93 → 121.02 | 0.00 → 0.00 | 31.64 → 31.74 | 30.67 → 30.16 |
| 24 MP o2, t=8 | 80.50 → 78.37 | 15.66 → 10.99 | 134.56 → 128.85 | 72.00 → 0.00 | 109.34 → 31.63 | 114.88 → 30.51 |
| 24 MP o3, t=8 | 78.23 → 87.66 | 15.48 → 13.26 | 131.47 → 141.36 | 72.00 → 72.00 | 109.19 → 103.91 | 114.54 → 102.76 |
| 24 MP o4, t=8 | 79.32 → 80.43 | 15.41 → 13.10 | 132.89 → 132.79 | 72.00 → 72.00 | 109.00 → 104.24 | 114.39 → 103.38 |
| 24 MP o6, t=8 | 83.29 → 83.60 | 24.02 → 23.39 | 147.43 → 148.20 | 72.00 → 72.00 | 106.60 → 103.66 | 110.17 → 104.89 |
| 48 MP o1, t=1 | 134.71 → 135.79 | 129.53 → 131.36 | 306.24 → 312.04 | 0.00 → 0.00 | 37.32 → 37.32 | 33.90 → 33.78 |
| 48 MP o2, t=1 | 192.43 → 152.82 | 129.64 → 129.80 | 364.40 → 327.93 | 144.00 → 0.00 | 181.32 → 37.35 | 177.54 → 33.93 |
| 48 MP o3, t=1 | 198.50 → 179.86 | 133.69 → 131.86 | 374.67 → 356.93 | 144.00 → 144.00 | 181.32 → 181.32 | 177.85 → 177.65 |
| 48 MP o4, t=1 | 202.96 → 170.70 | 135.20 → 133.80 | 381.13 → 350.94 | 144.00 → 144.00 | 181.32 → 181.32 | 179.06 → 178.13 |
| 48 MP o6, t=1 | 215.45 → 203.77 | 136.29 → 134.59 | 395.09 → 384.39 | 144.00 → 144.00 | 181.36 → 181.36 | 178.08 → 178.13 |
| 48 MP o1, t=8 | 138.18 → 138.44 | 18.47 → 18.94 | 199.48 → 202.26 | 0.00 → 0.00 | 39.24 → 39.24 | 37.18 → 36.90 |
| 48 MP o2, t=8 | 158.81 → 155.17 | 29.07 → 19.66 | 232.24 → 220.27 | 144.00 → 0.00 | 190.57 → 39.24 | 197.98 → 36.83 |
| 48 MP o3, t=8 | 156.07 → 171.52 | 25.96 → 20.61 | 224.86 → 237.22 | 144.00 → 144.00 | 190.52 → 183.46 | 199.10 → 181.09 |
| 48 MP o4, t=8 | 152.55 → 157.92 | 26.24 → 20.58 | 220.67 → 223.34 | 144.00 → 144.00 | 190.37 → 183.69 | 197.75 → 181.17 |
| 48 MP o6, t=8 | 153.39 → 152.07 | 33.20 → 38.24 | 229.84 → 235.13 | 144.00 → 144.00 | 188.12 → 183.63 | 205.60 → 193.17 |

At 20 threads (`t=20`, all M1 Ultra cores), orientation 6 managed peak dropped from 112.77 → 105.83 MB (RSS 118.75 → 107.54 MB) at 24 MP and from 196.97 → 186.17 MB (RSS 228.64 → 202.95 MB, −25.7 MB RSS) at 48 MP while eliminating 20 oversubscribed threads. Validation: `orientation::tests::{orientation_2_streams_in_one_row_budget_and_cancels_cleanly, orientations_3_and_4_match_reference_and_avoid_parallel_gather_blocks, camera_size_orientation_costs_are_reported_exactly}` and `examples/icc_orientation_bench.rs`.
