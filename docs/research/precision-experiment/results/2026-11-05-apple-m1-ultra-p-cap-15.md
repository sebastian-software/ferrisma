# ADR-0021 precision experiment

Parameters: chain images 1024x768, Q1.14 working format, coefficient precision cap 15, exact-encoder buckets 4096, exhaustive f32 encoder check off.

Tables built in 0.000 s.

## E1 Decode tables (exhaustive, 256 codes)

| Table | Result |
| --- | --- |
| f32 256-entry sRGB8 -> linear | max relative error vs f64 5.209e-8 (f32 rounding only) |
| Q1.14 256-entry sRGB8 -> linear | max abs error 3.047e-5 linear = 0.1004 dark-end code steps; max relative error 5.432e-3 |
| Q1.14 decode then exact Q0.14 encode | round-trip failures: 0 of 256 |
| Distinct Q1.14 values for 256 codes | 256 (codes 0 and 1 map to 0 and 5) |

## E2 Display P3 8-bit -> linear sRGB (exhaustive, 2^24 triples, 0.16 s)

| Quantity | R | G | B |
| --- | --- | --- | --- |
| max linear sRGB value | 1.2249 | 1.0421 | 1.0983 |
| min linear sRGB value | -0.2249 | -0.0421 | -0.0983 |

| Path | Result |
| --- | --- |
| Q.30 folded product tables, summed, rounded once to Q1.14 | max distance to the unrounded f64 value 0.5000 LSB; entries not equal to round(f64 * 2^14): 113 off by 1, 0 off by 2+ (of 50331648) |
| f32 folded product tables, summed in f32 | max abs error vs f64 1.223e-7 linear |

## E3 Linear -> sRGB8 encoder shapes (dense sweeps)

Full sweep: 2^24 points uniformly spaced in [0, 1]. Dark sweep: 2^20 points in [0, 0.01] (codes 0 to about 25).

| Encoder | Full sweep mismatches | Full max | Dark sweep mismatches | Dark max |
| --- | --- | --- | --- | --- |
| nearest index, 2^12+1 entries (5 KiB), no interpolation | 259173 (1.5448%) | 1 | 162400 (15.4877%) | 1 |
| nearest index, 2^14+1 entries (17 KiB), no interpolation | 68481 (0.4082%) | 1 | 33064 (3.1532%) | 1 |
| nearest index, 2^16+1 entries (65 KiB), no interpolation | 15315 (0.0913%) | 1 | 9602 (0.9157%) | 1 |
| exact f32: 4097-entry bucket table + 257 f32 thresholds (5 KiB) | 0 | 0 | 0 | 0 |

Sweeps took 0.44 s.

## E5 f32 `powf` decode for F32 input (2^24 points in [-0.2, 1.5])

max abs error 5.559e-7 linear (0.00183 dark-end code steps), max relative error 3.947e-7.

## E6 Fixed-point premultiply (exhaustive, 256 codes x 256 alphas)

| Operation | max |error| vs round(f64 * 2^14) |
| --- | --- |
| `(q14 * a + 127) div 255` | 1 LSB |
| `(q14 * (a * 257) + 2^15) >> 16` (shift instead of divide) | 1 LSB |
| alpha `(a * 2^14 + 127) / 255` | 0 LSB |

## E7 Filter gain and coefficient precision

Gain G = largest sum of positive normalized weights of any output position and N the largest sum of negative weights (together they bound the overshoot of one pass); max |w| and the accumulator bound decide the adaptive coefficient precision p (cap 15).

| Filter | Sizes | G (max positive sum) | N (max negative sum) | max |w| | where |
| --- | --- | --- | --- | --- | --- |
| Lanczos3 | all input x output sizes in 1..=96 | 1.28571 | 0.28571 | 1.28240 | G at input 4, output 5, position 2 |
| Area | all input x output sizes in 1..=96 | 1.00000 | 0.00000 | 1.00000 | G at input 81, output 1, position 0 |

| Filter | Ratio (8192 -> n) | taps per output | G | max |w| | adaptive p |
| --- | --- | --- | --- | --- | --- |
| Lanczos3 | 2x | 12 | 1.20202 | 0.47167 | 15 |
| Lanczos3 | 4x | 24 | 1.18545 | 0.26051 | 15 |
| Lanczos3 | 8x | 48 | 1.18138 | 0.13347 | 15 |
| Lanczos3 | 16x | 96 | 1.18036 | 0.06714 | 15 |
| Lanczos3 | 32x | 192 | 1.18011 | 0.03362 | 15 |
| Lanczos3 | 64x | 384 | 1.18005 | 0.01682 | 15 |
| Lanczos3 | 128x | 768 | 1.18003 | 0.00841 | 15 |
| Lanczos3 | 256x | 1536 | 1.18003 | 0.00420 | 15 |
| Lanczos3 | 512x | 3072 | 1.18003 | 0.00210 | 15 |
| Lanczos3 | 1024x | 6144 | 1.18003 | 0.00105 | 15 |
| Lanczos3 | 2048x | 8192 | 1.14330 | 0.00053 | 15 |
| Lanczos3 | 4096x | 8192 | 1.05852 | 0.00026 | 15 |
| Lanczos3 | 8192x | 8192 | 1.00000 | 0.00014 | 15 |
| Area | 2x | 2 | 1.00000 | 0.50000 | 15 |
| Area | 4x | 4 | 1.00000 | 0.25000 | 15 |
| Area | 8x | 8 | 1.00000 | 0.12500 | 15 |
| Area | 16x | 16 | 1.00000 | 0.06250 | 15 |
| Area | 32x | 32 | 1.00000 | 0.03125 | 15 |
| Area | 64x | 64 | 1.00000 | 0.01562 | 15 |
| Area | 128x | 128 | 1.00000 | 0.00781 | 15 |
| Area | 256x | 256 | 1.00000 | 0.00391 | 15 |
| Area | 512x | 512 | 1.00000 | 0.00195 | 15 |
| Area | 1024x | 1024 | 1.00000 | 0.00098 | 15 |
| Area | 2048x | 2048 | 1.00000 | 0.00049 | 15 |
| Area | 4096x | 4096 | 1.00000 | 0.00024 | 15 |
| Area | 8192x | 8192 | 1.00000 | 0.00012 | 15 |

### Derived headroom (Lanczos3 worst gain G = 1.28571, N = 0.28571; Area gain 1.0000)

Exhaustive E2 range of decoded Display P3: [-0.2249, 1.2249]; corner-derived range used by the planner rule: [-0.2249, 1.2249] = Q1.14 [-3687, 20071]. This table applies the worst gain of any geometry to both passes; the planner rule in E8 instead bounds each plan exactly from its own quantized integer weights.

| Input | decoded range | after horizontal pass | max |row| in Q1.14 (limit 32767) | after vertical pass | max |acc| / 2^31 at p = 16 | at p = 15 |
| --- | --- | --- | --- | --- | --- | --- |
| sRGB 8-bit | [0.0000, 1.0000] | [-0.2857, 1.2857] | 21065 | [-0.7347, 1.7347] | 0.8673 | 0.4337 |
| Display P3 8-bit | [-0.2249, 1.2249] | [-0.6392, 1.6392] | 26857 | [-1.2902, 2.2902] | 1.1451 | 0.5725 |

## E8 Full chains against the f64 reference

Every cell compares sRGB8 output samples per channel with the f64 reference: max |delta| in codes, share of exactly equal samples, count off by 1, count off by 2 or more. For RGBA input, RGB is compared where the reference output alpha is > 0; alpha is compared separately; RGB differences under reference alpha 0 are counted as invisible.

### noise (U8 RGB sRGB)

| Filter | Geometry | f32 RGB | fixed p (h, v) | fixed RGB | acc bound / 2^31 | peak acc / 2^31 | row bound / 32767 | peak row / 32767 | s |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Lanczos3 | identity | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 14, 14 | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 0.125 | 0.125 | 0.500 | 0.500 | 0.09 |
| Lanczos3 | up 1.5x | max 1 / 99.9996% exact / 23 off-1 / 0 off-2+ | 14, 14 | max 1 / 99.2443% exact / 40115 off-1 / 0 off-2+ | 0.211 | 0.175 | 0.636 | 0.621 | 0.22 |
| Lanczos3 | down 1.5x | max 1 / 99.9995% exact / 5 off-1 / 0 off-2+ | 15, 15 | max 1 / 99.4068% exact / 6214 off-1 / 0 off-2+ | 0.366 | 0.289 | 0.600 | 0.578 | 0.09 |
| Lanczos3 | down 2x | max 1 / 99.9993% exact / 4 off-1 / 0 off-2+ | 15, 15 | max 1 / 99.3946% exact / 3571 off-1 / 0 off-2+ | 0.371 | 0.280 | 0.601 | 0.560 | 0.10 |
| Lanczos3 | down 2.37x | max 1 / 99.9990% exact / 4 off-1 / 0 off-2+ | 15, 15 | max 1 / 99.5082% exact / 2065 off-1 / 0 off-2+ | 0.362 | 0.270 | 0.595 | 0.541 | 0.07 |
| Lanczos3 | down 8x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 15, 15 | max 1 / 99.5117% exact / 180 off-1 / 0 off-2+ | 0.357 | 0.250 | 0.591 | 0.500 | 0.05 |
| Lanczos3 | down 16x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 15, 15 | max 1 / 99.5985% exact / 37 off-1 / 0 off-2+ | 0.356 | 0.250 | 0.590 | 0.500 | 0.05 |
| Lanczos3 | down 32x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 15, 15 | max 1 / 99.6528% exact / 8 off-1 / 0 off-2+ | 0.356 | 0.250 | 0.590 | 0.500 | 0.04 |
| Lanczos3 | down 64x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 15, 15 | max 1 / 99.8264% exact / 1 off-1 / 0 off-2+ | 0.356 | 0.250 | 0.590 | 0.500 | 0.04 |
| Lanczos3 | down 128x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 15, 15 | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 0.356 | 0.250 | 0.590 | 0.500 | 0.06 |
| Lanczos3 | down to 2x2 | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 15, 15 | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 0.281 | 0.250 | 0.529 | 0.500 | 0.04 |
| Lanczos3 | aniso 1x/16x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 14, 15 | max 1 / 99.0587% exact / 1388 off-1 / 0 off-2+ | 0.295 | 0.250 | 0.500 | 0.500 | 0.07 |
| Area | identity | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 14, 14 | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 0.125 | 0.125 | 0.500 | 0.500 | 0.08 |
| Area | up 1.5x | max 1 / 99.9996% exact / 19 off-1 / 0 off-2+ | 14, 14 | max 1 / 99.3086% exact / 36700 off-1 / 0 off-2+ | 0.125 | 0.125 | 0.500 | 0.500 | 0.15 |
| Area | down 1.5x | max 1 / 99.9999% exact / 1 off-1 / 0 off-2+ | 15, 15 | max 1 / 99.4980% exact / 5259 off-1 / 0 off-2+ | 0.250 | 0.250 | 0.500 | 0.500 | 0.06 |
| Area | down 2x | max 1 / 99.9995% exact / 3 off-1 / 0 off-2+ | 15, 15 | max 1 / 99.2138% exact / 4637 off-1 / 0 off-2+ | 0.250 | 0.250 | 0.500 | 0.500 | 0.05 |
| Area | down 2.37x | max 1 / 99.9995% exact / 2 off-1 / 0 off-2+ | 15, 15 | max 1 / 99.5580% exact / 1856 off-1 / 0 off-2+ | 0.250 | 0.250 | 0.500 | 0.500 | 0.04 |
| Area | down 8x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 15, 15 | max 1 / 99.6609% exact / 125 off-1 / 0 off-2+ | 0.250 | 0.250 | 0.500 | 0.500 | 0.03 |
| Area | down 16x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 15, 15 | max 1 / 99.7504% exact / 23 off-1 / 0 off-2+ | 0.250 | 0.250 | 0.500 | 0.500 | 0.03 |
| Area | down 32x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 15, 15 | max 1 / 99.7396% exact / 6 off-1 / 0 off-2+ | 0.250 | 0.250 | 0.500 | 0.500 | 0.03 |
| Area | down 64x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 15, 15 | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 0.250 | 0.250 | 0.500 | 0.500 | 0.03 |
| Area | down 128x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 15, 15 | max 1 / 99.3056% exact / 1 off-1 / 0 off-2+ | 0.250 | 0.250 | 0.500 | 0.500 | 0.03 |
| Area | down to 2x2 | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 15, 15 | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 0.250 | 0.250 | 0.500 | 0.500 | 0.03 |
| Area | aniso 1x/16x | max 1 / 99.9993% exact / 1 off-1 / 0 off-2+ | 14, 15 | max 1 / 99.6338% exact / 540 off-1 / 0 off-2+ | 0.250 | 0.250 | 0.500 | 0.500 | 0.04 |

### dark-noise (codes 0-24) (U8 RGB sRGB)

| Filter | Geometry | f32 RGB | fixed p (h, v) | fixed RGB | acc bound / 2^31 | peak acc / 2^31 | row bound / 32767 | peak row / 32767 | s |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Lanczos3 | identity | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 14, 14 | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 0.125 | 0.125 | 0.500 | 0.500 | 0.11 |
| Lanczos3 | up 1.5x | max 1 / 99.9999% exact / 4 off-1 / 0 off-2+ | 14, 14 | max 1 / 94.8489% exact / 273444 off-1 / 0 off-2+ | 0.211 | 0.125 | 0.636 | 0.500 | 0.23 |
| Lanczos3 | down 1.5x | max 1 / 99.9999% exact / 1 off-1 / 0 off-2+ | 15, 15 | max 1 / 95.2138% exact / 50138 off-1 / 0 off-2+ | 0.366 | 0.250 | 0.600 | 0.500 | 0.10 |
| Lanczos3 | down 2x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 15, 15 | max 1 / 95.2779% exact / 27852 off-1 / 0 off-2+ | 0.371 | 0.250 | 0.601 | 0.500 | 0.09 |
| Lanczos3 | down 2.37x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 15, 15 | max 1 / 95.2063% exact / 20129 off-1 / 0 off-2+ | 0.362 | 0.250 | 0.595 | 0.500 | 0.08 |
| Lanczos3 | down 8x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 15, 15 | max 1 / 93.3757% exact / 2442 off-1 / 0 off-2+ | 0.357 | 0.250 | 0.591 | 0.500 | 0.05 |
| Lanczos3 | down 16x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 15, 15 | max 1 / 92.9905% exact / 646 off-1 / 0 off-2+ | 0.356 | 0.250 | 0.590 | 0.500 | 0.05 |
| Lanczos3 | down 32x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 15, 15 | max 1 / 97.3958% exact / 60 off-1 / 0 off-2+ | 0.356 | 0.250 | 0.590 | 0.500 | 0.06 |
| Lanczos3 | down 64x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 15, 15 | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 0.356 | 0.250 | 0.590 | 0.500 | 0.05 |
| Lanczos3 | down 128x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 15, 15 | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 0.356 | 0.250 | 0.590 | 0.500 | 0.05 |
| Lanczos3 | down to 2x2 | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 15, 15 | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 0.281 | 0.250 | 0.529 | 0.500 | 0.03 |
| Lanczos3 | aniso 1x/16x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 14, 15 | max 1 / 95.1172% exact / 7200 off-1 / 0 off-2+ | 0.295 | 0.250 | 0.500 | 0.500 | 0.07 |
| Area | identity | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 14, 14 | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 0.125 | 0.125 | 0.500 | 0.500 | 0.10 |
| Area | up 1.5x | max 1 / 99.8303% exact / 9009 off-1 / 0 off-2+ | 14, 14 | max 1 / 94.9190% exact / 269718 off-1 / 0 off-2+ | 0.125 | 0.125 | 0.500 | 0.500 | 0.18 |
| Area | down 1.5x | max 1 / 99.9985% exact / 16 off-1 / 0 off-2+ | 15, 15 | max 1 / 95.4992% exact / 47148 off-1 / 0 off-2+ | 0.250 | 0.250 | 0.500 | 0.500 | 0.06 |
| Area | down 2x | max 1 / 99.4071% exact / 3497 off-1 / 0 off-2+ | 15, 15 | max 1 / 93.2766% exact / 39656 off-1 / 0 off-2+ | 0.250 | 0.250 | 0.500 | 0.500 | 0.05 |
| Area | down 2.37x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 15, 15 | max 1 / 95.2839% exact / 19803 off-1 / 0 off-2+ | 0.250 | 0.250 | 0.500 | 0.500 | 0.05 |
| Area | down 8x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 15, 15 | max 1 / 93.5981% exact / 2360 off-1 / 0 off-2+ | 0.250 | 0.250 | 0.500 | 0.500 | 0.03 |
| Area | down 16x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 15, 15 | max 1 / 92.3503% exact / 705 off-1 / 0 off-2+ | 0.250 | 0.250 | 0.500 | 0.500 | 0.03 |
| Area | down 32x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 15, 15 | max 1 / 96.4844% exact / 81 off-1 / 0 off-2+ | 0.250 | 0.250 | 0.500 | 0.500 | 0.03 |
| Area | down 64x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 15, 15 | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 0.250 | 0.250 | 0.500 | 0.500 | 0.03 |
| Area | down 128x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 15, 15 | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 0.250 | 0.250 | 0.500 | 0.500 | 0.03 |
| Area | down to 2x2 | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 15, 15 | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 0.250 | 0.250 | 0.500 | 0.500 | 0.03 |
| Area | aniso 1x/16x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 14, 15 | max 1 / 95.2603% exact / 6989 off-1 / 0 off-2+ | 0.250 | 0.250 | 0.500 | 0.500 | 0.04 |

### bright-noise (codes 231-255) (U8 RGB sRGB)

| Filter | Geometry | f32 RGB | fixed p (h, v) | fixed RGB | acc bound / 2^31 | peak acc / 2^31 | row bound / 32767 | peak row / 32767 | s |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Lanczos3 | identity | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 14, 14 | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 0.125 | 0.125 | 0.500 | 0.500 | 0.12 |
| Lanczos3 | up 1.5x | max 1 / 99.9991% exact / 49 off-1 / 0 off-2+ | 14, 14 | max 1 / 99.6847% exact / 16740 off-1 / 0 off-2+ | 0.211 | 0.134 | 0.636 | 0.527 | 0.22 |
| Lanczos3 | down 1.5x | max 1 / 99.9991% exact / 9 off-1 / 0 off-2+ | 15, 15 | max 1 / 99.6971% exact / 3173 off-1 / 0 off-2+ | 0.366 | 0.259 | 0.600 | 0.518 | 0.07 |
| Lanczos3 | down 2x | max 1 / 99.9997% exact / 2 off-1 / 0 off-2+ | 15, 15 | max 1 / 99.7074% exact / 1726 off-1 / 0 off-2+ | 0.371 | 0.257 | 0.601 | 0.514 | 0.10 |
| Lanczos3 | down 2.37x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 15, 15 | max 1 / 99.7209% exact / 1172 off-1 / 0 off-2+ | 0.362 | 0.255 | 0.595 | 0.511 | 0.07 |
| Lanczos3 | down 8x | max 1 / 99.9973% exact / 1 off-1 / 0 off-2+ | 15, 15 | max 1 / 99.6365% exact / 134 off-1 / 0 off-2+ | 0.357 | 0.250 | 0.591 | 0.500 | 0.05 |
| Lanczos3 | down 16x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 15, 15 | max 1 / 99.6202% exact / 35 off-1 / 0 off-2+ | 0.356 | 0.250 | 0.590 | 0.500 | 0.05 |
| Lanczos3 | down 32x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 15, 15 | max 1 / 99.7830% exact / 5 off-1 / 0 off-2+ | 0.356 | 0.250 | 0.590 | 0.500 | 0.05 |
| Lanczos3 | down 64x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 15, 15 | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 0.356 | 0.250 | 0.590 | 0.500 | 0.04 |
| Lanczos3 | down 128x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 15, 15 | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 0.356 | 0.250 | 0.590 | 0.500 | 0.04 |
| Lanczos3 | down to 2x2 | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 15, 15 | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 0.281 | 0.250 | 0.529 | 0.500 | 0.04 |
| Lanczos3 | aniso 1x/16x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 14, 15 | max 1 / 99.7118% exact / 425 off-1 / 0 off-2+ | 0.295 | 0.250 | 0.500 | 0.500 | 0.07 |
| Area | identity | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 14, 14 | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 0.125 | 0.125 | 0.500 | 0.500 | 0.08 |
| Area | up 1.5x | max 1 / 99.9997% exact / 14 off-1 / 0 off-2+ | 14, 14 | max 1 / 99.7581% exact / 12842 off-1 / 0 off-2+ | 0.125 | 0.125 | 0.500 | 0.500 | 0.15 |
| Area | down 1.5x | max 1 / 99.9997% exact / 3 off-1 / 0 off-2+ | 15, 15 | max 1 / 99.7102% exact / 3036 off-1 / 0 off-2+ | 0.250 | 0.250 | 0.500 | 0.500 | 0.06 |
| Area | down 2x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 15, 15 | max 1 / 99.9396% exact / 356 off-1 / 0 off-2+ | 0.250 | 0.250 | 0.500 | 0.500 | 0.05 |
| Area | down 2.37x | max 1 / 99.9998% exact / 1 off-1 / 0 off-2+ | 15, 15 | max 1 / 99.7040% exact / 1243 off-1 / 0 off-2+ | 0.250 | 0.250 | 0.500 | 0.500 | 0.04 |
| Area | down 8x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 15, 15 | max 1 / 99.7314% exact / 99 off-1 / 0 off-2+ | 0.250 | 0.250 | 0.500 | 0.500 | 0.03 |
| Area | down 16x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 15, 15 | max 1 / 99.6636% exact / 31 off-1 / 0 off-2+ | 0.250 | 0.250 | 0.500 | 0.500 | 0.03 |
| Area | down 32x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 15, 15 | max 1 / 99.7830% exact / 5 off-1 / 0 off-2+ | 0.250 | 0.250 | 0.500 | 0.500 | 0.03 |
| Area | down 64x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 15, 15 | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 0.250 | 0.250 | 0.500 | 0.500 | 0.03 |
| Area | down 128x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 15, 15 | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 0.250 | 0.250 | 0.500 | 0.500 | 0.03 |
| Area | down to 2x2 | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 15, 15 | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 0.250 | 0.250 | 0.500 | 0.500 | 0.03 |
| Area | aniso 1x/16x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 14, 15 | max 1 / 99.6996% exact / 443 off-1 / 0 off-2+ | 0.250 | 0.250 | 0.500 | 0.500 | 0.04 |

### gradient (U8 RGB sRGB)

| Filter | Geometry | f32 RGB | fixed p (h, v) | fixed RGB | acc bound / 2^31 | peak acc / 2^31 | row bound / 32767 | peak row / 32767 | s |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Lanczos3 | identity | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 14, 14 | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 0.125 | 0.125 | 0.500 | 0.500 | 0.09 |
| Lanczos3 | up 1.5x | max 1 / 99.8972% exact / 5458 off-1 / 0 off-2+ | 14, 14 | max 1 / 98.5700% exact / 75910 off-1 / 0 off-2+ | 0.211 | 0.125 | 0.636 | 0.500 | 0.21 |
| Lanczos3 | down 1.5x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 15, 15 | max 1 / 99.8577% exact / 1491 off-1 / 0 off-2+ | 0.366 | 0.250 | 0.600 | 0.500 | 0.07 |
| Lanczos3 | down 2x | max 1 / 99.4166% exact / 3441 off-1 / 0 off-2+ | 15, 15 | max 1 / 90.9507% exact / 53375 off-1 / 0 off-2+ | 0.371 | 0.250 | 0.601 | 0.500 | 0.10 |
| Lanczos3 | down 2.37x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 15, 15 | max 1 / 99.6244% exact / 1577 off-1 / 0 off-2+ | 0.362 | 0.250 | 0.595 | 0.500 | 0.07 |
| Lanczos3 | down 8x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 15, 15 | max 1 / 94.0918% exact / 2178 off-1 / 0 off-2+ | 0.357 | 0.250 | 0.591 | 0.500 | 0.05 |
| Lanczos3 | down 16x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 15, 15 | max 1 / 95.1389% exact / 448 off-1 / 0 off-2+ | 0.356 | 0.250 | 0.590 | 0.500 | 0.04 |
| Lanczos3 | down 32x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 15, 15 | max 1 / 97.1354% exact / 66 off-1 / 0 off-2+ | 0.356 | 0.250 | 0.590 | 0.500 | 0.06 |
| Lanczos3 | down 64x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 15, 15 | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 0.356 | 0.250 | 0.590 | 0.500 | 0.04 |
| Lanczos3 | down 128x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 15, 15 | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 0.356 | 0.250 | 0.590 | 0.500 | 0.04 |
| Lanczos3 | down to 2x2 | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 15, 15 | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 0.281 | 0.250 | 0.529 | 0.500 | 0.04 |
| Lanczos3 | aniso 1x/16x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 14, 15 | max 1 / 93.4001% exact / 9732 off-1 / 0 off-2+ | 0.295 | 0.250 | 0.500 | 0.500 | 0.07 |
| Area | identity | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 14, 14 | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 0.125 | 0.125 | 0.500 | 0.500 | 0.08 |
| Area | up 1.5x | max 1 / 99.8785% exact / 6451 off-1 / 0 off-2+ | 14, 14 | max 1 / 98.5446% exact / 77261 off-1 / 0 off-2+ | 0.125 | 0.125 | 0.500 | 0.500 | 0.14 |
| Area | down 1.5x | max 1 / 99.9999% exact / 1 off-1 / 0 off-2+ | 15, 15 | max 1 / 99.8442% exact / 1632 off-1 / 0 off-2+ | 0.250 | 0.250 | 0.500 | 0.500 | 0.06 |
| Area | down 2x | max 1 / 99.5660% exact / 2560 off-1 / 0 off-2+ | 15, 15 | max 1 / 95.6163% exact / 25856 off-1 / 0 off-2+ | 0.250 | 0.250 | 0.500 | 0.500 | 0.04 |
| Area | down 2.37x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 15, 15 | max 1 / 99.9250% exact / 315 off-1 / 0 off-2+ | 0.250 | 0.250 | 0.500 | 0.500 | 0.04 |
| Area | down 8x | max 1 / 99.6528% exact / 128 off-1 / 0 off-2+ | 15, 15 | max 1 / 98.6111% exact / 512 off-1 / 0 off-2+ | 0.250 | 0.250 | 0.500 | 0.500 | 0.03 |
| Area | down 16x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 15, 15 | max 1 / 99.3056% exact / 64 off-1 / 0 off-2+ | 0.250 | 0.250 | 0.500 | 0.500 | 0.03 |
| Area | down 32x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 15, 15 | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 0.250 | 0.250 | 0.500 | 0.500 | 0.03 |
| Area | down 64x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 15, 15 | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 0.250 | 0.250 | 0.500 | 0.500 | 0.03 |
| Area | down 128x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 15, 15 | max 1 / 95.8333% exact / 6 off-1 / 0 off-2+ | 0.250 | 0.250 | 0.500 | 0.500 | 0.03 |
| Area | down to 2x2 | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 15, 15 | max 1 / 58.3333% exact / 5 off-1 / 0 off-2+ | 0.250 | 0.250 | 0.500 | 0.500 | 0.03 |
| Area | aniso 1x/16x | max 1 / 99.9932% exact / 10 off-1 / 0 off-2+ | 14, 15 | max 1 / 98.4063% exact / 2350 off-1 / 0 off-2+ | 0.250 | 0.250 | 0.500 | 0.500 | 0.04 |

### flat-blocks (32px constant blocks) (U8 RGB sRGB)

| Filter | Geometry | f32 RGB | fixed p (h, v) | fixed RGB | acc bound / 2^31 | peak acc / 2^31 | row bound / 32767 | peak row / 32767 | s |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Lanczos3 | identity | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 14, 14 | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 0.125 | 0.125 | 0.500 | 0.500 | 0.11 |
| Lanczos3 | up 1.5x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 14, 14 | max 1 / 99.7933% exact / 10971 off-1 / 0 off-2+ | 0.211 | 0.144 | 0.636 | 0.531 | 0.20 |
| Lanczos3 | down 1.5x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 15, 15 | max 1 / 99.6626% exact / 3534 off-1 / 0 off-2+ | 0.366 | 0.277 | 0.600 | 0.522 | 0.07 |
| Lanczos3 | down 2x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 15, 15 | max 1 / 99.5514% exact / 2646 off-1 / 0 off-2+ | 0.371 | 0.250 | 0.601 | 0.500 | 0.10 |
| Lanczos3 | down 2.37x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 15, 15 | max 1 / 99.3982% exact / 2527 off-1 / 0 off-2+ | 0.362 | 0.274 | 0.595 | 0.517 | 0.07 |
| Lanczos3 | down 8x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 15, 15 | max 1 / 98.7712% exact / 453 off-1 / 0 off-2+ | 0.357 | 0.250 | 0.591 | 0.500 | 0.05 |
| Lanczos3 | down 16x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 15, 15 | max 1 / 98.0360% exact / 181 off-1 / 0 off-2+ | 0.356 | 0.250 | 0.590 | 0.500 | 0.05 |
| Lanczos3 | down 32x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 15, 15 | max 1 / 98.6979% exact / 30 off-1 / 0 off-2+ | 0.356 | 0.250 | 0.590 | 0.500 | 0.04 |
| Lanczos3 | down 64x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 15, 15 | max 1 / 98.9583% exact / 6 off-1 / 0 off-2+ | 0.356 | 0.250 | 0.590 | 0.500 | 0.04 |
| Lanczos3 | down 128x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 15, 15 | max 1 / 96.5278% exact / 5 off-1 / 0 off-2+ | 0.356 | 0.250 | 0.590 | 0.500 | 0.04 |
| Lanczos3 | down to 2x2 | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 15, 15 | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 0.281 | 0.250 | 0.529 | 0.500 | 0.03 |
| Lanczos3 | aniso 1x/16x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 14, 15 | max 1 / 97.6345% exact / 3488 off-1 / 0 off-2+ | 0.295 | 0.250 | 0.500 | 0.500 | 0.07 |
| Area | identity | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 14, 14 | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 0.125 | 0.125 | 0.500 | 0.500 | 0.08 |
| Area | up 1.5x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 14, 14 | max 1 / 99.9678% exact / 1711 off-1 / 0 off-2+ | 0.125 | 0.125 | 0.500 | 0.500 | 0.14 |
| Area | down 1.5x | max 1 / 99.9961% exact / 41 off-1 / 0 off-2+ | 15, 15 | max 1 / 99.9604% exact / 415 off-1 / 0 off-2+ | 0.250 | 0.250 | 0.500 | 0.500 | 0.06 |
| Area | down 2x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 15, 15 | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 0.250 | 0.250 | 0.500 | 0.500 | 0.04 |
| Area | down 2.37x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 15, 15 | max 1 / 99.9409% exact / 248 off-1 / 0 off-2+ | 0.250 | 0.250 | 0.500 | 0.500 | 0.04 |
| Area | down 8x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 15, 15 | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 0.250 | 0.250 | 0.500 | 0.500 | 0.03 |
| Area | down 16x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 15, 15 | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 0.250 | 0.250 | 0.500 | 0.500 | 0.03 |
| Area | down 32x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 15, 15 | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 0.250 | 0.250 | 0.500 | 0.500 | 0.03 |
| Area | down 64x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 15, 15 | max 1 / 97.5694% exact / 14 off-1 / 0 off-2+ | 0.250 | 0.250 | 0.500 | 0.500 | 0.03 |
| Area | down 128x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 15, 15 | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 0.250 | 0.250 | 0.500 | 0.500 | 0.03 |
| Area | down to 2x2 | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 15, 15 | max 1 / 91.6667% exact / 1 off-1 / 0 off-2+ | 0.250 | 0.250 | 0.500 | 0.500 | 0.03 |
| Area | aniso 1x/16x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 14, 15 | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 0.250 | 0.250 | 0.500 | 0.500 | 0.04 |

### checker-1px (U8 RGB sRGB)

| Filter | Geometry | f32 RGB | fixed p (h, v) | fixed RGB | acc bound / 2^31 | peak acc / 2^31 | row bound / 32767 | peak row / 32767 | s |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Lanczos3 | identity | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 14, 14 | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 0.125 | 0.125 | 0.500 | 0.500 | 0.06 |
| Lanczos3 | up 1.5x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 14, 14 | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 0.211 | 0.159 | 0.636 | 0.560 | 0.18 |
| Lanczos3 | down 1.5x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 15, 15 | max 1 / 92.5936% exact / 77586 off-1 / 0 off-2+ | 0.366 | 0.250 | 0.600 | 0.500 | 0.05 |
| Lanczos3 | down 2x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 15, 15 | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 0.371 | 0.250 | 0.601 | 0.500 | 0.05 |
| Lanczos3 | down 2.37x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 15, 15 | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 0.362 | 0.250 | 0.595 | 0.500 | 0.06 |
| Lanczos3 | down 8x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 15, 15 | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 0.357 | 0.250 | 0.591 | 0.500 | 0.03 |
| Lanczos3 | down 16x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 15, 15 | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 0.356 | 0.250 | 0.590 | 0.500 | 0.05 |
| Lanczos3 | down 32x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 15, 15 | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 0.356 | 0.250 | 0.590 | 0.500 | 0.02 |
| Lanczos3 | down 64x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 15, 15 | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 0.356 | 0.250 | 0.590 | 0.500 | 0.02 |
| Lanczos3 | down 128x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 15, 15 | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 0.356 | 0.250 | 0.590 | 0.500 | 0.03 |
| Lanczos3 | down to 2x2 | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 15, 15 | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 0.281 | 0.250 | 0.529 | 0.500 | 0.01 |
| Lanczos3 | aniso 1x/16x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 14, 15 | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 0.295 | 0.250 | 0.500 | 0.500 | 0.05 |
| Area | identity | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 14, 14 | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 0.125 | 0.125 | 0.500 | 0.500 | 0.04 |
| Area | up 1.5x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 14, 14 | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 0.125 | 0.125 | 0.500 | 0.500 | 0.11 |
| Area | down 1.5x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 15, 15 | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 0.250 | 0.250 | 0.500 | 0.500 | 0.04 |
| Area | down 2x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 15, 15 | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 0.250 | 0.250 | 0.500 | 0.500 | 0.02 |
| Area | down 2.37x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 15, 15 | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 0.250 | 0.250 | 0.500 | 0.500 | 0.03 |
| Area | down 8x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 15, 15 | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 0.250 | 0.250 | 0.500 | 0.500 | 0.02 |
| Area | down 16x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 15, 15 | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 0.250 | 0.250 | 0.500 | 0.500 | 0.01 |
| Area | down 32x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 15, 15 | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 0.250 | 0.250 | 0.500 | 0.500 | 0.01 |
| Area | down 64x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 15, 15 | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 0.250 | 0.250 | 0.500 | 0.500 | 0.01 |
| Area | down 128x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 15, 15 | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 0.250 | 0.250 | 0.500 | 0.500 | 0.01 |
| Area | down to 2x2 | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 15, 15 | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 0.250 | 0.250 | 0.500 | 0.500 | 0.01 |
| Area | aniso 1x/16x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 14, 15 | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 0.250 | 0.250 | 0.500 | 0.500 | 0.02 |

### checker-2px (U8 RGB sRGB)

| Filter | Geometry | f32 RGB | fixed p (h, v) | fixed RGB | acc bound / 2^31 | peak acc / 2^31 | row bound / 32767 | peak row / 32767 | s |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Lanczos3 | identity | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 14, 14 | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 0.125 | 0.125 | 0.500 | 0.500 | 0.06 |
| Lanczos3 | up 1.5x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 14, 14 | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 0.211 | 0.193 | 0.636 | 0.611 | 0.14 |
| Lanczos3 | down 1.5x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 15, 15 | max 1 / 99.5624% exact / 4584 off-1 / 0 off-2+ | 0.366 | 0.313 | 0.600 | 0.562 | 0.06 |
| Lanczos3 | down 2x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 15, 15 | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 0.371 | 0.250 | 0.601 | 0.500 | 0.04 |
| Lanczos3 | down 2.37x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 15, 15 | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 0.362 | 0.250 | 0.595 | 0.500 | 0.05 |
| Lanczos3 | down 8x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 15, 15 | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 0.357 | 0.250 | 0.591 | 0.500 | 0.03 |
| Lanczos3 | down 16x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 15, 15 | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 0.356 | 0.250 | 0.590 | 0.500 | 0.02 |
| Lanczos3 | down 32x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 15, 15 | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 0.356 | 0.250 | 0.590 | 0.500 | 0.02 |
| Lanczos3 | down 64x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 15, 15 | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 0.356 | 0.250 | 0.590 | 0.500 | 0.06 |
| Lanczos3 | down 128x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 15, 15 | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 0.356 | 0.250 | 0.590 | 0.500 | 0.03 |
| Lanczos3 | down to 2x2 | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 15, 15 | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 0.281 | 0.250 | 0.529 | 0.500 | 0.02 |
| Lanczos3 | aniso 1x/16x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 14, 15 | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 0.295 | 0.250 | 0.500 | 0.500 | 0.05 |
| Area | identity | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 14, 14 | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 0.125 | 0.125 | 0.500 | 0.500 | 0.05 |
| Area | up 1.5x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 14, 14 | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 0.125 | 0.125 | 0.500 | 0.500 | 0.12 |
| Area | down 1.5x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 15, 15 | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 0.250 | 0.250 | 0.500 | 0.500 | 0.04 |
| Area | down 2x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 15, 15 | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 0.250 | 0.250 | 0.500 | 0.500 | 0.02 |
| Area | down 2.37x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 15, 15 | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 0.250 | 0.250 | 0.500 | 0.500 | 0.03 |
| Area | down 8x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 15, 15 | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 0.250 | 0.250 | 0.500 | 0.500 | 0.01 |
| Area | down 16x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 15, 15 | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 0.250 | 0.250 | 0.500 | 0.500 | 0.01 |
| Area | down 32x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 15, 15 | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 0.250 | 0.250 | 0.500 | 0.500 | 0.01 |
| Area | down 64x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 15, 15 | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 0.250 | 0.250 | 0.500 | 0.500 | 0.01 |
| Area | down 128x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 15, 15 | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 0.250 | 0.250 | 0.500 | 0.500 | 0.01 |
| Area | down to 2x2 | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 15, 15 | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 0.250 | 0.250 | 0.500 | 0.500 | 0.01 |
| Area | aniso 1x/16x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 14, 15 | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 0.250 | 0.250 | 0.500 | 0.500 | 0.02 |

### saturated-edges (U8 RGB sRGB)

| Filter | Geometry | f32 RGB | fixed p (h, v) | fixed RGB | acc bound / 2^31 | peak acc / 2^31 | row bound / 32767 | peak row / 32767 | s |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Lanczos3 | identity | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 14, 14 | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 0.125 | 0.125 | 0.500 | 0.500 | 0.07 |
| Lanczos3 | up 1.5x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 14, 14 | max 1 / 99.9841% exact / 842 off-1 / 0 off-2+ | 0.211 | 0.171 | 0.636 | 0.611 | 0.14 |
| Lanczos3 | down 1.5x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 15, 15 | max 1 / 99.9618% exact / 400 off-1 / 0 off-2+ | 0.366 | 0.299 | 0.600 | 0.550 | 0.05 |
| Lanczos3 | down 2x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 15, 15 | max 1 / 99.9761% exact / 141 off-1 / 0 off-2+ | 0.371 | 0.295 | 0.601 | 0.543 | 0.04 |
| Lanczos3 | down 2.37x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 15, 15 | max 1 / 99.8002% exact / 839 off-1 / 0 off-2+ | 0.362 | 0.290 | 0.595 | 0.570 | 0.04 |
| Lanczos3 | down 8x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 15, 15 | max 1 / 99.4331% exact / 209 off-1 / 0 off-2+ | 0.357 | 0.288 | 0.591 | 0.543 | 0.03 |
| Lanczos3 | down 16x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 15, 15 | max 1 / 97.7431% exact / 208 off-1 / 0 off-2+ | 0.356 | 0.300 | 0.590 | 0.542 | 0.03 |
| Lanczos3 | down 32x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 15, 15 | max 1 / 98.5243% exact / 34 off-1 / 0 off-2+ | 0.356 | 0.288 | 0.590 | 0.570 | 0.02 |
| Lanczos3 | down 64x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 15, 15 | max 1 / 97.7431% exact / 13 off-1 / 0 off-2+ | 0.356 | 0.297 | 0.590 | 0.574 | 0.05 |
| Lanczos3 | down 128x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 15, 15 | max 1 / 97.9167% exact / 3 off-1 / 0 off-2+ | 0.356 | 0.281 | 0.590 | 0.562 | 0.02 |
| Lanczos3 | down to 2x2 | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 15, 15 | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 0.281 | 0.250 | 0.529 | 0.500 | 0.02 |
| Lanczos3 | aniso 1x/16x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 14, 15 | max 1 / 98.6966% exact / 1922 off-1 / 0 off-2+ | 0.295 | 0.285 | 0.500 | 0.500 | 0.05 |
| Area | identity | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 14, 14 | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 0.125 | 0.125 | 0.500 | 0.500 | 0.03 |
| Area | up 1.5x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 14, 14 | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 0.125 | 0.125 | 0.500 | 0.500 | 0.09 |
| Area | down 1.5x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 15, 15 | max 1 / 99.9991% exact / 9 off-1 / 0 off-2+ | 0.250 | 0.250 | 0.500 | 0.500 | 0.03 |
| Area | down 2x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 15, 15 | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 0.250 | 0.250 | 0.500 | 0.500 | 0.02 |
| Area | down 2.37x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 15, 15 | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 0.250 | 0.250 | 0.500 | 0.500 | 0.02 |
| Area | down 8x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 15, 15 | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 0.250 | 0.250 | 0.500 | 0.500 | 0.01 |
| Area | down 16x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 15, 15 | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 0.250 | 0.250 | 0.500 | 0.500 | 0.01 |
| Area | down 32x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 15, 15 | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 0.250 | 0.250 | 0.500 | 0.500 | 0.01 |
| Area | down 64x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 15, 15 | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 0.250 | 0.250 | 0.500 | 0.500 | 0.01 |
| Area | down 128x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 15, 15 | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 0.250 | 0.250 | 0.500 | 0.500 | 0.01 |
| Area | down to 2x2 | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 15, 15 | max 1 / 75.0000% exact / 3 off-1 / 0 off-2+ | 0.250 | 0.250 | 0.500 | 0.500 | 0.01 |
| Area | aniso 1x/16x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 14, 15 | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 0.250 | 0.250 | 0.500 | 0.500 | 0.03 |

### p3-primaries (out-of-gamut edges) (U8 RGB Display P3)

| Filter | Geometry | f32 RGB | fixed p (h, v) | fixed RGB | acc bound / 2^31 | peak acc / 2^31 | row bound / 32767 | peak row / 32767 | s |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Lanczos3 | identity | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 14, 14 | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 0.153 | 0.153 | 0.613 | 0.612 | 0.06 |
| Lanczos3 | up 1.5x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 14, 14 | max 1 / 99.9487% exact / 2724 off-1 / 0 off-2+ | 0.278 | 0.196 | 0.810 | 0.693 | 0.14 |
| Lanczos3 | down 1.5x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 15, 15 | max 1 / 99.8848% exact / 1207 off-1 / 0 off-2+ | 0.475 | 0.371 | 0.757 | 0.678 | 0.08 |
| Lanczos3 | down 2x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 15, 15 | max 1 / 99.9769% exact / 136 off-1 / 0 off-2+ | 0.482 | 0.370 | 0.759 | 0.685 | 0.05 |
| Lanczos3 | down 2.37x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 15, 15 | max 1 / 99.9100% exact / 378 off-1 / 0 off-2+ | 0.469 | 0.356 | 0.750 | 0.670 | 0.04 |
| Lanczos3 | down 8x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 15, 15 | max 1 / 99.6474% exact / 130 off-1 / 0 off-2+ | 0.462 | 0.377 | 0.744 | 0.703 | 0.04 |
| Lanczos3 | down 16x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 15, 15 | max 1 / 98.2422% exact / 162 off-1 / 0 off-2+ | 0.461 | 0.356 | 0.743 | 0.706 | 0.06 |
| Lanczos3 | down 32x | max 1 / 99.9566% exact / 1 off-1 / 0 off-2+ | 15, 15 | max 1 / 98.9583% exact / 24 off-1 / 0 off-2+ | 0.460 | 0.356 | 0.743 | 0.703 | 0.05 |
| Lanczos3 | down 64x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 15, 15 | max 1 / 99.4792% exact / 3 off-1 / 0 off-2+ | 0.461 | 0.340 | 0.743 | 0.672 | 0.04 |
| Lanczos3 | down 128x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 15, 15 | max 1 / 96.5278% exact / 5 off-1 / 0 off-2+ | 0.460 | 0.334 | 0.743 | 0.669 | 0.03 |
| Lanczos3 | down to 2x2 | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 15, 15 | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 0.351 | 0.271 | 0.655 | 0.541 | 0.02 |
| Lanczos3 | aniso 1x/16x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 14, 15 | max 1 / 99.4609% exact / 795 off-1 / 0 off-2+ | 0.372 | 0.337 | 0.613 | 0.612 | 0.05 |
| Area | identity | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 14, 14 | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 0.153 | 0.153 | 0.613 | 0.612 | 0.05 |
| Area | up 1.5x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 14, 14 | max 1 / 100.0000% exact / 1 off-1 / 0 off-2+ | 0.153 | 0.153 | 0.613 | 0.612 | 0.11 |
| Area | down 1.5x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 15, 15 | max 1 / 99.9998% exact / 2 off-1 / 0 off-2+ | 0.306 | 0.306 | 0.613 | 0.612 | 0.04 |
| Area | down 2x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 15, 15 | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 0.306 | 0.306 | 0.613 | 0.612 | 0.03 |
| Area | down 2.37x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 15, 15 | max 1 / 99.9983% exact / 7 off-1 / 0 off-2+ | 0.306 | 0.306 | 0.613 | 0.612 | 0.03 |
| Area | down 8x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 15, 15 | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 0.306 | 0.306 | 0.613 | 0.612 | 0.02 |
| Area | down 16x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 15, 15 | max 1 / 99.9566% exact / 4 off-1 / 0 off-2+ | 0.306 | 0.306 | 0.613 | 0.612 | 0.02 |
| Area | down 32x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 15, 15 | max 1 / 99.9566% exact / 1 off-1 / 0 off-2+ | 0.306 | 0.306 | 0.613 | 0.612 | 0.02 |
| Area | down 64x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 15, 15 | max 1 / 99.8264% exact / 1 off-1 / 0 off-2+ | 0.306 | 0.306 | 0.613 | 0.612 | 0.02 |
| Area | down 128x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 15, 15 | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 0.306 | 0.306 | 0.613 | 0.612 | 0.02 |
| Area | down to 2x2 | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 15, 15 | max 1 / 91.6667% exact / 1 off-1 / 0 off-2+ | 0.306 | 0.264 | 0.613 | 0.529 | 0.02 |
| Area | aniso 1x/16x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 14, 15 | max 1 / 99.9471% exact / 78 off-1 / 0 off-2+ | 0.306 | 0.306 | 0.613 | 0.612 | 0.04 |

### p3-checker-1px (red/green) (U8 RGB Display P3)

| Filter | Geometry | f32 RGB | fixed p (h, v) | fixed RGB | acc bound / 2^31 | peak acc / 2^31 | row bound / 32767 | peak row / 32767 | s |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Lanczos3 | identity | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 14, 14 | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 0.153 | 0.153 | 0.613 | 0.612 | 0.06 |
| Lanczos3 | up 1.5x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 14, 14 | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 0.278 | 0.202 | 0.810 | 0.700 | 0.16 |
| Lanczos3 | down 1.5x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 15, 15 | max 1 / 99.5151% exact / 5080 off-1 / 0 off-2+ | 0.475 | 0.250 | 0.757 | 0.500 | 0.07 |
| Lanczos3 | down 2x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 15, 15 | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 0.482 | 0.250 | 0.759 | 0.500 | 0.06 |
| Lanczos3 | down 2.37x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 15, 15 | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 0.469 | 0.250 | 0.750 | 0.500 | 0.05 |
| Lanczos3 | down 8x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 15, 15 | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 0.462 | 0.250 | 0.744 | 0.500 | 0.03 |
| Lanczos3 | down 16x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 15, 15 | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 0.461 | 0.250 | 0.743 | 0.500 | 0.06 |
| Lanczos3 | down 32x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 15, 15 | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 0.460 | 0.250 | 0.743 | 0.500 | 0.03 |
| Lanczos3 | down 64x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 15, 15 | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 0.461 | 0.250 | 0.743 | 0.500 | 0.03 |
| Lanczos3 | down 128x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 15, 15 | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 0.460 | 0.250 | 0.743 | 0.500 | 0.03 |
| Lanczos3 | down to 2x2 | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 15, 15 | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 0.351 | 0.250 | 0.655 | 0.500 | 0.02 |
| Lanczos3 | aniso 1x/16x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 14, 15 | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 0.372 | 0.250 | 0.613 | 0.612 | 0.07 |
| Area | identity | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 14, 14 | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 0.153 | 0.153 | 0.613 | 0.612 | 0.05 |
| Area | up 1.5x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 14, 14 | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 0.153 | 0.153 | 0.613 | 0.612 | 0.12 |
| Area | down 1.5x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 15, 15 | max 1 / 99.9022% exact / 1024 off-1 / 0 off-2+ | 0.306 | 0.250 | 0.613 | 0.500 | 0.04 |
| Area | down 2x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 15, 15 | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 0.306 | 0.250 | 0.613 | 0.500 | 0.03 |
| Area | down 2.37x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 15, 15 | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 0.306 | 0.250 | 0.613 | 0.500 | 0.03 |
| Area | down 8x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 15, 15 | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 0.306 | 0.250 | 0.613 | 0.500 | 0.02 |
| Area | down 16x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 15, 15 | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 0.306 | 0.250 | 0.613 | 0.500 | 0.02 |
| Area | down 32x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 15, 15 | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 0.306 | 0.250 | 0.613 | 0.500 | 0.02 |
| Area | down 64x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 15, 15 | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 0.306 | 0.250 | 0.613 | 0.500 | 0.02 |
| Area | down 128x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 15, 15 | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 0.306 | 0.250 | 0.613 | 0.500 | 0.02 |
| Area | down to 2x2 | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 15, 15 | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 0.306 | 0.250 | 0.613 | 0.500 | 0.02 |
| Area | aniso 1x/16x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 14, 15 | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 0.306 | 0.250 | 0.613 | 0.612 | 0.03 |

### p3-noise (U8 RGB Display P3)

| Filter | Geometry | f32 RGB | fixed p (h, v) | fixed RGB | acc bound / 2^31 | peak acc / 2^31 | row bound / 32767 | peak row / 32767 | s |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Lanczos3 | identity | max 1 / 99.9999% exact / 2 off-1 / 0 off-2+ | 14, 14 | max 1 / 99.4432% exact / 13137 off-1 / 0 off-2+ | 0.153 | 0.153 | 0.613 | 0.612 | 0.15 |
| Lanczos3 | up 1.5x | max 1 / 99.9995% exact / 27 off-1 / 0 off-2+ | 14, 14 | max 1 / 99.2825% exact / 38090 off-1 / 0 off-2+ | 0.278 | 0.198 | 0.810 | 0.773 | 0.23 |
| Lanczos3 | down 1.5x | max 1 / 99.9991% exact / 9 off-1 / 0 off-2+ | 15, 15 | max 1 / 99.4096% exact / 6185 off-1 / 0 off-2+ | 0.475 | 0.341 | 0.757 | 0.682 | 0.12 |
| Lanczos3 | down 2x | max 1 / 99.9997% exact / 2 off-1 / 0 off-2+ | 15, 15 | max 1 / 99.3298% exact / 3953 off-1 / 0 off-2+ | 0.482 | 0.330 | 0.759 | 0.660 | 0.08 |
| Lanczos3 | down 2.37x | max 1 / 99.9993% exact / 3 off-1 / 0 off-2+ | 15, 15 | max 1 / 99.4870% exact / 2154 off-1 / 0 off-2+ | 0.469 | 0.317 | 0.750 | 0.635 | 0.08 |
| Lanczos3 | down 8x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 15, 15 | max 1 / 99.4792% exact / 192 off-1 / 0 off-2+ | 0.462 | 0.250 | 0.744 | 0.500 | 0.06 |
| Lanczos3 | down 16x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 15, 15 | max 1 / 99.6745% exact / 30 off-1 / 0 off-2+ | 0.461 | 0.250 | 0.743 | 0.500 | 0.05 |
| Lanczos3 | down 32x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 15, 15 | max 1 / 99.6962% exact / 7 off-1 / 0 off-2+ | 0.460 | 0.250 | 0.743 | 0.500 | 0.06 |
| Lanczos3 | down 64x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 15, 15 | max 1 / 99.6528% exact / 2 off-1 / 0 off-2+ | 0.461 | 0.250 | 0.743 | 0.500 | 0.06 |
| Lanczos3 | down 128x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 15, 15 | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 0.460 | 0.250 | 0.743 | 0.500 | 0.06 |
| Lanczos3 | down to 2x2 | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 15, 15 | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 0.351 | 0.250 | 0.655 | 0.500 | 0.04 |
| Lanczos3 | aniso 1x/16x | max 1 / 99.9993% exact / 1 off-1 / 0 off-2+ | 14, 15 | max 1 / 98.9814% exact / 1502 off-1 / 0 off-2+ | 0.372 | 0.250 | 0.613 | 0.612 | 0.09 |
| Area | identity | max 1 / 99.9999% exact / 2 off-1 / 0 off-2+ | 14, 14 | max 1 / 99.4432% exact / 13137 off-1 / 0 off-2+ | 0.153 | 0.153 | 0.613 | 0.612 | 0.10 |
| Area | up 1.5x | max 1 / 99.9996% exact / 23 off-1 / 0 off-2+ | 14, 14 | max 1 / 99.2515% exact / 39736 off-1 / 0 off-2+ | 0.153 | 0.153 | 0.613 | 0.612 | 0.16 |
| Area | down 1.5x | max 1 / 99.9993% exact / 7 off-1 / 0 off-2+ | 15, 15 | max 1 / 99.4566% exact / 5692 off-1 / 0 off-2+ | 0.306 | 0.304 | 0.613 | 0.607 | 0.07 |
| Area | down 2x | max 1 / 99.9997% exact / 2 off-1 / 0 off-2+ | 15, 15 | max 1 / 99.1670% exact / 4913 off-1 / 0 off-2+ | 0.306 | 0.305 | 0.613 | 0.609 | 0.05 |
| Area | down 2.37x | max 1 / 99.9995% exact / 2 off-1 / 0 off-2+ | 15, 15 | max 1 / 99.5525% exact / 1879 off-1 / 0 off-2+ | 0.306 | 0.296 | 0.613 | 0.591 | 0.05 |
| Area | down 8x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 15, 15 | max 1 / 99.6799% exact / 118 off-1 / 0 off-2+ | 0.306 | 0.250 | 0.613 | 0.500 | 0.04 |
| Area | down 16x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 15, 15 | max 1 / 99.6311% exact / 34 off-1 / 0 off-2+ | 0.306 | 0.250 | 0.613 | 0.500 | 0.04 |
| Area | down 32x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 15, 15 | max 1 / 99.7830% exact / 5 off-1 / 0 off-2+ | 0.306 | 0.250 | 0.613 | 0.500 | 0.04 |
| Area | down 64x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 15, 15 | max 1 / 99.6528% exact / 2 off-1 / 0 off-2+ | 0.306 | 0.250 | 0.613 | 0.500 | 0.04 |
| Area | down 128x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 15, 15 | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 0.306 | 0.250 | 0.613 | 0.500 | 0.04 |
| Area | down to 2x2 | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 15, 15 | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 0.306 | 0.250 | 0.613 | 0.500 | 0.04 |
| Area | aniso 1x/16x | max 1 / 99.9980% exact / 3 off-1 / 0 off-2+ | 14, 15 | max 1 / 99.6535% exact / 511 off-1 / 0 off-2+ | 0.306 | 0.250 | 0.613 | 0.612 | 0.05 |

### alpha-edges (opaque shapes on transparent) (U8 RGBA straight sRGB)

| Filter | Geometry | f32 RGB | fixed p (h, v) | fixed RGB | acc bound / 2^31 | peak acc / 2^31 | row bound / 32767 | peak row / 32767 | s |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Lanczos3 | identity | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 14, 14 | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 0.125 | 0.125 | 0.500 | 0.500 | 0.08 |
| Lanczos3 | up 1.5x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 14, 14 | max 24 / 99.4870% exact / 13846 off-1 / 7883 off-2+ | 0.211 | 0.173 | 0.636 | 0.560 | 0.16 |
| Lanczos3 | down 1.5x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 15, 15 | max 31 / 99.2135% exact / 3249 off-1 / 3371 off-2+ | 0.366 | 0.296 | 0.600 | 0.545 | 0.06 |
| Lanczos3 | down 2x | max 1 / 99.9929% exact / 34 off-1 / 0 off-2+ | 15, 15 | max 24 / 99.1389% exact / 1929 off-1 / 2193 off-2+ | 0.371 | 0.315 | 0.601 | 0.549 | 0.07 |
| Lanczos3 | down 2.37x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 15, 15 | max 30 / 98.8990% exact / 1962 off-1 / 1797 off-2+ | 0.362 | 0.311 | 0.595 | 0.540 | 0.06 |
| Lanczos3 | down 8x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 15, 15 | max 31 / 97.5488% exact / 478 off-1 / 292 off-2+ | 0.357 | 0.298 | 0.591 | 0.538 | 0.06 |
| Lanczos3 | down 16x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 15, 15 | max 20 / 95.8379% exact / 243 off-1 / 100 off-2+ | 0.356 | 0.302 | 0.590 | 0.569 | 0.03 |
| Lanczos3 | down 32x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 15, 15 | max 5 / 96.4183% exact / 58 off-1 / 17 off-2+ | 0.356 | 0.295 | 0.590 | 0.567 | 0.04 |
| Lanczos3 | down 64x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 15, 15 | max 1 / 96.5009% exact / 19 off-1 / 0 off-2+ | 0.356 | 0.287 | 0.590 | 0.574 | 0.05 |
| Lanczos3 | down 128x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 15, 15 | max 2 / 95.8333% exact / 5 off-1 / 1 off-2+ | 0.356 | 0.287 | 0.590 | 0.575 | 0.03 |
| Lanczos3 | down to 2x2 | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 15, 15 | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 0.281 | 0.258 | 0.529 | 0.516 | 0.02 |
| Lanczos3 | aniso 1x/16x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 14, 15 | max 9 / 96.9170% exact / 2886 off-1 / 1009 off-2+ | 0.295 | 0.283 | 0.500 | 0.500 | 0.07 |
| Area | identity | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 14, 14 | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 0.125 | 0.125 | 0.500 | 0.500 | 0.06 |
| Area | up 1.5x | max 1 / 99.9948% exact / 219 off-1 / 0 off-2+ | 14, 14 | max 3 / 99.9608% exact / 1642 off-1 / 1 off-2+ | 0.125 | 0.125 | 0.500 | 0.500 | 0.13 |
| Area | down 1.5x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 15, 15 | max 20 / 99.9300% exact / 478 off-1 / 102 off-2+ | 0.250 | 0.250 | 0.500 | 0.500 | 0.04 |
| Area | down 2x | max 1 / 99.9835% exact / 77 off-1 / 0 off-2+ | 15, 15 | max 1 / 99.9680% exact / 149 off-1 / 0 off-2+ | 0.250 | 0.250 | 0.500 | 0.500 | 0.03 |
| Area | down 2.37x | max 1 / 99.9931% exact / 23 off-1 / 0 off-2+ | 15, 15 | max 6 / 99.9297% exact / 216 off-1 / 19 off-2+ | 0.250 | 0.250 | 0.500 | 0.500 | 0.03 |
| Area | down 8x | max 1 / 99.9967% exact / 1 off-1 / 0 off-2+ | 15, 15 | max 2 / 99.6843% exact / 94 off-1 / 1 off-2+ | 0.250 | 0.250 | 0.500 | 0.500 | 0.02 |
| Area | down 16x | max 1 / 99.8456% exact / 12 off-1 / 0 off-2+ | 15, 15 | max 1 / 99.4466% exact / 43 off-1 / 0 off-2+ | 0.250 | 0.250 | 0.500 | 0.500 | 0.02 |
| Area | down 32x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 15, 15 | max 1 / 98.9136% exact / 22 off-1 / 0 off-2+ | 0.250 | 0.250 | 0.500 | 0.500 | 0.02 |
| Area | down 64x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 15, 15 | max 1 / 98.5267% exact / 8 off-1 / 0 off-2+ | 0.250 | 0.250 | 0.500 | 0.500 | 0.02 |
| Area | down 128x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 15, 15 | max 1 / 99.3056% exact / 1 off-1 / 0 off-2+ | 0.250 | 0.250 | 0.500 | 0.500 | 0.02 |
| Area | down to 2x2 | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 15, 15 | max 2 / 75.0000% exact / 2 off-1 / 1 off-2+ | 0.250 | 0.248 | 0.500 | 0.496 | 0.02 |
| Area | aniso 1x/16x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 14, 15 | max 1 / 99.7596% exact / 290 off-1 / 0 off-2+ | 0.250 | 0.250 | 0.500 | 0.500 | 0.03 |

Alpha channel and RGB by reference output alpha (buckets 1-25, 26-127, 128-254, 255):

| Filter | Geometry | Path | alpha | RGB a=1-25 | RGB a=26-127 | RGB a=128-254 | RGB a=255 | invisible RGB diffs |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Lanczos3 | identity | f32 | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | - | - | - | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 0 |
| Lanczos3 | identity | fixed | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | - | - | - | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 0 |
| Lanczos3 | up 1.5x | f32 | max 1 / 99.8787% exact / 2146 off-1 / 0 off-2+ | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 0 |
| Lanczos3 | up 1.5x | fixed | max 1 / 99.8775% exact / 2168 off-1 / 0 off-2+ | max 24 / 64.7960% exact / 7436 off-1 / 7883 off-2+ | max 1 / 97.4866% exact / 583 off-1 / 0 off-2+ | max 1 / 99.9416% exact / 39 off-1 / 0 off-2+ | max 1 / 99.8589% exact / 5788 off-1 / 0 off-2+ | 230 |
| Lanczos3 | down 1.5x | f32 | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 0 |
| Lanczos3 | down 1.5x | fixed | max 1 / 99.9971% exact / 10 off-1 / 0 off-2+ | max 31 / 65.8863% exact / 1569 off-1 / 3371 off-2+ | max 1 / 98.5753% exact / 58 off-1 / 0 off-2+ | max 1 / 99.8609% exact / 28 off-1 / 0 off-2+ | max 1 / 99.8015% exact / 1594 off-1 / 0 off-2+ | 722 |
| Lanczos3 | down 2x | f32 | max 1 / 99.4166% exact / 1147 off-1 / 0 off-2+ | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | max 1 / 99.9924% exact / 34 off-1 / 0 off-2+ | 0 |
| Lanczos3 | down 2x | fixed | max 1 / 99.4146% exact / 1151 off-1 / 0 off-2+ | max 24 / 74.8523% exact / 1129 off-1 / 2192 off-2+ | max 2 / 99.7155% exact / 10 off-1 / 1 off-2+ | max 1 / 99.8059% exact / 29 off-1 / 0 off-2+ | max 1 / 99.8296% exact / 761 off-1 / 0 off-2+ | 198 |
| Lanczos3 | down 2.37x | f32 | max 1 / 99.9807% exact / 27 off-1 / 0 off-2+ | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 0 |
| Lanczos3 | down 2.37x | fixed | max 1 / 99.9336% exact / 93 off-1 / 0 off-2+ | max 30 / 66.8788% exact / 858 off-1 / 1796 off-2+ | max 2 / 97.3099% exact / 91 off-1 / 1 off-2+ | max 1 / 99.8426% exact / 19 off-1 / 0 off-2+ | max 1 / 99.6873% exact / 994 off-1 / 0 off-2+ | 386 |
| Lanczos3 | down 8x | f32 | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 0 |
| Lanczos3 | down 8x | fixed | max 1 / 99.1536% exact / 104 off-1 / 0 off-2+ | max 31 / 66.0088% exact / 174 off-1 / 291 off-2+ | max 2 / 95.5381% exact / 50 off-1 / 1 off-2+ | max 1 / 98.7404% exact / 41 off-1 / 0 off-2+ | max 1 / 99.1695% exact / 213 off-1 / 0 off-2+ | 233 |
| Lanczos3 | down 16x | f32 | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 0 |
| Lanczos3 | down 16x | fixed | max 1 / 99.6094% exact / 12 off-1 / 0 off-2+ | max 20 / 65.3021% exact / 79 off-1 / 99 off-2+ | max 2 / 93.1624% exact / 31 off-1 / 1 off-2+ | max 1 / 98.1851% exact / 30 off-1 / 0 off-2+ | max 1 / 98.1630% exact / 103 off-1 / 0 off-2+ | 43 |
| Lanczos3 | down 32x | f32 | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 0 |
| Lanczos3 | down 32x | fixed | max 1 / 99.2188% exact / 6 off-1 / 0 off-2+ | max 5 / 68.5185% exact / 17 off-1 / 17 off-2+ | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | max 1 / 97.0711% exact / 21 off-1 / 0 off-2+ | max 1 / 98.2502% exact / 20 off-1 / 0 off-2+ | 12 |
| Lanczos3 | down 64x | f32 | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 0 |
| Lanczos3 | down 64x | fixed | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | max 1 / 93.0556% exact / 5 off-1 / 0 off-2+ | max 1 / 96.6245% exact / 8 off-1 / 0 off-2+ | max 1 / 97.3333% exact / 6 off-1 / 0 off-2+ | 0 |
| Lanczos3 | down 128x | f32 | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | - | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 0 |
| Lanczos3 | down 128x | fixed | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | - | max 2 / 87.5000% exact / 2 off-1 / 1 off-2+ | max 1 / 97.5309% exact / 2 off-1 / 0 off-2+ | max 1 / 97.4359% exact / 1 off-1 / 0 off-2+ | 0 |
| Lanczos3 | down to 2x2 | f32 | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | - | - | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | - | 0 |
| Lanczos3 | down to 2x2 | fixed | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | - | - | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | - | 0 |
| Lanczos3 | aniso 1x/16x | f32 | max 1 / 99.9329% exact / 33 off-1 / 0 off-2+ | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 0 |
| Lanczos3 | aniso 1x/16x | fixed | max 1 / 99.9308% exact / 34 off-1 / 0 off-2+ | max 9 / 62.4325% exact / 1147 off-1 / 1009 off-2+ | max 1 / 93.2340% exact / 327 off-1 / 0 off-2+ | max 1 / 99.5083% exact / 72 off-1 / 0 off-2+ | max 1 / 98.6749% exact / 1340 off-1 / 0 off-2+ | 71 |
| Area | identity | f32 | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | - | - | - | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 0 |
| Area | identity | fixed | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | - | - | - | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 0 |
| Area | up 1.5x | f32 | max 1 / 99.6837% exact / 5596 off-1 / 0 off-2+ | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | max 1 / 99.9947% exact / 219 off-1 / 0 off-2+ | 0 |
| Area | up 1.5x | fixed | max 1 / 99.6930% exact / 5433 off-1 / 0 off-2+ | max 3 / 78.7234% exact / 29 off-1 / 1 off-2+ | max 1 / 97.1098% exact / 480 off-1 / 0 off-2+ | max 1 / 99.9794% exact / 6 off-1 / 0 off-2+ | max 1 / 99.9728% exact / 1127 off-1 / 0 off-2+ | 0 |
| Area | down 1.5x | f32 | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 0 |
| Area | down 1.5x | fixed | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | max 20 / 67.9956% exact / 187 off-1 / 102 off-2+ | max 1 / 98.8676% exact / 46 off-1 / 0 off-2+ | max 1 / 99.9310% exact / 4 off-1 / 0 off-2+ | max 1 / 99.9705% exact / 241 off-1 / 0 off-2+ | 0 |
| Area | down 2x | f32 | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | - | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | max 1 / 99.9833% exact / 77 off-1 / 0 off-2+ | 0 |
| Area | down 2x | fixed | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | - | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | max 1 / 99.9761% exact / 1 off-1 / 0 off-2+ | max 1 / 99.9679% exact / 148 off-1 / 0 off-2+ | 0 |
| Area | down 2.37x | f32 | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | max 1 / 99.9929% exact / 23 off-1 / 0 off-2+ | 0 |
| Area | down 2.37x | fixed | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | max 6 / 96.1030% exact / 37 off-1 / 19 off-2+ | max 1 / 99.9225% exact / 2 off-1 / 0 off-2+ | max 1 / 99.8574% exact / 6 off-1 / 0 off-2+ | max 1 / 99.9475% exact / 171 off-1 / 0 off-2+ | 0 |
| Area | down 8x | f32 | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | max 1 / 99.9964% exact / 1 off-1 / 0 off-2+ | 0 |
| Area | down 8x | fixed | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | max 2 / 92.8571% exact / 2 off-1 / 1 off-2+ | max 1 / 99.0868% exact / 10 off-1 / 0 off-2+ | max 1 / 99.6774% exact / 3 off-1 / 0 off-2+ | max 1 / 99.7181% exact / 79 off-1 / 0 off-2+ | 0 |
| Area | down 16x | f32 | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | max 1 / 99.8214% exact / 12 off-1 / 0 off-2+ | 0 |
| Area | down 16x | fixed | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | max 1 / 78.1250% exact / 21 off-1 / 0 off-2+ | max 1 / 98.4733% exact / 6 off-1 / 0 off-2+ | max 1 / 98.9305% exact / 6 off-1 / 0 off-2+ | max 1 / 99.8512% exact / 10 off-1 / 0 off-2+ | 0 |
| Area | down 32x | f32 | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 0 |
| Area | down 32x | fixed | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | max 1 / 79.4872% exact / 8 off-1 / 0 off-2+ | max 1 / 96.7480% exact / 4 off-1 / 0 off-2+ | max 1 / 99.2188% exact / 3 off-1 / 0 off-2+ | max 1 / 99.5267% exact / 7 off-1 / 0 off-2+ | 0 |
| Area | down 64x | f32 | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 0 |
| Area | down 64x | fixed | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | max 1 / 83.3333% exact / 1 off-1 / 0 off-2+ | max 1 / 97.3333% exact / 2 off-1 / 0 off-2+ | max 1 / 98.8095% exact / 2 off-1 / 0 off-2+ | max 1 / 98.9796% exact / 3 off-1 / 0 off-2+ | 0 |
| Area | down 128x | f32 | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | - | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 0 |
| Area | down 128x | fixed | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | - | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | max 1 / 97.6190% exact / 1 off-1 / 0 off-2+ | 0 |
| Area | down to 2x2 | f32 | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | - | - | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | - | 0 |
| Area | down to 2x2 | fixed | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | - | - | max 2 / 75.0000% exact / 2 off-1 / 1 off-2+ | - | 0 |
| Area | aniso 1x/16x | f32 | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 0 |
| Area | aniso 1x/16x | fixed | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | max 1 / 76.3514% exact / 210 off-1 / 0 off-2+ | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | max 1 / 98.7828% exact / 52 off-1 / 0 off-2+ | max 1 / 99.9749% exact / 28 off-1 / 0 off-2+ | 0 |

### alpha-noise (U8 RGBA straight sRGB)

| Filter | Geometry | f32 RGB | fixed p (h, v) | fixed RGB | acc bound / 2^31 | peak acc / 2^31 | row bound / 32767 | peak row / 32767 | s |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Lanczos3 | identity | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 14, 14 | max 21 / 98.2907% exact / 29342 off-1 / 10830 off-2+ | 0.125 | 0.125 | 0.500 | 0.500 | 0.10 |
| Lanczos3 | up 1.5x | max 1 / 99.9993% exact / 36 off-1 / 0 off-2+ | 14, 14 | max 23 / 98.1475% exact / 95999 off-1 / 639 off-2+ | 0.211 | 0.171 | 0.636 | 0.616 | 0.26 |
| Lanczos3 | down 1.5x | max 1 / 99.9991% exact / 9 off-1 / 0 off-2+ | 15, 15 | max 5 / 98.5554% exact / 15117 off-1 / 8 off-2+ | 0.366 | 0.287 | 0.600 | 0.573 | 0.11 |
| Lanczos3 | down 2x | max 1 / 99.9993% exact / 4 off-1 / 0 off-2+ | 15, 15 | max 1 / 98.7537% exact / 7351 off-1 / 0 off-2+ | 0.371 | 0.275 | 0.601 | 0.550 | 0.08 |
| Lanczos3 | down 2.37x | max 1 / 99.9993% exact / 3 off-1 / 0 off-2+ | 15, 15 | max 1 / 98.9233% exact / 4521 off-1 / 0 off-2+ | 0.362 | 0.275 | 0.595 | 0.549 | 0.06 |
| Lanczos3 | down 8x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 15, 15 | max 1 / 99.1455% exact / 315 off-1 / 0 off-2+ | 0.357 | 0.222 | 0.591 | 0.444 | 0.05 |
| Lanczos3 | down 16x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 15, 15 | max 1 / 99.0560% exact / 87 off-1 / 0 off-2+ | 0.356 | 0.195 | 0.590 | 0.390 | 0.05 |
| Lanczos3 | down 32x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 15, 15 | max 1 / 99.3056% exact / 16 off-1 / 0 off-2+ | 0.356 | 0.175 | 0.590 | 0.350 | 0.05 |
| Lanczos3 | down 64x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 15, 15 | max 1 / 98.7847% exact / 7 off-1 / 0 off-2+ | 0.356 | 0.161 | 0.590 | 0.323 | 0.05 |
| Lanczos3 | down 128x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 15, 15 | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 0.356 | 0.150 | 0.590 | 0.299 | 0.05 |
| Lanczos3 | down to 2x2 | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 15, 15 | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 0.281 | 0.136 | 0.529 | 0.272 | 0.04 |
| Lanczos3 | aniso 1x/16x | max 1 / 99.9980% exact / 3 off-1 / 0 off-2+ | 14, 15 | max 1 / 98.7549% exact / 1836 off-1 / 0 off-2+ | 0.295 | 0.196 | 0.500 | 0.500 | 0.07 |
| Area | identity | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 14, 14 | max 21 / 98.2907% exact / 29342 off-1 / 10830 off-2+ | 0.125 | 0.125 | 0.500 | 0.500 | 0.09 |
| Area | up 1.5x | max 1 / 99.9996% exact / 23 off-1 / 0 off-2+ | 14, 14 | max 9 / 98.2200% exact / 94463 off-1 / 28 off-2+ | 0.125 | 0.125 | 0.500 | 0.500 | 0.18 |
| Area | down 1.5x | max 1 / 99.9993% exact / 7 off-1 / 0 off-2+ | 15, 15 | max 1 / 98.7007% exact / 13611 off-1 / 0 off-2+ | 0.250 | 0.250 | 0.500 | 0.500 | 0.07 |
| Area | down 2x | max 1 / 99.9998% exact / 1 off-1 / 0 off-2+ | 15, 15 | max 2 / 98.3205% exact / 9904 off-1 / 2 off-2+ | 0.250 | 0.250 | 0.500 | 0.500 | 0.05 |
| Area | down 2.37x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 15, 15 | max 1 / 99.0110% exact / 4153 off-1 / 0 off-2+ | 0.250 | 0.249 | 0.500 | 0.498 | 0.05 |
| Area | down 8x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 15, 15 | max 1 / 99.1347% exact / 319 off-1 / 0 off-2+ | 0.250 | 0.228 | 0.500 | 0.456 | 0.04 |
| Area | down 16x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 15, 15 | max 1 / 99.2296% exact / 71 off-1 / 0 off-2+ | 0.250 | 0.196 | 0.500 | 0.391 | 0.04 |
| Area | down 32x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 15, 15 | max 1 / 99.0885% exact / 21 off-1 / 0 off-2+ | 0.250 | 0.181 | 0.500 | 0.361 | 0.04 |
| Area | down 64x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 15, 15 | max 1 / 99.3056% exact / 4 off-1 / 0 off-2+ | 0.250 | 0.160 | 0.500 | 0.320 | 0.04 |
| Area | down 128x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 15, 15 | max 1 / 98.6111% exact / 2 off-1 / 0 off-2+ | 0.250 | 0.152 | 0.500 | 0.305 | 0.03 |
| Area | down to 2x2 | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 15, 15 | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 0.250 | 0.135 | 0.500 | 0.271 | 0.04 |
| Area | aniso 1x/16x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 14, 15 | max 1 / 99.2072% exact / 1169 off-1 / 0 off-2+ | 0.250 | 0.205 | 0.500 | 0.500 | 0.05 |

Alpha channel and RGB by reference output alpha (buckets 1-25, 26-127, 128-254, 255):

| Filter | Geometry | Path | alpha | RGB a=1-25 | RGB a=26-127 | RGB a=128-254 | RGB a=255 | invisible RGB diffs |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Lanczos3 | identity | f32 | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 0 |
| Lanczos3 | identity | fixed | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | max 21 / 84.0733% exact / 25859 off-1 / 10830 off-2+ | max 1 / 99.6289% exact / 3483 off-1 / 0 off-2+ | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 0 |
| Lanczos3 | up 1.5x | f32 | max 1 / 99.9986% exact / 25 off-1 / 0 off-2+ | max 1 / 99.9983% exact / 4 off-1 / 0 off-2+ | max 1 / 99.9993% exact / 16 off-1 / 0 off-2+ | max 1 / 99.9994% exact / 16 off-1 / 0 off-2+ | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 0 |
| Lanczos3 | up 1.5x | fixed | max 1 / 99.3423% exact / 11638 off-1 / 0 off-2+ | max 23 / 95.5541% exact / 10161 off-1 / 552 off-2+ | max 3 / 97.7387% exact / 52399 off-1 / 87 off-2+ | max 1 / 98.7271% exact / 32630 off-1 / 0 off-2+ | max 1 / 99.1143% exact / 809 off-1 / 0 off-2+ | 37 |
| Lanczos3 | down 1.5x | f32 | max 1 / 99.9983% exact / 6 off-1 / 0 off-2+ | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | max 1 / 99.9992% exact / 4 off-1 / 0 off-2+ | max 1 / 99.9990% exact / 5 off-1 / 0 off-2+ | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 0 |
| Lanczos3 | down 1.5x | fixed | max 1 / 99.4562% exact / 1899 off-1 / 0 off-2+ | max 5 / 95.2609% exact / 318 off-1 / 8 off-2+ | max 1 / 98.2131% exact / 9227 off-1 / 0 off-2+ | max 1 / 98.9357% exact / 5569 off-1 / 0 off-2+ | max 1 / 99.4220% exact / 3 off-1 / 0 off-2+ | 0 |
| Lanczos3 | down 2x | f32 | max 1 / 99.9990% exact / 2 off-1 / 0 off-2+ | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | max 1 / 99.9990% exact / 3 off-1 / 0 off-2+ | max 1 / 99.9997% exact / 1 off-1 / 0 off-2+ | - | 0 |
| Lanczos3 | down 2x | fixed | max 1 / 99.4456% exact / 1090 off-1 / 0 off-2+ | max 1 / 93.9759% exact / 15 off-1 / 0 off-2+ | max 1 / 98.4798% exact / 4481 off-1 / 0 off-2+ | max 1 / 99.0316% exact / 2855 off-1 / 0 off-2+ | - | 0 |
| Lanczos3 | down 2.37x | f32 | max 1 / 99.9986% exact / 2 off-1 / 0 off-2+ | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | max 1 / 99.9995% exact / 1 off-1 / 0 off-2+ | max 1 / 99.9990% exact / 2 off-1 / 0 off-2+ | - | 0 |
| Lanczos3 | down 2.37x | fixed | max 1 / 99.4742% exact / 736 off-1 / 0 off-2+ | max 1 / 95.2381% exact / 1 off-1 / 0 off-2+ | max 1 / 98.7241% exact / 2673 off-1 / 0 off-2+ | max 1 / 99.1221% exact / 1847 off-1 / 0 off-2+ | - | 0 |
| Lanczos3 | down 8x | f32 | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | - | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | - | 0 |
| Lanczos3 | down 8x | fixed | max 1 / 99.4141% exact / 72 off-1 / 0 off-2+ | - | max 1 / 99.1029% exact / 166 off-1 / 0 off-2+ | max 1 / 99.1885% exact / 149 off-1 / 0 off-2+ | - | 0 |
| Lanczos3 | down 16x | f32 | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | - | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | - | 0 |
| Lanczos3 | down 16x | fixed | max 1 / 99.3815% exact / 19 off-1 / 0 off-2+ | - | max 1 / 98.9814% exact / 47 off-1 / 0 off-2+ | max 1 / 99.1308% exact / 40 off-1 / 0 off-2+ | - | 0 |
| Lanczos3 | down 32x | f32 | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | - | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | - | 0 |
| Lanczos3 | down 32x | fixed | max 1 / 99.6094% exact / 3 off-1 / 0 off-2+ | - | max 1 / 99.1015% exact / 10 off-1 / 0 off-2+ | max 1 / 99.4962% exact / 6 off-1 / 0 off-2+ | - | 0 |
| Lanczos3 | down 64x | f32 | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | - | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | - | 0 |
| Lanczos3 | down 64x | fixed | max 1 / 98.4375% exact / 3 off-1 / 0 off-2+ | - | max 1 / 98.5663% exact / 4 off-1 / 0 off-2+ | max 1 / 98.9899% exact / 3 off-1 / 0 off-2+ | - | 0 |
| Lanczos3 | down 128x | f32 | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | - | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | - | 0 |
| Lanczos3 | down 128x | fixed | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | - | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | - | 0 |
| Lanczos3 | down to 2x2 | f32 | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | - | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | - | 0 |
| Lanczos3 | down to 2x2 | fixed | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | - | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | - | 0 |
| Lanczos3 | aniso 1x/16x | f32 | max 1 / 99.9959% exact / 2 off-1 / 0 off-2+ | - | max 1 / 99.9986% exact / 1 off-1 / 0 off-2+ | max 1 / 99.9973% exact / 2 off-1 / 0 off-2+ | - | 0 |
| Lanczos3 | aniso 1x/16x | fixed | max 1 / 98.8790% exact / 551 off-1 / 0 off-2+ | - | max 1 / 98.7249% exact / 940 off-1 / 0 off-2+ | max 1 / 98.7848% exact / 896 off-1 / 0 off-2+ | - | 0 |
| Area | identity | f32 | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 0 |
| Area | identity | fixed | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | max 21 / 84.0733% exact / 25859 off-1 / 10830 off-2+ | max 1 / 99.6289% exact / 3483 off-1 / 0 off-2+ | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 0 |
| Area | up 1.5x | f32 | max 1 / 97.1899% exact / 49724 off-1 / 0 off-2+ | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | max 1 / 99.9996% exact / 10 off-1 / 0 off-2+ | max 1 / 99.9995% exact / 13 off-1 / 0 off-2+ | - | 0 |
| Area | up 1.5x | fixed | max 1 / 97.0420% exact / 52341 off-1 / 0 off-2+ | max 9 / 90.9817% exact / 2684 off-1 / 25 off-2+ | max 2 / 97.7502% exact / 58996 off-1 / 3 off-2+ | max 1 / 98.7657% exact / 32783 off-1 / 0 off-2+ | - | 0 |
| Area | down 1.5x | f32 | max 1 / 99.9001% exact / 349 off-1 / 0 off-2+ | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | max 1 / 99.9990% exact / 5 off-1 / 0 off-2+ | max 1 / 99.9996% exact / 2 off-1 / 0 off-2+ | - | 0 |
| Area | down 1.5x | fixed | max 1 / 99.4693% exact / 1853 off-1 / 0 off-2+ | max 1 / 91.8285% exact / 101 off-1 / 0 off-2+ | max 1 / 98.3991% exact / 8357 off-1 / 0 off-2+ | max 1 / 99.0171% exact / 5153 off-1 / 0 off-2+ | - | 0 |
| Area | down 2x | f32 | max 1 / 96.8389% exact / 6215 off-1 / 0 off-2+ | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | max 1 / 99.9997% exact / 1 off-1 / 0 off-2+ | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | - | 0 |
| Area | down 2x | fixed | max 1 / 95.6528% exact / 8547 off-1 / 0 off-2+ | max 2 / 89.2199% exact / 74 off-1 / 2 off-2+ | max 1 / 97.9030% exact / 6158 off-1 / 0 off-2+ | max 1 / 98.7572% exact / 3672 off-1 / 0 off-2+ | - | 0 |
| Area | down 2.37x | f32 | max 1 / 99.9879% exact / 17 off-1 / 0 off-2+ | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | - | 0 |
| Area | down 2.37x | fixed | max 1 / 99.5535% exact / 625 off-1 / 0 off-2+ | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | max 1 / 98.8455% exact / 2419 off-1 / 0 off-2+ | max 1 / 99.1758% exact / 1734 off-1 / 0 off-2+ | - | 0 |
| Area | down 8x | f32 | max 1 / 99.6175% exact / 47 off-1 / 0 off-2+ | - | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | - | 0 |
| Area | down 8x | fixed | max 1 / 99.2269% exact / 95 off-1 / 0 off-2+ | - | max 1 / 99.1533% exact / 155 off-1 / 0 off-2+ | max 1 / 99.1163% exact / 164 off-1 / 0 off-2+ | - | 0 |
| Area | down 16x | f32 | max 1 / 99.7396% exact / 8 off-1 / 0 off-2+ | - | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | - | 0 |
| Area | down 16x | fixed | max 1 / 99.6745% exact / 10 off-1 / 0 off-2+ | - | max 1 / 99.1965% exact / 37 off-1 / 0 off-2+ | max 1 / 99.2626% exact / 34 off-1 / 0 off-2+ | - | 0 |
| Area | down 32x | f32 | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | - | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | - | 0 |
| Area | down 32x | fixed | max 1 / 99.4792% exact / 4 off-1 / 0 off-2+ | - | max 1 / 99.0550% exact / 11 off-1 / 0 off-2+ | max 1 / 99.1228% exact / 10 off-1 / 0 off-2+ | - | 0 |
| Area | down 64x | f32 | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | - | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | - | 0 |
| Area | down 64x | fixed | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | - | max 1 / 99.2908% exact / 2 off-1 / 0 off-2+ | max 1 / 99.3197% exact / 2 off-1 / 0 off-2+ | - | 0 |
| Area | down 128x | f32 | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | - | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | - | 0 |
| Area | down 128x | fixed | max 1 / 97.9167% exact / 1 off-1 / 0 off-2+ | - | max 1 / 98.4127% exact / 1 off-1 / 0 off-2+ | max 1 / 98.7654% exact / 1 off-1 / 0 off-2+ | - | 0 |
| Area | down to 2x2 | f32 | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | - | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | - | 0 |
| Area | down to 2x2 | fixed | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | - | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | - | 0 |
| Area | aniso 1x/16x | f32 | max 1 / 98.2117% exact / 879 off-1 / 0 off-2+ | - | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | - | 0 |
| Area | aniso 1x/16x | fixed | max 1 / 96.9523% exact / 1498 off-1 / 0 off-2+ | - | max 1 / 99.1351% exact / 636 off-1 / 0 off-2+ | max 1 / 99.2790% exact / 533 off-1 / 0 off-2+ | - | 0 |

### f32-noise (F32 RGB sRGB)

| Filter | Geometry | f32 RGB | s |
| --- | --- | --- | --- |
| Lanczos3 | identity | max 1 / 99.9990% exact / 23 off-1 / 0 off-2+ | 0.10 |
| Lanczos3 | up 1.5x | max 1 / 99.9993% exact / 36 off-1 / 0 off-2+ | 0.17 |
| Lanczos3 | down 1.5x | max 1 / 99.9990% exact / 10 off-1 / 0 off-2+ | 0.07 |
| Lanczos3 | down 2x | max 1 / 99.9988% exact / 7 off-1 / 0 off-2+ | 0.08 |
| Lanczos3 | down 2.37x | max 1 / 99.9995% exact / 2 off-1 / 0 off-2+ | 0.09 |
| Lanczos3 | down 8x | max 1 / 99.9973% exact / 1 off-1 / 0 off-2+ | 0.05 |
| Lanczos3 | down 16x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 0.05 |
| Lanczos3 | down 32x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 0.06 |
| Lanczos3 | down 64x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 0.05 |
| Lanczos3 | down 128x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 0.05 |
| Lanczos3 | down to 2x2 | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 0.06 |
| Lanczos3 | aniso 1x/16x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 0.07 |
| Area | identity | max 1 / 99.9990% exact / 23 off-1 / 0 off-2+ | 0.08 |
| Area | up 1.5x | max 1 / 99.9995% exact / 27 off-1 / 0 off-2+ | 0.14 |
| Area | down 1.5x | max 1 / 99.9989% exact / 12 off-1 / 0 off-2+ | 0.06 |
| Area | down 2x | max 1 / 99.9995% exact / 3 off-1 / 0 off-2+ | 0.05 |
| Area | down 2.37x | max 1 / 99.9995% exact / 2 off-1 / 0 off-2+ | 0.05 |
| Area | down 8x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 0.04 |
| Area | down 16x | max 1 / 99.9891% exact / 1 off-1 / 0 off-2+ | 0.04 |
| Area | down 32x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 0.04 |
| Area | down 64x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 0.04 |
| Area | down 128x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 0.04 |
| Area | down to 2x2 | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 0.04 |
| Area | aniso 1x/16x | max 1 / 99.9986% exact / 2 off-1 / 0 off-2+ | 0.05 |

### f32-dark (0-0.01) (F32 RGB sRGB)

| Filter | Geometry | f32 RGB | s |
| --- | --- | --- | --- |
| Lanczos3 | identity | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 0.04 |
| Lanczos3 | up 1.5x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 0.10 |
| Lanczos3 | down 1.5x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 0.03 |
| Lanczos3 | down 2x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 0.03 |
| Lanczos3 | down 2.37x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 0.02 |
| Lanczos3 | down 8x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 0.01 |
| Lanczos3 | down 16x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 0.01 |
| Lanczos3 | down 32x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 0.01 |
| Lanczos3 | down 64x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 0.01 |
| Lanczos3 | down 128x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 0.02 |
| Lanczos3 | down to 2x2 | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 0.01 |
| Lanczos3 | aniso 1x/16x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 0.03 |
| Area | identity | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 0.03 |
| Area | up 1.5x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 0.06 |
| Area | down 1.5x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 0.02 |
| Area | down 2x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 0.01 |
| Area | down 2.37x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 0.01 |
| Area | down 8x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 0.01 |
| Area | down 16x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 0.01 |
| Area | down 32x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 0.01 |
| Area | down 64x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 0.01 |
| Area | down 128x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 0.01 |
| Area | down to 2x2 | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 0.01 |
| Area | aniso 1x/16x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 0.01 |

### f32-extended (-0.2 to 1.5) (F32 RGB sRGB)

| Filter | Geometry | f32 RGB | s |
| --- | --- | --- | --- |
| Lanczos3 | identity | max 1 / 99.9996% exact / 9 off-1 / 0 off-2+ | 0.13 |
| Lanczos3 | up 1.5x | max 1 / 99.9993% exact / 37 off-1 / 0 off-2+ | 0.21 |
| Lanczos3 | down 1.5x | max 1 / 99.9990% exact / 10 off-1 / 0 off-2+ | 0.08 |
| Lanczos3 | down 2x | max 1 / 99.9993% exact / 4 off-1 / 0 off-2+ | 0.09 |
| Lanczos3 | down 2.37x | max 1 / 99.9990% exact / 4 off-1 / 0 off-2+ | 0.06 |
| Lanczos3 | down 8x | max 1 / 99.9973% exact / 1 off-1 / 0 off-2+ | 0.05 |
| Lanczos3 | down 16x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 0.05 |
| Lanczos3 | down 32x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 0.06 |
| Lanczos3 | down 64x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 0.05 |
| Lanczos3 | down 128x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 0.05 |
| Lanczos3 | down to 2x2 | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 0.05 |
| Lanczos3 | aniso 1x/16x | max 1 / 99.9993% exact / 1 off-1 / 0 off-2+ | 0.07 |
| Area | identity | max 1 / 99.9996% exact / 9 off-1 / 0 off-2+ | 0.10 |
| Area | up 1.5x | max 1 / 99.9994% exact / 30 off-1 / 0 off-2+ | 0.16 |
| Area | down 1.5x | max 1 / 99.9991% exact / 9 off-1 / 0 off-2+ | 0.07 |
| Area | down 2x | max 1 / 99.9997% exact / 2 off-1 / 0 off-2+ | 0.05 |
| Area | down 2.37x | max 1 / 99.9998% exact / 1 off-1 / 0 off-2+ | 0.05 |
| Area | down 8x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 0.04 |
| Area | down 16x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 0.04 |
| Area | down 32x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 0.04 |
| Area | down 64x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 0.04 |
| Area | down 128x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 0.04 |
| Area | down to 2x2 | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 0.04 |
| Area | aniso 1x/16x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 0.05 |

### f32-p3-noise (F32 RGB Display P3)

| Filter | Geometry | f32 RGB | s |
| --- | --- | --- | --- |
| Lanczos3 | identity | max 1 / 99.9994% exact / 14 off-1 / 0 off-2+ | 0.10 |
| Lanczos3 | up 1.5x | max 1 / 99.9990% exact / 54 off-1 / 0 off-2+ | 0.19 |
| Lanczos3 | down 1.5x | max 1 / 99.9989% exact / 11 off-1 / 0 off-2+ | 0.08 |
| Lanczos3 | down 2x | max 1 / 99.9983% exact / 10 off-1 / 0 off-2+ | 0.09 |
| Lanczos3 | down 2.37x | max 1 / 99.9988% exact / 5 off-1 / 0 off-2+ | 0.07 |
| Lanczos3 | down 8x | max 1 / 99.9973% exact / 1 off-1 / 0 off-2+ | 0.06 |
| Lanczos3 | down 16x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 0.06 |
| Lanczos3 | down 32x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 0.06 |
| Lanczos3 | down 64x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 0.06 |
| Lanczos3 | down 128x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 0.05 |
| Lanczos3 | down to 2x2 | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 0.05 |
| Lanczos3 | aniso 1x/16x | max 1 / 99.9986% exact / 2 off-1 / 0 off-2+ | 0.08 |
| Area | identity | max 1 / 99.9994% exact / 14 off-1 / 0 off-2+ | 0.09 |
| Area | up 1.5x | max 1 / 99.9994% exact / 34 off-1 / 0 off-2+ | 0.14 |
| Area | down 1.5x | max 1 / 99.9989% exact / 11 off-1 / 0 off-2+ | 0.07 |
| Area | down 2x | max 1 / 99.9990% exact / 6 off-1 / 0 off-2+ | 0.06 |
| Area | down 2.37x | max 1 / 99.9983% exact / 7 off-1 / 0 off-2+ | 0.06 |
| Area | down 8x | max 1 / 99.9973% exact / 1 off-1 / 0 off-2+ | 0.05 |
| Area | down 16x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 0.05 |
| Area | down 32x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 0.05 |
| Area | down 64x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 0.05 |
| Area | down 128x | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 0.05 |
| Area | down to 2x2 | max 0 / 100.0000% exact / 0 off-1 / 0 off-2+ | 0.05 |
| Area | aniso 1x/16x | max 1 / 99.9993% exact / 1 off-1 / 0 off-2+ | 0.05 |

## E9 Summary: worst case per geometry over all opaque U8 cases (sRGB and Display P3)

| Filter | Geometry | f32 worst max | f32 min exact % | f32 mean |delta| | fixed worst max | fixed min exact % | fixed mean |delta| | fixed off-2+ total |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Lanczos3 | identity | 1 | 99.9999 | 0.00000 | 1 | 99.4432 | 0.00051 | 0 |
| Lanczos3 | up 1.5x | 1 | 99.8972 | 0.00010 | 1 | 94.8489 | 0.00786 | 0 |
| Lanczos3 | down 1.5x | 1 | 99.9991 | 0.00000 | 1 | 92.5936 | 0.01385 | 0 |
| Lanczos3 | down 2x | 1 | 99.4166 | 0.00053 | 1 | 90.9507 | 0.01440 | 0 |
| Lanczos3 | down 2.37x | 1 | 99.9990 | 0.00000 | 1 | 95.2063 | 0.00668 | 0 |
| Lanczos3 | down 8x | 1 | 99.9973 | 0.00000 | 1 | 93.3757 | 0.01459 | 0 |
| Lanczos3 | down 16x | 0 | 100.0000 | 0.00000 | 1 | 92.9905 | 0.01723 | 0 |
| Lanczos3 | down 32x | 1 | 99.9566 | 0.00004 | 1 | 97.1354 | 0.00923 | 0 |
| Lanczos3 | down 64x | 0 | 100.0000 | 0.00000 | 1 | 97.7431 | 0.00395 | 0 |
| Lanczos3 | down 128x | 0 | 100.0000 | 0.00000 | 1 | 96.5278 | 0.00821 | 0 |
| Lanczos3 | down to 2x2 | 0 | 100.0000 | 0.00000 | 0 | 100.0000 | 0.00000 | 0 |
| Lanczos3 | aniso 1x/16x | 1 | 99.9993 | 0.00000 | 1 | 93.4001 | 0.01631 | 0 |
| Area | identity | 1 | 99.9999 | 0.00000 | 1 | 99.4432 | 0.00051 | 0 |
| Area | up 1.5x | 1 | 99.8303 | 0.00027 | 1 | 94.9190 | 0.00750 | 0 |
| Area | down 1.5x | 1 | 99.9961 | 0.00001 | 1 | 95.4992 | 0.00557 | 0 |
| Area | down 2x | 1 | 99.4071 | 0.00093 | 1 | 93.2766 | 0.01162 | 0 |
| Area | down 2.37x | 1 | 99.9995 | 0.00000 | 1 | 95.2839 | 0.00549 | 0 |
| Area | down 8x | 1 | 99.6528 | 0.00032 | 1 | 93.5981 | 0.00793 | 0 |
| Area | down 16x | 0 | 100.0000 | 0.00000 | 1 | 92.3503 | 0.00849 | 0 |
| Area | down 32x | 0 | 100.0000 | 0.00000 | 1 | 96.4844 | 0.00387 | 0 |
| Area | down 64x | 0 | 100.0000 | 0.00000 | 1 | 97.5694 | 0.00268 | 0 |
| Area | down 128x | 0 | 100.0000 | 0.00000 | 1 | 95.8333 | 0.00442 | 0 |
| Area | down to 2x2 | 0 | 100.0000 | 0.00000 | 1 | 58.3333 | 0.07576 | 0 |
| Area | aniso 1x/16x | 1 | 99.9932 | 0.00001 | 1 | 95.2603 | 0.00673 | 0 |

Worst case per geometry over the F32 input cases (f32 path only):

| Filter | Geometry | f32 worst max | f32 min exact % |
| --- | --- | --- | --- |
| Lanczos3 | identity | 1 | 99.9990 |
| Lanczos3 | up 1.5x | 1 | 99.9990 |
| Lanczos3 | down 1.5x | 1 | 99.9989 |
| Lanczos3 | down 2x | 1 | 99.9983 |
| Lanczos3 | down 2.37x | 1 | 99.9988 |
| Lanczos3 | down 8x | 1 | 99.9973 |
| Lanczos3 | down 16x | 0 | 100.0000 |
| Lanczos3 | down 32x | 0 | 100.0000 |
| Lanczos3 | down 64x | 0 | 100.0000 |
| Lanczos3 | down 128x | 0 | 100.0000 |
| Lanczos3 | down to 2x2 | 0 | 100.0000 |
| Lanczos3 | aniso 1x/16x | 1 | 99.9986 |
| Area | identity | 1 | 99.9990 |
| Area | up 1.5x | 1 | 99.9994 |
| Area | down 1.5x | 1 | 99.9989 |
| Area | down 2x | 1 | 99.9990 |
| Area | down 2.37x | 1 | 99.9983 |
| Area | down 8x | 1 | 99.9973 |
| Area | down 16x | 1 | 99.9891 |
| Area | down 32x | 0 | 100.0000 |
| Area | down 64x | 0 | 100.0000 |
| Area | down 128x | 0 | 100.0000 |
| Area | down to 2x2 | 0 | 100.0000 |
| Area | aniso 1x/16x | 1 | 99.9986 |

Per input, all geometries and both filters pooled (RGB where the reference alpha is nonzero; alpha channel separately):

| Input | samples | f32 RGB | f32 alpha | fixed RGB | fixed alpha |
| --- | --- | --- | --- | --- | --- |
| noise | 19843128 | max 1 / 99.9997% exact / mean 0.00000 | - | max 1 / 99.4823% exact / mean 0.00518 | - |
| dark-noise (codes 0-24) | 19843128 | max 1 / 99.9369% exact / mean 0.00063 | - | max 1 / 96.1278% exact / mean 0.03872 | - |
| bright-noise (codes 231-255) | 19843128 | max 1 / 99.9996% exact / mean 0.00000 | - | max 1 / 99.7910% exact / mean 0.00209 | - |
| gradient | 19843128 | max 1 / 99.9090% exact / mean 0.00091 | - | max 1 / 98.7261% exact / mean 0.01274 | - |
| flat-blocks (32px constant blocks) | 19843128 | max 1 / 99.9998% exact / mean 0.00000 | - | max 1 / 99.8678% exact / mean 0.00132 | - |
| checker-1px | 19843128 | max 0 / 100.0000% exact / mean 0.00000 | - | max 1 / 99.6090% exact / mean 0.00391 | - |
| checker-2px | 19843128 | max 0 / 100.0000% exact / mean 0.00000 | - | max 1 / 99.9769% exact / mean 0.00023 | - |
| saturated-edges | 19843128 | max 0 / 100.0000% exact / mean 0.00000 | - | max 1 / 99.9767% exact / mean 0.00023 | - |
| p3-primaries (out-of-gamut edges) | 19843128 | max 1 / 100.0000% exact / mean 0.00000 | - | max 1 / 99.9715% exact / mean 0.00029 | - |
| p3-checker-1px (red/green) | 19843128 | max 0 / 100.0000% exact / mean 0.00000 | - | max 1 / 99.9692% exact / mean 0.00031 | - |
| p3-noise | 19843128 | max 1 / 99.9996% exact / mean 0.00000 | - | max 1 / 99.3384% exact / mean 0.00662 | - |
| alpha-edges (opaque shapes on transparent) | 15755067 | max 1 / 99.9977% exact / mean 0.00002 | max 1 / 99.8647% exact / mean 0.00135 | max 31 / 99.7181% exact / mean 0.00984 | max 1 / 99.8638% exact / mean 0.00136 |
| alpha-noise | 19732656 | max 1 / 99.9996% exact / mean 0.00000 | max 1 / 99.1341% exact / mean 0.00866 | max 23 / 98.3277% exact / mean 0.01986 | max 1 / 98.7756% exact / mean 0.01224 |
| f32-noise | 19843128 | max 1 / 99.9992% exact / mean 0.00001 | - | - | - |
| f32-dark (0-0.01) | 19843128 | max 0 / 100.0000% exact / mean 0.00000 | - | - | - |
| f32-extended (-0.2 to 1.5) | 19843128 | max 1 / 99.9994% exact / mean 0.00001 | - | - | - |
| f32-p3-noise | 19843128 | max 1 / 99.9991% exact / mean 0.00001 | - | - | - |

Largest fixed-point accumulator observed in any chain: 0.3771 of 2^31 (largest planner bound 0.4823); largest intermediate row value: 0.7732 of 32767 (largest planner bound 0.8096). Precision pairs (h, v) chosen: {(14, 14): 52, (14, 15): 26, (15, 15): 234}.

Chains took 2.0 s on 20 threads; whole run 3.0 s.
