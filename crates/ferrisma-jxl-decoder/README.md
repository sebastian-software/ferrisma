# ferrisma-jxl-decoder

A narrow, safe JPEG XL read bridge to `libjxl-rs`, the Ferrisma workspace's
mechanical Rust port of libjxl, the JPEG XL reference implementation. It is
the backend of the `jpeg-xl-read` feature (formerly `jpeg-xl-own-read`) of
[`ferrisma`](https://crates.io/crates/ferrisma) and is not meant to be used on
its own.

- The port's raw C-shaped decoder API stays behind this crate; callers see
  safe functions with caller-owned, bounded output.
- The decoder has no crop request: every read reconstructs the full frame,
  and `decode_region_into` keeps only the requested upright region through
  the decoder's image-out callback.
- Builds on stable Rust (see `rust-version`).

Experimental and unstable: its API may change in any 0.x release.

License: MIT OR Apache-2.0 (`LICENSE-MIT`, `LICENSE-APACHE`). The ported
codec carries its own licenses.
