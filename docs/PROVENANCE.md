# Provenance

The source revision is recorded in [upstream.json](../upstream.json) and pinned
as the `upstream/libjxl` Git submodule. Its original LICENSE and PATENTS are
copied without changes to the repository root. Ported modules retain the JPEG
XL Project Authors' copyright notice and name the source files they translate.

## Source and test inputs

The complete fixed image path connects the original EncodeFrameOneShot/global
header block to DecodeFrame and the DecodeFile helper in
`lib/jxl/render_pipeline/render_pipeline_test.cc`, followed by the original
integer buffer branch of `dec_external_image.cc`. CodecInOut::CheckMetadata in
`lib/extras/codec_in_out.h` and ImageBundle::VerifyMetadata supply the final
metadata checks. Parsed metadata moves into private admitted ownership; integer
samples and metadata leave without a self-referential image bundle.

`tools/oracle/extract_source.py` extracts exact original DecodeFile and ReadICC
function bodies with SHA-256 checks from `source_fragments.json`. The generated
fragments preserve upstream notices and line locations; their algorithms are
not rewritten. The actual original test memory manager is compiled directly.
This avoids linking unrelated test-framework registration while retaining the
real source routines and the default original ICC/CMS branch. The source/native
whole-file corpus also invokes the public C++ decoder on both encoder outputs.
Generated complete images, prefix/corruption cases, and the exhaustive native
16-bit image are local test infrastructure. They satisfy the fixed
[first-image gate](FIRST_IMAGE.md), not the full upstream image-test inventory.

`dec_external_image.cc`, its used scalar Highway NearestInt/DemoteTo, and the
UnpremultiplyAlpha loop in `alpha.cc` supply integer and floating buffer export,
orientation, clamping, rounding, interleaving, alpha behavior, and output-span
checks. The float functions explicitly select float_out=true with 16/32 bits;
the existing integer functions retain their original types and implementation.
Callback output and custom runners remain separate. Source
JxlEndianness value 9 produces an undefined C++ enum load; the representable but
unnamed value 3 tests its defined fall-through instead, with identical expected
bytes and no dropped case. No sanitizer or exact-comparison suppression is used.

Highway `hwy/ops/scalar-inl.h` delegates FloatToF16's DemoteTo to F16FromF32 in
`hwy/base.h`. The reference's current Clang/GCC configurations select
HWY_HAVE_SCALAR_F16_OPERATORS=1 and a native half cast. Stable Rust represents
the resulting binary16 words with u16. Its finite nearest-even rounding follows
Highway's explicit exponent/mantissa/carry algorithm; the selected native cast's
NaN branch preserves sign and the top ten payload bits and sets the quiet bit.
The fallback source branch instead canonicalizes NaNs, so it is not silently
used as native-cast behavior. The float oracle rejects a non-native-operator
configuration at compile time and invokes the unchanged original FloatToF16 and
conversion functions. Float32 stores copy raw binary32 words, retaining signaling
NaNs. Both branches retain original x-major interleaving and the half branch's
native stores followed by a separate byte-swap pass. The original default IEEE
rounding environment remains a precondition. Alternate floating environments,
old compiler branches, SIMD, and performance parity are separate work.

Generated floating-output cases cover every half word, both signs around every
finite half midpoint, overflow boundaries, NaN payloads, generated binary32
words, nullable channels, orientations, endian stores, alpha unpremultiplication,
unchanged allocation suffixes, and failure/lifecycle guards. Native tests also
round-trip all half words with the selected cast's NaN quieting. These are local
component fixtures, not complete floating-image decoding or the full upstream
test inventory.

Modular group decoding, image-to-pipeline conversion, and finalization translate
the complete corresponding methods in `lib/jxl/dec_modular.cc` under a null pool
and the original simple render implementation. `modular/modular_image.cc` move
assignment supplies the scalar-retention/vector-transfer mapping. The local
group and pixel fixtures call actual original methods, using original encoder
sections or standalone modular streams and initialized output sentinels.
Native expected-sample integration is local infrastructure; neither fixture
replaces complete codestream decoder or upstream image-test coverage.

`render_pipeline/stage_gaborish.{h,cc}` supplies the complete scalar decoder
Gaborish stage and factory. The oracle calls the actual original stage directly
and inside the unchanged simple pipeline. Padded-row/random-weight/exceptional
float corpora and the 3x2 ordinary Rust regression are local test infrastructure;
the latter's exact words were captured independently from the original stage.
The dedicated `tools/oracle/gaborish.cc` observation unit includes the pristine
original source at scoped `-O0`; original-library and old-adapter flags are
unchanged and every sanitizer flag remains active. Clang 21 x86 ordinary and
instrumented `-O3` objects had selected different competing NaN operands.
The isolated unit preserves Highway's literal Add/Mul/MulAdd boundaries and
operator*'s float-to-double/product/float sequence. Local Rust non-inlined
arithmetic functions preserve those conversions, operand order, and separate
rounding; promotions quiet signaling NaNs before multiplication. Link maps
verify that the real source factory and stage bind to the dedicated object.
The finite 3x2 native observation is unchanged; another captured regression with
competing payloads and 128 normalization/pixel cases guard the repaired seam.
The ARM native exceptional words came from the original stage. Native hosted
x86 [captures from both complete ordinary and ASan/UBSan original oracles](https://github.com/sebastian-software/libjxl-rs/actions/runs/41331075671/job/124921109245)
agree with the unchanged ARM nine-word table. The earlier x86 table was observed
under QEMU, whose quiet-NaN selection ignored operand order; those emulated
values are not evidence of native x86 payload behavior and were removed.
The original x86 plain `-O0` constructor multiplies reciprocal-first, whereas
ASan/UBSan commutes the first two normalized products and retains that order for
the third. A local Rust non-inlined binary32 normalization multiply preserves
reciprocal-first independently of release inlining; ARM compound assignments
remain unchanged. The complete original native captures also agree for the
existing competing signaling/quiet filter-pair normalization observations,
despite those differing first two products. Complete ordinary and instrumented
native differential gates remain required. Existing finite
expected words and every differential case remain unchanged.
No filter algorithm is supplied by the C++ adapter. Review this numeric
configuration after source/compiler/target/linker/instrumentation changes.
Full lossy frame integration, SIMD, and performance parity remain separate work.

`render_pipeline/stage_upsampling.{h,cc}` and Highway's scalar Min/Max/MulAdd
operations supply the complete scalar upsampling stage. Differential requests
execute the original stage with the original simple pipeline; the 3x2 native
rounding regression uses independently captured original words. Weight, edge,
chunk, factory, and argument-guard corpora are local infrastructure. No original
interpolation algorithm is copied into the C++ fixture.
The translated Highway scalar helpers use its offered BSD-3-Clause alternative;
the unchanged notice is in
[licenses/HIGHWAY-BSD-3-Clause.txt](../licenses/HIGHWAY-BSD-3-Clause.txt).

`opsin_params.{h,cc}` and OpsinParams/OutputEncodingInfo/CanOutputToColorEncoding
in `dec_xyb.{h,cc}` supply default inverse-matrix selection, lane expansion,
parameter initialization, output-profile selection, and matrix/luminance/gamma
updates. Original methods supply all reference observations; generated profile
and reuse sequences are local infrastructure. Fixtures value-initialize source
aggregate scalars explicitly and retain partial state after errors. Pixel
conversion is translated separately below; general image conversion and
frame-profile integration remain separate boundaries.

`dec_xyb-inl.h` and the normal stage/factory in
`render_pipeline/stage_xyb.cc` supply the one-lane XybToRgb helper and XYBStage.
`tools/oracle/xyb_stage.cc` includes `xyb_stage.inc`, which includes the pristine
stage source and calls its actual scalar helper and public factory. Its
source-scoped -O0 setting retains source operand order across ordinary and
ASan/UBSan builds; all instrumentations stay active. Other fixture/original
codec-library flags are unchanged. The native helper's local Add/Mul call
boundaries retain the same Highway scalar order, binary64 multiplication with
float narrowing, and unfused rounding. The scaled-output stage uses the same
local product mapping. This
pins a numeric configuration without changing any source algorithm or claiming
performance parity. It supplies input/profile/parameter
fixtures, observes all output words and unchanged guards/extra channels, and
executes the original simple pipeline with reverse group submission. Default
and raw custom parameter corpora include every scalar parameter position,
signed zeros, subnormals, infinities, and NaN payloads. Generated corpora remain
local infrastructure; no conversion algorithm is supplied by the fixture.
FastXYB/sRGB8, SIMD targets, general image conversion, and decoder-profile
integration remain separate boundaries.

`passes_state.cc`, decoder storage/constructor/Init and callback carriers in
`dec_cache.h`, InitForAC in `dec_cache.cc`, callback typedefs in the public
`decode.h`, and strategy-order/offset tables in `coeff_order.h` supply shared
and decoder initialization. The reference calls actual original methods and
observes their allocation metadata, bindings, partial state, and defined vector
entries. Lossless run-length notation compresses complete coefficient-vector
observations. Callback functions are local caller probes, not codec replacements.
Native ownership/retention regressions and generated action sequences are local
infrastructure. General pipeline preparation and VarDCT GroupDecCache
initialization remain unported; fixed preparation/cache construction are
connected by the complete path above.

`ColorEncoding::SetICC` and `ColorSpaceTransform` in
`color_encoding_internal.h`, `cms::ColorEncoding::SetFieldsFromICC` in
`cms/color_encoding_cms.h`, and `render_pipeline/stage_cms.{h,cc}` supply the
callback-backed ICC/transform/stage implementation. The fixture calls actual
original methods with a caller-owned trace/bit-copy CMS; it does not replace
their parsing/ownership/control flow or claim to translate the default CMS.
The probe retains allocations until teardown while recording every destroy
callback, including original repeated-Init behavior. Generated profile/layout/
error corpora and native ICC-lifetime tests are local infrastructure.

- `libjxl/libjxl` at `a16d32d0ac113f5e54125e49bb571281c5d0b704`:
  `dec_bit_reader`, `enc_bit_writer`, `field_encodings`, `fields`, `enc_fields`,
  `headers`, the relevant `base/status` semantics, writer storage, alignment,
  and layer accounting. See [PORT_STATUS.md](PORT_STATUS.md) for partial units.
- Upstream `bit_reader_test.cc`, `enc_bit_writer_test.cc`, and `fields_test.cc`:
  fixed cases, test scenarios, and old/new bundle fixtures translated into Rust.
  Generated Rust test sequences use a local deterministic generator rather than
  claiming to reproduce the C++ RNG sequences exactly. Test threads are
  sequential in these adapted core cases; upstream thread-pool behavior is not
  yet translated or verified.
- Upstream `padded_bytes_test.cc`: first-byte initialization scenarios, extended
  with local checks for storage preservation, padding, and alignment.
- `base/bits.h`, `base/byte_order.h`, the endianness definition in
  `lib/include/jxl/types.h`, enum helpers in `field_encodings.h`/`fields.h`,
  `ans_params.h`, and `ans_common.{h,cc}` from the same pinned source: shared
  helpers and ANS alias tables. Native cases are adapted from `bits_test.cc`,
  `byte_order_test.cc`, and `ans_common_test.cc`; the raw enum fixture and
  generated differential distributions are local test infrastructure.
- Highway at `457c891775a7397bdb0376bb1031e6e027af1c48`, selected by upstream's
  submodule: C++ oracle compilation only. Its code remains in that submodule
  with its original LICENSE; it is not compiled into the Rust library.
- `enc_huffman_tree`, `huffman_table`, `enc_huffman`, and `dec_huffman` headers
  and implementations from the same pinned source: tree construction,
  serialization, lookup tables, and prefix decoding. The histogram corpora,
  code-length RLE cases, malformed/truncated streams, and table-reuse fixtures
  are local test infrastructure, not claimed as translated upstream tests.
- `dec_ans`, `dec_context_map`, token/state/config portions of `enc_ans`, the
  scalar specialization in `inverse_mtf-inl.h`, and the used prefetch operation
  from Highway's `hwy/cache_control.h`: entropy streams and supporting storage.
  The uint-config vector round trip adapts `ans_test.cc`; deterministic stream,
  corruption, ring-wrap, and fast-path fixtures are local test infrastructure.
  Original `BuildAndStoreANSEncodingData` and `WriteTokens` generate reference
  fixtures without changing or copying their algorithms into the oracle.
  The oracle generates public export/version headers through CMake using the
  pinned source's template, version settings, and export-target pattern.
  The translated Highway prefetch helper uses Highway's offered BSD-3-Clause
  alternative; its unchanged notice is in
  [licenses/HIGHWAY-BSD-3-Clause.txt](../licenses/HIGHWAY-BSD-3-Clause.txt).
- Histogram storage and ANS strategy in `enc_ans_params.h`, and the anonymous
  `ANSEncodingHistogram` implementation plus table-building/cost entry points
  in `enc_ans.cc`: normalization, fixed-point cost tables, histogram output,
  and reciprocal/reverse encoding maps. Local differential cases cover sparse,
  tied, high-count, empty, oversized, and prefix histograms with all original
  search strategies and both writer-present/absent paths.
- `enc_cluster.{h,cc}`, HistogramParams defaults/strategies and SpeedTier, plus
  scalar `FastLog2f` and its rational polynomial: clustering and encoder cost
  dependencies. The approximation's bounded native test adapts
  `fast_math_test.cc`; bit-pattern and cluster corpora are local differential
  infrastructure. SIMD targets and other fast-math functions remain unported.
- `enc_lz77.{h,cc}`: original cost tables/model, RLE, greedy bucket variants,
  optimal hash-chain variants, and dispatch. The oracle includes the pristine
  `.cc` in its adapter translation unit to observe the private cost helper
  directly. Generated token/cost corpora and the native copy-expansion test are
  local infrastructure. Explicit float-bit regressions retain the AArch64
  reference compiler's fused scalar accumulation and final penalty. The oracle
  preserves upstream's Clang `-fno-slp-vectorize -fno-vectorize` settings from
  `lib/CMakeLists.txt`; omitting them changed this cost model's float rounding.
- Remaining release-state entropy orchestration in `enc_ans.cc`, the scalar
  token-cost specialization in `enc_ans_simd.cc`, scalar MaxValue/vector-size
  support in `simd_util`, and context output in `enc_context_map.cc`: uint
  strategy selection, complete histogram/code output, streaming state, and MTF
  cost decisions. The new oracle adapter calls the original complete encoder
  entry points. Its corpus, cross-decoding checks, and native entropy pipeline
  test are local infrastructure; they are not image-codec acceptance evidence.
- `base/matrix_ops.h`, `cms/opsin_params.h`, `cms/transfer_functions.h`,
  `cms/tone_mapping.h`, and `cms/jxl_cms_internal.h`: profile-generation
  dependencies, original scaling/cube construction, transfer/tone functions,
  descriptions, ICC tags/tables, and profile-ID MD5. The enum masks and external
  color fields originate in `color_encoding_internal.h` and
  `lib/include/jxl/color_encoding.h`. The `%g` mapping follows `base/common.h`
  through Rust formatting, without linking a C formatter. Differential corpora
  are local infrastructure, including parameter ranges from the upstream
  transfer/tone test scenarios; they do not claim to port those SIMD tests.
  Native MD5 known answers and ICC tag-layout checks are local tests.
- `cms/color_encoding_cms.h`, `color_encoding_internal.{h,cc}`,
  `image_metadata.{h,cc}`, the name visitor in `frame_header.h`, the used
  quantizer bias constants, and codestream/metadata writers in `enc_fields.cc`:
  metadata field storage, conversion, visitors, and header output. Native
  coordinate/gamma scenarios adapt `color_encoding_internal_test.cc`; complete
  header, malformed input, custom-weight, and reused-object corpora are local
  infrastructure. These compare original source entry points, without
  substituting test framing for the codestream-header writer.

- `frame_header.{h,cc}`, `frame_dimensions.h`, `base/rect.h`, `pack_signed.h`,
  `loop_filter.{h,cc}`, and the frame writer in `enc_fields.cc`: frame fields,
  progressive passes, geometry, and filter parameter bundles. Cross-language
  byte/state corpora, malformed/truncated headers, geometry edge cases, and
  native metadata/frame integration tests are local infrastructure. The oracle
  seeds inactive pass-array entries before observations to avoid reading
  undefined initial C++ storage. Pixel filter kernels remain separate work.

- `lehmer_code.h`, `coeff_order_fwd.h`, permutation portions of
  `coeff_order.{h,cc}` and `enc_coeff_order.{h,cc}`, `toc.{h,cc}`, and
  `enc_toc.{h,cc}`: original Fenwick trees, permutation entropy streams, section
  sizes, budget checks, and group offsets. Native round-trip size ranges adapt
  `lehmer_code_test.cc` using a local deterministic shuffle. Differential
  workspaces, byte corpora, truncation/corruption, and section payload integration
  are local infrastructure. The oracle uses non-null empty input spans to match
  Rust slices and the shared Input fixture's bit-reader close lifecycle.

- `image.{h,cc}`, the copy/fill/mirror portions of `image_ops.h`, row sizing
  from `memory_manager_internal.cc`, `modular/options.h`,
  `modular/modular_image.{h,cc}`, and transform/squeeze parameter fields in
  `modular/transform`: original storage, copies, channel validation, defaults,
  and field serialization. Storage/resize/clone/field corpora and native
  ownership tests are local infrastructure. The oracle explicitly paints
  caller-owned physical storage before observing it, and observes only valid
  samples after a source clone. These tests do not read uninitialized padding
  or claim to translate the upstream image and transform pixel test inventory.

- `modular/transform/rct.{h,cc}` and `enc_rct.{h,cc}`: original Highway scalar
  inverse/forward RCT, channel permutation, and null-pool row traversal. Native
  known answers/round trips and differential image/channel corpora are local
  infrastructure. Forward inputs avoid the source scalar path's undefined
  signed overflow; inverse cases include the source's defined Highway wrapping.
  No upstream color-transform test inventory or custom runner is claimed here.

- `modular/encoding/context_predict.h`: basic/weighted predictors, header and
  presets, original fixed-point reciprocal table, two-row error state, reference
  properties, flat-node unions/lookup, and all prediction wrappers. Differential
  header/math/property/tree/raster corpora and native rounding/prefix-replay tests
  are local infrastructure. The oracle calls the actual inline source and
  inspects only defined result members, never an indeterminate non-tree
  multiplier. No residual codec or prediction-tree learning algorithm is
  substituted by these test fixtures.

- `modular/encoding/ma_common.h`, `dec_ma.{h,cc}`, the `TokenizeTree` entry point
  in `enc_ma.cc`, and the group-header/filter/LUT portions of `encoding.{h,cc}`:
  original constants, tree storage, iterative validation, entropy-backed parsing,
  BFS tokenization, static pruning, flattening, and lookup-table range traversal.
  Generated trees, raw token fixtures, bundle inputs, and native integration
  cases are local infrastructure. Stream tests compose the actual original
  tokenization, histogram encoding, and token-writing calls, as the source
  encoder does. LUT observations losslessly encode every initialized entry as
  runs, including untouched entries after failure. No tree-learning or residual
  algorithm is replaced by fixtures, and no upstream tree test inventory is
  claimed as translated.

- `modular/encoding/encoding.cc` channel residual decoder and
  `enc_encoding.{h,cc}` predefined trees, given-tree `ModularCompress`, and its
  channel encoder: original scalar fast paths, LZ77 dispatch, single-value and
  Huffman/RLE cases, interior prediction loops, residual arithmetic, overflow
  checks, and predictor-image scratch allocation/writes. Generated raster,
  token-state, malformed-stream, and native integration tests are local
  infrastructure. They call the actual original functions, including the
  cross-channel run-state guard, and supply no substitute image/frame codec.
  Tree learning and generic image encoding have their own evidence below;
  frame orchestration and the upstream image test inventory remain separate.

The differential protocol adapter and corpus generators are project test
infrastructure. Their reference side calls original upstream implementations,
not copied algorithm rewrites. The local `DefaultedBundle` fixture exercises
the upstream `AllDefault`/`SetDefault`/`VisitNested` pattern and is not presented
as a preexisting upstream test case. No external image corpus has been imported
yet.

Palette lookup/inversion/metadata in `modular/transform/palette.{h,cc}`, scalar
Squeeze kernels/defaults/metadata in `squeeze.{h,cc}` and `enc_squeeze.{h,cc}`,
the original `Transform::MetaApply`/`Inverse` dispatch, and
`Image::undo_transforms` are translated. Differential tables, schedules, raster
fixtures, invalid-state inputs, and native known-answer/stack tests are local
infrastructure. The oracle calls those original APIs with a null pool. Metadata
observations omit indeterminate pixel storage; forward arithmetic cases remain
inside the source's defined i32 range. No forward palette algorithm, SIMD
specialization, custom runner, or upstream full-image test inventory is claimed
by these fixtures.

The remaining image-level portions of `modular/encoding/encoding.{h,cc}` now
provide ValidateChannelDimensions, ModularDecode, and ModularGenericDecompress.
Generated standalone images use the actual original ModularGenericCompress,
including original forward Palette/Squeeze/RCT calls in the oracle. Global
table fixtures compose the source entropy writers and ModularCompress as test
infrastructure, not a replacement frame encoder. Full decodes independently
check source samples and shape; malformed cases compare defined partial state.
Two native fixed fixtures are captured from the pinned scalar C++ generator
with their configurations documented in `tests/modular_decode.rs`. These
fixtures do not substitute for Rust encoding or frame orchestration.

`base/random.h`, the remaining `modular/encoding/enc_ma.{h,cc}`, and the
learning/generic encoder portions of `enc_encoding.{h,cc}` provide the source
RNG/distributions, sample collection, quantization, hash deduplication, original
scalar entropy/split search, and complete standalone generic modular encoding.
The adapter calls the original APIs and observes exact float bits, sample
state, trees, accounting, encoded bytes, and decoded rasters. Generated corpora
and native integration cases in `tests/modular_encode.rs` are local test
infrastructure, not a translated upstream image-test inventory. The native gray
encoder test reuses the independently captured C++ decoder fixture. Complete
JPEG XL image/frame orchestration remains a separate boundary.

`modular/transform/enc_palette.{h,cc}` and `enc_transform.{h,cc}` supply the
original forward palette algorithm, scalar color distance, quantization,
single-channel lookup/set paths, luma ordering, lossy delta learning/error
diffusion, forward dispatch, and min/max scan. The oracle includes the
unmodified `enc_palette.cc` in its adapter translation unit to call private
helpers; it is not separately compiled a second time. Its generated tests
observe defined pixels and mutations. >=8-bit lossy delta cases use the
source frame encoder's RGB configuration because a fourth source delta row
contains indeterminate entries. The native full modular encoder test reuses
the independent C++ Palette/Squeeze RGB fixture without changing its bytes.

Ordered palette comparisons can tie for distinct colors. To preserve the
reference's concrete order, `base/libcpp_sort.rs` translates its non-arithmetic
libc++ sort specialization and heap dependencies. The original SDK headers,
version 220106, and SHA-256 hashes are retained in
[tools/reference/libcxx](../tools/reference/libcxx/README.md). They retain their
LLVM notices; the translation uses
[Apache-2.0](../licenses/LLVM-Apache-2.0.txt) with the
[LLVM exception](../licenses/LLVM-exception.txt). This explicitly tracked
dependency boundary is not a claim to translate the full standard library.
The oracle calls its actual C++ standard-library algorithms; the snapshots
are reference material and are never a Rust runtime fallback.

`enc_params.h`, the preparation functions in `enc_frame.{h,cc}`,
`enc_progressive_split.h`, `base/override.h`, ModularStreamId in
`dec_modular.h`, and QuantTable indices in `quant_weights.h` provide the
original parameter state/defaults, lossless setters, entropy policy, group
indices, progressive/header preparation, filter policy, and error-time state.
`splines.h` supplies point comparison, quantized storage/copy, and non-owning
views; spline algorithms remain outside this boundary. Parsed JPEG storage
from `jpeg/jpeg_data.h` and chroma/color-selection helpers from
`jpeg/enc_jpeg_data.cc` retain the source's marker precedence and partial
subsampling failures. These do not provide JPEG parsing or reconstruction.

`lib/include/jxl/cms_interface.h`, the debug callback signature in `encode.h`,
and bit-depth descriptors in `types.h` supply non-owning C data carriers.
Their Rust layouts, plus the external color-encoding layout, are checked
against original sizes, alignments, and offsets. No callback is invoked by
these storage/setter tests. Source-indeterminate CMS/opaque storage is omitted
from observations before assignment; source fixtures explicitly initialize
inactive storage when copying it.

The oracle includes pristine `enc_frame.cc` in its adapter translation unit
to observe private helpers and compiles the actual original JPEG data/helper
sources. The source's Brotli submodule at
`db24a98aed38a16de78cc05ca15cdd8dc2a0d92d` supplies their required headers.
It remains unmodified and outside the Rust library. The oracle generates the
CMS export header using upstream's CMake export-header pattern and uses the
static CMS configuration. Generated parameter/header corpora and native
ownership/partial-state tests are local infrastructure, not the upstream
full-image test inventory or first-image acceptance evidence.

The sample functions in `enc_modular.cc` and `dec_modular.cc` supply integer
normalization/rounding, custom-float packing/unpacking, raw binary32 copies,
the scalar MultiplySum/RgbFromSingle/SingleFromSingle kernels, and
SingleFromSingleAccurate. The translation preserves the 22-bit precision
threshold, original scalar contraction mapping, wrapped Highway addition,
special-value bit patterns, and output prefixes on representability failure.
Disjoint slices replace restrict pointers; assertions describe the source's
buffer, shift, and defined floating-to-integer conversion preconditions.
Original early format errors precede row-length assertions and any access.
`tools/oracle/sample_conversion.cc` includes both unchanged source files to
call the actual private encoder helper and decoder functions. It does not
compile those source files a second time or provide replacement frame methods.
The generated sample corpora and native row tests are local infrastructure.
Frame and group orchestration remain outside this completed sample boundary.

The private helpers MergeTrees, QuantizeChannel, EstimateWPCost, do_transform,
maybe_do_transform, and try_palettes in `enc_modular.cc`, plus the scalar
EstimateCost specialization in `enc_modular_simd.cc`, supply the original frame
transform selection dependencies. Their translation retains context thresholds,
fixed-point weighted state, overflow rejection, fractional cost accumulation,
source float/double casts, tier decisions, inverse rollback, palette ordering,
and partial metadata/stack mutations. The oracle's `modular_frame.inc` is
included after unchanged encoder source in `sample_conversion.cc` and calls
those actual helpers; unchanged `enc_modular_simd.cc` supplies the cost function.
Metadata-only force fixtures do not read uninitialized pixel planes. Newly
created source transforms' indeterminate max_delta_error is omitted from state
observations. Generated helper corpora and native tests do not replace complete
frame/group integration or the first-image acceptance corpus.

`ModularFrameEncoder` storage and Create/Init, PrepareStreamParams, ComputeTree,
ComputeTokens, EncodeGlobalInfo, EncodeStream, both clearing methods, and
ComputeStreamingAbsoluteAcGroupId come from `enc_modular.{h,cc}`. Their Rust
translation retains the original native/null-pool algorithms, resizing and
copy checkpoints, unsigned arithmetic, group/shift filtering, local transform
selection, multiplier-range merging, token/header state, and entropy output.
ComputeEncodingData and VarDCT-specific methods remain outside this boundary.
The private preparation method borrows its owner's parameter field directly;
all upstream calls pass that same field. Existing native allocation and borrowed
custom-view mappings remain in effect.

`tools/oracle/modular_frame_state.inc` calls the actual original methods and
observes defined private state using `-fno-access-control` on its fixture
translation unit only. No source/layout/algorithm substitution is involved.
Inputs seed group options at the original caller's checkpoint. Shared and local
stream tests follow enc_frame.cc's distinct non-streaming/streaming lifecycles;
they do not treat invalid predictor indices or unsupported repeated global-info
calls as reference results. Entropy tables are exercised through serialized
output; indeterminate scalars are omitted from direct observations. Writer
observations copy bits before padding, leaving the real
writer unchanged. The native zero-gray test uses independently captured original
42-bit modular sections (`258cc50ac100`, padded), not a JPEG XL codestream.
Generated state/action corpora remain local infrastructure; complete frame
entry points, decoding, and the first-image corpus are separate requirements.

The Linux reference explicitly uses Clang with libc++ for the translated
palette sort boundary. The same tied-element and heap-fallback differential
corpus compares the actual selected standard library; no sort implementation
is substituted in the adapter. Empty frame-group fixtures allocate a real
zero-length span using the original writer before its original vector append.
`tests/linux_scalar.rs` records the original x86_64/AArch64 downsampling and
failed DC-scaling observations. Local x86 scalar calls retain the original
ADDSS/MULSS operand order for competing NaN payloads; the source algorithms,
existing AArch64 words, and differential comparison semantics remain unchanged.

The original x86_64 GamutMap result for the existing native regression was
captured independently with the ordinary Clang 21 reference. Its
three output words are `0x7fe12345`; AArch64 retains the existing distinct blue
word `0x7fc98765`. The native test now checks its target's exact source words,
and the same input is added to the differential corpus. No production gamut
algorithm or existing AArch64 expectation changes.

The actual x86_64 oracle's optimized GamutMap object retains the original
right-nested luminance tree with the green product as the inner ADDSS
destination and red product as the outer destination. Each final blend also
uses the separately computed product as the ADDSS destination. Local native x86
Add call boundaries preserve those operand choices in both build modes. The
earlier standalone source probe allocated different operands, while the
emulated runtime selected the same larger quiet-NaN payload in either order.
Its ordinary/UBSan agreement therefore did not prove native NaN parity. Native
execution and disassembly of the actual source and Rust objects are the
required evidence for this mapping. MULSS products preserve separate rounding
without executing the source's folded double promotions.
The original source, oracle settings, global Highway helpers, existing cases,
expected words, and AArch64 arithmetic remain unchanged.

The decoder-effects fixtures call pristine scalar EPF, noise, spline, patch,
blending and alpha boundaries. Dedicated original EPF/noise/alpha objects use
only the numeric reference options recorded in ADR 0002, retaining all sanitizer
instrumentation. Durable audits compare their selected observations with untouched
optimized originals and check actual source binding. The source and dependency
pins remain unchanged. The two captured combined-effects raw files and all their
original integer/color-float outputs are documented in
[tests/data/README.md](../tests/data/README.md); they were generated and observed
through actual original entry points independently of Rust.

Additional decoder-effects x86_64 float captures come from both actual native
original graphs in CI run 41440766024. Each graph agrees on all 72 words for each
unchanged input file. They preserve independent source outputs instead of using
Rust results as golden data; the original AArch64 captures are retained. See the
fixture README for the native job and selection rule.

The patch port translates `dec_patch_dictionary.{h,cc}`,
`patch_dictionary_internal.h`, `blending.{h,cc}`, `alpha.{h,cc}`, and
`render_pipeline/stage_patches.{h,cc}`. Genuine original entropy encoders produce
the dictionary fixtures. Observations retain initialized partial dictionary/tree
state and invoke actual original row/stage factories. The translated libc++
non-arithmetic sort preserves tied interval order. Shared Rust stage bindings
retain the existing dictionary and reference owners through their identities.

`tools/oracle/alpha.cc` includes the pristine original file and supplies all six
real public symbols, including complete-file calls. Only that unit uses scoped
O0/no-inline configuration, with strict FP additionally on native x86_64 Linux.
ARM keeps its original contraction. The numeric source audit verifies selected
bindings, 1,944 ordinary/instrumented requests and all 1,632 finite observations
against each untouched optimized archive. Native diagnostics retain complete
source object disassembly and rejected candidate observations. Source revision,
body, dependencies and every sanitizer remain unchanged.

The original zero-width extra-channel weighted-add alias path can pass a null
temporary destination to memcpy at `alpha.cc:70`. All 24 triggering old requests are
retained as explicit original-only probes, with positive native Linux UBSan
diagnostics; no output word from that undefined call is treated as reference
semantics. Exact parity retains all 360 other blend requests, including 72
zero-width cases. Direct zero-count alpha calls have non-null backing. See the
[living oracle decision](adr/0002-differential-oracle.md) for the configuration
and its review triggers.

The YCbCr/chroma component translates the complete scalar stages/factories in
`render_pipeline/stage_ycbcr.{h,cc}` and
`render_pipeline/stage_chroma_upsampling.{h,cc}`, including original channel
order, coefficients, interpolation, borders, shifts and argument guards.
Highway scalar double-promotion/product-narrowing and ordered binary32 additions
retain the selected target mappings recorded in ADRs 0002 and 0003. Only the
already established native Linux x86_64 pristine stage wrappers use scoped O0;
AArch64 remains bound to the untouched optimized original archive. The durable
5,562-request component audit retains all exceptional words and every finite
comparison against both unchanged defaults.

The sequence integration follows `dec_cache.cc::PreparePipeline`: each channel's
horizontal then vertical chroma stages precede filters, and YCbCr conversion
occupies the original color checkpoint before blending/post-color reference
writes. Reconstruction and group buffers retain their existing source-derived
subsampling dimensions and traversal. Existing lifecycle owners and retained
reference identities remain unchanged.

`tools/oracle/chroma_image.cc` supplies only fixture orchestration around real
`jpeg/enc_jpeg_data.cc::ParseJPG`, public `encode.cc` entry points and production
header readers. Twenty-five static baseline JPEG inputs are recompressed into
raw JXL with actual JxlEncoderAddJPEGFrame calls; no container or JPEG metadata
is stored. Both original graphs independently reproduce every wire byte and
every initialized planar float sample. The source audit verifies actual parser,
public encoder, PreparePipeline and selected chroma factory symbol bindings,
retains all complete/partial observations and leaves active objects unchanged.
The fixture manifest and [fixture README](../tests/data/README.md) record input
recipes, sampling, dimensions, tool version and byte hashes. These captures
establish the bounded internal sequence path, not JPEG reconstruction or the
public incremental decoder API.

The extra-channel integration follows the actual metadata admission and channel
paths in image_metadata.{h,cc}, dec_modular.cc, frame_header.cc and
DecFrame/PreparePipeline. Scalar SpotColorStage/factories are translated directly
from render_pipeline/stage_spot.{h,cc}; their immutable coefficient borrow and
selected native Linux scalar operation boundaries retain the reviewed source
configuration in ADRs 0002 and 0003. The original archive and all other target
settings remain unchanged. Source-defined depth, selection, black, CFA, thermal
and optional channels receive no new display transformation.

`tools/oracle/extra_channels.cc` supplies only fixture input/orchestration around
actual original header writers, EncodeFrame/DecodeFrame and external buffer
conversion functions. Its 1x1 mixed-sampling controls preserve genuine entropy
payloads and verify exact zero-change source-writer reproduction. Malformed enum
fixtures mutate actual encoder files and use the original parser to identify the
reached rejected type; no replacement metadata parser supplies their status.
`tools/extra_channels_reference.py` verifies the real source symbol bindings,
source/dependency pins and fixture hashes, and retains independent ordinary and
ASan/UBSan native captures of every initialized final color/extra-channel word.
The dyadic all-plane sample capture is generated from original exports before
any Rust comparison. Error observations use existing reached-write coverage to
exclude source-indeterminate storage, preserving every initialized retained
reference/output sample, metadata field and reader checkpoint.

The Float16 external converter's original typed scanline store requires aligned
row pointers. A separate actual-source census retains every initially introduced
odd-stride fixture, ordinary capture and positive native UBSan diagnostic, with
precise per-request classification in the fixture TSV. Defined single-row and
decode-error requests remain paired comparisons; aligned positive controls add
every fixture/channel/unpremultiplication combination. Existing integer16 byte
stores and float32 memcpy stores retain their odd-stride comparisons unchanged.
No undefined source output is used as a Rust expected word.

The complete geometry corpus stores 128 genuine JPEG inputs and their actual
pinned public JPEG-encoder outputs, plus 20 original-encoder sequence
recipes. The manifest records cjpeg version/options, deterministic source pixels,
separate component scan script, byte hashes and actual original encoder/header
observations. Default validation reproduces JPEG XL directly from stored JPEG
inputs and does not require cjpeg. Orientation/intrinsic variants use the actual
WriteCodestreamHeaders implementation; a zero-change header control and unchanged
frame-payload comparison prevent replacement serialization in the adapter.

The geometry export observer calls actual DecodeFrame and ConvertToExternal,
retaining persistent shared state across all source frames. It performs no image
arithmetic. Independently captured RGBA buffers include unchanged seeded row
padding and trailing guards. The source audit binds those entry points to their
pristine source objects, retains both original graphs' full observations and
checks that diagnostic relinking changes no active binary or compiled object.
