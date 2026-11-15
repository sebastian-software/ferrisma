# Format-parity fixtures (ADR-0043 D5)

Files for the rows of `tests/format_parity.rs` that had no fixture in the
repository: a property only counts as supported in a format once a real
file carries it. Each file is written by an original tool or derived from a
tracked file by a documented byte patch; nothing here is hand-edited.

| File | What it carries | Made by | SHA-256 |
| --- | --- | --- | --- |
| `rgb-p3-icc.jxl` | 16 x 8 RGB8, lossless, with an embedded ICC profile (`../icc/display-p3-gamma22.icc`, unchanged) instead of an enum color encoding | `cjxl` | `37862bf386c1d596ba2e7b91170262afe5f1a7e0b6054b23066b47291eadec79` |
| `gray-gamma22.icc` | ICC v2.1 monitor profile, color space `GRAY`, PCS XYZ, D50 white, one `kTRC` gamma curve 2.19921875 (u8Fixed8 `0x0233`) | `tools/make.py` | `2c1f8f9ac2b6441d455d78c1d2bc6e2d1b2d6938e0befb7f04828749f0e0aa0a` |
| `gray-gray-icc.jxl` | 16 x 8 gray8, lossless, with `gray-gamma22.icc` embedded | `cjxl` | `bbe930856b34eab670b1560d881f212a39c983f1f75ba08d8c620dd6846a9674` |
| `gray-gray-icc.jpg` | libjxl's `ycbcr-jpeg-gray.jpg` with `gray-gamma22.icc` in an `APP2` segment | `tools/patch.py` | `ace8dd9fed0fc3acc016d87631a0ad077d0b8361bbf43a035bbee4eee7dcc638` |
| `gray-rgb-icc.jpg` | the same gray JPEG with the RGB profile `../icc/paris-srgb.icc` (a mismatch) | `tools/patch.py` | `5429dfa4fa50b73f5e48d50489165d13e5039abb6b2e13b81a3daf9706ca9d45` |
| `rgb-gray-icc.jpg` | `../sample.jpg` (RGB) with `gray-gamma22.icc` (a mismatch) | `tools/patch.py` | `acd5ba2630ce0c7eabaf89bd51ecc2675d98dfbf6803d42012ffd1e0e32bf978` |

## Inputs

`tools/make.py` (standard library only) writes the encoder inputs into a
scratch directory: the 16 x 8 RGB samples (`r = 16x`, `g = 32y`,
`b = 255 - 16x`) as `rgb-16x8.raw`, the 16 x 8 gray samples
(`(16x + 8y) mod 256`) as `gray-16x8.raw`, and the gray profile, which is
also committed here. `cjxl` reads PNM, so the raw samples are wrapped in a
binary PPM or PGM header. `tools/patch.py` derives the JPEG and AVIF files
from tracked files; its docstring lists every changed byte.

## Regeneration

`cjxl` is the original libjxl command-line encoder at the commit the
workspace port is pinned to: `cjxl v0.13.0 a16d32d [_NEON_]`, built from
`codecs/libjxl-rs/upstream/libjxl` (CMake `Release`, AppleClang 21,
skcms) on macOS ARM64, 2026-11-15. From the repository root, with `CJXL`
pointing at that build:

```sh
out=$(mktemp -d)
python3 -I tests/fixtures/parity/tools/make.py "$out"
{ printf 'P6\n16 8\n255\n'; cat "$out/rgb-16x8.raw"; } > "$out/rgb-16x8.ppm"
"$CJXL" "$out/rgb-16x8.ppm" tests/fixtures/parity/rgb-p3-icc.jxl -d 0 -e 7 \
  -x icc_pathname=tests/fixtures/icc/display-p3-gamma22.icc
{ printf 'P5\n16 8\n255\n'; cat "$out/gray-16x8.raw"; } > "$out/gray-16x8.pgm"
cmp "$out/gray-gamma22.icc" tests/fixtures/parity/gray-gamma22.icc
"$CJXL" "$out/gray-16x8.pgm" tests/fixtures/parity/gray-gray-icc.jxl -d 0 -e 7 \
  -x icc_pathname=tests/fixtures/parity/gray-gamma22.icc
python3 -I tests/fixtures/parity/tools/patch.py
shasum -a 256 tests/fixtures/parity/*.*
```

Two runs produced byte-identical files. The gray JPEG XL file keeps its
profile as an ICC stream as well (`original_icc`), and its gray samples are
exactly `gray-16x8.raw` (lossless). libjxl keeps the profile as an ICC
stream (`want_icc`) because it is not byte-identical to the profile libjxl
would generate for an enum encoding; the bridge reports
`ImageInfo::original_icc` for it and no enum DATA encoding.

The files are licensed like the repository's own test data; the embedded
profile keeps the provenance recorded in `../icc/README.md`.
