# ADR-0012: Bind real codecs through a capability-reporting source contract

- **Status:** Experimental
- **Date:** 2026-10-31
- **Last reviewed:** 2026-11-12
- **Supersedes:** None
- **Related:** [0002](0002-semantic-graph-and-planner.md), [0003](0003-demand-driven-regions.md), [0004](0004-buffer-ownership-and-budget.md), [0005](0005-codec-capabilities-and-pixel-contracts.md), [0010](0010-dual-licensing.md), [0013](0013-sample-domain-resize.md), [0016](0016-working-color-space-and-transform-boundaries.md), [0018](0018-optional-codec-and-cms-dependencies.md)

## Context

Synthetic sources prove region propagation but not the integration boundary with real codecs. Codecs differ in supported image layouts, metadata, buffer lifetimes, memory use and their ability to honor region requests. A crop API alone does not imply work was saved.

## Decision

Experiment with a **private** `DecodeSource` contract, independent of the graph and executor. Its reported capabilities include image dimensions, channel layout, emitted sample type, rendered color metadata, and an explicit access class:

- **FullFrameFallback:** the adapter must reconstruct the entire image before exposing the requested region.
- **CodecRegionRequest:** the adapter passes an ROI to the codec API; this makes **no guarantee** about skipped entropy decode, coefficient reconstruction, decoder allocations, or compute cost.

The existing graph planner maps requested output rectangles backwards through crop nodes. A codec source node stores no encoded bytes, decoder handles, or borrowed pixel memory. The source is bound at execution time and sends ephemeral borrowed scanlines through a callback sink; callback errors propagate as cancellation/backpressure.

Two initial **read-only** adapters validate the contract:

1. **JPEG** via optional `zune-jpeg` 0.5.15: header probe, RGB8 output, optional embedded ICC. Full RGB output is decoded directly into an engine-budgeted buffer; requested crop rows borrow from that buffer. Disable architecture-specific unsafe-intrinsic features for this first experiment. This does **not** create a JPEG encoder.
2. **JPEG XL** defaults to optional `jxl-oxide` 0.12.6: request a crop using the codec's ROI API; stream f32 samples via a single budgeted row buffer. Retain rendered ICC/CICP metadata. [ADR-0032](0032-jpeg-xl-workspace-decoder-input.md) adds an explicitly selected full-frame reference-port backend.

The graph currently carries sample layout, but **not** a complete graph-owned color/profile contract. Only crop and sink are permitted for these sources until output semantics for resize, color and alpha are properly designed. *(Amended 2026-11-05: resize and color stages are now admitted under ADR-0013, ADR-0014 and ADR-0016; see below.)*

## Memory and safety

The engine's RAII `MemoryBudget` covers allocations it creates. The original JPEG adapter charged a RGB8 frame; its replacement charges the scanline described in [the JPEG amendment below](#amendment-log). The default JPEG XL adapter charges one F32 row, while the workspace decoder charged the full F32 frame until 2026-11-12 and now charges only the requested region's F32 samples, although it still reconstructs the full frame ([ADR-0032](0032-jpeg-xl-workspace-decoder-input.md#region-sized-retained-buffer-2026-11-12)). The budget **does not cap or measure** codec-internal scratch, retained compressed data, metadata vectors, or whole-process RSS. A report with zero *known* full-frame buffers is not evidence that a codec created none internally.

The Ferrisma crate continues to forbid `unsafe` locally. Do not equate that with a guarantee that all dependencies avoid unsafe code, or that arbitrary malicious inputs have been hardened and fuzzed.

Both selected adapters are permissively licensed. Do not integrate AGPL/commercial AVIF libraries into the MIT OR Apache-2.0 package without an explicit license decision and dependency audit.

## Consequences

+ The graph stays codec-independent while the planner can observe known fallback costs.
+ JPEG read support is practical without optimizing the entire engine around it.
+ JPEG XL's float and region capabilities can be tested without forced RGB8 quantization.
- The original JPEG adapter decoded a full frame for every crop; the 2026-11-11 workspace scanline amendment below replaces that behavior.
- JPEG XL may perform opaque allocations or whole-image work despite the default backend's ROI API. The opt-in workspace decoder reports an explicit full-frame fallback (ADR-0032).
- Color metadata is source-side rather than a full semantic graph contract.
- No encoder, frame reuse, animation, or arbitrary untrusted-image hardening yet.

## Alternatives

- **Universal RGBA8 decoder:** straightforward but forces color conversions and loses HDR precision.
- **Treat every cropped output as partial decode:** misleading and impossible for many real files.
- **Codec-specific graph nodes:** couples graph semantics to arbitrary backend APIs.
- **Add every codec first:** makes it difficult to isolate the architectural problems.

## Validation

Exercise real small JPEG and JPEG XL fixtures, invalid input, region geometry, row typing, source binding, early cancellation and budget failures in feature-gated tests. Compare cropped pixels against full-frame decodes, with a documented numerical tolerance for floating-point output. CI must test the no-codec configuration and both optional codec adapters. Inspect codec and dependency licenses separately.

Before promotion to Accepted: validate actual codec work avoided, orientation/ICC/CMYK/HDR handling, adversarial dimensions and malformed files, memory usage inside decoders, output fidelity and a format-independent transform path.

## Review when

A source requires native planar output, push/pull access rather than callback rows, precise codec allocation accounting, or more complete color semantics to avoid incorrect transformations.

## Amendment log

- 2026-10-31 — Initial experimental contract with JPEG read-only fallback and JPEG XL region requests.
- 2026-11-05 — Admitted resize/color consumers, added the AVIF adapter, and required adapters to report rendered color before decoding pixels.
- 2026-11-11 — Replaced the JPEG full-frame adapter with the workspace libjpeg-turbo-rs safe scanline API. The JPEG-specific fallback and memory statements above are historical; the current behavior is recorded below.
- 2026-11-11 — [ADR-0031](0031-workspace-libavif-input-experiment.md) replaces the 2026-11-05 `avif-decode` implementation described below with an isolated safe wrapper over workspace libavif/AOM. The full-frame source contract, rendered-color checks, and orientation semantics remain; the managed RGB frame and native/nightly boundary are now explicit.
- 2026-11-12 — Every decode report carries the codec request, the codec output and lower bounds of codec work; the JPEG tail validation was repaired; jxl-oxide rejects animation. See the section below.
- 2026-11-12 — The default JXL-Oxide adapter rejects an enum header colour encoding that jxl-color 0.11 cannot synthesize into a rendered ICC profile (unknown or XYB colour space, unknown transfer function, zero gamma) with `UnsupportedColorEncoding` before any accessor synthesizes a profile or renders pixels. jxl-color panics on those encodings; a default Linux ASan campaign ([run 43117625746](https://github.com/sebastian-software/ferrisma/actions/runs/43117625746)) reached it from public `transcode`. The minimized input is retained in `tests/fixtures/hostile/` with adapter and public-facade regressions. Embedded ICC profiles are unchanged: jxl-render parses them or falls back to sRGB without synthesis. "Describe before decoding" still holds, but JXL-Oxide buffers frame data during construction; the engine pixel limit applies after the header is parsed and before rendering.
- 2026-11-12 — The workspace JPEG XL adapter retains only the requested region through the decoder's image-out callback; it still reports `FullFrameFallback` and full-frame codec work, and counts a known full-frame buffer only for a full-frame request (ADR-0032).

## 2026-11-05: transform consumers and the AVIF adapter

The crop-and-sink restriction above no longer holds. Decoder sources now feed the encoded-sample resize (ADR-0013), the strict linear-sRGB reference (ADR-0014) and the working-color pipeline (ADR-0016), each of which checks the source's rendered color and alpha semantics at binding time.

A third read-only adapter, **AVIF** via `avif-decode` (the original 2026-11-05 dependency choice in [ADR-0018](0018-optional-codec-and-cms-dependencies.md)), used `FullFrameFallback`. [ADR-0031](0031-workspace-libavif-input-experiment.md) replaces that implementation while retaining this access class. It sharpened two parts of the contract that apply to every adapter:

- **Describe before decoding.** Construction parses only metadata: dimensions, layout, sample type and rendered ICC/CICP. The caller's color policy runs before the first region request triggers the full decode, which is then checked against the advertised size, layout and depth.
- **Rendered, not signalled, color.** Because `avif-decode` converts YUV with the bitstream's matrix and range, the adapter reports the emitted full-range RGB as CICP `[primaries, transfer, 0, 1]` from the container `colr`/`nclx` and the AV1 sequence header, carries a `colr` ICC profile as rendered ICC, and fails closed when container and bitstream disagree. AVIF `irot`/`imir` are reported as the source orientation (ADR-0019); `clap` is rejected.

JPEG reports no CICP. Its EXIF `Orientation` tag is read from IFD0 at construction; a malformed TIFF structure fails as a decode error. JPEG and AVIF report their orientation through `DecodeSource::orientation()` and the facade applies it ([ADR-0019](0019-source-side-orientation.md)); JPEG XL is rendered upright by `jxl-oxide` itself, with crops in display coordinates.

## 2026-11-11: workspace JPEG scanlines

The `jpeg-read` adapter now uses `libjpeg-turbo-rs`'s safe `Decoder` and
`Scanlines` API. It probes RGB8 dimensions, APP2 ICC, and APP1 EXIF before
pixel decoding. For each source request it starts a fresh decoder, asks
`jpeg_crop_scanline` for the horizontal range, skips leading rows, then emits
the requested RGB8 samples from one engine-budgeted row. It skips the remaining
rows and finishes the decoder, with warnings treated as errors, so malformed
ICC and truncated tails cannot look like clean output. The codec can expand
the crop to an iMCU boundary; the adapter also includes one source column on
each side when available so fancy chroma upsampling matches full-frame RGB at
the crop edge, then uses the returned origin to slice the requested columns.
The source reports `CodecRegionRequest` and zero known engine-allocated
full-frame output buffers. This records the codec request,
not a proven reduction in entropy work, coefficient storage, or process RSS;
finishing the stream can require work outside the requested vertical range.
CMYK/YCCK files fail with `UnsupportedCodecLayout` rather than relabeling
four-channel samples as RGB. Progressive JPEG can still require whole-image
codec buffers, which remain outside the engine's memory budget. This replaces only the JPEG-specific
full-frame decision above; the `DecodeSource` contract and AVIF/JPEG XL
decisions are unchanged.

The alternative was retaining the zune adapter and its full-frame RGB buffer.
The workspace port already had independent C-oracle and conformance evidence
and a safe row API; removing the engine buffer improves the observed managed
memory contract. Re-evaluate if a supported JPEG layout changes, ICC/EXIF
regressions appear, or measured whole-process memory and crop costs conflict
with the intended use case. Scaled IDCT is available in the codec but is not
selected by this adapter; ADR-0023's quality policy is still required.

### 2026-11-11 Additional JPEG XL decoder backend

[ADR-0032](0032-jpeg-xl-workspace-decoder-input.md) adds an opt-in local
reference decoder. It retains the same private source contract but reports
`FullFrameFallback` and one budgeted F32 frame. The default jxl-oxide ROI
request and row-only budget do not apply to this backend.

## 2026-11-12: measured decoder work and the JPEG tail

Issue #19 required the report to separate what was requested from what the
codec produced and did. Each `DecodeReport` now carries a `CodecWork` for the
innermost codec call, which wrappers (orientation, ICC, explicit sRGB) pass
through unchanged, and the facade exposes it as `ProcessingReport::decoder_work`
(`DecoderWork`) and the CLI as a `decoder_work:` line:

- `codec_request`, the stored rectangle the adapter asked for, and
  `codec_output`, the stored rectangle the codec actually wrote. Both are exact;
  their area difference is the source overfetch. (Clarified 2026-11-12 after
  the independent v1.0 review: `codec_request` is the rectangle the adapter
  *delivers*, not its literal codec call. The JPEG adapter calls libjpeg with
  one context column more on each side; those columns count as overfetch. Both
  rectangles are in the codec's frame, which is the display frame for the two
  JPEG XL decoders, because they apply the orientation themselves.)
- `reconstructed_pixels` and `entropy_rows` as `WorkCount::Exact` or
  `WorkCount::AtLeast`. Lower bounds come from the codec output and from work
  the format cannot skip (sequential VP8/VP8L rows, the JPEG entropy pass).
  An adapter reports `Exact` only for work it controls. A returned crop is never
  an upper bound; the access class keeps its meaning.
- `requested` and `crop`: resize filters clamp at the crop's edges, so the
  executor's request equals the crop; there is no filter halo beyond it and the
  one request delivers each row once (`delivered_rows`). The tests pin this.

Per adapter (all pinned by `tests/decoder_work.rs` and the adapter tests):

| Adapter | `codec_output` | reconstructed | entropy rows |
| --- | --- | --- | --- |
| JPEG (libjpeg-turbo-rs, port of 3.2.1) | requested rows × libjpeg's column crop: left edge of request − 1 aligned down to the iMCU width, right edge + 1 | ≥ output width × (rows + 1 tail row) | exactly the image height |
| WebP lossy (libwebp-rs, port of 1.6.0) | full width, rows 0 to request bottom + 8 | ≥ full width × output rows | ≥ output rows |
| WebP lossless | request expanded to even x/y | ≥ full width × output bottom | ≥ output bottom |
| JPEG XL, jxl-oxide 0.12.6 | exactly the request | ≥ request | ≥ request rows |
| JPEG XL, libjxl-rs (a16d32d) | full frame | ≥ full frame | ≥ height |
| AVIF (libavif-rs 1.4.2 with its libaom decoder) | full frame | ≥ full frame | ≥ height |

Measured on a 4:2:0 403×302 JPEG (`tests/fixtures/decoder-work/paris-420.jpg`),
a 40×20 crop at (101, 50) returned a 46×20 codec output at x = 96: 120 pixels of
overfetch, all 302 rows entropy-decoded. On a generated 8000×6000 4:2:0 JPEG with
EXIF orientation 6, a 2000×1500 display crop became a 1500×2000 stored request
and a 1509×2000 codec output (18,000 pixels of overfetch); all 6,000 rows were
entropy-decoded. Source hashes and the full table are in the
[M1 evidence register](../milestones/M1-evidence-and-gates.md#2026-11-12-decoder-work-and-orientation-evidence).

**Defect corrected.** The 2026-11-11 JPEG amendment says the adapter
validates the encoded tail by skipping the remaining rows and finishing.
`jpeg_skip_scanlines` to the last row marks EOI as reached without parsing the
skipped data, so `jpeg_finish_decompress` did not read it: a top crop of a file
truncated in its lower half finished cleanly. The adapter now skips all but
the last row (whole iMCU rows are entropy-decoded with discarded
coefficients) and reads that row, which reconstructs one iMCU row of the
cropped columns and lets `finish` consume the data up to EOI.
`a_top_crop_of_a_file_truncated_below_it_fails_closed` fails on the previous
code for both 4:4:4 and 4:2:0 fixtures. The cost is the tail's entropy
decoding plus one iMCU row of reconstruction; no timing was recorded.

**jxl-oxide animation.** The default JPEG XL adapter rendered keyframe 0 of an
animated file silently. It now rejects any file whose header declares an
animation with `UnsupportedAnimation`, matching WebP, AVIF and the libjxl-rs
backend; a still keyframe composited from several frames remains accepted.

What remains unmeasured: allocations and internal work inside the codecs
(including which JPEG XL groups jxl-oxide decodes for a given crop beyond the
probes recorded in ADR-0023), whole-process memory as a bound, and wall time.
Re-evaluate when an adapter changes its codec call, when a codec exposes work
counters, or when a measurement contradicts a lower bound above.
