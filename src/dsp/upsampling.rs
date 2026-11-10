// libwebp v1.6.0, src/dsp/upsampling.c:
//
// Copyright 2011 Google Inc. All Rights Reserved.
//
// Use of this source code is governed by a BSD-style license
// that can be found in the COPYING file in the root of the source
// tree. An additional intellectual property rights grant can be found
// in the file PATENTS. All contributing project authors may
// be found in the AUTHORS file in the root of the source tree.
// -----------------------------------------------------------------------------
//
// YUV to RGB upsampling functions.
//
// Author: somnath@google.com (Somnath Banerjee)

//! Translation of `src/dsp/upsampling.c` from libwebp v1.6.0.
//!
//! Work package: WP4 (pixel ops).

use crate::dsp::yuv::{
    vp8_yuv_to_argb, vp8_yuv_to_bgr, vp8_yuv_to_bgra, vp8_yuv_to_rgb, vp8_yuv_to_rgb565,
    vp8_yuv_to_rgba, vp8_yuv_to_rgba4444,
};
use crate::webp::decode::WEBP_CSP_MODE;

// upstream: src/dsp/dsp.h WebPUpsampleLinePairFunc
/// Converts one pair of luma rows, and the chroma rows that they share, to the
/// output colorspace. The bottom row is optional: `None` stands for a NULL
/// `bottom_y` with its `bottom_dst`, which C passes together (ADR 0003).
pub type WebPUpsampleLinePairFunc =
    fn(&[u8], Option<(&[u8], &mut [u8])>, &[u8], &[u8], &[u8], &[u8], &mut [u8], i32);

// upstream: src/dsp/dsp.h WebPYUV444Converter
/// Converts a row of YUV 4:4:4 samples to the output colorspace.
pub type WebPYUV444Converter = fn(&[u8], &[u8], &[u8], &mut [u8], i32);

// upstream: src/dsp/upsampling.c LOAD_UV
/// Packs a U sample in the low 16 bits and a V sample in the high 16 bits.
#[must_use]
fn load_uv(u: u8, v: u8) -> u32 {
    u32::from(u) | (u32::from(v) << 16)
}

// upstream: src/dsp/upsampling.c UPSAMPLE_FUNC
/// Converts one pair of rows, with fancy upsampling of the chroma. The
/// samples of the pair are interpolated diagonally, as the C macro does, and
/// `func` converts each output pixel with its luma and the interpolated U and
/// V. The chroma samples are packed in one `u32` per pixel, U in the low lane
/// and V in the high lane, and each shift keeps the lanes apart as the C code
/// does. The `# Panics` below apply to every slice of the pair.
///
/// # Panics
///
/// Panics when a slice is shorter than the samples or pixels that `len` needs.
/// `len` must be at least 1, as in C, where the first pixel is always read.
#[expect(
    clippy::too_many_arguments,
    reason = "mirrors the parameters of the UPSAMPLE_FUNC functions"
)]
#[expect(
    clippy::similar_names,
    reason = "the C names tl_uv, t_uv and uv of the packed samples"
)]
fn upsample_line_pair(
    top_y: &[u8],
    bottom: Option<(&[u8], &mut [u8])>,
    top_u: &[u8],
    top_v: &[u8],
    cur_u: &[u8],
    cur_v: &[u8],
    top_dst: &mut [u8],
    len: i32,
    xstep: usize,
    func: impl Fn(u8, u8, u8, &mut [u8]),
) {
    let len = usize::try_from(len).expect("the row length is not negative");
    // `len - 1 >> 1` in C: the number of pixel pairs after the first pixel.
    let last_pixel_pair = (len - 1) >> 1;
    let mut bottom = bottom;
    let mut tl_uv = load_uv(top_u[0], top_v[0]); // top-left sample
    let mut l_uv = load_uv(cur_u[0], cur_v[0]); // left-sample
    let emit = |y: u8, uv: u32, dst: &mut [u8]| {
        func(
            y,
            u8::try_from(uv & 0xff).expect("the low lane is a byte"),
            u8::try_from(uv >> 16).expect("the high lane is a byte"),
            dst,
        );
    };
    {
        let uv0 = (3 * tl_uv + l_uv + 0x0002_0002) >> 2;
        emit(top_y[0], uv0, &mut top_dst[..]);
    }
    if let Some((bottom_y, bottom_dst)) = bottom.as_mut() {
        let uv0 = (3 * l_uv + tl_uv + 0x0002_0002) >> 2;
        emit(bottom_y[0], uv0, &mut bottom_dst[..]);
    }
    for x in 1..=last_pixel_pair {
        let t_uv = load_uv(top_u[x], top_v[x]); // top sample
        let uv = load_uv(cur_u[x], cur_v[x]); // sample
        // precompute invariant values associated with first and second diagonals
        let avg = tl_uv + t_uv + l_uv + uv + 0x0008_0008;
        let diag_12 = (avg + 2 * (t_uv + l_uv)) >> 3;
        let diag_03 = (avg + 2 * (tl_uv + uv)) >> 3;
        {
            let uv0 = u32::midpoint(diag_12, tl_uv);
            let uv1 = u32::midpoint(diag_03, t_uv);
            emit(top_y[2 * x - 1], uv0, &mut top_dst[(2 * x - 1) * xstep..]);
            emit(top_y[2 * x], uv1, &mut top_dst[2 * x * xstep..]);
        }
        if let Some((bottom_y, bottom_dst)) = bottom.as_mut() {
            let uv0 = u32::midpoint(diag_03, l_uv);
            let uv1 = u32::midpoint(diag_12, uv);
            emit(
                bottom_y[2 * x - 1],
                uv0,
                &mut bottom_dst[(2 * x - 1) * xstep..],
            );
            emit(bottom_y[2 * x], uv1, &mut bottom_dst[2 * x * xstep..]);
        }
        tl_uv = t_uv;
        l_uv = uv;
    }
    if len & 1 == 0 {
        {
            let uv0 = (3 * tl_uv + l_uv + 0x0002_0002) >> 2;
            emit(top_y[len - 1], uv0, &mut top_dst[(len - 1) * xstep..]);
        }
        if let Some((bottom_y, bottom_dst)) = bottom.as_mut() {
            let uv0 = (3 * l_uv + tl_uv + 0x0002_0002) >> 2;
            emit(bottom_y[len - 1], uv0, &mut bottom_dst[(len - 1) * xstep..]);
        }
    }
}

#[expect(
    clippy::too_many_arguments,
    reason = "the parameters of WebPUpsampleLinePairFunc"
)]
// upstream: src/dsp/upsampling.c UpsampleRgbaLinePair_C
fn upsample_rgba_line_pair_c(
    top_y: &[u8],
    bottom: Option<(&[u8], &mut [u8])>,
    top_u: &[u8],
    top_v: &[u8],
    cur_u: &[u8],
    cur_v: &[u8],
    top_dst: &mut [u8],
    len: i32,
) {
    upsample_line_pair(
        top_y,
        bottom,
        top_u,
        top_v,
        cur_u,
        cur_v,
        top_dst,
        len,
        4,
        vp8_yuv_to_rgba,
    );
}

#[expect(
    clippy::too_many_arguments,
    reason = "the parameters of WebPUpsampleLinePairFunc"
)]
// upstream: src/dsp/upsampling.c UpsampleBgraLinePair_C
fn upsample_bgra_line_pair_c(
    top_y: &[u8],
    bottom: Option<(&[u8], &mut [u8])>,
    top_u: &[u8],
    top_v: &[u8],
    cur_u: &[u8],
    cur_v: &[u8],
    top_dst: &mut [u8],
    len: i32,
) {
    upsample_line_pair(
        top_y,
        bottom,
        top_u,
        top_v,
        cur_u,
        cur_v,
        top_dst,
        len,
        4,
        vp8_yuv_to_bgra,
    );
}

#[expect(
    clippy::too_many_arguments,
    reason = "the parameters of WebPUpsampleLinePairFunc"
)]
// upstream: src/dsp/upsampling.c UpsampleArgbLinePair_C
fn upsample_argb_line_pair_c(
    top_y: &[u8],
    bottom: Option<(&[u8], &mut [u8])>,
    top_u: &[u8],
    top_v: &[u8],
    cur_u: &[u8],
    cur_v: &[u8],
    top_dst: &mut [u8],
    len: i32,
) {
    upsample_line_pair(
        top_y,
        bottom,
        top_u,
        top_v,
        cur_u,
        cur_v,
        top_dst,
        len,
        4,
        vp8_yuv_to_argb,
    );
}

#[expect(
    clippy::too_many_arguments,
    reason = "the parameters of WebPUpsampleLinePairFunc"
)]
// upstream: src/dsp/upsampling.c UpsampleRgbLinePair_C
fn upsample_rgb_line_pair_c(
    top_y: &[u8],
    bottom: Option<(&[u8], &mut [u8])>,
    top_u: &[u8],
    top_v: &[u8],
    cur_u: &[u8],
    cur_v: &[u8],
    top_dst: &mut [u8],
    len: i32,
) {
    upsample_line_pair(
        top_y,
        bottom,
        top_u,
        top_v,
        cur_u,
        cur_v,
        top_dst,
        len,
        3,
        |y, u, v, dst| {
            vp8_yuv_to_rgb(i32::from(y), i32::from(u), i32::from(v), dst);
        },
    );
}

#[expect(
    clippy::too_many_arguments,
    reason = "the parameters of WebPUpsampleLinePairFunc"
)]
// upstream: src/dsp/upsampling.c UpsampleBgrLinePair_C
fn upsample_bgr_line_pair_c(
    top_y: &[u8],
    bottom: Option<(&[u8], &mut [u8])>,
    top_u: &[u8],
    top_v: &[u8],
    cur_u: &[u8],
    cur_v: &[u8],
    top_dst: &mut [u8],
    len: i32,
) {
    upsample_line_pair(
        top_y,
        bottom,
        top_u,
        top_v,
        cur_u,
        cur_v,
        top_dst,
        len,
        3,
        |y, u, v, dst| {
            vp8_yuv_to_bgr(i32::from(y), i32::from(u), i32::from(v), dst);
        },
    );
}

#[expect(
    clippy::too_many_arguments,
    reason = "the parameters of WebPUpsampleLinePairFunc"
)]
// upstream: src/dsp/upsampling.c UpsampleRgba4444LinePair_C
fn upsample_rgba4444_line_pair_c(
    top_y: &[u8],
    bottom: Option<(&[u8], &mut [u8])>,
    top_u: &[u8],
    top_v: &[u8],
    cur_u: &[u8],
    cur_v: &[u8],
    top_dst: &mut [u8],
    len: i32,
) {
    upsample_line_pair(
        top_y,
        bottom,
        top_u,
        top_v,
        cur_u,
        cur_v,
        top_dst,
        len,
        2,
        |y, u, v, dst| {
            vp8_yuv_to_rgba4444(i32::from(y), i32::from(u), i32::from(v), dst);
        },
    );
}

#[expect(
    clippy::too_many_arguments,
    reason = "the parameters of WebPUpsampleLinePairFunc"
)]
// upstream: src/dsp/upsampling.c UpsampleRgb565LinePair_C
fn upsample_rgb565_line_pair_c(
    top_y: &[u8],
    bottom: Option<(&[u8], &mut [u8])>,
    top_u: &[u8],
    top_v: &[u8],
    cur_u: &[u8],
    cur_v: &[u8],
    top_dst: &mut [u8],
    len: i32,
) {
    upsample_line_pair(
        top_y,
        bottom,
        top_u,
        top_v,
        cur_u,
        cur_v,
        top_dst,
        len,
        2,
        |y, u, v, dst| {
            vp8_yuv_to_rgb565(i32::from(y), i32::from(u), i32::from(v), dst);
        },
    );
}

// upstream: src/dsp/upsampling.c WebPUpsamplers
// The table is filled by `WebPInitUpsamplers`, whose portable effect is to
// assign these pointers; its SIMD branches are not compiled in the reference
// configuration (ADR 0002). MODE_YUV and MODE_YUVA have no upsampler.
static WEBP_UPSAMPLERS: [Option<WebPUpsampleLinePairFunc>; 13] = [
    Some(upsample_rgb_line_pair_c),      // MODE_RGB
    Some(upsample_rgba_line_pair_c),     // MODE_RGBA
    Some(upsample_bgr_line_pair_c),      // MODE_BGR
    Some(upsample_bgra_line_pair_c),     // MODE_BGRA
    Some(upsample_argb_line_pair_c),     // MODE_ARGB
    Some(upsample_rgba4444_line_pair_c), // MODE_RGBA_4444
    Some(upsample_rgb565_line_pair_c),   // MODE_RGB_565
    Some(upsample_rgba_line_pair_c),     // MODE_rgbA
    Some(upsample_bgra_line_pair_c),     // MODE_bgrA
    Some(upsample_argb_line_pair_c),     // MODE_Argb
    Some(upsample_rgba4444_line_pair_c), // MODE_rgbA_4444
    None,                                // MODE_YUV
    None,                                // MODE_YUVA
];

// upstream: src/dsp/upsampling.c WebPUpsamplers
/// Converts one pair of rows with the upsampler of `mode`,
/// `WebPUpsamplers[mode]`.
///
/// # Panics
///
/// Panics for `MODE_YUV` and `MODE_YUVA`, which have no upsampler (NULL in C).
#[expect(
    clippy::too_many_arguments,
    reason = "mirrors the parameters of WebPUpsampleLinePairFunc, with the mode first"
)]
pub fn webp_upsamplers(
    mode: WEBP_CSP_MODE,
    top_y: &[u8],
    bottom: Option<(&[u8], &mut [u8])>,
    top_u: &[u8],
    top_v: &[u8],
    cur_u: &[u8],
    cur_v: &[u8],
    top_dst: &mut [u8],
    len: i32,
) {
    #[cfg(all(feature = "simd", any(target_arch = "x86", target_arch = "x86_64")))]
    {
        let features = crate::dsp::cpu::features();
        if features.sse4_1 {
            match mode {
                WEBP_CSP_MODE::MODE_RGB => {
                    return crate::dsp::upsampling_sse41::upsample_rgb_line_pair(
                        top_y, bottom, top_u, top_v, cur_u, cur_v, top_dst, len,
                    );
                }
                WEBP_CSP_MODE::MODE_BGR => {
                    return crate::dsp::upsampling_sse41::upsample_bgr_line_pair(
                        top_y, bottom, top_u, top_v, cur_u, cur_v, top_dst, len,
                    );
                }
                _ => {}
            }
        }
        if features.sse2 {
            match mode {
                WEBP_CSP_MODE::MODE_RGBA | WEBP_CSP_MODE::MODE_rgbA => {
                    return crate::dsp::upsampling_sse2::upsample_rgba_line_pair(
                        top_y, bottom, top_u, top_v, cur_u, cur_v, top_dst, len,
                    );
                }
                WEBP_CSP_MODE::MODE_BGRA | WEBP_CSP_MODE::MODE_bgrA => {
                    return crate::dsp::upsampling_sse2::upsample_bgra_line_pair(
                        top_y, bottom, top_u, top_v, cur_u, cur_v, top_dst, len,
                    );
                }
                WEBP_CSP_MODE::MODE_RGB => {
                    return crate::dsp::upsampling_sse2::upsample_rgb_line_pair(
                        top_y, bottom, top_u, top_v, cur_u, cur_v, top_dst, len,
                    );
                }
                WEBP_CSP_MODE::MODE_BGR => {
                    return crate::dsp::upsampling_sse2::upsample_bgr_line_pair(
                        top_y, bottom, top_u, top_v, cur_u, cur_v, top_dst, len,
                    );
                }
                WEBP_CSP_MODE::MODE_ARGB | WEBP_CSP_MODE::MODE_Argb => {
                    return crate::dsp::upsampling_sse2::upsample_argb_line_pair(
                        top_y, bottom, top_u, top_v, cur_u, cur_v, top_dst, len,
                    );
                }
                WEBP_CSP_MODE::MODE_RGB_565 => {
                    return crate::dsp::upsampling_sse2::upsample_rgb565_line_pair(
                        top_y, bottom, top_u, top_v, cur_u, cur_v, top_dst, len,
                    );
                }
                WEBP_CSP_MODE::MODE_RGBA_4444 | WEBP_CSP_MODE::MODE_rgbA_4444 => {
                    return crate::dsp::upsampling_sse2::upsample_rgba4444_line_pair(
                        top_y, bottom, top_u, top_v, cur_u, cur_v, top_dst, len,
                    );
                }
                _ => {}
            }
        }
    }
    #[allow(
        unreachable_code,
        reason = "the C path is reached only without a SIMD variant"
    )]
    let upsampler = WEBP_UPSAMPLERS[mode.0 as usize].expect("YUV modes have no upsampler");
    upsampler(top_y, bottom, top_u, top_v, cur_u, cur_v, top_dst, len);
}

// upstream: src/dsp/upsampling.c WebPGetLinePairConverter
/// Returns the upsampler for ARGB-like output (`alpha_is_last` false) or for
/// RGBA-like output (`alpha_is_last` true). The C function first runs
/// `WebPInitUpsamplers`, whose only effect is the table itself (ADR 0003).
///
/// # Panics
///
/// Never panics: the BGRA and ARGB entries of the table are set.
#[must_use]
pub fn webp_get_line_pair_converter(alpha_is_last: bool) -> WebPUpsampleLinePairFunc {
    #[cfg(all(feature = "simd", any(target_arch = "x86", target_arch = "x86_64")))]
    {
        // `WebPInitUpsamplersSSE41` assigns neither entry.
        let features = crate::dsp::cpu::features();
        if features.sse2 {
            return if alpha_is_last {
                crate::dsp::upsampling_sse2::upsample_bgra_line_pair
            } else {
                crate::dsp::upsampling_sse2::upsample_argb_line_pair
            };
        }
    }
    #[allow(
        unreachable_code,
        reason = "the C path is reached only without a SIMD variant"
    )]
    let mode = if alpha_is_last {
        WEBP_CSP_MODE::MODE_BGRA
    } else {
        WEBP_CSP_MODE::MODE_ARGB
    };
    WEBP_UPSAMPLERS[mode.0 as usize].expect("the ARGB and BGRA upsamplers exist")
}

// upstream: src/dsp/upsampling.c YUV444_FUNC
/// Converts a row of YUV 4:4:4 samples, one output pixel per sample, with
/// `func` as the per-pixel conversion.
fn yuv444_row(
    y: &[u8],
    u: &[u8],
    v: &[u8],
    dst: &mut [u8],
    len: i32,
    xstep: usize,
    func: impl Fn(u8, u8, u8, &mut [u8]),
) {
    let len = usize::try_from(len).expect("the row length is not negative");
    for i in 0..len {
        func(y[i], u[i], v[i], &mut dst[i * xstep..]);
    }
}

// upstream: src/dsp/upsampling.c WebPYuv444ToRgba_C
pub(crate) fn webp_yuv444_to_rgba_c(y: &[u8], u: &[u8], v: &[u8], dst: &mut [u8], len: i32) {
    yuv444_row(y, u, v, dst, len, 4, vp8_yuv_to_rgba);
}

// upstream: src/dsp/upsampling.c WebPYuv444ToBgra_C
pub(crate) fn webp_yuv444_to_bgra_c(y: &[u8], u: &[u8], v: &[u8], dst: &mut [u8], len: i32) {
    yuv444_row(y, u, v, dst, len, 4, vp8_yuv_to_bgra);
}

// upstream: src/dsp/upsampling.c WebPYuv444ToArgb_C
pub(crate) fn webp_yuv444_to_argb_c(y: &[u8], u: &[u8], v: &[u8], dst: &mut [u8], len: i32) {
    yuv444_row(y, u, v, dst, len, 4, vp8_yuv_to_argb);
}

// upstream: src/dsp/upsampling.c WebPYuv444ToRgb_C
pub(crate) fn webp_yuv444_to_rgb_c(y: &[u8], u: &[u8], v: &[u8], dst: &mut [u8], len: i32) {
    yuv444_row(y, u, v, dst, len, 3, |y, u, v, out| {
        vp8_yuv_to_rgb(i32::from(y), i32::from(u), i32::from(v), out);
    });
}

// upstream: src/dsp/upsampling.c WebPYuv444ToBgr_C
pub(crate) fn webp_yuv444_to_bgr_c(y: &[u8], u: &[u8], v: &[u8], dst: &mut [u8], len: i32) {
    yuv444_row(y, u, v, dst, len, 3, |y, u, v, out| {
        vp8_yuv_to_bgr(i32::from(y), i32::from(u), i32::from(v), out);
    });
}

// upstream: src/dsp/upsampling.c WebPYuv444ToRgba4444_C
pub(crate) fn webp_yuv444_to_rgba4444_c(y: &[u8], u: &[u8], v: &[u8], dst: &mut [u8], len: i32) {
    yuv444_row(y, u, v, dst, len, 2, |y, u, v, out| {
        vp8_yuv_to_rgba4444(i32::from(y), i32::from(u), i32::from(v), out);
    });
}

// upstream: src/dsp/upsampling.c WebPYuv444ToRgb565_C
pub(crate) fn webp_yuv444_to_rgb565_c(y: &[u8], u: &[u8], v: &[u8], dst: &mut [u8], len: i32) {
    yuv444_row(y, u, v, dst, len, 2, |y, u, v, out| {
        vp8_yuv_to_rgb565(i32::from(y), i32::from(u), i32::from(v), out);
    });
}

// upstream: src/dsp/upsampling.c WebPYUV444Converters
// The table is filled by `WebPInitYUV444Converters`, which assigns these
// pointers only (ADR 0003). MODE_YUV and MODE_YUVA have no converter.
static WEBP_YUV444_CONVERTERS: [Option<WebPYUV444Converter>; 13] = [
    Some(webp_yuv444_to_rgb_c),      // MODE_RGB
    Some(webp_yuv444_to_rgba_c),     // MODE_RGBA
    Some(webp_yuv444_to_bgr_c),      // MODE_BGR
    Some(webp_yuv444_to_bgra_c),     // MODE_BGRA
    Some(webp_yuv444_to_argb_c),     // MODE_ARGB
    Some(webp_yuv444_to_rgba4444_c), // MODE_RGBA_4444
    Some(webp_yuv444_to_rgb565_c),   // MODE_RGB_565
    Some(webp_yuv444_to_rgba_c),     // MODE_rgbA
    Some(webp_yuv444_to_bgra_c),     // MODE_bgrA
    Some(webp_yuv444_to_argb_c),     // MODE_Argb
    Some(webp_yuv444_to_rgba4444_c), // MODE_rgbA_4444
    None,                            // MODE_YUV
    None,                            // MODE_YUVA
];

// upstream: src/dsp/upsampling.c WebPYUV444Converters
/// Converts a row of YUV 4:4:4 samples with `WebPYUV444Converters[mode]`.
///
/// # Panics
///
/// Panics for `MODE_YUV` and `MODE_YUVA`, which have no converter (NULL in C).
pub fn webp_yuv444_converters(
    mode: WEBP_CSP_MODE,
    y: &[u8],
    u: &[u8],
    v: &[u8],
    dst: &mut [u8],
    len: i32,
) {
    #[cfg(all(feature = "simd", any(target_arch = "x86", target_arch = "x86_64")))]
    {
        let features = crate::dsp::cpu::features();
        if features.sse4_1 {
            match mode {
                WEBP_CSP_MODE::MODE_RGB => {
                    return crate::dsp::upsampling_sse41::yuv444_to_rgb(y, u, v, dst, len);
                }
                WEBP_CSP_MODE::MODE_BGR => {
                    return crate::dsp::upsampling_sse41::yuv444_to_bgr(y, u, v, dst, len);
                }
                _ => {}
            }
        }
        if features.sse2 {
            match mode {
                WEBP_CSP_MODE::MODE_RGBA | WEBP_CSP_MODE::MODE_rgbA => {
                    return crate::dsp::upsampling_sse2::yuv444_to_rgba(y, u, v, dst, len);
                }
                WEBP_CSP_MODE::MODE_BGRA | WEBP_CSP_MODE::MODE_bgrA => {
                    return crate::dsp::upsampling_sse2::yuv444_to_bgra(y, u, v, dst, len);
                }
                WEBP_CSP_MODE::MODE_RGB => {
                    return crate::dsp::upsampling_sse2::yuv444_to_rgb(y, u, v, dst, len);
                }
                WEBP_CSP_MODE::MODE_BGR => {
                    return crate::dsp::upsampling_sse2::yuv444_to_bgr(y, u, v, dst, len);
                }
                WEBP_CSP_MODE::MODE_ARGB | WEBP_CSP_MODE::MODE_Argb => {
                    return crate::dsp::upsampling_sse2::yuv444_to_argb(y, u, v, dst, len);
                }
                WEBP_CSP_MODE::MODE_RGB_565 => {
                    return crate::dsp::upsampling_sse2::yuv444_to_rgb565(y, u, v, dst, len);
                }
                WEBP_CSP_MODE::MODE_RGBA_4444 | WEBP_CSP_MODE::MODE_rgbA_4444 => {
                    return crate::dsp::upsampling_sse2::yuv444_to_rgba4444(y, u, v, dst, len);
                }
                _ => {}
            }
        }
    }
    #[allow(
        unreachable_code,
        reason = "the C path is reached only without a SIMD variant"
    )]
    let converter = WEBP_YUV444_CONVERTERS[mode.0 as usize].expect("YUV modes have no converter");
    converter(y, u, v, dst, len);
}
