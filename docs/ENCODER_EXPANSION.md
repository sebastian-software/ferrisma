# Encoder expansion

This plan orders the remaining encoder work into gated delivery steps. Its
scope is the [README checklist](../README.md#remaining-encoder-work) section
"Remaining encoder work" plus the encoder items under "Shared runtime and
full-port acceptance": container and box writing, encoder allocator
propagation, `JxlEncoderSetParallelRunner`, and the encoder C ABI. The
[porting contract](PORTING.md), the accepted [ADRs](adr/README.md) and the
delivery rules of the [decoder expansion](DECODER_EXPANSION.md) continue to
apply: translate the pinned source mechanically, keep C++ in the test oracle,
and compare with the original rather than with a Rust round trip.

The main goal is a usable encoder behind the public API whose output is
byte-identical to pinned libjxl. A pipeline engine such as ferrixel needs that
API for JPEG XL output, beginning with lossy SDR photographs. The sequence
therefore reaches a public lossy VarDCT encode at default settings first and
broadens from there.

## Reference configuration

"Byte-exact" in this plan means byte-identical to the pinned original built as
the [differential oracle](adr/0002-differential-oracle.md) builds it: scalar
Highway (`HWY_COMPILE_ONLY_SCALAR`), upstream's `-fno-slp-vectorize
-fno-vectorize`, Clang with libc++, the ordinary and ASan/UBSan graphs, on
native Linux x86-64 and macOS AArch64. A libjxl build with SIMD targets, such as
a distribution package, can choose differently in the float-heavy encoder
heuristics and produce other bytes. This plan does not claim parity with such
builds, nor bit-exact output across CPU targets (see [PORTING.md](PORTING.md)).
Expected bytes always come from the oracle on the same host. Stored fixtures in
ordinary Rust tests record their originating target.

## Starting point

These encoder-side units are already translated and mapped in
[PORT_STATUS.md](PORT_STATUS.md#source-mapping):

- Entropy coding: `enc_ans`, `enc_ans_params`, `enc_cluster`,
  `enc_context_map` (including the VarDCT BlockCtxMap output), `enc_huffman`,
  `enc_huffman_tree`, `enc_lz77`, and the scalar token cost from `enc_ans_simd`.
- Bitstream output: `enc_bit_writer`, `enc_fields` (codestream headers, frame
  headers, quantizer parameters), `enc_toc`, `enc_aux_out`.
- Frame scaffolding: all `enc_params.h` storage, `ParamsPostInit`,
  `MakeFrameHeader`, group-size and progressive-mode helpers, and
  `InitPasses` in `enc_progressive_split`. The fixed lossless Thunder path
  runs through `ComputeEncodingData`/`EncodeFrameOneShot` and the reached
  metadata block of `encode.cc` ([first image](FIRST_IMAGE.md)).
- State owners: `passes_state`/`enc_cache.h` storage and constructors, the
  buffer-backed `JxlEncoderChunkedFrameAdapter` in `encode_internal.h`, and
  `ConvertFromExternal` for planes.
- Modular: the standalone `enc_encoding`, `enc_ma` tree learning, the
  `enc_palette`, `enc_rct`, `enc_squeeze` and `enc_transform` transforms,
  `EstimateCost`, and the `ModularFrameEncoder` parts reached by Thunder.
- Partial VarDCT encoder units: coefficient-order permutation output in
  `enc_coeff_order`, DC serialization in `enc_quant_weights`.

The encoder reuses many decoder units that are complete: the scalar forward
and inverse DCT for sizes 1 to 256, `AcStrategy`/`AcStrategyImage`,
`Quantizer` scaling, `DequantMatrices`, `ColorCorrelationMap`, `DequantDC`
and `AdaptiveDCSmoothing`, EPF sigma, the render pipelines and
`PreparePipeline`, `DecodeFrame`, the default CMS, the thread runners, box
parsing, JPEG data structures, the JPEG writer and the Brotli decoder.

## Upstream inventory

The remaining encoder source, with line counts at the pinned revision:

| Source (`lib/jxl/`) | Lines | Step |
| --- | ---: | --- |
| `encode.cc`, `encode_internal.h`, `include/jxl/encode.h` | 3,108 + 719 + 1,650 | 1, 2, 3, 13, 14 |
| `enc_frame.cc` (general paths) | 2,779 | 1, 2, 5, 9, 10, 12 |
| `enc_heuristics.cc` | 1,227 | 1, 7 |
| `enc_adaptive_quantization.cc` | 1,292 | 1, 7 |
| `enc_ac_strategy.cc` | 1,200 | 1, 7 |
| `enc_modular.cc` (VarDCT DC/AC metadata, general images) | 1,864 | 1, 4, 5 |
| `enc_chroma_from_luma.cc`, `enc_group.cc`, `enc_entropy_coder.cc` | 420 + 571 + 270 | 1 |
| `enc_coeff_order.cc`, `enc_quant_weights.cc` (AC), `enc_progressive_split.cc` | 334 + 226 + 83 | 1, 9 |
| `enc_xyb.cc`, `enc_gaborish.cc`, `enc_transforms*.{h,cc}`, `convolve_symmetric5.cc` | 411 + 74 + 852 + 196 | 1 |
| `enc_cache.cc` | 251 | 1, 9 |
| `enc_patch_dictionary.cc` | 896 | 1 (detection), 7 |
| `enc_image_bundle.cc`, `enc_icc_codec.cc` | 168 + 484 | 6 |
| `enc_fast_lossless.cc` | 4,451 | 8 |
| `butteraugli/butteraugli.cc`, `enc_butteraugli_comparator.cc`, `enc_comparator.cc`, `enc_convolve_separable5.cc` | 2,189 + 117 + 152 + 413 | 7 |
| `enc_detect_dots.cc`, `enc_dot_dictionary.cc`, `enc_linalg.cc`, `enc_optimize.h` | 621 + 76 + 49 + 193 | 7 |
| `enc_noise.cc`, `enc_photon_noise.cc`, `enc_splines.cc` | 382 + 100 + 108 | 7 |
| `jpeg/enc_jpeg_data_reader.cc`, `jpeg/enc_jpeg_huffman_decode.cc`, `jpeg/enc_jpeg_data.cc` | 1,056 + 105 + 413 | 12 |
| `enc_debug_image.cc`, `modular/encoding/enc_debug_tree.cc` | 137 + 121 | 13 |
| Brotli `c/enc` at the pinned `third_party/brotli` | 9,967 + 15,186 headers | 11 |

The encoder tests at the pinned revision are `encode_test.cc` (30 test
definitions, two parameterized suites), `jxl_test.cc` (76), `roundtrip_test.cc`
(11), `passes_test.cc` (11), `modular_test.cc` (21), `speed_tier_test.cc` (one
suite over all speed tiers), `preview_test.cc`, `patch_dictionary_test.cc`, and
the unit tests `enc_gaborish_test.cc`, `enc_linalg_test.cc`,
`enc_optimize_test.cc`, `enc_photon_noise_test.cc`, `enc_bit_writer_test.cc`,
`enc_external_image_test.cc`, `quant_weights_test.cc`, `coeff_order_test.cc`
and `ac_strategy_test.cc`. Many reach the encoder through
`lib/extras/enc/jxl.cc` (`EncodeImageJXL`, the public API) or through
`test::EncodeFile` in `test_utils.cc`, which calls `WriteCodestreamHeaders`,
`EncodePreview` and `EncodeFrame` directly.

## Delivery and acceptance

Deliver each step through isolated worktrees, unsigned Conventional Commits and
pull requests, integrated serially after the local and native gates. A step may
span several PRs; it is complete only when all of its scope and gates are on
`main`. Check a README item only when its full stated scope is delivered.

### Public API admission

The public encoder API (`jxl::jxl::encode`, translating `encode.cc`) arrives in
step 1, before the general frame encoder underneath it is complete. Until step
14, it admits a recorded profile. A request outside the profile returns
`JXL_ENC_ERROR` with `JXL_ENC_ERR_NOT_SUPPORTED`, at the first source
checkpoint where the original would enter an unported branch. Admission
results are never counted as parity. For every admission case in the corpus,
the oracle must show that the original actually reaches the excluded branch.
Each step that widens the profile updates that record.

ADR 0001 currently admits configuration-specific internal entry points and
rules out a general entry point with unsupported branches. Step 1 first amends
ADR 0001 in place to admit a public encoder whose profile is enforced like
this. The C ABI is exported only in step 14, after the last admission is
removed, because a C caller cannot tell an admission result from a libjxl
status.

### Differential gate

Unless a step says otherwise, its gate includes all of these:

1. **Codestream and container bytes.** The complete caller history (setter
   statuses, `JxlEncoderGetError`, every `JxlEncoderProcessOutput` status, and
   each output chunk) is byte-identical to the original public API for the
   same history. This includes small `avail_out` schedules (0, 1, 7 and 64
   bytes, then doubling) to cover `NEED_MORE_OUTPUT` resumption.
2. **Decode-back.** Rust output decoded by the Rust public decoder and original
   output decoded by the original decoder give identical events, metadata and
   pixels. Each output is also decoded by the other decoder. Byte identity
   already implies this, but the check guards the harness and the decoder
   admission of new features.
3. **Rejection.** Invalid options, out-of-range values, call-order errors,
   mismatched buffer sizes and pixel formats, and settings that contradict the
   basic info give the same statuses and errors, with no output.
4. **Checkpoints.** Internal fixtures with `-fno-access-control`, as in the
   existing frame fixtures, compare intermediate state as exact float words or
   integers: XYB planes, quant fields, `AcStrategyImage`, CfL maps, EPF
   sharpness, coefficients, tokens, histograms, and section sizes. One
   flipped heuristic decision changes every later byte, so a whole-file
   mismatch without checkpoints cannot be localized.
5. **Upstream tests.** Each step names the upstream tests it turns into
   differential cases. Their helpers (`GetSomeTestImage`, `TestImage`,
   `test::Roundtrip`, `test::EncodeFile`, `EncodeImageJXL`) are extracted
   verbatim and hash-checked as ADR 0002 describes. Exact bytes replace
   Butteraugli, size and pixel-distance expectations. Tests that read PNG/PPM
   testdata run on raw pixel fixtures: an independent tool decodes them once,
   records provenance and a hash, and both encoders read the same raw buffer.
   The oracle does not build the extras image codecs.
6. **Configurations.** Run the checked profile against the ordinary and
   ASan/UBSan oracles on native Linux and macOS, Debug, Release and MSRV.
   Retain all existing corpora and expected words. Run the deep gate before
   merging any oracle, flag or audit change.

Settings are written as `e` (effort), `d` (distance), `VarDCT`/`Modular`,
progressive options, and threads. "Serial" means no parallel runner. Under
serial execution the original's `RunOnPool` runs tasks in index order on the
calling thread.

## Work sequence

### 1. Public lossy VarDCT encode at default settings (one-shot)

**Scope.** A public API encode with `JxlEncoderFrameSettingsCreate` defaults
(e7, kSquirrel, d1.0) that matches libjxl byte for byte. Admitted profile:
gray or RGB without extra channels; `uint8`, `uint16`, `float16` and `float32`
input at every endianness and alignment; enum sRGB or linear sRGB color with
`uses_original_profile = false` (XYB); a single displayed frame; raw
codestream output in buffer mode; automatic level 5; serial; default memory
manager. Basic-info metadata fields (orientation, intensity target, bit
depth) are admitted. Images must take the one-shot path: `NumGroupsForFrame` of
at most 8 (for example up to 512x1024), or `JXL_ENC_FRAME_SETTING_BUFFERING` 0.
The effort is 7. Distances are admitted below `kMinButteraugliForDots` (3.0),
which keeps dot detection, 2x resampling (d ≥ 10) and the d > 4.5 branch of
the AR heuristics outside the profile.
At e7 the original runs `FindTextLikePatches`. Port the detection; if it
returns patches, the reference-frame encode needs step 5, so that result is an
admission result (the oracle must report the found patches).

**Closes.**
- `encode.cc`: `JxlEncoderCreate/Reset/Destroy`, `JxlEncoderFrameSettingsCreate`,
  the option setters as storage with value validation, `SetFrameDistance`,
  `DistanceFromQuality`, `SetFrameLossless` (lossless is an admission result),
  `InitBasicInfo`,
  `SetBasicInfo`, `SetColorEncoding`, `ColorEncodingSetToSRGB/LinearSRGB`,
  `AddImageFrame(Internal)`, `CloseFrames/CloseInput`, `ProcessOutput`, the
  raw-codestream branch of `ProcessOneEnqueuedInput`, `QueueFrame`,
  `VerifyLevelSettings`, `CheckValidBitdepth`, `VerifyInputBitDepth`,
  `GetRequiredCodestreamLevel`, `GetError`, `JxlEncoderVersion`.
- `encode_internal.h`: `JxlEncoderStruct`, `JxlEncoderFrameSettingsValues`,
  queued input/frame owners, `CopyFromBuffer`, and the buffer mode of
  `JxlEncoderOutputProcessorWrapper`.
- `enc_frame.cc`: both `EncodeFrame` overloads (setting resolution), the
  VarDCT branch of `ComputeEncodingData`, general `CopyColorChannels`,
  `ComputeVarDCTEncodingData`, `ComputeAllCoeffOrders`,
  `TokenizeAllCoefficients`, `EncodeGlobalDCInfo`, `EncodeGlobalACInfo`,
  `EncodeGroups`, `RemoveUnusedHistograms`, `OutputAcGlobal` as reached, the
  reached `ComputeChromacityAdjustments`/`ComputeNoiseParams`/`DownsampleColorChannels`
  no-op branches, and VarDCT `EncodeFrameOneShot`.
- `enc_heuristics.cc`: `LossyFrameHeuristics` at kSquirrel,
  `FindBestDequantMatrices`, `FindBestBlockEntropyModel`, `ComputeARHeuristics`,
  `ReconstructImage`, `ComputeBlockL2Distance`.
- `enc_adaptive_quantization.cc`: `InitialQuantField`, `AdaptiveQuantizationMap`,
  `PerBlockModulations`, `FuzzyErosion`, `Blur1x1Masking`, `AdjustQuantField`,
  and the kSquirrel branch of `FindBestQuantizer`.
- `enc_ac_strategy.cc`: `AcStrategyHeuristics` with `EstimateEntropy`,
  `FindBest8x8Transform`, `TryMergeAcs`, `FindBestFirstLevelDivisionForSquare`,
  `ProcessRectACS`.
- `enc_chroma_from_luma.cc` (`CfLHeuristics`, `ColorCorrelationEncodeDC`),
  `enc_group.cc` (`QuantizeBlockAC`, `AdjustQuantBlockAC`,
  `QuantizeRoundtripYBlockAC`, `ComputeCoefficients`,
  `EncodeGroupTokenizedCoefficients`), `enc_entropy_coder.cc`
  (`TokenizeCoefficients`), `enc_coeff_order.cc` (`ComputeUsedOrders`,
  `ComputeCoeffOrder`, `EncodeCoeffOrders`), `SplitACCoefficients` for one
  pass, AC `DequantMatricesEncode` for default matrices.
- `enc_xyb.cc` (`ToXYB` sRGB and linear-sRGB branches, `ComputePremulAbsorb`),
  `enc_gaborish.cc` (`GaborishInverse`), `convolve_symmetric5.cc`
  (`Symmetric5`), `enc_transforms*.{h,cc}` (`TransformFromPixels`,
  `DCFromLowestFrequencies`, `AFVDCT4x4`).
- `enc_cache.cc`: `InitializePassesEncoder` without progressive DC,
  `ComputeACMetadata`. `enc_modular.cc`: VarDCT `Init`, `AddVarDCTDC`,
  `AddACMetadata`, and the reached tree, token and stream output.
- Quantizer field selection in `quantizer.cc` (`ComputeGlobalScaleAndQuant`,
  `SetQuantField`, `SetQuantFieldRect`). `dec_group.cc`:
  `DecodeGroupForRoundtrip`, the decoder in the loop for AR heuristics.
- `enc_patch_dictionary.cc`: `FindTextLikePatches` and the empty-result return
  of `FindBestPatchDictionary`. `enc_splines.cc`: `FindSplines` (empty at the
  pinned revision).

**Depends on.** Existing entropy, field, TOC, modular-tree, DCT, quantizer,
dequant-matrix, CfL-map, DC-smoothing, EPF-sigma, `PreparePipeline`, render
pipeline and external-image units.

**Gate.** Byte-exact output at e7 d1.0, plus d0.5, d1.5 and d2.9 and the
intensity targets 255 and 100, serial. Sizes: 1x1, 7x7, 8x8, 9x17, 41–48 x 15
(every `xsize mod 8`), 63x129, 256x256, 257x255 and 512x1024 (8 groups). Each
size runs in gray and RGB with every admitted sample type, flat, gradient,
noise and photographic content, and `GetSomeTestImage` content. Checkpoints
cover every listed heuristic output. Rejection covers adding a frame before
basic info or after closing input, wrong buffer sizes, invalid
distances and options, a gray color encoding for an RGB image (as in
`VerifyFrameEncoding`), and lossless requests (an admission result until
step 5). Upstream differential cases:
`jxl_test` RoundtripSinglePixel, RoundtripSmallD1 (both intensity targets),
RoundtripUnalignedD2, RoundtripD2Consistent (serial), the d1.0 case of
RoundtripGrayscale (through the internal `test::EncodeFile` path);
`roundtrip_test` Float/Uint16/Uint8FrameRoundtripTest restricted to lossy,
1 and 3 channels, no extra channels and no container; the kSquirrel instance of
`speed_tier_test`; all of `enc_gaborish_test.cc`; the encode halves of
`coeff_order_test.cc` and `quant_weights_test.cc`.

**Risks.** This is the largest step and the most sensitive to numerics.
`InitialQuantField`, `EstimateEntropy`, CfL and AR heuristics use
`FastLog2f`/`FastPowf`, `std::exp`/`std::pow`/`std::sqrt`, and long `a*b+c`
chains. One rounding difference changes an AC strategy or quant value and every
later byte. Clang contracts these chains on AArch64 and not on x86-64; the
port must reproduce each contraction site, as the alpha and transfer units do
(see [Cross-cutting risks](#cross-cutting-risks)). The decoder in the loop
(`ReconstructImage`) needs the already ported decoder to match the original
in every intermediate float, not only in final pixels. Patch admission happens
after analysis rather than at API entry.

### 2. Streaming VarDCT for large images, chunked input and output processors

**Scope.** Default lossy encoding of images with more than 8 groups, which
the original routes through `EncodeFrameStreaming` at e7 d<3 when no extra
channels are present. Also `JxlEncoderAddChunkedFrame` with input callbacks,
`JxlEncoderSetOutputProcessor`, `JxlEncoderFlushInput`, and every
`JXL_ENC_FRAME_SETTING_BUFFERING` value. This makes step 1 usable for
photographs of any size.

**Closes.** `enc_frame.cc`: `CanDoStreamingEncoding`, `EncodeFrameStreaming`,
`ComputePermutationForStreaming`, `PermuteGroups`, `TOCBucket`/`TOCSize`,
`ComputeGroupDataOffset`, `OutputGroups`, and the streaming parts of
`OutputAcGlobal`. `enc_modular.cc`: `ComputeStreamingAbsoluteAcGroupId`,
`PrepareStreamParams`, and streaming DC groups. `encode_internal.h`/`encode.cc`:
the complete `JxlEncoderOutputProcessorWrapper` (seek, finalized position,
external buffers, `AppendBufferToExternalProcessor`), the callback-backed
`JxlEncoderChunkedFrameAdapter`, and `JxlChunkedFrameInputSource` copying.

**Depends on.** Step 1.

**Gate.** Byte-exact at e7 d1.0 and d2.0 for 768x768, 1024x1024, 2048x1536,
2049x2049 and 4000x3000 (buffering -1, 0, 1, 2, 3), gray and RGB, serial, in
both buffer and output-processor mode. Compare the processor call sequence
(get buffer, release, seek, set finalized position) exactly. Rejection covers
callback errors, short buffers, invalid seeks and rect-bounds violations.
Upstream cases: `encode_test` ChunkedFrameAdapterRectBounds,
OutputModeComparisonTest, and the EncoderStreamingTest suite (OutputCallback,
ChunkedFrame, ChunkedAndOutputCallback) for its admitted instances;
the VarDCT instances of `jxl_test` StreamingSamePixels, run serially against
the original's runner-free history.

**Risks.** Streaming changes heuristic inputs per DC group and group
permutation, so checkpoints must cover per-group state. Output-processor
call sequences are public behavior and must match even when the bytes do.

**Milestone A (steps 1–2).** Public lossy VarDCT encode of SDR gray/RGB of any
size at default settings, byte-identical with libjxl.

### 3. Container and box writing

**Scope.** JPEG XL containers: `ftyp`, `jxll`, `jxlc`, `jxlp` with counters
(including the 12-byte gap that one-shot `output_mode == 2` reserves), `jxli`
frame index, and uncompressed metadata boxes (`Exif`, `xml `, `jumb`, others)
through `JxlEncoderUseContainer`, `JxlEncoderSetCodestreamLevel`,
`JxlEncoderUseBoxes`, `JxlEncoderAddBox` with `compress_box = false`, and
`JxlEncoderCloseBoxes`. Level 10 (`jxll`) becomes available, which
`VerifyFrameEncoding` in `encode_test.cc` needs.

**Closes.** `encode_internal.h`: `MakeContainerHeader`, `AppendBoxHeader`,
`JxlEncoderQueuedBox`, `JxlEncoderFrameIndexBox`. `encode.cc`:
`WriteBoxHeader`, `AppendBox`, `AppendBoxWithContents`,
`WriteJxlpBoxCounter`, `QueueBox`, `EncodeFrameIndexBox`,
`MustUseContainer`, and the container branches of `ProcessOneEnqueuedInput`.
`brob` compression stays an admission result until step 11.

**Depends on.** Steps 1–2. The ported box parser validates decode-back.

**Gate.** Byte-exact containers for every box order and size (small, 64-bit
extended sizes, unbounded last box), with frames before and after boxes, in
buffer and output-processor modes. Rejection covers boxes after
`CloseBoxes`, reserved types and level 5 violations. Upstream cases:
`encode_test` SingleFrameBoundedJXLCTest, CodestreamLevelTest,
CodestreamLevelVerificationTest, and the EncodeBoxTest suite without
compression; the `roundtrip_test` ExtraBoxesTest and `use_container`
instances.

**Risks.** Low numeric risk. Container offsets in the streaming path depend on
the output processor's seek history.

### 4. Alpha and extra channels with VarDCT

**Scope.** Alpha (straight and premultiplied, 1–16 and float bits) and every
extra-channel type in VarDCT frames. These channels are coded with Modular in
the same frame at the frame's effort. Also covers
`JxlEncoderSetExtraChannelInfo/Name/Buffer/Distance`, `IsAlphaBufferOpaque`
alpha stripping, `SimplifyInvisible`, `keep_invisible`, and the
`ec_resampling` default.

**Closes.** `enc_frame.cc`: general `CopyExtraChannels`, `SimplifyInvisible`.
`enc_modular.cc`: the extra-channel parts of `ComputeEncodingData`
(`float_to_int`, `QuantizeChannel`, `try_palettes`,
`do_transform`/`maybe_do_transform`, lossy extra-channel quantization),
`ComputeTree`, `ComputeTokens`, `EncodeGlobalInfo`, `EncodeStream` at efforts
2–9. `encode.cc`: the alpha and extra-channel setters and the opacity scan.

**Depends on.** Steps 1–3. The Modular parts are the first slice of step 5.

**Gate.** Byte-exact at e7 d1.0 with RGBA8/16, gray-alpha, float alpha,
premultiplied alpha, fully opaque alpha (stripped), alpha with distances 0 and
1, and the extra-channel cases of `Uint8FrameRoundtripTest`. Upstream cases:
`encode_test` FrameEncodingTest, DefaultAllocTest, EncoderResetTest,
AddFrameBeforeBasicInfoTest, AddFrameAfterCloseInputTest, StripAlphaSetting,
BasicInfoTest, the admitted part of FrameSettingsTest; `jxl_test`
RoundtripSinglePixelWithAlpha, RoundtripAlpha, RoundtripAlphaPremultiplied,
RoundtripAlphaNonMultipleOf8, RoundtripAlpha16; the remaining
`roundtrip_test` frame-roundtrip instances.

**Risks.** Modular tree learning at e7 uses sampled properties
(`modular_ma_tree_learning_percent`) and libc++ algorithm order. The ported
`libcpp_sort` and the existing learning boundary must also cover the new
callers.

**Milestone B (steps 1–4).** A ferrixel-ready SDR output path with alpha and
metadata boxes.

### 5. General Modular image and frame encoding

**Scope.** Modular frames beyond Thunder: lossless (`d = 0`) and lossy Modular
at efforts 2–11 (effort 1 is step 8), every predictor and tree option, global,
local and channel palettes, RCT search, Squeeze and `responsive`, the
`modular_group_size_shift` choices, `uses_original_profile`, custom bit depths,
float lossless, `JxlEncoderSetFrameLossless`, and the streaming Modular path
that `CanDoStreamingEncoding` selects for lossless images. This also closes the patch
reference-frame encode that step 1 admits away. It completes the README item
on general Modular encoding.

**Closes.** The remaining `ModularFrameEncoder` parts of `enc_modular.cc`
(`ComputeEncodingData` for color, `MergeTrees`, lossy palette and delta
palette, group and stream options), the Modular branches of `enc_frame.cc`,
`RoundtripPatchFrame` and the bin packing of `FindBestPatchDictionary` in
`enc_patch_dictionary.cc`, `PatchDictionaryEncoder::SubtractFrom`/`Encode`, and
the kTectonicPlate retry loop of `EncodeFrame`.

**Depends on.** Steps 1–4.

**Gate.** Byte-exact for e2–e11 lossless and lossy Modular (d0.5, d1, d3)
over gray, RGB, RGBA, 8/16-bit, float, palettized and screen content, with
`responsive` and group-size variants, plus VarDCT e7 screen content with
patches. Upstream cases: all of `modular_test.cc`; `jxl_test`
RoundtripLossless8 and its Thunder/Falcon/Alpha/16-bit/Gray variants,
RoundtripLargeEmptyModular, RoundtripSmallPatches, RoundtripSmallPatchesAlpha,
RoundtripUnsignedCustomBitdepthLossless, LosslessPNMRoundtrip,
LosslessSmallFewColors, the Modular instances of StreamingSamePixels;
`patch_dictionary_test.cc`; the Modular
`speed_tier_test` instances.

**Risks.** e10/e11 multiply search loops, which makes the corpus expensive.
Lossy Modular uses float quantization in Squeeze. Patch detection relies on
float similarity thresholds and an `std::sort` over patch sizes, whose order
for equal areas follows libc++.

### 6. Color profiles, ICC and HDR input

**Scope.** Non-sRGB enum encodings and ICC input through
`JxlEncoderSetColorEncoding`/`JxlEncoderSetICCProfile` and `JxlEncoderSetCms`:
the CMS conversion to linear sRGB before XYB, original-profile lossy encoding,
PQ/HLG and intensity targets, CMYK (black channel), and ICC serialization.

**Closes.** `enc_image_bundle.cc` (`ApplyColorTransform`, `TransformIfNeeded`,
`ImageBundle::TransformTo/CopyTo`), the CMS branch of `ToXYB` and
`RgbToYcbcr` in `enc_xyb.cc`, `enc_icc_codec.cc` (`PredictICC`,
`PredictAndShuffle`, `Unshuffle`, `WriteICC`), and the ICC paths of `encode.cc`.

**Depends on.** Steps 1–4, the ported default CMS and `ColorSpaceTransform`.

**Gate.** Byte-exact for Display P3, Rec. 2020 PQ and HLG, gamma 2.2, gray
gamma, and ICC profiles from testdata (including a CMYK profile), in XYB and
original-profile modes. The ICC byte stream is checked against the original
`WriteICC` independently of image data. Upstream cases: `encode_test` CmsTest,
LossyEncoderUseOriginalProfileTest, EncoderTest.CMYK; `roundtrip_test`
TestICCProfile, TestNonlinearSrgbAsXybEncoded; `jxl_test`
RoundtripOutputColorSpace, RoundtripRGBToGrayscale.

**Risks.** The CMS path runs translated skcms numerics before the heuristics,
so any skcms divergence surfaces as encoder bytes. The CMS callback contract
(caller-owned buffers, thread indices) is public behavior.

### 7. VarDCT across all efforts, distances and image features

**Scope.** Complete the VarDCT encoder for e1–e11 and every distance: the
kWombat-to-kLightning simplifications; kKitten and slower with
Butteraugli-driven `FindBestQuantization` and the linear-sRGB copy;
`max_error_mode` (`FindBestQuantizationMaxError`);
`disable_perceptual_optimizations`; dots at d≥3; 2x resampling at d≥10
(`DownsampleImage2_Iterative`/`Sharper`); `already_downsampled`;
noise (`noise`, `photon_noise_iso`, manual LUT); gaborish/EPF overrides;
`manual_xyb_factors`; `decoding_speed_tier`; custom splines as far as reachable.
It completes the README item on the VarDCT encoder together with step 9.

**Closes.** The remaining branches of `enc_heuristics.cc` and
`enc_adaptive_quantization.cc` (`FindBestQuantization`, `RoundtripImage`,
`TileDistMap`, the `DumpHeatmap` hooks as no-ops without a debug callback);
`butteraugli/butteraugli.cc`, `enc_butteraugli_comparator.cc`,
`enc_comparator.cc`, `enc_convolve_separable5.cc`; `enc_detect_dots.cc`,
`enc_dot_dictionary.cc`, `enc_linalg.cc`, `enc_optimize.h`; `enc_noise.cc`,
`enc_photon_noise.cc`; `enc_splines.cc`; the remaining branches of
`enc_ac_strategy.cc` and `enc_chroma_from_luma.cc`; and `enc_frame.cc`'s
`ComputeNoiseParams`, `DownsampleColorChannels`, `LoopFilterFromParams` paths.

**Depends on.** Steps 1–6.

**Gate.** Byte-exact for every effort 1–11 at d0.1, d1, d2, d3.5, d6, d12 and
d25, serial, on photographic and synthetic content. Butteraugli itself gets
component cases: exact diffmap words against the original comparator.
Upstream cases: all `speed_tier_test` instances; `jxl_test`
RoundtripTinyFast, RoundtripResample2/4/8, RoundtripResample2Slow,
RoundtripMultiGroup, RoundtripLargeFast, RoundtripLargeConsistent (serial),
RoundtripSmallNL, RoundtripNoGaborishNoAR, RoundtripSmallNoGaborish,
RoundtripDots, RoundtripDotsForceEpf, RoundtripDisablePerceptual,
RoundtripNoise, RoundtripAlphaResampling, RoundtripAlphaResamplingOnlyAlpha,
the d8.0 case of RoundtripGrayscale;
`roundtrip_test` Resampling; `enc_linalg_test`, `enc_optimize_test`,
`enc_photon_noise_test`; `ac_strategy_test` encode cases.

**Risks.** Butteraugli is 2,189 lines of float convolutions, blurs and
nonlinearities. Its result feeds iterative quant-field search, so it amplifies
any ULP difference into different bytes. `enc_optimize.h`
(conjugate-gradient) and dots' linear algebra add more `std::sqrt`/`std::exp`
and summation-order sensitivity. The corpus is slow at e8–e11; the gate keeps
small images for slow tiers and records its run time.

### 8. Effort 1 fast lossless

**Scope.** `JxlEncoderSetFrameLossless` at e1 (kLightning), which `encode.cc`
routes through `enc_fast_lossless.cc` (`JxlFastLosslessPrepareFrame`,
`JxlFastLosslessProcessFrame`, `QueueFastLosslessFrame`,
`FastLosslessRunnerAdapter`).

**Depends on.** Step 1, and the extra-channel setters of step 4 for RGBA
input. Independent of steps 5–7.

**Gate.** Byte-exact for 8/16-bit gray/RGB/RGBA, odd sizes, multi-group, and
the chunked and output-processor modes. Upstream cases: `jxl_test`
RoundtripLossless8LightningGradient and the kLightning `speed_tier_test`
instances.

**Risks.** This path does not use Highway. Its own `FJXL_ENABLE_NEON`,
`FJXL_ENABLE_AVX2` and `FJXL_ENABLE_AVX512` defaults compile NEON on AArch64
and dispatch AVX2/AVX-512 at runtime on x86, so the oracle's scalar Highway
setting does not reach a scalar path here. Before porting, record in ADR 0002
whether the oracle compiles this unit with the SIMD paths disabled (a scoped
flag, like the existing ones), or the port translates each target path the
reference reaches. Without that decision there is no defined expected output.

### 9. Progressive encoding

**Scope.** `progressive_ac` and `qprogressive_ac` with multi-pass
`SplitACCoefficients`, custom pass definitions, `progressive_dc` 1–2
(recursive DC frames through `EncodeFrame` plus `DecodeFrame`), `responsive`
for VarDCT extra channels, and group order (`group_order`,
`group_order_center_x/y`).

**Closes.** `enc_progressive_split.cc` (`SplitACCoefficients` for all modes),
the progressive-DC branch of `InitializePassesEncoder` in `enc_cache.cc`,
`SetProgressiveMode` and the center-first permutation in `enc_frame.cc`,
multi-pass `EncodeGroups`/`ComputeAllCoeffOrders`.

**Depends on.** Steps 1–7.

**Gate.** Byte-exact for every pass mode at e3, e7 and e9 with d1 and d3, with
progressive DC 0, 1 and 2, center group orders, and decode-back through the
progressive decoder events (`JxlDecoderSetProgressiveDetail`) on both
decoders. Upstream cases: all of `passes_test.cc`; `jxl_test`
RoundtripProgressive, RoundtripProgressiveLevel2Slow.

**Risks.** Recursive DC frames run a complete encoder and decoder inside the
encoder, multiplying the numeric surface of steps 1 and 7.

### 10. Animation, layers and multi-frame input

**Scope.** Multiple queued frames and animation: `JxlEncoderSetFrameHeader`,
durations, timecodes, blend info and extra-channel blend info, cropped frames
and layer offsets, `save_as_reference`, frame names,
`JxlEncoderSetFrameBitDepth`, `JxlEncoderSetUpsamplingMode`, mixed settings
per frame, the frame index box, and the preview frame (reached only through
`test::EncodeFile`'s `EncodePreview`).

**Closes.** The multi-frame and `FrameInfo` paths of `encode.cc` and
`enc_frame.cc` (`MakeFrameHeader` with crops and references, `FrameInfo`
fields, `EncodePreview` in `test_utils.cc` translated as a test helper), and
the queued-frame lifecycle in `encode_internal.h`.

**Depends on.** Steps 1–5. Step 7 for lossy patch frames across animations.

**Gate.** Byte-exact animations (lossy, lossless, with patches across frames),
layered stills, cropped frames, references and blend modes. Upstream cases:
`encode_test` AnimationHeaderTest, CroppedFrameTest, FrameSettingsTest
(complete); `roundtrip_test` MultiFrameTest; `jxl_test` RoundtripAnimation,
RoundtripLosslessAnimation, RoundtripAnimationPatches,
RoundtripOutOfOrderProcessing(Border); `preview_test.cc`.

**Risks.** Cross-frame patch and reference state. Error paths for invalid
references and blend sources must preserve the original's partial output.

### 11. Brotli encoder dependency

**Scope.** Mechanically translate the pinned Brotli encoder (`c/enc` at
`third_party/brotli` `db24a98`), as far as `BrotliEncoderCreateInstance`,
`SetParameter` (quality, size hint), `CompressStream`, `HasMoreOutput`,
`IsFinished`, `MaxCompressedSize` and `DestroyInstance` reach it, next to the
existing Brotli decoder module. Then connect `brob` boxes
(`BrotliCompress` in `encode.cc`, `compress_box = true`).

**Depends on.** Nothing in this plan. It can run in parallel with steps 2–10.
Its `brob` connection needs step 3.

**Gate.** The existing Brotli evidence style: independent source-bound
observers, exact compressed bytes against the original for each quality 0–11
over a corpus that includes the JPEG metadata layouts of step 12, decoder
round trips through both Brotli decoders, and allocator and state checkpoints.
Upstream cases: the compressed EncodeBoxTest instances; `jxl_test`
RoundtripJpegRecompressionMetadata.

**Risks.** About 25,000 lines including dictionary tables and hashers.
Quality 10–11 (Zopfli-style search) is float-heavy and slow. The encoder
selects hashers and match finders by quality and window, so each quality the
callers use (brob at `brotli_effort` or 4; JPEG data at its effort) must be
covered.

### 12. JPEG input and lossless JPEG recompression

**Scope.** `JxlEncoderAddJPEGFrame` and `JxlEncoderStoreJPEGMetadata`: JPEG
parsing, coefficient transcoding into VarDCT frames, the `jbrd`
reconstruction box, and Exif/XMP/ICC extraction into boxes and color
encoding. It completes the README item on JPEG recompression.

**Closes.** `jpeg/enc_jpeg_data_reader.cc` (`ReadJpeg`),
`jpeg/enc_jpeg_huffman_decode.cc`, `jpeg/enc_jpeg_data.cc` (`EncodeJPEGData`,
`SetColorEncodingFromJpegData`, the remaining helpers),
`ComputeJPEGTranscodingData` and `FindAvgIndexOfSumMaximum` in `enc_frame.cc`,
the JPEG branches of `ComputeEncodingData`, `AddVarDCTDC` with
`jpeg_transcode`, and `JxlEncoderAddJPEGFrame` in `encode.cc`.

**Depends on.** Steps 1–3 and 11. Step 10 for JPEG frames in multi-frame input.

**Gate.** Byte-exact containers for baseline, progressive, restart-marker,
4:2:0/4:2:2/4:4:0/asymmetric, gray, ICC, Exif/XMP and APP-marker JPEGs from
testdata and genuine cjpeg files. The original bytes must reconstruct exactly
through the Rust and original `JxlDecoderSetJPEGBuffer` paths. Rejection covers
malformed, truncated and unsupported JPEGs (arithmetic coding, 12-bit) with
identical statuses. Upstream cases: `encode_test` JPEGReconstructionTest,
ProgressiveJPEGReconstructionTest, JPEGFrameTest, AddJPEGAfterCloseTest;
`roundtrip_test` TestJPEGReconstruction, TestJPEGReconstructionNon444LFS;
all 17 `jxl_test` RoundtripJpegRecompression* cases.

**Risks.** `ReadJpeg` is a large malformed-input surface; fuzzing belongs to
this step's acceptance. JPEG transcoding at e7+ runs CfL and block-context
heuristics on integer coefficients.

### 13. Parallel runner, allocator, statistics and debug hooks

**Scope.** `JxlEncoderSetParallelRunner` over the encoder's `RunOnPool`
schedules, the caller's memory manager in every encoder owner,
`JxlEncoderCollectStats`/`JxlEncoderStats*`, and
`JxlEncoderSetDebugImageCallback` with `enc_debug_image.cc`.

**Closes.** The runner and allocator paths of `encode.cc` and of every ported
`RunOnPool` site, the encoder's `AlignedMemory`/`Image` owners through
`memory_manager_internal`, `enc_debug_image.cc`, `enc_debug_tree.cc`, and the
diagnostic `Dump*` hooks.

**Depends on.** The runner part can start after step 2 for the schedules
ported by then. The step completes only after steps 1–12, when every schedule
exists.

**Gate.** Every earlier corpus entry repeated with the thread and resizable
runners at 1, 2, 4 and 8 threads gives the original's runner-free bytes, on
both implementations. Allocator histories match the original's allocation
sizes and order. Statistics and debug images compare exactly. Upstream cases:
`encode_test` CustomAllocTest, DefaultParallelRunnerTest; `jxl_test`
RoundtripResample2MT and the runner variants of the consistency tests.

**Risks.** The original is designed to be runner-independent, but per-thread
scratch (`AcStrategyHeuristics::PrepareForThreads`, per-thread `AuxOut`
assimilation, `std::atomic` counters in patch detection) must be shown to
leave the bytes unaffected. As in the decoder, the Rust runners first execute
task bodies serially; concurrent execution is a separate item.

### 14. Encoder C ABI and full inventory

**Scope.** Remove the last admission results and export the 49 functions of
the pinned `encode.h` under their original names. Run the complete encoder
test inventory as differential cases.

**Closes.** `decode_abi.rs`'s counterpart for `encode.h`, struct and enum
layout checks, and the remaining `encode.cc` entry points.

**Depends on.** Steps 1–13.

**Gate.** The C driver of the decoder ABI, extended to encoder histories,
linked against the original archive and the Rust static library, reproduces
every in-process history exactly. Every test of `encode_test.cc`,
`roundtrip_test.cc`, `passes_test.cc`, `jxl_test.cc`, `speed_tier_test.cc`,
`modular_test.cc` and the encoder unit tests is a differential case or
recorded as not applicable with a reason, as for `decode_test.cc`. The README
encoder items, container writing, encoder allocator, `JxlEncoderSetParallelRunner`
and encoder C ABI items are then checked.

**Risks.** `gain_map.h` (`JxlGainMap*Bundle`) is an exported library API that
is neither in `encode.h` nor in this plan; schedule it with the remaining core
helpers.

## Cross-cutting risks

- **Floating-point contraction.** Clang's default `-ffp-contract=on` fuses
  `a*b+c` into FMA on AArch64 but not on baseline x86-64. ADR 0002 and ADR 0003
  already keep this per target: Rust uses explicit `mul_add` where the AArch64
  reference contracts and separate operations on x86. The encoder has
  thousands of such expressions. Each ported unit needs a contraction map from
  the original objects' disassembly (FMADD/FMSUB/FNMADD sites), checked by an
  audit tool in the style of the existing `*_representation.py` scripts.
- **Evaluation order and promotion.** Keep C++ overload resolution:
  `std::pow(float, float)` is `powf`, `std::pow(float, int)` promotes to
  double, and `0.79 / distance` is a double expression narrowed later. Keep
  operand order, `std::min/max` NaN semantics and unordered comparisons, as in
  the color matrices.
- **libm.** `std::exp`, `std::log`, `std::pow` and `std::cbrt` come from the
  host libm on both sides (glibc, libSystem). Parity is per host. Rust must
  call the function that the C++ overload selects (`expf` or `exp`), and
  compile-time folding of constant arguments needs the same exact-word
  evidence as runtime calls.
- **Heuristic sensitivity.** Integer outputs (strategies, quant values, tree
  splits) depend on float thresholds. A one-ULP difference moves a decision and
  rewrites all later bytes. Checkpoint fixtures per heuristic stage are
  therefore required, not optional.
- **SIMD-only paths.** The scalar Highway reference gives the one-lane
  semantics for Highway code (`HWY_FULL`, `HWY_CAPPED`, `MaxVectorSize`
  padding). `enc_fast_lossless.cc` bypasses Highway (step 8). The existing
  scalar subsets of `enc_ans_simd` and `enc_modular_simd` must be extended to
  every reached function.
- **libc++ algorithms.** `std::sort`, `std::nth_element`, `std::stable_sort`
  and priority-queue order for equal keys follow the recorded libc++ version
  (`base/libcpp_sort.rs`). Every new call site needs that mapping.
- **Threading.** Encoder bytes must not depend on the runner or thread count.
  Step 13 proves this; until then every gate is serial.
- **Corpus cost.** e8–e11 and Butteraugli make cases slow, especially under
  ASan. Keep the slow-tier images small and partition them like the existing
  checked tests.
- **Admission drift.** Admission checks are temporary code that the original
  does not contain. Each step deletes the checks it makes obsolete, and the
  corpus asserts that every remaining admission corresponds to an original
  branch.

## First step

Start step 1 with the bottom-up units that later fixtures need, in one
worktree per PR:

1. **ADR and harness PR.** Amend ADR 0001 in place for the admitted public
   encoder (see [Public API admission](#public-api-admission)) and add the
   oracle unit `tools/oracle/encoder_public.cc`. It takes a caller history
   (basic info, color encoding, frame settings, pixel buffer, `avail_out`
   schedule) and records every status, error and output chunk from the
   original `JxlEncoder*` API. Add its Rust driver,
   `tests/parity/encoder_public.rs`, which first runs only the rejection
   histories that need no frame encode (call order, invalid options, basic
   info validation). Port the corresponding `encode.cc` setters and
   `encode_internal.h` owners as storage.
2. **Transform and color PR.** `Symmetric5` from `convolve_symmetric5.cc`,
   `GaborishInverse` from `enc_gaborish.cc`, `TransformFromPixels`,
   `DCFromLowestFrequencies` and `AFVDCT4x4` from `enc_transforms*.{h,cc}`,
   and the sRGB/linear-sRGB `ToXYB` branches with `ComputePremulAbsorb` from
   `enc_xyb.cc`. Gate: exact float words against the original functions for all
   27 strategies, every `xsize mod 8`, exceptional values as in the existing
   DCT fixtures, and all of `enc_gaborish_test.cc`, with the AArch64
   contraction map recorded.
3. **Quantization heuristics PR.** `quantizer.cc` field selection,
   `InitialQuantField` and `AdjustQuantField`, then `CfLHeuristics`, then
   `AcStrategyHeuristics`. Each gets checkpoint fixtures on the step 1 image
   corpus at kSquirrel.
4. **Coefficient and DC PR.** `enc_group.cc`, `TokenizeCoefficients`,
   coefficient-order computation, `InitializePassesEncoder`,
   `ComputeACMetadata`, `AddVarDCTDC`/`AddACMetadata`, default AC matrix
   encoding.
5. **Decoder-in-the-loop and patch detection PR.** `DecodeGroupForRoundtrip`,
   `ReconstructImage`, `ComputeARHeuristics`, `FindTextLikePatches`.
6. **Integration PR.** `LossyFrameHeuristics`, the VarDCT
   `ComputeEncodingData`/`EncodeFrameOneShot`, `AddImageFrame` and
   `ProcessOutput`, and the full step 1 gate.

Progress:

- PR 1 (ADR and harness): ADR 0001 admits the public encoder with run-time
  admission results. `encode.rs` translates the encoder object, its lifecycle,
  every option and metadata setter, frame and box queueing and ProcessOutput;
  `encode_internal.rs` adds the queued owners and the buffer-mode output
  wrapper. The admitted profile is empty: AddImageFrame returns an admission
  result where CanDoFastLossless selects `enc_fast_lossless.cc`, and
  ProcessOutput where it would call ProcessOneEnqueuedInput.
  `tools/oracle/encoder_public.cc` and `tests/parity/encoder_public.rs` compare
  caller histories, stored-state checkpoints, allocator logs and the wrapper
  with the original; `epub-probe` supplies the original's branch evidence for
  each admission result. See
  [PORT_STATUS.md](PORT_STATUS.md#public-encoder-api-admission-and-setters-2026-11-05).
  A source audit for the unit is still open. The next PRs widen ProcessOneEnqueuedInput: its raw-codestream header
  path (WriteCodestreamHeaders, level checks) lands with the integration PR,
  which deletes the ProcessOutput admission for the admitted profile.

Step 1 is done when PR 6's gate passes on native Linux and macOS against both
oracle graphs, and [PORT_STATUS.md](PORT_STATUS.md) and this plan record the
admitted profile and evidence. The README encoder items stay unchecked until
their complete scope is delivered.
