# Port status

This repository contains codestream/frame-header, metadata, geometry, entropy,
and color-profile dependencies.
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
| `../include/jxl/types.h` | `types.rs` | Endianness only; other public API types unported |
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
| `enc_ans_params.h` | `enc_ans_params.rs` | Histogram storage, cost, parameter fields/defaults, speed-tier initialization, and uint config; ForModular unported |
| `enc_cluster.h`, `.cc` | `enc_cluster.rs` | Scalar Condition/entropy/divergence, cluster selection, pair refinement, and canonical reindexing |
| `base/fast_math-inl.h`, `base/rational_polynomial-inl.h` | `base/fast_math.rs` | Existing scalar FastLog2f and its degree-two polynomial; other functions/targets unported |
| `common.h` | `common.rs` | SpeedTier and maximum pass count; other constants unported |
| `../include/jxl/color_encoding.h`, enum masks in `color_encoding_internal.h` | `color_encoding.rs` | Raw color values and external field storage; public C ABI and CMS callbacks unported |
| `cms/color_encoding_cms.h` | `cms/color_encoding_cms.rs` | Coordinates, gamma, field storage/conversions, comparisons, and partial setter state; callback-driven ICC parsing unported |
| `color_encoding_internal.h`, `.cc` | `color_encoding_internal.rs` | Field bundles, profile generation, raw ICC storage, static encodings, and non-callback field APIs; CMS callbacks/transforms unported |
| `image_metadata.h`, `.cc`, quantizer bias constants | `image_metadata.rs`, `image_metadata_weights.rs` | Bit depth, extra channels, opsin/custom weights, tone mapping, image/codec metadata, sample/alpha/orientation helpers, and metadata reading; diagnostic output unported |
| `frame_header.h`, `.cc` | `frame_header.rs` | Frame fields, chroma subsampling, blending, animation, progressive passes, names, dimension/reference helpers, order field encoding, and parsing; diagnostic strings unported |
| `frame_dimensions.h` | `frame_dimensions.rs` | Pixel/block/group dimensions, original rounding/padding, and group rectangles |
| `base/rect.h` | `base/rect.rs` | Unsigned Rect coordinates, crop/intersection, translation, shifts, and extension; signed instantiations and image/row accessors unported |
| `pack_signed.h` | `pack_signed.rs` | Signed packing and pointer-width unpacking |
| `loop_filter.h`, `.cc` | `loop_filter.rs` | Gaborish/EPF parameter fields, defaults, validation, and padding; pixel filter kernels unported |
| `base/matrix_ops.h` | `base/matrix_ops.rs` | Float/double matrix products, vector products, and in-place inverse for disjoint input/output callers |
| `cms/opsin_params.h` | `cms/opsin_params.rs` | Original float constants, constexpr scaling, and A2B cube |
| `cms/transfer_functions.h`, `cms/tone_mapping.h` | `cms/transfer_functions.rs`, `cms/tone_mapping.rs` | Scalar HLG/PQ, Rec. 2408, HLG OOTF, and gamut mapping; SIMD variants unported |
| `cms/jxl_cms_internal.h`, used `base/common.h` helpers | `cms/icc*.rs`, `base/float.rs` | Complete profile generation with default 3D tone mapping, matrices, curves, tags, descriptions, and source MD5; ICC parsing and color transforms unported |
| Highway `hwy/cache_control.h` | `base/cache_control.rs` | Clang reference enablement: T0 on x86_64 with SSE2; source-disabled hint on other targets |
| `enc_fields.cc` | `enc_fields.rs` | Primitive/bundle writes, size/image metadata wrappers, complete codestream-header output, and frame header writing; quantizer writes unported |
| `headers.h`, `.cc` | `headers.rs` | Size, preview, animation, and size-header read wrapper |
| `base/status.h` | `base/status.rs` | Status-code/Result mapping; diagnostics and crash instrumentation unported |
| `padded_bytes.h` | `padded_bytes.rs` | Writer storage subset; other container operations unported |
| `memory_manager_internal.h`, `.cc` | `memory_manager_internal.rs` | Aligned allocation subset using Rust allocator; custom callbacks and image-row sizing unported |
| `enc_aux_out.h`, `.cc` | `enc_aux_out.rs` | Layer totals and writer charging; image statistics and diagnostic output unported |

Internal storage mappings are specified in the
[representation ADR](adr/0003-rust-representation.md). This table counts
translated boundaries, not whole-file or whole-library completion.

## Evidence

Validated on 2026-11-01 on macOS arm64 with Rust 1.99.0 (stable), Rust 1.85.1
(minimum-version series), and Apple Clang 21.0.0. The C++ oracle is built from
the unmodified pinned source with scalar Highway and release status behavior.

Command: `python3 tools/check.py --sanitizers --msrv`.

| Gate | Result | Evidence |
| --- | --- | --- |
| Source revision, Highway pin, notices, local doc links | PASSED | Checked against `upstream.json` and the Git submodule entries |
| Rust formatting and Clippy | PASSED | All targets and features, warnings treated as errors |
| Rust debug | PASSED | 100 tests, including 52 differential groups; zero ignored cases |
| Rust release | PASSED | Same 100 tests and corpus |
| Rust 1.85.1 | PASSED | Same 100 tests and corpus |
| C++ ASan and UBSan | PASSED | Fail-fast instrumentation during all three Rust test configurations |
| Rust x86_64 Linux compile | PASSED | `cargo check --locked --offline --all-targets --all-features --target x86_64-unknown-linux-gnu`; no Linux runtime tests claimed |

Each differential run compares **510,074 requests** against C++. These include
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

1. Port section tables and backing image/channel storage, then
   the first actual frame path.
2. Connect modular prediction/residual coding and the transforms reached by the
   first-image configuration.
3. Expand through remaining modular and VarDCT decoding and encoding before
   introducing the C ABI and optimized target-specific kernels.

Preserve the upstream test inventory as that scope grows. Adapted tests in this
first slice do not replace the unported image, threading, allocation-failure,
platform, or codec test suites.
