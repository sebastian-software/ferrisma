// Copyright 2019 Joe Drago. All rights reserved.
// SPDX-License-Identifier: BSD-2-Clause

// Mechanically translated from upstream/libavif/src/codec_aom.c.
// C ABI, ownership, pointer validity, and bounds follow the original headers.
extern "C" {
    pub type avifEncoderData;
    pub type aom_codec_priv;
    pub type aom_codec_iface;
    pub type aom_metadata_array;
    fn avifFree(p: *mut core::ffi::c_void);
    fn avifResultToString(result: avifResult) -> *const core::ffi::c_char;
    fn avifGetPixelFormatInfo(format: avifPixelFormat, info: *mut avifPixelFormatInfo);
    fn avifImageFreePlanes(image: *mut avifImage, planes: avifPlanesFlags);
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
    fn aom_img_alloc(
        img: *mut aom_image_t,
        fmt: aom_img_fmt_t,
        d_w: core::ffi::c_uint,
        d_h: core::ffi::c_uint,
        align: core::ffi::c_uint,
    ) -> *mut aom_image_t;
    fn aom_img_free(img: *mut aom_image_t);
    fn aom_codec_version() -> core::ffi::c_int;
    fn aom_codec_version_str() -> *const core::ffi::c_char;
    fn aom_codec_err_to_string(err: aom_codec_err_t) -> *const core::ffi::c_char;
    fn aom_codec_error(ctx: *const aom_codec_ctx_t) -> *const core::ffi::c_char;
    fn aom_codec_error_detail(ctx: *const aom_codec_ctx_t) -> *const core::ffi::c_char;
    fn aom_codec_destroy(ctx: *mut aom_codec_ctx_t) -> aom_codec_err_t;
    fn aom_codec_control(
        ctx: *mut aom_codec_ctx_t,
        ctrl_id: core::ffi::c_int,
        ...
    ) -> aom_codec_err_t;
    fn aom_codec_set_option(
        ctx: *mut aom_codec_ctx_t,
        name: *const core::ffi::c_char,
        value: *const core::ffi::c_char,
    ) -> aom_codec_err_t;
    fn aom_codec_enc_init_ver(
        ctx: *mut aom_codec_ctx_t,
        iface: *const aom_codec_iface_t,
        cfg: *const aom_codec_enc_cfg_t,
        flags: aom_codec_flags_t,
        ver: core::ffi::c_int,
    ) -> aom_codec_err_t;
    fn aom_codec_enc_config_default(
        iface: *const aom_codec_iface_t,
        cfg: *mut aom_codec_enc_cfg_t,
        usage: core::ffi::c_uint,
    ) -> aom_codec_err_t;
    fn aom_codec_enc_config_set(
        ctx: *mut aom_codec_ctx_t,
        cfg: *const aom_codec_enc_cfg_t,
    ) -> aom_codec_err_t;
    fn aom_codec_encode(
        ctx: *mut aom_codec_ctx_t,
        img: *const aom_image_t,
        pts: aom_codec_pts_t,
        duration: core::ffi::c_ulong,
        flags: aom_enc_frame_flags_t,
    ) -> aom_codec_err_t;
    fn aom_codec_get_cx_data(
        ctx: *mut aom_codec_ctx_t,
        iter: *mut aom_codec_iter_t,
    ) -> *const aom_codec_cx_pkt_t;
    fn aom_codec_av1_cx() -> *const aom_codec_iface_t;
    fn aom_codec_dec_init_ver(
        ctx: *mut aom_codec_ctx_t,
        iface: *const aom_codec_iface_t,
        cfg: *const aom_codec_dec_cfg_t,
        flags: aom_codec_flags_t,
        ver: core::ffi::c_int,
    ) -> aom_codec_err_t;
    fn aom_codec_decode(
        ctx: *mut aom_codec_ctx_t,
        data: *const uint8_t,
        data_sz: size_t,
        user_priv: *mut core::ffi::c_void,
    ) -> aom_codec_err_t;
    fn aom_codec_get_frame(
        ctx: *mut aom_codec_ctx_t,
        iter: *mut aom_codec_iter_t,
    ) -> *mut aom_image_t;
    fn aom_codec_av1_dx() -> *const aom_codec_iface_t;
    fn __assert_fail(
        __assertion: *const core::ffi::c_char,
        __file: *const core::ffi::c_char,
        __line: core::ffi::c_uint,
        __function: *const core::ffi::c_char,
    ) -> !;
    fn strtol(
        __nptr: *const core::ffi::c_char,
        __endptr: *mut *mut core::ffi::c_char,
        __base: core::ffi::c_int,
    ) -> core::ffi::c_long;
    fn memcpy(
        __dest: *mut core::ffi::c_void,
        __src: *const core::ffi::c_void,
        __n: size_t,
    ) -> *mut core::ffi::c_void;
    fn memset(
        __s: *mut core::ffi::c_void,
        __c: core::ffi::c_int,
        __n: size_t,
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
pub type __uint8_t = u8;
pub type __uint16_t = u16;
pub type __int32_t = i32;
pub type __uint32_t = u32;
pub type __int64_t = i64;
pub type __uint64_t = u64;
pub type int32_t = __int32_t;
pub type int64_t = __int64_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
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
pub type C2RustUnnamed = core::ffi::c_uint;
pub const AVIF_COLOR_PRIMARIES_EBU3213: C2RustUnnamed = 22;
pub const AVIF_COLOR_PRIMARIES_DCI_P3: C2RustUnnamed = 12;
pub const AVIF_COLOR_PRIMARIES_SMPTE432: C2RustUnnamed = 12;
pub const AVIF_COLOR_PRIMARIES_SMPTE431: C2RustUnnamed = 11;
pub const AVIF_COLOR_PRIMARIES_XYZ: C2RustUnnamed = 10;
pub const AVIF_COLOR_PRIMARIES_BT2100: C2RustUnnamed = 9;
pub const AVIF_COLOR_PRIMARIES_BT2020: C2RustUnnamed = 9;
pub const AVIF_COLOR_PRIMARIES_GENERIC_FILM: C2RustUnnamed = 8;
pub const AVIF_COLOR_PRIMARIES_SMPTE240: C2RustUnnamed = 7;
pub const AVIF_COLOR_PRIMARIES_BT601: C2RustUnnamed = 6;
pub const AVIF_COLOR_PRIMARIES_BT470BG: C2RustUnnamed = 5;
pub const AVIF_COLOR_PRIMARIES_BT470M: C2RustUnnamed = 4;
pub const AVIF_COLOR_PRIMARIES_UNSPECIFIED: C2RustUnnamed = 2;
pub const AVIF_COLOR_PRIMARIES_IEC61966_2_4: C2RustUnnamed = 1;
pub const AVIF_COLOR_PRIMARIES_SRGB: C2RustUnnamed = 1;
pub const AVIF_COLOR_PRIMARIES_BT709: C2RustUnnamed = 1;
pub const AVIF_COLOR_PRIMARIES_UNKNOWN: C2RustUnnamed = 0;
pub type avifColorPrimaries = uint16_t;
pub type C2RustUnnamed_0 = core::ffi::c_uint;
pub const AVIF_TRANSFER_CHARACTERISTICS_HLG: C2RustUnnamed_0 = 18;
pub const AVIF_TRANSFER_CHARACTERISTICS_SMPTE428: C2RustUnnamed_0 = 17;
pub const AVIF_TRANSFER_CHARACTERISTICS_SMPTE2084: C2RustUnnamed_0 = 16;
pub const AVIF_TRANSFER_CHARACTERISTICS_PQ: C2RustUnnamed_0 = 16;
pub const AVIF_TRANSFER_CHARACTERISTICS_BT2020_12BIT: C2RustUnnamed_0 = 15;
pub const AVIF_TRANSFER_CHARACTERISTICS_BT2020_10BIT: C2RustUnnamed_0 = 14;
pub const AVIF_TRANSFER_CHARACTERISTICS_SRGB: C2RustUnnamed_0 = 13;
pub const AVIF_TRANSFER_CHARACTERISTICS_BT1361: C2RustUnnamed_0 = 12;
pub const AVIF_TRANSFER_CHARACTERISTICS_IEC61966: C2RustUnnamed_0 = 11;
pub const AVIF_TRANSFER_CHARACTERISTICS_LOG100_SQRT10: C2RustUnnamed_0 = 10;
pub const AVIF_TRANSFER_CHARACTERISTICS_LOG100: C2RustUnnamed_0 = 9;
pub const AVIF_TRANSFER_CHARACTERISTICS_LINEAR: C2RustUnnamed_0 = 8;
pub const AVIF_TRANSFER_CHARACTERISTICS_SMPTE240: C2RustUnnamed_0 = 7;
pub const AVIF_TRANSFER_CHARACTERISTICS_BT601: C2RustUnnamed_0 = 6;
pub const AVIF_TRANSFER_CHARACTERISTICS_BT470BG: C2RustUnnamed_0 = 5;
pub const AVIF_TRANSFER_CHARACTERISTICS_BT470M: C2RustUnnamed_0 = 4;
pub const AVIF_TRANSFER_CHARACTERISTICS_UNSPECIFIED: C2RustUnnamed_0 = 2;
pub const AVIF_TRANSFER_CHARACTERISTICS_BT709: C2RustUnnamed_0 = 1;
pub const AVIF_TRANSFER_CHARACTERISTICS_UNKNOWN: C2RustUnnamed_0 = 0;
pub type avifTransferCharacteristics = uint16_t;
pub type C2RustUnnamed_1 = core::ffi::c_uint;
pub const AVIF_MATRIX_COEFFICIENTS_LAST: C2RustUnnamed_1 = 18;
pub const AVIF_MATRIX_COEFFICIENTS_YCGCO_RO: C2RustUnnamed_1 = 17;
pub const AVIF_MATRIX_COEFFICIENTS_YCGCO_RE: C2RustUnnamed_1 = 16;
pub const AVIF_MATRIX_COEFFICIENTS_ICTCP: C2RustUnnamed_1 = 14;
pub const AVIF_MATRIX_COEFFICIENTS_CHROMA_DERIVED_CL: C2RustUnnamed_1 = 13;
pub const AVIF_MATRIX_COEFFICIENTS_CHROMA_DERIVED_NCL: C2RustUnnamed_1 = 12;
pub const AVIF_MATRIX_COEFFICIENTS_SMPTE2085: C2RustUnnamed_1 = 11;
pub const AVIF_MATRIX_COEFFICIENTS_BT2020_CL: C2RustUnnamed_1 = 10;
pub const AVIF_MATRIX_COEFFICIENTS_BT2020_NCL: C2RustUnnamed_1 = 9;
pub const AVIF_MATRIX_COEFFICIENTS_YCGCO: C2RustUnnamed_1 = 8;
pub const AVIF_MATRIX_COEFFICIENTS_SMPTE240: C2RustUnnamed_1 = 7;
pub const AVIF_MATRIX_COEFFICIENTS_BT601: C2RustUnnamed_1 = 6;
pub const AVIF_MATRIX_COEFFICIENTS_BT470BG: C2RustUnnamed_1 = 5;
pub const AVIF_MATRIX_COEFFICIENTS_FCC: C2RustUnnamed_1 = 4;
pub const AVIF_MATRIX_COEFFICIENTS_UNSPECIFIED: C2RustUnnamed_1 = 2;
pub const AVIF_MATRIX_COEFFICIENTS_BT709: C2RustUnnamed_1 = 1;
pub const AVIF_MATRIX_COEFFICIENTS_IDENTITY: C2RustUnnamed_1 = 0;
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
    pub decoder: aom_codec_ctx_t,
    pub iter: aom_codec_iter_t,
    pub image: *mut aom_image_t,
    pub encoderInitialized: avifBool,
    pub encoder: aom_codec_ctx_t,
    pub cfg: aom_codec_enc_cfg,
    pub formatInfo: avifPixelFormatInfo,
    pub aomFormat: aom_img_fmt_t,
    pub currentLayer: uint32_t,
    pub qualityFirstLayer: core::ffi::c_int,
    pub previousFrameUsedTuneIq: avifBool,
}
pub type aom_img_fmt_t = aom_img_fmt;
pub type aom_img_fmt = core::ffi::c_uint;
pub const AOM_IMG_FMT_I44416: aom_img_fmt = 2310;
pub const AOM_IMG_FMT_I42216: aom_img_fmt = 2309;
pub const AOM_IMG_FMT_YV1216: aom_img_fmt = 2817;
pub const AOM_IMG_FMT_I42016: aom_img_fmt = 2306;
pub const AOM_IMG_FMT_NV12: aom_img_fmt = 263;
pub const AOM_IMG_FMT_I444: aom_img_fmt = 262;
pub const AOM_IMG_FMT_I422: aom_img_fmt = 261;
pub const AOM_IMG_FMT_AOMI420: aom_img_fmt = 260;
pub const AOM_IMG_FMT_AOMYV12: aom_img_fmt = 771;
pub const AOM_IMG_FMT_I420: aom_img_fmt = 258;
pub const AOM_IMG_FMT_YV12: aom_img_fmt = 769;
pub const AOM_IMG_FMT_NONE: aom_img_fmt = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aom_codec_enc_cfg {
    pub g_usage: core::ffi::c_uint,
    pub g_threads: core::ffi::c_uint,
    pub g_profile: core::ffi::c_uint,
    pub g_w: core::ffi::c_uint,
    pub g_h: core::ffi::c_uint,
    pub g_limit: core::ffi::c_uint,
    pub g_forced_max_frame_width: core::ffi::c_uint,
    pub g_forced_max_frame_height: core::ffi::c_uint,
    pub g_bit_depth: aom_bit_depth_t,
    pub g_input_bit_depth: core::ffi::c_uint,
    pub g_timebase: aom_rational,
    pub g_error_resilient: aom_codec_er_flags_t,
    pub g_pass: aom_enc_pass,
    pub g_lag_in_frames: core::ffi::c_uint,
    pub rc_dropframe_thresh: core::ffi::c_uint,
    pub rc_resize_mode: core::ffi::c_uint,
    pub rc_resize_denominator: core::ffi::c_uint,
    pub rc_resize_kf_denominator: core::ffi::c_uint,
    pub rc_superres_mode: aom_superres_mode,
    pub rc_superres_denominator: core::ffi::c_uint,
    pub rc_superres_kf_denominator: core::ffi::c_uint,
    pub rc_superres_qthresh: core::ffi::c_uint,
    pub rc_superres_kf_qthresh: core::ffi::c_uint,
    pub rc_end_usage: aom_rc_mode,
    pub rc_twopass_stats_in: aom_fixed_buf_t,
    pub rc_firstpass_mb_stats_in: aom_fixed_buf_t,
    pub rc_target_bitrate: core::ffi::c_uint,
    pub rc_min_quantizer: core::ffi::c_uint,
    pub rc_max_quantizer: core::ffi::c_uint,
    pub rc_undershoot_pct: core::ffi::c_uint,
    pub rc_overshoot_pct: core::ffi::c_uint,
    pub rc_buf_sz: core::ffi::c_uint,
    pub rc_buf_initial_sz: core::ffi::c_uint,
    pub rc_buf_optimal_sz: core::ffi::c_uint,
    pub rc_2pass_vbr_bias_pct: core::ffi::c_uint,
    pub rc_2pass_vbr_minsection_pct: core::ffi::c_uint,
    pub rc_2pass_vbr_maxsection_pct: core::ffi::c_uint,
    pub fwd_kf_enabled: core::ffi::c_int,
    pub kf_mode: aom_kf_mode,
    pub kf_min_dist: core::ffi::c_uint,
    pub kf_max_dist: core::ffi::c_uint,
    pub sframe_dist: core::ffi::c_uint,
    pub sframe_mode: core::ffi::c_uint,
    pub large_scale_tile: core::ffi::c_uint,
    pub monochrome: core::ffi::c_uint,
    pub full_still_picture_hdr: core::ffi::c_uint,
    pub save_as_annexb: core::ffi::c_uint,
    pub tile_width_count: core::ffi::c_int,
    pub tile_height_count: core::ffi::c_int,
    pub tile_widths: [core::ffi::c_int; 64],
    pub tile_heights: [core::ffi::c_int; 64],
    pub use_fixed_qp_offsets: core::ffi::c_uint,
    pub fixed_qp_offsets: [core::ffi::c_int; 5],
    pub encoder_cfg: cfg_options_t,
}
pub type cfg_options_t = cfg_options;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cfg_options {
    pub init_by_cfg_file: core::ffi::c_uint,
    pub super_block_size: core::ffi::c_uint,
    pub max_partition_size: core::ffi::c_uint,
    pub min_partition_size: core::ffi::c_uint,
    pub disable_ab_partition_type: core::ffi::c_uint,
    pub disable_rect_partition_type: core::ffi::c_uint,
    pub disable_1to4_partition_type: core::ffi::c_uint,
    pub disable_flip_idtx: core::ffi::c_uint,
    pub disable_cdef: core::ffi::c_uint,
    pub disable_lr: core::ffi::c_uint,
    pub disable_obmc: core::ffi::c_uint,
    pub disable_warp_motion: core::ffi::c_uint,
    pub disable_global_motion: core::ffi::c_uint,
    pub disable_dist_wtd_comp: core::ffi::c_uint,
    pub disable_diff_wtd_comp: core::ffi::c_uint,
    pub disable_inter_intra_comp: core::ffi::c_uint,
    pub disable_masked_comp: core::ffi::c_uint,
    pub disable_one_sided_comp: core::ffi::c_uint,
    pub disable_palette: core::ffi::c_uint,
    pub disable_intrabc: core::ffi::c_uint,
    pub disable_cfl: core::ffi::c_uint,
    pub disable_smooth_intra: core::ffi::c_uint,
    pub disable_filter_intra: core::ffi::c_uint,
    pub disable_dual_filter: core::ffi::c_uint,
    pub disable_intra_angle_delta: core::ffi::c_uint,
    pub disable_intra_edge_filter: core::ffi::c_uint,
    pub disable_tx_64x64: core::ffi::c_uint,
    pub disable_smooth_inter_intra: core::ffi::c_uint,
    pub disable_inter_inter_wedge: core::ffi::c_uint,
    pub disable_inter_intra_wedge: core::ffi::c_uint,
    pub disable_paeth_intra: core::ffi::c_uint,
    pub disable_trellis_quant: core::ffi::c_uint,
    pub disable_ref_frame_mv: core::ffi::c_uint,
    pub reduced_reference_set: core::ffi::c_uint,
    pub reduced_tx_type_set: core::ffi::c_uint,
}
pub type aom_kf_mode = core::ffi::c_uint;
pub const AOM_KF_DISABLED: aom_kf_mode = 0;
pub const AOM_KF_AUTO: aom_kf_mode = 1;
pub const AOM_KF_FIXED: aom_kf_mode = 0;
pub type aom_fixed_buf_t = aom_fixed_buf;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aom_fixed_buf {
    pub buf: *mut core::ffi::c_void,
    pub sz: size_t,
}
pub type aom_rc_mode = core::ffi::c_uint;
pub const AOM_Q: aom_rc_mode = 3;
pub const AOM_CQ: aom_rc_mode = 2;
pub const AOM_CBR: aom_rc_mode = 1;
pub const AOM_VBR: aom_rc_mode = 0;
pub type aom_superres_mode = core::ffi::c_uint;
pub const AOM_SUPERRES_AUTO: aom_superres_mode = 4;
pub const AOM_SUPERRES_QTHRESH: aom_superres_mode = 3;
pub const AOM_SUPERRES_RANDOM: aom_superres_mode = 2;
pub const AOM_SUPERRES_FIXED: aom_superres_mode = 1;
pub const AOM_SUPERRES_NONE: aom_superres_mode = 0;
pub type aom_enc_pass = core::ffi::c_uint;
pub const AOM_RC_LAST_PASS: aom_enc_pass = 2;
pub const AOM_RC_THIRD_PASS: aom_enc_pass = 3;
pub const AOM_RC_SECOND_PASS: aom_enc_pass = 2;
pub const AOM_RC_FIRST_PASS: aom_enc_pass = 1;
pub const AOM_RC_ONE_PASS: aom_enc_pass = 0;
pub type aom_codec_er_flags_t = uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aom_rational {
    pub num: core::ffi::c_int,
    pub den: core::ffi::c_int,
}
pub type aom_bit_depth_t = aom_bit_depth;
pub type aom_bit_depth = core::ffi::c_uint;
pub const AOM_BITS_12: aom_bit_depth = 12;
pub const AOM_BITS_10: aom_bit_depth = 10;
pub const AOM_BITS_8: aom_bit_depth = 8;
pub type aom_codec_ctx_t = aom_codec_ctx;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aom_codec_ctx {
    pub name: *const core::ffi::c_char,
    pub iface: *const aom_codec_iface_t,
    pub err: aom_codec_err_t,
    pub err_detail: *const core::ffi::c_char,
    pub init_flags: aom_codec_flags_t,
    pub config: C2RustUnnamed_2,
    pub priv_0: *mut aom_codec_priv_t,
}
pub type aom_codec_priv_t = aom_codec_priv;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_2 {
    pub dec: *const aom_codec_dec_cfg,
    pub enc: *const aom_codec_enc_cfg,
    pub raw: *const core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aom_codec_dec_cfg {
    pub threads: core::ffi::c_uint,
    pub w: core::ffi::c_uint,
    pub h: core::ffi::c_uint,
    pub allow_lowbitdepth: core::ffi::c_uint,
}
pub type aom_codec_flags_t = core::ffi::c_long;
pub type aom_codec_err_t = core::ffi::c_uint;
pub const AOM_CODEC_LIST_END: aom_codec_err_t = 9;
pub const AOM_CODEC_INVALID_PARAM: aom_codec_err_t = 8;
pub const AOM_CODEC_CORRUPT_FRAME: aom_codec_err_t = 7;
pub const AOM_CODEC_UNSUP_FEATURE: aom_codec_err_t = 6;
pub const AOM_CODEC_UNSUP_BITSTREAM: aom_codec_err_t = 5;
pub const AOM_CODEC_INCAPABLE: aom_codec_err_t = 4;
pub const AOM_CODEC_ABI_MISMATCH: aom_codec_err_t = 3;
pub const AOM_CODEC_MEM_ERROR: aom_codec_err_t = 2;
pub const AOM_CODEC_ERROR: aom_codec_err_t = 1;
pub const AOM_CODEC_OK: aom_codec_err_t = 0;
pub type aom_codec_iface_t = aom_codec_iface;
pub type aom_image_t = aom_image;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aom_image {
    pub fmt: aom_img_fmt_t,
    pub cp: aom_color_primaries_t,
    pub tc: aom_transfer_characteristics_t,
    pub mc: aom_matrix_coefficients_t,
    pub monochrome: core::ffi::c_int,
    pub csp: aom_chroma_sample_position_t,
    pub range: aom_color_range_t,
    pub w: core::ffi::c_uint,
    pub h: core::ffi::c_uint,
    pub bit_depth: core::ffi::c_uint,
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
    pub temporal_id: core::ffi::c_int,
    pub spatial_id: core::ffi::c_int,
    pub user_priv: *mut core::ffi::c_void,
    pub img_data: *mut core::ffi::c_uchar,
    pub img_data_owner: core::ffi::c_int,
    pub self_allocd: core::ffi::c_int,
    pub metadata: *mut aom_metadata_array_t,
    pub fb_priv: *mut core::ffi::c_void,
}
pub type aom_metadata_array_t = aom_metadata_array;
pub type aom_color_range_t = aom_color_range;
pub type aom_color_range = core::ffi::c_uint;
pub const AOM_CR_FULL_RANGE: aom_color_range = 1;
pub const AOM_CR_STUDIO_RANGE: aom_color_range = 0;
pub type aom_chroma_sample_position_t = aom_chroma_sample_position;
pub type aom_chroma_sample_position = core::ffi::c_uint;
pub const AOM_CSP_RESERVED: aom_chroma_sample_position = 3;
pub const AOM_CSP_COLOCATED: aom_chroma_sample_position = 2;
pub const AOM_CSP_VERTICAL: aom_chroma_sample_position = 1;
pub const AOM_CSP_UNKNOWN: aom_chroma_sample_position = 0;
pub type aom_matrix_coefficients_t = aom_matrix_coefficients;
pub type aom_matrix_coefficients = core::ffi::c_uint;
pub const AOM_CICP_MC_YCGCO_RO: aom_matrix_coefficients = 17;
pub const AOM_CICP_MC_YCGCO_RE: aom_matrix_coefficients = 16;
pub const AOM_CICP_MC_IPT_C2: aom_matrix_coefficients = 15;
pub const AOM_CICP_MC_RESERVED_15: aom_matrix_coefficients = 15;
pub const AOM_CICP_MC_ICTCP: aom_matrix_coefficients = 14;
pub const AOM_CICP_MC_CHROMAT_CL: aom_matrix_coefficients = 13;
pub const AOM_CICP_MC_CHROMAT_NCL: aom_matrix_coefficients = 12;
pub const AOM_CICP_MC_SMPTE_2085: aom_matrix_coefficients = 11;
pub const AOM_CICP_MC_BT_2020_CL: aom_matrix_coefficients = 10;
pub const AOM_CICP_MC_BT_2020_NCL: aom_matrix_coefficients = 9;
pub const AOM_CICP_MC_SMPTE_YCGCO: aom_matrix_coefficients = 8;
pub const AOM_CICP_MC_SMPTE_240: aom_matrix_coefficients = 7;
pub const AOM_CICP_MC_BT_601: aom_matrix_coefficients = 6;
pub const AOM_CICP_MC_BT_470_B_G: aom_matrix_coefficients = 5;
pub const AOM_CICP_MC_FCC: aom_matrix_coefficients = 4;
pub const AOM_CICP_MC_RESERVED_3: aom_matrix_coefficients = 3;
pub const AOM_CICP_MC_UNSPECIFIED: aom_matrix_coefficients = 2;
pub const AOM_CICP_MC_BT_709: aom_matrix_coefficients = 1;
pub const AOM_CICP_MC_IDENTITY: aom_matrix_coefficients = 0;
pub type aom_transfer_characteristics_t = aom_transfer_characteristics;
pub type aom_transfer_characteristics = core::ffi::c_uint;
pub const AOM_CICP_TC_RESERVED_19: aom_transfer_characteristics = 19;
pub const AOM_CICP_TC_HLG: aom_transfer_characteristics = 18;
pub const AOM_CICP_TC_SMPTE_428: aom_transfer_characteristics = 17;
pub const AOM_CICP_TC_SMPTE_2084: aom_transfer_characteristics = 16;
pub const AOM_CICP_TC_BT_2020_12_BIT: aom_transfer_characteristics = 15;
pub const AOM_CICP_TC_BT_2020_10_BIT: aom_transfer_characteristics = 14;
pub const AOM_CICP_TC_SRGB: aom_transfer_characteristics = 13;
pub const AOM_CICP_TC_BT_1361: aom_transfer_characteristics = 12;
pub const AOM_CICP_TC_IEC_61966: aom_transfer_characteristics = 11;
pub const AOM_CICP_TC_LOG_100_SQRT10: aom_transfer_characteristics = 10;
pub const AOM_CICP_TC_LOG_100: aom_transfer_characteristics = 9;
pub const AOM_CICP_TC_LINEAR: aom_transfer_characteristics = 8;
pub const AOM_CICP_TC_SMPTE_240: aom_transfer_characteristics = 7;
pub const AOM_CICP_TC_BT_601: aom_transfer_characteristics = 6;
pub const AOM_CICP_TC_BT_470_B_G: aom_transfer_characteristics = 5;
pub const AOM_CICP_TC_BT_470_M: aom_transfer_characteristics = 4;
pub const AOM_CICP_TC_RESERVED_3: aom_transfer_characteristics = 3;
pub const AOM_CICP_TC_UNSPECIFIED: aom_transfer_characteristics = 2;
pub const AOM_CICP_TC_BT_709: aom_transfer_characteristics = 1;
pub const AOM_CICP_TC_RESERVED_0: aom_transfer_characteristics = 0;
pub type aom_color_primaries_t = aom_color_primaries;
pub type aom_color_primaries = core::ffi::c_uint;
pub const AOM_CICP_CP_RESERVED_23: aom_color_primaries = 23;
pub const AOM_CICP_CP_EBU_3213: aom_color_primaries = 22;
pub const AOM_CICP_CP_RESERVED_13: aom_color_primaries = 13;
pub const AOM_CICP_CP_SMPTE_432: aom_color_primaries = 12;
pub const AOM_CICP_CP_SMPTE_431: aom_color_primaries = 11;
pub const AOM_CICP_CP_XYZ: aom_color_primaries = 10;
pub const AOM_CICP_CP_BT_2020: aom_color_primaries = 9;
pub const AOM_CICP_CP_GENERIC_FILM: aom_color_primaries = 8;
pub const AOM_CICP_CP_SMPTE_240: aom_color_primaries = 7;
pub const AOM_CICP_CP_BT_601: aom_color_primaries = 6;
pub const AOM_CICP_CP_BT_470_B_G: aom_color_primaries = 5;
pub const AOM_CICP_CP_BT_470_M: aom_color_primaries = 4;
pub const AOM_CICP_CP_RESERVED_3: aom_color_primaries = 3;
pub const AOM_CICP_CP_UNSPECIFIED: aom_color_primaries = 2;
pub const AOM_CICP_CP_BT_709: aom_color_primaries = 1;
pub const AOM_CICP_CP_RESERVED_0: aom_color_primaries = 0;
pub type aom_codec_iter_t = *const core::ffi::c_void;
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
pub type aom_codec_frame_flags_t = uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
    pub buf: *mut core::ffi::c_void,
    pub sz: size_t,
    pub pts: aom_codec_pts_t,
    pub duration: core::ffi::c_ulong,
    pub flags: aom_codec_frame_flags_t,
    pub partition_id: core::ffi::c_int,
    pub vis_frame_size: size_t,
}
pub type aom_codec_pts_t = int64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_4 {
    pub frame: C2RustUnnamed_3,
    pub twopass_stats: aom_fixed_buf_t,
    pub firstpass_mb_stats: aom_fixed_buf_t,
    pub psnr: aom_psnr_pkt,
    pub raw: aom_fixed_buf_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aom_psnr_pkt {
    pub samples: [core::ffi::c_uint; 4],
    pub sse: [uint64_t; 4],
    pub psnr: [core::ffi::c_double; 4],
    pub samples_hbd: [core::ffi::c_uint; 4],
    pub sse_hbd: [uint64_t; 4],
    pub psnr_hbd: [core::ffi::c_double; 4],
}
pub type aom_codec_cx_pkt_t = aom_codec_cx_pkt;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aom_codec_cx_pkt {
    pub kind: aom_codec_cx_pkt_kind,
    pub data: C2RustUnnamed_4,
}
pub type aom_codec_cx_pkt_kind = core::ffi::c_uint;
pub const AOM_CODEC_CUSTOM_PKT: aom_codec_cx_pkt_kind = 256;
pub const AOM_CODEC_PSNR_PKT: aom_codec_cx_pkt_kind = 3;
pub const AOM_CODEC_FPMB_STATS_PKT: aom_codec_cx_pkt_kind = 2;
pub const AOM_CODEC_STATS_PKT: aom_codec_cx_pkt_kind = 1;
pub const AOM_CODEC_CX_FRAME_PKT: aom_codec_cx_pkt_kind = 0;
pub type aom_enc_frame_flags_t = core::ffi::c_long;
pub const AV1E_SET_ENABLE_RESTORATION: aome_enc_control_id = 59;
pub type aom_scaling_mode_t = aom_scaling_mode;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aom_scaling_mode {
    pub h_scaling_mode: AOM_SCALING_MODE,
    pub v_scaling_mode: AOM_SCALING_MODE,
}
pub type AOM_SCALING_MODE = aom_scaling_mode_1d;
pub type aom_scaling_mode_1d = core::ffi::c_uint;
pub const AOME_ONETHREE: aom_scaling_mode_1d = 8;
pub const AOME_TWOTHREE: aom_scaling_mode_1d = 7;
pub const AOME_ONETWO: aom_scaling_mode_1d = 6;
pub const AOME_ONEEIGHT: aom_scaling_mode_1d = 5;
pub const AOME_ONEFOUR: aom_scaling_mode_1d = 4;
pub const AOME_THREEFOUR: aom_scaling_mode_1d = 3;
pub const AOME_THREEFIVE: aom_scaling_mode_1d = 2;
pub const AOME_FOURFIVE: aom_scaling_mode_1d = 1;
pub const AOME_NORMAL: aom_scaling_mode_1d = 0;
pub const AOME_SET_SCALEMODE: aome_enc_control_id = 11;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aomScalingModeMapList {
    pub avifMode: avifFraction,
    pub aomMode: AOM_SCALING_MODE,
}
pub const AOME_SET_SPATIAL_LAYER_ID: aome_enc_control_id = 12;
pub const AV1E_SET_LOSSLESS: aome_enc_control_id = 31;
pub const AOME_SET_CQ_LEVEL: aome_enc_control_id = 25;
pub const AV1E_SET_TILE_COLUMNS: aome_enc_control_id = 33;
pub const AV1E_SET_TILE_ROWS: aome_enc_control_id = 34;
pub type aom_codec_enc_cfg_t = aom_codec_enc_cfg;
pub type aom_tune_metric = core::ffi::c_uint;
pub const AOM_TUNE_SSIMULACRA2: aom_tune_metric = 11;
pub const AOM_TUNE_IQ: aom_tune_metric = 10;
pub const AOM_TUNE_VMAF_SALIENCY_MAP: aom_tune_metric = 9;
pub const AOM_TUNE_BUTTERAUGLI: aom_tune_metric = 8;
pub const AOM_TUNE_VMAF_NEG_MAX_GAIN: aom_tune_metric = 7;
pub const AOM_TUNE_VMAF_MAX_GAIN: aom_tune_metric = 6;
pub const AOM_TUNE_VMAF_WITHOUT_PREPROCESSING: aom_tune_metric = 5;
pub const AOM_TUNE_VMAF_WITH_PREPROCESSING: aom_tune_metric = 4;
pub const AOM_TUNE_SSIM: aom_tune_metric = 1;
pub const AOM_TUNE_PSNR: aom_tune_metric = 0;
pub const AOME_SET_TUNING: aome_enc_control_id = 24;
pub const AV1E_SET_SKIP_POSTPROC_FILTERING: aome_enc_control_id = 157;
pub const AV1E_SET_COLOR_RANGE: aome_enc_control_id = 52;
pub const AV1E_SET_CHROMA_SAMPLE_POSITION: aome_enc_control_id = 48;
pub const AV1E_SET_MATRIX_COEFFICIENTS: aome_enc_control_id = 47;
pub const AV1E_SET_TRANSFER_CHARACTERISTICS: aome_enc_control_id = 46;
pub const AV1E_SET_COLOR_PRIMARIES: aome_enc_control_id = 45;
pub const AOME_SET_CPUUSED: aome_enc_control_id = 13;
pub const AOME_SET_NUMBER_SPATIAL_LAYERS: aome_enc_control_id = 27;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aomOptionEnumList {
    pub name: *const core::ffi::c_char,
    pub val: core::ffi::c_int,
}
pub const AV1D_SET_OPERATING_POINT: aom_dec_control_id = 279;
pub const AV1D_SET_OUTPUT_ALL_LAYERS: aom_dec_control_id = 280;
pub const AOMD_SET_FRAME_SIZE_LIMIT: aom_dec_control_id = 296;
pub type aom_codec_dec_cfg_t = aom_codec_dec_cfg;
pub type aome_enc_control_id = core::ffi::c_uint;
pub const AV1E_SET_MODE_REF_DELTA_ENABLED: aome_enc_control_id = 176;
pub const AOME_SET_VALIDATE_HBD_INPUT: aome_enc_control_id = 175;
pub const AV1E_GET_GOP_INFO: aome_enc_control_id = 174;
pub const AV1E_SET_EXTERNAL_RATE_CONTROL: aome_enc_control_id = 173;
pub const AV1E_SET_ENABLE_ADAPTIVE_SHARPNESS: aome_enc_control_id = 172;
pub const AV1E_SET_SCREEN_CONTENT_DETECTION_MODE: aome_enc_control_id = 171;
pub const AV1E_SET_ENABLE_LOW_COMPLEXITY_DECODE: aome_enc_control_id = 170;
pub const AV1E_SET_MAX_CONSEC_FRAME_DROP_MS_CBR: aome_enc_control_id = 169;
pub const AV1E_SET_POSTENCODE_DROP_RTC: aome_enc_control_id = 168;
pub const AV1E_GET_HIGH_MOTION_CONTENT_SCREEN_RTC: aome_enc_control_id = 167;
pub const AV1E_SET_AUTO_TILES: aome_enc_control_id = 166;
pub const AV1E_SET_SVC_FRAME_DROP_MODE: aome_enc_control_id = 165;
pub const AV1E_SET_MAX_CONSEC_FRAME_DROP_CBR: aome_enc_control_id = 164;
pub const AV1E_SET_BITRATE_ONE_PASS_CBR: aome_enc_control_id = 163;
pub const AV1E_GET_LUMA_CDEF_STRENGTH: aome_enc_control_id = 162;
pub const AV1E_SET_RATE_DISTRIBUTION_INFO: aome_enc_control_id = 161;
pub const AV1E_ENABLE_RATE_GUIDE_DELTAQ: aome_enc_control_id = 160;
pub const AV1E_SET_QUANTIZER_ONE_PASS: aome_enc_control_id = 159;
pub const AV1E_ENABLE_SB_QP_SWEEP: aome_enc_control_id = 158;
pub const AV1E_GET_NUM_OPERATING_POINTS: aome_enc_control_id = 156;
pub const AV1E_GET_TARGET_SEQ_LEVEL_IDX: aome_enc_control_id = 155;
pub const AV1E_SET_FP_MT_UNIT_TEST: aome_enc_control_id = 154;
pub const AV1E_SET_FP_MT: aome_enc_control_id = 153;
pub const AV1E_SET_RTC_EXTERNAL_RC: aome_enc_control_id = 152;
pub const AV1E_SET_AUTO_INTRA_TOOLS_OFF: aome_enc_control_id = 151;
pub const AOME_GET_LOOPFILTER_LEVEL: aome_enc_control_id = 150;
pub const AV1E_SET_LOOPFILTER_CONTROL: aome_enc_control_id = 149;
pub const AV1E_SET_DELTAQ_STRENGTH: aome_enc_control_id = 148;
pub const AV1E_SET_SVC_REF_FRAME_COMP_PRED: aome_enc_control_id = 147;
pub const AV1E_SET_ENABLE_TX_SIZE_SEARCH: aome_enc_control_id = 146;
pub const AV1E_SET_ENABLE_DIRECTIONAL_INTRA: aome_enc_control_id = 145;
pub const AV1E_SET_EXTERNAL_PARTITION: aome_enc_control_id = 144;
pub const AV1E_SET_PARTITION_INFO_PATH: aome_enc_control_id = 143;
pub const AV1E_SET_DV_COST_UPD_FREQ: aome_enc_control_id = 142;
pub const AV1E_SET_ENABLE_DIAGONAL_INTRA: aome_enc_control_id = 141;
pub const AV1E_SET_ENABLE_DNL_DENOISING: aome_enc_control_id = 140;
pub const AV1E_GET_BASELINE_GF_INTERVAL: aome_enc_control_id = 139;
pub const AV1E_SET_VBR_CORPUS_COMPLEXITY_LAP: aome_enc_control_id = 138;
pub const AV1E_SET_GF_MIN_PYRAMID_HEIGHT: aome_enc_control_id = 137;
pub const AV1E_ENABLE_SB_MULTIPASS_UNIT_TEST: aome_enc_control_id = 136;
pub const AV1E_ENABLE_EXT_TILE_DEBUG: aome_enc_control_id = 135;
pub const AV1E_SET_VMAF_MODEL_PATH: aome_enc_control_id = 134;
pub const AV1E_SET_SVC_REF_FRAME_CONFIG: aome_enc_control_id = 133;
pub const AV1E_SET_SVC_PARAMS: aome_enc_control_id = 132;
pub const AV1E_SET_SVC_LAYER_ID: aome_enc_control_id = 131;
pub const AV1E_SET_MIN_CR: aome_enc_control_id = 130;
pub const AV1E_SET_TIER_MASK: aome_enc_control_id = 129;
pub const AV1E_SET_MV_COST_UPD_FREQ: aome_enc_control_id = 128;
pub const AV1E_SET_MODE_COST_UPD_FREQ: aome_enc_control_id = 127;
pub const AV1E_SET_COEFF_COST_UPD_FREQ: aome_enc_control_id = 126;
pub const AV1E_SET_REDUCED_REFERENCE_SET: aome_enc_control_id = 125;
pub const AV1E_SET_MAX_REFERENCE_FRAMES: aome_enc_control_id = 124;
pub const AV1E_SET_GF_MAX_PYRAMID_HEIGHT: aome_enc_control_id = 123;
pub const AV1E_SET_QUANT_B_ADAPT: aome_enc_control_id = 122;
pub const AV1E_SET_INTRA_DEFAULT_TX_ONLY: aome_enc_control_id = 121;
pub const AV1E_SET_INTER_DCT_ONLY: aome_enc_control_id = 120;
pub const AV1E_SET_INTRA_DCT_ONLY: aome_enc_control_id = 119;
pub const AV1E_SET_REDUCED_TX_TYPE_SET: aome_enc_control_id = 118;
pub const AV1E_SET_CHROMA_SUBSAMPLING_Y: aome_enc_control_id = 117;
pub const AV1E_SET_CHROMA_SUBSAMPLING_X: aome_enc_control_id = 116;
pub const AV1E_SET_DENOISE_BLOCK_SIZE: aome_enc_control_id = 115;
pub const AV1E_SET_DENOISE_NOISE_LEVEL: aome_enc_control_id = 114;
pub const AV1E_SET_FILM_GRAIN_TABLE: aome_enc_control_id = 113;
pub const AV1E_SET_FILM_GRAIN_TEST_VECTOR: aome_enc_control_id = 112;
pub const AV1E_SET_TIMING_INFO_TYPE: aome_enc_control_id = 111;
pub const AV1E_ENABLE_MOTION_VECTOR_UNIT_TEST: aome_enc_control_id = 110;
pub const AV1E_SET_SINGLE_TILE_DECODING: aome_enc_control_id = 109;
pub const AV1E_SET_DELTALF_MODE: aome_enc_control_id = 108;
pub const AV1E_SET_DELTAQ_MODE: aome_enc_control_id = 107;
pub const AV1E_SET_ENABLE_ANGLE_DELTA: aome_enc_control_id = 106;
pub const AV1E_SET_ENABLE_INTRABC: aome_enc_control_id = 105;
pub const AV1E_SET_ENABLE_PALETTE: aome_enc_control_id = 104;
pub const AV1E_SET_ENABLE_OVERLAY: aome_enc_control_id = 103;
pub const AV1E_SET_ENABLE_SUPERRES: aome_enc_control_id = 102;
pub const AV1E_SET_ENABLE_CFL_INTRA: aome_enc_control_id = 101;
pub const AV1E_SET_ENABLE_PAETH_INTRA: aome_enc_control_id = 100;
pub const AV1E_SET_ENABLE_SMOOTH_INTRA: aome_enc_control_id = 99;
pub const AV1E_SET_ENABLE_FILTER_INTRA: aome_enc_control_id = 98;
pub const AV1E_SET_ALLOW_WARPED_MOTION: aome_enc_control_id = 97;
pub const AV1E_SET_ENABLE_WARPED_MOTION: aome_enc_control_id = 96;
pub const AV1E_SET_ENABLE_GLOBAL_MOTION: aome_enc_control_id = 95;
pub const AV1E_SET_ENABLE_INTERINTRA_WEDGE: aome_enc_control_id = 94;
pub const AV1E_SET_ENABLE_INTERINTER_WEDGE: aome_enc_control_id = 93;
pub const AV1E_SET_ENABLE_DIFF_WTD_COMP: aome_enc_control_id = 92;
pub const AV1E_SET_ENABLE_SMOOTH_INTERINTRA: aome_enc_control_id = 91;
pub const AV1E_SET_ENABLE_INTERINTRA_COMP: aome_enc_control_id = 90;
pub const AV1E_SET_ENABLE_ONESIDED_COMP: aome_enc_control_id = 89;
pub const AV1E_SET_ENABLE_MASKED_COMP: aome_enc_control_id = 88;
pub const AV1E_SET_ENABLE_CHROMA_DELTAQ: aome_enc_control_id = 87;
pub const AV1E_SET_ENABLE_DUAL_FILTER: aome_enc_control_id = 86;
pub const AV1E_SET_ALLOW_REF_FRAME_MVS: aome_enc_control_id = 85;
pub const AV1E_SET_ENABLE_REF_FRAME_MVS: aome_enc_control_id = 84;
pub const AV1E_SET_ENABLE_DIST_WTD_COMP: aome_enc_control_id = 83;
pub const AV1E_SET_ENABLE_RECT_TX: aome_enc_control_id = 82;
pub const AV1E_SET_ENABLE_FLIP_IDTX: aome_enc_control_id = 81;
pub const AV1E_SET_ENABLE_TX64: aome_enc_control_id = 80;
pub const AV1E_SET_ENABLE_ORDER_HINT: aome_enc_control_id = 79;
pub const AV1E_SET_ENABLE_INTRA_EDGE_FILTER: aome_enc_control_id = 78;
pub const AV1E_SET_MAX_PARTITION_SIZE: aome_enc_control_id = 77;
pub const AV1E_SET_MIN_PARTITION_SIZE: aome_enc_control_id = 76;
pub const AV1E_SET_ENABLE_1TO4_PARTITIONS: aome_enc_control_id = 75;
pub const AV1E_SET_ENABLE_AB_PARTITIONS: aome_enc_control_id = 74;
pub const AV1E_SET_ENABLE_RECT_PARTITIONS: aome_enc_control_id = 73;
pub const AV1E_SET_MTU: aome_enc_control_id = 71;
pub const AV1E_SET_NUM_TG: aome_enc_control_id = 70;
pub const AV1E_SET_QM_V: aome_enc_control_id = 68;
pub const AV1E_SET_QM_U: aome_enc_control_id = 67;
pub const AV1E_SET_QM_Y: aome_enc_control_id = 66;
pub const AV1E_SET_QM_MAX: aome_enc_control_id = 65;
pub const AV1E_SET_QM_MIN: aome_enc_control_id = 64;
pub const AV1E_SET_ENABLE_QM: aome_enc_control_id = 63;
pub const AV1E_SET_DISABLE_TRELLIS_QUANT: aome_enc_control_id = 62;
pub const AV1E_SET_ENABLE_OBMC: aome_enc_control_id = 61;
pub const AV1E_SET_FORCE_VIDEO_MODE: aome_enc_control_id = 60;
pub const AV1E_SET_ENABLE_CDEF: aome_enc_control_id = 58;
pub const AOME_SET_ENABLEAUTOBWDREF: aome_enc_control_id = 57;
pub const AV1E_SET_SUPERBLOCK_SIZE: aome_enc_control_id = 56;
pub const AV1E_GET_SEQ_LEVEL_IDX: aome_enc_control_id = 55;
pub const AV1E_SET_TARGET_SEQ_LEVEL_IDX: aome_enc_control_id = 54;
pub const AV1E_SET_RENDER_SIZE: aome_enc_control_id = 53;
pub const AV1E_GET_ACTIVEMAP: aome_enc_control_id = 51;
pub const AV1E_SET_MAX_GF_INTERVAL: aome_enc_control_id = 50;
pub const AV1E_SET_MIN_GF_INTERVAL: aome_enc_control_id = 49;
pub const AV1E_SET_CDF_UPDATE_MODE: aome_enc_control_id = 44;
pub const AV1E_SET_TUNE_CONTENT: aome_enc_control_id = 43;
pub const AV1E_SET_NOISE_SENSITIVITY: aome_enc_control_id = 42;
pub const AV1E_SET_FRAME_PERIODIC_BOOST: aome_enc_control_id = 41;
pub const AV1E_SET_AQ_MODE: aome_enc_control_id = 40;
pub const AV1E_SET_S_FRAME_MODE: aome_enc_control_id = 39;
pub const AV1E_SET_ERROR_RESILIENT_MODE: aome_enc_control_id = 38;
pub const AV1E_SET_FRAME_PARALLEL_DECODING: aome_enc_control_id = 37;
pub const AV1E_SET_ENABLE_KEYFRAME_FILTERING: aome_enc_control_id = 36;
pub const AV1E_SET_ENABLE_TPL_MODEL: aome_enc_control_id = 35;
pub const AV1E_SET_ROW_MT: aome_enc_control_id = 32;
pub const AV1E_SET_GF_CBR_BOOST_PCT: aome_enc_control_id = 29;
pub const AV1E_SET_MAX_INTER_BITRATE_PCT: aome_enc_control_id = 28;
pub const AOME_SET_MAX_INTRA_BITRATE_PCT: aome_enc_control_id = 26;
pub const AOME_SET_ARNR_STRENGTH: aome_enc_control_id = 22;
pub const AOME_SET_ARNR_MAXFRAMES: aome_enc_control_id = 21;
pub const AOME_GET_LAST_QUANTIZER_64: aome_enc_control_id = 20;
pub const AOME_GET_LAST_QUANTIZER: aome_enc_control_id = 19;
pub const AOME_SET_STATIC_THRESHOLD: aome_enc_control_id = 17;
pub const AOME_SET_SHARPNESS: aome_enc_control_id = 16;
pub const AOME_SET_ENABLEAUTOALTREF: aome_enc_control_id = 14;
pub const AOME_SET_ACTIVEMAP: aome_enc_control_id = 9;
pub const AOME_SET_ROI_MAP: aome_enc_control_id = 8;
pub const AOME_USE_REFERENCE: aome_enc_control_id = 7;
pub type aom_dec_control_id = core::ffi::c_uint;
pub const AV1D_GET_MI_INFO: aom_dec_control_id = 295;
pub const AOMD_GET_ORDER_HINT: aom_dec_control_id = 294;
pub const AOMD_GET_BASE_Q_IDX: aom_dec_control_id = 293;
pub const AOMD_GET_SHOW_FRAME_FLAG: aom_dec_control_id = 292;
pub const AOMD_GET_S_FRAME_INFO: aom_dec_control_id = 291;
pub const AOMD_GET_SHOW_EXISTING_FRAME_FLAG: aom_dec_control_id = 290;
pub const AOMD_GET_SB_SIZE: aom_dec_control_id = 289;
pub const AOMD_GET_STILL_PICTURE: aom_dec_control_id = 288;
pub const AOMD_GET_SCREEN_CONTENT_TOOLS_INFO: aom_dec_control_id = 287;
pub const AOMD_GET_TILE_INFO: aom_dec_control_id = 286;
pub const AOMD_GET_ALTREF_PRESENT: aom_dec_control_id = 285;
pub const AOMD_GET_FRAME_FLAGS: aom_dec_control_id = 284;
pub const AOMD_GET_FWD_KF_PRESENT: aom_dec_control_id = 283;
pub const AV1D_SET_SKIP_FILM_GRAIN: aom_dec_control_id = 282;
pub const AV1_SET_INSPECTION_CALLBACK: aom_dec_control_id = 281;
pub const AV1D_SET_IS_ANNEXB: aom_dec_control_id = 278;
pub const AV1D_SET_ROW_MT: aom_dec_control_id = 277;
pub const AV1D_EXT_TILE_DEBUG: aom_dec_control_id = 276;
pub const AV1D_SET_EXT_REF_PTR: aom_dec_control_id = 275;
pub const AV1D_GET_TILE_DATA: aom_dec_control_id = 274;
pub const AV1D_GET_FRAME_HEADER_INFO: aom_dec_control_id = 273;
pub const AV1_SET_TILE_MODE: aom_dec_control_id = 272;
pub const AV1_SET_DECODE_TILE_COL: aom_dec_control_id = 271;
pub const AV1_SET_DECODE_TILE_ROW: aom_dec_control_id = 270;
pub const AOMD_GET_LAST_QUANTIZER: aom_dec_control_id = 269;
pub const AV1_GET_ACCOUNTING: aom_dec_control_id = 268;
pub const AV1_SET_SKIP_LOOP_FILTER: aom_dec_control_id = 267;
pub const AV1_INVERT_TILE_DECODE_ORDER: aom_dec_control_id = 266;
pub const AV1_SET_BYTE_ALIGNMENT: aom_dec_control_id = 265;
pub const AV1D_GET_TILE_COUNT: aom_dec_control_id = 264;
pub const AV1D_GET_TILE_SIZE: aom_dec_control_id = 263;
pub const AV1D_GET_IMG_FORMAT: aom_dec_control_id = 262;
pub const AV1D_GET_BIT_DEPTH: aom_dec_control_id = 261;
pub const AV1D_GET_DISPLAY_SIZE: aom_dec_control_id = 260;
pub const AV1D_GET_FRAME_SIZE: aom_dec_control_id = 259;
pub const AOMD_GET_LAST_REF_USED: aom_dec_control_id = 258;
pub const AOMD_GET_FRAME_CORRUPTED: aom_dec_control_id = 257;
pub const AOMD_GET_LAST_REF_UPDATES: aom_dec_control_id = 256;
pub const NULL: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
pub const AVIF_TRUE: core::ffi::c_int = 1 as core::ffi::c_int;
pub const AVIF_FALSE: core::ffi::c_int = 0 as core::ffi::c_int;
pub const AVIF_QUALITY_LOSSLESS: core::ffi::c_int = 100 as core::ffi::c_int;
pub const AVIF_QUANTIZER_LOSSLESS: core::ffi::c_int = 0 as core::ffi::c_int;
pub const AVIF_SPEED_DEFAULT: core::ffi::c_int = -(1 as core::ffi::c_int);
pub const AVIF_SPATIAL_ID_UNSET: core::ffi::c_int = 0xff as core::ffi::c_int;
pub const AOM_IMAGE_ABI_VERSION: core::ffi::c_int = 9 as core::ffi::c_int;
pub const AOM_IMG_FMT_HIGHBITDEPTH: core::ffi::c_int = 0x800 as core::ffi::c_int;
pub const AOM_CODEC_ABI_VERSION: core::ffi::c_int = 7 as core::ffi::c_int
    + AOM_IMAGE_ABI_VERSION;
pub const AOM_FRAME_IS_KEY: core::ffi::c_uint = 0x1 as core::ffi::c_uint;
pub const AOM_ENCODER_ABI_VERSION: core::ffi::c_int = 10 as core::ffi::c_int
    + AOM_CODEC_ABI_VERSION + 3 as core::ffi::c_int;
pub const AOM_CODEC_USE_HIGHBITDEPTH: core::ffi::c_int = 0x40000 as core::ffi::c_int;
pub const AOM_EFLAG_FORCE_KF: core::ffi::c_int = (1 as core::ffi::c_int)
    << 0 as core::ffi::c_int;
pub const AOM_USAGE_GOOD_QUALITY: core::ffi::c_uint = 0 as core::ffi::c_uint;
pub const AOM_USAGE_REALTIME: core::ffi::c_uint = 1;
pub const AOM_USAGE_ALL_INTRA: core::ffi::c_uint = 2 as core::ffi::c_uint;
pub const AOM_EFLAG_NO_REF_GF: core::ffi::c_int = (1 as core::ffi::c_int)
    << 19 as core::ffi::c_int;
pub const AOM_EFLAG_NO_REF_ARF: core::ffi::c_int = (1 as core::ffi::c_int)
    << 20 as core::ffi::c_int;
pub const AOM_EFLAG_NO_REF_BWD: core::ffi::c_int = (1 as core::ffi::c_int)
    << 21 as core::ffi::c_int;
pub const AOM_EFLAG_NO_REF_ARF2: core::ffi::c_int = (1 as core::ffi::c_int)
    << 22 as core::ffi::c_int;
pub const AOM_EFLAG_NO_UPD_GF: core::ffi::c_int = (1 as core::ffi::c_int)
    << 24 as core::ffi::c_int;
pub const AOM_EFLAG_NO_UPD_ARF: core::ffi::c_int = (1 as core::ffi::c_int)
    << 25 as core::ffi::c_int;
pub const AOM_DECODER_ABI_VERSION: core::ffi::c_int = 6 as core::ffi::c_int
    + AOM_CODEC_ABI_VERSION;
pub const TWO_LAYER_ALL_INTRA_QUALITY_THRESHOLD: core::ffi::c_int = 10
    as core::ffi::c_int;
unsafe extern "C" fn aomCodecDestroyInternal(mut codec: *mut avifCodec) {
    if (*(*codec).internal).decoderInitialized != 0 {
        aom_codec_destroy(&mut (*(*codec).internal).decoder);
    }
    if (*(*codec).internal).encoderInitialized != 0 {
        aom_codec_destroy(&mut (*(*codec).internal).encoder);
    }
    avifFree((*codec).internal as *mut core::ffi::c_void);
}
unsafe extern "C" fn aomDiagPrintf(
    mut diag: *mut avifDiagnostics,
    mut func: *const core::ffi::c_char,
    mut ctx: *const aom_codec_ctx_t,
) {
    let mut error: *const core::ffi::c_char = aom_codec_error(ctx);
    let mut error_detail: *const core::ffi::c_char = aom_codec_error_detail(ctx);
    avifDiagnosticsPrintf(
        diag,
        b"%s failed: %s: %s\0" as *const u8 as *const core::ffi::c_char,
        func,
        error,
        if !error_detail.is_null() {
            error_detail
        } else {
            b"no error detail\0" as *const u8 as *const core::ffi::c_char
        },
    );
}
unsafe extern "C" fn aomCodecGetNextImage(
    mut codec: *mut avifCodec,
    mut sample: *const avifDecodeSample,
    mut alpha: avifBool,
    mut isLimitedRangeAlpha: *mut avifBool,
    mut image: *mut avifImage,
) -> avifBool {
    'c_8530: {
        if !sample.is_null() {} else {
            __assert_fail(
                b"sample\0" as *const u8 as *const core::ffi::c_char,
                b"/home/runner/work/libavif-rs/libavif-rs/upstream/libavif/src/codec_aom.c\0"
                    as *const u8 as *const core::ffi::c_char,
                109 as core::ffi::c_uint,
                (::core::mem::transmute::<
                    [u8; 111],
                    [core::ffi::c_char; 111],
                >(
                    *b"avifBool aomCodecGetNextImage(struct avifCodec *, const avifDecodeSample *, avifBool, avifBool *, avifImage *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    let decoderInterface: *const aom_codec_iface_t = aom_codec_av1_dx()
        as *const aom_codec_iface_t;
    if (*(*codec).internal).decoderInitialized == 0 {
        let mut cfg: aom_codec_dec_cfg_t = aom_codec_dec_cfg {
            threads: 0,
            w: 0,
            h: 0,
            allow_lowbitdepth: 0,
        };
        memset(
            &mut cfg as *mut aom_codec_dec_cfg_t as *mut core::ffi::c_void,
            0 as core::ffi::c_int,
            ::core::mem::size_of::<aom_codec_dec_cfg_t>() as size_t,
        );
        cfg.threads = (*codec).maxThreads as core::ffi::c_uint;
        cfg.allow_lowbitdepth = 1 as core::ffi::c_uint;
        if aom_codec_dec_init_ver(
            &mut (*(*codec).internal).decoder,
            decoderInterface,
            &mut cfg,
            0 as aom_codec_flags_t,
            AOM_DECODER_ABI_VERSION,
        ) as u64 != 0
        {
            aomDiagPrintf(
                (*codec).diag,
                b"aom_codec_dec_init()\0" as *const u8 as *const core::ffi::c_char,
                &mut (*(*codec).internal).decoder,
            );
            return AVIF_FALSE;
        }
        (*(*codec).internal).decoderInitialized = AVIF_TRUE as avifBool;
        if (*codec).imageSizeLimit != 0 as uint32_t
            && aom_codec_control(
                &mut (*(*codec).internal).decoder as *mut aom_codec_ctx_t,
                AOMD_SET_FRAME_SIZE_LIMIT as core::ffi::c_int,
                (*codec).imageSizeLimit,
            ) as core::ffi::c_uint != 0
        {
            aomDiagPrintf(
                (*codec).diag,
                b"aom_codec_control(AOMD_SET_FRAME_SIZE_LIMIT)\0" as *const u8
                    as *const core::ffi::c_char,
                &mut (*(*codec).internal).decoder,
            );
            return AVIF_FALSE;
        }
        if aom_codec_control(
            &mut (*(*codec).internal).decoder as *mut aom_codec_ctx_t,
            AV1D_SET_OUTPUT_ALL_LAYERS as core::ffi::c_int,
            (*codec).allLayers,
        ) as u64 != 0
        {
            aomDiagPrintf(
                (*codec).diag,
                b"aom_codec_control(AV1D_SET_OUTPUT_ALL_LAYERS)\0" as *const u8
                    as *const core::ffi::c_char,
                &mut (*(*codec).internal).decoder,
            );
            return AVIF_FALSE;
        }
        if aom_codec_control(
            &mut (*(*codec).internal).decoder as *mut aom_codec_ctx_t,
            AV1D_SET_OPERATING_POINT as core::ffi::c_int,
            (*codec).operatingPoint as core::ffi::c_int,
        ) as u64 != 0
        {
            aomDiagPrintf(
                (*codec).diag,
                b"aom_codec_control(AV1D_SET_OPERATING_POINT)\0" as *const u8
                    as *const core::ffi::c_char,
                &mut (*(*codec).internal).decoder,
            );
            return AVIF_FALSE;
        }
        (*(*codec).internal).iter = 0 as aom_codec_iter_t;
    }
    let mut nextFrame: *mut aom_image_t = 0 as *mut aom_image_t;
    let mut spatialID: uint8_t = AVIF_SPATIAL_ID_UNSET as uint8_t;
    loop {
        nextFrame = aom_codec_get_frame(
            &mut (*(*codec).internal).decoder,
            &mut (*(*codec).internal).iter,
        );
        if !nextFrame.is_null() {
            if !(spatialID as core::ffi::c_int != AVIF_SPATIAL_ID_UNSET) {
                break;
            }
            if spatialID as core::ffi::c_int == (*nextFrame).spatial_id {
                break;
            }
        } else {
            if sample.is_null() {
                break;
            }
            (*(*codec).internal).iter = 0 as aom_codec_iter_t;
            if aom_codec_decode(
                &mut (*(*codec).internal).decoder,
                (*sample).data.data,
                (*sample).data.size,
                NULL,
            ) as u64 != 0
            {
                aomDiagPrintf(
                    (*codec).diag,
                    b"aom_codec_decode()\0" as *const u8 as *const core::ffi::c_char,
                    &mut (*(*codec).internal).decoder,
                );
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
            261 | 2309 => {
                yuvFormat = AVIF_PIXEL_FORMAT_YUV422;
            }
            262 | 2310 => {
                yuvFormat = AVIF_PIXEL_FORMAT_YUV444;
            }
            0 | 263 | 769 | 2817 | _ => return AVIF_FALSE,
        }
        if (*(*(*codec).internal).image).monochrome != 0 {
            yuvFormat = AVIF_PIXEL_FORMAT_YUV400;
        }
        (*image).width = (*(*(*codec).internal).image).d_w as uint32_t;
        (*image).height = (*(*(*codec).internal).image).d_h as uint32_t;
        (*image).depth = (*(*(*codec).internal).image).bit_depth as uint32_t;
        (*image).yuvFormat = yuvFormat;
        (*image).yuvRange = (if (*(*(*codec).internal).image).range as core::ffi::c_uint
            == AOM_CR_STUDIO_RANGE as core::ffi::c_int as core::ffi::c_uint
        {
            AVIF_RANGE_LIMITED as core::ffi::c_int
        } else {
            AVIF_RANGE_FULL as core::ffi::c_int
        }) as avifRange;
        (*image).yuvChromaSamplePosition = (*(*(*codec).internal).image).csp
            as avifChromaSamplePosition;
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
        let mut yuvPlane: core::ffi::c_int = 0 as core::ffi::c_int;
        while yuvPlane < yuvPlaneCount {
            (*image).yuvPlanes[yuvPlane as usize] = (*(*(*codec).internal).image)
                .planes[yuvPlane as usize] as *mut uint8_t;
            (*image).yuvRowBytes[yuvPlane as usize] = (*(*(*codec).internal).image)
                .stride[yuvPlane as usize] as uint32_t;
            yuvPlane += 1;
        }
        (*image).imageOwnsYUVPlanes = AVIF_FALSE as avifBool;
    } else {
        (*image).width = (*(*(*codec).internal).image).d_w as uint32_t;
        (*image).height = (*(*(*codec).internal).image).d_h as uint32_t;
        (*image).depth = (*(*(*codec).internal).image).bit_depth as uint32_t;
        avifImageFreePlanes(image, AVIF_PLANES_A as core::ffi::c_int as avifPlanesFlags);
        (*image).alphaPlane = (*(*(*codec).internal).image)
            .planes[0 as core::ffi::c_int as usize] as *mut uint8_t;
        (*image).alphaRowBytes = (*(*(*codec).internal).image)
            .stride[0 as core::ffi::c_int as usize] as uint32_t;
        *isLimitedRangeAlpha = ((*(*(*codec).internal).image).range as core::ffi::c_uint
            == AOM_CR_STUDIO_RANGE as core::ffi::c_int as core::ffi::c_uint)
            as core::ffi::c_int as avifBool;
        (*image).imageOwnsAlphaPlane = AVIF_FALSE as avifBool;
    }
    return AVIF_TRUE;
}
unsafe extern "C" fn avifImageCalcAOMFmt(
    mut image: *const avifImage,
    mut alpha: avifBool,
) -> aom_img_fmt_t {
    let mut fmt: aom_img_fmt_t = AOM_IMG_FMT_NONE;
    if alpha != 0 {
        fmt = AOM_IMG_FMT_I420;
    } else {
        match (*image).yuvFormat as core::ffi::c_uint {
            1 => {
                fmt = AOM_IMG_FMT_I444;
            }
            2 => {
                fmt = AOM_IMG_FMT_I422;
            }
            3 | 4 => {
                fmt = AOM_IMG_FMT_I420;
            }
            0 | 5 | _ => return AOM_IMG_FMT_NONE,
        }
    }
    if (*image).depth > 8 as uint32_t {
        fmt = ::core::mem::transmute::<
            core::ffi::c_uint,
            aom_img_fmt_t,
        >(fmt as core::ffi::c_uint | AOM_IMG_FMT_HIGHBITDEPTH as core::ffi::c_uint);
    }
    return fmt;
}
unsafe extern "C" fn aomOptionParseEnum(
    mut str: *const core::ffi::c_char,
    mut enums: *const aomOptionEnumList,
    mut val: *mut core::ffi::c_int,
) -> avifBool {
    let mut listptr: *const aomOptionEnumList = 0 as *const aomOptionEnumList;
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
static mut endUsageEnum: [aomOptionEnumList; 5] = [
    {
        let mut init = aomOptionEnumList {
            name: b"vbr\0" as *const u8 as *const core::ffi::c_char,
            val: AOM_VBR as core::ffi::c_int,
        };
        init
    },
    {
        let mut init = aomOptionEnumList {
            name: b"cbr\0" as *const u8 as *const core::ffi::c_char,
            val: AOM_CBR as core::ffi::c_int,
        };
        init
    },
    {
        let mut init = aomOptionEnumList {
            name: b"cq\0" as *const u8 as *const core::ffi::c_char,
            val: AOM_CQ as core::ffi::c_int,
        };
        init
    },
    {
        let mut init = aomOptionEnumList {
            name: b"q\0" as *const u8 as *const core::ffi::c_char,
            val: AOM_Q as core::ffi::c_int,
        };
        init
    },
    {
        let mut init = aomOptionEnumList {
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
static mut tuneIqEnum: [aomOptionEnumList; 2] = [
    {
        let mut init = aomOptionEnumList {
            name: b"iq\0" as *const u8 as *const core::ffi::c_char,
            val: AOM_TUNE_IQ as core::ffi::c_int,
        };
        init
    },
    {
        let mut init = aomOptionEnumList {
            name: 0 as *const core::ffi::c_char,
            val: 0 as core::ffi::c_int,
        };
        init
    },
];
unsafe extern "C" fn avifAOMOptionsContainExplicitTuning(
    mut codec: *const avifCodec,
    mut alpha: avifBool,
    mut useTuneIq: *mut avifBool,
) -> avifBool {
    *useTuneIq = AVIF_FALSE as avifBool;
    let mut isAnyTuneDefined: avifBool = AVIF_FALSE;
    let mut i: uint32_t = 0 as uint32_t;
    while i < (*(*codec).csOptions).count {
        let mut entry: *const avifCodecSpecificOption = &mut *((*(*codec).csOptions)
            .entries)
            .offset(i as isize) as *mut avifCodecSpecificOption;
        if avifKeyEqualsName(
            (*entry).key,
            b"tune\0" as *const u8 as *const core::ffi::c_char,
            alpha,
        ) != 0
        {
            isAnyTuneDefined = AVIF_TRUE as avifBool;
            let mut val: core::ffi::c_int = 0;
            if aomOptionParseEnum((*entry).value, core::ptr::addr_of!(tuneIqEnum).cast::<aomOptionEnumList>(), &mut val) != 0 {
                'c_7451: {
                    if val == AOM_TUNE_IQ as core::ffi::c_int {} else {
                        __assert_fail(
                            b"val == AOM_TUNE_IQ\0" as *const u8
                                as *const core::ffi::c_char,
                            b"/home/runner/work/libavif-rs/libavif-rs/upstream/libavif/src/codec_aom.c\0"
                                as *const u8 as *const core::ffi::c_char,
                            416 as core::ffi::c_uint,
                            (::core::mem::transmute::<
                                [u8; 86],
                                [core::ffi::c_char; 86],
                            >(
                                *b"avifBool avifAOMOptionsContainExplicitTuning(const avifCodec *, avifBool, avifBool *)\0",
                            ))
                                .as_ptr(),
                        );
                    }
                };
                *useTuneIq = AVIF_TRUE as avifBool;
            } else {
                *useTuneIq = AVIF_FALSE as avifBool;
            }
        }
        i = i.wrapping_add(1);
    }
    return isAnyTuneDefined;
}
unsafe extern "C" fn avifProcessAOMOptionsPreInit(
    mut codec: *mut avifCodec,
    mut alpha: avifBool,
    mut cfg: *mut aom_codec_enc_cfg,
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
            if aomOptionParseEnum((*entry).value, core::ptr::addr_of!(endUsageEnum).cast::<aomOptionEnumList>(), &mut val) == 0 {
                avifDiagnosticsPrintf(
                    (*codec).diag,
                    b"Invalid value for end-usage: %s\0" as *const u8
                        as *const core::ffi::c_char,
                    (*entry).value,
                );
                return AVIF_FALSE;
            }
            (*cfg).rc_end_usage = val as aom_rc_mode;
        }
        i = i.wrapping_add(1);
    }
    return AVIF_TRUE;
}
unsafe extern "C" fn avifProcessAOMOptionsPostInit(
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
                if aom_codec_set_option(
                    &mut (*(*codec).internal).encoder,
                    key,
                    (*entry).value,
                ) as core::ffi::c_uint
                    != AOM_CODEC_OK as core::ffi::c_int as core::ffi::c_uint
                {
                    let mut error_detail: *const core::ffi::c_char = aom_codec_error_detail(
                        &mut (*(*codec).internal).encoder,
                    );
                    avifDiagnosticsPrintf(
                        (*codec).diag,
                        b"aom_codec_set_option(\"%s\", \"%s\") failed: %s: %s\0"
                            as *const u8 as *const core::ffi::c_char,
                        key,
                        (*entry).value,
                        aom_codec_error(&mut (*(*codec).internal).encoder),
                        if !error_detail.is_null() {
                            error_detail
                        } else {
                            b"no error detail\0" as *const u8 as *const core::ffi::c_char
                        },
                    );
                    return AVIF_FALSE;
                }
            }
        }
        i = i.wrapping_add(1);
    }
    return AVIF_TRUE;
}
static mut scalingModeMap: [aomScalingModeMapList; 7] = [
    {
        let mut init = aomScalingModeMapList {
            avifMode: {
                let mut init = avifFraction {
                    n: 1 as int32_t,
                    d: 1 as int32_t,
                };
                init
            },
            aomMode: AOME_NORMAL,
        };
        init
    },
    {
        let mut init = aomScalingModeMapList {
            avifMode: {
                let mut init = avifFraction {
                    n: 1 as int32_t,
                    d: 2 as int32_t,
                };
                init
            },
            aomMode: AOME_ONETWO,
        };
        init
    },
    {
        let mut init = aomScalingModeMapList {
            avifMode: {
                let mut init = avifFraction {
                    n: 1 as int32_t,
                    d: 4 as int32_t,
                };
                init
            },
            aomMode: AOME_ONEFOUR,
        };
        init
    },
    {
        let mut init = aomScalingModeMapList {
            avifMode: {
                let mut init = avifFraction {
                    n: 1 as int32_t,
                    d: 8 as int32_t,
                };
                init
            },
            aomMode: AOME_ONEEIGHT,
        };
        init
    },
    {
        let mut init = aomScalingModeMapList {
            avifMode: {
                let mut init = avifFraction {
                    n: 3 as int32_t,
                    d: 4 as int32_t,
                };
                init
            },
            aomMode: AOME_THREEFOUR,
        };
        init
    },
    {
        let mut init = aomScalingModeMapList {
            avifMode: {
                let mut init = avifFraction {
                    n: 3 as int32_t,
                    d: 5 as int32_t,
                };
                init
            },
            aomMode: AOME_THREEFIVE,
        };
        init
    },
    {
        let mut init = aomScalingModeMapList {
            avifMode: {
                let mut init = avifFraction {
                    n: 4 as int32_t,
                    d: 5 as int32_t,
                };
                init
            },
            aomMode: AOME_FOURFIVE,
        };
        init
    },
];
static mut scalingModeMapSize: core::ffi::c_int = 0;
unsafe extern "C" fn avifFindAOMScalingMode(
    mut avifMode: *const avifFraction,
    mut aomMode: *mut AOM_SCALING_MODE,
) -> avifBool {
    let mut simplifiedFraction: avifFraction = *avifMode;
    avifFractionSimplify(&mut simplifiedFraction);
    let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
    while i < scalingModeMapSize {
        if scalingModeMap[i as usize].avifMode.n == simplifiedFraction.n
            && scalingModeMap[i as usize].avifMode.d == simplifiedFraction.d
        {
            *aomMode = scalingModeMap[i as usize].aomMode;
            return AVIF_TRUE;
        }
        i += 1;
    }
    return AVIF_FALSE;
}
unsafe extern "C" fn doesLevelMatch(
    mut width: core::ffi::c_int,
    mut height: core::ffi::c_int,
    mut levelWidth: core::ffi::c_int,
    mut levelHeight: core::ffi::c_int,
    mut levelDimMult: core::ffi::c_int,
) -> avifBool {
    let levelLumaPels: int64_t = levelWidth as int64_t * levelHeight as int64_t;
    let lumaPels: int64_t = width as int64_t * height as int64_t;
    return (lumaPels <= levelLumaPels && width <= levelWidth * levelDimMult
        && height <= levelHeight * levelDimMult) as core::ffi::c_int;
}
static mut tuneIqQualityToQuantizer: [core::ffi::c_int; 101] = [
    63 as core::ffi::c_int,
    63 as core::ffi::c_int,
    63 as core::ffi::c_int,
    62 as core::ffi::c_int,
    62 as core::ffi::c_int,
    62 as core::ffi::c_int,
    61 as core::ffi::c_int,
    61 as core::ffi::c_int,
    60 as core::ffi::c_int,
    60 as core::ffi::c_int,
    59 as core::ffi::c_int,
    59 as core::ffi::c_int,
    58 as core::ffi::c_int,
    58 as core::ffi::c_int,
    57 as core::ffi::c_int,
    57 as core::ffi::c_int,
    56 as core::ffi::c_int,
    56 as core::ffi::c_int,
    55 as core::ffi::c_int,
    55 as core::ffi::c_int,
    54 as core::ffi::c_int,
    54 as core::ffi::c_int,
    53 as core::ffi::c_int,
    53 as core::ffi::c_int,
    52 as core::ffi::c_int,
    52 as core::ffi::c_int,
    51 as core::ffi::c_int,
    51 as core::ffi::c_int,
    50 as core::ffi::c_int,
    50 as core::ffi::c_int,
    49 as core::ffi::c_int,
    49 as core::ffi::c_int,
    48 as core::ffi::c_int,
    48 as core::ffi::c_int,
    47 as core::ffi::c_int,
    46 as core::ffi::c_int,
    46 as core::ffi::c_int,
    45 as core::ffi::c_int,
    45 as core::ffi::c_int,
    44 as core::ffi::c_int,
    43 as core::ffi::c_int,
    43 as core::ffi::c_int,
    42 as core::ffi::c_int,
    42 as core::ffi::c_int,
    41 as core::ffi::c_int,
    40 as core::ffi::c_int,
    40 as core::ffi::c_int,
    39 as core::ffi::c_int,
    39 as core::ffi::c_int,
    38 as core::ffi::c_int,
    37 as core::ffi::c_int,
    37 as core::ffi::c_int,
    36 as core::ffi::c_int,
    36 as core::ffi::c_int,
    35 as core::ffi::c_int,
    34 as core::ffi::c_int,
    33 as core::ffi::c_int,
    33 as core::ffi::c_int,
    32 as core::ffi::c_int,
    31 as core::ffi::c_int,
    30 as core::ffi::c_int,
    30 as core::ffi::c_int,
    29 as core::ffi::c_int,
    28 as core::ffi::c_int,
    27 as core::ffi::c_int,
    27 as core::ffi::c_int,
    26 as core::ffi::c_int,
    25 as core::ffi::c_int,
    24 as core::ffi::c_int,
    24 as core::ffi::c_int,
    23 as core::ffi::c_int,
    22 as core::ffi::c_int,
    21 as core::ffi::c_int,
    21 as core::ffi::c_int,
    20 as core::ffi::c_int,
    19 as core::ffi::c_int,
    18 as core::ffi::c_int,
    18 as core::ffi::c_int,
    17 as core::ffi::c_int,
    16 as core::ffi::c_int,
    15 as core::ffi::c_int,
    15 as core::ffi::c_int,
    14 as core::ffi::c_int,
    13 as core::ffi::c_int,
    12 as core::ffi::c_int,
    12 as core::ffi::c_int,
    11 as core::ffi::c_int,
    10 as core::ffi::c_int,
    9 as core::ffi::c_int,
    9 as core::ffi::c_int,
    8 as core::ffi::c_int,
    7 as core::ffi::c_int,
    6 as core::ffi::c_int,
    6 as core::ffi::c_int,
    5 as core::ffi::c_int,
    4 as core::ffi::c_int,
    3 as core::ffi::c_int,
    3 as core::ffi::c_int,
    2 as core::ffi::c_int,
    1 as core::ffi::c_int,
    0 as core::ffi::c_int,
];
unsafe extern "C" fn aomQualityToQuantizer(
    mut quality: core::ffi::c_int,
    mut isTuneIq: avifBool,
) -> core::ffi::c_int {
    let mut quantizer: core::ffi::c_int = 0;
    if isTuneIq != 0 {
        quantizer = tuneIqQualityToQuantizer[quality as usize];
    } else {
        quantizer = ((100 as core::ffi::c_int - quality) * 63 as core::ffi::c_int
            + 50 as core::ffi::c_int) / 100 as core::ffi::c_int;
    }
    return quantizer;
}
unsafe extern "C" fn aomCodecEncodeImage(
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
    if (*encoder).extraLayerCount > 0 as uint32_t
        && (*(*codec).internal).currentLayer == 0 as uint32_t
    {
        (*(*codec).internal).qualityFirstLayer = quality;
    }
    static mut aomVersion_3_14_0: core::ffi::c_int = (3 as core::ffi::c_int)
        << 16 as core::ffi::c_int | (14 as core::ffi::c_int) << 8 as core::ffi::c_int;
    let aomVersion: core::ffi::c_int = aom_codec_version() as core::ffi::c_int;
    let mut useAllIntraForLayered: avifBool = (aomVersion < aomVersion_3_14_0
        && (*encoder).extraLayerCount == 1 as uint32_t
        && (*(*codec).internal).qualityFirstLayer
            <= TWO_LAYER_ALL_INTRA_QUALITY_THRESHOLD) as core::ffi::c_int;
    let mut useAllIntra: avifBool = (addImageFlags
        & AVIF_ADD_IMAGE_FLAG_SINGLE as core::ffi::c_int as avifAddImageFlags != 0
        || useAllIntraForLayered != 0) as core::ffi::c_int;
    let mut aomUsage: core::ffi::c_uint = AOM_USAGE_GOOD_QUALITY;
    if useAllIntra != 0 {
        aomUsage = AOM_USAGE_ALL_INTRA;
    }
    let mut aomCpuUsed: core::ffi::c_int = -(1 as core::ffi::c_int);
    if (*encoder).speed != AVIF_SPEED_DEFAULT {
        aomCpuUsed = if (*encoder).speed < 0 as core::ffi::c_int {
            0 as core::ffi::c_int
        } else if (9 as core::ffi::c_int) < (*encoder).speed {
            9 as core::ffi::c_int
        } else {
            (*encoder).speed
        };
        if aomCpuUsed >= 7 as core::ffi::c_int {
            if useAllIntra == 0 {
                aomUsage = AOM_USAGE_REALTIME;
            }
        }
    }
    static mut aomVersion_2_0_0: core::ffi::c_int = (2 as core::ffi::c_int)
        << 16 as core::ffi::c_int;
    if aomVersion <= aomVersion_2_0_0 {
        avifDiagnosticsPrintf(
            (*codec).diag,
            b"a libaom version strictly greater than 2.0.0 is required\0" as *const u8
                as *const core::ffi::c_char,
        );
        return AVIF_RESULT_UNKNOWN_ERROR;
    }
    let mut useLibavifDefaultTuneMetric: avifBool = AVIF_FALSE;
    let mut libavifDefaultTuneMetric: aom_tune_metric = AOM_TUNE_PSNR;
    let mut useTuneIq: avifBool = 0;
    if !(avifAOMOptionsContainExplicitTuning(codec, alpha, &mut useTuneIq) != 0) {
        if (*(*codec).internal).encoderInitialized == 0 {
            if quality == AVIF_QUALITY_LOSSLESS {
                useTuneIq = AVIF_FALSE as avifBool;
            } else {
                useLibavifDefaultTuneMetric = AVIF_TRUE as avifBool;
                if alpha != 0 {
                    libavifDefaultTuneMetric = AOM_TUNE_PSNR;
                } else {
                    libavifDefaultTuneMetric = AOM_TUNE_SSIM;
                    static mut aomVersion_3_13_0: core::ffi::c_int = (3
                        as core::ffi::c_int) << 16 as core::ffi::c_int
                        | (13 as core::ffi::c_int) << 8 as core::ffi::c_int;
                    if (*image).matrixCoefficients as core::ffi::c_int
                        != AVIF_MATRIX_COEFFICIENTS_IDENTITY as core::ffi::c_int
                        && (aomUsage == AOM_USAGE_ALL_INTRA
                            && aomVersion >= aomVersion_3_13_0
                            || (*encoder).extraLayerCount > 0 as uint32_t
                                && aomVersion >= aomVersion_3_14_0)
                    {
                        libavifDefaultTuneMetric = AOM_TUNE_IQ;
                    }
                }
                useTuneIq = (libavifDefaultTuneMetric as core::ffi::c_uint
                    == AOM_TUNE_IQ as core::ffi::c_int as core::ffi::c_uint)
                    as core::ffi::c_int as avifBool;
            }
        } else {
            useTuneIq = (*(*codec).internal).previousFrameUsedTuneIq;
        }
    }
    (*(*codec).internal).previousFrameUsedTuneIq = useTuneIq;
    let mut cfg: *mut aom_codec_enc_cfg = &mut (*(*codec).internal).cfg;
    let mut quantizerUpdated: avifBool = AVIF_FALSE;
    let quantizer: core::ffi::c_int = aomQualityToQuantizer(quality, useTuneIq)
        as core::ffi::c_int;
    encoderChanges &= !(AVIF_ENCODER_CHANGE_SCALING_MODE as core::ffi::c_int);
    if (*(*codec).internal).encoderInitialized == 0 {
        (*(*codec).internal).aomFormat = avifImageCalcAOMFmt(image, alpha);
        if (*(*codec).internal).aomFormat as core::ffi::c_uint
            == AOM_IMG_FMT_NONE as core::ffi::c_int as core::ffi::c_uint
        {
            return AVIF_RESULT_UNKNOWN_ERROR;
        }
        avifGetPixelFormatInfo((*image).yuvFormat, &mut (*(*codec).internal).formatInfo);
        let mut encoderInterface: *const aom_codec_iface_t = aom_codec_av1_cx();
        let mut err: aom_codec_err_t = aom_codec_enc_config_default(
            encoderInterface,
            cfg as *mut aom_codec_enc_cfg_t,
            aomUsage,
        );
        if err as core::ffi::c_uint
            != AOM_CODEC_OK as core::ffi::c_int as core::ffi::c_uint
        {
            avifDiagnosticsPrintf(
                (*codec).diag,
                b"aom_codec_enc_config_default() failed: %s\0" as *const u8
                    as *const core::ffi::c_char,
                aom_codec_err_to_string(err),
            );
            return AVIF_RESULT_UNKNOWN_ERROR;
        }
        match aomUsage {
            AOM_USAGE_GOOD_QUALITY => {
                (*cfg).rc_end_usage = AOM_Q;
            }
            AOM_USAGE_REALTIME => {
                (*cfg).rc_end_usage = AOM_CBR;
            }
            AOM_USAGE_ALL_INTRA => {
                (*cfg).rc_end_usage = AOM_Q;
            }
            _ => {}
        }
        let mut seqProfile: uint8_t = 0 as uint8_t;
        if (*image).depth == 12 as uint32_t {
            seqProfile = 2 as uint8_t;
        } else if alpha != 0 {
            seqProfile = 0 as uint8_t;
        } else {
            match (*image).yuvFormat as core::ffi::c_uint {
                1 => {
                    seqProfile = 1 as uint8_t;
                }
                2 => {
                    seqProfile = 2 as uint8_t;
                }
                3 => {
                    seqProfile = 0 as uint8_t;
                }
                4 => {
                    seqProfile = 0 as uint8_t;
                }
                0 | 5 | _ => {}
            }
        }
        (*cfg).g_profile = seqProfile as core::ffi::c_uint;
        (*cfg).g_bit_depth = (*image).depth as aom_bit_depth_t;
        (*cfg).g_input_bit_depth = (*image).depth as core::ffi::c_uint;
        (*cfg).g_w = (*image).width as core::ffi::c_uint;
        (*cfg).g_h = (*image).height as core::ffi::c_uint;
        (*cfg).g_forced_max_frame_width = (*encoder).width as core::ffi::c_uint;
        (*cfg).g_forced_max_frame_height = (*encoder).height as core::ffi::c_uint;
        static mut aomVersion_3_6_0: core::ffi::c_int = (3 as core::ffi::c_int)
            << 16 as core::ffi::c_int | (6 as core::ffi::c_int) << 8 as core::ffi::c_int;
        if aomVersion == aomVersion_3_6_0 {
            if doesLevelMatch(
                (*image).width as core::ffi::c_int,
                (*image).height as core::ffi::c_int,
                8192 as core::ffi::c_int,
                4352 as core::ffi::c_int,
                2 as core::ffi::c_int,
            ) == 0
                && (doesLevelMatch(
                    (*image).width as core::ffi::c_int,
                    (*image).height as core::ffi::c_int,
                    16384 as core::ffi::c_int,
                    8704 as core::ffi::c_int,
                    2 as core::ffi::c_int,
                ) != 0
                    || doesLevelMatch(
                        (*image).width as core::ffi::c_int,
                        (*image).height as core::ffi::c_int,
                        32768 as core::ffi::c_int,
                        17408 as core::ffi::c_int,
                        2 as core::ffi::c_int,
                    ) != 0)
            {
                avifDiagnosticsPrintf(
                    (*codec).diag,
                    b"Detected libaom v3.6.0 bug with large images. Upgrade to libaom v3.6.1 or later.\0"
                        as *const u8 as *const core::ffi::c_char,
                );
                return AVIF_RESULT_UNKNOWN_ERROR;
            }
        }
        if addImageFlags
            & AVIF_ADD_IMAGE_FLAG_SINGLE as core::ffi::c_int as avifAddImageFlags != 0
        {
            (*cfg).g_limit = 1 as core::ffi::c_uint;
        }
        if !(useAllIntra != 0) {
            if (*encoder).keyframeInterval > 0 as core::ffi::c_int {
                (*cfg).kf_max_dist = (*encoder).keyframeInterval as core::ffi::c_uint;
            }
        }
        if (*encoder).extraLayerCount > 0 as uint32_t {
            (*cfg).g_limit = ((*encoder).extraLayerCount).wrapping_add(1 as uint32_t)
                as core::ffi::c_uint;
            (*cfg).g_lag_in_frames = 0 as core::ffi::c_uint;
            if aomVersion >= aomVersion_3_14_0 {
                if (*cfg).rc_end_usage as core::ffi::c_uint
                    == AOM_Q as core::ffi::c_int as core::ffi::c_uint
                {
                    (*cfg).use_fixed_qp_offsets = 2 as core::ffi::c_uint;
                }
            }
        }
        if disableLaggedOutput != 0 {
            (*cfg).g_lag_in_frames = 0 as core::ffi::c_uint;
        }
        if ((*encoder).width != 0 || (*encoder).height != 0)
            && (*cfg).g_lag_in_frames > 1 as core::ffi::c_uint
        {
            (*cfg).g_lag_in_frames = 1 as core::ffi::c_uint;
        }
        if (*encoder).maxThreads > 1 as core::ffi::c_int {
            (*cfg).g_threads = (if (*encoder).maxThreads < 64 as core::ffi::c_int {
                (*encoder).maxThreads
            } else {
                64 as core::ffi::c_int
            }) as core::ffi::c_uint;
            static mut aomVersion_3_15_2: core::ffi::c_int = (3 as core::ffi::c_int)
                << 16 as core::ffi::c_int
                | (15 as core::ffi::c_int) << 8 as core::ffi::c_int
                | 2 as core::ffi::c_int;
            if aom_codec_version() < aomVersion_3_15_2 {
                if aomUsage == AOM_USAGE_GOOD_QUALITY
                    && ((*encoder).width != 0 || (*encoder).height != 0)
                {
                    (*cfg).g_threads = 1 as core::ffi::c_uint;
                }
            }
        }
        if alpha != 0
            || (*image).yuvFormat as core::ffi::c_uint
                == AVIF_PIXEL_FORMAT_YUV400 as core::ffi::c_int as core::ffi::c_uint
        {
            (*cfg).monochrome = 1 as core::ffi::c_uint;
        }
        if avifProcessAOMOptionsPreInit(codec, alpha, cfg) == 0 {
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
        minQuantizer = if minQuantizer < 0 as core::ffi::c_int {
            0 as core::ffi::c_int
        } else if (63 as core::ffi::c_int) < minQuantizer {
            63 as core::ffi::c_int
        } else {
            minQuantizer
        };
        maxQuantizer = if maxQuantizer < 0 as core::ffi::c_int {
            0 as core::ffi::c_int
        } else if (63 as core::ffi::c_int) < maxQuantizer {
            63 as core::ffi::c_int
        } else {
            maxQuantizer
        };
        if (*cfg).rc_end_usage as core::ffi::c_uint
            == AOM_VBR as core::ffi::c_int as core::ffi::c_uint
            || (*cfg).rc_end_usage as core::ffi::c_uint
                == AOM_CBR as core::ffi::c_int as core::ffi::c_uint
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
        (*cfg).rc_min_quantizer = minQuantizer as core::ffi::c_uint;
        (*cfg).rc_max_quantizer = maxQuantizer as core::ffi::c_uint;
        quantizerUpdated = AVIF_TRUE as avifBool;
        let mut encoderFlags: aom_codec_flags_t = 0 as aom_codec_flags_t;
        if (*image).depth > 8 as uint32_t {
            encoderFlags |= AOM_CODEC_USE_HIGHBITDEPTH as aom_codec_flags_t;
        }
        if aom_codec_enc_init_ver(
            &mut (*(*codec).internal).encoder,
            encoderInterface,
            cfg,
            encoderFlags,
            AOM_ENCODER_ABI_VERSION,
        ) as core::ffi::c_uint != AOM_CODEC_OK as core::ffi::c_int as core::ffi::c_uint
        {
            aomDiagPrintf(
                (*codec).diag,
                b"aom_codec_enc_init()\0" as *const u8 as *const core::ffi::c_char,
                &mut (*(*codec).internal).encoder,
            );
            return AVIF_RESULT_UNKNOWN_ERROR;
        }
        (*(*codec).internal).encoderInitialized = AVIF_TRUE as avifBool;
        if (*cfg).rc_end_usage as core::ffi::c_uint
            == AOM_CQ as core::ffi::c_int as core::ffi::c_uint
            || (*cfg).rc_end_usage as core::ffi::c_uint
                == AOM_Q as core::ffi::c_int as core::ffi::c_uint
        {
            aom_codec_control(
                &mut (*(*codec).internal).encoder as *mut aom_codec_ctx_t,
                AOME_SET_CQ_LEVEL as core::ffi::c_int,
                quantizer,
            );
        }
        let mut lossless: avifBool = (quantizer == AVIF_QUANTIZER_LOSSLESS)
            as core::ffi::c_int;
        if lossless != 0 {
            aom_codec_control(
                &mut (*(*codec).internal).encoder as *mut aom_codec_ctx_t,
                AV1E_SET_LOSSLESS as core::ffi::c_int,
                1 as core::ffi::c_int,
            );
        }
        if tileRowsLog2 != 0 as core::ffi::c_int {
            aom_codec_control(
                &mut (*(*codec).internal).encoder as *mut aom_codec_ctx_t,
                AV1E_SET_TILE_ROWS as core::ffi::c_int,
                tileRowsLog2,
            );
        }
        if tileColsLog2 != 0 as core::ffi::c_int {
            aom_codec_control(
                &mut (*(*codec).internal).encoder as *mut aom_codec_ctx_t,
                AV1E_SET_TILE_COLUMNS as core::ffi::c_int,
                tileColsLog2,
            );
        }
        if (*encoder).extraLayerCount > 0 as uint32_t {
            let mut layerCount: core::ffi::c_int = ((*encoder).extraLayerCount)
                .wrapping_add(1 as uint32_t) as core::ffi::c_int;
            if aom_codec_control(
                &mut (*(*codec).internal).encoder as *mut aom_codec_ctx_t,
                AOME_SET_NUMBER_SPATIAL_LAYERS as core::ffi::c_int,
                layerCount,
            ) as core::ffi::c_uint
                != AOM_CODEC_OK as core::ffi::c_int as core::ffi::c_uint
            {
                aomDiagPrintf(
                    (*codec).diag,
                    b"aom_codec_control(AOME_SET_NUMBER_SPATIAL_LAYERS)\0" as *const u8
                        as *const core::ffi::c_char,
                    &mut (*(*codec).internal).encoder,
                );
                return AVIF_RESULT_UNKNOWN_ERROR;
            }
        }
        if aomCpuUsed != -(1 as core::ffi::c_int) {
            if aom_codec_control(
                &mut (*(*codec).internal).encoder as *mut aom_codec_ctx_t,
                AOME_SET_CPUUSED as core::ffi::c_int,
                aomCpuUsed,
            ) as core::ffi::c_uint
                != AOM_CODEC_OK as core::ffi::c_int as core::ffi::c_uint
            {
                aomDiagPrintf(
                    (*codec).diag,
                    b"aom_codec_control(AOME_SET_CPUUSED)\0" as *const u8
                        as *const core::ffi::c_char,
                    &mut (*(*codec).internal).encoder,
                );
                return AVIF_RESULT_UNKNOWN_ERROR;
            }
        }
        if alpha != 0 {
            aom_codec_control(
                &mut (*(*codec).internal).encoder as *mut aom_codec_ctx_t,
                AV1E_SET_COLOR_RANGE as core::ffi::c_int,
                AOM_CR_FULL_RANGE as core::ffi::c_int,
            );
        } else {
            if (*image).colorPrimaries as core::ffi::c_int
                != AVIF_COLOR_PRIMARIES_UNSPECIFIED as core::ffi::c_int
            {
                aom_codec_control(
                    &mut (*(*codec).internal).encoder as *mut aom_codec_ctx_t,
                    AV1E_SET_COLOR_PRIMARIES as core::ffi::c_int,
                    (*image).colorPrimaries as core::ffi::c_int,
                );
            }
            if (*image).transferCharacteristics as core::ffi::c_int
                != AVIF_TRANSFER_CHARACTERISTICS_UNSPECIFIED as core::ffi::c_int
            {
                aom_codec_control(
                    &mut (*(*codec).internal).encoder as *mut aom_codec_ctx_t,
                    AV1E_SET_TRANSFER_CHARACTERISTICS as core::ffi::c_int,
                    (*image).transferCharacteristics as core::ffi::c_int,
                );
            }
            if (*image).matrixCoefficients as core::ffi::c_int
                != AVIF_MATRIX_COEFFICIENTS_UNSPECIFIED as core::ffi::c_int
            {
                aom_codec_control(
                    &mut (*(*codec).internal).encoder as *mut aom_codec_ctx_t,
                    AV1E_SET_MATRIX_COEFFICIENTS as core::ffi::c_int,
                    (*image).matrixCoefficients as core::ffi::c_int,
                );
            }
            if (*image).yuvChromaSamplePosition as core::ffi::c_uint
                != AVIF_CHROMA_SAMPLE_POSITION_UNKNOWN as core::ffi::c_int
                    as core::ffi::c_uint
            {
                aom_codec_control(
                    &mut (*(*codec).internal).encoder as *mut aom_codec_ctx_t,
                    AV1E_SET_CHROMA_SAMPLE_POSITION as core::ffi::c_int,
                    (*image).yuvChromaSamplePosition as core::ffi::c_int,
                );
            }
            if (*image).yuvRange as core::ffi::c_uint
                != AVIF_RANGE_LIMITED as core::ffi::c_int as core::ffi::c_uint
            {
                aom_codec_control(
                    &mut (*(*codec).internal).encoder as *mut aom_codec_ctx_t,
                    AV1E_SET_COLOR_RANGE as core::ffi::c_int,
                    (*image).yuvRange as core::ffi::c_int,
                );
            }
        }
        if (*cfg).g_usage == AOM_USAGE_ALL_INTRA {
            aom_codec_control(
                &mut (*(*codec).internal).encoder as *mut aom_codec_ctx_t,
                AV1E_SET_SKIP_POSTPROC_FILTERING as core::ffi::c_int,
                1 as core::ffi::c_int,
            );
        }
        if useLibavifDefaultTuneMetric != 0
            && aom_codec_control(
                &mut (*(*codec).internal).encoder as *mut aom_codec_ctx_t,
                AOME_SET_TUNING as core::ffi::c_int,
                libavifDefaultTuneMetric as core::ffi::c_uint,
            ) as core::ffi::c_uint
                != AOM_CODEC_OK as core::ffi::c_int as core::ffi::c_uint
        {
            aomDiagPrintf(
                (*codec).diag,
                b"aom_codec_control(AOME_SET_TUNING)\0" as *const u8
                    as *const core::ffi::c_char,
                &mut (*(*codec).internal).encoder,
            );
            return AVIF_RESULT_UNKNOWN_ERROR;
        }
        if avifProcessAOMOptionsPostInit(codec, alpha) == 0 {
            return AVIF_RESULT_INVALID_CODEC_SPECIFIC_OPTION;
        }
        static mut aomVersion_3_15_2_0: core::ffi::c_int = (3 as core::ffi::c_int)
            << 16 as core::ffi::c_int | (15 as core::ffi::c_int) << 8 as core::ffi::c_int
            | 2 as core::ffi::c_int;
        if aomVersion < aomVersion_3_15_2_0 && (*image).depth == 12 as uint32_t {
            if aom_codec_control(
                &mut (*(*codec).internal).encoder as *mut aom_codec_ctx_t,
                AV1E_SET_ENABLE_RESTORATION as core::ffi::c_int,
                0 as core::ffi::c_int,
            ) as core::ffi::c_uint
                != AOM_CODEC_OK as core::ffi::c_int as core::ffi::c_uint
            {
                aomDiagPrintf(
                    (*codec).diag,
                    b"aom_codec_control(AV1E_SET_ENABLE_RESTORATION)\0" as *const u8
                        as *const core::ffi::c_char,
                    &mut (*(*codec).internal).encoder,
                );
                return AVIF_RESULT_UNKNOWN_ERROR;
            }
        }
    } else {
        let mut dimensionsChanged: avifBool = AVIF_FALSE;
        if (*cfg).g_w as uint32_t != (*image).width
            || (*cfg).g_h as uint32_t != (*image).height
        {
            (*cfg).g_w = (*image).width as core::ffi::c_uint;
            (*cfg).g_h = (*image).height as core::ffi::c_uint;
            dimensionsChanged = AVIF_TRUE as avifBool;
        }
        if alpha != 0 {
            if encoderChanges as core::ffi::c_int
                & (AVIF_ENCODER_CHANGE_MIN_QUANTIZER_ALPHA as core::ffi::c_int
                    | AVIF_ENCODER_CHANGE_MAX_QUANTIZER_ALPHA as core::ffi::c_int) != 0
            {
                (*cfg).rc_min_quantizer = (if (*encoder).minQuantizerAlpha
                    < 0 as core::ffi::c_int
                {
                    0 as core::ffi::c_int
                } else if (63 as core::ffi::c_int) < (*encoder).minQuantizerAlpha {
                    63 as core::ffi::c_int
                } else {
                    (*encoder).minQuantizerAlpha
                }) as core::ffi::c_uint;
                (*cfg).rc_max_quantizer = (if (*encoder).maxQuantizerAlpha
                    < 0 as core::ffi::c_int
                {
                    0 as core::ffi::c_int
                } else if (63 as core::ffi::c_int) < (*encoder).maxQuantizerAlpha {
                    63 as core::ffi::c_int
                } else {
                    (*encoder).maxQuantizerAlpha
                }) as core::ffi::c_uint;
                quantizerUpdated = AVIF_TRUE as avifBool;
            }
        } else if encoderChanges as core::ffi::c_int
            & (AVIF_ENCODER_CHANGE_MIN_QUANTIZER as core::ffi::c_int
                | AVIF_ENCODER_CHANGE_MAX_QUANTIZER as core::ffi::c_int) != 0
        {
            (*cfg).rc_min_quantizer = (if (*encoder).minQuantizer < 0 as core::ffi::c_int
            {
                0 as core::ffi::c_int
            } else if (63 as core::ffi::c_int) < (*encoder).minQuantizer {
                63 as core::ffi::c_int
            } else {
                (*encoder).minQuantizer
            }) as core::ffi::c_uint;
            (*cfg).rc_max_quantizer = (if (*encoder).maxQuantizer < 0 as core::ffi::c_int
            {
                0 as core::ffi::c_int
            } else if (63 as core::ffi::c_int) < (*encoder).maxQuantizer {
                63 as core::ffi::c_int
            } else {
                (*encoder).maxQuantizer
            }) as core::ffi::c_uint;
            quantizerUpdated = AVIF_TRUE as avifBool;
        }
        let qualityChangedBit: core::ffi::c_int = if alpha != 0 {
            AVIF_ENCODER_CHANGE_QUALITY_ALPHA as core::ffi::c_int as core::ffi::c_int
        } else {
            AVIF_ENCODER_CHANGE_QUALITY as core::ffi::c_int as core::ffi::c_int
        };
        if encoderChanges as core::ffi::c_int & qualityChangedBit != 0 {
            if (*cfg).rc_end_usage as core::ffi::c_uint
                == AOM_VBR as core::ffi::c_int as core::ffi::c_uint
                || (*cfg).rc_end_usage as core::ffi::c_uint
                    == AOM_CBR as core::ffi::c_int as core::ffi::c_uint
            {
                if quantizer == AVIF_QUANTIZER_LOSSLESS {
                    (*cfg).rc_min_quantizer = AVIF_QUANTIZER_LOSSLESS
                        as core::ffi::c_uint;
                    (*cfg).rc_max_quantizer = AVIF_QUANTIZER_LOSSLESS
                        as core::ffi::c_uint;
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
                    minQuantizer_0 = if minQuantizer_0 < 0 as core::ffi::c_int {
                        0 as core::ffi::c_int
                    } else if (63 as core::ffi::c_int) < minQuantizer_0 {
                        63 as core::ffi::c_int
                    } else {
                        minQuantizer_0
                    };
                    maxQuantizer_0 = if maxQuantizer_0 < 0 as core::ffi::c_int {
                        0 as core::ffi::c_int
                    } else if (63 as core::ffi::c_int) < maxQuantizer_0 {
                        63 as core::ffi::c_int
                    } else {
                        maxQuantizer_0
                    };
                    (*cfg).rc_min_quantizer = (if quantizer - 4 as core::ffi::c_int
                        > minQuantizer_0
                    {
                        quantizer - 4 as core::ffi::c_int
                    } else {
                        minQuantizer_0
                    }) as core::ffi::c_uint;
                    (*cfg).rc_max_quantizer = (if (quantizer + 4 as core::ffi::c_int)
                        < maxQuantizer_0
                    {
                        quantizer + 4 as core::ffi::c_int
                    } else {
                        maxQuantizer_0
                    }) as core::ffi::c_uint;
                }
                quantizerUpdated = AVIF_TRUE as avifBool;
            }
        }
        if quantizerUpdated != 0 || dimensionsChanged != 0 {
            let mut err_0: aom_codec_err_t = aom_codec_enc_config_set(
                &mut (*(*codec).internal).encoder,
                cfg,
            );
            if err_0 as core::ffi::c_uint
                != AOM_CODEC_OK as core::ffi::c_int as core::ffi::c_uint
            {
                aomDiagPrintf(
                    (*codec).diag,
                    b"aom_codec_enc_config_set()\0" as *const u8
                        as *const core::ffi::c_char,
                    &mut (*(*codec).internal).encoder,
                );
                return AVIF_RESULT_UNKNOWN_ERROR;
            }
        }
        if encoderChanges as core::ffi::c_int
            & AVIF_ENCODER_CHANGE_TILE_ROWS_LOG2 as core::ffi::c_int != 0
        {
            aom_codec_control(
                &mut (*(*codec).internal).encoder as *mut aom_codec_ctx_t,
                AV1E_SET_TILE_ROWS as core::ffi::c_int,
                tileRowsLog2,
            );
        }
        if encoderChanges as core::ffi::c_int
            & AVIF_ENCODER_CHANGE_TILE_COLS_LOG2 as core::ffi::c_int != 0
        {
            aom_codec_control(
                &mut (*(*codec).internal).encoder as *mut aom_codec_ctx_t,
                AV1E_SET_TILE_COLUMNS as core::ffi::c_int,
                tileColsLog2,
            );
        }
        if encoderChanges as core::ffi::c_int & qualityChangedBit != 0 {
            if (*cfg).rc_end_usage as core::ffi::c_uint
                == AOM_CQ as core::ffi::c_int as core::ffi::c_uint
                || (*cfg).rc_end_usage as core::ffi::c_uint
                    == AOM_Q as core::ffi::c_int as core::ffi::c_uint
            {
                aom_codec_control(
                    &mut (*(*codec).internal).encoder as *mut aom_codec_ctx_t,
                    AOME_SET_CQ_LEVEL as core::ffi::c_int,
                    quantizer,
                );
            }
            let mut lossless_0: avifBool = (quantizer == AVIF_QUANTIZER_LOSSLESS)
                as core::ffi::c_int;
            aom_codec_control(
                &mut (*(*codec).internal).encoder as *mut aom_codec_ctx_t,
                AV1E_SET_LOSSLESS as core::ffi::c_int,
                lossless_0,
            );
        }
        if encoderChanges as core::ffi::c_int
            & AVIF_ENCODER_CHANGE_CODEC_SPECIFIC as core::ffi::c_int != 0
        {
            if avifProcessAOMOptionsPostInit(codec, alpha) == 0 {
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
        aom_codec_control(
            &mut (*(*codec).internal).encoder as *mut aom_codec_ctx_t,
            AOME_SET_SPATIAL_LAYER_ID as core::ffi::c_int,
            (*(*codec).internal).currentLayer,
        );
    }
    let mut aomScalingMode: aom_scaling_mode_t = aom_scaling_mode {
        h_scaling_mode: AOME_NORMAL,
        v_scaling_mode: AOME_NORMAL,
    };
    if avifFindAOMScalingMode(
        &mut (*encoder).scalingMode.horizontal,
        &mut aomScalingMode.h_scaling_mode,
    ) == 0
    {
        return AVIF_RESULT_NOT_IMPLEMENTED;
    }
    if avifFindAOMScalingMode(
        &mut (*encoder).scalingMode.vertical,
        &mut aomScalingMode.v_scaling_mode,
    ) == 0
    {
        return AVIF_RESULT_NOT_IMPLEMENTED;
    }
    if aomScalingMode.h_scaling_mode as core::ffi::c_uint
        != AOME_NORMAL as core::ffi::c_int as core::ffi::c_uint
        || aomScalingMode.v_scaling_mode as core::ffi::c_uint
            != AOME_NORMAL as core::ffi::c_int as core::ffi::c_uint
    {
        aom_codec_control(
            &mut (*(*codec).internal).encoder as *mut aom_codec_ctx_t,
            AOME_SET_SCALEMODE as core::ffi::c_int,
            &mut aomScalingMode as *mut aom_scaling_mode_t,
        );
        static mut aomVersion_3_13_3: core::ffi::c_int = (3 as core::ffi::c_int)
            << 16 as core::ffi::c_int | (13 as core::ffi::c_int) << 8 as core::ffi::c_int
            | 3 as core::ffi::c_int;
        if aomVersion < aomVersion_3_13_3
            && (*encoder).maxThreads > 1 as core::ffi::c_int
            && (*encoder).extraLayerCount > 0 as uint32_t
            && (*(*codec).internal).currentLayer == 0 as uint32_t
        {
            aom_codec_control(
                &mut (*(*codec).internal).encoder as *mut aom_codec_ctx_t,
                AV1E_SET_ENABLE_RESTORATION as core::ffi::c_int,
                0 as core::ffi::c_int,
            );
        }
    }
    let mut aomImage: aom_image_t = aom_image {
        fmt: AOM_IMG_FMT_NONE,
        cp: AOM_CICP_CP_RESERVED_0,
        tc: AOM_CICP_TC_RESERVED_0,
        mc: AOM_CICP_MC_IDENTITY,
        monochrome: 0,
        csp: AOM_CSP_UNKNOWN,
        range: AOM_CR_STUDIO_RANGE,
        w: 0,
        h: 0,
        bit_depth: 0,
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
        temporal_id: 0,
        spatial_id: 0,
        user_priv: 0 as *mut core::ffi::c_void,
        img_data: 0 as *mut core::ffi::c_uchar,
        img_data_owner: 0,
        self_allocd: 0,
        metadata: 0 as *mut aom_metadata_array_t,
        fb_priv: 0 as *mut core::ffi::c_void,
    };
    let mut aomImageAllocated: avifBool = ((*image).width == 1 as uint32_t
        || (*image).height == 1 as uint32_t) as core::ffi::c_int;
    if aomImageAllocated != 0 {
        aom_img_alloc(
            &mut aomImage,
            (*(*codec).internal).aomFormat,
            (*image).width as core::ffi::c_uint,
            (*image).height as core::ffi::c_uint,
            16 as core::ffi::c_uint,
        );
    } else {
        memset(
            &mut aomImage as *mut aom_image_t as *mut core::ffi::c_void,
            0 as core::ffi::c_int,
            ::core::mem::size_of::<aom_image_t>() as size_t,
        );
        aomImage.fmt = (*(*codec).internal).aomFormat;
        aomImage.bit_depth = (if (*image).depth > 8 as uint32_t {
            16 as core::ffi::c_int
        } else {
            8 as core::ffi::c_int
        }) as core::ffi::c_uint;
        aomImage.w = (*image).width as core::ffi::c_uint;
        aomImage.h = (*image).height as core::ffi::c_uint;
        aomImage.d_w = (*image).width as core::ffi::c_uint;
        aomImage.d_h = (*image).height as core::ffi::c_uint;
        let mut bps: core::ffi::c_uint = 0;
        if (*(*codec).internal).aomFormat as core::ffi::c_uint
            == AOM_IMG_FMT_I420 as core::ffi::c_int as core::ffi::c_uint
        {
            bps = 12 as core::ffi::c_uint;
        } else if (*(*codec).internal).aomFormat as core::ffi::c_uint
            == AOM_IMG_FMT_I422 as core::ffi::c_int as core::ffi::c_uint
        {
            bps = 16 as core::ffi::c_uint;
        } else if (*(*codec).internal).aomFormat as core::ffi::c_uint
            == AOM_IMG_FMT_I444 as core::ffi::c_int as core::ffi::c_uint
        {
            bps = 24 as core::ffi::c_uint;
        } else if (*(*codec).internal).aomFormat as core::ffi::c_uint
            == AOM_IMG_FMT_I42016 as core::ffi::c_int as core::ffi::c_uint
        {
            bps = 24 as core::ffi::c_uint;
        } else if (*(*codec).internal).aomFormat as core::ffi::c_uint
            == AOM_IMG_FMT_I42216 as core::ffi::c_int as core::ffi::c_uint
        {
            bps = 32 as core::ffi::c_uint;
        } else if (*(*codec).internal).aomFormat as core::ffi::c_uint
            == AOM_IMG_FMT_I44416 as core::ffi::c_int as core::ffi::c_uint
        {
            bps = 48 as core::ffi::c_uint;
        } else {
            bps = 16 as core::ffi::c_uint;
        }
        aomImage.bps = bps as core::ffi::c_int;
        aomImage.x_chroma_shift = (if alpha != 0
            || (*(*codec).internal).formatInfo.monochrome != 0
        {
            1 as core::ffi::c_int
        } else {
            (*(*codec).internal).formatInfo.chromaShiftX
        }) as core::ffi::c_uint;
        aomImage.y_chroma_shift = (if alpha != 0
            || (*(*codec).internal).formatInfo.monochrome != 0
        {
            1 as core::ffi::c_int
        } else {
            (*(*codec).internal).formatInfo.chromaShiftY
        }) as core::ffi::c_uint;
    }
    if alpha != 0 {
        if aomImageAllocated != 0 {
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
                let mut dstAlphaRow: *mut uint8_t = &mut *(*(aomImage.planes)
                    .as_mut_ptr()
                    .offset(0 as core::ffi::c_int as isize))
                    .offset(
                        (j as size_t)
                            .wrapping_mul(
                                *(aomImage.stride)
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
            aomImage.planes[0 as core::ffi::c_int as usize] = (*image).alphaPlane
                as *mut core::ffi::c_uchar;
            aomImage.stride[0 as core::ffi::c_int as usize] = (*image).alphaRowBytes
                as core::ffi::c_int;
        }
    } else {
        let mut yuvPlaneCount: core::ffi::c_int = 3 as core::ffi::c_int;
        if (*image).yuvFormat as core::ffi::c_uint
            == AVIF_PIXEL_FORMAT_YUV400 as core::ffi::c_int as core::ffi::c_uint
        {
            yuvPlaneCount = 1 as core::ffi::c_int;
        }
        if aomImageAllocated != 0 {
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
                    let mut dstRow: *mut uint8_t = &mut *(*(aomImage.planes)
                        .as_mut_ptr()
                        .offset(yuvPlane as isize))
                        .offset(
                            (j_0 as size_t)
                                .wrapping_mul(
                                    *(aomImage.stride).as_mut_ptr().offset(yuvPlane as isize)
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
                aomImage.planes[yuvPlane_0 as usize] = (*image)
                    .yuvPlanes[yuvPlane_0 as usize] as *mut core::ffi::c_uchar;
                aomImage.stride[yuvPlane_0 as usize] = (*image)
                    .yuvRowBytes[yuvPlane_0 as usize] as core::ffi::c_int;
                yuvPlane_0 += 1;
            }
        }
    }
    let mut encodeFlags: aom_enc_frame_flags_t = 0 as aom_enc_frame_flags_t;
    if addImageFlags
        & AVIF_ADD_IMAGE_FLAG_FORCE_KEYFRAME as core::ffi::c_int as avifAddImageFlags
        != 0
    {
        encodeFlags |= AOM_EFLAG_FORCE_KF as aom_enc_frame_flags_t;
    }
    if (*(*codec).internal).currentLayer > 0 as uint32_t {
        encodeFlags
            |= (AOM_EFLAG_NO_REF_GF | AOM_EFLAG_NO_REF_ARF | AOM_EFLAG_NO_REF_BWD
                | AOM_EFLAG_NO_REF_ARF2 | AOM_EFLAG_NO_UPD_GF | AOM_EFLAG_NO_UPD_ARF)
                as aom_enc_frame_flags_t;
    }
    let mut encodeErr: aom_codec_err_t = aom_codec_encode(
        &mut (*(*codec).internal).encoder,
        &mut aomImage,
        0 as aom_codec_pts_t,
        1 as core::ffi::c_ulong,
        encodeFlags,
    );
    if aomImageAllocated != 0 {
        aom_img_free(&mut aomImage);
    }
    if encodeErr as core::ffi::c_uint
        != AOM_CODEC_OK as core::ffi::c_int as core::ffi::c_uint
    {
        aomDiagPrintf(
            (*codec).diag,
            b"aom_codec_encode()\0" as *const u8 as *const core::ffi::c_char,
            &mut (*(*codec).internal).encoder,
        );
        return AVIF_RESULT_UNKNOWN_ERROR;
    }
    let mut iter: aom_codec_iter_t = 0 as aom_codec_iter_t;
    loop {
        let mut pkt: *const aom_codec_cx_pkt_t = aom_codec_get_cx_data(
            &mut (*(*codec).internal).encoder,
            &mut iter,
        );
        if pkt.is_null() {
            break;
        }
        if (*pkt).kind as core::ffi::c_uint
            == AOM_CODEC_CX_FRAME_PKT as core::ffi::c_int as core::ffi::c_uint
        {
            let result__: avifResult = avifCodecEncodeOutputAddSample(
                output,
                (*pkt).data.frame.buf as *const uint8_t,
                (*pkt).data.frame.sz,
                ((*pkt).data.frame.flags & 0x1 as aom_codec_frame_flags_t) as avifBool,
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
        if aomCodecEncodeFinish(codec, output) == 0 {
            return AVIF_RESULT_UNKNOWN_ERROR;
        }
        aom_codec_destroy(&mut (*(*codec).internal).encoder);
        (*(*codec).internal).encoderInitialized = AVIF_FALSE as avifBool;
    }
    if (*encoder).extraLayerCount > 0 as uint32_t {
        (*(*codec).internal).currentLayer = ((*(*codec).internal).currentLayer)
            .wrapping_add(1);
    }
    return AVIF_RESULT_OK;
}
unsafe extern "C" fn aomCodecEncodeFinish(
    mut codec: *mut avifCodec,
    mut output: *mut avifCodecEncodeOutput,
) -> avifBool {
    if (*(*codec).internal).encoderInitialized == 0 {
        return AVIF_TRUE;
    }
    loop {
        if aom_codec_encode(
            &mut (*(*codec).internal).encoder,
            0 as *const aom_image_t,
            0 as aom_codec_pts_t,
            1 as core::ffi::c_ulong,
            0 as aom_enc_frame_flags_t,
        ) as core::ffi::c_uint != AOM_CODEC_OK as core::ffi::c_int as core::ffi::c_uint
        {
            aomDiagPrintf(
                (*codec).diag,
                b"aom_codec_encode() with img=NULL\0" as *const u8
                    as *const core::ffi::c_char,
                &mut (*(*codec).internal).encoder,
            );
            return AVIF_FALSE;
        }
        let mut gotPacket: avifBool = AVIF_FALSE;
        let mut iter: aom_codec_iter_t = 0 as aom_codec_iter_t;
        loop {
            let mut pkt: *const aom_codec_cx_pkt_t = aom_codec_get_cx_data(
                &mut (*(*codec).internal).encoder,
                &mut iter,
            );
            if pkt.is_null() {
                break;
            }
            if (*pkt).kind as core::ffi::c_uint
                == AOM_CODEC_CX_FRAME_PKT as core::ffi::c_int as core::ffi::c_uint
            {
                gotPacket = AVIF_TRUE as avifBool;
                let result: avifResult = avifCodecEncodeOutputAddSample(
                    output,
                    (*pkt).data.frame.buf as *const uint8_t,
                    (*pkt).data.frame.sz,
                    ((*pkt).data.frame.flags
                        & AOM_FRAME_IS_KEY as aom_codec_frame_flags_t) as avifBool,
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
pub unsafe extern "C" fn avifCodecVersionAOM() -> *const core::ffi::c_char {
    return aom_codec_version_str();
}
#[no_mangle]
pub unsafe extern "C" fn avifCodecCreateAOM() -> *mut avifCodec {
    let mut codec: *mut avifCodec = avifCalloc(
        1 as size_t,
        ::core::mem::size_of::<avifCodec>() as size_t,
    ) as *mut avifCodec;
    if codec.is_null() {
        return 0 as *mut avifCodec;
    }
    (*codec).getNextImage = Some(
        aomCodecGetNextImage
            as unsafe extern "C" fn(
                *mut avifCodec,
                *const avifDecodeSample,
                avifBool,
                *mut avifBool,
                *mut avifImage,
            ) -> avifBool,
    ) as avifCodecGetNextImageFunc;
    (*codec).encodeImage = Some(
        aomCodecEncodeImage
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
        aomCodecEncodeFinish
            as unsafe extern "C" fn(
                *mut avifCodec,
                *mut avifCodecEncodeOutput,
            ) -> avifBool,
    ) as avifCodecEncodeFinishFunc;
    (*codec).destroyInternal = Some(
        aomCodecDestroyInternal as unsafe extern "C" fn(*mut avifCodec) -> (),
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
    scalingModeMapSize = (::core::mem::size_of::<[aomScalingModeMapList; 7]>() as usize)
        .wrapping_div(::core::mem::size_of::<aomScalingModeMapList>() as usize)
        as core::ffi::c_int;
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
#[cfg_attr(libavif_rs_asan, sanitize(address = "off"))]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
