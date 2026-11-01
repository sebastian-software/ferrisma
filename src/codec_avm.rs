// Copyright 2023 Google LLC
// SPDX-License-Identifier: BSD-2-Clause

// Mechanically translated from upstream/libavif/src/codec_avm.c.
// C ABI, ownership, pointer validity, and bounds follow the original headers.
extern "C" {
    pub type avifEncoderData;
    pub type avm_codec_priv;
    pub type avm_codec_iface;
    pub type avm_metadata_array;
    fn avifAlloc(size: size_t) -> *mut core::ffi::c_void;
    fn avifFree(p: *mut core::ffi::c_void);
    fn avifResultToString(result: avifResult) -> *const core::ffi::c_char;
    fn avifGetPixelFormatInfo(format: avifPixelFormat, info: *mut avifPixelFormatInfo);
    fn avifImageAllocatePlanes(
        image: *mut avifImage,
        planes: avifPlanesFlags,
    ) -> avifResult;
    fn avifImageFreePlanes(image: *mut avifImage, planes: avifPlanesFlags);
    fn avifImageUsesU16(image: *const avifImage) -> avifBool;
    fn avifImagePlane(
        image: *const avifImage,
        channel: core::ffi::c_int,
    ) -> *mut uint8_t;
    fn avifImagePlaneRowBytes(
        image: *const avifImage,
        channel: core::ffi::c_int,
    ) -> uint32_t;
    fn avifImagePlaneWidth(
        image: *const avifImage,
        channel: core::ffi::c_int,
    ) -> uint32_t;
    fn avifImagePlaneHeight(
        image: *const avifImage,
        channel: core::ffi::c_int,
    ) -> uint32_t;
    fn avifCalloc(count: size_t, size: size_t) -> *mut core::ffi::c_void;
    fn avifFractionSimplify(f: *mut avifFraction);
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
    fn avm_img_alloc(
        img: *mut avm_image_t,
        fmt: avm_img_fmt_t,
        d_w: core::ffi::c_uint,
        d_h: core::ffi::c_uint,
        align: core::ffi::c_uint,
    ) -> *mut avm_image_t;
    fn avm_img_free(img: *mut avm_image_t);
    fn avm_codec_version_str() -> *const core::ffi::c_char;
    fn avm_codec_err_to_string(err: avm_codec_err_t) -> *const core::ffi::c_char;
    fn avm_codec_error(ctx: *mut avm_codec_ctx_t) -> *const core::ffi::c_char;
    fn avm_codec_error_detail(ctx: *mut avm_codec_ctx_t) -> *const core::ffi::c_char;
    fn avm_codec_destroy(ctx: *mut avm_codec_ctx_t) -> avm_codec_err_t;
    fn avm_codec_control(
        ctx: *mut avm_codec_ctx_t,
        ctrl_id: core::ffi::c_int,
        ...
    ) -> avm_codec_err_t;
    fn avm_codec_set_option(
        ctx: *mut avm_codec_ctx_t,
        name: *const core::ffi::c_char,
        value: *const core::ffi::c_char,
    ) -> avm_codec_err_t;
    fn avm_codec_dec_init_ver(
        ctx: *mut avm_codec_ctx_t,
        iface: *const avm_codec_iface_t,
        cfg: *const avm_codec_dec_cfg_t,
        flags: avm_codec_flags_t,
        ver: core::ffi::c_int,
    ) -> avm_codec_err_t;
    fn avm_codec_decode(
        ctx: *mut avm_codec_ctx_t,
        data: *const uint8_t,
        data_sz: size_t,
        user_priv: *mut core::ffi::c_void,
    ) -> avm_codec_err_t;
    fn avm_codec_get_frame(
        ctx: *mut avm_codec_ctx_t,
        iter: *mut avm_codec_iter_t,
    ) -> *mut avm_image_t;
    fn avm_codec_enc_init_ver(
        ctx: *mut avm_codec_ctx_t,
        iface: *const avm_codec_iface_t,
        cfg: *const avm_codec_enc_cfg_t,
        flags: avm_codec_flags_t,
        ver: core::ffi::c_int,
    ) -> avm_codec_err_t;
    fn avm_codec_enc_config_default(
        iface: *const avm_codec_iface_t,
        cfg: *mut avm_codec_enc_cfg_t,
        usage: core::ffi::c_uint,
    ) -> avm_codec_err_t;
    fn avm_codec_enc_config_set(
        ctx: *mut avm_codec_ctx_t,
        cfg: *const avm_codec_enc_cfg_t,
    ) -> avm_codec_err_t;
    fn avm_codec_encode(
        ctx: *mut avm_codec_ctx_t,
        img: *const avm_image_t,
        pts: avm_codec_pts_t,
        duration: core::ffi::c_ulong,
        flags: avm_enc_frame_flags_t,
    ) -> avm_codec_err_t;
    fn avm_codec_get_cx_data(
        ctx: *mut avm_codec_ctx_t,
        iter: *mut avm_codec_iter_t,
    ) -> *const avm_codec_cx_pkt_t;
    fn avm_codec_av2_cx() -> *const avm_codec_iface_t;
    fn avm_codec_av2_dx() -> *const avm_codec_iface_t;
    fn strtol(
        __str: *const core::ffi::c_char,
        __endptr: *mut *mut core::ffi::c_char,
        __base: core::ffi::c_int,
    ) -> core::ffi::c_long;
    fn memcpy(
        __dst: *mut core::ffi::c_void,
        __src: *const core::ffi::c_void,
        __n: size_t,
    ) -> *mut core::ffi::c_void;
    fn memset(
        __b: *mut core::ffi::c_void,
        __c: core::ffi::c_int,
        __len: size_t,
    ) -> *mut core::ffi::c_void;
    fn strcmp(
        __s1: *const core::ffi::c_char,
        __s2: *const core::ffi::c_char,
    ) -> core::ffi::c_int;
    fn strncmp(
        __s1: *const core::ffi::c_char,
        __s2: *const core::ffi::c_char,
        __n: size_t,
    ) -> core::ffi::c_int;
}
pub type size_t = usize;
pub type int32_t = i32;
pub type int64_t = i64;
pub type uint8_t = u8;
pub type uint16_t = u16;
pub type uint32_t = u32;
pub type uint64_t = u64;
pub type avifBool = core::ffi::c_int;
pub type avifPlanesFlag = core::ffi::c_uint;
pub const AVIF_PLANES_ALL: avifPlanesFlag = 255;
pub const AVIF_PLANES_A: avifPlanesFlag = 2;
pub const AVIF_PLANES_YUV: avifPlanesFlag = 1;
pub type avifPlanesFlags = uint32_t;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct avifPixelFormatInfo {
    pub monochrome: avifBool,
    pub chromaShiftX: core::ffi::c_int,
    pub chromaShiftY: core::ffi::c_int,
}
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
    pub decoderInitialized: avifBool,
    pub decoder: avm_codec_ctx_t,
    pub iter: avm_codec_iter_t,
    pub image: *mut avm_image_t,
    pub encoderInitialized: avifBool,
    pub encoder: avm_codec_ctx_t,
    pub cfg: avm_codec_enc_cfg,
    pub formatInfo: avifPixelFormatInfo,
    pub avmFormat: avm_img_fmt_t,
    pub monochromeEnabled: avifBool,
    pub tuningSet: avifBool,
    pub currentLayer: uint32_t,
}
pub type avm_img_fmt_t = avm_img_fmt;
pub type avm_img_fmt = core::ffi::c_uint;
pub const AVM_IMG_FMT_I44416: avm_img_fmt = 2308;
pub const AVM_IMG_FMT_I42216: avm_img_fmt = 2307;
pub const AVM_IMG_FMT_YV1216: avm_img_fmt = 2817;
pub const AVM_IMG_FMT_I42016: avm_img_fmt = 2306;
pub const AVM_IMG_FMT_I444: avm_img_fmt = 260;
pub const AVM_IMG_FMT_I422: avm_img_fmt = 259;
pub const AVM_IMG_FMT_I420: avm_img_fmt = 258;
pub const AVM_IMG_FMT_YV12: avm_img_fmt = 769;
pub const AVM_IMG_FMT_NONE: avm_img_fmt = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct avm_codec_enc_cfg {
    pub g_usage: core::ffi::c_uint,
    pub g_threads: core::ffi::c_uint,
    pub g_profile: core::ffi::c_uint,
    pub g_w: core::ffi::c_uint,
    pub g_h: core::ffi::c_uint,
    pub g_limit: core::ffi::c_uint,
    pub g_forced_max_frame_width: core::ffi::c_uint,
    pub g_forced_max_frame_height: core::ffi::c_uint,
    pub g_bit_depth: avm_bit_depth_t,
    pub g_input_bit_depth: core::ffi::c_uint,
    pub g_timebase: avm_rational,
    pub g_error_resilient: avm_codec_er_flags_t,
    pub g_pass: avm_enc_pass,
    pub g_lag_in_frames: core::ffi::c_uint,
    pub rc_dropframe_thresh: core::ffi::c_uint,
    pub rc_resize_mode: core::ffi::c_uint,
    pub rc_resize_denominator: core::ffi::c_uint,
    pub rc_resize_kf_denominator: core::ffi::c_uint,
    pub rc_end_usage: avm_rc_mode,
    pub rc_firstpass_mb_stats_in: avm_fixed_buf_t,
    pub rc_target_bitrate: core::ffi::c_uint,
    pub rc_min_quantizer: core::ffi::c_int,
    pub rc_max_quantizer: core::ffi::c_int,
    pub rc_undershoot_pct: core::ffi::c_uint,
    pub rc_overshoot_pct: core::ffi::c_uint,
    pub rc_buf_sz: core::ffi::c_uint,
    pub rc_buf_initial_sz: core::ffi::c_uint,
    pub rc_buf_optimal_sz: core::ffi::c_uint,
    pub rc_2pass_vbr_minsection_pct: core::ffi::c_uint,
    pub rc_2pass_vbr_maxsection_pct: core::ffi::c_uint,
    pub fwd_kf_enabled: core::ffi::c_int,
    pub kf_mode: avm_kf_mode,
    pub kf_min_dist: core::ffi::c_uint,
    pub kf_max_dist: core::ffi::c_uint,
    pub enable_sframe: core::ffi::c_uint,
    pub sframe_dist: core::ffi::c_uint,
    pub sframe_mode: core::ffi::c_uint,
    pub sframe_type: core::ffi::c_uint,
    pub monochrome: core::ffi::c_uint,
    pub full_still_picture_hdr: core::ffi::c_uint,
    pub enable_tcq: core::ffi::c_uint,
    pub enable_lcr: core::ffi::c_uint,
    pub enable_ops: core::ffi::c_uint,
    pub num_ops: core::ffi::c_uint,
    pub enable_atlas: core::ffi::c_uint,
    pub tile_width_count: core::ffi::c_int,
    pub tile_height_count: core::ffi::c_int,
    pub tile_widths: [core::ffi::c_int; 64],
    pub tile_heights: [core::ffi::c_int; 64],
    pub use_fixed_qp_offsets: core::ffi::c_uint,
    pub fixed_qp_offsets: [core::ffi::c_int; 6],
    pub frame_hash_metadata: avm_frame_hash_mode,
    pub frame_hash_per_plane: core::ffi::c_uint,
    pub use_short_metadata: core::ffi::c_uint,
    pub encoder_cfg: cfg_options_t,
}
pub type cfg_options_t = cfg_options;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cfg_options {
    pub init_by_cfg_file: core::ffi::c_uint,
    pub superblock_size: core::ffi::c_uint,
    pub max_partition_size: core::ffi::c_uint,
    pub min_partition_size: core::ffi::c_uint,
    pub enable_rect_partitions: core::ffi::c_uint,
    pub enable_uneven_4way_partitions: core::ffi::c_uint,
    pub disable_ml_partition_speed_features: core::ffi::c_uint,
    pub erp_pruning_level: core::ffi::c_uint,
    pub use_ml_erp_pruning: core::ffi::c_uint,
    pub enable_ext_partitions: core::ffi::c_uint,
    pub enable_tx_partition: core::ffi::c_uint,
    pub max_partition_aspect_ratio: core::ffi::c_uint,
    pub disable_ml_transform_speed_features: core::ffi::c_uint,
    pub enable_sdp: core::ffi::c_uint,
    pub enable_extended_sdp: core::ffi::c_uint,
    pub enable_mrls: core::ffi::c_uint,
    pub enable_tip: core::ffi::c_uint,
    pub enable_tip_refinemv: core::ffi::c_uint,
    pub enable_mv_traj: core::ffi::c_uint,
    pub enable_high_motion: core::ffi::c_uint,
    pub enable_bawp: core::ffi::c_uint,
    pub enable_cwp: core::ffi::c_uint,
    pub enable_imp_msk_bld: core::ffi::c_uint,
    pub enable_fsc: core::ffi::c_uint,
    pub enable_idtx_intra: core::ffi::c_uint,
    pub enable_ist: core::ffi::c_uint,
    pub enable_inter_ist: core::ffi::c_uint,
    pub enable_chroma_dctonly: core::ffi::c_uint,
    pub enable_inter_ddt: core::ffi::c_uint,
    pub enable_cctx: core::ffi::c_uint,
    pub enable_ibp: core::ffi::c_uint,
    pub enable_adaptive_mvd: core::ffi::c_uint,
    pub enable_flex_mvres: core::ffi::c_uint,
    pub select_cfl_ds_filter: core::ffi::c_uint,
    pub enable_joint_mvd: core::ffi::c_uint,
    pub enable_refinemv: core::ffi::c_uint,
    pub enable_mvd_sign_derive: core::ffi::c_uint,
    pub enable_flip_idtx: core::ffi::c_uint,
    pub enable_deblocking: core::ffi::c_uint,
    pub enable_cdef: core::ffi::c_uint,
    pub enable_gdf: core::ffi::c_uint,
    pub gdf_unit_matches_sb: core::ffi::c_uint,
    pub enable_restoration: core::ffi::c_uint,
    pub enable_pc_wiener: core::ffi::c_uint,
    pub enable_wiener_nonsep: core::ffi::c_uint,
    pub enable_ccso: core::ffi::c_uint,
    pub ccso_unit_matches_sb: core::ffi::c_uint,
    pub enable_band_metadata: core::ffi::c_uint,
    pub enable_lf_sub_pu: core::ffi::c_uint,
    pub enable_warped_motion: core::ffi::c_uint,
    pub enable_warp_causal: core::ffi::c_uint,
    pub enable_warp_delta: core::ffi::c_uint,
    pub enable_six_param_warp_delta: core::ffi::c_uint,
    pub enable_warp_extend: core::ffi::c_uint,
    pub enable_global_motion: core::ffi::c_uint,
    pub enable_skip_mode: core::ffi::c_uint,
    pub enable_diff_wtd_comp: core::ffi::c_uint,
    pub enable_interintra_comp: core::ffi::c_uint,
    pub enable_masked_comp: core::ffi::c_uint,
    pub enable_onesided_comp: core::ffi::c_uint,
    pub enable_palette: core::ffi::c_uint,
    pub enable_intrabc: core::ffi::c_uint,
    pub enable_intrabc_ext: core::ffi::c_uint,
    pub enable_cfl_intra: core::ffi::c_uint,
    pub enable_mhccp: core::ffi::c_uint,
    pub enable_smooth_intra: core::ffi::c_uint,
    pub enable_intra_dip: core::ffi::c_uint,
    pub enable_angle_delta: core::ffi::c_uint,
    pub enable_opfl_refine: avm_opfl_refine_type,
    pub enable_intra_edge_filter: core::ffi::c_uint,
    pub reduced_tx_part_set: core::ffi::c_uint,
    pub enable_smooth_interintra: core::ffi::c_uint,
    pub enable_interinter_wedge: core::ffi::c_uint,
    pub enable_interintra_wedge: core::ffi::c_uint,
    pub enable_paeth_intra: core::ffi::c_uint,
    pub enable_trellis_quant: core::ffi::c_uint,
    pub enable_ref_frame_mvs: core::ffi::c_uint,
    pub reduced_ref_frame_mvs_mode: core::ffi::c_uint,
    pub enable_reduced_reference_set: core::ffi::c_uint,
    pub explicit_ref_frame_map: core::ffi::c_uint,
    pub add_sef_for_hidden_frames: core::ffi::c_uint,
    pub monotonic_output_order: core::ffi::c_uint,
    pub reduced_tx_type_set: core::ffi::c_uint,
    pub max_drl_refmvs: core::ffi::c_uint,
    pub max_drl_refbvs: core::ffi::c_uint,
    pub enable_refmvbank: core::ffi::c_uint,
    pub enable_drl_reorder: core::ffi::c_uint,
    pub enable_cdef_on_skip_txfm: core::ffi::c_uint,
    pub enable_avg_cdf: core::ffi::c_uint,
    pub avg_cdf_type: core::ffi::c_uint,
    pub enable_parity_hiding: core::ffi::c_uint,
    pub enable_short_refresh_frame_flags: core::ffi::c_uint,
    pub enable_ext_seg: core::ffi::c_uint,
    pub dpb_size: core::ffi::c_int,
    pub enable_bru: core::ffi::c_uint,
    pub disable_loopfilters_across_tiles: core::ffi::c_uint,
    pub enable_cropping_window: core::ffi::c_int,
    pub crop_win_left_offset: core::ffi::c_int,
    pub crop_win_right_offset: core::ffi::c_int,
    pub crop_win_top_offset: core::ffi::c_int,
    pub crop_win_bottom_offset: core::ffi::c_int,
    pub icc_data: *mut uint8_t,
    pub icc_size: size_t,
    pub scan_type_info_present_flag: core::ffi::c_uint,
    pub enable_mfh_obu_signaling: core::ffi::c_uint,
    pub operating_points_count: core::ffi::c_int,
}
pub type avm_opfl_refine_type = core::ffi::c_uint;
pub const AVM_OPFL_REFINE_AUTO: avm_opfl_refine_type = 3;
pub const AVM_OPFL_REFINE_ALL: avm_opfl_refine_type = 2;
pub const AVM_OPFL_REFINE_SWITCHABLE: avm_opfl_refine_type = 1;
pub const AVM_OPFL_REFINE_NONE: avm_opfl_refine_type = 0;
pub type avm_frame_hash_mode = core::ffi::c_uint;
pub const AVM_DFH_BOTH: avm_frame_hash_mode = 3;
pub const AVM_DFH_FG: avm_frame_hash_mode = 2;
pub const AVM_DFH_RAW: avm_frame_hash_mode = 1;
pub const AVM_DFH_DISABLED: avm_frame_hash_mode = 0;
pub type avm_kf_mode = core::ffi::c_uint;
pub const AVM_KF_DISABLED: avm_kf_mode = 0;
pub const AVM_KF_AUTO: avm_kf_mode = 1;
pub const AVM_KF_FIXED: avm_kf_mode = 0;
pub type avm_fixed_buf_t = avm_fixed_buf;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct avm_fixed_buf {
    pub buf: *mut core::ffi::c_void,
    pub sz: size_t,
}
pub type avm_rc_mode = core::ffi::c_uint;
pub const AVM_Q: avm_rc_mode = 3;
pub const AVM_CQ: avm_rc_mode = 2;
pub const AVM_CBR: avm_rc_mode = 1;
pub const AVM_VBR: avm_rc_mode = 0;
pub type avm_enc_pass = core::ffi::c_uint;
pub const AVM_RC_LAST_PASS: avm_enc_pass = 2;
pub const AVM_RC_FIRST_PASS: avm_enc_pass = 1;
pub const AVM_RC_ONE_PASS: avm_enc_pass = 0;
pub type avm_codec_er_flags_t = uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct avm_rational {
    pub num: core::ffi::c_int,
    pub den: core::ffi::c_int,
}
pub type avm_bit_depth_t = avm_bit_depth;
pub type avm_bit_depth = core::ffi::c_uint;
pub const AVM_BITS_12: avm_bit_depth = 12;
pub const AVM_BITS_10: avm_bit_depth = 10;
pub const AVM_BITS_8: avm_bit_depth = 8;
pub type avm_codec_ctx_t = avm_codec_ctx;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct avm_codec_ctx {
    pub name: *const core::ffi::c_char,
    pub iface: *const avm_codec_iface_t,
    pub err: avm_codec_err_t,
    pub err_detail: *const core::ffi::c_char,
    pub init_flags: avm_codec_flags_t,
    pub config: C2RustUnnamed,
    pub priv_0: *mut avm_codec_priv_t,
}
pub type avm_codec_priv_t = avm_codec_priv;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub dec: *const avm_codec_dec_cfg,
    pub enc: *const avm_codec_enc_cfg,
    pub raw: *const core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct avm_codec_dec_cfg {
    pub threads: core::ffi::c_uint,
    pub w: core::ffi::c_uint,
    pub h: core::ffi::c_uint,
    pub path_parakit: *mut core::ffi::c_char,
    pub suffix_parakit: *mut core::ffi::c_char,
}
pub type avm_codec_flags_t = core::ffi::c_long;
pub type avm_codec_err_t = core::ffi::c_uint;
pub const AVM_CODEC_LIST_END: avm_codec_err_t = 9;
pub const AVM_CODEC_INVALID_PARAM: avm_codec_err_t = 8;
pub const AVM_CODEC_CORRUPT_FRAME: avm_codec_err_t = 7;
pub const AVM_CODEC_UNSUP_FEATURE: avm_codec_err_t = 6;
pub const AVM_CODEC_UNSUP_BITSTREAM: avm_codec_err_t = 5;
pub const AVM_CODEC_INCAPABLE: avm_codec_err_t = 4;
pub const AVM_CODEC_ABI_MISMATCH: avm_codec_err_t = 3;
pub const AVM_CODEC_MEM_ERROR: avm_codec_err_t = 2;
pub const AVM_CODEC_ERROR: avm_codec_err_t = 1;
pub const AVM_CODEC_OK: avm_codec_err_t = 0;
pub type avm_codec_iface_t = avm_codec_iface;
pub type avm_image_t = avm_image;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct avm_image {
    pub fmt: avm_img_fmt_t,
    pub cp: avm_color_primaries_t,
    pub tc: avm_transfer_characteristics_t,
    pub mc: avm_matrix_coefficients_t,
    pub monochrome: core::ffi::c_int,
    pub csp: avm_chroma_sample_position_t,
    pub range: avm_color_range_t,
    pub w: core::ffi::c_uint,
    pub h: core::ffi::c_uint,
    pub bit_depth: core::ffi::c_uint,
    pub w_conf_win_enabled_flag: core::ffi::c_int,
    pub w_conf_win_left_offset: core::ffi::c_int,
    pub w_conf_win_right_offset: core::ffi::c_int,
    pub w_conf_win_top_offset: core::ffi::c_int,
    pub w_conf_win_bottom_offset: core::ffi::c_int,
    pub max_width: core::ffi::c_int,
    pub max_height: core::ffi::c_int,
    pub crop_width: core::ffi::c_int,
    pub crop_height: core::ffi::c_int,
    pub d_w: core::ffi::c_uint,
    pub d_h: core::ffi::c_uint,
    pub r_w: core::ffi::c_uint,
    pub r_h: core::ffi::c_uint,
    pub x_chroma_shift: core::ffi::c_uint,
    pub y_chroma_shift: core::ffi::c_uint,
    pub planes: [*mut core::ffi::c_uchar; 3],
    pub stride: [core::ffi::c_int; 3],
    pub sz: size_t,
    pub bps: core::ffi::c_int,
    pub tlayer_id: core::ffi::c_int,
    pub mlayer_id: core::ffi::c_int,
    pub xlayer_id: core::ffi::c_int,
    pub stream_id: core::ffi::c_int,
    pub user_priv: *mut core::ffi::c_void,
    pub img_data: *mut core::ffi::c_uchar,
    pub img_data_owner: core::ffi::c_int,
    pub self_allocd: core::ffi::c_int,
    pub metadata: *mut avm_metadata_array_t,
    pub fb_priv: *mut core::ffi::c_void,
}
pub type avm_metadata_array_t = avm_metadata_array;
pub type avm_color_range_t = avm_color_range;
pub type avm_color_range = core::ffi::c_uint;
pub const AVM_CR_FULL_RANGE: avm_color_range = 1;
pub const AVM_CR_STUDIO_RANGE: avm_color_range = 0;
pub type avm_chroma_sample_position_t = avm_chroma_sample_position;
pub type avm_chroma_sample_position = core::ffi::c_uint;
pub const AVM_CSP_BOTTOM: avm_chroma_sample_position = 5;
pub const AVM_CSP_BOTTOMLEFT: avm_chroma_sample_position = 4;
pub const AVM_CSP_TOP: avm_chroma_sample_position = 3;
pub const AVM_CSP_TOPLEFT: avm_chroma_sample_position = 2;
pub const AVM_CSP_CENTER: avm_chroma_sample_position = 1;
pub const AVM_CSP_LEFT: avm_chroma_sample_position = 0;
pub const AVM_CSP_UNSPECIFIED: avm_chroma_sample_position = 6;
pub type avm_matrix_coefficients_t = avm_matrix_coefficients;
pub type avm_matrix_coefficients = core::ffi::c_uint;
pub const AVM_CICP_MC_RESERVED_18: avm_matrix_coefficients = 18;
pub const AVM_CICP_MC_YCGCO_RO: avm_matrix_coefficients = 17;
pub const AVM_CICP_MC_YCGCO_RE: avm_matrix_coefficients = 16;
pub const AVM_CICP_MC_IPT_C2: avm_matrix_coefficients = 15;
pub const AVM_CICP_MC_ICTCP: avm_matrix_coefficients = 14;
pub const AVM_CICP_MC_CHROMAT_CL: avm_matrix_coefficients = 13;
pub const AVM_CICP_MC_CHROMAT_NCL: avm_matrix_coefficients = 12;
pub const AVM_CICP_MC_SMPTE_2085: avm_matrix_coefficients = 11;
pub const AVM_CICP_MC_BT_2020_CL: avm_matrix_coefficients = 10;
pub const AVM_CICP_MC_BT_2020_NCL: avm_matrix_coefficients = 9;
pub const AVM_CICP_MC_SMPTE_YCGCO: avm_matrix_coefficients = 8;
pub const AVM_CICP_MC_SMPTE_240: avm_matrix_coefficients = 7;
pub const AVM_CICP_MC_BT_601: avm_matrix_coefficients = 6;
pub const AVM_CICP_MC_BT_470_B_G: avm_matrix_coefficients = 5;
pub const AVM_CICP_MC_FCC: avm_matrix_coefficients = 4;
pub const AVM_CICP_MC_RESERVED_3: avm_matrix_coefficients = 3;
pub const AVM_CICP_MC_UNSPECIFIED: avm_matrix_coefficients = 2;
pub const AVM_CICP_MC_BT_709: avm_matrix_coefficients = 1;
pub const AVM_CICP_MC_IDENTITY: avm_matrix_coefficients = 0;
pub type avm_transfer_characteristics_t = avm_transfer_characteristics;
pub type avm_transfer_characteristics = core::ffi::c_uint;
pub const AVM_CICP_TC_RESERVED_19: avm_transfer_characteristics = 19;
pub const AVM_CICP_TC_HLG: avm_transfer_characteristics = 18;
pub const AVM_CICP_TC_SMPTE_428: avm_transfer_characteristics = 17;
pub const AVM_CICP_TC_SMPTE_2084: avm_transfer_characteristics = 16;
pub const AVM_CICP_TC_BT_2020_12_BIT: avm_transfer_characteristics = 15;
pub const AVM_CICP_TC_BT_2020_10_BIT: avm_transfer_characteristics = 14;
pub const AVM_CICP_TC_SRGB: avm_transfer_characteristics = 13;
pub const AVM_CICP_TC_BT_1361: avm_transfer_characteristics = 12;
pub const AVM_CICP_TC_IEC_61966: avm_transfer_characteristics = 11;
pub const AVM_CICP_TC_LOG_100_SQRT10: avm_transfer_characteristics = 10;
pub const AVM_CICP_TC_LOG_100: avm_transfer_characteristics = 9;
pub const AVM_CICP_TC_LINEAR: avm_transfer_characteristics = 8;
pub const AVM_CICP_TC_SMPTE_240: avm_transfer_characteristics = 7;
pub const AVM_CICP_TC_BT_601: avm_transfer_characteristics = 6;
pub const AVM_CICP_TC_BT_470_B_G: avm_transfer_characteristics = 5;
pub const AVM_CICP_TC_BT_470_M: avm_transfer_characteristics = 4;
pub const AVM_CICP_TC_RESERVED_3: avm_transfer_characteristics = 3;
pub const AVM_CICP_TC_UNSPECIFIED: avm_transfer_characteristics = 2;
pub const AVM_CICP_TC_BT_709: avm_transfer_characteristics = 1;
pub const AVM_CICP_TC_RESERVED_0: avm_transfer_characteristics = 0;
pub type avm_color_primaries_t = avm_color_primaries;
pub type avm_color_primaries = core::ffi::c_uint;
pub const AVM_CICP_CP_RESERVED_23: avm_color_primaries = 23;
pub const AVM_CICP_CP_EBU_3213: avm_color_primaries = 22;
pub const AVM_CICP_CP_RESERVED_13: avm_color_primaries = 13;
pub const AVM_CICP_CP_SMPTE_432: avm_color_primaries = 12;
pub const AVM_CICP_CP_SMPTE_431: avm_color_primaries = 11;
pub const AVM_CICP_CP_XYZ: avm_color_primaries = 10;
pub const AVM_CICP_CP_BT_2020: avm_color_primaries = 9;
pub const AVM_CICP_CP_GENERIC_FILM: avm_color_primaries = 8;
pub const AVM_CICP_CP_SMPTE_240: avm_color_primaries = 7;
pub const AVM_CICP_CP_BT_601: avm_color_primaries = 6;
pub const AVM_CICP_CP_BT_470_B_G: avm_color_primaries = 5;
pub const AVM_CICP_CP_BT_470_M: avm_color_primaries = 4;
pub const AVM_CICP_CP_RESERVED_3: avm_color_primaries = 3;
pub const AVM_CICP_CP_UNSPECIFIED: avm_color_primaries = 2;
pub const AVM_CICP_CP_BT_709: avm_color_primaries = 1;
pub const AVM_CICP_CP_RESERVED_0: avm_color_primaries = 0;
pub type avm_codec_iter_t = *const core::ffi::c_void;
pub type avifEncoderChange = core::ffi::c_uint;
pub const AVIF_ENCODER_CHANGE_CODEC_SPECIFIC: avifEncoderChange = 1073741824;
pub const AVIF_ENCODER_CHANGE_SCALING_MODE: avifEncoderChange = 256;
pub const AVIF_ENCODER_CHANGE_QUALITY_ALPHA: avifEncoderChange = 128;
pub const AVIF_ENCODER_CHANGE_QUALITY: avifEncoderChange = 64;
pub const AVIF_ENCODER_CHANGE_TILE_COLS_LOG2: avifEncoderChange = 32;
pub const AVIF_ENCODER_CHANGE_TILE_ROWS_LOG2: avifEncoderChange = 16;
pub const AVIF_ENCODER_CHANGE_MAX_QUANTIZER_ALPHA: avifEncoderChange = 8;
pub const AVIF_ENCODER_CHANGE_MIN_QUANTIZER_ALPHA: avifEncoderChange = 4;
pub const AVIF_ENCODER_CHANGE_MAX_QUANTIZER: avifEncoderChange = 2;
pub const AVIF_ENCODER_CHANGE_MIN_QUANTIZER: avifEncoderChange = 1;
pub type avm_codec_frame_flags_t = uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub buf: *mut core::ffi::c_void,
    pub sz: size_t,
    pub pts: avm_codec_pts_t,
    pub duration: core::ffi::c_ulong,
    pub flags: avm_codec_frame_flags_t,
    pub partition_id: core::ffi::c_int,
    pub vis_frame_size: size_t,
}
pub type avm_codec_pts_t = int64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
    pub frame: C2RustUnnamed_0,
    pub twopass_stats: avm_fixed_buf_t,
    pub firstpass_mb_stats: avm_fixed_buf_t,
    pub psnr: avm_psnr_pkt,
    pub raw: avm_fixed_buf_t,
    pub pad: [core::ffi::c_char; 124],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct avm_psnr_pkt {
    pub samples: [core::ffi::c_uint; 4],
    pub sse: [uint64_t; 4],
    pub psnr: [core::ffi::c_double; 4],
    pub samples_hbd: [core::ffi::c_uint; 4],
    pub sse_hbd: [uint64_t; 4],
    pub psnr_hbd: [core::ffi::c_double; 4],
}
pub type avm_codec_cx_pkt_t = avm_codec_cx_pkt;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct avm_codec_cx_pkt {
    pub kind: avm_codec_cx_pkt_kind,
    pub data: C2RustUnnamed_1,
}
pub type avm_codec_cx_pkt_kind = core::ffi::c_uint;
pub const AVM_CODEC_CUSTOM_PKT: avm_codec_cx_pkt_kind = 256;
pub const AVM_CODEC_CX_FRAME_NULL_PKT: avm_codec_cx_pkt_kind = 4;
pub const AVM_CODEC_PSNR_PKT: avm_codec_cx_pkt_kind = 3;
pub const AVM_CODEC_FPMB_STATS_PKT: avm_codec_cx_pkt_kind = 2;
pub const AVM_CODEC_STATS_PKT: avm_codec_cx_pkt_kind = 1;
pub const AVM_CODEC_CX_FRAME_PKT: avm_codec_cx_pkt_kind = 0;
pub type avm_enc_frame_flags_t = core::ffi::c_long;
pub type avm_scaling_mode_t = avm_scaling_mode;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct avm_scaling_mode {
    pub h_scaling_mode: AVM_SCALING_MODE,
    pub v_scaling_mode: AVM_SCALING_MODE,
}
pub type AVM_SCALING_MODE = avm_scaling_mode_1d;
pub type avm_scaling_mode_1d = core::ffi::c_uint;
pub const AVME_ONETWO: avm_scaling_mode_1d = 6;
pub const AVME_ONEEIGHT: avm_scaling_mode_1d = 5;
pub const AVME_ONEFOUR: avm_scaling_mode_1d = 4;
pub const AVME_THREEFOUR: avm_scaling_mode_1d = 3;
pub const AVME_THREEFIVE: avm_scaling_mode_1d = 2;
pub const AVME_FOURFIVE: avm_scaling_mode_1d = 1;
pub const AVME_NORMAL: avm_scaling_mode_1d = 0;
pub const AVME_SET_SCALEMODE: avme_enc_control_id = 11;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct avmScalingModeMapList {
    pub avifMode: avifFraction,
    pub avmMode: AVM_SCALING_MODE,
}
pub const AVME_SET_MLAYER_ID: avme_enc_control_id = 12;
pub const AV2E_SET_LOSSLESS: avme_enc_control_id = 31;
pub const AVME_SET_QP: avme_enc_control_id = 25;
pub const AV2E_SET_TILE_COLUMNS: avme_enc_control_id = 33;
pub const AV2E_SET_TILE_ROWS: avme_enc_control_id = 34;
pub type avm_codec_enc_cfg_t = avm_codec_enc_cfg;
pub const AVM_TUNE_SSIM: C2RustUnnamed_2 = 1;
pub const AVME_SET_TUNING: avme_enc_control_id = 24;
pub const AV2E_SET_COLOR_RANGE: avme_enc_control_id = 52;
pub const AV2E_SET_CHROMA_SAMPLE_POSITION: avme_enc_control_id = 48;
pub const AVME_SET_CPUUSED: avme_enc_control_id = 13;
pub const AVME_SET_NUMBER_MLAYERS: avme_enc_control_id = 27;
pub const AV2E_SET_ROW_MT: avme_enc_control_id = 32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct avmOptionEnumList {
    pub name: *const core::ffi::c_char,
    pub val: core::ffi::c_int,
}
pub const AV2D_SET_OUTPUT_ALL_LAYERS: avm_dec_control_id = 274;
pub type avm_codec_dec_cfg_t = avm_codec_dec_cfg;
pub type avme_enc_control_id = core::ffi::c_uint;
pub const AV2E_SET_FORCE_DEFERRED_FRAMES_FOR_RAS_TEST: avme_enc_control_id = 185;
pub const AV2E_SET_MONOTONIC_OUTPUT_ORDER: avme_enc_control_id = 184;
pub const AV2E_SET_ADD_SEF_FOR_HIDDEN_FRAMES: avme_enc_control_id = 183;
pub const AV2E_SET_GDF_UNIT_SIZE_MATCHES_SB: avme_enc_control_id = 182;
pub const AV2E_SET_ENABLE_FLAG_MULTI_LAYER_LAG_TEST: avme_enc_control_id = 181;
pub const AV2E_SET_ENABLE_BUFFER_REFRESH_TEST: avme_enc_control_id = 180;
pub const AV2E_SET_ENABLE_EXPLICIT_REF_FRAME_MAP: avme_enc_control_id = 179;
pub const AVME_SET_NUMBER_TLAYERS: avme_enc_control_id = 178;
pub const AVME_SET_TLAYER_ID: avme_enc_control_id = 177;
pub const AV2E_SET_MULTI_SEQ_HEADER_TEST: avme_enc_control_id = 176;
pub const AV2E_SET_SEF_WITH_ORDER_HINT_TEST: avme_enc_control_id = 175;
pub const AV2E_SET_FILM_GRAIN_BLOCK_SIZE: avme_enc_control_id = 174;
pub const AV2E_SET_ENABLE_GDF: avme_enc_control_id = 173;
pub const AV2E_SET_FRAME_MULTI_QMATRIX_UNIT_TEST: avme_enc_control_id = 172;
pub const AV2E_SET_USER_DEFINED_QMATRIX: avme_enc_control_id = 171;
pub const AV2E_GET_ENABLE_BRU: avme_enc_control_id = 170;
pub const AV2E_SET_ENABLE_BRU: avme_enc_control_id = 169;
pub const AV2E_SET_ENABLE_CDF_AVERAGING: avme_enc_control_id = 168;
pub const AV2E_SET_ENABLE_INTRA_DIP: avme_enc_control_id = 167;
pub const AV2E_GET_FRAME_INFO: avme_enc_control_id = 165;
pub const AV2E_GET_SUB_GOP_CONFIG: avme_enc_control_id = 164;
pub const AV2E_ENABLE_SUBGOP_STATS: avme_enc_control_id = 163;
pub const AV2E_GET_FRAME_TYPE: avme_enc_control_id = 162;
pub const AV2E_SET_ENABLE_DEBLOCKING: avme_enc_control_id = 161;
pub const AV2E_GET_BASELINE_GF_INTERVAL: avme_enc_control_id = 160;
pub const AV2E_SET_SUBGOP_CONFIG_PATH: avme_enc_control_id = 159;
pub const AV2E_SET_SUBGOP_CONFIG_STR: avme_enc_control_id = 158;
pub const AV2E_SET_VBR_CORPUS_COMPLEXITY_LAP: avme_enc_control_id = 157;
pub const AV2E_SET_GF_MIN_PYRAMID_HEIGHT: avme_enc_control_id = 156;
pub const AV2E_ENABLE_SB_MULTIPASS_UNIT_TEST: avme_enc_control_id = 155;
pub const AV2E_SET_VMAF_MODEL_PATH: avme_enc_control_id = 153;
pub const AV2E_SET_MIN_CR: avme_enc_control_id = 144;
pub const AV2E_SET_TIER_MASK: avme_enc_control_id = 143;
pub const AV2E_SET_MV_COST_UPD_FREQ: avme_enc_control_id = 142;
pub const AV2E_SET_MODE_COST_UPD_FREQ: avme_enc_control_id = 141;
pub const AV2E_SET_COEFF_COST_UPD_FREQ: avme_enc_control_id = 140;
pub const AV2E_SET_REDUCED_REFERENCE_SET: avme_enc_control_id = 125;
pub const AV2E_SET_MAX_REFERENCE_FRAMES: avme_enc_control_id = 124;
pub const AV2E_SET_GF_MAX_PYRAMID_HEIGHT: avme_enc_control_id = 123;
pub const AV2E_SET_QUANT_B_ADAPT: avme_enc_control_id = 122;
pub const AV2E_SET_INTRA_DEFAULT_TX_ONLY: avme_enc_control_id = 121;
pub const AV2E_SET_INTER_DCT_ONLY: avme_enc_control_id = 120;
pub const AV2E_SET_INTRA_DCT_ONLY: avme_enc_control_id = 119;
pub const AV2E_SET_REDUCED_TX_TYPE_SET: avme_enc_control_id = 118;
pub const AV2E_SET_CHROMA_SUBSAMPLING_Y: avme_enc_control_id = 117;
pub const AV2E_SET_CHROMA_SUBSAMPLING_X: avme_enc_control_id = 116;
pub const AV2E_SET_DENOISE_BLOCK_SIZE: avme_enc_control_id = 115;
pub const AV2E_SET_DENOISE_NOISE_LEVEL: avme_enc_control_id = 114;
pub const AV2E_SET_FILM_GRAIN_TABLE: avme_enc_control_id = 113;
pub const AV2E_SET_FILM_GRAIN_TEST_VECTOR: avme_enc_control_id = 112;
pub const AV2E_SET_TIMING_INFO_TYPE: avme_enc_control_id = 111;
pub const AV2E_ENABLE_MOTION_VECTOR_UNIT_TEST: avme_enc_control_id = 110;
pub const AV2E_SET_DELTAQ_MODE: avme_enc_control_id = 107;
pub const AV2E_SET_ENABLE_ANGLE_DELTA: avme_enc_control_id = 106;
pub const AV2E_SET_ENABLE_INTRABC: avme_enc_control_id = 105;
pub const AV2E_SET_ENABLE_PALETTE: avme_enc_control_id = 104;
pub const AV2E_SET_ENABLE_OVERLAY: avme_enc_control_id = 103;
pub const AV2E_SET_ENABLE_CFL_INTRA: avme_enc_control_id = 101;
pub const AV2E_SET_ENABLE_PAETH_INTRA: avme_enc_control_id = 100;
pub const AV2E_SET_ENABLE_SMOOTH_INTRA: avme_enc_control_id = 99;
pub const AV2E_SET_ENABLE_WARPED_MOTION: avme_enc_control_id = 96;
pub const AV2E_SET_ENABLE_GLOBAL_MOTION: avme_enc_control_id = 95;
pub const AV2E_SET_ENABLE_INTERINTRA_WEDGE: avme_enc_control_id = 94;
pub const AV2E_SET_ENABLE_INTERINTER_WEDGE: avme_enc_control_id = 93;
pub const AV2E_SET_ENABLE_DIFF_WTD_COMP: avme_enc_control_id = 92;
pub const AV2E_SET_ENABLE_SMOOTH_INTERINTRA: avme_enc_control_id = 91;
pub const AV2E_SET_ENABLE_INTERINTRA_COMP: avme_enc_control_id = 90;
pub const AV2E_SET_ENABLE_ONESIDED_COMP: avme_enc_control_id = 89;
pub const AV2E_SET_ENABLE_MASKED_COMP: avme_enc_control_id = 88;
pub const AV2E_SET_ENABLE_CHROMA_DELTAQ: avme_enc_control_id = 87;
pub const AV2E_SET_ALLOW_REF_FRAME_MVS: avme_enc_control_id = 85;
pub const AV2E_SET_ENABLE_REF_FRAME_MVS: avme_enc_control_id = 84;
pub const AV2E_SET_ENABLE_FLIP_IDTX: avme_enc_control_id = 81;
pub const AV2E_SET_ENABLE_INTRA_EDGE_FILTER: avme_enc_control_id = 78;
pub const AV2E_SET_MAX_PARTITION_SIZE: avme_enc_control_id = 77;
pub const AV2E_SET_MIN_PARTITION_SIZE: avme_enc_control_id = 76;
pub const AV2E_SET_ENABLE_1TO4_PARTITIONS: avme_enc_control_id = 75;
pub const AV2E_SET_ENABLE_RECT_PARTITIONS: avme_enc_control_id = 73;
pub const AV2E_SET_MTU: avme_enc_control_id = 71;
pub const AV2E_SET_NUM_TG: avme_enc_control_id = 70;
pub const AV2E_SET_QM_V: avme_enc_control_id = 68;
pub const AV2E_SET_QM_U: avme_enc_control_id = 67;
pub const AV2E_SET_QM_Y: avme_enc_control_id = 66;
pub const AV2E_SET_QM_MAX: avme_enc_control_id = 65;
pub const AV2E_SET_QM_MIN: avme_enc_control_id = 64;
pub const AV2E_SET_ENABLE_QM: avme_enc_control_id = 63;
pub const AV2E_SET_ENABLE_TRELLIS_QUANT: avme_enc_control_id = 62;
pub const AV2E_SET_FORCE_VIDEO_MODE: avme_enc_control_id = 60;
pub const AV2E_SET_ENABLE_RESTORATION: avme_enc_control_id = 59;
pub const AV2E_SET_ENABLE_CDEF: avme_enc_control_id = 58;
pub const AVME_SET_ENABLEAUTOBWDREF: avme_enc_control_id = 57;
pub const AV2E_SET_SUPERBLOCK_SIZE: avme_enc_control_id = 56;
pub const AV2E_GET_SEQ_LEVEL_IDX: avme_enc_control_id = 55;
pub const AV2E_SET_TARGET_SEQ_LEVEL_IDX: avme_enc_control_id = 54;
pub const AV2E_GET_ACTIVEMAP: avme_enc_control_id = 51;
pub const AV2E_SET_MAX_GF_INTERVAL: avme_enc_control_id = 50;
pub const AV2E_SET_MIN_GF_INTERVAL: avme_enc_control_id = 49;
pub const AV2E_SET_MATRIX_COEFFICIENTS: avme_enc_control_id = 47;
pub const AV2E_SET_TRANSFER_CHARACTERISTICS: avme_enc_control_id = 46;
pub const AV2E_SET_COLOR_PRIMARIES: avme_enc_control_id = 45;
pub const AV2E_SET_CDF_UPDATE_MODE: avme_enc_control_id = 44;
pub const AV2E_SET_TUNE_CONTENT: avme_enc_control_id = 43;
pub const AV2E_SET_NOISE_SENSITIVITY: avme_enc_control_id = 42;
pub const AV2E_SET_FRAME_PERIODIC_BOOST: avme_enc_control_id = 41;
pub const AV2E_SET_AQ_MODE: avme_enc_control_id = 40;
pub const AV2E_SET_ENABLE_SFRAME: avme_enc_control_id = 39;
pub const AV2E_SET_FRAME_PARALLEL_DECODING: avme_enc_control_id = 37;
pub const AV2E_SET_ENABLE_KEYFRAME_FILTERING: avme_enc_control_id = 36;
pub const AV2E_SET_ENABLE_TPL_MODEL: avme_enc_control_id = 35;
pub const AV2E_SET_GF_CBR_BOOST_PCT: avme_enc_control_id = 29;
pub const AV2E_SET_MAX_INTER_BITRATE_PCT: avme_enc_control_id = 28;
pub const AVME_SET_MAX_INTRA_BITRATE_PCT: avme_enc_control_id = 26;
pub const AVME_SET_ARNR_STRENGTH: avme_enc_control_id = 22;
pub const AVME_SET_ARNR_MAXFRAMES: avme_enc_control_id = 21;
pub const AVME_GET_LAST_QUANTIZER: avme_enc_control_id = 19;
pub const AVME_SET_STATIC_THRESHOLD: avme_enc_control_id = 17;
pub const AVME_SET_SHARPNESS: avme_enc_control_id = 16;
pub const AVME_SET_ENABLEAUTOALTREF: avme_enc_control_id = 14;
pub const AVME_SET_ACTIVEMAP: avme_enc_control_id = 9;
pub const AVME_SET_ROI_MAP: avme_enc_control_id = 8;
pub const AVME_USE_REFERENCE: avme_enc_control_id = 7;
pub type C2RustUnnamed_2 = core::ffi::c_uint;
pub const AVM_TUNE_VMAF_NEG_MAX_GAIN: C2RustUnnamed_2 = 7;
pub const AVM_TUNE_VMAF_MAX_GAIN: C2RustUnnamed_2 = 6;
pub const AVM_TUNE_VMAF_WITHOUT_PREPROCESSING: C2RustUnnamed_2 = 5;
pub const AVM_TUNE_VMAF_WITH_PREPROCESSING: C2RustUnnamed_2 = 4;
pub const AVM_TUNE_PSNR: C2RustUnnamed_2 = 0;
pub type avm_dec_control_id = core::ffi::c_uint;
pub const AVMD_INCR_OUTPUT_FRAMES_OFFSET: avm_dec_control_id = 291;
pub const AV2D_ENABLE_SUBGOP_STATS: avm_dec_control_id = 290;
pub const AVMD_GET_FRAME_INFO: avm_dec_control_id = 289;
pub const AVMD_GET_S_FRAME_INFO: avm_dec_control_id = 288;
pub const AVMD_GET_SHOW_EXISTING_FRAME_FLAG: avm_dec_control_id = 287;
pub const AVMD_GET_SB_SIZE: avm_dec_control_id = 286;
pub const AVMD_GET_STILL_PICTURE: avm_dec_control_id = 285;
pub const AVMD_GET_SCREEN_CONTENT_TOOLS_INFO: avm_dec_control_id = 284;
pub const AVMD_GET_TILE_INFO: avm_dec_control_id = 283;
pub const AVMD_GET_ALTREF_PRESENT: avm_dec_control_id = 282;
pub const AVMD_GET_FRAME_FLAGS: avm_dec_control_id = 281;
pub const AVMD_GET_FWD_KF_PRESENT: avm_dec_control_id = 280;
pub const AVM_DECODER_CTRL_ID_MAX: avm_dec_control_id = 279;
pub const AV2D_SET_BRU_OPT_MODE: avm_dec_control_id = 278;
pub const AV2D_SET_RANDOM_ACCESS: avm_dec_control_id = 277;
pub const AV2D_SET_SKIP_FILM_GRAIN: avm_dec_control_id = 276;
pub const AV2_SET_INSPECTION_CALLBACK: avm_dec_control_id = 275;
pub const AV2D_SET_SELECTED_LOCAL_OPS: avm_dec_control_id = 273;
pub const AV2D_SET_SUB_BITSTREAM_EXTRACTION: avm_dec_control_id = 272;
pub const AV2D_SET_SELECTED_OPS: avm_dec_control_id = 271;
pub const AV2D_SET_ROW_MT: avm_dec_control_id = 270;
pub const AVMD_GET_LAST_QUANTIZER: avm_dec_control_id = 269;
pub const AV2_GET_ACCOUNTING: avm_dec_control_id = 268;
pub const AV2_SET_SKIP_LOOP_FILTER: avm_dec_control_id = 267;
pub const AV2_INVERT_TILE_DECODE_ORDER: avm_dec_control_id = 266;
pub const AV2_SET_BYTE_ALIGNMENT: avm_dec_control_id = 265;
pub const AV2D_GET_TILE_COUNT: avm_dec_control_id = 264;
pub const AV2D_GET_TILE_SIZE: avm_dec_control_id = 263;
pub const AV2D_GET_IMG_FORMAT: avm_dec_control_id = 262;
pub const AV2D_GET_BIT_DEPTH: avm_dec_control_id = 261;
pub const AV2D_GET_DISPLAY_SIZE: avm_dec_control_id = 260;
pub const AV2D_GET_FRAME_SIZE: avm_dec_control_id = 259;
pub const AVMD_GET_LAST_REF_USED: avm_dec_control_id = 258;
pub const AVMD_GET_FRAME_CORRUPTED: avm_dec_control_id = 257;
pub const AVMD_GET_LAST_REF_UPDATES: avm_dec_control_id = 256;
pub const NULL: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
pub const AVIF_TRUE: core::ffi::c_int = 1 as core::ffi::c_int;
pub const AVIF_FALSE: core::ffi::c_int = 0 as core::ffi::c_int;
pub const AVIF_QUANTIZER_LOSSLESS: core::ffi::c_int = 0 as core::ffi::c_int;
pub const AVIF_SPEED_DEFAULT: core::ffi::c_int = -(1 as core::ffi::c_int);
pub const AVIF_SPATIAL_ID_UNSET: core::ffi::c_int = 0xff as core::ffi::c_int;
pub const AVM_IMAGE_ABI_VERSION: core::ffi::c_int = 9 as core::ffi::c_int;
pub const AVM_IMG_FMT_HIGHBITDEPTH: core::ffi::c_int = 0x800 as core::ffi::c_int;
pub const AVM_CODEC_ABI_VERSION: core::ffi::c_int = 6 as core::ffi::c_int
    + AVM_IMAGE_ABI_VERSION;
pub const AVM_FRAME_IS_KEY: core::ffi::c_int = 0x1 as core::ffi::c_int;
pub const AVM_DECODER_ABI_VERSION: core::ffi::c_int = 6 as core::ffi::c_int
    + AVM_CODEC_ABI_VERSION;
pub const AVM_ENCODER_ABI_VERSION: core::ffi::c_int = 8 as core::ffi::c_int
    + AVM_CODEC_ABI_VERSION;
pub const AVM_EFLAG_FORCE_KF: core::ffi::c_int = (1 as core::ffi::c_int)
    << 0 as core::ffi::c_int;
pub const AVM_USAGE_GOOD_QUALITY: core::ffi::c_int = 0 as core::ffi::c_int;
pub const AVM_EFLAG_NO_REF_GF: core::ffi::c_int = (1 as core::ffi::c_int)
    << 19 as core::ffi::c_int;
pub const AVM_EFLAG_NO_REF_ARF: core::ffi::c_int = (1 as core::ffi::c_int)
    << 20 as core::ffi::c_int;
pub const AVM_EFLAG_NO_REF_BWD: core::ffi::c_int = (1 as core::ffi::c_int)
    << 21 as core::ffi::c_int;
pub const AVM_EFLAG_NO_REF_ARF2: core::ffi::c_int = (1 as core::ffi::c_int)
    << 22 as core::ffi::c_int;
pub const AVM_EFLAG_NO_UPD_ALL: core::ffi::c_int = (1 as core::ffi::c_int)
    << 23 as core::ffi::c_int;
pub const TWO_LAYER_ALL_INTRA_QUALITY_THRESHOLD: core::ffi::c_int = 10
    as core::ffi::c_int;
unsafe extern "C" fn avmCodecDestroyInternal(mut codec: *mut avifCodec) {
    if (*(*codec).internal).decoderInitialized != 0 {
        avm_codec_destroy(&mut (*(*codec).internal).decoder);
    }
    if (*(*codec).internal).encoderInitialized != 0 {
        avm_codec_destroy(&mut (*(*codec).internal).encoder);
    }
    avifFree((*codec).internal as *mut core::ffi::c_void);
}
unsafe extern "C" fn avmCodecGetNextImage(
    mut codec: *mut avifCodec,
    mut sample: *const avifDecodeSample,
    mut alpha: avifBool,
    mut isLimitedRangeAlpha: *mut avifBool,
    mut image: *mut avifImage,
) -> avifBool {
    if (*(*codec).internal).decoderInitialized == 0 {
        let mut cfg: avm_codec_dec_cfg_t = avm_codec_dec_cfg {
            threads: 0,
            w: 0,
            h: 0,
            path_parakit: 0 as *mut core::ffi::c_char,
            suffix_parakit: 0 as *mut core::ffi::c_char,
        };
        memset(
            &mut cfg as *mut avm_codec_dec_cfg_t as *mut core::ffi::c_void,
            0 as core::ffi::c_int,
            ::core::mem::size_of::<avm_codec_dec_cfg_t>() as size_t,
        );
        cfg.threads = (*codec).maxThreads as core::ffi::c_uint;
        let mut decoder_interface: *const avm_codec_iface_t = avm_codec_av2_dx();
        if avm_codec_dec_init_ver(
            &mut (*(*codec).internal).decoder,
            decoder_interface,
            &mut cfg,
            0 as avm_codec_flags_t,
            AVM_DECODER_ABI_VERSION,
        ) as u64 != 0
        {
            return AVIF_FALSE;
        }
        (*(*codec).internal).decoderInitialized = AVIF_TRUE as avifBool;
        if avm_codec_control(
            &mut (*(*codec).internal).decoder as *mut avm_codec_ctx_t,
            AV2D_SET_OUTPUT_ALL_LAYERS as core::ffi::c_int,
            (*codec).allLayers,
        ) as u64 != 0
        {
            return AVIF_FALSE;
        }
        if (*codec).operatingPoint as core::ffi::c_int != 0 as core::ffi::c_int {
            return AVIF_FALSE;
        }
        (*(*codec).internal).iter = 0 as avm_codec_iter_t;
    }
    let mut nextFrame: *mut avm_image_t = 0 as *mut avm_image_t;
    let mut spatialID: uint8_t = AVIF_SPATIAL_ID_UNSET as uint8_t;
    loop {
        nextFrame = avm_codec_get_frame(
            &mut (*(*codec).internal).decoder,
            &mut (*(*codec).internal).iter,
        );
        if !nextFrame.is_null() {
            if !(spatialID as core::ffi::c_int != AVIF_SPATIAL_ID_UNSET) {
                break;
            }
            if spatialID as core::ffi::c_int == (*nextFrame).mlayer_id {
                break;
            }
        } else {
            if sample.is_null() {
                break;
            }
            (*(*codec).internal).iter = 0 as avm_codec_iter_t;
            if avm_codec_decode(
                &mut (*(*codec).internal).decoder,
                (*sample).data.data,
                (*sample).data.size,
                NULL,
            ) as u64 != 0
            {
                return AVIF_FALSE;
            }
            spatialID = (*sample).spatialID;
            sample = 0 as *const avifDecodeSample;
        }
    }
    if !nextFrame.is_null() {
        (*(*codec).internal).image = nextFrame;
    } else if alpha != 0 && !((*(*codec).internal).image).is_null() {} else {
        return AVIF_FALSE
    }
    let mut isColor: avifBool = (alpha == 0) as core::ffi::c_int;
    if isColor != 0 {
        let mut yuvFormat: avifPixelFormat = AVIF_PIXEL_FORMAT_NONE;
        match (*(*(*codec).internal).image).fmt as core::ffi::c_uint {
            258 | 2306 => {
                yuvFormat = AVIF_PIXEL_FORMAT_YUV420;
            }
            259 | 2307 => {
                yuvFormat = AVIF_PIXEL_FORMAT_YUV422;
            }
            260 | 2308 => {
                yuvFormat = AVIF_PIXEL_FORMAT_YUV444;
            }
            0 | 769 | 2817 | _ => return AVIF_FALSE,
        }
        if (*(*(*codec).internal).image).monochrome != 0 {
            yuvFormat = AVIF_PIXEL_FORMAT_YUV400;
        }
        (*image).width = (*(*(*codec).internal).image).d_w as uint32_t;
        (*image).height = (*(*(*codec).internal).image).d_h as uint32_t;
        (*image).depth = (*(*(*codec).internal).image).bit_depth as uint32_t;
        (*image).yuvFormat = yuvFormat;
        (*image).yuvRange = (if (*(*(*codec).internal).image).range as core::ffi::c_uint
            == AVM_CR_STUDIO_RANGE as core::ffi::c_int as core::ffi::c_uint
        {
            AVIF_RANGE_LIMITED as core::ffi::c_int
        } else {
            AVIF_RANGE_FULL as core::ffi::c_int
        }) as avifRange;
        if (*(*(*codec).internal).image).csp as core::ffi::c_uint
            == AVM_CSP_LEFT as core::ffi::c_int as core::ffi::c_uint
        {
            (*image).yuvChromaSamplePosition = AVIF_CHROMA_SAMPLE_POSITION_VERTICAL;
        } else if (*(*(*codec).internal).image).csp as core::ffi::c_uint
            == AVM_CSP_CENTER as core::ffi::c_int as core::ffi::c_uint
        {
            (*image).yuvChromaSamplePosition = AVIF_CHROMA_SAMPLE_POSITION_UNKNOWN;
        } else if (*(*(*codec).internal).image).csp as core::ffi::c_uint
            == AVM_CSP_TOPLEFT as core::ffi::c_int as core::ffi::c_uint
        {
            (*image).yuvChromaSamplePosition = AVIF_CHROMA_SAMPLE_POSITION_COLOCATED;
        } else {
            (*image).yuvChromaSamplePosition = AVIF_CHROMA_SAMPLE_POSITION_UNKNOWN;
        }
        (*image).colorPrimaries = (*(*(*codec).internal).image).cp as avifColorPrimaries;
        (*image).transferCharacteristics = (*(*(*codec).internal).image).tc
            as avifTransferCharacteristics;
        (*image).matrixCoefficients = (*(*(*codec).internal).image).mc
            as avifMatrixCoefficients;
        avifImageFreePlanes(
            image,
            AVIF_PLANES_YUV as core::ffi::c_int as avifPlanesFlags,
        );
        let mut yuvPlaneCount: core::ffi::c_int = if yuvFormat as core::ffi::c_uint
            == AVIF_PIXEL_FORMAT_YUV400 as core::ffi::c_int as core::ffi::c_uint
        {
            1 as core::ffi::c_int
        } else {
            3 as core::ffi::c_int
        };
        if avifImageUsesU16(image) == 0
            && (*(*(*codec).internal).image).fmt as core::ffi::c_uint
                & AVM_IMG_FMT_HIGHBITDEPTH as core::ffi::c_uint != 0
        {
            if !(avifImageAllocatePlanes(
                image,
                AVIF_PLANES_YUV as core::ffi::c_int as avifPlanesFlags,
            ) as core::ffi::c_uint
                == AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint)
            {
                return AVIF_FALSE;
            }
            let mut yuvPlane: core::ffi::c_int = 0 as core::ffi::c_int;
            while yuvPlane < yuvPlaneCount {
                let planeWidth: uint32_t = avifImagePlaneWidth(image, yuvPlane)
                    as uint32_t;
                let planeHeight: uint32_t = avifImagePlaneHeight(image, yuvPlane)
                    as uint32_t;
                let mut srcRow: *const uint8_t = (*(*(*codec).internal).image)
                    .planes[yuvPlane as usize];
                let mut dstRow: *mut uint8_t = avifImagePlane(image, yuvPlane);
                let dstRowBytes: uint32_t = avifImagePlaneRowBytes(image, yuvPlane)
                    as uint32_t;
                let mut y: uint32_t = 0 as uint32_t;
                while y < planeHeight {
                    let mut srcRow16: *const uint16_t = srcRow as *const uint16_t;
                    let mut x: uint32_t = 0 as uint32_t;
                    while x < planeWidth {
                        *dstRow.offset(x as isize) = *srcRow16.offset(x as isize)
                            as uint8_t;
                        x = x.wrapping_add(1);
                    }
                    srcRow = srcRow
                        .offset(
                            (*(*(*codec).internal).image).stride[yuvPlane as usize]
                                as isize,
                        );
                    dstRow = dstRow.offset(dstRowBytes as isize);
                    y = y.wrapping_add(1);
                }
                yuvPlane += 1;
            }
        } else {
            let mut yuvPlane_0: core::ffi::c_int = 0 as core::ffi::c_int;
            while yuvPlane_0 < yuvPlaneCount {
                (*image).yuvPlanes[yuvPlane_0 as usize] = (*(*(*codec).internal).image)
                    .planes[yuvPlane_0 as usize] as *mut uint8_t;
                (*image).yuvRowBytes[yuvPlane_0 as usize] = (*(*(*codec).internal).image)
                    .stride[yuvPlane_0 as usize] as uint32_t;
                yuvPlane_0 += 1;
            }
            (*image).imageOwnsYUVPlanes = AVIF_FALSE as avifBool;
        }
    } else {
        (*image).width = (*(*(*codec).internal).image).d_w as uint32_t;
        (*image).height = (*(*(*codec).internal).image).d_h as uint32_t;
        (*image).depth = (*(*(*codec).internal).image).bit_depth as uint32_t;
        avifImageFreePlanes(image, AVIF_PLANES_A as core::ffi::c_int as avifPlanesFlags);
        if avifImageUsesU16(image) == 0
            && (*(*(*codec).internal).image).fmt as core::ffi::c_uint
                & AVM_IMG_FMT_HIGHBITDEPTH as core::ffi::c_uint != 0
        {
            if !(avifImageAllocatePlanes(
                image,
                AVIF_PLANES_A as core::ffi::c_int as avifPlanesFlags,
            ) as core::ffi::c_uint
                == AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint)
            {
                return AVIF_FALSE;
            }
            let mut srcRow_0: *const uint8_t = (*(*(*codec).internal).image)
                .planes[0 as core::ffi::c_int as usize];
            let mut dstRow_0: *mut uint8_t = (*image).alphaPlane;
            let mut y_0: uint32_t = 0 as uint32_t;
            while y_0 < (*image).height {
                let mut srcRow16_0: *const uint16_t = srcRow_0 as *const uint16_t;
                let mut x_0: uint32_t = 0 as uint32_t;
                while x_0 < (*image).width {
                    *dstRow_0.offset(x_0 as isize) = *srcRow16_0.offset(x_0 as isize)
                        as uint8_t;
                    x_0 = x_0.wrapping_add(1);
                }
                srcRow_0 = srcRow_0
                    .offset(
                        (*(*(*codec).internal).image)
                            .stride[0 as core::ffi::c_int as usize] as isize,
                    );
                dstRow_0 = dstRow_0.offset((*image).alphaRowBytes as isize);
                y_0 = y_0.wrapping_add(1);
            }
        } else {
            (*image).alphaPlane = (*(*(*codec).internal).image)
                .planes[0 as core::ffi::c_int as usize] as *mut uint8_t;
            (*image).alphaRowBytes = (*(*(*codec).internal).image)
                .stride[0 as core::ffi::c_int as usize] as uint32_t;
            (*image).imageOwnsAlphaPlane = AVIF_FALSE as avifBool;
        }
        *isLimitedRangeAlpha = ((*(*(*codec).internal).image).range as core::ffi::c_uint
            == AVM_CR_STUDIO_RANGE as core::ffi::c_int as core::ffi::c_uint)
            as core::ffi::c_int as avifBool;
    }
    return AVIF_TRUE;
}
unsafe extern "C" fn avifImageCalcAVMFmt(
    mut image: *const avifImage,
    mut alpha: avifBool,
) -> avm_img_fmt_t {
    let mut fmt: avm_img_fmt_t = AVM_IMG_FMT_NONE;
    if alpha != 0 {
        fmt = AVM_IMG_FMT_I420;
    } else {
        match (*image).yuvFormat as core::ffi::c_uint {
            1 => {
                fmt = AVM_IMG_FMT_I444;
            }
            2 => {
                fmt = AVM_IMG_FMT_I422;
            }
            3 | 4 => {
                fmt = AVM_IMG_FMT_I420;
            }
            0 | 5 | _ => return AVM_IMG_FMT_NONE,
        }
    }
    if (*image).depth > 8 as uint32_t {
        fmt = ::core::mem::transmute::<
            core::ffi::c_uint,
            avm_img_fmt_t,
        >(fmt as core::ffi::c_uint | AVM_IMG_FMT_HIGHBITDEPTH as core::ffi::c_uint);
    }
    return fmt;
}
unsafe extern "C" fn avmOptionParseEnum(
    mut str: *const core::ffi::c_char,
    mut enums: *const avmOptionEnumList,
    mut val: *mut core::ffi::c_int,
) -> avifBool {
    let mut listptr: *const avmOptionEnumList = 0 as *const avmOptionEnumList;
    let mut rawval: core::ffi::c_long = 0;
    let mut endptr: *mut core::ffi::c_char = 0 as *mut core::ffi::c_char;
    rawval = strtol(str, &mut endptr, 10 as core::ffi::c_int);
    if *str.offset(0 as core::ffi::c_int as isize) as core::ffi::c_int != '\0' as i32
        && *endptr.offset(0 as core::ffi::c_int as isize) as core::ffi::c_int
            == '\0' as i32
    {
        listptr = enums;
        while !((*listptr).name).is_null() {
            if (*listptr).val as core::ffi::c_long == rawval {
                *val = rawval as core::ffi::c_int;
                return AVIF_TRUE;
            }
            listptr = listptr.offset(1);
        }
    }
    listptr = enums;
    while !((*listptr).name).is_null() {
        if strcmp(str, (*listptr).name) == 0 {
            *val = (*listptr).val;
            return AVIF_TRUE;
        }
        listptr = listptr.offset(1);
    }
    return AVIF_FALSE;
}
static mut endUsageEnum: [avmOptionEnumList; 5] = [
    {
        let mut init = avmOptionEnumList {
            name: b"vbr\0" as *const u8 as *const core::ffi::c_char,
            val: AVM_VBR as core::ffi::c_int,
        };
        init
    },
    {
        let mut init = avmOptionEnumList {
            name: b"cbr\0" as *const u8 as *const core::ffi::c_char,
            val: AVM_CBR as core::ffi::c_int,
        };
        init
    },
    {
        let mut init = avmOptionEnumList {
            name: b"cq\0" as *const u8 as *const core::ffi::c_char,
            val: AVM_CQ as core::ffi::c_int,
        };
        init
    },
    {
        let mut init = avmOptionEnumList {
            name: b"q\0" as *const u8 as *const core::ffi::c_char,
            val: AVM_Q as core::ffi::c_int,
        };
        init
    },
    {
        let mut init = avmOptionEnumList {
            name: 0 as *const core::ffi::c_char,
            val: 0 as core::ffi::c_int,
        };
        init
    },
];
unsafe extern "C" fn avifKeyEqualsName(
    mut key: *const core::ffi::c_char,
    mut name: *const core::ffi::c_char,
    mut alpha: avifBool,
) -> avifBool {
    let mut prefix: *const core::ffi::c_char = if alpha != 0 {
        b"alpha:\0" as *const u8 as *const core::ffi::c_char
    } else {
        b"color:\0" as *const u8 as *const core::ffi::c_char
    };
    let mut prefixLen: size_t = 6 as size_t;
    let mut shortPrefix: *const core::ffi::c_char = if alpha != 0 {
        b"a:\0" as *const u8 as *const core::ffi::c_char
    } else {
        b"c:\0" as *const u8 as *const core::ffi::c_char
    };
    let mut shortPrefixLen: size_t = 2 as size_t;
    return (strcmp(key, name) == 0
        || strncmp(key, prefix, prefixLen) == 0
            && strcmp(key.offset(prefixLen as isize), name) == 0
        || strncmp(key, shortPrefix, shortPrefixLen) == 0
            && strcmp(key.offset(shortPrefixLen as isize), name) == 0)
        as core::ffi::c_int;
}
unsafe extern "C" fn avifProcessAVMOptionsPreInit(
    mut codec: *mut avifCodec,
    mut alpha: avifBool,
    mut cfg: *mut avm_codec_enc_cfg,
) -> avifBool {
    let mut i: uint32_t = 0 as uint32_t;
    while i < (*(*codec).csOptions).count {
        let mut entry: *const avifCodecSpecificOption = &mut *((*(*codec).csOptions)
            .entries)
            .offset(i as isize) as *mut avifCodecSpecificOption;
        let mut val: core::ffi::c_int = 0;
        if avifKeyEqualsName(
            (*entry).key,
            b"end-usage\0" as *const u8 as *const core::ffi::c_char,
            alpha,
        ) != 0
        {
            if avmOptionParseEnum((*entry).value, core::ptr::addr_of!(endUsageEnum).cast::<avmOptionEnumList>(), &mut val) == 0 {
                avifDiagnosticsPrintf(
                    (*codec).diag,
                    b"Invalid value for end-usage: %s\0" as *const u8
                        as *const core::ffi::c_char,
                    (*entry).value,
                );
                return AVIF_FALSE;
            }
            (*cfg).rc_end_usage = val as avm_rc_mode;
        }
        i = i.wrapping_add(1);
    }
    return AVIF_TRUE;
}
unsafe extern "C" fn avifProcessAVMOptionsPostInit(
    mut codec: *mut avifCodec,
    mut alpha: avifBool,
) -> avifBool {
    let mut i: uint32_t = 0 as uint32_t;
    while i < (*(*codec).csOptions).count {
        let mut entry: *const avifCodecSpecificOption = &mut *((*(*codec).csOptions)
            .entries)
            .offset(i as isize) as *mut avifCodecSpecificOption;
        let mut otherPrefix: *const core::ffi::c_char = if alpha != 0 {
            b"color:\0" as *const u8 as *const core::ffi::c_char
        } else {
            b"alpha:\0" as *const u8 as *const core::ffi::c_char
        };
        let mut otherPrefixLen: size_t = 6 as size_t;
        let mut otherShortPrefix: *const core::ffi::c_char = if alpha != 0 {
            b"c:\0" as *const u8 as *const core::ffi::c_char
        } else {
            b"a:\0" as *const u8 as *const core::ffi::c_char
        };
        let mut otherShortPrefixLen: size_t = 2 as size_t;
        if !(strncmp((*entry).key, otherPrefix, otherPrefixLen) == 0
            || strncmp((*entry).key, otherShortPrefix, otherShortPrefixLen) == 0)
        {
            if !(avifKeyEqualsName(
                (*entry).key,
                b"end-usage\0" as *const u8 as *const core::ffi::c_char,
                alpha,
            ) != 0)
            {
                let mut prefix: *const core::ffi::c_char = if alpha != 0 {
                    b"alpha:\0" as *const u8 as *const core::ffi::c_char
                } else {
                    b"color:\0" as *const u8 as *const core::ffi::c_char
                };
                let mut prefixLen: size_t = 6 as size_t;
                let mut shortPrefix: *const core::ffi::c_char = if alpha != 0 {
                    b"a:\0" as *const u8 as *const core::ffi::c_char
                } else {
                    b"c:\0" as *const u8 as *const core::ffi::c_char
                };
                let mut shortPrefixLen: size_t = 2 as size_t;
                let mut key: *const core::ffi::c_char = (*entry).key;
                if strncmp(key, prefix, prefixLen) == 0 {
                    key = key.offset(prefixLen as isize);
                } else if strncmp(key, shortPrefix, shortPrefixLen) == 0 {
                    key = key.offset(shortPrefixLen as isize);
                }
                if avm_codec_set_option(
                    &mut (*(*codec).internal).encoder,
                    key,
                    (*entry).value,
                ) as core::ffi::c_uint
                    != AVM_CODEC_OK as core::ffi::c_int as core::ffi::c_uint
                {
                    avifDiagnosticsPrintf(
                        (*codec).diag,
                        b"avm_codec_set_option(\"%s\", \"%s\") failed: %s: %s\0"
                            as *const u8 as *const core::ffi::c_char,
                        key,
                        (*entry).value,
                        avm_codec_error(&mut (*(*codec).internal).encoder),
                        avm_codec_error_detail(&mut (*(*codec).internal).encoder),
                    );
                    return AVIF_FALSE;
                }
                if strcmp(key, b"tune\0" as *const u8 as *const core::ffi::c_char) == 0 {
                    (*(*codec).internal).tuningSet = AVIF_TRUE as avifBool;
                }
            }
        }
        i = i.wrapping_add(1);
    }
    return AVIF_TRUE;
}
static mut scalingModeMap: [avmScalingModeMapList; 7] = [
    {
        let mut init = avmScalingModeMapList {
            avifMode: {
                let mut init = avifFraction {
                    n: 1 as int32_t,
                    d: 1 as int32_t,
                };
                init
            },
            avmMode: AVME_NORMAL,
        };
        init
    },
    {
        let mut init = avmScalingModeMapList {
            avifMode: {
                let mut init = avifFraction {
                    n: 1 as int32_t,
                    d: 2 as int32_t,
                };
                init
            },
            avmMode: AVME_ONETWO,
        };
        init
    },
    {
        let mut init = avmScalingModeMapList {
            avifMode: {
                let mut init = avifFraction {
                    n: 1 as int32_t,
                    d: 4 as int32_t,
                };
                init
            },
            avmMode: AVME_ONEFOUR,
        };
        init
    },
    {
        let mut init = avmScalingModeMapList {
            avifMode: {
                let mut init = avifFraction {
                    n: 1 as int32_t,
                    d: 8 as int32_t,
                };
                init
            },
            avmMode: AVME_ONEEIGHT,
        };
        init
    },
    {
        let mut init = avmScalingModeMapList {
            avifMode: {
                let mut init = avifFraction {
                    n: 3 as int32_t,
                    d: 4 as int32_t,
                };
                init
            },
            avmMode: AVME_THREEFOUR,
        };
        init
    },
    {
        let mut init = avmScalingModeMapList {
            avifMode: {
                let mut init = avifFraction {
                    n: 3 as int32_t,
                    d: 5 as int32_t,
                };
                init
            },
            avmMode: AVME_THREEFIVE,
        };
        init
    },
    {
        let mut init = avmScalingModeMapList {
            avifMode: {
                let mut init = avifFraction {
                    n: 4 as int32_t,
                    d: 5 as int32_t,
                };
                init
            },
            avmMode: AVME_FOURFIVE,
        };
        init
    },
];
static mut scalingModeMapSize: core::ffi::c_int = 0;
unsafe extern "C" fn avifFindAVMScalingMode(
    mut avifMode: *const avifFraction,
    mut avmMode: *mut AVM_SCALING_MODE,
) -> avifBool {
    let mut simplifiedFraction: avifFraction = *avifMode;
    avifFractionSimplify(&mut simplifiedFraction);
    let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
    while i < scalingModeMapSize {
        if scalingModeMap[i as usize].avifMode.n == simplifiedFraction.n
            && scalingModeMap[i as usize].avifMode.d == simplifiedFraction.d
        {
            *avmMode = scalingModeMap[i as usize].avmMode;
            return AVIF_TRUE;
        }
        i += 1;
    }
    return AVIF_FALSE;
}
unsafe extern "C" fn avmScaleQuantizer(
    mut quantizer: core::ffi::c_int,
    mut depth: uint32_t,
) -> core::ffi::c_int {
    if depth == 10 as uint32_t {
        return if ((quantizer * (255 as core::ffi::c_int + 48 as core::ffi::c_int)
            + 31 as core::ffi::c_int) / 63 as core::ffi::c_int - 48 as core::ffi::c_int)
            < -(48 as core::ffi::c_int)
        {
            -(48 as core::ffi::c_int)
        } else if (255 as core::ffi::c_int)
            < (quantizer * (255 as core::ffi::c_int + 48 as core::ffi::c_int)
                + 31 as core::ffi::c_int) / 63 as core::ffi::c_int
                - 48 as core::ffi::c_int
        {
            255 as core::ffi::c_int
        } else {
            (quantizer * (255 as core::ffi::c_int + 48 as core::ffi::c_int)
                + 31 as core::ffi::c_int) / 63 as core::ffi::c_int
                - 48 as core::ffi::c_int
        };
    }
    if depth == 12 as uint32_t {
        return if ((quantizer * (255 as core::ffi::c_int + 96 as core::ffi::c_int)
            + 31 as core::ffi::c_int) / 63 as core::ffi::c_int - 96 as core::ffi::c_int)
            < -(96 as core::ffi::c_int)
        {
            -(96 as core::ffi::c_int)
        } else if (255 as core::ffi::c_int)
            < (quantizer * (255 as core::ffi::c_int + 96 as core::ffi::c_int)
                + 31 as core::ffi::c_int) / 63 as core::ffi::c_int
                - 96 as core::ffi::c_int
        {
            255 as core::ffi::c_int
        } else {
            (quantizer * (255 as core::ffi::c_int + 96 as core::ffi::c_int)
                + 31 as core::ffi::c_int) / 63 as core::ffi::c_int
                - 96 as core::ffi::c_int
        };
    }
    return if ((quantizer * 255 as core::ffi::c_int + 31 as core::ffi::c_int)
        / 63 as core::ffi::c_int) < 0 as core::ffi::c_int
    {
        0 as core::ffi::c_int
    } else if (255 as core::ffi::c_int)
        < (quantizer * 255 as core::ffi::c_int + 31 as core::ffi::c_int)
            / 63 as core::ffi::c_int
    {
        255 as core::ffi::c_int
    } else {
        (quantizer * 255 as core::ffi::c_int + 31 as core::ffi::c_int)
            / 63 as core::ffi::c_int
    };
}
unsafe extern "C" fn avmQualityToQuantizer(
    mut quality: core::ffi::c_int,
    mut depth: uint32_t,
) -> core::ffi::c_int {
    if depth == 10 as uint32_t {
        return 255 as core::ffi::c_int
            - (quality * (255 as core::ffi::c_int + 48 as core::ffi::c_int)
                + 50 as core::ffi::c_int) / 100 as core::ffi::c_int;
    }
    if depth == 12 as uint32_t {
        return 255 as core::ffi::c_int
            - (quality * (255 as core::ffi::c_int + 96 as core::ffi::c_int)
                + 50 as core::ffi::c_int) / 100 as core::ffi::c_int;
    }
    return 255 as core::ffi::c_int
        - (quality * 255 as core::ffi::c_int + 50 as core::ffi::c_int)
            / 100 as core::ffi::c_int;
}
unsafe extern "C" fn avmCodecEncodeImage(
    mut codec: *mut avifCodec,
    mut encoder: *mut avifEncoder,
    mut image: *const avifImage,
    mut alpha: avifBool,
    mut tileRowsLog2: core::ffi::c_int,
    mut tileColsLog2: core::ffi::c_int,
    mut quality: core::ffi::c_int,
    mut encoderChanges: avifEncoderChanges,
    mut disableLaggedOutput: avifBool,
    mut addImageFlags: avifAddImageFlags,
    mut output: *mut avifCodecEncodeOutput,
) -> avifResult {
    if (*encoder).width != 0 || (*encoder).height != 0 {
        return AVIF_RESULT_NOT_IMPLEMENTED;
    }
    let mut cfg: *mut avm_codec_enc_cfg = &mut (*(*codec).internal).cfg;
    let mut quantizerUpdated: avifBool = AVIF_FALSE;
    let quantizer: core::ffi::c_int = avmQualityToQuantizer(quality, (*image).depth)
        as core::ffi::c_int;
    encoderChanges &= !(AVIF_ENCODER_CHANGE_SCALING_MODE as core::ffi::c_int);
    if (*(*codec).internal).encoderInitialized == 0 {
        let mut avmCpuUsed: core::ffi::c_int = -(1 as core::ffi::c_int);
        if (*encoder).speed != AVIF_SPEED_DEFAULT {
            avmCpuUsed = if (*encoder).speed < 0 as core::ffi::c_int {
                0 as core::ffi::c_int
            } else if (9 as core::ffi::c_int) < (*encoder).speed {
                9 as core::ffi::c_int
            } else {
                (*encoder).speed
            };
        }
        (*(*codec).internal).avmFormat = avifImageCalcAVMFmt(image, alpha);
        if (*(*codec).internal).avmFormat as core::ffi::c_uint
            == AVM_IMG_FMT_NONE as core::ffi::c_int as core::ffi::c_uint
        {
            return AVIF_RESULT_UNKNOWN_ERROR;
        }
        avifGetPixelFormatInfo((*image).yuvFormat, &mut (*(*codec).internal).formatInfo);
        let mut encoderInterface: *const avm_codec_iface_t = avm_codec_av2_cx();
        let mut err: avm_codec_err_t = avm_codec_enc_config_default(
            encoderInterface,
            cfg as *mut avm_codec_enc_cfg_t,
            AVM_USAGE_GOOD_QUALITY as core::ffi::c_uint,
        );
        if err as core::ffi::c_uint
            != AVM_CODEC_OK as core::ffi::c_int as core::ffi::c_uint
        {
            avifDiagnosticsPrintf(
                (*codec).diag,
                b"avm_codec_enc_config_default() failed: %s\0" as *const u8
                    as *const core::ffi::c_char,
                avm_codec_err_to_string(err),
            );
            return AVIF_RESULT_UNKNOWN_ERROR;
        }
        (*cfg).rc_end_usage = AVM_Q;
        let mut seqProfile: uint8_t = 0 as uint8_t;
        if (*image).depth != 8 as uint32_t && (*image).depth != 10 as uint32_t {
            avifDiagnosticsPrintf(
                (*codec).diag,
                b"%d-bit is not supported in AV2 encoder.\0" as *const u8
                    as *const core::ffi::c_char,
                (*image).depth,
            );
            return AVIF_RESULT_INVALID_ARGUMENT;
        }
        if alpha != 0 {
            seqProfile = 1 as uint8_t;
        } else {
            match (*image).yuvFormat as core::ffi::c_uint {
                1 => {
                    seqProfile = 4 as uint8_t;
                }
                2 => {
                    seqProfile = 3 as uint8_t;
                }
                3 => {
                    seqProfile = 1 as uint8_t;
                }
                4 => {
                    seqProfile = 1 as uint8_t;
                }
                0 | 5 | _ => {}
            }
        }
        (*cfg).g_profile = seqProfile as core::ffi::c_uint;
        (*cfg).g_bit_depth = (*image).depth as avm_bit_depth_t;
        (*cfg).g_input_bit_depth = (*image).depth as core::ffi::c_uint;
        (*cfg).g_w = (*image).width as core::ffi::c_uint;
        (*cfg).g_h = (*image).height as core::ffi::c_uint;
        if addImageFlags
            & AVIF_ADD_IMAGE_FLAG_SINGLE as core::ffi::c_int as avifAddImageFlags != 0
        {
            (*cfg).g_limit = 1 as core::ffi::c_uint;
        }
        let mut useAllIntraForLayered: avifBool = ((*encoder).extraLayerCount
            == 1 as uint32_t && quality <= TWO_LAYER_ALL_INTRA_QUALITY_THRESHOLD)
            as core::ffi::c_int;
        let mut useAllIntra: avifBool = (addImageFlags
            & AVIF_ADD_IMAGE_FLAG_SINGLE as core::ffi::c_int as avifAddImageFlags != 0
            || useAllIntraForLayered != 0) as core::ffi::c_int;
        if useAllIntra != 0 {
            (*cfg).g_lag_in_frames = 0 as core::ffi::c_uint;
            (*cfg).kf_mode = AVM_KF_DISABLED;
            (*cfg).kf_max_dist = 0 as core::ffi::c_uint;
        }
        if (*encoder).extraLayerCount > 0 as uint32_t {
            (*cfg).g_limit = ((*encoder).extraLayerCount).wrapping_add(1 as uint32_t)
                as core::ffi::c_uint;
            (*cfg).g_lag_in_frames = 0 as core::ffi::c_uint;
        }
        if disableLaggedOutput != 0 {
            (*cfg).g_lag_in_frames = 0 as core::ffi::c_uint;
        }
        if (*encoder).maxThreads > 1 as core::ffi::c_int {
            (*cfg).g_threads = (if (*encoder).maxThreads < 64 as core::ffi::c_int {
                (*encoder).maxThreads
            } else {
                64 as core::ffi::c_int
            }) as core::ffi::c_uint;
        }
        (*(*codec).internal).monochromeEnabled = AVIF_FALSE as avifBool;
        if alpha != 0
            || (*image).yuvFormat as core::ffi::c_uint
                == AVIF_PIXEL_FORMAT_YUV400 as core::ffi::c_int as core::ffi::c_uint
        {
            (*(*codec).internal).monochromeEnabled = AVIF_TRUE as avifBool;
            (*cfg).monochrome = 1 as core::ffi::c_uint;
        }
        if avifProcessAVMOptionsPreInit(codec, alpha, cfg) == 0 {
            return AVIF_RESULT_INVALID_CODEC_SPECIFIC_OPTION;
        }
        let mut minQuantizer: core::ffi::c_int = 0;
        let mut maxQuantizer: core::ffi::c_int = 0;
        if alpha != 0 {
            minQuantizer = (*encoder).minQuantizerAlpha;
            maxQuantizer = (*encoder).maxQuantizerAlpha;
        } else {
            minQuantizer = (*encoder).minQuantizer;
            maxQuantizer = (*encoder).maxQuantizer;
        }
        minQuantizer = avmScaleQuantizer(minQuantizer, (*image).depth);
        maxQuantizer = avmScaleQuantizer(maxQuantizer, (*image).depth);
        if (*cfg).rc_end_usage as core::ffi::c_uint
            == AVM_VBR as core::ffi::c_int as core::ffi::c_uint
            || (*cfg).rc_end_usage as core::ffi::c_uint
                == AVM_CBR as core::ffi::c_int as core::ffi::c_uint
        {
            if quantizer == AVIF_QUANTIZER_LOSSLESS {
                minQuantizer = AVIF_QUANTIZER_LOSSLESS;
                maxQuantizer = AVIF_QUANTIZER_LOSSLESS;
            } else {
                minQuantizer = if quantizer - 4 as core::ffi::c_int > minQuantizer {
                    quantizer - 4 as core::ffi::c_int
                } else {
                    minQuantizer
                };
                maxQuantizer = if (quantizer + 4 as core::ffi::c_int) < maxQuantizer {
                    quantizer + 4 as core::ffi::c_int
                } else {
                    maxQuantizer
                };
            }
        }
        (*cfg).rc_min_quantizer = minQuantizer;
        (*cfg).rc_max_quantizer = maxQuantizer;
        quantizerUpdated = AVIF_TRUE as avifBool;
        if avm_codec_enc_init_ver(
            &mut (*(*codec).internal).encoder,
            encoderInterface,
            cfg,
            0 as avm_codec_flags_t,
            AVM_ENCODER_ABI_VERSION,
        ) as core::ffi::c_uint != AVM_CODEC_OK as core::ffi::c_int as core::ffi::c_uint
        {
            avifDiagnosticsPrintf(
                (*codec).diag,
                b"avm_codec_enc_init() failed: %s: %s\0" as *const u8
                    as *const core::ffi::c_char,
                avm_codec_error(&mut (*(*codec).internal).encoder),
                avm_codec_error_detail(&mut (*(*codec).internal).encoder),
            );
            return AVIF_RESULT_UNKNOWN_ERROR;
        }
        (*(*codec).internal).encoderInitialized = AVIF_TRUE as avifBool;
        if (*cfg).rc_end_usage as core::ffi::c_uint
            == AVM_CQ as core::ffi::c_int as core::ffi::c_uint
            || (*cfg).rc_end_usage as core::ffi::c_uint
                == AVM_Q as core::ffi::c_int as core::ffi::c_uint
        {
            avm_codec_control(
                &mut (*(*codec).internal).encoder as *mut avm_codec_ctx_t,
                AVME_SET_QP as core::ffi::c_int,
                quantizer,
            );
        }
        let mut lossless: avifBool = (quantizer == AVIF_QUANTIZER_LOSSLESS)
            as core::ffi::c_int;
        if lossless != 0 {
            avm_codec_control(
                &mut (*(*codec).internal).encoder as *mut avm_codec_ctx_t,
                AV2E_SET_LOSSLESS as core::ffi::c_int,
                1 as core::ffi::c_int,
            );
        }
        if (*encoder).maxThreads > 1 as core::ffi::c_int {
            avm_codec_control(
                &mut (*(*codec).internal).encoder as *mut avm_codec_ctx_t,
                AV2E_SET_ROW_MT as core::ffi::c_int,
                1 as core::ffi::c_int,
            );
        }
        if tileRowsLog2 != 0 as core::ffi::c_int {
            avm_codec_control(
                &mut (*(*codec).internal).encoder as *mut avm_codec_ctx_t,
                AV2E_SET_TILE_ROWS as core::ffi::c_int,
                tileRowsLog2,
            );
        }
        if tileColsLog2 != 0 as core::ffi::c_int {
            avm_codec_control(
                &mut (*(*codec).internal).encoder as *mut avm_codec_ctx_t,
                AV2E_SET_TILE_COLUMNS as core::ffi::c_int,
                tileColsLog2,
            );
        }
        if (*encoder).extraLayerCount > 0 as uint32_t {
            let mut layerCount: core::ffi::c_int = ((*encoder).extraLayerCount)
                .wrapping_add(1 as uint32_t) as core::ffi::c_int;
            if avm_codec_control(
                &mut (*(*codec).internal).encoder as *mut avm_codec_ctx_t,
                AVME_SET_NUMBER_MLAYERS as core::ffi::c_int,
                layerCount,
            ) as core::ffi::c_uint
                != AVM_CODEC_OK as core::ffi::c_int as core::ffi::c_uint
            {
                return AVIF_RESULT_UNKNOWN_ERROR;
            }
        }
        if avmCpuUsed != -(1 as core::ffi::c_int) {
            if avm_codec_control(
                &mut (*(*codec).internal).encoder as *mut avm_codec_ctx_t,
                AVME_SET_CPUUSED as core::ffi::c_int,
                avmCpuUsed,
            ) as core::ffi::c_uint
                != AVM_CODEC_OK as core::ffi::c_int as core::ffi::c_uint
            {
                return AVIF_RESULT_UNKNOWN_ERROR;
            }
        }
        if alpha != 0 {
            avm_codec_control(
                &mut (*(*codec).internal).encoder as *mut avm_codec_ctx_t,
                AV2E_SET_COLOR_RANGE as core::ffi::c_int,
                AVM_CR_FULL_RANGE as core::ffi::c_int,
            );
        } else {
            if (*image).yuvChromaSamplePosition as core::ffi::c_uint
                != AVIF_CHROMA_SAMPLE_POSITION_UNKNOWN as core::ffi::c_int
                    as core::ffi::c_uint
            {
                avm_codec_control(
                    &mut (*(*codec).internal).encoder as *mut avm_codec_ctx_t,
                    AV2E_SET_CHROMA_SAMPLE_POSITION as core::ffi::c_int,
                    (*image).yuvChromaSamplePosition as core::ffi::c_int,
                );
            }
            if (*image).yuvRange as core::ffi::c_uint
                != AVIF_RANGE_LIMITED as core::ffi::c_int as core::ffi::c_uint
            {
                avm_codec_control(
                    &mut (*(*codec).internal).encoder as *mut avm_codec_ctx_t,
                    AV2E_SET_COLOR_RANGE as core::ffi::c_int,
                    (*image).yuvRange as core::ffi::c_int,
                );
            }
        }
        if avifProcessAVMOptionsPostInit(codec, alpha) == 0 {
            return AVIF_RESULT_INVALID_CODEC_SPECIFIC_OPTION;
        }
        if avm_codec_set_option(
            &mut (*(*codec).internal).encoder,
            b"enable-ext-partitions\0" as *const u8 as *const core::ffi::c_char,
            b"0\0" as *const u8 as *const core::ffi::c_char,
        ) as core::ffi::c_uint != AVM_CODEC_OK as core::ffi::c_int as core::ffi::c_uint
            || avm_codec_set_option(
                &mut (*(*codec).internal).encoder,
                b"enable-uneven-4way-partitions\0" as *const u8
                    as *const core::ffi::c_char,
                b"0\0" as *const u8 as *const core::ffi::c_char,
            ) as core::ffi::c_uint
                != AVM_CODEC_OK as core::ffi::c_int as core::ffi::c_uint
        {
            return AVIF_RESULT_UNKNOWN_ERROR;
        }
        if (*(*codec).internal).tuningSet == 0 {
            if avm_codec_control(
                &mut (*(*codec).internal).encoder as *mut avm_codec_ctx_t,
                AVME_SET_TUNING as core::ffi::c_int,
                AVM_TUNE_SSIM as core::ffi::c_int,
            ) as core::ffi::c_uint
                != AVM_CODEC_OK as core::ffi::c_int as core::ffi::c_uint
            {
                return AVIF_RESULT_UNKNOWN_ERROR;
            }
        }
    } else {
        let mut dimensionsChanged: avifBool = AVIF_FALSE;
        if (*cfg).g_w as uint32_t != (*image).width
            || (*cfg).g_h as uint32_t != (*image).height
        {
            return AVIF_RESULT_NOT_IMPLEMENTED;
        }
        if alpha != 0 {
            if encoderChanges as core::ffi::c_int
                & (AVIF_ENCODER_CHANGE_MIN_QUANTIZER_ALPHA as core::ffi::c_int
                    | AVIF_ENCODER_CHANGE_MAX_QUANTIZER_ALPHA as core::ffi::c_int) != 0
            {
                (*cfg).rc_min_quantizer = avmScaleQuantizer(
                    (*encoder).minQuantizerAlpha,
                    (*image).depth,
                );
                (*cfg).rc_max_quantizer = avmScaleQuantizer(
                    (*encoder).maxQuantizerAlpha,
                    (*image).depth,
                );
                quantizerUpdated = AVIF_TRUE as avifBool;
            }
        } else if encoderChanges as core::ffi::c_int
            & (AVIF_ENCODER_CHANGE_MIN_QUANTIZER as core::ffi::c_int
                | AVIF_ENCODER_CHANGE_MAX_QUANTIZER as core::ffi::c_int) != 0
        {
            (*cfg).rc_min_quantizer = avmScaleQuantizer(
                (*encoder).minQuantizer,
                (*image).depth,
            );
            (*cfg).rc_max_quantizer = avmScaleQuantizer(
                (*encoder).maxQuantizer,
                (*image).depth,
            );
            quantizerUpdated = AVIF_TRUE as avifBool;
        }
        let qualityChangedBit: core::ffi::c_int = if alpha != 0 {
            AVIF_ENCODER_CHANGE_QUALITY_ALPHA as core::ffi::c_int as core::ffi::c_int
        } else {
            AVIF_ENCODER_CHANGE_QUALITY as core::ffi::c_int as core::ffi::c_int
        };
        if encoderChanges as core::ffi::c_int & qualityChangedBit != 0 {
            if (*cfg).rc_end_usage as core::ffi::c_uint
                == AVM_VBR as core::ffi::c_int as core::ffi::c_uint
                || (*cfg).rc_end_usage as core::ffi::c_uint
                    == AVM_CBR as core::ffi::c_int as core::ffi::c_uint
            {
                if quantizer == AVIF_QUANTIZER_LOSSLESS {
                    (*cfg).rc_min_quantizer = AVIF_QUANTIZER_LOSSLESS;
                    (*cfg).rc_max_quantizer = AVIF_QUANTIZER_LOSSLESS;
                } else {
                    let mut minQuantizer_0: core::ffi::c_int = 0;
                    let mut maxQuantizer_0: core::ffi::c_int = 0;
                    if alpha != 0 {
                        minQuantizer_0 = (*encoder).minQuantizerAlpha;
                        maxQuantizer_0 = (*encoder).maxQuantizerAlpha;
                    } else {
                        minQuantizer_0 = (*encoder).minQuantizer;
                        maxQuantizer_0 = (*encoder).maxQuantizer;
                    }
                    minQuantizer_0 = avmScaleQuantizer(minQuantizer_0, (*image).depth);
                    maxQuantizer_0 = avmScaleQuantizer(maxQuantizer_0, (*image).depth);
                    (*cfg).rc_min_quantizer = if quantizer - 4 as core::ffi::c_int
                        > minQuantizer_0
                    {
                        quantizer - 4 as core::ffi::c_int
                    } else {
                        minQuantizer_0
                    };
                    (*cfg).rc_max_quantizer = if (quantizer + 4 as core::ffi::c_int)
                        < maxQuantizer_0
                    {
                        quantizer + 4 as core::ffi::c_int
                    } else {
                        maxQuantizer_0
                    };
                }
                quantizerUpdated = AVIF_TRUE as avifBool;
            }
        }
        if quantizerUpdated != 0 || dimensionsChanged != 0 {
            let mut err_0: avm_codec_err_t = avm_codec_enc_config_set(
                &mut (*(*codec).internal).encoder,
                cfg,
            );
            if err_0 as core::ffi::c_uint
                != AVM_CODEC_OK as core::ffi::c_int as core::ffi::c_uint
            {
                avifDiagnosticsPrintf(
                    (*codec).diag,
                    b"avm_codec_enc_config_set() failed: %s: %s\0" as *const u8
                        as *const core::ffi::c_char,
                    avm_codec_error(&mut (*(*codec).internal).encoder),
                    avm_codec_error_detail(&mut (*(*codec).internal).encoder),
                );
                return AVIF_RESULT_UNKNOWN_ERROR;
            }
        }
        if encoderChanges as core::ffi::c_int
            & AVIF_ENCODER_CHANGE_TILE_ROWS_LOG2 as core::ffi::c_int != 0
        {
            avm_codec_control(
                &mut (*(*codec).internal).encoder as *mut avm_codec_ctx_t,
                AV2E_SET_TILE_ROWS as core::ffi::c_int,
                tileRowsLog2,
            );
        }
        if encoderChanges as core::ffi::c_int
            & AVIF_ENCODER_CHANGE_TILE_COLS_LOG2 as core::ffi::c_int != 0
        {
            avm_codec_control(
                &mut (*(*codec).internal).encoder as *mut avm_codec_ctx_t,
                AV2E_SET_TILE_COLUMNS as core::ffi::c_int,
                tileColsLog2,
            );
        }
        if encoderChanges as core::ffi::c_int & qualityChangedBit != 0 {
            if (*cfg).rc_end_usage as core::ffi::c_uint
                == AVM_CQ as core::ffi::c_int as core::ffi::c_uint
                || (*cfg).rc_end_usage as core::ffi::c_uint
                    == AVM_Q as core::ffi::c_int as core::ffi::c_uint
            {
                avm_codec_control(
                    &mut (*(*codec).internal).encoder as *mut avm_codec_ctx_t,
                    AVME_SET_QP as core::ffi::c_int,
                    quantizer,
                );
            }
            let mut lossless_0: avifBool = (quantizer == AVIF_QUANTIZER_LOSSLESS)
                as core::ffi::c_int;
            avm_codec_control(
                &mut (*(*codec).internal).encoder as *mut avm_codec_ctx_t,
                AV2E_SET_LOSSLESS as core::ffi::c_int,
                lossless_0,
            );
        }
        if encoderChanges as core::ffi::c_int
            & AVIF_ENCODER_CHANGE_CODEC_SPECIFIC as core::ffi::c_int != 0
        {
            if avifProcessAVMOptionsPostInit(codec, alpha) == 0 {
                return AVIF_RESULT_INVALID_CODEC_SPECIFIC_OPTION;
            }
        }
    }
    if (*(*codec).internal).currentLayer > (*encoder).extraLayerCount {
        avifDiagnosticsPrintf(
            (*codec).diag,
            b"Too many layers sent. Expected %u layers, but got %u layers.\0"
                as *const u8 as *const core::ffi::c_char,
            ((*encoder).extraLayerCount).wrapping_add(1 as uint32_t),
            ((*(*codec).internal).currentLayer).wrapping_add(1 as uint32_t),
        );
        return AVIF_RESULT_INVALID_ARGUMENT;
    }
    if (*encoder).extraLayerCount > 0 as uint32_t {
        avm_codec_control(
            &mut (*(*codec).internal).encoder as *mut avm_codec_ctx_t,
            AVME_SET_MLAYER_ID as core::ffi::c_int,
            (*(*codec).internal).currentLayer,
        );
    }
    let mut avmScalingMode: avm_scaling_mode_t = avm_scaling_mode {
        h_scaling_mode: AVME_NORMAL,
        v_scaling_mode: AVME_NORMAL,
    };
    if avifFindAVMScalingMode(
        &mut (*encoder).scalingMode.horizontal,
        &mut avmScalingMode.h_scaling_mode,
    ) == 0
    {
        return AVIF_RESULT_NOT_IMPLEMENTED;
    }
    if avifFindAVMScalingMode(
        &mut (*encoder).scalingMode.vertical,
        &mut avmScalingMode.v_scaling_mode,
    ) == 0
    {
        return AVIF_RESULT_NOT_IMPLEMENTED;
    }
    if avmScalingMode.h_scaling_mode as core::ffi::c_uint
        != AVME_NORMAL as core::ffi::c_int as core::ffi::c_uint
        || avmScalingMode.v_scaling_mode as core::ffi::c_uint
            != AVME_NORMAL as core::ffi::c_int as core::ffi::c_uint
    {
        avm_codec_control(
            &mut (*(*codec).internal).encoder as *mut avm_codec_ctx_t,
            AVME_SET_SCALEMODE as core::ffi::c_int,
            &mut avmScalingMode as *mut avm_scaling_mode_t,
        );
    }
    let mut avmImage: avm_image_t = avm_image {
        fmt: AVM_IMG_FMT_NONE,
        cp: AVM_CICP_CP_RESERVED_0,
        tc: AVM_CICP_TC_RESERVED_0,
        mc: AVM_CICP_MC_IDENTITY,
        monochrome: 0,
        csp: AVM_CSP_LEFT,
        range: AVM_CR_STUDIO_RANGE,
        w: 0,
        h: 0,
        bit_depth: 0,
        w_conf_win_enabled_flag: 0,
        w_conf_win_left_offset: 0,
        w_conf_win_right_offset: 0,
        w_conf_win_top_offset: 0,
        w_conf_win_bottom_offset: 0,
        max_width: 0,
        max_height: 0,
        crop_width: 0,
        crop_height: 0,
        d_w: 0,
        d_h: 0,
        r_w: 0,
        r_h: 0,
        x_chroma_shift: 0,
        y_chroma_shift: 0,
        planes: [0 as *mut core::ffi::c_uchar; 3],
        stride: [0; 3],
        sz: 0,
        bps: 0,
        tlayer_id: 0,
        mlayer_id: 0,
        xlayer_id: 0,
        stream_id: 0,
        user_priv: 0 as *mut core::ffi::c_void,
        img_data: 0 as *mut core::ffi::c_uchar,
        img_data_owner: 0,
        self_allocd: 0,
        metadata: 0 as *mut avm_metadata_array_t,
        fb_priv: 0 as *mut core::ffi::c_void,
    };
    let mut avmImageAllocated: avifBool = ((*image).width == 1 as uint32_t
        || (*image).height == 1 as uint32_t) as core::ffi::c_int;
    if avmImageAllocated != 0 {
        avm_img_alloc(
            &mut avmImage,
            (*(*codec).internal).avmFormat,
            (*image).width as core::ffi::c_uint,
            (*image).height as core::ffi::c_uint,
            16 as core::ffi::c_uint,
        );
    } else {
        memset(
            &mut avmImage as *mut avm_image_t as *mut core::ffi::c_void,
            0 as core::ffi::c_int,
            ::core::mem::size_of::<avm_image_t>() as size_t,
        );
        avmImage.fmt = (*(*codec).internal).avmFormat;
        avmImage.bit_depth = (if (*image).depth > 8 as uint32_t {
            16 as core::ffi::c_int
        } else {
            8 as core::ffi::c_int
        }) as core::ffi::c_uint;
        avmImage.w = (*image).width as core::ffi::c_uint;
        avmImage.h = (*image).height as core::ffi::c_uint;
        avmImage.d_w = (*image).width as core::ffi::c_uint;
        avmImage.d_h = (*image).height as core::ffi::c_uint;
        let mut bps: core::ffi::c_uint = 0;
        if (*(*codec).internal).avmFormat as core::ffi::c_uint
            == AVM_IMG_FMT_I420 as core::ffi::c_int as core::ffi::c_uint
        {
            bps = 12 as core::ffi::c_uint;
        } else if (*(*codec).internal).avmFormat as core::ffi::c_uint
            == AVM_IMG_FMT_I422 as core::ffi::c_int as core::ffi::c_uint
        {
            bps = 16 as core::ffi::c_uint;
        } else if (*(*codec).internal).avmFormat as core::ffi::c_uint
            == AVM_IMG_FMT_I444 as core::ffi::c_int as core::ffi::c_uint
        {
            bps = 24 as core::ffi::c_uint;
        } else if (*(*codec).internal).avmFormat as core::ffi::c_uint
            == AVM_IMG_FMT_I42016 as core::ffi::c_int as core::ffi::c_uint
        {
            bps = 24 as core::ffi::c_uint;
        } else if (*(*codec).internal).avmFormat as core::ffi::c_uint
            == AVM_IMG_FMT_I42216 as core::ffi::c_int as core::ffi::c_uint
        {
            bps = 32 as core::ffi::c_uint;
        } else if (*(*codec).internal).avmFormat as core::ffi::c_uint
            == AVM_IMG_FMT_I44416 as core::ffi::c_int as core::ffi::c_uint
        {
            bps = 48 as core::ffi::c_uint;
        } else {
            bps = 16 as core::ffi::c_uint;
        }
        avmImage.bps = bps as core::ffi::c_int;
        avmImage.x_chroma_shift = (if alpha != 0
            || (*(*codec).internal).formatInfo.monochrome != 0
        {
            1 as core::ffi::c_int
        } else {
            (*(*codec).internal).formatInfo.chromaShiftX
        }) as core::ffi::c_uint;
        avmImage.y_chroma_shift = (if alpha != 0
            || (*(*codec).internal).formatInfo.monochrome != 0
        {
            1 as core::ffi::c_int
        } else {
            (*(*codec).internal).formatInfo.chromaShiftY
        }) as core::ffi::c_uint;
    }
    let mut monochromeRequested: avifBool = AVIF_FALSE;
    if alpha != 0 {
        avmImage.range = AVM_CR_FULL_RANGE;
        monochromeRequested = AVIF_TRUE as avifBool;
        if avmImageAllocated != 0 {
            let bytesPerRow: uint32_t = ((if (*image).depth > 8 as uint32_t {
                2 as core::ffi::c_int
            } else {
                1 as core::ffi::c_int
            }) as uint32_t)
                .wrapping_mul((*image).width);
            let mut j: uint32_t = 0 as uint32_t;
            while j < (*image).height {
                let mut srcAlphaRow: *const uint8_t = &mut *((*image).alphaPlane)
                    .offset(
                        (j as size_t).wrapping_mul((*image).alphaRowBytes as size_t)
                            as isize,
                    ) as *mut uint8_t;
                let mut dstAlphaRow: *mut uint8_t = &mut *(*(avmImage.planes)
                    .as_mut_ptr()
                    .offset(0 as core::ffi::c_int as isize))
                    .offset(
                        (j as size_t)
                            .wrapping_mul(
                                *(avmImage.stride)
                                    .as_mut_ptr()
                                    .offset(0 as core::ffi::c_int as isize) as size_t,
                            ) as isize,
                    ) as *mut uint8_t;
                memcpy(
                    dstAlphaRow as *mut core::ffi::c_void,
                    srcAlphaRow as *const core::ffi::c_void,
                    bytesPerRow as size_t,
                );
                j = j.wrapping_add(1);
            }
        } else {
            avmImage.planes[0 as core::ffi::c_int as usize] = (*image).alphaPlane
                as *mut core::ffi::c_uchar;
            avmImage.stride[0 as core::ffi::c_int as usize] = (*image).alphaRowBytes
                as core::ffi::c_int;
        }
    } else {
        let mut yuvPlaneCount: core::ffi::c_int = 3 as core::ffi::c_int;
        if (*image).yuvFormat as core::ffi::c_uint
            == AVIF_PIXEL_FORMAT_YUV400 as core::ffi::c_int as core::ffi::c_uint
        {
            yuvPlaneCount = 1 as core::ffi::c_int;
            monochromeRequested = AVIF_TRUE as avifBool;
        }
        if avmImageAllocated != 0 {
            let mut bytesPerPixel: uint32_t = (if (*image).depth > 8 as uint32_t {
                2 as core::ffi::c_int
            } else {
                1 as core::ffi::c_int
            }) as uint32_t;
            let mut yuvPlane: core::ffi::c_int = 0 as core::ffi::c_int;
            while yuvPlane < yuvPlaneCount {
                let mut planeWidth: uint32_t = avifImagePlaneWidth(image, yuvPlane);
                let mut planeHeight: uint32_t = avifImagePlaneHeight(image, yuvPlane);
                let mut bytesPerRow_0: uint32_t = bytesPerPixel.wrapping_mul(planeWidth);
                let mut j_0: uint32_t = 0 as uint32_t;
                while j_0 < planeHeight {
                    let mut srcRow: *const uint8_t = &mut *(*((*image).yuvPlanes)
                        .as_ptr()
                        .offset(yuvPlane as isize))
                        .offset(
                            (j_0 as size_t)
                                .wrapping_mul(
                                    *((*image).yuvRowBytes).as_ptr().offset(yuvPlane as isize)
                                        as size_t,
                                ) as isize,
                        ) as *mut uint8_t;
                    let mut dstRow: *mut uint8_t = &mut *(*(avmImage.planes)
                        .as_mut_ptr()
                        .offset(yuvPlane as isize))
                        .offset(
                            (j_0 as size_t)
                                .wrapping_mul(
                                    *(avmImage.stride).as_mut_ptr().offset(yuvPlane as isize)
                                        as size_t,
                                ) as isize,
                        ) as *mut uint8_t;
                    memcpy(
                        dstRow as *mut core::ffi::c_void,
                        srcRow as *const core::ffi::c_void,
                        bytesPerRow_0 as size_t,
                    );
                    j_0 = j_0.wrapping_add(1);
                }
                yuvPlane += 1;
            }
        } else {
            let mut yuvPlane_0: core::ffi::c_int = 0 as core::ffi::c_int;
            while yuvPlane_0 < yuvPlaneCount {
                avmImage.planes[yuvPlane_0 as usize] = (*image)
                    .yuvPlanes[yuvPlane_0 as usize] as *mut core::ffi::c_uchar;
                avmImage.stride[yuvPlane_0 as usize] = (*image)
                    .yuvRowBytes[yuvPlane_0 as usize] as core::ffi::c_int;
                yuvPlane_0 += 1;
            }
        }
        if (*image).yuvChromaSamplePosition as core::ffi::c_uint
            == AVIF_CHROMA_SAMPLE_POSITION_VERTICAL as core::ffi::c_int
                as core::ffi::c_uint
        {
            avmImage.csp = AVM_CSP_LEFT;
        } else if (*image).yuvChromaSamplePosition as core::ffi::c_uint
            == AVIF_CHROMA_SAMPLE_POSITION_COLOCATED as core::ffi::c_int
                as core::ffi::c_uint
        {
            avmImage.csp = AVM_CSP_TOPLEFT;
        } else if (*image).yuvChromaSamplePosition as core::ffi::c_uint
            == AVIF_CHROMA_SAMPLE_POSITION_RESERVED as core::ffi::c_int
                as core::ffi::c_uint
        {
            avmImage.csp = AVM_CSP_CENTER;
        } else {
            avmImage.csp = AVM_CSP_UNSPECIFIED;
        }
        avmImage.range = (*image).yuvRange as avm_color_range_t;
    }
    let mut monoUVPlane: *mut core::ffi::c_uchar = 0 as *mut core::ffi::c_uchar;
    if monochromeRequested != 0 {
        if (*(*codec).internal).monochromeEnabled != 0 {
            avmImage.monochrome = 1 as core::ffi::c_int;
        } else {
            let mut monoUVWidth: uint32_t = ((*image).width).wrapping_add(1 as uint32_t)
                >> 1 as core::ffi::c_int;
            let mut monoUVHeight: uint32_t = ((*image).height)
                .wrapping_add(1 as uint32_t) >> 1 as core::ffi::c_int;
            if avmImageAllocated == 0 {
                let mut channelSize: uint32_t = (if avifImageUsesU16(image) != 0 {
                    2 as core::ffi::c_int
                } else {
                    1 as core::ffi::c_int
                }) as uint32_t;
                let mut monoUVRowBytes: uint32_t = channelSize.wrapping_mul(monoUVWidth);
                let mut monoUVSize: size_t = (monoUVHeight as size_t)
                    .wrapping_mul(monoUVRowBytes as size_t);
                monoUVPlane = avifAlloc(monoUVSize) as *mut core::ffi::c_uchar;
                if monoUVPlane.is_null() {
                    return AVIF_RESULT_OUT_OF_MEMORY;
                }
                avmImage.planes[1 as core::ffi::c_int as usize] = monoUVPlane;
                avmImage.stride[1 as core::ffi::c_int as usize] = monoUVRowBytes
                    as core::ffi::c_int;
            }
            if (*image).depth > 8 as uint32_t {
                let half: uint16_t = ((1 as core::ffi::c_int)
                    << ((*image).depth).wrapping_sub(1 as uint32_t)) as uint16_t;
                let mut j_1: uint32_t = 0 as uint32_t;
                while j_1 < monoUVHeight {
                    let mut dstRow_0: *mut uint16_t = &mut *(*(avmImage.planes)
                        .as_mut_ptr()
                        .offset(1 as core::ffi::c_int as isize))
                        .offset(
                            (j_1 as size_t)
                                .wrapping_mul(
                                    *(avmImage.stride)
                                        .as_mut_ptr()
                                        .offset(1 as core::ffi::c_int as isize) as size_t,
                                ) as isize,
                        ) as *mut core::ffi::c_uchar as *mut uint16_t;
                    let mut i: uint32_t = 0 as uint32_t;
                    while i < monoUVWidth {
                        *dstRow_0.offset(i as isize) = half;
                        i = i.wrapping_add(1);
                    }
                    j_1 = j_1.wrapping_add(1);
                }
            } else {
                let half_0: uint8_t = 128 as uint8_t;
                let mut planeSize: size_t = (monoUVHeight as size_t)
                    .wrapping_mul(
                        avmImage.stride[1 as core::ffi::c_int as usize] as size_t,
                    );
                memset(
                    avmImage.planes[1 as core::ffi::c_int as usize]
                        as *mut core::ffi::c_void,
                    half_0 as core::ffi::c_int,
                    planeSize,
                );
            }
            avmImage.planes[2 as core::ffi::c_int as usize] = avmImage
                .planes[1 as core::ffi::c_int as usize];
            avmImage.stride[2 as core::ffi::c_int as usize] = avmImage
                .stride[1 as core::ffi::c_int as usize];
        }
    }
    let mut encodeFlags: avm_enc_frame_flags_t = 0 as avm_enc_frame_flags_t;
    if addImageFlags
        & AVIF_ADD_IMAGE_FLAG_FORCE_KEYFRAME as core::ffi::c_int as avifAddImageFlags
        != 0
    {
        encodeFlags |= AVM_EFLAG_FORCE_KF as avm_enc_frame_flags_t;
    }
    if (*(*codec).internal).currentLayer > 0 as uint32_t {
        encodeFlags
            |= (AVM_EFLAG_NO_REF_GF | AVM_EFLAG_NO_REF_ARF | AVM_EFLAG_NO_REF_BWD
                | AVM_EFLAG_NO_REF_ARF2 | AVM_EFLAG_NO_UPD_ALL) as avm_enc_frame_flags_t;
    }
    let mut encodeErr: avm_codec_err_t = avm_codec_encode(
        &mut (*(*codec).internal).encoder,
        &mut avmImage,
        0 as avm_codec_pts_t,
        1 as core::ffi::c_ulong,
        encodeFlags,
    );
    avifFree(monoUVPlane as *mut core::ffi::c_void);
    if avmImageAllocated != 0 {
        avm_img_free(&mut avmImage);
    }
    if encodeErr as core::ffi::c_uint
        != AVM_CODEC_OK as core::ffi::c_int as core::ffi::c_uint
    {
        avifDiagnosticsPrintf(
            (*codec).diag,
            b"avm_codec_encode() failed: %s: %s\0" as *const u8
                as *const core::ffi::c_char,
            avm_codec_error(&mut (*(*codec).internal).encoder),
            avm_codec_error_detail(&mut (*(*codec).internal).encoder),
        );
        return AVIF_RESULT_UNKNOWN_ERROR;
    }
    let mut iter: avm_codec_iter_t = 0 as avm_codec_iter_t;
    loop {
        let mut pkt: *const avm_codec_cx_pkt_t = avm_codec_get_cx_data(
            &mut (*(*codec).internal).encoder,
            &mut iter,
        );
        if pkt.is_null() {
            break;
        }
        if (*pkt).kind as core::ffi::c_uint
            == AVM_CODEC_CX_FRAME_PKT as core::ffi::c_int as core::ffi::c_uint
        {
            let result__: avifResult = avifCodecEncodeOutputAddSample(
                output,
                (*pkt).data.frame.buf as *const uint8_t,
                (*pkt).data.frame.sz,
                ((*pkt).data.frame.flags & 0x1 as avm_codec_frame_flags_t) as avifBool,
            ) as avifResult;
            if result__ as core::ffi::c_uint
                != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
            {
                return result__;
            }
        }
    }
    if addImageFlags
        & AVIF_ADD_IMAGE_FLAG_SINGLE as core::ffi::c_int as avifAddImageFlags != 0
        || (*encoder).extraLayerCount > 0 as uint32_t
            && (*encoder).extraLayerCount == (*(*codec).internal).currentLayer
    {
        if avmCodecEncodeFinish(codec, output) == 0 {
            return AVIF_RESULT_UNKNOWN_ERROR;
        }
        avm_codec_destroy(&mut (*(*codec).internal).encoder);
        (*(*codec).internal).encoderInitialized = AVIF_FALSE as avifBool;
    }
    if (*encoder).extraLayerCount > 0 as uint32_t {
        (*(*codec).internal).currentLayer = ((*(*codec).internal).currentLayer)
            .wrapping_add(1);
    }
    return AVIF_RESULT_OK;
}
unsafe extern "C" fn avmCodecEncodeFinish(
    mut codec: *mut avifCodec,
    mut output: *mut avifCodecEncodeOutput,
) -> avifBool {
    if (*(*codec).internal).encoderInitialized == 0 {
        return AVIF_TRUE;
    }
    loop {
        if avm_codec_encode(
            &mut (*(*codec).internal).encoder,
            0 as *const avm_image_t,
            0 as avm_codec_pts_t,
            1 as core::ffi::c_ulong,
            0 as avm_enc_frame_flags_t,
        ) as core::ffi::c_uint != AVM_CODEC_OK as core::ffi::c_int as core::ffi::c_uint
        {
            avifDiagnosticsPrintf(
                (*codec).diag,
                b"avm_codec_encode() with img=NULL failed: %s: %s\0" as *const u8
                    as *const core::ffi::c_char,
                avm_codec_error(&mut (*(*codec).internal).encoder),
                avm_codec_error_detail(&mut (*(*codec).internal).encoder),
            );
            return AVIF_FALSE;
        }
        let mut gotPacket: avifBool = AVIF_FALSE;
        let mut iter: avm_codec_iter_t = 0 as avm_codec_iter_t;
        loop {
            let mut pkt: *const avm_codec_cx_pkt_t = avm_codec_get_cx_data(
                &mut (*(*codec).internal).encoder,
                &mut iter,
            );
            if pkt.is_null() {
                break;
            }
            if (*pkt).kind as core::ffi::c_uint
                == AVM_CODEC_CX_FRAME_PKT as core::ffi::c_int as core::ffi::c_uint
            {
                gotPacket = AVIF_TRUE as avifBool;
                let result: avifResult = avifCodecEncodeOutputAddSample(
                    output,
                    (*pkt).data.frame.buf as *const uint8_t,
                    (*pkt).data.frame.sz,
                    ((*pkt).data.frame.flags
                        & AVM_FRAME_IS_KEY as avm_codec_frame_flags_t) as avifBool,
                ) as avifResult;
                if result as core::ffi::c_uint
                    != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
                {
                    avifDiagnosticsPrintf(
                        (*codec).diag,
                        b"avifCodecEncodeOutputAddSample() failed: %s\0" as *const u8
                            as *const core::ffi::c_char,
                        avifResultToString(result),
                    );
                    return AVIF_FALSE;
                }
            }
        }
        if gotPacket == 0 {
            break;
        }
    }
    return AVIF_TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn avifCodecVersionAVM() -> *const core::ffi::c_char {
    return avm_codec_version_str();
}
#[no_mangle]
pub unsafe extern "C" fn avifCodecCreateAVM() -> *mut avifCodec {
    let mut codec: *mut avifCodec = avifCalloc(
        1 as size_t,
        ::core::mem::size_of::<avifCodec>() as size_t,
    ) as *mut avifCodec;
    if codec.is_null() {
        return 0 as *mut avifCodec;
    }
    (*codec).getNextImage = Some(
        avmCodecGetNextImage
            as unsafe extern "C" fn(
                *mut avifCodec,
                *const avifDecodeSample,
                avifBool,
                *mut avifBool,
                *mut avifImage,
            ) -> avifBool,
    ) as avifCodecGetNextImageFunc;
    (*codec).encodeImage = Some(
        avmCodecEncodeImage
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
                avifAddImageFlags,
                *mut avifCodecEncodeOutput,
            ) -> avifResult,
    ) as avifCodecEncodeImageFunc;
    (*codec).encodeFinish = Some(
        avmCodecEncodeFinish
            as unsafe extern "C" fn(
                *mut avifCodec,
                *mut avifCodecEncodeOutput,
            ) -> avifBool,
    ) as avifCodecEncodeFinishFunc;
    (*codec).destroyInternal = Some(
        avmCodecDestroyInternal as unsafe extern "C" fn(*mut avifCodec) -> (),
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
unsafe extern "C" fn run_static_initializers() {
    scalingModeMapSize = (::core::mem::size_of::<[avmScalingModeMapList; 7]>() as usize)
        .wrapping_div(::core::mem::size_of::<avmScalingModeMapList>() as usize)
        as core::ffi::c_int;
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
