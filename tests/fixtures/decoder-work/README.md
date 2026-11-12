# Decoder-work fixtures

Small inputs for the decoder-work and animation tests (issues #19 and #68,
ADR-0012, ADR-0019). All derive from libavif's tracked test photograph
`codecs/libavif-rs/upstream/libavif/tests/data/paris_exif_xmp_icc.jpg`
(SHA-256 `a827613b7d2b44b62547a83452d5399f2f42d96b88738032a5fde933cf31263c`)
or from flat colors, and were written with ImageMagick 7.1.2-35 (libwebp
1.6.0) and `img2webp` 1.6.0 on 2026-11-12.

| file | SHA-256 | command |
| --- | --- | --- |
| `paris-420.jpg` | `fc4cc9c9401243aee4956fe4bfd54453194211171b80ea947b6f2b67bcdf8aba` | `magick paris_exif_xmp_icc.jpg -strip -sampling-factor 2x2 -quality 85 paris-420.jpg` (403×302, 4:2:0 baseline, iMCU 16×16, no ICC or EXIF) |
| `paris-o5-lossy.webp` | `b2d3eeea7d7378938dc0b4894beb4130316d33054c22b1a98cdc71a3cce12b9b` | `magick paris_exif_orientation_5.jpg -resize 96x72 -quality 80 paris-o5-lossy.webp` (96×72 stored, VP8, EXIF orientation 5 kept from the source) |
| `paris-o5-lossless.webp` | `6dd17abeaf0ccdf8a087c71b9b7f5a0d10f0470f2454bf8d64036c10ba39915d` | `magick paris_exif_orientation_5.jpg -resize 96x72 -define webp:lossless=true paris-o5-lossless.webp` (VP8L, EXIF orientation 5) |
| `animated.webp` | `198692be7b7473df0ad7d413718ae1dc406342f4e9f1c4d96966541b4520309e` | `magick -size 8x8 xc:red f1.png; magick -size 8x8 xc:blue f2.png; img2webp -lossless f1.png -d 100 f2.png -o animated.webp` (two frames) |

`paris_exif_orientation_5.jpg` is the libavif test file next to
`paris_exif_xmp_icc.jpg`. The tests pin counts derived from the codec
behavior (iMCU alignment, VP8/VP8L row order), not encoder-specific pixels.
