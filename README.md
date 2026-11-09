# libjpeg-turbo-rs

A mechanical, behavior-preserving Rust port of the JPEG decoder in
[libjpeg-turbo](https://github.com/libjpeg-turbo/libjpeg-turbo) 3.2.1, the
fastest descendant of the Independent JPEG Group's reference libjpeg. It is the
fourth sister port next to
[libavif-rs](https://github.com/sebastian-software/libavif-rs),
[libjxl-rs](https://github.com/sebastian-software/libjxl-rs) and
[libwebp-rs](https://github.com/sebastian-software/libwebp-rs), and will supply
JPEG input, including scaled decoding for shrink-on-load, to
[Ferrixel](https://github.com/sebastian-software/ferrixel).

**Status:** the decoder is translated and passes upstream's in-scope bit
tests. The C oracle, the
probe harness, the bit-test inventory and the module skeleton exist; work
package 1 has translated the types, error codes and messages, the memory
manager and the utilities, work package 2 the input side up to
`jpeg_read_header`, work package 3 the Huffman, progressive Huffman and
arithmetic decoders, work package 4 the coefficient buffer controller and
every inverse DCT, work package 5 the upsamplers, the color conversion
and the main and postprocessing controllers, work package 6 the master
control and the decompression API (`jpeg_start_decompress`,
`jpeg_read_scanlines`, `jpeg_skip_scanlines`, `jpeg_crop_scanline`, raw data
and buffered-image mode), and work package 7 runs upstream's 30 in-scope
`djpeg` bit tests on the port through a Rust driver, with upstream's MD5
sums. Work package 8 adds the safe Rust API: `jpeg::Decoder` and
`jpeg::Scanlines` stream rows with scaling, cropping and skipping, and return
the EXIF data, the ICC profile and libjpeg's errors and warnings as values
([ADR 0006](docs/adr/0006-safe-decoding-api.md)). Work package 9 translates
upstream's Neon kernels for AArch64 behind the default `simd` feature, with
unsafe code only in the loads, stores and kernel calls
([ADR 0007](docs/adr/0007-neon-kernels-and-unsafe.md)); without default
features the crate is the portable reference configuration. Work package 10
rewrites the x86-64 SSE2 and AVX2 kernels from upstream's NASM assembly as
intrinsics, with the same unsafe boundaries
([ADR 0008](docs/adr/0008-x86-64-kernels-from-nasm.md)). Work package 11
runs the ISO/IEC 10918-2 (ITU-T T.83) compliance data for the 8-bit
baseline, extended and progressive processes: every stream decodes to the
coded coefficients bit for bit, and the decoder test's measured values are
recorded. The port serves Rust programs and exports no C ABI. See the
[port plan](docs/PORT_PLAN.md) for the scope and the work packages, and the
[port status](docs/PORT_STATUS.md) for what is verified.

## Why libjpeg-turbo, and why only the decoder

Ferrixel selects codecs by reference lineage or proven conformance and measures
alternatives before choosing
([Ferrixel ADR-0025](https://github.com/sebastian-software/ferrixel/blob/main/docs/adr/0025-codec-selection-reference-conformance-and-image-scope.md)).
On an Apple M1 Ultra, single-threaded, libjpeg-turbo decoded a 12 Mpx photograph
in 39 ms, against 54 ms for `zune-jpeg` and 67–70 ms for jpegli
([measurement](https://github.com/sebastian-software/ferrixel/blob/main/docs/benchmarks/M2-jpeg-decoders.md)).
Ferrixel writes AVIF, WebP and JPEG XL, so JPEG matters as input. The port
therefore covers decompression only ([ADR 0001](docs/adr/0001-mechanical-port.md)).

## How it works

- Each upstream C file in scope becomes one Rust module, translated without
  redesign ([ADR 0001](docs/adr/0001-mechanical-port.md)).
- Every translated unit is compared with the pinned C library in a fixed
  reference configuration, in separate processes, and upstream's own `djpeg`
  bit tests run on the port through a Rust driver
  ([ADR 0002](docs/adr/0002-differential-oracle.md)).
- The ISO/IEC 10918-2 compliance data, which ITU publishes but does not
  license for redistribution, is fetched and verified at test time
  ([ADR 0002](docs/adr/0002-differential-oracle.md)).
- Safe Rust by default; `unsafe` only where an ADR names it
  ([ADR 0003](docs/adr/0003-toolchain-and-unsafe-policy.md)).
- The portable C code is ported first; SIMD comes afterwards, NEON from
  upstream's C intrinsics and x86-64 as intrinsics rewritten from upstream's
  NASM assembly ([ADR 0004](docs/adr/0004-simd-after-the-portable-port.md)).

## Building

```sh
git submodule update --init
cargo build
cargo test
```

The differential tests need the C oracle, which needs CMake and a C compiler:

```sh
python3 tools/build_oracle.py          # build/oracle/release
python3 tools/fetch_t83.py             # build/t83, the ISO/IEC 10918-2 data
python3 tools/check.py --oracle        # all checks, bit tests and oracle tests
```

[AGENTS.md](AGENTS.md) has the rules for porting sessions.

## License

The translated libjpeg API library is covered by the IJG License
([README.ijg](README.ijg)); translated SIMD code will be covered by the zlib
License. [LICENSE.md](LICENSE.md) and [README.ijg](README.ijg) are unchanged
copies of the upstream files. Translated modules keep the upstream copyright
notices.

This software is based in part on the work of the Independent JPEG Group.
