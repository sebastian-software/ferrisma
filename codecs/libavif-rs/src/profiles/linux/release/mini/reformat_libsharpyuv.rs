// Copyright 2022 Google LLC
// SPDX-License-Identifier: BSD-2-Clause

// Mechanically translated from upstream/libavif/src/reformat_libsharpyuv.c.
// C ABI, ownership, pointer validity, and bounds follow the original headers.
extern "C" {
    fn SharpYuvOptionsInitInternal(
        _: *const SharpYuvConversionMatrix,
        _: *mut SharpYuvOptions,
        _: core::ffi::c_int,
    ) -> core::ffi::c_int;
    fn SharpYuvConvertWithOptions(
        r_ptr: *const core::ffi::c_void,
        g_ptr: *const core::ffi::c_void,
        b_ptr: *const core::ffi::c_void,
        rgb_step: core::ffi::c_int,
        rgb_stride: core::ffi::c_int,
        rgb_bit_depth: core::ffi::c_int,
        y_ptr: *mut core::ffi::c_void,
        y_stride: core::ffi::c_int,
        u_ptr: *mut core::ffi::c_void,
        u_stride: core::ffi::c_int,
        v_ptr: *mut core::ffi::c_void,
        v_stride: core::ffi::c_int,
        yuv_bit_depth: core::ffi::c_int,
        width: core::ffi::c_int,
        height: core::ffi::c_int,
        options: *const SharpYuvOptions,
    ) -> core::ffi::c_int;
    fn SharpYuvComputeConversionMatrix(
        yuv_color_space: *const SharpYuvColorSpace,
        matrix: *mut SharpYuvConversionMatrix,
    );
}
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
pub type C2RustUnnamed = core::ffi::c_uint;
pub const AVIF_TRANSFER_CHARACTERISTICS_HLG: C2RustUnnamed = 18;
pub const AVIF_TRANSFER_CHARACTERISTICS_SMPTE428: C2RustUnnamed = 17;
pub const AVIF_TRANSFER_CHARACTERISTICS_SMPTE2084: C2RustUnnamed = 16;
pub const AVIF_TRANSFER_CHARACTERISTICS_PQ: C2RustUnnamed = 16;
pub const AVIF_TRANSFER_CHARACTERISTICS_BT2020_12BIT: C2RustUnnamed = 15;
pub const AVIF_TRANSFER_CHARACTERISTICS_BT2020_10BIT: C2RustUnnamed = 14;
pub const AVIF_TRANSFER_CHARACTERISTICS_SRGB: C2RustUnnamed = 13;
pub const AVIF_TRANSFER_CHARACTERISTICS_BT1361: C2RustUnnamed = 12;
pub const AVIF_TRANSFER_CHARACTERISTICS_IEC61966: C2RustUnnamed = 11;
pub const AVIF_TRANSFER_CHARACTERISTICS_LOG100_SQRT10: C2RustUnnamed = 10;
pub const AVIF_TRANSFER_CHARACTERISTICS_LOG100: C2RustUnnamed = 9;
pub const AVIF_TRANSFER_CHARACTERISTICS_LINEAR: C2RustUnnamed = 8;
pub const AVIF_TRANSFER_CHARACTERISTICS_SMPTE240: C2RustUnnamed = 7;
pub const AVIF_TRANSFER_CHARACTERISTICS_BT601: C2RustUnnamed = 6;
pub const AVIF_TRANSFER_CHARACTERISTICS_BT470BG: C2RustUnnamed = 5;
pub const AVIF_TRANSFER_CHARACTERISTICS_BT470M: C2RustUnnamed = 4;
pub const AVIF_TRANSFER_CHARACTERISTICS_UNSPECIFIED: C2RustUnnamed = 2;
pub const AVIF_TRANSFER_CHARACTERISTICS_BT709: C2RustUnnamed = 1;
pub const AVIF_TRANSFER_CHARACTERISTICS_UNKNOWN: C2RustUnnamed = 0;
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
pub type avifReformatMode = core::ffi::c_uint;
pub const AVIF_REFORMAT_MODE_YCGCO_RO: avifReformatMode = 4;
pub const AVIF_REFORMAT_MODE_YCGCO_RE: avifReformatMode = 3;
pub const AVIF_REFORMAT_MODE_YCGCO: avifReformatMode = 2;
pub const AVIF_REFORMAT_MODE_IDENTITY: avifReformatMode = 1;
pub const AVIF_REFORMAT_MODE_YUV_COEFFICIENTS: avifReformatMode = 0;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct avifReformatState {
    pub rgb: avifRGBColorSpaceInfo,
    pub yuv: avifYUVColorSpaceInfo,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SharpYuvOptions {
    pub yuv_matrix: *const SharpYuvConversionMatrix,
    pub transfer_type: SharpYuvTransferFunctionType,
}
pub type SharpYuvTransferFunctionType = core::ffi::c_uint;
pub const kSharpYuvTransferFunctionNum: SharpYuvTransferFunctionType = 19;
pub const kSharpYuvTransferFunctionHlg: SharpYuvTransferFunctionType = 18;
pub const kSharpYuvTransferFunctionSmpte428: SharpYuvTransferFunctionType = 17;
pub const kSharpYuvTransferFunctionSmpte2084: SharpYuvTransferFunctionType = 16;
pub const kSharpYuvTransferFunctionBt2020_12Bit: SharpYuvTransferFunctionType = 15;
pub const kSharpYuvTransferFunctionBt2020_10Bit: SharpYuvTransferFunctionType = 14;
pub const kSharpYuvTransferFunctionSrgb: SharpYuvTransferFunctionType = 13;
pub const kSharpYuvTransferFunctionBt1361: SharpYuvTransferFunctionType = 12;
pub const kSharpYuvTransferFunctionIec61966: SharpYuvTransferFunctionType = 11;
pub const kSharpYuvTransferFunctionLog100_Sqrt10: SharpYuvTransferFunctionType = 10;
pub const kSharpYuvTransferFunctionLog100: SharpYuvTransferFunctionType = 9;
pub const kSharpYuvTransferFunctionLinear: SharpYuvTransferFunctionType = 8;
pub const kSharpYuvTransferFunctionSmpte240: SharpYuvTransferFunctionType = 7;
pub const kSharpYuvTransferFunctionBt601: SharpYuvTransferFunctionType = 6;
pub const kSharpYuvTransferFunctionBt470Bg: SharpYuvTransferFunctionType = 5;
pub const kSharpYuvTransferFunctionBt470M: SharpYuvTransferFunctionType = 4;
pub const kSharpYuvTransferFunctionBt709: SharpYuvTransferFunctionType = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SharpYuvConversionMatrix {
    pub rgb_to_y: [core::ffi::c_int; 4],
    pub rgb_to_u: [core::ffi::c_int; 4],
    pub rgb_to_v: [core::ffi::c_int; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SharpYuvColorSpace {
    pub kr: core::ffi::c_float,
    pub kb: core::ffi::c_float,
    pub bit_depth: core::ffi::c_int,
    pub range: SharpYuvRange,
}
pub type SharpYuvRange = core::ffi::c_uint;
pub const kSharpYuvRangeLimited: SharpYuvRange = 1;
pub const kSharpYuvRangeFull: SharpYuvRange = 0;
pub const SHARPYUV_VERSION: core::ffi::c_int = (0 as core::ffi::c_int)
    << 24 as core::ffi::c_int | (4 as core::ffi::c_int) << 16 as core::ffi::c_int
    | 2 as core::ffi::c_int;
#[inline]
unsafe extern "C" fn SharpYuvOptionsInit(
    mut yuv_matrix: *const SharpYuvConversionMatrix,
    mut options: *mut SharpYuvOptions,
) -> core::ffi::c_int {
    return SharpYuvOptionsInitInternal(yuv_matrix, options, SHARPYUV_VERSION);
}
#[no_mangle]
pub unsafe extern "C" fn avifImageRGBToYUVLibSharpYUV(
    mut image: *mut avifImage,
    mut rgb: *const avifRGBImage,
    mut state: *const avifReformatState,
) -> avifResult {
    if (*rgb).width > INT_MAX as uint32_t || (*rgb).height > INT_MAX as uint32_t
        || (*rgb).rowBytes > INT_MAX as uint32_t
        || (*image).yuvRowBytes[AVIF_CHAN_Y as core::ffi::c_int as usize]
            > INT_MAX as uint32_t
    {
        return AVIF_RESULT_NOT_IMPLEMENTED;
    }
    let colorSpace: SharpYuvColorSpace = {
        let mut init = SharpYuvColorSpace {
            kr: (*state).yuv.kr,
            kb: (*state).yuv.kb,
            bit_depth: (*image).depth as core::ffi::c_int,
            range: (if (*state).yuv.range as core::ffi::c_uint
                == AVIF_RANGE_LIMITED as core::ffi::c_int as core::ffi::c_uint
            {
                kSharpYuvRangeLimited as core::ffi::c_int
            } else {
                kSharpYuvRangeFull as core::ffi::c_int
            }) as SharpYuvRange,
        };
        init
    };
    let mut matrix: SharpYuvConversionMatrix = SharpYuvConversionMatrix {
        rgb_to_y: [0; 4],
        rgb_to_u: [0; 4],
        rgb_to_v: [0; 4],
    };
    SharpYuvComputeConversionMatrix(&colorSpace, &mut matrix);
    let mut options: SharpYuvOptions = SharpYuvOptions {
        yuv_matrix: 0 as *const SharpYuvConversionMatrix,
        transfer_type: 0 as SharpYuvTransferFunctionType,
    };
    SharpYuvOptionsInit(&mut matrix, &mut options);
    if (*image).transferCharacteristics as core::ffi::c_int
        != AVIF_TRANSFER_CHARACTERISTICS_UNSPECIFIED as core::ffi::c_int
    {
        options.transfer_type = (*image).transferCharacteristics
            as SharpYuvTransferFunctionType;
    }
    let sharpyuvRes: core::ffi::c_int = SharpYuvConvertWithOptions(
        &mut *((*rgb).pixels).offset((*state).rgb.offsetBytesR as isize) as *mut uint8_t
            as *const core::ffi::c_void,
        &mut *((*rgb).pixels).offset((*state).rgb.offsetBytesG as isize) as *mut uint8_t
            as *const core::ffi::c_void,
        &mut *((*rgb).pixels).offset((*state).rgb.offsetBytesB as isize) as *mut uint8_t
            as *const core::ffi::c_void,
        (*state).rgb.pixelBytes as core::ffi::c_int,
        (*rgb).rowBytes as core::ffi::c_int,
        (*rgb).depth as core::ffi::c_int,
        (*image).yuvPlanes[AVIF_CHAN_Y as core::ffi::c_int as usize]
            as *mut core::ffi::c_void,
        (*image).yuvRowBytes[AVIF_CHAN_Y as core::ffi::c_int as usize]
            as core::ffi::c_int,
        (*image).yuvPlanes[AVIF_CHAN_U as core::ffi::c_int as usize]
            as *mut core::ffi::c_void,
        (*image).yuvRowBytes[AVIF_CHAN_U as core::ffi::c_int as usize]
            as core::ffi::c_int,
        (*image).yuvPlanes[AVIF_CHAN_V as core::ffi::c_int as usize]
            as *mut core::ffi::c_void,
        (*image).yuvRowBytes[AVIF_CHAN_V as core::ffi::c_int as usize]
            as core::ffi::c_int,
        (*image).depth as core::ffi::c_int,
        (*rgb).width as core::ffi::c_int,
        (*rgb).height as core::ffi::c_int,
        &mut options,
    ) as core::ffi::c_int;
    if sharpyuvRes == 0 {
        return AVIF_RESULT_REFORMAT_FAILED;
    }
    return AVIF_RESULT_OK;
}
pub const __INT_MAX__: core::ffi::c_int = 2147483647 as core::ffi::c_int;
pub const INT_MAX: core::ffi::c_int = __INT_MAX__;
