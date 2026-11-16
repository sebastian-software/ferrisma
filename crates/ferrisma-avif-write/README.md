# ferrisma-avif-write

A narrow, safe still-image AVIF encoder over `libavif-rs`, the Ferrisma workspace's
mechanical Rust port of libavif (with its libaom translation; libavif is built
without dav1d).
It is the backend of the `avif-write` feature (formerly `avif-own-write`) of
[`ferrisma`](https://crates.io/crates/ferrisma) and is not meant to be used on
its own.

- The port's raw C-shaped API stays behind this crate; callers see safe
  functions only.
- Input rows are straight-alpha sRGB8 RGBA; the output uses CICP 1/13/6 and
  full range. Codec-owned memory is not bounded by Ferrisma's memory budget.
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
