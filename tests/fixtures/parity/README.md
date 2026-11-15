# Format-parity fixtures (ADR-0043 D5)

Files for the rows of `tests/format_parity.rs` that had no fixture in the
repository: a property only counts as supported in a format once a real
file carries it. Each file is written by an original tool or derived from a
tracked file by a documented byte patch; nothing here is hand-edited.

| File | What it carries | Made by | SHA-256 |
| --- | --- | --- | --- |
| `rgb-p3-icc.jxl` | 16 x 8 RGB8, lossless, with an embedded ICC profile (`../icc/display-p3-gamma22.icc`, unchanged) instead of an enum color encoding | `cjxl` | `37862bf386c1d596ba2e7b91170262afe5f1a7e0b6054b23066b47291eadec79` |

## Inputs

`tools/make.py` (standard library only) writes the encoder inputs into a
scratch directory: the 16 x 8 RGB samples (`r = 16x`, `g = 32y`,
`b = 255 - 16x`) as `rgb-16x8.raw`. `cjxl` reads PNM, so the raw samples are
wrapped in a binary PPM header.

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
shasum -a 256 tests/fixtures/parity/*.jxl
```

Two runs produced byte-identical files. libjxl keeps the profile as an ICC
stream (`want_icc`) because it is not byte-identical to the profile libjxl
would generate for an enum encoding; the bridge reports
`ImageInfo::original_icc` for it and no enum DATA encoding.

The files are licensed like the repository's own test data; the embedded
profile keeps the provenance recorded in `../icc/README.md`.
