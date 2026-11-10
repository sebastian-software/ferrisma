# M2 encoder comparison: `rav1e` and libavif-rs at matched quality

**Status:** measurements for [ADR-0024](../adr/0024-pluggable-encoders-and-libavif-rs-backend.md) on one machine (Apple M1 Ultra, 20 logical cores, macOS 27.0.1), 2026-11-06. Ferrixel at the ADR-0024 commit, libavif-rs `f3b2981` (Rust AOM, no SIMD). Same corpus, references and target A as the [Sharp comparison](M2-sharp-comparison.md); read every number as "these four photographs, this machine".

**Current context:** The adapter and codec now share the Ferrixel workspace ([ADR-0026](../adr/0026-codecs-in-one-repository-and-workspace.md)); `rav1e` remains the default. The tables below describe the specific historical revisions and hosts, not current performance guarantees.

**Update 2026-11-07:** libavif-rs `c9b583c` adds Neon kernels to its AOM ([retained Neon commit](https://github.com/sebastian-software/ferrixel/commit/c9b583ccd12aabcfe2bb1ad5ad2f8b3165ad9bc0)). The re-run is in [After Neon](#after-neon-libavif-rs-c9b583c); bytes are unchanged, time drops by a third to a half.

**Update 2026-11-09:** libavif-rs `3df161d` adds SSE2, SSE4.1 and AVX2 kernels on x86-64 GNU Linux ([retained x86 SIMD commit](https://github.com/sebastian-software/ferrixel/commit/3df161d9715ac743a1c7305f9a37ffaa3ae51c05)). The first x86-64 run is in [x86-64 Linux with AVX2](#x86-64-linux-with-avx2-libavif-rs-3df161d): libavif-rs speed 6 is 41–47 % faster than `rav1e` with all cores and 5–13 % smaller.

## Method

- **Pipelines.** Both runs are Ferrixel end to end (decode, orientation, ICC, linear-light Lanczos3, sRGB8): `ferrixel` encodes with the built-in `rav1e` sink, `ferrixel-libavif` hands the same rows to libavif-rs. 4:4:4, 10 bits, CICP 1/13/6 for both.
- **Quality matching.** Target A is the SSIMULACRA2 score of `rav1e` at quality 80, speed 6, all cores. libavif-rs's integer quality is bisected to that score at speeds 6 and 7, all cores; bytes are interpolated between the bracketing settings (`scripts/bench/quality-match`). `rav1e` bytes are those of its quality-80 anchor.
- **Timing.** Best of five runs of Ferrixel's own stage report, all cores (`--encoder-threads=0 --transform-threads=0`) and one thread, at the matched settings. The run waited for a 1-minute load average below 4 and the load during each image is recorded; two background services kept it between 3.9 and 5.9. One-thread libavif-rs rows use the all-cores matched quality, so their bytes differ slightly from the matched value.
- **Reproduce:** `scripts/bench/compare-encoders.sh OUT_DIR` after `compare-sharp.py prepare` and `match` have written `OUT_DIR`.

## Bytes at equal SSIMULACRA2 (target A)

| image | target | `rav1e` speed 6 | Sharp (libaom, effort 4) | libavif-rs speed 6 | libavif-rs speed 7 |
| --- | ---: | ---: | ---: | ---: | ---: |
| dog 1600×2133 (ICC, EXIF 6) | 73.61 | 261,847 | 242,950 | 236,050 (−9.9 %) | 248,455 (−5.1 %) |
| flower 1600×1067 | 76.48 | 126,582 | 115,932 | 110,156 (−13.0 %) | 117,493 (−7.2 %) |
| kodim03 512×341 | 73.71 | 14,467 | 13,593 | 12,907 (−10.8 %) | 13,519 (−6.6 %) |
| kodim23 512×341 | 76.32 | 14,335 | 13,569 | 13,569 (−5.3 %) | 14,693 (+2.5 %) |

Percentages are against `rav1e`. Sharp's numbers come from the Sharp comparison (its own resize, libaom 3.13 with SIMD, 8 bits); libavif-rs at speed 6 is 2.8–5.0 % below Sharp on three images and equal on kodim23.

## Time at the matched settings

| image | encoder | threads | quality | decode ms | transform ms | encode ms | total ms | bytes | load |
| --- | --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| dog | `rav1e` speed 6 | all | 79.77 | 65 | 20 | 264 | 357 | 261,712 | 4.29 |
| dog | libavif-rs speed 6 | all | 56 | 65 | 17 | 314 | 401 | 240,705 | 4.29 |
| dog | libavif-rs speed 7 | all | 56 | 66 | 18 | 178 | 267 | 252,286 | 4.29 |
| dog | `rav1e` speed 6 | 1 | 80.48 | 76 | 121 | 2,676 | 2,881 | 252,391 | 4.57 |
| dog | libavif-rs speed 6 | 1 | 56 | 77 | 120 | 3,713 | 3,914 | 239,488 | 5.58 |
| flower | `rav1e` speed 6 | all | 79.77 | 12 | 8 | 181 | 206 | 126,582 | 5.58 |
| flower | libavif-rs speed 6 | all | 55 | 12 | 7 | 249 | 271 | 112,437 | 5.86 |
| flower | libavif-rs speed 7 | all | 55 | 12 | 8 | 124 | 147 | 120,137 | 5.86 |
| flower | `rav1e` speed 6 | 1 | 79.77 | 12 | 30 | 1,588 | 1,635 | 121,359 | 5.71 |
| flower | libavif-rs speed 6 | 1 | 55 | 12 | 29 | 2,404 | 2,448 | 111,885 | 5.70 |
| kodim03 | `rav1e` speed 6 | all | 79.77 | 1 | 2 | 90 | 97 | 14,467 | 5.70 |
| kodim03 | libavif-rs speed 6 | all | 55 | 2 | 2 | 104 | 110 | 12,545 | 5.70 |
| kodim03 | libavif-rs speed 7 | all | 56 | 2 | 2 | 49 | 55 | 13,700 | 5.70 |
| kodim03 | `rav1e` speed 6 | 1 | 79.77 | 1 | 3 | 173 | 180 | 14,284 | 5.70 |
| kodim03 | libavif-rs speed 6 | 1 | 55 | 1 | 3 | 212 | 218 | 12,626 | 5.65 |
| kodim23 | `rav1e` speed 6 | all | 81.27 | 1 | 2 | 91 | 97 | 14,573 | 5.65 |
| kodim23 | libavif-rs speed 6 | all | 58 | 2 | 2 | 99 | 105 | 13,783 | 5.65 |
| kodim23 | libavif-rs speed 7 | all | 58 | 1 | 2 | 50 | 56 | 14,614 | 5.65 |
| kodim23 | `rav1e` speed 6 | 1 | 81.80 | 1 | 3 | 172 | 179 | 14,572 | 5.65 |
| kodim23 | libavif-rs speed 6 | 1 | 58 | 1 | 3 | 216 | 223 | 13,806 | 5.65 |

The quality column is each run's setting nearest the target from its bisection (for `rav1e` the Sharp comparison's, which sits within 0.7 points of the anchor).

## After Neon (libavif-rs `c9b583c`)

Re-run on 2026-11-07, same machine, corpus, references and method, with libavif-rs at `c9b583c` (242 hand-translated Neon kernels, dispatched at run time; the encoder writes the same bytes as without them). The bisection reproduced every matched setting and byte count above exactly. The 1-minute load was between 3.3 and 4.5.

| image | encoder | threads | quality | decode ms | transform ms | encode ms | total ms | bytes | before Neon (total ms) |
| --- | --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| dog | `rav1e` speed 6 | all | 80 | 63 | 19 | 265 | 355 | 261,712 | |
| dog | libavif-rs speed 6 | all | 56 | 63 | 18 | 181 | 267 | 240,705 | 401 |
| dog | libavif-rs speed 7 | all | 56 | 65 | 18 | 119 | 207 | 252,286 | 267 |
| dog | `rav1e` speed 6 | 1 | 80 | 74 | 120 | 2,632 | 2,832 | 252,391 | |
| dog | libavif-rs speed 6 | 1 | 56 | 75 | 119 | 1,831 | 2,029 | 239,488 | 3,914 |
| flower | `rav1e` speed 6 | all | 80 | 12 | 8 | 176 | 201 | 126,582 | |
| flower | libavif-rs speed 6 | all | 55 | 12 | 8 | 136 | 159 | 112,437 | 271 |
| flower | libavif-rs speed 7 | all | 55 | 12 | 7 | 77 | 99 | 120,137 | 147 |
| flower | `rav1e` speed 6 | 1 | 80 | 11 | 29 | 1,558 | 1,602 | 121,359 | |
| flower | libavif-rs speed 6 | 1 | 55 | 12 | 28 | 1,136 | 1,179 | 111,885 | 2,448 |
| kodim03 | `rav1e` speed 6 | all | 80 | 1 | 2 | 91 | 97 | 14,467 | |
| kodim03 | libavif-rs speed 6 | all | 55 | 1 | 2 | 57 | 63 | 12,545 | 110 |
| kodim03 | libavif-rs speed 7 | all | 56 | 2 | 2 | 29 | 35 | 13,700 | 55 |
| kodim03 | `rav1e` speed 6 | 1 | 80 | 1 | 3 | 171 | 177 | 14,284 | |
| kodim03 | libavif-rs speed 6 | 1 | 55 | 1 | 3 | 106 | 112 | 12,626 | 218 |
| kodim23 | `rav1e` speed 6 | all | 80 | 2 | 2 | 91 | 97 | 14,335 | |
| kodim23 | libavif-rs speed 6 | all | 58 | 2 | 2 | 52 | 58 | 13,783 | 105 |
| kodim23 | libavif-rs speed 7 | all | 58 | 2 | 2 | 28 | 34 | 14,614 | 56 |
| kodim23 | `rav1e` speed 6 | 1 | 80 | 1 | 3 | 166 | 173 | 14,102 | |
| kodim23 | libavif-rs speed 6 | 1 | 58 | 1 | 3 | 104 | 110 | 13,806 | 223 |

`rav1e` is unchanged; its rows only re-check the load.

1. **Neon cuts libavif-rs's end-to-end time by 33–45 % with all cores at speed 6** (dog 401 to 267 ms, flower 271 to 159 ms) **and by 48–52 % with one thread** (dog 3.9 to 2.0 s). The encode stage alone drops 42–47 % with all cores.
2. **libavif-rs speed 6 is now both smaller and faster than `rav1e` on all four photographs**: 5–13 % fewer bytes, 21–40 % less total time with all cores and 26–37 % less with one thread. Speed 7 is 42–65 % faster than `rav1e` with all cores.
3. **Against Sharp** (its in-process times from the Sharp comparison, a different run): dog 679 against 267 ms, flower 314 against 159 ms, Kodak 61–67 against 58–63 ms, with Sharp's files 3–5 % larger on three images.
4. **Only macOS ARM64 has Neon.** x86-64 Linux ran the scalar port at this revision; see [x86-64 Linux with AVX2](#x86-64-linux-with-avx2-libavif-rs-3df161d) for the result there.
5. **Bug fixed in the harness.** `compare-encoders.sh time` read the matched score instead of the matched quality from `matched.tsv` and timed libavif-rs at the wrong setting; it now reads the `q_nearest` column. The 2026-11-06 table above was produced with the right settings, as its byte counts show.

## x86-64 Linux with AVX2 (libavif-rs `3df161d`)

Run on 2026-11-09 on a GitHub-hosted `ubuntu-24.04` runner (AMD EPYC 7763, 4 vCPUs, AVX2 without AVX-512) by the manual workflow `m2-encoders-linux.yml` ([run 42586794419](https://github.com/sebastian-software/ferrixel/actions/runs/42586794419)), Ferrixel `a2dc650` with the adapter pinned to libavif-rs `3df161d`. Same corpus and method, with two differences: target A comes from `compare-encoders.sh anchor` (only `rav1e` at quality 80, not the full Sharp comparison), and the timing runs twice, with libavif-rs's run-time SIMD dispatch (AVX2 on this CPU) and with `AOM_SIMD_CAPS_MASK=0`, which selects its scalar AOM. `rav1e` uses its x86 assembly. The 1-minute load, which on four vCPUs includes the measurement's own all-cores runs, was between 1.8 and 3.8.

Target A, the matched qualities and the libavif-rs bytes reproduce the macOS run: the same target scores to the two decimals recorded there, the same qualities, and bytes within 0.3 %.

| image | target | `rav1e` speed 6 | libavif-rs speed 6 | libavif-rs speed 7 |
| --- | ---: | ---: | ---: | ---: |
| dog | 73.608 | 261,905 | 235,393 (−10.1 %) | 249,323 (−4.8 %) |
| flower | 76.481 | 126,573 | 110,377 (−12.8 %) | 117,488 (−7.2 %) |
| kodim03 | 73.710 | 14,467 | 12,907 (−10.8 %) | 13,519 (−6.6 %) |
| kodim23 | 76.321 | 14,335 | 13,569 (−5.3 %) | 14,693 (+2.5 %) |

Total ms, best of five, at the matched settings:

| image | encoder | threads | AVX2 | scalar AOM | `rav1e` speed 6 |
| --- | --- | ---: | ---: | ---: | ---: |
| dog | libavif-rs speed 6 | all | 1,842 | 3,328 | 3,331 |
| dog | libavif-rs speed 7 | all | 1,242 | 1,778 | |
| dog | libavif-rs speed 6 | 1 | 3,697 | 7,198 | 6,808 |
| flower | libavif-rs speed 6 | all | 1,051 | 2,054 | 1,782 |
| flower | libavif-rs speed 7 | all | 618 | 964 | |
| flower | libavif-rs speed 6 | 1 | 2,130 | 4,472 | 3,723 |
| kodim03 | libavif-rs speed 6 | all | 127 | 231 | 230 |
| kodim03 | libavif-rs speed 7 | all | 82 | 113 | |
| kodim03 | libavif-rs speed 6 | 1 | 205 | 400 | 400 |
| kodim23 | libavif-rs speed 6 | all | 120 | 241 | 227 |
| kodim23 | libavif-rs speed 7 | all | 85 | 121 | |
| kodim23 | libavif-rs speed 6 | 1 | 200 | 410 | 400 |

The `rav1e` column is from the AVX2 pass; its second pass differed by at most 8 ms. Decode takes the same time for both encoders; the transform stage is shorter for libavif-rs (dog 125 against 180 ms with all cores), and the rest of the difference is encode time. Full stage tables are in the run log and its `m2-encoders-linux` artifact.

1. **AVX2 halves libavif-rs's time on x86-64**: 45–50 % less end to end with all cores at speed 6 and 49–52 % less with one thread, the same order as Neon on macOS.
2. **libavif-rs speed 6 is now smaller and faster than `rav1e` on x86-64 too**: 5–13 % fewer bytes, 41–47 % less total time with all cores and 43–50 % less with one thread. Speed 7 is 62–65 % faster than `rav1e` with all cores.
3. **Without SIMD the two were close on x86-64**: with all cores the scalar AOM is within 6 % of `rav1e` on dog and Kodak and 15 % slower on the flower, where on the M1 Ultra it was 8–32 % slower. The lead with AVX2 is therefore larger on this runner than with Neon on macOS (41–47 % against 21–40 %).

## What the numbers say

1. **libavif-rs speed 6 is the smallest encoder measured**: 5–13 % below `rav1e` and at or below Sharp's libaom on all four photographs, at 8–32 % more end-to-end time than `rav1e` with all cores (dog 401 against 357 ms) and 21–50 % more with one thread.
2. **libavif-rs speed 7 is faster than `rav1e` and smaller on three of four images**: 25–43 % less total time with all cores (dog 267 against 357 ms, flower 147 against 206 ms, Kodak 55–56 against 97 ms), 5–7 % fewer bytes on dog, flower and kodim03, 2.5 % more on kodim23.
3. **The port had no SIMD yet** (see [After Neon](#after-neon-libavif-rs-c9b583c) for the re-run). Its single-thread speed 6 encode is 1.4–1.5× `rav1e`'s on the 1600-wide photos (3.7 against 2.7 s for the dog); the all-cores result comes from libavif's automatic tiling and AOM's threading. SIMD in libavif-rs's AOM would move every libavif-rs row.
4. **Against Sharp end to end** (from the Sharp comparison, a different run and load): Sharp needed 679 ms in process for the dog and 314 ms for the flower, with matched files 3–5 % larger than libavif-rs speed 6; Ferrixel with libavif-rs speed 6 needs 401 and 271 ms. On the 512-wide Kodak images Sharp's 61–67 ms in process stays ahead of both Ferrixel encoders at speed 6.

## Limits

- Two machines (one M1 Ultra, one shared four-vCPU CI runner), four photographs of at most 12 Mpx, SSIMULACRA2 only (no Butteraugli).
- The default stays `rav1e`; libavif-rs needs nightly and runs on aarch64 macOS and x86-64 GNU Linux only (ADR-0024). It needed libvmaf at the measured revisions; since `8be367f` its AOM is built without VMAF tuning, with byte-identical output.
- libavif's bytes depend on the thread count; these are all-cores numbers.
