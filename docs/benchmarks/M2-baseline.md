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

Settings are environment variables: `BENCH_WIDTH` (default 1600, aspect preserved), `BENCH_SMALL_WIDTH` (320), `BENCH_QUALITY` (80), `BENCH_SPEED` (6), `BENCH_FILTER` (`lanczos` or `area`), `BENCH_PROFILE` (`assume-srgb`, `strict`, `rendered-icc`), `BENCH_PRECISION` (`f32`, or `reference` for the M1 `f64` executor), `BENCH_MAX_INPUT_MB` (512). The harness writes `results.csv`, `throughput.csv`, `summary.md`, `environment.txt` and the raw hyperfine JSON into the output directory.

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
