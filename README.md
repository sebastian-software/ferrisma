# libwebp-rs

A mechanical, behavior-preserving Rust port of [libwebp](https://chromium.googlesource.com/webm/libwebp)
v1.6.0, the WebP reference implementation: decoder, demuxer, encoder, muxer and
SharpYUV, in safe Rust except for an audited C ABI module. It is the third sister
port next to [libavif-rs](https://github.com/sebastian-software/libavif-rs) and
[libjxl-rs](https://github.com/sebastian-software/libjxl-rs), and it will supply
WebP input and output to [Ferrixel](https://github.com/sebastian-software/ferrixel).

**Status:** bootstrap only. No codec code has been translated yet; every
upstream unit has an empty Rust module waiting for its work package. See
[PORT_STATUS.md](docs/PORT_STATUS.md) and the [work plan](docs/WORKPLAN.md).

## How it works

- Each upstream C file becomes one Rust module with the same name, translated
  without redesign ([porting contract](docs/PORTING.md),
  [ADR 0001](docs/adr/0001-mechanical-port.md)).
- Every translated unit is compared with the pinned C library, built in a fixed
  reference configuration and run in separate processes
  ([ADR 0002](docs/adr/0002-differential-oracle.md)).
- `#![deny(unsafe_code)]` applies everywhere except `src/abi`, which exports the
  original C API for the unmodified upstream tools
  ([ADR 0004](docs/adr/0004-c-abi-boundary.md),
  [ADR 0005](docs/adr/0005-toolchain-and-unsafe-policy.md)).
- Ordinary builds compile and link no C and use no dependencies. The crate
  builds on stable Rust 1.95 or later.

## Building and checking

```sh
git submodule update --init
cargo build
python3 tools/check.py            # fmt, Clippy, tests in three profiles, provenance
python3 tools/check.py --oracle   # also builds the C oracle and requires it
```

[CONTRIBUTING.md](CONTRIBUTING.md) lists the prerequisites and focused checks;
[AGENTS.md](AGENTS.md) has the rules for porting sessions.

## License

BSD-3-Clause, as libwebp, with the WebM project's additional patent grant in
[PATENTS](PATENTS). [LICENSE](LICENSE), [PATENTS](PATENTS) and [AUTHORS](AUTHORS)
are unchanged copies of the upstream files; translated modules keep Google's
copyright notices ([PROVENANCE.md](docs/PROVENANCE.md)). The crate is not
published.
