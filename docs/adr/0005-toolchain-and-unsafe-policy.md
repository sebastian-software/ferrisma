# Build on stable Rust 1.95 with unsafe code denied outside the C ABI

- Status: accepted
- Created: 2026-11-05
- Updated: 2026-11-07

## Context

Ferrixel's dependency gates require a safe Rust API and a stable toolchain.
libjxl-rs uses `rust-version = "1.95"` and edition 2024 and allows `unsafe`
inside the codec with local safety arguments. VP8 and VP8L translate into safe
Rust (as `image-webp` shows), so libwebp-rs can be stricter.

## Decision

- `rust-version = "1.95"`, edition 2024, stable toolchain
  (`rust-toolchain.toml` selects `stable` with rustfmt and Clippy). CI compiles
  and tests on 1.95 in a separate job. Raise the minimum deliberately, in its
  own change.
- `src/lib.rs` declares `#![deny(unsafe_code)]`. The only exception is
  `#[allow(unsafe_code, reason = ...)]` on `pub mod abi`
  ([ADR 0004](0004-c-abi-boundary.md)). The crate cannot use `forbid`, as the
  port plan proposed, because `forbid` cannot be relaxed for the ABI module.
- Relaxing the policy for any other module needs a new ADR that names the
  modules, the operations and the local safety arguments. The wave 4 SIMD
  packages (WP18 to WP20) need such an ADR before they start: `std::arch`
  intrinsics and `#[target_feature]` functions are `unsafe`.
- `src/lib.rs` also declares `#![warn(clippy::all, clippy::pedantic)]`, and
  `tools/check.py` runs Clippy with `-D warnings` on all targets. In translated
  code, keep the source's form and add a function- or item-scoped
  `#[expect(clippy::<lint>, reason = "...")]` that names the C construct, for
  example a truncating cast or a C-style index loop. Use `expect`, not `allow`,
  so that a stale exception fails the check. Module- or crate-wide allowances
  need an amendment of this record; WP0 fixed the only ones: the
  `module_inception` expectation on `dsp::dsp`, `demux::demux` and
  `sharpyuv::sharpyuv`, and `dead_code` in the shared test helper
  `tests/support/oracle.rs`.
- No runtime dependencies. Development dependencies need an amendment of this
  record that names them; there are none.
- The fuzz targets live in `fuzz/`, a separate crate (amended 2026-11-07, WP12;
  WORKPLAN decision 6). It is not a member of the library's workspace (it
  declares its own) and never a dependency of the library. Its only dependency
  besides the library is `libfuzzer-sys` 0.4, which re-exports `arbitrary` for
  structured inputs. cargo-fuzz builds it with a nightly toolchain
  (`cargo +nightly fuzz build`); the library's checks and its stable and MSRV
  builds do not compile it. WP12 created it with `dec_fuzzer`,
  `simple_api_fuzzer`, `advanced_api_fuzzer` and `huffman_fuzzer`, ported from
  `upstream/libwebp/tests/fuzzer/`; WP13, WP14 and WP15 add their targets.

## Consequences

The library builds with a plain `cargo build` on stable Rust and contains no
`unsafe` outside five ABI files. Fuzzing needs nightly and `libfuzzer-sys`, but
only in the separate `fuzz/` crate. Some C constructs need an `#[expect]` line;
the pedantic lints still catch untranslated mistakes elsewhere.

## Review triggers

Revisit before wave 4 starts, when a unit cannot be expressed in safe Rust,
or when the minimum Rust version is raised.
