# Port status

This repository contains codestream/frame-header, metadata, geometry, entropy,
and color-profile dependencies, plus standalone modular image encoding/decoding.
It cannot encode or decode JPEG XL images. There are no C++ runtime fallbacks
and no placeholder codec APIs. The source revision is pinned in
[upstream.json](../upstream.json).

## Source mapping

| Upstream source under `lib/jxl/` | Rust module under `src/jxl/` | Boundary |
| --- | --- | --- |
| `dec_bit_reader.h`, `.cc` | `dec_bit_reader.rs` | Reader, checkpoints, native moves, scoped closer; unused private aligned-word helper omitted |
| `enc_bit_writer.h`, `.cc` | `enc_bit_writer.rs` | Bit writes, byte/unaligned appends, nested allotments and charging; custom allocator and padded-buffer transfer unported |
| `field_encodings.h` | `field_encodings.rs` | Fields trait, U32 distributions, raw enum enumeration and validation |
| `fields.h`, `.cc` | `fields.rs` | Bits/U32/U64/F16 coders, bundle visitors, and enum visitor helper |
| `base/bits.h` | `base/bits.rs` | Unsigned 32/64-bit and pointer-width zero counts and logarithms |
| `base/byte_order.h` | `base/byte_order.rs` | Endian loads/stores, float-bit conversions, and order-tag helpers |
| `../include/jxl/types.h` | `types.rs` | Endianness and bit-depth descriptor storage; other public API types unported |
| `base/override.h` | `base/override_option.rs` | Tri-state option storage, boolean conversion, and default behavior for other fixed-underlying enum values |
| `ans_params.h` | `ans_params.rs` | ANS and prefix coding constants |
| `ans_common.h`, `.cc` | `ans_common.rs` | Alias table construction/lookup/prefetch, flat histograms, count precision |
| `enc_huffman_tree.h`, `.cc` | `enc_huffman_tree.rs` | Population trees, depth limiting, canonical symbols, and code-length RLE |
| `huffman_table.h`, `.cc` | `huffman_table.rs` | Two-level lookup tables and in-place count consumption |
| `enc_huffman.h`, `.cc` | `enc_huffman.rs` | Simple/general tree serialization from histograms |
| `dec_huffman.h`, `.cc` | `dec_huffman.rs` | Tree parsing, preserved partial state, and prefix symbol decoding |
| `dec_ans.h`, `.cc` | `dec_ans.rs` | Hybrid integers, histogram parsing, ANS/prefix symbols, LZ77, fast paths, and checkpoints |
| `dec_context_map.h`, `.cc` | `dec_context_map.rs` | Simple/entropy-coded maps, completeness validation, and recursion restriction |
| `inverse_mtf-inl.h` | `inverse_mtf.rs` | Existing scalar Highway specialization; SIMD specializations unported |
| `enc_ans.h`, `.cc` | `enc_ans.rs`, `enc_ans_histogram.rs`, `enc_entropy.rs` | ANS state, packed tokens, normalization, table construction, uint-strategy selection, complete entropy orchestration, streaming histogram output, and token writes; debug-only fuzzer setter unported |
| `enc_ans_simd.h`, `.cc`, `simd_util.h`, `.cc` | `enc_entropy.rs` | Existing scalar token-cost loop, MaxValue, and maximum-vector padding for config selection; other functions/targets unported |
| `enc_context_map.h`, `.cc` | `enc_context_map.rs` | MTF, cost comparison, and simple/entropy-coded context output; VarDCT BlockCtxMap output unported |
| `enc_lz77.h`, `.cc` | `enc_lz77.rs`, `enc_lz77_costs.rs` | Symbol costs, RLE, greedy bucket variants, optimal hash-chain variants, and original method dispatch |
| `enc_ans_params.h`, ForModular in `enc_ans.cc` | `enc_ans_params.rs` | Histogram storage, cost, parameter fields/defaults, speed-tier initialization, uint config, and complete modular entropy-policy selection |
| `enc_cluster.h`, `.cc` | `enc_cluster.rs` | Scalar Condition/entropy/divergence, cluster selection, pair refinement, and canonical reindexing |
| `base/fast_math-inl.h`, `base/rational_polynomial-inl.h` | `base/fast_math.rs` | Existing scalar FastLog2f and its degree-two polynomial; other functions/targets unported |
| `common.h` | `common.rs` | SpeedTier and maximum pass count; other constants unported |
| `../include/jxl/color_encoding.h`, enum masks in `color_encoding_internal.h` | `color_encoding.rs` | Raw color values and external field storage with checked C layout; public API entry points and CMS callback execution unported |
| `../include/jxl/cms_interface.h`, debug callback signature in `encode.h` | `cms_interface.rs`, `enc_params.rs` | Non-owning profile/descriptor storage and callback signatures; invocation, transform lifecycle, and public encoder API remain unported |
| `cms/color_encoding_cms.h` | `cms/color_encoding_cms.rs` | Coordinates, gamma, field storage/conversions, comparisons, and partial setter state; callback-driven ICC parsing unported |
| `color_encoding_internal.h`, `.cc` | `color_encoding_internal.rs` | Field bundles, profile generation, raw ICC storage, static encodings, and non-callback field APIs; CMS callbacks/transforms unported |
| `image_metadata.h`, `.cc`, quantizer bias constants | `image_metadata.rs`, `image_metadata_weights.rs` | Bit depth, extra channels, opsin/custom weights, tone mapping, image/codec metadata, sample/alpha/orientation helpers, and metadata reading; diagnostic output unported |
| `frame_header.h`, `.cc` | `frame_header.rs` | Frame fields, chroma subsampling, blending, animation, progressive passes, names, dimension/reference helpers, order field encoding, and parsing; diagnostic strings unported |
| `enc_params.h` | `enc_params.rs` | All compression parameter storage/defaults, lossless predicates/setter, CMS descriptor setter, and borrowed custom inputs; encoding operations remain separate |
| `enc_progressive_split.h` | `enc_progressive_split.rs` | Pass definitions/mode construction, splitter configuration, and InitPasses with original narrowing and reused-array state; coefficient splitting unported |
| `enc_frame.h`, `.cc` | `enc_frame.rs` | FrameInfo, ParamsPostInit, group-size/count helpers, progressive selection, flags/filter policy, complete MakeFrameHeader, and fixed lossless Thunder configuration/frame ownership with modular pixel preparation; full pixel/section orchestration unported |
| `dec_modular.h`, sample functions in `dec_modular.cc` | `dec_modular.rs` | ModularStreamId constructors/index mapping, custom-float bit decoding, scalar row/RGB conversion, wrapped integer sums, and double-precision conversion; frame decoder and debug descriptions unported |
| `enc_modular.h`, helpers and stream methods in `enc_modular.cc` | `enc_modular.rs` | Native stream-owner construction/init, group channel/crop selection, local palettes/RCT/WP selection, shared/custom/learned tree construction, token generation, global/local entropy output, stream cleanup/indexing, sample/transform helpers, and ComputeEncodingData for the fixed lossless Thunder path; general pixel preparation, VarDCT, frame/section orchestration, and custom runners/allocators unported |
| `EstimateCost` in `enc_modular_simd.cc` | `enc_modular_simd.rs` | Original scalar gradient residual cost, context histograms, integer/fractional accumulation, and final rounding; SIMD kernels/context-map table unported |
| `quant_weights.h`, DC methods in `.cc` | `quant_weights.rs` | QuantTable indices/count, DC member initialization/accessors/setter, and DecodeDC with retained partial state; AC owners/full constructor, weights, and matrix/table APIs unported |
| DC methods in `enc_quant_weights.cc` | `enc_quant_weights.rs` | Complete DC serialization, custom-DC encode/decode round trip, and DC scaling; AC quantization encoding and whole-matrix round trips unported |
| `splines.h` | `splines.rs` | Point comparison, quantized storage/default/copy, and borrowed SplineDataView; spline quantization, entropy, and rendering unported |
| `jpeg/jpeg_data.h`, header-selection helpers in `jpeg/enc_jpeg_data.cc` | `jpeg/jpeg_data.rs`, `jpeg/enc_jpeg_data.rs` | Parsed marker/component/coefficient storage, chroma selection, and JFIF/Adobe/component-ID color selection; JPEG parsing, field serialization, and reconstruction unported |
| `frame_dimensions.h` | `frame_dimensions.rs` | Pixel/block/group dimensions, original rounding/padding, and group rectangles |
| `base/rect.h` | `base/rect.rs` | Unsigned Rect coordinates, crop/intersection, translation, shifts, and extension; signed instantiations and image/row accessors unported |
| `pack_signed.h` | `pack_signed.rs` | Signed packing and pointer-width unpacking |
| `loop_filter.h`, `.cc` | `loop_filter.rs` | Gaborish/EPF parameter fields, defaults, validation, and padding; pixel filter kernels unported |
| `lehmer_code.h` | `lehmer_code.rs` | Original Fenwick-tree encode/decode with u16/u32 permutation storage and caller-owned workspaces |
| `coeff_order_fwd.h` | `coeff_order_fwd.rs` | Coefficient type, order count, and row/column layout helpers |
| `coeff_order.h`, `.cc`, `enc_coeff_order.h`, `.cc` | `coeff_order.rs`, `enc_coeff_order.rs` | Permutation context, tokenization, complete entropy output/parsing, optional output, and ANS final-state validation; AC-strategy coefficient orders unported |
| `toc.h`, `.cc`, `enc_toc.h`, `.cc` | `toc.rs`, `enc_toc.rs` | Section size/permutation output, budget validation, entry cap, parsing, group offsets/reordering, and layer accounting |
| `image.h`, `.cc` | `image.rs` | Plane/Image3 numeric storage, padded row views, shrink/restore, swaps, and native moves; C allocators and MSan-only padding instrumentation unported |
| `image_ops.h`, `.cc` | `image_ops.rs` | Plane/Image3 copies and fills, rectangular/padded copies, Mirror, in-place block padding, and both original scalar downsampling overloads; other header-only arithmetic/conversion/min/max APIs unported |
| `modular/options.h` | `modular/options.rs` | Raw predictors, property types, tree modes/kinds, and all modular option defaults |
| `modular/encoding/context_predict.h` | `modular/encoding/context_predict.rs` | All basic predictors, weighted header/presets/state, reference/property computation, flat-node storage and lookup, and tree/learning/edge-specialized wrappers |
| `modular/encoding/ma_common.h`, `dec_ma.h`, `.cc`, `enc_ma.h`, `.cc` | `modular/encoding/ma_common.rs`, `modular/encoding/dec_ma.rs`, `modular/encoding/enc_ma.rs` | Tree constants/storage, complete entropy-backed tree parsing and validation, BFS tokenization, sample storage/deduplication/quantization, pixel sampling, and original scalar best-split learning |
| `modular/encoding/encoding.h`, `.cc` | `modular/encoding/encoding.rs` | Complete header/channel/modular image decoding, static filtering/flattening, LUTs, local/global trees and entropy tables, dimension validation, transform application, and original truncation/cleanup/finalization behavior; frame orchestration remains separate |
| `modular/encoding/enc_encoding.h`, `.cc` | `modular/encoding/enc_encoding.rs` | Complete standalone generic modular encoder, learned/predefined trees, given-tree ModularCompress and channel residual encoding, original fast paths, overflow checks, and predictor-image scratch writes; frame orchestration remains separate |
| `base/random.h` | `base/random.rs` | Original xorshift state, integer/float/Bernoulli/geometric distributions, and forward shuffle; source argument preconditions retained |
| `modular/modular_image.h`, `.cc` | `modular/modular_image.rs` | Channels, image storage, explicit cloning, empty detection, resize failure state, and reverse transform-stack execution with retained failures; diagnostics/custom runners unported |
| `modular/transform/transform.h`, `.cc`, `squeeze_params.h`, `.cc` | `modular/transform/transform.rs`, `modular/transform/squeeze_params.rs` | Transform/squeeze field storage and serialization, PixelAdd, channel-range validation, and complete MetaApply/Inverse dispatch to scalar null-pool kernels |
| `modular/transform/rct.h`, `.cc`, `enc_rct.h`, `.cc` | `modular/transform/rct.rs`, `modular/transform/enc_rct.rs` | Existing scalar forward/inverse color transforms, both forward call forms, and whole-channel permutations in the null-pool configuration; SIMD and custom runners unported |
| `modular/transform/palette.h`, `.cc`, `enc_palette.h`, `.cc` | `modular/transform/palette.rs`, `modular/transform/enc_palette.rs` | Explicit/implicit/delta lookup, metadata edits, inverse prediction, lossless lookup/set selection, ordered palettes, and original lossy iterations, delta learning, and dithering in defined source configurations |
| `modular/transform/enc_transform.h`, `.cc` | `modular/transform/enc_transform.rs` | Complete forward transform dispatch and optional-output min/max scan in the null-pool configuration |
| Reference libc++ non-arithmetic `std::sort` specialization | `base/libcpp_sort.rs` | Original pivot selection, partitions, insertion loops, equivalent-element order, and heap fallback; source version/hashes retained in `tools/reference/libcxx` |
| `modular/transform/squeeze.h`, `.cc`, `enc_squeeze.h`, `.cc` | `modular/transform/squeeze.rs`, `modular/transform/enc_squeeze.rs` | Original scalar forward/inverse kernels, tendency, default schedules, validation, metadata edits, task spans, and channel replacement; SIMD/custom runners unported |
| `base/matrix_ops.h` | `base/matrix_ops.rs` | Float/double matrix products, vector products, and in-place inverse for disjoint input/output callers |
| `cms/opsin_params.h` | `cms/opsin_params.rs` | Original float constants, constexpr scaling, and A2B cube |
| `cms/transfer_functions.h`, `cms/tone_mapping.h` | `cms/transfer_functions.rs`, `cms/tone_mapping.rs` | Scalar HLG/PQ, Rec. 2408, HLG OOTF, and gamut mapping; SIMD variants unported |
| `cms/jxl_cms_internal.h`, used `base/common.h` helpers | `cms/icc*.rs`, `base/float.rs` | Complete profile generation with default 3D tone mapping, matrices, curves, tags, descriptions, and source MD5; ICC parsing and color transforms unported |
| Highway `hwy/cache_control.h` | `base/cache_control.rs` | Clang reference enablement: T0 on x86_64 with SSE2; source-disabled hint on other targets |
| `enc_fields.cc` | `enc_fields.rs` | Primitive/bundle writes, size/image metadata wrappers, complete codestream-header output, and frame header writing; quantizer writes unported |
| `headers.h`, `.cc` | `headers.rs` | Size, preview, animation, and size-header read wrapper |
| `base/status.h` | `base/status.rs` | Status-code/Result mapping; diagnostics and crash instrumentation unported |
| `padded_bytes.h` | `padded_bytes.rs` | Writer storage subset; other container operations unported |
| `memory_manager_internal.h`, `.cc` | `memory_manager_internal.rs` | Aligned allocation using Rust allocator and source scalar row sizing; custom C callbacks unported |
| `enc_aux_out.h`, `.cc` | `enc_aux_out.rs` | Layer totals and writer charging; image statistics and diagnostic output unported |

Internal storage mappings are specified in the
[representation ADR](adr/0003-rust-representation.md). This table counts
translated boundaries, not whole-file or whole-library completion.

## Evidence

Validated on 2026-11-01 on macOS arm64 with Rust 1.99.0 (stable), Rust 1.85.1
(minimum-version series), and Apple Clang 21.0.0. The C++ oracle is built from
the unmodified pinned source with scalar Highway and release status behavior.
Repository Rust builds disable loop/SLP vectorization to match the upstream
Clang settings; exact float-bit comparisons include unchanged signaling-NaN
copies in release builds.
Upstream CMake builds the complete original static codec library with its pinned
Highway, Brotli, and skcms dependencies. ASan/UBSan cover those built targets as
well as the fixture adapters. Existing private-boundary adapters still include
pristine source files; the original archive supplies the remaining objects.
This reference build adds no C/C++ dependency to ordinary Rust builds and does
not establish completion of the Rust image path.

Command: `python3 tools/check.py --sanitizers --msrv`.

| Gate | Result | Evidence |
| --- | --- | --- |
| Source revision, Highway/Brotli/skcms pins, notices, local doc links | PASSED | Checked against `upstream.json` and the Git submodule entries |
| Rust formatting and Clippy | PASSED | All targets and features, warnings treated as errors |
| Rust debug | PASSED | 239 tests, including 135 differential groups; zero ignored cases |
| Rust release | PASSED | Same 239 tests and corpus |
| Rust 1.85.1 | PASSED | Same 239 tests and corpus |
| C++ ASan and UBSan | PASSED | Fail-fast instrumentation during all three Rust test configurations |
| Rust x86_64 Linux compile | PASSED | `cargo check --locked --offline --all-targets --all-features --target x86_64-unknown-linux-gnu`; no Linux runtime tests claimed |

Each differential run compares **627,615 requests** against C++. These include
all 65,536 binary16 input patterns; primitive encodings; header limits, aspect
ratios and truncations; reader refills and checkpoints; nested accounting;
appends; extension compatibility; default/nested bundle control flow; raw enum
validation; unsigned word/byte helpers; ANS alias tables; and Huffman coding.
Alias comparisons include all 4,096 lookups for every successful generated
table. Huffman comparisons cover exact serialized bytes, depths/canonical
codes, every populated table entry, decoded symbols, consumed-bit counters,
and close status. Cases include depth-limited and tied populations, unsigned
population-sum wrapping, RLE boundaries, unaligned positions, truncated and
damaged streams, and preservation of reused tables on failed parses. Direct
table-builder cases include the 32,768-symbol limit; generated encoder
histograms range from 1 to 4,096 symbols.
Entropy cases cover hybrid-config combinations and extreme integers, complete
ANS/prefix streams generated by the original encoder, damaged/truncated
histograms and context maps, LZ77 lengths/distances, fast paths, and checkpoint
replay across the 1,048,576-entry ring boundary. Rust now constructs its own
encoding tables from the histogram in the token-output comparisons. Additional
normalization cases compare every reciprocal/reverse-map entry, serialized
histograms, all three cost-search strategies, estimates with no output writer,
and exact population-cost float bits. Native tests also round-trip complete
normalized ANS histogram/token sequences without reference tables. Clustering
cases compare source scalar logarithm bits, entropy/divergence, mutable input/output caches,
cluster IDs and order, prior histogram prefixes, limits, and all three
clustering policies. The native prefix-code regression preserves the source's
penalty for merging a large singleton into a multi-symbol histogram.
LZ77 comparisons cover all 16 method settings, force-Huffman choices, changing
contexts, multiple streams, short tails, empty inputs, periodic/random/running
values, special-distance collisions, all cost-table entries, extreme unsigned
inputs, and exact model float bits. A native test expands the output of every
compressing method across multiple streams and widths, including overlapping
copies. The AArch64 cost regression preserves the original compiler's fused
scalar accumulation and final penalty.

The complete entropy encoder is compared through the original
BuildAndEncodeHistograms, EncodeHistograms, and EncodeContextMap entry points.
Cases check chosen uint configs, ANS/prefix selection, mutated token streams,
costs, code tables, exact bytes, streaming/fixed/missing histograms, context
offsets across batches, rejected single-context prior histograms, and layer
accounting. Rust and C++ cross-decode the produced complete entropy streams,
including short/constant streams, multiple contexts, LZ77, and streaming headers.
These streams use the translated encoder's own framing, with no adapter-supplied
entropy envelope. A native test also runs that entire path without the oracle.

Color-profile cases compare complete ICC bytes and profile IDs for RGB, gray,
XYB, HDR, custom primaries/white points, gamma curves, and failures with reused
output storage. Numeric observations include float/double matrix operations,
HLG/PQ signs and intensity targets, tone/gamut mapping with variable ranges,
and curve tables. Text cases compare 32,783 double-bit patterns against the
source's `%g` formatter, including exponent-selection boundaries and signed
NaNs; MD5 cases cross block/padding boundaries. Native known answers and tag
bounds/alignment checks complement the original-source comparisons. The
reference Darwin formatter omits a negative NaN's sign; an exact regression
retains that platform behavior. Other platform runtime formatting remains
unverified, as do SIMD and alternative ICC compile settings.

Metadata comparisons cover color storage/conversion and serialized bundles,
raw ICC selection, bit-depth validation and unsigned offsets, complete
codestream-header bytes/accounting, orientation, preview/animation/intrinsic
sizes, extra-channel byte names, custom transform masks/weights, and truncated
or damaged input. Observations retain private field storage exposed by the
source's View, including partial conversion failures and exact ICC bytes.
Reused-object cases preserve stale extra-channel vectors and names in the
source's default visitor. Native RGB/gray header round trips, upstream-derived
coordinate/gamma scenarios, and a reused-name regression run without C++.
Header completion is not a complete image encode/decode path.

Frame comparisons cover exact header bytes, accounting, decoded fields, and
partial error state across regular, DC, reference-only, and skip-progressive
frames; animation/timecodes, extra channels, cropping, blending, upsampling,
YCbCr subsampling, and Gaborish/EPF parameters. Progressive-pass cases check
decreasing resolution sequences, increasing last-pass indices, resolution
brackets/targets, and rejections. Geometry cases retain pixel/block/group units,
modular versus VarDCT padding, unsigned wrapping, and partial subsampling-setter
mutations. The latter includes signed integer promotion of byte differences
before conversion to pointer width. Native tests join real RGB/gray codestream
metadata to a modular frame header and exercise an optional-metadata reference
frame; they still contain no pixel data.
An exact float-threshold regression rejects the nearest f32 below the source's
double `1e-8` sigma bound. Lowering that comparison to f32 first produced an
incorrectly accepted header, which the independent oracle caught.

Section-table comparisons include original entropy-coded permutations, 16/32-bit
Lehmer workspaces, partial failures, skipped prefixes, structured/random orders,
size-distribution boundaries, the 65,536-entry cap, group offsets/reordering,
unaligned input/output, padding, nested layer charges, and damaged/truncated
tables. Reused outputs retain stale permutations when the source reads no
permutation bit. Native tests adapt the upstream Lehmer size ranges, including
values above 16 bits, and locate distinct section payloads through the decoded
offsets. These payloads are test sections, not an encoded image.

Image-storage comparisons cover row sizing/overflow, 128-byte alignment, scalar
vector padding, avoidance of 2-KiB row aliases, shrink/restore, swaps/moves,
copy/fill boundaries, preserved destination padding, border mirroring, channel
resize errors, explicit image clones, option defaults, and transform fields.
Image3 keeps the first plane's row stride and preserves partial shrink/copy
state when a later plane fails. The scalar source rejects positive widths with
eight-byte samples, even at zero height; this restriction is retained. Empty
planes carry no source allocation context, so reallocating an empty channel to
positive dimensions fails without replacing its old plane. The fixture paints
every observed source byte first; cloned images expose valid samples only,
because C++ leaves their padding uninitialized. Native tests check ownership,
clone independence, float sample bits, padding preservation, and resize state.

RCT comparisons cover all 42 serialized color-transform types, in-place and
separate-buffer forward calls, inverse full-width Highway wrapping, pure channel
moves and component tags, channel/meta-channel validation, zero forward status
versus successful inverse no-op, and uint32 task-range narrowing. Forward
samples stay within the scalar source's defined signed-arithmetic domain;
inverse inputs also exercise int32 extremes. The separate forward overload
retains its different validation behavior. Native known answers, sample round
trips, and padding checks complement the source comparisons. This evidence is
for the original scalar/null-pool path; it does not establish thread-runner or
SIMD behavior.

Prediction comparisons check every weighted error-buffer entry after each
pixel, all presets and basic predictors, integer reciprocal/weight arithmetic,
negative-value rounding, property narrowing, previous-channel filters/scattering,
flat-node layout and two-level traversal, and the complete no-tree/tree/learning
wrappers with and without weighted state. Interior-only variants run alongside
the original boundary handling. Cases include signed 32-bit samples and error
wrapping, absent reference properties, and reused/truncated weighted headers.
The oracle observes only defined result members: the source leaves multiplier
indeterminate without a tree, whereas Rust initializes that storage. Native
tests check signed ties/rounding, raw union storage, parameter-read failure
ordering, and prediction from only the already decoded pixel prefix. This
prefix replay has no bitstream and is not an image codec.

Modular-tree coverage adds 5,444 differential requests: original BFS tokenization
and partial failures, ANS/prefix tree streams with and without RLE, signed offset
narrowing, multiplier limits, property validation, infinite-tree rejection,
2,048-level height checks, size checkpoints, and original-stream cross-decoding.
Truncations and malformed input retain consumed-bit counts, close status, and
partially parsed nodes. Filtering compares static pruning, two-level flattening,
property counts, predictor flags, and every initialized LUT entry, including
partial writes before failure, for u8/u16/u32 contexts and all optional-field
combinations. Group bundles cover nested transforms, reused state, field limits,
and truncation. Native integration tests connect actual tree tokens and entropy
bytes through parsing and static pruning to prediction-context selection; no
image residuals or complete image codestream are supplied by these tests.

Residual coverage adds 6,998 differential requests. Predefined trees retain their
cutoffs, pixel-count height reduction, bit-depth scaling, and reference-property
selection. Given-tree encoding compares all tokens, reused header state, width
output, empty/limited channels, and partially written output on overflow. Raster
tests connect the original residual and entropy entry points, compare bytes,
cross-decode original streams, and observe every output sample, consumed-bit
count, RLE state, and finalization status after truncation or corruption. They
exercise all 14 predictors, static and reference properties, original gradient
and weighted lookup paths, interior-only loops, ANS single-value handling, and
the Huffman/RLE shortcut with a run spanning channel boundaries. The original
global-tree predicate keeps mixed-predictor trees on the general LZ77 reader.
Native tests recover complete raster samples, preserve row padding, compare
fast/slow tokens, and check known tokens and overflow state. These streams lack
codestream/frame framing and transform orchestration; they are not yet a
complete JPEG XL image path.

Palette/Squeeze/stack coverage adds 29,367 differential requests. Palette tests
cover every delta-table entry and implicit cube color, component/stride lookup,
signed extrema, unsigned scaling, all inverse predictors, channel insertion and
removal, meta-channel arithmetic, and zero-width/empty-palette handling. The
source Clamp1 checks the low bound first; an empty single-channel palette has
reversed bounds and retains that branch-dependent result. Squeeze tests compare
signed tendency/rounding, default schedules, forward scalar inputs within the
source's nonoverflowing range, inverse int32 extrema, eight-row/64-column task
boundaries, corrupted geometry, and partial metadata changes. Dispatch/stack
tests verify retained failed transforms, popped successes, and combined RCT/
Squeeze sample restoration. Metadata-only calls omit undefined newly allocated
samples. Native tests check known palette colors, the empty-palette regression,
RGB expansion, odd/negative Squeeze samples, partial failure state, and stack
ownership/order. Forward palette selection has additional evidence below.

Modular image orchestration adds 3,128 differential requests. The original C++
ModularGenericCompress supplies standalone streams for grayscale/RGB, 8-/16-bit
samples, multiple dimensions, constants, ramps, noise, learned/predefined trees,
and RCT/Palette/Squeeze chains. Full Rust decodes check every original sample,
requested shape/channel count, consumed-bit count, close status, and transform
completion. Global-table fixtures call the source tree/token/histogram writers
and ModularCompress; missing/empty global references are also tested. Malformed
and truncated cases compare header/transform mutations, channel data where
defined, strict zero-filling, allowed-truncation state, dimension limits, empty
work, and error-check order. Metadata-only observations omit indeterminate source
storage after allowed truncated transforms. Native tests decode two fixed
original streams without C++, and verify header/global-reference cleanup.
Standalone modular streams require caller-supplied dimensions and bit depth;
they do not include JPEG XL codestream/frame headers. Frame orchestration
remains unported.

Tree learning and generic modular encoding add 2,593 differential requests.
RNG cases compare every distribution draw and shuffle state. Sample cases
compare every quantization-map entry, mutated sample arrays, residual tokens,
hash growth/shrink behavior, and u16 count saturation. Split cases compare exact
entropy float bits, forced multiplier ranges, resulting trees, and sample
partition order. Image batches cover sampling fractions, reference properties,
all tree modes, multiple predictors/groups, empty ranges, and failed setters.
Generic encoder cases compare complete bytes, bit counts, both accounting
layers, and decoded rasters across all predefined/learned tree kinds, entropy
settings, prefixes, RCT/Squeeze, and partial output on errors. Native tests
reproduce the fixed C++ gray fixture and round-trip learned gray/RGB streams
with reference properties and odd dimensions. These streams still require
caller-supplied dimensions and bit depth; they are not complete JPEG XL files.

Forward palette selection and dispatch add 6,729 differential requests.
Cases compare all color-distance float bits, symmetric rounding, implicit cube
quantization, exact tied-element sort order and forced heap fallback, complete
palette/index planes, optional min/max output, failed-call mutations, and
inverse reconstruction. Lossy cases include both iterations, learned deltas,
error diffusion, weighted presets, and RGB configurations at multiple bit
depths. For >=8-bit lossy palettes, a generated four-channel case exposed
indeterminate entries in the source's fourth delta row; observations use the
source frame encoder's three-color-channel configuration there. No new source
validation or fourth-row meaning is inferred. Standalone integration compares
Palette/Squeeze plus generic encoder bytes, accounting, and decoded samples;
the native encoder also reproduces the previously captured C++ RGB fixture.
The translated libc++ specialization has its own source snapshots and LLVM
notices. These results establish the reference configuration's sort order,
not arbitrary C++ standard-library versions or other specializations.

Encoder parameter/header preparation adds 13,128 differential requests.
They compare every defined default parameter, lossless setters/predicates,
post-initialization mutation/error order, all modular entropy policy choices,
group geometry, wrapped stream indices, progressive modes and reused arrays,
filter overrides/NaN bits, JPEG marker precedence/subsampling failures, spline
point thresholds, and complete prepared headers and bytes. Descriptor layout
checks compare sizes, alignments, and member offsets against the actual C types.
The oracle includes unmodified `enc_frame.cc` to call its private preparation
helpers and compiles original JPEG data/helper sources for their real types and
functions. Its pinned Brotli headers are a C++ reference prerequisite only.
Native tests check copied owners/borrowed views, inactive callback descriptors,
failed-call partial state, progressive array preservation, and lossless RGB/gray
header I/O. No callback execution, complete frame/group pixel codec, or complete
JPEG XL image is established by these preparation tests.

Modular frame sample conversion adds 11,007 differential requests, including
every binary16 bit pattern, both conversion directions using original decoded
floats, all supported exponent/mantissa widths, signed zero, NaN payloads,
integer rounding thresholds, wrapped scalar sums, single/double precision, and
untouched output suffixes. Custom-float failures compare already written
prefixes and unchanged remaining storage. Early format failures precede buffer
access, including when no row storage is supplied. The separate oracle
translation unit includes unmodified `enc_modular.cc` and `dec_modular.cc`; unused frame methods
are discarded, without substitute definitions. Native tests recover all 8-bit
and 16-bit integer samples through the source's row conversions, round-trip all
binary16 patterns, and check raw binary32 copies and partial errors. These are
sample boundaries, not complete JPEG XL encode/decode evidence.

Modular frame selection helpers add 5,714 differential requests. They compare
gradient/weighted cost bits, edge dimensions and predictor modes, weighted
overflow rejection, quantization over physical planes with different logical
dimensions, append-only tree merging and child-offset narrowing, transform
attempts and cost-based rollback, and global/all-but-one/single-channel palette
selection. Fixtures preserve original slow/fast tier decisions, signed palette
limits, compaction percentages, and metadata-only force modes. Undefined source
gradient residual overflow, invalid numeric conversions, and indeterminate
force-mode pixels are outside observations. New transform observations omit
the source's indeterminate max_delta_error field. Native tests check merge
direction/offsets, original pixel restoration after rejection, retained fast
transforms, palette undo, physical-plane quantization, and weighted rejection.
These helpers also feed the translated frame stream state below.

Modular frame stream state adds 1,616 differential requests. Constructor cases
compare copied parameters, metadata/group geometry, all speed/predictor/property
policies, raw float bits, and unsigned threshold arithmetic. Action sequences
compare defined stream state, entropy-table sizes/maps, channels and transformations,
learned/predefined/custom trees, multiple useful/empty tree chunks, quantization
ranges, heterogeneous channel shifts/crops, tokens, headers, encoded
bits/bytes, and all layer accounting. They cover repeated tree/token work,
cleanup with retained mappings/tokens, generic streams, original streaming-tile
local streams, and partial image/token/tree errors. Shared entropy follows the
source's non-streaming lifecycle; streaming fixtures omit shared tree/token
generation as the original enc_frame.cc caller does. Group options are seeded
at that caller's input checkpoint. Entropy-table contents are exercised through
serialized output; indeterminate entropy scalars and transform scratch fields
are omitted from direct observations. Observation pads
a scratch copy, preserving the real writer's position and accounting.

The oracle calls unmodified ModularFrameEncoder methods and uses compiler
access-control support only in its fixture translation unit. Native tests
check copied owners, progressive policy, cropped/shifted group selection,
cleanup, retained error prefixes/widths, and independently captured C++ modular
section bytes. Those 42-bit zero-gray sections do not include codestream/frame
headers or a section table. General ComputeEncodingData, actual frame entry
points, frame decoding, and the complete first-image acceptance gate remain
unported.

The fixed lossless Thunder path adds 356 differential requests at the original
ComputeEncodingData boundary. Cases compare converted samples, default global
palette/compaction and RCT selection, metadata/transform order, all stream
owners/options, group schedules/mappings, plane release, and unchanged Image3F
input bits. They cover gray/RGB, 8/16-bit integers, constants, ramps, restricted
palettes, deterministic noise, edge groups, and multiple DC groups. Extra input
rows/columns contain NaNs and remain unread. Invalid dimensions fail at the
original SizeHeader boundary before pixel access, including its uint32 cap.
Native tests invert the selected transforms to recover integer samples and
check retained logical dimensions after releasing copied global planes.
The configuration owns private metadata and its frame exposes read-only
observations, preventing selection of unported branches through this path.
Input is already Image3F; CopyColorChannels, outer frame preparation,
section orchestration, complete codestream output, and frame decoding remain
separate dependencies.

Image preparation helpers add 739 differential requests: in-place block padding
and both original downsampling overloads. Padding compares all caller-seeded
capacity bytes, exact float payloads, unchanged row padding, repeated calls,
insufficient capacity, and partial plane-size changes. Downsampling compares
exact accumulated float bits, actual edge-block sample counts, scalar/three-plane
overloads, infinities/NaNs/subnormals, factor-one failures, and unsigned rounding
at large factors. Empty-image cases distinguish missing allocation owners from
allocated images shrunk to zero; the output inherits that source allocator
boundary. Native tests check repeated edge pixels, unchanged allocation addresses
and row padding, known edge averages, and the empty-owner allocation regression.
Connecting these helpers to the outer frame path remains separate work.

DC quantization adds 2,548 differential requests through the original setters,
DC encoder/decoder, custom round trip, and scaling. Action chains compare exact
DC/inverse float bits, reused default-flag behavior, unaligned bytes and bit
counts, layer accounting, consumed bits, close status, and changes retained
after errors. They include special float factors, binary16 validation thresholds,
invalid coefficients in each component, truncated/corrupt input, failed custom
round trips, and recovery on reused state. Native tests independently check the
one-bit default encoding and the per-component mutation/inverse checkpoint.
The signaling-NaN copy regression runs in debug and release with the repository's
source-matched scalar compiler configuration.
Only DC storage is represented; this evidence does not establish the full
DequantMatrices constructor, AC owners, or VarDCT matrix computation.

The fixture adapter's outer entropy framing is test infrastructure and is not
an image encoder. Failed allocations/table builds are not observed through
uninitialized C++ alias entries.
The native writer test additionally round-trips 1,048,576 generated bit patches.

Cold review caught a prefetch configuration mismatch: translating only the
function body added an AArch64 hint even though the source's top-level SSE2
guard disables it there. The Rust mapping now preserves that disabled branch;
bitstream parity alone would not have exposed this unintended optimization.

The default-bundle test first failed on an early Rust return after a truncated
`AllDefault` predicate: C++ consumed five bits and updated the next field, while
Rust consumed one bit and left that field unchanged. The corrected predicate
mapping passes that regression; see
[ADR 0003](adr/0003-rust-representation.md).

The pinned Highway CMake file emits a CMP0111 deprecation warning. It does not
fail the build; the reference source has not been edited to suppress it.

Not yet established: complete image decoding/encoding, public C ABI compatibility,
thread runners, SIMD parity, full upstream test inventory, external conformance
corpus, cross-platform CI results, fuzzing, Rust sanitizer/Miri results, or
performance/memory parity. Test passing here does not establish any of those.
The Linux/macOS workflow is configured but has not run on a hosted service.

## Next dependency boundaries

The active [first-image milestone](FIRST_IMAGE.md) requires a complete Rust
image encode/decode path and cross-decoding with the original implementation.
The entropy slices above do not complete it.

1. Complete the source ownership/constructor dependencies reached by the fixed
   path, including DequantMatrices AC-owner initialization. Connect the remaining
   outer pixel preparation, frame decoding, and image/frame/section entry points,
   then run the cross-language image
   corpus in the first-image acceptance gate.
2. Expand through remaining modular and VarDCT decoding and encoding before
   introducing the C ABI and optimized target-specific kernels.

Preserve the upstream test inventory as that scope grows. Adapted tests in this
first slice do not replace the unported image, threading, allocation-failure,
platform, or codec test suites.
