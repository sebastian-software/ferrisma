# CMYK JPEG XL fixture (issue #275)

`cmyk_layers.jxl` is the `cmyk_layers` case of the public JPEG XL conformance
corpus, copied unchanged.

- Source: <https://github.com/libjxl/conformance>, revision
  `b1d0f990b03e57bf6d137c365cd5dc8b470b9191`, `testcases/cmyk_layers/input.jxl`.
- License: CC0 (per-case table in the corpus's `testcases/README.md`), so the
  file may be committed and redistributed.
- SHA-256: `d732c8836bf1abeadf310d2e07387a32813ed4690d32650c1c25e541b80eed4a`,
  the value pinned in `codecs/libjxl-rs/tools/conformance/MANIFEST.sha256`.
- 387,140 bytes; RGB color with a Black (K) extra channel and an alpha channel
  at 8 bits.

The engine refuses it (`UnsupportedInput`, `UnsupportedLayout`); see
`jpeg_xl_cmyk_is_rejected_as_a_layout` in `tests/supported_matrix.rs` and the
JPEG XL section of `docs/supported-matrix.md`.
