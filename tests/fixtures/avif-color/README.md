# AVIF with an ICC profile and an nclx beside it

Two 16 x 16 AVIF stills that carry both a `colr` of type `prof` (an sRGB
ICC profile) and a `colr` of type `nclx`. They test the engine's policy for
CICP evidence beside a rendered ICC profile (ADR-0015 rule 4, ADR-0016,
review of 2026-11-12). MIAF gives the ICC profile precedence for color
interpretation; the engine does not apply that precedence silently.

| File | `nclx` (primaries/transfer/matrix, full range) | Expected under `UseRenderedIcc` | SHA-256 |
| --- | --- | --- | --- |
| `icc-nclx-1-13-6.avif` | 1/13/6, full | converts: both signals say sRGB and agree | `21c2e3c3048f2239fe8e755aef9c0cfa1d503c736f350e02c4a0d09ab2685434` |
| `icc-nclx-1-1-6.avif` | 1/1/6, full (BT.709 transfer) | `UnsupportedColor`, `CicpBesideIcc` naming the reason (pending owner confirmation) | `ec7a00ca5d2bd9c40b486df43a69a625d58c6c0d3509a3e4ad84911291ca8dff` |

Both files embed `../icc/paris-srgb.icc` unchanged (596 bytes, SHA-256
`33c34ecfeb998459e0075ba26a0f1ea5042e20e2419c5962e495eba3208576f2`), as
`tools/colr.py` confirms. The pixels are a 16 x 16 crop at (100, 100) of
`../decoder-work/paris-420.jpg`, which derives from libavif's tracked test
photograph `paris_exif_xmp_icc.jpg` (see `../decoder-work/README.md`); the
files inherit that photograph's terms in the libavif repository.

## Regeneration

Written on 2026-11-12 with ImageMagick 7.1.2-35 and `heif-enc` from libheif
1.23.7 with the aom 3.15.3 AV1 encoder. `heif-enc` writes both boxes only with
`--enable-two-colr-boxes`. ImageMagick did not keep the profile in the PNG, so
`tools/tag_png.py` inserts it as an `iCCP` chunk. From the repository root:

```sh
out=$(mktemp -d)
magick tests/fixtures/decoder-work/paris-420.jpg -crop 16x16+100+100 +repage -strip "$out/in.png"
python3 -I tests/fixtures/avif-color/tools/tag_png.py "$out/in.png" tests/fixtures/icc/paris-srgb.icc "$out/tagged.png"
for t in 13 1; do
  heif-enc --enable-two-colr-boxes --colour_primaries 1 --transfer_characteristic "$t" \
    --matrix_coefficients 6 --full_range_flag 1 -q 90 \
    -o "tests/fixtures/avif-color/icc-nclx-1-$t-6.avif" "$out/tagged.png"
done
python3 -I tests/fixtures/avif-color/tools/colr.py tests/fixtures/avif-color/*.avif
shasum -a 256 tests/fixtures/avif-color/*.avif
```

Two runs on the development host produced byte-identical files. Other
encoder versions may produce different bytes. The tests depend only on the
`colr` boxes, which `tools/colr.py` prints:

```text
icc-nclx-1-13-6.avif: prof 596 bytes sha256 33c34ecf...
icc-nclx-1-13-6.avif: nclx 1/13/6 full_range=1
icc-nclx-1-1-6.avif: prof 596 bytes sha256 33c34ecf...
icc-nclx-1-1-6.avif: nclx 1/1/6 full_range=1
```
