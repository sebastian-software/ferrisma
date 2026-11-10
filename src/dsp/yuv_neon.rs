// libwebp v1.6.0, src/dsp/yuv_neon.c:
//
// Copyright 2017 Google Inc. All Rights Reserved.
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

//! Translation of `src/dsp/yuv_neon.c` from libwebp v1.6.0.
//!
//! Work package: WP20 (NEON).
//!
//! The NEON variants of the RGB to YUV converters of `src/dsp/yuv.c`. Each
//! `_neon` function is the body that upstream's
//! `WebPInitConvertARGBToYUVNEON` assigns; the safe entry point below it,
//! named after the function without the suffix, is what the dispatch
//! function of `src/dsp/yuv.rs` calls on 64-bit ARM.
//!
//! `WebPInitConvertARGBToYUVNEON` assigns `WebPConvertRGB24ToY`,
//! `WebPConvertBGR24ToY`, `WebPConvertARGBToY`, `WebPConvertARGBToUV` and
//! `WebPConvertRGBA32ToUV`; `WebPInitConvertARGBToYUV` calls it
//! unconditionally on 64-bit ARM (`WEBP_NEON_OMIT_C_CODE`), so the five
//! dispatch functions return the NEON entry points there.
//!
//! # Deviations from upstream
//!
//! 1. **Loads, stores and alignment** follow the deviations 1 to 3 of
//!    `src/dsp/neon.rs`.
//! 2. **Pointers** are slices from the first element, as in the C functions of
//!    `src/dsp/yuv.rs`; the pointer increments of the loops (`rgb += 3 * 8`,
//!    `u += 8`) are offsets into those slices. A negative width panics, as
//!    the C translations of `src/dsp/yuv.rs` do; C runs no iteration.
//! 3. **`MULTIPLY_16b_PREAMBLE`, `MULTIPLY_16b` and `CONVERT_RGB_TO_UV`** are
//!    functions: the preamble returns the six halves, and `SHIFT`, which the
//!    macro needs as an immediate, is a const parameter of the callers'
//!    `vqrshrun_n_s16`.
//!
//! # Excluded undefined behavior
//!
//! None.
//!
//! # Checks not applicable
//!
//! 1. **`WEBP_RESTRICT`** has no counterpart: the slices cannot alias.
//! 2. **Parity of `ConvertRGBA32ToUV_NEON` with `WebPConvertRGBA32ToUV_C`
//!    for components of 32768 and more.** The NEON kernel reinterprets its
//!    16-bit lanes as signed (`vreinterpret_s16_u16`) before the
//!    multiplication, so for a red, green or blue sum of 32768 or more it
//!    computes with a negative value where the C function computes with the
//!    unsigned one, and the two differ; upstream's kernel does the same,
//!    and the translation keeps it. The encoder passes sums of four 8-bit
//!    samples after the gamma mapping of `picture_csp_enc.c`, at most 1020,
//!    and below 32768 the kernel equals the C function: every product and
//!    sum fits its lane, and the saturating narrowing clips as `VP8ClipUV`
//!    does. `tests/wp20_yuv.rs` compares the two over that whole range;
//!    against the `release-simd` oracle, whose probe passes random 16-bit
//!    values, `tests/wp4_yuv.rs` gives upstream's NEON bytes.

use core::arch::aarch64::{
    int16x4_t, int16x8_t, uint8x8_t, uint16x8_t, vaddq_s16, vaddq_u16, vcombine_s16, vcombine_u16,
    vdupq_n_s16, vdupq_n_u16, vget_high_u16, vget_low_u16, vmlal_n_s16, vmlal_n_u16, vmovl_u8,
    vmull_n_s16, vmull_n_u16, vpaddlq_u8, vqmovn_u16, vqrshrun_n_s16, vreinterpret_s16_u16,
    vrhadd_u8, vrshrn_n_u32, vshrn_n_s32,
};

use crate::dsp::neon::{vld1_u8, vld3_u8, vld4_u8_words, vld4q_u8_words, vld4q_u16, vst1_u8};
use crate::dsp::yuv::{
    YUV_HALF, store_u8, vp8rgb_to_u, vp8rgb_to_v, vp8rgb_to_y, webp_convert_argb_to_uv_c,
};

/// A width that the C translations of `src/dsp/yuv.rs` take as not negative.
#[must_use]
fn width_of(width: i32) -> usize {
    usize::try_from(width).expect("the width is not negative")
}

//-----------------------------------------------------------------------------

// upstream: src/dsp/yuv_neon.c ConvertRGBToY_NEON
#[target_feature(enable = "neon")]
#[must_use]
fn convert_rgb_to_y_neon(r_: uint8x8_t, g_: uint8x8_t, b_: uint8x8_t) -> uint8x8_t {
    let r = vmovl_u8(r_);
    let g = vmovl_u8(g_);
    let b = vmovl_u8(b_);
    let r_lo = vget_low_u16(r);
    let r_hi = vget_high_u16(r);
    let g_lo = vget_low_u16(g);
    let g_hi = vget_high_u16(g);
    let b_lo = vget_low_u16(b);
    let b_hi = vget_high_u16(b);
    let tmp0_lo = vmull_n_u16(r_lo, 16839);
    let tmp0_hi = vmull_n_u16(r_hi, 16839);
    let tmp1_lo = vmlal_n_u16(tmp0_lo, g_lo, 33059);
    let tmp1_hi = vmlal_n_u16(tmp0_hi, g_hi, 33059);
    let tmp2_lo = vmlal_n_u16(tmp1_lo, b_lo, 6420);
    let tmp2_hi = vmlal_n_u16(tmp1_hi, b_hi, 6420);
    let y1 = vcombine_u16(vrshrn_n_u32::<16>(tmp2_lo), vrshrn_n_u32::<16>(tmp2_hi));
    let y2 = vaddq_u16(y1, vdupq_n_u16(16));
    vqmovn_u16(y2)
}

// upstream: src/dsp/yuv_neon.c ConvertRGB24ToY_NEON
#[target_feature(enable = "neon")]
fn convert_rgb24_to_y_neon(rgb: &[u8], y: &mut [u8], width: i32) {
    let width = width_of(width);
    let mut i = 0;
    let mut p = 0;
    while i + 8 <= width {
        let rgb_v = vld3_u8(&rgb[p..]);
        let y_v = convert_rgb_to_y_neon(rgb_v.0, rgb_v.1, rgb_v.2);
        vst1_u8(&mut y[i..], y_v);
        i += 8;
        p += 3 * 8;
    }
    while i < width {
        // left-over
        y[i] = store_u8(vp8rgb_to_y(
            i32::from(rgb[p]),
            i32::from(rgb[p + 1]),
            i32::from(rgb[p + 2]),
            YUV_HALF,
        ));
        i += 1;
        p += 3;
    }
}

/// Entry point of `ConvertRGB24ToY_NEON` for `WebPConvertRGB24ToY`.
pub fn convert_rgb24_to_y(rgb: &[u8], y: &mut [u8], width: i32) {
    // SAFETY: NEON is in the AArch64 baseline that this module is compiled for,
    // and upstream selects the NEON variant there unconditionally (ADR 0008).
    unsafe { convert_rgb24_to_y_neon(rgb, y, width) }
}

// upstream: src/dsp/yuv_neon.c ConvertBGR24ToY_NEON
#[target_feature(enable = "neon")]
fn convert_bgr24_to_y_neon(bgr: &[u8], y: &mut [u8], width: i32) {
    let width = width_of(width);
    let mut i = 0;
    let mut p = 0;
    while i + 8 <= width {
        let bgr_v = vld3_u8(&bgr[p..]);
        let y_v = convert_rgb_to_y_neon(bgr_v.2, bgr_v.1, bgr_v.0);
        vst1_u8(&mut y[i..], y_v);
        i += 8;
        p += 3 * 8;
    }
    while i < width {
        // left-over
        y[i] = store_u8(vp8rgb_to_y(
            i32::from(bgr[p + 2]),
            i32::from(bgr[p + 1]),
            i32::from(bgr[p]),
            YUV_HALF,
        ));
        i += 1;
        p += 3;
    }
}

/// Entry point of `ConvertBGR24ToY_NEON` for `WebPConvertBGR24ToY`.
pub fn convert_bgr24_to_y(bgr: &[u8], y: &mut [u8], width: i32) {
    // SAFETY: NEON is in the AArch64 baseline that this module is compiled for,
    // and upstream selects the NEON variant there unconditionally (ADR 0008).
    unsafe { convert_bgr24_to_y_neon(bgr, y, width) }
}

// upstream: src/dsp/yuv_neon.c ConvertARGBToY_NEON
#[target_feature(enable = "neon")]
#[expect(
    clippy::cast_possible_wrap,
    reason = "the masked channels of an ARGB value are below 2^31"
)]
fn convert_argb_to_y_neon(argb: &[u32], y: &mut [u8], width: i32) {
    let width = width_of(width);
    let mut i = 0;
    while i + 8 <= width {
        let rgb = vld4_u8_words(&argb[i..]);
        let y_v = convert_rgb_to_y_neon(rgb.2, rgb.1, rgb.0);
        vst1_u8(&mut y[i..], y_v);
        i += 8;
    }
    while i < width {
        // left-over
        let p = argb[i];
        y[i] = store_u8(vp8rgb_to_y(
            ((p >> 16) & 0xff) as i32,
            ((p >> 8) & 0xff) as i32,
            (p & 0xff) as i32,
            YUV_HALF,
        ));
        i += 1;
    }
}

/// Entry point of `ConvertARGBToY_NEON` for `WebPConvertARGBToY`.
pub fn convert_argb_to_y(argb: &[u32], y: &mut [u8], width: i32) {
    // SAFETY: NEON is in the AArch64 baseline that this module is compiled for,
    // and upstream selects the NEON variant there unconditionally (ADR 0008).
    unsafe { convert_argb_to_y_neon(argb, y, width) }
}

//-----------------------------------------------------------------------------

// upstream: src/dsp/yuv_neon.c MULTIPLY_16b_PREAMBLE
/// The low and high halves of `r`, `g` and `b` as signed 16-bit lanes:
/// `[r_lo, r_hi, g_lo, g_hi, b_lo, b_hi]`.
#[target_feature(enable = "neon")]
#[must_use]
fn multiply_16b_preamble(r: uint16x8_t, g: uint16x8_t, b: uint16x8_t) -> [int16x4_t; 6] {
    [
        vreinterpret_s16_u16(vget_low_u16(r)),
        vreinterpret_s16_u16(vget_high_u16(r)),
        vreinterpret_s16_u16(vget_low_u16(g)),
        vreinterpret_s16_u16(vget_high_u16(g)),
        vreinterpret_s16_u16(vget_low_u16(b)),
        vreinterpret_s16_u16(vget_high_u16(b)),
    ]
}

// upstream: src/dsp/yuv_neon.c MULTIPLY_16b
/// computes: `DST_s16 = [(C0 * r + C1 * g + C2 * b) >> 16] + CST`
#[target_feature(enable = "neon")]
#[must_use]
fn multiply_16b(lanes: [int16x4_t; 6], c0: i16, c1: i16, c2: i16, cst: i16) -> int16x8_t {
    let [r_lo, r_hi, g_lo, g_hi, b_lo, b_hi] = lanes;
    let tmp0_lo = vmull_n_s16(r_lo, c0);
    let tmp0_hi = vmull_n_s16(r_hi, c0);
    let tmp1_lo = vmlal_n_s16(tmp0_lo, g_lo, c1);
    let tmp1_hi = vmlal_n_s16(tmp0_hi, g_hi, c1);
    let tmp2_lo = vmlal_n_s16(tmp1_lo, b_lo, c2);
    let tmp2_hi = vmlal_n_s16(tmp1_hi, b_hi, c2);
    let tmp3 = vcombine_s16(vshrn_n_s32::<16>(tmp2_lo), vshrn_n_s32::<16>(tmp2_hi));
    vaddq_s16(tmp3, vdupq_n_s16(cst))
}

// upstream: src/dsp/yuv_neon.c CONVERT_RGB_TO_UV
/// The U and V lanes of `r`, `g` and `b`, offset by `128 << shift`, before
/// the final rounding shift by `shift`.
#[target_feature(enable = "neon")]
#[must_use]
fn convert_rgb_to_uv(
    r: uint16x8_t,
    g: uint16x8_t,
    b: uint16x8_t,
    shift: i16,
) -> (int16x8_t, int16x8_t) {
    let lanes = multiply_16b_preamble(r, g, b);
    let u_dst = multiply_16b(lanes, -9719, -19081, 28800, 128 << shift);
    let v_dst = multiply_16b(lanes, 28800, -24116, -4684, 128 << shift);
    (u_dst, v_dst)
}

// upstream: src/dsp/yuv_neon.c ConvertRGBA32ToUV_NEON
#[target_feature(enable = "neon")]
#[expect(
    clippy::many_single_char_names,
    reason = "the YUV and RGB component names of the C source"
)]
fn convert_rgba32_to_uv_neon(rgb: &[u16], u: &mut [u8], v: &mut [u8], width: i32) {
    let width = width_of(width);
    let mut i = 0;
    let mut p = 0;
    while i + 8 <= width {
        let rgb_v = vld4q_u16(&rgb[p..]);
        let (u_v, v_v) = convert_rgb_to_uv(rgb_v.0, rgb_v.1, rgb_v.2, 2);
        vst1_u8(&mut u[i..], vqrshrun_n_s16::<2>(u_v));
        vst1_u8(&mut v[i..], vqrshrun_n_s16::<2>(v_v));
        i += 8;
        p += 4 * 8;
    }
    while i < width {
        let r = i32::from(rgb[p]);
        let g = i32::from(rgb[p + 1]);
        let b = i32::from(rgb[p + 2]);
        u[i] = store_u8(vp8rgb_to_u(r, g, b, YUV_HALF << 2));
        v[i] = store_u8(vp8rgb_to_v(r, g, b, YUV_HALF << 2));
        i += 1;
        p += 4;
    }
}

/// Entry point of `ConvertRGBA32ToUV_NEON` for `WebPConvertRGBA32ToUV`.
pub fn convert_rgba32_to_uv(rgb: &[u16], u: &mut [u8], v: &mut [u8], width: i32) {
    // SAFETY: NEON is in the AArch64 baseline that this module is compiled for,
    // and upstream selects the NEON variant there unconditionally (ADR 0008).
    unsafe { convert_rgba32_to_uv_neon(rgb, u, v, width) }
}

// upstream: src/dsp/yuv_neon.c ConvertARGBToUV_NEON
#[target_feature(enable = "neon")]
#[expect(
    clippy::many_single_char_names,
    reason = "the C names u, v, r, g, b and i"
)]
fn convert_argb_to_uv_neon(
    argb: &[u32],
    u: &mut [u8],
    v: &mut [u8],
    src_width: i32,
    do_store: i32,
) {
    let mut i = 0_i32;
    let mut uv = 0;
    while i + 16 <= src_width {
        let rgb = vld4q_u8_words(&argb[width_of(i)..]);
        let r = vpaddlq_u8(rgb.2); // pair-wise adds
        let g = vpaddlq_u8(rgb.1);
        let b = vpaddlq_u8(rgb.0);
        let (u_tmp, v_tmp) = convert_rgb_to_uv(r, g, b, 1);
        {
            let u_v = vqrshrun_n_s16::<1>(u_tmp);
            let v_v = vqrshrun_n_s16::<1>(v_tmp);
            if do_store != 0 {
                vst1_u8(&mut u[uv..], u_v);
                vst1_u8(&mut v[uv..], v_v);
            } else {
                let prev_u = vld1_u8(&u[uv..]);
                let prev_v = vld1_u8(&v[uv..]);
                vst1_u8(&mut u[uv..], vrhadd_u8(u_v, prev_u));
                vst1_u8(&mut v[uv..], vrhadd_u8(v_v, prev_v));
            }
        }
        i += 16;
        uv += 8;
    }
    if i < src_width {
        // left-over
        webp_convert_argb_to_uv_c(
            &argb[width_of(i)..],
            &mut u[uv..],
            &mut v[uv..],
            src_width - i,
            do_store,
        );
    }
}

/// Entry point of `ConvertARGBToUV_NEON` for `WebPConvertARGBToUV`.
pub fn convert_argb_to_uv(argb: &[u32], u: &mut [u8], v: &mut [u8], src_width: i32, do_store: i32) {
    // SAFETY: NEON is in the AArch64 baseline that this module is compiled for,
    // and upstream selects the NEON variant there unconditionally (ADR 0008).
    unsafe { convert_argb_to_uv_neon(argb, u, v, src_width, do_store) }
}

//------------------------------------------------------------------------------

// upstream: src/dsp/yuv_neon.c WebPInitConvertARGBToYUVNEON
// The pointer assignments are the dispatch functions of `src/dsp/yuv.rs` named
// in the module documentation (ADR 0008).
