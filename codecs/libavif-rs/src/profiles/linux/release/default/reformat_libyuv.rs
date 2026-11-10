// Copyright 2019 Joe Drago. All rights reserved.
// SPDX-License-Identifier: BSD-2-Clause

// Mechanically translated from upstream/libavif/src/reformat_libyuv.c.
// C ABI, ownership, pointer validity, and bounds follow the original headers.
pub type size_t = usize;
pub type __uint8_t = u8;
pub type __uint16_t = u16;
pub type __int32_t = i32;
pub type __uint32_t = u32;
pub type int32_t = __int32_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
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
pub const AVIF_FALSE: core::ffi::c_int = 0 as core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn avifImageRGBToYUVLibYUV(
    mut image: *mut avifImage,
    mut rgb: *const avifRGBImage,
) -> avifResult {
    return AVIF_RESULT_NOT_IMPLEMENTED;
}
#[no_mangle]
pub unsafe extern "C" fn avifImageYUVToRGBLibYUV(
    mut image: *const avifImage,
    mut rgb: *mut avifRGBImage,
    mut reformatAlpha: avifBool,
    mut alphaReformattedWithLibYUV: *mut avifBool,
) -> avifResult {
    *alphaReformattedWithLibYUV = AVIF_FALSE as avifBool;
    return AVIF_RESULT_NOT_IMPLEMENTED;
}
#[no_mangle]
pub unsafe extern "C" fn avifRGBImagePremultiplyAlphaLibYUV(
    mut rgb: *mut avifRGBImage,
) -> avifResult {
    return AVIF_RESULT_NOT_IMPLEMENTED;
}
#[no_mangle]
pub unsafe extern "C" fn avifRGBImageUnpremultiplyAlphaLibYUV(
    mut rgb: *mut avifRGBImage,
) -> avifResult {
    return AVIF_RESULT_NOT_IMPLEMENTED;
}
#[no_mangle]
pub unsafe extern "C" fn avifRGBImageToF16LibYUV(
    mut rgb: *mut avifRGBImage,
) -> avifResult {
    return AVIF_RESULT_NOT_IMPLEMENTED;
}
#[no_mangle]
pub unsafe extern "C" fn avifLibYUVVersion() -> core::ffi::c_uint {
    return 0 as core::ffi::c_uint;
}
