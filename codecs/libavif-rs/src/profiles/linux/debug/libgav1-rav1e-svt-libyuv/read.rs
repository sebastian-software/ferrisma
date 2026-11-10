// Copyright 2019 Joe Drago. All rights reserved.
// SPDX-License-Identifier: BSD-2-Clause

// Mechanically translated from upstream/libavif/src/read.c.
// C ABI, ownership, pointer validity, and bounds follow the original headers.
extern "C" {
    pub type avifEncoderData;
    pub type avifCodecInternal;
    fn avifAlloc(size: size_t) -> *mut core::ffi::c_void;
    fn avifFree(p: *mut core::ffi::c_void);
    fn avifRWDataRealloc(raw: *mut avifRWData, newSize: size_t) -> avifResult;
    fn avifRWDataSet(
        raw: *mut avifRWData,
        data: *const uint8_t,
        len: size_t,
    ) -> avifResult;
    fn avifRWDataFree(raw: *mut avifRWData);
    fn avifGetExifTiffHeaderOffset(
        exif: *const uint8_t,
        exifSize: size_t,
        offset: *mut size_t,
    ) -> avifResult;
    fn avifDiagnosticsClearError(diag: *mut avifDiagnostics);
    fn avifCropRectFromCleanApertureBox(
        cropRect: *mut avifCropRect,
        clap: *const avifCleanApertureBox,
        imageW: uint32_t,
        imageH: uint32_t,
        diag: *mut avifDiagnostics,
    ) -> avifBool;
    fn avifGainMapCreate() -> *mut avifGainMap;
    fn avifImageCreate(
        width: uint32_t,
        height: uint32_t,
        depth: uint32_t,
        yuvFormat: avifPixelFormat,
    ) -> *mut avifImage;
    fn avifImageCreateEmpty() -> *mut avifImage;
    fn avifImageCopy(
        dstImage: *mut avifImage,
        srcImage: *const avifImage,
        planes: avifPlanesFlags,
    ) -> avifResult;
    fn avifImageSetViewRect(
        dstImage: *mut avifImage,
        srcImage: *const avifImage,
        rect: *const avifCropRect,
    ) -> avifResult;
    fn avifImageDestroy(image: *mut avifImage);
    fn avifImageSetMetadataXMP(
        image: *mut avifImage,
        xmp: *const uint8_t,
        xmpSize: size_t,
    ) -> avifResult;
    fn avifImageAllocatePlanes(
        image: *mut avifImage,
        planes: avifPlanesFlags,
    ) -> avifResult;
    fn avifImageFreePlanes(image: *mut avifImage, planes: avifPlanesFlags);
    fn avifImageStealPlanes(
        dstImage: *mut avifImage,
        srcImage: *mut avifImage,
        planes: avifPlanesFlags,
    );
    fn avifLimitedToFullY(depth: uint32_t, v: core::ffi::c_int) -> core::ffi::c_int;
    fn avifCodecName(
        choice: avifCodecChoice,
        requiredFlags: avifCodecFlags,
    ) -> *const core::ffi::c_char;
    fn avifIOCreateMemoryReader(data: *const uint8_t, size: size_t) -> *mut avifIO;
    fn avifIOCreateFileReader(filename: *const core::ffi::c_char) -> *mut avifIO;
    fn avifIODestroy(io: *mut avifIO);
    fn avifArrayCreate(
        arrayStruct: *mut core::ffi::c_void,
        elementSize: uint32_t,
        initialCapacity: uint32_t,
    ) -> avifBool;
    fn avifArrayPush(arrayStruct: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn avifArrayPop(arrayStruct: *mut core::ffi::c_void);
    fn avifArrayDestroy(arrayStruct: *mut core::ffi::c_void);
    fn avifImageSetDefaults(image: *mut avifImage);
    fn avifImageCopyNoAlloc(dstImage: *mut avifImage, srcImage: *const avifImage);
    fn avifImageCopySamples(
        dstImage: *mut avifImage,
        srcImage: *const avifImage,
        planes: avifPlanesFlags,
    );
    fn avifImagePushProperty(
        image: *mut avifImage,
        boxtype: *const uint8_t,
        usertype: *const uint8_t,
        boxPayload: *const uint8_t,
        boxPayloadSize: size_t,
    ) -> avifResult;
    fn avifSampleTransformExpressionIsValid(
        tokens: *const avifSampleTransformExpression,
        numInputImageItems: uint32_t,
    ) -> avifBool;
    fn avifImageApplyExpression(
        dstImage: *mut avifImage,
        bitDepth: avifSampleTransformBitDepth,
        expression: *const avifSampleTransformExpression,
        numInputImageItems: uint8_t,
        inputImageItems: *mut *const avifImage,
        planes: avifPlanesFlags,
    ) -> avifResult;
    fn avifDimensionsTooLarge(
        width: uint32_t,
        height: uint32_t,
        imageSizeLimit: uint32_t,
        imageDimensionLimit: uint32_t,
    ) -> avifBool;
    fn avifImageScaleWithLimit(
        image: *mut avifImage,
        dstWidth: uint32_t,
        dstHeight: uint32_t,
        imageSizeLimit: uint32_t,
        imageDimensionLimit: uint32_t,
        diag: *mut avifDiagnostics,
    ) -> avifResult;
    fn avifIsAlpha(itemCategory: avifItemCategory) -> avifBool;
    fn avifAreGridDimensionsValid(
        yuvFormat: avifPixelFormat,
        imageW: uint32_t,
        imageH: uint32_t,
        tileW: uint32_t,
        tileH: uint32_t,
        diag: *mut avifDiagnostics,
    ) -> avifBool;
    fn avifCodecTypeFromChoice(
        choice: avifCodecChoice,
        requiredFlags: avifCodecFlags,
    ) -> avifCodecType;
    fn avifCodecCreate(
        choice: avifCodecChoice,
        requiredFlags: avifCodecFlags,
        codec: *mut *mut avifCodec,
    ) -> avifResult;
    fn avifCodecDestroy(codec: *mut avifCodec);
    fn avifDiagnosticsPrintf(
        diag: *mut avifDiagnostics,
        format: *const core::ffi::c_char,
        ...
    );
    fn avifROStreamCurrent(stream: *mut avifROStream) -> *const uint8_t;
    fn avifROStreamStart(
        stream: *mut avifROStream,
        raw: *mut avifROData,
        diag: *mut avifDiagnostics,
        diagContext: *const core::ffi::c_char,
    );
    fn avifROStreamOffset(stream: *const avifROStream) -> size_t;
    fn avifROStreamHasBytesLeft(
        stream: *const avifROStream,
        byteCount: size_t,
    ) -> avifBool;
    fn avifROStreamRemainingBytes(stream: *const avifROStream) -> size_t;
    fn avifROStreamSkip(stream: *mut avifROStream, byteCount: size_t) -> avifBool;
    fn avifROStreamRead(
        stream: *mut avifROStream,
        data: *mut uint8_t,
        size: size_t,
    ) -> avifBool;
    fn avifROStreamReadU16(stream: *mut avifROStream, v: *mut uint16_t) -> avifBool;
    fn avifROStreamReadU32(stream: *mut avifROStream, v: *mut uint32_t) -> avifBool;
    fn avifROStreamReadUX8(
        stream: *mut avifROStream,
        v: *mut uint64_t,
        factor: uint64_t,
    ) -> avifBool;
    fn avifROStreamReadU64(stream: *mut avifROStream, v: *mut uint64_t) -> avifBool;
    fn avifROStreamReadString(
        stream: *mut avifROStream,
        output: *mut core::ffi::c_char,
        outputSize: size_t,
    ) -> avifBool;
    fn avifROStreamReadBoxHeader(
        stream: *mut avifROStream,
        header: *mut avifBoxHeader,
    ) -> avifBool;
    fn avifROStreamReadBoxHeaderPartial(
        stream: *mut avifROStream,
        header: *mut avifBoxHeader,
        topLevel: avifBool,
    ) -> avifBool;
    fn avifROStreamReadVersionAndFlags(
        stream: *mut avifROStream,
        version: *mut uint8_t,
        flags: *mut uint32_t,
    ) -> avifBool;
    fn avifROStreamReadAndEnforceVersion(
        stream: *mut avifROStream,
        enforcedVersion: uint8_t,
        flags: *mut uint32_t,
    ) -> avifBool;
    fn avifROStreamReadBitsU8(
        stream: *mut avifROStream,
        v: *mut uint8_t,
        bitCount: size_t,
    ) -> avifBool;
    fn avifROStreamReadBitsU16(
        stream: *mut avifROStream,
        v: *mut uint16_t,
        bitCount: size_t,
    ) -> avifBool;
    fn avifROStreamReadBitsU32(
        stream: *mut avifROStream,
        v: *mut uint32_t,
        bitCount: size_t,
    ) -> avifBool;
    fn avifSequenceHeaderParse(
        header: *mut avifSequenceHeader,
        sample: *const avifROData,
        codecType: avifCodecType,
    ) -> avifBool;
    fn avifGainMapSetDefaults(gainMap: *mut avifGainMap);
    fn avifGainMapValidateMetadata(
        gainMap: *const avifGainMap,
        diag: *mut avifDiagnostics,
    ) -> avifResult;
    fn __assert_fail(
        __assertion: *const core::ffi::c_char,
        __file: *const core::ffi::c_char,
        __line: core::ffi::c_uint,
        __function: *const core::ffi::c_char,
    ) -> !;
    fn __ctype_b_loc() -> *mut *const core::ffi::c_ushort;
    fn lround(__x: core::ffi::c_double) -> core::ffi::c_long;
    fn floorf(__x: core::ffi::c_float) -> core::ffi::c_float;
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
    fn memcmp(
        __s1: *const core::ffi::c_void,
        __s2: *const core::ffi::c_void,
        __n: size_t,
    ) -> core::ffi::c_int;
    fn strcmp(
        __s1: *const core::ffi::c_char,
        __s2: *const core::ffi::c_char,
    ) -> core::ffi::c_int;
    fn snprintf(
        __s: *mut core::ffi::c_char,
        __maxlen: size_t,
        __format: *const core::ffi::c_char,
        ...
    ) -> core::ffi::c_int;
}
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
pub type avifTransformFlag = core::ffi::c_uint;
pub const AVIF_TRANSFORM_IMIR: avifTransformFlag = 8;
pub const AVIF_TRANSFORM_IROT: avifTransformFlag = 4;
pub const AVIF_TRANSFORM_CLAP: avifTransformFlag = 2;
pub const AVIF_TRANSFORM_PASP: avifTransformFlag = 1;
pub const AVIF_TRANSFORM_NONE: avifTransformFlag = 0;
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
pub struct avifCropRect {
    pub x: uint32_t,
    pub y: uint32_t,
    pub width: uint32_t,
    pub height: uint32_t,
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
pub type avifCodecFlag = core::ffi::c_uint;
pub const AVIF_CODEC_FLAG_CAN_ENCODE: avifCodecFlag = 2;
pub const AVIF_CODEC_FLAG_CAN_DECODE: avifCodecFlag = 1;
pub type avifCodecFlags = uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct avifIO {
    pub destroy: avifIODestroyFunc,
    pub read: avifIOReadFunc,
    pub write: avifIOWriteFunc,
    pub sizeHint: uint64_t,
    pub persistent: avifBool,
    pub data: *mut core::ffi::c_void,
}
pub type avifIOWriteFunc = Option<
    unsafe extern "C" fn(
        *mut avifIO,
        uint32_t,
        uint64_t,
        *const uint8_t,
        size_t,
    ) -> avifResult,
>;
pub type avifIOReadFunc = Option<
    unsafe extern "C" fn(
        *mut avifIO,
        uint32_t,
        uint64_t,
        size_t,
        *mut avifROData,
    ) -> avifResult,
>;
pub type avifIODestroyFunc = Option<unsafe extern "C" fn(*mut avifIO) -> ()>;
pub type avifStrictFlag = core::ffi::c_uint;
pub const AVIF_STRICT_ENABLED: avifStrictFlag = 7;
pub const AVIF_STRICT_ALPHA_ISPE_REQUIRED: avifStrictFlag = 4;
pub const AVIF_STRICT_CLAP_VALID: avifStrictFlag = 2;
pub const AVIF_STRICT_PIXI_REQUIRED: avifStrictFlag = 1;
pub const AVIF_STRICT_DISABLED: avifStrictFlag = 0;
pub type avifStrictFlags = uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct avifIOStats {
    pub colorOBUSize: size_t,
    pub alphaOBUSize: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct avifDecoderData {
    pub meta: *mut avifMeta,
    pub tracks: avifTrackArray,
    pub tiles: avifTileArray,
    pub tileInfos: [avifTileInfo; 8],
    pub source: avifDecoderSource,
    pub codec: *mut avifCodec,
    pub codecAlpha: *mut avifCodec,
    pub majorBrand: [uint8_t; 4],
    pub compatibleBrands: avifBrandArray,
    pub diag: *mut avifDiagnostics,
    pub sourceSampleTable: *const avifSampleTable,
    pub cicpSet: avifBool,
    pub sampleTransformNumInputImageItems: uint8_t,
    pub sampleTransformInputImageItems: [avifItemCategory; 3],
}
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct avifSampleTable {
    pub chunks: avifSampleTableChunkArray,
    pub sampleDescriptions: avifSampleDescriptionArray,
    pub sampleToChunks: avifSampleTableSampleToChunkArray,
    pub sampleSizes: avifSampleTableSampleSizeArray,
    pub timeToSamples: avifSampleTableTimeToSampleArray,
    pub syncSamples: avifSyncSampleArray,
    pub allSamplesSize: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct avifSyncSampleArray {
    pub syncSample: *mut avifSyncSample,
    pub elementSize: uint32_t,
    pub count: uint32_t,
    pub capacity: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct avifSyncSample {
    pub sampleNumber: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct avifSampleTableTimeToSampleArray {
    pub timeToSample: *mut avifSampleTableTimeToSample,
    pub elementSize: uint32_t,
    pub count: uint32_t,
    pub capacity: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct avifSampleTableTimeToSample {
    pub sampleCount: uint32_t,
    pub sampleDelta: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct avifSampleTableSampleSizeArray {
    pub sampleSize: *mut avifSampleTableSampleSize,
    pub elementSize: uint32_t,
    pub count: uint32_t,
    pub capacity: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct avifSampleTableSampleSize {
    pub size: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct avifSampleTableSampleToChunkArray {
    pub sampleToChunk: *mut avifSampleTableSampleToChunk,
    pub elementSize: uint32_t,
    pub count: uint32_t,
    pub capacity: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct avifSampleTableSampleToChunk {
    pub firstChunk: uint32_t,
    pub samplesPerChunk: uint32_t,
    pub sampleDescriptionIndex: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct avifSampleDescriptionArray {
    pub description: *mut avifSampleDescription,
    pub elementSize: uint32_t,
    pub count: uint32_t,
    pub capacity: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct avifSampleDescription {
    pub format: [uint8_t; 4],
    pub properties: avifPropertyArray,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct avifPropertyArray {
    pub prop: *mut avifProperty,
    pub elementSize: uint32_t,
    pub count: uint32_t,
    pub capacity: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct avifProperty {
    pub type_0: [uint8_t; 4],
    pub isOpaque: avifBool,
    pub u: C2RustUnnamed_2,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_2 {
    pub ispe: avifImageSpatialExtents,
    pub auxC: avifAuxiliaryType,
    pub colr: avifColourInformationBox,
    pub av1C: avifCodecConfigurationBox,
    pub pasp: avifPixelAspectRatioBox,
    pub clap: avifCleanApertureBox,
    pub irot: avifImageRotation,
    pub imir: avifImageMirror,
    pub pixi: avifPixelInformationProperty,
    pub a1op: avifOperatingPointSelectorProperty,
    pub lsel: avifLayerSelectorProperty,
    pub a1lx: avifAV1LayeredImageIndexingProperty,
    pub clli: avifContentLightLevelInformationBox,
    pub opaque: avifOpaqueProperty,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct avifOpaqueProperty {
    pub usertype: [uint8_t; 16],
    pub boxPayload: avifRWData,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct avifAV1LayeredImageIndexingProperty {
    pub layerSize: [uint32_t; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct avifLayerSelectorProperty {
    pub layerID: uint16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct avifOperatingPointSelectorProperty {
    pub opIndex: uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct avifPixelInformationProperty {
    pub planeDepths: [uint8_t; 4],
    pub planeCount: uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct avifCodecConfigurationBox {
    pub seqProfile: uint8_t,
    pub seqLevelIdx0: uint8_t,
    pub seqTier0: uint8_t,
    pub highBitdepth: uint8_t,
    pub twelveBit: uint8_t,
    pub monochrome: uint8_t,
    pub chromaSubsamplingX: uint8_t,
    pub chromaSubsamplingY: uint8_t,
    pub chromaSamplePosition: uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct avifColourInformationBox {
    pub hasICC: avifBool,
    pub iccOffset: uint64_t,
    pub iccSize: size_t,
    pub hasNCLX: avifBool,
    pub colorPrimaries: avifColorPrimaries,
    pub transferCharacteristics: avifTransferCharacteristics,
    pub matrixCoefficients: avifMatrixCoefficients,
    pub range: avifRange,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct avifAuxiliaryType {
    pub auxType: [core::ffi::c_char; 64],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct avifImageSpatialExtents {
    pub width: uint32_t,
    pub height: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct avifSampleTableChunkArray {
    pub chunk: *mut avifSampleTableChunk,
    pub elementSize: uint32_t,
    pub count: uint32_t,
    pub capacity: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct avifSampleTableChunk {
    pub offset: uint64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct avifBrandArray {
    pub brand: *mut avifBrand,
    pub elementSize: uint32_t,
    pub count: uint32_t,
    pub capacity: uint32_t,
}
pub type avifBrand = [uint8_t; 4];
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
pub type avifDecoderSource = core::ffi::c_uint;
pub const AVIF_DECODER_SOURCE_TRACKS: avifDecoderSource = 2;
pub const AVIF_DECODER_SOURCE_PRIMARY_ITEM: avifDecoderSource = 1;
pub const AVIF_DECODER_SOURCE_AUTO: avifDecoderSource = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct avifTileInfo {
    pub tileCount: core::ffi::c_uint,
    pub decodedTileCount: core::ffi::c_uint,
    pub firstTileIndex: core::ffi::c_uint,
    pub grid: avifImageGrid,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct avifImageGrid {
    pub rows: uint32_t,
    pub columns: uint32_t,
    pub outputWidth: uint32_t,
    pub outputHeight: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct avifTileArray {
    pub tile: *mut avifTile,
    pub elementSize: uint32_t,
    pub count: uint32_t,
    pub capacity: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct avifTile {
    pub input: *mut avifCodecDecodeInput,
    pub codecType: avifCodecType,
    pub codec: *mut avifCodec,
    pub image: *mut avifImage,
    pub width: uint32_t,
    pub height: uint32_t,
    pub operatingPoint: uint8_t,
}
pub type avifCodecType = core::ffi::c_uint;
pub const AVIF_CODEC_TYPE_AV1: avifCodecType = 1;
pub const AVIF_CODEC_TYPE_UNKNOWN: avifCodecType = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct avifCodecDecodeInput {
    pub samples: avifDecodeSampleArray,
    pub allLayers: avifBool,
    pub itemCategory: avifItemCategory,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct avifDecodeSampleArray {
    pub sample: *mut avifDecodeSample,
    pub elementSize: uint32_t,
    pub count: uint32_t,
    pub capacity: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct avifTrackArray {
    pub track: *mut avifTrack,
    pub elementSize: uint32_t,
    pub count: uint32_t,
    pub capacity: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct avifTrack {
    pub id: uint32_t,
    pub handlerType: [uint8_t; 4],
    pub references: avifItemReferenceArray,
    pub mediaTimescale: uint32_t,
    pub mediaDuration: uint64_t,
    pub trackDuration: uint64_t,
    pub segmentDuration: uint64_t,
    pub isRepeating: avifBool,
    pub repetitionCount: core::ffi::c_int,
    pub width: uint32_t,
    pub height: uint32_t,
    pub sampleTable: *mut avifSampleTable,
    pub meta: *mut avifMeta,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct avifMeta {
    pub items: avifDecoderItemArray,
    pub properties: avifPropertyArray,
    pub idat: avifRWData,
    pub idatID: uint32_t,
    pub primaryItemID: uint32_t,
    pub entityToGroups: avifEntityToGroups,
    pub sampleTransformExpression: avifSampleTransformExpression,
    pub sampleTransformDepth: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct avifSampleTransformExpression {
    pub tokens: *mut avifSampleTransformToken,
    pub elementSize: uint32_t,
    pub count: uint32_t,
    pub capacity: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct avifSampleTransformToken {
    pub type_0: avifSampleTransformTokenType,
    pub constant: int32_t,
    pub inputImageItemIndex: uint8_t,
}
pub type avifSampleTransformTokenType = core::ffi::c_uint;
pub const AVIF_SAMPLE_TRANSFORM_RESERVED: avifSampleTransformTokenType = 138;
pub const AVIF_SAMPLE_TRANSFORM_LAST_BINARY_OPERATOR: avifSampleTransformTokenType = 137;
pub const AVIF_SAMPLE_TRANSFORM_MAX: avifSampleTransformTokenType = 137;
pub const AVIF_SAMPLE_TRANSFORM_MIN: avifSampleTransformTokenType = 136;
pub const AVIF_SAMPLE_TRANSFORM_POW: avifSampleTransformTokenType = 135;
pub const AVIF_SAMPLE_TRANSFORM_XOR: avifSampleTransformTokenType = 134;
pub const AVIF_SAMPLE_TRANSFORM_OR: avifSampleTransformTokenType = 133;
pub const AVIF_SAMPLE_TRANSFORM_AND: avifSampleTransformTokenType = 132;
pub const AVIF_SAMPLE_TRANSFORM_QUOTIENT: avifSampleTransformTokenType = 131;
pub const AVIF_SAMPLE_TRANSFORM_PRODUCT: avifSampleTransformTokenType = 130;
pub const AVIF_SAMPLE_TRANSFORM_DIFFERENCE: avifSampleTransformTokenType = 129;
pub const AVIF_SAMPLE_TRANSFORM_SUM: avifSampleTransformTokenType = 128;
pub const AVIF_SAMPLE_TRANSFORM_FIRST_BINARY_OPERATOR: avifSampleTransformTokenType = 128;
pub const AVIF_SAMPLE_TRANSFORM_LAST_UNARY_OPERATOR: avifSampleTransformTokenType = 67;
pub const AVIF_SAMPLE_TRANSFORM_BSR: avifSampleTransformTokenType = 67;
pub const AVIF_SAMPLE_TRANSFORM_NOT: avifSampleTransformTokenType = 66;
pub const AVIF_SAMPLE_TRANSFORM_ABSOLUTE: avifSampleTransformTokenType = 65;
pub const AVIF_SAMPLE_TRANSFORM_NEGATION: avifSampleTransformTokenType = 64;
pub const AVIF_SAMPLE_TRANSFORM_FIRST_UNARY_OPERATOR: avifSampleTransformTokenType = 64;
pub const AVIF_SAMPLE_TRANSFORM_LAST_INPUT_IMAGE_ITEM_INDEX: avifSampleTransformTokenType = 32;
pub const AVIF_SAMPLE_TRANSFORM_FIRST_INPUT_IMAGE_ITEM_INDEX: avifSampleTransformTokenType = 1;
pub const AVIF_SAMPLE_TRANSFORM_INPUT_IMAGE_ITEM_INDEX: avifSampleTransformTokenType = 1;
pub const AVIF_SAMPLE_TRANSFORM_CONSTANT: avifSampleTransformTokenType = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct avifEntityToGroups {
    pub groups: *mut avifEntityToGroup,
    pub elementSize: uint32_t,
    pub count: uint32_t,
    pub capacity: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct avifEntityToGroup {
    pub groupingType: [uint8_t; 4],
    pub groupID: uint32_t,
    pub entityIDs: avifCodecEntityIDs,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct avifCodecEntityIDs {
    pub ids: *mut uint32_t,
    pub elementSize: uint32_t,
    pub count: uint32_t,
    pub capacity: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct avifDecoderItemArray {
    pub item: *mut *mut avifDecoderItem,
    pub elementSize: uint32_t,
    pub count: uint32_t,
    pub capacity: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct avifDecoderItem {
    pub id: uint32_t,
    pub meta: *mut avifMeta,
    pub type_0: [uint8_t; 4],
    pub size: size_t,
    pub idatStored: avifBool,
    pub width: uint32_t,
    pub height: uint32_t,
    pub contentType: avifContentType,
    pub properties: avifPropertyArray,
    pub extents: avifExtentArray,
    pub mergedExtents: avifRWData,
    pub ownsMergedExtents: avifBool,
    pub partialMergedExtents: avifBool,
    pub hasDimgFrom: avifBool,
    pub dimgInputs: avifItemIDArray,
    pub references: avifItemReferenceArray,
    pub hasUnsupportedEssentialProperty: avifBool,
    pub ipmaSeen: avifBool,
    pub progressive: avifBool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct avifItemReferenceArray {
    pub ref_0: *mut avifItemReference,
    pub elementSize: uint32_t,
    pub count: uint32_t,
    pub capacity: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct avifItemReference {
    pub type_0: [uint8_t; 4],
    pub toID: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct avifItemIDArray {
    pub id: *mut uint32_t,
    pub elementSize: uint32_t,
    pub count: uint32_t,
    pub capacity: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct avifExtentArray {
    pub extent: *mut avifExtent,
    pub elementSize: uint32_t,
    pub count: uint32_t,
    pub capacity: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct avifExtent {
    pub offset: uint64_t,
    pub size: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct avifContentType {
    pub contentType: [core::ffi::c_char; 64],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct avifImageTiming {
    pub timescale: uint64_t,
    pub pts: core::ffi::c_double,
    pub ptsInTimescales: uint64_t,
    pub duration: core::ffi::c_double,
    pub durationInTimescales: uint64_t,
}
pub type avifProgressiveState = core::ffi::c_uint;
pub const AVIF_PROGRESSIVE_STATE_ACTIVE: avifProgressiveState = 2;
pub const AVIF_PROGRESSIVE_STATE_AVAILABLE: avifProgressiveState = 1;
pub const AVIF_PROGRESSIVE_STATE_UNAVAILABLE: avifProgressiveState = 0;
pub type avifImageContentTypeFlag = core::ffi::c_uint;
pub const AVIF_IMAGE_CONTENT_DECODE_DEFAULT: avifImageContentTypeFlag = 3;
pub const AVIF_IMAGE_CONTENT_SAMPLE_TRANSFORMS: avifImageContentTypeFlag = 8;
pub const AVIF_IMAGE_CONTENT_ALL: avifImageContentTypeFlag = 7;
pub const AVIF_IMAGE_CONTENT_GAIN_MAP: avifImageContentTypeFlag = 4;
pub const AVIF_IMAGE_CONTENT_COLOR_AND_ALPHA: avifImageContentTypeFlag = 3;
pub const AVIF_IMAGE_CONTENT_ALPHA: avifImageContentTypeFlag = 2;
pub const AVIF_IMAGE_CONTENT_COLOR: avifImageContentTypeFlag = 1;
pub const AVIF_IMAGE_CONTENT_NONE: avifImageContentTypeFlag = 0;
pub type avifImageContentTypeFlags = uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct avifDecoder {
    pub codecChoice: avifCodecChoice,
    pub maxThreads: core::ffi::c_int,
    pub requestedSource: avifDecoderSource,
    pub allowProgressive: avifBool,
    pub allowIncremental: avifBool,
    pub ignoreExif: avifBool,
    pub ignoreXMP: avifBool,
    pub imageSizeLimit: uint32_t,
    pub imageDimensionLimit: uint32_t,
    pub imageCountLimit: uint32_t,
    pub strictFlags: avifStrictFlags,
    pub image: *mut avifImage,
    pub imageIndex: core::ffi::c_int,
    pub imageCount: core::ffi::c_int,
    pub progressiveState: avifProgressiveState,
    pub imageTiming: avifImageTiming,
    pub timescale: uint64_t,
    pub duration: core::ffi::c_double,
    pub durationInTimescales: uint64_t,
    pub repetitionCount: core::ffi::c_int,
    pub alphaPresent: avifBool,
    pub ioStats: avifIOStats,
    pub diag: avifDiagnostics,
    pub io: *mut avifIO,
    pub data: *mut avifDecoderData,
    pub imageSequenceTrackPresent: avifBool,
    pub imageContentToDecode: avifImageContentTypeFlags,
    pub ignoreICC: avifBool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct avifSequenceHeader {
    pub reduced_still_picture_header: uint8_t,
    pub maxWidth: uint32_t,
    pub maxHeight: uint32_t,
    pub bitDepth: uint32_t,
    pub yuvFormat: avifPixelFormat,
    pub chromaSamplePosition: avifChromaSamplePosition,
    pub colorPrimaries: avifColorPrimaries,
    pub transferCharacteristics: avifTransferCharacteristics,
    pub matrixCoefficients: avifMatrixCoefficients,
    pub range: avifRange,
    pub av1C: avifCodecConfigurationBox,
}
pub const _ISprint: C2RustUnnamed_3 = 16384;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct avifROStream {
    pub raw: *mut avifROData,
    pub offset: size_t,
    pub numUsedBitsInPartialByte: size_t,
    pub diag: *mut avifDiagnostics,
    pub diagContext: *const core::ffi::c_char,
}
pub const AVIF_SAMPLE_TRANSFORM_BIT_DEPTH_32: avifSampleTransformBitDepth = 2;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct avifBoxHeader {
    pub isSizeZeroBox: avifBool,
    pub size: size_t,
    pub type_0: [uint8_t; 4],
    pub usertype: [uint8_t; 16],
}
pub type avifUniqueBoxFlag = core::ffi::c_uint;
pub const AVIF_UNIQUE_GRPL: avifUniqueBoxFlag = 6;
pub const AVIF_UNIQUE_IREF: avifUniqueBoxFlag = 5;
pub const AVIF_UNIQUE_IINF: avifUniqueBoxFlag = 4;
pub const AVIF_UNIQUE_IPRP: avifUniqueBoxFlag = 3;
pub const AVIF_UNIQUE_IDAT: avifUniqueBoxFlag = 2;
pub const AVIF_UNIQUE_PITM: avifUniqueBoxFlag = 1;
pub const AVIF_UNIQUE_ILOC: avifUniqueBoxFlag = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct avifFileType {
    pub majorBrand: [uint8_t; 4],
    pub minorVersion: [uint8_t; 4],
    pub compatibleBrands: *const uint8_t,
    pub compatibleBrandsCount: core::ffi::c_int,
}
pub type avifSampleTransformBitDepth = core::ffi::c_uint;
pub const AVIF_SAMPLE_TRANSFORM_BIT_DEPTH_64: avifSampleTransformBitDepth = 3;
pub const AVIF_SAMPLE_TRANSFORM_BIT_DEPTH_16: avifSampleTransformBitDepth = 1;
pub const AVIF_SAMPLE_TRANSFORM_BIT_DEPTH_8: avifSampleTransformBitDepth = 0;
pub type C2RustUnnamed_3 = core::ffi::c_uint;
pub const _ISalnum: C2RustUnnamed_3 = 8;
pub const _ISpunct: C2RustUnnamed_3 = 4;
pub const _IScntrl: C2RustUnnamed_3 = 2;
pub const _ISblank: C2RustUnnamed_3 = 1;
pub const _ISgraph: C2RustUnnamed_3 = 32768;
pub const _ISspace: C2RustUnnamed_3 = 8192;
pub const _ISxdigit: C2RustUnnamed_3 = 4096;
pub const _ISdigit: C2RustUnnamed_3 = 2048;
pub const _ISalpha: C2RustUnnamed_3 = 1024;
pub const _ISlower: C2RustUnnamed_3 = 512;
pub const _ISupper: C2RustUnnamed_3 = 256;
pub const NULL: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
pub const UINT32_MAX: core::ffi::c_uint = 4294967295 as core::ffi::c_uint;
pub const UINT64_MAX: core::ffi::c_ulong = 18446744073709551615 as core::ffi::c_ulong;
pub const SIZE_MAX: core::ffi::c_ulong = 18446744073709551615 as core::ffi::c_ulong;
pub const AVIF_TRUE: core::ffi::c_int = 1 as core::ffi::c_int;
pub const AVIF_FALSE: core::ffi::c_int = 0 as core::ffi::c_int;
pub const AVIF_DEFAULT_IMAGE_SIZE_LIMIT: core::ffi::c_int = 16384 as core::ffi::c_int
    * 16384 as core::ffi::c_int;
pub const AVIF_DEFAULT_IMAGE_DIMENSION_LIMIT: core::ffi::c_int = 32768
    as core::ffi::c_int;
pub const AVIF_DEFAULT_IMAGE_COUNT_LIMIT: core::ffi::c_int = 12 as core::ffi::c_int
    * 3600 as core::ffi::c_int * 60 as core::ffi::c_int;
pub const AVIF_REPETITION_COUNT_INFINITE: core::ffi::c_int = -(1 as core::ffi::c_int);
pub const AVIF_REPETITION_COUNT_UNKNOWN: core::ffi::c_int = -(2 as core::ffi::c_int);
pub const AVIF_MAX_AV1_LAYER_COUNT: core::ffi::c_int = 4 as core::ffi::c_int;
pub const AVIF_URN_ALPHA0: [core::ffi::c_char; 44] = unsafe {
    ::core::mem::transmute::<
        [u8; 44],
        [core::ffi::c_char; 44],
    >(*b"urn:mpeg:mpegB:cicp:systems:auxiliary:alpha\0")
};
pub const AVIF_URN_ALPHA1: [core::ffi::c_char; 27] = unsafe {
    ::core::mem::transmute::<
        [u8; 27],
        [core::ffi::c_char; 27],
    >(*b"urn:mpeg:hevc:2015:auxid:1\0")
};
pub const AVIF_CONTENT_TYPE_XMP: [core::ffi::c_char; 20] = unsafe {
    ::core::mem::transmute::<
        [u8; 20],
        [core::ffi::c_char; 20],
    >(*b"application/rdf+xml\0")
};
pub const AVIF_SPATIAL_ID_UNSET: core::ffi::c_int = 0xff as core::ffi::c_int;
pub const AVIF_INDEFINITE_DURATION64: core::ffi::c_ulong = UINT64_MAX;
pub const AVIF_INDEFINITE_DURATION32: core::ffi::c_uint = UINT32_MAX;
pub const INT_MAX: core::ffi::c_int = __INT_MAX__;
static mut VISUALSAMPLEENTRY_SIZE: size_t = 78 as size_t;
pub const MAX_IPMA_VERSION_AND_FLAGS_SEEN: core::ffi::c_int = 4 as core::ffi::c_int;
unsafe extern "C" fn avifGetCodecType(mut fourcc: *const uint8_t) -> avifCodecType {
    if memcmp(
        fourcc as *const core::ffi::c_void,
        b"av01\0" as *const u8 as *const core::ffi::c_char as *const core::ffi::c_void,
        4 as size_t,
    ) == 0
    {
        return AVIF_CODEC_TYPE_AV1;
    }
    return AVIF_CODEC_TYPE_UNKNOWN;
}
unsafe extern "C" fn avifGetConfigurationPropertyName(
    mut codecType: avifCodecType,
) -> *const core::ffi::c_char {
    static mut kUnknown: [core::ffi::c_char; 5] = unsafe {
        ::core::mem::transmute::<[u8; 5], [core::ffi::c_char; 5]>(*b"****\0")
    };
    match codecType as core::ffi::c_uint {
        1 => return b"av1C\0" as *const u8 as *const core::ffi::c_char,
        _ => {
            'c_3300: {
                __assert_fail(
                    b"AVIF_FALSE\0" as *const u8 as *const core::ffi::c_char,
                    b"/home/runner/work/libavif-rs/libavif-rs/upstream/libavif/src/read.c\0"
                        as *const u8 as *const core::ffi::c_char,
                    68 as core::ffi::c_uint,
                    (::core::mem::transmute::<
                        [u8; 60],
                        [core::ffi::c_char; 60],
                    >(*b"const char *avifGetConfigurationPropertyName(avifCodecType)\0"))
                        .as_ptr(),
                );
            };
            return core::ptr::addr_of!(kUnknown).cast::<core::ffi::c_char>();
        }
    };
}
pub const MAX_PIXI_PLANE_DEPTHS: core::ffi::c_int = 4 as core::ffi::c_int;
unsafe extern "C" fn avifPropertyArrayFind(
    mut properties: *const avifPropertyArray,
    mut type_0: *const core::ffi::c_char,
) -> *const avifProperty {
    let mut propertyIndex: uint32_t = 0 as uint32_t;
    while propertyIndex < (*properties).count {
        let mut prop: *const avifProperty = &mut *((*properties).prop)
            .offset(propertyIndex as isize) as *mut avifProperty;
        if memcmp(
            ((*prop).type_0).as_ptr() as *const core::ffi::c_void,
            type_0 as *const core::ffi::c_void,
            4 as size_t,
        ) == 0
        {
            return prop;
        }
        propertyIndex = propertyIndex.wrapping_add(1);
    }
    return 0 as *const avifProperty;
}
unsafe extern "C" fn avifIsDimgInput(
    mut derivedItem: *const avifDecoderItem,
    mut inputItemID: uint32_t,
    mut dimgIdx: *mut uint32_t,
) -> avifBool {
    let mut i: uint32_t = 0 as uint32_t;
    while i < (*derivedItem).dimgInputs.count {
        if *((*derivedItem).dimgInputs.id).offset(i as isize) == inputItemID {
            if !dimgIdx.is_null() {
                *dimgIdx = i;
            }
            return AVIF_TRUE;
        }
        i = i.wrapping_add(1);
    }
    return AVIF_FALSE;
}
unsafe extern "C" fn avifAddDimgInput(
    mut derivedItem: *mut avifDecoderItem,
    mut inputItem: *mut avifDecoderItem,
) -> avifResult {
    let mut dimgInput: *mut uint32_t = avifArrayPush(
        &mut (*derivedItem).dimgInputs as *mut avifItemIDArray as *mut core::ffi::c_void,
    ) as *mut uint32_t;
    if dimgInput.is_null() {
        return AVIF_RESULT_OUT_OF_MEMORY;
    }
    *dimgInput = (*inputItem).id;
    return AVIF_RESULT_OK;
}
unsafe extern "C" fn avifAddReference(
    mut references: *mut avifItemReferenceArray,
    mut type_0: *const core::ffi::c_char,
    mut toID: uint32_t,
) -> avifResult {
    let mut reference: *mut avifItemReference = avifArrayPush(
        references as *mut core::ffi::c_void,
    ) as *mut avifItemReference;
    if reference.is_null() {
        return AVIF_RESULT_OUT_OF_MEMORY;
    }
    memcpy(
        ((*reference).type_0).as_mut_ptr() as *mut core::ffi::c_void,
        type_0 as *const core::ffi::c_void,
        4 as size_t,
    );
    (*reference).toID = toID;
    return AVIF_RESULT_OK;
}
unsafe extern "C" fn avifReferences(
    mut references: *const avifItemReferenceArray,
    mut type_0: *const core::ffi::c_char,
    mut toID: uint32_t,
) -> avifBool {
    let mut i: uint32_t = 0 as uint32_t;
    while i < (*references).count {
        let mut reference: *const avifItemReference = &mut *((*references).ref_0)
            .offset(i as isize) as *mut avifItemReference;
        if memcmp(
            ((*reference).type_0).as_ptr() as *const core::ffi::c_void,
            type_0 as *const core::ffi::c_void,
            4 as size_t,
        ) == 0 && (*reference).toID == toID
        {
            return AVIF_TRUE;
        }
        i = i.wrapping_add(1);
    }
    return AVIF_FALSE;
}
unsafe extern "C" fn avifHasReference(
    mut references: *const avifItemReferenceArray,
    mut type_0: *const core::ffi::c_char,
) -> avifBool {
    let mut i: uint32_t = 0 as uint32_t;
    while i < (*references).count {
        if memcmp(
            ((*((*references).ref_0).offset(i as isize)).type_0).as_mut_ptr()
                as *const core::ffi::c_void,
            type_0 as *const core::ffi::c_void,
            4 as size_t,
        ) == 0
        {
            return AVIF_TRUE;
        }
        i = i.wrapping_add(1);
    }
    return AVIF_FALSE;
}
unsafe extern "C" fn avifSampleTableCreate() -> *mut avifSampleTable {
    let mut sampleTable: *mut avifSampleTable = avifAlloc(
        ::core::mem::size_of::<avifSampleTable>() as size_t,
    ) as *mut avifSampleTable;
    if sampleTable.is_null() {
        return 0 as *mut avifSampleTable;
    }
    memset(
        sampleTable as *mut core::ffi::c_void,
        0 as core::ffi::c_int,
        ::core::mem::size_of::<avifSampleTable>() as size_t,
    );
    if avifArrayCreate(
        &mut (*sampleTable).chunks as *mut avifSampleTableChunkArray
            as *mut core::ffi::c_void,
        ::core::mem::size_of::<avifSampleTableChunk>() as uint32_t,
        16 as uint32_t,
    ) == 0
        || avifArrayCreate(
            &mut (*sampleTable).sampleDescriptions as *mut avifSampleDescriptionArray
                as *mut core::ffi::c_void,
            ::core::mem::size_of::<avifSampleDescription>() as uint32_t,
            2 as uint32_t,
        ) == 0
        || avifArrayCreate(
            &mut (*sampleTable).sampleToChunks as *mut avifSampleTableSampleToChunkArray
                as *mut core::ffi::c_void,
            ::core::mem::size_of::<avifSampleTableSampleToChunk>() as uint32_t,
            16 as uint32_t,
        ) == 0
        || avifArrayCreate(
            &mut (*sampleTable).sampleSizes as *mut avifSampleTableSampleSizeArray
                as *mut core::ffi::c_void,
            ::core::mem::size_of::<avifSampleTableSampleSize>() as uint32_t,
            16 as uint32_t,
        ) == 0
        || avifArrayCreate(
            &mut (*sampleTable).timeToSamples as *mut avifSampleTableTimeToSampleArray
                as *mut core::ffi::c_void,
            ::core::mem::size_of::<avifSampleTableTimeToSample>() as uint32_t,
            16 as uint32_t,
        ) == 0
        || avifArrayCreate(
            &mut (*sampleTable).syncSamples as *mut avifSyncSampleArray
                as *mut core::ffi::c_void,
            ::core::mem::size_of::<avifSyncSample>() as uint32_t,
            16 as uint32_t,
        ) == 0
    {
        avifSampleTableDestroy(sampleTable);
        return 0 as *mut avifSampleTable;
    }
    return sampleTable;
}
unsafe extern "C" fn avifPropertyArrayDestroy(mut array: *mut avifPropertyArray) {
    let mut i: size_t = 0 as size_t;
    while i < (*array).count as size_t {
        if (*((*array).prop).offset(i as isize)).isOpaque != 0 {
            avifRWDataFree(
                &mut (*((*array).prop).offset(i as isize)).u.opaque.boxPayload,
            );
        }
        i = i.wrapping_add(1);
    }
    avifArrayDestroy(array as *mut core::ffi::c_void);
}
unsafe extern "C" fn avifSampleTableDestroy(mut sampleTable: *mut avifSampleTable) {
    avifArrayDestroy(
        &mut (*sampleTable).chunks as *mut avifSampleTableChunkArray
            as *mut core::ffi::c_void,
    );
    let mut i: uint32_t = 0 as uint32_t;
    while i < (*sampleTable).sampleDescriptions.count {
        let mut description: *mut avifSampleDescription = &mut *((*sampleTable)
            .sampleDescriptions
            .description)
            .offset(i as isize) as *mut avifSampleDescription;
        avifPropertyArrayDestroy(&mut (*description).properties);
        i = i.wrapping_add(1);
    }
    avifArrayDestroy(
        &mut (*sampleTable).sampleDescriptions as *mut avifSampleDescriptionArray
            as *mut core::ffi::c_void,
    );
    avifArrayDestroy(
        &mut (*sampleTable).sampleToChunks as *mut avifSampleTableSampleToChunkArray
            as *mut core::ffi::c_void,
    );
    avifArrayDestroy(
        &mut (*sampleTable).sampleSizes as *mut avifSampleTableSampleSizeArray
            as *mut core::ffi::c_void,
    );
    avifArrayDestroy(
        &mut (*sampleTable).timeToSamples as *mut avifSampleTableTimeToSampleArray
            as *mut core::ffi::c_void,
    );
    avifArrayDestroy(
        &mut (*sampleTable).syncSamples as *mut avifSyncSampleArray
            as *mut core::ffi::c_void,
    );
    avifFree(sampleTable as *mut core::ffi::c_void);
}
unsafe extern "C" fn avifSampleTableGetImageDelta(
    mut sampleTable: *const avifSampleTable,
    mut imageIndex: uint32_t,
) -> uint32_t {
    let mut maxSampleIndex: uint32_t = 0 as uint32_t;
    let mut i: uint32_t = 0 as uint32_t;
    while i < (*sampleTable).timeToSamples.count {
        let mut timeToSample: *const avifSampleTableTimeToSample = &mut *((*sampleTable)
            .timeToSamples
            .timeToSample)
            .offset(i as isize) as *mut avifSampleTableTimeToSample;
        maxSampleIndex = maxSampleIndex.wrapping_add((*timeToSample).sampleCount);
        if imageIndex < maxSampleIndex
            || i == ((*sampleTable).timeToSamples.count).wrapping_sub(1 as uint32_t)
        {
            return (*timeToSample).sampleDelta;
        }
        i = i.wrapping_add(1);
    }
    return 1 as uint32_t;
}
unsafe extern "C" fn avifSampleTableGetCodecType(
    mut sampleTable: *const avifSampleTable,
) -> avifCodecType {
    let mut i: uint32_t = 0 as uint32_t;
    while i < (*sampleTable).sampleDescriptions.count {
        let codecType: avifCodecType = avifGetCodecType(
            ((*((*sampleTable).sampleDescriptions.description).offset(i as isize))
                .format)
                .as_mut_ptr(),
        ) as avifCodecType;
        if codecType as core::ffi::c_uint
            != AVIF_CODEC_TYPE_UNKNOWN as core::ffi::c_int as core::ffi::c_uint
        {
            return codecType;
        }
        i = i.wrapping_add(1);
    }
    return AVIF_CODEC_TYPE_UNKNOWN;
}
unsafe extern "C" fn avifCodecConfigurationBoxGetDepth(
    mut av1C: *const avifCodecConfigurationBox,
) -> uint32_t {
    if (*av1C).twelveBit != 0 {
        return 12 as uint32_t
    } else if (*av1C).highBitdepth != 0 {
        return 10 as uint32_t
    }
    return 8 as uint32_t;
}
unsafe extern "C" fn avifSampleTableGetProperties(
    mut sampleTable: *const avifSampleTable,
    mut codecType: avifCodecType,
) -> *const avifPropertyArray {
    let mut i: uint32_t = 0 as uint32_t;
    while i < (*sampleTable).sampleDescriptions.count {
        let mut description: *const avifSampleDescription = &mut *((*sampleTable)
            .sampleDescriptions
            .description)
            .offset(i as isize) as *mut avifSampleDescription;
        if avifGetCodecType(((*description).format).as_ptr()) as core::ffi::c_uint
            == codecType as core::ffi::c_uint
        {
            return &(*description).properties;
        }
        i = i.wrapping_add(1);
    }
    return 0 as *const avifPropertyArray;
}
#[no_mangle]
pub unsafe extern "C" fn avifCodecDecodeInputCreate() -> *mut avifCodecDecodeInput {
    let mut decodeInput: *mut avifCodecDecodeInput = avifAlloc(
        ::core::mem::size_of::<avifCodecDecodeInput>() as size_t,
    ) as *mut avifCodecDecodeInput;
    if decodeInput.is_null() {
        return 0 as *mut avifCodecDecodeInput;
    }
    memset(
        decodeInput as *mut core::ffi::c_void,
        0 as core::ffi::c_int,
        ::core::mem::size_of::<avifCodecDecodeInput>() as size_t,
    );
    if avifArrayCreate(
        &mut (*decodeInput).samples as *mut avifDecodeSampleArray
            as *mut core::ffi::c_void,
        ::core::mem::size_of::<avifDecodeSample>() as uint32_t,
        1 as uint32_t,
    ) == 0
    {
        avifFree(decodeInput as *mut core::ffi::c_void);
        return 0 as *mut avifCodecDecodeInput;
    }
    return decodeInput;
}
#[no_mangle]
pub unsafe extern "C" fn avifCodecDecodeInputDestroy(
    mut decodeInput: *mut avifCodecDecodeInput,
) {
    let mut sampleIndex: uint32_t = 0 as uint32_t;
    while sampleIndex < (*decodeInput).samples.count {
        let mut sample: *mut avifDecodeSample = &mut *((*decodeInput).samples.sample)
            .offset(sampleIndex as isize) as *mut avifDecodeSample;
        if (*sample).ownsData != 0 {
            avifRWDataFree(&mut (*sample).data as *mut avifROData as *mut avifRWData);
        }
        sampleIndex = sampleIndex.wrapping_add(1);
    }
    avifArrayDestroy(
        &mut (*decodeInput).samples as *mut avifDecodeSampleArray
            as *mut core::ffi::c_void,
    );
    avifFree(decodeInput as *mut core::ffi::c_void);
}
unsafe extern "C" fn avifGetSampleCountOfChunk(
    mut sampleToChunks: *const avifSampleTableSampleToChunkArray,
    mut chunkIndex: uint32_t,
) -> uint32_t {
    let mut sampleCount: uint32_t = 0 as uint32_t;
    let mut sampleToChunkIndex: core::ffi::c_int = ((*sampleToChunks).count)
        .wrapping_sub(1 as uint32_t) as core::ffi::c_int;
    while sampleToChunkIndex >= 0 as core::ffi::c_int {
        let mut sampleToChunk: *const avifSampleTableSampleToChunk = &mut *((*sampleToChunks)
            .sampleToChunk)
            .offset(sampleToChunkIndex as isize) as *mut avifSampleTableSampleToChunk;
        if (*sampleToChunk).firstChunk <= chunkIndex.wrapping_add(1 as uint32_t) {
            sampleCount = (*sampleToChunk).samplesPerChunk;
            break;
        } else {
            sampleToChunkIndex -= 1;
        }
    }
    return sampleCount;
}
unsafe extern "C" fn avifCodecDecodeInputFillFromSampleTable(
    mut decodeInput: *mut avifCodecDecodeInput,
    mut sampleTable: *mut avifSampleTable,
    imageCountLimit: uint32_t,
    sizeHint: uint64_t,
    mut diag: *mut avifDiagnostics,
) -> avifResult {
    if imageCountLimit != 0 {
        let mut imageCountLeft: uint32_t = imageCountLimit;
        let mut chunkIndex: uint32_t = 0 as uint32_t;
        while chunkIndex < (*sampleTable).chunks.count {
            let mut sampleCount: uint32_t = avifGetSampleCountOfChunk(
                &mut (*sampleTable).sampleToChunks,
                chunkIndex,
            );
            if sampleCount == 0 as uint32_t {
                avifDiagnosticsPrintf(
                    diag,
                    b"Sample table contains a chunk with 0 samples\0" as *const u8
                        as *const core::ffi::c_char,
                );
                return AVIF_RESULT_BMFF_PARSE_FAILED;
            }
            if sampleCount > imageCountLeft {
                avifDiagnosticsPrintf(
                    diag,
                    b"Exceeded avifDecoder's imageCountLimit\0" as *const u8
                        as *const core::ffi::c_char,
                );
                return AVIF_RESULT_BMFF_PARSE_FAILED;
            }
            imageCountLeft = imageCountLeft.wrapping_sub(sampleCount);
            chunkIndex = chunkIndex.wrapping_add(1);
        }
    }
    let mut sampleSizeIndex: uint32_t = 0 as uint32_t;
    let mut chunkIndex_0: uint32_t = 0 as uint32_t;
    while chunkIndex_0 < (*sampleTable).chunks.count {
        let mut chunk: *mut avifSampleTableChunk = &mut *((*sampleTable).chunks.chunk)
            .offset(chunkIndex_0 as isize) as *mut avifSampleTableChunk;
        let mut sampleCount_0: uint32_t = avifGetSampleCountOfChunk(
            &mut (*sampleTable).sampleToChunks,
            chunkIndex_0,
        );
        if sampleCount_0 == 0 as uint32_t {
            avifDiagnosticsPrintf(
                diag,
                b"Sample table contains a chunk with 0 samples\0" as *const u8
                    as *const core::ffi::c_char,
            );
            return AVIF_RESULT_BMFF_PARSE_FAILED;
        }
        let mut sampleOffset: uint64_t = (*chunk).offset;
        let mut sampleIndex: uint32_t = 0 as uint32_t;
        while sampleIndex < sampleCount_0 {
            let mut sampleSize: uint32_t = (*sampleTable).allSamplesSize;
            if sampleSize == 0 as uint32_t {
                if sampleSizeIndex >= (*sampleTable).sampleSizes.count {
                    avifDiagnosticsPrintf(
                        diag,
                        b"Truncated sample table\0" as *const u8
                            as *const core::ffi::c_char,
                    );
                    return AVIF_RESULT_BMFF_PARSE_FAILED;
                }
                let mut sampleSizePtr: *mut avifSampleTableSampleSize = &mut *((*sampleTable)
                    .sampleSizes
                    .sampleSize)
                    .offset(sampleSizeIndex as isize) as *mut avifSampleTableSampleSize;
                sampleSize = (*sampleSizePtr).size;
            }
            let mut sample: *mut avifDecodeSample = avifArrayPush(
                &mut (*decodeInput).samples as *mut avifDecodeSampleArray
                    as *mut core::ffi::c_void,
            ) as *mut avifDecodeSample;
            if sample.is_null() {
                return AVIF_RESULT_OUT_OF_MEMORY;
            }
            (*sample).offset = sampleOffset;
            (*sample).size = sampleSize as size_t;
            (*sample).spatialID = AVIF_SPATIAL_ID_UNSET as uint8_t;
            (*sample).sync = AVIF_FALSE as avifBool;
            if sampleSize as uint64_t
                > (UINT64_MAX as uint64_t).wrapping_sub(sampleOffset)
            {
                avifDiagnosticsPrintf(
                    diag,
                    b"Sample table contains an offset/size pair which overflows: [%lu / %u]\0"
                        as *const u8 as *const core::ffi::c_char,
                    sampleOffset,
                    sampleSize,
                );
                return AVIF_RESULT_BMFF_PARSE_FAILED;
            }
            if sizeHint != 0
                && sampleOffset.wrapping_add(sampleSize as uint64_t) > sizeHint
            {
                avifDiagnosticsPrintf(
                    diag,
                    b"Exceeded avifIO's sizeHint, possibly truncated data\0" as *const u8
                        as *const core::ffi::c_char,
                );
                return AVIF_RESULT_BMFF_PARSE_FAILED;
            }
            sampleOffset = sampleOffset.wrapping_add(sampleSize as uint64_t);
            sampleSizeIndex = sampleSizeIndex.wrapping_add(1);
            sampleIndex = sampleIndex.wrapping_add(1);
        }
        chunkIndex_0 = chunkIndex_0.wrapping_add(1);
    }
    let mut syncSampleIndex: uint32_t = 0 as uint32_t;
    while syncSampleIndex < (*sampleTable).syncSamples.count {
        let mut frameIndex: uint32_t = ((*((*sampleTable).syncSamples.syncSample)
            .offset(syncSampleIndex as isize))
            .sampleNumber)
            .wrapping_sub(1 as uint32_t);
        if frameIndex < (*decodeInput).samples.count {
            (*((*decodeInput).samples.sample).offset(frameIndex as isize)).sync = AVIF_TRUE
                as avifBool;
        }
        syncSampleIndex = syncSampleIndex.wrapping_add(1);
    }
    if (*decodeInput).samples.count > 0 as uint32_t {
        (*((*decodeInput).samples.sample).offset(0 as core::ffi::c_int as isize)).sync = AVIF_TRUE
            as avifBool;
    }
    return AVIF_RESULT_OK;
}
unsafe extern "C" fn avifCodecDecodeInputFillFromDecoderItem(
    mut decodeInput: *mut avifCodecDecodeInput,
    mut item: *mut avifDecoderItem,
    mut allowProgressive: avifBool,
    imageCountLimit: uint32_t,
    sizeHint: uint64_t,
    mut diag: *mut avifDiagnostics,
) -> avifResult {
    if sizeHint != 0 && (*item).size > sizeHint as size_t {
        avifDiagnosticsPrintf(
            diag,
            b"Exceeded avifIO's sizeHint, possibly truncated data\0" as *const u8
                as *const core::ffi::c_char,
        );
        return AVIF_RESULT_BMFF_PARSE_FAILED;
    }
    let mut layerCount: uint8_t = 0 as uint8_t;
    let mut layerSizes: [size_t; 4] = [0 as core::ffi::c_int as size_t, 0, 0, 0];
    let mut a1lxProp: *const avifProperty = avifPropertyArrayFind(
        &mut (*item).properties,
        b"a1lx\0" as *const u8 as *const core::ffi::c_char,
    );
    if !a1lxProp.is_null() {
        let mut remainingSize: size_t = (*item).size;
        let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
        while i < 3 as core::ffi::c_int {
            layerCount = layerCount.wrapping_add(1);
            let layerSize: size_t = (*a1lxProp).u.a1lx.layerSize[i as usize] as size_t;
            if layerSize != 0 {
                if layerSize >= remainingSize {
                    avifDiagnosticsPrintf(
                        diag,
                        b"a1lx layer index [%d] does not fit in item size\0" as *const u8
                            as *const core::ffi::c_char,
                        i,
                    );
                    return AVIF_RESULT_BMFF_PARSE_FAILED;
                }
                layerSizes[i as usize] = layerSize;
                remainingSize = remainingSize.wrapping_sub(layerSize);
                i += 1;
            } else {
                layerSizes[i as usize] = remainingSize;
                remainingSize = 0 as size_t;
                break;
            }
        }
        if remainingSize > 0 as size_t {
            'c_8897: {
                if layerCount as core::ffi::c_int == 3 as core::ffi::c_int {} else {
                    __assert_fail(
                        b"layerCount == 3\0" as *const u8 as *const core::ffi::c_char,
                        b"/home/runner/work/libavif-rs/libavif-rs/upstream/libavif/src/read.c\0"
                            as *const u8 as *const core::ffi::c_char,
                        722 as core::ffi::c_uint,
                        (::core::mem::transmute::<
                            [u8; 155],
                            [core::ffi::c_char; 155],
                        >(
                            *b"avifResult avifCodecDecodeInputFillFromDecoderItem(avifCodecDecodeInput *, avifDecoderItem *, avifBool, const uint32_t, const uint64_t, avifDiagnostics *)\0",
                        ))
                            .as_ptr(),
                    );
                }
            };
            layerCount = layerCount.wrapping_add(1);
            layerSizes[3 as core::ffi::c_int as usize] = remainingSize;
        }
    }
    let mut lselProp: *const avifProperty = avifPropertyArrayFind(
        &mut (*item).properties,
        b"lsel\0" as *const u8 as *const core::ffi::c_char,
    );
    (*item).progressive = (!a1lxProp.is_null()
        && (lselProp.is_null()
            || (*lselProp).u.lsel.layerID as core::ffi::c_int
                == 0xffff as core::ffi::c_int)) as core::ffi::c_int as avifBool;
    if !lselProp.is_null()
        && (*lselProp).u.lsel.layerID as core::ffi::c_int != 0xffff as core::ffi::c_int
    {
        (*decodeInput).allLayers = AVIF_TRUE as avifBool;
        let mut sampleSize: size_t = 0 as size_t;
        if layerCount as core::ffi::c_int > 0 as core::ffi::c_int {
            if (*lselProp).u.lsel.layerID as core::ffi::c_int
                >= layerCount as core::ffi::c_int
            {
                avifDiagnosticsPrintf(
                    diag,
                    b"lsel property requests layer index [%u] which isn't present in a1lx property ([%u] layers)\0"
                        as *const u8 as *const core::ffi::c_char,
                    (*lselProp).u.lsel.layerID as core::ffi::c_int,
                    layerCount as core::ffi::c_int,
                );
                return AVIF_RESULT_BMFF_PARSE_FAILED;
            }
            let mut i_0: uint8_t = 0 as uint8_t;
            while i_0 as core::ffi::c_int
                <= (*lselProp).u.lsel.layerID as core::ffi::c_int
            {
                sampleSize = sampleSize.wrapping_add(layerSizes[i_0 as usize]);
                i_0 = i_0.wrapping_add(1);
            }
        } else {
            sampleSize = (*item).size;
        }
        let mut sample: *mut avifDecodeSample = avifArrayPush(
            &mut (*decodeInput).samples as *mut avifDecodeSampleArray
                as *mut core::ffi::c_void,
        ) as *mut avifDecodeSample;
        if sample.is_null() {
            return AVIF_RESULT_OUT_OF_MEMORY;
        }
        (*sample).itemID = (*item).id;
        (*sample).offset = 0 as uint64_t;
        (*sample).size = sampleSize;
        'c_8657: {
            if ((*lselProp).u.lsel.layerID as core::ffi::c_int) < 4 as core::ffi::c_int
            {} else {
                __assert_fail(
                    b"lselProp->u.lsel.layerID < 4\0" as *const u8
                        as *const core::ffi::c_char,
                    b"/home/runner/work/libavif-rs/libavif-rs/upstream/libavif/src/read.c\0"
                        as *const u8 as *const core::ffi::c_char,
                    767 as core::ffi::c_uint,
                    (::core::mem::transmute::<
                        [u8; 155],
                        [core::ffi::c_char; 155],
                    >(
                        *b"avifResult avifCodecDecodeInputFillFromDecoderItem(avifCodecDecodeInput *, avifDecoderItem *, avifBool, const uint32_t, const uint64_t, avifDiagnostics *)\0",
                    ))
                        .as_ptr(),
                );
            }
        };
        (*sample).spatialID = (*lselProp).u.lsel.layerID as uint8_t;
        (*sample).sync = AVIF_TRUE as avifBool;
    } else if allowProgressive != 0 && (*item).progressive != 0 {
        if imageCountLimit != 0 && layerCount as uint32_t > imageCountLimit {
            avifDiagnosticsPrintf(
                diag,
                b"Exceeded avifDecoder's imageCountLimit (progressive)\0" as *const u8
                    as *const core::ffi::c_char,
            );
            return AVIF_RESULT_BMFF_PARSE_FAILED;
        }
        (*decodeInput).allLayers = AVIF_TRUE as avifBool;
        let mut offset: size_t = 0 as size_t;
        let mut i_1: core::ffi::c_int = 0 as core::ffi::c_int;
        while i_1 < layerCount as core::ffi::c_int {
            let mut sample_0: *mut avifDecodeSample = avifArrayPush(
                &mut (*decodeInput).samples as *mut avifDecodeSampleArray
                    as *mut core::ffi::c_void,
            ) as *mut avifDecodeSample;
            if sample_0.is_null() {
                return AVIF_RESULT_OUT_OF_MEMORY;
            }
            (*sample_0).itemID = (*item).id;
            (*sample_0).offset = offset as uint64_t;
            (*sample_0).size = layerSizes[i_1 as usize];
            (*sample_0).spatialID = AVIF_SPATIAL_ID_UNSET as uint8_t;
            (*sample_0).sync = (i_1 == 0 as core::ffi::c_int) as core::ffi::c_int
                as avifBool;
            offset = offset.wrapping_add(layerSizes[i_1 as usize]);
            i_1 += 1;
        }
    } else {
        let mut sample_1: *mut avifDecodeSample = avifArrayPush(
            &mut (*decodeInput).samples as *mut avifDecodeSampleArray
                as *mut core::ffi::c_void,
        ) as *mut avifDecodeSample;
        if sample_1.is_null() {
            return AVIF_RESULT_OUT_OF_MEMORY;
        }
        (*sample_1).itemID = (*item).id;
        (*sample_1).offset = 0 as uint64_t;
        (*sample_1).size = (*item).size;
        (*sample_1).spatialID = AVIF_SPATIAL_ID_UNSET as uint8_t;
        (*sample_1).sync = AVIF_TRUE as avifBool;
    }
    return AVIF_RESULT_OK;
}
unsafe extern "C" fn uniqueBoxSeen(
    mut uniqueBoxFlags: *mut uint32_t,
    mut whichFlag: avifUniqueBoxFlag,
    mut parentBoxType: *const core::ffi::c_char,
    mut boxType: *const core::ffi::c_char,
    mut diagnostics: *mut avifDiagnostics,
) -> avifBool {
    let flag: uint32_t = ((1 as core::ffi::c_int) << whichFlag as core::ffi::c_uint)
        as uint32_t;
    if *uniqueBoxFlags & flag != 0 {
        avifDiagnosticsPrintf(
            diagnostics,
            b"Box[%s] contains a duplicate unique box of type '%s'\0" as *const u8
                as *const core::ffi::c_char,
            parentBoxType,
            boxType,
        );
        return AVIF_FALSE;
    }
    *uniqueBoxFlags |= flag;
    return AVIF_TRUE;
}
unsafe extern "C" fn avifMetaCreate() -> *mut avifMeta {
    let mut meta: *mut avifMeta = avifAlloc(::core::mem::size_of::<avifMeta>() as size_t)
        as *mut avifMeta;
    if meta.is_null() {
        return 0 as *mut avifMeta;
    }
    memset(
        meta as *mut core::ffi::c_void,
        0 as core::ffi::c_int,
        ::core::mem::size_of::<avifMeta>() as size_t,
    );
    if avifArrayCreate(
        &mut (*meta).items as *mut avifDecoderItemArray as *mut core::ffi::c_void,
        ::core::mem::size_of::<*mut avifDecoderItem>() as uint32_t,
        8 as uint32_t,
    ) == 0
        || avifArrayCreate(
            &mut (*meta).properties as *mut avifPropertyArray as *mut core::ffi::c_void,
            ::core::mem::size_of::<avifProperty>() as uint32_t,
            16 as uint32_t,
        ) == 0
        || avifArrayCreate(
            &mut (*meta).entityToGroups as *mut avifEntityToGroups
                as *mut core::ffi::c_void,
            ::core::mem::size_of::<avifEntityToGroup>() as uint32_t,
            1 as uint32_t,
        ) == 0
    {
        avifMetaDestroy(meta);
        return 0 as *mut avifMeta;
    }
    return meta;
}
unsafe extern "C" fn avifMetaDestroy(mut meta: *mut avifMeta) {
    let mut i: uint32_t = 0 as uint32_t;
    while i < (*meta).items.count {
        let mut item: *mut avifDecoderItem = *((*meta).items.item).offset(i as isize);
        avifPropertyArrayDestroy(&mut (*item).properties);
        avifArrayDestroy(
            &mut (*item).extents as *mut avifExtentArray as *mut core::ffi::c_void,
        );
        avifArrayDestroy(
            &mut (*item).references as *mut avifItemReferenceArray
                as *mut core::ffi::c_void,
        );
        avifArrayDestroy(
            &mut (*item).dimgInputs as *mut avifItemIDArray as *mut core::ffi::c_void,
        );
        if (*item).ownsMergedExtents != 0 {
            avifRWDataFree(&mut (*item).mergedExtents);
        }
        avifFree(item as *mut core::ffi::c_void);
        i = i.wrapping_add(1);
    }
    avifArrayDestroy(
        &mut (*meta).items as *mut avifDecoderItemArray as *mut core::ffi::c_void,
    );
    avifPropertyArrayDestroy(&mut (*meta).properties);
    avifRWDataFree(&mut (*meta).idat);
    avifArrayDestroy(
        &mut (*meta).sampleTransformExpression as *mut avifSampleTransformExpression
            as *mut core::ffi::c_void,
    );
    let mut i_0: uint32_t = 0 as uint32_t;
    while i_0 < (*meta).entityToGroups.count {
        avifArrayDestroy(
            &mut (*((*meta).entityToGroups.groups).offset(i_0 as isize)).entityIDs
                as *mut avifCodecEntityIDs as *mut core::ffi::c_void,
        );
        i_0 = i_0.wrapping_add(1);
    }
    avifArrayDestroy(
        &mut (*meta).entityToGroups as *mut avifEntityToGroups as *mut core::ffi::c_void,
    );
    avifFree(meta as *mut core::ffi::c_void);
}
unsafe extern "C" fn avifCheckItemID(
    mut boxFourcc: *const core::ffi::c_char,
    mut itemID: uint32_t,
    mut diag: *mut avifDiagnostics,
) -> avifResult {
    if itemID == 0 as uint32_t {
        avifDiagnosticsPrintf(
            diag,
            b"Box[%.4s] has an invalid item ID [%u]\0" as *const u8
                as *const core::ffi::c_char,
            boxFourcc,
            itemID,
        );
        return AVIF_RESULT_BMFF_PARSE_FAILED;
    }
    return AVIF_RESULT_OK;
}
unsafe extern "C" fn avifMetaFindOrCreateItem(
    mut meta: *mut avifMeta,
    mut itemID: uint32_t,
    mut item: *mut *mut avifDecoderItem,
) -> avifResult {
    *item = 0 as *mut avifDecoderItem;
    'c_5266: {
        if itemID != 0 as uint32_t {} else {
            __assert_fail(
                b"itemID != 0\0" as *const u8 as *const core::ffi::c_char,
                b"/home/runner/work/libavif-rs/libavif-rs/upstream/libavif/src/read.c\0"
                    as *const u8 as *const core::ffi::c_char,
                983 as core::ffi::c_uint,
                (::core::mem::transmute::<
                    [u8; 78],
                    [core::ffi::c_char; 78],
                >(
                    *b"avifResult avifMetaFindOrCreateItem(avifMeta *, uint32_t, avifDecoderItem **)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    let mut i: uint32_t = 0 as uint32_t;
    while i < (*meta).items.count {
        if (**((*meta).items.item).offset(i as isize)).id == itemID {
            *item = *((*meta).items.item).offset(i as isize);
            return AVIF_RESULT_OK;
        }
        i = i.wrapping_add(1);
    }
    let mut itemPtr: *mut *mut avifDecoderItem = avifArrayPush(
        &mut (*meta).items as *mut avifDecoderItemArray as *mut core::ffi::c_void,
    ) as *mut *mut avifDecoderItem;
    if itemPtr.is_null() {
        return AVIF_RESULT_OUT_OF_MEMORY;
    }
    *item = avifAlloc(::core::mem::size_of::<avifDecoderItem>() as size_t)
        as *mut avifDecoderItem;
    if (*item).is_null() {
        avifArrayPop(
            &mut (*meta).items as *mut avifDecoderItemArray as *mut core::ffi::c_void,
        );
        return AVIF_RESULT_OUT_OF_MEMORY;
    }
    memset(
        *item as *mut core::ffi::c_void,
        0 as core::ffi::c_int,
        ::core::mem::size_of::<avifDecoderItem>() as size_t,
    );
    *itemPtr = *item;
    if avifArrayCreate(
        &mut (**item).properties as *mut avifPropertyArray as *mut core::ffi::c_void,
        ::core::mem::size_of::<avifProperty>() as uint32_t,
        16 as uint32_t,
    ) == 0
    {
        avifFree(*item as *mut core::ffi::c_void);
        *item = 0 as *mut avifDecoderItem;
        avifArrayPop(
            &mut (*meta).items as *mut avifDecoderItemArray as *mut core::ffi::c_void,
        );
        return AVIF_RESULT_OUT_OF_MEMORY;
    }
    if avifArrayCreate(
        &mut (**item).extents as *mut avifExtentArray as *mut core::ffi::c_void,
        ::core::mem::size_of::<avifExtent>() as uint32_t,
        1 as uint32_t,
    ) == 0
    {
        avifPropertyArrayDestroy(&mut (**item).properties);
        avifFree(*item as *mut core::ffi::c_void);
        *item = 0 as *mut avifDecoderItem;
        avifArrayPop(
            &mut (*meta).items as *mut avifDecoderItemArray as *mut core::ffi::c_void,
        );
        return AVIF_RESULT_OUT_OF_MEMORY;
    }
    if avifArrayCreate(
        &mut (**item).references as *mut avifItemReferenceArray
            as *mut core::ffi::c_void,
        ::core::mem::size_of::<avifItemReference>() as uint32_t,
        1 as uint32_t,
    ) == 0
    {
        avifArrayDestroy(
            &mut (**item).extents as *mut avifExtentArray as *mut core::ffi::c_void,
        );
        avifPropertyArrayDestroy(&mut (**item).properties);
        avifFree(*item as *mut core::ffi::c_void);
        *item = 0 as *mut avifDecoderItem;
        avifArrayPop(
            &mut (*meta).items as *mut avifDecoderItemArray as *mut core::ffi::c_void,
        );
        return AVIF_RESULT_OUT_OF_MEMORY;
    }
    if avifArrayCreate(
        &mut (**item).dimgInputs as *mut avifItemIDArray as *mut core::ffi::c_void,
        ::core::mem::size_of::<uint32_t>() as uint32_t,
        1 as uint32_t,
    ) == 0
    {
        avifArrayDestroy(
            &mut (**item).references as *mut avifItemReferenceArray
                as *mut core::ffi::c_void,
        );
        avifArrayDestroy(
            &mut (**item).extents as *mut avifExtentArray as *mut core::ffi::c_void,
        );
        avifPropertyArrayDestroy(&mut (**item).properties);
        avifFree(*item as *mut core::ffi::c_void);
        *item = 0 as *mut avifDecoderItem;
        avifArrayPop(
            &mut (*meta).items as *mut avifDecoderItemArray as *mut core::ffi::c_void,
        );
        return AVIF_RESULT_OUT_OF_MEMORY;
    }
    (**item).id = itemID;
    (**item).meta = meta as *mut avifMeta;
    return AVIF_RESULT_OK;
}
unsafe extern "C" fn avifDecoderDataCreate() -> *mut avifDecoderData {
    let mut data: *mut avifDecoderData = avifAlloc(
        ::core::mem::size_of::<avifDecoderData>() as size_t,
    ) as *mut avifDecoderData;
    if data.is_null() {
        return 0 as *mut avifDecoderData;
    }
    memset(
        data as *mut core::ffi::c_void,
        0 as core::ffi::c_int,
        ::core::mem::size_of::<avifDecoderData>() as size_t,
    );
    (*data).meta = avifMetaCreate();
    if ((*data).meta).is_null()
        || avifArrayCreate(
            &mut (*data).tracks as *mut avifTrackArray as *mut core::ffi::c_void,
            ::core::mem::size_of::<avifTrack>() as uint32_t,
            2 as uint32_t,
        ) == 0
        || avifArrayCreate(
            &mut (*data).tiles as *mut avifTileArray as *mut core::ffi::c_void,
            ::core::mem::size_of::<avifTile>() as uint32_t,
            8 as uint32_t,
        ) == 0
    {
        avifDecoderDataDestroy(data);
        return 0 as *mut avifDecoderData;
    }
    return data;
}
unsafe extern "C" fn avifDecoderDataResetCodec(mut data: *mut avifDecoderData) {
    let mut i: core::ffi::c_uint = 0 as core::ffi::c_uint;
    while (i as uint32_t) < (*data).tiles.count {
        let mut tile: *mut avifTile = &mut *((*data).tiles.tile).offset(i as isize)
            as *mut avifTile;
        if !((*tile).image).is_null() {
            avifImageFreePlanes(
                (*tile).image,
                AVIF_PLANES_ALL as core::ffi::c_int as avifPlanesFlags,
            );
        }
        if !((*tile).codec).is_null() {
            if (*tile).codec != (*data).codec && (*tile).codec != (*data).codecAlpha {
                avifCodecDestroy((*tile).codec as *mut avifCodec);
            }
            (*tile).codec = 0 as *mut avifCodec;
        }
        i = i.wrapping_add(1);
    }
    let mut c: core::ffi::c_int = 0 as core::ffi::c_int;
    while c < AVIF_ITEM_CATEGORY_COUNT as core::ffi::c_int {
        (*data).tileInfos[c as usize].decodedTileCount = 0 as core::ffi::c_uint;
        c += 1;
    }
    if !((*data).codec).is_null() {
        avifCodecDestroy((*data).codec);
        (*data).codec = 0 as *mut avifCodec;
    }
    if !((*data).codecAlpha).is_null() {
        avifCodecDestroy((*data).codecAlpha);
        (*data).codecAlpha = 0 as *mut avifCodec;
    }
}
unsafe extern "C" fn avifDecoderDataCreateTile(
    mut data: *mut avifDecoderData,
    mut codecType: avifCodecType,
    mut width: uint32_t,
    mut height: uint32_t,
    mut operatingPoint: uint8_t,
) -> *mut avifTile {
    let mut tile: *mut avifTile = avifArrayPush(
        &mut (*data).tiles as *mut avifTileArray as *mut core::ffi::c_void,
    ) as *mut avifTile;
    if tile.is_null() {
        return 0 as *mut avifTile;
    }
    (*tile).codecType = codecType;
    (*tile).image = avifImageCreateEmpty();
    if !((*tile).image).is_null() {
        (*tile).input = avifCodecDecodeInputCreate();
        if !((*tile).input).is_null() {
            (*tile).width = width;
            (*tile).height = height;
            (*tile).operatingPoint = operatingPoint;
            return tile;
        }
    }
    if !((*tile).input).is_null() {
        avifCodecDecodeInputDestroy((*tile).input);
    }
    if !((*tile).image).is_null() {
        avifImageDestroy((*tile).image);
    }
    avifArrayPop(&mut (*data).tiles as *mut avifTileArray as *mut core::ffi::c_void);
    return 0 as *mut avifTile;
}
unsafe extern "C" fn avifDecoderDataCreateTrack(
    mut data: *mut avifDecoderData,
) -> *mut avifTrack {
    let mut track: *mut avifTrack = avifArrayPush(
        &mut (*data).tracks as *mut avifTrackArray as *mut core::ffi::c_void,
    ) as *mut avifTrack;
    if track.is_null() {
        return 0 as *mut avifTrack;
    }
    (*track).meta = avifMetaCreate() as *mut avifMeta;
    if ((*track).meta).is_null() {
        avifArrayPop(
            &mut (*data).tracks as *mut avifTrackArray as *mut core::ffi::c_void,
        );
        return 0 as *mut avifTrack;
    }
    if avifArrayCreate(
        &mut (*track).references as *mut avifItemReferenceArray
            as *mut core::ffi::c_void,
        ::core::mem::size_of::<avifItemReference>() as uint32_t,
        1 as uint32_t,
    ) == 0
    {
        avifMetaDestroy((*track).meta as *mut avifMeta);
        avifArrayPop(
            &mut (*data).tracks as *mut avifTrackArray as *mut core::ffi::c_void,
        );
        return 0 as *mut avifTrack;
    }
    return track;
}
unsafe extern "C" fn avifDecoderDataClearTiles(mut data: *mut avifDecoderData) {
    let mut i: core::ffi::c_uint = 0 as core::ffi::c_uint;
    while (i as uint32_t) < (*data).tiles.count {
        let mut tile: *mut avifTile = &mut *((*data).tiles.tile).offset(i as isize)
            as *mut avifTile;
        if !((*tile).input).is_null() {
            avifCodecDecodeInputDestroy((*tile).input);
            (*tile).input = 0 as *mut avifCodecDecodeInput;
        }
        if !((*tile).codec).is_null() {
            if (*tile).codec != (*data).codec && (*tile).codec != (*data).codecAlpha {
                avifCodecDestroy((*tile).codec as *mut avifCodec);
            }
            (*tile).codec = 0 as *mut avifCodec;
        }
        if !((*tile).image).is_null() {
            avifImageDestroy((*tile).image);
            (*tile).image = 0 as *mut avifImage;
        }
        i = i.wrapping_add(1);
    }
    (*data).tiles.count = 0 as uint32_t;
    let mut c: core::ffi::c_int = 0 as core::ffi::c_int;
    while c < AVIF_ITEM_CATEGORY_COUNT as core::ffi::c_int {
        (*data).tileInfos[c as usize].tileCount = 0 as core::ffi::c_uint;
        (*data).tileInfos[c as usize].decodedTileCount = 0 as core::ffi::c_uint;
        c += 1;
    }
    if !((*data).codec).is_null() {
        avifCodecDestroy((*data).codec);
        (*data).codec = 0 as *mut avifCodec;
    }
    if !((*data).codecAlpha).is_null() {
        avifCodecDestroy((*data).codecAlpha);
        (*data).codecAlpha = 0 as *mut avifCodec;
    }
}
unsafe extern "C" fn avifDecoderDataDestroy(mut data: *mut avifDecoderData) {
    if !((*data).meta).is_null() {
        avifMetaDestroy((*data).meta);
    }
    let mut i: uint32_t = 0 as uint32_t;
    while i < (*data).tracks.count {
        let mut track: *mut avifTrack = &mut *((*data).tracks.track).offset(i as isize)
            as *mut avifTrack;
        if !((*track).sampleTable).is_null() {
            avifSampleTableDestroy((*track).sampleTable);
        }
        if !((*track).meta).is_null() {
            avifMetaDestroy((*track).meta as *mut avifMeta);
        }
        avifArrayDestroy(
            &mut (*track).references as *mut avifItemReferenceArray
                as *mut core::ffi::c_void,
        );
        i = i.wrapping_add(1);
    }
    avifArrayDestroy(
        &mut (*data).tracks as *mut avifTrackArray as *mut core::ffi::c_void,
    );
    avifDecoderDataClearTiles(data);
    avifArrayDestroy(&mut (*data).tiles as *mut avifTileArray as *mut core::ffi::c_void);
    avifArrayDestroy(
        &mut (*data).compatibleBrands as *mut avifBrandArray as *mut core::ffi::c_void,
    );
    avifFree(data as *mut core::ffi::c_void);
}
unsafe extern "C" fn avifDecoderItemMaxExtent(
    mut item: *const avifDecoderItem,
    mut sample: *const avifDecodeSample,
    mut outExtent: *mut avifExtent,
) -> avifResult {
    if (*item).extents.count == 0 as uint32_t {
        return AVIF_RESULT_TRUNCATED_DATA;
    }
    if (*item).idatStored != 0 {
        if (*(*item).meta).idat.size > 0 as size_t {
            memset(
                outExtent as *mut core::ffi::c_void,
                0 as core::ffi::c_int,
                ::core::mem::size_of::<avifExtent>() as size_t,
            );
            return AVIF_RESULT_OK;
        }
        return AVIF_RESULT_NO_CONTENT;
    }
    if (*sample).size == 0 as size_t {
        return AVIF_RESULT_TRUNCATED_DATA;
    }
    let mut remainingOffset: uint64_t = (*sample).offset;
    let mut remainingBytes: size_t = (*sample).size;
    'c_40623: {
        if (*item).extents.count != 0 as uint32_t {} else {
            __assert_fail(
                b"item->extents.count != 0\0" as *const u8 as *const core::ffi::c_char,
                b"/home/runner/work/libavif-rs/libavif-rs/upstream/libavif/src/read.c\0"
                    as *const u8 as *const core::ffi::c_char,
                1268 as core::ffi::c_uint,
                (::core::mem::transmute::<
                    [u8; 101],
                    [core::ffi::c_char; 101],
                >(
                    *b"avifResult avifDecoderItemMaxExtent(const avifDecoderItem *, const avifDecodeSample *, avifExtent *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    let mut minOffset: uint64_t = UINT64_MAX as uint64_t;
    let mut maxOffset: uint64_t = 0 as uint64_t;
    let mut current_block_31: u64;
    let mut extentIter: uint32_t = 0 as uint32_t;
    while extentIter < (*item).extents.count {
        let mut extent: *mut avifExtent = &mut *((*item).extents.extent)
            .offset(extentIter as isize) as *mut avifExtent;
        let mut startOffset: uint64_t = (*extent).offset;
        let mut extentSize: size_t = (*extent).size;
        if remainingOffset != 0 {
            if remainingOffset >= extentSize as uint64_t {
                remainingOffset = (remainingOffset as core::ffi::c_ulong)
                    .wrapping_sub(extentSize as core::ffi::c_ulong) as uint64_t
                    as uint64_t;
                current_block_31 = 0;
            } else {
                if remainingOffset > (UINT64_MAX as uint64_t).wrapping_sub(startOffset) {
                    return AVIF_RESULT_BMFF_PARSE_FAILED;
                }
                startOffset = startOffset.wrapping_add(remainingOffset);
                extentSize = extentSize.wrapping_sub(remainingOffset as size_t);
                remainingOffset = 0 as uint64_t;
                current_block_31 = 1;
            }
        } else {
            current_block_31 = 1;
        }
        match current_block_31 {
            1 => {
                let usedExtentSize: size_t = if extentSize < remainingBytes {
                    extentSize
                } else {
                    remainingBytes
                };
                if usedExtentSize
                    > (UINT64_MAX as size_t).wrapping_sub(startOffset as size_t)
                {
                    return AVIF_RESULT_BMFF_PARSE_FAILED;
                }
                let endOffset: uint64_t = startOffset
                    .wrapping_add(usedExtentSize as uint64_t);
                if minOffset > startOffset {
                    minOffset = startOffset;
                }
                if maxOffset < endOffset {
                    maxOffset = endOffset;
                }
                remainingBytes = remainingBytes.wrapping_sub(usedExtentSize);
                if remainingBytes == 0 as size_t {
                    break;
                }
            }
            _ => {}
        }
        extentIter = extentIter.wrapping_add(1);
    }
    if remainingBytes != 0 as size_t {
        return AVIF_RESULT_TRUNCATED_DATA;
    }
    (*outExtent).offset = minOffset;
    let extentLength: uint64_t = maxOffset.wrapping_sub(minOffset);
    (*outExtent).size = extentLength as size_t;
    return AVIF_RESULT_OK;
}
unsafe extern "C" fn avifDecoderItemOperatingPoint(
    mut item: *const avifDecoderItem,
) -> uint8_t {
    let mut a1opProp: *const avifProperty = avifPropertyArrayFind(
        &(*item).properties,
        b"a1op\0" as *const u8 as *const core::ffi::c_char,
    );
    if !a1opProp.is_null() {
        return (*a1opProp).u.a1op.opIndex;
    }
    return 0 as uint8_t;
}
unsafe extern "C" fn avifDecoderItemValidateProperties(
    mut item: *const avifDecoderItem,
    mut configPropName: *const core::ffi::c_char,
    mut diag: *mut avifDiagnostics,
    strictFlags: avifStrictFlags,
) -> avifResult {
    let configProp: *const avifProperty = avifPropertyArrayFind(
        &(*item).properties,
        configPropName,
    ) as *const avifProperty;
    if configProp.is_null() {
        avifDiagnosticsPrintf(
            diag,
            b"Item ID %u of type '%.4s' is missing mandatory %s property\0" as *const u8
                as *const core::ffi::c_char,
            (*item).id,
            ((*item).type_0).as_ptr() as *const core::ffi::c_char,
            configPropName,
        );
        return AVIF_RESULT_BMFF_PARSE_FAILED;
    }
    if memcmp(
        ((*item).type_0).as_ptr() as *const core::ffi::c_void,
        b"grid\0" as *const u8 as *const core::ffi::c_char as *const core::ffi::c_void,
        4 as size_t,
    ) == 0
    {
        let mut i: uint32_t = 0 as uint32_t;
        while i < (*(*item).meta).items.count {
            let mut tile: *mut avifDecoderItem = *((*(*item).meta).items.item)
                .offset(i as isize);
            if !(avifIsDimgInput(item, (*tile).id, 0 as *mut uint32_t) == 0) {
                let mut tileConfigProp: *const avifProperty = avifPropertyArrayFind(
                    &mut (*tile).properties,
                    configPropName,
                );
                if tileConfigProp.is_null() {
                    avifDiagnosticsPrintf(
                        diag,
                        b"Tile item ID %u of type '%.4s' is missing mandatory %s property\0"
                            as *const u8 as *const core::ffi::c_char,
                        (*tile).id,
                        ((*tile).type_0).as_mut_ptr() as *const core::ffi::c_char,
                        configPropName,
                    );
                    return AVIF_RESULT_BMFF_PARSE_FAILED;
                }
                if (*tileConfigProp).u.av1C.seqProfile as core::ffi::c_int
                    != (*configProp).u.av1C.seqProfile as core::ffi::c_int
                    || (*tileConfigProp).u.av1C.seqLevelIdx0 as core::ffi::c_int
                        != (*configProp).u.av1C.seqLevelIdx0 as core::ffi::c_int
                    || (*tileConfigProp).u.av1C.seqTier0 as core::ffi::c_int
                        != (*configProp).u.av1C.seqTier0 as core::ffi::c_int
                    || (*tileConfigProp).u.av1C.highBitdepth as core::ffi::c_int
                        != (*configProp).u.av1C.highBitdepth as core::ffi::c_int
                    || (*tileConfigProp).u.av1C.twelveBit as core::ffi::c_int
                        != (*configProp).u.av1C.twelveBit as core::ffi::c_int
                    || (*tileConfigProp).u.av1C.monochrome as core::ffi::c_int
                        != (*configProp).u.av1C.monochrome as core::ffi::c_int
                    || (*tileConfigProp).u.av1C.chromaSubsamplingX as core::ffi::c_int
                        != (*configProp).u.av1C.chromaSubsamplingX as core::ffi::c_int
                    || (*tileConfigProp).u.av1C.chromaSubsamplingY as core::ffi::c_int
                        != (*configProp).u.av1C.chromaSubsamplingY as core::ffi::c_int
                    || (*tileConfigProp).u.av1C.chromaSamplePosition as core::ffi::c_int
                        != (*configProp).u.av1C.chromaSamplePosition as core::ffi::c_int
                {
                    avifDiagnosticsPrintf(
                        diag,
                        b"The fields of the %s property of tile item ID %u of type '%.4s' differs from other tiles\0"
                            as *const u8 as *const core::ffi::c_char,
                        configPropName,
                        (*tile).id,
                        ((*tile).type_0).as_mut_ptr() as *const core::ffi::c_char,
                    );
                    return AVIF_RESULT_BMFF_PARSE_FAILED;
                }
            }
            i = i.wrapping_add(1);
        }
    }
    let mut pixiProp: *const avifProperty = avifPropertyArrayFind(
        &(*item).properties,
        b"pixi\0" as *const u8 as *const core::ffi::c_char,
    );
    if pixiProp.is_null()
        && strictFlags & AVIF_STRICT_PIXI_REQUIRED as core::ffi::c_int as avifStrictFlags
            != 0
    {
        avifDiagnosticsPrintf(
            diag,
            b"[Strict] Item ID %u of type '%.4s' is missing mandatory pixi property\0"
                as *const u8 as *const core::ffi::c_char,
            (*item).id,
            ((*item).type_0).as_ptr() as *const core::ffi::c_char,
        );
        return AVIF_RESULT_BMFF_PARSE_FAILED;
    }
    if !pixiProp.is_null() {
        let configDepth: uint32_t = avifCodecConfigurationBoxGetDepth(
            &(*configProp).u.av1C,
        ) as uint32_t;
        let mut i_0: uint8_t = 0 as uint8_t;
        while (i_0 as core::ffi::c_int)
            < (*pixiProp).u.pixi.planeCount as core::ffi::c_int
        {
            if (*pixiProp).u.pixi.planeDepths[i_0 as usize] as uint32_t != configDepth {
                avifDiagnosticsPrintf(
                    diag,
                    b"Item ID %u depth specified by pixi property [%u] does not match %s property depth [%u]\0"
                        as *const u8 as *const core::ffi::c_char,
                    (*item).id,
                    (*pixiProp).u.pixi.planeDepths[i_0 as usize] as core::ffi::c_int,
                    configPropName,
                    configDepth,
                );
                return AVIF_RESULT_BMFF_PARSE_FAILED;
            }
            i_0 = i_0.wrapping_add(1);
        }
    }
    if strictFlags & AVIF_STRICT_CLAP_VALID as core::ffi::c_int as avifStrictFlags != 0 {
        let mut clapProp: *const avifProperty = avifPropertyArrayFind(
            &(*item).properties,
            b"clap\0" as *const u8 as *const core::ffi::c_char,
        );
        if !clapProp.is_null() {
            let mut ispeProp: *const avifProperty = avifPropertyArrayFind(
                &(*item).properties,
                b"ispe\0" as *const u8 as *const core::ffi::c_char,
            );
            if ispeProp.is_null() {
                avifDiagnosticsPrintf(
                    diag,
                    b"[Strict] Item ID %u is missing an ispe property, so its clap property cannot be validated\0"
                        as *const u8 as *const core::ffi::c_char,
                    (*item).id,
                );
                return AVIF_RESULT_BMFF_PARSE_FAILED;
            }
            let mut cropRect: avifCropRect = avifCropRect {
                x: 0,
                y: 0,
                width: 0,
                height: 0,
            };
            let imageW: uint32_t = (*ispeProp).u.ispe.width;
            let imageH: uint32_t = (*ispeProp).u.ispe.height;
            let validClap: avifBool = avifCropRectFromCleanApertureBox(
                &mut cropRect,
                &(*clapProp).u.clap,
                imageW,
                imageH,
                diag,
            ) as avifBool;
            if validClap == 0 {
                return AVIF_RESULT_BMFF_PARSE_FAILED;
            }
        }
    }
    return AVIF_RESULT_OK;
}
unsafe extern "C" fn avifDecoderItemRead(
    mut item: *mut avifDecoderItem,
    mut io: *mut avifIO,
    mut outData: *mut avifROData,
    mut offset: size_t,
    mut partialByteCount: size_t,
    mut diag: *mut avifDiagnostics,
) -> avifResult {
    if !((*item).mergedExtents.data).is_null() && (*item).partialMergedExtents == 0 {
        if offset >= (*item).mergedExtents.size {
            avifDiagnosticsPrintf(
                diag,
                b"Item ID %u read has overflowing offset\0" as *const u8
                    as *const core::ffi::c_char,
                (*item).id,
            );
            return AVIF_RESULT_TRUNCATED_DATA;
        }
        (*outData).data = ((*item).mergedExtents.data).offset(offset as isize);
        (*outData).size = ((*item).mergedExtents.size).wrapping_sub(offset);
        return AVIF_RESULT_OK;
    }
    if (*item).extents.count == 0 as uint32_t {
        avifDiagnosticsPrintf(
            diag,
            b"Item ID %u has zero extents\0" as *const u8 as *const core::ffi::c_char,
            (*item).id,
        );
        return AVIF_RESULT_TRUNCATED_DATA;
    }
    let mut idatBuffer: *const avifRWData = 0 as *const avifRWData;
    if (*item).idatStored != 0 {
        if (*(*item).meta).idat.size > 0 as size_t {
            idatBuffer = &mut (*(*item).meta).idat;
        } else {
            avifDiagnosticsPrintf(
                diag,
                b"Item ID %u is stored in an idat, but no associated idat box was found\0"
                    as *const u8 as *const core::ffi::c_char,
                (*item).id,
            );
            return AVIF_RESULT_NO_CONTENT;
        }
    }
    if (*io).sizeHint > 0 as uint64_t && (*item).size > (*io).sizeHint as size_t {
        avifDiagnosticsPrintf(
            diag,
            b"Item ID %u reported size failed size hint sanity check. Truncated data?\0"
                as *const u8 as *const core::ffi::c_char,
            (*item).id,
        );
        return AVIF_RESULT_TRUNCATED_DATA;
    }
    if offset >= (*item).size {
        avifDiagnosticsPrintf(
            diag,
            b"Item ID %u read has overflowing offset\0" as *const u8
                as *const core::ffi::c_char,
            (*item).id,
        );
        return AVIF_RESULT_TRUNCATED_DATA;
    }
    let maxOutputSize: size_t = ((*item).size).wrapping_sub(offset);
    let readOutputSize: size_t = if partialByteCount != 0
        && partialByteCount < maxOutputSize
    {
        partialByteCount
    } else {
        maxOutputSize
    };
    let totalBytesToRead: size_t = offset.wrapping_add(readOutputSize);
    let mut singlePersistentBuffer: avifBool = ((*item).extents.count == 1 as uint32_t
        && (!idatBuffer.is_null() || (*io).persistent != 0)) as core::ffi::c_int;
    if singlePersistentBuffer == 0 {
        let result__: avifResult = avifRWDataRealloc(
            &mut (*item).mergedExtents,
            (*item).size,
        ) as avifResult;
        if result__ as core::ffi::c_uint
            != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
        {
            return result__;
        }
        (*item).ownsMergedExtents = AVIF_TRUE as avifBool;
    }
    (*item).partialMergedExtents = AVIF_TRUE as avifBool;
    let mut writeOffset: size_t = 0 as size_t;
    let mut remainingBytes: size_t = totalBytesToRead;
    let mut extentIter: uint32_t = 0 as uint32_t;
    while extentIter < (*item).extents.count {
        let mut extent: *mut avifExtent = &mut *((*item).extents.extent)
            .offset(extentIter as isize) as *mut avifExtent;
        let mut bytesToRead: size_t = (*extent).size;
        if bytesToRead > remainingBytes {
            bytesToRead = remainingBytes;
        }
        let mut offsetBuffer: avifROData = avifROData {
            data: 0 as *const uint8_t,
            size: 0,
        };
        if !idatBuffer.is_null() {
            if (*extent).offset > (*idatBuffer).size as uint64_t {
                avifDiagnosticsPrintf(
                    diag,
                    b"Item ID %u has impossible extent offset in idat buffer\0"
                        as *const u8 as *const core::ffi::c_char,
                    (*item).id,
                );
                return AVIF_RESULT_BMFF_PARSE_FAILED;
            }
            let extentOffset: size_t = (*extent).offset as size_t;
            if (*extent).size > ((*idatBuffer).size).wrapping_sub(extentOffset) {
                avifDiagnosticsPrintf(
                    diag,
                    b"Item ID %u has impossible extent size in idat buffer\0"
                        as *const u8 as *const core::ffi::c_char,
                    (*item).id,
                );
                return AVIF_RESULT_BMFF_PARSE_FAILED;
            }
            offsetBuffer.data = ((*idatBuffer).data).offset(extentOffset as isize);
            offsetBuffer.size = ((*idatBuffer).size).wrapping_sub(extentOffset);
        } else {
            if (*io).sizeHint > 0 as uint64_t && (*extent).offset > (*io).sizeHint {
                avifDiagnosticsPrintf(
                    diag,
                    b"Item ID %u extent offset failed size hint sanity check. Truncated data?\0"
                        as *const u8 as *const core::ffi::c_char,
                    (*item).id,
                );
                return AVIF_RESULT_BMFF_PARSE_FAILED;
            }
            let mut readResult: avifResult = ((*io).read)
                .expect(
                    "non-null function pointer",
                )(
                io as *mut avifIO,
                0 as uint32_t,
                (*extent).offset,
                bytesToRead,
                &mut offsetBuffer,
            );
            if readResult as core::ffi::c_uint
                != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
            {
                return readResult;
            }
            if bytesToRead != offsetBuffer.size {
                avifDiagnosticsPrintf(
                    diag,
                    b"Item ID %u tried to read %zu bytes, but only received %zu bytes\0"
                        as *const u8 as *const core::ffi::c_char,
                    (*item).id,
                    bytesToRead,
                    offsetBuffer.size,
                );
                return AVIF_RESULT_TRUNCATED_DATA;
            }
        }
        if singlePersistentBuffer != 0 {
            (*item).mergedExtents.data = offsetBuffer.data as *mut uint8_t;
            'c_4273: {
                if bytesToRead <= offsetBuffer.size {} else {
                    __assert_fail(
                        b"bytesToRead <= offsetBuffer.size\0" as *const u8
                            as *const core::ffi::c_char,
                        b"/home/runner/work/libavif-rs/libavif-rs/upstream/libavif/src/read.c\0"
                            as *const u8 as *const core::ffi::c_char,
                        1634 as core::ffi::c_uint,
                        (::core::mem::transmute::<
                            [u8; 109],
                            [core::ffi::c_char; 109],
                        >(
                            *b"avifResult avifDecoderItemRead(avifDecoderItem *, avifIO *, avifROData *, size_t, size_t, avifDiagnostics *)\0",
                        ))
                            .as_ptr(),
                    );
                }
            };
            (*item).mergedExtents.size = bytesToRead;
        } else {
            'c_4226: {
                if (*item).ownsMergedExtents != 0 {} else {
                    __assert_fail(
                        b"item->ownsMergedExtents\0" as *const u8
                            as *const core::ffi::c_char,
                        b"/home/runner/work/libavif-rs/libavif-rs/upstream/libavif/src/read.c\0"
                            as *const u8 as *const core::ffi::c_char,
                        1637 as core::ffi::c_uint,
                        (::core::mem::transmute::<
                            [u8; 109],
                            [core::ffi::c_char; 109],
                        >(
                            *b"avifResult avifDecoderItemRead(avifDecoderItem *, avifIO *, avifROData *, size_t, size_t, avifDiagnostics *)\0",
                        ))
                            .as_ptr(),
                    );
                }
            };
            'c_4181: {
                if writeOffset < (*item).mergedExtents.size {} else {
                    __assert_fail(
                        b"writeOffset < item->mergedExtents.size\0" as *const u8
                            as *const core::ffi::c_char,
                        b"/home/runner/work/libavif-rs/libavif-rs/upstream/libavif/src/read.c\0"
                            as *const u8 as *const core::ffi::c_char,
                        1638 as core::ffi::c_uint,
                        (::core::mem::transmute::<
                            [u8; 109],
                            [core::ffi::c_char; 109],
                        >(
                            *b"avifResult avifDecoderItemRead(avifDecoderItem *, avifIO *, avifROData *, size_t, size_t, avifDiagnostics *)\0",
                        ))
                            .as_ptr(),
                    );
                }
            };
            'c_4129: {
                if bytesToRead <= ((*item).mergedExtents.size).wrapping_sub(writeOffset)
                {} else {
                    __assert_fail(
                        b"bytesToRead <= item->mergedExtents.size - writeOffset\0"
                            as *const u8 as *const core::ffi::c_char,
                        b"/home/runner/work/libavif-rs/libavif-rs/upstream/libavif/src/read.c\0"
                            as *const u8 as *const core::ffi::c_char,
                        1639 as core::ffi::c_uint,
                        (::core::mem::transmute::<
                            [u8; 109],
                            [core::ffi::c_char; 109],
                        >(
                            *b"avifResult avifDecoderItemRead(avifDecoderItem *, avifIO *, avifROData *, size_t, size_t, avifDiagnostics *)\0",
                        ))
                            .as_ptr(),
                    );
                }
            };
            memcpy(
                ((*item).mergedExtents.data).offset(writeOffset as isize)
                    as *mut core::ffi::c_void,
                offsetBuffer.data as *const core::ffi::c_void,
                bytesToRead,
            );
            writeOffset = writeOffset.wrapping_add(bytesToRead);
        }
        remainingBytes = remainingBytes.wrapping_sub(bytesToRead);
        if remainingBytes == 0 as size_t {
            break;
        }
        extentIter = extentIter.wrapping_add(1);
    }
    if remainingBytes != 0 as size_t {
        avifDiagnosticsPrintf(
            diag,
            b"Item ID %u has %zu unexpected trailing bytes\0" as *const u8
                as *const core::ffi::c_char,
            (*item).id,
            remainingBytes,
        );
        return AVIF_RESULT_TRUNCATED_DATA;
    }
    (*outData).data = ((*item).mergedExtents.data).offset(offset as isize);
    (*outData).size = readOutputSize;
    (*item).partialMergedExtents = ((*item).size != totalBytesToRead) as core::ffi::c_int
        as avifBool;
    return AVIF_RESULT_OK;
}
unsafe extern "C" fn avifDecoderItemGetGridCodecType(
    mut gridItem: *const avifDecoderItem,
) -> avifCodecType {
    let mut i: uint32_t = 0 as uint32_t;
    while i < (*(*gridItem).meta).items.count {
        let mut item: *mut avifDecoderItem = *((*(*gridItem).meta).items.item)
            .offset(i as isize);
        let tileCodecType: avifCodecType = avifGetCodecType(
            ((*item).type_0).as_mut_ptr(),
        ) as avifCodecType;
        if avifIsDimgInput(gridItem, (*item).id, 0 as *mut uint32_t) != 0
            && tileCodecType as core::ffi::c_uint
                != AVIF_CODEC_TYPE_UNKNOWN as core::ffi::c_int as core::ffi::c_uint
        {
            return tileCodecType;
        }
        i = i.wrapping_add(1);
    }
    return AVIF_CODEC_TYPE_UNKNOWN;
}
unsafe extern "C" fn avifFillDimgIdxToItemIdxArray(
    mut dimgIdxToItemIdx: *mut uint32_t,
    mut numExpectedTiles: uint32_t,
    mut gridItem: *const avifDecoderItem,
) -> avifResult {
    if !((*gridItem).dimgInputs.count == numExpectedTiles) {
        return AVIF_RESULT_INVALID_IMAGE_GRID;
    }
    let mut dimgIdx: uint32_t = 0 as uint32_t;
    while dimgIdx < numExpectedTiles {
        let inputItemID: uint32_t = *((*gridItem).dimgInputs.id)
            .offset(dimgIdx as isize);
        let mut itemIdx: uint32_t = UINT32_MAX as uint32_t;
        let mut i: uint32_t = 0 as uint32_t;
        while i < (*(*gridItem).meta).items.count {
            if (**((*(*gridItem).meta).items.item).offset(i as isize)).id == inputItemID
            {
                itemIdx = i;
                break;
            } else {
                i = i.wrapping_add(1);
            }
        }
        if !(itemIdx != 4294967295 as uint32_t) {
            return AVIF_RESULT_INVALID_IMAGE_GRID;
        }
        *dimgIdxToItemIdx.offset(dimgIdx as isize) = itemIdx;
        dimgIdx = dimgIdx.wrapping_add(1);
    }
    return AVIF_RESULT_OK;
}
unsafe extern "C" fn avifDecoderAdoptGridTileCodecType(
    mut decoder: *mut avifDecoder,
    mut gridItem: *mut avifDecoderItem,
    mut dimgIdxToItemIdx: *const uint32_t,
    mut numTiles: uint32_t,
) -> avifResult {
    let mut firstTileItem: *mut avifDecoderItem = 0 as *mut avifDecoderItem;
    let mut dimgIdx: uint32_t = 0 as uint32_t;
    while dimgIdx < numTiles {
        let itemIdx: uint32_t = *dimgIdxToItemIdx.offset(dimgIdx as isize);
        'c_10355: {
            if itemIdx < (*(*gridItem).meta).items.count {} else {
                __assert_fail(
                    b"itemIdx < gridItem->meta->items.count\0" as *const u8
                        as *const core::ffi::c_char,
                    b"/home/runner/work/libavif-rs/libavif-rs/upstream/libavif/src/read.c\0"
                        as *const u8 as *const core::ffi::c_char,
                    1707 as core::ffi::c_uint,
                    (::core::mem::transmute::<
                        [u8; 107],
                        [core::ffi::c_char; 107],
                    >(
                        *b"avifResult avifDecoderAdoptGridTileCodecType(avifDecoder *, avifDecoderItem *, const uint32_t *, uint32_t)\0",
                    ))
                        .as_ptr(),
                );
            }
        };
        let mut item: *mut avifDecoderItem = *((*(*gridItem).meta).items.item)
            .offset(itemIdx as isize);
        let tileCodecType: avifCodecType = avifGetCodecType(
            ((*item).type_0).as_mut_ptr(),
        ) as avifCodecType;
        if tileCodecType as core::ffi::c_uint
            == AVIF_CODEC_TYPE_UNKNOWN as core::ffi::c_int as core::ffi::c_uint
        {
            let mut type_0: [core::ffi::c_char; 4] = [0; 4];
            let mut j: core::ffi::c_int = 0 as core::ffi::c_int;
            while j < 4 as core::ffi::c_int {
                if *(*__ctype_b_loc())
                    .offset(
                        (*item).type_0[j as usize] as core::ffi::c_uchar
                            as core::ffi::c_int as isize,
                    ) as core::ffi::c_int
                    & _ISprint as core::ffi::c_int as core::ffi::c_ushort
                        as core::ffi::c_int != 0
                {
                    type_0[j as usize] = (*item).type_0[j as usize] as core::ffi::c_char;
                } else {
                    type_0[j as usize] = '.' as i32 as core::ffi::c_char;
                }
                j += 1;
            }
            avifDiagnosticsPrintf(
                &mut (*decoder).diag as *mut avifDiagnostics,
                b"Tile item ID %u has an unknown item type '%.4s' (%02x%02x%02x%02x)\0"
                    as *const u8 as *const core::ffi::c_char,
                (*item).id,
                type_0.as_mut_ptr(),
                (*item).type_0[0 as core::ffi::c_int as usize] as core::ffi::c_int,
                (*item).type_0[1 as core::ffi::c_int as usize] as core::ffi::c_int,
                (*item).type_0[2 as core::ffi::c_int as usize] as core::ffi::c_int,
                (*item).type_0[3 as core::ffi::c_int as usize] as core::ffi::c_int,
            );
            return AVIF_RESULT_INVALID_IMAGE_GRID;
        }
        if (*item).hasUnsupportedEssentialProperty != 0 {
            avifDiagnosticsPrintf(
                &mut (*decoder).diag as *mut avifDiagnostics,
                b"Grid image contains tile with an unsupported property marked as essential\0"
                    as *const u8 as *const core::ffi::c_char,
            );
            return AVIF_RESULT_INVALID_IMAGE_GRID;
        }
        if firstTileItem.is_null() {
            firstTileItem = item;
            let codecType: avifCodecType = avifGetCodecType(
                ((*item).type_0).as_mut_ptr(),
            ) as avifCodecType;
            let mut configPropName: *const core::ffi::c_char = avifGetConfigurationPropertyName(
                codecType,
            );
            let mut srcProp: *const avifProperty = avifPropertyArrayFind(
                &mut (*item).properties,
                configPropName,
            );
            if srcProp.is_null() {
                avifDiagnosticsPrintf(
                    &mut (*decoder).diag as *mut avifDiagnostics,
                    b"Grid image's first tile is missing an %s property\0" as *const u8
                        as *const core::ffi::c_char,
                    configPropName,
                );
                return AVIF_RESULT_INVALID_IMAGE_GRID;
            }
            let mut dstProp: *mut avifProperty = avifArrayPush(
                &mut (*gridItem).properties as *mut avifPropertyArray
                    as *mut core::ffi::c_void,
            ) as *mut avifProperty;
            if dstProp.is_null() {
                return AVIF_RESULT_OUT_OF_MEMORY;
            }
            *dstProp = *srcProp;
        } else if memcmp(
            ((*item).type_0).as_mut_ptr() as *const core::ffi::c_void,
            ((*firstTileItem).type_0).as_mut_ptr() as *const core::ffi::c_void,
            4 as size_t,
        ) != 0
        {
            avifDiagnosticsPrintf(
                &mut (*decoder).diag as *mut avifDiagnostics,
                b"Tile item ID %u of type '%.4s' differs from other tile type '%.4s'\0"
                    as *const u8 as *const core::ffi::c_char,
                (*item).id,
                ((*item).type_0).as_mut_ptr() as *const core::ffi::c_char,
                ((*firstTileItem).type_0).as_mut_ptr() as *const core::ffi::c_char,
            );
            return AVIF_RESULT_INVALID_IMAGE_GRID;
        }
        dimgIdx = dimgIdx.wrapping_add(1);
    }
    return AVIF_RESULT_OK;
}
unsafe extern "C" fn avifDecoderAdoptGridTileCodecTypeIfNeeded(
    mut decoder: *mut avifDecoder,
    mut item: *mut avifDecoderItem,
    mut info: *const avifTileInfo,
) -> avifResult {
    if (*info).grid.rows > 0 as uint32_t && (*info).grid.columns > 0 as uint32_t {
        let numTiles: uint32_t = ((*info).grid.rows).wrapping_mul((*info).grid.columns);
        let mut dimgIdxToItemIdx: *mut uint32_t = avifAlloc(
            (numTiles as size_t)
                .wrapping_mul(::core::mem::size_of::<uint32_t>() as size_t),
        ) as *mut uint32_t;
        if dimgIdxToItemIdx.is_null() {
            return AVIF_RESULT_OUT_OF_MEMORY;
        }
        let mut result: avifResult = avifFillDimgIdxToItemIdxArray(
            dimgIdxToItemIdx,
            numTiles,
            item,
        );
        if result as core::ffi::c_uint
            == AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
        {
            result = avifDecoderAdoptGridTileCodecType(
                decoder,
                item,
                dimgIdxToItemIdx,
                numTiles,
            );
        }
        avifFree(dimgIdxToItemIdx as *mut core::ffi::c_void);
        let result__: avifResult = result;
        if result__ as core::ffi::c_uint
            != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
        {
            return result__;
        }
    }
    return AVIF_RESULT_OK;
}
unsafe extern "C" fn avifDecoderGenerateImageGridTiles(
    mut decoder: *mut avifDecoder,
    mut gridItem: *mut avifDecoderItem,
    mut itemCategory: avifItemCategory,
    mut dimgIdxToItemIdx: *const uint32_t,
    mut numTiles: uint32_t,
) -> avifResult {
    let mut progressive: avifBool = AVIF_TRUE;
    let mut dimgIdx: uint32_t = 0 as uint32_t;
    while dimgIdx < numTiles {
        let itemIdx: uint32_t = *dimgIdxToItemIdx.offset(dimgIdx as isize);
        'c_9531: {
            if itemIdx < (*(*gridItem).meta).items.count {} else {
                __assert_fail(
                    b"itemIdx < gridItem->meta->items.count\0" as *const u8
                        as *const core::ffi::c_char,
                    b"/home/runner/work/libavif-rs/libavif-rs/upstream/libavif/src/read.c\0"
                        as *const u8 as *const core::ffi::c_char,
                    1801 as core::ffi::c_uint,
                    (::core::mem::transmute::<
                        [u8; 125],
                        [core::ffi::c_char; 125],
                    >(
                        *b"avifResult avifDecoderGenerateImageGridTiles(avifDecoder *, avifDecoderItem *, avifItemCategory, const uint32_t *, uint32_t)\0",
                    ))
                        .as_ptr(),
                );
            }
        };
        let mut item: *mut avifDecoderItem = *((*(*gridItem).meta).items.item)
            .offset(itemIdx as isize);
        let tileCodecType: avifCodecType = avifGetCodecType(
            ((*item).type_0).as_mut_ptr(),
        ) as avifCodecType;
        if !(tileCodecType as core::ffi::c_uint
            != AVIF_CODEC_TYPE_UNKNOWN as core::ffi::c_int as core::ffi::c_uint)
        {
            return AVIF_RESULT_INVALID_IMAGE_GRID;
        }
        let mut tile: *const avifTile = avifDecoderDataCreateTile(
            (*decoder).data as *mut avifDecoderData,
            tileCodecType,
            (*item).width,
            (*item).height,
            avifDecoderItemOperatingPoint(item),
        );
        if tile.is_null() {
            return AVIF_RESULT_OUT_OF_MEMORY;
        }
        let result__: avifResult = avifCodecDecodeInputFillFromDecoderItem(
            (*tile).input,
            item,
            (*decoder).allowProgressive,
            (*decoder).imageCountLimit,
            (*(*decoder).io).sizeHint,
            &mut (*decoder).diag,
        ) as avifResult;
        if result__ as core::ffi::c_uint
            != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
        {
            return result__;
        }
        (*(*tile).input).itemCategory = itemCategory;
        if (*item).progressive == 0 {
            progressive = AVIF_FALSE as avifBool;
        }
        dimgIdx = dimgIdx.wrapping_add(1);
    }
    if itemCategory as core::ffi::c_uint
        == AVIF_ITEM_COLOR as core::ffi::c_int as core::ffi::c_uint && progressive != 0
    {
        (*gridItem).progressive = AVIF_TRUE as avifBool;
    }
    return AVIF_RESULT_OK;
}
unsafe extern "C" fn avifDecoderDataAllocateImagePlanes(
    mut data: *const avifDecoderData,
    mut info: *const avifTileInfo,
    mut dstImage: *mut avifImage,
    mut cicpSet: *mut avifBool,
) -> avifResult {
    let mut tile: *const avifTile = &mut *((*data).tiles.tile)
        .offset((*info).firstTileIndex as isize) as *mut avifTile;
    let mut dstWidth: uint32_t = 0;
    let mut dstHeight: uint32_t = 0;
    if (*info).grid.rows > 0 as uint32_t && (*info).grid.columns > 0 as uint32_t {
        let mut grid: *const avifImageGrid = &(*info).grid;
        if ((*(*tile).image).width as uint64_t).wrapping_mul((*grid).columns as uint64_t)
            < (*grid).outputWidth as uint64_t
            || ((*(*tile).image).height as uint64_t)
                .wrapping_mul((*grid).rows as uint64_t)
                < (*grid).outputHeight as uint64_t
        {
            avifDiagnosticsPrintf(
                (*data).diag,
                b"Grid image tiles do not completely cover the image (HEIF (ISO/IEC 23008-12:2017), Section 6.6.2.3.1)\0"
                    as *const u8 as *const core::ffi::c_char,
            );
            return AVIF_RESULT_INVALID_IMAGE_GRID;
        }
        if ((*(*tile).image).width as uint64_t)
            .wrapping_mul(((*grid).columns).wrapping_sub(1 as uint32_t) as uint64_t)
            >= (*grid).outputWidth as uint64_t
            || ((*(*tile).image).height as uint64_t)
                .wrapping_mul(((*grid).rows).wrapping_sub(1 as uint32_t) as uint64_t)
                >= (*grid).outputHeight as uint64_t
        {
            avifDiagnosticsPrintf(
                (*data).diag,
                b"Grid image tiles in the rightmost column and bottommost row do not overlap the reconstructed image grid canvas. See MIAF (ISO/IEC 23000-22:2019), Section 7.3.11.4.2, Figure 2\0"
                    as *const u8 as *const core::ffi::c_char,
            );
            return AVIF_RESULT_INVALID_IMAGE_GRID;
        }
        if avifAreGridDimensionsValid(
            (*(*tile).image).yuvFormat,
            (*grid).outputWidth,
            (*grid).outputHeight,
            (*(*tile).image).width,
            (*(*tile).image).height,
            (*data).diag,
        ) == 0
        {
            return AVIF_RESULT_INVALID_IMAGE_GRID;
        }
        dstWidth = (*grid).outputWidth;
        dstHeight = (*grid).outputHeight;
    } else {
        dstWidth = (*tile).width;
        dstHeight = (*tile).height;
    }
    let alpha: avifBool = avifIsAlpha((*(*tile).input).itemCategory) as avifBool;
    if alpha != 0 {
        'c_35206: {
            if (*(*tile).image).yuvFormat as core::ffi::c_uint
                == AVIF_PIXEL_FORMAT_NONE as core::ffi::c_int as core::ffi::c_uint
            {} else {
                __assert_fail(
                    b"tile->image->yuvFormat == AVIF_PIXEL_FORMAT_NONE\0" as *const u8
                        as *const core::ffi::c_char,
                    b"/home/runner/work/libavif-rs/libavif-rs/upstream/libavif/src/read.c\0"
                        as *const u8 as *const core::ffi::c_char,
                    1873 as core::ffi::c_uint,
                    (::core::mem::transmute::<
                        [u8; 118],
                        [core::ffi::c_char; 118],
                    >(
                        *b"avifResult avifDecoderDataAllocateImagePlanes(const avifDecoderData *, const avifTileInfo *, avifImage *, avifBool *)\0",
                    ))
                        .as_ptr(),
                );
            }
        };
    }
    let dstDepth: uint32_t = (*(*tile).image).depth;
    let dimsOrDepthIsDifferent: avifBool = ((*dstImage).width != dstWidth
        || (*dstImage).height != dstHeight || (*dstImage).depth != dstDepth)
        as core::ffi::c_int;
    let yuvFormatIsDifferent: avifBool = (alpha == 0
        && (*dstImage).yuvFormat as core::ffi::c_uint
            != (*(*tile).image).yuvFormat as core::ffi::c_uint) as core::ffi::c_int;
    if dimsOrDepthIsDifferent != 0 || yuvFormatIsDifferent != 0 {
        if alpha != 0 {
            avifDiagnosticsPrintf(
                (*data).diag,
                b"Alpha plane dimensions do not match color plane dimensions\0"
                    as *const u8 as *const core::ffi::c_char,
            );
            return AVIF_RESULT_INVALID_IMAGE_GRID;
        }
        if dimsOrDepthIsDifferent != 0 {
            avifImageFreePlanes(
                dstImage,
                AVIF_PLANES_ALL as core::ffi::c_int as avifPlanesFlags,
            );
            (*dstImage).width = dstWidth;
            (*dstImage).height = dstHeight;
            (*dstImage).depth = dstDepth;
        }
        if yuvFormatIsDifferent != 0 {
            avifImageFreePlanes(
                dstImage,
                AVIF_PLANES_YUV as core::ffi::c_int as avifPlanesFlags,
            );
            (*dstImage).yuvFormat = (*(*tile).image).yuvFormat;
        }
        if *cicpSet == 0 {
            *cicpSet = AVIF_TRUE as avifBool;
            (*dstImage).colorPrimaries = (*(*tile).image).colorPrimaries;
            (*dstImage).transferCharacteristics = (*(*tile).image)
                .transferCharacteristics;
            (*dstImage).matrixCoefficients = (*(*tile).image).matrixCoefficients;
        }
    }
    if avifImageAllocatePlanes(
        dstImage,
        (if alpha != 0 {
            AVIF_PLANES_A as core::ffi::c_int
        } else {
            AVIF_PLANES_YUV as core::ffi::c_int
        }) as avifPlanesFlags,
    ) as core::ffi::c_uint != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
    {
        avifDiagnosticsPrintf(
            (*data).diag,
            b"Image allocation failure\0" as *const u8 as *const core::ffi::c_char,
        );
        return AVIF_RESULT_OUT_OF_MEMORY;
    }
    return AVIF_RESULT_OK;
}
unsafe extern "C" fn avifDecoderDataCopyTileToImage(
    mut data: *mut avifDecoderData,
    mut info: *const avifTileInfo,
    mut dstImage: *mut avifImage,
    mut tile: *const avifTile,
    mut tileIndex: core::ffi::c_uint,
) -> avifResult {
    let mut firstTile: *const avifTile = &mut *((*data).tiles.tile)
        .offset((*info).firstTileIndex as isize) as *mut avifTile;
    if tile != firstTile {
        if (*(*tile).image).width != (*(*firstTile).image).width
            || (*(*tile).image).height != (*(*firstTile).image).height
            || (*(*tile).image).depth != (*(*firstTile).image).depth
            || (*(*tile).image).yuvFormat as core::ffi::c_uint
                != (*(*firstTile).image).yuvFormat as core::ffi::c_uint
            || (*(*tile).image).yuvRange as core::ffi::c_uint
                != (*(*firstTile).image).yuvRange as core::ffi::c_uint
            || (*(*tile).image).colorPrimaries as core::ffi::c_int
                != (*(*firstTile).image).colorPrimaries as core::ffi::c_int
            || (*(*tile).image).transferCharacteristics as core::ffi::c_int
                != (*(*firstTile).image).transferCharacteristics as core::ffi::c_int
            || (*(*tile).image).matrixCoefficients as core::ffi::c_int
                != (*(*firstTile).image).matrixCoefficients as core::ffi::c_int
        {
            avifDiagnosticsPrintf(
                (*data).diag,
                b"Grid image contains mismatched tiles\0" as *const u8
                    as *const core::ffi::c_char,
            );
            return AVIF_RESULT_INVALID_IMAGE_GRID;
        }
    }
    let mut dstView: avifImage = avifImage {
        width: 0,
        height: 0,
        depth: 0,
        yuvFormat: AVIF_PIXEL_FORMAT_NONE,
        yuvRange: AVIF_RANGE_LIMITED,
        yuvChromaSamplePosition: AVIF_CHROMA_SAMPLE_POSITION_UNKNOWN,
        yuvPlanes: [0 as *mut uint8_t; 3],
        yuvRowBytes: [0; 3],
        imageOwnsYUVPlanes: 0,
        alphaPlane: 0 as *mut uint8_t,
        alphaRowBytes: 0,
        imageOwnsAlphaPlane: 0,
        alphaPremultiplied: 0,
        icc: avifRWData {
            data: 0 as *mut uint8_t,
            size: 0,
        },
        colorPrimaries: 0,
        transferCharacteristics: 0,
        matrixCoefficients: 0,
        clli: avifContentLightLevelInformationBox {
            maxCLL: 0,
            maxPALL: 0,
        },
        transformFlags: 0,
        pasp: avifPixelAspectRatioBox {
            hSpacing: 0,
            vSpacing: 0,
        },
        clap: avifCleanApertureBox {
            widthN: 0,
            widthD: 0,
            heightN: 0,
            heightD: 0,
            horizOffN: 0,
            horizOffD: 0,
            vertOffN: 0,
            vertOffD: 0,
        },
        irot: avifImageRotation { angle: 0 },
        imir: avifImageMirror { axis: 0 },
        exif: avifRWData {
            data: 0 as *mut uint8_t,
            size: 0,
        },
        xmp: avifRWData {
            data: 0 as *mut uint8_t,
            size: 0,
        },
        properties: 0 as *mut avifImageItemProperty,
        numProperties: 0,
        gainMap: 0 as *mut avifGainMap,
    };
    avifImageSetDefaults(&mut dstView);
    let srcViewRect: avifCropRect = {
        let mut init = avifCropRect {
            x: 0 as uint32_t,
            y: 0 as uint32_t,
            width: (*dstImage).width,
            height: (*dstImage).height,
        };
        init
    };
    'c_34675: {
        if avifImageSetViewRect(&mut dstView, dstImage, &srcViewRect)
            as core::ffi::c_uint
            == AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
        {} else {
            __assert_fail(
                b"avifImageSetViewRect(&dstView, dstImage, &srcViewRect) == AVIF_RESULT_OK\0"
                    as *const u8 as *const core::ffi::c_char,
                b"/home/runner/work/libavif-rs/libavif-rs/upstream/libavif/src/read.c\0"
                    as *const u8 as *const core::ffi::c_char,
                1944 as core::ffi::c_uint,
                (::core::mem::transmute::<
                    [u8; 128],
                    [core::ffi::c_char; 128],
                >(
                    *b"avifResult avifDecoderDataCopyTileToImage(avifDecoderData *, const avifTileInfo *, avifImage *, const avifTile *, unsigned int)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if avifIsAlpha((*(*tile).input).itemCategory) != 0 {
        avifImageFreePlanes(
            &mut dstView,
            AVIF_PLANES_YUV as core::ffi::c_int as avifPlanesFlags,
        );
        dstView.yuvFormat = AVIF_PIXEL_FORMAT_NONE;
    } else {
        avifImageFreePlanes(
            &mut dstView,
            AVIF_PLANES_A as core::ffi::c_int as avifPlanesFlags,
        );
    }
    let mut srcTileView: avifImage = avifImage {
        width: 0,
        height: 0,
        depth: 0,
        yuvFormat: AVIF_PIXEL_FORMAT_NONE,
        yuvRange: AVIF_RANGE_LIMITED,
        yuvChromaSamplePosition: AVIF_CHROMA_SAMPLE_POSITION_UNKNOWN,
        yuvPlanes: [0 as *mut uint8_t; 3],
        yuvRowBytes: [0; 3],
        imageOwnsYUVPlanes: 0,
        alphaPlane: 0 as *mut uint8_t,
        alphaRowBytes: 0,
        imageOwnsAlphaPlane: 0,
        alphaPremultiplied: 0,
        icc: avifRWData {
            data: 0 as *mut uint8_t,
            size: 0,
        },
        colorPrimaries: 0,
        transferCharacteristics: 0,
        matrixCoefficients: 0,
        clli: avifContentLightLevelInformationBox {
            maxCLL: 0,
            maxPALL: 0,
        },
        transformFlags: 0,
        pasp: avifPixelAspectRatioBox {
            hSpacing: 0,
            vSpacing: 0,
        },
        clap: avifCleanApertureBox {
            widthN: 0,
            widthD: 0,
            heightN: 0,
            heightD: 0,
            horizOffN: 0,
            horizOffD: 0,
            vertOffN: 0,
            vertOffD: 0,
        },
        irot: avifImageRotation { angle: 0 },
        imir: avifImageMirror { axis: 0 },
        exif: avifRWData {
            data: 0 as *mut uint8_t,
            size: 0,
        },
        xmp: avifRWData {
            data: 0 as *mut uint8_t,
            size: 0,
        },
        properties: 0 as *mut avifImageItemProperty,
        numProperties: 0,
        gainMap: 0 as *mut avifGainMap,
    };
    avifImageSetDefaults(&mut srcTileView);
    let mut dstTileView: avifImage = avifImage {
        width: 0,
        height: 0,
        depth: 0,
        yuvFormat: AVIF_PIXEL_FORMAT_NONE,
        yuvRange: AVIF_RANGE_LIMITED,
        yuvChromaSamplePosition: AVIF_CHROMA_SAMPLE_POSITION_UNKNOWN,
        yuvPlanes: [0 as *mut uint8_t; 3],
        yuvRowBytes: [0; 3],
        imageOwnsYUVPlanes: 0,
        alphaPlane: 0 as *mut uint8_t,
        alphaRowBytes: 0,
        imageOwnsAlphaPlane: 0,
        alphaPremultiplied: 0,
        icc: avifRWData {
            data: 0 as *mut uint8_t,
            size: 0,
        },
        colorPrimaries: 0,
        transferCharacteristics: 0,
        matrixCoefficients: 0,
        clli: avifContentLightLevelInformationBox {
            maxCLL: 0,
            maxPALL: 0,
        },
        transformFlags: 0,
        pasp: avifPixelAspectRatioBox {
            hSpacing: 0,
            vSpacing: 0,
        },
        clap: avifCleanApertureBox {
            widthN: 0,
            widthD: 0,
            heightN: 0,
            heightD: 0,
            horizOffN: 0,
            horizOffD: 0,
            vertOffN: 0,
            vertOffD: 0,
        },
        irot: avifImageRotation { angle: 0 },
        imir: avifImageMirror { axis: 0 },
        exif: avifRWData {
            data: 0 as *mut uint8_t,
            size: 0,
        },
        xmp: avifRWData {
            data: 0 as *mut uint8_t,
            size: 0,
        },
        properties: 0 as *mut avifImageItemProperty,
        numProperties: 0,
        gainMap: 0 as *mut avifGainMap,
    };
    avifImageSetDefaults(&mut dstTileView);
    let mut dstTileViewRect: avifCropRect = {
        let mut init = avifCropRect {
            x: 0 as uint32_t,
            y: 0 as uint32_t,
            width: (*(*firstTile).image).width,
            height: (*(*firstTile).image).height,
        };
        init
    };
    if (*info).grid.rows > 0 as uint32_t && (*info).grid.columns > 0 as uint32_t {
        let mut rowIndex: core::ffi::c_uint = tileIndex
            .wrapping_div((*info).grid.columns as core::ffi::c_uint);
        let mut colIndex: core::ffi::c_uint = tileIndex
            .wrapping_rem((*info).grid.columns as core::ffi::c_uint);
        dstTileViewRect.x = ((*(*firstTile).image).width)
            .wrapping_mul(colIndex as uint32_t);
        dstTileViewRect.y = ((*(*firstTile).image).height)
            .wrapping_mul(rowIndex as uint32_t);
        if (dstTileViewRect.x).wrapping_add(dstTileViewRect.width)
            > (*info).grid.outputWidth
        {
            dstTileViewRect.width = ((*info).grid.outputWidth)
                .wrapping_sub(dstTileViewRect.x);
        }
        if (dstTileViewRect.y).wrapping_add(dstTileViewRect.height)
            > (*info).grid.outputHeight
        {
            dstTileViewRect.height = ((*info).grid.outputHeight)
                .wrapping_sub(dstTileViewRect.y);
        }
    }
    let srcTileViewRect: avifCropRect = {
        let mut init = avifCropRect {
            x: 0 as uint32_t,
            y: 0 as uint32_t,
            width: dstTileViewRect.width,
            height: dstTileViewRect.height,
        };
        init
    };
    'c_34450: {
        if avifImageSetViewRect(&mut dstTileView, &mut dstView, &mut dstTileViewRect)
            as core::ffi::c_uint
            == AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
        {} else {
            __assert_fail(
                b"avifImageSetViewRect(&dstTileView, &dstView, &dstTileViewRect) == AVIF_RESULT_OK\0"
                    as *const u8 as *const core::ffi::c_char,
                b"/home/runner/work/libavif-rs/libavif-rs/upstream/libavif/src/read.c\0"
                    as *const u8 as *const core::ffi::c_char,
                1970 as core::ffi::c_uint,
                (::core::mem::transmute::<
                    [u8; 128],
                    [core::ffi::c_char; 128],
                >(
                    *b"avifResult avifDecoderDataCopyTileToImage(avifDecoderData *, const avifTileInfo *, avifImage *, const avifTile *, unsigned int)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    'c_34342: {
        if avifImageSetViewRect(&mut srcTileView, (*tile).image, &srcTileViewRect)
            as core::ffi::c_uint
            == AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
        {} else {
            __assert_fail(
                b"avifImageSetViewRect(&srcTileView, tile->image, &srcTileViewRect) == AVIF_RESULT_OK\0"
                    as *const u8 as *const core::ffi::c_char,
                b"/home/runner/work/libavif-rs/libavif-rs/upstream/libavif/src/read.c\0"
                    as *const u8 as *const core::ffi::c_char,
                1971 as core::ffi::c_uint,
                (::core::mem::transmute::<
                    [u8; 128],
                    [core::ffi::c_char; 128],
                >(
                    *b"avifResult avifDecoderDataCopyTileToImage(avifDecoderData *, const avifTileInfo *, avifImage *, const avifTile *, unsigned int)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    avifImageCopySamples(
        &mut dstTileView,
        &mut srcTileView,
        (if avifIsAlpha((*(*tile).input).itemCategory) != 0 {
            AVIF_PLANES_A as core::ffi::c_int
        } else {
            AVIF_PLANES_YUV as core::ffi::c_int
        }) as avifPlanesFlags,
    );
    return AVIF_RESULT_OK;
}
unsafe extern "C" fn avifDecoderFindMetadata(
    mut decoder: *mut avifDecoder,
    mut meta: *mut avifMeta,
    mut image: *mut avifImage,
    mut colorId: uint32_t,
) -> avifResult {
    if (*decoder).ignoreExif != 0 && (*decoder).ignoreXMP != 0 {
        return AVIF_RESULT_OK;
    }
    let mut itemIndex: uint32_t = 0 as uint32_t;
    while itemIndex < (*meta).items.count {
        let mut item: *mut avifDecoderItem = *((*meta).items.item)
            .offset(itemIndex as isize);
        if !((*item).size == 0) {
            if !((*item).hasUnsupportedEssentialProperty != 0) {
                if !(colorId > 0 as uint32_t
                    && avifReferences(
                        &mut (*item).references,
                        b"cdsc\0" as *const u8 as *const core::ffi::c_char,
                        colorId,
                    ) == 0)
                {
                    if (*decoder).ignoreExif == 0
                        && memcmp(
                            ((*item).type_0).as_mut_ptr() as *const core::ffi::c_void,
                            b"Exif\0" as *const u8 as *const core::ffi::c_char
                                as *const core::ffi::c_void,
                            4 as size_t,
                        ) == 0
                    {
                        let mut exifContents: avifROData = avifROData {
                            data: 0 as *const uint8_t,
                            size: 0,
                        };
                        let mut readResult: avifResult = avifDecoderItemRead(
                            item,
                            (*decoder).io,
                            &mut exifContents,
                            0 as size_t,
                            0 as size_t,
                            &mut (*decoder).diag,
                        );
                        if readResult as core::ffi::c_uint
                            != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
                        {
                            return readResult;
                        }
                        let mut exifBoxStream: avifROStream = avifROStream {
                            raw: 0 as *mut avifROData,
                            offset: 0,
                            numUsedBitsInPartialByte: 0,
                            diag: 0 as *mut avifDiagnostics,
                            diagContext: 0 as *const core::ffi::c_char,
                        };
                        let mut exifBoxStream_roData: avifROData = avifROData {
                            data: 0 as *const uint8_t,
                            size: 0,
                        };
                        exifBoxStream_roData.data = exifContents.data;
                        exifBoxStream_roData.size = exifContents.size;
                        avifROStreamStart(
                            &mut exifBoxStream,
                            &mut exifBoxStream_roData,
                            &mut (*decoder).diag,
                            b"Exif header\0" as *const u8 as *const core::ffi::c_char,
                        );
                        let mut exifTiffHeaderOffset: uint32_t = 0;
                        if avifROStreamReadU32(
                            &mut exifBoxStream,
                            &mut exifTiffHeaderOffset,
                        ) == 0
                        {
                            return AVIF_RESULT_INVALID_EXIF_PAYLOAD;
                        }
                        let mut expectedExifTiffHeaderOffset: size_t = 0;
                        let result__: avifResult = avifGetExifTiffHeaderOffset(
                            avifROStreamCurrent(&mut exifBoxStream),
                            avifROStreamRemainingBytes(&mut exifBoxStream),
                            &mut expectedExifTiffHeaderOffset,
                        ) as avifResult;
                        if result__ as core::ffi::c_uint
                            != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
                        {
                            return result__;
                        }
                        if !(exifTiffHeaderOffset as size_t
                            == expectedExifTiffHeaderOffset)
                        {
                            return AVIF_RESULT_INVALID_EXIF_PAYLOAD;
                        }
                        let result___0: avifResult = avifRWDataSet(
                            &mut (*image).exif,
                            avifROStreamCurrent(&mut exifBoxStream),
                            avifROStreamRemainingBytes(&mut exifBoxStream),
                        ) as avifResult;
                        if result___0 as core::ffi::c_uint
                            != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
                        {
                            return result___0;
                        }
                    } else if (*decoder).ignoreXMP == 0
                        && memcmp(
                            ((*item).type_0).as_mut_ptr() as *const core::ffi::c_void,
                            b"mime\0" as *const u8 as *const core::ffi::c_char
                                as *const core::ffi::c_void,
                            4 as size_t,
                        ) == 0
                        && strcmp(
                            ((*item).contentType.contentType).as_mut_ptr(),
                            AVIF_CONTENT_TYPE_XMP.as_ptr(),
                        ) == 0
                    {
                        let mut xmpContents: avifROData = avifROData {
                            data: 0 as *const uint8_t,
                            size: 0,
                        };
                        let mut readResult_0: avifResult = avifDecoderItemRead(
                            item,
                            (*decoder).io,
                            &mut xmpContents,
                            0 as size_t,
                            0 as size_t,
                            &mut (*decoder).diag,
                        );
                        if readResult_0 as core::ffi::c_uint
                            != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
                        {
                            return readResult_0;
                        }
                        let result___1: avifResult = avifImageSetMetadataXMP(
                            image,
                            xmpContents.data,
                            xmpContents.size,
                        ) as avifResult;
                        if result___1 as core::ffi::c_uint
                            != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
                        {
                            return result___1;
                        }
                    }
                }
            }
        }
        itemIndex = itemIndex.wrapping_add(1);
    }
    return AVIF_RESULT_OK;
}
unsafe extern "C" fn isAlphaURN(mut urn: *const core::ffi::c_char) -> avifBool {
    return (strcmp(urn, AVIF_URN_ALPHA0.as_ptr()) == 0
        || strcmp(urn, AVIF_URN_ALPHA1.as_ptr()) == 0) as core::ffi::c_int;
}
unsafe extern "C" fn avifParseHandlerBox(
    mut raw: *const uint8_t,
    mut rawLen: size_t,
    mut handlerType: *mut uint8_t,
    mut diag: *mut avifDiagnostics,
) -> avifBool {
    let mut s: avifROStream = avifROStream {
        raw: 0 as *mut avifROData,
        offset: 0,
        numUsedBitsInPartialByte: 0,
        diag: 0 as *mut avifDiagnostics,
        diagContext: 0 as *const core::ffi::c_char,
    };
    let mut s_roData: avifROData = avifROData {
        data: 0 as *const uint8_t,
        size: 0,
    };
    s_roData.data = raw;
    s_roData.size = rawLen;
    avifROStreamStart(
        &mut s,
        &mut s_roData,
        diag,
        b"Box[hdlr]\0" as *const u8 as *const core::ffi::c_char,
    );
    if avifROStreamReadAndEnforceVersion(&mut s, 0 as uint8_t, 0 as *mut uint32_t) == 0 {
        return AVIF_FALSE;
    }
    let mut predefined: uint32_t = 0;
    if avifROStreamReadU32(&mut s, &mut predefined) == 0 {
        return AVIF_FALSE;
    }
    if predefined != 0 as uint32_t {
        avifDiagnosticsPrintf(
            diag,
            b"Box[hdlr] contains a pre_defined value that is nonzero\0" as *const u8
                as *const core::ffi::c_char,
        );
        return AVIF_FALSE;
    }
    if avifROStreamRead(&mut s, handlerType as *mut uint8_t, 4 as size_t) == 0 {
        return AVIF_FALSE;
    }
    let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
    while i < 3 as core::ffi::c_int {
        let mut reserved: uint32_t = 0;
        if avifROStreamReadU32(&mut s, &mut reserved) == 0 {
            return AVIF_FALSE;
        }
        i += 1;
    }
    if avifROStreamReadString(&mut s, 0 as *mut core::ffi::c_char, 0 as size_t) == 0 {
        return AVIF_FALSE;
    }
    return AVIF_TRUE;
}
unsafe extern "C" fn avifParseItemLocationBox(
    mut meta: *mut avifMeta,
    mut raw: *const uint8_t,
    mut rawLen: size_t,
    mut diag: *mut avifDiagnostics,
) -> avifResult {
    let mut s: avifROStream = avifROStream {
        raw: 0 as *mut avifROData,
        offset: 0,
        numUsedBitsInPartialByte: 0,
        diag: 0 as *mut avifDiagnostics,
        diagContext: 0 as *const core::ffi::c_char,
    };
    let mut s_roData: avifROData = avifROData {
        data: 0 as *const uint8_t,
        size: 0,
    };
    s_roData.data = raw;
    s_roData.size = rawLen;
    avifROStreamStart(
        &mut s,
        &mut s_roData,
        diag,
        b"Box[iloc]\0" as *const u8 as *const core::ffi::c_char,
    );
    let mut version: uint8_t = 0;
    if avifROStreamReadVersionAndFlags(&mut s, &mut version, 0 as *mut uint32_t) == 0 {
        return AVIF_RESULT_BMFF_PARSE_FAILED;
    }
    if version as core::ffi::c_int > 2 as core::ffi::c_int {
        avifDiagnosticsPrintf(
            diag,
            b"Box[iloc] has an unsupported version [%u]\0" as *const u8
                as *const core::ffi::c_char,
            version as core::ffi::c_int,
        );
        return AVIF_RESULT_BMFF_PARSE_FAILED;
    }
    let mut offsetSize: uint8_t = 0;
    let mut lengthSize: uint8_t = 0;
    let mut baseOffsetSize: uint8_t = 0;
    let mut indexSize: uint8_t = 0 as uint8_t;
    let mut reserved: uint32_t = 0;
    if avifROStreamReadBitsU8(&mut s, &mut offsetSize, 4 as size_t) == 0 {
        return AVIF_RESULT_BMFF_PARSE_FAILED;
    }
    if avifROStreamReadBitsU8(&mut s, &mut lengthSize, 4 as size_t) == 0 {
        return AVIF_RESULT_BMFF_PARSE_FAILED;
    }
    if avifROStreamReadBitsU8(&mut s, &mut baseOffsetSize, 4 as size_t) == 0 {
        return AVIF_RESULT_BMFF_PARSE_FAILED;
    }
    if version as core::ffi::c_int == 1 as core::ffi::c_int
        || version as core::ffi::c_int == 2 as core::ffi::c_int
    {
        if avifROStreamReadBitsU8(&mut s, &mut indexSize, 4 as size_t) == 0 {
            return AVIF_RESULT_BMFF_PARSE_FAILED;
        }
    } else if avifROStreamReadBitsU32(&mut s, &mut reserved, 4 as size_t) == 0 {
        return AVIF_RESULT_BMFF_PARSE_FAILED
    }
    if offsetSize as core::ffi::c_int != 0 as core::ffi::c_int
        && offsetSize as core::ffi::c_int != 4 as core::ffi::c_int
        && offsetSize as core::ffi::c_int != 8 as core::ffi::c_int
        || lengthSize as core::ffi::c_int != 0 as core::ffi::c_int
            && lengthSize as core::ffi::c_int != 4 as core::ffi::c_int
            && lengthSize as core::ffi::c_int != 8 as core::ffi::c_int
        || baseOffsetSize as core::ffi::c_int != 0 as core::ffi::c_int
            && baseOffsetSize as core::ffi::c_int != 4 as core::ffi::c_int
            && baseOffsetSize as core::ffi::c_int != 8 as core::ffi::c_int
        || indexSize as core::ffi::c_int != 0 as core::ffi::c_int
            && indexSize as core::ffi::c_int != 4 as core::ffi::c_int
            && indexSize as core::ffi::c_int != 8 as core::ffi::c_int
    {
        avifDiagnosticsPrintf(
            diag,
            b"Box[iloc] has an invalid size\0" as *const u8 as *const core::ffi::c_char,
        );
        return AVIF_RESULT_BMFF_PARSE_FAILED;
    }
    let mut tmp16: uint16_t = 0;
    let mut itemCount: uint32_t = 0;
    if (version as core::ffi::c_int) < 2 as core::ffi::c_int {
        if avifROStreamReadU16(&mut s, &mut tmp16) == 0 {
            return AVIF_RESULT_BMFF_PARSE_FAILED;
        }
        itemCount = tmp16 as uint32_t;
    } else if avifROStreamReadU32(&mut s, &mut itemCount) == 0 {
        return AVIF_RESULT_BMFF_PARSE_FAILED
    }
    let mut i: uint32_t = 0 as uint32_t;
    while i < itemCount {
        let mut itemID: uint32_t = 0;
        if (version as core::ffi::c_int) < 2 as core::ffi::c_int {
            if avifROStreamReadU16(&mut s, &mut tmp16) == 0 {
                return AVIF_RESULT_BMFF_PARSE_FAILED;
            }
            itemID = tmp16 as uint32_t;
        } else if avifROStreamReadU32(&mut s, &mut itemID) == 0 {
            return AVIF_RESULT_BMFF_PARSE_FAILED
        }
        let result__: avifResult = avifCheckItemID(
            b"iloc\0" as *const u8 as *const core::ffi::c_char,
            itemID,
            diag,
        ) as avifResult;
        if result__ as core::ffi::c_uint
            != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
        {
            return result__;
        }
        let mut item: *mut avifDecoderItem = 0 as *mut avifDecoderItem;
        let result___0: avifResult = avifMetaFindOrCreateItem(meta, itemID, &mut item)
            as avifResult;
        if result___0 as core::ffi::c_uint
            != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
        {
            return result___0;
        }
        if (*item).extents.count > 0 as uint32_t {
            avifDiagnosticsPrintf(
                diag,
                b"Item ID [%u] contains duplicate sets of extents\0" as *const u8
                    as *const core::ffi::c_char,
                itemID,
            );
            return AVIF_RESULT_BMFF_PARSE_FAILED;
        }
        if version as core::ffi::c_int == 1 as core::ffi::c_int
            || version as core::ffi::c_int == 2 as core::ffi::c_int
        {
            if avifROStreamReadBitsU32(&mut s, &mut reserved, 12 as size_t) == 0 {
                return AVIF_RESULT_BMFF_PARSE_FAILED;
            }
            if reserved != 0 {
                avifDiagnosticsPrintf(
                    diag,
                    b"Box[iloc] has a non null reserved field [%u]\0" as *const u8
                        as *const core::ffi::c_char,
                    reserved,
                );
                return AVIF_RESULT_BMFF_PARSE_FAILED;
            }
            let mut constructionMethod: uint8_t = 0;
            if avifROStreamReadBitsU8(&mut s, &mut constructionMethod, 4 as size_t) == 0
            {
                return AVIF_RESULT_BMFF_PARSE_FAILED;
            }
            if constructionMethod as core::ffi::c_int != 0 as core::ffi::c_int
                && constructionMethod as core::ffi::c_int != 1 as core::ffi::c_int
            {
                avifDiagnosticsPrintf(
                    diag,
                    b"Box[iloc] has an unsupported construction method [%u]\0"
                        as *const u8 as *const core::ffi::c_char,
                    constructionMethod as core::ffi::c_int,
                );
                return AVIF_RESULT_BMFF_PARSE_FAILED;
            }
            if constructionMethod as core::ffi::c_int == 1 as core::ffi::c_int {
                (*item).idatStored = AVIF_TRUE as avifBool;
            }
        }
        let mut dataReferenceIndex: uint16_t = 0;
        if avifROStreamReadU16(&mut s, &mut dataReferenceIndex) == 0 {
            return AVIF_RESULT_BMFF_PARSE_FAILED;
        }
        let mut baseOffset: uint64_t = 0;
        if avifROStreamReadUX8(&mut s, &mut baseOffset, baseOffsetSize as uint64_t) == 0
        {
            return AVIF_RESULT_BMFF_PARSE_FAILED;
        }
        let mut extentCount: uint16_t = 0;
        if avifROStreamReadU16(&mut s, &mut extentCount) == 0 {
            return AVIF_RESULT_BMFF_PARSE_FAILED;
        }
        let mut extentIter: core::ffi::c_int = 0 as core::ffi::c_int;
        while extentIter < extentCount as core::ffi::c_int {
            if (version as core::ffi::c_int == 1 as core::ffi::c_int
                || version as core::ffi::c_int == 2 as core::ffi::c_int)
                && indexSize as core::ffi::c_int > 0 as core::ffi::c_int
            {
                let mut itemReferenceIndex: uint64_t = 0;
                if avifROStreamReadUX8(
                    &mut s,
                    &mut itemReferenceIndex,
                    indexSize as uint64_t,
                ) == 0
                {
                    return AVIF_RESULT_BMFF_PARSE_FAILED;
                }
            }
            let mut extentOffset: uint64_t = 0;
            if avifROStreamReadUX8(&mut s, &mut extentOffset, offsetSize as uint64_t)
                == 0
            {
                return AVIF_RESULT_BMFF_PARSE_FAILED;
            }
            let mut extentLength: uint64_t = 0;
            if avifROStreamReadUX8(&mut s, &mut extentLength, lengthSize as uint64_t)
                == 0
            {
                return AVIF_RESULT_BMFF_PARSE_FAILED;
            }
            let mut extent: *mut avifExtent = avifArrayPush(
                &mut (*item).extents as *mut avifExtentArray as *mut core::ffi::c_void,
            ) as *mut avifExtent;
            if extent.is_null() {
                return AVIF_RESULT_OUT_OF_MEMORY;
            }
            if extentOffset > (UINT64_MAX as uint64_t).wrapping_sub(baseOffset) {
                avifDiagnosticsPrintf(
                    diag,
                    b"Item ID [%u] contains an extent offset which overflows: [base: %lu offset:%lu]\0"
                        as *const u8 as *const core::ffi::c_char,
                    itemID,
                    baseOffset,
                    extentOffset,
                );
                return AVIF_RESULT_BMFF_PARSE_FAILED;
            }
            let mut offset: uint64_t = baseOffset.wrapping_add(extentOffset);
            (*extent).offset = offset;
            (*extent).size = extentLength as size_t;
            if (*extent).size > (SIZE_MAX as size_t).wrapping_sub((*item).size) {
                avifDiagnosticsPrintf(
                    diag,
                    b"Item ID [%u] contains an extent length which overflows the item size: [%zu, %zu]\0"
                        as *const u8 as *const core::ffi::c_char,
                    itemID,
                    (*extent).size,
                    (*item).size,
                );
                return AVIF_RESULT_BMFF_PARSE_FAILED;
            }
            (*item).size = ((*item).size).wrapping_add((*extent).size);
            extentIter += 1;
        }
        i = i.wrapping_add(1);
    }
    return AVIF_RESULT_OK;
}
unsafe extern "C" fn avifParseImageGridBox(
    mut grid: *mut avifImageGrid,
    mut raw: *const uint8_t,
    mut rawLen: size_t,
    mut imageSizeLimit: uint32_t,
    mut imageDimensionLimit: uint32_t,
    mut diag: *mut avifDiagnostics,
) -> avifResult {
    let mut s: avifROStream = avifROStream {
        raw: 0 as *mut avifROData,
        offset: 0,
        numUsedBitsInPartialByte: 0,
        diag: 0 as *mut avifDiagnostics,
        diagContext: 0 as *const core::ffi::c_char,
    };
    let mut s_roData: avifROData = avifROData {
        data: 0 as *const uint8_t,
        size: 0,
    };
    s_roData.data = raw;
    s_roData.size = rawLen;
    avifROStreamStart(
        &mut s,
        &mut s_roData,
        diag,
        b"Box[grid]\0" as *const u8 as *const core::ffi::c_char,
    );
    let mut version: uint8_t = 0;
    let mut flags: uint8_t = 0;
    if avifROStreamRead(&mut s, &mut version, 1 as size_t) == 0 {
        return AVIF_RESULT_BMFF_PARSE_FAILED;
    }
    if version as core::ffi::c_int != 0 as core::ffi::c_int {
        avifDiagnosticsPrintf(
            diag,
            b"Box[grid] has unsupported version [%u]\0" as *const u8
                as *const core::ffi::c_char,
            version as core::ffi::c_int,
        );
        return AVIF_RESULT_NOT_IMPLEMENTED;
    }
    let mut rowsMinusOne: uint8_t = 0;
    let mut columnsMinusOne: uint8_t = 0;
    if avifROStreamRead(&mut s, &mut flags, 1 as size_t) == 0 {
        return AVIF_RESULT_BMFF_PARSE_FAILED;
    }
    if avifROStreamRead(&mut s, &mut rowsMinusOne, 1 as size_t) == 0 {
        return AVIF_RESULT_BMFF_PARSE_FAILED;
    }
    if avifROStreamRead(&mut s, &mut columnsMinusOne, 1 as size_t) == 0 {
        return AVIF_RESULT_BMFF_PARSE_FAILED;
    }
    (*grid).rows = (rowsMinusOne as uint32_t).wrapping_add(1 as uint32_t);
    (*grid).columns = (columnsMinusOne as uint32_t).wrapping_add(1 as uint32_t);
    let mut fieldLength: uint32_t = (((flags as core::ffi::c_int & 1 as core::ffi::c_int)
        + 1 as core::ffi::c_int) * 16 as core::ffi::c_int) as uint32_t;
    if fieldLength == 16 as uint32_t {
        let mut outputWidth16: uint16_t = 0;
        let mut outputHeight16: uint16_t = 0;
        if avifROStreamReadU16(&mut s, &mut outputWidth16) == 0 {
            return AVIF_RESULT_BMFF_PARSE_FAILED;
        }
        if avifROStreamReadU16(&mut s, &mut outputHeight16) == 0 {
            return AVIF_RESULT_BMFF_PARSE_FAILED;
        }
        (*grid).outputWidth = outputWidth16 as uint32_t;
        (*grid).outputHeight = outputHeight16 as uint32_t;
    } else {
        if fieldLength != 32 as uint32_t {
            avifDiagnosticsPrintf(
                diag,
                b"Grid box contains illegal field length: [%u]\0" as *const u8
                    as *const core::ffi::c_char,
                fieldLength,
            );
            return AVIF_RESULT_INVALID_IMAGE_GRID;
        }
        if avifROStreamReadU32(&mut s, &mut (*grid).outputWidth) == 0 {
            return AVIF_RESULT_BMFF_PARSE_FAILED;
        }
        if avifROStreamReadU32(&mut s, &mut (*grid).outputHeight) == 0 {
            return AVIF_RESULT_BMFF_PARSE_FAILED;
        }
    }
    if (*grid).outputWidth == 0 as uint32_t || (*grid).outputHeight == 0 as uint32_t {
        avifDiagnosticsPrintf(
            diag,
            b"Grid box contains illegal dimensions: [%u x %u]\0" as *const u8
                as *const core::ffi::c_char,
            (*grid).outputWidth,
            (*grid).outputHeight,
        );
        return AVIF_RESULT_INVALID_IMAGE_GRID;
    }
    if avifDimensionsTooLarge(
        (*grid).outputWidth,
        (*grid).outputHeight,
        imageSizeLimit,
        imageDimensionLimit,
    ) != 0
    {
        avifDiagnosticsPrintf(
            diag,
            b"Grid box dimensions are too large: [%u x %u]\0" as *const u8
                as *const core::ffi::c_char,
            (*grid).outputWidth,
            (*grid).outputHeight,
        );
        return AVIF_RESULT_NOT_IMPLEMENTED;
    }
    if avifROStreamRemainingBytes(&mut s) != 0 as size_t {
        return AVIF_RESULT_BMFF_PARSE_FAILED;
    }
    return AVIF_RESULT_OK;
}
unsafe extern "C" fn avifParseGainMapMetadata(
    mut gainMap: *mut avifGainMap,
    mut s: *mut avifROStream,
) -> avifBool {
    let mut isMultichannel: uint32_t = 0;
    if avifROStreamReadBitsU32(s, &mut isMultichannel, 1 as size_t) == 0 {
        return AVIF_FALSE;
    }
    let channelCount: uint8_t = (if isMultichannel != 0 {
        3 as core::ffi::c_int
    } else {
        1 as core::ffi::c_int
    }) as uint8_t;
    let mut useBaseColorSpace: uint32_t = 0;
    if avifROStreamReadBitsU32(s, &mut useBaseColorSpace, 1 as size_t) == 0 {
        return AVIF_FALSE;
    }
    (*gainMap).useBaseColorSpace = (if useBaseColorSpace != 0 {
        AVIF_TRUE
    } else {
        AVIF_FALSE
    }) as avifBool;
    let mut reserved: uint32_t = 0;
    if avifROStreamReadBitsU32(s, &mut reserved, 6 as size_t) == 0 {
        return AVIF_FALSE;
    }
    if avifROStreamReadU32(s, &mut (*gainMap).baseHdrHeadroom.n) == 0 {
        return AVIF_FALSE;
    }
    if avifROStreamReadU32(s, &mut (*gainMap).baseHdrHeadroom.d) == 0 {
        return AVIF_FALSE;
    }
    if avifROStreamReadU32(s, &mut (*gainMap).alternateHdrHeadroom.n) == 0 {
        return AVIF_FALSE;
    }
    if avifROStreamReadU32(s, &mut (*gainMap).alternateHdrHeadroom.d) == 0 {
        return AVIF_FALSE;
    }
    let mut c: core::ffi::c_int = 0 as core::ffi::c_int;
    while c < channelCount as core::ffi::c_int {
        if avifROStreamReadU32(
            s,
            &mut (*((*gainMap).gainMapMin).as_mut_ptr().offset(c as isize)).n
                as *mut int32_t as *mut uint32_t,
        ) == 0
        {
            return AVIF_FALSE;
        }
        if avifROStreamReadU32(
            s,
            &mut (*((*gainMap).gainMapMin).as_mut_ptr().offset(c as isize)).d,
        ) == 0
        {
            return AVIF_FALSE;
        }
        if avifROStreamReadU32(
            s,
            &mut (*((*gainMap).gainMapMax).as_mut_ptr().offset(c as isize)).n
                as *mut int32_t as *mut uint32_t,
        ) == 0
        {
            return AVIF_FALSE;
        }
        if avifROStreamReadU32(
            s,
            &mut (*((*gainMap).gainMapMax).as_mut_ptr().offset(c as isize)).d,
        ) == 0
        {
            return AVIF_FALSE;
        }
        if avifROStreamReadU32(
            s,
            &mut (*((*gainMap).gainMapGamma).as_mut_ptr().offset(c as isize)).n,
        ) == 0
        {
            return AVIF_FALSE;
        }
        if avifROStreamReadU32(
            s,
            &mut (*((*gainMap).gainMapGamma).as_mut_ptr().offset(c as isize)).d,
        ) == 0
        {
            return AVIF_FALSE;
        }
        if avifROStreamReadU32(
            s,
            &mut (*((*gainMap).baseOffset).as_mut_ptr().offset(c as isize)).n
                as *mut int32_t as *mut uint32_t,
        ) == 0
        {
            return AVIF_FALSE;
        }
        if avifROStreamReadU32(
            s,
            &mut (*((*gainMap).baseOffset).as_mut_ptr().offset(c as isize)).d,
        ) == 0
        {
            return AVIF_FALSE;
        }
        if avifROStreamReadU32(
            s,
            &mut (*((*gainMap).alternateOffset).as_mut_ptr().offset(c as isize)).n
                as *mut int32_t as *mut uint32_t,
        ) == 0
        {
            return AVIF_FALSE;
        }
        if avifROStreamReadU32(
            s,
            &mut (*((*gainMap).alternateOffset).as_mut_ptr().offset(c as isize)).d,
        ) == 0
        {
            return AVIF_FALSE;
        }
        c += 1;
    }
    let mut c_0: core::ffi::c_int = channelCount as core::ffi::c_int;
    while c_0 < 3 as core::ffi::c_int {
        (*gainMap).gainMapMin[c_0 as usize] = (*gainMap)
            .gainMapMin[0 as core::ffi::c_int as usize];
        (*gainMap).gainMapMax[c_0 as usize] = (*gainMap)
            .gainMapMax[0 as core::ffi::c_int as usize];
        (*gainMap).gainMapGamma[c_0 as usize] = (*gainMap)
            .gainMapGamma[0 as core::ffi::c_int as usize];
        (*gainMap).baseOffset[c_0 as usize] = (*gainMap)
            .baseOffset[0 as core::ffi::c_int as usize];
        (*gainMap).alternateOffset[c_0 as usize] = (*gainMap)
            .alternateOffset[0 as core::ffi::c_int as usize];
        c_0 += 1;
    }
    return AVIF_TRUE;
}
unsafe extern "C" fn avifParseToneMappedImageBox(
    mut gainMap: *mut avifGainMap,
    mut raw: *const uint8_t,
    mut rawLen: size_t,
    mut diag: *mut avifDiagnostics,
) -> avifResult {
    let mut s: avifROStream = avifROStream {
        raw: 0 as *mut avifROData,
        offset: 0,
        numUsedBitsInPartialByte: 0,
        diag: 0 as *mut avifDiagnostics,
        diagContext: 0 as *const core::ffi::c_char,
    };
    let mut s_roData: avifROData = avifROData {
        data: 0 as *const uint8_t,
        size: 0,
    };
    s_roData.data = raw;
    s_roData.size = rawLen;
    avifROStreamStart(
        &mut s,
        &mut s_roData,
        diag,
        b"Box[tmap]\0" as *const u8 as *const core::ffi::c_char,
    );
    let mut version: uint8_t = 0;
    if avifROStreamRead(&mut s, &mut version, 1 as size_t) == 0 {
        return AVIF_RESULT_INVALID_TONE_MAPPED_IMAGE;
    }
    if version as core::ffi::c_int != 0 as core::ffi::c_int {
        avifDiagnosticsPrintf(
            diag,
            b"Box[tmap] has unsupported version [%u]\0" as *const u8
                as *const core::ffi::c_char,
            version as core::ffi::c_int,
        );
        return AVIF_RESULT_NOT_IMPLEMENTED;
    }
    let mut minimumVersion: uint16_t = 0;
    if avifROStreamReadU16(&mut s, &mut minimumVersion) == 0 {
        return AVIF_RESULT_INVALID_TONE_MAPPED_IMAGE;
    }
    let supportedMetadataVersion: uint16_t = 0 as uint16_t;
    if minimumVersion as core::ffi::c_int > supportedMetadataVersion as core::ffi::c_int
    {
        avifDiagnosticsPrintf(
            diag,
            b"Box[tmap] has unsupported minimum version [%u]\0" as *const u8
                as *const core::ffi::c_char,
            minimumVersion as core::ffi::c_int,
        );
        return AVIF_RESULT_NOT_IMPLEMENTED;
    }
    let mut writerVersion: uint16_t = 0;
    if avifROStreamReadU16(&mut s, &mut writerVersion) == 0 {
        return AVIF_RESULT_INVALID_TONE_MAPPED_IMAGE;
    }
    if !(writerVersion as core::ffi::c_int >= minimumVersion as core::ffi::c_int) {
        return AVIF_RESULT_INVALID_TONE_MAPPED_IMAGE;
    }
    if avifParseGainMapMetadata(gainMap, &mut s) == 0 {
        return AVIF_RESULT_INVALID_TONE_MAPPED_IMAGE;
    }
    if writerVersion as core::ffi::c_int <= supportedMetadataVersion as core::ffi::c_int
    {
        if !(avifROStreamRemainingBytes(&mut s) == 0 as size_t) {
            return AVIF_RESULT_INVALID_TONE_MAPPED_IMAGE;
        }
    }
    if avifGainMapValidateMetadata(gainMap, diag) as core::ffi::c_uint
        != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
    {
        return AVIF_RESULT_INVALID_TONE_MAPPED_IMAGE;
    }
    return AVIF_RESULT_OK;
}
unsafe extern "C" fn avifParseSampleTransformTokens(
    mut s: *mut avifROStream,
    mut expression: *mut avifSampleTransformExpression,
) -> avifResult {
    let mut tokenCount: uint8_t = 0;
    if avifROStreamRead(s, &mut tokenCount, 1 as size_t) == 0 {
        return AVIF_RESULT_BMFF_PARSE_FAILED;
    }
    if !(tokenCount as core::ffi::c_int != 0 as core::ffi::c_int) {
        return AVIF_RESULT_BMFF_PARSE_FAILED;
    }
    if avifArrayCreate(
        expression as *mut core::ffi::c_void,
        ::core::mem::size_of::<avifSampleTransformToken>() as uint32_t,
        tokenCount as uint32_t,
    ) == 0
    {
        return AVIF_RESULT_OUT_OF_MEMORY;
    }
    let mut t: uint32_t = 0 as uint32_t;
    while t < tokenCount as uint32_t {
        let mut token: *mut avifSampleTransformToken = avifArrayPush(
            expression as *mut core::ffi::c_void,
        ) as *mut avifSampleTransformToken;
        if token.is_null() {
            return AVIF_RESULT_OUT_OF_MEMORY;
        }
        let mut tokenValue: uint8_t = 0;
        if avifROStreamRead(s, &mut tokenValue, 1 as size_t) == 0 {
            return AVIF_RESULT_BMFF_PARSE_FAILED;
        }
        if tokenValue as core::ffi::c_int
            == AVIF_SAMPLE_TRANSFORM_CONSTANT as core::ffi::c_int
        {
            (*token).type_0 = AVIF_SAMPLE_TRANSFORM_CONSTANT;
            let mut constant: uint32_t = 0;
            if avifROStreamReadU32(s, &mut constant) == 0 {
                return AVIF_RESULT_BMFF_PARSE_FAILED;
            }
            (*token).constant = constant as int32_t;
        } else if tokenValue as core::ffi::c_int
            <= AVIF_SAMPLE_TRANSFORM_LAST_INPUT_IMAGE_ITEM_INDEX as core::ffi::c_int
        {
            'c_12658: {
                if tokenValue as core::ffi::c_int
                    >= AVIF_SAMPLE_TRANSFORM_FIRST_INPUT_IMAGE_ITEM_INDEX
                        as core::ffi::c_int
                {} else {
                    __assert_fail(
                        b"tokenValue >= AVIF_SAMPLE_TRANSFORM_FIRST_INPUT_IMAGE_ITEM_INDEX\0"
                            as *const u8 as *const core::ffi::c_char,
                        b"/home/runner/work/libavif-rs/libavif-rs/upstream/libavif/src/read.c\0"
                            as *const u8 as *const core::ffi::c_char,
                        2357 as core::ffi::c_uint,
                        (::core::mem::transmute::<
                            [u8; 91],
                            [core::ffi::c_char; 91],
                        >(
                            *b"avifResult avifParseSampleTransformTokens(avifROStream *, avifSampleTransformExpression *)\0",
                        ))
                            .as_ptr(),
                    );
                }
            };
            (*token).type_0 = AVIF_SAMPLE_TRANSFORM_INPUT_IMAGE_ITEM_INDEX;
            (*token).inputImageItemIndex = tokenValue;
        } else if tokenValue as core::ffi::c_int
            >= AVIF_SAMPLE_TRANSFORM_FIRST_UNARY_OPERATOR as core::ffi::c_int
            && tokenValue as core::ffi::c_int
                <= AVIF_SAMPLE_TRANSFORM_LAST_UNARY_OPERATOR as core::ffi::c_int
        {
            (*token).type_0 = tokenValue as avifSampleTransformTokenType;
        } else if tokenValue as core::ffi::c_int
            >= AVIF_SAMPLE_TRANSFORM_FIRST_BINARY_OPERATOR as core::ffi::c_int
            && tokenValue as core::ffi::c_int
                <= AVIF_SAMPLE_TRANSFORM_LAST_BINARY_OPERATOR as core::ffi::c_int
        {
            (*token).type_0 = tokenValue as avifSampleTransformTokenType;
        } else {
            (*token).type_0 = AVIF_SAMPLE_TRANSFORM_RESERVED;
        }
        t = t.wrapping_add(1);
    }
    if !(avifROStreamRemainingBytes(s) == 0 as size_t) {
        return AVIF_RESULT_BMFF_PARSE_FAILED;
    }
    return AVIF_RESULT_OK;
}
unsafe extern "C" fn avifParseSampleTransformImageBox(
    mut raw: *const uint8_t,
    mut rawLen: size_t,
    mut numInputImageItems: uint32_t,
    mut expression: *mut avifSampleTransformExpression,
    mut diag: *mut avifDiagnostics,
) -> avifResult {
    let mut s: avifROStream = avifROStream {
        raw: 0 as *mut avifROData,
        offset: 0,
        numUsedBitsInPartialByte: 0,
        diag: 0 as *mut avifDiagnostics,
        diagContext: 0 as *const core::ffi::c_char,
    };
    let mut s_roData: avifROData = avifROData {
        data: 0 as *const uint8_t,
        size: 0,
    };
    s_roData.data = raw;
    s_roData.size = rawLen;
    avifROStreamStart(
        &mut s,
        &mut s_roData,
        diag,
        b"Box[sato]\0" as *const u8 as *const core::ffi::c_char,
    );
    let mut version: uint8_t = 0;
    let mut reserved: uint8_t = 0;
    let mut bitDepth: uint8_t = 0;
    if avifROStreamReadBitsU8(&mut s, &mut version, 2 as size_t) == 0 {
        return AVIF_RESULT_BMFF_PARSE_FAILED;
    }
    if avifROStreamReadBitsU8(&mut s, &mut reserved, 4 as size_t) == 0 {
        return AVIF_RESULT_BMFF_PARSE_FAILED;
    }
    if avifROStreamReadBitsU8(&mut s, &mut bitDepth, 2 as size_t) == 0 {
        return AVIF_RESULT_BMFF_PARSE_FAILED;
    }
    if !(version as core::ffi::c_int == 0 as core::ffi::c_int) {
        return AVIF_RESULT_NOT_IMPLEMENTED;
    }
    if !(bitDepth as core::ffi::c_int
        == AVIF_SAMPLE_TRANSFORM_BIT_DEPTH_32 as core::ffi::c_int)
    {
        return AVIF_RESULT_NOT_IMPLEMENTED;
    }
    let result: avifResult = avifParseSampleTransformTokens(&mut s, expression)
        as avifResult;
    if result as core::ffi::c_uint
        != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
    {
        avifArrayDestroy(expression as *mut core::ffi::c_void);
        return result;
    }
    if avifSampleTransformExpressionIsValid(expression, numInputImageItems) == 0 {
        avifArrayDestroy(expression as *mut core::ffi::c_void);
        return AVIF_RESULT_BMFF_PARSE_FAILED;
    }
    return AVIF_RESULT_OK;
}
unsafe extern "C" fn avifDecoderItemCodecConfigOrFirstCellCodecConfig(
    mut item: *const avifDecoderItem,
) -> *const avifProperty {
    if memcmp(
        ((*item).type_0).as_ptr() as *const core::ffi::c_void,
        b"grid\0" as *const u8 as *const core::ffi::c_char as *const core::ffi::c_void,
        4 as size_t,
    ) == 0
    {
        let mut i: uint32_t = 0 as uint32_t;
        while i < (*(*item).meta).items.count {
            let mut inputImageItem: *mut avifDecoderItem = *((*(*item).meta).items.item)
                .offset(i as isize);
            if avifIsDimgInput(item, (*inputImageItem).id, 0 as *mut uint32_t) != 0 {
                return avifPropertyArrayFind(
                    &mut (*inputImageItem).properties,
                    avifGetConfigurationPropertyName(
                        avifGetCodecType(((*inputImageItem).type_0).as_mut_ptr()),
                    ),
                );
            }
            i = i.wrapping_add(1);
        }
        'c_11712: {
            __assert_fail(
                b"AVIF_FALSE\0" as *const u8 as *const core::ffi::c_char,
                b"/home/runner/work/libavif-rs/libavif-rs/upstream/libavif/src/read.c\0"
                    as *const u8 as *const core::ffi::c_char,
                2413 as core::ffi::c_uint,
                (::core::mem::transmute::<
                    [u8; 94],
                    [core::ffi::c_char; 94],
                >(
                    *b"const avifProperty *avifDecoderItemCodecConfigOrFirstCellCodecConfig(const avifDecoderItem *)\0",
                ))
                    .as_ptr(),
            );
        };
    }
    return avifPropertyArrayFind(
        &(*item).properties,
        avifGetConfigurationPropertyName(avifGetCodecType(((*item).type_0).as_ptr())),
    );
}
unsafe extern "C" fn avifDecoderSampleTransformItemValidateProperties(
    mut satoItem: *const avifDecoderItem,
    mut diag: *mut avifDiagnostics,
) -> avifResult {
    'c_12366: {
        if memcmp(
            ((*satoItem).type_0).as_ptr() as *const core::ffi::c_void,
            b"sato\0" as *const u8 as *const core::ffi::c_char
                as *const core::ffi::c_void,
            4 as size_t,
        ) == 0 as core::ffi::c_int
        {} else {
            __assert_fail(
                b"memcmp(satoItem->type, \"sato\", 4) == 0\0" as *const u8
                    as *const core::ffi::c_char,
                b"/home/runner/work/libavif-rs/libavif-rs/upstream/libavif/src/read.c\0"
                    as *const u8 as *const core::ffi::c_char,
                2420 as core::ffi::c_uint,
                (::core::mem::transmute::<
                    [u8; 104],
                    [core::ffi::c_char; 104],
                >(
                    *b"avifResult avifDecoderSampleTransformItemValidateProperties(const avifDecoderItem *, avifDiagnostics *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    let mut pixiProp: *const avifProperty = avifPropertyArrayFind(
        &(*satoItem).properties,
        b"pixi\0" as *const u8 as *const core::ffi::c_char,
    );
    if pixiProp.is_null() {
        avifDiagnosticsPrintf(
            diag,
            b"Item ID %u of type 'sato' is missing mandatory pixi property\0"
                as *const u8 as *const core::ffi::c_char,
            (*satoItem).id,
        );
        return AVIF_RESULT_BMFF_PARSE_FAILED;
    }
    let mut i: uint8_t = 1 as uint8_t;
    while (i as core::ffi::c_int) < (*pixiProp).u.pixi.planeCount as core::ffi::c_int {
        'c_12261: {
            if (*pixiProp).u.pixi.planeDepths[i as usize] as core::ffi::c_int
                == (*pixiProp).u.pixi.planeDepths[0 as core::ffi::c_int as usize]
                    as core::ffi::c_int
            {} else {
                __assert_fail(
                    b"pixiProp->u.pixi.planeDepths[i] == pixiProp->u.pixi.planeDepths[0]\0"
                        as *const u8 as *const core::ffi::c_char,
                    b"/home/runner/work/libavif-rs/libavif-rs/upstream/libavif/src/read.c\0"
                        as *const u8 as *const core::ffi::c_char,
                    2428 as core::ffi::c_uint,
                    (::core::mem::transmute::<
                        [u8; 104],
                        [core::ffi::c_char; 104],
                    >(
                        *b"avifResult avifDecoderSampleTransformItemValidateProperties(const avifDecoderItem *, avifDiagnostics *)\0",
                    ))
                        .as_ptr(),
                );
            }
        };
        i = i.wrapping_add(1);
    }
    'c_12209: {
        if (*pixiProp).u.pixi.planeCount as core::ffi::c_int >= 1 as core::ffi::c_int
        {} else {
            __assert_fail(
                b"pixiProp->u.pixi.planeCount >= 1\0" as *const u8
                    as *const core::ffi::c_char,
                b"/home/runner/work/libavif-rs/libavif-rs/upstream/libavif/src/read.c\0"
                    as *const u8 as *const core::ffi::c_char,
                2430 as core::ffi::c_uint,
                (::core::mem::transmute::<
                    [u8; 104],
                    [core::ffi::c_char; 104],
                >(
                    *b"avifResult avifDecoderSampleTransformItemValidateProperties(const avifDecoderItem *, avifDiagnostics *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    let depth: uint8_t = (*pixiProp).u.pixi.planeDepths[0 as core::ffi::c_int as usize];
    if depth as core::ffi::c_int != 8 as core::ffi::c_int
        && depth as core::ffi::c_int != 10 as core::ffi::c_int
        && depth as core::ffi::c_int != 12 as core::ffi::c_int
        && depth as core::ffi::c_int != 16 as core::ffi::c_int
    {
        avifDiagnosticsPrintf(
            diag,
            b"Item ID %u of type 'sato' with depth %u (specified by pixi property) is not supported\0"
                as *const u8 as *const core::ffi::c_char,
            (*satoItem).id,
            depth as core::ffi::c_int,
        );
        return AVIF_RESULT_NOT_IMPLEMENTED;
    }
    let mut ispeProp: *const avifProperty = avifPropertyArrayFind(
        &(*satoItem).properties,
        b"ispe\0" as *const u8 as *const core::ffi::c_char,
    );
    if ispeProp.is_null() {
        avifDiagnosticsPrintf(
            diag,
            b"Item ID %u of type 'sato' is missing mandatory ispe property\0"
                as *const u8 as *const core::ffi::c_char,
            (*satoItem).id,
        );
        return AVIF_RESULT_BMFF_PARSE_FAILED;
    }
    let mut i_0: uint32_t = 0 as uint32_t;
    while i_0 < (*(*satoItem).meta).items.count {
        let mut inputImageItem: *mut avifDecoderItem = *((*(*satoItem).meta).items.item)
            .offset(i_0 as isize);
        if avifIsDimgInput(satoItem, (*inputImageItem).id, 0 as *mut uint32_t) == 0 {
            i_0 = i_0.wrapping_add(1);
        } else {
            let mut inputImageItemIspeProp: *const avifProperty = avifPropertyArrayFind(
                &mut (*inputImageItem).properties,
                b"ispe\0" as *const u8 as *const core::ffi::c_char,
            );
            if inputImageItemIspeProp.is_null() {
                avifDiagnosticsPrintf(
                    diag,
                    b"Item ID %u is missing mandatory ispe property\0" as *const u8
                        as *const core::ffi::c_char,
                    (*inputImageItem).id,
                );
                return AVIF_RESULT_BMFF_PARSE_FAILED;
            }
            let mut inputImageItemCodecConfig: *const avifProperty = avifDecoderItemCodecConfigOrFirstCellCodecConfig(
                inputImageItem,
            );
            if inputImageItemCodecConfig.is_null() {
                avifDiagnosticsPrintf(
                    diag,
                    b"Item ID %u of type '%.4s' is missing mandatory codec configuration property\0"
                        as *const u8 as *const core::ffi::c_char,
                    (*inputImageItem).id,
                    ((*inputImageItem).type_0).as_mut_ptr() as *const core::ffi::c_char,
                );
                return AVIF_RESULT_BMFF_PARSE_FAILED;
            }
            let mut j: uint32_t = i_0.wrapping_add(1 as uint32_t);
            while j < (*(*satoItem).meta).items.count {
                let mut otherInputImageItem: *mut avifDecoderItem = *((*(*satoItem).meta)
                    .items
                    .item)
                    .offset(j as isize);
                if !(avifIsDimgInput(
                    satoItem,
                    (*otherInputImageItem).id,
                    0 as *mut uint32_t,
                ) == 0)
                {
                    let mut otherInputImageItemIspeProp: *const avifProperty = avifPropertyArrayFind(
                        &mut (*otherInputImageItem).properties,
                        b"ispe\0" as *const u8 as *const core::ffi::c_char,
                    );
                    if otherInputImageItemIspeProp.is_null() {
                        avifDiagnosticsPrintf(
                            diag,
                            b"Item ID %u of type '%.4s' is missing mandatory ispe property\0"
                                as *const u8 as *const core::ffi::c_char,
                            (*otherInputImageItem).id,
                            ((*otherInputImageItem).type_0).as_mut_ptr()
                                as *const core::ffi::c_char,
                        );
                        return AVIF_RESULT_BMFF_PARSE_FAILED;
                    }
                    if (*inputImageItemIspeProp).u.ispe.width
                        != (*otherInputImageItemIspeProp).u.ispe.width
                        || (*inputImageItemIspeProp).u.ispe.height
                            != (*otherInputImageItemIspeProp).u.ispe.height
                    {
                        avifDiagnosticsPrintf(
                            diag,
                            b"The fields of the ispe property of item ID %u of type '%.4s' differs from item ID %u\0"
                                as *const u8 as *const core::ffi::c_char,
                            (*inputImageItem).id,
                            ((*inputImageItem).type_0).as_mut_ptr()
                                as *const core::ffi::c_char,
                            (*otherInputImageItem).id,
                        );
                        return AVIF_RESULT_BMFF_PARSE_FAILED;
                    }
                    let mut otherInputImageItemCodecConfig: *const avifProperty = avifDecoderItemCodecConfigOrFirstCellCodecConfig(
                        otherInputImageItem,
                    );
                    if otherInputImageItemCodecConfig.is_null() {
                        avifDiagnosticsPrintf(
                            diag,
                            b"Item ID %u of type '%.4s' is missing mandatory codec configuration property\0"
                                as *const u8 as *const core::ffi::c_char,
                            (*otherInputImageItem).id,
                            ((*otherInputImageItem).type_0).as_mut_ptr()
                                as *const core::ffi::c_char,
                        );
                        return AVIF_RESULT_BMFF_PARSE_FAILED;
                    }
                    if (*inputImageItemCodecConfig).u.av1C.monochrome as core::ffi::c_int
                        != (*otherInputImageItemCodecConfig).u.av1C.monochrome
                            as core::ffi::c_int
                        || (*inputImageItemCodecConfig).u.av1C.chromaSubsamplingX
                            as core::ffi::c_int
                            != (*otherInputImageItemCodecConfig)
                                .u
                                .av1C
                                .chromaSubsamplingX as core::ffi::c_int
                        || (*inputImageItemCodecConfig).u.av1C.chromaSubsamplingY
                            as core::ffi::c_int
                            != (*otherInputImageItemCodecConfig)
                                .u
                                .av1C
                                .chromaSubsamplingY as core::ffi::c_int
                        || (*inputImageItemCodecConfig).u.av1C.chromaSamplePosition
                            as core::ffi::c_int
                            != (*otherInputImageItemCodecConfig)
                                .u
                                .av1C
                                .chromaSamplePosition as core::ffi::c_int
                    {
                        avifDiagnosticsPrintf(
                            diag,
                            b"The plane count or subsampling in the codec configuration property of item ID %u of type '%.4s' differs from item ID %u\0"
                                as *const u8 as *const core::ffi::c_char,
                            (*inputImageItem).id,
                            ((*inputImageItem).type_0).as_mut_ptr()
                                as *const core::ffi::c_char,
                            (*otherInputImageItem).id,
                        );
                        return AVIF_RESULT_BMFF_PARSE_FAILED;
                    }
                }
                j = j.wrapping_add(1);
            }
            break;
        }
    }
    if !(avifPropertyArrayFind(
        &(*satoItem).properties,
        b"clap\0" as *const u8 as *const core::ffi::c_char,
    ))
        .is_null()
    {
        return AVIF_RESULT_NOT_IMPLEMENTED;
    }
    return AVIF_RESULT_OK;
}
unsafe extern "C" fn avifDecoderItemReadAndParse(
    mut decoder: *const avifDecoder,
    mut item: *mut avifDecoderItem,
    mut isItemInInput: avifBool,
    mut grid: *mut avifImageGrid,
    mut codecType: *mut avifCodecType,
) -> avifResult {
    if memcmp(
        ((*item).type_0).as_mut_ptr() as *const core::ffi::c_void,
        b"grid\0" as *const u8 as *const core::ffi::c_char as *const core::ffi::c_void,
        4 as size_t,
    ) == 0
    {
        if isItemInInput != 0 {
            let mut readData: avifROData = avifROData {
                data: 0 as *const uint8_t,
                size: 0,
            };
            let result__: avifResult = avifDecoderItemRead(
                item,
                (*decoder).io,
                &mut readData,
                0 as size_t,
                0 as size_t,
                (*(*decoder).data).diag,
            ) as avifResult;
            if result__ as core::ffi::c_uint
                != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
            {
                return result__;
            }
            let result___0: avifResult = avifParseImageGridBox(
                grid,
                readData.data,
                readData.size,
                (*decoder).imageSizeLimit,
                (*decoder).imageDimensionLimit,
                (*(*decoder).data).diag,
            ) as avifResult;
            if result___0 as core::ffi::c_uint
                != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
            {
                return result___0;
            }
            let mut dimgItemCount: uint32_t = 0 as uint32_t;
            let mut i: uint32_t = 0 as uint32_t;
            while i < (*(*item).meta).items.count {
                if avifIsDimgInput(
                    item,
                    (**((*(*item).meta).items.item).offset(i as isize)).id,
                    0 as *mut uint32_t,
                ) != 0
                {
                    dimgItemCount = dimgItemCount.wrapping_add(1);
                }
                i = i.wrapping_add(1);
            }
            if !(dimgItemCount == ((*grid).rows).wrapping_mul((*grid).columns)) {
                return AVIF_RESULT_INVALID_IMAGE_GRID;
            }
        } else {
            'c_13349: {
                if (*grid).rows > 0 as uint32_t && (*grid).columns > 0 as uint32_t
                {} else {
                    __assert_fail(
                        b"grid->rows > 0 && grid->columns > 0\0" as *const u8
                            as *const core::ffi::c_char,
                        b"/home/runner/work/libavif-rs/libavif-rs/upstream/libavif/src/read.c\0"
                            as *const u8 as *const core::ffi::c_char,
                        2559 as core::ffi::c_uint,
                        (::core::mem::transmute::<
                            [u8; 123],
                            [core::ffi::c_char; 123],
                        >(
                            *b"avifResult avifDecoderItemReadAndParse(const avifDecoder *, avifDecoderItem *, avifBool, avifImageGrid *, avifCodecType *)\0",
                        ))
                            .as_ptr(),
                    );
                }
            };
        }
        *codecType = avifDecoderItemGetGridCodecType(item);
        if !(*codecType as core::ffi::c_uint
            != AVIF_CODEC_TYPE_UNKNOWN as core::ffi::c_int as core::ffi::c_uint)
        {
            return AVIF_RESULT_INVALID_IMAGE_GRID;
        }
    } else {
        *codecType = avifGetCodecType(((*item).type_0).as_mut_ptr());
        'c_13181: {
            if *codecType as core::ffi::c_uint
                != AVIF_CODEC_TYPE_UNKNOWN as core::ffi::c_int as core::ffi::c_uint
            {} else {
                __assert_fail(
                    b"*codecType != AVIF_CODEC_TYPE_UNKNOWN\0" as *const u8
                        as *const core::ffi::c_char,
                    b"/home/runner/work/libavif-rs/libavif-rs/upstream/libavif/src/read.c\0"
                        as *const u8 as *const core::ffi::c_char,
                    2565 as core::ffi::c_uint,
                    (::core::mem::transmute::<
                        [u8; 123],
                        [core::ffi::c_char; 123],
                    >(
                        *b"avifResult avifDecoderItemReadAndParse(const avifDecoder *, avifDecoderItem *, avifBool, avifImageGrid *, avifCodecType *)\0",
                    ))
                        .as_ptr(),
                );
            }
        };
    }
    return AVIF_RESULT_OK;
}
unsafe extern "C" fn avifParseImageSpatialExtentsProperty(
    mut prop: *mut avifProperty,
    mut raw: *const uint8_t,
    mut rawLen: size_t,
    mut diag: *mut avifDiagnostics,
) -> avifBool {
    let mut s: avifROStream = avifROStream {
        raw: 0 as *mut avifROData,
        offset: 0,
        numUsedBitsInPartialByte: 0,
        diag: 0 as *mut avifDiagnostics,
        diagContext: 0 as *const core::ffi::c_char,
    };
    let mut s_roData: avifROData = avifROData {
        data: 0 as *const uint8_t,
        size: 0,
    };
    s_roData.data = raw;
    s_roData.size = rawLen;
    avifROStreamStart(
        &mut s,
        &mut s_roData,
        diag,
        b"Box[ispe]\0" as *const u8 as *const core::ffi::c_char,
    );
    if avifROStreamReadAndEnforceVersion(&mut s, 0 as uint8_t, 0 as *mut uint32_t) == 0 {
        return AVIF_FALSE;
    }
    let mut ispe: *mut avifImageSpatialExtents = &mut (*prop).u.ispe;
    if avifROStreamReadU32(&mut s, &mut (*ispe).width) == 0 {
        return AVIF_FALSE;
    }
    if avifROStreamReadU32(&mut s, &mut (*ispe).height) == 0 {
        return AVIF_FALSE;
    }
    return AVIF_TRUE;
}
unsafe extern "C" fn avifParseAuxiliaryTypeProperty(
    mut prop: *mut avifProperty,
    mut raw: *const uint8_t,
    mut rawLen: size_t,
    mut diag: *mut avifDiagnostics,
) -> avifBool {
    let mut s: avifROStream = avifROStream {
        raw: 0 as *mut avifROData,
        offset: 0,
        numUsedBitsInPartialByte: 0,
        diag: 0 as *mut avifDiagnostics,
        diagContext: 0 as *const core::ffi::c_char,
    };
    let mut s_roData: avifROData = avifROData {
        data: 0 as *const uint8_t,
        size: 0,
    };
    s_roData.data = raw;
    s_roData.size = rawLen;
    avifROStreamStart(
        &mut s,
        &mut s_roData,
        diag,
        b"Box[auxC]\0" as *const u8 as *const core::ffi::c_char,
    );
    if avifROStreamReadAndEnforceVersion(&mut s, 0 as uint8_t, 0 as *mut uint32_t) == 0 {
        return AVIF_FALSE;
    }
    if avifROStreamReadString(
        &mut s,
        ((*prop).u.auxC.auxType).as_mut_ptr(),
        64 as size_t,
    ) == 0
    {
        return AVIF_FALSE;
    }
    return AVIF_TRUE;
}
unsafe extern "C" fn avifParseColourInformationBox(
    mut prop: *mut avifProperty,
    mut rawOffset: uint64_t,
    mut raw: *const uint8_t,
    mut rawLen: size_t,
    mut diag: *mut avifDiagnostics,
) -> avifBool {
    let mut s: avifROStream = avifROStream {
        raw: 0 as *mut avifROData,
        offset: 0,
        numUsedBitsInPartialByte: 0,
        diag: 0 as *mut avifDiagnostics,
        diagContext: 0 as *const core::ffi::c_char,
    };
    let mut s_roData: avifROData = avifROData {
        data: 0 as *const uint8_t,
        size: 0,
    };
    s_roData.data = raw;
    s_roData.size = rawLen;
    avifROStreamStart(
        &mut s,
        &mut s_roData,
        diag,
        b"Box[colr]\0" as *const u8 as *const core::ffi::c_char,
    );
    let mut colr: *mut avifColourInformationBox = &mut (*prop).u.colr;
    (*colr).hasICC = AVIF_FALSE as avifBool;
    (*colr).hasNCLX = AVIF_FALSE as avifBool;
    let mut colorType: [uint8_t; 4] = [0; 4];
    if avifROStreamRead(&mut s, colorType.as_mut_ptr(), 4 as size_t) == 0 {
        return AVIF_FALSE;
    }
    if memcmp(
        colorType.as_mut_ptr() as *const core::ffi::c_void,
        b"rICC\0" as *const u8 as *const core::ffi::c_char as *const core::ffi::c_void,
        4 as size_t,
    ) == 0
        || memcmp(
            colorType.as_mut_ptr() as *const core::ffi::c_void,
            b"prof\0" as *const u8 as *const core::ffi::c_char
                as *const core::ffi::c_void,
            4 as size_t,
        ) == 0
    {
        (*colr).iccOffset = rawOffset
            .wrapping_add(avifROStreamOffset(&mut s) as uint64_t);
        (*colr).iccSize = avifROStreamRemainingBytes(&mut s);
        if (*colr).iccSize == 0 as size_t {
            avifDiagnosticsPrintf(
                diag,
                b"Box[colr] contains empty ICC_profile\0" as *const u8
                    as *const core::ffi::c_char,
            );
            return AVIF_FALSE;
        }
        (*colr).hasICC = AVIF_TRUE as avifBool;
    } else if memcmp(
        colorType.as_mut_ptr() as *const core::ffi::c_void,
        b"nclx\0" as *const u8 as *const core::ffi::c_char as *const core::ffi::c_void,
        4 as size_t,
    ) == 0
    {
        if avifROStreamReadU16(&mut s, &mut (*colr).colorPrimaries) == 0 {
            return AVIF_FALSE;
        }
        if avifROStreamReadU16(&mut s, &mut (*colr).transferCharacteristics) == 0 {
            return AVIF_FALSE;
        }
        if avifROStreamReadU16(&mut s, &mut (*colr).matrixCoefficients) == 0 {
            return AVIF_FALSE;
        }
        let mut full_range_flag: uint8_t = 0;
        if avifROStreamReadBitsU8(&mut s, &mut full_range_flag, 1 as size_t) == 0 {
            return AVIF_FALSE;
        }
        (*colr).range = (if full_range_flag as core::ffi::c_int != 0 {
            AVIF_RANGE_FULL as core::ffi::c_int
        } else {
            AVIF_RANGE_LIMITED as core::ffi::c_int
        }) as avifRange;
        let mut reserved: uint8_t = 0;
        if avifROStreamReadBitsU8(&mut s, &mut reserved, 7 as size_t) == 0 {
            return AVIF_FALSE;
        }
        if reserved != 0 {
            avifDiagnosticsPrintf(
                diag,
                b"Box[colr] contains nonzero reserved bits [%u]\0" as *const u8
                    as *const core::ffi::c_char,
                reserved as core::ffi::c_int,
            );
            return AVIF_FALSE;
        }
        (*colr).hasNCLX = AVIF_TRUE as avifBool;
    }
    return AVIF_TRUE;
}
unsafe extern "C" fn avifParseContentLightLevelInformation(
    mut s: *mut avifROStream,
    mut clli: *mut avifContentLightLevelInformationBox,
) -> avifResult {
    if avifROStreamReadBitsU16(s, &mut (*clli).maxCLL, 16 as size_t) == 0 {
        return AVIF_RESULT_BMFF_PARSE_FAILED;
    }
    if avifROStreamReadBitsU16(s, &mut (*clli).maxPALL, 16 as size_t) == 0 {
        return AVIF_RESULT_BMFF_PARSE_FAILED;
    }
    return AVIF_RESULT_OK;
}
unsafe extern "C" fn avifParseContentLightLevelInformationBox(
    mut prop: *mut avifProperty,
    mut raw: *const uint8_t,
    mut rawLen: size_t,
    mut diag: *mut avifDiagnostics,
) -> avifResult {
    let mut s: avifROStream = avifROStream {
        raw: 0 as *mut avifROData,
        offset: 0,
        numUsedBitsInPartialByte: 0,
        diag: 0 as *mut avifDiagnostics,
        diagContext: 0 as *const core::ffi::c_char,
    };
    let mut s_roData: avifROData = avifROData {
        data: 0 as *const uint8_t,
        size: 0,
    };
    s_roData.data = raw;
    s_roData.size = rawLen;
    avifROStreamStart(
        &mut s,
        &mut s_roData,
        diag,
        b"Box[clli]\0" as *const u8 as *const core::ffi::c_char,
    );
    let result__: avifResult = avifParseContentLightLevelInformation(
        &mut s,
        &mut (*prop).u.clli,
    ) as avifResult;
    if result__ as core::ffi::c_uint
        != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
    {
        return result__;
    }
    return AVIF_RESULT_OK;
}
unsafe extern "C" fn avifParseCodecConfiguration(
    mut s: *mut avifROStream,
    mut config: *mut avifCodecConfigurationBox,
    mut configPropName: *const core::ffi::c_char,
    mut diag: *mut avifDiagnostics,
) -> avifBool {
    let av1COffset: size_t = avifROStreamOffset(s) as size_t;
    let mut marker: uint32_t = 0;
    let mut version: uint32_t = 0;
    if avifROStreamReadBitsU32(s, &mut marker, 1 as size_t) == 0 {
        return AVIF_FALSE;
    }
    if marker == 0 {
        avifDiagnosticsPrintf(
            diag,
            b"%.4s contains illegal marker: [%u]\0" as *const u8
                as *const core::ffi::c_char,
            configPropName,
            marker,
        );
        return AVIF_FALSE;
    }
    if avifROStreamReadBitsU32(s, &mut version, 7 as size_t) == 0 {
        return AVIF_FALSE;
    }
    if version != 1 as uint32_t {
        avifDiagnosticsPrintf(
            diag,
            b"%.4s contains illegal version: [%u]\0" as *const u8
                as *const core::ffi::c_char,
            configPropName,
            version,
        );
        return AVIF_FALSE;
    }
    if avifROStreamReadBitsU8(s, &mut (*config).seqProfile, 3 as size_t) == 0 {
        return AVIF_FALSE;
    }
    if avifROStreamReadBitsU8(s, &mut (*config).seqLevelIdx0, 5 as size_t) == 0 {
        return AVIF_FALSE;
    }
    if avifROStreamReadBitsU8(s, &mut (*config).seqTier0, 1 as size_t) == 0 {
        return AVIF_FALSE;
    }
    if avifROStreamReadBitsU8(s, &mut (*config).highBitdepth, 1 as size_t) == 0 {
        return AVIF_FALSE;
    }
    if avifROStreamReadBitsU8(s, &mut (*config).twelveBit, 1 as size_t) == 0 {
        return AVIF_FALSE;
    }
    if avifROStreamReadBitsU8(s, &mut (*config).monochrome, 1 as size_t) == 0 {
        return AVIF_FALSE;
    }
    if avifROStreamReadBitsU8(s, &mut (*config).chromaSubsamplingX, 1 as size_t) == 0 {
        return AVIF_FALSE;
    }
    if avifROStreamReadBitsU8(s, &mut (*config).chromaSubsamplingY, 1 as size_t) == 0 {
        return AVIF_FALSE;
    }
    if avifROStreamReadBitsU8(s, &mut (*config).chromaSamplePosition, 2 as size_t) == 0 {
        return AVIF_FALSE;
    }
    if avifROStreamSkip(s, 1 as size_t) == 0 {
        return AVIF_FALSE;
    }
    if !((avifROStreamOffset(s)).wrapping_sub(av1COffset) == 4 as size_t) {
        return AVIF_FALSE;
    }
    return AVIF_TRUE;
}
unsafe extern "C" fn avifParseCodecConfigurationBoxProperty(
    mut prop: *mut avifProperty,
    mut raw: *const uint8_t,
    mut rawLen: size_t,
    mut configPropName: *const core::ffi::c_char,
    mut diag: *mut avifDiagnostics,
) -> avifBool {
    let mut diagContext: [core::ffi::c_char; 10] = [0; 10];
    snprintf(
        diagContext.as_mut_ptr(),
        ::core::mem::size_of::<[core::ffi::c_char; 10]>() as size_t,
        b"Box[%.4s]\0" as *const u8 as *const core::ffi::c_char,
        configPropName,
    );
    let mut s: avifROStream = avifROStream {
        raw: 0 as *mut avifROData,
        offset: 0,
        numUsedBitsInPartialByte: 0,
        diag: 0 as *mut avifDiagnostics,
        diagContext: 0 as *const core::ffi::c_char,
    };
    let mut s_roData: avifROData = avifROData {
        data: 0 as *const uint8_t,
        size: 0,
    };
    s_roData.data = raw;
    s_roData.size = rawLen;
    avifROStreamStart(&mut s, &mut s_roData, diag, diagContext.as_mut_ptr());
    return avifParseCodecConfiguration(
        &mut s,
        &mut (*prop).u.av1C,
        configPropName,
        diag,
    );
}
unsafe extern "C" fn avifParsePixelAspectRatioBoxProperty(
    mut prop: *mut avifProperty,
    mut raw: *const uint8_t,
    mut rawLen: size_t,
    mut diag: *mut avifDiagnostics,
) -> avifBool {
    let mut s: avifROStream = avifROStream {
        raw: 0 as *mut avifROData,
        offset: 0,
        numUsedBitsInPartialByte: 0,
        diag: 0 as *mut avifDiagnostics,
        diagContext: 0 as *const core::ffi::c_char,
    };
    let mut s_roData: avifROData = avifROData {
        data: 0 as *const uint8_t,
        size: 0,
    };
    s_roData.data = raw;
    s_roData.size = rawLen;
    avifROStreamStart(
        &mut s,
        &mut s_roData,
        diag,
        b"Box[pasp]\0" as *const u8 as *const core::ffi::c_char,
    );
    let mut pasp: *mut avifPixelAspectRatioBox = &mut (*prop).u.pasp;
    if avifROStreamReadU32(&mut s, &mut (*pasp).hSpacing) == 0 {
        return AVIF_FALSE;
    }
    if avifROStreamReadU32(&mut s, &mut (*pasp).vSpacing) == 0 {
        return AVIF_FALSE;
    }
    return AVIF_TRUE;
}
unsafe extern "C" fn avifParseCleanApertureBoxProperty(
    mut prop: *mut avifProperty,
    mut raw: *const uint8_t,
    mut rawLen: size_t,
    mut diag: *mut avifDiagnostics,
) -> avifBool {
    let mut s: avifROStream = avifROStream {
        raw: 0 as *mut avifROData,
        offset: 0,
        numUsedBitsInPartialByte: 0,
        diag: 0 as *mut avifDiagnostics,
        diagContext: 0 as *const core::ffi::c_char,
    };
    let mut s_roData: avifROData = avifROData {
        data: 0 as *const uint8_t,
        size: 0,
    };
    s_roData.data = raw;
    s_roData.size = rawLen;
    avifROStreamStart(
        &mut s,
        &mut s_roData,
        diag,
        b"Box[clap]\0" as *const u8 as *const core::ffi::c_char,
    );
    let mut clap: *mut avifCleanApertureBox = &mut (*prop).u.clap;
    if avifROStreamReadU32(&mut s, &mut (*clap).widthN) == 0 {
        return AVIF_FALSE;
    }
    if avifROStreamReadU32(&mut s, &mut (*clap).widthD) == 0 {
        return AVIF_FALSE;
    }
    if avifROStreamReadU32(&mut s, &mut (*clap).heightN) == 0 {
        return AVIF_FALSE;
    }
    if avifROStreamReadU32(&mut s, &mut (*clap).heightD) == 0 {
        return AVIF_FALSE;
    }
    if avifROStreamReadU32(&mut s, &mut (*clap).horizOffN) == 0 {
        return AVIF_FALSE;
    }
    if avifROStreamReadU32(&mut s, &mut (*clap).horizOffD) == 0 {
        return AVIF_FALSE;
    }
    if avifROStreamReadU32(&mut s, &mut (*clap).vertOffN) == 0 {
        return AVIF_FALSE;
    }
    if avifROStreamReadU32(&mut s, &mut (*clap).vertOffD) == 0 {
        return AVIF_FALSE;
    }
    return AVIF_TRUE;
}
unsafe extern "C" fn avifParseImageRotationProperty(
    mut prop: *mut avifProperty,
    mut raw: *const uint8_t,
    mut rawLen: size_t,
    mut diag: *mut avifDiagnostics,
) -> avifBool {
    let mut s: avifROStream = avifROStream {
        raw: 0 as *mut avifROData,
        offset: 0,
        numUsedBitsInPartialByte: 0,
        diag: 0 as *mut avifDiagnostics,
        diagContext: 0 as *const core::ffi::c_char,
    };
    let mut s_roData: avifROData = avifROData {
        data: 0 as *const uint8_t,
        size: 0,
    };
    s_roData.data = raw;
    s_roData.size = rawLen;
    avifROStreamStart(
        &mut s,
        &mut s_roData,
        diag,
        b"Box[irot]\0" as *const u8 as *const core::ffi::c_char,
    );
    let mut irot: *mut avifImageRotation = &mut (*prop).u.irot;
    let mut reserved: uint8_t = 0;
    if avifROStreamReadBitsU8(&mut s, &mut reserved, 6 as size_t) == 0 {
        return AVIF_FALSE;
    }
    if reserved != 0 {
        avifDiagnosticsPrintf(
            diag,
            b"Box[irot] contains nonzero reserved bits [%u]\0" as *const u8
                as *const core::ffi::c_char,
            reserved as core::ffi::c_int,
        );
        return AVIF_FALSE;
    }
    if avifROStreamReadBitsU8(&mut s, &mut (*irot).angle, 2 as size_t) == 0 {
        return AVIF_FALSE;
    }
    return AVIF_TRUE;
}
unsafe extern "C" fn avifParseImageMirrorProperty(
    mut prop: *mut avifProperty,
    mut raw: *const uint8_t,
    mut rawLen: size_t,
    mut diag: *mut avifDiagnostics,
) -> avifBool {
    let mut s: avifROStream = avifROStream {
        raw: 0 as *mut avifROData,
        offset: 0,
        numUsedBitsInPartialByte: 0,
        diag: 0 as *mut avifDiagnostics,
        diagContext: 0 as *const core::ffi::c_char,
    };
    let mut s_roData: avifROData = avifROData {
        data: 0 as *const uint8_t,
        size: 0,
    };
    s_roData.data = raw;
    s_roData.size = rawLen;
    avifROStreamStart(
        &mut s,
        &mut s_roData,
        diag,
        b"Box[imir]\0" as *const u8 as *const core::ffi::c_char,
    );
    let mut imir: *mut avifImageMirror = &mut (*prop).u.imir;
    let mut reserved: uint8_t = 0;
    if avifROStreamReadBitsU8(&mut s, &mut reserved, 7 as size_t) == 0 {
        return AVIF_FALSE;
    }
    if reserved != 0 {
        avifDiagnosticsPrintf(
            diag,
            b"Box[imir] contains nonzero reserved bits [%u]\0" as *const u8
                as *const core::ffi::c_char,
            reserved as core::ffi::c_int,
        );
        return AVIF_FALSE;
    }
    if avifROStreamReadBitsU8(&mut s, &mut (*imir).axis, 1 as size_t) == 0 {
        return AVIF_FALSE;
    }
    return AVIF_TRUE;
}
unsafe extern "C" fn avifParsePixelInformationProperty(
    mut prop: *mut avifProperty,
    mut raw: *const uint8_t,
    mut rawLen: size_t,
    mut diag: *mut avifDiagnostics,
) -> avifResult {
    let mut s: avifROStream = avifROStream {
        raw: 0 as *mut avifROData,
        offset: 0,
        numUsedBitsInPartialByte: 0,
        diag: 0 as *mut avifDiagnostics,
        diagContext: 0 as *const core::ffi::c_char,
    };
    let mut s_roData: avifROData = avifROData {
        data: 0 as *const uint8_t,
        size: 0,
    };
    s_roData.data = raw;
    s_roData.size = rawLen;
    avifROStreamStart(
        &mut s,
        &mut s_roData,
        diag,
        b"Box[pixi]\0" as *const u8 as *const core::ffi::c_char,
    );
    let mut flags: uint32_t = 0 as uint32_t;
    if avifROStreamReadAndEnforceVersion(&mut s, 0 as uint8_t, &mut flags) == 0 {
        return AVIF_RESULT_BMFF_PARSE_FAILED;
    }
    let mut pixi: *mut avifPixelInformationProperty = &mut (*prop).u.pixi;
    if avifROStreamRead(&mut s, &mut (*pixi).planeCount, 1 as size_t) == 0 {
        return AVIF_RESULT_BMFF_PARSE_FAILED;
    }
    if ((*pixi).planeCount as core::ffi::c_int) < 1 as core::ffi::c_int
        || (*pixi).planeCount as core::ffi::c_int > MAX_PIXI_PLANE_DEPTHS
    {
        avifDiagnosticsPrintf(
            diag,
            b"Box[pixi] contains unsupported plane count [%u]\0" as *const u8
                as *const core::ffi::c_char,
            (*pixi).planeCount as core::ffi::c_int,
        );
        return AVIF_RESULT_NOT_IMPLEMENTED;
    }
    let mut i: uint8_t = 0 as uint8_t;
    while (i as core::ffi::c_int) < (*pixi).planeCount as core::ffi::c_int {
        if avifROStreamRead(
            &mut s,
            &mut *((*pixi).planeDepths).as_mut_ptr().offset(i as isize),
            1 as size_t,
        ) == 0
        {
            return AVIF_RESULT_BMFF_PARSE_FAILED;
        }
        if (*pixi).planeDepths[i as usize] as core::ffi::c_int == 0 as core::ffi::c_int {
            avifDiagnosticsPrintf(
                diag,
                b"Box[pixi] plane depth shall not be 0 for channel %u\0" as *const u8
                    as *const core::ffi::c_char,
                i as core::ffi::c_int,
            );
            return AVIF_RESULT_BMFF_PARSE_FAILED;
        }
        if (*pixi).planeDepths[i as usize] as core::ffi::c_int > 16 as core::ffi::c_int {
            avifDiagnosticsPrintf(
                diag,
                b"Box[pixi] plane depth %d is not supported\0" as *const u8
                    as *const core::ffi::c_char,
                (*pixi).planeDepths[i as usize] as core::ffi::c_int,
            );
            return AVIF_RESULT_NOT_IMPLEMENTED;
        }
        if (*pixi).planeDepths[i as usize] as core::ffi::c_int
            != (*pixi).planeDepths[0 as core::ffi::c_int as usize] as core::ffi::c_int
        {
            avifDiagnosticsPrintf(
                diag,
                b"Box[pixi] contains unsupported mismatched plane depths [%u != %u]\0"
                    as *const u8 as *const core::ffi::c_char,
                (*pixi).planeDepths[i as usize] as core::ffi::c_int,
                (*pixi).planeDepths[0 as core::ffi::c_int as usize] as core::ffi::c_int,
            );
            return AVIF_RESULT_NOT_IMPLEMENTED;
        }
        i = i.wrapping_add(1);
    }
    return AVIF_RESULT_OK;
}
unsafe extern "C" fn avifParseOperatingPointSelectorProperty(
    mut prop: *mut avifProperty,
    mut raw: *const uint8_t,
    mut rawLen: size_t,
    mut diag: *mut avifDiagnostics,
) -> avifBool {
    let mut s: avifROStream = avifROStream {
        raw: 0 as *mut avifROData,
        offset: 0,
        numUsedBitsInPartialByte: 0,
        diag: 0 as *mut avifDiagnostics,
        diagContext: 0 as *const core::ffi::c_char,
    };
    let mut s_roData: avifROData = avifROData {
        data: 0 as *const uint8_t,
        size: 0,
    };
    s_roData.data = raw;
    s_roData.size = rawLen;
    avifROStreamStart(
        &mut s,
        &mut s_roData,
        diag,
        b"Box[a1op]\0" as *const u8 as *const core::ffi::c_char,
    );
    let mut a1op: *mut avifOperatingPointSelectorProperty = &mut (*prop).u.a1op;
    if avifROStreamRead(&mut s, &mut (*a1op).opIndex, 1 as size_t) == 0 {
        return AVIF_FALSE;
    }
    if (*a1op).opIndex as core::ffi::c_int > 31 as core::ffi::c_int {
        avifDiagnosticsPrintf(
            diag,
            b"Box[a1op] contains an unsupported operating point [%u]\0" as *const u8
                as *const core::ffi::c_char,
            (*a1op).opIndex as core::ffi::c_int,
        );
        return AVIF_FALSE;
    }
    return AVIF_TRUE;
}
unsafe extern "C" fn avifParseLayerSelectorProperty(
    mut prop: *mut avifProperty,
    mut raw: *const uint8_t,
    mut rawLen: size_t,
    mut diag: *mut avifDiagnostics,
) -> avifBool {
    let mut s: avifROStream = avifROStream {
        raw: 0 as *mut avifROData,
        offset: 0,
        numUsedBitsInPartialByte: 0,
        diag: 0 as *mut avifDiagnostics,
        diagContext: 0 as *const core::ffi::c_char,
    };
    let mut s_roData: avifROData = avifROData {
        data: 0 as *const uint8_t,
        size: 0,
    };
    s_roData.data = raw;
    s_roData.size = rawLen;
    avifROStreamStart(
        &mut s,
        &mut s_roData,
        diag,
        b"Box[lsel]\0" as *const u8 as *const core::ffi::c_char,
    );
    let mut lsel: *mut avifLayerSelectorProperty = &mut (*prop).u.lsel;
    if avifROStreamReadU16(&mut s, &mut (*lsel).layerID) == 0 {
        return AVIF_FALSE;
    }
    if (*lsel).layerID as core::ffi::c_int != 0xffff as core::ffi::c_int
        && (*lsel).layerID as core::ffi::c_int >= AVIF_MAX_AV1_LAYER_COUNT
    {
        avifDiagnosticsPrintf(
            diag,
            b"Box[lsel] contains an unsupported layer [%u]\0" as *const u8
                as *const core::ffi::c_char,
            (*lsel).layerID as core::ffi::c_int,
        );
        return AVIF_FALSE;
    }
    return AVIF_TRUE;
}
unsafe extern "C" fn avifParseAV1LayeredImageIndexingProperty(
    mut prop: *mut avifProperty,
    mut raw: *const uint8_t,
    mut rawLen: size_t,
    mut diag: *mut avifDiagnostics,
) -> avifBool {
    let mut s: avifROStream = avifROStream {
        raw: 0 as *mut avifROData,
        offset: 0,
        numUsedBitsInPartialByte: 0,
        diag: 0 as *mut avifDiagnostics,
        diagContext: 0 as *const core::ffi::c_char,
    };
    let mut s_roData: avifROData = avifROData {
        data: 0 as *const uint8_t,
        size: 0,
    };
    s_roData.data = raw;
    s_roData.size = rawLen;
    avifROStreamStart(
        &mut s,
        &mut s_roData,
        diag,
        b"Box[a1lx]\0" as *const u8 as *const core::ffi::c_char,
    );
    let mut a1lx: *mut avifAV1LayeredImageIndexingProperty = &mut (*prop).u.a1lx;
    let mut largeSize: uint8_t = 0 as uint8_t;
    if avifROStreamRead(&mut s, &mut largeSize, 1 as size_t) == 0 {
        return AVIF_FALSE;
    }
    if largeSize as core::ffi::c_int & 0xfe as core::ffi::c_int != 0 {
        avifDiagnosticsPrintf(
            diag,
            b"Box[a1lx] has bits set in the reserved section [%u]\0" as *const u8
                as *const core::ffi::c_char,
            largeSize as core::ffi::c_int,
        );
        return AVIF_FALSE;
    }
    let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
    while i < 3 as core::ffi::c_int {
        if largeSize != 0 {
            if avifROStreamReadU32(
                &mut s,
                &mut *((*a1lx).layerSize).as_mut_ptr().offset(i as isize),
            ) == 0
            {
                return AVIF_FALSE;
            }
        } else {
            let mut layerSize16: uint16_t = 0;
            if avifROStreamReadU16(&mut s, &mut layerSize16) == 0 {
                return AVIF_FALSE;
            }
            (*a1lx).layerSize[i as usize] = layerSize16 as uint32_t;
        }
        i += 1;
    }
    return AVIF_TRUE;
}
unsafe extern "C" fn avifParseItemPropertyContainerBox(
    mut properties: *mut avifPropertyArray,
    mut rawOffset: uint64_t,
    mut raw: *const uint8_t,
    mut rawLen: size_t,
    mut isTrack: avifBool,
    mut diag: *mut avifDiagnostics,
) -> avifResult {
    let mut s: avifROStream = avifROStream {
        raw: 0 as *mut avifROData,
        offset: 0,
        numUsedBitsInPartialByte: 0,
        diag: 0 as *mut avifDiagnostics,
        diagContext: 0 as *const core::ffi::c_char,
    };
    let mut s_roData: avifROData = avifROData {
        data: 0 as *const uint8_t,
        size: 0,
    };
    s_roData.data = raw;
    s_roData.size = rawLen;
    avifROStreamStart(
        &mut s,
        &mut s_roData,
        diag,
        b"Box[ipco]\0" as *const u8 as *const core::ffi::c_char,
    );
    while avifROStreamHasBytesLeft(&mut s, 1 as size_t) != 0 {
        let mut header: avifBoxHeader = avifBoxHeader {
            isSizeZeroBox: 0,
            size: 0,
            type_0: [0; 4],
            usertype: [0; 16],
        };
        if avifROStreamReadBoxHeader(&mut s, &mut header) == 0 {
            return AVIF_RESULT_BMFF_PARSE_FAILED;
        }
        let mut prop: *mut avifProperty = avifArrayPush(
            properties as *mut core::ffi::c_void,
        ) as *mut avifProperty;
        if prop.is_null() {
            return AVIF_RESULT_OUT_OF_MEMORY;
        }
        memcpy(
            ((*prop).type_0).as_mut_ptr() as *mut core::ffi::c_void,
            (header.type_0).as_mut_ptr() as *const core::ffi::c_void,
            4 as size_t,
        );
        (*prop).isOpaque = AVIF_FALSE as avifBool;
        if memcmp(
            (header.type_0).as_mut_ptr() as *const core::ffi::c_void,
            b"ispe\0" as *const u8 as *const core::ffi::c_char
                as *const core::ffi::c_void,
            4 as size_t,
        ) == 0
        {
            if avifParseImageSpatialExtentsProperty(
                prop,
                avifROStreamCurrent(&mut s),
                header.size,
                diag,
            ) == 0
            {
                return AVIF_RESULT_BMFF_PARSE_FAILED;
            }
        } else if memcmp(
            (header.type_0).as_mut_ptr() as *const core::ffi::c_void,
            b"auxC\0" as *const u8 as *const core::ffi::c_char
                as *const core::ffi::c_void,
            4 as size_t,
        ) == 0 && isTrack == 0
            || memcmp(
                (header.type_0).as_mut_ptr() as *const core::ffi::c_void,
                b"auxi\0" as *const u8 as *const core::ffi::c_char
                    as *const core::ffi::c_void,
                4 as size_t,
            ) == 0 && isTrack != 0
        {
            if avifParseAuxiliaryTypeProperty(
                prop,
                avifROStreamCurrent(&mut s),
                header.size,
                diag,
            ) == 0
            {
                return AVIF_RESULT_BMFF_PARSE_FAILED;
            }
        } else if memcmp(
            (header.type_0).as_mut_ptr() as *const core::ffi::c_void,
            b"colr\0" as *const u8 as *const core::ffi::c_char
                as *const core::ffi::c_void,
            4 as size_t,
        ) == 0
        {
            if avifParseColourInformationBox(
                prop,
                rawOffset.wrapping_add(avifROStreamOffset(&mut s) as uint64_t),
                avifROStreamCurrent(&mut s),
                header.size,
                diag,
            ) == 0
            {
                return AVIF_RESULT_BMFF_PARSE_FAILED;
            }
        } else if memcmp(
            (header.type_0).as_mut_ptr() as *const core::ffi::c_void,
            b"av1C\0" as *const u8 as *const core::ffi::c_char
                as *const core::ffi::c_void,
            4 as size_t,
        ) == 0
        {
            if avifParseCodecConfigurationBoxProperty(
                prop,
                avifROStreamCurrent(&mut s),
                header.size,
                b"av1C\0" as *const u8 as *const core::ffi::c_char,
                diag,
            ) == 0
            {
                return AVIF_RESULT_BMFF_PARSE_FAILED;
            }
        } else if memcmp(
            (header.type_0).as_mut_ptr() as *const core::ffi::c_void,
            b"pasp\0" as *const u8 as *const core::ffi::c_char
                as *const core::ffi::c_void,
            4 as size_t,
        ) == 0
        {
            if avifParsePixelAspectRatioBoxProperty(
                prop,
                avifROStreamCurrent(&mut s),
                header.size,
                diag,
            ) == 0
            {
                return AVIF_RESULT_BMFF_PARSE_FAILED;
            }
        } else if memcmp(
            (header.type_0).as_mut_ptr() as *const core::ffi::c_void,
            b"clap\0" as *const u8 as *const core::ffi::c_char
                as *const core::ffi::c_void,
            4 as size_t,
        ) == 0
        {
            if avifParseCleanApertureBoxProperty(
                prop,
                avifROStreamCurrent(&mut s),
                header.size,
                diag,
            ) == 0
            {
                return AVIF_RESULT_BMFF_PARSE_FAILED;
            }
        } else if memcmp(
            (header.type_0).as_mut_ptr() as *const core::ffi::c_void,
            b"irot\0" as *const u8 as *const core::ffi::c_char
                as *const core::ffi::c_void,
            4 as size_t,
        ) == 0
        {
            if avifParseImageRotationProperty(
                prop,
                avifROStreamCurrent(&mut s),
                header.size,
                diag,
            ) == 0
            {
                return AVIF_RESULT_BMFF_PARSE_FAILED;
            }
        } else if memcmp(
            (header.type_0).as_mut_ptr() as *const core::ffi::c_void,
            b"imir\0" as *const u8 as *const core::ffi::c_char
                as *const core::ffi::c_void,
            4 as size_t,
        ) == 0
        {
            if avifParseImageMirrorProperty(
                prop,
                avifROStreamCurrent(&mut s),
                header.size,
                diag,
            ) == 0
            {
                return AVIF_RESULT_BMFF_PARSE_FAILED;
            }
        } else if memcmp(
            (header.type_0).as_mut_ptr() as *const core::ffi::c_void,
            b"pixi\0" as *const u8 as *const core::ffi::c_char
                as *const core::ffi::c_void,
            4 as size_t,
        ) == 0
        {
            let result__: avifResult = avifParsePixelInformationProperty(
                prop,
                avifROStreamCurrent(&mut s),
                header.size,
                diag,
            ) as avifResult;
            if result__ as core::ffi::c_uint
                != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
            {
                return result__;
            }
        } else if memcmp(
            (header.type_0).as_mut_ptr() as *const core::ffi::c_void,
            b"a1op\0" as *const u8 as *const core::ffi::c_char
                as *const core::ffi::c_void,
            4 as size_t,
        ) == 0
        {
            if avifParseOperatingPointSelectorProperty(
                prop,
                avifROStreamCurrent(&mut s),
                header.size,
                diag,
            ) == 0
            {
                return AVIF_RESULT_BMFF_PARSE_FAILED;
            }
        } else if memcmp(
            (header.type_0).as_mut_ptr() as *const core::ffi::c_void,
            b"lsel\0" as *const u8 as *const core::ffi::c_char
                as *const core::ffi::c_void,
            4 as size_t,
        ) == 0
        {
            if avifParseLayerSelectorProperty(
                prop,
                avifROStreamCurrent(&mut s),
                header.size,
                diag,
            ) == 0
            {
                return AVIF_RESULT_BMFF_PARSE_FAILED;
            }
        } else if memcmp(
            (header.type_0).as_mut_ptr() as *const core::ffi::c_void,
            b"a1lx\0" as *const u8 as *const core::ffi::c_char
                as *const core::ffi::c_void,
            4 as size_t,
        ) == 0
        {
            if avifParseAV1LayeredImageIndexingProperty(
                prop,
                avifROStreamCurrent(&mut s),
                header.size,
                diag,
            ) == 0
            {
                return AVIF_RESULT_BMFF_PARSE_FAILED;
            }
        } else if memcmp(
            (header.type_0).as_mut_ptr() as *const core::ffi::c_void,
            b"clli\0" as *const u8 as *const core::ffi::c_char
                as *const core::ffi::c_void,
            4 as size_t,
        ) == 0
        {
            let result___0: avifResult = avifParseContentLightLevelInformationBox(
                prop,
                avifROStreamCurrent(&mut s),
                header.size,
                diag,
            ) as avifResult;
            if result___0 as core::ffi::c_uint
                != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
            {
                return result___0;
            }
        } else {
            (*prop).isOpaque = AVIF_TRUE as avifBool;
            memset(
                &mut (*prop).u.opaque as *mut avifOpaqueProperty
                    as *mut core::ffi::c_void,
                0 as core::ffi::c_int,
                ::core::mem::size_of::<avifOpaqueProperty>() as size_t,
            );
            memcpy(
                ((*prop).u.opaque.usertype).as_mut_ptr() as *mut core::ffi::c_void,
                (header.usertype).as_mut_ptr() as *const core::ffi::c_void,
                ::core::mem::size_of::<[uint8_t; 16]>() as size_t,
            );
            let result___1: avifResult = avifRWDataSet(
                &mut (*prop).u.opaque.boxPayload,
                avifROStreamCurrent(&mut s),
                header.size,
            ) as avifResult;
            if result___1 as core::ffi::c_uint
                != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
            {
                return result___1;
            }
        }
        if avifROStreamSkip(&mut s, header.size) == 0 {
            return AVIF_RESULT_BMFF_PARSE_FAILED;
        }
    }
    return AVIF_RESULT_OK;
}
unsafe extern "C" fn avifParseItemPropertyAssociation(
    mut meta: *mut avifMeta,
    mut raw: *const uint8_t,
    mut rawLen: size_t,
    mut diag: *mut avifDiagnostics,
    mut outVersionAndFlags: *mut uint32_t,
) -> avifResult {
    let mut s: avifROStream = avifROStream {
        raw: 0 as *mut avifROData,
        offset: 0,
        numUsedBitsInPartialByte: 0,
        diag: 0 as *mut avifDiagnostics,
        diagContext: 0 as *const core::ffi::c_char,
    };
    let mut s_roData: avifROData = avifROData {
        data: 0 as *const uint8_t,
        size: 0,
    };
    s_roData.data = raw;
    s_roData.size = rawLen;
    avifROStreamStart(
        &mut s,
        &mut s_roData,
        diag,
        b"Box[ipma]\0" as *const u8 as *const core::ffi::c_char,
    );
    let mut version: uint8_t = 0;
    let mut flags: uint32_t = 0;
    if avifROStreamReadVersionAndFlags(&mut s, &mut version, &mut flags) == 0 {
        return AVIF_RESULT_BMFF_PARSE_FAILED;
    }
    let mut propertyIndexIsU15: avifBool = (flags & 0x1 as uint32_t != 0 as uint32_t)
        as core::ffi::c_int;
    *outVersionAndFlags = (version as uint32_t) << 24 as core::ffi::c_int | flags;
    let mut entryCount: uint32_t = 0;
    if avifROStreamReadU32(&mut s, &mut entryCount) == 0 {
        return AVIF_RESULT_BMFF_PARSE_FAILED;
    }
    let mut prevItemID: core::ffi::c_uint = 0 as core::ffi::c_uint;
    let mut entryIndex: uint32_t = 0 as uint32_t;
    while entryIndex < entryCount {
        let mut itemID: core::ffi::c_uint = 0;
        if (version as core::ffi::c_int) < 1 as core::ffi::c_int {
            let mut tmp: uint16_t = 0;
            if avifROStreamReadU16(&mut s, &mut tmp) == 0 {
                return AVIF_RESULT_BMFF_PARSE_FAILED;
            }
            itemID = tmp as core::ffi::c_uint;
        } else if avifROStreamReadU32(&mut s, &mut itemID) == 0 {
            return AVIF_RESULT_BMFF_PARSE_FAILED
        }
        let result__: avifResult = avifCheckItemID(
            b"ipma\0" as *const u8 as *const core::ffi::c_char,
            itemID as uint32_t,
            diag,
        ) as avifResult;
        if result__ as core::ffi::c_uint
            != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
        {
            return result__;
        }
        if itemID <= prevItemID {
            avifDiagnosticsPrintf(
                diag,
                b"Box[ipma] item IDs are not ordered by increasing ID\0" as *const u8
                    as *const core::ffi::c_char,
            );
            return AVIF_RESULT_BMFF_PARSE_FAILED;
        }
        prevItemID = itemID;
        let mut item: *mut avifDecoderItem = 0 as *mut avifDecoderItem;
        let result___0: avifResult = avifMetaFindOrCreateItem(
            meta,
            itemID as uint32_t,
            &mut item,
        ) as avifResult;
        if result___0 as core::ffi::c_uint
            != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
        {
            return result___0;
        }
        if (*item).ipmaSeen != 0 {
            avifDiagnosticsPrintf(
                diag,
                b"Duplicate Box[ipma] for item ID [%u]\0" as *const u8
                    as *const core::ffi::c_char,
                itemID,
            );
            return AVIF_RESULT_BMFF_PARSE_FAILED;
        }
        (*item).ipmaSeen = AVIF_TRUE as avifBool;
        let mut associationCount: uint8_t = 0;
        if avifROStreamRead(&mut s, &mut associationCount, 1 as size_t) == 0 {
            return AVIF_RESULT_BMFF_PARSE_FAILED;
        }
        let mut associationIndex: uint8_t = 0 as uint8_t;
        while (associationIndex as core::ffi::c_int)
            < associationCount as core::ffi::c_int
        {
            let mut essential: uint8_t = 0;
            if avifROStreamReadBitsU8(&mut s, &mut essential, 1 as size_t) == 0 {
                return AVIF_RESULT_BMFF_PARSE_FAILED;
            }
            let mut propertyIndex: uint32_t = 0;
            if avifROStreamReadBitsU32(
                &mut s,
                &mut propertyIndex,
                (if propertyIndexIsU15 != 0 {
                    15 as core::ffi::c_int
                } else {
                    7 as core::ffi::c_int
                }) as size_t,
            ) == 0
            {
                return AVIF_RESULT_BMFF_PARSE_FAILED;
            }
            if propertyIndex == 0 as uint32_t {
                if essential != 0 {
                    avifDiagnosticsPrintf(
                        diag,
                        b"Box[ipma] for item ID [%u] contains an illegal essential property index 0\0"
                            as *const u8 as *const core::ffi::c_char,
                        itemID,
                    );
                    return AVIF_RESULT_BMFF_PARSE_FAILED;
                }
            } else {
                propertyIndex = propertyIndex.wrapping_sub(1);
                if propertyIndex >= (*meta).properties.count {
                    avifDiagnosticsPrintf(
                        diag,
                        b"Box[ipma] for item ID [%u] contains an illegal property index [%u] (out of [%u] properties)\0"
                            as *const u8 as *const core::ffi::c_char,
                        itemID,
                        propertyIndex,
                        (*meta).properties.count,
                    );
                    return AVIF_RESULT_BMFF_PARSE_FAILED;
                }
                let mut srcProp: *const avifProperty = &mut *((*meta).properties.prop)
                    .offset(propertyIndex as isize) as *mut avifProperty;
                let supportedType: avifBool = ((*srcProp).isOpaque == 0)
                    as core::ffi::c_int;
                if supportedType != 0 {
                    if essential != 0 {
                        static mut nonessentialTypes: [*const core::ffi::c_char; 1] = [
                            b"a1lx\0" as *const u8 as *const core::ffi::c_char,
                        ];
                        let mut nonessentialTypesCount: size_t = (::core::mem::size_of::<
                            [*const core::ffi::c_char; 1],
                        >() as size_t)
                            .wrapping_div(
                                ::core::mem::size_of::<*const core::ffi::c_char>() as size_t,
                            );
                        let mut i: size_t = 0 as size_t;
                        while i < nonessentialTypesCount {
                            if memcmp(
                                ((*srcProp).type_0).as_ptr() as *const core::ffi::c_void,
                                nonessentialTypes[i as usize] as *const core::ffi::c_void,
                                4 as size_t,
                            ) == 0
                            {
                                avifDiagnosticsPrintf(
                                    diag,
                                    b"Item ID [%u] has a %s property association which must not be marked essential, but is\0"
                                        as *const u8 as *const core::ffi::c_char,
                                    itemID,
                                    nonessentialTypes[i as usize],
                                );
                                return AVIF_RESULT_BMFF_PARSE_FAILED;
                            }
                            i = i.wrapping_add(1);
                        }
                    } else {
                        static mut essentialTypes: [*const core::ffi::c_char; 5] = [
                            b"a1op\0" as *const u8 as *const core::ffi::c_char,
                            b"lsel\0" as *const u8 as *const core::ffi::c_char,
                            b"clap\0" as *const u8 as *const core::ffi::c_char,
                            b"irot\0" as *const u8 as *const core::ffi::c_char,
                            b"imir\0" as *const u8 as *const core::ffi::c_char,
                        ];
                        let mut essentialTypesCount: size_t = (::core::mem::size_of::<
                            [*const core::ffi::c_char; 5],
                        >() as size_t)
                            .wrapping_div(
                                ::core::mem::size_of::<*const core::ffi::c_char>() as size_t,
                            );
                        let mut i_0: size_t = 0 as size_t;
                        while i_0 < essentialTypesCount {
                            if memcmp(
                                ((*srcProp).type_0).as_ptr() as *const core::ffi::c_void,
                                essentialTypes[i_0 as usize] as *const core::ffi::c_void,
                                4 as size_t,
                            ) == 0
                            {
                                avifDiagnosticsPrintf(
                                    diag,
                                    b"Item ID [%u] has a %s property association which must be marked essential, but is not\0"
                                        as *const u8 as *const core::ffi::c_char,
                                    itemID,
                                    essentialTypes[i_0 as usize],
                                );
                                return AVIF_RESULT_BMFF_PARSE_FAILED;
                            }
                            i_0 = i_0.wrapping_add(1);
                        }
                    }
                    let mut dstProp: *mut avifProperty = avifArrayPush(
                        &mut (*item).properties as *mut avifPropertyArray
                            as *mut core::ffi::c_void,
                    ) as *mut avifProperty;
                    if dstProp.is_null() {
                        return AVIF_RESULT_OUT_OF_MEMORY;
                    }
                    *dstProp = *srcProp;
                } else {
                    if essential != 0 {
                        (*item).hasUnsupportedEssentialProperty = AVIF_TRUE as avifBool;
                    }
                    let mut dstProp_0: *mut avifProperty = avifArrayPush(
                        &mut (*item).properties as *mut avifPropertyArray
                            as *mut core::ffi::c_void,
                    ) as *mut avifProperty;
                    if dstProp_0.is_null() {
                        return AVIF_RESULT_OUT_OF_MEMORY;
                    }
                    (*dstProp_0).isOpaque = AVIF_TRUE as avifBool;
                    memcpy(
                        ((*dstProp_0).type_0).as_mut_ptr() as *mut core::ffi::c_void,
                        ((*srcProp).type_0).as_ptr() as *const core::ffi::c_void,
                        ::core::mem::size_of::<[uint8_t; 4]>() as size_t,
                    );
                    memcpy(
                        ((*dstProp_0).u.opaque.usertype).as_mut_ptr()
                            as *mut core::ffi::c_void,
                        ((*srcProp).u.opaque.usertype).as_ptr()
                            as *const core::ffi::c_void,
                        ::core::mem::size_of::<[uint8_t; 16]>() as size_t,
                    );
                    let result___1: avifResult = avifRWDataSet(
                        &mut (*dstProp_0).u.opaque.boxPayload,
                        (*srcProp).u.opaque.boxPayload.data,
                        (*srcProp).u.opaque.boxPayload.size,
                    ) as avifResult;
                    if result___1 as core::ffi::c_uint
                        != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
                    {
                        return result___1;
                    }
                }
            }
            associationIndex = associationIndex.wrapping_add(1);
        }
        entryIndex = entryIndex.wrapping_add(1);
    }
    return AVIF_RESULT_OK;
}
unsafe extern "C" fn avifParsePrimaryItemBox(
    mut meta: *mut avifMeta,
    mut raw: *const uint8_t,
    mut rawLen: size_t,
    mut diag: *mut avifDiagnostics,
) -> avifBool {
    if (*meta).primaryItemID > 0 as uint32_t {
        avifDiagnosticsPrintf(
            diag,
            b"Multiple boxes of unique Box[pitm] found\0" as *const u8
                as *const core::ffi::c_char,
        );
        return AVIF_FALSE;
    }
    let mut s: avifROStream = avifROStream {
        raw: 0 as *mut avifROData,
        offset: 0,
        numUsedBitsInPartialByte: 0,
        diag: 0 as *mut avifDiagnostics,
        diagContext: 0 as *const core::ffi::c_char,
    };
    let mut s_roData: avifROData = avifROData {
        data: 0 as *const uint8_t,
        size: 0,
    };
    s_roData.data = raw;
    s_roData.size = rawLen;
    avifROStreamStart(
        &mut s,
        &mut s_roData,
        diag,
        b"Box[pitm]\0" as *const u8 as *const core::ffi::c_char,
    );
    let mut version: uint8_t = 0;
    if avifROStreamReadVersionAndFlags(&mut s, &mut version, 0 as *mut uint32_t) == 0 {
        return AVIF_FALSE;
    }
    if version as core::ffi::c_int == 0 as core::ffi::c_int {
        let mut tmp16: uint16_t = 0;
        if avifROStreamReadU16(&mut s, &mut tmp16) == 0 {
            return AVIF_FALSE;
        }
        (*meta).primaryItemID = tmp16 as uint32_t;
    } else if avifROStreamReadU32(&mut s, &mut (*meta).primaryItemID) == 0 {
        return AVIF_FALSE
    }
    return AVIF_TRUE;
}
unsafe extern "C" fn avifParseItemDataBox(
    mut meta: *mut avifMeta,
    mut raw: *const uint8_t,
    mut rawLen: size_t,
    mut diag: *mut avifDiagnostics,
) -> avifBool {
    if (*meta).idat.size > 0 as size_t {
        avifDiagnosticsPrintf(
            diag,
            b"Meta box contains multiple idat boxes\0" as *const u8
                as *const core::ffi::c_char,
        );
        return AVIF_FALSE;
    }
    if rawLen == 0 as size_t {
        avifDiagnosticsPrintf(
            diag,
            b"idat box has a length of 0\0" as *const u8 as *const core::ffi::c_char,
        );
        return AVIF_FALSE;
    }
    if avifRWDataSet(&mut (*meta).idat, raw, rawLen) as core::ffi::c_uint
        != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
    {
        return AVIF_FALSE;
    }
    return AVIF_TRUE;
}
unsafe extern "C" fn avifParseItemPropertiesBox(
    mut meta: *mut avifMeta,
    mut rawOffset: uint64_t,
    mut raw: *const uint8_t,
    mut rawLen: size_t,
    mut diag: *mut avifDiagnostics,
) -> avifResult {
    let mut s: avifROStream = avifROStream {
        raw: 0 as *mut avifROData,
        offset: 0,
        numUsedBitsInPartialByte: 0,
        diag: 0 as *mut avifDiagnostics,
        diagContext: 0 as *const core::ffi::c_char,
    };
    let mut s_roData: avifROData = avifROData {
        data: 0 as *const uint8_t,
        size: 0,
    };
    s_roData.data = raw;
    s_roData.size = rawLen;
    avifROStreamStart(
        &mut s,
        &mut s_roData,
        diag,
        b"Box[iprp]\0" as *const u8 as *const core::ffi::c_char,
    );
    let mut ipcoHeader: avifBoxHeader = avifBoxHeader {
        isSizeZeroBox: 0,
        size: 0,
        type_0: [0; 4],
        usertype: [0; 16],
    };
    if avifROStreamReadBoxHeader(&mut s, &mut ipcoHeader) == 0 {
        return AVIF_RESULT_BMFF_PARSE_FAILED;
    }
    if memcmp(
        (ipcoHeader.type_0).as_mut_ptr() as *const core::ffi::c_void,
        b"ipco\0" as *const u8 as *const core::ffi::c_char as *const core::ffi::c_void,
        4 as size_t,
    ) != 0
    {
        avifDiagnosticsPrintf(
            diag,
            b"Failed to find Box[ipco] as the first box in Box[iprp]\0" as *const u8
                as *const core::ffi::c_char,
        );
        return AVIF_RESULT_BMFF_PARSE_FAILED;
    }
    let result__: avifResult = avifParseItemPropertyContainerBox(
        &mut (*meta).properties,
        rawOffset.wrapping_add(avifROStreamOffset(&mut s) as uint64_t),
        avifROStreamCurrent(&mut s),
        ipcoHeader.size,
        0 as avifBool,
        diag,
    ) as avifResult;
    if result__ as core::ffi::c_uint
        != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
    {
        return result__;
    }
    if avifROStreamSkip(&mut s, ipcoHeader.size) == 0 {
        return AVIF_RESULT_BMFF_PARSE_FAILED;
    }
    let mut versionAndFlagsSeen: [uint32_t; 4] = [0; 4];
    let mut versionAndFlagsSeenCount: uint32_t = 0 as uint32_t;
    while avifROStreamHasBytesLeft(&mut s, 1 as size_t) != 0 {
        let mut ipmaHeader: avifBoxHeader = avifBoxHeader {
            isSizeZeroBox: 0,
            size: 0,
            type_0: [0; 4],
            usertype: [0; 16],
        };
        if avifROStreamReadBoxHeader(&mut s, &mut ipmaHeader) == 0 {
            return AVIF_RESULT_BMFF_PARSE_FAILED;
        }
        if memcmp(
            (ipmaHeader.type_0).as_mut_ptr() as *const core::ffi::c_void,
            b"ipma\0" as *const u8 as *const core::ffi::c_char
                as *const core::ffi::c_void,
            4 as size_t,
        ) == 0
        {
            let mut versionAndFlags: uint32_t = 0;
            let result___0: avifResult = avifParseItemPropertyAssociation(
                meta,
                avifROStreamCurrent(&mut s),
                ipmaHeader.size,
                diag,
                &mut versionAndFlags,
            ) as avifResult;
            if result___0 as core::ffi::c_uint
                != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
            {
                return result___0;
            }
            let mut i: uint32_t = 0 as uint32_t;
            while i < versionAndFlagsSeenCount {
                if versionAndFlagsSeen[i as usize] == versionAndFlags {
                    avifDiagnosticsPrintf(
                        diag,
                        b"Multiple Box[ipma] with a given pair of values of version and flags. See BMFF (ISO/IEC 14496-12:2022) 8.11.14.1\0"
                            as *const u8 as *const core::ffi::c_char,
                    );
                    return AVIF_RESULT_BMFF_PARSE_FAILED;
                }
                i = i.wrapping_add(1);
            }
            if versionAndFlagsSeenCount == MAX_IPMA_VERSION_AND_FLAGS_SEEN as uint32_t {
                avifDiagnosticsPrintf(
                    diag,
                    b"Exceeded possible count of unique ipma version and flags tuples\0"
                        as *const u8 as *const core::ffi::c_char,
                );
                return AVIF_RESULT_BMFF_PARSE_FAILED;
            }
            versionAndFlagsSeen[versionAndFlagsSeenCount as usize] = versionAndFlags;
            versionAndFlagsSeenCount = versionAndFlagsSeenCount.wrapping_add(1);
        } else {
            avifDiagnosticsPrintf(
                diag,
                b"Box[iprp] contains a box that isn't type 'ipma'\0" as *const u8
                    as *const core::ffi::c_char,
            );
            return AVIF_RESULT_BMFF_PARSE_FAILED;
        }
        if avifROStreamSkip(&mut s, ipmaHeader.size) == 0 {
            return AVIF_RESULT_BMFF_PARSE_FAILED;
        }
    }
    return AVIF_RESULT_OK;
}
unsafe extern "C" fn avifParseItemInfoEntry(
    mut meta: *mut avifMeta,
    mut raw: *const uint8_t,
    mut rawLen: size_t,
    mut diag: *mut avifDiagnostics,
) -> avifResult {
    let mut s: avifROStream = avifROStream {
        raw: 0 as *mut avifROData,
        offset: 0,
        numUsedBitsInPartialByte: 0,
        diag: 0 as *mut avifDiagnostics,
        diagContext: 0 as *const core::ffi::c_char,
    };
    let mut s_roData: avifROData = avifROData {
        data: 0 as *const uint8_t,
        size: 0,
    };
    s_roData.data = raw;
    s_roData.size = rawLen;
    avifROStreamStart(
        &mut s,
        &mut s_roData,
        diag,
        b"Box[infe]\0" as *const u8 as *const core::ffi::c_char,
    );
    let mut version: uint8_t = 0;
    let mut flags: uint32_t = 0;
    if avifROStreamReadVersionAndFlags(&mut s, &mut version, &mut flags) == 0 {
        return AVIF_RESULT_BMFF_PARSE_FAILED;
    }
    if version as core::ffi::c_int != 2 as core::ffi::c_int
        && version as core::ffi::c_int != 3 as core::ffi::c_int
    {
        avifDiagnosticsPrintf(
            s.diag,
            b"%s: Expecting box version 2 or 3, got version %u\0" as *const u8
                as *const core::ffi::c_char,
            s.diagContext,
            version as core::ffi::c_int,
        );
        return AVIF_RESULT_BMFF_PARSE_FAILED;
    }
    let mut itemID: uint32_t = 0;
    if version as core::ffi::c_int == 2 as core::ffi::c_int {
        let mut tmp: uint16_t = 0;
        if avifROStreamReadU16(&mut s, &mut tmp) == 0 {
            return AVIF_RESULT_BMFF_PARSE_FAILED;
        }
        itemID = tmp as uint32_t;
    } else {
        'c_28813: {
            if version as core::ffi::c_int == 3 as core::ffi::c_int {} else {
                __assert_fail(
                    b"version == 3\0" as *const u8 as *const core::ffi::c_char,
                    b"/home/runner/work/libavif-rs/libavif-rs/upstream/libavif/src/read.c\0"
                        as *const u8 as *const core::ffi::c_char,
                    3368 as core::ffi::c_uint,
                    (::core::mem::transmute::<
                        [u8; 90],
                        [core::ffi::c_char; 90],
                    >(
                        *b"avifResult avifParseItemInfoEntry(avifMeta *, const uint8_t *, size_t, avifDiagnostics *)\0",
                    ))
                        .as_ptr(),
                );
            }
        };
        if avifROStreamReadU32(&mut s, &mut itemID) == 0 {
            return AVIF_RESULT_BMFF_PARSE_FAILED;
        }
    }
    let result__: avifResult = avifCheckItemID(
        b"infe\0" as *const u8 as *const core::ffi::c_char,
        itemID,
        diag,
    ) as avifResult;
    if result__ as core::ffi::c_uint
        != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
    {
        return result__;
    }
    let mut itemProtectionIndex: uint16_t = 0;
    if avifROStreamReadU16(&mut s, &mut itemProtectionIndex) == 0 {
        return AVIF_RESULT_BMFF_PARSE_FAILED;
    }
    let mut itemType: [uint8_t; 4] = [0; 4];
    if avifROStreamRead(&mut s, itemType.as_mut_ptr(), 4 as size_t) == 0 {
        return AVIF_RESULT_BMFF_PARSE_FAILED;
    }
    if avifROStreamReadString(&mut s, 0 as *mut core::ffi::c_char, 0 as size_t) == 0 {
        return AVIF_RESULT_BMFF_PARSE_FAILED;
    }
    let mut contentType: avifContentType = avifContentType {
        contentType: [0; 64],
    };
    if memcmp(
        itemType.as_mut_ptr() as *const core::ffi::c_void,
        b"mime\0" as *const u8 as *const core::ffi::c_char as *const core::ffi::c_void,
        4 as size_t,
    ) == 0
    {
        if avifROStreamReadString(
            &mut s,
            (contentType.contentType).as_mut_ptr(),
            64 as size_t,
        ) == 0
        {
            return AVIF_RESULT_BMFF_PARSE_FAILED;
        }
    } else {
        memset(
            &mut contentType as *mut avifContentType as *mut core::ffi::c_void,
            0 as core::ffi::c_int,
            ::core::mem::size_of::<avifContentType>() as size_t,
        );
    }
    let mut item: *mut avifDecoderItem = 0 as *mut avifDecoderItem;
    let result___0: avifResult = avifMetaFindOrCreateItem(meta, itemID, &mut item)
        as avifResult;
    if result___0 as core::ffi::c_uint
        != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
    {
        return result___0;
    }
    memcpy(
        ((*item).type_0).as_mut_ptr() as *mut core::ffi::c_void,
        itemType.as_mut_ptr() as *const core::ffi::c_void,
        ::core::mem::size_of::<[uint8_t; 4]>() as size_t,
    );
    (*item).contentType = contentType;
    return AVIF_RESULT_OK;
}
unsafe extern "C" fn avifParseItemInfoBox(
    mut meta: *mut avifMeta,
    mut raw: *const uint8_t,
    mut rawLen: size_t,
    mut diag: *mut avifDiagnostics,
) -> avifResult {
    let mut s: avifROStream = avifROStream {
        raw: 0 as *mut avifROData,
        offset: 0,
        numUsedBitsInPartialByte: 0,
        diag: 0 as *mut avifDiagnostics,
        diagContext: 0 as *const core::ffi::c_char,
    };
    let mut s_roData: avifROData = avifROData {
        data: 0 as *const uint8_t,
        size: 0,
    };
    s_roData.data = raw;
    s_roData.size = rawLen;
    avifROStreamStart(
        &mut s,
        &mut s_roData,
        diag,
        b"Box[iinf]\0" as *const u8 as *const core::ffi::c_char,
    );
    let mut version: uint8_t = 0;
    if avifROStreamReadVersionAndFlags(&mut s, &mut version, 0 as *mut uint32_t) == 0 {
        return AVIF_RESULT_BMFF_PARSE_FAILED;
    }
    let mut entryCount: uint32_t = 0;
    if version as core::ffi::c_int == 0 as core::ffi::c_int {
        let mut tmp: uint16_t = 0;
        if avifROStreamReadU16(&mut s, &mut tmp) == 0 {
            return AVIF_RESULT_BMFF_PARSE_FAILED;
        }
        entryCount = tmp as uint32_t;
    } else if version as core::ffi::c_int == 1 as core::ffi::c_int {
        if avifROStreamReadU32(&mut s, &mut entryCount) == 0 {
            return AVIF_RESULT_BMFF_PARSE_FAILED;
        }
    } else {
        avifDiagnosticsPrintf(
            diag,
            b"Box[iinf] has an unsupported version %u\0" as *const u8
                as *const core::ffi::c_char,
            version as core::ffi::c_int,
        );
        return AVIF_RESULT_BMFF_PARSE_FAILED;
    }
    let mut entryIndex: uint32_t = 0 as uint32_t;
    while entryIndex < entryCount {
        let mut infeHeader: avifBoxHeader = avifBoxHeader {
            isSizeZeroBox: 0,
            size: 0,
            type_0: [0; 4],
            usertype: [0; 16],
        };
        if avifROStreamReadBoxHeader(&mut s, &mut infeHeader) == 0 {
            return AVIF_RESULT_BMFF_PARSE_FAILED;
        }
        if memcmp(
            (infeHeader.type_0).as_mut_ptr() as *const core::ffi::c_void,
            b"infe\0" as *const u8 as *const core::ffi::c_char
                as *const core::ffi::c_void,
            4 as size_t,
        ) == 0
        {
            let result__: avifResult = avifParseItemInfoEntry(
                meta,
                avifROStreamCurrent(&mut s),
                infeHeader.size,
                diag,
            ) as avifResult;
            if result__ as core::ffi::c_uint
                != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
            {
                return result__;
            }
        } else {
            avifDiagnosticsPrintf(
                diag,
                b"Box[iinf] contains a box that isn't type 'infe'\0" as *const u8
                    as *const core::ffi::c_char,
            );
            return AVIF_RESULT_BMFF_PARSE_FAILED;
        }
        if avifROStreamSkip(&mut s, infeHeader.size) == 0 {
            return AVIF_RESULT_BMFF_PARSE_FAILED;
        }
        entryIndex = entryIndex.wrapping_add(1);
    }
    return AVIF_RESULT_OK;
}
unsafe extern "C" fn avifParseItemReferenceBox(
    mut meta: *mut avifMeta,
    mut raw: *const uint8_t,
    mut rawLen: size_t,
    mut diag: *mut avifDiagnostics,
) -> avifResult {
    let mut s: avifROStream = avifROStream {
        raw: 0 as *mut avifROData,
        offset: 0,
        numUsedBitsInPartialByte: 0,
        diag: 0 as *mut avifDiagnostics,
        diagContext: 0 as *const core::ffi::c_char,
    };
    let mut s_roData: avifROData = avifROData {
        data: 0 as *const uint8_t,
        size: 0,
    };
    s_roData.data = raw;
    s_roData.size = rawLen;
    avifROStreamStart(
        &mut s,
        &mut s_roData,
        diag,
        b"Box[iref]\0" as *const u8 as *const core::ffi::c_char,
    );
    let mut version: uint8_t = 0;
    if avifROStreamReadVersionAndFlags(&mut s, &mut version, 0 as *mut uint32_t) == 0 {
        return AVIF_RESULT_BMFF_PARSE_FAILED;
    }
    if version as core::ffi::c_int > 1 as core::ffi::c_int {
        return AVIF_RESULT_OK;
    }
    while avifROStreamHasBytesLeft(&mut s, 1 as size_t) != 0 {
        let mut irefHeader: avifBoxHeader = avifBoxHeader {
            isSizeZeroBox: 0,
            size: 0,
            type_0: [0; 4],
            usertype: [0; 16],
        };
        if avifROStreamReadBoxHeader(&mut s, &mut irefHeader) == 0 {
            return AVIF_RESULT_BMFF_PARSE_FAILED;
        }
        let mut refStream: avifROStream = avifROStream {
            raw: 0 as *mut avifROData,
            offset: 0,
            numUsedBitsInPartialByte: 0,
            diag: 0 as *mut avifDiagnostics,
            diagContext: 0 as *const core::ffi::c_char,
        };
        let mut refStream_roData: avifROData = avifROData {
            data: 0 as *const uint8_t,
            size: 0,
        };
        refStream_roData.data = avifROStreamCurrent(&mut s);
        refStream_roData.size = irefHeader.size;
        avifROStreamStart(
            &mut refStream,
            &mut refStream_roData,
            diag,
            b"Box[iref] SingleItemTypeReference\0" as *const u8
                as *const core::ffi::c_char,
        );
        let mut fromID: uint32_t = 0 as uint32_t;
        if version as core::ffi::c_int == 0 as core::ffi::c_int {
            let mut tmp: uint16_t = 0;
            if avifROStreamReadU16(&mut refStream, &mut tmp) == 0 {
                return AVIF_RESULT_BMFF_PARSE_FAILED;
            }
            fromID = tmp as uint32_t;
        } else if avifROStreamReadU32(&mut refStream, &mut fromID) == 0 {
            return AVIF_RESULT_BMFF_PARSE_FAILED
        }
        let result__: avifResult = avifCheckItemID(
            b"iref\0" as *const u8 as *const core::ffi::c_char,
            fromID,
            diag,
        ) as avifResult;
        if result__ as core::ffi::c_uint
            != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
        {
            return result__;
        }
        let mut item: *mut avifDecoderItem = 0 as *mut avifDecoderItem;
        let result___0: avifResult = avifMetaFindOrCreateItem(meta, fromID, &mut item)
            as avifResult;
        if result___0 as core::ffi::c_uint
            != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
        {
            return result___0;
        }
        if memcmp(
            (irefHeader.type_0).as_mut_ptr() as *const core::ffi::c_void,
            b"dimg\0" as *const u8 as *const core::ffi::c_char
                as *const core::ffi::c_void,
            4 as size_t,
        ) == 0
        {
            if (*item).hasDimgFrom != 0 {
                avifDiagnosticsPrintf(
                    diag,
                    b"Box[iinf] contains duplicate boxes of type 'dimg' with the same from_item_ID value %u\0"
                        as *const u8 as *const core::ffi::c_char,
                    fromID,
                );
                return AVIF_RESULT_BMFF_PARSE_FAILED;
            }
            (*item).hasDimgFrom = AVIF_TRUE as avifBool;
        }
        let mut referenceCount: uint16_t = 0 as uint16_t;
        if avifROStreamReadU16(&mut refStream, &mut referenceCount) == 0 {
            return AVIF_RESULT_BMFF_PARSE_FAILED;
        }
        let mut refIndex: uint16_t = 0 as uint16_t;
        while (refIndex as core::ffi::c_int) < referenceCount as core::ffi::c_int {
            let mut toID: uint32_t = 0 as uint32_t;
            if version as core::ffi::c_int == 0 as core::ffi::c_int {
                let mut tmp_0: uint16_t = 0;
                if avifROStreamReadU16(&mut refStream, &mut tmp_0) == 0 {
                    return AVIF_RESULT_BMFF_PARSE_FAILED;
                }
                toID = tmp_0 as uint32_t;
            } else if avifROStreamReadU32(&mut refStream, &mut toID) == 0 {
                return AVIF_RESULT_BMFF_PARSE_FAILED
            }
            let result___1: avifResult = avifCheckItemID(
                b"iref\0" as *const u8 as *const core::ffi::c_char,
                toID,
                diag,
            ) as avifResult;
            if result___1 as core::ffi::c_uint
                != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
            {
                return result___1;
            }
            if memcmp(
                (irefHeader.type_0).as_mut_ptr() as *const core::ffi::c_void,
                b"thmb\0" as *const u8 as *const core::ffi::c_char
                    as *const core::ffi::c_void,
                4 as size_t,
            ) == 0
                || memcmp(
                    (irefHeader.type_0).as_mut_ptr() as *const core::ffi::c_void,
                    b"auxl\0" as *const u8 as *const core::ffi::c_char
                        as *const core::ffi::c_void,
                    4 as size_t,
                ) == 0
                || memcmp(
                    (irefHeader.type_0).as_mut_ptr() as *const core::ffi::c_void,
                    b"cdsc\0" as *const u8 as *const core::ffi::c_char
                        as *const core::ffi::c_void,
                    4 as size_t,
                ) == 0
                || memcmp(
                    (irefHeader.type_0).as_mut_ptr() as *const core::ffi::c_void,
                    b"prem\0" as *const u8 as *const core::ffi::c_char
                        as *const core::ffi::c_void,
                    4 as size_t,
                ) == 0
            {
                let result___2: avifResult = avifAddReference(
                    &mut (*item).references,
                    (irefHeader.type_0).as_mut_ptr() as *const core::ffi::c_char,
                    toID,
                ) as avifResult;
                if result___2 as core::ffi::c_uint
                    != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
                {
                    return result___2;
                }
            } else if memcmp(
                (irefHeader.type_0).as_mut_ptr() as *const core::ffi::c_void,
                b"dimg\0" as *const u8 as *const core::ffi::c_char
                    as *const core::ffi::c_void,
                4 as size_t,
            ) == 0
            {
                let mut dimgInput: *mut avifDecoderItem = 0 as *mut avifDecoderItem;
                let result___3: avifResult = avifMetaFindOrCreateItem(
                    meta,
                    toID,
                    &mut dimgInput,
                ) as avifResult;
                if result___3 as core::ffi::c_uint
                    != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
                {
                    return result___3;
                }
                if avifIsDimgInput(item, toID, 0 as *mut uint32_t) != 0 {
                    return AVIF_RESULT_INVALID_IMAGE_GRID;
                }
                let result___4: avifResult = avifAddDimgInput(item, dimgInput)
                    as avifResult;
                if result___4 as core::ffi::c_uint
                    != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
                {
                    return result___4;
                }
            }
            refIndex = refIndex.wrapping_add(1);
        }
        if avifROStreamSkip(&mut s, irefHeader.size) == 0 {
            return AVIF_RESULT_BMFF_PARSE_FAILED;
        }
    }
    return AVIF_RESULT_OK;
}
unsafe extern "C" fn avifParseGroupsListBox(
    mut meta: *mut avifMeta,
    mut raw: *const uint8_t,
    mut rawLen: size_t,
    mut diag: *mut avifDiagnostics,
) -> avifResult {
    let mut s: avifROStream = avifROStream {
        raw: 0 as *mut avifROData,
        offset: 0,
        numUsedBitsInPartialByte: 0,
        diag: 0 as *mut avifDiagnostics,
        diagContext: 0 as *const core::ffi::c_char,
    };
    let mut s_roData: avifROData = avifROData {
        data: 0 as *const uint8_t,
        size: 0,
    };
    s_roData.data = raw;
    s_roData.size = rawLen;
    avifROStreamStart(
        &mut s,
        &mut s_roData,
        diag,
        b"Box[grpl]\0" as *const u8 as *const core::ffi::c_char,
    );
    while avifROStreamHasBytesLeft(&mut s, 1 as size_t) != 0 {
        let mut groupHeader: avifBoxHeader = avifBoxHeader {
            isSizeZeroBox: 0,
            size: 0,
            type_0: [0; 4],
            usertype: [0; 16],
        };
        if avifROStreamReadBoxHeader(&mut s, &mut groupHeader) == 0 {
            return AVIF_RESULT_BMFF_PARSE_FAILED;
        }
        let mut groupStream: avifROStream = avifROStream {
            raw: 0 as *mut avifROData,
            offset: 0,
            numUsedBitsInPartialByte: 0,
            diag: 0 as *mut avifDiagnostics,
            diagContext: 0 as *const core::ffi::c_char,
        };
        let mut groupStream_roData: avifROData = avifROData {
            data: 0 as *const uint8_t,
            size: 0,
        };
        groupStream_roData.data = avifROStreamCurrent(&mut s);
        groupStream_roData.size = groupHeader.size;
        avifROStreamStart(
            &mut groupStream,
            &mut groupStream_roData,
            diag,
            b"Box[grpl] EntityToGroup\0" as *const u8 as *const core::ffi::c_char,
        );
        if avifROStreamReadVersionAndFlags(
            &mut groupStream,
            0 as *mut uint8_t,
            0 as *mut uint32_t,
        ) == 0
        {
            return AVIF_RESULT_BMFF_PARSE_FAILED;
        }
        let mut group: *mut avifEntityToGroup = avifArrayPush(
            &mut (*meta).entityToGroups as *mut avifEntityToGroups
                as *mut core::ffi::c_void,
        ) as *mut avifEntityToGroup;
        if group.is_null() {
            return AVIF_RESULT_OUT_OF_MEMORY;
        }
        if avifArrayCreate(
            &mut (*group).entityIDs as *mut avifCodecEntityIDs as *mut core::ffi::c_void,
            ::core::mem::size_of::<uint32_t>() as uint32_t,
            2 as uint32_t,
        ) == 0
        {
            return AVIF_RESULT_OUT_OF_MEMORY;
        }
        memcpy(
            ((*group).groupingType).as_mut_ptr() as *mut core::ffi::c_void,
            (groupHeader.type_0).as_mut_ptr() as *const core::ffi::c_void,
            4 as size_t,
        );
        if avifROStreamReadU32(&mut groupStream, &mut (*group).groupID) == 0 {
            return AVIF_RESULT_BMFF_PARSE_FAILED;
        }
        let mut numEntitiesInGroup: uint32_t = 0;
        if avifROStreamReadU32(&mut groupStream, &mut numEntitiesInGroup) == 0 {
            return AVIF_RESULT_BMFF_PARSE_FAILED;
        }
        let mut i: uint32_t = 0 as uint32_t;
        while i < numEntitiesInGroup {
            let mut entityId: *mut uint32_t = avifArrayPush(
                &mut (*group).entityIDs as *mut avifCodecEntityIDs
                    as *mut core::ffi::c_void,
            ) as *mut uint32_t;
            if entityId.is_null() {
                return AVIF_RESULT_OUT_OF_MEMORY;
            }
            if avifROStreamReadU32(&mut groupStream, entityId) == 0 {
                return AVIF_RESULT_BMFF_PARSE_FAILED;
            }
            i = i.wrapping_add(1);
        }
        if avifROStreamSkip(&mut s, groupHeader.size) == 0 {
            return AVIF_RESULT_BMFF_PARSE_FAILED;
        }
    }
    return AVIF_RESULT_OK;
}
unsafe extern "C" fn avifParseMetaBox(
    mut meta: *mut avifMeta,
    mut rawOffset: uint64_t,
    mut raw: *const uint8_t,
    mut rawLen: size_t,
    mut diag: *mut avifDiagnostics,
) -> avifResult {
    let mut s: avifROStream = avifROStream {
        raw: 0 as *mut avifROData,
        offset: 0,
        numUsedBitsInPartialByte: 0,
        diag: 0 as *mut avifDiagnostics,
        diagContext: 0 as *const core::ffi::c_char,
    };
    let mut s_roData: avifROData = avifROData {
        data: 0 as *const uint8_t,
        size: 0,
    };
    s_roData.data = raw;
    s_roData.size = rawLen;
    avifROStreamStart(
        &mut s,
        &mut s_roData,
        diag,
        b"Box[meta]\0" as *const u8 as *const core::ffi::c_char,
    );
    let mut flags: uint32_t = 0;
    if avifROStreamReadAndEnforceVersion(&mut s, 0 as uint8_t, &mut flags) == 0 {
        return AVIF_RESULT_BMFF_PARSE_FAILED;
    }
    (*meta).idatID = ((*meta).idatID).wrapping_add(1);
    let mut firstBox: avifBool = AVIF_TRUE;
    let mut uniqueBoxFlags: uint32_t = 0 as uint32_t;
    while avifROStreamHasBytesLeft(&mut s, 1 as size_t) != 0 {
        let mut header: avifBoxHeader = avifBoxHeader {
            isSizeZeroBox: 0,
            size: 0,
            type_0: [0; 4],
            usertype: [0; 16],
        };
        if avifROStreamReadBoxHeader(&mut s, &mut header) == 0 {
            return AVIF_RESULT_BMFF_PARSE_FAILED;
        }
        if firstBox != 0 {
            if memcmp(
                (header.type_0).as_mut_ptr() as *const core::ffi::c_void,
                b"hdlr\0" as *const u8 as *const core::ffi::c_char
                    as *const core::ffi::c_void,
                4 as size_t,
            ) == 0
            {
                let mut handlerType: [uint8_t; 4] = [0; 4];
                if avifParseHandlerBox(
                    avifROStreamCurrent(&mut s),
                    header.size,
                    handlerType.as_mut_ptr(),
                    diag,
                ) == 0
                {
                    return AVIF_RESULT_BMFF_PARSE_FAILED;
                }
                if memcmp(
                    handlerType.as_mut_ptr() as *const core::ffi::c_void,
                    b"pict\0" as *const u8 as *const core::ffi::c_char
                        as *const core::ffi::c_void,
                    4 as size_t,
                ) != 0 as core::ffi::c_int
                {
                    avifDiagnosticsPrintf(
                        diag,
                        b"Box[hdlr] handler_type is not 'pict'\0" as *const u8
                            as *const core::ffi::c_char,
                    );
                    return AVIF_RESULT_BMFF_PARSE_FAILED;
                }
                firstBox = AVIF_FALSE as avifBool;
            } else {
                avifDiagnosticsPrintf(
                    diag,
                    b"Box[meta] does not have a Box[hdlr] as its first child box\0"
                        as *const u8 as *const core::ffi::c_char,
                );
                return AVIF_RESULT_BMFF_PARSE_FAILED;
            }
        } else if memcmp(
            (header.type_0).as_mut_ptr() as *const core::ffi::c_void,
            b"hdlr\0" as *const u8 as *const core::ffi::c_char
                as *const core::ffi::c_void,
            4 as size_t,
        ) == 0
        {
            avifDiagnosticsPrintf(
                diag,
                b"Box[meta] contains a duplicate unique box of type 'hdlr'\0"
                    as *const u8 as *const core::ffi::c_char,
            );
            return AVIF_RESULT_BMFF_PARSE_FAILED;
        } else if memcmp(
            (header.type_0).as_mut_ptr() as *const core::ffi::c_void,
            b"iloc\0" as *const u8 as *const core::ffi::c_char
                as *const core::ffi::c_void,
            4 as size_t,
        ) == 0
        {
            if uniqueBoxSeen(
                &mut uniqueBoxFlags,
                AVIF_UNIQUE_ILOC,
                b"meta\0" as *const u8 as *const core::ffi::c_char,
                b"iloc\0" as *const u8 as *const core::ffi::c_char,
                diag,
            ) == 0
            {
                return AVIF_RESULT_BMFF_PARSE_FAILED;
            }
            let result__: avifResult = avifParseItemLocationBox(
                meta,
                avifROStreamCurrent(&mut s),
                header.size,
                diag,
            ) as avifResult;
            if result__ as core::ffi::c_uint
                != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
            {
                return result__;
            }
        } else if memcmp(
            (header.type_0).as_mut_ptr() as *const core::ffi::c_void,
            b"pitm\0" as *const u8 as *const core::ffi::c_char
                as *const core::ffi::c_void,
            4 as size_t,
        ) == 0
        {
            if uniqueBoxSeen(
                &mut uniqueBoxFlags,
                AVIF_UNIQUE_PITM,
                b"meta\0" as *const u8 as *const core::ffi::c_char,
                b"pitm\0" as *const u8 as *const core::ffi::c_char,
                diag,
            ) == 0
            {
                return AVIF_RESULT_BMFF_PARSE_FAILED;
            }
            if avifParsePrimaryItemBox(
                meta,
                avifROStreamCurrent(&mut s),
                header.size,
                diag,
            ) == 0
            {
                return AVIF_RESULT_BMFF_PARSE_FAILED;
            }
        } else if memcmp(
            (header.type_0).as_mut_ptr() as *const core::ffi::c_void,
            b"idat\0" as *const u8 as *const core::ffi::c_char
                as *const core::ffi::c_void,
            4 as size_t,
        ) == 0
        {
            if uniqueBoxSeen(
                &mut uniqueBoxFlags,
                AVIF_UNIQUE_IDAT,
                b"meta\0" as *const u8 as *const core::ffi::c_char,
                b"idat\0" as *const u8 as *const core::ffi::c_char,
                diag,
            ) == 0
            {
                return AVIF_RESULT_BMFF_PARSE_FAILED;
            }
            if avifParseItemDataBox(meta, avifROStreamCurrent(&mut s), header.size, diag)
                == 0
            {
                return AVIF_RESULT_BMFF_PARSE_FAILED;
            }
        } else if memcmp(
            (header.type_0).as_mut_ptr() as *const core::ffi::c_void,
            b"iprp\0" as *const u8 as *const core::ffi::c_char
                as *const core::ffi::c_void,
            4 as size_t,
        ) == 0
        {
            if uniqueBoxSeen(
                &mut uniqueBoxFlags,
                AVIF_UNIQUE_IPRP,
                b"meta\0" as *const u8 as *const core::ffi::c_char,
                b"iprp\0" as *const u8 as *const core::ffi::c_char,
                diag,
            ) == 0
            {
                return AVIF_RESULT_BMFF_PARSE_FAILED;
            }
            let result___0: avifResult = avifParseItemPropertiesBox(
                meta,
                rawOffset.wrapping_add(avifROStreamOffset(&mut s) as uint64_t),
                avifROStreamCurrent(&mut s),
                header.size,
                diag,
            ) as avifResult;
            if result___0 as core::ffi::c_uint
                != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
            {
                return result___0;
            }
        } else if memcmp(
            (header.type_0).as_mut_ptr() as *const core::ffi::c_void,
            b"iinf\0" as *const u8 as *const core::ffi::c_char
                as *const core::ffi::c_void,
            4 as size_t,
        ) == 0
        {
            if uniqueBoxSeen(
                &mut uniqueBoxFlags,
                AVIF_UNIQUE_IINF,
                b"meta\0" as *const u8 as *const core::ffi::c_char,
                b"iinf\0" as *const u8 as *const core::ffi::c_char,
                diag,
            ) == 0
            {
                return AVIF_RESULT_BMFF_PARSE_FAILED;
            }
            let result___1: avifResult = avifParseItemInfoBox(
                meta,
                avifROStreamCurrent(&mut s),
                header.size,
                diag,
            ) as avifResult;
            if result___1 as core::ffi::c_uint
                != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
            {
                return result___1;
            }
        } else if memcmp(
            (header.type_0).as_mut_ptr() as *const core::ffi::c_void,
            b"iref\0" as *const u8 as *const core::ffi::c_char
                as *const core::ffi::c_void,
            4 as size_t,
        ) == 0
        {
            if uniqueBoxSeen(
                &mut uniqueBoxFlags,
                AVIF_UNIQUE_IREF,
                b"meta\0" as *const u8 as *const core::ffi::c_char,
                b"iref\0" as *const u8 as *const core::ffi::c_char,
                diag,
            ) == 0
            {
                return AVIF_RESULT_BMFF_PARSE_FAILED;
            }
            let result___2: avifResult = avifParseItemReferenceBox(
                meta,
                avifROStreamCurrent(&mut s),
                header.size,
                diag,
            ) as avifResult;
            if result___2 as core::ffi::c_uint
                != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
            {
                return result___2;
            }
        } else if memcmp(
            (header.type_0).as_mut_ptr() as *const core::ffi::c_void,
            b"grpl\0" as *const u8 as *const core::ffi::c_char
                as *const core::ffi::c_void,
            4 as size_t,
        ) == 0
        {
            if uniqueBoxSeen(
                &mut uniqueBoxFlags,
                AVIF_UNIQUE_GRPL,
                b"meta\0" as *const u8 as *const core::ffi::c_char,
                b"grpl\0" as *const u8 as *const core::ffi::c_char,
                diag,
            ) == 0
            {
                return AVIF_RESULT_BMFF_PARSE_FAILED;
            }
            let result___3: avifResult = avifParseGroupsListBox(
                meta,
                avifROStreamCurrent(&mut s),
                header.size,
                diag,
            ) as avifResult;
            if result___3 as core::ffi::c_uint
                != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
            {
                return result___3;
            }
        }
        if avifROStreamSkip(&mut s, header.size) == 0 {
            return AVIF_RESULT_BMFF_PARSE_FAILED;
        }
    }
    if firstBox != 0 {
        avifDiagnosticsPrintf(
            diag,
            b"Box[meta] has no child boxes\0" as *const u8 as *const core::ffi::c_char,
        );
        return AVIF_RESULT_BMFF_PARSE_FAILED;
    }
    return AVIF_RESULT_OK;
}
unsafe extern "C" fn avifParseTrackHeaderBox(
    mut track: *mut avifTrack,
    mut raw: *const uint8_t,
    mut rawLen: size_t,
    mut diag: *mut avifDiagnostics,
) -> avifBool {
    let mut s: avifROStream = avifROStream {
        raw: 0 as *mut avifROData,
        offset: 0,
        numUsedBitsInPartialByte: 0,
        diag: 0 as *mut avifDiagnostics,
        diagContext: 0 as *const core::ffi::c_char,
    };
    let mut s_roData: avifROData = avifROData {
        data: 0 as *const uint8_t,
        size: 0,
    };
    s_roData.data = raw;
    s_roData.size = rawLen;
    avifROStreamStart(
        &mut s,
        &mut s_roData,
        diag,
        b"Box[tkhd]\0" as *const u8 as *const core::ffi::c_char,
    );
    let mut version: uint8_t = 0;
    if avifROStreamReadVersionAndFlags(&mut s, &mut version, 0 as *mut uint32_t) == 0 {
        return AVIF_FALSE;
    }
    let mut ignored32: uint32_t = 0;
    let mut trackID: uint32_t = 0;
    let mut ignored64: uint64_t = 0;
    if version as core::ffi::c_int == 1 as core::ffi::c_int {
        if avifROStreamReadU64(&mut s, &mut ignored64) == 0 {
            return AVIF_FALSE;
        }
        if avifROStreamReadU64(&mut s, &mut ignored64) == 0 {
            return AVIF_FALSE;
        }
        if avifROStreamReadU32(&mut s, &mut trackID) == 0 {
            return AVIF_FALSE;
        }
        if avifROStreamReadU32(&mut s, &mut ignored32) == 0 {
            return AVIF_FALSE;
        }
        if avifROStreamReadU64(&mut s, &mut (*track).trackDuration) == 0 {
            return AVIF_FALSE;
        }
    } else if version as core::ffi::c_int == 0 as core::ffi::c_int {
        let mut trackDuration: uint32_t = 0;
        if avifROStreamReadU32(&mut s, &mut ignored32) == 0 {
            return AVIF_FALSE;
        }
        if avifROStreamReadU32(&mut s, &mut ignored32) == 0 {
            return AVIF_FALSE;
        }
        if avifROStreamReadU32(&mut s, &mut trackID) == 0 {
            return AVIF_FALSE;
        }
        if avifROStreamReadU32(&mut s, &mut ignored32) == 0 {
            return AVIF_FALSE;
        }
        if avifROStreamReadU32(&mut s, &mut trackDuration) == 0 {
            return AVIF_FALSE;
        }
        (*track).trackDuration = (if trackDuration
            == AVIF_INDEFINITE_DURATION32 as uint32_t
        {
            AVIF_INDEFINITE_DURATION64
        } else {
            trackDuration as core::ffi::c_ulong
        }) as uint64_t;
    } else {
        avifDiagnosticsPrintf(
            diag,
            b"Box[tkhd] has an unsupported version [%u]\0" as *const u8
                as *const core::ffi::c_char,
            version as core::ffi::c_int,
        );
        return AVIF_FALSE;
    }
    (*track).id = trackID;
    if avifROStreamSkip(&mut s, 52 as size_t) == 0 {
        return AVIF_FALSE;
    }
    let mut width: uint32_t = 0;
    let mut height: uint32_t = 0;
    if avifROStreamReadU32(&mut s, &mut width) == 0 {
        return AVIF_FALSE;
    }
    if avifROStreamReadU32(&mut s, &mut height) == 0 {
        return AVIF_FALSE;
    }
    (*track).width = width >> 16 as core::ffi::c_int;
    (*track).height = height >> 16 as core::ffi::c_int;
    return AVIF_TRUE;
}
unsafe extern "C" fn avifParseMediaHeaderBox(
    mut track: *mut avifTrack,
    mut raw: *const uint8_t,
    mut rawLen: size_t,
    mut diag: *mut avifDiagnostics,
) -> avifBool {
    let mut s: avifROStream = avifROStream {
        raw: 0 as *mut avifROData,
        offset: 0,
        numUsedBitsInPartialByte: 0,
        diag: 0 as *mut avifDiagnostics,
        diagContext: 0 as *const core::ffi::c_char,
    };
    let mut s_roData: avifROData = avifROData {
        data: 0 as *const uint8_t,
        size: 0,
    };
    s_roData.data = raw;
    s_roData.size = rawLen;
    avifROStreamStart(
        &mut s,
        &mut s_roData,
        diag,
        b"Box[mdhd]\0" as *const u8 as *const core::ffi::c_char,
    );
    let mut version: uint8_t = 0;
    if avifROStreamReadVersionAndFlags(&mut s, &mut version, 0 as *mut uint32_t) == 0 {
        return AVIF_FALSE;
    }
    let mut ignored32: uint32_t = 0;
    let mut mediaTimescale: uint32_t = 0;
    let mut mediaDuration32: uint32_t = 0;
    let mut ignored64: uint64_t = 0;
    let mut mediaDuration64: uint64_t = 0;
    if version as core::ffi::c_int == 1 as core::ffi::c_int {
        if avifROStreamReadU64(&mut s, &mut ignored64) == 0 {
            return AVIF_FALSE;
        }
        if avifROStreamReadU64(&mut s, &mut ignored64) == 0 {
            return AVIF_FALSE;
        }
        if avifROStreamReadU32(&mut s, &mut mediaTimescale) == 0 {
            return AVIF_FALSE;
        }
        if avifROStreamReadU64(&mut s, &mut mediaDuration64) == 0 {
            return AVIF_FALSE;
        }
        (*track).mediaDuration = mediaDuration64;
    } else if version as core::ffi::c_int == 0 as core::ffi::c_int {
        if avifROStreamReadU32(&mut s, &mut ignored32) == 0 {
            return AVIF_FALSE;
        }
        if avifROStreamReadU32(&mut s, &mut ignored32) == 0 {
            return AVIF_FALSE;
        }
        if avifROStreamReadU32(&mut s, &mut mediaTimescale) == 0 {
            return AVIF_FALSE;
        }
        if avifROStreamReadU32(&mut s, &mut mediaDuration32) == 0 {
            return AVIF_FALSE;
        }
        (*track).mediaDuration = mediaDuration32 as uint64_t;
    } else {
        avifDiagnosticsPrintf(
            diag,
            b"Box[mdhd] has an unsupported version [%u]\0" as *const u8
                as *const core::ffi::c_char,
            version as core::ffi::c_int,
        );
        return AVIF_FALSE;
    }
    (*track).mediaTimescale = mediaTimescale;
    return AVIF_TRUE;
}
unsafe extern "C" fn avifParseChunkOffsetBox(
    mut sampleTable: *mut avifSampleTable,
    mut largeOffsets: avifBool,
    mut raw: *const uint8_t,
    mut rawLen: size_t,
    mut diag: *mut avifDiagnostics,
) -> avifResult {
    let mut s: avifROStream = avifROStream {
        raw: 0 as *mut avifROData,
        offset: 0,
        numUsedBitsInPartialByte: 0,
        diag: 0 as *mut avifDiagnostics,
        diagContext: 0 as *const core::ffi::c_char,
    };
    let mut s_roData: avifROData = avifROData {
        data: 0 as *const uint8_t,
        size: 0,
    };
    s_roData.data = raw;
    s_roData.size = rawLen;
    avifROStreamStart(
        &mut s,
        &mut s_roData,
        diag,
        if largeOffsets != 0 {
            b"Box[co64]\0" as *const u8 as *const core::ffi::c_char
        } else {
            b"Box[stco]\0" as *const u8 as *const core::ffi::c_char
        },
    );
    if avifROStreamReadAndEnforceVersion(&mut s, 0 as uint8_t, 0 as *mut uint32_t) == 0 {
        return AVIF_RESULT_BMFF_PARSE_FAILED;
    }
    let mut entryCount: uint32_t = 0;
    if avifROStreamReadU32(&mut s, &mut entryCount) == 0 {
        return AVIF_RESULT_BMFF_PARSE_FAILED;
    }
    let mut i: uint32_t = 0 as uint32_t;
    while i < entryCount {
        let mut offset: uint64_t = 0;
        if largeOffsets != 0 {
            if avifROStreamReadU64(&mut s, &mut offset) == 0 {
                return AVIF_RESULT_BMFF_PARSE_FAILED;
            }
        } else {
            let mut offset32: uint32_t = 0;
            if avifROStreamReadU32(&mut s, &mut offset32) == 0 {
                return AVIF_RESULT_BMFF_PARSE_FAILED;
            }
            offset = offset32 as uint64_t;
        }
        let mut chunk: *mut avifSampleTableChunk = avifArrayPush(
            &mut (*sampleTable).chunks as *mut avifSampleTableChunkArray
                as *mut core::ffi::c_void,
        ) as *mut avifSampleTableChunk;
        if chunk.is_null() {
            return AVIF_RESULT_OUT_OF_MEMORY;
        }
        (*chunk).offset = offset;
        i = i.wrapping_add(1);
    }
    return AVIF_RESULT_OK;
}
unsafe extern "C" fn avifParseSampleToChunkBox(
    mut sampleTable: *mut avifSampleTable,
    mut raw: *const uint8_t,
    mut rawLen: size_t,
    mut diag: *mut avifDiagnostics,
) -> avifResult {
    let mut s: avifROStream = avifROStream {
        raw: 0 as *mut avifROData,
        offset: 0,
        numUsedBitsInPartialByte: 0,
        diag: 0 as *mut avifDiagnostics,
        diagContext: 0 as *const core::ffi::c_char,
    };
    let mut s_roData: avifROData = avifROData {
        data: 0 as *const uint8_t,
        size: 0,
    };
    s_roData.data = raw;
    s_roData.size = rawLen;
    avifROStreamStart(
        &mut s,
        &mut s_roData,
        diag,
        b"Box[stsc]\0" as *const u8 as *const core::ffi::c_char,
    );
    if avifROStreamReadAndEnforceVersion(&mut s, 0 as uint8_t, 0 as *mut uint32_t) == 0 {
        return AVIF_RESULT_BMFF_PARSE_FAILED;
    }
    let mut entryCount: uint32_t = 0;
    if avifROStreamReadU32(&mut s, &mut entryCount) == 0 {
        return AVIF_RESULT_BMFF_PARSE_FAILED;
    }
    let mut prevFirstChunk: uint32_t = 0 as uint32_t;
    let mut i: uint32_t = 0 as uint32_t;
    while i < entryCount {
        let mut sampleToChunk: *mut avifSampleTableSampleToChunk = avifArrayPush(
            &mut (*sampleTable).sampleToChunks as *mut avifSampleTableSampleToChunkArray
                as *mut core::ffi::c_void,
        ) as *mut avifSampleTableSampleToChunk;
        if sampleToChunk.is_null() {
            return AVIF_RESULT_OUT_OF_MEMORY;
        }
        if avifROStreamReadU32(&mut s, &mut (*sampleToChunk).firstChunk) == 0 {
            return AVIF_RESULT_BMFF_PARSE_FAILED;
        }
        if avifROStreamReadU32(&mut s, &mut (*sampleToChunk).samplesPerChunk) == 0 {
            return AVIF_RESULT_BMFF_PARSE_FAILED;
        }
        if avifROStreamReadU32(&mut s, &mut (*sampleToChunk).sampleDescriptionIndex) == 0
        {
            return AVIF_RESULT_BMFF_PARSE_FAILED;
        }
        if i == 0 as uint32_t {
            if (*sampleToChunk).firstChunk != 1 as uint32_t {
                avifDiagnosticsPrintf(
                    diag,
                    b"Box[stsc] does not begin with chunk 1 [%u]\0" as *const u8
                        as *const core::ffi::c_char,
                    (*sampleToChunk).firstChunk,
                );
                return AVIF_RESULT_BMFF_PARSE_FAILED;
            }
        } else if (*sampleToChunk).firstChunk <= prevFirstChunk {
            avifDiagnosticsPrintf(
                diag,
                b"Box[stsc] chunks are not strictly increasing\0" as *const u8
                    as *const core::ffi::c_char,
            );
            return AVIF_RESULT_BMFF_PARSE_FAILED;
        }
        prevFirstChunk = (*sampleToChunk).firstChunk;
        i = i.wrapping_add(1);
    }
    return AVIF_RESULT_OK;
}
unsafe extern "C" fn avifParseSampleSizeBox(
    mut sampleTable: *mut avifSampleTable,
    mut raw: *const uint8_t,
    mut rawLen: size_t,
    mut diag: *mut avifDiagnostics,
) -> avifResult {
    let mut s: avifROStream = avifROStream {
        raw: 0 as *mut avifROData,
        offset: 0,
        numUsedBitsInPartialByte: 0,
        diag: 0 as *mut avifDiagnostics,
        diagContext: 0 as *const core::ffi::c_char,
    };
    let mut s_roData: avifROData = avifROData {
        data: 0 as *const uint8_t,
        size: 0,
    };
    s_roData.data = raw;
    s_roData.size = rawLen;
    avifROStreamStart(
        &mut s,
        &mut s_roData,
        diag,
        b"Box[stsz]\0" as *const u8 as *const core::ffi::c_char,
    );
    if avifROStreamReadAndEnforceVersion(&mut s, 0 as uint8_t, 0 as *mut uint32_t) == 0 {
        return AVIF_RESULT_BMFF_PARSE_FAILED;
    }
    let mut allSamplesSize: uint32_t = 0;
    let mut sampleCount: uint32_t = 0;
    if avifROStreamReadU32(&mut s, &mut allSamplesSize) == 0 {
        return AVIF_RESULT_BMFF_PARSE_FAILED;
    }
    if avifROStreamReadU32(&mut s, &mut sampleCount) == 0 {
        return AVIF_RESULT_BMFF_PARSE_FAILED;
    }
    if allSamplesSize > 0 as uint32_t {
        (*sampleTable).allSamplesSize = allSamplesSize;
    } else {
        let mut i: uint32_t = 0 as uint32_t;
        while i < sampleCount {
            let mut sampleSize: *mut avifSampleTableSampleSize = avifArrayPush(
                &mut (*sampleTable).sampleSizes as *mut avifSampleTableSampleSizeArray
                    as *mut core::ffi::c_void,
            ) as *mut avifSampleTableSampleSize;
            if sampleSize.is_null() {
                return AVIF_RESULT_OUT_OF_MEMORY;
            }
            if avifROStreamReadU32(&mut s, &mut (*sampleSize).size) == 0 {
                return AVIF_RESULT_BMFF_PARSE_FAILED;
            }
            i = i.wrapping_add(1);
        }
    }
    return AVIF_RESULT_OK;
}
unsafe extern "C" fn avifParseSyncSampleBox(
    mut sampleTable: *mut avifSampleTable,
    mut raw: *const uint8_t,
    mut rawLen: size_t,
    mut diag: *mut avifDiagnostics,
) -> avifResult {
    let mut s: avifROStream = avifROStream {
        raw: 0 as *mut avifROData,
        offset: 0,
        numUsedBitsInPartialByte: 0,
        diag: 0 as *mut avifDiagnostics,
        diagContext: 0 as *const core::ffi::c_char,
    };
    let mut s_roData: avifROData = avifROData {
        data: 0 as *const uint8_t,
        size: 0,
    };
    s_roData.data = raw;
    s_roData.size = rawLen;
    avifROStreamStart(
        &mut s,
        &mut s_roData,
        diag,
        b"Box[stss]\0" as *const u8 as *const core::ffi::c_char,
    );
    if avifROStreamReadAndEnforceVersion(&mut s, 0 as uint8_t, 0 as *mut uint32_t) == 0 {
        return AVIF_RESULT_BMFF_PARSE_FAILED;
    }
    let mut entryCount: uint32_t = 0;
    if avifROStreamReadU32(&mut s, &mut entryCount) == 0 {
        return AVIF_RESULT_BMFF_PARSE_FAILED;
    }
    let mut i: uint32_t = 0 as uint32_t;
    while i < entryCount {
        let mut sampleNumber: uint32_t = 0 as uint32_t;
        if avifROStreamReadU32(&mut s, &mut sampleNumber) == 0 {
            return AVIF_RESULT_BMFF_PARSE_FAILED;
        }
        let mut syncSample: *mut avifSyncSample = avifArrayPush(
            &mut (*sampleTable).syncSamples as *mut avifSyncSampleArray
                as *mut core::ffi::c_void,
        ) as *mut avifSyncSample;
        if syncSample.is_null() {
            return AVIF_RESULT_OUT_OF_MEMORY;
        }
        (*syncSample).sampleNumber = sampleNumber;
        i = i.wrapping_add(1);
    }
    return AVIF_RESULT_OK;
}
unsafe extern "C" fn avifParseTimeToSampleBox(
    mut sampleTable: *mut avifSampleTable,
    mut raw: *const uint8_t,
    mut rawLen: size_t,
    mut diag: *mut avifDiagnostics,
) -> avifResult {
    let mut s: avifROStream = avifROStream {
        raw: 0 as *mut avifROData,
        offset: 0,
        numUsedBitsInPartialByte: 0,
        diag: 0 as *mut avifDiagnostics,
        diagContext: 0 as *const core::ffi::c_char,
    };
    let mut s_roData: avifROData = avifROData {
        data: 0 as *const uint8_t,
        size: 0,
    };
    s_roData.data = raw;
    s_roData.size = rawLen;
    avifROStreamStart(
        &mut s,
        &mut s_roData,
        diag,
        b"Box[stts]\0" as *const u8 as *const core::ffi::c_char,
    );
    if avifROStreamReadAndEnforceVersion(&mut s, 0 as uint8_t, 0 as *mut uint32_t) == 0 {
        return AVIF_RESULT_BMFF_PARSE_FAILED;
    }
    let mut entryCount: uint32_t = 0;
    if avifROStreamReadU32(&mut s, &mut entryCount) == 0 {
        return AVIF_RESULT_BMFF_PARSE_FAILED;
    }
    let mut i: uint32_t = 0 as uint32_t;
    while i < entryCount {
        let mut timeToSample: *mut avifSampleTableTimeToSample = avifArrayPush(
            &mut (*sampleTable).timeToSamples as *mut avifSampleTableTimeToSampleArray
                as *mut core::ffi::c_void,
        ) as *mut avifSampleTableTimeToSample;
        if timeToSample.is_null() {
            return AVIF_RESULT_OUT_OF_MEMORY;
        }
        if avifROStreamReadU32(&mut s, &mut (*timeToSample).sampleCount) == 0 {
            return AVIF_RESULT_BMFF_PARSE_FAILED;
        }
        if avifROStreamReadU32(&mut s, &mut (*timeToSample).sampleDelta) == 0 {
            return AVIF_RESULT_BMFF_PARSE_FAILED;
        }
        i = i.wrapping_add(1);
    }
    return AVIF_RESULT_OK;
}
unsafe extern "C" fn avifParseSampleDescriptionBox(
    mut sampleTable: *mut avifSampleTable,
    mut rawOffset: uint64_t,
    mut raw: *const uint8_t,
    mut rawLen: size_t,
    mut diag: *mut avifDiagnostics,
) -> avifResult {
    let mut s: avifROStream = avifROStream {
        raw: 0 as *mut avifROData,
        offset: 0,
        numUsedBitsInPartialByte: 0,
        diag: 0 as *mut avifDiagnostics,
        diagContext: 0 as *const core::ffi::c_char,
    };
    let mut s_roData: avifROData = avifROData {
        data: 0 as *const uint8_t,
        size: 0,
    };
    s_roData.data = raw;
    s_roData.size = rawLen;
    avifROStreamStart(
        &mut s,
        &mut s_roData,
        diag,
        b"Box[stsd]\0" as *const u8 as *const core::ffi::c_char,
    );
    let mut version: uint8_t = 0;
    if avifROStreamReadVersionAndFlags(&mut s, &mut version, 0 as *mut uint32_t) == 0 {
        return AVIF_RESULT_BMFF_PARSE_FAILED;
    }
    if version as core::ffi::c_int != 0 as core::ffi::c_int
        && version as core::ffi::c_int != 1 as core::ffi::c_int
    {
        avifDiagnosticsPrintf(
            diag,
            b"Box[stsd]: Expecting box version 0 or 1, got version %u\0" as *const u8
                as *const core::ffi::c_char,
            version as core::ffi::c_int,
        );
        return AVIF_RESULT_BMFF_PARSE_FAILED;
    }
    let mut entryCount: uint32_t = 0;
    if avifROStreamReadU32(&mut s, &mut entryCount) == 0 {
        return AVIF_RESULT_BMFF_PARSE_FAILED;
    }
    let mut i: uint32_t = 0 as uint32_t;
    while i < entryCount {
        let mut sampleEntryHeader: avifBoxHeader = avifBoxHeader {
            isSizeZeroBox: 0,
            size: 0,
            type_0: [0; 4],
            usertype: [0; 16],
        };
        if avifROStreamReadBoxHeader(&mut s, &mut sampleEntryHeader) == 0 {
            return AVIF_RESULT_BMFF_PARSE_FAILED;
        }
        let mut description: *mut avifSampleDescription = avifArrayPush(
            &mut (*sampleTable).sampleDescriptions as *mut avifSampleDescriptionArray
                as *mut core::ffi::c_void,
        ) as *mut avifSampleDescription;
        if description.is_null() {
            return AVIF_RESULT_OUT_OF_MEMORY;
        }
        if avifArrayCreate(
            &mut (*description).properties as *mut avifPropertyArray
                as *mut core::ffi::c_void,
            ::core::mem::size_of::<avifProperty>() as uint32_t,
            16 as uint32_t,
        ) == 0
        {
            avifArrayPop(
                &mut (*sampleTable).sampleDescriptions as *mut avifSampleDescriptionArray
                    as *mut core::ffi::c_void,
            );
            return AVIF_RESULT_OUT_OF_MEMORY;
        }
        memcpy(
            ((*description).format).as_mut_ptr() as *mut core::ffi::c_void,
            (sampleEntryHeader.type_0).as_mut_ptr() as *const core::ffi::c_void,
            ::core::mem::size_of::<[uint8_t; 4]>() as size_t,
        );
        let sampleEntryBytes: size_t = sampleEntryHeader.size;
        if avifGetCodecType(((*description).format).as_mut_ptr()) as core::ffi::c_uint
            != AVIF_CODEC_TYPE_UNKNOWN as core::ffi::c_int as core::ffi::c_uint
        {
            if sampleEntryBytes < VISUALSAMPLEENTRY_SIZE {
                avifDiagnosticsPrintf(
                    diag,
                    b"Not enough bytes to parse VisualSampleEntry\0" as *const u8
                        as *const core::ffi::c_char,
                );
                return AVIF_RESULT_BMFF_PARSE_FAILED;
            }
            let result__: avifResult = avifParseItemPropertyContainerBox(
                &mut (*description).properties,
                rawOffset
                    .wrapping_add(avifROStreamOffset(&mut s) as uint64_t)
                    .wrapping_add(VISUALSAMPLEENTRY_SIZE as uint64_t),
                (avifROStreamCurrent(&mut s)).offset(VISUALSAMPLEENTRY_SIZE as isize),
                sampleEntryBytes.wrapping_sub(VISUALSAMPLEENTRY_SIZE),
                1 as avifBool,
                diag,
            ) as avifResult;
            if result__ as core::ffi::c_uint
                != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
            {
                return result__;
            }
        }
        if avifROStreamSkip(&mut s, sampleEntryBytes) == 0 {
            return AVIF_RESULT_BMFF_PARSE_FAILED;
        }
        i = i.wrapping_add(1);
    }
    return AVIF_RESULT_OK;
}
unsafe extern "C" fn avifParseSampleTableBox(
    mut track: *mut avifTrack,
    mut rawOffset: uint64_t,
    mut raw: *const uint8_t,
    mut rawLen: size_t,
    mut diag: *mut avifDiagnostics,
) -> avifResult {
    if !((*track).sampleTable).is_null() {
        avifDiagnosticsPrintf(
            diag,
            b"Duplicate Box[stbl] for a single track detected\0" as *const u8
                as *const core::ffi::c_char,
        );
        return AVIF_RESULT_BMFF_PARSE_FAILED;
    }
    (*track).sampleTable = avifSampleTableCreate();
    if ((*track).sampleTable).is_null() {
        return AVIF_RESULT_OUT_OF_MEMORY;
    }
    let mut s: avifROStream = avifROStream {
        raw: 0 as *mut avifROData,
        offset: 0,
        numUsedBitsInPartialByte: 0,
        diag: 0 as *mut avifDiagnostics,
        diagContext: 0 as *const core::ffi::c_char,
    };
    let mut s_roData: avifROData = avifROData {
        data: 0 as *const uint8_t,
        size: 0,
    };
    s_roData.data = raw;
    s_roData.size = rawLen;
    avifROStreamStart(
        &mut s,
        &mut s_roData,
        diag,
        b"Box[stbl]\0" as *const u8 as *const core::ffi::c_char,
    );
    while avifROStreamHasBytesLeft(&mut s, 1 as size_t) != 0 {
        let mut header: avifBoxHeader = avifBoxHeader {
            isSizeZeroBox: 0,
            size: 0,
            type_0: [0; 4],
            usertype: [0; 16],
        };
        if avifROStreamReadBoxHeader(&mut s, &mut header) == 0 {
            return AVIF_RESULT_BMFF_PARSE_FAILED;
        }
        if memcmp(
            (header.type_0).as_mut_ptr() as *const core::ffi::c_void,
            b"stco\0" as *const u8 as *const core::ffi::c_char
                as *const core::ffi::c_void,
            4 as size_t,
        ) == 0
        {
            let result__: avifResult = avifParseChunkOffsetBox(
                (*track).sampleTable,
                0 as avifBool,
                avifROStreamCurrent(&mut s),
                header.size,
                diag,
            ) as avifResult;
            if result__ as core::ffi::c_uint
                != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
            {
                return result__;
            }
        } else if memcmp(
            (header.type_0).as_mut_ptr() as *const core::ffi::c_void,
            b"co64\0" as *const u8 as *const core::ffi::c_char
                as *const core::ffi::c_void,
            4 as size_t,
        ) == 0
        {
            let result___0: avifResult = avifParseChunkOffsetBox(
                (*track).sampleTable,
                1 as avifBool,
                avifROStreamCurrent(&mut s),
                header.size,
                diag,
            ) as avifResult;
            if result___0 as core::ffi::c_uint
                != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
            {
                return result___0;
            }
        } else if memcmp(
            (header.type_0).as_mut_ptr() as *const core::ffi::c_void,
            b"stsc\0" as *const u8 as *const core::ffi::c_char
                as *const core::ffi::c_void,
            4 as size_t,
        ) == 0
        {
            let result___1: avifResult = avifParseSampleToChunkBox(
                (*track).sampleTable,
                avifROStreamCurrent(&mut s),
                header.size,
                diag,
            ) as avifResult;
            if result___1 as core::ffi::c_uint
                != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
            {
                return result___1;
            }
        } else if memcmp(
            (header.type_0).as_mut_ptr() as *const core::ffi::c_void,
            b"stsz\0" as *const u8 as *const core::ffi::c_char
                as *const core::ffi::c_void,
            4 as size_t,
        ) == 0
        {
            let result___2: avifResult = avifParseSampleSizeBox(
                (*track).sampleTable,
                avifROStreamCurrent(&mut s),
                header.size,
                diag,
            ) as avifResult;
            if result___2 as core::ffi::c_uint
                != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
            {
                return result___2;
            }
        } else if memcmp(
            (header.type_0).as_mut_ptr() as *const core::ffi::c_void,
            b"stss\0" as *const u8 as *const core::ffi::c_char
                as *const core::ffi::c_void,
            4 as size_t,
        ) == 0
        {
            let result___3: avifResult = avifParseSyncSampleBox(
                (*track).sampleTable,
                avifROStreamCurrent(&mut s),
                header.size,
                diag,
            ) as avifResult;
            if result___3 as core::ffi::c_uint
                != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
            {
                return result___3;
            }
        } else if memcmp(
            (header.type_0).as_mut_ptr() as *const core::ffi::c_void,
            b"stts\0" as *const u8 as *const core::ffi::c_char
                as *const core::ffi::c_void,
            4 as size_t,
        ) == 0
        {
            let result___4: avifResult = avifParseTimeToSampleBox(
                (*track).sampleTable,
                avifROStreamCurrent(&mut s),
                header.size,
                diag,
            ) as avifResult;
            if result___4 as core::ffi::c_uint
                != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
            {
                return result___4;
            }
        } else if memcmp(
            (header.type_0).as_mut_ptr() as *const core::ffi::c_void,
            b"stsd\0" as *const u8 as *const core::ffi::c_char
                as *const core::ffi::c_void,
            4 as size_t,
        ) == 0
        {
            let result___5: avifResult = avifParseSampleDescriptionBox(
                (*track).sampleTable,
                rawOffset.wrapping_add(avifROStreamOffset(&mut s) as uint64_t),
                avifROStreamCurrent(&mut s),
                header.size,
                diag,
            ) as avifResult;
            if result___5 as core::ffi::c_uint
                != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
            {
                return result___5;
            }
        }
        if avifROStreamSkip(&mut s, header.size) == 0 {
            return AVIF_RESULT_BMFF_PARSE_FAILED;
        }
    }
    return AVIF_RESULT_OK;
}
unsafe extern "C" fn avifParseMediaInformationBox(
    mut track: *mut avifTrack,
    mut rawOffset: uint64_t,
    mut raw: *const uint8_t,
    mut rawLen: size_t,
    mut diag: *mut avifDiagnostics,
) -> avifResult {
    let mut s: avifROStream = avifROStream {
        raw: 0 as *mut avifROData,
        offset: 0,
        numUsedBitsInPartialByte: 0,
        diag: 0 as *mut avifDiagnostics,
        diagContext: 0 as *const core::ffi::c_char,
    };
    let mut s_roData: avifROData = avifROData {
        data: 0 as *const uint8_t,
        size: 0,
    };
    s_roData.data = raw;
    s_roData.size = rawLen;
    avifROStreamStart(
        &mut s,
        &mut s_roData,
        diag,
        b"Box[minf]\0" as *const u8 as *const core::ffi::c_char,
    );
    while avifROStreamHasBytesLeft(&mut s, 1 as size_t) != 0 {
        let mut header: avifBoxHeader = avifBoxHeader {
            isSizeZeroBox: 0,
            size: 0,
            type_0: [0; 4],
            usertype: [0; 16],
        };
        if avifROStreamReadBoxHeader(&mut s, &mut header) == 0 {
            return AVIF_RESULT_BMFF_PARSE_FAILED;
        }
        if memcmp(
            (header.type_0).as_mut_ptr() as *const core::ffi::c_void,
            b"stbl\0" as *const u8 as *const core::ffi::c_char
                as *const core::ffi::c_void,
            4 as size_t,
        ) == 0
        {
            let result__: avifResult = avifParseSampleTableBox(
                track,
                rawOffset.wrapping_add(avifROStreamOffset(&mut s) as uint64_t),
                avifROStreamCurrent(&mut s),
                header.size,
                diag,
            ) as avifResult;
            if result__ as core::ffi::c_uint
                != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
            {
                return result__;
            }
        }
        if avifROStreamSkip(&mut s, header.size) == 0 {
            return AVIF_RESULT_BMFF_PARSE_FAILED;
        }
    }
    return AVIF_RESULT_OK;
}
unsafe extern "C" fn avifParseMediaBox(
    mut track: *mut avifTrack,
    mut rawOffset: uint64_t,
    mut raw: *const uint8_t,
    mut rawLen: size_t,
    mut diag: *mut avifDiagnostics,
) -> avifResult {
    let mut s: avifROStream = avifROStream {
        raw: 0 as *mut avifROData,
        offset: 0,
        numUsedBitsInPartialByte: 0,
        diag: 0 as *mut avifDiagnostics,
        diagContext: 0 as *const core::ffi::c_char,
    };
    let mut s_roData: avifROData = avifROData {
        data: 0 as *const uint8_t,
        size: 0,
    };
    s_roData.data = raw;
    s_roData.size = rawLen;
    avifROStreamStart(
        &mut s,
        &mut s_roData,
        diag,
        b"Box[mdia]\0" as *const u8 as *const core::ffi::c_char,
    );
    while avifROStreamHasBytesLeft(&mut s, 1 as size_t) != 0 {
        let mut header: avifBoxHeader = avifBoxHeader {
            isSizeZeroBox: 0,
            size: 0,
            type_0: [0; 4],
            usertype: [0; 16],
        };
        if avifROStreamReadBoxHeader(&mut s, &mut header) == 0 {
            return AVIF_RESULT_BMFF_PARSE_FAILED;
        }
        if memcmp(
            (header.type_0).as_mut_ptr() as *const core::ffi::c_void,
            b"mdhd\0" as *const u8 as *const core::ffi::c_char
                as *const core::ffi::c_void,
            4 as size_t,
        ) == 0
        {
            if avifParseMediaHeaderBox(
                track,
                avifROStreamCurrent(&mut s),
                header.size,
                diag,
            ) == 0
            {
                return AVIF_RESULT_BMFF_PARSE_FAILED;
            }
        } else if memcmp(
            (header.type_0).as_mut_ptr() as *const core::ffi::c_void,
            b"minf\0" as *const u8 as *const core::ffi::c_char
                as *const core::ffi::c_void,
            4 as size_t,
        ) == 0
        {
            let result__: avifResult = avifParseMediaInformationBox(
                track,
                rawOffset.wrapping_add(avifROStreamOffset(&mut s) as uint64_t),
                avifROStreamCurrent(&mut s),
                header.size,
                diag,
            ) as avifResult;
            if result__ as core::ffi::c_uint
                != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
            {
                return result__;
            }
        } else if memcmp(
            (header.type_0).as_mut_ptr() as *const core::ffi::c_void,
            b"hdlr\0" as *const u8 as *const core::ffi::c_char
                as *const core::ffi::c_void,
            4 as size_t,
        ) == 0
        {
            if avifParseHandlerBox(
                avifROStreamCurrent(&mut s),
                header.size,
                ((*track).handlerType).as_mut_ptr(),
                diag,
            ) == 0
            {
                return AVIF_RESULT_BMFF_PARSE_FAILED;
            }
        }
        if avifROStreamSkip(&mut s, header.size) == 0 {
            return AVIF_RESULT_BMFF_PARSE_FAILED;
        }
    }
    return AVIF_RESULT_OK;
}
unsafe extern "C" fn avifTrackReferenceBox(
    mut track: *mut avifTrack,
    mut raw: *const uint8_t,
    mut rawLen: size_t,
    mut diag: *mut avifDiagnostics,
) -> avifResult {
    let mut s: avifROStream = avifROStream {
        raw: 0 as *mut avifROData,
        offset: 0,
        numUsedBitsInPartialByte: 0,
        diag: 0 as *mut avifDiagnostics,
        diagContext: 0 as *const core::ffi::c_char,
    };
    let mut s_roData: avifROData = avifROData {
        data: 0 as *const uint8_t,
        size: 0,
    };
    s_roData.data = raw;
    s_roData.size = rawLen;
    avifROStreamStart(
        &mut s,
        &mut s_roData,
        diag,
        b"Box[tref]\0" as *const u8 as *const core::ffi::c_char,
    );
    while avifROStreamHasBytesLeft(&mut s, 1 as size_t) != 0 {
        let mut header: avifBoxHeader = avifBoxHeader {
            isSizeZeroBox: 0,
            size: 0,
            type_0: [0; 4],
            usertype: [0; 16],
        };
        if avifROStreamReadBoxHeader(&mut s, &mut header) == 0 {
            return AVIF_RESULT_BMFF_PARSE_FAILED;
        }
        if memcmp(
            (header.type_0).as_mut_ptr() as *const core::ffi::c_void,
            b"auxl\0" as *const u8 as *const core::ffi::c_char
                as *const core::ffi::c_void,
            4 as size_t,
        ) == 0
            || memcmp(
                (header.type_0).as_mut_ptr() as *const core::ffi::c_void,
                b"prem\0" as *const u8 as *const core::ffi::c_char
                    as *const core::ffi::c_void,
                4 as size_t,
            ) == 0
        {
            if !(header.size >= ::core::mem::size_of::<uint32_t>() as usize) {
                return AVIF_RESULT_BMFF_PARSE_FAILED;
            }
            let numTrackIDs: size_t = (header.size)
                .wrapping_div(::core::mem::size_of::<uint32_t>() as size_t);
            let mut i: size_t = 0 as size_t;
            while i < numTrackIDs {
                let mut toID: uint32_t = 0;
                if avifROStreamReadU32(&mut s, &mut toID) == 0 {
                    return AVIF_RESULT_BMFF_PARSE_FAILED;
                }
                if !(toID == 0 as uint32_t) {
                    let result__: avifResult = avifAddReference(
                        &mut (*track).references,
                        (header.type_0).as_mut_ptr() as *const core::ffi::c_char,
                        toID,
                    ) as avifResult;
                    if result__ as core::ffi::c_uint
                        != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
                    {
                        return result__;
                    }
                }
                i = i.wrapping_add(1);
            }
            if avifROStreamSkip(
                &mut s,
                (header.size)
                    .wrapping_sub(
                        numTrackIDs
                            .wrapping_mul(::core::mem::size_of::<uint32_t>() as size_t),
                    ),
            ) == 0
            {
                return AVIF_RESULT_BMFF_PARSE_FAILED;
            }
        } else if avifROStreamSkip(&mut s, header.size) == 0 {
            return AVIF_RESULT_BMFF_PARSE_FAILED
        }
    }
    return AVIF_RESULT_OK;
}
unsafe extern "C" fn avifParseEditListBox(
    mut track: *mut avifTrack,
    mut raw: *const uint8_t,
    mut rawLen: size_t,
    mut diag: *mut avifDiagnostics,
) -> avifBool {
    let mut s: avifROStream = avifROStream {
        raw: 0 as *mut avifROData,
        offset: 0,
        numUsedBitsInPartialByte: 0,
        diag: 0 as *mut avifDiagnostics,
        diagContext: 0 as *const core::ffi::c_char,
    };
    let mut s_roData: avifROData = avifROData {
        data: 0 as *const uint8_t,
        size: 0,
    };
    s_roData.data = raw;
    s_roData.size = rawLen;
    avifROStreamStart(
        &mut s,
        &mut s_roData,
        diag,
        b"Box[elst]\0" as *const u8 as *const core::ffi::c_char,
    );
    let mut version: uint8_t = 0;
    let mut flags: uint32_t = 0;
    if avifROStreamReadVersionAndFlags(&mut s, &mut version, &mut flags) == 0 {
        return AVIF_FALSE;
    }
    if flags & 1 as uint32_t == 0 as uint32_t {
        (*track).isRepeating = AVIF_FALSE as avifBool;
        return AVIF_TRUE;
    }
    (*track).isRepeating = AVIF_TRUE as avifBool;
    let mut entryCount: uint32_t = 0;
    if avifROStreamReadU32(&mut s, &mut entryCount) == 0 {
        return AVIF_FALSE;
    }
    if entryCount != 1 as uint32_t {
        avifDiagnosticsPrintf(
            diag,
            b"Box[elst] contains an entry_count != 1 [%u]\0" as *const u8
                as *const core::ffi::c_char,
            entryCount,
        );
        return AVIF_FALSE;
    }
    if version as core::ffi::c_int == 1 as core::ffi::c_int {
        if avifROStreamReadU64(&mut s, &mut (*track).segmentDuration) == 0 {
            return AVIF_FALSE;
        }
    } else if version as core::ffi::c_int == 0 as core::ffi::c_int {
        let mut segmentDuration: uint32_t = 0;
        if avifROStreamReadU32(&mut s, &mut segmentDuration) == 0 {
            return AVIF_FALSE;
        }
        (*track).segmentDuration = segmentDuration as uint64_t;
    } else {
        avifDiagnosticsPrintf(
            diag,
            b"Box[elst] has an unsupported version [%u]\0" as *const u8
                as *const core::ffi::c_char,
            version as core::ffi::c_int,
        );
        return AVIF_FALSE;
    }
    if (*track).segmentDuration == 0 as uint64_t {
        avifDiagnosticsPrintf(
            diag,
            b"Box[elst] Invalid value for segment_duration (0).\0" as *const u8
                as *const core::ffi::c_char,
        );
        return AVIF_FALSE;
    }
    return AVIF_TRUE;
}
unsafe extern "C" fn avifParseEditBox(
    mut track: *mut avifTrack,
    mut raw: *const uint8_t,
    mut rawLen: size_t,
    mut diag: *mut avifDiagnostics,
) -> avifBool {
    let mut s: avifROStream = avifROStream {
        raw: 0 as *mut avifROData,
        offset: 0,
        numUsedBitsInPartialByte: 0,
        diag: 0 as *mut avifDiagnostics,
        diagContext: 0 as *const core::ffi::c_char,
    };
    let mut s_roData: avifROData = avifROData {
        data: 0 as *const uint8_t,
        size: 0,
    };
    s_roData.data = raw;
    s_roData.size = rawLen;
    avifROStreamStart(
        &mut s,
        &mut s_roData,
        diag,
        b"Box[edts]\0" as *const u8 as *const core::ffi::c_char,
    );
    let mut elstBoxSeen: avifBool = AVIF_FALSE;
    while avifROStreamHasBytesLeft(&mut s, 1 as size_t) != 0 {
        let mut header: avifBoxHeader = avifBoxHeader {
            isSizeZeroBox: 0,
            size: 0,
            type_0: [0; 4],
            usertype: [0; 16],
        };
        if avifROStreamReadBoxHeader(&mut s, &mut header) == 0 {
            return AVIF_FALSE;
        }
        if memcmp(
            (header.type_0).as_mut_ptr() as *const core::ffi::c_void,
            b"elst\0" as *const u8 as *const core::ffi::c_char
                as *const core::ffi::c_void,
            4 as size_t,
        ) == 0
        {
            if elstBoxSeen != 0 {
                avifDiagnosticsPrintf(
                    diag,
                    b"More than one [elst] Box was found.\0" as *const u8
                        as *const core::ffi::c_char,
                );
                return AVIF_FALSE;
            }
            if avifParseEditListBox(
                track,
                avifROStreamCurrent(&mut s),
                header.size,
                diag,
            ) == 0
            {
                return AVIF_FALSE;
            }
            elstBoxSeen = AVIF_TRUE as avifBool;
        }
        if avifROStreamSkip(&mut s, header.size) == 0 {
            return AVIF_FALSE;
        }
    }
    if elstBoxSeen == 0 {
        avifDiagnosticsPrintf(
            diag,
            b"Box[edts] contains no [elst] Box.\0" as *const u8
                as *const core::ffi::c_char,
        );
        return AVIF_FALSE;
    }
    return AVIF_TRUE;
}
unsafe extern "C" fn avifParseTrackBox(
    mut data: *mut avifDecoderData,
    mut rawOffset: uint64_t,
    mut raw: *const uint8_t,
    mut rawLen: size_t,
) -> avifResult {
    let mut s: avifROStream = avifROStream {
        raw: 0 as *mut avifROData,
        offset: 0,
        numUsedBitsInPartialByte: 0,
        diag: 0 as *mut avifDiagnostics,
        diagContext: 0 as *const core::ffi::c_char,
    };
    let mut s_roData: avifROData = avifROData {
        data: 0 as *const uint8_t,
        size: 0,
    };
    s_roData.data = raw;
    s_roData.size = rawLen;
    avifROStreamStart(
        &mut s,
        &mut s_roData,
        (*data).diag,
        b"Box[trak]\0" as *const u8 as *const core::ffi::c_char,
    );
    let mut track: *mut avifTrack = avifDecoderDataCreateTrack(data);
    if track.is_null() {
        return AVIF_RESULT_OUT_OF_MEMORY;
    }
    let mut edtsBoxSeen: avifBool = AVIF_FALSE;
    let mut tkhdSeen: avifBool = AVIF_FALSE;
    while avifROStreamHasBytesLeft(&mut s, 1 as size_t) != 0 {
        let mut header: avifBoxHeader = avifBoxHeader {
            isSizeZeroBox: 0,
            size: 0,
            type_0: [0; 4],
            usertype: [0; 16],
        };
        if avifROStreamReadBoxHeader(&mut s, &mut header) == 0 {
            return AVIF_RESULT_BMFF_PARSE_FAILED;
        }
        if memcmp(
            (header.type_0).as_mut_ptr() as *const core::ffi::c_void,
            b"tkhd\0" as *const u8 as *const core::ffi::c_char
                as *const core::ffi::c_void,
            4 as size_t,
        ) == 0
        {
            if tkhdSeen != 0 {
                avifDiagnosticsPrintf(
                    (*data).diag,
                    b"Box[trak] contains a duplicate unique box of type 'tkhd'\0"
                        as *const u8 as *const core::ffi::c_char,
                );
                return AVIF_RESULT_BMFF_PARSE_FAILED;
            }
            if avifParseTrackHeaderBox(
                track,
                avifROStreamCurrent(&mut s),
                header.size,
                (*data).diag,
            ) == 0
            {
                return AVIF_RESULT_BMFF_PARSE_FAILED;
            }
            tkhdSeen = AVIF_TRUE as avifBool;
        } else if memcmp(
            (header.type_0).as_mut_ptr() as *const core::ffi::c_void,
            b"meta\0" as *const u8 as *const core::ffi::c_char
                as *const core::ffi::c_void,
            4 as size_t,
        ) == 0
        {
            let result__: avifResult = avifParseMetaBox(
                (*track).meta as *mut avifMeta,
                rawOffset.wrapping_add(avifROStreamOffset(&mut s) as uint64_t),
                avifROStreamCurrent(&mut s),
                header.size,
                (*data).diag,
            ) as avifResult;
            if result__ as core::ffi::c_uint
                != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
            {
                return result__;
            }
        } else if memcmp(
            (header.type_0).as_mut_ptr() as *const core::ffi::c_void,
            b"mdia\0" as *const u8 as *const core::ffi::c_char
                as *const core::ffi::c_void,
            4 as size_t,
        ) == 0
        {
            let result___0: avifResult = avifParseMediaBox(
                track,
                rawOffset.wrapping_add(avifROStreamOffset(&mut s) as uint64_t),
                avifROStreamCurrent(&mut s),
                header.size,
                (*data).diag,
            ) as avifResult;
            if result___0 as core::ffi::c_uint
                != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
            {
                return result___0;
            }
        } else if memcmp(
            (header.type_0).as_mut_ptr() as *const core::ffi::c_void,
            b"tref\0" as *const u8 as *const core::ffi::c_char
                as *const core::ffi::c_void,
            4 as size_t,
        ) == 0
        {
            let result___1: avifResult = avifTrackReferenceBox(
                track,
                avifROStreamCurrent(&mut s),
                header.size,
                (*data).diag,
            ) as avifResult;
            if result___1 as core::ffi::c_uint
                != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
            {
                return result___1;
            }
        } else if memcmp(
            (header.type_0).as_mut_ptr() as *const core::ffi::c_void,
            b"edts\0" as *const u8 as *const core::ffi::c_char
                as *const core::ffi::c_void,
            4 as size_t,
        ) == 0
        {
            if edtsBoxSeen != 0 {
                avifDiagnosticsPrintf(
                    (*data).diag,
                    b"Box[trak] contains a duplicate unique box of type 'edts'\0"
                        as *const u8 as *const core::ffi::c_char,
                );
                return AVIF_RESULT_BMFF_PARSE_FAILED;
            }
            if avifParseEditBox(
                track,
                avifROStreamCurrent(&mut s),
                header.size,
                (*data).diag,
            ) == 0
            {
                return AVIF_RESULT_BMFF_PARSE_FAILED;
            }
            edtsBoxSeen = AVIF_TRUE as avifBool;
        }
        if avifROStreamSkip(&mut s, header.size) == 0 {
            return AVIF_RESULT_BMFF_PARSE_FAILED;
        }
    }
    if tkhdSeen == 0 {
        avifDiagnosticsPrintf(
            (*data).diag,
            b"Box[trak] does not contain a mandatory [tkhd] box\0" as *const u8
                as *const core::ffi::c_char,
        );
        return AVIF_RESULT_BMFF_PARSE_FAILED;
    }
    if edtsBoxSeen == 0 {
        (*track).repetitionCount = AVIF_REPETITION_COUNT_UNKNOWN;
    } else if (*track).isRepeating != 0 {
        if (*track).trackDuration == AVIF_INDEFINITE_DURATION64 as uint64_t {
            (*track).repetitionCount = AVIF_REPETITION_COUNT_INFINITE;
        } else {
            'c_20553: {
                if (*track).segmentDuration != 0 as uint64_t {} else {
                    __assert_fail(
                        b"track->segmentDuration != 0\0" as *const u8
                            as *const core::ffi::c_char,
                        b"/home/runner/work/libavif-rs/libavif-rs/upstream/libavif/src/read.c\0"
                            as *const u8 as *const core::ffi::c_char,
                        4105 as core::ffi::c_uint,
                        (::core::mem::transmute::<
                            [u8; 83],
                            [core::ffi::c_char; 83],
                        >(
                            *b"avifResult avifParseTrackBox(avifDecoderData *, uint64_t, const uint8_t *, size_t)\0",
                        ))
                            .as_ptr(),
                    );
                }
            };
            if (*track).trackDuration == 0 as uint64_t {
                avifDiagnosticsPrintf(
                    (*data).diag,
                    b"Invalid track duration 0.\0" as *const u8
                        as *const core::ffi::c_char,
                );
                return AVIF_RESULT_BMFF_PARSE_FAILED;
            }
            let repetitionCount: uint64_t = ((*track).trackDuration)
                .wrapping_div((*track).segmentDuration)
                .wrapping_add(
                    (((*track).trackDuration).wrapping_rem((*track).segmentDuration)
                        != 0 as uint64_t) as core::ffi::c_int as uint64_t,
                )
                .wrapping_sub(1 as uint64_t);
            if repetitionCount > INT_MAX as uint64_t {
                (*track).repetitionCount = AVIF_REPETITION_COUNT_INFINITE;
            } else {
                (*track).repetitionCount = repetitionCount as core::ffi::c_int;
            }
        }
    } else {
        (*track).repetitionCount = 0 as core::ffi::c_int;
    }
    return AVIF_RESULT_OK;
}
unsafe extern "C" fn avifParseMovieBox(
    mut data: *mut avifDecoderData,
    mut rawOffset: uint64_t,
    mut raw: *const uint8_t,
    mut rawLen: size_t,
    mut imageSizeLimit: uint32_t,
    mut imageDimensionLimit: uint32_t,
) -> avifResult {
    let mut s: avifROStream = avifROStream {
        raw: 0 as *mut avifROData,
        offset: 0,
        numUsedBitsInPartialByte: 0,
        diag: 0 as *mut avifDiagnostics,
        diagContext: 0 as *const core::ffi::c_char,
    };
    let mut s_roData: avifROData = avifROData {
        data: 0 as *const uint8_t,
        size: 0,
    };
    s_roData.data = raw;
    s_roData.size = rawLen;
    avifROStreamStart(
        &mut s,
        &mut s_roData,
        (*data).diag,
        b"Box[moov]\0" as *const u8 as *const core::ffi::c_char,
    );
    let mut hasTrak: avifBool = AVIF_FALSE;
    while avifROStreamHasBytesLeft(&mut s, 1 as size_t) != 0 {
        let mut header: avifBoxHeader = avifBoxHeader {
            isSizeZeroBox: 0,
            size: 0,
            type_0: [0; 4],
            usertype: [0; 16],
        };
        if avifROStreamReadBoxHeader(&mut s, &mut header) == 0 {
            return AVIF_RESULT_BMFF_PARSE_FAILED;
        }
        if memcmp(
            (header.type_0).as_mut_ptr() as *const core::ffi::c_void,
            b"trak\0" as *const u8 as *const core::ffi::c_char
                as *const core::ffi::c_void,
            4 as size_t,
        ) == 0
        {
            let result__: avifResult = avifParseTrackBox(
                data,
                rawOffset.wrapping_add(avifROStreamOffset(&mut s) as uint64_t),
                avifROStreamCurrent(&mut s),
                header.size,
            ) as avifResult;
            if result__ as core::ffi::c_uint
                != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
            {
                return result__;
            }
            hasTrak = AVIF_TRUE as avifBool;
            let mut track: *const avifTrack = &mut *((*data).tracks.track)
                .offset(((*data).tracks.count).wrapping_sub(1 as uint32_t) as isize)
                as *mut avifTrack;
            if memcmp(
                ((*track).handlerType).as_ptr() as *const core::ffi::c_void,
                b"pict\0" as *const u8 as *const core::ffi::c_char
                    as *const core::ffi::c_void,
                4 as size_t,
            ) == 0
                || memcmp(
                    ((*track).handlerType).as_ptr() as *const core::ffi::c_void,
                    b"vide\0" as *const u8 as *const core::ffi::c_char
                        as *const core::ffi::c_void,
                    4 as size_t,
                ) == 0
                || memcmp(
                    ((*track).handlerType).as_ptr() as *const core::ffi::c_void,
                    b"auxv\0" as *const u8 as *const core::ffi::c_char
                        as *const core::ffi::c_void,
                    4 as size_t,
                ) == 0
            {
                if (*track).width == 0 as uint32_t || (*track).height == 0 as uint32_t {
                    avifDiagnosticsPrintf(
                        (*data).diag,
                        b"Track ID [%u] has an invalid size [%ux%u]\0" as *const u8
                            as *const core::ffi::c_char,
                        (*track).id,
                        (*track).width,
                        (*track).height,
                    );
                    return AVIF_RESULT_BMFF_PARSE_FAILED;
                }
                if avifDimensionsTooLarge(
                    (*track).width,
                    (*track).height,
                    imageSizeLimit,
                    imageDimensionLimit,
                ) != 0
                {
                    avifDiagnosticsPrintf(
                        (*data).diag,
                        b"Track ID [%u] dimensions are too large [%ux%u]\0" as *const u8
                            as *const core::ffi::c_char,
                        (*track).id,
                        (*track).width,
                        (*track).height,
                    );
                    return AVIF_RESULT_BMFF_PARSE_FAILED;
                }
            }
        }
        if avifROStreamSkip(&mut s, header.size) == 0 {
            return AVIF_RESULT_BMFF_PARSE_FAILED;
        }
    }
    if hasTrak == 0 {
        avifDiagnosticsPrintf(
            (*data).diag,
            b"moov box does not contain any tracks\0" as *const u8
                as *const core::ffi::c_char,
        );
        return AVIF_RESULT_BMFF_PARSE_FAILED;
    }
    return AVIF_RESULT_OK;
}
unsafe extern "C" fn avifParseFileTypeBox(
    mut ftyp: *mut avifFileType,
    mut raw: *const uint8_t,
    mut rawLen: size_t,
    mut diag: *mut avifDiagnostics,
) -> avifBool {
    let mut s: avifROStream = avifROStream {
        raw: 0 as *mut avifROData,
        offset: 0,
        numUsedBitsInPartialByte: 0,
        diag: 0 as *mut avifDiagnostics,
        diagContext: 0 as *const core::ffi::c_char,
    };
    let mut s_roData: avifROData = avifROData {
        data: 0 as *const uint8_t,
        size: 0,
    };
    s_roData.data = raw;
    s_roData.size = rawLen;
    avifROStreamStart(
        &mut s,
        &mut s_roData,
        diag,
        b"Box[ftyp]\0" as *const u8 as *const core::ffi::c_char,
    );
    if avifROStreamRead(&mut s, ((*ftyp).majorBrand).as_mut_ptr(), 4 as size_t) == 0 {
        return AVIF_FALSE;
    }
    if avifROStreamRead(&mut s, ((*ftyp).minorVersion).as_mut_ptr(), 4 as size_t) == 0 {
        return AVIF_FALSE;
    }
    let mut compatibleBrandsBytes: size_t = avifROStreamRemainingBytes(&mut s);
    if compatibleBrandsBytes.wrapping_rem(4 as size_t) != 0 as size_t {
        avifDiagnosticsPrintf(
            diag,
            b"Box[ftyp] contains a compatible brands section that isn't divisible by 4 [%zu]\0"
                as *const u8 as *const core::ffi::c_char,
            compatibleBrandsBytes,
        );
        return AVIF_FALSE;
    }
    (*ftyp).compatibleBrands = avifROStreamCurrent(&mut s);
    if avifROStreamSkip(&mut s, compatibleBrandsBytes) == 0 {
        return AVIF_FALSE;
    }
    (*ftyp).compatibleBrandsCount = compatibleBrandsBytes as core::ffi::c_int
        / 4 as core::ffi::c_int;
    return AVIF_TRUE;
}
unsafe extern "C" fn avifParse(mut decoder: *mut avifDecoder) -> avifResult {
    let mut readResult: avifResult = AVIF_RESULT_OK;
    let mut parseOffset: uint64_t = 0 as uint64_t;
    let mut data: *mut avifDecoderData = (*decoder).data as *mut avifDecoderData;
    let mut ftypSeen: avifBool = AVIF_FALSE;
    let mut metaSeen: avifBool = AVIF_FALSE;
    let mut metaIsSizeZero: avifBool = AVIF_FALSE;
    let mut moovSeen: avifBool = AVIF_FALSE;
    let mut needsMeta: avifBool = AVIF_FALSE;
    let mut needsMoov: avifBool = AVIF_FALSE;
    let mut needsTmap: avifBool = AVIF_FALSE;
    let mut tmapSeen: avifBool = AVIF_FALSE;
    let mut ftyp: avifFileType = {
        let mut init = avifFileType {
            majorBrand: [0 as core::ffi::c_int as uint8_t, 0, 0, 0],
            minorVersion: [0; 4],
            compatibleBrands: 0 as *const uint8_t,
            compatibleBrandsCount: 0,
        };
        init
    };
    loop {
        let mut headerContents: avifROData = avifROData {
            data: 0 as *const uint8_t,
            size: 0,
        };
        if (*(*decoder).io).sizeHint > 0 as uint64_t
            && parseOffset > (*(*decoder).io).sizeHint
        {
            return AVIF_RESULT_BMFF_PARSE_FAILED;
        }
        readResult = ((*(*decoder).io).read)
            .expect(
                "non-null function pointer",
            )(
            (*decoder).io as *mut avifIO,
            0 as uint32_t,
            parseOffset,
            32 as size_t,
            &mut headerContents,
        );
        if readResult as core::ffi::c_uint
            != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
        {
            return readResult;
        }
        if headerContents.size == 0 {
            break;
        }
        let mut headerStream: avifROStream = avifROStream {
            raw: 0 as *mut avifROData,
            offset: 0,
            numUsedBitsInPartialByte: 0,
            diag: 0 as *mut avifDiagnostics,
            diagContext: 0 as *const core::ffi::c_char,
        };
        let mut headerStream_roData: avifROData = avifROData {
            data: 0 as *const uint8_t,
            size: 0,
        };
        headerStream_roData.data = headerContents.data;
        headerStream_roData.size = headerContents.size;
        avifROStreamStart(
            &mut headerStream,
            &mut headerStream_roData,
            &mut (*decoder).diag,
            b"File-level box header\0" as *const u8 as *const core::ffi::c_char,
        );
        let mut header: avifBoxHeader = avifBoxHeader {
            isSizeZeroBox: 0,
            size: 0,
            type_0: [0; 4],
            usertype: [0; 16],
        };
        if avifROStreamReadBoxHeaderPartial(
            &mut headerStream,
            &mut header,
            1 as avifBool,
        ) == 0
        {
            return AVIF_RESULT_BMFF_PARSE_FAILED;
        }
        parseOffset = (parseOffset as core::ffi::c_ulong)
            .wrapping_add(avifROStreamOffset(&mut headerStream) as core::ffi::c_ulong)
            as uint64_t as uint64_t;
        'c_33248: {
            if (*(*decoder).io).sizeHint == 0 as uint64_t
                || parseOffset <= (*(*decoder).io).sizeHint
            {} else {
                __assert_fail(
                    b"decoder->io->sizeHint == 0 || parseOffset <= decoder->io->sizeHint\0"
                        as *const u8 as *const core::ffi::c_char,
                    b"/home/runner/work/libavif-rs/libavif-rs/upstream/libavif/src/read.c\0"
                        as *const u8 as *const core::ffi::c_char,
                    4955 as core::ffi::c_uint,
                    (::core::mem::transmute::<
                        [u8; 36],
                        [core::ffi::c_char; 36],
                    >(*b"avifResult avifParse(avifDecoder *)\0"))
                        .as_ptr(),
                );
            }
        };
        let mut boxOffset: uint64_t = 0 as uint64_t;
        let mut boxContents: avifROData = {
            let mut init = avifROData {
                data: 0 as *const uint8_t,
                size: 0 as size_t,
            };
            init
        };
        let mut isFtyp: avifBool = AVIF_FALSE;
        let mut isMeta: avifBool = AVIF_FALSE;
        let mut isMoov: avifBool = AVIF_FALSE;
        let mut isNonSkippableVariableLengthBox: avifBool = AVIF_FALSE;
        if memcmp(
            (header.type_0).as_mut_ptr() as *const core::ffi::c_void,
            b"ftyp\0" as *const u8 as *const core::ffi::c_char
                as *const core::ffi::c_void,
            4 as size_t,
        ) == 0
        {
            isFtyp = AVIF_TRUE as avifBool;
            isNonSkippableVariableLengthBox = AVIF_TRUE as avifBool;
        } else if memcmp(
            (header.type_0).as_mut_ptr() as *const core::ffi::c_void,
            b"meta\0" as *const u8 as *const core::ffi::c_char
                as *const core::ffi::c_void,
            4 as size_t,
        ) == 0
        {
            isMeta = AVIF_TRUE as avifBool;
            isNonSkippableVariableLengthBox = AVIF_TRUE as avifBool;
            metaIsSizeZero = header.isSizeZeroBox;
        } else if memcmp(
            (header.type_0).as_mut_ptr() as *const core::ffi::c_void,
            b"moov\0" as *const u8 as *const core::ffi::c_char
                as *const core::ffi::c_void,
            4 as size_t,
        ) == 0
        {
            isMoov = AVIF_TRUE as avifBool;
            isNonSkippableVariableLengthBox = AVIF_TRUE as avifBool;
        }
        if isFtyp == 0
            && (isNonSkippableVariableLengthBox != 0
                || memcmp(
                    (header.type_0).as_mut_ptr() as *const core::ffi::c_void,
                    b"free\0" as *const u8 as *const core::ffi::c_char
                        as *const core::ffi::c_void,
                    4 as size_t,
                ) == 0
                || memcmp(
                    (header.type_0).as_mut_ptr() as *const core::ffi::c_void,
                    b"skip\0" as *const u8 as *const core::ffi::c_char
                        as *const core::ffi::c_void,
                    4 as size_t,
                ) == 0
                || memcmp(
                    (header.type_0).as_mut_ptr() as *const core::ffi::c_void,
                    b"mdat\0" as *const u8 as *const core::ffi::c_char
                        as *const core::ffi::c_void,
                    4 as size_t,
                ) == 0)
        {
            if ftypSeen == 0 {
                return AVIF_RESULT_BMFF_PARSE_FAILED;
            }
        }
        if isNonSkippableVariableLengthBox != 0 {
            boxOffset = parseOffset;
            let mut sizeToRead: size_t = 0;
            if header.isSizeZeroBox != 0 {
                if (*(*decoder).io).sizeHint != 0 as uint64_t
                    && ((*(*decoder).io).sizeHint).wrapping_sub(parseOffset)
                        < SIZE_MAX as uint64_t
                {
                    sizeToRead = ((*(*decoder).io).sizeHint).wrapping_sub(parseOffset)
                        as size_t;
                } else {
                    sizeToRead = SIZE_MAX as size_t;
                }
            } else {
                sizeToRead = header.size;
            }
            readResult = ((*(*decoder).io).read)
                .expect(
                    "non-null function pointer",
                )(
                (*decoder).io as *mut avifIO,
                0 as uint32_t,
                parseOffset,
                sizeToRead,
                &mut boxContents,
            );
            if readResult as core::ffi::c_uint
                != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
            {
                return readResult;
            }
            if header.isSizeZeroBox != 0 {
                header.size = boxContents.size;
            } else if boxContents.size != header.size {
                return AVIF_RESULT_TRUNCATED_DATA
            }
        } else if header.isSizeZeroBox != 0 {
            return AVIF_RESULT_BMFF_PARSE_FAILED
        } else if header.size
            > (UINT64_MAX as size_t).wrapping_sub(parseOffset as size_t)
        {
            return AVIF_RESULT_BMFF_PARSE_FAILED
        }
        parseOffset = (parseOffset as core::ffi::c_ulong)
            .wrapping_add(header.size as core::ffi::c_ulong) as uint64_t as uint64_t;
        if isFtyp != 0 {
            if ftypSeen != 0 {
                return AVIF_RESULT_BMFF_PARSE_FAILED;
            }
            if avifParseFileTypeBox(
                &mut ftyp,
                boxContents.data,
                boxContents.size,
                (*data).diag,
            ) == 0
            {
                return AVIF_RESULT_BMFF_PARSE_FAILED;
            }
            if avifFileTypeIsCompatible(&mut ftyp) == 0 {
                return AVIF_RESULT_INVALID_FTYP;
            }
            ftypSeen = AVIF_TRUE as avifBool;
            memcpy(
                ((*data).majorBrand).as_mut_ptr() as *mut core::ffi::c_void,
                (ftyp.majorBrand).as_mut_ptr() as *const core::ffi::c_void,
                4 as size_t,
            );
            if ftyp.compatibleBrandsCount > 0 as core::ffi::c_int {
                if avifArrayCreate(
                    &mut (*data).compatibleBrands as *mut avifBrandArray
                        as *mut core::ffi::c_void,
                    ::core::mem::size_of::<avifBrand>() as uint32_t,
                    ftyp.compatibleBrandsCount as uint32_t,
                ) == 0
                {
                    return AVIF_RESULT_OUT_OF_MEMORY;
                }
                memcpy(
                    (*data).compatibleBrands.brand as *mut core::ffi::c_void,
                    ftyp.compatibleBrands as *const core::ffi::c_void,
                    (::core::mem::size_of::<avifBrand>() as size_t)
                        .wrapping_mul(ftyp.compatibleBrandsCount as size_t),
                );
                (*data).compatibleBrands.count = ftyp.compatibleBrandsCount as uint32_t;
            }
            needsMeta = avifFileTypeHasBrand(
                &mut ftyp,
                b"avif\0" as *const u8 as *const core::ffi::c_char,
            );
            needsMoov = avifFileTypeHasBrand(
                &mut ftyp,
                b"avis\0" as *const u8 as *const core::ffi::c_char,
            );
            needsTmap = avifFileTypeHasBrand(
                &mut ftyp,
                b"tmap\0" as *const u8 as *const core::ffi::c_char,
            );
            if needsTmap != 0 {
                needsMeta = AVIF_TRUE as avifBool;
            }
        } else if isMeta != 0 {
            if metaSeen != 0 {
                return AVIF_RESULT_BMFF_PARSE_FAILED;
            }
            let result__: avifResult = avifParseMetaBox(
                (*data).meta,
                boxOffset,
                boxContents.data,
                boxContents.size,
                (*data).diag,
            ) as avifResult;
            if result__ as core::ffi::c_uint
                != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
            {
                return result__;
            }
            metaSeen = AVIF_TRUE as avifBool;
            let mut itemIndex: uint32_t = 0 as uint32_t;
            while itemIndex < (*(*data).meta).items.count {
                if memcmp(
                    ((**((*(*data).meta).items.item).offset(itemIndex as isize)).type_0)
                        .as_mut_ptr() as *const core::ffi::c_void,
                    b"tmap\0" as *const u8 as *const core::ffi::c_char
                        as *const core::ffi::c_void,
                    4 as size_t,
                ) == 0
                {
                    tmapSeen = AVIF_TRUE as avifBool;
                    break;
                } else {
                    itemIndex = itemIndex.wrapping_add(1);
                }
            }
        } else if isMoov != 0 {
            if moovSeen != 0 {
                return AVIF_RESULT_BMFF_PARSE_FAILED;
            }
            let result___0: avifResult = avifParseMovieBox(
                data,
                boxOffset,
                boxContents.data,
                boxContents.size,
                (*decoder).imageSizeLimit,
                (*decoder).imageDimensionLimit,
            ) as avifResult;
            if result___0 as core::ffi::c_uint
                != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
            {
                return result___0;
            }
            moovSeen = AVIF_TRUE as avifBool;
            (*decoder).imageSequenceTrackPresent = AVIF_TRUE as avifBool;
        }
        let mut sawEverythingNeeded: avifBool = (ftypSeen != 0
            && (needsMeta == 0 || metaSeen != 0) && (needsMoov == 0 || moovSeen != 0)
            && (needsTmap == 0 || tmapSeen != 0)) as core::ffi::c_int;
        if sawEverythingNeeded != 0 {
            return AVIF_RESULT_OK;
        }
    }
    if ftypSeen == 0 {
        return AVIF_RESULT_INVALID_FTYP;
    }
    if needsMeta != 0 && metaSeen == 0 || needsMoov != 0 && moovSeen == 0 {
        return AVIF_RESULT_TRUNCATED_DATA;
    }
    if needsTmap != 0 && tmapSeen == 0 {
        return (if metaIsSizeZero != 0 {
            AVIF_RESULT_TRUNCATED_DATA as core::ffi::c_int
        } else {
            AVIF_RESULT_BMFF_PARSE_FAILED as core::ffi::c_int
        }) as avifResult;
    }
    return AVIF_RESULT_OK;
}
unsafe extern "C" fn avifFileTypeHasBrand(
    mut ftyp: *mut avifFileType,
    mut brand: *const core::ffi::c_char,
) -> avifBool {
    if memcmp(
        ((*ftyp).majorBrand).as_mut_ptr() as *const core::ffi::c_void,
        brand as *const core::ffi::c_void,
        4 as size_t,
    ) == 0
    {
        return AVIF_TRUE;
    }
    let mut compatibleBrandIndex: core::ffi::c_int = 0 as core::ffi::c_int;
    while compatibleBrandIndex < (*ftyp).compatibleBrandsCount {
        let mut compatibleBrand: *const uint8_t = &*((*ftyp).compatibleBrands)
            .offset((4 as core::ffi::c_int * compatibleBrandIndex) as isize)
            as *const uint8_t;
        if memcmp(
            compatibleBrand as *const core::ffi::c_void,
            brand as *const core::ffi::c_void,
            4 as size_t,
        ) == 0
        {
            return AVIF_TRUE;
        }
        compatibleBrandIndex += 1;
    }
    return AVIF_FALSE;
}
unsafe extern "C" fn avifFileTypeIsCompatible(mut ftyp: *mut avifFileType) -> avifBool {
    return (avifFileTypeHasBrand(
        ftyp,
        b"avif\0" as *const u8 as *const core::ffi::c_char,
    ) != 0
        || avifFileTypeHasBrand(ftyp, b"avis\0" as *const u8 as *const core::ffi::c_char)
            != 0) as core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn avifPeekCompatibleFileType(
    mut input: *const avifROData,
) -> avifBool {
    let mut s: avifROStream = avifROStream {
        raw: 0 as *mut avifROData,
        offset: 0,
        numUsedBitsInPartialByte: 0,
        diag: 0 as *mut avifDiagnostics,
        diagContext: 0 as *const core::ffi::c_char,
    };
    let mut s_roData: avifROData = avifROData {
        data: 0 as *const uint8_t,
        size: 0,
    };
    s_roData.data = (*input).data;
    s_roData.size = (*input).size;
    avifROStreamStart(
        &mut s,
        &mut s_roData,
        0 as *mut avifDiagnostics,
        0 as *const core::ffi::c_char,
    );
    let mut header: avifBoxHeader = avifBoxHeader {
        isSizeZeroBox: 0,
        size: 0,
        type_0: [0; 4],
        usertype: [0; 16],
    };
    if avifROStreamReadBoxHeaderPartial(&mut s, &mut header, AVIF_TRUE) == 0
        || memcmp(
            (header.type_0).as_mut_ptr() as *const core::ffi::c_void,
            b"ftyp\0" as *const u8 as *const core::ffi::c_char
                as *const core::ffi::c_void,
            4 as size_t,
        ) != 0
    {
        return AVIF_FALSE;
    }
    if header.isSizeZeroBox != 0 {
        return AVIF_FALSE;
    }
    if avifROStreamHasBytesLeft(&mut s, header.size) == 0 {
        return AVIF_FALSE;
    }
    let mut ftyp: avifFileType = avifFileType {
        majorBrand: [0; 4],
        minorVersion: [0; 4],
        compatibleBrands: 0 as *const uint8_t,
        compatibleBrandsCount: 0,
    };
    memset(
        &mut ftyp as *mut avifFileType as *mut core::ffi::c_void,
        0 as core::ffi::c_int,
        ::core::mem::size_of::<avifFileType>() as size_t,
    );
    let mut parsed: avifBool = avifParseFileTypeBox(
        &mut ftyp,
        avifROStreamCurrent(&mut s),
        header.size,
        0 as *mut avifDiagnostics,
    );
    if parsed == 0 {
        return AVIF_FALSE;
    }
    return avifFileTypeIsCompatible(&mut ftyp);
}
unsafe extern "C" fn avifBrandArrayHasBrand(
    mut brands: *mut avifBrandArray,
    mut brand: *const core::ffi::c_char,
) -> avifBool {
    let mut brandIndex: uint32_t = 0 as uint32_t;
    while brandIndex < (*brands).count {
        if memcmp(
            (*((*brands).brand).offset(brandIndex as isize)).as_mut_ptr()
                as *const core::ffi::c_void,
            brand as *const core::ffi::c_void,
            4 as size_t,
        ) == 0
        {
            return AVIF_TRUE;
        }
        brandIndex = brandIndex.wrapping_add(1);
    }
    return AVIF_FALSE;
}
#[no_mangle]
pub unsafe extern "C" fn avifDecoderCreate() -> *mut avifDecoder {
    let mut decoder: *mut avifDecoder = avifAlloc(
        ::core::mem::size_of::<avifDecoder>() as size_t,
    ) as *mut avifDecoder;
    if decoder.is_null() {
        return 0 as *mut avifDecoder;
    }
    memset(
        decoder as *mut core::ffi::c_void,
        0 as core::ffi::c_int,
        ::core::mem::size_of::<avifDecoder>() as size_t,
    );
    (*decoder).maxThreads = 1 as core::ffi::c_int;
    (*decoder).imageSizeLimit = AVIF_DEFAULT_IMAGE_SIZE_LIMIT as uint32_t;
    (*decoder).imageDimensionLimit = AVIF_DEFAULT_IMAGE_DIMENSION_LIMIT as uint32_t;
    (*decoder).imageCountLimit = AVIF_DEFAULT_IMAGE_COUNT_LIMIT as uint32_t;
    (*decoder).strictFlags = AVIF_STRICT_ENABLED as core::ffi::c_int as avifStrictFlags;
    (*decoder).imageContentToDecode = AVIF_IMAGE_CONTENT_DECODE_DEFAULT
        as core::ffi::c_int as avifImageContentTypeFlags;
    return decoder;
}
unsafe extern "C" fn avifDecoderCleanup(mut decoder: *mut avifDecoder) {
    if !((*decoder).data).is_null() {
        avifDecoderDataDestroy((*decoder).data as *mut avifDecoderData);
        (*decoder).data = 0 as *mut avifDecoderData;
    }
    if !((*decoder).image).is_null() {
        avifImageDestroy((*decoder).image);
        (*decoder).image = 0 as *mut avifImage;
    }
    avifDiagnosticsClearError(&mut (*decoder).diag);
}
#[no_mangle]
pub unsafe extern "C" fn avifDecoderDestroy(mut decoder: *mut avifDecoder) {
    avifDecoderCleanup(decoder);
    avifIODestroy((*decoder).io);
    avifFree(decoder as *mut core::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn avifDecoderSetSource(
    mut decoder: *mut avifDecoder,
    mut source: avifDecoderSource,
) -> avifResult {
    (*decoder).requestedSource = source;
    return avifDecoderReset(decoder);
}
#[no_mangle]
pub unsafe extern "C" fn avifDecoderSetIO(
    mut decoder: *mut avifDecoder,
    mut io: *mut avifIO,
) {
    avifIODestroy((*decoder).io);
    (*decoder).io = io;
}
#[no_mangle]
pub unsafe extern "C" fn avifDecoderSetIOMemory(
    mut decoder: *mut avifDecoder,
    mut data: *const uint8_t,
    mut size: size_t,
) -> avifResult {
    let mut io: *mut avifIO = avifIOCreateMemoryReader(data, size);
    if io.is_null() {
        return AVIF_RESULT_OUT_OF_MEMORY;
    }
    avifDecoderSetIO(decoder, io);
    return AVIF_RESULT_OK;
}
#[no_mangle]
pub unsafe extern "C" fn avifDecoderSetIOFile(
    mut decoder: *mut avifDecoder,
    mut filename: *const core::ffi::c_char,
) -> avifResult {
    let mut io: *mut avifIO = avifIOCreateFileReader(filename);
    if io.is_null() {
        return AVIF_RESULT_IO_ERROR;
    }
    avifDecoderSetIO(decoder, io);
    return AVIF_RESULT_OK;
}
unsafe extern "C" fn avifExtentMerge(
    mut dst: *mut avifExtent,
    mut src: *const avifExtent,
) -> avifResult {
    if (*dst).size == 0 {
        *dst = *src;
        return AVIF_RESULT_OK;
    }
    if (*src).size == 0 {
        return AVIF_RESULT_OK;
    }
    let minExtent1: uint64_t = (*dst).offset;
    let maxExtent1: uint64_t = ((*dst).offset).wrapping_add((*dst).size as uint64_t);
    let minExtent2: uint64_t = (*src).offset;
    let maxExtent2: uint64_t = ((*src).offset).wrapping_add((*src).size as uint64_t);
    (*dst).offset = if minExtent1 < minExtent2 { minExtent1 } else { minExtent2 };
    let extentLength: uint64_t = (if maxExtent1 > maxExtent2 {
        maxExtent1
    } else {
        maxExtent2
    })
        .wrapping_sub((*dst).offset);
    (*dst).size = extentLength as size_t;
    return AVIF_RESULT_OK;
}
#[no_mangle]
pub unsafe extern "C" fn avifDecoderNthImageMaxExtent(
    mut decoder: *const avifDecoder,
    mut frameIndex: uint32_t,
    mut outExtent: *mut avifExtent,
) -> avifResult {
    if ((*decoder).data).is_null() {
        return AVIF_RESULT_NO_CONTENT;
    }
    memset(
        outExtent as *mut core::ffi::c_void,
        0 as core::ffi::c_int,
        ::core::mem::size_of::<avifExtent>() as size_t,
    );
    let mut startFrameIndex: uint32_t = avifDecoderNearestKeyframe(decoder, frameIndex);
    let mut endFrameIndex: uint32_t = frameIndex;
    let mut currentFrameIndex: uint32_t = startFrameIndex;
    while currentFrameIndex <= endFrameIndex {
        let mut tileIndex: core::ffi::c_uint = 0 as core::ffi::c_uint;
        while (tileIndex as uint32_t) < (*(*decoder).data).tiles.count {
            let mut tile: *mut avifTile = &mut *((*(*decoder).data).tiles.tile)
                .offset(tileIndex as isize) as *mut avifTile;
            if currentFrameIndex >= (*(*tile).input).samples.count {
                return AVIF_RESULT_NO_IMAGES_REMAINING;
            }
            let mut sample: *mut avifDecodeSample = &mut *((*(*tile).input)
                .samples
                .sample)
                .offset(currentFrameIndex as isize) as *mut avifDecodeSample;
            let mut sampleExtent: avifExtent = avifExtent { offset: 0, size: 0 };
            if (*sample).itemID != 0 {
                let mut item: *mut avifDecoderItem = 0 as *mut avifDecoderItem;
                let result__: avifResult = avifMetaFindOrCreateItem(
                    (*(*decoder).data).meta,
                    (*sample).itemID,
                    &mut item,
                ) as avifResult;
                if result__ as core::ffi::c_uint
                    != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
                {
                    return result__;
                }
                let mut maxExtentResult: avifResult = avifDecoderItemMaxExtent(
                    item,
                    sample,
                    &mut sampleExtent,
                );
                if maxExtentResult as core::ffi::c_uint
                    != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
                {
                    return maxExtentResult;
                }
            } else {
                sampleExtent.offset = (*sample).offset;
                sampleExtent.size = (*sample).size;
            }
            if sampleExtent.size
                > (UINT64_MAX as size_t).wrapping_sub(sampleExtent.offset as size_t)
            {
                return AVIF_RESULT_BMFF_PARSE_FAILED;
            }
            let mut extentMergeResult: avifResult = avifExtentMerge(
                outExtent,
                &mut sampleExtent,
            );
            if extentMergeResult as core::ffi::c_uint
                != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
            {
                return extentMergeResult;
            }
            tileIndex = tileIndex.wrapping_add(1);
        }
        currentFrameIndex = currentFrameIndex.wrapping_add(1);
    }
    return AVIF_RESULT_OK;
}
unsafe extern "C" fn avifDecoderPrepareSample(
    mut decoder: *mut avifDecoder,
    mut sample: *mut avifDecodeSample,
    mut partialByteCount: size_t,
) -> avifResult {
    if (*sample).data.size == 0 || (*sample).partialData != 0 {
        let mut bytesToRead: size_t = (*sample).size;
        if partialByteCount != 0 && bytesToRead > partialByteCount {
            bytesToRead = partialByteCount;
        }
        if (*sample).itemID != 0 {
            let mut item: *mut avifDecoderItem = 0 as *mut avifDecoderItem;
            let result__: avifResult = avifMetaFindOrCreateItem(
                (*(*decoder).data).meta,
                (*sample).itemID,
                &mut item,
            ) as avifResult;
            if result__ as core::ffi::c_uint
                != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
            {
                return result__;
            }
            let mut itemContents: avifROData = avifROData {
                data: 0 as *const uint8_t,
                size: 0,
            };
            let mut offset: size_t = (*sample).offset as size_t;
            let mut readResult: avifResult = avifDecoderItemRead(
                item,
                (*decoder).io,
                &mut itemContents,
                offset,
                bytesToRead,
                &mut (*decoder).diag,
            );
            if readResult as core::ffi::c_uint
                != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
            {
                return readResult;
            }
            (*sample).data = itemContents;
            (*sample).ownsData = AVIF_FALSE as avifBool;
            (*sample).partialData = (*item).partialMergedExtents;
        } else {
            let mut sampleContents: avifROData = avifROData {
                data: 0 as *const uint8_t,
                size: 0,
            };
            if (*(*decoder).io).sizeHint > 0 as uint64_t
                && (*sample).offset > (*(*decoder).io).sizeHint
            {
                return AVIF_RESULT_BMFF_PARSE_FAILED;
            }
            let mut readResult_0: avifResult = ((*(*decoder).io).read)
                .expect(
                    "non-null function pointer",
                )(
                (*decoder).io as *mut avifIO,
                0 as uint32_t,
                (*sample).offset,
                bytesToRead,
                &mut sampleContents,
            );
            if readResult_0 as core::ffi::c_uint
                != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
            {
                return readResult_0;
            }
            if sampleContents.size != bytesToRead {
                return AVIF_RESULT_TRUNCATED_DATA;
            }
            (*sample).ownsData = ((*(*decoder).io).persistent == 0) as core::ffi::c_int
                as avifBool;
            (*sample).partialData = (bytesToRead != (*sample).size) as core::ffi::c_int
                as avifBool;
            if (*(*decoder).io).persistent != 0 {
                (*sample).data = sampleContents;
            } else {
                let result___0: avifResult = avifRWDataSet(
                    &mut (*sample).data as *mut avifROData as *mut avifRWData,
                    sampleContents.data,
                    sampleContents.size,
                ) as avifResult;
                if result___0 as core::ffi::c_uint
                    != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
                {
                    return result___0;
                }
            }
        }
    }
    return AVIF_RESULT_OK;
}
unsafe extern "C" fn avifDecoderItemShouldBeSkipped(
    mut item: *const avifDecoderItem,
) -> avifBool {
    return ((*item).size == 0 || (*item).hasUnsupportedEssentialProperty != 0
        || avifGetCodecType(((*item).type_0).as_ptr()) as core::ffi::c_uint
            == AVIF_CODEC_TYPE_UNKNOWN as core::ffi::c_int as core::ffi::c_uint
            && memcmp(
                ((*item).type_0).as_ptr() as *const core::ffi::c_void,
                b"grid\0" as *const u8 as *const core::ffi::c_char
                    as *const core::ffi::c_void,
                4 as size_t,
            ) != 0
        || avifHasReference(
            &(*item).references,
            b"thmb\0" as *const u8 as *const core::ffi::c_char,
        ) != 0) as core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn avifDecoderParse(mut decoder: *mut avifDecoder) -> avifResult {
    avifDiagnosticsClearError(&mut (*decoder).diag);
    if (*decoder).imageContentToDecode
        & AVIF_IMAGE_CONTENT_COLOR_AND_ALPHA as core::ffi::c_int
            as avifImageContentTypeFlags
        == AVIF_IMAGE_CONTENT_ALPHA as core::ffi::c_int as avifImageContentTypeFlags
    {
        avifDiagnosticsPrintf(
            &mut (*decoder).diag as *mut avifDiagnostics,
            b"imageContentToDecode set to only alpha is not supported\0" as *const u8
                as *const core::ffi::c_char,
        );
        return AVIF_RESULT_NOT_IMPLEMENTED;
    }
    if ((*decoder).io).is_null() || ((*(*decoder).io).read).is_none() {
        return AVIF_RESULT_IO_NOT_SET;
    }
    avifDecoderCleanup(decoder);
    (*decoder).data = avifDecoderDataCreate() as *mut avifDecoderData;
    if ((*decoder).data).is_null() {
        return AVIF_RESULT_OUT_OF_MEMORY;
    }
    (*(*decoder).data).diag = &mut (*decoder).diag;
    let result__: avifResult = avifParse(decoder) as avifResult;
    if result__ as core::ffi::c_uint
        != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
    {
        return result__;
    }
    let mut data: *mut avifDecoderData = (*decoder).data as *mut avifDecoderData;
    let mut itemIndex: uint32_t = 0 as uint32_t;
    while itemIndex < (*(*data).meta).items.count {
        let mut item: *mut avifDecoderItem = *((*(*data).meta).items.item)
            .offset(itemIndex as isize);
        if !(avifDecoderItemShouldBeSkipped(item) != 0) {
            let mut ispeProp: *const avifProperty = avifPropertyArrayFind(
                &mut (*item).properties,
                b"ispe\0" as *const u8 as *const core::ffi::c_char,
            );
            if !ispeProp.is_null() {
                (*item).width = (*ispeProp).u.ispe.width;
                (*item).height = (*ispeProp).u.ispe.height;
                if (*item).width == 0 as uint32_t || (*item).height == 0 as uint32_t {
                    avifDiagnosticsPrintf(
                        (*data).diag,
                        b"Item ID [%u] has an invalid size [%ux%u]\0" as *const u8
                            as *const core::ffi::c_char,
                        (*item).id,
                        (*item).width,
                        (*item).height,
                    );
                    return AVIF_RESULT_BMFF_PARSE_FAILED;
                }
                if avifDimensionsTooLarge(
                    (*item).width,
                    (*item).height,
                    (*decoder).imageSizeLimit,
                    (*decoder).imageDimensionLimit,
                ) != 0
                {
                    avifDiagnosticsPrintf(
                        (*data).diag,
                        b"Item ID [%u] dimensions are too large [%ux%u]\0" as *const u8
                            as *const core::ffi::c_char,
                        (*item).id,
                        (*item).width,
                        (*item).height,
                    );
                    return AVIF_RESULT_BMFF_PARSE_FAILED;
                }
            } else {
                let mut auxCProp: *const avifProperty = avifPropertyArrayFind(
                    &mut (*item).properties,
                    b"auxC\0" as *const u8 as *const core::ffi::c_char,
                );
                if !auxCProp.is_null()
                    && isAlphaURN(((*auxCProp).u.auxC.auxType).as_ptr()) != 0
                {
                    if (*decoder).strictFlags
                        & AVIF_STRICT_ALPHA_ISPE_REQUIRED as core::ffi::c_int
                            as avifStrictFlags != 0
                    {
                        avifDiagnosticsPrintf(
                            (*data).diag,
                            b"[Strict] Alpha auxiliary image item ID [%u] is missing a mandatory ispe property\0"
                                as *const u8 as *const core::ffi::c_char,
                            (*item).id,
                        );
                        return AVIF_RESULT_BMFF_PARSE_FAILED;
                    }
                } else {
                    avifDiagnosticsPrintf(
                        (*data).diag,
                        b"Item ID [%u] is missing a mandatory ispe property\0"
                            as *const u8 as *const core::ffi::c_char,
                        (*item).id,
                    );
                    return AVIF_RESULT_BMFF_PARSE_FAILED;
                }
            }
        }
        itemIndex = itemIndex.wrapping_add(1);
    }
    return avifDecoderReset(decoder);
}
unsafe extern "C" fn avifCodecCreateInternal(
    mut choice: avifCodecChoice,
    mut tile: *const avifTile,
    mut diag: *mut avifDiagnostics,
    mut codec: *mut *mut avifCodec,
) -> avifResult {
    let codecTypeFromChoice: avifCodecType = avifCodecTypeFromChoice(
        choice,
        AVIF_CODEC_FLAG_CAN_DECODE as core::ffi::c_int as avifCodecFlags,
    ) as avifCodecType;
    if codecTypeFromChoice as core::ffi::c_uint
        == AVIF_CODEC_TYPE_UNKNOWN as core::ffi::c_int as core::ffi::c_uint
    {
        avifDiagnosticsPrintf(
            diag,
            b"Tile type is %s but there is no compatible codec available to decode it\0"
                as *const u8 as *const core::ffi::c_char,
            avifGetConfigurationPropertyName((*tile).codecType),
        );
        return AVIF_RESULT_NO_CODEC_AVAILABLE;
    } else if choice as core::ffi::c_uint
        != AVIF_CODEC_CHOICE_AUTO as core::ffi::c_int as core::ffi::c_uint
        && codecTypeFromChoice as core::ffi::c_uint
            != (*tile).codecType as core::ffi::c_uint
    {
        avifDiagnosticsPrintf(
            diag,
            b"Tile type is %s but incompatible %s codec was explicitly set as decoding implementation\0"
                as *const u8 as *const core::ffi::c_char,
            avifGetConfigurationPropertyName((*tile).codecType),
            avifCodecName(
                choice,
                AVIF_CODEC_FLAG_CAN_DECODE as core::ffi::c_int as avifCodecFlags,
            ),
        );
        return AVIF_RESULT_DECODE_COLOR_FAILED;
    }
    let result__: avifResult = avifCodecCreate(
        choice,
        AVIF_CODEC_FLAG_CAN_DECODE as core::ffi::c_int as avifCodecFlags,
        codec,
    ) as avifResult;
    if result__ as core::ffi::c_uint
        != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
    {
        return result__;
    }
    if (*codec).is_null() {
        return AVIF_RESULT_OUT_OF_MEMORY;
    }
    (**codec).diag = diag;
    (**codec).operatingPoint = (*tile).operatingPoint;
    (**codec).allLayers = (*(*tile).input).allLayers;
    return AVIF_RESULT_OK;
}
unsafe extern "C" fn avifTilesCanBeDecodedWithSameCodecInstance(
    mut data: *const avifDecoderData,
) -> avifBool {
    let mut numImageBuffers: int32_t = 0 as int32_t;
    let mut numStolenImageBuffers: int32_t = 0 as int32_t;
    let mut c: core::ffi::c_int = 0 as core::ffi::c_int;
    while c < AVIF_ITEM_CATEGORY_COUNT as core::ffi::c_int {
        if (*data).tileInfos[c as usize].tileCount > 0 as core::ffi::c_uint {
            numImageBuffers += 1;
        }
        if !(c >= AVIF_ITEM_SAMPLE_TRANSFORM_INPUT_0_COLOR as core::ffi::c_int
            && c
                <= AVIF_ITEM_SAMPLE_TRANSFORM_INPUT_0_ALPHA as core::ffi::c_int
                    + (AVIF_ITEM_SAMPLE_TRANSFORM_INPUT_0_ALPHA as core::ffi::c_int
                        - AVIF_ITEM_SAMPLE_TRANSFORM_INPUT_0_COLOR as core::ffi::c_int)
                    - 1 as core::ffi::c_int
            && (*data).tileInfos[c as usize].tileCount > 0 as core::ffi::c_uint)
        {
            if (*data).tileInfos[c as usize].tileCount == 1 as core::ffi::c_uint {
                numStolenImageBuffers += 1;
            }
        }
        c += 1;
    }
    if numStolenImageBuffers > 0 as int32_t && numImageBuffers > 1 as int32_t {
        return AVIF_FALSE;
    }
    let firstTileOperatingPoint: uint8_t = (*((*data).tiles.tile)
        .offset(0 as core::ffi::c_int as isize))
        .operatingPoint;
    let firstTileAllLayers: avifBool = (*(*((*data).tiles.tile)
        .offset(0 as core::ffi::c_int as isize))
        .input)
        .allLayers;
    let mut i: core::ffi::c_uint = 1 as core::ffi::c_uint;
    while (i as uint32_t) < (*data).tiles.count {
        let mut tile: *const avifTile = &mut *((*data).tiles.tile).offset(i as isize)
            as *mut avifTile;
        if (*tile).operatingPoint as core::ffi::c_int
            != firstTileOperatingPoint as core::ffi::c_int
            || (*(*tile).input).allLayers != firstTileAllLayers
        {
            return AVIF_FALSE;
        }
        i = i.wrapping_add(1);
    }
    return AVIF_TRUE;
}
unsafe extern "C" fn avifDecoderCreateCodecs(
    mut decoder: *mut avifDecoder,
) -> avifResult {
    let mut data: *mut avifDecoderData = (*decoder).data as *mut avifDecoderData;
    avifDecoderDataResetCodec(data);
    if (*data).source as core::ffi::c_uint
        == AVIF_DECODER_SOURCE_TRACKS as core::ffi::c_int as core::ffi::c_uint
    {
        let result__: avifResult = avifCodecCreateInternal(
            (*decoder).codecChoice,
            &mut *((*(*decoder).data).tiles.tile).offset(0 as core::ffi::c_int as isize),
            &mut (*decoder).diag,
            &mut (*data).codec,
        ) as avifResult;
        if result__ as core::ffi::c_uint
            != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
        {
            return result__;
        }
        let ref mut fresh1 = (*((*data).tiles.tile)
            .offset(0 as core::ffi::c_int as isize))
            .codec;
        *fresh1 = (*data).codec as *mut avifCodec;
        if (*data).tiles.count > 1 as uint32_t {
            let result___0: avifResult = avifCodecCreateInternal(
                (*decoder).codecChoice,
                &mut *((*(*decoder).data).tiles.tile)
                    .offset(1 as core::ffi::c_int as isize),
                &mut (*decoder).diag,
                &mut (*data).codecAlpha,
            ) as avifResult;
            if result___0 as core::ffi::c_uint
                != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
            {
                return result___0;
            }
            let ref mut fresh2 = (*((*data).tiles.tile)
                .offset(1 as core::ffi::c_int as isize))
                .codec;
            *fresh2 = (*data).codecAlpha as *mut avifCodec;
        }
    } else {
        let canUseSingleCodecInstance: avifBool = (((*data).tiles.count == 1 as uint32_t
            || (*decoder).imageCount == 1 as core::ffi::c_int
                && avifTilesCanBeDecodedWithSameCodecInstance(data) != 0)
            && (*data).sampleTransformNumInputImageItems as core::ffi::c_int
                == 0 as core::ffi::c_int) as core::ffi::c_int;
        if canUseSingleCodecInstance != 0 {
            let result___1: avifResult = avifCodecCreateInternal(
                (*decoder).codecChoice,
                &mut *((*(*decoder).data).tiles.tile)
                    .offset(0 as core::ffi::c_int as isize),
                &mut (*decoder).diag,
                &mut (*data).codec,
            ) as avifResult;
            if result___1 as core::ffi::c_uint
                != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
            {
                return result___1;
            }
            let mut i: core::ffi::c_uint = 0 as core::ffi::c_uint;
            while (i as uint32_t) < (*(*decoder).data).tiles.count {
                let ref mut fresh3 = (*((*(*decoder).data).tiles.tile)
                    .offset(i as isize))
                    .codec;
                *fresh3 = (*data).codec as *mut avifCodec;
                i = i.wrapping_add(1);
            }
        } else {
            let mut i_0: core::ffi::c_uint = 0 as core::ffi::c_uint;
            while (i_0 as uint32_t) < (*(*decoder).data).tiles.count {
                let mut tile: *mut avifTile = &mut *((*(*decoder).data).tiles.tile)
                    .offset(i_0 as isize) as *mut avifTile;
                let result___2: avifResult = avifCodecCreateInternal(
                    (*decoder).codecChoice,
                    tile,
                    &mut (*decoder).diag,
                    &mut (*tile).codec,
                ) as avifResult;
                if result___2 as core::ffi::c_uint
                    != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
                {
                    return result___2;
                }
                i_0 = i_0.wrapping_add(1);
            }
        }
    }
    return AVIF_RESULT_OK;
}
unsafe extern "C" fn avifMetaFindColorItem(
    mut meta: *mut avifMeta,
) -> *mut avifDecoderItem {
    let mut itemIndex: uint32_t = 0 as uint32_t;
    while itemIndex < (*meta).items.count {
        let mut item: *mut avifDecoderItem = *((*meta).items.item)
            .offset(itemIndex as isize);
        if !(avifDecoderItemShouldBeSkipped(item) != 0) {
            if (*item).id == (*meta).primaryItemID {
                return item;
            }
        }
        itemIndex = itemIndex.wrapping_add(1);
    }
    return 0 as *mut avifDecoderItem;
}
unsafe extern "C" fn avifDecoderItemIsAlphaAux(
    mut item: *const avifDecoderItem,
    mut colorItemId: uint32_t,
) -> avifBool {
    if avifReferences(
        &(*item).references,
        b"auxl\0" as *const u8 as *const core::ffi::c_char,
        colorItemId,
    ) == 0
    {
        return AVIF_FALSE;
    }
    let mut auxCProp: *const avifProperty = avifPropertyArrayFind(
        &(*item).properties,
        b"auxC\0" as *const u8 as *const core::ffi::c_char,
    );
    return (!auxCProp.is_null()
        && isAlphaURN(((*auxCProp).u.auxC.auxType).as_ptr()) != 0) as core::ffi::c_int;
}
unsafe extern "C" fn avifMetaFindAlphaItem(
    mut meta: *mut avifMeta,
    mut colorItem: *const avifDecoderItem,
    mut colorInfo: *const avifTileInfo,
    mut alphaItem: *mut *mut avifDecoderItem,
    mut alphaInfo: *mut avifTileInfo,
    mut isAlphaItemInInput: *mut avifBool,
) -> avifResult {
    let mut itemIndex: uint32_t = 0 as uint32_t;
    while itemIndex < (*meta).items.count {
        let mut item: *mut avifDecoderItem = *((*meta).items.item)
            .offset(itemIndex as isize);
        if !(avifDecoderItemShouldBeSkipped(item) != 0) {
            if avifDecoderItemIsAlphaAux(item, (*colorItem).id) != 0 {
                *alphaItem = item;
                *isAlphaItemInInput = AVIF_TRUE as avifBool;
                return AVIF_RESULT_OK;
            }
        }
        itemIndex = itemIndex.wrapping_add(1);
    }
    if memcmp(
        ((*colorItem).type_0).as_ptr() as *const core::ffi::c_void,
        b"grid\0" as *const u8 as *const core::ffi::c_char as *const core::ffi::c_void,
        4 as size_t,
    ) != 0
    {
        *alphaItem = 0 as *mut avifDecoderItem;
        *isAlphaItemInInput = AVIF_FALSE as avifBool;
        return AVIF_RESULT_OK;
    }
    let tileCount: uint32_t = ((*colorInfo).grid.rows)
        .wrapping_mul((*colorInfo).grid.columns);
    if tileCount == 0 as uint32_t {
        *alphaItem = 0 as *mut avifDecoderItem;
        *isAlphaItemInInput = AVIF_FALSE as avifBool;
        return AVIF_RESULT_OK;
    }
    let mut dimgIdxToAlphaItemIdx: *mut uint32_t = avifAlloc(
        (tileCount as size_t).wrapping_mul(::core::mem::size_of::<uint32_t>() as size_t),
    ) as *mut uint32_t;
    if dimgIdxToAlphaItemIdx.is_null() {
        return AVIF_RESULT_OUT_OF_MEMORY;
    }
    let itemIndexNotSet: uint32_t = UINT32_MAX as uint32_t;
    let mut dimgIdx: uint32_t = 0 as uint32_t;
    while dimgIdx < tileCount {
        *dimgIdxToAlphaItemIdx.offset(dimgIdx as isize) = itemIndexNotSet;
        dimgIdx = dimgIdx.wrapping_add(1);
    }
    let mut alphaItemCount: uint32_t = 0 as uint32_t;
    let mut i: uint32_t = 0 as uint32_t;
    while i < (*meta).items.count {
        let item_0: *const avifDecoderItem = *((*meta).items.item).offset(i as isize);
        let mut colorTileIdx: uint32_t = 0;
        if avifIsDimgInput(colorItem, (*item_0).id, &mut colorTileIdx) != 0 {
            let mut seenAlphaForCurrentItem: avifBool = AVIF_FALSE;
            let mut j: uint32_t = 0 as uint32_t;
            while j < (*meta).items.count {
                let mut auxlItem: *mut avifDecoderItem = *((*meta).items.item)
                    .offset(j as isize);
                if avifDecoderItemIsAlphaAux(auxlItem, (*item_0).id) != 0 {
                    if seenAlphaForCurrentItem != 0 || colorTileIdx >= tileCount
                        || *dimgIdxToAlphaItemIdx.offset(colorTileIdx as isize)
                            != itemIndexNotSet
                    {
                        avifFree(dimgIdxToAlphaItemIdx as *mut core::ffi::c_void);
                        return AVIF_RESULT_INVALID_IMAGE_GRID;
                    }
                    *dimgIdxToAlphaItemIdx.offset(colorTileIdx as isize) = j;
                    alphaItemCount = alphaItemCount.wrapping_add(1);
                    seenAlphaForCurrentItem = AVIF_TRUE as avifBool;
                }
                j = j.wrapping_add(1);
            }
            if seenAlphaForCurrentItem == 0 {
                avifFree(dimgIdxToAlphaItemIdx as *mut core::ffi::c_void);
                *alphaItem = 0 as *mut avifDecoderItem;
                *isAlphaItemInInput = AVIF_FALSE as avifBool;
                return AVIF_RESULT_OK;
            }
        }
        i = i.wrapping_add(1);
    }
    if alphaItemCount != tileCount {
        avifFree(dimgIdxToAlphaItemIdx as *mut core::ffi::c_void);
        return AVIF_RESULT_INVALID_IMAGE_GRID;
    }
    let mut result: avifResult = AVIF_RESULT_OK;
    if (*meta).items.count >= (UINT32_MAX as uint32_t).wrapping_sub(1 as uint32_t) {
        result = AVIF_RESULT_DECODE_ALPHA_FAILED;
    } else {
        let mut newItemID: uint32_t = 0 as uint32_t;
        let mut isUsed: avifBool = 0;
        loop {
            newItemID = newItemID.wrapping_add(1);
            isUsed = AVIF_FALSE as avifBool;
            let mut i_0: uint32_t = 0 as uint32_t;
            while i_0 < (*meta).items.count {
                if (**((*meta).items.item).offset(i_0 as isize)).id == newItemID {
                    isUsed = AVIF_TRUE as avifBool;
                    break;
                } else {
                    i_0 = i_0.wrapping_add(1);
                }
            }
            if !(isUsed != 0 && newItemID != 0 as uint32_t) {
                break;
            }
        }
        result = avifMetaFindOrCreateItem(meta, newItemID, alphaItem);
    }
    if result as core::ffi::c_uint
        != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
    {
        avifFree(dimgIdxToAlphaItemIdx as *mut core::ffi::c_void);
        return result;
    }
    memcpy(
        ((**alphaItem).type_0).as_mut_ptr() as *mut core::ffi::c_void,
        b"grid\0" as *const u8 as *const core::ffi::c_char as *const core::ffi::c_void,
        4 as size_t,
    );
    (**alphaItem).width = (*colorItem).width;
    (**alphaItem).height = (*colorItem).height;
    let mut dimgIdx_0: uint32_t = 0 as uint32_t;
    while dimgIdx_0 < tileCount {
        if *dimgIdxToAlphaItemIdx.offset(dimgIdx_0 as isize) >= (*meta).items.count {
            avifFree(dimgIdxToAlphaItemIdx as *mut core::ffi::c_void);
            'c_14346: {
                __assert_fail(
                    b"0\0" as *const u8 as *const core::ffi::c_char,
                    b"/home/runner/work/libavif-rs/libavif-rs/upstream/libavif/src/read.c\0"
                        as *const u8 as *const core::ffi::c_char,
                    5709 as core::ffi::c_uint,
                    (::core::mem::transmute::<
                        [u8; 140],
                        [core::ffi::c_char; 140],
                    >(
                        *b"avifResult avifMetaFindAlphaItem(avifMeta *, const avifDecoderItem *, const avifTileInfo *, avifDecoderItem **, avifTileInfo *, avifBool *)\0",
                    ))
                        .as_ptr(),
                );
            };
        }
        let mut alphaTileItem: *mut avifDecoderItem = *((*meta).items.item)
            .offset(*dimgIdxToAlphaItemIdx.offset(dimgIdx_0 as isize) as isize);
        let addResult: avifResult = avifAddDimgInput(*alphaItem, alphaTileItem)
            as avifResult;
        if addResult as core::ffi::c_uint
            != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
        {
            avifFree(dimgIdxToAlphaItemIdx as *mut core::ffi::c_void);
            return addResult;
        }
        dimgIdx_0 = dimgIdx_0.wrapping_add(1);
    }
    avifFree(dimgIdxToAlphaItemIdx as *mut core::ffi::c_void);
    *isAlphaItemInInput = AVIF_FALSE as avifBool;
    (*alphaInfo).grid = (*colorInfo).grid;
    return AVIF_RESULT_OK;
}
unsafe extern "C" fn avifReadColorNclxProperty(
    mut properties: *const avifPropertyArray,
    mut colorPrimaries: *mut avifColorPrimaries,
    mut transferCharacteristics: *mut avifTransferCharacteristics,
    mut matrixCoefficients: *mut avifMatrixCoefficients,
    mut yuvRange: *mut avifRange,
    mut cicpSet: *mut avifBool,
) -> avifResult {
    'c_6492: {
        if cicpSet.is_null() || *cicpSet == 0 as core::ffi::c_int {} else {
            __assert_fail(
                b"cicpSet == NULL || *cicpSet == AVIF_FALSE\0" as *const u8
                    as *const core::ffi::c_char,
                b"/home/runner/work/libavif-rs/libavif-rs/upstream/libavif/src/read.c\0"
                    as *const u8 as *const core::ffi::c_char,
                5744 as core::ffi::c_uint,
                (::core::mem::transmute::<
                    [u8; 168],
                    [core::ffi::c_char; 168],
                >(
                    *b"avifResult avifReadColorNclxProperty(const avifPropertyArray *, avifColorPrimaries *, avifTransferCharacteristics *, avifMatrixCoefficients *, avifRange *, avifBool *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    let mut colrNCLXSeen: avifBool = AVIF_FALSE;
    let mut propertyIndex: uint32_t = 0 as uint32_t;
    while propertyIndex < (*properties).count {
        let mut prop: *mut avifProperty = &mut *((*properties).prop)
            .offset(propertyIndex as isize) as *mut avifProperty;
        if memcmp(
            ((*prop).type_0).as_mut_ptr() as *const core::ffi::c_void,
            b"colr\0" as *const u8 as *const core::ffi::c_char
                as *const core::ffi::c_void,
            4 as size_t,
        ) == 0 && (*prop).u.colr.hasNCLX != 0
        {
            if colrNCLXSeen != 0 {
                return AVIF_RESULT_BMFF_PARSE_FAILED;
            }
            colrNCLXSeen = AVIF_TRUE as avifBool;
            if !cicpSet.is_null() {
                *cicpSet = AVIF_TRUE as avifBool;
            }
            *colorPrimaries = (*prop).u.colr.colorPrimaries;
            *transferCharacteristics = (*prop).u.colr.transferCharacteristics;
            *matrixCoefficients = (*prop).u.colr.matrixCoefficients;
            *yuvRange = (*prop).u.colr.range;
        }
        propertyIndex = propertyIndex.wrapping_add(1);
    }
    return AVIF_RESULT_OK;
}
unsafe extern "C" fn avifReadColorProperties(
    mut io: *mut avifIO,
    mut properties: *const avifPropertyArray,
    mut icc: *mut avifRWData,
    mut colorPrimaries: *mut avifColorPrimaries,
    mut transferCharacteristics: *mut avifTransferCharacteristics,
    mut matrixCoefficients: *mut avifMatrixCoefficients,
    mut yuvRange: *mut avifRange,
    mut cicpSet: *mut avifBool,
) -> avifResult {
    let mut colrICCSeen: avifBool = AVIF_FALSE;
    let mut propertyIndex: uint32_t = 0 as uint32_t;
    while propertyIndex < (*properties).count {
        let mut prop: *mut avifProperty = &mut *((*properties).prop)
            .offset(propertyIndex as isize) as *mut avifProperty;
        if memcmp(
            ((*prop).type_0).as_mut_ptr() as *const core::ffi::c_void,
            b"colr\0" as *const u8 as *const core::ffi::c_char
                as *const core::ffi::c_void,
            4 as size_t,
        ) == 0 && (*prop).u.colr.hasICC != 0
        {
            if colrICCSeen != 0 {
                return AVIF_RESULT_BMFF_PARSE_FAILED;
            }
            if !icc.is_null() {
                let mut iccRead: avifROData = avifROData {
                    data: 0 as *const uint8_t,
                    size: 0,
                };
                let result__: avifResult = ((*io).read)
                    .expect(
                        "non-null function pointer",
                    )(
                    io as *mut avifIO,
                    0 as uint32_t,
                    (*prop).u.colr.iccOffset,
                    (*prop).u.colr.iccSize,
                    &mut iccRead,
                ) as avifResult;
                if result__ as core::ffi::c_uint
                    != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
                {
                    return result__;
                }
                let result___0: avifResult = avifRWDataSet(
                    icc,
                    iccRead.data,
                    iccRead.size,
                ) as avifResult;
                if result___0 as core::ffi::c_uint
                    != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
                {
                    return result___0;
                }
            }
            colrICCSeen = AVIF_TRUE as avifBool;
        }
        propertyIndex = propertyIndex.wrapping_add(1);
    }
    return avifReadColorNclxProperty(
        properties,
        colorPrimaries,
        transferCharacteristics,
        matrixCoefficients,
        yuvRange,
        cicpSet,
    );
}
unsafe extern "C" fn avifDecoderDataFindToneMappedImageItem(
    mut data: *const avifDecoderData,
    mut colorItem: *const avifDecoderItem,
    mut toneMappedImageItem: *mut *mut avifDecoderItem,
    mut gainMapItemID: *mut uint32_t,
) -> avifResult {
    let mut itemIndex: uint32_t = 0 as uint32_t;
    while itemIndex < (*(*data).meta).items.count {
        let mut item: *mut avifDecoderItem = *((*(*data).meta).items.item)
            .offset(itemIndex as isize);
        if !((*item).size == 0 || (*item).hasUnsupportedEssentialProperty != 0
            || avifHasReference(
                &mut (*item).references,
                b"thmb\0" as *const u8 as *const core::ffi::c_char,
            ) != 0)
        {
            if memcmp(
                ((*item).type_0).as_mut_ptr() as *const core::ffi::c_void,
                b"tmap\0" as *const u8 as *const core::ffi::c_char
                    as *const core::ffi::c_void,
                4 as size_t,
            ) == 0
            {
                let mut dimgItemIDs: [uint32_t; 2] = [
                    0 as core::ffi::c_int as uint32_t,
                    0 as core::ffi::c_int as uint32_t,
                ];
                let mut numDimgItemIDs: uint32_t = 0 as uint32_t;
                let mut otherItemIndex: uint32_t = 0 as uint32_t;
                while otherItemIndex < (*(*data).meta).items.count {
                    let mut otherItem: *mut avifDecoderItem = *((*(*data).meta)
                        .items
                        .item)
                        .offset(otherItemIndex as isize);
                    let mut otherDimgIdx: uint32_t = 0;
                    if !(avifIsDimgInput(item, (*otherItem).id, &mut otherDimgIdx) == 0)
                    {
                        if otherDimgIdx < 2 as uint32_t {
                            'c_17240: {
                                if dimgItemIDs[otherDimgIdx as usize] == 0 as uint32_t
                                {} else {
                                    __assert_fail(
                                        b"dimgItemIDs[otherDimgIdx] == 0\0" as *const u8
                                            as *const core::ffi::c_char,
                                        b"/home/runner/work/libavif-rs/libavif-rs/upstream/libavif/src/read.c\0"
                                            as *const u8 as *const core::ffi::c_char,
                                        5829 as core::ffi::c_uint,
                                        (::core::mem::transmute::<
                                            [u8; 132],
                                            [core::ffi::c_char; 132],
                                        >(
                                            *b"avifResult avifDecoderDataFindToneMappedImageItem(const avifDecoderData *, const avifDecoderItem *, avifDecoderItem **, uint32_t *)\0",
                                        ))
                                            .as_ptr(),
                                    );
                                }
                            };
                            dimgItemIDs[otherDimgIdx as usize] = (*otherItem).id;
                        }
                        numDimgItemIDs = numDimgItemIDs.wrapping_add(1);
                    }
                    otherItemIndex = otherItemIndex.wrapping_add(1);
                }
                if numDimgItemIDs != 2 as uint32_t
                    || dimgItemIDs[0 as core::ffi::c_int as usize] == 0 as uint32_t
                    || dimgItemIDs[1 as core::ffi::c_int as usize] == 0 as uint32_t
                {
                    avifDiagnosticsPrintf(
                        (*data).diag,
                        b"box[dimg] for 'tmap' item %d must have exactly 2 entries with distinct ids\0"
                            as *const u8 as *const core::ffi::c_char,
                        (*item).id,
                    );
                    return AVIF_RESULT_INVALID_TONE_MAPPED_IMAGE;
                }
                if !(dimgItemIDs[0 as core::ffi::c_int as usize] != (*colorItem).id) {
                    *toneMappedImageItem = item;
                    *gainMapItemID = dimgItemIDs[1 as core::ffi::c_int as usize];
                    return AVIF_RESULT_OK;
                }
            }
        }
        itemIndex = itemIndex.wrapping_add(1);
    }
    *toneMappedImageItem = 0 as *mut avifDecoderItem;
    *gainMapItemID = 0 as uint32_t;
    return AVIF_RESULT_OK;
}
unsafe extern "C" fn avifIsPreferredAlternativeTo(
    mut data: *const avifDecoderData,
    mut id1: uint32_t,
    mut id2: uint32_t,
) -> avifBool {
    let mut i: uint32_t = 0 as uint32_t;
    while i < (*(*data).meta).entityToGroups.count {
        let mut group: *mut avifEntityToGroup = &mut *((*(*data).meta)
            .entityToGroups
            .groups)
            .offset(i as isize) as *mut avifEntityToGroup;
        if !(memcmp(
            ((*group).groupingType).as_mut_ptr() as *const core::ffi::c_void,
            b"altr\0" as *const u8 as *const core::ffi::c_char
                as *const core::ffi::c_void,
            4 as size_t,
        ) != 0 as core::ffi::c_int)
        {
            let mut id1Found: avifBool = AVIF_FALSE;
            let mut j: uint32_t = 0 as uint32_t;
            while j < (*group).entityIDs.count {
                if *((*group).entityIDs.ids).offset(j as isize) == id1 {
                    id1Found = AVIF_TRUE as avifBool;
                } else if *((*group).entityIDs.ids).offset(j as isize) == id2 {
                    return id1Found
                }
                j = j.wrapping_add(1);
            }
        }
        i = i.wrapping_add(1);
    }
    return AVIF_FALSE;
}
unsafe extern "C" fn avifDecoderFindGainMapItem(
    mut decoder: *const avifDecoder,
    mut colorItem: *const avifDecoderItem,
    mut gainMapItem: *mut *mut avifDecoderItem,
    mut gainMapCodecType: *mut avifCodecType,
) -> avifResult {
    *gainMapItem = 0 as *mut avifDecoderItem;
    *gainMapCodecType = AVIF_CODEC_TYPE_UNKNOWN;
    let mut data: *mut avifDecoderData = (*decoder).data as *mut avifDecoderData;
    let mut gainMapItemID: uint32_t = 0;
    let mut toneMappedImageItemTmp: *mut avifDecoderItem = 0 as *mut avifDecoderItem;
    let result__: avifResult = avifDecoderDataFindToneMappedImageItem(
        data,
        colorItem,
        &mut toneMappedImageItemTmp,
        &mut gainMapItemID,
    ) as avifResult;
    if result__ as core::ffi::c_uint
        != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
    {
        return result__;
    }
    if toneMappedImageItemTmp.is_null() || gainMapItemID == 0 {
        return AVIF_RESULT_OK;
    }
    if avifIsPreferredAlternativeTo(data, (*toneMappedImageItemTmp).id, (*colorItem).id)
        == 0
    {
        return AVIF_RESULT_OK;
    }
    let mut tmapData: avifROData = avifROData {
        data: 0 as *const uint8_t,
        size: 0,
    };
    let result___0: avifResult = avifDecoderItemRead(
        toneMappedImageItemTmp,
        (*decoder).io,
        &mut tmapData,
        0 as size_t,
        0 as size_t,
        (*data).diag,
    ) as avifResult;
    if result___0 as core::ffi::c_uint
        != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
    {
        return result___0;
    }
    let mut gainMapTmp: avifGainMap = avifGainMap {
        image: 0 as *mut avifImage,
        gainMapMin: [avifSignedFraction { n: 0, d: 0 }; 3],
        gainMapMax: [avifSignedFraction { n: 0, d: 0 }; 3],
        gainMapGamma: [avifUnsignedFraction { n: 0, d: 0 }; 3],
        baseOffset: [avifSignedFraction { n: 0, d: 0 }; 3],
        alternateOffset: [avifSignedFraction { n: 0, d: 0 }; 3],
        baseHdrHeadroom: avifUnsignedFraction { n: 0, d: 0 },
        alternateHdrHeadroom: avifUnsignedFraction { n: 0, d: 0 },
        useBaseColorSpace: 0,
        altICC: avifRWData {
            data: 0 as *mut uint8_t,
            size: 0,
        },
        altColorPrimaries: 0,
        altTransferCharacteristics: 0,
        altMatrixCoefficients: 0,
        altYUVRange: AVIF_RANGE_LIMITED,
        altDepth: 0,
        altPlaneCount: 0,
        altCLLI: avifContentLightLevelInformationBox {
            maxCLL: 0,
            maxPALL: 0,
        },
    };
    avifGainMapSetDefaults(&mut gainMapTmp);
    let mut result: avifResult = avifParseToneMappedImageBox(
        &mut gainMapTmp,
        tmapData.data,
        tmapData.size,
        (*data).diag,
    );
    if result as core::ffi::c_uint
        == AVIF_RESULT_NOT_IMPLEMENTED as core::ffi::c_int as core::ffi::c_uint
    {
        return AVIF_RESULT_OK;
    }
    let result___1: avifResult = result;
    if result___1 as core::ffi::c_uint
        != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
    {
        return result___1;
    }
    let mut gainMapItemTmp: *mut avifDecoderItem = 0 as *mut avifDecoderItem;
    let result___2: avifResult = avifMetaFindOrCreateItem(
        (*data).meta,
        gainMapItemID,
        &mut gainMapItemTmp,
    ) as avifResult;
    if result___2 as core::ffi::c_uint
        != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
    {
        return result___2;
    }
    if avifDecoderItemShouldBeSkipped(gainMapItemTmp) != 0 {
        return AVIF_RESULT_NOT_IMPLEMENTED;
    }
    let mut gainMapCodecTypeTmp: avifCodecType = AVIF_CODEC_TYPE_UNKNOWN;
    result = avifDecoderItemReadAndParse(
        decoder,
        gainMapItemTmp,
        AVIF_TRUE,
        &mut (*((*data).tileInfos)
            .as_mut_ptr()
            .offset(AVIF_ITEM_GAIN_MAP as core::ffi::c_int as isize))
            .grid,
        &mut gainMapCodecTypeTmp,
    );
    if result as core::ffi::c_uint
        == AVIF_RESULT_NOT_IMPLEMENTED as core::ffi::c_int as core::ffi::c_uint
    {
        return AVIF_RESULT_OK;
    }
    let result___3: avifResult = result;
    if result___3 as core::ffi::c_uint
        != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
    {
        return result___3;
    }
    result = avifReadColorProperties(
        (*decoder).io,
        &mut (*toneMappedImageItemTmp).properties,
        if (*decoder).ignoreICC != 0 {
            0 as *mut avifRWData
        } else {
            &mut gainMapTmp.altICC
        },
        &mut gainMapTmp.altColorPrimaries,
        &mut gainMapTmp.altTransferCharacteristics,
        &mut gainMapTmp.altMatrixCoefficients,
        &mut gainMapTmp.altYUVRange,
        0 as *mut avifBool,
    );
    if result as core::ffi::c_uint
        != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
    {
        avifRWDataFree(&mut gainMapTmp.altICC);
        return result;
    }
    let mut clliProp: *const avifProperty = avifPropertyArrayFind(
        &mut (*toneMappedImageItemTmp).properties,
        b"clli\0" as *const u8 as *const core::ffi::c_char,
    );
    if !clliProp.is_null() {
        gainMapTmp.altCLLI = (*clliProp).u.clli;
    }
    let mut pixiProp: *const avifProperty = avifPropertyArrayFind(
        &mut (*toneMappedImageItemTmp).properties,
        b"pixi\0" as *const u8 as *const core::ffi::c_char,
    );
    if !pixiProp.is_null() {
        gainMapTmp.altPlaneCount = (*pixiProp).u.pixi.planeCount as uint32_t;
        gainMapTmp.altDepth = (*pixiProp)
            .u
            .pixi
            .planeDepths[0 as core::ffi::c_int as usize] as uint32_t;
    }
    let mut ispeProp: *const avifProperty = avifPropertyArrayFind(
        &mut (*toneMappedImageItemTmp).properties,
        b"ispe\0" as *const u8 as *const core::ffi::c_char,
    );
    if ispeProp.is_null() {
        avifDiagnosticsPrintf(
            (*data).diag,
            b"Box[tmap] missing mandatory ispe property\0" as *const u8
                as *const core::ffi::c_char,
        );
        avifRWDataFree(&mut gainMapTmp.altICC);
        return AVIF_RESULT_BMFF_PARSE_FAILED;
    }
    if (*ispeProp).u.ispe.width != (*colorItem).width
        || (*ispeProp).u.ispe.height != (*colorItem).height
    {
        avifDiagnosticsPrintf(
            (*data).diag,
            b"Box[tmap] ispe property width/height does not match base image\0"
                as *const u8 as *const core::ffi::c_char,
        );
        avifRWDataFree(&mut gainMapTmp.altICC);
        return AVIF_RESULT_BMFF_PARSE_FAILED;
    }
    if !(avifPropertyArrayFind(
        &mut (*toneMappedImageItemTmp).properties,
        b"pasp\0" as *const u8 as *const core::ffi::c_char,
    ))
        .is_null()
        || !(avifPropertyArrayFind(
            &mut (*toneMappedImageItemTmp).properties,
            b"clap\0" as *const u8 as *const core::ffi::c_char,
        ))
            .is_null()
        || !(avifPropertyArrayFind(
            &mut (*toneMappedImageItemTmp).properties,
            b"irot\0" as *const u8 as *const core::ffi::c_char,
        ))
            .is_null()
        || !(avifPropertyArrayFind(
            &mut (*toneMappedImageItemTmp).properties,
            b"imir\0" as *const u8 as *const core::ffi::c_char,
        ))
            .is_null()
    {
        avifDiagnosticsPrintf(
            (*data).diag,
            b"Box[tmap] 'pasp', 'clap', 'irot' and 'imir' properties must be associated with base and gain map items instead of 'tmap'\0"
                as *const u8 as *const core::ffi::c_char,
        );
        avifRWDataFree(&mut gainMapTmp.altICC);
        return AVIF_RESULT_INVALID_TONE_MAPPED_IMAGE;
    }
    let mut colorPrimaries: avifColorPrimaries = AVIF_COLOR_PRIMARIES_UNSPECIFIED
        as core::ffi::c_int as avifColorPrimaries;
    let mut transferCharacteristics: avifTransferCharacteristics = AVIF_TRANSFER_CHARACTERISTICS_UNSPECIFIED
        as core::ffi::c_int as avifTransferCharacteristics;
    let mut matrixCoefficients: avifMatrixCoefficients = AVIF_MATRIX_COEFFICIENTS_UNSPECIFIED
        as core::ffi::c_int as avifMatrixCoefficients;
    let mut yuvRange: avifRange = AVIF_RANGE_FULL;
    let mut cicpSet: avifBool = AVIF_FALSE;
    result = avifReadColorNclxProperty(
        &mut (*gainMapItemTmp).properties,
        &mut colorPrimaries,
        &mut transferCharacteristics,
        &mut matrixCoefficients,
        &mut yuvRange,
        &mut cicpSet,
    );
    if result as core::ffi::c_uint
        != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
    {
        avifRWDataFree(&mut gainMapTmp.altICC);
        return result;
    }
    (*(*decoder).image).gainMap = avifGainMapCreate();
    if ((*(*decoder).image).gainMap).is_null() {
        avifRWDataFree(&mut gainMapTmp.altICC);
        return AVIF_RESULT_OUT_OF_MEMORY;
    }
    if (*decoder).imageContentToDecode
        & AVIF_IMAGE_CONTENT_GAIN_MAP as core::ffi::c_int as avifImageContentTypeFlags
        != 0
    {
        let mut image: *mut avifImage = avifImageCreateEmpty();
        if image.is_null() {
            avifRWDataFree(&mut gainMapTmp.altICC);
            return AVIF_RESULT_OUT_OF_MEMORY;
        }
        if cicpSet != 0 {
            (*image).colorPrimaries = colorPrimaries;
            (*image).transferCharacteristics = transferCharacteristics;
            (*image).matrixCoefficients = matrixCoefficients;
            (*image).yuvRange = yuvRange;
        }
        gainMapTmp.image = image as *mut avifImage;
    }
    *(*(*decoder).image).gainMap = gainMapTmp;
    *gainMapItem = gainMapItemTmp;
    *gainMapCodecType = gainMapCodecTypeTmp;
    return AVIF_RESULT_OK;
}
unsafe extern "C" fn avifDecoderCheckAlphaProperties(
    mut decoder: *mut avifDecoder,
    mut alphaProperties: *const avifPropertyArray,
) -> avifResult {
    let mut image: *const avifImage = (*decoder).image;
    let mut clapProp: *const avifProperty = avifPropertyArrayFind(
        alphaProperties,
        b"clap\0" as *const u8 as *const core::ffi::c_char,
    );
    let mut irotProp: *const avifProperty = avifPropertyArrayFind(
        alphaProperties,
        b"irot\0" as *const u8 as *const core::ffi::c_char,
    );
    let mut imirProp: *const avifProperty = avifPropertyArrayFind(
        alphaProperties,
        b"imir\0" as *const u8 as *const core::ffi::c_char,
    );
    if clapProp.is_null() && irotProp.is_null() && imirProp.is_null() {
        return AVIF_RESULT_OK;
    }
    if clapProp.is_null() as core::ffi::c_int
        != ((*image).transformFlags
            & AVIF_TRANSFORM_CLAP as core::ffi::c_int as avifTransformFlags == 0)
            as core::ffi::c_int
        || !clapProp.is_null()
            && ((*clapProp).u.clap.widthN != (*image).clap.widthN
                || (*clapProp).u.clap.widthD != (*image).clap.widthD
                || (*clapProp).u.clap.heightN != (*image).clap.heightN
                || (*clapProp).u.clap.heightD != (*image).clap.heightD
                || (*clapProp).u.clap.horizOffN != (*image).clap.horizOffN
                || (*clapProp).u.clap.horizOffD != (*image).clap.horizOffD
                || (*clapProp).u.clap.vertOffN != (*image).clap.vertOffN
                || (*clapProp).u.clap.vertOffD != (*image).clap.vertOffD)
    {
        avifDiagnosticsPrintf(
            &mut (*decoder).diag as *mut avifDiagnostics,
            b"Clean aperture property mismatch between alpha auxiliary image item and color item\0"
                as *const u8 as *const core::ffi::c_char,
        );
        return AVIF_RESULT_NOT_IMPLEMENTED;
    }
    if irotProp.is_null() as core::ffi::c_int
        != ((*image).transformFlags
            & AVIF_TRANSFORM_IROT as core::ffi::c_int as avifTransformFlags == 0)
            as core::ffi::c_int
        || !irotProp.is_null()
            && (*irotProp).u.irot.angle as core::ffi::c_int
                != (*image).irot.angle as core::ffi::c_int
    {
        avifDiagnosticsPrintf(
            &mut (*decoder).diag as *mut avifDiagnostics,
            b"Rotation property mismatch between alpha auxiliary image item and color item\0"
                as *const u8 as *const core::ffi::c_char,
        );
        return AVIF_RESULT_NOT_IMPLEMENTED;
    }
    if imirProp.is_null() as core::ffi::c_int
        != ((*image).transformFlags
            & AVIF_TRANSFORM_IMIR as core::ffi::c_int as avifTransformFlags == 0)
            as core::ffi::c_int
        || !imirProp.is_null()
            && (*imirProp).u.imir.axis as core::ffi::c_int
                != (*image).imir.axis as core::ffi::c_int
    {
        avifDiagnosticsPrintf(
            &mut (*decoder).diag as *mut avifDiagnostics,
            b"Mirroring property mismatch between alpha auxiliary image item and color item\0"
                as *const u8 as *const core::ffi::c_char,
        );
        return AVIF_RESULT_NOT_IMPLEMENTED;
    }
    return AVIF_RESULT_OK;
}
unsafe extern "C" fn avifDecoderCheckGainMapProperties(
    mut decoder: *mut avifDecoder,
    mut gainMapProperties: *const avifPropertyArray,
) -> avifResult {
    let mut image: *const avifImage = (*decoder).image;
    let mut paspProp: *const avifProperty = avifPropertyArrayFind(
        gainMapProperties,
        b"pasp\0" as *const u8 as *const core::ffi::c_char,
    );
    if paspProp.is_null() as core::ffi::c_int
        != ((*image).transformFlags
            & AVIF_TRANSFORM_PASP as core::ffi::c_int as avifTransformFlags == 0)
            as core::ffi::c_int
        || !paspProp.is_null()
            && ((*paspProp).u.pasp.hSpacing != (*image).pasp.hSpacing
                || (*paspProp).u.pasp.vSpacing != (*image).pasp.vSpacing)
    {
        avifDiagnosticsPrintf(
            &mut (*decoder).diag as *mut avifDiagnostics,
            b"Pixel aspect ratio property mismatch between input items of tone-mapping derived image item\0"
                as *const u8 as *const core::ffi::c_char,
        );
        return AVIF_RESULT_DECODE_GAIN_MAP_FAILED;
    }
    let mut clapProp: *const avifProperty = avifPropertyArrayFind(
        gainMapProperties,
        b"clap\0" as *const u8 as *const core::ffi::c_char,
    );
    if clapProp.is_null() as core::ffi::c_int
        != ((*image).transformFlags
            & AVIF_TRANSFORM_CLAP as core::ffi::c_int as avifTransformFlags == 0)
            as core::ffi::c_int
        || !clapProp.is_null()
            && ((*clapProp).u.clap.widthN != (*image).clap.widthN
                || (*clapProp).u.clap.widthD != (*image).clap.widthD
                || (*clapProp).u.clap.heightN != (*image).clap.heightN
                || (*clapProp).u.clap.heightD != (*image).clap.heightD
                || (*clapProp).u.clap.horizOffN != (*image).clap.horizOffN
                || (*clapProp).u.clap.horizOffD != (*image).clap.horizOffD
                || (*clapProp).u.clap.vertOffN != (*image).clap.vertOffN
                || (*clapProp).u.clap.vertOffD != (*image).clap.vertOffD)
    {
        avifDiagnosticsPrintf(
            &mut (*decoder).diag as *mut avifDiagnostics,
            b"Clean aperture property mismatch between input items of tone-mapping derived image item\0"
                as *const u8 as *const core::ffi::c_char,
        );
        return AVIF_RESULT_DECODE_GAIN_MAP_FAILED;
    }
    let mut irotProp: *const avifProperty = avifPropertyArrayFind(
        gainMapProperties,
        b"irot\0" as *const u8 as *const core::ffi::c_char,
    );
    if irotProp.is_null() as core::ffi::c_int
        != ((*image).transformFlags
            & AVIF_TRANSFORM_IROT as core::ffi::c_int as avifTransformFlags == 0)
            as core::ffi::c_int
        || !irotProp.is_null()
            && (*irotProp).u.irot.angle as core::ffi::c_int
                != (*image).irot.angle as core::ffi::c_int
    {
        avifDiagnosticsPrintf(
            &mut (*decoder).diag as *mut avifDiagnostics,
            b"Rotation property mismatch between input items of tone-mapping derived image item\0"
                as *const u8 as *const core::ffi::c_char,
        );
        return AVIF_RESULT_DECODE_GAIN_MAP_FAILED;
    }
    let mut imirProp: *const avifProperty = avifPropertyArrayFind(
        gainMapProperties,
        b"imir\0" as *const u8 as *const core::ffi::c_char,
    );
    if imirProp.is_null() as core::ffi::c_int
        != ((*image).transformFlags
            & AVIF_TRANSFORM_IMIR as core::ffi::c_int as avifTransformFlags == 0)
            as core::ffi::c_int
        || !imirProp.is_null()
            && (*imirProp).u.imir.axis as core::ffi::c_int
                != (*image).imir.axis as core::ffi::c_int
    {
        avifDiagnosticsPrintf(
            &mut (*decoder).diag as *mut avifDiagnostics,
            b"Mirroring property mismatch between input items of tone-mapping derived image item\0"
                as *const u8 as *const core::ffi::c_char,
        );
        return AVIF_RESULT_DECODE_GAIN_MAP_FAILED;
    }
    return AVIF_RESULT_OK;
}
unsafe extern "C" fn avifDecoderDataFindSampleTransformImageItem(
    mut data: *mut avifDecoderData,
) -> *mut avifDecoderItem {
    let mut itemIndex: uint32_t = 0 as uint32_t;
    while itemIndex < (*(*data).meta).items.count {
        let mut item: *mut avifDecoderItem = *((*(*data).meta).items.item)
            .offset(itemIndex as isize);
        if memcmp(
            ((*item).type_0).as_mut_ptr() as *const core::ffi::c_void,
            b"sato\0" as *const u8 as *const core::ffi::c_char
                as *const core::ffi::c_void,
            4 as size_t,
        ) == 0 && (*item).id != (*(*data).meta).primaryItemID
            && (*item).size != 0 as size_t
            && (*item).hasUnsupportedEssentialProperty == 0
            && avifHasReference(
                &mut (*item).references,
                b"thmb\0" as *const u8 as *const core::ffi::c_char,
            ) == 0
            && avifIsPreferredAlternativeTo(
                data,
                (*item).id,
                (*(*data).meta).primaryItemID,
            ) != 0
        {
            return item;
        }
        itemIndex = itemIndex.wrapping_add(1);
    }
    return 0 as *mut avifDecoderItem;
}
unsafe extern "C" fn avifDecoderGenerateImageTiles(
    mut decoder: *mut avifDecoder,
    mut info: *mut avifTileInfo,
    mut item: *mut avifDecoderItem,
    mut itemCategory: avifItemCategory,
) -> avifResult {
    let previousTileCount: uint32_t = (*(*decoder).data).tiles.count;
    if (*info).grid.rows > 0 as uint32_t && (*info).grid.columns > 0 as uint32_t {
        let numTiles: uint32_t = ((*info).grid.rows).wrapping_mul((*info).grid.columns);
        let mut dimgIdxToItemIdx: *mut uint32_t = avifAlloc(
            (numTiles as size_t)
                .wrapping_mul(::core::mem::size_of::<uint32_t>() as size_t),
        ) as *mut uint32_t;
        if dimgIdxToItemIdx.is_null() {
            return AVIF_RESULT_OUT_OF_MEMORY;
        }
        let mut result: avifResult = avifFillDimgIdxToItemIdxArray(
            dimgIdxToItemIdx,
            numTiles,
            item,
        );
        if result as core::ffi::c_uint
            == AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
        {
            result = avifDecoderGenerateImageGridTiles(
                decoder,
                item,
                itemCategory,
                dimgIdxToItemIdx,
                numTiles,
            );
        }
        avifFree(dimgIdxToItemIdx as *mut core::ffi::c_void);
        let result__: avifResult = result;
        if result__ as core::ffi::c_uint
            != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
        {
            return result__;
        }
    } else {
        if !((*item).size != 0 as size_t) {
            return AVIF_RESULT_MISSING_IMAGE_ITEM;
        }
        let codecType: avifCodecType = avifGetCodecType(((*item).type_0).as_mut_ptr())
            as avifCodecType;
        'c_9064: {
            if codecType as core::ffi::c_uint
                != AVIF_CODEC_TYPE_UNKNOWN as core::ffi::c_int as core::ffi::c_uint
            {} else {
                __assert_fail(
                    b"codecType != AVIF_CODEC_TYPE_UNKNOWN\0" as *const u8
                        as *const core::ffi::c_char,
                    b"/home/runner/work/libavif-rs/libavif-rs/upstream/libavif/src/read.c\0"
                        as *const u8 as *const core::ffi::c_char,
                    6150 as core::ffi::c_uint,
                    (::core::mem::transmute::<
                        [u8; 109],
                        [core::ffi::c_char; 109],
                    >(
                        *b"avifResult avifDecoderGenerateImageTiles(avifDecoder *, avifTileInfo *, avifDecoderItem *, avifItemCategory)\0",
                    ))
                        .as_ptr(),
                );
            }
        };
        let mut tile: *mut avifTile = avifDecoderDataCreateTile(
            (*decoder).data as *mut avifDecoderData,
            codecType,
            (*item).width,
            (*item).height,
            avifDecoderItemOperatingPoint(item),
        );
        if tile.is_null() {
            return AVIF_RESULT_OUT_OF_MEMORY;
        }
        let result___0: avifResult = avifCodecDecodeInputFillFromDecoderItem(
            (*tile).input,
            item,
            (*decoder).allowProgressive,
            (*decoder).imageCountLimit,
            (*(*decoder).io).sizeHint,
            &mut (*decoder).diag,
        ) as avifResult;
        if result___0 as core::ffi::c_uint
            != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
        {
            return result___0;
        }
        (*(*tile).input).itemCategory = itemCategory;
    }
    (*info).tileCount = ((*(*decoder).data).tiles.count).wrapping_sub(previousTileCount)
        as core::ffi::c_uint;
    return AVIF_RESULT_OK;
}
unsafe extern "C" fn avifReadCodecConfigProperty(
    mut image: *mut avifImage,
    mut properties: *const avifPropertyArray,
    mut codecType: avifCodecType,
) -> avifResult {
    let mut configProp: *const avifProperty = avifPropertyArrayFind(
        properties,
        avifGetConfigurationPropertyName(codecType),
    );
    if !configProp.is_null() {
        (*image).depth = avifCodecConfigurationBoxGetDepth(&(*configProp).u.av1C);
        if (*configProp).u.av1C.monochrome != 0 {
            (*image).yuvFormat = AVIF_PIXEL_FORMAT_YUV400;
        } else if (*configProp).u.av1C.chromaSubsamplingX as core::ffi::c_int != 0
            && (*configProp).u.av1C.chromaSubsamplingY as core::ffi::c_int != 0
        {
            (*image).yuvFormat = AVIF_PIXEL_FORMAT_YUV420;
        } else if (*configProp).u.av1C.chromaSubsamplingX != 0 {
            (*image).yuvFormat = AVIF_PIXEL_FORMAT_YUV422;
        } else {
            (*image).yuvFormat = AVIF_PIXEL_FORMAT_YUV444;
        }
        (*image).yuvChromaSamplePosition = (*configProp).u.av1C.chromaSamplePosition
            as avifChromaSamplePosition;
    } else {
        return AVIF_RESULT_BMFF_PARSE_FAILED
    }
    return AVIF_RESULT_OK;
}
#[no_mangle]
pub unsafe extern "C" fn avifDecoderReset(mut decoder: *mut avifDecoder) -> avifResult {
    avifDiagnosticsClearError(&mut (*decoder).diag);
    let mut data: *mut avifDecoderData = (*decoder).data as *mut avifDecoderData;
    if data.is_null() {
        return AVIF_RESULT_OK;
    }
    let mut c: core::ffi::c_int = 0 as core::ffi::c_int;
    while c < AVIF_ITEM_CATEGORY_COUNT as core::ffi::c_int {
        memset(
            &mut (*((*data).tileInfos).as_mut_ptr().offset(c as isize)).grid
                as *mut avifImageGrid as *mut core::ffi::c_void,
            0 as core::ffi::c_int,
            ::core::mem::size_of::<avifImageGrid>() as size_t,
        );
        c += 1;
    }
    avifDecoderDataClearTiles(data);
    (*data).sampleTransformNumInputImageItems = 0 as uint8_t;
    avifArrayDestroy(
        &mut (*(*data).meta).sampleTransformExpression
            as *mut avifSampleTransformExpression as *mut core::ffi::c_void,
    );
    (*(*data).meta).sampleTransformDepth = 0 as uint32_t;
    if !((*decoder).image).is_null() {
        avifImageDestroy((*decoder).image);
    }
    (*decoder).image = avifImageCreateEmpty();
    if ((*decoder).image).is_null() {
        return AVIF_RESULT_OUT_OF_MEMORY;
    }
    (*decoder).progressiveState = AVIF_PROGRESSIVE_STATE_UNAVAILABLE;
    (*data).cicpSet = AVIF_FALSE as avifBool;
    memset(
        &mut (*decoder).ioStats as *mut avifIOStats as *mut core::ffi::c_void,
        0 as core::ffi::c_int,
        ::core::mem::size_of::<avifIOStats>() as size_t,
    );
    if (*decoder).imageContentToDecode
        & AVIF_IMAGE_CONTENT_COLOR_AND_ALPHA as core::ffi::c_int
            as avifImageContentTypeFlags
        == AVIF_IMAGE_CONTENT_ALPHA as core::ffi::c_int as avifImageContentTypeFlags
    {
        avifDiagnosticsPrintf(
            &mut (*decoder).diag as *mut avifDiagnostics,
            b"imageContentToDecode set to only alpha is not supported\0" as *const u8
                as *const core::ffi::c_char,
        );
        return AVIF_RESULT_NOT_IMPLEMENTED;
    }
    (*data).sourceSampleTable = 0 as *const avifSampleTable;
    if (*decoder).requestedSource as core::ffi::c_uint
        == AVIF_DECODER_SOURCE_AUTO as core::ffi::c_int as core::ffi::c_uint
    {
        if memcmp(
            ((*data).majorBrand).as_mut_ptr() as *const core::ffi::c_void,
            b"avis\0" as *const u8 as *const core::ffi::c_char
                as *const core::ffi::c_void,
            4 as size_t,
        ) == 0
        {
            (*data).source = AVIF_DECODER_SOURCE_TRACKS;
        } else if memcmp(
            ((*data).majorBrand).as_mut_ptr() as *const core::ffi::c_void,
            b"avif\0" as *const u8 as *const core::ffi::c_char
                as *const core::ffi::c_void,
            4 as size_t,
        ) == 0
        {
            (*data).source = AVIF_DECODER_SOURCE_PRIMARY_ITEM;
        } else if (*data).tracks.count > 0 as uint32_t {
            (*data).source = AVIF_DECODER_SOURCE_TRACKS;
        } else {
            (*data).source = AVIF_DECODER_SOURCE_PRIMARY_ITEM;
        }
    } else {
        (*data).source = (*decoder).requestedSource;
    }
    let mut colorCodecType: avifCodecType = AVIF_CODEC_TYPE_UNKNOWN;
    let mut colorProperties: *const avifPropertyArray = 0 as *const avifPropertyArray;
    let mut alphaProperties: *const avifPropertyArray = 0 as *const avifPropertyArray;
    let mut gainMapProperties: *const avifPropertyArray = 0 as *const avifPropertyArray;
    if (*data).source as core::ffi::c_uint
        == AVIF_DECODER_SOURCE_TRACKS as core::ffi::c_int as core::ffi::c_uint
    {
        let mut colorTrack: *mut avifTrack = 0 as *mut avifTrack;
        let mut alphaTrack: *mut avifTrack = 0 as *mut avifTrack;
        let mut colorTrackIndex: uint32_t = 0 as uint32_t;
        while colorTrackIndex < (*data).tracks.count {
            let mut track: *mut avifTrack = &mut *((*data).tracks.track)
                .offset(colorTrackIndex as isize) as *mut avifTrack;
            if !((*track).sampleTable).is_null() {
                if !((*track).id == 0) {
                    if !((*(*track).sampleTable).chunks.count == 0) {
                        colorCodecType = avifSampleTableGetCodecType(
                            (*track).sampleTable,
                        );
                        if !(colorCodecType as core::ffi::c_uint
                            == AVIF_CODEC_TYPE_UNKNOWN as core::ffi::c_int
                                as core::ffi::c_uint)
                        {
                            if !(avifHasReference(
                                &mut (*track).references,
                                b"auxl\0" as *const u8 as *const core::ffi::c_char,
                            ) != 0)
                            {
                                break;
                            }
                        }
                    }
                }
            }
            colorTrackIndex = colorTrackIndex.wrapping_add(1);
        }
        if colorTrackIndex == (*data).tracks.count {
            avifDiagnosticsPrintf(
                &mut (*decoder).diag as *mut avifDiagnostics,
                b"Failed to find AV1 color track\0" as *const u8
                    as *const core::ffi::c_char,
            );
            return AVIF_RESULT_NO_CONTENT;
        }
        colorTrack = &mut *((*data).tracks.track).offset(colorTrackIndex as isize)
            as *mut avifTrack;
        colorProperties = avifSampleTableGetProperties(
            (*colorTrack).sampleTable,
            colorCodecType,
        );
        if colorProperties.is_null() {
            avifDiagnosticsPrintf(
                &mut (*decoder).diag as *mut avifDiagnostics,
                b"Failed to find AV1 color track's color properties\0" as *const u8
                    as *const core::ffi::c_char,
            );
            return AVIF_RESULT_BMFF_PARSE_FAILED;
        }
        if !((*colorTrack).meta).is_null() {
            let mut findResult: avifResult = avifDecoderFindMetadata(
                decoder,
                (*colorTrack).meta as *mut avifMeta,
                (*decoder).image,
                0 as uint32_t,
            );
            if findResult as core::ffi::c_uint
                != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
            {
                return findResult;
            }
        }
        let mut alphaTrackIndex: uint32_t = 0 as uint32_t;
        let mut alphaCodecType: avifCodecType = AVIF_CODEC_TYPE_UNKNOWN;
        while alphaTrackIndex < (*data).tracks.count {
            let mut track_0: *mut avifTrack = &mut *((*data).tracks.track)
                .offset(alphaTrackIndex as isize) as *mut avifTrack;
            if !((*track_0).sampleTable).is_null() {
                if !((*track_0).id == 0) {
                    if !((*(*track_0).sampleTable).chunks.count == 0) {
                        alphaCodecType = avifSampleTableGetCodecType(
                            (*track_0).sampleTable,
                        );
                        if !(alphaCodecType as core::ffi::c_uint
                            == AVIF_CODEC_TYPE_UNKNOWN as core::ffi::c_int
                                as core::ffi::c_uint)
                        {
                            let mut properties: *const avifPropertyArray = avifSampleTableGetProperties(
                                (*track_0).sampleTable,
                                alphaCodecType,
                            );
                            let mut auxiProp: *const avifProperty = if !properties
                                .is_null()
                            {
                                avifPropertyArrayFind(
                                    properties,
                                    b"auxi\0" as *const u8 as *const core::ffi::c_char,
                                )
                            } else {
                                0 as *const avifProperty
                            };
                            if !(!auxiProp.is_null()
                                && isAlphaURN(((*auxiProp).u.auxC.auxType).as_ptr()) == 0)
                            {
                                if avifReferences(
                                    &mut (*track_0).references,
                                    b"auxl\0" as *const u8 as *const core::ffi::c_char,
                                    (*colorTrack).id,
                                ) != 0
                                {
                                    alphaProperties = properties;
                                    break;
                                }
                            }
                        }
                    }
                }
            }
            alphaTrackIndex = alphaTrackIndex.wrapping_add(1);
        }
        if alphaTrackIndex != (*data).tracks.count {
            alphaTrack = &mut *((*data).tracks.track).offset(alphaTrackIndex as isize)
                as *mut avifTrack;
        }
        let operatingPoint: uint8_t = 0 as uint8_t;
        let mut colorTile: *mut avifTile = 0 as *mut avifTile;
        if (*decoder).imageContentToDecode
            & AVIF_IMAGE_CONTENT_COLOR as core::ffi::c_int as avifImageContentTypeFlags
            != 0
        {
            colorTile = avifDecoderDataCreateTile(
                data,
                colorCodecType,
                (*colorTrack).width,
                (*colorTrack).height,
                operatingPoint,
            );
            if colorTile.is_null() {
                return AVIF_RESULT_OUT_OF_MEMORY;
            }
            let result__: avifResult = avifCodecDecodeInputFillFromSampleTable(
                (*colorTile).input,
                (*colorTrack).sampleTable,
                (*decoder).imageCountLimit,
                (*(*decoder).io).sizeHint,
                (*data).diag,
            ) as avifResult;
            if result__ as core::ffi::c_uint
                != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
            {
                return result__;
            }
            (*data).tileInfos[AVIF_ITEM_COLOR as core::ffi::c_int as usize].tileCount = 1
                as core::ffi::c_uint;
        }
        if !alphaTrack.is_null()
            && (*decoder).imageContentToDecode
                & AVIF_IMAGE_CONTENT_ALPHA as core::ffi::c_int
                    as avifImageContentTypeFlags != 0
        {
            let mut alphaTile: *mut avifTile = avifDecoderDataCreateTile(
                data,
                alphaCodecType,
                (*alphaTrack).width,
                (*alphaTrack).height,
                operatingPoint,
            );
            if alphaTile.is_null() {
                return AVIF_RESULT_OUT_OF_MEMORY;
            }
            let result___0: avifResult = avifCodecDecodeInputFillFromSampleTable(
                (*alphaTile).input,
                (*alphaTrack).sampleTable,
                (*decoder).imageCountLimit,
                (*(*decoder).io).sizeHint,
                (*data).diag,
            ) as avifResult;
            if result___0 as core::ffi::c_uint
                != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
            {
                return result___0;
            }
            (*(*alphaTile).input).itemCategory = AVIF_ITEM_ALPHA;
            (*data).tileInfos[AVIF_ITEM_ALPHA as core::ffi::c_int as usize].tileCount = 1
                as core::ffi::c_uint;
        }
        (*data).sourceSampleTable = (*colorTrack).sampleTable;
        (*decoder).imageIndex = -(1 as core::ffi::c_int);
        let mut imageCount: uint32_t = 0;
        if !colorTile.is_null() {
            imageCount = (*(*colorTile).input).samples.count;
        } else {
            imageCount = 0 as uint32_t;
            let mut chunkIndex: uint32_t = 0 as uint32_t;
            while chunkIndex < (*(*colorTrack).sampleTable).chunks.count {
                let mut sampleCount: uint32_t = avifGetSampleCountOfChunk(
                    &mut (*(*colorTrack).sampleTable).sampleToChunks,
                    chunkIndex,
                );
                if sampleCount == 0 as uint32_t {
                    avifDiagnosticsPrintf(
                        (*data).diag,
                        b"Sample table contains a chunk with 0 samples\0" as *const u8
                            as *const core::ffi::c_char,
                    );
                    return AVIF_RESULT_BMFF_PARSE_FAILED;
                }
                if imageCount > (UINT32_MAX as uint32_t).wrapping_sub(sampleCount) {
                    avifDiagnosticsPrintf(
                        (*data).diag,
                        b"Total number of samples exceeds UINT32_MAX\0" as *const u8
                            as *const core::ffi::c_char,
                    );
                    return AVIF_RESULT_BMFF_PARSE_FAILED;
                }
                imageCount = imageCount.wrapping_add(sampleCount);
                chunkIndex = chunkIndex.wrapping_add(1);
            }
        }
        if imageCount > INT_MAX as uint32_t {
            avifDiagnosticsPrintf(
                (*data).diag,
                b"Total number of samples exceeds INT_MAX\0" as *const u8
                    as *const core::ffi::c_char,
            );
            return AVIF_RESULT_BMFF_PARSE_FAILED;
        }
        (*decoder).imageCount = imageCount as core::ffi::c_int;
        (*decoder).timescale = (*colorTrack).mediaTimescale as uint64_t;
        (*decoder).durationInTimescales = (*colorTrack).mediaDuration;
        if (*colorTrack).mediaTimescale != 0 {
            (*decoder).duration = (*decoder).durationInTimescales as core::ffi::c_double
                / (*colorTrack).mediaTimescale as core::ffi::c_double;
        } else {
            (*decoder).duration = 0 as core::ffi::c_int as core::ffi::c_double;
        }
        (*decoder).repetitionCount = (*colorTrack).repetitionCount;
        memset(
            &mut (*decoder).imageTiming as *mut avifImageTiming
                as *mut core::ffi::c_void,
            0 as core::ffi::c_int,
            ::core::mem::size_of::<avifImageTiming>() as size_t,
        );
        (*(*decoder).image).width = (*colorTrack).width;
        (*(*decoder).image).height = (*colorTrack).height;
        (*decoder).alphaPresent = (alphaTrack != NULL as *mut avifTrack)
            as core::ffi::c_int as avifBool;
        (*(*decoder).image).alphaPremultiplied = ((*decoder).alphaPresent != 0
            && avifReferences(
                &mut (*colorTrack).references,
                b"prem\0" as *const u8 as *const core::ffi::c_char,
                (*alphaTrack).id,
            ) != 0) as core::ffi::c_int as avifBool;
    } else {
        if (*(*data).meta).primaryItemID == 0 as uint32_t {
            avifDiagnosticsPrintf(
                &mut (*decoder).diag as *mut avifDiagnostics,
                b"Primary item not specified\0" as *const u8 as *const core::ffi::c_char,
            );
            return AVIF_RESULT_MISSING_IMAGE_ITEM;
        }
        let mut mainItems: [*mut avifDecoderItem; 8] = [0 as *mut avifDecoderItem; 8];
        let mut codecType: [avifCodecType; 8] = [AVIF_CODEC_TYPE_UNKNOWN; 8];
        let mut c_0: core::ffi::c_int = 0 as core::ffi::c_int;
        while c_0 < AVIF_ITEM_CATEGORY_COUNT as core::ffi::c_int {
            mainItems[c_0 as usize] = 0 as *mut avifDecoderItem;
            codecType[c_0 as usize] = AVIF_CODEC_TYPE_UNKNOWN;
            c_0 += 1;
        }
        mainItems[AVIF_ITEM_COLOR as core::ffi::c_int as usize] = avifMetaFindColorItem(
            (*data).meta,
        );
        if (mainItems[AVIF_ITEM_COLOR as core::ffi::c_int as usize]).is_null() {
            avifDiagnosticsPrintf(
                &mut (*decoder).diag as *mut avifDiagnostics,
                b"Primary item not found\0" as *const u8 as *const core::ffi::c_char,
            );
            return AVIF_RESULT_MISSING_IMAGE_ITEM;
        }
        let result___1: avifResult = avifDecoderItemReadAndParse(
            decoder,
            mainItems[AVIF_ITEM_COLOR as core::ffi::c_int as usize],
            1 as avifBool,
            &mut (*((*data).tileInfos)
                .as_mut_ptr()
                .offset(AVIF_ITEM_COLOR as core::ffi::c_int as isize))
                .grid,
            &mut *codecType
                .as_mut_ptr()
                .offset(AVIF_ITEM_COLOR as core::ffi::c_int as isize),
        ) as avifResult;
        if result___1 as core::ffi::c_uint
            != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
        {
            return result___1;
        }
        colorProperties = &mut (**mainItems
            .as_mut_ptr()
            .offset(AVIF_ITEM_COLOR as core::ffi::c_int as isize))
            .properties;
        colorCodecType = codecType[AVIF_ITEM_COLOR as core::ffi::c_int as usize];
        let mut isAlphaItemInInput: avifBool = 0;
        let result___2: avifResult = avifMetaFindAlphaItem(
            (*data).meta,
            mainItems[AVIF_ITEM_COLOR as core::ffi::c_int as usize],
            &mut *((*data).tileInfos)
                .as_mut_ptr()
                .offset(AVIF_ITEM_COLOR as core::ffi::c_int as isize),
            &mut *mainItems
                .as_mut_ptr()
                .offset(AVIF_ITEM_ALPHA as core::ffi::c_int as isize),
            &mut *((*data).tileInfos)
                .as_mut_ptr()
                .offset(AVIF_ITEM_ALPHA as core::ffi::c_int as isize),
            &mut isAlphaItemInInput,
        ) as avifResult;
        if result___2 as core::ffi::c_uint
            != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
        {
            return result___2;
        }
        if !(mainItems[AVIF_ITEM_ALPHA as core::ffi::c_int as usize]).is_null() {
            let result___3: avifResult = avifDecoderItemReadAndParse(
                decoder,
                mainItems[AVIF_ITEM_ALPHA as core::ffi::c_int as usize],
                isAlphaItemInInput,
                &mut (*((*data).tileInfos)
                    .as_mut_ptr()
                    .offset(AVIF_ITEM_ALPHA as core::ffi::c_int as isize))
                    .grid,
                &mut *codecType
                    .as_mut_ptr()
                    .offset(AVIF_ITEM_ALPHA as core::ffi::c_int as isize),
            ) as avifResult;
            if result___3 as core::ffi::c_uint
                != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
            {
                return result___3;
            }
        }
        if avifBrandArrayHasBrand(
            &mut (*data).compatibleBrands,
            b"tmap\0" as *const u8 as *const core::ffi::c_char,
        ) != 0
        {
            let mut gainMapItem: *mut avifDecoderItem = 0 as *mut avifDecoderItem;
            let mut gainMapCodecType: avifCodecType = AVIF_CODEC_TYPE_UNKNOWN;
            let result___4: avifResult = avifDecoderFindGainMapItem(
                decoder,
                mainItems[AVIF_ITEM_COLOR as core::ffi::c_int as usize],
                &mut gainMapItem,
                &mut gainMapCodecType,
            ) as avifResult;
            if result___4 as core::ffi::c_uint
                != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
            {
                return result___4;
            }
            if !gainMapItem.is_null()
                && (*decoder).imageContentToDecode
                    & AVIF_IMAGE_CONTENT_GAIN_MAP as core::ffi::c_int
                        as avifImageContentTypeFlags != 0
            {
                mainItems[AVIF_ITEM_GAIN_MAP as core::ffi::c_int as usize] = gainMapItem;
                codecType[AVIF_ITEM_GAIN_MAP as core::ffi::c_int as usize] = gainMapCodecType;
            }
        }
        let sampleTransformItem: *mut avifDecoderItem = avifDecoderDataFindSampleTransformImageItem(
            data,
        ) as *mut avifDecoderItem;
        if (*decoder).imageContentToDecode
            & AVIF_IMAGE_CONTENT_COLOR as core::ffi::c_int as avifImageContentTypeFlags
            != 0
            && (*decoder).imageContentToDecode
                & AVIF_IMAGE_CONTENT_SAMPLE_TRANSFORMS as core::ffi::c_int
                    as avifImageContentTypeFlags != 0 && !sampleTransformItem.is_null()
        {
            'c_15425: {
                if (*data).sampleTransformNumInputImageItems as core::ffi::c_int
                    == 0 as core::ffi::c_int
                {} else {
                    __assert_fail(
                        b"data->sampleTransformNumInputImageItems == 0\0" as *const u8
                            as *const core::ffi::c_char,
                        b"/home/runner/work/libavif-rs/libavif-rs/upstream/libavif/src/read.c\0"
                            as *const u8 as *const core::ffi::c_char,
                        6485 as core::ffi::c_uint,
                        (::core::mem::transmute::<
                            [u8; 43],
                            [core::ffi::c_char; 43],
                        >(*b"avifResult avifDecoderReset(avifDecoder *)\0"))
                            .as_ptr(),
                    );
                }
            };
            let mut i: uint32_t = 0 as uint32_t;
            while i < (*(*data).meta).items.count {
                let mut inputImageItem: *mut avifDecoderItem = *((*(*data).meta)
                    .items
                    .item)
                    .offset(i as isize);
                if avifIsDimgInput(
                    sampleTransformItem,
                    (*inputImageItem).id,
                    0 as *mut uint32_t,
                ) != 0
                {
                    (*data).sampleTransformNumInputImageItems = ((*data)
                        .sampleTransformNumInputImageItems)
                        .wrapping_add(1);
                }
                i = i.wrapping_add(1);
            }
            if (*data).sampleTransformNumInputImageItems as core::ffi::c_int
                == 0 as core::ffi::c_int
            {
                avifDiagnosticsPrintf(
                    (*data).diag,
                    b"Box[sato] item %u has no input image items\0" as *const u8
                        as *const core::ffi::c_char,
                    (*sampleTransformItem).id,
                );
                return AVIF_RESULT_BMFF_PARSE_FAILED;
            }
            if (*data).sampleTransformNumInputImageItems as core::ffi::c_int
                > 32 as core::ffi::c_int
            {
                avifDiagnosticsPrintf(
                    (*data).diag,
                    b"Box[sato] too many input items, format allows up to 32, got %d\0"
                        as *const u8 as *const core::ffi::c_char,
                    (*data).sampleTransformNumInputImageItems as core::ffi::c_int,
                );
                return AVIF_RESULT_BMFF_PARSE_FAILED;
            }
            if !((*data).sampleTransformNumInputImageItems as core::ffi::c_int
                <= 1 as core::ffi::c_int
                    + (AVIF_ITEM_SAMPLE_TRANSFORM_INPUT_0_ALPHA as core::ffi::c_int
                        - AVIF_ITEM_SAMPLE_TRANSFORM_INPUT_0_COLOR as core::ffi::c_int))
            {
                return AVIF_RESULT_NOT_IMPLEMENTED;
            }
            let mut numExtraInputImageItems: uint32_t = 0 as uint32_t;
            let mut i_0: uint32_t = 0 as uint32_t;
            while i_0 < (*(*data).meta).items.count {
                let mut inputImageItem_0: *mut avifDecoderItem = *((*(*data).meta)
                    .items
                    .item)
                    .offset(i_0 as isize);
                let mut inputDimgIdx: uint32_t = 0;
                if !(avifIsDimgInput(
                    sampleTransformItem,
                    (*inputImageItem_0).id,
                    &mut inputDimgIdx,
                ) == 0)
                {
                    if avifDecoderItemShouldBeSkipped(inputImageItem_0) != 0 {
                        avifDiagnosticsPrintf(
                            (*data).diag,
                            b"Box[sato] input item %u is not a supported image type\0"
                                as *const u8 as *const core::ffi::c_char,
                            (*inputImageItem_0).id,
                        );
                        return AVIF_RESULT_DECODE_SAMPLE_TRANSFORM_FAILED;
                    }
                    'c_15200: {
                        if inputDimgIdx
                            < (1 as core::ffi::c_int
                                + (AVIF_ITEM_SAMPLE_TRANSFORM_INPUT_0_ALPHA
                                    as core::ffi::c_int
                                    - AVIF_ITEM_SAMPLE_TRANSFORM_INPUT_0_COLOR
                                        as core::ffi::c_int)) as uint32_t
                        {} else {
                            __assert_fail(
                                b"inputDimgIdx < (1 + (AVIF_ITEM_SAMPLE_TRANSFORM_INPUT_0_ALPHA - AVIF_ITEM_SAMPLE_TRANSFORM_INPUT_0_COLOR))\0"
                                    as *const u8 as *const core::ffi::c_char,
                                b"/home/runner/work/libavif-rs/libavif-rs/upstream/libavif/src/read.c\0"
                                    as *const u8 as *const core::ffi::c_char,
                                6521 as core::ffi::c_uint,
                                (::core::mem::transmute::<
                                    [u8; 43],
                                    [core::ffi::c_char; 43],
                                >(*b"avifResult avifDecoderReset(avifDecoder *)\0"))
                                    .as_ptr(),
                            );
                        }
                    };
                    let mut category: *mut avifItemCategory = &mut *((*data)
                        .sampleTransformInputImageItems)
                        .as_mut_ptr()
                        .offset(inputDimgIdx as isize) as *mut avifItemCategory;
                    let mut foundItem: avifBool = AVIF_FALSE;
                    let mut c_1: core::ffi::c_int = AVIF_ITEM_COLOR as core::ffi::c_int;
                    while c_1 < AVIF_ITEM_CATEGORY_COUNT as core::ffi::c_int {
                        if !(mainItems[c_1 as usize]).is_null()
                            && (*inputImageItem_0).id == (*mainItems[c_1 as usize]).id
                        {
                            *category = c_1 as avifItemCategory;
                            if !(*category as core::ffi::c_uint
                                == AVIF_ITEM_COLOR as core::ffi::c_int as core::ffi::c_uint)
                            {
                                return AVIF_RESULT_NOT_IMPLEMENTED;
                            }
                            foundItem = AVIF_TRUE as avifBool;
                            break;
                        } else {
                            c_1 += 1;
                        }
                    }
                    if !(foundItem != 0) {
                        if !(numExtraInputImageItems
                            < (AVIF_ITEM_SAMPLE_TRANSFORM_INPUT_0_ALPHA
                                as core::ffi::c_int
                                - AVIF_ITEM_SAMPLE_TRANSFORM_INPUT_0_COLOR
                                    as core::ffi::c_int) as uint32_t)
                        {
                            return AVIF_RESULT_NOT_IMPLEMENTED;
                        }
                        *category = (AVIF_ITEM_SAMPLE_TRANSFORM_INPUT_0_COLOR
                            as core::ffi::c_int as uint32_t)
                            .wrapping_add(numExtraInputImageItems) as avifItemCategory;
                        let alphaCategory: avifItemCategory = (AVIF_ITEM_SAMPLE_TRANSFORM_INPUT_0_ALPHA
                            as core::ffi::c_int as uint32_t)
                            .wrapping_add(numExtraInputImageItems) as avifItemCategory;
                        mainItems[*category as usize] = inputImageItem_0;
                        numExtraInputImageItems = numExtraInputImageItems
                            .wrapping_add(1);
                        let result___5: avifResult = avifDecoderItemReadAndParse(
                            decoder,
                            inputImageItem_0,
                            1 as avifBool,
                            &mut (*((*data).tileInfos)
                                .as_mut_ptr()
                                .offset(*category as isize))
                                .grid,
                            &mut *codecType.as_mut_ptr().offset(*category as isize),
                        ) as avifResult;
                        if result___5 as core::ffi::c_uint
                            != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
                        {
                            return result___5;
                        }
                        if !((*decoder).imageContentToDecode
                            & AVIF_IMAGE_CONTENT_ALPHA as core::ffi::c_int
                                as avifImageContentTypeFlags == 0)
                        {
                            let mut isAlphaInputImageItemInInput: avifBool = AVIF_FALSE;
                            let result___6: avifResult = avifMetaFindAlphaItem(
                                (*data).meta,
                                mainItems[*category as usize],
                                &mut *((*data).tileInfos)
                                    .as_mut_ptr()
                                    .offset(*category as isize),
                                &mut *mainItems.as_mut_ptr().offset(alphaCategory as isize),
                                &mut *((*data).tileInfos)
                                    .as_mut_ptr()
                                    .offset(alphaCategory as isize),
                                &mut isAlphaInputImageItemInInput,
                            ) as avifResult;
                            if result___6 as core::ffi::c_uint
                                != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
                            {
                                return result___6;
                            }
                            if !((mainItems[alphaCategory as usize]).is_null()
                                as core::ffi::c_int
                                == (mainItems[AVIF_ITEM_ALPHA as core::ffi::c_int as usize])
                                    .is_null() as core::ffi::c_int)
                            {
                                return AVIF_RESULT_NOT_IMPLEMENTED;
                            }
                            if !(mainItems[alphaCategory as usize]).is_null() {
                                if !(isAlphaInputImageItemInInput == isAlphaItemInInput) {
                                    return AVIF_RESULT_NOT_IMPLEMENTED;
                                }
                                if !(avifReferences(
                                    &mut (**mainItems.as_mut_ptr().offset(*category as isize))
                                        .references,
                                    b"prem\0" as *const u8 as *const core::ffi::c_char,
                                    (*mainItems[alphaCategory as usize]).id,
                                )
                                    == avifReferences(
                                        &mut (**mainItems
                                            .as_mut_ptr()
                                            .offset(AVIF_ITEM_COLOR as core::ffi::c_int as isize))
                                            .references,
                                        b"prem\0" as *const u8 as *const core::ffi::c_char,
                                        (*mainItems[AVIF_ITEM_ALPHA as core::ffi::c_int as usize])
                                            .id,
                                    ))
                                {
                                    return AVIF_RESULT_NOT_IMPLEMENTED;
                                }
                                let result___7: avifResult = avifDecoderItemReadAndParse(
                                    decoder,
                                    mainItems[alphaCategory as usize],
                                    isAlphaInputImageItemInInput,
                                    &mut (*((*data).tileInfos)
                                        .as_mut_ptr()
                                        .offset(alphaCategory as isize))
                                        .grid,
                                    &mut *codecType.as_mut_ptr().offset(alphaCategory as isize),
                                ) as avifResult;
                                if result___7 as core::ffi::c_uint
                                    != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
                                {
                                    return result___7;
                                }
                            }
                        }
                    }
                }
                i_0 = i_0.wrapping_add(1);
            }
            'c_13025: {
                if ((*(*data).meta).sampleTransformExpression.tokens).is_null() {} else {
                    __assert_fail(
                        b"data->meta->sampleTransformExpression.tokens == ((void*)0)\0"
                            as *const u8 as *const core::ffi::c_char,
                        b"/home/runner/work/libavif-rs/libavif-rs/upstream/libavif/src/read.c\0"
                            as *const u8 as *const core::ffi::c_char,
                        6574 as core::ffi::c_uint,
                        (::core::mem::transmute::<
                            [u8; 43],
                            [core::ffi::c_char; 43],
                        >(*b"avifResult avifDecoderReset(avifDecoder *)\0"))
                            .as_ptr(),
                    );
                }
            };
            let mut satoData: avifROData = avifROData {
                data: 0 as *const uint8_t,
                size: 0,
            };
            let result___8: avifResult = avifDecoderItemRead(
                sampleTransformItem,
                (*decoder).io,
                &mut satoData,
                0 as size_t,
                0 as size_t,
                (*data).diag,
            ) as avifResult;
            if result___8 as core::ffi::c_uint
                != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
            {
                return result___8;
            }
            let result___9: avifResult = avifParseSampleTransformImageBox(
                satoData.data,
                satoData.size,
                (*data).sampleTransformNumInputImageItems as uint32_t,
                &mut (*(*data).meta).sampleTransformExpression,
                (*data).diag,
            ) as avifResult;
            if result___9 as core::ffi::c_uint
                != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
            {
                return result___9;
            }
            let result___10: avifResult = avifDecoderSampleTransformItemValidateProperties(
                sampleTransformItem,
                (*data).diag,
            ) as avifResult;
            if result___10 as core::ffi::c_uint
                != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
            {
                return result___10;
            }
            let mut pixiProp: *const avifProperty = avifPropertyArrayFind(
                &mut (*sampleTransformItem).properties,
                b"pixi\0" as *const u8 as *const core::ffi::c_char,
            );
            'c_11472: {
                if !pixiProp.is_null() {} else {
                    __assert_fail(
                        b"pixiProp != ((void*)0)\0" as *const u8
                            as *const core::ffi::c_char,
                        b"/home/runner/work/libavif-rs/libavif-rs/upstream/libavif/src/read.c\0"
                            as *const u8 as *const core::ffi::c_char,
                        6584 as core::ffi::c_uint,
                        (::core::mem::transmute::<
                            [u8; 43],
                            [core::ffi::c_char; 43],
                        >(*b"avifResult avifDecoderReset(avifDecoder *)\0"))
                            .as_ptr(),
                    );
                }
            };
            (*(*data).meta).sampleTransformDepth = (*pixiProp)
                .u
                .pixi
                .planeDepths[0 as core::ffi::c_int as usize] as uint32_t;
        }
        let result___11: avifResult = avifDecoderFindMetadata(
            decoder,
            (*data).meta,
            (*decoder).image,
            (*mainItems[AVIF_ITEM_COLOR as core::ffi::c_int as usize]).id,
        ) as avifResult;
        if result___11 as core::ffi::c_uint
            != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
        {
            return result___11;
        }
        (*decoder).imageIndex = -(1 as core::ffi::c_int);
        (*decoder).imageCount = 1 as core::ffi::c_int;
        (*decoder).imageTiming.timescale = 1 as uint64_t;
        (*decoder).imageTiming.pts = 0 as core::ffi::c_int as core::ffi::c_double;
        (*decoder).imageTiming.ptsInTimescales = 0 as uint64_t;
        (*decoder).imageTiming.duration = 1 as core::ffi::c_int as core::ffi::c_double;
        (*decoder).imageTiming.durationInTimescales = 1 as uint64_t;
        (*decoder).timescale = 1 as uint64_t;
        (*decoder).duration = 1 as core::ffi::c_int as core::ffi::c_double;
        (*decoder).durationInTimescales = 1 as uint64_t;
        let mut current_block_335: u64;
        let mut c_2: core::ffi::c_int = AVIF_ITEM_COLOR as core::ffi::c_int;
        while c_2 < AVIF_ITEM_CATEGORY_COUNT as core::ffi::c_int {
            if !(mainItems[c_2 as usize]).is_null() {
                'c_10560: {
                    if c_2 != AVIF_ITEM_SAMPLE_TRANSFORM as core::ffi::c_int {} else {
                        __assert_fail(
                            b"c != AVIF_ITEM_SAMPLE_TRANSFORM\0" as *const u8
                                as *const core::ffi::c_char,
                            b"/home/runner/work/libavif-rs/libavif-rs/upstream/libavif/src/read.c\0"
                                as *const u8 as *const core::ffi::c_char,
                            6607 as core::ffi::c_uint,
                            (::core::mem::transmute::<
                                [u8; 43],
                                [core::ffi::c_char; 43],
                            >(*b"avifResult avifDecoderReset(avifDecoder *)\0"))
                                .as_ptr(),
                        );
                    }
                };
                if avifIsAlpha(c_2 as avifItemCategory) != 0
                    && (*mainItems[c_2 as usize]).width == 0
                    && (*mainItems[c_2 as usize]).height == 0
                {
                    'c_10489: {
                        if (*decoder).strictFlags
                            & AVIF_STRICT_ALPHA_ISPE_REQUIRED as core::ffi::c_int
                                as avifStrictFlags == 0
                        {} else {
                            __assert_fail(
                                b"!(decoder->strictFlags & AVIF_STRICT_ALPHA_ISPE_REQUIRED)\0"
                                    as *const u8 as *const core::ffi::c_char,
                                b"/home/runner/work/libavif-rs/libavif-rs/upstream/libavif/src/read.c\0"
                                    as *const u8 as *const core::ffi::c_char,
                                6611 as core::ffi::c_uint,
                                (::core::mem::transmute::<
                                    [u8; 43],
                                    [core::ffi::c_char; 43],
                                >(*b"avifResult avifDecoderReset(avifDecoder *)\0"))
                                    .as_ptr(),
                            );
                        }
                    };
                    (*mainItems[c_2 as usize]).width = (*mainItems[AVIF_ITEM_COLOR
                        as core::ffi::c_int as usize])
                        .width;
                    (*mainItems[c_2 as usize]).height = (*mainItems[AVIF_ITEM_COLOR
                        as core::ffi::c_int as usize])
                        .height;
                }
                let result___12: avifResult = avifDecoderAdoptGridTileCodecTypeIfNeeded(
                    decoder,
                    mainItems[c_2 as usize],
                    &mut *((*data).tileInfos).as_mut_ptr().offset(c_2 as isize),
                ) as avifResult;
                if result___12 as core::ffi::c_uint
                    != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
                {
                    return result___12;
                }
                if c_2 == AVIF_ITEM_COLOR as core::ffi::c_int {
                    if (*decoder).imageContentToDecode
                        & AVIF_IMAGE_CONTENT_COLOR as core::ffi::c_int
                            as avifImageContentTypeFlags == 0
                    {
                        current_block_335 = 2;
                    } else {
                        current_block_335 = 3;
                    }
                } else if c_2 == AVIF_ITEM_ALPHA as core::ffi::c_int {
                    if (*decoder).imageContentToDecode
                        & AVIF_IMAGE_CONTENT_ALPHA as core::ffi::c_int
                            as avifImageContentTypeFlags == 0
                    {
                        current_block_335 = 2;
                    } else {
                        current_block_335 = 3;
                    }
                } else if c_2
                    == AVIF_ITEM_SAMPLE_TRANSFORM_INPUT_0_COLOR as core::ffi::c_int
                    || c_2
                        == AVIF_ITEM_SAMPLE_TRANSFORM_INPUT_1_COLOR as core::ffi::c_int
                {
                    'c_9765: {
                        if (*decoder).imageContentToDecode
                            & AVIF_IMAGE_CONTENT_COLOR as core::ffi::c_int
                                as avifImageContentTypeFlags != 0
                            && (*decoder).imageContentToDecode
                                & AVIF_IMAGE_CONTENT_SAMPLE_TRANSFORMS as core::ffi::c_int
                                    as avifImageContentTypeFlags != 0
                        {} else {
                            __assert_fail(
                                b"(decoder->imageContentToDecode & AVIF_IMAGE_CONTENT_COLOR) && (decoder->imageContentToDecode & AVIF_IMAGE_CONTENT_SAMPLE_TRANSFORMS)\0"
                                    as *const u8 as *const core::ffi::c_char,
                                b"/home/runner/work/libavif-rs/libavif-rs/upstream/libavif/src/read.c\0"
                                    as *const u8 as *const core::ffi::c_char,
                                6628 as core::ffi::c_uint,
                                (::core::mem::transmute::<
                                    [u8; 43],
                                    [core::ffi::c_char; 43],
                                >(*b"avifResult avifDecoderReset(avifDecoder *)\0"))
                                    .as_ptr(),
                            );
                        }
                    };
                    current_block_335 = 3;
                } else if c_2
                    == AVIF_ITEM_SAMPLE_TRANSFORM_INPUT_0_ALPHA as core::ffi::c_int
                    || c_2
                        == AVIF_ITEM_SAMPLE_TRANSFORM_INPUT_1_ALPHA as core::ffi::c_int
                {
                    'c_9693: {
                        if (*decoder).imageContentToDecode
                            & AVIF_IMAGE_CONTENT_ALPHA as core::ffi::c_int
                                as avifImageContentTypeFlags != 0
                            && (*decoder).imageContentToDecode
                                & AVIF_IMAGE_CONTENT_SAMPLE_TRANSFORMS as core::ffi::c_int
                                    as avifImageContentTypeFlags != 0
                        {} else {
                            __assert_fail(
                                b"(decoder->imageContentToDecode & AVIF_IMAGE_CONTENT_ALPHA) && (decoder->imageContentToDecode & AVIF_IMAGE_CONTENT_SAMPLE_TRANSFORMS)\0"
                                    as *const u8 as *const core::ffi::c_char,
                                b"/home/runner/work/libavif-rs/libavif-rs/upstream/libavif/src/read.c\0"
                                    as *const u8 as *const core::ffi::c_char,
                                6631 as core::ffi::c_uint,
                                (::core::mem::transmute::<
                                    [u8; 43],
                                    [core::ffi::c_char; 43],
                                >(*b"avifResult avifDecoderReset(avifDecoder *)\0"))
                                    .as_ptr(),
                            );
                        }
                    };
                    current_block_335 = 3;
                } else {
                    'c_9656: {
                        if c_2 == AVIF_ITEM_GAIN_MAP as core::ffi::c_int {} else {
                            __assert_fail(
                                b"c == AVIF_ITEM_GAIN_MAP\0" as *const u8
                                    as *const core::ffi::c_char,
                                b"/home/runner/work/libavif-rs/libavif-rs/upstream/libavif/src/read.c\0"
                                    as *const u8 as *const core::ffi::c_char,
                                6633 as core::ffi::c_uint,
                                (::core::mem::transmute::<
                                    [u8; 43],
                                    [core::ffi::c_char; 43],
                                >(*b"avifResult avifDecoderReset(avifDecoder *)\0"))
                                    .as_ptr(),
                            );
                        }
                    };
                    if (*decoder).imageContentToDecode
                        & AVIF_IMAGE_CONTENT_GAIN_MAP as core::ffi::c_int
                            as avifImageContentTypeFlags == 0
                    {
                        current_block_335 = 2;
                    } else {
                        current_block_335 = 3;
                    }
                }
                match current_block_335 {
                    2 => {}
                    _ => {
                        let result___13: avifResult = avifDecoderGenerateImageTiles(
                            decoder,
                            &mut *((*data).tileInfos).as_mut_ptr().offset(c_2 as isize),
                            mainItems[c_2 as usize],
                            c_2 as avifItemCategory,
                        ) as avifResult;
                        if result___13 as core::ffi::c_uint
                            != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
                        {
                            return result___13;
                        }
                        let mut strictFlags: avifStrictFlags = (*decoder).strictFlags;
                        if avifIsAlpha(c_2 as avifItemCategory) != 0
                            && isAlphaItemInInput == 0
                        {
                            strictFlags
                                &= !(AVIF_STRICT_PIXI_REQUIRED as core::ffi::c_int
                                    as avifStrictFlags);
                        }
                        let result___14: avifResult = avifDecoderItemValidateProperties(
                            mainItems[c_2 as usize],
                            avifGetConfigurationPropertyName(codecType[c_2 as usize]),
                            &mut (*decoder).diag,
                            strictFlags,
                        ) as avifResult;
                        if result___14 as core::ffi::c_uint
                            != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
                        {
                            return result___14;
                        }
                    }
                }
            }
            c_2 += 1;
        }
        if (*mainItems[AVIF_ITEM_COLOR as core::ffi::c_int as usize]).progressive != 0 {
            (*decoder).progressiveState = AVIF_PROGRESSIVE_STATE_AVAILABLE;
            let mut colorTile_0: *const avifTile = &mut *((*data).tiles.tile)
                .offset(0 as core::ffi::c_int as isize) as *mut avifTile;
            if (*(*colorTile_0).input).samples.count > 1 as uint32_t {
                (*decoder).progressiveState = AVIF_PROGRESSIVE_STATE_ACTIVE;
                (*decoder).imageCount = (*(*colorTile_0).input).samples.count
                    as core::ffi::c_int;
            }
        }
        (*(*decoder).image).width = (*mainItems[AVIF_ITEM_COLOR as core::ffi::c_int
            as usize])
            .width;
        (*(*decoder).image).height = (*mainItems[AVIF_ITEM_COLOR as core::ffi::c_int
            as usize])
            .height;
        (*decoder).alphaPresent = (mainItems[AVIF_ITEM_ALPHA as core::ffi::c_int
            as usize] != NULL as *mut avifDecoderItem) as core::ffi::c_int as avifBool;
        (*(*decoder).image).alphaPremultiplied = ((*decoder).alphaPresent != 0
            && avifReferences(
                &mut (**mainItems
                    .as_mut_ptr()
                    .offset(AVIF_ITEM_COLOR as core::ffi::c_int as isize))
                    .references,
                b"prem\0" as *const u8 as *const core::ffi::c_char,
                (*mainItems[AVIF_ITEM_ALPHA as core::ffi::c_int as usize]).id,
            ) != 0) as core::ffi::c_int as avifBool;
        if !(mainItems[AVIF_ITEM_ALPHA as core::ffi::c_int as usize]).is_null() {
            alphaProperties = &mut (**mainItems
                .as_mut_ptr()
                .offset(AVIF_ITEM_ALPHA as core::ffi::c_int as isize))
                .properties;
        }
        if !(mainItems[AVIF_ITEM_GAIN_MAP as core::ffi::c_int as usize]).is_null() {
            'c_6973: {
                if !((*(*decoder).image).gainMap).is_null()
                    && !((*(*(*decoder).image).gainMap).image).is_null()
                {} else {
                    __assert_fail(
                        b"decoder->image->gainMap && decoder->image->gainMap->image\0"
                            as *const u8 as *const core::ffi::c_char,
                        b"/home/runner/work/libavif-rs/libavif-rs/upstream/libavif/src/read.c\0"
                            as *const u8 as *const core::ffi::c_char,
                        6671 as core::ffi::c_uint,
                        (::core::mem::transmute::<
                            [u8; 43],
                            [core::ffi::c_char; 43],
                        >(*b"avifResult avifDecoderReset(avifDecoder *)\0"))
                            .as_ptr(),
                    );
                }
            };
            (*(*(*(*decoder).image).gainMap).image).width = (*mainItems[AVIF_ITEM_GAIN_MAP
                as core::ffi::c_int as usize])
                .width;
            (*(*(*(*decoder).image).gainMap).image).height = (*mainItems[AVIF_ITEM_GAIN_MAP
                as core::ffi::c_int as usize])
                .height;
            let result___15: avifResult = avifReadCodecConfigProperty(
                (*(*(*decoder).image).gainMap).image as *mut avifImage,
                &mut (**mainItems
                    .as_mut_ptr()
                    .offset(AVIF_ITEM_GAIN_MAP as core::ffi::c_int as isize))
                    .properties,
                codecType[AVIF_ITEM_GAIN_MAP as core::ffi::c_int as usize],
            ) as avifResult;
            if result___15 as core::ffi::c_uint
                != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
            {
                return result___15;
            }
            gainMapProperties = &mut (**mainItems
                .as_mut_ptr()
                .offset(AVIF_ITEM_GAIN_MAP as core::ffi::c_int as isize))
                .properties;
        }
    }
    let mut firstTileIndex: uint32_t = 0 as uint32_t;
    let mut c_3: core::ffi::c_int = 0 as core::ffi::c_int;
    while c_3 < AVIF_ITEM_CATEGORY_COUNT as core::ffi::c_int {
        (*data).tileInfos[c_3 as usize].firstTileIndex = firstTileIndex
            as core::ffi::c_uint;
        firstTileIndex = (firstTileIndex as core::ffi::c_uint)
            .wrapping_add((*data).tileInfos[c_3 as usize].tileCount) as uint32_t
            as uint32_t;
        c_3 += 1;
    }
    let mut tileIndex: uint32_t = 0 as uint32_t;
    while tileIndex < (*data).tiles.count {
        let mut tile: *mut avifTile = &mut *((*data).tiles.tile)
            .offset(tileIndex as isize) as *mut avifTile;
        let mut sampleIndex: uint32_t = 0 as uint32_t;
        while sampleIndex < (*(*tile).input).samples.count {
            let mut sample: *mut avifDecodeSample = &mut *((*(*tile).input)
                .samples
                .sample)
                .offset(sampleIndex as isize) as *mut avifDecodeSample;
            if (*sample).size == 0 {
                return AVIF_RESULT_BMFF_PARSE_FAILED;
            }
            if (*(*tile).input).itemCategory as core::ffi::c_uint
                == AVIF_ITEM_COLOR as core::ffi::c_int as core::ffi::c_uint
            {
                (*decoder).ioStats.colorOBUSize = ((*decoder).ioStats.colorOBUSize)
                    .wrapping_add((*sample).size);
            } else if (*(*tile).input).itemCategory as core::ffi::c_uint
                == AVIF_ITEM_ALPHA as core::ffi::c_int as core::ffi::c_uint
            {
                (*decoder).ioStats.alphaOBUSize = ((*decoder).ioStats.alphaOBUSize)
                    .wrapping_add((*sample).size);
            }
            sampleIndex = sampleIndex.wrapping_add(1);
        }
        tileIndex = tileIndex.wrapping_add(1);
    }
    let result___16: avifResult = avifReadColorProperties(
        (*decoder).io,
        colorProperties,
        if (*decoder).ignoreICC != 0 {
            0 as *mut avifRWData
        } else {
            &mut (*(*decoder).image).icc
        },
        &mut (*(*decoder).image).colorPrimaries,
        &mut (*(*decoder).image).transferCharacteristics,
        &mut (*(*decoder).image).matrixCoefficients,
        &mut (*(*decoder).image).yuvRange,
        &mut (*data).cicpSet,
    ) as avifResult;
    if result___16 as core::ffi::c_uint
        != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
    {
        return result___16;
    }
    let mut clliProp: *const avifProperty = avifPropertyArrayFind(
        colorProperties,
        b"clli\0" as *const u8 as *const core::ffi::c_char,
    );
    if !clliProp.is_null() {
        (*(*decoder).image).clli = (*clliProp).u.clli;
    }
    let mut paspProp: *const avifProperty = avifPropertyArrayFind(
        colorProperties,
        b"pasp\0" as *const u8 as *const core::ffi::c_char,
    );
    if !paspProp.is_null() {
        (*(*decoder).image).transformFlags
            |= AVIF_TRANSFORM_PASP as core::ffi::c_int as avifTransformFlags;
        (*(*decoder).image).pasp = (*paspProp).u.pasp;
    }
    let mut clapProp: *const avifProperty = avifPropertyArrayFind(
        colorProperties,
        b"clap\0" as *const u8 as *const core::ffi::c_char,
    );
    if !clapProp.is_null() {
        (*(*decoder).image).transformFlags
            |= AVIF_TRANSFORM_CLAP as core::ffi::c_int as avifTransformFlags;
        (*(*decoder).image).clap = (*clapProp).u.clap;
    }
    let mut irotProp: *const avifProperty = avifPropertyArrayFind(
        colorProperties,
        b"irot\0" as *const u8 as *const core::ffi::c_char,
    );
    if !irotProp.is_null() {
        (*(*decoder).image).transformFlags
            |= AVIF_TRANSFORM_IROT as core::ffi::c_int as avifTransformFlags;
        (*(*decoder).image).irot = (*irotProp).u.irot;
    }
    let mut imirProp: *const avifProperty = avifPropertyArrayFind(
        colorProperties,
        b"imir\0" as *const u8 as *const core::ffi::c_char,
    );
    if !imirProp.is_null() {
        (*(*decoder).image).transformFlags
            |= AVIF_TRANSFORM_IMIR as core::ffi::c_int as avifTransformFlags;
        (*(*decoder).image).imir = (*imirProp).u.imir;
    }
    if !alphaProperties.is_null() {
        let result___17: avifResult = avifDecoderCheckAlphaProperties(
            decoder,
            alphaProperties,
        ) as avifResult;
        if result___17 as core::ffi::c_uint
            != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
        {
            return result___17;
        }
    }
    if !gainMapProperties.is_null() {
        let result___18: avifResult = avifDecoderCheckGainMapProperties(
            decoder,
            gainMapProperties,
        ) as avifResult;
        if result___18 as core::ffi::c_uint
            != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
        {
            return result___18;
        }
    }
    if (*data).cicpSet == 0 && (*data).tiles.count > 0 as uint32_t {
        let mut firstTile: *mut avifTile = &mut *((*data).tiles.tile)
            .offset(0 as core::ffi::c_int as isize) as *mut avifTile;
        if (*(*firstTile).input).samples.count > 0 as uint32_t {
            let mut sample_0: *mut avifDecodeSample = &mut *((*(*firstTile).input)
                .samples
                .sample)
                .offset(0 as core::ffi::c_int as isize) as *mut avifDecodeSample;
            static mut searchSampleChunkIncrement: size_t = 64 as size_t;
            static mut searchSampleSizeMax: size_t = 4096 as size_t;
            let mut searchSampleSize: size_t = 0 as size_t;
            loop {
                searchSampleSize = searchSampleSize
                    .wrapping_add(searchSampleChunkIncrement);
                if searchSampleSize > (*sample_0).size {
                    searchSampleSize = (*sample_0).size;
                }
                let mut prepareResult: avifResult = avifDecoderPrepareSample(
                    decoder,
                    sample_0,
                    searchSampleSize,
                );
                if prepareResult as core::ffi::c_uint
                    != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
                {
                    return prepareResult;
                }
                let mut sequenceHeader: avifSequenceHeader = avifSequenceHeader {
                    reduced_still_picture_header: 0,
                    maxWidth: 0,
                    maxHeight: 0,
                    bitDepth: 0,
                    yuvFormat: AVIF_PIXEL_FORMAT_NONE,
                    chromaSamplePosition: AVIF_CHROMA_SAMPLE_POSITION_UNKNOWN,
                    colorPrimaries: 0,
                    transferCharacteristics: 0,
                    matrixCoefficients: 0,
                    range: AVIF_RANGE_LIMITED,
                    av1C: avifCodecConfigurationBox {
                        seqProfile: 0,
                        seqLevelIdx0: 0,
                        seqTier0: 0,
                        highBitdepth: 0,
                        twelveBit: 0,
                        monochrome: 0,
                        chromaSubsamplingX: 0,
                        chromaSubsamplingY: 0,
                        chromaSamplePosition: 0,
                    },
                };
                if avifSequenceHeaderParse(
                    &mut sequenceHeader,
                    &mut (*sample_0).data,
                    (*firstTile).codecType,
                ) != 0
                {
                    (*data).cicpSet = AVIF_TRUE as avifBool;
                    (*(*decoder).image).colorPrimaries = sequenceHeader.colorPrimaries;
                    (*(*decoder).image).transferCharacteristics = sequenceHeader
                        .transferCharacteristics;
                    (*(*decoder).image).matrixCoefficients = sequenceHeader
                        .matrixCoefficients;
                    (*(*decoder).image).yuvRange = sequenceHeader.range;
                    break;
                } else if !(searchSampleSize != (*sample_0).size
                    && searchSampleSize < searchSampleSizeMax)
                {
                    break;
                }
            }
        }
    }
    let result___19: avifResult = avifReadCodecConfigProperty(
        (*decoder).image,
        colorProperties,
        colorCodecType,
    ) as avifResult;
    if result___19 as core::ffi::c_uint
        != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
    {
        return result___19;
    }
    if (*(*(*decoder).data).meta).sampleTransformExpression.count > 0 as uint32_t {
        'c_3142: {
            if (*(*(*decoder).data).meta).sampleTransformDepth != 0 as uint32_t {} else {
                __assert_fail(
                    b"decoder->data->meta->sampleTransformDepth != 0\0" as *const u8
                        as *const core::ffi::c_char,
                    b"/home/runner/work/libavif-rs/libavif-rs/upstream/libavif/src/read.c\0"
                        as *const u8 as *const core::ffi::c_char,
                    6785 as core::ffi::c_uint,
                    (::core::mem::transmute::<
                        [u8; 43],
                        [core::ffi::c_char; 43],
                    >(*b"avifResult avifDecoderReset(avifDecoder *)\0"))
                        .as_ptr(),
                );
            }
        };
        (*(*decoder).image).depth = (*(*(*decoder).data).meta).sampleTransformDepth;
    }
    let mut i_1: size_t = 0 as size_t;
    while i_1 < (*colorProperties).count as size_t {
        let mut property: *const avifProperty = &mut *((*colorProperties).prop)
            .offset(i_1 as isize) as *mut avifProperty;
        if (*property).isOpaque != 0 {
            let result___20: avifResult = avifImagePushProperty(
                (*decoder).image,
                ((*property).type_0).as_ptr(),
                ((*property).u.opaque.usertype).as_ptr(),
                (*property).u.opaque.boxPayload.data,
                (*property).u.opaque.boxPayload.size,
            ) as avifResult;
            if result___20 as core::ffi::c_uint
                != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
            {
                return result___20;
            }
        }
        i_1 = i_1.wrapping_add(1);
    }
    if !gainMapProperties.is_null() {
        let mut i_2: size_t = 0 as size_t;
        while i_2 < (*gainMapProperties).count as size_t {
            let mut property_0: *const avifProperty = &mut *((*gainMapProperties).prop)
                .offset(i_2 as isize) as *mut avifProperty;
            if (*property_0).isOpaque != 0 {
                let result___21: avifResult = avifImagePushProperty(
                    (*(*(*decoder).image).gainMap).image as *mut avifImage,
                    ((*property_0).type_0).as_ptr(),
                    ((*property_0).u.opaque.usertype).as_ptr(),
                    (*property_0).u.opaque.boxPayload.data,
                    (*property_0).u.opaque.boxPayload.size,
                ) as avifResult;
                if result___21 as core::ffi::c_uint
                    != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
                {
                    return result___21;
                }
            }
            i_2 = i_2.wrapping_add(1);
        }
    }
    return AVIF_RESULT_OK;
}
unsafe extern "C" fn avifDecoderPrepareTiles(
    mut decoder: *mut avifDecoder,
    mut nextImageIndex: uint32_t,
    mut info: *const avifTileInfo,
) -> avifResult {
    let mut tileIndex: core::ffi::c_uint = (*info).decodedTileCount;
    while tileIndex < (*info).tileCount {
        let mut tile: *mut avifTile = &mut *((*(*decoder).data).tiles.tile)
            .offset(((*info).firstTileIndex).wrapping_add(tileIndex) as isize)
            as *mut avifTile;
        if nextImageIndex >= (*(*tile).input).samples.count {
            return AVIF_RESULT_NO_IMAGES_REMAINING;
        }
        let mut sample: *mut avifDecodeSample = &mut *((*(*tile).input).samples.sample)
            .offset(nextImageIndex as isize) as *mut avifDecodeSample;
        let mut prepareResult: avifResult = avifDecoderPrepareSample(
            decoder,
            sample,
            0 as size_t,
        );
        if prepareResult as core::ffi::c_uint
            != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
        {
            return prepareResult;
        }
        tileIndex = tileIndex.wrapping_add(1);
    }
    return AVIF_RESULT_OK;
}
unsafe extern "C" fn avifImageLimitedToFullAlpha(
    mut image: *mut avifImage,
) -> avifResult {
    if (*image).imageOwnsAlphaPlane != 0 {
        return AVIF_RESULT_NOT_IMPLEMENTED;
    }
    let mut alphaPlane: *const uint8_t = (*image).alphaPlane;
    let alphaRowBytes: uint32_t = (*image).alphaRowBytes;
    (*image).alphaPlane = 0 as *mut uint8_t;
    (*image).alphaRowBytes = 0 as uint32_t;
    let allocationResult: avifResult = avifImageAllocatePlanes(
        image,
        AVIF_PLANES_A as core::ffi::c_int as avifPlanesFlags,
    ) as avifResult;
    if allocationResult as core::ffi::c_uint
        != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
    {
        return allocationResult;
    }
    if (*image).depth > 8 as uint32_t {
        let mut j: uint32_t = 0 as uint32_t;
        while j < (*image).height {
            let mut srcRow: *const uint8_t = &*alphaPlane
                .offset((j as size_t).wrapping_mul(alphaRowBytes as size_t) as isize)
                as *const uint8_t;
            let mut dstRow: *mut uint8_t = &mut *((*image).alphaPlane)
                .offset(
                    (j as size_t).wrapping_mul((*image).alphaRowBytes as size_t) as isize,
                ) as *mut uint8_t;
            let mut i: uint32_t = 0 as uint32_t;
            while i < (*image).width {
                let mut srcAlpha: core::ffi::c_int = *(&*srcRow
                    .offset(i.wrapping_mul(2 as uint32_t) as isize) as *const uint8_t
                    as *const uint16_t) as core::ffi::c_int;
                let mut dstAlpha: core::ffi::c_int = avifLimitedToFullY(
                    (*image).depth,
                    srcAlpha,
                );
                *(&mut *dstRow.offset(i.wrapping_mul(2 as uint32_t) as isize)
                    as *mut uint8_t as *mut uint16_t) = dstAlpha as uint16_t;
                i = i.wrapping_add(1);
            }
            j = j.wrapping_add(1);
        }
    } else {
        let mut j_0: uint32_t = 0 as uint32_t;
        while j_0 < (*image).height {
            let mut srcRow_0: *const uint8_t = &*alphaPlane
                .offset((j_0 as size_t).wrapping_mul(alphaRowBytes as size_t) as isize)
                as *const uint8_t;
            let mut dstRow_0: *mut uint8_t = &mut *((*image).alphaPlane)
                .offset(
                    (j_0 as size_t).wrapping_mul((*image).alphaRowBytes as size_t)
                        as isize,
                ) as *mut uint8_t;
            let mut i_0: uint32_t = 0 as uint32_t;
            while i_0 < (*image).width {
                let mut srcAlpha_0: core::ffi::c_int = *srcRow_0.offset(i_0 as isize)
                    as core::ffi::c_int;
                let mut dstAlpha_0: core::ffi::c_int = avifLimitedToFullY(
                    (*image).depth,
                    srcAlpha_0,
                );
                *dstRow_0.offset(i_0 as isize) = dstAlpha_0 as uint8_t;
                i_0 = i_0.wrapping_add(1);
            }
            j_0 = j_0.wrapping_add(1);
        }
    }
    return AVIF_RESULT_OK;
}
unsafe extern "C" fn avifGetErrorForItemCategory(
    mut itemCategory: avifItemCategory,
) -> avifResult {
    if itemCategory as core::ffi::c_uint
        == AVIF_ITEM_GAIN_MAP as core::ffi::c_int as core::ffi::c_uint
    {
        return AVIF_RESULT_DECODE_GAIN_MAP_FAILED;
    }
    if itemCategory as core::ffi::c_uint
        >= AVIF_ITEM_SAMPLE_TRANSFORM_INPUT_0_COLOR as core::ffi::c_int
            as core::ffi::c_uint
        && itemCategory as core::ffi::c_uint
            <= (AVIF_ITEM_SAMPLE_TRANSFORM_INPUT_0_ALPHA as core::ffi::c_int
                + (AVIF_ITEM_SAMPLE_TRANSFORM_INPUT_0_ALPHA as core::ffi::c_int
                    - AVIF_ITEM_SAMPLE_TRANSFORM_INPUT_0_COLOR as core::ffi::c_int)
                - 1 as core::ffi::c_int) as core::ffi::c_uint
    {
        return AVIF_RESULT_DECODE_SAMPLE_TRANSFORM_FAILED;
    }
    return (if avifIsAlpha(itemCategory) != 0 {
        AVIF_RESULT_DECODE_ALPHA_FAILED as core::ffi::c_int
    } else {
        AVIF_RESULT_DECODE_COLOR_FAILED as core::ffi::c_int
    }) as avifResult;
}
unsafe extern "C" fn avifDecoderDecodeTiles(
    mut decoder: *mut avifDecoder,
    mut nextImageIndex: uint32_t,
    mut info: *mut avifTileInfo,
) -> avifResult {
    let oldDecodedTileCount: core::ffi::c_uint = (*info).decodedTileCount;
    let mut tileIndex: core::ffi::c_uint = oldDecodedTileCount;
    while tileIndex < (*info).tileCount {
        let mut tile: *mut avifTile = &mut *((*(*decoder).data).tiles.tile)
            .offset(((*info).firstTileIndex).wrapping_add(tileIndex) as isize)
            as *mut avifTile;
        let mut sample: *const avifDecodeSample = &mut *((*(*tile).input).samples.sample)
            .offset(nextImageIndex as isize) as *mut avifDecodeSample;
        if (*sample).data.size < (*sample).size {
            'c_37878: {
                if (*decoder).allowIncremental != 0 {} else {
                    __assert_fail(
                        b"decoder->allowIncremental\0" as *const u8
                            as *const core::ffi::c_char,
                        b"/home/runner/work/libavif-rs/libavif-rs/upstream/libavif/src/read.c\0"
                            as *const u8 as *const core::ffi::c_char,
                        6895 as core::ffi::c_uint,
                        (::core::mem::transmute::<
                            [u8; 75],
                            [core::ffi::c_char; 75],
                        >(
                            *b"avifResult avifDecoderDecodeTiles(avifDecoder *, uint32_t, avifTileInfo *)\0",
                        ))
                            .as_ptr(),
                    );
                }
            };
            return AVIF_RESULT_OK;
        }
        let mut isLimitedRangeAlpha: avifBool = AVIF_FALSE;
        (*(*tile).codec).maxThreads = (*decoder).maxThreads;
        (*(*tile).codec).imageSizeLimit = (*decoder).imageSizeLimit;
        (*(*tile).codec).imageDimensionLimit = (*decoder).imageDimensionLimit;
        if ((*(*tile).codec).getNextImage)
            .expect(
                "non-null function pointer",
            )(
            (*tile).codec,
            sample,
            avifIsAlpha((*(*tile).input).itemCategory),
            &mut isLimitedRangeAlpha,
            (*tile).image,
        ) == 0
        {
            avifDiagnosticsPrintf(
                &mut (*decoder).diag as *mut avifDiagnostics,
                b"tile->codec->getNextImage() failed\0" as *const u8
                    as *const core::ffi::c_char,
            );
            return avifGetErrorForItemCategory((*(*tile).input).itemCategory);
        }
        if avifIsAlpha((*(*tile).input).itemCategory) != 0 && isLimitedRangeAlpha != 0 {
            let mut result: avifResult = avifImageLimitedToFullAlpha((*tile).image);
            if result as core::ffi::c_uint
                != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
            {
                avifDiagnosticsPrintf(
                    &mut (*decoder).diag as *mut avifDiagnostics,
                    b"avifImageLimitedToFullAlpha failed\0" as *const u8
                        as *const core::ffi::c_char,
                );
                return result;
            }
        }
        if (*tile).width != (*(*tile).image).width
            || (*tile).height != (*(*tile).image).height
        {
            if avifImageScaleWithLimit(
                (*tile).image,
                (*tile).width,
                (*tile).height,
                (*decoder).imageSizeLimit,
                (*decoder).imageDimensionLimit,
                &mut (*decoder).diag,
            ) as core::ffi::c_uint
                != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
            {
                return avifGetErrorForItemCategory((*(*tile).input).itemCategory);
            }
        }
        (*info).decodedTileCount = ((*info).decodedTileCount).wrapping_add(1);
        let isGrid: avifBool = ((*info).grid.rows > 0 as uint32_t
            && (*info).grid.columns > 0 as uint32_t) as core::ffi::c_int;
        let mut stealPlanes: avifBool = (isGrid == 0) as core::ffi::c_int;
        if (*(*(*decoder).data).meta).sampleTransformExpression.count > 0 as uint32_t {
            stealPlanes = AVIF_FALSE as avifBool;
        }
        if !((*(*tile).input).itemCategory as core::ffi::c_uint
            >= AVIF_ITEM_SAMPLE_TRANSFORM_INPUT_0_COLOR as core::ffi::c_int
                as core::ffi::c_uint
            && (*(*tile).input).itemCategory as core::ffi::c_uint
                <= (AVIF_ITEM_SAMPLE_TRANSFORM_INPUT_0_ALPHA as core::ffi::c_int
                    + (AVIF_ITEM_SAMPLE_TRANSFORM_INPUT_0_ALPHA as core::ffi::c_int
                        - AVIF_ITEM_SAMPLE_TRANSFORM_INPUT_0_COLOR as core::ffi::c_int)
                    - 1 as core::ffi::c_int) as core::ffi::c_uint)
        {
            if stealPlanes == 0 {
                let mut dstImage: *mut avifImage = (*decoder).image;
                if (*(*tile).input).itemCategory as core::ffi::c_uint
                    == AVIF_ITEM_GAIN_MAP as core::ffi::c_int as core::ffi::c_uint
                {
                    'c_37190: {
                        if !((*dstImage).gainMap).is_null()
                            && !((*(*dstImage).gainMap).image).is_null()
                        {} else {
                            __assert_fail(
                                b"dstImage->gainMap && dstImage->gainMap->image\0"
                                    as *const u8 as *const core::ffi::c_char,
                                b"/home/runner/work/libavif-rs/libavif-rs/upstream/libavif/src/read.c\0"
                                    as *const u8 as *const core::ffi::c_char,
                                6970 as core::ffi::c_uint,
                                (::core::mem::transmute::<
                                    [u8; 75],
                                    [core::ffi::c_char; 75],
                                >(
                                    *b"avifResult avifDecoderDecodeTiles(avifDecoder *, uint32_t, avifTileInfo *)\0",
                                ))
                                    .as_ptr(),
                            );
                        }
                    };
                    dstImage = (*(*dstImage).gainMap).image as *mut avifImage;
                }
                if tileIndex == 0 as core::ffi::c_uint {
                    let result__: avifResult = avifDecoderDataAllocateImagePlanes(
                        (*decoder).data,
                        info,
                        dstImage,
                        &mut (*(*decoder).data).cicpSet,
                    ) as avifResult;
                    if result__ as core::ffi::c_uint
                        != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
                    {
                        return result__;
                    }
                }
                let result___0: avifResult = avifDecoderDataCopyTileToImage(
                    (*decoder).data as *mut avifDecoderData,
                    info,
                    dstImage,
                    tile,
                    tileIndex,
                ) as avifResult;
                if result___0 as core::ffi::c_uint
                    != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
                {
                    return result___0;
                }
            } else {
                'c_37049: {
                    if (*info).tileCount == 1 as core::ffi::c_uint {} else {
                        __assert_fail(
                            b"info->tileCount == 1\0" as *const u8
                                as *const core::ffi::c_char,
                            b"/home/runner/work/libavif-rs/libavif-rs/upstream/libavif/src/read.c\0"
                                as *const u8 as *const core::ffi::c_char,
                            6978 as core::ffi::c_uint,
                            (::core::mem::transmute::<
                                [u8; 75],
                                [core::ffi::c_char; 75],
                            >(
                                *b"avifResult avifDecoderDecodeTiles(avifDecoder *, uint32_t, avifTileInfo *)\0",
                            ))
                                .as_ptr(),
                        );
                    }
                };
                'c_37010: {
                    if tileIndex == 0 as core::ffi::c_uint {} else {
                        __assert_fail(
                            b"tileIndex == 0\0" as *const u8 as *const core::ffi::c_char,
                            b"/home/runner/work/libavif-rs/libavif-rs/upstream/libavif/src/read.c\0"
                                as *const u8 as *const core::ffi::c_char,
                            6979 as core::ffi::c_uint,
                            (::core::mem::transmute::<
                                [u8; 75],
                                [core::ffi::c_char; 75],
                            >(
                                *b"avifResult avifDecoderDecodeTiles(avifDecoder *, uint32_t, avifTileInfo *)\0",
                            ))
                                .as_ptr(),
                        );
                    }
                };
                let mut src: *mut avifImage = (*tile).image;
                if (*(*tile).input).itemCategory as core::ffi::c_uint
                    == AVIF_ITEM_GAIN_MAP as core::ffi::c_int as core::ffi::c_uint
                {
                    'c_36943: {
                        if !((*(*decoder).image).gainMap).is_null()
                            && !((*(*(*decoder).image).gainMap).image).is_null()
                        {} else {
                            __assert_fail(
                                b"decoder->image->gainMap && decoder->image->gainMap->image\0"
                                    as *const u8 as *const core::ffi::c_char,
                                b"/home/runner/work/libavif-rs/libavif-rs/upstream/libavif/src/read.c\0"
                                    as *const u8 as *const core::ffi::c_char,
                                6983 as core::ffi::c_uint,
                                (::core::mem::transmute::<
                                    [u8; 75],
                                    [core::ffi::c_char; 75],
                                >(
                                    *b"avifResult avifDecoderDecodeTiles(avifDecoder *, uint32_t, avifTileInfo *)\0",
                                ))
                                    .as_ptr(),
                            );
                        }
                    };
                    (*(*(*(*decoder).image).gainMap).image).width = (*src).width;
                    (*(*(*(*decoder).image).gainMap).image).height = (*src).height;
                    (*(*(*(*decoder).image).gainMap).image).depth = (*src).depth;
                } else if (*(*decoder).image).width != (*src).width
                    || (*(*decoder).image).height != (*src).height
                    || (*(*decoder).image).depth != (*src).depth
                {
                    if avifIsAlpha((*(*tile).input).itemCategory) != 0 {
                        avifDiagnosticsPrintf(
                            &mut (*decoder).diag as *mut avifDiagnostics,
                            b"The color image item does not match the alpha image item in width, height, or bit depth\0"
                                as *const u8 as *const core::ffi::c_char,
                        );
                        return AVIF_RESULT_DECODE_ALPHA_FAILED;
                    }
                    avifImageFreePlanes(
                        (*decoder).image,
                        AVIF_PLANES_ALL as core::ffi::c_int as avifPlanesFlags,
                    );
                    (*(*decoder).image).width = (*src).width;
                    (*(*decoder).image).height = (*src).height;
                    (*(*decoder).image).depth = (*src).depth;
                }
                if avifIsAlpha((*(*tile).input).itemCategory) != 0 {
                    avifImageStealPlanes(
                        (*decoder).image,
                        src,
                        AVIF_PLANES_A as core::ffi::c_int as avifPlanesFlags,
                    );
                } else if (*(*tile).input).itemCategory as core::ffi::c_uint
                    == AVIF_ITEM_GAIN_MAP as core::ffi::c_int as core::ffi::c_uint
                {
                    'c_36700: {
                        if !((*(*decoder).image).gainMap).is_null()
                            && !((*(*(*decoder).image).gainMap).image).is_null()
                        {} else {
                            __assert_fail(
                                b"decoder->image->gainMap && decoder->image->gainMap->image\0"
                                    as *const u8 as *const core::ffi::c_char,
                                b"/home/runner/work/libavif-rs/libavif-rs/upstream/libavif/src/read.c\0"
                                    as *const u8 as *const core::ffi::c_char,
                                7006 as core::ffi::c_uint,
                                (::core::mem::transmute::<
                                    [u8; 75],
                                    [core::ffi::c_char; 75],
                                >(
                                    *b"avifResult avifDecoderDecodeTiles(avifDecoder *, uint32_t, avifTileInfo *)\0",
                                ))
                                    .as_ptr(),
                            );
                        }
                    };
                    avifImageStealPlanes(
                        (*(*(*decoder).image).gainMap).image as *mut avifImage,
                        src,
                        AVIF_PLANES_YUV as core::ffi::c_int as avifPlanesFlags,
                    );
                } else {
                    avifImageStealPlanes(
                        (*decoder).image,
                        src,
                        AVIF_PLANES_YUV as core::ffi::c_int as avifPlanesFlags,
                    );
                }
            }
        }
        tileIndex = tileIndex.wrapping_add(1);
    }
    return AVIF_RESULT_OK;
}
unsafe extern "C" fn avifDecoderDataFrameFullyDecoded(
    mut data: *const avifDecoderData,
) -> avifBool {
    let mut c: core::ffi::c_int = 0 as core::ffi::c_int;
    while c < AVIF_ITEM_CATEGORY_COUNT as core::ffi::c_int {
        if (*data).tileInfos[c as usize].decodedTileCount
            != (*data).tileInfos[c as usize].tileCount
        {
            return AVIF_FALSE;
        }
        c += 1;
    }
    return AVIF_TRUE;
}
unsafe extern "C" fn avifDecoderApplySampleTransformForPlanesImpl(
    mut decoder: *const avifDecoder,
    mut planes: avifPlanesFlag,
    mut dstImage: *mut avifImage,
    mut reconstructedInputImages: *mut *mut avifImage,
) -> avifResult {
    'c_35858: {
        if (*(*decoder).data).sampleTransformNumInputImageItems as core::ffi::c_int
            != 0 as core::ffi::c_int
        {} else {
            __assert_fail(
                b"decoder->data->sampleTransformNumInputImageItems != 0\0" as *const u8
                    as *const core::ffi::c_char,
                b"/home/runner/work/libavif-rs/libavif-rs/upstream/libavif/src/read.c\0"
                    as *const u8 as *const core::ffi::c_char,
                7036 as core::ffi::c_uint,
                (::core::mem::transmute::<
                    [u8; 120],
                    [core::ffi::c_char; 120],
                >(
                    *b"avifResult avifDecoderApplySampleTransformForPlanesImpl(const avifDecoder *, avifPlanesFlag, avifImage *, avifImage **)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    'c_35799: {
        if (*(*decoder).data).sampleTransformNumInputImageItems as core::ffi::c_int
            <= 1 as core::ffi::c_int
                + (AVIF_ITEM_SAMPLE_TRANSFORM_INPUT_0_ALPHA as core::ffi::c_int
                    - AVIF_ITEM_SAMPLE_TRANSFORM_INPUT_0_COLOR as core::ffi::c_int)
        {} else {
            __assert_fail(
                b"decoder->data->sampleTransformNumInputImageItems <= (1 + (AVIF_ITEM_SAMPLE_TRANSFORM_INPUT_0_ALPHA - AVIF_ITEM_SAMPLE_TRANSFORM_INPUT_0_COLOR))\0"
                    as *const u8 as *const core::ffi::c_char,
                b"/home/runner/work/libavif-rs/libavif-rs/upstream/libavif/src/read.c\0"
                    as *const u8 as *const core::ffi::c_char,
                7037 as core::ffi::c_uint,
                (::core::mem::transmute::<
                    [u8; 120],
                    [core::ffi::c_char; 120],
                >(
                    *b"avifResult avifDecoderApplySampleTransformForPlanesImpl(const avifDecoder *, avifPlanesFlag, avifImage *, avifImage **)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    let mut inputImages: [*const avifImage; 3] = [0 as *const avifImage; 3];
    let mut i: uint32_t = 0 as uint32_t;
    while i < (*(*decoder).data).sampleTransformNumInputImageItems as uint32_t {
        let mut category: avifItemCategory = (*(*decoder).data)
            .sampleTransformInputImageItems[i as usize];
        if category as core::ffi::c_uint
            == AVIF_ITEM_COLOR as core::ffi::c_int as core::ffi::c_uint
        {
            inputImages[i as usize] = (*decoder).image;
        } else {
            'c_35707: {
                if category as core::ffi::c_uint
                    >= AVIF_ITEM_SAMPLE_TRANSFORM_INPUT_0_COLOR as core::ffi::c_int
                        as core::ffi::c_uint
                    && (category as core::ffi::c_uint)
                        < (AVIF_ITEM_SAMPLE_TRANSFORM_INPUT_0_COLOR as core::ffi::c_int
                            + (AVIF_ITEM_SAMPLE_TRANSFORM_INPUT_0_ALPHA
                                as core::ffi::c_int
                                - AVIF_ITEM_SAMPLE_TRANSFORM_INPUT_0_COLOR
                                    as core::ffi::c_int)) as core::ffi::c_uint
                {} else {
                    __assert_fail(
                        b"category >= AVIF_ITEM_SAMPLE_TRANSFORM_INPUT_0_COLOR && category < AVIF_ITEM_SAMPLE_TRANSFORM_INPUT_0_COLOR + (AVIF_ITEM_SAMPLE_TRANSFORM_INPUT_0_ALPHA - AVIF_ITEM_SAMPLE_TRANSFORM_INPUT_0_COLOR)\0"
                            as *const u8 as *const core::ffi::c_char,
                        b"/home/runner/work/libavif-rs/libavif-rs/upstream/libavif/src/read.c\0"
                            as *const u8 as *const core::ffi::c_char,
                        7048 as core::ffi::c_uint,
                        (::core::mem::transmute::<
                            [u8; 120],
                            [core::ffi::c_char; 120],
                        >(
                            *b"avifResult avifDecoderApplySampleTransformForPlanesImpl(const avifDecoder *, avifPlanesFlag, avifImage *, avifImage **)\0",
                        ))
                            .as_ptr(),
                    );
                }
            };
            if planes as core::ffi::c_uint
                == AVIF_PLANES_A as core::ffi::c_int as core::ffi::c_uint
            {
                category = ::core::mem::transmute::<
                    core::ffi::c_uint,
                    avifItemCategory,
                >(
                    (category as core::ffi::c_uint)
                        .wrapping_add(
                            (AVIF_ITEM_SAMPLE_TRANSFORM_INPUT_0_ALPHA as core::ffi::c_int
                                - AVIF_ITEM_SAMPLE_TRANSFORM_INPUT_0_COLOR
                                    as core::ffi::c_int) as core::ffi::c_uint,
                        ),
                ) as avifItemCategory;
            }
            let mut info: *const avifTileInfo = &mut *((*(*decoder).data).tileInfos)
                .as_mut_ptr()
                .offset(category as isize) as *mut avifTileInfo;
            'c_35648: {
                if !info.is_null() {} else {
                    __assert_fail(
                        b"info != ((void*)0)\0" as *const u8 as *const core::ffi::c_char,
                        b"/home/runner/work/libavif-rs/libavif-rs/upstream/libavif/src/read.c\0"
                            as *const u8 as *const core::ffi::c_char,
                        7053 as core::ffi::c_uint,
                        (::core::mem::transmute::<
                            [u8; 120],
                            [core::ffi::c_char; 120],
                        >(
                            *b"avifResult avifDecoderApplySampleTransformForPlanesImpl(const avifDecoder *, avifPlanesFlag, avifImage *, avifImage **)\0",
                        ))
                            .as_ptr(),
                    );
                }
            };
            let mut firstTile: *const avifTile = &mut *((*(*decoder).data).tiles.tile)
                .offset((*info).firstTileIndex as isize) as *mut avifTile;
            'c_35586: {
                if !firstTile.is_null() && !((*firstTile).image).is_null() {} else {
                    __assert_fail(
                        b"firstTile != ((void*)0) && firstTile->image != ((void*)0)\0"
                            as *const u8 as *const core::ffi::c_char,
                        b"/home/runner/work/libavif-rs/libavif-rs/upstream/libavif/src/read.c\0"
                            as *const u8 as *const core::ffi::c_char,
                        7055 as core::ffi::c_uint,
                        (::core::mem::transmute::<
                            [u8; 120],
                            [core::ffi::c_char; 120],
                        >(
                            *b"avifResult avifDecoderApplySampleTransformForPlanesImpl(const avifDecoder *, avifPlanesFlag, avifImage *, avifImage **)\0",
                        ))
                            .as_ptr(),
                    );
                }
            };
            if (*info).tileCount == 1 as core::ffi::c_uint {
                inputImages[i as usize] = (*firstTile).image;
            } else {
                let ref mut fresh0 = *reconstructedInputImages.offset(i as isize);
                *fresh0 = avifImageCreateEmpty();
                if (*reconstructedInputImages.offset(i as isize)).is_null() {
                    return AVIF_RESULT_OUT_OF_MEMORY;
                }
                avifImageCopyNoAlloc(
                    *reconstructedInputImages.offset(i as isize),
                    (*firstTile).image,
                );
                (**reconstructedInputImages.offset(i as isize)).width = (*(*decoder)
                    .image)
                    .width;
                (**reconstructedInputImages.offset(i as isize)).height = (*(*decoder)
                    .image)
                    .height;
                let mut cicpSet: avifBool = AVIF_TRUE;
                let result__: avifResult = avifDecoderDataAllocateImagePlanes(
                    (*decoder).data,
                    info,
                    *reconstructedInputImages.offset(i as isize),
                    &mut cicpSet,
                ) as avifResult;
                if result__ as core::ffi::c_uint
                    != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
                {
                    return result__;
                }
                let mut tileIndex: core::ffi::c_uint = 0 as core::ffi::c_uint;
                while tileIndex < (*info).tileCount {
                    let mut tile: *const avifTile = firstTile.offset(tileIndex as isize);
                    let result___0: avifResult = avifDecoderDataCopyTileToImage(
                        (*decoder).data as *mut avifDecoderData,
                        info,
                        *reconstructedInputImages.offset(i as isize),
                        tile,
                        tileIndex,
                    ) as avifResult;
                    if result___0 as core::ffi::c_uint
                        != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
                    {
                        return result___0;
                    }
                    tileIndex = tileIndex.wrapping_add(1);
                }
                inputImages[i as usize] = *reconstructedInputImages.offset(i as isize);
            }
        }
        i = i.wrapping_add(1);
    }
    let result___1: avifResult = avifImageApplyExpression(
        dstImage,
        AVIF_SAMPLE_TRANSFORM_BIT_DEPTH_32,
        &mut (*(*(*decoder).data).meta).sampleTransformExpression,
        (*(*decoder).data).sampleTransformNumInputImageItems,
        inputImages.as_mut_ptr(),
        planes as avifPlanesFlags,
    ) as avifResult;
    if result___1 as core::ffi::c_uint
        != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
    {
        return result___1;
    }
    return AVIF_RESULT_OK;
}
unsafe extern "C" fn avifDecoderApplySampleTransformForPlanes(
    mut decoder: *const avifDecoder,
    mut planes: avifPlanesFlag,
    mut dstImage: *mut avifImage,
) -> avifResult {
    let mut toDestroy: [*mut avifImage; 3] = [
        0 as *mut avifImage,
        0 as *mut avifImage,
        0 as *mut avifImage,
    ];
    let result: avifResult = avifDecoderApplySampleTransformForPlanesImpl(
        decoder,
        planes,
        dstImage,
        toDestroy.as_mut_ptr(),
    ) as avifResult;
    let mut i: uint32_t = 0 as uint32_t;
    while i
        < (1 as core::ffi::c_int
            + (AVIF_ITEM_SAMPLE_TRANSFORM_INPUT_0_ALPHA as core::ffi::c_int
                - AVIF_ITEM_SAMPLE_TRANSFORM_INPUT_0_COLOR as core::ffi::c_int))
            as uint32_t
    {
        if !(toDestroy[i as usize]).is_null() {
            avifImageDestroy(toDestroy[i as usize]);
        }
        i = i.wrapping_add(1);
    }
    return result;
}
unsafe extern "C" fn avifDecoderApplySampleTransform(
    mut decoder: *const avifDecoder,
    mut dstImage: *mut avifImage,
) -> avifResult {
    if (*dstImage).depth != (*(*(*decoder).data).meta).sampleTransformDepth {
        'c_36165: {
            if !((*dstImage).yuvPlanes[0 as core::ffi::c_int as usize]).is_null()
            {} else {
                __assert_fail(
                    b"dstImage->yuvPlanes[0] != ((void*)0)\0" as *const u8
                        as *const core::ffi::c_char,
                    b"/home/runner/work/libavif-rs/libavif-rs/upstream/libavif/src/read.c\0"
                        as *const u8 as *const core::ffi::c_char,
                    7100 as core::ffi::c_uint,
                    (::core::mem::transmute::<
                        [u8; 77],
                        [core::ffi::c_char; 77],
                    >(
                        *b"avifResult avifDecoderApplySampleTransform(const avifDecoder *, avifImage *)\0",
                    ))
                        .as_ptr(),
                );
            }
        };
        'c_36129: {
            if (*dstImage).imageOwnsYUVPlanes != 0 {} else {
                __assert_fail(
                    b"dstImage->imageOwnsYUVPlanes\0" as *const u8
                        as *const core::ffi::c_char,
                    b"/home/runner/work/libavif-rs/libavif-rs/upstream/libavif/src/read.c\0"
                        as *const u8 as *const core::ffi::c_char,
                    7101 as core::ffi::c_uint,
                    (::core::mem::transmute::<
                        [u8; 77],
                        [core::ffi::c_char; 77],
                    >(
                        *b"avifResult avifDecoderApplySampleTransform(const avifDecoder *, avifImage *)\0",
                    ))
                        .as_ptr(),
                );
            }
        };
        let mut dstImageWithCorrectDepth: *mut avifImage = avifImageCreate(
            (*dstImage).width,
            (*dstImage).height,
            (*(*(*decoder).data).meta).sampleTransformDepth,
            (*dstImage).yuvFormat,
        );
        if dstImageWithCorrectDepth.is_null() {
            return AVIF_RESULT_OUT_OF_MEMORY;
        }
        (*dstImageWithCorrectDepth).yuvRange = (*dstImage).yuvRange;
        let mut result: avifResult = avifImageAllocatePlanes(
            dstImageWithCorrectDepth,
            (if !((*dstImage).alphaPlane).is_null() {
                AVIF_PLANES_ALL as core::ffi::c_int
            } else {
                AVIF_PLANES_YUV as core::ffi::c_int
            }) as avifPlanesFlags,
        );
        if result as core::ffi::c_uint
            == AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
        {
            result = avifDecoderApplySampleTransform(decoder, dstImageWithCorrectDepth);
            if result as core::ffi::c_uint
                == AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
            {
                avifImageFreePlanes(
                    dstImage,
                    AVIF_PLANES_ALL as core::ffi::c_int as avifPlanesFlags,
                );
                (*dstImage).depth = (*dstImageWithCorrectDepth).depth;
                avifImageStealPlanes(
                    dstImage,
                    dstImageWithCorrectDepth,
                    AVIF_PLANES_ALL as core::ffi::c_int as avifPlanesFlags,
                );
            }
        }
        avifImageDestroy(dstImageWithCorrectDepth);
        return result;
    }
    let result__: avifResult = avifDecoderApplySampleTransformForPlanes(
        decoder,
        AVIF_PLANES_YUV,
        dstImage,
    ) as avifResult;
    if result__ as core::ffi::c_uint
        != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
    {
        return result__;
    }
    if (*(*decoder).data)
        .tileInfos[AVIF_ITEM_ALPHA as core::ffi::c_int as usize]
        .tileCount != 0 as core::ffi::c_uint
    {
        let result___0: avifResult = avifDecoderApplySampleTransformForPlanes(
            decoder,
            AVIF_PLANES_A,
            dstImage,
        ) as avifResult;
        if result___0 as core::ffi::c_uint
            != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
        {
            return result___0;
        }
    }
    return AVIF_RESULT_OK;
}
#[no_mangle]
pub unsafe extern "C" fn avifDecoderNextImage(
    mut decoder: *mut avifDecoder,
) -> avifResult {
    avifDiagnosticsClearError(&mut (*decoder).diag);
    if ((*decoder).data).is_null() || (*(*decoder).data).tiles.count == 0 as uint32_t {
        return AVIF_RESULT_NO_CONTENT;
    }
    if ((*decoder).io).is_null() || ((*(*decoder).io).read).is_none() {
        return AVIF_RESULT_IO_NOT_SET;
    }
    if avifDecoderDataFrameFullyDecoded((*decoder).data) != 0 {
        let mut c: core::ffi::c_int = 0 as core::ffi::c_int;
        while c < AVIF_ITEM_CATEGORY_COUNT as core::ffi::c_int {
            (*(*decoder).data).tileInfos[c as usize].decodedTileCount = 0
                as core::ffi::c_uint;
            c += 1;
        }
    }
    'c_39009: {
        if (*(*decoder).data).tiles.count
            == ((*(*decoder).data)
                .tileInfos[(AVIF_ITEM_CATEGORY_COUNT as core::ffi::c_int
                    - 1 as core::ffi::c_int) as usize]
                .firstTileIndex as uint32_t)
                .wrapping_add(
                    (*(*decoder).data)
                        .tileInfos[(AVIF_ITEM_CATEGORY_COUNT as core::ffi::c_int
                            - 1 as core::ffi::c_int) as usize]
                        .tileCount as uint32_t,
                )
        {} else {
            __assert_fail(
                b"decoder->data->tiles.count == (decoder->data->tileInfos[AVIF_ITEM_CATEGORY_COUNT - 1].firstTileIndex + decoder->data->tileInfos[AVIF_ITEM_CATEGORY_COUNT - 1].tileCount)\0"
                    as *const u8 as *const core::ffi::c_char,
                b"/home/runner/work/libavif-rs/libavif-rs/upstream/libavif/src/read.c\0"
                    as *const u8 as *const core::ffi::c_char,
                7154 as core::ffi::c_uint,
                (::core::mem::transmute::<
                    [u8; 47],
                    [core::ffi::c_char; 47],
                >(*b"avifResult avifDecoderNextImage(avifDecoder *)\0"))
                    .as_ptr(),
            );
        }
    };
    let nextImageIndex: uint32_t = ((*decoder).imageIndex + 1 as core::ffi::c_int)
        as uint32_t;
    if ((*((*(*decoder).data).tiles.tile).offset(0 as core::ffi::c_int as isize)).codec)
        .is_null()
    {
        let result__: avifResult = avifDecoderCreateCodecs(decoder) as avifResult;
        if result__ as core::ffi::c_uint
            != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
        {
            return result__;
        }
    }
    let mut prepareTileResult: [avifResult; 8] = [AVIF_RESULT_OK; 8];
    let mut c_0: core::ffi::c_int = 0 as core::ffi::c_int;
    while c_0 < AVIF_ITEM_CATEGORY_COUNT as core::ffi::c_int {
        prepareTileResult[c_0 as usize] = avifDecoderPrepareTiles(
            decoder,
            nextImageIndex,
            &mut *((*(*decoder).data).tileInfos).as_mut_ptr().offset(c_0 as isize),
        );
        if (*decoder).allowIncremental == 0
            || prepareTileResult[c_0 as usize] as core::ffi::c_uint
                != AVIF_RESULT_WAITING_ON_IO as core::ffi::c_int as core::ffi::c_uint
        {
            let result___0: avifResult = prepareTileResult[c_0 as usize];
            if result___0 as core::ffi::c_uint
                != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
            {
                return result___0;
            }
        }
        c_0 += 1;
    }
    let mut c_1: core::ffi::c_int = 0 as core::ffi::c_int;
    while c_1 < AVIF_ITEM_CATEGORY_COUNT as core::ffi::c_int {
        let result___1: avifResult = avifDecoderDecodeTiles(
            decoder,
            nextImageIndex,
            &mut *((*(*decoder).data).tileInfos).as_mut_ptr().offset(c_1 as isize),
        ) as avifResult;
        if result___1 as core::ffi::c_uint
            != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
        {
            return result___1;
        }
        c_1 += 1;
    }
    if avifDecoderDataFrameFullyDecoded((*decoder).data) == 0 {
        'c_36472: {
            if (*decoder).allowIncremental != 0 {} else {
                __assert_fail(
                    b"decoder->allowIncremental\0" as *const u8
                        as *const core::ffi::c_char,
                    b"/home/runner/work/libavif-rs/libavif-rs/upstream/libavif/src/read.c\0"
                        as *const u8 as *const core::ffi::c_char,
                    7183 as core::ffi::c_uint,
                    (::core::mem::transmute::<
                        [u8; 47],
                        [core::ffi::c_char; 47],
                    >(*b"avifResult avifDecoderNextImage(avifDecoder *)\0"))
                        .as_ptr(),
                );
            }
        };
        let mut firstNonOkResult: avifResult = AVIF_RESULT_OK;
        let mut c_2: core::ffi::c_int = 0 as core::ffi::c_int;
        while c_2 < AVIF_ITEM_CATEGORY_COUNT as core::ffi::c_int {
            'c_36398: {
                if prepareTileResult[c_2 as usize] as core::ffi::c_uint
                    == AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
                    || prepareTileResult[c_2 as usize] as core::ffi::c_uint
                        == AVIF_RESULT_WAITING_ON_IO as core::ffi::c_int
                            as core::ffi::c_uint
                {} else {
                    __assert_fail(
                        b"prepareTileResult[c] == AVIF_RESULT_OK || prepareTileResult[c] == AVIF_RESULT_WAITING_ON_IO\0"
                            as *const u8 as *const core::ffi::c_char,
                        b"/home/runner/work/libavif-rs/libavif-rs/upstream/libavif/src/read.c\0"
                            as *const u8 as *const core::ffi::c_char,
                        7188 as core::ffi::c_uint,
                        (::core::mem::transmute::<
                            [u8; 47],
                            [core::ffi::c_char; 47],
                        >(*b"avifResult avifDecoderNextImage(avifDecoder *)\0"))
                            .as_ptr(),
                    );
                }
            };
            if firstNonOkResult as core::ffi::c_uint
                == AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
            {
                firstNonOkResult = prepareTileResult[c_2 as usize];
            }
            c_2 += 1;
        }
        'c_36331: {
            if firstNonOkResult as core::ffi::c_uint
                != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
            {} else {
                __assert_fail(
                    b"firstNonOkResult != AVIF_RESULT_OK\0" as *const u8
                        as *const core::ffi::c_char,
                    b"/home/runner/work/libavif-rs/libavif-rs/upstream/libavif/src/read.c\0"
                        as *const u8 as *const core::ffi::c_char,
                    7193 as core::ffi::c_uint,
                    (::core::mem::transmute::<
                        [u8; 47],
                        [core::ffi::c_char; 47],
                    >(*b"avifResult avifDecoderNextImage(avifDecoder *)\0"))
                        .as_ptr(),
                );
            }
        };
        return AVIF_RESULT_WAITING_ON_IO;
    }
    let mut c_3: core::ffi::c_int = 0 as core::ffi::c_int;
    while c_3 < AVIF_ITEM_CATEGORY_COUNT as core::ffi::c_int {
        'c_36265: {
            if prepareTileResult[c_3 as usize] as core::ffi::c_uint
                == AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
            {} else {
                __assert_fail(
                    b"prepareTileResult[c] == AVIF_RESULT_OK\0" as *const u8
                        as *const core::ffi::c_char,
                    b"/home/runner/work/libavif-rs/libavif-rs/upstream/libavif/src/read.c\0"
                        as *const u8 as *const core::ffi::c_char,
                    7198 as core::ffi::c_uint,
                    (::core::mem::transmute::<
                        [u8; 47],
                        [core::ffi::c_char; 47],
                    >(*b"avifResult avifDecoderNextImage(avifDecoder *)\0"))
                        .as_ptr(),
                );
            }
        };
        c_3 += 1;
    }
    if (*(*decoder).data)
        .tileInfos[AVIF_ITEM_COLOR as core::ffi::c_int as usize]
        .tileCount != 0 as core::ffi::c_uint
        && (*(*(*decoder).data).meta).sampleTransformExpression.count > 0 as uint32_t
    {
        let result___2: avifResult = avifDecoderApplySampleTransform(
            decoder,
            (*decoder).image,
        ) as avifResult;
        if result___2 as core::ffi::c_uint
            != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
        {
            return result___2;
        }
    }
    (*decoder).imageIndex = nextImageIndex as core::ffi::c_int;
    if !((*(*decoder).data).sourceSampleTable).is_null() {
        let mut timingResult: avifResult = avifDecoderNthImageTiming(
            decoder,
            (*decoder).imageIndex as uint32_t,
            &mut (*decoder).imageTiming,
        );
        if timingResult as core::ffi::c_uint
            != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
        {
            return timingResult;
        }
    }
    return AVIF_RESULT_OK;
}
#[no_mangle]
pub unsafe extern "C" fn avifDecoderNthImageTiming(
    mut decoder: *const avifDecoder,
    mut frameIndex: uint32_t,
    mut outTiming: *mut avifImageTiming,
) -> avifResult {
    if ((*decoder).data).is_null() {
        return AVIF_RESULT_NO_CONTENT;
    }
    if frameIndex > INT_MAX as uint32_t
        || frameIndex as core::ffi::c_int >= (*decoder).imageCount
    {
        return AVIF_RESULT_NO_IMAGES_REMAINING;
    }
    if ((*(*decoder).data).sourceSampleTable).is_null() {
        *outTiming = (*decoder).imageTiming;
        return AVIF_RESULT_OK;
    }
    (*outTiming).timescale = (*decoder).timescale;
    (*outTiming).ptsInTimescales = 0 as uint64_t;
    let mut imageIndex: uint32_t = 0 as uint32_t;
    while imageIndex < frameIndex {
        (*outTiming).ptsInTimescales = ((*outTiming).ptsInTimescales)
            .wrapping_add(
                avifSampleTableGetImageDelta(
                    (*(*decoder).data).sourceSampleTable,
                    imageIndex,
                ) as uint64_t,
            );
        imageIndex = imageIndex.wrapping_add(1);
    }
    (*outTiming).durationInTimescales = avifSampleTableGetImageDelta(
        (*(*decoder).data).sourceSampleTable,
        frameIndex,
    ) as uint64_t;
    if (*outTiming).timescale > 0 as uint64_t {
        (*outTiming).pts = (*outTiming).ptsInTimescales as core::ffi::c_double
            / (*outTiming).timescale as core::ffi::c_double;
        (*outTiming).duration = (*outTiming).durationInTimescales as core::ffi::c_double
            / (*outTiming).timescale as core::ffi::c_double;
    } else {
        (*outTiming).pts = 0.0f64;
        (*outTiming).duration = 0.0f64;
    }
    return AVIF_RESULT_OK;
}
#[no_mangle]
pub unsafe extern "C" fn avifDecoderNthImage(
    mut decoder: *mut avifDecoder,
    mut frameIndex: uint32_t,
) -> avifResult {
    avifDiagnosticsClearError(&mut (*decoder).diag);
    if ((*decoder).data).is_null() {
        return AVIF_RESULT_NO_CONTENT;
    }
    if frameIndex > INT_MAX as uint32_t
        || frameIndex as core::ffi::c_int >= (*decoder).imageCount
    {
        return AVIF_RESULT_NO_IMAGES_REMAINING;
    }
    let mut requestedIndex: core::ffi::c_int = frameIndex as core::ffi::c_int;
    if requestedIndex == (*decoder).imageIndex + 1 as core::ffi::c_int {
        return avifDecoderNextImage(decoder);
    }
    if requestedIndex == (*decoder).imageIndex {
        if avifDecoderDataFrameFullyDecoded((*decoder).data) != 0 {
            return AVIF_RESULT_OK;
        }
    }
    let mut nearestKeyFrame: core::ffi::c_int = avifDecoderNearestKeyframe(
        decoder,
        frameIndex,
    ) as core::ffi::c_int;
    if nearestKeyFrame > (*decoder).imageIndex + 1 as core::ffi::c_int
        || requestedIndex <= (*decoder).imageIndex
    {
        (*decoder).imageIndex = nearestKeyFrame - 1 as core::ffi::c_int;
        avifDecoderDataResetCodec((*decoder).data as *mut avifDecoderData);
    }
    loop {
        let mut result: avifResult = avifDecoderNextImage(decoder);
        if result as core::ffi::c_uint
            != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
        {
            return result;
        }
        if requestedIndex == (*decoder).imageIndex {
            break;
        }
    }
    return AVIF_RESULT_OK;
}
#[no_mangle]
pub unsafe extern "C" fn avifDecoderIsKeyframe(
    mut decoder: *const avifDecoder,
    mut frameIndex: uint32_t,
) -> avifBool {
    if ((*decoder).data).is_null() || (*(*decoder).data).tiles.count == 0 as uint32_t {
        return AVIF_FALSE;
    }
    let mut i: core::ffi::c_uint = 0 as core::ffi::c_uint;
    while (i as uint32_t) < (*(*decoder).data).tiles.count {
        let mut tile: *const avifTile = &mut *((*(*decoder).data).tiles.tile)
            .offset(i as isize) as *mut avifTile;
        if frameIndex >= (*(*tile).input).samples.count
            || (*((*(*tile).input).samples.sample).offset(frameIndex as isize)).sync == 0
        {
            return AVIF_FALSE;
        }
        i = i.wrapping_add(1);
    }
    return AVIF_TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn avifDecoderNearestKeyframe(
    mut decoder: *const avifDecoder,
    mut frameIndex: uint32_t,
) -> uint32_t {
    if ((*decoder).data).is_null() {
        return 0 as uint32_t;
    }
    while frameIndex != 0 as uint32_t {
        if avifDecoderIsKeyframe(decoder, frameIndex) != 0 {
            break;
        }
        frameIndex = frameIndex.wrapping_sub(1);
    }
    return frameIndex;
}
unsafe extern "C" fn avifGetDecodedRowCount(
    mut decoder: *const avifDecoder,
    mut info: *const avifTileInfo,
    mut image: *const avifImage,
) -> uint32_t {
    if (*info).decodedTileCount == (*info).tileCount {
        return (*image).height;
    }
    if (*info).decodedTileCount == 0 as core::ffi::c_uint {
        return 0 as uint32_t;
    }
    if (*(*(*decoder).data).meta).sampleTransformExpression.count > 0 as uint32_t {
        return 0 as uint32_t;
    }
    if (*info).grid.rows > 0 as uint32_t && (*info).grid.columns > 0 as uint32_t {
        let tileHeight: uint32_t = (*((*(*decoder).data).tiles.tile)
            .offset((*info).firstTileIndex as isize))
            .height;
        return if ((*info).decodedTileCount as uint32_t)
            .wrapping_div((*info).grid.columns)
            .wrapping_mul(tileHeight) < (*image).height
        {
            ((*info).decodedTileCount as uint32_t)
                .wrapping_div((*info).grid.columns)
                .wrapping_mul(tileHeight)
        } else {
            (*image).height
        };
    } else {
        return (*image).height
    };
}
#[no_mangle]
pub unsafe extern "C" fn avifDecoderDecodedRowCount(
    mut decoder: *const avifDecoder,
) -> uint32_t {
    if (*(*decoder).data)
        .tileInfos[AVIF_ITEM_COLOR as core::ffi::c_int as usize]
        .tileCount == 0 as core::ffi::c_uint
    {
        return 0 as uint32_t;
    }
    let mut minRowCount: uint32_t = (*(*decoder).image).height;
    let mut c: core::ffi::c_int = 0 as core::ffi::c_int;
    while c < AVIF_ITEM_CATEGORY_COUNT as core::ffi::c_int {
        if c == AVIF_ITEM_GAIN_MAP as core::ffi::c_int {
            let gainMap: *const avifImage = if !((*(*decoder).image).gainMap).is_null() {
                (*(*(*decoder).image).gainMap).image
            } else {
                0 as *mut avifImage
            };
            if !gainMap.is_null() && (*gainMap).height != 0 as uint32_t
                && (*(*decoder).data)
                    .tileInfos[AVIF_ITEM_GAIN_MAP as core::ffi::c_int as usize]
                    .tileCount != 0 as core::ffi::c_uint
            {
                let mut gainMapRowCount: uint32_t = avifGetDecodedRowCount(
                    decoder,
                    &mut *((*(*decoder).data).tileInfos)
                        .as_mut_ptr()
                        .offset(AVIF_ITEM_GAIN_MAP as core::ffi::c_int as isize),
                    gainMap,
                );
                if (*gainMap).height != (*(*decoder).image).height {
                    let scaledGainMapRowCount: uint32_t = floorf(
                        gainMapRowCount as core::ffi::c_float
                            / (*gainMap).height as core::ffi::c_float
                            * (*(*decoder).image).height as core::ffi::c_float,
                    ) as uint32_t;
                    if !(lround(
                        scaledGainMapRowCount as core::ffi::c_double
                            / (*(*decoder).image).height as core::ffi::c_double
                            * (*(*(*(*decoder).image).gainMap).image).height
                                as core::ffi::c_double,
                    ) as uint32_t <= gainMapRowCount)
                    {
                        return 0 as uint32_t;
                    }
                    gainMapRowCount = scaledGainMapRowCount;
                }
                minRowCount = if minRowCount < gainMapRowCount {
                    minRowCount
                } else {
                    gainMapRowCount
                };
            }
        } else {
            let rowCount: uint32_t = avifGetDecodedRowCount(
                decoder,
                &mut *((*(*decoder).data).tileInfos).as_mut_ptr().offset(c as isize),
                (*decoder).image,
            ) as uint32_t;
            minRowCount = if minRowCount < rowCount { minRowCount } else { rowCount };
        }
        c += 1;
    }
    return minRowCount;
}
#[no_mangle]
pub unsafe extern "C" fn avifDecoderRead(
    mut decoder: *mut avifDecoder,
    mut image: *mut avifImage,
) -> avifResult {
    let mut result: avifResult = avifDecoderParse(decoder);
    if result as core::ffi::c_uint
        != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
    {
        return result;
    }
    result = avifDecoderNextImage(decoder);
    if result as core::ffi::c_uint
        != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
    {
        return result;
    }
    return avifImageCopy(
        image,
        (*decoder).image,
        AVIF_PLANES_ALL as core::ffi::c_int as avifPlanesFlags,
    );
}
#[no_mangle]
pub unsafe extern "C" fn avifDecoderReadMemory(
    mut decoder: *mut avifDecoder,
    mut image: *mut avifImage,
    mut data: *const uint8_t,
    mut size: size_t,
) -> avifResult {
    avifDiagnosticsClearError(&mut (*decoder).diag);
    let mut result: avifResult = avifDecoderSetIOMemory(decoder, data, size);
    if result as core::ffi::c_uint
        != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
    {
        return result;
    }
    return avifDecoderRead(decoder, image);
}
#[no_mangle]
pub unsafe extern "C" fn avifDecoderReadFile(
    mut decoder: *mut avifDecoder,
    mut image: *mut avifImage,
    mut filename: *const core::ffi::c_char,
) -> avifResult {
    avifDiagnosticsClearError(&mut (*decoder).diag);
    let mut result: avifResult = avifDecoderSetIOFile(decoder, filename);
    if result as core::ffi::c_uint
        != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
    {
        return result;
    }
    return avifDecoderRead(decoder, image);
}
pub const __INT_MAX__: core::ffi::c_int = 2147483647 as core::ffi::c_int;
