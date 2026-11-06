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

**Status:** tooling only. The C oracle, the probe harness, the bit-test
inventory and the module skeleton exist; no upstream unit has been translated
yet. See the [port plan](docs/PORT_PLAN.md) for the scope and the work
packages, and the [port status](docs/PORT_STATUS.md) for what is verified.

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
  bit tests run against the port
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
