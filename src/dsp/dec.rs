// libwebp v1.6.0, src/dsp/dec.c:
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
// Speed-critical decoding functions, default plain-C implementations.
//
// Author: Skal (pascal.massimino@gmail.com)

//! Translation of `src/dsp/dec.c` from libwebp v1.6.0.
//!
//! Work package: WP6 (VP8 decoder core).
//!
//! The dispatch pointers (`VP8Transform`, `VP8PredLuma16`, the loop filters and
//! the others) are plain functions over the portable `_C` code, and the tables
//! of predictors are static arrays behind functions that take the mode first
//! (ADR 0003, "DSP dispatch"). `VP8DspInit` only assigns portable pointers in
//! the reference configuration, which compiles no SIMD, so it has no Rust
//! counterpart; its call of `VP8InitClipTables` is covered by the static tables
//! of `dec_clip_tables`. `WEBP_NEON_OMIT_C_CODE` is 0 there, so every `_C`
//! function is translated.
//!
//! A pointer that the C code only moves forward is a slice that starts at it. A
//! pointer from which the code also reads backwards, such as the predictors'
//! `dst[-1]` and `dst[-BPS]` or the filters' `p[-2 * step]`, is a slice and the
//! position of the pointer in it (ADR 0003, "Pointers"). A position outside the
//! slice panics, where C would read or write outside the buffer (WP4,
//! deviation 6).

use crate::dsp::dec_clip_tables::{VP8KABS0, VP8KCLIP1, VP8KSCLIP1, VP8KSCLIP2};
use crate::dsp::dsp::{
    BPS, VP8_DITHER_AMP_CENTER, VP8_DITHER_DESCALE, VP8_DITHER_DESCALE_ROUNDER,
    webp_transform_ac3_mul1, webp_transform_ac3_mul2,
};
use crate::utils::utils::webp_uint32_to_mem;

/// `BPS` as an index step.
const BPS_U: usize = BPS as usize;

/// The position `delta` elements away from `pos`, as C's pointer arithmetic
/// `p + delta`.
fn at(pos: usize, delta: i32) -> usize {
    pos.checked_add_signed(isize::try_from(delta).expect("an i32 fits in isize"))
        .expect("a position before the start of the buffer")
}

// upstream: src/dsp/dec.c clip_8b
fn clip_8b(v: i32) -> u8 {
    #[expect(
        clippy::cast_possible_truncation,
        clippy::cast_sign_loss,
        reason = "C returns the int in [0, 255] as uint8_t"
    )]
    if (v & !0xff) == 0 {
        v as u8
    } else if v < 0 {
        0
    } else {
        255
    }
}

//------------------------------------------------------------------------------
// Transforms (Paragraph 14.4)

// upstream: src/dsp/dec.c STORE
/// `dst[x + y * BPS] = clip_8b(dst[x + y * BPS] + (v >> 3))`.
fn store(dst: &mut [u8], x: usize, y: usize, v: i32) {
    let i = x + y * BPS_U;
    dst[i] = clip_8b(i32::from(dst[i]) + (v >> 3));
}

// upstream: src/dsp/dec.c STORE2
fn store2(dst: &mut [u8], y: usize, dc: i32, d: i32, c: i32) {
    store(dst, 0, y, dc + d);
    store(dst, 1, y, dc + c);
    store(dst, 2, y, dc - c);
    store(dst, 3, y, dc - d);
}

// upstream: src/dsp/dec.c TransformOne_C
/// Inverse transform of one 4x4 block of coefficients, added to `dst`.
fn transform_one_c(input: &[i16], dst: &mut [u8]) {
    let mut c = [0i32; 4 * 4];
    let mut tmp = 0;
    for i in 0..4 {
        // vertical pass
        let a = i32::from(input[i]) + i32::from(input[i + 8]); // [-4096, 4094]
        let b = i32::from(input[i]) - i32::from(input[i + 8]); // [-4095, 4095]
        let cc = webp_transform_ac3_mul2(i32::from(input[i + 4]))
            - webp_transform_ac3_mul1(i32::from(input[i + 12])); // [-3783, 3783]
        let d = webp_transform_ac3_mul1(i32::from(input[i + 4]))
            + webp_transform_ac3_mul2(i32::from(input[i + 12])); // [-3785, 3781]
        c[tmp] = a + d; // [-7881, 7875]
        c[tmp + 1] = b + cc; // [-7878, 7878]
        c[tmp + 2] = b - cc; // [-7878, 7878]
        c[tmp + 3] = a - d; // [-7877, 7879]
        tmp += 4;
    }
    // Each pass is expanding the dynamic range by ~3.85 (upper bound).
    // The exact value is (2. + (20091 + 35468) / 65536).
    // After the second pass, maximum interval is [-3794, 3794], assuming
    // an input in [-2048, 2047] interval. We then need to add a dst value
    // in the [0, 255] range.
    // In the worst case scenario, the input to clip_8b() can be as large as
    // [-60713, 60968].
    for i in 0..4 {
        // horizontal pass
        let dc = c[i] + 4;
        let a = dc + c[i + 8];
        let b = dc - c[i + 8];
        let cc = webp_transform_ac3_mul2(c[i + 4]) - webp_transform_ac3_mul1(c[i + 12]);
        let d = webp_transform_ac3_mul1(c[i + 4]) + webp_transform_ac3_mul2(c[i + 12]);
        let row = &mut dst[i * BPS_U..];
        store(row, 0, 0, a + d);
        store(row, 1, 0, b + cc);
        store(row, 2, 0, b - cc);
        store(row, 3, 0, a - d);
    }
}

// upstream: src/dsp/dec.c TransformAC3_C
/// Simplified transform when only `in[0]`, `in[1]` and `in[4]` are non-zero.
pub fn transform_ac3_c(input: &[i16], dst: &mut [u8]) {
    let a = i32::from(input[0]) + 4;
    let c4 = webp_transform_ac3_mul2(i32::from(input[4]));
    let d4 = webp_transform_ac3_mul1(i32::from(input[4]));
    let c1 = webp_transform_ac3_mul2(i32::from(input[1]));
    let d1 = webp_transform_ac3_mul1(i32::from(input[1]));
    store2(dst, 0, a + d4, d1, c1);
    store2(dst, 1, a + c4, d1, c1);
    store2(dst, 2, a - c4, d1, c1);
    store2(dst, 3, a - d4, d1, c1);
}

// upstream: src/dsp/dec.c TransformTwo_C
/// One block, or two side by side when `do_two` is set.
pub fn transform_two_c(input: &[i16], dst: &mut [u8], do_two: bool) {
    transform_one_c(input, dst);
    if do_two {
        transform_one_c(&input[16..], &mut dst[4..]);
    }
}

// upstream: src/dsp/dec.c TransformUV_C
fn transform_uv_c(input: &[i16], dst: &mut [u8]) {
    vp8_transform(input, dst, true);
    vp8_transform(&input[2 * 16..], &mut dst[4 * BPS_U..], true);
}

// upstream: src/dsp/dec.c TransformDC_C
pub fn transform_dc_c(input: &[i16], dst: &mut [u8]) {
    let dc = i32::from(input[0]) + 4;
    for j in 0..4 {
        for i in 0..4 {
            store(dst, i, j, dc);
        }
    }
}

// upstream: src/dsp/dec.c TransformDCUV_C
fn transform_dcuv_c(input: &[i16], dst: &mut [u8]) {
    if input[0] != 0 {
        vp8_transform_dc(input, dst);
    }
    if input[16] != 0 {
        vp8_transform_dc(&input[16..], &mut dst[4..]);
    }
    if input[2 * 16] != 0 {
        vp8_transform_dc(&input[2 * 16..], &mut dst[4 * BPS_U..]);
    }
    if input[3 * 16] != 0 {
        vp8_transform_dc(&input[3 * 16..], &mut dst[4 * BPS_U + 4..]);
    }
}

//------------------------------------------------------------------------------
// Paragraph 14.3

// upstream: src/dsp/dec.c TransformWHT_C
/// Inverse Walsh-Hadamard transform of the 16 DC coefficients into the DC
/// position of each of the 16 blocks of `out`.
pub fn transform_wht_c(input: &[i16], out: &mut [i16]) {
    let mut tmp = [0i32; 16];
    for i in 0..4 {
        let a0 = i32::from(input[i]) + i32::from(input[12 + i]);
        let a1 = i32::from(input[4 + i]) + i32::from(input[8 + i]);
        let a2 = i32::from(input[4 + i]) - i32::from(input[8 + i]);
        let a3 = i32::from(input[i]) - i32::from(input[12 + i]);
        tmp[i] = a0 + a1;
        tmp[8 + i] = a0 - a1;
        tmp[4 + i] = a3 + a2;
        tmp[12 + i] = a3 - a2;
    }
    let mut out_pos = 0;
    for i in 0..4 {
        let dc = tmp[i * 4] + 3; // w/ rounder
        let a0 = dc + tmp[3 + i * 4];
        let a1 = tmp[1 + i * 4] + tmp[2 + i * 4];
        let a2 = tmp[1 + i * 4] - tmp[2 + i * 4];
        let a3 = dc - tmp[3 + i * 4];
        #[expect(
            clippy::cast_possible_truncation,
            reason = "C stores the int results in int16_t"
        )]
        {
            out[out_pos] = ((a0 + a1) >> 3) as i16;
            out[out_pos + 16] = ((a3 + a2) >> 3) as i16;
            out[out_pos + 32] = ((a0 - a1) >> 3) as i16;
            out[out_pos + 48] = ((a3 - a2) >> 3) as i16;
        }
        out_pos += 64;
    }
}

// upstream: src/dsp/dsp.h VP8DecIdct2
/// An inverse transform of one block, or two side by side (C: `VP8DecIdct2`).
pub type VP8DecIdct2 = fn(&[i16], &mut [u8], bool);

// upstream: src/dsp/dsp.h VP8DecIdct
/// An inverse transform of one block (C: `VP8DecIdct`).
pub type VP8DecIdct = fn(&[i16], &mut [u8]);

// upstream: src/dsp/dec.c VP8Transform
/// Inverse transform of one block, or two when `do_two` is set, added to `dst`
/// (C: the `VP8Transform` pointer, `TransformTwo_C`).
pub fn vp8_transform(input: &[i16], dst: &mut [u8], do_two: bool) {
    #[cfg(all(feature = "simd", target_arch = "aarch64"))]
    {
        return crate::dsp::dec_neon::transform_two(input, dst, do_two);
    }
    #[allow(
        unreachable_code,
        reason = "the C path is reached only without a SIMD variant"
    )]
    transform_two_c(input, dst, do_two);
}

// upstream: src/dsp/dec.c VP8TransformAC3
/// Inverse transform when only `in[0]`, `in[1]` and `in[4]` are non-zero (C:
/// the `VP8TransformAC3` pointer, `TransformAC3_C`).
pub fn vp8_transform_ac3(input: &[i16], dst: &mut [u8]) {
    #[cfg(all(feature = "simd", target_arch = "aarch64"))]
    {
        return crate::dsp::dec_neon::transform_ac3(input, dst);
    }
    #[allow(
        unreachable_code,
        reason = "the C path is reached only without a SIMD variant"
    )]
    transform_ac3_c(input, dst);
}

// upstream: src/dsp/dec.c VP8TransformUV
/// Inverse transform of the four blocks of one chroma plane (C: the
/// `VP8TransformUV` pointer, `TransformUV_C`).
pub fn vp8_transform_uv(input: &[i16], dst: &mut [u8]) {
    transform_uv_c(input, dst);
}

// upstream: src/dsp/dec.c VP8TransformDC
/// Inverse transform of a block with only a DC coefficient (C: the
/// `VP8TransformDC` pointer, `TransformDC_C`).
pub fn vp8_transform_dc(input: &[i16], dst: &mut [u8]) {
    #[cfg(all(feature = "simd", target_arch = "aarch64"))]
    {
        return crate::dsp::dec_neon::transform_dc(input, dst);
    }
    #[allow(
        unreachable_code,
        reason = "the C path is reached only without a SIMD variant"
    )]
    transform_dc_c(input, dst);
}

// upstream: src/dsp/dec.c VP8TransformDCUV
/// DC-only inverse transform of the four blocks of one chroma plane (C: the
/// `VP8TransformDCUV` pointer, `TransformDCUV_C`).
pub fn vp8_transform_dcuv(input: &[i16], dst: &mut [u8]) {
    transform_dcuv_c(input, dst);
}

// upstream: src/dsp/dec.c VP8TransformWHT
/// Inverse Walsh-Hadamard transform (C: the `VP8TransformWHT` pointer,
/// `TransformWHT_C`).
pub fn vp8_transform_wht(input: &[i16], out: &mut [i16]) {
    #[cfg(all(feature = "simd", target_arch = "aarch64"))]
    {
        return crate::dsp::dec_neon::transform_wht(input, out);
    }
    #[allow(
        unreachable_code,
        reason = "the C path is reached only without a SIMD variant"
    )]
    transform_wht_c(input, out);
}

//------------------------------------------------------------------------------
// Intra predictions

// upstream: src/dsp/dec.c DST
/// The position of `DST(x, y)`, `dst[x + y * BPS]`.
const fn dst_at(dst: usize, x: usize, y: usize) -> usize {
    dst + x + y * BPS_U
}

// upstream: src/dsp/dec.c TrueMotion
fn true_motion(buf: &mut [u8], dst: usize, size: usize) {
    let top = dst - BPS_U;
    // C: clip0 = VP8kclip1 - top[-1]; clip = clip0 + dst[-1]; dst[x] = clip[top[x]].
    let top_left = i32::from(buf[top - 1]);
    let mut row = dst;
    for _ in 0..size {
        let left = i32::from(buf[row - 1]);
        for x in 0..size {
            buf[row + x] = VP8KCLIP1[i32::from(buf[top + x]) + left - top_left];
        }
        row += BPS_U;
    }
}

// upstream: src/dsp/dec.c TM4_C
pub fn tm4_c(buf: &mut [u8], dst: usize) {
    true_motion(buf, dst, 4);
}

// upstream: src/dsp/dec.c TM8uv_C
pub fn tm8uv_c(buf: &mut [u8], dst: usize) {
    true_motion(buf, dst, 8);
}

// upstream: src/dsp/dec.c TM16_C
pub fn tm16_c(buf: &mut [u8], dst: usize) {
    true_motion(buf, dst, 16);
}

//------------------------------------------------------------------------------
// 16x16

// upstream: src/dsp/dec.c VE16_C
/// Vertical prediction.
pub fn ve16_c(buf: &mut [u8], dst: usize) {
    for j in 0..16 {
        buf.copy_within(dst - BPS_U..dst - BPS_U + 16, dst + j * BPS_U);
    }
}

// upstream: src/dsp/dec.c HE16_C
/// Horizontal prediction.
pub fn he16_c(buf: &mut [u8], dst: usize) {
    let mut row = dst;
    for _ in 0..16 {
        let left = buf[row - 1];
        buf[row..row + 16].fill(left);
        row += BPS_U;
    }
}

// upstream: src/dsp/dec.c Put16
#[expect(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    reason = "memset converts the int value to unsigned char"
)]
fn put16(v: i32, buf: &mut [u8], dst: usize) {
    for j in 0..16 {
        let row = dst + j * BPS_U;
        buf[row..row + 16].fill(v as u8);
    }
}

// upstream: src/dsp/dec.c DC16_C
/// DC prediction.
pub fn dc16_c(buf: &mut [u8], dst: usize) {
    let mut dc = 16;
    for j in 0..16 {
        dc += i32::from(buf[dst - 1 + j * BPS_U]) + i32::from(buf[dst + j - BPS_U]);
    }
    put16(dc >> 5, buf, dst);
}

// upstream: src/dsp/dec.c DC16NoTop_C
/// DC with top samples not available.
pub fn dc16_no_top_c(buf: &mut [u8], dst: usize) {
    let mut dc = 8;
    for j in 0..16 {
        dc += i32::from(buf[dst - 1 + j * BPS_U]);
    }
    put16(dc >> 4, buf, dst);
}

// upstream: src/dsp/dec.c DC16NoLeft_C
/// DC with left samples not available.
pub fn dc16_no_left_c(buf: &mut [u8], dst: usize) {
    let mut dc = 8;
    for i in 0..16 {
        dc += i32::from(buf[dst + i - BPS_U]);
    }
    put16(dc >> 4, buf, dst);
}

// upstream: src/dsp/dec.c DC16NoTopLeft_C
/// DC with no top and left samples.
pub fn dc16_no_top_left_c(buf: &mut [u8], dst: usize) {
    put16(0x80, buf, dst);
}

// upstream: src/dsp/dsp.h VP8PredFunc
/// An intra predictor: the buffer and the position of the block in it, whose
/// top and left samples it reads at negative offsets (C: `VP8PredFunc`).
pub type VP8PredFunc = fn(&mut [u8], usize);

// upstream: src/dsp/dec.c VP8PredLuma16
/// The 16x16 predictors, indexed by mode (C: `VP8PredLuma16`, filled by
/// `VP8DspInit`).
static VP8_PRED_LUMA16: [VP8PredFunc; 7] = [
    dc16_c,
    tm16_c,
    ve16_c,
    he16_c,
    dc16_no_top_c,
    dc16_no_left_c,
    dc16_no_top_left_c,
];

// upstream: src/dsp/dec_neon.c VP8DspInitNEON (the VP8PredLuma16 entries)
/// `VP8PredLuma16` after `VP8DspInitNEON`, which replaces every entry and
/// which `VP8DspInit` calls unconditionally on 64-bit ARM.
#[cfg(all(feature = "simd", target_arch = "aarch64"))]
static VP8_PRED_LUMA16_NEON: [VP8PredFunc; 7] = [
    crate::dsp::dec_neon::dc16_top_left,
    crate::dsp::dec_neon::tm16,
    crate::dsp::dec_neon::ve16,
    crate::dsp::dec_neon::he16,
    crate::dsp::dec_neon::dc16_no_top,
    crate::dsp::dec_neon::dc16_no_left,
    crate::dsp::dec_neon::dc16_no_top_left,
];

// upstream: src/dsp/dec.c VP8PredLuma16
/// Predicts the 16x16 luma block at `dst` with mode `mode` (C:
/// `VP8PredLuma16[mode](dst)`).
///
/// # Panics
///
/// Panics for a mode outside `[0, NUM_B_DC_MODES)`, where C indexes outside its
/// table, and when the block or its top and left samples lie outside `buf`.
pub fn vp8_pred_luma16(mode: i32, buf: &mut [u8], dst: usize) {
    #[cfg(all(feature = "simd", target_arch = "aarch64"))]
    {
        return VP8_PRED_LUMA16_NEON[usize::try_from(mode).expect("a non-negative mode")](buf, dst);
    }
    #[allow(
        unreachable_code,
        reason = "the C path is reached only without a SIMD variant"
    )]
    VP8_PRED_LUMA16[usize::try_from(mode).expect("a non-negative mode")](buf, dst);
}

//------------------------------------------------------------------------------
// 4x4

// upstream: src/dsp/dec.c AVG3
#[expect(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    reason = "the C macro casts the average to uint8_t"
)]
fn avg3(a: i32, b: i32, c: i32) -> u8 {
    ((a + 2 * b + c + 2) >> 2) as u8
}

// upstream: src/dsp/dec.c AVG2
#[expect(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    reason = "C stores the int average in uint8_t"
)]
fn avg2(a: i32, b: i32) -> u8 {
    ((a + b + 1) >> 1) as u8
}

// upstream: src/dsp/dec.c VE4_C
/// Vertical prediction.
pub fn ve4_c(buf: &mut [u8], dst: usize) {
    let top = dst - BPS_U;
    let t = |k: usize| i32::from(buf[top - 1 + k]);
    let vals = [
        avg3(t(0), t(1), t(2)),
        avg3(t(1), t(2), t(3)),
        avg3(t(2), t(3), t(4)),
        avg3(t(3), t(4), t(5)),
    ];
    for i in 0..4 {
        let row = dst + i * BPS_U;
        buf[row..row + 4].copy_from_slice(&vals);
    }
}

// upstream: src/dsp/dec.c HE4_C
/// Horizontal prediction.
#[expect(clippy::many_single_char_names, reason = "the C sample names")]
fn he4_c(buf: &mut [u8], dst: usize) {
    let a = i32::from(buf[dst - 1 - BPS_U]);
    let b = i32::from(buf[dst - 1]);
    let c = i32::from(buf[dst - 1 + BPS_U]);
    let d = i32::from(buf[dst - 1 + 2 * BPS_U]);
    let e = i32::from(buf[dst - 1 + 3 * BPS_U]);
    webp_uint32_to_mem(&mut buf[dst..], 0x0101_0101_u32 * u32::from(avg3(a, b, c)));
    webp_uint32_to_mem(
        &mut buf[dst + BPS_U..],
        0x0101_0101_u32 * u32::from(avg3(b, c, d)),
    );
    webp_uint32_to_mem(
        &mut buf[dst + 2 * BPS_U..],
        0x0101_0101_u32 * u32::from(avg3(c, d, e)),
    );
    webp_uint32_to_mem(
        &mut buf[dst + 3 * BPS_U..],
        0x0101_0101_u32 * u32::from(avg3(d, e, e)),
    );
}

// upstream: src/dsp/dec.c DC4_C
/// DC prediction.
pub fn dc4_c(buf: &mut [u8], dst: usize) {
    let mut dc: u32 = 4;
    for i in 0..4 {
        dc += u32::from(buf[dst + i - BPS_U]) + u32::from(buf[dst - 1 + i * BPS_U]);
    }
    dc >>= 3;
    #[expect(
        clippy::cast_possible_truncation,
        reason = "memset converts the value to unsigned char"
    )]
    let value = dc as u8;
    for i in 0..4 {
        let row = dst + i * BPS_U;
        buf[row..row + 4].fill(value);
    }
}

// upstream: src/dsp/dec.c RD4_C
/// Down-right prediction.
#[expect(clippy::many_single_char_names, reason = "the C sample names")]
pub fn rd4_c(buf: &mut [u8], dst: usize) {
    let i = i32::from(buf[dst - 1]);
    let j = i32::from(buf[dst - 1 + BPS_U]);
    let k = i32::from(buf[dst - 1 + 2 * BPS_U]);
    let l = i32::from(buf[dst - 1 + 3 * BPS_U]);
    let x = i32::from(buf[dst - 1 - BPS_U]);
    let a = i32::from(buf[dst - BPS_U]);
    let b = i32::from(buf[dst + 1 - BPS_U]);
    let c = i32::from(buf[dst + 2 - BPS_U]);
    let d = i32::from(buf[dst + 3 - BPS_U]);
    let mut put = |pos: &[(usize, usize)], v: u8| {
        for &(px, py) in pos {
            buf[dst_at(dst, px, py)] = v;
        }
    };
    put(&[(0, 3)], avg3(j, k, l));
    put(&[(1, 3), (0, 2)], avg3(i, j, k));
    put(&[(2, 3), (1, 2), (0, 1)], avg3(x, i, j));
    put(&[(3, 3), (2, 2), (1, 1), (0, 0)], avg3(a, x, i));
    put(&[(3, 2), (2, 1), (1, 0)], avg3(b, a, x));
    put(&[(3, 1), (2, 0)], avg3(c, b, a));
    put(&[(3, 0)], avg3(d, c, b));
}

// upstream: src/dsp/dec.c LD4_C
/// Down-left prediction.
#[expect(clippy::many_single_char_names, reason = "the C sample names")]
pub fn ld4_c(buf: &mut [u8], dst: usize) {
    let a = i32::from(buf[dst - BPS_U]);
    let b = i32::from(buf[dst + 1 - BPS_U]);
    let c = i32::from(buf[dst + 2 - BPS_U]);
    let d = i32::from(buf[dst + 3 - BPS_U]);
    let e = i32::from(buf[dst + 4 - BPS_U]);
    let f = i32::from(buf[dst + 5 - BPS_U]);
    let g = i32::from(buf[dst + 6 - BPS_U]);
    let h = i32::from(buf[dst + 7 - BPS_U]);
    let mut put = |pos: &[(usize, usize)], v: u8| {
        for &(px, py) in pos {
            buf[dst_at(dst, px, py)] = v;
        }
    };
    put(&[(0, 0)], avg3(a, b, c));
    put(&[(1, 0), (0, 1)], avg3(b, c, d));
    put(&[(2, 0), (1, 1), (0, 2)], avg3(c, d, e));
    put(&[(3, 0), (2, 1), (1, 2), (0, 3)], avg3(d, e, f));
    put(&[(3, 1), (2, 2), (1, 3)], avg3(e, f, g));
    put(&[(3, 2), (2, 3)], avg3(f, g, h));
    put(&[(3, 3)], avg3(g, h, h));
}

// upstream: src/dsp/dec.c VR4_C
/// Vertical-right prediction.
#[expect(clippy::many_single_char_names, reason = "the C sample names")]
fn vr4_c(buf: &mut [u8], dst: usize) {
    let i = i32::from(buf[dst - 1]);
    let j = i32::from(buf[dst - 1 + BPS_U]);
    let k = i32::from(buf[dst - 1 + 2 * BPS_U]);
    let x = i32::from(buf[dst - 1 - BPS_U]);
    let a = i32::from(buf[dst - BPS_U]);
    let b = i32::from(buf[dst + 1 - BPS_U]);
    let c = i32::from(buf[dst + 2 - BPS_U]);
    let d = i32::from(buf[dst + 3 - BPS_U]);
    let mut put = |pos: &[(usize, usize)], v: u8| {
        for &(px, py) in pos {
            buf[dst_at(dst, px, py)] = v;
        }
    };
    put(&[(0, 0), (1, 2)], avg2(x, a));
    put(&[(1, 0), (2, 2)], avg2(a, b));
    put(&[(2, 0), (3, 2)], avg2(b, c));
    put(&[(3, 0)], avg2(c, d));

    put(&[(0, 3)], avg3(k, j, i));
    put(&[(0, 2)], avg3(j, i, x));
    put(&[(0, 1), (1, 3)], avg3(i, x, a));
    put(&[(1, 1), (2, 3)], avg3(x, a, b));
    put(&[(2, 1), (3, 3)], avg3(a, b, c));
    put(&[(3, 1)], avg3(b, c, d));
}

// upstream: src/dsp/dec.c VL4_C
/// Vertical-left prediction.
#[expect(clippy::many_single_char_names, reason = "the C sample names")]
fn vl4_c(buf: &mut [u8], dst: usize) {
    let a = i32::from(buf[dst - BPS_U]);
    let b = i32::from(buf[dst + 1 - BPS_U]);
    let c = i32::from(buf[dst + 2 - BPS_U]);
    let d = i32::from(buf[dst + 3 - BPS_U]);
    let e = i32::from(buf[dst + 4 - BPS_U]);
    let f = i32::from(buf[dst + 5 - BPS_U]);
    let g = i32::from(buf[dst + 6 - BPS_U]);
    let h = i32::from(buf[dst + 7 - BPS_U]);
    let mut put = |pos: &[(usize, usize)], v: u8| {
        for &(px, py) in pos {
            buf[dst_at(dst, px, py)] = v;
        }
    };
    put(&[(0, 0)], avg2(a, b));
    put(&[(1, 0), (0, 2)], avg2(b, c));
    put(&[(2, 0), (1, 2)], avg2(c, d));
    put(&[(3, 0), (2, 2)], avg2(d, e));

    put(&[(0, 1)], avg3(a, b, c));
    put(&[(1, 1), (0, 3)], avg3(b, c, d));
    put(&[(2, 1), (1, 3)], avg3(c, d, e));
    put(&[(3, 1), (2, 3)], avg3(d, e, f));
    put(&[(3, 2)], avg3(e, f, g));
    put(&[(3, 3)], avg3(f, g, h));
}

// upstream: src/dsp/dec.c HU4_C
/// Horizontal-up prediction.
fn hu4_c(buf: &mut [u8], dst: usize) {
    let i = i32::from(buf[dst - 1]);
    let j = i32::from(buf[dst - 1 + BPS_U]);
    let k = i32::from(buf[dst - 1 + 2 * BPS_U]);
    let l = buf[dst - 1 + 3 * BPS_U];
    let li = i32::from(l);
    let mut put = |pos: &[(usize, usize)], v: u8| {
        for &(px, py) in pos {
            buf[dst_at(dst, px, py)] = v;
        }
    };
    put(&[(0, 0)], avg2(i, j));
    put(&[(2, 0), (0, 1)], avg2(j, k));
    put(&[(2, 1), (0, 2)], avg2(k, li));
    put(&[(1, 0)], avg3(i, j, k));
    put(&[(3, 0), (1, 1)], avg3(j, k, li));
    put(&[(3, 1), (1, 2)], avg3(k, li, li));
    put(&[(3, 2), (2, 2), (0, 3), (1, 3), (2, 3), (3, 3)], l);
}

// upstream: src/dsp/dec.c HD4_C
/// Horizontal-down prediction.
#[expect(clippy::many_single_char_names, reason = "the C sample names")]
fn hd4_c(buf: &mut [u8], dst: usize) {
    let i = i32::from(buf[dst - 1]);
    let j = i32::from(buf[dst - 1 + BPS_U]);
    let k = i32::from(buf[dst - 1 + 2 * BPS_U]);
    let l = i32::from(buf[dst - 1 + 3 * BPS_U]);
    let x = i32::from(buf[dst - 1 - BPS_U]);
    let a = i32::from(buf[dst - BPS_U]);
    let b = i32::from(buf[dst + 1 - BPS_U]);
    let c = i32::from(buf[dst + 2 - BPS_U]);
    let mut put = |pos: &[(usize, usize)], v: u8| {
        for &(px, py) in pos {
            buf[dst_at(dst, px, py)] = v;
        }
    };
    put(&[(0, 0), (2, 1)], avg2(i, x));
    put(&[(0, 1), (2, 2)], avg2(j, i));
    put(&[(0, 2), (2, 3)], avg2(k, j));
    put(&[(0, 3)], avg2(l, k));

    put(&[(3, 0)], avg3(a, b, c));
    put(&[(2, 0)], avg3(x, a, b));
    put(&[(1, 0), (3, 1)], avg3(i, x, a));
    put(&[(1, 1), (3, 2)], avg3(j, i, x));
    put(&[(1, 2), (3, 3)], avg3(k, j, i));
    put(&[(1, 3)], avg3(l, k, j));
}

// upstream: src/dsp/dec.c VP8PredLuma4
/// The 4x4 predictors, indexed by mode (C: `VP8PredLuma4`, filled by
/// `VP8DspInit`).
static VP8_PRED_LUMA4: [VP8PredFunc; 10] = [
    dc4_c, tm4_c, ve4_c, he4_c, rd4_c, vr4_c, ld4_c, vl4_c, hd4_c, hu4_c,
];

// upstream: src/dsp/dec_neon.c VP8DspInitNEON (the VP8PredLuma4 entries)
/// `VP8PredLuma4` after `VP8DspInitNEON`, which replaces the entries 0, 1,
/// 2, 4 and 6 and which `VP8DspInit` calls unconditionally on 64-bit ARM;
/// the other entries stay the C functions.
#[cfg(all(feature = "simd", target_arch = "aarch64"))]
static VP8_PRED_LUMA4_NEON: [VP8PredFunc; 10] = [
    crate::dsp::dec_neon::dc4,
    crate::dsp::dec_neon::tm4,
    crate::dsp::dec_neon::ve4,
    he4_c,
    crate::dsp::dec_neon::rd4,
    vr4_c,
    crate::dsp::dec_neon::ld4,
    vl4_c,
    hd4_c,
    hu4_c,
];

// upstream: src/dsp/dec.c VP8PredLuma4
/// Predicts the 4x4 luma block at `dst` with mode `mode` (C:
/// `VP8PredLuma4[mode](dst)`).
///
/// # Panics
///
/// Panics for a mode outside `[0, NUM_BMODES)`, where C indexes outside its
/// table, and when the block or its top and left samples lie outside `buf`.
pub fn vp8_pred_luma4(mode: i32, buf: &mut [u8], dst: usize) {
    #[cfg(all(feature = "simd", target_arch = "aarch64"))]
    {
        return VP8_PRED_LUMA4_NEON[usize::try_from(mode).expect("a non-negative mode")](buf, dst);
    }
    #[allow(
        unreachable_code,
        reason = "the C path is reached only without a SIMD variant"
    )]
    VP8_PRED_LUMA4[usize::try_from(mode).expect("a non-negative mode")](buf, dst);
}

//------------------------------------------------------------------------------
// Chroma

// upstream: src/dsp/dec.c VE8uv_C
/// Vertical prediction.
pub fn ve8uv_c(buf: &mut [u8], dst: usize) {
    for j in 0..8 {
        buf.copy_within(dst - BPS_U..dst - BPS_U + 8, dst + j * BPS_U);
    }
}

// upstream: src/dsp/dec.c HE8uv_C
/// Horizontal prediction.
pub fn he8uv_c(buf: &mut [u8], dst: usize) {
    let mut row = dst;
    for _ in 0..8 {
        let left = buf[row - 1];
        buf[row..row + 8].fill(left);
        row += BPS_U;
    }
}

// upstream: src/dsp/dec.c Put8x8uv
/// Helper for chroma-DC predictions.
fn put8x8uv(value: u8, buf: &mut [u8], dst: usize) {
    for j in 0..8 {
        let row = dst + j * BPS_U;
        buf[row..row + 8].fill(value);
    }
}

/// The `uint8_t` parameter of `Put8x8uv`, to which C converts the int.
#[expect(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    reason = "C converts the int argument to the uint8_t parameter"
)]
fn to_u8(v: i32) -> u8 {
    v as u8
}

// upstream: src/dsp/dec.c DC8uv_C
/// DC prediction.
pub fn dc8uv_c(buf: &mut [u8], dst: usize) {
    let mut dc0 = 8;
    for i in 0..8 {
        dc0 += i32::from(buf[dst + i - BPS_U]) + i32::from(buf[dst - 1 + i * BPS_U]);
    }
    put8x8uv(to_u8(dc0 >> 4), buf, dst);
}

// upstream: src/dsp/dec.c DC8uvNoLeft_C
/// DC with no left samples.
pub fn dc8uv_no_left_c(buf: &mut [u8], dst: usize) {
    let mut dc0 = 4;
    for i in 0..8 {
        dc0 += i32::from(buf[dst + i - BPS_U]);
    }
    put8x8uv(to_u8(dc0 >> 3), buf, dst);
}

// upstream: src/dsp/dec.c DC8uvNoTop_C
/// DC with no top samples.
pub fn dc8uv_no_top_c(buf: &mut [u8], dst: usize) {
    let mut dc0 = 4;
    for i in 0..8 {
        dc0 += i32::from(buf[dst - 1 + i * BPS_U]);
    }
    put8x8uv(to_u8(dc0 >> 3), buf, dst);
}

// upstream: src/dsp/dec.c DC8uvNoTopLeft_C
/// DC with nothing.
pub fn dc8uv_no_top_left_c(buf: &mut [u8], dst: usize) {
    put8x8uv(0x80, buf, dst);
}

// upstream: src/dsp/dec.c VP8PredChroma8
/// The 8x8 chroma predictors, indexed by mode (C: `VP8PredChroma8`, filled by
/// `VP8DspInit`).
static VP8_PRED_CHROMA8: [VP8PredFunc; 7] = [
    dc8uv_c,
    tm8uv_c,
    ve8uv_c,
    he8uv_c,
    dc8uv_no_top_c,
    dc8uv_no_left_c,
    dc8uv_no_top_left_c,
];

// upstream: src/dsp/dec_neon.c VP8DspInitNEON (the VP8PredChroma8 entries)
/// `VP8PredChroma8` after `VP8DspInitNEON`, which replaces every entry and
/// which `VP8DspInit` calls unconditionally on 64-bit ARM.
#[cfg(all(feature = "simd", target_arch = "aarch64"))]
static VP8_PRED_CHROMA8_NEON: [VP8PredFunc; 7] = [
    crate::dsp::dec_neon::dc8uv,
    crate::dsp::dec_neon::tm8uv,
    crate::dsp::dec_neon::ve8uv,
    crate::dsp::dec_neon::he8uv,
    crate::dsp::dec_neon::dc8uv_no_top,
    crate::dsp::dec_neon::dc8uv_no_left,
    crate::dsp::dec_neon::dc8uv_no_top_left,
];

// upstream: src/dsp/dec.c VP8PredChroma8
/// Predicts the 8x8 chroma block at `dst` with mode `mode` (C:
/// `VP8PredChroma8[mode](dst)`).
///
/// # Panics
///
/// Panics for a mode outside `[0, NUM_B_DC_MODES)`, where C indexes outside its
/// table, and when the block or its top and left samples lie outside `buf`.
pub fn vp8_pred_chroma8(mode: i32, buf: &mut [u8], dst: usize) {
    #[cfg(all(feature = "simd", target_arch = "aarch64"))]
    {
        return VP8_PRED_CHROMA8_NEON[usize::try_from(mode).expect("a non-negative mode")](
            buf, dst,
        );
    }
    #[allow(
        unreachable_code,
        reason = "the C path is reached only without a SIMD variant"
    )]
    VP8_PRED_CHROMA8[usize::try_from(mode).expect("a non-negative mode")](buf, dst);
}

//------------------------------------------------------------------------------
// Edge filtering functions

/// The `uint8_t` that C stores from the clipping table.
fn sample(p: &[u8], pos: usize, delta: i32) -> i32 {
    i32::from(p[at(pos, delta)])
}

// upstream: src/dsp/dec.c DoFilter2_C
/// 4 pixels in, 2 pixels out.
fn do_filter2_c(p: &mut [u8], pos: usize, step: i32) {
    let p1 = sample(p, pos, -2 * step);
    let p0 = sample(p, pos, -step);
    let q0 = sample(p, pos, 0);
    let q1 = sample(p, pos, step);
    let a = 3 * (q0 - p0) + i32::from(VP8KSCLIP1[p1 - q1]); // in [-893,892]
    let a1 = i32::from(VP8KSCLIP2[(a + 4) >> 3]); // in [-16,15]
    let a2 = i32::from(VP8KSCLIP2[(a + 3) >> 3]);
    p[at(pos, -step)] = VP8KCLIP1[p0 + a2];
    p[pos] = VP8KCLIP1[q0 - a1];
}

// upstream: src/dsp/dec.c DoFilter4_C
/// 4 pixels in, 4 pixels out.
fn do_filter4_c(p: &mut [u8], pos: usize, step: i32) {
    let p1 = sample(p, pos, -2 * step);
    let p0 = sample(p, pos, -step);
    let q0 = sample(p, pos, 0);
    let q1 = sample(p, pos, step);
    let a = 3 * (q0 - p0);
    let a1 = i32::from(VP8KSCLIP2[(a + 4) >> 3]);
    let a2 = i32::from(VP8KSCLIP2[(a + 3) >> 3]);
    let a3 = (a1 + 1) >> 1;
    p[at(pos, -2 * step)] = VP8KCLIP1[p1 + a3];
    p[at(pos, -step)] = VP8KCLIP1[p0 + a2];
    p[pos] = VP8KCLIP1[q0 - a1];
    p[at(pos, step)] = VP8KCLIP1[q1 - a3];
}

// upstream: src/dsp/dec.c DoFilter6_C
/// 6 pixels in, 6 pixels out.
fn do_filter6_c(p: &mut [u8], pos: usize, step: i32) {
    let p2 = sample(p, pos, -3 * step);
    let p1 = sample(p, pos, -2 * step);
    let p0 = sample(p, pos, -step);
    let q0 = sample(p, pos, 0);
    let q1 = sample(p, pos, step);
    let q2 = sample(p, pos, 2 * step);
    let a = i32::from(VP8KSCLIP1[3 * (q0 - p0) + i32::from(VP8KSCLIP1[p1 - q1])]);
    // a is in [-128,127], a1 in [-27,27], a2 in [-18,18] and a3 in [-9,9]
    let a1 = (27 * a + 63) >> 7; // eq. to ((3 * a + 7) * 9) >> 7
    let a2 = (18 * a + 63) >> 7; // eq. to ((2 * a + 7) * 9) >> 7
    let a3 = (9 * a + 63) >> 7; // eq. to ((1 * a + 7) * 9) >> 7
    p[at(pos, -3 * step)] = VP8KCLIP1[p2 + a3];
    p[at(pos, -2 * step)] = VP8KCLIP1[p1 + a2];
    p[at(pos, -step)] = VP8KCLIP1[p0 + a1];
    p[pos] = VP8KCLIP1[q0 - a1];
    p[at(pos, step)] = VP8KCLIP1[q1 - a2];
    p[at(pos, 2 * step)] = VP8KCLIP1[q2 - a3];
}

// upstream: src/dsp/dec.c Hev
/// High edge variance.
fn hev(p: &[u8], pos: usize, step: i32, thresh: i32) -> bool {
    let p1 = sample(p, pos, -2 * step);
    let p0 = sample(p, pos, -step);
    let q0 = sample(p, pos, 0);
    let q1 = sample(p, pos, step);
    i32::from(VP8KABS0[p1 - p0]) > thresh || i32::from(VP8KABS0[q1 - q0]) > thresh
}

// upstream: src/dsp/dec.c NeedsFilter_C
fn needs_filter_c(p: &[u8], pos: usize, step: i32, t: i32) -> bool {
    let p1 = sample(p, pos, -2 * step);
    let p0 = sample(p, pos, -step);
    let q0 = sample(p, pos, 0);
    let q1 = sample(p, pos, step);
    (4 * i32::from(VP8KABS0[p0 - q0]) + i32::from(VP8KABS0[p1 - q1])) <= t
}

// upstream: src/dsp/dec.c NeedsFilter2_C
fn needs_filter2_c(p: &[u8], pos: usize, step: i32, t: i32, it: i32) -> bool {
    let p3 = sample(p, pos, -4 * step);
    let p2 = sample(p, pos, -3 * step);
    let p1 = sample(p, pos, -2 * step);
    let p0 = sample(p, pos, -step);
    let q0 = sample(p, pos, 0);
    let q1 = sample(p, pos, step);
    let q2 = sample(p, pos, 2 * step);
    let q3 = sample(p, pos, 3 * step);
    if (4 * i32::from(VP8KABS0[p0 - q0]) + i32::from(VP8KABS0[p1 - q1])) > t {
        return false;
    }
    i32::from(VP8KABS0[p3 - p2]) <= it
        && i32::from(VP8KABS0[p2 - p1]) <= it
        && i32::from(VP8KABS0[p1 - p0]) <= it
        && i32::from(VP8KABS0[q3 - q2]) <= it
        && i32::from(VP8KABS0[q2 - q1]) <= it
        && i32::from(VP8KABS0[q1 - q0]) <= it
}

//------------------------------------------------------------------------------
// Simple In-loop filtering (Paragraph 15.2)

// upstream: src/dsp/dec.c SimpleVFilter16_C
pub fn simple_v_filter16_c(p: &mut [u8], pos: usize, stride: i32, thresh: i32) {
    let thresh2 = 2 * thresh + 1;
    for i in 0..16 {
        if needs_filter_c(p, pos + i, stride, thresh2) {
            do_filter2_c(p, pos + i, stride);
        }
    }
}

// upstream: src/dsp/dec.c SimpleHFilter16_C
pub fn simple_h_filter16_c(p: &mut [u8], pos: usize, stride: i32, thresh: i32) {
    let thresh2 = 2 * thresh + 1;
    for i in 0..16 {
        let row = at(pos, i * stride);
        if needs_filter_c(p, row, 1, thresh2) {
            do_filter2_c(p, row, 1);
        }
    }
}

// upstream: src/dsp/dec.c SimpleVFilter16i_C
pub fn simple_v_filter16i_c(p: &mut [u8], pos: usize, stride: i32, thresh: i32) {
    let mut pos = pos;
    for _ in 0..3 {
        pos = at(pos, 4 * stride);
        simple_v_filter16_c(p, pos, stride, thresh);
    }
}

// upstream: src/dsp/dec.c SimpleHFilter16i_C
pub fn simple_h_filter16i_c(p: &mut [u8], pos: usize, stride: i32, thresh: i32) {
    let mut pos = pos;
    for _ in 0..3 {
        pos += 4;
        simple_h_filter16_c(p, pos, stride, thresh);
    }
}

//------------------------------------------------------------------------------
// Complex In-loop filtering (Paragraph 15.3)

// upstream: src/dsp/dec.c FilterLoop26_C
#[expect(clippy::too_many_arguments, reason = "the C parameters")]
fn filter_loop26_c(
    p: &mut [u8],
    pos: usize,
    hstride: i32,
    vstride: i32,
    size: i32,
    thresh: i32,
    ithresh: i32,
    hev_thresh: i32,
) {
    let thresh2 = 2 * thresh + 1;
    let mut pos = pos;
    let mut size = size;
    while size > 0 {
        size -= 1;
        if needs_filter2_c(p, pos, hstride, thresh2, ithresh) {
            if hev(p, pos, hstride, hev_thresh) {
                do_filter2_c(p, pos, hstride);
            } else {
                do_filter6_c(p, pos, hstride);
            }
        }
        pos = at(pos, vstride);
    }
}

// upstream: src/dsp/dec.c FilterLoop24_C
#[expect(clippy::too_many_arguments, reason = "the C parameters")]
fn filter_loop24_c(
    p: &mut [u8],
    pos: usize,
    hstride: i32,
    vstride: i32,
    size: i32,
    thresh: i32,
    ithresh: i32,
    hev_thresh: i32,
) {
    let thresh2 = 2 * thresh + 1;
    let mut pos = pos;
    let mut size = size;
    while size > 0 {
        size -= 1;
        if needs_filter2_c(p, pos, hstride, thresh2, ithresh) {
            if hev(p, pos, hstride, hev_thresh) {
                do_filter2_c(p, pos, hstride);
            } else {
                do_filter4_c(p, pos, hstride);
            }
        }
        pos = at(pos, vstride);
    }
}

// upstream: src/dsp/dec.c VFilter16_C
/// On macroblock edges.
pub fn v_filter16_c(
    p: &mut [u8],
    pos: usize,
    stride: i32,
    thresh: i32,
    ithresh: i32,
    hev_thresh: i32,
) {
    filter_loop26_c(p, pos, stride, 1, 16, thresh, ithresh, hev_thresh);
}

// upstream: src/dsp/dec.c HFilter16_C
pub fn h_filter16_c(
    p: &mut [u8],
    pos: usize,
    stride: i32,
    thresh: i32,
    ithresh: i32,
    hev_thresh: i32,
) {
    filter_loop26_c(p, pos, 1, stride, 16, thresh, ithresh, hev_thresh);
}

// upstream: src/dsp/dec.c VFilter16i_C
/// On three inner edges.
pub fn v_filter16i_c(
    p: &mut [u8],
    pos: usize,
    stride: i32,
    thresh: i32,
    ithresh: i32,
    hev_thresh: i32,
) {
    let mut pos = pos;
    for _ in 0..3 {
        pos = at(pos, 4 * stride);
        filter_loop24_c(p, pos, stride, 1, 16, thresh, ithresh, hev_thresh);
    }
}

// upstream: src/dsp/dec.c HFilter16i_C
pub fn h_filter16i_c(
    p: &mut [u8],
    pos: usize,
    stride: i32,
    thresh: i32,
    ithresh: i32,
    hev_thresh: i32,
) {
    let mut pos = pos;
    for _ in 0..3 {
        pos += 4;
        filter_loop24_c(p, pos, 1, stride, 16, thresh, ithresh, hev_thresh);
    }
}

// upstream: src/dsp/dec.c VFilter8_C
/// 8-pixels wide variant, for chroma filtering.
#[expect(
    clippy::too_many_arguments,
    reason = "the C parameters with two positions"
)]
pub fn v_filter8_c(
    u: &mut [u8],
    u_pos: usize,
    v: &mut [u8],
    v_pos: usize,
    stride: i32,
    thresh: i32,
    ithresh: i32,
    hev_thresh: i32,
) {
    filter_loop26_c(u, u_pos, stride, 1, 8, thresh, ithresh, hev_thresh);
    filter_loop26_c(v, v_pos, stride, 1, 8, thresh, ithresh, hev_thresh);
}

// upstream: src/dsp/dec.c HFilter8_C
#[expect(
    clippy::too_many_arguments,
    reason = "the C parameters with two positions"
)]
pub fn h_filter8_c(
    u: &mut [u8],
    u_pos: usize,
    v: &mut [u8],
    v_pos: usize,
    stride: i32,
    thresh: i32,
    ithresh: i32,
    hev_thresh: i32,
) {
    filter_loop26_c(u, u_pos, 1, stride, 8, thresh, ithresh, hev_thresh);
    filter_loop26_c(v, v_pos, 1, stride, 8, thresh, ithresh, hev_thresh);
}

// upstream: src/dsp/dec.c VFilter8i_C
#[expect(
    clippy::too_many_arguments,
    reason = "the C parameters with two positions"
)]
pub fn v_filter8i_c(
    u: &mut [u8],
    u_pos: usize,
    v: &mut [u8],
    v_pos: usize,
    stride: i32,
    thresh: i32,
    ithresh: i32,
    hev_thresh: i32,
) {
    filter_loop24_c(
        u,
        at(u_pos, 4 * stride),
        stride,
        1,
        8,
        thresh,
        ithresh,
        hev_thresh,
    );
    filter_loop24_c(
        v,
        at(v_pos, 4 * stride),
        stride,
        1,
        8,
        thresh,
        ithresh,
        hev_thresh,
    );
}

// upstream: src/dsp/dec.c HFilter8i_C
#[expect(
    clippy::too_many_arguments,
    reason = "the C parameters with two positions"
)]
pub fn h_filter8i_c(
    u: &mut [u8],
    u_pos: usize,
    v: &mut [u8],
    v_pos: usize,
    stride: i32,
    thresh: i32,
    ithresh: i32,
    hev_thresh: i32,
) {
    filter_loop24_c(u, u_pos + 4, 1, stride, 8, thresh, ithresh, hev_thresh);
    filter_loop24_c(v, v_pos + 4, 1, stride, 8, thresh, ithresh, hev_thresh);
}

//------------------------------------------------------------------------------

// upstream: src/dsp/dec.c DitherCombine8x8_C
/// Adds the 8x8 block of dithering values to `dst`.
fn dither_combine8x8_c(dither: &[u8], dst: &mut [u8], dst_stride: i32) {
    let stride = usize::try_from(dst_stride).expect("a non-negative stride");
    for j in 0..8 {
        let row = &mut dst[j * stride..];
        for i in 0..8 {
            let delta0 = i32::from(dither[j * 8 + i]) - VP8_DITHER_AMP_CENTER;
            let delta1 = (delta0 + VP8_DITHER_DESCALE_ROUNDER) >> VP8_DITHER_DESCALE;
            row[i] = clip_8b(i32::from(row[i]) + delta1);
        }
    }
}

//------------------------------------------------------------------------------
// Dispatch (C: the pointers that `VP8DspInit` fills)

// upstream: src/dsp/dsp.h VP8SimpleFilterFunc
/// A simple loop filter: the buffer, the position of `p`, the stride and the
/// threshold (C: `VP8SimpleFilterFunc`).
pub type VP8SimpleFilterFunc = fn(&mut [u8], usize, i32, i32);

// upstream: src/dsp/dsp.h VP8LumaFilterFunc
/// A complex luma loop filter: the buffer, the position of `luma`, the stride,
/// `thresh`, `ithresh` and `hev_t` (C: `VP8LumaFilterFunc`).
pub type VP8LumaFilterFunc = fn(&mut [u8], usize, i32, i32, i32, i32);

// upstream: src/dsp/dsp.h VP8ChromaFilterFunc
/// A complex chroma loop filter over the U and V planes, each a buffer and a
/// position (C: `VP8ChromaFilterFunc`).
pub type VP8ChromaFilterFunc = fn(&mut [u8], usize, &mut [u8], usize, i32, i32, i32, i32);

// upstream: src/dsp/dec.c VP8VFilter16
/// Filters the horizontal macroblock edge above `pos` (C: `VFilter16_C`).
/// With `simd` on 64-bit ARM, thresholds above 254 follow upstream's NEON
/// filter, not the C function (`src/dsp/dec_neon.rs`, checks not applicable 5).
pub fn vp8v_filter16(p: &mut [u8], pos: usize, stride: i32, thresh: i32, ithresh: i32, hev_t: i32) {
    #[cfg(all(feature = "simd", target_arch = "aarch64"))]
    {
        return crate::dsp::dec_neon::v_filter16(p, pos, stride, thresh, ithresh, hev_t);
    }
    #[allow(
        unreachable_code,
        reason = "the C path is reached only without a SIMD variant"
    )]
    v_filter16_c(p, pos, stride, thresh, ithresh, hev_t);
}

// upstream: src/dsp/dec.c VP8HFilter16
/// Filters the vertical macroblock edge left of `pos` (C: `HFilter16_C`).
/// With `simd` on 64-bit ARM, thresholds above 254 follow upstream's NEON
/// filter, not the C function (`src/dsp/dec_neon.rs`, checks not applicable 5).
pub fn vp8h_filter16(p: &mut [u8], pos: usize, stride: i32, thresh: i32, ithresh: i32, hev_t: i32) {
    #[cfg(all(feature = "simd", target_arch = "aarch64"))]
    {
        return crate::dsp::dec_neon::h_filter16(p, pos, stride, thresh, ithresh, hev_t);
    }
    #[allow(
        unreachable_code,
        reason = "the C path is reached only without a SIMD variant"
    )]
    h_filter16_c(p, pos, stride, thresh, ithresh, hev_t);
}

// upstream: src/dsp/dec.c VP8VFilter8
/// Chroma variant of [`vp8v_filter16`] (C: `VFilter8_C`).
/// With `simd` on 64-bit ARM, thresholds above 254 follow upstream's NEON
/// filter, not the C function (`src/dsp/dec_neon.rs`, checks not applicable 5).
#[expect(
    clippy::too_many_arguments,
    reason = "the C parameters with two positions"
)]
pub fn vp8v_filter8(
    u: &mut [u8],
    u_pos: usize,
    v: &mut [u8],
    v_pos: usize,
    stride: i32,
    thresh: i32,
    ithresh: i32,
    hev_t: i32,
) {
    #[cfg(all(feature = "simd", target_arch = "aarch64"))]
    {
        return crate::dsp::dec_neon::v_filter8(u, u_pos, v, v_pos, stride, thresh, ithresh, hev_t);
    }
    #[allow(
        unreachable_code,
        reason = "the C path is reached only without a SIMD variant"
    )]
    v_filter8_c(u, u_pos, v, v_pos, stride, thresh, ithresh, hev_t);
}

// upstream: src/dsp/dec.c VP8HFilter8
/// Chroma variant of [`vp8h_filter16`] (C: `HFilter8_C`).
/// With `simd` on 64-bit ARM, thresholds above 254 follow upstream's NEON
/// filter, not the C function (`src/dsp/dec_neon.rs`, checks not applicable 5).
#[expect(
    clippy::too_many_arguments,
    reason = "the C parameters with two positions"
)]
pub fn vp8h_filter8(
    u: &mut [u8],
    u_pos: usize,
    v: &mut [u8],
    v_pos: usize,
    stride: i32,
    thresh: i32,
    ithresh: i32,
    hev_t: i32,
) {
    #[cfg(all(feature = "simd", target_arch = "aarch64"))]
    {
        return crate::dsp::dec_neon::h_filter8(u, u_pos, v, v_pos, stride, thresh, ithresh, hev_t);
    }
    #[allow(
        unreachable_code,
        reason = "the C path is reached only without a SIMD variant"
    )]
    h_filter8_c(u, u_pos, v, v_pos, stride, thresh, ithresh, hev_t);
}

// upstream: src/dsp/dec.c VP8VFilter16i
/// Filters the three inner horizontal edges (C: `VFilter16i_C`).
/// With `simd` on 64-bit ARM, thresholds above 254 follow upstream's NEON
/// filter, not the C function (`src/dsp/dec_neon.rs`, checks not applicable 5).
pub fn vp8v_filter16i(
    p: &mut [u8],
    pos: usize,
    stride: i32,
    thresh: i32,
    ithresh: i32,
    hev_t: i32,
) {
    #[cfg(all(feature = "simd", target_arch = "aarch64"))]
    {
        return crate::dsp::dec_neon::v_filter16i(p, pos, stride, thresh, ithresh, hev_t);
    }
    #[allow(
        unreachable_code,
        reason = "the C path is reached only without a SIMD variant"
    )]
    v_filter16i_c(p, pos, stride, thresh, ithresh, hev_t);
}

// upstream: src/dsp/dec.c VP8HFilter16i
/// Filters the three inner vertical edges (C: `HFilter16i_C`).
/// With `simd` on 64-bit ARM, thresholds above 254 follow upstream's NEON
/// filter, not the C function (`src/dsp/dec_neon.rs`, checks not applicable 5).
pub fn vp8h_filter16i(
    p: &mut [u8],
    pos: usize,
    stride: i32,
    thresh: i32,
    ithresh: i32,
    hev_t: i32,
) {
    #[cfg(all(feature = "simd", target_arch = "aarch64"))]
    {
        return crate::dsp::dec_neon::h_filter16i(p, pos, stride, thresh, ithresh, hev_t);
    }
    #[allow(
        unreachable_code,
        reason = "the C path is reached only without a SIMD variant"
    )]
    h_filter16i_c(p, pos, stride, thresh, ithresh, hev_t);
}

// upstream: src/dsp/dec.c VP8VFilter8i
/// Chroma variant of [`vp8v_filter16i`] (C: `VFilter8i_C`).
/// With `simd` on 64-bit ARM, thresholds above 254 follow upstream's NEON
/// filter, not the C function (`src/dsp/dec_neon.rs`, checks not applicable 5).
#[expect(
    clippy::too_many_arguments,
    reason = "the C parameters with two positions"
)]
pub fn vp8v_filter8i(
    u: &mut [u8],
    u_pos: usize,
    v: &mut [u8],
    v_pos: usize,
    stride: i32,
    thresh: i32,
    ithresh: i32,
    hev_t: i32,
) {
    #[cfg(all(feature = "simd", target_arch = "aarch64"))]
    {
        return crate::dsp::dec_neon::v_filter8i(
            u, u_pos, v, v_pos, stride, thresh, ithresh, hev_t,
        );
    }
    #[allow(
        unreachable_code,
        reason = "the C path is reached only without a SIMD variant"
    )]
    v_filter8i_c(u, u_pos, v, v_pos, stride, thresh, ithresh, hev_t);
}

// upstream: src/dsp/dec.c VP8HFilter8i
/// Chroma variant of [`vp8h_filter16i`] (C: `HFilter8i_C`).
/// With `simd` on 64-bit ARM, thresholds above 254 follow upstream's NEON
/// filter, not the C function (`src/dsp/dec_neon.rs`, checks not applicable 5).
#[expect(
    clippy::too_many_arguments,
    reason = "the C parameters with two positions"
)]
pub fn vp8h_filter8i(
    u: &mut [u8],
    u_pos: usize,
    v: &mut [u8],
    v_pos: usize,
    stride: i32,
    thresh: i32,
    ithresh: i32,
    hev_t: i32,
) {
    #[cfg(all(feature = "simd", target_arch = "aarch64"))]
    {
        return crate::dsp::dec_neon::h_filter8i(
            u, u_pos, v, v_pos, stride, thresh, ithresh, hev_t,
        );
    }
    #[allow(
        unreachable_code,
        reason = "the C path is reached only without a SIMD variant"
    )]
    h_filter8i_c(u, u_pos, v, v_pos, stride, thresh, ithresh, hev_t);
}

// upstream: src/dsp/dec.c VP8SimpleVFilter16
/// Simple filter of the horizontal macroblock edge (C: `SimpleVFilter16_C`).
/// With `simd` on 64-bit ARM, thresholds above 254 follow upstream's NEON
/// filter, not the C function (`src/dsp/dec_neon.rs`, checks not applicable 5).
pub fn vp8_simple_v_filter16(p: &mut [u8], pos: usize, stride: i32, thresh: i32) {
    #[cfg(all(feature = "simd", target_arch = "aarch64"))]
    {
        return crate::dsp::dec_neon::simple_v_filter16(p, pos, stride, thresh);
    }
    #[allow(
        unreachable_code,
        reason = "the C path is reached only without a SIMD variant"
    )]
    simple_v_filter16_c(p, pos, stride, thresh);
}

// upstream: src/dsp/dec.c VP8SimpleHFilter16
/// Simple filter of the vertical macroblock edge (C: `SimpleHFilter16_C`).
/// With `simd` on 64-bit ARM, thresholds above 254 follow upstream's NEON
/// filter, not the C function (`src/dsp/dec_neon.rs`, checks not applicable 5).
pub fn vp8_simple_h_filter16(p: &mut [u8], pos: usize, stride: i32, thresh: i32) {
    #[cfg(all(feature = "simd", target_arch = "aarch64"))]
    {
        return crate::dsp::dec_neon::simple_h_filter16(p, pos, stride, thresh);
    }
    #[allow(
        unreachable_code,
        reason = "the C path is reached only without a SIMD variant"
    )]
    simple_h_filter16_c(p, pos, stride, thresh);
}

// upstream: src/dsp/dec.c VP8SimpleVFilter16i
/// Simple filter of the three inner horizontal edges (C: `SimpleVFilter16i_C`).
/// With `simd` on 64-bit ARM, thresholds above 254 follow upstream's NEON
/// filter, not the C function (`src/dsp/dec_neon.rs`, checks not applicable 5).
pub fn vp8_simple_v_filter16i(p: &mut [u8], pos: usize, stride: i32, thresh: i32) {
    #[cfg(all(feature = "simd", target_arch = "aarch64"))]
    {
        return crate::dsp::dec_neon::simple_v_filter16i(p, pos, stride, thresh);
    }
    #[allow(
        unreachable_code,
        reason = "the C path is reached only without a SIMD variant"
    )]
    simple_v_filter16i_c(p, pos, stride, thresh);
}

// upstream: src/dsp/dec.c VP8SimpleHFilter16i
/// Simple filter of the three inner vertical edges (C: `SimpleHFilter16i_C`).
/// With `simd` on 64-bit ARM, thresholds above 254 follow upstream's NEON
/// filter, not the C function (`src/dsp/dec_neon.rs`, checks not applicable 5).
pub fn vp8_simple_h_filter16i(p: &mut [u8], pos: usize, stride: i32, thresh: i32) {
    #[cfg(all(feature = "simd", target_arch = "aarch64"))]
    {
        return crate::dsp::dec_neon::simple_h_filter16i(p, pos, stride, thresh);
    }
    #[allow(
        unreachable_code,
        reason = "the C path is reached only without a SIMD variant"
    )]
    simple_h_filter16i_c(p, pos, stride, thresh);
}

// upstream: src/dsp/dec.c VP8DitherCombine8x8
/// Adds an 8x8 block of dithering values to `dst` (C: `DitherCombine8x8_C`).
pub fn vp8_dither_combine8x8(dither: &[u8], dst: &mut [u8], dst_stride: i32) {
    dither_combine8x8_c(dither, dst, dst_stride);
}

// upstream: src/dsp/dec.c VP8DspInit (no counterpart: it only assigns portable pointers, ADR 0003)
