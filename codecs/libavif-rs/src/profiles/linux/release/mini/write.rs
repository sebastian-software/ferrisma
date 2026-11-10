// Copyright 2019 Joe Drago. All rights reserved.
// SPDX-License-Identifier: BSD-2-Clause

// Mechanically translated from upstream/libavif/src/write.c.
// C ABI, ownership, pointer validity, and bounds follow the original headers.
extern "C" {
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
    fn avifImageDestroy(image: *mut avifImage);
    fn avifImageAllocatePlanes(
        image: *mut avifImage,
        planes: avifPlanesFlags,
    ) -> avifResult;
    fn avifCodecName(
        choice: avifCodecChoice,
        requiredFlags: avifCodecFlags,
    ) -> *const core::ffi::c_char;
    fn avifImageUsesU16(image: *const avifImage) -> avifBool;
    fn avifImageIsOpaque(image: *const avifImage) -> avifBool;
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
    fn avifHTONL(l: uint32_t) -> uint32_t;
    fn avifArrayCreate(
        arrayStruct: *mut core::ffi::c_void,
        elementSize: uint32_t,
        initialCapacity: uint32_t,
    ) -> avifBool;
    fn avifArrayPush(arrayStruct: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn avifArrayPop(arrayStruct: *mut core::ffi::c_void);
    fn avifArrayDestroy(arrayStruct: *mut core::ffi::c_void);
    fn avifSampleTransformRecipeToExpression(
        recipe: avifSampleTransformRecipe,
        expression: *mut avifSampleTransformExpression,
    ) -> avifResult;
    fn avifImageApplyOperations(
        dstImage: *mut avifImage,
        bitDepth: avifSampleTransformBitDepth,
        numTokens: uint32_t,
        tokens: *const avifSampleTransformToken,
        numInputImageItems: uint8_t,
        inputImageItems: *mut *const avifImage,
        planes: avifPlanesFlags,
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
    fn avifImageIrotImirToExifOrientation(image: *const avifImage) -> uint8_t;
    fn avifCodecSpecificOptionsCreate() -> *mut avifCodecSpecificOptions;
    fn avifCodecSpecificOptionsClear(csOptions: *mut avifCodecSpecificOptions);
    fn avifCodecSpecificOptionsDestroy(csOptions: *mut avifCodecSpecificOptions);
    fn avifCodecSpecificOptionsSet(
        csOptions: *mut avifCodecSpecificOptions,
        key: *const core::ffi::c_char,
        value: *const core::ffi::c_char,
    ) -> avifResult;
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
    fn avifRWStreamStart(stream: *mut avifRWStream, raw: *mut avifRWData);
    fn avifRWStreamOffset(stream: *const avifRWStream) -> size_t;
    fn avifRWStreamSetOffset(stream: *mut avifRWStream, offset: size_t);
    fn avifRWStreamFinishWrite(stream: *mut avifRWStream);
    fn avifRWStreamWrite(
        stream: *mut avifRWStream,
        data: *const core::ffi::c_void,
        size: size_t,
    ) -> avifResult;
    fn avifRWStreamWriteChars(
        stream: *mut avifRWStream,
        chars: *const core::ffi::c_char,
        size: size_t,
    ) -> avifResult;
    fn avifRWStreamWriteBox(
        stream: *mut avifRWStream,
        type_0: *const core::ffi::c_char,
        contentSize: size_t,
        marker: *mut avifBoxMarker,
    ) -> avifResult;
    fn avifRWStreamWriteFullBox(
        stream: *mut avifRWStream,
        type_0: *const core::ffi::c_char,
        contentSize: size_t,
        version: core::ffi::c_int,
        flags: uint32_t,
        marker: *mut avifBoxMarker,
    ) -> avifResult;
    fn avifRWStreamFinishBox(
        stream: *mut avifRWStream,
        marker: avifBoxMarker,
    ) -> avifResult;
    fn avifRWStreamWriteU8(stream: *mut avifRWStream, v: uint8_t) -> avifResult;
    fn avifRWStreamWriteU16(stream: *mut avifRWStream, v: uint16_t) -> avifResult;
    fn avifRWStreamWriteU32(stream: *mut avifRWStream, v: uint32_t) -> avifResult;
    fn avifRWStreamWriteU64(stream: *mut avifRWStream, v: uint64_t) -> avifResult;
    fn avifRWStreamWriteZeros(
        stream: *mut avifRWStream,
        byteCount: size_t,
    ) -> avifResult;
    fn avifRWStreamWriteBits(
        stream: *mut avifRWStream,
        v: uint32_t,
        bitCount: size_t,
    ) -> avifResult;
    fn avifSequenceHeaderParse(
        header: *mut avifSequenceHeader,
        sample: *const avifROData,
        codecType: avifCodecType,
    ) -> avifBool;
    fn avifGainMapValidateMetadata(
        gainMap: *const avifGainMap,
        diag: *mut avifDiagnostics,
    ) -> avifResult;
    fn avifSameGainMapMetadata(a: *const avifGainMap, b: *const avifGainMap) -> avifBool;
    fn avifSameGainMapAltMetadata(
        a: *const avifGainMap,
        b: *const avifGainMap,
    ) -> avifBool;
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
    fn strlen(__s: *const core::ffi::c_char) -> size_t;
    fn time(__timer: *mut time_t) -> time_t;
}
pub type size_t = usize;
pub type __uint8_t = u8;
pub type __uint16_t = u16;
pub type __int32_t = i32;
pub type __uint32_t = u32;
pub type __uint64_t = u64;
pub type __time_t = core::ffi::c_long;
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
pub type avifChannelIndex = core::ffi::c_uint;
pub const AVIF_CHAN_A: avifChannelIndex = 3;
pub const AVIF_CHAN_V: avifChannelIndex = 2;
pub const AVIF_CHAN_U: avifChannelIndex = 1;
pub const AVIF_CHAN_Y: avifChannelIndex = 0;
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
pub type avifHeaderFormat = core::ffi::c_uint;
pub const AVIF_HEADER_FULL: avifHeaderFormat = 0;
pub const AVIF_HEADER_MINI: avifHeaderFormat = 1;
pub const AVIF_HEADER_DEFAULT: avifHeaderFormat = 0;
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
pub struct avifIOStats {
    pub colorOBUSize: size_t,
    pub alphaOBUSize: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct avifEncoderData {
    pub items: avifEncoderItemArray,
    pub frames: avifEncoderFrameArray,
    pub quality: core::ffi::c_int,
    pub qualityAlpha: core::ffi::c_int,
    pub qualityGainMap: core::ffi::c_int,
    pub tileRowsLog2: core::ffi::c_int,
    pub tileColsLog2: core::ffi::c_int,
    pub lastEncoder: avifEncoder,
    pub lastQuality: core::ffi::c_int,
    pub lastQualityAlpha: core::ffi::c_int,
    pub lastTileRowsLog2: core::ffi::c_int,
    pub lastTileColsLog2: core::ffi::c_int,
    pub imageMetadata: *mut avifImage,
    pub altImageMetadata: *mut avifImage,
    pub lastItemID: uint32_t,
    pub primaryItemID: uint32_t,
    pub alternativeItemIDs: avifEncoderItemIdArray,
    pub singleImage: avifBool,
    pub alphaPresent: avifBool,
    pub gainMapSizeBytes: size_t,
    pub imageItemType: *const core::ffi::c_char,
    pub configPropName: *const core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct avifEncoderItemIdArray {
    pub itemID: *mut uint32_t,
    pub elementSize: uint32_t,
    pub count: uint32_t,
    pub capacity: uint32_t,
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
pub struct avifEncoderFrameArray {
    pub frame: *mut avifEncoderFrame,
    pub elementSize: uint32_t,
    pub count: uint32_t,
    pub capacity: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct avifEncoderFrame {
    pub durationInTimescales: uint64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct avifEncoderItemArray {
    pub item: *mut avifEncoderItem,
    pub elementSize: uint32_t,
    pub count: uint32_t,
    pub capacity: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct avifEncoderItem {
    pub id: uint32_t,
    pub type_0: [uint8_t; 4],
    pub codec: *mut avifCodec,
    pub encodeOutput: *mut avifCodecEncodeOutput,
    pub metadataPayload: avifRWData,
    pub av1C: avifCodecConfigurationBox,
    pub cellIndex: uint32_t,
    pub itemCategory: avifItemCategory,
    pub hiddenImage: avifBool,
    pub infeName: *const core::ffi::c_char,
    pub infeNameSize: size_t,
    pub infeContentType: *const core::ffi::c_char,
    pub infeContentTypeSize: size_t,
    pub mdatFixups: avifOffsetFixupArray,
    pub irefToID: uint32_t,
    pub irefType: *const core::ffi::c_char,
    pub gridCols: uint32_t,
    pub gridRows: uint32_t,
    pub gridWidth: uint32_t,
    pub gridHeight: uint32_t,
    pub extraLayerCount: uint32_t,
    pub dimgFromID: uint32_t,
    pub associations: avifItemPropertyAssociationArray,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct avifItemPropertyAssociationArray {
    pub association: *mut avifItemPropertyAssociation,
    pub elementSize: uint32_t,
    pub count: uint32_t,
    pub capacity: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct avifItemPropertyAssociation {
    pub property_index: uint8_t,
    pub essential: avifBool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct avifOffsetFixupArray {
    pub fixup: *mut avifOffsetFixup,
    pub elementSize: uint32_t,
    pub count: uint32_t,
    pub capacity: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct avifOffsetFixup {
    pub offset: size_t,
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
pub type avifAddImageFlags = uint32_t;
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
pub struct avifRWStream {
    pub raw: *mut avifRWData,
    pub offset: size_t,
    pub numUsedBitsInPartialByte: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct avifItemPropertyDedup {
    pub properties: avifItemPropertyArray,
    pub s: avifRWStream,
    pub buffer: avifRWData,
    pub nextIndex: uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct avifItemPropertyArray {
    pub property: *mut avifItemProperty,
    pub elementSize: uint32_t,
    pub count: uint32_t,
    pub capacity: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct avifItemProperty {
    pub index: uint8_t,
    pub offset: size_t,
    pub size: size_t,
}
pub type avifBoxMarker = size_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct avifEncoderItemReferenceArray {
    pub ref_0: *mut avifEncoderItemReference,
    pub elementSize: uint32_t,
    pub count: uint32_t,
    pub capacity: uint32_t,
}
pub type avifEncoderItemReference = *mut avifEncoderItem;
pub type time_t = __time_t;
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
pub type avifCodecType = core::ffi::c_uint;
pub const AVIF_CODEC_TYPE_AV1: avifCodecType = 1;
pub const AVIF_CODEC_TYPE_UNKNOWN: avifCodecType = 0;
pub const AVIF_ADD_IMAGE_FLAG_SINGLE: avifAddImageFlag = 2;
pub const AVIF_ADD_IMAGE_FLAG_FORCE_KEYFRAME: avifAddImageFlag = 1;
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
pub type avifSampleTransformBitDepth = core::ffi::c_uint;
pub const AVIF_SAMPLE_TRANSFORM_BIT_DEPTH_64: avifSampleTransformBitDepth = 3;
pub const AVIF_SAMPLE_TRANSFORM_BIT_DEPTH_32: avifSampleTransformBitDepth = 2;
pub const AVIF_SAMPLE_TRANSFORM_BIT_DEPTH_16: avifSampleTransformBitDepth = 1;
pub const AVIF_SAMPLE_TRANSFORM_BIT_DEPTH_8: avifSampleTransformBitDepth = 0;
pub const AVIF_ENCODER_CHANGE_CODEC_SPECIFIC: avifEncoderChange = 1073741824;
pub const AVIF_ENCODER_CHANGE_SCALING_MODE: avifEncoderChange = 256;
pub const AVIF_ENCODER_CHANGE_TILE_COLS_LOG2: avifEncoderChange = 32;
pub const AVIF_ENCODER_CHANGE_TILE_ROWS_LOG2: avifEncoderChange = 16;
pub const AVIF_ENCODER_CHANGE_MAX_QUANTIZER_ALPHA: avifEncoderChange = 8;
pub const AVIF_ENCODER_CHANGE_MIN_QUANTIZER_ALPHA: avifEncoderChange = 4;
pub const AVIF_ENCODER_CHANGE_MAX_QUANTIZER: avifEncoderChange = 2;
pub const AVIF_ENCODER_CHANGE_MIN_QUANTIZER: avifEncoderChange = 1;
pub const AVIF_ENCODER_CHANGE_QUALITY_ALPHA: avifEncoderChange = 128;
pub const AVIF_ENCODER_CHANGE_QUALITY: avifEncoderChange = 64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct avifSampleTransformExpression {
    pub tokens: *mut avifSampleTransformToken,
    pub elementSize: uint32_t,
    pub count: uint32_t,
    pub capacity: uint32_t,
}
pub type avifAddImageFlag = core::ffi::c_uint;
pub const AVIF_ADD_IMAGE_FLAG_NONE: avifAddImageFlag = 0;
pub type avifEncoderChange = core::ffi::c_uint;
pub const NULL: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
pub const UINT16_MAX: core::ffi::c_int = 65535 as core::ffi::c_int;
pub const UINT64_MAX: core::ffi::c_ulong = 18446744073709551615 as core::ffi::c_ulong;
pub const AVIF_TRUE: core::ffi::c_int = 1 as core::ffi::c_int;
pub const AVIF_FALSE: core::ffi::c_int = 0 as core::ffi::c_int;
pub const AVIF_DEFAULT_IMAGE_SIZE_LIMIT: core::ffi::c_int = 16384 as core::ffi::c_int
    * 16384 as core::ffi::c_int;
pub const AVIF_DEFAULT_IMAGE_DIMENSION_LIMIT: core::ffi::c_int = 32768
    as core::ffi::c_int;
pub const AVIF_QUALITY_DEFAULT: core::ffi::c_int = -(1 as core::ffi::c_int);
pub const AVIF_QUALITY_LOSSLESS: core::ffi::c_int = 100 as core::ffi::c_int;
pub const AVIF_QUANTIZER_LOSSLESS: core::ffi::c_int = 0 as core::ffi::c_int;
pub const AVIF_QUANTIZER_BEST_QUALITY: core::ffi::c_int = 0 as core::ffi::c_int;
pub const AVIF_QUANTIZER_WORST_QUALITY: core::ffi::c_int = 63 as core::ffi::c_int;
pub const AVIF_SPEED_DEFAULT: core::ffi::c_int = -(1 as core::ffi::c_int);
pub const AVIF_REPETITION_COUNT_INFINITE: core::ffi::c_int = -(1 as core::ffi::c_int);
pub const AVIF_MAX_AV1_LAYER_COUNT: core::ffi::c_int = 4 as core::ffi::c_int;
pub const AVIF_URN_ALPHA0: [core::ffi::c_char; 44] = unsafe {
    ::core::mem::transmute::<
        [u8; 44],
        [core::ffi::c_char; 44],
    >(*b"urn:mpeg:mpegB:cicp:systems:auxiliary:alpha\0")
};
pub const AVIF_CONTENT_TYPE_XMP: [core::ffi::c_char; 20] = unsafe {
    ::core::mem::transmute::<
        [u8; 20],
        [core::ffi::c_char; 20],
    >(*b"application/rdf+xml\0")
};
pub const AVIF_SPATIAL_ID_UNSET: core::ffi::c_int = 0xff as core::ffi::c_int;
pub const AVIF_INDEFINITE_DURATION64: core::ffi::c_ulong = UINT64_MAX;
static mut alphaURN: [core::ffi::c_char; 44] = AVIF_URN_ALPHA0;
static mut alphaURNSize: size_t = unsafe {
    ::core::mem::size_of::<[core::ffi::c_char; 44]>() as size_t
};
static mut xmpContentType: [core::ffi::c_char; 20] = AVIF_CONTENT_TYPE_XMP;
static mut xmpContentTypeSize: size_t = unsafe {
    ::core::mem::size_of::<[core::ffi::c_char; 20]>() as size_t
};
unsafe extern "C" fn floorLog2(mut n: uint32_t) -> core::ffi::c_int {
    let mut count: core::ffi::c_int = 0 as core::ffi::c_int;
    while n != 0 as uint32_t {
        count += 1;
        n >>= 1 as core::ffi::c_int;
    }
    return count - 1 as core::ffi::c_int;
}
unsafe extern "C" fn splitTilesLog2(
    mut dim1: uint32_t,
    mut dim2: uint32_t,
    mut tilesLog2: core::ffi::c_int,
    mut tileDim1Log2: *mut core::ffi::c_int,
    mut tileDim2Log2: *mut core::ffi::c_int,
) {
    let mut ratio: uint32_t = dim1.wrapping_div(dim2);
    let mut diffLog2: core::ffi::c_int = floorLog2(ratio);
    let mut subtract: core::ffi::c_int = tilesLog2 - diffLog2;
    if subtract < 0 as core::ffi::c_int {
        subtract = 0 as core::ffi::c_int;
    }
    *tileDim2Log2 = subtract / 2 as core::ffi::c_int;
    *tileDim1Log2 = tilesLog2 - *tileDim2Log2;
}
#[no_mangle]
pub unsafe extern "C" fn avifSetTileConfiguration(
    mut threads: core::ffi::c_int,
    mut width: uint32_t,
    mut height: uint32_t,
    mut tileRowsLog2: *mut core::ffi::c_int,
    mut tileColsLog2: *mut core::ffi::c_int,
) {
    *tileRowsLog2 = 0 as core::ffi::c_int;
    *tileColsLog2 = 0 as core::ffi::c_int;
    if threads > 1 as core::ffi::c_int {
        let kMinTileArea: uint32_t = (512 as core::ffi::c_int * 512 as core::ffi::c_int)
            as uint32_t;
        let kMaxTiles: uint32_t = 32 as uint32_t;
        let imageArea: uint64_t = (width as uint64_t).wrapping_mul(height as uint64_t);
        let mut tiles: uint32_t = imageArea
            .wrapping_add(kMinTileArea as uint64_t)
            .wrapping_sub(1 as uint64_t)
            .wrapping_div(kMinTileArea as uint64_t) as uint32_t;
        if tiles > kMaxTiles {
            tiles = kMaxTiles;
        }
        if tiles > threads as uint32_t {
            tiles = threads as uint32_t;
        }
        let mut tilesLog2: core::ffi::c_int = floorLog2(tiles);
        if width >= height {
            splitTilesLog2(width, height, tilesLog2, tileColsLog2, tileRowsLog2);
        } else {
            splitTilesLog2(height, width, tilesLog2, tileRowsLog2, tileColsLog2);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn avifCodecEncodeOutputCreate() -> *mut avifCodecEncodeOutput {
    let mut encodeOutput: *mut avifCodecEncodeOutput = avifAlloc(
        ::core::mem::size_of::<avifCodecEncodeOutput>() as size_t,
    ) as *mut avifCodecEncodeOutput;
    if encodeOutput.is_null() {
        return 0 as *mut avifCodecEncodeOutput;
    }
    memset(
        encodeOutput as *mut core::ffi::c_void,
        0 as core::ffi::c_int,
        ::core::mem::size_of::<avifCodecEncodeOutput>() as size_t,
    );
    if avifArrayCreate(
        &mut (*encodeOutput).samples as *mut avifEncodeSampleArray
            as *mut core::ffi::c_void,
        ::core::mem::size_of::<avifEncodeSample>() as uint32_t,
        1 as uint32_t,
    ) == 0
    {
        avifCodecEncodeOutputDestroy(encodeOutput);
        return 0 as *mut avifCodecEncodeOutput;
    }
    return encodeOutput;
}
#[no_mangle]
pub unsafe extern "C" fn avifCodecEncodeOutputAddSample(
    mut encodeOutput: *mut avifCodecEncodeOutput,
    mut data: *const uint8_t,
    mut len: size_t,
    mut sync: avifBool,
) -> avifResult {
    let mut sample: *mut avifEncodeSample = avifArrayPush(
        &mut (*encodeOutput).samples as *mut avifEncodeSampleArray
            as *mut core::ffi::c_void,
    ) as *mut avifEncodeSample;
    if sample.is_null() {
        return AVIF_RESULT_OUT_OF_MEMORY;
    }
    let result: avifResult = avifRWDataSet(&mut (*sample).data, data, len) as avifResult;
    if result as core::ffi::c_uint
        != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
    {
        avifArrayPop(
            &mut (*encodeOutput).samples as *mut avifEncodeSampleArray
                as *mut core::ffi::c_void,
        );
        return result;
    }
    (*sample).sync = sync;
    return AVIF_RESULT_OK;
}
#[no_mangle]
pub unsafe extern "C" fn avifCodecEncodeOutputDestroy(
    mut encodeOutput: *mut avifCodecEncodeOutput,
) {
    let mut sampleIndex: uint32_t = 0 as uint32_t;
    while sampleIndex < (*encodeOutput).samples.count {
        avifRWDataFree(
            &mut (*((*encodeOutput).samples.sample).offset(sampleIndex as isize)).data,
        );
        sampleIndex = sampleIndex.wrapping_add(1);
    }
    avifArrayDestroy(
        &mut (*encodeOutput).samples as *mut avifEncodeSampleArray
            as *mut core::ffi::c_void,
    );
    avifFree(encodeOutput as *mut core::ffi::c_void);
}
unsafe extern "C" fn avifEncoderDataCreate() -> *mut avifEncoderData {
    let mut data: *mut avifEncoderData = avifAlloc(
        ::core::mem::size_of::<avifEncoderData>() as size_t,
    ) as *mut avifEncoderData;
    if data.is_null() {
        return 0 as *mut avifEncoderData;
    }
    memset(
        data as *mut core::ffi::c_void,
        0 as core::ffi::c_int,
        ::core::mem::size_of::<avifEncoderData>() as size_t,
    );
    (*data).imageMetadata = avifImageCreateEmpty();
    if !((*data).imageMetadata).is_null() {
        (*data).altImageMetadata = avifImageCreateEmpty();
        if !((*data).altImageMetadata).is_null() {
            if !(avifArrayCreate(
                &mut (*data).items as *mut avifEncoderItemArray
                    as *mut core::ffi::c_void,
                ::core::mem::size_of::<avifEncoderItem>() as uint32_t,
                8 as uint32_t,
            ) == 0)
            {
                if !(avifArrayCreate(
                    &mut (*data).frames as *mut avifEncoderFrameArray
                        as *mut core::ffi::c_void,
                    ::core::mem::size_of::<avifEncoderFrame>() as uint32_t,
                    1 as uint32_t,
                ) == 0)
                {
                    if !(avifArrayCreate(
                        &mut (*data).alternativeItemIDs as *mut avifEncoderItemIdArray
                            as *mut core::ffi::c_void,
                        ::core::mem::size_of::<uint32_t>() as uint32_t,
                        1 as uint32_t,
                    ) == 0)
                    {
                        return data;
                    }
                }
            }
        }
    }
    avifEncoderDataDestroy(data);
    return 0 as *mut avifEncoderData;
}
unsafe extern "C" fn avifEncoderDataCreateItem(
    mut data: *mut avifEncoderData,
    mut type_0: *const core::ffi::c_char,
    mut infeName: *const core::ffi::c_char,
    mut infeNameSize: size_t,
    mut cellIndex: uint32_t,
    mut itemPtr: *mut *mut avifEncoderItem,
) -> avifResult {
    if !((*data).lastItemID != 4294967295 as uint32_t) {
        return AVIF_RESULT_INTERNAL_ERROR;
    }
    let mut item: *mut avifEncoderItem = avifArrayPush(
        &mut (*data).items as *mut avifEncoderItemArray as *mut core::ffi::c_void,
    ) as *mut avifEncoderItem;
    if item.is_null() {
        return AVIF_RESULT_OUT_OF_MEMORY;
    }
    (*data).lastItemID = ((*data).lastItemID).wrapping_add(1);
    (*item).id = (*data).lastItemID;
    memcpy(
        ((*item).type_0).as_mut_ptr() as *mut core::ffi::c_void,
        type_0 as *const core::ffi::c_void,
        ::core::mem::size_of::<[uint8_t; 4]>() as size_t,
    );
    (*item).infeName = infeName;
    (*item).infeNameSize = infeNameSize;
    (*item).encodeOutput = avifCodecEncodeOutputCreate();
    if !((*item).encodeOutput).is_null() {
        (*item).cellIndex = cellIndex;
        if !(avifArrayCreate(
            &mut (*item).mdatFixups as *mut avifOffsetFixupArray
                as *mut core::ffi::c_void,
            ::core::mem::size_of::<avifOffsetFixup>() as uint32_t,
            4 as uint32_t,
        ) == 0)
        {
            if !(avifArrayCreate(
                &mut (*item).associations as *mut avifItemPropertyAssociationArray
                    as *mut core::ffi::c_void,
                ::core::mem::size_of::<avifItemPropertyAssociation>() as uint32_t,
                4 as uint32_t,
            ) == 0)
            {
                *itemPtr = item;
                return AVIF_RESULT_OK;
            }
        }
    }
    if !((*item).encodeOutput).is_null() {
        avifCodecEncodeOutputDestroy((*item).encodeOutput);
    }
    avifArrayDestroy(
        &mut (*item).mdatFixups as *mut avifOffsetFixupArray as *mut core::ffi::c_void,
    );
    (*data).lastItemID = ((*data).lastItemID).wrapping_sub(1);
    avifArrayPop(
        &mut (*data).items as *mut avifEncoderItemArray as *mut core::ffi::c_void,
    );
    return AVIF_RESULT_OUT_OF_MEMORY;
}
unsafe extern "C" fn avifEncoderDataFindItemByID(
    mut data: *mut avifEncoderData,
    mut id: uint32_t,
) -> *mut avifEncoderItem {
    let mut itemIndex: uint32_t = 0 as uint32_t;
    while itemIndex < (*data).items.count {
        let mut item: *mut avifEncoderItem = &mut *((*data).items.item)
            .offset(itemIndex as isize) as *mut avifEncoderItem;
        if (*item).id == id {
            return item;
        }
        itemIndex = itemIndex.wrapping_add(1);
    }
    return 0 as *mut avifEncoderItem;
}
unsafe extern "C" fn avifEncoderDataDestroy(mut data: *mut avifEncoderData) {
    let mut i: uint32_t = 0 as uint32_t;
    while i < (*data).items.count {
        let mut item: *mut avifEncoderItem = &mut *((*data).items.item)
            .offset(i as isize) as *mut avifEncoderItem;
        if !((*item).codec).is_null() {
            avifCodecDestroy((*item).codec);
        }
        avifCodecEncodeOutputDestroy((*item).encodeOutput);
        avifRWDataFree(&mut (*item).metadataPayload);
        avifArrayDestroy(
            &mut (*item).mdatFixups as *mut avifOffsetFixupArray
                as *mut core::ffi::c_void,
        );
        avifArrayDestroy(
            &mut (*item).associations as *mut avifItemPropertyAssociationArray
                as *mut core::ffi::c_void,
        );
        i = i.wrapping_add(1);
    }
    if !((*data).imageMetadata).is_null() {
        avifImageDestroy((*data).imageMetadata);
    }
    if !((*data).altImageMetadata).is_null() {
        avifImageDestroy((*data).altImageMetadata);
    }
    avifArrayDestroy(
        &mut (*data).items as *mut avifEncoderItemArray as *mut core::ffi::c_void,
    );
    avifArrayDestroy(
        &mut (*data).frames as *mut avifEncoderFrameArray as *mut core::ffi::c_void,
    );
    avifArrayDestroy(
        &mut (*data).alternativeItemIDs as *mut avifEncoderItemIdArray
            as *mut core::ffi::c_void,
    );
    avifFree(data as *mut core::ffi::c_void);
}
unsafe extern "C" fn avifEncoderItemAddMdatFixup(
    mut item: *mut avifEncoderItem,
    mut s: *const avifRWStream,
) -> avifResult {
    let mut fixup: *mut avifOffsetFixup = avifArrayPush(
        &mut (*item).mdatFixups as *mut avifOffsetFixupArray as *mut core::ffi::c_void,
    ) as *mut avifOffsetFixup;
    if fixup.is_null() {
        return AVIF_RESULT_OUT_OF_MEMORY;
    }
    (*fixup).offset = avifRWStreamOffset(s);
    return AVIF_RESULT_OK;
}
unsafe extern "C" fn avifItemPropertyDedupCreate() -> *mut avifItemPropertyDedup {
    let mut dedup: *mut avifItemPropertyDedup = avifAlloc(
        ::core::mem::size_of::<avifItemPropertyDedup>() as size_t,
    ) as *mut avifItemPropertyDedup;
    if dedup.is_null() {
        return 0 as *mut avifItemPropertyDedup;
    }
    memset(
        dedup as *mut core::ffi::c_void,
        0 as core::ffi::c_int,
        ::core::mem::size_of::<avifItemPropertyDedup>() as size_t,
    );
    if avifArrayCreate(
        &mut (*dedup).properties as *mut avifItemPropertyArray as *mut core::ffi::c_void,
        ::core::mem::size_of::<avifItemProperty>() as uint32_t,
        8 as uint32_t,
    ) == 0
    {
        avifFree(dedup as *mut core::ffi::c_void);
        return 0 as *mut avifItemPropertyDedup;
    }
    if avifRWDataRealloc(&mut (*dedup).buffer, 2048 as size_t) as core::ffi::c_uint
        != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
    {
        avifArrayDestroy(
            &mut (*dedup).properties as *mut avifItemPropertyArray
                as *mut core::ffi::c_void,
        );
        avifFree(dedup as *mut core::ffi::c_void);
        return 0 as *mut avifItemPropertyDedup;
    }
    return dedup;
}
unsafe extern "C" fn avifItemPropertyDedupDestroy(
    mut dedup: *mut avifItemPropertyDedup,
) {
    avifArrayDestroy(
        &mut (*dedup).properties as *mut avifItemPropertyArray as *mut core::ffi::c_void,
    );
    avifRWDataFree(&mut (*dedup).buffer);
    avifFree(dedup as *mut core::ffi::c_void);
}
unsafe extern "C" fn avifItemPropertyDedupStart(mut dedup: *mut avifItemPropertyDedup) {
    avifRWStreamStart(&mut (*dedup).s, &mut (*dedup).buffer);
}
unsafe extern "C" fn avifItemPropertyDedupFinish(
    mut dedup: *mut avifItemPropertyDedup,
    mut outputStream: *mut avifRWStream,
    mut associations: *mut avifItemPropertyAssociationArray,
    mut essential: avifBool,
) -> avifResult {
    let mut propertyIndex: uint8_t = 0 as uint8_t;
    let newPropertySize: size_t = avifRWStreamOffset(&mut (*dedup).s) as size_t;
    let mut i: size_t = 0 as size_t;
    while i < (*dedup).properties.count as size_t {
        let mut property: *mut avifItemProperty = &mut *((*dedup).properties.property)
            .offset(i as isize) as *mut avifItemProperty;
        if (*property).size == newPropertySize
            && memcmp(
                &mut *((*(*outputStream).raw).data).offset((*property).offset as isize)
                    as *mut uint8_t as *const core::ffi::c_void,
                (*dedup).buffer.data as *const core::ffi::c_void,
                newPropertySize,
            ) == 0
        {
            propertyIndex = (*property).index;
            if !(propertyIndex as core::ffi::c_int != 0 as core::ffi::c_int) {
                return AVIF_RESULT_INTERNAL_ERROR;
            }
            break;
        } else {
            i = i.wrapping_add(1);
        }
    }
    if propertyIndex as core::ffi::c_int == 0 as core::ffi::c_int {
        let mut property_0: *mut avifItemProperty = avifArrayPush(
            &mut (*dedup).properties as *mut avifItemPropertyArray
                as *mut core::ffi::c_void,
        ) as *mut avifItemProperty;
        if property_0.is_null() {
            return AVIF_RESULT_OUT_OF_MEMORY;
        }
        if !(((*dedup).nextIndex as core::ffi::c_int)
            < ((1 as core::ffi::c_int) << 7 as core::ffi::c_int) - 1 as core::ffi::c_int)
        {
            return AVIF_RESULT_INVALID_ARGUMENT;
        }
        (*dedup).nextIndex = ((*dedup).nextIndex).wrapping_add(1);
        (*property_0).index = (*dedup).nextIndex;
        (*property_0).size = newPropertySize;
        (*property_0).offset = avifRWStreamOffset(outputStream);
        let result__: avifResult = avifRWStreamWrite(
            outputStream,
            (*dedup).buffer.data as *const core::ffi::c_void,
            newPropertySize,
        ) as avifResult;
        if result__ as core::ffi::c_uint
            != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
        {
            return result__;
        }
        propertyIndex = (*property_0).index;
    }
    let mut association: *mut avifItemPropertyAssociation = avifArrayPush(
        associations as *mut core::ffi::c_void,
    ) as *mut avifItemPropertyAssociation;
    if association.is_null() {
        return AVIF_RESULT_OUT_OF_MEMORY;
    }
    (*association).property_index = propertyIndex;
    (*association).essential = essential;
    return AVIF_RESULT_OK;
}
static mut noScaling: avifScalingMode = {
    let mut init = avifScalingMode {
        horizontal: {
            let mut init = avifFraction {
                n: 1 as int32_t,
                d: 1 as int32_t,
            };
            init
        },
        vertical: {
            let mut init = avifFraction {
                n: 1 as int32_t,
                d: 1 as int32_t,
            };
            init
        },
    };
    init
};
unsafe extern "C" fn avifEncoderSizeIsSet(mut encoder: *const avifEncoder) -> avifBool {
    return ((*encoder).width != 0 as uint32_t || (*encoder).height != 0 as uint32_t)
        as core::ffi::c_int;
}
unsafe extern "C" fn avifScalingModeIsNoScaling(
    mut scalingMode: *const avifScalingMode,
) -> avifBool {
    return ((*scalingMode).horizontal.n == (*scalingMode).horizontal.d
        && (*scalingMode).vertical.n == (*scalingMode).vertical.d) as core::ffi::c_int;
}
unsafe extern "C" fn avifImageHasEquivalentTransformProperties(
    mut lhs: *const avifImage,
    mut rhs: *const avifImage,
) -> avifBool {
    let lhsTransformFlags: uint32_t = (*lhs).transformFlags as uint32_t
        & (AVIF_TRANSFORM_PASP as core::ffi::c_int
            | AVIF_TRANSFORM_CLAP as core::ffi::c_int
            | AVIF_TRANSFORM_IROT as core::ffi::c_int
            | AVIF_TRANSFORM_IMIR as core::ffi::c_int) as uint32_t;
    let rhsTransformFlags: uint32_t = (*rhs).transformFlags as uint32_t
        & (AVIF_TRANSFORM_PASP as core::ffi::c_int
            | AVIF_TRANSFORM_CLAP as core::ffi::c_int
            | AVIF_TRANSFORM_IROT as core::ffi::c_int
            | AVIF_TRANSFORM_IMIR as core::ffi::c_int) as uint32_t;
    if lhsTransformFlags != rhsTransformFlags {
        return AVIF_FALSE;
    }
    if lhsTransformFlags & AVIF_TRANSFORM_PASP as core::ffi::c_int as uint32_t != 0
        && ((*lhs).pasp.hSpacing != (*rhs).pasp.hSpacing
            || (*lhs).pasp.vSpacing != (*rhs).pasp.vSpacing)
    {
        return AVIF_FALSE;
    }
    if lhsTransformFlags & AVIF_TRANSFORM_CLAP as core::ffi::c_int as uint32_t != 0
        && ((*lhs).clap.widthN != (*rhs).clap.widthN
            || (*lhs).clap.widthD != (*rhs).clap.widthD
            || (*lhs).clap.heightN != (*rhs).clap.heightN
            || (*lhs).clap.heightD != (*rhs).clap.heightD
            || (*lhs).clap.horizOffN != (*rhs).clap.horizOffN
            || (*lhs).clap.horizOffD != (*rhs).clap.horizOffD
            || (*lhs).clap.vertOffN != (*rhs).clap.vertOffN
            || (*lhs).clap.vertOffD != (*rhs).clap.vertOffD)
    {
        return AVIF_FALSE;
    }
    if lhsTransformFlags & AVIF_TRANSFORM_IROT as core::ffi::c_int as uint32_t != 0
        && (*lhs).irot.angle as core::ffi::c_int != (*rhs).irot.angle as core::ffi::c_int
    {
        return AVIF_FALSE;
    }
    if lhsTransformFlags & AVIF_TRANSFORM_IMIR as core::ffi::c_int as uint32_t != 0
        && (*lhs).imir.axis as core::ffi::c_int != (*rhs).imir.axis as core::ffi::c_int
    {
        return AVIF_FALSE;
    }
    return AVIF_TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn avifEncoderCreate() -> *mut avifEncoder {
    let mut encoder: *mut avifEncoder = avifAlloc(
        ::core::mem::size_of::<avifEncoder>() as size_t,
    ) as *mut avifEncoder;
    if encoder.is_null() {
        return 0 as *mut avifEncoder;
    }
    memset(
        encoder as *mut core::ffi::c_void,
        0 as core::ffi::c_int,
        ::core::mem::size_of::<avifEncoder>() as size_t,
    );
    (*encoder).codecChoice = AVIF_CODEC_CHOICE_AUTO;
    (*encoder).maxThreads = 1 as core::ffi::c_int;
    (*encoder).speed = AVIF_SPEED_DEFAULT;
    (*encoder).keyframeInterval = 0 as core::ffi::c_int;
    (*encoder).timescale = 1 as uint64_t;
    (*encoder).repetitionCount = AVIF_REPETITION_COUNT_INFINITE;
    (*encoder).quality = AVIF_QUALITY_DEFAULT;
    (*encoder).qualityAlpha = AVIF_QUALITY_DEFAULT;
    (*encoder).qualityGainMap = AVIF_QUALITY_DEFAULT;
    (*encoder).minQuantizer = AVIF_QUANTIZER_BEST_QUALITY;
    (*encoder).maxQuantizer = AVIF_QUANTIZER_WORST_QUALITY;
    (*encoder).minQuantizerAlpha = AVIF_QUANTIZER_BEST_QUALITY;
    (*encoder).maxQuantizerAlpha = AVIF_QUANTIZER_WORST_QUALITY;
    (*encoder).tileRowsLog2 = 0 as core::ffi::c_int;
    (*encoder).tileColsLog2 = 0 as core::ffi::c_int;
    (*encoder).autoTiling = AVIF_FALSE as avifBool;
    (*encoder).scalingMode = noScaling;
    (*encoder).data = avifEncoderDataCreate() as *mut avifEncoderData;
    (*encoder).csOptions = avifCodecSpecificOptionsCreate()
        as *mut avifCodecSpecificOptions;
    if ((*encoder).data).is_null() || ((*encoder).csOptions).is_null() {
        avifEncoderDestroy(encoder);
        return 0 as *mut avifEncoder;
    }
    (*encoder).headerFormat = AVIF_HEADER_DEFAULT as core::ffi::c_int
        as avifHeaderFormatFlags;
    (*encoder).creationTime = 0 as uint64_t;
    (*encoder).modificationTime = 0 as uint64_t;
    (*encoder).sampleTransformRecipe = AVIF_SAMPLE_TRANSFORM_NONE;
    (*encoder).width = 0 as uint32_t;
    (*encoder).height = 0 as uint32_t;
    return encoder;
}
#[no_mangle]
pub unsafe extern "C" fn avifEncoderDestroy(mut encoder: *mut avifEncoder) {
    if !((*encoder).csOptions).is_null() {
        avifCodecSpecificOptionsDestroy(
            (*encoder).csOptions as *mut avifCodecSpecificOptions,
        );
    }
    if !((*encoder).data).is_null() {
        avifEncoderDataDestroy((*encoder).data as *mut avifEncoderData);
    }
    avifFree(encoder as *mut core::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn avifEncoderSetCodecSpecificOption(
    mut encoder: *mut avifEncoder,
    mut key: *const core::ffi::c_char,
    mut value: *const core::ffi::c_char,
) -> avifResult {
    return avifCodecSpecificOptionsSet(
        (*encoder).csOptions as *mut avifCodecSpecificOptions,
        key,
        value,
    );
}
unsafe extern "C" fn avifEncoderBackupSettings(mut encoder: *mut avifEncoder) {
    let mut lastEncoder: *mut avifEncoder = &mut (*(*encoder).data).lastEncoder;
    (*lastEncoder).data = (*encoder).data;
    (*lastEncoder).codecChoice = (*encoder).codecChoice;
    (*lastEncoder).maxThreads = (*encoder).maxThreads;
    (*lastEncoder).speed = (*encoder).speed;
    (*lastEncoder).keyframeInterval = (*encoder).keyframeInterval;
    (*lastEncoder).timescale = (*encoder).timescale;
    (*lastEncoder).repetitionCount = (*encoder).repetitionCount;
    (*lastEncoder).extraLayerCount = (*encoder).extraLayerCount;
    (*lastEncoder).minQuantizer = (*encoder).minQuantizer;
    (*lastEncoder).maxQuantizer = (*encoder).maxQuantizer;
    (*lastEncoder).minQuantizerAlpha = (*encoder).minQuantizerAlpha;
    (*lastEncoder).maxQuantizerAlpha = (*encoder).maxQuantizerAlpha;
    (*(*encoder).data).lastQuality = (*(*encoder).data).quality;
    (*(*encoder).data).lastQualityAlpha = (*(*encoder).data).qualityAlpha;
    (*(*encoder).data).lastTileRowsLog2 = (*(*encoder).data).tileRowsLog2;
    (*(*encoder).data).lastTileColsLog2 = (*(*encoder).data).tileColsLog2;
    (*lastEncoder).scalingMode = (*encoder).scalingMode;
    (*lastEncoder).sampleTransformRecipe = (*encoder).sampleTransformRecipe;
    (*lastEncoder).width = (*encoder).width;
    (*lastEncoder).height = (*encoder).height;
}
unsafe extern "C" fn avifEncoderDetectChanges(
    mut encoder: *const avifEncoder,
    mut encoderChanges: *mut avifEncoderChanges,
) -> avifBool {
    let mut lastEncoder: *const avifEncoder = &mut (*(*encoder).data).lastEncoder;
    *encoderChanges = 0 as core::ffi::c_int as avifEncoderChanges;
    if ((*lastEncoder).data).is_null() {
        return AVIF_TRUE;
    }
    if (*lastEncoder).codecChoice as core::ffi::c_uint
        != (*encoder).codecChoice as core::ffi::c_uint
        || (*lastEncoder).maxThreads != (*encoder).maxThreads
        || (*lastEncoder).speed != (*encoder).speed
        || (*lastEncoder).keyframeInterval != (*encoder).keyframeInterval
        || (*lastEncoder).timescale != (*encoder).timescale
        || (*lastEncoder).repetitionCount != (*encoder).repetitionCount
        || (*lastEncoder).extraLayerCount != (*encoder).extraLayerCount
        || (*lastEncoder).width != (*encoder).width
        || (*lastEncoder).height != (*encoder).height
    {
        return AVIF_FALSE;
    }
    if (*(*encoder).data).lastQuality != (*(*encoder).data).quality {
        *encoderChanges |= AVIF_ENCODER_CHANGE_QUALITY as core::ffi::c_int;
    }
    if (*(*encoder).data).lastQualityAlpha != (*(*encoder).data).qualityAlpha {
        *encoderChanges |= AVIF_ENCODER_CHANGE_QUALITY_ALPHA as core::ffi::c_int;
    }
    if (*lastEncoder).minQuantizer != (*encoder).minQuantizer {
        *encoderChanges |= AVIF_ENCODER_CHANGE_MIN_QUANTIZER as core::ffi::c_int;
    }
    if (*lastEncoder).maxQuantizer != (*encoder).maxQuantizer {
        *encoderChanges |= AVIF_ENCODER_CHANGE_MAX_QUANTIZER as core::ffi::c_int;
    }
    if (*lastEncoder).minQuantizerAlpha != (*encoder).minQuantizerAlpha {
        *encoderChanges |= AVIF_ENCODER_CHANGE_MIN_QUANTIZER_ALPHA as core::ffi::c_int;
    }
    if (*lastEncoder).maxQuantizerAlpha != (*encoder).maxQuantizerAlpha {
        *encoderChanges |= AVIF_ENCODER_CHANGE_MAX_QUANTIZER_ALPHA as core::ffi::c_int;
    }
    if (*(*encoder).data).lastTileRowsLog2 != (*(*encoder).data).tileRowsLog2 {
        *encoderChanges |= AVIF_ENCODER_CHANGE_TILE_ROWS_LOG2 as core::ffi::c_int;
    }
    if (*(*encoder).data).lastTileColsLog2 != (*(*encoder).data).tileColsLog2 {
        *encoderChanges |= AVIF_ENCODER_CHANGE_TILE_COLS_LOG2 as core::ffi::c_int;
    }
    if memcmp(
        &(*lastEncoder).scalingMode as *const avifScalingMode
            as *const core::ffi::c_void,
        &(*encoder).scalingMode as *const avifScalingMode as *const core::ffi::c_void,
        ::core::mem::size_of::<avifScalingMode>() as size_t,
    ) != 0 as core::ffi::c_int
    {
        *encoderChanges |= AVIF_ENCODER_CHANGE_SCALING_MODE as core::ffi::c_int;
    }
    if (*(*encoder).csOptions).count > 0 as uint32_t {
        *encoderChanges |= AVIF_ENCODER_CHANGE_CODEC_SPECIFIC as core::ffi::c_int;
    }
    if (*lastEncoder).sampleTransformRecipe as core::ffi::c_uint
        != (*encoder).sampleTransformRecipe as core::ffi::c_uint
    {
        return AVIF_FALSE;
    }
    return AVIF_TRUE;
}
unsafe extern "C" fn avifEncoderWriteNclxProperty(
    mut dedupStream: *mut avifRWStream,
    mut outputStream: *mut avifRWStream,
    mut imageMetadata: *const avifImage,
    mut associations: *mut avifItemPropertyAssociationArray,
    mut dedup: *mut avifItemPropertyDedup,
) -> avifResult {
    if !dedup.is_null() {
        avifItemPropertyDedupStart(dedup);
    }
    let mut colr: avifBoxMarker = 0;
    let result__: avifResult = avifRWStreamWriteBox(
        dedupStream,
        b"colr\0" as *const u8 as *const core::ffi::c_char,
        0 as size_t,
        &mut colr,
    ) as avifResult;
    if result__ as core::ffi::c_uint
        != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
    {
        return result__;
    }
    let result___0: avifResult = avifRWStreamWriteChars(
        dedupStream,
        b"nclx\0" as *const u8 as *const core::ffi::c_char,
        4 as size_t,
    ) as avifResult;
    if result___0 as core::ffi::c_uint
        != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
    {
        return result___0;
    }
    let result___1: avifResult = avifRWStreamWriteU16(
        dedupStream,
        (*imageMetadata).colorPrimaries as uint16_t,
    ) as avifResult;
    if result___1 as core::ffi::c_uint
        != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
    {
        return result___1;
    }
    let result___2: avifResult = avifRWStreamWriteU16(
        dedupStream,
        (*imageMetadata).transferCharacteristics as uint16_t,
    ) as avifResult;
    if result___2 as core::ffi::c_uint
        != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
    {
        return result___2;
    }
    let result___3: avifResult = avifRWStreamWriteU16(
        dedupStream,
        (*imageMetadata).matrixCoefficients as uint16_t,
    ) as avifResult;
    if result___3 as core::ffi::c_uint
        != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
    {
        return result___3;
    }
    let result___4: avifResult = avifRWStreamWriteBits(
        dedupStream,
        (if (*imageMetadata).yuvRange as core::ffi::c_uint
            == AVIF_RANGE_FULL as core::ffi::c_int as core::ffi::c_uint
        {
            1 as core::ffi::c_int
        } else {
            0 as core::ffi::c_int
        }) as uint32_t,
        1 as size_t,
    ) as avifResult;
    if result___4 as core::ffi::c_uint
        != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
    {
        return result___4;
    }
    let result___5: avifResult = avifRWStreamWriteBits(
        dedupStream,
        0 as uint32_t,
        7 as size_t,
    ) as avifResult;
    if result___5 as core::ffi::c_uint
        != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
    {
        return result___5;
    }
    let result___6: avifResult = avifRWStreamFinishBox(dedupStream, colr) as avifResult;
    if result___6 as core::ffi::c_uint
        != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
    {
        return result___6;
    }
    if !dedup.is_null() {
        let result___7: avifResult = avifItemPropertyDedupFinish(
            dedup,
            outputStream,
            associations,
            0 as avifBool,
        ) as avifResult;
        if result___7 as core::ffi::c_uint
            != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
        {
            return result___7;
        }
    }
    return AVIF_RESULT_OK;
}
unsafe extern "C" fn avifEncoderWriteColorProperties(
    mut outputStream: *mut avifRWStream,
    mut imageMetadata: *const avifImage,
    mut associations: *mut avifItemPropertyAssociationArray,
    mut dedup: *mut avifItemPropertyDedup,
) -> avifResult {
    let mut dedupStream: *mut avifRWStream = outputStream;
    if !dedup.is_null() {
        if associations.is_null() {
            return AVIF_RESULT_INTERNAL_ERROR;
        }
        dedupStream = &mut (*dedup).s;
    }
    if (*imageMetadata).icc.size > 0 as size_t {
        if !dedup.is_null() {
            avifItemPropertyDedupStart(dedup);
        }
        let mut colr: avifBoxMarker = 0;
        let result__: avifResult = avifRWStreamWriteBox(
            dedupStream,
            b"colr\0" as *const u8 as *const core::ffi::c_char,
            0 as size_t,
            &mut colr,
        ) as avifResult;
        if result__ as core::ffi::c_uint
            != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
        {
            return result__;
        }
        let result___0: avifResult = avifRWStreamWriteChars(
            dedupStream,
            b"prof\0" as *const u8 as *const core::ffi::c_char,
            4 as size_t,
        ) as avifResult;
        if result___0 as core::ffi::c_uint
            != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
        {
            return result___0;
        }
        let result___1: avifResult = avifRWStreamWrite(
            dedupStream,
            (*imageMetadata).icc.data as *const core::ffi::c_void,
            (*imageMetadata).icc.size,
        ) as avifResult;
        if result___1 as core::ffi::c_uint
            != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
        {
            return result___1;
        }
        let result___2: avifResult = avifRWStreamFinishBox(dedupStream, colr)
            as avifResult;
        if result___2 as core::ffi::c_uint
            != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
        {
            return result___2;
        }
        if !dedup.is_null() {
            let result___3: avifResult = avifItemPropertyDedupFinish(
                dedup,
                outputStream,
                associations,
                0 as avifBool,
            ) as avifResult;
            if result___3 as core::ffi::c_uint
                != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
            {
                return result___3;
            }
        }
    }
    let result___4: avifResult = avifEncoderWriteNclxProperty(
        dedupStream,
        outputStream,
        imageMetadata,
        associations,
        dedup,
    ) as avifResult;
    if result___4 as core::ffi::c_uint
        != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
    {
        return result___4;
    }
    let result___5: avifResult = avifEncoderWritePaspProperty(
        dedupStream,
        outputStream,
        imageMetadata,
        associations,
        dedup,
    ) as avifResult;
    if result___5 as core::ffi::c_uint
        != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
    {
        return result___5;
    }
    return AVIF_RESULT_OK;
}
unsafe extern "C" fn avifEncoderWriteContentLightLevelInformation(
    mut outputStream: *mut avifRWStream,
    mut clli: *const avifContentLightLevelInformationBox,
) -> avifResult {
    let result__: avifResult = avifRWStreamWriteBits(
        outputStream,
        (*clli).maxCLL as uint32_t,
        16 as size_t,
    ) as avifResult;
    if result__ as core::ffi::c_uint
        != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
    {
        return result__;
    }
    let result___0: avifResult = avifRWStreamWriteBits(
        outputStream,
        (*clli).maxPALL as uint32_t,
        16 as size_t,
    ) as avifResult;
    if result___0 as core::ffi::c_uint
        != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
    {
        return result___0;
    }
    return AVIF_RESULT_OK;
}
unsafe extern "C" fn avifEncoderWriteHDRProperties(
    mut dedupStream: *mut avifRWStream,
    mut outputStream: *mut avifRWStream,
    mut imageMetadata: *const avifImage,
    mut associations: *mut avifItemPropertyAssociationArray,
    mut dedup: *mut avifItemPropertyDedup,
) -> avifResult {
    if (*imageMetadata).clli.maxCLL as core::ffi::c_int != 0
        || (*imageMetadata).clli.maxPALL as core::ffi::c_int != 0
    {
        if !dedup.is_null() {
            avifItemPropertyDedupStart(dedup);
        }
        let mut clli: avifBoxMarker = 0;
        let result__: avifResult = avifRWStreamWriteBox(
            dedupStream,
            b"clli\0" as *const u8 as *const core::ffi::c_char,
            0 as size_t,
            &mut clli,
        ) as avifResult;
        if result__ as core::ffi::c_uint
            != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
        {
            return result__;
        }
        let result___0: avifResult = avifEncoderWriteContentLightLevelInformation(
            dedupStream,
            &(*imageMetadata).clli,
        ) as avifResult;
        if result___0 as core::ffi::c_uint
            != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
        {
            return result___0;
        }
        let result___1: avifResult = avifRWStreamFinishBox(dedupStream, clli)
            as avifResult;
        if result___1 as core::ffi::c_uint
            != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
        {
            return result___1;
        }
        if !dedup.is_null() {
            let result___2: avifResult = avifItemPropertyDedupFinish(
                dedup,
                outputStream,
                associations,
                0 as avifBool,
            ) as avifResult;
            if result___2 as core::ffi::c_uint
                != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
            {
                return result___2;
            }
        }
    }
    return AVIF_RESULT_OK;
}
unsafe extern "C" fn avifEncoderWriteMiniHDRProperties(
    mut outputStream: *mut avifRWStream,
    mut imageMetadata: *const avifImage,
) -> avifResult {
    let hasClli: avifBool = ((*imageMetadata).clli.maxCLL as core::ffi::c_int
        != 0 as core::ffi::c_int
        || (*imageMetadata).clli.maxPALL as core::ffi::c_int != 0 as core::ffi::c_int)
        as core::ffi::c_int;
    let hasMdcv: avifBool = AVIF_FALSE;
    let hasCclv: avifBool = AVIF_FALSE;
    let hasAmve: avifBool = AVIF_FALSE;
    let hasReve: avifBool = AVIF_FALSE;
    let hasNdwt: avifBool = AVIF_FALSE;
    let result__: avifResult = avifRWStreamWriteBits(
        outputStream,
        hasClli as uint32_t,
        1 as size_t,
    ) as avifResult;
    if result__ as core::ffi::c_uint
        != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
    {
        return result__;
    }
    let result___0: avifResult = avifRWStreamWriteBits(
        outputStream,
        hasMdcv as uint32_t,
        1 as size_t,
    ) as avifResult;
    if result___0 as core::ffi::c_uint
        != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
    {
        return result___0;
    }
    let result___1: avifResult = avifRWStreamWriteBits(
        outputStream,
        hasCclv as uint32_t,
        1 as size_t,
    ) as avifResult;
    if result___1 as core::ffi::c_uint
        != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
    {
        return result___1;
    }
    let result___2: avifResult = avifRWStreamWriteBits(
        outputStream,
        hasAmve as uint32_t,
        1 as size_t,
    ) as avifResult;
    if result___2 as core::ffi::c_uint
        != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
    {
        return result___2;
    }
    let result___3: avifResult = avifRWStreamWriteBits(
        outputStream,
        hasReve as uint32_t,
        1 as size_t,
    ) as avifResult;
    if result___3 as core::ffi::c_uint
        != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
    {
        return result___3;
    }
    let result___4: avifResult = avifRWStreamWriteBits(
        outputStream,
        hasNdwt as uint32_t,
        1 as size_t,
    ) as avifResult;
    if result___4 as core::ffi::c_uint
        != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
    {
        return result___4;
    }
    if hasClli != 0 {
        let result___5: avifResult = avifEncoderWriteContentLightLevelInformation(
            outputStream,
            &(*imageMetadata).clli,
        ) as avifResult;
        if result___5 as core::ffi::c_uint
            != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
        {
            return result___5;
        }
    }
    hasMdcv != 0;
    hasCclv != 0;
    hasAmve != 0;
    hasReve != 0;
    hasNdwt != 0;
    return AVIF_RESULT_OK;
}
unsafe extern "C" fn avifEncoderWritePaspProperty(
    mut dedupStream: *mut avifRWStream,
    mut outputStream: *mut avifRWStream,
    mut imageMetadata: *const avifImage,
    mut associations: *mut avifItemPropertyAssociationArray,
    mut dedup: *mut avifItemPropertyDedup,
) -> avifResult {
    if (*imageMetadata).transformFlags
        & AVIF_TRANSFORM_PASP as core::ffi::c_int as avifTransformFlags != 0
    {
        if !dedup.is_null() {
            avifItemPropertyDedupStart(dedup);
        }
        let mut pasp: avifBoxMarker = 0;
        let result__: avifResult = avifRWStreamWriteBox(
            dedupStream,
            b"pasp\0" as *const u8 as *const core::ffi::c_char,
            0 as size_t,
            &mut pasp,
        ) as avifResult;
        if result__ as core::ffi::c_uint
            != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
        {
            return result__;
        }
        let result___0: avifResult = avifRWStreamWriteU32(
            dedupStream,
            (*imageMetadata).pasp.hSpacing,
        ) as avifResult;
        if result___0 as core::ffi::c_uint
            != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
        {
            return result___0;
        }
        let result___1: avifResult = avifRWStreamWriteU32(
            dedupStream,
            (*imageMetadata).pasp.vSpacing,
        ) as avifResult;
        if result___1 as core::ffi::c_uint
            != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
        {
            return result___1;
        }
        let result___2: avifResult = avifRWStreamFinishBox(dedupStream, pasp)
            as avifResult;
        if result___2 as core::ffi::c_uint
            != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
        {
            return result___2;
        }
        if !dedup.is_null() {
            let result___3: avifResult = avifItemPropertyDedupFinish(
                dedup,
                outputStream,
                associations,
                0 as avifBool,
            ) as avifResult;
            if result___3 as core::ffi::c_uint
                != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
            {
                return result___3;
            }
        }
    }
    return AVIF_RESULT_OK;
}
unsafe extern "C" fn avifEncoderWriteTransformativeProperties(
    mut dedupStream: *mut avifRWStream,
    mut outputStream: *mut avifRWStream,
    mut imageMetadata: *const avifImage,
    mut associations: *mut avifItemPropertyAssociationArray,
    mut dedup: *mut avifItemPropertyDedup,
) -> avifResult {
    if (*imageMetadata).transformFlags
        & AVIF_TRANSFORM_CLAP as core::ffi::c_int as avifTransformFlags != 0
    {
        if !dedup.is_null() {
            avifItemPropertyDedupStart(dedup);
        }
        let mut clap: avifBoxMarker = 0;
        let result__: avifResult = avifRWStreamWriteBox(
            dedupStream,
            b"clap\0" as *const u8 as *const core::ffi::c_char,
            0 as size_t,
            &mut clap,
        ) as avifResult;
        if result__ as core::ffi::c_uint
            != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
        {
            return result__;
        }
        let result___0: avifResult = avifRWStreamWriteU32(
            dedupStream,
            (*imageMetadata).clap.widthN,
        ) as avifResult;
        if result___0 as core::ffi::c_uint
            != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
        {
            return result___0;
        }
        let result___1: avifResult = avifRWStreamWriteU32(
            dedupStream,
            (*imageMetadata).clap.widthD,
        ) as avifResult;
        if result___1 as core::ffi::c_uint
            != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
        {
            return result___1;
        }
        let result___2: avifResult = avifRWStreamWriteU32(
            dedupStream,
            (*imageMetadata).clap.heightN,
        ) as avifResult;
        if result___2 as core::ffi::c_uint
            != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
        {
            return result___2;
        }
        let result___3: avifResult = avifRWStreamWriteU32(
            dedupStream,
            (*imageMetadata).clap.heightD,
        ) as avifResult;
        if result___3 as core::ffi::c_uint
            != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
        {
            return result___3;
        }
        let result___4: avifResult = avifRWStreamWriteU32(
            dedupStream,
            (*imageMetadata).clap.horizOffN,
        ) as avifResult;
        if result___4 as core::ffi::c_uint
            != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
        {
            return result___4;
        }
        let result___5: avifResult = avifRWStreamWriteU32(
            dedupStream,
            (*imageMetadata).clap.horizOffD,
        ) as avifResult;
        if result___5 as core::ffi::c_uint
            != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
        {
            return result___5;
        }
        let result___6: avifResult = avifRWStreamWriteU32(
            dedupStream,
            (*imageMetadata).clap.vertOffN,
        ) as avifResult;
        if result___6 as core::ffi::c_uint
            != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
        {
            return result___6;
        }
        let result___7: avifResult = avifRWStreamWriteU32(
            dedupStream,
            (*imageMetadata).clap.vertOffD,
        ) as avifResult;
        if result___7 as core::ffi::c_uint
            != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
        {
            return result___7;
        }
        let result___8: avifResult = avifRWStreamFinishBox(dedupStream, clap)
            as avifResult;
        if result___8 as core::ffi::c_uint
            != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
        {
            return result___8;
        }
        if !dedup.is_null() {
            let result___9: avifResult = avifItemPropertyDedupFinish(
                dedup,
                outputStream,
                associations,
                1 as avifBool,
            ) as avifResult;
            if result___9 as core::ffi::c_uint
                != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
            {
                return result___9;
            }
        }
    }
    if (*imageMetadata).transformFlags
        & AVIF_TRANSFORM_IROT as core::ffi::c_int as avifTransformFlags != 0
    {
        if !dedup.is_null() {
            avifItemPropertyDedupStart(dedup);
        }
        let mut irot: avifBoxMarker = 0;
        let result___10: avifResult = avifRWStreamWriteBox(
            dedupStream,
            b"irot\0" as *const u8 as *const core::ffi::c_char,
            0 as size_t,
            &mut irot,
        ) as avifResult;
        if result___10 as core::ffi::c_uint
            != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
        {
            return result___10;
        }
        let result___11: avifResult = avifRWStreamWriteBits(
            dedupStream,
            0 as uint32_t,
            6 as size_t,
        ) as avifResult;
        if result___11 as core::ffi::c_uint
            != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
        {
            return result___11;
        }
        let result___12: avifResult = avifRWStreamWriteBits(
            dedupStream,
            ((*imageMetadata).irot.angle as core::ffi::c_int & 0x3 as core::ffi::c_int)
                as uint32_t,
            2 as size_t,
        ) as avifResult;
        if result___12 as core::ffi::c_uint
            != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
        {
            return result___12;
        }
        let result___13: avifResult = avifRWStreamFinishBox(dedupStream, irot)
            as avifResult;
        if result___13 as core::ffi::c_uint
            != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
        {
            return result___13;
        }
        if !dedup.is_null() {
            let result___14: avifResult = avifItemPropertyDedupFinish(
                dedup,
                outputStream,
                associations,
                1 as avifBool,
            ) as avifResult;
            if result___14 as core::ffi::c_uint
                != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
            {
                return result___14;
            }
        }
    }
    if (*imageMetadata).transformFlags
        & AVIF_TRANSFORM_IMIR as core::ffi::c_int as avifTransformFlags != 0
    {
        if !dedup.is_null() {
            avifItemPropertyDedupStart(dedup);
        }
        let mut imir: avifBoxMarker = 0;
        let result___15: avifResult = avifRWStreamWriteBox(
            dedupStream,
            b"imir\0" as *const u8 as *const core::ffi::c_char,
            0 as size_t,
            &mut imir,
        ) as avifResult;
        if result___15 as core::ffi::c_uint
            != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
        {
            return result___15;
        }
        let result___16: avifResult = avifRWStreamWriteBits(
            dedupStream,
            0 as uint32_t,
            7 as size_t,
        ) as avifResult;
        if result___16 as core::ffi::c_uint
            != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
        {
            return result___16;
        }
        let result___17: avifResult = avifRWStreamWriteBits(
            dedupStream,
            (if (*imageMetadata).imir.axis as core::ffi::c_int != 0 {
                1 as core::ffi::c_int
            } else {
                0 as core::ffi::c_int
            }) as uint32_t,
            1 as size_t,
        ) as avifResult;
        if result___17 as core::ffi::c_uint
            != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
        {
            return result___17;
        }
        let result___18: avifResult = avifRWStreamFinishBox(dedupStream, imir)
            as avifResult;
        if result___18 as core::ffi::c_uint
            != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
        {
            return result___18;
        }
        if !dedup.is_null() {
            let result___19: avifResult = avifItemPropertyDedupFinish(
                dedup,
                outputStream,
                associations,
                1 as avifBool,
            ) as avifResult;
            if result___19 as core::ffi::c_uint
                != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
            {
                return result___19;
            }
        }
    }
    return AVIF_RESULT_OK;
}
unsafe extern "C" fn avifRWStreamWriteHandlerBox(
    mut s: *mut avifRWStream,
    mut handlerType: *const core::ffi::c_char,
) -> avifResult {
    let mut hdlr: avifBoxMarker = 0;
    let result__: avifResult = avifRWStreamWriteFullBox(
        s,
        b"hdlr\0" as *const u8 as *const core::ffi::c_char,
        0 as size_t,
        0 as core::ffi::c_int,
        0 as uint32_t,
        &mut hdlr,
    ) as avifResult;
    if result__ as core::ffi::c_uint
        != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
    {
        return result__;
    }
    let result___0: avifResult = avifRWStreamWriteU32(s, 0 as uint32_t) as avifResult;
    if result___0 as core::ffi::c_uint
        != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
    {
        return result___0;
    }
    let result___1: avifResult = avifRWStreamWriteChars(
        s,
        handlerType as *const core::ffi::c_char,
        4 as size_t,
    ) as avifResult;
    if result___1 as core::ffi::c_uint
        != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
    {
        return result___1;
    }
    let result___2: avifResult = avifRWStreamWriteZeros(s, 12 as size_t) as avifResult;
    if result___2 as core::ffi::c_uint
        != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
    {
        return result___2;
    }
    let result___3: avifResult = avifRWStreamWriteChars(
        s,
        b"\0" as *const u8 as *const core::ffi::c_char,
        1 as size_t,
    ) as avifResult;
    if result___3 as core::ffi::c_uint
        != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
    {
        return result___3;
    }
    let result___4: avifResult = avifRWStreamFinishBox(s, hdlr) as avifResult;
    if result___4 as core::ffi::c_uint
        != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
    {
        return result___4;
    }
    return AVIF_RESULT_OK;
}
unsafe extern "C" fn avifEncoderWriteTrackMetaBox(
    mut encoder: *mut avifEncoder,
    mut s: *mut avifRWStream,
) -> avifResult {
    let mut metadataItemCount: uint32_t = 0 as uint32_t;
    let mut largeItemIDs: avifBool = AVIF_FALSE;
    let mut itemIndex: uint32_t = 0 as uint32_t;
    while itemIndex < (*(*encoder).data).items.count {
        let mut item: *mut avifEncoderItem = &mut *((*(*encoder).data).items.item)
            .offset(itemIndex as isize) as *mut avifEncoderItem;
        if memcmp(
            ((*item).type_0).as_mut_ptr() as *const core::ffi::c_void,
            (*(*encoder).data).imageItemType as *const core::ffi::c_void,
            4 as size_t,
        ) != 0 as core::ffi::c_int
        {
            metadataItemCount = metadataItemCount.wrapping_add(1);
            largeItemIDs |= ((*item).id > UINT16_MAX as uint32_t) as core::ffi::c_int;
        }
        itemIndex = itemIndex.wrapping_add(1);
    }
    largeItemIDs |= (metadataItemCount > UINT16_MAX as uint32_t) as core::ffi::c_int;
    if metadataItemCount == 0 as uint32_t {
        return AVIF_RESULT_OK;
    }
    let mut meta: avifBoxMarker = 0;
    let result__: avifResult = avifRWStreamWriteFullBox(
        s,
        b"meta\0" as *const u8 as *const core::ffi::c_char,
        0 as size_t,
        0 as core::ffi::c_int,
        0 as uint32_t,
        &mut meta,
    ) as avifResult;
    if result__ as core::ffi::c_uint
        != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
    {
        return result__;
    }
    let result___0: avifResult = avifRWStreamWriteHandlerBox(
        s,
        b"pict\0" as *const u8 as *const core::ffi::c_char,
    ) as avifResult;
    if result___0 as core::ffi::c_uint
        != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
    {
        return result___0;
    }
    let mut iloc: avifBoxMarker = 0;
    let result___1: avifResult = avifRWStreamWriteFullBox(
        s,
        b"iloc\0" as *const u8 as *const core::ffi::c_char,
        0 as size_t,
        if largeItemIDs != 0 { 2 as core::ffi::c_int } else { 0 as core::ffi::c_int },
        0 as uint32_t,
        &mut iloc,
    ) as avifResult;
    if result___1 as core::ffi::c_uint
        != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
    {
        return result___1;
    }
    let result___2: avifResult = avifRWStreamWriteBits(s, 4 as uint32_t, 4 as size_t)
        as avifResult;
    if result___2 as core::ffi::c_uint
        != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
    {
        return result___2;
    }
    let result___3: avifResult = avifRWStreamWriteBits(s, 4 as uint32_t, 4 as size_t)
        as avifResult;
    if result___3 as core::ffi::c_uint
        != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
    {
        return result___3;
    }
    let result___4: avifResult = avifRWStreamWriteBits(s, 0 as uint32_t, 4 as size_t)
        as avifResult;
    if result___4 as core::ffi::c_uint
        != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
    {
        return result___4;
    }
    let result___5: avifResult = avifRWStreamWriteBits(s, 0 as uint32_t, 4 as size_t)
        as avifResult;
    if result___5 as core::ffi::c_uint
        != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
    {
        return result___5;
    }
    if largeItemIDs != 0 {
        let result___6: avifResult = avifRWStreamWriteU32(s, metadataItemCount)
            as avifResult;
        if result___6 as core::ffi::c_uint
            != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
        {
            return result___6;
        }
    } else {
        let result___7: avifResult = avifRWStreamWriteU16(
            s,
            metadataItemCount as uint16_t,
        ) as avifResult;
        if result___7 as core::ffi::c_uint
            != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
        {
            return result___7;
        }
    }
    let mut trakItemIndex: uint32_t = 0 as uint32_t;
    while trakItemIndex < (*(*encoder).data).items.count {
        let mut item_0: *mut avifEncoderItem = &mut *((*(*encoder).data).items.item)
            .offset(trakItemIndex as isize) as *mut avifEncoderItem;
        if !(memcmp(
            ((*item_0).type_0).as_mut_ptr() as *const core::ffi::c_void,
            (*(*encoder).data).imageItemType as *const core::ffi::c_void,
            4 as size_t,
        ) == 0 as core::ffi::c_int)
        {
            if largeItemIDs != 0 {
                let result___8: avifResult = avifRWStreamWriteU32(s, (*item_0).id)
                    as avifResult;
                if result___8 as core::ffi::c_uint
                    != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
                {
                    return result___8;
                }
                let result___9: avifResult = avifRWStreamWriteBits(
                    s,
                    0 as uint32_t,
                    12 as size_t,
                ) as avifResult;
                if result___9 as core::ffi::c_uint
                    != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
                {
                    return result___9;
                }
                let result___10: avifResult = avifRWStreamWriteBits(
                    s,
                    0 as uint32_t,
                    4 as size_t,
                ) as avifResult;
                if result___10 as core::ffi::c_uint
                    != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
                {
                    return result___10;
                }
            } else {
                let result___11: avifResult = avifRWStreamWriteU16(
                    s,
                    (*item_0).id as uint16_t,
                ) as avifResult;
                if result___11 as core::ffi::c_uint
                    != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
                {
                    return result___11;
                }
            }
            let result___12: avifResult = avifRWStreamWriteU16(s, 0 as uint16_t)
                as avifResult;
            if result___12 as core::ffi::c_uint
                != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
            {
                return result___12;
            }
            let result___13: avifResult = avifRWStreamWriteU16(s, 1 as uint16_t)
                as avifResult;
            if result___13 as core::ffi::c_uint
                != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
            {
                return result___13;
            }
            let result___14: avifResult = avifEncoderItemAddMdatFixup(item_0, s)
                as avifResult;
            if result___14 as core::ffi::c_uint
                != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
            {
                return result___14;
            }
            let result___15: avifResult = avifRWStreamWriteU32(s, 0 as uint32_t)
                as avifResult;
            if result___15 as core::ffi::c_uint
                != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
            {
                return result___15;
            }
            let result___16: avifResult = avifRWStreamWriteU32(
                s,
                (*item_0).metadataPayload.size as uint32_t,
            ) as avifResult;
            if result___16 as core::ffi::c_uint
                != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
            {
                return result___16;
            }
        }
        trakItemIndex = trakItemIndex.wrapping_add(1);
    }
    let result___17: avifResult = avifRWStreamFinishBox(s, iloc) as avifResult;
    if result___17 as core::ffi::c_uint
        != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
    {
        return result___17;
    }
    let mut iinf: avifBoxMarker = 0;
    let result___18: avifResult = avifRWStreamWriteFullBox(
        s,
        b"iinf\0" as *const u8 as *const core::ffi::c_char,
        0 as size_t,
        if largeItemIDs != 0 { 1 as core::ffi::c_int } else { 0 as core::ffi::c_int },
        0 as uint32_t,
        &mut iinf,
    ) as avifResult;
    if result___18 as core::ffi::c_uint
        != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
    {
        return result___18;
    }
    if largeItemIDs != 0 {
        let result___19: avifResult = avifRWStreamWriteU32(s, metadataItemCount)
            as avifResult;
        if result___19 as core::ffi::c_uint
            != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
        {
            return result___19;
        }
    } else {
        let result___20: avifResult = avifRWStreamWriteU16(
            s,
            metadataItemCount as uint16_t,
        ) as avifResult;
        if result___20 as core::ffi::c_uint
            != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
        {
            return result___20;
        }
    }
    let mut trakItemIndex_0: uint32_t = 0 as uint32_t;
    while trakItemIndex_0 < (*(*encoder).data).items.count {
        let mut item_1: *mut avifEncoderItem = &mut *((*(*encoder).data).items.item)
            .offset(trakItemIndex_0 as isize) as *mut avifEncoderItem;
        if !(memcmp(
            ((*item_1).type_0).as_mut_ptr() as *const core::ffi::c_void,
            (*(*encoder).data).imageItemType as *const core::ffi::c_void,
            4 as size_t,
        ) == 0 as core::ffi::c_int)
        {
            if (*item_1).hiddenImage != 0 {
                return AVIF_RESULT_INTERNAL_ERROR;
            }
            let mut infe: avifBoxMarker = 0;
            let result___21: avifResult = avifRWStreamWriteFullBox(
                s,
                b"infe\0" as *const u8 as *const core::ffi::c_char,
                0 as size_t,
                if largeItemIDs != 0 {
                    3 as core::ffi::c_int
                } else {
                    2 as core::ffi::c_int
                },
                0 as uint32_t,
                &mut infe,
            ) as avifResult;
            if result___21 as core::ffi::c_uint
                != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
            {
                return result___21;
            }
            if largeItemIDs != 0 {
                let result___22: avifResult = avifRWStreamWriteU32(s, (*item_1).id)
                    as avifResult;
                if result___22 as core::ffi::c_uint
                    != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
                {
                    return result___22;
                }
            } else {
                let result___23: avifResult = avifRWStreamWriteU16(
                    s,
                    (*item_1).id as uint16_t,
                ) as avifResult;
                if result___23 as core::ffi::c_uint
                    != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
                {
                    return result___23;
                }
            }
            let result___24: avifResult = avifRWStreamWriteU16(s, 0 as uint16_t)
                as avifResult;
            if result___24 as core::ffi::c_uint
                != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
            {
                return result___24;
            }
            let result___25: avifResult = avifRWStreamWrite(
                s,
                ((*item_1).type_0).as_mut_ptr() as *const core::ffi::c_void,
                4 as size_t,
            ) as avifResult;
            if result___25 as core::ffi::c_uint
                != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
            {
                return result___25;
            }
            let result___26: avifResult = avifRWStreamWriteChars(
                s,
                (*item_1).infeName,
                (*item_1).infeNameSize,
            ) as avifResult;
            if result___26 as core::ffi::c_uint
                != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
            {
                return result___26;
            }
            if !((*item_1).infeContentType).is_null()
                && (*item_1).infeContentTypeSize != 0
            {
                let result___27: avifResult = avifRWStreamWriteChars(
                    s,
                    (*item_1).infeContentType,
                    (*item_1).infeContentTypeSize,
                ) as avifResult;
                if result___27 as core::ffi::c_uint
                    != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
                {
                    return result___27;
                }
            }
            let result___28: avifResult = avifRWStreamFinishBox(s, infe) as avifResult;
            if result___28 as core::ffi::c_uint
                != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
            {
                return result___28;
            }
        }
        trakItemIndex_0 = trakItemIndex_0.wrapping_add(1);
    }
    let result___29: avifResult = avifRWStreamFinishBox(s, iinf) as avifResult;
    if result___29 as core::ffi::c_uint
        != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
    {
        return result___29;
    }
    let result___30: avifResult = avifRWStreamFinishBox(s, meta) as avifResult;
    if result___30 as core::ffi::c_uint
        != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
    {
        return result___30;
    }
    return AVIF_RESULT_OK;
}
unsafe extern "C" fn avifWriteGridPayload(
    mut data: *mut avifRWData,
    mut gridCols: uint32_t,
    mut gridRows: uint32_t,
    mut gridWidth: uint32_t,
    mut gridHeight: uint32_t,
) -> avifResult {
    let mut gridFlags: uint8_t = (if gridWidth > 65535 as uint32_t
        || gridHeight > 65535 as uint32_t
    {
        1 as core::ffi::c_int
    } else {
        0 as core::ffi::c_int
    }) as uint8_t;
    let mut s: avifRWStream = avifRWStream {
        raw: 0 as *mut avifRWData,
        offset: 0,
        numUsedBitsInPartialByte: 0,
    };
    avifRWStreamStart(&mut s, data);
    let result__: avifResult = avifRWStreamWriteU8(&mut s, 0 as uint8_t) as avifResult;
    if result__ as core::ffi::c_uint
        != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
    {
        return result__;
    }
    let result___0: avifResult = avifRWStreamWriteU8(&mut s, gridFlags) as avifResult;
    if result___0 as core::ffi::c_uint
        != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
    {
        return result___0;
    }
    let result___1: avifResult = avifRWStreamWriteU8(
        &mut s,
        gridRows.wrapping_sub(1 as uint32_t) as uint8_t,
    ) as avifResult;
    if result___1 as core::ffi::c_uint
        != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
    {
        return result___1;
    }
    let result___2: avifResult = avifRWStreamWriteU8(
        &mut s,
        gridCols.wrapping_sub(1 as uint32_t) as uint8_t,
    ) as avifResult;
    if result___2 as core::ffi::c_uint
        != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
    {
        return result___2;
    }
    if gridFlags as core::ffi::c_int & 1 as core::ffi::c_int != 0 {
        let result___3: avifResult = avifRWStreamWriteU32(&mut s, gridWidth)
            as avifResult;
        if result___3 as core::ffi::c_uint
            != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
        {
            return result___3;
        }
        let result___4: avifResult = avifRWStreamWriteU32(&mut s, gridHeight)
            as avifResult;
        if result___4 as core::ffi::c_uint
            != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
        {
            return result___4;
        }
    } else {
        let mut tmpWidth: uint16_t = gridWidth as uint16_t;
        let mut tmpHeight: uint16_t = gridHeight as uint16_t;
        let result___5: avifResult = avifRWStreamWriteU16(&mut s, tmpWidth)
            as avifResult;
        if result___5 as core::ffi::c_uint
            != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
        {
            return result___5;
        }
        let result___6: avifResult = avifRWStreamWriteU16(&mut s, tmpHeight)
            as avifResult;
        if result___6 as core::ffi::c_uint
            != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
        {
            return result___6;
        }
    }
    avifRWStreamFinishWrite(&mut s);
    return AVIF_RESULT_OK;
}
unsafe extern "C" fn avifGainMapIdenticalChannels(
    mut gainMap: *const avifGainMap,
) -> avifBool {
    return ((*gainMap).gainMapMin[0 as core::ffi::c_int as usize].n
        == (*gainMap).gainMapMin[1 as core::ffi::c_int as usize].n
        && (*gainMap).gainMapMin[0 as core::ffi::c_int as usize].n
            == (*gainMap).gainMapMin[2 as core::ffi::c_int as usize].n
        && (*gainMap).gainMapMin[0 as core::ffi::c_int as usize].d
            == (*gainMap).gainMapMin[1 as core::ffi::c_int as usize].d
        && (*gainMap).gainMapMin[0 as core::ffi::c_int as usize].d
            == (*gainMap).gainMapMin[2 as core::ffi::c_int as usize].d
        && (*gainMap).gainMapMax[0 as core::ffi::c_int as usize].n
            == (*gainMap).gainMapMax[1 as core::ffi::c_int as usize].n
        && (*gainMap).gainMapMax[0 as core::ffi::c_int as usize].n
            == (*gainMap).gainMapMax[2 as core::ffi::c_int as usize].n
        && (*gainMap).gainMapMax[0 as core::ffi::c_int as usize].d
            == (*gainMap).gainMapMax[1 as core::ffi::c_int as usize].d
        && (*gainMap).gainMapMax[0 as core::ffi::c_int as usize].d
            == (*gainMap).gainMapMax[2 as core::ffi::c_int as usize].d
        && (*gainMap).gainMapGamma[0 as core::ffi::c_int as usize].n
            == (*gainMap).gainMapGamma[1 as core::ffi::c_int as usize].n
        && (*gainMap).gainMapGamma[0 as core::ffi::c_int as usize].n
            == (*gainMap).gainMapGamma[2 as core::ffi::c_int as usize].n
        && (*gainMap).gainMapGamma[0 as core::ffi::c_int as usize].d
            == (*gainMap).gainMapGamma[1 as core::ffi::c_int as usize].d
        && (*gainMap).gainMapGamma[0 as core::ffi::c_int as usize].d
            == (*gainMap).gainMapGamma[2 as core::ffi::c_int as usize].d
        && (*gainMap).baseOffset[0 as core::ffi::c_int as usize].n
            == (*gainMap).baseOffset[1 as core::ffi::c_int as usize].n
        && (*gainMap).baseOffset[0 as core::ffi::c_int as usize].n
            == (*gainMap).baseOffset[2 as core::ffi::c_int as usize].n
        && (*gainMap).baseOffset[0 as core::ffi::c_int as usize].d
            == (*gainMap).baseOffset[1 as core::ffi::c_int as usize].d
        && (*gainMap).baseOffset[0 as core::ffi::c_int as usize].d
            == (*gainMap).baseOffset[2 as core::ffi::c_int as usize].d
        && (*gainMap).alternateOffset[0 as core::ffi::c_int as usize].n
            == (*gainMap).alternateOffset[1 as core::ffi::c_int as usize].n
        && (*gainMap).alternateOffset[0 as core::ffi::c_int as usize].n
            == (*gainMap).alternateOffset[2 as core::ffi::c_int as usize].n
        && (*gainMap).alternateOffset[0 as core::ffi::c_int as usize].d
            == (*gainMap).alternateOffset[1 as core::ffi::c_int as usize].d
        && (*gainMap).alternateOffset[0 as core::ffi::c_int as usize].d
            == (*gainMap).alternateOffset[2 as core::ffi::c_int as usize].d)
        as core::ffi::c_int;
}
unsafe extern "C" fn avifGainMapMetadataSize(
    mut gainMap: *const avifGainMap,
) -> uint32_t {
    let channelCount: uint8_t = (if avifGainMapIdenticalChannels(gainMap) != 0 {
        1 as core::ffi::c_uint
    } else {
        3 as core::ffi::c_uint
    }) as uint8_t;
    return (::core::mem::size_of::<uint16_t>() as usize)
        .wrapping_mul(2 as usize)
        .wrapping_add(::core::mem::size_of::<uint8_t>() as usize)
        .wrapping_add(
            (::core::mem::size_of::<uint32_t>() as usize).wrapping_mul(4 as usize),
        )
        .wrapping_add(
            (channelCount as usize)
                .wrapping_mul(::core::mem::size_of::<uint32_t>() as usize)
                .wrapping_mul(10 as usize),
        ) as uint32_t;
}
unsafe extern "C" fn avifWriteGainmapMetadata(
    mut s: *mut avifRWStream,
    mut gainMap: *const avifGainMap,
    mut diag: *mut avifDiagnostics,
) -> avifResult {
    let result__: avifResult = avifGainMapValidateMetadata(gainMap, diag) as avifResult;
    if result__ as core::ffi::c_uint
        != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
    {
        return result__;
    }
    let offset: size_t = avifRWStreamOffset(s) as size_t;
    let minimumVersion: uint16_t = 0 as uint16_t;
    let result___0: avifResult = avifRWStreamWriteBits(
        s,
        minimumVersion as uint32_t,
        16 as size_t,
    ) as avifResult;
    if result___0 as core::ffi::c_uint
        != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
    {
        return result___0;
    }
    let writerVersion: uint16_t = 0 as uint16_t;
    let result___1: avifResult = avifRWStreamWriteBits(
        s,
        writerVersion as uint32_t,
        16 as size_t,
    ) as avifResult;
    if result___1 as core::ffi::c_uint
        != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
    {
        return result___1;
    }
    if minimumVersion as core::ffi::c_int == 0 as core::ffi::c_int {
        let channelCount: uint8_t = (if avifGainMapIdenticalChannels(gainMap) != 0 {
            1 as core::ffi::c_uint
        } else {
            3 as core::ffi::c_uint
        }) as uint8_t;
        let result___2: avifResult = avifRWStreamWriteBits(
            s,
            (channelCount as core::ffi::c_int == 3 as core::ffi::c_int)
                as core::ffi::c_int as uint32_t,
            1 as size_t,
        ) as avifResult;
        if result___2 as core::ffi::c_uint
            != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
        {
            return result___2;
        }
        let result___3: avifResult = avifRWStreamWriteBits(
            s,
            (*gainMap).useBaseColorSpace as uint32_t,
            1 as size_t,
        ) as avifResult;
        if result___3 as core::ffi::c_uint
            != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
        {
            return result___3;
        }
        let result___4: avifResult = avifRWStreamWriteBits(s, 0 as uint32_t, 6 as size_t)
            as avifResult;
        if result___4 as core::ffi::c_uint
            != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
        {
            return result___4;
        }
        let result___5: avifResult = avifRWStreamWriteBits(
            s,
            (*gainMap).baseHdrHeadroom.n,
            32 as size_t,
        ) as avifResult;
        if result___5 as core::ffi::c_uint
            != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
        {
            return result___5;
        }
        let result___6: avifResult = avifRWStreamWriteBits(
            s,
            (*gainMap).baseHdrHeadroom.d,
            32 as size_t,
        ) as avifResult;
        if result___6 as core::ffi::c_uint
            != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
        {
            return result___6;
        }
        let result___7: avifResult = avifRWStreamWriteBits(
            s,
            (*gainMap).alternateHdrHeadroom.n,
            32 as size_t,
        ) as avifResult;
        if result___7 as core::ffi::c_uint
            != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
        {
            return result___7;
        }
        let result___8: avifResult = avifRWStreamWriteBits(
            s,
            (*gainMap).alternateHdrHeadroom.d,
            32 as size_t,
        ) as avifResult;
        if result___8 as core::ffi::c_uint
            != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
        {
            return result___8;
        }
        let mut c: core::ffi::c_int = 0 as core::ffi::c_int;
        while c < channelCount as core::ffi::c_int {
            let result___9: avifResult = avifRWStreamWriteBits(
                s,
                (*gainMap).gainMapMin[c as usize].n as uint32_t,
                32 as size_t,
            ) as avifResult;
            if result___9 as core::ffi::c_uint
                != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
            {
                return result___9;
            }
            let result___10: avifResult = avifRWStreamWriteBits(
                s,
                (*gainMap).gainMapMin[c as usize].d,
                32 as size_t,
            ) as avifResult;
            if result___10 as core::ffi::c_uint
                != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
            {
                return result___10;
            }
            let result___11: avifResult = avifRWStreamWriteBits(
                s,
                (*gainMap).gainMapMax[c as usize].n as uint32_t,
                32 as size_t,
            ) as avifResult;
            if result___11 as core::ffi::c_uint
                != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
            {
                return result___11;
            }
            let result___12: avifResult = avifRWStreamWriteBits(
                s,
                (*gainMap).gainMapMax[c as usize].d,
                32 as size_t,
            ) as avifResult;
            if result___12 as core::ffi::c_uint
                != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
            {
                return result___12;
            }
            let result___13: avifResult = avifRWStreamWriteBits(
                s,
                (*gainMap).gainMapGamma[c as usize].n,
                32 as size_t,
            ) as avifResult;
            if result___13 as core::ffi::c_uint
                != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
            {
                return result___13;
            }
            let result___14: avifResult = avifRWStreamWriteBits(
                s,
                (*gainMap).gainMapGamma[c as usize].d,
                32 as size_t,
            ) as avifResult;
            if result___14 as core::ffi::c_uint
                != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
            {
                return result___14;
            }
            let result___15: avifResult = avifRWStreamWriteBits(
                s,
                (*gainMap).baseOffset[c as usize].n as uint32_t,
                32 as size_t,
            ) as avifResult;
            if result___15 as core::ffi::c_uint
                != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
            {
                return result___15;
            }
            let result___16: avifResult = avifRWStreamWriteBits(
                s,
                (*gainMap).baseOffset[c as usize].d,
                32 as size_t,
            ) as avifResult;
            if result___16 as core::ffi::c_uint
                != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
            {
                return result___16;
            }
            let result___17: avifResult = avifRWStreamWriteBits(
                s,
                (*gainMap).alternateOffset[c as usize].n as uint32_t,
                32 as size_t,
            ) as avifResult;
            if result___17 as core::ffi::c_uint
                != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
            {
                return result___17;
            }
            let result___18: avifResult = avifRWStreamWriteBits(
                s,
                (*gainMap).alternateOffset[c as usize].d,
                32 as size_t,
            ) as avifResult;
            if result___18 as core::ffi::c_uint
                != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
            {
                return result___18;
            }
            c += 1;
        }
    }
    if !(avifRWStreamOffset(s)
        == offset.wrapping_add(avifGainMapMetadataSize(gainMap) as size_t))
    {
        return AVIF_RESULT_INTERNAL_ERROR;
    }
    return AVIF_RESULT_OK;
}
unsafe extern "C" fn avifWriteToneMappedImagePayload(
    mut data: *mut avifRWData,
    mut gainMap: *const avifGainMap,
    mut diag: *mut avifDiagnostics,
) -> avifResult {
    let mut s: avifRWStream = avifRWStream {
        raw: 0 as *mut avifRWData,
        offset: 0,
        numUsedBitsInPartialByte: 0,
    };
    avifRWStreamStart(&mut s, data);
    let version: uint8_t = 0 as uint8_t;
    let result__: avifResult = avifRWStreamWriteU8(&mut s, version) as avifResult;
    if result__ as core::ffi::c_uint
        != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
    {
        return result__;
    }
    if version as core::ffi::c_int == 0 as core::ffi::c_int {
        let result___0: avifResult = avifWriteGainmapMetadata(&mut s, gainMap, diag)
            as avifResult;
        if result___0 as core::ffi::c_uint
            != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
        {
            return result___0;
        }
    }
    avifRWStreamFinishWrite(&mut s);
    return AVIF_RESULT_OK;
}
#[no_mangle]
pub unsafe extern "C" fn avifEncoderGetGainMapSizeBytes(
    mut encoder: *mut avifEncoder,
) -> size_t {
    return (*(*encoder).data).gainMapSizeBytes;
}
unsafe extern "C" fn avifImageCopyAltImageMetadata(
    mut altImageMetadata: *mut avifImage,
    mut imageWithGainMap: *const avifImage,
    mut gridWidth: uint32_t,
    mut gridHeight: uint32_t,
) -> avifResult {
    (*altImageMetadata).width = gridWidth;
    (*altImageMetadata).height = gridHeight;
    let result__: avifResult = avifRWDataSet(
        &mut (*altImageMetadata).icc,
        (*(*imageWithGainMap).gainMap).altICC.data,
        (*(*imageWithGainMap).gainMap).altICC.size,
    ) as avifResult;
    if result__ as core::ffi::c_uint
        != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
    {
        return result__;
    }
    (*altImageMetadata).colorPrimaries = (*(*imageWithGainMap).gainMap)
        .altColorPrimaries;
    (*altImageMetadata).transferCharacteristics = (*(*imageWithGainMap).gainMap)
        .altTransferCharacteristics;
    (*altImageMetadata).matrixCoefficients = (*(*imageWithGainMap).gainMap)
        .altMatrixCoefficients;
    (*altImageMetadata).yuvRange = (*(*imageWithGainMap).gainMap).altYUVRange;
    (*altImageMetadata).depth = if (*(*imageWithGainMap).gainMap).altDepth != 0 {
        (*(*imageWithGainMap).gainMap).altDepth
    } else if (*imageWithGainMap).depth > (*(*(*imageWithGainMap).gainMap).image).depth {
        (*imageWithGainMap).depth
    } else {
        (*(*(*imageWithGainMap).gainMap).image).depth
    };
    (*altImageMetadata).yuvFormat = (if (*(*imageWithGainMap).gainMap).altPlaneCount
        == 1 as uint32_t
    {
        AVIF_PIXEL_FORMAT_YUV400 as core::ffi::c_int
    } else {
        AVIF_PIXEL_FORMAT_YUV444 as core::ffi::c_int
    }) as avifPixelFormat;
    (*altImageMetadata).clli = (*(*imageWithGainMap).gainMap).altCLLI;
    return AVIF_RESULT_OK;
}
unsafe extern "C" fn avifEncoderWriteSampleTransformTokens(
    mut s: *mut avifRWStream,
    mut expression: *const avifSampleTransformExpression,
) -> avifResult {
    if !((*expression).count <= 255 as uint32_t) {
        return AVIF_RESULT_INTERNAL_ERROR;
    }
    let result__: avifResult = avifRWStreamWriteU8(s, (*expression).count as uint8_t)
        as avifResult;
    if result__ as core::ffi::c_uint
        != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
    {
        return result__;
    }
    let mut t: uint32_t = 0 as uint32_t;
    while t < (*expression).count {
        let mut token: *const avifSampleTransformToken = &mut *((*expression).tokens)
            .offset(t as isize) as *mut avifSampleTransformToken;
        if (*token).type_0 as core::ffi::c_uint
            == AVIF_SAMPLE_TRANSFORM_CONSTANT as core::ffi::c_int as core::ffi::c_uint
        {
            let result___0: avifResult = avifRWStreamWriteU8(
                s,
                (*token).type_0 as uint8_t,
            ) as avifResult;
            if result___0 as core::ffi::c_uint
                != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
            {
                return result___0;
            }
            let constant: uint32_t = (*token).constant as uint32_t;
            let result___1: avifResult = avifRWStreamWriteU32(s, constant) as avifResult;
            if result___1 as core::ffi::c_uint
                != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
            {
                return result___1;
            }
        } else if (*token).type_0 as core::ffi::c_uint
            == AVIF_SAMPLE_TRANSFORM_INPUT_IMAGE_ITEM_INDEX as core::ffi::c_int
                as core::ffi::c_uint
        {
            let result___2: avifResult = avifRWStreamWriteU8(
                s,
                (*token).inputImageItemIndex,
            ) as avifResult;
            if result___2 as core::ffi::c_uint
                != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
            {
                return result___2;
            }
        } else {
            let result___3: avifResult = avifRWStreamWriteU8(
                s,
                (*token).type_0 as uint8_t,
            ) as avifResult;
            if result___3 as core::ffi::c_uint
                != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
            {
                return result___3;
            }
        }
        t = t.wrapping_add(1);
    }
    return AVIF_RESULT_OK;
}
unsafe extern "C" fn avifEncoderWriteSampleTransformPayload(
    mut encoder: *mut avifEncoder,
    mut data: *mut avifRWData,
) -> avifResult {
    let mut s: avifRWStream = avifRWStream {
        raw: 0 as *mut avifRWData,
        offset: 0,
        numUsedBitsInPartialByte: 0,
    };
    avifRWStreamStart(&mut s, data);
    let result__: avifResult = avifRWStreamWriteBits(&mut s, 0 as uint32_t, 2 as size_t)
        as avifResult;
    if result__ as core::ffi::c_uint
        != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
    {
        return result__;
    }
    let result___0: avifResult = avifRWStreamWriteBits(
        &mut s,
        0 as uint32_t,
        4 as size_t,
    ) as avifResult;
    if result___0 as core::ffi::c_uint
        != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
    {
        return result___0;
    }
    let result___1: avifResult = avifRWStreamWriteBits(
        &mut s,
        AVIF_SAMPLE_TRANSFORM_BIT_DEPTH_32 as core::ffi::c_int as uint32_t,
        2 as size_t,
    ) as avifResult;
    if result___1 as core::ffi::c_uint
        != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
    {
        return result___1;
    }
    let mut expression: avifSampleTransformExpression = {
        let mut init = avifSampleTransformExpression {
            tokens: 0 as *mut avifSampleTransformToken,
            elementSize: 0,
            count: 0,
            capacity: 0,
        };
        init
    };
    let result___2: avifResult = avifSampleTransformRecipeToExpression(
        (*encoder).sampleTransformRecipe,
        &mut expression,
    ) as avifResult;
    if result___2 as core::ffi::c_uint
        != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
    {
        return result___2;
    }
    let result: avifResult = avifEncoderWriteSampleTransformTokens(
        &mut s,
        &mut expression,
    ) as avifResult;
    avifArrayDestroy(
        &mut expression as *mut avifSampleTransformExpression as *mut core::ffi::c_void,
    );
    if result as core::ffi::c_uint
        != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
    {
        avifDiagnosticsPrintf(
            &mut (*encoder).diag as *mut avifDiagnostics,
            b"Failed to write sample transform metadata for recipe %d\0" as *const u8
                as *const core::ffi::c_char,
            (*encoder).sampleTransformRecipe as core::ffi::c_int,
        );
        return result;
    }
    avifRWStreamFinishWrite(&mut s);
    return AVIF_RESULT_OK;
}
unsafe extern "C" fn avifEncoderDataCreateExifItem(
    mut data: *mut avifEncoderData,
    mut exif: *const avifRWData,
) -> avifResult {
    let mut exifTiffHeaderOffset: size_t = 0;
    let result: avifResult = avifGetExifTiffHeaderOffset(
        (*exif).data,
        (*exif).size,
        &mut exifTiffHeaderOffset,
    ) as avifResult;
    if result as core::ffi::c_uint
        != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
    {
        return result;
    }
    let mut exifItem: *mut avifEncoderItem = 0 as *mut avifEncoderItem;
    let result__: avifResult = avifEncoderDataCreateItem(
        data,
        b"Exif\0" as *const u8 as *const core::ffi::c_char,
        b"Exif\0" as *const u8 as *const core::ffi::c_char,
        5 as size_t,
        0 as uint32_t,
        &mut exifItem,
    ) as avifResult;
    if result__ as core::ffi::c_uint
        != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
    {
        return result__;
    }
    (*exifItem).irefToID = (*data).primaryItemID;
    (*exifItem).irefType = b"cdsc\0" as *const u8 as *const core::ffi::c_char;
    let offset32bit: uint32_t = avifHTONL(exifTiffHeaderOffset as uint32_t) as uint32_t;
    let result___0: avifResult = avifRWDataRealloc(
        &mut (*exifItem).metadataPayload,
        (::core::mem::size_of::<uint32_t>() as size_t).wrapping_add((*exif).size),
    ) as avifResult;
    if result___0 as core::ffi::c_uint
        != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
    {
        return result___0;
    }
    memcpy(
        (*exifItem).metadataPayload.data as *mut core::ffi::c_void,
        &offset32bit as *const uint32_t as *const core::ffi::c_void,
        ::core::mem::size_of::<uint32_t>() as size_t,
    );
    memcpy(
        ((*exifItem).metadataPayload.data)
            .offset(::core::mem::size_of::<uint32_t>() as usize as isize)
            as *mut core::ffi::c_void,
        (*exif).data as *const core::ffi::c_void,
        (*exif).size,
    );
    return AVIF_RESULT_OK;
}
unsafe extern "C" fn avifEncoderDataCreateXMPItem(
    mut data: *mut avifEncoderData,
    mut xmp: *const avifRWData,
) -> avifResult {
    let mut xmpItem: *mut avifEncoderItem = 0 as *mut avifEncoderItem;
    let result__: avifResult = avifEncoderDataCreateItem(
        data,
        b"mime\0" as *const u8 as *const core::ffi::c_char,
        b"XMP\0" as *const u8 as *const core::ffi::c_char,
        4 as size_t,
        0 as uint32_t,
        &mut xmpItem,
    ) as avifResult;
    if result__ as core::ffi::c_uint
        != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
    {
        return result__;
    }
    (*xmpItem).irefToID = (*data).primaryItemID;
    (*xmpItem).irefType = b"cdsc\0" as *const u8 as *const core::ffi::c_char;
    (*xmpItem).infeContentType = core::ptr::addr_of!(xmpContentType).cast::<core::ffi::c_char>();
    (*xmpItem).infeContentTypeSize = xmpContentTypeSize;
    let result___0: avifResult = avifRWDataSet(
        &mut (*xmpItem).metadataPayload,
        (*xmp).data,
        (*xmp).size,
    ) as avifResult;
    if result___0 as core::ffi::c_uint
        != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
    {
        return result___0;
    }
    return AVIF_RESULT_OK;
}
unsafe extern "C" fn avifImageCopyAndPad(
    dstImage: *mut avifImage,
    mut srcImage: *const avifImage,
    mut dstWidth: uint32_t,
    mut dstHeight: uint32_t,
) -> avifResult {
    if dstImage.is_null() {
        return AVIF_RESULT_INTERNAL_ERROR;
    }
    if !((*dstImage).width == 0 && (*dstImage).height == 0) {
        return AVIF_RESULT_INTERNAL_ERROR;
    }
    if !(dstWidth >= (*srcImage).width) {
        return AVIF_RESULT_INTERNAL_ERROR;
    }
    if !(dstHeight >= (*srcImage).height) {
        return AVIF_RESULT_INTERNAL_ERROR;
    }
    let result__: avifResult = avifImageCopy(
        dstImage,
        srcImage,
        0 as avifPlanesFlag as avifPlanesFlags,
    ) as avifResult;
    if result__ as core::ffi::c_uint
        != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
    {
        return result__;
    }
    (*dstImage).width = dstWidth;
    (*dstImage).height = dstHeight;
    if !((*srcImage).yuvPlanes[AVIF_CHAN_Y as core::ffi::c_int as usize]).is_null() {
        let result___0: avifResult = avifImageAllocatePlanes(
            dstImage,
            AVIF_PLANES_YUV as core::ffi::c_int as avifPlanesFlags,
        ) as avifResult;
        if result___0 as core::ffi::c_uint
            != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
        {
            return result___0;
        }
    }
    if !((*srcImage).alphaPlane).is_null() {
        let result___1: avifResult = avifImageAllocatePlanes(
            dstImage,
            AVIF_PLANES_A as core::ffi::c_int as avifPlanesFlags,
        ) as avifResult;
        if result___1 as core::ffi::c_uint
            != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
        {
            return result___1;
        }
    }
    let usesU16: avifBool = avifImageUsesU16(srcImage) as avifBool;
    let mut plane: core::ffi::c_int = AVIF_CHAN_Y as core::ffi::c_int;
    while plane <= AVIF_CHAN_A as core::ffi::c_int {
        let mut srcRow: *const uint8_t = avifImagePlane(srcImage, plane);
        let srcRowBytes: uint32_t = avifImagePlaneRowBytes(srcImage, plane) as uint32_t;
        let srcPlaneWidth: uint32_t = avifImagePlaneWidth(srcImage, plane) as uint32_t;
        let srcPlaneHeight: uint32_t = avifImagePlaneHeight(srcImage, plane) as uint32_t;
        let srcPlaneWidthBytes: size_t = (srcPlaneWidth as size_t) << usesU16;
        let mut dstRow: *mut uint8_t = avifImagePlane(dstImage, plane);
        let dstRowBytes: uint32_t = avifImagePlaneRowBytes(dstImage, plane) as uint32_t;
        let dstPlaneWidth: uint32_t = avifImagePlaneWidth(dstImage, plane) as uint32_t;
        let dstPlaneHeight: uint32_t = avifImagePlaneHeight(dstImage, plane) as uint32_t;
        let dstPlaneWidthBytes: size_t = (dstPlaneWidth as size_t) << usesU16;
        let mut j: uint32_t = 0 as uint32_t;
        while j < srcPlaneHeight {
            memcpy(
                dstRow as *mut core::ffi::c_void,
                srcRow as *const core::ffi::c_void,
                srcPlaneWidthBytes,
            );
            if dstPlaneWidth > srcPlaneWidth {
                if usesU16 != 0 {
                    let mut dstRow16: *mut uint16_t = dstRow as *mut uint16_t;
                    let mut x: uint32_t = srcPlaneWidth;
                    while x < dstPlaneWidth {
                        *dstRow16.offset(x as isize) = *dstRow16
                            .offset(srcPlaneWidth.wrapping_sub(1 as uint32_t) as isize);
                        x = x.wrapping_add(1);
                    }
                } else {
                    memset(
                        &mut *dstRow.offset(srcPlaneWidth as isize) as *mut uint8_t
                            as *mut core::ffi::c_void,
                        *dstRow
                            .offset(srcPlaneWidth.wrapping_sub(1 as uint32_t) as isize)
                            as core::ffi::c_int,
                        dstPlaneWidth.wrapping_sub(srcPlaneWidth) as size_t,
                    );
                }
            }
            srcRow = srcRow.offset(srcRowBytes as isize);
            dstRow = dstRow.offset(dstRowBytes as isize);
            j = j.wrapping_add(1);
        }
        let mut j_0: uint32_t = srcPlaneHeight;
        while j_0 < dstPlaneHeight {
            memcpy(
                dstRow as *mut core::ffi::c_void,
                dstRow.offset(-(dstRowBytes as isize)) as *const core::ffi::c_void,
                dstPlaneWidthBytes,
            );
            dstRow = dstRow.offset(dstRowBytes as isize);
            j_0 = j_0.wrapping_add(1);
        }
        plane += 1;
    }
    return AVIF_RESULT_OK;
}
unsafe extern "C" fn avifGetQuality(
    mut quality: core::ffi::c_int,
    mut minQuantizer: core::ffi::c_int,
    mut maxQuantizer: core::ffi::c_int,
) -> core::ffi::c_int {
    let mut quantizer: core::ffi::c_int = 0;
    if quality == AVIF_QUALITY_DEFAULT {
        quantizer = (minQuantizer + maxQuantizer) / 2 as core::ffi::c_int;
        quantizer = if quantizer < 0 as core::ffi::c_int {
            0 as core::ffi::c_int
        } else if (63 as core::ffi::c_int) < quantizer {
            63 as core::ffi::c_int
        } else {
            quantizer
        };
        quality = ((63 as core::ffi::c_int - quantizer) * 100 as core::ffi::c_int
            + 31 as core::ffi::c_int) / 63 as core::ffi::c_int;
    } else {
        quality = if quality < 0 as core::ffi::c_int {
            0 as core::ffi::c_int
        } else if (100 as core::ffi::c_int) < quality {
            100 as core::ffi::c_int
        } else {
            quality
        };
    }
    return quality;
}
static mut infeNameColor: [core::ffi::c_char; 6] = unsafe {
    ::core::mem::transmute::<[u8; 6], [core::ffi::c_char; 6]>(*b"Color\0")
};
static mut infeNameAlpha: [core::ffi::c_char; 6] = unsafe {
    ::core::mem::transmute::<[u8; 6], [core::ffi::c_char; 6]>(*b"Alpha\0")
};
static mut infeNameGainMap: [core::ffi::c_char; 5] = unsafe {
    ::core::mem::transmute::<[u8; 5], [core::ffi::c_char; 5]>(*b"GMap\0")
};
static mut infeNameSampleTransform: [core::ffi::c_char; 16] = unsafe {
    ::core::mem::transmute::<[u8; 16], [core::ffi::c_char; 16]>(*b"SampleTransform\0")
};
unsafe extern "C" fn getInfeName(
    mut itemCategory: avifItemCategory,
) -> *const core::ffi::c_char {
    if avifIsAlpha(itemCategory) != 0 {
        return core::ptr::addr_of!(infeNameAlpha).cast::<core::ffi::c_char>();
    }
    if itemCategory as core::ffi::c_uint
        == AVIF_ITEM_GAIN_MAP as core::ffi::c_int as core::ffi::c_uint
    {
        return core::ptr::addr_of!(infeNameGainMap).cast::<core::ffi::c_char>();
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
        return core::ptr::addr_of!(infeNameSampleTransform).cast::<core::ffi::c_char>();
    }
    return core::ptr::addr_of!(infeNameColor).cast::<core::ffi::c_char>();
}
unsafe extern "C" fn avifEncoderAddImageItems(
    mut encoder: *mut avifEncoder,
    mut gridCols: uint32_t,
    mut gridRows: uint32_t,
    mut gridWidth: uint32_t,
    mut gridHeight: uint32_t,
    mut itemCategory: avifItemCategory,
    mut topLevelItemID: *mut uint32_t,
) -> avifResult {
    let cellCount: uint32_t = gridCols.wrapping_mul(gridRows);
    let mut infeName: *const core::ffi::c_char = getInfeName(itemCategory);
    let infeNameSize: size_t = (strlen(infeName) as size_t).wrapping_add(1 as size_t);
    if cellCount > 1 as uint32_t {
        let mut gridItem: *mut avifEncoderItem = 0 as *mut avifEncoderItem;
        let result__: avifResult = avifEncoderDataCreateItem(
            (*encoder).data as *mut avifEncoderData,
            b"grid\0" as *const u8 as *const core::ffi::c_char,
            infeName,
            infeNameSize,
            0 as uint32_t,
            &mut gridItem,
        ) as avifResult;
        if result__ as core::ffi::c_uint
            != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
        {
            return result__;
        }
        let result___0: avifResult = avifWriteGridPayload(
            &mut (*gridItem).metadataPayload,
            gridCols,
            gridRows,
            gridWidth,
            gridHeight,
        ) as avifResult;
        if result___0 as core::ffi::c_uint
            != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
        {
            return result___0;
        }
        (*gridItem).itemCategory = itemCategory;
        (*gridItem).gridCols = gridCols;
        (*gridItem).gridRows = gridRows;
        (*gridItem).gridWidth = gridWidth;
        (*gridItem).gridHeight = gridHeight;
        *topLevelItemID = (*gridItem).id;
    }
    let mut cellIndex: uint32_t = 0 as uint32_t;
    while cellIndex < cellCount {
        let mut item: *mut avifEncoderItem = 0 as *mut avifEncoderItem;
        let result___1: avifResult = avifEncoderDataCreateItem(
            (*encoder).data as *mut avifEncoderData,
            (*(*encoder).data).imageItemType,
            infeName,
            infeNameSize,
            cellIndex,
            &mut item,
        ) as avifResult;
        if result___1 as core::ffi::c_uint
            != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
        {
            return result___1;
        }
        let result___2: avifResult = avifCodecCreate(
            (*encoder).codecChoice,
            AVIF_CODEC_FLAG_CAN_ENCODE as core::ffi::c_int as avifCodecFlags,
            &mut (*item).codec,
        ) as avifResult;
        if result___2 as core::ffi::c_uint
            != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
        {
            return result___2;
        }
        (*(*item).codec).csOptions = (*encoder).csOptions;
        (*(*item).codec).diag = &mut (*encoder).diag;
        (*item).itemCategory = itemCategory;
        (*item).extraLayerCount = (*encoder).extraLayerCount;
        if cellCount > 1 as uint32_t {
            (*item).dimgFromID = *topLevelItemID;
            (*item).hiddenImage = AVIF_TRUE as avifBool;
        } else {
            *topLevelItemID = (*item).id;
        }
        cellIndex = cellIndex.wrapping_add(1);
    }
    return AVIF_RESULT_OK;
}
unsafe extern "C" fn avifEncoderCreateBitDepthExtensionItems(
    mut encoder: *mut avifEncoder,
    mut gridCols: uint32_t,
    mut gridRows: uint32_t,
    mut gridWidth: uint32_t,
    mut gridHeight: uint32_t,
    mut colorItemID: uint32_t,
) -> avifResult {
    if !((*encoder).sampleTransformRecipe as core::ffi::c_uint
        == AVIF_SAMPLE_TRANSFORM_BIT_DEPTH_EXTENSION_8B_8B as core::ffi::c_int
            as core::ffi::c_uint
        || (*encoder).sampleTransformRecipe as core::ffi::c_uint
            == AVIF_SAMPLE_TRANSFORM_BIT_DEPTH_EXTENSION_12B_4B as core::ffi::c_int
                as core::ffi::c_uint
        || (*encoder).sampleTransformRecipe as core::ffi::c_uint
            == AVIF_SAMPLE_TRANSFORM_BIT_DEPTH_EXTENSION_12B_8B_OVERLAP_4B
                as core::ffi::c_int as core::ffi::c_uint)
    {
        return AVIF_RESULT_INTERNAL_ERROR;
    }
    let mut sampleTransformItem: *mut avifEncoderItem = 0 as *mut avifEncoderItem;
    let result__: avifResult = avifEncoderDataCreateItem(
        (*encoder).data as *mut avifEncoderData,
        b"sato\0" as *const u8 as *const core::ffi::c_char,
        core::ptr::addr_of!(infeNameSampleTransform).cast::<core::ffi::c_char>(),
        (strlen(core::ptr::addr_of!(infeNameSampleTransform).cast::<core::ffi::c_char>())).wrapping_add(1 as size_t),
        0 as uint32_t,
        &mut sampleTransformItem,
    ) as avifResult;
    if result__ as core::ffi::c_uint
        != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
    {
        return result__;
    }
    let result___0: avifResult = avifEncoderWriteSampleTransformPayload(
        encoder,
        &mut (*sampleTransformItem).metadataPayload,
    ) as avifResult;
    if result___0 as core::ffi::c_uint
        != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
    {
        return result___0;
    }
    (*sampleTransformItem).itemCategory = AVIF_ITEM_SAMPLE_TRANSFORM;
    let mut sampleTransformItemID: uint32_t = (*sampleTransformItem).id;
    if !((*(*encoder).data).alternativeItemIDs.count == 0 as uint32_t) {
        return AVIF_RESULT_INTERNAL_ERROR;
    }
    let mut alternativeItemID: *mut uint32_t = avifArrayPush(
        &mut (*(*encoder).data).alternativeItemIDs as *mut avifEncoderItemIdArray
            as *mut core::ffi::c_void,
    ) as *mut uint32_t;
    if alternativeItemID.is_null() {
        return AVIF_RESULT_OUT_OF_MEMORY;
    }
    *alternativeItemID = (*sampleTransformItem).id;
    alternativeItemID = avifArrayPush(
        &mut (*(*encoder).data).alternativeItemIDs as *mut avifEncoderItemIdArray
            as *mut core::ffi::c_void,
    ) as *mut uint32_t;
    if alternativeItemID.is_null() {
        return AVIF_RESULT_OUT_OF_MEMORY;
    }
    *alternativeItemID = colorItemID;
    let mut bitDepthExtensionColorItemId: uint32_t = 0;
    let result___1: avifResult = avifEncoderAddImageItems(
        encoder,
        gridCols,
        gridRows,
        gridWidth,
        gridHeight,
        AVIF_ITEM_SAMPLE_TRANSFORM_INPUT_0_COLOR,
        &mut bitDepthExtensionColorItemId,
    ) as avifResult;
    if result___1 as core::ffi::c_uint
        != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
    {
        return result___1;
    }
    let mut bitDepthExtensionColorItem: *mut avifEncoderItem = avifEncoderDataFindItemByID(
        (*encoder).data as *mut avifEncoderData,
        bitDepthExtensionColorItemId,
    );
    (*bitDepthExtensionColorItem).hiddenImage = AVIF_TRUE as avifBool;
    if !(colorItemID < bitDepthExtensionColorItemId) {
        return AVIF_RESULT_INTERNAL_ERROR;
    }
    let mut colorItem: *mut avifEncoderItem = avifEncoderDataFindItemByID(
        (*encoder).data as *mut avifEncoderData,
        colorItemID,
    );
    if colorItem.is_null() {
        return AVIF_RESULT_INTERNAL_ERROR;
    }
    if !((*colorItem).dimgFromID == 0 as uint32_t) {
        return AVIF_RESULT_INTERNAL_ERROR;
    }
    (*colorItem).dimgFromID = sampleTransformItemID;
    (*bitDepthExtensionColorItem).dimgFromID = sampleTransformItemID;
    if (*(*encoder).data).alphaPresent != 0 {
        let mut bitDepthExtensionAlphaItemId: uint32_t = 0;
        let result___2: avifResult = avifEncoderAddImageItems(
            encoder,
            gridCols,
            gridRows,
            gridWidth,
            gridHeight,
            AVIF_ITEM_SAMPLE_TRANSFORM_INPUT_0_ALPHA,
            &mut bitDepthExtensionAlphaItemId,
        ) as avifResult;
        if result___2 as core::ffi::c_uint
            != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
        {
            return result___2;
        }
        let mut bitDepthExtensionAlphaItem: *mut avifEncoderItem = avifEncoderDataFindItemByID(
            (*encoder).data as *mut avifEncoderData,
            bitDepthExtensionAlphaItemId,
        );
        (*bitDepthExtensionAlphaItem).irefType = b"auxl\0" as *const u8
            as *const core::ffi::c_char;
        (*bitDepthExtensionAlphaItem).irefToID = bitDepthExtensionColorItemId;
        if (*(*(*encoder).data).imageMetadata).alphaPremultiplied != 0 {
            bitDepthExtensionColorItem = avifEncoderDataFindItemByID(
                (*encoder).data as *mut avifEncoderData,
                bitDepthExtensionColorItemId,
            );
            (*bitDepthExtensionColorItem).irefType = b"prem\0" as *const u8
                as *const core::ffi::c_char;
            (*bitDepthExtensionColorItem).irefToID = bitDepthExtensionAlphaItemId;
        }
    }
    return AVIF_RESULT_OK;
}
unsafe extern "C" fn avifImageApplyImgOpConst(
    mut result: *mut avifImage,
    mut inputImageItem: *const avifImage,
    mut op: avifSampleTransformTokenType,
    mut constant: int32_t,
    mut planes: avifPlanesFlags,
) -> avifResult {
    let tokens: [avifSampleTransformToken; 3] = [
        {
            let mut init = avifSampleTransformToken {
                type_0: AVIF_SAMPLE_TRANSFORM_INPUT_IMAGE_ITEM_INDEX,
                constant: 0 as int32_t,
                inputImageItemIndex: 1 as uint8_t,
            };
            init
        },
        {
            let mut init = avifSampleTransformToken {
                type_0: AVIF_SAMPLE_TRANSFORM_CONSTANT,
                constant: constant,
                inputImageItemIndex: 0 as uint8_t,
            };
            init
        },
        {
            let mut init = avifSampleTransformToken {
                type_0: op as uint8_t as avifSampleTransformTokenType,
                constant: 0 as int32_t,
                inputImageItemIndex: 0 as uint8_t,
            };
            init
        },
    ];
    return avifImageApplyOperations(
        result,
        AVIF_SAMPLE_TRANSFORM_BIT_DEPTH_32,
        3 as uint32_t,
        tokens.as_ptr(),
        1 as uint8_t,
        &mut inputImageItem,
        planes,
    );
}
unsafe extern "C" fn avifImageCreateAllocate(
    mut sampleTransformedImage: *mut *mut avifImage,
    mut reference: *const avifImage,
    mut numBits: uint32_t,
    mut planes: avifPlanesFlag,
) -> avifResult {
    *sampleTransformedImage = avifImageCreate(
        (*reference).width,
        (*reference).height,
        numBits,
        (*reference).yuvFormat,
    );
    if (*sampleTransformedImage).is_null() {
        return AVIF_RESULT_OUT_OF_MEMORY;
    }
    return avifImageAllocatePlanes(*sampleTransformedImage, planes as avifPlanesFlags);
}
unsafe extern "C" fn avifEncoderDecodeSatoBaseImage(
    mut encoder: *mut avifEncoder,
    mut cellIndex: uint32_t,
    mut original: *const avifImage,
    mut numBits: uint32_t,
    mut planes: avifPlanesFlag,
    mut codec: *mut *mut avifCodec,
    mut decodedBaseImage: *mut *mut avifImage,
) -> avifResult {
    let mut sample: avifDecodeSample = avifDecodeSample {
        data: avifROData {
            data: 0 as *const uint8_t,
            size: 0,
        },
        ownsData: 0,
        partialData: 0,
        itemID: 0,
        offset: 0,
        size: 0,
        spatialID: 0,
        sync: 0,
    };
    memset(
        &mut sample as *mut avifDecodeSample as *mut core::ffi::c_void,
        0 as core::ffi::c_int,
        ::core::mem::size_of::<avifDecodeSample>() as size_t,
    );
    sample.spatialID = AVIF_SPATIAL_ID_UNSET as uint8_t;
    let mut itemIndex: uint32_t = 0 as uint32_t;
    while itemIndex < (*(*encoder).data).items.count {
        let mut item: *mut avifEncoderItem = &mut *((*(*encoder).data).items.item)
            .offset(itemIndex as isize) as *mut avifEncoderItem;
        if !((*item).codec).is_null() {
            if !(((*item).itemCategory as core::ffi::c_uint
                != AVIF_ITEM_COLOR as core::ffi::c_int as core::ffi::c_uint
                || planes as core::ffi::c_uint
                    != AVIF_PLANES_YUV as core::ffi::c_int as core::ffi::c_uint)
                && ((*item).itemCategory as core::ffi::c_uint
                    != AVIF_ITEM_ALPHA as core::ffi::c_int as core::ffi::c_uint
                    || planes as core::ffi::c_uint
                        != AVIF_PLANES_A as core::ffi::c_int as core::ffi::c_uint))
            {
                if !((*item).cellIndex != cellIndex) {
                    if ((*item).encodeOutput).is_null() {
                        return AVIF_RESULT_INTERNAL_ERROR;
                    }
                    if !((*(*item).encodeOutput).samples.count == 1 as uint32_t) {
                        return AVIF_RESULT_INTERNAL_ERROR;
                    }
                    if !((*((*(*item).encodeOutput).samples.sample)
                        .offset(0 as core::ffi::c_int as isize))
                        .data
                        .size != 0 as size_t)
                    {
                        return AVIF_RESULT_INTERNAL_ERROR;
                    }
                    if !(sample.data.size == 0 as size_t) {
                        return AVIF_RESULT_INTERNAL_ERROR;
                    }
                    sample.data.data = (*((*(*item).encodeOutput).samples.sample)
                        .offset(0 as core::ffi::c_int as isize))
                        .data
                        .data;
                    sample.data.size = (*((*(*item).encodeOutput).samples.sample)
                        .offset(0 as core::ffi::c_int as isize))
                        .data
                        .size;
                }
            }
        }
        itemIndex = itemIndex.wrapping_add(1);
    }
    if !(sample.data.size != 0 as size_t) {
        return AVIF_RESULT_INTERNAL_ERROR;
    }
    let result__: avifResult = avifCodecCreate(
        AVIF_CODEC_CHOICE_AUTO,
        AVIF_CODEC_FLAG_CAN_DECODE as core::ffi::c_int as avifCodecFlags,
        codec,
    ) as avifResult;
    if result__ as core::ffi::c_uint
        != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
    {
        return result__;
    }
    (**codec).diag = &mut (*encoder).diag;
    (**codec).maxThreads = (*encoder).maxThreads;
    (**codec).imageSizeLimit = AVIF_DEFAULT_IMAGE_SIZE_LIMIT as uint32_t;
    (**codec).imageDimensionLimit = AVIF_DEFAULT_IMAGE_DIMENSION_LIMIT as uint32_t;
    let result___0: avifResult = avifImageCreateAllocate(
        decodedBaseImage,
        original,
        numBits,
        planes,
    ) as avifResult;
    if result___0 as core::ffi::c_uint
        != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
    {
        return result___0;
    }
    let mut isLimitedRangeAlpha: avifBool = AVIF_FALSE;
    if ((**codec).getNextImage)
        .expect(
            "non-null function pointer",
        )(
        *codec,
        &mut sample,
        (planes as core::ffi::c_uint
            == AVIF_PLANES_A as core::ffi::c_int as core::ffi::c_uint)
            as core::ffi::c_int,
        &mut isLimitedRangeAlpha,
        *decodedBaseImage,
    ) == 0
    {
        return AVIF_RESULT_ENCODE_SAMPLE_TRANSFORM_FAILED;
    }
    return AVIF_RESULT_OK;
}
unsafe extern "C" fn avifEncoderCreateSatoImage(
    mut encoder: *mut avifEncoder,
    mut item: *const avifEncoderItem,
    mut itemWillBeEncodedLosslessly: avifBool,
    mut image: *const avifImage,
    mut sampleTransformedImage: *mut *mut avifImage,
) -> avifResult {
    let planes: avifPlanesFlag = (if avifIsAlpha((*item).itemCategory) != 0 {
        AVIF_PLANES_A as core::ffi::c_int
    } else {
        AVIF_PLANES_YUV as core::ffi::c_int
    }) as avifPlanesFlag;
    let mut isBase: avifBool = ((*item).itemCategory as core::ffi::c_uint
        == AVIF_ITEM_COLOR as core::ffi::c_int as core::ffi::c_uint
        || (*item).itemCategory as core::ffi::c_uint
            == AVIF_ITEM_ALPHA as core::ffi::c_int as core::ffi::c_uint)
        as core::ffi::c_int;
    if isBase == 0 {
        if !((*item).itemCategory as core::ffi::c_uint
            >= AVIF_ITEM_SAMPLE_TRANSFORM_INPUT_0_COLOR as core::ffi::c_int
                as core::ffi::c_uint
            && (*item).itemCategory as core::ffi::c_uint
                <= (AVIF_ITEM_SAMPLE_TRANSFORM_INPUT_0_ALPHA as core::ffi::c_int
                    + (AVIF_ITEM_SAMPLE_TRANSFORM_INPUT_0_ALPHA as core::ffi::c_int
                        - AVIF_ITEM_SAMPLE_TRANSFORM_INPUT_0_COLOR as core::ffi::c_int)
                    - 1 as core::ffi::c_int) as core::ffi::c_uint)
        {
            return AVIF_RESULT_INTERNAL_ERROR;
        }
    }
    if (*encoder).sampleTransformRecipe as core::ffi::c_uint
        == AVIF_SAMPLE_TRANSFORM_BIT_DEPTH_EXTENSION_8B_8B as core::ffi::c_int
            as core::ffi::c_uint
    {
        if isBase != 0 {
            let result__: avifResult = avifImageCreateAllocate(
                sampleTransformedImage,
                image,
                8 as uint32_t,
                planes,
            ) as avifResult;
            if result__ as core::ffi::c_uint
                != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
            {
                return result__;
            }
            let result___0: avifResult = avifImageApplyImgOpConst(
                *sampleTransformedImage,
                image,
                AVIF_SAMPLE_TRANSFORM_QUOTIENT,
                256 as int32_t,
                planes as avifPlanesFlags,
            ) as avifResult;
            if result___0 as core::ffi::c_uint
                != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
            {
                return result___0;
            }
        } else {
            let result___1: avifResult = avifImageCreateAllocate(
                sampleTransformedImage,
                image,
                8 as uint32_t,
                planes,
            ) as avifResult;
            if result___1 as core::ffi::c_uint
                != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
            {
                return result___1;
            }
            let result___2: avifResult = avifImageApplyImgOpConst(
                *sampleTransformedImage,
                image,
                AVIF_SAMPLE_TRANSFORM_AND,
                255 as int32_t,
                planes as avifPlanesFlags,
            ) as avifResult;
            if result___2 as core::ffi::c_uint
                != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
            {
                return result___2;
            }
        }
    } else if (*encoder).sampleTransformRecipe as core::ffi::c_uint
        == AVIF_SAMPLE_TRANSFORM_BIT_DEPTH_EXTENSION_12B_4B as core::ffi::c_int
            as core::ffi::c_uint
    {
        if isBase != 0 {
            let result___3: avifResult = avifImageCreateAllocate(
                sampleTransformedImage,
                image,
                12 as uint32_t,
                planes,
            ) as avifResult;
            if result___3 as core::ffi::c_uint
                != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
            {
                return result___3;
            }
            let result___4: avifResult = avifImageApplyImgOpConst(
                *sampleTransformedImage,
                image,
                AVIF_SAMPLE_TRANSFORM_QUOTIENT,
                16 as int32_t,
                planes as avifPlanesFlags,
            ) as avifResult;
            if result___4 as core::ffi::c_uint
                != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
            {
                return result___4;
            }
        } else {
            let result___5: avifResult = avifImageCreateAllocate(
                sampleTransformedImage,
                image,
                8 as uint32_t,
                planes,
            ) as avifResult;
            if result___5 as core::ffi::c_uint
                != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
            {
                return result___5;
            }
            let result___6: avifResult = avifImageApplyImgOpConst(
                *sampleTransformedImage,
                image,
                AVIF_SAMPLE_TRANSFORM_AND,
                15 as int32_t,
                planes as avifPlanesFlags,
            ) as avifResult;
            if result___6 as core::ffi::c_uint
                != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
            {
                return result___6;
            }
            let result___7: avifResult = avifImageApplyImgOpConst(
                *sampleTransformedImage,
                *sampleTransformedImage,
                AVIF_SAMPLE_TRANSFORM_PRODUCT,
                16 as int32_t,
                planes as avifPlanesFlags,
            ) as avifResult;
            if result___7 as core::ffi::c_uint
                != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
            {
                return result___7;
            }
            if itemWillBeEncodedLosslessly == 0 {
                let result___8: avifResult = avifImageApplyImgOpConst(
                    *sampleTransformedImage,
                    *sampleTransformedImage,
                    AVIF_SAMPLE_TRANSFORM_SUM,
                    7 as int32_t,
                    planes as avifPlanesFlags,
                ) as avifResult;
                if result___8 as core::ffi::c_uint
                    != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
                {
                    return result___8;
                }
            }
        }
    } else {
        if !((*encoder).sampleTransformRecipe as core::ffi::c_uint
            == AVIF_SAMPLE_TRANSFORM_BIT_DEPTH_EXTENSION_12B_8B_OVERLAP_4B
                as core::ffi::c_int as core::ffi::c_uint)
        {
            return AVIF_RESULT_NOT_IMPLEMENTED;
        }
        if isBase != 0 {
            let result___9: avifResult = avifImageCreateAllocate(
                sampleTransformedImage,
                image,
                12 as uint32_t,
                planes,
            ) as avifResult;
            if result___9 as core::ffi::c_uint
                != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
            {
                return result___9;
            }
            let result___10: avifResult = avifImageApplyImgOpConst(
                *sampleTransformedImage,
                image,
                AVIF_SAMPLE_TRANSFORM_QUOTIENT,
                16 as int32_t,
                planes as avifPlanesFlags,
            ) as avifResult;
            if result___10 as core::ffi::c_uint
                != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
            {
                return result___10;
            }
        } else {
            let result___11: avifResult = avifImageCreateAllocate(
                sampleTransformedImage,
                image,
                8 as uint32_t,
                planes,
            ) as avifResult;
            if result___11 as core::ffi::c_uint
                != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
            {
                return result___11;
            }
            let mut codec: *mut avifCodec = 0 as *mut avifCodec;
            let mut decodedBaseImage: *mut avifImage = 0 as *mut avifImage;
            let mut result: avifResult = avifEncoderDecodeSatoBaseImage(
                encoder,
                (*item).cellIndex,
                image,
                12 as uint32_t,
                planes,
                &mut codec,
                &mut decodedBaseImage,
            );
            if result as core::ffi::c_uint
                == AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
            {
                let tokens: [avifSampleTransformToken; 7] = [
                    {
                        let mut init = avifSampleTransformToken {
                            type_0: AVIF_SAMPLE_TRANSFORM_INPUT_IMAGE_ITEM_INDEX,
                            constant: 0 as int32_t,
                            inputImageItemIndex: 1 as uint8_t,
                        };
                        init
                    },
                    {
                        let mut init = avifSampleTransformToken {
                            type_0: AVIF_SAMPLE_TRANSFORM_INPUT_IMAGE_ITEM_INDEX,
                            constant: 0 as int32_t,
                            inputImageItemIndex: 2 as uint8_t,
                        };
                        init
                    },
                    {
                        let mut init = avifSampleTransformToken {
                            type_0: AVIF_SAMPLE_TRANSFORM_CONSTANT,
                            constant: 16 as int32_t,
                            inputImageItemIndex: 0 as uint8_t,
                        };
                        init
                    },
                    {
                        let mut init = avifSampleTransformToken {
                            type_0: AVIF_SAMPLE_TRANSFORM_PRODUCT,
                            constant: 0 as int32_t,
                            inputImageItemIndex: 0 as uint8_t,
                        };
                        init
                    },
                    {
                        let mut init = avifSampleTransformToken {
                            type_0: AVIF_SAMPLE_TRANSFORM_DIFFERENCE,
                            constant: 0 as int32_t,
                            inputImageItemIndex: 0 as uint8_t,
                        };
                        init
                    },
                    {
                        let mut init = avifSampleTransformToken {
                            type_0: AVIF_SAMPLE_TRANSFORM_CONSTANT,
                            constant: 128 as int32_t,
                            inputImageItemIndex: 0 as uint8_t,
                        };
                        init
                    },
                    {
                        let mut init = avifSampleTransformToken {
                            type_0: AVIF_SAMPLE_TRANSFORM_SUM,
                            constant: 0 as int32_t,
                            inputImageItemIndex: 0 as uint8_t,
                        };
                        init
                    },
                ];
                let mut inputImageItems: [*const avifImage; 2] = [
                    image,
                    decodedBaseImage as *const avifImage,
                ];
                result = avifImageApplyOperations(
                    *sampleTransformedImage,
                    AVIF_SAMPLE_TRANSFORM_BIT_DEPTH_32,
                    7 as uint32_t,
                    tokens.as_ptr(),
                    2 as uint8_t,
                    inputImageItems.as_mut_ptr(),
                    planes as avifPlanesFlags,
                );
            }
            if !decodedBaseImage.is_null() {
                avifImageDestroy(decodedBaseImage);
            }
            if !codec.is_null() {
                avifCodecDestroy(codec);
            }
            let result___12: avifResult = result;
            if result___12 as core::ffi::c_uint
                != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
            {
                return result___12;
            }
        }
    }
    return AVIF_RESULT_OK;
}
unsafe extern "C" fn avifEncoderCreateBitDepthExtensionImage(
    mut encoder: *mut avifEncoder,
    mut item: *const avifEncoderItem,
    mut itemWillBeEncodedLosslessly: avifBool,
    mut image: *const avifImage,
    mut sampleTransformedImage: *mut *mut avifImage,
) -> avifResult {
    if !((*image).depth == 16 as uint32_t) {
        return AVIF_RESULT_INTERNAL_ERROR;
    }
    *sampleTransformedImage = 0 as *mut avifImage;
    let result: avifResult = avifEncoderCreateSatoImage(
        encoder,
        item,
        itemWillBeEncodedLosslessly,
        image,
        sampleTransformedImage,
    ) as avifResult;
    if result as core::ffi::c_uint
        != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
        && !(*sampleTransformedImage).is_null()
    {
        avifImageDestroy(*sampleTransformedImage);
    }
    return result;
}
unsafe extern "C" fn avifEncoderGetCodecType(
    mut encoder: *const avifEncoder,
) -> avifCodecType {
    return avifCodecTypeFromChoice(
        (*encoder).codecChoice,
        AVIF_CODEC_FLAG_CAN_ENCODE as core::ffi::c_int as avifCodecFlags,
    );
}
unsafe extern "C" fn avifEncoderValidateSize(
    mut encoder: *mut avifEncoder,
    mut gridCols: uint32_t,
    mut gridRows: uint32_t,
    mut firstCell: *const avifImage,
) -> avifResult {
    if avifEncoderSizeIsSet(encoder) == 0 {
        if (*(*encoder).data).items.count == 0 as uint32_t {
            return AVIF_RESULT_OK;
        }
        if (*firstCell).width != (*(*(*encoder).data).imageMetadata).width
            || (*firstCell).height != (*(*(*encoder).data).imageMetadata).height
        {
            avifDiagnosticsPrintf(
                &mut (*encoder).diag as *mut avifDiagnostics,
                b"All images must have the same width/height unless avifEncoder.width/height is set\0"
                    as *const u8 as *const core::ffi::c_char,
            );
            return AVIF_RESULT_INVALID_ARGUMENT;
        }
        if (*(*encoder).data).frames.count == (*encoder).extraLayerCount
            && avifScalingModeIsNoScaling(&mut (*encoder).scalingMode) == 0
        {
            avifDiagnosticsPrintf(
                &mut (*encoder).diag as *mut avifDiagnostics,
                b"The last layer must not be scaled, but got scalingMode=(%d/%d, %d/%d)\0"
                    as *const u8 as *const core::ffi::c_char,
                (*encoder).scalingMode.horizontal.n,
                (*encoder).scalingMode.horizontal.d,
                (*encoder).scalingMode.vertical.n,
                (*encoder).scalingMode.vertical.d,
            );
            return AVIF_RESULT_INVALID_ARGUMENT;
        }
        return AVIF_RESULT_OK;
    }
    if (*encoder).width == 0 as uint32_t || (*encoder).height == 0 as uint32_t {
        avifDiagnosticsPrintf(
            &mut (*encoder).diag as *mut avifDiagnostics,
            b"avifEncoder.width and avifEncoder.height must either both be zero or both be nonzero\0"
                as *const u8 as *const core::ffi::c_char,
        );
        return AVIF_RESULT_INVALID_ARGUMENT;
    }
    if gridCols > 1 as uint32_t || gridRows > 1 as uint32_t {
        avifDiagnosticsPrintf(
            &mut (*encoder).diag as *mut avifDiagnostics,
            b"avifEncoder.width/height cannot be set with grid images\0" as *const u8
                as *const core::ffi::c_char,
        );
        return AVIF_RESULT_NOT_IMPLEMENTED;
    }
    if avifScalingModeIsNoScaling(&mut (*encoder).scalingMode) == 0 {
        avifDiagnosticsPrintf(
            &mut (*encoder).diag as *mut avifDiagnostics,
            b"avifEncoder.width/height cannot be set together with encoder->scalingMode\0"
                as *const u8 as *const core::ffi::c_char,
        );
        return AVIF_RESULT_INVALID_ARGUMENT;
    }
    if (*encoder).sampleTransformRecipe as core::ffi::c_uint
        != AVIF_SAMPLE_TRANSFORM_NONE as core::ffi::c_int as core::ffi::c_uint
    {
        avifDiagnosticsPrintf(
            &mut (*encoder).diag as *mut avifDiagnostics,
            b"avifEncoder.width/height cannot be set with sample transforms\0"
                as *const u8 as *const core::ffi::c_char,
        );
        return AVIF_RESULT_NOT_IMPLEMENTED;
    }
    if !((*firstCell).gainMap).is_null() && !((*(*firstCell).gainMap).image).is_null() {
        avifDiagnosticsPrintf(
            &mut (*encoder).diag as *mut avifDiagnostics,
            b"avifEncoder.width/height cannot be set with gain maps\0" as *const u8
                as *const core::ffi::c_char,
        );
        return AVIF_RESULT_NOT_IMPLEMENTED;
    }
    if (*encoder).extraLayerCount == 0 as uint32_t {
        avifDiagnosticsPrintf(
            &mut (*encoder).diag as *mut avifDiagnostics,
            b"avifEncoder.width/height can only be set for layered images (extraLayerCount > 0)\0"
                as *const u8 as *const core::ffi::c_char,
        );
        return AVIF_RESULT_INVALID_ARGUMENT;
    }
    if (*encoder).width < (*firstCell).width || (*encoder).height < (*firstCell).height {
        avifDiagnosticsPrintf(
            &mut (*encoder).diag as *mut avifDiagnostics,
            b"avifEncoder.width/height %ux%u must be at least the coded image size %ux%u\0"
                as *const u8 as *const core::ffi::c_char,
            (*encoder).width,
            (*encoder).height,
            (*firstCell).width,
            (*firstCell).height,
        );
        return AVIF_RESULT_INCOMPATIBLE_IMAGE;
    }
    if (*(*encoder).data).frames.count == (*encoder).extraLayerCount
        && ((*encoder).width != (*firstCell).width
            || (*encoder).height != (*firstCell).height)
    {
        avifDiagnosticsPrintf(
            &mut (*encoder).diag as *mut avifDiagnostics,
            b"avifEncoder.width/height %ux%u must exactly match the coded size %ux%u of the last layer\0"
                as *const u8 as *const core::ffi::c_char,
            (*encoder).width,
            (*encoder).height,
            (*firstCell).width,
            (*firstCell).height,
        );
        return AVIF_RESULT_INCOMPATIBLE_IMAGE;
    }
    if (*firstCell).transformFlags
        & AVIF_TRANSFORM_CLAP as core::ffi::c_int as avifTransformFlags != 0
    {
        let mut cropRect: avifCropRect = avifCropRect {
            x: 0,
            y: 0,
            width: 0,
            height: 0,
        };
        if avifCropRectFromCleanApertureBox(
            &mut cropRect,
            &(*firstCell).clap,
            (*encoder).width,
            (*encoder).height,
            &mut (*encoder).diag,
        ) == 0
        {
            return AVIF_RESULT_INVALID_ARGUMENT;
        }
    }
    if (*(*encoder).data).items.count > 0 as uint32_t
        && (*encoder).extraLayerCount > 0 as uint32_t
        && avifImageHasEquivalentTransformProperties(
            firstCell,
            (*(*encoder).data).imageMetadata,
        ) == 0
    {
        avifDiagnosticsPrintf(
            &mut (*encoder).diag as *mut avifDiagnostics,
            b"When avifEncoder.width/height is set, 'pasp', 'clap', 'irot' and 'imir' must match across layers\0"
                as *const u8 as *const core::ffi::c_char,
        );
        return AVIF_RESULT_INCOMPATIBLE_IMAGE;
    }
    return AVIF_RESULT_OK;
}
unsafe extern "C" fn avifEncoderDataShouldForceKeyframeForAlpha(
    mut data: *const avifEncoderData,
    mut colorItem: *const avifEncoderItem,
    mut addImageFlags: avifAddImageFlags,
) -> avifBool {
    if (*data).alphaPresent == 0 {
        return AVIF_FALSE;
    }
    if addImageFlags
        & AVIF_ADD_IMAGE_FLAG_SINGLE as core::ffi::c_int as avifAddImageFlags != 0
    {
        return AVIF_FALSE;
    }
    if (*data).frames.count == 0 as uint32_t {
        return AVIF_FALSE;
    }
    let colorFramesOutputSoFar: uint32_t = (*(*colorItem).encodeOutput).samples.count;
    let isLaggedOutput: avifBool = (((*data).frames.count).wrapping_add(1 as uint32_t)
        != colorFramesOutputSoFar) as core::ffi::c_int;
    if isLaggedOutput != 0 {
        return AVIF_FALSE;
    }
    return (*((*(*colorItem).encodeOutput).samples.sample)
        .offset(colorFramesOutputSoFar.wrapping_sub(1 as uint32_t) as isize))
        .sync;
}
unsafe extern "C" fn avifGetErrorForItemCategory(
    mut itemCategory: avifItemCategory,
) -> avifResult {
    if itemCategory as core::ffi::c_uint
        == AVIF_ITEM_GAIN_MAP as core::ffi::c_int as core::ffi::c_uint
    {
        return AVIF_RESULT_ENCODE_GAIN_MAP_FAILED;
    }
    if itemCategory as core::ffi::c_uint
        == AVIF_ITEM_SAMPLE_TRANSFORM as core::ffi::c_int as core::ffi::c_uint
        || itemCategory as core::ffi::c_uint
            >= AVIF_ITEM_SAMPLE_TRANSFORM_INPUT_0_COLOR as core::ffi::c_int
                as core::ffi::c_uint
            && itemCategory as core::ffi::c_uint
                <= (AVIF_ITEM_SAMPLE_TRANSFORM_INPUT_0_ALPHA as core::ffi::c_int
                    + (AVIF_ITEM_SAMPLE_TRANSFORM_INPUT_0_ALPHA as core::ffi::c_int
                        - AVIF_ITEM_SAMPLE_TRANSFORM_INPUT_0_COLOR as core::ffi::c_int)
                    - 1 as core::ffi::c_int) as core::ffi::c_uint
    {
        return AVIF_RESULT_ENCODE_SAMPLE_TRANSFORM_FAILED;
    }
    return (if avifIsAlpha(itemCategory) != 0 {
        AVIF_RESULT_ENCODE_ALPHA_FAILED as core::ffi::c_int
    } else {
        AVIF_RESULT_ENCODE_COLOR_FAILED as core::ffi::c_int
    }) as avifResult;
}
unsafe extern "C" fn avifGridWidth(
    mut gridCols: uint32_t,
    mut firstCell: *const avifImage,
    mut bottomRightCell: *const avifImage,
) -> uint32_t {
    return gridCols
        .wrapping_sub(1 as uint32_t)
        .wrapping_mul((*firstCell).width)
        .wrapping_add((*bottomRightCell).width);
}
unsafe extern "C" fn avifGridHeight(
    mut gridRows: uint32_t,
    mut firstCell: *const avifImage,
    mut bottomRightCell: *const avifImage,
) -> uint32_t {
    return gridRows
        .wrapping_sub(1 as uint32_t)
        .wrapping_mul((*firstCell).height)
        .wrapping_add((*bottomRightCell).height);
}
unsafe extern "C" fn avifValidateGrid(
    mut gridCols: uint32_t,
    mut gridRows: uint32_t,
    mut cellImages: *const *const avifImage,
    mut validateGainMap: avifBool,
    mut diag: *mut avifDiagnostics,
) -> avifResult {
    let cellCount: uint32_t = gridCols.wrapping_mul(gridRows);
    let mut firstCell: *const avifImage = *cellImages
        .offset(0 as core::ffi::c_int as isize);
    let mut bottomRightCell: *const avifImage = *cellImages
        .offset(cellCount.wrapping_sub(1 as uint32_t) as isize);
    if validateGainMap != 0 {
        if !(!((*firstCell).gainMap).is_null()
            && !((*(*firstCell).gainMap).image).is_null())
        {
            return AVIF_RESULT_INTERNAL_ERROR;
        }
        firstCell = (*(*firstCell).gainMap).image;
        if !(!((*bottomRightCell).gainMap).is_null()
            && !((*(*bottomRightCell).gainMap).image).is_null())
        {
            return AVIF_RESULT_INTERNAL_ERROR;
        }
        bottomRightCell = (*(*bottomRightCell).gainMap).image;
    }
    let tileWidth: uint32_t = (*firstCell).width;
    let tileHeight: uint32_t = (*firstCell).height;
    if tileWidth > 65536 as uint32_t || tileHeight > 65536 as uint32_t {
        avifDiagnosticsPrintf(
            diag,
            b"the first %s cell has invalid dimensions for AV1 and AV2: %ux%u\0"
                as *const u8 as *const core::ffi::c_char,
            if validateGainMap != 0 {
                b"gain map\0" as *const u8 as *const core::ffi::c_char
            } else {
                b"image\0" as *const u8 as *const core::ffi::c_char
            },
            tileWidth,
            tileHeight,
        );
        return AVIF_RESULT_INVALID_ARGUMENT;
    }
    let mut cellIndex: uint32_t = 0 as uint32_t;
    while cellIndex < cellCount {
        let mut cellImage: *const avifImage = *cellImages.offset(cellIndex as isize);
        if validateGainMap != 0 {
            if !(!((*cellImage).gainMap).is_null()
                && !((*(*cellImage).gainMap).image).is_null())
            {
                return AVIF_RESULT_INTERNAL_ERROR;
            }
            cellImage = (*(*cellImage).gainMap).image;
        }
        let expectedCellWidth: uint32_t = if cellIndex
            .wrapping_add(1 as uint32_t)
            .wrapping_rem(gridCols) != 0
        {
            tileWidth
        } else {
            (*bottomRightCell).width
        };
        let expectedCellHeight: uint32_t = if cellIndex
            < cellCount.wrapping_sub(gridCols)
        {
            tileHeight
        } else {
            (*bottomRightCell).height
        };
        if (*cellImage).width != expectedCellWidth
            || (*cellImage).height != expectedCellHeight
        {
            avifDiagnosticsPrintf(
                diag,
                b"%s cell %u has invalid dimensions: expected %ux%u found %ux%u\0"
                    as *const u8 as *const core::ffi::c_char,
                if validateGainMap != 0 {
                    b"gain map\0" as *const u8 as *const core::ffi::c_char
                } else {
                    b"image\0" as *const u8 as *const core::ffi::c_char
                },
                cellIndex,
                expectedCellWidth,
                expectedCellHeight,
                (*cellImage).width,
                (*cellImage).height,
            );
            return AVIF_RESULT_INVALID_IMAGE_GRID;
        }
        if (*cellImage).depth != (*firstCell).depth
            || (*cellImage).yuvFormat as core::ffi::c_uint
                != (*firstCell).yuvFormat as core::ffi::c_uint
            || (*cellImage).yuvRange as core::ffi::c_uint
                != (*firstCell).yuvRange as core::ffi::c_uint
            || (*cellImage).colorPrimaries as core::ffi::c_int
                != (*firstCell).colorPrimaries as core::ffi::c_int
            || (*cellImage).transferCharacteristics as core::ffi::c_int
                != (*firstCell).transferCharacteristics as core::ffi::c_int
            || (*cellImage).matrixCoefficients as core::ffi::c_int
                != (*firstCell).matrixCoefficients as core::ffi::c_int
            || !((*cellImage).alphaPlane).is_null() as core::ffi::c_int
                != !((*firstCell).alphaPlane).is_null() as core::ffi::c_int
            || (*cellImage).alphaPremultiplied != (*firstCell).alphaPremultiplied
        {
            avifDiagnosticsPrintf(
                diag,
                b"all grid cells should have the same value for: depth, yuvFormat, yuvRange, colorPrimaries, transferCharacteristics, matrixCoefficients, alphaPlane presence, alphaPremultiplied\0"
                    as *const u8 as *const core::ffi::c_char,
            );
            return AVIF_RESULT_INVALID_IMAGE_GRID;
        }
        if (*cellImage).matrixCoefficients as core::ffi::c_int
            == AVIF_MATRIX_COEFFICIENTS_IDENTITY as core::ffi::c_int
            && (*cellImage).yuvFormat as core::ffi::c_uint
                != AVIF_PIXEL_FORMAT_YUV444 as core::ffi::c_int as core::ffi::c_uint
        {
            avifDiagnosticsPrintf(
                diag,
                b"subsampling must be 0 (4:4:4) with identity matrix coefficients\0"
                    as *const u8 as *const core::ffi::c_char,
            );
            return AVIF_RESULT_INVALID_ARGUMENT;
        }
        if ((*cellImage).yuvPlanes[AVIF_CHAN_Y as core::ffi::c_int as usize]).is_null() {
            return AVIF_RESULT_NO_CONTENT;
        }
        cellIndex = cellIndex.wrapping_add(1);
    }
    if (*bottomRightCell).width > tileWidth || (*bottomRightCell).height > tileHeight {
        avifDiagnosticsPrintf(
            diag,
            b"the last %s cell can be smaller but not larger than the other cells which are %ux%u, found %ux%u\0"
                as *const u8 as *const core::ffi::c_char,
            if validateGainMap != 0 {
                b"gain map\0" as *const u8 as *const core::ffi::c_char
            } else {
                b"image\0" as *const u8 as *const core::ffi::c_char
            },
            tileWidth,
            tileHeight,
            (*bottomRightCell).width,
            (*bottomRightCell).height,
        );
        return AVIF_RESULT_INVALID_IMAGE_GRID;
    }
    let gridWidth: uint32_t = avifGridWidth(gridCols, firstCell, bottomRightCell)
        as uint32_t;
    let gridHeight: uint32_t = avifGridHeight(gridRows, firstCell, bottomRightCell)
        as uint32_t;
    if cellCount > 1 as uint32_t
        && avifAreGridDimensionsValid(
            (*firstCell).yuvFormat,
            gridWidth,
            gridHeight,
            tileWidth,
            tileHeight,
            diag,
        ) == 0
    {
        return AVIF_RESULT_INVALID_IMAGE_GRID;
    }
    return AVIF_RESULT_OK;
}
unsafe extern "C" fn avifEncoderAddImageInternal(
    mut encoder: *mut avifEncoder,
    mut gridCols: uint32_t,
    mut gridRows: uint32_t,
    mut cellImages: *const *const avifImage,
    mut durationInTimescales: uint64_t,
    mut addImageFlags: avifAddImageFlags,
) -> avifResult {
    if (avifCodecName(
        (*encoder).codecChoice,
        AVIF_CODEC_FLAG_CAN_ENCODE as core::ffi::c_int as avifCodecFlags,
    ))
        .is_null()
    {
        return AVIF_RESULT_NO_CODEC_AVAILABLE;
    }
    if (*encoder).extraLayerCount >= AVIF_MAX_AV1_LAYER_COUNT as uint32_t {
        avifDiagnosticsPrintf(
            &mut (*encoder).diag as *mut avifDiagnostics,
            b"extraLayerCount [%u] must be less than %d\0" as *const u8
                as *const core::ffi::c_char,
            (*encoder).extraLayerCount,
            AVIF_MAX_AV1_LAYER_COUNT,
        );
        return AVIF_RESULT_INVALID_ARGUMENT;
    }
    let cellCount: uint32_t = gridCols.wrapping_mul(gridRows);
    if cellCount == 0 as uint32_t {
        return AVIF_RESULT_INVALID_ARGUMENT;
    }
    let mut firstCell: *const avifImage = *cellImages
        .offset(0 as core::ffi::c_int as isize);
    let mut bottomRightCell: *const avifImage = *cellImages
        .offset(cellCount.wrapping_sub(1 as uint32_t) as isize);
    if !((*firstCell).depth == 8 as uint32_t || (*firstCell).depth == 10 as uint32_t
        || (*firstCell).depth == 12 as uint32_t
        || (*firstCell).depth == 16 as uint32_t
            && (*encoder).sampleTransformRecipe as core::ffi::c_uint
                != AVIF_SAMPLE_TRANSFORM_NONE as core::ffi::c_int as core::ffi::c_uint)
    {
        return AVIF_RESULT_UNSUPPORTED_DEPTH;
    }
    if !((*firstCell).yuvFormat as core::ffi::c_uint
        != AVIF_PIXEL_FORMAT_NONE as core::ffi::c_int as core::ffi::c_uint)
    {
        return AVIF_RESULT_NO_YUV_FORMAT_SELECTED;
    }
    if (*firstCell).width == 0 || (*firstCell).height == 0
        || (*bottomRightCell).width == 0 || (*bottomRightCell).height == 0
    {
        return AVIF_RESULT_NO_CONTENT;
    }
    let result__: avifResult = avifEncoderValidateSize(
        encoder,
        gridCols,
        gridRows,
        firstCell,
    ) as avifResult;
    if result__ as core::ffi::c_uint
        != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
    {
        return result__;
    }
    let result___0: avifResult = avifValidateGrid(
        gridCols,
        gridRows,
        cellImages,
        0 as avifBool,
        &mut (*encoder).diag,
    ) as avifResult;
    if result___0 as core::ffi::c_uint
        != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
    {
        return result___0;
    }
    let hasGainMap: avifBool = (!((*firstCell).gainMap).is_null()
        && !((*(*firstCell).gainMap).image).is_null()) as core::ffi::c_int;
    let mut cellIndex: uint32_t = 0 as uint32_t;
    while cellIndex < cellCount {
        let mut cellImage: *const avifImage = *cellImages.offset(cellIndex as isize);
        let cellHasGainMap: avifBool = (!((*cellImage).gainMap).is_null()
            && !((*(*cellImage).gainMap).image).is_null()) as core::ffi::c_int;
        if cellHasGainMap != hasGainMap {
            avifDiagnosticsPrintf(
                &mut (*encoder).diag as *mut avifDiagnostics,
                b"cells should either all have a gain map image, or none of them should, found a mix\0"
                    as *const u8 as *const core::ffi::c_char,
            );
            return AVIF_RESULT_INVALID_IMAGE_GRID;
        }
        if hasGainMap != 0 {
            if avifSameGainMapAltMetadata((*firstCell).gainMap, (*cellImage).gainMap)
                == 0
            {
                avifDiagnosticsPrintf(
                    &mut (*encoder).diag as *mut avifDiagnostics,
                    b"all cells should have the same alternate image metadata in the gain map\0"
                        as *const u8 as *const core::ffi::c_char,
                );
                return AVIF_RESULT_INVALID_IMAGE_GRID;
            }
            if avifSameGainMapMetadata((*firstCell).gainMap, (*cellImage).gainMap) == 0 {
                avifDiagnosticsPrintf(
                    &mut (*encoder).diag as *mut avifDiagnostics,
                    b"all cells should have the same gain map metadata\0" as *const u8
                        as *const core::ffi::c_char,
                );
                return AVIF_RESULT_INVALID_IMAGE_GRID;
            }
        }
        cellIndex = cellIndex.wrapping_add(1);
    }
    if hasGainMap != 0 {
        if !((*(*(*firstCell).gainMap).image).depth == 8 as uint32_t
            || (*(*(*firstCell).gainMap).image).depth == 10 as uint32_t
            || (*(*(*firstCell).gainMap).image).depth == 12 as uint32_t)
        {
            return AVIF_RESULT_UNSUPPORTED_DEPTH;
        }
        if !((*(*(*firstCell).gainMap).image).yuvFormat as core::ffi::c_uint
            != AVIF_PIXEL_FORMAT_NONE as core::ffi::c_int as core::ffi::c_uint)
        {
            return AVIF_RESULT_NO_YUV_FORMAT_SELECTED;
        }
        let result___1: avifResult = avifValidateGrid(
            gridCols,
            gridRows,
            cellImages,
            1 as avifBool,
            &mut (*encoder).diag,
        ) as avifResult;
        if result___1 as core::ffi::c_uint
            != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
        {
            return result___1;
        }
        if (*(*(*firstCell).gainMap).image).colorPrimaries as core::ffi::c_int
            != AVIF_COLOR_PRIMARIES_UNSPECIFIED as core::ffi::c_int
            || (*(*(*firstCell).gainMap).image).transferCharacteristics
                as core::ffi::c_int
                != AVIF_TRANSFER_CHARACTERISTICS_UNSPECIFIED as core::ffi::c_int
        {
            avifDiagnosticsPrintf(
                &mut (*encoder).diag as *mut avifDiagnostics,
                b"the gain map image must have colorPrimaries = 2 and transferCharacteristics = 2\0"
                    as *const u8 as *const core::ffi::c_char,
            );
            return AVIF_RESULT_INVALID_ARGUMENT;
        }
    }
    if (*(*encoder).data).singleImage != 0 {
        return AVIF_RESULT_ENCODE_COLOR_FAILED;
    }
    if addImageFlags
        & AVIF_ADD_IMAGE_FLAG_SINGLE as core::ffi::c_int as avifAddImageFlags != 0
    {
        (*(*encoder).data).singleImage = AVIF_TRUE as avifBool;
        if (*encoder).extraLayerCount > 0 as uint32_t {
            return AVIF_RESULT_INVALID_ARGUMENT;
        }
        if (*(*encoder).data).items.count > 0 as uint32_t {
            return AVIF_RESULT_INVALID_ARGUMENT;
        }
    }
    let codecType: avifCodecType = avifEncoderGetCodecType(encoder) as avifCodecType;
    match codecType as core::ffi::c_uint {
        1 => {
            (*(*encoder).data).imageItemType = b"av01\0" as *const u8
                as *const core::ffi::c_char;
            (*(*encoder).data).configPropName = b"av1C\0" as *const u8
                as *const core::ffi::c_char;
        }
        _ => return AVIF_RESULT_NO_CODEC_AVAILABLE,
    }
    (*(*encoder).data).quality = avifGetQuality(
        (*encoder).quality,
        (*encoder).minQuantizer,
        (*encoder).maxQuantizer,
    );
    if (*encoder).qualityAlpha == AVIF_QUALITY_DEFAULT
        && (*encoder).minQuantizerAlpha == AVIF_QUANTIZER_BEST_QUALITY
        && (*encoder).maxQuantizerAlpha == AVIF_QUANTIZER_WORST_QUALITY
    {
        (*(*encoder).data).qualityAlpha = (*(*encoder).data).quality;
    } else {
        (*(*encoder).data).qualityAlpha = avifGetQuality(
            (*encoder).qualityAlpha,
            (*encoder).minQuantizerAlpha,
            (*encoder).maxQuantizerAlpha,
        );
    }
    if (*encoder).qualityGainMap == AVIF_QUALITY_DEFAULT {
        (*(*encoder).data).qualityGainMap = (*(*encoder).data).quality;
    } else {
        (*(*encoder).data).qualityGainMap = avifGetQuality(
            (*encoder).qualityGainMap,
            AVIF_QUANTIZER_BEST_QUALITY,
            AVIF_QUANTIZER_WORST_QUALITY,
        );
    }
    (*(*encoder).data).tileRowsLog2 = if (*encoder).tileRowsLog2 < 0 as core::ffi::c_int
    {
        0 as core::ffi::c_int
    } else if (6 as core::ffi::c_int) < (*encoder).tileRowsLog2 {
        6 as core::ffi::c_int
    } else {
        (*encoder).tileRowsLog2
    };
    (*(*encoder).data).tileColsLog2 = if (*encoder).tileColsLog2 < 0 as core::ffi::c_int
    {
        0 as core::ffi::c_int
    } else if (6 as core::ffi::c_int) < (*encoder).tileColsLog2 {
        6 as core::ffi::c_int
    } else {
        (*encoder).tileColsLog2
    };
    if (*encoder).autoTiling != 0 {
        let threads: core::ffi::c_int = 8 as core::ffi::c_int;
        avifSetTileConfiguration(
            threads,
            (*firstCell).width,
            (*firstCell).height,
            &mut (*(*encoder).data).tileRowsLog2,
            &mut (*(*encoder).data).tileColsLog2,
        );
    }
    let mut encoderChanges: avifEncoderChanges = 0;
    if avifEncoderDetectChanges(encoder, &mut encoderChanges) == 0 {
        return AVIF_RESULT_CANNOT_CHANGE_SETTING;
    }
    avifEncoderBackupSettings(encoder);
    if durationInTimescales == 0 as uint64_t {
        durationInTimescales = 1 as uint64_t;
    }
    if (*(*encoder).data).items.count == 0 as uint32_t {
        let result___2: avifResult = avifImageCopy(
            (*(*encoder).data).imageMetadata,
            firstCell,
            0 as avifPlanesFlags,
        ) as avifResult;
        if result___2 as core::ffi::c_uint
            != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
        {
            return result___2;
        }
        if avifEncoderSizeIsSet(encoder) != 0 {
            (*(*(*encoder).data).imageMetadata).width = (*encoder).width;
            (*(*(*encoder).data).imageMetadata).height = (*encoder).height;
        }
        let gridWidth: uint32_t = avifGridWidth(gridCols, firstCell, bottomRightCell)
            as uint32_t;
        let gridHeight: uint32_t = avifGridHeight(gridRows, firstCell, bottomRightCell)
            as uint32_t;
        if hasGainMap != 0 {
            let result___3: avifResult = avifImageCopyAltImageMetadata(
                (*(*encoder).data).altImageMetadata,
                (*(*encoder).data).imageMetadata,
                gridWidth,
                gridHeight,
            ) as avifResult;
            if result___3 as core::ffi::c_uint
                != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
            {
                return result___3;
            }
        }
        let mut colorItemID: uint32_t = 0;
        let result___4: avifResult = avifEncoderAddImageItems(
            encoder,
            gridCols,
            gridRows,
            gridWidth,
            gridHeight,
            AVIF_ITEM_COLOR,
            &mut colorItemID,
        ) as avifResult;
        if result___4 as core::ffi::c_uint
            != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
        {
            return result___4;
        }
        (*(*encoder).data).primaryItemID = colorItemID;
        (*(*encoder).data).alphaPresent = ((*firstCell).alphaPlane
            != NULL as *mut uint8_t) as core::ffi::c_int as avifBool;
        if (*(*encoder).data).alphaPresent != 0
            && addImageFlags
                & AVIF_ADD_IMAGE_FLAG_SINGLE as core::ffi::c_int as avifAddImageFlags
                != 0
        {
            (*(*encoder).data).alphaPresent = AVIF_FALSE as avifBool;
            let mut cellIndex_0: uint32_t = 0 as uint32_t;
            while cellIndex_0 < cellCount {
                let mut cellImage_0: *const avifImage = *cellImages
                    .offset(cellIndex_0 as isize);
                if avifImageIsOpaque(cellImage_0) == 0 {
                    (*(*encoder).data).alphaPresent = AVIF_TRUE as avifBool;
                    break;
                } else {
                    cellIndex_0 = cellIndex_0.wrapping_add(1);
                }
            }
        }
        if (*(*encoder).data).alphaPresent != 0 {
            let mut alphaItemID: uint32_t = 0;
            let result___5: avifResult = avifEncoderAddImageItems(
                encoder,
                gridCols,
                gridRows,
                gridWidth,
                gridHeight,
                AVIF_ITEM_ALPHA,
                &mut alphaItemID,
            ) as avifResult;
            if result___5 as core::ffi::c_uint
                != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
            {
                return result___5;
            }
            let mut alphaItem: *mut avifEncoderItem = avifEncoderDataFindItemByID(
                (*encoder).data as *mut avifEncoderData,
                alphaItemID,
            );
            if alphaItem.is_null() {
                return AVIF_RESULT_INTERNAL_ERROR;
            }
            (*alphaItem).irefType = b"auxl\0" as *const u8 as *const core::ffi::c_char;
            (*alphaItem).irefToID = colorItemID;
            if (*(*(*encoder).data).imageMetadata).alphaPremultiplied != 0 {
                let mut colorItem: *mut avifEncoderItem = avifEncoderDataFindItemByID(
                    (*encoder).data as *mut avifEncoderData,
                    colorItemID,
                );
                if colorItem.is_null() {
                    return AVIF_RESULT_INTERNAL_ERROR;
                }
                (*colorItem).irefType = b"prem\0" as *const u8
                    as *const core::ffi::c_char;
                (*colorItem).irefToID = alphaItemID;
            }
        }
        if !((*firstCell).gainMap).is_null()
            && !((*(*firstCell).gainMap).image).is_null()
        {
            let mut toneMappedItem: *mut avifEncoderItem = 0 as *mut avifEncoderItem;
            let result___6: avifResult = avifEncoderDataCreateItem(
                (*encoder).data as *mut avifEncoderData,
                b"tmap\0" as *const u8 as *const core::ffi::c_char,
                core::ptr::addr_of!(infeNameGainMap).cast::<core::ffi::c_char>(),
                (strlen(core::ptr::addr_of!(infeNameGainMap).cast::<core::ffi::c_char>())).wrapping_add(1 as size_t),
                0 as uint32_t,
                &mut toneMappedItem,
            ) as avifResult;
            if result___6 as core::ffi::c_uint
                != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
            {
                return result___6;
            }
            let result___7: avifResult = avifWriteToneMappedImagePayload(
                &mut (*toneMappedItem).metadataPayload,
                (*firstCell).gainMap,
                &mut (*encoder).diag,
            ) as avifResult;
            if result___7 as core::ffi::c_uint
                != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
            {
                return result___7;
            }
            (*toneMappedItem).itemCategory = AVIF_ITEM_COLOR;
            let mut toneMappedItemID: uint32_t = (*toneMappedItem).id;
            if !((*(*encoder).data).alternativeItemIDs.count == 0 as uint32_t) {
                return AVIF_RESULT_INTERNAL_ERROR;
            }
            let mut alternativeItemID: *mut uint32_t = avifArrayPush(
                &mut (*(*encoder).data).alternativeItemIDs as *mut avifEncoderItemIdArray
                    as *mut core::ffi::c_void,
            ) as *mut uint32_t;
            if alternativeItemID.is_null() {
                return AVIF_RESULT_OUT_OF_MEMORY;
            }
            *alternativeItemID = toneMappedItemID;
            alternativeItemID = avifArrayPush(
                &mut (*(*encoder).data).alternativeItemIDs as *mut avifEncoderItemIdArray
                    as *mut core::ffi::c_void,
            ) as *mut uint32_t;
            if alternativeItemID.is_null() {
                return AVIF_RESULT_OUT_OF_MEMORY;
            }
            *alternativeItemID = colorItemID;
            let gainMapGridWidth: uint32_t = avifGridWidth(
                gridCols,
                (*(**cellImages.offset(0 as core::ffi::c_int as isize)).gainMap).image,
                (*(**cellImages
                    .offset(
                        gridCols.wrapping_mul(gridRows).wrapping_sub(1 as uint32_t)
                            as isize,
                    ))
                    .gainMap)
                    .image,
            ) as uint32_t;
            let gainMapGridHeight: uint32_t = avifGridHeight(
                gridRows,
                (*(**cellImages.offset(0 as core::ffi::c_int as isize)).gainMap).image,
                (*(**cellImages
                    .offset(
                        gridCols.wrapping_mul(gridRows).wrapping_sub(1 as uint32_t)
                            as isize,
                    ))
                    .gainMap)
                    .image,
            ) as uint32_t;
            let mut gainMapItemID: uint32_t = 0;
            let result___8: avifResult = avifEncoderAddImageItems(
                encoder,
                gridCols,
                gridRows,
                gainMapGridWidth,
                gainMapGridHeight,
                AVIF_ITEM_GAIN_MAP,
                &mut gainMapItemID,
            ) as avifResult;
            if result___8 as core::ffi::c_uint
                != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
            {
                return result___8;
            }
            let mut gainMapItem: *mut avifEncoderItem = avifEncoderDataFindItemByID(
                (*encoder).data as *mut avifEncoderData,
                gainMapItemID,
            );
            if gainMapItem.is_null() {
                return AVIF_RESULT_INTERNAL_ERROR;
            }
            (*gainMapItem).hiddenImage = AVIF_TRUE as avifBool;
            if !(colorItemID < gainMapItemID) {
                return AVIF_RESULT_INTERNAL_ERROR;
            }
            let mut colorItem_0: *mut avifEncoderItem = avifEncoderDataFindItemByID(
                (*encoder).data as *mut avifEncoderData,
                colorItemID,
            );
            if colorItem_0.is_null() {
                return AVIF_RESULT_INTERNAL_ERROR;
            }
            if !((*colorItem_0).dimgFromID == 0 as uint32_t) {
                return AVIF_RESULT_INTERNAL_ERROR;
            }
            (*colorItem_0).dimgFromID = toneMappedItemID;
            (*gainMapItem).dimgFromID = toneMappedItemID;
        }
        if (*encoder).sampleTransformRecipe as core::ffi::c_uint
            == AVIF_SAMPLE_TRANSFORM_BIT_DEPTH_EXTENSION_8B_8B as core::ffi::c_int
                as core::ffi::c_uint
            || (*encoder).sampleTransformRecipe as core::ffi::c_uint
                == AVIF_SAMPLE_TRANSFORM_BIT_DEPTH_EXTENSION_12B_4B as core::ffi::c_int
                    as core::ffi::c_uint
            || (*encoder).sampleTransformRecipe as core::ffi::c_uint
                == AVIF_SAMPLE_TRANSFORM_BIT_DEPTH_EXTENSION_12B_8B_OVERLAP_4B
                    as core::ffi::c_int as core::ffi::c_uint
        {
            if !((*firstCell).depth == 16 as uint32_t) {
                return AVIF_RESULT_NOT_IMPLEMENTED;
            }
            if !((*firstCell).gainMap).is_null() {
                return AVIF_RESULT_NOT_IMPLEMENTED;
            }
            let result___9: avifResult = avifEncoderCreateBitDepthExtensionItems(
                encoder,
                gridCols,
                gridRows,
                gridWidth,
                gridHeight,
                colorItemID,
            ) as avifResult;
            if result___9 as core::ffi::c_uint
                != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
            {
                return result___9;
            }
        } else if !((*encoder).sampleTransformRecipe as core::ffi::c_uint
            == AVIF_SAMPLE_TRANSFORM_NONE as core::ffi::c_int as core::ffi::c_uint)
        {
            return AVIF_RESULT_NOT_IMPLEMENTED
        }
        if (*firstCell).exif.size > 0 as size_t {
            let result: avifResult = avifEncoderDataCreateExifItem(
                (*encoder).data as *mut avifEncoderData,
                &(*firstCell).exif,
            ) as avifResult;
            if result as core::ffi::c_uint
                != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
            {
                return result;
            }
        }
        if (*firstCell).xmp.size > 0 as size_t {
            let result_0: avifResult = avifEncoderDataCreateXMPItem(
                (*encoder).data as *mut avifEncoderData,
                &(*firstCell).xmp,
            ) as avifResult;
            if result_0 as core::ffi::c_uint
                != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
            {
                return result_0;
            }
        }
    } else {
        if hasGainMap != 0 {
            avifDiagnosticsPrintf(
                &mut (*encoder).diag as *mut avifDiagnostics,
                b"gain maps are not supported for image sequences or layered images\0"
                    as *const u8 as *const core::ffi::c_char,
            );
            return AVIF_RESULT_NOT_IMPLEMENTED;
        }
        let mut imageMetadata: *const avifImage = (*(*encoder).data).imageMetadata;
        if (*imageMetadata).depth != (*firstCell).depth
            || (*imageMetadata).yuvFormat as core::ffi::c_uint
                != (*firstCell).yuvFormat as core::ffi::c_uint
            || (*imageMetadata).yuvRange as core::ffi::c_uint
                != (*firstCell).yuvRange as core::ffi::c_uint
            || (*imageMetadata).yuvChromaSamplePosition as core::ffi::c_uint
                != (*firstCell).yuvChromaSamplePosition as core::ffi::c_uint
            || (*imageMetadata).colorPrimaries as core::ffi::c_int
                != (*firstCell).colorPrimaries as core::ffi::c_int
            || (*imageMetadata).transferCharacteristics as core::ffi::c_int
                != (*firstCell).transferCharacteristics as core::ffi::c_int
            || (*imageMetadata).matrixCoefficients as core::ffi::c_int
                != (*firstCell).matrixCoefficients as core::ffi::c_int
            || (*imageMetadata).alphaPremultiplied != (*firstCell).alphaPremultiplied
            || (*(*encoder).data).alphaPresent != 0
                && ((*firstCell).alphaPlane).is_null()
        {
            return AVIF_RESULT_INCOMPATIBLE_IMAGE;
        }
    }
    if (*(*encoder).data).frames.count == 1 as uint32_t {
        if !((*(*encoder).data).items.count > 0 as uint32_t) {
            return AVIF_RESULT_INTERNAL_ERROR;
        }
        let mut imageMetadata_0: *const avifImage = (*(*encoder).data).imageMetadata;
        if !((*imageMetadata_0).width <= 65535 as uint32_t
            && (*imageMetadata_0).height <= 65535 as uint32_t)
        {
            return AVIF_RESULT_INVALID_ARGUMENT;
        }
    }
    let mut itemIndex: uint32_t = 0 as uint32_t;
    while itemIndex < (*(*encoder).data).items.count {
        let mut item: *mut avifEncoderItem = &mut *((*(*encoder).data).items.item)
            .offset(itemIndex as isize) as *mut avifEncoderItem;
        if !((*item).codec).is_null() {
            let mut cellImage_1: *const avifImage = *cellImages
                .offset((*item).cellIndex as isize);
            let mut cellImagePlaceholder: *mut avifImage = 0 as *mut avifImage;
            let mut firstCellImage: *const avifImage = firstCell;
            if (*item).itemCategory as core::ffi::c_uint
                == AVIF_ITEM_GAIN_MAP as core::ffi::c_int as core::ffi::c_uint
            {
                if !(!((*cellImage_1).gainMap).is_null()
                    && !((*(*cellImage_1).gainMap).image).is_null())
                {
                    return AVIF_RESULT_INTERNAL_ERROR;
                }
                cellImage_1 = (*(*cellImage_1).gainMap).image;
                if !(!((*firstCell).gainMap).is_null()
                    && !((*(*firstCell).gainMap).image).is_null())
                {
                    return AVIF_RESULT_INTERNAL_ERROR;
                }
                firstCellImage = (*(*firstCell).gainMap).image;
            }
            if (*cellImage_1).width != (*firstCellImage).width
                || (*cellImage_1).height != (*firstCellImage).height
            {
                cellImagePlaceholder = avifImageCreateEmpty();
                if cellImagePlaceholder.is_null() {
                    return AVIF_RESULT_OUT_OF_MEMORY;
                }
                let result_1: avifResult = avifImageCopyAndPad(
                    cellImagePlaceholder,
                    cellImage_1,
                    (*firstCellImage).width,
                    (*firstCellImage).height,
                ) as avifResult;
                if result_1 as core::ffi::c_uint
                    != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
                {
                    avifImageDestroy(cellImagePlaceholder);
                    return result_1;
                }
                cellImage_1 = cellImagePlaceholder;
            }
            let isAlpha: avifBool = avifIsAlpha((*item).itemCategory) as avifBool;
            let mut quality: core::ffi::c_int = if isAlpha != 0 {
                (*(*encoder).data).qualityAlpha
            } else if (*item).itemCategory as core::ffi::c_uint
                == AVIF_ITEM_GAIN_MAP as core::ffi::c_int as core::ffi::c_uint
            {
                (*(*encoder).data).qualityGainMap
            } else {
                (*(*encoder).data).quality
            };
            let mut encoderMinQuantizer: *mut core::ffi::c_int = if isAlpha != 0 {
                &mut (*encoder).minQuantizerAlpha
            } else {
                &mut (*encoder).minQuantizer
            };
            let mut encoderMaxQuantizer: *mut core::ffi::c_int = if isAlpha != 0 {
                &mut (*encoder).maxQuantizerAlpha
            } else {
                &mut (*encoder).maxQuantizer
            };
            let originalMinQuantizer: core::ffi::c_int = *encoderMinQuantizer;
            let originalMaxQuantizer: core::ffi::c_int = *encoderMaxQuantizer;
            if (*encoder).sampleTransformRecipe as core::ffi::c_uint
                != AVIF_SAMPLE_TRANSFORM_NONE as core::ffi::c_int as core::ffi::c_uint
            {
                if ((*encoder).sampleTransformRecipe as core::ffi::c_uint
                    == AVIF_SAMPLE_TRANSFORM_BIT_DEPTH_EXTENSION_8B_8B
                        as core::ffi::c_int as core::ffi::c_uint
                    || (*encoder).sampleTransformRecipe as core::ffi::c_uint
                        == AVIF_SAMPLE_TRANSFORM_BIT_DEPTH_EXTENSION_12B_4B
                            as core::ffi::c_int as core::ffi::c_uint)
                    && ((*item).itemCategory as core::ffi::c_uint
                        == AVIF_ITEM_COLOR as core::ffi::c_int as core::ffi::c_uint
                        || (*item).itemCategory as core::ffi::c_uint
                            == AVIF_ITEM_ALPHA as core::ffi::c_int as core::ffi::c_uint)
                {
                    quality = AVIF_QUALITY_LOSSLESS;
                    *encoderMinQuantizer = AVIF_QUANTIZER_LOSSLESS;
                    *encoderMaxQuantizer = AVIF_QUANTIZER_LOSSLESS;
                    avifEncoderDetectChanges(encoder, &mut encoderChanges) == 0;
                }
                let itemWillBeEncodedLosslessly: avifBool = (quality
                    == AVIF_QUALITY_LOSSLESS) as core::ffi::c_int;
                let mut sampleTransformedImage: *mut avifImage = 0 as *mut avifImage;
                if !cellImagePlaceholder.is_null() {
                    avifImageDestroy(cellImagePlaceholder);
                    cellImagePlaceholder = 0 as *mut avifImage;
                }
                let result___10: avifResult = avifEncoderCreateBitDepthExtensionImage(
                    encoder,
                    item,
                    itemWillBeEncodedLosslessly,
                    cellImage_1,
                    &mut sampleTransformedImage,
                ) as avifResult;
                if result___10 as core::ffi::c_uint
                    != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
                {
                    return result___10;
                }
                cellImagePlaceholder = sampleTransformedImage;
                cellImage_1 = cellImagePlaceholder;
            }
            let mut encodeResult: avifResult = ((*(*item).codec).encodeImage)
                .expect(
                    "non-null function pointer",
                )(
                (*item).codec as *mut avifCodec,
                encoder,
                cellImage_1,
                isAlpha,
                (*(*encoder).data).tileRowsLog2,
                (*(*encoder).data).tileColsLog2,
                quality,
                encoderChanges,
                (*(*encoder).data).alphaPresent,
                addImageFlags,
                (*item).encodeOutput,
            );
            if *encoderMinQuantizer != originalMinQuantizer
                || *encoderMaxQuantizer != originalMaxQuantizer
            {
                avifEncoderBackupSettings(encoder);
                *encoderMinQuantizer = originalMinQuantizer;
                *encoderMaxQuantizer = originalMaxQuantizer;
            }
            if !cellImagePlaceholder.is_null() {
                avifImageDestroy(cellImagePlaceholder);
            }
            if encodeResult as core::ffi::c_uint
                == AVIF_RESULT_UNKNOWN_ERROR as core::ffi::c_int as core::ffi::c_uint
            {
                encodeResult = avifGetErrorForItemCategory((*item).itemCategory);
            }
            let result___11: avifResult = encodeResult;
            if result___11 as core::ffi::c_uint
                != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
            {
                return result___11;
            }
            if itemIndex == 0 as uint32_t
                && avifEncoderDataShouldForceKeyframeForAlpha(
                    (*encoder).data,
                    item,
                    addImageFlags,
                ) != 0
            {
                addImageFlags
                    |= AVIF_ADD_IMAGE_FLAG_FORCE_KEYFRAME as core::ffi::c_int
                        as avifAddImageFlags;
            }
        }
        itemIndex = itemIndex.wrapping_add(1);
    }
    let mut frame: *mut avifEncoderFrame = avifArrayPush(
        &mut (*(*encoder).data).frames as *mut avifEncoderFrameArray
            as *mut core::ffi::c_void,
    ) as *mut avifEncoderFrame;
    if frame.is_null() {
        return AVIF_RESULT_OUT_OF_MEMORY;
    }
    (*frame).durationInTimescales = durationInTimescales;
    avifCodecSpecificOptionsClear((*encoder).csOptions as *mut avifCodecSpecificOptions);
    return AVIF_RESULT_OK;
}
#[no_mangle]
pub unsafe extern "C" fn avifEncoderAddImage(
    mut encoder: *mut avifEncoder,
    mut image: *const avifImage,
    mut durationInTimescales: uint64_t,
    mut addImageFlags: avifAddImageFlags,
) -> avifResult {
    avifDiagnosticsClearError(&mut (*encoder).diag);
    return avifEncoderAddImageInternal(
        encoder,
        1 as uint32_t,
        1 as uint32_t,
        &mut image,
        durationInTimescales,
        addImageFlags,
    );
}
#[no_mangle]
pub unsafe extern "C" fn avifEncoderAddImageGrid(
    mut encoder: *mut avifEncoder,
    mut gridCols: uint32_t,
    mut gridRows: uint32_t,
    mut cellImages: *const *const avifImage,
    mut addImageFlags: avifAddImageFlags,
) -> avifResult {
    avifDiagnosticsClearError(&mut (*encoder).diag);
    if gridCols == 0 as uint32_t || gridCols > 256 as uint32_t
        || gridRows == 0 as uint32_t || gridRows > 256 as uint32_t
        || gridCols.wrapping_mul(gridRows) > 65535 as uint32_t
    {
        return AVIF_RESULT_INVALID_IMAGE_GRID;
    }
    if gridCols.wrapping_mul(gridRows) > 65535 as uint32_t {
        avifDiagnosticsPrintf(
            &mut (*encoder).diag as *mut avifDiagnostics,
            b"avifEncoderAddImageGrid() failed because a grid cannot have more than 65535 cells due to the 16-bit 'dimg' reference_count field (%u columns x %u rows)\0"
                as *const u8 as *const core::ffi::c_char,
            gridCols,
            gridRows,
        );
        return AVIF_RESULT_INVALID_IMAGE_GRID;
    }
    if (*encoder).extraLayerCount == 0 as uint32_t {
        addImageFlags
            |= AVIF_ADD_IMAGE_FLAG_SINGLE as core::ffi::c_int as avifAddImageFlags;
    }
    return avifEncoderAddImageInternal(
        encoder,
        gridCols,
        gridRows,
        cellImages,
        1 as uint64_t,
        addImageFlags,
    );
}
unsafe extern "C" fn avifEncoderFindExistingChunk(
    mut s: *mut avifRWStream,
    mut mdatStartOffset: size_t,
    mut data: *const uint8_t,
    mut size: size_t,
) -> size_t {
    let mdatCurrentOffset: size_t = avifRWStreamOffset(s) as size_t;
    let mdatSearchSize: size_t = mdatCurrentOffset.wrapping_sub(mdatStartOffset);
    if mdatSearchSize < size {
        return 0 as size_t;
    }
    let mdatEndSearchOffset: size_t = mdatCurrentOffset.wrapping_sub(size);
    let mut searchOffset: size_t = mdatStartOffset;
    while searchOffset <= mdatEndSearchOffset {
        if memcmp(
            data as *const core::ffi::c_void,
            &mut *((*(*s).raw).data).offset(searchOffset as isize) as *mut uint8_t
                as *const core::ffi::c_void,
            size,
        ) == 0
        {
            return searchOffset;
        }
        searchOffset = searchOffset.wrapping_add(1);
    }
    return 0 as size_t;
}
unsafe extern "C" fn avifEncoderWriteMediaDataBox(
    mut encoder: *mut avifEncoder,
    mut s: *mut avifRWStream,
    mut layeredColorItems: *mut avifEncoderItemReferenceArray,
    mut layeredAlphaItems: *mut avifEncoderItemReferenceArray,
) -> avifResult {
    (*encoder).ioStats.colorOBUSize = 0 as size_t;
    (*encoder).ioStats.alphaOBUSize = 0 as size_t;
    (*(*encoder).data).gainMapSizeBytes = 0 as size_t;
    let mut mdat: avifBoxMarker = 0;
    let result__: avifResult = avifRWStreamWriteBox(
        s,
        b"mdat\0" as *const u8 as *const core::ffi::c_char,
        0 as size_t,
        &mut mdat,
    ) as avifResult;
    if result__ as core::ffi::c_uint
        != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
    {
        return result__;
    }
    let mdatStartOffset: size_t = avifRWStreamOffset(s) as size_t;
    let mut itemPasses: uint32_t = 0 as uint32_t;
    while itemPasses < 3 as uint32_t {
        let metadataPass: avifBool = (itemPasses == 0 as uint32_t) as core::ffi::c_int;
        let alphaAndGainMapPass: avifBool = (itemPasses == 1 as uint32_t)
            as core::ffi::c_int;
        let mut itemIndex: uint32_t = 0 as uint32_t;
        while itemIndex < (*(*encoder).data).items.count {
            let mut item: *mut avifEncoderItem = &mut *((*(*encoder).data).items.item)
                .offset(itemIndex as isize) as *mut avifEncoderItem;
            if !((*item).metadataPayload.size == 0 as size_t
                && (*(*item).encodeOutput).samples.count == 0 as uint32_t)
            {
                let isMetadata: avifBool = (memcmp(
                    ((*item).type_0).as_mut_ptr() as *const core::ffi::c_void,
                    b"mime\0" as *const u8 as *const core::ffi::c_char
                        as *const core::ffi::c_void,
                    4 as size_t,
                ) == 0
                    || memcmp(
                        ((*item).type_0).as_mut_ptr() as *const core::ffi::c_void,
                        b"Exif\0" as *const u8 as *const core::ffi::c_char
                            as *const core::ffi::c_void,
                        4 as size_t,
                    ) == 0
                    || memcmp(
                        ((*item).type_0).as_mut_ptr() as *const core::ffi::c_void,
                        b"tmap\0" as *const u8 as *const core::ffi::c_char
                            as *const core::ffi::c_void,
                        4 as size_t,
                    ) == 0) as core::ffi::c_int;
                if !(metadataPass != isMetadata) {
                    let isAlpha: avifBool = avifIsAlpha((*item).itemCategory)
                        as avifBool;
                    let isAlphaOrGainMap: avifBool = (isAlpha != 0
                        || (*item).itemCategory as core::ffi::c_uint
                            == AVIF_ITEM_GAIN_MAP as core::ffi::c_int
                                as core::ffi::c_uint) as core::ffi::c_int;
                    if !(alphaAndGainMapPass != isAlphaOrGainMap) {
                        if (*encoder).extraLayerCount > 0 as uint32_t
                            && (*(*item).encodeOutput).samples.count > 0 as uint32_t
                        {
                            if !((*(*item).encodeOutput).samples.count
                                == (*item).mdatFixups.count)
                            {
                                return AVIF_RESULT_INTERNAL_ERROR;
                            }
                            let mut ref_0: *mut avifEncoderItemReference = avifArrayPush(
                                (if isAlpha != 0 {
                                    layeredAlphaItems
                                } else {
                                    layeredColorItems
                                }) as *mut core::ffi::c_void,
                            ) as *mut avifEncoderItemReference;
                            if ref_0.is_null() {
                                return AVIF_RESULT_OUT_OF_MEMORY;
                            }
                            *ref_0 = item as avifEncoderItemReference;
                        } else {
                            let mut chunkOffset: size_t = 0 as size_t;
                            if (*(*item).encodeOutput).samples.count == 1 as uint32_t {
                                let mut sample: *mut avifEncodeSample = &mut *((*(*item)
                                    .encodeOutput)
                                    .samples
                                    .sample)
                                    .offset(0 as core::ffi::c_int as isize)
                                    as *mut avifEncodeSample;
                                chunkOffset = avifEncoderFindExistingChunk(
                                    s,
                                    mdatStartOffset,
                                    (*sample).data.data,
                                    (*sample).data.size,
                                );
                            } else if (*(*item).encodeOutput).samples.count
                                == 0 as uint32_t
                            {
                                chunkOffset = avifEncoderFindExistingChunk(
                                    s,
                                    mdatStartOffset,
                                    (*item).metadataPayload.data,
                                    (*item).metadataPayload.size,
                                );
                            }
                            if chunkOffset == 0 {
                                chunkOffset = avifRWStreamOffset(s);
                                if (*(*item).encodeOutput).samples.count > 0 as uint32_t {
                                    let mut sampleIndex: uint32_t = 0 as uint32_t;
                                    while sampleIndex < (*(*item).encodeOutput).samples.count {
                                        let mut sample_0: *mut avifEncodeSample = &mut *((*(*item)
                                            .encodeOutput)
                                            .samples
                                            .sample)
                                            .offset(sampleIndex as isize) as *mut avifEncodeSample;
                                        let result___0: avifResult = avifRWStreamWrite(
                                            s,
                                            (*sample_0).data.data as *const core::ffi::c_void,
                                            (*sample_0).data.size,
                                        ) as avifResult;
                                        if result___0 as core::ffi::c_uint
                                            != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
                                        {
                                            return result___0;
                                        }
                                        if isAlpha != 0 {
                                            (*encoder).ioStats.alphaOBUSize = ((*encoder)
                                                .ioStats
                                                .alphaOBUSize)
                                                .wrapping_add((*sample_0).data.size);
                                        } else if (*item).itemCategory as core::ffi::c_uint
                                            == AVIF_ITEM_COLOR as core::ffi::c_int as core::ffi::c_uint
                                        {
                                            (*encoder).ioStats.colorOBUSize = ((*encoder)
                                                .ioStats
                                                .colorOBUSize)
                                                .wrapping_add((*sample_0).data.size);
                                        } else if (*item).itemCategory as core::ffi::c_uint
                                            == AVIF_ITEM_GAIN_MAP as core::ffi::c_int
                                                as core::ffi::c_uint
                                        {
                                            (*(*encoder).data).gainMapSizeBytes = ((*(*encoder).data)
                                                .gainMapSizeBytes)
                                                .wrapping_add((*sample_0).data.size);
                                        }
                                        sampleIndex = sampleIndex.wrapping_add(1);
                                    }
                                } else {
                                    let result___1: avifResult = avifRWStreamWrite(
                                        s,
                                        (*item).metadataPayload.data as *const core::ffi::c_void,
                                        (*item).metadataPayload.size,
                                    ) as avifResult;
                                    if result___1 as core::ffi::c_uint
                                        != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
                                    {
                                        return result___1;
                                    }
                                }
                            }
                            let mut fixupIndex: uint32_t = 0 as uint32_t;
                            while fixupIndex < (*item).mdatFixups.count {
                                let mut fixup: *mut avifOffsetFixup = &mut *((*item)
                                    .mdatFixups
                                    .fixup)
                                    .offset(fixupIndex as isize) as *mut avifOffsetFixup;
                                let mut prevOffset: size_t = avifRWStreamOffset(s);
                                avifRWStreamSetOffset(s, (*fixup).offset);
                                let result___2: avifResult = avifRWStreamWriteU32(
                                    s,
                                    chunkOffset as uint32_t,
                                ) as avifResult;
                                if result___2 as core::ffi::c_uint
                                    != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
                                {
                                    return result___2;
                                }
                                avifRWStreamSetOffset(s, prevOffset);
                                fixupIndex = fixupIndex.wrapping_add(1);
                            }
                        }
                    }
                }
            }
            itemIndex = itemIndex.wrapping_add(1);
        }
        itemPasses = itemPasses.wrapping_add(1);
    }
    let mut layeredItemCount: uint32_t = if (*layeredColorItems).count
        > (*layeredAlphaItems).count
    {
        (*layeredColorItems).count
    } else {
        (*layeredAlphaItems).count
    };
    if layeredItemCount > 0 as uint32_t {
        let mut hasMoreSample: avifBool = 0;
        let mut layerIndex: uint32_t = 0 as uint32_t;
        loop {
            hasMoreSample = AVIF_FALSE as avifBool;
            let mut itemIndex_0: uint32_t = 0 as uint32_t;
            while itemIndex_0 < layeredItemCount {
                let mut samplePass: core::ffi::c_int = 0 as core::ffi::c_int;
                while samplePass < 2 as core::ffi::c_int {
                    let mut currentItems: *mut avifEncoderItemReferenceArray = if samplePass
                        == 0 as core::ffi::c_int
                    {
                        layeredAlphaItems
                    } else {
                        layeredColorItems
                    };
                    if !(itemIndex_0 >= (*currentItems).count) {
                        let mut item_0: *mut avifEncoderItem = *((*currentItems).ref_0)
                            .offset(itemIndex_0 as isize);
                        if !((*(*item_0).encodeOutput).samples.count <= layerIndex) {
                            if (*(*item_0).encodeOutput).samples.count
                                > layerIndex.wrapping_add(1 as uint32_t)
                            {
                                hasMoreSample = AVIF_TRUE as avifBool;
                            }
                            let mut data: *mut avifRWData = &mut (*((*(*item_0)
                                .encodeOutput)
                                .samples
                                .sample)
                                .offset(layerIndex as isize))
                                .data;
                            let mut chunkOffset_0: size_t = avifEncoderFindExistingChunk(
                                s,
                                mdatStartOffset,
                                (*data).data,
                                (*data).size,
                            );
                            if chunkOffset_0 == 0 {
                                chunkOffset_0 = avifRWStreamOffset(s);
                                let result___3: avifResult = avifRWStreamWrite(
                                    s,
                                    (*data).data as *const core::ffi::c_void,
                                    (*data).size,
                                ) as avifResult;
                                if result___3 as core::ffi::c_uint
                                    != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
                                {
                                    return result___3;
                                }
                                if samplePass == 0 as core::ffi::c_int {
                                    (*encoder).ioStats.alphaOBUSize = ((*encoder)
                                        .ioStats
                                        .alphaOBUSize)
                                        .wrapping_add((*data).size);
                                } else {
                                    (*encoder).ioStats.colorOBUSize = ((*encoder)
                                        .ioStats
                                        .colorOBUSize)
                                        .wrapping_add((*data).size);
                                }
                            }
                            let mut prevOffset_0: size_t = avifRWStreamOffset(s);
                            avifRWStreamSetOffset(
                                s,
                                (*((*item_0).mdatFixups.fixup).offset(layerIndex as isize))
                                    .offset,
                            );
                            let result___4: avifResult = avifRWStreamWriteU32(
                                s,
                                chunkOffset_0 as uint32_t,
                            ) as avifResult;
                            if result___4 as core::ffi::c_uint
                                != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
                            {
                                return result___4;
                            }
                            avifRWStreamSetOffset(s, prevOffset_0);
                        }
                    }
                    samplePass += 1;
                }
                itemIndex_0 = itemIndex_0.wrapping_add(1);
            }
            layerIndex = layerIndex.wrapping_add(1);
            if !(hasMoreSample != 0) {
                break;
            }
        }
        if !(layerIndex <= 4 as uint32_t) {
            return AVIF_RESULT_INTERNAL_ERROR;
        }
    }
    let result___5: avifResult = avifRWStreamFinishBox(s, mdat) as avifResult;
    if result___5 as core::ffi::c_uint
        != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
    {
        return result___5;
    }
    return AVIF_RESULT_OK;
}
unsafe extern "C" fn avifWriteAltrGroup(
    mut s: *mut avifRWStream,
    mut groupID: uint32_t,
    mut itemIDs: *const avifEncoderItemIdArray,
) -> avifResult {
    let mut grpl: avifBoxMarker = 0;
    let result__: avifResult = avifRWStreamWriteBox(
        s,
        b"grpl\0" as *const u8 as *const core::ffi::c_char,
        0 as size_t,
        &mut grpl,
    ) as avifResult;
    if result__ as core::ffi::c_uint
        != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
    {
        return result__;
    }
    let mut altr: avifBoxMarker = 0;
    let result___0: avifResult = avifRWStreamWriteFullBox(
        s,
        b"altr\0" as *const u8 as *const core::ffi::c_char,
        0 as size_t,
        0 as core::ffi::c_int,
        0 as uint32_t,
        &mut altr,
    ) as avifResult;
    if result___0 as core::ffi::c_uint
        != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
    {
        return result___0;
    }
    let result___1: avifResult = avifRWStreamWriteU32(s, groupID) as avifResult;
    if result___1 as core::ffi::c_uint
        != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
    {
        return result___1;
    }
    let result___2: avifResult = avifRWStreamWriteU32(s, (*itemIDs).count) as avifResult;
    if result___2 as core::ffi::c_uint
        != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
    {
        return result___2;
    }
    let mut i: uint32_t = 0 as uint32_t;
    while i < (*itemIDs).count {
        let result___3: avifResult = avifRWStreamWriteU32(
            s,
            *((*itemIDs).itemID).offset(i as isize),
        ) as avifResult;
        if result___3 as core::ffi::c_uint
            != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
        {
            return result___3;
        }
        i = i.wrapping_add(1);
    }
    let result___4: avifResult = avifRWStreamFinishBox(s, altr) as avifResult;
    if result___4 as core::ffi::c_uint
        != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
    {
        return result___4;
    }
    let result___5: avifResult = avifRWStreamFinishBox(s, grpl) as avifResult;
    if result___5 as core::ffi::c_uint
        != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
    {
        return result___5;
    }
    return AVIF_RESULT_OK;
}
unsafe extern "C" fn avifEncoderIsMiniCompatible(
    mut encoder: *const avifEncoder,
) -> avifBool {
    if (*encoder).extraLayerCount != 0
        || (*(*encoder).data).frames.count != 1 as uint32_t
    {
        return AVIF_FALSE;
    }
    if (*encoder).sampleTransformRecipe as core::ffi::c_uint
        != AVIF_SAMPLE_TRANSFORM_NONE as core::ffi::c_int as core::ffi::c_uint
    {
        return AVIF_FALSE;
    }
    if (*(*(*encoder).data).imageMetadata).width
        > ((1 as core::ffi::c_int) << 15 as core::ffi::c_int) as uint32_t
        || (*(*(*encoder).data).imageMetadata).height
            > ((1 as core::ffi::c_int) << 15 as core::ffi::c_int) as uint32_t
    {
        return AVIF_FALSE;
    }
    if (*(*(*encoder).data).imageMetadata).icc.size
        > ((1 as core::ffi::c_int) << 20 as core::ffi::c_int) as size_t
        || (*(*(*encoder).data).imageMetadata).exif.size
            > ((1 as core::ffi::c_int) << 20 as core::ffi::c_int) as size_t
        || (*(*(*encoder).data).imageMetadata).xmp.size
            > ((1 as core::ffi::c_int) << 20 as core::ffi::c_int) as size_t
    {
        return AVIF_FALSE;
    }
    if !((*(*(*encoder).data).imageMetadata).gainMap).is_null()
        && !((*(*(*(*encoder).data).imageMetadata).gainMap).image).is_null()
        && ((*(*(*(*(*encoder).data).imageMetadata).gainMap).image).width
            > ((1 as core::ffi::c_int) << 15 as core::ffi::c_int) as uint32_t
            || (*(*(*(*(*encoder).data).imageMetadata).gainMap).image).height
                > ((1 as core::ffi::c_int) << 15 as core::ffi::c_int) as uint32_t)
    {
        return AVIF_FALSE;
    }
    if (*(*(*encoder).data).altImageMetadata).icc.size
        > ((1 as core::ffi::c_int) << 20 as core::ffi::c_int) as size_t
    {
        return AVIF_FALSE;
    }
    if !((*(*(*encoder).data).imageMetadata).gainMap).is_null()
        && avifGainMapMetadataSize((*(*(*encoder).data).imageMetadata).gainMap)
            >= ((1 as core::ffi::c_int) << 20 as core::ffi::c_int) as uint32_t
    {
        return AVIF_FALSE;
    }
    if (*(*(*encoder).data).imageMetadata).yuvFormat as core::ffi::c_uint
        != AVIF_PIXEL_FORMAT_YUV444 as core::ffi::c_int as core::ffi::c_uint
        && (*(*(*encoder).data).imageMetadata).yuvFormat as core::ffi::c_uint
            != AVIF_PIXEL_FORMAT_YUV422 as core::ffi::c_int as core::ffi::c_uint
        && (*(*(*encoder).data).imageMetadata).yuvFormat as core::ffi::c_uint
            != AVIF_PIXEL_FORMAT_YUV420 as core::ffi::c_int as core::ffi::c_uint
        && (*(*(*encoder).data).imageMetadata).yuvFormat as core::ffi::c_uint
            != AVIF_PIXEL_FORMAT_YUV400 as core::ffi::c_int as core::ffi::c_uint
    {
        return AVIF_FALSE;
    }
    if !((*(*(*encoder).data).imageMetadata).gainMap).is_null()
        && !((*(*(*(*encoder).data).imageMetadata).gainMap).image).is_null()
        && ((*(*(*(*(*encoder).data).imageMetadata).gainMap).image).yuvFormat
            as core::ffi::c_uint
            != AVIF_PIXEL_FORMAT_YUV444 as core::ffi::c_int as core::ffi::c_uint
            && (*(*(*(*(*encoder).data).imageMetadata).gainMap).image).yuvFormat
                as core::ffi::c_uint
                != AVIF_PIXEL_FORMAT_YUV422 as core::ffi::c_int as core::ffi::c_uint
            && (*(*(*(*(*encoder).data).imageMetadata).gainMap).image).yuvFormat
                as core::ffi::c_uint
                != AVIF_PIXEL_FORMAT_YUV420 as core::ffi::c_int as core::ffi::c_uint
            && (*(*(*(*(*encoder).data).imageMetadata).gainMap).image).yuvFormat
                as core::ffi::c_uint
                != AVIF_PIXEL_FORMAT_YUV400 as core::ffi::c_int as core::ffi::c_uint)
    {
        return AVIF_FALSE;
    }
    if (*(*(*encoder).data).imageMetadata).colorPrimaries as core::ffi::c_int
        > 255 as core::ffi::c_int
        || (*(*(*encoder).data).imageMetadata).transferCharacteristics
            as core::ffi::c_int > 255 as core::ffi::c_int
        || (*(*(*encoder).data).imageMetadata).matrixCoefficients as core::ffi::c_int
            > 255 as core::ffi::c_int
    {
        return AVIF_FALSE;
    }
    if !((*(*(*encoder).data).imageMetadata).gainMap).is_null()
        && !((*(*(*(*encoder).data).imageMetadata).gainMap).image).is_null()
        && ((*(*(*(*(*encoder).data).imageMetadata).gainMap).image).colorPrimaries
            as core::ffi::c_int > 255 as core::ffi::c_int
            || (*(*(*(*(*encoder).data).imageMetadata).gainMap).image)
                .transferCharacteristics as core::ffi::c_int > 255 as core::ffi::c_int
            || (*(*(*(*(*encoder).data).imageMetadata).gainMap).image).matrixCoefficients
                as core::ffi::c_int > 255 as core::ffi::c_int)
    {
        return AVIF_FALSE;
    }
    if (*(*(*encoder).data).altImageMetadata).colorPrimaries as core::ffi::c_int
        > 255 as core::ffi::c_int
        || (*(*(*encoder).data).altImageMetadata).transferCharacteristics
            as core::ffi::c_int > 255 as core::ffi::c_int
        || (*(*(*encoder).data).altImageMetadata).matrixCoefficients as core::ffi::c_int
            > 255 as core::ffi::c_int
    {
        return AVIF_FALSE;
    }
    let mut colorItem: *const avifEncoderItem = 0 as *const avifEncoderItem;
    let mut itemIndex: uint32_t = 0 as uint32_t;
    while itemIndex < (*(*encoder).data).items.count {
        let mut item: *mut avifEncoderItem = &mut *((*(*encoder).data).items.item)
            .offset(itemIndex as isize) as *mut avifEncoderItem;
        if (*item).gridCols != 0 || (*item).gridRows != 0 {
            return AVIF_FALSE;
        }
        if (*item).id == (*(*encoder).data).primaryItemID {
            colorItem = item;
            if (*(*item).encodeOutput).samples.count != 1 as uint32_t
                || (*((*(*item).encodeOutput).samples.sample)
                    .offset(0 as core::ffi::c_int as isize))
                    .data
                    .size > ((1 as core::ffi::c_int) << 28 as core::ffi::c_int) as size_t
            {
                return AVIF_FALSE;
            }
        } else if (*item).itemCategory as core::ffi::c_uint
            == AVIF_ITEM_ALPHA as core::ffi::c_int as core::ffi::c_uint
            && (*item).irefToID == (*(*encoder).data).primaryItemID
        {
            if (*(*item).encodeOutput).samples.count != 1 as uint32_t
                || (*((*(*item).encodeOutput).samples.sample)
                    .offset(0 as core::ffi::c_int as isize))
                    .data
                    .size
                    >= ((1 as core::ffi::c_int) << 28 as core::ffi::c_int) as size_t
            {
                return AVIF_FALSE;
            }
        } else if (*item).itemCategory as core::ffi::c_uint
            == AVIF_ITEM_GAIN_MAP as core::ffi::c_int as core::ffi::c_uint
        {
            if (*(*item).encodeOutput).samples.count != 1 as uint32_t
                || (*((*(*item).encodeOutput).samples.sample)
                    .offset(0 as core::ffi::c_int as isize))
                    .data
                    .size
                    >= ((1 as core::ffi::c_int) << 28 as core::ffi::c_int) as size_t
            {
                return AVIF_FALSE;
            }
        } else if !(memcmp(
            ((*item).type_0).as_mut_ptr() as *const core::ffi::c_void,
            b"tmap\0" as *const u8 as *const core::ffi::c_char
                as *const core::ffi::c_void,
            4 as size_t,
        ) == 0)
        {
            if !(memcmp(
                ((*item).type_0).as_mut_ptr() as *const core::ffi::c_void,
                b"mime\0" as *const u8 as *const core::ffi::c_char
                    as *const core::ffi::c_void,
                4 as size_t,
            ) == 0
                && memcmp(
                    (*item).infeName as *const core::ffi::c_void,
                    b"XMP\0" as *const u8 as *const core::ffi::c_char
                        as *const core::ffi::c_void,
                    (*item).infeNameSize,
                ) == 0)
            {
                if memcmp(
                    ((*item).type_0).as_mut_ptr() as *const core::ffi::c_void,
                    b"Exif\0" as *const u8 as *const core::ffi::c_char
                        as *const core::ffi::c_void,
                    4 as size_t,
                ) == 0
                    && memcmp(
                        (*item).infeName as *const core::ffi::c_void,
                        b"Exif\0" as *const u8 as *const core::ffi::c_char
                            as *const core::ffi::c_void,
                        (*item).infeNameSize,
                    ) == 0
                {
                    let exif_tiff_header_offset: uint32_t = *((*item)
                        .metadataPayload
                        .data as *mut uint32_t);
                    if exif_tiff_header_offset != 0 as uint32_t {
                        return AVIF_FALSE;
                    }
                } else {
                    return AVIF_FALSE
                }
            }
        }
        itemIndex = itemIndex.wrapping_add(1);
    }
    if colorItem.is_null() {
        return AVIF_FALSE;
    }
    return AVIF_TRUE;
}
unsafe extern "C" fn avifEncoderWriteFileTypeBoxAndMiniBox(
    mut encoder: *mut avifEncoder,
    mut output: *mut avifRWData,
) -> avifResult {
    let mut s: avifRWStream = avifRWStream {
        raw: 0 as *mut avifRWData,
        offset: 0,
        numUsedBitsInPartialByte: 0,
    };
    avifRWStreamStart(&mut s, output);
    let mut ftyp: avifBoxMarker = 0;
    let result__: avifResult = avifRWStreamWriteBox(
        &mut s,
        b"ftyp\0" as *const u8 as *const core::ffi::c_char,
        0 as size_t,
        &mut ftyp,
    ) as avifResult;
    if result__ as core::ffi::c_uint
        != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
    {
        return result__;
    }
    let result___0: avifResult = avifRWStreamWriteChars(
        &mut s,
        b"mif3\0" as *const u8 as *const core::ffi::c_char,
        4 as size_t,
    ) as avifResult;
    if result___0 as core::ffi::c_uint
        != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
    {
        return result___0;
    }
    let result___1: avifResult = avifRWStreamWriteChars(
        &mut s,
        b"avif\0" as *const u8 as *const core::ffi::c_char,
        4 as size_t,
    ) as avifResult;
    if result___1 as core::ffi::c_uint
        != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
    {
        return result___1;
    }
    let result___2: avifResult = avifRWStreamFinishBox(&mut s, ftyp) as avifResult;
    if result___2 as core::ffi::c_uint
        != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
    {
        return result___2;
    }
    let result___3: avifResult = avifEncoderWriteMiniBox(encoder, &mut s) as avifResult;
    if result___3 as core::ffi::c_uint
        != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
    {
        return result___3;
    }
    avifRWStreamFinishWrite(&mut s);
    return AVIF_RESULT_OK;
}
unsafe extern "C" fn avifEncoderWriteMiniBox(
    mut encoder: *mut avifEncoder,
    mut s: *mut avifRWStream,
) -> avifResult {
    let mut colorItem: *const avifEncoderItem = 0 as *const avifEncoderItem;
    let mut alphaItem: *const avifEncoderItem = 0 as *const avifEncoderItem;
    let mut gainmapItem: *const avifEncoderItem = 0 as *const avifEncoderItem;
    let mut itemIndex: uint32_t = 0 as uint32_t;
    while itemIndex < (*(*encoder).data).items.count {
        let mut item: *mut avifEncoderItem = &mut *((*(*encoder).data).items.item)
            .offset(itemIndex as isize) as *mut avifEncoderItem;
        if (*item).id == (*(*encoder).data).primaryItemID {
            if !colorItem.is_null() {
                return AVIF_RESULT_INTERNAL_ERROR;
            }
            colorItem = item;
        } else if (*item).itemCategory as core::ffi::c_uint
            == AVIF_ITEM_ALPHA as core::ffi::c_int as core::ffi::c_uint
            && (*item).irefToID == (*(*encoder).data).primaryItemID
        {
            if !alphaItem.is_null() {
                return AVIF_RESULT_INTERNAL_ERROR;
            }
            alphaItem = item;
        }
        if (*item).itemCategory as core::ffi::c_uint
            == AVIF_ITEM_GAIN_MAP as core::ffi::c_int as core::ffi::c_uint
        {
            if !gainmapItem.is_null() {
                return AVIF_RESULT_INTERNAL_ERROR;
            }
            gainmapItem = item;
        }
        itemIndex = itemIndex.wrapping_add(1);
    }
    if colorItem.is_null() {
        return AVIF_RESULT_INTERNAL_ERROR;
    }
    let mut colorData: *const avifRWData = &mut (*((*(*colorItem).encodeOutput)
        .samples
        .sample)
        .offset(0 as core::ffi::c_int as isize))
        .data;
    let mut alphaData: *const avifRWData = if !alphaItem.is_null() {
        &mut (*((*(*alphaItem).encodeOutput).samples.sample)
            .offset(0 as core::ffi::c_int as isize))
            .data
    } else {
        0 as *mut avifRWData
    };
    let mut gainmapData: *const avifRWData = if !gainmapItem.is_null() {
        &mut (*((*(*gainmapItem).encodeOutput).samples.sample)
            .offset(0 as core::ffi::c_int as isize))
            .data
    } else {
        0 as *mut avifRWData
    };
    let image: *const avifImage = (*(*encoder).data).imageMetadata;
    let hasAlpha: avifBool = (alphaItem != NULL as *const avifEncoderItem)
        as core::ffi::c_int;
    let alphaIsPremultiplied: avifBool = (*(*(*encoder).data).imageMetadata)
        .alphaPremultiplied;
    let hasGainmap: avifBool = (gainmapItem != NULL as *const avifEncoderItem)
        as core::ffi::c_int;
    let hasHdr: avifBool = hasGainmap;
    let hasIcc: avifBool = ((*image).icc.size != 0 as size_t) as core::ffi::c_int;
    let chromaSubsampling: uint32_t = (if (*image).yuvFormat as core::ffi::c_uint
        == AVIF_PIXEL_FORMAT_YUV400 as core::ffi::c_int as core::ffi::c_uint
    {
        0 as core::ffi::c_int
    } else if (*image).yuvFormat as core::ffi::c_uint
        == AVIF_PIXEL_FORMAT_YUV420 as core::ffi::c_int as core::ffi::c_uint
    {
        1 as core::ffi::c_int
    } else if (*image).yuvFormat as core::ffi::c_uint
        == AVIF_PIXEL_FORMAT_YUV422 as core::ffi::c_int as core::ffi::c_uint
    {
        2 as core::ffi::c_int
    } else {
        3 as core::ffi::c_int
    }) as uint32_t;
    let defaultColorPrimaries: avifColorPrimaries = (if hasIcc != 0 {
        AVIF_COLOR_PRIMARIES_UNSPECIFIED as core::ffi::c_int
    } else {
        AVIF_COLOR_PRIMARIES_BT709 as core::ffi::c_int
    }) as avifColorPrimaries;
    let defaultTransferCharacteristics: avifTransferCharacteristics = (if hasIcc != 0 {
        AVIF_TRANSFER_CHARACTERISTICS_UNSPECIFIED as core::ffi::c_int
    } else {
        AVIF_TRANSFER_CHARACTERISTICS_SRGB as core::ffi::c_int
    }) as avifTransferCharacteristics;
    let defaultMatrixCoefficients: avifMatrixCoefficients = (if chromaSubsampling
        == 0 as uint32_t
    {
        AVIF_MATRIX_COEFFICIENTS_UNSPECIFIED as core::ffi::c_int
    } else {
        AVIF_MATRIX_COEFFICIENTS_BT601 as core::ffi::c_int
    }) as avifMatrixCoefficients;
    let hasExplicitCicp: avifBool = ((*image).colorPrimaries as core::ffi::c_int
        != defaultColorPrimaries as core::ffi::c_int
        || (*image).transferCharacteristics as core::ffi::c_int
            != defaultTransferCharacteristics as core::ffi::c_int
        || (*image).matrixCoefficients as core::ffi::c_int
            != defaultMatrixCoefficients as core::ffi::c_int) as core::ffi::c_int;
    let floatFlag: avifBool = AVIF_FALSE;
    let fullRange: avifBool = ((*image).yuvRange as core::ffi::c_uint
        == AVIF_RANGE_FULL as core::ffi::c_int as core::ffi::c_uint) as core::ffi::c_int;
    if (*image).yuvFormat as core::ffi::c_uint
        != AVIF_PIXEL_FORMAT_YUV420 as core::ffi::c_int as core::ffi::c_uint
        && (*image).yuvChromaSamplePosition as core::ffi::c_uint
            != AVIF_CHROMA_SAMPLE_POSITION_UNKNOWN as core::ffi::c_int
                as core::ffi::c_uint
    {
        avifDiagnosticsPrintf(
            &mut (*encoder).diag as *mut avifDiagnostics,
            b"YUV chroma sample position %d is only supported with 4:2:0 YUV format in AV1\0"
                as *const u8 as *const core::ffi::c_char,
            (*image).yuvChromaSamplePosition as core::ffi::c_uint,
        );
        return AVIF_RESULT_INVALID_ARGUMENT;
    }
    let chromaIsHorizontallyCentered: avifBool = ((*image).yuvFormat as core::ffi::c_uint
        == AVIF_PIXEL_FORMAT_YUV420 as core::ffi::c_int as core::ffi::c_uint
        && (*image).yuvChromaSamplePosition as core::ffi::c_uint
            != AVIF_CHROMA_SAMPLE_POSITION_VERTICAL as core::ffi::c_int
                as core::ffi::c_uint
        && (*image).yuvChromaSamplePosition as core::ffi::c_uint
            != AVIF_CHROMA_SAMPLE_POSITION_COLOCATED as core::ffi::c_int
                as core::ffi::c_uint) as core::ffi::c_int;
    let chromaIsVerticallyCentered: avifBool = ((*image).yuvFormat as core::ffi::c_uint
        == AVIF_PIXEL_FORMAT_YUV420 as core::ffi::c_int as core::ffi::c_uint
        && (*image).yuvChromaSamplePosition as core::ffi::c_uint
            != AVIF_CHROMA_SAMPLE_POSITION_COLOCATED as core::ffi::c_int
                as core::ffi::c_uint) as core::ffi::c_int;
    let orientationMinus1: uint32_t = (avifImageIrotImirToExifOrientation(image)
        as core::ffi::c_int - 1 as core::ffi::c_int) as uint32_t;
    let mut infeType: [uint8_t; 4] = [0; 4];
    let mut codecConfigType: [uint8_t; 4] = [0; 4];
    let mut hasExplicitCodecTypes: avifBool = 0;
    if (*encoder).codecChoice as core::ffi::c_uint
        == AVIF_CODEC_CHOICE_AVM as core::ffi::c_int as core::ffi::c_uint
    {
        memcpy(
            infeType.as_mut_ptr() as *mut core::ffi::c_void,
            b"av02\0" as *const u8 as *const core::ffi::c_char
                as *const core::ffi::c_void,
            4 as size_t,
        );
        memcpy(
            codecConfigType.as_mut_ptr() as *mut core::ffi::c_void,
            b"av2C\0" as *const u8 as *const core::ffi::c_char
                as *const core::ffi::c_void,
            4 as size_t,
        );
        hasExplicitCodecTypes = AVIF_TRUE as avifBool;
    } else {
        memcpy(
            infeType.as_mut_ptr() as *mut core::ffi::c_void,
            b"av01\0" as *const u8 as *const core::ffi::c_char
                as *const core::ffi::c_void,
            4 as size_t,
        );
        memcpy(
            codecConfigType.as_mut_ptr() as *mut core::ffi::c_void,
            b"av1C\0" as *const u8 as *const core::ffi::c_char
                as *const core::ffi::c_void,
            4 as size_t,
        );
        hasExplicitCodecTypes = AVIF_FALSE as avifBool;
    }
    if !((*image).width != 0 as uint32_t && (*image).height != 0 as uint32_t) {
        return AVIF_RESULT_INTERNAL_ERROR;
    }
    if !((*colorData).size != 0 as size_t) {
        return AVIF_RESULT_INTERNAL_ERROR;
    }
    let mut largeDimensionsFlag: uint32_t = (((*image).width).wrapping_sub(1 as uint32_t)
        >= ((1 as core::ffi::c_int) << 7 as core::ffi::c_int) as uint32_t
        || ((*image).height).wrapping_sub(1 as uint32_t)
            >= ((1 as core::ffi::c_int) << 7 as core::ffi::c_int) as uint32_t)
        as core::ffi::c_int as uint32_t;
    let codecConfigSize: uint32_t = 4 as uint32_t;
    let mut alphaCodecConfigSize: uint32_t = 0 as uint32_t;
    let mut gainmapCodecConfigSize: uint32_t = 0 as uint32_t;
    let mut gainmapMetadataSize: uint32_t = 0 as uint32_t;
    let largeCodecConfigFlag: uint32_t = (codecConfigSize
        >= ((1 as core::ffi::c_int) << 3 as core::ffi::c_int) as uint32_t)
        as core::ffi::c_int as uint32_t;
    let mut largeItemDataFlag: uint32_t = (((*colorData).size).wrapping_sub(1 as size_t)
        >= ((1 as core::ffi::c_int) << 15 as core::ffi::c_int) as size_t
        || !alphaData.is_null()
            && (*alphaData).size
                >= ((1 as core::ffi::c_int) << 15 as core::ffi::c_int) as size_t)
        as core::ffi::c_int as uint32_t;
    let mut largeMetadataFlag: uint32_t = (hasIcc != 0
        && ((*image).icc.size).wrapping_sub(1 as size_t)
            >= ((1 as core::ffi::c_int) << 10 as core::ffi::c_int) as size_t
        || (*image).exif.size != 0 as size_t
            && ((*image).exif.size).wrapping_sub(1 as size_t)
                >= ((1 as core::ffi::c_int) << 10 as core::ffi::c_int) as size_t
        || (*image).xmp.size != 0 as size_t
            && ((*image).xmp.size).wrapping_sub(1 as size_t)
                >= ((1 as core::ffi::c_int) << 10 as core::ffi::c_int) as size_t)
        as core::ffi::c_int as uint32_t;
    if hasGainmap != 0 {
        if !(!((*image).gainMap).is_null() && !((*(*image).gainMap).image).is_null()) {
            return AVIF_RESULT_INTERNAL_ERROR;
        }
        gainmapMetadataSize = avifGainMapMetadataSize((*image).gainMap);
        if gainmapData.is_null() {
            return AVIF_RESULT_INTERNAL_ERROR;
        }
        if !((*(*(*image).gainMap).image).width != 0 as uint32_t
            && (*(*(*image).gainMap).image).height != 0 as uint32_t)
        {
            return AVIF_RESULT_INTERNAL_ERROR;
        }
        largeDimensionsFlag
            |= (((*(*(*image).gainMap).image).width).wrapping_sub(1 as uint32_t)
                >= ((1 as core::ffi::c_int) << 7 as core::ffi::c_int) as uint32_t
                || ((*(*(*image).gainMap).image).height).wrapping_sub(1 as uint32_t)
                    >= ((1 as core::ffi::c_int) << 7 as core::ffi::c_int) as uint32_t)
                as core::ffi::c_int as uint32_t;
        largeItemDataFlag
            |= ((*gainmapData).size
                >= ((1 as core::ffi::c_int) << 15 as core::ffi::c_int) as size_t)
                as core::ffi::c_int as uint32_t;
        largeMetadataFlag
            |= ((*(*(*encoder).data).altImageMetadata).icc.size != 0 as size_t
                && ((*(*(*encoder).data).altImageMetadata).icc.size)
                    .wrapping_sub(1 as size_t)
                    >= ((1 as core::ffi::c_int) << 10 as core::ffi::c_int) as size_t
                || gainmapMetadataSize
                    >= ((1 as core::ffi::c_int) << 10 as core::ffi::c_int) as uint32_t)
                as core::ffi::c_int as uint32_t;
    }
    let mut mini: avifBoxMarker = 0;
    let result__: avifResult = avifRWStreamWriteBox(
        s,
        b"mini\0" as *const u8 as *const core::ffi::c_char,
        0 as size_t,
        &mut mini,
    ) as avifResult;
    if result__ as core::ffi::c_uint
        != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
    {
        return result__;
    }
    let result___0: avifResult = avifRWStreamWriteBits(s, 0 as uint32_t, 2 as size_t)
        as avifResult;
    if result___0 as core::ffi::c_uint
        != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
    {
        return result___0;
    }
    let result___1: avifResult = avifRWStreamWriteBits(
        s,
        hasExplicitCodecTypes as uint32_t,
        1 as size_t,
    ) as avifResult;
    if result___1 as core::ffi::c_uint
        != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
    {
        return result___1;
    }
    let result___2: avifResult = avifRWStreamWriteBits(
        s,
        floatFlag as uint32_t,
        1 as size_t,
    ) as avifResult;
    if result___2 as core::ffi::c_uint
        != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
    {
        return result___2;
    }
    let result___3: avifResult = avifRWStreamWriteBits(
        s,
        fullRange as uint32_t,
        1 as size_t,
    ) as avifResult;
    if result___3 as core::ffi::c_uint
        != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
    {
        return result___3;
    }
    let result___4: avifResult = avifRWStreamWriteBits(
        s,
        (alphaItem != 0 as *const avifEncoderItem) as core::ffi::c_int as uint32_t,
        1 as size_t,
    ) as avifResult;
    if result___4 as core::ffi::c_uint
        != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
    {
        return result___4;
    }
    let result___5: avifResult = avifRWStreamWriteBits(
        s,
        hasExplicitCicp as uint32_t,
        1 as size_t,
    ) as avifResult;
    if result___5 as core::ffi::c_uint
        != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
    {
        return result___5;
    }
    let result___6: avifResult = avifRWStreamWriteBits(
        s,
        hasHdr as uint32_t,
        1 as size_t,
    ) as avifResult;
    if result___6 as core::ffi::c_uint
        != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
    {
        return result___6;
    }
    let result___7: avifResult = avifRWStreamWriteBits(
        s,
        hasIcc as uint32_t,
        1 as size_t,
    ) as avifResult;
    if result___7 as core::ffi::c_uint
        != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
    {
        return result___7;
    }
    let result___8: avifResult = avifRWStreamWriteBits(
        s,
        ((*image).exif.size != 0 as size_t) as core::ffi::c_int as uint32_t,
        1 as size_t,
    ) as avifResult;
    if result___8 as core::ffi::c_uint
        != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
    {
        return result___8;
    }
    let result___9: avifResult = avifRWStreamWriteBits(
        s,
        ((*image).xmp.size != 0 as size_t) as core::ffi::c_int as uint32_t,
        1 as size_t,
    ) as avifResult;
    if result___9 as core::ffi::c_uint
        != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
    {
        return result___9;
    }
    let result___10: avifResult = avifRWStreamWriteBits(
        s,
        chromaSubsampling,
        2 as size_t,
    ) as avifResult;
    if result___10 as core::ffi::c_uint
        != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
    {
        return result___10;
    }
    let result___11: avifResult = avifRWStreamWriteBits(
        s,
        orientationMinus1,
        3 as size_t,
    ) as avifResult;
    if result___11 as core::ffi::c_uint
        != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
    {
        return result___11;
    }
    let result___12: avifResult = avifRWStreamWriteBits(
        s,
        largeDimensionsFlag,
        1 as size_t,
    ) as avifResult;
    if result___12 as core::ffi::c_uint
        != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
    {
        return result___12;
    }
    let result___13: avifResult = avifRWStreamWriteBits(
        s,
        ((*image).width).wrapping_sub(1 as uint32_t),
        (if largeDimensionsFlag != 0 {
            15 as core::ffi::c_int
        } else {
            7 as core::ffi::c_int
        }) as size_t,
    ) as avifResult;
    if result___13 as core::ffi::c_uint
        != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
    {
        return result___13;
    }
    let result___14: avifResult = avifRWStreamWriteBits(
        s,
        ((*image).height).wrapping_sub(1 as uint32_t),
        (if largeDimensionsFlag != 0 {
            15 as core::ffi::c_int
        } else {
            7 as core::ffi::c_int
        }) as size_t,
    ) as avifResult;
    if result___14 as core::ffi::c_uint
        != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
    {
        return result___14;
    }
    if chromaSubsampling == 1 as uint32_t || chromaSubsampling == 2 as uint32_t {
        let result___15: avifResult = avifRWStreamWriteBits(
            s,
            chromaIsHorizontallyCentered as uint32_t,
            1 as size_t,
        ) as avifResult;
        if result___15 as core::ffi::c_uint
            != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
        {
            return result___15;
        }
    }
    if chromaSubsampling == 1 as uint32_t {
        let result___16: avifResult = avifRWStreamWriteBits(
            s,
            chromaIsVerticallyCentered as uint32_t,
            1 as size_t,
        ) as avifResult;
        if result___16 as core::ffi::c_uint
            != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
        {
            return result___16;
        }
    }
    if floatFlag != 0 {
        return AVIF_RESULT_INTERNAL_ERROR
    } else {
        let result___17: avifResult = avifRWStreamWriteBits(
            s,
            ((*image).depth > 8 as uint32_t) as core::ffi::c_int as uint32_t,
            1 as size_t,
        ) as avifResult;
        if result___17 as core::ffi::c_uint
            != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
        {
            return result___17;
        }
        if (*image).depth > 8 as uint32_t {
            let result___18: avifResult = avifRWStreamWriteBits(
                s,
                ((*image).depth).wrapping_sub(9 as uint32_t),
                3 as size_t,
            ) as avifResult;
            if result___18 as core::ffi::c_uint
                != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
            {
                return result___18;
            }
        }
    }
    if !alphaItem.is_null() {
        let result___19: avifResult = avifRWStreamWriteBits(
            s,
            alphaIsPremultiplied as uint32_t,
            1 as size_t,
        ) as avifResult;
        if result___19 as core::ffi::c_uint
            != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
        {
            return result___19;
        }
    }
    if hasExplicitCicp != 0 {
        let result___20: avifResult = avifRWStreamWriteBits(
            s,
            (*image).colorPrimaries as uint32_t,
            8 as size_t,
        ) as avifResult;
        if result___20 as core::ffi::c_uint
            != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
        {
            return result___20;
        }
        let result___21: avifResult = avifRWStreamWriteBits(
            s,
            (*image).transferCharacteristics as uint32_t,
            8 as size_t,
        ) as avifResult;
        if result___21 as core::ffi::c_uint
            != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
        {
            return result___21;
        }
        if chromaSubsampling != 0 as uint32_t {
            let result___22: avifResult = avifRWStreamWriteBits(
                s,
                (*image).matrixCoefficients as uint32_t,
                8 as size_t,
            ) as avifResult;
            if result___22 as core::ffi::c_uint
                != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
            {
                return result___22;
            }
        } else if !((*image).matrixCoefficients as core::ffi::c_int
            == AVIF_MATRIX_COEFFICIENTS_UNSPECIFIED as core::ffi::c_int)
        {
            return AVIF_RESULT_ENCODE_COLOR_FAILED
        }
    }
    if hasExplicitCodecTypes != 0 {
        let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
        while i < 4 as core::ffi::c_int {
            let result___23: avifResult = avifRWStreamWriteBits(
                s,
                infeType[i as usize] as uint32_t,
                8 as size_t,
            ) as avifResult;
            if result___23 as core::ffi::c_uint
                != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
            {
                return result___23;
            }
            i += 1;
        }
        let mut i_0: core::ffi::c_int = 0 as core::ffi::c_int;
        while i_0 < 4 as core::ffi::c_int {
            let result___24: avifResult = avifRWStreamWriteBits(
                s,
                codecConfigType[i_0 as usize] as uint32_t,
                8 as size_t,
            ) as avifResult;
            if result___24 as core::ffi::c_uint
                != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
            {
                return result___24;
            }
            i_0 += 1;
        }
    }
    let mut tmapIccSize: size_t = 0 as size_t;
    if hasHdr != 0 {
        let result___25: avifResult = avifRWStreamWriteBits(
            s,
            hasGainmap as uint32_t,
            1 as size_t,
        ) as avifResult;
        if result___25 as core::ffi::c_uint
            != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
        {
            return result___25;
        }
        if hasGainmap != 0 {
            let mut tmap: *const avifImage = (*(*encoder).data).altImageMetadata;
            let mut gainmap: *const avifImage = (*(*image).gainMap).image;
            let result___26: avifResult = avifRWStreamWriteBits(
                s,
                ((*gainmap).width).wrapping_sub(1 as uint32_t),
                (if largeDimensionsFlag != 0 {
                    15 as core::ffi::c_int
                } else {
                    7 as core::ffi::c_int
                }) as size_t,
            ) as avifResult;
            if result___26 as core::ffi::c_uint
                != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
            {
                return result___26;
            }
            let result___27: avifResult = avifRWStreamWriteBits(
                s,
                ((*gainmap).height).wrapping_sub(1 as uint32_t),
                (if largeDimensionsFlag != 0 {
                    15 as core::ffi::c_int
                } else {
                    7 as core::ffi::c_int
                }) as size_t,
            ) as avifResult;
            if result___27 as core::ffi::c_uint
                != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
            {
                return result___27;
            }
            let result___28: avifResult = avifRWStreamWriteBits(
                s,
                (*gainmap).matrixCoefficients as uint32_t,
                8 as size_t,
            ) as avifResult;
            if result___28 as core::ffi::c_uint
                != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
            {
                return result___28;
            }
            let result___29: avifResult = avifRWStreamWriteBits(
                s,
                ((*gainmap).yuvRange as core::ffi::c_uint
                    == AVIF_RANGE_FULL as core::ffi::c_int as core::ffi::c_uint)
                    as core::ffi::c_int as uint32_t,
                1 as size_t,
            ) as avifResult;
            if result___29 as core::ffi::c_uint
                != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
            {
                return result___29;
            }
            let gainmapChromaSubsampling: uint32_t = (if (*gainmap).yuvFormat
                as core::ffi::c_uint
                == AVIF_PIXEL_FORMAT_YUV400 as core::ffi::c_int as core::ffi::c_uint
            {
                0 as core::ffi::c_int
            } else if (*gainmap).yuvFormat as core::ffi::c_uint
                == AVIF_PIXEL_FORMAT_YUV420 as core::ffi::c_int as core::ffi::c_uint
            {
                1 as core::ffi::c_int
            } else if (*gainmap).yuvFormat as core::ffi::c_uint
                == AVIF_PIXEL_FORMAT_YUV422 as core::ffi::c_int as core::ffi::c_uint
            {
                2 as core::ffi::c_int
            } else {
                3 as core::ffi::c_int
            }) as uint32_t;
            let result___30: avifResult = avifRWStreamWriteBits(
                s,
                gainmapChromaSubsampling,
                2 as size_t,
            ) as avifResult;
            if result___30 as core::ffi::c_uint
                != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
            {
                return result___30;
            }
            if gainmapChromaSubsampling == 1 as uint32_t
                || gainmapChromaSubsampling == 2 as uint32_t
            {
                let result___31: avifResult = avifRWStreamWriteBits(
                    s,
                    ((*gainmap).yuvFormat as core::ffi::c_uint
                        == AVIF_PIXEL_FORMAT_YUV420 as core::ffi::c_int
                            as core::ffi::c_uint
                        && (*gainmap).yuvChromaSamplePosition as core::ffi::c_uint
                            != AVIF_CHROMA_SAMPLE_POSITION_VERTICAL as core::ffi::c_int
                                as core::ffi::c_uint
                        && (*gainmap).yuvChromaSamplePosition as core::ffi::c_uint
                            != AVIF_CHROMA_SAMPLE_POSITION_COLOCATED as core::ffi::c_int
                                as core::ffi::c_uint) as core::ffi::c_int as uint32_t,
                    1 as size_t,
                ) as avifResult;
                if result___31 as core::ffi::c_uint
                    != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
                {
                    return result___31;
                }
            }
            if gainmapChromaSubsampling == 1 as uint32_t {
                let result___32: avifResult = avifRWStreamWriteBits(
                    s,
                    ((*gainmap).yuvFormat as core::ffi::c_uint
                        == AVIF_PIXEL_FORMAT_YUV420 as core::ffi::c_int
                            as core::ffi::c_uint
                        && (*gainmap).yuvChromaSamplePosition as core::ffi::c_uint
                            != AVIF_CHROMA_SAMPLE_POSITION_COLOCATED as core::ffi::c_int
                                as core::ffi::c_uint) as core::ffi::c_int as uint32_t,
                    1 as size_t,
                ) as avifResult;
                if result___32 as core::ffi::c_uint
                    != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
                {
                    return result___32;
                }
            }
            let gainmapFloatFlag: avifBool = AVIF_FALSE;
            let result___33: avifResult = avifRWStreamWriteBits(
                s,
                gainmapFloatFlag as uint32_t,
                1 as size_t,
            ) as avifResult;
            if result___33 as core::ffi::c_uint
                != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
            {
                return result___33;
            }
            if gainmapFloatFlag != 0 {
                return AVIF_RESULT_INTERNAL_ERROR
            } else {
                let result___34: avifResult = avifRWStreamWriteBits(
                    s,
                    ((*gainmap).depth > 8 as uint32_t) as core::ffi::c_int as uint32_t,
                    1 as size_t,
                ) as avifResult;
                if result___34 as core::ffi::c_uint
                    != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
                {
                    return result___34;
                }
                if (*gainmap).depth > 8 as uint32_t {
                    let result___35: avifResult = avifRWStreamWriteBits(
                        s,
                        ((*gainmap).depth).wrapping_sub(9 as uint32_t),
                        3 as size_t,
                    ) as avifResult;
                    if result___35 as core::ffi::c_uint
                        != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
                    {
                        return result___35;
                    }
                }
            }
            tmapIccSize = (*(*(*encoder).data).altImageMetadata).icc.size;
            let result___36: avifResult = avifRWStreamWriteBits(
                s,
                (tmapIccSize != 0 as size_t) as core::ffi::c_int as uint32_t,
                1 as size_t,
            ) as avifResult;
            if result___36 as core::ffi::c_uint
                != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
            {
                return result___36;
            }
            let tmapHasExplicitCicp: avifBool = ((*tmap).colorPrimaries
                as core::ffi::c_int != AVIF_COLOR_PRIMARIES_BT709 as core::ffi::c_int
                || (*tmap).transferCharacteristics as core::ffi::c_int
                    != AVIF_TRANSFER_CHARACTERISTICS_SRGB as core::ffi::c_int
                || (*tmap).matrixCoefficients as core::ffi::c_int
                    != AVIF_MATRIX_COEFFICIENTS_BT601 as core::ffi::c_int
                || (*tmap).yuvRange as core::ffi::c_uint
                    != AVIF_RANGE_FULL as core::ffi::c_int as core::ffi::c_uint)
                as core::ffi::c_int;
            let result___37: avifResult = avifRWStreamWriteBits(
                s,
                tmapHasExplicitCicp as uint32_t,
                1 as size_t,
            ) as avifResult;
            if result___37 as core::ffi::c_uint
                != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
            {
                return result___37;
            }
            if tmapHasExplicitCicp != 0 {
                let result___38: avifResult = avifRWStreamWriteBits(
                    s,
                    (*tmap).colorPrimaries as uint32_t,
                    8 as size_t,
                ) as avifResult;
                if result___38 as core::ffi::c_uint
                    != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
                {
                    return result___38;
                }
                let result___39: avifResult = avifRWStreamWriteBits(
                    s,
                    (*tmap).transferCharacteristics as uint32_t,
                    8 as size_t,
                ) as avifResult;
                if result___39 as core::ffi::c_uint
                    != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
                {
                    return result___39;
                }
                let result___40: avifResult = avifRWStreamWriteBits(
                    s,
                    (*tmap).matrixCoefficients as uint32_t,
                    8 as size_t,
                ) as avifResult;
                if result___40 as core::ffi::c_uint
                    != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
                {
                    return result___40;
                }
                let result___41: avifResult = avifRWStreamWriteBits(
                    s,
                    ((*tmap).yuvRange as core::ffi::c_uint
                        == AVIF_RANGE_FULL as core::ffi::c_int as core::ffi::c_uint)
                        as core::ffi::c_int as uint32_t,
                    1 as size_t,
                ) as avifResult;
                if result___41 as core::ffi::c_uint
                    != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
                {
                    return result___41;
                }
            }
        }
        let result___42: avifResult = avifEncoderWriteMiniHDRProperties(s, image)
            as avifResult;
        if result___42 as core::ffi::c_uint
            != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
        {
            return result___42;
        }
        if hasGainmap != 0 {
            let result___43: avifResult = avifEncoderWriteMiniHDRProperties(
                s,
                (*(*encoder).data).altImageMetadata,
            ) as avifResult;
            if result___43 as core::ffi::c_uint
                != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
            {
                return result___43;
            }
        }
    }
    if hasIcc != 0 || (*image).exif.size != 0 || (*image).xmp.size != 0
        || hasHdr != 0 && hasGainmap != 0
    {
        let result___44: avifResult = avifRWStreamWriteBits(
            s,
            largeMetadataFlag,
            1 as size_t,
        ) as avifResult;
        if result___44 as core::ffi::c_uint
            != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
        {
            return result___44;
        }
    }
    let result___45: avifResult = avifRWStreamWriteBits(
        s,
        largeCodecConfigFlag,
        1 as size_t,
    ) as avifResult;
    if result___45 as core::ffi::c_uint
        != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
    {
        return result___45;
    }
    let result___46: avifResult = avifRWStreamWriteBits(
        s,
        largeItemDataFlag,
        1 as size_t,
    ) as avifResult;
    if result___46 as core::ffi::c_uint
        != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
    {
        return result___46;
    }
    if hasIcc != 0 {
        let result___47: avifResult = avifRWStreamWriteBits(
            s,
            ((*image).icc.size as uint32_t).wrapping_sub(1 as uint32_t),
            (if largeMetadataFlag != 0 {
                20 as core::ffi::c_int
            } else {
                10 as core::ffi::c_int
            }) as size_t,
        ) as avifResult;
        if result___47 as core::ffi::c_uint
            != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
        {
            return result___47;
        }
    }
    if hasHdr != 0 && hasGainmap != 0 && tmapIccSize != 0 as size_t {
        let result___48: avifResult = avifRWStreamWriteBits(
            s,
            (tmapIccSize as uint32_t).wrapping_sub(1 as uint32_t),
            (if largeMetadataFlag != 0 {
                20 as core::ffi::c_int
            } else {
                10 as core::ffi::c_int
            }) as size_t,
        ) as avifResult;
        if result___48 as core::ffi::c_uint
            != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
        {
            return result___48;
        }
    }
    if hasHdr != 0 && hasGainmap != 0 {
        let result___49: avifResult = avifRWStreamWriteBits(
            s,
            gainmapMetadataSize,
            (if largeMetadataFlag != 0 {
                20 as core::ffi::c_int
            } else {
                10 as core::ffi::c_int
            }) as size_t,
        ) as avifResult;
        if result___49 as core::ffi::c_uint
            != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
        {
            return result___49;
        }
    }
    if hasHdr != 0 && hasGainmap != 0 {
        let result___50: avifResult = avifRWStreamWriteBits(
            s,
            (*gainmapData).size as uint32_t,
            (if largeItemDataFlag != 0 {
                28 as core::ffi::c_int
            } else {
                15 as core::ffi::c_int
            }) as size_t,
        ) as avifResult;
        if result___50 as core::ffi::c_uint
            != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
        {
            return result___50;
        }
    }
    if hasHdr != 0 && hasGainmap != 0 && (*gainmapData).size != 0 as size_t {
        if memcmp(
            &(*colorItem).av1C as *const avifCodecConfigurationBox
                as *const core::ffi::c_void,
            &(*gainmapItem).av1C as *const avifCodecConfigurationBox
                as *const core::ffi::c_void,
            ::core::mem::size_of::<avifCodecConfigurationBox>() as size_t,
        ) == 0
        {
            gainmapCodecConfigSize = 0 as uint32_t;
        } else {
            gainmapCodecConfigSize = codecConfigSize;
        }
        let result___51: avifResult = avifRWStreamWriteBits(
            s,
            gainmapCodecConfigSize,
            (if largeCodecConfigFlag != 0 {
                12 as core::ffi::c_int
            } else {
                3 as core::ffi::c_int
            }) as size_t,
        ) as avifResult;
        if result___51 as core::ffi::c_uint
            != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
        {
            return result___51;
        }
    }
    let result___52: avifResult = avifRWStreamWriteBits(
        s,
        codecConfigSize,
        (if largeCodecConfigFlag != 0 {
            12 as core::ffi::c_int
        } else {
            3 as core::ffi::c_int
        }) as size_t,
    ) as avifResult;
    if result___52 as core::ffi::c_uint
        != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
    {
        return result___52;
    }
    let result___53: avifResult = avifRWStreamWriteBits(
        s,
        ((*colorData).size as uint32_t).wrapping_sub(1 as uint32_t),
        (if largeItemDataFlag != 0 {
            28 as core::ffi::c_int
        } else {
            15 as core::ffi::c_int
        }) as size_t,
    ) as avifResult;
    if result___53 as core::ffi::c_uint
        != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
    {
        return result___53;
    }
    if hasAlpha != 0 {
        let result___54: avifResult = avifRWStreamWriteBits(
            s,
            (*alphaData).size as uint32_t,
            (if largeItemDataFlag != 0 {
                28 as core::ffi::c_int
            } else {
                15 as core::ffi::c_int
            }) as size_t,
        ) as avifResult;
        if result___54 as core::ffi::c_uint
            != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
        {
            return result___54;
        }
    }
    if hasAlpha != 0 && (*alphaData).size != 0 as size_t {
        if memcmp(
            &(*colorItem).av1C as *const avifCodecConfigurationBox
                as *const core::ffi::c_void,
            &(*alphaItem).av1C as *const avifCodecConfigurationBox
                as *const core::ffi::c_void,
            ::core::mem::size_of::<avifCodecConfigurationBox>() as size_t,
        ) == 0
        {
            alphaCodecConfigSize = 0 as uint32_t;
        } else {
            alphaCodecConfigSize = codecConfigSize;
        }
        let result___55: avifResult = avifRWStreamWriteBits(
            s,
            alphaCodecConfigSize,
            (if largeCodecConfigFlag != 0 {
                12 as core::ffi::c_int
            } else {
                3 as core::ffi::c_int
            }) as size_t,
        ) as avifResult;
        if result___55 as core::ffi::c_uint
            != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
        {
            return result___55;
        }
    }
    if (*image).exif.size != 0 || (*image).xmp.size != 0 {
        let result___56: avifResult = avifRWStreamWriteBits(
            s,
            0 as uint32_t,
            1 as size_t,
        ) as avifResult;
        if result___56 as core::ffi::c_uint
            != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
        {
            return result___56;
        }
    }
    if (*image).exif.size != 0 {
        let result___57: avifResult = avifRWStreamWriteBits(
            s,
            ((*image).exif.size as uint32_t).wrapping_sub(1 as uint32_t),
            (if largeMetadataFlag != 0 {
                20 as core::ffi::c_int
            } else {
                10 as core::ffi::c_int
            }) as size_t,
        ) as avifResult;
        if result___57 as core::ffi::c_uint
            != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
        {
            return result___57;
        }
    }
    if (*image).xmp.size != 0 {
        let result___58: avifResult = avifRWStreamWriteBits(
            s,
            ((*image).xmp.size as uint32_t).wrapping_sub(1 as uint32_t),
            (if largeMetadataFlag != 0 {
                20 as core::ffi::c_int
            } else {
                10 as core::ffi::c_int
            }) as size_t,
        ) as avifResult;
        if result___58 as core::ffi::c_uint
            != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
        {
            return result___58;
        }
    }
    if (*s).numUsedBitsInPartialByte != 0 as size_t {
        let result___59: avifResult = avifRWStreamWriteBits(
            s,
            0 as uint32_t,
            (8 as size_t).wrapping_sub((*s).numUsedBitsInPartialByte),
        ) as avifResult;
        if result___59 as core::ffi::c_uint
            != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
        {
            return result___59;
        }
    }
    let headerBytes: size_t = avifRWStreamOffset(s) as size_t;
    if codecConfigSize > 0 as uint32_t {
        let result___60: avifResult = writeCodecConfig(s, &(*colorItem).av1C)
            as avifResult;
        if result___60 as core::ffi::c_uint
            != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
        {
            return result___60;
        }
    }
    if hasAlpha != 0 && (*alphaData).size != 0 as size_t
        && alphaCodecConfigSize != 0 as uint32_t
    {
        let result___61: avifResult = writeCodecConfig(s, &(*alphaItem).av1C)
            as avifResult;
        if result___61 as core::ffi::c_uint
            != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
        {
            return result___61;
        }
    }
    if hasHdr != 0 && hasGainmap != 0 && gainmapCodecConfigSize != 0 as uint32_t {
        let result___62: avifResult = writeCodecConfig(s, &(*gainmapItem).av1C)
            as avifResult;
        if result___62 as core::ffi::c_uint
            != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
        {
            return result___62;
        }
    }
    if hasIcc != 0 {
        let result___63: avifResult = avifRWStreamWrite(
            s,
            (*image).icc.data as *const core::ffi::c_void,
            (*image).icc.size,
        ) as avifResult;
        if result___63 as core::ffi::c_uint
            != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
        {
            return result___63;
        }
    }
    if hasHdr != 0 && hasGainmap != 0 && tmapIccSize != 0 as size_t {
        let result___64: avifResult = avifRWStreamWrite(
            s,
            (*(*(*encoder).data).altImageMetadata).icc.data as *const core::ffi::c_void,
            tmapIccSize,
        ) as avifResult;
        if result___64 as core::ffi::c_uint
            != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
        {
            return result___64;
        }
    }
    if hasHdr != 0 && hasGainmap != 0 && gainmapMetadataSize != 0 as uint32_t {
        let result___65: avifResult = avifWriteGainmapMetadata(
            s,
            (*image).gainMap,
            &mut (*encoder).diag,
        ) as avifResult;
        if result___65 as core::ffi::c_uint
            != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
        {
            return result___65;
        }
    }
    if hasAlpha != 0 && (*alphaData).size != 0 as size_t {
        let result___66: avifResult = avifRWStreamWrite(
            s,
            (*alphaData).data as *const core::ffi::c_void,
            (*alphaData).size,
        ) as avifResult;
        if result___66 as core::ffi::c_uint
            != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
        {
            return result___66;
        }
    }
    if hasHdr != 0 && hasGainmap != 0 && (*gainmapData).size != 0 as size_t {
        let result___67: avifResult = avifRWStreamWrite(
            s,
            (*gainmapData).data as *const core::ffi::c_void,
            (*gainmapData).size,
        ) as avifResult;
        if result___67 as core::ffi::c_uint
            != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
        {
            return result___67;
        }
    }
    let result___68: avifResult = avifRWStreamWrite(
        s,
        (*colorData).data as *const core::ffi::c_void,
        (*colorData).size,
    ) as avifResult;
    if result___68 as core::ffi::c_uint
        != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
    {
        return result___68;
    }
    if (*image).exif.size != 0 {
        let result___69: avifResult = avifRWStreamWrite(
            s,
            (*image).exif.data as *const core::ffi::c_void,
            (*image).exif.size,
        ) as avifResult;
        if result___69 as core::ffi::c_uint
            != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
        {
            return result___69;
        }
    }
    if (*image).xmp.size != 0 {
        let result___70: avifResult = avifRWStreamWrite(
            s,
            (*image).xmp.data as *const core::ffi::c_void,
            (*image).xmp.size,
        ) as avifResult;
        if result___70 as core::ffi::c_uint
            != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
        {
            return result___70;
        }
    }
    let expectedChunkBytes: size_t = (codecConfigSize
        .wrapping_add(alphaCodecConfigSize)
        .wrapping_add(gainmapCodecConfigSize) as size_t)
        .wrapping_add((*image).icc.size)
        .wrapping_add(tmapIccSize)
        .wrapping_add(gainmapMetadataSize as size_t)
        .wrapping_add((if hasAlpha != 0 { (*alphaData).size } else { 0 as size_t }))
        .wrapping_add((if hasGainmap != 0 { (*gainmapData).size } else { 0 as size_t }))
        .wrapping_add((*colorData).size)
        .wrapping_add((*image).exif.size)
        .wrapping_add((*image).xmp.size);
    if !(avifRWStreamOffset(s) == headerBytes.wrapping_add(expectedChunkBytes)) {
        return AVIF_RESULT_INTERNAL_ERROR;
    }
    let result___71: avifResult = avifRWStreamFinishBox(s, mini) as avifResult;
    if result___71 as core::ffi::c_uint
        != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
    {
        return result___71;
    }
    return AVIF_RESULT_OK;
}
unsafe extern "C" fn avifRWStreamWriteProperties(
    dedup: *mut avifItemPropertyDedup,
    s: *mut avifRWStream,
    encoder: *const avifEncoder,
    imageMetadata: *const avifImage,
    altImageMetadata: *const avifImage,
) -> avifResult {
    let mut current_block_335: u64;
    let mut itemIndex: uint32_t = 0 as uint32_t;
    while itemIndex < (*(*encoder).data).items.count {
        let mut item: *mut avifEncoderItem = &mut *((*(*encoder).data).items.item)
            .offset(itemIndex as isize) as *mut avifEncoderItem;
        let isGrid: avifBool = ((*item).gridCols > 0 as uint32_t) as core::ffi::c_int;
        let mut hasIpmaToWrite: avifBool = (!((*item).codec).is_null() || isGrid != 0)
            as core::ffi::c_int;
        let isToneMappedImage: avifBool = (memcmp(
            ((*item).type_0).as_mut_ptr() as *const core::ffi::c_void,
            b"tmap\0" as *const u8 as *const core::ffi::c_char
                as *const core::ffi::c_void,
            4 as size_t,
        ) == 0) as core::ffi::c_int;
        if isToneMappedImage != 0 {
            hasIpmaToWrite = AVIF_TRUE as avifBool;
        }
        let isSampleTransformImage: avifBool = (memcmp(
            ((*item).type_0).as_mut_ptr() as *const core::ffi::c_void,
            b"sato\0" as *const u8 as *const core::ffi::c_char
                as *const core::ffi::c_void,
            4 as size_t,
        ) == 0) as core::ffi::c_int;
        if isSampleTransformImage != 0 {
            hasIpmaToWrite = AVIF_TRUE as avifBool;
        }
        (*item).associations.count = 0 as uint32_t;
        if !(hasIpmaToWrite == 0) {
            if (*item).dimgFromID != 0 && (*item).extraLayerCount == 0 as uint32_t {
                let mut parentItem: *mut avifEncoderItem = avifEncoderDataFindItemByID(
                    (*encoder).data as *mut avifEncoderData,
                    (*item).dimgFromID,
                );
                if !parentItem.is_null()
                    && memcmp(
                        ((*parentItem).type_0).as_mut_ptr() as *const core::ffi::c_void,
                        b"grid\0" as *const u8 as *const core::ffi::c_char
                            as *const core::ffi::c_void,
                        4 as size_t,
                    ) == 0
                {
                    let mut foundPreviousCell: avifBool = AVIF_FALSE;
                    let mut dedupIndex: uint32_t = 0 as uint32_t;
                    while dedupIndex < itemIndex {
                        let mut dedupItem: *mut avifEncoderItem = &mut *((*(*encoder)
                            .data)
                            .items
                            .item)
                            .offset(dedupIndex as isize) as *mut avifEncoderItem;
                        if (*item).dimgFromID == (*dedupItem).dimgFromID
                            && (*dedupItem).extraLayerCount == 0 as uint32_t
                        {
                            (*item).associations.count = 0 as uint32_t;
                            let mut associationIndex: uint32_t = 0 as uint32_t;
                            while associationIndex < (*dedupItem).associations.count {
                                let mut association: *mut avifItemPropertyAssociation = avifArrayPush(
                                    &mut (*item).associations
                                        as *mut avifItemPropertyAssociationArray
                                        as *mut core::ffi::c_void,
                                ) as *mut avifItemPropertyAssociation;
                                if association.is_null() {
                                    return AVIF_RESULT_OUT_OF_MEMORY;
                                }
                                *association = *((*dedupItem).associations.association)
                                    .offset(associationIndex as isize);
                                associationIndex = associationIndex.wrapping_add(1);
                            }
                            foundPreviousCell = AVIF_TRUE as avifBool;
                            break;
                        } else {
                            dedupIndex = dedupIndex.wrapping_add(1);
                        }
                    }
                    if foundPreviousCell != 0 {
                        current_block_335 = 0;
                    } else {
                        current_block_335 = 1;
                    }
                } else {
                    current_block_335 = 1;
                }
            } else {
                current_block_335 = 1;
            }
            match current_block_335 {
                0 => {}
                _ => {
                    let mut itemMetadata: *const avifImage = imageMetadata;
                    if isToneMappedImage != 0 {
                        itemMetadata = altImageMetadata;
                    } else if (*item).itemCategory as core::ffi::c_uint
                        == AVIF_ITEM_GAIN_MAP as core::ffi::c_int as core::ffi::c_uint
                    {
                        if !(!((*itemMetadata).gainMap).is_null()
                            && !((*(*itemMetadata).gainMap).image).is_null())
                        {
                            return AVIF_RESULT_INTERNAL_ERROR;
                        }
                        itemMetadata = (*(*itemMetadata).gainMap).image;
                    }
                    let mut imageWidth: uint32_t = (*itemMetadata).width;
                    let mut imageHeight: uint32_t = (*itemMetadata).height;
                    if isGrid != 0 {
                        imageWidth = (*item).gridWidth;
                        imageHeight = (*item).gridHeight;
                    }
                    avifItemPropertyDedupStart(dedup);
                    let mut ispe: avifBoxMarker = 0;
                    let result__: avifResult = avifRWStreamWriteFullBox(
                        &mut (*dedup).s,
                        b"ispe\0" as *const u8 as *const core::ffi::c_char,
                        0 as size_t,
                        0 as core::ffi::c_int,
                        0 as uint32_t,
                        &mut ispe,
                    ) as avifResult;
                    if result__ as core::ffi::c_uint
                        != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
                    {
                        return result__;
                    }
                    let result___0: avifResult = avifRWStreamWriteU32(
                        &mut (*dedup).s,
                        imageWidth,
                    ) as avifResult;
                    if result___0 as core::ffi::c_uint
                        != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
                    {
                        return result___0;
                    }
                    let result___1: avifResult = avifRWStreamWriteU32(
                        &mut (*dedup).s,
                        imageHeight,
                    ) as avifResult;
                    if result___1 as core::ffi::c_uint
                        != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
                    {
                        return result___1;
                    }
                    let result___2: avifResult = avifRWStreamFinishBox(
                        &mut (*dedup).s,
                        ispe,
                    ) as avifResult;
                    if result___2 as core::ffi::c_uint
                        != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
                    {
                        return result___2;
                    }
                    let result___3: avifResult = avifItemPropertyDedupFinish(
                        dedup,
                        s,
                        &mut (*item).associations,
                        0 as avifBool,
                    ) as avifResult;
                    if result___3 as core::ffi::c_uint
                        != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
                    {
                        return result___3;
                    }
                    let mut hasPixi: avifBool = AVIF_TRUE;
                    if isToneMappedImage != 0
                        && (*(*imageMetadata).gainMap).altDepth == 0 as uint32_t
                        && (*(*imageMetadata).gainMap).altPlaneCount == 0 as uint32_t
                    {
                        hasPixi = AVIF_FALSE as avifBool;
                    }
                    let isAlpha: avifBool = avifIsAlpha((*item).itemCategory)
                        as avifBool;
                    let mut depth: uint8_t = (*itemMetadata).depth as uint8_t;
                    if (*encoder).sampleTransformRecipe as core::ffi::c_uint
                        == AVIF_SAMPLE_TRANSFORM_BIT_DEPTH_EXTENSION_8B_8B
                            as core::ffi::c_int as core::ffi::c_uint
                        || (*encoder).sampleTransformRecipe as core::ffi::c_uint
                            == AVIF_SAMPLE_TRANSFORM_BIT_DEPTH_EXTENSION_12B_4B
                                as core::ffi::c_int as core::ffi::c_uint
                        || (*encoder).sampleTransformRecipe as core::ffi::c_uint
                            == AVIF_SAMPLE_TRANSFORM_BIT_DEPTH_EXTENSION_12B_8B_OVERLAP_4B
                                as core::ffi::c_int as core::ffi::c_uint
                    {
                        if (*item).itemCategory as core::ffi::c_uint
                            == AVIF_ITEM_SAMPLE_TRANSFORM as core::ffi::c_int
                                as core::ffi::c_uint
                        {
                            if !(depth as core::ffi::c_int == 16 as core::ffi::c_int) {
                                return AVIF_RESULT_INTERNAL_ERROR;
                            }
                        } else if (*encoder).sampleTransformRecipe as core::ffi::c_uint
                            == AVIF_SAMPLE_TRANSFORM_BIT_DEPTH_EXTENSION_8B_8B
                                as core::ffi::c_int as core::ffi::c_uint
                        {
                            depth = 8 as uint8_t;
                        } else if (*item).itemCategory as core::ffi::c_uint
                            == AVIF_ITEM_COLOR as core::ffi::c_int as core::ffi::c_uint
                            || (*item).itemCategory as core::ffi::c_uint
                                == AVIF_ITEM_ALPHA as core::ffi::c_int as core::ffi::c_uint
                        {
                            depth = 12 as uint8_t;
                        } else {
                            if !((*item).itemCategory as core::ffi::c_uint
                                == AVIF_ITEM_SAMPLE_TRANSFORM_INPUT_0_COLOR
                                    as core::ffi::c_int as core::ffi::c_uint
                                || (*item).itemCategory as core::ffi::c_uint
                                    == AVIF_ITEM_SAMPLE_TRANSFORM_INPUT_0_ALPHA
                                        as core::ffi::c_int as core::ffi::c_uint)
                            {
                                return AVIF_RESULT_INTERNAL_ERROR;
                            }
                            depth = 8 as uint8_t;
                        }
                    } else if !((*encoder).sampleTransformRecipe as core::ffi::c_uint
                        == AVIF_SAMPLE_TRANSFORM_NONE as core::ffi::c_int
                            as core::ffi::c_uint)
                    {
                        return AVIF_RESULT_NOT_IMPLEMENTED
                    }
                    if hasPixi != 0 {
                        avifItemPropertyDedupStart(dedup);
                        let mut channelCount: uint8_t = (if isAlpha != 0
                            || (*itemMetadata).yuvFormat as core::ffi::c_uint
                                == AVIF_PIXEL_FORMAT_YUV400 as core::ffi::c_int
                                    as core::ffi::c_uint
                        {
                            1 as core::ffi::c_int
                        } else {
                            3 as core::ffi::c_int
                        }) as uint8_t;
                        let mut pixi: avifBoxMarker = 0;
                        let mut flags: uint32_t = 0 as uint32_t;
                        let result___4: avifResult = avifRWStreamWriteFullBox(
                            &mut (*dedup).s,
                            b"pixi\0" as *const u8 as *const core::ffi::c_char,
                            0 as size_t,
                            0 as core::ffi::c_int,
                            flags,
                            &mut pixi,
                        ) as avifResult;
                        if result___4 as core::ffi::c_uint
                            != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
                        {
                            return result___4;
                        }
                        let result___5: avifResult = avifRWStreamWriteU8(
                            &mut (*dedup).s,
                            channelCount,
                        ) as avifResult;
                        if result___5 as core::ffi::c_uint
                            != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
                        {
                            return result___5;
                        }
                        let mut chan: uint8_t = 0 as uint8_t;
                        while (chan as core::ffi::c_int)
                            < channelCount as core::ffi::c_int
                        {
                            let result___6: avifResult = avifRWStreamWriteU8(
                                &mut (*dedup).s,
                                depth,
                            ) as avifResult;
                            if result___6 as core::ffi::c_uint
                                != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
                            {
                                return result___6;
                            }
                            chan = chan.wrapping_add(1);
                        }
                        let result___7: avifResult = avifRWStreamFinishBox(
                            &mut (*dedup).s,
                            pixi,
                        ) as avifResult;
                        if result___7 as core::ffi::c_uint
                            != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
                        {
                            return result___7;
                        }
                        let result___8: avifResult = avifItemPropertyDedupFinish(
                            dedup,
                            s,
                            &mut (*item).associations,
                            0 as avifBool,
                        ) as avifResult;
                        if result___8 as core::ffi::c_uint
                            != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
                        {
                            return result___8;
                        }
                    }
                    if !((*item).codec).is_null() {
                        avifItemPropertyDedupStart(dedup);
                        let result___9: avifResult = writeConfigBox(
                            &mut (*dedup).s,
                            &mut (*item).av1C,
                            (*(*encoder).data).configPropName,
                        ) as avifResult;
                        if result___9 as core::ffi::c_uint
                            != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
                        {
                            return result___9;
                        }
                        let result___10: avifResult = avifItemPropertyDedupFinish(
                            dedup,
                            s,
                            &mut (*item).associations,
                            1 as avifBool,
                        ) as avifResult;
                        if result___10 as core::ffi::c_uint
                            != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
                        {
                            return result___10;
                        }
                    }
                    if isAlpha != 0 {
                        avifItemPropertyDedupStart(dedup);
                        let mut auxC: avifBoxMarker = 0;
                        let result___11: avifResult = avifRWStreamWriteFullBox(
                            &mut (*dedup).s,
                            b"auxC\0" as *const u8 as *const core::ffi::c_char,
                            0 as size_t,
                            0 as core::ffi::c_int,
                            0 as uint32_t,
                            &mut auxC,
                        ) as avifResult;
                        if result___11 as core::ffi::c_uint
                            != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
                        {
                            return result___11;
                        }
                        let result___12: avifResult = avifRWStreamWriteChars(
                            &mut (*dedup).s,
                            core::ptr::addr_of!(alphaURN).cast::<core::ffi::c_char>(),
                            alphaURNSize,
                        ) as avifResult;
                        if result___12 as core::ffi::c_uint
                            != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
                        {
                            return result___12;
                        }
                        let result___13: avifResult = avifRWStreamFinishBox(
                            &mut (*dedup).s,
                            auxC,
                        ) as avifResult;
                        if result___13 as core::ffi::c_uint
                            != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
                        {
                            return result___13;
                        }
                        let result___14: avifResult = avifItemPropertyDedupFinish(
                            dedup,
                            s,
                            &mut (*item).associations,
                            0 as avifBool,
                        ) as avifResult;
                        if result___14 as core::ffi::c_uint
                            != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
                        {
                            return result___14;
                        }
                    } else if (*item).itemCategory as core::ffi::c_uint
                        == AVIF_ITEM_COLOR as core::ffi::c_int as core::ffi::c_uint
                    {
                        let result___15: avifResult = avifEncoderWriteColorProperties(
                            s,
                            itemMetadata,
                            &mut (*item).associations,
                            dedup,
                        ) as avifResult;
                        if result___15 as core::ffi::c_uint
                            != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
                        {
                            return result___15;
                        }
                        let result___16: avifResult = avifEncoderWriteHDRProperties(
                            &mut (*dedup).s,
                            s,
                            itemMetadata,
                            &mut (*item).associations,
                            dedup,
                        ) as avifResult;
                        if result___16 as core::ffi::c_uint
                            != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
                        {
                            return result___16;
                        }
                    } else if (*item).itemCategory as core::ffi::c_uint
                        == AVIF_ITEM_GAIN_MAP as core::ffi::c_int as core::ffi::c_uint
                    {
                        let result___17: avifResult = avifEncoderWriteNclxProperty(
                            &mut (*dedup).s,
                            s,
                            itemMetadata,
                            &mut (*item).associations,
                            dedup,
                        ) as avifResult;
                        if result___17 as core::ffi::c_uint
                            != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
                        {
                            return result___17;
                        }
                        let result___18: avifResult = avifEncoderWritePaspProperty(
                            &mut (*dedup).s,
                            s,
                            imageMetadata,
                            &mut (*item).associations,
                            dedup,
                        ) as avifResult;
                        if result___18 as core::ffi::c_uint
                            != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
                        {
                            return result___18;
                        }
                    }
                    if (*item).extraLayerCount > 0 as uint32_t {
                        avifItemPropertyDedupStart(dedup);
                        let mut a1lx: avifBoxMarker = 0;
                        let result___19: avifResult = avifRWStreamWriteBox(
                            &mut (*dedup).s,
                            b"a1lx\0" as *const u8 as *const core::ffi::c_char,
                            0 as size_t,
                            &mut a1lx,
                        ) as avifResult;
                        if result___19 as core::ffi::c_uint
                            != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
                        {
                            return result___19;
                        }
                        let mut layerSize: [uint32_t; 3] = [
                            0 as core::ffi::c_int as uint32_t,
                            0,
                            0,
                        ];
                        let mut largeSize: avifBool = AVIF_FALSE;
                        let mut validLayer: uint32_t = 0 as uint32_t;
                        while validLayer < (*item).extraLayerCount {
                            let mut size: uint32_t = (*((*(*item).encodeOutput)
                                .samples
                                .sample)
                                .offset(validLayer as isize))
                                .data
                                .size as uint32_t;
                            layerSize[validLayer as usize] = size;
                            if size > 0xffff as uint32_t {
                                largeSize = AVIF_TRUE as avifBool;
                            }
                            validLayer = validLayer.wrapping_add(1);
                        }
                        let result___20: avifResult = avifRWStreamWriteBits(
                            &mut (*dedup).s,
                            0 as uint32_t,
                            7 as size_t,
                        ) as avifResult;
                        if result___20 as core::ffi::c_uint
                            != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
                        {
                            return result___20;
                        }
                        let result___21: avifResult = avifRWStreamWriteBits(
                            &mut (*dedup).s,
                            (if largeSize != 0 {
                                1 as core::ffi::c_int
                            } else {
                                0 as core::ffi::c_int
                            }) as uint32_t,
                            1 as size_t,
                        ) as avifResult;
                        if result___21 as core::ffi::c_uint
                            != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
                        {
                            return result___21;
                        }
                        let mut layer: uint32_t = 0 as uint32_t;
                        while layer
                            < (AVIF_MAX_AV1_LAYER_COUNT - 1 as core::ffi::c_int)
                                as uint32_t
                        {
                            if largeSize != 0 {
                                let result___22: avifResult = avifRWStreamWriteU32(
                                    &mut (*dedup).s,
                                    layerSize[layer as usize],
                                ) as avifResult;
                                if result___22 as core::ffi::c_uint
                                    != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
                                {
                                    return result___22;
                                }
                            } else {
                                let result___23: avifResult = avifRWStreamWriteU16(
                                    &mut (*dedup).s,
                                    layerSize[layer as usize] as uint16_t,
                                ) as avifResult;
                                if result___23 as core::ffi::c_uint
                                    != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
                                {
                                    return result___23;
                                }
                            }
                            layer = layer.wrapping_add(1);
                        }
                        let result___24: avifResult = avifRWStreamFinishBox(
                            &mut (*dedup).s,
                            a1lx,
                        ) as avifResult;
                        if result___24 as core::ffi::c_uint
                            != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
                        {
                            return result___24;
                        }
                        let result___25: avifResult = avifItemPropertyDedupFinish(
                            dedup,
                            s,
                            &mut (*item).associations,
                            0 as avifBool,
                        ) as avifResult;
                        if result___25 as core::ffi::c_uint
                            != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
                        {
                            return result___25;
                        }
                    }
                    let mut i: size_t = 0 as size_t;
                    while i < (*itemMetadata).numProperties {
                        avifItemPropertyDedupStart(dedup);
                        let mut prop: *const avifImageItemProperty = &mut *((*itemMetadata)
                            .properties)
                            .offset(i as isize) as *mut avifImageItemProperty;
                        let mut propMarker: avifBoxMarker = 0;
                        let result___26: avifResult = avifRWStreamWriteBox(
                            &mut (*dedup).s,
                            ((*prop).boxtype).as_ptr() as *const core::ffi::c_char,
                            0 as size_t,
                            &mut propMarker,
                        ) as avifResult;
                        if result___26 as core::ffi::c_uint
                            != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
                        {
                            return result___26;
                        }
                        if memcmp(
                            ((*prop).boxtype).as_ptr() as *const core::ffi::c_void,
                            b"uuid\0" as *const u8 as *const core::ffi::c_char
                                as *const core::ffi::c_void,
                            4 as size_t,
                        ) == 0 as core::ffi::c_int
                        {
                            let result___27: avifResult = avifRWStreamWrite(
                                &mut (*dedup).s,
                                ((*prop).usertype).as_ptr() as *const core::ffi::c_void,
                                16 as size_t,
                            ) as avifResult;
                            if result___27 as core::ffi::c_uint
                                != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
                            {
                                return result___27;
                            }
                        }
                        let result___28: avifResult = avifRWStreamWrite(
                            &mut (*dedup).s,
                            (*prop).boxPayload.data as *const core::ffi::c_void,
                            (*prop).boxPayload.size,
                        ) as avifResult;
                        if result___28 as core::ffi::c_uint
                            != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
                        {
                            return result___28;
                        }
                        let result___29: avifResult = avifRWStreamFinishBox(
                            &mut (*dedup).s,
                            propMarker,
                        ) as avifResult;
                        if result___29 as core::ffi::c_uint
                            != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
                        {
                            return result___29;
                        }
                        let result___30: avifResult = avifItemPropertyDedupFinish(
                            dedup,
                            s,
                            &mut (*item).associations,
                            0 as avifBool,
                        ) as avifResult;
                        if result___30 as core::ffi::c_uint
                            != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
                        {
                            return result___30;
                        }
                        i = i.wrapping_add(1);
                    }
                    if (*item).itemCategory as core::ffi::c_uint
                        == AVIF_ITEM_COLOR as core::ffi::c_int as core::ffi::c_uint
                    {
                        let result___31: avifResult = avifEncoderWriteTransformativeProperties(
                            &mut (*dedup).s,
                            s,
                            itemMetadata,
                            &mut (*item).associations,
                            dedup,
                        ) as avifResult;
                        if result___31 as core::ffi::c_uint
                            != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
                        {
                            return result___31;
                        }
                    } else if (*item).itemCategory as core::ffi::c_uint
                        == AVIF_ITEM_ALPHA as core::ffi::c_int as core::ffi::c_uint
                    {
                        let result___32: avifResult = avifEncoderWriteTransformativeProperties(
                            &mut (*dedup).s,
                            s,
                            itemMetadata,
                            &mut (*item).associations,
                            dedup,
                        ) as avifResult;
                        if result___32 as core::ffi::c_uint
                            != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
                        {
                            return result___32;
                        }
                    } else if (*item).itemCategory as core::ffi::c_uint
                        == AVIF_ITEM_GAIN_MAP as core::ffi::c_int as core::ffi::c_uint
                    {
                        if (*imageMetadata).transformFlags
                            & AVIF_TRANSFORM_CLAP as core::ffi::c_int
                                as avifTransformFlags != 0
                        {
                            if !((*imageMetadata).width != (*itemMetadata).width
                                || (*imageMetadata).height != (*itemMetadata).height)
                            {
                                return AVIF_RESULT_NOT_IMPLEMENTED;
                            }
                        }
                        if !((*itemMetadata).transformFlags
                            == AVIF_TRANSFORM_NONE as core::ffi::c_int
                                as avifTransformFlags)
                        {
                            return AVIF_RESULT_ENCODE_GAIN_MAP_FAILED;
                        }
                        let result___33: avifResult = avifEncoderWriteTransformativeProperties(
                            &mut (*dedup).s,
                            s,
                            imageMetadata,
                            &mut (*item).associations,
                            dedup,
                        ) as avifResult;
                        if result___33 as core::ffi::c_uint
                            != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
                        {
                            return result___33;
                        }
                    }
                }
            }
        }
        itemIndex = itemIndex.wrapping_add(1);
    }
    return AVIF_RESULT_OK;
}
#[no_mangle]
pub unsafe extern "C" fn avifEncoderFinish(
    mut encoder: *mut avifEncoder,
    mut output: *mut avifRWData,
) -> avifResult {
    avifDiagnosticsClearError(&mut (*encoder).diag);
    if (*(*encoder).data).items.count == 0 as uint32_t {
        return AVIF_RESULT_NO_CONTENT;
    }
    let codecType: avifCodecType = avifEncoderGetCodecType(encoder) as avifCodecType;
    if codecType as core::ffi::c_uint
        == AVIF_CODEC_TYPE_UNKNOWN as core::ffi::c_int as core::ffi::c_uint
    {
        return AVIF_RESULT_NO_CODEC_AVAILABLE;
    }
    let mut itemIndex: uint32_t = 0 as uint32_t;
    while itemIndex < (*(*encoder).data).items.count {
        let mut item: *mut avifEncoderItem = &mut *((*(*encoder).data).items.item)
            .offset(itemIndex as isize) as *mut avifEncoderItem;
        if !((*item).codec).is_null() {
            if ((*(*item).codec).encodeFinish)
                .expect(
                    "non-null function pointer",
                )((*item).codec as *mut avifCodec, (*item).encodeOutput) == 0
            {
                return avifGetErrorForItemCategory((*item).itemCategory);
            }
            if (*(*item).encodeOutput).samples.count != (*(*encoder).data).frames.count {
                return avifGetErrorForItemCategory((*item).itemCategory);
            }
            if (*item).extraLayerCount > 0 as uint32_t
                && (*(*item).encodeOutput).samples.count
                    != ((*item).extraLayerCount).wrapping_add(1 as uint32_t)
            {
                avifDiagnosticsPrintf(
                    &mut (*encoder).diag as *mut avifDiagnostics,
                    b"Expected %u frames given to avifEncoderAddImage() to encode this layered image according to extraLayerCount, but got %u frames.\0"
                        as *const u8 as *const core::ffi::c_char,
                    ((*item).extraLayerCount).wrapping_add(1 as uint32_t),
                    (*(*item).encodeOutput).samples.count,
                );
                return AVIF_RESULT_INVALID_ARGUMENT;
            }
        }
        itemIndex = itemIndex.wrapping_add(1);
    }
    let mut itemIndex_0: uint32_t = 0 as uint32_t;
    while itemIndex_0 < (*(*encoder).data).items.count {
        let mut item_0: *mut avifEncoderItem = &mut *((*(*encoder).data).items.item)
            .offset(itemIndex_0 as isize) as *mut avifEncoderItem;
        if (*(*item_0).encodeOutput).samples.count > 0 as uint32_t {
            let mut firstSample: *const avifEncodeSample = &mut *((*(*item_0)
                .encodeOutput)
                .samples
                .sample)
                .offset(0 as core::ffi::c_int as isize) as *mut avifEncodeSample;
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
                &(*firstSample).data as *const avifRWData as *const avifROData,
                codecType,
            ) == 0
            {
                return avifGetErrorForItemCategory((*item_0).itemCategory);
            }
            (*item_0).av1C = sequenceHeader.av1C;
        }
        itemIndex_0 = itemIndex_0.wrapping_add(1);
    }
    if (*encoder).headerFormat as core::ffi::c_int & AVIF_HEADER_MINI as core::ffi::c_int
        != 0 && avifEncoderIsMiniCompatible(encoder) != 0
    {
        let result__: avifResult = avifEncoderWriteFileTypeBoxAndMiniBox(encoder, output)
            as avifResult;
        if result__ as core::ffi::c_uint
            != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
        {
            return result__;
        }
        return AVIF_RESULT_OK;
    }
    let mut imageMetadata: *const avifImage = (*(*encoder).data).imageMetadata;
    let mut now: uint64_t = time(0 as *mut time_t) as uint64_t;
    let mut modificationTime: uint64_t = if (*encoder).modificationTime != 0 as uint64_t
    {
        (*encoder).modificationTime
    } else {
        now
    };
    let mut creationTime: uint64_t = if (*encoder).creationTime != 0 as uint64_t {
        (*encoder).creationTime
    } else {
        modificationTime
    };
    creationTime = creationTime.wrapping_add(2082844800 as uint64_t);
    modificationTime = modificationTime.wrapping_add(2082844800 as uint64_t);
    let mut s: avifRWStream = avifRWStream {
        raw: 0 as *mut avifRWData,
        offset: 0,
        numUsedBitsInPartialByte: 0,
    };
    avifRWStreamStart(&mut s, output);
    let isSequence: avifBool = ((*encoder).extraLayerCount == 0 as uint32_t
        && (*(*encoder).data).frames.count > 1 as uint32_t) as core::ffi::c_int;
    let mut majorBrand: *const core::ffi::c_char = b"avif\0" as *const u8
        as *const core::ffi::c_char;
    if isSequence != 0 {
        majorBrand = b"avis\0" as *const u8 as *const core::ffi::c_char;
    }
    let mut minorVersion: uint32_t = 0 as uint32_t;
    let mut useAvioBrand: avifBool = 0;
    if isSequence != 0 {
        useAvioBrand = AVIF_FALSE as avifBool;
        let mut itemIndex_1: uint32_t = 0 as uint32_t;
        while itemIndex_1 < (*(*encoder).data).items.count {
            let mut item_1: *mut avifEncoderItem = &mut *((*(*encoder).data).items.item)
                .offset(itemIndex_1 as isize) as *mut avifEncoderItem;
            if !((*(*item_1).encodeOutput).samples.count == 0 as uint32_t) {
                let mut onlySyncSamples: avifBool = AVIF_TRUE;
                let mut sampleIndex: uint32_t = 0 as uint32_t;
                while sampleIndex < (*(*item_1).encodeOutput).samples.count {
                    if (*((*(*item_1).encodeOutput).samples.sample)
                        .offset(sampleIndex as isize))
                        .sync == 0
                    {
                        onlySyncSamples = AVIF_FALSE as avifBool;
                        break;
                    } else {
                        sampleIndex = sampleIndex.wrapping_add(1);
                    }
                }
                if onlySyncSamples != 0 {
                    useAvioBrand = AVIF_TRUE as avifBool;
                    break;
                }
            }
            itemIndex_1 = itemIndex_1.wrapping_add(1);
        }
    } else {
        useAvioBrand = AVIF_FALSE as avifBool;
    }
    let mut ftyp: avifBoxMarker = 0;
    let result___0: avifResult = avifRWStreamWriteBox(
        &mut s,
        b"ftyp\0" as *const u8 as *const core::ffi::c_char,
        0 as size_t,
        &mut ftyp,
    ) as avifResult;
    if result___0 as core::ffi::c_uint
        != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
    {
        return result___0;
    }
    let result___1: avifResult = avifRWStreamWriteChars(&mut s, majorBrand, 4 as size_t)
        as avifResult;
    if result___1 as core::ffi::c_uint
        != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
    {
        return result___1;
    }
    let result___2: avifResult = avifRWStreamWriteU32(&mut s, minorVersion)
        as avifResult;
    if result___2 as core::ffi::c_uint
        != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
    {
        return result___2;
    }
    let result___3: avifResult = avifRWStreamWriteChars(
        &mut s,
        b"avif\0" as *const u8 as *const core::ffi::c_char,
        4 as size_t,
    ) as avifResult;
    if result___3 as core::ffi::c_uint
        != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
    {
        return result___3;
    }
    if useAvioBrand != 0 {
        let result___4: avifResult = avifRWStreamWriteChars(
            &mut s,
            b"avio\0" as *const u8 as *const core::ffi::c_char,
            4 as size_t,
        ) as avifResult;
        if result___4 as core::ffi::c_uint
            != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
        {
            return result___4;
        }
    }
    if isSequence != 0 {
        let result___5: avifResult = avifRWStreamWriteChars(
            &mut s,
            b"avis\0" as *const u8 as *const core::ffi::c_char,
            4 as size_t,
        ) as avifResult;
        if result___5 as core::ffi::c_uint
            != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
        {
            return result___5;
        }
        let result___6: avifResult = avifRWStreamWriteChars(
            &mut s,
            b"msf1\0" as *const u8 as *const core::ffi::c_char,
            4 as size_t,
        ) as avifResult;
        if result___6 as core::ffi::c_uint
            != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
        {
            return result___6;
        }
        let result___7: avifResult = avifRWStreamWriteChars(
            &mut s,
            b"iso8\0" as *const u8 as *const core::ffi::c_char,
            4 as size_t,
        ) as avifResult;
        if result___7 as core::ffi::c_uint
            != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
        {
            return result___7;
        }
    }
    let result___8: avifResult = avifRWStreamWriteChars(
        &mut s,
        b"mif1\0" as *const u8 as *const core::ffi::c_char,
        4 as size_t,
    ) as avifResult;
    if result___8 as core::ffi::c_uint
        != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
    {
        return result___8;
    }
    let result___9: avifResult = avifRWStreamWriteChars(
        &mut s,
        b"miaf\0" as *const u8 as *const core::ffi::c_char,
        4 as size_t,
    ) as avifResult;
    if result___9 as core::ffi::c_uint
        != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
    {
        return result___9;
    }
    if (*imageMetadata).depth == 8 as uint32_t
        || (*imageMetadata).depth == 10 as uint32_t
    {
        if (*imageMetadata).yuvFormat as core::ffi::c_uint
            == AVIF_PIXEL_FORMAT_YUV420 as core::ffi::c_int as core::ffi::c_uint
        {
            let result___10: avifResult = avifRWStreamWriteChars(
                &mut s,
                b"MA1B\0" as *const u8 as *const core::ffi::c_char,
                4 as size_t,
            ) as avifResult;
            if result___10 as core::ffi::c_uint
                != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
            {
                return result___10;
            }
        } else if (*imageMetadata).yuvFormat as core::ffi::c_uint
            == AVIF_PIXEL_FORMAT_YUV444 as core::ffi::c_int as core::ffi::c_uint
        {
            let result___11: avifResult = avifRWStreamWriteChars(
                &mut s,
                b"MA1A\0" as *const u8 as *const core::ffi::c_char,
                4 as size_t,
            ) as avifResult;
            if result___11 as core::ffi::c_uint
                != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
            {
                return result___11;
            }
        }
    }
    let mut itemIndex_2: uint32_t = 0 as uint32_t;
    while itemIndex_2 < (*(*encoder).data).items.count {
        if memcmp(
            ((*((*(*encoder).data).items.item).offset(itemIndex_2 as isize)).type_0)
                .as_mut_ptr() as *const core::ffi::c_void,
            b"tmap\0" as *const u8 as *const core::ffi::c_char
                as *const core::ffi::c_void,
            4 as size_t,
        ) == 0
        {
            let result___12: avifResult = avifRWStreamWriteChars(
                &mut s,
                b"tmap\0" as *const u8 as *const core::ffi::c_char,
                4 as size_t,
            ) as avifResult;
            if result___12 as core::ffi::c_uint
                != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
            {
                return result___12;
            }
            break;
        } else {
            itemIndex_2 = itemIndex_2.wrapping_add(1);
        }
    }
    let result___13: avifResult = avifRWStreamFinishBox(&mut s, ftyp) as avifResult;
    if result___13 as core::ffi::c_uint
        != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
    {
        return result___13;
    }
    let mut meta: avifBoxMarker = 0;
    let result___14: avifResult = avifRWStreamWriteFullBox(
        &mut s,
        b"meta\0" as *const u8 as *const core::ffi::c_char,
        0 as size_t,
        0 as core::ffi::c_int,
        0 as uint32_t,
        &mut meta,
    ) as avifResult;
    if result___14 as core::ffi::c_uint
        != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
    {
        return result___14;
    }
    let result___15: avifResult = avifRWStreamWriteHandlerBox(
        &mut s,
        b"pict\0" as *const u8 as *const core::ffi::c_char,
    ) as avifResult;
    if result___15 as core::ffi::c_uint
        != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
    {
        return result___15;
    }
    let largeItemIDs: avifBool = ((*(*encoder).data).lastItemID > UINT16_MAX as uint32_t
        || (*(*encoder).data).items.count > UINT16_MAX as uint32_t) as core::ffi::c_int;
    if (*(*encoder).data).primaryItemID != 0 as uint32_t {
        if largeItemIDs != 0 {
            let result___16: avifResult = avifRWStreamWriteFullBox(
                &mut s,
                b"pitm\0" as *const u8 as *const core::ffi::c_char,
                ::core::mem::size_of::<uint32_t>() as size_t,
                1 as core::ffi::c_int,
                0 as uint32_t,
                0 as *mut avifBoxMarker,
            ) as avifResult;
            if result___16 as core::ffi::c_uint
                != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
            {
                return result___16;
            }
            let result___17: avifResult = avifRWStreamWriteU32(
                &mut s,
                (*(*encoder).data).primaryItemID,
            ) as avifResult;
            if result___17 as core::ffi::c_uint
                != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
            {
                return result___17;
            }
        } else {
            let result___18: avifResult = avifRWStreamWriteFullBox(
                &mut s,
                b"pitm\0" as *const u8 as *const core::ffi::c_char,
                ::core::mem::size_of::<uint16_t>() as size_t,
                0 as core::ffi::c_int,
                0 as uint32_t,
                0 as *mut avifBoxMarker,
            ) as avifResult;
            if result___18 as core::ffi::c_uint
                != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
            {
                return result___18;
            }
            let result___19: avifResult = avifRWStreamWriteU16(
                &mut s,
                (*(*encoder).data).primaryItemID as uint16_t,
            ) as avifResult;
            if result___19 as core::ffi::c_uint
                != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
            {
                return result___19;
            }
        }
    }
    let mut iloc: avifBoxMarker = 0;
    let result___20: avifResult = avifRWStreamWriteFullBox(
        &mut s,
        b"iloc\0" as *const u8 as *const core::ffi::c_char,
        0 as size_t,
        if largeItemIDs != 0 { 2 as core::ffi::c_int } else { 0 as core::ffi::c_int },
        0 as uint32_t,
        &mut iloc,
    ) as avifResult;
    if result___20 as core::ffi::c_uint
        != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
    {
        return result___20;
    }
    let result___21: avifResult = avifRWStreamWriteBits(
        &mut s,
        4 as uint32_t,
        4 as size_t,
    ) as avifResult;
    if result___21 as core::ffi::c_uint
        != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
    {
        return result___21;
    }
    let result___22: avifResult = avifRWStreamWriteBits(
        &mut s,
        4 as uint32_t,
        4 as size_t,
    ) as avifResult;
    if result___22 as core::ffi::c_uint
        != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
    {
        return result___22;
    }
    let result___23: avifResult = avifRWStreamWriteBits(
        &mut s,
        0 as uint32_t,
        4 as size_t,
    ) as avifResult;
    if result___23 as core::ffi::c_uint
        != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
    {
        return result___23;
    }
    let result___24: avifResult = avifRWStreamWriteBits(
        &mut s,
        0 as uint32_t,
        4 as size_t,
    ) as avifResult;
    if result___24 as core::ffi::c_uint
        != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
    {
        return result___24;
    }
    if largeItemIDs != 0 {
        let result___25: avifResult = avifRWStreamWriteU32(
            &mut s,
            (*(*encoder).data).items.count,
        ) as avifResult;
        if result___25 as core::ffi::c_uint
            != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
        {
            return result___25;
        }
    } else {
        let result___26: avifResult = avifRWStreamWriteU16(
            &mut s,
            (*(*encoder).data).items.count as uint16_t,
        ) as avifResult;
        if result___26 as core::ffi::c_uint
            != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
        {
            return result___26;
        }
    }
    let mut itemIndex_3: uint32_t = 0 as uint32_t;
    while itemIndex_3 < (*(*encoder).data).items.count {
        let mut item_2: *mut avifEncoderItem = &mut *((*(*encoder).data).items.item)
            .offset(itemIndex_3 as isize) as *mut avifEncoderItem;
        if largeItemIDs != 0 {
            let result___27: avifResult = avifRWStreamWriteU32(&mut s, (*item_2).id)
                as avifResult;
            if result___27 as core::ffi::c_uint
                != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
            {
                return result___27;
            }
            let result___28: avifResult = avifRWStreamWriteBits(
                &mut s,
                0 as uint32_t,
                12 as size_t,
            ) as avifResult;
            if result___28 as core::ffi::c_uint
                != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
            {
                return result___28;
            }
            let result___29: avifResult = avifRWStreamWriteBits(
                &mut s,
                0 as uint32_t,
                4 as size_t,
            ) as avifResult;
            if result___29 as core::ffi::c_uint
                != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
            {
                return result___29;
            }
        } else {
            let result___30: avifResult = avifRWStreamWriteU16(
                &mut s,
                (*item_2).id as uint16_t,
            ) as avifResult;
            if result___30 as core::ffi::c_uint
                != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
            {
                return result___30;
            }
        }
        let result___31: avifResult = avifRWStreamWriteU16(&mut s, 0 as uint16_t)
            as avifResult;
        if result___31 as core::ffi::c_uint
            != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
        {
            return result___31;
        }
        if (*item_2).extraLayerCount > 0 as uint32_t {
            let mut layerCount: uint32_t = ((*item_2).extraLayerCount)
                .wrapping_add(1 as uint32_t);
            let result___32: avifResult = avifRWStreamWriteU16(
                &mut s,
                layerCount as uint16_t,
            ) as avifResult;
            if result___32 as core::ffi::c_uint
                != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
            {
                return result___32;
            }
            let mut i: uint32_t = 0 as uint32_t;
            while i < layerCount {
                let result___33: avifResult = avifEncoderItemAddMdatFixup(item_2, &mut s)
                    as avifResult;
                if result___33 as core::ffi::c_uint
                    != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
                {
                    return result___33;
                }
                let result___34: avifResult = avifRWStreamWriteU32(&mut s, 0 as uint32_t)
                    as avifResult;
                if result___34 as core::ffi::c_uint
                    != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
                {
                    return result___34;
                }
                let result___35: avifResult = avifRWStreamWriteU32(
                    &mut s,
                    (*((*(*item_2).encodeOutput).samples.sample).offset(i as isize))
                        .data
                        .size as uint32_t,
                ) as avifResult;
                if result___35 as core::ffi::c_uint
                    != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
                {
                    return result___35;
                }
                i = i.wrapping_add(1);
            }
        } else {
            let mut contentSize: uint32_t = (*item_2).metadataPayload.size as uint32_t;
            if (*(*item_2).encodeOutput).samples.count > 0 as uint32_t {
                contentSize = (*((*(*item_2).encodeOutput).samples.sample)
                    .offset(0 as core::ffi::c_int as isize))
                    .data
                    .size as uint32_t;
            }
            let result___36: avifResult = avifRWStreamWriteU16(&mut s, 1 as uint16_t)
                as avifResult;
            if result___36 as core::ffi::c_uint
                != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
            {
                return result___36;
            }
            let result___37: avifResult = avifEncoderItemAddMdatFixup(item_2, &mut s)
                as avifResult;
            if result___37 as core::ffi::c_uint
                != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
            {
                return result___37;
            }
            let result___38: avifResult = avifRWStreamWriteU32(&mut s, 0 as uint32_t)
                as avifResult;
            if result___38 as core::ffi::c_uint
                != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
            {
                return result___38;
            }
            let result___39: avifResult = avifRWStreamWriteU32(&mut s, contentSize)
                as avifResult;
            if result___39 as core::ffi::c_uint
                != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
            {
                return result___39;
            }
        }
        itemIndex_3 = itemIndex_3.wrapping_add(1);
    }
    let result___40: avifResult = avifRWStreamFinishBox(&mut s, iloc) as avifResult;
    if result___40 as core::ffi::c_uint
        != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
    {
        return result___40;
    }
    let mut iinf: avifBoxMarker = 0;
    let result___41: avifResult = avifRWStreamWriteFullBox(
        &mut s,
        b"iinf\0" as *const u8 as *const core::ffi::c_char,
        0 as size_t,
        if largeItemIDs != 0 { 1 as core::ffi::c_int } else { 0 as core::ffi::c_int },
        0 as uint32_t,
        &mut iinf,
    ) as avifResult;
    if result___41 as core::ffi::c_uint
        != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
    {
        return result___41;
    }
    if largeItemIDs != 0 {
        let result___42: avifResult = avifRWStreamWriteU32(
            &mut s,
            (*(*encoder).data).items.count,
        ) as avifResult;
        if result___42 as core::ffi::c_uint
            != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
        {
            return result___42;
        }
    } else {
        let result___43: avifResult = avifRWStreamWriteU16(
            &mut s,
            (*(*encoder).data).items.count as uint16_t,
        ) as avifResult;
        if result___43 as core::ffi::c_uint
            != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
        {
            return result___43;
        }
    }
    let mut itemIndex_4: uint32_t = 0 as uint32_t;
    while itemIndex_4 < (*(*encoder).data).items.count {
        let mut item_3: *mut avifEncoderItem = &mut *((*(*encoder).data).items.item)
            .offset(itemIndex_4 as isize) as *mut avifEncoderItem;
        let mut flags: uint32_t = (if (*item_3).hiddenImage != 0 {
            1 as core::ffi::c_int
        } else {
            0 as core::ffi::c_int
        }) as uint32_t;
        let mut infe: avifBoxMarker = 0;
        let result___44: avifResult = avifRWStreamWriteFullBox(
            &mut s,
            b"infe\0" as *const u8 as *const core::ffi::c_char,
            0 as size_t,
            if largeItemIDs != 0 {
                3 as core::ffi::c_int
            } else {
                2 as core::ffi::c_int
            },
            flags,
            &mut infe,
        ) as avifResult;
        if result___44 as core::ffi::c_uint
            != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
        {
            return result___44;
        }
        if largeItemIDs != 0 {
            let result___45: avifResult = avifRWStreamWriteU32(&mut s, (*item_3).id)
                as avifResult;
            if result___45 as core::ffi::c_uint
                != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
            {
                return result___45;
            }
        } else {
            let result___46: avifResult = avifRWStreamWriteU16(
                &mut s,
                (*item_3).id as uint16_t,
            ) as avifResult;
            if result___46 as core::ffi::c_uint
                != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
            {
                return result___46;
            }
        }
        let result___47: avifResult = avifRWStreamWriteU16(&mut s, 0 as uint16_t)
            as avifResult;
        if result___47 as core::ffi::c_uint
            != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
        {
            return result___47;
        }
        let result___48: avifResult = avifRWStreamWrite(
            &mut s,
            ((*item_3).type_0).as_mut_ptr() as *const core::ffi::c_void,
            4 as size_t,
        ) as avifResult;
        if result___48 as core::ffi::c_uint
            != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
        {
            return result___48;
        }
        let result___49: avifResult = avifRWStreamWriteChars(
            &mut s,
            (*item_3).infeName,
            (*item_3).infeNameSize,
        ) as avifResult;
        if result___49 as core::ffi::c_uint
            != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
        {
            return result___49;
        }
        if memcmp(
            ((*item_3).type_0).as_mut_ptr() as *const core::ffi::c_void,
            b"mime\0" as *const u8 as *const core::ffi::c_char
                as *const core::ffi::c_void,
            4 as size_t,
        ) == 0
        {
            let result___50: avifResult = avifRWStreamWriteChars(
                &mut s,
                (*item_3).infeContentType,
                (*item_3).infeContentTypeSize,
            ) as avifResult;
            if result___50 as core::ffi::c_uint
                != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
            {
                return result___50;
            }
        } else if memcmp(
            ((*item_3).type_0).as_mut_ptr() as *const core::ffi::c_void,
            b"uri \0" as *const u8 as *const core::ffi::c_char
                as *const core::ffi::c_void,
            4 as size_t,
        ) == 0
        {
            return AVIF_RESULT_NOT_IMPLEMENTED
        }
        let result___51: avifResult = avifRWStreamFinishBox(&mut s, infe) as avifResult;
        if result___51 as core::ffi::c_uint
            != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
        {
            return result___51;
        }
        itemIndex_4 = itemIndex_4.wrapping_add(1);
    }
    let result___52: avifResult = avifRWStreamFinishBox(&mut s, iinf) as avifResult;
    if result___52 as core::ffi::c_uint
        != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
    {
        return result___52;
    }
    let mut iref: avifBoxMarker = 0 as avifBoxMarker;
    let mut itemIndex_5: uint32_t = 0 as uint32_t;
    while itemIndex_5 < (*(*encoder).data).items.count {
        let mut item_4: *mut avifEncoderItem = &mut *((*(*encoder).data).items.item)
            .offset(itemIndex_5 as isize) as *mut avifEncoderItem;
        let mut dimgCount: uint16_t = 0 as uint16_t;
        let mut dimgIndex: uint32_t = 0 as uint32_t;
        while dimgIndex < (*(*encoder).data).items.count {
            let mut dimgItem: *mut avifEncoderItem = &mut *((*(*encoder).data)
                .items
                .item)
                .offset(dimgIndex as isize) as *mut avifEncoderItem;
            if (*dimgItem).dimgFromID == (*item_4).id {
                dimgCount = dimgCount.wrapping_add(1);
            }
            dimgIndex = dimgIndex.wrapping_add(1);
        }
        if dimgCount as core::ffi::c_int > 0 as core::ffi::c_int {
            if iref == 0 {
                let result___53: avifResult = avifRWStreamWriteFullBox(
                    &mut s,
                    b"iref\0" as *const u8 as *const core::ffi::c_char,
                    0 as size_t,
                    if largeItemIDs != 0 {
                        1 as core::ffi::c_int
                    } else {
                        0 as core::ffi::c_int
                    },
                    0 as uint32_t,
                    &mut iref,
                ) as avifResult;
                if result___53 as core::ffi::c_uint
                    != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
                {
                    return result___53;
                }
            }
            let mut refType: avifBoxMarker = 0;
            let result___54: avifResult = avifRWStreamWriteBox(
                &mut s,
                b"dimg\0" as *const u8 as *const core::ffi::c_char,
                0 as size_t,
                &mut refType,
            ) as avifResult;
            if result___54 as core::ffi::c_uint
                != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
            {
                return result___54;
            }
            if largeItemIDs != 0 {
                let result___55: avifResult = avifRWStreamWriteU32(&mut s, (*item_4).id)
                    as avifResult;
                if result___55 as core::ffi::c_uint
                    != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
                {
                    return result___55;
                }
            } else {
                let result___56: avifResult = avifRWStreamWriteU16(
                    &mut s,
                    (*item_4).id as uint16_t,
                ) as avifResult;
                if result___56 as core::ffi::c_uint
                    != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
                {
                    return result___56;
                }
            }
            let result___57: avifResult = avifRWStreamWriteU16(&mut s, dimgCount)
                as avifResult;
            if result___57 as core::ffi::c_uint
                != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
            {
                return result___57;
            }
            let mut dimgIndex_0: uint32_t = 0 as uint32_t;
            while dimgIndex_0 < (*(*encoder).data).items.count {
                let mut dimgItem_0: *mut avifEncoderItem = &mut *((*(*encoder).data)
                    .items
                    .item)
                    .offset(dimgIndex_0 as isize) as *mut avifEncoderItem;
                if (*dimgItem_0).dimgFromID == (*item_4).id {
                    if largeItemIDs != 0 {
                        let result___58: avifResult = avifRWStreamWriteU32(
                            &mut s,
                            (*dimgItem_0).id,
                        ) as avifResult;
                        if result___58 as core::ffi::c_uint
                            != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
                        {
                            return result___58;
                        }
                    } else {
                        let result___59: avifResult = avifRWStreamWriteU16(
                            &mut s,
                            (*dimgItem_0).id as uint16_t,
                        ) as avifResult;
                        if result___59 as core::ffi::c_uint
                            != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
                        {
                            return result___59;
                        }
                    }
                }
                dimgIndex_0 = dimgIndex_0.wrapping_add(1);
            }
            let result___60: avifResult = avifRWStreamFinishBox(&mut s, refType)
                as avifResult;
            if result___60 as core::ffi::c_uint
                != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
            {
                return result___60;
            }
        }
        if (*item_4).irefToID != 0 as uint32_t {
            if iref == 0 {
                let result___61: avifResult = avifRWStreamWriteFullBox(
                    &mut s,
                    b"iref\0" as *const u8 as *const core::ffi::c_char,
                    0 as size_t,
                    if largeItemIDs != 0 {
                        1 as core::ffi::c_int
                    } else {
                        0 as core::ffi::c_int
                    },
                    0 as uint32_t,
                    &mut iref,
                ) as avifResult;
                if result___61 as core::ffi::c_uint
                    != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
                {
                    return result___61;
                }
            }
            let mut refType_0: avifBoxMarker = 0;
            let result___62: avifResult = avifRWStreamWriteBox(
                &mut s,
                (*item_4).irefType,
                0 as size_t,
                &mut refType_0,
            ) as avifResult;
            if result___62 as core::ffi::c_uint
                != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
            {
                return result___62;
            }
            if largeItemIDs != 0 {
                let result___63: avifResult = avifRWStreamWriteU32(&mut s, (*item_4).id)
                    as avifResult;
                if result___63 as core::ffi::c_uint
                    != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
                {
                    return result___63;
                }
                let result___64: avifResult = avifRWStreamWriteU16(&mut s, 1 as uint16_t)
                    as avifResult;
                if result___64 as core::ffi::c_uint
                    != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
                {
                    return result___64;
                }
                let result___65: avifResult = avifRWStreamWriteU32(
                    &mut s,
                    (*item_4).irefToID,
                ) as avifResult;
                if result___65 as core::ffi::c_uint
                    != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
                {
                    return result___65;
                }
            } else {
                let result___66: avifResult = avifRWStreamWriteU16(
                    &mut s,
                    (*item_4).id as uint16_t,
                ) as avifResult;
                if result___66 as core::ffi::c_uint
                    != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
                {
                    return result___66;
                }
                let result___67: avifResult = avifRWStreamWriteU16(&mut s, 1 as uint16_t)
                    as avifResult;
                if result___67 as core::ffi::c_uint
                    != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
                {
                    return result___67;
                }
                let result___68: avifResult = avifRWStreamWriteU16(
                    &mut s,
                    (*item_4).irefToID as uint16_t,
                ) as avifResult;
                if result___68 as core::ffi::c_uint
                    != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
                {
                    return result___68;
                }
            }
            let result___69: avifResult = avifRWStreamFinishBox(&mut s, refType_0)
                as avifResult;
            if result___69 as core::ffi::c_uint
                != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
            {
                return result___69;
            }
        }
        itemIndex_5 = itemIndex_5.wrapping_add(1);
    }
    if iref != 0 {
        let result___70: avifResult = avifRWStreamFinishBox(&mut s, iref) as avifResult;
        if result___70 as core::ffi::c_uint
            != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
        {
            return result___70;
        }
    }
    let mut iprp: avifBoxMarker = 0;
    let result___71: avifResult = avifRWStreamWriteBox(
        &mut s,
        b"iprp\0" as *const u8 as *const core::ffi::c_char,
        0 as size_t,
        &mut iprp,
    ) as avifResult;
    if result___71 as core::ffi::c_uint
        != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
    {
        return result___71;
    }
    let mut dedup: *mut avifItemPropertyDedup = avifItemPropertyDedupCreate();
    if dedup.is_null() {
        return AVIF_RESULT_OUT_OF_MEMORY;
    }
    let mut ipco: avifBoxMarker = 0;
    let result___72: avifResult = avifRWStreamWriteBox(
        &mut s,
        b"ipco\0" as *const u8 as *const core::ffi::c_char,
        0 as size_t,
        &mut ipco,
    ) as avifResult;
    if result___72 as core::ffi::c_uint
        != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
    {
        return result___72;
    }
    let mut altImageMetadata: *mut avifImage = 0 as *mut avifImage;
    altImageMetadata = (*(*encoder).data).altImageMetadata;
    let mut result: avifResult = avifRWStreamWriteProperties(
        dedup,
        &mut s,
        encoder,
        imageMetadata,
        altImageMetadata,
    );
    avifItemPropertyDedupDestroy(dedup);
    dedup = 0 as *mut avifItemPropertyDedup;
    let result___73: avifResult = result;
    if result___73 as core::ffi::c_uint
        != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
    {
        return result___73;
    }
    let result___74: avifResult = avifRWStreamFinishBox(&mut s, ipco) as avifResult;
    if result___74 as core::ffi::c_uint
        != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
    {
        return result___74;
    }
    let mut ipma: avifBoxMarker = 0;
    let result___75: avifResult = avifRWStreamWriteFullBox(
        &mut s,
        b"ipma\0" as *const u8 as *const core::ffi::c_char,
        0 as size_t,
        if largeItemIDs != 0 { 1 as core::ffi::c_int } else { 0 as core::ffi::c_int },
        0 as uint32_t,
        &mut ipma,
    ) as avifResult;
    if result___75 as core::ffi::c_uint
        != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
    {
        return result___75;
    }
    let mut ipmaCount: uint32_t = 0 as uint32_t;
    let mut itemIndex_6: uint32_t = 0 as uint32_t;
    while itemIndex_6 < (*(*encoder).data).items.count {
        let mut item_5: *mut avifEncoderItem = &mut *((*(*encoder).data).items.item)
            .offset(itemIndex_6 as isize) as *mut avifEncoderItem;
        if (*item_5).associations.count > 0 as uint32_t {
            if !(ipmaCount < 4294967295 as uint32_t) {
                return AVIF_RESULT_INVALID_ARGUMENT;
            }
            ipmaCount = ipmaCount.wrapping_add(1);
        }
        itemIndex_6 = itemIndex_6.wrapping_add(1);
    }
    let result___76: avifResult = avifRWStreamWriteU32(&mut s, ipmaCount) as avifResult;
    if result___76 as core::ffi::c_uint
        != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
    {
        return result___76;
    }
    let mut itemIndex_7: uint32_t = 0 as uint32_t;
    while itemIndex_7 < (*(*encoder).data).items.count {
        let mut item_6: *mut avifEncoderItem = &mut *((*(*encoder).data).items.item)
            .offset(itemIndex_7 as isize) as *mut avifEncoderItem;
        if !((*item_6).associations.count == 0 as uint32_t) {
            if largeItemIDs != 0 {
                let result___77: avifResult = avifRWStreamWriteU32(&mut s, (*item_6).id)
                    as avifResult;
                if result___77 as core::ffi::c_uint
                    != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
                {
                    return result___77;
                }
            } else {
                let result___78: avifResult = avifRWStreamWriteU16(
                    &mut s,
                    (*item_6).id as uint16_t,
                ) as avifResult;
                if result___78 as core::ffi::c_uint
                    != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
                {
                    return result___78;
                }
            }
            if !((*item_6).associations.count
                < ((1 as core::ffi::c_int) << 8 as core::ffi::c_int) as uint32_t)
            {
                return AVIF_RESULT_INTERNAL_ERROR;
            }
            let result___79: avifResult = avifRWStreamWriteU8(
                &mut s,
                (*item_6).associations.count as uint8_t,
            ) as avifResult;
            if result___79 as core::ffi::c_uint
                != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
            {
                return result___79;
            }
            let mut i_0: uint32_t = 0 as uint32_t;
            while i_0 < (*item_6).associations.count {
                let mut association: *const avifItemPropertyAssociation = &mut *((*item_6)
                    .associations
                    .association)
                    .offset(i_0 as isize) as *mut avifItemPropertyAssociation;
                let result___80: avifResult = avifRWStreamWriteBits(
                    &mut s,
                    (if (*association).essential != 0 {
                        1 as core::ffi::c_int
                    } else {
                        0 as core::ffi::c_int
                    }) as uint32_t,
                    1 as size_t,
                ) as avifResult;
                if result___80 as core::ffi::c_uint
                    != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
                {
                    return result___80;
                }
                if !((*association).property_index as core::ffi::c_int
                    <= ((1 as core::ffi::c_int) << 7 as core::ffi::c_int)
                        - 1 as core::ffi::c_int)
                {
                    return AVIF_RESULT_INTERNAL_ERROR;
                }
                let result___81: avifResult = avifRWStreamWriteBits(
                    &mut s,
                    (*association).property_index as uint32_t,
                    7 as size_t,
                ) as avifResult;
                if result___81 as core::ffi::c_uint
                    != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
                {
                    return result___81;
                }
                i_0 = i_0.wrapping_add(1);
            }
        }
        itemIndex_7 = itemIndex_7.wrapping_add(1);
    }
    let result___82: avifResult = avifRWStreamFinishBox(&mut s, ipma) as avifResult;
    if result___82 as core::ffi::c_uint
        != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
    {
        return result___82;
    }
    let result___83: avifResult = avifRWStreamFinishBox(&mut s, iprp) as avifResult;
    if result___83 as core::ffi::c_uint
        != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
    {
        return result___83;
    }
    if (*(*encoder).data).alternativeItemIDs.count != 0 {
        if !((*(*encoder).data).lastItemID < 4294967295 as uint32_t) {
            return AVIF_RESULT_INTERNAL_ERROR;
        }
        (*(*encoder).data).lastItemID = ((*(*encoder).data).lastItemID).wrapping_add(1);
        let groupID: uint32_t = (*(*encoder).data).lastItemID;
        let result___84: avifResult = avifWriteAltrGroup(
            &mut s,
            groupID,
            &mut (*(*encoder).data).alternativeItemIDs,
        ) as avifResult;
        if result___84 as core::ffi::c_uint
            != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
        {
            return result___84;
        }
    }
    let result___85: avifResult = avifRWStreamFinishBox(&mut s, meta) as avifResult;
    if result___85 as core::ffi::c_uint
        != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
    {
        return result___85;
    }
    if isSequence != 0 {
        static mut unityMatrix: [[uint8_t; 4]; 9] = [
            [
                0 as core::ffi::c_int as uint8_t,
                0x1 as core::ffi::c_int as uint8_t,
                0 as core::ffi::c_int as uint8_t,
                0 as core::ffi::c_int as uint8_t,
            ],
            [0 as core::ffi::c_int as uint8_t, 0, 0, 0],
            [0 as core::ffi::c_int as uint8_t, 0, 0, 0],
            [0 as core::ffi::c_int as uint8_t, 0, 0, 0],
            [
                0 as core::ffi::c_int as uint8_t,
                0x1 as core::ffi::c_int as uint8_t,
                0 as core::ffi::c_int as uint8_t,
                0 as core::ffi::c_int as uint8_t,
            ],
            [0 as core::ffi::c_int as uint8_t, 0, 0, 0],
            [0 as core::ffi::c_int as uint8_t, 0, 0, 0],
            [0 as core::ffi::c_int as uint8_t, 0, 0, 0],
            [
                0x40 as core::ffi::c_int as uint8_t,
                0 as core::ffi::c_int as uint8_t,
                0 as core::ffi::c_int as uint8_t,
                0 as core::ffi::c_int as uint8_t,
            ],
        ];
        if (*encoder).repetitionCount < 0 as core::ffi::c_int
            && (*encoder).repetitionCount != AVIF_REPETITION_COUNT_INFINITE
        {
            return AVIF_RESULT_INVALID_ARGUMENT;
        }
        let mut framesDurationInTimescales: uint64_t = 0 as uint64_t;
        let mut frameIndex: uint32_t = 0 as uint32_t;
        while frameIndex < (*(*encoder).data).frames.count {
            let mut frame: *const avifEncoderFrame = &mut *((*(*encoder).data)
                .frames
                .frame)
                .offset(frameIndex as isize) as *mut avifEncoderFrame;
            framesDurationInTimescales = framesDurationInTimescales
                .wrapping_add((*frame).durationInTimescales);
            frameIndex = frameIndex.wrapping_add(1);
        }
        let mut durationInTimescales: uint64_t = 0;
        if (*encoder).repetitionCount == AVIF_REPETITION_COUNT_INFINITE {
            durationInTimescales = AVIF_INDEFINITE_DURATION64 as uint64_t;
        } else {
            let mut loopCount: uint64_t = ((*encoder).repetitionCount
                + 1 as core::ffi::c_int) as uint64_t;
            if !(framesDurationInTimescales != 0 as uint64_t) {
                return AVIF_RESULT_INTERNAL_ERROR;
            }
            if loopCount
                > (UINT64_MAX as uint64_t).wrapping_div(framesDurationInTimescales)
            {
                return AVIF_RESULT_INVALID_ARGUMENT;
            }
            durationInTimescales = framesDurationInTimescales.wrapping_mul(loopCount);
        }
        let mut moov: avifBoxMarker = 0;
        let result___86: avifResult = avifRWStreamWriteBox(
            &mut s,
            b"moov\0" as *const u8 as *const core::ffi::c_char,
            0 as size_t,
            &mut moov,
        ) as avifResult;
        if result___86 as core::ffi::c_uint
            != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
        {
            return result___86;
        }
        let mut mvhd: avifBoxMarker = 0;
        let result___87: avifResult = avifRWStreamWriteFullBox(
            &mut s,
            b"mvhd\0" as *const u8 as *const core::ffi::c_char,
            0 as size_t,
            1 as core::ffi::c_int,
            0 as uint32_t,
            &mut mvhd,
        ) as avifResult;
        if result___87 as core::ffi::c_uint
            != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
        {
            return result___87;
        }
        let result___88: avifResult = avifRWStreamWriteU64(&mut s, creationTime)
            as avifResult;
        if result___88 as core::ffi::c_uint
            != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
        {
            return result___88;
        }
        let result___89: avifResult = avifRWStreamWriteU64(&mut s, modificationTime)
            as avifResult;
        if result___89 as core::ffi::c_uint
            != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
        {
            return result___89;
        }
        let result___90: avifResult = avifRWStreamWriteU32(
            &mut s,
            (*encoder).timescale as uint32_t,
        ) as avifResult;
        if result___90 as core::ffi::c_uint
            != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
        {
            return result___90;
        }
        let result___91: avifResult = avifRWStreamWriteU64(&mut s, durationInTimescales)
            as avifResult;
        if result___91 as core::ffi::c_uint
            != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
        {
            return result___91;
        }
        let result___92: avifResult = avifRWStreamWriteU32(&mut s, 0x10000 as uint32_t)
            as avifResult;
        if result___92 as core::ffi::c_uint
            != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
        {
            return result___92;
        }
        let result___93: avifResult = avifRWStreamWriteU16(&mut s, 0x100 as uint16_t)
            as avifResult;
        if result___93 as core::ffi::c_uint
            != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
        {
            return result___93;
        }
        let result___94: avifResult = avifRWStreamWriteU16(&mut s, 0 as uint16_t)
            as avifResult;
        if result___94 as core::ffi::c_uint
            != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
        {
            return result___94;
        }
        let result___95: avifResult = avifRWStreamWriteZeros(&mut s, 8 as size_t)
            as avifResult;
        if result___95 as core::ffi::c_uint
            != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
        {
            return result___95;
        }
        let result___96: avifResult = avifRWStreamWrite(
            &mut s,
            core::ptr::addr_of!(unityMatrix).cast::<[uint8_t; 4]>() as *const core::ffi::c_void,
            ::core::mem::size_of::<[[uint8_t; 4]; 9]>() as size_t,
        ) as avifResult;
        if result___96 as core::ffi::c_uint
            != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
        {
            return result___96;
        }
        let result___97: avifResult = avifRWStreamWriteZeros(&mut s, 24 as size_t)
            as avifResult;
        if result___97 as core::ffi::c_uint
            != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
        {
            return result___97;
        }
        let result___98: avifResult = avifRWStreamWriteU32(
            &mut s,
            (*(*encoder).data).items.count,
        ) as avifResult;
        if result___98 as core::ffi::c_uint
            != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
        {
            return result___98;
        }
        let result___99: avifResult = avifRWStreamFinishBox(&mut s, mvhd) as avifResult;
        if result___99 as core::ffi::c_uint
            != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
        {
            return result___99;
        }
        let mut itemIndex_8: uint32_t = 0 as uint32_t;
        while itemIndex_8 < (*(*encoder).data).items.count {
            let mut item_7: *mut avifEncoderItem = &mut *((*(*encoder).data).items.item)
                .offset(itemIndex_8 as isize) as *mut avifEncoderItem;
            if !((*(*item_7).encodeOutput).samples.count == 0 as uint32_t) {
                let mut syncSamplesCount: uint32_t = 0 as uint32_t;
                let mut sampleIndex_0: uint32_t = 0 as uint32_t;
                while sampleIndex_0 < (*(*item_7).encodeOutput).samples.count {
                    let mut sample: *mut avifEncodeSample = &mut *((*(*item_7)
                        .encodeOutput)
                        .samples
                        .sample)
                        .offset(sampleIndex_0 as isize) as *mut avifEncodeSample;
                    if (*sample).sync != 0 {
                        syncSamplesCount = syncSamplesCount.wrapping_add(1);
                    }
                    sampleIndex_0 = sampleIndex_0.wrapping_add(1);
                }
                let mut trak: avifBoxMarker = 0;
                let result___100: avifResult = avifRWStreamWriteBox(
                    &mut s,
                    b"trak\0" as *const u8 as *const core::ffi::c_char,
                    0 as size_t,
                    &mut trak,
                ) as avifResult;
                if result___100 as core::ffi::c_uint
                    != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
                {
                    return result___100;
                }
                let mut tkhd: avifBoxMarker = 0;
                let result___101: avifResult = avifRWStreamWriteFullBox(
                    &mut s,
                    b"tkhd\0" as *const u8 as *const core::ffi::c_char,
                    0 as size_t,
                    1 as core::ffi::c_int,
                    1 as uint32_t,
                    &mut tkhd,
                ) as avifResult;
                if result___101 as core::ffi::c_uint
                    != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
                {
                    return result___101;
                }
                let result___102: avifResult = avifRWStreamWriteU64(&mut s, creationTime)
                    as avifResult;
                if result___102 as core::ffi::c_uint
                    != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
                {
                    return result___102;
                }
                let result___103: avifResult = avifRWStreamWriteU64(
                    &mut s,
                    modificationTime,
                ) as avifResult;
                if result___103 as core::ffi::c_uint
                    != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
                {
                    return result___103;
                }
                let result___104: avifResult = avifRWStreamWriteU32(
                    &mut s,
                    itemIndex_8.wrapping_add(1 as uint32_t),
                ) as avifResult;
                if result___104 as core::ffi::c_uint
                    != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
                {
                    return result___104;
                }
                let result___105: avifResult = avifRWStreamWriteU32(
                    &mut s,
                    0 as uint32_t,
                ) as avifResult;
                if result___105 as core::ffi::c_uint
                    != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
                {
                    return result___105;
                }
                let result___106: avifResult = avifRWStreamWriteU64(
                    &mut s,
                    durationInTimescales,
                ) as avifResult;
                if result___106 as core::ffi::c_uint
                    != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
                {
                    return result___106;
                }
                let result___107: avifResult = avifRWStreamWriteZeros(
                    &mut s,
                    (::core::mem::size_of::<uint32_t>() as size_t)
                        .wrapping_mul(2 as size_t),
                ) as avifResult;
                if result___107 as core::ffi::c_uint
                    != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
                {
                    return result___107;
                }
                let result___108: avifResult = avifRWStreamWriteU16(
                    &mut s,
                    0 as uint16_t,
                ) as avifResult;
                if result___108 as core::ffi::c_uint
                    != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
                {
                    return result___108;
                }
                let result___109: avifResult = avifRWStreamWriteU16(
                    &mut s,
                    0 as uint16_t,
                ) as avifResult;
                if result___109 as core::ffi::c_uint
                    != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
                {
                    return result___109;
                }
                let result___110: avifResult = avifRWStreamWriteU16(
                    &mut s,
                    0 as uint16_t,
                ) as avifResult;
                if result___110 as core::ffi::c_uint
                    != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
                {
                    return result___110;
                }
                let result___111: avifResult = avifRWStreamWriteU16(
                    &mut s,
                    0 as uint16_t,
                ) as avifResult;
                if result___111 as core::ffi::c_uint
                    != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
                {
                    return result___111;
                }
                let result___112: avifResult = avifRWStreamWrite(
                    &mut s,
                    core::ptr::addr_of!(unityMatrix).cast::<[uint8_t; 4]>() as *const core::ffi::c_void,
                    ::core::mem::size_of::<[[uint8_t; 4]; 9]>() as size_t,
                ) as avifResult;
                if result___112 as core::ffi::c_uint
                    != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
                {
                    return result___112;
                }
                let result___113: avifResult = avifRWStreamWriteU32(
                    &mut s,
                    (*imageMetadata).width << 16 as core::ffi::c_int,
                ) as avifResult;
                if result___113 as core::ffi::c_uint
                    != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
                {
                    return result___113;
                }
                let result___114: avifResult = avifRWStreamWriteU32(
                    &mut s,
                    (*imageMetadata).height << 16 as core::ffi::c_int,
                ) as avifResult;
                if result___114 as core::ffi::c_uint
                    != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
                {
                    return result___114;
                }
                let result___115: avifResult = avifRWStreamFinishBox(&mut s, tkhd)
                    as avifResult;
                if result___115 as core::ffi::c_uint
                    != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
                {
                    return result___115;
                }
                if (*item_7).irefToID != 0 as uint32_t {
                    let mut tref: avifBoxMarker = 0;
                    let result___116: avifResult = avifRWStreamWriteBox(
                        &mut s,
                        b"tref\0" as *const u8 as *const core::ffi::c_char,
                        0 as size_t,
                        &mut tref,
                    ) as avifResult;
                    if result___116 as core::ffi::c_uint
                        != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
                    {
                        return result___116;
                    }
                    let mut refType_1: avifBoxMarker = 0;
                    let result___117: avifResult = avifRWStreamWriteBox(
                        &mut s,
                        (*item_7).irefType,
                        0 as size_t,
                        &mut refType_1,
                    ) as avifResult;
                    if result___117 as core::ffi::c_uint
                        != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
                    {
                        return result___117;
                    }
                    let result___118: avifResult = avifRWStreamWriteU32(
                        &mut s,
                        (*item_7).irefToID,
                    ) as avifResult;
                    if result___118 as core::ffi::c_uint
                        != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
                    {
                        return result___118;
                    }
                    let result___119: avifResult = avifRWStreamFinishBox(
                        &mut s,
                        refType_1,
                    ) as avifResult;
                    if result___119 as core::ffi::c_uint
                        != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
                    {
                        return result___119;
                    }
                    let result___120: avifResult = avifRWStreamFinishBox(&mut s, tref)
                        as avifResult;
                    if result___120 as core::ffi::c_uint
                        != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
                    {
                        return result___120;
                    }
                }
                let mut edts: avifBoxMarker = 0;
                let result___121: avifResult = avifRWStreamWriteBox(
                    &mut s,
                    b"edts\0" as *const u8 as *const core::ffi::c_char,
                    0 as size_t,
                    &mut edts,
                ) as avifResult;
                if result___121 as core::ffi::c_uint
                    != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
                {
                    return result___121;
                }
                let mut elstFlags: uint32_t = ((*encoder).repetitionCount
                    != 0 as core::ffi::c_int) as core::ffi::c_int as uint32_t;
                let mut elst: avifBoxMarker = 0;
                let result___122: avifResult = avifRWStreamWriteFullBox(
                    &mut s,
                    b"elst\0" as *const u8 as *const core::ffi::c_char,
                    0 as size_t,
                    1 as core::ffi::c_int,
                    elstFlags,
                    &mut elst,
                ) as avifResult;
                if result___122 as core::ffi::c_uint
                    != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
                {
                    return result___122;
                }
                let result___123: avifResult = avifRWStreamWriteU32(
                    &mut s,
                    1 as uint32_t,
                ) as avifResult;
                if result___123 as core::ffi::c_uint
                    != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
                {
                    return result___123;
                }
                let result___124: avifResult = avifRWStreamWriteU64(
                    &mut s,
                    framesDurationInTimescales,
                ) as avifResult;
                if result___124 as core::ffi::c_uint
                    != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
                {
                    return result___124;
                }
                let result___125: avifResult = avifRWStreamWriteU64(
                    &mut s,
                    0 as uint64_t,
                ) as avifResult;
                if result___125 as core::ffi::c_uint
                    != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
                {
                    return result___125;
                }
                let result___126: avifResult = avifRWStreamWriteU16(
                    &mut s,
                    1 as uint16_t,
                ) as avifResult;
                if result___126 as core::ffi::c_uint
                    != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
                {
                    return result___126;
                }
                let result___127: avifResult = avifRWStreamWriteU16(
                    &mut s,
                    0 as uint16_t,
                ) as avifResult;
                if result___127 as core::ffi::c_uint
                    != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
                {
                    return result___127;
                }
                let result___128: avifResult = avifRWStreamFinishBox(&mut s, elst)
                    as avifResult;
                if result___128 as core::ffi::c_uint
                    != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
                {
                    return result___128;
                }
                let result___129: avifResult = avifRWStreamFinishBox(&mut s, edts)
                    as avifResult;
                if result___129 as core::ffi::c_uint
                    != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
                {
                    return result___129;
                }
                if (*item_7).itemCategory as core::ffi::c_uint
                    != AVIF_ITEM_ALPHA as core::ffi::c_int as core::ffi::c_uint
                {
                    let result___130: avifResult = avifEncoderWriteTrackMetaBox(
                        encoder,
                        &mut s,
                    ) as avifResult;
                    if result___130 as core::ffi::c_uint
                        != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
                    {
                        return result___130;
                    }
                }
                let mut mdia: avifBoxMarker = 0;
                let result___131: avifResult = avifRWStreamWriteBox(
                    &mut s,
                    b"mdia\0" as *const u8 as *const core::ffi::c_char,
                    0 as size_t,
                    &mut mdia,
                ) as avifResult;
                if result___131 as core::ffi::c_uint
                    != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
                {
                    return result___131;
                }
                let mut mdhd: avifBoxMarker = 0;
                let result___132: avifResult = avifRWStreamWriteFullBox(
                    &mut s,
                    b"mdhd\0" as *const u8 as *const core::ffi::c_char,
                    0 as size_t,
                    1 as core::ffi::c_int,
                    0 as uint32_t,
                    &mut mdhd,
                ) as avifResult;
                if result___132 as core::ffi::c_uint
                    != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
                {
                    return result___132;
                }
                let result___133: avifResult = avifRWStreamWriteU64(&mut s, creationTime)
                    as avifResult;
                if result___133 as core::ffi::c_uint
                    != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
                {
                    return result___133;
                }
                let result___134: avifResult = avifRWStreamWriteU64(
                    &mut s,
                    modificationTime,
                ) as avifResult;
                if result___134 as core::ffi::c_uint
                    != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
                {
                    return result___134;
                }
                let result___135: avifResult = avifRWStreamWriteU32(
                    &mut s,
                    (*encoder).timescale as uint32_t,
                ) as avifResult;
                if result___135 as core::ffi::c_uint
                    != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
                {
                    return result___135;
                }
                let result___136: avifResult = avifRWStreamWriteU64(
                    &mut s,
                    framesDurationInTimescales,
                ) as avifResult;
                if result___136 as core::ffi::c_uint
                    != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
                {
                    return result___136;
                }
                let result___137: avifResult = avifRWStreamWriteU16(
                    &mut s,
                    21956 as uint16_t,
                ) as avifResult;
                if result___137 as core::ffi::c_uint
                    != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
                {
                    return result___137;
                }
                let result___138: avifResult = avifRWStreamWriteU16(
                    &mut s,
                    0 as uint16_t,
                ) as avifResult;
                if result___138 as core::ffi::c_uint
                    != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
                {
                    return result___138;
                }
                let result___139: avifResult = avifRWStreamFinishBox(&mut s, mdhd)
                    as avifResult;
                if result___139 as core::ffi::c_uint
                    != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
                {
                    return result___139;
                }
                let result___140: avifResult = avifRWStreamWriteHandlerBox(
                    &mut s,
                    if (*item_7).itemCategory as core::ffi::c_uint
                        == AVIF_ITEM_ALPHA as core::ffi::c_int as core::ffi::c_uint
                    {
                        b"auxv\0" as *const u8 as *const core::ffi::c_char
                    } else {
                        b"pict\0" as *const u8 as *const core::ffi::c_char
                    },
                ) as avifResult;
                if result___140 as core::ffi::c_uint
                    != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
                {
                    return result___140;
                }
                let mut minf: avifBoxMarker = 0;
                let result___141: avifResult = avifRWStreamWriteBox(
                    &mut s,
                    b"minf\0" as *const u8 as *const core::ffi::c_char,
                    0 as size_t,
                    &mut minf,
                ) as avifResult;
                if result___141 as core::ffi::c_uint
                    != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
                {
                    return result___141;
                }
                let mut vmhd: avifBoxMarker = 0;
                let result___142: avifResult = avifRWStreamWriteFullBox(
                    &mut s,
                    b"vmhd\0" as *const u8 as *const core::ffi::c_char,
                    0 as size_t,
                    0 as core::ffi::c_int,
                    1 as uint32_t,
                    &mut vmhd,
                ) as avifResult;
                if result___142 as core::ffi::c_uint
                    != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
                {
                    return result___142;
                }
                let result___143: avifResult = avifRWStreamWriteU16(
                    &mut s,
                    0 as uint16_t,
                ) as avifResult;
                if result___143 as core::ffi::c_uint
                    != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
                {
                    return result___143;
                }
                let result___144: avifResult = avifRWStreamWriteZeros(
                    &mut s,
                    6 as size_t,
                ) as avifResult;
                if result___144 as core::ffi::c_uint
                    != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
                {
                    return result___144;
                }
                let result___145: avifResult = avifRWStreamFinishBox(&mut s, vmhd)
                    as avifResult;
                if result___145 as core::ffi::c_uint
                    != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
                {
                    return result___145;
                }
                let mut dinf: avifBoxMarker = 0;
                let result___146: avifResult = avifRWStreamWriteBox(
                    &mut s,
                    b"dinf\0" as *const u8 as *const core::ffi::c_char,
                    0 as size_t,
                    &mut dinf,
                ) as avifResult;
                if result___146 as core::ffi::c_uint
                    != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
                {
                    return result___146;
                }
                let mut dref: avifBoxMarker = 0;
                let result___147: avifResult = avifRWStreamWriteFullBox(
                    &mut s,
                    b"dref\0" as *const u8 as *const core::ffi::c_char,
                    0 as size_t,
                    0 as core::ffi::c_int,
                    0 as uint32_t,
                    &mut dref,
                ) as avifResult;
                if result___147 as core::ffi::c_uint
                    != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
                {
                    return result___147;
                }
                let result___148: avifResult = avifRWStreamWriteU32(
                    &mut s,
                    1 as uint32_t,
                ) as avifResult;
                if result___148 as core::ffi::c_uint
                    != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
                {
                    return result___148;
                }
                let result___149: avifResult = avifRWStreamWriteFullBox(
                    &mut s,
                    b"url \0" as *const u8 as *const core::ffi::c_char,
                    0 as size_t,
                    0 as core::ffi::c_int,
                    1 as uint32_t,
                    0 as *mut avifBoxMarker,
                ) as avifResult;
                if result___149 as core::ffi::c_uint
                    != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
                {
                    return result___149;
                }
                let result___150: avifResult = avifRWStreamFinishBox(&mut s, dref)
                    as avifResult;
                if result___150 as core::ffi::c_uint
                    != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
                {
                    return result___150;
                }
                let result___151: avifResult = avifRWStreamFinishBox(&mut s, dinf)
                    as avifResult;
                if result___151 as core::ffi::c_uint
                    != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
                {
                    return result___151;
                }
                let mut stbl: avifBoxMarker = 0;
                let result___152: avifResult = avifRWStreamWriteBox(
                    &mut s,
                    b"stbl\0" as *const u8 as *const core::ffi::c_char,
                    0 as size_t,
                    &mut stbl,
                ) as avifResult;
                if result___152 as core::ffi::c_uint
                    != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
                {
                    return result___152;
                }
                let mut stsd: avifBoxMarker = 0;
                let result___153: avifResult = avifRWStreamWriteFullBox(
                    &mut s,
                    b"stsd\0" as *const u8 as *const core::ffi::c_char,
                    0 as size_t,
                    0 as core::ffi::c_int,
                    0 as uint32_t,
                    &mut stsd,
                ) as avifResult;
                if result___153 as core::ffi::c_uint
                    != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
                {
                    return result___153;
                }
                let result___154: avifResult = avifRWStreamWriteU32(
                    &mut s,
                    1 as uint32_t,
                ) as avifResult;
                if result___154 as core::ffi::c_uint
                    != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
                {
                    return result___154;
                }
                let mut imageItem: avifBoxMarker = 0;
                let result___155: avifResult = avifRWStreamWriteBox(
                    &mut s,
                    (*(*encoder).data).imageItemType,
                    0 as size_t,
                    &mut imageItem,
                ) as avifResult;
                if result___155 as core::ffi::c_uint
                    != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
                {
                    return result___155;
                }
                let result___156: avifResult = avifRWStreamWriteZeros(
                    &mut s,
                    6 as size_t,
                ) as avifResult;
                if result___156 as core::ffi::c_uint
                    != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
                {
                    return result___156;
                }
                let result___157: avifResult = avifRWStreamWriteU16(
                    &mut s,
                    1 as uint16_t,
                ) as avifResult;
                if result___157 as core::ffi::c_uint
                    != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
                {
                    return result___157;
                }
                let result___158: avifResult = avifRWStreamWriteU16(
                    &mut s,
                    0 as uint16_t,
                ) as avifResult;
                if result___158 as core::ffi::c_uint
                    != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
                {
                    return result___158;
                }
                let result___159: avifResult = avifRWStreamWriteU16(
                    &mut s,
                    0 as uint16_t,
                ) as avifResult;
                if result___159 as core::ffi::c_uint
                    != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
                {
                    return result___159;
                }
                let result___160: avifResult = avifRWStreamWriteZeros(
                    &mut s,
                    (::core::mem::size_of::<uint32_t>() as size_t)
                        .wrapping_mul(3 as size_t),
                ) as avifResult;
                if result___160 as core::ffi::c_uint
                    != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
                {
                    return result___160;
                }
                let result___161: avifResult = avifRWStreamWriteU16(
                    &mut s,
                    (*imageMetadata).width as uint16_t,
                ) as avifResult;
                if result___161 as core::ffi::c_uint
                    != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
                {
                    return result___161;
                }
                let result___162: avifResult = avifRWStreamWriteU16(
                    &mut s,
                    (*imageMetadata).height as uint16_t,
                ) as avifResult;
                if result___162 as core::ffi::c_uint
                    != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
                {
                    return result___162;
                }
                let result___163: avifResult = avifRWStreamWriteU32(
                    &mut s,
                    0x480000 as uint32_t,
                ) as avifResult;
                if result___163 as core::ffi::c_uint
                    != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
                {
                    return result___163;
                }
                let result___164: avifResult = avifRWStreamWriteU32(
                    &mut s,
                    0x480000 as uint32_t,
                ) as avifResult;
                if result___164 as core::ffi::c_uint
                    != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
                {
                    return result___164;
                }
                let result___165: avifResult = avifRWStreamWriteU32(
                    &mut s,
                    0 as uint32_t,
                ) as avifResult;
                if result___165 as core::ffi::c_uint
                    != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
                {
                    return result___165;
                }
                let result___166: avifResult = avifRWStreamWriteU16(
                    &mut s,
                    1 as uint16_t,
                ) as avifResult;
                if result___166 as core::ffi::c_uint
                    != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
                {
                    return result___166;
                }
                let result___167: avifResult = avifRWStreamWriteChars(
                    &mut s,
                    b"\nAOM Coding\0" as *const u8 as *const core::ffi::c_char,
                    11 as size_t,
                ) as avifResult;
                if result___167 as core::ffi::c_uint
                    != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
                {
                    return result___167;
                }
                let result___168: avifResult = avifRWStreamWriteZeros(
                    &mut s,
                    (32 as core::ffi::c_int - 11 as core::ffi::c_int) as size_t,
                ) as avifResult;
                if result___168 as core::ffi::c_uint
                    != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
                {
                    return result___168;
                }
                let result___169: avifResult = avifRWStreamWriteU16(
                    &mut s,
                    0x18 as uint16_t,
                ) as avifResult;
                if result___169 as core::ffi::c_uint
                    != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
                {
                    return result___169;
                }
                let result___170: avifResult = avifRWStreamWriteU16(
                    &mut s,
                    0xffff as core::ffi::c_int as uint16_t,
                ) as avifResult;
                if result___170 as core::ffi::c_uint
                    != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
                {
                    return result___170;
                }
                let result___171: avifResult = writeConfigBox(
                    &mut s,
                    &mut (*item_7).av1C,
                    (*(*encoder).data).configPropName,
                ) as avifResult;
                if result___171 as core::ffi::c_uint
                    != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
                {
                    return result___171;
                }
                if (*item_7).itemCategory as core::ffi::c_uint
                    == AVIF_ITEM_COLOR as core::ffi::c_int as core::ffi::c_uint
                {
                    let result___172: avifResult = avifEncoderWriteColorProperties(
                        &mut s,
                        imageMetadata,
                        0 as *mut avifItemPropertyAssociationArray,
                        0 as *mut avifItemPropertyDedup,
                    ) as avifResult;
                    if result___172 as core::ffi::c_uint
                        != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
                    {
                        return result___172;
                    }
                    let result___173: avifResult = avifEncoderWriteHDRProperties(
                        0 as *mut avifRWStream,
                        &mut s,
                        imageMetadata,
                        0 as *mut avifItemPropertyAssociationArray,
                        0 as *mut avifItemPropertyDedup,
                    ) as avifResult;
                    if result___173 as core::ffi::c_uint
                        != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
                    {
                        return result___173;
                    }
                    let result___174: avifResult = avifEncoderWriteTransformativeProperties(
                        0 as *mut avifRWStream,
                        &mut s,
                        imageMetadata,
                        0 as *mut avifItemPropertyAssociationArray,
                        0 as *mut avifItemPropertyDedup,
                    ) as avifResult;
                    if result___174 as core::ffi::c_uint
                        != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
                    {
                        return result___174;
                    }
                }
                let mut ccst: avifBoxMarker = 0;
                let result___175: avifResult = avifRWStreamWriteFullBox(
                    &mut s,
                    b"ccst\0" as *const u8 as *const core::ffi::c_char,
                    0 as size_t,
                    0 as core::ffi::c_int,
                    0 as uint32_t,
                    &mut ccst,
                ) as avifResult;
                if result___175 as core::ffi::c_uint
                    != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
                {
                    return result___175;
                }
                let result___176: avifResult = avifRWStreamWriteBits(
                    &mut s,
                    0 as uint32_t,
                    1 as size_t,
                ) as avifResult;
                if result___176 as core::ffi::c_uint
                    != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
                {
                    return result___176;
                }
                let result___177: avifResult = avifRWStreamWriteBits(
                    &mut s,
                    1 as uint32_t,
                    1 as size_t,
                ) as avifResult;
                if result___177 as core::ffi::c_uint
                    != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
                {
                    return result___177;
                }
                let result___178: avifResult = avifRWStreamWriteBits(
                    &mut s,
                    15 as uint32_t,
                    4 as size_t,
                ) as avifResult;
                if result___178 as core::ffi::c_uint
                    != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
                {
                    return result___178;
                }
                let result___179: avifResult = avifRWStreamWriteBits(
                    &mut s,
                    0 as uint32_t,
                    26 as size_t,
                ) as avifResult;
                if result___179 as core::ffi::c_uint
                    != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
                {
                    return result___179;
                }
                let result___180: avifResult = avifRWStreamFinishBox(&mut s, ccst)
                    as avifResult;
                if result___180 as core::ffi::c_uint
                    != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
                {
                    return result___180;
                }
                if (*item_7).itemCategory as core::ffi::c_uint
                    == AVIF_ITEM_ALPHA as core::ffi::c_int as core::ffi::c_uint
                {
                    let mut auxi: avifBoxMarker = 0;
                    let result___181: avifResult = avifRWStreamWriteFullBox(
                        &mut s,
                        b"auxi\0" as *const u8 as *const core::ffi::c_char,
                        0 as size_t,
                        0 as core::ffi::c_int,
                        0 as uint32_t,
                        &mut auxi,
                    ) as avifResult;
                    if result___181 as core::ffi::c_uint
                        != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
                    {
                        return result___181;
                    }
                    let result___182: avifResult = avifRWStreamWriteChars(
                        &mut s,
                        core::ptr::addr_of!(alphaURN).cast::<core::ffi::c_char>(),
                        alphaURNSize,
                    ) as avifResult;
                    if result___182 as core::ffi::c_uint
                        != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
                    {
                        return result___182;
                    }
                    let result___183: avifResult = avifRWStreamFinishBox(&mut s, auxi)
                        as avifResult;
                    if result___183 as core::ffi::c_uint
                        != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
                    {
                        return result___183;
                    }
                }
                let result___184: avifResult = avifRWStreamFinishBox(&mut s, imageItem)
                    as avifResult;
                if result___184 as core::ffi::c_uint
                    != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
                {
                    return result___184;
                }
                let result___185: avifResult = avifRWStreamFinishBox(&mut s, stsd)
                    as avifResult;
                if result___185 as core::ffi::c_uint
                    != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
                {
                    return result___185;
                }
                let mut stts: avifBoxMarker = 0;
                let result___186: avifResult = avifRWStreamWriteFullBox(
                    &mut s,
                    b"stts\0" as *const u8 as *const core::ffi::c_char,
                    0 as size_t,
                    0 as core::ffi::c_int,
                    0 as uint32_t,
                    &mut stts,
                ) as avifResult;
                if result___186 as core::ffi::c_uint
                    != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
                {
                    return result___186;
                }
                let mut sttsEntryCountOffset: size_t = avifRWStreamOffset(&mut s);
                let mut sttsEntryCount: uint32_t = 0 as uint32_t;
                let result___187: avifResult = avifRWStreamWriteU32(
                    &mut s,
                    0 as uint32_t,
                ) as avifResult;
                if result___187 as core::ffi::c_uint
                    != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
                {
                    return result___187;
                }
                let mut current_block_1350: u64;
                let mut sampleCount: uint32_t = 0 as uint32_t;
                let mut frameIndex_0: uint32_t = 0 as uint32_t;
                while frameIndex_0 < (*(*encoder).data).frames.count {
                    let mut frame_0: *mut avifEncoderFrame = &mut *((*(*encoder).data)
                        .frames
                        .frame)
                        .offset(frameIndex_0 as isize) as *mut avifEncoderFrame;
                    sampleCount = sampleCount.wrapping_add(1);
                    if frameIndex_0
                        < ((*(*encoder).data).frames.count).wrapping_sub(1 as uint32_t)
                    {
                        let mut nextFrame: *mut avifEncoderFrame = &mut *((*(*encoder)
                            .data)
                            .frames
                            .frame)
                            .offset(frameIndex_0.wrapping_add(1 as uint32_t) as isize)
                            as *mut avifEncoderFrame;
                        if (*frame_0).durationInTimescales
                            == (*nextFrame).durationInTimescales
                        {
                            current_block_1350 = 2;
                        } else {
                            current_block_1350 = 3;
                        }
                    } else {
                        current_block_1350 = 3;
                    }
                    match current_block_1350 {
                        3 => {
                            let result___188: avifResult = avifRWStreamWriteU32(
                                &mut s,
                                sampleCount,
                            ) as avifResult;
                            if result___188 as core::ffi::c_uint
                                != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
                            {
                                return result___188;
                            }
                            let result___189: avifResult = avifRWStreamWriteU32(
                                &mut s,
                                (*frame_0).durationInTimescales as uint32_t,
                            ) as avifResult;
                            if result___189 as core::ffi::c_uint
                                != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
                            {
                                return result___189;
                            }
                            sampleCount = 0 as uint32_t;
                            sttsEntryCount = sttsEntryCount.wrapping_add(1);
                        }
                        _ => {}
                    }
                    frameIndex_0 = frameIndex_0.wrapping_add(1);
                }
                let mut prevOffset: size_t = avifRWStreamOffset(&mut s);
                avifRWStreamSetOffset(&mut s, sttsEntryCountOffset);
                let result___190: avifResult = avifRWStreamWriteU32(
                    &mut s,
                    sttsEntryCount,
                ) as avifResult;
                if result___190 as core::ffi::c_uint
                    != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
                {
                    return result___190;
                }
                avifRWStreamSetOffset(&mut s, prevOffset);
                let result___191: avifResult = avifRWStreamFinishBox(&mut s, stts)
                    as avifResult;
                if result___191 as core::ffi::c_uint
                    != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
                {
                    return result___191;
                }
                let mut stsc: avifBoxMarker = 0;
                let result___192: avifResult = avifRWStreamWriteFullBox(
                    &mut s,
                    b"stsc\0" as *const u8 as *const core::ffi::c_char,
                    0 as size_t,
                    0 as core::ffi::c_int,
                    0 as uint32_t,
                    &mut stsc,
                ) as avifResult;
                if result___192 as core::ffi::c_uint
                    != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
                {
                    return result___192;
                }
                let result___193: avifResult = avifRWStreamWriteU32(
                    &mut s,
                    1 as uint32_t,
                ) as avifResult;
                if result___193 as core::ffi::c_uint
                    != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
                {
                    return result___193;
                }
                let result___194: avifResult = avifRWStreamWriteU32(
                    &mut s,
                    1 as uint32_t,
                ) as avifResult;
                if result___194 as core::ffi::c_uint
                    != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
                {
                    return result___194;
                }
                let result___195: avifResult = avifRWStreamWriteU32(
                    &mut s,
                    (*(*item_7).encodeOutput).samples.count,
                ) as avifResult;
                if result___195 as core::ffi::c_uint
                    != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
                {
                    return result___195;
                }
                let result___196: avifResult = avifRWStreamWriteU32(
                    &mut s,
                    1 as uint32_t,
                ) as avifResult;
                if result___196 as core::ffi::c_uint
                    != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
                {
                    return result___196;
                }
                let result___197: avifResult = avifRWStreamFinishBox(&mut s, stsc)
                    as avifResult;
                if result___197 as core::ffi::c_uint
                    != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
                {
                    return result___197;
                }
                let mut stsz: avifBoxMarker = 0;
                let result___198: avifResult = avifRWStreamWriteFullBox(
                    &mut s,
                    b"stsz\0" as *const u8 as *const core::ffi::c_char,
                    0 as size_t,
                    0 as core::ffi::c_int,
                    0 as uint32_t,
                    &mut stsz,
                ) as avifResult;
                if result___198 as core::ffi::c_uint
                    != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
                {
                    return result___198;
                }
                let result___199: avifResult = avifRWStreamWriteU32(
                    &mut s,
                    0 as uint32_t,
                ) as avifResult;
                if result___199 as core::ffi::c_uint
                    != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
                {
                    return result___199;
                }
                let result___200: avifResult = avifRWStreamWriteU32(
                    &mut s,
                    (*(*item_7).encodeOutput).samples.count,
                ) as avifResult;
                if result___200 as core::ffi::c_uint
                    != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
                {
                    return result___200;
                }
                let mut sampleIndex_1: uint32_t = 0 as uint32_t;
                while sampleIndex_1 < (*(*item_7).encodeOutput).samples.count {
                    let mut sample_0: *mut avifEncodeSample = &mut *((*(*item_7)
                        .encodeOutput)
                        .samples
                        .sample)
                        .offset(sampleIndex_1 as isize) as *mut avifEncodeSample;
                    let result___201: avifResult = avifRWStreamWriteU32(
                        &mut s,
                        (*sample_0).data.size as uint32_t,
                    ) as avifResult;
                    if result___201 as core::ffi::c_uint
                        != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
                    {
                        return result___201;
                    }
                    sampleIndex_1 = sampleIndex_1.wrapping_add(1);
                }
                let result___202: avifResult = avifRWStreamFinishBox(&mut s, stsz)
                    as avifResult;
                if result___202 as core::ffi::c_uint
                    != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
                {
                    return result___202;
                }
                let mut stco: avifBoxMarker = 0;
                let result___203: avifResult = avifRWStreamWriteFullBox(
                    &mut s,
                    b"stco\0" as *const u8 as *const core::ffi::c_char,
                    0 as size_t,
                    0 as core::ffi::c_int,
                    0 as uint32_t,
                    &mut stco,
                ) as avifResult;
                if result___203 as core::ffi::c_uint
                    != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
                {
                    return result___203;
                }
                let result___204: avifResult = avifRWStreamWriteU32(
                    &mut s,
                    1 as uint32_t,
                ) as avifResult;
                if result___204 as core::ffi::c_uint
                    != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
                {
                    return result___204;
                }
                let result___205: avifResult = avifEncoderItemAddMdatFixup(
                    item_7,
                    &mut s,
                ) as avifResult;
                if result___205 as core::ffi::c_uint
                    != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
                {
                    return result___205;
                }
                let result___206: avifResult = avifRWStreamWriteU32(
                    &mut s,
                    1 as uint32_t,
                ) as avifResult;
                if result___206 as core::ffi::c_uint
                    != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
                {
                    return result___206;
                }
                let result___207: avifResult = avifRWStreamFinishBox(&mut s, stco)
                    as avifResult;
                if result___207 as core::ffi::c_uint
                    != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
                {
                    return result___207;
                }
                let mut hasNonSyncSample: avifBool = AVIF_FALSE;
                let mut sampleIndex_2: uint32_t = 0 as uint32_t;
                while sampleIndex_2 < (*(*item_7).encodeOutput).samples.count {
                    if (*((*(*item_7).encodeOutput).samples.sample)
                        .offset(sampleIndex_2 as isize))
                        .sync == 0
                    {
                        hasNonSyncSample = AVIF_TRUE as avifBool;
                        break;
                    } else {
                        sampleIndex_2 = sampleIndex_2.wrapping_add(1);
                    }
                }
                if hasNonSyncSample != 0 {
                    let mut stss: avifBoxMarker = 0;
                    let result___208: avifResult = avifRWStreamWriteFullBox(
                        &mut s,
                        b"stss\0" as *const u8 as *const core::ffi::c_char,
                        0 as size_t,
                        0 as core::ffi::c_int,
                        0 as uint32_t,
                        &mut stss,
                    ) as avifResult;
                    if result___208 as core::ffi::c_uint
                        != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
                    {
                        return result___208;
                    }
                    let result___209: avifResult = avifRWStreamWriteU32(
                        &mut s,
                        syncSamplesCount,
                    ) as avifResult;
                    if result___209 as core::ffi::c_uint
                        != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
                    {
                        return result___209;
                    }
                    let mut sampleIndex_3: uint32_t = 0 as uint32_t;
                    while sampleIndex_3 < (*(*item_7).encodeOutput).samples.count {
                        let mut sample_1: *mut avifEncodeSample = &mut *((*(*item_7)
                            .encodeOutput)
                            .samples
                            .sample)
                            .offset(sampleIndex_3 as isize) as *mut avifEncodeSample;
                        if (*sample_1).sync != 0 {
                            let result___210: avifResult = avifRWStreamWriteU32(
                                &mut s,
                                sampleIndex_3.wrapping_add(1 as uint32_t),
                            ) as avifResult;
                            if result___210 as core::ffi::c_uint
                                != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
                            {
                                return result___210;
                            }
                        }
                        sampleIndex_3 = sampleIndex_3.wrapping_add(1);
                    }
                    let result___211: avifResult = avifRWStreamFinishBox(&mut s, stss)
                        as avifResult;
                    if result___211 as core::ffi::c_uint
                        != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
                    {
                        return result___211;
                    }
                }
                let result___212: avifResult = avifRWStreamFinishBox(&mut s, stbl)
                    as avifResult;
                if result___212 as core::ffi::c_uint
                    != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
                {
                    return result___212;
                }
                let result___213: avifResult = avifRWStreamFinishBox(&mut s, minf)
                    as avifResult;
                if result___213 as core::ffi::c_uint
                    != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
                {
                    return result___213;
                }
                let result___214: avifResult = avifRWStreamFinishBox(&mut s, mdia)
                    as avifResult;
                if result___214 as core::ffi::c_uint
                    != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
                {
                    return result___214;
                }
                let result___215: avifResult = avifRWStreamFinishBox(&mut s, trak)
                    as avifResult;
                if result___215 as core::ffi::c_uint
                    != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
                {
                    return result___215;
                }
            }
            itemIndex_8 = itemIndex_8.wrapping_add(1);
        }
        let result___216: avifResult = avifRWStreamFinishBox(&mut s, moov) as avifResult;
        if result___216 as core::ffi::c_uint
            != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
        {
            return result___216;
        }
    }
    let mut layeredColorItems: avifEncoderItemReferenceArray = avifEncoderItemReferenceArray {
        ref_0: 0 as *mut avifEncoderItemReference,
        elementSize: 0,
        count: 0,
        capacity: 0,
    };
    let mut layeredAlphaItems: avifEncoderItemReferenceArray = avifEncoderItemReferenceArray {
        ref_0: 0 as *mut avifEncoderItemReference,
        elementSize: 0,
        count: 0,
        capacity: 0,
    };
    if avifArrayCreate(
        &mut layeredColorItems as *mut avifEncoderItemReferenceArray
            as *mut core::ffi::c_void,
        ::core::mem::size_of::<avifEncoderItemReference>() as uint32_t,
        1 as uint32_t,
    ) == 0
    {
        result = AVIF_RESULT_OUT_OF_MEMORY;
    }
    if avifArrayCreate(
        &mut layeredAlphaItems as *mut avifEncoderItemReferenceArray
            as *mut core::ffi::c_void,
        ::core::mem::size_of::<avifEncoderItemReference>() as uint32_t,
        1 as uint32_t,
    ) == 0
    {
        result = AVIF_RESULT_OUT_OF_MEMORY;
    }
    if result as core::ffi::c_uint
        == AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
    {
        result = avifEncoderWriteMediaDataBox(
            encoder,
            &mut s,
            &mut layeredColorItems,
            &mut layeredAlphaItems,
        );
    }
    avifArrayDestroy(
        &mut layeredColorItems as *mut avifEncoderItemReferenceArray
            as *mut core::ffi::c_void,
    );
    avifArrayDestroy(
        &mut layeredAlphaItems as *mut avifEncoderItemReferenceArray
            as *mut core::ffi::c_void,
    );
    let result___217: avifResult = result;
    if result___217 as core::ffi::c_uint
        != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
    {
        return result___217;
    }
    avifRWStreamFinishWrite(&mut s);
    return AVIF_RESULT_OK;
}
#[no_mangle]
pub unsafe extern "C" fn avifEncoderWrite(
    mut encoder: *mut avifEncoder,
    mut image: *const avifImage,
    mut output: *mut avifRWData,
) -> avifResult {
    let mut addImageResult: avifResult = avifEncoderAddImage(
        encoder,
        image,
        1 as uint64_t,
        AVIF_ADD_IMAGE_FLAG_SINGLE as core::ffi::c_int as avifAddImageFlags,
    );
    if addImageResult as core::ffi::c_uint
        != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
    {
        return addImageResult;
    }
    return avifEncoderFinish(encoder, output);
}
unsafe extern "C" fn writeCodecConfig(
    mut s: *mut avifRWStream,
    mut cfg: *const avifCodecConfigurationBox,
) -> avifResult {
    let av1COffset: size_t = (*s).offset;
    let result__: avifResult = avifRWStreamWriteBits(s, 1 as uint32_t, 1 as size_t)
        as avifResult;
    if result__ as core::ffi::c_uint
        != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
    {
        return result__;
    }
    let result___0: avifResult = avifRWStreamWriteBits(s, 1 as uint32_t, 7 as size_t)
        as avifResult;
    if result___0 as core::ffi::c_uint
        != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
    {
        return result___0;
    }
    let result___1: avifResult = avifRWStreamWriteBits(
        s,
        (*cfg).seqProfile as uint32_t,
        3 as size_t,
    ) as avifResult;
    if result___1 as core::ffi::c_uint
        != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
    {
        return result___1;
    }
    let result___2: avifResult = avifRWStreamWriteBits(
        s,
        (*cfg).seqLevelIdx0 as uint32_t,
        5 as size_t,
    ) as avifResult;
    if result___2 as core::ffi::c_uint
        != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
    {
        return result___2;
    }
    let result___3: avifResult = avifRWStreamWriteBits(
        s,
        (*cfg).seqTier0 as uint32_t,
        1 as size_t,
    ) as avifResult;
    if result___3 as core::ffi::c_uint
        != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
    {
        return result___3;
    }
    let result___4: avifResult = avifRWStreamWriteBits(
        s,
        (*cfg).highBitdepth as uint32_t,
        1 as size_t,
    ) as avifResult;
    if result___4 as core::ffi::c_uint
        != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
    {
        return result___4;
    }
    let result___5: avifResult = avifRWStreamWriteBits(
        s,
        (*cfg).twelveBit as uint32_t,
        1 as size_t,
    ) as avifResult;
    if result___5 as core::ffi::c_uint
        != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
    {
        return result___5;
    }
    let result___6: avifResult = avifRWStreamWriteBits(
        s,
        (*cfg).monochrome as uint32_t,
        1 as size_t,
    ) as avifResult;
    if result___6 as core::ffi::c_uint
        != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
    {
        return result___6;
    }
    let result___7: avifResult = avifRWStreamWriteBits(
        s,
        (*cfg).chromaSubsamplingX as uint32_t,
        1 as size_t,
    ) as avifResult;
    if result___7 as core::ffi::c_uint
        != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
    {
        return result___7;
    }
    let result___8: avifResult = avifRWStreamWriteBits(
        s,
        (*cfg).chromaSubsamplingY as uint32_t,
        1 as size_t,
    ) as avifResult;
    if result___8 as core::ffi::c_uint
        != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
    {
        return result___8;
    }
    let result___9: avifResult = avifRWStreamWriteBits(
        s,
        (*cfg).chromaSamplePosition as uint32_t,
        2 as size_t,
    ) as avifResult;
    if result___9 as core::ffi::c_uint
        != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
    {
        return result___9;
    }
    let result___10: avifResult = avifRWStreamWriteBits(s, 0 as uint32_t, 3 as size_t)
        as avifResult;
    if result___10 as core::ffi::c_uint
        != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
    {
        return result___10;
    }
    let result___11: avifResult = avifRWStreamWriteBits(s, 0 as uint32_t, 1 as size_t)
        as avifResult;
    if result___11 as core::ffi::c_uint
        != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
    {
        return result___11;
    }
    let result___12: avifResult = avifRWStreamWriteBits(s, 0 as uint32_t, 4 as size_t)
        as avifResult;
    if result___12 as core::ffi::c_uint
        != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
    {
        return result___12;
    }
    if !(((*s).offset).wrapping_sub(av1COffset) == 4 as size_t) {
        return AVIF_RESULT_INTERNAL_ERROR;
    }
    return AVIF_RESULT_OK;
}
unsafe extern "C" fn writeConfigBox(
    mut s: *mut avifRWStream,
    mut cfg: *const avifCodecConfigurationBox,
    mut configPropName: *const core::ffi::c_char,
) -> avifResult {
    let mut configBox: avifBoxMarker = 0;
    let result__: avifResult = avifRWStreamWriteBox(
        s,
        configPropName,
        0 as size_t,
        &mut configBox,
    ) as avifResult;
    if result__ as core::ffi::c_uint
        != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
    {
        return result__;
    }
    let result___0: avifResult = writeCodecConfig(s, cfg) as avifResult;
    if result___0 as core::ffi::c_uint
        != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
    {
        return result___0;
    }
    let result___1: avifResult = avifRWStreamFinishBox(s, configBox) as avifResult;
    if result___1 as core::ffi::c_uint
        != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
    {
        return result___1;
    }
    return AVIF_RESULT_OK;
}
