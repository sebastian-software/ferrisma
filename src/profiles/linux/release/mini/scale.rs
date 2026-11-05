// Copyright 2021 Joe Drago. All rights reserved.
// SPDX-License-Identifier: BSD-2-Clause

// Mechanically translated from upstream/libavif/src/scale.c.
// C ABI, ownership, pointer validity, and bounds follow the original headers.
extern "C" {
    fn avifFree(p: *mut core::ffi::c_void);
    fn avifResultToString(result: avifResult) -> *const core::ffi::c_char;
    fn avifDiagnosticsClearError(diag: *mut avifDiagnostics);
    fn avifImageAllocatePlanes(
        image: *mut avifImage,
        planes: avifPlanesFlags,
    ) -> avifResult;
    fn avifImagePlaneWidth(
        image: *const avifImage,
        channel: core::ffi::c_int,
    ) -> uint32_t;
    fn avifImagePlaneHeight(
        image: *const avifImage,
        channel: core::ffi::c_int,
    ) -> uint32_t;
    fn avifDimensionsTooLarge(
        width: uint32_t,
        height: uint32_t,
        imageSizeLimit: uint32_t,
        imageDimensionLimit: uint32_t,
    ) -> avifBool;
    fn avifDiagnosticsPrintf(
        diag: *mut avifDiagnostics,
        format: *const core::ffi::c_char,
        ...
    );
    fn ScalePlane(
        src: *const uint8_t,
        src_stride: core::ffi::c_int,
        src_width: core::ffi::c_int,
        src_height: core::ffi::c_int,
        dst: *mut uint8_t,
        dst_stride: core::ffi::c_int,
        dst_width: core::ffi::c_int,
        dst_height: core::ffi::c_int,
        filtering: FilterMode,
    ) -> core::ffi::c_int;
    fn ScalePlane_12(
        src: *const uint16_t,
        src_stride: core::ffi::c_int,
        src_width: core::ffi::c_int,
        src_height: core::ffi::c_int,
        dst: *mut uint16_t,
        dst_stride: core::ffi::c_int,
        dst_width: core::ffi::c_int,
        dst_height: core::ffi::c_int,
        filtering: FilterMode,
    ) -> core::ffi::c_int;
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
pub type FilterMode = core::ffi::c_uint;
pub const kFilterBox: FilterMode = 3;
pub const kFilterBilinear: FilterMode = 2;
pub const kFilterLinear: FilterMode = 1;
pub const kFilterNone: FilterMode = 0;
pub const NULL: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
pub const AVIF_FALSE: core::ffi::c_int = 0 as core::ffi::c_int;
pub const AVIF_DEFAULT_IMAGE_SIZE_LIMIT: core::ffi::c_int = 16384 as core::ffi::c_int
    * 16384 as core::ffi::c_int;
pub const AVIF_DEFAULT_IMAGE_DIMENSION_LIMIT: core::ffi::c_int = 32768
    as core::ffi::c_int;
pub const AVIF_PLANE_COUNT_YUV: core::ffi::c_int = 3 as core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn avifImageScaleWithLimit(
    mut image: *mut avifImage,
    mut dstWidth: uint32_t,
    mut dstHeight: uint32_t,
    mut imageSizeLimit: uint32_t,
    mut imageDimensionLimit: uint32_t,
    mut diag: *mut avifDiagnostics,
) -> avifResult {
    let mut current_block: u64;
    if (*image).width == dstWidth && (*image).height == dstHeight {
        return AVIF_RESULT_OK;
    }
    if dstWidth == 0 as uint32_t || dstHeight == 0 as uint32_t {
        avifDiagnosticsPrintf(
            diag,
            b"avifImageScaleWithLimit requested invalid dst dimensions [%ux%u]\0"
                as *const u8 as *const core::ffi::c_char,
            dstWidth,
            dstHeight,
        );
        return AVIF_RESULT_INVALID_ARGUMENT;
    }
    if avifDimensionsTooLarge(dstWidth, dstHeight, imageSizeLimit, imageDimensionLimit)
        != 0
    {
        avifDiagnosticsPrintf(
            diag,
            b"avifImageScaleWithLimit requested dst dimensions that are too large [%ux%u]\0"
                as *const u8 as *const core::ffi::c_char,
            dstWidth,
            dstHeight,
        );
        return AVIF_RESULT_NOT_IMPLEMENTED;
    }
    let mut srcYUVPlanes: [*mut uint8_t; 3] = [0 as *mut uint8_t; 3];
    let mut srcYUVRowBytes: [uint32_t; 3] = [0; 3];
    let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
    while i < AVIF_PLANE_COUNT_YUV {
        srcYUVPlanes[i as usize] = (*image).yuvPlanes[i as usize];
        (*image).yuvPlanes[i as usize] = 0 as *mut uint8_t;
        srcYUVRowBytes[i as usize] = (*image).yuvRowBytes[i as usize];
        (*image).yuvRowBytes[i as usize] = 0 as uint32_t;
        i += 1;
    }
    let srcImageOwnsYUVPlanes: avifBool = (*image).imageOwnsYUVPlanes;
    (*image).imageOwnsYUVPlanes = AVIF_FALSE as avifBool;
    let mut srcAlphaPlane: *mut uint8_t = (*image).alphaPlane;
    (*image).alphaPlane = 0 as *mut uint8_t;
    let mut srcAlphaRowBytes: uint32_t = (*image).alphaRowBytes;
    (*image).alphaRowBytes = 0 as uint32_t;
    let srcImageOwnsAlphaPlane: avifBool = (*image).imageOwnsAlphaPlane;
    (*image).imageOwnsAlphaPlane = AVIF_FALSE as avifBool;
    let srcWidth: uint32_t = (*image).width;
    let srcHeight: uint32_t = (*image).height;
    let srcUVWidth: uint32_t = avifImagePlaneWidth(
        image,
        AVIF_CHAN_U as core::ffi::c_int,
    ) as uint32_t;
    let srcUVHeight: uint32_t = avifImagePlaneHeight(
        image,
        AVIF_CHAN_U as core::ffi::c_int,
    ) as uint32_t;
    (*image).width = dstWidth;
    (*image).height = dstHeight;
    let mut result: avifResult = AVIF_RESULT_OK;
    if !(srcYUVPlanes[0 as core::ffi::c_int as usize]).is_null()
        || !srcAlphaPlane.is_null()
    {
        if srcWidth > 16384 as uint32_t {
            avifDiagnosticsPrintf(
                diag,
                b"avifImageScaleWithLimit requested invalid width scale for libyuv [%u -> %u]\0"
                    as *const u8 as *const core::ffi::c_char,
                srcWidth,
                dstWidth,
            );
            result = AVIF_RESULT_NOT_IMPLEMENTED;
            current_block = 0;
        } else if srcHeight > 16384 as uint32_t {
            avifDiagnosticsPrintf(
                diag,
                b"avifImageScaleWithLimit requested invalid height scale for libyuv [%u -> %u]\0"
                    as *const u8 as *const core::ffi::c_char,
                srcHeight,
                dstHeight,
            );
            result = AVIF_RESULT_NOT_IMPLEMENTED;
            current_block = 0;
        } else {
            current_block = 1;
        }
    } else {
        current_block = 1;
    }
    match current_block {
        1 => {
            if !(srcYUVPlanes[0 as core::ffi::c_int as usize]).is_null() {
                let allocationResult: avifResult = avifImageAllocatePlanes(
                    image,
                    AVIF_PLANES_YUV as core::ffi::c_int as avifPlanesFlags,
                ) as avifResult;
                if allocationResult as core::ffi::c_uint
                    != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
                {
                    avifDiagnosticsPrintf(
                        diag,
                        b"Allocation of YUV planes failed: %s\0" as *const u8
                            as *const core::ffi::c_char,
                        avifResultToString(allocationResult),
                    );
                    result = AVIF_RESULT_OUT_OF_MEMORY;
                    current_block = 0;
                } else {
                    let mut i_0: core::ffi::c_int = 0 as core::ffi::c_int;
                    loop {
                        if !(i_0 < AVIF_PLANE_COUNT_YUV) {
                            current_block = 2;
                            break;
                        }
                        if !(srcYUVPlanes[i_0 as usize]).is_null() {
                            let srcW: uint32_t = if i_0
                                == AVIF_CHAN_Y as core::ffi::c_int
                            {
                                srcWidth
                            } else {
                                srcUVWidth
                            };
                            let srcH: uint32_t = if i_0
                                == AVIF_CHAN_Y as core::ffi::c_int
                            {
                                srcHeight
                            } else {
                                srcUVHeight
                            };
                            let dstW: uint32_t = avifImagePlaneWidth(image, i_0)
                                as uint32_t;
                            let dstH: uint32_t = avifImagePlaneHeight(image, i_0)
                                as uint32_t;
                            if (*image).depth > 8 as uint32_t {
                                let srcPlane: *mut uint16_t = srcYUVPlanes[i_0 as usize]
                                    as *mut uint16_t;
                                let srcStride: uint32_t = (srcYUVRowBytes[i_0 as usize])
                                    .wrapping_div(2 as uint32_t);
                                let dstPlane: *mut uint16_t = (*image)
                                    .yuvPlanes[i_0 as usize] as *mut uint16_t;
                                let dstStride: uint32_t = ((*image)
                                    .yuvRowBytes[i_0 as usize])
                                    .wrapping_div(2 as uint32_t);
                                let failure: core::ffi::c_int = ScalePlane_12(
                                    srcPlane,
                                    srcStride as core::ffi::c_int,
                                    srcW as core::ffi::c_int,
                                    srcH as core::ffi::c_int,
                                    dstPlane,
                                    dstStride as core::ffi::c_int,
                                    dstW as core::ffi::c_int,
                                    dstH as core::ffi::c_int,
                                    kFilterBox,
                                ) as core::ffi::c_int;
                                if failure != 0 {
                                    avifDiagnosticsPrintf(
                                        diag,
                                        b"ScalePlane_12() failed (%d)\0" as *const u8
                                            as *const core::ffi::c_char,
                                        failure,
                                    );
                                    result = (if failure == 1 as core::ffi::c_int {
                                        AVIF_RESULT_OUT_OF_MEMORY as core::ffi::c_int
                                    } else {
                                        AVIF_RESULT_UNKNOWN_ERROR as core::ffi::c_int
                                    }) as avifResult;
                                    current_block = 0;
                                    break;
                                }
                            } else {
                                let srcPlane_0: *mut uint8_t = srcYUVPlanes[i_0 as usize];
                                let srcStride_0: uint32_t = srcYUVRowBytes[i_0 as usize];
                                let dstPlane_0: *mut uint8_t = (*image)
                                    .yuvPlanes[i_0 as usize];
                                let dstStride_0: uint32_t = (*image)
                                    .yuvRowBytes[i_0 as usize];
                                let failure_0: core::ffi::c_int = ScalePlane(
                                    srcPlane_0,
                                    srcStride_0 as core::ffi::c_int,
                                    srcW as core::ffi::c_int,
                                    srcH as core::ffi::c_int,
                                    dstPlane_0,
                                    dstStride_0 as core::ffi::c_int,
                                    dstW as core::ffi::c_int,
                                    dstH as core::ffi::c_int,
                                    kFilterBox,
                                ) as core::ffi::c_int;
                                if failure_0 != 0 {
                                    avifDiagnosticsPrintf(
                                        diag,
                                        b"ScalePlane() failed (%d)\0" as *const u8
                                            as *const core::ffi::c_char,
                                        failure_0,
                                    );
                                    result = (if failure_0 == 1 as core::ffi::c_int {
                                        AVIF_RESULT_OUT_OF_MEMORY as core::ffi::c_int
                                    } else {
                                        AVIF_RESULT_UNKNOWN_ERROR as core::ffi::c_int
                                    }) as avifResult;
                                    current_block = 0;
                                    break;
                                }
                            }
                        }
                        i_0 += 1;
                    }
                }
            } else {
                current_block = 2;
            }
            match current_block {
                0 => {}
                _ => {
                    if !srcAlphaPlane.is_null() {
                        let allocationResult_0: avifResult = avifImageAllocatePlanes(
                            image,
                            AVIF_PLANES_A as core::ffi::c_int as avifPlanesFlags,
                        ) as avifResult;
                        if allocationResult_0 as core::ffi::c_uint
                            != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
                        {
                            avifDiagnosticsPrintf(
                                diag,
                                b"Allocation of alpha plane failed: %s\0" as *const u8
                                    as *const core::ffi::c_char,
                                avifResultToString(allocationResult_0),
                            );
                            result = AVIF_RESULT_OUT_OF_MEMORY;
                        } else if (*image).depth > 8 as uint32_t {
                            let srcPlane_1: *mut uint16_t = srcAlphaPlane
                                as *mut uint16_t;
                            let srcStride_1: uint32_t = srcAlphaRowBytes
                                .wrapping_div(2 as uint32_t);
                            let dstPlane_1: *mut uint16_t = (*image).alphaPlane
                                as *mut uint16_t;
                            let dstStride_1: uint32_t = ((*image).alphaRowBytes)
                                .wrapping_div(2 as uint32_t);
                            let failure_1: core::ffi::c_int = ScalePlane_12(
                                srcPlane_1,
                                srcStride_1 as core::ffi::c_int,
                                srcWidth as core::ffi::c_int,
                                srcHeight as core::ffi::c_int,
                                dstPlane_1,
                                dstStride_1 as core::ffi::c_int,
                                dstWidth as core::ffi::c_int,
                                dstHeight as core::ffi::c_int,
                                kFilterBox,
                            ) as core::ffi::c_int;
                            if failure_1 != 0 {
                                avifDiagnosticsPrintf(
                                    diag,
                                    b"ScalePlane_12() failed (%d)\0" as *const u8
                                        as *const core::ffi::c_char,
                                    failure_1,
                                );
                                result = (if failure_1 == 1 as core::ffi::c_int {
                                    AVIF_RESULT_OUT_OF_MEMORY as core::ffi::c_int
                                } else {
                                    AVIF_RESULT_UNKNOWN_ERROR as core::ffi::c_int
                                }) as avifResult;
                            }
                        } else {
                            let srcPlane_2: *mut uint8_t = srcAlphaPlane;
                            let srcStride_2: uint32_t = srcAlphaRowBytes;
                            let dstPlane_2: *mut uint8_t = (*image).alphaPlane;
                            let dstStride_2: uint32_t = (*image).alphaRowBytes;
                            let failure_2: core::ffi::c_int = ScalePlane(
                                srcPlane_2,
                                srcStride_2 as core::ffi::c_int,
                                srcWidth as core::ffi::c_int,
                                srcHeight as core::ffi::c_int,
                                dstPlane_2,
                                dstStride_2 as core::ffi::c_int,
                                dstWidth as core::ffi::c_int,
                                dstHeight as core::ffi::c_int,
                                kFilterBox,
                            ) as core::ffi::c_int;
                            if failure_2 != 0 {
                                avifDiagnosticsPrintf(
                                    diag,
                                    b"ScalePlane() failed (%d)\0" as *const u8
                                        as *const core::ffi::c_char,
                                    failure_2,
                                );
                                result = (if failure_2 == 1 as core::ffi::c_int {
                                    AVIF_RESULT_OUT_OF_MEMORY as core::ffi::c_int
                                } else {
                                    AVIF_RESULT_UNKNOWN_ERROR as core::ffi::c_int
                                }) as avifResult;
                            }
                        }
                    }
                }
            }
        }
        _ => {}
    }
    if !(srcYUVPlanes[0 as core::ffi::c_int as usize]).is_null()
        && srcImageOwnsYUVPlanes != 0
    {
        let mut i_1: core::ffi::c_int = 0 as core::ffi::c_int;
        while i_1 < AVIF_PLANE_COUNT_YUV {
            avifFree(srcYUVPlanes[i_1 as usize] as *mut core::ffi::c_void);
            i_1 += 1;
        }
    }
    if !srcAlphaPlane.is_null() && srcImageOwnsAlphaPlane != 0 {
        avifFree(srcAlphaPlane as *mut core::ffi::c_void);
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn avifImageScale(
    mut image: *mut avifImage,
    mut dstWidth: uint32_t,
    mut dstHeight: uint32_t,
    mut diag: *mut avifDiagnostics,
) -> avifResult {
    avifDiagnosticsClearError(diag);
    return avifImageScaleWithLimit(
        image,
        dstWidth,
        dstHeight,
        AVIF_DEFAULT_IMAGE_SIZE_LIMIT as uint32_t,
        AVIF_DEFAULT_IMAGE_DIMENSION_LIMIT as uint32_t,
        diag,
    );
}
