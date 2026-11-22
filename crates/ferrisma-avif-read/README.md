# ferrisma-avif-read

A narrow, safe AVIF reader over `ferrisma-libavif`, the Ferrisma workspace's
mechanical Rust port of libavif (with its libaom and dav1d translations).
It is the backend of the `avif-read` feature of
[`ferrisma`](https://crates.io/crates/ferrisma) and is not meant to be used on
its own.

- The port's raw C-shaped API stays behind this crate; callers see safe
  functions only.
- The AV1 decoder is chosen explicitly, never by libavif's `AUTO`: the dav1d
  port by default since 2026-11-22 (owner decision), the libaom port, the
  AV1 reference decoder, on request (`Codec::Libaom`). A test decodes every
  AVIF file of the repository with both and requires identical output.
- Caller-owned RGB output buffers are bounded. Codec-owned scratch memory
  stays opaque and is not bounded by Ferrisma's memory budget.
- It builds on stable Rust 1.99 or later (see `rust-version`; the pinned
  nightly was required until 2026-11-15, libavif-rs ADR 0060), and only for
  macOS ARM64 and GNU Linux x86-64, the targets the generated libavif code
  supports
  ([ADR-0026](https://github.com/sebastian-software/ferrisma/blob/main/docs/adr/0026-codecs-in-one-repository-and-workspace.md)).
- A dependent's build profile must keep `panic = "unwind"` (Cargo's default;
  `panic = "abort"` is a compile error). Turning overflow checks off for the
  four AVIF packages is recommended; see
  [publishing](https://github.com/sebastian-software/ferrisma/blob/main/docs/publishing.md),
  blocker 3.

Internal to `ferrisma`: this crate follows no SemVer of its own. `ferrisma`
requires exactly the version released with it (an `=` requirement), and any
release may change this crate's API. Do not depend on it directly; enable
the `ferrisma` feature named above instead
([ADR-0040](https://github.com/sebastian-software/ferrisma/blob/main/docs/adr/0040-stable-public-contract-and-compatibility-policy.md),
problem 7).

License: MIT OR Apache-2.0 (`LICENSE-MIT`, `LICENSE-APACHE`). The ported
codecs carry their own licenses.
