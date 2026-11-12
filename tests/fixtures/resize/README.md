# Independent resize references (issue #18)

`tests/resize_oracle.rs` compares the engine's Area and Lanczos3 resize, end
to end through the public facade, with two oracles that do not use the
engine's own `f64` reference (`src/quality.rs`, `src/quality_execute.rs`):

1. **ImageMagick 7.1.2-35 Q16-HDRI** (`aarch64 acaf7e418:20261031`, Homebrew,
   macOS 27.0, Apple M1 Ultra). The `.f32` files here are its results.
2. **`tests/support/resize_oracle.rs`**, a separately written direct 2D
   summation from the filter definitions in its module comment. It is itself
   compared with (1) in linear light.

## Inputs

Inputs are synthesized, not stored. `tools/generate.py` and
`resize_oracle::pattern` produce the same RGBA8 images bit for bit; the test
asserts the FNV-1a 64 hashes printed by the script:

| Input | Size | Alpha | Seed | FNV-1a 64 |
| --- | --- | --- | --- | --- |
| `opaque` | 48 x 36 | no | `0x5EED0001` | `30b0f565c6c5260b` |
| `alpha` | 40 x 30 | yes | `0x5EED0002` | `993d2ea6c942cb59` |
| `large_opaque` | 240 x 160 | no | `0x5EED0003` | `684a822c501a4db5` |
| `large_alpha` | 240 x 160 | yes | `0x5EED0004` | `89e564541ce9e416` |

Each image has four vertical bands: random RGB noise, a one-pixel black/white
checkerboard (ringing and halo), a hard red/blue edge, and a gradient. Alpha
inputs add quadrants: opaque; fully transparent with saturated green RGB
that must never leak; 0/255 stripes two pixels wide (hard straight-alpha
edges and exact rounding ties); random alpha including tiny values.

The engine receives these images as lossless WebP (libwebp-rs), so the WebP
decoder, crop, sRGB decode, linear-light association, both filter passes,
unassociation and the final sRGB8 quantization are inside the comparison.

## Regeneration

```sh
python3 -I tests/fixtures/resize/tools/generate.py
shasum -a 256 tests/fixtures/resize/*.f32
```

Each command is printed by the script. Every reference starts with
`-size WxH -depth 8 rgba:<input> -set colorspace sRGB [-crop ... +repage]
-colorspace RGB` (ImageMagick's linear-light RGB) and ends with `-depth 32
-define quantum:format=floating-point -endian LSB rgba:<case>.f32`: linear
light, straight alpha, unclamped float32. The filter steps are:

- **Lanczos3:** `-filter Lanczos -define filter:filter=Sinc -define
  filter:window=Sinc -define filter:lobes=3 -resize WxH!`. The defines replace
  ImageMagick's default `SincFast` polynomial with the exact sinc, so the
  kernel is `sinc(t) sinc(t/3)`, `|t| < 3`, stretched by the downscale ratio.
- **Area:** ImageMagick has no single filter equal to Ferrisma's Area at all
  ratios, and matching names are not matching kernels. Ferrisma's Area is a
  box of width `max(n/m, 1)` source pixels integrated over each source pixel.
  On a downscaled axis that is pixel-area averaging, which is ImageMagick's
  `-scale`. On an upscaled axis a unit box over unit pixels is the tent
  `1 - |d|`, which is ImageMagick's `-filter Triangle -resize` at
  magnification. The script applies the matching operator per axis
  (horizontal, then vertical). `-filter Box -resize` was rejected because it
  point-samples the box instead of integrating it.

Both programs clip taps at the image border and renormalize the remaining
weights; the agreement below confirms the shared convention.

| Case | Source (crop) | Output | SHA-256 of `.f32` |
| --- | --- | --- | --- |
| `opaque-up-lanczos3` | `opaque` | 101 x 77 | `7b5ae5851617a4fa08d310a3f64624f3b9658c6a5da2b8a37c3146442a64cc9e` |
| `opaque-up-area` | `opaque` | 101 x 77 | `dcf2b29642f6dc222c5d0914f4da3a17e274f915018e2861a07903187a8c6066` |
| `alpha-up-lanczos3` | `alpha` | 87 x 61 | `acbe44b5ecac8c5f06f3ddb35723bb0cf898b0fc42d293e6f8a8f0a0ae84a5a5` |
| `alpha-up-area` | `alpha` | 87 x 61 | `42fd7367f19c28660da4223f75594a679f92a64339efa95ace9d52041c625eb2` |
| `large-opaque-extreme-lanczos3` | `large_opaque` | 3 x 2 | `75c6efc168f5cf44896cd5a7c668d4e7c65a02c5ef818ab9edddce59eba52e0a` |
| `large-opaque-extreme-area` | `large_opaque` | 3 x 2 | `604820ac642a7c4b279c275601f8b4f877feb0a85236c391f2b44061c295aeba` |
| `large-alpha-extreme-lanczos3` | `large_alpha` | 4 x 3 | `3df94c43ba05c7848b4c34bf5f8e4b566b2349edad1b9347801a66dd31bd92f7` |
| `large-alpha-extreme-area` | `large_alpha` | 4 x 3 | `bcf872a3c252c7d9e6890b314a93402a86f76d326cb105ce37a248a455862219` |
| `opaque-crop-mixed-lanczos3` | `opaque` (7, 5, 29 x 23) | 11 x 40 | `f2c42254632e9d7a2ca040aef8fb2fb7fe3b643ba52a2bce9c175b5b0c95238c` |
| `opaque-crop-mixed-area` | `opaque` (7, 5, 29 x 23) | 11 x 40 | `e5f7a1e537f258461bd2df9a9f8275ad61920eedb6e7ca3072b5b579d9adfb73` |
| `alpha-crop-odd-lanczos3` | `alpha` (3, 1, 31 x 27) | 9 x 8 | `ab77a46bc4a0efbf7b54be6068945808e7f3ffd49b03e6a49912d9307624d70e` |
| `alpha-crop-odd-area` | `alpha` (3, 1, 31 x 27) | 9 x 8 | `6056a8420ed93f2ec410bb8f90ca7a9dff3005bddaab1a46dc76d888bf487a95` |
| `alpha-column-lanczos3` | `alpha` (13, 0, 1 x 30) | 2 x 7 | `6f84bf29aa40e33254400ecfe838449e38d587c36b6203a42b94c9267c9f361a` |
| `large-alpha-mixed-lanczos3` | `large_alpha` (1, 3, 237 x 9) | 5 x 31 | `3bd0660d2cdfbfaa892f0d5407d46b9a243d5435e1ea51d0dc94f675bf22970a` |
| `large-alpha-mixed-area` | `large_alpha` (1, 3, 237 x 9) | 5 x 31 | `e132dd6b7d2c49611246ff2edd011f9063efdeaee8d97a391be309d78b0bfc93` |

The `.f32` files are ImageMagick output and carry no license claim beyond the
repository's; the inputs are generated by the script.

## Tolerances and observed results (2026-11-12)

- **Oracle versus ImageMagick, linear light**, associated RGB and alpha:
  bound `1e-6`, derived in the test from ImageMagick's float32 storage
  (`2^-24` relative, at most three roundings, Lanczos absolute weight sum
  below 1.3) with a factor of four for summation order. Observed maximum
  `1.67e-7` over all 15 cases.
- **Engine versus both oracles, after sRGB8 quantization**: ADR-0021 rule 4
  for `f32` paths, adopted unchanged: at most one code per sample, pooled
  exact RGB at least 99.9 %, pooled exact alpha at least 98 %. Every alpha
  difference must additionally be a rounding tie (the oracle's `255 * alpha`
  within `1e-6` of a half code). Observed for both `F32` and `F64Reference`:
  **all 73,119 compared RGB samples equal**, alpha 27,541 to 27,546 of 27,552
  equal, and every alpha difference a tie (the 0/255 stripes produce exact
  halves). The geometry sweep (identity, 1.5x, 2.37x, 16x, 120x/160x, 1 x 1,
  anisotropic 1x/16x, a far-corner crop, one- and two-pixel sources) against
  the in-tree oracle: all 138,606 RGB samples equal, alpha 60,042 of 60,436
  equal, all differences ties.
- **Display P3 via the rendered-ICC policy** (`DisplayP3-v4.icc`, moxcms):
  per-sample bound `1e-3 * 1.2857^2` linear around the oracle, derived from
  ADR-0023's moxcms/LittleCMS bound, profile quantization and the Lanczos3
  absolute weight sum. All 108,888 samples are inside. An oracle that clips to
  `[0, 1]` before filtering leaves the bound on 24,884 of them, so the test
  detects premature gamut clipping.

## Intentional difference: unassociation above alpha one

Lanczos3 overshoot can make the filtered alpha exceed one near hard alpha
edges. Ferrisma clamps alpha to `[0, 1]` first and divides the associated
color by the clamped alpha, so the opaque result shows the filtered
associated light. ImageMagick divides by the unclamped alpha. The filters
agree (linear-light test above); in `alpha-up-lanczos3` the conventions give
the same codes except on the 506 pixels whose alpha exceeds one, where they
differ by up to 21 codes. The test
`alpha_overshoot_unassociation_differs_from_imagemagick_only_above_one`
retains this as a regression. Both comparisons above apply Ferrisma's output
step to ImageMagick's associated filter result.
