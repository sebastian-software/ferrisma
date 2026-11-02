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

## JPEG-derived YCbCr and chroma fixtures

`ycbcr-jpeg-fixtures.json` records 25 genuine baseline JPEG files and their raw
JXL codestreams: 4:4:4, 4:2:2, 4:2:0, 4:4:0 and grayscale at 65x33, 1x1, 9x7,
257x3 and 3x257. RGB8 source samples follow
`(x*17 + y*23 + c*61 + (x*y)%47)%256`, written as binary P6 PPM. The recorded
libjpeg-turbo cjpeg version encodes quality 90, precision 8, integer DCT and
baseline JPEG; sampling is `1x1,1x1,1x1`, `2x1,1x1,1x1`, `2x2,1x1,1x1` or
`1x2,1x1,1x1`, with `-grayscale` for the final variant. Validation uses the stored
JPEGs and does not require cjpeg or ImageMagick.

The pristine pinned parser independently reports actual JPEG sampling. Public
JxlEncoderUseContainer(false), JxlEncoderStoreJPEGMetadata(false), effort 3 and
JxlEncoderAddJPEGFrame produce the stored raw codestreams, with every API status
successful. Every `.float-pixels` file contains all source-initialized RGB words
in planar channel/y/x order as little-endian binary32. These are independently
captured from matching ordinary and ASan/UBSan AArch64 original graphs; the
already quantized JPEG samples are never compared to pre-JPEG RGB as a golden.
The manifest retains original writer observations and JPEG/JXL/pixel hashes.

`tools/chroma_image_reference.py` repeats the genuine original encoder calls,
checks each exact codestream and captured pixel word, verifies actual original
symbol bindings, and compares 1,548 complete/header/truncated/mutated/trailing
observations between both source graphs. Existing stage exceptional/finite
acceptance and lifecycle sequences remain independent controls. Strict native
Linux/macOS validation is required for an integration tree.

### Complete extra-channel files

`extra-channel-*.jxl` files are genuine pinned-original EncodeFrame outputs or
original frame-header/TOC-writer controls. `extra-channels.requests` freezes all
111 source-generation requests, `extra-channels.decode-statuses` records their
69 source successes and 42 genuine source errors, and
`extra-channels-fixtures.json` records each request, byte hash, actual writer
observation and source revision. The tests reproduce source bytes independently
and retain failures as actual decoder-status/partial-state cases.

Fixtures cover every defined extra-channel type, independent integer/float
depths, associated/unassociated and multiple alpha planes, early/late spatial
upsampling, dimension shifts 0..3, per-channel mixed sampling, odd/tiny/group
boundaries, spot order, binary names, signed/extreme spot coefficients, filters,
patches and pre-/post-color references. Mixed per-channel factors reuse genuine
1x1 payloads with unchanged modular dimensions and exact original-writer controls.
The 12 `extra-channel-rejected-*.jxl` files deliberately mutate a genuine optional
channel file; actual source metadata parsing identifies the rejected enum and
its partial state. Their source mutation requests/proofs remain in the manifest
and `extra-channels-rejections.requests`.

`extra-channel-all-types-dyadic.float-pixels` contains every initialized planar
output word from an independently executed pristine native source export: first
RGB interleaved in y/x/channel order, then each complete extra plane in metadata
order, all as little-endian binary32 words. The input uses exactly representable
dyadic samples/coefficients so the capture has no target-dependent contraction
ambiguity. `tools/extra_channel_fixtures.py` regenerates through the real source
encoder and refuses changes to existing fixture/capture bytes.
`tools/extra_channels_reference.py --output <new-directory>` independently
reproduces both original graphs, verifies actual source bindings, retains native
captures and all 15,315 complete/partial/export observations. Direct output-stage
callbacks and the public incremental decoder API remain separate work.

`extra-channels-half-strides.tsv` freezes the actual original sanitizer census
for all 888 initially introduced odd-stride Float16 requests: 528 positively
reached span.h:59 misaligned stores and 360 defined single-row/decode-error
requests. Each original request remains in the source-only census, all defined
ones keep exact parity, and 888 aligned Float16 controls are added. The separate
`tools/extra_half_boundaries.py` audit records every ordinary observation and
requires each actual nonrecovering UBSan diagnostic. The C++ source and existing
integer16/float32 odd-stride requests remain unchanged.

## skcms transfer and matrix input inventories

`skcms-transfer-classification.requests` retains the original 16,000
classification/inversion inputs. `skcms-transfer-exceptional.tsv` retains all
8,624 exceptional-parameter calls, marked D (8,262 defined) or U (362 positively
diagnosed original float-to-integer boundaries). Its manifest records source
pins and census provenance. Expected outputs are always obtained independently
from the actual pinned public functions.

`skcms-matrix-requests.bin` retains all 71,226 initial matrix requests without
changing any float word, alias flag or destination sentinel. Each record stores
a command byte (0 concat, 1 invert, 2 adapt, 3 primaries), a flag byte, and
18/18/11/17 little-endian u32 float words. The adjacent JSON manifest records
both the binary hash and the exact original textual request hash. The source
audit decodes and checks that inventory independently. The fixture contains no
expected outputs; the original CMS archive remains the numerical oracle.
