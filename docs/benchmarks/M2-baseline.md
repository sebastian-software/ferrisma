# M2 baseline harness and first numbers

**Status:** work package 0 of [M2](../milestones/M2-performance-architecture.md), in progress. This page records how the baseline is measured and what was observed on one machine. Nothing here is a performance claim; comparators are not quality-matched yet.

## What is measured

- **Stage timings** come from the pipeline itself. `ProcessingReport::timings` ([ADR-0017](../adr/0017-minimal-public-facade-and-cli.md)) holds wall-clock durations for `open` (option validation, header parsing, orientation and ICC setup, graph construction), `decode` (time inside the source's region reader excluding the per-row transform callback: codec decode, reorientation, ICC conversion), `transform` (transfer decode, resampling, quantization, copying rows into the encoder surface), `encode` (AVIF encoding of the materialized frame) and `total`. The CLI prints them and a stable `report: key=value ...` line.
- **End-to-end latency** is measured outside the process with `hyperfine` (one warmup, `RUNS` timed runs, mean, standard deviation, min, max).
- **Peak RSS** comes from one extra run under `/usr/bin/time -l` (macOS, bytes) or GNU `time -f %M` (Linux, KiB). It includes codec-internal memory that the engine budget deliberately does not cover.
- **Sequential throughput** runs one process per small image, so it includes process start, codec initialization and encoder setup for every image. Ferrixel has no batch API yet; this number is the ceiling a batch API has to beat.
- **Comparators** (Sharp through `scripts/bench/sharp-resize.cjs`, libvips through `vipsthumbnail --linear`) run only when installed and are labeled `comparable_quality=no`: filters, transfer intent, chroma mode, threading and AV1 encoders differ, and no SSIMULACRA2 matching exists yet.

## How to run

```sh
# Synthetic fallback corpus (plasma noise, deterministic); prefer real photographs.
scripts/bench/make-corpus.sh target/bench/corpus

# Latency, stage timings, peak RSS, sequential throughput; optional comparators.
BENCH_SHARP_NODE_MODULES=/path/to/node_modules \
  scripts/bench/run-baseline.sh target/bench/corpus target/bench/baseline 5

# Hotspots of one run (macOS: sample, Linux: perf).
scripts/bench/profile.sh target/bench/corpus/single/synthetic-24mpx-6000x4000.jpg 1600 1067 target/bench/profile
```

Settings are environment variables: `BENCH_WIDTH` (default 1600, aspect preserved), `BENCH_SMALL_WIDTH` (320), `BENCH_QUALITY` (80), `BENCH_SPEED` (6), `BENCH_FILTER` (`lanczos` or `area`), `BENCH_PROFILE` (`assume-srgb`, `strict`, `rendered-icc`), `BENCH_PRECISION` (`f32`, or `reference` for the M1 `f64` executor), `BENCH_ENCODER_THREADS` (1; 0 means all cores), `BENCH_CHROMA` (`444` or `420`), `BENCH_KERNEL_LEVEL` (`best`, `scalar`, `neon`, `avx2`; a ceiling), `BENCH_MAX_INPUT_MB` (512). The harness writes `results.csv`, `throughput.csv`, `summary.md`, `environment.txt` and the raw hyperfine JSON into the output directory.

## Corpus caveats

The synthetic corpus exists so that the harness runs anywhere. Plasma noise after a blur compresses far better than a photograph (the 12-megapixel JPEG is about 1 MB), so JPEG entropy decoding is cheaper than on real input, and AVIF encoding sees unusual content. Real-photo numbers must come from a corpus of photographs at 1, 12, 24 and 48 megapixels, including orientation, ICC-tagged, P3 and alpha cases, before any comparison is published. The committed `tests/fixtures/sample.jpg` is a correctness fixture, not a benchmark input.

## 2026-11-05 — first dev-machine baseline (synthetic corpus)

**Machine:** Apple M1 Ultra, 20 logical cores, 64 GiB, macOS 27.0 (Darwin 27.0.0, arm64). rustc 1.100.0-nightly (2026-09-06), hyperfine 2.0.0. Crates: zune-jpeg 0.5.15, jxl-oxide 0.12.6, avif-decode 3.0.0, rav1d 1.1.0, ravif 0.13.0, rav1e 0.8.1, moxcms 0.9.1. Ferrixel at `6984255` plus the uncommitted package-0 changes. Settings: `--assume-srgb --quality=80 --speed=6`, Lanczos3, output width 1600, 5 timed runs after 1 warmup. Comparators were not run: neither Sharp nor libvips is installed on this machine.

Wall time, CPU/wall and peak RSS are hyperfine and `/usr/bin/time` observations; the stage columns come from one additional run's `report:` line.

| image | input | output | mean s | ± s | CPU/wall | peak RSS MiB | out KiB | open ms | decode ms | transform ms | encode ms |
| --- | --- | --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| synthetic 1 Mpx JPEG | 1280×853 | 1600×1066 | 3.291 | 0.101 | 0.98 | 88.8 | 42 | 1 | 4 | 410 | 2743 |
| synthetic 12 Mpx JPEG | 4000×3000 | 1600×1200 | 6.185 | 0.018 | 0.99 | 138.8 | 86 | 1 | 44 | 2978 | 3175 |
| synthetic 24 Mpx JPEG | 6000×4000 | 1600×1067 | 8.832 | 0.079 | 0.99 | 167.5 | 91 | 1 | 93 | 5919 | 2947 |
| synthetic 48 Mpx JPEG | 8000×6000 | 1600×1200 | 13.219 | 0.196 | 0.99 | 250.8 | 105 | 1 | 191 | 10722 | 3642 |
| synthetic 12 Mpx AVIF | 4000×3000 | 1600×1200 | 6.350 | 0.040 | 1.01 | 163.4 | 85 | 1 | 107 | 3928 | 3582 |

Sequential throughput, 100 synthetic 640×427 JPEGs to 320×213 AVIF, one process per image, 2 runs: **26.25 s, 3.81 images/s** (about 262 ms per image, encoder-dominated at speed 6).

### Hotspots (24 Mpx JPEG, macOS `sample` at 1 ms)

`scripts/bench/profile.sh corpus/single/synthetic-24mpx-6000x4000.jpg 1600 1067 ...`. The sampler slowed the run to 11.8 s (transform 7.7 s, encode 3.9 s, decode 0.13 s). Top of stack, self samples:

| self samples | symbol | stage |
| ---: | --- | --- |
| 3655 (+162 stub) | `pow` (libsystem_m) | transform: sRGB transfer decode per filter tap |
| 1089 | `ferrixel::working_color::decode_to_linear_srgb` | transform |
| 1036 | `execute_quality_srgb8` row closure | transform: horizontal and vertical accumulation |
| 601 | `rav1e::transform::inverse::rust::inverse_transform_add` | encode |
| 268 | `rav1e::cdef::rust::cdef_filter_block` | encode |
| 185 | `rav1e::lrf::rust::sgrproj_box_ab_r1` | encode |
| 175 | `rav1e::transform::forward::rust::forward_transform` | encode |
| 167 | `rav1e::quantize::QuantizationContext::quantize` | encode |
| 29 + 28 + 22 | `zune_jpeg` MCU decode, YCbCr→RGB, IDCT | decode |
| 20 | `VecDeque<CachedRow>::push_back` | transform: row cache |

Every rav1e symbol in the list carries the `rust::` fallback path: the encoder runs its scalar kernels. Verified cause: Ferrixel declares `ravif` with `default-features = false`, which drops ravif's default `asm` and `threading` features (ravif 0.13.0 `Cargo.toml`); rav1e 0.8.1 ships x86-64 and aarch64 assembly behind `asm` (`src/asm/aarch64`, `build.rs`), so this build uses neither NEON kernels nor the encoder's own thread pool, and `AvifSink` additionally pins `with_num_threads(Some(1))`. Enabling `asm` pulls in C-compiled and NASM-assembled foreign code, which ADR-0018's dependency policy has to weigh; that trade-off is package 2's first measurement.

### What the numbers say

1. **The transform is a `pow` benchmark.** `pow` plus `decode_to_linear_srgb` are about 4.7 k of the roughly 6 k transform samples: around 80 % of the transform stage is the sRGB transfer decode, executed once per filter tap instead of once per source sample. Lanczos3 touches every source sample about six times. Transform time grows linearly with *input* pixels (0.22–0.25 s per megapixel) and is independent of the output size. Package 1's LUT-based, once-per-sample linearization removes most of it.
2. **The encoder is a fixed 2.7–3.9 s per 1600-wide output**, independent of input size, because `ravif` is built without its `asm` and `threading` features and the sink pins rav1e to one thread at speed 6. For the 1 Mpx input it is 83 % of wall time. Package 2 measures threads, speed presets, kernel features, YUV input and encoder alternatives.
3. **Decode is negligible on this corpus**: 4 ms per megapixel for JPEG, 107 ms for the 12 Mpx AVIF. Plasma content is easy to entropy-decode; real photographs will cost more, and the JPEG adapter still materializes the full frame (72 MB at 24 Mpx, 144 MB at 48 Mpx), which dominates peak RSS.
4. **CPU/wall is 1.0 on a 20-core machine.** Nothing runs in parallel; package 4 (tile parallelism) and package 2 (encoder threads) are the only ways to use the hardware.
5. **Sequential throughput is 3.8 images/s**, set by process start plus the encoder; a batch API with a warm encoder and pool (package 4) has this as its floor.

None of this compares to Sharp or libvips yet. Installing either comparator and a real-photo corpus is the next step of package 0; quality matching through SSIMULACRA2 remains open.

## 2026-11-05 — after work package 1 (scalar `f32` executor)

Same machine, corpus, settings and harness as above; `BENCH_PRECISION=f32` (the new default). The executor is `src/fast_execute.rs`: `f32` working precision, transfer decode once per source sample through an exact 256-entry table, flat precomputed tap tables, an O(1) ring buffer of horizontally filtered rows, three channels for opaque RGB, and an exact table-driven `f32` to sRGB8 encoder (ADR-0021). The `f64` executor stays as the oracle (`--reference`, `BENCH_PRECISION=reference`).

| image | wall s before | wall s after | transform ms before | transform ms after | transform speedup | encode ms after | peak RSS MiB after |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| synthetic 1 Mpx JPEG | 3.291 | 2.785 | 410 | 25 | 16× | 2800 | 88.6 |
| synthetic 12 Mpx JPEG | 6.185 | 3.312 | 2978 | 100 | 30× | 3147 | 135.9 |
| synthetic 24 Mpx JPEG | 8.832 | 3.197 | 5919 | 168 | 35× | 2951 | 163.0 |
| synthetic 48 Mpx JPEG | 13.219 | 3.777 | 10722 | 268 | 40× | 3320 | 244.0 |
| synthetic 12 Mpx AVIF | 6.350 | 3.414 | 3928 | 100 | 39× | 3166 | 161.7 |

Standard deviations after: 0.005–0.073 s over 5 runs. Sequential throughput (100 small images, one process each): **15.30 s, 6.54 images/s**, up from 3.81.

Agreement with the `f64` oracle, from the differential test `fast_execute_tests` (16 synthetic 72×60 inputs × 11 geometries including crops × Area and Lanczos3; ADR-0021 rules A–E): every sample within one code; exact RGB share 100 % on 10 inputs, lowest 99.9135 % (gradient) and 99.9237 % (dark noise); exact alpha share 99.1569 % on RGBA noise; mean absolute error at most 0.00087.

### What the numbers say now

1. **The transform is no longer the bottleneck.** It costs 5.6 ms per input megapixel (about 40× less than the per-tap `pow` path) and is 3–7 % of wall time. SIMD and tiling (packages 3 and 4) will matter for throughput once the encoder is fixed, not before.
2. **The encoder is now 86–100 % of wall time** for every image: about 3 s for a 1600-wide output, single-threaded, scalar rav1e. Package 2 is the next lever by an order of magnitude.
3. **Peak RSS is unchanged** (the full JPEG frame still dominates); engine-managed memory for the transform is smaller than in the reference, which the differential test asserts.

## 2026-11-06 — after work package 2 (encoder features and threads)

The `avif-write` feature now enables `ravif`'s `asm` and `threading` features, and `TransformOptions::encoder_threads` (CLI `--encoder-threads`) sets the encoder's thread count; 1 is the default, 0 means all cores. Evidence for the choice comes from the quality-matched [encoder experiment](../research/encoder-experiment/results/2026-11-06-apple-m1-ultra.md) on real photographs, summarized below. Same machine, corpus and harness as above.

| image | after package 1 | after package 2, 1 encoder thread | after package 2, all 20 threads | encode ms, 1 thread → 20 | out KiB, 1 thread → 20 |
| --- | ---: | ---: | ---: | ---: | ---: |
| synthetic 1 Mpx JPEG | 2.785 s | 1.150 s | 0.195 s | 1108 → 159 | 42 → 48 |
| synthetic 12 Mpx JPEG | 3.312 s | 1.501 s | 0.345 s | 1342 → 184 | 87 → 92 |
| synthetic 24 Mpx JPEG | 3.197 s | 1.554 s | 0.436 s | 1287 → 158 | 91 → 95 |
| synthetic 48 Mpx JPEG | 3.777 s | 1.948 s | 0.652 s | 1454 → 182 | 105 → 111 |
| synthetic 12 Mpx AVIF | 3.414 s | 1.570 s | 0.407 s | 1368 → 235 | 85 → 91 |

Standard deviations 0.006–0.013 s over 5 runs. Sequential throughput (100 small images, one process each): **13.29 images/s** with one encoder thread, **27.91 images/s** with all cores, up from 6.54 after package 1 and 3.81 at the start of M2. The output bytes with one thread are identical to before package 2. With all threads the files grow because `ravif` derives the AV1 tile count from the thread count; at fixed quality 80 the encoder output is also slightly different in quality, so the bytes column is not a quality-matched comparison (the experiment is).

From the start of M2 to now, 24 Mpx JPEG to 1600-wide AVIF went from 8.83 s to 1.55 s on one encoder thread (5.7×) and to 0.44 s on all cores (20×), at the same output with one thread.

### Encoder experiment summary (quality-matched, SSIMULACRA2)

Five images: two real photographs downscaled to 1600 wide (a 12 Mpx phone photo from libavif's test data and a 3.4 Mpx photo from libjxl's), two 768×512 Kodak images and the synthetic plasma. Anchor: `ravif` speed 6, one thread, quality 80. Every other variant is bisected to the anchor's SSIMULACRA2 score.

| knob | measured effect | decision |
| --- | --- | --- |
| `asm` (rav1e NEON assembly) | **bit-identical** output (equal FNV-1a hashes on all five images); encode 2.1–2.4× faster (3291 → 1523 ms on the phone photo) | enabled in `avif-write` |
| threads 2 / 4 / 8 / 20 with `ravif` | 1.7–1.9× / 3.0–3.5× / 4.8–5.7× / 7.4–9.1× faster on 1600-wide images; **+0.4–1.4 % / +1.1–2.7 % / +1.3–4.7 % / +3.2–6.5 % bytes** at equal score, because tiles follow threads | `threading` enabled; default stays 1 thread for machine-independent output |
| speed 7–8 at 20 threads | identical bytes for 7 and 8; +4.4–9.0 % bytes, about 1.3× faster than speed 6 at 20 threads | not changed |
| speed 9 / 10 at 20 threads | 18–34× faster than the anchor; +6.1–15.0 % / +12.5–22.5 % bytes | not changed; a candidate for an explicit "fast" preset |
| `rav1e` direct, 4:4:4 10-bit | same bytes as `ravif` at the same threads (validates the direct path) | — |
| `rav1e` direct, 4:2:0 10-bit, 1 thread | **−0.5 % to −11.9 % bytes** at equal score, 1.3–1.4× faster on 1600-wide images | follow-up: direct `rav1e` sink |
| `rav1e` direct, 4:2:0 8-bit | +1.0–10.8 % bytes against the anchor | rejected; 10-bit stays |
| libavif-rs encoders (libaom port, SVT-AV1) | not measured yet; since `f3b2981` (2026-11-06) libavif-rs runs on macOS ARM64 and GNU Linux x86-64, but needs nightly, an `unsafe` C ABI and libvmaf, so only a standalone experiment can measure it | open, see the [codec capability audit](../research/2026-11-05-decode-scaling-and-encoder-capabilities.md) |

### What the numbers say now

1. **Encode is still the largest stage** with one thread (about 85 % of wall time), but with all cores decode plus transform (0.27 s at 24 Mpx) now cost more than encode (0.16 s). Packages 3–5 (SIMD transform, tiled parallel executor, decode-side scaling) become the next levers for large inputs.
2. **Multi-threaded encoding costs bytes through tiles, not threads.** A direct `rav1e` adapter can set tiles from the image size and threads from the machine independently, which removes the trade-off and makes multi-threaded output machine-independent; it also unlocks 4:2:0 10-bit, the only knob that saved bytes at equal quality. That adapter is the remaining part of package 2.
3. **For batch work one encoder thread per image is the efficient setting**: it has the smallest files and leaves cores to other images; the package 4 engine should schedule images, not tiles, when many are queued.

## 2026-11-06 — Ferrixel's own `rav1e` sink

`avif-write` now drives `rav1e` directly (`src/avif_sink.rs`) instead of through `ravif`. Rows are converted to BT.601 full-range 10-bit YCbCr on arrival and written into `rav1e`'s input frame, so the engine no longer keeps an RGBA copy of the output; the frame's exact size is charged to the budget. AV1 tiles depend only on the output size (one per 256×256 pixels, at most 64) whenever `encoder_threads` is not 1, so parallel output is the same on every machine and for every thread count. `ChromaSubsampling::Yuv420` (CLI `--chroma=420`) is new. With one thread and 4:4:4 the bytes are identical to the `ravif` path (sink candidate run of the [encoder experiment](../research/encoder-experiment/results/2026-11-06-apple-m1-ultra.md), and unchanged output sizes below).

| image | 1 thread 4:4:4 | all cores 4:4:4 (`ravif`, tiles = threads) | all cores 4:4:4 (size tiles) | all cores 4:2:0 (size tiles) |
| --- | ---: | ---: | ---: | ---: |
| synthetic 1 Mpx JPEG | 1.154 s, 42 KiB | 0.195 s, 48 KiB | 0.180 s, 48 KiB | 0.133 s, 42 KiB |
| synthetic 12 Mpx JPEG | 1.511 s, 87 KiB | 0.345 s, 92 KiB | 0.303 s, 93 KiB | 0.263 s, 75 KiB |
| synthetic 24 Mpx JPEG | 1.547 s, 91 KiB | 0.436 s, 95 KiB | 0.414 s, 95 KiB | 0.385 s, 75 KiB |
| synthetic 48 Mpx JPEG | 1.923 s, 105 KiB | 0.652 s, 111 KiB | 0.608 s, 112 KiB | 0.576 s, 87 KiB |
| synthetic 12 Mpx AVIF | 1.570 s, 85 KiB | 0.407 s, 91 KiB | 0.356 s, 92 KiB | 0.317 s, 75 KiB |

Peak RSS fell with the RGBA copy: 24 Mpx all cores 4:4:4 164.4 MiB (169.1 before), 4:2:0 130.5 MiB. Sequential throughput of 100 small images: 13.34 images/s with one thread, 13.62 with all cores at 4:4:4 and 18.07 at 4:2:0. The previous all-cores figure of 27.91 images/s came from `ravif` splitting each 320×213 image into four tiles; the size rule gives such images one tile, which keeps them smaller, and throughput across many images is the engine's job (work package 4), not per-image tiling.

The 4:2:0 byte column is at equal quality *setting*, not equal quality. The quality-matched experiment measured 4:2:0 10-bit at −0.5 % to −15 % bytes against 4:4:4 at the same SSIMULACRA2 on five images.

A denser rule was measured against the first candidate: 512×512 pixels per tile encoded 24 Mpx in 0.488 s with 93 KiB, 256×256 in 0.414 s with 95 KiB; the sink uses 256×256 for parallel encoding, and one-thread encoding keeps a single tile for the smallest files.

## 2026-11-06 — work package 3, first step: kernel crate with NEON and AVX2

The f32 executor's resample now runs on `crates/ferrixel-kernels` (ADR-0020) with runtime dispatch: `scalar`, `neon` and `avx2` levels for the horizontal pass and the vertical `axpy`. Pixels are stored as four `f32` channels with contiguous, zero-padded taps. `--kernel-level` (or `BENCH_KERNEL_LEVEL`) sets a ceiling; the report names the level. Same machine and corpus, all encoder cores, 4:4:4.

| image | transform ms before | transform ms, `scalar` | transform ms, `neon` | wall s, `neon` |
| --- | ---: | ---: | ---: | ---: |
| synthetic 1 Mpx JPEG | 26 | 23 | 22 | 0.157 |
| synthetic 12 Mpx JPEG | 102 | 69 | 69 | 0.260 |
| synthetic 24 Mpx JPEG | 169 | 107 | 106 | 0.345 |
| synthetic 48 Mpx JPEG | 269 | 192 | 191 | 0.518 |
| synthetic 12 Mpx AVIF | 104 | 69 | 69 | 0.314 |

"Before" is the 256×256-tile all-cores run of the `rav1e` sink section. Output bytes are unchanged (`neon` and `scalar` are bit-identical to the previous executor and to each other).

### What the numbers say

1. **The layout, not the intrinsics, made this step faster on Apple M1.** The scalar body over four-channel pixels is vectorized by LLVM with the NEON baseline, so `scalar` and `neon` run at the same speed; both are 1.4–1.6× faster than the three-channel per-tap loop of package 1. On x86-64 the baseline has no FMA, so the scalar body calls `fmaf`; `avx2` should matter there, and it is covered by the differential tests on the CI x86-64 runner, but no x86-64 timing exists yet.
2. **Interleaving accumulators did not help.** Four output positions with separate accumulators measured 116 ms (`neon`) and 155 ms (`scalar`) at 24 Mpx and was reverted; iterating taps with chunk iterators instead of indices changed nothing (192 ms at 48 Mpx either way).
3. **Where the transform time goes now** (48 Mpx, `sample`): horizontal pass about 57 %, linearization and quantization about 33 %, vertical pass about 10 %. The remaining single-core levers are the Q1.14 integer path (twice the lanes, ADR-0021 rule 2) and table-driven linearize/quantize kernels; the larger lever is running the row loop on several cores (work package 4).

## 2026-11-06 — kernel levels on three hardware classes

`crates/ferrixel-kernels/examples/bench.rs` times the resample kernels on the shape of the 24 Mpx row (6000×4000 to 1600×1067, Lanczos3, synthetic data). Apple M1 locally; x86-64 and Arm on GitHub-hosted runners in [CI run 42179398696](https://github.com/sebastian-software/ferrixel/actions/runs/42179398696). Hosted runners are shared virtual machines; treat their numbers as indicative.

| hardware | level | horizontal ms | vertical ms | total ms |
| --- | --- | ---: | ---: | ---: |
| Apple M1 Ultra | scalar | 63.0 | 13.8 | 76.8 |
| Apple M1 Ultra | neon | 61.7 | 13.7 | 75.5 |
| x86-64 runner (AVX-512) | scalar | 1679.7 | 440.2 | 2119.9 |
| x86-64 runner | avx2 | 53.2 | 15.6 | 68.8 |
| x86-64 runner | avx512 | 37.7 | 13.2 | 50.9 |
| Arm runner (Neoverse, SVE2) | scalar | 74.7 | 20.3 | 95.0 |
| Arm runner | neon | 78.3 | 20.1 | 98.4 |
| Arm runner | sve2 (autovectorized, removed) | 74.6 | 62.1 | 136.7 |

The same run timed a Q1.14 prototype (ADR-0021 rule 2) against these `f32` kernels: 1.03× on M1, 0.51× (AVX2) and 0.57× (AVX-512) on x86-64, 0.68–0.90× on Arm. It stayed below the 1.25× threshold everywhere and was dropped; details in ADR-0021's amendment log.

Engine-level transform times on M1 are unchanged by this step (24 Mpx 106 ms, 48 Mpx 191 ms with all encoder cores); no x86-64 end-to-end timing exists yet because the harness needs ImageMagick and the codecs on the runner.

## 2026-11-06 — work package 4: parallel row pipeline and batch engine

Transform stage alone (best of three runs, one encoder thread, `--transform-threads=N`):

| threads | 24 Mpx transform ms | 48 Mpx transform ms |
| ---: | ---: | ---: |
| 1 | 104 | 194 |
| 2 | 59 | 109 |
| 4 | 33 | 57 |
| 8 | 20 | 31 |
| 12 | 21 | 31 |
| 16 | 20 | 30 |
| 20 | 19 | 29 |

End to end with the harness (transform threads default to all cores; 5 runs):

| image | all encoder cores: wall s | decode ms | transform ms | encode ms | peak RSS MiB | one encoder thread: wall s |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| synthetic 1 Mpx JPEG | 0.156 | 4 | 7 | 143 | 92.8 | 1.109 |
| synthetic 12 Mpx JPEG | 0.209 | 45 | 15 | 142 | 141.3 | 1.382 |
| synthetic 24 Mpx JPEG | 0.262 | 88 | 20 | 149 | 166.7 | 1.353 |
| synthetic 48 Mpx JPEG | 0.358 | 173 | 30 | 138 | 251.5 | 1.620 |
| synthetic 12 Mpx AVIF | 0.258 | 96 | 14 | 135 | 165.5 | 1.431 |

Throughput of 100 small images (640×427 to 320 wide), measured by the harness with `examples/batch.rs`:

| mode | images/s |
| --- | ---: |
| one process per image | 13.9 |
| sequential in one process | 15.6 |
| `Engine::transcode_batch`, 20 workers | 216.4–221.6 |

### Where M2 stands on this machine

| 24 Mpx JPEG to 1600-wide AVIF | wall time |
| --- | ---: |
| start of M2 | 8.83 s |
| after package 1 (f32 executor) | 3.20 s |
| after package 2 (encoder, all cores) | 0.44 s |
| after package 3 (kernels) | 0.35 s |
| after package 4 (parallel transform) | 0.26 s |

Decode (88 ms, single-threaded, full frame) and encode (about 150 ms) now make up nearly all of the remaining time; the transform is 20 ms. The next levers are decode-side scaling (work package 5) and the encoder backend, not the transform.

## 2026-11-06 — work package 5, first step: cheaper decode stage

ADR-0023 decision 2: `zune-jpeg` SIMD on, the ICC adapter emits linear-light sRGB, and the orientation gather no longer calls `memmove` per pixel. Measured against `0612ccf` on the same machine (best of five, all cores, speed 6, 1600 wide). The 1-minute load average was 5.3–5.6 because of unrelated processes; both builds ran alternately under the same load.

| image | decode ms before | decode ms after | total ms before | total ms after |
| --- | ---: | ---: | ---: | ---: |
| phone photo 4032×3024, ICC, EXIF orientation 2 (libavif test data) | 150 | 111 | 341 | 296 |
| flower photo 2268×1512 (libjxl test data) | 14 | 12 | 206 | 202 |
| synthetic 24 Mpx | 83 | 68 | 234 | 225 |
| synthetic 48 Mpx | 170 | 133 | 347 | 310 |

The real photograph is the first one in these tables. Its decode stage is more than ten times that of the synthetic 12 Mpx JPEG (150 against 45 ms before this step), because real entropy data is denser and because it carries an ICC profile and an EXIF orientation, both handled serially before the row pipeline.
