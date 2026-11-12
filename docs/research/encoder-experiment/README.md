# M2 work package 2: encoder experiment

Evidence for work package 2 of [M2](../../milestones/M2-performance-architecture.md): how fast the AVIF encoder can run at the **same perceptual quality** as the encoder configuration Ferrisma ships, and what each knob costs in bytes.

The crate is standalone (its own `Cargo.toml` and lockfile, an empty `[workspace]`), never shipped, and has no dependency on the `ferrisma` crate. It pins the encoder crates Ferrisma uses (`ravif` 0.13.0, `rav1e` 0.8.1, `avif-serialize` 0.8.9) and adds `avif-decode` 3.0.0 and `ssimulacra2` 0.5 for scoring.

## Method

- **Inputs** are real photographs, decoded and area-downscaled in the sRGB domain to the 1600-pixel width the baseline harness uses, or kept at native size for the 768×512 Kodak images. The downscale only produces realistic encoder input; it is not Ferrisma's color-managed resize.
- **Anchor** is what Ferrisma ships: `ravif` speed 6, one thread, quality 80, 4:4:4, 10-bit, BT.601 full range.
- **Quality matching.** For every other variant the quality knob is bisected (up to nine encodes) until the SSIMULACRA2 score brackets the anchor's score; bytes are interpolated linearly in score between the bracketing encodes. Time is the median of three encodes at the bracketing quality nearest to the anchor score.
- **Direct `rav1e`** variants drive `rav1e` with `ravif`'s quantizer mapping and speed tweaks copied from `ravif` 0.13.0 (BSD-3-Clause), so that chroma subsampling and bit depth are the only differences to `ravif`. 4:2:0 chroma is the 2×2 average of unrounded BT.601 YCbCr; the container is written by `avif-serialize` with profile 0.
- **Builds.** `asm` and `threading` are compile-time features of `rav1e`, so the experiment is built twice: once without them (Ferrisma's current build) and once with both. The anchor runs in both builds; identical FNV-1a hashes prove the assembly path is bit-exact.

## Run

```sh
cd docs/research/encoder-experiment
cargo build --release && cp target/release/ferrisma-encoder-experiment target/release/encoder-scalar
cargo build --release --features asm,threading && cp target/release/ferrisma-encoder-experiment target/release/encoder-asm
IMAGES="dog=PATH/dog_exif_extended_xmp_icc.jpg@1600 flower=PATH/flower.png.im_q85_420.jpg@1600 \
  kodim03=PATH/kodim03_yuv420_8bpc.y4m kodim23=PATH/kodim23_yuv420_8bpc.y4m"
target/release/encoder-scalar anchor $IMAGES
target/release/encoder-asm anchor $IMAGES
target/release/encoder-asm matrix $IMAGES
target/release/encoder-asm sink $IMAGES   # candidates for Ferrisma's own rav1e sink
```

In zsh, write `${=IMAGES}` so the list splits into arguments.

## Images

| Name | Source | License | Content |
| --- | --- | --- | --- |
| dog | libavif `tests/data/dog_exif_extended_xmp_icc.jpg` | libavif test data (BSD-2-Clause repository) | 4032×3024 phone photo with EXIF orientation and ICC; orientation and profile are ignored here |
| flower | libjxl `tests/data/decode-test/flower.png.im_q85_420.jpg` | CC-BY-4.0 (libjxl test data) | 2268×1512 photo, JPEG q85 4:2:0 |
| kodim03, kodim23 | libavif `tests/data/kodim03_yuv420_8bpc.y4m`, `kodim23_yuv420_8bpc.y4m` | Kodak, released for unrestricted usage | 768×512, 8-bit 4:2:0 full range |
| plasma | `scripts/bench/make-corpus.sh` 12 Mpx output | generated | synthetic plasma noise, for continuity with the harness only |

The images are not committed; the paths refer to the sister ports' checkouts.

## Results

See `results/`.
