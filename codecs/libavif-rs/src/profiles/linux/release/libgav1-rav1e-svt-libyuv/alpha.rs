// Copyright 2020 Joe Drago. All rights reserved.
// SPDX-License-Identifier: BSD-2-Clause

// Mechanically translated from upstream/libavif/src/alpha.c.
// C ABI, ownership, pointer validity, and bounds follow the original headers.
extern "C" {
    fn avifRGBFormatHasAlpha(format: avifRGBFormat) -> avifBool;
    fn avifRoundf(v: core::ffi::c_float) -> core::ffi::c_float;
    fn avifRGBImagePremultiplyAlphaLibYUV(rgb: *mut avifRGBImage) -> avifResult;
    fn avifRGBImageUnpremultiplyAlphaLibYUV(rgb: *mut avifRGBImage) -> avifResult;
}
pub type __uint8_t = u8;
pub type __uint16_t = u16;
pub type __uint32_t = u32;
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
#[no_mangle]
pub unsafe extern "C" fn avifFillAlpha(mut params: *const avifAlphaParams) {
    if (*params).dstDepth > 8 as uint32_t {
        let maxChannel: uint16_t = (((1 as core::ffi::c_int) << (*params).dstDepth)
            - 1 as core::ffi::c_int) as uint16_t;
        let mut dstRow: *mut uint8_t = &mut *((*params).dstPlane)
            .offset((*params).dstOffsetBytes as isize) as *mut uint8_t;
        let mut j: uint32_t = 0 as uint32_t;
        while j < (*params).height {
            let mut dstPixel: *mut uint8_t = dstRow;
            let mut i: uint32_t = 0 as uint32_t;
            while i < (*params).width {
                *(dstPixel as *mut uint16_t) = maxChannel;
                dstPixel = dstPixel.offset((*params).dstPixelBytes as isize);
                i = i.wrapping_add(1);
            }
            dstRow = dstRow.offset((*params).dstRowBytes as isize);
            j = j.wrapping_add(1);
        }
    } else {
        let maxChannel_0: uint8_t = 255 as uint8_t;
        let mut dstRow_0: *mut uint8_t = &mut *((*params).dstPlane)
            .offset((*params).dstOffsetBytes as isize) as *mut uint8_t;
        let mut j_0: uint32_t = 0 as uint32_t;
        while j_0 < (*params).height {
            let mut dstPixel_0: *mut uint8_t = dstRow_0;
            let mut i_0: uint32_t = 0 as uint32_t;
            while i_0 < (*params).width {
                *dstPixel_0 = maxChannel_0;
                dstPixel_0 = dstPixel_0.offset((*params).dstPixelBytes as isize);
                i_0 = i_0.wrapping_add(1);
            }
            dstRow_0 = dstRow_0.offset((*params).dstRowBytes as isize);
            j_0 = j_0.wrapping_add(1);
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn avifReformatAlpha(mut params: *const avifAlphaParams) {
    let srcMaxChannel: core::ffi::c_int = ((1 as core::ffi::c_int) << (*params).srcDepth)
        - 1 as core::ffi::c_int;
    let dstMaxChannel: core::ffi::c_int = ((1 as core::ffi::c_int) << (*params).dstDepth)
        - 1 as core::ffi::c_int;
    let srcMaxChannelF: core::ffi::c_float = srcMaxChannel as core::ffi::c_float;
    let dstMaxChannelF: core::ffi::c_float = dstMaxChannel as core::ffi::c_float;
    if (*params).srcDepth == (*params).dstDepth {
        if (*params).srcDepth > 8 as uint32_t {
            let mut srcRow: *const uint8_t = &*((*params).srcPlane)
                .offset((*params).srcOffsetBytes as isize) as *const uint8_t;
            let mut dstRow: *mut uint8_t = &mut *((*params).dstPlane)
                .offset((*params).dstOffsetBytes as isize) as *mut uint8_t;
            let mut j: uint32_t = 0 as uint32_t;
            while j < (*params).height {
                let mut srcPixel: *const uint8_t = srcRow;
                let mut dstPixel: *mut uint8_t = dstRow;
                let mut i: uint32_t = 0 as uint32_t;
                while i < (*params).width {
                    *(dstPixel as *mut uint16_t) = *(srcPixel as *const uint16_t);
                    srcPixel = srcPixel.offset((*params).srcPixelBytes as isize);
                    dstPixel = dstPixel.offset((*params).dstPixelBytes as isize);
                    i = i.wrapping_add(1);
                }
                srcRow = srcRow.offset((*params).srcRowBytes as isize);
                dstRow = dstRow.offset((*params).dstRowBytes as isize);
                j = j.wrapping_add(1);
            }
        } else {
            let mut srcRow_0: *const uint8_t = &*((*params).srcPlane)
                .offset((*params).srcOffsetBytes as isize) as *const uint8_t;
            let mut dstRow_0: *mut uint8_t = &mut *((*params).dstPlane)
                .offset((*params).dstOffsetBytes as isize) as *mut uint8_t;
            let mut j_0: uint32_t = 0 as uint32_t;
            while j_0 < (*params).height {
                let mut srcPixel_0: *const uint8_t = srcRow_0;
                let mut dstPixel_0: *mut uint8_t = dstRow_0;
                let mut i_0: uint32_t = 0 as uint32_t;
                while i_0 < (*params).width {
                    *dstPixel_0 = *srcPixel_0;
                    srcPixel_0 = srcPixel_0.offset((*params).srcPixelBytes as isize);
                    dstPixel_0 = dstPixel_0.offset((*params).dstPixelBytes as isize);
                    i_0 = i_0.wrapping_add(1);
                }
                srcRow_0 = srcRow_0.offset((*params).srcRowBytes as isize);
                dstRow_0 = dstRow_0.offset((*params).dstRowBytes as isize);
                j_0 = j_0.wrapping_add(1);
            }
        }
    } else if (*params).srcDepth > 8 as uint32_t {
        if (*params).dstDepth > 8 as uint32_t {
            let mut srcRow_1: *const uint8_t = &*((*params).srcPlane)
                .offset((*params).srcOffsetBytes as isize) as *const uint8_t;
            let mut dstRow_1: *mut uint8_t = &mut *((*params).dstPlane)
                .offset((*params).dstOffsetBytes as isize) as *mut uint8_t;
            let mut j_1: uint32_t = 0 as uint32_t;
            while j_1 < (*params).height {
                let mut srcPixel_1: *const uint8_t = srcRow_1;
                let mut dstPixel_1: *mut uint8_t = dstRow_1;
                let mut i_1: uint32_t = 0 as uint32_t;
                while i_1 < (*params).width {
                    let mut srcAlpha: core::ffi::c_int = *(srcPixel_1 as *const uint16_t)
                        as core::ffi::c_int;
                    let mut alphaF: core::ffi::c_float = srcAlpha as core::ffi::c_float
                        / srcMaxChannelF;
                    let mut dstAlpha: core::ffi::c_int = (0.5f32
                        + alphaF * dstMaxChannelF) as core::ffi::c_int;
                    dstAlpha = if dstAlpha < 0 as core::ffi::c_int {
                        0 as core::ffi::c_int
                    } else if dstMaxChannel < dstAlpha {
                        dstMaxChannel
                    } else {
                        dstAlpha
                    };
                    *(dstPixel_1 as *mut uint16_t) = dstAlpha as uint16_t;
                    srcPixel_1 = srcPixel_1.offset((*params).srcPixelBytes as isize);
                    dstPixel_1 = dstPixel_1.offset((*params).dstPixelBytes as isize);
                    i_1 = i_1.wrapping_add(1);
                }
                srcRow_1 = srcRow_1.offset((*params).srcRowBytes as isize);
                dstRow_1 = dstRow_1.offset((*params).dstRowBytes as isize);
                j_1 = j_1.wrapping_add(1);
            }
        } else {
            let mut srcRow_2: *const uint8_t = &*((*params).srcPlane)
                .offset((*params).srcOffsetBytes as isize) as *const uint8_t;
            let mut dstRow_2: *mut uint8_t = &mut *((*params).dstPlane)
                .offset((*params).dstOffsetBytes as isize) as *mut uint8_t;
            let mut j_2: uint32_t = 0 as uint32_t;
            while j_2 < (*params).height {
                let mut srcPixel_2: *const uint8_t = srcRow_2;
                let mut dstPixel_2: *mut uint8_t = dstRow_2;
                let mut i_2: uint32_t = 0 as uint32_t;
                while i_2 < (*params).width {
                    let mut srcAlpha_0: core::ffi::c_int = *(srcPixel_2
                        as *const uint16_t) as core::ffi::c_int;
                    let mut alphaF_0: core::ffi::c_float = srcAlpha_0
                        as core::ffi::c_float / srcMaxChannelF;
                    let mut dstAlpha_0: core::ffi::c_int = (0.5f32
                        + alphaF_0 * dstMaxChannelF) as core::ffi::c_int;
                    dstAlpha_0 = if dstAlpha_0 < 0 as core::ffi::c_int {
                        0 as core::ffi::c_int
                    } else if dstMaxChannel < dstAlpha_0 {
                        dstMaxChannel
                    } else {
                        dstAlpha_0
                    };
                    *dstPixel_2 = dstAlpha_0 as uint8_t;
                    srcPixel_2 = srcPixel_2.offset((*params).srcPixelBytes as isize);
                    dstPixel_2 = dstPixel_2.offset((*params).dstPixelBytes as isize);
                    i_2 = i_2.wrapping_add(1);
                }
                srcRow_2 = srcRow_2.offset((*params).srcRowBytes as isize);
                dstRow_2 = dstRow_2.offset((*params).dstRowBytes as isize);
                j_2 = j_2.wrapping_add(1);
            }
        }
    } else {
        let mut srcRow_3: *const uint8_t = &*((*params).srcPlane)
            .offset((*params).srcOffsetBytes as isize) as *const uint8_t;
        let mut dstRow_3: *mut uint8_t = &mut *((*params).dstPlane)
            .offset((*params).dstOffsetBytes as isize) as *mut uint8_t;
        let mut j_3: uint32_t = 0 as uint32_t;
        while j_3 < (*params).height {
            let mut srcPixel_3: *const uint8_t = srcRow_3;
            let mut dstPixel_3: *mut uint8_t = dstRow_3;
            let mut i_3: uint32_t = 0 as uint32_t;
            while i_3 < (*params).width {
                let mut srcAlpha_1: core::ffi::c_int = *srcPixel_3 as core::ffi::c_int;
                let mut alphaF_1: core::ffi::c_float = srcAlpha_1 as core::ffi::c_float
                    / srcMaxChannelF;
                let mut dstAlpha_1: core::ffi::c_int = (0.5f32
                    + alphaF_1 * dstMaxChannelF) as core::ffi::c_int;
                dstAlpha_1 = if dstAlpha_1 < 0 as core::ffi::c_int {
                    0 as core::ffi::c_int
                } else if dstMaxChannel < dstAlpha_1 {
                    dstMaxChannel
                } else {
                    dstAlpha_1
                };
                *(dstPixel_3 as *mut uint16_t) = dstAlpha_1 as uint16_t;
                srcPixel_3 = srcPixel_3.offset((*params).srcPixelBytes as isize);
                dstPixel_3 = dstPixel_3.offset((*params).dstPixelBytes as isize);
                i_3 = i_3.wrapping_add(1);
            }
            srcRow_3 = srcRow_3.offset((*params).srcRowBytes as isize);
            dstRow_3 = dstRow_3.offset((*params).dstRowBytes as isize);
            j_3 = j_3.wrapping_add(1);
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn avifRGBImagePremultiplyAlpha(
    mut rgb: *mut avifRGBImage,
) -> avifResult {
    if ((*rgb).pixels).is_null() || (*rgb).rowBytes == 0 {
        return AVIF_RESULT_REFORMAT_FAILED;
    }
    if avifRGBFormatHasAlpha((*rgb).format) == 0 {
        return AVIF_RESULT_INVALID_ARGUMENT;
    }
    let mut libyuvResult: avifResult = avifRGBImagePremultiplyAlphaLibYUV(rgb);
    if libyuvResult as core::ffi::c_uint
        != AVIF_RESULT_NOT_IMPLEMENTED as core::ffi::c_int as core::ffi::c_uint
    {
        return libyuvResult;
    }
    let mut max: uint32_t = (((1 as core::ffi::c_int) << (*rgb).depth)
        - 1 as core::ffi::c_int) as uint32_t;
    let mut maxF: core::ffi::c_float = max as core::ffi::c_float;
    if (*rgb).depth > 8 as uint32_t {
        if (*rgb).format as core::ffi::c_uint
            == AVIF_RGB_FORMAT_RGBA as core::ffi::c_int as core::ffi::c_uint
            || (*rgb).format as core::ffi::c_uint
                == AVIF_RGB_FORMAT_BGRA as core::ffi::c_int as core::ffi::c_uint
        {
            let mut row: *mut uint8_t = (*rgb).pixels;
            let mut j: uint32_t = 0 as uint32_t;
            while j < (*rgb).height {
                let mut pixel: *mut uint16_t = row as *mut uint16_t;
                let mut i: uint32_t = 0 as uint32_t;
                while i < (*rgb).width {
                    let mut a: uint16_t = *pixel.offset(3 as core::ffi::c_int as isize);
                    if !(a as uint32_t >= max) {
                        if a as core::ffi::c_int == 0 as core::ffi::c_int {
                            *pixel.offset(0 as core::ffi::c_int as isize) = 0
                                as uint16_t;
                            *pixel.offset(1 as core::ffi::c_int as isize) = 0
                                as uint16_t;
                            *pixel.offset(2 as core::ffi::c_int as isize) = 0
                                as uint16_t;
                        } else {
                            *pixel.offset(0 as core::ffi::c_int as isize) = avifRoundf(
                                *pixel.offset(0 as core::ffi::c_int as isize)
                                    as core::ffi::c_float * a as core::ffi::c_float / maxF,
                            ) as uint16_t;
                            *pixel.offset(1 as core::ffi::c_int as isize) = avifRoundf(
                                *pixel.offset(1 as core::ffi::c_int as isize)
                                    as core::ffi::c_float * a as core::ffi::c_float / maxF,
                            ) as uint16_t;
                            *pixel.offset(2 as core::ffi::c_int as isize) = avifRoundf(
                                *pixel.offset(2 as core::ffi::c_int as isize)
                                    as core::ffi::c_float * a as core::ffi::c_float / maxF,
                            ) as uint16_t;
                        }
                    }
                    pixel = pixel.offset(4 as core::ffi::c_int as isize);
                    i = i.wrapping_add(1);
                }
                row = row.offset((*rgb).rowBytes as isize);
                j = j.wrapping_add(1);
            }
        } else if (*rgb).format as core::ffi::c_uint
            == AVIF_RGB_FORMAT_ARGB as core::ffi::c_int as core::ffi::c_uint
            || (*rgb).format as core::ffi::c_uint
                == AVIF_RGB_FORMAT_ABGR as core::ffi::c_int as core::ffi::c_uint
        {
            let mut row_0: *mut uint8_t = (*rgb).pixels;
            let mut j_0: uint32_t = 0 as uint32_t;
            while j_0 < (*rgb).height {
                let mut pixel_0: *mut uint16_t = row_0 as *mut uint16_t;
                let mut i_0: uint32_t = 0 as uint32_t;
                while i_0 < (*rgb).width {
                    let mut a_0: uint16_t = *pixel_0
                        .offset(0 as core::ffi::c_int as isize);
                    if !(a_0 as uint32_t >= max) {
                        if a_0 as core::ffi::c_int == 0 as core::ffi::c_int {
                            *pixel_0.offset(1 as core::ffi::c_int as isize) = 0
                                as uint16_t;
                            *pixel_0.offset(2 as core::ffi::c_int as isize) = 0
                                as uint16_t;
                            *pixel_0.offset(3 as core::ffi::c_int as isize) = 0
                                as uint16_t;
                        } else {
                            *pixel_0.offset(1 as core::ffi::c_int as isize) = avifRoundf(
                                *pixel_0.offset(1 as core::ffi::c_int as isize)
                                    as core::ffi::c_float * a_0 as core::ffi::c_float / maxF,
                            ) as uint16_t;
                            *pixel_0.offset(2 as core::ffi::c_int as isize) = avifRoundf(
                                *pixel_0.offset(2 as core::ffi::c_int as isize)
                                    as core::ffi::c_float * a_0 as core::ffi::c_float / maxF,
                            ) as uint16_t;
                            *pixel_0.offset(3 as core::ffi::c_int as isize) = avifRoundf(
                                *pixel_0.offset(3 as core::ffi::c_int as isize)
                                    as core::ffi::c_float * a_0 as core::ffi::c_float / maxF,
                            ) as uint16_t;
                        }
                    }
                    pixel_0 = pixel_0.offset(4 as core::ffi::c_int as isize);
                    i_0 = i_0.wrapping_add(1);
                }
                row_0 = row_0.offset((*rgb).rowBytes as isize);
                j_0 = j_0.wrapping_add(1);
            }
        } else if (*rgb).format as core::ffi::c_uint
            == AVIF_RGB_FORMAT_GRAYA as core::ffi::c_int as core::ffi::c_uint
        {
            let mut row_1: *mut uint8_t = (*rgb).pixels;
            let mut j_1: uint32_t = 0 as uint32_t;
            while j_1 < (*rgb).height {
                let mut pixel_1: *mut uint16_t = row_1 as *mut uint16_t;
                let mut i_1: uint32_t = 0 as uint32_t;
                while i_1 < (*rgb).width {
                    let mut a_1: uint16_t = *pixel_1
                        .offset(1 as core::ffi::c_int as isize);
                    if !(a_1 as uint32_t >= max) {
                        if a_1 as core::ffi::c_int == 0 as core::ffi::c_int {
                            *pixel_1.offset(0 as core::ffi::c_int as isize) = 0
                                as uint16_t;
                        } else {
                            *pixel_1.offset(0 as core::ffi::c_int as isize) = avifRoundf(
                                *pixel_1.offset(0 as core::ffi::c_int as isize)
                                    as core::ffi::c_float * a_1 as core::ffi::c_float / maxF,
                            ) as uint16_t;
                        }
                    }
                    pixel_1 = pixel_1.offset(2 as core::ffi::c_int as isize);
                    i_1 = i_1.wrapping_add(1);
                }
                row_1 = row_1.offset((*rgb).rowBytes as isize);
                j_1 = j_1.wrapping_add(1);
            }
        } else if (*rgb).format as core::ffi::c_uint
            == AVIF_RGB_FORMAT_AGRAY as core::ffi::c_int as core::ffi::c_uint
        {
            let mut row_2: *mut uint8_t = (*rgb).pixels;
            let mut j_2: uint32_t = 0 as uint32_t;
            while j_2 < (*rgb).height {
                let mut pixel_2: *mut uint16_t = row_2 as *mut uint16_t;
                let mut i_2: uint32_t = 0 as uint32_t;
                while i_2 < (*rgb).width {
                    let mut a_2: uint16_t = *pixel_2
                        .offset(0 as core::ffi::c_int as isize);
                    if !(a_2 as uint32_t >= max) {
                        if a_2 as core::ffi::c_int == 0 as core::ffi::c_int {
                            *pixel_2.offset(1 as core::ffi::c_int as isize) = 0
                                as uint16_t;
                        } else {
                            *pixel_2.offset(1 as core::ffi::c_int as isize) = avifRoundf(
                                *pixel_2.offset(1 as core::ffi::c_int as isize)
                                    as core::ffi::c_float * a_2 as core::ffi::c_float / maxF,
                            ) as uint16_t;
                        }
                    }
                    pixel_2 = pixel_2.offset(2 as core::ffi::c_int as isize);
                    i_2 = i_2.wrapping_add(1);
                }
                row_2 = row_2.offset((*rgb).rowBytes as isize);
                j_2 = j_2.wrapping_add(1);
            }
        } else {
            return AVIF_RESULT_NOT_IMPLEMENTED
        }
    } else if (*rgb).format as core::ffi::c_uint
        == AVIF_RGB_FORMAT_RGBA as core::ffi::c_int as core::ffi::c_uint
        || (*rgb).format as core::ffi::c_uint
            == AVIF_RGB_FORMAT_BGRA as core::ffi::c_int as core::ffi::c_uint
    {
        let mut row_3: *mut uint8_t = (*rgb).pixels;
        let mut j_3: uint32_t = 0 as uint32_t;
        while j_3 < (*rgb).height {
            let mut pixel_3: *mut uint8_t = row_3;
            let mut i_3: uint32_t = 0 as uint32_t;
            while i_3 < (*rgb).width {
                let mut a_3: uint8_t = *pixel_3.offset(3 as core::ffi::c_int as isize);
                if !(a_3 as uint32_t == max) {
                    if a_3 as core::ffi::c_int == 0 as core::ffi::c_int {
                        *pixel_3.offset(0 as core::ffi::c_int as isize) = 0 as uint8_t;
                        *pixel_3.offset(1 as core::ffi::c_int as isize) = 0 as uint8_t;
                        *pixel_3.offset(2 as core::ffi::c_int as isize) = 0 as uint8_t;
                    } else {
                        *pixel_3.offset(0 as core::ffi::c_int as isize) = avifRoundf(
                            *pixel_3.offset(0 as core::ffi::c_int as isize)
                                as core::ffi::c_float * a_3 as core::ffi::c_float / maxF,
                        ) as uint8_t;
                        *pixel_3.offset(1 as core::ffi::c_int as isize) = avifRoundf(
                            *pixel_3.offset(1 as core::ffi::c_int as isize)
                                as core::ffi::c_float * a_3 as core::ffi::c_float / maxF,
                        ) as uint8_t;
                        *pixel_3.offset(2 as core::ffi::c_int as isize) = avifRoundf(
                            *pixel_3.offset(2 as core::ffi::c_int as isize)
                                as core::ffi::c_float * a_3 as core::ffi::c_float / maxF,
                        ) as uint8_t;
                    }
                }
                pixel_3 = pixel_3.offset(4 as core::ffi::c_int as isize);
                i_3 = i_3.wrapping_add(1);
            }
            row_3 = row_3.offset((*rgb).rowBytes as isize);
            j_3 = j_3.wrapping_add(1);
        }
    } else if (*rgb).format as core::ffi::c_uint
        == AVIF_RGB_FORMAT_ARGB as core::ffi::c_int as core::ffi::c_uint
        || (*rgb).format as core::ffi::c_uint
            == AVIF_RGB_FORMAT_ABGR as core::ffi::c_int as core::ffi::c_uint
    {
        let mut row_4: *mut uint8_t = (*rgb).pixels;
        let mut j_4: uint32_t = 0 as uint32_t;
        while j_4 < (*rgb).height {
            let mut pixel_4: *mut uint8_t = row_4;
            let mut i_4: uint32_t = 0 as uint32_t;
            while i_4 < (*rgb).width {
                let mut a_4: uint8_t = *pixel_4.offset(0 as core::ffi::c_int as isize);
                if !(a_4 as uint32_t == max) {
                    if a_4 as core::ffi::c_int == 0 as core::ffi::c_int {
                        *pixel_4.offset(1 as core::ffi::c_int as isize) = 0 as uint8_t;
                        *pixel_4.offset(2 as core::ffi::c_int as isize) = 0 as uint8_t;
                        *pixel_4.offset(3 as core::ffi::c_int as isize) = 0 as uint8_t;
                    } else {
                        *pixel_4.offset(1 as core::ffi::c_int as isize) = avifRoundf(
                            *pixel_4.offset(1 as core::ffi::c_int as isize)
                                as core::ffi::c_float * a_4 as core::ffi::c_float / maxF,
                        ) as uint8_t;
                        *pixel_4.offset(2 as core::ffi::c_int as isize) = avifRoundf(
                            *pixel_4.offset(2 as core::ffi::c_int as isize)
                                as core::ffi::c_float * a_4 as core::ffi::c_float / maxF,
                        ) as uint8_t;
                        *pixel_4.offset(3 as core::ffi::c_int as isize) = avifRoundf(
                            *pixel_4.offset(3 as core::ffi::c_int as isize)
                                as core::ffi::c_float * a_4 as core::ffi::c_float / maxF,
                        ) as uint8_t;
                    }
                }
                pixel_4 = pixel_4.offset(4 as core::ffi::c_int as isize);
                i_4 = i_4.wrapping_add(1);
            }
            row_4 = row_4.offset((*rgb).rowBytes as isize);
            j_4 = j_4.wrapping_add(1);
        }
    } else if (*rgb).format as core::ffi::c_uint
        == AVIF_RGB_FORMAT_GRAYA as core::ffi::c_int as core::ffi::c_uint
    {
        let mut row_5: *mut uint8_t = (*rgb).pixels;
        let mut j_5: uint32_t = 0 as uint32_t;
        while j_5 < (*rgb).height {
            let mut pixel_5: *mut uint8_t = row_5;
            let mut i_5: uint32_t = 0 as uint32_t;
            while i_5 < (*rgb).width {
                let mut a_5: uint8_t = *pixel_5.offset(1 as core::ffi::c_int as isize);
                if !(a_5 as uint32_t == max) {
                    if a_5 as core::ffi::c_int == 0 as core::ffi::c_int {
                        *pixel_5.offset(0 as core::ffi::c_int as isize) = 0 as uint8_t;
                    } else {
                        *pixel_5.offset(0 as core::ffi::c_int as isize) = avifRoundf(
                            *pixel_5.offset(0 as core::ffi::c_int as isize)
                                as core::ffi::c_float * a_5 as core::ffi::c_float / maxF,
                        ) as uint8_t;
                    }
                }
                pixel_5 = pixel_5.offset(2 as core::ffi::c_int as isize);
                i_5 = i_5.wrapping_add(1);
            }
            row_5 = row_5.offset((*rgb).rowBytes as isize);
            j_5 = j_5.wrapping_add(1);
        }
    } else if (*rgb).format as core::ffi::c_uint
        == AVIF_RGB_FORMAT_AGRAY as core::ffi::c_int as core::ffi::c_uint
    {
        let mut row_6: *mut uint8_t = (*rgb).pixels;
        let mut j_6: uint32_t = 0 as uint32_t;
        while j_6 < (*rgb).height {
            let mut pixel_6: *mut uint8_t = row_6;
            let mut i_6: uint32_t = 0 as uint32_t;
            while i_6 < (*rgb).width {
                let mut a_6: uint8_t = *pixel_6.offset(0 as core::ffi::c_int as isize);
                if !(a_6 as uint32_t == max) {
                    if a_6 as core::ffi::c_int == 0 as core::ffi::c_int {
                        *pixel_6.offset(1 as core::ffi::c_int as isize) = 0 as uint8_t;
                    } else {
                        *pixel_6.offset(1 as core::ffi::c_int as isize) = avifRoundf(
                            *pixel_6.offset(1 as core::ffi::c_int as isize)
                                as core::ffi::c_float * a_6 as core::ffi::c_float / maxF,
                        ) as uint8_t;
                    }
                }
                pixel_6 = pixel_6.offset(2 as core::ffi::c_int as isize);
                i_6 = i_6.wrapping_add(1);
            }
            row_6 = row_6.offset((*rgb).rowBytes as isize);
            j_6 = j_6.wrapping_add(1);
        }
    } else {
        return AVIF_RESULT_NOT_IMPLEMENTED
    }
    return AVIF_RESULT_OK;
}
#[no_mangle]
pub unsafe extern "C" fn avifRGBImageUnpremultiplyAlpha(
    mut rgb: *mut avifRGBImage,
) -> avifResult {
    if ((*rgb).pixels).is_null() || (*rgb).rowBytes == 0 {
        return AVIF_RESULT_REFORMAT_FAILED;
    }
    if avifRGBFormatHasAlpha((*rgb).format) == 0 {
        return AVIF_RESULT_REFORMAT_FAILED;
    }
    let mut libyuvResult: avifResult = avifRGBImageUnpremultiplyAlphaLibYUV(rgb);
    if libyuvResult as core::ffi::c_uint
        != AVIF_RESULT_NOT_IMPLEMENTED as core::ffi::c_int as core::ffi::c_uint
    {
        return libyuvResult;
    }
    let mut max: uint32_t = (((1 as core::ffi::c_int) << (*rgb).depth)
        - 1 as core::ffi::c_int) as uint32_t;
    let mut maxF: core::ffi::c_float = max as core::ffi::c_float;
    if (*rgb).depth > 8 as uint32_t {
        if (*rgb).format as core::ffi::c_uint
            == AVIF_RGB_FORMAT_RGBA as core::ffi::c_int as core::ffi::c_uint
            || (*rgb).format as core::ffi::c_uint
                == AVIF_RGB_FORMAT_BGRA as core::ffi::c_int as core::ffi::c_uint
        {
            let mut row: *mut uint8_t = (*rgb).pixels;
            let mut j: uint32_t = 0 as uint32_t;
            while j < (*rgb).height {
                let mut pixel: *mut uint16_t = row as *mut uint16_t;
                let mut i: uint32_t = 0 as uint32_t;
                while i < (*rgb).width {
                    let mut a: uint16_t = *pixel.offset(3 as core::ffi::c_int as isize);
                    if !(a as uint32_t >= max) {
                        if a as core::ffi::c_int == 0 as core::ffi::c_int {
                            *pixel.offset(0 as core::ffi::c_int as isize) = 0
                                as uint16_t;
                            *pixel.offset(1 as core::ffi::c_int as isize) = 0
                                as uint16_t;
                            *pixel.offset(2 as core::ffi::c_int as isize) = 0
                                as uint16_t;
                        } else {
                            let mut c1: core::ffi::c_float = avifRoundf(
                                *pixel.offset(0 as core::ffi::c_int as isize)
                                    as core::ffi::c_float * maxF / a as core::ffi::c_float,
                            );
                            let mut c2: core::ffi::c_float = avifRoundf(
                                *pixel.offset(1 as core::ffi::c_int as isize)
                                    as core::ffi::c_float * maxF / a as core::ffi::c_float,
                            );
                            let mut c3: core::ffi::c_float = avifRoundf(
                                *pixel.offset(2 as core::ffi::c_int as isize)
                                    as core::ffi::c_float * maxF / a as core::ffi::c_float,
                            );
                            *pixel.offset(0 as core::ffi::c_int as isize) = (if c1 < maxF
                            {
                                c1
                            } else {
                                maxF
                            }) as uint16_t;
                            *pixel.offset(1 as core::ffi::c_int as isize) = (if c2 < maxF
                            {
                                c2
                            } else {
                                maxF
                            }) as uint16_t;
                            *pixel.offset(2 as core::ffi::c_int as isize) = (if c3 < maxF
                            {
                                c3
                            } else {
                                maxF
                            }) as uint16_t;
                        }
                    }
                    pixel = pixel.offset(4 as core::ffi::c_int as isize);
                    i = i.wrapping_add(1);
                }
                row = row.offset((*rgb).rowBytes as isize);
                j = j.wrapping_add(1);
            }
        } else if (*rgb).format as core::ffi::c_uint
            == AVIF_RGB_FORMAT_ARGB as core::ffi::c_int as core::ffi::c_uint
            || (*rgb).format as core::ffi::c_uint
                == AVIF_RGB_FORMAT_ABGR as core::ffi::c_int as core::ffi::c_uint
        {
            let mut row_0: *mut uint8_t = (*rgb).pixels;
            let mut j_0: uint32_t = 0 as uint32_t;
            while j_0 < (*rgb).height {
                let mut pixel_0: *mut uint16_t = row_0 as *mut uint16_t;
                let mut i_0: uint32_t = 0 as uint32_t;
                while i_0 < (*rgb).width {
                    let mut a_0: uint16_t = *pixel_0
                        .offset(0 as core::ffi::c_int as isize);
                    if !(a_0 as uint32_t >= max) {
                        if a_0 as core::ffi::c_int == 0 as core::ffi::c_int {
                            *pixel_0.offset(1 as core::ffi::c_int as isize) = 0
                                as uint16_t;
                            *pixel_0.offset(2 as core::ffi::c_int as isize) = 0
                                as uint16_t;
                            *pixel_0.offset(3 as core::ffi::c_int as isize) = 0
                                as uint16_t;
                        } else {
                            let mut c1_0: core::ffi::c_float = avifRoundf(
                                *pixel_0.offset(1 as core::ffi::c_int as isize)
                                    as core::ffi::c_float * maxF / a_0 as core::ffi::c_float,
                            );
                            let mut c2_0: core::ffi::c_float = avifRoundf(
                                *pixel_0.offset(2 as core::ffi::c_int as isize)
                                    as core::ffi::c_float * maxF / a_0 as core::ffi::c_float,
                            );
                            let mut c3_0: core::ffi::c_float = avifRoundf(
                                *pixel_0.offset(3 as core::ffi::c_int as isize)
                                    as core::ffi::c_float * maxF / a_0 as core::ffi::c_float,
                            );
                            *pixel_0.offset(1 as core::ffi::c_int as isize) = (if c1_0
                                < maxF
                            {
                                c1_0
                            } else {
                                maxF
                            }) as uint16_t;
                            *pixel_0.offset(2 as core::ffi::c_int as isize) = (if c2_0
                                < maxF
                            {
                                c2_0
                            } else {
                                maxF
                            }) as uint16_t;
                            *pixel_0.offset(3 as core::ffi::c_int as isize) = (if c3_0
                                < maxF
                            {
                                c3_0
                            } else {
                                maxF
                            }) as uint16_t;
                        }
                    }
                    pixel_0 = pixel_0.offset(4 as core::ffi::c_int as isize);
                    i_0 = i_0.wrapping_add(1);
                }
                row_0 = row_0.offset((*rgb).rowBytes as isize);
                j_0 = j_0.wrapping_add(1);
            }
        } else if (*rgb).format as core::ffi::c_uint
            == AVIF_RGB_FORMAT_GRAYA as core::ffi::c_int as core::ffi::c_uint
        {
            let mut row_1: *mut uint8_t = (*rgb).pixels;
            let mut j_1: uint32_t = 0 as uint32_t;
            while j_1 < (*rgb).height {
                let mut pixel_1: *mut uint16_t = row_1 as *mut uint16_t;
                let mut i_1: uint32_t = 0 as uint32_t;
                while i_1 < (*rgb).width {
                    let mut a_1: uint16_t = *pixel_1
                        .offset(1 as core::ffi::c_int as isize);
                    if !(a_1 as uint32_t >= max) {
                        if a_1 as core::ffi::c_int == 0 as core::ffi::c_int {
                            *pixel_1.offset(0 as core::ffi::c_int as isize) = 0
                                as uint16_t;
                        } else {
                            let mut c1_1: core::ffi::c_float = avifRoundf(
                                *pixel_1.offset(0 as core::ffi::c_int as isize)
                                    as core::ffi::c_float * maxF / a_1 as core::ffi::c_float,
                            );
                            *pixel_1.offset(0 as core::ffi::c_int as isize) = (if c1_1
                                < maxF
                            {
                                c1_1
                            } else {
                                maxF
                            }) as uint16_t;
                        }
                    }
                    pixel_1 = pixel_1.offset(2 as core::ffi::c_int as isize);
                    i_1 = i_1.wrapping_add(1);
                }
                row_1 = row_1.offset((*rgb).rowBytes as isize);
                j_1 = j_1.wrapping_add(1);
            }
        } else if (*rgb).format as core::ffi::c_uint
            == AVIF_RGB_FORMAT_AGRAY as core::ffi::c_int as core::ffi::c_uint
        {
            let mut row_2: *mut uint8_t = (*rgb).pixels;
            let mut j_2: uint32_t = 0 as uint32_t;
            while j_2 < (*rgb).height {
                let mut pixel_2: *mut uint16_t = row_2 as *mut uint16_t;
                let mut i_2: uint32_t = 0 as uint32_t;
                while i_2 < (*rgb).width {
                    let mut a_2: uint16_t = *pixel_2
                        .offset(0 as core::ffi::c_int as isize);
                    if !(a_2 as uint32_t >= max) {
                        if a_2 as core::ffi::c_int == 0 as core::ffi::c_int {
                            *pixel_2.offset(1 as core::ffi::c_int as isize) = 0
                                as uint16_t;
                        } else {
                            let mut c1_2: core::ffi::c_float = avifRoundf(
                                *pixel_2.offset(1 as core::ffi::c_int as isize)
                                    as core::ffi::c_float * maxF / a_2 as core::ffi::c_float,
                            );
                            *pixel_2.offset(1 as core::ffi::c_int as isize) = (if c1_2
                                < maxF
                            {
                                c1_2
                            } else {
                                maxF
                            }) as uint16_t;
                        }
                    }
                    pixel_2 = pixel_2.offset(2 as core::ffi::c_int as isize);
                    i_2 = i_2.wrapping_add(1);
                }
                row_2 = row_2.offset((*rgb).rowBytes as isize);
                j_2 = j_2.wrapping_add(1);
            }
        } else {
            return AVIF_RESULT_NOT_IMPLEMENTED
        }
    } else if (*rgb).format as core::ffi::c_uint
        == AVIF_RGB_FORMAT_RGBA as core::ffi::c_int as core::ffi::c_uint
        || (*rgb).format as core::ffi::c_uint
            == AVIF_RGB_FORMAT_BGRA as core::ffi::c_int as core::ffi::c_uint
    {
        let mut row_3: *mut uint8_t = (*rgb).pixels;
        let mut j_3: uint32_t = 0 as uint32_t;
        while j_3 < (*rgb).height {
            let mut pixel_3: *mut uint8_t = row_3;
            let mut i_3: uint32_t = 0 as uint32_t;
            while i_3 < (*rgb).width {
                let mut a_3: uint8_t = *pixel_3.offset(3 as core::ffi::c_int as isize);
                if !(a_3 as uint32_t == max) {
                    if a_3 as core::ffi::c_int == 0 as core::ffi::c_int {
                        *pixel_3.offset(0 as core::ffi::c_int as isize) = 0 as uint8_t;
                        *pixel_3.offset(1 as core::ffi::c_int as isize) = 0 as uint8_t;
                        *pixel_3.offset(2 as core::ffi::c_int as isize) = 0 as uint8_t;
                    } else {
                        let mut c1_3: core::ffi::c_float = avifRoundf(
                            *pixel_3.offset(0 as core::ffi::c_int as isize)
                                as core::ffi::c_float * maxF / a_3 as core::ffi::c_float,
                        );
                        let mut c2_1: core::ffi::c_float = avifRoundf(
                            *pixel_3.offset(1 as core::ffi::c_int as isize)
                                as core::ffi::c_float * maxF / a_3 as core::ffi::c_float,
                        );
                        let mut c3_1: core::ffi::c_float = avifRoundf(
                            *pixel_3.offset(2 as core::ffi::c_int as isize)
                                as core::ffi::c_float * maxF / a_3 as core::ffi::c_float,
                        );
                        *pixel_3.offset(0 as core::ffi::c_int as isize) = (if c1_3 < maxF
                        {
                            c1_3
                        } else {
                            maxF
                        }) as uint8_t;
                        *pixel_3.offset(1 as core::ffi::c_int as isize) = (if c2_1 < maxF
                        {
                            c2_1
                        } else {
                            maxF
                        }) as uint8_t;
                        *pixel_3.offset(2 as core::ffi::c_int as isize) = (if c3_1 < maxF
                        {
                            c3_1
                        } else {
                            maxF
                        }) as uint8_t;
                    }
                }
                pixel_3 = pixel_3.offset(4 as core::ffi::c_int as isize);
                i_3 = i_3.wrapping_add(1);
            }
            row_3 = row_3.offset((*rgb).rowBytes as isize);
            j_3 = j_3.wrapping_add(1);
        }
    } else if (*rgb).format as core::ffi::c_uint
        == AVIF_RGB_FORMAT_ARGB as core::ffi::c_int as core::ffi::c_uint
        || (*rgb).format as core::ffi::c_uint
            == AVIF_RGB_FORMAT_ABGR as core::ffi::c_int as core::ffi::c_uint
    {
        let mut row_4: *mut uint8_t = (*rgb).pixels;
        let mut j_4: uint32_t = 0 as uint32_t;
        while j_4 < (*rgb).height {
            let mut pixel_4: *mut uint8_t = row_4;
            let mut i_4: uint32_t = 0 as uint32_t;
            while i_4 < (*rgb).width {
                let mut a_4: uint8_t = *pixel_4.offset(0 as core::ffi::c_int as isize);
                if !(a_4 as uint32_t == max) {
                    if a_4 as core::ffi::c_int == 0 as core::ffi::c_int {
                        *pixel_4.offset(1 as core::ffi::c_int as isize) = 0 as uint8_t;
                        *pixel_4.offset(2 as core::ffi::c_int as isize) = 0 as uint8_t;
                        *pixel_4.offset(3 as core::ffi::c_int as isize) = 0 as uint8_t;
                    } else {
                        let mut c1_4: core::ffi::c_float = avifRoundf(
                            *pixel_4.offset(1 as core::ffi::c_int as isize)
                                as core::ffi::c_float * maxF / a_4 as core::ffi::c_float,
                        );
                        let mut c2_2: core::ffi::c_float = avifRoundf(
                            *pixel_4.offset(2 as core::ffi::c_int as isize)
                                as core::ffi::c_float * maxF / a_4 as core::ffi::c_float,
                        );
                        let mut c3_2: core::ffi::c_float = avifRoundf(
                            *pixel_4.offset(3 as core::ffi::c_int as isize)
                                as core::ffi::c_float * maxF / a_4 as core::ffi::c_float,
                        );
                        *pixel_4.offset(1 as core::ffi::c_int as isize) = (if c1_4 < maxF
                        {
                            c1_4
                        } else {
                            maxF
                        }) as uint8_t;
                        *pixel_4.offset(2 as core::ffi::c_int as isize) = (if c2_2 < maxF
                        {
                            c2_2
                        } else {
                            maxF
                        }) as uint8_t;
                        *pixel_4.offset(3 as core::ffi::c_int as isize) = (if c3_2 < maxF
                        {
                            c3_2
                        } else {
                            maxF
                        }) as uint8_t;
                    }
                }
                pixel_4 = pixel_4.offset(4 as core::ffi::c_int as isize);
                i_4 = i_4.wrapping_add(1);
            }
            row_4 = row_4.offset((*rgb).rowBytes as isize);
            j_4 = j_4.wrapping_add(1);
        }
    } else if (*rgb).format as core::ffi::c_uint
        == AVIF_RGB_FORMAT_GRAYA as core::ffi::c_int as core::ffi::c_uint
    {
        let mut row_5: *mut uint8_t = (*rgb).pixels;
        let mut j_5: uint32_t = 0 as uint32_t;
        while j_5 < (*rgb).height {
            let mut pixel_5: *mut uint8_t = row_5;
            let mut i_5: uint32_t = 0 as uint32_t;
            while i_5 < (*rgb).width {
                let mut a_5: uint8_t = *pixel_5.offset(1 as core::ffi::c_int as isize);
                if !(a_5 as uint32_t == max) {
                    if a_5 as core::ffi::c_int == 0 as core::ffi::c_int {
                        *pixel_5.offset(0 as core::ffi::c_int as isize) = 0 as uint8_t;
                    } else {
                        let mut c1_5: core::ffi::c_float = avifRoundf(
                            *pixel_5.offset(0 as core::ffi::c_int as isize)
                                as core::ffi::c_float * maxF / a_5 as core::ffi::c_float,
                        );
                        *pixel_5.offset(0 as core::ffi::c_int as isize) = (if c1_5 < maxF
                        {
                            c1_5
                        } else {
                            maxF
                        }) as uint8_t;
                    }
                }
                pixel_5 = pixel_5.offset(2 as core::ffi::c_int as isize);
                i_5 = i_5.wrapping_add(1);
            }
            row_5 = row_5.offset((*rgb).rowBytes as isize);
            j_5 = j_5.wrapping_add(1);
        }
    } else if (*rgb).format as core::ffi::c_uint
        == AVIF_RGB_FORMAT_AGRAY as core::ffi::c_int as core::ffi::c_uint
    {
        let mut row_6: *mut uint8_t = (*rgb).pixels;
        let mut j_6: uint32_t = 0 as uint32_t;
        while j_6 < (*rgb).height {
            let mut pixel_6: *mut uint8_t = row_6;
            let mut i_6: uint32_t = 0 as uint32_t;
            while i_6 < (*rgb).width {
                let mut a_6: uint8_t = *pixel_6.offset(0 as core::ffi::c_int as isize);
                if !(a_6 as uint32_t == max) {
                    if a_6 as core::ffi::c_int == 0 as core::ffi::c_int {
                        *pixel_6.offset(1 as core::ffi::c_int as isize) = 0 as uint8_t;
                    } else {
                        let mut c1_6: core::ffi::c_float = avifRoundf(
                            *pixel_6.offset(1 as core::ffi::c_int as isize)
                                as core::ffi::c_float * maxF / a_6 as core::ffi::c_float,
                        );
                        *pixel_6.offset(1 as core::ffi::c_int as isize) = (if c1_6 < maxF
                        {
                            c1_6
                        } else {
                            maxF
                        }) as uint8_t;
                    }
                }
                pixel_6 = pixel_6.offset(2 as core::ffi::c_int as isize);
                i_6 = i_6.wrapping_add(1);
            }
            row_6 = row_6.offset((*rgb).rowBytes as isize);
            j_6 = j_6.wrapping_add(1);
        }
    } else {
        return AVIF_RESULT_NOT_IMPLEMENTED
    }
    return AVIF_RESULT_OK;
}
