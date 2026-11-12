# libwebp-rs fuzz targets

This is the WebP port in the Ferrisma monorepo. See the [project overview](../../../README.md),
[status](../../../docs/status.md), and [codec workspace guide](../../../docs/codecs.md).

cargo-fuzz targets that exercise the Rust translation, ported from the
upstream fuzzers in `upstream/libwebp/tests/fuzzer/` (Apache License 2.0; each
target keeps its upstream notice). This crate is separate from the library: it
is not a member of the library's workspace, it is never a dependency of the
library, and its only dependency besides the library is `libfuzzer-sys`
([ADR 0005](../docs/adr/0005-toolchain-and-unsafe-policy.md)).

| Target | Upstream source | What it calls |
| --- | --- | --- |
| `dec_fuzzer` | `dec_fuzzer.cc` | `WebPDecode` with the default configuration |
| `simple_api_fuzzer` | `simple_api_fuzzer.cc` | `WebPDecodeRGBA` and friends, the `...Into` functions |
| `advanced_api_fuzzer` | `advanced_api_fuzzer.cc` | `WebPDecode` and the incremental decoder with fuzzed options |
| `huffman_fuzzer` | `huffman_fuzzer.cc` | `ReadHuffmanCodesHelper` of the lossless decoder |
| `enc_fuzzer` | `enc_fuzzer.cc` | `WebPEncode` with a fuzzed configuration on a WebP file (imageio's other readers have no Rust counterpart), cropped or scaled, then decoding and the exactness check |
| `enc_dec_fuzzer` | `enc_dec_fuzzer.cc` | `WebPEncode` with a fuzzed configuration on the three built-in pictures, cropped or scaled, then `WebPDecode` and the exactness check |
| `animdecoder_fuzzer` | `animdecoder_fuzzer.cc` | `WebPAnimDecoderNew` with the default options, then `WebPAnimDecoderGetNext` for every frame, after upstream's canvas-size limits |
| `mux_demux_api_fuzzer` | `mux_demux_api_fuzzer.cc` | `WebPMuxCreate` (copying or not) and the mux getters, or `WebPDemux`/`WebPDemuxPartial` and the chunk and frame iterators, on arbitrary bytes |
| `animation_api_fuzzer` | `animation_api_fuzzer.cc` | `WebPAnimDecoderNew` with fuzzed threading and color mode, then up to three frames, after upstream's canvas- and frame-size limits |
| `animencoder_fuzzer` | `animencoder_fuzzer.cc` | `WebPAnimEncoderNew` with fuzzed key-frame options and up to 15 built-in pictures, cropped, scaled and rescaled, each added with a fuzzed configuration and timestamp step, then `WebPAnimEncoderAssemble` |

cargo-fuzz needs a nightly toolchain. From the WebP component directory (`codecs/libwebp-rs`), after initializing
its pinned submodules:

```sh
cargo +nightly fuzz build
cargo +nightly fuzz run dec_fuzzer -- -dict=upstream/libwebp/tests/fuzzer/fuzz.dict
```

Seed the corpus with the test vectors, for example
`mkdir -p fuzz/corpus/dec_fuzzer && cp upstream/libwebp-test-data/*.webp fuzz/corpus/dec_fuzzer/`.

## Deviations from upstream

The targets translate upstream's fuzzers with these changes:

1. **Decoder targets.** FuzzTest domains become one `arbitrary` tuple (the
   colorspace is taken modulo `MODE_LAST`), and `kFuzzPxLimit` keeps its value
   without sanitizers. The targets keep upstream's names, which nightly cargo's
   `non_kebab_case_bins` lint warns about.
2. **Encoder targets.** The FuzzTest domains become one `arbitrary` tuple; the
   24 values of `ArbitraryWebPConfig` pick their field's range by modulo. The
   optimization index is dropped (SIMD: none in the reference configuration,
   `tests/wp13_encode.rs`, `# Checks not applicable`, item 2). imageio's PNG,
   JPEG, TIFF and PNM readers have no Rust counterpart, so `enc_fuzzer` reads
   WebP input only, with a port of `ReadWebP` that decodes into the library's
   memory and copies the samples into the picture's planes, where C decodes
   into them. The source images of `img_*.h` are read from upstream's headers
   at compile time. The targets keep upstream's names (cargo's
   `non_kebab_case_bins` warning).
3. **Animation decoder target.** The FuzzTest domain is libFuzzer's input; the
   target keeps upstream's name (cargo's `non_kebab_case_bins` warning).
4. **Muxer and animation targets.** The FuzzTest domains are `arbitrary`
   tuples (modulo for the ranges); `animencoder_fuzzer` drops the unused input
   string and the SIMD optimization index, returns on an empty frame list and
   ignores frames past 15; the targets keep upstream's names (cargo's
   `non_kebab_case_bins` warning).
