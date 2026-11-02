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

## Captured decoder effects

`effects_rgb_alpha8` and `effects_gray_alpha16` were captured independently from
original `vd-encode-splines` and `vd-decode` calls on 2026-11-02. Their matching
`.jxl`, `.pixels`, and `.f32le` files retain the same pinned source/license as
above. Both are raw level-10 3x8 files, effort 7, distance 1, progressive AC,
Gaborish, EPF=3, nonzero manual noise and one genuine custom spline. Input samples
are `(i * 113 + i * i * 17) & 65535`, using their high byte at eight bits.
The adapter supplies parameters to the original header/one-shot encoder and
original spline quantization; it implements no framing or codec algorithm.

Manual noise LUT values are 0.005 through 0.04 in steps of 0.005. The spline's
four control points are `(1,1)`, `(0.3*w,0.8*h)`, `(0.6*w,0.2*h)` and `(w,h)`.
Its X/Y/B DC coefficients are 0.02/0.3/0.5, its X AC coefficient 1 is 0.1, and
its sigma coefficients 0/1 are 2.0/0.5; other coefficients are zero. The original
quantizer uses adjustment zero and CfL ratios zero/one. See the actual adapter
for source float rounding. The original decoder uses the selected scalar
reference configuration in ADR 0002.

`.pixels` contains interleaved original gray/alpha or RGBA integer samples,
with little-endian 16-bit words. `.f32le` contains all three interleaved original
color float words as little-endian u32 bytes. `tests/decoder_effects.rs` compares
both without compiling, linking or invoking C++. Broader generated effects and
error comparisons remain in the opt-in differential suite.

The additional `.x86_64.f32le` files were independently captured from both native
x86_64 original graphs in [CI run 41440766024](https://github.com/sebastian-software/libjxl-rs/actions/runs/41440766024),
job 125261745194. The ordinary and ASan/UBSan originals agree on every captured
word for each unchanged `.jxl` file. Their values were printed by the original
`vd-decode` entry point, independently of Rust's failed comparison. The existing
AArch64 captures remain unchanged. The standalone test selects the exact native
capture: source contraction differs across these CPU targets, as allowed by the
porting contract. Integer captures are shared; no float tolerance is used.
