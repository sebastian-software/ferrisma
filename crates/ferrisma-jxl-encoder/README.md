# ferrisma-jxl-encoder

A narrow, safe JPEG XL write bridge to the encoder of `libjxl-rs`, the
Ferrisma workspace's mechanical Rust port of libjxl. It is the lossy backend
of the experimental `jpeg-xl-write` feature of
[`ferrisma`](https://crates.io/crates/ferrisma)
([ADR-0039](https://github.com/sebastian-software/ferrisma/blob/main/docs/adr/0039-jpeg-xl-output-through-the-workspace-encoder.md)) and is not meant to be used
on its own.

- `Encoder` owns the port's raw encoder and frame-settings objects, copies
  the caller's pixels, and drains the codestream into a caller-owned
  `Vec<u8>` with an output-byte limit.
- Only the profile the port admits is exposed (effort 7, VarDCT with XYB, a
  bounded distance range, gray or RGB with optional 8-bit alpha, one frame,
  raw codestream). Requests outside it return `Error::NotAdmitted`.
- Builds on stable Rust (see `rust-version`).

Internal to `ferrisma`: this crate follows no SemVer of its own. `ferrisma`
requires exactly the version released with it (an `=` requirement), and any
release may change this crate's API. Do not depend on it directly; enable
the `ferrisma` feature named above instead
([ADR-0040](https://github.com/sebastian-software/ferrisma/blob/main/docs/adr/0040-stable-public-contract-and-compatibility-policy.md),
problem 7).

License: MIT OR Apache-2.0 (`LICENSE-MIT`, `LICENSE-APACHE`). The ported
codec carries its own licenses.
