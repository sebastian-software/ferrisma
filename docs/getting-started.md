# Getting started

Ferrisma is an experimental Rust image pipeline. Its M1 release
and audit gates remain open; see [current status](status.md).

## Build the CLI

Clone the repository and run from its root. The root toolchain file selects the
nightly needed by the workspace's generated AVIF code. On x86-64, the AVIF
path needs NASM.

```sh
git clone https://github.com/sebastian-software/ferrisma.git
cd ferrisma
cargo run --release --features cli -- \
  photo.jpg scaled.avif 1600 900 --assume-srgb --quality=80 --speed=8

cargo run --release --features cli -- \
  photo.jpg scaled.webp 1600 900 --assume-srgb --quality=80 --speed=8

cargo run --release --features cli -- \
  drawing.webp resized.webp 800 600 --assume-srgb --webp-lossless --webp-level=6
```

The `cli` feature enables JPEG, still WebP, JPEG XL and AVIF input, AVIF and
still WebP output, and ICC support. The default AVIF encoder is `rav1e`;
`--avif-encoder=libavif --avif-depth=8` selects the workspace AOM encoder
through `avif-own-write` (depth defaults to 10; 8 and 10 are supported).
Rust callers use `AvifEncoder::Libavif { depth: 8 }` in `TransformOptions`.
The separate [`ferrisma-libavif` adapter](../crates/ferrisma-libavif/README.md)
delegates to the same implementation. Quality and speed use each encoder's own
scale; their numeric values do not establish equivalent image quality.
The current `avif-read` uses the same
workspace port's AOM decoder through an isolated safe wrapper, so the former
`rav1d`/dav1d symbol collision is gone. Own AVIF reading and writing require the pinned
nightly and a supported native target (macOS ARM64 or GNU Linux x86-64).
The codec-free core and features without `avif-read`, `avif-own-write`, or `cli` retain stable Rust 1.98
support. See the [measured AVIF input trade-off](benchmarks/M2-avif-input.md).

WebP lossy uses its own quality and speed settings. Lossless uses only
`--webp-level=0..9`; `--quality`, `--speed`, `--chroma`, and multiple encoder
threads are unsupported for that output. Still WebP output embeds an sRGB ICC
profile for the rendered samples and drops source EXIF orientation and XMP.
`--max-output-mb` caps the retained file; the WebP writer checks it during
encoding, while AVIF and external backends check after encoding.

JPEG XL input uses `jxl-oxide` by default. To inspect the experimental local
reference decoder, add `jpeg-xl-own-read` to the Cargo features and select it
explicitly:

```sh
cargo run --release --features cli,jpeg-xl-own-read -- \
  photo.jxl scaled.avif 1600 900 --use-rendered-icc --jxl-decoder=libjxl-rs
```

This decoder budgets a complete F32 input frame for every request, including
crops. It remains opt-in after the [measured photo regression](benchmarks/M2-jxl-input-switch.md).

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
