// libwebp v1.6.0, src/dsp/yuv.c:
//
// Copyright 2010 Google Inc. All Rights Reserved.
//
// Use of this source code is governed by a BSD-style license
// that can be found in the COPYING file in the root of the source
// tree. An additional intellectual property rights grant can be found
// in the file PATENTS. All contributing project authors may
// be found in the AUTHORS file in the root of the source tree.
// -----------------------------------------------------------------------------
//
// YUV->RGB conversion functions
//
// Author: Skal (pascal.massimino@gmail.com)

// libwebp v1.6.0, src/dsp/yuv.h:
//
// Copyright 2010 Google Inc. All Rights Reserved.
//
// Use of this source code is governed by a BSD-style license
// that can be found in the COPYING file in the root of the source
// tree. An additional intellectual property rights grant can be found
// in the file PATENTS. All contributing project authors may
// be found in the AUTHORS file in the root of the source tree.
// -----------------------------------------------------------------------------
//
// inline YUV<->RGB conversion function
//
// The exact naming is Y'CbCr, following the ITU-R BT.601 standard.
// More information at: https://en.wikipedia.org/wiki/YCbCr
// Y = 0.2568 * R + 0.5041 * G + 0.0979 * B + 16
// U = -0.1482 * R - 0.2910 * G + 0.4392 * B + 128
// V = 0.4392 * R - 0.3678 * G - 0.0714 * B + 128
// We use 16bit fixed point operations for RGB->YUV conversion (YUV_FIX).
//
// For the Y'CbCr to RGB conversion, the BT.601 specification reads:
//   R = 1.164 * (Y-16) + 1.596 * (V-128)
//   G = 1.164 * (Y-16) - 0.813 * (V-128) - 0.392 * (U-128)
//   B = 1.164 * (Y-16)                   + 2.017 * (U-128)
// where Y is in the [16,235] range, and U/V in the [16,240] range.
//
// The fixed-point implementation used here is:
//  R = (19077 . y             + 26149 . v - 14234) >> 6
//  G = (19077 . y -  6419 . u - 13320 . v +  8708) >> 6
//  B = (19077 . y + 33050 . u             - 17685) >> 6
// where the '.' operator is the mulhi_epu16 variant:
//   a . b = ((a << 8) * b) >> 16
// that preserves 8 bits of fractional precision before final descaling.

//! Translation of `src/dsp/yuv.c` and `src/dsp/yuv.h` from libwebp v1.6.0.
//!
//! Work package: WP4 (pixel ops).

use crate::webp::decode::WEBP_CSP_MODE;

/// Stores an `int` in a `uint8_t`: the value keeps its low eight bits, as the
/// C conversion does.
#[expect(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    reason = "C stores an int in a uint8_t, which keeps the low eight bits"
)]
#[must_use]
pub fn store_u8(value: i32) -> u8 {
    value as u8
}

// upstream: src/dsp/yuv.h YUV_FIX
/// Fixed-point precision for the RGB to YUV conversion.
pub const YUV_FIX: i32 = 16;
// upstream: src/dsp/yuv.h YUV_HALF
/// Rounding term of the RGB to YUV conversion.
pub const YUV_HALF: i32 = 1 << (YUV_FIX - 1);
// upstream: src/dsp/yuv.h YUV_FIX2
/// Fixed-point precision for the YUV to RGB conversion.
pub const YUV_FIX2: i32 = 6;
// upstream: src/dsp/yuv.h YUV_MASK2
/// Mask of the values that `vp8_clip8` leaves unclipped.
pub const YUV_MASK2: i32 = (256 << YUV_FIX2) - 1;

// upstream: src/dsp/yuv.h MultHi
/// `_mm_mulhi_epu16` emulation: the product scaled down by 2^8.
#[must_use]
pub fn mult_hi(v: i32, coeff: i32) -> i32 {
    (v * coeff) >> 8
}

// upstream: src/dsp/yuv.h VP8Clip8
/// Clips the fixed-point value to a byte, descaled by `YUV_FIX2`.
#[must_use]
pub fn vp8_clip8(v: i32) -> i32 {
    if (v & !YUV_MASK2) == 0 {
        v >> YUV_FIX2
    } else if v < 0 {
        0
    } else {
        255
    }
}

// upstream: src/dsp/yuv.h VP8YUVToR
/// Red component of a YUV sample.
#[must_use]
pub fn vp8yuv_to_r(y: i32, v: i32) -> i32 {
    vp8_clip8(mult_hi(y, 19077) + mult_hi(v, 26149) - 14234)
}

// upstream: src/dsp/yuv.h VP8YUVToG
/// Green component of a YUV sample.
#[must_use]
pub fn vp8yuv_to_g(y: i32, u: i32, v: i32) -> i32 {
    vp8_clip8(mult_hi(y, 19077) - mult_hi(u, 6419) - mult_hi(v, 13320) + 8708)
}

// upstream: src/dsp/yuv.h VP8YUVToB
/// Blue component of a YUV sample.
#[must_use]
pub fn vp8yuv_to_b(y: i32, u: i32) -> i32 {
    vp8_clip8(mult_hi(y, 19077) + mult_hi(u, 33050) - 17685)
}

// upstream: src/dsp/yuv.h VP8YuvToRgb
/// Writes the RGB bytes of one YUV sample to `rgb[0..3]`.
pub fn vp8_yuv_to_rgb(y: i32, u: i32, v: i32, rgb: &mut [u8]) {
    rgb[0] = store_u8(vp8yuv_to_r(y, v));
    rgb[1] = store_u8(vp8yuv_to_g(y, u, v));
    rgb[2] = store_u8(vp8yuv_to_b(y, u));
}

// upstream: src/dsp/yuv.h VP8YuvToBgr
/// Writes the BGR bytes of one YUV sample to `bgr[0..3]`.
pub fn vp8_yuv_to_bgr(y: i32, u: i32, v: i32, bgr: &mut [u8]) {
    bgr[0] = store_u8(vp8yuv_to_b(y, u));
    bgr[1] = store_u8(vp8yuv_to_g(y, u, v));
    bgr[2] = store_u8(vp8yuv_to_r(y, v));
}

// upstream: src/dsp/yuv.h VP8YuvToRgb565
/// Writes one YUV sample as RGB-565 to `rgb[0..2]`, in the order of
/// `WEBP_SWAP_16BIT_CSP == 0`, which is the reference configuration.
#[expect(
    clippy::many_single_char_names,
    reason = "the YUV and RGB component names of the C source"
)]
pub fn vp8_yuv_to_rgb565(y: i32, u: i32, v: i32, rgb: &mut [u8]) {
    let r = vp8yuv_to_r(y, v); // 5 usable bits
    let g = vp8yuv_to_g(y, u, v); // 6 usable bits
    let b = vp8yuv_to_b(y, u); // 5 usable bits
    let rg = (r & 0xf8) | (g >> 5);
    let gb = ((g << 3) & 0xe0) | (b >> 3);
    rgb[0] = store_u8(rg);
    rgb[1] = store_u8(gb);
}

// upstream: src/dsp/yuv.h VP8YuvToRgba4444
/// Writes one YUV sample as RGBA-4444 to `argb[0..2]`, in the order of
/// `WEBP_SWAP_16BIT_CSP == 0`, which is the reference configuration.
#[expect(
    clippy::many_single_char_names,
    reason = "the YUV and RGB component names of the C source"
)]
pub fn vp8_yuv_to_rgba4444(y: i32, u: i32, v: i32, argb: &mut [u8]) {
    let r = vp8yuv_to_r(y, v); // 4 usable bits
    let g = vp8yuv_to_g(y, u, v); // 4 usable bits
    let b = vp8yuv_to_b(y, u); // 4 usable bits
    let rg = (r & 0xf0) | (g >> 4);
    let ba = (b & 0xf0) | 0x0f; // overwrite the lower 4 bits
    argb[0] = store_u8(rg);
    argb[1] = store_u8(ba);
}

// upstream: src/dsp/yuv.h VP8YuvToArgb
/// Writes one YUV sample as ARGB with opaque alpha to `argb[0..4]`.
pub fn vp8_yuv_to_argb(y: u8, u: u8, v: u8, argb: &mut [u8]) {
    argb[0] = 0xff;
    vp8_yuv_to_rgb(i32::from(y), i32::from(u), i32::from(v), &mut argb[1..]);
}

// upstream: src/dsp/yuv.h VP8YuvToBgra
/// Writes one YUV sample as BGRA with opaque alpha to `bgra[0..4]`.
pub fn vp8_yuv_to_bgra(y: u8, u: u8, v: u8, bgra: &mut [u8]) {
    vp8_yuv_to_bgr(i32::from(y), i32::from(u), i32::from(v), bgra);
    bgra[3] = 0xff;
}

// upstream: src/dsp/yuv.h VP8YuvToRgba
/// Writes one YUV sample as RGBA with opaque alpha to `rgba[0..4]`.
pub fn vp8_yuv_to_rgba(y: u8, u: u8, v: u8, rgba: &mut [u8]) {
    vp8_yuv_to_rgb(i32::from(y), i32::from(u), i32::from(v), rgba);
    rgba[3] = 0xff;
}

// upstream: src/dsp/yuv.h VP8ClipUV
/// Descales a chroma sum with its rounding term and clips it to a byte.
#[must_use]
pub fn vp8_clip_uv(uv: i32, rounding: i32) -> i32 {
    let uv = (uv + rounding + (128 << (YUV_FIX + 2))) >> (YUV_FIX + 2);
    if (uv & !0xff) == 0 {
        uv
    } else if uv < 0 {
        0
    } else {
        255
    }
}

// upstream: src/dsp/yuv.h VP8RGBToY
/// Luma of an RGB sample. The result needs no clipping.
#[must_use]
pub fn vp8rgb_to_y(r: i32, g: i32, b: i32, rounding: i32) -> i32 {
    let luma = 16839 * r + 33059 * g + 6420 * b;
    (luma + rounding + (16 << YUV_FIX)) >> YUV_FIX // no need to clip
}

// upstream: src/dsp/yuv.h VP8RGBToU
/// Blue-difference chroma of an RGB sample.
#[must_use]
pub fn vp8rgb_to_u(r: i32, g: i32, b: i32, rounding: i32) -> i32 {
    let u = -9719 * r - 19081 * g + 28800 * b;
    vp8_clip_uv(u, rounding)
}

// upstream: src/dsp/yuv.h VP8RGBToV
/// Red-difference chroma of an RGB sample.
#[must_use]
pub fn vp8rgb_to_v(r: i32, g: i32, b: i32, rounding: i32) -> i32 {
    let v = 28800 * r - 24116 * g - 4684 * b;
    vp8_clip_uv(v, rounding)
}

// upstream: src/dsp/yuv.c ROW_FUNC
/// Converts `len` pixels, two per step, from `y`, `u` and `v` into `dst`, which
/// takes `xstep` bytes per pixel. Each pair shares one chroma sample, and an
/// odd last pixel is converted alone. `func` is the per-pixel conversion.
fn row_func(
    y: &[u8],
    u: &[u8],
    v: &[u8],
    dst: &mut [u8],
    len: i32,
    xstep: usize,
    func: impl Fn(u8, u8, u8, &mut [u8]),
) {
    let len = usize::try_from(len).expect("the row length is not negative");
    let mut y_pos = 0;
    let mut uv_pos = 0;
    let mut dst_pos = 0;
    for _ in 0..len / 2 {
        func(y[y_pos], u[uv_pos], v[uv_pos], &mut dst[dst_pos..]);
        func(
            y[y_pos + 1],
            u[uv_pos],
            v[uv_pos],
            &mut dst[dst_pos + xstep..],
        );
        y_pos += 2;
        uv_pos += 1;
        dst_pos += 2 * xstep;
    }
    if len & 1 != 0 {
        func(y[y_pos], u[uv_pos], v[uv_pos], &mut dst[dst_pos..]);
    }
}

// upstream: src/dsp/yuv.c YuvToRgbRow
fn yuv_to_rgb_row(y: &[u8], u: &[u8], v: &[u8], dst: &mut [u8], len: i32) {
    row_func(y, u, v, dst, len, 3, |y, u, v, out| {
        vp8_yuv_to_rgb(i32::from(y), i32::from(u), i32::from(v), out);
    });
}

// upstream: src/dsp/yuv.c YuvToBgrRow
fn yuv_to_bgr_row(y: &[u8], u: &[u8], v: &[u8], dst: &mut [u8], len: i32) {
    row_func(y, u, v, dst, len, 3, |y, u, v, out| {
        vp8_yuv_to_bgr(i32::from(y), i32::from(u), i32::from(v), out);
    });
}

// upstream: src/dsp/yuv.c YuvToRgbaRow
fn yuv_to_rgba_row(y: &[u8], u: &[u8], v: &[u8], dst: &mut [u8], len: i32) {
    row_func(y, u, v, dst, len, 4, vp8_yuv_to_rgba);
}

// upstream: src/dsp/yuv.c YuvToBgraRow
fn yuv_to_bgra_row(y: &[u8], u: &[u8], v: &[u8], dst: &mut [u8], len: i32) {
    row_func(y, u, v, dst, len, 4, vp8_yuv_to_bgra);
}

// upstream: src/dsp/yuv.c YuvToArgbRow
fn yuv_to_argb_row(y: &[u8], u: &[u8], v: &[u8], dst: &mut [u8], len: i32) {
    row_func(y, u, v, dst, len, 4, vp8_yuv_to_argb);
}

// upstream: src/dsp/yuv.c YuvToRgba4444Row
fn yuv_to_rgba4444_row(y: &[u8], u: &[u8], v: &[u8], dst: &mut [u8], len: i32) {
    row_func(y, u, v, dst, len, 2, |y, u, v, out| {
        vp8_yuv_to_rgba4444(i32::from(y), i32::from(u), i32::from(v), out);
    });
}

// upstream: src/dsp/yuv.c YuvToRgb565Row
fn yuv_to_rgb565_row(y: &[u8], u: &[u8], v: &[u8], dst: &mut [u8], len: i32) {
    row_func(y, u, v, dst, len, 2, |y, u, v, out| {
        vp8_yuv_to_rgb565(i32::from(y), i32::from(u), i32::from(v), out);
    });
}

// upstream: src/dsp/dsp.h WebPSamplerRowFunc
/// A row sampler: converts `len` pixels from one row of `y`, `u` and `v` into
/// `dst`.
pub type WebPSamplerRowFunc = fn(&[u8], &[u8], &[u8], &mut [u8], i32);

// upstream: src/dsp/yuv.c WebPSamplerProcessPlane
/// Applies `func` to `height` rows of a plane. The chroma rows advance after
/// every odd row, as the C loop does. Strides are signed, and each row start is
/// converted to an offset with a checked conversion.
#[expect(
    clippy::too_many_arguments,
    reason = "mirrors the parameters of WebPSamplerProcessPlane"
)]
/// # Panics
///
/// Panics when a row or a stride addresses a sample outside its slice, which
/// the C code reads or writes out of bounds (PORTING rule 8).
pub fn webp_sampler_process_plane(
    y: &[u8],
    y_stride: i32,
    u: &[u8],
    v: &[u8],
    uv_stride: i32,
    dst: &mut [u8],
    dst_stride: i32,
    width: i32,
    height: i32,
    func: impl Fn(&[u8], &[u8], &[u8], &mut [u8], i32),
) {
    let row = |offset: isize| usize::try_from(offset).expect("a row starts inside the buffer");
    let mut y_offset = 0_isize;
    let mut uv_offset = 0_isize;
    let mut dst_offset = 0_isize;
    for j in 0..height {
        func(
            &y[row(y_offset)..],
            &u[row(uv_offset)..],
            &v[row(uv_offset)..],
            &mut dst[row(dst_offset)..],
            width,
        );
        y_offset += isize::try_from(y_stride).expect("the stride fits in isize");
        if j & 1 != 0 {
            uv_offset += isize::try_from(uv_stride).expect("the stride fits in isize");
        }
        dst_offset += isize::try_from(dst_stride).expect("the stride fits in isize");
    }
}

// upstream: src/dsp/yuv.c WebPSamplers
// The table is filled by `WebPInitSamplers`, whose only portable effect is to
// assign these pointers. Its SIMD branches are not compiled in the reference
// configuration (ADR 0002), so the table holds the portable row functions.
// The YUV modes have no sampler: their entries are NULL in C, and a call through
// them is undefined behavior, which `webp_samplers` reports as a panic.
static WEBP_SAMPLERS: [Option<WebPSamplerRowFunc>; 13] = [
    Some(yuv_to_rgb_row),      // MODE_RGB
    Some(yuv_to_rgba_row),     // MODE_RGBA
    Some(yuv_to_bgr_row),      // MODE_BGR
    Some(yuv_to_bgra_row),     // MODE_BGRA
    Some(yuv_to_argb_row),     // MODE_ARGB
    Some(yuv_to_rgba4444_row), // MODE_RGBA_4444
    Some(yuv_to_rgb565_row),   // MODE_RGB_565
    Some(yuv_to_rgba_row),     // MODE_rgbA
    Some(yuv_to_bgra_row),     // MODE_bgrA
    Some(yuv_to_argb_row),     // MODE_Argb
    Some(yuv_to_rgba4444_row), // MODE_rgbA_4444
    None,                      // MODE_YUV
    None,                      // MODE_YUVA
];

// upstream: src/dsp/yuv.c WebPSamplers
/// Converts one row with the sampler of `mode`, which is `WebPSamplers[mode]`.
///
/// # Panics
///
/// Panics for `MODE_YUV` and `MODE_YUVA`, which have no sampler; the C table
/// holds NULL there and calling it is undefined behavior (PORTING rule 8).
pub fn webp_samplers(mode: WEBP_CSP_MODE, y: &[u8], u: &[u8], v: &[u8], dst: &mut [u8], len: i32) {
    #[cfg(all(feature = "simd", any(target_arch = "x86", target_arch = "x86_64")))]
    {
        let features = crate::dsp::cpu::features();
        if features.sse4_1 {
            match mode {
                WEBP_CSP_MODE::MODE_RGB => {
                    return crate::dsp::yuv_sse41::yuv_to_rgb_row(y, u, v, dst, len);
                }
                WEBP_CSP_MODE::MODE_BGR => {
                    return crate::dsp::yuv_sse41::yuv_to_bgr_row(y, u, v, dst, len);
                }
                _ => {}
            }
        }
        if features.sse2 {
            match mode {
                WEBP_CSP_MODE::MODE_RGB => {
                    return crate::dsp::yuv_sse2::yuv_to_rgb_row(y, u, v, dst, len);
                }
                WEBP_CSP_MODE::MODE_RGBA => {
                    return crate::dsp::yuv_sse2::yuv_to_rgba_row(y, u, v, dst, len);
                }
                WEBP_CSP_MODE::MODE_BGR => {
                    return crate::dsp::yuv_sse2::yuv_to_bgr_row(y, u, v, dst, len);
                }
                WEBP_CSP_MODE::MODE_BGRA => {
                    return crate::dsp::yuv_sse2::yuv_to_bgra_row(y, u, v, dst, len);
                }
                WEBP_CSP_MODE::MODE_ARGB => {
                    return crate::dsp::yuv_sse2::yuv_to_argb_row(y, u, v, dst, len);
                }
                _ => {}
            }
        }
    }
    #[allow(
        unreachable_code,
        reason = "the C path is reached only without a SIMD variant"
    )]
    let sampler = WEBP_SAMPLERS[mode.0 as usize].expect("YUV modes have no row sampler");
    sampler(y, u, v, dst, len);
}

#[expect(
    clippy::cast_possible_wrap,
    reason = "the masked channels of an ARGB value are below 2^31"
)]
// upstream: src/dsp/yuv.c ConvertARGBToY_C
fn convert_argb_to_y_c(argb: &[u32], y: &mut [u8], width: i32) {
    let width = usize::try_from(width).expect("the width is not negative");
    for i in 0..width {
        let p = argb[i];
        y[i] = store_u8(vp8rgb_to_y(
            ((p >> 16) & 0xff) as i32,
            ((p >> 8) & 0xff) as i32,
            (p & 0xff) as i32,
            YUV_HALF,
        ));
    }
}

#[expect(
    clippy::cast_possible_wrap,
    reason = "the masked channels of an ARGB value are below 2^31"
)]
/// # Panics
///
/// Panics when `argb`, `u` or `v` is shorter than the pixels and chroma samples
/// that the conversion reads or writes.
// upstream: src/dsp/yuv.c WebPConvertARGBToUV_C
/// Converts ARGB pixels to subsampled U and V. With `do_store` false, the
/// result is averaged with the values already in `u` and `v`.
#[expect(
    clippy::many_single_char_names,
    reason = "the YUV and RGB component names of the C source"
)]
pub fn webp_convert_argb_to_uv_c(
    argb: &[u32],
    u: &mut [u8],
    v: &mut [u8],
    src_width: i32,
    do_store: i32,
) {
    // No rounding. Last pixel is dealt with separately.
    let uv_width = usize::try_from(src_width >> 1).expect("the width is not negative");
    let mut i = 0_usize;
    while i < uv_width {
        let v0 = argb[2 * i];
        let v1 = argb[2 * i + 1];
        // VP8RGBToU/V expects four accumulated pixels. Hence we need to
        // scale r/g/b value by a factor 2. We just shift v0/v1 one bit less.
        let r = (((v0 >> 15) & 0x1fe) + ((v1 >> 15) & 0x1fe)) as i32;
        let g = (((v0 >> 7) & 0x1fe) + ((v1 >> 7) & 0x1fe)) as i32;
        let b = (((v0 << 1) & 0x1fe) + ((v1 << 1) & 0x1fe)) as i32;
        let tmp_u = vp8rgb_to_u(r, g, b, YUV_HALF << 2);
        let tmp_v = vp8rgb_to_v(r, g, b, YUV_HALF << 2);
        if do_store != 0 {
            u[i] = store_u8(tmp_u);
            v[i] = store_u8(tmp_v);
        } else {
            // Approximated average-of-four. But it's an acceptable diff.
            u[i] = store_u8((i32::from(u[i]) + tmp_u + 1) >> 1);
            v[i] = store_u8((i32::from(v[i]) + tmp_v + 1) >> 1);
        }
        i += 1;
    }
    if src_width & 1 != 0 {
        // last pixel
        let v0 = argb[2 * i];
        let r = ((v0 >> 14) & 0x3fc) as i32;
        let g = ((v0 >> 6) & 0x3fc) as i32;
        let b = ((v0 << 2) & 0x3fc) as i32;
        let tmp_u = vp8rgb_to_u(r, g, b, YUV_HALF << 2);
        let tmp_v = vp8rgb_to_v(r, g, b, YUV_HALF << 2);
        if do_store != 0 {
            u[i] = store_u8(tmp_u);
            v[i] = store_u8(tmp_v);
        } else {
            u[i] = store_u8((i32::from(u[i]) + tmp_u + 1) >> 1);
            v[i] = store_u8((i32::from(v[i]) + tmp_v + 1) >> 1);
        }
    }
}

// upstream: src/dsp/yuv.c ConvertRGB24ToY_C
fn convert_rgb24_to_y_c(rgb: &[u8], y: &mut [u8], width: i32) {
    let width = usize::try_from(width).expect("the width is not negative");
    for i in 0..width {
        let px = &rgb[3 * i..3 * i + 3];
        y[i] = store_u8(vp8rgb_to_y(
            i32::from(px[0]),
            i32::from(px[1]),
            i32::from(px[2]),
            YUV_HALF,
        ));
    }
}

// upstream: src/dsp/yuv.c ConvertBGR24ToY_C
fn convert_bgr24_to_y_c(bgr: &[u8], y: &mut [u8], width: i32) {
    let width = usize::try_from(width).expect("the width is not negative");
    for i in 0..width {
        let px = &bgr[3 * i..3 * i + 3];
        y[i] = store_u8(vp8rgb_to_y(
            i32::from(px[2]),
            i32::from(px[1]),
            i32::from(px[0]),
            YUV_HALF,
        ));
    }
}

/// Converts `width` RGBA samples of accumulated 16-bit values to U and V.
///
/// # Panics
///
/// Panics when a slice is shorter than the samples the conversion reads or
/// writes.
// upstream: src/dsp/yuv.c WebPConvertRGBA32ToUV_C
#[expect(
    clippy::many_single_char_names,
    reason = "the YUV and RGB component names of the C source"
)]
pub fn webp_convert_rgba32_to_uv_c(rgb: &[u16], u: &mut [u8], v: &mut [u8], width: i32) {
    let width = usize::try_from(width).expect("the width is not negative");
    for i in 0..width {
        let r = i32::from(rgb[4 * i]);
        let g = i32::from(rgb[4 * i + 1]);
        let b = i32::from(rgb[4 * i + 2]);
        u[i] = store_u8(vp8rgb_to_u(r, g, b, YUV_HALF << 2));
        v[i] = store_u8(vp8rgb_to_v(r, g, b, YUV_HALF << 2));
    }
}

// upstream: src/dsp/yuv.c WebPConvertARGBToY
// `WebPConvertARGBToY` is a function pointer that `WebPInitConvertARGBToYUV`
// assigns to `ConvertARGBToY_C`; the portable pointer is the only one that the
// reference configuration uses (ADR 0003).
pub fn webp_convert_argb_to_y(argb: &[u32], y: &mut [u8], width: i32) {
    #[cfg(all(feature = "simd", any(target_arch = "x86", target_arch = "x86_64")))]
    {
        let features = crate::dsp::cpu::features();
        if features.sse4_1 {
            return crate::dsp::yuv_sse41::convert_argb_to_y(argb, y, width);
        }
        if features.sse2 {
            return crate::dsp::yuv_sse2::convert_argb_to_y(argb, y, width);
        }
    }
    #[allow(
        unreachable_code,
        reason = "the C path is reached only without a SIMD variant"
    )]
    convert_argb_to_y_c(argb, y, width);
}

// upstream: src/dsp/yuv.c WebPConvertARGBToUV
// The dispatch to `WebPConvertARGBToUV_C`, as for `WebPConvertARGBToY`.
pub fn webp_convert_argb_to_uv(
    argb: &[u32],
    u: &mut [u8],
    v: &mut [u8],
    src_width: i32,
    do_store: i32,
) {
    #[cfg(all(feature = "simd", any(target_arch = "x86", target_arch = "x86_64")))]
    {
        let features = crate::dsp::cpu::features();
        if features.sse4_1 {
            return crate::dsp::yuv_sse41::convert_argb_to_uv(argb, u, v, src_width, do_store);
        }
        if features.sse2 {
            return crate::dsp::yuv_sse2::convert_argb_to_uv(argb, u, v, src_width, do_store);
        }
    }
    #[allow(
        unreachable_code,
        reason = "the C path is reached only without a SIMD variant"
    )]
    webp_convert_argb_to_uv_c(argb, u, v, src_width, do_store);
}

// upstream: src/dsp/yuv.c WebPConvertRGB24ToY
// The dispatch to `ConvertRGB24ToY_C`, as for `WebPConvertARGBToY`.
pub fn webp_convert_rgb24_to_y(rgb: &[u8], y: &mut [u8], width: i32) {
    #[cfg(all(feature = "simd", any(target_arch = "x86", target_arch = "x86_64")))]
    {
        let features = crate::dsp::cpu::features();
        if features.sse4_1 {
            return crate::dsp::yuv_sse41::convert_rgb24_to_y(rgb, y, width);
        }
        if features.sse2 {
            return crate::dsp::yuv_sse2::convert_rgb24_to_y(rgb, y, width);
        }
    }
    #[allow(
        unreachable_code,
        reason = "the C path is reached only without a SIMD variant"
    )]
    convert_rgb24_to_y_c(rgb, y, width);
}

// upstream: src/dsp/yuv.c WebPConvertBGR24ToY
// The dispatch to `ConvertBGR24ToY_C`, as for `WebPConvertARGBToY`.
pub fn webp_convert_bgr24_to_y(bgr: &[u8], y: &mut [u8], width: i32) {
    #[cfg(all(feature = "simd", any(target_arch = "x86", target_arch = "x86_64")))]
    {
        let features = crate::dsp::cpu::features();
        if features.sse4_1 {
            return crate::dsp::yuv_sse41::convert_bgr24_to_y(bgr, y, width);
        }
        if features.sse2 {
            return crate::dsp::yuv_sse2::convert_bgr24_to_y(bgr, y, width);
        }
    }
    #[allow(
        unreachable_code,
        reason = "the C path is reached only without a SIMD variant"
    )]
    convert_bgr24_to_y_c(bgr, y, width);
}

// upstream: src/dsp/yuv.c WebPConvertRGBA32ToUV
// The dispatch to `WebPConvertRGBA32ToUV_C`, as for `WebPConvertARGBToY`.
pub fn webp_convert_rgba32_to_uv(rgb: &[u16], u: &mut [u8], v: &mut [u8], width: i32) {
    #[cfg(all(feature = "simd", any(target_arch = "x86", target_arch = "x86_64")))]
    {
        let features = crate::dsp::cpu::features();
        if features.sse4_1 {
            return crate::dsp::yuv_sse41::convert_rgba32_to_uv(rgb, u, v, width);
        }
        if features.sse2 {
            return crate::dsp::yuv_sse2::convert_rgba32_to_uv(rgb, u, v, width);
        }
    }
    #[allow(
        unreachable_code,
        reason = "the C path is reached only without a SIMD variant"
    )]
    webp_convert_rgba32_to_uv_c(rgb, u, v, width);
}
