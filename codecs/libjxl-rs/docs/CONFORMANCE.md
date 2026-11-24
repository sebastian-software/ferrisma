# External JPEG XL conformance corpus

This record covers the decoder side of issue #95: the public JPEG XL
conformance corpus, run against the Rust port with the corpus's own runner and
tolerance rules. It complements the `decode_test.cc` inventory, which compares
the port with the pinned C++ decoder exactly (see
[PORT_STATUS.md](PORT_STATUS.md#public-decoder-api-test-inventory-2026-11-04)).
The inventory compares against the original implementation; this corpus
compares against independently published reference images.

## Provenance

- Corpus: [`libjxl/conformance`](https://github.com/libjxl/conformance) at
  `b1d0f990b03e57bf6d137c365cd5dc8b470b9191` (committed 2026-09-05), BSD-style
  repository license; per-case image licenses are listed in its
  `testcases/README.md` (CC0, CC BY 3.0, CC BY-SA 3.0, ITU-T T.24 and one
  Microsoft custom license). No corpus file is vendored here.
- [`tools/conformance/MANIFEST.sha256`](../tools/conformance/MANIFEST.sha256)
  pins the SHA-256 of all 40 `input.jxl` files, their `test.json` descriptors,
  both corpus lists and the runner scripts `scripts/conformance.py` and
  `scripts/lcms2.py`. Reference images (`reference_image.npy`), reference and
  original ICC profiles and reconstructed JPEGs are content-addressed objects
  in the corpus's public bucket; each is checked against the SHA-256 that its
  `test.json` records before use. The `_5` cases link to the inputs of their
  level-10 siblings and differ only in references and tolerances.
- Runner: the corpus's unmodified `scripts/conformance.py` with numpy 2.5.4 and
  Little CMS 2.19.1 (Homebrew) on 2026-11-12; on 2026-11-15 with numpy 2.5.4
  and Little CMS 2.17, the revision libjxl pins as `third_party/lcms`, built
  as a local shared library (`LCMS2_LIB_PATH`). The AArch64 scalar run of
  2026-11-15 reproduces every value of the 2026-11-12 tables, so the CMS
  version does not show at the recorded precision. The script decides pass
  or fail.

## Method

`tools/conformance/run.sh [--target <triple>] <workdir>` downloads the
archive at the pinned revision, verifies the manifest and every object
(objects already in `<workdir>/objects` are reused after their check), builds
the [`conformance_decode`](../examples/conformance_decode.rs) example in
release mode from this directory (so the port's scalar `.cargo/config.toml`
flags apply), for `--target` if given, and runs `conformance.py` over
`main_level10.txt` and `main_level5.txt`.

The decoder path is the port's default selection (NEON on AArch64, AVX2 on
x86-64 when detected; port [ADR 0004](adr/0004-simd-paths.md), "Default
selection on x86-64"); `JXL_DECODER_TARGET=scalar` in the environment makes
`conformance_decode` force the scalar path. `run.sh` prints the path the
decoder selects (`conformance_decode --decoder_path`), and every decode
appends the selected path and whether a SIMD kernel ran
(`hwy::simd_selections_on_this_thread`) to
`<workdir>/decoder-path-level<N>.log`, which `run.sh` counts after each
level. Under Rosetta 2 the x86-64 decoder needs `ROSETTA_ADVERTISE_AVX=1`,
or it does not detect AVX2 and prints `scalar`.

`conformance_decode` is a djxl-compatible front end that links only the Rust
port. It mirrors the public-API sequence of the pinned
`lib/extras/dec/jxl.cc` `DecodeImageJXL` as djxl uses it for `.npy` output:
`--norender_spotcolors`, coalesced frames, upright orientation, no
unpremultiplication, float little-endian color channels and every extra
channel as a separate float plane, the DATA profile for `--icc_out` and the
ORIGINAL profile for `--orig_icc_out`. For `.jpg` output it performs JPEG
reconstruction with djxl's growing output buffer. Two documented differences
do not affect samples: pixels go to an output buffer rather than djxl's
image-out callback, and the metadata JSON keeps the codestream bit depths.

The runner's rules, unchanged:

- Each frame: maximum per-channel RMSE and peak absolute error against the
  reference, after transforming the decoded samples from the decoder's DATA
  ICC to `reference.icc` with Little CMS whenever the two profiles differ and
  the case is not lossless (`peak_error > 0`).
- `original.icc` and `reconstructed.jpg`, where listed, must be byte-identical.
- The metadata check reaches the frame list and the first frame's name (the
  script returns from its first comparison).

## Results per decoder path (2026-11-15)

Rust port at `36ffeb81` plus the `run.sh` and `conformance_decode` changes
that add this section, on an M1 Pro (macOS 27). The Python runner and Little
CMS run natively (arm64) in all four configurations; only the decoder process
changes.

| Configuration | Decoder build | Path (every decode) | Level 10 | Level 5 | Exact files | Wrapper `--lax` |
| --- | --- | --- | --- | --- | --- | --- |
| AArch64, default | `aarch64-apple-darwin` | NEON, SIMD kernels ran | 27 / 27 | 24 / 24 | all identical | 32 / 40 |
| AArch64, forced scalar | `aarch64-apple-darwin`, `JXL_DECODER_TARGET=scalar` | scalar, no SIMD kernel | 27 / 27 | 24 / 24 | all identical | 32 / 40 |
| x86-64 under Rosetta 2, default | `x86_64-apple-darwin`, `ROSETTA_ADVERTISE_AVX=1` | AVX2, SIMD kernels ran | 27 / 27 | 24 / 24 | all identical | 32 / 40 |
| x86-64 under Rosetta 2, forced scalar | `x86_64-apple-darwin`, `ROSETTA_ADVERTISE_AVX=1`, `JXL_DECODER_TARGET=scalar` | scalar, no SIMD kernel | 27 / 27 | 24 / 24 | all identical | 32 / 40 |

"Path" is the decoder-path log over all 30 level-10 and 27 level-5 decodes
(one per frame set or reconstructed JPEG). The wrapper ignores
`JXL_DECODER_TARGET` and runs the default path in every row; its eight
failures are the by-design `Unsupported` rejections described below, the
same in every configuration.

What a Rosetta run is: the x86-64 machine code of the port, including its
AVX2 and FMA kernels and its scalar x86-64 build, executed by Rosetta 2's
translation. What it is not: Linux, or x86-64 hardware. Rosetta 2 does not
fault on a misaligned aligned move and is not a vendor's floating-point unit
(`rcpps`, for example, is whatever Rosetta returns). The manual workflow
[`jxl-conformance.yml`](../../../.github/workflows/jxl-conformance.yml) runs
`run.sh` on `ubuntu-24.04` with the default and the forced scalar path.

### Native Linux x86-64 (2026-11-15)

Hosted run
[43690766156](https://github.com/sebastian-software/ferrisma/actions/runs/43690766156)
of that workflow on `main` at `b3f29a71` (`ubuntu-24.04`, x86-64 hardware):

| Configuration | Path (every decode) | Level 10 | Level 5 |
| --- | --- | --- | --- |
| Linux x86-64, default | AVX2, SIMD kernels ran | 27 / 27 | 24 / 24 |
| Linux x86-64, forced scalar (`JXL_DECODER_TARGET=scalar`) | scalar, no SIMD kernel | 27 / 27 | 24 / 24 |

The path is the decoder-path log of the 30 level-10 decodes of each job. The
run's logs and result files are its artifacts (kept seven days); this table
is the record.

Repeated on 2026-11-24 at the current tree: hosted run
[45343416525](https://github.com/sebastian-software/ferrisma/actions/runs/45343416525)
of the same workflow on branch `v1/jxl-x86-conformance-276` (`01dfdc80`, the tip of
`main` at the time, `ubuntu-24.04`, x86-64 hardware; the runner does
not print its CPU model). Both jobs succeeded:

| Configuration | Path (every decode) | Level 10 | Level 5 |
| --- | --- | --- | --- |
| Linux x86-64, default | AVX2, SIMD kernels ran (30 and 27 decodes) | 27 / 27 | 24 / 24 |
| Linux x86-64, forced scalar | scalar, no SIMD kernel (30 and 27 decodes) | 27 / 27 | 24 / 24 |

The `alpha_premultiplied` peak error is 3.07e-06 (AVX2) and 3.04e-06
(scalar), the same as on 2026-11-15. This is the external corpus's own runner
on x86-64 hardware: no Rosetta 2 result stands in for it. The AVX2 cap on
AVX-512 hardware is a separate item (`docs/quality-gate.md`).

The wrapper row of that run reads 31 of 40, not 32: `alpha_premultiplied`
fails in `--lax` mode on both paths (RMSE 0.468 against a limit of 3.815e-06).
This is not a hardware difference. Commit `ebc0c5a7` (2026-11-15, after the
`b3f29a71` run above) made the wrapper hand out straight alpha through
libjxl's own un-premultiply step, while the corpus reference of that case is
premultiplied and the runner's lax mode compares samples as emitted. The
decoder rows above, which keep premultiplied alpha, pass. The eight
by-design `Unsupported` rejections are unchanged. Peak error of `alpha_premultiplied`: 3.07e-06 on AVX2 and
3.04e-06 on the scalar path.

Peak error per case, the maximum over frames and channels, and in
parentheses its share of the case's limit (the maximum over frames of
peak / limit; "-" for a lossless case with limit 0). Level-5 cases that are
also listed at level 10 repeat that row.

| Case | Peak limit | AArch64 scalar | AArch64 NEON | x86-64 scalar | x86-64 AVX2 |
| --- | --- | --- | --- | --- | --- |
| `alpha_nonpremultiplied` | 6.1035e-05 | 1.86e-09 (0.0 %) | 1.86e-09 (0.0 %) | 1.86e-09 (0.0 %) | 1.86e-09 (0.0 %) |
| `alpha_premultiplied` | 3.815e-06 | 3.04e-06 (79.7 %) | 3.07e-06 (80.5 %) | 3.04e-06 (79.7 %) | 3.07e-06 (80.5 %) |
| `alpha_triangles` | 0.00195312 | 2.98e-08 (0.0 %) | 2.98e-08 (0.0 %) | 2.98e-08 (0.0 %) | 2.98e-08 (0.0 %) |
| `animation_icos4d` | 0.005 | 2.63e-05 (0.5 %) | 7.73e-05 (1.5 %) | 2.63e-05 (0.5 %) | 2.91e-05 (0.6 %) |
| `animation_newtons_cradle` | 0.000976562 | 0 (0.0 %) | 0 (0.0 %) | 0 (0.0 %) | 0 (0.0 %) |
| `animation_spline` | 0.004 | 1.17e-05 (0.3 %) | 1.17e-05 (0.3 %) | 1.4e-05 (0.4 %) | 1.39e-05 (0.3 %) |
| `bench_oriented_brg` | 0.004 | 1.91e-06 (0.0 %) | 1.37e-05 (0.3 %) | 1.91e-06 (0.0 %) | 8.94e-07 (0.0 %) |
| `bicycles` | 0.000976562 | 4.95e-06 (0.5 %) | 1.49e-08 (0.0 %) | 4.95e-06 (0.5 %) | 1.49e-08 (0.0 %) |
| `bike` | 0.007 | 0.000249 (3.6 %) | 0.000258 (3.7 %) | 0.000249 (3.6 %) | 0.00025 (3.6 %) |
| `blendmodes` | 0.0001 | 1.43e-06 (1.4 %) | 1.43e-06 (1.4 %) | 2.98e-08 (0.0 %) | 2.98e-08 (0.0 %) |
| `cafe` | 0.004 | 4.95e-06 (0.1 %) | 3.51e-05 (0.9 %) | 4.95e-06 (0.1 %) | 2.62e-06 (0.1 %) |
| `cmyk_layers` | 0.000976562 | 5.96e-08 (0.0 %) | 5.96e-08 (0.0 %) | 0 (0.0 %) | 0 (0.0 %) |
| `delta_palette` | 0.000976562 | 0 (0.0 %) | 0 (0.0 %) | 0 (0.0 %) | 0 (0.0 %) |
| `grayscale_jpeg` | 0.004 | 1.79e-06 (0.0 %) | 1.31e-05 (0.3 %) | 1.79e-06 (0.0 %) | 8.94e-07 (0.0 %) |
| `grayscale` | 0.004 | 0.000228 (5.7 %) | 0.000228 (5.7 %) | 0.000228 (5.7 %) | 0.000228 (5.7 %) |
| `grayscale_public_university` | 0.000976562 | 8.05e-07 (0.1 %) | 0 (0.0 %) | 8.05e-07 (0.1 %) | 0 (0.0 %) |
| `lossless_pfm` | 0 | 0 (-) | 0 (-) | 0 (-) | 0 (-) |
| `lz77_flower` | 0.000976562 | 0 (0.0 %) | 0 (0.0 %) | 0 (0.0 %) | 0 (0.0 %) |
| `mul_no_extra_channels` | 0.004 | 0 (0.0 %) | 0 (0.0 %) | 0 (0.0 %) | 0 (0.0 %) |
| `noise` | 0.004 | 2.93e-05 (0.7 %) | 0.000166 (4.1 %) | 2.93e-05 (0.7 %) | 1.35e-05 (0.3 %) |
| `opsin_inverse` | 0.004 | 7.62e-06 (0.2 %) | 4.2e-05 (1.0 %) | 7.62e-06 (0.2 %) | 7.64e-06 (0.2 %) |
| `patches` | 0.004 | 6.68e-06 (0.2 %) | 4.72e-05 (1.2 %) | 6.68e-06 (0.2 %) | 3.76e-06 (0.1 %) |
| `patches_lossless` | 0.000976562 | 0 (0.0 %) | 0 (0.0 %) | 0 (0.0 %) | 0 (0.0 %) |
| `progressive` | 0.02 | 2.01e-05 (0.1 %) | 0.000115 (0.6 %) | 2.01e-05 (0.1 %) | 1.03e-05 (0.1 %) |
| `spot` | 3.815e-06 | 5.96e-08 (1.6 %) | 5.96e-08 (1.6 %) | 0 (0.0 %) | 0 (0.0 %) |
| `sunset_logo` | 0.000244141 | 2.38e-07 (0.1 %) | 2.38e-07 (0.1 %) | 2.38e-07 (0.1 %) | 2.38e-07 (0.1 %) |
| `upsampling` | 0.004 | 4.45e-05 (1.1 %) | 0.000295 (7.4 %) | 4.45e-05 (1.1 %) | 4.77e-05 (1.2 %) |
| `animation_icos4d_5` | 0.06 | 0.0557 (92.9 %) | 0.0557 (92.9 %) | 0.0557 (92.9 %) | 0.0557 (92.9 %) |
| `animation_spline_5` | 0.06 | 1.17e-05 (0.0 %) | 1.17e-05 (0.0 %) | 1.4e-05 (0.0 %) | 1.39e-05 (0.0 %) |
| `bench_oriented_brg_5` | 0.06 | 1.91e-06 (0.0 %) | 1.37e-05 (0.0 %) | 1.91e-06 (0.0 %) | 8.94e-07 (0.0 %) |
| `bike_5` | 0.06 | 0.000249 (0.4 %) | 0.000258 (0.4 %) | 0.000249 (0.4 %) | 0.00025 (0.4 %) |
| `blendmodes_5` | 0.06 | 1.43e-06 (0.0 %) | 1.43e-06 (0.0 %) | 2.98e-08 (0.0 %) | 2.98e-08 (0.0 %) |
| `cafe_5` | 0.06 | 4.95e-06 (0.0 %) | 3.51e-05 (0.1 %) | 4.95e-06 (0.0 %) | 2.62e-06 (0.0 %) |
| `grayscale_5` | 0.06 | 0.000228 (0.4 %) | 0.000228 (0.4 %) | 0.000228 (0.4 %) | 0.000228 (0.4 %) |
| `grayscale_jpeg_5` | 0.06 | 1.79e-06 (0.0 %) | 1.31e-05 (0.0 %) | 1.79e-06 (0.0 %) | 8.94e-07 (0.0 %) |
| `noise_5` | 0.06 | 2.93e-05 (0.0 %) | 0.000166 (0.3 %) | 2.93e-05 (0.0 %) | 1.35e-05 (0.0 %) |
| `opsin_inverse_5` | 0.06 | 7.62e-06 (0.0 %) | 4.2e-05 (0.1 %) | 7.62e-06 (0.0 %) | 7.64e-06 (0.0 %) |
| `patches_5` | 0.06 | 0.0241 (40.1 %) | 0.024 (40.1 %) | 0.0241 (40.1 %) | 0.0241 (40.1 %) |
| `progressive_5` | 0.06 | 2.01e-05 (0.0 %) | 0.000115 (0.2 %) | 2.01e-05 (0.0 %) | 1.03e-05 (0.0 %) |
| `upsampling_5` | 0.06 | 0.0163 (27.2 %) | 0.0164 (27.3 %) | 0.0163 (27.2 %) | 0.0163 (27.2 %) |

Margins:

- No case changes result. The largest move between configurations is 6.3
  points of a limit (`upsampling`, NEON). The closest margin is the same
  everywhere:
  `animation_icos4d_5` 0.0557 of 0.06 (92.9 %) on all four.
- `alpha_premultiplied` (issue #215's "about 80 %"): 3.04e-06 of 3.815e-06
  (79.7 %) on both scalar paths, 3.07e-06 (80.5 %) on NEON and on AVX2. The
  SIMD paths are 0.8 points closer to the limit.
- NEON is further from the references than the other three on the lossy
  VarDCT cases (`upsampling` 7.4 % against 1.1-1.2 %, `noise` 4.1 % against
  0.3-0.7 %, `cafe`, `patches`, `progressive`, `opsin_inverse`), and AVX2 is
  at or below the scalar paths. This is the original's own target
  difference, not a port defect: the public decoder gate
  (`tests/simd_public.rs` with `JXL_CONFORMANCE_CORPUS` set, run again at
  this revision on 2026-11-15) finds the port's NEON decodes word-identical
  with libjxl's NEON build and its AVX2 decodes (Rosetta 2) with libjxl's
  AVX2 build over these inputs. `djxl` itself was not run.
- The two scalar paths differ slightly from each other on `animation_spline`,
  `blendmodes`, `cmyk_layers` and `spot` (all far below their limits). The
  scalar gate compares each target's scalar path with the scalar original
  built for that target (`tools/check.py`, `--target x86_64-apple-darwin`);
  this run did not compare them with `djxl`.

## Detailed record: AArch64 scalar path (2026-11-12, reproduced 2026-11-15)

Rust port at the commit that added this record, when the decoder still ran
the scalar path by default on every target; the forced scalar AArch64 run of
2026-11-15 gives the same values in every cell. All 40 corpus directories are
covered by the union of the two lists. Every case passes; every
`original.icc` and every reconstructed JPEG is byte-identical. Values are the
maximum over frames and channels; "limit" is the case's tolerance.

### Level 10 (`main_level10.txt`): 27 of 27 pass

| Case | Result | Frames | Max RMSE / limit | Max peak / limit | Exact files |
| --- | --- | ---: | --- | --- | --- |
| `alpha_nonpremultiplied` | pass | 1 | 5.82e-11 / 6.1035e-05 | 1.86e-09 / 6.1035e-05 | - |
| `alpha_premultiplied` | pass | 1 | 1.78e-07 / 3.815e-06 | 3.04e-06 / 3.815e-06 | - |
| `alpha_triangles` | pass | 1 | 1.64e-09 / 0.00195312 | 2.98e-08 / 0.00195312 | - |
| `animation_icos4d` | pass | 48 | 5.39e-07 / 0.0001 | 2.63e-05 / 0.005 | - |
| `animation_newtons_cradle` | pass | 36 | 0 / 0.000976562 | 0 / 0.000976562 | - |
| `animation_spline` | pass | 60 | 2.93e-07 / 0.0001 | 1.17e-05 / 0.004 | - |
| `bench_oriented_brg` | pass | 1 | 2.66e-07 / 1e-05 | 1.91e-06 / 0.004 | reconstructed.jpg identical, original.icc identical |
| `bicycles` | pass | 1 | 1.78e-07 / 0.000976562 | 4.95e-06 / 0.000976562 | - |
| `bike` | pass | 1 | 2.59e-06 / 0.0001 | 0.000249 / 0.007 | original.icc identical |
| `blendmodes` | pass | 1 | 1.17e-07 / 0.004 | 1.43e-06 / 0.0001 | - |
| `cafe` | pass | 1 | 4.59e-07 / 1e-05 | 4.95e-06 / 0.004 | reconstructed.jpg identical, original.icc identical |
| `cmyk_layers` | pass | 1 | 2.69e-09 / 0.000976562 | 5.96e-08 / 0.000976562 | original.icc identical |
| `delta_palette` | pass | 1 | 0 / 0.000976562 | 0 / 0.000976562 | - |
| `grayscale_jpeg` | pass | 1 | 1.33e-07 / 1e-05 | 1.79e-06 / 0.004 | reconstructed.jpg identical, original.icc identical |
| `grayscale` | pass | 1 | 7.82e-06 / 0.0001 | 0.000228 / 0.004 | original.icc identical |
| `grayscale_public_university` | pass | 1 | 5.11e-08 / 0.000976562 | 8.05e-07 / 0.000976562 | original.icc identical |
| `lossless_pfm` | pass | 1 | 0 / 0 | 0 / 0 | - |
| `lz77_flower` | pass | 1 | 0 / 0.000976562 | 0 / 0.000976562 | original.icc identical |
| `mul_no_extra_channels` | pass | 1 | 0 / 0.0001 | 0 / 0.004 | - |
| `noise` | pass | 1 | 5.39e-07 / 0.0001 | 2.93e-05 / 0.004 | - |
| `opsin_inverse` | pass | 1 | 6.12e-07 / 0.0001 | 7.62e-06 / 0.004 | - |
| `patches` | pass | 1 | 1.39e-07 / 0.0001 | 6.68e-06 / 0.004 | original.icc identical |
| `patches_lossless` | pass | 1 | 0 / 0.000976562 | 0 / 0.000976562 | original.icc identical |
| `progressive` | pass | 1 | 4.08e-07 / 0.0001 | 2.01e-05 / 0.02 | original.icc identical |
| `spot` | pass | 1 | 1.52e-09 / 3.815e-06 | 5.96e-08 / 3.815e-06 | original.icc identical |
| `sunset_logo` | pass | 1 | 1.71e-08 / 0.000244141 | 2.38e-07 / 0.000244141 | - |
| `upsampling` | pass | 1 | 1.25e-06 / 0.0001 | 4.45e-05 / 0.004 | - |

### Level 5 (`main_level5.txt`): 24 of 24 pass

Cases also listed at level 10 repeat that result. The level-5-only cases:

| Case | Result | Frames | Max RMSE / limit | Max peak / limit | Exact files |
| --- | --- | ---: | --- | --- | --- |
| `animation_icos4d_5` | pass | 48 | 0.00113 / 0.02 | 0.0557 / 0.06 | - |
| `animation_spline_5` | pass | 60 | 2.93e-07 / 0.02 | 1.17e-05 / 0.06 | - |
| `bench_oriented_brg_5` | pass | 1 | 2.66e-07 / 0.02 | 1.91e-06 / 0.06 | reconstructed.jpg identical, original.icc identical |
| `bike_5` | pass | 1 | 2.59e-06 / 0.02 | 0.000249 / 0.06 | original.icc identical |
| `blendmodes_5` | pass | 1 | 1.17e-07 / 0.02 | 1.43e-06 / 0.06 | - |
| `cafe_5` | pass | 1 | 4.59e-07 / 0.02 | 4.95e-06 / 0.06 | reconstructed.jpg identical, original.icc identical |
| `grayscale_5` | pass | 1 | 7.82e-06 / 0.02 | 0.000228 / 0.06 | original.icc identical |
| `grayscale_jpeg_5` | pass | 1 | 1.33e-07 / 0.02 | 1.79e-06 / 0.06 | reconstructed.jpg identical, original.icc identical |
| `noise_5` | pass | 1 | 5.39e-07 / 0.02 | 2.93e-05 / 0.06 | - |
| `opsin_inverse_5` | pass | 1 | 6.12e-07 / 0.02 | 7.62e-06 / 0.06 | - |
| `patches_5` | pass | 1 | 0.000255 / 0.02 | 0.0241 / 0.06 | original.icc identical |
| `progressive_5` | pass | 1 | 4.08e-07 / 0.02 | 2.01e-05 / 0.06 | original.icc identical |
| `upsampling_5` | pass | 1 | 0.000258 / 0.02 | 0.0163 / 0.06 | - |

The level-5 references of `animation_icos4d_5`, `patches_5` and
`upsampling_5` differ from their level-10 references; the same decoded
samples therefore show larger, still passing, errors there.

Margins worth watching: `alpha_premultiplied` reaches 3.04e-06 of its
3.815e-06 peak limit (about 80 %) and `animation_icos4d_5` 0.0557 of 0.06.
A future SIMD or reordered-float change should rerun this corpus, not only
the exact C++ oracle, before it lands.

## Ferrisma wrapper in application mode

`run.sh` also builds `ferrisma-jxl-decoder`'s
[`jxl_conformance_wrapper`](../../../crates/ferrisma-jxl-decoder/examples/jxl_conformance_wrapper.rs)
from the workspace root (the engine's build, without this directory's scalar
flags) and runs all 40 cases in the runner's `--lax` mode. That front end
emits exactly the engine input: the upright interleaved F32 frame of
`decode_into` and the DATA ICC of `inspect`. Lax mode skips the ORIGINAL
profile and JPEG files, which the wrapper does not expose, and clamps both
images to 0..1 before comparing; the color transform from the wrapper's DATA
ICC to the reference profile still runs. 32 of 40 cases pass. The other
eight are rejected by design in `inspect` with `Unsupported` before any pixel
allocation: five animations, `cmyk_layers` (Black channel), `spot` (spot
colors) and `lossless_pfm` (float samples). No accepted case fails, which
covers premultiplied and straight alpha, grayscale with ICC, lossy images
whose DATA profile is the original enumerated encoding (`bike`, sRGB
primaries with a BT.709 transfer) or the linear fallback the decoder emits
for XYB images with an ICC original (`progressive`, linear sRGB;
`grayscale`, linear gray), orientation (`bench_oriented_brg`, `sunset_logo`),
upsampling, patches, noise, splines and progressive input.

## Not established

- Linux AArch64.
- A scheduled CI job: the run needs network access, the reference objects
  (about 0.6 GiB in `objects/`) and numpy plus Little CMS, so it is a manual
  deep check (`workflow_dispatch` only).
- Preview output (`--preview_out`): no corpus case has a preview.
- Encoder conformance: the corpus tests decoders only.
