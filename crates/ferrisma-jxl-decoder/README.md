# ferrisma-jxl-decoder

A narrow, safe JPEG XL read bridge to `ferrisma-libjxl`, the Ferrisma workspace's
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

Internal to `ferrisma`: this crate follows no SemVer of its own. `ferrisma`
requires exactly the version released with it (an `=` requirement), and any
release may change this crate's API. Do not depend on it directly; enable
the `ferrisma` feature named above instead
([ADR-0040](https://github.com/sebastian-software/ferrisma/blob/main/docs/adr/0040-stable-public-contract-and-compatibility-policy.md),
problem 7).

License: MIT OR Apache-2.0 (`LICENSE-MIT`, `LICENSE-APACHE`). The ported
codec carries its own licenses.
