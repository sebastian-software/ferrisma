# ADR-0023: Decode-side work, shrink-on-load and native layouts

- **Status:** Experimental (decisions 1 and 2 implemented; 3 and 4 open)
- **Date:** 2026-11-06
- **Last reviewed:** 2026-11-12
- **Supersedes:** None
- **Related:** [0012](0012-codec-source-contract.md), [0014](0014-linear-srgb-straight-alpha.md), [0015](0015-typed-output-color-signals.md), [0016](0016-working-color-space-and-transform-boundaries.md), [0018](0018-optional-codec-and-cms-dependencies.md), [0019](0019-source-side-orientation.md), [0021](0021-working-precision-policy.md), [0022](0022-parallel-row-pipeline-and-engine.md), [codec capability audit](../research/2026-11-05-decode-scaling-and-encoder-capabilities.md), [M2 milestone](../milestones/M2-performance-architecture.md) work package 5

## Context

After work package 4 the transform takes 20 ms for a 24 Mpx JPEG on 20 cores; the decode stage is the largest stage that Ferrisma controls. The work-package-0 harness measured it on synthetic plasma images only (88 ms at 24 Mpx, 173 ms at 48 Mpx). The first real photograph in the measurements, a 4032×3024 phone JPEG with an ICC profile and EXIF orientation 6 (a quarter turn) (libavif test data), showed 154 ms for its decode stage at 12 Mpx, and a profile placed most of that outside the JPEG decoder: in the ICC adapter's `f32` output being decoded again per sample through the `f64` reference transfer function, in the moxcms transform itself, and in the orientation wrapper's full-region buffer and copies. Camera JPEGs nearly always carry an ICC profile, so this is the common case, not an edge case.

Verified facts from the [codec capability audit](../research/2026-11-05-decode-scaling-and-encoder-capabilities.md) and the code:

- **`zune-jpeg` 0.5.15** has runtime-detected AVX2 and NEON paths for IDCT, upsampling and color conversion behind its `x86` and `neon` features, which Ferrisma's `default-features = false` had turned off. It has no scaled (DCT-domain) decode, no region decode and no row-streaming output; adding scaled IDCT touches six places upstream, and entropy decoding is never saved.
- **libvips' shrink-on-load** sets `scale_denom` on libjpeg, but returns a shrink of 1 in linear mode because "libjpeg shrinks in Y (of YCbCr), not linear space" (`libvips/resample/thumbnail.c:547-575`). DCT-domain scaling is a gamma-domain low-pass; it does not satisfy Ferrisma's linear-light reference (ADR-0014, ADR-0021).
- **`jxl-oxide`** skips entropy decoding and transforms outside a region for VarDCT and plain modular frames, has no reduced-resolution render, and runs single-threaded without its `rayon` feature.
- **`avif-decode`/`rav1d`** decode with up to 32 threads and convert to a full RGB frame; AV1 has no reduced-resolution decode.
- **moxcms** (the `icc` feature) produced sRGB-encoded `f32`, which the executor then linearized again per sample. Its matrix-shaper path clamps to `[0, 1]`, so wide-gamut colors outside sRGB are clipped before resampling; that predates this ADR and is recorded here as a limitation.

## Decision

1. **The default path keeps full-resolution decode.** Linear-light resampling from full-resolution samples remains the default and the oracle's definition of the image, as in libvips' linear mode.
2. **Make the decode stage cheaper without changing what it means.**
   - Enable `zune-jpeg`'s `neon` and `x86` features (runtime-detected SIMD in the decoder; same treatment as `rav1e`'s assembly in ADR-0018).
   - The ICC adapter converts to **linear-light sRGB** directly (sRGB primaries, identity transfer; reported as CICP `[1, 8, 0, 1]`, which is already the graph's working-space signal). The color contract treats that signal as transfer-ready, the reference executor decodes it with the identity, and the fast executor copies it. This removes an encode and a decode of the transfer curve per sample and their rounding.
   - Measure the orientation wrapper on real photographs before changing ADR-0019; a streaming path is possible for mirror-only orientations but not for rotations.
   - Take the per-row work of source wrappers off the thread that drives the decoder. A wrapper may hand its stateless per-row conversion to the executor (`DecodeSource::defer_conversion`, a `RowConversion`), which runs it where it linearizes the row, on the row-pipeline workers when parallel; the ICC adapter does this, and the reference executor keeps converting inside the adapter with the same function. The orientation wrapper gathers upright rows from its stored-order buffer in blocks on the transform threads and emits them in order. Both produce the same values as before for every thread count.
3. **Shrink-on-load is a separate, explicit quality policy, not a planner optimization.** A future `DecodeScaling` option may let the caller accept DCT-domain scaling (1/2, 1/4, 1/8, chosen so that at least a 2× residual linear-light resize remains, as libvips does). It is off by default, reported in the processing report, and must come with a measured quality delta (SSIMULACRA2 against the full-decode result on a real-photo corpus). It needs a scaled-IDCT path in `zune-jpeg`, which is an upstream contribution or a maintained fork; that choice is the maintainer's, outside this ADR.
4. **JPEG XL threads and AVIF native layouts are measured before they are enabled.** `jxl-oxide`'s `rayon` feature and a native YUV path for AVIF to AVIF wait for a measurement on real inputs.

## Consequences

+ The common real-photo case (ICC-tagged JPEG) loses a full per-sample `powf` pass and a rounding step.
+ Linear-light sRGB becomes a first-class rendered signal, so a JPEG XL decoder that emits linear sRGB is now accepted too.
- The JPEG decoder now runs third-party SIMD code with its own `unsafe`; its output may differ from the scalar decoder by rounding (JPEG decoding is not bit-exact across implementations).
- Without shrink-on-load, Ferrisma decodes every source pixel for a thumbnail; a non-linear Sharp default with shrink-on-load does less decode work for strong downscales. Comparisons must say which mode each side ran (see the Sharp comparison).
- The ICC matrix-shaper path now retains out-of-sRGB-gamut linear samples until final output quantization. LUT-profile behavior remains unchanged and has no extended-range guarantee.

## Alternatives considered

- **Shrink-on-load by default.** Rejected: it changes the image (gamma-domain low-pass before the linear-light filter) and would break the oracle contract silently.
- **libjpeg-turbo through FFI** for its scaled decode. Rejected under ADR-0018's dependency and `unsafe` policy for the core build.
- **Ask moxcms for sRGB-encoded 8-bit or 16-bit output** and use the 256-entry decode table. Cheaper than `f32` `powf`, but it quantizes before resampling; linear `f32` output avoids both costs.
- **Parallelize JPEG decode by restart intervals.** Possible only for files with restart markers, which most camera files lack; deferred.

## Validation

- The ICC adapter, fed a Display P3 profile written by moxcms, must match Ferrisma's own CSS Color 4 P3 to sRGB matrix within `2e-3` linear for both in-gamut and out-of-gamut values (test `cms::tests`). Independent LittleCMS profile and numeric fixtures also check signed and above-one output from the matrix-shaper path.
- All existing differential tests (ADR-0021) and orientation fixtures (ADR-0019) stay green.
- The deferred ICC conversion gives bit-identical rows to the adapter's own conversion, and the executor gives byte-identical output with the conversion deferred or not, on 1, 2 and 8 threads. The parallel gather matches the independent reference for all eight EXIF orientations, `u8` and `f32` rows, three crops and 1, 2, 3 and 8 threads, and cancels and fails with `live == 0`.
- Decode and total time of real photographs before and after, at low machine load, recorded in the M2 baseline. The decision is falsified if enabling the decoder's SIMD or the linear ICC output makes any measured case slower.

## Review when

The workspace JPEG adapter gains an explicit scaled-IDCT quality policy; moxcms changes its extended-range matrix-shaper or LUT behavior; a real-photo corpus shows the orientation buffer as a dominant cost; the libavif-rs or libjxl-rs ports replace the interim decoders (ADR-0018).

## Amendment log

- 2026-11-06 — Initial proposal: full-resolution decode stays the default; decoder SIMD and linear ICC output as meaning-preserving speedups; shrink-on-load only as an explicit, measured quality policy.
- 2026-11-06 — Decision 2 implemented and measured. `zune-jpeg` runs with `neon`/`x86`; the ICC adapter emits linear sRGB (`[1, 8, 0, 1]`), recognized by the color contract and both executors and checked against the reference P3 matrix (in-gamut worst `1.36e-4` linear; out-of-gamut values clamped by moxcms, as before); the orientation gather copies fixed-size pixels instead of calling `memmove` per pixel (ADR-0019). Before (`0612ccf`) and after, Apple M1 Ultra, best of five, all encoder and transform cores, speed 6, 1600 wide, machine load 5.3–5.6 from unrelated work:
- 2026-11-12 — The ICC adapter checks CICP beside the profile and the bounded ICC header; real-photograph LittleCMS evidence (#17, ADR-0016).

| image | decode ms before | decode ms after | total ms before | total ms after |
| --- | ---: | ---: | ---: | ---: |
| phone photo 4032×3024, ICC, EXIF orientation 6 (libavif test data), to 1600×1200 | 150 | 111 | 341 | 296 |
| flower photo 2268×1512 (libjxl test data) | 14 | 12 | 206 | 202 |
| synthetic 24 Mpx | 83 | 68 | 234 | 225 |
| synthetic 48 Mpx | 170 | 133 | 347 | 310 |

The real photo's decode stage still splits roughly into a third JPEG decoding, a quarter moxcms and the rest the orientation buffer (full-region copy); moxcms and the orientation run serially on the driver thread in front of the row pipeline, which is the next lever for real photographs. No measured case got slower. Decisions 3 (shrink-on-load as an opt-in policy, which needs a scaled IDCT in `zune-jpeg` through an upstream contribution or a fork) and 4 (`jxl-oxide` threads, native AVIF YUV) are open.
- 2026-11-06 — Corrected the test photo's orientation: `dog_exif_extended_xmp_icc.jpg` carries EXIF orientation 6 (a quarter turn), not 2, and the table above resized it to 1600×1200 rather than to its upright aspect.
- 2026-11-06 — Decision 2 extended and implemented: the ICC conversion runs on the row-pipeline workers (`RowConversion`, deferred by the fast executor), and the orientation gather runs in blocks of 16 rows on the transform threads with at most twice the thread count of blocks in flight. A timing split of the old build on the dog photo, under load, put its decode stage at about 63 ms for `zune-jpeg`, 18 ms for the rotation gather, 34 ms for moxcms and 30 ms for the driver's row handoff, all on one thread. AVIF output is byte-identical to `526e752` for the dog photo (full frame and a crop, 1, 3 and all transform threads, and `--reference`) and for the flower photo. Before (`526e752`) and after, Apple M1 Ultra, best of five, all encoder and transform cores, speed 6:

| image | decode ms before | decode ms after | total ms before | total ms after | load |
| --- | ---: | ---: | ---: | ---: | ---: |
| phone photo 4032×3024, ICC, EXIF orientation 6, to 1600×2133 | 113 | 66 | 407 | 356 | 3.95 |
| flower photo 2268×1512 (no ICC, no orientation) | 12 | 12 | 193 | 198 | 5.79 |
| synthetic 24 Mpx | 70 | 68 | 220 | 216 | 5.79 |
| synthetic 48 Mpx | 135 | 134 | 309 | 308 | 7.17 |

The real photo's decode stage is now about the `zune-jpeg` decode itself, which is single-threaded. Inputs without ICC or orientation take the unchanged path; their differences are within the noise of the higher load during their runs (the flower's 5 ms are in the encoder, which this step does not touch).
- 2026-11-06 — Decision 3's scaled IDCT comes from a JPEG decoder port under ADR-0025 (libjpeg-turbo or jpegli, both with libjpeg's `scale_num`/`scale_denom`), not from a `zune-jpeg` contribution or fork; the fork stays only as a fallback. The policy itself (opt-in, reported, quality measured) is unchanged.
- 2026-11-10 — Corrected decision 2's ICC gamut limitation: enable moxcms 0.9.1's `extended_range` feature and `allow_extended_range_rgb_xyz` only for matrix-shaper RGB/XYZ sources without device-to-PCS LUT tags. The previous default clipped P3 blue at 1.0 instead of retaining approximately 1.098 linear; the changed regression failed before the option was set and now passes. Selected samples from CC0 Display P3 and Adobe-compatible ICC files and two independently generated variants agree with a LittleCMS 2.19 relative-colorimetric float transform within `8e-4` linear; the observed maximum was `6.54e-4`. U8 codes around the sRGB transfer knee agree within `1e-5` (observed maximum `1.12e-6`; fixture provenance and hashes in `tests/fixtures/icc/README.md`). LittleCMS reopens the exact ICC bytes consumed by the Rust test. The adapter's U8 and F32 row conversion, f32/f64 executor comparison, and deferred conversion on 1, 2, and 8 workers pass. This choice preserves moxcms's existing LUT/intent selection and adds no native CMS dependency; broader #17 real-image, arbitrary-LUT, and HDR evidence remains open. Re-evaluate if moxcms changes either branch or if the larger corpus exposes a mismatch.
- 2026-11-11 — The JPEG adapter now uses the workspace libjpeg-turbo-rs decoder's safe scanlines (ADR-0012). The earlier zune SIMD and timing evidence remains historical. Full-resolution decoding stays the default; the port's scaled-IDCT capability is not enabled in the engine, and no shrink-on-load quality policy has been accepted. The adapter requests horizontal crop and leading-row skip without claiming skipped entropy work. Re-evaluate with measured image quality, wall time, and process memory before using scaled decode.
- 2026-11-12 — Measured decoder work replaces inferred ROI savings: per-adapter counts, jxl-oxide group-skipping probes and corrected statements (issue #19).
- 2026-11-12 — Decode-side threads: `DecodeSource::set_decode_threads` lets a source use the resolved transform thread count where its samples do not depend on it; the AVIF source uses it for libavif's row-split YUV-to-RGB conversion (ADR-0031). Profiles, port-versus-C ratios and ranked decode-side fixes (JPEG Huffman inlining, ICC per-channel table, tiled orientation gather) are in the [engine performance analysis](../benchmarks/v1.0-engine-performance-analysis-2026-11-12.md); none of those proposals is adopted here.
- 2026-11-12 — **Pending owner confirmation; revert commit bd3c4d44 to restore exact moxcms output.** The rendered-ICC conversion of 8-bit matrix-shaper input uses a 768-entry per-channel table built from moxcms's own single-channel outputs and checked against moxcms when built (issue #219; mechanism, scope and the exhaustive bound of 1.19e-7 linear over all 2^24 triplets for five profiles in ADR-0016). This is decision 2's meaning-preserving class only within that bound: a final rounding can flip (1 and 6 of 5,121,600 samples by one code in two 8-bit lossless checks), so the output bytes of this path may change. Interleaved A/B, synthetic 6000×4000 JPEG with an embedded profile to WebP 1600×1067, 7 repetitions, **indicative; loaded host** (load 13.6–14.9): transform stage on one thread 610.9 → 135.7 ms (sRGB v4 profile) and 611.4 → 136.0 ms (Display P3 v4); with all cores, where the conversion runs on the row workers, 16.8 → 11.1 ms of transform and 338.8 → 332.5 ms total. Managed peaks unchanged. Float input, LUT-based profiles and other paths stay on moxcms. Evidence: the [performance analysis](../benchmarks/v1.0-engine-performance-analysis-2026-11-12.md#applied-fixes-2026-11-12-evening).


## 2026-11-12: measured decoder work (issue #19)

The decoder work of each transcode is now reported as requested, codec-output
and lower-bound work counts (ADR-0012). The measurements change or qualify
earlier statements in this ADR:

- **JPEG.** Every row is entropy-decoded for any request: whole skipped iMCU rows
  call the entropy decoder with discarded coefficients, and a progressive file
  decodes every scan at start. Reconstruction (IDCT, upsampling, color
  conversion) covers the requested rows of libjpeg's column crop plus partial
  iMCU and context rows that the API does not expose, and since 2026-11-12 one
  final iMCU row for tail validation (ADR-0012). Crop/skip therefore saves
  reconstruction, not entropy work. Scaled IDCT (shrink-on-load, #56) stays
  unselected.
- **jxl-oxide 0.12.6.** The Context said that jxl-oxide skips entropy decoding
  outside a region for VarDCT and modular frames. Byte-inversion probes on
  multi-group files support this in part. When every byte of pass group 1 of
  two 257×129 multi-group VarDCT files is inverted, a 200×129 crop away from
  that group renders identically, while the full frame and a crop over it change
  or fail. A single-TOC frame stores everything in one group, so a 4×4 crop
  still reads its bytes. Patches copy from a reference-only frame, and any crop
  reads that frame. Crops of files with noise and splines equal the full render
  pixel for pixel. These are probes on small libjxl test files, not a general
  guarantee, so the adapter reports only `AtLeast` the returned rectangle.
- **libjxl-rs, AVIF.** Full-frame reconstruction for every request, as declared.
  On a 6000×4000 AVIF the managed peak was 93.4 MB while maximum RSS was
  415–508 MB (indicative, loaded host): most AVIF decode memory is opaque to the
  budget.
- **WebP.** Both bitstreams reconstruct every row above the request at full
  width (ADR-0029). A low lossy crop can hold most of the frame in managed
  memory.

No filter halo is requested beyond the crop and no row is decoded twice
(ADR-0003). Timing and representative RSS on an unloaded host are deferred;
the counts above do not depend on load.

## 2026-11-12: ICC adapter limits observed (#17)

The ICC adapter (`src/cms.rs`) now runs ADR-0015's bounded header check before moxcms (moxcms 0.9.1 accepted a declared size beyond the data), and rejects HDR, unknown or contradicting CICP beside the profile (ADR-0016, 2026-11-12). Hostile inputs (empty, truncated, oversized declared size, overflowing tag count, out-of-range tag offsets, over 4 MiB) are controlled `UnsupportedColorEncoding` errors in `cms::tests`. A real photograph with a compact sRGB ICC v4 profile, and the same crop as Display P3 with 103 out-of-sRGB samples, match LittleCMS 2.19 within `8e-4` linear (`tests/icc_photo.rs`). Observed limits: matrix-shaper RGB profiles only; LUT-based profiles keep moxcms' default path with no extended-range or LittleCMS evidence; only relative colorimetric is compared; ICC-signalled alpha input is rejected (`UnsupportedInput`); one photograph, not a corpus.
