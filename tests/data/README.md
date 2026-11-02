# Captured original VarDCT files

These small fixtures were generated on 2026-11-02 by the unmodified pinned
libjxl encoder, revision `a16d32d0ac113f5e54125e49bb571281c5d0b704`, through
`tools/oracle/vardct_image.inc`. Derived fixtures retain the repository's
upstream BSD-3-Clause license and PATENTS grant.

Both inputs are 3x8, effort 7, distance 1, XYB, progressive AC and Gaborish,
with EPF, patches, dots and noise off. Input samples are
`(i * 173 + i * i * 31 + 29) & 65535`; RGB8 uses their high byte and
gray-alpha16 uses the native little-endian 16-bit words.

- `vardct-rgb8.jxl`: original public encoder, raw level-5 codestream.
- `vardct-gray-alpha16.jxl`: original header/one-shot encoder, raw level-10
  codestream. The public API wraps this configuration in a container, while
  the admitted Rust entry point accepts raw codestreams.
- Matching `.pixels` files contain the original decoder's interleaved integer
  output, captured independently of Rust. The 16-bit words are little-endian
  on the verified native ARM64 macOS and x86_64 Linux targets.

No C++ executable is needed to run `tests/vardct_image.rs`. Broader generated
original-file comparisons remain in the opt-in differential suite.
