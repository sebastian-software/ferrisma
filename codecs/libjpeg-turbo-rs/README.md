# libjpeg-turbo-rs

This decoder is a workspace member of the Ferrixel monorepo. Start with the
[project overview](../../README.md), [current status](../../docs/status.md), or
[codec workspace guide](../../docs/codecs.md). Its local porting contracts
and licenses still apply.

A mechanical, behavior-preserving Rust port of the JPEG decoder in
[libjpeg-turbo](https://github.com/libjpeg-turbo/libjpeg-turbo) 3.2.1, a
descendant of the Independent JPEG Group's reference libjpeg. It is maintained
alongside [libavif-rs](../libavif-rs),
[libjxl-rs](../libjxl-rs) and
[libwebp-rs](../libwebp-rs), as a candidate for Ferrixel JPEG input.

**Port status:** the in-scope 8-bit decoder and Neon, SSE2 and AVX2 kernels
are translated. The validation includes differential tests against the C
library, upstream `djpeg` bit tests and ISO/IEC 10918-2 (ITU-T T.83)
compliance data. The scope excludes JPEG encoding, 12- and 16-bit samples,
lossless JPEG, color quantization and RGB565 output; see
[ADR 0001](docs/adr/0001-mechanical-port.md) and
[ADR 0006](docs/adr/0006-safe-decoding-api.md).

**Engine integration:** Ferrixel currently uses `zune-jpeg` for `jpeg-read`;
this port is not yet wired into the engine. Its row-streaming API exposes
scaled decoding, but that capability is not yet available through the
Ferrixel JPEG adapter. See [ADR-0025](../../docs/adr/0025-codec-selection-reference-conformance-and-image-scope.md)
and the [project status](../../docs/status.md) for adoption and open work.

## Why libjpeg-turbo, and why only the decoder

Ferrixel selects codecs by reference lineage or proven conformance and measures
alternatives before choosing
([Ferrixel ADR-0025](https://github.com/sebastian-software/ferrixel/blob/main/docs/adr/0025-codec-selection-reference-conformance-and-image-scope.md)).
On an Apple M1 Ultra, single-threaded, libjpeg-turbo decoded a 12 Mpx photograph
in 39 ms, against 54 ms for `zune-jpeg` and 67–70 ms for jpegli
([measurement](https://github.com/sebastian-software/ferrixel/blob/main/docs/benchmarks/M2-jpeg-decoders.md)).
Ferrixel treats JPEG as an input format. The port therefore covers decompression only ([ADR 0001](docs/adr/0001-mechanical-port.md)).

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
- Safe Rust by default; `unsafe` only where an ADR names it. The crate serves
  Rust programs and exports no C ABI
  ([ADR 0003](docs/adr/0003-toolchain-and-unsafe-policy.md)).
- The portable C code is ported first; SIMD comes afterwards, NEON from
  upstream's C intrinsics and x86-64 as intrinsics rewritten from upstream's
  NASM assembly ([ADR 0004](docs/adr/0004-simd-after-the-portable-port.md)).
  Without default features the crate is the portable reference
  configuration ([ADR 0007](docs/adr/0007-neon-kernels-and-unsafe.md)).
- Rust programs decode through a safe row-streaming API: `jpeg::Decoder` and
  `jpeg::Scanlines` read rows with scaling, cropping and skipping, and return
  the EXIF data, the ICC profile and libjpeg's errors and warnings as values
  ([ADR 0006](docs/adr/0006-safe-decoding-api.md)).

## Building

Run from `codecs/libjpeg-turbo-rs`:

```sh
git submodule update --init upstream/libjpeg-turbo
cargo fetch --locked
cargo build -p libjpeg-turbo-rs --locked
cargo test -p libjpeg-turbo-rs --locked
```

Rust artifacts go to the monorepo root `target/`. The check script uses
offline Cargo commands, so fetch dependencies first. Tests requiring the
C oracle skip without it unless the `oracle` feature is enabled.

The differential tests need the C oracle, which needs CMake and a C compiler:

```sh
python3 tools/build_oracle.py          # build/oracle/release
python3 tools/fetch_t83.py             # build/t83, the ISO/IEC 10918-2 data
python3 tools/check.py --oracle        # all checks, bit tests and oracle tests
```

[AGENTS.md](AGENTS.md) has the rules for working on the port.

## License

The translated libjpeg API library is covered by the IJG License
([README.ijg](README.ijg)); translated SIMD code will be covered by the zlib
License. [LICENSE.md](LICENSE.md) and [README.ijg](README.ijg) are unchanged
copies of the upstream files. Translated modules keep the upstream copyright
notices.

This software is based in part on the work of the Independent JPEG Group.
