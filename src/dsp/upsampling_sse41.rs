// libwebp v1.6.0, src/dsp/upsampling_sse41.c:
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
// SSE41 version of YUV to RGB upsampling functions.
//
// Author: somnath@google.com (Somnath Banerjee)

//! Translation of `src/dsp/upsampling_sse41.c` from libwebp v1.6.0.
//!
//! Work package: WP18 (SIMD x86 decoder).
//!
//! The SSE4.1 fancy upsamplers and YUV 4:4:4 converters to RGB and BGR,
//! behind `webp_upsamplers` and `webp_yuv444_converters` of
//! `src/dsp/upsampling.rs` (ADR 0008). `WebPInitUpsamplers` and
//! `WebPInitYUV444Converters` run `WebPInitUpsamplersSSE41` and
//! `WebPInitYUV444ConvertersSSE41` after and independently of their SSE2
//! counterparts, so the dispatch functions test `features().sse4_1` alone,
//! before `features().sse2`.
//!
//! - `WebPInitUpsamplersSSE41` assigns `WebPUpsamplers[MODE_RGB, MODE_BGR]`:
//!   [`upsample_rgb_line_pair`], [`upsample_bgr_line_pair`].
//! - `WebPInitYUV444ConvertersSSE41` assigns `WebPYUV444Converters[MODE_RGB,
//!   MODE_BGR]`: [`yuv444_to_rgb`], [`yuv444_to_bgr`].
//!
//! The file repeats the upsampling macros of `upsampling_sse2.c` compiled
//! with SSE4.1; they are translated again here, as SSE4.1 functions, rather
//! than shared with `upsampling_sse2.rs`.
//!
//! # Deviations from upstream
//!
//! 1. **Unaligned stores, the plain cache array, the macros as generic
//!    functions and the bottom row as an `Option`**, as in
//!    `upsampling_sse2.rs` (deviations 1 to 3).
//! 2. **Panics on short buffers** come at the vector access, before or after
//!    other writes than in the scalar translation (`common_sse2.rs`,
//!    deviation 3).
//!
//! # Excluded undefined behavior
//!
//! None, as in `upsampling_sse2.rs`; `len >= 1` as in C.
//!
//! # Checks not applicable
//!
//! 1. **`WEBP_REDUCE_CSP`** is unset in the reference configuration, so the
//!    RGB and BGR functions are compiled and translated.
//! 2. **`WebPInitUpsamplersSSE41` and `WebPInitYUV444ConvertersSSE41`** only
//!    assign pointers (ADR 0003); their assignments are the selection in the
//!    dispatch functions listed above.
//! 3. **Differential probe.** The parity tests compare each function with the
//!    C translation it replaces (`tests/wp18_upsampling.rs`).

#[cfg(target_arch = "x86")]
use core::arch::x86 as arch;
#[cfg(target_arch = "x86_64")]
use core::arch::x86_64 as arch;

use arch::{
    __m128i, _mm_and_si128, _mm_avg_epu8, _mm_or_si128, _mm_set1_epi8, _mm_sub_epi8,
    _mm_unpackhi_epi8, _mm_unpacklo_epi8, _mm_xor_si128,
};

use crate::dsp::common_sse2::{loadu_si128, storeu_si128};
use crate::dsp::upsampling::{WebPYUV444Converter, webp_yuv444_to_bgr_c, webp_yuv444_to_rgb_c};
use crate::dsp::yuv::{vp8_yuv_to_bgr, vp8_yuv_to_rgb};
use crate::dsp::yuv_sse41::{vp8_yuv_to_bgr32_sse41, vp8_yuv_to_rgb32_sse41};

/// C's conversion of an `int` sample in `[0, 255]` to the `uint8_t`
/// parameter of the per-pixel conversion.
#[expect(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    reason = "C converts the int sample to uint8_t"
)]
fn to_u8(v: i32) -> u8 {
    v as u8
}

// We compute (9*a + 3*b + 3*c + d + 8) / 16 as follows
// u = (9*a + 3*b + 3*c + d + 8) / 16
//   = (a + (a + 3*b + 3*c + d) / 8 + 1) / 2
//   = (a + m + 1) / 2
// where m = (a + 3*b + 3*c + d) / 8
//         = ((a + b + c + d) / 2 + b + c) / 4
//
// Let's say  k = (a + b + c + d) / 4.
// We can compute k as
// k = (s + t + 1) / 2 - ((a^d) | (b^c) | (s^t)) & 1
// where s = (a + d + 1) / 2 and t = (b + c + 1) / 2
//
// Then m can be written as
// m = (k + t + 1) / 2 - (((b^c) & (s^t)) | (k^t)) & 1

// upstream: src/dsp/upsampling_sse41.c GET_M
/// Computes `out = (k + in + 1) / 2 - ((ij & (s^t)) | (k^in)) & 1`
#[target_feature(enable = "sse4.1")]
#[inline]
fn get_m(k: __m128i, st: __m128i, one: __m128i, ij: __m128i, input: __m128i) -> __m128i {
    let tmp0 = _mm_avg_epu8(k, input); // (k + in + 1) / 2
    let tmp1 = _mm_and_si128(ij, st); // (ij) & (s^t)
    let tmp2 = _mm_xor_si128(k, input); // (k^in)
    let tmp3 = _mm_or_si128(tmp1, tmp2); // ((ij) & (s^t)) | (k^in)
    let tmp4 = _mm_and_si128(tmp3, one); // & 1 -> lsb_correction
    _mm_sub_epi8(tmp0, tmp4) // (k + in + 1) / 2 - lsb_correction
}

// upstream: src/dsp/upsampling_sse41.c PACK_AND_STORE
/// pack and store two alternating pixel rows
#[target_feature(enable = "sse4.1")]
#[inline]
fn pack_and_store(a: __m128i, b: __m128i, da: __m128i, db: __m128i, out: &mut [u8]) {
    let t_a = _mm_avg_epu8(a, da); // (9a + 3b + 3c +  d + 8) / 16
    let t_b = _mm_avg_epu8(b, db); // (3a + 9b +  c + 3d + 8) / 16
    let t_1 = _mm_unpacklo_epi8(t_a, t_b);
    let t_2 = _mm_unpackhi_epi8(t_a, t_b);
    storeu_si128(&mut out[0..], t_1);
    storeu_si128(&mut out[16..], t_2);
}

// upstream: src/dsp/upsampling_sse41.c UPSAMPLE_32PIXELS
/// Loads 17 pixels each from rows r1 and r2 and generates 32 pixels.
#[target_feature(enable = "sse4.1")]
#[inline]
#[expect(clippy::many_single_char_names, reason = "the C names of the samples")]
fn upsample_32pixels(r1: &[u8], r2: &[u8], out: &mut [u8]) {
    let one = _mm_set1_epi8(1);
    let a = loadu_si128(&r1[0..]);
    let b = loadu_si128(&r1[1..]);
    let c = loadu_si128(&r2[0..]);
    let d = loadu_si128(&r2[1..]);

    let s = _mm_avg_epu8(a, d); // s = (a + d + 1) / 2
    let t = _mm_avg_epu8(b, c); // t = (b + c + 1) / 2
    let st = _mm_xor_si128(s, t); // st = s^t

    let ad = _mm_xor_si128(a, d); // ad = a^d
    let bc = _mm_xor_si128(b, c); // bc = b^c

    let t1 = _mm_or_si128(ad, bc); // (a^d) | (b^c)
    let t2 = _mm_or_si128(t1, st); // (a^d) | (b^c) | (s^t)
    let t3 = _mm_and_si128(t2, one); // (a^d) | (b^c) | (s^t) & 1
    let t4 = _mm_avg_epu8(s, t);
    let k = _mm_sub_epi8(t4, t3); // k = (a + b + c + d) / 4

    let diag1 = get_m(k, st, one, bc, t); // diag1 = (a + 3b + 3c + d) / 8
    let diag2 = get_m(k, st, one, ad, s); // diag2 = (3a + b + c + 3d) / 8

    // pack the alternate pixels
    pack_and_store(a, b, diag1, diag2, &mut out[0..]); // store top
    pack_and_store(c, d, diag2, diag1, &mut out[2 * 32..]); // store bottom
}

// upstream: src/dsp/upsampling_sse41.c Upsample32Pixels_SSE41
/// Turn the macro into a function for reducing code-size when non-critical
#[target_feature(enable = "sse4.1")]
fn upsample32_pixels_sse41(r1: &[u8], r2: &[u8], out: &mut [u8]) {
    upsample_32pixels(r1, r2, out);
}

// upstream: src/dsp/upsampling_sse41.c UPSAMPLE_LAST_BLOCK
/// Upsamples the last `num_pixels` (1 to 17) samples of `tb` and `bb`, with the
/// last sample replicated to 17.
#[target_feature(enable = "sse4.1")]
#[inline]
fn upsample_last_block(tb: &[u8], bb: &[u8], num_pixels: usize, out: &mut [u8]) {
    let mut r1 = [0_u8; 17];
    let mut r2 = [0_u8; 17];
    r1[..num_pixels].copy_from_slice(&tb[..num_pixels]);
    r2[..num_pixels].copy_from_slice(&bb[..num_pixels]);
    // replicate last byte
    let last1 = r1[num_pixels - 1];
    r1[num_pixels..].fill(last1);
    let last2 = r2[num_pixels - 1];
    r2[num_pixels..].fill(last2);
    // using the shared function instead of the macro saves ~3k code size
    upsample32_pixels_sse41(&r1, &r2, out);
}

// upstream: src/dsp/upsampling_sse41.c CONVERT2RGB_32
/// Converts the 32 pixels at `cur_x` of the top row and, when present, of the
/// bottom row, with the upsampled U and V of `r_u` (`r_u`, `r_v = r_u + 32`,
/// and the bottom row's at `+ 64`).
#[target_feature(enable = "sse4.1")]
#[inline]
fn convert2rgb_32(
    func32: &impl Fn(&[u8], &[u8], &[u8], &mut [u8]),
    xstep: usize,
    top_y: &[u8],
    bottom: Option<(&[u8], &mut [u8])>,
    top_dst: &mut [u8],
    cur_x: usize,
    r_u: &[u8],
) {
    func32(
        &top_y[cur_x..],
        &r_u[0..],
        &r_u[32..],
        &mut top_dst[cur_x * xstep..],
    );
    if let Some((bottom_y, bottom_dst)) = bottom {
        func32(
            &bottom_y[cur_x..],
            &r_u[64..],
            &r_u[32 + 64..],
            &mut bottom_dst[cur_x * xstep..],
        );
    }
}

// upstream: src/dsp/upsampling_sse41.c SSE4_UPSAMPLE_FUNC
/// The fancy upsampler of one output format: `func` converts one pixel and
/// `func32` 32 pixels (C: the macro's `FUNC` and `FUNC##32_SSE41`).
#[target_feature(enable = "sse4.1")]
#[expect(
    clippy::too_many_arguments,
    reason = "the C parameters with the macro's"
)]
#[expect(clippy::similar_names, reason = "the C names u0_t and v0_t")]
#[expect(
    clippy::int_plus_one,
    reason = "the C loop condition pos + 32 + 1 <= len"
)]
fn sse4_upsample_func(
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
    func32: impl Fn(&[u8], &[u8], &[u8], &mut [u8]),
) {
    let len = usize::try_from(len).expect("the row length is not negative");
    let mut bottom = bottom;
    // 16byte-aligned array to cache reconstructed u and v
    let mut uv_buf = [0_u8; 14 * 32];

    {
        // Treat the first pixel in regular way
        let u_diag = ((i32::from(top_u[0]) + i32::from(cur_u[0])) >> 1) + 1;
        let v_diag = ((i32::from(top_v[0]) + i32::from(cur_v[0])) >> 1) + 1;
        let u0_t = (i32::from(top_u[0]) + u_diag) >> 1;
        let v0_t = (i32::from(top_v[0]) + v_diag) >> 1;
        func(top_y[0], to_u8(u0_t), to_u8(v0_t), top_dst);
        if let Some((bottom_y, bottom_dst)) = bottom.as_mut() {
            let u0_b = (i32::from(cur_u[0]) + u_diag) >> 1;
            let v0_b = (i32::from(cur_v[0]) + v_diag) >> 1;
            func(bottom_y[0], to_u8(u0_b), to_u8(v0_b), bottom_dst);
        }
    }
    // For UPSAMPLE_32PIXELS, 17 u/v values must be read-able for each block
    let mut pos = 1;
    let mut uv_pos = 0;
    while pos + 32 + 1 <= len {
        upsample_32pixels(&top_u[uv_pos..], &cur_u[uv_pos..], &mut uv_buf[0..]);
        upsample_32pixels(&top_v[uv_pos..], &cur_v[uv_pos..], &mut uv_buf[32..]);
        convert2rgb_32(
            &func32,
            xstep,
            top_y,
            bottom.as_mut().map(|(y, dst)| (*y, &mut **dst)),
            top_dst,
            pos,
            &uv_buf,
        );
        pos += 32;
        uv_pos += 16;
    }
    if len > 1 {
        let left_over = ((len + 1) >> 1) - (pos >> 1);
        let (r_u, rest) = uv_buf.split_at_mut(4 * 32);
        let (tmp_top_dst, rest) = rest.split_at_mut(4 * 32);
        let (tmp_bottom_dst, rest) = rest.split_at_mut(4 * 32);
        let (tmp_top, tmp_bottom) = rest.split_at_mut(32);
        debug_assert!(left_over > 0);
        upsample_last_block(&top_u[uv_pos..], &cur_u[uv_pos..], left_over, &mut r_u[0..]);
        upsample_last_block(
            &top_v[uv_pos..],
            &cur_v[uv_pos..],
            left_over,
            &mut r_u[32..],
        );
        tmp_top[..len - pos].copy_from_slice(&top_y[pos..len]);
        if let Some((bottom_y, _)) = bottom.as_ref() {
            tmp_bottom[..len - pos].copy_from_slice(&bottom_y[pos..len]);
        }
        let tmp_bottom_pair = if bottom.is_some() {
            Some((&*tmp_bottom, &mut *tmp_bottom_dst))
        } else {
            None
        };
        convert2rgb_32(
            &func32,
            xstep,
            tmp_top,
            tmp_bottom_pair,
            tmp_top_dst,
            0,
            r_u,
        );
        top_dst[pos * xstep..len * xstep].copy_from_slice(&tmp_top_dst[..(len - pos) * xstep]);
        if let Some((_, bottom_dst)) = bottom {
            bottom_dst[pos * xstep..len * xstep]
                .copy_from_slice(&tmp_bottom_dst[..(len - pos) * xstep]);
        }
    }
}

// SSE4 variants of the fancy upsampler.

// upstream: src/dsp/upsampling_sse41.c UpsampleRgbLinePair_SSE41
#[target_feature(enable = "sse4.1")]
#[expect(clippy::too_many_arguments, reason = "the C parameters")]
fn upsample_rgb_line_pair_sse41(
    top_y: &[u8],
    bottom: Option<(&[u8], &mut [u8])>,
    top_u: &[u8],
    top_v: &[u8],
    cur_u: &[u8],
    cur_v: &[u8],
    top_dst: &mut [u8],
    len: i32,
) {
    sse4_upsample_func(
        top_y,
        bottom,
        top_u,
        top_v,
        cur_u,
        cur_v,
        top_dst,
        len,
        3,
        |y, u, v, dst| vp8_yuv_to_rgb(i32::from(y), i32::from(u), i32::from(v), dst),
        |y, u, v, dst| vp8_yuv_to_rgb32_sse41(y, u, v, dst),
    );
}

/// Entry point of [`upsample_rgb_line_pair_sse41`] for `webp_upsamplers`
/// (`MODE_RGB`).
#[expect(clippy::too_many_arguments, reason = "the C parameters")]
pub(crate) fn upsample_rgb_line_pair(
    top_y: &[u8],
    bottom: Option<(&[u8], &mut [u8])>,
    top_u: &[u8],
    top_v: &[u8],
    cur_u: &[u8],
    cur_v: &[u8],
    top_dst: &mut [u8],
    len: i32,
) {
    // SAFETY: `webp_upsamplers` calls this entry point only after
    // `features().sse4_1` selected the SSE4.1 variant, so the CPU has SSE4.1.
    unsafe {
        upsample_rgb_line_pair_sse41(top_y, bottom, top_u, top_v, cur_u, cur_v, top_dst, len);
    }
}

// upstream: src/dsp/upsampling_sse41.c UpsampleBgrLinePair_SSE41
#[target_feature(enable = "sse4.1")]
#[expect(clippy::too_many_arguments, reason = "the C parameters")]
fn upsample_bgr_line_pair_sse41(
    top_y: &[u8],
    bottom: Option<(&[u8], &mut [u8])>,
    top_u: &[u8],
    top_v: &[u8],
    cur_u: &[u8],
    cur_v: &[u8],
    top_dst: &mut [u8],
    len: i32,
) {
    sse4_upsample_func(
        top_y,
        bottom,
        top_u,
        top_v,
        cur_u,
        cur_v,
        top_dst,
        len,
        3,
        |y, u, v, dst| vp8_yuv_to_bgr(i32::from(y), i32::from(u), i32::from(v), dst),
        |y, u, v, dst| vp8_yuv_to_bgr32_sse41(y, u, v, dst),
    );
}

/// Entry point of [`upsample_bgr_line_pair_sse41`] for `webp_upsamplers`
/// (`MODE_BGR`).
#[expect(clippy::too_many_arguments, reason = "the C parameters")]
pub(crate) fn upsample_bgr_line_pair(
    top_y: &[u8],
    bottom: Option<(&[u8], &mut [u8])>,
    top_u: &[u8],
    top_v: &[u8],
    cur_u: &[u8],
    cur_v: &[u8],
    top_dst: &mut [u8],
    len: i32,
) {
    // SAFETY: `webp_upsamplers` calls this entry point only after
    // `features().sse4_1` selected the SSE4.1 variant, so the CPU has SSE4.1.
    unsafe {
        upsample_bgr_line_pair_sse41(top_y, bottom, top_u, top_v, cur_u, cur_v, top_dst, len);
    }
}

//------------------------------------------------------------------------------
// Entry point

// upstream: src/dsp/upsampling_sse41.c WebPInitUpsamplersSSE41 (no counterpart: it only assigns pointers, ADR 0003; the module documentation lists them)

//------------------------------------------------------------------------------

// upstream: src/dsp/upsampling_sse41.c YUV444_FUNC
/// Converts a row of YUV 4:4:4 samples, 32 at a time with `call` and the rest
/// with the C function `call_c` (C: the macro's `CALL` and `CALL_C`).
#[target_feature(enable = "sse4.1")]
#[inline]
#[expect(
    clippy::too_many_arguments,
    reason = "the C parameters with the macro's"
)]
fn yuv444_func(
    y: &[u8],
    u: &[u8],
    v: &[u8],
    dst: &mut [u8],
    len: i32,
    xstep: usize,
    call: impl Fn(&[u8], &[u8], &[u8], &mut [u8]),
    call_c: WebPYUV444Converter,
) {
    let max_len = len & !31;
    let mut i = 0;
    while i < max_len {
        let at = usize::try_from(i).expect("a non-negative position");
        call(&y[at..], &u[at..], &v[at..], &mut dst[at * xstep..]);
        i += 32;
    }
    if i < len {
        // C-fallback
        let at = usize::try_from(i).expect("a non-negative position");
        call_c(
            &y[at..],
            &u[at..],
            &v[at..],
            &mut dst[at * xstep..],
            len - i,
        );
    }
}

// upstream: src/dsp/upsampling_sse41.c Yuv444ToRgb_SSE41
#[target_feature(enable = "sse4.1")]
fn yuv444_to_rgb_sse41(y: &[u8], u: &[u8], v: &[u8], dst: &mut [u8], len: i32) {
    yuv444_func(
        y,
        u,
        v,
        dst,
        len,
        3,
        |y, u, v, dst| vp8_yuv_to_rgb32_sse41(y, u, v, dst),
        webp_yuv444_to_rgb_c,
    );
}

/// Entry point of [`yuv444_to_rgb_sse41`] for `webp_yuv444_converters`
/// (`MODE_RGB`).
pub(crate) fn yuv444_to_rgb(y: &[u8], u: &[u8], v: &[u8], dst: &mut [u8], len: i32) {
    // SAFETY: `webp_yuv444_converters` calls this entry point only after
    // `features().sse4_1` selected the SSE4.1 variant, so the CPU has SSE4.1.
    unsafe { yuv444_to_rgb_sse41(y, u, v, dst, len) }
}

// upstream: src/dsp/upsampling_sse41.c Yuv444ToBgr_SSE41
#[target_feature(enable = "sse4.1")]
fn yuv444_to_bgr_sse41(y: &[u8], u: &[u8], v: &[u8], dst: &mut [u8], len: i32) {
    yuv444_func(
        y,
        u,
        v,
        dst,
        len,
        3,
        |y, u, v, dst| vp8_yuv_to_bgr32_sse41(y, u, v, dst),
        webp_yuv444_to_bgr_c,
    );
}

/// Entry point of [`yuv444_to_bgr_sse41`] for `webp_yuv444_converters`
/// (`MODE_BGR`).
pub(crate) fn yuv444_to_bgr(y: &[u8], u: &[u8], v: &[u8], dst: &mut [u8], len: i32) {
    // SAFETY: `webp_yuv444_converters` calls this entry point only after
    // `features().sse4_1` selected the SSE4.1 variant, so the CPU has SSE4.1.
    unsafe { yuv444_to_bgr_sse41(y, u, v, dst, len) }
}

// upstream: src/dsp/upsampling_sse41.c WebPInitYUV444ConvertersSSE41 (no counterpart: it only assigns pointers, ADR 0003; the module documentation lists them)
