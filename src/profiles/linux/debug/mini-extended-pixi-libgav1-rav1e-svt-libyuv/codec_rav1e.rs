// Copyright 2019 Joe Drago. All rights reserved.
// SPDX-License-Identifier: BSD-2-Clause

// Mechanically translated from upstream/libavif/src/codec_rav1e.c.
// C ABI, ownership, pointer validity, and bounds follow the original headers.
extern "C" {
    pub type avifEncoderData;
    pub type RaContext;
    pub type RaFrame;
    pub type RaConfig;
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
    fn atoi(__nptr: *const core::ffi::c_char) -> core::ffi::c_int;
    fn rav1e_version_short() -> *const core::ffi::c_char;
    fn rav1e_version_full() -> *const core::ffi::c_char;
    fn rav1e_config_default() -> *mut RaConfig;
    fn rav1e_config_set_pixel_format(
        cfg: *mut RaConfig,
        bit_depth: uint8_t,
        subsampling: RaChromaSampling,
        chroma_pos: RaChromaSamplePosition,
        pixel_range: RaPixelRange,
    ) -> core::ffi::c_int;
    fn rav1e_config_set_color_description(
        cfg: *mut RaConfig,
        matrix: RaMatrixCoefficients,
        primaries: RaColorPrimaries,
        transfer: RaTransferCharacteristics,
    ) -> core::ffi::c_int;
    fn rav1e_config_unref(cfg: *mut RaConfig);
    fn rav1e_config_parse(
        cfg: *mut RaConfig,
        key: *const core::ffi::c_char,
        value: *const core::ffi::c_char,
    ) -> core::ffi::c_int;
    fn rav1e_config_parse_int(
        cfg: *mut RaConfig,
        key: *const core::ffi::c_char,
        value: core::ffi::c_int,
    ) -> core::ffi::c_int;
    fn rav1e_context_new(cfg: *const RaConfig) -> *mut RaContext;
    fn rav1e_context_unref(ctx: *mut RaContext);
    fn rav1e_frame_new(ctx: *const RaContext) -> *mut RaFrame;
    fn rav1e_frame_unref(frame: *mut RaFrame);
    fn rav1e_frame_set_type(
        frame: *mut RaFrame,
        frame_type: RaFrameTypeOverride,
    ) -> core::ffi::c_int;
    fn rav1e_send_frame(ctx: *mut RaContext, frame: *mut RaFrame) -> RaEncoderStatus;
    fn rav1e_receive_packet(
        ctx: *mut RaContext,
        pkt: *mut *mut RaPacket,
    ) -> RaEncoderStatus;
    fn rav1e_packet_unref(pkt: *mut RaPacket);
    fn rav1e_frame_fill_plane(
        frame: *mut RaFrame,
        plane: core::ffi::c_int,
        data: *const uint8_t,
        data_len: size_t,
        stride: ptrdiff_t,
        bytewidth: core::ffi::c_int,
    );
    fn strchr(
        __s: *const core::ffi::c_char,
        __c: core::ffi::c_int,
    ) -> *mut core::ffi::c_char;
}
pub type ptrdiff_t = isize;
pub type size_t = usize;
pub type __uint8_t = u8;
pub type __uint16_t = u16;
pub type __int32_t = i32;
pub type __uint32_t = u32;
pub type __uint64_t = u64;
pub type int32_t = __int32_t;
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
    pub rav1eContext: *mut RaContext,
    pub chromaSampling: RaChromaSampling,
    pub yShift: core::ffi::c_int,
    pub encodeWidth: uint32_t,
    pub encodeHeight: uint32_t,
}
pub type RaChromaSampling = core::ffi::c_uint;
pub const RA_CHROMA_SAMPLING_CS400: RaChromaSampling = 3;
pub const RA_CHROMA_SAMPLING_CS444: RaChromaSampling = 2;
pub const RA_CHROMA_SAMPLING_CS422: RaChromaSampling = 1;
pub const RA_CHROMA_SAMPLING_CS420: RaChromaSampling = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RaPacket {
    pub data: *const uint8_t,
    pub len: size_t,
    pub input_frameno: uint64_t,
    pub frame_type: RaFrameType,
    pub opaque: *mut core::ffi::c_void,
    pub rec: *mut RaFrame,
    pub source: *mut RaFrame,
}
pub type RaFrameType = core::ffi::c_uint;
pub const RA_FRAME_TYPE_SWITCH: RaFrameType = 3;
pub const RA_FRAME_TYPE_INTRA_ONLY: RaFrameType = 2;
pub const RA_FRAME_TYPE_INTER: RaFrameType = 1;
pub const RA_FRAME_TYPE_KEY: RaFrameType = 0;
pub const RA_ENCODER_STATUS_LIMIT_REACHED: RaEncoderStatus = 3;
pub type RaEncoderStatus = core::ffi::c_int;
pub const RA_ENCODER_STATUS_NOT_READY: RaEncoderStatus = -2;
pub const RA_ENCODER_STATUS_FAILURE: RaEncoderStatus = -1;
pub const RA_ENCODER_STATUS_ENCODED: RaEncoderStatus = 4;
pub const RA_ENCODER_STATUS_ENOUGH_DATA: RaEncoderStatus = 2;
pub const RA_ENCODER_STATUS_NEED_MORE_DATA: RaEncoderStatus = 1;
pub const RA_ENCODER_STATUS_SUCCESS: RaEncoderStatus = 0;
pub type RaFrameTypeOverride = core::ffi::c_uint;
pub const RA_FRAME_TYPE_OVERRIDE_KEY: RaFrameTypeOverride = 1;
pub const RA_FRAME_TYPE_OVERRIDE_NO: RaFrameTypeOverride = 0;
pub type RaTransferCharacteristics = core::ffi::c_uint;
pub const RA_TRANSFER_CHARACTERISTICS_HLG: RaTransferCharacteristics = 18;
pub const RA_TRANSFER_CHARACTERISTICS_SMPTE428: RaTransferCharacteristics = 17;
pub const RA_TRANSFER_CHARACTERISTICS_SMPTE2084: RaTransferCharacteristics = 16;
pub const RA_TRANSFER_CHARACTERISTICS_BT2020_12BIT: RaTransferCharacteristics = 15;
pub const RA_TRANSFER_CHARACTERISTICS_BT2020_10BIT: RaTransferCharacteristics = 14;
pub const RA_TRANSFER_CHARACTERISTICS_SRGB: RaTransferCharacteristics = 13;
pub const RA_TRANSFER_CHARACTERISTICS_BT1361: RaTransferCharacteristics = 12;
pub const RA_TRANSFER_CHARACTERISTICS_IEC61966: RaTransferCharacteristics = 11;
pub const RA_TRANSFER_CHARACTERISTICS_LOG100_SQRT10: RaTransferCharacteristics = 10;
pub const RA_TRANSFER_CHARACTERISTICS_LOG100: RaTransferCharacteristics = 9;
pub const RA_TRANSFER_CHARACTERISTICS_LINEAR: RaTransferCharacteristics = 8;
pub const RA_TRANSFER_CHARACTERISTICS_SMPTE240: RaTransferCharacteristics = 7;
pub const RA_TRANSFER_CHARACTERISTICS_BT601: RaTransferCharacteristics = 6;
pub const RA_TRANSFER_CHARACTERISTICS_BT470BG: RaTransferCharacteristics = 5;
pub const RA_TRANSFER_CHARACTERISTICS_BT470M: RaTransferCharacteristics = 4;
pub const RA_TRANSFER_CHARACTERISTICS_UNSPECIFIED: RaTransferCharacteristics = 2;
pub const RA_TRANSFER_CHARACTERISTICS_BT709: RaTransferCharacteristics = 1;
pub type RaColorPrimaries = core::ffi::c_uint;
pub const RA_COLOR_PRIMARIES_EBU3213: RaColorPrimaries = 22;
pub const RA_COLOR_PRIMARIES_SMPTE432: RaColorPrimaries = 12;
pub const RA_COLOR_PRIMARIES_SMPTE431: RaColorPrimaries = 11;
pub const RA_COLOR_PRIMARIES_XYZ: RaColorPrimaries = 10;
pub const RA_COLOR_PRIMARIES_BT2020: RaColorPrimaries = 9;
pub const RA_COLOR_PRIMARIES_GENERIC_FILM: RaColorPrimaries = 8;
pub const RA_COLOR_PRIMARIES_SMPTE240: RaColorPrimaries = 7;
pub const RA_COLOR_PRIMARIES_BT601: RaColorPrimaries = 6;
pub const RA_COLOR_PRIMARIES_BT470BG: RaColorPrimaries = 5;
pub const RA_COLOR_PRIMARIES_BT470M: RaColorPrimaries = 4;
pub const RA_COLOR_PRIMARIES_UNSPECIFIED: RaColorPrimaries = 2;
pub const RA_COLOR_PRIMARIES_BT709: RaColorPrimaries = 1;
pub type RaMatrixCoefficients = core::ffi::c_uint;
pub const RA_MATRIX_COEFFICIENTS_I_CT_CP: RaMatrixCoefficients = 14;
pub const RA_MATRIX_COEFFICIENTS_CHROMAT_CL: RaMatrixCoefficients = 13;
pub const RA_MATRIX_COEFFICIENTS_CHROMAT_NCL: RaMatrixCoefficients = 12;
pub const RA_MATRIX_COEFFICIENTS_SMPTE2085: RaMatrixCoefficients = 11;
pub const RA_MATRIX_COEFFICIENTS_BT2020CL: RaMatrixCoefficients = 10;
pub const RA_MATRIX_COEFFICIENTS_BT2020NCL: RaMatrixCoefficients = 9;
pub const RA_MATRIX_COEFFICIENTS_Y_CG_CO: RaMatrixCoefficients = 8;
pub const RA_MATRIX_COEFFICIENTS_SMPTE240: RaMatrixCoefficients = 7;
pub const RA_MATRIX_COEFFICIENTS_BT601: RaMatrixCoefficients = 6;
pub const RA_MATRIX_COEFFICIENTS_BT470BG: RaMatrixCoefficients = 5;
pub const RA_MATRIX_COEFFICIENTS_FCC: RaMatrixCoefficients = 4;
pub const RA_MATRIX_COEFFICIENTS_UNSPECIFIED: RaMatrixCoefficients = 2;
pub const RA_MATRIX_COEFFICIENTS_BT709: RaMatrixCoefficients = 1;
pub const RA_MATRIX_COEFFICIENTS_IDENTITY: RaMatrixCoefficients = 0;
pub type RaPixelRange = core::ffi::c_uint;
pub const RA_PIXEL_RANGE_FULL: RaPixelRange = 1;
pub const RA_PIXEL_RANGE_LIMITED: RaPixelRange = 0;
pub type RaChromaSamplePosition = core::ffi::c_uint;
pub const RA_CHROMA_SAMPLE_POSITION_COLOCATED: RaChromaSamplePosition = 2;
pub const RA_CHROMA_SAMPLE_POSITION_VERTICAL: RaChromaSamplePosition = 1;
pub const RA_CHROMA_SAMPLE_POSITION_UNKNOWN: RaChromaSamplePosition = 0;
pub const NULL: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
pub const AVIF_TRUE: core::ffi::c_int = 1 as core::ffi::c_int;
pub const AVIF_FALSE: core::ffi::c_int = 0 as core::ffi::c_int;
pub const AVIF_SPEED_DEFAULT: core::ffi::c_int = -(1 as core::ffi::c_int);
unsafe extern "C" fn rav1eCodecDestroyInternal(mut codec: *mut avifCodec) {
    if !((*(*codec).internal).rav1eContext).is_null() {
        rav1e_context_unref((*(*codec).internal).rav1eContext);
        (*(*codec).internal).rav1eContext = 0 as *mut RaContext;
    }
    avifFree((*codec).internal as *mut core::ffi::c_void);
}
unsafe extern "C" fn rav1eSupports400() -> avifBool {
    let mut rav1eVersionString: *const core::ffi::c_char = rav1e_version_short();
    let mut majorVersion: core::ffi::c_int = atoi(rav1eVersionString);
    if majorVersion > 0 as core::ffi::c_int {
        return AVIF_TRUE;
    }
    let mut minorVersionString: *const core::ffi::c_char = strchr(
        rav1eVersionString,
        '.' as i32,
    );
    if minorVersionString.is_null() {
        return AVIF_FALSE;
    }
    minorVersionString = minorVersionString.offset(1);
    if *minorVersionString == 0 {
        return AVIF_FALSE;
    }
    let mut minorVersion: core::ffi::c_int = atoi(minorVersionString);
    return (minorVersion >= 4 as core::ffi::c_int) as core::ffi::c_int;
}
unsafe extern "C" fn rav1eQualityToQuantizer(
    mut quality: core::ffi::c_int,
) -> core::ffi::c_int {
    let quantizer: core::ffi::c_int = ((100 as core::ffi::c_int - quality)
        * 255 as core::ffi::c_int + 50 as core::ffi::c_int) / 100 as core::ffi::c_int;
    return quantizer;
}
unsafe extern "C" fn rav1eCodecEncodeImage(
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
    let mut byteWidth: core::ffi::c_int = 0;
    let mut frameType: RaFrameTypeOverride = RA_FRAME_TYPE_OVERRIDE_NO;
    let mut encoderStatus: RaEncoderStatus = RA_ENCODER_STATUS_SUCCESS;
    let mut pkt: *mut RaPacket = 0 as *mut RaPacket;
    let mut current_block: u64;
    if encoderChanges != 0 {
        return AVIF_RESULT_NOT_IMPLEMENTED;
    }
    if ((*(*codec).internal).rav1eContext).is_null() {
        (*(*codec).internal).encodeWidth = (*image).width;
        (*(*codec).internal).encodeHeight = (*image).height;
    } else if (*(*codec).internal).encodeWidth != (*image).width
        || (*(*codec).internal).encodeHeight != (*image).height
    {
        return AVIF_RESULT_NOT_IMPLEMENTED
    }
    if (*encoder).width != 0 || (*encoder).height != 0
        || (*encoder).extraLayerCount > 0 as uint32_t
    {
        return AVIF_RESULT_NOT_IMPLEMENTED;
    }
    let mut result: avifResult = AVIF_RESULT_UNKNOWN_ERROR;
    let mut rav1eConfig: *mut RaConfig = 0 as *mut RaConfig;
    let mut rav1eFrame: *mut RaFrame = 0 as *mut RaFrame;
    if ((*(*codec).internal).rav1eContext).is_null() {
        let supports400: avifBool = rav1eSupports400() as avifBool;
        let mut rav1eRange: RaPixelRange = RA_PIXEL_RANGE_LIMITED;
        if alpha != 0 {
            rav1eRange = RA_PIXEL_RANGE_FULL;
            (*(*codec).internal).chromaSampling = (if supports400 != 0 {
                RA_CHROMA_SAMPLING_CS400 as core::ffi::c_int
            } else {
                RA_CHROMA_SAMPLING_CS420 as core::ffi::c_int
            }) as RaChromaSampling;
            (*(*codec).internal).yShift = 1 as core::ffi::c_int;
        } else {
            rav1eRange = (if (*image).yuvRange as core::ffi::c_uint
                == AVIF_RANGE_FULL as core::ffi::c_int as core::ffi::c_uint
            {
                RA_PIXEL_RANGE_FULL as core::ffi::c_int
            } else {
                RA_PIXEL_RANGE_LIMITED as core::ffi::c_int
            }) as RaPixelRange;
            (*(*codec).internal).yShift = 0 as core::ffi::c_int;
            match (*image).yuvFormat as core::ffi::c_uint {
                1 => {
                    (*(*codec).internal).chromaSampling = RA_CHROMA_SAMPLING_CS444;
                }
                2 => {
                    (*(*codec).internal).chromaSampling = RA_CHROMA_SAMPLING_CS422;
                }
                3 => {
                    (*(*codec).internal).chromaSampling = RA_CHROMA_SAMPLING_CS420;
                    (*(*codec).internal).yShift = 1 as core::ffi::c_int;
                }
                4 => {
                    (*(*codec).internal).chromaSampling = (if supports400 != 0 {
                        RA_CHROMA_SAMPLING_CS400 as core::ffi::c_int
                    } else {
                        RA_CHROMA_SAMPLING_CS420 as core::ffi::c_int
                    }) as RaChromaSampling;
                    (*(*codec).internal).yShift = 1 as core::ffi::c_int;
                }
                0 | 5 | _ => return AVIF_RESULT_UNKNOWN_ERROR,
            }
        }
        rav1eConfig = rav1e_config_default();
        if rav1e_config_set_pixel_format(
            rav1eConfig,
            (*image).depth as uint8_t,
            (*(*codec).internal).chromaSampling,
            (*image).yuvChromaSamplePosition as RaChromaSamplePosition,
            rav1eRange,
        ) < 0 as core::ffi::c_int
        {
            current_block = 0;
        } else {
            if addImageFlags & AVIF_ADD_IMAGE_FLAG_SINGLE as core::ffi::c_int as uint32_t
                != 0
            {
                if rav1e_config_parse(
                    rav1eConfig,
                    b"still_picture\0" as *const u8 as *const core::ffi::c_char,
                    b"true\0" as *const u8 as *const core::ffi::c_char,
                ) == -(1 as core::ffi::c_int)
                {
                    current_block = 0;
                } else {
                    current_block = 1;
                }
            } else {
                current_block = 1;
            }
            match current_block {
                0 => {}
                _ => {
                    if rav1e_config_parse_int(
                        rav1eConfig,
                        b"width\0" as *const u8 as *const core::ffi::c_char,
                        (*image).width as core::ffi::c_int,
                    ) == -(1 as core::ffi::c_int)
                    {
                        current_block = 0;
                    } else if rav1e_config_parse_int(
                        rav1eConfig,
                        b"height\0" as *const u8 as *const core::ffi::c_char,
                        (*image).height as core::ffi::c_int,
                    ) == -(1 as core::ffi::c_int)
                    {
                        current_block = 0;
                    } else if rav1e_config_parse_int(
                        rav1eConfig,
                        b"threads\0" as *const u8 as *const core::ffi::c_char,
                        (*encoder).maxThreads,
                    ) == -(1 as core::ffi::c_int)
                    {
                        current_block = 0;
                    } else {
                        let mut minQuantizer: core::ffi::c_int = if (*encoder)
                            .minQuantizer < 0 as core::ffi::c_int
                        {
                            0 as core::ffi::c_int
                        } else if (63 as core::ffi::c_int) < (*encoder).minQuantizer {
                            63 as core::ffi::c_int
                        } else {
                            (*encoder).minQuantizer
                        };
                        if alpha != 0 {
                            minQuantizer = if (*encoder).minQuantizerAlpha
                                < 0 as core::ffi::c_int
                            {
                                0 as core::ffi::c_int
                            } else if (63 as core::ffi::c_int)
                                < (*encoder).minQuantizerAlpha
                            {
                                63 as core::ffi::c_int
                            } else {
                                (*encoder).minQuantizerAlpha
                            };
                        }
                        minQuantizer = minQuantizer * 255 as core::ffi::c_int
                            / 63 as core::ffi::c_int;
                        let quantizer: core::ffi::c_int = rav1eQualityToQuantizer(
                            quality,
                        ) as core::ffi::c_int;
                        if rav1e_config_parse_int(
                            rav1eConfig,
                            b"min_quantizer\0" as *const u8 as *const core::ffi::c_char,
                            minQuantizer,
                        ) == -(1 as core::ffi::c_int)
                        {
                            current_block = 0;
                        } else if rav1e_config_parse_int(
                            rav1eConfig,
                            b"quantizer\0" as *const u8 as *const core::ffi::c_char,
                            quantizer,
                        ) == -(1 as core::ffi::c_int)
                        {
                            current_block = 0;
                        } else {
                            if tileRowsLog2 != 0 as core::ffi::c_int {
                                if rav1e_config_parse_int(
                                    rav1eConfig,
                                    b"tile_rows\0" as *const u8 as *const core::ffi::c_char,
                                    (1 as core::ffi::c_int) << tileRowsLog2,
                                ) == -(1 as core::ffi::c_int)
                                {
                                    current_block = 0;
                                } else {
                                    current_block = 2;
                                }
                            } else {
                                current_block = 2;
                            }
                            match current_block {
                                0 => {}
                                _ => {
                                    if tileColsLog2 != 0 as core::ffi::c_int {
                                        if rav1e_config_parse_int(
                                            rav1eConfig,
                                            b"tile_cols\0" as *const u8 as *const core::ffi::c_char,
                                            (1 as core::ffi::c_int) << tileColsLog2,
                                        ) == -(1 as core::ffi::c_int)
                                        {
                                            current_block = 0;
                                        } else {
                                            current_block = 3;
                                        }
                                    } else {
                                        current_block = 3;
                                    }
                                    match current_block {
                                        0 => {}
                                        _ => {
                                            if (*encoder).speed != AVIF_SPEED_DEFAULT {
                                                let mut speed: core::ffi::c_int = if (*encoder).speed
                                                    < 0 as core::ffi::c_int
                                                {
                                                    0 as core::ffi::c_int
                                                } else if (10 as core::ffi::c_int) < (*encoder).speed {
                                                    10 as core::ffi::c_int
                                                } else {
                                                    (*encoder).speed
                                                };
                                                if rav1e_config_parse_int(
                                                    rav1eConfig,
                                                    b"speed\0" as *const u8 as *const core::ffi::c_char,
                                                    speed,
                                                ) == -(1 as core::ffi::c_int)
                                                {
                                                    current_block = 0;
                                                } else {
                                                    current_block = 4;
                                                }
                                            } else {
                                                current_block = 4;
                                            }
                                            match current_block {
                                                0 => {}
                                                _ => {
                                                    if (*encoder).keyframeInterval > 0 as core::ffi::c_int {
                                                        if rav1e_config_parse_int(
                                                            rav1eConfig,
                                                            b"key_frame_interval\0" as *const u8
                                                                as *const core::ffi::c_char,
                                                            (*encoder).keyframeInterval,
                                                        ) == -(1 as core::ffi::c_int)
                                                        {
                                                            current_block = 0;
                                                        } else {
                                                            current_block = 5;
                                                        }
                                                    } else {
                                                        current_block = 5;
                                                    }
                                                    match current_block {
                                                        0 => {}
                                                        _ => {
                                                            let mut i: uint32_t = 0 as uint32_t;
                                                            loop {
                                                                if !(i < (*(*codec).csOptions).count) {
                                                                    current_block = 6;
                                                                    break;
                                                                }
                                                                let mut entry: *const avifCodecSpecificOption = &mut *((*(*codec)
                                                                    .csOptions)
                                                                    .entries)
                                                                    .offset(i as isize) as *mut avifCodecSpecificOption;
                                                                if rav1e_config_parse(
                                                                    rav1eConfig,
                                                                    (*entry).key,
                                                                    (*entry).value,
                                                                ) < 0 as core::ffi::c_int
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
                                                                    rav1e_config_set_color_description(
                                                                        rav1eConfig,
                                                                        (*image).matrixCoefficients as RaMatrixCoefficients,
                                                                        (*image).colorPrimaries as RaColorPrimaries,
                                                                        (*image).transferCharacteristics
                                                                            as RaTransferCharacteristics,
                                                                    );
                                                                    (*(*codec).internal).rav1eContext = rav1e_context_new(
                                                                        rav1eConfig,
                                                                    );
                                                                    if ((*(*codec).internal).rav1eContext).is_null() {
                                                                        current_block = 0;
                                                                    } else {
                                                                        current_block = 7;
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    } else {
        current_block = 7;
    }
    match current_block {
        7 => {
            rav1eFrame = rav1e_frame_new((*(*codec).internal).rav1eContext);
            byteWidth = if (*image).depth > 8 as uint32_t {
                2 as core::ffi::c_int
            } else {
                1 as core::ffi::c_int
            };
            if alpha != 0 {
                rav1e_frame_fill_plane(
                    rav1eFrame,
                    0 as core::ffi::c_int,
                    (*image).alphaPlane,
                    ((*image).alphaRowBytes as size_t)
                        .wrapping_mul((*image).height as size_t),
                    (*image).alphaRowBytes as ptrdiff_t,
                    byteWidth,
                );
            } else {
                rav1e_frame_fill_plane(
                    rav1eFrame,
                    0 as core::ffi::c_int,
                    (*image).yuvPlanes[0 as core::ffi::c_int as usize],
                    ((*image).yuvRowBytes[0 as core::ffi::c_int as usize] as size_t)
                        .wrapping_mul((*image).height as size_t),
                    (*image).yuvRowBytes[0 as core::ffi::c_int as usize] as ptrdiff_t,
                    byteWidth,
                );
                if (*image).yuvFormat as core::ffi::c_uint
                    != AVIF_PIXEL_FORMAT_YUV400 as core::ffi::c_int as core::ffi::c_uint
                {
                    let mut uvHeight: uint32_t = ((*image).height)
                        .wrapping_add((*(*codec).internal).yShift as uint32_t)
                        >> (*(*codec).internal).yShift;
                    rav1e_frame_fill_plane(
                        rav1eFrame,
                        1 as core::ffi::c_int,
                        (*image).yuvPlanes[1 as core::ffi::c_int as usize],
                        ((*image).yuvRowBytes[1 as core::ffi::c_int as usize] as size_t)
                            .wrapping_mul(uvHeight as size_t),
                        (*image).yuvRowBytes[1 as core::ffi::c_int as usize]
                            as ptrdiff_t,
                        byteWidth,
                    );
                    rav1e_frame_fill_plane(
                        rav1eFrame,
                        2 as core::ffi::c_int,
                        (*image).yuvPlanes[2 as core::ffi::c_int as usize],
                        ((*image).yuvRowBytes[2 as core::ffi::c_int as usize] as size_t)
                            .wrapping_mul(uvHeight as size_t),
                        (*image).yuvRowBytes[2 as core::ffi::c_int as usize]
                            as ptrdiff_t,
                        byteWidth,
                    );
                }
            }
            frameType = RA_FRAME_TYPE_OVERRIDE_NO;
            if addImageFlags
                & AVIF_ADD_IMAGE_FLAG_FORCE_KEYFRAME as core::ffi::c_int as uint32_t != 0
            {
                frameType = RA_FRAME_TYPE_OVERRIDE_KEY;
            }
            rav1e_frame_set_type(rav1eFrame, frameType);
            encoderStatus = rav1e_send_frame(
                (*(*codec).internal).rav1eContext,
                rav1eFrame,
            );
            if !(encoderStatus as core::ffi::c_int
                != RA_ENCODER_STATUS_SUCCESS as core::ffi::c_int)
            {
                pkt = 0 as *mut RaPacket;
                loop {
                    encoderStatus = rav1e_receive_packet(
                        (*(*codec).internal).rav1eContext,
                        &mut pkt,
                    );
                    if encoderStatus as core::ffi::c_int
                        == RA_ENCODER_STATUS_ENCODED as core::ffi::c_int
                    {
                        continue;
                    }
                    if encoderStatus as core::ffi::c_int
                        != RA_ENCODER_STATUS_SUCCESS as core::ffi::c_int
                        && encoderStatus as core::ffi::c_int
                            != RA_ENCODER_STATUS_NEED_MORE_DATA as core::ffi::c_int
                    {
                        current_block = 0;
                        break;
                    }
                    if pkt.is_null() {
                        current_block = 8;
                        break;
                    }
                    if !((*pkt).data).is_null() && (*pkt).len > 0 as size_t {
                        result = avifCodecEncodeOutputAddSample(
                            output,
                            (*pkt).data,
                            (*pkt).len,
                            ((*pkt).frame_type as core::ffi::c_uint
                                == RA_FRAME_TYPE_KEY as core::ffi::c_int
                                    as core::ffi::c_uint) as core::ffi::c_int,
                        );
                        if result as core::ffi::c_uint
                            != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
                        {
                            current_block = 0;
                            break;
                        }
                    }
                    rav1e_packet_unref(pkt);
                    pkt = 0 as *mut RaPacket;
                }
                match current_block {
                    0 => {}
                    _ => {
                        result = AVIF_RESULT_OK;
                    }
                }
            }
        }
        _ => {}
    }
    if !rav1eFrame.is_null() {
        rav1e_frame_unref(rav1eFrame);
        rav1eFrame = 0 as *mut RaFrame;
    }
    if !rav1eConfig.is_null() {
        rav1e_config_unref(rav1eConfig);
        rav1eConfig = 0 as *mut RaConfig;
    }
    return result;
}
unsafe extern "C" fn rav1eCodecEncodeFinish(
    mut codec: *mut avifCodec,
    mut output: *mut avifCodecEncodeOutput,
) -> avifBool {
    loop {
        let mut encoderStatus: RaEncoderStatus = rav1e_send_frame(
            (*(*codec).internal).rav1eContext,
            0 as *mut RaFrame,
        );
        if encoderStatus as core::ffi::c_int
            != RA_ENCODER_STATUS_SUCCESS as core::ffi::c_int
        {
            return AVIF_FALSE;
        }
        let mut gotPacket: avifBool = AVIF_FALSE;
        let mut pkt: *mut RaPacket = 0 as *mut RaPacket;
        loop {
            encoderStatus = rav1e_receive_packet(
                (*(*codec).internal).rav1eContext,
                &mut pkt,
            );
            if encoderStatus as core::ffi::c_int
                == RA_ENCODER_STATUS_ENCODED as core::ffi::c_int
            {
                continue;
            }
            if encoderStatus as core::ffi::c_int
                != RA_ENCODER_STATUS_SUCCESS as core::ffi::c_int
                && encoderStatus as core::ffi::c_int
                    != RA_ENCODER_STATUS_LIMIT_REACHED as core::ffi::c_int
            {
                return AVIF_FALSE;
            }
            if pkt.is_null() {
                break;
            }
            gotPacket = AVIF_TRUE as avifBool;
            if !((*pkt).data).is_null() && (*pkt).len > 0 as size_t {
                if avifCodecEncodeOutputAddSample(
                    output,
                    (*pkt).data,
                    (*pkt).len,
                    ((*pkt).frame_type as core::ffi::c_uint
                        == RA_FRAME_TYPE_KEY as core::ffi::c_int as core::ffi::c_uint)
                        as core::ffi::c_int,
                ) as core::ffi::c_uint
                    != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
                {
                    return AVIF_FALSE;
                }
            }
            rav1e_packet_unref(pkt);
            pkt = 0 as *mut RaPacket;
        }
        if gotPacket == 0 {
            break;
        }
    }
    return AVIF_TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn avifCodecVersionRav1e() -> *const core::ffi::c_char {
    return rav1e_version_full();
}
#[no_mangle]
pub unsafe extern "C" fn avifCodecCreateRav1e() -> *mut avifCodec {
    let mut codec: *mut avifCodec = avifCalloc(
        1 as size_t,
        ::core::mem::size_of::<avifCodec>() as size_t,
    ) as *mut avifCodec;
    if codec.is_null() {
        return 0 as *mut avifCodec;
    }
    (*codec).encodeImage = Some(
        rav1eCodecEncodeImage
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
        rav1eCodecEncodeFinish
            as unsafe extern "C" fn(
                *mut avifCodec,
                *mut avifCodecEncodeOutput,
            ) -> avifBool,
    ) as avifCodecEncodeFinishFunc;
    (*codec).destroyInternal = Some(
        rav1eCodecDestroyInternal as unsafe extern "C" fn(*mut avifCodec) -> (),
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
