# Getting started

Ferrixel is an experimental, unpublished Rust image pipeline. Its M1 release
and audit gates remain open; see [current status](status.md).

## Build the CLI

Clone the repository and run from its root. The root toolchain file selects the
nightly needed by the workspace's generated AVIF code. On x86-64, the AVIF
path needs NASM.

```sh
git clone https://github.com/sebastian-software/ferrixel.git
cd ferrixel
cargo run --release --features cli -- \
  photo.jpg scaled.avif 1600 900 --assume-srgb --quality=80 --speed=8

cargo run --release --features cli -- \
  photo.jpg scaled.webp 1600 900 --assume-srgb --quality=80 --speed=8

cargo run --release --features cli -- \
  drawing.webp resized.webp 800 600 --assume-srgb --webp-lossless --webp-level=6
```

The `cli` feature enables JPEG, still WebP, JPEG XL and AVIF input, AVIF and still WebP output, and ICC
support. The default AVIF encoder is `rav1e`. The separate
[`ferrixel-libavif` adapter](../crates/ferrixel-libavif/README.md) provides an
experimental alternative; do not combine it with the interim AVIF reader in one
build because they export conflicting dav1d symbols.

WebP lossy uses its own quality and speed settings. Lossless uses only
`--webp-level=0..9`; `--quality`, `--speed`, `--chroma`, and multiple encoder
threads are unsupported for that output. Still WebP output embeds an sRGB ICC
profile for the rendered samples and drops source EXIF orientation and XMP.
`--max-output-mb` caps the retained file; the WebP writer checks it during
encoding, while AVIF and external backends check after encoding.

Use `--assume-srgb` only when you accept that assumption for the input. For a
supported rendered RGB ICC profile, `--use-rendered-icc` selects the explicit
ICC path. Untagged color is not automatically sRGB; unsupported HDR and
ambiguous color are rejected. Orientation is applied before crop coordinates;
JPEG and lossless WebP codec internals, as well as AVIF input, may require
full-frame work. The AVIF and WebP encoders materialize output frames. See the
[color contract](design/color-management-contract.md)
and [architecture](architecture.md).

For codec source setup and the contributor checks, see [codec development](codecs.md)
and [validation](validation.md).
