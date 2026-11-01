# Port status

This repository contains codestream/frame-header, metadata, geometry, entropy,
and color-profile dependencies, standalone modular image encoding/decoding,
and a fixed lossless Thunder encoder from pixel buffers to complete raw JPEG XL
codestreams, plus complete Rust parsing and integer decoding for that same fixed
profile. Rust and the original C++ decoder recover the exact samples from both
native and reference codestreams. The first-image milestone is verified; general
image/frame decoding and the complete library remain unported.
There are no C++ runtime fallbacks and no placeholder codec APIs. The source revision is pinned in
[upstream.json](../upstream.json).

## Source mapping

| Upstream source under `lib/jxl/` | Rust module under `src/jxl/` | Boundary |
| --- | --- | --- |
| `dec_bit_reader.h`, `.cc` | `dec_bit_reader.rs` | Reader, checkpoints, native moves, scoped closer; unused private aligned-word helper omitted |
| `enc_bit_writer.h`, `.cc` | `enc_bit_writer.rs` | Bit writes, byte/unaligned appends, nested allotments/charging, and TakeBytes allocation transfer; custom allocator unported |
| `field_encodings.h` | `field_encodings.rs` | Fields trait, U32 distributions, raw enum enumeration and validation |
| `fields.h`, `.cc` | `fields.rs` | Bits/U32/U64/F16 coders, bundle visitors, and enum visitor helper |
| `base/bits.h` | `base/bits.rs` | Unsigned 32/64-bit and pointer-width zero counts and logarithms |
| `base/byte_order.h` | `base/byte_order.rs` | Endian loads/stores, float-bit conversions, and order-tag helpers |
| `base/float.h` | `base/float.rs` | Complete LoadFloat16 and callback-based LoadFloatRow, including unsupported-format failure before access |
| `enc_external_image.cc` | `enc_external_image.rs` | Complete null-pool ConvertFromExternalPlaneNoSizeCheck and checked plane ConvertFromExternal; ImageBundle and BufferToImage wrappers unported |
| `dec_external_image.h`, `.cc`, used `alpha.cc` loop | `dec_external_image.rs` | Complete integer and Float16/Float32 buffer/null-pool specializations of ConvertChannelsToExternal and ConvertToExternal, original scalar FloatToU32/FloatToF16 with selected native-half-cast mapping, generic UndoOrientation, store order, nullable channels, checked output spans, and alpha unpremultiplication; callbacks, custom runners, and alternate Highway half-conversion compile branches unported |
| `encode_internal.h` | `encode_internal.rs` | BitsPerChannel, BytesPerPixel, and the buffer-backed adapter/channel construction, binding, copying, views, and format accessors; callback/JPEG input, CopyFromBuffer, queued frames, and output processors unported |
| `../include/jxl/types.h` | `types.rs` | Raw endianness/data-type values, pixel-format layout, and bit-depth descriptor storage; other public API types unported |
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
| `base/fast_math-inl.h`, `base/rational_polynomial-inl.h` | `base/fast_math.rs`, `cms/transfer_functions_scalar.rs` | Existing one-lane FastLog2f/FastPow2f/FastPowf and degree-two/degree-four rational evaluation with original x86 fence sites; other functions/targets unported |
| Highway `hwy/ops/scalar-inl.h`, `generic_ops-inl.h` | `base/hwy_scalar.rs` | Used one-lane Min/Max, unfused MulAdd, Clamp, Floor, saturating float-to-i32 conversion, and sign-bit ZeroIfNegative; other operations/targets unported |
| `common.h` | `common.rs` | SpeedTier and maximum pass count; other constants unported |
| `ac_strategy.h`, `.cc` | `ac_strategy.rs` | Raw AcStrategyType values, size/count constants, and AcStrategyImage allocation/fills/validity/counting; strategy objects, interpreted row views, geometry/transform methods, and strategy setters unported |
| `chroma_from_luma.h`, `.cc` | `chroma_from_luma.rs` | Complete ColorCorrelation constants/defaults, ratios, setters/getters, DC parsing, and ColorCorrelationMap allocation/default/move ownership/delegated DC parsing |
| `quantizer.h`, `.cc` | `quantizer.rs` | Constructors, RecomputeFromGlobalScale, DC scaling getters, inverse AC scaling, and ClearDCMul; quantization fields, scale selection, parsing, and other APIs unported |
| `../include/jxl/color_encoding.h`, enum masks in `color_encoding_internal.h` | `color_encoding.rs` | Raw color values and external field storage with checked C layout; public API entry points unported |
| `../include/jxl/cms_interface.h`, debug callback signature in `encode.h` | `cms_interface.rs`, `enc_params.rs` | Non-owning profile/descriptor storage and callback signatures; ICC/transform invocation is translated below; default CMS backend and public encoder API remain unported |
| `cms/color_encoding_cms.h` | `cms/color_encoding_cms.rs` | Coordinates, gamma, field storage/conversions, comparisons, partial setter state, and callback-backed ICC field extraction; default ICC parser unported |
| `color_encoding_internal.h`, `.cc`, SetFieldsFromICC in `cms/color_encoding_cms.h` | `color_encoding_internal.rs`, `color_space_transform.rs`, `cms/color_encoding_cms.rs` | Field bundles, profile generation, raw ICC storage, static encodings, callback SetICC/SetFieldsFromICC with conditional byte transfer, complete callback-backed ColorSpaceTransform and destruction; default CMS backend and C ABI entry points unported |
| `image_metadata.h`, `.cc`, quantizer bias constants | `image_metadata.rs`, `image_metadata_weights.rs` | Bit depth, extra channels, opsin/custom weights, tone mapping, image/codec metadata, sample/alpha/orientation helpers, and metadata reading; diagnostic output unported |
| `frame_header.h`, `.cc` | `frame_header.rs` | Frame fields, chroma subsampling, blending, animation, progressive passes, names, dimension/reference helpers, order field encoding, and parsing; diagnostic strings unported |
| `enc_params.h` | `enc_params.rs` | All compression parameter storage/defaults, lossless predicates/setter, CMS descriptor setter, and borrowed custom inputs; encoding operations remain separate |
| `enc_progressive_split.h` | `enc_progressive_split.rs` | Pass definitions/mode construction, splitter configuration, and InitPasses with original narrowing and reused-array state; coefficient splitting unported |
| `enc_frame.h`, `.cc` | `enc_frame.rs` | FrameInfo, ParamsPostInit, group-size/count helpers, progressive selection, flags/filter policy, complete MakeFrameHeader, and fixed lossless Thunder CopyColorChannels/ComputeEncodingData/EncodeFrameOneShot through byte-owner handoff; reached encode.cc metadata/padding block connects complete raw codestream output; general frame/image encoding, C output processors/containers, and remaining entry points unported |
| `passes_state.h`, `.cc`, `enc_cache.h` | `passes_state.rs`, `enc_cache.rs` | Complete shared/encoder storage and constructors, InitializePassesSharedState allocation/error order, four reference owners, same-owner DC/patch/frame bindings; remaining VarDCT methods unported |
| `dec_cache.h`, InitForAC in `.cc`, callback typedefs in `lib/include/jxl/decode.h` | `dec_cache.rs` | Complete decoder-state storage/constructor/Init, own or borrowed shared selection, original coefficient owner, PixelCallback storage/presence/unsafe Init, ImageOutput carriers, null-pool InitForAC, complete GroupDecCache constructor storage and InitDCBufferOnce; fixed PreparePipeline is connected below, general preparation, VarDCT cache initialization, output lifecycle, and custom runners unported |
| `dec_frame.h`, `.cc`, reached PreparePipeline in `dec_cache.cc` | `dec_frame.rs` | Closed lossless Thunder raw-frame DecodeFrame/FrameDecoder path: complete frame header/TOC parsing, output initialization, global/DC/AC scheduling, original simple pipeline preparation, cache/thread setup, section-reader close lifecycle, and modular finalization; explicit profile admission; general/progressive frame APIs, JPEG, and VarDCT orchestration unported |
| DecodeFile in `render_pipeline/render_pipeline_test.cc`, CheckMetadata in `../extras/codec_in_out.h` | `dec_frame.rs`, private admission in `enc_frame.rs` | Whole fixed-profile codestream parsing, owned metadata, pre-padding output initialization, exact frame dispatch, EOF/bounds/metadata/close checks, and connected original integer export; preview/animation/ICC/general profiles remain outside admission |
| `opsin_params.h`, `.cc`, output-encoding methods in `dec_xyb.h`, `.cc` | `opsin_params.rs`, `dec_xyb.rs` | Pinned default-matrix selection, four-lane inverse-matrix expansion, OpsinParams initialization, CanOutputToColorEncoding, complete OutputEncodingInfo storage and metadata/desired-profile setters; general image conversion and fast XYB output unported |
| `image_bundle.h`, selected methods in `.cc` | `image_bundle.rs` | Storage/constructors, metadata/color/extra/size access, SetFromImage conditional ownership transfer, VerifySizes/VerifyMetadata, alpha presence/premultiplication/plane access, and decoded-byte accessors; general import/export/copy unported |
| `render_pipeline/render_pipeline_stage.h`, `.cc`, write stages in `stage_write.cc` | `render_pipeline/render_pipeline_stage.rs` | Complete stage settings/base hooks/row access and WriteToImage3F/WriteToImageBundle stages; pixel-buffer output/callback stage and remaining concrete stages unported |
| `render_pipeline/render_pipeline.h`, `.cc`, `simple_render_pipeline.h`, `.cc` | `render_pipeline/simple_render_pipeline.rs` | Explicit simple-builder selection, base input/counter/thread-preparation operations, and complete simple storage, mirroring, stage scheduling, clipping, padding, and dimension transition; default low-memory implementation and concurrency unported |
| `render_pipeline/stage_gaborish.h`, `.cc` | `render_pipeline/stage_gaborish.rs` | Complete original one-lane stage/factory, channel-specific weight normalization, three-row 3x3 kernel, extra spans, symmetric border settings, and ignored extra channels; SIMD targets and full lossy frame integration unported |
| `render_pipeline/stage_upsampling.h`, `.cc` | `render_pipeline/stage_upsampling.rs` | Complete original one-lane stage/factory, symmetric weight expansion, per-thread aligned workspaces, min/max preprocessing, chunked 2x/4x/8x interpolation/clamping, and original argument guards; SIMD specializations and sanitizer annotations unported |
| `render_pipeline/stage_cms.h`, `.cc` | `render_pipeline/stage_cms.rs` | Complete factory/IsNeeded, profile copy/selection, input-size validation, transform preparation, RGB/gray/CMYK interleaving, run-status propagation, deinterleaving, and destruction; native unsafe construction carries the foreign callback contract, and source caller preconditions are checked before buffer access |
| `render_pipeline/stage_tone_mapping.h`, `.cc` | `render_pipeline/stage_tone_mapping.rs` | Complete one-lane factory/stage, original no-op/guard order, PQ scaling, Rec. 2408/HLG selection, per-pixel tone/gamut mapping, and in-place RGB stores; other SIMD targets unported |
| `dec_xyb-inl.h`, normal stage/factory in `render_pipeline/stage_xyb.cc` | `dec_xyb_scalar.rs`, `render_pipeline/stage_xyb.rs` | Complete original one-lane XybToRgb, normal XYBStage/GetXYBStage, scaled XYB output, copied opsin/profile selection, original xextra guards, and in-place color/ignored extra-channel modes; general image conversion, FastXYB/sRGB8, SIMD, and decoder-profile expansion remain unported |
| `dec_patch_dictionary.h`, `splines.h`, `noise.h` | `dec_patch_dictionary.rs`, `splines.rs`, `noise.rs` | Feature/reference storage, constructors, spline Clear/getters, patch Clear/HasAny/same-owner SetShared, and noise Clear/HasAny; nonempty patch-tree construction, parsing/rendering and remaining operations unported |
| `dct_util.h`, `ac_context.h` | `dct_util.rs`, `ac_context.rs` | Complete two-type ACImage owners/virtual operations and BlockCtxMap storage/defaults; context lookup helpers unported |
| `dec_modular.h`, selected functions in `dec_modular.cc` | `dec_modular.rs` | ModularStreamId constructors/index mapping, sample conversion, complete ModularFrameDecoder storage/constructor/Init/DecodeGlobalInfo/MaybeDropFullImage/DecodeGroup, modular-to-pipeline conversion, null-pool/simple-pipeline FinalizeDecoding, and state accessors; full decoder orchestration, VarDCT/quantization-table decoding, and debug descriptions unported |
| `enc_modular.h`, helpers and stream methods in `enc_modular.cc` | `enc_modular.rs` | Native stream-owner construction/init, group channel/crop selection, local palettes/RCT/WP selection, shared/custom/learned tree construction, token generation, global/local entropy output, stream cleanup/indexing, sample/transform helpers, and ComputeEncodingData for the fixed lossless Thunder path; general pixel preparation, VarDCT, frame/section orchestration, and custom runners/allocators unported |
| `EstimateCost` in `enc_modular_simd.cc` | `enc_modular_simd.rs` | Original scalar gradient residual cost, context histograms, integer/fractional accumulation, and final rounding; SIMD kernels/context-map table unported |
| `quant_weights.h`, constructor and DC methods in `.cc` | `quant_weights.rs` | Distance-band and owning quantization-encoding constructors, active union storage, deep copies/assignment/moves, strategy table mapping, complete initialized DequantMatrices constructor state, SetEncodings, and DC APIs; non-owning raw base copies, coefficient libraries, matrix computation/accessors, and AC table algorithms unported |
| DC methods in `enc_quant_weights.cc` | `enc_quant_weights.rs` | Complete DC serialization, custom-DC encode/decode round trip, and DC scaling; AC quantization encoding and whole-matrix round trips unported |
| `splines.h` | `splines.rs` | Point comparison, quantized storage/default/copy, and borrowed SplineDataView; spline quantization, entropy, and rendering unported |
| `jpeg/jpeg_data.h`, header-selection helpers in `jpeg/enc_jpeg_data.cc` | `jpeg/jpeg_data.rs`, `jpeg/enc_jpeg_data.rs` | Parsed marker/component/coefficient storage, chroma selection, and JFIF/Adobe/component-ID color selection; JPEG parsing, field serialization, and reconstruction unported |
| `frame_dimensions.h` | `frame_dimensions.rs` | Pixel/block/group dimensions, original rounding/padding, and group rectangles |
| `base/rect.h` | `base/rect.rs` | Unsigned Rect coordinates, crop/intersection, translation, shifts, and extension; signed instantiations and image/row accessors unported |
| `pack_signed.h` | `pack_signed.rs` | Signed packing and pointer-width unpacking |
| `loop_filter.h`, `.cc` | `loop_filter.rs` | Gaborish/EPF parameter fields, defaults, validation, and padding; scalar decoder Gaborish stage tracked above, EPF kernels unported |
| `lehmer_code.h` | `lehmer_code.rs` | Original Fenwick-tree encode/decode with u16/u32 permutation storage and caller-owned workspaces |
| `coeff_order_fwd.h` | `coeff_order_fwd.rs` | Coefficient type, order count, and row/column layout helpers |
| `coeff_order.h`, `.cc`, `enc_coeff_order.h`, `.cc` | `coeff_order.rs`, `enc_coeff_order.rs` | Coefficient storage-size limits, permutation context, tokenization, complete entropy output/parsing, optional output, and ANS final-state validation; AC-strategy coefficient orders unported |
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
| `cms/transfer_functions-inl.h`, `cms/tone_mapping-inl.h` | `cms/transfer_functions_scalar.rs`, `cms/tone_mapping_scalar.rs` | Complete original one-lane TF_PQ rational approximation and derived Rec2408ToneMapper/HlgOOTF/GamutMap, including scene-light factories and original base constructors; other transfer functions/SIMD targets unported |
| `cms/jxl_cms_internal.h`, used `base/common.h` helpers | `cms/icc*.rs`, `base/float.rs` | Complete profile generation with default 3D tone mapping, matrices, curves, tags, descriptions, and source MD5; default ICC parser/CMS transform backend unported |
| Highway `hwy/cache_control.h` | `base/cache_control.rs` | Clang reference enablement: T0 on x86_64 with SSE2; source-disabled hint on other targets |
| `enc_fields.cc` | `enc_fields.rs` | Primitive/bundle writes, size/image metadata wrappers, complete codestream-header output, and frame header writing; quantizer writes unported |
| `headers.h`, `.cc` | `headers.rs` | Size, preview, animation, and size-header read wrapper |
| `base/status.h` | `base/status.rs` | Status-code/Result mapping; diagnostics and crash instrumentation unported |
| `padded_bytes.h` | `padded_bytes.rs` | Writer storage, native slice/length access, and transferred output ownership; other container operations unported |
| `memory_manager_internal.h`, `.cc` | `memory_manager_internal.rs` | Aligned allocation using Rust allocator and source scalar row sizing; custom C callbacks unported |
| `enc_aux_out.h`, `.cc` | `enc_aux_out.rs` | Complete statistics storage/defaults, layer/object assimilation, totals, and writer charging; formatted diagnostic output unported |

Internal storage mappings are specified in the
[representation ADR](adr/0003-rust-representation.md). This table counts
translated boundaries, not whole-file or whole-library completion.

## Evidence

Validated on 2026-11-01 on macOS arm64 with Rust 1.99.0 (stable), Rust 1.85.1
(minimum-version series), and Apple Clang 21.0.0. The C++ oracle is built from
the unmodified pinned source with scalar Highway and release status behavior.
Repository Rust builds disable loop/SLP vectorization to match the upstream
Clang settings; exact float-bit comparisons include unchanged signaling-NaN
copies in release builds. Quantizer recomputation retains its original call
boundary in Rust and in the C++ quantizer.cc so inlining cannot fold arithmetic
on signaling NaNs into copies differently between reference builds.
Noise HasAny observations retain their early-return loop in a separate unit
with loop unrolling disabled, as documented in ADR 0002.
Upstream CMake builds the complete original static codec library with its pinned
Highway, Brotli, and skcms dependencies. ASan/UBSan cover those built targets as
well as the fixture adapters. Existing private-boundary adapters still include
pristine source files; the original archive supplies the remaining objects.
This reference build adds no C/C++ dependency to ordinary Rust builds and does
not establish completion of the Rust image path.

Commands: `python3 tools/check.py --sanitizers --msrv` and
`python3 tools/check.py --msrv` (ordinary C++ reference).

| Gate | Result | Evidence |
| --- | --- | --- |
| Source revision, Highway/Brotli/skcms pins, notices, local doc links | PASSED | Checked against `upstream.json` and the Git submodule entries |
| Rust formatting and Clippy | PASSED | All targets and features, warnings treated as errors |
| Rust debug | PASSED | 344 tests, including 195 differential test groups; zero ignored cases |
| Rust release | PASSED | Same 344 tests and corpus |
| Rust 1.85.1 | PASSED | Same 344 tests and corpus |
| C++ ASan and UBSan | PASSED | Fail-fast instrumentation during all three Rust test configurations |
| Ordinary C++ reference | PASSED | The same 344 tests and 744,124 requests in debug, release, and Rust 1.85.1 |
| Rust x86_64 Linux compile | PASSED | `cargo check --locked --offline --all-targets --all-features --target x86_64-unknown-linux-gnu`; component runtime evidence below does not establish the full Linux suite |

Each differential run compares **744,124 requests** against C++. These include
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
headers or a section table. These component cases do not establish general
ComputeEncodingData or frame decoding. The fixed-profile caller is connected
below; those component requests alone do not establish complete image decoding.

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
This component accepts Image3F. Its CopyColorChannels, outer preparation, and
complete raw encoder callers are now connected below. Frame decoding remains
unported.

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
DC evidence does not establish VarDCT matrix computation.

Quantization constructor/ownership state adds 509 differential requests. The
oracle observes the original initialized fields, all 81 strategy offsets, default
library entries, all active quantization modes, distance-band constructors,
exact float payloads, and reused encoding vectors. RAW cases compare retained
input vectors, deep-copy isolation, replacement of old owners, moved-from null
owners, copies of moved-from values, empty tables, and the sign bit reached by
shift 31. Matrix setters invalidate seeded masks
without reading coefficient storage. Indeterminate coefficient pointers and
inactive union bytes are omitted. Native tests check known block offsets, copied
table independence, null-versus-empty ownership, and zeroed band suffixes.
Coefficient libraries, EnsureComputed, AC table algorithms, and actual matrix
accessors remain unported.

Fixed-profile group output adds 223 differential requests through the original
EncodeGroups. Cases join pixel preparation, shared trees/tokens, DC quantization,
global modular data, DC/AC groups, and final byte padding. They compare every
section's bytes/bit count, defined modular state, all statistics, null/present
auxiliary output, and the nonempty-vector guard. The corpus covers gray/RGB,
8/16-bit samples, ramps/noise/constants, partial edge groups, multiple DC groups,
and repeated calls that stop at the original guard. A native regression captures
the original seven-byte 1x1 zero-gray group output; it still has no frame header
or TOC. Another checks empty DC/global-AC slots and AC group indexing.

Auxiliary assimilation adds 240 differential requests covering every statistics
field, layer-only and whole-object operations, self inputs, unsigned wrapping,
signed iteration counts within the defined range, and exact double bits for
zeros, infinities, and NaNs. The native counter regression independently checks
unsigned wrap and signed-zero addition. General streaming/VarDCT/effect group
encoding remains absent; the fixed profile cannot select those branches.

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

Not yet established: general Rust image decoding and encoding,
public C ABI compatibility,
thread runners, SIMD parity, full upstream test inventory, external conformance
corpus, cross-platform CI results, fuzzing, Rust sanitizer/Miri results, or
performance/memory parity. Test passing here does not establish any of those.
The Linux/macOS workflow is configured but has not run on a hosted service.

## Next dependency boundaries

The [first-image milestone](FIRST_IMAGE.md) is verified for its fixed original
configuration. The component slices alone are distinct from that whole-file
evidence. Subsequent full-port boundaries include the default low-memory
pipeline, remaining modular and VarDCT paths, other image/frame configurations,
ICC/default CMS integration, progressive/streaming APIs, the public C ABI,
threading, and target-specific kernels. Full-library and release gates remain
in [PORTING.md](PORTING.md).

Preserve the upstream test inventory as that scope grows. Adapted tests in this
first slice do not replace the unported image, threading, allocation-failure,
platform, or codec test suites.

Frame section finalization adds 230 differential requests at the local-writer
boundary in EncodeFrameOneShot. The fixture calls the original header,
PermuteGroups, TOC, and append stages. It compares complete section bytes,
header mutations, and all auxiliary statistics with and without accounting.
Unaligned group writers fail after the header and before TOC output; coverage
also crosses all three TOC size distribution transitions with real payloads.
The private profile retains centerfirst=false, which returns at PermuteGroups'
first guard. The returned writer precedes TakeBytes and AppendData; this is
frame finalization evidence. The complete fixed pixel-buffer caller is now
connected below; its C output processor remains separate.

External input conversion adds 13,627 differential requests: 1,303 row/layout
cases and 12,324 plane cases. Both endian orders cover every binary16 pattern
and every uint16 sample in row fixtures. Additional cases retain signaling-NaN
payloads in direct binary32 copies, signed zero, integer scaling with exceptional
scale bits, zero stride, unsupported formats, callback counts, and output tails.
Plane fixtures compare every physical row sample after conversion, including
untouched padding. They cover interleaved channel offsets, all three named
endian values and the unnamed source enum value 3, arbitrary row alignment,
minimum/full last-row sizes, size=0, invalid bit depths, dimension/stride errors,
and checked arithmetic overflow. Native regressions retain original half-float
bit patterns, RGB16 channel selection with padded rows, last-row padding limits,
and unclamped custom integer-depth scaling. These functions provide external
planes. The buffer-backed fixed image caller is connected below; the callback
pixel adapter and general image entry point remain unported.

Frame buffer integration adds 11,762 differential requests: 4,032 binding/helper
cases, 384 rebinding/copy sequences with twelve operations each, and 7,346
fixed-profile CopyColorChannels cases. The original buffer adapter and its
private channel state are observed after explicit initialization. Comparisons
retain old-copy precedence, null/empty ownership, immediate extra-channel
copies, color-only CopyBuffers, format/stride mutations before failed binds,
unsigned overflow checks, and unmodified row-offset outputs for unavailable
views. Color-copy cases cover all sample types, endian values, aligned strides,
cropped/out-of-bounds rectangles, grayscale replication, interleaved-alpha
flags, and errors after earlier planes were written. Every physical output-row
sample is compared, including seeded padding and rows retained after shrinking.
Native regressions cover wrapped stride assignment, stale copies, gray-copy
partial failure, and composition from a zero-gray pixel buffer through padding,
modular coding, and the original 13 finalized frame bytes. That composition
does not independently establish enclosing initialization or image decoding.
The fixed encoder now connects its complete shared-state constructor and raw
codestream output below.

Shared scalar/field integration adds 2,559 differential requests: 392 Quantizer
sequences, 1,458 ColorCorrelation setter/ratio/decode cases, 148 map cases,
221 strategy-storage cases, and 340 pixel/frame cases. Quantizer observations
retain all defined multiplier lanes, exact double/float conversion points,
zero/infinite/NaN arithmetic, and getters after the original matrix owner
changes but before recomputation. Correlation tests include zero factors, full
signed DC values, raw float bits, half-float range failures, defaults retaining
prior state, unaligned/truncated input, consumed bits, and reader close status.
Map cases distinguish uncomputed default DC factors from Create's recomputed
factors, retain XYB=false behavior, round tile dimensions, and compare zeroed
logical samples. Strategy cases cover native moves, the cached row owner,
fills, validity, and unnamed raw types with the original byte/int promotions.

The frame fixture calls the complete original ComputeEncodingData and, in a
separate run, EncodeFrameOneShot with the original output processor. The native
fixture calls the translated fixed-profile ComputeEncodingData and independently
EncodeFrameOneShot, including the complete shared-state constructor, reference
binding, buffer input, padding, modular preparation, shared tokens, groups,
section finalization, and byte-owner transfer. All 336 regular
frame cases compare complete frame bytes and auxiliary statistics: 8/16-bit
gray/RGB constants, ramps, and deterministic noise across seven dimensions and
borrowed/copied input. Four null-buffer cases retain the original generic error
and allocated field state. Uninitialized source plane contents and the fourth
uncleared quantizer lanes are excluded from observations. This verifies the
fixed native one-shot frame against the actual original caller. Complete
Those frame-output requests are separate from the complete-image evidence below.

A release regression verifies that default Quantizer construction quiets a
signaling NaN through the original multiplier arithmetic while retaining the
unmodified signaling bits in its matrix owner. Both C++ reference variants pin
the quantizer.cc recomputation boundary; Rust uses the corresponding no-inline
method. The earlier DC matrix-copy regression and all exact float-bit checks
remain intact.


Complete fixed-profile encoding connects the reached encode.cc global metadata
and padding block with the original one-shot frame path. Input is an integer
8-/16-bit native-endian grayscale/RGB byte buffer; output transfers a PaddedBytes
owner containing the complete raw codestream. Metadata and compression settings
remain private. C output processors, container wrapping, callbacks, custom
allocators, and general encoder settings remain unported.

The new image fixture compares 192 successful complete codestream encodes over
eight dimensions, both sample depths/layouts, constants, ramps, deterministic
noise, and optional statistics. The public C++ decoder reads both the original
and supplied native output and must recover exact dimensions, channels, bit
depth, and integer samples. Another 32 short-buffer cases retain the generic
error before metadata output or accounting changes. This is encoder-to-C++
decode evidence; it does not claim a Rust image decoder or complete milestone.

Owner fixtures compare 181 initialized-state observations for ImageBundle,
PassesEncoderState/PassesSharedState, all four reference bundles, BlockCtxMap,
patch binding, spline construction/repeated clearing, native moves, and noise
threshold/exceptional-value behavior. Source-indeterminate fields are omitted.
Another 48 ACImage cases cover both admitted integer types, allocation failures,
empty owners, virtual mutable/const row offsets, stride, moves, whole/plane
zero-fill, and unchanged physical row padding. Native regressions verify
same-owner bindings after moves, byte transfer without copying, repeated image
encoding, and pre-output short-buffer rejection.

Noise HasAny observations retain the original early-return loop with the
reference compile setting documented in ADR 0002. The ordinary AArch64 O3
unrolled FMAXNM reduction changed its signaling-NaN result; no case or exact
comparison was removed.

Modular frame global decoding adds 4,248 differential requests. The original
Thunder encoder supplies real first-group payloads for gray/RGB, both integer
depths, constants, ramps, noise, and seven dimensions. The fixture begins after
the caller's default DC-matrix bit; it compares global decoding with shared
trees, truncation permission, damaged data, repeated calls, and optional plane
release. Separate original generic-encoder streams exercise local trees,
YCbCr shifts, XYB metadata, extra-channel upsampling, VarDCT's no-color branch,
and the source bit-depth rejection checkpoints.

Observations include exact status, consumed bits/close, frame dimensions,
logical/physical channel metadata, initialized global pixels, deferred
transforms, tree/context/code state, and group-header mutations. Failed global
decoding retains the previous full-image owner until the original assignment;
nonfatal truncation follows the original transfer behavior. Constructor-undefined
source flags/code scalars receive explicit fixture seeds before observation.
Unread image channels and source-indeterminate transform members are omitted.
MaybeDropFullImage preserves channel metadata while releasing plane owners.
These global component requests alone do not satisfy the first-image gate.
Group/render and complete-image integration evidence is recorded below.

The original simple render pipeline adds 724 differential requests across four
test groups. They exercise original pipeline/base operations and both original
float image write stages, including gray/RGB profiles and extra planes. The
reference includes the original upstream X/Y interpolation test stages without
changes; native fixture translations exercise their row offsets and mirrored
borders on odd dimensions and edge groups. Comparisons retain inferred channel
shifts, cumulative padding, physical strides, initialized interior float bits,
group completion counts, repeated passes, reverse group order, input-owner
moves, and the original uint8 pass-counter wrap. Row padding and incomplete
initial group data remain unobserved.

Fixture callbacks exercise initialization/preparation/size/row failures, image
dimension switching, signed foreground clipping, and background-row writes.
They also mutate border_y during SetInputSizes: the current pass must read it
after that mutable callback, matching the source checkpoint. ImageBundle setter
cases distinguish failed guards before ownership transfer from VerifySizes
errors after the color/profile assignments. The native input binding becomes
None only at the original move; no additional pixel allocation/copy is used.
The complete first-image path explicitly selects this original simple
implementation. These independent pipeline requests remain separate from the
whole-frame/file evidence below.

Modular DecodeGroup, ModularImageToDecodedRect, and FinalizeDecoding add 5,888
differential requests. The original encoder supplies complete section vectors
for eight dimensions, gray/RGB, both integer depths, constants, ramps, and
noise. Comparisons retain full-image versus dropped-plane behavior, deferred
global RCT, clone versus move finalization, repeated finalization, empty-group
readiness, nullable state/output pointers, zero-fill without a reader, wrong
stream-kind rejection, and individual truncated sections. Observations include
output float bits, pipeline counters/layout, remaining decoder metadata,
status, and each section reader's consumed bits and close status.

Separate initialized local streams exercise XYB factors/channel ordering,
gray replication, accurate high-depth conversion, custom-float input, VarDCT's
no-color branch, extra-channel integer/float depths, subsampling, cropping,
remaining transforms, absent/empty channels, and partial output before errors.
Pipeline interiors are seeded before these error comparisons; undefined C++
padding and unread pixels are not observed. Native section integration checks
expected sample normalization for gray/RGB, 8/16 bits, single/multiple groups,
and retained bit-depth metadata after in-place finalization. It still supplies
metadata directly and does not parse complete JPEG XL codestreams.

The translated callees borrow real shared-state and pipeline fields separately;
they do not substitute a partial PassesDecoderState constructor. Pipeline
preparation, outer frame parsing/scheduling, and final external pixel-buffer
conversion are connected by the complete-image entry point recorded below.

The original scalar UpsamplingStage adds 1,360 differential requests. The
actual original factory/stage runs inside the original simple pipeline, with
all three shifts and each RGB channel, mirrored odd/tiny dimensions, edge
groups, reversed group order, default/custom/zero/NaN weights, signed zeros,
infinities, subnormals, and signaling/quiet NaN inputs. Wide rows cross the
original 1,024-input-pixel chunk boundary, including 8x output beyond 8,192
pixels. Factory rejection, zero/repeated workspace preparation, zero-width row
processing, and nonzero-extra guards retain exact statuses and untouched output.
All observed source pixels are initialized, including direct error fixtures.

The scalar kernel uses separate multiply/add rounding, matching the actual
original object; a fused replacement changed a mirrored 3x2 result by one ULP.
The native regression retains independently captured output words for that
case. Highway scalar min/max ignore one NaN and choose the second equal operand;
those rules are distinct from the port's C++ std::min/max helpers. Workspace
owners retain the original allocation sizes/order and transfer float words
through existing aligned u32 views, adding no unsafe casts. This stage is a
decoder-initialization dependency, not a complete frame decoder.

OpsinParams/OutputEncodingInfo and inverse-matrix lane expansion add 359
differential requests. Observations retain all three color bundles, ICC bytes,
inverse matrices, bias/cube-root/quantization words, intensity targets, gamma,
luminances, original/default predicates, and statuses. The corpus covers RGB,
gray, XYB, unknown color spaces, supported transfer functions, custom primaries/
white points, singular/invalid coordinates, raw ICC fields, original-profile
fallback, desired-profile rejection, custom matrices, exceptional floats, and
intensity-threshold neighbors. Source-indeterminate aggregate scalars are
explicitly value-initialized in these fixtures before observation.

Repeated metadata sequences alternate XYB and non-XYB, preserving prior matrix
and all_default_opsin state when the original non-XYB branch does not assign
them, including after partial errors. A native regression checks that retention
while updating to a gray lossless profile. The private SetColorEncoding copy is
resolved at each caller's original assignment checkpoint; the remaining body
borrows the stored copy, avoiding an extra clone for a same-owner argument.
These are decoder-state dependencies; complete frame/image decoding remains
unported.

Shared/decoder-state initialization adds 460 differential requests. They call
the original InitializePassesSharedState, PassesDecoderState constructor/Init,
InitForAC, and PixelCallback constructor/IsPresent/Init. Shared-state sequences
cover missing metadata, all DC-frame levels and availability masks, encoder
versus decoder allocation, modular versus VarDCT coefficient buffers, partial
errors, and retained prefixes through repeated initialization. Non-XYB metadata
still uses ColorCorrelationMap::Create's original xyb=true default. Missing DC
frames leave the cleared storage and newly selected empty frame visible; level
four fails before that transfer.

Decoder observations retain own/external shared selection, sigma geometry,
unconditional original 8x stage construction, stale output format/depth/stride,
callback/buffer resets, unchanged frame counters/entropy owners/reference
bundle, coefficient type, and original output-encoding storage. AC tests cover
each strategy bucket/offset, mask combinations, retained coefficient-vector
prefixes, zero passes, and defined size_t multiplication wrapping. Exact vector
observations use lossless run-length notation, without omitting entries.
Callback fixtures exercise all release-state descriptor combinations, null
opaque data, and complete forwarding of pointer-width argument values.

Native regressions verify a DC binding after failure and owner moves, and that
Init sizes sigma from the selected shared state while InitForAC mutates the
owning shared_storage fields. Uninitialized source pixels and source-indeterminate
members are omitted or explicitly seeded before observation. The connected
pipeline and outer frame/image entry points are recorded below.

Callback-backed ICC assignment, ColorSpaceTransform, and CmsStage add 2,962
differential requests. The actual original methods run with a caller CMS that
records profiles, retained ICC spans, thread/pixel arguments, buffer aliases,
statuses, and destruction. Its bit-copy operation is caller behavior, not a
replacement color-conversion algorithm. Cases include raw/empty ICC profiles,
CMYK sources/destination rejection, partial external-field errors, source gamma
retention during invalidation, input allocation transfer only on success,
profile/linear selection, size guards, zero pixels/threads, initialization/run
failure, repeated preparation, and all defined source/destination layouts.
Foreign buffers include signed zero, infinities, subnormals, and NaN payloads.

The stage uses stack row descriptors and actual callback buffers, with no pixel
clone. Input/output CMS buffers can be identical for channel reduction, without
forming overlapping Rust slice references. Run failure leaves pipeline pixels
untouched. The original repeated transform Init overwrites its current context
without destroying a previous one; exact callback traces retain that behavior.
The caller probe owns its registry until fixture teardown and reports which
contexts received destroy, so it does not hide a missing callback.

Native regressions verify that ICC caches survive owner moves and source-profile
destruction, remain live during the single destroy callback, and that the fixed
lossless configuration needs no CMS callbacks. Native precondition checks cover
initialized size/context and successful CMS thread/pixel bounds; undefined
source call order is not treated as a valid decoding result. Default CMS backend,
C ABI reentrancy, custom runners, and general frame/image decoding remain
unported.

The one-lane tone-mapping stage and numeric dependencies add 17,136 differential
requests: 5,148 FastPow2f/FastPowf/TF_PQ cases, 10,260 derived Rec. 2408/HLG/gamut
cases, and 1,728 factory/direct-row/simple-pipeline cases. They retain exact
float words through rational approximation thresholds, signed zeros, subnormals,
infinities, signaling/quiet NaNs, intensity changes, gamut policies, no-op
selection, early guards, PQ scaling, and reversed multi-group scheduling.
The oracle calls the pristine specialized functions and concrete stage, and
the original simple pipeline performs all scheduling and output writes.

Assembly inspection confirms separate multiply/add instructions in the scalar
stage and gamut kernel. The gamut blend's original-value-first add preserves
the original quiet-NaN payload; a captured native regression retains that case.
Another native test establishes the original no-tone-mapping selection for all
four fixed lossless sample/channel configurations. These component results do
not by themselves complete the frame/image decoder.

The fixed raw-frame decoder adds 815 differential requests: 80 independent
source/native encoder-byte comparisons, 160 cross-decoding cases, and 575
truncation/corruption/cache-initialization cases. The oracle calls complete
original DecodeFrame with its real simple-pipeline option, decoder state,
modular decoder, and output bundle. Cases cover 8/16-bit gray/RGB, constants,
ramps, restricted palettes, deterministic patterns, edge groups, multiple DC
groups, every prefix of small complete frames, and payload bit flips.

Observations include exact status, the original output-header copy checkpoint,
dimensions, profile, duration/origin, decoded-byte accounting, and every defined
successful output float word. Error paths retain defined partial state while
omitting source pixels that remain uninitialized. The unchanged frame/write/
modular components perform all decoding and output work; no test adapter
supplies a replacement algorithm. Native integration also checks recovered
samples independently, and a profile-admission regression prevents entering
unported noise parsing. This admission result is outside the general decoder
contract and is not compared as a C++ codec status.

The stage and decoder borrow one stack RefCell output owner for individual
sequential calls. This preserves the source origin assignment after pipeline
preparation without overlapping Rust mutable references or adding an output
clone. This component entry point still accepts private configuration metadata;
the complete whole-codestream entry point below obtains it from the file.

Integer buffer export adds 11,168 differential requests: 160 scalar rounding
corpora, 5,248 channel/orientation/endian/span cases, and 5,760 image-bundle/
alpha/layout cases. Comparisons preserve halfway neighbors across 1..16 bits,
NaN/infinity clamps, untouched row and allocation suffixes, all seven actual
orientation transforms, nullable channels, source guard order, alpha
unpremultiplication, metadata verification, and both byte orders. The C++
adapter includes pristine dec_external_image.cc; no integer algorithm is
provided by the fixture. An early fixture value 9 exposed an undefined
JxlEndianness enum load. Its unknown-value case now uses representable value 3,
which retains the same source fall-through/output while preserving the case
count; undefined value 9 receives no invented C++ codec meaning.

Floating buffer export adds 27,587 differential requests in five groups and
three ordinary native tests. The half-kernel groups compare 2,641 rows covering
all 65,536 half words and both signs at every finite half midpoint's immediate
binary32 neighbors, plus 1,434 rows of NaN payloads, exponent boundaries, and
generated binary32 words. Another 2,232 raw-bit buffer cases compare native
half casts and binary32 copies through both byte orders and 1..4 channels.
The 2,848 channel cases retain orientation, missing channels, span/format guards,
zero-row lifecycle, and unchanged padding/suffixes; 18,432 bundle cases preserve
gray/RGB layouts, alpha presence/association/unpremultiplication, metadata state,
and guard order. Native coverage independently checks every half word with
native-cast NaN quieting, exact signaling-NaN Float32 copies, nullable-channel
ones, endian/orientation stores, and error-time untouched buffers.
The oracle enforces Highway's selected native-half-operator compile branch;
alternate compiler/floating-environment behavior remains outside this slice.
Floating file-level decoding is not inferred from this output component.

Complete first-image integration adds 1,081 differential requests. The C++
oracle executes the exact original DecodeFile and ReadICC helper bodies,
extracted without rewriting and checked against pinned SHA-256 hashes.
Original source line directives and copyright notices remain in generated
fragments; the real original test allocator, metadata checks, decoder state,
frame decoder, and integer exporter execute normally. The public C++ decoder
independently reads both encoder outputs as well.

The corpus has 140 source/native complete-file byte comparisons and paired
public C++ decodes, 280 original/Rust complete-file decode comparisons, and 661
malformed/truncated whole-file cases. Seven dimensions include tiny images,
odd sizes, edge groups, and multiple DC groups; gray/RGB and 8/16-bit samples
include constants, ramps, limited palettes, and deterministic noise. Observations
retain exact parsed metadata, source status, defined partial frame dimensions/
decoded-byte accounting, and integer bytes. Prefixes, damaged signatures,
trailing bytes, and payload corruption exercise integrated cleanup and failure
checkpoints. Profile admission is explicitly separate from the general decoder
contract and never supplies an unported codec implementation.

Ordinary Rust tests execute sixteen complete pixel/file/pixel round trips and
one 256x256 image containing all 65,536 unsigned 16-bit samples. No C++ feature,
runtime link, external crate, manually assembled codestream, or image-specific
expected output is used by that native path. The [first-image acceptance
boundary](FIRST_IMAGE.md) is satisfied for the fixed profile; this result does
not establish the full library port or any unclaimed platform/release gate.


The original scalar Gaborish stage adds 1,985 differential requests: ten
factory/settings/channel-mode/lifecycle observations, 1,734 direct padded-row
observations, and 241 original simple-pipeline runs. Rows cover zero widths,
left/right extra spans, nonzero descriptor offsets, untouched output sentinels,
ignored extra channels, default/distinct/zero/canceling/random weights, finite
extremes, subnormals, infinities, and signed signaling/quiet NaN payloads.
Simple-pipeline runs cover tiny/odd dimensions, both sides of group boundaries,
reversed group arrival, repeated passes, and the original mirrored borders.
The factory retains its gab guard; this component does not admit a general
lossy frame path.

Hosted x86 exceptional-value observations exposed different NaN operand
schedules between the original ordinary and sanitized Clang 21 `-O3` objects.
The dedicated pristine Gaborish observation TU now uses scoped `-O0`, retaining
all sanitizer flags and the literal Highway Add/Mul/MulAdd boundaries. The
original library and existing fixture TU options are unchanged. Link maps bind
the actual original stage and factory to that dedicated object. Rust retains
literal product-first nested additions and operator*'s explicit promotions,
double product, and binary32 narrowing through local non-inlined boundaries.
The implementation does not rewrite NaN words or change kernel grouping.

All 1,985 original differential cases remain; another 128 requests exercise
competing signed signaling/quiet weight and pixel payloads during normalization
and filtering. The original finite 3x2 native regression and its captured words
are unchanged. A second native regression uses independently captured competing
payload words and verifies untouched rows, extra spans, and ignored channels.
Its separate ARM/x86 NaN tables were independently captured from each target's
original stage; the input stencil is identical despite different row padding.
Both run without the C++ feature. A temporary fused-pixel replacement failed the
original finite regression, proving that it detects separate-rounding changes.
This numeric configuration establishes no SIMD or performance parity.

Focused x86_64 Linux runs under QEMU use Clang 21.1.8/libc++ 21 and Rust 1.99.0.
Debug and release each compare all 2,113 Gaborish requests in four groups
against both an ordinary and a fail-fast UBSan observation executable; both
native Gaborish regressions pass in both profiles. The temporary focused
executables retain the actual original fixture observation bodies and dedicated
stage, linked against the complete original ordinary static archives. UBSan
instruments that focused fixture/stage, not those ordinary archive members.
Link maps bind the original factory and ProcessRow to the dedicated stage
object. Emulated ASan is unavailable; it is not counted as a passing gate.
Native hosted CI must validate the complete Linux ASan/UBSan graph and MSRV
before merge. The complete ordinary/sanitized macOS runs in the evidence table
remain separate, fully instrumented reference gates.

### Scalar XYB render boundary follow-up

The normal scalar XYB stage adds 11,004 exact differential requests in three
groups: 6,540 helper/default/custom/exceptional-value cases, 4,320 factory,
channel-mode, row-offset, guard, and retained-storage cases, and 144 simple
pipeline cases including reverse group submission across group edges. The
raw-parameter corpus independently varies all 48 stored words and combines
coefficient/bias and input NaN payloads; unused broadcast/fourth/quant lanes
remain ignored. Two ordinary Rust tests use independently captured original
words and exercise both the actual helper and scaled stage, guards before row
access, copied output selection, and untouched extra channels/storage.

The reference uses the actual original helper/stage from a dedicated fixture
unit at -O0, with every ASan/UBSan check active. ADRs 0002 and 0003 document
why competing-NaN operand order requires that isolated numeric configuration
and native local primitive call boundaries. The original codec build, other
fixtures, old tests, and complete first-image corpus retain their existing
configuration and cases. This does not translate FastXYB/sRGB8, SIMD targets,
general image conversion, a VarDCT decoder, or any new admitted image profile.

Validated on 2026-11-01 on macOS arm64 with
`python3 tools/check.py --msrv` and
`python3 tools/check.py --sanitizers --msrv`: each configuration passes 328
tests, including 186 differential test groups and 714,423 requests, with no
failed or ignored cases. Both references pass debug, release, and Rust 1.85.1,
including the existing complete first-image corpus. The Linux x86_64 all-target,
all-feature offline cargo check also passes; Linux runtime evidence remains a
separate hosted gate. The ordinary native XYB test executable links only
libSystem and introduces no C++ runtime dependency.

A structural precision review additionally preserves Highway scalar operator*'s
literal float-to-double promotions, binary64 product, and narrowing to the
float lane in both XybToRgb and scaled XYB output. Local out-of-line widening
and product calls retain those checkpoints in native release assembly. The
existing finite/NaN corpora and captured regression words stay unchanged; no
extra request, expected-word replacement, or global helper change is needed.
Renewed ordinary and sanitized full checks retain the same test/group/request
counts in debug, release, and Rust 1.85.1 after this precision mapping repair.

## Linux reference configuration and scalar operand order

The first hosted Linux run identified four reference-platform gaps. The Linux
oracle now explicitly selects libc++ for the recorded palette-sort algorithm;
libstdc++ uses a different equivalent-element order. The frame-finalization
adapter gives empty groups real backing storage without adding bits, retaining
all empty-group cases and the original writer under Linux's nonnull memcpy
checks. Local out-of-line x86 scalar additions/multiplications preserve the
original downsampling/DC-scaling instruction operand order and NaN payloads.
Existing AArch64 expressions and all prior differential cases are unchanged.
Two ordinary Rust regressions retain independently observed source words and
failed-roundtrip partial state. Compiler and standard-library changes require
renewed evidence; these fixes do not establish any new codec configuration.

The ordinary macOS check with `--msrv` passes all 325 tests and 703,420
differential requests in each Debug, Release, and Rust 1.85 configuration.
The two captured regressions also pass in the isolated x86_64 Linux runtime;
full Linux and sanitized checks remain required before integration. CI runs
the unchanged full Linux/macOS matrix on each PR update and on pushes to main,
avoiding duplicate full runs for a feature-branch push followed by its PR.

After those four fixes, native Linux CI passed every existing differential
group and exposed an AArch64-only captured GamutMap word in an ordinary test.
The test now retains independently captured exact words for both reference
targets, with its input also added to the differential corpus. The existing
AArch64 expectation and production gamut code remain unchanged.
