// Copyright 2019 Joe Drago. All rights reserved.
// SPDX-License-Identifier: BSD-2-Clause

// Mechanically translated from upstream/libavif/src/reformat.c.
// C ABI, ownership, pointer validity, and bounds follow the original headers.
extern "C" {
    fn avifAlloc(size: size_t) -> *mut core::ffi::c_void;
    fn avifFree(p: *mut core::ffi::c_void);
    fn avifGetPixelFormatInfo(format: avifPixelFormat, info: *mut avifPixelFormatInfo);
    fn avifImageSetViewRect(
        dstImage: *mut avifImage,
        srcImage: *const avifImage,
        rect: *const avifCropRect,
    ) -> avifResult;
    fn avifImageAllocatePlanes(
        image: *mut avifImage,
        planes: avifPlanesFlags,
    ) -> avifResult;
    fn avifRGBFormatChannelCount(format: avifRGBFormat) -> uint32_t;
    fn avifRGBFormatHasAlpha(format: avifRGBFormat) -> avifBool;
    fn avifRGBFormatIsGray(format: avifRGBFormat) -> avifBool;
    fn avifRGBImagePixelSize(rgb: *const avifRGBImage) -> uint32_t;
    fn avifRGBImagePremultiplyAlpha(rgb: *mut avifRGBImage) -> avifResult;
    fn avifRGBImageUnpremultiplyAlpha(rgb: *mut avifRGBImage) -> avifResult;
    fn avifRoundf(v: core::ffi::c_float) -> core::ffi::c_float;
    fn avifCalcYUVCoefficients(
        image: *const avifImage,
        outR: *mut core::ffi::c_float,
        outG: *mut core::ffi::c_float,
        outB: *mut core::ffi::c_float,
    );
    fn avifFillAlpha(params: *const avifAlphaParams);
    fn avifReformatAlpha(params: *const avifAlphaParams);
    fn avifImageRGBToYUVLibYUV(
        image: *mut avifImage,
        rgb: *const avifRGBImage,
    ) -> avifResult;
    fn avifImageYUVToRGBLibYUV(
        image: *const avifImage,
        rgb: *mut avifRGBImage,
        reformatAlpha: avifBool,
        alphaReformattedWithLibYUV: *mut avifBool,
    ) -> avifResult;
    fn avifImageRGBToYUVLibSharpYUV(
        image: *mut avifImage,
        rgb: *const avifRGBImage,
        state: *const avifReformatState,
    ) -> avifResult;
    fn avifRGBImageToF16LibYUV(rgb: *mut avifRGBImage) -> avifResult;
    fn __assert_fail(
        __assertion: *const core::ffi::c_char,
        __file: *const core::ffi::c_char,
        __line: core::ffi::c_uint,
        __function: *const core::ffi::c_char,
    ) -> !;
    fn memset(
        __s: *mut core::ffi::c_void,
        __c: core::ffi::c_int,
        __n: size_t,
    ) -> *mut core::ffi::c_void;
    fn pthread_create(
        __newthread: *mut pthread_t,
        __attr: *const pthread_attr_t,
        __start_routine: Option<
            unsafe extern "C" fn(*mut core::ffi::c_void) -> *mut core::ffi::c_void,
        >,
        __arg: *mut core::ffi::c_void,
    ) -> core::ffi::c_int;
    fn pthread_join(
        __th: pthread_t,
        __thread_return: *mut *mut core::ffi::c_void,
    ) -> core::ffi::c_int;
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
pub type C2RustUnnamed = core::ffi::c_uint;
pub const AVIF_MATRIX_COEFFICIENTS_LAST: C2RustUnnamed = 18;
pub const AVIF_MATRIX_COEFFICIENTS_YCGCO_RO: C2RustUnnamed = 17;
pub const AVIF_MATRIX_COEFFICIENTS_YCGCO_RE: C2RustUnnamed = 16;
pub const AVIF_MATRIX_COEFFICIENTS_ICTCP: C2RustUnnamed = 14;
pub const AVIF_MATRIX_COEFFICIENTS_CHROMA_DERIVED_CL: C2RustUnnamed = 13;
pub const AVIF_MATRIX_COEFFICIENTS_CHROMA_DERIVED_NCL: C2RustUnnamed = 12;
pub const AVIF_MATRIX_COEFFICIENTS_SMPTE2085: C2RustUnnamed = 11;
pub const AVIF_MATRIX_COEFFICIENTS_BT2020_CL: C2RustUnnamed = 10;
pub const AVIF_MATRIX_COEFFICIENTS_BT2020_NCL: C2RustUnnamed = 9;
pub const AVIF_MATRIX_COEFFICIENTS_YCGCO: C2RustUnnamed = 8;
pub const AVIF_MATRIX_COEFFICIENTS_SMPTE240: C2RustUnnamed = 7;
pub const AVIF_MATRIX_COEFFICIENTS_BT601: C2RustUnnamed = 6;
pub const AVIF_MATRIX_COEFFICIENTS_BT470BG: C2RustUnnamed = 5;
pub const AVIF_MATRIX_COEFFICIENTS_FCC: C2RustUnnamed = 4;
pub const AVIF_MATRIX_COEFFICIENTS_UNSPECIFIED: C2RustUnnamed = 2;
pub const AVIF_MATRIX_COEFFICIENTS_BT709: C2RustUnnamed = 1;
pub const AVIF_MATRIX_COEFFICIENTS_IDENTITY: C2RustUnnamed = 0;
pub type avifMatrixCoefficients = uint16_t;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct avifAlphaParams {
    pub width: uint32_t,
    pub height: uint32_t,
    pub srcDepth: uint32_t,
    pub srcPlane: *const uint8_t,
    pub srcRowBytes: uint32_t,
    pub srcOffsetBytes: uint32_t,
    pub srcPixelBytes: uint32_t,
    pub dstDepth: uint32_t,
    pub dstPlane: *mut uint8_t,
    pub dstRowBytes: uint32_t,
    pub dstOffsetBytes: uint32_t,
    pub dstPixelBytes: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct avifRGBColorSpaceInfo {
    pub channelBytes: uint32_t,
    pub pixelBytes: uint32_t,
    pub offsetBytesR: uint32_t,
    pub offsetBytesG: uint32_t,
    pub offsetBytesB: uint32_t,
    pub offsetBytesA: uint32_t,
    pub offsetBytesGray: uint32_t,
    pub maxChannel: core::ffi::c_int,
    pub maxChannelF: core::ffi::c_float,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct avifReformatState {
    pub rgb: avifRGBColorSpaceInfo,
    pub yuv: avifYUVColorSpaceInfo,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct avifYUVColorSpaceInfo {
    pub kr: core::ffi::c_float,
    pub kg: core::ffi::c_float,
    pub kb: core::ffi::c_float,
    pub channelBytes: uint32_t,
    pub depth: uint32_t,
    pub range: avifRange,
    pub maxChannel: core::ffi::c_int,
    pub biasY: core::ffi::c_float,
    pub biasUV: core::ffi::c_float,
    pub rangeY: core::ffi::c_float,
    pub rangeUV: core::ffi::c_float,
    pub formatInfo: avifPixelFormatInfo,
    pub mode: avifReformatMode,
}
pub type avifReformatMode = core::ffi::c_uint;
pub const AVIF_REFORMAT_MODE_YCGCO_RO: avifReformatMode = 4;
pub const AVIF_REFORMAT_MODE_YCGCO_RE: avifReformatMode = 3;
pub const AVIF_REFORMAT_MODE_YCGCO: avifReformatMode = 2;
pub const AVIF_REFORMAT_MODE_IDENTITY: avifReformatMode = 1;
pub const AVIF_REFORMAT_MODE_YUV_COEFFICIENTS: avifReformatMode = 0;
pub const AVIF_ALPHA_MULTIPLY_MODE_MULTIPLY: avifAlphaMultiplyMode = 1;
pub type avifAlphaMultiplyMode = core::ffi::c_uint;
pub const AVIF_ALPHA_MULTIPLY_MODE_UNMULTIPLY: avifAlphaMultiplyMode = 2;
pub const AVIF_ALPHA_MULTIPLY_MODE_NO_OP: avifAlphaMultiplyMode = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct YUVBlock {
    pub y: core::ffi::c_float,
    pub u: core::ffi::c_float,
    pub v: core::ffi::c_float,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct YUVToRGBThreadData {
    pub thread: pthread_t,
    pub image: avifImage,
    pub rgb: avifRGBImage,
    pub state: *mut avifReformatState,
    pub alphaMultiplyMode: avifAlphaMultiplyMode,
    pub result: avifResult,
    pub threadCreated: avifBool,
}
pub type pthread_t = core::ffi::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub union avifF16 {
    pub f: core::ffi::c_float,
    pub u32_0: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_attr_t {
    pub __size: [core::ffi::c_char; 56],
    pub __align: core::ffi::c_long,
}
pub const NULL: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
pub const AVIF_TRUE: core::ffi::c_int = 1 as core::ffi::c_int;
pub const AVIF_FALSE: core::ffi::c_int = 0 as core::ffi::c_int;
unsafe extern "C" fn avifMemset16(
    mut dest: *mut core::ffi::c_void,
    mut val: core::ffi::c_int,
    mut count: size_t,
) -> *mut core::ffi::c_void {
    let mut dest16: *mut uint16_t = dest as *mut uint16_t;
    let mut i: size_t = 0 as size_t;
    while i < count {
        let fresh0 = dest16;
        dest16 = dest16.offset(1);
        *fresh0 = val as uint16_t;
        i = i.wrapping_add(1);
    }
    return dest;
}
#[no_mangle]
pub unsafe extern "C" fn avifGetRGBColorSpaceInfo(
    mut rgb: *const avifRGBImage,
    mut info: *mut avifRGBColorSpaceInfo,
) -> avifBool {
    if !((*rgb).depth == 8 as uint32_t || (*rgb).depth == 10 as uint32_t
        || (*rgb).depth == 12 as uint32_t || (*rgb).depth == 16 as uint32_t)
    {
        return AVIF_FALSE;
    }
    if (*rgb).isFloat != 0 {
        if !((*rgb).depth == 16 as uint32_t) {
            return AVIF_FALSE;
        }
    }
    if (*rgb).format as core::ffi::c_uint
        == AVIF_RGB_FORMAT_RGB_565 as core::ffi::c_int as core::ffi::c_uint
    {
        if !((*rgb).depth == 8 as uint32_t) {
            return AVIF_FALSE;
        }
    }
    if !((*rgb).format as core::ffi::c_int >= AVIF_RGB_FORMAT_RGB as core::ffi::c_int
        && ((*rgb).format as core::ffi::c_uint)
            < AVIF_RGB_FORMAT_COUNT as core::ffi::c_int as core::ffi::c_uint)
    {
        return AVIF_FALSE;
    }
    (*info).channelBytes = (if (*rgb).depth > 8 as uint32_t {
        2 as core::ffi::c_int
    } else {
        1 as core::ffi::c_int
    }) as uint32_t;
    (*info).pixelBytes = avifRGBImagePixelSize(rgb);
    (*info).offsetBytesR = 0 as uint32_t;
    (*info).offsetBytesG = 0 as uint32_t;
    (*info).offsetBytesB = 0 as uint32_t;
    (*info).offsetBytesA = 0 as uint32_t;
    (*info).offsetBytesGray = 0 as uint32_t;
    match (*rgb).format as core::ffi::c_uint {
        0 => {
            (*info).offsetBytesR = ((*info).channelBytes).wrapping_mul(0 as uint32_t);
            (*info).offsetBytesG = ((*info).channelBytes).wrapping_mul(1 as uint32_t);
            (*info).offsetBytesB = ((*info).channelBytes).wrapping_mul(2 as uint32_t);
        }
        1 => {
            (*info).offsetBytesR = ((*info).channelBytes).wrapping_mul(0 as uint32_t);
            (*info).offsetBytesG = ((*info).channelBytes).wrapping_mul(1 as uint32_t);
            (*info).offsetBytesB = ((*info).channelBytes).wrapping_mul(2 as uint32_t);
            (*info).offsetBytesA = ((*info).channelBytes).wrapping_mul(3 as uint32_t);
        }
        2 => {
            (*info).offsetBytesA = ((*info).channelBytes).wrapping_mul(0 as uint32_t);
            (*info).offsetBytesR = ((*info).channelBytes).wrapping_mul(1 as uint32_t);
            (*info).offsetBytesG = ((*info).channelBytes).wrapping_mul(2 as uint32_t);
            (*info).offsetBytesB = ((*info).channelBytes).wrapping_mul(3 as uint32_t);
        }
        3 => {
            (*info).offsetBytesB = ((*info).channelBytes).wrapping_mul(0 as uint32_t);
            (*info).offsetBytesG = ((*info).channelBytes).wrapping_mul(1 as uint32_t);
            (*info).offsetBytesR = ((*info).channelBytes).wrapping_mul(2 as uint32_t);
        }
        4 => {
            (*info).offsetBytesB = ((*info).channelBytes).wrapping_mul(0 as uint32_t);
            (*info).offsetBytesG = ((*info).channelBytes).wrapping_mul(1 as uint32_t);
            (*info).offsetBytesR = ((*info).channelBytes).wrapping_mul(2 as uint32_t);
            (*info).offsetBytesA = ((*info).channelBytes).wrapping_mul(3 as uint32_t);
        }
        5 => {
            (*info).offsetBytesA = ((*info).channelBytes).wrapping_mul(0 as uint32_t);
            (*info).offsetBytesB = ((*info).channelBytes).wrapping_mul(1 as uint32_t);
            (*info).offsetBytesG = ((*info).channelBytes).wrapping_mul(2 as uint32_t);
            (*info).offsetBytesR = ((*info).channelBytes).wrapping_mul(3 as uint32_t);
        }
        6 => {
            (*info).offsetBytesR = 0 as uint32_t;
            (*info).offsetBytesG = 0 as uint32_t;
            (*info).offsetBytesB = 0 as uint32_t;
        }
        7 => {
            (*info).offsetBytesGray = ((*info).channelBytes).wrapping_mul(0 as uint32_t);
        }
        8 => {
            (*info).offsetBytesGray = ((*info).channelBytes).wrapping_mul(0 as uint32_t);
            (*info).offsetBytesA = ((*info).channelBytes).wrapping_mul(1 as uint32_t);
        }
        9 => {
            (*info).offsetBytesA = ((*info).channelBytes).wrapping_mul(0 as uint32_t);
            (*info).offsetBytesGray = ((*info).channelBytes).wrapping_mul(1 as uint32_t);
        }
        10 => return AVIF_FALSE,
        _ => {}
    }
    (*info).maxChannel = ((1 as core::ffi::c_int) << (*rgb).depth)
        - 1 as core::ffi::c_int;
    (*info).maxChannelF = (*info).maxChannel as core::ffi::c_float;
    return AVIF_TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn avifGetYUVColorSpaceInfo(
    mut image: *const avifImage,
    mut info: *mut avifYUVColorSpaceInfo,
) -> avifBool {
    if !((*image).depth == 8 as uint32_t || (*image).depth == 10 as uint32_t
        || (*image).depth == 12 as uint32_t || (*image).depth == 16 as uint32_t)
    {
        return AVIF_FALSE;
    }
    if !((*image).yuvFormat as core::ffi::c_uint
        >= AVIF_PIXEL_FORMAT_YUV444 as core::ffi::c_int as core::ffi::c_uint
        && ((*image).yuvFormat as core::ffi::c_uint)
            < AVIF_PIXEL_FORMAT_COUNT as core::ffi::c_int as core::ffi::c_uint)
    {
        return AVIF_FALSE;
    }
    if !((*image).yuvRange as core::ffi::c_uint
        == AVIF_RANGE_LIMITED as core::ffi::c_int as core::ffi::c_uint
        || (*image).yuvRange as core::ffi::c_uint
            == AVIF_RANGE_FULL as core::ffi::c_int as core::ffi::c_uint)
    {
        return AVIF_FALSE;
    }
    if (*image).matrixCoefficients as core::ffi::c_int == 3 as core::ffi::c_int
        || ((*image).matrixCoefficients as core::ffi::c_int
            == AVIF_MATRIX_COEFFICIENTS_YCGCO as core::ffi::c_int
            || (*image).matrixCoefficients as core::ffi::c_int
                == AVIF_MATRIX_COEFFICIENTS_YCGCO_RE as core::ffi::c_int
            || (*image).matrixCoefficients as core::ffi::c_int
                == AVIF_MATRIX_COEFFICIENTS_YCGCO_RO as core::ffi::c_int)
            && (*image).yuvRange as core::ffi::c_uint
                == AVIF_RANGE_LIMITED as core::ffi::c_int as core::ffi::c_uint
        || (*image).matrixCoefficients as core::ffi::c_int
            == AVIF_MATRIX_COEFFICIENTS_BT2020_CL as core::ffi::c_int
        || (*image).matrixCoefficients as core::ffi::c_int
            == AVIF_MATRIX_COEFFICIENTS_SMPTE2085 as core::ffi::c_int
        || (*image).matrixCoefficients as core::ffi::c_int
            == AVIF_MATRIX_COEFFICIENTS_CHROMA_DERIVED_CL as core::ffi::c_int
        || (*image).matrixCoefficients as core::ffi::c_int
            == AVIF_MATRIX_COEFFICIENTS_ICTCP as core::ffi::c_int
        || (*image).matrixCoefficients as core::ffi::c_int
            >= AVIF_MATRIX_COEFFICIENTS_LAST as core::ffi::c_int
    {
        return AVIF_FALSE;
    }
    if (*image).matrixCoefficients as core::ffi::c_int
        == AVIF_MATRIX_COEFFICIENTS_IDENTITY as core::ffi::c_int
        && (*image).yuvFormat as core::ffi::c_uint
            != AVIF_PIXEL_FORMAT_YUV444 as core::ffi::c_int as core::ffi::c_uint
        && (*image).yuvFormat as core::ffi::c_uint
            != AVIF_PIXEL_FORMAT_YUV400 as core::ffi::c_int as core::ffi::c_uint
    {
        return AVIF_FALSE;
    }
    avifGetPixelFormatInfo((*image).yuvFormat, &mut (*info).formatInfo);
    avifCalcYUVCoefficients(image, &mut (*info).kr, &mut (*info).kg, &mut (*info).kb);
    (*info).channelBytes = (if (*image).depth > 8 as uint32_t {
        2 as core::ffi::c_int
    } else {
        1 as core::ffi::c_int
    }) as uint32_t;
    (*info).depth = (*image).depth;
    (*info).range = (*image).yuvRange;
    (*info).maxChannel = ((1 as core::ffi::c_int) << (*image).depth)
        - 1 as core::ffi::c_int;
    (*info).biasY = if (*info).range as core::ffi::c_uint
        == AVIF_RANGE_LIMITED as core::ffi::c_int as core::ffi::c_uint
    {
        ((16 as core::ffi::c_int) << ((*info).depth).wrapping_sub(8 as uint32_t))
            as core::ffi::c_float
    } else {
        0.0f32
    };
    (*info).biasUV = ((1 as core::ffi::c_int)
        << ((*info).depth).wrapping_sub(1 as uint32_t)) as core::ffi::c_float;
    (*info).rangeY = (if (*info).range as core::ffi::c_uint
        == AVIF_RANGE_LIMITED as core::ffi::c_int as core::ffi::c_uint
    {
        (219 as core::ffi::c_int) << ((*info).depth).wrapping_sub(8 as uint32_t)
    } else {
        (*info).maxChannel
    }) as core::ffi::c_float;
    (*info).rangeUV = (if (*info).range as core::ffi::c_uint
        == AVIF_RANGE_LIMITED as core::ffi::c_int as core::ffi::c_uint
    {
        (224 as core::ffi::c_int) << ((*info).depth).wrapping_sub(8 as uint32_t)
    } else {
        (*info).maxChannel
    }) as core::ffi::c_float;
    return AVIF_TRUE;
}
unsafe extern "C" fn avifPrepareReformatState(
    mut image: *const avifImage,
    mut rgb: *const avifRGBImage,
    mut state: *mut avifReformatState,
) -> avifBool {
    if (*rgb).width != (*image).width || (*rgb).height != (*image).height {
        return AVIF_FALSE;
    }
    let useYCgCoRe: avifBool = ((*image).matrixCoefficients as core::ffi::c_int
        == AVIF_MATRIX_COEFFICIENTS_YCGCO_RE as core::ffi::c_int) as core::ffi::c_int;
    let useYCgCoRo: avifBool = ((*image).matrixCoefficients as core::ffi::c_int
        == AVIF_MATRIX_COEFFICIENTS_YCGCO_RO as core::ffi::c_int) as core::ffi::c_int;
    if useYCgCoRe != 0 || useYCgCoRo != 0 {
        let bitOffset: core::ffi::c_int = if useYCgCoRe != 0 {
            2 as core::ffi::c_int
        } else {
            1 as core::ffi::c_int
        };
        if ((*image).depth).wrapping_sub(bitOffset as uint32_t) != (*rgb).depth {
            return AVIF_FALSE;
        }
    }
    if avifGetRGBColorSpaceInfo(rgb, &mut (*state).rgb) == 0 {
        return AVIF_FALSE;
    }
    if avifGetYUVColorSpaceInfo(image, &mut (*state).yuv) == 0 {
        return AVIF_FALSE;
    }
    (*state).yuv.mode = AVIF_REFORMAT_MODE_YUV_COEFFICIENTS;
    if (*image).matrixCoefficients as core::ffi::c_int
        == AVIF_MATRIX_COEFFICIENTS_IDENTITY as core::ffi::c_int
    {
        (*state).yuv.mode = AVIF_REFORMAT_MODE_IDENTITY;
    } else if (*image).matrixCoefficients as core::ffi::c_int
        == AVIF_MATRIX_COEFFICIENTS_YCGCO as core::ffi::c_int
    {
        (*state).yuv.mode = AVIF_REFORMAT_MODE_YCGCO;
    } else if useYCgCoRe != 0 {
        (*state).yuv.mode = AVIF_REFORMAT_MODE_YCGCO_RE;
    } else if useYCgCoRo != 0 {
        (*state).yuv.mode = AVIF_REFORMAT_MODE_YCGCO_RO;
    }
    if (*state).yuv.mode as core::ffi::c_uint
        != AVIF_REFORMAT_MODE_YUV_COEFFICIENTS as core::ffi::c_int as core::ffi::c_uint
    {
        (*state).yuv.kr = 0.0f32;
        (*state).yuv.kg = 0.0f32;
        (*state).yuv.kb = 0.0f32;
    }
    return AVIF_TRUE;
}
unsafe extern "C" fn avifYUVColorSpaceInfoYToUNorm(
    mut info: *mut avifYUVColorSpaceInfo,
    mut v: core::ffi::c_float,
) -> core::ffi::c_int {
    let mut unorm: core::ffi::c_int = avifRoundf(v * (*info).rangeY + (*info).biasY)
        as core::ffi::c_int;
    return if unorm < 0 as core::ffi::c_int {
        0 as core::ffi::c_int
    } else if (*info).maxChannel < unorm {
        (*info).maxChannel
    } else {
        unorm
    };
}
unsafe extern "C" fn avifYUVColorSpaceInfoUVToUNorm(
    mut info: *mut avifYUVColorSpaceInfo,
    mut v: core::ffi::c_float,
) -> core::ffi::c_int {
    let mut unorm: core::ffi::c_int = 0;
    'c_2035: {
        if (*info).mode as core::ffi::c_uint
            != AVIF_REFORMAT_MODE_YCGCO as core::ffi::c_int as core::ffi::c_uint
            && (*info).mode as core::ffi::c_uint
                != AVIF_REFORMAT_MODE_YCGCO_RE as core::ffi::c_int as core::ffi::c_uint
            && (*info).mode as core::ffi::c_uint
                != AVIF_REFORMAT_MODE_YCGCO_RO as core::ffi::c_int as core::ffi::c_uint
            || (*info).range as core::ffi::c_uint
                == AVIF_RANGE_FULL as core::ffi::c_int as core::ffi::c_uint
        {} else {
            __assert_fail(
                b"(info->mode != AVIF_REFORMAT_MODE_YCGCO && info->mode != AVIF_REFORMAT_MODE_YCGCO_RE && info->mode != AVIF_REFORMAT_MODE_YCGCO_RO) || (info->range == AVIF_RANGE_FULL)\0"
                    as *const u8 as *const core::ffi::c_char,
                b"/home/runner/work/libavif-rs/libavif-rs/upstream/libavif/src/reformat.c\0"
                    as *const u8 as *const core::ffi::c_char,
                214 as core::ffi::c_uint,
                (::core::mem::transmute::<
                    [u8; 67],
                    [core::ffi::c_char; 67],
                >(
                    *b"int avifYUVColorSpaceInfoUVToUNorm(avifYUVColorSpaceInfo *, float)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if (*info).mode as core::ffi::c_uint
        == AVIF_REFORMAT_MODE_IDENTITY as core::ffi::c_int as core::ffi::c_uint
    {
        unorm = avifRoundf(v * (*info).rangeY + (*info).biasY) as core::ffi::c_int;
    } else {
        unorm = avifRoundf(v * (*info).rangeUV + (*info).biasUV) as core::ffi::c_int;
    }
    return if unorm < 0 as core::ffi::c_int {
        0 as core::ffi::c_int
    } else if (*info).maxChannel < unorm {
        (*info).maxChannel
    } else {
        unorm
    };
}
#[no_mangle]
pub unsafe extern "C" fn avifImageRGBToYUV(
    mut image: *mut avifImage,
    mut rgb: *const avifRGBImage,
) -> avifResult {
    if ((*rgb).pixels).is_null()
        || (*rgb).format as core::ffi::c_uint
            == AVIF_RGB_FORMAT_RGB_565 as core::ffi::c_int as core::ffi::c_uint
    {
        return AVIF_RESULT_REFORMAT_FAILED;
    }
    let mut state: avifReformatState = avifReformatState {
        rgb: avifRGBColorSpaceInfo {
            channelBytes: 0,
            pixelBytes: 0,
            offsetBytesR: 0,
            offsetBytesG: 0,
            offsetBytesB: 0,
            offsetBytesA: 0,
            offsetBytesGray: 0,
            maxChannel: 0,
            maxChannelF: 0.,
        },
        yuv: avifYUVColorSpaceInfo {
            kr: 0.,
            kg: 0.,
            kb: 0.,
            channelBytes: 0,
            depth: 0,
            range: AVIF_RANGE_LIMITED,
            maxChannel: 0,
            biasY: 0.,
            biasUV: 0.,
            rangeY: 0.,
            rangeUV: 0.,
            formatInfo: avifPixelFormatInfo {
                monochrome: 0,
                chromaShiftX: 0,
                chromaShiftY: 0,
            },
            mode: AVIF_REFORMAT_MODE_YUV_COEFFICIENTS,
        },
    };
    if avifPrepareReformatState(image, rgb, &mut state) == 0 {
        return AVIF_RESULT_REFORMAT_FAILED;
    }
    if (*rgb).isFloat != 0 {
        return AVIF_RESULT_NOT_IMPLEMENTED;
    }
    let hasAlpha: avifBool = (avifRGBFormatHasAlpha((*rgb).format) != 0
        && (*rgb).ignoreAlpha == 0) as core::ffi::c_int;
    let mut allocationResult: avifResult = avifImageAllocatePlanes(
        image,
        (if hasAlpha != 0 {
            AVIF_PLANES_ALL as core::ffi::c_int
        } else {
            AVIF_PLANES_YUV as core::ffi::c_int
        }) as avifPlanesFlags,
    );
    if allocationResult as core::ffi::c_uint
        != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
    {
        return allocationResult;
    }
    let mut alphaMode: avifAlphaMultiplyMode = AVIF_ALPHA_MULTIPLY_MODE_NO_OP;
    if hasAlpha != 0 {
        if (*rgb).alphaPremultiplied == 0 && (*image).alphaPremultiplied != 0 {
            alphaMode = AVIF_ALPHA_MULTIPLY_MODE_MULTIPLY;
        } else if (*rgb).alphaPremultiplied != 0 && (*image).alphaPremultiplied == 0 {
            alphaMode = AVIF_ALPHA_MULTIPLY_MODE_UNMULTIPLY;
        }
    }
    let isGray: avifBool = avifRGBFormatIsGray((*rgb).format) as avifBool;
    let mut converted: avifBool = AVIF_FALSE;
    if isGray == 0 {
        if (*rgb).chromaDownsampling as core::ffi::c_uint
            == AVIF_CHROMA_DOWNSAMPLING_SHARP_YUV as core::ffi::c_int
                as core::ffi::c_uint
            && (*image).yuvFormat as core::ffi::c_uint
                == AVIF_PIXEL_FORMAT_YUV420 as core::ffi::c_int as core::ffi::c_uint
        {
            let libSharpYUVResult: avifResult = avifImageRGBToYUVLibSharpYUV(
                image,
                rgb,
                &mut state,
            ) as avifResult;
            if libSharpYUVResult as core::ffi::c_uint
                != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
            {
                return libSharpYUVResult;
            }
            converted = AVIF_TRUE as avifBool;
        }
        if converted == 0 && (*rgb).avoidLibYUV == 0
            && alphaMode as core::ffi::c_uint
                == AVIF_ALPHA_MULTIPLY_MODE_NO_OP as core::ffi::c_int
                    as core::ffi::c_uint
        {
            let mut libyuvResult: avifResult = avifImageRGBToYUVLibYUV(image, rgb);
            if libyuvResult as core::ffi::c_uint
                == AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
            {
                converted = AVIF_TRUE as avifBool;
            } else if libyuvResult as core::ffi::c_uint
                != AVIF_RESULT_NOT_IMPLEMENTED as core::ffi::c_int as core::ffi::c_uint
            {
                return libyuvResult
            }
        }
    }
    if converted == 0 && isGray == 0 {
        let kr: core::ffi::c_float = state.yuv.kr;
        let kg: core::ffi::c_float = state.yuv.kg;
        let kb: core::ffi::c_float = state.yuv.kb;
        let mut yuvBlock: [[YUVBlock; 2]; 2] = [[YUVBlock {
            y: 0.,
            u: 0.,
            v: 0.,
        }; 2]; 2];
        let mut rgbPixel: [core::ffi::c_float; 3] = [0.; 3];
        let rgbPixelBytes: uint32_t = state.rgb.pixelBytes;
        let offsetBytesR: uint32_t = state.rgb.offsetBytesR;
        let offsetBytesG: uint32_t = state.rgb.offsetBytesG;
        let offsetBytesB: uint32_t = state.rgb.offsetBytesB;
        let offsetBytesA: uint32_t = state.rgb.offsetBytesA;
        let rgbRowBytes: size_t = (*rgb).rowBytes as size_t;
        let rgbMaxChannelF: core::ffi::c_float = state.rgb.maxChannelF;
        let mut yPlane: *mut uint8_t = (*image)
            .yuvPlanes[AVIF_CHAN_Y as core::ffi::c_int as usize];
        let mut uPlane: *mut uint8_t = (*image)
            .yuvPlanes[AVIF_CHAN_U as core::ffi::c_int as usize];
        let mut vPlane: *mut uint8_t = (*image)
            .yuvPlanes[AVIF_CHAN_V as core::ffi::c_int as usize];
        let yRowBytes: size_t = (*image)
            .yuvRowBytes[AVIF_CHAN_Y as core::ffi::c_int as usize] as size_t;
        let uRowBytes: size_t = (*image)
            .yuvRowBytes[AVIF_CHAN_U as core::ffi::c_int as usize] as size_t;
        let vRowBytes: size_t = (*image)
            .yuvRowBytes[AVIF_CHAN_V as core::ffi::c_int as usize] as size_t;
        let mut outerJ: size_t = 0 as size_t;
        while outerJ < (*image).height as size_t {
            let mut outerI: size_t = 0 as size_t;
            while outerI < (*image).width as size_t {
                let mut blockW: uint32_t = 2 as uint32_t;
                let mut blockH: uint32_t = 2 as uint32_t;
                if outerI.wrapping_add(1 as size_t) >= (*image).width as size_t {
                    blockW = 1 as uint32_t;
                }
                if outerJ.wrapping_add(1 as size_t) >= (*image).height as size_t {
                    blockH = 1 as uint32_t;
                }
                let mut bJ: uint32_t = 0 as uint32_t;
                while bJ < blockH {
                    let mut bI: uint32_t = 0 as uint32_t;
                    while bI < blockW {
                        let i: size_t = outerI.wrapping_add(bI as size_t);
                        let j: size_t = outerJ.wrapping_add(bJ as size_t);
                        if state.rgb.channelBytes > 1 as uint32_t {
                            rgbPixel[0 as core::ffi::c_int as usize] = *(&mut *((*rgb)
                                .pixels)
                                .offset(
                                    (offsetBytesR as size_t)
                                        .wrapping_add(i.wrapping_mul(rgbPixelBytes as size_t))
                                        .wrapping_add(j.wrapping_mul(rgbRowBytes)) as isize,
                                ) as *mut uint8_t as *mut uint16_t) as core::ffi::c_int
                                as core::ffi::c_float / rgbMaxChannelF;
                            rgbPixel[1 as core::ffi::c_int as usize] = *(&mut *((*rgb)
                                .pixels)
                                .offset(
                                    (offsetBytesG as size_t)
                                        .wrapping_add(i.wrapping_mul(rgbPixelBytes as size_t))
                                        .wrapping_add(j.wrapping_mul(rgbRowBytes)) as isize,
                                ) as *mut uint8_t as *mut uint16_t) as core::ffi::c_int
                                as core::ffi::c_float / rgbMaxChannelF;
                            rgbPixel[2 as core::ffi::c_int as usize] = *(&mut *((*rgb)
                                .pixels)
                                .offset(
                                    (offsetBytesB as size_t)
                                        .wrapping_add(i.wrapping_mul(rgbPixelBytes as size_t))
                                        .wrapping_add(j.wrapping_mul(rgbRowBytes)) as isize,
                                ) as *mut uint8_t as *mut uint16_t) as core::ffi::c_int
                                as core::ffi::c_float / rgbMaxChannelF;
                        } else {
                            rgbPixel[0 as core::ffi::c_int as usize] = *((*rgb).pixels)
                                .offset(
                                    (offsetBytesR as size_t)
                                        .wrapping_add(i.wrapping_mul(rgbPixelBytes as size_t))
                                        .wrapping_add(j.wrapping_mul(rgbRowBytes)) as isize,
                                ) as core::ffi::c_int as core::ffi::c_float
                                / rgbMaxChannelF;
                            rgbPixel[1 as core::ffi::c_int as usize] = *((*rgb).pixels)
                                .offset(
                                    (offsetBytesG as size_t)
                                        .wrapping_add(i.wrapping_mul(rgbPixelBytes as size_t))
                                        .wrapping_add(j.wrapping_mul(rgbRowBytes)) as isize,
                                ) as core::ffi::c_int as core::ffi::c_float
                                / rgbMaxChannelF;
                            rgbPixel[2 as core::ffi::c_int as usize] = *((*rgb).pixels)
                                .offset(
                                    (offsetBytesB as size_t)
                                        .wrapping_add(i.wrapping_mul(rgbPixelBytes as size_t))
                                        .wrapping_add(j.wrapping_mul(rgbRowBytes)) as isize,
                                ) as core::ffi::c_int as core::ffi::c_float
                                / rgbMaxChannelF;
                        }
                        if alphaMode as core::ffi::c_uint
                            != AVIF_ALPHA_MULTIPLY_MODE_NO_OP as core::ffi::c_int
                                as core::ffi::c_uint
                        {
                            let mut a: core::ffi::c_float = 0.;
                            if state.rgb.channelBytes > 1 as uint32_t {
                                a = *(&mut *((*rgb).pixels)
                                    .offset(
                                        (offsetBytesA as size_t)
                                            .wrapping_add(i.wrapping_mul(rgbPixelBytes as size_t))
                                            .wrapping_add(j.wrapping_mul(rgbRowBytes)) as isize,
                                    ) as *mut uint8_t as *mut uint16_t) as core::ffi::c_int
                                    as core::ffi::c_float / rgbMaxChannelF;
                            } else {
                                a = *((*rgb).pixels)
                                    .offset(
                                        (offsetBytesA as size_t)
                                            .wrapping_add(i.wrapping_mul(rgbPixelBytes as size_t))
                                            .wrapping_add(j.wrapping_mul(rgbRowBytes)) as isize,
                                    ) as core::ffi::c_int as core::ffi::c_float
                                    / rgbMaxChannelF;
                            }
                            if alphaMode as core::ffi::c_uint
                                == AVIF_ALPHA_MULTIPLY_MODE_MULTIPLY as core::ffi::c_int
                                    as core::ffi::c_uint
                            {
                                if a == 0 as core::ffi::c_int as core::ffi::c_float {
                                    rgbPixel[0 as core::ffi::c_int as usize] = 0
                                        as core::ffi::c_int as core::ffi::c_float;
                                    rgbPixel[1 as core::ffi::c_int as usize] = 0
                                        as core::ffi::c_int as core::ffi::c_float;
                                    rgbPixel[2 as core::ffi::c_int as usize] = 0
                                        as core::ffi::c_int as core::ffi::c_float;
                                } else if a < 1.0f32 {
                                    rgbPixel[0 as core::ffi::c_int as usize] *= a;
                                    rgbPixel[1 as core::ffi::c_int as usize] *= a;
                                    rgbPixel[2 as core::ffi::c_int as usize] *= a;
                                }
                            } else if a == 0 as core::ffi::c_int as core::ffi::c_float {
                                rgbPixel[0 as core::ffi::c_int as usize] = 0
                                    as core::ffi::c_int as core::ffi::c_float;
                                rgbPixel[1 as core::ffi::c_int as usize] = 0
                                    as core::ffi::c_int as core::ffi::c_float;
                                rgbPixel[2 as core::ffi::c_int as usize] = 0
                                    as core::ffi::c_int as core::ffi::c_float;
                            } else if a < 1.0f32 {
                                rgbPixel[0 as core::ffi::c_int as usize] /= a;
                                rgbPixel[1 as core::ffi::c_int as usize] /= a;
                                rgbPixel[2 as core::ffi::c_int as usize] /= a;
                                rgbPixel[0 as core::ffi::c_int as usize] = if rgbPixel[0
                                    as core::ffi::c_int as usize] < 1.0f32
                                {
                                    rgbPixel[0 as core::ffi::c_int as usize]
                                } else {
                                    1.0f32
                                };
                                rgbPixel[1 as core::ffi::c_int as usize] = if rgbPixel[1
                                    as core::ffi::c_int as usize] < 1.0f32
                                {
                                    rgbPixel[1 as core::ffi::c_int as usize]
                                } else {
                                    1.0f32
                                };
                                rgbPixel[2 as core::ffi::c_int as usize] = if rgbPixel[2
                                    as core::ffi::c_int as usize] < 1.0f32
                                {
                                    rgbPixel[2 as core::ffi::c_int as usize]
                                } else {
                                    1.0f32
                                };
                            }
                        }
                        if state.yuv.mode as core::ffi::c_uint
                            == AVIF_REFORMAT_MODE_IDENTITY as core::ffi::c_int
                                as core::ffi::c_uint
                        {
                            yuvBlock[bI as usize][bJ as usize].y = rgbPixel[1
                                as core::ffi::c_int as usize];
                            yuvBlock[bI as usize][bJ as usize].u = rgbPixel[2
                                as core::ffi::c_int as usize];
                            yuvBlock[bI as usize][bJ as usize].v = rgbPixel[0
                                as core::ffi::c_int as usize];
                        } else if state.yuv.mode as core::ffi::c_uint
                            == AVIF_REFORMAT_MODE_YCGCO as core::ffi::c_int
                                as core::ffi::c_uint
                        {
                            yuvBlock[bI as usize][bJ as usize].y = 0.5f32
                                * rgbPixel[1 as core::ffi::c_int as usize]
                                + 0.25f32
                                    * (rgbPixel[0 as core::ffi::c_int as usize]
                                        + rgbPixel[2 as core::ffi::c_int as usize]);
                            yuvBlock[bI as usize][bJ as usize].u = 0.5f32
                                * rgbPixel[1 as core::ffi::c_int as usize]
                                - 0.25f32
                                    * (rgbPixel[0 as core::ffi::c_int as usize]
                                        + rgbPixel[2 as core::ffi::c_int as usize]);
                            yuvBlock[bI as usize][bJ as usize].v = 0.5f32
                                * (rgbPixel[0 as core::ffi::c_int as usize]
                                    - rgbPixel[2 as core::ffi::c_int as usize]);
                        } else if state.yuv.mode as core::ffi::c_uint
                            == AVIF_REFORMAT_MODE_YCGCO_RE as core::ffi::c_int
                                as core::ffi::c_uint
                            || state.yuv.mode as core::ffi::c_uint
                                == AVIF_REFORMAT_MODE_YCGCO_RO as core::ffi::c_int
                                    as core::ffi::c_uint
                        {
                            let R: core::ffi::c_int = avifRoundf(
                                if rgbPixel[0 as core::ffi::c_int as usize] * rgbMaxChannelF
                                    < 0.0f32
                                {
                                    0.0f32
                                } else if rgbMaxChannelF
                                    < rgbPixel[0 as core::ffi::c_int as usize] * rgbMaxChannelF
                                {
                                    rgbMaxChannelF
                                } else {
                                    rgbPixel[0 as core::ffi::c_int as usize] * rgbMaxChannelF
                                },
                            ) as core::ffi::c_int;
                            let G: core::ffi::c_int = avifRoundf(
                                if rgbPixel[1 as core::ffi::c_int as usize] * rgbMaxChannelF
                                    < 0.0f32
                                {
                                    0.0f32
                                } else if rgbMaxChannelF
                                    < rgbPixel[1 as core::ffi::c_int as usize] * rgbMaxChannelF
                                {
                                    rgbMaxChannelF
                                } else {
                                    rgbPixel[1 as core::ffi::c_int as usize] * rgbMaxChannelF
                                },
                            ) as core::ffi::c_int;
                            let B: core::ffi::c_int = avifRoundf(
                                if rgbPixel[2 as core::ffi::c_int as usize] * rgbMaxChannelF
                                    < 0.0f32
                                {
                                    0.0f32
                                } else if rgbMaxChannelF
                                    < rgbPixel[2 as core::ffi::c_int as usize] * rgbMaxChannelF
                                {
                                    rgbMaxChannelF
                                } else {
                                    rgbPixel[2 as core::ffi::c_int as usize] * rgbMaxChannelF
                                },
                            ) as core::ffi::c_int;
                            let Co: core::ffi::c_int = R - B;
                            let t: core::ffi::c_int = B + (Co >> 1 as core::ffi::c_int);
                            let Cg: core::ffi::c_int = G - t;
                            yuvBlock[bI as usize][bJ as usize].y = (t
                                + (Cg >> 1 as core::ffi::c_int)) as core::ffi::c_float
                                / state.yuv.rangeY;
                            yuvBlock[bI as usize][bJ as usize].u = Cg
                                as core::ffi::c_float / state.yuv.rangeUV;
                            yuvBlock[bI as usize][bJ as usize].v = Co
                                as core::ffi::c_float / state.yuv.rangeUV;
                        } else {
                            let mut Y: core::ffi::c_float = kr
                                * rgbPixel[0 as core::ffi::c_int as usize]
                                + kg * rgbPixel[1 as core::ffi::c_int as usize]
                                + kb * rgbPixel[2 as core::ffi::c_int as usize];
                            yuvBlock[bI as usize][bJ as usize].y = Y;
                            yuvBlock[bI as usize][bJ as usize].u = (rgbPixel[2
                                as core::ffi::c_int as usize] - Y)
                                / (2 as core::ffi::c_int as core::ffi::c_float
                                    * (1 as core::ffi::c_int as core::ffi::c_float - kb));
                            yuvBlock[bI as usize][bJ as usize].v = (rgbPixel[0
                                as core::ffi::c_int as usize] - Y)
                                / (2 as core::ffi::c_int as core::ffi::c_float
                                    * (1 as core::ffi::c_int as core::ffi::c_float - kr));
                        }
                        if state.yuv.channelBytes > 1 as uint32_t {
                            let mut pY: *mut uint16_t = &mut *yPlane
                                .offset(
                                    i
                                        .wrapping_mul(2 as size_t)
                                        .wrapping_add(j.wrapping_mul(yRowBytes)) as isize,
                                ) as *mut uint8_t as *mut uint16_t;
                            *pY = avifYUVColorSpaceInfoYToUNorm(
                                &mut state.yuv,
                                yuvBlock[bI as usize][bJ as usize].y,
                            ) as uint16_t;
                            if (*image).yuvFormat as core::ffi::c_uint
                                == AVIF_PIXEL_FORMAT_YUV444 as core::ffi::c_int
                                    as core::ffi::c_uint
                            {
                                let mut pU: *mut uint16_t = &mut *uPlane
                                    .offset(
                                        i
                                            .wrapping_mul(2 as size_t)
                                            .wrapping_add(j.wrapping_mul(uRowBytes)) as isize,
                                    ) as *mut uint8_t as *mut uint16_t;
                                *pU = avifYUVColorSpaceInfoUVToUNorm(
                                    &mut state.yuv,
                                    yuvBlock[bI as usize][bJ as usize].u,
                                ) as uint16_t;
                                let mut pV: *mut uint16_t = &mut *vPlane
                                    .offset(
                                        i
                                            .wrapping_mul(2 as size_t)
                                            .wrapping_add(j.wrapping_mul(vRowBytes)) as isize,
                                    ) as *mut uint8_t as *mut uint16_t;
                                *pV = avifYUVColorSpaceInfoUVToUNorm(
                                    &mut state.yuv,
                                    yuvBlock[bI as usize][bJ as usize].v,
                                ) as uint16_t;
                            }
                        } else {
                            *yPlane
                                .offset(
                                    i.wrapping_add(j.wrapping_mul(yRowBytes)) as isize,
                                ) = avifYUVColorSpaceInfoYToUNorm(
                                &mut state.yuv,
                                yuvBlock[bI as usize][bJ as usize].y,
                            ) as uint8_t;
                            if (*image).yuvFormat as core::ffi::c_uint
                                == AVIF_PIXEL_FORMAT_YUV444 as core::ffi::c_int
                                    as core::ffi::c_uint
                            {
                                *uPlane
                                    .offset(
                                        i.wrapping_add(j.wrapping_mul(uRowBytes)) as isize,
                                    ) = avifYUVColorSpaceInfoUVToUNorm(
                                    &mut state.yuv,
                                    yuvBlock[bI as usize][bJ as usize].u,
                                ) as uint8_t;
                                *vPlane
                                    .offset(
                                        i.wrapping_add(j.wrapping_mul(vRowBytes)) as isize,
                                    ) = avifYUVColorSpaceInfoUVToUNorm(
                                    &mut state.yuv,
                                    yuvBlock[bI as usize][bJ as usize].v,
                                ) as uint8_t;
                            }
                        }
                        bI = bI.wrapping_add(1);
                    }
                    bJ = bJ.wrapping_add(1);
                }
                if !((*image).yuvFormat as core::ffi::c_uint
                    == AVIF_PIXEL_FORMAT_YUV400 as core::ffi::c_int as core::ffi::c_uint)
                {
                    if (*image).yuvFormat as core::ffi::c_uint
                        == AVIF_PIXEL_FORMAT_YUV420 as core::ffi::c_int
                            as core::ffi::c_uint
                    {
                        let mut sumU: core::ffi::c_float = 0.0f32;
                        let mut sumV: core::ffi::c_float = 0.0f32;
                        let mut bJ_0: uint32_t = 0 as uint32_t;
                        while bJ_0 < blockH {
                            let mut bI_0: uint32_t = 0 as uint32_t;
                            while bI_0 < blockW {
                                sumU += yuvBlock[bI_0 as usize][bJ_0 as usize].u;
                                sumV += yuvBlock[bI_0 as usize][bJ_0 as usize].v;
                                bI_0 = bI_0.wrapping_add(1);
                            }
                            bJ_0 = bJ_0.wrapping_add(1);
                        }
                        let mut totalSamples: core::ffi::c_float = blockW
                            .wrapping_mul(blockH) as core::ffi::c_float;
                        let mut avgU: core::ffi::c_float = sumU / totalSamples;
                        let mut avgV: core::ffi::c_float = sumV / totalSamples;
                        let chromaShiftX: core::ffi::c_int = 1 as core::ffi::c_int;
                        let chromaShiftY: core::ffi::c_int = 1 as core::ffi::c_int;
                        let mut uvI: size_t = outerI >> chromaShiftX;
                        let mut uvJ: size_t = outerJ >> chromaShiftY;
                        if state.yuv.channelBytes > 1 as uint32_t {
                            let mut pU_0: *mut uint16_t = &mut *uPlane
                                .offset(
                                    uvI
                                        .wrapping_mul(2 as size_t)
                                        .wrapping_add(uvJ.wrapping_mul(uRowBytes)) as isize,
                                ) as *mut uint8_t as *mut uint16_t;
                            *pU_0 = avifYUVColorSpaceInfoUVToUNorm(&mut state.yuv, avgU)
                                as uint16_t;
                            let mut pV_0: *mut uint16_t = &mut *vPlane
                                .offset(
                                    uvI
                                        .wrapping_mul(2 as size_t)
                                        .wrapping_add(uvJ.wrapping_mul(vRowBytes)) as isize,
                                ) as *mut uint8_t as *mut uint16_t;
                            *pV_0 = avifYUVColorSpaceInfoUVToUNorm(&mut state.yuv, avgV)
                                as uint16_t;
                        } else {
                            *uPlane
                                .offset(
                                    uvI.wrapping_add(uvJ.wrapping_mul(uRowBytes)) as isize,
                                ) = avifYUVColorSpaceInfoUVToUNorm(&mut state.yuv, avgU)
                                as uint8_t;
                            *vPlane
                                .offset(
                                    uvI.wrapping_add(uvJ.wrapping_mul(vRowBytes)) as isize,
                                ) = avifYUVColorSpaceInfoUVToUNorm(&mut state.yuv, avgV)
                                as uint8_t;
                        }
                    } else if (*image).yuvFormat as core::ffi::c_uint
                        == AVIF_PIXEL_FORMAT_YUV422 as core::ffi::c_int
                            as core::ffi::c_uint
                    {
                        let mut bJ_1: uint32_t = 0 as uint32_t;
                        while bJ_1 < blockH {
                            let mut sumU_0: core::ffi::c_float = 0.0f32;
                            let mut sumV_0: core::ffi::c_float = 0.0f32;
                            let mut bI_1: uint32_t = 0 as uint32_t;
                            while bI_1 < blockW {
                                sumU_0 += yuvBlock[bI_1 as usize][bJ_1 as usize].u;
                                sumV_0 += yuvBlock[bI_1 as usize][bJ_1 as usize].v;
                                bI_1 = bI_1.wrapping_add(1);
                            }
                            let mut totalSamples_0: core::ffi::c_float = blockW
                                as core::ffi::c_float;
                            let mut avgU_0: core::ffi::c_float = sumU_0 / totalSamples_0;
                            let mut avgV_0: core::ffi::c_float = sumV_0 / totalSamples_0;
                            let chromaShiftX_0: core::ffi::c_int = 1 as core::ffi::c_int;
                            let mut uvI_0: size_t = outerI >> chromaShiftX_0;
                            let mut uvJ_0: size_t = outerJ.wrapping_add(bJ_1 as size_t);
                            if state.yuv.channelBytes > 1 as uint32_t {
                                let mut pU_1: *mut uint16_t = &mut *uPlane
                                    .offset(
                                        uvI_0
                                            .wrapping_mul(2 as size_t)
                                            .wrapping_add(uvJ_0.wrapping_mul(uRowBytes)) as isize,
                                    ) as *mut uint8_t as *mut uint16_t;
                                *pU_1 = avifYUVColorSpaceInfoUVToUNorm(
                                    &mut state.yuv,
                                    avgU_0,
                                ) as uint16_t;
                                let mut pV_1: *mut uint16_t = &mut *vPlane
                                    .offset(
                                        uvI_0
                                            .wrapping_mul(2 as size_t)
                                            .wrapping_add(uvJ_0.wrapping_mul(vRowBytes)) as isize,
                                    ) as *mut uint8_t as *mut uint16_t;
                                *pV_1 = avifYUVColorSpaceInfoUVToUNorm(
                                    &mut state.yuv,
                                    avgV_0,
                                ) as uint16_t;
                            } else {
                                *uPlane
                                    .offset(
                                        uvI_0.wrapping_add(uvJ_0.wrapping_mul(uRowBytes)) as isize,
                                    ) = avifYUVColorSpaceInfoUVToUNorm(&mut state.yuv, avgU_0)
                                    as uint8_t;
                                *vPlane
                                    .offset(
                                        uvI_0.wrapping_add(uvJ_0.wrapping_mul(vRowBytes)) as isize,
                                    ) = avifYUVColorSpaceInfoUVToUNorm(&mut state.yuv, avgV_0)
                                    as uint8_t;
                            }
                            bJ_1 = bJ_1.wrapping_add(1);
                        }
                    }
                }
                outerI = outerI.wrapping_add(2 as size_t);
            }
            outerJ = outerJ.wrapping_add(2 as size_t);
        }
    } else if converted == 0 && isGray != 0 {
        let grayPixelBytes: uint32_t = state.rgb.pixelBytes;
        let offsetBytesGray: uint32_t = state.rgb.offsetBytesGray;
        let offsetBytesA_0: uint32_t = state.rgb.offsetBytesA;
        let grayRowBytes: size_t = (*rgb).rowBytes as size_t;
        let grayMaxChannelF: core::ffi::c_float = state.rgb.maxChannelF;
        let mut yPlane_0: *mut uint8_t = (*image)
            .yuvPlanes[AVIF_CHAN_Y as core::ffi::c_int as usize];
        let yRowBytes_0: size_t = (*image)
            .yuvRowBytes[AVIF_CHAN_Y as core::ffi::c_int as usize] as size_t;
        let mut j_0: size_t = 0 as size_t;
        while j_0 < (*image).height as size_t {
            let mut i_0: size_t = 0 as size_t;
            while i_0 < (*image).width as size_t {
                let mut g: core::ffi::c_float = 0.;
                if state.rgb.channelBytes > 1 as uint32_t {
                    g = *(&mut *((*rgb).pixels)
                        .offset(
                            (offsetBytesGray as size_t)
                                .wrapping_add(i_0.wrapping_mul(grayPixelBytes as size_t))
                                .wrapping_add(j_0.wrapping_mul(grayRowBytes)) as isize,
                        ) as *mut uint8_t as *mut uint16_t) as core::ffi::c_int
                        as core::ffi::c_float / grayMaxChannelF;
                } else {
                    g = *((*rgb).pixels)
                        .offset(
                            (offsetBytesGray as size_t)
                                .wrapping_add(i_0.wrapping_mul(grayPixelBytes as size_t))
                                .wrapping_add(j_0.wrapping_mul(grayRowBytes)) as isize,
                        ) as core::ffi::c_int as core::ffi::c_float / grayMaxChannelF;
                }
                if alphaMode as core::ffi::c_uint
                    != AVIF_ALPHA_MULTIPLY_MODE_NO_OP as core::ffi::c_int
                        as core::ffi::c_uint
                {
                    let mut a_0: core::ffi::c_float = 0.;
                    if state.rgb.channelBytes > 1 as uint32_t {
                        a_0 = *(&mut *((*rgb).pixels)
                            .offset(
                                (offsetBytesA_0 as size_t)
                                    .wrapping_add(i_0.wrapping_mul(grayPixelBytes as size_t))
                                    .wrapping_add(j_0.wrapping_mul(grayRowBytes)) as isize,
                            ) as *mut uint8_t as *mut uint16_t) as core::ffi::c_int
                            as core::ffi::c_float / grayMaxChannelF;
                    } else {
                        a_0 = *((*rgb).pixels)
                            .offset(
                                (offsetBytesA_0 as size_t)
                                    .wrapping_add(i_0.wrapping_mul(grayPixelBytes as size_t))
                                    .wrapping_add(j_0.wrapping_mul(grayRowBytes)) as isize,
                            ) as core::ffi::c_int as core::ffi::c_float
                            / grayMaxChannelF;
                    }
                    if alphaMode as core::ffi::c_uint
                        == AVIF_ALPHA_MULTIPLY_MODE_MULTIPLY as core::ffi::c_int
                            as core::ffi::c_uint
                    {
                        if a_0 == 0 as core::ffi::c_int as core::ffi::c_float {
                            g = 0 as core::ffi::c_int as core::ffi::c_float;
                        } else if a_0 < 1.0f32 {
                            g *= a_0;
                        }
                    } else if a_0 == 0 as core::ffi::c_int as core::ffi::c_float {
                        g = 0 as core::ffi::c_int as core::ffi::c_float;
                    } else if a_0 < 1.0f32 {
                        g /= a_0;
                        g = if g < 1.0f32 { g } else { 1.0f32 };
                    }
                }
                let mut gInt: core::ffi::c_int = avifYUVColorSpaceInfoYToUNorm(
                    &mut state.yuv,
                    g,
                );
                if state.yuv.channelBytes > 1 as uint32_t {
                    let mut pY_0: *mut uint16_t = &mut *yPlane_0
                        .offset(
                            i_0
                                .wrapping_mul(2 as size_t)
                                .wrapping_add(j_0.wrapping_mul(yRowBytes_0)) as isize,
                        ) as *mut uint8_t as *mut uint16_t;
                    *pY_0 = gInt as uint16_t;
                } else {
                    *yPlane_0
                        .offset(
                            i_0.wrapping_add(j_0.wrapping_mul(yRowBytes_0)) as isize,
                        ) = gInt as uint8_t;
                }
                i_0 = i_0.wrapping_add(1);
            }
            j_0 = j_0.wrapping_add(1);
        }
        let mut info: avifPixelFormatInfo = avifPixelFormatInfo {
            monochrome: 0,
            chromaShiftX: 0,
            chromaShiftY: 0,
        };
        avifGetPixelFormatInfo((*image).yuvFormat, &mut info);
        let shiftedH: uint32_t = (((*image).height as uint64_t)
            .wrapping_add(info.chromaShiftY as uint64_t) >> info.chromaShiftY)
            as uint32_t;
        let half: core::ffi::c_int = (1 as core::ffi::c_int)
            << ((*image).depth).wrapping_sub(1 as uint32_t);
        if !((*image).yuvPlanes[AVIF_CHAN_U as core::ffi::c_int as usize]).is_null() {
            let mut uPlane_0: *mut uint8_t = (*image)
                .yuvPlanes[AVIF_CHAN_U as core::ffi::c_int as usize];
            let uRowBytes_0: size_t = (*image)
                .yuvRowBytes[AVIF_CHAN_U as core::ffi::c_int as usize] as size_t;
            if state.yuv.channelBytes > 1 as uint32_t {
                avifMemset16(
                    uPlane_0 as *mut core::ffi::c_void,
                    half,
                    (shiftedH as size_t)
                        .wrapping_mul(uRowBytes_0)
                        .wrapping_div(2 as size_t),
                );
            } else {
                memset(
                    uPlane_0 as *mut core::ffi::c_void,
                    half,
                    (shiftedH as size_t).wrapping_mul(uRowBytes_0),
                );
            }
        }
        if !((*image).yuvPlanes[AVIF_CHAN_V as core::ffi::c_int as usize]).is_null() {
            let mut vPlane_0: *mut uint8_t = (*image)
                .yuvPlanes[AVIF_CHAN_V as core::ffi::c_int as usize];
            let vRowBytes_0: size_t = (*image)
                .yuvRowBytes[AVIF_CHAN_V as core::ffi::c_int as usize] as size_t;
            if state.yuv.channelBytes > 1 as uint32_t {
                avifMemset16(
                    vPlane_0 as *mut core::ffi::c_void,
                    half,
                    (shiftedH as size_t)
                        .wrapping_mul(vRowBytes_0)
                        .wrapping_div(2 as size_t),
                );
            } else {
                memset(
                    vPlane_0 as *mut core::ffi::c_void,
                    half,
                    (shiftedH as size_t).wrapping_mul(vRowBytes_0),
                );
            }
        }
    }
    if !((*image).alphaPlane).is_null() && (*image).alphaRowBytes != 0 {
        let mut params: avifAlphaParams = avifAlphaParams {
            width: 0,
            height: 0,
            srcDepth: 0,
            srcPlane: 0 as *const uint8_t,
            srcRowBytes: 0,
            srcOffsetBytes: 0,
            srcPixelBytes: 0,
            dstDepth: 0,
            dstPlane: 0 as *mut uint8_t,
            dstRowBytes: 0,
            dstOffsetBytes: 0,
            dstPixelBytes: 0,
        };
        params.width = (*image).width;
        params.height = (*image).height;
        params.dstDepth = (*image).depth;
        params.dstPlane = (*image).alphaPlane;
        params.dstRowBytes = (*image).alphaRowBytes;
        params.dstOffsetBytes = 0 as uint32_t;
        params.dstPixelBytes = state.yuv.channelBytes;
        if avifRGBFormatHasAlpha((*rgb).format) != 0 && (*rgb).ignoreAlpha == 0 {
            params.srcDepth = (*rgb).depth;
            params.srcPlane = (*rgb).pixels;
            params.srcRowBytes = (*rgb).rowBytes;
            params.srcOffsetBytes = state.rgb.offsetBytesA;
            params.srcPixelBytes = state.rgb.pixelBytes;
            avifReformatAlpha(&mut params);
        } else {
            avifFillAlpha(&mut params);
        }
    }
    return AVIF_RESULT_OK;
}
unsafe extern "C" fn avifCreateYUVToRGBLookUpTables(
    mut unormFloatTableY: *mut *mut core::ffi::c_float,
    mut unormFloatTableUV: *mut *mut core::ffi::c_float,
    mut depth: uint32_t,
    mut state: *const avifReformatState,
) -> avifBool {
    let cpCount: size_t = (1 as core::ffi::c_int as size_t) << depth;
    'c_9092: {
        if !unormFloatTableY.is_null() {} else {
            __assert_fail(
                b"unormFloatTableY\0" as *const u8 as *const core::ffi::c_char,
                b"/home/runner/work/libavif-rs/libavif-rs/upstream/libavif/src/reformat.c\0"
                    as *const u8 as *const core::ffi::c_char,
                583 as core::ffi::c_uint,
                (::core::mem::transmute::<
                    [u8; 97],
                    [core::ffi::c_char; 97],
                >(
                    *b"avifBool avifCreateYUVToRGBLookUpTables(float **, float **, uint32_t, const avifReformatState *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    *unormFloatTableY = avifAlloc(
        cpCount.wrapping_mul(::core::mem::size_of::<core::ffi::c_float>() as size_t),
    ) as *mut core::ffi::c_float;
    if (*unormFloatTableY).is_null() {
        return AVIF_FALSE;
    }
    let mut cp: uint32_t = 0 as uint32_t;
    while (cp as size_t) < cpCount {
        *(*unormFloatTableY).offset(cp as isize) = (cp as core::ffi::c_float
            - (*state).yuv.biasY) / (*state).yuv.rangeY;
        cp = cp.wrapping_add(1);
    }
    if !unormFloatTableUV.is_null() {
        if (*state).yuv.mode as core::ffi::c_uint
            == AVIF_REFORMAT_MODE_IDENTITY as core::ffi::c_int as core::ffi::c_uint
        {
            *unormFloatTableUV = *unormFloatTableY;
        } else {
            *unormFloatTableUV = avifAlloc(
                cpCount
                    .wrapping_mul(::core::mem::size_of::<core::ffi::c_float>() as size_t),
            ) as *mut core::ffi::c_float;
            if (*unormFloatTableUV).is_null() {
                avifFree(*unormFloatTableY as *mut core::ffi::c_void);
                *unormFloatTableY = 0 as *mut core::ffi::c_float;
                return AVIF_FALSE;
            }
            let mut cp_0: uint32_t = 0 as uint32_t;
            while (cp_0 as size_t) < cpCount {
                *(*unormFloatTableUV).offset(cp_0 as isize) = (cp_0 as core::ffi::c_float
                    - (*state).yuv.biasUV) / (*state).yuv.rangeUV;
                cp_0 = cp_0.wrapping_add(1);
            }
        }
    }
    return AVIF_TRUE;
}
unsafe extern "C" fn avifFreeYUVToRGBLookUpTables(
    mut unormFloatTableY: *mut *mut core::ffi::c_float,
    mut unormFloatTableUV: *mut *mut core::ffi::c_float,
) {
    if !unormFloatTableUV.is_null() {
        if *unormFloatTableUV != *unormFloatTableY {
            avifFree(*unormFloatTableUV as *mut core::ffi::c_void);
        }
        *unormFloatTableUV = 0 as *mut core::ffi::c_float;
    }
    avifFree(*unormFloatTableY as *mut core::ffi::c_void);
    *unormFloatTableY = 0 as *mut core::ffi::c_float;
}
unsafe extern "C" fn avifStoreRGB8Pixel(
    mut format: avifRGBFormat,
    mut R: uint8_t,
    mut G: uint8_t,
    mut B: uint8_t,
    mut ptrR: *mut uint8_t,
    mut ptrG: *mut uint8_t,
    mut ptrB: *mut uint8_t,
) {
    if format as core::ffi::c_uint
        == AVIF_RGB_FORMAT_RGB_565 as core::ffi::c_int as core::ffi::c_uint
    {
        *(ptrR as *mut uint16_t) = (B as core::ffi::c_int >> 3 as core::ffi::c_int
            | (G as core::ffi::c_int >> 2 as core::ffi::c_int) << 5 as core::ffi::c_int
            | (R as core::ffi::c_int >> 3 as core::ffi::c_int) << 11 as core::ffi::c_int)
            as uint16_t;
        return;
    }
    *ptrR = R;
    *ptrG = G;
    *ptrB = B;
}
unsafe extern "C" fn avifGetRGB565(
    mut ptrR: *const uint8_t,
    mut R: *mut uint8_t,
    mut G: *mut uint8_t,
    mut B: *mut uint8_t,
) {
    let rgb656: uint16_t = *(ptrR as *const uint16_t)
        .offset(0 as core::ffi::c_int as isize);
    let r5: uint16_t = ((rgb656 as core::ffi::c_int & 0xf800 as core::ffi::c_int)
        >> 11 as core::ffi::c_int) as uint16_t;
    let g6: uint16_t = ((rgb656 as core::ffi::c_int & 0x7e0 as core::ffi::c_int)
        >> 5 as core::ffi::c_int) as uint16_t;
    let b5: uint16_t = (rgb656 as core::ffi::c_int & 0x1f as core::ffi::c_int)
        as uint16_t;
    *R = ((r5 as core::ffi::c_int) << 3 as core::ffi::c_int
        | r5 as core::ffi::c_int >> 2 as core::ffi::c_int) as uint8_t;
    *G = ((g6 as core::ffi::c_int) << 2 as core::ffi::c_int
        | g6 as core::ffi::c_int >> 4 as core::ffi::c_int) as uint8_t;
    *B = ((b5 as core::ffi::c_int) << 3 as core::ffi::c_int
        | b5 as core::ffi::c_int >> 2 as core::ffi::c_int) as uint8_t;
}
unsafe extern "C" fn avifImageYUVAnyToRGBAnySlow(
    mut image: *const avifImage,
    mut rgb: *mut avifRGBImage,
    mut state: *const avifReformatState,
    mut alphaMultiplyMode: avifAlphaMultiplyMode,
) -> avifResult {
    let kr: core::ffi::c_float = (*state).yuv.kr;
    let kg: core::ffi::c_float = (*state).yuv.kg;
    let kb: core::ffi::c_float = (*state).yuv.kb;
    let mut unormFloatTableY: *mut core::ffi::c_float = 0 as *mut core::ffi::c_float;
    let mut unormFloatTableUV: *mut core::ffi::c_float = 0 as *mut core::ffi::c_float;
    if avifCreateYUVToRGBLookUpTables(
        &mut unormFloatTableY,
        &mut unormFloatTableUV,
        (*image).depth,
        state,
    ) == 0
    {
        return AVIF_RESULT_OUT_OF_MEMORY;
    }
    let yuvChannelBytes: size_t = (*state).yuv.channelBytes as size_t;
    let rgbPixelBytes: uint32_t = (*state).rgb.pixelBytes;
    let mut yPlane: *const uint8_t = (*image)
        .yuvPlanes[AVIF_CHAN_Y as core::ffi::c_int as usize];
    let mut uPlane: *const uint8_t = (*image)
        .yuvPlanes[AVIF_CHAN_U as core::ffi::c_int as usize];
    let mut vPlane: *const uint8_t = (*image)
        .yuvPlanes[AVIF_CHAN_V as core::ffi::c_int as usize];
    let mut aPlane: *const uint8_t = (*image).alphaPlane;
    let yRowBytes: size_t = (*image)
        .yuvRowBytes[AVIF_CHAN_Y as core::ffi::c_int as usize] as size_t;
    let uRowBytes: size_t = (*image)
        .yuvRowBytes[AVIF_CHAN_U as core::ffi::c_int as usize] as size_t;
    let vRowBytes: size_t = (*image)
        .yuvRowBytes[AVIF_CHAN_V as core::ffi::c_int as usize] as size_t;
    let aRowBytes: size_t = (*image).alphaRowBytes as size_t;
    let yuvHasColor: avifBool = (!uPlane.is_null() && !vPlane.is_null()
        && (*image).yuvFormat as core::ffi::c_uint
            != AVIF_PIXEL_FORMAT_YUV400 as core::ffi::c_int as core::ffi::c_uint)
        as core::ffi::c_int;
    let rgbHasColor: avifBool = (avifRGBFormatIsGray((*rgb).format) == 0)
        as core::ffi::c_int;
    let yuvMaxChannel: uint16_t = (*state).yuv.maxChannel as uint16_t;
    let rgbMaxChannelF: core::ffi::c_float = (*state).rgb.maxChannelF;
    'c_8828: {
        if alphaMultiplyMode as core::ffi::c_uint
            == AVIF_ALPHA_MULTIPLY_MODE_NO_OP as core::ffi::c_int as core::ffi::c_uint
            || !aPlane.is_null()
        {} else {
            __assert_fail(
                b"(alphaMultiplyMode == AVIF_ALPHA_MULTIPLY_MODE_NO_OP) || aPlane\0"
                    as *const u8 as *const core::ffi::c_char,
                b"/home/runner/work/libavif-rs/libavif-rs/upstream/libavif/src/reformat.c\0"
                    as *const u8 as *const core::ffi::c_char,
                688 as core::ffi::c_uint,
                (::core::mem::transmute::<
                    [u8; 124],
                    [core::ffi::c_char; 124],
                >(
                    *b"avifResult avifImageYUVAnyToRGBAnySlow(const avifImage *, avifRGBImage *, const avifReformatState *, avifAlphaMultiplyMode)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    let mut j: uint32_t = 0 as uint32_t;
    while j < (*image).height {
        let uvJ: uint32_t = if yuvHasColor != 0 {
            j >> (*state).yuv.formatInfo.chromaShiftY
        } else {
            0 as uint32_t
        };
        let mut ptrY8: *const uint8_t = &*yPlane
            .offset((j as size_t).wrapping_mul(yRowBytes) as isize) as *const uint8_t;
        let mut ptrU8: *const uint8_t = if !uPlane.is_null() {
            &*uPlane.offset((uvJ as size_t).wrapping_mul(uRowBytes) as isize)
                as *const uint8_t
        } else {
            0 as *const uint8_t
        };
        let mut ptrV8: *const uint8_t = if !vPlane.is_null() {
            &*vPlane.offset((uvJ as size_t).wrapping_mul(vRowBytes) as isize)
                as *const uint8_t
        } else {
            0 as *const uint8_t
        };
        let mut ptrA8: *const uint8_t = if !aPlane.is_null() {
            &*aPlane.offset((j as size_t).wrapping_mul(aRowBytes) as isize)
                as *const uint8_t
        } else {
            0 as *const uint8_t
        };
        let mut ptrY16: *const uint16_t = ptrY8 as *const uint16_t;
        let mut ptrU16: *const uint16_t = ptrU8 as *const uint16_t;
        let mut ptrV16: *const uint16_t = ptrV8 as *const uint16_t;
        let mut ptrA16: *const uint16_t = ptrA8 as *const uint16_t;
        let mut ptrR: *mut uint8_t = &mut *((*rgb).pixels)
            .offset(
                ((*state).rgb.offsetBytesR as size_t)
                    .wrapping_add((j as size_t).wrapping_mul((*rgb).rowBytes as size_t))
                    as isize,
            ) as *mut uint8_t;
        let mut ptrG: *mut uint8_t = &mut *((*rgb).pixels)
            .offset(
                ((*state).rgb.offsetBytesG as size_t)
                    .wrapping_add((j as size_t).wrapping_mul((*rgb).rowBytes as size_t))
                    as isize,
            ) as *mut uint8_t;
        let mut ptrB: *mut uint8_t = &mut *((*rgb).pixels)
            .offset(
                ((*state).rgb.offsetBytesB as size_t)
                    .wrapping_add((j as size_t).wrapping_mul((*rgb).rowBytes as size_t))
                    as isize,
            ) as *mut uint8_t;
        let mut ptrGray: *mut uint8_t = &mut *((*rgb).pixels)
            .offset(
                ((*state).rgb.offsetBytesGray as size_t)
                    .wrapping_add((j as size_t).wrapping_mul((*rgb).rowBytes as size_t))
                    as isize,
            ) as *mut uint8_t;
        let mut i: uint32_t = 0 as uint32_t;
        while i < (*image).width {
            let mut Y: core::ffi::c_float = 0.;
            let mut Cb: core::ffi::c_float = 0.5f32;
            let mut Cr: core::ffi::c_float = 0.5f32;
            let mut unormY: uint16_t = 0;
            if (*image).depth == 8 as uint32_t {
                unormY = *ptrY8.offset(i as isize) as uint16_t;
            } else {
                unormY = (if (*ptrY16.offset(i as isize) as core::ffi::c_int)
                    < yuvMaxChannel as core::ffi::c_int
                {
                    *ptrY16.offset(i as isize) as core::ffi::c_int
                } else {
                    yuvMaxChannel as core::ffi::c_int
                }) as uint16_t;
            }
            Y = *unormFloatTableY.offset(unormY as isize);
            if yuvHasColor != 0 {
                let uvI: uint32_t = i >> (*state).yuv.formatInfo.chromaShiftX;
                if (*image).yuvFormat as core::ffi::c_uint
                    == AVIF_PIXEL_FORMAT_YUV444 as core::ffi::c_int as core::ffi::c_uint
                {
                    let mut unormU: uint16_t = 0;
                    let mut unormV: uint16_t = 0;
                    if (*image).depth == 8 as uint32_t {
                        unormU = *ptrU8.offset(uvI as isize) as uint16_t;
                        unormV = *ptrV8.offset(uvI as isize) as uint16_t;
                    } else {
                        unormU = (if (*ptrU16.offset(uvI as isize) as core::ffi::c_int)
                            < yuvMaxChannel as core::ffi::c_int
                        {
                            *ptrU16.offset(uvI as isize) as core::ffi::c_int
                        } else {
                            yuvMaxChannel as core::ffi::c_int
                        }) as uint16_t;
                        unormV = (if (*ptrV16.offset(uvI as isize) as core::ffi::c_int)
                            < yuvMaxChannel as core::ffi::c_int
                        {
                            *ptrV16.offset(uvI as isize) as core::ffi::c_int
                        } else {
                            yuvMaxChannel as core::ffi::c_int
                        }) as uint16_t;
                    }
                    Cb = *unormFloatTableUV.offset(unormU as isize);
                    Cr = *unormFloatTableUV.offset(unormV as isize);
                } else {
                    let mut unormU_0: [[uint16_t; 2]; 2] = [[0; 2]; 2];
                    let mut unormV_0: [[uint16_t; 2]; 2] = [[0; 2]; 2];
                    let mut uAdjCol: ptrdiff_t = 0;
                    let mut vAdjCol: ptrdiff_t = 0;
                    let mut uAdjRow: ptrdiff_t = 0;
                    let mut vAdjRow: ptrdiff_t = 0;
                    if i == 0 as uint32_t
                        || i == ((*image).width).wrapping_sub(1 as uint32_t)
                            && i.wrapping_rem(2 as uint32_t) != 0 as uint32_t
                    {
                        uAdjCol = 0 as ptrdiff_t;
                        vAdjCol = 0 as ptrdiff_t;
                    } else if i.wrapping_rem(2 as uint32_t) != 0 as uint32_t {
                        uAdjCol = yuvChannelBytes as ptrdiff_t;
                        vAdjCol = yuvChannelBytes as ptrdiff_t;
                    } else {
                        uAdjCol = -(1 as core::ffi::c_int) as ptrdiff_t
                            * yuvChannelBytes as ptrdiff_t;
                        vAdjCol = -(1 as core::ffi::c_int) as ptrdiff_t
                            * yuvChannelBytes as ptrdiff_t;
                    }
                    if j == 0 as uint32_t
                        || j == ((*image).height).wrapping_sub(1 as uint32_t)
                            && j.wrapping_rem(2 as uint32_t) != 0 as uint32_t
                        || (*image).yuvFormat as core::ffi::c_uint
                            == AVIF_PIXEL_FORMAT_YUV422 as core::ffi::c_int
                                as core::ffi::c_uint
                    {
                        uAdjRow = 0 as ptrdiff_t;
                        vAdjRow = 0 as ptrdiff_t;
                    } else if j.wrapping_rem(2 as uint32_t) != 0 as uint32_t {
                        uAdjRow = uRowBytes as ptrdiff_t;
                        vAdjRow = vRowBytes as ptrdiff_t;
                    } else {
                        uAdjRow = -(1 as core::ffi::c_int) as ptrdiff_t
                            * uRowBytes as ptrdiff_t;
                        vAdjRow = -(1 as core::ffi::c_int) as ptrdiff_t
                            * vRowBytes as ptrdiff_t;
                    }
                    let mut pU: *const uint8_t = &*uPlane
                        .offset(
                            (uvJ as size_t)
                                .wrapping_mul(uRowBytes)
                                .wrapping_add((uvI as size_t).wrapping_mul(yuvChannelBytes))
                                as isize,
                        ) as *const uint8_t;
                    let mut pV: *const uint8_t = &*vPlane
                        .offset(
                            (uvJ as size_t)
                                .wrapping_mul(vRowBytes)
                                .wrapping_add((uvI as size_t).wrapping_mul(yuvChannelBytes))
                                as isize,
                        ) as *const uint8_t;
                    if (*image).depth == 8 as uint32_t {
                        unormU_0[0 as core::ffi::c_int
                            as usize][0 as core::ffi::c_int as usize] = *pU as uint16_t;
                        unormV_0[0 as core::ffi::c_int
                            as usize][0 as core::ffi::c_int as usize] = *pV as uint16_t;
                        unormU_0[1 as core::ffi::c_int
                            as usize][0 as core::ffi::c_int as usize] = *pU
                            .offset(uAdjCol as isize) as uint16_t;
                        unormV_0[1 as core::ffi::c_int
                            as usize][0 as core::ffi::c_int as usize] = *pV
                            .offset(vAdjCol as isize) as uint16_t;
                        unormU_0[0 as core::ffi::c_int
                            as usize][1 as core::ffi::c_int as usize] = *pU
                            .offset(uAdjRow as isize) as uint16_t;
                        unormV_0[0 as core::ffi::c_int
                            as usize][1 as core::ffi::c_int as usize] = *pV
                            .offset(vAdjRow as isize) as uint16_t;
                        unormU_0[1 as core::ffi::c_int
                            as usize][1 as core::ffi::c_int as usize] = *pU
                            .offset(uAdjCol as isize)
                            .offset(uAdjRow as isize) as uint16_t;
                        unormV_0[1 as core::ffi::c_int
                            as usize][1 as core::ffi::c_int as usize] = *pV
                            .offset(vAdjCol as isize)
                            .offset(vAdjRow as isize) as uint16_t;
                    } else {
                        unormU_0[0 as core::ffi::c_int
                            as usize][0 as core::ffi::c_int as usize] = *(pU
                            as *const uint16_t);
                        unormV_0[0 as core::ffi::c_int
                            as usize][0 as core::ffi::c_int as usize] = *(pV
                            as *const uint16_t);
                        unormU_0[1 as core::ffi::c_int
                            as usize][0 as core::ffi::c_int as usize] = *(pU
                            .offset(uAdjCol as isize) as *const uint16_t);
                        unormV_0[1 as core::ffi::c_int
                            as usize][0 as core::ffi::c_int as usize] = *(pV
                            .offset(vAdjCol as isize) as *const uint16_t);
                        unormU_0[0 as core::ffi::c_int
                            as usize][1 as core::ffi::c_int as usize] = *(pU
                            .offset(uAdjRow as isize) as *const uint16_t);
                        unormV_0[0 as core::ffi::c_int
                            as usize][1 as core::ffi::c_int as usize] = *(pV
                            .offset(vAdjRow as isize) as *const uint16_t);
                        unormU_0[1 as core::ffi::c_int
                            as usize][1 as core::ffi::c_int as usize] = *(pU
                            .offset(uAdjCol as isize)
                            .offset(uAdjRow as isize) as *const uint16_t);
                        unormV_0[1 as core::ffi::c_int
                            as usize][1 as core::ffi::c_int as usize] = *(pV
                            .offset(vAdjCol as isize)
                            .offset(vAdjRow as isize) as *const uint16_t);
                        let mut bJ: core::ffi::c_int = 0 as core::ffi::c_int;
                        while bJ < 2 as core::ffi::c_int {
                            let mut bI: core::ffi::c_int = 0 as core::ffi::c_int;
                            while bI < 2 as core::ffi::c_int {
                                unormU_0[bI as usize][bJ as usize] = (if (unormU_0[bI
                                    as usize][bJ as usize] as core::ffi::c_int)
                                    < yuvMaxChannel as core::ffi::c_int
                                {
                                    unormU_0[bI as usize][bJ as usize] as core::ffi::c_int
                                } else {
                                    yuvMaxChannel as core::ffi::c_int
                                }) as uint16_t;
                                unormV_0[bI as usize][bJ as usize] = (if (unormV_0[bI
                                    as usize][bJ as usize] as core::ffi::c_int)
                                    < yuvMaxChannel as core::ffi::c_int
                                {
                                    unormV_0[bI as usize][bJ as usize] as core::ffi::c_int
                                } else {
                                    yuvMaxChannel as core::ffi::c_int
                                }) as uint16_t;
                                bI += 1;
                            }
                            bJ += 1;
                        }
                    }
                    if (*rgb).chromaUpsampling as core::ffi::c_uint
                        == AVIF_CHROMA_UPSAMPLING_FASTEST as core::ffi::c_int
                            as core::ffi::c_uint
                        || (*rgb).chromaUpsampling as core::ffi::c_uint
                            == AVIF_CHROMA_UPSAMPLING_NEAREST as core::ffi::c_int
                                as core::ffi::c_uint
                    {
                        Cb = *unormFloatTableUV
                            .offset(
                                unormU_0[0 as core::ffi::c_int
                                    as usize][0 as core::ffi::c_int as usize] as isize,
                            );
                        Cr = *unormFloatTableUV
                            .offset(
                                unormV_0[0 as core::ffi::c_int
                                    as usize][0 as core::ffi::c_int as usize] as isize,
                            );
                    } else {
                        Cb = *unormFloatTableUV
                            .offset(
                                unormU_0[0 as core::ffi::c_int
                                    as usize][0 as core::ffi::c_int as usize] as isize,
                            ) * (9.0f32 / 16.0f32)
                            + *unormFloatTableUV
                                .offset(
                                    unormU_0[1 as core::ffi::c_int
                                        as usize][0 as core::ffi::c_int as usize] as isize,
                                ) * (3.0f32 / 16.0f32)
                            + *unormFloatTableUV
                                .offset(
                                    unormU_0[0 as core::ffi::c_int
                                        as usize][1 as core::ffi::c_int as usize] as isize,
                                ) * (3.0f32 / 16.0f32)
                            + *unormFloatTableUV
                                .offset(
                                    unormU_0[1 as core::ffi::c_int
                                        as usize][1 as core::ffi::c_int as usize] as isize,
                                ) * (1.0f32 / 16.0f32);
                        Cr = *unormFloatTableUV
                            .offset(
                                unormV_0[0 as core::ffi::c_int
                                    as usize][0 as core::ffi::c_int as usize] as isize,
                            ) * (9.0f32 / 16.0f32)
                            + *unormFloatTableUV
                                .offset(
                                    unormV_0[1 as core::ffi::c_int
                                        as usize][0 as core::ffi::c_int as usize] as isize,
                                ) * (3.0f32 / 16.0f32)
                            + *unormFloatTableUV
                                .offset(
                                    unormV_0[0 as core::ffi::c_int
                                        as usize][1 as core::ffi::c_int as usize] as isize,
                                ) * (3.0f32 / 16.0f32)
                            + *unormFloatTableUV
                                .offset(
                                    unormV_0[1 as core::ffi::c_int
                                        as usize][1 as core::ffi::c_int as usize] as isize,
                                ) * (1.0f32 / 16.0f32);
                    }
                }
            }
            let mut Rc: core::ffi::c_float = 0.0f32;
            let mut Gc: core::ffi::c_float = 0.0f32;
            let mut Bc: core::ffi::c_float = 0.0f32;
            let mut grayc: core::ffi::c_float = 0.0f32;
            if rgbHasColor != 0 {
                let mut R: core::ffi::c_float = 0.;
                let mut G: core::ffi::c_float = 0.;
                let mut B: core::ffi::c_float = 0.;
                if yuvHasColor != 0 {
                    if (*state).yuv.mode as core::ffi::c_uint
                        == AVIF_REFORMAT_MODE_IDENTITY as core::ffi::c_int
                            as core::ffi::c_uint
                    {
                        G = Y;
                        B = Cb;
                        R = Cr;
                    } else if (*state).yuv.mode as core::ffi::c_uint
                        == AVIF_REFORMAT_MODE_YCGCO as core::ffi::c_int
                            as core::ffi::c_uint
                    {
                        let t: core::ffi::c_float = Y - Cb;
                        G = Y + Cb;
                        B = t - Cr;
                        R = t + Cr;
                    } else if (*state).yuv.mode as core::ffi::c_uint
                        == AVIF_REFORMAT_MODE_YCGCO_RE as core::ffi::c_int
                            as core::ffi::c_uint
                        || (*state).yuv.mode as core::ffi::c_uint
                            == AVIF_REFORMAT_MODE_YCGCO_RO as core::ffi::c_int
                                as core::ffi::c_uint
                    {
                        let YY: core::ffi::c_int = unormY as core::ffi::c_int;
                        let Cg: core::ffi::c_int = avifRoundf(
                            Cb * yuvMaxChannel as core::ffi::c_int as core::ffi::c_float,
                        ) as core::ffi::c_int;
                        let Co: core::ffi::c_int = avifRoundf(
                            Cr * yuvMaxChannel as core::ffi::c_int as core::ffi::c_float,
                        ) as core::ffi::c_int;
                        let t_0: core::ffi::c_int = YY - (Cg >> 1 as core::ffi::c_int);
                        G = (if t_0 + Cg < 0 as core::ffi::c_int {
                            0 as core::ffi::c_int
                        } else if (*state).rgb.maxChannel < t_0 + Cg {
                            (*state).rgb.maxChannel
                        } else {
                            t_0 + Cg
                        }) as core::ffi::c_float;
                        B = (if t_0 - (Co >> 1 as core::ffi::c_int)
                            < 0 as core::ffi::c_int
                        {
                            0 as core::ffi::c_int
                        } else if (*state).rgb.maxChannel
                            < t_0 - (Co >> 1 as core::ffi::c_int)
                        {
                            (*state).rgb.maxChannel
                        } else {
                            t_0 - (Co >> 1 as core::ffi::c_int)
                        }) as core::ffi::c_float;
                        R = if (B + Co as core::ffi::c_float)
                            < 0 as core::ffi::c_int as core::ffi::c_float
                        {
                            0 as core::ffi::c_int as core::ffi::c_float
                        } else if ((*state).rgb.maxChannel as core::ffi::c_float)
                            < B + Co as core::ffi::c_float
                        {
                            (*state).rgb.maxChannel as core::ffi::c_float
                        } else {
                            B + Co as core::ffi::c_float
                        };
                        G /= rgbMaxChannelF;
                        B /= rgbMaxChannelF;
                        R /= rgbMaxChannelF;
                    } else {
                        R = Y
                            + 2 as core::ffi::c_int as core::ffi::c_float
                                * (1 as core::ffi::c_int as core::ffi::c_float - kr) * Cr;
                        B = Y
                            + 2 as core::ffi::c_int as core::ffi::c_float
                                * (1 as core::ffi::c_int as core::ffi::c_float - kb) * Cb;
                        G = Y
                            - 2 as core::ffi::c_int as core::ffi::c_float
                                * (kr * (1 as core::ffi::c_int as core::ffi::c_float - kr)
                                    * Cr
                                    + kb * (1 as core::ffi::c_int as core::ffi::c_float - kb)
                                        * Cb) / kg;
                    }
                } else {
                    R = Y;
                    G = Y;
                    B = Y;
                }
                Rc = if R < 0.0f32 { 0.0f32 } else if 1.0f32 < R { 1.0f32 } else { R };
                Gc = if G < 0.0f32 { 0.0f32 } else if 1.0f32 < G { 1.0f32 } else { G };
                Bc = if B < 0.0f32 { 0.0f32 } else if 1.0f32 < B { 1.0f32 } else { B };
            } else {
                let mut gray: core::ffi::c_float = Y;
                grayc = if gray < 0.0f32 {
                    0.0f32
                } else if 1.0f32 < gray {
                    1.0f32
                } else {
                    gray
                };
            }
            if alphaMultiplyMode as core::ffi::c_uint
                != AVIF_ALPHA_MULTIPLY_MODE_NO_OP as core::ffi::c_int
                    as core::ffi::c_uint
            {
                let mut unormA: uint16_t = 0;
                if (*image).depth == 8 as uint32_t {
                    unormA = *ptrA8.offset(i as isize) as uint16_t;
                } else {
                    unormA = (if (*ptrA16.offset(i as isize) as core::ffi::c_int)
                        < yuvMaxChannel as core::ffi::c_int
                    {
                        *ptrA16.offset(i as isize) as core::ffi::c_int
                    } else {
                        yuvMaxChannel as core::ffi::c_int
                    }) as uint16_t;
                }
                let A: core::ffi::c_float = unormA as core::ffi::c_int
                    as core::ffi::c_float
                    / (*state).yuv.maxChannel as core::ffi::c_float;
                let Ac: core::ffi::c_float = if A < 0.0f32 {
                    0.0f32
                } else if 1.0f32 < A {
                    1.0f32
                } else {
                    A
                };
                if alphaMultiplyMode as core::ffi::c_uint
                    == AVIF_ALPHA_MULTIPLY_MODE_MULTIPLY as core::ffi::c_int
                        as core::ffi::c_uint
                {
                    if rgbHasColor != 0 {
                        if Ac == 0.0f32 {
                            Rc = 0.0f32;
                            Gc = 0.0f32;
                            Bc = 0.0f32;
                        } else if Ac < 1.0f32 {
                            Rc *= Ac;
                            Gc *= Ac;
                            Bc *= Ac;
                        }
                    } else if Ac == 0.0f32 {
                        grayc = 0.0f32;
                    } else if Ac < 1.0f32 {
                        grayc *= Ac;
                    }
                } else if rgbHasColor != 0 {
                    if Ac == 0.0f32 {
                        Rc = 0.0f32;
                        Gc = 0.0f32;
                        Bc = 0.0f32;
                    } else if Ac < 1.0f32 {
                        Rc /= Ac;
                        Gc /= Ac;
                        Bc /= Ac;
                        Rc = if Rc < 1.0f32 { Rc } else { 1.0f32 };
                        Gc = if Gc < 1.0f32 { Gc } else { 1.0f32 };
                        Bc = if Bc < 1.0f32 { Bc } else { 1.0f32 };
                    }
                } else if Ac == 0.0f32 {
                    grayc = 0.0f32;
                } else if Ac < 1.0f32 {
                    grayc /= Ac;
                    grayc = if grayc < 1.0f32 { grayc } else { 1.0f32 };
                }
            }
            if rgbHasColor != 0 {
                if (*rgb).depth == 8 as uint32_t {
                    avifStoreRGB8Pixel(
                        (*rgb).format,
                        (0.5f32 + Rc * rgbMaxChannelF) as uint8_t,
                        (0.5f32 + Gc * rgbMaxChannelF) as uint8_t,
                        (0.5f32 + Bc * rgbMaxChannelF) as uint8_t,
                        ptrR,
                        ptrG,
                        ptrB,
                    );
                } else {
                    *(ptrR as *mut uint16_t) = (0.5f32 + Rc * rgbMaxChannelF)
                        as uint16_t;
                    *(ptrG as *mut uint16_t) = (0.5f32 + Gc * rgbMaxChannelF)
                        as uint16_t;
                    *(ptrB as *mut uint16_t) = (0.5f32 + Bc * rgbMaxChannelF)
                        as uint16_t;
                }
                ptrR = ptrR.offset(rgbPixelBytes as isize);
                ptrG = ptrG.offset(rgbPixelBytes as isize);
                ptrB = ptrB.offset(rgbPixelBytes as isize);
            } else {
                if (*rgb).depth == 8 as uint32_t {
                    *ptrGray = (0.5f32 + grayc * rgbMaxChannelF) as uint8_t;
                } else {
                    *(ptrGray as *mut uint16_t) = (0.5f32 + grayc * rgbMaxChannelF)
                        as uint16_t;
                }
                ptrGray = ptrGray.offset(rgbPixelBytes as isize);
            }
            i = i.wrapping_add(1);
        }
        j = j.wrapping_add(1);
    }
    avifFreeYUVToRGBLookUpTables(&mut unormFloatTableY, &mut unormFloatTableUV);
    return AVIF_RESULT_OK;
}
unsafe extern "C" fn avifImageYUV16ToRGB16Color(
    mut image: *const avifImage,
    mut rgb: *mut avifRGBImage,
    mut state: *mut avifReformatState,
) -> avifResult {
    let kr: core::ffi::c_float = (*state).yuv.kr;
    let kg: core::ffi::c_float = (*state).yuv.kg;
    let kb: core::ffi::c_float = (*state).yuv.kb;
    let rgbPixelBytes: uint32_t = (*state).rgb.pixelBytes;
    let mut unormFloatTableY: *mut core::ffi::c_float = 0 as *mut core::ffi::c_float;
    let mut unormFloatTableUV: *mut core::ffi::c_float = 0 as *mut core::ffi::c_float;
    if avifCreateYUVToRGBLookUpTables(
        &mut unormFloatTableY,
        &mut unormFloatTableUV,
        (*image).depth,
        state,
    ) == 0
    {
        return AVIF_RESULT_OUT_OF_MEMORY;
    }
    let yuvMaxChannel: uint16_t = (*state).yuv.maxChannel as uint16_t;
    let rgbMaxChannelF: core::ffi::c_float = (*state).rgb.maxChannelF;
    let mut j: size_t = 0 as size_t;
    while j < (*image).height as size_t {
        let uvJ: size_t = j >> (*state).yuv.formatInfo.chromaShiftY;
        let ptrY: *const uint16_t = &mut *(*((*image).yuvPlanes)
            .as_ptr()
            .offset(AVIF_CHAN_Y as core::ffi::c_int as isize))
            .offset(
                j
                    .wrapping_mul(
                        *((*image).yuvRowBytes)
                            .as_ptr()
                            .offset(AVIF_CHAN_Y as core::ffi::c_int as isize) as size_t,
                    ) as isize,
            ) as *mut uint8_t as *mut uint16_t;
        let ptrU: *const uint16_t = &mut *(*((*image).yuvPlanes)
            .as_ptr()
            .offset(AVIF_CHAN_U as core::ffi::c_int as isize))
            .offset(
                uvJ
                    .wrapping_mul(
                        *((*image).yuvRowBytes)
                            .as_ptr()
                            .offset(AVIF_CHAN_U as core::ffi::c_int as isize) as size_t,
                    ) as isize,
            ) as *mut uint8_t as *mut uint16_t;
        let ptrV: *const uint16_t = &mut *(*((*image).yuvPlanes)
            .as_ptr()
            .offset(AVIF_CHAN_V as core::ffi::c_int as isize))
            .offset(
                uvJ
                    .wrapping_mul(
                        *((*image).yuvRowBytes)
                            .as_ptr()
                            .offset(AVIF_CHAN_V as core::ffi::c_int as isize) as size_t,
                    ) as isize,
            ) as *mut uint8_t as *mut uint16_t;
        let mut ptrR: *mut uint8_t = &mut *((*rgb).pixels)
            .offset(
                ((*state).rgb.offsetBytesR as size_t)
                    .wrapping_add(j.wrapping_mul((*rgb).rowBytes as size_t)) as isize,
            ) as *mut uint8_t;
        let mut ptrG: *mut uint8_t = &mut *((*rgb).pixels)
            .offset(
                ((*state).rgb.offsetBytesG as size_t)
                    .wrapping_add(j.wrapping_mul((*rgb).rowBytes as size_t)) as isize,
            ) as *mut uint8_t;
        let mut ptrB: *mut uint8_t = &mut *((*rgb).pixels)
            .offset(
                ((*state).rgb.offsetBytesB as size_t)
                    .wrapping_add(j.wrapping_mul((*rgb).rowBytes as size_t)) as isize,
            ) as *mut uint8_t;
        let mut i: size_t = 0 as size_t;
        while i < (*image).width as size_t {
            let mut uvI: size_t = i >> (*state).yuv.formatInfo.chromaShiftX;
            let unormY: uint16_t = (if (*ptrY.offset(i as isize) as core::ffi::c_int)
                < yuvMaxChannel as core::ffi::c_int
            {
                *ptrY.offset(i as isize) as core::ffi::c_int
            } else {
                yuvMaxChannel as core::ffi::c_int
            }) as uint16_t;
            let unormU: uint16_t = (if (*ptrU.offset(uvI as isize) as core::ffi::c_int)
                < yuvMaxChannel as core::ffi::c_int
            {
                *ptrU.offset(uvI as isize) as core::ffi::c_int
            } else {
                yuvMaxChannel as core::ffi::c_int
            }) as uint16_t;
            let unormV: uint16_t = (if (*ptrV.offset(uvI as isize) as core::ffi::c_int)
                < yuvMaxChannel as core::ffi::c_int
            {
                *ptrV.offset(uvI as isize) as core::ffi::c_int
            } else {
                yuvMaxChannel as core::ffi::c_int
            }) as uint16_t;
            let Y: core::ffi::c_float = *unormFloatTableY.offset(unormY as isize);
            let Cb: core::ffi::c_float = *unormFloatTableUV.offset(unormU as isize);
            let Cr: core::ffi::c_float = *unormFloatTableUV.offset(unormV as isize);
            let R: core::ffi::c_float = Y
                + 2 as core::ffi::c_int as core::ffi::c_float
                    * (1 as core::ffi::c_int as core::ffi::c_float - kr) * Cr;
            let B: core::ffi::c_float = Y
                + 2 as core::ffi::c_int as core::ffi::c_float
                    * (1 as core::ffi::c_int as core::ffi::c_float - kb) * Cb;
            let G: core::ffi::c_float = Y
                - 2 as core::ffi::c_int as core::ffi::c_float
                    * (kr * (1 as core::ffi::c_int as core::ffi::c_float - kr) * Cr
                        + kb * (1 as core::ffi::c_int as core::ffi::c_float - kb) * Cb)
                    / kg;
            let Rc: core::ffi::c_float = if R < 0.0f32 {
                0.0f32
            } else if 1.0f32 < R {
                1.0f32
            } else {
                R
            };
            let Gc: core::ffi::c_float = if G < 0.0f32 {
                0.0f32
            } else if 1.0f32 < G {
                1.0f32
            } else {
                G
            };
            let Bc: core::ffi::c_float = if B < 0.0f32 {
                0.0f32
            } else if 1.0f32 < B {
                1.0f32
            } else {
                B
            };
            *(ptrR as *mut uint16_t) = (0.5f32 + Rc * rgbMaxChannelF) as uint16_t;
            *(ptrG as *mut uint16_t) = (0.5f32 + Gc * rgbMaxChannelF) as uint16_t;
            *(ptrB as *mut uint16_t) = (0.5f32 + Bc * rgbMaxChannelF) as uint16_t;
            ptrR = ptrR.offset(rgbPixelBytes as isize);
            ptrG = ptrG.offset(rgbPixelBytes as isize);
            ptrB = ptrB.offset(rgbPixelBytes as isize);
            i = i.wrapping_add(1);
        }
        j = j.wrapping_add(1);
    }
    avifFreeYUVToRGBLookUpTables(&mut unormFloatTableY, &mut unormFloatTableUV);
    return AVIF_RESULT_OK;
}
unsafe extern "C" fn avifImageYUV16ToRGB16Mono(
    mut image: *const avifImage,
    mut rgb: *mut avifRGBImage,
    mut state: *mut avifReformatState,
) -> avifResult {
    let kr: core::ffi::c_float = (*state).yuv.kr;
    let kg: core::ffi::c_float = (*state).yuv.kg;
    let kb: core::ffi::c_float = (*state).yuv.kb;
    let rgbPixelBytes: uint32_t = (*state).rgb.pixelBytes;
    let mut unormFloatTableY: *mut core::ffi::c_float = 0 as *mut core::ffi::c_float;
    if avifCreateYUVToRGBLookUpTables(
        &mut unormFloatTableY,
        0 as *mut *mut core::ffi::c_float,
        (*image).depth,
        state,
    ) == 0
    {
        return AVIF_RESULT_OUT_OF_MEMORY;
    }
    let maxChannel: uint16_t = (*state).yuv.maxChannel as uint16_t;
    let maxChannelF: core::ffi::c_float = (*state).rgb.maxChannelF;
    let mut j: size_t = 0 as size_t;
    while j < (*image).height as size_t {
        let ptrY: *const uint16_t = &mut *(*((*image).yuvPlanes)
            .as_ptr()
            .offset(AVIF_CHAN_Y as core::ffi::c_int as isize))
            .offset(
                j
                    .wrapping_mul(
                        *((*image).yuvRowBytes)
                            .as_ptr()
                            .offset(AVIF_CHAN_Y as core::ffi::c_int as isize) as size_t,
                    ) as isize,
            ) as *mut uint8_t as *mut uint16_t;
        let mut ptrR: *mut uint8_t = &mut *((*rgb).pixels)
            .offset(
                ((*state).rgb.offsetBytesR as size_t)
                    .wrapping_add(j.wrapping_mul((*rgb).rowBytes as size_t)) as isize,
            ) as *mut uint8_t;
        let mut ptrG: *mut uint8_t = &mut *((*rgb).pixels)
            .offset(
                ((*state).rgb.offsetBytesG as size_t)
                    .wrapping_add(j.wrapping_mul((*rgb).rowBytes as size_t)) as isize,
            ) as *mut uint8_t;
        let mut ptrB: *mut uint8_t = &mut *((*rgb).pixels)
            .offset(
                ((*state).rgb.offsetBytesB as size_t)
                    .wrapping_add(j.wrapping_mul((*rgb).rowBytes as size_t)) as isize,
            ) as *mut uint8_t;
        let mut i: size_t = 0 as size_t;
        while i < (*image).width as size_t {
            let unormY: uint16_t = (if (*ptrY.offset(i as isize) as core::ffi::c_int)
                < maxChannel as core::ffi::c_int
            {
                *ptrY.offset(i as isize) as core::ffi::c_int
            } else {
                maxChannel as core::ffi::c_int
            }) as uint16_t;
            let Y: core::ffi::c_float = *unormFloatTableY.offset(unormY as isize);
            let Cb: core::ffi::c_float = 0.0f32;
            let Cr: core::ffi::c_float = 0.0f32;
            let R: core::ffi::c_float = Y
                + 2 as core::ffi::c_int as core::ffi::c_float
                    * (1 as core::ffi::c_int as core::ffi::c_float - kr) * Cr;
            let B: core::ffi::c_float = Y
                + 2 as core::ffi::c_int as core::ffi::c_float
                    * (1 as core::ffi::c_int as core::ffi::c_float - kb) * Cb;
            let G: core::ffi::c_float = Y
                - 2 as core::ffi::c_int as core::ffi::c_float
                    * (kr * (1 as core::ffi::c_int as core::ffi::c_float - kr) * Cr
                        + kb * (1 as core::ffi::c_int as core::ffi::c_float - kb) * Cb)
                    / kg;
            let Rc: core::ffi::c_float = if R < 0.0f32 {
                0.0f32
            } else if 1.0f32 < R {
                1.0f32
            } else {
                R
            };
            let Gc: core::ffi::c_float = if G < 0.0f32 {
                0.0f32
            } else if 1.0f32 < G {
                1.0f32
            } else {
                G
            };
            let Bc: core::ffi::c_float = if B < 0.0f32 {
                0.0f32
            } else if 1.0f32 < B {
                1.0f32
            } else {
                B
            };
            *(ptrR as *mut uint16_t) = (0.5f32 + Rc * maxChannelF) as uint16_t;
            *(ptrG as *mut uint16_t) = (0.5f32 + Gc * maxChannelF) as uint16_t;
            *(ptrB as *mut uint16_t) = (0.5f32 + Bc * maxChannelF) as uint16_t;
            ptrR = ptrR.offset(rgbPixelBytes as isize);
            ptrG = ptrG.offset(rgbPixelBytes as isize);
            ptrB = ptrB.offset(rgbPixelBytes as isize);
            i = i.wrapping_add(1);
        }
        j = j.wrapping_add(1);
    }
    avifFreeYUVToRGBLookUpTables(
        &mut unormFloatTableY,
        0 as *mut *mut core::ffi::c_float,
    );
    return AVIF_RESULT_OK;
}
unsafe extern "C" fn avifImageYUV16ToRGB8Color(
    mut image: *const avifImage,
    mut rgb: *mut avifRGBImage,
    mut state: *mut avifReformatState,
) -> avifResult {
    let kr: core::ffi::c_float = (*state).yuv.kr;
    let kg: core::ffi::c_float = (*state).yuv.kg;
    let kb: core::ffi::c_float = (*state).yuv.kb;
    let rgbPixelBytes: uint32_t = (*state).rgb.pixelBytes;
    let mut unormFloatTableY: *mut core::ffi::c_float = 0 as *mut core::ffi::c_float;
    let mut unormFloatTableUV: *mut core::ffi::c_float = 0 as *mut core::ffi::c_float;
    if avifCreateYUVToRGBLookUpTables(
        &mut unormFloatTableY,
        &mut unormFloatTableUV,
        (*image).depth,
        state,
    ) == 0
    {
        return AVIF_RESULT_OUT_OF_MEMORY;
    }
    let yuvMaxChannel: uint16_t = (*state).yuv.maxChannel as uint16_t;
    let rgbMaxChannelF: core::ffi::c_float = (*state).rgb.maxChannelF;
    let mut j: size_t = 0 as size_t;
    while j < (*image).height as size_t {
        let uvJ: size_t = j >> (*state).yuv.formatInfo.chromaShiftY;
        let ptrY: *const uint16_t = &mut *(*((*image).yuvPlanes)
            .as_ptr()
            .offset(AVIF_CHAN_Y as core::ffi::c_int as isize))
            .offset(
                j
                    .wrapping_mul(
                        *((*image).yuvRowBytes)
                            .as_ptr()
                            .offset(AVIF_CHAN_Y as core::ffi::c_int as isize) as size_t,
                    ) as isize,
            ) as *mut uint8_t as *mut uint16_t;
        let ptrU: *const uint16_t = &mut *(*((*image).yuvPlanes)
            .as_ptr()
            .offset(AVIF_CHAN_U as core::ffi::c_int as isize))
            .offset(
                uvJ
                    .wrapping_mul(
                        *((*image).yuvRowBytes)
                            .as_ptr()
                            .offset(AVIF_CHAN_U as core::ffi::c_int as isize) as size_t,
                    ) as isize,
            ) as *mut uint8_t as *mut uint16_t;
        let ptrV: *const uint16_t = &mut *(*((*image).yuvPlanes)
            .as_ptr()
            .offset(AVIF_CHAN_V as core::ffi::c_int as isize))
            .offset(
                uvJ
                    .wrapping_mul(
                        *((*image).yuvRowBytes)
                            .as_ptr()
                            .offset(AVIF_CHAN_V as core::ffi::c_int as isize) as size_t,
                    ) as isize,
            ) as *mut uint8_t as *mut uint16_t;
        let mut ptrR: *mut uint8_t = &mut *((*rgb).pixels)
            .offset(
                ((*state).rgb.offsetBytesR as size_t)
                    .wrapping_add(j.wrapping_mul((*rgb).rowBytes as size_t)) as isize,
            ) as *mut uint8_t;
        let mut ptrG: *mut uint8_t = &mut *((*rgb).pixels)
            .offset(
                ((*state).rgb.offsetBytesG as size_t)
                    .wrapping_add(j.wrapping_mul((*rgb).rowBytes as size_t)) as isize,
            ) as *mut uint8_t;
        let mut ptrB: *mut uint8_t = &mut *((*rgb).pixels)
            .offset(
                ((*state).rgb.offsetBytesB as size_t)
                    .wrapping_add(j.wrapping_mul((*rgb).rowBytes as size_t)) as isize,
            ) as *mut uint8_t;
        let mut i: size_t = 0 as size_t;
        while i < (*image).width as size_t {
            let mut uvI: size_t = i >> (*state).yuv.formatInfo.chromaShiftX;
            let unormY: uint16_t = (if (*ptrY.offset(i as isize) as core::ffi::c_int)
                < yuvMaxChannel as core::ffi::c_int
            {
                *ptrY.offset(i as isize) as core::ffi::c_int
            } else {
                yuvMaxChannel as core::ffi::c_int
            }) as uint16_t;
            let unormU: uint16_t = (if (*ptrU.offset(uvI as isize) as core::ffi::c_int)
                < yuvMaxChannel as core::ffi::c_int
            {
                *ptrU.offset(uvI as isize) as core::ffi::c_int
            } else {
                yuvMaxChannel as core::ffi::c_int
            }) as uint16_t;
            let unormV: uint16_t = (if (*ptrV.offset(uvI as isize) as core::ffi::c_int)
                < yuvMaxChannel as core::ffi::c_int
            {
                *ptrV.offset(uvI as isize) as core::ffi::c_int
            } else {
                yuvMaxChannel as core::ffi::c_int
            }) as uint16_t;
            let Y: core::ffi::c_float = *unormFloatTableY.offset(unormY as isize);
            let Cb: core::ffi::c_float = *unormFloatTableUV.offset(unormU as isize);
            let Cr: core::ffi::c_float = *unormFloatTableUV.offset(unormV as isize);
            let R: core::ffi::c_float = Y
                + 2 as core::ffi::c_int as core::ffi::c_float
                    * (1 as core::ffi::c_int as core::ffi::c_float - kr) * Cr;
            let B: core::ffi::c_float = Y
                + 2 as core::ffi::c_int as core::ffi::c_float
                    * (1 as core::ffi::c_int as core::ffi::c_float - kb) * Cb;
            let G: core::ffi::c_float = Y
                - 2 as core::ffi::c_int as core::ffi::c_float
                    * (kr * (1 as core::ffi::c_int as core::ffi::c_float - kr) * Cr
                        + kb * (1 as core::ffi::c_int as core::ffi::c_float - kb) * Cb)
                    / kg;
            let Rc: core::ffi::c_float = if R < 0.0f32 {
                0.0f32
            } else if 1.0f32 < R {
                1.0f32
            } else {
                R
            };
            let Gc: core::ffi::c_float = if G < 0.0f32 {
                0.0f32
            } else if 1.0f32 < G {
                1.0f32
            } else {
                G
            };
            let Bc: core::ffi::c_float = if B < 0.0f32 {
                0.0f32
            } else if 1.0f32 < B {
                1.0f32
            } else {
                B
            };
            avifStoreRGB8Pixel(
                (*rgb).format,
                (0.5f32 + Rc * rgbMaxChannelF) as uint8_t,
                (0.5f32 + Gc * rgbMaxChannelF) as uint8_t,
                (0.5f32 + Bc * rgbMaxChannelF) as uint8_t,
                ptrR,
                ptrG,
                ptrB,
            );
            ptrR = ptrR.offset(rgbPixelBytes as isize);
            ptrG = ptrG.offset(rgbPixelBytes as isize);
            ptrB = ptrB.offset(rgbPixelBytes as isize);
            i = i.wrapping_add(1);
        }
        j = j.wrapping_add(1);
    }
    avifFreeYUVToRGBLookUpTables(&mut unormFloatTableY, &mut unormFloatTableUV);
    return AVIF_RESULT_OK;
}
unsafe extern "C" fn avifImageYUV16ToRGB8Mono(
    mut image: *const avifImage,
    mut rgb: *mut avifRGBImage,
    mut state: *mut avifReformatState,
) -> avifResult {
    let kr: core::ffi::c_float = (*state).yuv.kr;
    let kg: core::ffi::c_float = (*state).yuv.kg;
    let kb: core::ffi::c_float = (*state).yuv.kb;
    let rgbPixelBytes: uint32_t = (*state).rgb.pixelBytes;
    let mut unormFloatTableY: *mut core::ffi::c_float = 0 as *mut core::ffi::c_float;
    if avifCreateYUVToRGBLookUpTables(
        &mut unormFloatTableY,
        0 as *mut *mut core::ffi::c_float,
        (*image).depth,
        state,
    ) == 0
    {
        return AVIF_RESULT_OUT_OF_MEMORY;
    }
    let yuvMaxChannel: uint16_t = (*state).yuv.maxChannel as uint16_t;
    let rgbMaxChannelF: core::ffi::c_float = (*state).rgb.maxChannelF;
    let mut j: size_t = 0 as size_t;
    while j < (*image).height as size_t {
        let ptrY: *const uint16_t = &mut *(*((*image).yuvPlanes)
            .as_ptr()
            .offset(AVIF_CHAN_Y as core::ffi::c_int as isize))
            .offset(
                j
                    .wrapping_mul(
                        *((*image).yuvRowBytes)
                            .as_ptr()
                            .offset(AVIF_CHAN_Y as core::ffi::c_int as isize) as size_t,
                    ) as isize,
            ) as *mut uint8_t as *mut uint16_t;
        let mut ptrR: *mut uint8_t = &mut *((*rgb).pixels)
            .offset(
                ((*state).rgb.offsetBytesR as size_t)
                    .wrapping_add(j.wrapping_mul((*rgb).rowBytes as size_t)) as isize,
            ) as *mut uint8_t;
        let mut ptrG: *mut uint8_t = &mut *((*rgb).pixels)
            .offset(
                ((*state).rgb.offsetBytesG as size_t)
                    .wrapping_add(j.wrapping_mul((*rgb).rowBytes as size_t)) as isize,
            ) as *mut uint8_t;
        let mut ptrB: *mut uint8_t = &mut *((*rgb).pixels)
            .offset(
                ((*state).rgb.offsetBytesB as size_t)
                    .wrapping_add(j.wrapping_mul((*rgb).rowBytes as size_t)) as isize,
            ) as *mut uint8_t;
        let mut i: size_t = 0 as size_t;
        while i < (*image).width as size_t {
            let unormY: uint16_t = (if (*ptrY.offset(i as isize) as core::ffi::c_int)
                < yuvMaxChannel as core::ffi::c_int
            {
                *ptrY.offset(i as isize) as core::ffi::c_int
            } else {
                yuvMaxChannel as core::ffi::c_int
            }) as uint16_t;
            let Y: core::ffi::c_float = *unormFloatTableY.offset(unormY as isize);
            let Cb: core::ffi::c_float = 0.0f32;
            let Cr: core::ffi::c_float = 0.0f32;
            let R: core::ffi::c_float = Y
                + 2 as core::ffi::c_int as core::ffi::c_float
                    * (1 as core::ffi::c_int as core::ffi::c_float - kr) * Cr;
            let B: core::ffi::c_float = Y
                + 2 as core::ffi::c_int as core::ffi::c_float
                    * (1 as core::ffi::c_int as core::ffi::c_float - kb) * Cb;
            let G: core::ffi::c_float = Y
                - 2 as core::ffi::c_int as core::ffi::c_float
                    * (kr * (1 as core::ffi::c_int as core::ffi::c_float - kr) * Cr
                        + kb * (1 as core::ffi::c_int as core::ffi::c_float - kb) * Cb)
                    / kg;
            let Rc: core::ffi::c_float = if R < 0.0f32 {
                0.0f32
            } else if 1.0f32 < R {
                1.0f32
            } else {
                R
            };
            let Gc: core::ffi::c_float = if G < 0.0f32 {
                0.0f32
            } else if 1.0f32 < G {
                1.0f32
            } else {
                G
            };
            let Bc: core::ffi::c_float = if B < 0.0f32 {
                0.0f32
            } else if 1.0f32 < B {
                1.0f32
            } else {
                B
            };
            avifStoreRGB8Pixel(
                (*rgb).format,
                (0.5f32 + Rc * rgbMaxChannelF) as uint8_t,
                (0.5f32 + Gc * rgbMaxChannelF) as uint8_t,
                (0.5f32 + Bc * rgbMaxChannelF) as uint8_t,
                ptrR,
                ptrG,
                ptrB,
            );
            ptrR = ptrR.offset(rgbPixelBytes as isize);
            ptrG = ptrG.offset(rgbPixelBytes as isize);
            ptrB = ptrB.offset(rgbPixelBytes as isize);
            i = i.wrapping_add(1);
        }
        j = j.wrapping_add(1);
    }
    avifFreeYUVToRGBLookUpTables(
        &mut unormFloatTableY,
        0 as *mut *mut core::ffi::c_float,
    );
    return AVIF_RESULT_OK;
}
unsafe extern "C" fn avifImageYUV8ToRGB16Color(
    mut image: *const avifImage,
    mut rgb: *mut avifRGBImage,
    mut state: *mut avifReformatState,
) -> avifResult {
    let kr: core::ffi::c_float = (*state).yuv.kr;
    let kg: core::ffi::c_float = (*state).yuv.kg;
    let kb: core::ffi::c_float = (*state).yuv.kb;
    let rgbPixelBytes: uint32_t = (*state).rgb.pixelBytes;
    let mut unormFloatTableY: *mut core::ffi::c_float = 0 as *mut core::ffi::c_float;
    let mut unormFloatTableUV: *mut core::ffi::c_float = 0 as *mut core::ffi::c_float;
    if avifCreateYUVToRGBLookUpTables(
        &mut unormFloatTableY,
        &mut unormFloatTableUV,
        (*image).depth,
        state,
    ) == 0
    {
        return AVIF_RESULT_OUT_OF_MEMORY;
    }
    let rgbMaxChannelF: core::ffi::c_float = (*state).rgb.maxChannelF;
    let mut j: size_t = 0 as size_t;
    while j < (*image).height as size_t {
        let uvJ: size_t = j >> (*state).yuv.formatInfo.chromaShiftY;
        let ptrY: *const uint8_t = &mut *(*((*image).yuvPlanes)
            .as_ptr()
            .offset(AVIF_CHAN_Y as core::ffi::c_int as isize))
            .offset(
                j
                    .wrapping_mul(
                        *((*image).yuvRowBytes)
                            .as_ptr()
                            .offset(AVIF_CHAN_Y as core::ffi::c_int as isize) as size_t,
                    ) as isize,
            ) as *mut uint8_t;
        let ptrU: *const uint8_t = &mut *(*((*image).yuvPlanes)
            .as_ptr()
            .offset(AVIF_CHAN_U as core::ffi::c_int as isize))
            .offset(
                uvJ
                    .wrapping_mul(
                        *((*image).yuvRowBytes)
                            .as_ptr()
                            .offset(AVIF_CHAN_U as core::ffi::c_int as isize) as size_t,
                    ) as isize,
            ) as *mut uint8_t;
        let ptrV: *const uint8_t = &mut *(*((*image).yuvPlanes)
            .as_ptr()
            .offset(AVIF_CHAN_V as core::ffi::c_int as isize))
            .offset(
                uvJ
                    .wrapping_mul(
                        *((*image).yuvRowBytes)
                            .as_ptr()
                            .offset(AVIF_CHAN_V as core::ffi::c_int as isize) as size_t,
                    ) as isize,
            ) as *mut uint8_t;
        let mut ptrR: *mut uint8_t = &mut *((*rgb).pixels)
            .offset(
                ((*state).rgb.offsetBytesR as size_t)
                    .wrapping_add(j.wrapping_mul((*rgb).rowBytes as size_t)) as isize,
            ) as *mut uint8_t;
        let mut ptrG: *mut uint8_t = &mut *((*rgb).pixels)
            .offset(
                ((*state).rgb.offsetBytesG as size_t)
                    .wrapping_add(j.wrapping_mul((*rgb).rowBytes as size_t)) as isize,
            ) as *mut uint8_t;
        let mut ptrB: *mut uint8_t = &mut *((*rgb).pixels)
            .offset(
                ((*state).rgb.offsetBytesB as size_t)
                    .wrapping_add(j.wrapping_mul((*rgb).rowBytes as size_t)) as isize,
            ) as *mut uint8_t;
        let mut i: size_t = 0 as size_t;
        while i < (*image).width as size_t {
            let mut uvI: size_t = i >> (*state).yuv.formatInfo.chromaShiftX;
            let Y: core::ffi::c_float = *unormFloatTableY
                .offset(*ptrY.offset(i as isize) as isize);
            let Cb: core::ffi::c_float = *unormFloatTableUV
                .offset(*ptrU.offset(uvI as isize) as isize);
            let Cr: core::ffi::c_float = *unormFloatTableUV
                .offset(*ptrV.offset(uvI as isize) as isize);
            let R: core::ffi::c_float = Y
                + 2 as core::ffi::c_int as core::ffi::c_float
                    * (1 as core::ffi::c_int as core::ffi::c_float - kr) * Cr;
            let B: core::ffi::c_float = Y
                + 2 as core::ffi::c_int as core::ffi::c_float
                    * (1 as core::ffi::c_int as core::ffi::c_float - kb) * Cb;
            let G: core::ffi::c_float = Y
                - 2 as core::ffi::c_int as core::ffi::c_float
                    * (kr * (1 as core::ffi::c_int as core::ffi::c_float - kr) * Cr
                        + kb * (1 as core::ffi::c_int as core::ffi::c_float - kb) * Cb)
                    / kg;
            let Rc: core::ffi::c_float = if R < 0.0f32 {
                0.0f32
            } else if 1.0f32 < R {
                1.0f32
            } else {
                R
            };
            let Gc: core::ffi::c_float = if G < 0.0f32 {
                0.0f32
            } else if 1.0f32 < G {
                1.0f32
            } else {
                G
            };
            let Bc: core::ffi::c_float = if B < 0.0f32 {
                0.0f32
            } else if 1.0f32 < B {
                1.0f32
            } else {
                B
            };
            *(ptrR as *mut uint16_t) = (0.5f32 + Rc * rgbMaxChannelF) as uint16_t;
            *(ptrG as *mut uint16_t) = (0.5f32 + Gc * rgbMaxChannelF) as uint16_t;
            *(ptrB as *mut uint16_t) = (0.5f32 + Bc * rgbMaxChannelF) as uint16_t;
            ptrR = ptrR.offset(rgbPixelBytes as isize);
            ptrG = ptrG.offset(rgbPixelBytes as isize);
            ptrB = ptrB.offset(rgbPixelBytes as isize);
            i = i.wrapping_add(1);
        }
        j = j.wrapping_add(1);
    }
    avifFreeYUVToRGBLookUpTables(&mut unormFloatTableY, &mut unormFloatTableUV);
    return AVIF_RESULT_OK;
}
unsafe extern "C" fn avifImageYUV8ToRGB16Mono(
    mut image: *const avifImage,
    mut rgb: *mut avifRGBImage,
    mut state: *mut avifReformatState,
) -> avifResult {
    let kr: core::ffi::c_float = (*state).yuv.kr;
    let kg: core::ffi::c_float = (*state).yuv.kg;
    let kb: core::ffi::c_float = (*state).yuv.kb;
    let rgbPixelBytes: uint32_t = (*state).rgb.pixelBytes;
    let mut unormFloatTableY: *mut core::ffi::c_float = 0 as *mut core::ffi::c_float;
    if avifCreateYUVToRGBLookUpTables(
        &mut unormFloatTableY,
        0 as *mut *mut core::ffi::c_float,
        (*image).depth,
        state,
    ) == 0
    {
        return AVIF_RESULT_OUT_OF_MEMORY;
    }
    let rgbMaxChannelF: core::ffi::c_float = (*state).rgb.maxChannelF;
    let mut j: size_t = 0 as size_t;
    while j < (*image).height as size_t {
        let ptrY: *const uint8_t = &mut *(*((*image).yuvPlanes)
            .as_ptr()
            .offset(AVIF_CHAN_Y as core::ffi::c_int as isize))
            .offset(
                j
                    .wrapping_mul(
                        *((*image).yuvRowBytes)
                            .as_ptr()
                            .offset(AVIF_CHAN_Y as core::ffi::c_int as isize) as size_t,
                    ) as isize,
            ) as *mut uint8_t;
        let mut ptrR: *mut uint8_t = &mut *((*rgb).pixels)
            .offset(
                ((*state).rgb.offsetBytesR as size_t)
                    .wrapping_add(j.wrapping_mul((*rgb).rowBytes as size_t)) as isize,
            ) as *mut uint8_t;
        let mut ptrG: *mut uint8_t = &mut *((*rgb).pixels)
            .offset(
                ((*state).rgb.offsetBytesG as size_t)
                    .wrapping_add(j.wrapping_mul((*rgb).rowBytes as size_t)) as isize,
            ) as *mut uint8_t;
        let mut ptrB: *mut uint8_t = &mut *((*rgb).pixels)
            .offset(
                ((*state).rgb.offsetBytesB as size_t)
                    .wrapping_add(j.wrapping_mul((*rgb).rowBytes as size_t)) as isize,
            ) as *mut uint8_t;
        let mut i: size_t = 0 as size_t;
        while i < (*image).width as size_t {
            let Y: core::ffi::c_float = *unormFloatTableY
                .offset(*ptrY.offset(i as isize) as isize);
            let Cb: core::ffi::c_float = 0.0f32;
            let Cr: core::ffi::c_float = 0.0f32;
            let R: core::ffi::c_float = Y
                + 2 as core::ffi::c_int as core::ffi::c_float
                    * (1 as core::ffi::c_int as core::ffi::c_float - kr) * Cr;
            let B: core::ffi::c_float = Y
                + 2 as core::ffi::c_int as core::ffi::c_float
                    * (1 as core::ffi::c_int as core::ffi::c_float - kb) * Cb;
            let G: core::ffi::c_float = Y
                - 2 as core::ffi::c_int as core::ffi::c_float
                    * (kr * (1 as core::ffi::c_int as core::ffi::c_float - kr) * Cr
                        + kb * (1 as core::ffi::c_int as core::ffi::c_float - kb) * Cb)
                    / kg;
            let Rc: core::ffi::c_float = if R < 0.0f32 {
                0.0f32
            } else if 1.0f32 < R {
                1.0f32
            } else {
                R
            };
            let Gc: core::ffi::c_float = if G < 0.0f32 {
                0.0f32
            } else if 1.0f32 < G {
                1.0f32
            } else {
                G
            };
            let Bc: core::ffi::c_float = if B < 0.0f32 {
                0.0f32
            } else if 1.0f32 < B {
                1.0f32
            } else {
                B
            };
            *(ptrR as *mut uint16_t) = (0.5f32 + Rc * rgbMaxChannelF) as uint16_t;
            *(ptrG as *mut uint16_t) = (0.5f32 + Gc * rgbMaxChannelF) as uint16_t;
            *(ptrB as *mut uint16_t) = (0.5f32 + Bc * rgbMaxChannelF) as uint16_t;
            ptrR = ptrR.offset(rgbPixelBytes as isize);
            ptrG = ptrG.offset(rgbPixelBytes as isize);
            ptrB = ptrB.offset(rgbPixelBytes as isize);
            i = i.wrapping_add(1);
        }
        j = j.wrapping_add(1);
    }
    avifFreeYUVToRGBLookUpTables(
        &mut unormFloatTableY,
        0 as *mut *mut core::ffi::c_float,
    );
    return AVIF_RESULT_OK;
}
unsafe extern "C" fn avifImageIdentity8ToRGB8ColorFullRange(
    mut image: *const avifImage,
    mut rgb: *mut avifRGBImage,
    mut state: *mut avifReformatState,
) -> avifResult {
    let rgbPixelBytes: uint32_t = (*state).rgb.pixelBytes;
    let mut j: size_t = 0 as size_t;
    while j < (*image).height as size_t {
        let ptrY: *const uint8_t = &mut *(*((*image).yuvPlanes)
            .as_ptr()
            .offset(AVIF_CHAN_Y as core::ffi::c_int as isize))
            .offset(
                j
                    .wrapping_mul(
                        *((*image).yuvRowBytes)
                            .as_ptr()
                            .offset(AVIF_CHAN_Y as core::ffi::c_int as isize) as size_t,
                    ) as isize,
            ) as *mut uint8_t;
        let ptrU: *const uint8_t = &mut *(*((*image).yuvPlanes)
            .as_ptr()
            .offset(AVIF_CHAN_U as core::ffi::c_int as isize))
            .offset(
                j
                    .wrapping_mul(
                        *((*image).yuvRowBytes)
                            .as_ptr()
                            .offset(AVIF_CHAN_U as core::ffi::c_int as isize) as size_t,
                    ) as isize,
            ) as *mut uint8_t;
        let ptrV: *const uint8_t = &mut *(*((*image).yuvPlanes)
            .as_ptr()
            .offset(AVIF_CHAN_V as core::ffi::c_int as isize))
            .offset(
                j
                    .wrapping_mul(
                        *((*image).yuvRowBytes)
                            .as_ptr()
                            .offset(AVIF_CHAN_V as core::ffi::c_int as isize) as size_t,
                    ) as isize,
            ) as *mut uint8_t;
        let mut ptrR: *mut uint8_t = &mut *((*rgb).pixels)
            .offset(
                ((*state).rgb.offsetBytesR as size_t)
                    .wrapping_add(j.wrapping_mul((*rgb).rowBytes as size_t)) as isize,
            ) as *mut uint8_t;
        let mut ptrG: *mut uint8_t = &mut *((*rgb).pixels)
            .offset(
                ((*state).rgb.offsetBytesG as size_t)
                    .wrapping_add(j.wrapping_mul((*rgb).rowBytes as size_t)) as isize,
            ) as *mut uint8_t;
        let mut ptrB: *mut uint8_t = &mut *((*rgb).pixels)
            .offset(
                ((*state).rgb.offsetBytesB as size_t)
                    .wrapping_add(j.wrapping_mul((*rgb).rowBytes as size_t)) as isize,
            ) as *mut uint8_t;
        if (*rgb).format as core::ffi::c_uint
            == AVIF_RGB_FORMAT_RGB_565 as core::ffi::c_int as core::ffi::c_uint
        {
            let mut i: size_t = 0 as size_t;
            while i < (*image).width as size_t {
                *(ptrR as *mut uint16_t) = (*ptrU.offset(i as isize) as core::ffi::c_int
                    >> 3 as core::ffi::c_int
                    | (*ptrY.offset(i as isize) as core::ffi::c_int
                        >> 2 as core::ffi::c_int) << 5 as core::ffi::c_int
                    | (*ptrV.offset(i as isize) as core::ffi::c_int
                        >> 3 as core::ffi::c_int) << 11 as core::ffi::c_int) as uint16_t;
                ptrR = ptrR.offset(rgbPixelBytes as isize);
                i = i.wrapping_add(1);
            }
        } else {
            let mut i_0: size_t = 0 as size_t;
            while i_0 < (*image).width as size_t {
                *ptrR = *ptrV.offset(i_0 as isize);
                *ptrG = *ptrY.offset(i_0 as isize);
                *ptrB = *ptrU.offset(i_0 as isize);
                ptrR = ptrR.offset(rgbPixelBytes as isize);
                ptrG = ptrG.offset(rgbPixelBytes as isize);
                ptrB = ptrB.offset(rgbPixelBytes as isize);
                i_0 = i_0.wrapping_add(1);
            }
        }
        j = j.wrapping_add(1);
    }
    return AVIF_RESULT_OK;
}
unsafe extern "C" fn avifImageYUV8ToRGB8Color(
    mut image: *const avifImage,
    mut rgb: *mut avifRGBImage,
    mut state: *mut avifReformatState,
) -> avifResult {
    let kr: core::ffi::c_float = (*state).yuv.kr;
    let kg: core::ffi::c_float = (*state).yuv.kg;
    let kb: core::ffi::c_float = (*state).yuv.kb;
    let rgbPixelBytes: uint32_t = (*state).rgb.pixelBytes;
    let mut unormFloatTableY: *mut core::ffi::c_float = 0 as *mut core::ffi::c_float;
    let mut unormFloatTableUV: *mut core::ffi::c_float = 0 as *mut core::ffi::c_float;
    if avifCreateYUVToRGBLookUpTables(
        &mut unormFloatTableY,
        &mut unormFloatTableUV,
        (*image).depth,
        state,
    ) == 0
    {
        return AVIF_RESULT_OUT_OF_MEMORY;
    }
    let rgbMaxChannelF: core::ffi::c_float = (*state).rgb.maxChannelF;
    let mut j: size_t = 0 as size_t;
    while j < (*image).height as size_t {
        let uvJ: size_t = j >> (*state).yuv.formatInfo.chromaShiftY;
        let ptrY: *const uint8_t = &mut *(*((*image).yuvPlanes)
            .as_ptr()
            .offset(AVIF_CHAN_Y as core::ffi::c_int as isize))
            .offset(
                j
                    .wrapping_mul(
                        *((*image).yuvRowBytes)
                            .as_ptr()
                            .offset(AVIF_CHAN_Y as core::ffi::c_int as isize) as size_t,
                    ) as isize,
            ) as *mut uint8_t;
        let ptrU: *const uint8_t = &mut *(*((*image).yuvPlanes)
            .as_ptr()
            .offset(AVIF_CHAN_U as core::ffi::c_int as isize))
            .offset(
                uvJ
                    .wrapping_mul(
                        *((*image).yuvRowBytes)
                            .as_ptr()
                            .offset(AVIF_CHAN_U as core::ffi::c_int as isize) as size_t,
                    ) as isize,
            ) as *mut uint8_t;
        let ptrV: *const uint8_t = &mut *(*((*image).yuvPlanes)
            .as_ptr()
            .offset(AVIF_CHAN_V as core::ffi::c_int as isize))
            .offset(
                uvJ
                    .wrapping_mul(
                        *((*image).yuvRowBytes)
                            .as_ptr()
                            .offset(AVIF_CHAN_V as core::ffi::c_int as isize) as size_t,
                    ) as isize,
            ) as *mut uint8_t;
        let mut ptrR: *mut uint8_t = &mut *((*rgb).pixels)
            .offset(
                ((*state).rgb.offsetBytesR as size_t)
                    .wrapping_add(j.wrapping_mul((*rgb).rowBytes as size_t)) as isize,
            ) as *mut uint8_t;
        let mut ptrG: *mut uint8_t = &mut *((*rgb).pixels)
            .offset(
                ((*state).rgb.offsetBytesG as size_t)
                    .wrapping_add(j.wrapping_mul((*rgb).rowBytes as size_t)) as isize,
            ) as *mut uint8_t;
        let mut ptrB: *mut uint8_t = &mut *((*rgb).pixels)
            .offset(
                ((*state).rgb.offsetBytesB as size_t)
                    .wrapping_add(j.wrapping_mul((*rgb).rowBytes as size_t)) as isize,
            ) as *mut uint8_t;
        let mut i: size_t = 0 as size_t;
        while i < (*image).width as size_t {
            let mut uvI: size_t = i >> (*state).yuv.formatInfo.chromaShiftX;
            let Y: core::ffi::c_float = *unormFloatTableY
                .offset(*ptrY.offset(i as isize) as isize);
            let Cb: core::ffi::c_float = *unormFloatTableUV
                .offset(*ptrU.offset(uvI as isize) as isize);
            let Cr: core::ffi::c_float = *unormFloatTableUV
                .offset(*ptrV.offset(uvI as isize) as isize);
            let R: core::ffi::c_float = Y
                + 2 as core::ffi::c_int as core::ffi::c_float
                    * (1 as core::ffi::c_int as core::ffi::c_float - kr) * Cr;
            let B: core::ffi::c_float = Y
                + 2 as core::ffi::c_int as core::ffi::c_float
                    * (1 as core::ffi::c_int as core::ffi::c_float - kb) * Cb;
            let G: core::ffi::c_float = Y
                - 2 as core::ffi::c_int as core::ffi::c_float
                    * (kr * (1 as core::ffi::c_int as core::ffi::c_float - kr) * Cr
                        + kb * (1 as core::ffi::c_int as core::ffi::c_float - kb) * Cb)
                    / kg;
            let Rc: core::ffi::c_float = if R < 0.0f32 {
                0.0f32
            } else if 1.0f32 < R {
                1.0f32
            } else {
                R
            };
            let Gc: core::ffi::c_float = if G < 0.0f32 {
                0.0f32
            } else if 1.0f32 < G {
                1.0f32
            } else {
                G
            };
            let Bc: core::ffi::c_float = if B < 0.0f32 {
                0.0f32
            } else if 1.0f32 < B {
                1.0f32
            } else {
                B
            };
            avifStoreRGB8Pixel(
                (*rgb).format,
                (0.5f32 + Rc * rgbMaxChannelF) as uint8_t,
                (0.5f32 + Gc * rgbMaxChannelF) as uint8_t,
                (0.5f32 + Bc * rgbMaxChannelF) as uint8_t,
                ptrR,
                ptrG,
                ptrB,
            );
            ptrR = ptrR.offset(rgbPixelBytes as isize);
            ptrG = ptrG.offset(rgbPixelBytes as isize);
            ptrB = ptrB.offset(rgbPixelBytes as isize);
            i = i.wrapping_add(1);
        }
        j = j.wrapping_add(1);
    }
    avifFreeYUVToRGBLookUpTables(&mut unormFloatTableY, &mut unormFloatTableUV);
    return AVIF_RESULT_OK;
}
unsafe extern "C" fn avifImageYUV8ToRGB8Mono(
    mut image: *const avifImage,
    mut rgb: *mut avifRGBImage,
    mut state: *mut avifReformatState,
) -> avifResult {
    let kr: core::ffi::c_float = (*state).yuv.kr;
    let kg: core::ffi::c_float = (*state).yuv.kg;
    let kb: core::ffi::c_float = (*state).yuv.kb;
    let rgbPixelBytes: uint32_t = (*state).rgb.pixelBytes;
    let mut unormFloatTableY: *mut core::ffi::c_float = 0 as *mut core::ffi::c_float;
    if avifCreateYUVToRGBLookUpTables(
        &mut unormFloatTableY,
        0 as *mut *mut core::ffi::c_float,
        (*image).depth,
        state,
    ) == 0
    {
        return AVIF_RESULT_OUT_OF_MEMORY;
    }
    let rgbMaxChannelF: core::ffi::c_float = (*state).rgb.maxChannelF;
    let mut j: size_t = 0 as size_t;
    while j < (*image).height as size_t {
        let ptrY: *const uint8_t = &mut *(*((*image).yuvPlanes)
            .as_ptr()
            .offset(AVIF_CHAN_Y as core::ffi::c_int as isize))
            .offset(
                j
                    .wrapping_mul(
                        *((*image).yuvRowBytes)
                            .as_ptr()
                            .offset(AVIF_CHAN_Y as core::ffi::c_int as isize) as size_t,
                    ) as isize,
            ) as *mut uint8_t;
        let mut ptrR: *mut uint8_t = &mut *((*rgb).pixels)
            .offset(
                ((*state).rgb.offsetBytesR as size_t)
                    .wrapping_add(j.wrapping_mul((*rgb).rowBytes as size_t)) as isize,
            ) as *mut uint8_t;
        let mut ptrG: *mut uint8_t = &mut *((*rgb).pixels)
            .offset(
                ((*state).rgb.offsetBytesG as size_t)
                    .wrapping_add(j.wrapping_mul((*rgb).rowBytes as size_t)) as isize,
            ) as *mut uint8_t;
        let mut ptrB: *mut uint8_t = &mut *((*rgb).pixels)
            .offset(
                ((*state).rgb.offsetBytesB as size_t)
                    .wrapping_add(j.wrapping_mul((*rgb).rowBytes as size_t)) as isize,
            ) as *mut uint8_t;
        let mut i: size_t = 0 as size_t;
        while i < (*image).width as size_t {
            let Y: core::ffi::c_float = *unormFloatTableY
                .offset(*ptrY.offset(i as isize) as isize);
            let Cb: core::ffi::c_float = 0.0f32;
            let Cr: core::ffi::c_float = 0.0f32;
            let R: core::ffi::c_float = Y
                + 2 as core::ffi::c_int as core::ffi::c_float
                    * (1 as core::ffi::c_int as core::ffi::c_float - kr) * Cr;
            let B: core::ffi::c_float = Y
                + 2 as core::ffi::c_int as core::ffi::c_float
                    * (1 as core::ffi::c_int as core::ffi::c_float - kb) * Cb;
            let G: core::ffi::c_float = Y
                - 2 as core::ffi::c_int as core::ffi::c_float
                    * (kr * (1 as core::ffi::c_int as core::ffi::c_float - kr) * Cr
                        + kb * (1 as core::ffi::c_int as core::ffi::c_float - kb) * Cb)
                    / kg;
            let Rc: core::ffi::c_float = if R < 0.0f32 {
                0.0f32
            } else if 1.0f32 < R {
                1.0f32
            } else {
                R
            };
            let Gc: core::ffi::c_float = if G < 0.0f32 {
                0.0f32
            } else if 1.0f32 < G {
                1.0f32
            } else {
                G
            };
            let Bc: core::ffi::c_float = if B < 0.0f32 {
                0.0f32
            } else if 1.0f32 < B {
                1.0f32
            } else {
                B
            };
            avifStoreRGB8Pixel(
                (*rgb).format,
                (0.5f32 + Rc * rgbMaxChannelF) as uint8_t,
                (0.5f32 + Gc * rgbMaxChannelF) as uint8_t,
                (0.5f32 + Bc * rgbMaxChannelF) as uint8_t,
                ptrR,
                ptrG,
                ptrB,
            );
            ptrR = ptrR.offset(rgbPixelBytes as isize);
            ptrG = ptrG.offset(rgbPixelBytes as isize);
            ptrB = ptrB.offset(rgbPixelBytes as isize);
            i = i.wrapping_add(1);
        }
        j = j.wrapping_add(1);
    }
    avifFreeYUVToRGBLookUpTables(
        &mut unormFloatTableY,
        0 as *mut *mut core::ffi::c_float,
    );
    return AVIF_RESULT_OK;
}
pub const F16_MULTIPLIER: core::ffi::c_float = 1.9259299444e-34f32;
unsafe extern "C" fn avifRGBImageToF16(mut rgb: *mut avifRGBImage) -> avifResult {
    let mut libyuvResult: avifResult = AVIF_RESULT_NOT_IMPLEMENTED;
    if (*rgb).avoidLibYUV == 0 {
        libyuvResult = avifRGBImageToF16LibYUV(rgb);
    }
    if libyuvResult as core::ffi::c_uint
        != AVIF_RESULT_NOT_IMPLEMENTED as core::ffi::c_int as core::ffi::c_uint
    {
        return libyuvResult;
    }
    let channelCount: size_t = avifRGBFormatChannelCount((*rgb).format) as size_t;
    let scale: core::ffi::c_float = 1.0f32
        / (((1 as core::ffi::c_int) << (*rgb).depth) - 1 as core::ffi::c_int)
            as core::ffi::c_float;
    let multiplier: core::ffi::c_float = F16_MULTIPLIER * scale;
    let mut pixelRowBase: *mut uint16_t = (*rgb).pixels as *mut uint16_t;
    let stride: uint32_t = (*rgb).rowBytes >> 1 as core::ffi::c_int;
    let mut j: size_t = 0 as size_t;
    while j < (*rgb).height as size_t {
        let mut pixel: *mut uint16_t = pixelRowBase;
        let mut i: size_t = 0 as size_t;
        while i < ((*rgb).width as size_t).wrapping_mul(channelCount) {
            let mut f16: avifF16 = avifF16 { f: 0. };
            f16.f = *pixel as core::ffi::c_int as core::ffi::c_float * multiplier;
            *pixel = (f16.u32_0 >> 13 as core::ffi::c_int) as uint16_t;
            i = i.wrapping_add(1);
            pixel = pixel.offset(1);
        }
        pixelRowBase = pixelRowBase.offset(stride as isize);
        j = j.wrapping_add(1);
    }
    return AVIF_RESULT_OK;
}
unsafe extern "C" fn avifImageYUVToRGBImpl(
    mut image: *const avifImage,
    mut rgb: *mut avifRGBImage,
    mut state: *mut avifReformatState,
    mut alphaMultiplyMode: avifAlphaMultiplyMode,
) -> avifResult {
    let mut convertedWithLibYUV: avifBool = AVIF_FALSE;
    let mut reformatAlpha: avifBool = (avifRGBFormatHasAlpha((*rgb).format) != 0
        && ((*rgb).ignoreAlpha == 0
            || alphaMultiplyMode as core::ffi::c_uint
                != AVIF_ALPHA_MULTIPLY_MODE_NO_OP as core::ffi::c_int
                    as core::ffi::c_uint)) as core::ffi::c_int;
    let mut alphaReformattedWithLibYUV: avifBool = AVIF_FALSE;
    if (*rgb).avoidLibYUV == 0
        && (alphaMultiplyMode as core::ffi::c_uint
            == AVIF_ALPHA_MULTIPLY_MODE_NO_OP as core::ffi::c_int as core::ffi::c_uint
            || avifRGBFormatHasAlpha((*rgb).format) != 0)
    {
        let mut libyuvResult: avifResult = avifImageYUVToRGBLibYUV(
            image,
            rgb,
            reformatAlpha,
            &mut alphaReformattedWithLibYUV,
        );
        if libyuvResult as core::ffi::c_uint
            == AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
        {
            convertedWithLibYUV = AVIF_TRUE as avifBool;
        } else if libyuvResult as core::ffi::c_uint
            != AVIF_RESULT_NOT_IMPLEMENTED as core::ffi::c_int as core::ffi::c_uint
        {
            return libyuvResult
        }
    }
    if reformatAlpha != 0 && alphaReformattedWithLibYUV == 0 {
        let mut params: avifAlphaParams = avifAlphaParams {
            width: 0,
            height: 0,
            srcDepth: 0,
            srcPlane: 0 as *const uint8_t,
            srcRowBytes: 0,
            srcOffsetBytes: 0,
            srcPixelBytes: 0,
            dstDepth: 0,
            dstPlane: 0 as *mut uint8_t,
            dstRowBytes: 0,
            dstOffsetBytes: 0,
            dstPixelBytes: 0,
        };
        params.width = (*rgb).width;
        params.height = (*rgb).height;
        params.dstDepth = (*rgb).depth;
        params.dstPlane = (*rgb).pixels;
        params.dstRowBytes = (*rgb).rowBytes;
        params.dstOffsetBytes = (*state).rgb.offsetBytesA;
        params.dstPixelBytes = (*state).rgb.pixelBytes;
        if !((*image).alphaPlane).is_null() && (*image).alphaRowBytes != 0 {
            params.srcDepth = (*image).depth;
            params.srcPlane = (*image).alphaPlane;
            params.srcRowBytes = (*image).alphaRowBytes;
            params.srcOffsetBytes = 0 as uint32_t;
            params.srcPixelBytes = (*state).yuv.channelBytes;
            avifReformatAlpha(&mut params);
        } else {
            avifFillAlpha(&mut params);
        }
    }
    if convertedWithLibYUV == 0 {
        let mut convertResult: avifResult = AVIF_RESULT_NOT_IMPLEMENTED;
        let hasColor: avifBool = ((*image)
            .yuvRowBytes[AVIF_CHAN_U as core::ffi::c_int as usize] != 0
            && (*image).yuvRowBytes[AVIF_CHAN_V as core::ffi::c_int as usize] != 0
            && (*image).yuvFormat as core::ffi::c_uint
                != AVIF_PIXEL_FORMAT_YUV400 as core::ffi::c_int as core::ffi::c_uint)
            as core::ffi::c_int;
        if avifRGBFormatIsGray((*rgb).format) == 0
            && (hasColor == 0
                || (*image).yuvFormat as core::ffi::c_uint
                    == AVIF_PIXEL_FORMAT_YUV444 as core::ffi::c_int as core::ffi::c_uint
                || ((*rgb).chromaUpsampling as core::ffi::c_uint
                    == AVIF_CHROMA_UPSAMPLING_FASTEST as core::ffi::c_int
                        as core::ffi::c_uint
                    || (*rgb).chromaUpsampling as core::ffi::c_uint
                        == AVIF_CHROMA_UPSAMPLING_NEAREST as core::ffi::c_int
                            as core::ffi::c_uint))
            && (alphaMultiplyMode as core::ffi::c_uint
                == AVIF_ALPHA_MULTIPLY_MODE_NO_OP as core::ffi::c_int
                    as core::ffi::c_uint || avifRGBFormatHasAlpha((*rgb).format) != 0)
        {
            if (*state).yuv.mode as core::ffi::c_uint
                == AVIF_REFORMAT_MODE_IDENTITY as core::ffi::c_int as core::ffi::c_uint
            {
                if (*image).depth == 8 as uint32_t && (*rgb).depth == 8 as uint32_t
                    && (*image).yuvFormat as core::ffi::c_uint
                        == AVIF_PIXEL_FORMAT_YUV444 as core::ffi::c_int
                            as core::ffi::c_uint
                    && (*image).yuvRange as core::ffi::c_uint
                        == AVIF_RANGE_FULL as core::ffi::c_int as core::ffi::c_uint
                {
                    convertResult = avifImageIdentity8ToRGB8ColorFullRange(
                        image,
                        rgb,
                        state,
                    );
                }
            } else if (*state).yuv.mode as core::ffi::c_uint
                == AVIF_REFORMAT_MODE_YUV_COEFFICIENTS as core::ffi::c_int
                    as core::ffi::c_uint
            {
                if (*image).depth > 8 as uint32_t {
                    if (*rgb).depth > 8 as uint32_t {
                        if hasColor != 0 {
                            convertResult = avifImageYUV16ToRGB16Color(
                                image,
                                rgb,
                                state,
                            );
                        } else {
                            convertResult = avifImageYUV16ToRGB16Mono(image, rgb, state);
                        }
                    } else if hasColor != 0 {
                        convertResult = avifImageYUV16ToRGB8Color(image, rgb, state);
                    } else {
                        convertResult = avifImageYUV16ToRGB8Mono(image, rgb, state);
                    }
                } else if (*rgb).depth > 8 as uint32_t {
                    if hasColor != 0 {
                        convertResult = avifImageYUV8ToRGB16Color(image, rgb, state);
                    } else {
                        convertResult = avifImageYUV8ToRGB16Mono(image, rgb, state);
                    }
                } else if hasColor != 0 {
                    convertResult = avifImageYUV8ToRGB8Color(image, rgb, state);
                } else {
                    convertResult = avifImageYUV8ToRGB8Mono(image, rgb, state);
                }
            }
        }
        if convertResult as core::ffi::c_uint
            == AVIF_RESULT_NOT_IMPLEMENTED as core::ffi::c_int as core::ffi::c_uint
        {
            convertResult = avifImageYUVAnyToRGBAnySlow(
                image,
                rgb,
                state,
                alphaMultiplyMode,
            );
            alphaMultiplyMode = AVIF_ALPHA_MULTIPLY_MODE_NO_OP;
        }
        if convertResult as core::ffi::c_uint
            != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
        {
            return convertResult;
        }
    }
    if alphaMultiplyMode as core::ffi::c_uint
        == AVIF_ALPHA_MULTIPLY_MODE_MULTIPLY as core::ffi::c_int as core::ffi::c_uint
    {
        let mut result: avifResult = avifRGBImagePremultiplyAlpha(rgb);
        if result as core::ffi::c_uint
            != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
        {
            return result;
        }
    } else if alphaMultiplyMode as core::ffi::c_uint
        == AVIF_ALPHA_MULTIPLY_MODE_UNMULTIPLY as core::ffi::c_int as core::ffi::c_uint
    {
        let mut result_0: avifResult = avifRGBImageUnpremultiplyAlpha(rgb);
        if result_0 as core::ffi::c_uint
            != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
        {
            return result_0;
        }
    }
    if (*rgb).isFloat != 0 {
        return avifRGBImageToF16(rgb);
    }
    return AVIF_RESULT_OK;
}
unsafe extern "C" fn avifImageYUVToRGBThreadWorker(
    mut arg: *mut core::ffi::c_void,
) -> *mut core::ffi::c_void {
    let mut data: *mut YUVToRGBThreadData = arg as *mut YUVToRGBThreadData;
    (*data).result = avifImageYUVToRGBImpl(
        &mut (*data).image,
        &mut (*data).rgb,
        (*data).state,
        (*data).alphaMultiplyMode,
    );
    return NULL;
}
unsafe extern "C" fn avifCreateYUVToRGBThread(
    mut tdata: *mut YUVToRGBThreadData,
) -> avifBool {
    return (pthread_create(
        &mut (*tdata).thread,
        0 as *const pthread_attr_t,
        Some(
            avifImageYUVToRGBThreadWorker
                as unsafe extern "C" fn(*mut core::ffi::c_void) -> *mut core::ffi::c_void,
        ),
        tdata as *mut core::ffi::c_void,
    ) == 0 as core::ffi::c_int) as core::ffi::c_int;
}
unsafe extern "C" fn avifJoinYUVToRGBThread(
    mut tdata: *mut YUVToRGBThreadData,
) -> avifBool {
    return (pthread_join((*tdata).thread, 0 as *mut *mut core::ffi::c_void)
        == 0 as core::ffi::c_int) as core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn avifImageYUVToRGB(
    mut image: *const avifImage,
    mut rgb: *mut avifRGBImage,
) -> avifResult {
    if ((*image).yuvPlanes[AVIF_CHAN_Y as core::ffi::c_int as usize]).is_null()
        || (*rgb).maxThreads < 0 as core::ffi::c_int
    {
        return AVIF_RESULT_REFORMAT_FAILED;
    }
    let mut state: avifReformatState = avifReformatState {
        rgb: avifRGBColorSpaceInfo {
            channelBytes: 0,
            pixelBytes: 0,
            offsetBytesR: 0,
            offsetBytesG: 0,
            offsetBytesB: 0,
            offsetBytesA: 0,
            offsetBytesGray: 0,
            maxChannel: 0,
            maxChannelF: 0.,
        },
        yuv: avifYUVColorSpaceInfo {
            kr: 0.,
            kg: 0.,
            kb: 0.,
            channelBytes: 0,
            depth: 0,
            range: AVIF_RANGE_LIMITED,
            maxChannel: 0,
            biasY: 0.,
            biasUV: 0.,
            rangeY: 0.,
            rangeUV: 0.,
            formatInfo: avifPixelFormatInfo {
                monochrome: 0,
                chromaShiftX: 0,
                chromaShiftY: 0,
            },
            mode: AVIF_REFORMAT_MODE_YUV_COEFFICIENTS,
        },
    };
    if avifPrepareReformatState(image, rgb, &mut state) == 0 {
        return AVIF_RESULT_REFORMAT_FAILED;
    }
    let mut alphaMultiplyMode: avifAlphaMultiplyMode = AVIF_ALPHA_MULTIPLY_MODE_NO_OP;
    if !((*image).alphaPlane).is_null() {
        if avifRGBFormatHasAlpha((*rgb).format) == 0 || (*rgb).ignoreAlpha != 0 {
            if (*image).alphaPremultiplied == 0 {
                alphaMultiplyMode = AVIF_ALPHA_MULTIPLY_MODE_MULTIPLY;
            }
        } else if (*image).alphaPremultiplied == 0 && (*rgb).alphaPremultiplied != 0 {
            alphaMultiplyMode = AVIF_ALPHA_MULTIPLY_MODE_MULTIPLY;
        } else if (*image).alphaPremultiplied != 0 && (*rgb).alphaPremultiplied == 0 {
            alphaMultiplyMode = AVIF_ALPHA_MULTIPLY_MODE_UNMULTIPLY;
        }
    }
    let mut jobs: uint32_t = (if (*rgb).maxThreads < 1 as core::ffi::c_int {
        1 as core::ffi::c_int
    } else if (8 as core::ffi::c_int) < (*rgb).maxThreads {
        8 as core::ffi::c_int
    } else {
        (*rgb).maxThreads
    }) as uint32_t;
    if (*image).yuvFormat as core::ffi::c_uint
        == AVIF_PIXEL_FORMAT_YUV420 as core::ffi::c_int as core::ffi::c_uint
        && ((*rgb).chromaUpsampling as core::ffi::c_uint
            == AVIF_CHROMA_UPSAMPLING_AUTOMATIC as core::ffi::c_int as core::ffi::c_uint
            || (*rgb).chromaUpsampling as core::ffi::c_uint
                == AVIF_CHROMA_UPSAMPLING_BEST_QUALITY as core::ffi::c_int
                    as core::ffi::c_uint
            || (*rgb).chromaUpsampling as core::ffi::c_uint
                == AVIF_CHROMA_UPSAMPLING_BILINEAR as core::ffi::c_int
                    as core::ffi::c_uint)
    {
        jobs = 1 as uint32_t;
    }
    if jobs == 1 as uint32_t || ((*image).height).wrapping_div(2 as uint32_t) < jobs {
        return avifImageYUVToRGBImpl(image, rgb, &mut state, alphaMultiplyMode);
    }
    let byteCount: size_t = (::core::mem::size_of::<YUVToRGBThreadData>() as size_t)
        .wrapping_mul(jobs as size_t);
    let mut threadData: *mut YUVToRGBThreadData = avifAlloc(byteCount)
        as *mut YUVToRGBThreadData;
    if threadData.is_null() {
        return AVIF_RESULT_OUT_OF_MEMORY;
    }
    memset(threadData as *mut core::ffi::c_void, 0 as core::ffi::c_int, byteCount);
    let mut rowsPerJob: uint32_t = ((*image).height).wrapping_div(jobs);
    if rowsPerJob.wrapping_rem(2 as uint32_t) != 0 {
        rowsPerJob = rowsPerJob.wrapping_add(1);
        jobs = ((*image).height)
            .wrapping_add(rowsPerJob)
            .wrapping_sub(1 as uint32_t)
            .wrapping_div(rowsPerJob);
    }
    let rowsForLastJob: uint32_t = ((*image).height)
        .wrapping_sub(rowsPerJob.wrapping_mul(jobs.wrapping_sub(1 as uint32_t)));
    let mut startRow: uint32_t = 0 as uint32_t;
    let mut i: uint32_t = 0;
    i = 0 as uint32_t;
    while i < jobs {
        let mut tdata: *mut YUVToRGBThreadData = &mut *threadData.offset(i as isize)
            as *mut YUVToRGBThreadData;
        let rect: avifCropRect = {
            let mut init = avifCropRect {
                x: 0 as uint32_t,
                y: startRow,
                width: (*image).width,
                height: if i == jobs.wrapping_sub(1 as uint32_t) {
                    rowsForLastJob
                } else {
                    rowsPerJob
                },
            };
            init
        };
        if avifImageSetViewRect(&mut (*tdata).image, image, &rect) as core::ffi::c_uint
            != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
        {
            (*tdata).result = AVIF_RESULT_REFORMAT_FAILED;
            break;
        } else {
            (*tdata).rgb = *rgb;
            (*tdata).rgb.pixels = ((*tdata).rgb.pixels)
                .offset(
                    (startRow as size_t).wrapping_mul((*rgb).rowBytes as size_t) as isize,
                );
            (*tdata).rgb.height = (*tdata).image.height;
            (*tdata).state = &mut state;
            (*tdata).alphaMultiplyMode = alphaMultiplyMode;
            if i > 0 as uint32_t {
                (*tdata).threadCreated = avifCreateYUVToRGBThread(tdata);
                if (*tdata).threadCreated == 0 {
                    (*tdata).result = AVIF_RESULT_REFORMAT_FAILED;
                    break;
                }
            }
            i = i.wrapping_add(1);
            startRow = startRow.wrapping_add(rowsPerJob);
        }
    }
    if i == jobs {
        avifImageYUVToRGBThreadWorker(
            &mut *threadData.offset(0 as core::ffi::c_int as isize)
                as *mut YUVToRGBThreadData as *mut core::ffi::c_void,
        );
    }
    let mut result: avifResult = AVIF_RESULT_OK;
    i = 0 as uint32_t;
    while i < jobs {
        let mut tdata_0: *mut YUVToRGBThreadData = &mut *threadData.offset(i as isize)
            as *mut YUVToRGBThreadData;
        if (*tdata_0).threadCreated != 0 && avifJoinYUVToRGBThread(tdata_0) == 0 {
            result = AVIF_RESULT_REFORMAT_FAILED;
        }
        if (*tdata_0).result as core::ffi::c_uint
            != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
        {
            result = (*tdata_0).result;
        }
        i = i.wrapping_add(1);
    }
    avifFree(threadData as *mut core::ffi::c_void);
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn avifLimitedToFullY(
    mut depth: uint32_t,
    mut v: core::ffi::c_int,
) -> core::ffi::c_int {
    match depth {
        8 => {
            v = ((v - 16 as core::ffi::c_int) * 255 as core::ffi::c_int
                + (235 as core::ffi::c_int - 16 as core::ffi::c_int)
                    / 2 as core::ffi::c_int)
                / (235 as core::ffi::c_int - 16 as core::ffi::c_int);
            v = if v < 0 as core::ffi::c_int {
                0 as core::ffi::c_int
            } else if (255 as core::ffi::c_int) < v {
                255 as core::ffi::c_int
            } else {
                v
            };
        }
        10 => {
            v = ((v - 64 as core::ffi::c_int) * 1023 as core::ffi::c_int
                + (940 as core::ffi::c_int - 64 as core::ffi::c_int)
                    / 2 as core::ffi::c_int)
                / (940 as core::ffi::c_int - 64 as core::ffi::c_int);
            v = if v < 0 as core::ffi::c_int {
                0 as core::ffi::c_int
            } else if (1023 as core::ffi::c_int) < v {
                1023 as core::ffi::c_int
            } else {
                v
            };
        }
        12 => {
            v = ((v - 256 as core::ffi::c_int) * 4095 as core::ffi::c_int
                + (3760 as core::ffi::c_int - 256 as core::ffi::c_int)
                    / 2 as core::ffi::c_int)
                / (3760 as core::ffi::c_int - 256 as core::ffi::c_int);
            v = if v < 0 as core::ffi::c_int {
                0 as core::ffi::c_int
            } else if (4095 as core::ffi::c_int) < v {
                4095 as core::ffi::c_int
            } else {
                v
            };
        }
        _ => {}
    }
    return v;
}
#[no_mangle]
pub unsafe extern "C" fn avifLimitedToFullUV(
    mut depth: uint32_t,
    mut v: core::ffi::c_int,
) -> core::ffi::c_int {
    match depth {
        8 => {
            v = ((v - 16 as core::ffi::c_int) * 255 as core::ffi::c_int
                + (240 as core::ffi::c_int - 16 as core::ffi::c_int)
                    / 2 as core::ffi::c_int)
                / (240 as core::ffi::c_int - 16 as core::ffi::c_int);
            v = if v < 0 as core::ffi::c_int {
                0 as core::ffi::c_int
            } else if (255 as core::ffi::c_int) < v {
                255 as core::ffi::c_int
            } else {
                v
            };
        }
        10 => {
            v = ((v - 64 as core::ffi::c_int) * 1023 as core::ffi::c_int
                + (960 as core::ffi::c_int - 64 as core::ffi::c_int)
                    / 2 as core::ffi::c_int)
                / (960 as core::ffi::c_int - 64 as core::ffi::c_int);
            v = if v < 0 as core::ffi::c_int {
                0 as core::ffi::c_int
            } else if (1023 as core::ffi::c_int) < v {
                1023 as core::ffi::c_int
            } else {
                v
            };
        }
        12 => {
            v = ((v - 256 as core::ffi::c_int) * 4095 as core::ffi::c_int
                + (3840 as core::ffi::c_int - 256 as core::ffi::c_int)
                    / 2 as core::ffi::c_int)
                / (3840 as core::ffi::c_int - 256 as core::ffi::c_int);
            v = if v < 0 as core::ffi::c_int {
                0 as core::ffi::c_int
            } else if (4095 as core::ffi::c_int) < v {
                4095 as core::ffi::c_int
            } else {
                v
            };
        }
        _ => {}
    }
    return v;
}
#[no_mangle]
pub unsafe extern "C" fn avifFullToLimitedY(
    mut depth: uint32_t,
    mut v: core::ffi::c_int,
) -> core::ffi::c_int {
    match depth {
        8 => {
            v = (v * (235 as core::ffi::c_int - 16 as core::ffi::c_int)
                + 255 as core::ffi::c_int / 2 as core::ffi::c_int)
                / 255 as core::ffi::c_int + 16 as core::ffi::c_int;
            v = if v < 16 as core::ffi::c_int {
                16 as core::ffi::c_int
            } else if (235 as core::ffi::c_int) < v {
                235 as core::ffi::c_int
            } else {
                v
            };
        }
        10 => {
            v = (v * (940 as core::ffi::c_int - 64 as core::ffi::c_int)
                + 1023 as core::ffi::c_int / 2 as core::ffi::c_int)
                / 1023 as core::ffi::c_int + 64 as core::ffi::c_int;
            v = if v < 64 as core::ffi::c_int {
                64 as core::ffi::c_int
            } else if (940 as core::ffi::c_int) < v {
                940 as core::ffi::c_int
            } else {
                v
            };
        }
        12 => {
            v = (v * (3760 as core::ffi::c_int - 256 as core::ffi::c_int)
                + 4095 as core::ffi::c_int / 2 as core::ffi::c_int)
                / 4095 as core::ffi::c_int + 256 as core::ffi::c_int;
            v = if v < 256 as core::ffi::c_int {
                256 as core::ffi::c_int
            } else if (3760 as core::ffi::c_int) < v {
                3760 as core::ffi::c_int
            } else {
                v
            };
        }
        _ => {}
    }
    return v;
}
#[no_mangle]
pub unsafe extern "C" fn avifFullToLimitedUV(
    mut depth: uint32_t,
    mut v: core::ffi::c_int,
) -> core::ffi::c_int {
    match depth {
        8 => {
            v = (v * (240 as core::ffi::c_int - 16 as core::ffi::c_int)
                + 255 as core::ffi::c_int / 2 as core::ffi::c_int)
                / 255 as core::ffi::c_int + 16 as core::ffi::c_int;
            v = if v < 16 as core::ffi::c_int {
                16 as core::ffi::c_int
            } else if (240 as core::ffi::c_int) < v {
                240 as core::ffi::c_int
            } else {
                v
            };
        }
        10 => {
            v = (v * (960 as core::ffi::c_int - 64 as core::ffi::c_int)
                + 1023 as core::ffi::c_int / 2 as core::ffi::c_int)
                / 1023 as core::ffi::c_int + 64 as core::ffi::c_int;
            v = if v < 64 as core::ffi::c_int {
                64 as core::ffi::c_int
            } else if (960 as core::ffi::c_int) < v {
                960 as core::ffi::c_int
            } else {
                v
            };
        }
        12 => {
            v = (v * (3840 as core::ffi::c_int - 256 as core::ffi::c_int)
                + 4095 as core::ffi::c_int / 2 as core::ffi::c_int)
                / 4095 as core::ffi::c_int + 256 as core::ffi::c_int;
            v = if v < 256 as core::ffi::c_int {
                256 as core::ffi::c_int
            } else if (3840 as core::ffi::c_int) < v {
                3840 as core::ffi::c_int
            } else {
                v
            };
        }
        _ => {}
    }
    return v;
}
#[inline]
unsafe extern "C" fn avifFloatToF16(mut v: core::ffi::c_float) -> uint16_t {
    let mut f16: avifF16 = avifF16 { f: 0. };
    f16.f = v * F16_MULTIPLIER;
    return (f16.u32_0 >> 13 as core::ffi::c_int) as uint16_t;
}
#[inline]
unsafe extern "C" fn avifF16ToFloat(mut v: uint16_t) -> core::ffi::c_float {
    let mut f16: avifF16 = avifF16 { f: 0. };
    f16.u32_0 = ((v as core::ffi::c_int) << 13 as core::ffi::c_int) as uint32_t;
    return f16.f / F16_MULTIPLIER;
}
#[no_mangle]
pub unsafe extern "C" fn avifGetRGBAPixel(
    mut src: *const avifRGBImage,
    mut x: uint32_t,
    mut y: uint32_t,
    mut info: *const avifRGBColorSpaceInfo,
    mut rgbaPixel: *mut core::ffi::c_float,
) {
    'c_16801: {
        if !src.is_null() {} else {
            __assert_fail(
                b"src != NULL\0" as *const u8 as *const core::ffi::c_char,
                b"/home/runner/work/libavif-rs/libavif-rs/upstream/libavif/src/reformat.c\0"
                    as *const u8 as *const core::ffi::c_char,
                1862 as core::ffi::c_uint,
                (::core::mem::transmute::<
                    [u8; 104],
                    [core::ffi::c_char; 104],
                >(
                    *b"void avifGetRGBAPixel(const avifRGBImage *, uint32_t, uint32_t, const avifRGBColorSpaceInfo *, float *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    'c_16746: {
        if (*src).isFloat == 0 || (*src).depth == 16 as uint32_t {} else {
            __assert_fail(
                b"!src->isFloat || src->depth == 16\0" as *const u8
                    as *const core::ffi::c_char,
                b"/home/runner/work/libavif-rs/libavif-rs/upstream/libavif/src/reformat.c\0"
                    as *const u8 as *const core::ffi::c_char,
                1863 as core::ffi::c_uint,
                (::core::mem::transmute::<
                    [u8; 104],
                    [core::ffi::c_char; 104],
                >(
                    *b"void avifGetRGBAPixel(const avifRGBImage *, uint32_t, uint32_t, const avifRGBColorSpaceInfo *, float *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    'c_16684: {
        if (*src).format as core::ffi::c_uint
            != AVIF_RGB_FORMAT_RGB_565 as core::ffi::c_int as core::ffi::c_uint
            || (*src).depth == 8 as uint32_t
        {} else {
            __assert_fail(
                b"src->format != AVIF_RGB_FORMAT_RGB_565 || src->depth == 8\0"
                    as *const u8 as *const core::ffi::c_char,
                b"/home/runner/work/libavif-rs/libavif-rs/upstream/libavif/src/reformat.c\0"
                    as *const u8 as *const core::ffi::c_char,
                1864 as core::ffi::c_uint,
                (::core::mem::transmute::<
                    [u8; 104],
                    [core::ffi::c_char; 104],
                >(
                    *b"void avifGetRGBAPixel(const avifRGBImage *, uint32_t, uint32_t, const avifRGBColorSpaceInfo *, float *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    let srcPixel: *const uint8_t = &mut *((*src).pixels)
        .offset(
            (y as size_t)
                .wrapping_mul((*src).rowBytes as size_t)
                .wrapping_add((x as size_t).wrapping_mul((*info).pixelBytes as size_t))
                as isize,
        ) as *mut uint8_t;
    if (*info).channelBytes > 1 as uint32_t {
        let mut r: uint16_t = *(&*srcPixel.offset((*info).offsetBytesR as isize)
            as *const uint8_t as *const uint16_t);
        let mut g: uint16_t = *(&*srcPixel.offset((*info).offsetBytesG as isize)
            as *const uint8_t as *const uint16_t);
        let mut b: uint16_t = *(&*srcPixel.offset((*info).offsetBytesB as isize)
            as *const uint8_t as *const uint16_t);
        let mut a: uint16_t = (if avifRGBFormatHasAlpha((*src).format) != 0 {
            *(&*srcPixel.offset((*info).offsetBytesA as isize) as *const uint8_t
                as *const uint16_t) as core::ffi::c_int
        } else {
            (*info).maxChannel as uint16_t as core::ffi::c_int
        }) as uint16_t;
        if (*src).isFloat != 0 {
            *rgbaPixel.offset(0 as core::ffi::c_int as isize) = avifF16ToFloat(r);
            *rgbaPixel.offset(1 as core::ffi::c_int as isize) = avifF16ToFloat(g);
            *rgbaPixel.offset(2 as core::ffi::c_int as isize) = avifF16ToFloat(b);
            *rgbaPixel.offset(3 as core::ffi::c_int as isize) = if avifRGBFormatHasAlpha(
                (*src).format,
            ) != 0
            {
                avifF16ToFloat(a)
            } else {
                1.0f32
            };
        } else {
            *rgbaPixel.offset(0 as core::ffi::c_int as isize) = r as core::ffi::c_int
                as core::ffi::c_float / (*info).maxChannelF;
            *rgbaPixel.offset(1 as core::ffi::c_int as isize) = g as core::ffi::c_int
                as core::ffi::c_float / (*info).maxChannelF;
            *rgbaPixel.offset(2 as core::ffi::c_int as isize) = b as core::ffi::c_int
                as core::ffi::c_float / (*info).maxChannelF;
            *rgbaPixel.offset(3 as core::ffi::c_int as isize) = a as core::ffi::c_int
                as core::ffi::c_float / (*info).maxChannelF;
        }
    } else if (*src).format as core::ffi::c_uint
        == AVIF_RGB_FORMAT_RGB_565 as core::ffi::c_int as core::ffi::c_uint
    {
        let mut r_0: uint8_t = 0;
        let mut g_0: uint8_t = 0;
        let mut b_0: uint8_t = 0;
        avifGetRGB565(
            &*srcPixel.offset((*info).offsetBytesR as isize),
            &mut r_0,
            &mut g_0,
            &mut b_0,
        );
        *rgbaPixel.offset(0 as core::ffi::c_int as isize) = r_0 as core::ffi::c_int
            as core::ffi::c_float / (*info).maxChannelF;
        *rgbaPixel.offset(1 as core::ffi::c_int as isize) = g_0 as core::ffi::c_int
            as core::ffi::c_float / (*info).maxChannelF;
        *rgbaPixel.offset(2 as core::ffi::c_int as isize) = b_0 as core::ffi::c_int
            as core::ffi::c_float / (*info).maxChannelF;
        *rgbaPixel.offset(3 as core::ffi::c_int as isize) = 1.0f32;
    } else {
        *rgbaPixel.offset(0 as core::ffi::c_int as isize) = *srcPixel
            .offset((*info).offsetBytesR as isize) as core::ffi::c_int
            as core::ffi::c_float / (*info).maxChannelF;
        *rgbaPixel.offset(1 as core::ffi::c_int as isize) = *srcPixel
            .offset((*info).offsetBytesG as isize) as core::ffi::c_int
            as core::ffi::c_float / (*info).maxChannelF;
        *rgbaPixel.offset(2 as core::ffi::c_int as isize) = *srcPixel
            .offset((*info).offsetBytesB as isize) as core::ffi::c_int
            as core::ffi::c_float / (*info).maxChannelF;
        *rgbaPixel.offset(3 as core::ffi::c_int as isize) = if avifRGBFormatHasAlpha(
            (*src).format,
        ) != 0
        {
            *srcPixel.offset((*info).offsetBytesA as isize) as core::ffi::c_int
                as core::ffi::c_float / (*info).maxChannelF
        } else {
            1.0f32
        };
    };
}
#[no_mangle]
pub unsafe extern "C" fn avifSetRGBAPixel(
    mut dst: *const avifRGBImage,
    mut x: uint32_t,
    mut y: uint32_t,
    mut info: *const avifRGBColorSpaceInfo,
    mut rgbaPixel: *const core::ffi::c_float,
) {
    'c_17509: {
        if !dst.is_null() {} else {
            __assert_fail(
                b"dst != NULL\0" as *const u8 as *const core::ffi::c_char,
                b"/home/runner/work/libavif-rs/libavif-rs/upstream/libavif/src/reformat.c\0"
                    as *const u8 as *const core::ffi::c_char,
                1902 as core::ffi::c_uint,
                (::core::mem::transmute::<
                    [u8; 110],
                    [core::ffi::c_char; 110],
                >(
                    *b"void avifSetRGBAPixel(const avifRGBImage *, uint32_t, uint32_t, const avifRGBColorSpaceInfo *, const float *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    'c_17455: {
        if (*dst).isFloat == 0 || (*dst).depth == 16 as uint32_t {} else {
            __assert_fail(
                b"!dst->isFloat || dst->depth == 16\0" as *const u8
                    as *const core::ffi::c_char,
                b"/home/runner/work/libavif-rs/libavif-rs/upstream/libavif/src/reformat.c\0"
                    as *const u8 as *const core::ffi::c_char,
                1903 as core::ffi::c_uint,
                (::core::mem::transmute::<
                    [u8; 110],
                    [core::ffi::c_char; 110],
                >(
                    *b"void avifSetRGBAPixel(const avifRGBImage *, uint32_t, uint32_t, const avifRGBColorSpaceInfo *, const float *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    'c_17395: {
        if (*dst).format as core::ffi::c_uint
            != AVIF_RGB_FORMAT_RGB_565 as core::ffi::c_int as core::ffi::c_uint
            || (*dst).depth == 8 as uint32_t
        {} else {
            __assert_fail(
                b"dst->format != AVIF_RGB_FORMAT_RGB_565 || dst->depth == 8\0"
                    as *const u8 as *const core::ffi::c_char,
                b"/home/runner/work/libavif-rs/libavif-rs/upstream/libavif/src/reformat.c\0"
                    as *const u8 as *const core::ffi::c_char,
                1904 as core::ffi::c_uint,
                (::core::mem::transmute::<
                    [u8; 110],
                    [core::ffi::c_char; 110],
                >(
                    *b"void avifSetRGBAPixel(const avifRGBImage *, uint32_t, uint32_t, const avifRGBColorSpaceInfo *, const float *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    'c_17337: {
        if *rgbaPixel.offset(0 as core::ffi::c_int as isize) >= 0.0f32
            && *rgbaPixel.offset(0 as core::ffi::c_int as isize) <= 1.0f32
        {} else {
            __assert_fail(
                b"rgbaPixel[0] >= 0.0f && rgbaPixel[0] <= 1.0f\0" as *const u8
                    as *const core::ffi::c_char,
                b"/home/runner/work/libavif-rs/libavif-rs/upstream/libavif/src/reformat.c\0"
                    as *const u8 as *const core::ffi::c_char,
                1905 as core::ffi::c_uint,
                (::core::mem::transmute::<
                    [u8; 110],
                    [core::ffi::c_char; 110],
                >(
                    *b"void avifSetRGBAPixel(const avifRGBImage *, uint32_t, uint32_t, const avifRGBColorSpaceInfo *, const float *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    'c_17279: {
        if *rgbaPixel.offset(1 as core::ffi::c_int as isize) >= 0.0f32
            && *rgbaPixel.offset(1 as core::ffi::c_int as isize) <= 1.0f32
        {} else {
            __assert_fail(
                b"rgbaPixel[1] >= 0.0f && rgbaPixel[1] <= 1.0f\0" as *const u8
                    as *const core::ffi::c_char,
                b"/home/runner/work/libavif-rs/libavif-rs/upstream/libavif/src/reformat.c\0"
                    as *const u8 as *const core::ffi::c_char,
                1906 as core::ffi::c_uint,
                (::core::mem::transmute::<
                    [u8; 110],
                    [core::ffi::c_char; 110],
                >(
                    *b"void avifSetRGBAPixel(const avifRGBImage *, uint32_t, uint32_t, const avifRGBColorSpaceInfo *, const float *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    'c_17219: {
        if *rgbaPixel.offset(2 as core::ffi::c_int as isize) >= 0.0f32
            && *rgbaPixel.offset(2 as core::ffi::c_int as isize) <= 1.0f32
        {} else {
            __assert_fail(
                b"rgbaPixel[2] >= 0.0f && rgbaPixel[2] <= 1.0f\0" as *const u8
                    as *const core::ffi::c_char,
                b"/home/runner/work/libavif-rs/libavif-rs/upstream/libavif/src/reformat.c\0"
                    as *const u8 as *const core::ffi::c_char,
                1907 as core::ffi::c_uint,
                (::core::mem::transmute::<
                    [u8; 110],
                    [core::ffi::c_char; 110],
                >(
                    *b"void avifSetRGBAPixel(const avifRGBImage *, uint32_t, uint32_t, const avifRGBColorSpaceInfo *, const float *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    let dstPixel: *mut uint8_t = &mut *((*dst).pixels)
        .offset(
            (y as size_t)
                .wrapping_mul((*dst).rowBytes as size_t)
                .wrapping_add((x as size_t).wrapping_mul((*info).pixelBytes as size_t))
                as isize,
        ) as *mut uint8_t;
    let ptrR: *mut uint8_t = &mut *dstPixel.offset((*info).offsetBytesR as isize)
        as *mut uint8_t;
    let ptrG: *mut uint8_t = &mut *dstPixel.offset((*info).offsetBytesG as isize)
        as *mut uint8_t;
    let ptrB: *mut uint8_t = &mut *dstPixel.offset((*info).offsetBytesB as isize)
        as *mut uint8_t;
    let ptrA: *mut uint8_t = if avifRGBFormatHasAlpha((*dst).format) != 0 {
        &mut *dstPixel.offset((*info).offsetBytesA as isize) as *mut uint8_t
    } else {
        0 as *mut uint8_t
    };
    if (*dst).depth > 8 as uint32_t {
        if (*dst).isFloat != 0 {
            *(ptrR as *mut uint16_t) = avifFloatToF16(
                *rgbaPixel.offset(0 as core::ffi::c_int as isize),
            );
            *(ptrG as *mut uint16_t) = avifFloatToF16(
                *rgbaPixel.offset(1 as core::ffi::c_int as isize),
            );
            *(ptrB as *mut uint16_t) = avifFloatToF16(
                *rgbaPixel.offset(2 as core::ffi::c_int as isize),
            );
            if !ptrA.is_null() {
                *(ptrA as *mut uint16_t) = avifFloatToF16(
                    *rgbaPixel.offset(3 as core::ffi::c_int as isize),
                );
            }
        } else {
            *(ptrR as *mut uint16_t) = (0.5f32
                + *rgbaPixel.offset(0 as core::ffi::c_int as isize)
                    * (*info).maxChannelF) as uint16_t;
            *(ptrG as *mut uint16_t) = (0.5f32
                + *rgbaPixel.offset(1 as core::ffi::c_int as isize)
                    * (*info).maxChannelF) as uint16_t;
            *(ptrB as *mut uint16_t) = (0.5f32
                + *rgbaPixel.offset(2 as core::ffi::c_int as isize)
                    * (*info).maxChannelF) as uint16_t;
            if !ptrA.is_null() {
                *(ptrA as *mut uint16_t) = (0.5f32
                    + *rgbaPixel.offset(3 as core::ffi::c_int as isize)
                        * (*info).maxChannelF) as uint16_t;
            }
        }
    } else {
        avifStoreRGB8Pixel(
            (*dst).format,
            (0.5f32
                + *rgbaPixel.offset(0 as core::ffi::c_int as isize)
                    * (*info).maxChannelF) as uint8_t,
            (0.5f32
                + *rgbaPixel.offset(1 as core::ffi::c_int as isize)
                    * (*info).maxChannelF) as uint8_t,
            (0.5f32
                + *rgbaPixel.offset(2 as core::ffi::c_int as isize)
                    * (*info).maxChannelF) as uint8_t,
            ptrR,
            ptrG,
            ptrB,
        );
        if !ptrA.is_null() {
            *ptrA = (0.5f32
                + *rgbaPixel.offset(3 as core::ffi::c_int as isize)
                    * (*info).maxChannelF) as uint8_t;
        }
    };
}
