# libwebp-rs

This port is a workspace member of the Ferrisma monorepo. Start with the
[project overview](../../README.md), [current status](../../docs/status.md), or
[codec workspace guide](../../docs/codecs.md). Its local porting contracts
and licenses still apply.

A mechanical, behavior-preserving Rust port of [libwebp](https://chromium.googlesource.com/webm/libwebp)
v1.6.0, the WebP reference implementation: decoder, demuxer, animation
decoder, encoder, muxer, animation encoder and SharpYUV, in safe Rust except
for an audited C ABI module and the SIMD modules. It is maintained alongside [libjpeg-turbo-rs](../libjpeg-turbo-rs),
[libavif-rs](../libavif-rs), and [libjxl-rs](../libjxl-rs).

**Port status:** the portable decoder, encoder, container, animation and
SharpYUV translations have differential evidence against libwebp v1.6.0.
The C ABI is used to run upstream tools; safe Rust APIs cover decoding and
encoding. SIMD implementations and their exceptions are documented in
[ADR 0008](docs/adr/0008-simd-modules.md): the scalar build matches libwebp's
scalar build and the SIMD build matches libwebp's SIMD build at the same
instruction-set level ([SIMD status](docs/SIMD-STATUS.md)). Each module records its deviations
and excluded undefined behavior.

**Engine integration:** Ferrisma does not currently provide WebP input or
output adapters. The port's APIs are the intended adoption boundary, subject
to the gates in [ADR 0010](docs/adr/0010-ferrisma-adoption.md); see the
[project status](../../docs/status.md) for current integration work.

## Using the library

The safe API is `webp::api`:

- `webp::api::decode` ([ADR 0007](docs/adr/0007-safe-decoder-api.md)):
  `Features::parse` probes a header; `Config` with cropping, scaling, color
  mode, flipping, dithering and threads; `decode` into library memory or
  `decode_into` caller rows or planes; `IncrementalDecoder` for data that
  arrives in pieces; `Container` for the `ICCP`, `EXIF` and `XMP` chunks and
  the animation parameters; `AnimationDecoder` for the frames of an animation.
- `webp::api::encode` ([ADR 0006](docs/adr/0006-safe-encoder-api.md)):
  `Config` with presets and every encoder option; `Picture` from RGB or RGBA
  rows or from YUVA planes; `Encoder` with progress and abort hooks and output
  through `Vec<u8>` or a writer closure; `encode_lossy` and `encode_lossless`.

The differential suite compares results with libwebp v1.6.0 for its tested
inputs and configurations; it is not proof for every possible input. The
translated modules (`webp::dec`, `webp::enc`, `webp::dsp`, ...) are public but
mirror the C library's shapes; `webp::abi` exports the C API for the upstream
tools and is not meant for Rust callers. What the library promises Ferrisma is
[ADR 0010](docs/adr/0010-ferrisma-adoption.md).

## How it works

- Each upstream C file is one Rust module with the same name, translated
  without redesign ([porting contract](docs/PORTING.md),
  [ADR 0001](docs/adr/0001-mechanical-port.md), [ADR 0003](docs/adr/0003-rust-representation.md)).
- Every unit is compared with the pinned C library, built in a fixed reference
  configuration and run in separate processes
  ([ADR 0002](docs/adr/0002-differential-oracle.md)); upstream's own test
  scripts run with its unmodified tools linked against this library.
- `#![deny(unsafe_code)]` applies everywhere except `src/abi`, which exports
  the C API ([ADR 0004](docs/adr/0004-c-abi-boundary.md),
  [ADR 0005](docs/adr/0005-toolchain-and-unsafe-policy.md)), and the SIMD
  modules, which may load and store vectors and call target-feature functions
  ([ADR 0008](docs/adr/0008-simd-modules.md)).
- Ordinary builds compile and link no C. The only library dependency is the
  workspace's small, safe `libsharpyuv-core` conversion crate, shared with
  AVIF through a separate adapter ([ADR-0033](../../docs/adr/0033-shared-sharpyuv-conversion-core.md)). The crate
  builds on stable Rust 1.95 or later. The `simd` feature (default) compiles
  the SIMD modules and selects them at run time; `--no-default-features` is
  the scalar translation.
- The work was delivered in packages by parallel sessions, and the records
  live with the code ([ADR 0009](docs/adr/0009-work-packages-and-integration.md)).

## Building and checking

Run from `codecs/libwebp-rs`:

```sh
git submodule update --init upstream/libwebp upstream/libwebp-test-data
cargo fetch --locked
cargo build -p libwebp-rs --locked
python3 tools/check.py            # fmt, Clippy, tests in three profiles, provenance
python3 tools/check.py --oracle   # also builds the C oracle and requires it
```

The check script uses offline Cargo commands after the fetch; Rust artifacts
are in the monorepo root `target/`. Tests needing an oracle skip without it
unless the `oracle` feature is enabled.

[CONTRIBUTING.md](CONTRIBUTING.md) lists the prerequisites and focused checks;
[AGENTS.md](AGENTS.md) has the rules for porting sessions.

## License

BSD-3-Clause, as libwebp, with the WebM project's additional patent grant in
[PATENTS](PATENTS). [LICENSE](LICENSE), [PATENTS](PATENTS) and [AUTHORS](AUTHORS)
are unchanged copies of the upstream files; translated modules keep Google's
copyright notices ([PROVENANCE.md](docs/PROVENANCE.md)). The crate is not
published.
