# ferrisma-avif-read

A narrow, safe AVIF reader over `libavif-rs`, the Ferrisma workspace's
mechanical Rust port of libavif (with its libaom and dav1d translations).
It is the backend of the `avif-read` feature of
[`ferrisma`](https://crates.io/crates/ferrisma) and is not meant to be used on
its own.

- The port's raw C-shaped API stays behind this crate; callers see safe
  functions only.
- Caller-owned RGB output buffers are bounded. Codec-owned scratch memory
  stays opaque and is not bounded by Ferrisma's memory budget.
- It needs the nightly toolchain pinned in the repository's
  `rust-toolchain.toml` and builds only for macOS ARM64 and GNU Linux
  x86-64, the targets the generated libavif code supports
  ([ADR-0026](https://github.com/sebastian-software/ferrisma/blob/main/docs/adr/0026-codecs-in-one-repository-and-workspace.md)).
- A dependent's build profile must keep `panic = "unwind"` (Cargo's default;
  `panic = "abort"` is a compile error). Turning overflow checks off for the
  four AVIF packages is recommended; see
  [publishing](https://github.com/sebastian-software/ferrisma/blob/main/docs/publishing.md),
  blocker 3.

Experimental and unstable: its API may change in any 0.x release.

License: MIT OR Apache-2.0 (`LICENSE-MIT`, `LICENSE-APACHE`). The ported
codecs carry their own licenses.
