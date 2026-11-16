# AVIF references above 8 bits, written by the C original (2026-11-15)

The byte reference of the AVIF writer's deep rows (ADR-0021 amendment of
2026-11-15, owner decision: the output depth follows the input;
`ferrisma_avif_write::tests::deep_output_equals_the_c_original`). Every
`.avif` here was written by `encode_deep.c`, which makes the writer's
libavif calls in its order and with its values (avifImageCreate at the
depth, CICP 1/13/6 full range, an RGBA `avifRGBImage` at the same depth with
`uint16_t` samples, avifImageRGBToYUV, avifEncoderWrite with AOM, quality,
qualityAlpha, speed 6, one thread, no automatic tiling), linked to:

- the pinned C libavif 1.4.2 (`codecs/libavif-rs/upstream/libavif`, revision
  `9b07a9d7`), CMake Release, `-ffp-contract=off`, `AVIF_CODEC_AOM=SYSTEM`,
  no dav1d, libyuv or libsharpyuv (none is on the encode path of these
  calls), the `avif` target (`libavif_internal.a`);
- the portable libaom reference of the port's gates
  (`codecs/libavif-rs/tools/build_aom.py --library-only`:
  `AOM_TARGET_CPU=generic`, `-ffp-contract=off`, Release, `_c` functions
  only), AppleClang 21 on macOS AArch64.

The input is the `rows16.py` pattern at 67x45 (odd, partial superblocks),
little-endian `uint16` RGBA. The Rust writer (the port with its run-time
NEON dispatch) wrote the same bytes for all five. Never refreshed from the
port's output.

```sh
python3 -I rows16.py 67 45 10 0 d10-444-q60.rgba16
encode_deep d10-444-q60.rgba16 d10-444-q60.avif 67 45 10 444 0 60 6
```

| File | Depth | Chroma | Alpha | Quality | Bytes | SHA-256 |
| --- | ---: | --- | --- | ---: | ---: | --- |
| `d10-444-q60.avif` | 10 | 4:4:4 | no | 60 | 974 | `41780bd01a3b93640e8e869f295c30e32201fa7bcc966a1f79e9648ebd9d1761` |
| `d10-420-alpha-q60.avif` | 10 | 4:2:0 | yes | 60 | 1,215 | `877b7dd4989661d742da26ef4fb648070ed19292ce8b4fd59631db62ad10281d` |
| `d10-444-q100.avif` | 10 | 4:4:4 | no | 100 | 8,140 | `12edeae3230c90749220aa695452ede0ae98d9392ceec24f392573259548f3f5` |
| `d12-444-alpha-q60.avif` | 12 | 4:4:4 | yes | 60 | 690 | `afa5febe67977707e28a4d4d75921c9ed5312afa55931cd91967abb81dfe0d55` |
| `d12-420-q60.avif` | 12 | 4:2:0 | no | 60 | 406 | `d7fa84822b0443cfe458431b4e32488b3d61718e1d7e2df386d3a14a5c5896ca` |
