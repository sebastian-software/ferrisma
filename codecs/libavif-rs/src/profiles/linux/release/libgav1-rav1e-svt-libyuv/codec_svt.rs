// Copyright 2020 Cloudinary. All rights reserved.
// SPDX-License-Identifier: BSD-2-Clause

// Mechanically translated from upstream/libavif/src/codec_svt.c.
// C ABI, ownership, pointer validity, and bounds follow the original headers.
extern "C" {
    pub type avifEncoderData;
    pub type SvtMetadataArray;
    fn avifAlloc(size: size_t) -> *mut core::ffi::c_void;
    fn avifFree(p: *mut core::ffi::c_void);
    fn avifCalloc(count: size_t, size: size_t) -> *mut core::ffi::c_void;
    fn avifCodecEncodeOutputAddSample(
        encodeOutput: *mut avifCodecEncodeOutput,
        data: *const uint8_t,
        len: size_t,
        sync: avifBool,
    ) -> avifResult;
    fn avifDiagnosticsPrintf(
        diag: *mut avifDiagnostics,
        format: *const core::ffi::c_char,
        ...
    );
    fn svt_av1_get_version() -> *const core::ffi::c_char;
    fn svt_av1_enc_init_handle(
        p_handle: *mut *mut EbComponentType,
        config_ptr: *mut EbSvtAv1EncConfiguration,
    ) -> EbErrorType;
    fn svt_av1_enc_set_parameter(
        svt_enc_component: *mut EbComponentType,
        pComponentParameterStructure: *mut EbSvtAv1EncConfiguration,
    ) -> EbErrorType;
    fn svt_av1_enc_parse_parameter(
        pComponentParameterStructure: *mut EbSvtAv1EncConfiguration,
        name: *const core::ffi::c_char,
        value: *const core::ffi::c_char,
    ) -> EbErrorType;
    fn svt_av1_enc_init(svt_enc_component: *mut EbComponentType) -> EbErrorType;
    fn svt_av1_enc_send_picture(
        svt_enc_component: *mut EbComponentType,
        p_buffer: *mut EbBufferHeaderType,
    ) -> EbErrorType;
    fn svt_av1_enc_get_packet(
        svt_enc_component: *mut EbComponentType,
        p_buffer: *mut *mut EbBufferHeaderType,
        pic_send_done: uint8_t,
    ) -> EbErrorType;
    fn svt_av1_enc_release_out_buffer(p_buffer: *mut *mut EbBufferHeaderType);
    fn svt_av1_enc_deinit(svt_enc_component: *mut EbComponentType) -> EbErrorType;
    fn svt_av1_enc_deinit_handle(svt_enc_component: *mut EbComponentType) -> EbErrorType;
    fn memset(
        __s: *mut core::ffi::c_void,
        __c: core::ffi::c_int,
        __n: size_t,
    ) -> *mut core::ffi::c_void;
}
pub type size_t = usize;
pub type __int8_t = i8;
pub type __uint8_t = u8;
pub type __uint16_t = u16;
pub type __int32_t = i32;
pub type __uint32_t = u32;
pub type __int64_t = i64;
pub type __uint64_t = u64;
pub type int8_t = __int8_t;
pub type int32_t = __int32_t;
pub type int64_t = __int64_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type avifBool = core::ffi::c_int;
pub type avifResult = core::ffi::c_uint;
pub const AVIF_RESULT_NO_AV1_ITEMS_FOUND: avifResult = 10;
pub const AVIF_RESULT_DECODE_SAMPLE_TRANSFORM_FAILED: avifResult = 34;
pub const AVIF_RESULT_ENCODE_SAMPLE_TRANSFORM_FAILED: avifResult = 33;
pub const AVIF_RESULT_INVALID_TONE_MAPPED_IMAGE: avifResult = 32;
pub const AVIF_RESULT_DECODE_GAIN_MAP_FAILED: avifResult = 31;
pub const AVIF_RESULT_ENCODE_GAIN_MAP_FAILED: avifResult = 30;
pub const AVIF_RESULT_INTERNAL_ERROR: avifResult = 29;
pub const AVIF_RESULT_INCOMPATIBLE_IMAGE: avifResult = 28;
pub const AVIF_RESULT_CANNOT_CHANGE_SETTING: avifResult = 27;
pub const AVIF_RESULT_OUT_OF_MEMORY: avifResult = 26;
pub const AVIF_RESULT_NOT_IMPLEMENTED: avifResult = 25;
pub const AVIF_RESULT_INVALID_ARGUMENT: avifResult = 24;
pub const AVIF_RESULT_WAITING_ON_IO: avifResult = 23;
pub const AVIF_RESULT_IO_ERROR: avifResult = 22;
pub const AVIF_RESULT_IO_NOT_SET: avifResult = 21;
pub const AVIF_RESULT_TRUNCATED_DATA: avifResult = 20;
pub const AVIF_RESULT_INVALID_CODEC_SPECIFIC_OPTION: avifResult = 19;
pub const AVIF_RESULT_INVALID_IMAGE_GRID: avifResult = 18;
pub const AVIF_RESULT_INVALID_EXIF_PAYLOAD: avifResult = 17;
pub const AVIF_RESULT_NO_IMAGES_REMAINING: avifResult = 16;
pub const AVIF_RESULT_NO_CODEC_AVAILABLE: avifResult = 15;
pub const AVIF_RESULT_ISPE_SIZE_MISMATCH: avifResult = 14;
pub const AVIF_RESULT_COLOR_ALPHA_SIZE_MISMATCH: avifResult = 13;
pub const AVIF_RESULT_DECODE_ALPHA_FAILED: avifResult = 12;
pub const AVIF_RESULT_DECODE_COLOR_FAILED: avifResult = 11;
pub const AVIF_RESULT_MISSING_IMAGE_ITEM: avifResult = 10;
pub const AVIF_RESULT_BMFF_PARSE_FAILED: avifResult = 9;
pub const AVIF_RESULT_ENCODE_ALPHA_FAILED: avifResult = 8;
pub const AVIF_RESULT_ENCODE_COLOR_FAILED: avifResult = 7;
pub const AVIF_RESULT_UNSUPPORTED_DEPTH: avifResult = 6;
pub const AVIF_RESULT_REFORMAT_FAILED: avifResult = 5;
pub const AVIF_RESULT_NO_YUV_FORMAT_SELECTED: avifResult = 4;
pub const AVIF_RESULT_NO_CONTENT: avifResult = 3;
pub const AVIF_RESULT_INVALID_FTYP: avifResult = 2;
pub const AVIF_RESULT_UNKNOWN_ERROR: avifResult = 1;
pub const AVIF_RESULT_OK: avifResult = 0;
pub type avifHeaderFormatFlags = core::ffi::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct avifROData {
    pub data: *const uint8_t,
    pub size: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct avifRWData {
    pub data: *mut uint8_t,
    pub size: size_t,
}
pub type avifPixelFormat = core::ffi::c_uint;
pub const AVIF_PIXEL_FORMAT_COUNT: avifPixelFormat = 5;
pub const AVIF_PIXEL_FORMAT_YUV400: avifPixelFormat = 4;
pub const AVIF_PIXEL_FORMAT_YUV420: avifPixelFormat = 3;
pub const AVIF_PIXEL_FORMAT_YUV422: avifPixelFormat = 2;
pub const AVIF_PIXEL_FORMAT_YUV444: avifPixelFormat = 1;
pub const AVIF_PIXEL_FORMAT_NONE: avifPixelFormat = 0;
pub type avifChromaSamplePosition = core::ffi::c_uint;
pub const AVIF_CHROMA_SAMPLE_POSITION_RESERVED: avifChromaSamplePosition = 3;
pub const AVIF_CHROMA_SAMPLE_POSITION_COLOCATED: avifChromaSamplePosition = 2;
pub const AVIF_CHROMA_SAMPLE_POSITION_VERTICAL: avifChromaSamplePosition = 1;
pub const AVIF_CHROMA_SAMPLE_POSITION_UNKNOWN: avifChromaSamplePosition = 0;
pub type avifRange = core::ffi::c_uint;
pub const AVIF_RANGE_FULL: avifRange = 1;
pub const AVIF_RANGE_LIMITED: avifRange = 0;
pub type avifColorPrimaries = uint16_t;
pub type avifTransferCharacteristics = uint16_t;
pub type avifMatrixCoefficients = uint16_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct avifDiagnostics {
    pub error: [core::ffi::c_char; 256],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct avifFraction {
    pub n: int32_t,
    pub d: int32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct avifSignedFraction {
    pub n: int32_t,
    pub d: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct avifUnsignedFraction {
    pub n: uint32_t,
    pub d: uint32_t,
}
pub type avifTransformFlags = uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct avifPixelAspectRatioBox {
    pub hSpacing: uint32_t,
    pub vSpacing: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct avifCleanApertureBox {
    pub widthN: uint32_t,
    pub widthD: uint32_t,
    pub heightN: uint32_t,
    pub heightD: uint32_t,
    pub horizOffN: uint32_t,
    pub horizOffD: uint32_t,
    pub vertOffN: uint32_t,
    pub vertOffD: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct avifImageRotation {
    pub angle: uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct avifImageMirror {
    pub axis: uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct avifContentLightLevelInformationBox {
    pub maxCLL: uint16_t,
    pub maxPALL: uint16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct avifImage {
    pub width: uint32_t,
    pub height: uint32_t,
    pub depth: uint32_t,
    pub yuvFormat: avifPixelFormat,
    pub yuvRange: avifRange,
    pub yuvChromaSamplePosition: avifChromaSamplePosition,
    pub yuvPlanes: [*mut uint8_t; 3],
    pub yuvRowBytes: [uint32_t; 3],
    pub imageOwnsYUVPlanes: avifBool,
    pub alphaPlane: *mut uint8_t,
    pub alphaRowBytes: uint32_t,
    pub imageOwnsAlphaPlane: avifBool,
    pub alphaPremultiplied: avifBool,
    pub icc: avifRWData,
    pub colorPrimaries: avifColorPrimaries,
    pub transferCharacteristics: avifTransferCharacteristics,
    pub matrixCoefficients: avifMatrixCoefficients,
    pub clli: avifContentLightLevelInformationBox,
    pub transformFlags: avifTransformFlags,
    pub pasp: avifPixelAspectRatioBox,
    pub clap: avifCleanApertureBox,
    pub irot: avifImageRotation,
    pub imir: avifImageMirror,
    pub exif: avifRWData,
    pub xmp: avifRWData,
    pub properties: *mut avifImageItemProperty,
    pub numProperties: size_t,
    pub gainMap: *mut avifGainMap,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct avifGainMap {
    pub image: *mut avifImage,
    pub gainMapMin: [avifSignedFraction; 3],
    pub gainMapMax: [avifSignedFraction; 3],
    pub gainMapGamma: [avifUnsignedFraction; 3],
    pub baseOffset: [avifSignedFraction; 3],
    pub alternateOffset: [avifSignedFraction; 3],
    pub baseHdrHeadroom: avifUnsignedFraction,
    pub alternateHdrHeadroom: avifUnsignedFraction,
    pub useBaseColorSpace: avifBool,
    pub altICC: avifRWData,
    pub altColorPrimaries: avifColorPrimaries,
    pub altTransferCharacteristics: avifTransferCharacteristics,
    pub altMatrixCoefficients: avifMatrixCoefficients,
    pub altYUVRange: avifRange,
    pub altDepth: uint32_t,
    pub altPlaneCount: uint32_t,
    pub altCLLI: avifContentLightLevelInformationBox,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct avifImageItemProperty {
    pub boxtype: [uint8_t; 4],
    pub usertype: [uint8_t; 16],
    pub boxPayload: avifRWData,
}
pub type avifSampleTransformRecipe = core::ffi::c_uint;
pub const AVIF_SAMPLE_TRANSFORM_BIT_DEPTH_EXTENSION_12B_8B_OVERLAP_4B: avifSampleTransformRecipe = 3;
pub const AVIF_SAMPLE_TRANSFORM_BIT_DEPTH_EXTENSION_12B_4B: avifSampleTransformRecipe = 2;
pub const AVIF_SAMPLE_TRANSFORM_BIT_DEPTH_EXTENSION_8B_8B: avifSampleTransformRecipe = 1;
pub const AVIF_SAMPLE_TRANSFORM_NONE: avifSampleTransformRecipe = 0;
pub type avifCodecChoice = core::ffi::c_uint;
pub const AVIF_CODEC_CHOICE_AVM: avifCodecChoice = 6;
pub const AVIF_CODEC_CHOICE_SVT: avifCodecChoice = 5;
pub const AVIF_CODEC_CHOICE_RAV1E: avifCodecChoice = 4;
pub const AVIF_CODEC_CHOICE_LIBGAV1: avifCodecChoice = 3;
pub const AVIF_CODEC_CHOICE_DAV1D: avifCodecChoice = 2;
pub const AVIF_CODEC_CHOICE_AOM: avifCodecChoice = 1;
pub const AVIF_CODEC_CHOICE_AUTO: avifCodecChoice = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct avifIOStats {
    pub colorOBUSize: size_t,
    pub alphaOBUSize: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct avifCodecSpecificOptions {
    pub entries: *mut avifCodecSpecificOption,
    pub elementSize: uint32_t,
    pub count: uint32_t,
    pub capacity: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct avifCodecSpecificOption {
    pub key: *mut core::ffi::c_char,
    pub value: *mut core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct avifScalingMode {
    pub horizontal: avifFraction,
    pub vertical: avifFraction,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct avifEncoder {
    pub codecChoice: avifCodecChoice,
    pub maxThreads: core::ffi::c_int,
    pub speed: core::ffi::c_int,
    pub keyframeInterval: core::ffi::c_int,
    pub timescale: uint64_t,
    pub repetitionCount: core::ffi::c_int,
    pub extraLayerCount: uint32_t,
    pub quality: core::ffi::c_int,
    pub qualityAlpha: core::ffi::c_int,
    pub minQuantizer: core::ffi::c_int,
    pub maxQuantizer: core::ffi::c_int,
    pub minQuantizerAlpha: core::ffi::c_int,
    pub maxQuantizerAlpha: core::ffi::c_int,
    pub tileRowsLog2: core::ffi::c_int,
    pub tileColsLog2: core::ffi::c_int,
    pub autoTiling: avifBool,
    pub scalingMode: avifScalingMode,
    pub ioStats: avifIOStats,
    pub diag: avifDiagnostics,
    pub data: *mut avifEncoderData,
    pub csOptions: *mut avifCodecSpecificOptions,
    pub headerFormat: avifHeaderFormatFlags,
    pub qualityGainMap: core::ffi::c_int,
    pub creationTime: uint64_t,
    pub modificationTime: uint64_t,
    pub sampleTransformRecipe: avifSampleTransformRecipe,
    pub width: uint32_t,
    pub height: uint32_t,
}
pub type avifAddImageFlag = core::ffi::c_uint;
pub const AVIF_ADD_IMAGE_FLAG_SINGLE: avifAddImageFlag = 2;
pub const AVIF_ADD_IMAGE_FLAG_FORCE_KEYFRAME: avifAddImageFlag = 1;
pub const AVIF_ADD_IMAGE_FLAG_NONE: avifAddImageFlag = 0;
pub type avifAddImageFlags = uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct avifDecodeSample {
    pub data: avifROData,
    pub ownsData: avifBool,
    pub partialData: avifBool,
    pub itemID: uint32_t,
    pub offset: uint64_t,
    pub size: size_t,
    pub spatialID: uint8_t,
    pub sync: avifBool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct avifEncodeSample {
    pub data: avifRWData,
    pub sync: avifBool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct avifEncodeSampleArray {
    pub sample: *mut avifEncodeSample,
    pub elementSize: uint32_t,
    pub count: uint32_t,
    pub capacity: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct avifCodecEncodeOutput {
    pub samples: avifEncodeSampleArray,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct avifCodec {
    pub csOptions: *const avifCodecSpecificOptions,
    pub internal: *mut avifCodecInternal,
    pub diag: *mut avifDiagnostics,
    pub maxThreads: core::ffi::c_int,
    pub imageSizeLimit: uint32_t,
    pub imageDimensionLimit: uint32_t,
    pub operatingPoint: uint8_t,
    pub allLayers: avifBool,
    pub getNextImage: avifCodecGetNextImageFunc,
    pub encodeImage: avifCodecEncodeImageFunc,
    pub encodeFinish: avifCodecEncodeFinishFunc,
    pub destroyInternal: avifCodecDestroyInternalFunc,
}
pub type avifCodecDestroyInternalFunc = Option<
    unsafe extern "C" fn(*mut avifCodec) -> (),
>;
pub type avifCodecEncodeFinishFunc = Option<
    unsafe extern "C" fn(*mut avifCodec, *mut avifCodecEncodeOutput) -> avifBool,
>;
pub type avifCodecEncodeImageFunc = Option<
    unsafe extern "C" fn(
        *mut avifCodec,
        *mut avifEncoder,
        *const avifImage,
        avifBool,
        core::ffi::c_int,
        core::ffi::c_int,
        core::ffi::c_int,
        avifEncoderChanges,
        avifBool,
        avifAddImageFlags,
        *mut avifCodecEncodeOutput,
    ) -> avifResult,
>;
pub type avifEncoderChanges = core::ffi::c_int;
pub type avifCodecGetNextImageFunc = Option<
    unsafe extern "C" fn(
        *mut avifCodec,
        *const avifDecodeSample,
        avifBool,
        *mut avifBool,
        *mut avifImage,
    ) -> avifBool,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct avifCodecInternal {
    pub svt_encoder: *mut EbComponentType,
    pub svt_config: EbSvtAv1EncConfiguration,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct EbSvtAv1EncConfiguration {
    pub enc_mode: int8_t,
    pub intra_period_length: int32_t,
    pub intra_refresh_type: SvtAv1IntraRefreshType,
    pub hierarchical_levels: uint32_t,
    pub pred_structure: PredStructure,
    pub source_width: uint32_t,
    pub source_height: uint32_t,
    pub forced_max_frame_width: uint32_t,
    pub forced_max_frame_height: uint32_t,
    pub frame_rate_numerator: uint32_t,
    pub frame_rate_denominator: uint32_t,
    pub encoder_bit_depth: uint32_t,
    pub encoder_color_format: EbColorFormat,
    pub profile: EbAv1SeqProfile,
    pub tier: uint32_t,
    pub level: uint32_t,
    pub color_primaries: EbColorPrimaries,
    pub transfer_characteristics: EbTransferCharacteristics,
    pub matrix_coefficients: EbMatrixCoefficients,
    pub color_range: EbColorRange,
    pub mastering_display: EbSvtAv1MasteringDisplayInfo,
    pub content_light_level: EbContentLightLevel,
    pub chroma_sample_position: EbChromaSamplePosition,
    pub rate_control_mode: uint8_t,
    pub qp: uint32_t,
    pub use_qp_file: bool,
    pub target_bit_rate: uint32_t,
    pub max_bit_rate: uint32_t,
    pub max_qp_allowed: uint32_t,
    pub min_qp_allowed: uint32_t,
    pub vbr_min_section_pct: uint32_t,
    pub vbr_max_section_pct: uint32_t,
    pub under_shoot_pct: uint32_t,
    pub over_shoot_pct: uint32_t,
    pub mbr_over_shoot_pct: uint32_t,
    pub starting_buffer_level_ms: int64_t,
    pub optimal_buffer_level_ms: int64_t,
    pub maximum_buffer_size_ms: int64_t,
    pub rc_stats_buffer: SvtAv1FixedBuf,
    pub pass: core::ffi::c_int,
    pub use_fixed_qindex_offsets: uint8_t,
    pub qindex_offsets: [int32_t; 6],
    pub key_frame_chroma_qindex_offset: int32_t,
    pub key_frame_qindex_offset: int32_t,
    pub chroma_qindex_offsets: [int32_t; 6],
    pub luma_y_dc_qindex_offset: int32_t,
    pub chroma_u_dc_qindex_offset: int32_t,
    pub chroma_u_ac_qindex_offset: int32_t,
    pub chroma_v_dc_qindex_offset: int32_t,
    pub chroma_v_ac_qindex_offset: int32_t,
    pub enable_dlf_flag: uint8_t,
    pub film_grain_denoise_strength: uint32_t,
    pub film_grain_denoise_apply: uint8_t,
    pub cdef_level: core::ffi::c_int,
    pub enable_restoration_filtering: core::ffi::c_int,
    pub enable_mfmv: core::ffi::c_int,
    pub scene_change_detection: uint32_t,
    pub tile_columns: int32_t,
    pub tile_rows: int32_t,
    pub look_ahead_distance: uint32_t,
    pub recode_loop: uint32_t,
    pub screen_content_mode: uint32_t,
    pub aq_mode: uint8_t,
    pub enable_tf: uint8_t,
    pub enable_overlays: bool,
    pub tune: uint8_t,
    pub superres_mode: uint8_t,
    pub superres_denom: uint8_t,
    pub superres_kf_denom: uint8_t,
    pub superres_qthres: uint8_t,
    pub superres_kf_qthres: uint8_t,
    pub superres_auto_search_type: uint8_t,
    pub fast_decode: uint8_t,
    pub sframe_dist: int32_t,
    pub sframe_mode: EbSFrameMode,
    pub level_of_parallelism: uint32_t,
    pub use_cpu_flags: EbCpuFlags,
    pub stat_report: uint32_t,
    pub recon_enabled: bool,
    pub force_key_frames: bool,
    pub multiply_keyint: bool,
    pub resize_mode: uint8_t,
    pub resize_denom: uint8_t,
    pub resize_kf_denom: uint8_t,
    pub enable_qm: bool,
    pub min_qm_level: uint8_t,
    pub max_qm_level: uint8_t,
    pub gop_constraint_rc: bool,
    pub lambda_scale_factors: [int32_t; 7],
    pub enable_dg: bool,
    pub startup_mg_size: uint8_t,
    pub startup_qp_offset: int8_t,
    pub frame_scale_evts: SvtAv1FrameScaleEvts,
    pub enable_roi_map: bool,
    pub tf_strength: uint8_t,
    pub fgs_table: *mut AomFilmGrain,
    pub enable_variance_boost: bool,
    pub variance_boost_strength: uint8_t,
    pub variance_octile: uint8_t,
    pub sharpness: int8_t,
    pub variance_boost_curve: uint8_t,
    pub luminance_qp_bias: uint8_t,
    pub lossless: bool,
    pub avif: bool,
    pub min_chroma_qm_level: uint8_t,
    pub max_chroma_qm_level: uint8_t,
    pub rtc: bool,
    pub qp_scale_compress_strength: uint8_t,
    pub sframe_posi: SvtAv1SFramePositions,
    pub sframe_qp: uint8_t,
    pub sframe_qp_offset: int8_t,
    pub adaptive_film_grain: bool,
    pub max_tx_size: uint8_t,
    pub extended_crf_qindex_offset: uint8_t,
    pub ac_bias: core::ffi::c_double,
    pub hbd_mds: core::ffi::c_int,
    pub enable_tf_key: bool,
    pub max_intra_bitrate_pct: uint32_t,
    pub max_inter_bitrate_pct: uint32_t,
    pub enable_intrabc: bool,
    pub max_managed_refs: uint8_t,
    pub padding: [uint8_t; 110],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SvtAv1SFramePositions {
    pub sframe_num: uint32_t,
    pub sframe_posis: *mut uint64_t,
    pub sframe_qp_num: uint32_t,
    pub sframe_qps: *mut uint8_t,
    pub sframe_qp_offsets: *mut int8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AomFilmGrain {
    pub apply_grain: int32_t,
    pub update_parameters: int32_t,
    pub scaling_points_y: [[int32_t; 2]; 14],
    pub num_y_points: int32_t,
    pub scaling_points_cb: [[int32_t; 2]; 10],
    pub num_cb_points: int32_t,
    pub scaling_points_cr: [[int32_t; 2]; 10],
    pub num_cr_points: int32_t,
    pub scaling_shift: int32_t,
    pub ar_coeff_lag: int32_t,
    pub ar_coeffs_y: [int32_t; 24],
    pub ar_coeffs_cb: [int32_t; 25],
    pub ar_coeffs_cr: [int32_t; 25],
    pub ar_coeff_shift: int32_t,
    pub cb_mult: int32_t,
    pub cb_luma_mult: int32_t,
    pub cb_offset: int32_t,
    pub cr_mult: int32_t,
    pub cr_luma_mult: int32_t,
    pub cr_offset: int32_t,
    pub overlap_flag: int32_t,
    pub clip_to_restricted_range: int32_t,
    pub bit_depth: int32_t,
    pub chroma_scaling_from_luma: int32_t,
    pub grain_scale_shift: int32_t,
    pub random_seed: uint16_t,
    pub ignore_ref: int32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SvtAv1FrameScaleEvts {
    pub evt_num: uint32_t,
    pub start_frame_nums: *mut uint64_t,
    pub resize_kf_denoms: *mut uint32_t,
    pub resize_denoms: *mut uint32_t,
}
pub type EbCpuFlags = uint64_t;
pub type EbSFrameMode = core::ffi::c_uint;
pub const SFRAME_DEC_POSI_BASE: EbSFrameMode = 4;
pub const SFRAME_FLEXIBLE_BASE: EbSFrameMode = 3;
pub const SFRAME_NEAREST_BASE: EbSFrameMode = 2;
pub const SFRAME_STRICT_BASE: EbSFrameMode = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SvtAv1FixedBuf {
    pub buf: *mut core::ffi::c_void,
    pub sz: uint64_t,
}
pub type EbChromaSamplePosition = core::ffi::c_uint;
pub const EB_CSP_RESERVED: EbChromaSamplePosition = 3;
pub const EB_CSP_COLOCATED: EbChromaSamplePosition = 2;
pub const EB_CSP_VERTICAL: EbChromaSamplePosition = 1;
pub const EB_CSP_UNKNOWN: EbChromaSamplePosition = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct EbContentLightLevel {
    pub max_cll: uint16_t,
    pub max_fall: uint16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct EbSvtAv1MasteringDisplayInfo {
    pub r: EbSvtAv1ChromaPoints,
    pub g: EbSvtAv1ChromaPoints,
    pub b: EbSvtAv1ChromaPoints,
    pub white_point: EbSvtAv1ChromaPoints,
    pub max_luma: uint32_t,
    pub min_luma: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct EbSvtAv1ChromaPoints {
    pub x: uint16_t,
    pub y: uint16_t,
}
pub type EbColorRange = core::ffi::c_uint;
pub const EB_CR_FULL_RANGE: EbColorRange = 1;
pub const EB_CR_STUDIO_RANGE: EbColorRange = 0;
pub type EbMatrixCoefficients = core::ffi::c_uint;
pub const EB_CICP_MC_RESERVED_18: EbMatrixCoefficients = 18;
pub const EB_CICP_MC_RESERVED_17: EbMatrixCoefficients = 17;
pub const EB_CICP_MC_RESERVED_16: EbMatrixCoefficients = 16;
pub const EB_CICP_MC_RESERVED_15: EbMatrixCoefficients = 15;
pub const EB_CICP_MC_ICTCP: EbMatrixCoefficients = 14;
pub const EB_CICP_MC_CHROMAT_CL: EbMatrixCoefficients = 13;
pub const EB_CICP_MC_CHROMAT_NCL: EbMatrixCoefficients = 12;
pub const EB_CICP_MC_SMPTE_2085: EbMatrixCoefficients = 11;
pub const EB_CICP_MC_BT_2020_CL: EbMatrixCoefficients = 10;
pub const EB_CICP_MC_BT_2020_NCL: EbMatrixCoefficients = 9;
pub const EB_CICP_MC_SMPTE_YCGCO: EbMatrixCoefficients = 8;
pub const EB_CICP_MC_SMPTE_240: EbMatrixCoefficients = 7;
pub const EB_CICP_MC_BT_601: EbMatrixCoefficients = 6;
pub const EB_CICP_MC_BT_470_B_G: EbMatrixCoefficients = 5;
pub const EB_CICP_MC_FCC: EbMatrixCoefficients = 4;
pub const EB_CICP_MC_RESERVED_3: EbMatrixCoefficients = 3;
pub const EB_CICP_MC_UNSPECIFIED: EbMatrixCoefficients = 2;
pub const EB_CICP_MC_BT_709: EbMatrixCoefficients = 1;
pub const EB_CICP_MC_IDENTITY: EbMatrixCoefficients = 0;
pub type EbTransferCharacteristics = core::ffi::c_uint;
pub const EB_CICP_TC_RESERVED_23: EbTransferCharacteristics = 23;
pub const EB_CICP_TC_RESERVED_22: EbTransferCharacteristics = 22;
pub const EB_CICP_TC_RESERVED_21: EbTransferCharacteristics = 21;
pub const EB_CICP_TC_RESERVED_20: EbTransferCharacteristics = 20;
pub const EB_CICP_TC_RESERVED_19: EbTransferCharacteristics = 19;
pub const EB_CICP_TC_HLG: EbTransferCharacteristics = 18;
pub const EB_CICP_TC_SMPTE_428: EbTransferCharacteristics = 17;
pub const EB_CICP_TC_SMPTE_2084: EbTransferCharacteristics = 16;
pub const EB_CICP_TC_BT_2020_12_BIT: EbTransferCharacteristics = 15;
pub const EB_CICP_TC_BT_2020_10_BIT: EbTransferCharacteristics = 14;
pub const EB_CICP_TC_SRGB: EbTransferCharacteristics = 13;
pub const EB_CICP_TC_BT_1361: EbTransferCharacteristics = 12;
pub const EB_CICP_TC_IEC_61966: EbTransferCharacteristics = 11;
pub const EB_CICP_TC_LOG_100_SQRT10: EbTransferCharacteristics = 10;
pub const EB_CICP_TC_LOG_100: EbTransferCharacteristics = 9;
pub const EB_CICP_TC_LINEAR: EbTransferCharacteristics = 8;
pub const EB_CICP_TC_SMPTE_240: EbTransferCharacteristics = 7;
pub const EB_CICP_TC_BT_601: EbTransferCharacteristics = 6;
pub const EB_CICP_TC_BT_470_B_G: EbTransferCharacteristics = 5;
pub const EB_CICP_TC_BT_470_M: EbTransferCharacteristics = 4;
pub const EB_CICP_TC_RESERVED_3: EbTransferCharacteristics = 3;
pub const EB_CICP_TC_UNSPECIFIED: EbTransferCharacteristics = 2;
pub const EB_CICP_TC_BT_709: EbTransferCharacteristics = 1;
pub const EB_CICP_TC_RESERVED_0: EbTransferCharacteristics = 0;
pub type EbColorPrimaries = core::ffi::c_uint;
pub const EB_CICP_CP_RESERVED_26: EbColorPrimaries = 26;
pub const EB_CICP_CP_RESERVED_25: EbColorPrimaries = 25;
pub const EB_CICP_CP_RESERVED_24: EbColorPrimaries = 24;
pub const EB_CICP_CP_RESERVED_23: EbColorPrimaries = 23;
pub const EB_CICP_CP_EBU_3213: EbColorPrimaries = 22;
pub const EB_CICP_CP_RESERVED_13: EbColorPrimaries = 13;
pub const EB_CICP_CP_SMPTE_432: EbColorPrimaries = 12;
pub const EB_CICP_CP_SMPTE_431: EbColorPrimaries = 11;
pub const EB_CICP_CP_XYZ: EbColorPrimaries = 10;
pub const EB_CICP_CP_BT_2020: EbColorPrimaries = 9;
pub const EB_CICP_CP_GENERIC_FILM: EbColorPrimaries = 8;
pub const EB_CICP_CP_SMPTE_240: EbColorPrimaries = 7;
pub const EB_CICP_CP_BT_601: EbColorPrimaries = 6;
pub const EB_CICP_CP_BT_470_B_G: EbColorPrimaries = 5;
pub const EB_CICP_CP_BT_470_M: EbColorPrimaries = 4;
pub const EB_CICP_CP_RESERVED_3: EbColorPrimaries = 3;
pub const EB_CICP_CP_UNSPECIFIED: EbColorPrimaries = 2;
pub const EB_CICP_CP_BT_709: EbColorPrimaries = 1;
pub const EB_CICP_CP_RESERVED_0: EbColorPrimaries = 0;
pub type EbAv1SeqProfile = core::ffi::c_uint;
pub const PROFESSIONAL_PROFILE: EbAv1SeqProfile = 2;
pub const HIGH_PROFILE: EbAv1SeqProfile = 1;
pub const MAIN_PROFILE: EbAv1SeqProfile = 0;
pub type EbColorFormat = core::ffi::c_uint;
pub const EB_YUV444: EbColorFormat = 3;
pub const EB_YUV422: EbColorFormat = 2;
pub const EB_YUV420: EbColorFormat = 1;
pub const EB_YUV400: EbColorFormat = 0;
pub type PredStructure = core::ffi::c_uint;
pub const PRED_INVALID: PredStructure = 255;
pub const PRED_TOTAL_COUNT: PredStructure = 3;
pub const RANDOM_ACCESS: PredStructure = 2;
pub const LOW_DELAY: PredStructure = 1;
pub const ALL_INTRA: PredStructure = 0;
pub type SvtAv1IntraRefreshType = core::ffi::c_uint;
pub const SVT_AV1_KF_REFRESH: SvtAv1IntraRefreshType = 2;
pub const SVT_AV1_FWDKF_REFRESH: SvtAv1IntraRefreshType = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct EbComponentType {
    pub size: uint32_t,
    pub p_component_private: *mut core::ffi::c_void,
    pub p_application_private: *mut core::ffi::c_void,
}
pub type EbErrorType = core::ffi::c_int;
pub const EB_ErrorMax: EbErrorType = 2147483647;
pub const EB_NoErrorFifoShutdown: EbErrorType = -2147475404;
pub const EB_NoErrorEmptyQueue: EbErrorType = -2147475405;
pub const EB_ErrorDestroyMutexFailed: EbErrorType = -2147475406;
pub const EB_ErrorMutexUnresponsive: EbErrorType = -2147475407;
pub const EB_ErrorCreateMutexFailed: EbErrorType = -2147475408;
pub const EB_ErrorDestroySemaphoreFailed: EbErrorType = -2147475422;
pub const EB_ErrorSemaphoreUnresponsive: EbErrorType = -2147475423;
pub const EB_ErrorDestroyThreadFailed: EbErrorType = -2147475438;
pub const EB_ErrorBadParameter: EbErrorType = -2147479547;
pub const EB_ErrorInvalidComponent: EbErrorType = -2147479548;
pub const EB_ErrorUndefined: EbErrorType = -2147479551;
pub const EB_ErrorInsufficientResources: EbErrorType = -2147479552;
pub const EB_Corrupt_Frame: EbErrorType = 1073745932;
pub const EB_DecDecodingError: EbErrorType = 1073745928;
pub const EB_DecNoOutputPicture: EbErrorType = 1073745924;
pub const EB_DecUnsupportedBitstream: EbErrorType = 1073745920;
pub const EB_ErrorNone: EbErrorType = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct EbBufferHeaderType {
    pub size: uint32_t,
    pub p_buffer: *mut uint8_t,
    pub n_filled_len: uint32_t,
    pub n_alloc_len: uint32_t,
    pub p_app_private: *mut core::ffi::c_void,
    pub wrapper_ptr: *mut core::ffi::c_void,
    pub n_tick_count: uint32_t,
    pub dts: int64_t,
    pub pts: int64_t,
    pub temporal_layer_index: uint8_t,
    pub qp: uint32_t,
    pub avg_qp: uint32_t,
    pub pic_type: EbAv1PictureType,
    pub luma_sse: uint64_t,
    pub cr_sse: uint64_t,
    pub cb_sse: uint64_t,
    pub flags: uint32_t,
    pub luma_ssim: core::ffi::c_double,
    pub cr_ssim: core::ffi::c_double,
    pub cb_ssim: core::ffi::c_double,
    pub metadata: *mut SvtMetadataArray,
}
pub type EbAv1PictureType = core::ffi::c_uint;
pub const EB_AV1_INVALID_PICTURE: EbAv1PictureType = 255;
pub const EB_AV1_SWITCH_PICTURE: EbAv1PictureType = 7;
pub const EB_AV1_FW_KEY_PICTURE: EbAv1PictureType = 5;
pub const EB_AV1_SHOW_EXISTING_PICTURE: EbAv1PictureType = 6;
pub const EB_AV1_NON_REF_PICTURE: EbAv1PictureType = 4;
pub const EB_AV1_KEY_PICTURE: EbAv1PictureType = 3;
pub const EB_AV1_INTRA_ONLY_PICTURE: EbAv1PictureType = 2;
pub const EB_AV1_ALT_REF_PICTURE: EbAv1PictureType = 1;
pub const EB_AV1_INTER_PICTURE: EbAv1PictureType = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct EbSvtIOFormat {
    pub luma: *mut uint8_t,
    pub cb: *mut uint8_t,
    pub cr: *mut uint8_t,
    pub y_stride: uint32_t,
    pub cr_stride: uint32_t,
    pub cb_stride: uint32_t,
}
pub const NULL: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
pub const UINT32_MAX: core::ffi::c_uint = 4294967295 as core::ffi::c_uint;
pub const AVIF_TRUE: core::ffi::c_int = 1 as core::ffi::c_int;
pub const AVIF_FALSE: core::ffi::c_int = 0 as core::ffi::c_int;
pub const AVIF_QUALITY_LOSSLESS: core::ffi::c_int = 100 as core::ffi::c_int;
pub const AVIF_SPEED_DEFAULT: core::ffi::c_int = -(1 as core::ffi::c_int);
pub const true_0: core::ffi::c_int = 1 as core::ffi::c_int;
pub const EB_BUFFERFLAG_EOS: core::ffi::c_int = 0x1 as core::ffi::c_int;
unsafe extern "C" fn svtQualityToQuantizer(
    mut quality: core::ffi::c_int,
) -> core::ffi::c_int {
    let quantizer: core::ffi::c_int = ((100 as core::ffi::c_int - quality)
        * 63 as core::ffi::c_int + 50 as core::ffi::c_int) / 100 as core::ffi::c_int;
    return quantizer;
}
unsafe extern "C" fn svtCodecEncodeImage(
    mut codec: *mut avifCodec,
    mut encoder: *mut avifEncoder,
    mut image: *const avifImage,
    mut alpha: avifBool,
    mut tileRowsLog2: core::ffi::c_int,
    mut tileColsLog2: core::ffi::c_int,
    mut quality: core::ffi::c_int,
    mut encoderChanges: avifEncoderChanges,
    mut disableLaggedOutput: avifBool,
    mut addImageFlags: uint32_t,
    mut output: *mut avifCodecEncodeOutput,
) -> avifResult {
    let mut input_picture_buffer: *mut EbSvtIOFormat = 0 as *mut EbSvtIOFormat;
    let mut bytesPerPixel: uint32_t = 0;
    let mut uvHeight: uint32_t = 0;
    let mut frame_type: EbAv1PictureType = EB_AV1_INTER_PICTURE;
    let mut current_block: u64;
    if encoderChanges != 0 {
        return AVIF_RESULT_NOT_IMPLEMENTED;
    }
    if !((*(*codec).internal).svt_encoder).is_null() {
        if (*(*codec).internal).svt_config.source_width != (*image).width
            || (*(*codec).internal).svt_config.source_height != (*image).height
        {
            return AVIF_RESULT_NOT_IMPLEMENTED;
        }
    }
    if (*encoder).width != 0 || (*encoder).height != 0
        || (*encoder).extraLayerCount > 0 as uint32_t
    {
        return AVIF_RESULT_NOT_IMPLEMENTED;
    }
    let mut result: avifResult = AVIF_RESULT_UNKNOWN_ERROR;
    let mut color_format: EbColorFormat = EB_YUV420;
    let mut uvPlanes: *mut uint8_t = 0 as *mut uint8_t;
    let mut input_buffer: *mut EbBufferHeaderType = 0 as *mut EbBufferHeaderType;
    let mut res: EbErrorType = EB_ErrorNone;
    let mut y_shift: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut svt_range: EbColorRange = EB_CR_STUDIO_RANGE;
    if alpha != 0 {
        svt_range = EB_CR_FULL_RANGE;
        y_shift = 1 as core::ffi::c_int;
    } else {
        svt_range = (if (*image).yuvRange as core::ffi::c_uint
            == AVIF_RANGE_FULL as core::ffi::c_int as core::ffi::c_uint
        {
            EB_CR_FULL_RANGE as core::ffi::c_int
        } else {
            EB_CR_STUDIO_RANGE as core::ffi::c_int
        }) as EbColorRange;
        match (*image).yuvFormat as core::ffi::c_uint {
            1 => {
                color_format = EB_YUV444;
            }
            2 => {
                color_format = EB_YUV422;
            }
            3 => {
                color_format = EB_YUV420;
                y_shift = 1 as core::ffi::c_int;
            }
            4 | 0 | 5 | _ => return AVIF_RESULT_UNKNOWN_ERROR,
        }
    }
    if ((*(*codec).internal).svt_encoder).is_null() {
        let mut svt_config: *mut EbSvtAv1EncConfiguration = &mut (*(*codec).internal)
            .svt_config;
        memset(
            svt_config as *mut core::ffi::c_void,
            0 as core::ffi::c_int,
            ::core::mem::size_of::<EbSvtAv1EncConfiguration>() as size_t,
        );
        res = svt_av1_enc_init_handle(&mut (*(*codec).internal).svt_encoder, svt_config);
        if res as core::ffi::c_int != EB_ErrorNone as core::ffi::c_int {
            current_block = 0;
        } else {
            (*svt_config).encoder_color_format = color_format;
            (*svt_config).encoder_bit_depth = (*image).depth as uint8_t as uint32_t;
            (*svt_config).color_primaries = (*image).colorPrimaries as EbColorPrimaries;
            (*svt_config).transfer_characteristics = (*image).transferCharacteristics
                as EbTransferCharacteristics;
            (*svt_config).matrix_coefficients = (*image).matrixCoefficients
                as EbMatrixCoefficients;
            (*svt_config).color_range = svt_range;
            (*svt_config).source_width = (*image).width;
            (*svt_config).source_height = (*image).height;
            (*svt_config).level_of_parallelism = (*encoder).maxThreads as uint32_t;
            (*svt_config).aq_mode = 2 as uint8_t;
            (*svt_config).rc_stats_buffer = {
                let mut init = SvtAv1FixedBuf {
                    buf: NULL,
                    sz: 0 as uint64_t,
                };
                init
            };
            (*svt_config).rate_control_mode = 0 as uint8_t;
            if alpha != 0 {
                (*svt_config).min_qp_allowed = (if (*encoder).minQuantizerAlpha
                    < 0 as core::ffi::c_int
                {
                    0 as core::ffi::c_int
                } else if (62 as core::ffi::c_int) < (*encoder).minQuantizerAlpha {
                    62 as core::ffi::c_int
                } else {
                    (*encoder).minQuantizerAlpha
                }) as uint32_t;
                (*svt_config).max_qp_allowed = (if (*encoder).maxQuantizerAlpha
                    < 0 as core::ffi::c_int
                {
                    0 as core::ffi::c_int
                } else if (63 as core::ffi::c_int) < (*encoder).maxQuantizerAlpha {
                    63 as core::ffi::c_int
                } else {
                    (*encoder).maxQuantizerAlpha
                }) as uint32_t;
            } else {
                (*svt_config).min_qp_allowed = (if (*encoder).minQuantizer
                    < 0 as core::ffi::c_int
                {
                    0 as core::ffi::c_int
                } else if (62 as core::ffi::c_int) < (*encoder).minQuantizer {
                    62 as core::ffi::c_int
                } else {
                    (*encoder).minQuantizer
                }) as uint32_t;
                (*svt_config).max_qp_allowed = (if (*encoder).maxQuantizer
                    < 0 as core::ffi::c_int
                {
                    0 as core::ffi::c_int
                } else if (63 as core::ffi::c_int) < (*encoder).maxQuantizer {
                    63 as core::ffi::c_int
                } else {
                    (*encoder).maxQuantizer
                }) as uint32_t;
            }
            (*svt_config).qp = svtQualityToQuantizer(quality) as uint32_t;
            if tileRowsLog2 != 0 as core::ffi::c_int {
                (*svt_config).tile_rows = tileRowsLog2 as int32_t;
            }
            if tileColsLog2 != 0 as core::ffi::c_int {
                (*svt_config).tile_columns = tileColsLog2 as int32_t;
            }
            if (*encoder).speed != AVIF_SPEED_DEFAULT {
                (*svt_config).enc_mode = (*encoder).speed as int8_t;
            }
            if color_format as core::ffi::c_uint
                == EB_YUV422 as core::ffi::c_int as core::ffi::c_uint
                || (*image).depth > 10 as uint32_t
            {
                (*svt_config).profile = PROFESSIONAL_PROFILE;
            } else if color_format as core::ffi::c_uint
                == EB_YUV444 as core::ffi::c_int as core::ffi::c_uint
            {
                (*svt_config).profile = HIGH_PROFILE;
            }
            (*svt_config).force_key_frames = true_0 != 0;
            if (*encoder).keyframeInterval > 1 as core::ffi::c_int {
                (*svt_config).intra_period_length = ((*encoder).keyframeInterval
                    - 1 as core::ffi::c_int) as int32_t;
            }
            let mut i: uint32_t = 0 as uint32_t;
            loop {
                if !(i < (*(*codec).csOptions).count) {
                    current_block = 1;
                    break;
                }
                let mut entry: *const avifCodecSpecificOption = &mut *((*(*codec)
                    .csOptions)
                    .entries)
                    .offset(i as isize) as *mut avifCodecSpecificOption;
                if (svt_av1_enc_parse_parameter(svt_config, (*entry).key, (*entry).value)
                    as core::ffi::c_int) < 0 as core::ffi::c_int
                {
                    avifDiagnosticsPrintf(
                        (*codec).diag,
                        b"Invalid value for %s: %s.\0" as *const u8
                            as *const core::ffi::c_char,
                        (*entry).key,
                        (*entry).value,
                    );
                    result = AVIF_RESULT_INVALID_CODEC_SPECIFIC_OPTION;
                    current_block = 0;
                    break;
                } else {
                    i = i.wrapping_add(1);
                }
            }
            match current_block {
                0 => {}
                _ => {
                    (*svt_config).lossless = quality == AVIF_QUALITY_LOSSLESS;
                    (*svt_config).avif = addImageFlags
                        & AVIF_ADD_IMAGE_FLAG_SINGLE as core::ffi::c_int as uint32_t
                        != 0 as uint32_t;
                    res = svt_av1_enc_set_parameter(
                        (*(*codec).internal).svt_encoder,
                        svt_config,
                    );
                    if res as core::ffi::c_int
                        == EB_ErrorBadParameter as core::ffi::c_int
                    {
                        current_block = 0;
                    } else {
                        res = svt_av1_enc_init((*(*codec).internal).svt_encoder);
                        if res as core::ffi::c_int != EB_ErrorNone as core::ffi::c_int {
                            current_block = 0;
                        } else {
                            current_block = 2;
                        }
                    }
                }
            }
        }
    } else {
        current_block = 2;
    }
    match current_block {
        2 => {
            if !(allocate_svt_buffers(&mut input_buffer) == 0) {
                input_picture_buffer = (*input_buffer).p_buffer as *mut EbSvtIOFormat;
                bytesPerPixel = (if (*image).depth > 8 as uint32_t {
                    2 as core::ffi::c_int
                } else {
                    1 as core::ffi::c_int
                }) as uint32_t;
                uvHeight = ((*image).height).wrapping_add(y_shift as uint32_t)
                    >> y_shift;
                if alpha != 0 {
                    (*input_picture_buffer).y_stride = ((*image).alphaRowBytes)
                        .wrapping_div(bytesPerPixel);
                    (*input_picture_buffer).luma = (*image).alphaPlane;
                    let alphaSize: size_t = ((*image).alphaRowBytes as size_t)
                        .wrapping_mul((*image).height as size_t);
                    if alphaSize > UINT32_MAX as size_t {
                        current_block = 0;
                    } else {
                        (*input_buffer).n_filled_len = alphaSize as uint32_t;
                        let uvWidth: uint32_t = ((*image).width)
                            .wrapping_add(y_shift as uint32_t) >> y_shift;
                        let uvRowBytes: uint32_t = uvWidth.wrapping_mul(bytesPerPixel);
                        let uvSize: size_t = (uvRowBytes as size_t)
                            .wrapping_mul(uvHeight as size_t);
                        if uvSize
                            > UINT32_MAX.wrapping_div(2 as core::ffi::c_uint) as size_t
                        {
                            current_block = 0;
                        } else if uvSize.wrapping_mul(2 as size_t)
                            > (UINT32_MAX as uint32_t)
                                .wrapping_sub((*input_buffer).n_filled_len) as size_t
                        {
                            current_block = 0;
                        } else {
                            uvPlanes = avifCalloc(
                                uvSize,
                                ::core::mem::size_of::<uint8_t>() as size_t,
                            ) as *mut uint8_t;
                            if uvPlanes.is_null() {
                                current_block = 0;
                            } else {
                                (*input_picture_buffer).cb = uvPlanes;
                                (*input_buffer).n_filled_len = ((*input_buffer)
                                    .n_filled_len)
                                    .wrapping_add(uvSize as uint32_t);
                                (*input_picture_buffer).cr = uvPlanes;
                                (*input_buffer).n_filled_len = ((*input_buffer)
                                    .n_filled_len)
                                    .wrapping_add(uvSize as uint32_t);
                                (*input_picture_buffer).cb_stride = uvWidth;
                                (*input_picture_buffer).cr_stride = uvWidth;
                                current_block = 3;
                            }
                        }
                    }
                } else {
                    (*input_picture_buffer).y_stride = ((*image)
                        .yuvRowBytes[0 as core::ffi::c_int as usize])
                        .wrapping_div(bytesPerPixel);
                    (*input_picture_buffer).luma = (*image)
                        .yuvPlanes[0 as core::ffi::c_int as usize];
                    let ySize: size_t = ((*image)
                        .yuvRowBytes[0 as core::ffi::c_int as usize] as size_t)
                        .wrapping_mul((*image).height as size_t);
                    if ySize > UINT32_MAX as size_t {
                        current_block = 0;
                    } else {
                        (*input_buffer).n_filled_len = ySize as uint32_t;
                        (*input_picture_buffer).cb = (*image)
                            .yuvPlanes[1 as core::ffi::c_int as usize];
                        let uSize: size_t = ((*image)
                            .yuvRowBytes[1 as core::ffi::c_int as usize] as size_t)
                            .wrapping_mul(uvHeight as size_t);
                        if uSize
                            > (UINT32_MAX as uint32_t)
                                .wrapping_sub((*input_buffer).n_filled_len) as size_t
                        {
                            current_block = 0;
                        } else {
                            (*input_buffer).n_filled_len = ((*input_buffer).n_filled_len)
                                .wrapping_add(uSize as uint32_t);
                            (*input_picture_buffer).cr = (*image)
                                .yuvPlanes[2 as core::ffi::c_int as usize];
                            let vSize: size_t = ((*image)
                                .yuvRowBytes[2 as core::ffi::c_int as usize] as size_t)
                                .wrapping_mul(uvHeight as size_t);
                            if vSize
                                > (UINT32_MAX as uint32_t)
                                    .wrapping_sub((*input_buffer).n_filled_len) as size_t
                            {
                                current_block = 0;
                            } else {
                                (*input_buffer).n_filled_len = ((*input_buffer)
                                    .n_filled_len)
                                    .wrapping_add(vSize as uint32_t);
                                (*input_picture_buffer).cb_stride = ((*image)
                                    .yuvRowBytes[1 as core::ffi::c_int as usize])
                                    .wrapping_div(bytesPerPixel);
                                (*input_picture_buffer).cr_stride = ((*image)
                                    .yuvRowBytes[2 as core::ffi::c_int as usize])
                                    .wrapping_div(bytesPerPixel);
                                current_block = 3;
                            }
                        }
                    }
                }
                match current_block {
                    0 => {}
                    _ => {
                        (*input_buffer).flags = 0 as uint32_t;
                        (*input_buffer).pts = 0 as int64_t;
                        frame_type = EB_AV1_INVALID_PICTURE;
                        if addImageFlags
                            & AVIF_ADD_IMAGE_FLAG_FORCE_KEYFRAME as core::ffi::c_int
                                as uint32_t != 0
                            || (*encoder).keyframeInterval == 1 as core::ffi::c_int
                        {
                            frame_type = EB_AV1_KEY_PICTURE;
                        }
                        (*input_buffer).pic_type = frame_type;
                        res = svt_av1_enc_send_picture(
                            (*(*codec).internal).svt_encoder,
                            input_buffer,
                        );
                        if !(res as core::ffi::c_int != EB_ErrorNone as core::ffi::c_int)
                        {
                            result = dequeue_frame(codec, output, AVIF_FALSE);
                        }
                    }
                }
            }
        }
        _ => {}
    }
    if !uvPlanes.is_null() {
        avifFree(uvPlanes as *mut core::ffi::c_void);
    }
    if !input_buffer.is_null() {
        if !((*input_buffer).p_buffer).is_null() {
            avifFree((*input_buffer).p_buffer as *mut core::ffi::c_void);
        }
        avifFree(input_buffer as *mut core::ffi::c_void);
    }
    return result;
}
unsafe extern "C" fn svtCodecEncodeFinish(
    mut codec: *mut avifCodec,
    mut output: *mut avifCodecEncodeOutput,
) -> avifBool {
    let mut ret: EbErrorType = EB_ErrorNone;
    let mut input_buffer: EbBufferHeaderType = EbBufferHeaderType {
        size: 0,
        p_buffer: 0 as *mut uint8_t,
        n_filled_len: 0,
        n_alloc_len: 0,
        p_app_private: 0 as *mut core::ffi::c_void,
        wrapper_ptr: 0 as *mut core::ffi::c_void,
        n_tick_count: 0,
        dts: 0,
        pts: 0,
        temporal_layer_index: 0,
        qp: 0,
        avg_qp: 0,
        pic_type: EB_AV1_INTER_PICTURE,
        luma_sse: 0,
        cr_sse: 0,
        cb_sse: 0,
        flags: 0,
        luma_ssim: 0.,
        cr_ssim: 0.,
        cb_ssim: 0.,
        metadata: 0 as *mut SvtMetadataArray,
    };
    input_buffer.n_alloc_len = 0 as uint32_t;
    input_buffer.n_filled_len = 0 as uint32_t;
    input_buffer.n_tick_count = 0 as uint32_t;
    input_buffer.p_app_private = NULL;
    input_buffer.flags = EB_BUFFERFLAG_EOS as uint32_t;
    input_buffer.p_buffer = 0 as *mut uint8_t;
    input_buffer.metadata = 0 as *mut SvtMetadataArray;
    ret = svt_av1_enc_send_picture((*(*codec).internal).svt_encoder, &mut input_buffer);
    if ret as core::ffi::c_int != EB_ErrorNone as core::ffi::c_int {
        return AVIF_FALSE;
    }
    return (dequeue_frame(codec, output, AVIF_TRUE) as core::ffi::c_uint
        == AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint) as core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn avifCodecVersionSvt() -> *const core::ffi::c_char {
    return svt_av1_get_version();
}
unsafe extern "C" fn svtCodecDestroyInternal(mut codec: *mut avifCodec) {
    if !((*(*codec).internal).svt_encoder).is_null() {
        svt_av1_enc_deinit((*(*codec).internal).svt_encoder);
        svt_av1_enc_deinit_handle((*(*codec).internal).svt_encoder);
        (*(*codec).internal).svt_encoder = 0 as *mut EbComponentType;
    }
    avifFree((*codec).internal as *mut core::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn avifCodecCreateSvt() -> *mut avifCodec {
    let mut codec: *mut avifCodec = avifCalloc(
        1 as size_t,
        ::core::mem::size_of::<avifCodec>() as size_t,
    ) as *mut avifCodec;
    if codec.is_null() {
        return 0 as *mut avifCodec;
    }
    (*codec).encodeImage = Some(
        svtCodecEncodeImage
            as unsafe extern "C" fn(
                *mut avifCodec,
                *mut avifEncoder,
                *const avifImage,
                avifBool,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
                avifEncoderChanges,
                avifBool,
                uint32_t,
                *mut avifCodecEncodeOutput,
            ) -> avifResult,
    ) as avifCodecEncodeImageFunc;
    (*codec).encodeFinish = Some(
        svtCodecEncodeFinish
            as unsafe extern "C" fn(
                *mut avifCodec,
                *mut avifCodecEncodeOutput,
            ) -> avifBool,
    ) as avifCodecEncodeFinishFunc;
    (*codec).destroyInternal = Some(
        svtCodecDestroyInternal as unsafe extern "C" fn(*mut avifCodec) -> (),
    ) as avifCodecDestroyInternalFunc;
    (*codec).internal = avifCalloc(
        1 as size_t,
        ::core::mem::size_of::<avifCodecInternal>() as size_t,
    ) as *mut avifCodecInternal;
    if ((*codec).internal).is_null() {
        avifFree(codec as *mut core::ffi::c_void);
        return 0 as *mut avifCodec;
    }
    return codec;
}
unsafe extern "C" fn allocate_svt_buffers(
    mut input_buf: *mut *mut EbBufferHeaderType,
) -> avifBool {
    *input_buf = avifAlloc(::core::mem::size_of::<EbBufferHeaderType>() as size_t)
        as *mut EbBufferHeaderType;
    if (*input_buf).is_null() {
        return AVIF_FALSE;
    }
    (**input_buf).p_buffer = avifCalloc(
        1 as size_t,
        ::core::mem::size_of::<EbSvtIOFormat>() as size_t,
    ) as *mut uint8_t;
    if ((**input_buf).p_buffer).is_null() {
        return AVIF_FALSE;
    }
    (**input_buf).size = ::core::mem::size_of::<EbBufferHeaderType>() as uint32_t;
    (**input_buf).p_app_private = NULL;
    (**input_buf).pic_type = EB_AV1_INVALID_PICTURE;
    (**input_buf).metadata = 0 as *mut SvtMetadataArray;
    return AVIF_TRUE;
}
unsafe extern "C" fn dequeue_frame(
    mut codec: *mut avifCodec,
    mut output: *mut avifCodecEncodeOutput,
    mut done_sending_pics: avifBool,
) -> avifResult {
    let mut res: EbErrorType = EB_ErrorNone;
    let mut encode_at_eos: core::ffi::c_int = 0 as core::ffi::c_int;
    loop {
        let mut output_buf: *mut EbBufferHeaderType = 0 as *mut EbBufferHeaderType;
        res = svt_av1_enc_get_packet(
            (*(*codec).internal).svt_encoder,
            &mut output_buf,
            done_sending_pics as uint8_t,
        );
        if !output_buf.is_null() {
            encode_at_eos = ((*output_buf).flags & EB_BUFFERFLAG_EOS as uint32_t
                == EB_BUFFERFLAG_EOS as uint32_t) as core::ffi::c_int;
            if !((*output_buf).p_buffer).is_null()
                && (*output_buf).n_filled_len > 0 as uint32_t
            {
                let result: avifResult = avifCodecEncodeOutputAddSample(
                    output,
                    (*output_buf).p_buffer,
                    (*output_buf).n_filled_len as size_t,
                    ((*output_buf).pic_type as core::ffi::c_uint
                        == EB_AV1_KEY_PICTURE as core::ffi::c_int as core::ffi::c_uint)
                        as core::ffi::c_int,
                ) as avifResult;
                if result as core::ffi::c_uint
                    != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
                {
                    svt_av1_enc_release_out_buffer(&mut output_buf);
                    return result;
                }
            }
            svt_av1_enc_release_out_buffer(&mut output_buf);
        }
        output_buf = 0 as *mut EbBufferHeaderType;
        if !(res as core::ffi::c_int == EB_ErrorNone as core::ffi::c_int
            && encode_at_eos == 0)
        {
            break;
        }
    }
    if done_sending_pics == 0
        && (res as core::ffi::c_int == EB_ErrorNone as core::ffi::c_int
            || res as core::ffi::c_int == EB_NoErrorEmptyQueue as core::ffi::c_int)
    {
        return AVIF_RESULT_OK;
    }
    return (if res as core::ffi::c_int == EB_ErrorNone as core::ffi::c_int {
        AVIF_RESULT_OK as core::ffi::c_int
    } else {
        AVIF_RESULT_UNKNOWN_ERROR as core::ffi::c_int
    }) as avifResult;
}
