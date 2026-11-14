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

Experimental and unstable: its API may change in any 0.x release.

License: MIT OR Apache-2.0 (`LICENSE-MIT`, `LICENSE-APACHE`). The ported
codec carries its own licenses.
