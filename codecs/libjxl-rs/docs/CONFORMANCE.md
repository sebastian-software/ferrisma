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
  Little CMS 2.19.1 (Homebrew). The script decides pass or fail.

## Method

`tools/conformance/run.sh <workdir>` downloads the archive at the pinned
revision, verifies the manifest and every object, builds the
[`conformance_decode`](../examples/conformance_decode.rs) example in release
mode from this directory (so the port's scalar `.cargo/config.toml` flags
apply), and runs `conformance.py` over `main_level10.txt` and
`main_level5.txt`.

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

## Results (macOS ARM64, 2026-11-12)

Rust port at the commit that adds this record. All 40 corpus directories are
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

- x86-64 and Linux results; only macOS ARM64 was run.
- A CI job: the run needs network access, about 1.6 GiB of references and
  numpy plus Little CMS, so it is a manual deep check for now.
- Preview output (`--preview_out`): no corpus case has a preview.
- Encoder conformance: the corpus tests decoders only.
