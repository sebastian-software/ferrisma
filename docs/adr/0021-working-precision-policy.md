# ADR-0021: Working precision policy for the linear-light resize chain

- **Status:** Accepted (rule 1 and the `f32` tables implemented; rule 2, the Q1.14 path, withdrawn 2026-11-06)
- **Date:** 2026-11-05
- **Last reviewed:** 2026-11-26
- **Supersedes:** None
- **Related:** [0006](0006-cpu-first-gpu-capable-execution.md), [0007](0007-correctness-and-performance-evidence.md), [0014](0014-linear-srgb-straight-alpha.md), [0016](0016-working-color-space-and-transform-boundaries.md), [0020](0020-kernel-crate-and-isa-dispatch.md), [M2 milestone](../milestones/M2-performance-architecture.md) work packages 1 and 3, [precision experiment](../research/precision-experiment/README.md)

## Context

The M1 reference executor ([`src/quality_execute.rs`](../../src/quality_execute.rs), [`src/working_color.rs`](../../src/working_color.rs)) computes the chain `decode -> premultiply -> horizontal resample -> vertical resample -> sRGB8 quantize` entirely in `f64`. It decodes the sRGB transfer function with `powf` once per filter tap, not once per source sample, and encodes with `powf` per output sample. This is the correct oracle and stays as such, but the M2 milestone wants the same pictures from fused SIMD kernels: work package 1 asks for `f32` and lookup tables, work package 3 for a 16-bit fixed-point path with a measured error bound, and the acceptance contract requires that the plan reports the working precision.

Three precision questions have to be settled before any kernel is written, because they define the kernel contract of ADR-0020:

1. Which general working type replaces `f64`, and what it must guarantee against the oracle.
2. Whether 8-bit SDR input may run in 16-bit fixed point (twice the SIMD lanes of `f32` on the NEON and AVX2 floors, which both have `i16 x i16 -> i32` multiply-accumulate), with which representation, and when the planner must not use it.
3. Which lookup tables replace the per-sample transfer functions, with which sizes, and how they are verified.

Verified constraints (see the experiment below for the measurements):

- The sRGB8 code step at the dark end is `1 / (255 * 12.92) = 3.035e-4` in linear light; at white it is about `1 / 112`. Any representation that cannot resolve a fraction of `3.035e-4` changes dark codes.
- Lanczos3 has negative lobes. Over all input and output sizes up to 96 and ratios up to 8192, the largest sum of positive normalized weights of one output position is `G = 1.28571` and the largest sum of negative weights is `N = 0.28571`; both occur for tiny images where [`quality::taps`](../../src/quality.rs) clips the support at the border and renormalizes, which also pushes the largest single weight to `1.2824`. For large downscales `G` tends to `1.18`; for an interior upscale phase it is `1.2718`. The reference never clamps between passes, so a fast path must carry these overshoots too.
- Display P3 input decodes to linear sRGB outside `[0, 1]`: exhaustively over all `2^24` 8-bit triples the range is `[-0.2249, 1.2249]` (red channel; green `[-0.0421, 1.0421]`, blue `[-0.0983, 1.0983]`). The reference preserves these values until the final clamp.
- Both supported SDR color encodings use the sRGB transfer function, so one 256-entry decode table serves sRGB and Display P3; only the 3x3 matrix differs.

Hypotheses, deliberately not claimed here: that the fixed-point path is faster end to end, and by how much. ADR-0007 requires that number from the work package 0 harness before any kernel is kept. This ADR settles the correctness side only, so that the speed question can be asked with the semantics fixed.

### How other engines choose precision

Verified in the upstream sources on 2026-11-05. Line numbers refer to the stated revisions.

| Engine (revision) | 8-bit path | 16-bit path | Float path | Linear light | Alpha |
| --- | --- | --- | --- | --- | --- |
| libvips master `12c9899` (release 8.18.7) | `int16` coefficients with 12 fractional bits (`VIPS_INTERPOLATE_SHIFT` = 12, `libvips/include/vips/interpolate.h:117`), produced by a truncating cast from `double` (`resample/reducev.cpp:957`, `reduceh.cpp:497`), so row sums are not forced to 4096; `int32` accumulation (`templates.h` `LongT`), rounding by adding 2048 and shifting (`templates.h:152-156`). The Highway SIMD path exists for `uchar` only (`reducev.cpp:986`, `reduceh.cpp:526`): `int16 x int16 -> int32` `ReorderWidenMulAccumulate`, `ShiftRight<12>`, demote to `u8` (`reducev_hwy.cpp:110-235`). | `ushort` uses the same 12-bit `int16` coefficients and `int32` accumulation (`reducev.cpp:574`, `reduceh.cpp:289`). | `float` images use the `double` coefficient table with `double` accumulation; `double` images recompute the mask per position in `long double` (`reducev.cpp:594-603`). | Only `vips_thumbnail(linear=TRUE)`: converts to `scRGB` float before `vips_resize` (`thumbnail.c:869-907`). | Premultiplies before resizing; `uchar` input stays `uchar` through a 256-entry scale table (`thumbnail.c:958-962`, `premultiply.c:252-258`), other formats become `float`. Each pass writes the image's own format, so ringing is clipped between passes. |
| Sharp 0.35.5 (libvips 8.18 via `@img/sharp-libvips` 1.3.4) | `premultiply().cast(format)` then `resize` with the kernel option (`src/pipeline.cc:473-485`): 8-bit input is resized by libvips's 12-bit integer path on 8-bit premultiplied samples. | `pipelineColourspace('rgb16')` (`docs/src/content/docs/api-colour.md:65-92`) switches to `ushort`, same 12-bit coefficients. | `pipelineColourspace('scrgb')` selects libvips's float path. | Not by default. | Premultiplied in the pipeline color space, unpremultiplied after all transforms (`pipeline.cc:768-772`). |
| Pillow main `32a2498` (release 13.0.0) | `int32` coefficients with 22 fractional bits (`PRECISION_BITS = 32 - 8 - 2`, `src/libImaging/Resample.c:137`), rounded (`314-328`); `u8 x int32` in `int32` accumulators seeded with `2^21`, result through a 1280-entry clip table (`219-224`, `362-431`). | `I;16`: `double` coefficients and sums (`_ImagingResampleHorizontal_16bpc`). | `INT32`/`FLOAT32`: `double` coefficients and sums (`_32bpc`, `649-760`). | None; resampling in the stored encoding. | None inside resize. Each pass writes 8-bit, so ringing is clipped between passes. |
| Pillow-SIMD `simd/master` `788bf91` (v12.3.0.post0) | `int16` coefficients with adaptive precision: the largest `p < 22` with `round(max_w * 2^(p+1)) < 2^15` (`Resample.c:97-98`, `275-306`); `_mm_madd_epi16`/`_mm256_madd_epi16` on `u8` unpacked to `i16`, `int32` sums, `srai_epi32(p)`, saturating packs (`ResampleSIMDHorizontalConv.c:22-102`). | As Pillow. | As Pillow. | None. | None. |
| fast_image_resize 6.1.0 (main `ce36aeb`) | "Based on Pillow-SIMD": `Normalizer16` makes `i16` coefficients with adaptive precision (`PRECISION_BITS` 22, `MAX_COEFFS_PRECISION` 15, asserted >= 4) for `i32` accumulation and a 1280-entry clip table (`src/convolution/optimisations.rs`, `u8x4/native.rs`). | `Normalizer32` makes `i32` coefficients (up to 31 bits) for `i64` accumulation (`u16x4/native.rs`). | `f64` coefficients and `f64` sums, result cast to `f32` (`f32x4/native.rs`). | Not inside the resizer. `PixelComponentMapper` holds 256- and 65536-entry tables built from an `f32` function with `.round()` (`src/color/mod.rs:40-50`, `172-177`); `create_srgb_mapper()` uses `f32` `powf` (`color/mappers.rs`). The documented route is sRGB `u8` -> linear `u16` -> resize -> `u8`. | Multiply and divide by alpha for `U8x2/U8x4/U16x2/U16x4/F32x2/F32x4` in the image's own encoding (`src/alpha/mod.rs`). |

What Ferrisma takes from them: the `i16 x i16 -> i32` multiply-accumulate shape (libvips Highway, Pillow-SIMD, fast_image_resize) and the adaptive per-axis coefficient precision (Pillow-SIMD, fast_image_resize). What is different: all four resample 8-bit *gamma-encoded* samples and clip to the stored format after every pass, and none defines its error against an independent higher-precision oracle. Ferrisma's samples are linear light, the chain keeps overshoot between passes, and the bound is stated against `f64`.

## Decision

### 1. `f32` is the general working type

Every fast path works in `f32`. `f64` remains only in the scalar reference executor and in tests. Specifically:

- 8-bit input decodes through the 256-entry `f32` table (rule 3); F32 input (JPEG XL float) decodes with the extended sign-preserving sRGB EOTF computed in `f32` per sample; Display P3 input adds the folded P3 to sRGB matrix in `f32`.
- Filter weights are normalized in `f64` and stored as `f32`. Kernels may sum taps in any order and with any fusion; `f32` paths are therefore *not* required to be bit-identical to each other, only bounded against the oracle (rule 4).
- Straight alpha is premultiplied in `f32` linear light, filtered as a fourth channel, and divided out in `f32` with the oracle's `alpha > 1e-12` rule.
- Output encodes through the exact `f32` encoder of rule 3 after clamping to `[0, 1]`.

### 2. The Q1.14 fixed-point path: dropped

Dropped on 2026-11-06 by this ADR's own review trigger (the fused Q1.14 chain was not faster than `f32`). Only `f32` paths exist; the plan reports `f32`.

### 3. Lookup tables

| Table | Shape | Definition | Verification |
| --- | --- | --- | --- |
| sRGB8 to linear, `f32` | 256 x `f32` (1 KiB) | `decode_srgb(code / 255)` computed in `f64`, cast to `f32` | Entry-wise equality with the `f64` reference cast to `f32` (the measured relative error is `5.2e-8`, pure `f32` rounding) |
| Linear `f32` to sRGB8, exact | 4097 x `u8` bucket table + 257 x `f32` thresholds (5 KiB) | bucket `b = floor(x * 4096)` gives the code at the bucket's lower edge; `threshold[c]` is the smallest `f32` whose reference encoding is `>= c`; result is `code[b] + (x >= threshold[code[b] + 1])`. The construction asserts that no bucket spans more than one code. | Exhaustive over every `f32` in `[0, 1]` (1,065,353,217 values): 0 mismatches against `encode_linear_srgb8(x as f64)`, 0.6 s on 20 threads |

No table is interpolated. Tables are generated from the `f64` reference functions and checked against them by tests; a committed table that drifts from the reference fails the test. The sizes keep every table in L1 on the target hardware.

Rejected encoder shapes, measured in section E3 of the experiment: nearest-index tables without interpolation mismatch the reference in 1.54 % (12 fractional bits), 0.41 % (14 bits) and 0.09 % (16 bits) of uniformly spaced inputs, and in 15.5 %, 3.2 % and 0.9 % of inputs in the darkest range `[0, 0.01]`, always by one code. The exact encoder costs two lookups and one compare per output sample and removes this error class entirely.

### 4. The acceptance bound

Every fast path `P` (scalar `f32`, SIMD `f32` on any ISA, scalar Q1.14, SIMD Q1.14 on any ISA, any tile shape) is tested against the `f64` reference executor on the same decoded input, after sRGB8 quantization, per channel and per sample. Rule A applies to every sample; rules B to D are evaluated per input, pooled over both filters (Area, Lanczos3) and the geometries identity, 1.5x up, 1.5x, 2x, 2.37x, 8x, 16x, 32x, 64x and 128x down, down to 2x2, and anisotropic 1x/16x, so that tiny outputs cannot dominate a percentage:

| Rule | `f32` paths | Q1.14 paths |
| --- | --- | --- |
| A. Maximum error, every sample and channel (RGB where the reference output alpha is nonzero; the alpha channel itself) | `<= 1` code | `<= 1` code |
| B. Share of exactly equal RGB samples on full-range inputs (uniform noise, bright noise, saturated edges, P3 primaries, P3 noise, F32 noise, F32 extended range, F32 P3 noise, and the RGB of the alpha inputs) | `>= 99.9 %` | `>= 99 %` |
| C. Share of exactly equal RGB samples on dark and tie-prone inputs (noise in codes 0-24, gradient, 32-pixel flat blocks, 1- and 2-pixel checkerboards, P3 checkerboard, F32 values in `[0, 0.01]`) | `>= 99.5 %` | `>= 95 %` |
| C'. Share of exactly equal alpha samples on RGBA inputs | `>= 98 %` | not applicable (alpha is `f32` only) |
| D. Mean absolute error per input | `<= 0.002` | `<= 0.05` |
| E. Where the fast path's own output alpha byte is 0, it emits RGB 0; RGB is compared only where both alpha bytes are nonzero | required | not applicable |
| F. Bit-identical to the scalar implementation of the same precision across ISAs, tile shapes and thread counts | not required | required |

The inputs are the seventeen synthetic images of the experiment plus, once work package 0 exists, every image of the benchmark corpus. ADR-0020 adds its own intra-level contract on top: integer variants bit-exact and `f32` variants within 4 ULP of the scalar body of the same kernel; this ADR's rules bound every path against the `f64` oracle after quantization. The thresholds are set below the pooled minima of section E9: `f32` 99.91 % on the gradient and 99.13 % on the alpha channel of alpha noise, mean at most 0.0009; Q1.14 96.13 % on dark noise and 98.73 % on the gradient in the tie-prone set, 99.35 % on P3 noise in the full-range set, mean at most 0.039. Single geometries go lower (Q1.14 90.95 % on the gradient at exactly 2x, where the reference lands on rounding midpoints), which is why the rule pools. One-code differences are tolerated deliberately: any implementation with different rounding points differs from the oracle at ties, and the dark-end code step is 4.97 Q1.14 LSB, so three half-LSB roundings move about 4 % of dark samples across a code boundary. The photo-corpus threshold for Q1.14 is expected near the full-range value and is to be recorded when the corpus exists.

Rule E documents an oracle imperfection found by the experiment: `final_rgba` divides whenever alpha exceeds `1e-12`, so a reference pixel with alpha below `1/510` carries nonzero RGB although its alpha byte rounds to 0, contrary to the canonical zero of ADR-0014. The comparison counts such pixels as invisible, and the reference should be sharpened in a separate change.

### 5. The plan reports the precision

The processing report names the working precision (`WorkingPrecision`, `f32`).

## Consequences

+ One testable bound against one oracle for every fast path; the integer path is additionally deterministic across machines, which the `f32` path cannot be.
+ No `powf` per sample for 8-bit input: four lookups and a few integer or float operations per source sample, two lookups and a compare per output sample.
+ Display P3 costs seven lookups and adds instead of a matrix multiply, without a separate pass.
+ The planner's interval bound is cheap (one pass over the quantized taps) and turns overflow safety into a per-plan proof instead of a global assumption.
+ Tables total under 30 KiB and are verified exhaustively, so a wrong entry cannot survive the tests.
- Two working types plus the oracle to maintain; the Q1.14 kernels are a second kernel set in ADR-0020.
- `f32` SIMD output can differ between ISAs by one code at ties; differential tests compare each path to `f64`, not to each other.
- The speed advantage of Q1.14 is a hypothesis until work package 3 measures it; if it does not materialize the path is dropped (see Review when).

## Alternatives considered

- **Keep `f64` everywhere.** Twice the memory traffic and half the lanes of `f32` for no measurable fidelity gain: `f32` matches the oracle in at least 99.4 % of samples and never by more than one code.
- **`u16` Q0.16 samples with `i32` weights and `i64` accumulation (fast_image_resize's 16-bit path).** Two more bits of sample precision, but no headroom for negative lobes or overshoot: the intermediate must either be clamped (changing Lanczos ringing relative to the reference) or widened to `i32`, which gives up the lane advantage.
- **libvips-style fixed 12-bit coefficients.** Simple, but the truncating cast does not preserve row sums and 12 bits leave a 384-tap 64x downscale with coefficient steps of `2.4e-4`; the adaptive `p` of Pillow-SIMD and fast_image_resize is strictly better at no cost.
- **Universal coefficient cap of 15 bits** instead of the per-plan bound. Provably safe for Display P3 without a bound check, but measurably worse at extreme downscales (section E9 with `--p-cap 15`: 97.7 % instead of 99.1 % exact at 64x, 96.5 % instead of 98.6 % at 128x). The exact bound keeps `p = 16` wherever it is safe.
- **Q2.13 or Q3.12 samples** for more headroom. Unnecessary: the bound shows Q1.14 holds every plan of both filters and both color encodings, and each lost bit doubles the dark-end error.
- **Clamp intermediate rows to `[0, 1]`** as libvips, Pillow and fast_image_resize do by writing the stored format after every pass. Rejected: it changes ringing at edges relative to the reference, which never clamps before the final encode.
- **`i32` intermediate rows** (no rounding between passes). Removes one of three roundings, so dark-end mismatches would fall by about a third, but halves the vertical pass lanes and doubles the row memory. Deferred until the Q1.14 path is measured.
- **Interpolated encode table.** Still inexact unless proven; the bucket-plus-threshold encoder is exact and cheaper.
- **Per-sample `f32` `powf` for 8-bit decode and encode.** Inexact at thresholds and, by every published measurement of transcendental functions, slower than a lookup; the tables are exact.
- **8-bit fixed point** (`u8 x i8` dot products: `vpmaddubsw`, VNNI, SME int8). Rejected for linear light: codes 0 to 24 span linear `[0, 0.0094]`, which is 2.4 of 255 levels, so 8-bit linear samples cannot represent dark content. It may serve the gamma-domain native policy of ADR-0023, which is a different quality policy.
- **`f16` storage.** Not on the CPU floor (AVX2 converts but does not compute in `f16`; NEON `f16` arithmetic is optional), and 11 significant bits are below Q1.14 at the dark end. A GPU option for ADR-0024 with its own bound.
- **Q1.14 for RGBA with the division in `f32`.** Rejected by measurement: the error comes from the rounding before the division, not from the division.

## Validation

**Existing evidence.** The standalone program in [`docs/research/precision-experiment/`](../research/precision-experiment/README.md) (own `Cargo.toml`, no dependency on the engine crate; its reference functions are hand copies of `src/working_color.rs` and `src/quality.rs` at commit `17f3f46`) was run as

```sh
cd docs/research/precision-experiment
cargo run --release > results/2026-11-05-apple-m1-ultra.md
cargo run --release -- --p-cap 15 --skip-exhaustive-f32 > results/2026-11-05-apple-m1-ultra-p-cap-15.md
```

on an Apple M1 Ultra (20 threads), macOS 27.0, rustc 1.100.0-nightly (5a2be9f5f 2026-09-06); the whole run takes 4.2 s. Its results, all recorded in [the results file](../research/precision-experiment/results/2026-11-05-apple-m1-ultra.md):

| Section | Result |
| --- | --- |
| E1 decode tables, exhaustive 256 codes | `f32` table: max relative error `5.2e-8`. Q1.14 table: max error `3.05e-5` linear (0.10 dark-end code steps), max relative error `5.4e-3` at code 1; 256 distinct values; decode then exact encode round-trips 256 of 256 codes. |
| E2 Display P3, exhaustive `2^24` triples (0.17 s) | Linear sRGB range `[-0.2249, 1.2249]`; Q.30 product tables within 0.5 LSB of `f64`, 113 entries one LSB off the rounded reference, none by two; `f32` tables within `1.22e-7`. |
| E3 encoder shapes, `2^24` points in `[0, 1]` and `2^20` in `[0, 0.01]` | Nearest-index tables: 1.54 % / 15.5 % (12 bits), 0.41 % / 3.2 % (14 bits), 0.09 % / 0.9 % (16 bits) mismatches, max one code. Exact encoder: 0 and 0. |
| E4 exact `f32` encoder, every `f32` in `[0, 1]` | 1,065,353,217 values, 0 mismatches, 0.6 s on 20 threads. |
| E5 `f32` `powf` decode, `2^24` points in `[-0.2, 1.5]` | Max error `5.6e-7` linear (0.0018 dark-end code steps), relative `3.9e-7`. |
| E6 fixed-point premultiply, exhaustive 256 x 256 | Divide form and shift form within 1 LSB of `round(f64 * 2^14)`; alpha exact. Recorded for a future RGBA fixed path; not permitted by rule 2. |
| E7 filter gain | Lanczos3: `G = 1.28571`, `N = 0.28571`, max weight `1.2824` (input 4, output 5, position 2); large ratios `G -> 1.18`; Area `G = 1`. Worst-case interval propagation: sRGB rows within `[-0.2857, 1.2857]` (21065 of 32767), vertical accumulator `0.867 * 2^31` at `p = 16`; Display P3 rows within `[-0.6392, 1.6392]` (26857), vertical `1.145 * 2^31` at `p = 16` and `0.573 * 2^31` at `p = 15`, which is why rule 2.4 bounds each plan exactly instead of capping `p` globally. |
| E8/E9 full chains, 17 inputs x 12 geometries x 2 filters, 19.8 million output samples per input | `f32`: max one code on every input; pooled exact share 99.91 % on the gradient, 99.94 % on dark noise, at least 99.99 % on every other input including F32 input; per geometry 99.42 % worst (gradient at 2x). Q1.14, opaque inputs: max one code everywhere, no sample off by two; pooled exact share 96.13 % on dark noise, 98.73 % on the gradient, 99.35 % to 99.99 % on everything else, 100 % at identity for sRGB; pooled mean absolute error at most 0.039; per geometry 90.95 % worst (gradient at 2x). Q1.14, RGBA inputs: max 31 codes, 64.8 % exact where the reference alpha is 1 to 25, max one to three codes elsewhere; `f32` RGBA: max one code, 100 % exact in the low-alpha buckets, alpha channel 99.13 % exact pooled. Largest accumulator observed `0.754 * 2^31` against a largest planner bound of `0.965 * 2^31`; largest row `0.773 * 32767` against `0.810`. |

**Done:** the kernel crate implements the `f32` path and the tables of rule 3; the differential suite encodes the bound.

**Not recorded:** the benchmark corpus is not part of the bound's input set, and no photograph-corpus threshold is written down.

## Review when

- Fired on 2026-11-06: the fused Q1.14 chain was measured and dropped; `f32` and the oracle stay.
- A source with more than 8 bits per sample (AVIF 10-bit, JPEG XL 16-bit) should run in fixed point: Q1.14 cannot resolve a 10-bit dark step (`7.6e-5` linear is 1.2 LSB), so this needs a new representation and its own experiment.
- The output is not sRGB8 (10-bit AVIF, 16-bit, float output) or the input is PQ/HLG: the encoder tables and the headroom analysis are sRGB8-specific.
- A new filter or stage with larger negative lobes (sharpening, Lanczos with more lobes) enters the chain: the per-plan bound still applies, but the row format may reject plans, and the gain constants in this ADR must be re-measured.
- ADR-0023 produces decoder-side scaled samples in another format, or the gamma-domain native path needs a precision statement.
- ADR-0024's GPU path wants `f16` storage or a different accumulation: it needs its own bound, measured by the same program.
- The oracle changes (transfer constants, matrices, the alpha rule of `final_rgba`): regenerate the tables and rerun the experiment; the hand-copied reference in the experiment must be updated in the same change.
- A perceptual metric on the benchmark corpus shows that one-code dark-end differences change encoder decisions or the quality-matched comparison: tighten rule C or narrow Q1.14 eligibility.
- libvips or Sharp change their default precision or add a linear-light default: the matched-quality comparison of M2 must be re-examined.
- Stable Rust gains portable SIMD or `f16` arithmetic that changes the kernel trade-offs in ADR-0020.

## Amendment: output depth follows the input (2026-11-15, owner decision)

**Decision (owner, 2026-11-15, before 1.0).** The output's bit depth follows
the input's bit depth, by one rule in every format that can represent it
(ADR-0043, decided application "Bit depth"). Until this amendment every
output row was sRGB8: a "10-bit" AVIF carried 8-bit content, JPEG XL output
was always 8-bit, and a deeper input was reported as
`Discarded::PrecisionReduced` whatever the output depth. What follows is
how the engine implements the decision; the choices marked as working
decisions are delegated and revisable by the owner.

**1. The input's depth.** Each reader states the significant bits of the
samples it delivers, color and alpha alike, as one number
(`DecoderInfo::depth`): the larger of the color and the alpha depth, so
that a deeper alpha is never lost silently. JPEG: 8 (12-bit and lossless
JPEG are outside the port's scope and refused). WebP: 8. AVIF: the AV1
sequence header's `BitDepth`, 8, 10 or 12; AVIF alpha has the color depth.
Gray (monochrome AVIF, gray JPEG XL, gray JPEG) has the depth of its one
channel; the expansion to RGB (ADR-0043) copies it. JPEG XL:
`max(bits_per_sample, alpha_bits)` for integer samples, 1 to 16 (a reader
depth below 8 is written at 8, which holds it). Float JPEG XL samples
(`exponent_bits_per_sample != 0`) stay refused (`UnsupportedLayout`, as
before); this amendment does not add float output. A resize, a crop, an
orientation or a color conversion does not change the depth: the pipeline
computes in `f32` (rule 1), so the depth is a property of the final
quantization, not of the arithmetic. **Dithering:** there is none, at 8
bits (today's path rounds to nearest, `encode_linear_srgb8`) or deeper;
none is added: an output at its input's depth needs none, and a reduction
is reported, not hidden.

**2. The rows handed to encoders.** One representation per depth class for
every sink, built-in or external: at 8 bits straight-alpha RGBA `u8`, byte
for byte as before; above 8 bits the same layout as native-endian `u16`
samples scaled to the target depth, `0..=2^depth - 1` (never a padded or
shifted 8-bit value). This is the form libavif's `avifRGBImage` (`depth`
10/12, `uint16_t` pixels) and libjxl's `JXL_TYPE_UINT16` with
`JXL_BIT_DEPTH_FROM_CODESTREAM` take. Cost: an output row and every sink
frame double, 8 B/px instead of 4 for RGBA. The managed budget (ADR-0004)
charges what exists: the executors' output rows by their type size, the
AVIF writer's RGBA frame at 4 or 8 B/px plus its YUV(A) planes at 1 or
2 B/sample (it always charged 2 B/sample for 10-bit planes), and the JPEG
XL sink's `u16` frame plus a codec reservation measured for 16-bit rows
(unit D of this change; the 8-bit model of 2026-11-15 is not scaled).

**3. The quantization.** One function for every depth:
`round(oetf(clamp(x, 0, 1)) * (2^depth - 1))`, rounding half away from
zero, alpha `round(clamp(a, 0, 1) * (2^depth - 1))`, and transparent black
where the alpha code is 0 (ADR-0014). At depth 8 it is the sRGB8 encoder
bit for bit (`encode_linear_srgb8` is now `encode_linear_srgb(x, 255)`).
The `f64` reference applies it to its `f64` accumulator, the fast
executors to the `f32` value: for 8 bits through the exact table of rule 3,
above 8 bits through the reference function itself on `f64::from(x)`, so
both fast encoders are the oracle's encoder of the same `f32` value; the
only difference to the oracle is the `f32` accumulation. Bound (rule 4 at
deeper depths, `fast_execute_tests::deep_output_meets_the_bound_against_the_f64_oracle`,
the 17 inputs of rule 4 plus five deep inputs, every geometry and filter,
scalar and NEON): rule A (at most one code, every sample) and rule E hold
unchanged at 10, 12 and 16 bits; the exact shares are set per depth below
the measured minima (10-bit: full range 99.99 %, dark/ties 99.95 %, mean
0.001 codes; 12-bit: 99.95 %, 99.0 %, 0.01; 16-bit: 99.8 %, 99.8 %,
0.002). They are new bounds, not loosened ones: one 16-bit code is 1/257
of an 8-bit code. The lowest share, 99.27 % at 12 bits, is the 12-bit
dark-noise input: its codes lie on the sRGB linear segment, where an Area
average of integer codes lands on exact half codes and `f32` and `f64`
round the tie differently. Identity at every depth is exact
(`every_deep_code_survives_an_identity_transform_at_its_depth`: all 1024,
4096 and 65536 codes, both executors, sequential and parallel).

The deeper quantization is scalar, beside the 8-bit one in
`src/fast_execute.rs`. **Finding:** `ferrisma-kernels` has no quantize
kernel at any ISA level (only `horizontal_rgba` and `axpy`); the 8-bit
encoder is the engine's scalar table code. A NEON/AVX2 deep quantizer
would be a first SIMD quantizer and a performance change, which AGENTS.md
allows only with a measurement; it is left as a follow-up.

**4. The public surface.** `AvifOptions::depth` and a `depth` field on both
variants of `JpegXlOptions` are `Option<u8>`: `None` (the default) follows
the input, `Some(bits)` is explicit. Same name, same semantics; WebP has
none (8 bits is the format's limit). The rule for `None`: the largest
depth the output admits that is at most the input's depth (AVIF 8/10/12,
JPEG XL 8 to 16, WebP 8), and never below 8. An explicit depth outside the
output's set is `InvalidOptions`; an explicit depth below the input's is
written and reported as `PrecisionReduced`; an explicit depth above the
input's is written with correctly scaled content (the quantization above
at that depth). `PrecisionReduced` is reported exactly when the output
depth is below the input's depth, by the pipeline, not by a reader. CLI:
`--avif-depth=8|10|12` and `--jxl-depth=8..16`; the report line names the
depth read and written (`input_depth=`, `depth=`; also
`ProcessingReport::input_depth` and `output_depth`). External backends (`EncoderBackend`, experimental
tier): additive, no break for existing implementations:
`EncodeRequest::depth` (the resolved depth of the rows),
`EncoderBackend::row_depths` (default `[8]`: the depth rule takes the
largest of them at most the input's and reports a reduction, so an existing
backend keeps receiving 8-bit rows)
and `RowEncoder::write_row_u16` (default: refused with `InvalidOptions`).
The break is in the options: `AvifOptions::depth` changes from `u8` to
`Option<u8>`, and `JpegXlOptions::{Lossy, Lossless}` gain a field.

**5. What each port admits, checked in the port.** AOM through libavif:
10 and 12 bits, 4:4:4 and 4:2:0, with alpha (libavif writes the alpha item
at the color depth); 12 bits selects AV1's Professional profile in
libavif. Checked by encoding through the engine and by the byte comparison
with the C original in unit C. JPEG XL public encoder (`admitted_frame_profile`
and `admitted_lossless_profile` in `codecs/libjxl-rs/src/jxl/enc_frame_vardct.rs`,
`modular_16_bit_buffer_sufficient` in `encode.rs`): lossy VarDCT admits
integer samples of any depth up to 16 from `uint16` input, and alpha of
any integer depth; the lossless profile (effort 2) admits integer gray or
RGB of at most 16 bits with integer alpha. Above 12 bits of lossless
color, or alpha above 12 bits, libjxl requires codestream level 10
(`VerifyLevelSettings`), and both the original and the port then write the
ISOBMFF container with a `jxll` box (the container is ported, "encoder
container"); the engine's raw codestream becomes a container exactly
there, as `cjxl` writes it. No gap was found for the depths of this
decision; the gaps of ADR-0039 (patches, dots at distance 3) are unchanged.

**What the originals do** (pinned sources). `avifenc` (libavif 1.4.2,
`codecs/libavif-rs/upstream/libavif`) writes its input's depth: PNG at 8
bits stays 8, any deeper PNG becomes 12, its largest
(`apps/shared/avifpng.c:376`-`382`, `avif->depth = requestedDepth;` then
8 or 12), JPEG is 8 (`apps/shared/avifjpeg.c:1340`, `:1376`), Y4M keeps its
own (`apps/avifenc.c:706`-`710`). `cjxl` (libjxl `a16d32d0`) copies the
decoded input's basic info, `bits_per_sample` and `alpha_bits` included
(`lib/extras/enc/jxl.cc:257` `JxlBasicInfo basic_info = ppf.info;`, alpha
at `:82`); its PNM reader takes the depth from `maxval`
(`lib/extras/dec/pnm.cc:276`, `:300`), gives alpha the color depth
(`:537`) and passes the samples as they are (`:649`,
`JXL_BIT_DEPTH_FROM_CODESTREAM`). The engine's rule is the same, with the
nearest admitted depth below where the output cannot hold the input's
(16-bit to AVIF gives 12, as `avifenc` does for a 16-bit PNG).

## Amendment: 1:1 sRGB identity pass-through and single-pass tap planning (2026-11-26, #284)

**1. 1:1 sRGB identity pass-through.** When the fast executor receives an exact 1:1 resize (`plan.input == plan.output`, `plan.footprint == plan.resize`, `plan.source.{width,height} == requested.{width,height}`) on sRGB-encoded samples (`ColorEncoding::Cicp([1, 13, 0, 1])`) with no deferred row conversion (`conversion.is_none()`), it streams rows directly through a single budgeted `OutputBuffer` without building tap tables, linearizing to 4×`f32`, running 1-tap horizontal/vertical resample passes, or re-encoding:

- **U8 → U8:** opaque RGB packs `[r, g, b, 255]`; straight RGBA masks RGB to zero whenever `alpha == 0` (ADR-0014 transparent black).
- **U8 → U16 (depths 9–16):** maps each byte through a 256-entry exact table `round((v / 255.0) * max)`, applying transparent black when `alpha == 0`.
- **F32 → U8/U16:** validates straight alpha (`0.0..=1.0`, finite) and RGB finiteness in the same order as `working_color`, clamps to `[0.0, 1.0]`, zeroes RGB when the output alpha code is `0`, and quantizes via `round(f64::from(x) * max)`.

Display P3 (`[12, 13, 0, 1]`), linear sRGB (`[1, 8, 0, 1]`), and deferred ICC transforms are explicitly excluded and continue to run the full linear-light executor. `HiddenColorProbe` wraps the decoder source upstream of the executor, so `Discarded::HiddenColor` reporting is unchanged. Because the identity pass-through processes every row on the decoder callback thread, `ProcessingReport::transform_threads` reports `1` (without reducing the decoder's own thread allowance). Output is byte-identical across thread counts and verified bit-exact against the `f64` oracle across all `256 × 256` U8 `(value, alpha)` pairs at depths 8, 10, 12, and 16 (`identity_passthrough_u8_rgba_and_rgb_match_f64_oracle_exhaustively`), across all 10-, 12-, and 16-bit F32 codes (`every_deep_code_survives_an_identity_transform_at_its_depth`), and within ADR-0021 rule A on arbitrary `F32` inputs.

**2. Single-pass tap planning.** `quality::footprint` previously evaluated `taps()` (calling `sin()` and allocating a temporary `Vec<Tap>`) for every output column and row solely to determine min/max source indices, after which `TapTable::build` recomputed every tap with fresh temporary vectors. `quality::footprint` now short-circuits `input == output` in O(1) and otherwise scans from the output ROI edges inward using exact filter-support monotonicity (handling Lanczos3 sinc zero crossings without scanning interior positions and with zero heap allocations). `TapTable::build` reuses a single `Vec<Tap>` scratch buffer per axis via `quality::taps_into`. Verified against the brute-force `taps()` scan across both filters and all small/interior/boundary geometries (`quality_tests::footprint_matches_exhaustive_taps_scan_for_both_filters`).

**3. Measurements (2026-11-26, Apple M1 Ultra, macOS arm64, release build; indicative).**

Interleaved base (`cef60399`) vs head micro-benchmark (5 interleaved process runs of the identical `bench_issue_284_planning_and_identity` workload: `footprint` + `TapTable::build` for both axes, 3 warmup + 15 timed iterations median; plus synthetic 1:1 `execute_fast` runs; `cargo test --release -p ferrisma --lib bench_issue_284 -- --ignored --nocapture`):

| benchmark (median of 5 interleaved runs) | base (`cef60399`) | after #284 | speedup / delta | managed peak before → after |
| --- | ---: | ---: | ---: | ---: |
| plan `6000×4000 → 1600×1067` Lanczos3 | 1,564 µs | 778 µs | 2.01× (−50.3 %) | — |
| plan `6000×4000 → 1600×1067` Area | 239 µs | 55 µs | 4.35× (−77.0 %) | — |
| plan `6000×4000 → 6000×4000` Lanczos3 | 1,252 µs | 44 µs | 28.45× (−96.5 %) | — |
| identity `24 MP U8 RGB → U8` | 152,784 µs | 2,425 µs | 63.00× (−98.4 %) | 31,672,112 → 24,000 B |
| identity `24 MP U8 RGBA → U8` | 210,460 µs | 4,716 µs | 44.63× (−97.8 %) | 31,672,112 → 24,000 B |
| identity `12 MP F32 RGB → U16` | 851,933 µs | 32,675 µs | 26.07× (−96.2 %) | 22,176,112 → 32,000 B |
| identity `12 MP F32 RGBA → U16` | 928,860 µs | 42,773 µs | 21.72× (−95.4 %) | 22,176,112 → 32,000 B |

Interleaved CLI A/B (5 repetitions, `/usr/bin/time -l`, bit-identical output SHA-256 on every case; synthetic 24 MP 8-bit AVIF and 12 MP 16-bit lossless JXL derived from committed `tests/fixtures/decoder-work/paris-420.jpg`, plus committed `tests/fixtures/jxl-deep/d16-2049x40-a1-lossless.jxl`):

```sh
# Generate deterministic inputs from committed fixture:
target/release/ferrisma tests/fixtures/decoder-work/paris-420.jpg /tmp/bench-24mp-8b.avif 6000 4000 --assume-srgb --avif-depth=8 --avif-speed=10 --avif-chroma=420 --encoder-threads=0
target/release/ferrisma tests/fixtures/decoder-work/paris-420.jpg /tmp/bench-12mp-16b.jxl 4000 3000 --assume-srgb --jxl-lossless --jxl-depth=16

# 1:1 cases timed under /usr/bin/time -l (t1 = --transform-threads=1, t0 = --transform-threads=0):
target/release/ferrisma /tmp/bench-24mp-8b.avif /tmp/out.webp 6000 4000 --webp-quality=75 --webp-method=0 --transform-threads=1
target/release/ferrisma /tmp/bench-12mp-16b.jxl /tmp/out.jxl 4000 3000 --jxl-lossless --transform-threads=1
target/release/ferrisma tests/fixtures/jxl-deep/d16-2049x40-a1-lossless.jxl /tmp/out.jxl 2049 40 --jxl-lossless --transform-threads=1
```

| case (median of 5 interleaved runs) | `transform_us` before → after | `total_us` before → after | `peak_managed_bytes` before → after | max RSS before → after | output SHA-256 |
| --- | ---: | ---: | ---: | ---: | --- |
| 24 MP 8-bit AVIF → WebP 1:1 (`t1`) | 164,079 → 8,885 (−94.6 %, 18.5×) | 774,270 → 606,723 (−21.6 %) | 199.7 → 168.0 MB (−30.2 MiB) | 309.3 → 308.7 MiB | identical |
| 24 MP 8-bit AVIF → WebP 1:1 (`t0`) | 25,848 → 8,804 (−65.9 %, 2.9×) | 489,719 → 457,329 (−6.6 %) | 212.1 → 168.0 MB (−42.0 MiB) | 333.9 → 319.4 MiB (−14.5 MiB) | identical |
| 12 MP 16-bit JXL → JXL 1:1 (`t1`) | 850,336 → 34,407 (−96.0 %, 24.7×) | 2,234,323 → 1,435,981 (−35.7 %) | 778.7 → 778.7 MB | 681.5 → 694.9 MiB | identical |
| 12 MP 16-bit JXL → JXL 1:1 (`t0`) | 75,016 → 33,497 (−55.3 %, 2.2×) | 1,452,269 → 1,419,498 (−2.3 %) | 778.7 → 778.7 MB | 664.4 → 666.8 MiB | identical |
| `d16-2049x40-a1-lossless.jxl` 1:1 (`t1`) | 6,303 → 275 (−95.6 %, 22.9×) | 21,894 → 15,028 (−31.4 %) | 21.2 → 21.2 MB | 15.8 → 14.5 MiB (−1.3 MiB) | identical |

## Amendment log

- 2026-11-05 — Initial proposal: `f32` as the working type, a Q1.14 fixed-point path for opaque 8-bit sRGB and Display P3 input with a per-plan integer interval bound, exact lookup tables, and a testable bound, backed by the precision experiment.
- 2026-11-05 — The oracle's transparent-pixel rule is fixed: `final_rgba` now emits RGB 0 whenever the output alpha byte is 0, as ADR-0014 requires, and the experiment's hand copy and `f32` chain apply the same rule. A rerun of the experiment reproduced every measured value of the committed results; only timings differed, so the result files were kept. Rule E remains as a test of the fast paths.
- 2026-11-05 — First implementation: `src/fast_execute.rs` (scalar `f32`, rule 1 and the `f32` tables of rule 3) passes rules A to E in `fast_execute_tests` against the `f64` oracle on 16 inputs, 11 geometries and both filters. Rule E is restated per executor: at an exact alpha tie the oracle may round to 0 and the `f32` path to 1, which rule A permits; each side must emit RGB 0 behind its own zero alpha, and RGB is compared only where both alphas are nonzero.
- 2026-11-06 — **Q1.14 dropped (rule 2 withdrawn).** The first review trigger fired: work package 3 measured the Q1.14 chain below 1.25× the `f32` chain on every hardware class. Prototype kernels (`i16` samples and weights, `i32` sums, one rounding shift; plain loops vectorized by LLVM inside each level's `#[target_feature]` entry, bit-identical across levels) were timed by `crates/ferrisma-kernels/examples/bench.rs` on a 6000×4000 to 1600×1067 Lanczos3 resample, horizontal + vertical ms, f32 against Q1.14: Apple M1 NEON 61.7 + 13.6 against 61.9 + 11.3 (1.03×); x86-64 hosted runner ([run 42179398696](https://github.com/sebastian-software/ferrisma/actions/runs/42179398696)) AVX2 53.2 + 15.6 against 126.9 + 8.0 (0.51×) and AVX-512 37.7 + 13.2 against 83.8 + 5.9 (0.57×); `ubuntu-24.04-arm` (Neoverse with SVE2) NEON 78.3 + 20.1 against 124.7 + 14.5 (0.71×). The vertical integer pass was faster everywhere (1.2–2.2×), the horizontal integer pass slower everywhere: one tap per pixel costs one instruction in both formats on NEON, and the `f32` FMA kernels with two or four taps per vector beat LLVM's integer code on x86-64. A hand-written `vpmaddwd` horizontal kernel was not tried; it is the condition for reopening rule 2 (new review trigger: a hand-written integer horizontal kernel measured faster than the `f32` one on some hardware class). Rule 1 (`f32` everywhere), rule 3's `f32` tables and the bound of rule 4 for `f32` paths stand and are implemented; the Q1.14 tables, eligibility rules and report fields are not built. The precision experiment stays as evidence.
- 2026-11-15 — Output depth follows the input (owner decision): one quantization for every depth, `u16` rows above 8 bits, per-depth shares of rules B to D with rule A unchanged, the public depth fields and the backend additions; see the amendment above. The 8-bit path is unchanged bit for bit.
- 2026-11-26 — Finalized as a living record, owner decision: status Accepted; stale passages corrected in place, dropped decisions marked, open questions limited to open issues.
- 2026-11-26 — Added 1:1 sRGB identity pass-through in `fast_execute` and zero-allocation boundary `quality::footprint` / scratch-reusing `TapTable::build` (#284); see the amendment above for semantics and reproducible benchmarks.
