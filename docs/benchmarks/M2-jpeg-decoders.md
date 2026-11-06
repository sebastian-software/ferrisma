# M2: JPEG decoder candidates (libjpeg-turbo, jpegli, zune-jpeg)

- **Date:** 2026-11-06
- **Decision context:** [ADR-0025](../adr/0025-codec-selection-reference-conformance-and-image-scope.md) point 7 (choosing the JPEG decoder for a sister port) and [ADR-0023](../adr/0023-decode-side-work-and-scaling-policy.md) decision 3 (shrink-on-load as a measured quality policy)
- **Harness:** [`scripts/bench/jpeg-decoders`](../../scripts/bench/jpeg-decoders): `build.sh` builds the pinned libraries and the harnesses, `run.py` runs all measurements, and `jpegdec.c` is a single libjpeg-API harness compiled once against each library

## Setup

| | |
| --- | --- |
| Machine | Apple M1 Ultra, macOS 27.0.1, shared; 1-minute load 3.8–4.3 during the runs |
| libjpeg-turbo | 3.2.1 (`7b49937`), static, CMake Release, Apple clang 21, its own NEON SIMD; default `JDCT_ISLOW` and fancy upsampling |
| jpegli | `google/jpegli` `031a007`, static `jpegli-static`, CMake Release, Apple clang 21, Highway NEON dispatch; `jpegli_*` API |
| zune-jpeg | 0.5.15 with Ferrixel's features (`std`, `neon`, `x86`), rustc 1.100 nightly, release |
| Mode | One thread (none of the three decoders threads), decode from memory to interleaved RGB, best and median of 7–60 runs |

Quality is SSIMULACRA2 from `scripts/bench/quality-match`, with jpegli's `f32` output clamped to [0, 1] for scoring.

## Speed (full decode, single thread)

Best time in ms. The real photographs are the 12 Mpx phone photo (`dog.jpg`, 4:2:0) and libjxl's flower JPEG (3.4 Mpx, q85 4:2:0). The 24 and 48 Mpx files are synthetic plasma from `make-corpus.sh`.

| image | libjpeg-turbo | zune-jpeg | jpegli u8 | jpegli f32 |
| --- | ---: | ---: | ---: | ---: |
| kodim03 768×512 | **1.22** | 1.75 | 2.08 | 2.01 |
| flower 2268×1512 | **7.59** | 11.38 | 14.99 | 14.85 |
| dog 4032×3024 | **39.3** | 53.9 | 69.9 | 67.2 |
| plasma 6000×4000 | **39.3** | 62.9 | 103.1 | 100.1 |
| plasma 8000×6000 | **77.6** | 123.4 | 207.3 | 199.9 |

- libjpeg-turbo is 1.4–1.6× faster than zune-jpeg, Ferrixel's current decoder, and 1.7–2.7× faster than jpegli.
- jpegli's `f32` output is as fast as its `u8` output, or slightly faster.

## Speed of scaled decode

Best time in ms, single thread; the scale in parentheses is relative to full decode.

| image | decoder | 1/1 | 1/2 | 1/4 | 1/8 |
| --- | --- | ---: | ---: | ---: | ---: |
| dog 12 Mpx | libjpeg-turbo | 39.3 | 32.4 (0.82) | 30.1 (0.77) | 27.1 (0.69) |
| dog 12 Mpx | jpegli f32 | 67.2 | 48.8 (0.73) | 47.5 (0.71) | 39.8 (0.59) |
| plasma 48 Mpx | libjpeg-turbo | 77.6 | 59.8 (0.77) | 53.1 (0.68) | 46.8 (0.60) |
| plasma 48 Mpx | jpegli f32 | 199.9 | 124.7 (0.62) | 118.1 (0.59) | 88.7 (0.44) |

Entropy decoding always runs at full resolution. Scaled decode therefore saves at most 31–41 % of the decode time on the photo; the larger saving is the smaller image that every later stage handles.

## Decoder quality on the same file

The originals are lossless:
- kodim03 and kodim23, 768×512, from libavif's 4:2:0 `.y4m` test data.
- The dog and flower photographs, downscaled 4× and 3× in linear light with Lanczos3, so that the camera's JPEG artifacts average out.

libjpeg-turbo's `cjpeg` encodes each original. The table shows SSIMULACRA2 of each decoder's output against the original (higher is better).

| image | setting | bytes | libjpeg-turbo | jpegli u8 | jpegli f32 | zune-jpeg |
| --- | --- | ---: | ---: | ---: | ---: | ---: |
| kodim03 | q50 4:2:0 | 30,124 | 61.45 | 62.00 | 62.08 | 61.39 |
| kodim03 | q75 4:2:0 | 45,543 | 74.52 | 75.80 | 75.91 | 74.44 |
| kodim03 | q90 4:2:0 | 79,138 | 84.87 | 86.35 | 86.48 | 84.85 |
| kodim03 | q90 4:4:4 | 93,972 | 87.25 | 87.96 | 88.08 | 87.26 |
| kodim23 | q50 4:2:0 | 27,861 | 64.47 | 64.94 | 64.98 | 64.45 |
| kodim23 | q75 4:2:0 | 42,179 | 76.78 | 77.45 | 77.53 | 76.66 |
| kodim23 | q90 4:2:0 | 77,731 | 85.78 | 86.53 | 86.61 | 85.63 |
| kodim23 | q90 4:4:4 | 98,423 | 88.28 | 88.64 | 88.74 | 88.22 |
| dog 1008×756 | q50 4:2:0 | 87,083 | 63.77 | 63.99 | 64.02 | 63.70 |
| dog 1008×756 | q75 4:2:0 | 131,276 | 75.12 | 75.60 | 75.61 | 75.06 |
| dog 1008×756 | q90 4:2:0 | 223,452 | 84.63 | 85.18 | 85.24 | 84.49 |
| dog 1008×756 | q90 4:4:4 | 265,147 | 87.55 | 88.01 | 88.09 | 87.54 |
| flower 756×504 | q50 4:2:0 | 50,474 | 68.12 | 68.62 | 68.63 | 68.14 |
| flower 756×504 | q75 4:2:0 | 74,896 | 77.47 | 78.21 | 78.23 | 77.47 |
| flower 756×504 | q90 4:2:0 | 124,931 | 85.76 | 86.25 | 86.28 | 85.71 |
| flower 756×504 | q90 4:4:4 | 163,587 | 89.17 | 89.74 | 89.78 | 89.17 |

- jpegli scores 0.2–1.6 points higher than libjpeg-turbo on every file. The largest gains are at q75–q90 with 4:2:0.
- For scale: going from q75 to q90 adds about 10 points for 70 % more bytes. jpegli's gain is therefore real, but small.
- `f32` output adds 0.01–0.13 points over jpegli's own `u8` output.
- zune-jpeg is within 0.15 points of libjpeg-turbo.

## Shrink-on-load loss

This measures how much a decoder loses when it decodes at reduced scale and then resizes, instead of decoding at full size and resizing. The reference is the decoder's own full decode, resized in linear light with Lanczos3 to the target. The candidate is its scaled decode, resized the same way. Both decoders scale in the DCT domain, i.e. in gamma-encoded YCbCr.

| image | target | scale (decoded size) | residual | libjpeg-turbo | jpegli f32 |
| --- | --- | --- | ---: | ---: | ---: |
| dog | 400×300 | 1/2 (2016×1512) | 5.0× | 95.15 | 95.33 |
| dog | 400×300 | 1/4 (1008×756) | 2.5× | 89.72 | 89.88 |
| dog | 400×300 | 1/8 (504×378) | 1.3× | 81.34 | 81.48 |
| dog | 1000×750 | 1/2 (2016×1512) | 2.0× | 92.17 | 92.42 |
| dog | 1000×750 | 1/4 (1008×756) | 1.0× | 80.13 | 80.36 |
| flower | 400×267 | 1/2 (1134×756) | 2.8× | 96.52 | 97.06 |
| flower | 400×267 | 1/4 (567×378) | 1.4× | 91.27 | 92.01 |
| flower | 1000×667 | 1/2 (1134×756) | 1.1× | 93.18 | 93.75 |

- Under libvips' rule (at least a 2× residual), shrink-on-load scores 89.7–96.5 against Ferrixel's linear-light result.
- Even at a 5× residual, the photo loses about 5 points, so the loss is not only resolution.
- Both decoders lose about the same amount.
- This supports ADR-0023: shrink-on-load stays an explicit, reported quality policy and is never a silent optimization.

## Port size

| | libjpeg-turbo 3.2.1 | jpegli `031a007` |
| --- | --- | --- |
| Library source | about 43k lines of C in `src/j*` (decoder, encoder, lossless, arithmetic) | about 21k lines of C++ in `lib/jpegli` (decoder and encoder) |
| SIMD | about 7k lines of NEON intrinsics in C (`simd/arm`); about 14k lines of NASM assembly for x86-64 | Highway (portable C++ SIMD library), one source for all targets |
| Linked `__TEXT` of the harness | 344 kB (static link, referenced objects only) | 131 kB |

A decoder-only port keeps only part of either code base. For libjpeg-turbo, the x86-64 SIMD is assembly that a Rust port has to rewrite as intrinsics or replace.

## Reading the results under ADR-0025

- **Speed:** libjpeg-turbo wins clearly. Replacing zune-jpeg with it would cut the decode stage of the 12 Mpx photo from about 54 to 39 ms on one thread.
- **Quality:** jpegli wins on every file, by 0.2–1.6 SSIMULACRA2. It pays for that with 1.7–2.7× the decode time.
- **Origin:** libjpeg-turbo descends from the IJG reference libjpeg 6b. jpegli deliberately reconstructs differently, which makes the ISO/IEC 10918-2 conformance evidence required by ADR-0025 point 1 all the more important for it.
- **Port effort:** libjpeg-turbo is larger and has x86 SIMD in assembly. jpegli is smaller, and its SIMD is one portable source.

Not covered yet:
- x86-64.
- JPEGs from other encoders, such as phone hardware encoders, progressive files and restart intervals.
- The effect of the decoder's quality difference after Ferrixel's downscale.
- Conformance runs.
- Memory.

## Reproduce

```sh
scripts/bench/jpeg-decoders/build.sh WORK
(cd scripts/bench/quality-match && cargo build --release)
python3 scripts/bench/compare-sharp.py ...   # once, for CORPUS (dog, flower, Kodak PNGs)
scripts/bench/make-corpus.sh SYNTH           # 24 and 48 Mpx plasma
python3 scripts/bench/jpeg-decoders/run.py WORK CORPUS SYNTH/single OUT
```
