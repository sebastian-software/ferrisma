// Copyright 2023 Google LLC
// SPDX-License-Identifier: BSD-2-Clause

// Mechanically translated from upstream/libavif/src/gainmap.c.
// C ABI, ownership, pointer validity, and bounds follow the original headers.
extern "C" {
    fn avifAlloc(size: size_t) -> *mut core::ffi::c_void;
    fn avifFree(p: *mut core::ffi::c_void);
    fn avifRWDataSet(
        raw: *mut avifRWData,
        data: *const uint8_t,
        len: size_t,
    ) -> avifResult;
    fn avifDiagnosticsClearError(diag: *mut avifDiagnostics);
    fn avifDoubleToSignedFraction(
        v: core::ffi::c_double,
        fraction: *mut avifSignedFraction,
    ) -> avifBool;
    fn avifDoubleToUnsignedFraction(
        v: core::ffi::c_double,
        fraction: *mut avifUnsignedFraction,
    ) -> avifBool;
    fn avifImageCreateEmpty() -> *mut avifImage;
    fn avifImageSetViewRect(
        dstImage: *mut avifImage,
        srcImage: *const avifImage,
        rect: *const avifCropRect,
    ) -> avifResult;
    fn avifImageDestroy(image: *mut avifImage);
    fn avifImageFreePlanes(image: *mut avifImage, planes: avifPlanesFlags);
    fn avifImageScale(
        image: *mut avifImage,
        dstWidth: uint32_t,
        dstHeight: uint32_t,
        diag: *mut avifDiagnostics,
    ) -> avifResult;
    fn avifRGBImageSetDefaults(rgb: *mut avifRGBImage, image: *const avifImage);
    fn avifRGBImageAllocatePixels(rgb: *mut avifRGBImage) -> avifResult;
    fn avifRGBImageFreePixels(rgb: *mut avifRGBImage);
    fn avifImageRGBToYUV(image: *mut avifImage, rgb: *const avifRGBImage) -> avifResult;
    fn avifImageYUVToRGB(image: *const avifImage, rgb: *mut avifRGBImage) -> avifResult;
    fn avifCalloc(count: size_t, size: size_t) -> *mut core::ffi::c_void;
    fn avifRoundf(v: core::ffi::c_float) -> core::ffi::c_float;
    fn avifTransferCharacteristicsGetGammaToLinearFunction(
        atc: avifTransferCharacteristics,
    ) -> avifTransferFunction;
    fn avifTransferCharacteristicsGetLinearToGammaFunction(
        atc: avifTransferCharacteristics,
    ) -> avifTransferFunction;
    fn avifColorPrimariesComputeYCoeffs(
        colorPrimaries: avifColorPrimaries,
        coeffs: *mut core::ffi::c_float,
    );
    fn avifColorPrimariesComputeRGBToRGBMatrix(
        srcColorPrimaries: avifColorPrimaries,
        dstColorPrimaries: avifColorPrimaries,
        coeffs: *mut [core::ffi::c_double; 3],
    ) -> avifBool;
    fn avifLinearRGBConvertColorSpace(
        rgb: *mut core::ffi::c_float,
        coeffs: *mut [core::ffi::c_double; 3],
    );
    fn avifGetRGBColorSpaceInfo(
        rgb: *const avifRGBImage,
        info: *mut avifRGBColorSpaceInfo,
    ) -> avifBool;
    fn avifGetRGBAPixel(
        src: *const avifRGBImage,
        x: uint32_t,
        y: uint32_t,
        info: *const avifRGBColorSpaceInfo,
        rgbaPixel: *mut core::ffi::c_float,
    );
    fn avifSetRGBAPixel(
        dst: *const avifRGBImage,
        x: uint32_t,
        y: uint32_t,
        info: *const avifRGBColorSpaceInfo,
        rgbaPixel: *const core::ffi::c_float,
    );
    fn avifDiagnosticsPrintf(
        diag: *mut avifDiagnostics,
        format: *const core::ffi::c_char,
        ...
    );
    fn __assert_fail(
        __assertion: *const core::ffi::c_char,
        __file: *const core::ffi::c_char,
        __line: core::ffi::c_uint,
        __function: *const core::ffi::c_char,
    ) -> !;
    fn exp2f(__x: core::ffi::c_float) -> core::ffi::c_float;
    fn log2f(__x: core::ffi::c_float) -> core::ffi::c_float;
    fn powf(__x: core::ffi::c_float, __y: core::ffi::c_float) -> core::ffi::c_float;
    fn ceilf(__x: core::ffi::c_float) -> core::ffi::c_float;
    fn fmaxf(__x: core::ffi::c_float, __y: core::ffi::c_float) -> core::ffi::c_float;
    fn fminf(__x: core::ffi::c_float, __y: core::ffi::c_float) -> core::ffi::c_float;
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
}
pub type size_t = usize;
pub type __uint8_t = u8;
pub type __uint16_t = u16;
pub type __int32_t = i32;
pub type __uint32_t = u32;
pub type __int64_t = i64;
pub type int32_t = __int32_t;
pub type int64_t = __int64_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
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
pub type avifTransferCharacteristics = uint16_t;
pub type avifMatrixCoefficients = uint16_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct avifDiagnostics {
    pub error: [core::ffi::c_char; 256],
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
pub type avifTransferFunction = Option<
    unsafe extern "C" fn(core::ffi::c_float) -> core::ffi::c_float,
>;
pub const NULL: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
pub const SIZE_MAX: core::ffi::c_ulong = 18446744073709551615 as core::ffi::c_ulong;
pub const AVIF_TRUE: core::ffi::c_int = 1 as core::ffi::c_int;
pub const AVIF_FALSE: core::ffi::c_int = 0 as core::ffi::c_int;
pub const __ASSERT_FUNCTION: [core::ffi::c_char; 258] = unsafe {
    ::core::mem::transmute::<
        [u8; 258],
        [core::ffi::c_char; 258],
    >(
        *b"avifResult avifRGBImageApplyGainMap(const avifRGBImage *, avifColorPrimaries, avifTransferCharacteristics, const avifGainMap *, float, avifColorPrimaries, avifTransferCharacteristics, avifRGBImage *, avifContentLightLevelInformationBox *, avifDiagnostics *)\0",
    )
};
unsafe extern "C" fn avifNanSafeClamp(
    mut val: core::ffi::c_float,
) -> core::ffi::c_float {
    return fminf(1.0f32, fmaxf(0.0f32, val));
}
unsafe extern "C" fn avifGainMapSetEncodingDefaults(mut gainMap: *mut avifGainMap) {
    let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
    while i < 3 as core::ffi::c_int {
        (*gainMap).gainMapMin[i as usize] = {
            let mut init = avifSignedFraction {
                n: 1 as int32_t,
                d: 1 as uint32_t,
            };
            init
        };
        (*gainMap).gainMapMax[i as usize] = {
            let mut init = avifSignedFraction {
                n: 1 as int32_t,
                d: 1 as uint32_t,
            };
            init
        };
        (*gainMap).baseOffset[i as usize] = {
            let mut init = avifSignedFraction {
                n: 1 as int32_t,
                d: 64 as uint32_t,
            };
            init
        };
        (*gainMap).alternateOffset[i as usize] = {
            let mut init = avifSignedFraction {
                n: 1 as int32_t,
                d: 64 as uint32_t,
            };
            init
        };
        (*gainMap).gainMapGamma[i as usize] = {
            let mut init = avifUnsignedFraction {
                n: 1 as uint32_t,
                d: 1 as uint32_t,
            };
            init
        };
        i += 1;
    }
    (*gainMap).baseHdrHeadroom = {
        let mut init = avifUnsignedFraction {
            n: 0 as uint32_t,
            d: 1 as uint32_t,
        };
        init
    };
    (*gainMap).alternateHdrHeadroom = {
        let mut init = avifUnsignedFraction {
            n: 1 as uint32_t,
            d: 1 as uint32_t,
        };
        init
    };
    (*gainMap).useBaseColorSpace = AVIF_TRUE as avifBool;
}
unsafe extern "C" fn avifSignedFractionToFloat(
    mut f: avifSignedFraction,
) -> core::ffi::c_float {
    if f.d == 0 as uint32_t {
        return 0.0f32;
    }
    return f.n as core::ffi::c_float / f.d as core::ffi::c_float;
}
unsafe extern "C" fn avifUnsignedFractionToFloat(
    mut f: avifUnsignedFraction,
) -> core::ffi::c_float {
    if f.d == 0 as uint32_t {
        return 0.0f32;
    }
    return f.n as core::ffi::c_float / f.d as core::ffi::c_float;
}
unsafe extern "C" fn avifGetGainMapWeight(
    mut hdrHeadroom: core::ffi::c_float,
    mut gainMap: *const avifGainMap,
) -> core::ffi::c_float {
    let baseHdrHeadroom: core::ffi::c_float = avifUnsignedFractionToFloat(
        (*gainMap).baseHdrHeadroom,
    ) as core::ffi::c_float;
    let alternateHdrHeadroom: core::ffi::c_float = avifUnsignedFractionToFloat(
        (*gainMap).alternateHdrHeadroom,
    ) as core::ffi::c_float;
    if baseHdrHeadroom == alternateHdrHeadroom {
        return 0.0f32;
    }
    let w: core::ffi::c_float = if (hdrHeadroom - baseHdrHeadroom)
        / (alternateHdrHeadroom - baseHdrHeadroom) < 0.0f32
    {
        0.0f32
    } else if 1.0f32
        < (hdrHeadroom - baseHdrHeadroom) / (alternateHdrHeadroom - baseHdrHeadroom)
    {
        1.0f32
    } else {
        (hdrHeadroom - baseHdrHeadroom) / (alternateHdrHeadroom - baseHdrHeadroom)
    };
    return if alternateHdrHeadroom < baseHdrHeadroom { -w } else { w };
}
#[inline]
unsafe extern "C" fn lerp(
    mut a: core::ffi::c_float,
    mut b: core::ffi::c_float,
    mut w: core::ffi::c_float,
) -> core::ffi::c_float {
    return (1.0f32 - w) * a + w * b;
}
#[no_mangle]
pub unsafe extern "C" fn avifRGBImageApplyGainMap(
    mut baseImage: *const avifRGBImage,
    mut baseColorPrimaries: avifColorPrimaries,
    mut baseTransferCharacteristics: avifTransferCharacteristics,
    mut gainMap: *const avifGainMap,
    mut hdrHeadroom: core::ffi::c_float,
    mut outputColorPrimaries: avifColorPrimaries,
    mut outputTransferCharacteristics: avifTransferCharacteristics,
    mut toneMappedImage: *mut avifRGBImage,
    mut clli: *mut avifContentLightLevelInformationBox,
    mut diag: *mut avifDiagnostics,
) -> avifResult {
    let mut baseRGBInfo: avifRGBColorSpaceInfo = avifRGBColorSpaceInfo {
        channelBytes: 0,
        pixelBytes: 0,
        offsetBytesR: 0,
        offsetBytesG: 0,
        offsetBytesB: 0,
        offsetBytesA: 0,
        offsetBytesGray: 0,
        maxChannel: 0,
        maxChannelF: 0.,
    };
    let mut toneMappedPixelRGBInfo: avifRGBColorSpaceInfo = avifRGBColorSpaceInfo {
        channelBytes: 0,
        pixelBytes: 0,
        offsetBytesR: 0,
        offsetBytesG: 0,
        offsetBytesB: 0,
        offsetBytesA: 0,
        offsetBytesGray: 0,
        maxChannel: 0,
        maxChannelF: 0.,
    };
    let mut gammaToLinear: avifTransferFunction = None;
    let mut linearToGamma: avifTransferFunction = None;
    let mut inputConversionCoeffs: [[core::ffi::c_double; 3]; 3] = [[0.; 3]; 3];
    let mut outputConversionCoeffs: [[core::ffi::c_double; 3]; 3] = [[0.; 3]; 3];
    let mut gainMapImage: *const avifImage = 0 as *const avifImage;
    let mut gainMapRGBInfo: avifRGBColorSpaceInfo = avifRGBColorSpaceInfo {
        channelBytes: 0,
        pixelBytes: 0,
        offsetBytesR: 0,
        offsetBytesG: 0,
        offsetBytesB: 0,
        offsetBytesA: 0,
        offsetBytesGray: 0,
        maxChannel: 0,
        maxChannelF: 0.,
    };
    let mut rgbMaxLinear: core::ffi::c_float = 0.;
    let mut rgbSumLinear: core::ffi::c_float = 0.;
    let mut gammaInv: [core::ffi::c_float; 3] = [0.; 3];
    let mut gainMapMin: [core::ffi::c_float; 3] = [0.; 3];
    let mut gainMapMax: [core::ffi::c_float; 3] = [0.; 3];
    let mut baseOffset: [core::ffi::c_float; 3] = [0.; 3];
    let mut alternateOffset: [core::ffi::c_float; 3] = [0.; 3];
    let mut current_block: u64;
    avifDiagnosticsClearError(diag);
    if hdrHeadroom < 0.0f32 {
        avifDiagnosticsPrintf(
            diag,
            b"hdrHeadroom should be >= 0, got %f\0" as *const u8
                as *const core::ffi::c_char,
            hdrHeadroom as core::ffi::c_double,
        );
        return AVIF_RESULT_INVALID_ARGUMENT;
    }
    if baseImage.is_null() || gainMap.is_null() || toneMappedImage.is_null() {
        avifDiagnosticsPrintf(
            diag,
            b"NULL input image\0" as *const u8 as *const core::ffi::c_char,
        );
        return AVIF_RESULT_INVALID_ARGUMENT;
    }
    if ((*gainMap).image).is_null() {
        avifDiagnosticsPrintf(
            diag,
            b"gainMap->image is null (gain map image not decoded?)\0" as *const u8
                as *const core::ffi::c_char,
        );
        return AVIF_RESULT_INVALID_ARGUMENT;
    }
    let result__: avifResult = avifGainMapValidateMetadata(gainMap, diag) as avifResult;
    if result__ as core::ffi::c_uint
        != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
    {
        return result__;
    }
    let width: uint32_t = (*baseImage).width;
    let height: uint32_t = (*baseImage).height;
    let useBaseColorSpace: avifBool = (*gainMap).useBaseColorSpace;
    let gainMapMathPrimaries: avifColorPrimaries = (if useBaseColorSpace != 0
        || (*gainMap).altColorPrimaries as core::ffi::c_int
            == AVIF_COLOR_PRIMARIES_UNSPECIFIED as core::ffi::c_int
    {
        baseColorPrimaries as core::ffi::c_int
    } else {
        (*gainMap).altColorPrimaries as core::ffi::c_int
    }) as avifColorPrimaries;
    let needsInputColorConversion: avifBool = (baseColorPrimaries as core::ffi::c_int
        != gainMapMathPrimaries as core::ffi::c_int) as core::ffi::c_int;
    let needsOutputColorConversion: avifBool = (gainMapMathPrimaries as core::ffi::c_int
        != outputColorPrimaries as core::ffi::c_int) as core::ffi::c_int;
    let mut rescaledGainMap: *mut avifImage = 0 as *mut avifImage;
    let mut rgbGainMap: avifRGBImage = avifRGBImage {
        width: 0,
        height: 0,
        depth: 0,
        format: AVIF_RGB_FORMAT_RGB,
        chromaUpsampling: AVIF_CHROMA_UPSAMPLING_AUTOMATIC,
        chromaDownsampling: AVIF_CHROMA_DOWNSAMPLING_AUTOMATIC,
        avoidLibYUV: 0,
        ignoreAlpha: 0,
        alphaPremultiplied: 0,
        isFloat: 0,
        maxThreads: 0,
        pixels: 0 as *mut uint8_t,
        rowBytes: 0,
    };
    memset(
        &mut rgbGainMap as *mut avifRGBImage as *mut core::ffi::c_void,
        0 as core::ffi::c_int,
        ::core::mem::size_of::<avifRGBImage>() as size_t,
    );
    let mut res: avifResult = AVIF_RESULT_OK;
    (*toneMappedImage).width = width;
    (*toneMappedImage).height = height;
    let result___0: avifResult = avifRGBImageAllocatePixels(toneMappedImage)
        as avifResult;
    if result___0 as core::ffi::c_uint
        != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
    {
        return result___0;
    }
    let weight: core::ffi::c_float = avifGetGainMapWeight(hdrHeadroom, gainMap)
        as core::ffi::c_float;
    if weight == 0.0f32
        && outputTransferCharacteristics as core::ffi::c_int
            == baseTransferCharacteristics as core::ffi::c_int
        && outputColorPrimaries as core::ffi::c_int
            == baseColorPrimaries as core::ffi::c_int
        && (*baseImage).format as core::ffi::c_uint
            == (*toneMappedImage).format as core::ffi::c_uint
        && (*baseImage).depth == (*toneMappedImage).depth
        && (*baseImage).isFloat == (*toneMappedImage).isFloat
        && (*baseImage).rowBytes == (*toneMappedImage).rowBytes
    {
        'c_3075: {
            if (*baseImage).height == (*toneMappedImage).height {} else {
                __assert_fail(
                    b"baseImage->height == toneMappedImage->height\0" as *const u8
                        as *const core::ffi::c_char,
                    b"/home/runner/work/libavif-rs/libavif-rs/upstream/libavif/src/gainmap.c\0"
                        as *const u8 as *const core::ffi::c_char,
                    132 as core::ffi::c_uint,
                    __ASSERT_FUNCTION.as_ptr(),
                );
            }
        };
        memcpy(
            (*toneMappedImage).pixels as *mut core::ffi::c_void,
            (*baseImage).pixels as *const core::ffi::c_void,
            ((*baseImage).rowBytes as size_t).wrapping_mul((*baseImage).height as size_t),
        );
    } else {
        baseRGBInfo = avifRGBColorSpaceInfo {
            channelBytes: 0,
            pixelBytes: 0,
            offsetBytesR: 0,
            offsetBytesG: 0,
            offsetBytesB: 0,
            offsetBytesA: 0,
            offsetBytesGray: 0,
            maxChannel: 0,
            maxChannelF: 0.,
        };
        toneMappedPixelRGBInfo = avifRGBColorSpaceInfo {
            channelBytes: 0,
            pixelBytes: 0,
            offsetBytesR: 0,
            offsetBytesG: 0,
            offsetBytesB: 0,
            offsetBytesA: 0,
            offsetBytesGray: 0,
            maxChannel: 0,
            maxChannelF: 0.,
        };
        if avifGetRGBColorSpaceInfo(baseImage, &mut baseRGBInfo) == 0
            || avifGetRGBColorSpaceInfo(toneMappedImage, &mut toneMappedPixelRGBInfo)
                == 0
        {
            avifDiagnosticsPrintf(
                diag,
                b"Unsupported RGB color space\0" as *const u8 as *const core::ffi::c_char,
            );
            res = AVIF_RESULT_NOT_IMPLEMENTED;
        } else {
            gammaToLinear = avifTransferCharacteristicsGetGammaToLinearFunction(
                baseTransferCharacteristics,
            ) as avifTransferFunction;
            linearToGamma = avifTransferCharacteristicsGetLinearToGammaFunction(
                outputTransferCharacteristics,
            ) as avifTransferFunction;
            if weight == 0.0f32 {
                let primariesDiffer: avifBool = (baseColorPrimaries as core::ffi::c_int
                    != outputColorPrimaries as core::ffi::c_int) as core::ffi::c_int;
                let mut conversionCoeffs: [[core::ffi::c_double; 3]; 3] = [[0.; 3]; 3];
                if primariesDiffer != 0
                    && avifColorPrimariesComputeRGBToRGBMatrix(
                        baseColorPrimaries,
                        outputColorPrimaries,
                        conversionCoeffs.as_mut_ptr(),
                    ) == 0
                {
                    avifDiagnosticsPrintf(
                        diag,
                        b"Unsupported RGB color space conversion\0" as *const u8
                            as *const core::ffi::c_char,
                    );
                    res = AVIF_RESULT_NOT_IMPLEMENTED;
                } else {
                    let mut j: uint32_t = 0 as uint32_t;
                    while j < height {
                        let mut i: uint32_t = 0 as uint32_t;
                        while i < width {
                            let mut basePixelRGBA: [core::ffi::c_float; 4] = [0.; 4];
                            avifGetRGBAPixel(
                                baseImage,
                                i,
                                j,
                                &mut baseRGBInfo,
                                basePixelRGBA.as_mut_ptr(),
                            );
                            if outputTransferCharacteristics as core::ffi::c_int
                                != baseTransferCharacteristics as core::ffi::c_int
                                || primariesDiffer != 0
                            {
                                let mut c: core::ffi::c_int = 0 as core::ffi::c_int;
                                while c < 3 as core::ffi::c_int {
                                    basePixelRGBA[c as usize] = gammaToLinear
                                        .expect(
                                            "non-null function pointer",
                                        )(basePixelRGBA[c as usize]);
                                    c += 1;
                                }
                                if primariesDiffer != 0 {
                                    avifLinearRGBConvertColorSpace(
                                        basePixelRGBA.as_mut_ptr(),
                                        conversionCoeffs.as_mut_ptr(),
                                    );
                                }
                                let mut c_0: core::ffi::c_int = 0 as core::ffi::c_int;
                                while c_0 < 3 as core::ffi::c_int {
                                    basePixelRGBA[c_0 as usize] = avifNanSafeClamp(
                                        linearToGamma
                                            .expect(
                                                "non-null function pointer",
                                            )(basePixelRGBA[c_0 as usize]),
                                    );
                                    c_0 += 1;
                                }
                            }
                            avifSetRGBAPixel(
                                toneMappedImage,
                                i,
                                j,
                                &mut toneMappedPixelRGBInfo,
                                basePixelRGBA.as_mut_ptr() as *const core::ffi::c_float,
                            );
                            i = i.wrapping_add(1);
                        }
                        j = j.wrapping_add(1);
                    }
                }
            } else {
                inputConversionCoeffs = [[0.; 3]; 3];
                outputConversionCoeffs = [[0.; 3]; 3];
                if needsInputColorConversion != 0
                    && avifColorPrimariesComputeRGBToRGBMatrix(
                        baseColorPrimaries,
                        gainMapMathPrimaries,
                        inputConversionCoeffs.as_mut_ptr(),
                    ) == 0
                {
                    avifDiagnosticsPrintf(
                        diag,
                        b"Unsupported RGB color space conversion\0" as *const u8
                            as *const core::ffi::c_char,
                    );
                    res = AVIF_RESULT_NOT_IMPLEMENTED;
                } else if needsOutputColorConversion != 0
                    && avifColorPrimariesComputeRGBToRGBMatrix(
                        gainMapMathPrimaries,
                        outputColorPrimaries,
                        outputConversionCoeffs.as_mut_ptr(),
                    ) == 0
                {
                    avifDiagnosticsPrintf(
                        diag,
                        b"Unsupported RGB color space conversion\0" as *const u8
                            as *const core::ffi::c_char,
                    );
                    res = AVIF_RESULT_NOT_IMPLEMENTED;
                } else {
                    if (*(*gainMap).image).width != width
                        || (*(*gainMap).image).height != height
                    {
                        rescaledGainMap = avifImageCreateEmpty();
                        if rescaledGainMap.is_null() {
                            res = AVIF_RESULT_OUT_OF_MEMORY;
                            current_block = 0;
                        } else {
                            let rect: avifCropRect = {
                                let mut init = avifCropRect {
                                    x: 0 as uint32_t,
                                    y: 0 as uint32_t,
                                    width: (*(*gainMap).image).width,
                                    height: (*(*gainMap).image).height,
                                };
                                init
                            };
                            res = avifImageSetViewRect(
                                rescaledGainMap,
                                (*gainMap).image,
                                &rect,
                            );
                            if res as core::ffi::c_uint
                                != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
                            {
                                current_block = 0;
                            } else {
                                res = avifImageScale(rescaledGainMap, width, height, diag);
                                if res as core::ffi::c_uint
                                    != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
                                {
                                    current_block = 0;
                                } else {
                                    current_block = 1;
                                }
                            }
                        }
                    } else {
                        current_block = 1;
                    }
                    match current_block {
                        0 => {}
                        _ => {
                            gainMapImage = if !rescaledGainMap.is_null() {
                                rescaledGainMap as *mut avifImage
                            } else {
                                (*gainMap).image
                            };
                            avifRGBImageSetDefaults(&mut rgbGainMap, gainMapImage);
                            res = avifRGBImageAllocatePixels(&mut rgbGainMap);
                            if !(res as core::ffi::c_uint
                                != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint)
                            {
                                res = avifImageYUVToRGB(gainMapImage, &mut rgbGainMap);
                                if !(res as core::ffi::c_uint
                                    != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint)
                                {
                                    gainMapRGBInfo = avifRGBColorSpaceInfo {
                                        channelBytes: 0,
                                        pixelBytes: 0,
                                        offsetBytesR: 0,
                                        offsetBytesG: 0,
                                        offsetBytesB: 0,
                                        offsetBytesA: 0,
                                        offsetBytesGray: 0,
                                        maxChannel: 0,
                                        maxChannelF: 0.,
                                    };
                                    if avifGetRGBColorSpaceInfo(
                                        &mut rgbGainMap,
                                        &mut gainMapRGBInfo,
                                    ) == 0
                                    {
                                        avifDiagnosticsPrintf(
                                            diag,
                                            b"Unsupported RGB color space\0" as *const u8
                                                as *const core::ffi::c_char,
                                        );
                                        res = AVIF_RESULT_NOT_IMPLEMENTED;
                                    } else {
                                        rgbMaxLinear = 0 as core::ffi::c_int as core::ffi::c_float;
                                        rgbSumLinear = 0 as core::ffi::c_int as core::ffi::c_float;
                                        gammaInv = [
                                            1.0f32
                                                / avifUnsignedFractionToFloat(
                                                    (*gainMap).gainMapGamma[0 as core::ffi::c_int as usize],
                                                ),
                                            1.0f32
                                                / avifUnsignedFractionToFloat(
                                                    (*gainMap).gainMapGamma[1 as core::ffi::c_int as usize],
                                                ),
                                            1.0f32
                                                / avifUnsignedFractionToFloat(
                                                    (*gainMap).gainMapGamma[2 as core::ffi::c_int as usize],
                                                ),
                                        ];
                                        gainMapMin = [
                                            avifSignedFractionToFloat(
                                                (*gainMap).gainMapMin[0 as core::ffi::c_int as usize],
                                            ),
                                            avifSignedFractionToFloat(
                                                (*gainMap).gainMapMin[1 as core::ffi::c_int as usize],
                                            ),
                                            avifSignedFractionToFloat(
                                                (*gainMap).gainMapMin[2 as core::ffi::c_int as usize],
                                            ),
                                        ];
                                        gainMapMax = [
                                            avifSignedFractionToFloat(
                                                (*gainMap).gainMapMax[0 as core::ffi::c_int as usize],
                                            ),
                                            avifSignedFractionToFloat(
                                                (*gainMap).gainMapMax[1 as core::ffi::c_int as usize],
                                            ),
                                            avifSignedFractionToFloat(
                                                (*gainMap).gainMapMax[2 as core::ffi::c_int as usize],
                                            ),
                                        ];
                                        baseOffset = [
                                            avifSignedFractionToFloat(
                                                (*gainMap).baseOffset[0 as core::ffi::c_int as usize],
                                            ),
                                            avifSignedFractionToFloat(
                                                (*gainMap).baseOffset[1 as core::ffi::c_int as usize],
                                            ),
                                            avifSignedFractionToFloat(
                                                (*gainMap).baseOffset[2 as core::ffi::c_int as usize],
                                            ),
                                        ];
                                        alternateOffset = [
                                            avifSignedFractionToFloat(
                                                (*gainMap).alternateOffset[0 as core::ffi::c_int as usize],
                                            ),
                                            avifSignedFractionToFloat(
                                                (*gainMap).alternateOffset[1 as core::ffi::c_int as usize],
                                            ),
                                            avifSignedFractionToFloat(
                                                (*gainMap).alternateOffset[2 as core::ffi::c_int as usize],
                                            ),
                                        ];
                                        let mut j_0: uint32_t = 0 as uint32_t;
                                        's_398: loop {
                                            if !(j_0 < height) {
                                                current_block = 2;
                                                break;
                                            }
                                            let mut i_0: uint32_t = 0 as uint32_t;
                                            while i_0 < width {
                                                let mut basePixelRGBA_0: [core::ffi::c_float; 4] = [0.; 4];
                                                avifGetRGBAPixel(
                                                    baseImage,
                                                    i_0,
                                                    j_0,
                                                    &mut baseRGBInfo,
                                                    basePixelRGBA_0.as_mut_ptr(),
                                                );
                                                let mut gainMapRGBA: [core::ffi::c_float; 4] = [0.; 4];
                                                avifGetRGBAPixel(
                                                    &mut rgbGainMap,
                                                    i_0,
                                                    j_0,
                                                    &mut gainMapRGBInfo,
                                                    gainMapRGBA.as_mut_ptr(),
                                                );
                                                let mut toneMappedPixelRGBA: [core::ffi::c_float; 4] = [0.; 4];
                                                let mut pixelRgbMaxLinear: core::ffi::c_float = 0.0f32;
                                                let mut c_1: core::ffi::c_int = 0 as core::ffi::c_int;
                                                while c_1 < 3 as core::ffi::c_int {
                                                    basePixelRGBA_0[c_1 as usize] = gammaToLinear
                                                        .expect(
                                                            "non-null function pointer",
                                                        )(basePixelRGBA_0[c_1 as usize]);
                                                    c_1 += 1;
                                                }
                                                if needsInputColorConversion != 0 {
                                                    avifLinearRGBConvertColorSpace(
                                                        basePixelRGBA_0.as_mut_ptr(),
                                                        inputConversionCoeffs.as_mut_ptr(),
                                                    );
                                                }
                                                let mut c_2: core::ffi::c_int = 0 as core::ffi::c_int;
                                                while c_2 < 3 as core::ffi::c_int {
                                                    let baseLinear: core::ffi::c_float = basePixelRGBA_0[c_2
                                                        as usize];
                                                    let gainMapValue: core::ffi::c_float = gainMapRGBA[c_2
                                                        as usize];
                                                    let gainMapLog2: core::ffi::c_float = lerp(
                                                        gainMapMin[c_2 as usize],
                                                        gainMapMax[c_2 as usize],
                                                        powf(gainMapValue, gammaInv[c_2 as usize]),
                                                    ) as core::ffi::c_float;
                                                    let toneMappedLinear: core::ffi::c_float = (baseLinear
                                                        + baseOffset[c_2 as usize])
                                                        * exp2f(gainMapLog2 * weight) as core::ffi::c_float
                                                        - alternateOffset[c_2 as usize];
                                                    if toneMappedLinear > rgbMaxLinear {
                                                        rgbMaxLinear = toneMappedLinear;
                                                    }
                                                    if toneMappedLinear > pixelRgbMaxLinear {
                                                        pixelRgbMaxLinear = toneMappedLinear;
                                                    }
                                                    toneMappedPixelRGBA[c_2 as usize] = toneMappedLinear;
                                                    c_2 += 1;
                                                }
                                                if needsOutputColorConversion != 0 {
                                                    avifLinearRGBConvertColorSpace(
                                                        toneMappedPixelRGBA.as_mut_ptr(),
                                                        outputConversionCoeffs.as_mut_ptr(),
                                                    );
                                                }
                                                let mut c_3: core::ffi::c_int = 0 as core::ffi::c_int;
                                                while c_3 < 3 as core::ffi::c_int {
                                                    if (toneMappedPixelRGBA[c_3 as usize]).is_nan() as i32 != 0
                                                    {
                                                        avifDiagnosticsPrintf(
                                                            diag,
                                                            b"Degenerate gain map parameters produce NaN at pixel (%u, %u)\0"
                                                                as *const u8 as *const core::ffi::c_char,
                                                            i_0,
                                                            j_0,
                                                        );
                                                        res = AVIF_RESULT_INVALID_TONE_MAPPED_IMAGE;
                                                        current_block = 0;
                                                        break 's_398;
                                                    } else {
                                                        toneMappedPixelRGBA[c_3 as usize] = avifNanSafeClamp(
                                                            linearToGamma
                                                                .expect(
                                                                    "non-null function pointer",
                                                                )(toneMappedPixelRGBA[c_3 as usize]),
                                                        );
                                                        c_3 += 1;
                                                    }
                                                }
                                                toneMappedPixelRGBA[3 as core::ffi::c_int as usize] = basePixelRGBA_0[3
                                                    as core::ffi::c_int as usize];
                                                rgbSumLinear += pixelRgbMaxLinear;
                                                avifSetRGBAPixel(
                                                    toneMappedImage,
                                                    i_0,
                                                    j_0,
                                                    &mut toneMappedPixelRGBInfo,
                                                    toneMappedPixelRGBA.as_mut_ptr()
                                                        as *const core::ffi::c_float,
                                                );
                                                i_0 = i_0.wrapping_add(1);
                                            }
                                            j_0 = j_0.wrapping_add(1);
                                        }
                                        match current_block {
                                            0 => {}
                                            _ => {
                                                if !clli.is_null() {
                                                    (*clli).maxCLL = (if avifRoundf(rgbMaxLinear * 203.0f32)
                                                        < 0.0f32
                                                    {
                                                        0.0f32
                                                    } else if (65535 as core::ffi::c_int as core::ffi::c_float)
                                                        < avifRoundf(rgbMaxLinear * 203.0f32)
                                                    {
                                                        65535 as core::ffi::c_int as core::ffi::c_float
                                                    } else {
                                                        avifRoundf(rgbMaxLinear * 203.0f32)
                                                    }) as uint16_t;
                                                    let rgbAverageLinear: core::ffi::c_float = rgbSumLinear
                                                        / (width as size_t).wrapping_mul(height as size_t)
                                                            as core::ffi::c_float;
                                                    (*clli).maxPALL = (if avifRoundf(
                                                        rgbAverageLinear * 203.0f32,
                                                    ) < 0.0f32
                                                    {
                                                        0.0f32
                                                    } else if (65535 as core::ffi::c_int as core::ffi::c_float)
                                                        < avifRoundf(rgbAverageLinear * 203.0f32)
                                                    {
                                                        65535 as core::ffi::c_int as core::ffi::c_float
                                                    } else {
                                                        avifRoundf(rgbAverageLinear * 203.0f32)
                                                    }) as uint16_t;
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
    avifRGBImageFreePixels(&mut rgbGainMap);
    if !rescaledGainMap.is_null() {
        avifImageDestroy(rescaledGainMap);
    }
    if res as core::ffi::c_uint
        != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
    {
        avifRGBImageFreePixels(toneMappedImage);
    }
    return res;
}
#[no_mangle]
pub unsafe extern "C" fn avifImageApplyGainMap(
    mut baseImage: *const avifImage,
    mut gainMap: *const avifGainMap,
    mut hdrHeadroom: core::ffi::c_float,
    mut outputColorPrimaries: avifColorPrimaries,
    mut outputTransferCharacteristics: avifTransferCharacteristics,
    mut toneMappedImage: *mut avifRGBImage,
    mut clli: *mut avifContentLightLevelInformationBox,
    mut diag: *mut avifDiagnostics,
) -> avifResult {
    avifDiagnosticsClearError(diag);
    if (*baseImage).icc.size > 0 as size_t || (*gainMap).altICC.size > 0 as size_t {
        avifDiagnosticsPrintf(
            diag,
            b"Tone mapping for images with ICC profiles is not supported\0" as *const u8
                as *const core::ffi::c_char,
        );
        return AVIF_RESULT_NOT_IMPLEMENTED;
    }
    let mut baseImageRgb: avifRGBImage = avifRGBImage {
        width: 0,
        height: 0,
        depth: 0,
        format: AVIF_RGB_FORMAT_RGB,
        chromaUpsampling: AVIF_CHROMA_UPSAMPLING_AUTOMATIC,
        chromaDownsampling: AVIF_CHROMA_DOWNSAMPLING_AUTOMATIC,
        avoidLibYUV: 0,
        ignoreAlpha: 0,
        alphaPremultiplied: 0,
        isFloat: 0,
        maxThreads: 0,
        pixels: 0 as *mut uint8_t,
        rowBytes: 0,
    };
    avifRGBImageSetDefaults(&mut baseImageRgb, baseImage);
    let result__: avifResult = avifRGBImageAllocatePixels(&mut baseImageRgb)
        as avifResult;
    if result__ as core::ffi::c_uint
        != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
    {
        return result__;
    }
    let mut res: avifResult = avifImageYUVToRGB(baseImage, &mut baseImageRgb);
    if !(res as core::ffi::c_uint
        != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint)
    {
        res = avifRGBImageApplyGainMap(
            &mut baseImageRgb,
            (*baseImage).colorPrimaries,
            (*baseImage).transferCharacteristics,
            gainMap,
            hdrHeadroom,
            outputColorPrimaries,
            outputTransferCharacteristics,
            toneMappedImage,
            clli,
            diag,
        );
    }
    avifRGBImageFreePixels(&mut baseImageRgb);
    return res;
}
unsafe extern "C" fn avifValueToBucketIdx(
    mut v: core::ffi::c_float,
    mut bucketMin: core::ffi::c_float,
    mut bucketMax: core::ffi::c_float,
    mut numBuckets: core::ffi::c_int,
) -> core::ffi::c_int {
    v = if v < bucketMin { bucketMin } else if bucketMax < v { bucketMax } else { v };
    return if (avifRoundf(
        (v - bucketMin) / (bucketMax - bucketMin) * numBuckets as core::ffi::c_float,
    ) as core::ffi::c_int) < numBuckets - 1 as core::ffi::c_int
    {
        avifRoundf(
            (v - bucketMin) / (bucketMax - bucketMin) * numBuckets as core::ffi::c_float,
        ) as core::ffi::c_int
    } else {
        numBuckets - 1 as core::ffi::c_int
    };
}
unsafe extern "C" fn avifBucketIdxToValue(
    mut idx: core::ffi::c_int,
    mut bucketMin: core::ffi::c_float,
    mut bucketMax: core::ffi::c_float,
    mut numBuckets: core::ffi::c_int,
) -> core::ffi::c_float {
    return idx as core::ffi::c_float * (bucketMax - bucketMin)
        / numBuckets as core::ffi::c_float + bucketMin;
}
#[no_mangle]
pub unsafe extern "C" fn avifFindMinMaxWithoutOutliers(
    mut gainMapF: *const core::ffi::c_float,
    mut numPixels: size_t,
    mut rangeMin: *mut core::ffi::c_float,
    mut rangeMax: *mut core::ffi::c_float,
) -> avifResult {
    let bucketSize: core::ffi::c_float = 0.01f32;
    let maxOutliersRatio: core::ffi::c_float = 0.001f32;
    let maxOutliersOnEachSide: core::ffi::c_int = avifRoundf(
        numPixels as core::ffi::c_float * maxOutliersRatio / 2.0f32,
    ) as core::ffi::c_int;
    let mut min: core::ffi::c_float = *gainMapF.offset(0 as core::ffi::c_int as isize);
    let mut max: core::ffi::c_float = *gainMapF.offset(0 as core::ffi::c_int as isize);
    let mut i: size_t = 1 as size_t;
    while i < numPixels {
        min = if min < *gainMapF.offset(i as isize) {
            min
        } else {
            *gainMapF.offset(i as isize)
        };
        max = if max > *gainMapF.offset(i as isize) {
            max
        } else {
            *gainMapF.offset(i as isize)
        };
        i = i.wrapping_add(1);
    }
    *rangeMin = min;
    *rangeMax = max;
    if max - min <= bucketSize * 2 as core::ffi::c_int as core::ffi::c_float
        || maxOutliersOnEachSide == 0 as core::ffi::c_int
    {
        return AVIF_RESULT_OK;
    }
    let maxNumBuckets: core::ffi::c_int = 10000 as core::ffi::c_int;
    let numBuckets: core::ffi::c_int = if (ceilf((max - min) / bucketSize)
        as core::ffi::c_int) < maxNumBuckets
    {
        ceilf((max - min) / bucketSize) as core::ffi::c_int
    } else {
        maxNumBuckets
    };
    let mut histogram: *mut core::ffi::c_int = avifCalloc(
        numBuckets as size_t,
        ::core::mem::size_of::<core::ffi::c_int>() as size_t,
    ) as *mut core::ffi::c_int;
    if histogram.is_null() {
        return AVIF_RESULT_OUT_OF_MEMORY;
    }
    let mut i_0: size_t = 0 as size_t;
    while i_0 < numPixels {
        let ref mut fresh0 = *histogram
            .offset(
                avifValueToBucketIdx(
                    *gainMapF.offset(i_0 as isize),
                    min,
                    max,
                    numBuckets,
                ) as isize,
            );
        *fresh0 += 1;
        i_0 = i_0.wrapping_add(1);
    }
    let mut leftOutliers: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut i_1: core::ffi::c_int = 0 as core::ffi::c_int;
    while i_1 < numBuckets {
        leftOutliers += *histogram.offset(i_1 as isize);
        if leftOutliers > maxOutliersOnEachSide {
            break;
        }
        if *histogram.offset(i_1 as isize) == 0 as core::ffi::c_int {
            *rangeMin = avifBucketIdxToValue(
                i_1 + 1 as core::ffi::c_int,
                min,
                max,
                numBuckets,
            );
        }
        i_1 += 1;
    }
    let mut rightOutliers: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut i_2: core::ffi::c_int = numBuckets - 1 as core::ffi::c_int;
    while i_2 >= 0 as core::ffi::c_int {
        rightOutliers += *histogram.offset(i_2 as isize);
        if rightOutliers > maxOutliersOnEachSide {
            break;
        }
        if *histogram.offset(i_2 as isize) == 0 as core::ffi::c_int {
            *rangeMax = avifBucketIdxToValue(i_2, min, max, numBuckets);
        }
        i_2 -= 1;
    }
    avifFree(histogram as *mut core::ffi::c_void);
    return AVIF_RESULT_OK;
}
#[no_mangle]
pub unsafe extern "C" fn avifGainMapValidateMetadata(
    mut gainMap: *const avifGainMap,
    mut diag: *mut avifDiagnostics,
) -> avifResult {
    let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
    while i < 3 as core::ffi::c_int {
        if (*gainMap).gainMapMin[i as usize].d == 0 as uint32_t
            || (*gainMap).gainMapMax[i as usize].d == 0 as uint32_t
            || (*gainMap).gainMapGamma[i as usize].d == 0 as uint32_t
            || (*gainMap).baseOffset[i as usize].d == 0 as uint32_t
            || (*gainMap).alternateOffset[i as usize].d == 0 as uint32_t
        {
            avifDiagnosticsPrintf(
                diag,
                b"Per-channel denominator is 0 in gain map metadata\0" as *const u8
                    as *const core::ffi::c_char,
            );
            return AVIF_RESULT_INVALID_ARGUMENT;
        }
        if ((*gainMap).gainMapMax[i as usize].n as int64_t
            * (*gainMap).gainMapMin[i as usize].d as int64_t)
            < (*gainMap).gainMapMin[i as usize].n as int64_t
                * (*gainMap).gainMapMax[i as usize].d as int64_t
        {
            avifDiagnosticsPrintf(
                diag,
                b"Per-channel max is less than per-channel min in gain map metadata\0"
                    as *const u8 as *const core::ffi::c_char,
            );
            return AVIF_RESULT_INVALID_ARGUMENT;
        }
        if (*gainMap).gainMapGamma[i as usize].n == 0 as uint32_t {
            avifDiagnosticsPrintf(
                diag,
                b"Per-channel gamma is 0 in gain map metadata\0" as *const u8
                    as *const core::ffi::c_char,
            );
            return AVIF_RESULT_INVALID_ARGUMENT;
        }
        i += 1;
    }
    if (*gainMap).baseHdrHeadroom.d == 0 as uint32_t
        || (*gainMap).alternateHdrHeadroom.d == 0 as uint32_t
    {
        avifDiagnosticsPrintf(
            diag,
            b"Headroom denominator is 0 in gain map metadata\0" as *const u8
                as *const core::ffi::c_char,
        );
        return AVIF_RESULT_INVALID_ARGUMENT;
    }
    if (*gainMap).useBaseColorSpace != 0 as core::ffi::c_int
        && (*gainMap).useBaseColorSpace != 1 as core::ffi::c_int
    {
        avifDiagnosticsPrintf(
            diag,
            b"useBaseColorSpace is %d in gain map metadata\0" as *const u8
                as *const core::ffi::c_char,
            (*gainMap).useBaseColorSpace,
        );
        return AVIF_RESULT_INVALID_ARGUMENT;
    }
    return AVIF_RESULT_OK;
}
#[no_mangle]
pub unsafe extern "C" fn avifSameGainMapMetadata(
    mut a: *const avifGainMap,
    mut b: *const avifGainMap,
) -> avifBool {
    if (*a).baseHdrHeadroom.n != (*b).baseHdrHeadroom.n
        || (*a).baseHdrHeadroom.d != (*b).baseHdrHeadroom.d
        || (*a).alternateHdrHeadroom.n != (*b).alternateHdrHeadroom.n
        || (*a).alternateHdrHeadroom.d != (*b).alternateHdrHeadroom.d
    {
        return AVIF_FALSE;
    }
    let mut c: core::ffi::c_int = 0 as core::ffi::c_int;
    while c < 3 as core::ffi::c_int {
        if (*a).gainMapMin[c as usize].n != (*b).gainMapMin[c as usize].n
            || (*a).gainMapMin[c as usize].d != (*b).gainMapMin[c as usize].d
            || (*a).gainMapMax[c as usize].n != (*b).gainMapMax[c as usize].n
            || (*a).gainMapMax[c as usize].d != (*b).gainMapMax[c as usize].d
            || (*a).gainMapGamma[c as usize].n != (*b).gainMapGamma[c as usize].n
            || (*a).gainMapGamma[c as usize].d != (*b).gainMapGamma[c as usize].d
            || (*a).baseOffset[c as usize].n != (*b).baseOffset[c as usize].n
            || (*a).baseOffset[c as usize].d != (*b).baseOffset[c as usize].d
            || (*a).alternateOffset[c as usize].n != (*b).alternateOffset[c as usize].n
            || (*a).alternateOffset[c as usize].d != (*b).alternateOffset[c as usize].d
        {
            return AVIF_FALSE;
        }
        c += 1;
    }
    return AVIF_TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn avifSameGainMapAltMetadata(
    mut a: *const avifGainMap,
    mut b: *const avifGainMap,
) -> avifBool {
    if (*a).altICC.size != (*b).altICC.size
        || (*a).altICC.size > 0 as size_t
            && memcmp(
                (*a).altICC.data as *const core::ffi::c_void,
                (*b).altICC.data as *const core::ffi::c_void,
                (*a).altICC.size,
            ) != 0 as core::ffi::c_int
        || (*a).altColorPrimaries as core::ffi::c_int
            != (*b).altColorPrimaries as core::ffi::c_int
        || (*a).altTransferCharacteristics as core::ffi::c_int
            != (*b).altTransferCharacteristics as core::ffi::c_int
        || (*a).altMatrixCoefficients as core::ffi::c_int
            != (*b).altMatrixCoefficients as core::ffi::c_int
        || (*a).altYUVRange as core::ffi::c_uint != (*b).altYUVRange as core::ffi::c_uint
        || (*a).altDepth != (*b).altDepth || (*a).altPlaneCount != (*b).altPlaneCount
        || (*a).altCLLI.maxCLL as core::ffi::c_int
            != (*b).altCLLI.maxCLL as core::ffi::c_int
        || (*a).altCLLI.maxPALL as core::ffi::c_int
            != (*b).altCLLI.maxPALL as core::ffi::c_int
    {
        return AVIF_FALSE;
    }
    return AVIF_TRUE;
}
static mut kEpsilon: core::ffi::c_float = 1e-10f32;
unsafe extern "C" fn avifChooseColorSpaceForGainMapMath(
    mut basePrimaries: avifColorPrimaries,
    mut altPrimaries: avifColorPrimaries,
    mut gainMapMathColorSpace: *mut avifColorPrimaries,
) -> avifResult {
    if basePrimaries as core::ffi::c_int == altPrimaries as core::ffi::c_int {
        *gainMapMathColorSpace = basePrimaries;
        return AVIF_RESULT_OK;
    }
    let mut rgba: [core::ffi::c_float; 4] = [
        0 as core::ffi::c_int as core::ffi::c_float,
        0.,
        0.,
        0.,
    ];
    let mut baseToAltCoeffs: [[core::ffi::c_double; 3]; 3] = [[0.; 3]; 3];
    let mut altToBaseCoeffs: [[core::ffi::c_double; 3]; 3] = [[0.; 3]; 3];
    if avifColorPrimariesComputeRGBToRGBMatrix(
        basePrimaries,
        altPrimaries,
        baseToAltCoeffs.as_mut_ptr(),
    ) == 0
        || avifColorPrimariesComputeRGBToRGBMatrix(
            altPrimaries,
            basePrimaries,
            altToBaseCoeffs.as_mut_ptr(),
        ) == 0
    {
        return AVIF_RESULT_NOT_IMPLEMENTED;
    }
    let mut baseColorspaceChannelMin: core::ffi::c_float = 0 as core::ffi::c_int
        as core::ffi::c_float;
    let mut altColorspaceChannelMin: core::ffi::c_float = 0 as core::ffi::c_int
        as core::ffi::c_float;
    let mut c: core::ffi::c_int = 0 as core::ffi::c_int;
    while c < 3 as core::ffi::c_int {
        rgba[2 as core::ffi::c_int as usize] = 0 as core::ffi::c_int
            as core::ffi::c_float;
        rgba[1 as core::ffi::c_int as usize] = rgba[2 as core::ffi::c_int as usize];
        rgba[0 as core::ffi::c_int as usize] = rgba[1 as core::ffi::c_int as usize];
        rgba[c as usize] = 1.0f32;
        avifLinearRGBConvertColorSpace(rgba.as_mut_ptr(), altToBaseCoeffs.as_mut_ptr());
        let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
        while i < 3 as core::ffi::c_int {
            baseColorspaceChannelMin = if baseColorspaceChannelMin < rgba[i as usize] {
                baseColorspaceChannelMin
            } else {
                rgba[i as usize]
            };
            i += 1;
        }
        rgba[2 as core::ffi::c_int as usize] = 0 as core::ffi::c_int
            as core::ffi::c_float;
        rgba[1 as core::ffi::c_int as usize] = rgba[2 as core::ffi::c_int as usize];
        rgba[0 as core::ffi::c_int as usize] = rgba[1 as core::ffi::c_int as usize];
        rgba[c as usize] = 1.0f32;
        avifLinearRGBConvertColorSpace(rgba.as_mut_ptr(), baseToAltCoeffs.as_mut_ptr());
        let mut i_0: core::ffi::c_int = 0 as core::ffi::c_int;
        while i_0 < 3 as core::ffi::c_int {
            altColorspaceChannelMin = if altColorspaceChannelMin < rgba[i_0 as usize] {
                altColorspaceChannelMin
            } else {
                rgba[i_0 as usize]
            };
            i_0 += 1;
        }
        c += 1;
    }
    *gainMapMathColorSpace = (if altColorspaceChannelMin <= baseColorspaceChannelMin {
        basePrimaries as core::ffi::c_int
    } else {
        altPrimaries as core::ffi::c_int
    }) as avifColorPrimaries;
    return AVIF_RESULT_OK;
}
#[no_mangle]
pub unsafe extern "C" fn avifRGBImageComputeGainMap(
    mut baseRgbImage: *const avifRGBImage,
    mut baseColorPrimaries: avifColorPrimaries,
    mut baseTransferCharacteristics: avifTransferCharacteristics,
    mut altRgbImage: *const avifRGBImage,
    mut altColorPrimaries: avifColorPrimaries,
    mut altTransferCharacteristics: avifTransferCharacteristics,
    mut gainMap: *mut avifGainMap,
    mut diag: *mut avifDiagnostics,
) -> avifResult {
    let mut gainMapChannelSize: size_t = 0;
    let mut singleChannel: avifBool = 0;
    let mut numGainMapChannels: core::ffi::c_int = 0;
    let mut baseGammaToLinear: Option<
        unsafe extern "C" fn(core::ffi::c_float) -> core::ffi::c_float,
    > = None;
    let mut altGammaToLinear: Option<
        unsafe extern "C" fn(core::ffi::c_float) -> core::ffi::c_float,
    > = None;
    let mut yCoeffs: [core::ffi::c_float; 3] = [0.; 3];
    let mut rgbConversionCoeffs: [[core::ffi::c_double; 3]; 3] = [[0.; 3]; 3];
    let mut baseOffset: [core::ffi::c_float; 3] = [0.; 3];
    let mut alternateOffset: [core::ffi::c_float; 3] = [0.; 3];
    let mut baseMax: core::ffi::c_float = 0.;
    let mut altMax: core::ffi::c_float = 0.;
    let mut baseHeadroom: core::ffi::c_double = 0.;
    let mut alternateHeadroom: core::ffi::c_double = 0.;
    let mut gainMapMinLog2: [core::ffi::c_float; 3] = [0.; 3];
    let mut gainMapMaxLog2: [core::ffi::c_float; 3] = [0.; 3];
    let mut requestedWidth: uint32_t = 0;
    let mut requestedHeight: uint32_t = 0;
    let mut gainMapRGBInfo: avifRGBColorSpaceInfo = avifRGBColorSpaceInfo {
        channelBytes: 0,
        pixelBytes: 0,
        offsetBytesR: 0,
        offsetBytesG: 0,
        offsetBytesB: 0,
        offsetBytesA: 0,
        offsetBytesGray: 0,
        maxChannel: 0,
        maxChannelF: 0.,
    };
    let mut current_block: u64;
    avifDiagnosticsClearError(diag);
    if !(!baseRgbImage.is_null() && !altRgbImage.is_null() && !gainMap.is_null()
        && !((*gainMap).image).is_null())
    {
        return AVIF_RESULT_INVALID_ARGUMENT;
    }
    if (*baseRgbImage).width != (*altRgbImage).width
        || (*baseRgbImage).height != (*altRgbImage).height
    {
        avifDiagnosticsPrintf(
            diag,
            b"Both images should have the same dimensions\0" as *const u8
                as *const core::ffi::c_char,
        );
        return AVIF_RESULT_INVALID_ARGUMENT;
    }
    if (*(*gainMap).image).width == 0 as uint32_t
        || (*(*gainMap).image).height == 0 as uint32_t
        || (*(*gainMap).image).depth == 0 as uint32_t
        || (*(*gainMap).image).yuvFormat as core::ffi::c_uint
            <= AVIF_PIXEL_FORMAT_NONE as core::ffi::c_int as core::ffi::c_uint
        || (*(*gainMap).image).yuvFormat as core::ffi::c_uint
            >= AVIF_PIXEL_FORMAT_COUNT as core::ffi::c_int as core::ffi::c_uint
    {
        avifDiagnosticsPrintf(
            diag,
            b"gainMap->image should be non null with desired width, height, depth and yuvFormat set\0"
                as *const u8 as *const core::ffi::c_char,
        );
        return AVIF_RESULT_INVALID_ARGUMENT;
    }
    let colorSpacesDiffer: avifBool = (baseColorPrimaries as core::ffi::c_int
        != altColorPrimaries as core::ffi::c_int) as core::ffi::c_int;
    let mut gainMapMathPrimaries: avifColorPrimaries = 0;
    let result__: avifResult = avifChooseColorSpaceForGainMapMath(
        baseColorPrimaries,
        altColorPrimaries,
        &mut gainMapMathPrimaries,
    ) as avifResult;
    if result__ as core::ffi::c_uint
        != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
    {
        return result__;
    }
    let width: uint32_t = (*baseRgbImage).width;
    let height: uint32_t = (*baseRgbImage).height;
    let mut baseRGBInfo: avifRGBColorSpaceInfo = avifRGBColorSpaceInfo {
        channelBytes: 0,
        pixelBytes: 0,
        offsetBytesR: 0,
        offsetBytesG: 0,
        offsetBytesB: 0,
        offsetBytesA: 0,
        offsetBytesGray: 0,
        maxChannel: 0,
        maxChannelF: 0.,
    };
    let mut altRGBInfo: avifRGBColorSpaceInfo = avifRGBColorSpaceInfo {
        channelBytes: 0,
        pixelBytes: 0,
        offsetBytesR: 0,
        offsetBytesG: 0,
        offsetBytesB: 0,
        offsetBytesA: 0,
        offsetBytesGray: 0,
        maxChannel: 0,
        maxChannelF: 0.,
    };
    if avifGetRGBColorSpaceInfo(baseRgbImage, &mut baseRGBInfo) == 0
        || avifGetRGBColorSpaceInfo(altRgbImage, &mut altRGBInfo) == 0
    {
        avifDiagnosticsPrintf(
            diag,
            b"Unsupported RGB color space\0" as *const u8 as *const core::ffi::c_char,
        );
        return AVIF_RESULT_NOT_IMPLEMENTED;
    }
    let mut gainMapF: [*mut core::ffi::c_float; 3] = [
        0 as *mut core::ffi::c_float,
        0 as *mut core::ffi::c_float,
        0 as *mut core::ffi::c_float,
    ];
    let mut gainMapRGB: avifRGBImage = avifRGBImage {
        width: 0,
        height: 0,
        depth: 0,
        format: AVIF_RGB_FORMAT_RGB,
        chromaUpsampling: AVIF_CHROMA_UPSAMPLING_AUTOMATIC,
        chromaDownsampling: AVIF_CHROMA_DOWNSAMPLING_AUTOMATIC,
        avoidLibYUV: 0,
        ignoreAlpha: 0,
        alphaPremultiplied: 0,
        isFloat: 0,
        maxThreads: 0,
        pixels: 0 as *mut uint8_t,
        rowBytes: 0,
    };
    memset(
        &mut gainMapRGB as *mut avifRGBImage as *mut core::ffi::c_void,
        0 as core::ffi::c_int,
        ::core::mem::size_of::<avifRGBImage>() as size_t,
    );
    let mut gainMapImage: *mut avifImage = (*gainMap).image as *mut avifImage;
    let mut res: avifResult = AVIF_RESULT_OK;
    let numPixels: size_t = (width as size_t).wrapping_mul(height as size_t);
    if numPixels
        > (SIZE_MAX as usize)
            .wrapping_div(::core::mem::size_of::<core::ffi::c_float>() as usize)
    {
        res = AVIF_RESULT_INVALID_ARGUMENT;
    } else {
        gainMapChannelSize = numPixels
            .wrapping_mul(::core::mem::size_of::<core::ffi::c_float>() as size_t);
        singleChannel = ((*(*gainMap).image).yuvFormat as core::ffi::c_uint
            == AVIF_PIXEL_FORMAT_YUV400 as core::ffi::c_int as core::ffi::c_uint)
            as core::ffi::c_int;
        numGainMapChannels = if singleChannel != 0 {
            1 as core::ffi::c_int
        } else {
            3 as core::ffi::c_int
        };
        let mut c: core::ffi::c_int = 0 as core::ffi::c_int;
        loop {
            if !(c < numGainMapChannels) {
                current_block = 3;
                break;
            }
            gainMapF[c as usize] = avifAlloc(gainMapChannelSize)
                as *mut core::ffi::c_float;
            if (gainMapF[c as usize]).is_null() {
                res = AVIF_RESULT_OUT_OF_MEMORY;
                current_block = 4;
                break;
            } else {
                c += 1;
            }
        }
        match current_block {
            4 => {}
            _ => {
                avifGainMapSetEncodingDefaults(gainMap);
                (*gainMap).useBaseColorSpace = (gainMapMathPrimaries as core::ffi::c_int
                    == baseColorPrimaries as core::ffi::c_int) as core::ffi::c_int
                    as avifBool;
                baseGammaToLinear = avifTransferCharacteristicsGetGammaToLinearFunction(
                    baseTransferCharacteristics,
                )
                    as Option<
                        unsafe extern "C" fn(core::ffi::c_float) -> core::ffi::c_float,
                    >;
                altGammaToLinear = avifTransferCharacteristicsGetGammaToLinearFunction(
                    altTransferCharacteristics,
                )
                    as Option<
                        unsafe extern "C" fn(core::ffi::c_float) -> core::ffi::c_float,
                    >;
                yCoeffs = [0.; 3];
                avifColorPrimariesComputeYCoeffs(
                    gainMapMathPrimaries,
                    yCoeffs.as_mut_ptr(),
                );
                rgbConversionCoeffs = [[0.; 3]; 3];
                if colorSpacesDiffer != 0 {
                    if (*gainMap).useBaseColorSpace != 0 {
                        if avifColorPrimariesComputeRGBToRGBMatrix(
                            altColorPrimaries,
                            baseColorPrimaries,
                            rgbConversionCoeffs.as_mut_ptr(),
                        ) == 0
                        {
                            avifDiagnosticsPrintf(
                                diag,
                                b"Unsupported RGB color space conversion\0" as *const u8
                                    as *const core::ffi::c_char,
                            );
                            res = AVIF_RESULT_NOT_IMPLEMENTED;
                            current_block = 4;
                        } else {
                            current_block = 5;
                        }
                    } else if avifColorPrimariesComputeRGBToRGBMatrix(
                        baseColorPrimaries,
                        altColorPrimaries,
                        rgbConversionCoeffs.as_mut_ptr(),
                    ) == 0
                    {
                        avifDiagnosticsPrintf(
                            diag,
                            b"Unsupported RGB color space conversion\0" as *const u8
                                as *const core::ffi::c_char,
                        );
                        res = AVIF_RESULT_NOT_IMPLEMENTED;
                        current_block = 4;
                    } else {
                        current_block = 5;
                    }
                } else {
                    current_block = 5;
                }
                match current_block {
                    4 => {}
                    _ => {
                        baseOffset = [
                            avifSignedFractionToFloat(
                                (*gainMap).baseOffset[0 as core::ffi::c_int as usize],
                            ),
                            avifSignedFractionToFloat(
                                (*gainMap).baseOffset[1 as core::ffi::c_int as usize],
                            ),
                            avifSignedFractionToFloat(
                                (*gainMap).baseOffset[2 as core::ffi::c_int as usize],
                            ),
                        ];
                        alternateOffset = [
                            avifSignedFractionToFloat(
                                (*gainMap).alternateOffset[0 as core::ffi::c_int as usize],
                            ),
                            avifSignedFractionToFloat(
                                (*gainMap).alternateOffset[1 as core::ffi::c_int as usize],
                            ),
                            avifSignedFractionToFloat(
                                (*gainMap).alternateOffset[2 as core::ffi::c_int as usize],
                            ),
                        ];
                        if colorSpacesDiffer != 0 {
                            let mut rgba: [core::ffi::c_float; 4] = [0.0f32, 0., 0., 0.];
                            let mut channelMin: [core::ffi::c_float; 3] = [
                                0.0f32,
                                0.,
                                0.,
                            ];
                            let mut j: uint32_t = 0 as uint32_t;
                            while j < height {
                                let mut i: uint32_t = 0 as uint32_t;
                                while i < width {
                                    avifGetRGBAPixel(
                                        if (*gainMap).useBaseColorSpace != 0 {
                                            altRgbImage
                                        } else {
                                            baseRgbImage
                                        },
                                        i,
                                        j,
                                        if (*gainMap).useBaseColorSpace != 0 {
                                            &mut altRGBInfo
                                        } else {
                                            &mut baseRGBInfo
                                        },
                                        rgba.as_mut_ptr(),
                                    );
                                    let mut c_0: core::ffi::c_int = 0 as core::ffi::c_int;
                                    while c_0 < 3 as core::ffi::c_int {
                                        if (*gainMap).useBaseColorSpace != 0 {
                                            rgba[c_0 as usize] = altGammaToLinear
                                                .expect("non-null function pointer")(rgba[c_0 as usize]);
                                        } else {
                                            rgba[c_0 as usize] = baseGammaToLinear
                                                .expect("non-null function pointer")(rgba[c_0 as usize]);
                                        }
                                        c_0 += 1;
                                    }
                                    avifLinearRGBConvertColorSpace(
                                        rgba.as_mut_ptr(),
                                        rgbConversionCoeffs.as_mut_ptr(),
                                    );
                                    let mut c_1: core::ffi::c_int = 0 as core::ffi::c_int;
                                    while c_1 < 3 as core::ffi::c_int {
                                        channelMin[c_1 as usize] = if channelMin[c_1 as usize]
                                            < rgba[c_1 as usize]
                                        {
                                            channelMin[c_1 as usize]
                                        } else {
                                            rgba[c_1 as usize]
                                        };
                                        c_1 += 1;
                                    }
                                    i = i.wrapping_add(1);
                                }
                                j = j.wrapping_add(1);
                            }
                            let mut c_2: core::ffi::c_int = 0 as core::ffi::c_int;
                            while c_2 < 3 as core::ffi::c_int {
                                let maxOffset: core::ffi::c_float = 0.1f32;
                                if channelMin[c_2 as usize] < -kEpsilon {
                                    if (*gainMap).useBaseColorSpace != 0 {
                                        alternateOffset[c_2 as usize] = if alternateOffset[c_2
                                            as usize] - channelMin[c_2 as usize] < maxOffset
                                        {
                                            alternateOffset[c_2 as usize] - channelMin[c_2 as usize]
                                        } else {
                                            maxOffset
                                        };
                                    } else {
                                        baseOffset[c_2 as usize] = if baseOffset[c_2 as usize]
                                            - channelMin[c_2 as usize] < maxOffset
                                        {
                                            baseOffset[c_2 as usize] - channelMin[c_2 as usize]
                                        } else {
                                            maxOffset
                                        };
                                    }
                                }
                                c_2 += 1;
                            }
                        }
                        baseMax = 1.0f32;
                        altMax = 1.0f32;
                        let mut j_0: uint32_t = 0 as uint32_t;
                        while j_0 < height {
                            let mut i_0: uint32_t = 0 as uint32_t;
                            while i_0 < width {
                                let mut baseRGBA: [core::ffi::c_float; 4] = [0.; 4];
                                avifGetRGBAPixel(
                                    baseRgbImage,
                                    i_0,
                                    j_0,
                                    &mut baseRGBInfo,
                                    baseRGBA.as_mut_ptr(),
                                );
                                let mut altRGBA: [core::ffi::c_float; 4] = [0.; 4];
                                avifGetRGBAPixel(
                                    altRgbImage,
                                    i_0,
                                    j_0,
                                    &mut altRGBInfo,
                                    altRGBA.as_mut_ptr(),
                                );
                                let mut c_3: core::ffi::c_int = 0 as core::ffi::c_int;
                                while c_3 < 3 as core::ffi::c_int {
                                    baseRGBA[c_3 as usize] = baseGammaToLinear
                                        .expect(
                                            "non-null function pointer",
                                        )(baseRGBA[c_3 as usize]);
                                    altRGBA[c_3 as usize] = altGammaToLinear
                                        .expect("non-null function pointer")(altRGBA[c_3 as usize]);
                                    c_3 += 1;
                                }
                                if colorSpacesDiffer != 0 {
                                    if (*gainMap).useBaseColorSpace != 0 {
                                        avifLinearRGBConvertColorSpace(
                                            altRGBA.as_mut_ptr(),
                                            rgbConversionCoeffs.as_mut_ptr(),
                                        );
                                    } else {
                                        avifLinearRGBConvertColorSpace(
                                            baseRGBA.as_mut_ptr(),
                                            rgbConversionCoeffs.as_mut_ptr(),
                                        );
                                    }
                                }
                                let mut c_4: core::ffi::c_int = 0 as core::ffi::c_int;
                                while c_4 < numGainMapChannels {
                                    let mut base: core::ffi::c_float = baseRGBA[c_4 as usize];
                                    let mut alt: core::ffi::c_float = altRGBA[c_4 as usize];
                                    if singleChannel != 0 {
                                        base = yCoeffs[0 as core::ffi::c_int as usize]
                                            * baseRGBA[0 as core::ffi::c_int as usize]
                                            + yCoeffs[1 as core::ffi::c_int as usize]
                                                * baseRGBA[1 as core::ffi::c_int as usize]
                                            + yCoeffs[2 as core::ffi::c_int as usize]
                                                * baseRGBA[2 as core::ffi::c_int as usize];
                                        alt = yCoeffs[0 as core::ffi::c_int as usize]
                                            * altRGBA[0 as core::ffi::c_int as usize]
                                            + yCoeffs[1 as core::ffi::c_int as usize]
                                                * altRGBA[1 as core::ffi::c_int as usize]
                                            + yCoeffs[2 as core::ffi::c_int as usize]
                                                * altRGBA[2 as core::ffi::c_int as usize];
                                    }
                                    if base > baseMax {
                                        baseMax = base;
                                    }
                                    if alt > altMax {
                                        altMax = alt;
                                    }
                                    let ratio: core::ffi::c_float = (alt
                                        + alternateOffset[c_4 as usize])
                                        / (base + baseOffset[c_4 as usize]);
                                    let ratioLog2: core::ffi::c_float = log2f(
                                        if ratio > kEpsilon { ratio } else { kEpsilon },
                                    ) as core::ffi::c_float;
                                    *(gainMapF[c_4 as usize])
                                        .offset(
                                            (j_0 as size_t)
                                                .wrapping_mul(width as size_t)
                                                .wrapping_add(i_0 as size_t) as isize,
                                        ) = ratioLog2;
                                    c_4 += 1;
                                }
                                i_0 = i_0.wrapping_add(1);
                            }
                            j_0 = j_0.wrapping_add(1);
                        }
                        baseHeadroom = log2f(
                            if baseMax > kEpsilon { baseMax } else { kEpsilon },
                        ) as core::ffi::c_double;
                        alternateHeadroom = log2f(
                            if altMax > kEpsilon { altMax } else { kEpsilon },
                        ) as core::ffi::c_double;
                        if avifDoubleToUnsignedFraction(
                            baseHeadroom,
                            &mut (*gainMap).baseHdrHeadroom,
                        ) == 0
                            || avifDoubleToUnsignedFraction(
                                alternateHeadroom,
                                &mut (*gainMap).alternateHdrHeadroom,
                            ) == 0
                        {
                            res = AVIF_RESULT_INVALID_ARGUMENT;
                        } else {
                            if alternateHeadroom < baseHeadroom {
                                let mut c_5: core::ffi::c_int = 0 as core::ffi::c_int;
                                while c_5 < numGainMapChannels {
                                    let mut j_1: uint32_t = 0 as uint32_t;
                                    while j_1 < height {
                                        let mut i_1: uint32_t = 0 as uint32_t;
                                        while i_1 < width {
                                            *(gainMapF[c_5 as usize])
                                                .offset(
                                                    (j_1 as size_t)
                                                        .wrapping_mul(width as size_t)
                                                        .wrapping_add(i_1 as size_t) as isize,
                                                ) *= -1.0f32;
                                            i_1 = i_1.wrapping_add(1);
                                        }
                                        j_1 = j_1.wrapping_add(1);
                                    }
                                    c_5 += 1;
                                }
                            }
                            gainMapMinLog2 = [0.0f32, 0.0f32, 0.0f32];
                            gainMapMaxLog2 = [0.0f32, 0.0f32, 0.0f32];
                            let mut c_6: core::ffi::c_int = 0 as core::ffi::c_int;
                            loop {
                                if !(c_6 < numGainMapChannels) {
                                    current_block = 6;
                                    break;
                                }
                                res = avifFindMinMaxWithoutOutliers(
                                    gainMapF[c_6 as usize],
                                    numPixels,
                                    &mut *gainMapMinLog2.as_mut_ptr().offset(c_6 as isize),
                                    &mut *gainMapMaxLog2.as_mut_ptr().offset(c_6 as isize),
                                );
                                if res as core::ffi::c_uint
                                    != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
                                {
                                    current_block = 4;
                                    break;
                                }
                                c_6 += 1;
                            }
                            match current_block {
                                4 => {}
                                _ => {
                                    let mut c_7: core::ffi::c_int = 0 as core::ffi::c_int;
                                    loop {
                                        if !(c_7 < 3 as core::ffi::c_int) {
                                            current_block = 7;
                                            break;
                                        }
                                        if avifDoubleToSignedFraction(
                                            gainMapMinLog2[(if singleChannel != 0 {
                                                0 as core::ffi::c_int
                                            } else {
                                                c_7
                                            }) as usize] as core::ffi::c_double,
                                            &mut *((*gainMap).gainMapMin)
                                                .as_mut_ptr()
                                                .offset(c_7 as isize),
                                        ) == 0
                                            || avifDoubleToSignedFraction(
                                                gainMapMaxLog2[(if singleChannel != 0 {
                                                    0 as core::ffi::c_int
                                                } else {
                                                    c_7
                                                }) as usize] as core::ffi::c_double,
                                                &mut *((*gainMap).gainMapMax)
                                                    .as_mut_ptr()
                                                    .offset(c_7 as isize),
                                            ) == 0
                                            || avifDoubleToSignedFraction(
                                                alternateOffset[c_7 as usize] as core::ffi::c_double,
                                                &mut *((*gainMap).alternateOffset)
                                                    .as_mut_ptr()
                                                    .offset(c_7 as isize),
                                            ) == 0
                                            || avifDoubleToSignedFraction(
                                                baseOffset[c_7 as usize] as core::ffi::c_double,
                                                &mut *((*gainMap).baseOffset)
                                                    .as_mut_ptr()
                                                    .offset(c_7 as isize),
                                            ) == 0
                                        {
                                            res = AVIF_RESULT_INVALID_ARGUMENT;
                                            current_block = 4;
                                            break;
                                        } else {
                                            c_7 += 1;
                                        }
                                    }
                                    match current_block {
                                        4 => {}
                                        _ => {
                                            let mut c_8: core::ffi::c_int = 0 as core::ffi::c_int;
                                            while c_8 < numGainMapChannels {
                                                let range: core::ffi::c_float = if gainMapMaxLog2[c_8
                                                    as usize] - gainMapMinLog2[c_8 as usize] > 0.0f32
                                                {
                                                    gainMapMaxLog2[c_8 as usize] - gainMapMinLog2[c_8 as usize]
                                                } else {
                                                    0.0f32
                                                };
                                                if range == 0.0f32 {
                                                    let mut j_2: uint32_t = 0 as uint32_t;
                                                    while j_2 < height {
                                                        let mut i_2: uint32_t = 0 as uint32_t;
                                                        while i_2 < width {
                                                            *(gainMapF[c_8 as usize])
                                                                .offset(
                                                                    (j_2 as size_t)
                                                                        .wrapping_mul(width as size_t)
                                                                        .wrapping_add(i_2 as size_t) as isize,
                                                                ) = 0.0f32;
                                                            i_2 = i_2.wrapping_add(1);
                                                        }
                                                        j_2 = j_2.wrapping_add(1);
                                                    }
                                                } else {
                                                    let gainMapGamma: core::ffi::c_float = avifUnsignedFractionToFloat(
                                                        (*gainMap).gainMapGamma[c_8 as usize],
                                                    ) as core::ffi::c_float;
                                                    let mut j_3: uint32_t = 0 as uint32_t;
                                                    while j_3 < height {
                                                        let mut i_3: uint32_t = 0 as uint32_t;
                                                        while i_3 < width {
                                                            let mut v: core::ffi::c_float = *(gainMapF[c_8 as usize])
                                                                .offset(
                                                                    (j_3 as size_t)
                                                                        .wrapping_mul(width as size_t)
                                                                        .wrapping_add(i_3 as size_t) as isize,
                                                                );
                                                            v = if v < gainMapMinLog2[c_8 as usize] {
                                                                gainMapMinLog2[c_8 as usize]
                                                            } else if gainMapMaxLog2[c_8 as usize] < v {
                                                                gainMapMaxLog2[c_8 as usize]
                                                            } else {
                                                                v
                                                            };
                                                            v = powf(
                                                                (v - gainMapMinLog2[c_8 as usize]) / range,
                                                                gainMapGamma,
                                                            );
                                                            *(gainMapF[c_8 as usize])
                                                                .offset(
                                                                    (j_3 as size_t)
                                                                        .wrapping_mul(width as size_t)
                                                                        .wrapping_add(i_3 as size_t) as isize,
                                                                ) = avifNanSafeClamp(v);
                                                            i_3 = i_3.wrapping_add(1);
                                                        }
                                                        j_3 = j_3.wrapping_add(1);
                                                    }
                                                }
                                                c_8 += 1;
                                            }
                                            requestedWidth = (*gainMapImage).width;
                                            requestedHeight = (*gainMapImage).height;
                                            (*gainMapImage).width = width;
                                            (*gainMapImage).height = height;
                                            avifImageFreePlanes(
                                                gainMapImage,
                                                AVIF_PLANES_ALL as core::ffi::c_int as avifPlanesFlags,
                                            );
                                            avifRGBImageSetDefaults(&mut gainMapRGB, gainMapImage);
                                            res = avifRGBImageAllocatePixels(&mut gainMapRGB);
                                            if !(res as core::ffi::c_uint
                                                != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint)
                                            {
                                                gainMapRGBInfo = avifRGBColorSpaceInfo {
                                                    channelBytes: 0,
                                                    pixelBytes: 0,
                                                    offsetBytesR: 0,
                                                    offsetBytesG: 0,
                                                    offsetBytesB: 0,
                                                    offsetBytesA: 0,
                                                    offsetBytesGray: 0,
                                                    maxChannel: 0,
                                                    maxChannelF: 0.,
                                                };
                                                if avifGetRGBColorSpaceInfo(
                                                    &mut gainMapRGB,
                                                    &mut gainMapRGBInfo,
                                                ) == 0
                                                {
                                                    avifDiagnosticsPrintf(
                                                        diag,
                                                        b"Unsupported RGB color space\0" as *const u8
                                                            as *const core::ffi::c_char,
                                                    );
                                                    res = AVIF_RESULT_NOT_IMPLEMENTED;
                                                } else {
                                                    let mut j_4: uint32_t = 0 as uint32_t;
                                                    while j_4 < height {
                                                        let mut i_4: uint32_t = 0 as uint32_t;
                                                        while i_4 < width {
                                                            let offset: size_t = (j_4 as size_t)
                                                                .wrapping_mul(width as size_t)
                                                                .wrapping_add(i_4 as size_t);
                                                            let r: core::ffi::c_float = *(gainMapF[0 as core::ffi::c_int
                                                                as usize])
                                                                .offset(offset as isize);
                                                            let g: core::ffi::c_float = if singleChannel != 0 {
                                                                r
                                                            } else {
                                                                *(gainMapF[1 as core::ffi::c_int as usize])
                                                                    .offset(offset as isize)
                                                            };
                                                            let b: core::ffi::c_float = if singleChannel != 0 {
                                                                r
                                                            } else {
                                                                *(gainMapF[2 as core::ffi::c_int as usize])
                                                                    .offset(offset as isize)
                                                            };
                                                            let rgbaPixel: [core::ffi::c_float; 4] = [r, g, b, 1.0f32];
                                                            avifSetRGBAPixel(
                                                                &mut gainMapRGB,
                                                                i_4,
                                                                j_4,
                                                                &mut gainMapRGBInfo,
                                                                rgbaPixel.as_ptr(),
                                                            );
                                                            i_4 = i_4.wrapping_add(1);
                                                        }
                                                        j_4 = j_4.wrapping_add(1);
                                                    }
                                                    res = avifImageRGBToYUV(gainMapImage, &mut gainMapRGB);
                                                    if !(res as core::ffi::c_uint
                                                        != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint)
                                                    {
                                                        if requestedWidth != (*gainMapImage).width
                                                            || requestedHeight != (*gainMapImage).height
                                                        {
                                                            res = avifImageScale(
                                                                (*gainMap).image as *mut avifImage,
                                                                requestedWidth,
                                                                requestedHeight,
                                                                diag,
                                                            );
                                                            res as core::ffi::c_uint
                                                                != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint;
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
    let mut c_9: core::ffi::c_int = 0 as core::ffi::c_int;
    while c_9 < 3 as core::ffi::c_int {
        avifFree(gainMapF[c_9 as usize] as *mut core::ffi::c_void);
        c_9 += 1;
    }
    avifRGBImageFreePixels(&mut gainMapRGB);
    if res as core::ffi::c_uint
        != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
    {
        avifImageFreePlanes(
            gainMapImage,
            AVIF_PLANES_ALL as core::ffi::c_int as avifPlanesFlags,
        );
    }
    return res;
}
#[no_mangle]
pub unsafe extern "C" fn avifImageComputeGainMap(
    mut baseImage: *const avifImage,
    mut altImage: *const avifImage,
    mut gainMap: *mut avifGainMap,
    mut diag: *mut avifDiagnostics,
) -> avifResult {
    avifDiagnosticsClearError(diag);
    if baseImage.is_null() || altImage.is_null() || gainMap.is_null() {
        return AVIF_RESULT_INVALID_ARGUMENT;
    }
    if (*baseImage).icc.size > 0 as size_t || (*altImage).icc.size > 0 as size_t {
        avifDiagnosticsPrintf(
            diag,
            b"Computing gain maps for images with ICC profiles is not supported\0"
                as *const u8 as *const core::ffi::c_char,
        );
        return AVIF_RESULT_NOT_IMPLEMENTED;
    }
    if (*baseImage).width != (*altImage).width
        || (*baseImage).height != (*altImage).height
    {
        avifDiagnosticsPrintf(
            diag,
            b"Image dimensions don't match, got %dx%d and %dx%d\0" as *const u8
                as *const core::ffi::c_char,
            (*baseImage).width,
            (*baseImage).height,
            (*altImage).width,
            (*altImage).height,
        );
        return AVIF_RESULT_INVALID_ARGUMENT;
    }
    let mut res: avifResult = AVIF_RESULT_OK;
    let mut baseImageRgb: avifRGBImage = avifRGBImage {
        width: 0,
        height: 0,
        depth: 0,
        format: AVIF_RGB_FORMAT_RGB,
        chromaUpsampling: AVIF_CHROMA_UPSAMPLING_AUTOMATIC,
        chromaDownsampling: AVIF_CHROMA_DOWNSAMPLING_AUTOMATIC,
        avoidLibYUV: 0,
        ignoreAlpha: 0,
        alphaPremultiplied: 0,
        isFloat: 0,
        maxThreads: 0,
        pixels: 0 as *mut uint8_t,
        rowBytes: 0,
    };
    avifRGBImageSetDefaults(&mut baseImageRgb, baseImage);
    let mut altImageRgb: avifRGBImage = avifRGBImage {
        width: 0,
        height: 0,
        depth: 0,
        format: AVIF_RGB_FORMAT_RGB,
        chromaUpsampling: AVIF_CHROMA_UPSAMPLING_AUTOMATIC,
        chromaDownsampling: AVIF_CHROMA_DOWNSAMPLING_AUTOMATIC,
        avoidLibYUV: 0,
        ignoreAlpha: 0,
        alphaPremultiplied: 0,
        isFloat: 0,
        maxThreads: 0,
        pixels: 0 as *mut uint8_t,
        rowBytes: 0,
    };
    avifRGBImageSetDefaults(&mut altImageRgb, altImage);
    let result__: avifResult = avifRGBImageAllocatePixels(&mut baseImageRgb)
        as avifResult;
    if result__ as core::ffi::c_uint
        != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
    {
        return result__;
    }
    res = avifImageYUVToRGB(baseImage, &mut baseImageRgb);
    if !(res as core::ffi::c_uint
        != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint)
    {
        res = avifRGBImageAllocatePixels(&mut altImageRgb);
        if !(res as core::ffi::c_uint
            != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint)
        {
            res = avifImageYUVToRGB(altImage, &mut altImageRgb);
            if !(res as core::ffi::c_uint
                != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint)
            {
                res = avifRGBImageComputeGainMap(
                    &mut baseImageRgb,
                    (*baseImage).colorPrimaries,
                    (*baseImage).transferCharacteristics,
                    &mut altImageRgb,
                    (*altImage).colorPrimaries,
                    (*altImage).transferCharacteristics,
                    gainMap,
                    diag,
                );
                if !(res as core::ffi::c_uint
                    != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint)
                {
                    let result___0: avifResult = avifRWDataSet(
                        &mut (*gainMap).altICC,
                        (*altImage).icc.data,
                        (*altImage).icc.size,
                    ) as avifResult;
                    if result___0 as core::ffi::c_uint
                        != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
                    {
                        return result___0;
                    }
                    (*gainMap).altColorPrimaries = (*altImage).colorPrimaries;
                    (*gainMap).altTransferCharacteristics = (*altImage)
                        .transferCharacteristics;
                    (*gainMap).altMatrixCoefficients = (*altImage).matrixCoefficients;
                    (*gainMap).altDepth = (*altImage).depth;
                    (*gainMap).altPlaneCount = (if (*altImage).yuvFormat
                        as core::ffi::c_uint
                        == AVIF_PIXEL_FORMAT_YUV400 as core::ffi::c_int
                            as core::ffi::c_uint
                    {
                        1 as core::ffi::c_int
                    } else {
                        3 as core::ffi::c_int
                    }) as uint32_t;
                    (*gainMap).altCLLI = (*altImage).clli;
                }
            }
        }
    }
    avifRGBImageFreePixels(&mut baseImageRgb);
    avifRGBImageFreePixels(&mut altImageRgb);
    return res;
}
