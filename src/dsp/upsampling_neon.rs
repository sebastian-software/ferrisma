// libwebp v1.6.0, src/dsp/upsampling_neon.c:
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
// NEON version of YUV to RGB upsampling functions.
//
// Author: mans@mansr.com (Mans Rullgard)
// Based on SSE code by: somnath@google.com (Somnath Banerjee)

//! Translation of `src/dsp/upsampling_neon.c` from libwebp v1.6.0.
//!
//! Work package: WP20 (NEON).
//!
//! The NEON variants of the fancy upsamplers of `src/dsp/upsampling.c`. Each
//! `_neon` function is the body that upstream's `WebPInitUpsamplersNEON`
//! stores in `WebPUpsamplers`; the safe entry point below it, named after the
//! function without the suffix, has the type `WebPUpsampleLinePairFunc`, and
//! [`upsamplers`] gives the entry of a mode, which `webp_upsamplers` and
//! `webp_get_line_pair_converter` of `src/dsp/upsampling.rs` use on 64-bit
//! ARM.
//!
//! `WebPInitUpsamplersNEON` sets the entries `MODE_RGBA`, `MODE_BGRA`,
//! `MODE_rgbA`, `MODE_bgrA`, `MODE_RGB`, `MODE_BGR`, `MODE_ARGB`, `MODE_Argb`,
//! `MODE_RGB_565`, `MODE_RGBA_4444` and `MODE_rgbA_4444`, all eleven upsampled
//! modes; `WebPInitUpsamplers` calls it unconditionally on 64-bit ARM
//! (`WEBP_NEON_OMIT_C_CODE`), so the table functions select the NEON entry
//! for each of them. The YUV 4:4:4 converters and the samplers have no NEON
//! variant.
//!
//! # Deviations from upstream
//!
//! 1. **Loads, stores and alignment** follow the deviations 1 to 3 of
//!    `src/dsp/neon.rs`. `uv_buf`, which C aligns to 16 bytes by hand
//!    (`r_uv`), is a plain `[u8; 64]`.
//! 2. **`NEON_UPSAMPLE_FUNC`** generates one function per output format; the
//!    body is the generic function [`neon_upsample_func`] over the format,
//!    which the seven `_neon` functions call. The `STORE_*` macros and the
//!    `VP8YuvTo*` call of each format are selected by the const parameter.
//! 3. **The bottom row** is `Option<(&[u8], &mut [u8])>`, as in
//!    `WebPUpsampleLinePairFunc` (the deviation 5 of `src/dsp/yuv.rs`);
//!    `assert(top_y != NULL)` has no counterpart, as the slice is never null.
//! 4. **`v255`**, an expression macro, is a function returning the vector.
//!
//! # Excluded undefined behavior
//!
//! None.
//!
//! # Checks not applicable
//!
//! 1. **`WEBP_REDUCE_CSP`** is unset in the reference configuration, so all
//!    seven upsamplers are compiled.
//! 2. **`WEBP_SWAP_16BIT_CSP`** is 0, so `ZIP_U8(lo, hi)` is `vzip_u8(lo, hi)`;
//!    the swapped order is not compiled.
//! 3. **`FANCY_UPSAMPLING`** is defined by `dsp.h`; without it upstream
//!    compiles no NEON upsampler.

use core::arch::aarch64::{
    int16x4_t, int16x8_t, uint8x8_t, uint8x8x2_t, uint8x8x3_t, uint8x8x4_t, vaddl_u8, vaddq_u16,
    vcombine_u8, vdup_n_u8, vdupq_n_s16, vqaddq_s16, vqdmulhq_lane_s16, vqdmulhq_n_s16,
    vqshrun_n_s16, vqsubq_s16, vreinterpretq_s16_u16, vrhadd_u8, vshl_n_u8, vshll_n_u8,
    vshlq_n_u16, vshrn_n_u16, vsri_n_u8, vzip_u8,
};

use crate::dsp::neon::{vld1_s16, vld1_u8, vst1q_u8, vst2_u8, vst3_u8, vst4_u8};
use crate::dsp::upsampling::WebPUpsampleLinePairFunc;
use crate::dsp::yuv::{
    YUV_FIX2, vp8_yuv_to_argb, vp8_yuv_to_bgr, vp8_yuv_to_bgra, vp8_yuv_to_rgb, vp8_yuv_to_rgb565,
    vp8_yuv_to_rgba, vp8_yuv_to_rgba4444,
};
use crate::webp::decode::WEBP_CSP_MODE;

//-----------------------------------------------------------------------------
// U/V upsampling

// upstream: src/dsp/upsampling_neon.c UPSAMPLE_16PIXELS
/// Loads 9 pixels each from rows r1 and r2 and generates 16 pixels.
#[target_feature(enable = "neon")]
fn upsample_16pixels(r1: &[u8], r2: &[u8], out: &mut [u8]) {
    let a = vld1_u8(&r1[0..]);
    let b = vld1_u8(&r1[1..]);
    let c = vld1_u8(&r2[0..]);
    let d = vld1_u8(&r2[1..]);
    // a + b + c + d
    let ad = vaddl_u8(a, d);
    let bc = vaddl_u8(b, c);
    let abcd = vaddq_u16(ad, bc);
    // 3a +  b +  c + 3d
    let al = vaddq_u16(abcd, vshlq_n_u16::<1>(ad));
    //  a + 3b + 3c +  d
    let bl = vaddq_u16(abcd, vshlq_n_u16::<1>(bc));

    let diag2 = vshrn_n_u16::<3>(al);
    let diag1 = vshrn_n_u16::<3>(bl);

    let a_ = vrhadd_u8(a, diag1);
    let b_ = vrhadd_u8(b, diag2);
    let c_ = vrhadd_u8(c, diag2);
    let d_ = vrhadd_u8(d, diag1);

    let a_b = uint8x8x2_t(a_, b_);
    let c_d = uint8x8x2_t(c_, d_);
    vst2_u8(&mut out[0..], a_b);
    vst2_u8(&mut out[32..], c_d);
}

// upstream: src/dsp/upsampling_neon.c Upsample16Pixels_NEON
/// Turn the macro into a function for reducing code-size when non-critical
#[target_feature(enable = "neon")]
fn upsample16_pixels_neon(r1: &[u8], r2: &[u8], out: &mut [u8]) {
    upsample_16pixels(r1, r2, out);
}

// upstream: src/dsp/upsampling_neon.c UPSAMPLE_LAST_BLOCK
/// Upsamples the last `num_pixels` samples of `tb` and `bb`, with the last
/// sample of each row repeated up to the 9 that the block reads.
#[target_feature(enable = "neon")]
fn upsample_last_block(tb: &[u8], bb: &[u8], num_pixels: usize, out: &mut [u8]) {
    let mut r1 = [0_u8; 9];
    let mut r2 = [0_u8; 9];
    r1[..num_pixels].copy_from_slice(&tb[..num_pixels]);
    r2[..num_pixels].copy_from_slice(&bb[..num_pixels]);
    // replicate last byte
    let last1 = r1[num_pixels - 1];
    let last2 = r2[num_pixels - 1];
    r1[num_pixels..].fill(last1);
    r2[num_pixels..].fill(last2);
    upsample16_pixels_neon(&r1, &r2, out);
}

//-----------------------------------------------------------------------------
// YUV->RGB conversion

// upstream: src/dsp/upsampling_neon.c kCoeffs1
/// note: we represent the 33050 large constant as 32768 + 282
const COEFFS1: [i16; 4] = [19077, 26149, 6419, 13320];

// upstream: src/dsp/upsampling_neon.c v255
#[target_feature(enable = "neon")]
#[must_use]
fn v255() -> uint8x8_t {
    vdup_n_u8(255)
}

/// The output formats of `NEON_UPSAMPLE_FUNC`, as the const parameter of
/// [`neon_upsample_func`].
const RGBA: u8 = 0;
const BGRA: u8 = 1;
const RGB: u8 = 2;
const BGR: u8 = 3;
const ARGB: u8 = 4;
const RGBA4444: u8 = 5;
const RGB565: u8 = 6;

// upstream: src/dsp/upsampling_neon.c STORE_Rgb
#[target_feature(enable = "neon")]
fn store_rgb(out: &mut [u8], r: uint8x8_t, g: uint8x8_t, b: uint8x8_t) {
    let r_g_b = uint8x8x3_t(r, g, b);
    vst3_u8(out, r_g_b);
}

// upstream: src/dsp/upsampling_neon.c STORE_Bgr
#[target_feature(enable = "neon")]
fn store_bgr(out: &mut [u8], r: uint8x8_t, g: uint8x8_t, b: uint8x8_t) {
    let b_g_r = uint8x8x3_t(b, g, r);
    vst3_u8(out, b_g_r);
}

// upstream: src/dsp/upsampling_neon.c STORE_Rgba
#[target_feature(enable = "neon")]
fn store_rgba(out: &mut [u8], r: uint8x8_t, g: uint8x8_t, b: uint8x8_t) {
    let r_g_b_v255 = uint8x8x4_t(r, g, b, v255());
    vst4_u8(out, r_g_b_v255);
}

// upstream: src/dsp/upsampling_neon.c STORE_Bgra
#[target_feature(enable = "neon")]
fn store_bgra(out: &mut [u8], r: uint8x8_t, g: uint8x8_t, b: uint8x8_t) {
    let b_g_r_v255 = uint8x8x4_t(b, g, r, v255());
    vst4_u8(out, b_g_r_v255);
}

// upstream: src/dsp/upsampling_neon.c STORE_Argb
#[target_feature(enable = "neon")]
fn store_argb(out: &mut [u8], r: uint8x8_t, g: uint8x8_t, b: uint8x8_t) {
    let v255_r_g_b = uint8x8x4_t(v255(), r, g, b);
    vst4_u8(out, v255_r_g_b);
}

// upstream: src/dsp/upsampling_neon.c ZIP_U8
/// `vzip_u8(lo, hi)`, as `WEBP_SWAP_16BIT_CSP` is 0.
#[target_feature(enable = "neon")]
#[must_use]
fn zip_u8(lo: uint8x8_t, hi: uint8x8_t) -> uint8x8x2_t {
    vzip_u8(lo, hi)
}

// upstream: src/dsp/upsampling_neon.c STORE_Rgba4444
#[target_feature(enable = "neon")]
fn store_rgba4444(out: &mut [u8], r: uint8x8_t, g: uint8x8_t, b: uint8x8_t) {
    let rg = vsri_n_u8::<4>(r, g); // shift g, insert r
    let ba = vsri_n_u8::<4>(b, v255()); // shift a, insert b
    let rgba4444 = zip_u8(rg, ba);
    vst1q_u8(out, vcombine_u8(rgba4444.0, rgba4444.1));
}

// upstream: src/dsp/upsampling_neon.c STORE_Rgb565
#[target_feature(enable = "neon")]
fn store_rgb565(out: &mut [u8], r: uint8x8_t, g: uint8x8_t, b: uint8x8_t) {
    let rg = vsri_n_u8::<5>(r, g); // shift g and insert r
    let g1 = vshl_n_u8::<3>(g); // pre-shift g: 3bits
    let gb = vsri_n_u8::<3>(g1, b); // shift b and insert g
    let rgb565 = zip_u8(rg, gb);
    vst1q_u8(out, vcombine_u8(rgb565.0, rgb565.1));
}

/// `STORE_ ## FMT`: the store of the format.
#[target_feature(enable = "neon")]
fn store<const FMT: u8>(out: &mut [u8], r: uint8x8_t, g: uint8x8_t, b: uint8x8_t) {
    match FMT {
        RGBA => store_rgba(out, r, g, b),
        BGRA => store_bgra(out, r, g, b),
        RGB => store_rgb(out, r, g, b),
        BGR => store_bgr(out, r, g, b),
        ARGB => store_argb(out, r, g, b),
        RGBA4444 => store_rgba4444(out, r, g, b),
        _ => store_rgb565(out, r, g, b),
    }
}

/// `VP8YuvTo ## FMT`: the scalar conversion of the format.
fn yuv_to<const FMT: u8>(y: u8, u: u8, v: u8, dst: &mut [u8]) {
    let (y32, u32_, v32) = (i32::from(y), i32::from(u), i32::from(v));
    match FMT {
        RGBA => vp8_yuv_to_rgba(y, u, v, dst),
        BGRA => vp8_yuv_to_bgra(y, u, v, dst),
        RGB => vp8_yuv_to_rgb(y32, u32_, v32, dst),
        BGR => vp8_yuv_to_bgr(y32, u32_, v32, dst),
        ARGB => vp8_yuv_to_argb(y, u, v, dst),
        RGBA4444 => vp8_yuv_to_rgba4444(y32, u32_, v32, dst),
        _ => vp8_yuv_to_rgb565(y32, u32_, v32, dst),
    }
}

/// The constants that `CONVERT8` reads from the scope of
/// `NEON_UPSAMPLE_FUNC`: `coeff1` and the three rounders.
#[derive(Clone, Copy)]
struct Constants {
    coeff1: int16x4_t,
    r_rounder: int16x8_t,
    g_rounder: int16x8_t,
    b_rounder: int16x8_t,
}

// upstream: src/dsp/upsampling_neon.c CONVERT8
#[target_feature(enable = "neon")]
#[expect(
    clippy::many_single_char_names,
    reason = "the short C names of the source"
)]
fn convert8<const FMT: u8, const XSTEP: usize>(
    n: usize,
    src_y: &[u8],
    src_uv: &[u8],
    out: &mut [u8],
    cur_x: usize,
    k: Constants,
) {
    let mut i = 0;
    while i < n {
        let off = (cur_x + i) * XSTEP;
        let y = vld1_u8(&src_y[cur_x + i..]);
        let u = vld1_u8(&src_uv[i..]);
        let v = vld1_u8(&src_uv[i + 16..]);
        let y0 = vreinterpretq_s16_u16(vshll_n_u8::<7>(y));
        let u0 = vreinterpretq_s16_u16(vshll_n_u8::<7>(u));
        let v0 = vreinterpretq_s16_u16(vshll_n_u8::<7>(v));
        let y1 = vqdmulhq_lane_s16::<0>(y0, k.coeff1);
        let r0 = vqdmulhq_lane_s16::<1>(v0, k.coeff1);
        let g0 = vqdmulhq_lane_s16::<2>(u0, k.coeff1);
        let g1 = vqdmulhq_lane_s16::<3>(v0, k.coeff1);
        let b0 = vqdmulhq_n_s16(u0, 282);
        let r1 = vqaddq_s16(y1, k.r_rounder);
        let g2 = vqaddq_s16(y1, k.g_rounder);
        let b1 = vqaddq_s16(y1, k.b_rounder);
        let r2 = vqaddq_s16(r0, r1);
        let g3 = vqaddq_s16(g0, g1);
        let b2 = vqaddq_s16(b0, b1);
        let g4 = vqsubq_s16(g2, g3);
        let b3 = vqaddq_s16(b2, u0);
        let r = vqshrun_n_s16::<YUV_FIX2>(r2);
        let g = vqshrun_n_s16::<YUV_FIX2>(g4);
        let b = vqshrun_n_s16::<YUV_FIX2>(b3);
        store::<FMT>(&mut out[off..], r, g, b);
        i += 8;
    }
}

// upstream: src/dsp/upsampling_neon.c CONVERT1
fn convert1<const FMT: u8, const XSTEP: usize>(
    n: usize,
    src_y: &[u8],
    src_uv: &[u8],
    rgb: &mut [u8],
    cur_x: usize,
) {
    for i in 0..n {
        let off = (cur_x + i) * XSTEP;
        let y = src_y[cur_x + i];
        let u = src_uv[i];
        let v = src_uv[i + 16];
        yuv_to::<FMT>(y, u, v, &mut rgb[off..]);
    }
}

// upstream: src/dsp/upsampling_neon.c CONVERT2RGB_8
#[target_feature(enable = "neon")]
fn convert2rgb_8<const FMT: u8, const XSTEP: usize>(
    top_y: &[u8],
    bottom: Option<(&[u8], &mut [u8])>,
    uv: &[u8],
    top_dst: &mut [u8],
    cur_x: usize,
    len: usize,
    k: Constants,
) {
    convert8::<FMT, XSTEP>(len, top_y, uv, top_dst, cur_x, k);
    if let Some((bottom_y, bottom_dst)) = bottom {
        convert8::<FMT, XSTEP>(len, bottom_y, &uv[32..], bottom_dst, cur_x, k);
    }
}

// upstream: src/dsp/upsampling_neon.c CONVERT2RGB_1
fn convert2rgb_1<const FMT: u8, const XSTEP: usize>(
    top_y: &[u8],
    bottom: Option<(&[u8], &mut [u8])>,
    uv: &[u8],
    top_dst: &mut [u8],
    cur_x: usize,
    len: usize,
) {
    convert1::<FMT, XSTEP>(len, top_y, uv, top_dst, cur_x);
    if let Some((bottom_y, bottom_dst)) = bottom {
        convert1::<FMT, XSTEP>(len, bottom_y, &uv[32..], bottom_dst, cur_x);
    }
}

/// `(top + cur + 1) >> 1`-style averages of two samples as the `int` that
/// `VP8YuvTo*` receives; the result is a byte.
#[must_use]
fn sample(value: i32) -> u8 {
    u8::try_from(value).expect("an average of samples is a byte")
}

// upstream: src/dsp/upsampling_neon.c NEON_UPSAMPLE_FUNC
/// The body of the seven NEON upsamplers, for the output format `FMT` with
/// `XSTEP` bytes per pixel.
#[target_feature(enable = "neon")]
#[expect(
    clippy::too_many_arguments,
    reason = "the parameters of WebPUpsampleLinePairFunc"
)]
fn neon_upsample_func<const FMT: u8, const XSTEP: usize>(
    top_y: &[u8],
    mut bottom: Option<(&[u8], &mut [u8])>,
    top_u: &[u8],
    top_v: &[u8],
    cur_u: &[u8],
    cur_v: &[u8],
    top_dst: &mut [u8],
    len: i32,
) {
    // 16 byte aligned array to cache reconstructed u and v
    let mut r_uv = [0_u8; 2 * 32];
    let len = usize::try_from(len).expect("the row length is not negative");
    let uv_len = (len + 1) >> 1;
    // 9 pixels must be read-able for each block
    let num_blocks = (uv_len - 1) >> 3;
    let leftover = uv_len - num_blocks * 8;
    let last_pos = 1 + 16 * num_blocks;

    let u_diag = ((i32::from(top_u[0]) + i32::from(cur_u[0])) >> 1) + 1;
    let v_diag = ((i32::from(top_v[0]) + i32::from(cur_v[0])) >> 1) + 1;

    let k = Constants {
        coeff1: vld1_s16(&COEFFS1),
        r_rounder: vdupq_n_s16(-14234),
        g_rounder: vdupq_n_s16(8708),
        b_rounder: vdupq_n_s16(-17685),
    };

    // Treat the first pixel in regular way
    {
        let u0 = (i32::from(top_u[0]) + u_diag) >> 1;
        let v0 = (i32::from(top_v[0]) + v_diag) >> 1;
        yuv_to::<FMT>(top_y[0], sample(u0), sample(v0), top_dst);
    }
    if let Some((bottom_y, bottom_dst)) = bottom.as_mut() {
        let u0 = (i32::from(cur_u[0]) + u_diag) >> 1;
        let v0 = (i32::from(cur_v[0]) + v_diag) >> 1;
        yuv_to::<FMT>(bottom_y[0], sample(u0), sample(v0), bottom_dst);
    }

    let mut uv = 0;
    for block in 0..num_blocks {
        upsample_16pixels(&top_u[uv..], &cur_u[uv..], &mut r_uv);
        upsample_16pixels(&top_v[uv..], &cur_v[uv..], &mut r_uv[16..]);
        convert2rgb_8::<FMT, XSTEP>(
            top_y,
            bottom.as_mut().map(|(y, dst)| (*y, &mut **dst)),
            &r_uv,
            top_dst,
            16 * block + 1,
            16,
            k,
        );
        uv += 8;
    }

    upsample_last_block(&top_u[uv..], &cur_u[uv..], leftover, &mut r_uv);
    upsample_last_block(&top_v[uv..], &cur_v[uv..], leftover, &mut r_uv[16..]);
    convert2rgb_1::<FMT, XSTEP>(top_y, bottom, &r_uv, top_dst, last_pos, len - last_pos);
}

/// Declares the NEON upsampler `$name` of one format and its entry point.
macro_rules! neon_upsampler {
    ($(#[$doc:meta])* $upstream:literal, $name:ident, $entry:ident, $fmt:ident, $xstep:literal) => {
        #[doc = concat!("upstream: src/dsp/upsampling_neon.c ", $upstream)]
        #[target_feature(enable = "neon")]
        #[expect(
            clippy::too_many_arguments,
            reason = "the parameters of WebPUpsampleLinePairFunc"
        )]
        fn $name(
            top_y: &[u8],
            bottom: Option<(&[u8], &mut [u8])>,
            top_u: &[u8],
            top_v: &[u8],
            cur_u: &[u8],
            cur_v: &[u8],
            top_dst: &mut [u8],
            len: i32,
        ) {
            neon_upsample_func::<$fmt, $xstep>(
                top_y, bottom, top_u, top_v, cur_u, cur_v, top_dst, len,
            );
        }

        $(#[$doc])*
        #[expect(
            clippy::too_many_arguments,
            reason = "the parameters of WebPUpsampleLinePairFunc"
        )]
        pub(crate) fn $entry(
            top_y: &[u8],
            bottom: Option<(&[u8], &mut [u8])>,
            top_u: &[u8],
            top_v: &[u8],
            cur_u: &[u8],
            cur_v: &[u8],
            top_dst: &mut [u8],
            len: i32,
        ) {
            // SAFETY: NEON is in the AArch64 baseline that this module is
            // compiled for, and upstream selects the NEON variant there
            // unconditionally (ADR 0008).
            unsafe { $name(top_y, bottom, top_u, top_v, cur_u, cur_v, top_dst, len) }
        }
    };
}

// NEON variants of the fancy upsampler.
// upstream: src/dsp/upsampling_neon.c UpsampleRgbaLinePair_NEON
neon_upsampler!(
    /// Entry point of `UpsampleRgbaLinePair_NEON`.
    "UpsampleRgbaLinePair_NEON", upsample_rgba_line_pair_neon, upsample_rgba_line_pair, RGBA, 4
);
// upstream: src/dsp/upsampling_neon.c UpsampleBgraLinePair_NEON
neon_upsampler!(
    /// Entry point of `UpsampleBgraLinePair_NEON`.
    "UpsampleBgraLinePair_NEON", upsample_bgra_line_pair_neon, upsample_bgra_line_pair, BGRA, 4
);
// upstream: src/dsp/upsampling_neon.c UpsampleRgbLinePair_NEON
neon_upsampler!(
    /// Entry point of `UpsampleRgbLinePair_NEON`.
    "UpsampleRgbLinePair_NEON", upsample_rgb_line_pair_neon, upsample_rgb_line_pair, RGB, 3
);
// upstream: src/dsp/upsampling_neon.c UpsampleBgrLinePair_NEON
neon_upsampler!(
    /// Entry point of `UpsampleBgrLinePair_NEON`.
    "UpsampleBgrLinePair_NEON", upsample_bgr_line_pair_neon, upsample_bgr_line_pair, BGR, 3
);
// upstream: src/dsp/upsampling_neon.c UpsampleArgbLinePair_NEON
neon_upsampler!(
    /// Entry point of `UpsampleArgbLinePair_NEON`.
    "UpsampleArgbLinePair_NEON", upsample_argb_line_pair_neon, upsample_argb_line_pair, ARGB, 4
);
// upstream: src/dsp/upsampling_neon.c UpsampleRgba4444LinePair_NEON
neon_upsampler!(
    /// Entry point of `UpsampleRgba4444LinePair_NEON`.
    "UpsampleRgba4444LinePair_NEON", upsample_rgba4444_line_pair_neon,
    upsample_rgba4444_line_pair, RGBA4444, 2
);
// upstream: src/dsp/upsampling_neon.c UpsampleRgb565LinePair_NEON
neon_upsampler!(
    /// Entry point of `UpsampleRgb565LinePair_NEON`.
    "UpsampleRgb565LinePair_NEON", upsample_rgb565_line_pair_neon, upsample_rgb565_line_pair,
    RGB565, 2
);

//------------------------------------------------------------------------------
// Entry point

// upstream: src/dsp/upsampling_neon.c WebPInitUpsamplersNEON
/// The entry of `WebPUpsamplers` that `WebPInitUpsamplersNEON` sets for
/// `mode`, or `None` for the modes it leaves (`MODE_YUV`, `MODE_YUVA`).
#[must_use]
pub(crate) fn upsamplers(mode: WEBP_CSP_MODE) -> Option<WebPUpsampleLinePairFunc> {
    let upsampler: WebPUpsampleLinePairFunc = match mode {
        WEBP_CSP_MODE::MODE_RGBA | WEBP_CSP_MODE::MODE_rgbA => upsample_rgba_line_pair,
        WEBP_CSP_MODE::MODE_BGRA | WEBP_CSP_MODE::MODE_bgrA => upsample_bgra_line_pair,
        WEBP_CSP_MODE::MODE_RGB => upsample_rgb_line_pair,
        WEBP_CSP_MODE::MODE_BGR => upsample_bgr_line_pair,
        WEBP_CSP_MODE::MODE_ARGB | WEBP_CSP_MODE::MODE_Argb => upsample_argb_line_pair,
        WEBP_CSP_MODE::MODE_RGB_565 => upsample_rgb565_line_pair,
        WEBP_CSP_MODE::MODE_RGBA_4444 | WEBP_CSP_MODE::MODE_rgbA_4444 => {
            upsample_rgba4444_line_pair
        }
        _ => return None,
    };
    Some(upsampler)
}
