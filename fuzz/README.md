# libwebp-rs fuzz targets

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

cargo-fuzz needs a nightly toolchain. From the repository root:

```sh
cargo +nightly fuzz build
cargo +nightly fuzz run dec_fuzzer -- -dict=upstream/libwebp/tests/fuzzer/fuzz.dict
```

Seed the corpus with the test vectors, for example
`mkdir -p fuzz/corpus/dec_fuzzer && cp upstream/libwebp-test-data/*.webp fuzz/corpus/dec_fuzzer/`.
WP13, WP14 and WP15 add their targets here.
