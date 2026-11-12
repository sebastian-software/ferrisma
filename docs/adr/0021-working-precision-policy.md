# ADR-0021: Working precision policy for the linear-light resize chain

- **Status:** Experimental (rule 1 and the `f32` tables implemented; rule 2, the Q1.14 path, dropped by its own review trigger)
- **Date:** 2026-11-05
- **Last reviewed:** 2026-11-06
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

Every fast path works in `f32` unless rule 2 applies. `f64` remains only in the scalar reference executor and in tests. Specifically:

- 8-bit input decodes through the 256-entry `f32` table (rule 3); F32 input (JPEG XL float) decodes with the extended sign-preserving sRGB EOTF computed in `f32` per sample; Display P3 input adds the folded P3 to sRGB matrix in `f32`.
- Filter weights are normalized in `f64` and stored as `f32`. Kernels may sum taps in any order and with any fusion; `f32` paths are therefore *not* required to be bit-identical to each other, only bounded against the oracle (rule 4).
- Straight alpha is premultiplied in `f32` linear light, filtered as a fourth channel, and divided out in `f32` with the oracle's `alpha > 1e-12` rule.
- Output encodes through the exact `f32` encoder of rule 3 after clamping to `[0, 1]`.

### 2. The Q1.14 fixed-point path and when the planner may choose it

**Representation.** Linear light is stored as `i16` with 14 fractional bits (Q1.14: `1.0 = 16384`, representable range `[-2, 2)`), for both the decoded source samples and the horizontal intermediate rows. Weights are `i16` with a per-axis precision `p` (14, 15 or 16 fractional bits), each row of integer weights summing to exactly `2^p` (the largest tap absorbs the rounding difference), so that a constant region reproduces its code exactly. Accumulation is `i32`. Each pass rounds half up: add `2^(p-1)`, arithmetic shift right by `p`. The vertical result is clamped to `[0, 16384]` and encoded through the 16385-entry table of rule 3. Display P3 decodes through seven folded product tables in `i32` with 30 fractional bits (the two zero matrix entries need none), summed and rounded once to Q1.14. The kernel table of ADR-0020 lists `u16` as the linear integer sample type; this ADR fixes it as signed `i16` Q1.14, because Display P3 decodes to negative values and Lanczos3 rows overshoot in both directions.

**Eligibility.** The planner may choose Q1.14 only when all of the following hold; otherwise the plan uses `f32`:

1. Source samples are `U8` (not `F32`, not 10- or 16-bit).
2. The source color is CICP sRGB `[1, 13, 0, 1]` or Display P3 `[12, 13, 0, 1]`; the output is sRGB8.
3. The layout is opaque RGB. Any alpha channel disqualifies the plan: the straight-alpha division amplifies the half-LSB rounding of premultiplied RGB and alpha by `1 / alpha`, and the experiment measures up to 31 codes of error at alpha 1 to 25 (section E8), even at identity geometry.
4. The per-plan interval bound holds. For each axis the planner quantizes the weights at the largest `p <= 16` whose largest `|weight|` fits `i16`, then lowers `p` (not below 14) until the accumulator interval, computed by interval arithmetic on the *integer* weights over the integer input range, fits `i32`. The input range of the horizontal pass is the decode table range (`[0, 16384]` for sRGB, `[-3687, 20071]` for Display P3, from the cube corners widened by one LSB); its output range including rounding must fit `i16` rows (`|value| <= 32767`) and becomes the input range of the vertical pass, whose accumulator must fit `i32` at its own `p`. If no `p >= 14` satisfies this, or the row range does not fit, the plan uses `f32`.
5. The filter's taps are known at plan time (true for Area and Lanczos3).

For the M1 filters and both color encodings rule 4 never rejects a plan in the experiment: the largest planner bound seen is `0.965 * 2^31` (Display P3 at 2x with `p = 16`), the largest row bound `0.810 * 32767`; the observed peaks are `0.754 * 2^31` and `0.773 * 32767`. Precision pairs chosen over the 312 fixed-point runs: `(16, 16)` 195 times, `(15, 15)` 39, `(14, 16)` 26, `(14, 14)` 52 (identity and upscale, where the largest weight is 1.0 or more).

**Determinism.** Integer arithmetic is associative, so every Q1.14 kernel (scalar, NEON, SVE2, AVX2, AVX-512, any tile shape or thread count) must produce bit-identical output to the scalar Q1.14 implementation, provided the kernel contract of ADR-0020 preserves the rounding points (one rounding after each pass, none inside the accumulation). This is the "bit-exact for integer paths" of the M2 acceptance contract: bit-exact across implementations, and bounded (rule 4) against the `f64` oracle. Bit-exactness against the oracle is impossible for any path with different rounding points, and the milestone wording should be amended to say so (follow-up outside this ADR).

### 3. Lookup tables

| Table | Shape | Definition | Verification |
| --- | --- | --- | --- |
| sRGB8 to linear, `f32` | 256 x `f32` (1 KiB) | `decode_srgb(code / 255)` computed in `f64`, cast to `f32` | Entry-wise equality with the `f64` reference cast to `f32` (the measured relative error is `5.2e-8`, pure `f32` rounding) |
| sRGB8 to linear, Q1.14 | 256 x `i16` (512 B) | `round(decode_srgb(code / 255) * 16384)` | Entry-wise equality with the reference; 256 distinct values (codes 0 and 1 map to 0 and 5); max error `3.05e-5` linear = 0.10 dark-end code steps; decode followed by the exact encoder round-trips all 256 codes |
| Display P3 8-bit to linear sRGB | 7 x 256 x `i32` (Q.30, 7 KiB) and 7 x 256 x `f32` | `m[o][i] * decode_srgb(code / 255)` for the seven non-zero entries of the folded CSS Color 4 matrix product; summed per output channel, rounded once | Exhaustive over all `2^24` triples: the Q1.14 result is within 0.5 LSB of the unrounded `f64` value, and equals `round(f64 * 2^14)` except for 113 of 50,331,648 entries that differ by one LSB (values within `9e-8` of a rounding midpoint); the `f32` result is within `1.22e-7` linear |
| Linear Q0.14 to sRGB8 | 16385 x `u8` (16 KiB) | `encode_linear_srgb8(q / 16384)` for `q` in `0..=16384` | Exact by construction for every value the Q1.14 path can produce; verified exhaustively over the 16385 inputs |
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

The processing report names, per resize stage, `f32` or `q1.14` with the chosen `(p_h, p_v)`, and the reason when `f32` was chosen for a Q1.14-eligible source (alpha, F32 input, bound rejected). This is acceptance item 5 of the M2 milestone.

## Consequences

+ One testable bound against one oracle for every fast path; the integer path is additionally deterministic across machines, which the `f32` path cannot be.
+ No `powf` per sample for 8-bit input: four lookups and a few integer or float operations per source sample, two lookups and a compare per output sample.
+ Display P3 costs seven lookups and adds instead of a matrix multiply, without a separate pass.
+ The planner's interval bound is cheap (one pass over the quantized taps) and turns overflow safety into a per-plan proof instead of a global assumption.
+ Tables total under 30 KiB and are verified exhaustively, so a wrong entry cannot survive the tests.
- Two working types plus the oracle to maintain; the Q1.14 kernels are a second kernel set in ADR-0020.
- Q1.14 is limited to opaque 8-bit sRGB and Display P3 input with sRGB8 output. Alpha, F32, 10-bit and HDR go to `f32`.
- About 4 % of samples in dark regions (up to 9 % at single geometries) and about 0.5 % of samples on full-range content differ from the oracle by one code in Q1.14.
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

**Before Experimental** (work package 3): the kernel crate implements both paths with the representation of rule 2 and the tables of rule 3; a differential test suite encodes rules A to F verbatim, with the experiment's generators as fixtures; the planner implements the eligibility rules and the plan report of rule 5.

**Before Accepted:** the end-to-end benchmark of work package 0 shows the Q1.14 gain over `f32` on the three hardware classes; the benchmark corpus is added to the bound's input set and its Q1.14 threshold recorded; SSIMULACRA2 and Butteraugli on the AVIF output confirm that one-code differences do not move the quality-matched comparison.

## Review when

- Work package 3 measures the fused Q1.14 chain at less than 1.25x the `f32` chain's throughput on every hardware class: drop the Q1.14 path and its kernels, keep `f32` and the oracle.
- A source with more than 8 bits per sample (AVIF 10-bit, JPEG XL 16-bit) should run in fixed point: Q1.14 cannot resolve a 10-bit dark step (`7.6e-5` linear is 1.2 LSB), so this needs a new representation and its own experiment.
- The output is not sRGB8 (10-bit AVIF, 16-bit, float output) or the input is PQ/HLG: the encoder tables and the headroom analysis are sRGB8-specific.
- A new filter or stage with larger negative lobes (sharpening, Lanczos with more lobes) enters the chain: the per-plan bound still applies, but the row format may reject plans, and the gain constants in this ADR must be re-measured.
- ADR-0023 produces decoder-side scaled samples in another format, or the gamma-domain native path needs a precision statement.
- ADR-0024's GPU path wants `f16` storage or a different accumulation: it needs its own bound, measured by the same program.
- The oracle changes (transfer constants, matrices, the alpha rule of `final_rgba`): regenerate the tables and rerun the experiment; the hand-copied reference in the experiment must be updated in the same change.
- A perceptual metric on the benchmark corpus shows that one-code dark-end differences change encoder decisions or the quality-matched comparison: tighten rule C or narrow Q1.14 eligibility.
- libvips or Sharp change their default precision or add a linear-light default: the matched-quality comparison of M2 must be re-examined.
- Stable Rust gains portable SIMD or `f16` arithmetic that changes the kernel trade-offs in ADR-0020.

## Amendment log

- 2026-11-05 — Initial proposal: `f32` as the working type, a Q1.14 fixed-point path for opaque 8-bit sRGB and Display P3 input with a per-plan integer interval bound, exact lookup tables, and a testable bound, backed by the precision experiment.
- 2026-11-05 — The oracle's transparent-pixel rule is fixed: `final_rgba` now emits RGB 0 whenever the output alpha byte is 0, as ADR-0014 requires, and the experiment's hand copy and `f32` chain apply the same rule. A rerun of the experiment reproduced every measured value of the committed results; only timings differed, so the result files were kept. Rule E remains as a test of the fast paths.
- 2026-11-05 — First implementation: `src/fast_execute.rs` (scalar `f32`, rule 1 and the `f32` tables of rule 3) passes rules A to E in `fast_execute_tests` against the `f64` oracle on 16 inputs, 11 geometries and both filters. Rule E is restated per executor: at an exact alpha tie the oracle may round to 0 and the `f32` path to 1, which rule A permits; each side must emit RGB 0 behind its own zero alpha, and RGB is compared only where both alphas are nonzero.
- 2026-11-06 — **Q1.14 dropped (rule 2 withdrawn).** The first review trigger fired: work package 3 measured the Q1.14 chain below 1.25× the `f32` chain on every hardware class. Prototype kernels (`i16` samples and weights, `i32` sums, one rounding shift; plain loops vectorized by LLVM inside each level's `#[target_feature]` entry, bit-identical across levels) were timed by `crates/ferrisma-kernels/examples/bench.rs` on a 6000×4000 to 1600×1067 Lanczos3 resample, horizontal + vertical ms, f32 against Q1.14: Apple M1 NEON 61.7 + 13.6 against 61.9 + 11.3 (1.03×); x86-64 hosted runner ([run 42179398696](https://github.com/sebastian-software/ferrisma/actions/runs/42179398696)) AVX2 53.2 + 15.6 against 126.9 + 8.0 (0.51×) and AVX-512 37.7 + 13.2 against 83.8 + 5.9 (0.57×); `ubuntu-24.04-arm` (Neoverse with SVE2) NEON 78.3 + 20.1 against 124.7 + 14.5 (0.71×). The vertical integer pass was faster everywhere (1.2–2.2×), the horizontal integer pass slower everywhere: one tap per pixel costs one instruction in both formats on NEON, and the `f32` FMA kernels with two or four taps per vector beat LLVM's integer code on x86-64. A hand-written `vpmaddwd` horizontal kernel was not tried; it is the condition for reopening rule 2 (new review trigger: a hand-written integer horizontal kernel measured faster than the `f32` one on some hardware class). Rule 1 (`f32` everywhere), rule 3's `f32` tables and the bound of rule 4 for `f32` paths stand and are implemented; the Q1.14 tables, eligibility rules and report fields are not built. The precision experiment stays as evidence.
