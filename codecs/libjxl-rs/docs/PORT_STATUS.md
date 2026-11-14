# Port status

This is the JXL port and historical evidence record in the
[Ferrisma workspace](../../../README.md). See the [current project status](../../../docs/status.md)
and [codec guide](../../../docs/codecs.md) for integration and validation cadence.
Ferrisma currently uses `jxl-oxide` for JPEG XL input; this port is not its decoder.
Historical PR and CI references below describe their original revisions, not a
current release or platform acceptance.

This port contains complete Rust raw-codestream paths for the fixed
lossless Thunder profile and the first VarDCT profile. VarDCT decoding connects
global metadata/frame parsing, actual DC/AC groups, dequantization, all scalar
inverse transforms, optional Gaborish, XYB conversion and high-precision sRGB
output. The admitted profile supports gray/RGB, 8/16-bit samples, one optional
full-resolution unassociated alpha channel, multiple AC passes and DC/AC groups.
Scalar EPF and noise/spline effects are connected for this profile. The internal
sequence decoder also connects patches, general NONE/XYB Modular frames,
reference/DC frames, blending, animation, previews and the admitted YCbCr/chroma
paths. Associated/subsampled alpha and every source-defined extra-channel type are
connected at this internal boundary. Complete geometry/orientation is connected
through the verified source boundary below.
See [VARDCT_IMAGE.md](VARDCT_IMAGE.md) for the complete configuration and evidence.
The lossless encoder/decoder profile and its existing corpus remain unchanged.
There are no C++ runtime fallbacks and no placeholder codec APIs. The source revision is pinned in
[upstream.json](../upstream.json).

The first six decoder expansion steps are merged into `main` through
PR #30 (retired libjxl-rs repository). Its
native acceptance run (retired libjxl-rs repository, run 41493531478)
passes all six ordinary/ASan/UBSan Debug, Release and Rust 1.85 configurations
on both Linux and macOS: 307 differential groups and 926,392 exact defined
requests per configuration, with zero failures or ignored tests. Linux runs
514 tests per configuration; macOS runs 513. The independent original-source
audits and retained lifecycle corpus also pass. The remaining decoder scope is
tracked in [DECODER_EXPANSION.md](DECODER_EXPANSION.md).

The public decoder API from `decode.cc` is ported in `src/jxl/decode.rs`
(PR #50 (retired libjxl-rs repository)). It drives
the incremental FrameDecoder (PR #49) with the low-memory render pipeline
(PR #48), whole-image color output, tone mapping and CMS output conversions
through the translated default CMS (PR #47), and JPEG reconstruction with
Exif/XMP restoration through JxlToJpegDecoder (PR #46). It reads raw
codestreams and containers with their boxes. The
deep gate (retired libjxl-rs repository, run 41762944219)
on the merged PR #50 tree passes the ordinary and ASan/UBSan oracles and its
source audits on Linux and macOS. Every test definition in the pinned
`decode_test.cc` runs as a differential case
(PR #52 (retired libjxl-rs repository)). The
crate exports the decoder C ABI of `decode.h` from a static and a dynamic
library, so a C program can link it instead of libjxl
(PR #54 (retired libjxl-rs repository)). The thread
and resizable runners and JxlDecoderSetParallelRunner are ported and exported
under their C names, and the public decoder runs its RunOnPool schedules
through them. The sections below
record each slice, its evidence and its not-applicable checks.

Remaining decoder-side gaps: the decoder's RunOnPool task bodies are
serialized under a runner, so a runner brings no parallel speedup yet; the
encoder C ABI is absent; and the translated skcms SKX engine has no AVX-512
acceptance. Every decoder owner allocates through the caller's memory manager
(see "Decoder memory-manager propagation" below). Container writing, general
encoding, the encoder C ABI and JxlEncoderSetParallelRunner remain unported.
The public encoder API's object lifecycle, option and metadata setters, frame
and box queueing and ProcessOutput are translated with explicit admission
results for every untranslated branch (see "Public encoder API admission and
setters" below); it encodes no pixels yet. The encoder's forward transforms,
GaborishInverse and sRGB/linear-sRGB ToXYB are translated as standalone units
(see "Encoder transforms, Gaborish and XYB" below), as are the kSquirrel
quantization field, chroma-from-luma and AC strategy heuristics (see "Encoder
quantization, CfL and AC strategy heuristics" below), and the VarDCT
coefficient, order, DC, AC-metadata and text-like patch detection units (see
"Encoder coefficients, DC streams and patch detection" below), and the
adaptive-reconstruction heuristics with the decoder in the loop (see "Encoder
AR heuristics with the decoder in the loop" below). The public
encoder encodes one lossy VarDCT frame at effort 7 and distances below 3.0,
the default d1.0 included, byte for byte with the original, one-shot and
streaming (see "Public lossy VarDCT encode below d0.7" below, which records
the later extensions), with alpha and other extra channels and with the
patches option off (see "Lossy VarDCT with extra channels and patches off"
below); other frame encodes remain admission results.

## SIMD foundation: NEON/AVX2 layer and first stages (2026-11-13)

[ADR 0004](adr/0004-simd-paths.md). `src/jxl/hwy` translates 63 Highway
operations for `HWY_NEON` and `HWY_AVX2`; the SIMD Gaborish, EPF (passes
0–2), YCbCr and chroma upsampling stages are written against it. They match
the NEON and AVX2 oracle variants at row level on both targets
(`tests/simd_parity.rs`). The stage factories now dispatch on
`hwy::decoder_target()` with target row padding, but the decoder does **not**
select SIMD by default (`hwy::DECODER_SELECTS_SIMD` is off): the new
whole-decode comparison with the NEON oracle differs on every VarDCT file,
with or without the SIMD stages, because the original's NEON build also runs
dequantization, IDCT, XYB and output kernels as SIMD (ADR 0004 evidence).
The other Highway exports remain open (issues #96, #231). The row adapters of the oracle now hand the stages
128-byte aligned rows, as libjxl's pipelines do; the misaligned rows made the
AVX2 oracle fault on hosted x86-64 hardware (issue #252, ADR 0004 evidence).
The fix is verified on AArch64 only; x86-64 cannot run on the local machine.
<!-- render stages -->
Render stages of ADR 0004 Plan item 1 (2026-11-13): the SIMD XYB and
FromLinear stages (`stage_xyb_simd.rs`, `stage_from_linear_simd.rs`, all six
transfer operations) and the output conversions of WriteToOutputStage
(`stage_write_simd.rs`: integer, binary16, unpremultiplied alpha) match the
NEON oracle at row level (`tests/simd_render.rs`) and dispatch like the
stages above; the layer has 67 operations. Whole decodes still differ
(ADR 0004 table). AVX2 is compile-checked only.
<!-- end render stages -->

<!-- vardct kernels -->
VarDCT decode kernels (2026-11-13): the SIMD inverse/forward DCTs,
TransformToPixels, LowestFrequenciesFromDC, DequantBlock with
AdjustQuantBias and the quant tables (ComputeQuantTable) match the NEON
oracle word for word (`tests/simd_vardct.rs`, ADR 0004 "VarDCT decode
kernels") and dispatch on `hwy::decoder_target()` on both targets; on AVX2
the dequantization executes the original's vendor-specific `rcpps`, so AVX2
output can differ between CPU vendors as the original's does (identity per
host, working decision 2026-11-13 evening, delegated, revisable by the
owner; O4 #250 stays open for shipped defaults). AVX2 compiles but was not
run. With the render stages and the SIMD AdaptiveDCSmoothing/DequantDC,
all 24 admitted whole decodes of `decode_vardct` match the NEON oracle word
for word and byte for byte; the whole-decode test now asserts that with the
SIMD target forced. The default selection stays off until the per-target
conditions of ADR 0004 "Decoder path" hold (all reachable exports, public
decoder and low-memory pipeline comparison, a scalar gate that forces the
scalar path, real x86-64 evidence for AVX2).
First hosted AVX2 run (2026-11-13): oracle adapters fixed to give the
original aligned buffers and group-aligned DC rectangles as its callers do;
the SIMD path now fuses the CfL ratio as the original's AVX2
DecodeGroupImpl does (ADR 0004). AVX2 re-run pending.
<!-- end vardct kernels -->

<!-- public gate -->
Public decoder gate (2026-11-13): `tests/simd_public.rs` compares whole
decodes through the public decoder and its low-memory pipeline, SIMD target
forced, with the original's public decoder at the same target (float, 8-
and 16-bit output) over the `decode_vardct` gate files, the scalar suite's
public-decoder inputs and the pinned conformance corpus. NEON: 276 of 352
distinct inputs match word for word; the 76 others are the executable punch
list, each reaching a still-scalar `Upsample`, noise or splines stage (ADR
0004 "Public decoder gate"). The gate found and fixed a read before the row
start of the SIMD stages in the low-memory pipeline. The scalar gate now
forces the scalar decoder path (`JXL_DECODER_TARGET=scalar`). On the
combined branch with every SIMD package (2026-11-14), all 352 inputs match
on NEON, also with the default CMS and a Rec. 2020 PQ output profile, which
reaches the `Cms` stage; no input is HDR, so `ToLinear` and `ToneMapping`
are not reached by this gate.
<!-- end public gate -->

<!-- modular -->
Modular and output-conversion kernels (2026-11-13, branch `v1/jxl-modular`):
the SIMD `FloatToU32`/`FloatToF16` of dec_external_image
(`dec_external_image_simd.rs`) match the NEON oracle word for word
(`tests/simd_modular.rs`) and `ConvertChannelsToExternal` selects them on
`hwy::decoder_target()`. The whole-decode comparison still reports zero
differing words and bytes on all 24 VarDCT files with the SIMD target
forced. The SIMD `MultiplySum`/`RgbFromSingle`/`SingleFromSingle`
(`dec_modular_simd.rs`), inverse RCT (`rct_simd.rs`) and inverse squeeze
(`squeeze_simd.rs`, `FastUnsqueeze`) match the NEON oracle as well and are
dispatched on the same target, with twelve new signed 32-bit layer
operations. `convolve_symmetric5` and `simd_util`'s `MaxValue` are encoder
only. AVX2 compiles and is not yet run.
<!-- end modular -->

<!-- stages2 -->
Upsampling, noise, blending and splines stages of ADR 0004 Plan item 1
(2026-11-13): the SIMD upsampling stage (`stage_upsampling_simd.rs`, 2x, 4x
and 8x) matches the NEON oracle at row level and inside the simple and
low-memory pipelines; the SIMD noise generation and the AddNoise and
ConvolveNoise stages (`dec_noise_simd.rs`, `stage_noise_simd.rs`) match at
row level (`tests/simd_stages2.rs`, ADR 0004 "Upsampling, noise, blending
and splines stages"). Both dispatch on `hwy::decoder_target()`; the layer
has 69 operations. The blending stage has no Highway arithmetic (it calls
the non-Highway PerformBlending), so one stage serves every target; the
scalar suite's blending requests replayed against the NEON oracle match.
Spline rendering (`splines_simd.rs`: SegmentsFromPoints/ContinuousIDCT and
DrawSegments) dispatches on `hwy::decoder_target()` and matches the NEON
oracle over the replayed scalar spline requests. AVX2 is compile-checked
only.
<!-- end stages2 -->

<!-- color -->
Color stages of ADR 0004 Plan item 1 (2026-11-13): the SIMD ToLinear stage
(`stage_to_linear_simd.rs`, all seven operations, with the decode direction
of the transfer functions in `simd_transfer.rs`) and the SIMD tone-mapping
stage (`stage_tone_mapping_simd.rs`, Rec. 2408 tone mapping, HLG OOTF and
gamut mapping) match the NEON oracle at row level (`tests/simd_color.rs`)
and dispatch like the stages above. The default CMS's `run`
(`DoColorSpaceTransform`, the only Highway export of `cms/jxl_cms.cc`)
runs its PQ and sRGB pre- and post-processing as SIMD on the selected
target (`cms/jxl_cms_simd.rs`) and matches through the CMS stage;
`GetCmsStage` has no vector code.
AVX2 is compile-checked only.
<!-- end color -->

<!-- enable -->
Scope of the decoder's selection (2026-11-14, branch `v1/jxl-enable`): the
SIMD selection reaches decoding only. `JxlDecoder` fixes its target when it
is created; its working entry points and the internal whole-decode helpers
run inside `hwy::DispatchScope::Decoding`, `RunOnPool` carries the scope to
runner threads, and outside every decoder entry point (the encoder,
including the decoder code its AR heuristics, quant tables, splines and
modular inverse transforms run) `hwy::decoder_target()` is the scalar path,
with the scalar row padding. `tests/dispatch_scope.rs` proves the encoder's
bytes and its zero SIMD selections in both orders, inside a decoder
callback and concurrently, and the decoder's target in every public decode
(ADR 0004 "Scope of the selection").
<!-- end enable -->

## Lossy VarDCT with extra channels and patches off (2026-11-12)

Encoder step 4 (issue #115) and a patches setting under which every image
encodes ([ENCODER_EXPANSION.md](ENCODER_EXPANSION.md#4-alpha-and-extra-channels-with-vardct)).

Patches off. With `JXL_ENC_FRAME_SETTING_PATCHES` = 0 (`Override::kOff`)
LossyFrameHeuristics skips FindBestPatchDictionary (its only use on this
path; dots are found inside it and stay off below d3.0), and
CanDoStreamingEncoding only tests `kOn`. The FrameProfile gate admits the
patches option at its default or off; nothing else changes. At the default,
text-like content still returns the PatchDictionary admission result.

Extra channels. `enc_frame_vardct.rs` adds CopyExtraChannels (with GetBitDepth
for the channel's metadata), SimplifyInvisible and the extra-channel parts of
the VarDCT ComputeEncodingData (alpha into CopyColorChannels, the
SimplifyInvisible condition with `keep_invisible`, `ec_resampling`, the call
of the modular encoder). `enc_modular.rs` adds
`compute_encoding_data_extra_channels`, the source ComputeEncodingData with
`do_color = false`: float_to_int per extra channel, the level bit-depth check,
the lossy option reset, global palettes (`try_palettes`), `move_to_front`,
Squeeze and the per-channel quantization of lossy extra channels, the DC and
AC stream parameters, PrepareStreamParams (global and groupwise) and the
channel release. `encode.rs` replaces the alpha admission of
ProcessOneEnqueuedInput with the alpha stripping decision (`kStripAlphaAuto`,
`Force`, `IfOpaque`) and IsAlphaBufferOpaque over 2048x256 chunks, which
removes an opaque alpha channel from the metadata, the basic info and every
queued frame. The FrameProfile gate admits any number of extra channels of
every type except black (CMYK, step 6) at `dim_shift` 0, lossless or lossy
(JxlEncoderSetExtraChannelDistance), straight or premultiplied, with any
`keep_invisible` and strip-alpha setting. Lossless extra channels keep
CanDoStreamingEncoding's streaming path above 8 groups (its extra-channel
check excludes only lossy or responsive modular parts); lossy ones take the
one-shot path. Extra channels deeper than 12 bits clear
`modular_16_bit_buffer_sufficient`, so VerifyLevelSettings requires level 10
and the original writes a container: that remains the EnqueuedInput
admission result (step 3). Type 15 (optional) fails in the original's
ProcessOutput; the Rust encoder returns the same history.

Evidence (macOS AArch64, ordinary oracle, checked profile). The epub-run
content generator gains text-like content (content 5: repeated dark glyphs on
a light background) and alpha modes (content + 16 * m: opaque, a ramp with
transparent stripes, a ramp without zeros), for the interleaved alpha sample
and for `E` extra-channel buffers; both drivers mirror it. Every admitted
family also requires that the original's last ProcessOutput succeeds with a
codestream. Whole histories match byte for byte:
`public_encoder_lossy_vardct_patches_off_matches_original` (135: 1x1 to
512x1024 and three streaming sizes, every sample type, contents 1 to 5),
`..._text_like_default_admission_matches_original` (12 text-like histories:
the Rust encoder returns PatchDictionary after an exact prefix, the
original's default output differs from its patches-off output, and the 12
patches-off histories are exact), `..._alpha_small_sizes_...` (96: 1x1 to
48x15, RGBA and gray-alpha, 8-bit and 12-bit alpha from uint8 and uint16
rows), `..._alpha_large_sizes_...` (16: 63x129 to 512x1024),
`..._alpha_streaming_...` (16: 768x768 to 2049x300 at buffering -1 to 3),
`..._alpha_variants_...` (121: opaque alpha stripped at auto and if-opaque,
forced and disabled stripping, separate alpha planes, premultiplied alpha,
float16 and float32 color rows with integer alpha, alpha depths 1, 4, 7 and
12, text-like content with alpha and patches off), `..._alpha_settings_...`
(30: keep_invisible on and off, alpha distances 0.5, 1.0 and 3.0) and
`..._extra_channel_types_...` (30: depth, spot color, selection mask,
thermal, CFA and second alpha channels, up to four extra channels).

Contraction. SimplifyInvisible's five `row[x] += 2.f * v` sites
(`tools/encoder_frame_contraction.json`) use `mul_add_f32`; float_to_int's two
sites and try_palettes' two sites (`tools/encoder_coefficients_contraction.json`)
were translated with their helpers and are now reached.
`tools/contraction_inventory.py --check` passes.

The whole `encoder_` filter of the parity suite (75 tests: these families,
the earlier lossy families and the lossless, transform, heuristics,
coefficient and AR chains) passes against the ordinary and the ASan/UBSan
oracle graphs (macOS AArch64, checked profile).

Not done: the patch reference-frame encode (patches at the default), 16-bit
and float alpha (container output, step 3), black channels, `dim_shift`,
extra-channel resampling, native Linux x86-64 runs.

## Public lossy VarDCT encode below d0.7 (2026-11-12)

The integration pull request of encoder step 1
([ENCODER_EXPANSION.md](ENCODER_EXPANSION.md#first-step)), without the
adaptive-reconstruction heuristics. `enc_heuristics_lossy.rs` translates
LossyFrameHeuristics and FindBestDequantMatrices from enc_heuristics.cc;
`enc_frame_vardct.rs` translates EncodeFrame (setting resolution),
CanDoStreamingEncoding, EncodeFrameOneShot, GetBitDepth, general
CopyColorChannels, the VarDCT branch of ComputeEncodingData,
ComputeChromacityAdjustments with PixelStatsForChromacityAdjustment, the
reached ComputeNoiseParams, DownsampleColorChannels and PermuteGroups
branches, ComputeVarDCTEncodingData, ComputeAllCoeffOrders,
TokenizeAllCoefficients, EncodeGlobalDCInfo, the non-streaming
EncodeGlobalACInfo and EncodeGroups from enc_frame.cc; `encode.rs` translates
the raw-codestream branch of ProcessOneEnqueuedInput, AddFrame and
StoreFrameIndexBox of the frame index, MustUseContainer and ProcessOutput's
loop. All units run serially, as RunOnPool does without a runner.

Admission. Untranslated branches return typed admission results at their
checkpoints: `EnqueuedInput` (alpha, ICC profiles, containers, boxes, the jxli
box), `StreamingFrame` (CanDoStreamingEncoding), `FrameProfile` (EncodeFrame's
one-shot choice for settings outside the admitted profile, a custom memory
manager, and the reached untranslated branches inside the one-shot path),
`PatchDictionary` (FindTextLikePatches found patches) and `ArHeuristics`.
ComputeARHeuristics is reached only through `compute_ar_heuristics_entry`,
which translates the source's early return (distance below
kMinButteraugliForDynamicAR, a speed tier faster than kWombat or no EPF
iterations: fill the EPF sharpness with 4) and returns `ArHeuristics`
otherwise. At kSquirrel without an EPF override the early return holds below
d0.7. The FrameProfile gate deliberately sits before the first untranslated
branch for settings whose branches are translated but not yet compared (other
efforts, overrides, unclosed frames, crops). The admitted profile is recorded
in [ENCODER_EXPANSION.md](ENCODER_EXPANSION.md#first-step).

Evidence (macOS AArch64, ordinary oracle, checked profile, 2026-11-12). The
epub-run frame operation gained generated content (flat, gradient, noise and
a photograph-like mix of ramps, edges and texture), written exactly as q/1024
in every sample type, byte order and row alignment by both drivers. Whole
caller histories, including every ProcessOutput status and output chunk with
schedules of 0, 1, 7 and 64 bytes and then doubling, match the original:
`public_encoder_lossy_vardct_small_sizes_match_original` (384 histories: 1x1,
7x7, 8x8, 9x17 and 41 to 48 x 15, gray and RGB, all four sample types and
contents), `..._large_sizes_match_original` (32: 63x129, 256x256, 257x255 and
512x1024) and `..._d05_and_settings_match_original` (48: d0.5 at intensity
targets 255 and 100 for every gate size, buffering -1 to 3 at 512x1024,
1024x1024 with buffering 0, explicit effort 7, codestream and custom bit
depths, orientations). Distances 0.001 to 0.69, sRGB and linear sRGB. All 464
are exact; none is an admission result. `..._decode_back_matches_original_decoder`
decodes 16 of these codestreams with the Rust and the original public decoder
(float and uint8 output, whole and 64-byte input chunks) through both C ABI
libraries: 32 histories match. `..._admissions_and_rejections_match_original`
covers d0.7 to d2.9, an EPF override, streaming sizes, settings outside the
profile, a custom memory manager, an unclosed frame and the step 1 rejection
histories (frame before basic info or after CloseInput, short buffer, invalid
distances, gray color encoding for RGB). epub-probe now reports the front
queued frame's parameters after the original's own ParamsPostInit; every
ArHeuristics and StreamingFrame result is checked against the source's
branch conditions on that state. The existing encoder families keep passing.

Contraction. `tools/encoder_frame_contraction.json` records the 13 front-end
fmuladd sites of enc_frame.cc; `tools/encoder_transforms_contraction.py --map
tools/encoder_frame_contraction.json` confirms them and the fused AArch64
lowering in the ordinary oracle graph. Of these, PixelStatsForChromacityAdjustment's
`cur_b - cur_y * 1.2` (624:33, double) is reached and translated with
`mul_add_f64`; ParamsPostInit's site is mapped already; the others are in
unreached alpha, noise, JPEG and center-first code. LossyFrameHeuristics has
no fmuladd site of its own.

The same 16 encoder_public tests also pass against the ASan/UBSan oracle
graph (macOS AArch64).

Extensions later on 2026-11-12. (1) Streaming (encoder step 2, started):
EncodeFrameStreaming for output modes 0 and 1 with
ComputePermutationForStreaming, TOCBucket, TOCSize, ComputeGroupDataOffset,
ComputeDcGlobalPadding, OutputGroups, RemoveUnusedHistograms and
OutputAcGlobal, and the streaming branches of ComputeEncodingData,
EncodeGroups and EncodeGlobalACInfo; frames with more than 8 groups in the
profile encode instead of returning StreamingFrame.
`public_encoder_lossy_vardct_streaming_matches_original` compares 16 histories
(768x768, 768x769, 1000x700, 2049x300, 1031x517, 1024x1024; buffering -1 to 3,
so output mode 1 with its seeks is covered). (2) The AR heuristics from the
parallel PR 5 work (`enc_heuristics_ar.rs`, cherry-picked) are called from
`compute_ar_heuristics_entry`; the ArHeuristics admission result is removed
and the profile extends to every distance below 3.0. The history families now
include d0.7, d1.0, d1.5, d2.0 and d2.9 (d1.0 at both intensity targets for
every gate size): smoke 9, small 384, settings 84, large 32, streaming 16 and
the admission family's 22 exact histories match byte for byte against the
ordinary oracle; decode-back 32. All 17 encoder_public tests, with these
families, also pass against the ASan/UBSan oracle graph, and the cherry-picked
`ear-chain` and `ecf-chain` tests pass against the rebuilt ordinary oracle.

Not verified: native Linux x86-64, Debug and MSRV test runs, the upstream
differential cases of step 1 and checkpoint fixtures of LossyFrameHeuristics
as one unit. No timing or memory measurement was taken.

## Encoder AR heuristics with the decoder in the loop (2026-11-12)

This completes the fifth of the six pull requests of encoder step 1 in
[ENCODER_EXPANSION.md](ENCODER_EXPANSION.md#first-step). The original runs
ComputeARHeuristics at every distance of at least 0.7 at kSquirrel (where
LoopFilterFromParams sets `epf_iters > 0`), including the default d1.0. The
units are standalone until the integration PR calls the entry point below.

- `enc_heuristics_ar.rs`: ReconstructImage, ComputeBlockL2Distance and
  ComputeARHeuristics from `enc_heuristics.cc`, serial (the pool argument is
  absent; RunOnPool without a runner calls the init with one thread and runs
  the groups in order on thread 0). ReconstructImage's `dec_state->shared =
  &shared` is a scoped move of the encoder's PassesSharedState into the
  decoder state's storage slot and back, because the decoder state's
  shared-pointer type ties the borrowed state's lifetime to its own (ADR
  0003); the reached decoder paths only read it. The source reads the
  decoder's own storage only for a DC-level writer (`dc_level != 0`), which
  a VarDCT frame never has; that case is an admission result.
- `dec_group.rs` (additive): GetBlockFromEncoder and DecodeGroupForRoundtrip,
  which feed the encoder's 32-bit quantized coefficients (all passes, with
  their shifts) through the existing DecodeGroupImpl.
- `dec_cache.rs` (additive): `PassesDecoderState::prepare_pipeline`, the
  source PreparePipeline for callers outside FrameDecoder (the frame decoder
  keeps its interleaved, traced preparation). All branches are the source's
  except two admission results: `coalescing` with NeedsBlending (both
  encoder callers pass `coalescing = false`) and a CMS stage with the simple
  pipeline (as in the frame decoder). The fast XYB-to-sRGB8 branch is
  compiled out at JXL_HIGH_PRECISION=1.

Entry point for the frame encoder (ComputeVarDCTEncodingData calls it after
InitializePassesEncoder and before ComputeACMetadata):

```rust
pub fn compute_ar_heuristics(
    frame_header: &FrameHeader<'_>,
    enc_state: &mut PassesEncoderState<'_>,
    orig_opsin: &Image3F,
    _rect: Rect,
) -> Status
```

It mirrors `ComputeARHeuristics(frame_header, enc_state, orig_opsin, rect,
pool)` without the pool. Preconditions: `enc_state.coeffs` holds the 32-bit
quantized coefficients of every pass; the shared state carries the frame's
metadata, dimensions, quantizer, matrices, strategy, raw quant field, CfL
map, DC, quant_dc and image features, and `epf_sharpness` is allocated at
the block size; `enc_state.initial_quant_masking1x1` is the InitialQuantField
mask; `orig_opsin` is the padded XYB image copied before Gaborish and shrunk
to `frame_dim.xsize` x `frame_dim.ysize`. `rect` is unused, as in the source.

Contraction: ComputeBlockL2Distance's `err2[i] += mask2 * diff * diff`
(f32) and its two double sums use `mul_add_f32`/`mul_add_f64`; the sites are
recorded in `tools/encoder_coefficients_contraction.json`.
ComputeARHeuristics' multiplier divides before adding and has none. The
decoder-in-the-loop objects are recorded in
`tools/encoder_ar_contraction.json`: dec_group.cc (the inlined CfL ratios
and the IDENTITY transform, existing translations; GetBlockFromEncoder has
no float expression), dec_cache.cc (37 sites from the color, CMS and
tone-mapping headers other functions of that object inline; PreparePipeline
has none) and epf.cc (none). The audit passes against both graphs on macOS
AArch64: 40 sites with 186 (ordinary) and 93 (ASan/UBSan) fused instructions
for the new map, 31 sites with 106 and 54 for the coefficient map. A
per-site sensitivity check: computing the f32 site with separate operations
makes the `ear-chain` corpus fail (first difference in an error image of a
7x7 request, one ULP); the two double sites with separate operations still
match the 160-request corpus, so it does not discriminate them.

Evidence: `tools/oracle/encoder_ar.cc` and `tests/parity/encoder_ar.rs`. The
`ear-chain` request repeats the `ecf-chain` sequence from sRGB samples (ToXYB
and padding, the pre-Gaborish copy that ComputeVarDCTEncodingData keeps, the
kSquirrel heuristics, InitializePassesEncoder), selects the loop filter with
the internal LoopFilterFromParams (through `enc_frame.inc`) or explicit
values, and compares as exact words and integers: the loop filter; for every
candidate that ComputeARHeuristics tries at the distance ({0, 2, 7}, or {0,
4} above 4.5), the archive's ReconstructImage output (all three planes) with
the whole sharpness plane set to the candidate and the per-block
ComputeBlockL2Distance image; the epf_sharpness plane of the archive's
ComputeARHeuristics; and the AC-metadata modular images ComputeACMetadata
builds from it. 560 requests in three groups, 538 of them with the AR search
(the others take the fill-4 branch):
- the corpus sizes up to 257x255 (16 sizes), all five contents, gray and RGB,
  at d1.0 (160);
- the same sizes and contents at d0.7, d1.5, d2.0 and d2.9 with three
  quantization-matrix scale pairs, the 512x1024 (8 groups) images at d1.0
  and d2.9, and the boundaries d0.5 and d0.69 (no EPF, fill 4), d4.0 (three
  EPF iterations), d4.5, d4.6 and d6.0 (two candidates, clamped distance)
  (352);
- unit isolation of the decoder in the loop with explicit loop filters:
  neither Gaborish nor EPF (DecodeGroupForRoundtrip and the write stage
  alone), Gaborish alone, and one to three EPF iterations with and without
  Gaborish, on six sizes (48).

All 560 match the ordinary oracle and the ASan/UBSan oracle on native macOS
AArch64. Not covered: noise (`kNoise` frames and PrepareNoiseInput), patches
and splines in the reconstruction (the step 1 profile has none; a found
patch is an admission result), chroma subsampling and upsampling stages,
Linux x86-64 and the hosted deep gate.

Remaining for step 1 after this PR: LossyFrameHeuristics itself,
ComputeChromacityAdjustments, the VarDCT branches of ComputeEncodingData and
EncodeGroups, EncodeFrameOneShot for VarDCT, the EncodeFrame setting
resolution and ProcessOneEnqueuedInput's raw-codestream branch (the
integration PR).

## Encoder coefficients, DC streams and patch detection (2026-11-12)

This is the fourth of the six pull requests of encoder step 1 in
[ENCODER_EXPANSION.md](ENCODER_EXPANSION.md#first-step), plus the patch
detection half of the fifth. The units are standalone; LossyFrameHeuristics,
ComputeARHeuristics and the frame encode are not translated, so they are not
connected to the public API.

- `enc_group.rs`: one-lane QuantizeBlockAC, AdjustQuantBlockAC,
  QuantizeRoundtripYBlockAC, ComputeCoefficients and
  EncodeGroupTokenizedCoefficients.
- `enc_entropy_coder.rs`: NumNonZeroExceptLLF (with the source's LLF mask
  array, of which only four entries are set), NumNonZero8x8ExceptDC and
  TokenizeCoefficients.
- `enc_coeff_order.rs`: ComputeUsedOrders, ComputeCoeffOrder (xorshift
  block sampling, zero counts, the libc++ sort over unique keys) and
  EncodeCoeffOrders.
- `enc_progressive_split.rs`: SplitACCoefficients<int32_t>.
- `enc_cache.rs`: InitializePassesEncoder without the progressive-DC frame
  (a kUseDcFrame header returns an error before any state change) and
  ComputeACMetadata, serial.
- `enc_modular.rs`: AddVarDCTDC with QuantizeWP and QuantizeGradient (all four
  branches) and AddACMetadata.
- `enc_chroma_from_luma.rs`: ColorCorrelationEncodeDC.
- `enc_quant_weights.rs`: DequantMatricesEncode for all-default matrices (the
  only matrices FindBestDequantMatrices produces in the step 1 profile);
  custom encodings return an error before writing.
- `enc_heuristics.rs`: FindBestBlockEntropyModel (its decoding-speed-tier
  branch writes a local copy in the source and is a no-op here too).
- `enc_patch_dictionary.rs`: FindTextLikePatches, serial and without the debug
  dumps. The step 1 encoder needs only whether it finds patches; a non-empty
  result enters the untranslated reference-frame encode (step 5).
- `enc_ans_params.rs`: HistogramParams(SpeedTier, num_ctx).

Contraction map (`tools/encoder_coefficients_contraction.json`, audited with
`tools/encoder_transforms_contraction.py --map`): 31 front-end fmuladd sites in
the nine archive objects. Translated sites: QuantizeBlockAC's `thresholds[i] -=
0.00744f * xsize * ysize`, AdjustQuantBlockAC's `thresholds[i] += 0.01 *
activity` (double) and its two `kMul1 * ... * kBlockDim + kMul2 *
sum_of_vals` sums (the fused factor is the exact multiplication by kBlockDim),
ComputeCoeffOrder's `num_zeros * inv_sqrt_sz + 0.1f`, the three AddVarDCTDC
residuals `row[x] - quant_row_y[x] * (y_factor * cfl_factor)`, and
FindTextLikePatches' `distance += |v1 - v2| * weight`; the inlined CfL ratios
and AFV DC mixes are the existing translations. The other sites belong to
untranslated code (general and lossy Modular, the 2x resampling helpers,
ComputeBlockL2Distance of the AR heuristics, the patch bin packing and a debug
random generator) and are recorded as such. The audit passes against the
ordinary oracle graph on macOS AArch64 (31 sites, 106 fused instructions) and
the ASan/UBSan graph (31 sites, 54 fused instructions). A per-site
sensitivity check was run only for QuantizeBlockAC's threshold site: with
separate operations the 402 `ecf-chain` requests still match, so the corpus
does not discriminate that site; the other translated sites were not checked.

Evidence: `tools/oracle/encoder_coefficients.cc` (compiled with
`-fno-access-control` to read the modular stream images) and
`tests/parity/encoder_coefficients.rs`.
- `ecf-chain` runs, from sRGB samples, ToXYB and padding, the kSquirrel
  part of LossyFrameHeuristics as `eqh-chain` does plus
  AcStrategyHeuristics::Finalize, the epf_sharpness fill and
  FindBestBlockEntropyModel, then the archive's InitializePassesEncoder and
  ComputeACMetadata, ComputeUsedOrders/ComputeCoeffOrder and
  TokenizeCoefficients in the order of the (internal-linkage)
  ComputeAllCoeffOrders and TokenizeAllCoefficients, the modular
  ComputeTree/ComputeTokens, and the serialized sections: quantizer
  parameters, block context map and ColorCorrelationEncodeDC; the modular
  global info and stream; each DC group (extra precision, VarDCT DC stream,
  AC-metadata size and stream); the AC global section (default matrices,
  histogram count, used orders, EncodeCoeffOrders, AC histograms); and each
  AC group (EncodeGroupTokenizedCoefficients). It compares quantizer state,
  DC quant weights, AcStrategyImage, raw quant field, CfL maps, qm
  multipliers, DC and quant_dc images, all coefficients, the modular DC and
  AC-metadata images, the coefficient orders, all AC tokens and every
  section's bit count and bytes. ComputeARHeuristics is not run on either
  side, so the AC-metadata EPF channel holds the value 4 that
  LossyFrameHeuristics stores; this equals the encoded output only where the
  original also skips AR (distance below 0.7, where epf_iters is 0).
  The x/b quantization-matrix scales are request parameters, because
  ComputeChromacityAdjustments has internal linkage and is not translated.
  Two groups, 402 requests: the corpus sizes up to 257x255 in gray and RGB
  with all five contents at d1 (160) and, at d0.5, d1.5 and d2.9 with three
  scale pairs plus the 512x1024 (8 groups) images at d1 (242).
- `ecf-patches` runs the original FindBestPatchDictionary at kSquirrel and
  reports whether its FindTextLikePatches found patches and how many
  dictionary positions they cover: 101 requests on the corpus and on
  repeated-glyph content at d0.5, d1 and d2.9, 12 of which find patches.

Local results on native macOS AArch64: `python3 tools/check.py --stage test
--filter encoder_` passes against the ordinary and the ASan/UBSan oracle (57
encoder tests, including the three groups above and the existing encoder
transform, heuristics and public API groups). Linux x86-64 and the hosted deep
gate are not run.

Remaining for step 1 (encoder step 1 is not complete and no profile is
admitted): LossyFrameHeuristics itself (with FindSplines, empty at the pinned
revision, and the empty-patch path of FindBestPatchDictionary),
ComputeARHeuristics with ReconstructImage and DecodeGroupForRoundtrip (the
decoder in the loop; reached at every distance of at least 0.7, where
epf_iters is nonzero), ComputeChromacityAdjustments, the VarDCT branch of
ComputeEncodingData and EncodeGroups, EncodeFrameOneShot for VarDCT, the
EncodeFrame setting resolution and ProcessOneEnqueuedInput's raw-codestream
branch, which removes the admission result for the admitted profile.

## Public encoder API admission and setters (2026-11-05)

This is the first of the six pull requests of encoder step 1 in
[ENCODER_EXPANSION.md](ENCODER_EXPANSION.md#first-step). ADR 0001 now admits
the public encoder with run-time admission results.

`src/jxl/encode.rs` holds the original JxlEncoder and JxlEncoderFrameSettings
objects from `encode_internal.h`, with every field in source order, and these
`encode.cc` functions: JxlEncoderVersion, Create, Reset, Destroy, GetError,
UseContainer, StoreJPEGMetadata, SetCodestreamLevel,
GetRequiredCodestreamLevel with VerifyLevelSettings, SetCms,
AllowExpertOptions, InitBasicInfo, SetBasicInfo with CheckValidBitdepth,
InitExtraChannelInfo, SetExtraChannelInfo, SetExtraChannelName,
SetUpsamplingMode, SetColorEncoding, SetICCProfile (with the default CMS and
DecideIfWantICC), JxlColorEncodingSetToSRGB/LinearSRGB, FrameSettingsCreate,
SetFrameLossless, SetFrameDistance, SetExtraChannelDistance,
DistanceFromQuality, FrameSettingsSetOption, FrameSettingsSetFloatOption,
InitFrameHeader, InitBlendInfo, SetFrameHeader, SetExtraChannelBlendInfo,
SetFrameName, SetFrameBitDepth, AddImageFrame with GetCurrentDimensions,
AddImageFrameInternal, CanDoFastLossless, VerifyInputBitDepth and QueueFrame,
UseBoxes, AddBox with QueueBox, SetExtraChannelBuffer, CloseFrames,
CloseBoxes, CloseInput and ProcessOutput. `encode_internal.rs` adds MakeBoxType,
the frame-index-box storage, JxlEncoderFrameSettingsValues, the queued
frame/box/input owners and the buffer mode of JxlEncoderOutputProcessorWrapper
(GetBuffer, Seek, SetFinalizedPosition, SetAvailOut, ReleaseBuffer,
FlushOutput, JxlOutputProcessorBuffer and AppendData). The encoder object,
frame settings, queued frames and queued boxes allocate through the caller's
memory manager as MemoryManagerUniquePtr owners, released in the source's
order. SetIntensityTarget from `luminance.cc` and DecideIfWantICC are new.
DistanceFromQuality reproduces the original object's AArch64 FMADD
contractions and the separate x86-64 operations.

Admission (ADR 0001): two source checkpoints return `JXL_ENC_ERROR` with
`JXL_ENC_ERR_NOT_SUPPORTED` after every earlier original check. AddImageFrame
does so where CanDoFastLossless selects `enc_fast_lossless.cc` (step 8), and
ProcessOutput where its loop would call ProcessOneEnqueuedInput for a queued
frame or box (header, container, box and frame encoding). So every encode
request is currently an admission result; queueing, all rejections before
these calls and `NEED_MORE_OUTPUT` with zero output space compare exactly.
AddJPEGFrame, AddChunkedFrame, FlushInput, SetOutputProcessor,
SetParallelRunner, SetDebugImageCallback and the statistics functions are
absent; their owners are uninhabited fields.

Evidence: `tools/oracle/encoder_public.cc` replays `epub-run` caller histories
on the original public API of the complete archive and prints every status,
GetError value and output chunk; `tests/support/encoder_public.rs` mirrors it
token for token. An `S` checkpoint, compiled with `-fno-access-control` like
the other internal fixtures, compares the stored state exactly: all encoder
flags and counters, the metadata serialized by the original
WriteCodestreamHeaders on a copy, the ICC bytes and WantICC, the stored basic
info, every frame-settings value, and each queued frame's options,
`ec_initialized` and copied channel storage or queued box. With a custom
manager, the allocation and release order compares too, including each
injected allocation failure. `epub-probe` reports the original's internal
state at each AddImageFrame and ProcessOutput: every admission result must
coincide with the original queueing a fast-lossless frame or entering
ProcessOneEnqueuedInput, and no compared AddImageFrame may take the
fast-lossless branch. `epub-wrap` drives the original buffer-mode wrapper.
Local results on native macOS AArch64, with the ordinary and the ASan/UBSan
oracle: 3,305 exact public histories in nine families (lifecycle 44,
basic-info 449, extra-channels 136, color 336, options 1,895, distance 171,
frame-settings 66, frames 173, boxes-output 35) and 606 exact wrapper
histories. Another 153 histories match
up to their admission call with original branch evidence (12 fast-lossless,
141 ProcessOneEnqueuedInput); they are not counted as parity. The source
audit for this unit has not been written yet.

Not applicable or deferred: allocation sizes are those of the language
layouts and are not compared. The original JxlEncoder has a user-provided
constructor, so its stored basic info is default-initialized and
JxlEncoderInitBasicInfo leaves `padding` unset; those bytes are indeterminate
until SetBasicInfo copies a caller's struct (on Linux they held earlier heap
contents). Both checkpoints print the padding only while basic_info_set holds;
every other basic-info field compares in every state. Enum arguments stay
within their enumeration's value range (for example extra-channel type 31,
white point and primaries 15, blend mode 7, setting id 65534); values beyond
it are undefined in the original and abort the instrumented graph. Float and
float16 caller samples use bytes 0x30-0x3f, which form finite values below 2
at any offset and byte order; arbitrary bytes made the original's
float-to-integer conversion of extra channels undefined. A failed extra-channel SetFromBuffer leaves the
original's channel pointing to the caller's buffer, which the Rust owner does
not retain; the `S` checkpoint compares the other channel fields. A
SetExtraChannelBuffer after AddBox dereferences a null queued frame in the
original; the Rust function panics there and no history reaches it.

## Encoder quantization, CfL and AC strategy heuristics (2026-11-05)

This is the third of the six pull requests of encoder step 1 in
[ENCODER_EXPANSION.md](ENCODER_EXPANSION.md#first-step). It translates the
heuristics that LossyFrameHeuristics runs at kSquirrel between XYB conversion
and coefficient quantization, as standalone units; LossyFrameHeuristics itself
and the frame encode remain for the integration PR.

- `quantizer.rs`: the field selection of `quantizer.cc`,
  ComputeGlobalScaleAndQuant, SetQuantFieldRect, SetQuantField and SetQuant.
  SetQuantField's two std::nth_element calls use
  `base/libcpp_nth_element.rs`, a translation of the reference libc++
  algorithm for float ranges (branchless `__sort3`, `__selection_sort`,
  `__min_element`), with its headers pinned in `tools/reference/libcxx`.
  DumpQuantizationMap remains unported.
- `enc_adaptive_quantization.rs`: InitialQuantDC, InitialQuantField with the
  one-lane AdaptiveQuantizationMap (PrepareBuffers, ComputeTile with its
  1x1 Laplacian, the scalar and one-lane difference loops and the 4x4
  pre-erosion, FuzzyErosion with StoreMin4, ComputeMaskForAcStrategyUse,
  PerBlockModulations with ComputeMask, GammaModulation, HfModulation and
  BlueModulation, MaskingSqrt, RatioOfDerivativesOfCubicRootToSimpleGamma)
  and Blur1x1Masking, and AdjustQuantField. FindBestQuantizer does nothing at
  kSquirrel and faster; its butteraugli loops (FindBestQuantization,
  FindBestQuantizationMaxError, RoundtripImage, TileDistMap) and the debug
  heat maps remain unported.
- `enc_chroma_from_luma.rs`: CfLHeuristics (Init with InitDCStorage,
  PrepareForThreads, ComputeTile for the DCT8 and the strategy pass) with the
  one-lane CFLFunction and FindBestMultiplier (Newton and fast searches).
  ColorCorrelationEncodeDC remains unported.
- `enc_ac_strategy.rs`: AcStrategyHeuristics (Init, PrepareForThreads,
  ProcessRect, Finalize) with the one-lane ProcessRectACS,
  MultiBlockTransformCrossesHorizontal/VerticalBoundary, EstimateEntropy,
  FindBest8x8Transform, TryMergeAcs, SetEntropyForTransform, the
  AcsSquare/VerticalSplit/HorizontalSplit helpers and
  FindBestFirstLevelDivisionForSquare, including the merge table's three
  value-initialized entries. The disabled JXL_DEBUG_AC_STRATEGY dump
  (TypeColor, TypeMask, DumpAcStrategy) is not ported. ACSConfig's row
  pointers are borrows that ProcessRect receives (ADR 0003).
- `base/hwy_scalar.rs` adds Highway's scalar Round.

Contraction map (`tools/encoder_heuristics_contraction.json`): Clang's front
end records 28 `llvm.fmuladd` sites in the four archive objects (quantizer.cc,
enc_adaptive_quantization.cc, enc_chroma_from_luma.cc, enc_ac_strategy.cc),
counting the two inlined AFV sites in each of the two objects that contain
them. The translated ones are the three `x * inv_global_scale_ + 0.5f`
roundings of the quantizer; PerBlockModulations' final `FastPow2f(...) * mul +
add`, FuzzyErosion's weight `kMulBase + mul * kMulAdd` and its four-product
sum (the first product fused, then each later product), and AdjustQuantField's
mixer and mix; the fast CfL denominator `ca + num * distance_mul * 0.5f`;
EstimateEntropy's norm sum and its three `entropy +=` products, and
FindBest8x8Transform's two multiplier adjustments; plus the already ported
inlined YtoXRatio/YtoBRatio, AFV DC mix and AFV inverse residual. Each is
fused on AArch64 and separate on x86-64 (`mul_add_f32` or the existing
per-target forms). Two sites have exact
products and fold away in both graphs (Blur1x1Masking's constant filter sum,
TileDistMap's `1 * v`); FindBestQuantization's two double sites and
TileDistMap are unported. Highway's scalar operations, separate statements and
front-end-folded constant products (RatioOfDerivatives' offsets,
BlueModulation's limits) are never fused. `std::pow(float, float)` and
`std::log1p(float)` call the host `powf` and `log1pf`; `pow((5 - d) / 5,
2.0f)` is the product `x * x` in the original object. In optimized IR the
optimizer speculates FindBest8x8Transform's two adjustments into the branch
condition and gives them different debug locations in the ordinary and ASan
objects (the ASan object merges both), so the audit now reads front-end IR
(`-Xclang -disable-llvm-passes`); this also records the folded GaborishInverse
site of the transform map.

Evidence: `tools/oracle/encoder_heuristics.cc` calls the archive's own
functions and `tests/parity/encoder_heuristics.rs` compares exact words:
quantizer state (global scale, DC quant, scale, inverse scales, DC
multipliers), raw quant fields over canary-filled owners, the quant field and
both masking fields with their dimensions, CfL maps and all four DC value rows,
AcStrategyImage layers (strategy and first-block bit per block), the Finalize
block counts, the ACS configuration, and EstimateEntropy's entropy with its
coefficient, pixel and residual scratch words. The step 1 corpus is the sizes
1x1, 7x7, 8x8, 9x17, 41 to 48 x 15 (every `xsize mod 8`), 63x129, 128x72,
256x256, 257x255 and 512x1024, in gray and RGB, with flat, gradient, noise,
synthetic photographic and GetSomeTestImage content (from the extracted
original helper), converted with ToXYB and padded as enc_frame.cc does; the
512x1024 image runs with photographic and test content at d1. Thirteen groups
with 11,512 requests: ComputeGlobalScaleAndQuant and SetQuant over clamp,
cap and rounding boundaries and random inputs (4,432); SetQuantField and
SetQuantFieldRect over ten sizes, ties, signed zeros, sorted runs, ClampVal
limits and every selected position among up to nine elements (1,180);
InitialQuantDC (624); InitialQuantField on the corpus at d0.5, d0.62, d1, d1.5
and d2.9, distances across the dampening ramp and FuzzyErosion branch, and
the rescale argument (821); InitialQuantField on inner and offset rects that
take the +-2 and +-4 pixel extensions (15); AdjustQuantField over random
valid layouts, rects and the mixer's distance range (576); FindBestMultiplier,
both searches (336); CfLHeuristics' DCT8 and strategy passes on the corpus,
fast and slow (656); EstimateEntropy for every strategy up to 64x64 and
FindBest8x8Transform at five speed tiers (1,080); AcStrategyHeuristics on the
corpus at d0.5, d1, d1.5 and d2.9 (644) and across speed tiers 0 to 6 and
decoding speed tiers 0, 1, 2 and 4 (144); FindBest8x8Transform at distances
where its favored-transform adjustment rounds differently fused (360); and the
kSquirrel chain on the corpus (644). Compared inputs are finite, the
original's defined domain.

Contraction sensitivity: replacing each translated site in turn by separate
operations makes the focused group fail on AArch64 for 12 of the 17 Rust
sites. The other five cannot change any output: the three quantizer roundings
are truncated to integers, which absorb the saved rounding (an exact search of
+-40 ulps around every `k + 0.5` boundary for 14 global scales found no
difference), and the CfL denominator and FindBest8x8Transform's
`kAvoidEntropyOfTransforms * mul` multiply by 0.5 exactly (the CfL half product
only rounds when subnormal).

Local results on native macOS AArch64: the complete `python3 tools/check.py`
passes against the ordinary oracle (1,002 tests, none failed or ignored),
and `python3 tools/check.py --sanitizers --stage test --filter encoder_` runs
the thirteen groups, with the encoder transform and public API groups,
against the ASan/UBSan oracle; both audits of the contraction maps pass in
both graphs (heuristics: 28 sites, 32 fused instructions in the ordinary and 43
in the ASan objects). Linux x86-64 and the hosted deep gate are recorded with
the pull request.

## Encoder transforms, Gaborish and XYB (2026-11-05)

This is the second of the six pull requests of encoder step 1 in
[ENCODER_EXPANSION.md](ENCODER_EXPANSION.md#first-step). It translates the
image-level units that the lossy VarDCT encoder applies before its
heuristics, as standalone functions; none is connected to a frame encode yet.

- `convolve.rs`, `convolve_symmetric5.rs`, `convolve_slow.rs`: the
  WeightsSymmetric3/WeightsSymmetric5 owners of `convolve.h`, the one-lane
  Symmetric5 of `convolve_symmetric5.cc` (border and interior paths, mirrored
  and unchanged rows), and SlowSymmetric3 of `convolve_slow.cc`, which the
  original Gaborish test uses as its forward kernel. `image_ops.rs` adds the
  WrapMirror/WrapUnchanged modes.
- `enc_gaborish.rs`: GaborishInverse with its double-precision normalization,
  temporary plane, three Symmetric5 passes and plane swaps.
- `enc_transforms.rs`: TransformFromPixels for all 27 strategies with
  DCT2TopBlock and AFVTransformFromPixels, DCFromLowestFrequencies with
  ReinterpretingIDCT, and AFVDCT4x4. The scaled DCT/IDCT recursion is the
  existing `dct.rs` translation of dct-inl.h, reused unchanged.
- `enc_xyb.rs`: ComputePremulAbsorb, LinearRGBRowToXYB and ToXYB with
  OpsinAbsorbance, CubeRootAndAdd (added to `base/fast_math.rs`), StoreXYB and
  the LinearSRGBToXYB, SRGBToXYB and SRGBToXYBAndLinear row loops. The admitted
  input encodings are a type: `XybInputEncoding::new` selects linear sRGB
  (SameColorEncoding with LinearSRGB of the same grayness) or sRGB (IsSRGB) in
  the source's order and has no value for any other encoding, so the
  ApplyColorTransform branch and its CMS argument are absent. ScaleXYB,
  ScaleXYBRow and RgbToYcbcr remain unported.

Contraction map (`tools/encoder_transforms_contraction.json`, audited by
`tools/encoder_transforms_contraction.py`): Clang's front end records four
`llvm.fmuladd` sites in the five archive objects. Two are in
AFVTransformFromPixels (`block00 + block01 +/- 2 * block10`, lines 452 and 454
of enc_transforms-inl.h), one in SlowSymmetric3Pixel (`row_in[ix] * wc + ... *
wlr`, convolve_slow.cc:41) and one in the unported SlowSeparablePixel. Native
AArch64 lowers each to FMADD (the subtraction as FMADD with -2); baseline
x86-64 keeps separate operations. The Rust port uses `mul_add` at the three
translated sites on AArch64 only. enc_gaborish, convolve_symmetric5 and
enc_xyb contain no lowered site: Highway's scalar operations and separate
statements are never fused, and GaborishInverse converts its binary32 product
to double before the addition. Its constant kernel sum has one front-end site,
`kGaborish[4] + 2 * kGaborish[3]`, which the optimizer folds; the product is
exact, so fused and separate folding agree (recorded as `folded`). The audit
reran every archive compile command with front-end IR and line tables (since
PR 3; see "Encoder quantization, CfL and AC strategy heuristics") and found
exactly these sites in both local graphs; the ordinary
objects hold 39 fused AArch64 instructions and the ASan objects 25 (the counts
follow unrolling, not the sites). An x86-64 compile of the same commands on
the macOS host emitted no fused instruction.

Evidence: `tools/oracle/encoder_transforms.cc` calls the archive's own
functions; `tests/parity/encoder_transforms.rs` compares every request as
exact float words, including the unchanged input, all scratch words and tail
canaries, and the status codes of the original ensure checks. Eleven groups
with 5,346 requests: TransformFromPixels for all 27 strategies with 20 value
modes (zero, signed zero, impulses, ramps, noise, overflow, every exceptional
word: subnormals, infinities, quiet and signaling NaNs of both signs) and three
strides (432); DCFromLowestFrequencies for all 27 strategies (1,080);
AFVDCT4x4 (272); competing NaN payloads (564); Symmetric5 over 22 sizes that
cover every `xsize mod 8`, full, inner and offset rectangles and the rectangle
checks (432); SlowSymmetric3 (941); GaborishInverse over the same sizes with
the default and edge multipliers, including the `sum < 1e-5` clamp and NaN
and infinite multipliers (374); all of `enc_gaborish_test.cc` (TestZero,
TestFlat and the upstream-disabled TestDirac input, 3); ComputePremulAbsorb
for 14 intensity targets; LinearRGBRowToXYB for widths 1 to 17 and edge inputs
(490); and ToXYB for gray and color sRGB and linear sRGB at two intensity
targets, with and without linear output, the black and linear size checks,
and eight encodings that would take ApplyColorTransform, whose branch
predicates match while the Rust admission rejects them (744). About 16.5
million words are compared, most of them scratch and canary words.
`tests/enc_gaborish.rs` also runs TestZero and TestFlat with the original
VerifyRelativeError tolerances in ordinary builds.

Competing NaN payloads are register-allocation choices that differ between
the ordinary and ASan archive objects: in the 8-point and larger DCTs of
TransformFromPixels on AArch64, and in the AFVDCT4x4 basis sum on Linux
x86-64 (where the first hosted run failed 18 ordinary and 42 instrumented
requests). Those 432 TransformFromPixels and 24 AFVDCT4x4 requests run
against the observation unit `encoder_transforms_unit.cc`, the pristine
template with scoped -fno-unroll-loops, on which both graphs agree. Its
AFVDCT4x4 loop adds each product into the accumulator, so the accumulator's
NaN wins on both targets (x86-64 ADDSS returns its first source); the Rust
`afv_accumulate` keeps that operand order out of line. The 108
DCFromLowestFrequencies competing requests run on the archive (ADR 0002).

Local results on native macOS AArch64: the eleven groups pass against the
ordinary and the ASan/UBSan oracle. Linux x86-64 and the hosted deep gate are
recorded with the pull request.

## Thread runners and JxlDecoderSetParallelRunner (2026-11-04)

`src/threads/` translates `lib/threads`: ThreadParallelRunner from
`thread_parallel_runner_internal.{h,cc}` with its worker threads, the
command/condition-variable protocol, guided work distribution, re-entry
detection and RunOnEachThread; the JxlThreadParallelRunner functions of
`thread_parallel_runner.cc` (the runner callback, Create with its memory
manager and placement construction, Destroy and DefaultNumWorkerThreads); and
ResizeableParallelRunner with the JxlResizableParallelRunner functions of
`resizable_parallel_runner.cc` (the runner callback, Create, SetThreads,
Destroy and SuggestThreads). They use `std::thread` and `std::sync`; no C or
C++ code is linked. `src/threads/abi.rs` exports the nine functions of
`thread_parallel_runner.h` and `resizable_parallel_runner.h` under their
original names, from the same static and dynamic library as the decoder C ABI
(the original builds them as the separate `jxl_threads` library). ADR 0003
records the representation.

JxlDecoderSetParallelRunner in `decode.rs` replaces the decoder's ThreadPool
before decoding starts, as decode.cc does. The frame decoder retains the pool
and runs every RunOnPool site that the original runs through it: the DC
groups, the AC groups and the forced draw of Flush with PrepareStorage as the
init function, AdaptiveDCSmoothing's two row ranges, and FinalizeDecoding with
its inverse RCT, squeeze and palette transforms and its group conversion with
PrepareForThreads as the init function. FinalizeDecoding keeps the original
small-image null pool. The runner chooses the init thread count, the calling
threads, the task order and the tasks that the error flag skips; per-thread
storage follows the thread ids, and errors collapse to the original's runner
failure. The task bodies share non-thread-safe owners, so one lock per
RunOnPool call serializes them (`run_on_pool_exclusive`). The encoder's
RunOnPool sites keep a null pool.

Evidence: `tests/thread_runners.rs` ports `thread_parallel_runner_test.cc`
(TestPool with 0 to 18 threads, TestSmallAssignments, TestCounter) through
ThreadPool, runs the same properties on the resizable runner, and checks init
thread counts, thread-id bounds, range checks, error propagation, re-entry,
RunOnEachThread, the memory manager, resizing and SuggestThreads. The `runners`
family of `tests/parity/decoder_public.rs` decodes 14 genuine original
encodings that reach every site above with more than one task. It uses the
thread runner (0, 1, 2 and 5 workers), the resizable runner (SetThreads 0, 1, 3
and 8), the DecodeWithAPI runners and the null runner: 332 histories compare
exactly with the original under the same runner. The original's history with a
runner must equal its runner-free history wherever outputs go through caller
buffers. The multithreaded callback writes into a caller buffer too (output
mode 5); its init thread counts compare for explicit thread counts, and every
run call's thread id must be below the latest init's count. The decode_test.cc
runner histories now also run on Rust with the translated runner (see below),
and two retained captures replay runner histories without C++. Every runner
history also runs through both C ABI drivers (see "Exported decoder C ABI"),
the Rust one calling the exported JxlDecoderSetParallelRunner and runners.

Not established: concurrent execution of the decoder's task bodies and its
performance, JxlEncoderSetParallelRunner and encoder-side RunOnPool. Default
thread counts come from `available_parallelism`, which can
differ from libc++'s `hardware_concurrency` on restricted hosts; no decoded
output depends on them. The runner tests check properties, not schedules, and
no thread sanitizer runs on Rust.

## Exported decoder C ABI (2026-11-04)

`src/jxl/decode_abi.rs` exports all 52 functions that the pinned
`lib/include/jxl/decode.h` declares, including JxlDecoderSetParallelRunner,
and `src/threads/abi.rs` the nine functions of `thread_parallel_runner.h` and
`resizable_parallel_runner.h`, under their original unmangled names, from
the crate's static library (`libjxl.a`) and dynamic library (`libjxl.so` or
`libjxl.dylib`). Each wrapper only maps C pointers to the references and
options of the Rust functions in `decode.rs` and `src/threads`, which keep every original
argument check in source order. Pointers that the source dereferences without
a check now reach those functions as options, unwrapped where the source
dereferences them; the Rust signatures changed accordingly. Structs crossing
the ABI are the existing `#[repr(C)]` header types, with compile-time size,
alignment and offset assertions on 64-bit targets. ADR 0003 records the
representation, the null-pointer and panic boundary, and the per-decoder
reentrancy guard.

The original builds the runners as a separate `jxl_threads` library; a C
program links the Rust library in place of both. The exported runner callbacks
keep the JxlParallelRunner signature, whose init and data functions are
non-nullable Rust function pointers; the decoder always passes both. Open: the
encoder C ABI of `encode.h` is out of scope until the encoder API is
ported. `JxlGetDefaultCms` belongs to the separate `jxl_cms` library: a C
program links the original one alongside the Rust decoder.

Evidence: the oracle-tier driver `tools/oracle/decoder_abi.cc` includes only
the pinned public headers and shares the dpub-run driver of the original
oracle (`decoder_public_driver.h`). It is linked twice, as `jxl_abi_original`
against the original archive and as `jxl_abi_rust` against the Rust static
library, both with the original default CMS; the original variant also links
the original `jxl_threads` library. Every history of the public decoder
families and of the `decode_test.cc` inventory, with or without a parallel
runner, also runs through both executables: 53,050 histories in 57 families
(44,077 public decoder histories, 332 of them in the `runners` family, and
8,973 inventory histories, 2,669 of them with a runner) reproduce the original
oracle's statuses, events,
getters, consumed and released byte counts, box and JPEG bytes, output
digests, callback sequences and, where compared, allocator logs exactly. The
C unit `decoder_abi_layout.c` measures 216 values from the headers: the size
and alignment of 31 public structs, enums and typedefs, every field offset of
the public structs, every enumerator, the version, and the presence of all 52
decoder and 9 runner functions. They equal the Rust definitions for both
libraries. The 839 histories that reach the original's undefined
null raw-box copy remain original-only. Local evidence covers macOS AArch64;
the public decoder audit additionally replays its corpus through the C ABI
drivers of both original graphs.

## Public incremental decoder API (2026-11-04)

`src/jxl/decode.rs` holds the complete original JxlDecoder object and public
functions from decode.cc, with every field in source order and the existing
DecoderInput owner at the codestream_copy position. Create/Destroy allocate
and release the object through the caller's memory manager. Reset, Rewind,
RewindDecodingState, the option setters, SubscribeEvents, SizeHintBasicInfo,
ReadBundle, ReadBasicInfo, ReadAllHeaders, ProcessInput, HandleBoxes and
ProcessCodestream with its complete frame loop, ProcessSections, the basic-info,
extra-channel, color and frame-header getters, the box API, the frame-index box
storage, SkipFrames with GetFrameDependencies, SkipCurrentFrame, the frame
reference updates, FlushImage, the preview/image/extra-channel buffer and
callback setters, SetProgressiveDetail, SetImageOutBitDepth, the color output setters
(SetCms, SetPreferredColorProfile, SetOutputColorProfile and
SetDesiredIntensityTarget), and SetJPEGBuffer/ReleaseJPEGBuffer with the jbrd,
Exif/XMP and JPEG output stages are translated. The frame loop uses the merged
incremental FrameDecoder with the low-memory pipeline, as decode.cc does; the
CMS stage is admitted on that pipeline. The ThreadPool from data_parallel.h
and the public structures from codestream_header.h are new modules. ADR 0003 records the object representation.

The original public C API of the complete archive is the reference. The
oracle's `dpub-encode` produces genuine inputs with the original public
encoder: raw codestreams and containers with Exif, XMP and jumb boxes, brob
boxes, jxll level boxes, jxlp splits, out-of-order jxlp boxes with ftyp
version 1, frame index boxes, embedded ICC profiles, extra channels with
names, orientation, intrinsic size, animation, cropped and blended layers,
progressive DC/AC, resampling, noise, patches and EPF/Gaborish frames. The
retained original encodings in `tests/data`, including previews, references,
crops and every extra-channel type, are decoded too, as are original JPEG
transcodes. `dpub-run` replays the same caller history on both decoders:
byte-wise and random chunking, CloseInput, small box and JPEG buffers,
truncated and mutated inputs, Rewind and Reset mid-stream, API misuse, every
event subscription, image-out buffers, simple and multithreaded callbacks,
extra-channel buffers, output bit depths, coalescing off, kept orientation,
unpremultiplied alpha, SkipFrames, SkipCurrentFrame, progressive detail with
FlushImage, JPEG reconstruction to bytes and CMS output profiles. Statuses,
events, getter outputs, consumed and released byte counts, box contents, JPEG
bytes, output sample digests and callback sequences compare exactly, and with
a custom manager so do the complete allocator log and injected allocation
failures (see "Decoder memory-manager propagation"). The twelve original
differential groups compare 11,822 histories locally, including the 332
runner histories above; fourteen standalone original captures run without
C++. `tools/decoder_public_reference.py` audits both original graphs and three
Rust builds in its own nightly audit group, including the `custom-manager`
replay.

Another 839 histories (830 truncated or mutated inputs and 9 JPEG
reconstruction histories) release their input and then continue an
uncompressed box being written or stored. The original then calls
`memcpy(*next_out, nullptr, 0)` in JxlBoxContentDecoder::Process, which is
undefined. They run as separate original-only probes: instrumented Linux
graphs must report exactly the UBSan nonnull-argument diagnostic at
box_content_decoder.cc:110, and the other graphs, including macOS, whose libc
declares no nonnull memcpy arguments, must complete. They have no C++/Rust
comparison; the Rust copy is defined for zero bytes and unchanged (ADR 0003).

Admission: the sequence-profile checks that FrameDecoder retains cannot reject
a successfully parsed codestream. Parsed metadata always has 1 to 32 bits per
sample and one ExtraChannelInfo per channel; parsed frame headers always have
a valid encoding and color transform, 4:4:4 sampling unless YCbCr, and one
upsampling and blending entry per extra channel. No compared file is rejected
by them. The C ABI exports and the runners are recorded above.

## Decoder memory-manager propagation (2026-11-04)

The caller's JxlMemoryManager now reaches every owner that the original
routes it to: the ICC reader and its decoded profile (icc_codec.cc and
padded_bytes.h), ANS alias tables and LZ77 windows (dec_ans.cc, through the
context-map, TOC, coefficient-order, tree, patch, spline and modular stream
decoders), the codec state (passes_state.cc and dec_cache.h: AC strategy,
quantization and EPF fields, color correlation maps, DC images, quantization
tables, sigma, group caches and coefficients), the image bundle, the modular
frame decoder with its images, channels and inverse transforms (palette,
squeeze), adaptive DC smoothing, spline draw caches, patch and frame
blending, both render pipelines and the upsampling, write and blending
stages. dec_external_image.cc and the encoder are not reached by the decoder
and keep the native allocator. ADR 0003 records the representation and the
source release order.

With the logging custom manager (`dpub-run 1`), every allocation request size
and the complete allocation and free order compare. The `custom-manager`
group replays every other family's 9,462 compared histories with it; their
839 null raw-box copy histories again run original-only. The
`allocations` group now includes the ICC inputs and the color-encoding event
(2,028 histories, failing each of the first eleven calls). Three groups fail
every allocation call in turn, from the decoder object to the last frame
owner: whole frames of every frame input and retained encoding (14,597
histories); progressive flushing, frame skipping, Rewind and Reset (5,934);
and headers with ICC profiles, JPEG reconstruction and CMS output (2,262).
Statuses, outputs and the remaining log compare exactly. Fifteen groups now
compare 43,745 histories locally, each also through both C ABI libraries,
in about 190 seconds. These groups run without a parallel runner: under a
runner, the order of allocations made inside RunOnPool tasks follows the
runner's schedule, in the original as well.

Five progressive histories fail a render-pipeline group-buffer allocation
during FlushImage and then continue decoding. The original keeps the empty
buffer and later writes through its null row pointer, which is undefined; the
Rust port panics at that use. These run as original-only probes that must
terminate abnormally (SIGSEGV on macOS). An exploratory sweep of every
allocation index of all 10,662 earlier histories (166,268 runs) found no
other difference; its 39 remaining cases were this null-buffer kind.

## Public decoder API test inventory (2026-11-04)

Every test in the pinned `lib/jxl/decode_test.cc` runs as a differential
case: 64 test definitions, five of them parameterized into 2,836 instances.
A 2026-11-12 recount of the pinned file's `TEST`, `TEST_P`,
`JXL_*_TEST` and `JXL_TRANSCODE_JPEG_TEST` definitions found the same 64, each
in the table below and in a family of
`tests/parity/decode_test_inventory.rs` or `tests/box_content_decoder.rs`.
The independent external conformance corpus is recorded separately in
[CONFORMANCE.md](CONFORMANCE.md).
`tests/parity/decode_test_inventory.rs` replays each test's call history on the
original public API and on the Rust port and compares statuses, events, getter
outputs, consumed and released byte counts, box and JPEG bytes, and digests of
every output buffer exactly. For callback output, each call's position, length
and pixels also compare in order. The two JxlBoxContentDecoder tests run as
captures of the separate box-content oracle in `tests/box_content_decoder.rs`.

The inputs are the tests' own. The oracle's `dti-*` commands build them with
CreateTestJXLCodestream, GetTestHeader, AnalyzeCodestream, GetSomeTestImage,
test::EncodeFile, AllEncodings and JpegDataToCodecInOut, extracted verbatim
from `decode_test.cc`, `test_utils.cc` and `test_image.cc` and hash checked.
The frame-sequence tests copy their bodies up to the EncodeFile call. Five files
from the pinned testdata submodule (an 80 MiB pack, 110 MiB checked out) are
copied unchanged into `tests/data/decode-test/` (543 KiB), with their CC BY 4.0
and CC0 notices; the submodule is not needed to run the tests.

The table marks runner histories and the original-only parts. (a) The test
sets a thread or resizable runner. Its histories run on the original and on
Rust with the same runner and compare exactly; the original's runner history
must also equal its runner-free history, which Rust runs as well. Outputs
under a runner compare through caller buffers. Not applicable: (b) Butteraugli,
DistanceRMS and ComparePixels against the encoder input, and the CodecInOut
conversions that prepare those comparisons, measure the encoder. The exact
comparison with the original decoder's output replaces them. (c) The extras
libjpeg encoder is not part of the oracle. CreateTestJXLCodestream's JPEG
inputs therefore start from the genuine cjpeg file
`tests/data/jpeg-writer/progressive.jpg` instead of a libjpeg encoding of the
test image; the pinned code after the encoder call is unchanged. (d) Encoder
expectations in the test bodies, such as frame counts of AnalyzeCodestream,
are not repeated. The extracted helpers' own expectations throw in the oracle.

All 6,304 runner-free Rust histories match the original locally on macOS
AArch64, in 31 families and 29 tests. The ten runner families add 2,669
histories with the translated runners, 1,898 of which set a runner. Most
families also require that the original's statuses follow the test's own
expectations, so each history takes the path of the test it ports. No history
reaches the original's undefined raw-box copy with a null input, which would
run original-only as in the public decoder families. No deviation in the Rust
port was found.

| decode_test.cc test | Status | API sequence and checks |
| --- | --- | --- |
| JxlSignatureCheckTest | ported | JxlSignatureCheck on the ten vectors |
| DefaultAllocTest | ported | Create and Destroy with the default manager |
| CustomAllocTest | ported | Custom manager; the complete allocation and free log compares |
| DefaultParallelRunnerTest | ported; (a) | SetParallelRunner(nullptr, nullptr); equals the runner-free history |
| BasicInfoTest | ported | Every prefix of two GetTestHeader headers: GetBasicInfo, extra-channel info and name, late SubscribeEvents, Reset |
| BufferSizeTest | ported | ImageOutBufferSize after the header |
| BasicInfoSizeHintTest | ported | SizeHintBasicInfo before and after an inserted box |
| IccProfileTestOriginal | ported | Encoded-profile and ICC getters for both targets, null outputs |
| IccProfileTestXybEncoded | ported | The same, then SetPreferredColorProfile to sRGB and linear |
| ICCPartialTest | ported | One more byte per call through basic info and the ICC profile |
| DecodeTestParam/PixelTest (1,532) | ported; (a), (b) | DecodeWithAPI over formats, callbacks, upsampling, boxes, previews, intrinsic sizes, early buffers and orientations; thread and resizable runners on both decoders |
| PixelTestWithICCProfileLossless | ported; (a), (b) | Six DecodeWithAPI calls on one decoder with Reset between |
| PixelTestWithICCProfileLossy | ported; (a), (b) | DecodeWithAPI with the data ICC profile |
| DecodeAllEncodingsTest/PreserveOriginalProfileTest (216) | ported; (b) | 54 relative-intent encodings decode; the others return before any API call |
| SetPreferredColorProfileTestFromGray | ported; (b) | SetPreferredColorProfile or SetOutputColorProfile with ICC, with and without the CMS |
| DecodeAllEncodingsVariantsTest/SetPreferredColorProfileTest (864) | ported; (b) | The 72 instances with relative D65 sources run every destination; the others return before any API call |
| DecodeAllEncodingsWithCMSTest/DecodeWithCMS (216) | ported; (b) | SetOutputColorProfile by ICC with the CMS and by encoding without it |
| PixelTestOpaqueSrgbLossy | ported; (a), (b) | DecodeWithAPI with callbacks to RGB and RGBA |
| PixelTestOpaqueSrgbLossyNoise | ported; (a), (b) | DecodeWithAPI with early buffers, noise frame |
| ProcessEmptyInputWithBoxes | ported | ProcessInput before input and after releasing it, every box format |
| ExtraBytesAfterCompressedStream | ported; (a), (b) | DecodeWithAPI with trailing bytes; the unconsumed count |
| BrobBoxInputIsRestrictedToBoxSize | ported | JxlBoxContentDecoder capture `decode-test` |
| BrobBoxTruncatedStreamWithExactBoxSizeReturnsError | ported | JxlBoxContentDecoder capture `decode-test` |
| ExtraBytesAfterCompressedStreamRequireBoxes | ported; (a), (b) | The same with box events |
| ConcatenatedCompressedStreams | ported; (a), (b) | 100 box-format pairs; the second decoder starts at the first one's remainder |
| PixelPartialTest | ported | TestPartialStream: seven increments, eleven box formats, the increment carried as in the test |
| JPEGPartialTest | ported; (c) | The same with JPEG reconstruction and the growing JPEG buffer |
| DCNotGettableTest | ported | The 68-byte literal |
| PreviewTest | ported; (b) | Preview buffer size, NEED_PREVIEW_OUT_BUFFER, PREVIEW_IMAGE, small and big previews |
| AlignTest | ported; (a), (b) | Row alignment 17 with buffer and callback |
| AnimationTest | ported; (a), (b) | Frame headers and names per frame |
| AnimationTestStreaming | ported; (a), (b) | 16-byte input steps |
| ExtraChannelTest | ported; (b) | Extra-channel buffer with aligned rows |
| SkipCurrentFrameTest | ported | SkipCurrentFrame before, during and after progressive passes |
| SkipCurrentFrameForgetsExtraChannelBuffers | ported | The skipped frame's extra-channel buffer stays untouched |
| SkipFrameTest | ported; (a), (b) | SkipFrames, Rewind and a second skipping pass |
| SkipFrameWithBlendingTest | ported; (a), (b) | Reference decode, rewind mid-way, both skipping passes |
| SkipFrameWithAlphaBlendingTest | ported; (a), (b) | The same with cropped alpha layers, coalesced and not |
| OrientedCroppedFrameTest | ported; (a), (b) | Eight orientations, four resamplings, kept or applied, coalesced and not |
| ProgressionTest | ported; (b) | VerifyProgression at the AnalyzeCodestream breakpoints, with FlushImage |
| ProgressionTestLosslessAlpha | ported; (b) | The same for responsive lossless alpha |
| InputHandlingTestOneShot | ported | Released positions after every event, eleven box formats, six event sets |
| InputHandlingTestJPEGOneshot | ported; (c) | The same with JPEG reconstruction, ten box formats |
| InputHandlingTestStreaming | ported | Growing input windows of 1, 7, 27 and 1024 bytes, box events |
| FlushTest | ported; (b) | FlushImage before the output, on partial input, then the rest |
| FlushTestImageOutCallback | ported; (b) | The same through the image-out callback |
| FlushTestLossyProgressiveAlpha | ported; (b) | The same with alpha |
| FlushTestLossyProgressiveAlphaUpsampling | ported; (b) | The same with color and alpha upsampling |
| FlushTestLosslessProgressiveAlpha | ported; (b) | The same for responsive lossless alpha |
| DecodeProgressiveTest/ProgressiveEventTest (8) | ported; (b) | Progressive-detail errors and FRAME_PROGRESSION events with FlushImage for kDC, kLastPasses and kPasses |
| JPEGReconstructTestCodestream | ported; (c) | VerifyJPEGReconstruction with a preview |
| JPEGReconstructionBufferNotKeptAcrossReset | ported; (c) | The released buffer of the first decode stays untouched |
| JPEGReconstructionRewindInsideBox | ported; (c) | Rewind inside the jbrd box, then a complete reconstruction |
| JPEGReconstructionTest | ported | Container built from the testdata JPEG; reconstructs it exactly |
| JPEGReconstructionMetadataTest | ported | The testdata file reconstructs its JPEG exactly |
| ContinueFinalNonEssentialBoxTest | ported | SUCCESS before the final box, then after it |
| ExtendedBoxSizeTest | ported | Box types and sizes of the testdata file |
| BoxTest | ported | Box getters, SetBoxBuffer and ReleaseBoxBuffer per box, CloseInput |
| ExifBrobBoxTest | ported | Raw and decompressed brob boxes, all at once and in 64-byte steps |
| PartialCodestreamBoxTest | ported; (b) | jxlp contents through growing box buffers, then their codestream |
| OutOfOrderJxlpDuplicateIndexTest | ported | The duplicate out-of-order index is an error |
| SpotColorTest | ported; (b) | Spot colors rendered and not |
| CloseInput | ported | CloseInput on a one-byte file |
| Non444AdaptiveDCSmoothing | ported | The malformed 62-byte file is an error |

## Low-memory render pipeline (2026-11-04)

LowMemoryRenderPipeline and GroupBorderAssigner are ported from
`render_pipeline/low_memory_render_pipeline.{h,cc}` and
`dec_group_border.{h,cc}`, together with the RenderPipeline base, Builder and
RenderPipelineInput from `render_pipeline.{h,cc}`. The Builder now selects the
simple pipeline only after UseSimpleImplementation. PreparePipeline calls it
only when `use_slow_render_pipeline` is set, as in `dec_cache.cc`. The flag
comes from the FrameDecoder's `use_slow_rendering_pipeline`. The fixed profiles
and `SequenceDecoder::new` keep the simple pipeline and their parity cases.
`SequenceDecoder::with_rendering_pipeline(metadata, false)` selects the
low-memory pipeline, as `decode.cc` does. The public decoder API (PR #50) uses
it for every frame. Execution is sequential with a null pool.

The oracle's `fs-decode-lm` command runs the original FrameDecoder with
`use_slow_rendering_pipeline=false` on genuine original encodings: reference,
blending, crop, animation and preview sequences, main and extra-channel
upsampling, multigroup and DC-ladder files, every chroma/YCbCr mode, JPEG
chroma files, all extra-channel types including spot colors, patches, and
VarDCT files with EPF 1 to 3, Gaborish, noise and splines on single and several
groups and DC groups. Frames left of, above, right of and below the image
reach RenderPadding. Truncated, corrupted and trailing-data variants compare
the partial output. All 1,986 low-memory observations match their original
exactly, including every initialized output, reference and DC sample word.
When the two original pipelines agree, the Rust simple and low-memory outputs
also agree (1,882 cases). The original pipelines differ in 100 cases, and Rust
reproduces each original result. The simple pipeline rejects early-upsampled
odd-sized extra channels that the low-memory pipeline renders, and keeps
different samples for frames entirely left of or above the image. For frames
entirely right of or below the image, the original simple pipeline's
foreground copy has no defined result (the reference build crashes), so those
four files are compared with the low-memory pipeline only.

Local evidence: `python3 tools/check.py` passes on macOS AArch64. Hosted CI
evidence for this change is recorded in its pull request.

## Whole-image color output (2026-11-04)

The color, tone-mapping and CMS stage selection of
`PassesDecoderState::PreparePipeline` (`dec_cache.cc`) is translated into three
source-ordered segments in `dec_cache.rs`. The frame decoder keeps the
interleaved blending and reference writers between them. `OutputEncodingInfo`,
`OpsinParams` and `CanOutputToColorEncoding` were already translated in
`dec_xyb.rs` and are reused unchanged. The sequence decoder no longer limits
metadata to sRGB. It admits every enum and ICC color encoding, rendering intent
and intensity target. Its setters perform the `OutputEncodingInfo` operations of
decode.cc's desired-intensity, CMS and output-profile entry points; that API's
own argument checks remain with slice 7.

The original simple pipeline cannot run a CMS stage with defined behavior. It
calls `PrepareForThreads` before the first `SetInputSizes`, so
`CmsStage::PrepareForThreads` initializes the CMS with an indeterminate
`xsize_`. The sequence decoder therefore returns `Unsupported` at the
pipeline checkpoint when the source would add a CMS stage. That covers XYB
output to a non-original target with a CMS set, enum/ICC conversions of
non-XYB images and requested ICC targets. These paths need the low-memory
pipeline, which sets input sizes first; they are not claimed here.

All other configurations are connected. XYB images decode to every encoding
that `CanOutputToColorEncoding` accepts through FromLinear, including PQ, HLG,
linear, gamma, DCI and BT.709 transfers. PQ Rec. 2408 tone mapping and the HLG
OOTF reach desired intensity targets with and without a CMS. ICC-tagged images
keep their original profile as output. Gray/color requests follow the original
image-bundle writer, which rejects a gray/color change; both implementations
report that error.

`tests/parity/color_output.rs` compares 103 genuine files from the original
public encoder: lossless, XYB VarDCT and non-XYB VarDCT; every primaries, white
point and transfer family; gray, alpha, 12/16-bit, float and multigroup inputs;
and 27 files tagged with nine pinned skcms ICC profiles. One 16-bit lossless
file requires level 10; its unchanged codestream box payload is decoded. All
3,571 decode requests match exactly, including statuses, OutputEncodingInfo
state and every output float word of the 1,510 complete decodes. The actual
public `JxlDecoder` writes the same float words to its buffer for all 1,510.
A further 320 requests cover profiles spliced in by the original header/ICC
writers: malformed, CMYK, gray-on-RGB and garbage. They also cover truncated
and corrupted ICC and frame bytes. Twelve CMS-stage configurations stop at
the documented checkpoint, while the original public decoder completes them.
These are local macOS AArch64 results; native Linux CI acceptance is listed
in the pull request. PR #50 admits these CMS-stage conversions on the
low-memory pipeline that the public decoder uses. Its color-profile and frame
groups and the decode_test.cc CMS tests (PR #52) compare them exactly, which
completes decoder checklist step 9.

## JxlToJpegDecoder (2026-11-04)

`lib/jxl/decode_to_jpeg.{h,cc}` is translated completely into
`decode_to_jpeg.rs`: output buffer set/release, Reset and StartBox, jbrd
Process buffering and DecodeJPEGData, Exif/XMP marker counts and box sizes,
SetExif/SetXmp, SetImageBundleJpegData with ICC injection, and WriteOutput with
its JPEG_NEED_MORE_OUTPUT result. The jpeg_data.h tag arrays now have one shared
definition. Representation choices are recorded in ADR 0003.

The differential corpus uses 39 container files from the original public
encoder: nine genuine cjpeg inputs (baseline, gray, 4:2:0 progressive, 4:2:2
restarts, three RGB files and two multi-chunk files), each plain and with Exif,
XMP, COM and duplicate-Exif APP1 markers. Every complete script reconstructs the
original JPEG byte for byte. The 12,205 requests compare statuses, consumed
sizes, owner snapshots and every output byte, covering arbitrary input chunks,
bounded/unbounded boxes, sticky box_until_eof, repeated boxes on one owner,
every output buffer size up to two bytes past each JPEG of at most 1,100 bytes,
sampled sizes and multi-chunk writer boundaries for larger files, and ICC
injection through the original EncodeJPEGData. Of these, 9,855 requests cover
every jbrd truncation and single-bit mutations of seven Exif/XMP/COM fixtures.
A SetExif call with a matching Exif marker shorter than nine bytes overflows
both original copies. Four such requests run as original-only probes; the
instrumented original reports a heap-buffer-overflow write for each, and Rust
stops at its bounds check.

The complete ordinary and instrumented local checks pass on macOS AArch64: 439
parity tests and 879 tests in total each, with zero failures or ignored tests.
Native Linux acceptance runs in CI. PR #50 connects it to the JxlDecoder
events, JxlDecoderSetJPEGBuffer and the Exif/XMP box stages of `decode.cc`.

## Recovered decoder state and reconstruction (2026-11-03)

The recovery combines persistent incremental input, callback-backed PaddedBytes,
box-content decompression, JPEG coefficients and complete DecodeJPEGData marker
payload reconstruction. The input and accepted output paths now share the same
JxlMemoryManager type and the original alignment-group counter. JPEG coefficient
paths retain the accepted frame-storage ownership split. The new payload corpus
uses the actual original encoder and compares complete, truncated, trailing-data
and repeated-owner histories: 4,740 defined requests compare exactly. When an
earlier call on the same owner leaves an SOS marker behind, the original
validation reads one scan_info element past its end. Those 24 requests run as
separate original-only probes, and every instrumented graph must report that
heap-buffer-overflow; they have no C++/Rust comparison (ADR 0003). The public
container/event orchestration and JPEG reconstruction API followed in PR #50.

## Recovered Brotli decoder (2026-11-03)

The complete streaming decoder and its allocator/state/dictionary boundaries are
recovered on top of the reviewed decoder integration. All 40 Rust modules and
the pinned dictionary bytes match the saved implementation. The complete local
ordinary check passes Debug, Release and Rust 1.85: 765 tests per configuration,
including the unchanged 374 libjxl differential groups and 1,455,093 exact
comparisons, plus the Brotli tests and 16 independently bound source observers.
Dedicated streaming and API audits are now required in CI. The later
deep gate (retired libjxl-rs repository, run 41762944219)
on the merged PR #50 tree passes them, with the instrumented checks, on native
Linux and macOS. The stack-based one-shot decoder remains unported.

## Recovered CMS work (2026-11-03)

The integration recovers the transfer/matrix, nonlinear-fitting, baseline SIMD
transform and default-CMS work on top of the reviewed decoder components.
The complete ordinary local check passes Debug, Release and Rust 1.85. The
instrumented and native gates are being rerun for this combined tree;
earlier component counts below describe their original trees. The original
optimized Linux concat graphs disagree on new paired-NaN products. A dedicated
complete-source observer and preserved native investigation now pin that
boundary without changing the original archive, previous requests, or finite
results. The later
deep gate (retired libjxl-rs repository, run 41762944219)
on the merged PR #50 tree passes these groups and the skcms and default CMS
audits on native Linux and macOS. Complete-image ICC admission followed in
PRs #47 and #50.

The HSW (N=8) and SKX (N=16) skcms stage engines and the original cpu_type()
dispatch are now translated. Native Linux x86_64 CI previously failed three
public transform groups because the original archive selected an x86 engine
while Rust ran only the baseline: F16C half conversion, the 4*4*N tail
temporary and N-pixel alias traversal differ from N=4. Each native host exercises
only the engine its CPU selects, so a runner without AVX-512 does not compare
SKX. SKX acceptance requires an AVX-512 host or an explicitly pinned engine
configuration; neither is claimed yet.

The combined Brotli and CMS recovery passes the complete ordinary local check
on macOS AArch64 in Debug and Release: 833 tests per configuration, with zero
failures or ignored tests. Native Linux and macOS acceptance is covered by the
later deep gate above; SKX remains unaccepted.

## Reviewed decoder integration (2026-11-03)

PR #34 (retired libjxl-rs repository) delivers the
output stage and signature/container-header helpers, including PR #29. Its
native acceptance (retired libjxl-rs repository, run 41548861657)
passes all six ordinary/instrumented Debug, Release and Rust 1.85 configurations
on Linux and macOS. The lifecycle work in PR #26 was already integrated through
PR #30. The SpotColor implementation, component tests and independent source
audit from PR #23 are already identical to the accepted extra-channel integration.
Those two old PRs do not represent missing decoder features.

The next integration combines scalar color-transfer stages (#22), JPEG writing
(#25), incremental ICCReader (#27), JPEG metadata fields (#28), the skcms ICC
parser (#36), and frame-state ownership preparation (#38). It retains their
component implementations, fixtures and assertions unchanged. Shared module,
command, build and test registrations include every component once. The existing
native source-audit steps stay enabled, with additional ICCReader, JPEG metadata
and skcms parser audits on both platforms. The integrating PR must supply native
acceptance for this combined tree; earlier individual slice counts below are
historical and do not describe the combined test inventory.

Both complete local checks pass: `python3 tools/check.py --msrv` and
`python3 tools/check.py --sanitizers --msrv`. Each of the six Debug, Release and
Rust 1.85 configurations passes 611 tests, 374 differential groups and 1,455,093
exact defined comparisons, with zero failures or ignored tests. The independent
ICCReader, JPEG metadata and skcms parser audits also pass both original graphs:
2,159 reader histories / 7,894 steps, 11,852 metadata requests and 247,790 parser
requests. They retain the separate positive original-only diagnostics and verify
live source bindings without changing active original objects. Rustdoc builds
with all features and warnings denied. These local ARM results do not substitute
for the integrating PR's required native Linux/macOS acceptance.

The skcms transfer/matrix work in #35 and its curve-fitting dependent #37 remain
outside this integration. Native Linux matrix concatenation still fails exact
comparisons, and original ordinary/instrumented matrix observations disagree.
No relaxed comparison, changed expected word or unaccepted reference
configuration is used to admit these ports. Whole-image ICC/HDR, default CMS,
JPEG reconstruction, container input and the public incremental decoder
followed in PRs #46 to #50.

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
| `encode_internal.h`, out-of-line wrapper methods in `encode.cc` | `encode_internal.rs` | BitsPerChannel, BytesPerPixel, the buffer-backed adapter/channel construction, binding, copying, views and format accessors, MakeBoxType, frame-index-box storage, JxlEncoderFrameSettingsValues, queued frame/box/input owners, and the buffer mode of JxlEncoderOutputProcessorWrapper with JxlOutputProcessorBuffer and AppendData; callback/JPEG input, CopyFromBuffer, CopyOutput, the external output processor, container headers and frame-index-box methods unported |
| `encode.cc` lifecycle, setters and queueing, JxlEncoder/JxlEncoderFrameSettings in `encode_internal.h`, values in `../include/jxl/encode.h` | `encode.rs` | Create/Reset/Destroy, GetError, version, container/level/box flags, VerifyLevelSettings, basic-info, extra-channel, upsampling, color and ICC setters, frame-settings creation and every option/float option, distances, frame header/blend/name/bit depth, AddImageFrame, AddBox, SetExtraChannelBuffer, Close* and ProcessOutput; the raw-codestream branch of ProcessOneEnqueuedInput; admission results at CanDoFastLossless, ProcessOneEnqueuedInput (alpha, ICC, containers, boxes) and the frame encoder; containers, JPEG/chunked input, output processors, runner, debug and statistics APIs and the C ABI unported |
| `luminance.h`, `.cc` | `luminance.rs` | Complete SetIntensityTarget |
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
| `enc_context_map.h`, `.cc` | `enc_context_map.rs` | MTF, cost comparison, simple/entropy-coded context output, and complete VarDCT BlockCtxMap output |
| `enc_lz77.h`, `.cc` | `enc_lz77.rs`, `enc_lz77_costs.rs` | Symbol costs, RLE, greedy bucket variants, optimal hash-chain variants, and original method dispatch |
| `enc_ans_params.h`, ForModular in `enc_ans.cc` | `enc_ans_params.rs` | Histogram storage, cost, parameter fields/defaults, speed-tier initialization, uint config, and complete modular entropy-policy selection |
| `enc_cluster.h`, `.cc` | `enc_cluster.rs` | Scalar Condition/entropy/divergence, cluster selection, pair refinement, and canonical reindexing |
| `base/fast_math-inl.h`, `base/rational_polynomial-inl.h` | `base/fast_math.rs`, `cms/transfer_functions_scalar.rs` | Existing one-lane FastLog2f/FastPow2f/FastPowf, CubeRootAndAdd, and degree-two/degree-four rational evaluation with original x86 fence sites; other functions/targets unported |
| Highway `hwy/ops/scalar-inl.h`, `generic_ops-inl.h` | `base/hwy_scalar.rs` | Used one-lane Min/Max, unfused MulAdd, Clamp, Floor, Round, saturating float-to-i32 conversion, and sign-bit ZeroIfNegative; other operations/targets unported |
| `common.h` | `common.rs` | SpeedTier and maximum pass count; other constants unported |
| `ac_strategy.h`, `.cc` | `ac_strategy.rs` | Raw AcStrategyType values, size/count constants, strategy objects/geometry, complete natural coefficient order/LUT traversal, and AcStrategyImage allocation/fills/validity/counting, interpreted row views, and strategy setters with source overlap/error mutations |
| `chroma_from_luma.h`, `.cc` | `chroma_from_luma.rs` | Complete ColorCorrelation constants/defaults, ratios, setters/getters, DC parsing, and ColorCorrelationMap allocation/default/move ownership/delegated DC parsing |
| `quantizer.h`, `.cc` | `quantizer.rs` | Constructors/recomputation, DC and inverse AC scaling, ClearDCMul, QuantizerParams visitor, GetParams/Decode, ClampVal/ScaleGlobalScale, same-owner matrix forwarding, and the field selection ComputeGlobalScaleAndQuant/SetQuantFieldRect/SetQuantField/SetQuant with the AArch64 contraction; diagnostic dump unported |
| `../include/jxl/color_encoding.h`, enum masks in `color_encoding_internal.h` | `color_encoding.rs` | Raw color values and external field storage with checked C layout; public API entry points unported |
| `../include/jxl/cms_interface.h`, debug callback signature in `encode.h` | `cms_interface.rs`, `enc_params.rs` | Non-owning profile/descriptor storage and callback signatures; ICC/transform invocation is translated below; default CMS backend and public encoder API remain unported |
| `cms/color_encoding_cms.h` | `cms/color_encoding_cms.rs` | Coordinates, gamma, field storage/conversions, comparisons, partial setter state, and callback-backed ICC field extraction; default ICC parser unported |
| `color_encoding_internal.h`, `.cc`, SetFieldsFromICC in `cms/color_encoding_cms.h` | `color_encoding_internal.rs`, `color_space_transform.rs`, `cms/color_encoding_cms.rs` | Field bundles, profile generation, raw ICC storage, static encodings, callback SetICC/SetFieldsFromICC with conditional byte transfer, DecideIfWantICC, complete callback-backed ColorSpaceTransform and destruction; default CMS backend and C ABI entry points unported |
| `image_metadata.h`, `.cc`, quantizer bias constants | `image_metadata.rs`, `image_metadata_weights.rs` | Bit depth, extra channels, opsin/custom weights, tone mapping, image/codec metadata, sample/alpha/orientation helpers, and metadata reading; diagnostic output unported |
| `frame_header.h`, `.cc` | `frame_header.rs` | Frame fields, chroma subsampling, blending, animation, progressive passes, names, dimension/reference helpers, order field encoding, and parsing; diagnostic strings unported |
| `enc_params.h` | `enc_params.rs` | All compression parameter storage/defaults, lossless predicates/setter, CMS descriptor setter, and borrowed custom inputs; encoding operations remain separate |
| `enc_progressive_split.h` | `enc_progressive_split.rs` | Pass definitions/mode construction, splitter configuration, and InitPasses with original narrowing and reused-array state; SplitACCoefficients<int32_t> |
| `enc_frame.h`, `.cc` | `enc_frame.rs` | FrameInfo, ParamsPostInit, group-size/count helpers, progressive selection, flags/filter policy, complete MakeFrameHeader, and fixed lossless Thunder color/alpha CopyColorChannels, buffer-backed CopyExtraChannels, ComputeEncodingData/EncodeFrameOneShot through byte-owner handoff; reached encode.cc metadata/padding block connects complete raw codestream output; general frame/image encoding, C output processors/containers, and remaining entry points unported |
| `enc_frame.cc` (general frame entry) | `enc_frame_vardct.rs` | EncodeFrame setting resolution, CanDoStreamingEncoding, EncodeFrameOneShot, GetBitDepth, general CopyColorChannels, the VarDCT branch of ComputeEncodingData, ComputeChromacityAdjustments, reached ComputeNoiseParams/DownsampleColorChannels/PermuteGroups branches, ComputeVarDCTEncodingData, ComputeAllCoeffOrders, TokenizeAllCoefficients, EncodeGlobalDCInfo, non-streaming EncodeGlobalACInfo and EncodeGroups; streaming, JPEG, alpha/extra channels, noise, downsampling and center-first order are admission results |
| `passes_state.h`, `.cc`, `enc_cache.h`, `enc_cache.cc` | `passes_state.rs`, `enc_cache.rs` | Complete shared/encoder storage and constructors, InitializePassesSharedState allocation/error order, four reference owners, retained indexed DC write owners/mapped DC access, same-owner DC/patch/frame bindings; ComputeACMetadata and InitializePassesEncoder without the progressive-DC frame (serial) |
| `dec_cache.h`, InitForAC in `.cc`, callback typedefs in `lib/include/jxl/decode.h` | `dec_cache.rs` | Complete decoder-state storage/constructor/Init, own or borrowed shared selection, original coefficient owner, retained shared reference temporary, PixelCallback storage/presence/unsafe Init, ImageOutput carriers, null-pool InitForAC, complete GroupDecCache constructor storage and InitDCBufferOnce; fixed PreparePipeline is connected below, complete VarDCT GroupDecCache::InitOnce; the general PreparePipeline for callers outside FrameDecoder (`prepare_pipeline`, used by the encoder's ReconstructImage; the coalescing blending branch is an admission result); output lifecycle and custom runners remain separate |
| `dec_frame.h`, `.cc`, reached PreparePipeline in `dec_cache.cc` | `dec_frame.rs` | DecodeFrame and the complete public incremental FrameDecoder: InitFrame/InitFrameOutput, SectionInfo/TocEntry/SectionStatus, ProcessSections with partial and permuted input and progressive DC/pass pauses, Flush, FinalizeFrame, References/SavedAs, TOC/pass/DC accessors, SetPauseAtProgressive/NextNumPassesToPause, SetRenderSpotcolors/SetCoalescing, SetImageOutput/AddExtraChannelOutput with the original output-stage selection; the retained pool runs the DC/AC group and forced-draw RunOnPool sites (task bodies serialized under a runner; null for the internal profiles), with the constructor's simple/low-memory pipeline selection (fixed profiles select the simple pipeline), explicit profile admission; `decode.cc` orchestration remains separate |
| DecodeFile in `render_pipeline/render_pipeline_test.cc`, CheckMetadata in `../extras/codec_in_out.h` | `dec_frame.rs`, private admission in `enc_frame.rs` | Whole fixed-profile codestream parsing, owned metadata, pre-padding output initialization, exact frame dispatch, EOF/bounds/metadata/close checks, and connected original integer export for gray/RGB and gray/alpha/RGBA; preview/animation/ICC/general profiles remain outside admission |
| `opsin_params.h`, `.cc`, output-encoding methods in `dec_xyb.h`, `.cc` | `opsin_params.rs`, `dec_xyb.rs` | Pinned default-matrix selection, four-lane inverse-matrix expansion, OpsinParams initialization, CanOutputToColorEncoding, complete OutputEncodingInfo storage and metadata/desired-profile setters; general image conversion and fast XYB output unported |
| `image_bundle.h`, selected methods in `.cc` | `image_bundle.rs` | Storage/constructors, metadata/color/extra/size access, SetFromImage conditional ownership transfer, VerifySizes/VerifyMetadata, alpha presence/premultiplication/plane access, decoded-byte accessors, RemoveColor, and declaration-ordered explicit move with destination-borrowed source color aliases; general import/export/copy unported |
| `render_pipeline/render_pipeline_stage.h`, `.cc`, write stages in `stage_write.cc` | `render_pipeline/render_pipeline_stage.rs` | Complete stage settings/base hooks/row access and WriteToImage3F/WriteToImageBundle stages with borrowed/retained standalone, indexed DC and existing-reference write targets; pixel-buffer output/callback stage is mapped separately below; remaining concrete stages unported |
| `render_pipeline/render_pipeline.h`, `.cc` | `render_pipeline/render_pipeline.rs` | Complete RenderPipeline base, Builder (UseSimpleImplementation selection, padding/shift tables, Init), and RenderPipelineInput with input/counter/thread-preparation operations; concurrent pipeline access unported (runner tasks are serialized) |
| `render_pipeline/simple_render_pipeline.h`, `.cc` | `render_pipeline/simple_render_pipeline.rs` | Complete simple storage, mirroring, stage scheduling, clipping, padding, and dimension transition; SimpleRenderPipelineBuilder keeps the concrete type after UseSimpleImplementation |
| `render_pipeline/low_memory_render_pipeline.h`, `.cc`, `dec_group_border.h`, `.cc` | `render_pipeline/low_memory_render_pipeline.rs`, `dec_group_border.rs` | Complete LowMemoryRenderPipeline: Init border/padding/trailing-stage/image-dimension setup, ring-buffer sizing replay, per-group/per-thread storage, border save/load, GroupBorderAssigner scheduling, RenderRect virtual-row scheduling with mirroring and x-padding, out-of-frame RenderPadding; runner tasks reach it one at a time, with per-thread storage selected by thread id |
| `render_pipeline/stage_from_linear.h`, `.cc`, `render_pipeline/stage_to_linear.h`, `.cc` | `render_pipeline/stage_from_linear.rs`, `render_pipeline/stage_to_linear.rs` | Complete scalar factory-reached Linear/sRGB/PQ/HLG/BT709/gamma/DCI conversion stages, copied operation state, row guards and load-transform-store order; legacy sRGB factory retained; general HDR/CMS and ICC whole-file integration remains separate |
| `render_pipeline/stage_gaborish.h`, `.cc` | `render_pipeline/stage_gaborish.rs` | Complete original one-lane stage/factory, channel-specific weight normalization, three-row 3x3 kernel, extra spans, symmetric border settings, and ignored extra channels; SIMD targets and general lossy frame integration unported; first VarDCT profile connected |
| `render_pipeline/stage_ycbcr.h`, `.cc` | `render_pipeline/stage_ycbcr.rs` | Complete scalar YCbCr stage/factory with original coefficients, double promotion/product narrowing, ordered binary32 additions and extra-span guards; connected at the original sequence color-conversion checkpoint |
| `render_pipeline/stage_chroma_upsampling.h`, `.cc` | `render_pipeline/stage_chroma_upsampling.rs` | Complete scalar horizontal/vertical stages, 3/4–1/4 interpolation, original borders/shifts/channel modes and factory; source-ordered channel stages precede filters in admitted YCbCr frames |
| `render_pipeline/stage_upsampling.h`, `.cc` | `render_pipeline/stage_upsampling.rs` | Complete original one-lane stage/factory, symmetric weight expansion, per-thread aligned workspaces, min/max preprocessing, chunked 2x/4x/8x interpolation/clamping, and original argument guards; NEON/AVX2 stage in `render_pipeline/stage_upsampling_simd.rs` (ADR 0004); sanitizer annotations unported |
| `render_pipeline/stage_write.h`, `.cc` (WriteToOutputStage) | `render_pipeline/stage_write.rs`, `stage_write_dither.rs` | Complete scalar output factory, original buffer/callback conversions and lifecycle, 1,024-pixel chunks, all orientation/alpha/extra-output paths, byte/half/float words, actual callback-backed scratch, allocation and retry failures; public incremental decoder orchestration remains open |
| `render_pipeline/stage_cms.h`, `.cc` | `render_pipeline/stage_cms.rs` | Complete factory/IsNeeded, profile copy/selection, input-size validation, transform preparation, RGB/gray/CMYK interleaving, run-status propagation, deinterleaving, and destruction; native unsafe construction carries the foreign callback contract, and source caller preconditions are checked before buffer access |
| `render_pipeline/stage_tone_mapping.h`, `.cc` | `render_pipeline/stage_tone_mapping.rs` | Complete one-lane factory/stage, original no-op/guard order, PQ scaling, Rec. 2408/HLG selection, per-pixel tone/gamut mapping, and in-place RGB stores; other SIMD targets unported |
| `dec_xyb-inl.h`, normal stage/factory in `render_pipeline/stage_xyb.cc` | `dec_xyb_scalar.rs`, `render_pipeline/stage_xyb.rs` | Complete original one-lane XybToRgb, normal XYBStage/GetXYBStage, scaled XYB output, copied opsin/profile selection, original xextra guards, and in-place color/ignored extra-channel modes; general image conversion, FastXYB/sRGB8, SIMD, and decoder-profile expansion remain unported |
| `dec_patch_dictionary.h`, `splines.h`, `noise.h` | `dec_patch_dictionary.rs`, `splines.rs`, `noise.rs` | Complete documented scalar feature storage/parsing/rendering boundaries mapped below; reference identities are shared with stages, while general complete-frame orchestration remains separate |
| `convolve.h`, `convolve_symmetric5.cc`, SlowSymmetric3 in `convolve_slow.cc` | `convolve.rs`, `convolve_symmetric5.rs`, `convolve_slow.rs` | Weight owners, one-lane Symmetric5 border/interior rows with mirrored and unchanged row wraps, and SlowSymmetric3 with the AArch64 contraction; SlowSeparable5, Separable5 and SIMD targets unported |
| `enc_gaborish.h`, `.cc` | `enc_gaborish.rs` | Complete GaborishInverse |
| `enc_adaptive_quantization.h`, `.cc` | `enc_adaptive_quantization.rs` | InitialQuantDC, InitialQuantField with the complete one-lane AdaptiveQuantizationMap (tiles, masks, FuzzyErosion, PerBlockModulations, Blur1x1Masking) and AdjustQuantField, with the AArch64 contractions; FindBestQuantizer's butteraugli loops (kKitten and slower), TileDistMap, RoundtripImage and debug heat maps unported |
| `enc_chroma_from_luma.h`, `.cc` | `enc_chroma_from_luma.rs` | Complete CfLHeuristics (Init/InitDCStorage, PrepareForThreads, ComputeTile) and one-lane FindBestMultiplier; ColorCorrelationEncodeDC |
| `enc_ac_strategy.h`, `.cc` | `enc_ac_strategy.rs` | Complete AcStrategyHeuristics and one-lane ProcessRectACS with EstimateEntropy, FindBest8x8Transform, TryMergeAcs and FindBestFirstLevelDivisionForSquare; the disabled JXL_DEBUG_AC_STRATEGY dump unported |
| `enc_transforms.h`, `.cc`, `enc_transforms-inl.h` | `enc_transforms.rs` | Complete scalar TransformFromPixels for all 27 strategies, DCFromLowestFrequencies with ReinterpretingIDCT, and AFVDCT4x4, with the AFV DC-mix contraction; vector kernels unported |
| `enc_xyb.h`, `.cc` | `enc_xyb.rs` | ComputePremulAbsorb, LinearRGBRowToXYB, and ToXYB for linear sRGB and sRGB input (gray or color) with linear output; ApplyColorTransform inputs are not admitted by type; ScaleXYB, ScaleXYBRow, RgbToYcbcr and SIMD targets unported |
| `dct_scales.h`, scalar `dct-inl.h`, `dct_block-inl.h`, and `transpose-inl.h` | `dct_scales.rs`, `dct.rs` | Original literal scale/multiplier tables, one-lane forward/inverse radix-2 recursion, scaled 2D scheduling, transposed coefficient layout, coefficient mutation, and scratch offsets for power-of-two sizes 1 through 256; vector kernels unported |
| `dec_transforms-inl.h` | `dec_transforms.rs` | Complete original scalar TransformToPixels dispatch for all 27 strategies (including identity, hierarchical 2x2, split 4x4/4x8, and all AFV flips), ReinterpretingDCT and LowestFrequenciesFromDC; first VarDCT frame/group integration connected; general frame profiles remain separate |
| `dct_util.h`, `ac_context.h` | `dct_util.rs`, `ac_context.rs` | Complete two-type ACImage owners/virtual operations, BlockCtxMap storage/defaults/lookup/buckets, and normalized zero-density tables/helpers |
| `entropy_coder.h`, `.cc` | `entropy_coder.rs` | Complete scalar top/left prediction, threshold distributions, and BlockCtxMap parsing with original partial/reused state and validation order |
| `dec_group.cc` (`DecodeACVarBlock`, `GetBlockFromBitstream`, group orchestration) | `dec_group.rs`, `dec_group_render.rs`, `quantizer_inl.rs` | Complete per-block quantized AC symbol loop with real entropy reader, nonzero prediction/row writes, custom order scatter, signed coefficient reconstruction, both coefficient widths, pass shifts/addition, and partial failure state; GetBlockFromBitstream, complete DecodeGroup (including the zero-pass 8x-upsampled DC draw and stored-coefficient forced draw used by Flush) and final ANS validation are connected to scalar dequantization/traversal in dec_group_render.rs; GetBlockFromEncoder and DecodeGroupForRoundtrip (the encoder's decoder in the loop) |
| `dec_modular.h`, selected functions in `dec_modular.cc` | `dec_modular.rs` | ModularStreamId constructors/index mapping, sample conversion, complete ModularFrameDecoder storage/constructor/Init/DecodeGlobalInfo/MaybeDropFullImage/DecodeGroup, modular-to-pipeline conversion, FinalizeDecoding with the frame's pool (inverse transforms and group conversion; small images use a null pool), and state accessors; complete DecodeVarDCTDC and DecodeAcMetadata connected to the first VarDCT frame profile; general frame orchestration and debug descriptions remain separate; quantization-table decoding is mapped below |
| `enc_modular.h`, helpers and stream methods in `enc_modular.cc` | `enc_modular.rs` | Native stream-owner construction/init, group channel/crop selection, local palettes/RCT/WP selection, shared/custom/learned tree construction, token generation, global/local entropy output, stream cleanup/indexing, sample/transform helpers, and ComputeEncodingData including same-depth alpha for the fixed lossless Thunder path, and the VarDCT AddVarDCTDC (QuantizeWP, QuantizeGradient) and AddACMetadata; general pixel preparation, frame/section orchestration, and custom runners/allocators unported |
| `EstimateCost` in `enc_modular_simd.cc` | `enc_modular_simd.rs` | Original scalar gradient residual cost, context histograms, integer/fractional accumulation, and final rounding; SIMD kernels/context-map table unported |
| `quant_weights.h`, `.cc`; `dec_modular.cc` (`DecodeQuantTable`) | `quant_weights.rs`, `quant_weights_library.rs` | Distance-band and owning encoding constructors/copies/moves, strategy table mapping, DequantMatrices initialization/setters/DC APIs, all 17 coefficient-library entries, scalar AC matrix computation and aligned accessors, cached strategy aliases, and complete AC table decoding with local/shared modular RAW streams, allocating through the decoder's memory manager; non-owning raw base copies and SIMD targets remain separate |
| DC methods in `enc_quant_weights.cc` | `enc_quant_weights.rs` | Complete DC serialization, custom-DC encode/decode round trip, and DC scaling; DequantMatricesEncode for all-default matrices; custom AC quantization encoding (EncodeQuant) and whole-matrix round trips unported |
| `enc_group.cc` | `enc_group.rs` | One-lane QuantizeBlockAC, AdjustQuantBlockAC, QuantizeRoundtripYBlockAC, ComputeCoefficients and EncodeGroupTokenizedCoefficients |
| `enc_entropy_coder.cc` | `enc_entropy_coder.rs` | One-lane NumNonZeroExceptLLF, NumNonZero8x8ExceptDC and TokenizeCoefficients |
| `enc_heuristics.cc` (part) | `enc_heuristics.rs`, `enc_heuristics_lossy.rs`, `enc_heuristics_ar.rs` | FindBestBlockEntropyModel; LossyFrameHeuristics and FindBestDequantMatrices (default matrices; FindBestQuantizer reached only as its kSquirrel no-op); ReconstructImage, ComputeBlockL2Distance and ComputeARHeuristics (serial); the downsampling helpers unported |
| `enc_patch_dictionary.cc` (part) | `enc_patch_dictionary.rs` | Serial FindTextLikePatches without debug dumps; FindBestPatchDictionary bin packing, RoundtripPatchFrame and PatchDictionaryEncoder unported |
| `dec_noise.h`, `.cc`, `xorshift128plus-inl.h`, `render_pipeline/stage_noise.h`, `.cc` | `dec_noise.rs`, `xorshift128plus.rs`, `render_pipeline/stage_noise.rs` | Complete scalar noise parsing/generation/convolution/application and admitted VarDCT file integration; frame-index and upsampling branches retain source seeds and geometry; NEON/AVX2 generation and stages in `dec_noise_simd.rs`, `render_pipeline/stage_noise_simd.rs` (ADR 0004) |
| `dec_patch_dictionary.h`, `.cc`, `patch_dictionary_internal.h`, `render_pipeline/stage_patches.h`, `.cc` | `dec_patch_dictionary.rs`, `patch_dictionary_internal.rs`, `render_pipeline/stage_patches.rs` | Complete dictionary decoding/partial state, original libc++ interval tree, reference masks and clipped blending; borrowed/shared stages; complete saved-frame orchestration remains separate |
| `alpha.h`, `.cc`, `blending.h`, `.cc` | `alpha.rs`, `blending.rs` | Complete scalar blending/alpha kernels and temporary-plane ownership; original pointer-identity branches and selected numeric configuration |
| `splines.h`, `.cc`, `render_pipeline/stage_splines.h`, `.cc` | `splines.rs` | Complete scalar spline quantization, entropy decoding, curve/cache generation and rendering, with borrowed/shared SplineStage |
| `jpeg/jpeg_data.h`, header-selection helpers in `jpeg/enc_jpeg_data.cc` | `jpeg/jpeg_data.rs`, `jpeg/enc_jpeg_data.rs` | Parsed marker/component/coefficient storage, chroma selection, and JFIF/Adobe/component-ID color selection; JPEG parsing, field serialization, and reconstruction unported |
| `render_pipeline/stage_splines.h`, `.cc` | `render_pipeline/stage_splines.rs` | Complete scalar stage/factories with borrowed or shared cache identity and safe disjoint row spans; complete-file integration remains separate |
| `splines.h`, `.cc` | `splines.rs` | Complete scalar spline quantization/dequantization, entropy decoding, curve generation, draw-cache construction and rendering; borrowed/shared SplineStage is mapped above; NEON/AVX2 SegmentsFromPoints and DrawSegments in `splines_simd.rs` (ADR 0004) |
| `jpeg/jpeg_data.h`, CalculateMcuSize in `.cc`, header-selection helpers in `jpeg/enc_jpeg_data.cc` | `jpeg/jpeg_data.rs`, `jpeg/enc_jpeg_data.rs` | Parsed marker/component/coefficient storage, complete MCU geometry, chroma selection, and JFIF/Adobe/component-ID color selection; JPEG parsing and field serialization unported |
| `jpeg/dec_jpeg_data_writer.h`, `.cc`, `dec_jpeg_output_chunk.h`, `dec_jpeg_serialization_state.h` | `jpeg/dec_jpeg_data_writer.rs`, `jpeg/dec_jpeg_output_chunk.rs`, `jpeg/dec_jpeg_serialization_state.rs` | Complete WriteJpeg marker/Huffman/quantization serialization, sequential/progressive/refinement scan writing, restarts, padding, chunk ownership and output callback lifecycle; compressed reconstruction metadata and complete-file coefficient integration remain open |
| `jpeg/jpeg_data.h`, header-selection helpers in `jpeg/enc_jpeg_data.cc` | `jpeg/jpeg_data.rs`, `jpeg/enc_jpeg_data.rs` | Parsed marker/component/coefficient storage, chroma selection, and JFIF/Adobe/component-ID color selection; JPEG parsing and reconstruction unported |
| `jpeg/jpeg_data.cc`: complete `VisitFields` and `SetJPEGDataFromICC` | `jpeg/jpeg_data_fields.rs` | Complete metadata bundle field order, component/marker/table validation, delta-coded reset/zero-run indices, postponed buffer sizes, padding bits and ICC byte injection; complete Brotli DecodeJPEGData payload reconstruction is registered beside the bundle; full JPEG parsing and public reconstruction remain separate |
| `frame_dimensions.h` | `frame_dimensions.rs` | Pixel/block/group dimensions, original rounding/padding, and group rectangles |
| `base/rect.h` | `base/rect.rs` | Unsigned Rect coordinates, crop/intersection, translation, shifts, and extension; the RectT<ptrdiff_t> constructors, intersection, translation and shifts reached by the low-memory pipeline; image/row accessors unported |
| `pack_signed.h` | `pack_signed.rs` | Signed packing and pointer-width unpacking |
| `compressed_dc.cc` | `compressed_dc.rs` | Complete scalar DequantDC and AdaptiveDCSmoothing, source storage swaps and RunOnPool row ranges |
| `epf.cc`, `render_pipeline/stage_epf.h`, `.cc` | `epf.rs`, `render_pipeline/stage_epf.rs` | Complete block sigma generation and mirrored borders; complete scalar EPF0/1/2 stages and live shared sigma binding are connected for the expanded VarDCT profile |
| `loop_filter.h`, `.cc` | `loop_filter.rs` | Gaborish/EPF parameter fields, defaults, validation, and padding; scalar decoder Gaborish stage tracked above, scalar EPF stages mapped above |
| `lehmer_code.h` | `lehmer_code.rs` | Original Fenwick-tree encode/decode with u16/u32 permutation storage and caller-owned workspaces |
| `coeff_order_fwd.h` | `coeff_order_fwd.rs` | Coefficient type, order count, and row/column layout helpers |
| `coeff_order.h`, `.cc`, `enc_coeff_order.h`, `.cc` | `coeff_order.rs`, `enc_coeff_order.rs` | Coefficient storage-size limits/offsets, permutation context/tokenization/output/parsing, complete DecodeCoeffOrders with strategy aliases/natural defaults/unused encoded orders/final ANS validation; ComputeUsedOrders, ComputeCoeffOrder and EncodeCoeffOrders (encoder) |
| `toc.h`, `.cc`, `enc_toc.h`, `.cc` | `toc.rs`, `enc_toc.rs` | Section size/permutation output, budget validation, entry cap, parsing, group offsets/reordering, and layer accounting |
| `image.h`, `.cc` | `image.rs` | Plane/Image3 numeric storage through the owner's memory manager, padded row views, shrink/restore, swaps, native moves and source-header-preserving allocation transfer used by ImageBundle moves; MSan-only padding instrumentation unported |
| `image_ops.h`, `.cc` | `image_ops.rs` | Plane/Image3 copies and fills, rectangular/padded copies, Mirror with the WrapMirror/WrapUnchanged modes, in-place block padding, and both original scalar downsampling overloads; other header-only arithmetic/conversion/min/max APIs unported |
| `modular/options.h` | `modular/options.rs` | Raw predictors, property types, tree modes/kinds, and all modular option defaults |
| `modular/encoding/context_predict.h` | `modular/encoding/context_predict.rs` | All basic predictors, weighted header/presets/state, reference/property computation, flat-node storage and lookup, and tree/learning/edge-specialized wrappers |
| `modular/encoding/ma_common.h`, `dec_ma.h`, `.cc`, `enc_ma.h`, `.cc` | `modular/encoding/ma_common.rs`, `modular/encoding/dec_ma.rs`, `modular/encoding/enc_ma.rs` | Tree constants/storage, complete entropy-backed tree parsing and validation, BFS tokenization, sample storage/deduplication/quantization, pixel sampling, and original scalar best-split learning |
| `modular/encoding/encoding.h`, `.cc` | `modular/encoding/encoding.rs` | Complete header/channel/modular image decoding, static filtering/flattening, LUTs, local/global trees and entropy tables, dimension validation, transform application, and original truncation/cleanup/finalization behavior; frame orchestration remains separate |
| `modular/encoding/enc_encoding.h`, `.cc` | `modular/encoding/enc_encoding.rs` | Complete standalone generic modular encoder, learned/predefined trees, given-tree ModularCompress and channel residual encoding, original fast paths, overflow checks, and predictor-image scratch writes; frame orchestration remains separate |
| `base/random.h` | `base/random.rs` | Original xorshift state, integer/float/Bernoulli/geometric distributions, and forward shuffle; source argument preconditions retained |
| `modular/modular_image.h`, `.cc` | `modular/modular_image.rs` | Channels, image storage, explicit cloning, empty detection, resize failure state, and reverse transform-stack execution with retained failures and the caller's pool; diagnostics unported |
| `modular/transform/transform.h`, `.cc`, `squeeze_params.h`, `.cc` | `modular/transform/transform.rs`, `modular/transform/squeeze_params.rs` | Transform/squeeze field storage and serialization, PixelAdd, channel-range validation, and complete MetaApply/Inverse dispatch to scalar null-pool kernels |
| `modular/transform/rct.h`, `.cc`, `enc_rct.h`, `.cc` | `modular/transform/rct.rs`, `modular/transform/enc_rct.rs` | Existing scalar forward/inverse color transforms, both forward call forms, and whole-channel permutations; inverse rows run on the caller's pool; SIMD unported |
| `modular/transform/palette.h`, `.cc`, `enc_palette.h`, `.cc` | `modular/transform/palette.rs`, `modular/transform/enc_palette.rs` | Explicit/implicit/delta lookup, metadata edits, inverse prediction, lossless lookup/set selection, ordered palettes, and original lossy iterations, delta learning, and dithering in defined source configurations |
| `modular/transform/enc_transform.h`, `.cc` | `modular/transform/enc_transform.rs` | Complete forward transform dispatch and optional-output min/max scan in the null-pool configuration |
| Reference libc++ non-arithmetic `std::sort` specialization | `base/libcpp_sort.rs` | Original pivot selection, partitions, insertion loops, equivalent-element order, and heap fallback; source version/hashes retained in `tools/reference/libcxx` |
| Reference libc++ `std::nth_element` for float ranges | `base/libcpp_nth_element.rs` | Original guarded partitions, equal-element partition, branchless `__sort3`, selection sort and sortedness shortcuts; source version/hashes retained in `tools/reference/libcxx` |
| `modular/transform/squeeze.h`, `.cc`, `enc_squeeze.h`, `.cc` | `modular/transform/squeeze.rs`, `modular/transform/enc_squeeze.rs` | Original scalar forward/inverse kernels, tendency, default schedules, validation, metadata edits, task spans on the caller's pool, and channel replacement; SIMD unported |
| `base/matrix_ops.h` | `base/matrix_ops.rs` | Float/double matrix products, vector products, and in-place inverse for disjoint input/output callers |
| `cms/opsin_params.h` | `cms/opsin_params.rs` | Original float constants, constexpr scaling, and A2B cube |
| `cms/transfer_functions.h`, `cms/tone_mapping.h` | `cms/transfer_functions.rs`, `cms/tone_mapping.rs` | Scalar HLG/PQ, Rec. 2408, HLG OOTF, and gamut mapping; SIMD variants unported |
| `cms/transfer_functions-inl.h`, `cms/tone_mapping-inl.h` | `cms/transfer_functions_scalar.rs`, `cms/tone_mapping_scalar.rs` | Complete original one-lane TF_PQ, TF_HLG, TF_709 and TF_SRGB input/output algorithms, TF_709 double encoding overload, and derived Rec2408ToneMapper/HlgOOTF/GamutMap, including scene-light factories and original base constructors; alternate SIMD targets remain separate |
| `icc_codec_common.h`, `.cc` | `icc_codec_common.rs` | Complete tag/command tables, endian access, unsigned bounds checks, initial/adaptive header prediction, linear byte prediction and ICC ANS contexts |
| `icc_codec.h`, `.cc` | `icc_codec.rs` | Complete shuffle, bounded base-128 varint, preamble validation, ICC command decoding and incremental ICCReader Init/Process/Reset/CheckEOI with exact checkpoint, replay and defined partial state; whole-file ICC admission remains separate |
| `cms/jxl_cms_internal.h`, used `base/common.h` helpers | `cms/icc*.rs`, `base/float.rs` | Complete profile generation with default 3D tone mapping, matrices, curves, tags, descriptions, and source MD5; default CMS backend acceptance is recorded below |
| Highway `hwy/cache_control.h` | `base/cache_control.rs` | Clang reference enablement: T0 on x86_64 with SSE2; source-disabled hint on other targets |
| `enc_fields.cc` | `enc_fields.rs` | Primitive/bundle writes, size/image metadata wrappers, complete codestream-header output, frame header writing, and quantizer parameter writing |
| `decode.cc` signature/container-header helpers, constants in `lib/include/jxl/decode.h` | `decode.rs`, signature bytes in `common.rs` | Complete OutOfBounds, InitialBasicInfoSizeHint, ReadSignature, JxlSignatureCheck and ParseBoxHeader; public incremental decoder state/events and C ABI entry points remain separate |
| `JXL_EXPORT` functions of `decode.cc` declared in `lib/include/jxl/decode.h` | `decode_abi.rs` | Unmangled C ABI exports of all decode.h functions, with layout assertions for every public struct |
| `JXL_THREADS_EXPORT` functions of `lib/threads/thread_parallel_runner.cc` and `resizable_parallel_runner.cc` | `threads/abi.rs` | Unmangled C ABI exports of all thread_parallel_runner.h and resizable_parallel_runner.h functions |
| `decode_to_jpeg.h`, `.cc` | `decode_to_jpeg.rs`, tag constants in `jpeg/jpeg_data.rs` | Complete JxlToJpegDecoder: output buffer set/release, Reset/StartBox, jbrd Process buffering and decoding, Exif/XMP marker counts, box sizes and payload injection, SetImageBundleJpegData and chunked WriteOutput; JxlDecoder events and JxlDecoderSetJPEGBuffer integration remain separate |
| `icc_codec_common.h`, `.cc` | `icc_codec_common.rs` | Complete source common helpers and command tables from main |
| `icc_codec.cc`, through `UnpredictICC` | `icc_codec.rs` | Complete command decoding and defined partial output from main; streaming reader/CMS remain separate |
| `headers.h`, `.cc` | `headers.rs` | Size, preview, animation, and size-header read wrapper |
| `base/status.h` | `base/status.rs` | Status-code/Result mapping; diagnostics and crash instrumentation unported |
| `padded_bytes.h` | `padded_bytes.rs` | Writer and ICC storage through the owner's memory manager, native slice/length access, and transferred output ownership; other container operations unported |
| `memory_manager.h`, `memory_manager_internal.h`, `.cc` | `memory_manager.rs`, `memory_manager_internal.rs` | Complete JXL manager initialization/default callbacks, output-stage callback-backed scratch, and manager-backed AlignedMemory for every decoder owner, preserving exact custom sizes, live manager fields, the shared alignment counter and the source release order; MemoryManagerUniquePtr/MemoryManagerDeleteHelper for the public encoder's object, settings and queued owners; other encoder owners use the native allocator |
| `enc_aux_out.h`, `.cc` | `enc_aux_out.rs` | Complete statistics storage/defaults, layer/object assimilation, totals, and writer charging; formatted diagnostic output unported |

Internal storage mappings are specified in the
[representation ADR](adr/0003-rust-representation.md). This table counts
translated boundaries, not whole-file or whole-library completion.

## Evidence

From 2026-11-03, the checks run the Rust tests once per oracle graph in the
optimized `checked` profile, with overflow checks and debug assertions enabled,
and compile all targets with the minimum Rust version, now 1.95. Results below that name Debug, Release and Rust 1.85 test runs record the
configurations and minimum version in use at that time. Current PR and deep
validation cadence is governed by [Ferrisma ADR-0027](../../../docs/adr/0027-tiered-validation-and-rust-test-orchestration.md);
these historical results do not establish acceptance of a later revision.

Validated on 2026-11-02 on macOS arm64 with Rust 1.99.0 (stable), Rust 1.85.1
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
| Source revision, Highway/Brotli/skcms pins, notices, local doc links | PASSED | Checked against `upstream.json` and the Git submodule entries |
| Rust formatting and Clippy | PASSED | All targets and features, warnings treated as errors |
| Rust debug | PASSED | 425 tests, including 246 differential test groups; zero ignored cases |
| Rust release | PASSED | Same 425 tests and corpus |
| Rust 1.85.1 | PASSED | Same 425 tests and corpus |
| C++ ASan and UBSan | PASSED | Fail-fast instrumentation during all three Rust test configurations |
| Ordinary C++ reference | PASSED | The same 425 tests and 868,587 requests in debug, release, and Rust 1.85.1 |
| Native x86_64 Linux full baseline | PASSED | Hosted baseline run (retired libjxl-rs repository, run 41332171725), job 124924385248: ordinary and ASan/UBSan reference graphs, each in debug, release, and Rust 1.85; all six blocks ran 344 tests and 744,124 requests with zero failed/ignored cases |
| Native macOS hosted baseline | PASSED | The same run, job 124924385015: instrumented reference graph in debug, release, and Rust 1.85; this is evidence for the prior 344-test tree, separate from the alpha extension below |

Each differential run compares **868,587 requests** against C++. These include
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
The scalar coefficient libraries, EnsureComputed, AC table decoding, and actual
matrix accessors now have separate evidence below.

AC quantization adds 1,486 differential requests across five parity tests:
36 compare all 17 original library entries, every one of the 27 strategy layouts,
zero masks, repeated computation, and shared-kind masks; 80 cover custom scalar
modes, representative band counts up to seventeen, rectangular dimensions, RAW table
lengths/values, reciprocal limits, and failures after earlier table work; 784
compare unaligned/truncated table parsing, invalid mode dimensions, binary16
errors, consumed bits, close status, and retained partial encoding/mask state;
576 decode RAW table streams generated by the original encoder, including its
actual shared-tree frame context, rectangular tables, nonpositive samples,
truncation, and partial table copies. Another 96 original-encoder source
requests generate those fixtures and are counted separately from comparisons.
Every matrix/inverse coefficient is compared as an exact float word; DC inverse
slots retain the original layout-dependent zeroing. Ten additional requests
compare distinct signed quiet/signaling NaN seeds and multipliers in single-band
DCT4/DCT4X8 divisions and direct AFV slots. Their geometry stays finite and AFV
interpolation bands stay finite; no NaN enters FastLog2f.

Standalone AC validation on the prior 344-test baseline runs 349 tests, including
200 parity test groups with 745,610 exact C++/Rust comparisons per configuration.
Native macOS ordinary Debug/Release and complete ASan/UBSan Debug/Release/Rust
1.85 graphs pass. This standalone evidence covers the AC matrix slice on that
baseline. It does not establish native Linux execution or complete VarDCT
decoding. AC quantization encoding remains unported in Rust; original C++
encoding is fixture infrastructure only.

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
the encoder and runner C ABI, decoder C ABI compatibility beyond the measured
64-bit Linux and macOS targets,
thread runners, SIMD parity, upstream test inventories beyond `decode_test.cc`,
the external conformance corpus beyond macOS ARM64 (see
[CONFORMANCE.md](CONFORMANCE.md)), fuzzing, Rust sanitizer/Miri results, or
performance/memory parity.
Test passing here does not establish any of those. Native Linux/macOS hosted
results for the prior integrated tree are recorded above; new port batches
require their own current-head CI evidence.

## Next dependency boundaries

The [first-image milestone](FIRST_IMAGE.md) is verified for its fixed original
configuration. The component slices alone are distinct from that whole-file
evidence. Subsequent full-port boundaries include connecting the low-memory
pipeline to the public decoder API, remaining modular and VarDCT paths, other image/frame configurations,
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

Retained frame/DC ownership adds 6,240 original differential requests: 2,016
ImageBundle move-constructor/assignment and RemoveColor cases plus 4,224 actual
public write-stage cases. They observe destination identity, transferred pixel
allocation identity, moved-from scalar/plane headers including restoration to
the original allocated dimensions, name/ICC/vector state,
valid source color aliases while the destination lives, and JPEG/extra-plane
size fallback. Writer cases cover every DC/reference slot, enclosing owner
moves, later target allocation replacement, row and size guards, partial writes,
empty dimensions and exact exceptional float words. Four native regressions
exercise decoder-owned pipelines across owner moves, live indexed DC access,
existing reference Box identity, and the reference temporary's final allocation
transfer. These component checks do not admit additional complete-frame modes.

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
are unchanged. A second native regression uses captured competing
payload words and verifies untouched rows, extra spans, and ignored channels.
Both complete ordinary and ASan/UBSan x86 original oracles in hosted
run 41331075671, job 124921109245 (retired libjxl-rs repository)
produce the unchanged ARM nine-word table. The invalid earlier QEMU-specific
x86 alternate was removed using those original captures. The input stencil is
identical despite different row padding.
Both run without the C++ feature. A temporary fused-pixel replacement failed the
original finite regression, proving that it detects separate-rounding changes.
This numeric configuration establishes no SIMD or performance parity.

Focused x86_64 Linux runs under QEMU use Clang 21.1.8/libc++ 21 and Rust 1.99.0.
Debug and release each compared all 2,113 Gaborish requests in four groups
against both an ordinary and a fail-fast UBSan observation executable; both
Rust-only Gaborish regressions passed in both emulated profiles. QEMU selects
the same larger quiet-NaN payload in either operand order, so these results do
not prove native x86 payload parity. Actual hosted x86 execution exposed the
invalid emulated capture in the new native test; the old finite test passed.
Static inspection also found that Clang 21 plain `-O0` normalizes all three
weights reciprocal-first, while ASan/UBSan commutes the first two products and
retains reciprocal-first for the third. Rust release had commuted those
products too. A local binary32 multiply now retains reciprocal-first on x86,
with ARM's original compound assignments unchanged. Both complete original
native oracle captures agree for the existing signaling/quiet normalization
requests: each channel's six written words retain the first filter payload.
The complete native ordinary/instrumented Rust gates now pass in the combined
acceptance run linked above.
The temporary focused
executables retain the actual original fixture observation bodies and dedicated
stage, linked against the complete original ordinary static archives. UBSan
instruments that focused fixture/stage, not those ordinary archive members.
Link maps bind the original factory and ProcessRow to the dedicated stage
object. Emulated ASan is unavailable; it is not counted as a passing gate.
The complete Linux ASan/UBSan graph and MSRV now pass native hosted CI. The
ordinary/sanitized macOS runs remain separate, fully instrumented reference gates.

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

The ordinary and ASan/UBSan macOS checks with `--msrv` each pass all 325 tests
and 703,420 differential requests in Debug, Release, and Rust 1.85 configurations.
The captured regressions also pass in the isolated x86_64 Linux runtime;
the full hosted Linux matrix remains a separate integration gate. CI runs
the unchanged full Linux/macOS matrix on each PR update and on pushes to main,
avoiding duplicate full runs for a feature-branch push followed by its PR.

After those four fixes, native Linux CI passed every existing differential
group and exposed an AArch64-only captured GamutMap word in an ordinary test.
The test now retains independently captured exact words for both reference
targets, with its input also added to the differential corpus. The existing
AArch64 arithmetic and captured words remain unchanged. The x86 production
helper now additionally preserves the actual original object's optimized
luminance and product-first blend ADDSS operands through local out-of-line Add
calls. The same old scalar and stage/pipeline cases retain exact comparisons
in both build modes; no expected word or existing request is changed by that
mapping.

Focused emulated x86_64 ordinary and UBSan-reference runs both pass four filtered
groups and 56,085 requests in Debug and Release, including the original scalar
gamut and stage/pipeline corpora. Both ordinary Rust tone-mapping tests also pass
in each mode. The emulated runtime selects the same larger quiet-NaN payload
for both ADDSS operand orders; these passes do not prove native operand fidelity.
Native hosted execution exposed the earlier tiny source probe's different register
allocation and the need to inspect the actual oracle's GamutMap object. The
focused UBSan variant instruments the unchanged original-source adapter
containing the actual scalar functions/stage and links the cached
ordinary codec archive; this is not a fully instrumented Linux codec run.
Complete ordinary macOS Debug, Release, and Rust 1.85 checks retain the same 325
tests, 183 differential groups, and 703,420 requests. The preceding mapping also
passed all three macOS ASan/UBSan configurations; this correction only compiles
on x86_64 and leaves the AArch64 arithmetic unchanged. Existing cases and
expected words remain unchanged; no emulated ASan gate is claimed.

## Complete alpha image profile

The optional alpha extension preserves the initial first-image milestone as
historical evidence and adds a separate complete configuration. The
`new_with_alpha` constructor owns one full-resolution, unassociated alpha
channel with the same 8/16-bit depth as color. The original no-alpha constructor
and all its corpus remain unchanged. See [FIRST_IMAGE.md](FIRST_IMAGE.md#alpha-profile-extension)
for the source branches, bounded admission, and complete-file corpus.

The reached encoder branches allocate alpha before color import, copy the
interleaved last sample after RGB/gray import, skip that first alpha in
CopyExtraChannels, and append its integer modular channel before the original
palette/compaction selection. The original default keep-invisible condition
prevents SimplifyInvisible in lossless mode. Decoder frame admission permits
only matching channel counts, upsampling one, and replacement blending; the
simple pipeline allocates its fourth channel and existing modular/image-bundle
stages carry it through to gray/alpha or RGBA integer output. Non-admitted alpha
configurations remain explicit native admission results, not generic codec
status replacements.

The added parity corpus contains 400 complete encoder/statistics/cross-decoder
requests, 400 complete-file decoder observations, 606 prefix/payload/trailing
error observations, 216 short input-buffer observations, and 16 separate-alpha
buffer observations through the original complete encoder: 1,638 additional
exact C++/Rust requests. Ordinary Rust tests cover complete alpha image paths,
every uint16 alpha value, invisible RGB, and rejection of associated,
subsampled, mismatched-depth, non-alpha, and multiple-extra metadata.

Both local native macOS arm64 commands, `python3 tools/check.py --msrv` and
`python3 tools/check.py --sanitizers --msrv`, pass with the complete alpha corpus:
351 tests, 199 differential groups, and 745,762 exact requests in each debug,
release, and Rust 1.85 configuration; zero failed or ignored cases. The earlier
hosted Linux/macOS results above belong to the baseline tree. The hosted
validation for this alpha extension is recorded in the integrated checkpoint
below.

## Integrated alpha and AC matrix checkpoint

The combined lossless alpha and scalar AC matrix tree passes both complete
local native ARM macOS commands, `python3 tools/check.py --msrv` and
`python3 tools/check.py --sanitizers --msrv`. Each debug, release, and Rust 1.85
configuration runs 356 tests, including 204 differential groups and 747,248
exact C++/Rust requests, with zero failed or ignored cases. All earlier corpora
remain enabled. The source maps and representation records above include both
slices; AC encoding and general VarDCT frame decoding remain unported. Native
Linux/macOS checks are required for this combined PR tree before integration.

The separate alpha commit also passed the native hosted matrix (retired libjxl-rs repository, run 41343768003):
Linux ordinary and ASan/UBSan reference graphs each ran debug, release, and
Rust 1.85; macOS ran the instrumented graph in those three configurations.
Every block passed 351 tests and 745,762 requests with zero failed or ignored
cases. That hosted evidence belongs to the alpha tree, independently of this
combined checkpoint.

## Scalar DCT and inverse-transform dependencies

The scalar DCT/IDCT recursion, literal scale tables, maybe-transposed coefficient
layout, mutable coefficients, and original scratch scheduling are translated
for power-of-two sizes 1 through 256. TransformToPixels covers all 27 strategies,
including identity, hierarchical 2x2, split 4x4/4x8, all AFV orientations, and the
large DCTs. LowestFrequenciesFromDC retains its sparse writes and warm-scratch
branch; its required forward DCT is included. AcStrategy geometry and natural
coefficient orders/LUTs use the original tables and traversal.

Seven differential groups initially added 2,905 requests against the pristine
original methods, comparing all initialized coefficient, padded-output, and scratch
words. Coverage includes square/rectangular shapes, strides, DC and impulse
inputs, deterministic finite samples, signed zeros, subnormals, overflow,
infinities, signaling/quiet NaNs, and 342 competing-payload requests. Another
192 requests retain identity subnormal/cancellation/signed-zero rounding. Two
native AArch64 tests use independently captured original words for identity
contraction, IDCT/AFV operand order, and length-one signaling-NaN copies.

The native macOS ordinary reference passes Debug and Release with 353 tests,
202 differential test groups, and 747,029 exact requests per configuration.
The complete ASan/UBSan reference passes the same corpus with Debug, Release,
and Rust 1.85. No cases are failed or ignored. The new dct.cc observation unit
retains its source loops with scoped -fno-unroll-loops while keeping default
contraction, O3, and all sanitizers; ADRs 0002/0003 record the actual-object
numeric evidence. Both complete original graphs agree on all 2,905 focused
requests, and every finite-input observation matches the saved default ordinary
adapter. Existing fixtures and the original archive retain their flags.

This closes transform dependency boundaries, not general VarDCT group/frame
decoding or encoding. SIMD, other native target evidence, full-codec conformance,
and performance remain separate gates.

## Integrated alpha, AC, and transform checkpoint

Both complete local native ARM macOS commands, `python3 tools/check.py --msrv`
and `python3 tools/check.py --sanitizers --msrv`, pass with all three slices
combined. Every debug, release, and Rust 1.85 configuration runs 365 tests,
including 211 differential groups and 750,177 exact C++/Rust requests, with
zero failed or ignored cases. Existing corpora and expected words remain intact.
The captured-word unit tests are target-specific: two run on each of AArch64
and x86_64; every target runs the same complete differential corpus.
Integration PR #7 (retired libjxl-rs repository)
records the native hosted matrix for this combined tree.

The new scalar dependencies do not expand the admitted complete image profile.
General VarDCT group/frame orchestration, AC quantization encoding, SIMD,
full-codec conformance, and performance remain separate work.

The native Linux refinement retains the actual original IDCT addend-first
addition and AFV coefficient-1-first DC sum and accumulator-first basis sum.
Both complete original Linux graphs agree on the 24 formerly mismatching AFV
requests. Another 24 competing-basis requests exercise two even-even AFV
coefficients in all four orientations, increasing the DCT slice to 2,929
requests and its mixed-payload group to 366. No previous request or expected
word is removed or changed; the selected ARM operations remain identical.

## AC coefficient-order decoding

DecodeCoeffOrders now selects the first strategy for each order bucket, maps
used strategy bits to those buckets, fills natural defaults, and reads custom
permutations through the original histogram/symbol machinery. Encoded orders
that are unused by the caller still consume entropy with an absent output. A
nonzero mask initializes and validates the reader even when only reserved bits
are set. With a zero mask no bits are consumed. Failures retain the original
partial packed output; unused ranges and trailing guards remain unchanged.

New fixtures use genuine original EncodeCoeffOrders streams, including custom
LLF-preserving tails, unaligned input, all strategy aliases and 13 order buckets,
unused custom orders, reserved masks, truncation, corruption, and reused output.
The full packed output is compared losslessly as a sentinel plus every changed
index/value, avoiding hashes while bounding peak fixture memory. This slice
provides the order decoder for later AC/frame orchestration; it does not add a
complete VarDCT frame decoder or port the coefficient-order encoder dispatcher.

The coefficient-order slice adds four native tests and three differential groups
with 697 exact requests. Local full ordinary and ASan/UBSan validation, each in
Debug, Release, and Rust 1.85, runs 372 tests, 214 differential groups, and
750,874 requests with zero failed/ignored tests. A captured original custom-order
stream also exercises the entropy decoder in ordinary Rust-only tests.

## Quantizer parameters and scalar APIs

The parameter slice adds 6,760 exact requests in six differential groups and
seven native regressions. It covers the original U32 field distributions, defaults,
write/accounting order, unaligned and truncated parsing, failure before owner
assignment, GetParams, C-long scale rounding and signed integer narrowing,
ClampVal, and forwarding of all
strategy/channel matrix slices. Existing constructor/recomputation observations
and numeric flags remain unchanged. Signaling NaNs remain in defined arithmetic
paths; the demonstrated ClampVal float-to-int source boundary is recorded in
[ADR 0003](adr/0003-rust-representation.md). Quantization-field selection,
SetQuant/ComputeGlobalScaleAndQuant, and diagnostic dumps remain unported.

Local full quantizer validation runs both `python3 tools/check.py --msrv` and
`python3 tools/check.py --sanitizers --msrv`: each Debug, Release, and Rust 1.85
block runs 385 tests, including 220 differential groups and 757,634 exact
requests, with zero failed or ignored tests. Native hosted evidence for earlier
trees remains recorded separately above.

## VarDCT block contexts

The block-context slice adds six native tests and six differential groups with
98,233 exact requests: normalized density domains and both tables, bucket
arithmetic, threshold/order/channel lookup, predictor edges, original context
map encoding/accounting, and parsing into fresh/reused owners. Fixtures include
64 accepted versus 65 rejected threshold partitions, 15-entry threshold vectors,
truncation, malformed real-stream mutations, and narrowed U32 offsets. The
ordinary and fully instrumented original use their genuine entropy codec.

Default parsing replaces the entire owner. Nondefault parsing preserves the
original threshold mutations before size rejection, map resizing/partial
decoding, and final context-count validation. Source index and signed-predictor
preconditions are recorded in [ADR 0003](adr/0003-rust-representation.md). This
provides contexts for subsequent AC-group symbol decoding; complete VarDCT
frame/group orchestration remains unported.

Full local validation of the integrated block-context tree runs 397 tests,
226 differential groups, and 855,867 exact requests in each Debug, Release,
and Rust 1.85 configuration against ordinary and ASan/UBSan originals.
No test is failed or ignored.

## Quantized AC block symbols

DecodeACVarBlock now reads each pass through the existing real ANS/prefix/LZ77
reader, predicts/stores nonzero counts, selects block/density contexts, scatters
through the caller's coefficient order, and adds signed shifted coefficients to
either i16 or i32 storage. Invalid counts fail at the original point and retain
all earlier row/coefficient/reader mutations. Group scheduling, dequantization,
and pixels remain separate unported boundaries.

Three native tests and three differential groups add 534 exact block-decoding
comparisons. Fixtures call a hash-checked verbatim extraction of the original
template, with genuine BuildAndEncodeHistograms/WriteTokens streams in both
ANS/prefix and LZ77 modes. All 27 strategies, both coefficient widths, natural
and reversed orders, predictor edges, channels, multipass accumulation, shifts,
count errors, and truncations compare complete coefficient/row owners and
reader checkpoints. Real varied prefix tokens force actual LZ77 activation,
which is asserted and consumed through the same reader. No mock symbol reader
or C++ runtime fallback is used.

The DC, AC metadata, scheduling and rendering dependencies described by this
checkpoint are now connected in the [first complete VarDCT profile](VARDCT_IMAGE.md).
The previous four-slice validation below remains historical evidence for its own
tree. General frames, EPF and effects remain outside the admitted profile.

Full local validation of all four integrated slices runs 403 tests, including
229 differential groups and 856,401 exact C++/Rust requests per configuration.
Both `python3 tools/check.py --msrv` and
`python3 tools/check.py --sanitizers --msrv` pass in Debug, Release, and Rust 1.85
with zero failed or ignored tests. All prior requests and expected words remain.

The highest default block-context regression uses strategy 19, channel 2, and
context 14. A nonzero penultimate symbol leaves the final symbol at density 473;
fixture assertions prove the final two lookups select NumACContexts+14/+15,
including the last original ProcessACGlobal padding slot. Both genuine ANS and
prefix streams cover this partial-failure path; the previous strategy 0/18
requests and their encoded inputs remain unchanged.

## VarDCT DC and AC metadata dependencies

The DC slice translates DecodeVarDCTDC and DecodeAcMetadata with real original
modular streams, DequantDC, AdaptiveDCSmoothing, ComputeSigma, AC strategy
setters/interpreted rows, and the reached signed conversion helper. It retains
DC group offsets, CfL/quant/strategy/sharpness mutations on failure, reused
owners, source geometry rejection, and used_acs assignment checkpoints.

Four differential groups add 1,147 exact requests, including all 27 strategy
setters, real DC/AC metadata streams, truncation, malformed overlap/geometry,
multiple DC-group offsets, smoothing borders/subsampling and exceptional float
words. One native AArch64 regression retains independently captured original
smoothing words. Full frame/group integration remains a separate boundary.

The full ordinary and ASan/UBSan checks, each with Debug, Release and Rust 1.85,
run 408 native ARM64 tests, 233 differential groups, and 857,548 requests per
configuration. The ARM64-only captured regression makes the native x86_64 count
407 tests with the same differential corpus.
The numerical reference configuration is recorded in ADRs 0002 and 0003;
complete hosted native-platform evidence belongs to this slice's PR.

## First complete VarDCT image checkpoint

The [VarDCT milestone](VARDCT_IMAGE.md) connects original DC/AC setup,
quantization matrices, packed orders and histograms, group caches, per-pass
symbol readers, coefficient accumulation, dequantization, inverse transforms,
optional Gaborish, XYB and sRGB output. Source HIGH_PRECISION=1 selects TF_SRGB's
rational approximation; the separate fast helper does not replace it.

The full tree adds 22 tests, including 17 differential groups and 12,186 exact
requests over the preceding 403-test checkpoint. New whole-file observations
cover 141 original files, every initialized sRGB float word and exported integer
sample, 1,624 truncation/payload/trailing-error observations and both sRGB
helpers. Actual progressive pass counts and filter flags are asserted on the
original files. Alpha, odd AC-group dimensions in both axes and multiple DC
groups exercise deferred full-image pipeline storage. Four frozen source-derived
fixture files let ordinary Rust tests decode captured original files without
C++ at runtime. Full validation commands and platform results are recorded in
this milestone and the integration PRs.

Native x86_64 Linux validation exposed ordinary/ASan operand-order drift in the
isolated DequantLane observer. ADR 0002 records the scoped reference configuration
and independent default-optimization audit. One captured native x86 regression
complements the ARM-only capture; both targets now run 425 tests, the same 246
differential groups and 868,587 requests. The corrected implementation passes
the combined native gates linked above and is merged; earlier failed runs remain
in the PR history.

## Decoder effects integration

The expanded VarDCT raw-file profile connects all three scalar EPF stages,
noise parsing/input generation/convolution/application, and spline parsing/cache
construction/rendering. It retains the existing gray/RGB, 8/16-bit,
full-resolution unassociated alpha, progressive AC and group configurations.
Patch components and shared reference-owner binding are translated; complete-file
saved-reference/frame orchestration is connected by the sequence boundary below. See
[DECODER_EXPANSION.md](DECODER_EXPANSION.md) for the full remaining decoder scope.

Successful original-file comparisons cover 114 EPF files, 140 noise files and
51 spline files with combined EPF/Gaborish/noise/progressive/alpha features,
including AC/DC-group edges. Another 4,582 exact prefix/payload/trailing
observations cover EPF and combined effects failure paths. These five new
whole-file groups contribute 4,887 requests. Both captured effects files in
`tests/data` decode independently of a C++ executable and retain all original
integer samples and color float words. Source mappings and selected numeric
reference configurations remain in the living ADRs. Both full local commands
(`tools/check.py --msrv` and `tools/check.py --sanitizers --msrv`) passed all
six configurations, each with 462 tests, 272 differential groups and 893,729
exact requests, with zero failed or ignored tests. The combined native run linked
above now verifies this expanded image boundary, which is merged into `main`.

## Scalar patches, blending, and alpha

The complete patch dictionary now includes entropy decoding, reference checks,
partial failure state, interval-tree construction, row lookup, reference masks,
and clipped blending/rendering. PatchInterval sorting uses the already translated
libc++ non-arithmetic sort, preserving tied-element ordering in tree arrays.
The scalar alpha and blending dependencies and both borrowed/shared patch stages
are translated too. Complete-file reference-frame orchestration is connected by
the sequence boundary below.

Eight differential groups add 5,526 exact original requests. The corpus includes
real entropy streams, truncation/mutation/reuse, complete initialized tree arrays,
row/stage rendering, every blending mode, alpha aliases and exceptional floats.
Ninety padded-coordinate stage cases preserve size_t wrapping and clipping.
Fifty-six tied/nested/reversed interval cases cross 24- and 128-element sort
thresholds through 513 intervals. Persistent-stage cases verify that subsequent
dictionary and reference replacement remain visible. Two native tests retain
independently captured original streams and blending samples.

The alpha reference audit compares 1,944 actual fixture requests between selected
ordinary and fully ASan/UBSan graphs and checks all 1,632 finite requests against
both untouched default graphs. All six actual public alpha symbols bind to the
pristine selected unit, including complete-file calls. That unit alone uses O0
and no-inline configuration, with strict FP additionally on x86_64 Linux.
ARM retains source contraction; strict FP there changes finite words. The
native source experiment (retired libjxl-rs repository, run 41446120185)
records the accepted x86 strict-O0 configuration and rejected candidates. Both
initial local ARM gates passed Debug, Release and Rust 1.85 but included the
48 subsequently identified undefined tree observations; their apparent ARM
success does not establish pixel parity. The corrected multiply operation keeps
the source's background-first product through the existing binary32 call
boundary, retaining all old exceptional requests. All six corrected local ARM
focused configurations (ordinary/ASan × Debug/Release/Rust 1.85) pass 11 parity
tests with 5,478 exact defined requests, all 72 original-only probes and all
three standalone regressions. The local independent census executes all 1,728
tree requests in both graphs with no diagnostics and exact defined observations;
it explicitly leaves native Linux confirmation pending. Both full corrected
local gates also passed: all six ordinary/ASan/UBSan Debug, Release and Rust 1.85
configurations completed 439 tests, 257 parity tests, 874,065 exact defined
requests and all 72 original-only probes, with no failures or ignored tests.
The native census, final native Rust CI and main integration are now complete
in the combined acceptance run linked above.
No original source or sanitizer changes, unsafe operation, assembly,
relaxed float comparison or changed old defined case is used. The README patch
checkbox is checked after the complete-file reference orchestration and native
acceptance were merged into `main`.

## Scalar spline decoding and rendering

The complete scalar spline boundary adds four differential groups and 6,253
exact original requests: 259 quantize/dequantize cases, 129 curve/cache/render
cases, 5,862 genuine original entropy-stream cases, and three repeated-decode
sequences. The cache corpus asserts that every color and sigma coefficient
position survives quantization. It compares the actual stage on extended rows,
including unsigned wrapped left coordinates, as well as cache index order and
all initialized pixels. Entropy cases cover starting-point and control-point
limits, coefficient limits, truncation, damaged streams and partial state.

Four standalone tests retain an independently captured original stream, its
duplicate-point guard, disjoint three-row storage access and overlap rejection.
The ordinary and fully ASan/UBSan original scalar objects retain their existing
default optimized compilation; no new numeric compiler option or sanitizer
suppression is introduced. Ownership and source-defined input limits are recorded
in ADR 0003. Complete-file feature admission and cross-feature image evidence
remain a separate integration gate, so the README spline checkbox stays open.

## Scalar EPF filtering

The complete scalar EPF stage boundary adds six differential groups and 7,092
exact original requests: 12 factory observations, 144 original simple-pipeline
sequences, 3,672 mixed/exceptional rows, 1,728 active finite rows, 288 signed-zero,
subnormal and cancellation rows, and 1,248 independently varied filter-parameter
rows. Two native regressions retain independently captured original finite pixels,
live shared sigma updates, competing NaN payloads, and unquieted skipped pixels.

The selected ordinary and fully ASan/UBSan original graphs agree on all 6,936
row observations. The durable `tools/epf_reference.py` audit additionally requires
all 4,152 finite-arithmetic rows to match both untouched default optimized archive
objects and verifies real factory bindings. The new isolated pristine stage
observation unit alone uses scoped `-O0`; ADRs 0002 and 0003 record the demonstrated
original compiler-dependent NaN operand ambiguity and ownership mapping. No old
cases, expected words, source pins, sanitizer flags, or original archive options
change. Complete-file EPF admission and image evidence are a separate integration
milestone, so the README's EPF checkbox remains open here.

## Sequential decoder lifecycle integration

The internal `SequenceDecoder` now connects one shared decoder state across
regular and reference-only frames, repeated reference-slot saves, cropped
blending, animation, previews, progressive-DC dependencies, and NONE/XYB Modular
and VarDCT frames. It retains source InitFrame/InitFrameOutput order, physical
section traversal and reader closes, reference masks and slot identity, DC
selection, finalization and decoded-byte checkpoints. Main and early/late
extra-channel spatial upsampling run in original PreparePipeline order.
The earlier fixed complete-file entry points keep their existing admission.

The frozen manifest contains 113 genuine original encoder requests. Two of
those document actual unsupported higher-level DC encoder requests; no invented
DC codestream substitutes for them. The lifecycle corpus keeps all 1,110 prior
whole-file requests and adds 666 later-frame truncation/mutation requests, plus
18 additional original-header/TOC-writer section-size corruptions and
20 direct original shared-state observations spanning DC levels 0 through 4,
missing/existing slots and use-DC selection. The 1,794 whole-file cases include
754 errors with initialized saved reference pixels, 10 with initialized DC
pixels, and 476 with initialized output pixels. Two additional close failures
retain all written output and uncommitted reference-storage pixels before the
source's reference move. Their zero-change controls reproduce genuine encoder
bytes exactly, and all 18 writer-produced cases remain in the corpus. Source slot identities and masks, preview-last/main-EOF behavior,
metadata, every initialized retained sample, partial errors and reader/section
checkpoints are compared exactly. Allocation-generation and row-span tracing
prevents inspection of newly allocated unwritten source DC samples; ADR 0003
records the observer's null-runner limit.

The expanded corpus retains all 1,814 requests above and the original
113-request manifest byte for byte. A separate 65-request encoder manifest
adds 322 resampling, 234 deliberate multigroup, and 66 higher-DC whole-file
observations. Actual encoder controls cover main and extra-channel factors
1, 2, 4, and 8; the observer checks both ParamsPostInit readback and decoded
header factors. The source normalizes an extra factor below the main factor
upward inside EncodeFrame. Twelve early-extra, 18 late-extra, and six main-only
upsampling fixtures succeed. Six odd-sized Modular early-extra files genuinely
encode but the original decoder rejects them; aligned positive controls and
all six encoder-rejected unsupported-factor requests remain explicit.
Multigroup sequences use 257x3, 3x257, and 513x1 dimensions, parsed two/three-group
headers, reference-only saves, reference reuse, animation, and optional previews.

The encoder still rejects explicit DC levels 3 and 4. Separate source-writer
fixtures parse genuine 1x1 progressive-DC2 frames and rewrite only dc_level
through WriteFrameHeader/WriteToc. They reuse each genuine payload unchanged,
verify all 18 FrameDimensions fields, reparse emitted headers/TOCs, restore the
old level to reproduce the original frame bytes, and retain the exact-byte
DC2 control. Both original graphs accept the resulting DC3/DC4 dependency
chains; this is decoder acceptance, not an encoder capability claim.

One additional opt-in Squirrel/patches-on encoder request produces a frozen
112-byte Modular/NONE codestream from repeated glyph pixels. Its automatic
reference-only save to slot 3 precedes a main frame with FLAG_PATCHES, 24 patch
positions, two source rectangles, and reference mask 8. All 36,864 initialized
output words equal the 0/1 glyph inputs. The original FrameHeader/TOC parsers
locate its internal frame boundaries; 156 complete, truncated, and mutated
observations retain exact dictionary, mask, identity, output, and storage
snapshots. Of its 137 errors, 133 retain initialized reference pixels, 39 retain
partial patch positions, and 20 retain initialized output pixels. Reserved
frame flags remain accepted as in the original; the byte-40 flags=4 mutation
reaches output initialization before the later section error.

The resulting focused inventory contains seven groups and 2,592 differential
requests, including the unchanged 20 direct state observations. The independent
source audit compares all 2,592 requests between ordinary and ASan/UBSan graphs
and verifies the actual public writer factory bindings. The extension adds
241 errors retaining reference pixels, 45 retaining initialized DC pixels,
and 153 retaining initialized output pixels. These focused results do not
replace the required complete repository gates.

The connected YCbCr/chroma extension is described below. This internal lifecycle
boundary still excludes ICC/default CMS parsing and arbitrary
external layouts, JPEG reconstruction and the public progressive C API. Its
focused checks do not complete the general decoder milestone or establish the
remaining complete-codec acceptance gates.


## Complete-file YCbCr and chroma integration

The shared-state sequence decoder now connects the original scalar YCbCr and
horizontal/vertical chroma stages for sRGB 4:4:4, 4:2:2, 4:2:0 and 4:4:0. Genuine
grayscale JPEG input follows the original YCbCr path with 4:4:4 shifts. Source
stage order, group/channel dimensions, ceil-divided borders and pre-/post-color
reference write checkpoints remain explicit; existing reconstruction and buffer
algorithms are unchanged. ADRs 0002 and 0003 retain the independently verified
platform-specific scalar reference and ownership/numeric mapping.

Twenty-five genuine JPEG-derived raw JXL fixtures cover all five variants at
65x33, 1x1, 9x7, 257x3 and 3x257. The original JPEG parser reports the actual
component sampling; actual public encoder calls disable container and JPEG
metadata storage and reproduce every stored codestream byte. Three new
differential groups compare 25 complete files, 25 typed header/padding/group
observations and 1,498 truncated, payload-mutated and trailing-byte observations.
One Rust-only test decodes all 25 files against every planar binary32 sample
independently captured from matching ordinary and ASan/UBSan original graphs.
No comparison uses pre-JPEG RGB as a substitute for the already quantized image.

The complete-file source audit retains every request/observation, checks genuine
JPEG/wire/pixel fixture hashes, and verifies actual ParseJPG, public encoder,
PreparePipeline and selected chroma factory symbol bindings. The unchanged
5,562-request component audit and all 2,592 prior lifecycle requests, including
20 direct DC-state cases, remain separate required controls. No source algorithm,
old expected word, sanitizer option, global numeric flag, runtime dependency or
production C++ fallback changes. All six focused native ARM configurations pass:
ordinary and ASan/UBSan Debug, Release and Rust 1.85 each complete 27 tests,
including 15 differential groups and 9,702 exact requests, with no failed or
ignored tests. The no-feature 25-file capture test, all-target/all-feature Clippy,
formatting, source/notices, documentation links and all three independent source
audits pass. All six full local configurations also pass: each runs 513 tests,
307 differential groups and 926,392 exact defined requests, plus all 72 explicit
original-only boundary probes, with no failures or ignored tests. This union
includes the ICC command-codec prerequisite. Native Linux/macOS CI now passes
with the counts recorded above, and the union is merged into `main`. General orientation,
ICC/CMS parsing, broader sampling, JPEG reconstruction and the
public progressive decoder API remain open.

## JPEG bitstream writing

The complete pinned WriteJpeg boundary adds five differential groups with 940
exact requests and three independent source-captured native regressions. Six
actual cjpeg files exercise baseline/progressive scans, grayscale and RGB,
subsampling, restart markers, bit stuffing and multiple 16,384-byte bit-writer
chunks. The actual original ReadJpeg supplies their quantization, Huffman,
coefficient, scan and marker data; the writer reproduces each input byte-for-byte.
The [fixture provenance](../tests/data/jpeg-writer/README.md) records the generator,
options, original image data recipe and every file hash.

The corpus retains every callback offer, accepted count, null pointer distinction
and output byte, including short successful and failed writes. It covers all 256
marker values, table and metadata errors, explicit padding and malformed but
source-defined structure inputs. A cold review caught C++ int promotion for
Al16..31 and int16_t compound-assignment narrowing after many extra zero runs;
independent ordinary/instrumented source experiments confirmed both domains.
The corrections preserve those exact requests and the original algorithms.
A reversible prefix representation avoids duplicating large callback offers in
the test protocol; every byte remains present literally or as a previously
observed exact prefix. Earlier independent literal captures reconstruct exactly.

All six focused ordinary/ASan/UBSan Debug, Release and Rust 1.85 configurations
pass the 940 requests and three native regressions. Complete ordinary and
ASan/UBSan checks also pass Debug, Release and Rust 1.85: each configuration
completes 449 tests, 261 differential groups and 882,872 exact requests, with no
failures or ignored tests. Native Linux/macOS CI and integration with the latest
main remain required. No source body, numeric option, dependency or sanitizer coverage
changes. Brotli reconstruction metadata decoding, JPEGData field serialization,
and whole-file coefficient/reconstruction integration remain open; the README
JPEG reconstruction milestone stays unchecked.

Combining this writer with main's verified ICC command decoding also passes all
six complete local configurations: 457 tests, 266 differential groups and
895,119 exact requests each, with no failures or ignored tests. Native CI for
this combined tree remains required.

## ICC profile command decoding

The complete common-helper and UnpredictICC boundary adds five differential
groups and 12,247 exact original requests. These cover every byte-pair context
class, tag/endian access, adaptive header prediction, preceding-history loads,
unsigned overflow checks, all tenth-byte varint forms, nonrectangular shuffles,
every tag/command byte and prediction flag, command errors and initialized output
prefixes. Four sRGB/linear ICC profiles and 45 arbitrary byte buffers pass through
the actual original PredictICC function; their genuine command streams are then
decoded, truncated and mutated against both implementations. Three standalone
regressions retain independent original captures for tenth-byte overflow,
nonrectangular shuffling and partial output with the nonempty-result guard.

The observer compiles pristine icc_codec.cc and calls the original common helpers
and predictor. Source bodies, numeric options, dependencies and sanitizer
coverage remain unchanged. A fresh source review retained the fallible allocated
header checkpoint and the full logical-position/history domain. Focused
comparisons pass all six native ARM ordinary/ASan/UBSan Debug, Release and
Rust 1.85 configurations, including the three standalone regressions.
Complete ordinary and ASan/UBSan checks also pass Debug, Release and Rust 1.85: each configuration runs 449 tests, including 261 differential groups and 894,179 exact requests, with no failures or ignored tests. Native run 41463504601 (retired libjxl-rs repository) also passes: Linux completes all six configurations and macOS completes its three instrumented configurations, each with 449 tests, 261 differential groups and 894,179 exact requests. Streaming ICCReader entropy/checkpoint ownership, ICC parsing/default CMS and whole-file ICC admission remain open; the README color-profile checkbox stays unchecked.


## Output buffers and pixel callbacks

The complete original scalar WriteToOutputStage is available as an internal
pipeline leaf. It retains copied output descriptors, borrowed live manager
identity, buffer strides and sentinels, callback priority, init/run/destroy order,
partial PrepareForThreads failures and retries, all eight orientations, original
unpremultiplication, uint8 dithering, uint16 saturation, endian stores, and raw
half/float output. The 1,536 original dither words are literal source constants.
Callback-returned allocations physically back scratch; custom callbacks receive
exact source sizes and frees without allocator headers or shadow buffers.

Nine differential groups contain 4,004 exact requests. Nine immutable original
captures also run in an ordinary Rust test without C++ linkage. The independent
source audit verifies live public factory and virtual-method bindings, original
memory-manager archive bindings, compiler flags, libc++ identity and actual
reverse vector cleanup. It replays all requests and native captures against
both pristine source graphs without changing active binaries or objects.

Depth 31 has a defined uint32 shift but its float multiplier rounds to 2^31.
Reaching that value triggers the original Highway float-to-int32 undefined
conversion. The audit retains 64 explicit value-one probes and all 192 initial
depth-31 controls: 96 controls synthesize source alpha=1 and positively diagnose
the same conversion, while 96 remain exact in the defined corpus. Another 48
distinct four-channel real-alpha controls from the second extension remain
exact, and 48 new two-channel real-alpha controls complete the defined path.
Another 256 original-only requests positively diagnose oversized shifts at depths 32,
33, 63 and SIZE_MAX. Every source-only diagnostic is retained individually;
no source or sanitizer is changed and no prior defined request is discarded.

The internal sequence decoder still uses its existing bundle/DC writers. This
leaf does not complete the public incremental API, container parsing, direct
public callback registration, or custom allocator propagation through unrelated
codec owners. The pre-geometry union passed all six local configurations:
Debug, Release and Rust 1.85 against ordinary and nonrecovering ASan/UBSan
originals after integration with
the accepted extra-channel implementation. Each runs 540 tests, 327 differential
groups and 1,036,783 exact defined comparisons, with zero failures or ignored
tests. The independent output-stage, extra-channel and retained frame-lifecycle
source audits also pass. Native run 41517918249 (retired libjxl-rs repository)
passes all six configurations and source audits on both Linux and macOS for
this source tree: Linux runs 541 tests per configuration; macOS runs 540.
The geometry union passes all six complete local configurations with 545
tests, 331 differential groups and 1,095,471 exact defined requests each, with
zero failures or ignored tests. The output-stage, lifecycle, extra-channel and
geometry source audits all pass. Fresh native acceptance of this union remains
pending.

## Associated alpha and remaining extra channels in complete sequences

The internal sequence boundary now admits associated/subsampled alpha, independent
extra-channel bit depths, multiple alpha/spot channels, and the source-defined
depth, selection-mask, black, CFA, thermal and optional channels. The existing
fixed Thunder/Modular/VarDCT profiles keep their original admission. Metadata
parsing retains actual source enum/dimension/depth rejection statuses. Existing
modular dimensions, global/local group scheduling, early/late upsampling and
blending algorithms carry each plane without assigning new rendering semantics
to source-defined non-rendered channels.

The complete scalar spot-color stage is reused with its independently reviewed
native numeric configuration. PreparePipeline inserts each spot in metadata
order after blending and post-color reference storage. Coefficients remain a
borrow of the immutable metadata owner; retained reference pixels precede spot
rendering. Bundle output retains associated alpha. Actual ConvertToExternal
buffer exports compare requested unpremultiplication both ways, first-alpha
selection, integer/float16/float32 output and every independent extra plane,
including untouched output padding.

There are 111 genuine original encoder/header-writer fixtures: 69 complete source
successes and 42 explicit original failures, including odd-sized Modular EC
resampling and pre-transform reference consumption. No encoded stream is assumed
to decode successfully. Twelve source-selected malformed type fixtures retain
reserved/unknown/invalid enum errors and initialized partial metadata. Five
new differential groups retain 222 complete-file/metadata observations, 3,912
source exports, 11,157 truncation/corruption/later-reference observations and 24
rejected-type observations, plus reproduction of every genuine fixture byte.
Two ordinary Rust tests retain an independent native all-plane dyadic capture
and moved decoder/reference/metadata lifetimes. All 91,072 spot-component
requests, 2,592 lifecycle requests, 1,548 chroma image requests and 5,562 chroma
component requests remain required controls. The independent source audit
repeats all 15,315 defined requests with exact results from both original graphs.
All six focused configurations and all six complete local ordinary/ASan/UBSan
Debug, Release and Rust 1.85 checks pass. Each complete configuration runs 530
tests, 318 differential groups and 1,032,779 defined comparisons, with no
failures or ignored tests. The 888 half-stride source-only probes and 72 inherited
patch-boundary probes remain present. Native run 41494057418 (retired libjxl-rs repository)
now passes all six configurations on Linux and macOS with those same group and
request counts: Linux runs 531 tests per configuration; macOS runs 530. Both
native source audits verify the actual bindings, all 111 complete files, the
12 rejected-type fixtures and every half-stride probe. The accepted source is
merged into `main`, and the README extra-channel milestone is checked. This
internal bundle/output-conversion scope does not implement
direct output-stage callbacks, arbitrary color profiles or the public decoder.

The initial odd-stride Float16 export fixtures positively expose the original
Span<hwy::float16_t> row-store alignment precondition. All 888 padding-three
requests remain in an independent original-only census: 528 actually reach
nonrecovering UBSan span.h:59 misalignment diagnostics; 360 single-row or earlier
decode-error requests retain their original exact comparisons. All 888 aligned
padding-four Float16 controls are added, giving 15,315 defined differential
observations without weakening any prior case. Integer16 stores use byte stores
and Float32 uses byte-order memcpy stores, so their existing odd-stride cases
remain in parity unchanged. `tools/extra_half_boundaries.py` retains ordinary
captures and every positive diagnostic; no source routine, sanitizer, or old
expected word changes. The original undefined stores receive no Rust output
semantics. A separate sixth source-only group requires these exact diagnostics
from the instrumented graph.

## Complete sequence geometry and orientation

The sequence boundary now accepts every parsed YCbCr channel mode and retains
orientation/intrinsic-size metadata without adding a pixel transform inside the
decoder. Existing per-channel horizontal/vertical chroma stages, source crop and
blend scheduling, spatial resampling and external orientation conversion retain
their original algorithms. Fixed Thunder/Modular/VarDCT profile admission stays
unchanged. General color management and public callback orchestration remain
separate boundaries.

The genuine JPEG corpus includes all 64 triples of 1x1/2x2/2x1/1x2 input sampling
factors at 13x11 and 1x1, using separate sequential component scans. The actual
pinned public JPEG encoder reproduces all 128 stored JPEG XL payloads. Original
canonicalization can map different JPEG inputs to the same effective shifts;
these are 64 input triples, not 64 distinct effective frame layouts. Twenty
additional actual encoder sequences retain cropping/blending, reference overwrite,
animation, main/alpha resampling, multigroup edges and previews. Eight of
those sequences additionally combine every orientation with associated and
subsampled alpha, all source-defined extra-channel types, mixed sampling,
independent float alpha, multiple alpha/spot channels, filters and references.

The actual original global metadata writer produces all eight orientations with
alternating explicit/absent intrinsic size. Its zero-change control reproduces
the exact original header, and every rewrite preserves the complete frame
payload byte for byte. Differential groups compare 1,184 successful complete
files, 37,376 integer/half/float buffer exports and 20,128 truncation/corruption
observations. Output coverage includes one through four channels, keep-orientation
both ways, every byte-order option and aligned row padding. Source previews use
the existing explicit preview DecodeFrame path; this export observer receives
non-preview files. Every complete positive request must succeed in the original.

A separate ordinary Rust test checks 154 RGBA buffers captured independently from
both original graphs, including all eight oriented dimensions, untouched padding
and trailing guard bytes. The source audit reproduces fixture bytes and those
captures, verifies actual metadata/frame/output and selected chroma symbol
bindings, and repeats the full 58,688-request corpus with unchanged original
objects and active binaries. All previous 55,488 geometry requests remain in
that corpus, with 3,200 additional cross-feature observations.

With the verified extra-channel prerequisite, all six full local checks pass:
535 tests, 322 differential groups and 1,091,467 exact defined requests per
configuration, with no failures or ignored tests. Debug, Release and Rust 1.85
each compare against ordinary and ASan/UBSan originals. The separate source
audit reproduces all 128 JPEG encodings, 20 sequence encodings, eight metadata
rewrites and 154 native captures under both original graphs. Native run 41505031128 (retired libjxl-rs repository)
now passes all six configurations and every source audit on Linux and macOS.
Each configuration retains 322 groups and 1,091,467 exact defined requests, with
zero failures or ignored tests. Linux runs 536 tests per configuration; macOS
runs 535. The verified source is merged through PR #32 (retired libjxl-rs repository),
and the README geometry milestone is checked. Color management/HDR, original
JPEG reconstruction and the public incremental decoder remain open.


## Decoder signature and container-header helpers

The complete pinned signature and box-header helpers retain source minimum-byte
checks, unsigned size arithmetic, unbounded/extended box sizes, file-position
overflow errors, and exact output-parameter mutation on incomplete headers.
Public signature/status values follow decode.h and the existing transparent-u32
public-type convention. These Rust helpers do not expose a partial decoder
state machine or claim the public C ABI is complete.

The independent observer compiles five exact, hash-checked fragments from the
pinned decode.cc through the existing source extractor. It also calls the actual
public JxlSignatureCheck from the original archive. The corpus exhausts every
two-byte signature and varies container prefixes, output seeds, every box-header
byte, truncation, offsets and overflow. Three standalone regressions retain
independent original captures of partial assignments, signature prefixes and
file-position wrap. tools/decoder_api_reference.py audits the complete actual
Rust-generated corpus against both original graphs and checks source hashes,
public symbols, instrumentation and unchanged active artifacts.

All six focused ordinary/ASan/UBSan Debug, Release and Rust 1.85 configurations
pass four differential groups with 92,452 exact requests and three standalone
regressions each, with no failures or ignored tests. The independent source audit
retains every original request/observation pair in both source graphs. A separate
cold review found no source-defined discrepancy.
The geometry union passes all six complete local configurations: each runs 542
tests, 326 differential groups and 1,183,919 exact defined requests, with zero
failures or ignored tests. Complete lifecycle, extra-channel and geometry source
audits also pass. Its four API groups retain the original helper bodies and
corpus unchanged. Fresh native acceptance of this union remains pending.
The earlier leaf passed native run 41484988662 (retired libjxl-rs repository):
all six Linux configurations and the three instrumented macOS configurations.
The independent helper audit now checks exact optimization/sanitizer tokens,
every compiled generated fragment against the original source, and actual live
code-address ownership for JxlSignatureCheck and the observation adapter. All
92,452 original request/observation pairs remain exact in both source graphs.
The 32-bit size-narrowing branch is source-reviewed but requires its own native
target evidence; the current acceptance matrix is 64-bit Linux and macOS.
Container sequencing, compressed boxes, incremental events, callbacks
and progressive pixel output remain decoder work. The README public-decoder and
container checkboxes remain open.

## Decoder output and input-framing integration

The combined candidate retains the complete output-stage and signature/box-header
helper scopes above, on the verified geometry decoder. Both component source
and test inventories remain unchanged. All six complete local configurations
pass: Debug, Release and Rust 1.85 against ordinary and ASan/UBSan originals
each run 552 tests, 335 differential groups and 1,187,923 exact defined requests,
with zero failures or ignored tests. The independent output-stage, signature/box
helper, retained lifecycle, extra-channel and geometry source audits also pass.
Fresh native Linux/macOS acceptance of this combined tree remains pending.
The public incremental decoder and container checklist items remain open.
## Incremental decoder state ownership preparation

Frame-owned fields and the temporary codec-state borrow are represented
separately. All 17 owned frame fields retain their order and values, with no new
allocation or callback. Reversing the declared field projections and constructor
split recovers the prior complete module byte for byte after formatting. The
candidate passes all six complete local configurations: Debug, Release and
Rust 1.85 against ordinary and nonrecovering ASan/UBSan originals. Each runs
535 tests, 322 differential groups and 1,091,467 exact defined comparisons,
with zero failures or ignored tests. The lifecycle, extra-channel and geometry
source audits also pass. Native CI acceptance remains pending.
This prepares retained incremental state without implementing a public
decoder object, input processing, events, or callbacks. The public decoder
checklist item remains open.

## Scalar color transfer stages

The complete scalar FromLinear/ToLinear packet adds 11 differential groups with
10,360 exact original requests and seven standalone tests. It covers every
factory-selected operation, copied construction state, row guards, repeated
stage calls, ignored extra channels, and actual multigroup simple pipelines.
Existing PQ/sRGB encoding helpers and the legacy sRGB stage retain their bodies.
Both full local checks, including Rust 1.85, pass all six ordinary/ASan/UBSan
Debug, Release and MSRV configurations: 480 tests, 283 groups and 904,089 exact
requests each, with no failures or ignored tests. Native CI and complete-file
HDR/ICC/color-profile integration remain separate acceptance gates.
The command component passes complete ordinary and ASan/UBSan checks in Debug,
Release and Rust 1.85: each configuration runs 449 tests, including 261
differential groups and 894,179 exact requests, with no failures or ignored
tests. Native run 41463504601 (retired libjxl-rs repository)
also passes all six Linux and three instrumented macOS configurations. The
streaming reader component below requires its own complete integration checks.
ICC parsing/default CMS and whole-file ICC admission remain open; the README
color-profile checkbox stays unchecked.


## Incremental ICCReader

The complete pinned ICCReader Init, Process, Reset and CheckEOI component adds
eight differential groups and 2,159 exact original requests with 7,894 incremental
steps. Genuine original WriteICC streams and independently encoded ANS/Huffman
payloads exercise chunked replay, different starting-bit bases, shorter backing,
fatal headers/preambles/commands, output reuse and Reset. A full LZ77 ring and the
original 65,536-byte expansion-ratio checkpoint are reached. The corpus retains
4,425 NotEnoughBytes steps, 157 fatal observations, and 993 rollback observations
whose still-defined written prefix exceeds the restored logical index. All fatal
observations remain present alongside explicit caller Reset/replay recovery.

Four standalone regressions use independently captured original behavior for
Process before Init, owner moves after Init, and a partial LZ77 decode resumed
after moving the complete reader, and source-defined Process after failed Init
retaining the original cached Huffman mode. The genuine captured stream is 89 bytes and
708 bits; its 47-byte prefix restores index 20 while retaining a defined 512-byte
prefix, then resumes at the saved 373-bit skip.

The original observer includes headers and calls real methods from the existing
pristine complete icc_codec.cc unit. The durable tools/icc_reader_reference.py
audit verifies those symbol definitions and link bindings, existing ANS
decoder/checkpoint bodies after only the three enumerated source-cached mode
accesses and one constructor initializer are normalized, unchanged ICC command
bodies, full sanitizer coverage and both
unchanged optimized original graphs. Every request and initialized observation
is retained in the audit artifacts. ADR 0003 records the code-independent owned
ANS state and ephemeral borrowed-reader mapping. No source algorithm, numeric
flag, old expected output, runtime dependency or production C++ fallback changes.
All six focused native ARM configurations pass: ordinary and fully ASan/UBSan
Debug, Release and Rust 1.85 each complete 41 tests, including 32 differential
tests and 203,189 exact requests across the reader and affected ICC/ANS/entropy
boundaries, with no failures or ignored tests. Formatting, all-target/all-feature
Clippy and the independent source-binding audit pass. Both complete local checks
also pass ordinary and ASan/UBSan Debug, Release and Rust 1.85: each configuration
completes 461 tests, 269 differential groups and 896,338 exact requests, with no
failures or ignored tests. Native Linux/macOS CI and integration with current
main remain required before this component is delivered.


Independent cold review exposed a source-defined post-fatal Init path: histogram
parsing changes code.use_prefix_code before ANS creation while the prior cached
Huffman/config arrays remain valid. The source continues with its cached mode;
the corrected state carrier now retains that scalar. Ten additive requests retain
the exact original counterexample and both mode-switch directions, independently
observe cached scalar and table/config pointer identity, and positively execute
Process. Unused changed views do not exclude defined calls. Every previous 2,149
request and expected observation remains intact; corrected focused verification
and the source audit supersede the earlier packet's proof.
## JPEG reconstruction metadata bundle

The complete `JPEGData::VisitFields` and `SetJPEGDataFromICC` source boundaries
retain field ordering, all distributions and defaults, checks, partially mutated
delta indices, initialized owner contents, delayed allocations and ICC writes
before an error. Existing visitors handle encoding and reading; the production
crate adds no dependency, unsafe operation or C/C++ build requirement.

Five differential groups cover 7,715 exact requests: genuine baseline, grayscale,
progressive and restart JPEG metadata; every byte truncation; deterministic
mutations; all serialized fields and component-ID forms; malformed marker/table
and scan metadata; reset/zero-run boundaries; and live ICC owner writes. Three
standalone regressions retain original unaligned bundle bytes, initialized state
at a truncated read and an ICC error after a completed marker write. Focused
Debug, Release and Rust 1.85 checks compare both ordinary and fully ASan/UBSan
original graphs. All six configurations pass with five groups, 7,715 exact
requests and three standalone regressions each, with no failures or ignored
tests. All six full repository configurations also pass: each runs 457 tests,
266 differential groups and 901,894 exact requests, with no failures or ignored
tests. Native Linux/macOS CI and integration into `main` remain required.

`tools/jpeg_metadata_reference.py` independently captures real original
`ReadJpeg`, `Bundle::Write` and `Bundle::Read` observations and checks the pinned
source, fixture hashes, original symbol bindings, instrumentation and saved
source bytes. Both graphs agree on 11,852 source-only requests, including
11,787 truncated-read errors and exact complete owner snapshots. Its separate source-undefined probe positively diagnoses a reached
Huffman slot 4 at pristine `jpeg_data.cc:415` before `CanEncode` can report its
accumulated two-bit range error. Such reached out-of-bounds table indices have no
assigned codec result; defined aliases, malformed streams and partial errors
remain compared exactly. No compiler option, sanitizer, old case or expected
word changes.

`DecodeJPEGData` and its marker payload reconstruction are now translated over
the recovered Brotli decoder; see the recovered decoder state section above.
Whole-file JPEG reconstruction and public API admission still need separate
evidence. The README's JPEG reconstruction checkbox stays unchecked.

## skcms transfer functions and matrices

The pinned skcms dependency now has translated public transfer-function
classification, five predicates, constructors (including the original inline
HLGish shim), evaluation and inversion. The reached log2/exp2/power helpers keep
the source approximations and constants. Matrix support covers concatenation,
full double-temporary inversion, Bradford adaptation to XYZ D50 and primary
chromaticities to XYZ D50. Both inversion paths support actual input/output
aliasing and retain destination state at source failure checkpoints.

Twelve defined differential groups cover 207,144 exact requests: 119,268 transfer
calls and 87,876 matrix calls. All six focused ordinary/ASan/UBSan Debug, Release
and Rust 1.85 configurations pass on native ARM with zero failed or ignored
tests. The initial 71,226 matrix requests remain encoded exactly; extra controls
include paired exceptional entries, 432 paired concatenation NaN products and the
original sRGB/standard-primary cases.
All 362 positively undefined original transfer calls remain in a separate source
census and are never assigned invented Rust outputs. The independent source
binding audit verifies all 16 actual archive functions, every retained matrix
request and all 362 positive original-only boundaries. Both complete local
checks pass in Debug, Release and Rust 1.85: each configuration runs 544 tests,
331 differential groups and 1,239,923 exact defined comparisons, with zero
failures or ignored tests. Native Linux/macOS CI remains an acceptance gate.
ICC parsing, nonlinear curve fitting, pixel transformation and complete default
JXL CMS integration are still open; the README color-profile milestone remains
unchecked.


## skcms curve approximation and nonlinear fitting

The complete pinned `skcms.cc:328-342,2110-2421` fitting closure is translated:
maximum sampled round-trip error, inverse classification, residuals/gradients,
the original Gauss-Newton step, checked error, fixup, eight-iteration nonlinear
fitting and public ApproximateCurve. The canonical borrowed Curve representation
is shared with ICC parsing. Outputs retain null/count guards, partial writes,
best-TF restoration and error-before-approx assignment, including error aliases
with individual TF fields. Gradient evaluation reads the original six TF words
and does not require the unused e word to be initialized.

The retained initial inventory has 11,432 defined requests in six groups:
3,143 public approximations, 1,089 sampled round trips, 2,400 residual/gradient
requests, 3,600 Gauss-Newton steps, 600 complete nonlinear fits and 600 checked
errors. It contains 8-/16-bit tables, parametric curves, both original tolerances,
short and long grids, monotone/nonmonotone/constant/plateau tables, null/count
guards, output aliases and failed/restored destinations. Inputs are stored
without expected outputs. Ordinary tests additionally retain independent original
words for the degenerate two-point segment and six-field gradient, plus truly
partially initialized early-return state.

All initial requests agree between both unchanged original graphs and Debug,
Release and Rust 1.85 preflight probes. The independent audit verifies six public
archive functions and four actual private-helper code addresses in each graph,
with no changed source/compiler/sanitizer options or active graph mutation.
Regular Cargo differential tests call the actual Rust crate. All six complete
local configurations pass: each runs 551 tests, 336 differential groups and
1,250,923 exact defined comparisons, with zero failures or ignored tests. Both
independent source audits pass. All 11,432 initial fitting observations and all
206,712 inherited transfer/matrix request/observation pairs remain exact in both
original graphs. The transfer/matrix audit captures its own two module families;
the fitting audit supplies its separate private-helper environment.

Native acceptance and current-main integration remain pending. The separate
transfer/matrix PR still has a native Linux operand-order discrepancy under
investigation, including newly exposed competing-NaN cases; its corrected
prerequisite and expanded native gates must be accepted before this combined
boundary. This leaf does not complete pixel transformation, default JXL CMS
integration or whole-image color/HDR admission.
### skcms ICC parser and directly reached curves

The complete parser boundary translates profile/tag/curve carriers, public ICC
parsing with A2B/B2A priorities, all reached curv/para/MFT1/MFT2/mAB/mBA readers,
CICP, XYZ/CHAD/white-point getters, input-channel checks, built-in profiles and
inline setters. Scalar curve evaluation, linear-prefix fitting and identity
canonicalization preserve borrowed tables and partial output writes. This is a
parser dependency for general color management; it does not complete nonlinear
curve fitting, profile transforms, default JXL CMS orchestration or the public
incremental decoder.

Initial ordinary comparisons cover every one of the 88 pinned original ICC
profiles, 5,001 generated LUT/curve/channel combinations, every original public
priority request, 184,467 malformed/truncated structures, 34,920 direct curve
reader failures/null guards, and 20,400 evaluation/fitting/alias/identity cases.
Additional source-layout, static, getter, zeroing and same-count large-table
controls bring the corpus to 247,790 defined requests in ten groups. All six
complete local configurations pass: Debug, Release and Rust 1.85 against ordinary
and nonrecovering ASan/UBSan originals. Each passes 543 tests, 328 differential
groups and 1,280,569 exact comparisons, with zero failures or ignored tests.
The independent audit also passes both graphs, verifies 11 public and four
private live code addresses and actual source-object ownership, and retains all
247,790 prior request/observation pairs exactly. Twelve independent original
captures and moved/mutated borrowed ICC ownership tests pass. Native Linux/macOS
CI and current-main integration remain separate acceptance gates.

Nine original-only large-count probes positively diagnose source arithmetic UB:
canonicalization's signed `2*N` at skcms.cc:1245, evaluation's out-of-range
float-to-int at line 308, and 16-bit table evaluation's signed `2*lo` at line 318.
Every exact request and diagnostic remains present; 74 defined controls use the
same counts with reached index zero or the original canonicalization guard.
They are paired comparisons, without a blanket exclusion of large tables.
## Brotli streaming decoder dependency

The `brotli` crate module mechanically translates pinned Brotli
`db24a98aed38a16de78cc05ca15cdd8dc2a0d92d`. It registers the complete default
streaming decoder, retained state, raw dictionaries, metadata callbacks,
settings, status/error queries, output borrowing, and destruction. It introduces
no crate dependency or runtime C/C++ code. The source-distinct stack-owned
one-shot `BrotliDecoderDecompress` entry remains separate and unimplemented.
JPEG metadata/container consumers have not yet been connected to this module.

| Original source | Rust mapping |
| --- | --- |
| `c/common/platform.c`, `c/dec/state.{h,c}` | `memory`, `default_memory`, `layout`, `bitfield`, `state`: actual allocator-returned blocks, native layout and source-order field/allocation/free checkpoints |
| `c/common/{constants,context,dictionary,transform}.*`, `c/dec/{prefix,static_init}.*` | `tables`, `constants`, `dictionary`, `static_descriptors`, `transform`, `raw_transform`, `raw_bytes`: exact static bytes, named backing identity, initialized-prefix/tail behavior |
| `c/common/shared_dictionary.*`, `decode.c:1526–1718` | `shared_dictionary`, `compound_dictionary`, `stream_api`: caller-backed raw views and exact partial attachment/copy state |
| `c/dec/bit_reader.*`, `huffman.*`, `decode.c:361–1175` | `bit_reader`, `reader_copy`, `symbols`, `huffman`, `huffman_reader`, `block_length`, `mtf`, `context_map`, `tree_group`: complete reached scalar readers and header state |
| `decode.c:1179–1524,1719–1980` | `stream_switch`, `distance`, `command`, `ringbuffer`, `stream_blocks`, `stream_output`: source fast/safe kernels, raw output checkpoints and ring ownership |
| `decode.c:1982–2900` | `stream_commands`, `stream_header`, `stream_tables`, `stream_input`, `stream`: complete command kernels and streaming state machine |
| `decode.c` constructor/settings/getters/metadata/output APIs, `decode.h`, `state.h` | `instance`, `stream_api`, `status`, `running`: retained owner, callable fields, actual native callback handles and raw cursors |

The generator checks exact pinned source hashes and retains all 122,784
dictionary bytes, 2,048 context bytes, 704 command entries and built-in
prefix/suffix/transform tables with the MIT notice. Named static storage fixes
the independently reproduced CTFE promotion identity gap; no byte changes or
runtime initialization replace the original static data.

Focused macOS AArch64 evidence is complete for ordinary and fully ASan/UBSan
original graphs against Rust Debug, Release and 1.85. The complete stream packet
retains all 13,920 original requests, including its initial 3,780 genuine
compressed-stream schedules and all 548 earlier metadata traces. Each graph
produces 691,550,918 exact observation bytes. The API packet adds 3,084 exact
requests and 41 independently captured source cases; all 51 earlier complete
stream captures remain unchanged. Original encoder calls generate the inputs;
these are not Rust round trips. The public crate test replays seven of those
same source schedules through registered `jxl::brotli` APIs, checking cursor,
status, output-prefix, finished and pending-output observations.

`tools/check.py` prepares sixteen original support observers in one dedicated
pristine Brotli graph, verifies actual per-symbol live object/address bindings
and strict compiler options, and supplies their required test environments.
It generates and retains all 38,187 ContextMap requests from the actual source
fixture encoder. Every earlier component case remains in the all-feature test
inventory, including partial owner bytes, allocator failures and source-backed
callback replacement. The independent standalone runners retain their original
CMake graphs and every earlier request/output corpus. No source algorithms,
default configuration, sanitizer coverage or comparisons are relaxed.

Both native architectures independently accepted the raw packed-bit and raw
byte-XOR primitives before core use. The registered module now passes all six
complete ordinary/ASan/UBSan Debug, Release and Rust 1.85 repository
configurations on macOS AArch64: 689 tests, including all 322 retained libjxl
differential groups and 1,091,467 exact requests, with no failures or ignored
tests in each configuration. All sixteen mandatory Brotli source observers and
their retained component corpora also run in these gates. Native x86 complete
streaming/API acceptance remains pending. JPEG reconstruction, compressed
container integration and whole-codec completion remain open.
## Persistent incremental decoder input boundary

The complete original CanAddBuffer, InjectNextBufferedJxlpBox, AdvanceInput,
AvailableCodestream, AdvanceCodestream, RequestMoreInput and GetCodestreamInput
methods are translated with their reached SetInput/ReleaseInput/CloseInput
operations. Retained input identity, buffered-prefix consumption, skip debt,
out-of-order payload ownership and exact allocation-failure checkpoints remain
explicit. A callback-backed PaddedBytes representation preserves initialized
prefixes and unknown padding without changing prior global-allocator users.

The source-only initial probe captured 1,159 histories before implementation.
The additive corpus now includes unknown-field cursor short circuits,
callback-mutated input, bounded/unbounded fragments, release/replacement, unsigned
buffer-limit checks and all permutations of four out-of-order payloads. The
observer compiles exact hash-bound original member and public-function bodies;
its allocator records actual allocation/free events. Standalone captures come
only from that original observer. The final frozen candidate passes all six local
configurations: Debug, Release and Rust 1.85 against ordinary and nonrecovering
ASan/UBSan originals. Each runs 567 tests, 327 differential groups and 1,185,111
exact defined comparisons, with zero failures or ignored tests. Its new group
retains 1,192 histories and 24 standalone original captures. The independent
input audit verifies all 196 original C/C++ units in each graph, exact compiled
source fragments, live original/adapter code ownership, every original history
and both retained capture prefixes. The existing framing audit also retains all
92,452 requests against both originals. Native CI/current-main integration
remains a separate gate. The complete public decoder object, container state
machine, events, progressive output, callbacks and C ABI remain open.
## Original JPEG coefficient reconstruction

The JPEG output branch now retains actual quantized i32 coefficients through the
original group traversal, in-place transpose, fixed-point color correlation,
saturation, DC replacement and coefficient-range checks. Frame setup retains
component dimensions/sampling, DC multiplier clearing, i32 AC allocation,
quantization-table reconstruction and unused-table copying. JPEG finalization
keeps the source early return and never serializes inverse-transformed pixels.
The existing pixel entry points retain their null-JPEG behavior.

The candidate integrates the previously verified complete JPEG writer. Its
focused corpus retains 162 genuine JPEG/raw-JXL pairs: all 25 earlier chroma
fixtures, all 128 geometry/sampling cases, six original public-encoder outputs
from the writer corpus and three RGB baseline/progressive/restart files. These
include progressive scans/refinement, restarts and multiple writer chunks.
Each complete frame reconstructs the entire original
JPEG file byte for byte. Separate direct group calls retain initialized qblocks,
accumulated coefficients, partial-load failures, invalid strategies and tables,
DC clamps, grayscale/subsampling and fixed-point color-correlation boundaries.
Eighteen independent original complete/partial captures run without the optional
C++ oracle feature. Complete repository/native gates remain pending for this
candidate. Compressed JPEG reconstruction metadata decoding and whole-file
container/public decoder orchestration remain open; this does not complete
decoder expansion step 10.

Frame-setup mutation fixtures preserve the source vector resize, including
growth of a one-component input to two components. The missing-table control
clears the table vector and sets component indices to zero in both adapters,
so the original equality guard rejects before indexing. Retaining nonzero
indices after clearing violates vector indexing preconditions and cannot supply
a defined Rust expectation. Pre-correction failed evidence remains separate.
All 11,135 previously captured request/observation pairs remain byte-identical
within the expanded 13,874-request focused corpus. The complete source audit
passes both unchanged original graphs, verifies nine live source-object bindings
per graph, and reproduces all 18 complete/partial native captures. All 12
source-only NaN-conversion/zero-division probes retain their ordinary results
and positive nonrecovering sanitizer diagnostics. Full repository and native
CI acceptance remains pending.

### skcms baseline pixel transform candidate

The candidate completes the pinned baseline skcms pixel-transform and
destination-usability source closure: all original formats and 79 stages,
N=4 NEON/SSE arithmetic/load-store paths, tables/CLUTs, profile preparation,
reference PQ/HLG CICP helpers and approximate-profile/TRC predicates. It unites
the parser and fitting dependencies without changing their numeric algorithms.
HSW/SKX engines are now translated separately (see the recovered CMS work
above). JXL default CMS/ColorSpaceTransform/pipeline integration remains
separate work. Decoder checklist step 9 remains open until genuine
complete ICC/HDR files pass exact pixels/statuses and native acceptance.

All six local ordinary/nonrecovering ASan/UBSan configurations pass Debug,
Release and Rust 1.85 with 591 tests, 363 parity groups and 1,650,305 exact
observations per configuration. These retain all inherited requests, including
45 skcms parity tests with 617,526 exact observations: 465,934 inherited
transfer/fitting/parser requests and 151,592 transform requests. Ten standalone
transform tests also pass, including 16 independently captured original
observations. No tests are ignored or removed. All initial 11,448 transform
requests and their original observations remain retained unchanged.

The four independent source audits also pass: 206,712 transfer/matrix, 11,432
fitting, 247,790 parser and 151,592 transform requests. They preserve both
complete ordinary/instrumented observations and require actual archive/private
bindings and source configurations. The transform audit additionally verifies
all 42 formats, three alpha formats, 79 stages and actual baseline-object
disassembly.

Signed source channel counts retain the original post-decrement checkpoint and
u32-to-int call conversions. Negative counts except INT_MIN read no curve;
eight high-bit raw A2B/B2A controls remain valid at zero pixels without reading
unknown grids. Three source-only INT_MIN cases positively diagnose the original
signed decrement at skcms.cc:2536, alongside the existing 362 transfer and nine
parser diagnostics. This does not exclude other high-bit channel words.

Native Linux/macOS acceptance and current-main integration remain required.
The inherited native x86 numeric/source-context discrepancy remains an explicit
open gate; this candidate does not select diagnostic compiler flags to hide it.


### Default CMS profile metadata and callback backend (candidate)

The selected skcms branch of `lib/jxl/cms/jxl_cms.cc` is translated completely
into `cms/jxl_cms.rs` and `cms/jxl_cms_profile.rs`: profile identification,
white-point recovery, primaries/transfer detection, CICP admission, luminance
computation, default interface, callback initialization, generated intermediate
ICC owners, aligned per-thread buffers, row conversion and destruction. The
reference selects Highway's scalar transfer wrapper and the original skcms N=4
NEON/SSE engine, or the HSW/SKX engine cpu_type() selects on x86_64.

Seventeen mandatory groups retain 946,197 exact source cases, including all 88
original ICC profiles, 84 generated RGB/gray/white-point/primaries/transfer
profiles, 5,001 complete synthetic profiles, 184,467 malformed admissions, all
CICP byte combinations, all half inputs through the transfer wrapper, repeated
aligned allocations, actual aliases, partial callback outputs and callback/ICC
replacement order. Initial failures remain recorded in the input manifest.
Independent actual-archive and pristine-include source contexts retain their
original compiler and sanitizer flags. All six complete local configurations
pass: Debug, Release and Rust 1.85 against both ordinary and instrumented
originals, each with 609 tests, 380 differential groups and 2,596,502 exact
requests. The independent CMS audit passes all 946,197 requests across both
source graphs, including exact empty-span callback nullness. The four inherited
source audits also pass their 617,526 retained requests and original-only
undefined-operation diagnostics. The final local gates retain every inherited
case after the source-defined empty destination-copy correction. Sixteen
independent macOS ARM source captures pass without invoking a C++ process.
Native two-platform acceptance is covered by the later deep gate listed under
the recovered CMS work; SKX remains unaccepted.

This backend alone did not complete decoder checklist step 9. The public
decoder (PR #50) later connected it with the streaming ICCReader, requested
output profiles and the HDR transfer stages. It compares statuses and output
sample digests of complete files exactly on both native platforms.
Additional concatenation coverage keeps all 24,324 prior requests and adds 432
paired quiet/signaling NaN products at every output-cell/term position. Native
ARM focused diagnostics compare all 24,756 calls exactly in Debug, Release and
Rust 1.85 against both original graphs. The Rust concatenation mapping preserves
the original operand ordering at three local arithmetic checkpoints. A permanent
differential group and independently captured ARM regression protect this
boundary. The expanded complete local suite and source audit pass all six
configurations.
Every one of the prior 206,712 request/output pairs is retained in both original
graphs, with exactly 432 additional defined pairs. Native Linux captures expose
64 defined ordinary/ASan output differences in the new product probes; the
original 24,324 concatenation observations stay unchanged. Temporary source
configuration diagnostics retain every raw word before selecting a reproducible
reference profile. Native acceptance and the decoder color milestone remain
open; these additional cases are not undefined and retain exact comparisons.

## Incremental FrameDecoder interface (2026-11-04)

`dec_frame.rs` now exposes the complete incremental `FrameDecoder` from
`dec_frame.{h,cc}` that `decode.cc` drives: `new` (the constructor),
`set_render_spotcolors`, `set_coalescing`, `init_frame`, `init_frame_output`,
`process_sections` with `SectionInfo`/`SectionStatus`, `flush`,
`finalize_frame`, `references`, `saved_as`, `sum_section_sizes`, `toc`
(`TocEntry`), `get_frame_header`, `has_decoded_dc`, `has_decoded_all`,
`num_complete_passes`, `set_pause_at_progressive`, `next_num_passes_to_pause`,
`set_image_output` and `add_extra_channel_output`. `JxlProgressiveDetail`
follows `decode.h`. ProcessSections keeps the original section ordering,
duplicate/skipped/partial statuses, the kDC pause after FinalizeDC and the
kLastPasses/kPasses pass limits. Flush keeps the blending and skip-progressive
early returns, the forced zero-pass group draw and the non-in-place modular
finalization. `dec_group.rs` now carries the complete `DecodeGroup`, including
its zero-pass 8x-upsampled DC branch and the stored-coefficient forced draw.
PreparePipeline takes the original coalescing, spot-color and noise options and
selects the original WriteToOutputStage when an output descriptor is installed.

The previous `DecodeFrame` path (`SequenceDecoder` and every fixed `decode_*`
entry point) now runs over these public methods with unchanged behavior; all
existing frame-level parity groups pass unchanged. Language-required
deviations are listed in the module header and ADR 0003: per-call codec-state
borrows, the null pool, the explicit memory manager argument for the output
stage, and an unsafe constructor whose contract covers output descriptors and an
installed CMS. The constructor flag selects the simple or the low-memory
pipeline from PR #48 (the fixed profiles keep the simple one), and
metadata/header admission follows the sequence profile
from PR #47, including its whole-image color output and the Unsupported
result when the simple pipeline would need a CMS stage.
At the pinned revision `MaybeSetUnpremultiplyAlpha`, `MaybeSetFloatCallback`,
`MaybeSetRGB8OutputBuffer`, `GetProgressiveDetail`, `NumSections`,
`SectionOffsets` and `SectionSizes` do not exist, so none is added.

Two differential groups drive the original FrameDecoder in the oracle exactly as
`decode.cc` does: one decoder and image bundle per frame, input that grows one
section at a time, sections offered from `next_section`, reader bounds and close
checks, progressive pause events, FlushImage and skipped frames. Further modes
offer sections one at a time or all at once in a seeded permutation, truncate at
every frame, header and section boundary (and one byte before), turn coalescing
and spot rendering off, and use buffer or pixel-callback output in uint8,
uint16 and float with extra-channel outputs. Ten new genuine encoder fixtures
cover three-pass and two-pass VarDCT (XYB, gray, NONE, alpha, three groups, a
DC frame and a blended animation) and two- and three-pass Modular frames.
The sequence group reuses 87 genuine fixtures from the existing manifests plus
the two all-extra-channel files. Statuses, pass counts, section flags, every
output-buffer byte, callback counts, covered bundle samples and the
reference/DC state are compared exactly: 1,046 multipass and 1,824 sequence
requests, each mode run with both the simple and the low-memory pipeline, with
278 DC and 308 pass pauses, 1,392 successful and 196 rejected flushes, and 886
truncated decodes. The `decode.cc` orchestration and container input on top of
this interface followed in PR #50.
