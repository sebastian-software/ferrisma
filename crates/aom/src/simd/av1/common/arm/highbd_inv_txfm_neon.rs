/*
 * Copyright (c) 2020, Alliance for Open Media. All rights reserved.
 *
 * This source code is subject to the terms of the BSD 2 Clause License and
 * the Alliance for Open Media Patent License 1.0. If the BSD 2 Clause License
 * was not distributed with this source code in the LICENSE file, you canzip
 * obtain it at www.aomedia.org/license/software. If the Alliance for Open
 * Media Patent License 1.0 was not distributed with this source code in the
 * PATENTS file, you can obtain it at www.aomedia.org/license/patent.
 */

// Translated by hand from upstream/aom/av1/common/arm/highbd_inv_txfm_neon.c
// (ADR 0056). Intrinsic sequences, helper structure, and names follow the
// original; only the AOM_ARCH_AARCH64 branches are retained. C locals that
// the original leaves uninitialized start zeroed here.
#![allow(non_snake_case, non_upper_case_globals, unused_assignments, unused_mut, unused_parens)]
#![allow(unused_variables, dead_code, clippy::all)]

use core::arch::aarch64::*;

use crate::av1__common__av1_inv_txfm2d::av1_inv_txfm_shift_ls;
use crate::av1__common__av1_txfm::{av1_cospi_arr_data, av1_sinpi_arr_data};
use crate::av1__common__idct::{av1_highbd_iwht4x4_add, TxfmParam};
use crate::simd::types::tran_low_t;

// ---------------------------------------------------------------------------
// Header items used by the original translation unit. Header-local (`static`)
// tables, inline functions, and macros are transcribed here; exported tables
// and functions come from the translated modules imported above.

// aom_dsp/txfm_common.h
type TX_TYPE = u8;
type TX_SIZE = u8;

const TX_4X4: i32 = 0;
const TX_8X8: i32 = 1;
const TX_16X16: i32 = 2;
const TX_32X32: i32 = 3;
const TX_64X64: i32 = 4;
const TX_4X8: i32 = 5;
const TX_8X4: i32 = 6;
const TX_8X16: i32 = 7;
const TX_16X8: i32 = 8;
const TX_16X32: i32 = 9;
const TX_32X16: i32 = 10;
const TX_32X64: i32 = 11;
const TX_64X32: i32 = 12;
const TX_4X16: i32 = 13;
const TX_16X4: i32 = 14;
const TX_8X32: i32 = 15;
const TX_32X8: i32 = 16;
const TX_16X64: i32 = 17;
const TX_64X16: i32 = 18;
const TX_SIZES_ALL: i32 = 19;
const TX_SIZES: i32 = TX_4X8;

const DCT_DCT: i32 = 0;
const ADST_DCT: i32 = 1;
const DCT_ADST: i32 = 2;
const ADST_ADST: i32 = 3;
const FLIPADST_DCT: i32 = 4;
const DCT_FLIPADST: i32 = 5;
const FLIPADST_FLIPADST: i32 = 6;
const ADST_FLIPADST: i32 = 7;
const FLIPADST_ADST: i32 = 8;
const IDTX: i32 = 9;
const V_DCT: i32 = 10;
const H_DCT: i32 = 11;
const V_ADST: i32 = 12;
const H_ADST: i32 = 13;
const V_FLIPADST: i32 = 14;
const H_FLIPADST: i32 = 15;
const TX_TYPES: i32 = 16;

// av1/common/av1_inv_txfm1d_cfg.h
const INV_COS_BIT: i32 = 12;

// av1/common/av1_txfm.h
const cos_bit_min: i32 = 10;
const NewSqrt2Bits: i32 = 12;
// 2^12 * sqrt(2)
const NewSqrt2: i32 = 5793;
// 2^12 / sqrt(2)
const NewInvSqrt2: i32 = 2896;

#[inline(always)]
unsafe fn cospi_arr(n: i32) -> *const i32 {
    (&raw const av1_cospi_arr_data[(n - cos_bit_min) as usize]).cast::<i32>()
}

#[inline(always)]
unsafe fn sinpi_arr(n: i32) -> *const i32 {
    (&raw const av1_sinpi_arr_data[(n - cos_bit_min) as usize]).cast::<i32>()
}

#[inline(always)]
fn get_log_range_out(bd: i32) -> i32 {
    let log_range_out = AOMMAX(16, bd + 6);
    // bd is limited by the bitstream to 12. This assert is to satisfy static
    // analyzers that may assume `log_range_out - 1` is greater than 31.
    debug_assert!(log_range_out <= 18);
    log_range_out
}

#[inline(always)]
fn get_log_range(bd: i32, do_cols: i32) -> i32 {
    let log_range = AOMMAX(16, bd + if do_cols != 0 { 6 } else { 8 });
    // bd is limited by the bitstream to 12. This assert is to satisfy static
    // analyzers that may assume `log_range - 1` is greater than 31.
    debug_assert!(log_range <= 20);
    log_range
}

#[inline(always)]
fn get_flip_cfg(tx_type: TX_TYPE, ud_flip: *mut i32, lr_flip: *mut i32) {
    let (ud, lr) = match tx_type as i32 {
        DCT_DCT | ADST_DCT | DCT_ADST | ADST_ADST => (0, 0),
        IDTX | V_DCT | H_DCT | V_ADST | H_ADST => (0, 0),
        FLIPADST_DCT | FLIPADST_ADST | V_FLIPADST => (1, 0),
        DCT_FLIPADST | ADST_FLIPADST | H_FLIPADST => (0, 1),
        FLIPADST_FLIPADST => (1, 1),
        _ => {
            debug_assert!(false);
            (0, 0)
        }
    };
    // SAFETY: the callers pass pointers to their own locals.
    unsafe {
        *ud_flip = ud;
        *lr_flip = lr;
    }
}

// Utility function that returns the log of the ratio of the col and row
// sizes.
#[inline(always)]
fn get_rect_tx_log_ratio(col: i32, row: i32) -> i32 {
    if col == row {
        return 0;
    }
    if col > row {
        if col == row * 2 {
            return 1;
        }
        if col == row * 4 {
            return 2;
        }
        debug_assert!(false, "Unsupported transform size");
    } else {
        if row == col * 2 {
            return -1;
        }
        if row == col * 4 {
            return -2;
        }
        debug_assert!(false, "Unsupported transform size");
    }
    0 // Invalid
}

#[inline(always)]
fn get_txw_idx(tx_size: TX_SIZE) -> i32 {
    tx_size_wide_log2[tx_size as usize] - tx_size_wide_log2[0]
}

#[inline(always)]
fn get_txh_idx(tx_size: TX_SIZE) -> i32 {
    tx_size_high_log2[tx_size as usize] - tx_size_high_log2[0]
}

// av1/common/common_data.h
// Transform block width in pixels
static tx_size_wide: [i32; 19] =
    [4, 8, 16, 32, 64, 4, 8, 8, 16, 16, 32, 32, 64, 4, 16, 8, 32, 16, 64];
// Transform block height in pixels
static tx_size_high: [i32; 19] =
    [4, 8, 16, 32, 64, 8, 4, 16, 8, 32, 16, 64, 32, 16, 4, 32, 8, 64, 16];
// Transform block width in log2
static tx_size_wide_log2: [i32; 19] = [2, 3, 4, 5, 6, 2, 3, 3, 4, 4, 5, 5, 6, 2, 4, 3, 5, 4, 6];
// Transform block height in log2
static tx_size_high_log2: [i32; 19] = [2, 3, 4, 5, 6, 3, 2, 4, 3, 5, 4, 6, 5, 4, 2, 5, 3, 6, 4];

// av1/common/blockd.h
static av1_ext_tx_used: [[i32; 16]; 6] = [
    [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0],
    [1, 1, 1, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0],
    [1, 1, 1, 1, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0, 0],
    [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0],
    [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
];

// av1/common/idct.h
#[inline(always)]
fn cast_to_int32(input: *const tran_low_t) -> *const i32 {
    input as *const i32
}

// aom_dsp/aom_dsp_common.h
#[inline(always)]
fn AOMMIN(x: i32, y: i32) -> i32 {
    if x < y {
        x
    } else {
        y
    }
}

#[inline(always)]
fn AOMMAX(x: i32, y: i32) -> i32 {
    if x > y {
        x
    } else {
        y
    }
}

// aom_ports/mem.h
#[inline(always)]
fn CONVERT_TO_SHORTPTR(x: *mut u8) -> *mut u16 {
    ((x as usize) << 1) as *mut u16
}

// ---------------------------------------------------------------------------

macro_rules! TRANSPOSE_4X4 {
    ($x0:expr, $x1:expr, $x2:expr, $x3:expr, $y0:expr, $y1:expr, $y2:expr, $y3:expr) => {{
        let swap_low: int32x4x2_t = vtrnq_s32($x0, $x1);
        let swap_high: int32x4x2_t = vtrnq_s32($x2, $x3);
        $y0 = vreinterpretq_s32_s64(vzip1q_s64(
            vreinterpretq_s64_s32(swap_low.0),
            vreinterpretq_s64_s32(swap_high.0),
        ));
        $y1 = vreinterpretq_s32_s64(vzip1q_s64(
            vreinterpretq_s64_s32(swap_low.1),
            vreinterpretq_s64_s32(swap_high.1),
        ));
        $y2 = vreinterpretq_s32_s64(vzip2q_s64(
            vreinterpretq_s64_s32(swap_low.0),
            vreinterpretq_s64_s32(swap_high.0),
        ));
        $y3 = vreinterpretq_s32_s64(vzip2q_s64(
            vreinterpretq_s64_s32(swap_low.1),
            vreinterpretq_s64_s32(swap_high.1),
        ));
    }};
}

#[inline(always)]
unsafe fn transpose_4x4(mut in_: *const int32x4_t, mut out: *mut int32x4_t) {
    TRANSPOSE_4X4!(
        *in_,
        *in_.add(1),
        *in_.add(2),
        *in_.add(3),
        *out,
        *out.add(1),
        *out.add(2),
        *out.add(3)
    );
}

#[inline(always)]
unsafe fn transpose_8x8(mut in_: *const int32x4_t, mut out: *mut int32x4_t) {
    TRANSPOSE_4X4!(
        *in_,
        *in_.add(2),
        *in_.add(4),
        *in_.add(6),
        *out,
        *out.add(2),
        *out.add(4),
        *out.add(6)
    );
    TRANSPOSE_4X4!(
        *in_.add(1),
        *in_.add(3),
        *in_.add(5),
        *in_.add(7),
        *out.add(8),
        *out.add(10),
        *out.add(12),
        *out.add(14)
    );
    TRANSPOSE_4X4!(
        *in_.add(8),
        *in_.add(10),
        *in_.add(12),
        *in_.add(14),
        *out.add(1),
        *out.add(3),
        *out.add(5),
        *out.add(7)
    );
    TRANSPOSE_4X4!(
        *in_.add(9),
        *in_.add(11),
        *in_.add(13),
        *in_.add(15),
        *out.add(9),
        *out.add(11),
        *out.add(13),
        *out.add(15)
    );
}

#[inline(always)]
unsafe fn round_shift_array_32_neon(
    mut input: *mut int32x4_t,
    mut output: *mut int32x4_t,
    mut size: i32,
    mut bit: i32,
) {
    let v_bit: int32x4_t = vdupq_n_s32(-bit);
    for i in 0..size {
        *output.offset(i as isize) = vrshlq_s32(*input.offset(i as isize), v_bit);
    }
}

#[inline(always)]
unsafe fn round_shift_rect_array_32_neon(
    mut input: *mut int32x4_t,
    mut output: *mut int32x4_t,
    mut size: i32,
) {
    for i in 0..size {
        let r0: int32x4_t = vmulq_n_s32(*input.offset(i as isize), NewInvSqrt2);
        *output.offset(i as isize) = vrshrq_n_s32::<NewSqrt2Bits>(r0);
    }
}

#[inline(always)]
unsafe fn half_btf_neon_r(
    mut n0: *const i32,
    mut w0: *const int32x4_t,
    mut n1: *const i32,
    mut w1: *const int32x4_t,
    mut v_bit: *const int32x4_t,
    mut rnding: *const int32x4_t,
) -> int32x4_t {
    let mut x: int32x4_t = core::mem::zeroed();
    x = vmlaq_n_s32(*rnding, *w0, *n0);
    x = vmlaq_n_s32(x, *w1, *n1);
    x = vshlq_s32(x, *v_bit);
    return x;
}

#[inline(always)]
unsafe fn half_btf_neon_mode11_r(
    mut n0: *const i32,
    mut w0: *const int32x4_t,
    mut n1: *const i32,
    mut w1: *const int32x4_t,
    mut v_bit: *const int32x4_t,
    mut rnding: *const int32x4_t,
) -> int32x4_t {
    let mut x: int32x4_t = core::mem::zeroed();
    x = vmlaq_n_s32(*rnding, *w0, -*n0);
    x = vmlaq_n_s32(x, *w1, -*n1);
    x = vshlq_s32(x, *v_bit);
    return x;
}

#[inline(always)]
unsafe fn half_btf_neon_mode01_r(
    mut n0: *const i32,
    mut w0: *const int32x4_t,
    mut n1: *const i32,
    mut w1: *const int32x4_t,
    mut v_bit: *const int32x4_t,
    mut rnding: *const int32x4_t,
) -> int32x4_t {
    let mut x: int32x4_t = core::mem::zeroed();
    x = vmlaq_n_s32(*rnding, *w0, *n0);
    x = vmlsq_n_s32(x, *w1, *n1);
    x = vshlq_s32(x, *v_bit);
    return x;
}

#[inline(always)]
unsafe fn half_btf_neon_mode10_r(
    mut n0: *const i32,
    mut w0: *const int32x4_t,
    mut n1: *const i32,
    mut w1: *const int32x4_t,
    mut v_bit: *const int32x4_t,
    mut rnding: *const int32x4_t,
) -> int32x4_t {
    let mut x: int32x4_t = core::mem::zeroed();
    x = vmlaq_n_s32(*rnding, *w1, *n1);
    x = vmlsq_n_s32(x, *w0, *n0);
    x = vshlq_s32(x, *v_bit);
    return x;
}

#[inline(always)]
unsafe fn half_btf_0_neon_r(
    mut n0: *const i32,
    mut w0: *const int32x4_t,
    mut v_bit: *const int32x4_t,
    mut rnding: *const int32x4_t,
) -> int32x4_t {
    let mut x: int32x4_t = core::mem::zeroed();
    x = vmlaq_n_s32(*rnding, *w0, *n0);
    x = vshlq_s32(x, *v_bit);
    return x;
}

#[inline(always)]
unsafe fn half_btf_0_m_neon_r(
    mut n0: *const i32,
    mut w0: *const int32x4_t,
    mut v_bit: *const int32x4_t,
    mut rnding: *const int32x4_t,
) -> int32x4_t {
    let mut x: int32x4_t = core::mem::zeroed();
    x = vmlaq_n_s32(*rnding, *w0, -*n0);
    x = vshlq_s32(x, *v_bit);
    return x;
}

#[inline(always)]
unsafe fn flip_buf_neon(mut in_: *mut int32x4_t, mut out: *mut int32x4_t, mut size: i32) {
    for i in 0..size {
        *out.offset((size - i - 1) as isize) = *in_.offset(i as isize);
    }
}

type fwd_transform_1d_neon = Option<unsafe fn(*mut int32x4_t, *mut int32x4_t, i32, i32)>;

type transform_1d_neon = Option<unsafe fn(*mut int32x4_t, *mut int32x4_t, i32, i32, i32, i32)>;

#[inline(always)]
unsafe fn highbd_clamp_u16(
    mut u: *mut uint16x8_t,
    mut min: *const uint16x8_t,
    mut max: *const uint16x8_t,
) -> uint16x8_t {
    let mut clamped: int16x8_t = core::mem::zeroed();
    clamped = vminq_s16(vreinterpretq_s16_u16(*u), vreinterpretq_s16_u16(*max));
    clamped = vmaxq_s16(clamped, vreinterpretq_s16_u16(*min));
    return vreinterpretq_u16_s16(clamped);
}

#[inline(always)]
unsafe fn round_shift_4x4(mut in_: *mut int32x4_t, mut shift: i32) {
    if shift != 0 {
        let v_shift: int32x4_t = vdupq_n_s32(-shift);
        *in_ = vrshlq_s32(*in_, v_shift);
        *in_.add(1) = vrshlq_s32(*in_.add(1), v_shift);
        *in_.add(2) = vrshlq_s32(*in_.add(2), v_shift);
        *in_.add(3) = vrshlq_s32(*in_.add(3), v_shift);
    }
}

unsafe fn round_shift_8x8(mut in_: *mut int32x4_t, mut shift: i32) {
    debug_assert!(shift != 0);
    let v_shift: int32x4_t = vdupq_n_s32(-shift);
    *in_ = vrshlq_s32(*in_, v_shift);
    *in_.add(1) = vrshlq_s32(*in_.add(1), v_shift);
    *in_.add(2) = vrshlq_s32(*in_.add(2), v_shift);
    *in_.add(3) = vrshlq_s32(*in_.add(3), v_shift);
    *in_.add(4) = vrshlq_s32(*in_.add(4), v_shift);
    *in_.add(5) = vrshlq_s32(*in_.add(5), v_shift);
    *in_.add(6) = vrshlq_s32(*in_.add(6), v_shift);
    *in_.add(7) = vrshlq_s32(*in_.add(7), v_shift);
    *in_.add(8) = vrshlq_s32(*in_.add(8), v_shift);
    *in_.add(9) = vrshlq_s32(*in_.add(9), v_shift);
    *in_.add(10) = vrshlq_s32(*in_.add(10), v_shift);
    *in_.add(11) = vrshlq_s32(*in_.add(11), v_shift);
    *in_.add(12) = vrshlq_s32(*in_.add(12), v_shift);
    *in_.add(13) = vrshlq_s32(*in_.add(13), v_shift);
    *in_.add(14) = vrshlq_s32(*in_.add(14), v_shift);
    *in_.add(15) = vrshlq_s32(*in_.add(15), v_shift);
}

unsafe fn highbd_clamp_s32_neon(
    mut in_: *mut int32x4_t,
    mut out: *mut int32x4_t,
    mut clamp_lo: *const int32x4_t,
    mut clamp_hi: *const int32x4_t,
    mut size: i32,
) {
    let mut a0: int32x4_t = core::mem::zeroed();
    let mut a1: int32x4_t = core::mem::zeroed();
    for i in (0..size).step_by(4) {
        a0 = vmaxq_s32(*in_.offset(i as isize), *clamp_lo);
        *out.offset(i as isize) = vminq_s32(a0, *clamp_hi);

        a1 = vmaxq_s32(*in_.offset((i + 1) as isize), *clamp_lo);
        *out.offset((i + 1) as isize) = vminq_s32(a1, *clamp_hi);

        a0 = vmaxq_s32(*in_.offset((i + 2) as isize), *clamp_lo);
        *out.offset((i + 2) as isize) = vminq_s32(a0, *clamp_hi);

        a1 = vmaxq_s32(*in_.offset((i + 3) as isize), *clamp_lo);
        *out.offset((i + 3) as isize) = vminq_s32(a1, *clamp_hi);
    }
}

#[inline(always)]
unsafe fn highbd_get_recon_8x8_neon(
    mut pred: uint16x8_t,
    mut res0: int32x4_t,
    mut res1: int32x4_t,
    mut bd: i32,
) -> uint16x8_t {
    let v_zero: uint16x8_t = vdupq_n_u16(0);
    let mut min_clip_val: int32x4_t = vreinterpretq_s32_u16(v_zero);
    let mut max_clip_val: int32x4_t = vdupq_n_s32((1 << bd) - 1);
    let mut x: uint16x8x2_t = core::mem::zeroed();
    x.0 = vreinterpretq_u16_s32(vaddw_s16(res0, vreinterpret_s16_u16(vget_low_u16(pred))));
    x.1 = vreinterpretq_u16_s32(vaddw_s16(res1, vreinterpret_s16_u16(vget_high_u16(pred))));
    x.0 = vreinterpretq_u16_s32(vmaxq_s32(vreinterpretq_s32_u16(x.0), min_clip_val));
    x.0 = vreinterpretq_u16_s32(vminq_s32(vreinterpretq_s32_u16(x.0), max_clip_val));
    x.1 = vreinterpretq_u16_s32(vmaxq_s32(vreinterpretq_s32_u16(x.1), min_clip_val));
    x.1 = vreinterpretq_u16_s32(vminq_s32(vreinterpretq_s32_u16(x.1), max_clip_val));
    let mut res: uint16x8_t = vcombine_u16(
        vqmovn_u32(vreinterpretq_u32_u16(x.0)),
        vqmovn_u32(vreinterpretq_u32_u16(x.1)),
    );
    return res;
}

#[inline(always)]
unsafe fn highbd_get_recon_4xn_neon(
    mut pred: uint16x4_t,
    mut res0: int32x4_t,
    mut bd: i32,
) -> uint16x4_t {
    let mut x0_: uint16x4_t =
        vreinterpret_u16_s16(vmovn_s32(vaddw_s16(res0, vreinterpret_s16_u16(pred))));
    let mut x0: uint16x8_t = vcombine_u16(x0_, x0_);
    let vmin: uint16x8_t = vdupq_n_u16(0);
    let vmax: uint16x8_t = vdupq_n_u16(((1 << bd) - 1) as u16);
    x0 = highbd_clamp_u16(&raw mut x0, &raw const vmin, &raw const vmax);
    return vget_low_u16(x0);
}

#[inline(always)]
unsafe fn highbd_write_buffer_4xn_neon(
    mut in_: *mut int32x4_t,
    mut output: *mut u16,
    mut stride: i32,
    mut flipud: i32,
    mut height: i32,
    mut bd: i32,
) {
    let mut j: i32 = if flipud != 0 { (height - 1) } else { 0 };
    let step: i32 = if flipud != 0 { -1 } else { 1 };
    {
        let mut i: i32 = 0;
        while i < height {
            let mut v: uint16x4_t = vld1_u16(output.offset((i * stride) as isize));
            let mut u: uint16x4_t = highbd_get_recon_4xn_neon(v, *in_.offset(j as isize), bd);

            vst1_u16(output.offset((i * stride) as isize), u);
            i += 1;
            j += step;
        }
    }
}

#[inline(always)]
unsafe fn highbd_write_buffer_8xn_neon(
    mut in_: *mut int32x4_t,
    mut output: *mut u16,
    mut stride: i32,
    mut flipud: i32,
    mut height: i32,
    mut bd: i32,
) {
    let mut j: i32 = if flipud != 0 { (height - 1) } else { 0 };
    let step: i32 = if flipud != 0 { -1 } else { 1 };
    {
        let mut i: i32 = 0;
        while i < height {
            let mut v: uint16x8_t = vld1q_u16(output.offset((i * stride) as isize));
            let mut u: uint16x8_t = highbd_get_recon_8x8_neon(
                v,
                *in_.offset(j as isize),
                *in_.offset((j + height) as isize),
                bd,
            );

            vst1q_u16(output.offset((i * stride) as isize), u);
            i += 1;
            j += step;
        }
    }
}

#[inline(always)]
unsafe fn load_buffer_32bit_input(
    mut in_: *const i32,
    mut stride: i32,
    mut out: *mut int32x4_t,
    mut out_size: i32,
) {
    for i in 0..out_size {
        *out.offset(i as isize) = vld1q_s32(in_.offset((i * stride) as isize));
    }
}

#[inline(always)]
unsafe fn load_buffer_4x4(mut coeff: *const i32, mut in_: *mut int32x4_t) {
    *in_ = vld1q_s32(coeff);
    *in_.add(1) = vld1q_s32(coeff.add(4));
    *in_.add(2) = vld1q_s32(coeff.add(8));
    *in_.add(3) = vld1q_s32(coeff.add(12));
}

unsafe fn addsub_neon(
    mut in0: int32x4_t,
    mut in1: int32x4_t,
    mut out0: *mut int32x4_t,
    mut out1: *mut int32x4_t,
    mut clamp_lo: *const int32x4_t,
    mut clamp_hi: *const int32x4_t,
) {
    let mut a0: int32x4_t = vaddq_s32(in0, in1);
    let mut a1: int32x4_t = vsubq_s32(in0, in1);

    a0 = vmaxq_s32(a0, *clamp_lo);
    a0 = vminq_s32(a0, *clamp_hi);
    a1 = vmaxq_s32(a1, *clamp_lo);
    a1 = vminq_s32(a1, *clamp_hi);

    *out0 = a0;
    *out1 = a1;
}

unsafe fn shift_and_clamp_neon(
    mut in0: *mut int32x4_t,
    mut in1: *mut int32x4_t,
    mut clamp_lo: *const int32x4_t,
    mut clamp_hi: *const int32x4_t,
    mut v_shift: *const int32x4_t,
) {
    let mut in0_w_offset: int32x4_t = vrshlq_s32(*in0, *v_shift);
    let mut in1_w_offset: int32x4_t = vrshlq_s32(*in1, *v_shift);

    in0_w_offset = vmaxq_s32(in0_w_offset, *clamp_lo);
    in0_w_offset = vminq_s32(in0_w_offset, *clamp_hi);
    in1_w_offset = vmaxq_s32(in1_w_offset, *clamp_lo);
    in1_w_offset = vminq_s32(in1_w_offset, *clamp_hi);

    *in0 = in0_w_offset;
    *in1 = in1_w_offset;
}

#[inline(always)]
unsafe fn idct32_stage4_neon(
    mut bf1: *mut int32x4_t,
    mut cospi: *const i32,
    mut v_bit: *const int32x4_t,
    mut rnding: *const int32x4_t,
) {
    let mut temp1: int32x4_t = core::mem::zeroed();
    let mut temp2: int32x4_t = core::mem::zeroed();
    temp1 = half_btf_neon_mode10_r(
        cospi.add(8),
        bf1.add(17),
        cospi.add(56),
        bf1.add(30),
        v_bit,
        rnding,
    );
    *bf1.add(30) =
        half_btf_neon_r(cospi.add(56), bf1.add(17), cospi.add(8), bf1.add(30), v_bit, rnding);
    *bf1.add(17) = temp1;

    temp2 = half_btf_neon_mode11_r(
        cospi.add(56),
        bf1.add(18),
        cospi.add(8),
        bf1.add(29),
        v_bit,
        rnding,
    );
    *bf1.add(29) = half_btf_neon_mode10_r(
        cospi.add(8),
        bf1.add(18),
        cospi.add(56),
        bf1.add(29),
        v_bit,
        rnding,
    );
    *bf1.add(18) = temp2;

    temp1 = half_btf_neon_mode10_r(
        cospi.add(40),
        bf1.add(21),
        cospi.add(24),
        bf1.add(26),
        v_bit,
        rnding,
    );
    *bf1.add(26) =
        half_btf_neon_r(cospi.add(24), bf1.add(21), cospi.add(40), bf1.add(26), v_bit, rnding);
    *bf1.add(21) = temp1;

    temp2 = half_btf_neon_mode11_r(
        cospi.add(24),
        bf1.add(22),
        cospi.add(40),
        bf1.add(25),
        v_bit,
        rnding,
    );
    *bf1.add(25) = half_btf_neon_mode10_r(
        cospi.add(40),
        bf1.add(22),
        cospi.add(24),
        bf1.add(25),
        v_bit,
        rnding,
    );
    *bf1.add(22) = temp2;
}

#[inline(always)]
unsafe fn idct32_stage5_neon(
    mut bf1: *mut int32x4_t,
    mut cospi: *const i32,
    mut clamp_lo: *const int32x4_t,
    mut clamp_hi: *const int32x4_t,
    mut v_bit: *const int32x4_t,
    mut rnding: *const int32x4_t,
) {
    let mut temp1: int32x4_t = core::mem::zeroed();
    let mut temp2: int32x4_t = core::mem::zeroed();
    temp1 = half_btf_neon_mode10_r(
        cospi.add(16),
        bf1.add(9),
        cospi.add(48),
        bf1.add(14),
        v_bit,
        rnding,
    );
    *bf1.add(14) =
        half_btf_neon_r(cospi.add(48), bf1.add(9), cospi.add(16), bf1.add(14), v_bit, rnding);
    *bf1.add(9) = temp1;

    temp2 = half_btf_neon_mode11_r(
        cospi.add(48),
        bf1.add(10),
        cospi.add(16),
        bf1.add(13),
        v_bit,
        rnding,
    );
    *bf1.add(13) = half_btf_neon_mode10_r(
        cospi.add(16),
        bf1.add(10),
        cospi.add(48),
        bf1.add(13),
        v_bit,
        rnding,
    );
    *bf1.add(10) = temp2;

    addsub_neon(*bf1.add(16), *bf1.add(19), bf1.add(16), bf1.add(19), clamp_lo, clamp_hi);
    addsub_neon(*bf1.add(17), *bf1.add(18), bf1.add(17), bf1.add(18), clamp_lo, clamp_hi);
    addsub_neon(*bf1.add(23), *bf1.add(20), bf1.add(23), bf1.add(20), clamp_lo, clamp_hi);
    addsub_neon(*bf1.add(22), *bf1.add(21), bf1.add(22), bf1.add(21), clamp_lo, clamp_hi);
    addsub_neon(*bf1.add(24), *bf1.add(27), bf1.add(24), bf1.add(27), clamp_lo, clamp_hi);
    addsub_neon(*bf1.add(25), *bf1.add(26), bf1.add(25), bf1.add(26), clamp_lo, clamp_hi);
    addsub_neon(*bf1.add(31), *bf1.add(28), bf1.add(31), bf1.add(28), clamp_lo, clamp_hi);
    addsub_neon(*bf1.add(30), *bf1.add(29), bf1.add(30), bf1.add(29), clamp_lo, clamp_hi);
}

#[inline(always)]
unsafe fn idct32_stage6_neon(
    mut bf1: *mut int32x4_t,
    mut cospi: *const i32,
    mut clamp_lo: *const int32x4_t,
    mut clamp_hi: *const int32x4_t,
    mut v_bit: *const int32x4_t,
    mut rnding: *const int32x4_t,
) {
    let mut temp1: int32x4_t = core::mem::zeroed();
    let mut temp2: int32x4_t = core::mem::zeroed();
    temp1 =
        half_btf_neon_mode10_r(cospi.add(32), bf1.add(5), cospi.add(32), bf1.add(6), v_bit, rnding);
    *bf1.add(6) =
        half_btf_neon_r(cospi.add(32), bf1.add(5), cospi.add(32), bf1.add(6), v_bit, rnding);
    *bf1.add(5) = temp1;

    addsub_neon(*bf1.add(8), *bf1.add(11), bf1.add(8), bf1.add(11), clamp_lo, clamp_hi);
    addsub_neon(*bf1.add(9), *bf1.add(10), bf1.add(9), bf1.add(10), clamp_lo, clamp_hi);
    addsub_neon(*bf1.add(15), *bf1.add(12), bf1.add(15), bf1.add(12), clamp_lo, clamp_hi);
    addsub_neon(*bf1.add(14), *bf1.add(13), bf1.add(14), bf1.add(13), clamp_lo, clamp_hi);

    temp1 = half_btf_neon_mode10_r(
        cospi.add(16),
        bf1.add(18),
        cospi.add(48),
        bf1.add(29),
        v_bit,
        rnding,
    );
    *bf1.add(29) =
        half_btf_neon_r(cospi.add(48), bf1.add(18), cospi.add(16), bf1.add(29), v_bit, rnding);
    *bf1.add(18) = temp1;
    temp2 = half_btf_neon_mode10_r(
        cospi.add(16),
        bf1.add(19),
        cospi.add(48),
        bf1.add(28),
        v_bit,
        rnding,
    );
    *bf1.add(28) =
        half_btf_neon_r(cospi.add(48), bf1.add(19), cospi.add(16), bf1.add(28), v_bit, rnding);
    *bf1.add(19) = temp2;
    temp1 = half_btf_neon_mode11_r(
        cospi.add(48),
        bf1.add(20),
        cospi.add(16),
        bf1.add(27),
        v_bit,
        rnding,
    );
    *bf1.add(27) = half_btf_neon_mode10_r(
        cospi.add(16),
        bf1.add(20),
        cospi.add(48),
        bf1.add(27),
        v_bit,
        rnding,
    );
    *bf1.add(20) = temp1;
    temp2 = half_btf_neon_mode11_r(
        cospi.add(48),
        bf1.add(21),
        cospi.add(16),
        bf1.add(26),
        v_bit,
        rnding,
    );
    *bf1.add(26) = half_btf_neon_mode10_r(
        cospi.add(16),
        bf1.add(21),
        cospi.add(48),
        bf1.add(26),
        v_bit,
        rnding,
    );
    *bf1.add(21) = temp2;
}

#[inline(always)]
unsafe fn idct32_stage7_neon(
    mut bf1: *mut int32x4_t,
    mut cospi: *const i32,
    mut clamp_lo: *const int32x4_t,
    mut clamp_hi: *const int32x4_t,
    mut v_bit: *const int32x4_t,
    mut rnding: *const int32x4_t,
) {
    let mut temp1: int32x4_t = core::mem::zeroed();
    let mut temp2: int32x4_t = core::mem::zeroed();
    addsub_neon(*bf1, *bf1.add(7), bf1, bf1.add(7), clamp_lo, clamp_hi);
    addsub_neon(*bf1.add(1), *bf1.add(6), bf1.add(1), bf1.add(6), clamp_lo, clamp_hi);
    addsub_neon(*bf1.add(2), *bf1.add(5), bf1.add(2), bf1.add(5), clamp_lo, clamp_hi);
    addsub_neon(*bf1.add(3), *bf1.add(4), bf1.add(3), bf1.add(4), clamp_lo, clamp_hi);
    temp1 = half_btf_neon_mode10_r(
        cospi.add(32),
        bf1.add(10),
        cospi.add(32),
        bf1.add(13),
        v_bit,
        rnding,
    );
    *bf1.add(13) =
        half_btf_neon_r(cospi.add(32), bf1.add(10), cospi.add(32), bf1.add(13), v_bit, rnding);
    *bf1.add(10) = temp1;
    temp2 = half_btf_neon_mode10_r(
        cospi.add(32),
        bf1.add(11),
        cospi.add(32),
        bf1.add(12),
        v_bit,
        rnding,
    );
    *bf1.add(12) =
        half_btf_neon_r(cospi.add(32), bf1.add(11), cospi.add(32), bf1.add(12), v_bit, rnding);
    *bf1.add(11) = temp2;

    addsub_neon(*bf1.add(16), *bf1.add(23), bf1.add(16), bf1.add(23), clamp_lo, clamp_hi);
    addsub_neon(*bf1.add(17), *bf1.add(22), bf1.add(17), bf1.add(22), clamp_lo, clamp_hi);
    addsub_neon(*bf1.add(18), *bf1.add(21), bf1.add(18), bf1.add(21), clamp_lo, clamp_hi);
    addsub_neon(*bf1.add(19), *bf1.add(20), bf1.add(19), bf1.add(20), clamp_lo, clamp_hi);
    addsub_neon(*bf1.add(31), *bf1.add(24), bf1.add(31), bf1.add(24), clamp_lo, clamp_hi);
    addsub_neon(*bf1.add(30), *bf1.add(25), bf1.add(30), bf1.add(25), clamp_lo, clamp_hi);
    addsub_neon(*bf1.add(29), *bf1.add(26), bf1.add(29), bf1.add(26), clamp_lo, clamp_hi);
    addsub_neon(*bf1.add(28), *bf1.add(27), bf1.add(28), bf1.add(27), clamp_lo, clamp_hi);
}

#[inline(always)]
unsafe fn idct32_stage8_neon(
    mut bf1: *mut int32x4_t,
    mut cospi: *const i32,
    mut clamp_lo: *const int32x4_t,
    mut clamp_hi: *const int32x4_t,
    mut v_bit: *const int32x4_t,
    mut rnding: *const int32x4_t,
) {
    let mut temp1: int32x4_t = core::mem::zeroed();
    let mut temp2: int32x4_t = core::mem::zeroed();
    addsub_neon(*bf1, *bf1.add(15), bf1, bf1.add(15), clamp_lo, clamp_hi);
    addsub_neon(*bf1.add(1), *bf1.add(14), bf1.add(1), bf1.add(14), clamp_lo, clamp_hi);
    addsub_neon(*bf1.add(2), *bf1.add(13), bf1.add(2), bf1.add(13), clamp_lo, clamp_hi);
    addsub_neon(*bf1.add(3), *bf1.add(12), bf1.add(3), bf1.add(12), clamp_lo, clamp_hi);
    addsub_neon(*bf1.add(4), *bf1.add(11), bf1.add(4), bf1.add(11), clamp_lo, clamp_hi);
    addsub_neon(*bf1.add(5), *bf1.add(10), bf1.add(5), bf1.add(10), clamp_lo, clamp_hi);
    addsub_neon(*bf1.add(6), *bf1.add(9), bf1.add(6), bf1.add(9), clamp_lo, clamp_hi);
    addsub_neon(*bf1.add(7), *bf1.add(8), bf1.add(7), bf1.add(8), clamp_lo, clamp_hi);
    temp1 = half_btf_neon_mode10_r(
        cospi.add(32),
        bf1.add(20),
        cospi.add(32),
        bf1.add(27),
        v_bit,
        rnding,
    );
    *bf1.add(27) =
        half_btf_neon_r(cospi.add(32), bf1.add(20), cospi.add(32), bf1.add(27), v_bit, rnding);
    *bf1.add(20) = temp1;
    temp2 = half_btf_neon_mode10_r(
        cospi.add(32),
        bf1.add(21),
        cospi.add(32),
        bf1.add(26),
        v_bit,
        rnding,
    );
    *bf1.add(26) =
        half_btf_neon_r(cospi.add(32), bf1.add(21), cospi.add(32), bf1.add(26), v_bit, rnding);
    *bf1.add(21) = temp2;
    temp1 = half_btf_neon_mode10_r(
        cospi.add(32),
        bf1.add(22),
        cospi.add(32),
        bf1.add(25),
        v_bit,
        rnding,
    );
    *bf1.add(25) =
        half_btf_neon_r(cospi.add(32), bf1.add(22), cospi.add(32), bf1.add(25), v_bit, rnding);
    *bf1.add(22) = temp1;
    temp2 = half_btf_neon_mode10_r(
        cospi.add(32),
        bf1.add(23),
        cospi.add(32),
        bf1.add(24),
        v_bit,
        rnding,
    );
    *bf1.add(24) =
        half_btf_neon_r(cospi.add(32), bf1.add(23), cospi.add(32), bf1.add(24), v_bit, rnding);
    *bf1.add(23) = temp2;
}

#[inline(always)]
unsafe fn idct32_stage9_neon(
    mut bf1: *mut int32x4_t,
    mut out: *mut int32x4_t,
    mut do_cols: i32,
    mut bd: i32,
    mut out_shift: i32,
    mut clamp_lo: *const int32x4_t,
    mut clamp_hi: *const int32x4_t,
) {
    addsub_neon(*bf1, *bf1.add(31), out, out.add(31), clamp_lo, clamp_hi);
    addsub_neon(*bf1.add(1), *bf1.add(30), out.add(1), out.add(30), clamp_lo, clamp_hi);
    addsub_neon(*bf1.add(2), *bf1.add(29), out.add(2), out.add(29), clamp_lo, clamp_hi);
    addsub_neon(*bf1.add(3), *bf1.add(28), out.add(3), out.add(28), clamp_lo, clamp_hi);
    addsub_neon(*bf1.add(4), *bf1.add(27), out.add(4), out.add(27), clamp_lo, clamp_hi);
    addsub_neon(*bf1.add(5), *bf1.add(26), out.add(5), out.add(26), clamp_lo, clamp_hi);
    addsub_neon(*bf1.add(6), *bf1.add(25), out.add(6), out.add(25), clamp_lo, clamp_hi);
    addsub_neon(*bf1.add(7), *bf1.add(24), out.add(7), out.add(24), clamp_lo, clamp_hi);
    addsub_neon(*bf1.add(8), *bf1.add(23), out.add(8), out.add(23), clamp_lo, clamp_hi);
    addsub_neon(*bf1.add(9), *bf1.add(22), out.add(9), out.add(22), clamp_lo, clamp_hi);
    addsub_neon(*bf1.add(10), *bf1.add(21), out.add(10), out.add(21), clamp_lo, clamp_hi);
    addsub_neon(*bf1.add(11), *bf1.add(20), out.add(11), out.add(20), clamp_lo, clamp_hi);
    addsub_neon(*bf1.add(12), *bf1.add(19), out.add(12), out.add(19), clamp_lo, clamp_hi);
    addsub_neon(*bf1.add(13), *bf1.add(18), out.add(13), out.add(18), clamp_lo, clamp_hi);
    addsub_neon(*bf1.add(14), *bf1.add(17), out.add(14), out.add(17), clamp_lo, clamp_hi);
    addsub_neon(*bf1.add(15), *bf1.add(16), out.add(15), out.add(16), clamp_lo, clamp_hi);

    if !(do_cols != 0) {
        let log_range_out: i32 = get_log_range_out(bd);
        let clamp_lo_out: int32x4_t = vdupq_n_s32(-(1 << (log_range_out - 1)));
        let clamp_hi_out: int32x4_t = vdupq_n_s32((1 << (log_range_out - 1)) - 1);
        for i in (0..32).step_by(8) {
            round_shift_4x4(out.offset(i as isize), out_shift);
            round_shift_4x4(out.offset(i as isize).add(4), out_shift);
        }
        highbd_clamp_s32_neon(out, out, &raw const clamp_lo_out, &raw const clamp_hi_out, 32);
    }
}

unsafe fn neg_shift_neon(
    mut in0: *const int32x4_t,
    mut in1: *const int32x4_t,
    mut out0: *mut int32x4_t,
    mut out1: *mut int32x4_t,
    mut clamp_lo: *const int32x4_t,
    mut clamp_hi: *const int32x4_t,
    mut v_shift: *const int32x4_t,
    mut offset: *mut int32x4_t,
) {
    let mut a0: int32x4_t = vaddq_s32(*offset, *in0);
    let mut a1: int32x4_t = vsubq_s32(*offset, *in1);

    a0 = vshlq_s32(a0, *v_shift);
    a1 = vshlq_s32(a1, *v_shift);

    a0 = vmaxq_s32(a0, *clamp_lo);
    a0 = vminq_s32(a0, *clamp_hi);
    a1 = vmaxq_s32(a1, *clamp_lo);
    a1 = vminq_s32(a1, *clamp_hi);

    *out0 = a0;
    *out1 = a1;
}

unsafe fn idct4x4_neon(
    mut in_: *mut int32x4_t,
    mut out: *mut int32x4_t,
    mut bit: i32,
    mut do_cols: i32,
    mut bd: i32,
    mut out_shift: i32,
) {
    let mut cospi: *const i32 = cospi_arr(bit);
    let mut log_range: i32 = get_log_range(bd, do_cols);
    let mut clamp_lo: int32x4_t = vdupq_n_s32(-(1 << (log_range - 1)));
    let mut clamp_hi: int32x4_t = vdupq_n_s32((1 << (log_range - 1)) - 1);
    let mut rnding: int32x4_t = vdupq_n_s32(1 << (bit - 1));

    let mut u0: int32x4_t = core::mem::zeroed();
    let mut u1: int32x4_t = core::mem::zeroed();
    let mut u2: int32x4_t = core::mem::zeroed();
    let mut u3: int32x4_t = core::mem::zeroed();
    let mut v0: int32x4_t = core::mem::zeroed();
    let mut v1: int32x4_t = core::mem::zeroed();
    let mut v2: int32x4_t = core::mem::zeroed();
    let mut v3: int32x4_t = core::mem::zeroed();
    let mut x: int32x4_t = core::mem::zeroed();
    let mut y: int32x4_t = core::mem::zeroed();

    // Stage 0-1-2
    u0 = *in_;
    u1 = *in_.add(1);
    u2 = *in_.add(2);
    u3 = *in_.add(3);

    let v_bit: int32x4_t = vdupq_n_s32(-bit);

    x = vmlaq_n_s32(rnding, u0, *cospi.add(32));
    y = vmulq_n_s32(u2, *cospi.add(32));
    v0 = vaddq_s32(x, y);
    v0 = vshlq_s32(v0, v_bit);

    v1 = vsubq_s32(x, y);
    v1 = vshlq_s32(v1, v_bit);

    x = vmlaq_n_s32(rnding, u1, *cospi.add(48));
    v2 = vmlsq_n_s32(x, u3, *cospi.add(16));
    v2 = vshlq_s32(v2, v_bit);

    x = vmlaq_n_s32(rnding, u1, *cospi.add(16));
    v3 = vmlaq_n_s32(x, u3, *cospi.add(48));
    v3 = vshlq_s32(v3, v_bit);
    // Stage 3
    addsub_neon(v0, v3, out, out.add(3), &raw mut clamp_lo, &raw mut clamp_hi);
    addsub_neon(v1, v2, out.add(1), out.add(2), &raw mut clamp_lo, &raw mut clamp_hi);

    if !(do_cols != 0) {
        log_range = get_log_range_out(bd);
        clamp_lo = vdupq_n_s32(-(1 << (log_range - 1)));
        clamp_hi = vdupq_n_s32((1 << (log_range - 1)) - 1);
        let v_shift: int32x4_t = vdupq_n_s32(-out_shift);
        shift_and_clamp_neon(
            out,
            out.add(3),
            &raw mut clamp_lo,
            &raw mut clamp_hi,
            &raw const v_shift,
        );
        shift_and_clamp_neon(
            out.add(1),
            out.add(2),
            &raw mut clamp_lo,
            &raw mut clamp_hi,
            &raw const v_shift,
        );
    }
}

unsafe fn iadst4x4_neon(
    mut in_: *mut int32x4_t,
    mut out: *mut int32x4_t,
    mut bit: i32,
    mut do_cols: i32,
    mut bd: i32,
    mut out_shift: i32,
) {
    let mut sinpi: *const i32 = sinpi_arr(bit);
    let zero: int32x4_t = vdupq_n_s32(0);
    let mut rnding: int64x2_t = vdupq_n_s64(1i64 << (bit + 4 - 1));
    let mul: int32x2_t = vdup_n_s32(1 << 4);
    let mut t: int32x4_t = core::mem::zeroed();
    let mut s0: int32x4_t = core::mem::zeroed();
    let mut s1: int32x4_t = core::mem::zeroed();
    let mut s2: int32x4_t = core::mem::zeroed();
    let mut s3: int32x4_t = core::mem::zeroed();
    let mut s4: int32x4_t = core::mem::zeroed();
    let mut s5: int32x4_t = core::mem::zeroed();
    let mut s6: int32x4_t = core::mem::zeroed();
    let mut s7: int32x4_t = core::mem::zeroed();
    let mut x0: int32x4_t = core::mem::zeroed();
    let mut x1: int32x4_t = core::mem::zeroed();
    let mut x2: int32x4_t = core::mem::zeroed();
    let mut x3: int32x4_t = core::mem::zeroed();
    let mut u0: int32x4_t = core::mem::zeroed();
    let mut u1: int32x4_t = core::mem::zeroed();
    let mut u2: int32x4_t = core::mem::zeroed();
    let mut u3: int32x4_t = core::mem::zeroed();

    x0 = *in_;
    x1 = *in_.add(1);
    x2 = *in_.add(2);
    x3 = *in_.add(3);

    s0 = vmulq_n_s32(x0, *sinpi.add(1));
    s1 = vmulq_n_s32(x0, *sinpi.add(2));
    s2 = vmulq_n_s32(x1, *sinpi.add(3));
    s3 = vmulq_n_s32(x2, *sinpi.add(4));
    s4 = vmulq_n_s32(x2, *sinpi.add(1));
    s5 = vmulq_n_s32(x3, *sinpi.add(2));
    s6 = vmulq_n_s32(x3, *sinpi.add(4));
    t = vsubq_s32(x0, x2);
    s7 = vaddq_s32(t, x3);

    t = vaddq_s32(s0, s3);
    s0 = vaddq_s32(t, s5);
    t = vsubq_s32(s1, s4);
    s1 = vsubq_s32(t, s6);
    s3 = s2;
    s2 = vmulq_n_s32(s7, *sinpi.add(3));

    u0 = vaddq_s32(s0, s3);
    u1 = vaddq_s32(s1, s3);
    u2 = s2;
    t = vaddq_s32(s0, s1);
    u3 = vsubq_s32(t, s3);

    // u0
    let mut u0x: int32x4x2_t = core::mem::zeroed();
    u0x.0 = vreinterpretq_s32_s64(vmull_s32(vmovn_s64(vreinterpretq_s64_s32(u0)), mul));
    u0x.0 = vreinterpretq_s32_s64(vaddq_s64(vreinterpretq_s64_s32(u0x.0), rnding));

    u0 = vextq_s32::<1>(u0, zero);
    u0x.1 = vreinterpretq_s32_s64(vmull_s32(vmovn_s64(vreinterpretq_s64_s32(u0)), mul));
    u0x.1 = vreinterpretq_s32_s64(vaddq_s64(vreinterpretq_s64_s32(u0x.1), rnding));

    u0x.0 = vreinterpretq_s32_s16(vextq_s16::<1>(
        vreinterpretq_s16_s32(u0x.0),
        vreinterpretq_s16_s32(zero),
    ));
    u0x.1 = vreinterpretq_s32_s16(vextq_s16::<1>(
        vreinterpretq_s16_s32(u0x.1),
        vreinterpretq_s16_s32(zero),
    ));

    u0x = vzipq_s32(u0x.0, u0x.1);
    u0 = vreinterpretq_s32_s64(vzip1q_s64(
        vreinterpretq_s64_s32(u0x.0),
        vreinterpretq_s64_s32(u0x.1),
    ));
    // u1
    let mut u1x: int32x4x2_t = core::mem::zeroed();
    u1x.0 = vreinterpretq_s32_s64(vmull_s32(vmovn_s64(vreinterpretq_s64_s32(u1)), mul));
    u1x.0 = vreinterpretq_s32_s64(vaddq_s64(vreinterpretq_s64_s32(u1x.0), rnding));

    u1 = vextq_s32::<1>(u1, zero);
    u1x.1 = vreinterpretq_s32_s64(vmull_s32(vmovn_s64(vreinterpretq_s64_s32(u1)), mul));
    u1x.1 = vreinterpretq_s32_s64(vaddq_s64(vreinterpretq_s64_s32(u1x.1), rnding));

    u1x.0 = vreinterpretq_s32_s16(vextq_s16::<1>(
        vreinterpretq_s16_s32(u1x.0),
        vreinterpretq_s16_s32(zero),
    ));
    u1x.1 = vreinterpretq_s32_s16(vextq_s16::<1>(
        vreinterpretq_s16_s32(u1x.1),
        vreinterpretq_s16_s32(zero),
    ));

    u1x = vzipq_s32(u1x.0, u1x.1);
    u1 = vreinterpretq_s32_s64(vzip1q_s64(
        vreinterpretq_s64_s32(u1x.0),
        vreinterpretq_s64_s32(u1x.1),
    ));

    // u2
    let mut u2x: int32x4x2_t = core::mem::zeroed();
    u2x.0 = vreinterpretq_s32_s64(vmull_s32(vmovn_s64(vreinterpretq_s64_s32(u2)), mul));
    u2x.0 = vreinterpretq_s32_s64(vaddq_s64(vreinterpretq_s64_s32(u2x.0), rnding));

    u2 = vextq_s32::<1>(u2, zero);
    u2x.1 = vreinterpretq_s32_s64(vmull_s32(vmovn_s64(vreinterpretq_s64_s32(u2)), mul));
    u2x.1 = vreinterpretq_s32_s64(vaddq_s64(vreinterpretq_s64_s32(u2x.1), rnding));

    u2x.0 = vreinterpretq_s32_s16(vextq_s16::<1>(
        vreinterpretq_s16_s32(u2x.0),
        vreinterpretq_s16_s32(zero),
    ));
    u2x.1 = vreinterpretq_s32_s16(vextq_s16::<1>(
        vreinterpretq_s16_s32(u2x.1),
        vreinterpretq_s16_s32(zero),
    ));

    u2x = vzipq_s32(u2x.0, u2x.1);
    u2 = vreinterpretq_s32_s64(vzip1q_s64(
        vreinterpretq_s64_s32(u2x.0),
        vreinterpretq_s64_s32(u2x.1),
    ));

    // u3
    let mut u3x: int32x4x2_t = core::mem::zeroed();
    u3x.0 = vreinterpretq_s32_s64(vmull_s32(vmovn_s64(vreinterpretq_s64_s32(u3)), mul));
    u3x.0 = vreinterpretq_s32_s64(vaddq_s64(vreinterpretq_s64_s32(u3x.0), rnding));

    u3 = vextq_s32::<1>(u3, zero);
    u3x.1 = vreinterpretq_s32_s64(vmull_s32(vmovn_s64(vreinterpretq_s64_s32(u3)), mul));
    u3x.1 = vreinterpretq_s32_s64(vaddq_s64(vreinterpretq_s64_s32(u3x.1), rnding));

    u3x.0 = vreinterpretq_s32_s16(vextq_s16::<1>(
        vreinterpretq_s16_s32(u3x.0),
        vreinterpretq_s16_s32(zero),
    ));
    u3x.1 = vreinterpretq_s32_s16(vextq_s16::<1>(
        vreinterpretq_s16_s32(u3x.1),
        vreinterpretq_s16_s32(zero),
    ));

    u3x = vzipq_s32(u3x.0, u3x.1);
    u3 = vreinterpretq_s32_s64(vzip1q_s64(
        vreinterpretq_s64_s32(u3x.0),
        vreinterpretq_s64_s32(u3x.1),
    ));

    *out = u0;
    *out.add(1) = u1;
    *out.add(2) = u2;
    *out.add(3) = u3;

    if !(do_cols != 0) {
        let log_range: i32 = get_log_range_out(bd);
        let clamp_lo: int32x4_t = vdupq_n_s32(-(1 << (log_range - 1)));
        let clamp_hi: int32x4_t = vdupq_n_s32((1 << (log_range - 1)) - 1);
        round_shift_4x4(out, out_shift);
        highbd_clamp_s32_neon(out, out, &raw const clamp_lo, &raw const clamp_hi, 4);
    }
}

unsafe fn write_buffer_4x4(
    mut in_: *mut int32x4_t,
    mut output: *mut u16,
    mut stride: i32,
    mut fliplr: i32,
    mut flipud: i32,
    mut shift: i32,
    mut bd: i32,
) {
    let mut u0: uint32x4_t = core::mem::zeroed();
    let mut u1: uint32x4_t = core::mem::zeroed();
    let mut u2: uint32x4_t = core::mem::zeroed();
    let mut u3: uint32x4_t = core::mem::zeroed();
    let mut v0: uint16x4_t = core::mem::zeroed();
    let mut v1: uint16x4_t = core::mem::zeroed();
    let mut v2: uint16x4_t = core::mem::zeroed();
    let mut v3: uint16x4_t = core::mem::zeroed();
    round_shift_4x4(in_, shift);

    v0 = vld1_u16(output.offset((0 * stride) as isize));
    v1 = vld1_u16(output.offset((1 * stride) as isize));
    v2 = vld1_u16(output.offset((2 * stride) as isize));
    v3 = vld1_u16(output.offset((3 * stride) as isize));

    if fliplr != 0 {
        u0 = vrev64q_u32(vreinterpretq_u32_s32(*in_));
        *in_ = vreinterpretq_s32_u32(vextq_u32::<2>(u0, u0));
        u0 = vrev64q_u32(vreinterpretq_u32_s32(*in_.add(1)));
        *in_.add(1) = vreinterpretq_s32_u32(vextq_u32::<2>(u0, u0));
        u0 = vrev64q_u32(vreinterpretq_u32_s32(*in_.add(2)));
        *in_.add(2) = vreinterpretq_s32_u32(vextq_u32::<2>(u0, u0));
        u0 = vrev64q_u32(vreinterpretq_u32_s32(*in_.add(3)));
        *in_.add(3) = vreinterpretq_s32_u32(vextq_u32::<2>(u0, u0));
    }

    if flipud != 0 {
        u0 = vaddw_u16(vreinterpretq_u32_s32(*in_.add(3)), v0);
        u1 = vaddw_u16(vreinterpretq_u32_s32(*in_.add(2)), v1);
        u2 = vaddw_u16(vreinterpretq_u32_s32(*in_.add(1)), v2);
        u3 = vaddw_u16(vreinterpretq_u32_s32(*in_), v3);
    } else {
        u0 = vaddw_u16(vreinterpretq_u32_s32(*in_), v0);
        u1 = vaddw_u16(vreinterpretq_u32_s32(*in_.add(1)), v1);
        u2 = vaddw_u16(vreinterpretq_u32_s32(*in_.add(2)), v2);
        u3 = vaddw_u16(vreinterpretq_u32_s32(*in_.add(3)), v3);
    }

    let mut u4: uint16x8_t = vcombine_u16(vqmovn_u32(u0), vqmovn_u32(u1));
    let mut u5: uint16x8_t = vcombine_u16(vqmovn_u32(u2), vqmovn_u32(u3));
    let vmin: uint16x8_t = vdupq_n_u16(0);
    let vmax: uint16x8_t = vdupq_n_u16(((1 << bd) - 1) as u16);
    u4 = highbd_clamp_u16(&raw mut u4, &raw const vmin, &raw const vmax);
    u5 = highbd_clamp_u16(&raw mut u5, &raw const vmin, &raw const vmax);

    vst1_u16(output.offset((0 * stride) as isize), vget_low_u16(u4));
    vst1_u16(output.offset((1 * stride) as isize), vget_high_u16(u4));
    vst1_u16(output.offset((2 * stride) as isize), vget_low_u16(u5));
    vst1_u16(output.offset((3 * stride) as isize), vget_high_u16(u5));
}

unsafe fn iidentity4_neon(
    mut in_: *mut int32x4_t,
    mut out: *mut int32x4_t,
    mut bit: i32,
    mut do_cols: i32,
    mut bd: i32,
    mut out_shift: i32,
) {
    let _ = bit;
    let mut zero: int32x4_t = vdupq_n_s32(0);
    let mut fact: int32x2_t = vdup_n_s32(NewSqrt2);
    let mut a0: int32x4x2_t = core::mem::zeroed();
    let rnding: int64x2_t = vdupq_n_s64((1 << (NewSqrt2Bits - 1)) as i64);

    for i in 0..4 {
        a0.0 = vreinterpretq_s32_s64(vmlal_s32(
            rnding,
            vmovn_s64(vreinterpretq_s64_s32(*in_.offset(i as isize))),
            fact,
        ));
        a0.0 = vreinterpretq_s32_s64(vshrq_n_s64::<NewSqrt2Bits>(vreinterpretq_s64_s32(a0.0)));
        a0.1 = vextq_s32::<1>(*in_.offset(i as isize), zero);
        a0.1 =
            vreinterpretq_s32_s64(vmlal_s32(rnding, vmovn_s64(vreinterpretq_s64_s32(a0.1)), fact));
        a0.1 = vreinterpretq_s32_s64(vshrq_n_s64::<NewSqrt2Bits>(vreinterpretq_s64_s32(a0.1)));

        a0 = vzipq_s32(a0.0, a0.1);
        *out.offset(i as isize) = vreinterpretq_s32_s64(vzip1q_s64(
            vreinterpretq_s64_s32(a0.0),
            vreinterpretq_s64_s32(a0.1),
        ));
    }
    if !(do_cols != 0) {
        let log_range: i32 = get_log_range_out(bd);
        let clamp_lo: int32x4_t = vdupq_n_s32(-(1 << (log_range - 1)));
        let clamp_hi: int32x4_t = vdupq_n_s32((1 << (log_range - 1)) - 1);
        round_shift_4x4(out, out_shift);
        highbd_clamp_s32_neon(out, out, &raw const clamp_lo, &raw const clamp_hi, 4);
    }
}

#[no_mangle]
pub unsafe extern "C-unwind" fn av1_inv_txfm2d_add_4x4_neon(
    mut input: *const i32,
    mut output: *mut u16,
    mut stride: i32,
    mut tx_type: TX_TYPE,
    mut bd: i32,
) {
    let mut in_: [int32x4_t; 4] = core::mem::zeroed();

    let mut shift: *const i8 = av1_inv_txfm_shift_ls[TX_4X4 as usize];

    match tx_type as i32 {
        DCT_DCT => {
            load_buffer_4x4(input, in_.as_mut_ptr());
            idct4x4_neon(in_.as_mut_ptr(), in_.as_mut_ptr(), INV_COS_BIT, 0, bd, 0);
            transpose_4x4(in_.as_mut_ptr(), in_.as_mut_ptr());
            idct4x4_neon(in_.as_mut_ptr(), in_.as_mut_ptr(), INV_COS_BIT, 1, bd, 0);
            write_buffer_4x4(in_.as_mut_ptr(), output, stride, 0, 0, -(*shift.add(1) as i32), bd);
        }
        ADST_DCT => {
            load_buffer_4x4(input, in_.as_mut_ptr());
            idct4x4_neon(in_.as_mut_ptr(), in_.as_mut_ptr(), INV_COS_BIT, 0, bd, 0);
            transpose_4x4(in_.as_mut_ptr(), in_.as_mut_ptr());
            iadst4x4_neon(in_.as_mut_ptr(), in_.as_mut_ptr(), INV_COS_BIT, 1, bd, 0);
            write_buffer_4x4(in_.as_mut_ptr(), output, stride, 0, 0, -(*shift.add(1) as i32), bd);
        }
        DCT_ADST => {
            load_buffer_4x4(input, in_.as_mut_ptr());
            iadst4x4_neon(in_.as_mut_ptr(), in_.as_mut_ptr(), INV_COS_BIT, 0, bd, 0);
            transpose_4x4(in_.as_mut_ptr(), in_.as_mut_ptr());
            idct4x4_neon(in_.as_mut_ptr(), in_.as_mut_ptr(), INV_COS_BIT, 1, bd, 0);
            write_buffer_4x4(in_.as_mut_ptr(), output, stride, 0, 0, -(*shift.add(1) as i32), bd);
        }
        ADST_ADST => {
            load_buffer_4x4(input, in_.as_mut_ptr());
            iadst4x4_neon(in_.as_mut_ptr(), in_.as_mut_ptr(), INV_COS_BIT, 0, bd, 0);
            transpose_4x4(in_.as_mut_ptr(), in_.as_mut_ptr());
            iadst4x4_neon(in_.as_mut_ptr(), in_.as_mut_ptr(), INV_COS_BIT, 1, bd, 0);
            write_buffer_4x4(in_.as_mut_ptr(), output, stride, 0, 0, -(*shift.add(1) as i32), bd);
        }
        FLIPADST_DCT => {
            load_buffer_4x4(input, in_.as_mut_ptr());
            idct4x4_neon(in_.as_mut_ptr(), in_.as_mut_ptr(), INV_COS_BIT, 0, bd, 0);
            transpose_4x4(in_.as_mut_ptr(), in_.as_mut_ptr());
            iadst4x4_neon(in_.as_mut_ptr(), in_.as_mut_ptr(), INV_COS_BIT, 1, bd, 0);
            write_buffer_4x4(in_.as_mut_ptr(), output, stride, 0, 1, -(*shift.add(1) as i32), bd);
        }
        DCT_FLIPADST => {
            load_buffer_4x4(input, in_.as_mut_ptr());
            iadst4x4_neon(in_.as_mut_ptr(), in_.as_mut_ptr(), INV_COS_BIT, 0, bd, 0);
            transpose_4x4(in_.as_mut_ptr(), in_.as_mut_ptr());
            idct4x4_neon(in_.as_mut_ptr(), in_.as_mut_ptr(), INV_COS_BIT, 1, bd, 0);
            write_buffer_4x4(in_.as_mut_ptr(), output, stride, 1, 0, -(*shift.add(1) as i32), bd);
        }
        FLIPADST_FLIPADST => {
            load_buffer_4x4(input, in_.as_mut_ptr());
            iadst4x4_neon(in_.as_mut_ptr(), in_.as_mut_ptr(), INV_COS_BIT, 0, bd, 0);
            transpose_4x4(in_.as_mut_ptr(), in_.as_mut_ptr());
            iadst4x4_neon(in_.as_mut_ptr(), in_.as_mut_ptr(), INV_COS_BIT, 1, bd, 0);
            write_buffer_4x4(in_.as_mut_ptr(), output, stride, 1, 1, -(*shift.add(1) as i32), bd);
        }
        ADST_FLIPADST => {
            load_buffer_4x4(input, in_.as_mut_ptr());
            iadst4x4_neon(in_.as_mut_ptr(), in_.as_mut_ptr(), INV_COS_BIT, 0, bd, 0);
            transpose_4x4(in_.as_mut_ptr(), in_.as_mut_ptr());
            iadst4x4_neon(in_.as_mut_ptr(), in_.as_mut_ptr(), INV_COS_BIT, 1, bd, 0);
            write_buffer_4x4(in_.as_mut_ptr(), output, stride, 1, 0, -(*shift.add(1) as i32), bd);
        }
        FLIPADST_ADST => {
            load_buffer_4x4(input, in_.as_mut_ptr());
            iadst4x4_neon(in_.as_mut_ptr(), in_.as_mut_ptr(), INV_COS_BIT, 0, bd, 0);
            transpose_4x4(in_.as_mut_ptr(), in_.as_mut_ptr());
            iadst4x4_neon(in_.as_mut_ptr(), in_.as_mut_ptr(), INV_COS_BIT, 1, bd, 0);
            write_buffer_4x4(in_.as_mut_ptr(), output, stride, 0, 1, -(*shift.add(1) as i32), bd);
        }
        IDTX => {
            load_buffer_4x4(input, in_.as_mut_ptr());
            iidentity4_neon(in_.as_mut_ptr(), in_.as_mut_ptr(), INV_COS_BIT, 0, bd, 0);
            transpose_4x4(in_.as_mut_ptr(), in_.as_mut_ptr());
            iidentity4_neon(in_.as_mut_ptr(), in_.as_mut_ptr(), INV_COS_BIT, 1, bd, 0);
            write_buffer_4x4(in_.as_mut_ptr(), output, stride, 0, 0, -(*shift.add(1) as i32), bd);
        }
        V_DCT => {
            load_buffer_4x4(input, in_.as_mut_ptr());
            iidentity4_neon(in_.as_mut_ptr(), in_.as_mut_ptr(), INV_COS_BIT, 0, bd, 0);
            transpose_4x4(in_.as_mut_ptr(), in_.as_mut_ptr());
            idct4x4_neon(in_.as_mut_ptr(), in_.as_mut_ptr(), INV_COS_BIT, 1, bd, 0);
            write_buffer_4x4(in_.as_mut_ptr(), output, stride, 0, 0, -(*shift.add(1) as i32), bd);
        }
        H_DCT => {
            load_buffer_4x4(input, in_.as_mut_ptr());
            idct4x4_neon(in_.as_mut_ptr(), in_.as_mut_ptr(), INV_COS_BIT, 0, bd, 0);
            transpose_4x4(in_.as_mut_ptr(), in_.as_mut_ptr());
            iidentity4_neon(in_.as_mut_ptr(), in_.as_mut_ptr(), INV_COS_BIT, 1, bd, 0);
            write_buffer_4x4(in_.as_mut_ptr(), output, stride, 0, 0, -(*shift.add(1) as i32), bd);
        }
        V_ADST => {
            load_buffer_4x4(input, in_.as_mut_ptr());
            iidentity4_neon(in_.as_mut_ptr(), in_.as_mut_ptr(), INV_COS_BIT, 0, bd, 0);
            transpose_4x4(in_.as_mut_ptr(), in_.as_mut_ptr());
            iadst4x4_neon(in_.as_mut_ptr(), in_.as_mut_ptr(), INV_COS_BIT, 1, bd, 0);
            write_buffer_4x4(in_.as_mut_ptr(), output, stride, 0, 0, -(*shift.add(1) as i32), bd);
        }
        H_ADST => {
            load_buffer_4x4(input, in_.as_mut_ptr());
            iadst4x4_neon(in_.as_mut_ptr(), in_.as_mut_ptr(), INV_COS_BIT, 0, bd, 0);
            transpose_4x4(in_.as_mut_ptr(), in_.as_mut_ptr());
            iidentity4_neon(in_.as_mut_ptr(), in_.as_mut_ptr(), INV_COS_BIT, 1, bd, 0);
            write_buffer_4x4(in_.as_mut_ptr(), output, stride, 0, 0, -(*shift.add(1) as i32), bd);
        }
        V_FLIPADST => {
            load_buffer_4x4(input, in_.as_mut_ptr());
            iidentity4_neon(in_.as_mut_ptr(), in_.as_mut_ptr(), INV_COS_BIT, 0, bd, 0);
            transpose_4x4(in_.as_mut_ptr(), in_.as_mut_ptr());
            iadst4x4_neon(in_.as_mut_ptr(), in_.as_mut_ptr(), INV_COS_BIT, 1, bd, 0);
            write_buffer_4x4(in_.as_mut_ptr(), output, stride, 0, 1, -(*shift.add(1) as i32), bd);
        }
        H_FLIPADST => {
            load_buffer_4x4(input, in_.as_mut_ptr());
            iadst4x4_neon(in_.as_mut_ptr(), in_.as_mut_ptr(), INV_COS_BIT, 0, bd, 0);
            transpose_4x4(in_.as_mut_ptr(), in_.as_mut_ptr());
            iidentity4_neon(in_.as_mut_ptr(), in_.as_mut_ptr(), INV_COS_BIT, 1, bd, 0);
            write_buffer_4x4(in_.as_mut_ptr(), output, stride, 1, 0, -(*shift.add(1) as i32), bd);
        }
        _ => {
            debug_assert!(false);
        }
    }
}
// 8x8

unsafe fn load_buffer_8x8(mut coeff: *const i32, mut in_: *mut int32x4_t) {
    *in_ = vld1q_s32(coeff);
    *in_.add(1) = vld1q_s32(coeff.add(4));
    *in_.add(2) = vld1q_s32(coeff.add(8));
    *in_.add(3) = vld1q_s32(coeff.add(12));
    *in_.add(4) = vld1q_s32(coeff.add(16));
    *in_.add(5) = vld1q_s32(coeff.add(20));
    *in_.add(6) = vld1q_s32(coeff.add(24));
    *in_.add(7) = vld1q_s32(coeff.add(28));
    *in_.add(8) = vld1q_s32(coeff.add(32));
    *in_.add(9) = vld1q_s32(coeff.add(36));
    *in_.add(10) = vld1q_s32(coeff.add(40));
    *in_.add(11) = vld1q_s32(coeff.add(44));
    *in_.add(12) = vld1q_s32(coeff.add(48));
    *in_.add(13) = vld1q_s32(coeff.add(52));
    *in_.add(14) = vld1q_s32(coeff.add(56));
    *in_.add(15) = vld1q_s32(coeff.add(60));
}

unsafe fn idct8x8_neon(
    mut in_: *mut int32x4_t,
    mut out: *mut int32x4_t,
    mut bit: i32,
    mut do_cols: i32,
    mut bd: i32,
    mut out_shift: i32,
) {
    let mut cospi: *const i32 = cospi_arr(bit);
    let log_range: i32 = get_log_range(bd, do_cols);
    let clamp_lo: int32x4_t = vdupq_n_s32(-(1 << (log_range - 1)));
    let clamp_hi: int32x4_t = vdupq_n_s32((1 << (log_range - 1)) - 1);
    let mut u0: int32x4_t = core::mem::zeroed();
    let mut u1: int32x4_t = core::mem::zeroed();
    let mut u2: int32x4_t = core::mem::zeroed();
    let mut u3: int32x4_t = core::mem::zeroed();
    let mut u4: int32x4_t = core::mem::zeroed();
    let mut u5: int32x4_t = core::mem::zeroed();
    let mut u6: int32x4_t = core::mem::zeroed();
    let mut u7: int32x4_t = core::mem::zeroed();
    let mut v0: int32x4_t = core::mem::zeroed();
    let mut v1: int32x4_t = core::mem::zeroed();
    let mut v2: int32x4_t = core::mem::zeroed();
    let mut v3: int32x4_t = core::mem::zeroed();
    let mut v4: int32x4_t = core::mem::zeroed();
    let mut v5: int32x4_t = core::mem::zeroed();
    let mut v6: int32x4_t = core::mem::zeroed();
    let mut v7: int32x4_t = core::mem::zeroed();
    let mut x: int32x4_t = core::mem::zeroed();
    let mut y: int32x4_t = core::mem::zeroed();
    let mut col: i32 = 0;
    let rnding: int32x4_t = vdupq_n_s32(1 << (bit - 1));
    let v_bit: int32x4_t = vdupq_n_s32(-bit);
    // Note:
    //  Even column: 0, 2, ..., 14
    //  Odd column: 1, 3, ..., 15
    //  one even column plus one odd column constructs one row (8 coeffs)
    //  total we have 8 rows (8x8).
    col = 0;
    while col < 2 {
        // stage 0
        // stage 1
        // stage 2
        u0 = *in_.offset((0 * 2 + col) as isize);
        u1 = *in_.offset((4 * 2 + col) as isize);
        u2 = *in_.offset((2 * 2 + col) as isize);
        u3 = *in_.offset((6 * 2 + col) as isize);

        x = vmulq_n_s32(*in_.offset((1 * 2 + col) as isize), *cospi.add(56));
        u4 = vmlaq_n_s32(x, *in_.offset((7 * 2 + col) as isize), -*cospi.add(8));
        u4 = vaddq_s32(u4, rnding);
        u4 = vshlq_s32(u4, v_bit);

        x = vmulq_n_s32(*in_.offset((1 * 2 + col) as isize), *cospi.add(8));
        u7 = vmlaq_n_s32(x, *in_.offset((7 * 2 + col) as isize), *cospi.add(56));
        u7 = vaddq_s32(u7, rnding);
        u7 = vshlq_s32(u7, v_bit);

        x = vmulq_n_s32(*in_.offset((5 * 2 + col) as isize), *cospi.add(24));
        u5 = vmlaq_n_s32(x, *in_.offset((3 * 2 + col) as isize), -*cospi.add(40));
        u5 = vaddq_s32(u5, rnding);
        u5 = vshlq_s32(u5, v_bit);

        x = vmulq_n_s32(*in_.offset((5 * 2 + col) as isize), *cospi.add(40));
        u6 = vmlaq_n_s32(x, *in_.offset((3 * 2 + col) as isize), *cospi.add(24));
        u6 = vaddq_s32(u6, rnding);
        u6 = vshlq_s32(u6, v_bit);

        // stage 3
        x = vmulq_n_s32(u0, *cospi.add(32));
        y = vmulq_n_s32(u1, *cospi.add(32));
        v0 = vaddq_s32(x, y);
        v0 = vaddq_s32(v0, rnding);
        v0 = vshlq_s32(v0, v_bit);

        v1 = vsubq_s32(x, y);
        v1 = vaddq_s32(v1, rnding);
        v1 = vshlq_s32(v1, v_bit);

        x = vmulq_n_s32(u2, *cospi.add(48));
        v2 = vmlaq_n_s32(x, u3, -*cospi.add(16));
        v2 = vaddq_s32(v2, rnding);
        v2 = vshlq_s32(v2, v_bit);

        x = vmulq_n_s32(u2, *cospi.add(16));
        v3 = vmlaq_n_s32(x, u3, *cospi.add(48));
        v3 = vaddq_s32(v3, rnding);
        v3 = vshlq_s32(v3, v_bit);

        addsub_neon(u4, u5, &raw mut v4, &raw mut v5, &raw const clamp_lo, &raw const clamp_hi);
        addsub_neon(u7, u6, &raw mut v7, &raw mut v6, &raw const clamp_lo, &raw const clamp_hi);

        // stage 4
        addsub_neon(v0, v3, &raw mut u0, &raw mut u3, &raw const clamp_lo, &raw const clamp_hi);
        addsub_neon(v1, v2, &raw mut u1, &raw mut u2, &raw const clamp_lo, &raw const clamp_hi);
        u4 = v4;
        u7 = v7;

        x = vmulq_n_s32(v5, *cospi.add(32));
        y = vmulq_n_s32(v6, *cospi.add(32));
        u6 = vaddq_s32(y, x);
        u6 = vaddq_s32(u6, rnding);
        u6 = vshlq_s32(u6, v_bit);

        u5 = vsubq_s32(y, x);
        u5 = vaddq_s32(u5, rnding);
        u5 = vshlq_s32(u5, v_bit);

        // stage 5
        addsub_neon(
            u0,
            u7,
            out.offset((0 * 2) as isize).offset(col as isize),
            out.offset((7 * 2) as isize).offset(col as isize),
            &raw const clamp_lo,
            &raw const clamp_hi,
        );
        addsub_neon(
            u1,
            u6,
            out.offset((1 * 2) as isize).offset(col as isize),
            out.offset((6 * 2) as isize).offset(col as isize),
            &raw const clamp_lo,
            &raw const clamp_hi,
        );
        addsub_neon(
            u2,
            u5,
            out.offset((2 * 2) as isize).offset(col as isize),
            out.offset((5 * 2) as isize).offset(col as isize),
            &raw const clamp_lo,
            &raw const clamp_hi,
        );
        addsub_neon(
            u3,
            u4,
            out.offset((3 * 2) as isize).offset(col as isize),
            out.offset((4 * 2) as isize).offset(col as isize),
            &raw const clamp_lo,
            &raw const clamp_hi,
        );
        col += 1;
    }

    if !(do_cols != 0) {
        let log_range_out: i32 = get_log_range_out(bd);
        let clamp_lo_out: int32x4_t = vdupq_n_s32(-(1 << (log_range_out - 1)));
        let clamp_hi_out: int32x4_t = vdupq_n_s32((1 << (log_range_out - 1)) - 1);
        round_shift_8x8(out, out_shift);
        highbd_clamp_s32_neon(out, out, &raw const clamp_lo_out, &raw const clamp_hi_out, 16);
    }
}

unsafe fn iadst8x8_neon(
    mut in_: *mut int32x4_t,
    mut out: *mut int32x4_t,
    mut bit: i32,
    mut do_cols: i32,
    mut bd: i32,
    mut out_shift: i32,
) {
    let mut cospi: *const i32 = cospi_arr(bit);
    let kZero: int32x4_t = vdupq_n_s32(0);
    let log_range: i32 = get_log_range(bd, do_cols);
    let clamp_lo: int32x4_t = vdupq_n_s32(-(1 << (log_range - 1)));
    let clamp_hi: int32x4_t = vdupq_n_s32((1 << (log_range - 1)) - 1);
    let mut u: [int32x4_t; 8] = core::mem::zeroed();
    let mut v: [int32x4_t; 8] = core::mem::zeroed();
    let mut x: int32x4_t = core::mem::zeroed();
    let v_bit: int32x4_t = vdupq_n_s32(-bit);
    let rnding: int32x4_t = vdupq_n_s32(1 << (bit - 1));
    // stage 0-1-2
    // (1)
    u[0] = vmlaq_n_s32(rnding, *in_.add(14), *cospi.add(4));
    u[0] = vmlaq_n_s32(u[0], *in_, *cospi.add(60));
    u[0] = vshlq_s32(u[0], v_bit);

    u[1] = vmlaq_n_s32(rnding, *in_.add(14), *cospi.add(60));
    u[1] = vmlsq_n_s32(u[1], *in_, *cospi.add(4));
    u[1] = vshlq_s32(u[1], v_bit);

    // (2)
    u[2] = vmlaq_n_s32(rnding, *in_.add(10), *cospi.add(20));
    u[2] = vmlaq_n_s32(u[2], *in_.add(4), *cospi.add(44));
    u[2] = vshlq_s32(u[2], v_bit);

    u[3] = vmlaq_n_s32(rnding, *in_.add(10), *cospi.add(44));
    u[3] = vmlsq_n_s32(u[3], *in_.add(4), *cospi.add(20));
    u[3] = vshlq_s32(u[3], v_bit);

    // (3)
    u[4] = vmlaq_n_s32(rnding, *in_.add(6), *cospi.add(36));
    u[4] = vmlaq_n_s32(u[4], *in_.add(8), *cospi.add(28));
    u[4] = vshlq_s32(u[4], v_bit);

    u[5] = vmlaq_n_s32(rnding, *in_.add(6), *cospi.add(28));
    u[5] = vmlsq_n_s32(u[5], *in_.add(8), *cospi.add(36));
    u[5] = vshlq_s32(u[5], v_bit);

    // (4)
    u[6] = vmlaq_n_s32(rnding, *in_.add(2), *cospi.add(52));
    u[6] = vmlaq_n_s32(u[6], *in_.add(12), *cospi.add(12));
    u[6] = vshlq_s32(u[6], v_bit);

    u[7] = vmlaq_n_s32(rnding, *in_.add(2), *cospi.add(12));
    u[7] = vmlsq_n_s32(u[7], *in_.add(12), *cospi.add(52));
    u[7] = vshlq_s32(u[7], v_bit);

    // stage 3
    addsub_neon(
        u[0],
        u[4],
        v.as_mut_ptr(),
        v.as_mut_ptr().add(4),
        &raw const clamp_lo,
        &raw const clamp_hi,
    );
    addsub_neon(
        u[1],
        u[5],
        v.as_mut_ptr().add(1),
        v.as_mut_ptr().add(5),
        &raw const clamp_lo,
        &raw const clamp_hi,
    );
    addsub_neon(
        u[2],
        u[6],
        v.as_mut_ptr().add(2),
        v.as_mut_ptr().add(6),
        &raw const clamp_lo,
        &raw const clamp_hi,
    );
    addsub_neon(
        u[3],
        u[7],
        v.as_mut_ptr().add(3),
        v.as_mut_ptr().add(7),
        &raw const clamp_lo,
        &raw const clamp_hi,
    );

    // stage 4
    u[0] = v[0];
    u[1] = v[1];
    u[2] = v[2];
    u[3] = v[3];

    u[4] = vmlaq_n_s32(rnding, v[4], *cospi.add(16));
    u[4] = vmlaq_n_s32(u[4], v[5], *cospi.add(48));
    u[4] = vshlq_s32(u[4], v_bit);

    u[5] = vmlaq_n_s32(rnding, v[4], *cospi.add(48));
    u[5] = vmlsq_n_s32(u[5], v[5], *cospi.add(16));
    u[5] = vshlq_s32(u[5], v_bit);

    u[6] = vmlaq_n_s32(rnding, v[7], *cospi.add(16));
    u[6] = vmlsq_n_s32(u[6], v[6], *cospi.add(48));
    u[6] = vshlq_s32(u[6], v_bit);

    u[7] = vmlaq_n_s32(rnding, v[7], *cospi.add(48));
    u[7] = vmlaq_n_s32(u[7], v[6], *cospi.add(16));
    u[7] = vshlq_s32(u[7], v_bit);

    // stage 5
    addsub_neon(
        u[0],
        u[2],
        v.as_mut_ptr(),
        v.as_mut_ptr().add(2),
        &raw const clamp_lo,
        &raw const clamp_hi,
    );
    addsub_neon(
        u[1],
        u[3],
        v.as_mut_ptr().add(1),
        v.as_mut_ptr().add(3),
        &raw const clamp_lo,
        &raw const clamp_hi,
    );
    addsub_neon(
        u[4],
        u[6],
        v.as_mut_ptr().add(4),
        v.as_mut_ptr().add(6),
        &raw const clamp_lo,
        &raw const clamp_hi,
    );
    addsub_neon(
        u[5],
        u[7],
        v.as_mut_ptr().add(5),
        v.as_mut_ptr().add(7),
        &raw const clamp_lo,
        &raw const clamp_hi,
    );

    // stage 6
    u[0] = v[0];
    u[1] = v[1];
    u[4] = v[4];
    u[5] = v[5];

    v[0] = vmlaq_n_s32(rnding, v[2], *cospi.add(32));
    x = vmulq_n_s32(v[3], *cospi.add(32));
    u[2] = vaddq_s32(v[0], x);
    u[2] = vshlq_s32(u[2], v_bit);

    u[3] = vsubq_s32(v[0], x);
    u[3] = vshlq_s32(u[3], v_bit);

    v[0] = vmlaq_n_s32(rnding, v[6], *cospi.add(32));
    x = vmulq_n_s32(v[7], *cospi.add(32));
    u[6] = vaddq_s32(v[0], x);
    u[6] = vshlq_s32(u[6], v_bit);

    u[7] = vsubq_s32(v[0], x);
    u[7] = vshlq_s32(u[7], v_bit);

    // stage 7
    if do_cols != 0 {
        *out = u[0];
        *out.add(2) = vsubq_s32(kZero, u[4]);
        *out.add(4) = u[6];
        *out.add(6) = vsubq_s32(kZero, u[2]);
        *out.add(8) = u[3];
        *out.add(10) = vsubq_s32(kZero, u[7]);
        *out.add(12) = u[5];
        *out.add(14) = vsubq_s32(kZero, u[1]);
    } else {
        let log_range_out: i32 = get_log_range_out(bd);
        let clamp_lo_out: int32x4_t = vdupq_n_s32(-(1 << (log_range_out - 1)));
        let clamp_hi_out: int32x4_t = vdupq_n_s32((1 << (log_range_out - 1)) - 1);
        let v_shift: int32x4_t = vdupq_n_s32(-out_shift);
        let mut offset: int32x4_t = vdupq_n_s32((1 << out_shift) >> 1);
        neg_shift_neon(
            u.as_mut_ptr(),
            u.as_mut_ptr().add(4),
            out,
            out.add(2),
            &raw const clamp_lo_out,
            &raw const clamp_hi_out,
            &raw const v_shift,
            &raw mut offset,
        );
        neg_shift_neon(
            u.as_mut_ptr().add(6),
            u.as_mut_ptr().add(2),
            out.add(4),
            out.add(6),
            &raw const clamp_lo_out,
            &raw const clamp_hi_out,
            &raw const v_shift,
            &raw mut offset,
        );
        neg_shift_neon(
            u.as_mut_ptr().add(3),
            u.as_mut_ptr().add(7),
            out.add(8),
            out.add(10),
            &raw const clamp_lo_out,
            &raw const clamp_hi_out,
            &raw const v_shift,
            &raw mut offset,
        );
        neg_shift_neon(
            u.as_mut_ptr().add(5),
            u.as_mut_ptr().add(1),
            out.add(12),
            out.add(14),
            &raw const clamp_lo_out,
            &raw const clamp_hi_out,
            &raw const v_shift,
            &raw mut offset,
        );
    }

    // Odd 8 points: 1, 3, ..., 15
    // stage 0
    // stage 1
    // stage 2
    // (1)
    u[0] = vmlaq_n_s32(rnding, *in_.add(15), *cospi.add(4));
    u[0] = vmlaq_n_s32(u[0], *in_.add(1), *cospi.add(60));
    u[0] = vshlq_s32(u[0], v_bit);

    u[1] = vmlaq_n_s32(rnding, *in_.add(15), *cospi.add(60));
    u[1] = vmlsq_n_s32(u[1], *in_.add(1), *cospi.add(4));
    u[1] = vshlq_s32(u[1], v_bit);

    // (2)
    u[2] = vmlaq_n_s32(rnding, *in_.add(11), *cospi.add(20));
    u[2] = vmlaq_n_s32(u[2], *in_.add(5), *cospi.add(44));
    u[2] = vshlq_s32(u[2], v_bit);

    u[3] = vmlaq_n_s32(rnding, *in_.add(11), *cospi.add(44));
    u[3] = vmlsq_n_s32(u[3], *in_.add(5), *cospi.add(20));
    u[3] = vshlq_s32(u[3], v_bit);

    // (3)
    u[4] = vmlaq_n_s32(rnding, *in_.add(7), *cospi.add(36));
    u[4] = vmlaq_n_s32(u[4], *in_.add(9), *cospi.add(28));
    u[4] = vshlq_s32(u[4], v_bit);

    u[5] = vmlaq_n_s32(rnding, *in_.add(7), *cospi.add(28));
    u[5] = vmlsq_n_s32(u[5], *in_.add(9), *cospi.add(36));
    u[5] = vshlq_s32(u[5], v_bit);

    // (4)
    u[6] = vmlaq_n_s32(rnding, *in_.add(3), *cospi.add(52));
    u[6] = vmlaq_n_s32(u[6], *in_.add(13), *cospi.add(12));
    u[6] = vshlq_s32(u[6], v_bit);

    u[7] = vmlaq_n_s32(rnding, *in_.add(3), *cospi.add(12));
    u[7] = vmlsq_n_s32(u[7], *in_.add(13), *cospi.add(52));
    u[7] = vshlq_s32(u[7], v_bit);

    // stage 3
    addsub_neon(
        u[0],
        u[4],
        v.as_mut_ptr(),
        v.as_mut_ptr().add(4),
        &raw const clamp_lo,
        &raw const clamp_hi,
    );
    addsub_neon(
        u[1],
        u[5],
        v.as_mut_ptr().add(1),
        v.as_mut_ptr().add(5),
        &raw const clamp_lo,
        &raw const clamp_hi,
    );
    addsub_neon(
        u[2],
        u[6],
        v.as_mut_ptr().add(2),
        v.as_mut_ptr().add(6),
        &raw const clamp_lo,
        &raw const clamp_hi,
    );
    addsub_neon(
        u[3],
        u[7],
        v.as_mut_ptr().add(3),
        v.as_mut_ptr().add(7),
        &raw const clamp_lo,
        &raw const clamp_hi,
    );

    // stage 4
    u[0] = v[0];
    u[1] = v[1];
    u[2] = v[2];
    u[3] = v[3];

    u[4] = vmlaq_n_s32(rnding, v[4], *cospi.add(16));
    u[4] = vmlaq_n_s32(u[4], v[5], *cospi.add(48));
    u[4] = vshlq_s32(u[4], v_bit);

    u[5] = vmlaq_n_s32(rnding, v[4], *cospi.add(48));
    u[5] = vmlsq_n_s32(u[5], v[5], *cospi.add(16));
    u[5] = vshlq_s32(u[5], v_bit);

    u[6] = vmlaq_n_s32(rnding, v[7], *cospi.add(16));
    u[6] = vmlsq_n_s32(u[6], v[6], *cospi.add(48));
    u[6] = vshlq_s32(u[6], v_bit);

    u[7] = vmlaq_n_s32(rnding, v[6], *cospi.add(16));
    u[7] = vmlaq_n_s32(u[7], v[7], *cospi.add(48));
    u[7] = vshlq_s32(u[7], v_bit);

    // stage 5
    addsub_neon(
        u[0],
        u[2],
        v.as_mut_ptr(),
        v.as_mut_ptr().add(2),
        &raw const clamp_lo,
        &raw const clamp_hi,
    );
    addsub_neon(
        u[1],
        u[3],
        v.as_mut_ptr().add(1),
        v.as_mut_ptr().add(3),
        &raw const clamp_lo,
        &raw const clamp_hi,
    );
    addsub_neon(
        u[4],
        u[6],
        v.as_mut_ptr().add(4),
        v.as_mut_ptr().add(6),
        &raw const clamp_lo,
        &raw const clamp_hi,
    );
    addsub_neon(
        u[5],
        u[7],
        v.as_mut_ptr().add(5),
        v.as_mut_ptr().add(7),
        &raw const clamp_lo,
        &raw const clamp_hi,
    );

    // stage 6
    u[0] = v[0];
    u[1] = v[1];
    u[4] = v[4];
    u[5] = v[5];

    v[0] = vmlaq_n_s32(rnding, v[2], *cospi.add(32));
    x = vmulq_n_s32(v[3], *cospi.add(32));
    u[2] = vaddq_s32(v[0], x);
    u[2] = vshlq_s32(u[2], v_bit);

    u[3] = vsubq_s32(v[0], x);
    u[3] = vshlq_s32(u[3], v_bit);

    v[0] = vmlaq_n_s32(rnding, v[6], *cospi.add(32));
    x = vmulq_n_s32(v[7], *cospi.add(32));
    u[6] = vaddq_s32(v[0], x);
    u[6] = vshlq_s32(u[6], v_bit);

    u[7] = vsubq_s32(v[0], x);
    u[7] = vshlq_s32(u[7], v_bit);

    // stage 7
    if do_cols != 0 {
        *out.add(1) = u[0];
        *out.add(3) = vsubq_s32(kZero, u[4]);
        *out.add(5) = u[6];
        *out.add(7) = vsubq_s32(kZero, u[2]);
        *out.add(9) = u[3];
        *out.add(11) = vsubq_s32(kZero, u[7]);
        *out.add(13) = u[5];
        *out.add(15) = vsubq_s32(kZero, u[1]);
    } else {
        let log_range_out: i32 = get_log_range_out(bd);
        let clamp_lo_out: int32x4_t = vdupq_n_s32(-(1 << (log_range_out - 1)));
        let clamp_hi_out: int32x4_t = vdupq_n_s32((1 << (log_range_out - 1)) - 1);
        let v_shift: int32x4_t = vdupq_n_s32(-out_shift);
        let mut offset: int32x4_t = vdupq_n_s32((1 << out_shift) >> 1);
        neg_shift_neon(
            u.as_mut_ptr(),
            u.as_mut_ptr().add(4),
            out.add(1),
            out.add(3),
            &raw const clamp_lo_out,
            &raw const clamp_hi_out,
            &raw const v_shift,
            &raw mut offset,
        );
        neg_shift_neon(
            u.as_mut_ptr().add(6),
            u.as_mut_ptr().add(2),
            out.add(5),
            out.add(7),
            &raw const clamp_lo_out,
            &raw const clamp_hi_out,
            &raw const v_shift,
            &raw mut offset,
        );
        neg_shift_neon(
            u.as_mut_ptr().add(3),
            u.as_mut_ptr().add(7),
            out.add(9),
            out.add(11),
            &raw const clamp_lo_out,
            &raw const clamp_hi_out,
            &raw const v_shift,
            &raw mut offset,
        );
        neg_shift_neon(
            u.as_mut_ptr().add(5),
            u.as_mut_ptr().add(1),
            out.add(13),
            out.add(15),
            &raw const clamp_lo_out,
            &raw const clamp_hi_out,
            &raw const v_shift,
            &raw mut offset,
        );
    }
}

unsafe fn iidentity8_neon(
    mut in_: *mut int32x4_t,
    mut out: *mut int32x4_t,
    mut bit: i32,
    mut do_cols: i32,
    mut bd: i32,
    mut out_shift: i32,
) {
    let _ = bit;
    *out = vaddq_s32(*in_, *in_);
    *out.add(1) = vaddq_s32(*in_.add(1), *in_.add(1));
    *out.add(2) = vaddq_s32(*in_.add(2), *in_.add(2));
    *out.add(3) = vaddq_s32(*in_.add(3), *in_.add(3));
    *out.add(4) = vaddq_s32(*in_.add(4), *in_.add(4));
    *out.add(5) = vaddq_s32(*in_.add(5), *in_.add(5));
    *out.add(6) = vaddq_s32(*in_.add(6), *in_.add(6));
    *out.add(7) = vaddq_s32(*in_.add(7), *in_.add(7));

    if !(do_cols != 0) {
        let log_range: i32 = get_log_range_out(bd);
        let clamp_lo: int32x4_t = vdupq_n_s32(-(1 << (log_range - 1)));
        let clamp_hi: int32x4_t = vdupq_n_s32((1 << (log_range - 1)) - 1);
        round_shift_4x4(out, out_shift);
        round_shift_4x4(out.add(4), out_shift);
        highbd_clamp_s32_neon(out, out, &raw const clamp_lo, &raw const clamp_hi, 8);
    }
}

unsafe fn get_recon_8x8(
    mut pred: uint16x8_t,
    mut res_lo: int32x4_t,
    mut res_hi: int32x4_t,
    mut fliplr: i32,
    mut bd: i32,
) -> uint16x8_t {
    let mut x: uint16x8x2_t = core::mem::zeroed();

    if fliplr != 0 {
        res_lo = vrev64q_s32(res_lo);
        res_lo = vextq_s32::<2>(res_lo, res_lo);
        res_hi = vrev64q_s32(res_hi);
        res_hi = vextq_s32::<2>(res_hi, res_hi);
        x.0 = vreinterpretq_u16_s32(vaddw_s16(res_hi, vreinterpret_s16_u16(vget_low_u16(pred))));
        x.1 = vreinterpretq_u16_s32(vaddw_s16(res_lo, vreinterpret_s16_u16(vget_high_u16(pred))));
    } else {
        x.0 = vreinterpretq_u16_s32(vaddw_s16(res_lo, vreinterpret_s16_u16(vget_low_u16(pred))));
        x.1 = vreinterpretq_u16_s32(vaddw_s16(res_hi, vreinterpret_s16_u16(vget_high_u16(pred))));
    }

    let mut x2: uint16x8_t = vcombine_u16(
        vqmovn_u32(vreinterpretq_u32_u16(x.0)),
        vqmovn_u32(vreinterpretq_u32_u16(x.1)),
    );
    let vmin: uint16x8_t = vdupq_n_u16(0);
    let vmax: uint16x8_t = vdupq_n_u16(((1 << bd) - 1) as u16);
    return highbd_clamp_u16(&raw mut x2, &raw const vmin, &raw const vmax);
}

unsafe fn write_buffer_8x8(
    mut in_: *mut int32x4_t,
    mut output: *mut u16,
    mut stride: i32,
    mut fliplr: i32,
    mut flipud: i32,
    mut shift: i32,
    mut bd: i32,
) {
    let mut u0: uint16x8_t = core::mem::zeroed();
    let mut u1: uint16x8_t = core::mem::zeroed();
    let mut u2: uint16x8_t = core::mem::zeroed();
    let mut u3: uint16x8_t = core::mem::zeroed();
    let mut u4: uint16x8_t = core::mem::zeroed();
    let mut u5: uint16x8_t = core::mem::zeroed();
    let mut u6: uint16x8_t = core::mem::zeroed();
    let mut u7: uint16x8_t = core::mem::zeroed();
    let mut v0: uint16x8_t = core::mem::zeroed();
    let mut v1: uint16x8_t = core::mem::zeroed();
    let mut v2: uint16x8_t = core::mem::zeroed();
    let mut v3: uint16x8_t = core::mem::zeroed();
    let mut v4: uint16x8_t = core::mem::zeroed();
    let mut v5: uint16x8_t = core::mem::zeroed();
    let mut v6: uint16x8_t = core::mem::zeroed();
    let mut v7: uint16x8_t = core::mem::zeroed();
    round_shift_8x8(in_, shift);

    v0 = vld1q_u16(output.offset((0 * stride) as isize));
    v1 = vld1q_u16(output.offset((1 * stride) as isize));
    v2 = vld1q_u16(output.offset((2 * stride) as isize));
    v3 = vld1q_u16(output.offset((3 * stride) as isize));
    v4 = vld1q_u16(output.offset((4 * stride) as isize));
    v5 = vld1q_u16(output.offset((5 * stride) as isize));
    v6 = vld1q_u16(output.offset((6 * stride) as isize));
    v7 = vld1q_u16(output.offset((7 * stride) as isize));

    if flipud != 0 {
        u0 = get_recon_8x8(v0, *in_.add(14), *in_.add(15), fliplr, bd);
        u1 = get_recon_8x8(v1, *in_.add(12), *in_.add(13), fliplr, bd);
        u2 = get_recon_8x8(v2, *in_.add(10), *in_.add(11), fliplr, bd);
        u3 = get_recon_8x8(v3, *in_.add(8), *in_.add(9), fliplr, bd);
        u4 = get_recon_8x8(v4, *in_.add(6), *in_.add(7), fliplr, bd);
        u5 = get_recon_8x8(v5, *in_.add(4), *in_.add(5), fliplr, bd);
        u6 = get_recon_8x8(v6, *in_.add(2), *in_.add(3), fliplr, bd);
        u7 = get_recon_8x8(v7, *in_, *in_.add(1), fliplr, bd);
    } else {
        u0 = get_recon_8x8(v0, *in_, *in_.add(1), fliplr, bd);
        u1 = get_recon_8x8(v1, *in_.add(2), *in_.add(3), fliplr, bd);
        u2 = get_recon_8x8(v2, *in_.add(4), *in_.add(5), fliplr, bd);
        u3 = get_recon_8x8(v3, *in_.add(6), *in_.add(7), fliplr, bd);
        u4 = get_recon_8x8(v4, *in_.add(8), *in_.add(9), fliplr, bd);
        u5 = get_recon_8x8(v5, *in_.add(10), *in_.add(11), fliplr, bd);
        u6 = get_recon_8x8(v6, *in_.add(12), *in_.add(13), fliplr, bd);
        u7 = get_recon_8x8(v7, *in_.add(14), *in_.add(15), fliplr, bd);
    }

    vst1q_u16(output.offset((0 * stride) as isize), u0);
    vst1q_u16(output.offset((1 * stride) as isize), u1);
    vst1q_u16(output.offset((2 * stride) as isize), u2);
    vst1q_u16(output.offset((3 * stride) as isize), u3);
    vst1q_u16(output.offset((4 * stride) as isize), u4);
    vst1q_u16(output.offset((5 * stride) as isize), u5);
    vst1q_u16(output.offset((6 * stride) as isize), u6);
    vst1q_u16(output.offset((7 * stride) as isize), u7);
}

#[no_mangle]
pub unsafe extern "C-unwind" fn av1_inv_txfm2d_add_8x8_neon(
    mut input: *const i32,
    mut output: *mut u16,
    mut stride: i32,
    mut tx_type: TX_TYPE,
    mut bd: i32,
) {
    let mut in_: [int32x4_t; 16] = core::mem::zeroed();
    let mut out: [int32x4_t; 16] = core::mem::zeroed();
    let mut shift: *const i8 = av1_inv_txfm_shift_ls[TX_8X8 as usize];

    match tx_type as i32 {
        DCT_DCT => {
            load_buffer_8x8(input, in_.as_mut_ptr());
            idct8x8_neon(in_.as_mut_ptr(), out.as_mut_ptr(), INV_COS_BIT, 0, bd, -(*shift as i32));
            transpose_8x8(out.as_mut_ptr(), in_.as_mut_ptr());
            idct8x8_neon(in_.as_mut_ptr(), out.as_mut_ptr(), INV_COS_BIT, 1, bd, 0);
            write_buffer_8x8(out.as_mut_ptr(), output, stride, 0, 0, -(*shift.add(1) as i32), bd);
        }
        DCT_ADST => {
            load_buffer_8x8(input, in_.as_mut_ptr());
            iadst8x8_neon(in_.as_mut_ptr(), out.as_mut_ptr(), INV_COS_BIT, 0, bd, -(*shift as i32));
            transpose_8x8(out.as_mut_ptr(), in_.as_mut_ptr());
            idct8x8_neon(in_.as_mut_ptr(), out.as_mut_ptr(), INV_COS_BIT, 1, bd, 0);
            write_buffer_8x8(out.as_mut_ptr(), output, stride, 0, 0, -(*shift.add(1) as i32), bd);
        }
        ADST_DCT => {
            load_buffer_8x8(input, in_.as_mut_ptr());
            idct8x8_neon(in_.as_mut_ptr(), out.as_mut_ptr(), INV_COS_BIT, 0, bd, -(*shift as i32));
            transpose_8x8(out.as_mut_ptr(), in_.as_mut_ptr());
            iadst8x8_neon(in_.as_mut_ptr(), out.as_mut_ptr(), INV_COS_BIT, 1, bd, 0);
            write_buffer_8x8(out.as_mut_ptr(), output, stride, 0, 0, -(*shift.add(1) as i32), bd);
        }
        ADST_ADST => {
            load_buffer_8x8(input, in_.as_mut_ptr());
            iadst8x8_neon(in_.as_mut_ptr(), out.as_mut_ptr(), INV_COS_BIT, 0, bd, -(*shift as i32));
            transpose_8x8(out.as_mut_ptr(), in_.as_mut_ptr());
            iadst8x8_neon(in_.as_mut_ptr(), out.as_mut_ptr(), INV_COS_BIT, 1, bd, 0);
            write_buffer_8x8(out.as_mut_ptr(), output, stride, 0, 0, -(*shift.add(1) as i32), bd);
        }
        FLIPADST_DCT => {
            load_buffer_8x8(input, in_.as_mut_ptr());
            idct8x8_neon(in_.as_mut_ptr(), out.as_mut_ptr(), INV_COS_BIT, 0, bd, -(*shift as i32));
            transpose_8x8(out.as_mut_ptr(), in_.as_mut_ptr());
            iadst8x8_neon(in_.as_mut_ptr(), out.as_mut_ptr(), INV_COS_BIT, 1, bd, 0);
            write_buffer_8x8(out.as_mut_ptr(), output, stride, 0, 1, -(*shift.add(1) as i32), bd);
        }
        DCT_FLIPADST => {
            load_buffer_8x8(input, in_.as_mut_ptr());
            iadst8x8_neon(in_.as_mut_ptr(), out.as_mut_ptr(), INV_COS_BIT, 0, bd, -(*shift as i32));
            transpose_8x8(out.as_mut_ptr(), in_.as_mut_ptr());
            idct8x8_neon(in_.as_mut_ptr(), out.as_mut_ptr(), INV_COS_BIT, 1, bd, 0);
            write_buffer_8x8(out.as_mut_ptr(), output, stride, 1, 0, -(*shift.add(1) as i32), bd);
        }
        ADST_FLIPADST => {
            load_buffer_8x8(input, in_.as_mut_ptr());
            iadst8x8_neon(in_.as_mut_ptr(), out.as_mut_ptr(), INV_COS_BIT, 0, bd, -(*shift as i32));
            transpose_8x8(out.as_mut_ptr(), in_.as_mut_ptr());
            iadst8x8_neon(in_.as_mut_ptr(), out.as_mut_ptr(), INV_COS_BIT, 1, bd, 0);
            write_buffer_8x8(out.as_mut_ptr(), output, stride, 1, 0, -(*shift.add(1) as i32), bd);
        }
        FLIPADST_FLIPADST => {
            load_buffer_8x8(input, in_.as_mut_ptr());
            iadst8x8_neon(in_.as_mut_ptr(), out.as_mut_ptr(), INV_COS_BIT, 0, bd, -(*shift as i32));
            transpose_8x8(out.as_mut_ptr(), in_.as_mut_ptr());
            iadst8x8_neon(in_.as_mut_ptr(), out.as_mut_ptr(), INV_COS_BIT, 1, bd, 0);
            write_buffer_8x8(out.as_mut_ptr(), output, stride, 1, 1, -(*shift.add(1) as i32), bd);
        }
        FLIPADST_ADST => {
            load_buffer_8x8(input, in_.as_mut_ptr());
            iadst8x8_neon(in_.as_mut_ptr(), out.as_mut_ptr(), INV_COS_BIT, 0, bd, -(*shift as i32));
            transpose_8x8(out.as_mut_ptr(), in_.as_mut_ptr());
            iadst8x8_neon(in_.as_mut_ptr(), out.as_mut_ptr(), INV_COS_BIT, 1, bd, 0);
            write_buffer_8x8(out.as_mut_ptr(), output, stride, 0, 1, -(*shift.add(1) as i32), bd);
        }
        _ => {
            debug_assert!(false);
        }
    }
}

unsafe fn idct8x8_low1_neon(
    mut in_: *mut int32x4_t,
    mut out: *mut int32x4_t,
    mut bit: i32,
    mut do_cols: i32,
    mut bd: i32,
    mut out_shift: i32,
) {
    let mut cospi: *const i32 = cospi_arr(bit);
    let log_range: i32 = get_log_range(bd, do_cols);
    let mut clamp_lo: int32x4_t = vdupq_n_s32(-(1 << (log_range - 1)));
    let mut clamp_hi: int32x4_t = vdupq_n_s32((1 << (log_range - 1)) - 1);
    let mut x: int32x4_t = core::mem::zeroed();
    let v_bit: int32x4_t = vdupq_n_s32(-bit);
    let rnding: int32x4_t = vdupq_n_s32(1 << (bit - 1));
    // stage 0-1-2-3
    x = vmulq_n_s32(*in_, *cospi.add(32));
    x = vaddq_s32(vshlq_s32(x, v_bit), rnding);

    // stage 4-5
    if !(do_cols != 0) {
        let log_range_out: i32 = get_log_range_out(bd);
        clamp_lo = vdupq_n_s32(-(1 << (log_range_out - 1)));
        clamp_hi = vdupq_n_s32((1 << (log_range_out - 1)) - 1);

        let mut offset: int32x4_t = vdupq_n_s32((1 << out_shift) >> 1);
        x = vaddq_s32(x, offset);
        x = vshlq_s32(x, vdupq_n_s32(-out_shift));
    }

    x = vmaxq_s32(x, clamp_lo);
    x = vminq_s32(x, clamp_hi);
    *out = x;
    *out.add(1) = x;
    *out.add(2) = x;
    *out.add(3) = x;
    *out.add(4) = x;
    *out.add(5) = x;
    *out.add(6) = x;
    *out.add(7) = x;
}

unsafe fn idct8x8_new_neon(
    mut in_: *mut int32x4_t,
    mut out: *mut int32x4_t,
    mut bit: i32,
    mut do_cols: i32,
    mut bd: i32,
    mut out_shift: i32,
) {
    let mut cospi: *const i32 = cospi_arr(bit);
    let log_range: i32 = get_log_range(bd, do_cols);
    let clamp_lo: int32x4_t = vdupq_n_s32(-(1 << (log_range - 1)));
    let clamp_hi: int32x4_t = vdupq_n_s32((1 << (log_range - 1)) - 1);
    let mut u0: int32x4_t = core::mem::zeroed();
    let mut u1: int32x4_t = core::mem::zeroed();
    let mut u2: int32x4_t = core::mem::zeroed();
    let mut u3: int32x4_t = core::mem::zeroed();
    let mut u4: int32x4_t = core::mem::zeroed();
    let mut u5: int32x4_t = core::mem::zeroed();
    let mut u6: int32x4_t = core::mem::zeroed();
    let mut u7: int32x4_t = core::mem::zeroed();
    let mut v0: int32x4_t = core::mem::zeroed();
    let mut v1: int32x4_t = core::mem::zeroed();
    let mut v2: int32x4_t = core::mem::zeroed();
    let mut v3: int32x4_t = core::mem::zeroed();
    let mut v4: int32x4_t = core::mem::zeroed();
    let mut v5: int32x4_t = core::mem::zeroed();
    let mut v6: int32x4_t = core::mem::zeroed();
    let mut v7: int32x4_t = core::mem::zeroed();
    let mut x: int32x4_t = core::mem::zeroed();
    let mut y: int32x4_t = core::mem::zeroed();
    let v_bit: int32x4_t = vdupq_n_s32(-bit);
    let rnding: int32x4_t = vdupq_n_s32(1 << (bit - 1));

    // stage 0
    // stage 1
    // stage 2
    u0 = *in_;
    u1 = *in_.add(4);
    u2 = *in_.add(2);
    u3 = *in_.add(6);

    x = vmlaq_n_s32(rnding, *in_.add(1), *cospi.add(56));
    u4 = vmlaq_n_s32(x, *in_.add(7), -*cospi.add(8));
    u4 = vshlq_s32(u4, v_bit);

    x = vmlaq_n_s32(rnding, *in_.add(1), *cospi.add(8));
    u7 = vmlaq_n_s32(x, *in_.add(7), *cospi.add(56));
    u7 = vshlq_s32(u7, v_bit);

    x = vmlaq_n_s32(rnding, *in_.add(5), *cospi.add(24));
    u5 = vmlaq_n_s32(x, *in_.add(3), -*cospi.add(40));
    u5 = vshlq_s32(u5, v_bit);

    x = vmlaq_n_s32(rnding, *in_.add(5), *cospi.add(40));
    u6 = vmlaq_n_s32(x, *in_.add(3), *cospi.add(24));
    u6 = vshlq_s32(u6, v_bit);

    // stage 3
    x = vmlaq_n_s32(rnding, u0, *cospi.add(32));
    y = vmulq_n_s32(u1, *cospi.add(32));
    v0 = vaddq_s32(x, y);
    v0 = vshlq_s32(v0, v_bit);

    v1 = vsubq_s32(x, y);
    v1 = vshlq_s32(v1, v_bit);

    x = vmlaq_n_s32(rnding, u2, *cospi.add(48));
    v2 = vmlaq_n_s32(x, u3, -*cospi.add(16));
    v2 = vshlq_s32(v2, v_bit);

    x = vmlaq_n_s32(rnding, u2, *cospi.add(16));
    v3 = vmlaq_n_s32(x, u3, *cospi.add(48));
    v3 = vshlq_s32(v3, v_bit);

    addsub_neon(u4, u5, &raw mut v4, &raw mut v5, &raw const clamp_lo, &raw const clamp_hi);
    addsub_neon(u7, u6, &raw mut v7, &raw mut v6, &raw const clamp_lo, &raw const clamp_hi);

    // stage 4
    addsub_neon(v0, v3, &raw mut u0, &raw mut u3, &raw const clamp_lo, &raw const clamp_hi);
    addsub_neon(v1, v2, &raw mut u1, &raw mut u2, &raw const clamp_lo, &raw const clamp_hi);
    u4 = v4;
    u7 = v7;

    x = vmulq_n_s32(v5, *cospi.add(32));
    y = vmlaq_n_s32(rnding, v6, *cospi.add(32));
    u6 = vaddq_s32(y, x);
    u6 = vshlq_s32(u6, v_bit);

    u5 = vsubq_s32(y, x);
    u5 = vshlq_s32(u5, v_bit);

    // stage 5
    addsub_neon(u0, u7, out, out.add(7), &raw const clamp_lo, &raw const clamp_hi);
    addsub_neon(u1, u6, out.add(1), out.add(6), &raw const clamp_lo, &raw const clamp_hi);
    addsub_neon(u2, u5, out.add(2), out.add(5), &raw const clamp_lo, &raw const clamp_hi);
    addsub_neon(u3, u4, out.add(3), out.add(4), &raw const clamp_lo, &raw const clamp_hi);

    if !(do_cols != 0) {
        let log_range_out: i32 = get_log_range_out(bd);
        let clamp_lo_out: int32x4_t = vdupq_n_s32(-(1 << (log_range_out - 1)));
        let clamp_hi_out: int32x4_t = vdupq_n_s32((1 << (log_range_out - 1)) - 1);
        round_shift_4x4(out, out_shift);
        round_shift_4x4(out.add(4), out_shift);
        highbd_clamp_s32_neon(out, out, &raw const clamp_lo_out, &raw const clamp_hi_out, 8);
    }
}

unsafe fn iadst8x8_low1_neon(
    mut in_: *mut int32x4_t,
    mut out: *mut int32x4_t,
    mut bit: i32,
    mut do_cols: i32,
    mut bd: i32,
    mut out_shift: i32,
) {
    let mut cospi: *const i32 = cospi_arr(bit);
    let mut u: [int32x4_t; 8] = core::mem::zeroed();
    let mut x: int32x4_t = core::mem::zeroed();
    let v_bit: int32x4_t = vdupq_n_s32(-bit);
    let rnding: int32x4_t = vdupq_n_s32(1 << (bit - 1));

    // stage 0-2
    u[0] = vmlaq_n_s32(rnding, *in_, *cospi.add(60));
    u[0] = vshlq_s32(u[0], v_bit);

    u[1] = vmlaq_n_s32(rnding, *in_, *cospi.add(4));
    u[1] = vshlq_s32(vnegq_s32(u[1]), v_bit);

    // stage 3-4
    let mut temp1: int32x4_t = core::mem::zeroed();
    let mut temp2: int32x4_t = core::mem::zeroed();
    temp1 = vmlaq_n_s32(rnding, u[0], *cospi.add(16));
    temp1 = vmlaq_n_s32(temp1, u[1], *cospi.add(48));
    temp1 = vshlq_s32(temp1, v_bit);
    u[4] = temp1;

    temp2 = vmlaq_n_s32(rnding, u[0], *cospi.add(48));
    u[5] = vmlsq_n_s32(temp2, u[1], *cospi.add(16));
    u[5] = vshlq_s32(u[5], v_bit);

    // stage 5-6
    temp1 = vmlaq_n_s32(rnding, u[0], *cospi.add(32));
    x = vmulq_n_s32(u[1], *cospi.add(32));
    u[2] = vaddq_s32(temp1, x);
    u[2] = vshlq_s32(u[2], v_bit);

    u[3] = vsubq_s32(temp1, x);
    u[3] = vshlq_s32(u[3], v_bit);

    temp1 = vmlaq_n_s32(rnding, u[4], *cospi.add(32));
    x = vmulq_n_s32(u[5], *cospi.add(32));
    u[6] = vaddq_s32(temp1, x);
    u[6] = vshlq_s32(u[6], v_bit);

    u[7] = vsubq_s32(temp1, x);
    u[7] = vshlq_s32(u[7], v_bit);

    // stage 7
    if do_cols != 0 {
        *out = u[0];
        *out.add(1) = vnegq_s32(u[4]);
        *out.add(2) = u[6];
        *out.add(3) = vnegq_s32(u[2]);
        *out.add(4) = u[3];
        *out.add(5) = vnegq_s32(u[7]);
        *out.add(6) = u[5];
        *out.add(7) = vnegq_s32(u[1]);
    } else {
        let log_range_out: i32 = get_log_range_out(bd);
        let clamp_lo_out: int32x4_t = vdupq_n_s32(-(1 << (log_range_out - 1)));
        let clamp_hi_out: int32x4_t = vdupq_n_s32((1 << (log_range_out - 1)) - 1);
        let v_shift: int32x4_t = vdupq_n_s32(-out_shift);
        let mut offset: int32x4_t = vdupq_n_s32((1 << out_shift) >> 1);
        neg_shift_neon(
            u.as_mut_ptr(),
            u.as_mut_ptr().add(4),
            out,
            out.add(1),
            &raw const clamp_lo_out,
            &raw const clamp_hi_out,
            &raw const v_shift,
            &raw mut offset,
        );
        neg_shift_neon(
            u.as_mut_ptr().add(6),
            u.as_mut_ptr().add(2),
            out.add(2),
            out.add(3),
            &raw const clamp_lo_out,
            &raw const clamp_hi_out,
            &raw const v_shift,
            &raw mut offset,
        );
        neg_shift_neon(
            u.as_mut_ptr().add(3),
            u.as_mut_ptr().add(7),
            out.add(4),
            out.add(5),
            &raw const clamp_lo_out,
            &raw const clamp_hi_out,
            &raw const v_shift,
            &raw mut offset,
        );
        neg_shift_neon(
            u.as_mut_ptr().add(5),
            u.as_mut_ptr().add(1),
            out.add(6),
            out.add(7),
            &raw const clamp_lo_out,
            &raw const clamp_hi_out,
            &raw const v_shift,
            &raw mut offset,
        );
    }
}

unsafe fn iadst8x8_new_neon(
    mut in_: *mut int32x4_t,
    mut out: *mut int32x4_t,
    mut bit: i32,
    mut do_cols: i32,
    mut bd: i32,
    mut out_shift: i32,
) {
    let mut cospi: *const i32 = cospi_arr(bit);
    // const int32x4_t rnding = vdupq_n_s32(1 << (bit - 1));
    let log_range: i32 = get_log_range(bd, do_cols);
    let clamp_lo: int32x4_t = vdupq_n_s32(-(1 << (log_range - 1)));
    let clamp_hi: int32x4_t = vdupq_n_s32((1 << (log_range - 1)) - 1);
    let mut u: [int32x4_t; 8] = core::mem::zeroed();
    let mut v: [int32x4_t; 8] = core::mem::zeroed();
    let mut x: int32x4_t = core::mem::zeroed();
    let v_bit: int32x4_t = vdupq_n_s32(-bit);
    let rnding: int32x4_t = vdupq_n_s32(1 << (bit - 1));

    // stage 0-2
    u[0] = vmlaq_n_s32(rnding, *in_.add(7), *cospi.add(4));
    u[0] = vmlaq_n_s32(u[0], *in_, *cospi.add(60));
    u[0] = vshlq_s32(u[0], v_bit);

    u[1] = vmlaq_n_s32(rnding, *in_.add(7), *cospi.add(60));
    u[1] = vmlsq_n_s32(u[1], *in_, *cospi.add(4));
    u[1] = vshlq_s32(u[1], v_bit);

    // (2)
    u[2] = vmlaq_n_s32(rnding, *in_.add(5), *cospi.add(20));
    u[2] = vmlaq_n_s32(u[2], *in_.add(2), *cospi.add(44));
    u[2] = vshlq_s32(u[2], v_bit);

    u[3] = vmlaq_n_s32(rnding, *in_.add(5), *cospi.add(44));
    u[3] = vmlsq_n_s32(u[3], *in_.add(2), *cospi.add(20));
    u[3] = vshlq_s32(u[3], v_bit);

    // (3)
    u[4] = vmlaq_n_s32(rnding, *in_.add(3), *cospi.add(36));
    u[4] = vmlaq_n_s32(u[4], *in_.add(4), *cospi.add(28));
    u[4] = vshlq_s32(u[4], v_bit);

    u[5] = vmlaq_n_s32(rnding, *in_.add(3), *cospi.add(28));
    u[5] = vmlsq_n_s32(u[5], *in_.add(4), *cospi.add(36));
    u[5] = vshlq_s32(u[5], v_bit);

    // (4)
    u[6] = vmulq_n_s32(*in_.add(1), *cospi.add(52));
    u[6] = vmlaq_n_s32(u[6], *in_.add(6), *cospi.add(12));
    u[6] = vaddq_s32(u[6], rnding);
    u[6] = vshlq_s32(u[6], v_bit);

    u[7] = vmulq_n_s32(*in_.add(1), *cospi.add(12));
    u[7] = vmlsq_n_s32(u[7], *in_.add(6), *cospi.add(52));
    u[7] = vaddq_s32(u[7], rnding);
    u[7] = vshlq_s32(u[7], v_bit);

    // stage 3
    addsub_neon(
        u[0],
        u[4],
        v.as_mut_ptr(),
        v.as_mut_ptr().add(4),
        &raw const clamp_lo,
        &raw const clamp_hi,
    );
    addsub_neon(
        u[1],
        u[5],
        v.as_mut_ptr().add(1),
        v.as_mut_ptr().add(5),
        &raw const clamp_lo,
        &raw const clamp_hi,
    );
    addsub_neon(
        u[2],
        u[6],
        v.as_mut_ptr().add(2),
        v.as_mut_ptr().add(6),
        &raw const clamp_lo,
        &raw const clamp_hi,
    );
    addsub_neon(
        u[3],
        u[7],
        v.as_mut_ptr().add(3),
        v.as_mut_ptr().add(7),
        &raw const clamp_lo,
        &raw const clamp_hi,
    );

    // stage 4
    u[0] = v[0];
    u[1] = v[1];
    u[2] = v[2];
    u[3] = v[3];

    u[4] = vmlaq_n_s32(rnding, v[4], *cospi.add(16));
    u[4] = vmlaq_n_s32(u[4], v[5], *cospi.add(48));
    u[4] = vshlq_s32(u[4], v_bit);

    u[5] = vmlaq_n_s32(rnding, v[4], *cospi.add(48));
    u[5] = vmlsq_n_s32(u[5], v[5], *cospi.add(16));
    u[5] = vshlq_s32(u[5], v_bit);

    u[6] = vmlsq_n_s32(rnding, v[6], *cospi.add(48));
    u[6] = vmlaq_n_s32(u[6], v[7], *cospi.add(16));
    u[6] = vshlq_s32(u[6], v_bit);

    u[7] = vmlaq_n_s32(rnding, v[6], *cospi.add(16));
    u[7] = vmlaq_n_s32(u[7], v[7], *cospi.add(48));
    u[7] = vshlq_s32(u[7], v_bit);

    // stage 5
    addsub_neon(
        u[0],
        u[2],
        v.as_mut_ptr(),
        v.as_mut_ptr().add(2),
        &raw const clamp_lo,
        &raw const clamp_hi,
    );
    addsub_neon(
        u[1],
        u[3],
        v.as_mut_ptr().add(1),
        v.as_mut_ptr().add(3),
        &raw const clamp_lo,
        &raw const clamp_hi,
    );
    addsub_neon(
        u[4],
        u[6],
        v.as_mut_ptr().add(4),
        v.as_mut_ptr().add(6),
        &raw const clamp_lo,
        &raw const clamp_hi,
    );
    addsub_neon(
        u[5],
        u[7],
        v.as_mut_ptr().add(5),
        v.as_mut_ptr().add(7),
        &raw const clamp_lo,
        &raw const clamp_hi,
    );

    // stage 6
    u[0] = v[0];
    u[1] = v[1];
    u[4] = v[4];
    u[5] = v[5];

    v[0] = vmlaq_n_s32(rnding, v[2], *cospi.add(32));
    x = vmulq_n_s32(v[3], *cospi.add(32));
    u[2] = vaddq_s32(v[0], x);
    u[2] = vshlq_s32(u[2], v_bit);

    u[3] = vsubq_s32(v[0], x);
    u[3] = vshlq_s32(u[3], v_bit);

    v[0] = vmlaq_n_s32(rnding, v[6], *cospi.add(32));
    x = vmulq_n_s32(v[7], *cospi.add(32));
    u[6] = vaddq_s32(v[0], x);
    u[6] = vshlq_s32(u[6], v_bit);

    u[7] = vsubq_s32(v[0], x);
    u[7] = vshlq_s32(u[7], v_bit);

    // stage 7
    if do_cols != 0 {
        *out = u[0];
        *out.add(1) = vnegq_s32(u[4]);
        *out.add(2) = u[6];
        *out.add(3) = vnegq_s32(u[2]);
        *out.add(4) = u[3];
        *out.add(5) = vnegq_s32(u[7]);
        *out.add(6) = u[5];
        *out.add(7) = vnegq_s32(u[1]);
    } else {
        let log_range_out: i32 = get_log_range_out(bd);
        let clamp_lo_out: int32x4_t = vdupq_n_s32(-(1 << (log_range_out - 1)));
        let clamp_hi_out: int32x4_t = vdupq_n_s32((1 << (log_range_out - 1)) - 1);
        let v_shift: int32x4_t = vdupq_n_s32(-out_shift);
        let mut offset: int32x4_t = vdupq_n_s32((1 << out_shift) >> 1);
        neg_shift_neon(
            u.as_mut_ptr(),
            u.as_mut_ptr().add(4),
            out,
            out.add(1),
            &raw const clamp_lo_out,
            &raw const clamp_hi_out,
            &raw const v_shift,
            &raw mut offset,
        );
        neg_shift_neon(
            u.as_mut_ptr().add(6),
            u.as_mut_ptr().add(2),
            out.add(2),
            out.add(3),
            &raw const clamp_lo_out,
            &raw const clamp_hi_out,
            &raw const v_shift,
            &raw mut offset,
        );
        neg_shift_neon(
            u.as_mut_ptr().add(3),
            u.as_mut_ptr().add(7),
            out.add(4),
            out.add(5),
            &raw const clamp_lo_out,
            &raw const clamp_hi_out,
            &raw const v_shift,
            &raw mut offset,
        );
        neg_shift_neon(
            u.as_mut_ptr().add(5),
            u.as_mut_ptr().add(1),
            out.add(6),
            out.add(7),
            &raw const clamp_lo_out,
            &raw const clamp_hi_out,
            &raw const v_shift,
            &raw mut offset,
        );
    }
}

unsafe fn idct16x16_low1_neon(
    mut in_: *mut int32x4_t,
    mut out: *mut int32x4_t,
    mut bit: i32,
    mut do_cols: i32,
    mut bd: i32,
    mut out_shift: i32,
) {
    let mut cospi: *const i32 = cospi_arr(bit);
    let mut log_range: i32 = get_log_range(bd, do_cols);
    let mut clamp_lo: int32x4_t = vdupq_n_s32(-(1 << (log_range - 1)));
    let mut clamp_hi: int32x4_t = vdupq_n_s32((1 << (log_range - 1)) - 1);
    let v_bit: int32x4_t = vdupq_n_s32(-bit);
    let rnding: int32x4_t = vdupq_n_s32(1 << (bit - 1));
    // stage 0-4
    *in_ = vmlaq_n_s32(rnding, *in_, *cospi.add(32));
    *in_ = vshlq_s32(*in_, v_bit);

    // stage 5-7
    if !(do_cols != 0) {
        log_range = get_log_range_out(bd);
        clamp_lo = vdupq_n_s32(-(1 << (log_range - 1)));
        clamp_hi = vdupq_n_s32((1 << (log_range - 1)) - 1);
        if out_shift != 0 {
            let mut offset: int32x4_t = vdupq_n_s32((1 << out_shift) >> 1);
            *in_ = vaddq_s32(*in_, offset);
            *in_ = vshlq_s32(*in_, vdupq_n_s32(-out_shift));
        }
    }

    *in_ = vmaxq_s32(*in_, clamp_lo);
    *in_ = vminq_s32(*in_, clamp_hi);
    *out = *in_;
    *out.add(1) = *in_;
    *out.add(2) = *in_;
    *out.add(3) = *in_;
    *out.add(4) = *in_;
    *out.add(5) = *in_;
    *out.add(6) = *in_;
    *out.add(7) = *in_;
    *out.add(8) = *in_;
    *out.add(9) = *in_;
    *out.add(10) = *in_;
    *out.add(11) = *in_;
    *out.add(12) = *in_;
    *out.add(13) = *in_;
    *out.add(14) = *in_;
    *out.add(15) = *in_;
}

unsafe fn idct16x16_low8_neon(
    mut in_: *mut int32x4_t,
    mut out: *mut int32x4_t,
    mut bit: i32,
    mut do_cols: i32,
    mut bd: i32,
    mut out_shift: i32,
) {
    let mut cospi: *const i32 = cospi_arr(bit);
    let log_range: i32 = get_log_range(bd, do_cols);
    let clamp_lo: int32x4_t = vdupq_n_s32(-(1 << (log_range - 1)));
    let clamp_hi: int32x4_t = vdupq_n_s32((1 << (log_range - 1)) - 1);
    let v_bit: int32x4_t = vdupq_n_s32(-bit);
    let rnding: int32x4_t = vdupq_n_s32(1 << (bit - 1));
    let mut u: [int32x4_t; 16] = core::mem::zeroed();
    let mut x: int32x4_t = core::mem::zeroed();
    let mut y: int32x4_t = core::mem::zeroed();
    // stage 0-1
    u[0] = *in_;
    u[2] = *in_.add(4);
    u[4] = *in_.add(2);
    u[6] = *in_.add(6);
    u[8] = *in_.add(1);
    u[10] = *in_.add(5);
    u[12] = *in_.add(3);
    u[14] = *in_.add(7);

    // stage 2
    u[15] =
        half_btf_0_neon_r(cospi.add(4), u.as_mut_ptr().add(8), &raw const v_bit, &raw const rnding);
    u[8] = half_btf_0_neon_r(
        cospi.add(60),
        u.as_mut_ptr().add(8),
        &raw const v_bit,
        &raw const rnding,
    );

    u[9] = half_btf_0_m_neon_r(
        cospi.add(36),
        u.as_mut_ptr().add(14),
        &raw const v_bit,
        &raw const rnding,
    );
    u[14] = half_btf_0_neon_r(
        cospi.add(28),
        u.as_mut_ptr().add(14),
        &raw const v_bit,
        &raw const rnding,
    );

    u[13] = half_btf_0_neon_r(
        cospi.add(20),
        u.as_mut_ptr().add(10),
        &raw const v_bit,
        &raw const rnding,
    );
    u[10] = half_btf_0_neon_r(
        cospi.add(44),
        u.as_mut_ptr().add(10),
        &raw const v_bit,
        &raw const rnding,
    );

    u[11] = half_btf_0_m_neon_r(
        cospi.add(52),
        u.as_mut_ptr().add(12),
        &raw const v_bit,
        &raw const rnding,
    );
    u[12] = half_btf_0_neon_r(
        cospi.add(12),
        u.as_mut_ptr().add(12),
        &raw const v_bit,
        &raw const rnding,
    );

    // stage 3
    u[7] =
        half_btf_0_neon_r(cospi.add(8), u.as_mut_ptr().add(4), &raw const v_bit, &raw const rnding);
    u[4] = half_btf_0_neon_r(
        cospi.add(56),
        u.as_mut_ptr().add(4),
        &raw const v_bit,
        &raw const rnding,
    );
    u[5] = half_btf_0_m_neon_r(
        cospi.add(40),
        u.as_mut_ptr().add(6),
        &raw const v_bit,
        &raw const rnding,
    );
    u[6] = half_btf_0_neon_r(
        cospi.add(24),
        u.as_mut_ptr().add(6),
        &raw const v_bit,
        &raw const rnding,
    );

    addsub_neon(
        u[8],
        u[9],
        u.as_mut_ptr().add(8),
        u.as_mut_ptr().add(9),
        &raw const clamp_lo,
        &raw const clamp_hi,
    );
    addsub_neon(
        u[11],
        u[10],
        u.as_mut_ptr().add(11),
        u.as_mut_ptr().add(10),
        &raw const clamp_lo,
        &raw const clamp_hi,
    );
    addsub_neon(
        u[12],
        u[13],
        u.as_mut_ptr().add(12),
        u.as_mut_ptr().add(13),
        &raw const clamp_lo,
        &raw const clamp_hi,
    );
    addsub_neon(
        u[15],
        u[14],
        u.as_mut_ptr().add(15),
        u.as_mut_ptr().add(14),
        &raw const clamp_lo,
        &raw const clamp_hi,
    );

    // stage 4
    x = vmlaq_n_s32(rnding, u[0], *cospi.add(32));
    u[0] = vshlq_s32(x, v_bit);
    u[1] = u[0];

    u[3] = half_btf_0_neon_r(
        cospi.add(16),
        u.as_mut_ptr().add(2),
        &raw const v_bit,
        &raw const rnding,
    );
    u[2] = half_btf_0_neon_r(
        cospi.add(48),
        u.as_mut_ptr().add(2),
        &raw const v_bit,
        &raw const rnding,
    );

    addsub_neon(
        u[4],
        u[5],
        u.as_mut_ptr().add(4),
        u.as_mut_ptr().add(5),
        &raw const clamp_lo,
        &raw const clamp_hi,
    );
    addsub_neon(
        u[7],
        u[6],
        u.as_mut_ptr().add(7),
        u.as_mut_ptr().add(6),
        &raw const clamp_lo,
        &raw const clamp_hi,
    );

    x = half_btf_neon_mode10_r(
        cospi.add(16),
        u.as_mut_ptr().add(9),
        cospi.add(48),
        u.as_mut_ptr().add(14),
        &raw const v_bit,
        &raw const rnding,
    );
    u[14] = half_btf_neon_r(
        cospi.add(48),
        u.as_mut_ptr().add(9),
        cospi.add(16),
        u.as_mut_ptr().add(14),
        &raw const v_bit,
        &raw const rnding,
    );
    u[9] = x;
    y = half_btf_neon_mode11_r(
        cospi.add(48),
        u.as_mut_ptr().add(10),
        cospi.add(16),
        u.as_mut_ptr().add(13),
        &raw const v_bit,
        &raw const rnding,
    );
    u[13] = half_btf_neon_mode10_r(
        cospi.add(16),
        u.as_mut_ptr().add(10),
        cospi.add(48),
        u.as_mut_ptr().add(13),
        &raw const v_bit,
        &raw const rnding,
    );
    u[10] = y;

    // stage 5
    addsub_neon(
        u[0],
        u[3],
        u.as_mut_ptr(),
        u.as_mut_ptr().add(3),
        &raw const clamp_lo,
        &raw const clamp_hi,
    );
    addsub_neon(
        u[1],
        u[2],
        u.as_mut_ptr().add(1),
        u.as_mut_ptr().add(2),
        &raw const clamp_lo,
        &raw const clamp_hi,
    );

    x = vmulq_n_s32(u[5], *cospi.add(32));
    y = vmlaq_n_s32(rnding, u[6], *cospi.add(32));
    u[5] = vsubq_s32(y, x);
    u[5] = vshlq_s32(u[5], v_bit);

    u[6] = vaddq_s32(y, x);
    u[6] = vshlq_s32(u[6], v_bit);

    addsub_neon(
        u[8],
        u[11],
        u.as_mut_ptr().add(8),
        u.as_mut_ptr().add(11),
        &raw const clamp_lo,
        &raw const clamp_hi,
    );
    addsub_neon(
        u[9],
        u[10],
        u.as_mut_ptr().add(9),
        u.as_mut_ptr().add(10),
        &raw const clamp_lo,
        &raw const clamp_hi,
    );
    addsub_neon(
        u[15],
        u[12],
        u.as_mut_ptr().add(15),
        u.as_mut_ptr().add(12),
        &raw const clamp_lo,
        &raw const clamp_hi,
    );
    addsub_neon(
        u[14],
        u[13],
        u.as_mut_ptr().add(14),
        u.as_mut_ptr().add(13),
        &raw const clamp_lo,
        &raw const clamp_hi,
    );

    // stage 6
    addsub_neon(
        u[0],
        u[7],
        u.as_mut_ptr(),
        u.as_mut_ptr().add(7),
        &raw const clamp_lo,
        &raw const clamp_hi,
    );
    addsub_neon(
        u[1],
        u[6],
        u.as_mut_ptr().add(1),
        u.as_mut_ptr().add(6),
        &raw const clamp_lo,
        &raw const clamp_hi,
    );
    addsub_neon(
        u[2],
        u[5],
        u.as_mut_ptr().add(2),
        u.as_mut_ptr().add(5),
        &raw const clamp_lo,
        &raw const clamp_hi,
    );
    addsub_neon(
        u[3],
        u[4],
        u.as_mut_ptr().add(3),
        u.as_mut_ptr().add(4),
        &raw const clamp_lo,
        &raw const clamp_hi,
    );

    x = vmulq_n_s32(u[10], *cospi.add(32));
    y = vmlaq_n_s32(rnding, u[13], *cospi.add(32));
    u[10] = vsubq_s32(y, x);
    u[10] = vshlq_s32(u[10], v_bit);

    u[13] = vaddq_s32(x, y);
    u[13] = vshlq_s32(u[13], v_bit);

    x = vmulq_n_s32(u[11], *cospi.add(32));
    y = vmlaq_n_s32(rnding, u[12], *cospi.add(32));
    u[11] = vsubq_s32(y, x);
    u[11] = vshlq_s32(u[11], v_bit);

    u[12] = vaddq_s32(x, y);
    u[12] = vshlq_s32(u[12], v_bit);
    // stage 7
    addsub_neon(u[0], u[15], out, out.add(15), &raw const clamp_lo, &raw const clamp_hi);
    addsub_neon(u[1], u[14], out.add(1), out.add(14), &raw const clamp_lo, &raw const clamp_hi);
    addsub_neon(u[2], u[13], out.add(2), out.add(13), &raw const clamp_lo, &raw const clamp_hi);
    addsub_neon(u[3], u[12], out.add(3), out.add(12), &raw const clamp_lo, &raw const clamp_hi);
    addsub_neon(u[4], u[11], out.add(4), out.add(11), &raw const clamp_lo, &raw const clamp_hi);
    addsub_neon(u[5], u[10], out.add(5), out.add(10), &raw const clamp_lo, &raw const clamp_hi);
    addsub_neon(u[6], u[9], out.add(6), out.add(9), &raw const clamp_lo, &raw const clamp_hi);
    addsub_neon(u[7], u[8], out.add(7), out.add(8), &raw const clamp_lo, &raw const clamp_hi);

    if !(do_cols != 0) {
        let log_range_out: i32 = get_log_range_out(bd);
        let clamp_lo_out: int32x4_t = vdupq_n_s32(-(1 << (log_range_out - 1)));
        let clamp_hi_out: int32x4_t = vdupq_n_s32((1 << (log_range_out - 1)) - 1);
        round_shift_8x8(out, out_shift);
        highbd_clamp_s32_neon(out, out, &raw const clamp_lo_out, &raw const clamp_hi_out, 16);
    }
}

unsafe fn iadst16x16_low1_neon(
    mut in_: *mut int32x4_t,
    mut out: *mut int32x4_t,
    mut bit: i32,
    mut do_cols: i32,
    mut bd: i32,
    mut out_shift: i32,
) {
    let mut cospi: *const i32 = cospi_arr(bit);
    let mut v: [int32x4_t; 16] = core::mem::zeroed();
    let mut x: int32x4_t = core::mem::zeroed();
    let mut y: int32x4_t = core::mem::zeroed();
    let mut temp1: int32x4_t = core::mem::zeroed();
    let mut temp2: int32x4_t = core::mem::zeroed();
    let v_bit: int32x4_t = vdupq_n_s32(-bit);
    let rnding: int32x4_t = vdupq_n_s32(1 << (bit - 1));
    // stage 0
    // stage 1
    // stage 2
    v[0] = vmlaq_n_s32(rnding, *in_, *cospi.add(62));
    v[0] = vshlq_s32(v[0], v_bit);

    v[1] = vmlsq_n_s32(rnding, *in_, *cospi.add(2));
    v[1] = vshlq_s32(v[1], v_bit);

    // stage 3
    v[8] = v[0];
    v[9] = v[1];

    // stage 4
    temp1 = vmlaq_n_s32(rnding, v[8], *cospi.add(8));
    temp1 = vmlaq_n_s32(temp1, v[9], *cospi.add(56));
    temp1 = vshlq_s32(temp1, v_bit);

    temp2 = vmlaq_n_s32(rnding, v[8], *cospi.add(56));
    temp2 = vmlsq_n_s32(temp2, v[9], *cospi.add(8));
    temp2 = vshlq_s32(temp2, v_bit);
    v[8] = temp1;
    v[9] = temp2;

    // stage 5
    v[4] = v[0];
    v[5] = v[1];
    v[12] = v[8];
    v[13] = v[9];

    // stage 6
    temp1 = vmlaq_n_s32(rnding, v[4], *cospi.add(16));
    temp1 = vmlaq_n_s32(temp1, v[5], *cospi.add(48));
    temp1 = vshlq_s32(temp1, v_bit);

    temp2 = vmlaq_n_s32(rnding, v[4], *cospi.add(48));
    temp2 = vmlsq_n_s32(temp2, v[5], *cospi.add(16));
    temp2 = vshlq_s32(temp2, v_bit);
    v[4] = temp1;
    v[5] = temp2;

    temp1 = vmlaq_n_s32(rnding, v[12], *cospi.add(16));
    temp1 = vmlaq_n_s32(temp1, v[13], *cospi.add(48));
    temp1 = vshlq_s32(temp1, v_bit);

    temp2 = vmlaq_n_s32(rnding, v[12], *cospi.add(48));
    temp2 = vmlsq_n_s32(temp2, v[13], *cospi.add(16));
    temp2 = vshlq_s32(temp2, v_bit);
    v[12] = temp1;
    v[13] = temp2;

    // stage 7
    v[2] = v[0];
    v[3] = v[1];
    v[6] = v[4];
    v[7] = v[5];
    v[10] = v[8];
    v[11] = v[9];
    v[14] = v[12];
    v[15] = v[13];

    // stage 8
    y = vmlaq_n_s32(rnding, v[2], *cospi.add(32));
    x = vmulq_n_s32(v[3], *cospi.add(32));
    v[2] = vaddq_s32(y, x);
    v[2] = vshlq_s32(v[2], v_bit);

    v[3] = vsubq_s32(y, x);
    v[3] = vshlq_s32(v[3], v_bit);

    y = vmlaq_n_s32(rnding, v[6], *cospi.add(32));
    x = vmulq_n_s32(v[7], *cospi.add(32));
    v[6] = vaddq_s32(y, x);
    v[6] = vshlq_s32(v[6], v_bit);

    v[7] = vsubq_s32(y, x);
    v[7] = vshlq_s32(v[7], v_bit);

    y = vmlaq_n_s32(rnding, v[10], *cospi.add(32));
    x = vmulq_n_s32(v[11], *cospi.add(32));
    v[10] = vaddq_s32(y, x);
    v[10] = vshlq_s32(v[10], v_bit);

    v[11] = vsubq_s32(y, x);
    v[11] = vshlq_s32(v[11], v_bit);

    y = vmlaq_n_s32(rnding, v[14], *cospi.add(32));
    x = vmulq_n_s32(v[15], *cospi.add(32));
    v[14] = vaddq_s32(y, x);
    v[14] = vshlq_s32(v[14], v_bit);

    v[15] = vsubq_s32(y, x);
    v[15] = vshlq_s32(v[15], v_bit);

    // stage 9
    if do_cols != 0 {
        *out = v[0];
        *out.add(1) = vnegq_s32(v[8]);
        *out.add(2) = v[12];
        *out.add(3) = vnegq_s32(v[4]);
        *out.add(4) = v[6];
        *out.add(5) = vnegq_s32(v[14]);
        *out.add(6) = v[10];
        *out.add(7) = vnegq_s32(v[2]);
        *out.add(8) = v[3];
        *out.add(9) = vnegq_s32(v[11]);
        *out.add(10) = v[15];
        *out.add(11) = vnegq_s32(v[7]);
        *out.add(12) = v[5];
        *out.add(13) = vnegq_s32(v[13]);
        *out.add(14) = v[9];
        *out.add(15) = vnegq_s32(v[1]);
    } else {
        let log_range_out: i32 = get_log_range_out(bd);
        let clamp_lo_out: int32x4_t = vdupq_n_s32(-(1 << (log_range_out - 1)));
        let clamp_hi_out: int32x4_t = vdupq_n_s32((1 << (log_range_out - 1)) - 1);
        let v_shift: int32x4_t = vdupq_n_s32(-out_shift);
        let mut offset: int32x4_t = vdupq_n_s32((1 << out_shift) >> 1);
        neg_shift_neon(
            v.as_mut_ptr(),
            v.as_mut_ptr().add(8),
            out,
            out.add(1),
            &raw const clamp_lo_out,
            &raw const clamp_hi_out,
            &raw const v_shift,
            &raw mut offset,
        );
        neg_shift_neon(
            v.as_mut_ptr().add(12),
            v.as_mut_ptr().add(4),
            out.add(2),
            out.add(3),
            &raw const clamp_lo_out,
            &raw const clamp_hi_out,
            &raw const v_shift,
            &raw mut offset,
        );
        neg_shift_neon(
            v.as_mut_ptr().add(6),
            v.as_mut_ptr().add(14),
            out.add(4),
            out.add(5),
            &raw const clamp_lo_out,
            &raw const clamp_hi_out,
            &raw const v_shift,
            &raw mut offset,
        );
        neg_shift_neon(
            v.as_mut_ptr().add(10),
            v.as_mut_ptr().add(2),
            out.add(6),
            out.add(7),
            &raw const clamp_lo_out,
            &raw const clamp_hi_out,
            &raw const v_shift,
            &raw mut offset,
        );
        neg_shift_neon(
            v.as_mut_ptr().add(3),
            v.as_mut_ptr().add(11),
            out.add(8),
            out.add(9),
            &raw const clamp_lo_out,
            &raw const clamp_hi_out,
            &raw const v_shift,
            &raw mut offset,
        );
        neg_shift_neon(
            v.as_mut_ptr().add(15),
            v.as_mut_ptr().add(7),
            out.add(10),
            out.add(11),
            &raw const clamp_lo_out,
            &raw const clamp_hi_out,
            &raw const v_shift,
            &raw mut offset,
        );
        neg_shift_neon(
            v.as_mut_ptr().add(5),
            v.as_mut_ptr().add(13),
            out.add(12),
            out.add(13),
            &raw const clamp_lo_out,
            &raw const clamp_hi_out,
            &raw const v_shift,
            &raw mut offset,
        );
        neg_shift_neon(
            v.as_mut_ptr().add(9),
            v.as_mut_ptr().add(1),
            out.add(14),
            out.add(15),
            &raw const clamp_lo_out,
            &raw const clamp_hi_out,
            &raw const v_shift,
            &raw mut offset,
        );
    }
}

unsafe fn iadst16x16_low8_neon(
    mut in_: *mut int32x4_t,
    mut out: *mut int32x4_t,
    mut bit: i32,
    mut do_cols: i32,
    mut bd: i32,
    mut out_shift: i32,
) {
    let mut cospi: *const i32 = cospi_arr(bit);
    let log_range: i32 = get_log_range(bd, do_cols);
    let clamp_lo: int32x4_t = vdupq_n_s32(-(1 << (log_range - 1)));
    let clamp_hi: int32x4_t = vdupq_n_s32((1 << (log_range - 1)) - 1);
    let mut zero: int32x4_t = vdupq_n_s32(0);
    let mut u: [int32x4_t; 16] = core::mem::zeroed();
    let mut x: int32x4_t = core::mem::zeroed();
    let mut y: int32x4_t = core::mem::zeroed();
    let v_bit: int32x4_t = vdupq_n_s32(-bit);
    let rnding: int32x4_t = vdupq_n_s32(1 << (bit - 1));
    // stage 0-2
    u[0] = vmlaq_n_s32(rnding, *in_, *cospi.add(62));
    u[0] = vshlq_s32(u[0], v_bit);

    u[1] = vmlsq_n_s32(rnding, *in_, *cospi.add(2));
    u[1] = vshlq_s32(u[1], v_bit);

    u[2] = vmlaq_n_s32(rnding, *in_.add(2), *cospi.add(54));
    u[2] = vshlq_s32(u[2], v_bit);

    u[3] = vmlsq_n_s32(rnding, *in_.add(2), *cospi.add(10));
    u[3] = vshlq_s32(u[3], v_bit);

    u[4] = vmlaq_n_s32(rnding, *in_.add(4), *cospi.add(46));
    u[4] = vshlq_s32(u[4], v_bit);

    u[5] = vmlsq_n_s32(rnding, *in_.add(4), *cospi.add(18));
    u[5] = vshlq_s32(u[5], v_bit);

    u[6] = vmlaq_n_s32(rnding, *in_.add(6), *cospi.add(38));
    u[6] = vshlq_s32(u[6], v_bit);

    u[7] = vmlsq_n_s32(rnding, *in_.add(6), *cospi.add(26));
    u[7] = vshlq_s32(u[7], v_bit);

    u[8] = vmlaq_n_s32(rnding, *in_.add(7), *cospi.add(34));
    u[8] = vshlq_s32(u[8], v_bit);

    u[9] = vmlaq_n_s32(rnding, *in_.add(7), *cospi.add(30));
    u[9] = vshlq_s32(u[9], v_bit);

    u[10] = vmlaq_n_s32(rnding, *in_.add(5), *cospi.add(42));
    u[10] = vshlq_s32(u[10], v_bit);

    u[11] = vmlaq_n_s32(rnding, *in_.add(5), *cospi.add(22));
    u[11] = vshlq_s32(u[11], v_bit);

    u[12] = vmlaq_n_s32(rnding, *in_.add(3), *cospi.add(50));
    u[12] = vshlq_s32(u[12], v_bit);

    u[13] = vmlaq_n_s32(rnding, *in_.add(3), *cospi.add(14));
    u[13] = vshlq_s32(u[13], v_bit);

    u[14] = vmlaq_n_s32(rnding, *in_.add(1), *cospi.add(58));
    u[14] = vshlq_s32(u[14], v_bit);

    u[15] = vmlaq_n_s32(rnding, *in_.add(1), *cospi.add(6));
    u[15] = vshlq_s32(u[15], v_bit);

    // stage 3
    addsub_neon(
        u[0],
        u[8],
        u.as_mut_ptr(),
        u.as_mut_ptr().add(8),
        &raw const clamp_lo,
        &raw const clamp_hi,
    );
    addsub_neon(
        u[1],
        u[9],
        u.as_mut_ptr().add(1),
        u.as_mut_ptr().add(9),
        &raw const clamp_lo,
        &raw const clamp_hi,
    );
    addsub_neon(
        u[2],
        u[10],
        u.as_mut_ptr().add(2),
        u.as_mut_ptr().add(10),
        &raw const clamp_lo,
        &raw const clamp_hi,
    );
    addsub_neon(
        u[3],
        u[11],
        u.as_mut_ptr().add(3),
        u.as_mut_ptr().add(11),
        &raw const clamp_lo,
        &raw const clamp_hi,
    );
    addsub_neon(
        u[4],
        u[12],
        u.as_mut_ptr().add(4),
        u.as_mut_ptr().add(12),
        &raw const clamp_lo,
        &raw const clamp_hi,
    );
    addsub_neon(
        u[5],
        u[13],
        u.as_mut_ptr().add(5),
        u.as_mut_ptr().add(13),
        &raw const clamp_lo,
        &raw const clamp_hi,
    );
    addsub_neon(
        u[6],
        u[14],
        u.as_mut_ptr().add(6),
        u.as_mut_ptr().add(14),
        &raw const clamp_lo,
        &raw const clamp_hi,
    );
    addsub_neon(
        u[7],
        u[15],
        u.as_mut_ptr().add(7),
        u.as_mut_ptr().add(15),
        &raw const clamp_lo,
        &raw const clamp_hi,
    );

    // stage 4
    y = vmlaq_n_s32(rnding, u[8], *cospi.add(56));
    u[8] = vmlaq_n_s32(rnding, u[8], *cospi.add(8));
    u[8] = vmlaq_n_s32(u[8], u[9], *cospi.add(56));
    u[8] = vshlq_s32(u[8], v_bit);

    u[9] = vmlsq_n_s32(y, u[9], *cospi.add(8));
    u[9] = vshlq_s32(u[9], v_bit);

    y = vmlaq_n_s32(rnding, u[10], *cospi.add(24));
    u[10] = vmlaq_n_s32(rnding, u[10], *cospi.add(40));
    u[10] = vmlaq_n_s32(u[10], u[11], *cospi.add(24));
    u[10] = vshlq_s32(u[10], v_bit);

    u[11] = vmlsq_n_s32(y, u[11], *cospi.add(40));
    u[11] = vshlq_s32(u[11], v_bit);

    y = vmlaq_n_s32(rnding, u[12], *cospi.add(8));
    u[12] = vmlsq_n_s32(rnding, u[12], *cospi.add(56));
    u[12] = vmlaq_n_s32(u[12], u[13], *cospi.add(8));
    u[12] = vshlq_s32(u[12], v_bit);

    u[13] = vmlaq_n_s32(y, u[13], *cospi.add(56));
    u[13] = vshlq_s32(u[13], v_bit);

    y = vmlaq_n_s32(rnding, u[14], *cospi.add(40));
    u[14] = vmlsq_n_s32(rnding, u[14], *cospi.add(24));
    u[14] = vmlaq_n_s32(u[14], u[15], *cospi.add(40));
    u[14] = vshlq_s32(u[14], v_bit);

    u[15] = vmlaq_n_s32(y, u[15], *cospi.add(24));
    u[15] = vshlq_s32(u[15], v_bit);

    // stage 5
    addsub_neon(
        u[0],
        u[4],
        u.as_mut_ptr(),
        u.as_mut_ptr().add(4),
        &raw const clamp_lo,
        &raw const clamp_hi,
    );
    addsub_neon(
        u[1],
        u[5],
        u.as_mut_ptr().add(1),
        u.as_mut_ptr().add(5),
        &raw const clamp_lo,
        &raw const clamp_hi,
    );
    addsub_neon(
        u[2],
        u[6],
        u.as_mut_ptr().add(2),
        u.as_mut_ptr().add(6),
        &raw const clamp_lo,
        &raw const clamp_hi,
    );
    addsub_neon(
        u[3],
        u[7],
        u.as_mut_ptr().add(3),
        u.as_mut_ptr().add(7),
        &raw const clamp_lo,
        &raw const clamp_hi,
    );
    addsub_neon(
        u[8],
        u[12],
        u.as_mut_ptr().add(8),
        u.as_mut_ptr().add(12),
        &raw const clamp_lo,
        &raw const clamp_hi,
    );
    addsub_neon(
        u[9],
        u[13],
        u.as_mut_ptr().add(9),
        u.as_mut_ptr().add(13),
        &raw const clamp_lo,
        &raw const clamp_hi,
    );
    addsub_neon(
        u[10],
        u[14],
        u.as_mut_ptr().add(10),
        u.as_mut_ptr().add(14),
        &raw const clamp_lo,
        &raw const clamp_hi,
    );
    addsub_neon(
        u[11],
        u[15],
        u.as_mut_ptr().add(11),
        u.as_mut_ptr().add(15),
        &raw const clamp_lo,
        &raw const clamp_hi,
    );

    // stage 6
    y = vmlaq_n_s32(rnding, u[4], *cospi.add(48));
    u[4] = vmlaq_n_s32(rnding, u[4], *cospi.add(16));
    u[4] = vmlaq_n_s32(u[4], u[5], *cospi.add(48));
    u[4] = vshlq_s32(u[4], v_bit);

    u[5] = vmlsq_n_s32(y, u[5], *cospi.add(16));
    u[5] = vshlq_s32(u[5], v_bit);

    y = vmlaq_n_s32(rnding, u[6], *cospi.add(16));
    u[6] = vmlsq_n_s32(rnding, u[6], *cospi.add(48));
    u[6] = vmlaq_n_s32(u[6], u[7], *cospi.add(16));
    u[6] = vshlq_s32(u[6], v_bit);

    u[7] = vmlaq_n_s32(y, u[7], *cospi.add(48));
    u[7] = vshlq_s32(u[7], v_bit);

    y = vmlaq_n_s32(rnding, u[12], *cospi.add(48));
    u[12] = vmulq_n_s32(u[12], *cospi.add(16));
    u[12] = vmlaq_n_s32(u[12], u[13], *cospi.add(48));
    u[12] = vshlq_s32(u[12], v_bit);

    u[13] = vmlsq_n_s32(y, u[13], *cospi.add(16));
    u[13] = vshlq_s32(u[13], v_bit);

    y = vmlaq_n_s32(rnding, u[14], *cospi.add(16));
    u[14] = vmlsq_n_s32(rnding, u[14], *cospi.add(48));
    u[14] = vmlaq_n_s32(u[14], u[15], *cospi.add(16));
    u[14] = vshlq_s32(u[14], v_bit);

    u[15] = vmlaq_n_s32(y, u[15], *cospi.add(48));
    u[15] = vshlq_s32(u[15], v_bit);

    // stage 7
    addsub_neon(
        u[0],
        u[2],
        u.as_mut_ptr(),
        u.as_mut_ptr().add(2),
        &raw const clamp_lo,
        &raw const clamp_hi,
    );
    addsub_neon(
        u[1],
        u[3],
        u.as_mut_ptr().add(1),
        u.as_mut_ptr().add(3),
        &raw const clamp_lo,
        &raw const clamp_hi,
    );
    addsub_neon(
        u[4],
        u[6],
        u.as_mut_ptr().add(4),
        u.as_mut_ptr().add(6),
        &raw const clamp_lo,
        &raw const clamp_hi,
    );
    addsub_neon(
        u[5],
        u[7],
        u.as_mut_ptr().add(5),
        u.as_mut_ptr().add(7),
        &raw const clamp_lo,
        &raw const clamp_hi,
    );
    addsub_neon(
        u[8],
        u[10],
        u.as_mut_ptr().add(8),
        u.as_mut_ptr().add(10),
        &raw const clamp_lo,
        &raw const clamp_hi,
    );
    addsub_neon(
        u[9],
        u[11],
        u.as_mut_ptr().add(9),
        u.as_mut_ptr().add(11),
        &raw const clamp_lo,
        &raw const clamp_hi,
    );
    addsub_neon(
        u[12],
        u[14],
        u.as_mut_ptr().add(12),
        u.as_mut_ptr().add(14),
        &raw const clamp_lo,
        &raw const clamp_hi,
    );
    addsub_neon(
        u[13],
        u[15],
        u.as_mut_ptr().add(13),
        u.as_mut_ptr().add(15),
        &raw const clamp_lo,
        &raw const clamp_hi,
    );

    // stage 8
    y = vmlaq_n_s32(rnding, u[2], *cospi.add(32));
    x = vmulq_n_s32(u[3], *cospi.add(32));
    u[2] = vaddq_s32(y, x);
    u[2] = vshlq_s32(u[2], v_bit);

    u[3] = vsubq_s32(y, x);
    u[3] = vshlq_s32(u[3], v_bit);
    y = vmlaq_n_s32(rnding, u[6], *cospi.add(32));
    x = vmulq_n_s32(u[7], *cospi.add(32));
    u[6] = vaddq_s32(y, x);
    u[6] = vshlq_s32(u[6], v_bit);

    u[7] = vsubq_s32(y, x);
    u[7] = vshlq_s32(u[7], v_bit);

    y = vmlaq_n_s32(rnding, u[10], *cospi.add(32));
    x = vmulq_n_s32(u[11], *cospi.add(32));
    u[10] = vaddq_s32(y, x);
    u[10] = vshlq_s32(u[10], v_bit);

    u[11] = vsubq_s32(y, x);
    u[11] = vshlq_s32(u[11], v_bit);

    y = vmlaq_n_s32(rnding, u[14], *cospi.add(32));
    x = vmulq_n_s32(u[15], *cospi.add(32));
    u[14] = vaddq_s32(y, x);
    u[14] = vshlq_s32(u[14], v_bit);

    u[15] = vsubq_s32(y, x);
    u[15] = vshlq_s32(u[15], v_bit);

    // stage 9
    if do_cols != 0 {
        *out = u[0];
        *out.add(1) = vsubq_s32(zero, u[8]);
        *out.add(2) = u[12];
        *out.add(3) = vsubq_s32(zero, u[4]);
        *out.add(4) = u[6];
        *out.add(5) = vsubq_s32(zero, u[14]);
        *out.add(6) = u[10];
        *out.add(7) = vsubq_s32(zero, u[2]);
        *out.add(8) = u[3];
        *out.add(9) = vsubq_s32(zero, u[11]);
        *out.add(10) = u[15];
        *out.add(11) = vsubq_s32(zero, u[7]);
        *out.add(12) = u[5];
        *out.add(13) = vsubq_s32(zero, u[13]);
        *out.add(14) = u[9];
        *out.add(15) = vsubq_s32(zero, u[1]);
    } else {
        let log_range_out: i32 = get_log_range_out(bd);
        let clamp_lo_out: int32x4_t = vdupq_n_s32(-(1 << (log_range_out - 1)));
        let clamp_hi_out: int32x4_t = vdupq_n_s32((1 << (log_range_out - 1)) - 1);
        let v_shift: int32x4_t = vdupq_n_s32(-out_shift);
        let mut offset: int32x4_t = vdupq_n_s32((1 << out_shift) >> 1);
        neg_shift_neon(
            u.as_mut_ptr(),
            u.as_mut_ptr().add(8),
            out,
            out.add(1),
            &raw const clamp_lo_out,
            &raw const clamp_hi_out,
            &raw const v_shift,
            &raw mut offset,
        );
        neg_shift_neon(
            u.as_mut_ptr().add(12),
            u.as_mut_ptr().add(4),
            out.add(2),
            out.add(3),
            &raw const clamp_lo_out,
            &raw const clamp_hi_out,
            &raw const v_shift,
            &raw mut offset,
        );
        neg_shift_neon(
            u.as_mut_ptr().add(6),
            u.as_mut_ptr().add(14),
            out.add(4),
            out.add(5),
            &raw const clamp_lo_out,
            &raw const clamp_hi_out,
            &raw const v_shift,
            &raw mut offset,
        );
        neg_shift_neon(
            u.as_mut_ptr().add(10),
            u.as_mut_ptr().add(2),
            out.add(6),
            out.add(7),
            &raw const clamp_lo_out,
            &raw const clamp_hi_out,
            &raw const v_shift,
            &raw mut offset,
        );
        neg_shift_neon(
            u.as_mut_ptr().add(3),
            u.as_mut_ptr().add(11),
            out.add(8),
            out.add(9),
            &raw const clamp_lo_out,
            &raw const clamp_hi_out,
            &raw const v_shift,
            &raw mut offset,
        );
        neg_shift_neon(
            u.as_mut_ptr().add(15),
            u.as_mut_ptr().add(7),
            out.add(10),
            out.add(11),
            &raw const clamp_lo_out,
            &raw const clamp_hi_out,
            &raw const v_shift,
            &raw mut offset,
        );
        neg_shift_neon(
            u.as_mut_ptr().add(5),
            u.as_mut_ptr().add(13),
            out.add(12),
            out.add(13),
            &raw const clamp_lo_out,
            &raw const clamp_hi_out,
            &raw const v_shift,
            &raw mut offset,
        );
        neg_shift_neon(
            u.as_mut_ptr().add(9),
            u.as_mut_ptr().add(1),
            out.add(14),
            out.add(15),
            &raw const clamp_lo_out,
            &raw const clamp_hi_out,
            &raw const v_shift,
            &raw mut offset,
        );
    }
}

unsafe fn idct16x16_neon(
    mut in_: *mut int32x4_t,
    mut out: *mut int32x4_t,
    mut bit: i32,
    mut do_cols: i32,
    mut bd: i32,
    mut out_shift: i32,
) {
    let mut cospi: *const i32 = cospi_arr(bit);
    let log_range: i32 = get_log_range(bd, do_cols);
    let clamp_lo: int32x4_t = vdupq_n_s32(-(1 << (log_range - 1)));
    let clamp_hi: int32x4_t = vdupq_n_s32((1 << (log_range - 1)) - 1);
    let mut u: [int32x4_t; 16] = core::mem::zeroed();
    let mut v: [int32x4_t; 16] = core::mem::zeroed();
    let mut x: int32x4_t = core::mem::zeroed();
    let mut y: int32x4_t = core::mem::zeroed();
    let v_bit: int32x4_t = vdupq_n_s32(-bit);
    let rnding: int32x4_t = vdupq_n_s32(1 << (bit - 1));

    {
        // stage 0-1
        u[0] = *in_;
        u[1] = *in_.add(8);
        u[2] = *in_.add(4);
        u[3] = *in_.add(12);
        u[4] = *in_.add(2);
        u[5] = *in_.add(10);
        u[6] = *in_.add(6);
        u[7] = *in_.add(14);
        u[8] = *in_.add(1);
        u[9] = *in_.add(9);
        u[10] = *in_.add(5);
        u[11] = *in_.add(13);
        u[12] = *in_.add(3);
        u[13] = *in_.add(11);
        u[14] = *in_.add(7);
        u[15] = *in_.add(15);

        // stage 2
        v[0] = u[0];
        v[1] = u[1];
        v[2] = u[2];
        v[3] = u[3];
        v[4] = u[4];
        v[5] = u[5];
        v[6] = u[6];
        v[7] = u[7];

        v[8] = half_btf_neon_mode01_r(
            cospi.add(60),
            u.as_mut_ptr().add(8),
            cospi.add(4),
            u.as_mut_ptr().add(15),
            &raw const v_bit,
            &raw const rnding,
        );
        v[9] = half_btf_neon_mode01_r(
            cospi.add(28),
            u.as_mut_ptr().add(9),
            cospi.add(36),
            u.as_mut_ptr().add(14),
            &raw const v_bit,
            &raw const rnding,
        );
        v[10] = half_btf_neon_mode01_r(
            cospi.add(44),
            u.as_mut_ptr().add(10),
            cospi.add(20),
            u.as_mut_ptr().add(13),
            &raw const v_bit,
            &raw const rnding,
        );
        v[11] = half_btf_neon_mode01_r(
            cospi.add(12),
            u.as_mut_ptr().add(11),
            cospi.add(52),
            u.as_mut_ptr().add(12),
            &raw const v_bit,
            &raw const rnding,
        );
        v[12] = half_btf_neon_r(
            cospi.add(52),
            u.as_mut_ptr().add(11),
            cospi.add(12),
            u.as_mut_ptr().add(12),
            &raw const v_bit,
            &raw const rnding,
        );
        v[13] = half_btf_neon_r(
            cospi.add(20),
            u.as_mut_ptr().add(10),
            cospi.add(44),
            u.as_mut_ptr().add(13),
            &raw const v_bit,
            &raw const rnding,
        );
        v[14] = half_btf_neon_r(
            cospi.add(36),
            u.as_mut_ptr().add(9),
            cospi.add(28),
            u.as_mut_ptr().add(14),
            &raw const v_bit,
            &raw const rnding,
        );
        v[15] = half_btf_neon_r(
            cospi.add(4),
            u.as_mut_ptr().add(8),
            cospi.add(60),
            u.as_mut_ptr().add(15),
            &raw const v_bit,
            &raw const rnding,
        );

        // stage 3
        u[0] = v[0];
        u[1] = v[1];
        u[2] = v[2];
        u[3] = v[3];
        u[4] = half_btf_neon_mode01_r(
            cospi.add(56),
            v.as_mut_ptr().add(4),
            cospi.add(8),
            v.as_mut_ptr().add(7),
            &raw const v_bit,
            &raw const rnding,
        );
        u[5] = half_btf_neon_mode01_r(
            cospi.add(24),
            v.as_mut_ptr().add(5),
            cospi.add(40),
            v.as_mut_ptr().add(6),
            &raw const v_bit,
            &raw const rnding,
        );
        u[6] = half_btf_neon_r(
            cospi.add(40),
            v.as_mut_ptr().add(5),
            cospi.add(24),
            v.as_mut_ptr().add(6),
            &raw const v_bit,
            &raw const rnding,
        );
        u[7] = half_btf_neon_r(
            cospi.add(8),
            v.as_mut_ptr().add(4),
            cospi.add(56),
            v.as_mut_ptr().add(7),
            &raw const v_bit,
            &raw const rnding,
        );
        addsub_neon(
            v[8],
            v[9],
            u.as_mut_ptr().add(8),
            u.as_mut_ptr().add(9),
            &raw const clamp_lo,
            &raw const clamp_hi,
        );
        addsub_neon(
            v[11],
            v[10],
            u.as_mut_ptr().add(11),
            u.as_mut_ptr().add(10),
            &raw const clamp_lo,
            &raw const clamp_hi,
        );
        addsub_neon(
            v[12],
            v[13],
            u.as_mut_ptr().add(12),
            u.as_mut_ptr().add(13),
            &raw const clamp_lo,
            &raw const clamp_hi,
        );
        addsub_neon(
            v[15],
            v[14],
            u.as_mut_ptr().add(15),
            u.as_mut_ptr().add(14),
            &raw const clamp_lo,
            &raw const clamp_hi,
        );

        // stage 4
        x = vmlaq_n_s32(rnding, u[0], *cospi.add(32));
        y = vmulq_n_s32(u[1], *cospi.add(32));
        v[0] = vaddq_s32(x, y);
        v[0] = vshlq_s32(v[0], v_bit);

        v[1] = vsubq_s32(x, y);
        v[1] = vshlq_s32(v[1], v_bit);

        v[2] = half_btf_neon_mode01_r(
            cospi.add(48),
            u.as_mut_ptr().add(2),
            cospi.add(16),
            u.as_mut_ptr().add(3),
            &raw const v_bit,
            &raw const rnding,
        );
        v[3] = half_btf_neon_r(
            cospi.add(16),
            u.as_mut_ptr().add(2),
            cospi.add(48),
            u.as_mut_ptr().add(3),
            &raw const v_bit,
            &raw const rnding,
        );
        addsub_neon(
            u[4],
            u[5],
            v.as_mut_ptr().add(4),
            v.as_mut_ptr().add(5),
            &raw const clamp_lo,
            &raw const clamp_hi,
        );
        addsub_neon(
            u[7],
            u[6],
            v.as_mut_ptr().add(7),
            v.as_mut_ptr().add(6),
            &raw const clamp_lo,
            &raw const clamp_hi,
        );
        v[8] = u[8];
        v[9] = half_btf_neon_mode10_r(
            cospi.add(16),
            u.as_mut_ptr().add(9),
            cospi.add(48),
            u.as_mut_ptr().add(14),
            &raw const v_bit,
            &raw const rnding,
        );
        v[10] = half_btf_neon_mode11_r(
            cospi.add(48),
            u.as_mut_ptr().add(10),
            cospi.add(16),
            u.as_mut_ptr().add(13),
            &raw const v_bit,
            &raw const rnding,
        );
        v[11] = u[11];
        v[12] = u[12];
        v[13] = half_btf_neon_mode10_r(
            cospi.add(16),
            u.as_mut_ptr().add(10),
            cospi.add(48),
            u.as_mut_ptr().add(13),
            &raw const v_bit,
            &raw const rnding,
        );
        v[14] = half_btf_neon_r(
            cospi.add(48),
            u.as_mut_ptr().add(9),
            cospi.add(16),
            u.as_mut_ptr().add(14),
            &raw const v_bit,
            &raw const rnding,
        );
        v[15] = u[15];

        // stage 5
        addsub_neon(
            v[0],
            v[3],
            u.as_mut_ptr(),
            u.as_mut_ptr().add(3),
            &raw const clamp_lo,
            &raw const clamp_hi,
        );
        addsub_neon(
            v[1],
            v[2],
            u.as_mut_ptr().add(1),
            u.as_mut_ptr().add(2),
            &raw const clamp_lo,
            &raw const clamp_hi,
        );
        u[4] = v[4];

        x = vmulq_n_s32(v[5], *cospi.add(32));
        y = vmlaq_n_s32(rnding, v[6], *cospi.add(32));
        u[5] = vsubq_s32(y, x);
        u[5] = vshlq_s32(u[5], v_bit);

        u[6] = vaddq_s32(y, x);
        u[6] = vshlq_s32(u[6], v_bit);

        u[7] = v[7];
        addsub_neon(
            v[8],
            v[11],
            u.as_mut_ptr().add(8),
            u.as_mut_ptr().add(11),
            &raw const clamp_lo,
            &raw const clamp_hi,
        );
        addsub_neon(
            v[9],
            v[10],
            u.as_mut_ptr().add(9),
            u.as_mut_ptr().add(10),
            &raw const clamp_lo,
            &raw const clamp_hi,
        );
        addsub_neon(
            v[15],
            v[12],
            u.as_mut_ptr().add(15),
            u.as_mut_ptr().add(12),
            &raw const clamp_lo,
            &raw const clamp_hi,
        );
        addsub_neon(
            v[14],
            v[13],
            u.as_mut_ptr().add(14),
            u.as_mut_ptr().add(13),
            &raw const clamp_lo,
            &raw const clamp_hi,
        );

        // stage 6
        addsub_neon(
            u[0],
            u[7],
            v.as_mut_ptr(),
            v.as_mut_ptr().add(7),
            &raw const clamp_lo,
            &raw const clamp_hi,
        );
        addsub_neon(
            u[1],
            u[6],
            v.as_mut_ptr().add(1),
            v.as_mut_ptr().add(6),
            &raw const clamp_lo,
            &raw const clamp_hi,
        );
        addsub_neon(
            u[2],
            u[5],
            v.as_mut_ptr().add(2),
            v.as_mut_ptr().add(5),
            &raw const clamp_lo,
            &raw const clamp_hi,
        );
        addsub_neon(
            u[3],
            u[4],
            v.as_mut_ptr().add(3),
            v.as_mut_ptr().add(4),
            &raw const clamp_lo,
            &raw const clamp_hi,
        );
        v[8] = u[8];
        v[9] = u[9];

        x = vmulq_n_s32(u[10], *cospi.add(32));
        y = vmlaq_n_s32(rnding, u[13], *cospi.add(32));
        v[10] = vsubq_s32(y, x);
        v[10] = vshlq_s32(v[10], v_bit);

        v[13] = vaddq_s32(x, y);
        v[13] = vshlq_s32(v[13], v_bit);

        x = vmulq_n_s32(u[11], *cospi.add(32));
        y = vmlaq_n_s32(rnding, u[12], *cospi.add(32));
        v[11] = vsubq_s32(y, x);
        v[11] = vshlq_s32(v[11], v_bit);

        v[12] = vaddq_s32(x, y);
        v[12] = vshlq_s32(v[12], v_bit);

        v[14] = u[14];
        v[15] = u[15];

        // stage 7
        addsub_neon(v[0], v[15], out, out.add(15), &raw const clamp_lo, &raw const clamp_hi);
        addsub_neon(v[1], v[14], out.add(1), out.add(14), &raw const clamp_lo, &raw const clamp_hi);
        addsub_neon(v[2], v[13], out.add(2), out.add(13), &raw const clamp_lo, &raw const clamp_hi);
        addsub_neon(v[3], v[12], out.add(3), out.add(12), &raw const clamp_lo, &raw const clamp_hi);
        addsub_neon(v[4], v[11], out.add(4), out.add(11), &raw const clamp_lo, &raw const clamp_hi);
        addsub_neon(v[5], v[10], out.add(5), out.add(10), &raw const clamp_lo, &raw const clamp_hi);
        addsub_neon(v[6], v[9], out.add(6), out.add(9), &raw const clamp_lo, &raw const clamp_hi);
        addsub_neon(v[7], v[8], out.add(7), out.add(8), &raw const clamp_lo, &raw const clamp_hi);

        if !(do_cols != 0) {
            let log_range_out: i32 = get_log_range_out(bd);
            let clamp_lo_out: int32x4_t = vdupq_n_s32(-(1 << (log_range_out - 1)));
            let clamp_hi_out: int32x4_t = vdupq_n_s32((1 << (log_range_out - 1)) - 1);
            round_shift_8x8(out, out_shift);
            highbd_clamp_s32_neon(out, out, &raw const clamp_lo_out, &raw const clamp_hi_out, 16);
        }
    }
}

unsafe fn iadst16x16_neon(
    mut in_: *mut int32x4_t,
    mut out: *mut int32x4_t,
    mut bit: i32,
    mut do_cols: i32,
    mut bd: i32,
    mut out_shift: i32,
) {
    let mut cospi: *const i32 = cospi_arr(bit);
    let log_range: i32 = get_log_range(bd, do_cols);
    let clamp_lo: int32x4_t = vdupq_n_s32(-(1 << (log_range - 1)));
    let clamp_hi: int32x4_t = vdupq_n_s32((1 << (log_range - 1)) - 1);
    let zero: int32x4_t = vdupq_n_s32(0);
    let v_bit: int32x4_t = vdupq_n_s32(-bit);
    let rnding: int32x4_t = vdupq_n_s32(1 << (bit - 1));
    let mut u: [int32x4_t; 16] = core::mem::zeroed();
    let mut v: [int32x4_t; 16] = core::mem::zeroed();
    let mut x: int32x4_t = core::mem::zeroed();
    let mut y: int32x4_t = core::mem::zeroed();
    // Calculate the column 0, 1, 2, 3
    // stage 0
    // stage 1
    // stage 2
    v[0] = vmlaq_n_s32(rnding, *in_.add(15), *cospi.add(2));
    v[0] = vmlaq_n_s32(v[0], *in_, *cospi.add(62));
    v[0] = vshlq_s32(v[0], v_bit);

    v[1] = vmlaq_n_s32(rnding, *in_.add(15), *cospi.add(62));
    v[1] = vmlsq_n_s32(v[1], *in_, *cospi.add(2));
    v[1] = vshlq_s32(v[1], v_bit);

    v[2] = vmlaq_n_s32(rnding, *in_.add(13), *cospi.add(10));
    v[2] = vmlaq_n_s32(v[2], *in_.add(2), *cospi.add(54));
    v[2] = vshlq_s32(v[2], v_bit);

    v[3] = vmlaq_n_s32(rnding, *in_.add(13), *cospi.add(54));
    v[3] = vmlsq_n_s32(v[3], *in_.add(2), *cospi.add(10));
    v[3] = vshlq_s32(v[3], v_bit);

    v[4] = vmlaq_n_s32(rnding, *in_.add(11), *cospi.add(18));
    v[4] = vmlaq_n_s32(v[4], *in_.add(4), *cospi.add(46));
    v[4] = vshlq_s32(v[4], v_bit);

    v[5] = vmlaq_n_s32(rnding, *in_.add(11), *cospi.add(46));
    v[5] = vmlsq_n_s32(v[5], *in_.add(4), *cospi.add(18));
    v[5] = vshlq_s32(v[5], v_bit);

    v[6] = vmlaq_n_s32(rnding, *in_.add(9), *cospi.add(26));
    v[6] = vmlaq_n_s32(v[6], *in_.add(6), *cospi.add(38));
    v[6] = vshlq_s32(v[6], v_bit);

    v[7] = vmlaq_n_s32(rnding, *in_.add(9), *cospi.add(38));
    v[7] = vmlsq_n_s32(v[7], *in_.add(6), *cospi.add(26));
    v[7] = vshlq_s32(v[7], v_bit);

    v[8] = vmlaq_n_s32(rnding, *in_.add(7), *cospi.add(34));
    v[8] = vmlaq_n_s32(v[8], *in_.add(8), *cospi.add(30));
    v[8] = vshlq_s32(v[8], v_bit);

    v[9] = vmlaq_n_s32(rnding, *in_.add(7), *cospi.add(30));
    v[9] = vmlsq_n_s32(v[9], *in_.add(8), *cospi.add(34));
    v[9] = vshlq_s32(v[9], v_bit);

    v[10] = vmlaq_n_s32(rnding, *in_.add(5), *cospi.add(42));
    v[10] = vmlaq_n_s32(v[10], *in_.add(10), *cospi.add(22));
    v[10] = vshlq_s32(v[10], v_bit);

    v[11] = vmlaq_n_s32(rnding, *in_.add(5), *cospi.add(22));
    v[11] = vmlsq_n_s32(v[11], *in_.add(10), *cospi.add(42));
    v[11] = vshlq_s32(v[11], v_bit);

    v[12] = vmlaq_n_s32(rnding, *in_.add(3), *cospi.add(50));
    v[12] = vmlaq_n_s32(v[12], *in_.add(12), *cospi.add(14));
    v[12] = vshlq_s32(v[12], v_bit);

    v[13] = vmlaq_n_s32(rnding, *in_.add(3), *cospi.add(14));
    v[13] = vmlsq_n_s32(v[13], *in_.add(12), *cospi.add(50));
    v[13] = vshlq_s32(v[13], v_bit);

    v[14] = vmlaq_n_s32(rnding, *in_.add(1), *cospi.add(58));
    v[14] = vmlaq_n_s32(v[14], *in_.add(14), *cospi.add(6));
    v[14] = vshlq_s32(v[14], v_bit);

    v[15] = vmlaq_n_s32(rnding, *in_.add(1), *cospi.add(6));
    v[15] = vmlsq_n_s32(v[15], *in_.add(14), *cospi.add(58));
    v[15] = vshlq_s32(v[15], v_bit);

    // stage 3
    addsub_neon(
        v[0],
        v[8],
        u.as_mut_ptr(),
        u.as_mut_ptr().add(8),
        &raw const clamp_lo,
        &raw const clamp_hi,
    );
    addsub_neon(
        v[1],
        v[9],
        u.as_mut_ptr().add(1),
        u.as_mut_ptr().add(9),
        &raw const clamp_lo,
        &raw const clamp_hi,
    );
    addsub_neon(
        v[2],
        v[10],
        u.as_mut_ptr().add(2),
        u.as_mut_ptr().add(10),
        &raw const clamp_lo,
        &raw const clamp_hi,
    );
    addsub_neon(
        v[3],
        v[11],
        u.as_mut_ptr().add(3),
        u.as_mut_ptr().add(11),
        &raw const clamp_lo,
        &raw const clamp_hi,
    );
    addsub_neon(
        v[4],
        v[12],
        u.as_mut_ptr().add(4),
        u.as_mut_ptr().add(12),
        &raw const clamp_lo,
        &raw const clamp_hi,
    );
    addsub_neon(
        v[5],
        v[13],
        u.as_mut_ptr().add(5),
        u.as_mut_ptr().add(13),
        &raw const clamp_lo,
        &raw const clamp_hi,
    );
    addsub_neon(
        v[6],
        v[14],
        u.as_mut_ptr().add(6),
        u.as_mut_ptr().add(14),
        &raw const clamp_lo,
        &raw const clamp_hi,
    );
    addsub_neon(
        v[7],
        v[15],
        u.as_mut_ptr().add(7),
        u.as_mut_ptr().add(15),
        &raw const clamp_lo,
        &raw const clamp_hi,
    );

    // stage 4
    v[0] = u[0];
    v[1] = u[1];
    v[2] = u[2];
    v[3] = u[3];
    v[4] = u[4];
    v[5] = u[5];
    v[6] = u[6];
    v[7] = u[7];

    v[8] = vmlaq_n_s32(rnding, u[8], *cospi.add(8));
    v[8] = vmlaq_n_s32(v[8], u[9], *cospi.add(56));
    v[8] = vshlq_s32(v[8], v_bit);

    v[9] = vmlaq_n_s32(rnding, u[8], *cospi.add(56));
    v[9] = vmlsq_n_s32(v[9], u[9], *cospi.add(8));
    v[9] = vshlq_s32(v[9], v_bit);

    v[10] = vmlaq_n_s32(rnding, u[10], *cospi.add(40));
    v[10] = vmlaq_n_s32(v[10], u[11], *cospi.add(24));
    v[10] = vshlq_s32(v[10], v_bit);

    v[11] = vmlaq_n_s32(rnding, u[10], *cospi.add(24));
    v[11] = vmlsq_n_s32(v[11], u[11], *cospi.add(40));
    v[11] = vshlq_s32(v[11], v_bit);

    v[12] = vmlaq_n_s32(rnding, u[12], -*cospi.add(56));
    v[12] = vmlaq_n_s32(v[12], u[13], *cospi.add(8));
    v[12] = vshlq_s32(v[12], v_bit);

    v[13] = vmlaq_n_s32(rnding, u[12], *cospi.add(8));
    v[13] = vmlsq_n_s32(v[13], u[13], -*cospi.add(56));
    v[13] = vshlq_s32(v[13], v_bit);

    v[14] = vmlaq_n_s32(rnding, u[14], -*cospi.add(24));
    v[14] = vmlaq_n_s32(v[14], u[15], *cospi.add(40));
    v[14] = vshlq_s32(v[14], v_bit);

    v[15] = vmlaq_n_s32(rnding, u[14], *cospi.add(40));
    v[15] = vmlsq_n_s32(v[15], u[15], -*cospi.add(24));
    v[15] = vshlq_s32(v[15], v_bit);

    // stage 5
    addsub_neon(
        v[0],
        v[4],
        u.as_mut_ptr(),
        u.as_mut_ptr().add(4),
        &raw const clamp_lo,
        &raw const clamp_hi,
    );
    addsub_neon(
        v[1],
        v[5],
        u.as_mut_ptr().add(1),
        u.as_mut_ptr().add(5),
        &raw const clamp_lo,
        &raw const clamp_hi,
    );
    addsub_neon(
        v[2],
        v[6],
        u.as_mut_ptr().add(2),
        u.as_mut_ptr().add(6),
        &raw const clamp_lo,
        &raw const clamp_hi,
    );
    addsub_neon(
        v[3],
        v[7],
        u.as_mut_ptr().add(3),
        u.as_mut_ptr().add(7),
        &raw const clamp_lo,
        &raw const clamp_hi,
    );
    addsub_neon(
        v[8],
        v[12],
        u.as_mut_ptr().add(8),
        u.as_mut_ptr().add(12),
        &raw const clamp_lo,
        &raw const clamp_hi,
    );
    addsub_neon(
        v[9],
        v[13],
        u.as_mut_ptr().add(9),
        u.as_mut_ptr().add(13),
        &raw const clamp_lo,
        &raw const clamp_hi,
    );
    addsub_neon(
        v[10],
        v[14],
        u.as_mut_ptr().add(10),
        u.as_mut_ptr().add(14),
        &raw const clamp_lo,
        &raw const clamp_hi,
    );
    addsub_neon(
        v[11],
        v[15],
        u.as_mut_ptr().add(11),
        u.as_mut_ptr().add(15),
        &raw const clamp_lo,
        &raw const clamp_hi,
    );

    // stage 6
    v[0] = u[0];
    v[1] = u[1];
    v[2] = u[2];
    v[3] = u[3];

    v[4] = vmlaq_n_s32(rnding, u[4], *cospi.add(16));
    v[4] = vmlaq_n_s32(v[4], u[5], *cospi.add(48));
    v[4] = vshlq_s32(v[4], v_bit);

    v[5] = vmlaq_n_s32(rnding, u[4], *cospi.add(48));
    v[5] = vmlsq_n_s32(v[5], u[5], *cospi.add(16));
    v[5] = vshlq_s32(v[5], v_bit);

    v[6] = vmlaq_n_s32(rnding, u[6], -*cospi.add(48));
    v[6] = vmlaq_n_s32(v[6], u[7], *cospi.add(16));
    v[6] = vshlq_s32(v[6], v_bit);

    v[7] = vmlaq_n_s32(rnding, u[6], *cospi.add(16));
    v[7] = vmlsq_n_s32(v[7], u[7], -*cospi.add(48));
    v[7] = vshlq_s32(v[7], v_bit);

    v[8] = u[8];
    v[9] = u[9];
    v[10] = u[10];
    v[11] = u[11];

    v[12] = vmlaq_n_s32(rnding, u[12], *cospi.add(16));
    v[12] = vmlaq_n_s32(v[12], u[13], *cospi.add(48));
    v[12] = vshlq_s32(v[12], v_bit);

    v[13] = vmlaq_n_s32(rnding, u[12], *cospi.add(48));
    v[13] = vmlsq_n_s32(v[13], u[13], *cospi.add(16));
    v[13] = vshlq_s32(v[13], v_bit);

    v[14] = vmlaq_n_s32(rnding, u[14], -*cospi.add(48));
    v[14] = vmlaq_n_s32(v[14], u[15], *cospi.add(16));
    v[14] = vshlq_s32(v[14], v_bit);

    v[15] = vmlaq_n_s32(rnding, u[14], *cospi.add(16));
    v[15] = vmlsq_n_s32(v[15], u[15], -*cospi.add(48));
    v[15] = vshlq_s32(v[15], v_bit);

    // stage 7
    addsub_neon(
        v[0],
        v[2],
        u.as_mut_ptr(),
        u.as_mut_ptr().add(2),
        &raw const clamp_lo,
        &raw const clamp_hi,
    );
    addsub_neon(
        v[1],
        v[3],
        u.as_mut_ptr().add(1),
        u.as_mut_ptr().add(3),
        &raw const clamp_lo,
        &raw const clamp_hi,
    );
    addsub_neon(
        v[4],
        v[6],
        u.as_mut_ptr().add(4),
        u.as_mut_ptr().add(6),
        &raw const clamp_lo,
        &raw const clamp_hi,
    );
    addsub_neon(
        v[5],
        v[7],
        u.as_mut_ptr().add(5),
        u.as_mut_ptr().add(7),
        &raw const clamp_lo,
        &raw const clamp_hi,
    );
    addsub_neon(
        v[8],
        v[10],
        u.as_mut_ptr().add(8),
        u.as_mut_ptr().add(10),
        &raw const clamp_lo,
        &raw const clamp_hi,
    );
    addsub_neon(
        v[9],
        v[11],
        u.as_mut_ptr().add(9),
        u.as_mut_ptr().add(11),
        &raw const clamp_lo,
        &raw const clamp_hi,
    );
    addsub_neon(
        v[12],
        v[14],
        u.as_mut_ptr().add(12),
        u.as_mut_ptr().add(14),
        &raw const clamp_lo,
        &raw const clamp_hi,
    );
    addsub_neon(
        v[13],
        v[15],
        u.as_mut_ptr().add(13),
        u.as_mut_ptr().add(15),
        &raw const clamp_lo,
        &raw const clamp_hi,
    );

    // stage 8
    v[0] = u[0];
    v[1] = u[1];

    y = vmlaq_n_s32(rnding, u[2], *cospi.add(32));
    x = vmulq_n_s32(u[3], *cospi.add(32));
    v[2] = vaddq_s32(y, x);
    v[2] = vshlq_s32(v[2], v_bit);

    v[3] = vsubq_s32(y, x);
    v[3] = vshlq_s32(v[3], v_bit);

    v[4] = u[4];
    v[5] = u[5];

    y = vmlaq_n_s32(rnding, u[6], *cospi.add(32));
    x = vmulq_n_s32(u[7], *cospi.add(32));
    v[6] = vaddq_s32(y, x);
    v[6] = vshlq_s32(v[6], v_bit);

    v[7] = vsubq_s32(y, x);
    v[7] = vshlq_s32(v[7], v_bit);

    v[8] = u[8];
    v[9] = u[9];

    y = vmlaq_n_s32(rnding, u[10], *cospi.add(32));
    x = vmulq_n_s32(u[11], *cospi.add(32));
    v[10] = vaddq_s32(y, x);
    v[10] = vshlq_s32(v[10], v_bit);

    v[11] = vsubq_s32(y, x);
    v[11] = vshlq_s32(v[11], v_bit);

    v[12] = u[12];
    v[13] = u[13];

    y = vmlaq_n_s32(rnding, u[14], *cospi.add(32));
    x = vmulq_n_s32(u[15], *cospi.add(32));
    v[14] = vaddq_s32(y, x);
    v[14] = vshlq_s32(v[14], v_bit);

    v[15] = vsubq_s32(y, x);
    v[15] = vshlq_s32(v[15], v_bit);

    // stage 9
    if do_cols != 0 {
        *out = v[0];
        *out.add(1) = vsubq_s32(zero, v[8]);
        *out.add(2) = v[12];
        *out.add(3) = vsubq_s32(zero, v[4]);
        *out.add(4) = v[6];
        *out.add(5) = vsubq_s32(zero, v[14]);
        *out.add(6) = v[10];
        *out.add(7) = vsubq_s32(zero, v[2]);
        *out.add(8) = v[3];
        *out.add(9) = vsubq_s32(zero, v[11]);
        *out.add(10) = v[15];
        *out.add(11) = vsubq_s32(zero, v[7]);
        *out.add(12) = v[5];
        *out.add(13) = vsubq_s32(zero, v[13]);
        *out.add(14) = v[9];
        *out.add(15) = vsubq_s32(zero, v[1]);
    } else {
        let log_range_out: i32 = get_log_range_out(bd);
        let clamp_lo_out: int32x4_t = vdupq_n_s32(-(1 << (log_range_out - 1)));
        let clamp_hi_out: int32x4_t = vdupq_n_s32((1 << (log_range_out - 1)) - 1);
        let v_shift: int32x4_t = vdupq_n_s32(-out_shift);
        let mut offset: int32x4_t = vdupq_n_s32((1 << out_shift) >> 1);
        neg_shift_neon(
            v.as_mut_ptr(),
            v.as_mut_ptr().add(8),
            out,
            out.add(1),
            &raw const clamp_lo_out,
            &raw const clamp_hi_out,
            &raw const v_shift,
            &raw mut offset,
        );
        neg_shift_neon(
            v.as_mut_ptr().add(12),
            v.as_mut_ptr().add(4),
            out.add(2),
            out.add(3),
            &raw const clamp_lo_out,
            &raw const clamp_hi_out,
            &raw const v_shift,
            &raw mut offset,
        );
        neg_shift_neon(
            v.as_mut_ptr().add(6),
            v.as_mut_ptr().add(14),
            out.add(4),
            out.add(5),
            &raw const clamp_lo_out,
            &raw const clamp_hi_out,
            &raw const v_shift,
            &raw mut offset,
        );
        neg_shift_neon(
            v.as_mut_ptr().add(10),
            v.as_mut_ptr().add(2),
            out.add(6),
            out.add(7),
            &raw const clamp_lo_out,
            &raw const clamp_hi_out,
            &raw const v_shift,
            &raw mut offset,
        );
        neg_shift_neon(
            v.as_mut_ptr().add(3),
            v.as_mut_ptr().add(11),
            out.add(8),
            out.add(9),
            &raw const clamp_lo_out,
            &raw const clamp_hi_out,
            &raw const v_shift,
            &raw mut offset,
        );
        neg_shift_neon(
            v.as_mut_ptr().add(15),
            v.as_mut_ptr().add(7),
            out.add(10),
            out.add(11),
            &raw const clamp_lo_out,
            &raw const clamp_hi_out,
            &raw const v_shift,
            &raw mut offset,
        );
        neg_shift_neon(
            v.as_mut_ptr().add(5),
            v.as_mut_ptr().add(13),
            out.add(12),
            out.add(13),
            &raw const clamp_lo_out,
            &raw const clamp_hi_out,
            &raw const v_shift,
            &raw mut offset,
        );
        neg_shift_neon(
            v.as_mut_ptr().add(9),
            v.as_mut_ptr().add(1),
            out.add(14),
            out.add(15),
            &raw const clamp_lo_out,
            &raw const clamp_hi_out,
            &raw const v_shift,
            &raw mut offset,
        );
    }
}

unsafe fn iidentity16_neon(
    mut in_: *mut int32x4_t,
    mut out: *mut int32x4_t,
    mut bit: i32,
    mut do_cols: i32,
    mut bd: i32,
    mut out_shift: i32,
) {
    let _ = bit;
    let mut fact: int32x2_t = vdup_n_s32(2 * NewSqrt2);
    let mut a0: int32x4x2_t = core::mem::zeroed();
    let mut zero: int32x4_t = vdupq_n_s32(0);
    let rnding: int64x2_t = vdupq_n_s64((1 << (NewSqrt2Bits - 1)) as i64);
    for i in 0..16 {
        a0.0 = vreinterpretq_s32_s64(vmlal_s32(
            rnding,
            vmovn_s64(vreinterpretq_s64_s32(*in_.offset(i as isize))),
            fact,
        ));
        a0.0 = vreinterpretq_s32_s64(vshrq_n_s64::<NewSqrt2Bits>(vreinterpretq_s64_s32(a0.0)));
        a0.1 = vextq_s32::<1>(*in_.offset(i as isize), zero);
        a0.1 =
            vreinterpretq_s32_s64(vmlal_s32(rnding, vmovn_s64(vreinterpretq_s64_s32(a0.1)), fact));
        a0.1 = vreinterpretq_s32_s64(vshrq_n_s64::<NewSqrt2Bits>(vreinterpretq_s64_s32(a0.1)));
        a0 = vzipq_s32(a0.0, a0.1);
        *out.offset(i as isize) = vreinterpretq_s32_s64(vzip1q_s64(
            vreinterpretq_s64_s32(a0.0),
            vreinterpretq_s64_s32(a0.1),
        ));
    }

    if !(do_cols != 0) {
        let log_range: i32 = get_log_range_out(bd);
        let clamp_lo: int32x4_t = vdupq_n_s32(-(1 << (log_range - 1)));
        let clamp_hi: int32x4_t = vdupq_n_s32((1 << (log_range - 1)) - 1);
        round_shift_8x8(out, out_shift);
        highbd_clamp_s32_neon(out, out, &raw const clamp_lo, &raw const clamp_hi, 16);
    }
}

#[inline(always)]
unsafe fn idct64_stage8_neon(
    mut u: *mut int32x4_t,
    mut cospi: *const i32,
    mut clamp_lo: *const int32x4_t,
    mut clamp_hi: *const int32x4_t,
    mut v_bit: *const int32x4_t,
    mut rnding: *const int32x4_t,
) {
    let mut i: i32 = 0;
    let mut temp1: int32x4_t = core::mem::zeroed();
    let mut temp2: int32x4_t = core::mem::zeroed();
    let mut temp3: int32x4_t = core::mem::zeroed();
    let mut temp4: int32x4_t = core::mem::zeroed();
    temp1 =
        half_btf_neon_mode10_r(cospi.add(32), u.add(10), cospi.add(32), u.add(13), v_bit, rnding);
    *u.add(13) = half_btf_neon_r(cospi.add(32), u.add(10), cospi.add(32), u.add(13), v_bit, rnding);
    *u.add(10) = temp1;
    temp2 =
        half_btf_neon_mode10_r(cospi.add(32), u.add(11), cospi.add(32), u.add(12), v_bit, rnding);
    *u.add(12) = half_btf_neon_r(cospi.add(32), u.add(11), cospi.add(32), u.add(12), v_bit, rnding);
    *u.add(11) = temp2;

    i = 16;
    while i < 20 {
        addsub_neon(
            *u.offset(i as isize),
            *u.offset((i ^ 7) as isize),
            u.offset(i as isize),
            u.offset((i ^ 7) as isize),
            clamp_lo,
            clamp_hi,
        );
        addsub_neon(
            *u.offset((i ^ 15) as isize),
            *u.offset((i ^ 8) as isize),
            u.offset((i ^ 15) as isize),
            u.offset((i ^ 8) as isize),
            clamp_lo,
            clamp_hi,
        );
        i += 1;
    }

    temp1 =
        half_btf_neon_mode10_r(cospi.add(16), u.add(36), cospi.add(48), u.add(59), v_bit, rnding);
    temp2 =
        half_btf_neon_mode10_r(cospi.add(16), u.add(37), cospi.add(48), u.add(58), v_bit, rnding);
    temp3 =
        half_btf_neon_mode10_r(cospi.add(16), u.add(38), cospi.add(48), u.add(57), v_bit, rnding);
    temp4 =
        half_btf_neon_mode10_r(cospi.add(16), u.add(39), cospi.add(48), u.add(56), v_bit, rnding);
    *u.add(56) = half_btf_neon_r(cospi.add(48), u.add(39), cospi.add(16), u.add(56), v_bit, rnding);
    *u.add(57) = half_btf_neon_r(cospi.add(48), u.add(38), cospi.add(16), u.add(57), v_bit, rnding);
    *u.add(58) = half_btf_neon_r(cospi.add(48), u.add(37), cospi.add(16), u.add(58), v_bit, rnding);
    *u.add(59) = half_btf_neon_r(cospi.add(48), u.add(36), cospi.add(16), u.add(59), v_bit, rnding);
    *u.add(36) = temp1;
    *u.add(37) = temp2;
    *u.add(38) = temp3;
    *u.add(39) = temp4;

    temp1 =
        half_btf_neon_mode11_r(cospi.add(48), u.add(40), cospi.add(16), u.add(55), v_bit, rnding);
    temp2 =
        half_btf_neon_mode11_r(cospi.add(48), u.add(41), cospi.add(16), u.add(54), v_bit, rnding);
    temp3 =
        half_btf_neon_mode11_r(cospi.add(48), u.add(42), cospi.add(16), u.add(53), v_bit, rnding);
    temp4 =
        half_btf_neon_mode11_r(cospi.add(48), u.add(43), cospi.add(16), u.add(52), v_bit, rnding);
    *u.add(52) =
        half_btf_neon_mode10_r(cospi.add(16), u.add(43), cospi.add(48), u.add(52), v_bit, rnding);
    *u.add(53) =
        half_btf_neon_mode10_r(cospi.add(16), u.add(42), cospi.add(48), u.add(53), v_bit, rnding);
    *u.add(54) =
        half_btf_neon_mode10_r(cospi.add(16), u.add(41), cospi.add(48), u.add(54), v_bit, rnding);
    *u.add(55) =
        half_btf_neon_mode10_r(cospi.add(16), u.add(40), cospi.add(48), u.add(55), v_bit, rnding);
    *u.add(40) = temp1;
    *u.add(41) = temp2;
    *u.add(42) = temp3;
    *u.add(43) = temp4;
}

#[inline(always)]
unsafe fn idct64_stage9_neon(
    mut u: *mut int32x4_t,
    mut cospi: *const i32,
    mut clamp_lo: *const int32x4_t,
    mut clamp_hi: *const int32x4_t,
    mut v_bit: *const int32x4_t,
    mut rnding: *const int32x4_t,
) {
    let mut i: i32 = 0;
    let mut temp1: int32x4_t = core::mem::zeroed();
    let mut temp2: int32x4_t = core::mem::zeroed();
    let mut temp3: int32x4_t = core::mem::zeroed();
    let mut temp4: int32x4_t = core::mem::zeroed();
    i = 0;
    while i < 8 {
        addsub_neon(
            *u.offset(i as isize),
            *u.offset((15 - i) as isize),
            u.offset(i as isize),
            u.offset((15 - i) as isize),
            clamp_lo,
            clamp_hi,
        );
        i += 1;
    }
    temp1 =
        half_btf_neon_mode10_r(cospi.add(32), u.add(20), cospi.add(32), u.add(27), v_bit, rnding);
    temp2 =
        half_btf_neon_mode10_r(cospi.add(32), u.add(21), cospi.add(32), u.add(26), v_bit, rnding);
    temp3 =
        half_btf_neon_mode10_r(cospi.add(32), u.add(22), cospi.add(32), u.add(25), v_bit, rnding);
    temp4 =
        half_btf_neon_mode10_r(cospi.add(32), u.add(23), cospi.add(32), u.add(24), v_bit, rnding);
    *u.add(24) = half_btf_neon_r(cospi.add(32), u.add(23), cospi.add(32), u.add(24), v_bit, rnding);
    *u.add(25) = half_btf_neon_r(cospi.add(32), u.add(22), cospi.add(32), u.add(25), v_bit, rnding);
    *u.add(26) = half_btf_neon_r(cospi.add(32), u.add(21), cospi.add(32), u.add(26), v_bit, rnding);
    *u.add(27) = half_btf_neon_r(cospi.add(32), u.add(20), cospi.add(32), u.add(27), v_bit, rnding);
    *u.add(20) = temp1;
    *u.add(21) = temp2;
    *u.add(22) = temp3;
    *u.add(23) = temp4;
    i = 32;
    while i < 40 {
        addsub_neon(
            *u.offset(i as isize),
            *u.offset((i ^ 15) as isize),
            u.offset(i as isize),
            u.offset((i ^ 15) as isize),
            clamp_lo,
            clamp_hi,
        );
        i += 1;
    }

    i = 48;
    while i < 56 {
        addsub_neon(
            *u.offset((i ^ 15) as isize),
            *u.offset(i as isize),
            u.offset((i ^ 15) as isize),
            u.offset(i as isize),
            clamp_lo,
            clamp_hi,
        );
        i += 1;
    }
}

#[inline(always)]
unsafe fn idct64_stage10_neon(
    mut u: *mut int32x4_t,
    mut cospi: *const i32,
    mut clamp_lo: *const int32x4_t,
    mut clamp_hi: *const int32x4_t,
    mut v_bit: *const int32x4_t,
    mut rnding: *const int32x4_t,
) {
    let mut temp1: int32x4_t = core::mem::zeroed();
    let mut temp2: int32x4_t = core::mem::zeroed();
    let mut temp3: int32x4_t = core::mem::zeroed();
    let mut temp4: int32x4_t = core::mem::zeroed();
    for i in 0..16 {
        addsub_neon(
            *u.offset(i as isize),
            *u.offset((31 - i) as isize),
            u.offset(i as isize),
            u.offset((31 - i) as isize),
            clamp_lo,
            clamp_hi,
        );
    }
    temp1 =
        half_btf_neon_mode10_r(cospi.add(32), u.add(40), cospi.add(32), u.add(55), v_bit, rnding);
    temp2 =
        half_btf_neon_mode10_r(cospi.add(32), u.add(41), cospi.add(32), u.add(54), v_bit, rnding);
    temp3 =
        half_btf_neon_mode10_r(cospi.add(32), u.add(42), cospi.add(32), u.add(53), v_bit, rnding);
    temp4 =
        half_btf_neon_mode10_r(cospi.add(32), u.add(43), cospi.add(32), u.add(52), v_bit, rnding);
    *u.add(52) = half_btf_neon_r(cospi.add(32), u.add(43), cospi.add(32), u.add(52), v_bit, rnding);
    *u.add(53) = half_btf_neon_r(cospi.add(32), u.add(42), cospi.add(32), u.add(53), v_bit, rnding);
    *u.add(54) = half_btf_neon_r(cospi.add(32), u.add(41), cospi.add(32), u.add(54), v_bit, rnding);
    *u.add(55) = half_btf_neon_r(cospi.add(32), u.add(40), cospi.add(32), u.add(55), v_bit, rnding);
    *u.add(40) = temp1;
    *u.add(41) = temp2;
    *u.add(42) = temp3;
    *u.add(43) = temp4;

    temp1 =
        half_btf_neon_mode10_r(cospi.add(32), u.add(44), cospi.add(32), u.add(51), v_bit, rnding);
    temp2 =
        half_btf_neon_mode10_r(cospi.add(32), u.add(45), cospi.add(32), u.add(50), v_bit, rnding);
    temp3 =
        half_btf_neon_mode10_r(cospi.add(32), u.add(46), cospi.add(32), u.add(49), v_bit, rnding);
    temp4 =
        half_btf_neon_mode10_r(cospi.add(32), u.add(47), cospi.add(32), u.add(48), v_bit, rnding);
    *u.add(48) = half_btf_neon_r(cospi.add(32), u.add(47), cospi.add(32), u.add(48), v_bit, rnding);
    *u.add(49) = half_btf_neon_r(cospi.add(32), u.add(46), cospi.add(32), u.add(49), v_bit, rnding);
    *u.add(50) = half_btf_neon_r(cospi.add(32), u.add(45), cospi.add(32), u.add(50), v_bit, rnding);
    *u.add(51) = half_btf_neon_r(cospi.add(32), u.add(44), cospi.add(32), u.add(51), v_bit, rnding);
    *u.add(44) = temp1;
    *u.add(45) = temp2;
    *u.add(46) = temp3;
    *u.add(47) = temp4;
}

#[inline(always)]
unsafe fn idct64_stage11_neon(
    mut u: *mut int32x4_t,
    mut out: *mut int32x4_t,
    mut do_cols: i32,
    mut bd: i32,
    mut out_shift: i32,
    mut clamp_lo: *const int32x4_t,
    mut clamp_hi: *const int32x4_t,
) {
    for i in 0..32 {
        addsub_neon(
            *u.offset(i as isize),
            *u.offset((63 - i) as isize),
            out.offset(i as isize),
            out.add(63).offset(-(i as isize)),
            clamp_lo,
            clamp_hi,
        );
    }

    if !(do_cols != 0) {
        let log_range_out: i32 = get_log_range_out(bd);
        let clamp_lo_out: int32x4_t = vdupq_n_s32(-(1 << (log_range_out - 1)));
        let clamp_hi_out: int32x4_t = vdupq_n_s32((1 << (log_range_out - 1)) - 1);
        for i in (0..64).step_by(4) {
            round_shift_4x4(out.offset(i as isize), out_shift);
            highbd_clamp_s32_neon(
                out.offset(i as isize),
                out.offset(i as isize),
                &raw const clamp_lo_out,
                &raw const clamp_hi_out,
                4,
            );
        }
    }
}

unsafe fn idct64x64_low1_neon(
    mut in_: *mut int32x4_t,
    mut out: *mut int32x4_t,
    mut bit: i32,
    mut do_cols: i32,
    mut bd: i32,
    mut out_shift: i32,
) {
    let mut cospi: *const i32 = cospi_arr(bit);
    let log_range: i32 = get_log_range(bd, do_cols);
    let mut clamp_lo: int32x4_t = vdupq_n_s32(-(1 << (log_range - 1)));
    let mut clamp_hi: int32x4_t = vdupq_n_s32((1 << (log_range - 1)) - 1);

    let v_bit: int32x4_t = vdupq_n_s32(-bit);
    let rnding: int32x4_t = vdupq_n_s32(1 << (bit - 1));
    {
        let mut x: int32x4_t = core::mem::zeroed();

        // stage 1
        // stage 2
        // stage 3
        // stage 4
        // stage 5
        // stage 6
        x = half_btf_0_neon_r(cospi.add(32), in_, &raw const v_bit, &raw const rnding);

        // stage 8
        // stage 9
        // stage 10
        // stage 11
        if !(do_cols != 0) {
            let log_range_out: i32 = get_log_range_out(bd);
            clamp_lo = vdupq_n_s32(-(1 << (log_range_out - 1)));
            clamp_hi = vdupq_n_s32((1 << (log_range_out - 1)) - 1);
            if out_shift != 0 {
                let mut offset: int32x4_t = vdupq_n_s32((1 << out_shift) >> 1);
                x = vaddq_s32(x, offset);
                x = vshlq_s32(x, vdupq_n_s32(-out_shift));
            }
        }
        x = vmaxq_s32(x, clamp_lo);
        x = vminq_s32(x, clamp_hi);
        *out = x;
        *out.add(1) = x;
        *out.add(2) = x;
        *out.add(3) = x;
        *out.add(4) = x;
        *out.add(5) = x;
        *out.add(6) = x;
        *out.add(7) = x;
        *out.add(8) = x;
        *out.add(9) = x;
        *out.add(10) = x;
        *out.add(11) = x;
        *out.add(12) = x;
        *out.add(13) = x;
        *out.add(14) = x;
        *out.add(15) = x;
        *out.add(16) = x;
        *out.add(17) = x;
        *out.add(18) = x;
        *out.add(19) = x;
        *out.add(20) = x;
        *out.add(21) = x;
        *out.add(22) = x;
        *out.add(23) = x;
        *out.add(24) = x;
        *out.add(25) = x;
        *out.add(26) = x;
        *out.add(27) = x;
        *out.add(28) = x;
        *out.add(29) = x;
        *out.add(30) = x;
        *out.add(31) = x;
        *out.add(32) = x;
        *out.add(33) = x;
        *out.add(34) = x;
        *out.add(35) = x;
        *out.add(36) = x;
        *out.add(37) = x;
        *out.add(38) = x;
        *out.add(39) = x;
        *out.add(40) = x;
        *out.add(41) = x;
        *out.add(42) = x;
        *out.add(43) = x;
        *out.add(44) = x;
        *out.add(45) = x;
        *out.add(46) = x;
        *out.add(47) = x;
        *out.add(48) = x;
        *out.add(49) = x;
        *out.add(50) = x;
        *out.add(51) = x;
        *out.add(52) = x;
        *out.add(53) = x;
        *out.add(54) = x;
        *out.add(55) = x;
        *out.add(56) = x;
        *out.add(57) = x;
        *out.add(58) = x;
        *out.add(59) = x;
        *out.add(60) = x;
        *out.add(61) = x;
        *out.add(62) = x;
        *out.add(63) = x;
    }
}

unsafe fn idct64x64_low8_neon(
    mut in_: *mut int32x4_t,
    mut out: *mut int32x4_t,
    mut bit: i32,
    mut do_cols: i32,
    mut bd: i32,
    mut out_shift: i32,
) {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut cospi: *const i32 = cospi_arr(bit);
    let log_range: i32 = get_log_range(bd, do_cols);
    let clamp_lo: int32x4_t = vdupq_n_s32(-(1 << (log_range - 1)));
    let clamp_hi: int32x4_t = vdupq_n_s32((1 << (log_range - 1)) - 1);
    let v_bit: int32x4_t = vdupq_n_s32(-bit);
    let rnding: int32x4_t = vdupq_n_s32(1 << (bit - 1));
    {
        let mut u: [int32x4_t; 64] = core::mem::zeroed();

        // stage 1
        u[0] = *in_;
        u[8] = *in_.add(4);
        u[16] = *in_.add(2);
        u[24] = *in_.add(6);
        u[32] = *in_.add(1);
        u[40] = *in_.add(5);
        u[48] = *in_.add(3);
        u[56] = *in_.add(7);

        // stage 2
        u[63] = half_btf_0_neon_r(
            cospi.add(1),
            u.as_mut_ptr().add(32),
            &raw const v_bit,
            &raw const rnding,
        );
        u[32] = half_btf_0_neon_r(
            cospi.add(63),
            u.as_mut_ptr().add(32),
            &raw const v_bit,
            &raw const rnding,
        );
        u[39] = half_btf_0_m_neon_r(
            cospi.add(57),
            u.as_mut_ptr().add(56),
            &raw const v_bit,
            &raw const rnding,
        );
        u[56] = half_btf_0_neon_r(
            cospi.add(7),
            u.as_mut_ptr().add(56),
            &raw const v_bit,
            &raw const rnding,
        );
        u[55] = half_btf_0_neon_r(
            cospi.add(5),
            u.as_mut_ptr().add(40),
            &raw const v_bit,
            &raw const rnding,
        );
        u[40] = half_btf_0_neon_r(
            cospi.add(59),
            u.as_mut_ptr().add(40),
            &raw const v_bit,
            &raw const rnding,
        );
        u[47] = half_btf_0_m_neon_r(
            cospi.add(61),
            u.as_mut_ptr().add(48),
            &raw const v_bit,
            &raw const rnding,
        );
        u[48] = half_btf_0_neon_r(
            cospi.add(3),
            u.as_mut_ptr().add(48),
            &raw const v_bit,
            &raw const rnding,
        );

        // stage 3
        u[31] = half_btf_0_neon_r(
            cospi.add(2),
            u.as_mut_ptr().add(16),
            &raw const v_bit,
            &raw const rnding,
        );
        u[16] = half_btf_0_neon_r(
            cospi.add(62),
            u.as_mut_ptr().add(16),
            &raw const v_bit,
            &raw const rnding,
        );
        u[23] = half_btf_0_m_neon_r(
            cospi.add(58),
            u.as_mut_ptr().add(24),
            &raw const v_bit,
            &raw const rnding,
        );
        u[24] = half_btf_0_neon_r(
            cospi.add(6),
            u.as_mut_ptr().add(24),
            &raw const v_bit,
            &raw const rnding,
        );
        u[33] = u[32];
        u[38] = u[39];
        u[41] = u[40];
        u[46] = u[47];
        u[49] = u[48];
        u[54] = u[55];
        u[57] = u[56];
        u[62] = u[63];

        // stage 4
        let mut temp1: int32x4_t = core::mem::zeroed();
        let mut temp2: int32x4_t = core::mem::zeroed();
        u[15] = half_btf_0_neon_r(
            cospi.add(4),
            u.as_mut_ptr().add(8),
            &raw const v_bit,
            &raw const rnding,
        );
        u[8] = half_btf_0_neon_r(
            cospi.add(60),
            u.as_mut_ptr().add(8),
            &raw const v_bit,
            &raw const rnding,
        );
        u[17] = u[16];
        u[22] = u[23];
        u[25] = u[24];
        u[30] = u[31];

        temp1 = half_btf_neon_mode10_r(
            cospi.add(4),
            u.as_mut_ptr().add(33),
            cospi.add(60),
            u.as_mut_ptr().add(62),
            &raw const v_bit,
            &raw const rnding,
        );
        u[62] = half_btf_neon_r(
            cospi.add(60),
            u.as_mut_ptr().add(33),
            cospi.add(4),
            u.as_mut_ptr().add(62),
            &raw const v_bit,
            &raw const rnding,
        );
        u[33] = temp1;

        temp2 = half_btf_neon_mode10_r(
            cospi.add(36),
            u.as_mut_ptr().add(38),
            cospi.add(28),
            u.as_mut_ptr().add(57),
            &raw const v_bit,
            &raw const rnding,
        );
        u[38] = half_btf_neon_mode11_r(
            cospi.add(28),
            u.as_mut_ptr().add(38),
            cospi.add(36),
            u.as_mut_ptr().add(57),
            &raw const v_bit,
            &raw const rnding,
        );
        u[57] = temp2;

        temp1 = half_btf_neon_mode10_r(
            cospi.add(20),
            u.as_mut_ptr().add(41),
            cospi.add(44),
            u.as_mut_ptr().add(54),
            &raw const v_bit,
            &raw const rnding,
        );
        u[54] = half_btf_neon_r(
            cospi.add(44),
            u.as_mut_ptr().add(41),
            cospi.add(20),
            u.as_mut_ptr().add(54),
            &raw const v_bit,
            &raw const rnding,
        );
        u[41] = temp1;

        temp2 = half_btf_neon_mode11_r(
            cospi.add(12),
            u.as_mut_ptr().add(46),
            cospi.add(52),
            u.as_mut_ptr().add(49),
            &raw const v_bit,
            &raw const rnding,
        );
        u[49] = half_btf_neon_mode10_r(
            cospi.add(52),
            u.as_mut_ptr().add(46),
            cospi.add(12),
            u.as_mut_ptr().add(49),
            &raw const v_bit,
            &raw const rnding,
        );
        u[46] = temp2;

        // stage 5
        u[9] = u[8];
        u[14] = u[15];

        temp1 = half_btf_neon_mode10_r(
            cospi.add(8),
            u.as_mut_ptr().add(17),
            cospi.add(56),
            u.as_mut_ptr().add(30),
            &raw const v_bit,
            &raw const rnding,
        );
        u[30] = half_btf_neon_r(
            cospi.add(56),
            u.as_mut_ptr().add(17),
            cospi.add(8),
            u.as_mut_ptr().add(30),
            &raw const v_bit,
            &raw const rnding,
        );
        u[17] = temp1;

        temp2 = half_btf_neon_mode11_r(
            cospi.add(24),
            u.as_mut_ptr().add(22),
            cospi.add(40),
            u.as_mut_ptr().add(25),
            &raw const v_bit,
            &raw const rnding,
        );
        u[25] = half_btf_neon_mode10_r(
            cospi.add(40),
            u.as_mut_ptr().add(22),
            cospi.add(24),
            u.as_mut_ptr().add(25),
            &raw const v_bit,
            &raw const rnding,
        );
        u[22] = temp2;

        u[35] = u[32];
        u[34] = u[33];
        u[36] = u[39];
        u[37] = u[38];
        u[43] = u[40];
        u[42] = u[41];
        u[44] = u[47];
        u[45] = u[46];
        u[51] = u[48];
        u[50] = u[49];
        u[52] = u[55];
        u[53] = u[54];
        u[59] = u[56];
        u[58] = u[57];
        u[60] = u[63];
        u[61] = u[62];

        // stage 6
        temp1 =
            half_btf_0_neon_r(cospi.add(32), u.as_mut_ptr(), &raw const v_bit, &raw const rnding);
        u[1] =
            half_btf_0_neon_r(cospi.add(32), u.as_mut_ptr(), &raw const v_bit, &raw const rnding);
        u[0] = temp1;

        temp2 = half_btf_neon_mode10_r(
            cospi.add(16),
            u.as_mut_ptr().add(9),
            cospi.add(48),
            u.as_mut_ptr().add(14),
            &raw const v_bit,
            &raw const rnding,
        );
        u[14] = half_btf_neon_r(
            cospi.add(48),
            u.as_mut_ptr().add(9),
            cospi.add(16),
            u.as_mut_ptr().add(14),
            &raw const v_bit,
            &raw const rnding,
        );
        u[9] = temp2;
        u[19] = u[16];
        u[18] = u[17];
        u[20] = u[23];
        u[21] = u[22];
        u[27] = u[24];
        u[26] = u[25];
        u[28] = u[31];
        u[29] = u[30];

        temp1 = half_btf_neon_mode10_r(
            cospi.add(8),
            u.as_mut_ptr().add(34),
            cospi.add(56),
            u.as_mut_ptr().add(61),
            &raw const v_bit,
            &raw const rnding,
        );
        u[61] = half_btf_neon_r(
            cospi.add(56),
            u.as_mut_ptr().add(34),
            cospi.add(8),
            u.as_mut_ptr().add(61),
            &raw const v_bit,
            &raw const rnding,
        );
        u[34] = temp1;
        temp2 = half_btf_neon_mode10_r(
            cospi.add(8),
            u.as_mut_ptr().add(35),
            cospi.add(56),
            u.as_mut_ptr().add(60),
            &raw const v_bit,
            &raw const rnding,
        );
        u[60] = half_btf_neon_r(
            cospi.add(56),
            u.as_mut_ptr().add(35),
            cospi.add(8),
            u.as_mut_ptr().add(60),
            &raw const v_bit,
            &raw const rnding,
        );
        u[35] = temp2;
        temp1 = half_btf_neon_mode11_r(
            cospi.add(56),
            u.as_mut_ptr().add(36),
            cospi.add(8),
            u.as_mut_ptr().add(59),
            &raw const v_bit,
            &raw const rnding,
        );
        u[59] = half_btf_neon_mode10_r(
            cospi.add(8),
            u.as_mut_ptr().add(36),
            cospi.add(56),
            u.as_mut_ptr().add(59),
            &raw const v_bit,
            &raw const rnding,
        );
        u[36] = temp1;
        temp2 = half_btf_neon_mode11_r(
            cospi.add(56),
            u.as_mut_ptr().add(37),
            cospi.add(8),
            u.as_mut_ptr().add(58),
            &raw const v_bit,
            &raw const rnding,
        );
        u[58] = half_btf_neon_mode10_r(
            cospi.add(8),
            u.as_mut_ptr().add(37),
            cospi.add(56),
            u.as_mut_ptr().add(58),
            &raw const v_bit,
            &raw const rnding,
        );
        u[37] = temp2;
        temp1 = half_btf_neon_mode10_r(
            cospi.add(40),
            u.as_mut_ptr().add(42),
            cospi.add(24),
            u.as_mut_ptr().add(53),
            &raw const v_bit,
            &raw const rnding,
        );
        u[53] = half_btf_neon_r(
            cospi.add(24),
            u.as_mut_ptr().add(42),
            cospi.add(40),
            u.as_mut_ptr().add(53),
            &raw const v_bit,
            &raw const rnding,
        );
        u[42] = temp1;
        temp2 = half_btf_neon_mode10_r(
            cospi.add(40),
            u.as_mut_ptr().add(43),
            cospi.add(24),
            u.as_mut_ptr().add(52),
            &raw const v_bit,
            &raw const rnding,
        );
        u[52] = half_btf_neon_r(
            cospi.add(24),
            u.as_mut_ptr().add(43),
            cospi.add(40),
            u.as_mut_ptr().add(52),
            &raw const v_bit,
            &raw const rnding,
        );
        u[43] = temp2;
        temp1 = half_btf_neon_mode11_r(
            cospi.add(24),
            u.as_mut_ptr().add(44),
            cospi.add(40),
            u.as_mut_ptr().add(51),
            &raw const v_bit,
            &raw const rnding,
        );
        u[51] = half_btf_neon_mode10_r(
            cospi.add(40),
            u.as_mut_ptr().add(44),
            cospi.add(24),
            u.as_mut_ptr().add(51),
            &raw const v_bit,
            &raw const rnding,
        );
        u[44] = temp1;
        temp2 = half_btf_neon_mode11_r(
            cospi.add(24),
            u.as_mut_ptr().add(45),
            cospi.add(40),
            u.as_mut_ptr().add(50),
            &raw const v_bit,
            &raw const rnding,
        );
        u[50] = half_btf_neon_mode10_r(
            cospi.add(40),
            u.as_mut_ptr().add(45),
            cospi.add(24),
            u.as_mut_ptr().add(50),
            &raw const v_bit,
            &raw const rnding,
        );
        u[45] = temp2;

        // stage 7
        u[3] = u[0];
        u[2] = u[1];
        u[11] = u[8];
        u[10] = u[9];
        u[12] = u[15];
        u[13] = u[14];

        temp1 = half_btf_neon_mode10_r(
            cospi.add(16),
            u.as_mut_ptr().add(18),
            cospi.add(48),
            u.as_mut_ptr().add(29),
            &raw const v_bit,
            &raw const rnding,
        );
        u[29] = half_btf_neon_r(
            cospi.add(48),
            u.as_mut_ptr().add(18),
            cospi.add(16),
            u.as_mut_ptr().add(29),
            &raw const v_bit,
            &raw const rnding,
        );
        u[18] = temp1;
        temp2 = half_btf_neon_mode10_r(
            cospi.add(16),
            u.as_mut_ptr().add(19),
            cospi.add(48),
            u.as_mut_ptr().add(28),
            &raw const v_bit,
            &raw const rnding,
        );
        u[28] = half_btf_neon_r(
            cospi.add(48),
            u.as_mut_ptr().add(19),
            cospi.add(16),
            u.as_mut_ptr().add(28),
            &raw const v_bit,
            &raw const rnding,
        );
        u[19] = temp2;
        temp1 = half_btf_neon_mode11_r(
            cospi.add(48),
            u.as_mut_ptr().add(20),
            cospi.add(16),
            u.as_mut_ptr().add(27),
            &raw const v_bit,
            &raw const rnding,
        );
        u[27] = half_btf_neon_mode10_r(
            cospi.add(16),
            u.as_mut_ptr().add(20),
            cospi.add(48),
            u.as_mut_ptr().add(27),
            &raw const v_bit,
            &raw const rnding,
        );
        u[20] = temp1;
        temp2 = half_btf_neon_mode11_r(
            cospi.add(48),
            u.as_mut_ptr().add(21),
            cospi.add(16),
            u.as_mut_ptr().add(26),
            &raw const v_bit,
            &raw const rnding,
        );
        u[26] = half_btf_neon_mode10_r(
            cospi.add(16),
            u.as_mut_ptr().add(21),
            cospi.add(48),
            u.as_mut_ptr().add(26),
            &raw const v_bit,
            &raw const rnding,
        );
        u[21] = temp2;
        i = 32;
        while i < 64 {
            j = i;
            while j < i + 4 {
                addsub_neon(
                    u[j as usize],
                    u[(j ^ 7) as usize],
                    u.as_mut_ptr().offset(j as isize),
                    u.as_mut_ptr().offset((j ^ 7) as isize),
                    &raw const clamp_lo,
                    &raw const clamp_hi,
                );
                addsub_neon(
                    u[(j ^ 15) as usize],
                    u[(j ^ 8) as usize],
                    u.as_mut_ptr().offset((j ^ 15) as isize),
                    u.as_mut_ptr().offset((j ^ 8) as isize),
                    &raw const clamp_lo,
                    &raw const clamp_hi,
                );
                j += 1;
            }
            i += 16;
        }

        // stage 8
        u[7] = u[0];
        u[6] = u[1];
        u[5] = u[2];
        u[4] = u[3];
        u[9] = u[9];

        idct64_stage8_neon(
            u.as_mut_ptr(),
            cospi,
            &raw const clamp_lo,
            &raw const clamp_hi,
            &raw const v_bit,
            &raw const rnding,
        );

        // stage 9
        idct64_stage9_neon(
            u.as_mut_ptr(),
            cospi,
            &raw const clamp_lo,
            &raw const clamp_hi,
            &raw const v_bit,
            &raw const rnding,
        );

        // stage 10
        idct64_stage10_neon(
            u.as_mut_ptr(),
            cospi,
            &raw const clamp_lo,
            &raw const clamp_hi,
            &raw const v_bit,
            &raw const rnding,
        );

        // stage 11
        idct64_stage11_neon(
            u.as_mut_ptr(),
            out,
            do_cols,
            bd,
            out_shift,
            &raw const clamp_lo,
            &raw const clamp_hi,
        );
    }
}

unsafe fn idct64x64_low16_neon(
    mut in_: *mut int32x4_t,
    mut out: *mut int32x4_t,
    mut bit: i32,
    mut do_cols: i32,
    mut bd: i32,
    mut out_shift: i32,
) {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut cospi: *const i32 = cospi_arr(bit);
    let log_range: i32 = get_log_range(bd, do_cols);
    let clamp_lo: int32x4_t = vdupq_n_s32(-(1 << (log_range - 1)));
    let clamp_hi: int32x4_t = vdupq_n_s32((1 << (log_range - 1)) - 1);
    let v_bit: int32x4_t = vdupq_n_s32(-bit);
    let rnding: int32x4_t = vdupq_n_s32(1 << (bit - 1));

    {
        let mut u: [int32x4_t; 64] = core::mem::zeroed();
        let mut tmp1: int32x4_t = core::mem::zeroed();
        let mut tmp2: int32x4_t = core::mem::zeroed();
        let mut tmp3: int32x4_t = core::mem::zeroed();
        let mut tmp4: int32x4_t = core::mem::zeroed();
        // stage 1
        u[0] = *in_;
        u[32] = *in_.add(1);
        u[36] = *in_.add(9);
        u[40] = *in_.add(5);
        u[44] = *in_.add(13);
        u[48] = *in_.add(3);
        u[52] = *in_.add(11);
        u[56] = *in_.add(7);
        u[60] = *in_.add(15);
        u[16] = *in_.add(2);
        u[20] = *in_.add(10);
        u[24] = *in_.add(6);
        u[28] = *in_.add(14);
        u[4] = *in_.add(8);
        u[8] = *in_.add(4);
        u[12] = *in_.add(12);

        // stage 2
        u[63] = half_btf_0_neon_r(
            cospi.add(1),
            u.as_mut_ptr().add(32),
            &raw const v_bit,
            &raw const rnding,
        );
        u[32] = half_btf_0_neon_r(
            cospi.add(63),
            u.as_mut_ptr().add(32),
            &raw const v_bit,
            &raw const rnding,
        );
        u[35] = half_btf_0_m_neon_r(
            cospi.add(49),
            u.as_mut_ptr().add(60),
            &raw const v_bit,
            &raw const rnding,
        );
        u[60] = half_btf_0_neon_r(
            cospi.add(15),
            u.as_mut_ptr().add(60),
            &raw const v_bit,
            &raw const rnding,
        );
        u[59] = half_btf_0_neon_r(
            cospi.add(9),
            u.as_mut_ptr().add(36),
            &raw const v_bit,
            &raw const rnding,
        );
        u[36] = half_btf_0_neon_r(
            cospi.add(55),
            u.as_mut_ptr().add(36),
            &raw const v_bit,
            &raw const rnding,
        );
        u[39] = half_btf_0_m_neon_r(
            cospi.add(57),
            u.as_mut_ptr().add(56),
            &raw const v_bit,
            &raw const rnding,
        );
        u[56] = half_btf_0_neon_r(
            cospi.add(7),
            u.as_mut_ptr().add(56),
            &raw const v_bit,
            &raw const rnding,
        );
        u[55] = half_btf_0_neon_r(
            cospi.add(5),
            u.as_mut_ptr().add(40),
            &raw const v_bit,
            &raw const rnding,
        );
        u[40] = half_btf_0_neon_r(
            cospi.add(59),
            u.as_mut_ptr().add(40),
            &raw const v_bit,
            &raw const rnding,
        );
        u[43] = half_btf_0_m_neon_r(
            cospi.add(53),
            u.as_mut_ptr().add(52),
            &raw const v_bit,
            &raw const rnding,
        );
        u[52] = half_btf_0_neon_r(
            cospi.add(11),
            u.as_mut_ptr().add(52),
            &raw const v_bit,
            &raw const rnding,
        );
        u[47] = half_btf_0_m_neon_r(
            cospi.add(61),
            u.as_mut_ptr().add(48),
            &raw const v_bit,
            &raw const rnding,
        );
        u[48] = half_btf_0_neon_r(
            cospi.add(3),
            u.as_mut_ptr().add(48),
            &raw const v_bit,
            &raw const rnding,
        );
        u[51] = half_btf_0_neon_r(
            cospi.add(13),
            u.as_mut_ptr().add(44),
            &raw const v_bit,
            &raw const rnding,
        );
        u[44] = half_btf_0_neon_r(
            cospi.add(51),
            u.as_mut_ptr().add(44),
            &raw const v_bit,
            &raw const rnding,
        );

        // stage 3
        u[31] = half_btf_0_neon_r(
            cospi.add(2),
            u.as_mut_ptr().add(16),
            &raw const v_bit,
            &raw const rnding,
        );
        u[16] = half_btf_0_neon_r(
            cospi.add(62),
            u.as_mut_ptr().add(16),
            &raw const v_bit,
            &raw const rnding,
        );
        u[19] = half_btf_0_m_neon_r(
            cospi.add(50),
            u.as_mut_ptr().add(28),
            &raw const v_bit,
            &raw const rnding,
        );
        u[28] = half_btf_0_neon_r(
            cospi.add(14),
            u.as_mut_ptr().add(28),
            &raw const v_bit,
            &raw const rnding,
        );
        u[27] = half_btf_0_neon_r(
            cospi.add(10),
            u.as_mut_ptr().add(20),
            &raw const v_bit,
            &raw const rnding,
        );
        u[20] = half_btf_0_neon_r(
            cospi.add(54),
            u.as_mut_ptr().add(20),
            &raw const v_bit,
            &raw const rnding,
        );
        u[23] = half_btf_0_m_neon_r(
            cospi.add(58),
            u.as_mut_ptr().add(24),
            &raw const v_bit,
            &raw const rnding,
        );
        u[24] = half_btf_0_neon_r(
            cospi.add(6),
            u.as_mut_ptr().add(24),
            &raw const v_bit,
            &raw const rnding,
        );
        u[33] = u[32];
        u[34] = u[35];
        u[37] = u[36];
        u[38] = u[39];
        u[41] = u[40];
        u[42] = u[43];
        u[45] = u[44];
        u[46] = u[47];
        u[49] = u[48];
        u[50] = u[51];
        u[53] = u[52];
        u[54] = u[55];
        u[57] = u[56];
        u[58] = u[59];
        u[61] = u[60];
        u[62] = u[63];

        // stage 4
        u[15] = half_btf_0_neon_r(
            cospi.add(4),
            u.as_mut_ptr().add(8),
            &raw const v_bit,
            &raw const rnding,
        );
        u[8] = half_btf_0_neon_r(
            cospi.add(60),
            u.as_mut_ptr().add(8),
            &raw const v_bit,
            &raw const rnding,
        );
        u[11] = half_btf_0_m_neon_r(
            cospi.add(52),
            u.as_mut_ptr().add(12),
            &raw const v_bit,
            &raw const rnding,
        );
        u[12] = half_btf_0_neon_r(
            cospi.add(12),
            u.as_mut_ptr().add(12),
            &raw const v_bit,
            &raw const rnding,
        );

        u[17] = u[16];
        u[18] = u[19];
        u[21] = u[20];
        u[22] = u[23];
        u[25] = u[24];
        u[26] = u[27];
        u[29] = u[28];
        u[30] = u[31];

        tmp1 = half_btf_neon_mode10_r(
            cospi.add(4),
            u.as_mut_ptr().add(33),
            cospi.add(60),
            u.as_mut_ptr().add(62),
            &raw const v_bit,
            &raw const rnding,
        );
        tmp2 = half_btf_neon_mode11_r(
            cospi.add(60),
            u.as_mut_ptr().add(34),
            cospi.add(4),
            u.as_mut_ptr().add(61),
            &raw const v_bit,
            &raw const rnding,
        );
        tmp3 = half_btf_neon_mode10_r(
            cospi.add(36),
            u.as_mut_ptr().add(37),
            cospi.add(28),
            u.as_mut_ptr().add(58),
            &raw const v_bit,
            &raw const rnding,
        );
        tmp4 = half_btf_neon_mode11_r(
            cospi.add(28),
            u.as_mut_ptr().add(38),
            cospi.add(36),
            u.as_mut_ptr().add(57),
            &raw const v_bit,
            &raw const rnding,
        );
        u[57] = half_btf_neon_mode10_r(
            cospi.add(36),
            u.as_mut_ptr().add(38),
            cospi.add(28),
            u.as_mut_ptr().add(57),
            &raw const v_bit,
            &raw const rnding,
        );
        u[58] = half_btf_neon_r(
            cospi.add(28),
            u.as_mut_ptr().add(37),
            cospi.add(36),
            u.as_mut_ptr().add(58),
            &raw const v_bit,
            &raw const rnding,
        );
        u[61] = half_btf_neon_mode10_r(
            cospi.add(4),
            u.as_mut_ptr().add(34),
            cospi.add(60),
            u.as_mut_ptr().add(61),
            &raw const v_bit,
            &raw const rnding,
        );
        u[62] = half_btf_neon_r(
            cospi.add(60),
            u.as_mut_ptr().add(33),
            cospi.add(4),
            u.as_mut_ptr().add(62),
            &raw const v_bit,
            &raw const rnding,
        );
        u[33] = tmp1;
        u[34] = tmp2;
        u[37] = tmp3;
        u[38] = tmp4;

        tmp1 = half_btf_neon_mode10_r(
            cospi.add(20),
            u.as_mut_ptr().add(41),
            cospi.add(44),
            u.as_mut_ptr().add(54),
            &raw const v_bit,
            &raw const rnding,
        );
        tmp2 = half_btf_neon_mode11_r(
            cospi.add(44),
            u.as_mut_ptr().add(42),
            cospi.add(20),
            u.as_mut_ptr().add(53),
            &raw const v_bit,
            &raw const rnding,
        );
        tmp3 = half_btf_neon_r(
            cospi.add(52),
            u.as_mut_ptr().add(45),
            cospi.add(12),
            u.as_mut_ptr().add(50),
            &raw const v_bit,
            &raw const rnding,
        );
        tmp4 = half_btf_neon_mode11_r(
            cospi.add(12),
            u.as_mut_ptr().add(46),
            cospi.add(52),
            u.as_mut_ptr().add(49),
            &raw const v_bit,
            &raw const rnding,
        );
        u[49] = half_btf_neon_mode10_r(
            cospi.add(52),
            u.as_mut_ptr().add(46),
            cospi.add(12),
            u.as_mut_ptr().add(49),
            &raw const v_bit,
            &raw const rnding,
        );
        u[50] = half_btf_neon_r(
            cospi.add(12),
            u.as_mut_ptr().add(45),
            cospi.add(52),
            u.as_mut_ptr().add(50),
            &raw const v_bit,
            &raw const rnding,
        );
        u[53] = half_btf_neon_mode10_r(
            cospi.add(20),
            u.as_mut_ptr().add(42),
            cospi.add(44),
            u.as_mut_ptr().add(53),
            &raw const v_bit,
            &raw const rnding,
        );
        u[54] = half_btf_neon_r(
            cospi.add(44),
            u.as_mut_ptr().add(41),
            cospi.add(20),
            u.as_mut_ptr().add(54),
            &raw const v_bit,
            &raw const rnding,
        );
        u[41] = tmp1;
        u[42] = tmp2;
        u[45] = tmp3;
        u[46] = tmp4;

        // stage 5
        u[7] = half_btf_0_neon_r(
            cospi.add(8),
            u.as_mut_ptr().add(4),
            &raw const v_bit,
            &raw const rnding,
        );
        u[4] = half_btf_0_neon_r(
            cospi.add(56),
            u.as_mut_ptr().add(4),
            &raw const v_bit,
            &raw const rnding,
        );

        u[9] = u[8];
        u[10] = u[11];
        u[13] = u[12];
        u[14] = u[15];

        tmp1 = half_btf_neon_mode10_r(
            cospi.add(8),
            u.as_mut_ptr().add(17),
            cospi.add(56),
            u.as_mut_ptr().add(30),
            &raw const v_bit,
            &raw const rnding,
        );
        tmp2 = half_btf_neon_mode11_r(
            cospi.add(56),
            u.as_mut_ptr().add(18),
            cospi.add(8),
            u.as_mut_ptr().add(29),
            &raw const v_bit,
            &raw const rnding,
        );
        tmp3 = half_btf_neon_mode10_r(
            cospi.add(40),
            u.as_mut_ptr().add(21),
            cospi.add(24),
            u.as_mut_ptr().add(26),
            &raw const v_bit,
            &raw const rnding,
        );
        tmp4 = half_btf_neon_mode11_r(
            cospi.add(24),
            u.as_mut_ptr().add(22),
            cospi.add(40),
            u.as_mut_ptr().add(25),
            &raw const v_bit,
            &raw const rnding,
        );
        u[25] = half_btf_neon_mode10_r(
            cospi.add(40),
            u.as_mut_ptr().add(22),
            cospi.add(24),
            u.as_mut_ptr().add(25),
            &raw const v_bit,
            &raw const rnding,
        );
        u[26] = half_btf_neon_r(
            cospi.add(24),
            u.as_mut_ptr().add(21),
            cospi.add(40),
            u.as_mut_ptr().add(26),
            &raw const v_bit,
            &raw const rnding,
        );
        u[29] = half_btf_neon_mode10_r(
            cospi.add(8),
            u.as_mut_ptr().add(18),
            cospi.add(56),
            u.as_mut_ptr().add(29),
            &raw const v_bit,
            &raw const rnding,
        );
        u[30] = half_btf_neon_r(
            cospi.add(56),
            u.as_mut_ptr().add(17),
            cospi.add(8),
            u.as_mut_ptr().add(30),
            &raw const v_bit,
            &raw const rnding,
        );
        u[17] = tmp1;
        u[18] = tmp2;
        u[21] = tmp3;
        u[22] = tmp4;

        i = 32;
        while i < 64 {
            addsub_neon(
                u[(i + 0) as usize],
                u[(i + 3) as usize],
                u.as_mut_ptr().offset((i + 0) as isize),
                u.as_mut_ptr().offset((i + 3) as isize),
                &raw const clamp_lo,
                &raw const clamp_hi,
            );
            addsub_neon(
                u[(i + 1) as usize],
                u[(i + 2) as usize],
                u.as_mut_ptr().offset((i + 1) as isize),
                u.as_mut_ptr().offset((i + 2) as isize),
                &raw const clamp_lo,
                &raw const clamp_hi,
            );

            addsub_neon(
                u[(i + 7) as usize],
                u[(i + 4) as usize],
                u.as_mut_ptr().offset((i + 7) as isize),
                u.as_mut_ptr().offset((i + 4) as isize),
                &raw const clamp_lo,
                &raw const clamp_hi,
            );
            addsub_neon(
                u[(i + 6) as usize],
                u[(i + 5) as usize],
                u.as_mut_ptr().offset((i + 6) as isize),
                u.as_mut_ptr().offset((i + 5) as isize),
                &raw const clamp_lo,
                &raw const clamp_hi,
            );
            i += 8;
        }

        // stage 6
        tmp1 =
            half_btf_0_neon_r(cospi.add(32), u.as_mut_ptr(), &raw const v_bit, &raw const rnding);
        u[1] =
            half_btf_0_neon_r(cospi.add(32), u.as_mut_ptr(), &raw const v_bit, &raw const rnding);
        u[0] = tmp1;
        u[5] = u[4];
        u[6] = u[7];

        tmp1 = half_btf_neon_mode10_r(
            cospi.add(16),
            u.as_mut_ptr().add(9),
            cospi.add(48),
            u.as_mut_ptr().add(14),
            &raw const v_bit,
            &raw const rnding,
        );
        u[14] = half_btf_neon_r(
            cospi.add(48),
            u.as_mut_ptr().add(9),
            cospi.add(16),
            u.as_mut_ptr().add(14),
            &raw const v_bit,
            &raw const rnding,
        );
        u[9] = tmp1;
        tmp2 = half_btf_neon_mode01_r(
            cospi.add(48),
            u.as_mut_ptr().add(10),
            cospi.add(16),
            u.as_mut_ptr().add(13),
            &raw const v_bit,
            &raw const rnding,
        );
        u[13] = half_btf_neon_mode10_r(
            cospi.add(16),
            u.as_mut_ptr().add(10),
            cospi.add(48),
            u.as_mut_ptr().add(13),
            &raw const v_bit,
            &raw const rnding,
        );
        u[10] = tmp2;

        i = 16;
        while i < 32 {
            addsub_neon(
                u[(i + 0) as usize],
                u[(i + 3) as usize],
                u.as_mut_ptr().offset((i + 0) as isize),
                u.as_mut_ptr().offset((i + 3) as isize),
                &raw const clamp_lo,
                &raw const clamp_hi,
            );
            addsub_neon(
                u[(i + 1) as usize],
                u[(i + 2) as usize],
                u.as_mut_ptr().offset((i + 1) as isize),
                u.as_mut_ptr().offset((i + 2) as isize),
                &raw const clamp_lo,
                &raw const clamp_hi,
            );

            addsub_neon(
                u[(i + 7) as usize],
                u[(i + 4) as usize],
                u.as_mut_ptr().offset((i + 7) as isize),
                u.as_mut_ptr().offset((i + 4) as isize),
                &raw const clamp_lo,
                &raw const clamp_hi,
            );
            addsub_neon(
                u[(i + 6) as usize],
                u[(i + 5) as usize],
                u.as_mut_ptr().offset((i + 6) as isize),
                u.as_mut_ptr().offset((i + 5) as isize),
                &raw const clamp_lo,
                &raw const clamp_hi,
            );
            i += 8;
        }

        tmp1 = half_btf_neon_mode10_r(
            cospi.add(8),
            u.as_mut_ptr().add(34),
            cospi.add(56),
            u.as_mut_ptr().add(61),
            &raw const v_bit,
            &raw const rnding,
        );
        tmp2 = half_btf_neon_mode10_r(
            cospi.add(8),
            u.as_mut_ptr().add(35),
            cospi.add(56),
            u.as_mut_ptr().add(60),
            &raw const v_bit,
            &raw const rnding,
        );
        tmp3 = half_btf_neon_mode11_r(
            cospi.add(56),
            u.as_mut_ptr().add(36),
            cospi.add(8),
            u.as_mut_ptr().add(59),
            &raw const v_bit,
            &raw const rnding,
        );
        tmp4 = half_btf_neon_mode11_r(
            cospi.add(56),
            u.as_mut_ptr().add(37),
            cospi.add(8),
            u.as_mut_ptr().add(58),
            &raw const v_bit,
            &raw const rnding,
        );
        u[58] = half_btf_neon_mode10_r(
            cospi.add(8),
            u.as_mut_ptr().add(37),
            cospi.add(56),
            u.as_mut_ptr().add(58),
            &raw const v_bit,
            &raw const rnding,
        );
        u[59] = half_btf_neon_mode10_r(
            cospi.add(8),
            u.as_mut_ptr().add(36),
            cospi.add(56),
            u.as_mut_ptr().add(59),
            &raw const v_bit,
            &raw const rnding,
        );
        u[60] = half_btf_neon_r(
            cospi.add(56),
            u.as_mut_ptr().add(35),
            cospi.add(8),
            u.as_mut_ptr().add(60),
            &raw const v_bit,
            &raw const rnding,
        );
        u[61] = half_btf_neon_r(
            cospi.add(56),
            u.as_mut_ptr().add(34),
            cospi.add(8),
            u.as_mut_ptr().add(61),
            &raw const v_bit,
            &raw const rnding,
        );
        u[34] = tmp1;
        u[35] = tmp2;
        u[36] = tmp3;
        u[37] = tmp4;

        tmp1 = half_btf_neon_mode10_r(
            cospi.add(40),
            u.as_mut_ptr().add(42),
            cospi.add(24),
            u.as_mut_ptr().add(53),
            &raw const v_bit,
            &raw const rnding,
        );
        tmp2 = half_btf_neon_mode10_r(
            cospi.add(40),
            u.as_mut_ptr().add(43),
            cospi.add(24),
            u.as_mut_ptr().add(52),
            &raw const v_bit,
            &raw const rnding,
        );
        tmp3 = half_btf_neon_mode11_r(
            cospi.add(24),
            u.as_mut_ptr().add(44),
            cospi.add(40),
            u.as_mut_ptr().add(51),
            &raw const v_bit,
            &raw const rnding,
        );
        tmp4 = half_btf_neon_mode11_r(
            cospi.add(24),
            u.as_mut_ptr().add(45),
            cospi.add(40),
            u.as_mut_ptr().add(50),
            &raw const v_bit,
            &raw const rnding,
        );
        u[50] = half_btf_neon_mode10_r(
            cospi.add(40),
            u.as_mut_ptr().add(45),
            cospi.add(24),
            u.as_mut_ptr().add(50),
            &raw const v_bit,
            &raw const rnding,
        );
        u[51] = half_btf_neon_mode10_r(
            cospi.add(40),
            u.as_mut_ptr().add(44),
            cospi.add(24),
            u.as_mut_ptr().add(51),
            &raw const v_bit,
            &raw const rnding,
        );
        u[52] = half_btf_neon_r(
            cospi.add(24),
            u.as_mut_ptr().add(43),
            cospi.add(40),
            u.as_mut_ptr().add(52),
            &raw const v_bit,
            &raw const rnding,
        );
        u[53] = half_btf_neon_r(
            cospi.add(24),
            u.as_mut_ptr().add(42),
            cospi.add(40),
            u.as_mut_ptr().add(53),
            &raw const v_bit,
            &raw const rnding,
        );
        u[42] = tmp1;
        u[43] = tmp2;
        u[44] = tmp3;
        u[45] = tmp4;

        // stage 7
        u[3] = u[0];
        u[2] = u[1];
        tmp1 = half_btf_neon_mode10_r(
            cospi.add(32),
            u.as_mut_ptr().add(5),
            cospi.add(32),
            u.as_mut_ptr().add(6),
            &raw const v_bit,
            &raw const rnding,
        );
        u[6] = half_btf_neon_r(
            cospi.add(32),
            u.as_mut_ptr().add(5),
            cospi.add(32),
            u.as_mut_ptr().add(6),
            &raw const v_bit,
            &raw const rnding,
        );
        u[5] = tmp1;
        addsub_neon(
            u[8],
            u[11],
            u.as_mut_ptr().add(8),
            u.as_mut_ptr().add(11),
            &raw const clamp_lo,
            &raw const clamp_hi,
        );
        addsub_neon(
            u[9],
            u[10],
            u.as_mut_ptr().add(9),
            u.as_mut_ptr().add(10),
            &raw const clamp_lo,
            &raw const clamp_hi,
        );
        addsub_neon(
            u[15],
            u[12],
            u.as_mut_ptr().add(15),
            u.as_mut_ptr().add(12),
            &raw const clamp_lo,
            &raw const clamp_hi,
        );
        addsub_neon(
            u[14],
            u[13],
            u.as_mut_ptr().add(14),
            u.as_mut_ptr().add(13),
            &raw const clamp_lo,
            &raw const clamp_hi,
        );

        tmp1 = half_btf_neon_mode10_r(
            cospi.add(16),
            u.as_mut_ptr().add(18),
            cospi.add(48),
            u.as_mut_ptr().add(29),
            &raw const v_bit,
            &raw const rnding,
        );
        tmp2 = half_btf_neon_mode10_r(
            cospi.add(16),
            u.as_mut_ptr().add(19),
            cospi.add(48),
            u.as_mut_ptr().add(28),
            &raw const v_bit,
            &raw const rnding,
        );
        tmp3 = half_btf_neon_mode11_r(
            cospi.add(48),
            u.as_mut_ptr().add(20),
            cospi.add(16),
            u.as_mut_ptr().add(27),
            &raw const v_bit,
            &raw const rnding,
        );
        tmp4 = half_btf_neon_mode11_r(
            cospi.add(48),
            u.as_mut_ptr().add(21),
            cospi.add(16),
            u.as_mut_ptr().add(26),
            &raw const v_bit,
            &raw const rnding,
        );
        u[26] = half_btf_neon_mode10_r(
            cospi.add(16),
            u.as_mut_ptr().add(21),
            cospi.add(48),
            u.as_mut_ptr().add(26),
            &raw const v_bit,
            &raw const rnding,
        );
        u[27] = half_btf_neon_mode10_r(
            cospi.add(16),
            u.as_mut_ptr().add(20),
            cospi.add(48),
            u.as_mut_ptr().add(27),
            &raw const v_bit,
            &raw const rnding,
        );
        u[28] = half_btf_neon_r(
            cospi.add(48),
            u.as_mut_ptr().add(19),
            cospi.add(16),
            u.as_mut_ptr().add(28),
            &raw const v_bit,
            &raw const rnding,
        );
        u[29] = half_btf_neon_r(
            cospi.add(48),
            u.as_mut_ptr().add(18),
            cospi.add(16),
            u.as_mut_ptr().add(29),
            &raw const v_bit,
            &raw const rnding,
        );
        u[18] = tmp1;
        u[19] = tmp2;
        u[20] = tmp3;
        u[21] = tmp4;

        i = 32;
        while i < 64 {
            j = i;
            while j < i + 4 {
                addsub_neon(
                    u[j as usize],
                    u[(j ^ 7) as usize],
                    u.as_mut_ptr().offset(j as isize),
                    u.as_mut_ptr().offset((j ^ 7) as isize),
                    &raw const clamp_lo,
                    &raw const clamp_hi,
                );
                addsub_neon(
                    u[(j ^ 15) as usize],
                    u[(j ^ 8) as usize],
                    u.as_mut_ptr().offset((j ^ 15) as isize),
                    u.as_mut_ptr().offset((j ^ 8) as isize),
                    &raw const clamp_lo,
                    &raw const clamp_hi,
                );
                j += 1;
            }
            i += 16;
        }

        // stage 8
        i = 0;
        while i < 4 {
            addsub_neon(
                u[i as usize],
                u[(7 - i) as usize],
                u.as_mut_ptr().offset(i as isize),
                u.as_mut_ptr().offset((7 - i) as isize),
                &raw const clamp_lo,
                &raw const clamp_hi,
            );
            i += 1;
        }

        idct64_stage8_neon(
            u.as_mut_ptr(),
            cospi,
            &raw const clamp_lo,
            &raw const clamp_hi,
            &raw const v_bit,
            &raw const rnding,
        );

        // stage 9
        idct64_stage9_neon(
            u.as_mut_ptr(),
            cospi,
            &raw const clamp_lo,
            &raw const clamp_hi,
            &raw const v_bit,
            &raw const rnding,
        );

        // stage 10
        idct64_stage10_neon(
            u.as_mut_ptr(),
            cospi,
            &raw const clamp_lo,
            &raw const clamp_hi,
            &raw const v_bit,
            &raw const rnding,
        );

        // stage 11
        idct64_stage11_neon(
            u.as_mut_ptr(),
            out,
            do_cols,
            bd,
            out_shift,
            &raw const clamp_lo,
            &raw const clamp_hi,
        );
    }
}

unsafe fn idct64x64_neon(
    mut in_: *mut int32x4_t,
    mut out: *mut int32x4_t,
    mut bit: i32,
    mut do_cols: i32,
    mut bd: i32,
    mut out_shift: i32,
) {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut cospi: *const i32 = cospi_arr(bit);
    let v_bit: int32x4_t = vdupq_n_s32(-bit);
    let rnding: int32x4_t = vdupq_n_s32(1 << (bit - 1));

    let log_range: i32 = get_log_range(bd, do_cols);
    let clamp_lo: int32x4_t = vdupq_n_s32(-(1 << (log_range - 1)));
    let clamp_hi: int32x4_t = vdupq_n_s32((1 << (log_range - 1)) - 1);

    {
        let mut u: [int32x4_t; 64] = core::mem::zeroed();
        let mut v: [int32x4_t; 64] = core::mem::zeroed();

        // stage 1
        u[32] = *in_.add(1);
        u[34] = *in_.add(17);
        u[36] = *in_.add(9);
        u[38] = *in_.add(25);
        u[40] = *in_.add(5);
        u[42] = *in_.add(21);
        u[44] = *in_.add(13);
        u[46] = *in_.add(29);
        u[48] = *in_.add(3);
        u[50] = *in_.add(19);
        u[52] = *in_.add(11);
        u[54] = *in_.add(27);
        u[56] = *in_.add(7);
        u[58] = *in_.add(23);
        u[60] = *in_.add(15);
        u[62] = *in_.add(31);

        v[16] = *in_.add(2);
        v[18] = *in_.add(18);
        v[20] = *in_.add(10);
        v[22] = *in_.add(26);
        v[24] = *in_.add(6);
        v[26] = *in_.add(22);
        v[28] = *in_.add(14);
        v[30] = *in_.add(30);

        u[8] = *in_.add(4);
        u[10] = *in_.add(20);
        u[12] = *in_.add(12);
        u[14] = *in_.add(28);

        v[4] = *in_.add(8);
        v[6] = *in_.add(24);

        u[0] = *in_;
        u[2] = *in_.add(16);

        // stage 2
        v[32] = half_btf_0_neon_r(
            cospi.add(63),
            u.as_mut_ptr().add(32),
            &raw const v_bit,
            &raw const rnding,
        );
        v[33] = half_btf_0_m_neon_r(
            cospi.add(33),
            u.as_mut_ptr().add(62),
            &raw const v_bit,
            &raw const rnding,
        );
        v[34] = half_btf_0_neon_r(
            cospi.add(47),
            u.as_mut_ptr().add(34),
            &raw const v_bit,
            &raw const rnding,
        );
        v[35] = half_btf_0_m_neon_r(
            cospi.add(49),
            u.as_mut_ptr().add(60),
            &raw const v_bit,
            &raw const rnding,
        );
        v[36] = half_btf_0_neon_r(
            cospi.add(55),
            u.as_mut_ptr().add(36),
            &raw const v_bit,
            &raw const rnding,
        );
        v[37] = half_btf_0_m_neon_r(
            cospi.add(41),
            u.as_mut_ptr().add(58),
            &raw const v_bit,
            &raw const rnding,
        );
        v[38] = half_btf_0_neon_r(
            cospi.add(39),
            u.as_mut_ptr().add(38),
            &raw const v_bit,
            &raw const rnding,
        );
        v[39] = half_btf_0_m_neon_r(
            cospi.add(57),
            u.as_mut_ptr().add(56),
            &raw const v_bit,
            &raw const rnding,
        );
        v[40] = half_btf_0_neon_r(
            cospi.add(59),
            u.as_mut_ptr().add(40),
            &raw const v_bit,
            &raw const rnding,
        );
        v[41] = half_btf_0_m_neon_r(
            cospi.add(37),
            u.as_mut_ptr().add(54),
            &raw const v_bit,
            &raw const rnding,
        );
        v[42] = half_btf_0_neon_r(
            cospi.add(43),
            u.as_mut_ptr().add(42),
            &raw const v_bit,
            &raw const rnding,
        );
        v[43] = half_btf_0_m_neon_r(
            cospi.add(53),
            u.as_mut_ptr().add(52),
            &raw const v_bit,
            &raw const rnding,
        );
        v[44] = half_btf_0_neon_r(
            cospi.add(51),
            u.as_mut_ptr().add(44),
            &raw const v_bit,
            &raw const rnding,
        );
        v[45] = half_btf_0_m_neon_r(
            cospi.add(45),
            u.as_mut_ptr().add(50),
            &raw const v_bit,
            &raw const rnding,
        );
        v[46] = half_btf_0_neon_r(
            cospi.add(35),
            u.as_mut_ptr().add(46),
            &raw const v_bit,
            &raw const rnding,
        );
        v[47] = half_btf_0_m_neon_r(
            cospi.add(61),
            u.as_mut_ptr().add(48),
            &raw const v_bit,
            &raw const rnding,
        );
        v[48] = half_btf_0_neon_r(
            cospi.add(3),
            u.as_mut_ptr().add(48),
            &raw const v_bit,
            &raw const rnding,
        );
        v[49] = half_btf_0_neon_r(
            cospi.add(29),
            u.as_mut_ptr().add(46),
            &raw const v_bit,
            &raw const rnding,
        );
        v[50] = half_btf_0_neon_r(
            cospi.add(19),
            u.as_mut_ptr().add(50),
            &raw const v_bit,
            &raw const rnding,
        );
        v[51] = half_btf_0_neon_r(
            cospi.add(13),
            u.as_mut_ptr().add(44),
            &raw const v_bit,
            &raw const rnding,
        );
        v[52] = half_btf_0_neon_r(
            cospi.add(11),
            u.as_mut_ptr().add(52),
            &raw const v_bit,
            &raw const rnding,
        );
        v[53] = half_btf_0_neon_r(
            cospi.add(21),
            u.as_mut_ptr().add(42),
            &raw const v_bit,
            &raw const rnding,
        );
        v[54] = half_btf_0_neon_r(
            cospi.add(27),
            u.as_mut_ptr().add(54),
            &raw const v_bit,
            &raw const rnding,
        );
        v[55] = half_btf_0_neon_r(
            cospi.add(5),
            u.as_mut_ptr().add(40),
            &raw const v_bit,
            &raw const rnding,
        );
        v[56] = half_btf_0_neon_r(
            cospi.add(7),
            u.as_mut_ptr().add(56),
            &raw const v_bit,
            &raw const rnding,
        );
        v[57] = half_btf_0_neon_r(
            cospi.add(25),
            u.as_mut_ptr().add(38),
            &raw const v_bit,
            &raw const rnding,
        );
        v[58] = half_btf_0_neon_r(
            cospi.add(23),
            u.as_mut_ptr().add(58),
            &raw const v_bit,
            &raw const rnding,
        );
        v[59] = half_btf_0_neon_r(
            cospi.add(9),
            u.as_mut_ptr().add(36),
            &raw const v_bit,
            &raw const rnding,
        );
        v[60] = half_btf_0_neon_r(
            cospi.add(15),
            u.as_mut_ptr().add(60),
            &raw const v_bit,
            &raw const rnding,
        );
        v[61] = half_btf_0_neon_r(
            cospi.add(17),
            u.as_mut_ptr().add(34),
            &raw const v_bit,
            &raw const rnding,
        );
        v[62] = half_btf_0_neon_r(
            cospi.add(31),
            u.as_mut_ptr().add(62),
            &raw const v_bit,
            &raw const rnding,
        );
        v[63] = half_btf_0_neon_r(
            cospi.add(1),
            u.as_mut_ptr().add(32),
            &raw const v_bit,
            &raw const rnding,
        );

        // stage 3
        u[16] = half_btf_0_neon_r(
            cospi.add(62),
            v.as_mut_ptr().add(16),
            &raw const v_bit,
            &raw const rnding,
        );
        u[17] = half_btf_0_m_neon_r(
            cospi.add(34),
            v.as_mut_ptr().add(30),
            &raw const v_bit,
            &raw const rnding,
        );
        u[18] = half_btf_0_neon_r(
            cospi.add(46),
            v.as_mut_ptr().add(18),
            &raw const v_bit,
            &raw const rnding,
        );
        u[19] = half_btf_0_m_neon_r(
            cospi.add(50),
            v.as_mut_ptr().add(28),
            &raw const v_bit,
            &raw const rnding,
        );
        u[20] = half_btf_0_neon_r(
            cospi.add(54),
            v.as_mut_ptr().add(20),
            &raw const v_bit,
            &raw const rnding,
        );
        u[21] = half_btf_0_m_neon_r(
            cospi.add(42),
            v.as_mut_ptr().add(26),
            &raw const v_bit,
            &raw const rnding,
        );
        u[22] = half_btf_0_neon_r(
            cospi.add(38),
            v.as_mut_ptr().add(22),
            &raw const v_bit,
            &raw const rnding,
        );
        u[23] = half_btf_0_m_neon_r(
            cospi.add(58),
            v.as_mut_ptr().add(24),
            &raw const v_bit,
            &raw const rnding,
        );
        u[24] = half_btf_0_neon_r(
            cospi.add(6),
            v.as_mut_ptr().add(24),
            &raw const v_bit,
            &raw const rnding,
        );
        u[25] = half_btf_0_neon_r(
            cospi.add(26),
            v.as_mut_ptr().add(22),
            &raw const v_bit,
            &raw const rnding,
        );
        u[26] = half_btf_0_neon_r(
            cospi.add(22),
            v.as_mut_ptr().add(26),
            &raw const v_bit,
            &raw const rnding,
        );
        u[27] = half_btf_0_neon_r(
            cospi.add(10),
            v.as_mut_ptr().add(20),
            &raw const v_bit,
            &raw const rnding,
        );
        u[28] = half_btf_0_neon_r(
            cospi.add(14),
            v.as_mut_ptr().add(28),
            &raw const v_bit,
            &raw const rnding,
        );
        u[29] = half_btf_0_neon_r(
            cospi.add(18),
            v.as_mut_ptr().add(18),
            &raw const v_bit,
            &raw const rnding,
        );
        u[30] = half_btf_0_neon_r(
            cospi.add(30),
            v.as_mut_ptr().add(30),
            &raw const v_bit,
            &raw const rnding,
        );
        u[31] = half_btf_0_neon_r(
            cospi.add(2),
            v.as_mut_ptr().add(16),
            &raw const v_bit,
            &raw const rnding,
        );

        i = 32;
        while i < 64 {
            addsub_neon(
                v[(i + 0) as usize],
                v[(i + 1) as usize],
                u.as_mut_ptr().offset((i + 0) as isize),
                u.as_mut_ptr().offset((i + 1) as isize),
                &raw const clamp_lo,
                &raw const clamp_hi,
            );
            addsub_neon(
                v[(i + 3) as usize],
                v[(i + 2) as usize],
                u.as_mut_ptr().offset((i + 3) as isize),
                u.as_mut_ptr().offset((i + 2) as isize),
                &raw const clamp_lo,
                &raw const clamp_hi,
            );
            i += 4;
        }

        // stage 4
        v[8] = half_btf_0_neon_r(
            cospi.add(60),
            u.as_mut_ptr().add(8),
            &raw const v_bit,
            &raw const rnding,
        );
        v[9] = half_btf_0_m_neon_r(
            cospi.add(36),
            u.as_mut_ptr().add(14),
            &raw const v_bit,
            &raw const rnding,
        );
        v[10] = half_btf_0_neon_r(
            cospi.add(44),
            u.as_mut_ptr().add(10),
            &raw const v_bit,
            &raw const rnding,
        );
        v[11] = half_btf_0_m_neon_r(
            cospi.add(52),
            u.as_mut_ptr().add(12),
            &raw const v_bit,
            &raw const rnding,
        );
        v[12] = half_btf_0_neon_r(
            cospi.add(12),
            u.as_mut_ptr().add(12),
            &raw const v_bit,
            &raw const rnding,
        );
        v[13] = half_btf_0_neon_r(
            cospi.add(20),
            u.as_mut_ptr().add(10),
            &raw const v_bit,
            &raw const rnding,
        );
        v[14] = half_btf_0_neon_r(
            cospi.add(28),
            u.as_mut_ptr().add(14),
            &raw const v_bit,
            &raw const rnding,
        );
        v[15] = half_btf_0_neon_r(
            cospi.add(4),
            u.as_mut_ptr().add(8),
            &raw const v_bit,
            &raw const rnding,
        );

        i = 16;
        while i < 32 {
            addsub_neon(
                u[(i + 0) as usize],
                u[(i + 1) as usize],
                v.as_mut_ptr().offset((i + 0) as isize),
                v.as_mut_ptr().offset((i + 1) as isize),
                &raw const clamp_lo,
                &raw const clamp_hi,
            );
            addsub_neon(
                u[(i + 3) as usize],
                u[(i + 2) as usize],
                v.as_mut_ptr().offset((i + 3) as isize),
                v.as_mut_ptr().offset((i + 2) as isize),
                &raw const clamp_lo,
                &raw const clamp_hi,
            );
            i += 4;
        }

        i = 32;
        while i < 64 {
            v[(i + 0) as usize] = u[(i + 0) as usize];
            v[(i + 3) as usize] = u[(i + 3) as usize];
            i += 4;
        }

        v[33] = half_btf_neon_mode10_r(
            cospi.add(4),
            u.as_mut_ptr().add(33),
            cospi.add(60),
            u.as_mut_ptr().add(62),
            &raw const v_bit,
            &raw const rnding,
        );
        v[34] = half_btf_neon_mode11_r(
            cospi.add(60),
            u.as_mut_ptr().add(34),
            cospi.add(4),
            u.as_mut_ptr().add(61),
            &raw const v_bit,
            &raw const rnding,
        );
        v[37] = half_btf_neon_mode10_r(
            cospi.add(36),
            u.as_mut_ptr().add(37),
            cospi.add(28),
            u.as_mut_ptr().add(58),
            &raw const v_bit,
            &raw const rnding,
        );
        v[38] = half_btf_neon_mode11_r(
            cospi.add(28),
            u.as_mut_ptr().add(38),
            cospi.add(36),
            u.as_mut_ptr().add(57),
            &raw const v_bit,
            &raw const rnding,
        );
        v[41] = half_btf_neon_mode10_r(
            cospi.add(20),
            u.as_mut_ptr().add(41),
            cospi.add(44),
            u.as_mut_ptr().add(54),
            &raw const v_bit,
            &raw const rnding,
        );
        v[42] = half_btf_neon_mode11_r(
            cospi.add(44),
            u.as_mut_ptr().add(42),
            cospi.add(20),
            u.as_mut_ptr().add(53),
            &raw const v_bit,
            &raw const rnding,
        );
        v[45] = half_btf_neon_mode10_r(
            cospi.add(52),
            u.as_mut_ptr().add(45),
            cospi.add(12),
            u.as_mut_ptr().add(50),
            &raw const v_bit,
            &raw const rnding,
        );
        v[46] = half_btf_neon_mode11_r(
            cospi.add(12),
            u.as_mut_ptr().add(46),
            cospi.add(52),
            u.as_mut_ptr().add(49),
            &raw const v_bit,
            &raw const rnding,
        );
        v[49] = half_btf_neon_mode10_r(
            cospi.add(52),
            u.as_mut_ptr().add(46),
            cospi.add(12),
            u.as_mut_ptr().add(49),
            &raw const v_bit,
            &raw const rnding,
        );
        v[50] = half_btf_neon_r(
            cospi.add(12),
            u.as_mut_ptr().add(45),
            cospi.add(52),
            u.as_mut_ptr().add(50),
            &raw const v_bit,
            &raw const rnding,
        );
        v[53] = half_btf_neon_mode10_r(
            cospi.add(20),
            u.as_mut_ptr().add(42),
            cospi.add(44),
            u.as_mut_ptr().add(53),
            &raw const v_bit,
            &raw const rnding,
        );
        v[54] = half_btf_neon_r(
            cospi.add(44),
            u.as_mut_ptr().add(41),
            cospi.add(20),
            u.as_mut_ptr().add(54),
            &raw const v_bit,
            &raw const rnding,
        );
        v[57] = half_btf_neon_mode10_r(
            cospi.add(36),
            u.as_mut_ptr().add(38),
            cospi.add(28),
            u.as_mut_ptr().add(57),
            &raw const v_bit,
            &raw const rnding,
        );
        v[58] = half_btf_neon_r(
            cospi.add(28),
            u.as_mut_ptr().add(37),
            cospi.add(36),
            u.as_mut_ptr().add(58),
            &raw const v_bit,
            &raw const rnding,
        );
        v[61] = half_btf_neon_mode10_r(
            cospi.add(4),
            u.as_mut_ptr().add(34),
            cospi.add(60),
            u.as_mut_ptr().add(61),
            &raw const v_bit,
            &raw const rnding,
        );
        v[62] = half_btf_neon_r(
            cospi.add(60),
            u.as_mut_ptr().add(33),
            cospi.add(4),
            u.as_mut_ptr().add(62),
            &raw const v_bit,
            &raw const rnding,
        );

        // stage 5
        u[4] = half_btf_0_neon_r(
            cospi.add(56),
            v.as_mut_ptr().add(4),
            &raw const v_bit,
            &raw const rnding,
        );
        u[5] = half_btf_0_m_neon_r(
            cospi.add(40),
            v.as_mut_ptr().add(6),
            &raw const v_bit,
            &raw const rnding,
        );
        u[6] = half_btf_0_neon_r(
            cospi.add(24),
            v.as_mut_ptr().add(6),
            &raw const v_bit,
            &raw const rnding,
        );
        u[7] = half_btf_0_neon_r(
            cospi.add(8),
            v.as_mut_ptr().add(4),
            &raw const v_bit,
            &raw const rnding,
        );

        i = 8;
        while i < 16 {
            addsub_neon(
                v[(i + 0) as usize],
                v[(i + 1) as usize],
                u.as_mut_ptr().offset((i + 0) as isize),
                u.as_mut_ptr().offset((i + 1) as isize),
                &raw const clamp_lo,
                &raw const clamp_hi,
            );
            addsub_neon(
                v[(i + 3) as usize],
                v[(i + 2) as usize],
                u.as_mut_ptr().offset((i + 3) as isize),
                u.as_mut_ptr().offset((i + 2) as isize),
                &raw const clamp_lo,
                &raw const clamp_hi,
            );
            i += 4;
        }

        i = 16;
        while i < 32 {
            u[(i + 0) as usize] = v[(i + 0) as usize];
            u[(i + 3) as usize] = v[(i + 3) as usize];
            i += 4;
        }

        u[17] = half_btf_neon_mode10_r(
            cospi.add(8),
            v.as_mut_ptr().add(17),
            cospi.add(56),
            v.as_mut_ptr().add(30),
            &raw const v_bit,
            &raw const rnding,
        );
        u[18] = half_btf_neon_mode11_r(
            cospi.add(56),
            v.as_mut_ptr().add(18),
            cospi.add(8),
            v.as_mut_ptr().add(29),
            &raw const v_bit,
            &raw const rnding,
        );
        u[21] = half_btf_neon_mode10_r(
            cospi.add(40),
            v.as_mut_ptr().add(21),
            cospi.add(24),
            v.as_mut_ptr().add(26),
            &raw const v_bit,
            &raw const rnding,
        );
        u[22] = half_btf_neon_mode11_r(
            cospi.add(24),
            v.as_mut_ptr().add(22),
            cospi.add(40),
            v.as_mut_ptr().add(25),
            &raw const v_bit,
            &raw const rnding,
        );
        u[25] = half_btf_neon_mode10_r(
            cospi.add(40),
            v.as_mut_ptr().add(22),
            cospi.add(24),
            v.as_mut_ptr().add(25),
            &raw const v_bit,
            &raw const rnding,
        );
        u[26] = half_btf_neon_r(
            cospi.add(24),
            v.as_mut_ptr().add(21),
            cospi.add(40),
            v.as_mut_ptr().add(26),
            &raw const v_bit,
            &raw const rnding,
        );
        u[29] = half_btf_neon_mode10_r(
            cospi.add(8),
            v.as_mut_ptr().add(18),
            cospi.add(56),
            v.as_mut_ptr().add(29),
            &raw const v_bit,
            &raw const rnding,
        );
        u[30] = half_btf_neon_r(
            cospi.add(56),
            v.as_mut_ptr().add(17),
            cospi.add(8),
            v.as_mut_ptr().add(30),
            &raw const v_bit,
            &raw const rnding,
        );

        i = 32;
        while i < 64 {
            addsub_neon(
                v[(i + 0) as usize],
                v[(i + 3) as usize],
                u.as_mut_ptr().offset((i + 0) as isize),
                u.as_mut_ptr().offset((i + 3) as isize),
                &raw const clamp_lo,
                &raw const clamp_hi,
            );
            addsub_neon(
                v[(i + 1) as usize],
                v[(i + 2) as usize],
                u.as_mut_ptr().offset((i + 1) as isize),
                u.as_mut_ptr().offset((i + 2) as isize),
                &raw const clamp_lo,
                &raw const clamp_hi,
            );

            addsub_neon(
                v[(i + 7) as usize],
                v[(i + 4) as usize],
                u.as_mut_ptr().offset((i + 7) as isize),
                u.as_mut_ptr().offset((i + 4) as isize),
                &raw const clamp_lo,
                &raw const clamp_hi,
            );
            addsub_neon(
                v[(i + 6) as usize],
                v[(i + 5) as usize],
                u.as_mut_ptr().offset((i + 6) as isize),
                u.as_mut_ptr().offset((i + 5) as isize),
                &raw const clamp_lo,
                &raw const clamp_hi,
            );
            i += 8;
        }

        // stage 6
        v[0] =
            half_btf_0_neon_r(cospi.add(32), u.as_mut_ptr(), &raw const v_bit, &raw const rnding);
        v[1] =
            half_btf_0_neon_r(cospi.add(32), u.as_mut_ptr(), &raw const v_bit, &raw const rnding);
        v[2] = half_btf_0_neon_r(
            cospi.add(48),
            u.as_mut_ptr().add(2),
            &raw const v_bit,
            &raw const rnding,
        );
        v[3] = half_btf_0_neon_r(
            cospi.add(16),
            u.as_mut_ptr().add(2),
            &raw const v_bit,
            &raw const rnding,
        );

        addsub_neon(
            u[4],
            u[5],
            v.as_mut_ptr().add(4),
            v.as_mut_ptr().add(5),
            &raw const clamp_lo,
            &raw const clamp_hi,
        );
        addsub_neon(
            u[7],
            u[6],
            v.as_mut_ptr().add(7),
            v.as_mut_ptr().add(6),
            &raw const clamp_lo,
            &raw const clamp_hi,
        );

        i = 8;
        while i < 16 {
            v[(i + 0) as usize] = u[(i + 0) as usize];
            v[(i + 3) as usize] = u[(i + 3) as usize];
            i += 4;
        }

        v[9] = half_btf_neon_mode10_r(
            cospi.add(16),
            u.as_mut_ptr().add(9),
            cospi.add(48),
            u.as_mut_ptr().add(14),
            &raw const v_bit,
            &raw const rnding,
        );
        v[10] = half_btf_neon_mode11_r(
            cospi.add(48),
            u.as_mut_ptr().add(10),
            cospi.add(16),
            u.as_mut_ptr().add(13),
            &raw const v_bit,
            &raw const rnding,
        );
        v[13] = half_btf_neon_mode10_r(
            cospi.add(16),
            u.as_mut_ptr().add(10),
            cospi.add(48),
            u.as_mut_ptr().add(13),
            &raw const v_bit,
            &raw const rnding,
        );
        v[14] = half_btf_neon_r(
            cospi.add(48),
            u.as_mut_ptr().add(9),
            cospi.add(16),
            u.as_mut_ptr().add(14),
            &raw const v_bit,
            &raw const rnding,
        );

        i = 16;
        while i < 32 {
            addsub_neon(
                u[(i + 0) as usize],
                u[(i + 3) as usize],
                v.as_mut_ptr().offset((i + 0) as isize),
                v.as_mut_ptr().offset((i + 3) as isize),
                &raw const clamp_lo,
                &raw const clamp_hi,
            );
            addsub_neon(
                u[(i + 1) as usize],
                u[(i + 2) as usize],
                v.as_mut_ptr().offset((i + 1) as isize),
                v.as_mut_ptr().offset((i + 2) as isize),
                &raw const clamp_lo,
                &raw const clamp_hi,
            );

            addsub_neon(
                u[(i + 7) as usize],
                u[(i + 4) as usize],
                v.as_mut_ptr().offset((i + 7) as isize),
                v.as_mut_ptr().offset((i + 4) as isize),
                &raw const clamp_lo,
                &raw const clamp_hi,
            );
            addsub_neon(
                u[(i + 6) as usize],
                u[(i + 5) as usize],
                v.as_mut_ptr().offset((i + 6) as isize),
                v.as_mut_ptr().offset((i + 5) as isize),
                &raw const clamp_lo,
                &raw const clamp_hi,
            );
            i += 8;
        }

        i = 32;
        while i < 64 {
            v[(i + 0) as usize] = u[(i + 0) as usize];
            v[(i + 1) as usize] = u[(i + 1) as usize];
            v[(i + 6) as usize] = u[(i + 6) as usize];
            v[(i + 7) as usize] = u[(i + 7) as usize];
            i += 8;
        }

        v[34] = half_btf_neon_mode10_r(
            cospi.add(8),
            u.as_mut_ptr().add(34),
            cospi.add(56),
            u.as_mut_ptr().add(61),
            &raw const v_bit,
            &raw const rnding,
        );
        v[35] = half_btf_neon_mode10_r(
            cospi.add(8),
            u.as_mut_ptr().add(35),
            cospi.add(56),
            u.as_mut_ptr().add(60),
            &raw const v_bit,
            &raw const rnding,
        );
        v[36] = half_btf_neon_mode11_r(
            cospi.add(56),
            u.as_mut_ptr().add(36),
            cospi.add(8),
            u.as_mut_ptr().add(59),
            &raw const v_bit,
            &raw const rnding,
        );
        v[37] = half_btf_neon_mode11_r(
            cospi.add(56),
            u.as_mut_ptr().add(37),
            cospi.add(8),
            u.as_mut_ptr().add(58),
            &raw const v_bit,
            &raw const rnding,
        );
        v[42] = half_btf_neon_mode10_r(
            cospi.add(40),
            u.as_mut_ptr().add(42),
            cospi.add(24),
            u.as_mut_ptr().add(53),
            &raw const v_bit,
            &raw const rnding,
        );
        v[43] = half_btf_neon_mode10_r(
            cospi.add(40),
            u.as_mut_ptr().add(43),
            cospi.add(24),
            u.as_mut_ptr().add(52),
            &raw const v_bit,
            &raw const rnding,
        );
        v[44] = half_btf_neon_mode11_r(
            cospi.add(24),
            u.as_mut_ptr().add(44),
            cospi.add(40),
            u.as_mut_ptr().add(51),
            &raw const v_bit,
            &raw const rnding,
        );
        v[45] = half_btf_neon_mode11_r(
            cospi.add(24),
            u.as_mut_ptr().add(45),
            cospi.add(40),
            u.as_mut_ptr().add(50),
            &raw const v_bit,
            &raw const rnding,
        );
        v[50] = half_btf_neon_mode10_r(
            cospi.add(40),
            u.as_mut_ptr().add(45),
            cospi.add(24),
            u.as_mut_ptr().add(50),
            &raw const v_bit,
            &raw const rnding,
        );
        v[51] = half_btf_neon_mode10_r(
            cospi.add(40),
            u.as_mut_ptr().add(44),
            cospi.add(24),
            u.as_mut_ptr().add(51),
            &raw const v_bit,
            &raw const rnding,
        );
        v[52] = half_btf_neon_r(
            cospi.add(24),
            u.as_mut_ptr().add(43),
            cospi.add(40),
            u.as_mut_ptr().add(52),
            &raw const v_bit,
            &raw const rnding,
        );
        v[53] = half_btf_neon_r(
            cospi.add(24),
            u.as_mut_ptr().add(42),
            cospi.add(40),
            u.as_mut_ptr().add(53),
            &raw const v_bit,
            &raw const rnding,
        );
        v[58] = half_btf_neon_mode10_r(
            cospi.add(8),
            u.as_mut_ptr().add(37),
            cospi.add(56),
            u.as_mut_ptr().add(58),
            &raw const v_bit,
            &raw const rnding,
        );
        v[59] = half_btf_neon_mode10_r(
            cospi.add(8),
            u.as_mut_ptr().add(36),
            cospi.add(56),
            u.as_mut_ptr().add(59),
            &raw const v_bit,
            &raw const rnding,
        );
        v[60] = half_btf_neon_r(
            cospi.add(56),
            u.as_mut_ptr().add(35),
            cospi.add(8),
            u.as_mut_ptr().add(60),
            &raw const v_bit,
            &raw const rnding,
        );
        v[61] = half_btf_neon_r(
            cospi.add(56),
            u.as_mut_ptr().add(34),
            cospi.add(8),
            u.as_mut_ptr().add(61),
            &raw const v_bit,
            &raw const rnding,
        );

        // stage 7
        addsub_neon(
            v[0],
            v[3],
            u.as_mut_ptr(),
            u.as_mut_ptr().add(3),
            &raw const clamp_lo,
            &raw const clamp_hi,
        );
        addsub_neon(
            v[1],
            v[2],
            u.as_mut_ptr().add(1),
            u.as_mut_ptr().add(2),
            &raw const clamp_lo,
            &raw const clamp_hi,
        );

        u[4] = v[4];
        u[7] = v[7];
        u[5] = half_btf_neon_mode10_r(
            cospi.add(32),
            v.as_mut_ptr().add(5),
            cospi.add(32),
            v.as_mut_ptr().add(6),
            &raw const v_bit,
            &raw const rnding,
        );
        u[6] = half_btf_neon_r(
            cospi.add(32),
            v.as_mut_ptr().add(5),
            cospi.add(32),
            v.as_mut_ptr().add(6),
            &raw const v_bit,
            &raw const rnding,
        );

        addsub_neon(
            v[8],
            v[11],
            u.as_mut_ptr().add(8),
            u.as_mut_ptr().add(11),
            &raw const clamp_lo,
            &raw const clamp_hi,
        );
        addsub_neon(
            v[9],
            v[10],
            u.as_mut_ptr().add(9),
            u.as_mut_ptr().add(10),
            &raw const clamp_lo,
            &raw const clamp_hi,
        );
        addsub_neon(
            v[15],
            v[12],
            u.as_mut_ptr().add(15),
            u.as_mut_ptr().add(12),
            &raw const clamp_lo,
            &raw const clamp_hi,
        );
        addsub_neon(
            v[14],
            v[13],
            u.as_mut_ptr().add(14),
            u.as_mut_ptr().add(13),
            &raw const clamp_lo,
            &raw const clamp_hi,
        );

        i = 16;
        while i < 32 {
            u[(i + 0) as usize] = v[(i + 0) as usize];
            u[(i + 1) as usize] = v[(i + 1) as usize];
            u[(i + 6) as usize] = v[(i + 6) as usize];
            u[(i + 7) as usize] = v[(i + 7) as usize];
            i += 8;
        }

        u[18] = half_btf_neon_mode10_r(
            cospi.add(16),
            v.as_mut_ptr().add(18),
            cospi.add(48),
            v.as_mut_ptr().add(29),
            &raw const v_bit,
            &raw const rnding,
        );
        u[19] = half_btf_neon_mode10_r(
            cospi.add(16),
            v.as_mut_ptr().add(19),
            cospi.add(48),
            v.as_mut_ptr().add(28),
            &raw const v_bit,
            &raw const rnding,
        );
        u[20] = half_btf_neon_mode11_r(
            cospi.add(48),
            v.as_mut_ptr().add(20),
            cospi.add(16),
            v.as_mut_ptr().add(27),
            &raw const v_bit,
            &raw const rnding,
        );
        u[21] = half_btf_neon_mode11_r(
            cospi.add(48),
            v.as_mut_ptr().add(21),
            cospi.add(16),
            v.as_mut_ptr().add(26),
            &raw const v_bit,
            &raw const rnding,
        );
        u[26] = half_btf_neon_mode10_r(
            cospi.add(16),
            v.as_mut_ptr().add(21),
            cospi.add(48),
            v.as_mut_ptr().add(26),
            &raw const v_bit,
            &raw const rnding,
        );
        u[27] = half_btf_neon_mode10_r(
            cospi.add(16),
            v.as_mut_ptr().add(20),
            cospi.add(48),
            v.as_mut_ptr().add(27),
            &raw const v_bit,
            &raw const rnding,
        );
        u[28] = half_btf_neon_r(
            cospi.add(48),
            v.as_mut_ptr().add(19),
            cospi.add(16),
            v.as_mut_ptr().add(28),
            &raw const v_bit,
            &raw const rnding,
        );
        u[29] = half_btf_neon_r(
            cospi.add(48),
            v.as_mut_ptr().add(18),
            cospi.add(16),
            v.as_mut_ptr().add(29),
            &raw const v_bit,
            &raw const rnding,
        );

        i = 32;
        while i < 64 {
            j = i;
            while j < i + 4 {
                addsub_neon(
                    v[j as usize],
                    v[(j ^ 7) as usize],
                    u.as_mut_ptr().offset(j as isize),
                    u.as_mut_ptr().offset((j ^ 7) as isize),
                    &raw const clamp_lo,
                    &raw const clamp_hi,
                );
                addsub_neon(
                    v[(j ^ 15) as usize],
                    v[(j ^ 8) as usize],
                    u.as_mut_ptr().offset((j ^ 15) as isize),
                    u.as_mut_ptr().offset((j ^ 8) as isize),
                    &raw const clamp_lo,
                    &raw const clamp_hi,
                );
                j += 1;
            }
            i += 16;
        }

        // stage 8
        i = 0;
        while i < 4 {
            addsub_neon(
                u[i as usize],
                u[(7 - i) as usize],
                v.as_mut_ptr().offset(i as isize),
                v.as_mut_ptr().offset((7 - i) as isize),
                &raw const clamp_lo,
                &raw const clamp_hi,
            );
            i += 1;
        }

        v[8] = u[8];
        v[9] = u[9];
        v[14] = u[14];
        v[15] = u[15];

        v[10] = half_btf_neon_mode10_r(
            cospi.add(32),
            u.as_mut_ptr().add(10),
            cospi.add(32),
            u.as_mut_ptr().add(13),
            &raw const v_bit,
            &raw const rnding,
        );
        v[11] = half_btf_neon_mode10_r(
            cospi.add(32),
            u.as_mut_ptr().add(11),
            cospi.add(32),
            u.as_mut_ptr().add(12),
            &raw const v_bit,
            &raw const rnding,
        );
        v[12] = half_btf_neon_r(
            cospi.add(32),
            u.as_mut_ptr().add(11),
            cospi.add(32),
            u.as_mut_ptr().add(12),
            &raw const v_bit,
            &raw const rnding,
        );
        v[13] = half_btf_neon_r(
            cospi.add(32),
            u.as_mut_ptr().add(10),
            cospi.add(32),
            u.as_mut_ptr().add(13),
            &raw const v_bit,
            &raw const rnding,
        );

        i = 16;
        while i < 20 {
            addsub_neon(
                u[i as usize],
                u[(i ^ 7) as usize],
                v.as_mut_ptr().offset(i as isize),
                v.as_mut_ptr().offset((i ^ 7) as isize),
                &raw const clamp_lo,
                &raw const clamp_hi,
            );
            addsub_neon(
                u[(i ^ 15) as usize],
                u[(i ^ 8) as usize],
                v.as_mut_ptr().offset((i ^ 15) as isize),
                v.as_mut_ptr().offset((i ^ 8) as isize),
                &raw const clamp_lo,
                &raw const clamp_hi,
            );
            i += 1;
        }

        i = 32;
        while i < 36 {
            v[i as usize] = u[i as usize];
            v[(i + 12) as usize] = u[(i + 12) as usize];
            v[(i + 16) as usize] = u[(i + 16) as usize];
            v[(i + 28) as usize] = u[(i + 28) as usize];
            i += 1;
        }

        v[36] = half_btf_neon_mode10_r(
            cospi.add(16),
            u.as_mut_ptr().add(36),
            cospi.add(48),
            u.as_mut_ptr().add(59),
            &raw const v_bit,
            &raw const rnding,
        );
        v[37] = half_btf_neon_mode10_r(
            cospi.add(16),
            u.as_mut_ptr().add(37),
            cospi.add(48),
            u.as_mut_ptr().add(58),
            &raw const v_bit,
            &raw const rnding,
        );
        v[38] = half_btf_neon_mode10_r(
            cospi.add(16),
            u.as_mut_ptr().add(38),
            cospi.add(48),
            u.as_mut_ptr().add(57),
            &raw const v_bit,
            &raw const rnding,
        );
        v[39] = half_btf_neon_mode10_r(
            cospi.add(16),
            u.as_mut_ptr().add(39),
            cospi.add(48),
            u.as_mut_ptr().add(56),
            &raw const v_bit,
            &raw const rnding,
        );
        v[40] = half_btf_neon_mode11_r(
            cospi.add(48),
            u.as_mut_ptr().add(40),
            cospi.add(16),
            u.as_mut_ptr().add(55),
            &raw const v_bit,
            &raw const rnding,
        );
        v[41] = half_btf_neon_mode11_r(
            cospi.add(48),
            u.as_mut_ptr().add(41),
            cospi.add(16),
            u.as_mut_ptr().add(54),
            &raw const v_bit,
            &raw const rnding,
        );
        v[42] = half_btf_neon_mode11_r(
            cospi.add(48),
            u.as_mut_ptr().add(42),
            cospi.add(16),
            u.as_mut_ptr().add(53),
            &raw const v_bit,
            &raw const rnding,
        );
        v[43] = half_btf_neon_mode11_r(
            cospi.add(48),
            u.as_mut_ptr().add(43),
            cospi.add(16),
            u.as_mut_ptr().add(52),
            &raw const v_bit,
            &raw const rnding,
        );
        v[52] = half_btf_neon_mode10_r(
            cospi.add(16),
            u.as_mut_ptr().add(43),
            cospi.add(48),
            u.as_mut_ptr().add(52),
            &raw const v_bit,
            &raw const rnding,
        );
        v[53] = half_btf_neon_mode10_r(
            cospi.add(16),
            u.as_mut_ptr().add(42),
            cospi.add(48),
            u.as_mut_ptr().add(53),
            &raw const v_bit,
            &raw const rnding,
        );
        v[54] = half_btf_neon_mode10_r(
            cospi.add(16),
            u.as_mut_ptr().add(41),
            cospi.add(48),
            u.as_mut_ptr().add(54),
            &raw const v_bit,
            &raw const rnding,
        );
        v[55] = half_btf_neon_mode10_r(
            cospi.add(16),
            u.as_mut_ptr().add(40),
            cospi.add(48),
            u.as_mut_ptr().add(55),
            &raw const v_bit,
            &raw const rnding,
        );
        v[56] = half_btf_neon_r(
            cospi.add(48),
            u.as_mut_ptr().add(39),
            cospi.add(16),
            u.as_mut_ptr().add(56),
            &raw const v_bit,
            &raw const rnding,
        );
        v[57] = half_btf_neon_r(
            cospi.add(48),
            u.as_mut_ptr().add(38),
            cospi.add(16),
            u.as_mut_ptr().add(57),
            &raw const v_bit,
            &raw const rnding,
        );
        v[58] = half_btf_neon_r(
            cospi.add(48),
            u.as_mut_ptr().add(37),
            cospi.add(16),
            u.as_mut_ptr().add(58),
            &raw const v_bit,
            &raw const rnding,
        );
        v[59] = half_btf_neon_r(
            cospi.add(48),
            u.as_mut_ptr().add(36),
            cospi.add(16),
            u.as_mut_ptr().add(59),
            &raw const v_bit,
            &raw const rnding,
        );

        // stage 9
        i = 0;
        while i < 8 {
            addsub_neon(
                v[i as usize],
                v[(15 - i) as usize],
                u.as_mut_ptr().offset(i as isize),
                u.as_mut_ptr().offset((15 - i) as isize),
                &raw const clamp_lo,
                &raw const clamp_hi,
            );
            i += 1;
        }

        i = 16;
        while i < 20 {
            u[i as usize] = v[i as usize];
            u[(i + 12) as usize] = v[(i + 12) as usize];
            i += 1;
        }

        u[20] = half_btf_neon_mode10_r(
            cospi.add(32),
            v.as_mut_ptr().add(20),
            cospi.add(32),
            v.as_mut_ptr().add(27),
            &raw const v_bit,
            &raw const rnding,
        );
        u[21] = half_btf_neon_mode10_r(
            cospi.add(32),
            v.as_mut_ptr().add(21),
            cospi.add(32),
            v.as_mut_ptr().add(26),
            &raw const v_bit,
            &raw const rnding,
        );
        u[22] = half_btf_neon_mode10_r(
            cospi.add(32),
            v.as_mut_ptr().add(22),
            cospi.add(32),
            v.as_mut_ptr().add(25),
            &raw const v_bit,
            &raw const rnding,
        );
        u[23] = half_btf_neon_mode10_r(
            cospi.add(32),
            v.as_mut_ptr().add(23),
            cospi.add(32),
            v.as_mut_ptr().add(24),
            &raw const v_bit,
            &raw const rnding,
        );
        u[24] = half_btf_neon_r(
            cospi.add(32),
            v.as_mut_ptr().add(23),
            cospi.add(32),
            v.as_mut_ptr().add(24),
            &raw const v_bit,
            &raw const rnding,
        );
        u[25] = half_btf_neon_r(
            cospi.add(32),
            v.as_mut_ptr().add(22),
            cospi.add(32),
            v.as_mut_ptr().add(25),
            &raw const v_bit,
            &raw const rnding,
        );
        u[26] = half_btf_neon_r(
            cospi.add(32),
            v.as_mut_ptr().add(21),
            cospi.add(32),
            v.as_mut_ptr().add(26),
            &raw const v_bit,
            &raw const rnding,
        );
        u[27] = half_btf_neon_r(
            cospi.add(32),
            v.as_mut_ptr().add(20),
            cospi.add(32),
            v.as_mut_ptr().add(27),
            &raw const v_bit,
            &raw const rnding,
        );

        i = 32;
        while i < 40 {
            addsub_neon(
                v[i as usize],
                v[(i ^ 15) as usize],
                u.as_mut_ptr().offset(i as isize),
                u.as_mut_ptr().offset((i ^ 15) as isize),
                &raw const clamp_lo,
                &raw const clamp_hi,
            );
            i += 1;
        }

        i = 48;
        while i < 56 {
            addsub_neon(
                v[(i ^ 15) as usize],
                v[i as usize],
                u.as_mut_ptr().offset((i ^ 15) as isize),
                u.as_mut_ptr().offset(i as isize),
                &raw const clamp_lo,
                &raw const clamp_hi,
            );
            i += 1;
        }

        // stage 10
        i = 0;
        while i < 16 {
            addsub_neon(
                u[i as usize],
                u[(31 - i) as usize],
                v.as_mut_ptr().offset(i as isize),
                v.as_mut_ptr().offset((31 - i) as isize),
                &raw const clamp_lo,
                &raw const clamp_hi,
            );
            i += 1;
        }

        i = 32;
        while i < 40 {
            v[i as usize] = u[i as usize];
            i += 1;
        }

        v[40] = half_btf_neon_mode10_r(
            cospi.add(32),
            u.as_mut_ptr().add(40),
            cospi.add(32),
            u.as_mut_ptr().add(55),
            &raw const v_bit,
            &raw const rnding,
        );
        v[41] = half_btf_neon_mode10_r(
            cospi.add(32),
            u.as_mut_ptr().add(41),
            cospi.add(32),
            u.as_mut_ptr().add(54),
            &raw const v_bit,
            &raw const rnding,
        );
        v[42] = half_btf_neon_mode10_r(
            cospi.add(32),
            u.as_mut_ptr().add(42),
            cospi.add(32),
            u.as_mut_ptr().add(53),
            &raw const v_bit,
            &raw const rnding,
        );
        v[43] = half_btf_neon_mode10_r(
            cospi.add(32),
            u.as_mut_ptr().add(43),
            cospi.add(32),
            u.as_mut_ptr().add(52),
            &raw const v_bit,
            &raw const rnding,
        );
        v[44] = half_btf_neon_mode10_r(
            cospi.add(32),
            u.as_mut_ptr().add(44),
            cospi.add(32),
            u.as_mut_ptr().add(51),
            &raw const v_bit,
            &raw const rnding,
        );
        v[45] = half_btf_neon_mode10_r(
            cospi.add(32),
            u.as_mut_ptr().add(45),
            cospi.add(32),
            u.as_mut_ptr().add(50),
            &raw const v_bit,
            &raw const rnding,
        );
        v[46] = half_btf_neon_mode10_r(
            cospi.add(32),
            u.as_mut_ptr().add(46),
            cospi.add(32),
            u.as_mut_ptr().add(49),
            &raw const v_bit,
            &raw const rnding,
        );
        v[47] = half_btf_neon_mode10_r(
            cospi.add(32),
            u.as_mut_ptr().add(47),
            cospi.add(32),
            u.as_mut_ptr().add(48),
            &raw const v_bit,
            &raw const rnding,
        );
        v[48] = half_btf_neon_r(
            cospi.add(32),
            u.as_mut_ptr().add(47),
            cospi.add(32),
            u.as_mut_ptr().add(48),
            &raw const v_bit,
            &raw const rnding,
        );
        v[49] = half_btf_neon_r(
            cospi.add(32),
            u.as_mut_ptr().add(46),
            cospi.add(32),
            u.as_mut_ptr().add(49),
            &raw const v_bit,
            &raw const rnding,
        );
        v[50] = half_btf_neon_r(
            cospi.add(32),
            u.as_mut_ptr().add(45),
            cospi.add(32),
            u.as_mut_ptr().add(50),
            &raw const v_bit,
            &raw const rnding,
        );
        v[51] = half_btf_neon_r(
            cospi.add(32),
            u.as_mut_ptr().add(44),
            cospi.add(32),
            u.as_mut_ptr().add(51),
            &raw const v_bit,
            &raw const rnding,
        );
        v[52] = half_btf_neon_r(
            cospi.add(32),
            u.as_mut_ptr().add(43),
            cospi.add(32),
            u.as_mut_ptr().add(52),
            &raw const v_bit,
            &raw const rnding,
        );
        v[53] = half_btf_neon_r(
            cospi.add(32),
            u.as_mut_ptr().add(42),
            cospi.add(32),
            u.as_mut_ptr().add(53),
            &raw const v_bit,
            &raw const rnding,
        );
        v[54] = half_btf_neon_r(
            cospi.add(32),
            u.as_mut_ptr().add(41),
            cospi.add(32),
            u.as_mut_ptr().add(54),
            &raw const v_bit,
            &raw const rnding,
        );
        v[55] = half_btf_neon_r(
            cospi.add(32),
            u.as_mut_ptr().add(40),
            cospi.add(32),
            u.as_mut_ptr().add(55),
            &raw const v_bit,
            &raw const rnding,
        );

        i = 56;
        while i < 64 {
            v[i as usize] = u[i as usize];
            i += 1;
        }

        // stage 11
        i = 0;
        while i < 32 {
            addsub_neon(
                v[i as usize],
                v[(63 - i) as usize],
                out.offset(i as isize),
                out.offset((63 - i) as isize),
                &raw const clamp_lo,
                &raw const clamp_hi,
            );
            i += 1;
        }

        if !(do_cols != 0) {
            let log_range_out: i32 = get_log_range_out(bd);
            let clamp_lo_out: int32x4_t = vdupq_n_s32(-(1 << (log_range_out - 1)));
            let clamp_hi_out: int32x4_t = vdupq_n_s32((1 << (log_range_out - 1)) - 1);
            i = 0;
            while i < 64 {
                round_shift_4x4(out.offset(i as isize), out_shift);
                highbd_clamp_s32_neon(
                    out.offset(i as isize),
                    out.offset(i as isize),
                    &raw const clamp_lo_out,
                    &raw const clamp_hi_out,
                    4,
                );
                i += 4;
            }
        }
    }
}

unsafe fn idct32x32_low1_neon(
    mut in_: *mut int32x4_t,
    mut out: *mut int32x4_t,
    mut bit: i32,
    mut do_cols: i32,
    mut bd: i32,
    mut out_shift: i32,
) {
    let mut cospi: *const i32 = cospi_arr(bit);
    let log_range: i32 = get_log_range(bd, do_cols);
    let mut clamp_lo: int32x4_t = vdupq_n_s32(-(1 << (log_range - 1)));
    let mut clamp_hi: int32x4_t = vdupq_n_s32((1 << (log_range - 1)) - 1);
    let mut bf1: int32x4_t = core::mem::zeroed();
    let v_bit: int32x4_t = vdupq_n_s32(-bit);
    let rnding: int32x4_t = vdupq_n_s32(1 << (bit - 1));
    // stage 0-1
    bf1 = *in_;

    // stage 2-5
    bf1 = half_btf_0_neon_r(cospi.add(32), &raw mut bf1, &raw const v_bit, &raw const rnding);

    // stage 6-9
    if do_cols != 0 {
        bf1 = vmaxq_s32(bf1, clamp_lo);
        bf1 = vminq_s32(bf1, clamp_hi);
    } else {
        let log_range_out: i32 = get_log_range_out(bd);
        clamp_lo = vdupq_n_s32(-(1 << (log_range_out - 1)));
        clamp_hi = vdupq_n_s32((1 << (log_range_out - 1)) - 1);
        if out_shift != 0 {
            bf1 = vrshlq_s32(bf1, vdupq_n_s32(-out_shift));
        }
    }

    bf1 = vmaxq_s32(bf1, clamp_lo);
    bf1 = vminq_s32(bf1, clamp_hi);

    for i in 0..32 {
        *out.offset(i as isize) = bf1;
    }
}

unsafe fn idct32x32_low8_neon(
    mut in_: *mut int32x4_t,
    mut out: *mut int32x4_t,
    mut bit: i32,
    mut do_cols: i32,
    mut bd: i32,
    mut out_shift: i32,
) {
    let mut cospi: *const i32 = cospi_arr(bit);
    let log_range: i32 = get_log_range(bd, do_cols);
    let clamp_lo: int32x4_t = vdupq_n_s32(-(1 << (log_range - 1)));
    let clamp_hi: int32x4_t = vdupq_n_s32((1 << (log_range - 1)) - 1);
    let mut bf1: [int32x4_t; 32] = core::mem::zeroed();
    let v_bit: int32x4_t = vdupq_n_s32(-bit);
    let rnding: int32x4_t = vdupq_n_s32(1 << (bit - 1));

    // stage 0-1
    bf1[0] = *in_;
    bf1[4] = *in_.add(4);
    bf1[8] = *in_.add(2);
    bf1[12] = *in_.add(6);
    bf1[16] = *in_.add(1);
    bf1[20] = *in_.add(5);
    bf1[24] = *in_.add(3);
    bf1[28] = *in_.add(7);

    // stage 2
    bf1[31] = half_btf_0_neon_r(
        cospi.add(2),
        bf1.as_mut_ptr().add(16),
        &raw const v_bit,
        &raw const rnding,
    );
    bf1[16] = half_btf_0_neon_r(
        cospi.add(62),
        bf1.as_mut_ptr().add(16),
        &raw const v_bit,
        &raw const rnding,
    );
    bf1[19] = half_btf_0_m_neon_r(
        cospi.add(50),
        bf1.as_mut_ptr().add(28),
        &raw const v_bit,
        &raw const rnding,
    );
    bf1[28] = half_btf_0_neon_r(
        cospi.add(14),
        bf1.as_mut_ptr().add(28),
        &raw const v_bit,
        &raw const rnding,
    );
    bf1[27] = half_btf_0_neon_r(
        cospi.add(10),
        bf1.as_mut_ptr().add(20),
        &raw const v_bit,
        &raw const rnding,
    );
    bf1[20] = half_btf_0_neon_r(
        cospi.add(54),
        bf1.as_mut_ptr().add(20),
        &raw const v_bit,
        &raw const rnding,
    );
    bf1[23] = half_btf_0_m_neon_r(
        cospi.add(58),
        bf1.as_mut_ptr().add(24),
        &raw const v_bit,
        &raw const rnding,
    );
    bf1[24] = half_btf_0_neon_r(
        cospi.add(6),
        bf1.as_mut_ptr().add(24),
        &raw const v_bit,
        &raw const rnding,
    );

    // stage 3
    bf1[15] = half_btf_0_neon_r(
        cospi.add(4),
        bf1.as_mut_ptr().add(8),
        &raw const v_bit,
        &raw const rnding,
    );
    bf1[8] = half_btf_0_neon_r(
        cospi.add(60),
        bf1.as_mut_ptr().add(8),
        &raw const v_bit,
        &raw const rnding,
    );

    bf1[11] = half_btf_0_m_neon_r(
        cospi.add(52),
        bf1.as_mut_ptr().add(12),
        &raw const v_bit,
        &raw const rnding,
    );
    bf1[12] = half_btf_0_neon_r(
        cospi.add(12),
        bf1.as_mut_ptr().add(12),
        &raw const v_bit,
        &raw const rnding,
    );
    bf1[17] = bf1[16];
    bf1[18] = bf1[19];
    bf1[21] = bf1[20];
    bf1[22] = bf1[23];
    bf1[25] = bf1[24];
    bf1[26] = bf1[27];
    bf1[29] = bf1[28];
    bf1[30] = bf1[31];

    // stage 4 :
    bf1[7] = half_btf_0_neon_r(
        cospi.add(8),
        bf1.as_mut_ptr().add(4),
        &raw const v_bit,
        &raw const rnding,
    );
    bf1[4] = half_btf_0_neon_r(
        cospi.add(56),
        bf1.as_mut_ptr().add(4),
        &raw const v_bit,
        &raw const rnding,
    );

    bf1[9] = bf1[8];
    bf1[10] = bf1[11];
    bf1[13] = bf1[12];
    bf1[14] = bf1[15];

    idct32_stage4_neon(bf1.as_mut_ptr(), cospi, &raw const v_bit, &raw const rnding);

    // stage 5
    bf1[0] =
        half_btf_0_neon_r(cospi.add(32), bf1.as_mut_ptr(), &raw const v_bit, &raw const rnding);
    bf1[1] = bf1[0];
    bf1[5] = bf1[4];
    bf1[6] = bf1[7];

    idct32_stage5_neon(
        bf1.as_mut_ptr(),
        cospi,
        &raw const clamp_lo,
        &raw const clamp_hi,
        &raw const v_bit,
        &raw const rnding,
    );

    // stage 6
    bf1[3] = bf1[0];
    bf1[2] = bf1[1];

    idct32_stage6_neon(
        bf1.as_mut_ptr(),
        cospi,
        &raw const clamp_lo,
        &raw const clamp_hi,
        &raw const v_bit,
        &raw const rnding,
    );

    // stage 7
    idct32_stage7_neon(
        bf1.as_mut_ptr(),
        cospi,
        &raw const clamp_lo,
        &raw const clamp_hi,
        &raw const v_bit,
        &raw const rnding,
    );

    // stage 8
    idct32_stage8_neon(
        bf1.as_mut_ptr(),
        cospi,
        &raw const clamp_lo,
        &raw const clamp_hi,
        &raw const v_bit,
        &raw const rnding,
    );

    // stage 9
    idct32_stage9_neon(
        bf1.as_mut_ptr(),
        out,
        do_cols,
        bd,
        out_shift,
        &raw const clamp_lo,
        &raw const clamp_hi,
    );
}

unsafe fn idct32x32_low16_neon(
    mut in_: *mut int32x4_t,
    mut out: *mut int32x4_t,
    mut bit: i32,
    mut do_cols: i32,
    mut bd: i32,
    mut out_shift: i32,
) {
    let mut cospi: *const i32 = cospi_arr(bit);
    let log_range: i32 = get_log_range(bd, do_cols);
    let clamp_lo: int32x4_t = vdupq_n_s32(-(1 << (log_range - 1)));
    let clamp_hi: int32x4_t = vdupq_n_s32((1 << (log_range - 1)) - 1);
    let mut bf1: [int32x4_t; 32] = core::mem::zeroed();
    let v_bit: int32x4_t = vdupq_n_s32(-bit);
    let rnding: int32x4_t = vdupq_n_s32(1 << (bit - 1));

    // stage 0-1
    bf1[0] = *in_;
    bf1[2] = *in_.add(8);
    bf1[4] = *in_.add(4);
    bf1[6] = *in_.add(12);
    bf1[8] = *in_.add(2);
    bf1[10] = *in_.add(10);
    bf1[12] = *in_.add(6);
    bf1[14] = *in_.add(14);
    bf1[16] = *in_.add(1);
    bf1[18] = *in_.add(9);
    bf1[20] = *in_.add(5);
    bf1[22] = *in_.add(13);
    bf1[24] = *in_.add(3);
    bf1[26] = *in_.add(11);
    bf1[28] = *in_.add(7);
    bf1[30] = *in_.add(15);

    // stage 2
    bf1[31] = half_btf_0_neon_r(
        cospi.add(2),
        bf1.as_mut_ptr().add(16),
        &raw const v_bit,
        &raw const rnding,
    );
    bf1[16] = half_btf_0_neon_r(
        cospi.add(62),
        bf1.as_mut_ptr().add(16),
        &raw const v_bit,
        &raw const rnding,
    );
    bf1[17] = half_btf_0_m_neon_r(
        cospi.add(34),
        bf1.as_mut_ptr().add(30),
        &raw const v_bit,
        &raw const rnding,
    );
    bf1[30] = half_btf_0_neon_r(
        cospi.add(30),
        bf1.as_mut_ptr().add(30),
        &raw const v_bit,
        &raw const rnding,
    );
    bf1[29] = half_btf_0_neon_r(
        cospi.add(18),
        bf1.as_mut_ptr().add(18),
        &raw const v_bit,
        &raw const rnding,
    );
    bf1[18] = half_btf_0_neon_r(
        cospi.add(46),
        bf1.as_mut_ptr().add(18),
        &raw const v_bit,
        &raw const rnding,
    );
    bf1[19] = half_btf_0_m_neon_r(
        cospi.add(50),
        bf1.as_mut_ptr().add(28),
        &raw const v_bit,
        &raw const rnding,
    );
    bf1[28] = half_btf_0_neon_r(
        cospi.add(14),
        bf1.as_mut_ptr().add(28),
        &raw const v_bit,
        &raw const rnding,
    );
    bf1[27] = half_btf_0_neon_r(
        cospi.add(10),
        bf1.as_mut_ptr().add(20),
        &raw const v_bit,
        &raw const rnding,
    );
    bf1[20] = half_btf_0_neon_r(
        cospi.add(54),
        bf1.as_mut_ptr().add(20),
        &raw const v_bit,
        &raw const rnding,
    );
    bf1[21] = half_btf_0_m_neon_r(
        cospi.add(42),
        bf1.as_mut_ptr().add(26),
        &raw const v_bit,
        &raw const rnding,
    );
    bf1[26] = half_btf_0_neon_r(
        cospi.add(22),
        bf1.as_mut_ptr().add(26),
        &raw const v_bit,
        &raw const rnding,
    );
    bf1[25] = half_btf_0_neon_r(
        cospi.add(26),
        bf1.as_mut_ptr().add(22),
        &raw const v_bit,
        &raw const rnding,
    );
    bf1[22] = half_btf_0_neon_r(
        cospi.add(38),
        bf1.as_mut_ptr().add(22),
        &raw const v_bit,
        &raw const rnding,
    );
    bf1[23] = half_btf_0_m_neon_r(
        cospi.add(58),
        bf1.as_mut_ptr().add(24),
        &raw const v_bit,
        &raw const rnding,
    );
    bf1[24] = half_btf_0_neon_r(
        cospi.add(6),
        bf1.as_mut_ptr().add(24),
        &raw const v_bit,
        &raw const rnding,
    );

    // stage 3
    bf1[15] = half_btf_0_neon_r(
        cospi.add(4),
        bf1.as_mut_ptr().add(8),
        &raw const v_bit,
        &raw const rnding,
    );
    bf1[8] = half_btf_0_neon_r(
        cospi.add(60),
        bf1.as_mut_ptr().add(8),
        &raw const v_bit,
        &raw const rnding,
    );
    bf1[9] = half_btf_0_m_neon_r(
        cospi.add(36),
        bf1.as_mut_ptr().add(14),
        &raw const v_bit,
        &raw const rnding,
    );
    bf1[14] = half_btf_0_neon_r(
        cospi.add(28),
        bf1.as_mut_ptr().add(14),
        &raw const v_bit,
        &raw const rnding,
    );
    bf1[13] = half_btf_0_neon_r(
        cospi.add(20),
        bf1.as_mut_ptr().add(10),
        &raw const v_bit,
        &raw const rnding,
    );
    bf1[10] = half_btf_0_neon_r(
        cospi.add(44),
        bf1.as_mut_ptr().add(10),
        &raw const v_bit,
        &raw const rnding,
    );
    bf1[11] = half_btf_0_m_neon_r(
        cospi.add(52),
        bf1.as_mut_ptr().add(12),
        &raw const v_bit,
        &raw const rnding,
    );
    bf1[12] = half_btf_0_neon_r(
        cospi.add(12),
        bf1.as_mut_ptr().add(12),
        &raw const v_bit,
        &raw const rnding,
    );

    addsub_neon(
        bf1[16],
        bf1[17],
        bf1.as_mut_ptr().add(16),
        bf1.as_mut_ptr().add(17),
        &raw const clamp_lo,
        &raw const clamp_hi,
    );
    addsub_neon(
        bf1[19],
        bf1[18],
        bf1.as_mut_ptr().add(19),
        bf1.as_mut_ptr().add(18),
        &raw const clamp_lo,
        &raw const clamp_hi,
    );
    addsub_neon(
        bf1[20],
        bf1[21],
        bf1.as_mut_ptr().add(20),
        bf1.as_mut_ptr().add(21),
        &raw const clamp_lo,
        &raw const clamp_hi,
    );
    addsub_neon(
        bf1[23],
        bf1[22],
        bf1.as_mut_ptr().add(23),
        bf1.as_mut_ptr().add(22),
        &raw const clamp_lo,
        &raw const clamp_hi,
    );
    addsub_neon(
        bf1[24],
        bf1[25],
        bf1.as_mut_ptr().add(24),
        bf1.as_mut_ptr().add(25),
        &raw const clamp_lo,
        &raw const clamp_hi,
    );
    addsub_neon(
        bf1[27],
        bf1[26],
        bf1.as_mut_ptr().add(27),
        bf1.as_mut_ptr().add(26),
        &raw const clamp_lo,
        &raw const clamp_hi,
    );
    addsub_neon(
        bf1[28],
        bf1[29],
        bf1.as_mut_ptr().add(28),
        bf1.as_mut_ptr().add(29),
        &raw const clamp_lo,
        &raw const clamp_hi,
    );
    addsub_neon(
        bf1[31],
        bf1[30],
        bf1.as_mut_ptr().add(31),
        bf1.as_mut_ptr().add(30),
        &raw const clamp_lo,
        &raw const clamp_hi,
    );
    // stage 4
    bf1[7] = half_btf_0_neon_r(
        cospi.add(8),
        bf1.as_mut_ptr().add(4),
        &raw const v_bit,
        &raw const rnding,
    );
    bf1[4] = half_btf_0_neon_r(
        cospi.add(56),
        bf1.as_mut_ptr().add(4),
        &raw const v_bit,
        &raw const rnding,
    );
    bf1[5] = half_btf_0_m_neon_r(
        cospi.add(40),
        bf1.as_mut_ptr().add(6),
        &raw const v_bit,
        &raw const rnding,
    );
    bf1[6] = half_btf_0_neon_r(
        cospi.add(24),
        bf1.as_mut_ptr().add(6),
        &raw const v_bit,
        &raw const rnding,
    );

    addsub_neon(
        bf1[8],
        bf1[9],
        bf1.as_mut_ptr().add(8),
        bf1.as_mut_ptr().add(9),
        &raw const clamp_lo,
        &raw const clamp_hi,
    );
    addsub_neon(
        bf1[11],
        bf1[10],
        bf1.as_mut_ptr().add(11),
        bf1.as_mut_ptr().add(10),
        &raw const clamp_lo,
        &raw const clamp_hi,
    );
    addsub_neon(
        bf1[12],
        bf1[13],
        bf1.as_mut_ptr().add(12),
        bf1.as_mut_ptr().add(13),
        &raw const clamp_lo,
        &raw const clamp_hi,
    );
    addsub_neon(
        bf1[15],
        bf1[14],
        bf1.as_mut_ptr().add(15),
        bf1.as_mut_ptr().add(14),
        &raw const clamp_lo,
        &raw const clamp_hi,
    );

    idct32_stage4_neon(bf1.as_mut_ptr(), cospi, &raw const v_bit, &raw const rnding);

    // stage 5
    bf1[0] =
        half_btf_0_neon_r(cospi.add(32), bf1.as_mut_ptr(), &raw const v_bit, &raw const rnding);
    bf1[1] = bf1[0];
    bf1[3] = half_btf_0_neon_r(
        cospi.add(16),
        bf1.as_mut_ptr().add(2),
        &raw const v_bit,
        &raw const rnding,
    );
    bf1[2] = half_btf_0_neon_r(
        cospi.add(48),
        bf1.as_mut_ptr().add(2),
        &raw const v_bit,
        &raw const rnding,
    );

    addsub_neon(
        bf1[4],
        bf1[5],
        bf1.as_mut_ptr().add(4),
        bf1.as_mut_ptr().add(5),
        &raw const clamp_lo,
        &raw const clamp_hi,
    );
    addsub_neon(
        bf1[7],
        bf1[6],
        bf1.as_mut_ptr().add(7),
        bf1.as_mut_ptr().add(6),
        &raw const clamp_lo,
        &raw const clamp_hi,
    );

    idct32_stage5_neon(
        bf1.as_mut_ptr(),
        cospi,
        &raw const clamp_lo,
        &raw const clamp_hi,
        &raw const v_bit,
        &raw const rnding,
    );

    // stage 6
    addsub_neon(
        bf1[0],
        bf1[3],
        bf1.as_mut_ptr(),
        bf1.as_mut_ptr().add(3),
        &raw const clamp_lo,
        &raw const clamp_hi,
    );
    addsub_neon(
        bf1[1],
        bf1[2],
        bf1.as_mut_ptr().add(1),
        bf1.as_mut_ptr().add(2),
        &raw const clamp_lo,
        &raw const clamp_hi,
    );

    idct32_stage6_neon(
        bf1.as_mut_ptr(),
        cospi,
        &raw const clamp_lo,
        &raw const clamp_hi,
        &raw const v_bit,
        &raw const rnding,
    );

    // stage 7
    idct32_stage7_neon(
        bf1.as_mut_ptr(),
        cospi,
        &raw const clamp_lo,
        &raw const clamp_hi,
        &raw const v_bit,
        &raw const rnding,
    );

    // stage 8
    idct32_stage8_neon(
        bf1.as_mut_ptr(),
        cospi,
        &raw const clamp_lo,
        &raw const clamp_hi,
        &raw const v_bit,
        &raw const rnding,
    );
    // stage 9
    idct32_stage9_neon(
        bf1.as_mut_ptr(),
        out,
        do_cols,
        bd,
        out_shift,
        &raw const clamp_lo,
        &raw const clamp_hi,
    );
}

unsafe fn idct32x32_neon(
    mut in_: *mut int32x4_t,
    mut out: *mut int32x4_t,
    mut bit: i32,
    mut do_cols: i32,
    mut bd: i32,
    mut out_shift: i32,
) {
    let mut cospi: *const i32 = cospi_arr(bit);
    let log_range: i32 = get_log_range(bd, do_cols);
    let clamp_lo: int32x4_t = vdupq_n_s32(-(1 << (log_range - 1)));
    let clamp_hi: int32x4_t = vdupq_n_s32((1 << (log_range - 1)) - 1);
    let mut bf1: [int32x4_t; 32] = core::mem::zeroed();
    let mut bf0: [int32x4_t; 32] = core::mem::zeroed();
    let v_bit: int32x4_t = vdupq_n_s32(-bit);
    let rnding: int32x4_t = vdupq_n_s32(1 << (bit - 1));
    // stage 0
    // stage 1
    bf1[0] = *in_;
    bf1[1] = *in_.add(16);
    bf1[2] = *in_.add(8);
    bf1[3] = *in_.add(24);
    bf1[4] = *in_.add(4);
    bf1[5] = *in_.add(20);
    bf1[6] = *in_.add(12);
    bf1[7] = *in_.add(28);
    bf1[8] = *in_.add(2);
    bf1[9] = *in_.add(18);
    bf1[10] = *in_.add(10);
    bf1[11] = *in_.add(26);
    bf1[12] = *in_.add(6);
    bf1[13] = *in_.add(22);
    bf1[14] = *in_.add(14);
    bf1[15] = *in_.add(30);
    bf1[16] = *in_.add(1);
    bf1[17] = *in_.add(17);
    bf1[18] = *in_.add(9);
    bf1[19] = *in_.add(25);
    bf1[20] = *in_.add(5);
    bf1[21] = *in_.add(21);
    bf1[22] = *in_.add(13);
    bf1[23] = *in_.add(29);
    bf1[24] = *in_.add(3);
    bf1[25] = *in_.add(19);
    bf1[26] = *in_.add(11);
    bf1[27] = *in_.add(27);
    bf1[28] = *in_.add(7);
    bf1[29] = *in_.add(23);
    bf1[30] = *in_.add(15);
    bf1[31] = *in_.add(31);

    // stage 2
    for i in 0..16 {
        bf0[i as usize] = bf1[i as usize];
    }

    bf0[16] = half_btf_neon_mode01_r(
        cospi.add(62),
        bf1.as_mut_ptr().add(16),
        cospi.add(2),
        bf1.as_mut_ptr().add(31),
        &raw const v_bit,
        &raw const rnding,
    );
    bf0[17] = half_btf_neon_mode01_r(
        cospi.add(30),
        bf1.as_mut_ptr().add(17),
        cospi.add(34),
        bf1.as_mut_ptr().add(30),
        &raw const v_bit,
        &raw const rnding,
    );
    bf0[18] = half_btf_neon_mode01_r(
        cospi.add(46),
        bf1.as_mut_ptr().add(18),
        cospi.add(18),
        bf1.as_mut_ptr().add(29),
        &raw const v_bit,
        &raw const rnding,
    );
    bf0[19] = half_btf_neon_mode01_r(
        cospi.add(14),
        bf1.as_mut_ptr().add(19),
        cospi.add(50),
        bf1.as_mut_ptr().add(28),
        &raw const v_bit,
        &raw const rnding,
    );
    bf0[20] = half_btf_neon_mode01_r(
        cospi.add(54),
        bf1.as_mut_ptr().add(20),
        cospi.add(10),
        bf1.as_mut_ptr().add(27),
        &raw const v_bit,
        &raw const rnding,
    );
    bf0[21] = half_btf_neon_mode01_r(
        cospi.add(22),
        bf1.as_mut_ptr().add(21),
        cospi.add(42),
        bf1.as_mut_ptr().add(26),
        &raw const v_bit,
        &raw const rnding,
    );
    bf0[22] = half_btf_neon_mode01_r(
        cospi.add(38),
        bf1.as_mut_ptr().add(22),
        cospi.add(26),
        bf1.as_mut_ptr().add(25),
        &raw const v_bit,
        &raw const rnding,
    );
    bf0[23] = half_btf_neon_mode01_r(
        cospi.add(6),
        bf1.as_mut_ptr().add(23),
        cospi.add(58),
        bf1.as_mut_ptr().add(24),
        &raw const v_bit,
        &raw const rnding,
    );
    bf0[24] = half_btf_neon_r(
        cospi.add(58),
        bf1.as_mut_ptr().add(23),
        cospi.add(6),
        bf1.as_mut_ptr().add(24),
        &raw const v_bit,
        &raw const rnding,
    );
    bf0[25] = half_btf_neon_r(
        cospi.add(26),
        bf1.as_mut_ptr().add(22),
        cospi.add(38),
        bf1.as_mut_ptr().add(25),
        &raw const v_bit,
        &raw const rnding,
    );
    bf0[26] = half_btf_neon_r(
        cospi.add(42),
        bf1.as_mut_ptr().add(21),
        cospi.add(22),
        bf1.as_mut_ptr().add(26),
        &raw const v_bit,
        &raw const rnding,
    );
    bf0[27] = half_btf_neon_r(
        cospi.add(10),
        bf1.as_mut_ptr().add(20),
        cospi.add(54),
        bf1.as_mut_ptr().add(27),
        &raw const v_bit,
        &raw const rnding,
    );
    bf0[28] = half_btf_neon_r(
        cospi.add(50),
        bf1.as_mut_ptr().add(19),
        cospi.add(14),
        bf1.as_mut_ptr().add(28),
        &raw const v_bit,
        &raw const rnding,
    );
    bf0[29] = half_btf_neon_r(
        cospi.add(18),
        bf1.as_mut_ptr().add(18),
        cospi.add(46),
        bf1.as_mut_ptr().add(29),
        &raw const v_bit,
        &raw const rnding,
    );
    bf0[30] = half_btf_neon_r(
        cospi.add(34),
        bf1.as_mut_ptr().add(17),
        cospi.add(30),
        bf1.as_mut_ptr().add(30),
        &raw const v_bit,
        &raw const rnding,
    );
    bf0[31] = half_btf_neon_r(
        cospi.add(2),
        bf1.as_mut_ptr().add(16),
        cospi.add(62),
        bf1.as_mut_ptr().add(31),
        &raw const v_bit,
        &raw const rnding,
    );

    // stage 3
    for i in 0..8 {
        bf1[i as usize] = bf0[i as usize];
    }

    bf1[8] = half_btf_neon_mode01_r(
        cospi.add(60),
        bf0.as_mut_ptr().add(8),
        cospi.add(4),
        bf0.as_mut_ptr().add(15),
        &raw const v_bit,
        &raw const rnding,
    );
    bf1[9] = half_btf_neon_mode01_r(
        cospi.add(28),
        bf0.as_mut_ptr().add(9),
        cospi.add(36),
        bf0.as_mut_ptr().add(14),
        &raw const v_bit,
        &raw const rnding,
    );
    bf1[10] = half_btf_neon_mode01_r(
        cospi.add(44),
        bf0.as_mut_ptr().add(10),
        cospi.add(20),
        bf0.as_mut_ptr().add(13),
        &raw const v_bit,
        &raw const rnding,
    );
    bf1[11] = half_btf_neon_mode01_r(
        cospi.add(12),
        bf0.as_mut_ptr().add(11),
        cospi.add(52),
        bf0.as_mut_ptr().add(12),
        &raw const v_bit,
        &raw const rnding,
    );
    bf1[12] = half_btf_neon_r(
        cospi.add(52),
        bf0.as_mut_ptr().add(11),
        cospi.add(12),
        bf0.as_mut_ptr().add(12),
        &raw const v_bit,
        &raw const rnding,
    );
    bf1[13] = half_btf_neon_r(
        cospi.add(20),
        bf0.as_mut_ptr().add(10),
        cospi.add(44),
        bf0.as_mut_ptr().add(13),
        &raw const v_bit,
        &raw const rnding,
    );
    bf1[14] = half_btf_neon_r(
        cospi.add(36),
        bf0.as_mut_ptr().add(9),
        cospi.add(28),
        bf0.as_mut_ptr().add(14),
        &raw const v_bit,
        &raw const rnding,
    );
    bf1[15] = half_btf_neon_r(
        cospi.add(4),
        bf0.as_mut_ptr().add(8),
        cospi.add(60),
        bf0.as_mut_ptr().add(15),
        &raw const v_bit,
        &raw const rnding,
    );

    addsub_neon(
        bf0[16],
        bf0[17],
        bf1.as_mut_ptr().add(16),
        bf1.as_mut_ptr().add(17),
        &raw const clamp_lo,
        &raw const clamp_hi,
    );
    addsub_neon(
        bf0[19],
        bf0[18],
        bf1.as_mut_ptr().add(19),
        bf1.as_mut_ptr().add(18),
        &raw const clamp_lo,
        &raw const clamp_hi,
    );
    addsub_neon(
        bf0[20],
        bf0[21],
        bf1.as_mut_ptr().add(20),
        bf1.as_mut_ptr().add(21),
        &raw const clamp_lo,
        &raw const clamp_hi,
    );
    addsub_neon(
        bf0[23],
        bf0[22],
        bf1.as_mut_ptr().add(23),
        bf1.as_mut_ptr().add(22),
        &raw const clamp_lo,
        &raw const clamp_hi,
    );
    addsub_neon(
        bf0[24],
        bf0[25],
        bf1.as_mut_ptr().add(24),
        bf1.as_mut_ptr().add(25),
        &raw const clamp_lo,
        &raw const clamp_hi,
    );
    addsub_neon(
        bf0[27],
        bf0[26],
        bf1.as_mut_ptr().add(27),
        bf1.as_mut_ptr().add(26),
        &raw const clamp_lo,
        &raw const clamp_hi,
    );
    addsub_neon(
        bf0[28],
        bf0[29],
        bf1.as_mut_ptr().add(28),
        bf1.as_mut_ptr().add(29),
        &raw const clamp_lo,
        &raw const clamp_hi,
    );
    addsub_neon(
        bf0[31],
        bf0[30],
        bf1.as_mut_ptr().add(31),
        bf1.as_mut_ptr().add(30),
        &raw const clamp_lo,
        &raw const clamp_hi,
    );

    // stage 4
    bf0[0] = bf1[0];
    bf0[1] = bf1[1];
    bf0[2] = bf1[2];
    bf0[3] = bf1[3];
    bf0[4] = half_btf_neon_mode01_r(
        cospi.add(56),
        bf1.as_mut_ptr().add(4),
        cospi.add(8),
        bf1.as_mut_ptr().add(7),
        &raw const v_bit,
        &raw const rnding,
    );
    bf0[5] = half_btf_neon_mode01_r(
        cospi.add(24),
        bf1.as_mut_ptr().add(5),
        cospi.add(40),
        bf1.as_mut_ptr().add(6),
        &raw const v_bit,
        &raw const rnding,
    );
    bf0[6] = half_btf_neon_r(
        cospi.add(40),
        bf1.as_mut_ptr().add(5),
        cospi.add(24),
        bf1.as_mut_ptr().add(6),
        &raw const v_bit,
        &raw const rnding,
    );
    bf0[7] = half_btf_neon_r(
        cospi.add(8),
        bf1.as_mut_ptr().add(4),
        cospi.add(56),
        bf1.as_mut_ptr().add(7),
        &raw const v_bit,
        &raw const rnding,
    );

    addsub_neon(
        bf1[8],
        bf1[9],
        bf0.as_mut_ptr().add(8),
        bf0.as_mut_ptr().add(9),
        &raw const clamp_lo,
        &raw const clamp_hi,
    );
    addsub_neon(
        bf1[11],
        bf1[10],
        bf0.as_mut_ptr().add(11),
        bf0.as_mut_ptr().add(10),
        &raw const clamp_lo,
        &raw const clamp_hi,
    );
    addsub_neon(
        bf1[12],
        bf1[13],
        bf0.as_mut_ptr().add(12),
        bf0.as_mut_ptr().add(13),
        &raw const clamp_lo,
        &raw const clamp_hi,
    );
    addsub_neon(
        bf1[15],
        bf1[14],
        bf0.as_mut_ptr().add(15),
        bf0.as_mut_ptr().add(14),
        &raw const clamp_lo,
        &raw const clamp_hi,
    );

    bf0[16] = bf1[16];
    bf0[17] = half_btf_neon_mode10_r(
        cospi.add(8),
        bf1.as_mut_ptr().add(17),
        cospi.add(56),
        bf1.as_mut_ptr().add(30),
        &raw const v_bit,
        &raw const rnding,
    );
    bf0[18] = half_btf_neon_mode11_r(
        cospi.add(56),
        bf1.as_mut_ptr().add(18),
        cospi.add(8),
        bf1.as_mut_ptr().add(29),
        &raw const v_bit,
        &raw const rnding,
    );
    bf0[19] = bf1[19];
    bf0[20] = bf1[20];
    bf0[21] = half_btf_neon_mode10_r(
        cospi.add(40),
        bf1.as_mut_ptr().add(21),
        cospi.add(24),
        bf1.as_mut_ptr().add(26),
        &raw const v_bit,
        &raw const rnding,
    );
    bf0[22] = half_btf_neon_mode11_r(
        cospi.add(24),
        bf1.as_mut_ptr().add(22),
        cospi.add(40),
        bf1.as_mut_ptr().add(25),
        &raw const v_bit,
        &raw const rnding,
    );
    bf0[23] = bf1[23];
    bf0[24] = bf1[24];
    bf0[25] = half_btf_neon_mode10_r(
        cospi.add(40),
        bf1.as_mut_ptr().add(22),
        cospi.add(24),
        bf1.as_mut_ptr().add(25),
        &raw const v_bit,
        &raw const rnding,
    );
    bf0[26] = half_btf_neon_r(
        cospi.add(24),
        bf1.as_mut_ptr().add(21),
        cospi.add(40),
        bf1.as_mut_ptr().add(26),
        &raw const v_bit,
        &raw const rnding,
    );
    bf0[27] = bf1[27];
    bf0[28] = bf1[28];
    bf0[29] = half_btf_neon_mode10_r(
        cospi.add(8),
        bf1.as_mut_ptr().add(18),
        cospi.add(56),
        bf1.as_mut_ptr().add(29),
        &raw const v_bit,
        &raw const rnding,
    );
    bf0[30] = half_btf_neon_r(
        cospi.add(56),
        bf1.as_mut_ptr().add(17),
        cospi.add(8),
        bf1.as_mut_ptr().add(30),
        &raw const v_bit,
        &raw const rnding,
    );
    bf0[31] = bf1[31];

    // stage 5
    bf1[0] = half_btf_neon_r(
        cospi.add(32),
        bf0.as_mut_ptr(),
        cospi.add(32),
        bf0.as_mut_ptr().add(1),
        &raw const v_bit,
        &raw const rnding,
    );
    bf1[1] = half_btf_neon_mode01_r(
        cospi.add(32),
        bf0.as_mut_ptr(),
        cospi.add(32),
        bf0.as_mut_ptr().add(1),
        &raw const v_bit,
        &raw const rnding,
    );
    bf1[2] = half_btf_neon_mode01_r(
        cospi.add(48),
        bf0.as_mut_ptr().add(2),
        cospi.add(16),
        bf0.as_mut_ptr().add(3),
        &raw const v_bit,
        &raw const rnding,
    );
    bf1[3] = half_btf_neon_r(
        cospi.add(16),
        bf0.as_mut_ptr().add(2),
        cospi.add(48),
        bf0.as_mut_ptr().add(3),
        &raw const v_bit,
        &raw const rnding,
    );
    addsub_neon(
        bf0[4],
        bf0[5],
        bf1.as_mut_ptr().add(4),
        bf1.as_mut_ptr().add(5),
        &raw const clamp_lo,
        &raw const clamp_hi,
    );
    addsub_neon(
        bf0[7],
        bf0[6],
        bf1.as_mut_ptr().add(7),
        bf1.as_mut_ptr().add(6),
        &raw const clamp_lo,
        &raw const clamp_hi,
    );
    bf1[8] = bf0[8];
    bf1[9] = half_btf_neon_mode10_r(
        cospi.add(16),
        bf0.as_mut_ptr().add(9),
        cospi.add(48),
        bf0.as_mut_ptr().add(14),
        &raw const v_bit,
        &raw const rnding,
    );
    bf1[10] = half_btf_neon_mode11_r(
        cospi.add(48),
        bf0.as_mut_ptr().add(10),
        cospi.add(16),
        bf0.as_mut_ptr().add(13),
        &raw const v_bit,
        &raw const rnding,
    );
    bf1[11] = bf0[11];
    bf1[12] = bf0[12];
    bf1[13] = half_btf_neon_mode10_r(
        cospi.add(16),
        bf0.as_mut_ptr().add(10),
        cospi.add(48),
        bf0.as_mut_ptr().add(13),
        &raw const v_bit,
        &raw const rnding,
    );
    bf1[14] = half_btf_neon_r(
        cospi.add(48),
        bf0.as_mut_ptr().add(9),
        cospi.add(16),
        bf0.as_mut_ptr().add(14),
        &raw const v_bit,
        &raw const rnding,
    );
    bf1[15] = bf0[15];
    addsub_neon(
        bf0[16],
        bf0[19],
        bf1.as_mut_ptr().add(16),
        bf1.as_mut_ptr().add(19),
        &raw const clamp_lo,
        &raw const clamp_hi,
    );
    addsub_neon(
        bf0[17],
        bf0[18],
        bf1.as_mut_ptr().add(17),
        bf1.as_mut_ptr().add(18),
        &raw const clamp_lo,
        &raw const clamp_hi,
    );
    addsub_neon(
        bf0[23],
        bf0[20],
        bf1.as_mut_ptr().add(23),
        bf1.as_mut_ptr().add(20),
        &raw const clamp_lo,
        &raw const clamp_hi,
    );
    addsub_neon(
        bf0[22],
        bf0[21],
        bf1.as_mut_ptr().add(22),
        bf1.as_mut_ptr().add(21),
        &raw const clamp_lo,
        &raw const clamp_hi,
    );
    addsub_neon(
        bf0[24],
        bf0[27],
        bf1.as_mut_ptr().add(24),
        bf1.as_mut_ptr().add(27),
        &raw const clamp_lo,
        &raw const clamp_hi,
    );
    addsub_neon(
        bf0[25],
        bf0[26],
        bf1.as_mut_ptr().add(25),
        bf1.as_mut_ptr().add(26),
        &raw const clamp_lo,
        &raw const clamp_hi,
    );
    addsub_neon(
        bf0[31],
        bf0[28],
        bf1.as_mut_ptr().add(31),
        bf1.as_mut_ptr().add(28),
        &raw const clamp_lo,
        &raw const clamp_hi,
    );
    addsub_neon(
        bf0[30],
        bf0[29],
        bf1.as_mut_ptr().add(30),
        bf1.as_mut_ptr().add(29),
        &raw const clamp_lo,
        &raw const clamp_hi,
    );

    // stage 6
    addsub_neon(
        bf1[0],
        bf1[3],
        bf0.as_mut_ptr(),
        bf0.as_mut_ptr().add(3),
        &raw const clamp_lo,
        &raw const clamp_hi,
    );
    addsub_neon(
        bf1[1],
        bf1[2],
        bf0.as_mut_ptr().add(1),
        bf0.as_mut_ptr().add(2),
        &raw const clamp_lo,
        &raw const clamp_hi,
    );
    bf0[4] = bf1[4];
    bf0[5] = half_btf_neon_mode10_r(
        cospi.add(32),
        bf1.as_mut_ptr().add(5),
        cospi.add(32),
        bf1.as_mut_ptr().add(6),
        &raw const v_bit,
        &raw const rnding,
    );
    bf0[6] = half_btf_neon_r(
        cospi.add(32),
        bf1.as_mut_ptr().add(5),
        cospi.add(32),
        bf1.as_mut_ptr().add(6),
        &raw const v_bit,
        &raw const rnding,
    );
    bf0[7] = bf1[7];
    addsub_neon(
        bf1[8],
        bf1[11],
        bf0.as_mut_ptr().add(8),
        bf0.as_mut_ptr().add(11),
        &raw const clamp_lo,
        &raw const clamp_hi,
    );
    addsub_neon(
        bf1[9],
        bf1[10],
        bf0.as_mut_ptr().add(9),
        bf0.as_mut_ptr().add(10),
        &raw const clamp_lo,
        &raw const clamp_hi,
    );
    addsub_neon(
        bf1[15],
        bf1[12],
        bf0.as_mut_ptr().add(15),
        bf0.as_mut_ptr().add(12),
        &raw const clamp_lo,
        &raw const clamp_hi,
    );
    addsub_neon(
        bf1[14],
        bf1[13],
        bf0.as_mut_ptr().add(14),
        bf0.as_mut_ptr().add(13),
        &raw const clamp_lo,
        &raw const clamp_hi,
    );
    bf0[16] = bf1[16];
    bf0[17] = bf1[17];
    bf0[18] = half_btf_neon_mode10_r(
        cospi.add(16),
        bf1.as_mut_ptr().add(18),
        cospi.add(48),
        bf1.as_mut_ptr().add(29),
        &raw const v_bit,
        &raw const rnding,
    );
    bf0[19] = half_btf_neon_mode10_r(
        cospi.add(16),
        bf1.as_mut_ptr().add(19),
        cospi.add(48),
        bf1.as_mut_ptr().add(28),
        &raw const v_bit,
        &raw const rnding,
    );
    bf0[20] = half_btf_neon_mode11_r(
        cospi.add(48),
        bf1.as_mut_ptr().add(20),
        cospi.add(16),
        bf1.as_mut_ptr().add(27),
        &raw const v_bit,
        &raw const rnding,
    );
    bf0[21] = half_btf_neon_mode11_r(
        cospi.add(48),
        bf1.as_mut_ptr().add(21),
        cospi.add(16),
        bf1.as_mut_ptr().add(26),
        &raw const v_bit,
        &raw const rnding,
    );
    bf0[22] = bf1[22];
    bf0[23] = bf1[23];
    bf0[24] = bf1[24];
    bf0[25] = bf1[25];
    bf0[26] = half_btf_neon_mode10_r(
        cospi.add(16),
        bf1.as_mut_ptr().add(21),
        cospi.add(48),
        bf1.as_mut_ptr().add(26),
        &raw const v_bit,
        &raw const rnding,
    );
    bf0[27] = half_btf_neon_mode10_r(
        cospi.add(16),
        bf1.as_mut_ptr().add(20),
        cospi.add(48),
        bf1.as_mut_ptr().add(27),
        &raw const v_bit,
        &raw const rnding,
    );
    bf0[28] = half_btf_neon_r(
        cospi.add(48),
        bf1.as_mut_ptr().add(19),
        cospi.add(16),
        bf1.as_mut_ptr().add(28),
        &raw const v_bit,
        &raw const rnding,
    );
    bf0[29] = half_btf_neon_r(
        cospi.add(48),
        bf1.as_mut_ptr().add(18),
        cospi.add(16),
        bf1.as_mut_ptr().add(29),
        &raw const v_bit,
        &raw const rnding,
    );
    bf0[30] = bf1[30];
    bf0[31] = bf1[31];

    // stage 7
    addsub_neon(
        bf0[0],
        bf0[7],
        bf1.as_mut_ptr(),
        bf1.as_mut_ptr().add(7),
        &raw const clamp_lo,
        &raw const clamp_hi,
    );
    addsub_neon(
        bf0[1],
        bf0[6],
        bf1.as_mut_ptr().add(1),
        bf1.as_mut_ptr().add(6),
        &raw const clamp_lo,
        &raw const clamp_hi,
    );
    addsub_neon(
        bf0[2],
        bf0[5],
        bf1.as_mut_ptr().add(2),
        bf1.as_mut_ptr().add(5),
        &raw const clamp_lo,
        &raw const clamp_hi,
    );
    addsub_neon(
        bf0[3],
        bf0[4],
        bf1.as_mut_ptr().add(3),
        bf1.as_mut_ptr().add(4),
        &raw const clamp_lo,
        &raw const clamp_hi,
    );
    bf1[8] = bf0[8];
    bf1[9] = bf0[9];
    bf1[10] = half_btf_neon_mode10_r(
        cospi.add(32),
        bf0.as_mut_ptr().add(10),
        cospi.add(32),
        bf0.as_mut_ptr().add(13),
        &raw const v_bit,
        &raw const rnding,
    );
    bf1[11] = half_btf_neon_mode10_r(
        cospi.add(32),
        bf0.as_mut_ptr().add(11),
        cospi.add(32),
        bf0.as_mut_ptr().add(12),
        &raw const v_bit,
        &raw const rnding,
    );
    bf1[12] = half_btf_neon_r(
        cospi.add(32),
        bf0.as_mut_ptr().add(11),
        cospi.add(32),
        bf0.as_mut_ptr().add(12),
        &raw const v_bit,
        &raw const rnding,
    );
    bf1[13] = half_btf_neon_r(
        cospi.add(32),
        bf0.as_mut_ptr().add(10),
        cospi.add(32),
        bf0.as_mut_ptr().add(13),
        &raw const v_bit,
        &raw const rnding,
    );
    bf1[14] = bf0[14];
    bf1[15] = bf0[15];
    addsub_neon(
        bf0[16],
        bf0[23],
        bf1.as_mut_ptr().add(16),
        bf1.as_mut_ptr().add(23),
        &raw const clamp_lo,
        &raw const clamp_hi,
    );
    addsub_neon(
        bf0[17],
        bf0[22],
        bf1.as_mut_ptr().add(17),
        bf1.as_mut_ptr().add(22),
        &raw const clamp_lo,
        &raw const clamp_hi,
    );
    addsub_neon(
        bf0[18],
        bf0[21],
        bf1.as_mut_ptr().add(18),
        bf1.as_mut_ptr().add(21),
        &raw const clamp_lo,
        &raw const clamp_hi,
    );
    addsub_neon(
        bf0[19],
        bf0[20],
        bf1.as_mut_ptr().add(19),
        bf1.as_mut_ptr().add(20),
        &raw const clamp_lo,
        &raw const clamp_hi,
    );
    addsub_neon(
        bf0[31],
        bf0[24],
        bf1.as_mut_ptr().add(31),
        bf1.as_mut_ptr().add(24),
        &raw const clamp_lo,
        &raw const clamp_hi,
    );
    addsub_neon(
        bf0[30],
        bf0[25],
        bf1.as_mut_ptr().add(30),
        bf1.as_mut_ptr().add(25),
        &raw const clamp_lo,
        &raw const clamp_hi,
    );
    addsub_neon(
        bf0[29],
        bf0[26],
        bf1.as_mut_ptr().add(29),
        bf1.as_mut_ptr().add(26),
        &raw const clamp_lo,
        &raw const clamp_hi,
    );
    addsub_neon(
        bf0[28],
        bf0[27],
        bf1.as_mut_ptr().add(28),
        bf1.as_mut_ptr().add(27),
        &raw const clamp_lo,
        &raw const clamp_hi,
    );

    // stage 8
    addsub_neon(
        bf1[0],
        bf1[15],
        bf0.as_mut_ptr(),
        bf0.as_mut_ptr().add(15),
        &raw const clamp_lo,
        &raw const clamp_hi,
    );
    addsub_neon(
        bf1[1],
        bf1[14],
        bf0.as_mut_ptr().add(1),
        bf0.as_mut_ptr().add(14),
        &raw const clamp_lo,
        &raw const clamp_hi,
    );
    addsub_neon(
        bf1[2],
        bf1[13],
        bf0.as_mut_ptr().add(2),
        bf0.as_mut_ptr().add(13),
        &raw const clamp_lo,
        &raw const clamp_hi,
    );
    addsub_neon(
        bf1[3],
        bf1[12],
        bf0.as_mut_ptr().add(3),
        bf0.as_mut_ptr().add(12),
        &raw const clamp_lo,
        &raw const clamp_hi,
    );
    addsub_neon(
        bf1[4],
        bf1[11],
        bf0.as_mut_ptr().add(4),
        bf0.as_mut_ptr().add(11),
        &raw const clamp_lo,
        &raw const clamp_hi,
    );
    addsub_neon(
        bf1[5],
        bf1[10],
        bf0.as_mut_ptr().add(5),
        bf0.as_mut_ptr().add(10),
        &raw const clamp_lo,
        &raw const clamp_hi,
    );
    addsub_neon(
        bf1[6],
        bf1[9],
        bf0.as_mut_ptr().add(6),
        bf0.as_mut_ptr().add(9),
        &raw const clamp_lo,
        &raw const clamp_hi,
    );
    addsub_neon(
        bf1[7],
        bf1[8],
        bf0.as_mut_ptr().add(7),
        bf0.as_mut_ptr().add(8),
        &raw const clamp_lo,
        &raw const clamp_hi,
    );
    bf0[16] = bf1[16];
    bf0[17] = bf1[17];
    bf0[18] = bf1[18];
    bf0[19] = bf1[19];
    bf0[20] = half_btf_neon_mode10_r(
        cospi.add(32),
        bf1.as_mut_ptr().add(20),
        cospi.add(32),
        bf1.as_mut_ptr().add(27),
        &raw const v_bit,
        &raw const rnding,
    );
    bf0[21] = half_btf_neon_mode10_r(
        cospi.add(32),
        bf1.as_mut_ptr().add(21),
        cospi.add(32),
        bf1.as_mut_ptr().add(26),
        &raw const v_bit,
        &raw const rnding,
    );
    bf0[22] = half_btf_neon_mode10_r(
        cospi.add(32),
        bf1.as_mut_ptr().add(22),
        cospi.add(32),
        bf1.as_mut_ptr().add(25),
        &raw const v_bit,
        &raw const rnding,
    );
    bf0[23] = half_btf_neon_mode10_r(
        cospi.add(32),
        bf1.as_mut_ptr().add(23),
        cospi.add(32),
        bf1.as_mut_ptr().add(24),
        &raw const v_bit,
        &raw const rnding,
    );
    bf0[24] = half_btf_neon_r(
        cospi.add(32),
        bf1.as_mut_ptr().add(23),
        cospi.add(32),
        bf1.as_mut_ptr().add(24),
        &raw const v_bit,
        &raw const rnding,
    );
    bf0[25] = half_btf_neon_r(
        cospi.add(32),
        bf1.as_mut_ptr().add(22),
        cospi.add(32),
        bf1.as_mut_ptr().add(25),
        &raw const v_bit,
        &raw const rnding,
    );
    bf0[26] = half_btf_neon_r(
        cospi.add(32),
        bf1.as_mut_ptr().add(21),
        cospi.add(32),
        bf1.as_mut_ptr().add(26),
        &raw const v_bit,
        &raw const rnding,
    );
    bf0[27] = half_btf_neon_r(
        cospi.add(32),
        bf1.as_mut_ptr().add(20),
        cospi.add(32),
        bf1.as_mut_ptr().add(27),
        &raw const v_bit,
        &raw const rnding,
    );
    bf0[28] = bf1[28];
    bf0[29] = bf1[29];
    bf0[30] = bf1[30];
    bf0[31] = bf1[31];

    // stage 9
    addsub_neon(bf0[0], bf0[31], out, out.add(31), &raw const clamp_lo, &raw const clamp_hi);
    addsub_neon(bf0[1], bf0[30], out.add(1), out.add(30), &raw const clamp_lo, &raw const clamp_hi);
    addsub_neon(bf0[2], bf0[29], out.add(2), out.add(29), &raw const clamp_lo, &raw const clamp_hi);
    addsub_neon(bf0[3], bf0[28], out.add(3), out.add(28), &raw const clamp_lo, &raw const clamp_hi);
    addsub_neon(bf0[4], bf0[27], out.add(4), out.add(27), &raw const clamp_lo, &raw const clamp_hi);
    addsub_neon(bf0[5], bf0[26], out.add(5), out.add(26), &raw const clamp_lo, &raw const clamp_hi);
    addsub_neon(bf0[6], bf0[25], out.add(6), out.add(25), &raw const clamp_lo, &raw const clamp_hi);
    addsub_neon(bf0[7], bf0[24], out.add(7), out.add(24), &raw const clamp_lo, &raw const clamp_hi);
    addsub_neon(bf0[8], bf0[23], out.add(8), out.add(23), &raw const clamp_lo, &raw const clamp_hi);
    addsub_neon(bf0[9], bf0[22], out.add(9), out.add(22), &raw const clamp_lo, &raw const clamp_hi);
    addsub_neon(
        bf0[10],
        bf0[21],
        out.add(10),
        out.add(21),
        &raw const clamp_lo,
        &raw const clamp_hi,
    );
    addsub_neon(
        bf0[11],
        bf0[20],
        out.add(11),
        out.add(20),
        &raw const clamp_lo,
        &raw const clamp_hi,
    );
    addsub_neon(
        bf0[12],
        bf0[19],
        out.add(12),
        out.add(19),
        &raw const clamp_lo,
        &raw const clamp_hi,
    );
    addsub_neon(
        bf0[13],
        bf0[18],
        out.add(13),
        out.add(18),
        &raw const clamp_lo,
        &raw const clamp_hi,
    );
    addsub_neon(
        bf0[14],
        bf0[17],
        out.add(14),
        out.add(17),
        &raw const clamp_lo,
        &raw const clamp_hi,
    );
    addsub_neon(
        bf0[15],
        bf0[16],
        out.add(15),
        out.add(16),
        &raw const clamp_lo,
        &raw const clamp_hi,
    );

    if !(do_cols != 0) {
        let log_range_out: i32 = get_log_range_out(bd);
        let clamp_lo_out: int32x4_t = vdupq_n_s32(-(1 << (log_range_out - 1)));
        let clamp_hi_out: int32x4_t = vdupq_n_s32((1 << (log_range_out - 1)) - 1);
        round_shift_8x8(out, out_shift);
        round_shift_8x8(out.add(16), out_shift);
        highbd_clamp_s32_neon(out, out, &raw const clamp_lo_out, &raw const clamp_hi_out, 32);
    }
}

unsafe fn iidentity32_neon(
    mut in_: *mut int32x4_t,
    mut out: *mut int32x4_t,
    mut bit: i32,
    mut do_cols: i32,
    mut bd: i32,
    mut out_shift: i32,
) {
    let _ = bit;
    for i in (0..32).step_by(16) {
        *out.offset(i as isize) = vshlq_n_s32::<2>(*in_.offset(i as isize));
        *out.offset((i + 1) as isize) = vshlq_n_s32::<2>(*in_.offset((i + 1) as isize));
        *out.offset((i + 2) as isize) = vshlq_n_s32::<2>(*in_.offset((i + 2) as isize));
        *out.offset((i + 3) as isize) = vshlq_n_s32::<2>(*in_.offset((i + 3) as isize));
        *out.offset((i + 4) as isize) = vshlq_n_s32::<2>(*in_.offset((i + 4) as isize));
        *out.offset((i + 5) as isize) = vshlq_n_s32::<2>(*in_.offset((i + 5) as isize));
        *out.offset((i + 6) as isize) = vshlq_n_s32::<2>(*in_.offset((i + 6) as isize));
        *out.offset((i + 7) as isize) = vshlq_n_s32::<2>(*in_.offset((i + 7) as isize));
        *out.offset((i + 8) as isize) = vshlq_n_s32::<2>(*in_.offset((i + 8) as isize));
        *out.offset((i + 9) as isize) = vshlq_n_s32::<2>(*in_.offset((i + 9) as isize));
        *out.offset((i + 10) as isize) = vshlq_n_s32::<2>(*in_.offset((i + 10) as isize));
        *out.offset((i + 11) as isize) = vshlq_n_s32::<2>(*in_.offset((i + 11) as isize));
        *out.offset((i + 12) as isize) = vshlq_n_s32::<2>(*in_.offset((i + 12) as isize));
        *out.offset((i + 13) as isize) = vshlq_n_s32::<2>(*in_.offset((i + 13) as isize));
        *out.offset((i + 14) as isize) = vshlq_n_s32::<2>(*in_.offset((i + 14) as isize));
        *out.offset((i + 15) as isize) = vshlq_n_s32::<2>(*in_.offset((i + 15) as isize));
    }

    if !(do_cols != 0) {
        let log_range_out: i32 = get_log_range_out(bd);
        let clamp_lo_out: int32x4_t = vdupq_n_s32(-(1 << (log_range_out - 1)));
        let clamp_hi_out: int32x4_t = vdupq_n_s32((1 << (log_range_out - 1)) - 1);
        round_shift_8x8(out, out_shift);
        round_shift_8x8(out.add(16), out_shift);
        highbd_clamp_s32_neon(out, out, &raw const clamp_lo_out, &raw const clamp_hi_out, 32);
    }
}

// 1D itx types
const IDCT_1D: i32 = 0;
const IADST_1D: i32 = 1;
const IFLIPADST_1D: i32 = 1;
const IIDENTITY_1D: i32 = 2;
const ITX_TYPES_1D: i32 = 3;
// `typedef enum ATTRIBUTE_PACKED {...} ITX_TYPE_1D` is one byte wide.
type ITX_TYPE_1D = u8;

static vitx_1d_tab: [ITX_TYPE_1D; 16] = [
    IDCT_1D as ITX_TYPE_1D,
    IADST_1D as ITX_TYPE_1D,
    IDCT_1D as ITX_TYPE_1D,
    IADST_1D as ITX_TYPE_1D,
    IFLIPADST_1D as ITX_TYPE_1D,
    IDCT_1D as ITX_TYPE_1D,
    IFLIPADST_1D as ITX_TYPE_1D,
    IADST_1D as ITX_TYPE_1D,
    IFLIPADST_1D as ITX_TYPE_1D,
    IIDENTITY_1D as ITX_TYPE_1D,
    IDCT_1D as ITX_TYPE_1D,
    IIDENTITY_1D as ITX_TYPE_1D,
    IADST_1D as ITX_TYPE_1D,
    IIDENTITY_1D as ITX_TYPE_1D,
    IFLIPADST_1D as ITX_TYPE_1D,
    IIDENTITY_1D as ITX_TYPE_1D,
];
static hitx_1d_tab: [ITX_TYPE_1D; 16] = [
    IDCT_1D as ITX_TYPE_1D,
    IDCT_1D as ITX_TYPE_1D,
    IADST_1D as ITX_TYPE_1D,
    IADST_1D as ITX_TYPE_1D,
    IDCT_1D as ITX_TYPE_1D,
    IFLIPADST_1D as ITX_TYPE_1D,
    IFLIPADST_1D as ITX_TYPE_1D,
    IFLIPADST_1D as ITX_TYPE_1D,
    IADST_1D as ITX_TYPE_1D,
    IIDENTITY_1D as ITX_TYPE_1D,
    IIDENTITY_1D as ITX_TYPE_1D,
    IDCT_1D as ITX_TYPE_1D,
    IIDENTITY_1D as ITX_TYPE_1D,
    IADST_1D as ITX_TYPE_1D,
    IIDENTITY_1D as ITX_TYPE_1D,
    IFLIPADST_1D as ITX_TYPE_1D,
];

static highbd_txfm_all_1d_zeros_w8_arr: [[[transform_1d_neon; 4]; 3]; 5] = [
    [
        [Some(idct4x4_neon), None, None, None],
        [Some(iadst4x4_neon), None, None, None],
        [Some(iidentity4_neon), Some(iidentity4_neon), Some(iidentity4_neon), None],
    ],
    [
        [Some(idct8x8_low1_neon), Some(idct8x8_new_neon), None, None],
        [Some(iadst8x8_low1_neon), Some(iadst8x8_new_neon), None, None],
        [Some(iidentity8_neon), Some(iidentity8_neon), None, None],
    ],
    [
        [Some(idct16x16_low1_neon), Some(idct16x16_low8_neon), Some(idct16x16_neon), None],
        [Some(iadst16x16_low1_neon), Some(iadst16x16_low8_neon), Some(iadst16x16_neon), None],
        [Some(iidentity16_neon), None, Some(iidentity16_neon), None],
    ],
    [
        [
            Some(idct32x32_low1_neon),
            Some(idct32x32_low8_neon),
            Some(idct32x32_low16_neon),
            Some(idct32x32_neon),
        ],
        [None, None, None, None],
        [Some(iidentity32_neon), None, None, None],
    ],
    [
        [
            Some(idct64x64_low1_neon),
            Some(idct64x64_low8_neon),
            Some(idct64x64_low16_neon),
            Some(idct64x64_neon),
        ],
        [None, None, None, None],
        [None, None, None, None],
    ],
];

#[no_mangle]
pub unsafe extern "C-unwind" fn av1_inv_txfm2d_add_4x8_neon(
    mut input: *const tran_low_t,
    mut output: *mut u16,
    mut stride: i32,
    mut tx_type: TX_TYPE,
    mut bd: i32,
) {
    let mut tx_size: TX_SIZE = TX_4X8 as TX_SIZE;
    let mut buf1: [int32x4_t; 32] = [vdupq_n_s32(0); 32];

    let mut shift: *const i8 = av1_inv_txfm_shift_ls[tx_size as usize];
    let txw_idx: i32 = get_txw_idx(tx_size);
    let txh_idx: i32 = get_txh_idx(tx_size);
    let txfm_size_col: i32 = tx_size_wide[tx_size as usize];
    let txfm_size_row: i32 = tx_size_high[tx_size as usize];
    let row_txfm: transform_1d_neon = highbd_txfm_all_1d_zeros_w8_arr[txw_idx as usize]
        [hitx_1d_tab[tx_type as usize] as usize][0];
    let col_txfm: transform_1d_neon = highbd_txfm_all_1d_zeros_w8_arr[txh_idx as usize]
        [vitx_1d_tab[tx_type as usize] as usize][1];
    let input_stride: i32 = AOMMIN(32, txfm_size_row);

    debug_assert!(col_txfm.is_some());
    debug_assert!(row_txfm.is_some());
    let mut ud_flip: i32 = 0;
    let mut lr_flip: i32 = 0;
    get_flip_cfg(tx_type, &raw mut ud_flip, &raw mut lr_flip);

    // 1st stage: column transform
    let mut buf0: [int32x4_t; 8] = core::mem::zeroed();
    load_buffer_32bit_input(input, input_stride, buf0.as_mut_ptr(), txfm_size_col);
    load_buffer_32bit_input(input.add(4), input_stride, buf0.as_mut_ptr().add(4), txfm_size_col);
    round_shift_rect_array_32_neon(buf0.as_mut_ptr(), buf0.as_mut_ptr(), txfm_size_row);
    row_txfm.unwrap()(buf0.as_mut_ptr(), buf0.as_mut_ptr(), INV_COS_BIT, 0, bd, -(*shift as i32));
    row_txfm.unwrap()(
        buf0.as_mut_ptr().add(4),
        buf0.as_mut_ptr().add(4),
        INV_COS_BIT,
        0,
        bd,
        -(*shift as i32),
    );

    if lr_flip != 0 {
        TRANSPOSE_4X4!(buf0[3], buf0[2], buf0[1], buf0[0], buf1[0], buf1[1], buf1[2], buf1[3]);

        TRANSPOSE_4X4!(buf0[7], buf0[6], buf0[5], buf0[4], buf1[4], buf1[5], buf1[6], buf1[7]);
    } else {
        TRANSPOSE_4X4!(buf0[0], buf0[1], buf0[2], buf0[3], buf1[0], buf1[1], buf1[2], buf1[3]);

        TRANSPOSE_4X4!(buf0[4], buf0[5], buf0[6], buf0[7], buf1[4], buf1[5], buf1[6], buf1[7]);
    }

    // 2nd stage: column transform
    col_txfm.unwrap()(buf1.as_mut_ptr(), buf1.as_mut_ptr(), INV_COS_BIT, 1, bd, 0);

    round_shift_array_32_neon(
        buf1.as_mut_ptr(),
        buf1.as_mut_ptr(),
        txfm_size_row,
        -(*shift.add(1) as i32),
    );

    // write to buffer
    highbd_write_buffer_4xn_neon(buf1.as_mut_ptr(), output, stride, ud_flip, txfm_size_row, bd);
}

#[no_mangle]
pub unsafe extern "C-unwind" fn av1_inv_txfm2d_add_8x4_neon(
    mut input: *const i32,
    mut output: *mut u16,
    mut stride: i32,
    mut tx_type: TX_TYPE,
    mut bd: i32,
) {
    let mut tx_size: TX_SIZE = TX_8X4 as TX_SIZE;
    let mut buf1: [int32x4_t; 8] = core::mem::zeroed();
    let mut shift: *const i8 = av1_inv_txfm_shift_ls[tx_size as usize];
    let txw_idx: i32 = get_txw_idx(tx_size);
    let txh_idx: i32 = get_txh_idx(tx_size);
    let txfm_size_col: i32 = tx_size_wide[tx_size as usize];
    let txfm_size_row: i32 = tx_size_high[tx_size as usize];
    let row_txfm: transform_1d_neon = highbd_txfm_all_1d_zeros_w8_arr[txw_idx as usize]
        [hitx_1d_tab[tx_type as usize] as usize][1];
    let col_txfm: transform_1d_neon = highbd_txfm_all_1d_zeros_w8_arr[txh_idx as usize]
        [vitx_1d_tab[tx_type as usize] as usize][0];

    debug_assert!(col_txfm.is_some());
    debug_assert!(row_txfm.is_some());
    let mut ud_flip: i32 = 0;
    let mut lr_flip: i32 = 0;
    get_flip_cfg(tx_type, &raw mut ud_flip, &raw mut lr_flip);

    // 1st stage: column transform
    let mut buf0: [int32x4_t; 8] = core::mem::zeroed();
    let mut input_row: *const i32 = input;
    load_buffer_32bit_input(input_row, 4, buf0.as_mut_ptr(), txfm_size_col);

    round_shift_rect_array_32_neon(buf0.as_mut_ptr(), buf0.as_mut_ptr(), txfm_size_col);
    row_txfm.unwrap()(buf0.as_mut_ptr(), buf0.as_mut_ptr(), INV_COS_BIT, 0, bd, -(*shift as i32));

    let mut buf1_ptr: *mut int32x4_t = core::ptr::null_mut();
    if lr_flip != 0 {
        flip_buf_neon(buf0.as_mut_ptr(), buf1.as_mut_ptr(), txfm_size_col);
        buf1_ptr = buf1.as_mut_ptr();
    } else {
        buf1_ptr = buf0.as_mut_ptr();
    }

    // 2nd stage: column transform
    for i in 0..2 {
        let mut buf1_cur: *mut int32x4_t = buf1_ptr.offset((i * txfm_size_row) as isize);
        transpose_4x4(buf1_cur, buf1_cur);
        col_txfm.unwrap()(buf1_cur, buf1_cur, INV_COS_BIT, 1, bd, 0);
    }
    round_shift_array_32_neon(buf1_ptr, buf1_ptr, txfm_size_col, -(*shift.add(1) as i32));
    // write to buffer
    highbd_write_buffer_8xn_neon(buf1_ptr, output, stride, ud_flip, txfm_size_row, bd);
}

#[no_mangle]
pub unsafe extern "C-unwind" fn av1_inv_txfm2d_add_4x16_neon(
    mut input: *const i32,
    mut output: *mut u16,
    mut stride: i32,
    mut tx_type: TX_TYPE,
    mut bd: i32,
) {
    let mut tx_size: TX_SIZE = TX_4X16 as TX_SIZE;
    let mut buf1: [int32x4_t; 16] = core::mem::zeroed();
    let mut shift: *const i8 = av1_inv_txfm_shift_ls[tx_size as usize];
    let txw_idx: i32 = get_txw_idx(tx_size);
    let txh_idx: i32 = get_txh_idx(tx_size);
    let txfm_size_col: i32 = tx_size_wide[tx_size as usize];
    let txfm_size_row: i32 = tx_size_high[tx_size as usize];
    let buf_size_h_div8: i32 = txfm_size_row >> 2;
    let row_txfm: transform_1d_neon = highbd_txfm_all_1d_zeros_w8_arr[txw_idx as usize]
        [hitx_1d_tab[tx_type as usize] as usize][0];
    let col_txfm: transform_1d_neon = highbd_txfm_all_1d_zeros_w8_arr[txh_idx as usize]
        [vitx_1d_tab[tx_type as usize] as usize][2];
    let input_stride: i32 = AOMMIN(32, txfm_size_row);

    debug_assert!(col_txfm.is_some());
    debug_assert!(row_txfm.is_some());
    let mut ud_flip: i32 = 0;
    let mut lr_flip: i32 = 0;
    get_flip_cfg(tx_type, &raw mut ud_flip, &raw mut lr_flip);

    // 1st stage: column transform
    let mut buf0: [int32x4_t; 16] = core::mem::zeroed();
    for i in 0..(txfm_size_row >> 2) {
        let mut input_row: *const i32 = input.offset((i * 4) as isize);
        let mut buf0_cur: *mut int32x4_t = buf0.as_mut_ptr().offset((i * 4) as isize);
        load_buffer_32bit_input(input_row, input_stride, buf0_cur, txfm_size_col);
        row_txfm.unwrap()(
            buf0.as_mut_ptr().offset((i << 2) as isize),
            buf0.as_mut_ptr().offset((i << 2) as isize),
            INV_COS_BIT,
            0,
            bd,
            -(*shift as i32),
        );
    }

    if lr_flip != 0 {
        for j in 0..buf_size_h_div8 {
            TRANSPOSE_4X4!(
                buf0[(4 * j + 3) as usize],
                buf0[(4 * j + 2) as usize],
                buf0[(4 * j + 1) as usize],
                buf0[(4 * j) as usize],
                buf1[(4 * j) as usize],
                buf1[(4 * j + 1) as usize],
                buf1[(4 * j + 2) as usize],
                buf1[(4 * j + 3) as usize]
            );
        }
    } else {
        for j in 0..buf_size_h_div8 {
            TRANSPOSE_4X4!(
                buf0[(4 * j) as usize],
                buf0[(4 * j + 1) as usize],
                buf0[(4 * j + 2) as usize],
                buf0[(4 * j + 3) as usize],
                buf1[(4 * j) as usize],
                buf1[(4 * j + 1) as usize],
                buf1[(4 * j + 2) as usize],
                buf1[(4 * j + 3) as usize]
            );
        }
    }

    // 2nd stage: column transform
    col_txfm.unwrap()(buf1.as_mut_ptr(), buf1.as_mut_ptr(), INV_COS_BIT, 1, bd, 0);

    round_shift_array_32_neon(
        buf1.as_mut_ptr(),
        buf1.as_mut_ptr(),
        txfm_size_row,
        -(*shift.add(1) as i32),
    );

    // write to buffer
    highbd_write_buffer_4xn_neon(buf1.as_mut_ptr(), output, stride, ud_flip, txfm_size_row, bd);
}

#[no_mangle]
pub unsafe extern "C-unwind" fn av1_inv_txfm2d_add_16x4_neon(
    mut input: *const i32,
    mut output: *mut u16,
    mut stride: i32,
    mut tx_type: TX_TYPE,
    mut bd: i32,
) {
    let mut tx_size: TX_SIZE = TX_16X4 as TX_SIZE;
    let mut buf1: [int32x4_t; 16] = core::mem::zeroed();
    let mut shift: *const i8 = av1_inv_txfm_shift_ls[tx_size as usize];
    let txw_idx: i32 = get_txw_idx(tx_size);
    let txh_idx: i32 = get_txh_idx(tx_size);
    let txfm_size_col: i32 = tx_size_wide[tx_size as usize];
    let txfm_size_row: i32 = tx_size_high[tx_size as usize];
    let buf_size_w_div8: i32 = txfm_size_col >> 2;
    let row_txfm: transform_1d_neon = highbd_txfm_all_1d_zeros_w8_arr[txw_idx as usize]
        [hitx_1d_tab[tx_type as usize] as usize][2];
    let col_txfm: transform_1d_neon = highbd_txfm_all_1d_zeros_w8_arr[txh_idx as usize]
        [vitx_1d_tab[tx_type as usize] as usize][0];

    debug_assert!(col_txfm.is_some());
    debug_assert!(row_txfm.is_some());
    let mut ud_flip: i32 = 0;
    let mut lr_flip: i32 = 0;
    get_flip_cfg(tx_type, &raw mut ud_flip, &raw mut lr_flip);

    // 1st stage: column transform
    let mut buf0: [int32x4_t; 16] = core::mem::zeroed();
    let mut input_row: *const i32 = input;
    load_buffer_32bit_input(input_row, 4, buf0.as_mut_ptr(), txfm_size_col);

    row_txfm.unwrap()(buf0.as_mut_ptr(), buf0.as_mut_ptr(), INV_COS_BIT, 0, bd, -(*shift as i32));

    let mut buf1_ptr: *mut int32x4_t = core::ptr::null_mut();
    if lr_flip != 0 {
        flip_buf_neon(buf0.as_mut_ptr(), buf1.as_mut_ptr(), txfm_size_col);
        buf1_ptr = buf1.as_mut_ptr();
    } else {
        buf1_ptr = buf0.as_mut_ptr();
    }

    // 2nd stage: column transform
    for i in 0..buf_size_w_div8 {
        let mut buf1_cur: *mut int32x4_t = buf1_ptr.offset((i * txfm_size_row) as isize);
        transpose_4x4(buf1_cur, buf1_cur);
        col_txfm.unwrap()(buf1_cur, buf1_cur, INV_COS_BIT, 1, bd, 0);
    }
    round_shift_array_32_neon(buf1_ptr, buf1_ptr, txfm_size_col, -(*shift.add(1) as i32));

    // write to buffer
    for i in 0..(txfm_size_col >> 3) {
        highbd_write_buffer_8xn_neon(
            buf1_ptr.offset((i * txfm_size_row * 2) as isize),
            output.offset((8 * i) as isize),
            stride,
            ud_flip,
            txfm_size_row,
            bd,
        );
    }
}

static lowbd_txfm_all_1d_zeros_idx: [i32; 32] = [
    0, 1, 1, 1, 1, 1, 1, 1, 2, 2, 2, 2, 2, 2, 2, 2, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3,
];

// Transform block width in log2 for eob (size of 64 map to 32)
static tx_size_wide_log2_eob: [i32; 19] = [2, 3, 4, 5, 5, 2, 3, 3, 4, 4, 5, 5, 5, 2, 4, 3, 5, 4, 5];

// The DECLARE_ALIGNED(16, ...) alignment of these tables is not needed:
// they are only read element by element.
static av1_eob_to_eobxy_8x8_default: [i16; 8] =
    [0x0707, 0x0707, 0x0707, 0x0707, 0x0707, 0x0707, 0x0707, 0x0707];

static av1_eob_to_eobxy_16x16_default: [i16; 16] = [
    0x0707, 0x0707, 0x0f0f, 0x0f0f, 0x0f0f, 0x0f0f, 0x0f0f, 0x0f0f, 0x0f0f, 0x0f0f, 0x0f0f, 0x0f0f,
    0x0f0f, 0x0f0f, 0x0f0f, 0x0f0f,
];

static av1_eob_to_eobxy_32x32_default: [i16; 32] = [
    0x0707, 0x0f0f, 0x0f0f, 0x0f0f, 0x1f1f, 0x1f1f, 0x1f1f, 0x1f1f, 0x1f1f, 0x1f1f, 0x1f1f, 0x1f1f,
    0x1f1f, 0x1f1f, 0x1f1f, 0x1f1f, 0x1f1f, 0x1f1f, 0x1f1f, 0x1f1f, 0x1f1f, 0x1f1f, 0x1f1f, 0x1f1f,
    0x1f1f, 0x1f1f, 0x1f1f, 0x1f1f, 0x1f1f, 0x1f1f, 0x1f1f, 0x1f1f,
];

static av1_eob_to_eobxy_8x16_default: [i16; 16] = [
    0x0707, 0x0707, 0x0707, 0x0707, 0x0707, 0x0f07, 0x0f07, 0x0f07, 0x0f07, 0x0f07, 0x0f07, 0x0f07,
    0x0f07, 0x0f07, 0x0f07, 0x0f07,
];

static av1_eob_to_eobxy_16x8_default: [i16; 8] =
    [0x0707, 0x0707, 0x070f, 0x070f, 0x070f, 0x070f, 0x070f, 0x070f];

static av1_eob_to_eobxy_16x32_default: [i16; 32] = [
    0x0707, 0x0707, 0x0f0f, 0x0f0f, 0x0f0f, 0x0f0f, 0x0f0f, 0x0f0f, 0x0f0f, 0x1f0f, 0x1f0f, 0x1f0f,
    0x1f0f, 0x1f0f, 0x1f0f, 0x1f0f, 0x1f0f, 0x1f0f, 0x1f0f, 0x1f0f, 0x1f0f, 0x1f0f, 0x1f0f, 0x1f0f,
    0x1f0f, 0x1f0f, 0x1f0f, 0x1f0f, 0x1f0f, 0x1f0f, 0x1f0f, 0x1f0f,
];

static av1_eob_to_eobxy_32x16_default: [i16; 16] = [
    0x0707, 0x0f0f, 0x0f0f, 0x0f0f, 0x0f1f, 0x0f1f, 0x0f1f, 0x0f1f, 0x0f1f, 0x0f1f, 0x0f1f, 0x0f1f,
    0x0f1f, 0x0f1f, 0x0f1f, 0x0f1f,
];

static av1_eob_to_eobxy_8x32_default: [i16; 32] = [
    0x0707, 0x0707, 0x0707, 0x0707, 0x0707, 0x0f07, 0x0f07, 0x0f07, 0x0f07, 0x0f07, 0x0f07, 0x0f07,
    0x0f07, 0x1f07, 0x1f07, 0x1f07, 0x1f07, 0x1f07, 0x1f07, 0x1f07, 0x1f07, 0x1f07, 0x1f07, 0x1f07,
    0x1f07, 0x1f07, 0x1f07, 0x1f07, 0x1f07, 0x1f07, 0x1f07, 0x1f07,
];

static av1_eob_to_eobxy_32x8_default: [i16; 8] =
    [0x0707, 0x070f, 0x070f, 0x071f, 0x071f, 0x071f, 0x071f, 0x071f];

// DECLARE_ALIGNED(16, ...) in the original; `static mut` because the
// table holds raw pointers.
static mut av1_eob_to_eobxy_default: [*const i16; 19] = [
    core::ptr::null(),
    av1_eob_to_eobxy_8x8_default.as_ptr(),
    av1_eob_to_eobxy_16x16_default.as_ptr(),
    av1_eob_to_eobxy_32x32_default.as_ptr(),
    av1_eob_to_eobxy_32x32_default.as_ptr(),
    core::ptr::null(),
    core::ptr::null(),
    av1_eob_to_eobxy_8x16_default.as_ptr(),
    av1_eob_to_eobxy_16x8_default.as_ptr(),
    av1_eob_to_eobxy_16x32_default.as_ptr(),
    av1_eob_to_eobxy_32x16_default.as_ptr(),
    av1_eob_to_eobxy_32x32_default.as_ptr(),
    av1_eob_to_eobxy_32x32_default.as_ptr(),
    core::ptr::null(),
    core::ptr::null(),
    av1_eob_to_eobxy_8x32_default.as_ptr(),
    av1_eob_to_eobxy_32x8_default.as_ptr(),
    av1_eob_to_eobxy_16x32_default.as_ptr(),
    av1_eob_to_eobxy_32x16_default.as_ptr(),
];

#[inline(always)]
unsafe fn highbd_get_eobx_eoby_scan_default(
    mut eobx: *mut i32,
    mut eoby: *mut i32,
    mut tx_size: TX_SIZE,
    mut eob: i32,
) {
    if eob == 1 {
        *eobx = 0;
        *eoby = 0;
        return;
    }

    let tx_w_log2: i32 = tx_size_wide_log2_eob[tx_size as usize];
    let eob_row: i32 = (eob - 1) >> tx_w_log2;
    let eobxy: i32 = *av1_eob_to_eobxy_default[tx_size as usize].offset(eob_row as isize) as i32;
    *eobx = eobxy & 0xFF;
    *eoby = eobxy >> 8;
}

#[inline(always)]
unsafe fn get_eobx_eoby_scan_default(mut eobx: *mut i32, mut eoby: *mut i32, mut tx_size: TX_SIZE) {
    if (tx_size as i32) == 2 {
        *eoby = 15;
        *eobx = 15;
    } else if (tx_size as i32) == 3 {
        *eoby = 31;
        *eobx = 31;
    } else if (tx_size as i32) == 4 {
        *eoby = 31;
        *eobx = 31;
    } else if (tx_size as i32) == 7 {
        *eoby = 15;
        *eobx = 7;
    } else if (tx_size as i32) == 8 {
        *eoby = 7;
        *eobx = 15;
    } else if (tx_size as i32) == 9 {
        *eoby = 31;
        *eobx = 15;
    } else if (tx_size as i32) == 10 {
        *eoby = 15;
        *eobx = 31;
    } else if (tx_size as i32) == 11 {
        *eoby = 31;
        *eobx = 31;
    } else if (tx_size as i32) == 12 {
        *eoby = 31;
        *eobx = 31;
    } else if (tx_size as i32) == 15 {
        *eoby = 31;
        *eobx = 7;
    } else if (tx_size as i32) == 16 {
        *eoby = 7;
        *eobx = 31;
    } else if (tx_size as i32) == 17 {
        *eoby = 31;
        *eobx = 15;
    } else if (tx_size as i32) == 18 {
        *eoby = 15;
        *eobx = 31;
    } else {
        *eoby = 0;
        *eobx = 0;
    }
}

#[inline(always)]
unsafe fn get_eobx_eoby_scan_v_identity(
    mut eobx: *mut i32,
    mut eoby: *mut i32,
    mut tx_size: TX_SIZE,
) {
    let txfm_size_row: i32 = tx_size_high[tx_size as usize];
    *eoby = AOMMIN(32, txfm_size_row) - 1;
    *eobx = 0;
}

#[inline(always)]
unsafe fn get_eobx_eoby_scan_h_identity(
    mut eobx: *mut i32,
    mut eoby: *mut i32,
    mut tx_size: TX_SIZE,
) {
    let txfm_size_col: i32 = tx_size_wide[tx_size as usize];
    *eobx = AOMMIN(32, txfm_size_col) - 1;
    *eoby = 0;
}

unsafe fn inv_txfm2d_add_h_identity_neon(
    mut input: *const i32,
    mut output: *mut u16,
    mut stride: i32,
    mut tx_type: TX_TYPE,
    mut tx_size: TX_SIZE,
    mut bd: i32,
) {
    let mut buf1: [int32x4_t; 64] = core::mem::zeroed();
    let mut eobx: i32 = 0;
    let mut eoby: i32 = 0;
    get_eobx_eoby_scan_v_identity(&raw mut eobx, &raw mut eoby, tx_size);
    let mut shift: *const i8 = av1_inv_txfm_shift_ls[tx_size as usize];
    let txw_idx: i32 = get_txw_idx(tx_size);
    let txh_idx: i32 = get_txh_idx(tx_size);
    let txfm_size_col: i32 = tx_size_wide[tx_size as usize];
    let txfm_size_row: i32 = tx_size_high[tx_size as usize];
    let buf_size_w: i32 = AOMMIN(32, txfm_size_col);
    let buf_size_w_div4: i32 = buf_size_w >> 2;
    let buf_size_h_div8: i32 = (eoby + 8) >> 3;
    let row_max: i32 = AOMMIN(32, txfm_size_row);
    let input_stride: i32 = row_max;
    let rect_type: i32 = get_rect_tx_log_ratio(txfm_size_col, txfm_size_row);
    let fun_idx: i32 = lowbd_txfm_all_1d_zeros_idx[eoby as usize];
    let row_txfm: transform_1d_neon = highbd_txfm_all_1d_zeros_w8_arr[txw_idx as usize]
        [hitx_1d_tab[tx_type as usize] as usize][0];
    debug_assert!(row_txfm.is_some());
    let col_txfm: transform_1d_neon = highbd_txfm_all_1d_zeros_w8_arr[txh_idx as usize]
        [vitx_1d_tab[tx_type as usize] as usize][fun_idx as usize];
    debug_assert!(col_txfm.is_some());
    let mut ud_flip: i32 = 0;
    let mut lr_flip: i32 = 0;
    get_flip_cfg(tx_type, &raw mut ud_flip, &raw mut lr_flip);

    for i in 0..(buf_size_h_div8 << 1) {
        let mut buf0: [int32x4_t; 16] = core::mem::zeroed();
        load_buffer_32bit_input(
            input.offset((i * 4) as isize),
            input_stride,
            buf0.as_mut_ptr(),
            buf_size_w,
        );
        if rect_type == 1 || rect_type == -1 {
            round_shift_rect_array_32_neon(buf0.as_mut_ptr(), buf0.as_mut_ptr(), buf_size_w);
        }
        row_txfm.unwrap()(
            buf0.as_mut_ptr(),
            buf0.as_mut_ptr(),
            INV_COS_BIT,
            0,
            bd,
            -(*shift as i32),
        );

        let mut _buf1: *mut int32x4_t = buf1.as_mut_ptr().offset((i * 4) as isize);

        for j in 0..buf_size_w_div4 {
            let mut buf0_cur: *mut int32x4_t = buf0.as_mut_ptr().offset((j * 4) as isize);
            TRANSPOSE_4X4!(
                *buf0_cur,
                *buf0_cur.add(1),
                *buf0_cur.add(2),
                *buf0_cur.add(3),
                *buf0_cur,
                *buf0_cur.add(1),
                *buf0_cur.add(2),
                *buf0_cur.add(3)
            );
            *_buf1.offset((j * txfm_size_row + 0) as isize) = *buf0_cur;
            *_buf1.offset((j * txfm_size_row + 1) as isize) = *buf0_cur.add(1);
            *_buf1.offset((j * txfm_size_row + 2) as isize) = *buf0_cur.add(2);
            *_buf1.offset((j * txfm_size_row + 3) as isize) = *buf0_cur.add(3);
        }
    }
    for i in 0..buf_size_w_div4 {
        col_txfm.unwrap()(
            buf1.as_mut_ptr().offset((i * txfm_size_row) as isize),
            buf1.as_mut_ptr().offset((i * txfm_size_row) as isize),
            INV_COS_BIT,
            1,
            bd,
            0,
        );

        round_shift_array_32_neon(
            buf1.as_mut_ptr().offset((i * txfm_size_row) as isize),
            buf1.as_mut_ptr().offset((i * txfm_size_row) as isize),
            txfm_size_row,
            -(*shift.add(1) as i32),
        );
    }

    // write to buffer
    for i in 0..(txfm_size_col >> 3) {
        highbd_write_buffer_8xn_neon(
            buf1.as_mut_ptr().offset((i * txfm_size_row * 2) as isize),
            output.offset((8 * i) as isize),
            stride,
            ud_flip,
            txfm_size_row,
            bd,
        );
    }
}

unsafe fn inv_txfm2d_add_v_identity_neon(
    mut input: *const i32,
    mut output: *mut u16,
    mut stride: i32,
    mut tx_type: TX_TYPE,
    mut tx_size: TX_SIZE,
    mut bd: i32,
) {
    let mut buf1: [int32x4_t; 64] = core::mem::zeroed();
    let mut eobx: i32 = 0;
    let mut eoby: i32 = 0;
    get_eobx_eoby_scan_h_identity(&raw mut eobx, &raw mut eoby, tx_size);
    let mut shift: *const i8 = av1_inv_txfm_shift_ls[tx_size as usize];
    let txw_idx: i32 = get_txw_idx(tx_size);
    let txh_idx: i32 = get_txh_idx(tx_size);
    let txfm_size_col: i32 = tx_size_wide[tx_size as usize];
    let txfm_size_row: i32 = tx_size_high[tx_size as usize];
    let buf_size_w_div4: i32 = AOMMIN(32, txfm_size_col) >> 2;
    let row_max: i32 = AOMMIN(32, txfm_size_row);
    let input_stride: i32 = row_max;
    let buf_size_nonzero_w_div8: i32 = (eobx + 8) >> 3;
    let buf_size_nonzero_w: i32 = buf_size_nonzero_w_div8 << 3;
    let rect_type: i32 = get_rect_tx_log_ratio(txfm_size_col, txfm_size_row);
    let fun_idx: i32 = lowbd_txfm_all_1d_zeros_idx[eobx as usize];
    let row_txfm: transform_1d_neon = highbd_txfm_all_1d_zeros_w8_arr[txw_idx as usize]
        [hitx_1d_tab[tx_type as usize] as usize][fun_idx as usize];
    debug_assert!(row_txfm.is_some());
    let col_txfm: transform_1d_neon = highbd_txfm_all_1d_zeros_w8_arr[txh_idx as usize]
        [vitx_1d_tab[tx_type as usize] as usize][0];
    debug_assert!(col_txfm.is_some());
    let mut ud_flip: i32 = 0;
    let mut lr_flip: i32 = 0;
    get_flip_cfg(tx_type, &raw mut ud_flip, &raw mut lr_flip);

    for i in 0..(row_max >> 2) {
        let mut buf0: [int32x4_t; 16] = core::mem::zeroed();
        load_buffer_32bit_input(
            input.offset((i * 4) as isize),
            input_stride,
            buf0.as_mut_ptr(),
            buf_size_nonzero_w,
        );
        if rect_type == 1 || rect_type == -1 {
            round_shift_rect_array_32_neon(
                buf0.as_mut_ptr(),
                buf0.as_mut_ptr(),
                buf_size_nonzero_w,
            );
        }
        row_txfm.unwrap()(
            buf0.as_mut_ptr(),
            buf0.as_mut_ptr(),
            INV_COS_BIT,
            0,
            bd,
            -(*shift as i32),
        );

        let mut _buf1: *mut int32x4_t = buf1.as_mut_ptr().offset((i * 4) as isize);
        if lr_flip != 0 {
            for j in 0..buf_size_w_div4 {
                TRANSPOSE_4X4!(
                    buf0[(4 * j + 3) as usize],
                    buf0[(4 * j + 2) as usize],
                    buf0[(4 * j + 1) as usize],
                    buf0[(4 * j) as usize],
                    *_buf1.offset((txfm_size_row * (buf_size_w_div4 - 1 - j) + 0) as isize),
                    *_buf1.offset((txfm_size_row * (buf_size_w_div4 - 1 - j) + 1) as isize),
                    *_buf1.offset((txfm_size_row * (buf_size_w_div4 - 1 - j) + 2) as isize),
                    *_buf1.offset((txfm_size_row * (buf_size_w_div4 - 1 - j) + 3) as isize)
                );
            }
        } else {
            for j in 0..buf_size_w_div4 {
                TRANSPOSE_4X4!(
                    buf0[(j * 4 + 0) as usize],
                    buf0[(j * 4 + 1) as usize],
                    buf0[(j * 4 + 2) as usize],
                    buf0[(j * 4 + 3) as usize],
                    *_buf1.offset((j * txfm_size_row + 0) as isize),
                    *_buf1.offset((j * txfm_size_row + 1) as isize),
                    *_buf1.offset((j * txfm_size_row + 2) as isize),
                    *_buf1.offset((j * txfm_size_row + 3) as isize)
                );
            }
        }
    }
    for i in 0..buf_size_w_div4 {
        col_txfm.unwrap()(
            buf1.as_mut_ptr().offset((i * txfm_size_row) as isize),
            buf1.as_mut_ptr().offset((i * txfm_size_row) as isize),
            INV_COS_BIT,
            1,
            bd,
            0,
        );

        round_shift_array_32_neon(
            buf1.as_mut_ptr().offset((i * txfm_size_row) as isize),
            buf1.as_mut_ptr().offset((i * txfm_size_row) as isize),
            txfm_size_row,
            -(*shift.add(1) as i32),
        );
    }

    // write to buffer
    {
        for i in 0..(txfm_size_col >> 3) {
            highbd_write_buffer_8xn_neon(
                buf1.as_mut_ptr().offset((i * txfm_size_row * 2) as isize),
                output.offset((8 * i) as isize),
                stride,
                ud_flip,
                txfm_size_row,
                bd,
            );
        }
    }
}

unsafe fn inv_txfm2d_add_idtx_neon(
    mut input: *const i32,
    mut output: *mut u16,
    mut stride: i32,
    mut tx_type: TX_TYPE,
    mut tx_size: TX_SIZE,
    mut bd: i32,
) {
    let mut buf1: [int32x4_t; 256] = core::mem::zeroed();
    let mut shift: *const i8 = av1_inv_txfm_shift_ls[tx_size as usize];
    let txw_idx: i32 = get_txw_idx(tx_size);
    let txh_idx: i32 = get_txh_idx(tx_size);
    let txfm_size_col: i32 = tx_size_wide[tx_size as usize];
    let txfm_size_row: i32 = tx_size_high[tx_size as usize];
    let row_max: i32 = AOMMIN(32, txfm_size_row);
    let input_stride: i32 = row_max;
    let buf_size_w: i32 = AOMMIN(32, txfm_size_col);
    let buf_size_w_div4: i32 = buf_size_w >> 2;
    let rect_type: i32 = get_rect_tx_log_ratio(txfm_size_col, txfm_size_row);
    let row_txfm: transform_1d_neon = highbd_txfm_all_1d_zeros_w8_arr[txw_idx as usize]
        [hitx_1d_tab[tx_type as usize] as usize][0];
    debug_assert!(row_txfm.is_some());
    let col_txfm: transform_1d_neon = highbd_txfm_all_1d_zeros_w8_arr[txh_idx as usize]
        [vitx_1d_tab[tx_type as usize] as usize][0];
    debug_assert!(col_txfm.is_some());
    for i in 0..(row_max >> 2) {
        let mut buf0: [int32x4_t; 32] = core::mem::zeroed();
        load_buffer_32bit_input(
            input.offset((i * 4) as isize),
            input_stride,
            buf0.as_mut_ptr(),
            buf_size_w,
        );
        if rect_type == 1 || rect_type == -1 {
            round_shift_rect_array_32_neon(buf0.as_mut_ptr(), buf0.as_mut_ptr(), buf_size_w);
        }
        row_txfm.unwrap()(
            buf0.as_mut_ptr(),
            buf0.as_mut_ptr(),
            INV_COS_BIT,
            0,
            bd,
            -(*shift as i32),
        );

        let mut _buf1: *mut int32x4_t = buf1.as_mut_ptr().offset((i * 4) as isize);
        for j in 0..buf_size_w_div4 {
            let mut buf0_cur: *mut int32x4_t = buf0.as_mut_ptr().offset((j * 4) as isize);
            TRANSPOSE_4X4!(
                *buf0_cur,
                *buf0_cur.add(1),
                *buf0_cur.add(2),
                *buf0_cur.add(3),
                *buf0_cur,
                *buf0_cur.add(1),
                *buf0_cur.add(2),
                *buf0_cur.add(3)
            );
            *_buf1.offset((j * txfm_size_row + 0) as isize) = *buf0_cur;
            *_buf1.offset((j * txfm_size_row + 1) as isize) = *buf0_cur.add(1);
            *_buf1.offset((j * txfm_size_row + 2) as isize) = *buf0_cur.add(2);
            *_buf1.offset((j * txfm_size_row + 3) as isize) = *buf0_cur.add(3);
        }
    }
    for i in 0..buf_size_w_div4 {
        col_txfm.unwrap()(
            buf1.as_mut_ptr().offset((i * txfm_size_row) as isize),
            buf1.as_mut_ptr().offset((i * txfm_size_row) as isize),
            INV_COS_BIT,
            1,
            bd,
            0,
        );

        round_shift_array_32_neon(
            buf1.as_mut_ptr().offset((i * txfm_size_row) as isize),
            buf1.as_mut_ptr().offset((i * txfm_size_row) as isize),
            txfm_size_row,
            -(*shift.add(1) as i32),
        );
    }

    // write to buffer
    {
        for i in 0..(txfm_size_col >> 3) {
            highbd_write_buffer_8xn_neon(
                buf1.as_mut_ptr().offset((i * txfm_size_row * 2) as isize),
                output.offset((8 * i) as isize),
                stride,
                0,
                txfm_size_row,
                bd,
            );
        }
    }
}

unsafe fn inv_txfm2d_add_no_identity_neon(
    mut input: *const i32,
    mut output: *mut u16,
    mut stride: i32,
    mut tx_type: TX_TYPE,
    mut tx_size: TX_SIZE,
    mut bd: i32,
) {
    let mut buf1: [int32x4_t; 1024] = core::mem::zeroed();
    let mut eobx: i32 = 0;
    let mut eoby: i32 = 0;
    get_eobx_eoby_scan_default(&raw mut eobx, &raw mut eoby, tx_size);
    let mut shift: *const i8 = av1_inv_txfm_shift_ls[tx_size as usize];
    let txw_idx: i32 = get_txw_idx(tx_size);
    let txh_idx: i32 = get_txh_idx(tx_size);
    let txfm_size_col: i32 = tx_size_wide[tx_size as usize];
    let txfm_size_row: i32 = tx_size_high[tx_size as usize];
    let buf_size_w_div4: i32 = txfm_size_col >> 2;
    let buf_size_nonzero_w: i32 = (eobx + 8) >> 3 << 3;
    let buf_size_nonzero_h_div8: i32 = (eoby + 8) >> 3;
    let input_stride: i32 = AOMMIN(32, txfm_size_row);
    let rect_type: i32 = get_rect_tx_log_ratio(txfm_size_col, txfm_size_row);

    let fun_idx_x: i32 = lowbd_txfm_all_1d_zeros_idx[eobx as usize];
    let fun_idx_y: i32 = lowbd_txfm_all_1d_zeros_idx[eoby as usize];
    let row_txfm: transform_1d_neon = highbd_txfm_all_1d_zeros_w8_arr[txw_idx as usize]
        [hitx_1d_tab[tx_type as usize] as usize][fun_idx_x as usize];
    let col_txfm: transform_1d_neon = highbd_txfm_all_1d_zeros_w8_arr[txh_idx as usize]
        [vitx_1d_tab[tx_type as usize] as usize][fun_idx_y as usize];

    debug_assert!(col_txfm.is_some());
    debug_assert!(row_txfm.is_some());
    let mut ud_flip: i32 = 0;
    let mut lr_flip: i32 = 0;
    get_flip_cfg(tx_type, &raw mut ud_flip, &raw mut lr_flip);
    // 1st stage: column transform
    for i in 0..(buf_size_nonzero_h_div8 << 1) {
        let mut buf0: [int32x4_t; 64] = core::mem::zeroed();
        load_buffer_32bit_input(
            input.offset((i * 4) as isize),
            input_stride,
            buf0.as_mut_ptr(),
            buf_size_nonzero_w,
        );
        if rect_type == 1 || rect_type == -1 {
            round_shift_rect_array_32_neon(
                buf0.as_mut_ptr(),
                buf0.as_mut_ptr(),
                buf_size_nonzero_w,
            );
        }
        row_txfm.unwrap()(
            buf0.as_mut_ptr(),
            buf0.as_mut_ptr(),
            INV_COS_BIT,
            0,
            bd,
            -(*shift as i32),
        );

        let mut _buf1: *mut int32x4_t = buf1.as_mut_ptr().offset((i * 4) as isize);

        if lr_flip != 0 {
            for j in 0..buf_size_w_div4 {
                TRANSPOSE_4X4!(
                    buf0[(4 * j + 3) as usize],
                    buf0[(4 * j + 2) as usize],
                    buf0[(4 * j + 1) as usize],
                    buf0[(4 * j) as usize],
                    *_buf1.offset((txfm_size_row * (buf_size_w_div4 - 1 - j) + 0) as isize),
                    *_buf1.offset((txfm_size_row * (buf_size_w_div4 - 1 - j) + 1) as isize),
                    *_buf1.offset((txfm_size_row * (buf_size_w_div4 - 1 - j) + 2) as isize),
                    *_buf1.offset((txfm_size_row * (buf_size_w_div4 - 1 - j) + 3) as isize)
                );
            }
        } else {
            for j in 0..buf_size_w_div4 {
                TRANSPOSE_4X4!(
                    buf0[(j * 4 + 0) as usize],
                    buf0[(j * 4 + 1) as usize],
                    buf0[(j * 4 + 2) as usize],
                    buf0[(j * 4 + 3) as usize],
                    *_buf1.offset((j * txfm_size_row + 0) as isize),
                    *_buf1.offset((j * txfm_size_row + 1) as isize),
                    *_buf1.offset((j * txfm_size_row + 2) as isize),
                    *_buf1.offset((j * txfm_size_row + 3) as isize)
                );
            }
        }
    }
    // 2nd stage: column transform
    for i in 0..buf_size_w_div4 {
        col_txfm.unwrap()(
            buf1.as_mut_ptr().offset((i * txfm_size_row) as isize),
            buf1.as_mut_ptr().offset((i * txfm_size_row) as isize),
            INV_COS_BIT,
            1,
            bd,
            0,
        );

        round_shift_array_32_neon(
            buf1.as_mut_ptr().offset((i * txfm_size_row) as isize),
            buf1.as_mut_ptr().offset((i * txfm_size_row) as isize),
            txfm_size_row,
            -(*shift.add(1) as i32),
        );
    }

    // write to buffer
    {
        for i in 0..(txfm_size_col >> 3) {
            highbd_write_buffer_8xn_neon(
                buf1.as_mut_ptr().offset((i * txfm_size_row * 2) as isize),
                output.offset((8 * i) as isize),
                stride,
                ud_flip,
                txfm_size_row,
                bd,
            );
        }
    }
}

unsafe fn highbd_inv_txfm2d_add_no_identity_neon(
    mut input: *const i32,
    mut output: *mut u16,
    mut stride: i32,
    mut tx_type: TX_TYPE,
    mut tx_size: TX_SIZE,
    mut eob: i32,
    mut bd: i32,
) {
    let mut buf1: [int32x4_t; 1024] = core::mem::zeroed();
    let mut eobx: i32 = 0;
    let mut eoby: i32 = 0;
    highbd_get_eobx_eoby_scan_default(&raw mut eobx, &raw mut eoby, tx_size, eob);
    let mut shift: *const i8 = av1_inv_txfm_shift_ls[tx_size as usize];
    let txw_idx: i32 = get_txw_idx(tx_size);
    let txh_idx: i32 = get_txh_idx(tx_size);
    let txfm_size_col: i32 = tx_size_wide[tx_size as usize];
    let txfm_size_row: i32 = tx_size_high[tx_size as usize];
    let buf_size_w_div8: i32 = txfm_size_col >> 2;
    let buf_size_nonzero_w_div8: i32 = (eobx + 8) >> 3;
    let buf_size_nonzero_h_div8: i32 = (eoby + 8) >> 3;
    let input_stride: i32 = AOMMIN(32, txfm_size_col);
    let rect_type: i32 = get_rect_tx_log_ratio(txfm_size_col, txfm_size_row);

    let fun_idx_x: i32 = lowbd_txfm_all_1d_zeros_idx[eobx as usize];
    let fun_idx_y: i32 = lowbd_txfm_all_1d_zeros_idx[eoby as usize];
    let row_txfm: transform_1d_neon = highbd_txfm_all_1d_zeros_w8_arr[txw_idx as usize]
        [hitx_1d_tab[tx_type as usize] as usize][fun_idx_x as usize];
    let col_txfm: transform_1d_neon = highbd_txfm_all_1d_zeros_w8_arr[txh_idx as usize]
        [vitx_1d_tab[tx_type as usize] as usize][fun_idx_y as usize];

    debug_assert!(col_txfm.is_some());
    debug_assert!(row_txfm.is_some());
    let mut ud_flip: i32 = 0;
    let mut lr_flip: i32 = 0;
    get_flip_cfg(tx_type, &raw mut ud_flip, &raw mut lr_flip);
    // 1st stage: column transform
    for i in 0..(buf_size_nonzero_h_div8 << 1) {
        let mut buf0: [int32x4_t; 64] = core::mem::zeroed();
        let mut input_row: *const i32 = input.offset((i * input_stride * 4) as isize);
        for j in 0..(buf_size_nonzero_w_div8 << 1) {
            let mut buf0_cur: *mut int32x4_t = buf0.as_mut_ptr().offset((j * 4) as isize);
            load_buffer_32bit_input(input_row.offset((j * 4) as isize), input_stride, buf0_cur, 4);

            TRANSPOSE_4X4!(
                *buf0_cur,
                *buf0_cur.add(1),
                *buf0_cur.add(2),
                *buf0_cur.add(3),
                *buf0_cur,
                *buf0_cur.add(1),
                *buf0_cur.add(2),
                *buf0_cur.add(3)
            );
        }
        if rect_type == 1 || rect_type == -1 {
            round_shift_rect_array_32_neon(
                buf0.as_mut_ptr(),
                buf0.as_mut_ptr(),
                buf_size_nonzero_w_div8 << 3,
            );
        }
        row_txfm.unwrap()(
            buf0.as_mut_ptr(),
            buf0.as_mut_ptr(),
            INV_COS_BIT,
            0,
            bd,
            -(*shift as i32),
        );

        let mut _buf1: *mut int32x4_t = buf1.as_mut_ptr().offset((i * 4) as isize);

        if lr_flip != 0 {
            for j in 0..buf_size_w_div8 {
                TRANSPOSE_4X4!(
                    buf0[(4 * j + 3) as usize],
                    buf0[(4 * j + 2) as usize],
                    buf0[(4 * j + 1) as usize],
                    buf0[(4 * j) as usize],
                    *_buf1.offset((txfm_size_row * (buf_size_w_div8 - 1 - j) + 0) as isize),
                    *_buf1.offset((txfm_size_row * (buf_size_w_div8 - 1 - j) + 1) as isize),
                    *_buf1.offset((txfm_size_row * (buf_size_w_div8 - 1 - j) + 2) as isize),
                    *_buf1.offset((txfm_size_row * (buf_size_w_div8 - 1 - j) + 3) as isize)
                );
            }
        } else {
            for j in 0..buf_size_w_div8 {
                TRANSPOSE_4X4!(
                    buf0[(j * 4 + 0) as usize],
                    buf0[(j * 4 + 1) as usize],
                    buf0[(j * 4 + 2) as usize],
                    buf0[(j * 4 + 3) as usize],
                    *_buf1.offset((j * txfm_size_row + 0) as isize),
                    *_buf1.offset((j * txfm_size_row + 1) as isize),
                    *_buf1.offset((j * txfm_size_row + 2) as isize),
                    *_buf1.offset((j * txfm_size_row + 3) as isize)
                );
            }
        }
    }
    // 2nd stage: column transform
    for i in 0..buf_size_w_div8 {
        col_txfm.unwrap()(
            buf1.as_mut_ptr().offset((i * txfm_size_row) as isize),
            buf1.as_mut_ptr().offset((i * txfm_size_row) as isize),
            INV_COS_BIT,
            1,
            bd,
            0,
        );

        round_shift_array_32_neon(
            buf1.as_mut_ptr().offset((i * txfm_size_row) as isize),
            buf1.as_mut_ptr().offset((i * txfm_size_row) as isize),
            txfm_size_row,
            -(*shift.add(1) as i32),
        );
    }

    // write to buffer
    {
        for i in 0..(txfm_size_col >> 3) {
            highbd_write_buffer_8xn_neon(
                buf1.as_mut_ptr().offset((i * txfm_size_row * 2) as isize),
                output.offset((8 * i) as isize),
                stride,
                ud_flip,
                txfm_size_row,
                bd,
            );
        }
    }
}

unsafe fn highbd_inv_txfm2d_add_universe_neon(
    mut input: *const i32,
    mut output: *mut u8,
    mut stride: i32,
    mut tx_type: TX_TYPE,
    mut tx_size: TX_SIZE,
    mut eob: i32,
    mut bd: i32,
) {
    match tx_type as i32 {
        DCT_DCT | ADST_DCT | DCT_ADST | ADST_ADST | FLIPADST_DCT | DCT_FLIPADST
        | FLIPADST_FLIPADST | ADST_FLIPADST | FLIPADST_ADST => {
            highbd_inv_txfm2d_add_no_identity_neon(
                input,
                CONVERT_TO_SHORTPTR(output),
                stride,
                tx_type,
                tx_size,
                eob,
                bd,
            );
        }
        V_DCT | V_ADST | V_FLIPADST => {
            inv_txfm2d_add_h_identity_neon(
                input,
                CONVERT_TO_SHORTPTR(output),
                stride,
                tx_type,
                tx_size,
                bd,
            );
        }
        H_DCT | H_ADST | H_FLIPADST => {
            inv_txfm2d_add_v_identity_neon(
                input,
                CONVERT_TO_SHORTPTR(output),
                stride,
                tx_type,
                tx_size,
                bd,
            );
        }
        IDTX => {
            inv_txfm2d_add_idtx_neon(
                input,
                CONVERT_TO_SHORTPTR(output),
                stride,
                tx_type,
                tx_size,
                bd,
            );
        }
        _ => {
            debug_assert!(false);
        }
    }
}

unsafe fn inv_txfm2d_add_universe_neon(
    mut input: *const i32,
    mut output: *mut u8,
    mut stride: i32,
    mut tx_type: TX_TYPE,
    mut tx_size: TX_SIZE,
    mut bd: i32,
) {
    match tx_type as i32 {
        DCT_DCT | ADST_DCT | DCT_ADST | ADST_ADST | FLIPADST_DCT | DCT_FLIPADST
        | FLIPADST_FLIPADST | ADST_FLIPADST | FLIPADST_ADST => {
            inv_txfm2d_add_no_identity_neon(
                input,
                CONVERT_TO_SHORTPTR(output),
                stride,
                tx_type,
                tx_size,
                bd,
            );
        }
        V_DCT | V_ADST | V_FLIPADST => {
            inv_txfm2d_add_h_identity_neon(
                input,
                CONVERT_TO_SHORTPTR(output),
                stride,
                tx_type,
                tx_size,
                bd,
            );
        }
        H_DCT | H_ADST | H_FLIPADST => {
            inv_txfm2d_add_v_identity_neon(
                input,
                CONVERT_TO_SHORTPTR(output),
                stride,
                tx_type,
                tx_size,
                bd,
            );
        }
        IDTX => {
            inv_txfm2d_add_idtx_neon(
                input,
                CONVERT_TO_SHORTPTR(output),
                stride,
                tx_type,
                tx_size,
                bd,
            );
        }
        _ => {
            debug_assert!(false);
        }
    }
}

unsafe fn highbd_inv_txfm_add_8x8_neon(
    mut input: *const tran_low_t,
    mut dest: *mut u8,
    mut stride: i32,
    mut txfm_param: *const TxfmParam,
) {
    let mut bd: i32 = (*txfm_param).bd;
    let tx_type: TX_TYPE = (*txfm_param).tx_type;
    let mut src: *const i32 = cast_to_int32(input);
    match tx_type as i32 {
        IDTX | H_DCT | H_ADST | H_FLIPADST | V_DCT | V_ADST | V_FLIPADST => {
            highbd_inv_txfm2d_add_universe_neon(
                input,
                dest,
                stride,
                tx_type,
                (*txfm_param).tx_size,
                (*txfm_param).eob,
                bd,
            );
        }
        _ => {
            av1_inv_txfm2d_add_8x8_neon(src, CONVERT_TO_SHORTPTR(dest), stride, tx_type, bd);
        }
    }
}

unsafe fn highbd_inv_txfm_add_4x4_neon(
    mut input: *const tran_low_t,
    mut dest: *mut u8,
    mut stride: i32,
    mut txfm_param: *const TxfmParam,
) {
    debug_assert!(
        av1_ext_tx_used[(*txfm_param).tx_set_type as usize][(*txfm_param).tx_type as usize] != 0
    );
    let mut eob: i32 = (*txfm_param).eob;
    let mut bd: i32 = (*txfm_param).bd;
    let mut lossless: i32 = (*txfm_param).lossless;
    let mut src: *const i32 = cast_to_int32(input);
    let tx_type: TX_TYPE = (*txfm_param).tx_type;
    if lossless != 0 {
        debug_assert!((tx_type as i32) == DCT_DCT);
        av1_highbd_iwht4x4_add(input, dest, stride, eob, bd);
        return;
    }
    av1_inv_txfm2d_add_4x4_neon(src, CONVERT_TO_SHORTPTR(dest), stride, tx_type, bd);
}

#[no_mangle]
pub unsafe extern "C-unwind" fn av1_inv_txfm2d_add_8x16_neon(
    mut input: *const tran_low_t,
    mut dest: *mut u16,
    mut stride: i32,
    mut tx_type: TX_TYPE,
    mut bd: i32,
) {
    inv_txfm2d_add_universe_neon(input, dest as *mut u8, stride, tx_type, TX_8X16 as TX_SIZE, bd);
}

#[no_mangle]
pub unsafe extern "C-unwind" fn av1_inv_txfm2d_add_16x8_neon(
    mut input: *const tran_low_t,
    mut dest: *mut u16,
    mut stride: i32,
    mut tx_type: TX_TYPE,
    mut bd: i32,
) {
    inv_txfm2d_add_universe_neon(input, dest as *mut u8, stride, tx_type, TX_16X8 as TX_SIZE, bd);
}

#[no_mangle]
pub unsafe extern "C-unwind" fn av1_inv_txfm2d_add_16x32_neon(
    mut input: *const tran_low_t,
    mut dest: *mut u16,
    mut stride: i32,
    mut tx_type: TX_TYPE,
    mut bd: i32,
) {
    inv_txfm2d_add_universe_neon(input, dest as *mut u8, stride, tx_type, TX_16X32 as TX_SIZE, bd);
}

#[no_mangle]
pub unsafe extern "C-unwind" fn av1_inv_txfm2d_add_32x16_neon(
    mut input: *const tran_low_t,
    mut dest: *mut u16,
    mut stride: i32,
    mut tx_type: TX_TYPE,
    mut bd: i32,
) {
    inv_txfm2d_add_universe_neon(input, dest as *mut u8, stride, tx_type, TX_32X16 as TX_SIZE, bd);
}

#[no_mangle]
pub unsafe extern "C-unwind" fn av1_inv_txfm2d_add_32x32_neon(
    mut input: *const tran_low_t,
    mut dest: *mut u16,
    mut stride: i32,
    mut tx_type: TX_TYPE,
    mut bd: i32,
) {
    inv_txfm2d_add_universe_neon(input, dest as *mut u8, stride, tx_type, TX_32X32 as TX_SIZE, bd);
}

#[no_mangle]
pub unsafe extern "C-unwind" fn av1_inv_txfm2d_add_64x64_neon(
    mut input: *const tran_low_t,
    mut dest: *mut u16,
    mut stride: i32,
    mut tx_type: TX_TYPE,
    mut bd: i32,
) {
    inv_txfm2d_add_universe_neon(input, dest as *mut u8, stride, tx_type, TX_64X64 as TX_SIZE, bd);
}

#[no_mangle]
pub unsafe extern "C-unwind" fn av1_inv_txfm2d_add_32x64_neon(
    mut input: *const tran_low_t,
    mut dest: *mut u16,
    mut stride: i32,
    mut tx_type: TX_TYPE,
    mut bd: i32,
) {
    inv_txfm2d_add_universe_neon(input, dest as *mut u8, stride, tx_type, TX_32X64 as TX_SIZE, bd);
}

#[no_mangle]
pub unsafe extern "C-unwind" fn av1_inv_txfm2d_add_64x32_neon(
    mut input: *const tran_low_t,
    mut dest: *mut u16,
    mut stride: i32,
    mut tx_type: TX_TYPE,
    mut bd: i32,
) {
    inv_txfm2d_add_universe_neon(input, dest as *mut u8, stride, tx_type, TX_64X32 as TX_SIZE, bd);
}

#[no_mangle]
pub unsafe extern "C-unwind" fn av1_inv_txfm2d_add_64x16_neon(
    mut input: *const tran_low_t,
    mut dest: *mut u16,
    mut stride: i32,
    mut tx_type: TX_TYPE,
    mut bd: i32,
) {
    inv_txfm2d_add_universe_neon(input, dest as *mut u8, stride, tx_type, TX_64X16 as TX_SIZE, bd);
}

#[no_mangle]
pub unsafe extern "C-unwind" fn av1_inv_txfm2d_add_16x64_neon(
    mut input: *const tran_low_t,
    mut dest: *mut u16,
    mut stride: i32,
    mut tx_type: TX_TYPE,
    mut bd: i32,
) {
    inv_txfm2d_add_universe_neon(input, dest as *mut u8, stride, tx_type, TX_16X64 as TX_SIZE, bd);
}

unsafe fn av1_inv_txfm2d_add_16x16_neon(
    mut input: *const tran_low_t,
    mut dest: *mut u16,
    mut stride: i32,
    mut tx_type: TX_TYPE,
    mut bd: i32,
) {
    inv_txfm2d_add_universe_neon(input, dest as *mut u8, stride, tx_type, TX_16X16 as TX_SIZE, bd);
}

#[no_mangle]
pub unsafe extern "C-unwind" fn av1_inv_txfm2d_add_32x8_neon(
    mut input: *const tran_low_t,
    mut dest: *mut u16,
    mut stride: i32,
    mut tx_type: TX_TYPE,
    mut bd: i32,
) {
    inv_txfm2d_add_universe_neon(input, dest as *mut u8, stride, tx_type, TX_32X8 as TX_SIZE, bd);
}

#[no_mangle]
pub unsafe extern "C-unwind" fn av1_inv_txfm2d_add_8x32_neon(
    mut input: *const tran_low_t,
    mut dest: *mut u16,
    mut stride: i32,
    mut tx_type: TX_TYPE,
    mut bd: i32,
) {
    inv_txfm2d_add_universe_neon(input, dest as *mut u8, stride, tx_type, TX_8X32 as TX_SIZE, bd);
}

#[no_mangle]
pub unsafe extern "C-unwind" fn av1_highbd_inv_txfm_add_neon(
    mut input: *const tran_low_t,
    mut dest: *mut u8,
    mut stride: i32,
    mut txfm_param: *const TxfmParam,
) {
    let tx_size: TX_SIZE = (*txfm_param).tx_size;

    let mut tx_type: TX_TYPE = (*txfm_param).tx_type;
    let mut bd: i32 = (*txfm_param).bd;
    match tx_size as i32 {
        TX_8X8 => {
            highbd_inv_txfm_add_8x8_neon(input, dest, stride, txfm_param);
        }
        TX_4X8 => {
            av1_inv_txfm2d_add_4x8_neon(
                input,
                CONVERT_TO_SHORTPTR(dest),
                stride,
                (*txfm_param).tx_type,
                (*txfm_param).bd,
            );
        }
        TX_8X4 => {
            av1_inv_txfm2d_add_8x4_neon(
                input,
                CONVERT_TO_SHORTPTR(dest),
                stride,
                (*txfm_param).tx_type,
                (*txfm_param).bd,
            );
        }
        TX_4X4 => {
            highbd_inv_txfm_add_4x4_neon(input, dest, stride, txfm_param);
        }
        TX_16X4 => {
            av1_inv_txfm2d_add_16x4_neon(
                input,
                CONVERT_TO_SHORTPTR(dest),
                stride,
                (*txfm_param).tx_type,
                (*txfm_param).bd,
            );
        }
        TX_4X16 => {
            av1_inv_txfm2d_add_4x16_neon(
                input,
                CONVERT_TO_SHORTPTR(dest),
                stride,
                (*txfm_param).tx_type,
                (*txfm_param).bd,
            );
        }
        TX_8X16 => {
            av1_inv_txfm2d_add_8x16_neon(input, dest as *mut u16, stride, tx_type, bd);
        }
        TX_16X8 => {
            av1_inv_txfm2d_add_16x8_neon(input, dest as *mut u16, stride, tx_type, bd);
        }
        TX_16X32 => {
            av1_inv_txfm2d_add_16x32_neon(input, dest as *mut u16, stride, tx_type, bd);
        }
        TX_32X16 => {
            av1_inv_txfm2d_add_32x16_neon(input, dest as *mut u16, stride, tx_type, bd);
        }
        TX_16X16 => {
            av1_inv_txfm2d_add_16x16_neon(input, dest as *mut u16, stride, tx_type, bd);
        }
        TX_32X32 => {
            av1_inv_txfm2d_add_32x32_neon(input, dest as *mut u16, stride, tx_type, bd);
        }
        TX_64X64 => {
            av1_inv_txfm2d_add_64x64_neon(input, dest as *mut u16, stride, tx_type, bd);
        }
        TX_32X64 => {
            av1_inv_txfm2d_add_32x64_neon(input, dest as *mut u16, stride, tx_type, bd);
        }
        TX_64X32 => {
            av1_inv_txfm2d_add_64x32_neon(input, dest as *mut u16, stride, tx_type, bd);
        }
        TX_16X64 => {
            av1_inv_txfm2d_add_16x64_neon(input, dest as *mut u16, stride, tx_type, bd);
        }
        TX_64X16 => {
            av1_inv_txfm2d_add_64x16_neon(input, dest as *mut u16, stride, tx_type, bd);
        }
        TX_32X8 => {
            av1_inv_txfm2d_add_32x8_neon(input, dest as *mut u16, stride, tx_type, bd);
        }
        TX_8X32 => {
            av1_inv_txfm2d_add_8x32_neon(input, dest as *mut u16, stride, tx_type, bd);
        }
        _ => {}
    }
}
