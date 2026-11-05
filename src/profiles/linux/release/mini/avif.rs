// Copyright 2019 Joe Drago. All rights reserved.
// SPDX-License-Identifier: BSD-2-Clause

// Mechanically translated from upstream/libavif/src/avif.c.
// C ABI, ownership, pointer validity, and bounds follow the original headers.
extern "C" {
    fn avifCodecVersionSvt() -> *const core::ffi::c_char;
    fn avifCodecCreateSvt() -> *mut avifCodec;
    fn avifCodecVersionRav1e() -> *const core::ffi::c_char;
    fn avifCodecCreateRav1e() -> *mut avifCodec;
    fn avifCodecVersionGav1() -> *const core::ffi::c_char;
    fn avifCodecCreateGav1() -> *mut avifCodec;
    pub type avifEncoderData;
    pub type avifCodecInternal;
    fn avifAlloc(size: size_t) -> *mut core::ffi::c_void;
    fn avifFree(p: *mut core::ffi::c_void);
    fn avifRWDataSet(
        raw: *mut avifRWData,
        data: *const uint8_t,
        len: size_t,
    ) -> avifResult;
    fn avifRWDataFree(raw: *mut avifRWData);
    fn avifDiagnosticsClearError(diag: *mut avifDiagnostics);
    fn avifCalloc(count: size_t, size: size_t) -> *mut core::ffi::c_void;
    fn avifArrayCreate(
        arrayStruct: *mut core::ffi::c_void,
        elementSize: uint32_t,
        initialCapacity: uint32_t,
    ) -> avifBool;
    fn avifArrayPush(arrayStruct: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn avifArrayDestroy(arrayStruct: *mut core::ffi::c_void);
    fn avifFractionAdd(
        a: avifFraction,
        b: avifFraction,
        result: *mut avifFraction,
    ) -> avifBool;
    fn avifFractionSub(
        a: avifFraction,
        b: avifFraction,
        result: *mut avifFraction,
    ) -> avifBool;
    fn avifIsKnownPropertyType(boxtype: *const uint8_t) -> avifBool;
    fn avifIsValidUUID(uuid: *const uint8_t) -> avifBool;
    fn avifCodecCreateAOM() -> *mut avifCodec;
    fn avifCodecVersionAOM() -> *const core::ffi::c_char;
    fn avifCodecCreateDav1d() -> *mut avifCodec;
    fn avifCodecVersionDav1d() -> *const core::ffi::c_char;
    fn avifDiagnosticsPrintf(
        diag: *mut avifDiagnostics,
        format: *const core::ffi::c_char,
        ...
    );
    fn memcpy(
        __dest: *mut core::ffi::c_void,
        __src: *const core::ffi::c_void,
        __n: size_t,
    ) -> *mut core::ffi::c_void;
    fn memmove(
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
    fn strlen(__s: *const core::ffi::c_char) -> size_t;
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
pub type avifChannelIndex = core::ffi::c_uint;
pub const AVIF_CHAN_A: avifChannelIndex = 3;
pub const AVIF_CHAN_V: avifChannelIndex = 2;
pub const AVIF_CHAN_U: avifChannelIndex = 1;
pub const AVIF_CHAN_Y: avifChannelIndex = 0;
pub type versionFunc = Option<unsafe extern "C" fn() -> *const core::ffi::c_char>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AvailableCodec {
    pub choice: avifCodecChoice,
    pub type_0: avifCodecType,
    pub name: *const core::ffi::c_char,
    pub version: versionFunc,
    pub create: avifCodecCreateFunc,
    pub flags: uint32_t,
}
pub type avifCodecCreateFunc = Option<unsafe extern "C" fn() -> *mut avifCodec>;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct avifCodecEncodeOutput {
    pub samples: avifEncodeSampleArray,
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
pub struct avifEncodeSample {
    pub data: avifRWData,
    pub sync: avifBool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct avifRWData {
    pub data: *mut uint8_t,
    pub size: size_t,
}
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
pub type avifAddImageFlags = uint32_t;
pub type avifEncoderChanges = core::ffi::c_int;
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
pub struct avifContentLightLevelInformationBox {
    pub maxCLL: uint16_t,
    pub maxPALL: uint16_t,
}
pub type avifRange = core::ffi::c_uint;
pub const AVIF_RANGE_FULL: avifRange = 1;
pub const AVIF_RANGE_LIMITED: avifRange = 0;
pub type avifMatrixCoefficients = uint16_t;
pub type avifTransferCharacteristics = uint16_t;
pub type avifColorPrimaries = uint16_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct avifUnsignedFraction {
    pub n: uint32_t,
    pub d: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct avifSignedFraction {
    pub n: int32_t,
    pub d: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct avifImageItemProperty {
    pub boxtype: [uint8_t; 4],
    pub usertype: [uint8_t; 16],
    pub boxPayload: avifRWData,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct avifImageMirror {
    pub axis: uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct avifImageRotation {
    pub angle: uint8_t,
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
pub struct avifPixelAspectRatioBox {
    pub hSpacing: uint32_t,
    pub vSpacing: uint32_t,
}
pub type avifTransformFlags = uint32_t;
pub type avifChromaSamplePosition = core::ffi::c_uint;
pub const AVIF_CHROMA_SAMPLE_POSITION_RESERVED: avifChromaSamplePosition = 3;
pub const AVIF_CHROMA_SAMPLE_POSITION_COLOCATED: avifChromaSamplePosition = 2;
pub const AVIF_CHROMA_SAMPLE_POSITION_VERTICAL: avifChromaSamplePosition = 1;
pub const AVIF_CHROMA_SAMPLE_POSITION_UNKNOWN: avifChromaSamplePosition = 0;
pub type avifPixelFormat = core::ffi::c_uint;
pub const AVIF_PIXEL_FORMAT_COUNT: avifPixelFormat = 5;
pub const AVIF_PIXEL_FORMAT_YUV400: avifPixelFormat = 4;
pub const AVIF_PIXEL_FORMAT_YUV420: avifPixelFormat = 3;
pub const AVIF_PIXEL_FORMAT_YUV422: avifPixelFormat = 2;
pub const AVIF_PIXEL_FORMAT_YUV444: avifPixelFormat = 1;
pub const AVIF_PIXEL_FORMAT_NONE: avifPixelFormat = 0;
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
pub type avifSampleTransformRecipe = core::ffi::c_uint;
pub const AVIF_SAMPLE_TRANSFORM_BIT_DEPTH_EXTENSION_12B_8B_OVERLAP_4B: avifSampleTransformRecipe = 3;
pub const AVIF_SAMPLE_TRANSFORM_BIT_DEPTH_EXTENSION_12B_4B: avifSampleTransformRecipe = 2;
pub const AVIF_SAMPLE_TRANSFORM_BIT_DEPTH_EXTENSION_8B_8B: avifSampleTransformRecipe = 1;
pub const AVIF_SAMPLE_TRANSFORM_NONE: avifSampleTransformRecipe = 0;
pub type avifHeaderFormatFlags = core::ffi::c_int;
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
pub struct avifDiagnostics {
    pub error: [core::ffi::c_char; 256],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct avifIOStats {
    pub colorOBUSize: size_t,
    pub alphaOBUSize: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct avifScalingMode {
    pub horizontal: avifFraction,
    pub vertical: avifFraction,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct avifFraction {
    pub n: int32_t,
    pub d: int32_t,
}
pub type avifCodecChoice = core::ffi::c_uint;
pub const AVIF_CODEC_CHOICE_AVM: avifCodecChoice = 6;
pub const AVIF_CODEC_CHOICE_SVT: avifCodecChoice = 5;
pub const AVIF_CODEC_CHOICE_RAV1E: avifCodecChoice = 4;
pub const AVIF_CODEC_CHOICE_LIBGAV1: avifCodecChoice = 3;
pub const AVIF_CODEC_CHOICE_DAV1D: avifCodecChoice = 2;
pub const AVIF_CODEC_CHOICE_AOM: avifCodecChoice = 1;
pub const AVIF_CODEC_CHOICE_AUTO: avifCodecChoice = 0;
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
pub struct avifROData {
    pub data: *const uint8_t,
    pub size: size_t,
}
pub type avifCodecType = core::ffi::c_uint;
pub const AVIF_CODEC_TYPE_AV1: avifCodecType = 1;
pub const AVIF_CODEC_TYPE_UNKNOWN: avifCodecType = 0;
pub const AVIF_CODEC_FLAG_CAN_ENCODE: avifCodecFlag = 2;
pub const AVIF_CODEC_FLAG_CAN_DECODE: avifCodecFlag = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct avifPixelFormatInfo {
    pub monochrome: avifBool,
    pub chromaShiftX: core::ffi::c_int,
    pub chromaShiftY: core::ffi::c_int,
}
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct avifCropRect {
    pub x: uint32_t,
    pub y: uint32_t,
    pub width: uint32_t,
    pub height: uint32_t,
}
pub type avifRGBFormat = core::ffi::c_uint;
pub const AVIF_RGB_FORMAT_COUNT: avifRGBFormat = 10;
pub const AVIF_RGB_FORMAT_AGRAY: avifRGBFormat = 9;
pub const AVIF_RGB_FORMAT_GRAYA: avifRGBFormat = 8;
pub const AVIF_RGB_FORMAT_GRAY: avifRGBFormat = 7;
pub const AVIF_RGB_FORMAT_RGB_565: avifRGBFormat = 6;
pub const AVIF_RGB_FORMAT_ABGR: avifRGBFormat = 5;
pub const AVIF_RGB_FORMAT_BGRA: avifRGBFormat = 4;
pub const AVIF_RGB_FORMAT_BGR: avifRGBFormat = 3;
pub const AVIF_RGB_FORMAT_ARGB: avifRGBFormat = 2;
pub const AVIF_RGB_FORMAT_RGBA: avifRGBFormat = 1;
pub const AVIF_RGB_FORMAT_RGB: avifRGBFormat = 0;
pub type avifChromaUpsampling = core::ffi::c_uint;
pub const AVIF_CHROMA_UPSAMPLING_BILINEAR: avifChromaUpsampling = 4;
pub const AVIF_CHROMA_UPSAMPLING_NEAREST: avifChromaUpsampling = 3;
pub const AVIF_CHROMA_UPSAMPLING_BEST_QUALITY: avifChromaUpsampling = 2;
pub const AVIF_CHROMA_UPSAMPLING_FASTEST: avifChromaUpsampling = 1;
pub const AVIF_CHROMA_UPSAMPLING_AUTOMATIC: avifChromaUpsampling = 0;
pub type avifChromaDownsampling = core::ffi::c_uint;
pub const AVIF_CHROMA_DOWNSAMPLING_SHARP_YUV: avifChromaDownsampling = 4;
pub const AVIF_CHROMA_DOWNSAMPLING_AVERAGE: avifChromaDownsampling = 3;
pub const AVIF_CHROMA_DOWNSAMPLING_BEST_QUALITY: avifChromaDownsampling = 2;
pub const AVIF_CHROMA_DOWNSAMPLING_FASTEST: avifChromaDownsampling = 1;
pub const AVIF_CHROMA_DOWNSAMPLING_AUTOMATIC: avifChromaDownsampling = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct avifRGBImage {
    pub width: uint32_t,
    pub height: uint32_t,
    pub depth: uint32_t,
    pub format: avifRGBFormat,
    pub chromaUpsampling: avifChromaUpsampling,
    pub chromaDownsampling: avifChromaDownsampling,
    pub avoidLibYUV: avifBool,
    pub ignoreAlpha: avifBool,
    pub alphaPremultiplied: avifBool,
    pub isFloat: avifBool,
    pub maxThreads: core::ffi::c_int,
    pub pixels: *mut uint8_t,
    pub rowBytes: uint32_t,
}
pub type avifCodecFlag = core::ffi::c_uint;
pub type avifCodecFlags = uint32_t;
pub type avifProgressiveState = core::ffi::c_uint;
pub const AVIF_PROGRESSIVE_STATE_ACTIVE: avifProgressiveState = 2;
pub const AVIF_PROGRESSIVE_STATE_AVAILABLE: avifProgressiveState = 1;
pub const AVIF_PROGRESSIVE_STATE_UNAVAILABLE: avifProgressiveState = 0;
pub type avifItemCategory = core::ffi::c_uint;
pub const AVIF_ITEM_CATEGORY_COUNT: avifItemCategory = 8;
pub const AVIF_ITEM_SAMPLE_TRANSFORM_INPUT_1_ALPHA: avifItemCategory = 7;
pub const AVIF_ITEM_SAMPLE_TRANSFORM_INPUT_0_ALPHA: avifItemCategory = 6;
pub const AVIF_ITEM_SAMPLE_TRANSFORM_INPUT_1_COLOR: avifItemCategory = 5;
pub const AVIF_ITEM_SAMPLE_TRANSFORM_INPUT_0_COLOR: avifItemCategory = 4;
pub const AVIF_ITEM_SAMPLE_TRANSFORM: avifItemCategory = 3;
pub const AVIF_ITEM_GAIN_MAP: avifItemCategory = 2;
pub const AVIF_ITEM_ALPHA: avifItemCategory = 1;
pub const AVIF_ITEM_COLOR: avifItemCategory = 0;
pub const NULL: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
pub const INT32_MIN: core::ffi::c_int = -(2147483647 as core::ffi::c_int)
    - 1 as core::ffi::c_int;
pub const INT32_MAX: core::ffi::c_int = 2147483647 as core::ffi::c_int;
pub const UINT32_MAX: core::ffi::c_uint = 4294967295 as core::ffi::c_uint;
pub const PTRDIFF_MAX: core::ffi::c_long = 9223372036854775807 as core::ffi::c_long;
pub const AVIF_TRUE: core::ffi::c_int = 1 as core::ffi::c_int;
pub const AVIF_FALSE: core::ffi::c_int = 0 as core::ffi::c_int;
pub const AVIF_VERSION_STRING: [core::ffi::c_char; 6] = unsafe {
    ::core::mem::transmute::<[u8; 6], [core::ffi::c_char; 6]>(*b"1.4.2\0")
};
#[no_mangle]
pub unsafe extern "C" fn avifVersion() -> *const core::ffi::c_char {
    return AVIF_VERSION_STRING.as_ptr();
}
#[no_mangle]
pub unsafe extern "C" fn avifPixelFormatToString(
    mut format: avifPixelFormat,
) -> *const core::ffi::c_char {
    match format as core::ffi::c_uint {
        1 => return b"YUV444\0" as *const u8 as *const core::ffi::c_char,
        3 => return b"YUV420\0" as *const u8 as *const core::ffi::c_char,
        2 => return b"YUV422\0" as *const u8 as *const core::ffi::c_char,
        4 => return b"YUV400\0" as *const u8 as *const core::ffi::c_char,
        0 | 5 | _ => {}
    }
    return b"Unknown\0" as *const u8 as *const core::ffi::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn avifGetPixelFormatInfo(
    mut format: avifPixelFormat,
    mut info: *mut avifPixelFormatInfo,
) {
    memset(
        info as *mut core::ffi::c_void,
        0 as core::ffi::c_int,
        ::core::mem::size_of::<avifPixelFormatInfo>() as size_t,
    );
    match format as core::ffi::c_uint {
        1 => {
            (*info).chromaShiftX = 0 as core::ffi::c_int;
            (*info).chromaShiftY = 0 as core::ffi::c_int;
        }
        2 => {
            (*info).chromaShiftX = 1 as core::ffi::c_int;
            (*info).chromaShiftY = 0 as core::ffi::c_int;
        }
        3 => {
            (*info).chromaShiftX = 1 as core::ffi::c_int;
            (*info).chromaShiftY = 1 as core::ffi::c_int;
        }
        4 => {
            (*info).monochrome = AVIF_TRUE as avifBool;
            (*info).chromaShiftX = 1 as core::ffi::c_int;
            (*info).chromaShiftY = 1 as core::ffi::c_int;
        }
        0 | 5 | _ => {}
    };
}
#[no_mangle]
pub unsafe extern "C" fn avifResultToString(
    mut result: avifResult,
) -> *const core::ffi::c_char {
    match result as core::ffi::c_uint {
        0 => return b"OK\0" as *const u8 as *const core::ffi::c_char,
        2 => return b"Invalid ftyp\0" as *const u8 as *const core::ffi::c_char,
        3 => return b"No content\0" as *const u8 as *const core::ffi::c_char,
        4 => return b"No YUV format selected\0" as *const u8 as *const core::ffi::c_char,
        5 => return b"Reformat failed\0" as *const u8 as *const core::ffi::c_char,
        6 => return b"Unsupported depth\0" as *const u8 as *const core::ffi::c_char,
        7 => {
            return b"Encoding of color planes failed\0" as *const u8
                as *const core::ffi::c_char;
        }
        8 => {
            return b"Encoding of alpha plane failed\0" as *const u8
                as *const core::ffi::c_char;
        }
        9 => return b"BMFF parsing failed\0" as *const u8 as *const core::ffi::c_char,
        10 => {
            return b"Missing or empty image item\0" as *const u8
                as *const core::ffi::c_char;
        }
        11 => {
            return b"Decoding of color planes failed\0" as *const u8
                as *const core::ffi::c_char;
        }
        12 => {
            return b"Decoding of alpha plane failed\0" as *const u8
                as *const core::ffi::c_char;
        }
        13 => {
            return b"Color and alpha planes size mismatch\0" as *const u8
                as *const core::ffi::c_char;
        }
        14 => {
            return b"Plane sizes don't match ispe values\0" as *const u8
                as *const core::ffi::c_char;
        }
        15 => return b"No codec available\0" as *const u8 as *const core::ffi::c_char,
        16 => return b"No images remaining\0" as *const u8 as *const core::ffi::c_char,
        17 => return b"Invalid Exif payload\0" as *const u8 as *const core::ffi::c_char,
        18 => return b"Invalid image grid\0" as *const u8 as *const core::ffi::c_char,
        19 => {
            return b"Invalid codec-specific option\0" as *const u8
                as *const core::ffi::c_char;
        }
        20 => return b"Truncated data\0" as *const u8 as *const core::ffi::c_char,
        21 => return b"IO not set\0" as *const u8 as *const core::ffi::c_char,
        22 => return b"IO Error\0" as *const u8 as *const core::ffi::c_char,
        23 => return b"Waiting on IO\0" as *const u8 as *const core::ffi::c_char,
        24 => return b"Invalid argument\0" as *const u8 as *const core::ffi::c_char,
        25 => return b"Not implemented\0" as *const u8 as *const core::ffi::c_char,
        26 => return b"Out of memory\0" as *const u8 as *const core::ffi::c_char,
        27 => {
            return b"Cannot change some setting during encoding\0" as *const u8
                as *const core::ffi::c_char;
        }
        28 => {
            return b"The image is incompatible with already encoded images\0"
                as *const u8 as *const core::ffi::c_char;
        }
        29 => return b"Internal error\0" as *const u8 as *const core::ffi::c_char,
        30 => {
            return b"Encoding of gain map planes failed\0" as *const u8
                as *const core::ffi::c_char;
        }
        31 => {
            return b"Decoding of gain map planes failed\0" as *const u8
                as *const core::ffi::c_char;
        }
        32 => {
            return b"Invalid tone mapped image item\0" as *const u8
                as *const core::ffi::c_char;
        }
        33 => {
            return b"Encoding of sample transformed image failed\0" as *const u8
                as *const core::ffi::c_char;
        }
        34 => {
            return b"Decoding of sample transformed image failed\0" as *const u8
                as *const core::ffi::c_char;
        }
        1 | _ => {}
    }
    return b"Unknown Error\0" as *const u8 as *const core::ffi::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn avifProgressiveStateToString(
    mut progressiveState: avifProgressiveState,
) -> *const core::ffi::c_char {
    match progressiveState as core::ffi::c_uint {
        0 => return b"Unavailable\0" as *const u8 as *const core::ffi::c_char,
        1 => return b"Available\0" as *const u8 as *const core::ffi::c_char,
        2 => return b"Active\0" as *const u8 as *const core::ffi::c_char,
        _ => {}
    }
    return b"Unknown\0" as *const u8 as *const core::ffi::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn avifImageSetDefaults(mut image: *mut avifImage) {
    memset(
        image as *mut core::ffi::c_void,
        0 as core::ffi::c_int,
        ::core::mem::size_of::<avifImage>() as size_t,
    );
    (*image).yuvRange = AVIF_RANGE_FULL;
    (*image).colorPrimaries = AVIF_COLOR_PRIMARIES_UNSPECIFIED as core::ffi::c_int
        as avifColorPrimaries;
    (*image).transferCharacteristics = AVIF_TRANSFER_CHARACTERISTICS_UNSPECIFIED
        as core::ffi::c_int as avifTransferCharacteristics;
    (*image).matrixCoefficients = AVIF_MATRIX_COEFFICIENTS_UNSPECIFIED
        as core::ffi::c_int as avifMatrixCoefficients;
}
#[no_mangle]
pub unsafe extern "C" fn avifImageCreate(
    mut width: uint32_t,
    mut height: uint32_t,
    mut depth: uint32_t,
    mut yuvFormat: avifPixelFormat,
) -> *mut avifImage {
    if !(depth <= 16 as uint32_t) {
        return 0 as *mut avifImage;
    }
    if !(yuvFormat as core::ffi::c_int >= AVIF_PIXEL_FORMAT_NONE as core::ffi::c_int
        && (yuvFormat as core::ffi::c_uint)
            < AVIF_PIXEL_FORMAT_COUNT as core::ffi::c_int as core::ffi::c_uint)
    {
        return 0 as *mut avifImage;
    }
    let mut image: *mut avifImage = avifAlloc(
        ::core::mem::size_of::<avifImage>() as size_t,
    ) as *mut avifImage;
    if image.is_null() {
        return 0 as *mut avifImage;
    }
    avifImageSetDefaults(image);
    (*image).width = width;
    (*image).height = height;
    (*image).depth = depth;
    (*image).yuvFormat = yuvFormat;
    return image;
}
#[no_mangle]
pub unsafe extern "C" fn avifImageCreateEmpty() -> *mut avifImage {
    return avifImageCreate(
        0 as uint32_t,
        0 as uint32_t,
        0 as uint32_t,
        AVIF_PIXEL_FORMAT_NONE,
    );
}
#[no_mangle]
pub unsafe extern "C" fn avifImageCopyNoAlloc(
    mut dstImage: *mut avifImage,
    mut srcImage: *const avifImage,
) {
    (*dstImage).width = (*srcImage).width;
    (*dstImage).height = (*srcImage).height;
    (*dstImage).depth = (*srcImage).depth;
    (*dstImage).yuvFormat = (*srcImage).yuvFormat;
    (*dstImage).yuvRange = (*srcImage).yuvRange;
    (*dstImage).yuvChromaSamplePosition = (*srcImage).yuvChromaSamplePosition;
    (*dstImage).alphaPremultiplied = (*srcImage).alphaPremultiplied;
    (*dstImage).colorPrimaries = (*srcImage).colorPrimaries;
    (*dstImage).transferCharacteristics = (*srcImage).transferCharacteristics;
    (*dstImage).matrixCoefficients = (*srcImage).matrixCoefficients;
    (*dstImage).clli = (*srcImage).clli;
    (*dstImage).transformFlags = (*srcImage).transformFlags;
    (*dstImage).pasp = (*srcImage).pasp;
    (*dstImage).clap = (*srcImage).clap;
    (*dstImage).irot = (*srcImage).irot;
    (*dstImage).imir = (*srcImage).imir;
}
#[no_mangle]
pub unsafe extern "C" fn avifImageCopySamples(
    mut dstImage: *mut avifImage,
    mut srcImage: *const avifImage,
    mut planes: avifPlanesFlags,
) {
    planes & AVIF_PLANES_YUV as core::ffi::c_int as avifPlanesFlags != 0;
    let bytesPerPixel: size_t = (if avifImageUsesU16(srcImage) != 0 {
        2 as core::ffi::c_int
    } else {
        1 as core::ffi::c_int
    }) as size_t;
    let skipColor: avifBool = (planes
        & AVIF_PLANES_YUV as core::ffi::c_int as avifPlanesFlags == 0)
        as core::ffi::c_int;
    let skipAlpha: avifBool = (planes
        & AVIF_PLANES_A as core::ffi::c_int as avifPlanesFlags == 0) as core::ffi::c_int;
    let mut c: core::ffi::c_int = AVIF_CHAN_Y as core::ffi::c_int;
    while c <= AVIF_CHAN_A as core::ffi::c_int {
        let alpha: avifBool = (c == AVIF_CHAN_A as core::ffi::c_int) as core::ffi::c_int;
        if !(skipColor != 0 && alpha == 0 || skipAlpha != 0 && alpha != 0) {
            let planeWidth: uint32_t = avifImagePlaneWidth(srcImage, c) as uint32_t;
            let planeHeight: uint32_t = avifImagePlaneHeight(srcImage, c) as uint32_t;
            let mut srcRow: *const uint8_t = avifImagePlane(srcImage, c);
            let mut dstRow: *mut uint8_t = avifImagePlane(dstImage, c);
            let srcRowBytes: uint32_t = avifImagePlaneRowBytes(srcImage, c) as uint32_t;
            let dstRowBytes: uint32_t = avifImagePlaneRowBytes(dstImage, c) as uint32_t;
            if !srcRow.is_null() {
                let planeWidthBytes: size_t = (planeWidth as size_t)
                    .wrapping_mul(bytesPerPixel);
                let mut y: uint32_t = 0 as uint32_t;
                while y < planeHeight {
                    memcpy(
                        dstRow as *mut core::ffi::c_void,
                        srcRow as *const core::ffi::c_void,
                        planeWidthBytes,
                    );
                    srcRow = srcRow.offset(srcRowBytes as isize);
                    dstRow = dstRow.offset(dstRowBytes as isize);
                    y = y.wrapping_add(1);
                }
            }
        }
        c += 1;
    }
}
unsafe extern "C" fn avifImageCopyProperties(
    mut dstImage: *mut avifImage,
    mut srcImage: *const avifImage,
) -> avifResult {
    let mut i: size_t = 0 as size_t;
    while i < (*dstImage).numProperties {
        avifRWDataFree(&mut (*((*dstImage).properties).offset(i as isize)).boxPayload);
        i = i.wrapping_add(1);
    }
    avifFree((*dstImage).properties as *mut core::ffi::c_void);
    (*dstImage).properties = 0 as *mut avifImageItemProperty;
    (*dstImage).numProperties = 0 as size_t;
    if (*srcImage).numProperties != 0 as size_t {
        (*dstImage).properties = avifCalloc(
            (*srcImage).numProperties,
            ::core::mem::size_of::<avifImageItemProperty>() as size_t,
        ) as *mut avifImageItemProperty;
        if ((*dstImage).properties).is_null() {
            return AVIF_RESULT_OUT_OF_MEMORY;
        }
        (*dstImage).numProperties = (*srcImage).numProperties;
        let mut i_0: size_t = 0 as size_t;
        while i_0 < (*srcImage).numProperties {
            memcpy(
                ((*((*dstImage).properties).offset(i_0 as isize)).boxtype).as_mut_ptr()
                    as *mut core::ffi::c_void,
                ((*((*srcImage).properties).offset(i_0 as isize)).boxtype).as_mut_ptr()
                    as *const core::ffi::c_void,
                ::core::mem::size_of::<[uint8_t; 4]>() as size_t,
            );
            memcpy(
                ((*((*dstImage).properties).offset(i_0 as isize)).usertype).as_mut_ptr()
                    as *mut core::ffi::c_void,
                ((*((*srcImage).properties).offset(i_0 as isize)).usertype).as_mut_ptr()
                    as *const core::ffi::c_void,
                ::core::mem::size_of::<[uint8_t; 16]>() as size_t,
            );
            let result__: avifResult = avifRWDataSet(
                &mut (*((*dstImage).properties).offset(i_0 as isize)).boxPayload,
                (*((*srcImage).properties).offset(i_0 as isize)).boxPayload.data,
                (*((*srcImage).properties).offset(i_0 as isize)).boxPayload.size,
            ) as avifResult;
            if result__ as core::ffi::c_uint
                != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
            {
                return result__;
            }
            i_0 = i_0.wrapping_add(1);
        }
    }
    return AVIF_RESULT_OK;
}
#[no_mangle]
pub unsafe extern "C" fn avifImageCopy(
    mut dstImage: *mut avifImage,
    mut srcImage: *const avifImage,
    mut planes: avifPlanesFlags,
) -> avifResult {
    if !(dstImage != srcImage as *mut avifImage) {
        return AVIF_RESULT_INVALID_ARGUMENT;
    }
    avifImageFreePlanes(
        dstImage,
        AVIF_PLANES_ALL as core::ffi::c_int as avifPlanesFlags,
    );
    avifImageCopyNoAlloc(dstImage, srcImage);
    let result__: avifResult = avifImageSetProfileICC(
        dstImage,
        (*srcImage).icc.data,
        (*srcImage).icc.size,
    ) as avifResult;
    if result__ as core::ffi::c_uint
        != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
    {
        return result__;
    }
    let result___0: avifResult = avifRWDataSet(
        &mut (*dstImage).exif,
        (*srcImage).exif.data,
        (*srcImage).exif.size,
    ) as avifResult;
    if result___0 as core::ffi::c_uint
        != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
    {
        return result___0;
    }
    let result___1: avifResult = avifImageSetMetadataXMP(
        dstImage,
        (*srcImage).xmp.data,
        (*srcImage).xmp.size,
    ) as avifResult;
    if result___1 as core::ffi::c_uint
        != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
    {
        return result___1;
    }
    let result___2: avifResult = avifImageCopyProperties(dstImage, srcImage)
        as avifResult;
    if result___2 as core::ffi::c_uint
        != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
    {
        return result___2;
    }
    if planes & AVIF_PLANES_YUV as core::ffi::c_int as avifPlanesFlags != 0
        && !((*srcImage).yuvPlanes[AVIF_CHAN_Y as core::ffi::c_int as usize]).is_null()
    {
        if (*srcImage).yuvFormat as core::ffi::c_uint
            != AVIF_PIXEL_FORMAT_YUV400 as core::ffi::c_int as core::ffi::c_uint
            && (((*srcImage).yuvPlanes[AVIF_CHAN_U as core::ffi::c_int as usize])
                .is_null()
                || ((*srcImage).yuvPlanes[AVIF_CHAN_V as core::ffi::c_int as usize])
                    .is_null())
        {
            return AVIF_RESULT_INVALID_ARGUMENT;
        }
        let allocationResult: avifResult = avifImageAllocatePlanes(
            dstImage,
            AVIF_PLANES_YUV as core::ffi::c_int as avifPlanesFlags,
        ) as avifResult;
        if allocationResult as core::ffi::c_uint
            != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
        {
            return allocationResult;
        }
    }
    if planes & AVIF_PLANES_A as core::ffi::c_int as avifPlanesFlags != 0
        && !((*srcImage).alphaPlane).is_null()
    {
        let allocationResult_0: avifResult = avifImageAllocatePlanes(
            dstImage,
            AVIF_PLANES_A as core::ffi::c_int as avifPlanesFlags,
        ) as avifResult;
        if allocationResult_0 as core::ffi::c_uint
            != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
        {
            return allocationResult_0;
        }
    }
    avifImageCopySamples(dstImage, srcImage, planes);
    if !((*srcImage).gainMap).is_null() {
        if ((*dstImage).gainMap).is_null() {
            (*dstImage).gainMap = avifGainMapCreate();
            if ((*dstImage).gainMap).is_null() {
                return AVIF_RESULT_OUT_OF_MEMORY;
            }
        }
        let mut c: core::ffi::c_int = 0 as core::ffi::c_int;
        while c < 3 as core::ffi::c_int {
            (*(*dstImage).gainMap).gainMapMin[c as usize] = (*(*srcImage).gainMap)
                .gainMapMin[c as usize];
            (*(*dstImage).gainMap).gainMapMax[c as usize] = (*(*srcImage).gainMap)
                .gainMapMax[c as usize];
            (*(*dstImage).gainMap).gainMapGamma[c as usize] = (*(*srcImage).gainMap)
                .gainMapGamma[c as usize];
            (*(*dstImage).gainMap).baseOffset[c as usize] = (*(*srcImage).gainMap)
                .baseOffset[c as usize];
            (*(*dstImage).gainMap).alternateOffset[c as usize] = (*(*srcImage).gainMap)
                .alternateOffset[c as usize];
            c += 1;
        }
        (*(*dstImage).gainMap).baseHdrHeadroom = (*(*srcImage).gainMap).baseHdrHeadroom;
        (*(*dstImage).gainMap).alternateHdrHeadroom = (*(*srcImage).gainMap)
            .alternateHdrHeadroom;
        (*(*dstImage).gainMap).useBaseColorSpace = (*(*srcImage).gainMap)
            .useBaseColorSpace;
        let result___3: avifResult = avifRWDataSet(
            &mut (*(*dstImage).gainMap).altICC,
            (*(*srcImage).gainMap).altICC.data,
            (*(*srcImage).gainMap).altICC.size,
        ) as avifResult;
        if result___3 as core::ffi::c_uint
            != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
        {
            return result___3;
        }
        (*(*dstImage).gainMap).altColorPrimaries = (*(*srcImage).gainMap)
            .altColorPrimaries;
        (*(*dstImage).gainMap).altTransferCharacteristics = (*(*srcImage).gainMap)
            .altTransferCharacteristics;
        (*(*dstImage).gainMap).altMatrixCoefficients = (*(*srcImage).gainMap)
            .altMatrixCoefficients;
        (*(*dstImage).gainMap).altDepth = (*(*srcImage).gainMap).altDepth;
        (*(*dstImage).gainMap).altPlaneCount = (*(*srcImage).gainMap).altPlaneCount;
        (*(*dstImage).gainMap).altCLLI = (*(*srcImage).gainMap).altCLLI;
        if !((*(*srcImage).gainMap).image).is_null() {
            if ((*(*dstImage).gainMap).image).is_null() {
                (*(*dstImage).gainMap).image = avifImageCreateEmpty() as *mut avifImage;
                if ((*(*dstImage).gainMap).image).is_null() {
                    return AVIF_RESULT_OUT_OF_MEMORY;
                }
            }
            let result___4: avifResult = avifImageCopy(
                (*(*dstImage).gainMap).image as *mut avifImage,
                (*(*srcImage).gainMap).image,
                planes,
            ) as avifResult;
            if result___4 as core::ffi::c_uint
                != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
            {
                return result___4;
            }
        } else if !((*(*dstImage).gainMap).image).is_null() {
            avifImageDestroy((*(*dstImage).gainMap).image as *mut avifImage);
            (*(*dstImage).gainMap).image = 0 as *mut avifImage;
        }
    } else if !((*dstImage).gainMap).is_null() {
        avifGainMapDestroy((*dstImage).gainMap);
        (*dstImage).gainMap = 0 as *mut avifGainMap;
    }
    return AVIF_RESULT_OK;
}
#[no_mangle]
pub unsafe extern "C" fn avifImageSetViewRect(
    mut dstImage: *mut avifImage,
    mut srcImage: *const avifImage,
    mut rect: *const avifCropRect,
) -> avifResult {
    if !(dstImage != srcImage as *mut avifImage) {
        return AVIF_RESULT_INVALID_ARGUMENT;
    }
    let mut formatInfo: avifPixelFormatInfo = avifPixelFormatInfo {
        monochrome: 0,
        chromaShiftX: 0,
        chromaShiftY: 0,
    };
    avifGetPixelFormatInfo((*srcImage).yuvFormat, &mut formatInfo);
    if (*rect).width > (*srcImage).width || (*rect).height > (*srcImage).height
        || (*rect).x > ((*srcImage).width).wrapping_sub((*rect).width)
        || (*rect).y > ((*srcImage).height).wrapping_sub((*rect).height)
    {
        return AVIF_RESULT_INVALID_ARGUMENT;
    }
    if formatInfo.monochrome == 0
        && ((*rect).x & formatInfo.chromaShiftX as uint32_t != 0
            || (*rect).y & formatInfo.chromaShiftY as uint32_t != 0)
    {
        return AVIF_RESULT_INVALID_ARGUMENT;
    }
    avifImageFreePlanes(
        dstImage,
        AVIF_PLANES_ALL as core::ffi::c_int as avifPlanesFlags,
    );
    avifImageCopyNoAlloc(dstImage, srcImage);
    (*dstImage).width = (*rect).width;
    (*dstImage).height = (*rect).height;
    let pixelBytes: uint32_t = (if (*srcImage).depth > 8 as uint32_t {
        2 as core::ffi::c_int
    } else {
        1 as core::ffi::c_int
    }) as uint32_t;
    if !((*srcImage).yuvPlanes[AVIF_CHAN_Y as core::ffi::c_int as usize]).is_null() {
        let mut yuvPlane: core::ffi::c_int = AVIF_CHAN_Y as core::ffi::c_int;
        while yuvPlane <= AVIF_CHAN_V as core::ffi::c_int {
            if (*srcImage).yuvRowBytes[yuvPlane as usize] != 0 {
                let planeX: size_t = (if yuvPlane == AVIF_CHAN_Y as core::ffi::c_int {
                    (*rect).x
                } else {
                    (*rect).x >> formatInfo.chromaShiftX
                }) as size_t;
                let planeY: size_t = (if yuvPlane == AVIF_CHAN_Y as core::ffi::c_int {
                    (*rect).y
                } else {
                    (*rect).y >> formatInfo.chromaShiftY
                }) as size_t;
                (*dstImage).yuvPlanes[yuvPlane as usize] = ((*srcImage)
                    .yuvPlanes[yuvPlane as usize])
                    .offset(
                        planeY
                            .wrapping_mul(
                                (*srcImage).yuvRowBytes[yuvPlane as usize] as size_t,
                            ) as isize,
                    )
                    .offset(planeX.wrapping_mul(pixelBytes as size_t) as isize);
                (*dstImage).yuvRowBytes[yuvPlane as usize] = (*srcImage)
                    .yuvRowBytes[yuvPlane as usize];
            }
            yuvPlane += 1;
        }
    }
    if !((*srcImage).alphaPlane).is_null() {
        (*dstImage).alphaPlane = ((*srcImage).alphaPlane)
            .offset(
                ((*rect).y as size_t).wrapping_mul((*srcImage).alphaRowBytes as size_t)
                    as isize,
            )
            .offset(((*rect).x as size_t).wrapping_mul(pixelBytes as size_t) as isize);
        (*dstImage).alphaRowBytes = (*srcImage).alphaRowBytes;
    }
    return AVIF_RESULT_OK;
}
#[no_mangle]
pub unsafe extern "C" fn avifImageDestroy(mut image: *mut avifImage) {
    if !((*image).gainMap).is_null() {
        avifGainMapDestroy((*image).gainMap);
    }
    avifImageFreePlanes(image, AVIF_PLANES_ALL as core::ffi::c_int as avifPlanesFlags);
    avifRWDataFree(&mut (*image).icc);
    avifRWDataFree(&mut (*image).exif);
    avifRWDataFree(&mut (*image).xmp);
    let mut i: size_t = 0 as size_t;
    while i < (*image).numProperties {
        avifRWDataFree(&mut (*((*image).properties).offset(i as isize)).boxPayload);
        i = i.wrapping_add(1);
    }
    avifFree((*image).properties as *mut core::ffi::c_void);
    (*image).properties = 0 as *mut avifImageItemProperty;
    (*image).numProperties = 0 as size_t;
    avifFree(image as *mut core::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn avifImageSetProfileICC(
    mut image: *mut avifImage,
    mut icc: *const uint8_t,
    mut iccSize: size_t,
) -> avifResult {
    return avifRWDataSet(&mut (*image).icc, icc, iccSize);
}
#[no_mangle]
pub unsafe extern "C" fn avifImageSetMetadataXMP(
    mut image: *mut avifImage,
    mut xmp: *const uint8_t,
    mut xmpSize: size_t,
) -> avifResult {
    return avifRWDataSet(&mut (*image).xmp, xmp, xmpSize);
}
#[no_mangle]
pub unsafe extern "C" fn avifImagePushProperty(
    mut image: *mut avifImage,
    mut boxtype: *const uint8_t,
    mut usertype: *const uint8_t,
    mut boxPayload: *const uint8_t,
    mut boxPayloadSize: size_t,
) -> avifResult {
    if !((*image).numProperties
        < (18446744073709551615 as usize)
            .wrapping_div(::core::mem::size_of::<avifImageItemProperty>() as usize))
    {
        return AVIF_RESULT_INVALID_ARGUMENT;
    }
    let numProperties: size_t = ((*image).numProperties).wrapping_add(1 as size_t);
    let properties: *mut avifImageItemProperty = avifAlloc(
        numProperties
            .wrapping_mul(::core::mem::size_of::<avifImageItemProperty>() as size_t),
    ) as *mut avifImageItemProperty;
    if properties.is_null() {
        return AVIF_RESULT_OUT_OF_MEMORY;
    }
    if (*image).numProperties != 0 as size_t {
        memcpy(
            properties as *mut core::ffi::c_void,
            (*image).properties as *const core::ffi::c_void,
            ((*image).numProperties)
                .wrapping_mul(::core::mem::size_of::<avifImageItemProperty>() as size_t),
        );
    }
    avifFree((*image).properties as *mut core::ffi::c_void);
    (*image).properties = properties;
    (*image).numProperties = numProperties;
    let property: *mut avifImageItemProperty = &mut *((*image).properties)
        .offset(((*image).numProperties).wrapping_sub(1 as size_t) as isize)
        as *mut avifImageItemProperty;
    memset(
        property as *mut core::ffi::c_void,
        0 as core::ffi::c_int,
        ::core::mem::size_of::<avifImageItemProperty>() as size_t,
    );
    memcpy(
        ((*property).boxtype).as_mut_ptr() as *mut core::ffi::c_void,
        boxtype as *const core::ffi::c_void,
        ::core::mem::size_of::<[uint8_t; 4]>() as size_t,
    );
    memcpy(
        ((*property).usertype).as_mut_ptr() as *mut core::ffi::c_void,
        usertype as *const core::ffi::c_void,
        ::core::mem::size_of::<[uint8_t; 16]>() as size_t,
    );
    let result__: avifResult = avifRWDataSet(
        &mut (*property).boxPayload,
        boxPayload,
        boxPayloadSize,
    ) as avifResult;
    if result__ as core::ffi::c_uint
        != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
    {
        return result__;
    }
    return AVIF_RESULT_OK;
}
#[no_mangle]
pub unsafe extern "C" fn avifImageAddOpaqueProperty(
    mut image: *mut avifImage,
    mut boxtype: *const uint8_t,
    mut data: *const uint8_t,
    mut dataSize: size_t,
) -> avifResult {
    let uuid: [uint8_t; 16] = [
        0 as core::ffi::c_int as uint8_t,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
    ];
    if avifIsKnownPropertyType(boxtype) != 0 {
        return AVIF_RESULT_INVALID_ARGUMENT;
    }
    return avifImagePushProperty(image, boxtype, uuid.as_ptr(), data, dataSize);
}
#[no_mangle]
pub unsafe extern "C" fn avifImageAddUUIDProperty(
    mut image: *mut avifImage,
    mut uuid: *const uint8_t,
    mut data: *const uint8_t,
    mut dataSize: size_t,
) -> avifResult {
    let boxtype: [uint8_t; 4] = [
        'u' as i32 as uint8_t,
        'u' as i32 as uint8_t,
        'i' as i32 as uint8_t,
        'd' as i32 as uint8_t,
    ];
    if avifIsValidUUID(uuid) == 0 {
        return AVIF_RESULT_INVALID_ARGUMENT;
    }
    return avifImagePushProperty(image, boxtype.as_ptr(), uuid, data, dataSize);
}
#[no_mangle]
pub unsafe extern "C" fn avifImageAllocatePlanes(
    mut image: *mut avifImage,
    mut planes: avifPlanesFlags,
) -> avifResult {
    if (*image).width == 0 as uint32_t || (*image).height == 0 as uint32_t
        || (*image).depth == 0 as uint32_t || (*image).depth > 16 as uint32_t
    {
        return AVIF_RESULT_INVALID_ARGUMENT;
    }
    let channelSize: uint32_t = (if avifImageUsesU16(image) != 0 {
        2 as core::ffi::c_int
    } else {
        1 as core::ffi::c_int
    }) as uint32_t;
    if (*image).width > (UINT32_MAX as uint32_t).wrapping_div(channelSize) {
        return AVIF_RESULT_INVALID_ARGUMENT;
    }
    let fullRowBytes: uint32_t = channelSize.wrapping_mul((*image).width);
    if (*image).height as core::ffi::c_long
        > PTRDIFF_MAX / fullRowBytes as core::ffi::c_long
    {
        return AVIF_RESULT_INVALID_ARGUMENT;
    }
    let fullSize: size_t = (fullRowBytes as size_t)
        .wrapping_mul((*image).height as size_t);
    if planes & AVIF_PLANES_YUV as core::ffi::c_int as avifPlanesFlags != 0
        && (*image).yuvFormat as core::ffi::c_uint
            != AVIF_PIXEL_FORMAT_NONE as core::ffi::c_int as core::ffi::c_uint
    {
        let mut info: avifPixelFormatInfo = avifPixelFormatInfo {
            monochrome: 0,
            chromaShiftX: 0,
            chromaShiftY: 0,
        };
        avifGetPixelFormatInfo((*image).yuvFormat, &mut info);
        (*image).imageOwnsYUVPlanes = AVIF_TRUE as avifBool;
        if ((*image).yuvPlanes[AVIF_CHAN_Y as core::ffi::c_int as usize]).is_null() {
            (*image).yuvPlanes[AVIF_CHAN_Y as core::ffi::c_int as usize] = avifAlloc(
                fullSize,
            ) as *mut uint8_t;
            if ((*image).yuvPlanes[AVIF_CHAN_Y as core::ffi::c_int as usize]).is_null() {
                return AVIF_RESULT_OUT_OF_MEMORY;
            }
            (*image).yuvRowBytes[AVIF_CHAN_Y as core::ffi::c_int as usize] = fullRowBytes;
        }
        if info.monochrome == 0 {
            let shiftedW: uint32_t = (((*image).width as uint64_t)
                .wrapping_add(info.chromaShiftX as uint64_t) >> info.chromaShiftX)
                as uint32_t;
            let shiftedH: uint32_t = (((*image).height as uint64_t)
                .wrapping_add(info.chromaShiftY as uint64_t) >> info.chromaShiftY)
                as uint32_t;
            let uvRowBytes: uint32_t = channelSize.wrapping_mul(shiftedW);
            let uvSize: size_t = (uvRowBytes as size_t).wrapping_mul(shiftedH as size_t);
            let mut uvPlane: core::ffi::c_int = AVIF_CHAN_U as core::ffi::c_int;
            while uvPlane <= AVIF_CHAN_V as core::ffi::c_int {
                if ((*image).yuvPlanes[uvPlane as usize]).is_null() {
                    (*image).yuvPlanes[uvPlane as usize] = avifAlloc(uvSize)
                        as *mut uint8_t;
                    if ((*image).yuvPlanes[uvPlane as usize]).is_null() {
                        return AVIF_RESULT_OUT_OF_MEMORY;
                    }
                    (*image).yuvRowBytes[uvPlane as usize] = uvRowBytes;
                }
                uvPlane += 1;
            }
        }
    }
    if planes & AVIF_PLANES_A as core::ffi::c_int as avifPlanesFlags != 0 {
        (*image).imageOwnsAlphaPlane = AVIF_TRUE as avifBool;
        if ((*image).alphaPlane).is_null() {
            (*image).alphaPlane = avifAlloc(fullSize) as *mut uint8_t;
            if ((*image).alphaPlane).is_null() {
                return AVIF_RESULT_OUT_OF_MEMORY;
            }
            (*image).alphaRowBytes = fullRowBytes;
        }
    }
    return AVIF_RESULT_OK;
}
#[no_mangle]
pub unsafe extern "C" fn avifImageFreePlanes(
    mut image: *mut avifImage,
    mut planes: avifPlanesFlags,
) {
    if planes & AVIF_PLANES_YUV as core::ffi::c_int as avifPlanesFlags != 0
        && (*image).yuvFormat as core::ffi::c_uint
            != AVIF_PIXEL_FORMAT_NONE as core::ffi::c_int as core::ffi::c_uint
    {
        if (*image).imageOwnsYUVPlanes != 0 {
            avifFree(
                (*image).yuvPlanes[AVIF_CHAN_Y as core::ffi::c_int as usize]
                    as *mut core::ffi::c_void,
            );
            avifFree(
                (*image).yuvPlanes[AVIF_CHAN_U as core::ffi::c_int as usize]
                    as *mut core::ffi::c_void,
            );
            avifFree(
                (*image).yuvPlanes[AVIF_CHAN_V as core::ffi::c_int as usize]
                    as *mut core::ffi::c_void,
            );
        }
        (*image).yuvPlanes[AVIF_CHAN_Y as core::ffi::c_int as usize] = 0 as *mut uint8_t;
        (*image).yuvRowBytes[AVIF_CHAN_Y as core::ffi::c_int as usize] = 0 as uint32_t;
        (*image).yuvPlanes[AVIF_CHAN_U as core::ffi::c_int as usize] = 0 as *mut uint8_t;
        (*image).yuvRowBytes[AVIF_CHAN_U as core::ffi::c_int as usize] = 0 as uint32_t;
        (*image).yuvPlanes[AVIF_CHAN_V as core::ffi::c_int as usize] = 0 as *mut uint8_t;
        (*image).yuvRowBytes[AVIF_CHAN_V as core::ffi::c_int as usize] = 0 as uint32_t;
        (*image).imageOwnsYUVPlanes = AVIF_FALSE as avifBool;
    }
    if planes & AVIF_PLANES_A as core::ffi::c_int as avifPlanesFlags != 0 {
        if (*image).imageOwnsAlphaPlane != 0 {
            avifFree((*image).alphaPlane as *mut core::ffi::c_void);
        }
        (*image).alphaPlane = 0 as *mut uint8_t;
        (*image).alphaRowBytes = 0 as uint32_t;
        (*image).imageOwnsAlphaPlane = AVIF_FALSE as avifBool;
    }
}
#[no_mangle]
pub unsafe extern "C" fn avifImageStealPlanes(
    mut dstImage: *mut avifImage,
    mut srcImage: *mut avifImage,
    mut planes: avifPlanesFlags,
) {
    avifImageFreePlanes(dstImage, planes);
    if planes & AVIF_PLANES_YUV as core::ffi::c_int as avifPlanesFlags != 0 {
        (*dstImage).yuvPlanes[AVIF_CHAN_Y as core::ffi::c_int as usize] = (*srcImage)
            .yuvPlanes[AVIF_CHAN_Y as core::ffi::c_int as usize];
        (*dstImage).yuvRowBytes[AVIF_CHAN_Y as core::ffi::c_int as usize] = (*srcImage)
            .yuvRowBytes[AVIF_CHAN_Y as core::ffi::c_int as usize];
        (*dstImage).yuvPlanes[AVIF_CHAN_U as core::ffi::c_int as usize] = (*srcImage)
            .yuvPlanes[AVIF_CHAN_U as core::ffi::c_int as usize];
        (*dstImage).yuvRowBytes[AVIF_CHAN_U as core::ffi::c_int as usize] = (*srcImage)
            .yuvRowBytes[AVIF_CHAN_U as core::ffi::c_int as usize];
        (*dstImage).yuvPlanes[AVIF_CHAN_V as core::ffi::c_int as usize] = (*srcImage)
            .yuvPlanes[AVIF_CHAN_V as core::ffi::c_int as usize];
        (*dstImage).yuvRowBytes[AVIF_CHAN_V as core::ffi::c_int as usize] = (*srcImage)
            .yuvRowBytes[AVIF_CHAN_V as core::ffi::c_int as usize];
        (*srcImage).yuvPlanes[AVIF_CHAN_Y as core::ffi::c_int as usize] = 0
            as *mut uint8_t;
        (*srcImage).yuvRowBytes[AVIF_CHAN_Y as core::ffi::c_int as usize] = 0
            as uint32_t;
        (*srcImage).yuvPlanes[AVIF_CHAN_U as core::ffi::c_int as usize] = 0
            as *mut uint8_t;
        (*srcImage).yuvRowBytes[AVIF_CHAN_U as core::ffi::c_int as usize] = 0
            as uint32_t;
        (*srcImage).yuvPlanes[AVIF_CHAN_V as core::ffi::c_int as usize] = 0
            as *mut uint8_t;
        (*srcImage).yuvRowBytes[AVIF_CHAN_V as core::ffi::c_int as usize] = 0
            as uint32_t;
        (*dstImage).yuvFormat = (*srcImage).yuvFormat;
        (*dstImage).imageOwnsYUVPlanes = (*srcImage).imageOwnsYUVPlanes;
        (*srcImage).imageOwnsYUVPlanes = AVIF_FALSE as avifBool;
    }
    if planes & AVIF_PLANES_A as core::ffi::c_int as avifPlanesFlags != 0 {
        (*dstImage).alphaPlane = (*srcImage).alphaPlane;
        (*dstImage).alphaRowBytes = (*srcImage).alphaRowBytes;
        (*srcImage).alphaPlane = 0 as *mut uint8_t;
        (*srcImage).alphaRowBytes = 0 as uint32_t;
        (*dstImage).imageOwnsAlphaPlane = (*srcImage).imageOwnsAlphaPlane;
        (*srcImage).imageOwnsAlphaPlane = AVIF_FALSE as avifBool;
    }
}
#[no_mangle]
pub unsafe extern "C" fn avifImageUsesU16(mut image: *const avifImage) -> avifBool {
    return ((*image).depth > 8 as uint32_t) as core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn avifImageIsOpaque(mut image: *const avifImage) -> avifBool {
    if ((*image).alphaPlane).is_null() {
        return AVIF_TRUE;
    }
    let opaqueValue: uint32_t = ((1 as uint32_t) << (*image).depth)
        .wrapping_sub(1 as uint32_t);
    let mut row: *const uint8_t = (*image).alphaPlane;
    let mut y: uint32_t = 0 as uint32_t;
    while y < (*image).height {
        if avifImageUsesU16(image) != 0 {
            let mut row16: *const uint16_t = row as *const uint16_t;
            let mut x: uint32_t = 0 as uint32_t;
            while x < (*image).width {
                if *row16.offset(x as isize) as uint32_t != opaqueValue {
                    return AVIF_FALSE;
                }
                x = x.wrapping_add(1);
            }
        } else {
            let mut x_0: uint32_t = 0 as uint32_t;
            while x_0 < (*image).width {
                if *row.offset(x_0 as isize) as uint32_t != opaqueValue {
                    return AVIF_FALSE;
                }
                x_0 = x_0.wrapping_add(1);
            }
        }
        row = row.offset((*image).alphaRowBytes as isize);
        y = y.wrapping_add(1);
    }
    return AVIF_TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn avifImagePlane(
    mut image: *const avifImage,
    mut channel: core::ffi::c_int,
) -> *mut uint8_t {
    if channel == AVIF_CHAN_Y as core::ffi::c_int
        || channel == AVIF_CHAN_U as core::ffi::c_int
        || channel == AVIF_CHAN_V as core::ffi::c_int
    {
        return (*image).yuvPlanes[channel as usize];
    }
    if channel == AVIF_CHAN_A as core::ffi::c_int {
        return (*image).alphaPlane;
    }
    return 0 as *mut uint8_t;
}
#[no_mangle]
pub unsafe extern "C" fn avifImagePlaneRowBytes(
    mut image: *const avifImage,
    mut channel: core::ffi::c_int,
) -> uint32_t {
    if channel == AVIF_CHAN_Y as core::ffi::c_int
        || channel == AVIF_CHAN_U as core::ffi::c_int
        || channel == AVIF_CHAN_V as core::ffi::c_int
    {
        return (*image).yuvRowBytes[channel as usize];
    }
    if channel == AVIF_CHAN_A as core::ffi::c_int {
        return (*image).alphaRowBytes;
    }
    return 0 as uint32_t;
}
#[no_mangle]
pub unsafe extern "C" fn avifImagePlaneWidth(
    mut image: *const avifImage,
    mut channel: core::ffi::c_int,
) -> uint32_t {
    if channel == AVIF_CHAN_Y as core::ffi::c_int {
        return (*image).width;
    }
    if channel == AVIF_CHAN_U as core::ffi::c_int
        || channel == AVIF_CHAN_V as core::ffi::c_int
    {
        let mut formatInfo: avifPixelFormatInfo = avifPixelFormatInfo {
            monochrome: 0,
            chromaShiftX: 0,
            chromaShiftY: 0,
        };
        avifGetPixelFormatInfo((*image).yuvFormat, &mut formatInfo);
        if formatInfo.monochrome != 0 {
            return 0 as uint32_t;
        }
        return ((*image).width).wrapping_add(formatInfo.chromaShiftX as uint32_t)
            >> formatInfo.chromaShiftX;
    }
    if channel == AVIF_CHAN_A as core::ffi::c_int && !((*image).alphaPlane).is_null() {
        return (*image).width;
    }
    return 0 as uint32_t;
}
#[no_mangle]
pub unsafe extern "C" fn avifImagePlaneHeight(
    mut image: *const avifImage,
    mut channel: core::ffi::c_int,
) -> uint32_t {
    if channel == AVIF_CHAN_Y as core::ffi::c_int {
        return (*image).height;
    }
    if channel == AVIF_CHAN_U as core::ffi::c_int
        || channel == AVIF_CHAN_V as core::ffi::c_int
    {
        let mut formatInfo: avifPixelFormatInfo = avifPixelFormatInfo {
            monochrome: 0,
            chromaShiftX: 0,
            chromaShiftY: 0,
        };
        avifGetPixelFormatInfo((*image).yuvFormat, &mut formatInfo);
        if formatInfo.monochrome != 0 {
            return 0 as uint32_t;
        }
        return ((*image).height).wrapping_add(formatInfo.chromaShiftY as uint32_t)
            >> formatInfo.chromaShiftY;
    }
    if channel == AVIF_CHAN_A as core::ffi::c_int && !((*image).alphaPlane).is_null() {
        return (*image).height;
    }
    return 0 as uint32_t;
}
#[no_mangle]
pub unsafe extern "C" fn avifDimensionsTooLarge(
    mut width: uint32_t,
    mut height: uint32_t,
    mut imageSizeLimit: uint32_t,
    mut imageDimensionLimit: uint32_t,
) -> avifBool {
    if imageSizeLimit != 0 as uint32_t && width > imageSizeLimit.wrapping_div(height) {
        return AVIF_TRUE;
    }
    if imageDimensionLimit != 0 as uint32_t
        && (width > imageDimensionLimit || height > imageDimensionLimit)
    {
        return AVIF_TRUE;
    }
    return AVIF_FALSE;
}
#[no_mangle]
pub unsafe extern "C" fn avifCodecDestroy(mut codec: *mut avifCodec) {
    if !codec.is_null() && ((*codec).destroyInternal).is_some() {
        ((*codec).destroyInternal)
            .expect("non-null function pointer")(codec as *mut avifCodec);
    }
    avifFree(codec as *mut core::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn avifRGBFormatIsGray(mut format: avifRGBFormat) -> avifBool {
    return (format as core::ffi::c_uint
        == AVIF_RGB_FORMAT_GRAY as core::ffi::c_int as core::ffi::c_uint
        || format as core::ffi::c_uint
            == AVIF_RGB_FORMAT_GRAYA as core::ffi::c_int as core::ffi::c_uint
        || format as core::ffi::c_uint
            == AVIF_RGB_FORMAT_AGRAY as core::ffi::c_int as core::ffi::c_uint)
        as core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn avifRGBFormatHasAlpha(mut format: avifRGBFormat) -> avifBool {
    return (format as core::ffi::c_uint
        != AVIF_RGB_FORMAT_RGB as core::ffi::c_int as core::ffi::c_uint
        && format as core::ffi::c_uint
            != AVIF_RGB_FORMAT_BGR as core::ffi::c_int as core::ffi::c_uint
        && format as core::ffi::c_uint
            != AVIF_RGB_FORMAT_RGB_565 as core::ffi::c_int as core::ffi::c_uint
        && format as core::ffi::c_uint
            != AVIF_RGB_FORMAT_GRAY as core::ffi::c_int as core::ffi::c_uint)
        as core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn avifRGBFormatChannelCount(
    mut format: avifRGBFormat,
) -> uint32_t {
    if format as core::ffi::c_uint
        == AVIF_RGB_FORMAT_GRAY as core::ffi::c_int as core::ffi::c_uint
    {
        return 1 as uint32_t;
    }
    if format as core::ffi::c_uint
        == AVIF_RGB_FORMAT_GRAYA as core::ffi::c_int as core::ffi::c_uint
        || format as core::ffi::c_uint
            == AVIF_RGB_FORMAT_AGRAY as core::ffi::c_int as core::ffi::c_uint
    {
        return 2 as uint32_t;
    }
    return (if avifRGBFormatHasAlpha(format) != 0 {
        4 as core::ffi::c_int
    } else {
        3 as core::ffi::c_int
    }) as uint32_t;
}
#[no_mangle]
pub unsafe extern "C" fn avifRGBImagePixelSize(
    mut rgb: *const avifRGBImage,
) -> uint32_t {
    if (*rgb).format as core::ffi::c_uint
        == AVIF_RGB_FORMAT_RGB_565 as core::ffi::c_int as core::ffi::c_uint
    {
        return 2 as uint32_t;
    }
    return (avifRGBFormatChannelCount((*rgb).format))
        .wrapping_mul(
            (if (*rgb).depth > 8 as uint32_t {
                2 as core::ffi::c_int
            } else {
                1 as core::ffi::c_int
            }) as uint32_t,
        );
}
#[no_mangle]
pub unsafe extern "C" fn avifRGBImageSetDefaults(
    mut rgb: *mut avifRGBImage,
    mut image: *const avifImage,
) {
    (*rgb).width = (*image).width;
    (*rgb).height = (*image).height;
    (*rgb).depth = (*image).depth;
    (*rgb).format = AVIF_RGB_FORMAT_RGBA;
    (*rgb).chromaUpsampling = AVIF_CHROMA_UPSAMPLING_AUTOMATIC;
    (*rgb).chromaDownsampling = AVIF_CHROMA_DOWNSAMPLING_AUTOMATIC;
    (*rgb).avoidLibYUV = AVIF_FALSE as avifBool;
    (*rgb).ignoreAlpha = AVIF_FALSE as avifBool;
    (*rgb).pixels = 0 as *mut uint8_t;
    (*rgb).rowBytes = 0 as uint32_t;
    (*rgb).alphaPremultiplied = AVIF_FALSE as avifBool;
    (*rgb).isFloat = AVIF_FALSE as avifBool;
    (*rgb).maxThreads = 1 as core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn avifRGBImageAllocatePixels(
    mut rgb: *mut avifRGBImage,
) -> avifResult {
    avifRGBImageFreePixels(rgb);
    if (*rgb).width == 0 as uint32_t || (*rgb).height == 0 as uint32_t {
        return AVIF_RESULT_INVALID_ARGUMENT;
    }
    let pixelSize: uint32_t = avifRGBImagePixelSize(rgb) as uint32_t;
    if (*rgb).width > (UINT32_MAX as uint32_t).wrapping_div(pixelSize) {
        return AVIF_RESULT_INVALID_ARGUMENT;
    }
    let rowBytes: uint32_t = ((*rgb).width).wrapping_mul(pixelSize);
    if (*rgb).height as core::ffi::c_long > PTRDIFF_MAX / rowBytes as core::ffi::c_long {
        return AVIF_RESULT_INVALID_ARGUMENT;
    }
    (*rgb).pixels = avifAlloc((rowBytes as size_t).wrapping_mul((*rgb).height as size_t))
        as *mut uint8_t;
    if ((*rgb).pixels).is_null() {
        return AVIF_RESULT_OUT_OF_MEMORY;
    }
    (*rgb).rowBytes = rowBytes;
    return AVIF_RESULT_OK;
}
#[no_mangle]
pub unsafe extern "C" fn avifRGBImageFreePixels(mut rgb: *mut avifRGBImage) {
    if !((*rgb).pixels).is_null() {
        avifFree((*rgb).pixels as *mut core::ffi::c_void);
    }
    (*rgb).pixels = 0 as *mut uint8_t;
    (*rgb).rowBytes = 0 as uint32_t;
}
unsafe extern "C" fn calcCenter(mut dim: int32_t) -> avifFraction {
    let mut f: avifFraction = avifFraction { n: 0, d: 0 };
    f.n = dim >> 1 as core::ffi::c_int;
    f.d = 1 as core::ffi::c_int as int32_t;
    if dim % 2 as int32_t != 0 as int32_t {
        f.n = dim;
        f.d = 2 as core::ffi::c_int as int32_t;
    }
    return f;
}
unsafe extern "C" fn overflowsInt32(mut x: int64_t) -> avifBool {
    return (x < INT32_MIN as int64_t || x > INT32_MAX as int64_t) as core::ffi::c_int;
}
unsafe extern "C" fn avifCropRectIsValid(
    mut cropRect: *const avifCropRect,
    mut imageW: uint32_t,
    mut imageH: uint32_t,
    mut diag: *mut avifDiagnostics,
) -> avifBool {
    if (*cropRect).width == 0 as uint32_t || (*cropRect).height == 0 as uint32_t {
        avifDiagnosticsPrintf(
            diag,
            b"[Strict] crop rect width and height must be nonzero\0" as *const u8
                as *const core::ffi::c_char,
        );
        return AVIF_FALSE;
    }
    if (*cropRect).x > (UINT32_MAX as uint32_t).wrapping_sub((*cropRect).width)
        || ((*cropRect).x).wrapping_add((*cropRect).width) > imageW
        || (*cropRect).y > (UINT32_MAX as uint32_t).wrapping_sub((*cropRect).height)
        || ((*cropRect).y).wrapping_add((*cropRect).height) > imageH
    {
        avifDiagnosticsPrintf(
            diag,
            b"[Strict] crop rect is out of the image's bounds\0" as *const u8
                as *const core::ffi::c_char,
        );
        return AVIF_FALSE;
    }
    return AVIF_TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn avifCropRectFromCleanApertureBox(
    mut cropRect: *mut avifCropRect,
    mut clap: *const avifCleanApertureBox,
    mut imageW: uint32_t,
    mut imageH: uint32_t,
    mut diag: *mut avifDiagnostics,
) -> avifBool {
    avifDiagnosticsClearError(diag);
    let widthN: int32_t = (*clap).widthN as int32_t;
    let widthD: int32_t = (*clap).widthD as int32_t;
    let heightN: int32_t = (*clap).heightN as int32_t;
    let heightD: int32_t = (*clap).heightD as int32_t;
    let horizOffN: int32_t = (*clap).horizOffN as int32_t;
    let horizOffD: int32_t = (*clap).horizOffD as int32_t;
    let vertOffN: int32_t = (*clap).vertOffN as int32_t;
    let vertOffD: int32_t = (*clap).vertOffD as int32_t;
    if widthD <= 0 as int32_t || heightD <= 0 as int32_t || horizOffD <= 0 as int32_t
        || vertOffD <= 0 as int32_t
    {
        avifDiagnosticsPrintf(
            diag,
            b"[Strict] clap contains a denominator that is not strictly positive\0"
                as *const u8 as *const core::ffi::c_char,
        );
        return AVIF_FALSE;
    }
    if widthN < 0 as int32_t || heightN < 0 as int32_t {
        avifDiagnosticsPrintf(
            diag,
            b"[Strict] clap width or height is negative\0" as *const u8
                as *const core::ffi::c_char,
        );
        return AVIF_FALSE;
    }
    if widthN % widthD != 0 as int32_t {
        avifDiagnosticsPrintf(
            diag,
            b"[Strict] clap width %d/%d is not an integer\0" as *const u8
                as *const core::ffi::c_char,
            widthN,
            widthD,
        );
        return AVIF_FALSE;
    }
    if heightN % heightD != 0 as int32_t {
        avifDiagnosticsPrintf(
            diag,
            b"[Strict] clap height %d/%d is not an integer\0" as *const u8
                as *const core::ffi::c_char,
            heightN,
            heightD,
        );
        return AVIF_FALSE;
    }
    let clapW: int32_t = widthN / widthD;
    let clapH: int32_t = heightN / heightD;
    if imageW > INT32_MAX as uint32_t || imageH > INT32_MAX as uint32_t {
        avifDiagnosticsPrintf(
            diag,
            b"[Strict] image width %u or height %u is greater than INT32_MAX\0"
                as *const u8 as *const core::ffi::c_char,
            imageW,
            imageH,
        );
        return AVIF_FALSE;
    }
    let mut uncroppedCenterX: avifFraction = calcCenter(imageW as int32_t);
    let mut uncroppedCenterY: avifFraction = calcCenter(imageH as int32_t);
    let mut horizOff: avifFraction = avifFraction { n: 0, d: 0 };
    horizOff.n = horizOffN;
    horizOff.d = horizOffD;
    let mut croppedCenterX: avifFraction = avifFraction { n: 0, d: 0 };
    if avifFractionAdd(uncroppedCenterX, horizOff, &mut croppedCenterX) == 0 {
        avifDiagnosticsPrintf(
            diag,
            b"[Strict] croppedCenterX overflowed\0" as *const u8
                as *const core::ffi::c_char,
        );
        return AVIF_FALSE;
    }
    let mut vertOff: avifFraction = avifFraction { n: 0, d: 0 };
    vertOff.n = vertOffN;
    vertOff.d = vertOffD;
    let mut croppedCenterY: avifFraction = avifFraction { n: 0, d: 0 };
    if avifFractionAdd(uncroppedCenterY, vertOff, &mut croppedCenterY) == 0 {
        avifDiagnosticsPrintf(
            diag,
            b"[Strict] croppedCenterY overflowed\0" as *const u8
                as *const core::ffi::c_char,
        );
        return AVIF_FALSE;
    }
    let mut halfW: avifFraction = avifFraction { n: 0, d: 0 };
    halfW.n = clapW;
    halfW.d = 2 as core::ffi::c_int as int32_t;
    let mut cropX: avifFraction = avifFraction { n: 0, d: 0 };
    if avifFractionSub(croppedCenterX, halfW, &mut cropX) == 0 {
        avifDiagnosticsPrintf(
            diag,
            b"[Strict] cropX overflowed\0" as *const u8 as *const core::ffi::c_char,
        );
        return AVIF_FALSE;
    }
    if cropX.n % cropX.d != 0 as int32_t {
        avifDiagnosticsPrintf(
            diag,
            b"[Strict] calculated crop X offset %d/%d is not an integer\0" as *const u8
                as *const core::ffi::c_char,
            cropX.n,
            cropX.d,
        );
        return AVIF_FALSE;
    }
    let mut halfH: avifFraction = avifFraction { n: 0, d: 0 };
    halfH.n = clapH;
    halfH.d = 2 as core::ffi::c_int as int32_t;
    let mut cropY: avifFraction = avifFraction { n: 0, d: 0 };
    if avifFractionSub(croppedCenterY, halfH, &mut cropY) == 0 {
        avifDiagnosticsPrintf(
            diag,
            b"[Strict] cropY overflowed\0" as *const u8 as *const core::ffi::c_char,
        );
        return AVIF_FALSE;
    }
    if cropY.n % cropY.d != 0 as int32_t {
        avifDiagnosticsPrintf(
            diag,
            b"[Strict] calculated crop Y offset %d/%d is not an integer\0" as *const u8
                as *const core::ffi::c_char,
            cropY.n,
            cropY.d,
        );
        return AVIF_FALSE;
    }
    if cropX.n < 0 as int32_t || cropY.n < 0 as int32_t {
        avifDiagnosticsPrintf(
            diag,
            b"[Strict] at least one crop offset is not positive\0" as *const u8
                as *const core::ffi::c_char,
        );
        return AVIF_FALSE;
    }
    (*cropRect).x = (cropX.n / cropX.d) as uint32_t;
    (*cropRect).y = (cropY.n / cropY.d) as uint32_t;
    (*cropRect).width = clapW as uint32_t;
    (*cropRect).height = clapH as uint32_t;
    return avifCropRectIsValid(cropRect, imageW, imageH, diag);
}
#[no_mangle]
pub unsafe extern "C" fn avifCleanApertureBoxFromCropRect(
    mut clap: *mut avifCleanApertureBox,
    mut cropRect: *const avifCropRect,
    mut imageW: uint32_t,
    mut imageH: uint32_t,
    mut diag: *mut avifDiagnostics,
) -> avifBool {
    avifDiagnosticsClearError(diag);
    if avifCropRectIsValid(cropRect, imageW, imageH, diag) == 0 {
        return AVIF_FALSE;
    }
    if imageW > INT32_MAX as uint32_t || imageH > INT32_MAX as uint32_t {
        avifDiagnosticsPrintf(
            diag,
            b"[Strict] image width %u or height %u is greater than INT32_MAX\0"
                as *const u8 as *const core::ffi::c_char,
            imageW,
            imageH,
        );
        return AVIF_FALSE;
    }
    let mut uncroppedCenterX: avifFraction = calcCenter(imageW as int32_t);
    let mut uncroppedCenterY: avifFraction = calcCenter(imageH as int32_t);
    if (*cropRect).width > INT32_MAX as uint32_t
        || (*cropRect).height > INT32_MAX as uint32_t
    {
        avifDiagnosticsPrintf(
            diag,
            b"[Strict] crop rect width %u or height %u is greater than INT32_MAX\0"
                as *const u8 as *const core::ffi::c_char,
            (*cropRect).width,
            (*cropRect).height,
        );
        return AVIF_FALSE;
    }
    let mut croppedCenterX: avifFraction = calcCenter((*cropRect).width as int32_t);
    let croppedCenterXN: int64_t = croppedCenterX.n as int64_t
        + (*cropRect).x as int64_t * croppedCenterX.d as int64_t;
    if overflowsInt32(croppedCenterXN) != 0 {
        avifDiagnosticsPrintf(
            diag,
            b"[Strict] croppedCenterX overflowed\0" as *const u8
                as *const core::ffi::c_char,
        );
        return AVIF_FALSE;
    }
    croppedCenterX.n = croppedCenterXN as int32_t;
    let mut croppedCenterY: avifFraction = calcCenter((*cropRect).height as int32_t);
    let croppedCenterYN: int64_t = croppedCenterY.n as int64_t
        + (*cropRect).y as int64_t * croppedCenterY.d as int64_t;
    if overflowsInt32(croppedCenterYN) != 0 {
        avifDiagnosticsPrintf(
            diag,
            b"[Strict] croppedCenterY overflowed\0" as *const u8
                as *const core::ffi::c_char,
        );
        return AVIF_FALSE;
    }
    croppedCenterY.n = croppedCenterYN as int32_t;
    let mut horizOff: avifFraction = avifFraction { n: 0, d: 0 };
    if avifFractionSub(croppedCenterX, uncroppedCenterX, &mut horizOff) == 0 {
        avifDiagnosticsPrintf(
            diag,
            b"[Strict] horizOff overflowed\0" as *const u8 as *const core::ffi::c_char,
        );
        return AVIF_FALSE;
    }
    let mut vertOff: avifFraction = avifFraction { n: 0, d: 0 };
    if avifFractionSub(croppedCenterY, uncroppedCenterY, &mut vertOff) == 0 {
        avifDiagnosticsPrintf(
            diag,
            b"[Strict] vertOff overflowed\0" as *const u8 as *const core::ffi::c_char,
        );
        return AVIF_FALSE;
    }
    (*clap).widthN = (*cropRect).width;
    (*clap).widthD = 1 as uint32_t;
    (*clap).heightN = (*cropRect).height;
    (*clap).heightD = 1 as uint32_t;
    (*clap).horizOffN = horizOff.n as uint32_t;
    (*clap).horizOffD = horizOff.d as uint32_t;
    (*clap).vertOffN = vertOff.n as uint32_t;
    (*clap).vertOffD = vertOff.d as uint32_t;
    return AVIF_TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn avifCropRectRequiresUpsampling(
    mut cropRect: *const avifCropRect,
    mut yuvFormat: avifPixelFormat,
) -> avifBool {
    return ((yuvFormat as core::ffi::c_uint
        == AVIF_PIXEL_FORMAT_YUV420 as core::ffi::c_int as core::ffi::c_uint
        || yuvFormat as core::ffi::c_uint
            == AVIF_PIXEL_FORMAT_YUV422 as core::ffi::c_int as core::ffi::c_uint)
        && ((*cropRect).x).wrapping_rem(2 as uint32_t) != 0
        || yuvFormat as core::ffi::c_uint
            == AVIF_PIXEL_FORMAT_YUV420 as core::ffi::c_int as core::ffi::c_uint
            && ((*cropRect).y).wrapping_rem(2 as uint32_t) != 0) as core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn avifCropRectConvertCleanApertureBox(
    mut cropRect: *mut avifCropRect,
    mut clap: *const avifCleanApertureBox,
    mut imageW: uint32_t,
    mut imageH: uint32_t,
    mut yuvFormat: avifPixelFormat,
    mut diag: *mut avifDiagnostics,
) -> avifBool {
    if avifCropRectFromCleanApertureBox(cropRect, clap, imageW, imageH, diag) == 0 {
        return AVIF_FALSE;
    }
    if avifCropRectRequiresUpsampling(cropRect, yuvFormat) != 0 {
        avifDiagnosticsPrintf(
            diag,
            b"[Strict] crop rect X and Y offsets must be even due to this image's YUV subsampling\0"
                as *const u8 as *const core::ffi::c_char,
        );
        return AVIF_FALSE;
    }
    return AVIF_TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn avifCleanApertureBoxConvertCropRect(
    mut clap: *mut avifCleanApertureBox,
    mut cropRect: *const avifCropRect,
    mut imageW: uint32_t,
    mut imageH: uint32_t,
    mut yuvFormat: avifPixelFormat,
    mut diag: *mut avifDiagnostics,
) -> avifBool {
    if avifCropRectRequiresUpsampling(cropRect, yuvFormat) != 0 {
        avifDiagnosticsPrintf(
            diag,
            b"[Strict] crop rect X and Y offsets must be even due to this image's YUV subsampling\0"
                as *const u8 as *const core::ffi::c_char,
        );
        return AVIF_FALSE;
    }
    return avifCleanApertureBoxFromCropRect(clap, cropRect, imageW, imageH, diag);
}
#[no_mangle]
pub unsafe extern "C" fn avifIsAlpha(mut itemCategory: avifItemCategory) -> avifBool {
    if itemCategory as core::ffi::c_uint
        == AVIF_ITEM_ALPHA as core::ffi::c_int as core::ffi::c_uint
    {
        return AVIF_TRUE;
    }
    if itemCategory as core::ffi::c_uint
        >= AVIF_ITEM_SAMPLE_TRANSFORM_INPUT_0_ALPHA as core::ffi::c_int
            as core::ffi::c_uint
        && (itemCategory as core::ffi::c_uint)
            < (AVIF_ITEM_SAMPLE_TRANSFORM_INPUT_0_ALPHA as core::ffi::c_int
                + (AVIF_ITEM_SAMPLE_TRANSFORM_INPUT_0_ALPHA as core::ffi::c_int
                    - AVIF_ITEM_SAMPLE_TRANSFORM_INPUT_0_COLOR as core::ffi::c_int))
                as core::ffi::c_uint
    {
        return AVIF_TRUE;
    }
    return AVIF_FALSE;
}
#[no_mangle]
pub unsafe extern "C" fn avifAreGridDimensionsValid(
    mut yuvFormat: avifPixelFormat,
    mut imageW: uint32_t,
    mut imageH: uint32_t,
    mut tileW: uint32_t,
    mut tileH: uint32_t,
    mut diag: *mut avifDiagnostics,
) -> avifBool {
    if tileW < 64 as uint32_t || tileH < 64 as uint32_t {
        avifDiagnosticsPrintf(
            diag,
            b"Grid image tile width (%u) or height (%u) cannot be smaller than 64. See MIAF (ISO/IEC 23000-22:2019), Section 7.3.11.4.2\0"
                as *const u8 as *const core::ffi::c_char,
            tileW,
            tileH,
        );
        return AVIF_FALSE;
    }
    if (yuvFormat as core::ffi::c_uint
        == AVIF_PIXEL_FORMAT_YUV420 as core::ffi::c_int as core::ffi::c_uint
        || yuvFormat as core::ffi::c_uint
            == AVIF_PIXEL_FORMAT_YUV422 as core::ffi::c_int as core::ffi::c_uint)
        && (imageW.wrapping_rem(2 as uint32_t) != 0 as uint32_t
            || tileW.wrapping_rem(2 as uint32_t) != 0 as uint32_t)
        || yuvFormat as core::ffi::c_uint
            == AVIF_PIXEL_FORMAT_YUV420 as core::ffi::c_int as core::ffi::c_uint
            && (imageH.wrapping_rem(2 as uint32_t) != 0 as uint32_t
                || tileH.wrapping_rem(2 as uint32_t) != 0 as uint32_t)
    {
        avifDiagnosticsPrintf(
            diag,
            b"Grid image width (%u) or height (%u) or tile width (%u) or height (%u) shall be even if chroma is subsampled in that dimension. See MIAF (ISO/IEC 23000-22:2019), Section 7.3.11.4.2\0"
                as *const u8 as *const core::ffi::c_char,
            imageW,
            imageH,
            tileW,
            tileH,
        );
        return AVIF_FALSE;
    }
    return AVIF_TRUE;
}
unsafe extern "C" fn avifStrdup(
    mut str: *const core::ffi::c_char,
) -> *mut core::ffi::c_char {
    let mut len: size_t = strlen(str);
    let mut dup: *mut core::ffi::c_char = avifAlloc(len.wrapping_add(1 as size_t))
        as *mut core::ffi::c_char;
    if dup.is_null() {
        return 0 as *mut core::ffi::c_char;
    }
    memcpy(
        dup as *mut core::ffi::c_void,
        str as *const core::ffi::c_void,
        len.wrapping_add(1 as size_t),
    );
    return dup;
}
#[no_mangle]
pub unsafe extern "C" fn avifCodecSpecificOptionsCreate() -> *mut avifCodecSpecificOptions {
    let mut ava: *mut avifCodecSpecificOptions = avifAlloc(
        ::core::mem::size_of::<avifCodecSpecificOptions>() as size_t,
    ) as *mut avifCodecSpecificOptions;
    if ava.is_null()
        || avifArrayCreate(
            ava as *mut core::ffi::c_void,
            ::core::mem::size_of::<avifCodecSpecificOption>() as uint32_t,
            4 as uint32_t,
        ) == 0
    {
        avifFree(ava as *mut core::ffi::c_void);
        return 0 as *mut avifCodecSpecificOptions;
    } else {
        return ava
    };
}
#[no_mangle]
pub unsafe extern "C" fn avifCodecSpecificOptionsClear(
    mut csOptions: *mut avifCodecSpecificOptions,
) {
    let mut i: uint32_t = 0 as uint32_t;
    while i < (*csOptions).count {
        let mut entry: *mut avifCodecSpecificOption = &mut *((*csOptions).entries)
            .offset(i as isize) as *mut avifCodecSpecificOption;
        avifFree((*entry).key as *mut core::ffi::c_void);
        avifFree((*entry).value as *mut core::ffi::c_void);
        i = i.wrapping_add(1);
    }
    (*csOptions).count = 0 as uint32_t;
}
#[no_mangle]
pub unsafe extern "C" fn avifCodecSpecificOptionsDestroy(
    mut csOptions: *mut avifCodecSpecificOptions,
) {
    avifCodecSpecificOptionsClear(csOptions);
    avifArrayDestroy(csOptions as *mut core::ffi::c_void);
    avifFree(csOptions as *mut core::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn avifCodecSpecificOptionsSet(
    mut csOptions: *mut avifCodecSpecificOptions,
    mut key: *const core::ffi::c_char,
    mut value: *const core::ffi::c_char,
) -> avifResult {
    let mut i: uint32_t = 0 as uint32_t;
    while i < (*csOptions).count {
        let mut entry: *mut avifCodecSpecificOption = &mut *((*csOptions).entries)
            .offset(i as isize) as *mut avifCodecSpecificOption;
        if strcmp((*entry).key, key) == 0 {
            if !value.is_null() {
                let mut newValue: *mut core::ffi::c_char = avifStrdup(value);
                if newValue.is_null() {
                    return AVIF_RESULT_OUT_OF_MEMORY;
                }
                avifFree((*entry).value as *mut core::ffi::c_void);
                (*entry).value = newValue;
            } else {
                avifFree((*entry).key as *mut core::ffi::c_void);
                avifFree((*entry).value as *mut core::ffi::c_void);
                (*csOptions).count = ((*csOptions).count).wrapping_sub(1);
                if (*csOptions).count > 0 as uint32_t {
                    memmove(
                        &mut *((*csOptions).entries).offset(i as isize)
                            as *mut avifCodecSpecificOption as *mut core::ffi::c_void,
                        &mut *((*csOptions).entries)
                            .offset(i.wrapping_add(1 as uint32_t) as isize)
                            as *mut avifCodecSpecificOption as *const core::ffi::c_void,
                        (((*csOptions).count).wrapping_sub(i) as size_t)
                            .wrapping_mul((*csOptions).elementSize as size_t),
                    );
                }
            }
            return AVIF_RESULT_OK;
        }
        i = i.wrapping_add(1);
    }
    if !value.is_null() {
        let mut newKey: *mut core::ffi::c_char = avifStrdup(key);
        if newKey.is_null() {
            return AVIF_RESULT_OUT_OF_MEMORY;
        }
        let mut newValue_0: *mut core::ffi::c_char = avifStrdup(value);
        if newValue_0.is_null() {
            avifFree(newKey as *mut core::ffi::c_void);
            return AVIF_RESULT_OUT_OF_MEMORY;
        }
        let mut entry_0: *mut avifCodecSpecificOption = avifArrayPush(
            csOptions as *mut core::ffi::c_void,
        ) as *mut avifCodecSpecificOption;
        if entry_0.is_null() {
            avifFree(newKey as *mut core::ffi::c_void);
            avifFree(newValue_0 as *mut core::ffi::c_void);
            return AVIF_RESULT_OUT_OF_MEMORY;
        }
        (*entry_0).key = newKey;
        (*entry_0).value = newValue_0;
    }
    return AVIF_RESULT_OK;
}
static mut availableCodecs: [AvailableCodec; 3 + cfg!(feature = "libgav1") as usize + cfg!(feature = "rav1e") as usize + cfg!(feature = "svt") as usize + cfg!(feature = "avm") as usize] = unsafe {
    [
        {
            let mut init = AvailableCodec {
                choice: AVIF_CODEC_CHOICE_DAV1D,
                type_0: AVIF_CODEC_TYPE_AV1,
                name: b"dav1d\0" as *const u8 as *const core::ffi::c_char,
                version: Some(
                    avifCodecVersionDav1d
                        as unsafe extern "C" fn() -> *const core::ffi::c_char,
                ),
                create: Some(
                    avifCodecCreateDav1d as unsafe extern "C" fn() -> *mut avifCodec,
                ),
                flags: AVIF_CODEC_FLAG_CAN_DECODE as core::ffi::c_int as uint32_t,
            };
            init
        },
        #[cfg(feature = "libgav1")]
        {
            let mut init = AvailableCodec {
                choice: AVIF_CODEC_CHOICE_LIBGAV1,
                type_0: AVIF_CODEC_TYPE_AV1,
                name: b"libgav1\0" as *const u8 as *const core::ffi::c_char,
                version: Some(
                    avifCodecVersionGav1
                        as unsafe extern "C" fn() -> *const core::ffi::c_char,
                ),
                create: Some(
                    avifCodecCreateGav1 as unsafe extern "C" fn() -> *mut avifCodec,
                ),
                flags: AVIF_CODEC_FLAG_CAN_DECODE as core::ffi::c_int as uint32_t,
            };
            init
        },
        {
            let mut init = AvailableCodec {
                choice: AVIF_CODEC_CHOICE_AOM,
                type_0: AVIF_CODEC_TYPE_AV1,
                name: b"aom\0" as *const u8 as *const core::ffi::c_char,
                version: Some(
                    avifCodecVersionAOM
                        as unsafe extern "C" fn() -> *const core::ffi::c_char,
                ),
                create: Some(
                    avifCodecCreateAOM as unsafe extern "C" fn() -> *mut avifCodec,
                ),
                flags: (AVIF_CODEC_FLAG_CAN_DECODE as core::ffi::c_int
                    | AVIF_CODEC_FLAG_CAN_ENCODE as core::ffi::c_int) as uint32_t,
            };
            init
        },
        #[cfg(feature = "rav1e")]
        {
            let mut init = AvailableCodec {
                choice: AVIF_CODEC_CHOICE_RAV1E,
                type_0: AVIF_CODEC_TYPE_AV1,
                name: b"rav1e\0" as *const u8 as *const core::ffi::c_char,
                version: Some(
                    avifCodecVersionRav1e
                        as unsafe extern "C" fn() -> *const core::ffi::c_char,
                ),
                create: Some(
                    avifCodecCreateRav1e as unsafe extern "C" fn() -> *mut avifCodec,
                ),
                flags: AVIF_CODEC_FLAG_CAN_ENCODE as core::ffi::c_int as uint32_t,
            };
            init
        },
        #[cfg(feature = "svt")]
        {
            let mut init = AvailableCodec {
                choice: AVIF_CODEC_CHOICE_SVT,
                type_0: AVIF_CODEC_TYPE_AV1,
                name: b"svt\0" as *const u8 as *const core::ffi::c_char,
                version: Some(
                    avifCodecVersionSvt
                        as unsafe extern "C" fn() -> *const core::ffi::c_char,
                ),
                create: Some(
                    avifCodecCreateSvt as unsafe extern "C" fn() -> *mut avifCodec,
                ),
                flags: AVIF_CODEC_FLAG_CAN_ENCODE as core::ffi::c_int as uint32_t,
            };
            init
        },
        #[cfg(feature = "avm")]
        {
            let mut init = AvailableCodec {
                choice: AVIF_CODEC_CHOICE_AVM,
                type_0: AVIF_CODEC_TYPE_AV2,
                name: b"avm\0" as *const u8 as *const core::ffi::c_char,
                version: Some(
                    avifCodecVersionAVM
                        as unsafe extern "C" fn() -> *const core::ffi::c_char,
                ),
                create: Some(
                    avifCodecCreateAVM as unsafe extern "C" fn() -> *mut avifCodec,
                ),
                flags: (AVIF_CODEC_FLAG_CAN_DECODE as core::ffi::c_int
                    | AVIF_CODEC_FLAG_CAN_ENCODE as core::ffi::c_int) as uint32_t,
            };
            init
        },
        {
            let mut init = AvailableCodec {
                choice: AVIF_CODEC_CHOICE_AUTO,
                type_0: AVIF_CODEC_TYPE_UNKNOWN,
                name: 0 as *const core::ffi::c_char,
                version: None,
                create: None,
                flags: 0 as uint32_t,
            };
            init
        },
    ]
};
static mut availableCodecsCount: core::ffi::c_int = 0;
unsafe extern "C" fn findAvailableCodec(
    mut choice: avifCodecChoice,
    mut requiredFlags: avifCodecFlags,
) -> *mut AvailableCodec {
    let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
    while i < availableCodecsCount {
        if !(choice as core::ffi::c_uint
            != AVIF_CODEC_CHOICE_AUTO as core::ffi::c_int as core::ffi::c_uint
            && availableCodecs[i as usize].choice as core::ffi::c_uint
                != choice as core::ffi::c_uint)
        {
            if !(requiredFlags != 0
                && availableCodecs[i as usize].flags & requiredFlags as uint32_t
                    != requiredFlags)
            {
                if !(choice as core::ffi::c_uint
                    == AVIF_CODEC_CHOICE_AUTO as core::ffi::c_int as core::ffi::c_uint
                    && availableCodecs[i as usize].choice as core::ffi::c_uint
                        == AVIF_CODEC_CHOICE_AVM as core::ffi::c_int
                            as core::ffi::c_uint)
                {
                    return &mut *core::ptr::addr_of_mut!(availableCodecs).cast::<AvailableCodec>().offset(i as isize)
                        as *mut AvailableCodec;
                }
            }
        }
        i += 1;
    }
    return 0 as *mut AvailableCodec;
}
#[no_mangle]
pub unsafe extern "C" fn avifCodecName(
    mut choice: avifCodecChoice,
    mut requiredFlags: avifCodecFlags,
) -> *const core::ffi::c_char {
    let mut availableCodec: *mut AvailableCodec = findAvailableCodec(
        choice,
        requiredFlags,
    );
    if !availableCodec.is_null() {
        return (*availableCodec).name;
    }
    return 0 as *const core::ffi::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn avifCodecTypeFromChoice(
    mut choice: avifCodecChoice,
    mut requiredFlags: avifCodecFlags,
) -> avifCodecType {
    let mut availableCodec: *mut AvailableCodec = findAvailableCodec(
        choice,
        requiredFlags,
    );
    if !availableCodec.is_null() {
        return (*availableCodec).type_0;
    }
    return AVIF_CODEC_TYPE_UNKNOWN;
}
#[no_mangle]
pub unsafe extern "C" fn avifCodecChoiceFromName(
    mut name: *const core::ffi::c_char,
) -> avifCodecChoice {
    let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
    while i < availableCodecsCount {
        if strcmp(availableCodecs[i as usize].name, name) == 0 {
            return availableCodecs[i as usize].choice;
        }
        i += 1;
    }
    return AVIF_CODEC_CHOICE_AUTO;
}
#[no_mangle]
pub unsafe extern "C" fn avifCodecCreate(
    mut choice: avifCodecChoice,
    mut requiredFlags: avifCodecFlags,
    mut codec: *mut *mut avifCodec,
) -> avifResult {
    *codec = 0 as *mut avifCodec;
    let mut availableCodec: *mut AvailableCodec = findAvailableCodec(
        choice,
        requiredFlags,
    );
    if availableCodec.is_null() {
        return AVIF_RESULT_NO_CODEC_AVAILABLE;
    }
    *codec = ((*availableCodec).create).expect("non-null function pointer")();
    if (*codec).is_null() {
        return AVIF_RESULT_OUT_OF_MEMORY;
    }
    return AVIF_RESULT_OK;
}
unsafe extern "C" fn append(
    mut writePos: *mut *mut core::ffi::c_char,
    mut remainingLen: *mut size_t,
    mut appendStr: *const core::ffi::c_char,
) {
    let mut appendLen: size_t = strlen(appendStr);
    if appendLen > *remainingLen {
        appendLen = *remainingLen;
    }
    memcpy(
        *writePos as *mut core::ffi::c_void,
        appendStr as *const core::ffi::c_void,
        appendLen,
    );
    *remainingLen = (*remainingLen).wrapping_sub(appendLen);
    *writePos = (*writePos).offset(appendLen as isize);
    **writePos = 0 as core::ffi::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn avifCodecVersions(mut outBuffer: *mut core::ffi::c_char) {
    let mut remainingLen: size_t = 255 as size_t;
    let mut writePos: *mut core::ffi::c_char = outBuffer as *mut core::ffi::c_char;
    *writePos = 0 as core::ffi::c_char;
    let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
    while i < availableCodecsCount {
        if i > 0 as core::ffi::c_int {
            append(
                &mut writePos,
                &mut remainingLen,
                b", \0" as *const u8 as *const core::ffi::c_char,
            );
        }
        append(&mut writePos, &mut remainingLen, availableCodecs[i as usize].name);
        if availableCodecs[i as usize].flags
            & (AVIF_CODEC_FLAG_CAN_ENCODE as core::ffi::c_int
                | AVIF_CODEC_FLAG_CAN_DECODE as core::ffi::c_int) as uint32_t
            == (AVIF_CODEC_FLAG_CAN_ENCODE as core::ffi::c_int
                | AVIF_CODEC_FLAG_CAN_DECODE as core::ffi::c_int) as uint32_t
        {
            append(
                &mut writePos,
                &mut remainingLen,
                b" [enc/dec]\0" as *const u8 as *const core::ffi::c_char,
            );
        } else if availableCodecs[i as usize].flags
            & AVIF_CODEC_FLAG_CAN_ENCODE as core::ffi::c_int as uint32_t != 0
        {
            append(
                &mut writePos,
                &mut remainingLen,
                b" [enc]\0" as *const u8 as *const core::ffi::c_char,
            );
        } else if availableCodecs[i as usize].flags
            & AVIF_CODEC_FLAG_CAN_DECODE as core::ffi::c_int as uint32_t != 0
        {
            append(
                &mut writePos,
                &mut remainingLen,
                b" [dec]\0" as *const u8 as *const core::ffi::c_char,
            );
        }
        append(
            &mut writePos,
            &mut remainingLen,
            b":\0" as *const u8 as *const core::ffi::c_char,
        );
        append(
            &mut writePos,
            &mut remainingLen,
            (availableCodecs[i as usize].version).expect("non-null function pointer")(),
        );
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn avifGainMapCreate() -> *mut avifGainMap {
    let mut gainMap: *mut avifGainMap = avifAlloc(
        ::core::mem::size_of::<avifGainMap>() as size_t,
    ) as *mut avifGainMap;
    if gainMap.is_null() {
        return 0 as *mut avifGainMap;
    }
    avifGainMapSetDefaults(gainMap);
    return gainMap;
}
#[no_mangle]
pub unsafe extern "C" fn avifGainMapSetDefaults(mut gainMap: *mut avifGainMap) {
    memset(
        gainMap as *mut core::ffi::c_void,
        0 as core::ffi::c_int,
        ::core::mem::size_of::<avifGainMap>() as size_t,
    );
    (*gainMap).altColorPrimaries = AVIF_COLOR_PRIMARIES_UNSPECIFIED as core::ffi::c_int
        as avifColorPrimaries;
    (*gainMap).altTransferCharacteristics = AVIF_TRANSFER_CHARACTERISTICS_UNSPECIFIED
        as core::ffi::c_int as avifTransferCharacteristics;
    (*gainMap).altMatrixCoefficients = AVIF_MATRIX_COEFFICIENTS_UNSPECIFIED
        as core::ffi::c_int as avifMatrixCoefficients;
    (*gainMap).altYUVRange = AVIF_RANGE_FULL;
    (*gainMap).useBaseColorSpace = AVIF_TRUE as avifBool;
    let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
    while i < 3 as core::ffi::c_int {
        (*gainMap).gainMapMin[i as usize].d = 1 as uint32_t;
        (*gainMap).gainMapMax[i as usize].d = 1 as uint32_t;
        (*gainMap).gainMapGamma[i as usize].n = 1 as uint32_t;
        (*gainMap).gainMapGamma[i as usize].d = 1 as uint32_t;
        (*gainMap).baseOffset[i as usize].d = 1 as uint32_t;
        (*gainMap).alternateOffset[i as usize].d = 1 as uint32_t;
        i += 1;
    }
    (*gainMap).baseHdrHeadroom.d = 1 as uint32_t;
    (*gainMap).alternateHdrHeadroom.d = 1 as uint32_t;
}
#[no_mangle]
pub unsafe extern "C" fn avifGainMapDestroy(mut gainMap: *mut avifGainMap) {
    if !((*gainMap).image).is_null() {
        avifImageDestroy((*gainMap).image as *mut avifImage);
    }
    avifRWDataFree(&mut (*gainMap).altICC);
    avifFree(gainMap as *mut core::ffi::c_void);
}
unsafe extern "C" fn run_static_initializers() {
    availableCodecsCount = (::core::mem::size_of::<[AvailableCodec; 3 + cfg!(feature = "libgav1") as usize + cfg!(feature = "rav1e") as usize + cfg!(feature = "svt") as usize + cfg!(feature = "avm") as usize]>() as usize)
        .wrapping_div(::core::mem::size_of::<AvailableCodec>() as usize)
        .wrapping_sub(1 as usize) as core::ffi::c_int;
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
#[cfg_attr(libavif_rs_asan, sanitize(address = "off"))]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
