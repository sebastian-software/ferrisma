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
x86_64 original graphs in CI run 41440766024 (retired libjxl-rs repository),
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

### Complete sequence geometry

`decoder-geometry-fixtures.json` records 128 actual public JPEG encoder outputs
from all 64 component-sampling input triples at odd/tiny dimensions, 20 genuine
reference/crop/resampling/preview/extra-channel sequences, and eight original metadata-writer
orientation variants. JPEG source inputs, byte hashes, encoder/header observations
and cjpeg pixel/options/scan recipes are retained. Default audits use stored JPEG
inputs; cjpeg is not a build dependency. Distinct input factor triples may be
canonicalized to the same effective chroma shifts by the original encoder.

`decoder-geometry.requests` lists the 148 source base files. The native manifest
adds all eight oriented variants and excludes the two preview files from the
non-preview export adapter, leaving 154 independent RGBA8 captures. Each capture
contains complete oriented output rows with 16 bytes of untouched 0xa5 padding
and a seven-byte trailing guard. `tools/decoder_geometry_reference.py` reproduces
every source fixture and capture, verifies actual original bindings and repeats
all 58,688 defined complete-file/error/export requests under both original graphs.
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


### skcms fitting requests

`skcms-fitting-requests.bin` retains all 11,432 initial preflight inputs. Each
record is a command byte, a little-endian u32 word count, and that many
little-endian u32 protocol words. The command order and signed alias/L/N words
are described in `skcms-fitting-requests.json`. The fixture contains input data
only; the ordinary or fully instrumented original supplies every differential
expected output. `skcms-fitting-degenerate.table8` retains the original 255-byte
table for the five initially differing degenerate-segment/alias cases. Native
source words and both archive hashes are recorded in the manifest.


### skcms baseline transform inventories

`skcms-transform-ops.txt` contains the 79 original work/store stage names in
header order. The independent transform audit compares it and the Rust numeric
carriers to the actual pinned headers, including every one of the 42 pixel and
three alpha formats. It captures all 151,592 generated transform requests and
both complete original observations as artifacts. `skcms-transform-preflight.requests.gz` freezes every one of the original
11,448 preflight inputs; the adjacent JSON manifest records its hashes and all
148 initial failure indices/request hashes. The audit requires every frozen
input in the expanded generators and compares their original words under the
recorded native configuration. Formats/tails, CICP/profile, Lab, alias and
exceptional regressions remain in the expanded generators. All half words and integer halfway neighbors supplement
that corpus; they do not replace earlier failures.

`skcms-transform-native-arm64.tsv` stores 16 independent actual-original
observations, including source-defined partially initialized contexts. Its JSON
manifest records platform, compiler, pins and the fixture hash. These retained
numeric captures apply to the macOS ARM baseline; native CI independently
captures its own target and compares both original graphs. No Rust output is
used to produce a source fixture.


### Default CMS retained inputs and native observations

`default-cms-matrix-inputs.bin` retains the initial 4,096 MatrixProduct inputs,
including actual input/output aliases. Each 52-byte record is thirteen
little-endian u32 words: nine float matrix words, three float input words and
the alias flag. The file contains inputs only.

`default-cms-preflight.json` records the fixture hash, every initial failed
index, deterministic request hashes and the initial source-observation hashes
for the recorded native configuration. The generators reproduce all initial
public/profile/row/CICP/transfer/prefix/callback/numeric inputs byte-for-byte.
The independent audit requires those inputs everywhere and compares every
returned word with both actual native source graphs; architecture-scoped hashes
never substitute one platform's source words for another's.

`default-cms-native-arm64.tsv` stores 16 independent actual-source observations
covering interface/guards, profile rows, private arithmetic, partial contexts,
callback mutation and retained initial failures. The adjacent JSON records its
pins, native compiler/platform and hash. Its Rust test invokes no C++ process;
the independent audit reconstructs the captures from both actual source graphs.
An additional 88,064 MatrixProduct cases retain exceptional coefficients,
competing NaN inputs and original in-place write order in both source graphs.
`skcms-concat-paired-nan.requests` adds 432 defined inputs: every matrix output
cell and product term, crossed with four signed quiet/signaling NaN words in
both multiplicands. `skcms-concat-paired-nan-arm.words` records the actual native
ARM ordinary/ASan outputs as 432 matrices of nine row-major little-endian u32
float words. Both original graphs independently produce the same bytes; the
adjacent manifest records source bindings, numeric options and command/object
hashes. The source audit
reproduces the inputs and native captures. These inputs supplement every old
concatenation request; native x86 output words are independently validated.

`skcms-concat-paired-nan-x86.words` retains independently captured native Linux
words from run 41550202739's original declaration-only concat configuration.
`skcms-concat-x86-source-config.json` records the source/artifact hashes and all
48 changed paired-NaN observations per graph. No finite or prior observation
changed. The isolated complete-body observer must reproduce these captures.

## decode_test.cc testdata files

`decode-test/` holds the five files that `lib/jxl/decode_test.cc` reads from
the pinned libjxl testdata submodule, revision
`73695d303670c90e4d506ea89d9901b081385089`, copied unchanged:

| File | Testdata path | SHA-256 |
| --- | --- | --- |
| `scRGB-v2.icc` | `external/Compact-ICC-Profiles/profiles/scRGB-v2.icc` | `8ae25bd986ea0dc396540762a6aa8c7fbeab7ac777cfdf242fa7c35874f4b383` |
| `flower.png.im_q85_420.jpg` | `jxl/flower/flower.png.im_q85_420.jpg` | `6ad9a79e9f26d2723ef954a0dca774429606655ab7184f8ab400016f228ca2ce` |
| `1x1_exif_xmp.jpg` | `jxl/jpeg_reconstruction/1x1_exif_xmp.jpg` | `c6d30276e37711069950d50b8d8c94c037f5b7a1fdcc36235c6f3f066487bed8` |
| `1x1_exif_xmp.jxl` | `jxl/jpeg_reconstruction/1x1_exif_xmp.jxl` | `30966de3d58f38a4b9f29a26483f5839b32c283c69552cf5d6aa1ec18e5f77b2` |
| `square-extended-size-container.jxl` | `jxl/boxes/square-extended-size-container.jxl` | `c324d018c375f291230831fe27ba14e8ee39f3a1142d51b658a37227a6c1d850` |

The ICC profile is from Compact-ICC-Profiles under CC0; see
`LICENSE-Compact-ICC-Profiles-CC0.txt`. The other files are part of the libjxl
testdata repository (https://github.com/libjxl/testdata) by the JPEG XL Project
Authors under the Creative Commons Attribution 4.0 International license; see
`LICENSE-testdata-CC-BY-4.0.txt`. Only the testdata submodule's pinned revision
was used; it is not needed to run the tests. `tests/parity/decode_test_inventory.rs`
passes the files to the oracle and replays the test histories on them.
