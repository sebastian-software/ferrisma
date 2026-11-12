# M2 quality-matched comparison with Sharp

**Status:** work package 0 of [M2](../milestones/M2-performance-architecture.md): the first comparison that matches output quality. It covers Sharp on one machine (Apple M1 Ultra) and four real photographs of at most 12 megapixels. libvips was not measured on its own, and no 24 or 48 megapixel photograph was available. Read every number as "this corpus, this machine". Ferrisma was measured at `0612ccf`; the first decode step of work package 5 (`7651d1c`, decoder SIMD, linear ICC output, cheaper orientation) landed on `main` during this run and is not included, so the dog photo's decode time here is the older one.

## Short answer

At equal SSIMULACRA2 against one common reference:

- **Bytes: Sharp wins.** Sharp's AVIF files are 3.7–12.0 % smaller than Ferrisma's in all 24 matched pairs (geometric means −5.5 % to −10.3 %), and 5.9 % smaller over the 100-image batch set. Sharp encodes with libaom at effort 4 and 8 bits; Ferrisma uses rav1e at speed 6 and 10 bits.
- **Single-image latency: Ferrisma wins on the 1600-wide photographs, and on everything with one thread.** With all cores, Ferrisma is 1.70–1.79× faster by process wall time on the two 1600-wide photos (dog 0.458 s vs 0.780 s) and 1.34–1.40× on the 512-wide Kodak images. Inside the process, without Node's start-up, Sharp is faster on the small Kodak images (61–67 ms vs 97–98 ms). With one thread each, Ferrisma is 1.55–1.99× faster by wall time and 1.52–1.62× in process on every image.
- **Batch throughput: Ferrisma wins.** `Engine::transcode_batch` ran 199 images/s against 100–101 images/s for Sharp's best concurrency setting and 74 images/s for Sharp's defaults, on 100 small images at matched mean quality.
- **Peak memory: Ferrisma is lower everywhere measured.** 1.3–2.9× lower on the 1600-wide photos, about 5–6× lower on the Kodak images (Node plus Sharp alone take 62 MiB), and 181–200 MiB against 252–401 MiB for the batch.
- **Not comparable yet:** 24–48 Mpx photographs (none available), Sharp's shrink-on-load path (disabled by the linear-light pipeline), non-sRGB ICC input (Sharp's linear pipeline breaks on ICC input, see below), libvips without Sharp, and any machine other than this one.

Part of the time advantage may come from rav1e speed 6 doing less work per pixel than aom effort 4, which would also explain aom's smaller files. The preset sweep below measured the bytes at neighboring presets; its timing pass could not run under the machine's load, so an equal-bytes speed comparison is still open.

## What both pipelines do

| | Ferrisma | Sharp |
| --- | --- | --- |
| Version | `0612ccf` (main, 2026-11-06), `--features cli`, release | Sharp 0.34.5 (latest 0.34.x; 0.35.5 exists) with prebuilt libvips 8.17.3, libheif 1.20.2, aom 3.13.1, mozjpeg, lcms 2.17, highway 1.3.0 |
| JPEG decode | zune-jpeg 0.5.15, full frame | mozjpeg (libjpeg-turbo fork), full frame (see shrink-on-load below) |
| Orientation | EXIF applied | `.autoOrient()` |
| Color input | `--assume-srgb` for untagged files; `--use-rendered-icc` (moxcms 0.9.1) for the ICC-tagged photo, because Ferrisma refuses `--assume-srgb` when a profile is present | untagged: sRGB; ICC-tagged photo: `ignoreIcc` (explained below) |
| Resize | Lanczos3, support 3 × scale, linear light (exact sRGB transfer), `f32` | `.resize(w, h, { fit: "fill", kernel: lanczos3 })` with `.pipelineColourspace("scrgb")`, which runs libvips' `reduce` on linear-light `float` scRGB |
| Output | sRGB, AVIF 4:4:4 10-bit (4:2:0 optional), rav1e 0.8.1 speed 6 | `.toColourspace("srgb")`, AVIF 4:4:4 8-bit (4:2:0 optional), libaom effort 4 (Sharp's default) |
| Threads, "all cores" | `--encoder-threads=0 --transform-threads=0` (AV1 tiles from the output size) | Sharp's default `concurrency` (20 here) |
| Threads, "one thread" | `--encoder-threads=1 --transform-threads=1` (one AV1 tile) | `sharp.concurrency(1)` and `UV_THREADPOOL_SIZE=1` |

Commands: `scripts/bench/sharp-avif.cjs` (one image) and `scripts/bench/sharp-batch.cjs` (many images) for Sharp, the `ferrisma` CLI and `examples/batch.rs` for Ferrisma, all driven by `scripts/bench/compare-sharp.py`.

### Gamma handling in Sharp

Sharp resizes gamma-encoded values unless told otherwise. Three options were scored as resize-only lossless PNGs against the linear-light reference (next section):

| image | `pipelineColourspace("scrgb")` | Sharp default (sRGB values) | `.gamma(2.2)` |
| --- | ---: | ---: | ---: |
| dog | 94.26 | 89.33 | 67.74 |
| flower | 95.69 | 95.26 | 86.76 |
| kodim03 | 95.76 | 59.82 | 85.39 |
| kodim23 | 95.93 | 81.89 | 90.43 |

`pipelineColourspace("scrgb")` is the faithful match for Ferrisma's linear-light resize and is used throughout. `.gamma(2.2)` is no substitute: it uses a pure 2.2 power instead of the sRGB curve and scores 5–27 points below `scrgb` on every image. Why it loses even more than the gamma-space default on dog and flower was not investigated. The default gamma-space resize loses up to 36 points on high-contrast detail (kodim03). A comparison against Sharp's defaults would therefore compare different pictures; Sharp's default resize also does less work, so it would not be a fair speed comparison either.

### Shrink-on-load

Sharp normally lets libjpeg decode at 1/2, 1/4 or 1/8 scale. Its source (`src/pipeline.cc`, 0.34.5) skips that whenever a pipeline colourspace is set, gamma is applied, or the image is rotated before the resize. With `scrgb` it is therefore off for every image here, and none of the images would qualify anyway except the dog photo (2.52× after rotation, which also disables it). With libvips' default reducing `gap` of 2.0, `resize` adds an integer box pre-shrink only for reductions of 4× or more; every reduction here is below 2.6×, and the resize-only scores below show no sign of one. So for these images both tools run a plain Lanczos3 on the fully decoded frame. On large photographs Sharp's default path will use shrink-on-load; Ferrisma's equivalent is work package 5. This comparison does not cover that case.

### The ICC-tagged photo

`dog.jpg` carries a 596-byte Google "sRGB IEC61966-2.1" profile and EXIF orientation 6. Ferrisma converts through the profile with `--use-rendered-icc`. With `pipelineColourspace("scrgb")`, Sharp 0.34.5 returned an **all-black image** for this file (mean 0.0 on every channel; resize-only score −637.75). The likely cause, from reading Sharp's `src/pipeline.cc`: the switch to the `float` scRGB pipeline colourspace happens before the embedded-profile import into an 8-bit `srgb` processing profile. With `ignoreIcc: true` or `.keepIccProfile()` the output is correct, and identical in score to converting first and resizing in a second Sharp call (94.255 vs 94.253). Because the profile is sRGB, ignoring it is a no-op for the pixels. The comparison uses `ignoreIcc`, so neither output carries an ICC profile. For a non-sRGB profile (Display P3, Adobe RGB) this workaround would be wrong; a two-step conversion would be needed, and that case is not measured.

### Interoperability note

Sharp 0.34.5's prebuilt binaries cannot decode Ferrisma's 10-bit AVIF ("Bitstream not supported by this decoder"); they also refuse `bitdepth: 10` on output ("Expected 8 for bitdepth when using prebuilt binaries"). ImageMagick 7.1.2-35 and `avif-decode` 3.0.0 read Ferrisma's files. This is a limitation of Sharp's prebuilt aom build, not a defect found in Ferrisma's container.

## Corpus

| name | source | license | input | output |
| --- | --- | --- | --- | --- |
| dog | libavif `tests/data/dog_exif_extended_xmp_icc.jpg` | libavif test data (BSD-2-Clause repository) | 4032×3024 JPEG 4:2:0, EXIF orientation 6, sRGB ICC; 3.1 MiB | 1600×2133 |
| flower | libjxl `tests/data/decode-test/flower.png.im_q85_420.jpg` | CC-BY-4.0 (libjxl test data) | 2268×1512 JPEG q85 4:2:0, no ICC | 1600×1067 |
| kodim03, kodim23 | libavif `tests/data/kodim03_yuv420_8bpc.y4m`, `kodim23_yuv420_8bpc.y4m` | Kodak, released for unrestricted usage | 768×512 8-bit 4:2:0 full range, converted to PNG and then to JPEG q95 4:2:0 with Sharp (mozjpeg); both tools read the same JPEG | 512×341 |
| small set | 100 crops of 640×427 from the four photos (50 dog, 40 flower, 5 + 5 Kodak), deterministic positions, JPEG q90 4:2:0, no metadata | as their sources | 640×427 | 320×213 |

SHA-256 of the inputs: dog `fc67748b1f04aba4ae38476adae70bd9c6157ee0d86db1e93c05aca5815b42e4`, flower `6ad9a79e9f26d2723ef954a0dca774429606655ab7184f8ab400016f228ca2ce`. The images are not committed; `compare-sharp.py prepare` rebuilds the corpus from the sister repositories' test data. **Gap:** the acceptance contract asks for 1, 12, 24 and 48 megapixel photographs. The largest real photograph on this machine is the 12 Mpx dog image, and no image was downloaded for this run. The latency numbers below therefore say nothing about 24–48 Mpx photographs, where decode time and shrink-on-load dominate.

## Quality matching

**Reference.** `scripts/bench/quality-match` (a standalone crate outside the workspace, never shipped) computes one reference per image. It decodes the source with Sharp/mozjpeg (auto-oriented, sRGB, lossless PNG), then resizes it with its own `f64` Lanczos3 in linear light (exact sRGB transfer, support 3 × scale, pixel-center aligned, normalized weights) and keeps unquantized `f32` values. It shares no code with Ferrisma. The filter definition is the textbook one, which both Ferrisma and libvips `reduce` implement.

**Scoring.** Each AVIF is decoded with `avif-decode` 3.0.0 (rav1d) and scored with `ssimulacra2` 0.5.1 against that reference.

**Bisection.** `quality-match bisect` searches each tool's quality knob in 30–95 (Ferrisma continuous, Sharp integer) for the lowest setting whose score reaches the target. Bytes are interpolated linearly in score between the closest settings below and above the target. Time is measured at the bracketing setting nearest the target. Two targets per image:

- **A:** Ferrisma's benchmark setting (quality 80, speed 6, all cores, 4:4:4).
- **B:** Sharp's default (quality 50, effort 4, all cores, 4:4:4).

Every configuration (both tools, all cores and one thread, 4:4:4 and 4:2:0) is bisected separately. Tiles, and so bytes, depend on the thread setting in both tools: Sharp's aom output changes with `concurrency`, and Ferrisma uses one tile with one thread.

### Ceilings and sensitivity

What each pipeline reaches without lossy encoding, against the main reference and against a second reference built from Ferrisma's own decoder (zune-jpeg):

| image | zune reference vs main reference | Sharp resize-only PNG | Sharp AVIF q100 | Ferrisma AVIF q100 | Ferrisma q100 vs zune reference |
| --- | ---: | ---: | ---: | ---: | ---: |
| dog | 94.41 | 94.26 | 92.49 | 93.82 | 94.20 |
| flower | 95.19 | 95.69 | 94.07 | 94.70 | 95.54 |
| kodim03 | 95.67 | 95.76 | 93.44 | 95.07 | 96.15 |
| kodim23 | 95.78 | 95.93 | 94.15 | 95.19 | 96.30 |

1. **Both tools resize alike.** Sharp's linear-light resize scores 94–96 against the reference, the same as two decoders of the same JPEG score against each other. Decoder and rounding differences set this ceiling; the resize itself costs nothing measurable.
2. **The targets (65–76) sit far below the ceilings (92–96).** The choice of reference decoder moves the matched-target scores by less than 0.1 point (dog and flower anchors scored against both references: 73.53/73.54, 76.46/76.48, 65.45/65.41, 70.58/70.53).
3. **Scoring decoder.** ImageMagick 7.1.2 (libheif) scores Sharp's 8-bit files 0.25–0.33 points higher than `avif-decode` does, and Ferrisma's 10-bit files 0.03 higher. `avif-decode` rounds 8-bit YUV to 8-bit RGB, as browsers do. Its choice slightly favors Ferrisma, by roughly 1–2 % bytes at these slopes. The byte gap below would grow, not shrink, with the other decoder.

### Bytes at equal quality

**Target A** (Ferrisma quality 80, all cores, 4:4:4), bytes at the target score:

| image | target SSIMULACRA2 | Ferrisma quality | Ferrisma bytes | Sharp quality | Sharp bytes | Sharp vs Ferrisma |
| --- | ---: | --- | ---: | --- | ---: | ---: |
| dog | 73.61 | 80 | 261,847 | 57–58 | 242,950 | -7.2 % |
| flower | 76.48 | 80 | 126,582 | 57–58 | 115,932 | -8.4 % |
| kodim03 | 73.71 | 80 | 14,467 | 57–58 | 13,593 | -6.0 % |
| kodim23 | 76.32 | 80 | 14,335 | 57–58 | 13,569 | -5.3 % |

**Target B** (Sharp quality 50, all cores, 4:4:4), bytes at the target score:

| image | target SSIMULACRA2 | Ferrisma quality | Ferrisma bytes | Sharp quality | Sharp bytes | Sharp vs Ferrisma |
| --- | ---: | --- | ---: | --- | ---: | ---: |
| dog | 65.45 | 66.37–66.56 | 195,807 | 50 | 172,659 | -11.8 % |
| flower | 70.58 | 65.58–66.05 | 97,521 | 50 | 85,822 | -12.0 % |
| kodim03 | 66.52 | 66.05–66.56 | 10,931 | 50 | 9,858 | -9.8 % |
| kodim23 | 68.61 | 65.58–66.05 | 10,845 | 50 | 10,097 | -6.9 % |

Sharp's bytes relative to Ferrisma's at equal score, for every matched configuration (each tool bisected separately per configuration):

| target | configuration | dog | flower | kodim03 | kodim23 | geometric mean |
| --- | --- | ---: | ---: | ---: | ---: | ---: |
| A | all cores, 4:4:4 | -7.2 % | -8.4 % | -6.0 % | -5.3 % | -6.8 % |
| A | one thread, 4:4:4 | -5.6 % | -7.5 % | -8.9 % | -10.0 % | -8.0 % |
| A | all cores, 4:2:0 | -3.8 % | -9.3 % | -3.7 % | -5.2 % | -5.5 % |
| B | all cores, 4:4:4 | -11.8 % | -12.0 % | -9.8 % | -6.9 % | -10.2 % |
| B | one thread, 4:4:4 | -9.1 % | -10.4 % | -10.2 % | -11.5 % | -10.3 % |
| B | all cores, 4:2:0 | -7.4 % | -10.8 % | -5.5 % | -6.7 % | -7.6 % |

Within each tool, at target A, bytes relative to the same tool's all-cores 4:4:4 result:

| image | Ferrisma one thread | Ferrisma 4:2:0 | Sharp one thread | Sharp 4:2:0 |
| --- | ---: | ---: | ---: | ---: |
| dog | -3.5 % | +1.1 % | -1.8 % | +4.8 % |
| flower | -4.0 % | -0.8 % | -3.0 % | -1.8 % |
| kodim03 | -1.4 % | +0.4 % | -4.4 % | +2.9 % |
| kodim23 | +1.5 % | -0.3 % | -3.5 % | -0.2 % |

What the byte numbers say:

1. **libaom at effort 4 beats rav1e at speed 6 by 3.7–12.0 % bytes at equal SSIMULACRA2** on every image, target, thread setting and chroma layout. The gap is larger at the lower target B (−10 %) than at A (−7 %). This is an encoder difference: the resize ceilings are equal.
2. **Parallel encoding costs both tools up to 4–5 % bytes.** Ferrisma's size tiles cost up to 4 % against its one-thread single tile, and Sharp's threaded aom output 2–4 % against one thread. kodim23's +1.5 % for Ferrisma's single tile is within the bisection noise: scores are not strictly monotonic in the quality knob, so a bracket can be off by about 1.5 % in bytes.
3. **4:2:0 does not save bytes here for either tool** (Ferrisma −0.8 % to +1.1 %, Sharp −1.8 % to +4.8 %). This differs from the encoder experiment, where rav1e 4:2:0 saved 0.5–12 % at equal score. That experiment scored against an sRGB-domain area downscale at one thread; this one scores against a linear-light Lanczos3 reference with size tiles. Keeping 4:4:4 as the default is consistent with both.

## Single-image latency at matched quality

**Machine and conditions.** Apple M1 Ultra (16 performance and 4 efficiency cores, 20 logical), 64 GiB, macOS 27.0.1 (Darwin 27.0.0, arm64), Node 24.22.2, rustc 1.100.0-nightly (2026-09-06), hyperfine 2.0.0. The machine is shared: other agents' jobs pushed the load average to about 60 earlier on the day of the run, so no timing from that period is used. Before every measurement the driver waited until the 1-minute load average was below 4. The load before and after each measurement is in the tables. Two background services (`fseventsd` and a backup client) kept roughly two cores busy throughout, so the idle floor was a load of about 3–4.

**Method.** Each tool runs at the setting nearest target A from the bisection (Ferrisma's quality 80 anchor, or the bracketing Sharp setting), so its score is within about 0.7 point of the target. For each setting:

- "wall s" is the median of 10 `hyperfine` runs after 1 warmup, one process per run.
- "CPU/wall" is the mean CPU time divided by the mean wall time.
- "in-process ms" is Ferrisma's `report: total_us` median over 10 runs, or Sharp's own timer around 10 back-to-back pipelines in one Node process with Sharp's cache disabled. It excludes Node and module start-up (about 75 ms for Sharp) and, for Ferrisma, process start plus file reading and writing (4–11 ms).
- "peak RSS" comes from one more run under `/usr/bin/time -l`; for Sharp it includes Node and the Sharp module (62 MiB before any image work).

**All cores, 4:4:4**

| image | tool | quality | SSIMULACRA2 | bytes | load before/after | wall s (median of 10) | ± s | CPU/wall | in-process ms | peak RSS MiB |
| --- | --- | ---: | ---: | ---: | --- | ---: | ---: | ---: | ---: | ---: |
| dog | Ferrisma | 79.77 | 73.61 | 261,847 | 2.99/5.39 | 0.458 | 0.005 | 8.19 | 447 | 237 |
| dog | Sharp | 57 | 72.95 | 235,857 | 3.82/5.74 | 0.780 | 0.013 | 7.34 | 679 | 443 |
| flower | Ferrisma | 79.77 | 76.48 | 126,582 | 3.39/4.88 | 0.221 | 0.008 | 8.67 | 217 | 107 |
| flower | Sharp | 57 | 76.29 | 114,772 | 3.81/5.09 | 0.395 | 0.005 | 7.38 | 314 | 240 |
| kodim03 | Ferrisma | 79.77 | 73.71 | 14,467 | 3.88/3.88 | 0.102 | 0.001 | 2.16 | 98 | 19 |
| kodim03 | Sharp | 57 | 73.08 | 13,234 | 3.88/4.37 | 0.143 | 0.001 | 2.73 | 67 | 99 |
| kodim23 | Ferrisma | 81.27 | 76.10 | 14,573 | 3.72/3.82 | 0.102 | 0.001 | 2.18 | 97 | 20 |
| kodim23 | Sharp | 57 | 75.83 | 13,237 | 3.82/3.82 | 0.136 | 0.001 | 2.88 | 61 | 101 |

| image | Sharp/Ferrisma wall | Sharp/Ferrisma in-process | Sharp/Ferrisma peak RSS |
| --- | ---: | ---: | ---: |
| dog | 1.70× | 1.52× | 1.87× |
| flower | 1.79× | 1.45× | 2.24× |
| kodim03 | 1.40× | 0.69× | 5.19× |
| kodim23 | 1.34× | 0.63× | 5.16× |

**One thread, 4:4:4**

| image | tool | quality | SSIMULACRA2 | bytes | load before/after | wall s (median of 10) | ± s | CPU/wall | in-process ms | peak RSS MiB |
| --- | --- | ---: | ---: | ---: | --- | ---: | ---: | ---: | ---: | ---: |
| dog | Ferrisma | 80.48 | 73.53 | 251,705 | 3.79/3.90 | 3.316 | 0.011 | 0.99 | 3310 | 233 |
| dog | Sharp | 57 | 73.18 | 233,567 | 3.90/3.56 | 5.130 | 0.014 | 1.00 | 5028 | 312 |
| flower | Ferrisma | 79.77 | 76.46 | 121,359 | 3.85/3.75 | 1.611 | 0.012 | 0.99 | 1606 | 97 |
| flower | Sharp | 57 | 76.42 | 111,920 | 3.75/3.11 | 2.685 | 0.004 | 1.00 | 2603 | 172 |
| kodim03 | Ferrisma | 79.77 | 73.88 | 14,284 | 3.44/3.64 | 0.182 | 0.001 | 0.99 | 179 | 16 |
| kodim03 | Sharp | 57 | 73.62 | 12,914 | 3.64/3.70 | 0.351 | 0.002 | 1.01 | 277 | 82 |
| kodim23 | Ferrisma | 81.80 | 76.42 | 14,572 | 3.82/3.67 | 0.182 | 0.001 | 0.99 | 178 | 17 |
| kodim23 | Sharp | 57 | 76.05 | 12,852 | 3.67/3.72 | 0.363 | 0.002 | 1.01 | 288 | 82 |

| image | Sharp/Ferrisma wall | Sharp/Ferrisma in-process | Sharp/Ferrisma peak RSS |
| --- | ---: | ---: | ---: |
| dog | 1.55× | 1.52× | 1.33× |
| flower | 1.67× | 1.62× | 1.77× |
| kodim03 | 1.92× | 1.55× | 5.06× |
| kodim23 | 1.99× | 1.62× | 4.80× |

**All cores, 4:2:0**

| image | tool | quality | SSIMULACRA2 | bytes | load before/after | wall s (median of 10) | ± s | CPU/wall | in-process ms | peak RSS MiB |
| --- | --- | ---: | ---: | ---: | --- | ---: | ---: | ---: | ---: | ---: |
| dog | Ferrisma | 80.78 | 73.61 | 264,773 | 3.56/5.07 | 0.397 | 0.005 | 7.56 | 397 | 185 |
| dog | Sharp | 61 | 73.16 | 249,029 | 3.34/4.64 | 0.698 | 0.005 | 6.69 | 603 | 391 |
| flower | Ferrisma | 82.27 | 76.29 | 124,480 | 3.11/4.86 | 0.170 | 0.007 | 8.46 | 159 | 74 |
| flower | Sharp | 62 | 77.01 | 117,544 | 3.88/5.30 | 0.359 | 0.004 | 7.20 | 276 | 213 |
| kodim03 | Ferrisma | 82.11 | 73.60 | 14,424 | 3.70/3.70 | 0.079 | 0.001 | 2.27 | 75 | 15 |
| kodim03 | Sharp | 61 | 73.40 | 13,859 | 3.70/4.60 | 0.132 | 0.002 | 2.61 | 57 | 91 |
| kodim23 | Ferrisma | 82.30 | 76.33 | 14,288 | 3.72/3.72 | 0.078 | 0.001 | 2.31 | 74 | 15 |
| kodim23 | Sharp | 62 | 76.53 | 13,658 | 3.72/3.72 | 0.130 | 0.002 | 2.76 | 54 | 96 |

| image | Sharp/Ferrisma wall | Sharp/Ferrisma in-process | Sharp/Ferrisma peak RSS |
| --- | ---: | ---: | ---: |
| dog | 1.76× | 1.52× | 2.11× |
| flower | 2.11× | 1.73× | 2.86× |
| kodim03 | 1.67× | 0.75× | 6.18× |
| kodim23 | 1.67× | 0.73× | 6.35× |

Ferrisma's own stage report at the same settings (median of 10 runs, ms; `compare-sharp.py stages`):

| image | configuration | load before/after | open | decode | transform | encode | total |
| --- | --- | --- | ---: | ---: | ---: | ---: | ---: |
| dog | all cores, 4:4:4 | 3.65/3.65 | 3.8 | 154.9 | 28.1 | 271.2 | 461.2 |
| dog | one thread, 4:4:4 | 3.65/4.54 | 3.4 | 153.2 | 489.9 | 2701.7 | 3355.1 |
| dog | all cores, 4:2:0 | 3.59/4.03 | 2.6 | 154.6 | 39.9 | 204.7 | 404.2 |

The same breakdown for the other images was not measured: after the dog rows, the load average stayed between 4.3 and 10 for about 50 minutes (a backup, file indexing and a call service), and the run was stopped instead of measuring under load.

What the latency numbers say:

1. **With all cores, Ferrisma is faster on the 1600-wide photos** (1.70–1.79× wall, 1.45–1.52× in process). On the 512-wide Kodak images Ferrisma only wins because Sharp pays Node's start-up. In process Sharp needs 61–67 ms and Ferrisma 97–98 ms. Ferrisma's encoder barely parallelizes a 512×341 frame (CPU/wall 2.2; the frame holds only a few 256×256 tiles), so all cores speed Ferrisma up only 1.8× there (179 to 98 ms in process), while Sharp gains 4.1–4.7× (277–288 to 61–67 ms).
2. **With one thread each, Ferrisma is 1.5–2.0× faster** on every image. Per core, rav1e speed 6 plus Ferrisma's resize costs less than aom effort 4 plus libvips at the same score, but produces 6–11 % more bytes.
3. **The dog photo shows Ferrisma's remaining costs on real input.** Decode (zune-jpeg, plus rotation and ICC conversion into a full oriented frame) takes 155 ms and encode 271 ms of 461 ms, against 28 ms for the transform. The encode is 1.8× the synthetic baseline's 149 ms; the 1600×2133 output has twice the baseline's 1600×1067 area.
4. **Peak RSS** is 1.3–2.9× lower for Ferrisma on the 1600-wide photos. On the small images the comparison mostly reflects Node's 62 MiB floor.

## Batch throughput at matched quality

100 small JPEGs (640×427 crops of the four photographs) to 320×213 AVIF, 4:4:4. Ferrisma runs `examples/batch.rs` (`Engine::transcode_batch` with 20 workers at quality 80, one thread per image, then a sequential pass in the same process). Sharp runs `scripts/bench/sharp-batch.cjs`, which transcodes buffer to buffer with N images in flight (it also sets `UV_THREADPOOL_SIZE` to N) and `concurrency` threads per image. Both read all inputs into memory first and time only the transcoding. Sharp's quality was bisected on the **mean** SSIMULACRA2 over the set to Ferrisma's mean at quality 80 (77.43). Quality 56 gives 76.96 and quality 57 gives 77.96, so the throughput runs use quality 57, a slightly higher score than Ferrisma's.

| configuration | images/s (3 runs) | peak RSS MiB | output bytes | mean SSIMULACRA2 |
| --- | --- | ---: | ---: | ---: |
| Ferrisma `Engine`, 20 workers × 1 thread | 197.0, 200.3, 199.4 | 181–200 | 716,939 | 77.43 |
| Ferrisma sequential, one process | 14.1, 14.1, 14.0 | (same process) | 716,939 | 77.43 |
| Sharp defaults: 4 in flight × 20 threads | 73.8, 73.8, 74.7 | 259–286 | 706,626 | 77.83 |
| Sharp 20 in flight × 1 thread | 105.3, 100.0, 98.8 | 361–401 | 693,341 | 77.96 |
| Sharp 10 in flight × 2 threads | 103.5, 100.6, 101.2 | 252–277 | 699,019 | — |
| Sharp sequential, 1 thread | 7.4, 7.5, 7.4 | 90–96 | 693,341 | 77.96 |

Interpolated at Ferrisma's mean score, Sharp's set would be 674,971 bytes (−5.9 %). Load averages before the runs were 3.2–4.0 (`batch.txt` in the output directory lists each run).

What the batch numbers say: the `Engine` delivers about 2.0× the images per second of Sharp's best setting and 2.7× Sharp's defaults, with lower peak memory, at 6 % more bytes. Sequentially in one process Ferrisma is 1.9× faster (14.1 vs 7.4 images/s). Sharp's default of 4 images in flight leaves cores idle on small images; a Sharp user would raise `UV_THREADPOOL_SIZE` as done here.

## Encoder presets: the bytes–time trade-off

Part of Ferrisma's time advantage and Sharp's byte advantage may come from the encoders' default presets rather than from the pipelines. To see how far each tool can move, the flower and kodim23 images were also matched to target A at other presets (all cores, 4:4:4), with the same bisection (`compare-sharp.py sweepmatch`):

| image | tool and preset | bytes at target A | vs Ferrisma speed 6 |
| --- | --- | ---: | ---: |
| flower | Ferrisma speed 4 | 121,352 | -4.1 % |
| flower | Ferrisma speed 6 (default) | 126,582 | +0.0 % |
| flower | Ferrisma speed 8 | 131,368 | +3.8 % |
| flower | Ferrisma speed 10 | 144,252 | +14.0 % |
| flower | Sharp effort 0 | 162,644 | +28.5 % |
| flower | Sharp effort 2 | 133,784 | +5.7 % |
| flower | Sharp effort 4 (default) | 115,932 | -8.4 % |
| flower | Sharp effort 6 | 112,382 | -11.2 % |
| kodim23 | Ferrisma speed 4 | 13,811 | -3.7 % |
| kodim23 | Ferrisma speed 6 (default) | 14,335 | +0.0 % |
| kodim23 | Ferrisma speed 8 | 14,910 | +4.0 % |
| kodim23 | Ferrisma speed 10 | 17,542 | +22.4 % |
| kodim23 | Sharp effort 0 | 18,104 | +26.3 % |
| kodim23 | Sharp effort 2 | 16,207 | +13.1 % |
| kodim23 | Sharp effort 4 (default) | 13,569 | -5.3 % |
| kodim23 | Sharp effort 6 | 13,252 | -7.6 % |

The timing half of this sweep (`compare-sharp.py sweep`) did not run, for the same load reason as the stage table, so the table says nothing about speed. What the bytes alone show:

1. **Neither tool's neighboring presets close the gap.** rav1e speed 4 saves 3.7–4.1 % against speed 6, still above Sharp's default. Sharp's effort 6 saves another 2.3–3.1 %.
2. **Sharp's fast presets are worse than Ferrisma's default.** At effort 2, Sharp needs 5.7 % (flower) and 13.1 % (kodim23) more bytes than Ferrisma at speed 6; at effort 0, 26–29 % more.
3. **An equal-time comparison** (Sharp at a faster effort against Ferrisma at its default, or Ferrisma at speed 4 against Sharp's default) needs the timing pass. It is the most important open measurement for deciding whether Ferrisma's latency lead survives at equal bytes.

## Reproduce

```sh
npm install --prefix /tmp/sharp-0.34.5 --save-exact sharp@0.34.5
export BENCH_SHARP_NODE_MODULES=/tmp/sharp-0.34.5/node_modules
export COMPARE_LIBAVIF_DATA=PATH/libavif/tests/data
export COMPARE_LIBJXL_DATA=PATH/libjxl/tests/data/decode-test
python3 -I scripts/bench/compare-sharp.py all target/bench/sharp
```

`prepare` builds the CLI, `examples/batch.rs` and `scripts/bench/quality-match`, and writes the corpus and references. `ceiling`, `match` and `sweepmatch` (bytes and scores only, 10–15 minutes for `match`) and `time`, `batch`, `stages` and `sweep` (load-gated by `COMPARE_MAX_LOAD`, default 4) can also run one by one. Results land in the output directory as `ceiling.tsv`, `anchors.tsv`, `matched.tsv`, `time.csv`, `batch.txt`, `stages.tsv`, `sweep-matched.tsv` and `sweep.csv` next to the encoded files and hyperfine JSON.

## Follow-ups

- Real 24 and 48 Mpx photographs, where Sharp's shrink-on-load (in its default gamma-space mode) and Ferrisma's full-frame decode matter most; work package 5 is Ferrisma's answer to the first.
- A libvips CLI run (`vips`/`vipsthumbnail` were not installed, and no system packages were installed for this run).
- An encoder with a better bytes–time curve than rav1e: the libavif-rs encoders of work package 2, or aom through libavif-rs. Measured on 2026-11-06 in [M2-libavif-backend.md](M2-libavif-backend.md): libavif-rs's AOM at speed 6 is at or below Sharp's bytes on all four photographs.
- The same comparison on Linux arm64 and x86-64 (acceptance item 3).
- Sharp's behavior with non-sRGB ICC input in a linear-light pipeline.
- Butteraugli as the second quality opinion that acceptance item 2 asks for; only SSIMULACRA2 was used here.
- The timing passes that did not run under load: `compare-sharp.py stages` for flower and Kodak, and `compare-sharp.py sweep`.
