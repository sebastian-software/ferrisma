/*
 * Copyright (c) 2020, Alliance for Open Media. All rights reserved.
 *
 * This source code is subject to the terms of the BSD 2 Clause License and
 * the Alliance for Open Media Patent License 1.0. If the BSD 2 Clause License
 * was not distributed with this source code in the LICENSE file, you can
 * obtain it at www.aomedia.org/license/software. If the Alliance for Open
 * Media Patent License 1.0 was not distributed with this source code in the
 * PATENTS file, you can obtain it at www.aomedia.org/license/patent.
 */

// Translated by hand from upstream/aom/av1/encoder/arm/highbd_fwd_txfm_neon.c
// (ADR 0056). Intrinsic sequences, helper structure, and names follow the
// original. C arrays become stack arrays that are only accessed through a raw
// pointer to their first element (`a[i]` -> `*a.add(i)`, `&a[i]` -> `a.add(i)`,
// `a + i` -> `a.add(i)`), so in-place calls keep the C aliasing. Function
// generating macros are expanded with explicit identifiers. Local arrays are
// zero-initialized; the C code never reads an element before writing it.
// CONFIG_REALTIME_ONLY is 0 in the translated configuration, so the
// `#if !CONFIG_REALTIME_ONLY` parts are included.
#![allow(clippy::all)]

use core::arch::aarch64::*;

use crate::av1__common__av1_txfm::av1_sinpi_arr_data;
use crate::av1__encoder__av1_fwd_txfm2d::{av1_fwd_cos_bit_col, av1_fwd_cos_bit_row};
use crate::simd::aom_dsp__arm__transpose_neon::*;
use crate::simd::av1__common__av1_txfm::cospi_arr_s32;
use crate::simd::av1__encoder__arm__shift_neon::*;
use crate::simd::av1__encoder__arm__txfm_neon::ud_adjust_input_and_stride;

// From aom_dsp/txfm_common.h.
pub type TX_TYPE = u8;
const DCT_DCT: TX_TYPE = 0;
const ADST_DCT: TX_TYPE = 1;
const DCT_ADST: TX_TYPE = 2;
const ADST_ADST: TX_TYPE = 3;
const FLIPADST_DCT: TX_TYPE = 4;
const DCT_FLIPADST: TX_TYPE = 5;
const FLIPADST_FLIPADST: TX_TYPE = 6;
const ADST_FLIPADST: TX_TYPE = 7;
const FLIPADST_ADST: TX_TYPE = 8;
const IDTX: TX_TYPE = 9;
const V_DCT: TX_TYPE = 10;
const H_DCT: TX_TYPE = 11;
const V_ADST: TX_TYPE = 12;
const H_ADST: TX_TYPE = 13;
const V_FLIPADST: TX_TYPE = 14;
const H_FLIPADST: TX_TYPE = 15;
const TX_TYPES: usize = 16;

// From av1/common/av1_txfm.h.
const cos_bit_min: i32 = 10;
const NewSqrt2Bits: i32 = 12;
// 2^12 * sqrt(2)
const NewSqrt2: i32 = 5793;

// From av1/common/av1_txfm.h.
#[inline(always)]
unsafe fn sinpi_arr(n: i32) -> *const i32 {
    (&raw const av1_sinpi_arr_data[(n - cos_bit_min) as usize]).cast::<i32>()
}

// From av1/common/av1_txfm.h.
#[inline(always)]
unsafe fn get_flip_cfg(tx_type: TX_TYPE, ud_flip: *mut i32, lr_flip: *mut i32) {
    match tx_type {
        DCT_DCT | ADST_DCT | DCT_ADST | ADST_ADST => {
            *ud_flip = 0;
            *lr_flip = 0;
        }
        IDTX | V_DCT | H_DCT | V_ADST | H_ADST => {
            *ud_flip = 0;
            *lr_flip = 0;
        }
        FLIPADST_DCT | FLIPADST_ADST | V_FLIPADST => {
            *ud_flip = 1;
            *lr_flip = 0;
        }
        DCT_FLIPADST | ADST_FLIPADST | H_FLIPADST => {
            *ud_flip = 0;
            *lr_flip = 1;
        }
        FLIPADST_FLIPADST => {
            *ud_flip = 1;
            *lr_flip = 1;
        }
        _ => {
            *ud_flip = 0;
            *lr_flip = 0;
            debug_assert!(false);
        }
    }
}

#[inline(always)]
unsafe fn transpose_arrays_s32_64x64(in_: *const int32x4_t, out: *mut int32x4_t) {
    // This is not quite the same as the other transposes defined in
    // transpose_neon.h: We only write the low 64x32 sub-matrix since the rest is
    // unused by the following row transform.
    for j in 0..8 {
        for i in 0..16 {
            transpose_arrays_s32_4x4(in_.add(64 * i + 4 * j), out.add(64 * j + 4 * i));
        }
    }
}

// A note on butterfly helper naming:
//
// butterfly_[weight_indices]_neon
// e.g. butterfly_0312_neon
//                ^ Weights are applied as indices 0, 3, 2, 1
//                  (see more detail below)
//
// Weight indices are treated as an index into the 4-tuple of the weight
// itself, plus related and negated constants: w=(w0, 1-w0, -w0, w0-1).
// This is then represented in the helper naming by referring to the lane index
// in the loaded tuple that each multiply is performed with:
//
//         in0   in1
//      /------------
// out0 |  w[0]  w[1]   ==>  out0 = in0 * w[0] + in1 * w[1]
// out1 |  w[2]  w[3]   ==>  out1 = in0 * w[2] + in1 * w[3]
//
// So for indices 0321 from the earlier example, we end up with:
//
//          in0       in1
//      /------------------
// out0 | (lane 0) (lane 3)   ==>  out0 = in0 *  w0 + in1 * (w0-1)
// out1 | (lane 2) (lane 1)   ==>  out1 = in0 * -w0 + in1 * (1-w0)

macro_rules! butterfly_half_neon {
    ($wvec:expr, $lane0:literal, $lane1:literal, $in0:expr, $in1:expr, $out:expr, $v_bit:expr) => {{
        let wvecs = int32x2x2_t($wvec, vneg_s32($wvec));
        let mut x = vmulq_lane_s32::<{ $lane0 % 2 }>(
            $in0, if $lane0 / 2 == 0 { wvecs.0 } else { wvecs.1 });
        x = vmlaq_lane_s32::<{ $lane1 % 2 }>(
            x, $in1, if $lane1 / 2 == 0 { wvecs.0 } else { wvecs.1 });
        *$out = vrshlq_s32(x, $v_bit);
    }};
}

#[inline(always)]
unsafe fn butterfly_0112_neon(cospi: *const i32, widx0: i32, n0: int32x4_t, n1: int32x4_t,
                              out0: *mut int32x4_t, out1: *mut int32x4_t, v_bit: int32x4_t) {
    let w01 = vld1_s32(cospi.offset((2 * widx0) as isize));
    butterfly_half_neon!(w01, 0, 1, n0, n1, out0, v_bit);
    butterfly_half_neon!(w01, 1, 2, n0, n1, out1, v_bit);
}

#[inline(always)]
unsafe fn butterfly_2312_neon(cospi: *const i32, widx0: i32, n0: int32x4_t, n1: int32x4_t,
                              out0: *mut int32x4_t, out1: *mut int32x4_t, v_bit: int32x4_t) {
    let w01 = vld1_s32(cospi.offset((2 * widx0) as isize));
    butterfly_half_neon!(w01, 2, 3, n0, n1, out0, v_bit);
    butterfly_half_neon!(w01, 1, 2, n0, n1, out1, v_bit);
}

#[inline(always)]
unsafe fn butterfly_0332_neon(cospi: *const i32, widx0: i32, n0: int32x4_t, n1: int32x4_t,
                              out0: *mut int32x4_t, out1: *mut int32x4_t, v_bit: int32x4_t) {
    let w01 = vld1_s32(cospi.offset((2 * widx0) as isize));
    butterfly_half_neon!(w01, 0, 3, n0, n1, out0, v_bit);
    butterfly_half_neon!(w01, 3, 2, n0, n1, out1, v_bit);
}

#[inline(always)]
unsafe fn butterfly_0130_neon(cospi: *const i32, widx0: i32, n0: int32x4_t, n1: int32x4_t,
                              out0: *mut int32x4_t, out1: *mut int32x4_t, v_bit: int32x4_t) {
    let w01 = vld1_s32(cospi.offset((2 * widx0) as isize));
    butterfly_half_neon!(w01, 0, 1, n0, n1, out0, v_bit);
    butterfly_half_neon!(w01, 3, 0, n0, n1, out1, v_bit);
}

#[inline(always)]
unsafe fn butterfly_cospi32_0002_neon(cospi: *const i32, n0: int32x4_t, n1: int32x4_t,
                                      out0: *mut int32x4_t, out1: *mut int32x4_t,
                                      v_bit: int32x4_t) {
    let w01 = vld1_s32(cospi.add(2 * 32));
    butterfly_half_neon!(w01, 0, 0, n0, n1, out0, v_bit);
    butterfly_half_neon!(w01, 0, 2, n0, n1, out1, v_bit);
}

#[inline(always)]
unsafe fn butterfly_cospi32_0222_neon(cospi: *const i32, n0: int32x4_t, n1: int32x4_t,
                                      out0: *mut int32x4_t, out1: *mut int32x4_t,
                                      v_bit: int32x4_t) {
    let w01 = vld1_s32(cospi.add(2 * 32));
    butterfly_half_neon!(w01, 0, 2, n0, n1, out0, v_bit);
    butterfly_half_neon!(w01, 2, 2, n0, n1, out1, v_bit);
}

#[inline(always)]
unsafe fn round_rect_array_s32_neon(input: *const int32x4_t, output: *mut int32x4_t, size: i32) {
    let sqrt2 = vdupq_n_s32(NewSqrt2);
    let mut i: i32 = 0;
    loop {
        let r1 = vmulq_s32(*input.offset(i as isize), sqrt2);
        *output.offset(i as isize) = vrshrq_n_s32::<NewSqrt2Bits>(r1);
        i += 1;
        if !(i < size) {
            break;
        }
    }
}

#[inline(always)]
unsafe fn round_shift2_rect_array_s32_neon(input: *const int32x4_t, output: *mut int32x4_t,
                                           size: i32) {
    let sqrt2 = vdupq_n_s32(NewSqrt2);
    let mut i: i32 = 0;
    loop {
        let r0 = vrshrq_n_s32::<2>(*input.offset(i as isize));
        let r1 = vmulq_s32(r0, sqrt2);
        *output.offset(i as isize) = vrshrq_n_s32::<NewSqrt2Bits>(r1);
        i += 1;
        if !(i < size) {
            break;
        }
    }
}

macro_rules! LOAD_BUFFER_4XH {
    ($name:ident, $h:literal) => {
        #[inline(always)]
        unsafe fn $name(input: *const i16, in_: *mut int32x4_t, stride: i32, fliplr: i32) {
            if fliplr != 0 {
                for i in 0..($h) {
                    let mut a = vld1_s16(input.offset((i * stride) as isize));
                    a = vrev64_s16(a);
                    *in_.offset(i as isize) = vshll_n_s16::<2>(a);
                }
            } else {
                for i in 0..($h) {
                    let a = vld1_s16(input.offset((i * stride) as isize));
                    *in_.offset(i as isize) = vshll_n_s16::<2>(a);
                }
            }
        }
    };
}

// AArch32 does not permit the argument to vshll_n_s16 to be zero, so need to
// avoid the expression even though the compiler can prove that the code path
// is never taken if `shift == 0`.
macro_rules! shift_left_long_s16 {
    ($a:expr, 0) => {
        vmovl_s16($a)
    };
    ($a:expr, $shift:literal) => {
        vshll_n_s16::<$shift>($a)
    };
}

macro_rules! LOAD_BUFFER_WXH {
    ($name:ident, $w:literal, $h:literal, $shift:tt) => {
        #[inline(always)]
        unsafe fn $name(input: *const i16, in_: *mut int32x4_t, stride: i32, fliplr: i32) {
            debug_assert!($w >= 8);
            if fliplr != 0 {
                for i in 0..($h) {
                    for j in 0..($w) / 8 {
                        let mut a = vld1q_s16(input.offset((i * stride + j * 8) as isize));
                        a = vrev64q_s16(a);
                        let j2 = ($w) / 8 - j - 1;
                        *in_.offset((i + ($h) * (2 * j2 + 0)) as isize) =
                            shift_left_long_s16!(vget_high_s16(a), $shift);
                        *in_.offset((i + ($h) * (2 * j2 + 1)) as isize) =
                            shift_left_long_s16!(vget_low_s16(a), $shift);
                    }
                }
            } else {
                for i in 0..($h) {
                    for j in 0..($w) / 8 {
                        let a = vld1q_s16(input.offset((i * stride + j * 8) as isize));
                        *in_.offset((i + ($h) * (2 * j + 0)) as isize) =
                            shift_left_long_s16!(vget_low_s16(a), $shift);
                        *in_.offset((i + ($h) * (2 * j + 1)) as isize) =
                            shift_left_long_s16!(vget_high_s16(a), $shift);
                    }
                }
            }
        }
    };
}

LOAD_BUFFER_4XH!(load_buffer_4x4, 4);
LOAD_BUFFER_4XH!(load_buffer_4x8, 8);
LOAD_BUFFER_4XH!(load_buffer_4x16, 16);
LOAD_BUFFER_4XH!(load_buffer_4x32, 32);
LOAD_BUFFER_WXH!(load_buffer_8x8, 8, 8, 2);
LOAD_BUFFER_WXH!(load_buffer_16x16, 16, 16, 2);
LOAD_BUFFER_WXH!(load_buffer_32x64, 32, 64, 0);
LOAD_BUFFER_WXH!(load_buffer_64x32, 64, 32, 2);
LOAD_BUFFER_WXH!(load_buffer_64x64, 64, 64, 0);

// !CONFIG_REALTIME_ONLY
LOAD_BUFFER_WXH!(load_buffer_16x64, 16, 64, 0);
LOAD_BUFFER_WXH!(load_buffer_64x16, 64, 16, 2);

macro_rules! STORE_BUFFER_WXH {
    ($name:ident, $w:literal, $h:literal) => {
        #[inline(always)]
        unsafe fn $name(in_: *const int32x4_t, out: *mut i32, stride: i32) {
            for i in 0..($w) {
                for j in 0..($h) / 4 {
                    vst1q_s32(out.offset((i * stride + j * 4) as isize),
                              *in_.offset((i + j * ($w)) as isize));
                }
            }
        }
    };
}

STORE_BUFFER_WXH!(store_buffer_4x4, 4, 4);
STORE_BUFFER_WXH!(store_buffer_8x4, 8, 4);
STORE_BUFFER_WXH!(store_buffer_8x8, 8, 8);
STORE_BUFFER_WXH!(store_buffer_16x4, 16, 4);
STORE_BUFFER_WXH!(store_buffer_16x16, 16, 16);
STORE_BUFFER_WXH!(store_buffer_32x4, 32, 4);
STORE_BUFFER_WXH!(store_buffer_32x32, 32, 32);
STORE_BUFFER_WXH!(store_buffer_64x32, 64, 32);

// !CONFIG_REALTIME_ONLY
STORE_BUFFER_WXH!(store_buffer_16x32, 16, 32);
STORE_BUFFER_WXH!(store_buffer_64x16, 64, 16);

#[inline(always)]
unsafe fn highbd_fdct4_x4_neon(in_: *const int32x4_t, out: *mut int32x4_t, bit: i32) {
    let cospi = cospi_arr_s32(bit);
    let cospi32 = vdupq_n_s32(*cospi.add(2 * 32));
    let cospi16_48 = vld1_s32(cospi.add(2 * 16));

    let a0 = vaddq_s32(*in_.add(0), *in_.add(3));
    let a1 = vsubq_s32(*in_.add(0), *in_.add(3));
    let a2 = vaddq_s32(*in_.add(1), *in_.add(2));
    let a3 = vsubq_s32(*in_.add(1), *in_.add(2));

    let b0 = vmulq_s32(a0, cospi32);
    let b1 = vmulq_lane_s32::<1>(a1, cospi16_48);
    let b2 = vmulq_s32(a2, cospi32);
    let b3 = vmulq_lane_s32::<1>(a3, cospi16_48);

    let c0 = vaddq_s32(b0, b2);
    let c1 = vsubq_s32(b0, b2);
    let c2 = vmlaq_lane_s32::<0>(b3, a1, cospi16_48);
    let c3 = vmlsq_lane_s32::<0>(b1, a3, cospi16_48);

    let v_bit = vdupq_n_s32(-bit);
    let d0 = vrshlq_s32(c0, v_bit);
    let d1 = vrshlq_s32(c1, v_bit);
    let d2 = vrshlq_s32(c2, v_bit);
    let d3 = vrshlq_s32(c3, v_bit);

    *out.add(0) = d0;
    *out.add(1) = d2;
    *out.add(2) = d1;
    *out.add(3) = d3;
}

#[inline(always)]
unsafe fn highbd_fadst4_x4_neon(in_: *const int32x4_t, out: *mut int32x4_t, bit: i32) {
    let sinpi = vld1q_s32(sinpi_arr(bit).add(1));

    let a0 = vaddq_s32(*in_.add(0), *in_.add(1));
    let a1 = vmulq_lane_s32::<0>(*in_.add(0), vget_low_s32(sinpi));
    let a2 = vmulq_lane_s32::<1>(*in_.add(0), vget_high_s32(sinpi));
    let a3 = vmulq_lane_s32::<0>(*in_.add(2), vget_high_s32(sinpi));

    let b0 = vmlaq_lane_s32::<1>(a1, *in_.add(1), vget_low_s32(sinpi));
    let b1 = vmlsq_lane_s32::<0>(a2, *in_.add(1), vget_low_s32(sinpi));
    let b2 = vsubq_s32(a0, *in_.add(3));

    let c0 = vmlaq_lane_s32::<1>(b0, *in_.add(3), vget_high_s32(sinpi));
    let c1 = vmlaq_lane_s32::<1>(b1, *in_.add(3), vget_low_s32(sinpi));
    let c2 = vmulq_lane_s32::<0>(b2, vget_high_s32(sinpi));

    let d0 = vaddq_s32(c0, a3);
    let d1 = vsubq_s32(c1, a3);
    let d2 = vsubq_s32(c1, c0);

    let e0 = vaddq_s32(d2, a3);

    let v_bit = vdupq_n_s32(-bit);
    *out.add(0) = vrshlq_s32(d0, v_bit);
    *out.add(1) = vrshlq_s32(c2, v_bit);
    *out.add(2) = vrshlq_s32(d1, v_bit);
    *out.add(3) = vrshlq_s32(e0, v_bit);
}

#[inline(always)]
unsafe fn highbd_fidentity4_x4_neon(in_: *const int32x4_t, out: *mut int32x4_t, bit: i32) {
    let _ = bit;
    let fact = vdupq_n_s32(NewSqrt2);

    for i in 0..4 {
        let a_low = vmulq_s32(*in_.add(i), fact);
        *out.add(i) = vrshrq_n_s32::<NewSqrt2Bits>(a_low);
    }
}

#[no_mangle]
pub unsafe extern "C-unwind" fn av1_fwd_txfm2d_4x4_neon(mut input: *const i16, coeff: *mut i32,
                                                        mut input_stride: i32, tx_type: TX_TYPE,
                                                        bd: i32) {
    let _ = bd;

    let mut ud_flip: i32 = 0;
    let mut lr_flip: i32 = 0;
    get_flip_cfg(tx_type, &mut ud_flip, &mut lr_flip);
    ud_adjust_input_and_stride(ud_flip, &mut input, &mut input_stride, 4);

    // Workspace for column/row-wise transforms.
    let mut buf_: [int32x4_t; 4] = core::mem::zeroed();
    let buf = buf_.as_mut_ptr();

    match tx_type {
        DCT_DCT => {
            load_buffer_4x4(input, buf, input_stride, 0);
            highbd_fdct4_x4_neon(buf, buf, av1_fwd_cos_bit_col[0][0] as i32);
            transpose_arrays_s32_4x4(buf, buf);
            highbd_fdct4_x4_neon(buf, buf, av1_fwd_cos_bit_row[0][0] as i32);
            store_buffer_4x4(buf, coeff, /*stride=*/4);
        }
        ADST_DCT => {
            load_buffer_4x4(input, buf, input_stride, 0);
            highbd_fadst4_x4_neon(buf, buf, av1_fwd_cos_bit_col[0][0] as i32);
            transpose_arrays_s32_4x4(buf, buf);
            highbd_fdct4_x4_neon(buf, buf, av1_fwd_cos_bit_row[0][0] as i32);
            store_buffer_4x4(buf, coeff, /*stride=*/4);
        }
        DCT_ADST => {
            load_buffer_4x4(input, buf, input_stride, 0);
            highbd_fdct4_x4_neon(buf, buf, av1_fwd_cos_bit_col[0][0] as i32);
            transpose_arrays_s32_4x4(buf, buf);
            highbd_fadst4_x4_neon(buf, buf, av1_fwd_cos_bit_row[0][0] as i32);
            store_buffer_4x4(buf, coeff, /*stride=*/4);
        }
        ADST_ADST => {
            load_buffer_4x4(input, buf, input_stride, 0);
            highbd_fadst4_x4_neon(buf, buf, av1_fwd_cos_bit_col[0][0] as i32);
            transpose_arrays_s32_4x4(buf, buf);
            highbd_fadst4_x4_neon(buf, buf, av1_fwd_cos_bit_row[0][0] as i32);
            store_buffer_4x4(buf, coeff, /*stride=*/4);
        }
        FLIPADST_DCT => {
            load_buffer_4x4(input, buf, input_stride, 0);
            highbd_fadst4_x4_neon(buf, buf, av1_fwd_cos_bit_col[0][0] as i32);
            transpose_arrays_s32_4x4(buf, buf);
            highbd_fdct4_x4_neon(buf, buf, av1_fwd_cos_bit_row[0][0] as i32);
            store_buffer_4x4(buf, coeff, /*stride=*/4);
        }
        DCT_FLIPADST => {
            load_buffer_4x4(input, buf, input_stride, 1);
            highbd_fdct4_x4_neon(buf, buf, av1_fwd_cos_bit_col[0][0] as i32);
            transpose_arrays_s32_4x4(buf, buf);
            highbd_fadst4_x4_neon(buf, buf, av1_fwd_cos_bit_row[0][0] as i32);
            store_buffer_4x4(buf, coeff, /*stride=*/4);
        }
        FLIPADST_FLIPADST => {
            load_buffer_4x4(input, buf, input_stride, 1);
            highbd_fadst4_x4_neon(buf, buf, av1_fwd_cos_bit_col[0][0] as i32);
            transpose_arrays_s32_4x4(buf, buf);
            highbd_fadst4_x4_neon(buf, buf, av1_fwd_cos_bit_row[0][0] as i32);
            store_buffer_4x4(buf, coeff, /*stride=*/4);
        }
        ADST_FLIPADST => {
            load_buffer_4x4(input, buf, input_stride, 1);
            highbd_fadst4_x4_neon(buf, buf, av1_fwd_cos_bit_col[0][0] as i32);
            transpose_arrays_s32_4x4(buf, buf);
            highbd_fadst4_x4_neon(buf, buf, av1_fwd_cos_bit_row[0][0] as i32);
            store_buffer_4x4(buf, coeff, /*stride=*/4);
        }
        FLIPADST_ADST => {
            load_buffer_4x4(input, buf, input_stride, 0);
            highbd_fadst4_x4_neon(buf, buf, av1_fwd_cos_bit_col[0][0] as i32);
            transpose_arrays_s32_4x4(buf, buf);
            highbd_fadst4_x4_neon(buf, buf, av1_fwd_cos_bit_row[0][0] as i32);
            store_buffer_4x4(buf, coeff, /*stride=*/4);
        }
        IDTX => {
            load_buffer_4x4(input, buf, input_stride, 0);
            highbd_fidentity4_x4_neon(buf, buf, av1_fwd_cos_bit_col[0][0] as i32);
            transpose_arrays_s32_4x4(buf, buf);
            highbd_fidentity4_x4_neon(buf, buf, av1_fwd_cos_bit_row[0][0] as i32);
            store_buffer_4x4(buf, coeff, /*stride=*/4);
        }
        V_DCT => {
            load_buffer_4x4(input, buf, input_stride, 0);
            highbd_fdct4_x4_neon(buf, buf, av1_fwd_cos_bit_col[0][0] as i32);
            transpose_arrays_s32_4x4(buf, buf);
            highbd_fidentity4_x4_neon(buf, buf, av1_fwd_cos_bit_row[0][0] as i32);
            store_buffer_4x4(buf, coeff, /*stride=*/4);
        }
        H_DCT => {
            load_buffer_4x4(input, buf, input_stride, 0);
            highbd_fidentity4_x4_neon(buf, buf, av1_fwd_cos_bit_row[0][0] as i32);
            transpose_arrays_s32_4x4(buf, buf);
            highbd_fdct4_x4_neon(buf, buf, av1_fwd_cos_bit_col[0][0] as i32);
            store_buffer_4x4(buf, coeff, /*stride=*/4);
        }
        V_ADST => {
            load_buffer_4x4(input, buf, input_stride, 0);
            highbd_fadst4_x4_neon(buf, buf, av1_fwd_cos_bit_col[0][0] as i32);
            transpose_arrays_s32_4x4(buf, buf);
            highbd_fidentity4_x4_neon(buf, buf, av1_fwd_cos_bit_row[0][0] as i32);
            store_buffer_4x4(buf, coeff, /*stride=*/4);
        }
        H_ADST => {
            load_buffer_4x4(input, buf, input_stride, 0);
            highbd_fidentity4_x4_neon(buf, buf, av1_fwd_cos_bit_row[0][0] as i32);
            transpose_arrays_s32_4x4(buf, buf);
            highbd_fadst4_x4_neon(buf, buf, av1_fwd_cos_bit_col[0][0] as i32);
            store_buffer_4x4(buf, coeff, /*stride=*/4);
        }
        V_FLIPADST => {
            load_buffer_4x4(input, buf, input_stride, 0);
            highbd_fadst4_x4_neon(buf, buf, av1_fwd_cos_bit_row[0][0] as i32);
            transpose_arrays_s32_4x4(buf, buf);
            highbd_fidentity4_x4_neon(buf, buf, av1_fwd_cos_bit_row[0][0] as i32);
            store_buffer_4x4(buf, coeff, /*stride=*/4);
        }
        H_FLIPADST => {
            load_buffer_4x4(input, buf, input_stride, 1);
            highbd_fidentity4_x4_neon(buf, buf, av1_fwd_cos_bit_row[0][0] as i32);
            transpose_arrays_s32_4x4(buf, buf);
            highbd_fadst4_x4_neon(buf, buf, av1_fwd_cos_bit_row[0][0] as i32);
            store_buffer_4x4(buf, coeff, /*stride=*/4);
        }
        _ => debug_assert!(false),
    }
}

// Butterfly pre-processing:
// e.g. n=4:
//   out[0] = in[0] + in[3]
//   out[1] = in[1] + in[2]
//   out[2] = in[1] - in[2]
//   out[3] = in[0] - in[3]

#[inline(always)]
unsafe fn butterfly_dct_pre(input: *const int32x4_t, output: *mut int32x4_t, n: i32) {
    for i in 0..n / 2 {
        *output.offset(i as isize) =
            vaddq_s32(*input.offset(i as isize), *input.offset((n - i - 1) as isize));
    }
    for i in 0..n / 2 {
        *output.offset((n / 2 + i) as isize) =
            vsubq_s32(*input.offset((n / 2 - i - 1) as isize), *input.offset((n / 2 + i) as isize));
    }
}

// Butterfly post-processing:
// e.g. n=8:
//   out[0] = in0[0] + in1[3];
//   out[1] = in0[1] + in1[2];
//   out[2] = in0[1] - in1[2];
//   out[3] = in0[0] - in1[3];
//   out[4] = in0[7] - in1[4];
//   out[5] = in0[6] - in1[5];
//   out[6] = in0[6] + in1[5];
//   out[7] = in0[7] + in1[4];

#[inline(always)]
unsafe fn butterfly_dct_post(in0: *const int32x4_t, in1: *const int32x4_t,
                             output: *mut int32x4_t, n: i32) {
    for i in 0..n / 4 {
        *output.offset(i as isize) =
            vaddq_s32(*in0.offset(i as isize), *in1.offset((n / 2 - i - 1) as isize));
    }
    for i in 0..n / 4 {
        *output.offset((n / 4 + i) as isize) =
            vsubq_s32(*in0.offset((n / 4 - i - 1) as isize), *in1.offset((n / 4 + i) as isize));
    }
    for i in 0..n / 4 {
        *output.offset((n / 2 + i) as isize) =
            vsubq_s32(*in0.offset((n - i - 1) as isize), *in1.offset((n / 2 + i) as isize));
    }
    for i in 0..n / 4 {
        *output.offset(((3 * n) / 4 + i) as isize) =
            vaddq_s32(*in0.offset(((3 * n) / 4 + i) as isize),
                      *in1.offset(((3 * n) / 4 - i - 1) as isize));
    }
}

#[inline(always)]
unsafe fn highbd_fdct8_x4_neon(in_: *const int32x4_t, out: *mut int32x4_t, bit: i32) {
    let cospi = cospi_arr_s32(bit);
    let v_bit = vdupq_n_s32(-bit);

    // stage 1
    let mut a_: [int32x4_t; 8] = core::mem::zeroed();
    let a = a_.as_mut_ptr();
    butterfly_dct_pre(in_, a, 8);

    // stage 2
    let mut b_: [int32x4_t; 8] = core::mem::zeroed();
    let b = b_.as_mut_ptr();
    butterfly_dct_pre(a, b, 4);
    butterfly_0130_neon(cospi, 32, *a.add(5), *a.add(6), b.add(6), b.add(5), v_bit);

    // stage 3
    let mut c_: [int32x4_t; 8] = core::mem::zeroed();
    let c = c_.as_mut_ptr();
    butterfly_0130_neon(cospi, 32, *b.add(1), *b.add(0), c.add(0), c.add(1), v_bit);
    butterfly_0112_neon(cospi, 16, *b.add(3), *b.add(2), c.add(2), c.add(3), v_bit);
    butterfly_dct_post(a.add(4), b.add(4), c.add(4), 4);

    // stage 4-5
    butterfly_0112_neon(cospi, 8, *c.add(7), *c.add(4), out.add(1), out.add(7), v_bit);
    butterfly_0130_neon(cospi, 24, *c.add(5), *c.add(6), out.add(5), out.add(3), v_bit);

    *out.add(0) = *c.add(0);
    *out.add(2) = *c.add(2);
    *out.add(4) = *c.add(1);
    *out.add(6) = *c.add(3);
}

#[inline(always)]
unsafe fn highbd_fadst8_x4_neon(in_: *const int32x4_t, out: *mut int32x4_t, bit: i32) {
    let cospi = cospi_arr_s32(bit);
    let v_bit = vdupq_n_s32(-bit);

    let mut u0: int32x4_t = vdupq_n_s32(0);
    let mut u1: int32x4_t = vdupq_n_s32(0);
    let mut u2: int32x4_t = vdupq_n_s32(0);
    let mut u3: int32x4_t = vdupq_n_s32(0);
    let mut u4: int32x4_t = vdupq_n_s32(0);
    let mut u5: int32x4_t = vdupq_n_s32(0);
    let mut u6: int32x4_t = vdupq_n_s32(0);
    let mut u7: int32x4_t = vdupq_n_s32(0);
    let mut v0: int32x4_t = vdupq_n_s32(0);
    let mut v1: int32x4_t = vdupq_n_s32(0);
    let mut v2: int32x4_t = vdupq_n_s32(0);
    let mut v3: int32x4_t = vdupq_n_s32(0);
    let mut v4: int32x4_t = vdupq_n_s32(0);
    let mut v5: int32x4_t = vdupq_n_s32(0);
    let mut v6: int32x4_t = vdupq_n_s32(0);
    let mut v7: int32x4_t = vdupq_n_s32(0);

    // stage 0-1
    u0 = *in_.add(0);
    u1 = *in_.add(7);
    u2 = *in_.add(3);
    u3 = *in_.add(4);
    u4 = *in_.add(1);
    u5 = *in_.add(6);
    u6 = *in_.add(2);
    u7 = *in_.add(5);

    // stage 2
    v0 = u0;
    v1 = u1;
    butterfly_cospi32_0222_neon(cospi, u3, u2, &mut v2, &mut v3, v_bit);
    v4 = u4;
    v5 = u5;
    butterfly_cospi32_0002_neon(cospi, u6, u7, &mut v7, &mut v6, v_bit);

    // stage 3
    u0 = vaddq_s32(v0, v2);
    u1 = vsubq_s32(v3, v1);
    u2 = vsubq_s32(v0, v2);
    u3 = vaddq_s32(v1, v3);
    u4 = vsubq_s32(v6, v4);
    u5 = vaddq_s32(v5, v7);
    u6 = vaddq_s32(v4, v6);
    u7 = vsubq_s32(v5, v7);

    // stage 4
    v0 = u0;
    v1 = u1;
    v2 = u2;
    v3 = u3;

    butterfly_0112_neon(cospi, 16, u4, u5, &mut v4, &mut v5, v_bit);
    butterfly_0112_neon(cospi, 16, u7, u6, &mut v6, &mut v7, v_bit);

    // stage 5
    u0 = vaddq_s32(v0, v4);
    u1 = vaddq_s32(v1, v5);
    u2 = vaddq_s32(v2, v6);
    u3 = vsubq_s32(v7, v3);
    u4 = vsubq_s32(v0, v4);
    u5 = vsubq_s32(v1, v5);
    u6 = vsubq_s32(v2, v6);
    u7 = vaddq_s32(v3, v7);

    // stage 6
    butterfly_0112_neon(cospi, 4, u0, u1, &mut v0, &mut v1, v_bit);
    butterfly_0112_neon(cospi, 20, u2, u3, &mut v2, &mut v3, v_bit);
    butterfly_0130_neon(cospi, 28, u5, u4, &mut v4, &mut v5, v_bit);
    butterfly_0112_neon(cospi, 12, u6, u7, &mut v7, &mut v6, v_bit);

    // stage 7
    *out.add(0) = v1;
    *out.add(1) = v6;
    *out.add(2) = v3;
    *out.add(3) = v4;
    *out.add(4) = v5;
    *out.add(5) = v2;
    *out.add(6) = v7;
    *out.add(7) = v0;
}

#[inline(always)]
unsafe fn highbd_fidentity8_x4_neon(in_: *const int32x4_t, out: *mut int32x4_t, bit: i32) {
    let _ = bit;
    *out.add(0) = vshlq_n_s32::<1>(*in_.add(0));
    *out.add(1) = vshlq_n_s32::<1>(*in_.add(1));
    *out.add(2) = vshlq_n_s32::<1>(*in_.add(2));
    *out.add(3) = vshlq_n_s32::<1>(*in_.add(3));
    *out.add(4) = vshlq_n_s32::<1>(*in_.add(4));
    *out.add(5) = vshlq_n_s32::<1>(*in_.add(5));
    *out.add(6) = vshlq_n_s32::<1>(*in_.add(6));
    *out.add(7) = vshlq_n_s32::<1>(*in_.add(7));
}

#[inline(always)]
unsafe fn highbd_fdct8_xn_neon(in_: *const int32x4_t, out: *mut int32x4_t, bit: i32,
                               howmany: i32) {
    let stride: i32 = 8;
    let mut i: i32 = 0;
    loop {
        highbd_fdct8_x4_neon(in_.offset((i * stride) as isize), out.offset((i * stride) as isize),
                             bit);
        i += 1;
        if !(i < howmany) {
            break;
        }
    }
}

#[inline(always)]
unsafe fn highbd_fadst8_xn_neon(in_: *const int32x4_t, out: *mut int32x4_t, bit: i32,
                                howmany: i32) {
    let stride: i32 = 8;
    let mut i: i32 = 0;
    loop {
        highbd_fadst8_x4_neon(in_.offset((i * stride) as isize), out.offset((i * stride) as isize),
                              bit);
        i += 1;
        if !(i < howmany) {
            break;
        }
    }
}

#[inline(always)]
unsafe fn highbd_fidentity8_xn_neon(in_: *const int32x4_t, out: *mut int32x4_t, bit: i32,
                                    howmany: i32) {
    let _ = bit;
    let stride: i32 = 8;
    let mut i: i32 = 0;
    loop {
        highbd_fidentity8_x4_neon(in_.offset((i * stride) as isize),
                                  out.offset((i * stride) as isize), bit);
        i += 1;
        if !(i < howmany) {
            break;
        }
    }
}

#[no_mangle]
pub unsafe extern "C-unwind" fn av1_fwd_txfm2d_8x8_neon(mut input: *const i16, coeff: *mut i32,
                                                        mut stride: i32, tx_type: TX_TYPE,
                                                        bd: i32) {
    let _ = bd;

    let mut ud_flip: i32 = 0;
    let mut lr_flip: i32 = 0;
    get_flip_cfg(tx_type, &mut ud_flip, &mut lr_flip);
    ud_adjust_input_and_stride(ud_flip, &mut input, &mut stride, 8);

    // Workspaces for column/row-wise transforms.
    let mut buf0_: [int32x4_t; 16] = core::mem::zeroed();
    let buf0 = buf0_.as_mut_ptr();
    let mut buf1_: [int32x4_t; 16] = core::mem::zeroed();
    let buf1 = buf1_.as_mut_ptr();

    match tx_type {
        DCT_DCT => {
            load_buffer_8x8(input, buf0, stride, 0);
            highbd_fdct8_xn_neon(buf0, buf0, av1_fwd_cos_bit_col[1][1] as i32, 2);
            shift_right_1_round_s32_x4(buf0, buf0, 16);
            transpose_arrays_s32_8x8(buf0, buf1);
            highbd_fdct8_xn_neon(buf1, buf1, av1_fwd_cos_bit_row[1][1] as i32, 2);
            store_buffer_8x8(buf1, coeff, /*stride=*/8);
        }
        ADST_DCT => {
            load_buffer_8x8(input, buf0, stride, 0);
            highbd_fadst8_xn_neon(buf0, buf0, av1_fwd_cos_bit_col[1][1] as i32, 2);
            shift_right_1_round_s32_x4(buf0, buf0, 16);
            transpose_arrays_s32_8x8(buf0, buf1);
            highbd_fdct8_xn_neon(buf1, buf1, av1_fwd_cos_bit_row[1][1] as i32, 2);
            store_buffer_8x8(buf1, coeff, /*stride=*/8);
        }
        DCT_ADST => {
            load_buffer_8x8(input, buf0, stride, 0);
            highbd_fdct8_xn_neon(buf0, buf0, av1_fwd_cos_bit_col[1][1] as i32, 2);
            shift_right_1_round_s32_x4(buf0, buf0, 16);
            transpose_arrays_s32_8x8(buf0, buf1);
            highbd_fadst8_xn_neon(buf1, buf1, av1_fwd_cos_bit_row[1][1] as i32, 2);
            store_buffer_8x8(buf1, coeff, /*stride=*/8);
        }
        ADST_ADST => {
            load_buffer_8x8(input, buf0, stride, 0);
            highbd_fadst8_xn_neon(buf0, buf0, av1_fwd_cos_bit_col[1][1] as i32, 2);
            shift_right_1_round_s32_x4(buf0, buf0, 16);
            transpose_arrays_s32_8x8(buf0, buf1);
            highbd_fadst8_xn_neon(buf1, buf1, av1_fwd_cos_bit_row[1][1] as i32, 2);
            store_buffer_8x8(buf1, coeff, /*stride=*/8);
        }
        FLIPADST_DCT => {
            load_buffer_8x8(input, buf0, stride, 0);
            highbd_fadst8_xn_neon(buf0, buf0, av1_fwd_cos_bit_col[1][1] as i32, 2);
            shift_right_1_round_s32_x4(buf0, buf0, 16);
            transpose_arrays_s32_8x8(buf0, buf1);
            highbd_fdct8_xn_neon(buf1, buf1, av1_fwd_cos_bit_row[1][1] as i32, 2);
            store_buffer_8x8(buf1, coeff, /*stride=*/8);
        }
        DCT_FLIPADST => {
            load_buffer_8x8(input, buf0, stride, 1);
            highbd_fdct8_xn_neon(buf0, buf0, av1_fwd_cos_bit_col[1][1] as i32, 2);
            shift_right_1_round_s32_x4(buf0, buf0, 16);
            transpose_arrays_s32_8x8(buf0, buf1);
            highbd_fadst8_xn_neon(buf1, buf1, av1_fwd_cos_bit_row[1][1] as i32, 2);
            store_buffer_8x8(buf1, coeff, /*stride=*/8);
        }
        FLIPADST_FLIPADST => {
            load_buffer_8x8(input, buf0, stride, 1);
            highbd_fadst8_xn_neon(buf0, buf0, av1_fwd_cos_bit_col[1][1] as i32, 2);
            shift_right_1_round_s32_x4(buf0, buf0, 16);
            transpose_arrays_s32_8x8(buf0, buf1);
            highbd_fadst8_xn_neon(buf1, buf1, av1_fwd_cos_bit_row[1][1] as i32, 2);
            store_buffer_8x8(buf1, coeff, /*stride=*/8);
        }
        ADST_FLIPADST => {
            load_buffer_8x8(input, buf0, stride, 1);
            highbd_fadst8_xn_neon(buf0, buf0, av1_fwd_cos_bit_col[1][1] as i32, 2);
            shift_right_1_round_s32_x4(buf0, buf0, 16);
            transpose_arrays_s32_8x8(buf0, buf1);
            highbd_fadst8_xn_neon(buf1, buf1, av1_fwd_cos_bit_row[1][1] as i32, 2);
            store_buffer_8x8(buf1, coeff, /*stride=*/8);
        }
        FLIPADST_ADST => {
            load_buffer_8x8(input, buf0, stride, 0);
            highbd_fadst8_xn_neon(buf0, buf0, av1_fwd_cos_bit_col[1][1] as i32, 2);
            shift_right_1_round_s32_x4(buf0, buf0, 16);
            transpose_arrays_s32_8x8(buf0, buf1);
            highbd_fadst8_xn_neon(buf1, buf1, av1_fwd_cos_bit_row[1][1] as i32, 2);
            store_buffer_8x8(buf1, coeff, /*stride=*/8);
        }
        IDTX => {
            load_buffer_8x8(input, buf0, stride, 0);
            highbd_fidentity8_xn_neon(buf0, buf0, av1_fwd_cos_bit_col[1][1] as i32, 2);
            shift_right_1_round_s32_x4(buf0, buf0, 16);
            transpose_arrays_s32_8x8(buf0, buf1);
            highbd_fidentity8_xn_neon(buf1, buf1, av1_fwd_cos_bit_col[1][1] as i32, 2);
            store_buffer_8x8(buf1, coeff, /*stride=*/8);
        }
        V_DCT => {
            load_buffer_8x8(input, buf0, stride, 0);
            highbd_fdct8_xn_neon(buf0, buf0, av1_fwd_cos_bit_col[1][1] as i32, 2);
            shift_right_1_round_s32_x4(buf0, buf0, 16);
            transpose_arrays_s32_8x8(buf0, buf1);
            highbd_fidentity8_xn_neon(buf1, buf1, av1_fwd_cos_bit_col[1][1] as i32, 2);
            store_buffer_8x8(buf1, coeff, /*stride=*/8);
        }
        H_DCT => {
            load_buffer_8x8(input, buf0, stride, 0);
            highbd_fidentity8_xn_neon(buf0, buf0, av1_fwd_cos_bit_col[1][1] as i32, 2);
            shift_right_1_round_s32_x4(buf0, buf0, 16);
            transpose_arrays_s32_8x8(buf0, buf1);
            highbd_fdct8_xn_neon(buf1, buf1, av1_fwd_cos_bit_col[1][1] as i32, 2);
            store_buffer_8x8(buf1, coeff, /*stride=*/8);
        }
        V_ADST => {
            load_buffer_8x8(input, buf0, stride, 0);
            highbd_fadst8_xn_neon(buf0, buf0, av1_fwd_cos_bit_col[1][1] as i32, 2);
            shift_right_1_round_s32_x4(buf0, buf0, 16);
            transpose_arrays_s32_8x8(buf0, buf1);
            highbd_fidentity8_xn_neon(buf1, buf1, av1_fwd_cos_bit_col[1][1] as i32, 2);
            store_buffer_8x8(buf1, coeff, /*stride=*/8);
        }
        H_ADST => {
            load_buffer_8x8(input, buf0, stride, 0);
            highbd_fidentity8_xn_neon(buf0, buf0, av1_fwd_cos_bit_col[1][1] as i32, 2);
            shift_right_1_round_s32_x4(buf0, buf0, 16);
            transpose_arrays_s32_8x8(buf0, buf1);
            highbd_fadst8_xn_neon(buf1, buf1, av1_fwd_cos_bit_col[1][1] as i32, 2);
            store_buffer_8x8(buf1, coeff, /*stride=*/8);
        }
        V_FLIPADST => {
            load_buffer_8x8(input, buf0, stride, 0);
            highbd_fadst8_xn_neon(buf0, buf0, av1_fwd_cos_bit_col[1][1] as i32, 2);
            shift_right_1_round_s32_x4(buf0, buf0, 16);
            transpose_arrays_s32_8x8(buf0, buf1);
            highbd_fidentity8_xn_neon(buf1, buf1, av1_fwd_cos_bit_col[1][1] as i32, 2);
            store_buffer_8x8(buf1, coeff, /*stride=*/8);
        }
        H_FLIPADST => {
            load_buffer_8x8(input, buf0, stride, 1);
            highbd_fidentity8_xn_neon(buf0, buf0, av1_fwd_cos_bit_col[1][1] as i32, 2);
            shift_right_1_round_s32_x4(buf0, buf0, 16);
            transpose_arrays_s32_8x8(buf0, buf1);
            highbd_fadst8_xn_neon(buf1, buf1, av1_fwd_cos_bit_col[1][1] as i32, 2);
            store_buffer_8x8(buf1, coeff, /*stride=*/8);
        }
        _ => debug_assert!(false),
    }
}

unsafe fn highbd_fdct16_x4_neon(in_: *const int32x4_t, out: *mut int32x4_t, bit: i32) {
    let cospi = cospi_arr_s32(bit);
    let v_bit = vdupq_n_s32(-bit);

    let mut u_: [int32x4_t; 16] = core::mem::zeroed();
    let u = u_.as_mut_ptr();
    let mut v_: [int32x4_t; 16] = core::mem::zeroed();
    let v = v_.as_mut_ptr();

    // stage 1
    butterfly_dct_pre(in_, u, 16);

    // stage 2
    butterfly_dct_pre(u, v, 8);
    *v.add(8) = *u.add(8);
    *v.add(9) = *u.add(9);
    butterfly_cospi32_0002_neon(cospi, *u.add(13), *u.add(10), v.add(13), v.add(10), v_bit);
    butterfly_cospi32_0002_neon(cospi, *u.add(12), *u.add(11), v.add(12), v.add(11), v_bit);
    *v.add(14) = *u.add(14);
    *v.add(15) = *u.add(15);

    // stage 3
    butterfly_dct_pre(v, u, 4);
    *u.add(4) = *v.add(4);
    butterfly_cospi32_0002_neon(cospi, *v.add(6), *v.add(5), u.add(6), u.add(5), v_bit);
    *u.add(7) = *v.add(7);
    butterfly_dct_post(v.add(8), v.add(8), u.add(8), 8);

    // stage 4
    butterfly_cospi32_0002_neon(cospi, *u.add(0), *u.add(1), v.add(0), v.add(1), v_bit);
    butterfly_0112_neon(cospi, 16, *u.add(3), *u.add(2), v.add(2), v.add(3), v_bit);
    butterfly_dct_post(u.add(4), u.add(4), v.add(4), 4);
    *v.add(8) = *u.add(8);
    butterfly_0112_neon(cospi, 16, *u.add(14), *u.add(9), v.add(14), v.add(9), v_bit);
    butterfly_2312_neon(cospi, 16, *u.add(13), *u.add(10), v.add(10), v.add(13), v_bit);
    *v.add(11) = *u.add(11);
    *v.add(12) = *u.add(12);
    *v.add(15) = *u.add(15);

    // stage 5
    *u.add(0) = *v.add(0);
    *u.add(1) = *v.add(1);
    *u.add(2) = *v.add(2);
    *u.add(3) = *v.add(3);
    butterfly_0112_neon(cospi, 8, *v.add(7), *v.add(4), u.add(4), u.add(7), v_bit);
    butterfly_0130_neon(cospi, 24, *v.add(5), *v.add(6), u.add(5), u.add(6), v_bit);
    butterfly_dct_post(v.add(8), v.add(8), u.add(8), 4);
    butterfly_dct_post(v.add(12), v.add(12), u.add(12), 4);

    // stage 6
    *v.add(0) = *u.add(0);
    *v.add(1) = *u.add(1);
    *v.add(2) = *u.add(2);
    *v.add(3) = *u.add(3);
    *v.add(4) = *u.add(4);
    *v.add(5) = *u.add(5);
    *v.add(6) = *u.add(6);
    *v.add(7) = *u.add(7);
    butterfly_0112_neon(cospi, 4, *u.add(15), *u.add(8), v.add(8), v.add(15), v_bit);
    butterfly_0130_neon(cospi, 28, *u.add(9), *u.add(14), v.add(9), v.add(14), v_bit);
    butterfly_0112_neon(cospi, 20, *u.add(13), *u.add(10), v.add(10), v.add(13), v_bit);
    butterfly_0130_neon(cospi, 12, *u.add(11), *u.add(12), v.add(11), v.add(12), v_bit);

    *out.add(0) = *v.add(0);
    *out.add(1) = *v.add(8);
    *out.add(2) = *v.add(4);
    *out.add(3) = *v.add(12);
    *out.add(4) = *v.add(2);
    *out.add(5) = *v.add(10);
    *out.add(6) = *v.add(6);
    *out.add(7) = *v.add(14);
    *out.add(8) = *v.add(1);
    *out.add(9) = *v.add(9);
    *out.add(10) = *v.add(5);
    *out.add(11) = *v.add(13);
    *out.add(12) = *v.add(3);
    *out.add(13) = *v.add(11);
    *out.add(14) = *v.add(7);
    *out.add(15) = *v.add(15);
}

unsafe fn highbd_fadst16_x4_neon(in_: *const int32x4_t, out: *mut int32x4_t, bit: i32) {
    let cospi = cospi_arr_s32(bit);
    let v_bit = vdupq_n_s32(-bit);

    let mut u_: [int32x4_t; 16] = core::mem::zeroed();
    let u = u_.as_mut_ptr();
    let mut v_: [int32x4_t; 16] = core::mem::zeroed();
    let v = v_.as_mut_ptr();

    // stage 0-1
    *u.add(0) = *in_.add(0);
    *u.add(1) = *in_.add(15);
    *u.add(2) = *in_.add(7);
    *u.add(3) = *in_.add(8);
    *u.add(4) = *in_.add(3);
    *u.add(5) = *in_.add(12);
    *u.add(6) = *in_.add(4);
    *u.add(7) = *in_.add(11);
    *u.add(8) = *in_.add(1);
    *u.add(9) = *in_.add(14);
    *u.add(10) = *in_.add(6);
    *u.add(11) = *in_.add(9);
    *u.add(12) = *in_.add(2);
    *u.add(13) = *in_.add(13);
    *u.add(14) = *in_.add(5);
    *u.add(15) = *in_.add(10);

    // stage 2
    *v.add(0) = *u.add(0);
    *v.add(1) = *u.add(1);
    butterfly_cospi32_0222_neon(cospi, *u.add(3), *u.add(2), v.add(2), v.add(3), v_bit);
    *v.add(4) = *u.add(4);
    *v.add(5) = *u.add(5);
    butterfly_cospi32_0002_neon(cospi, *u.add(6), *u.add(7), v.add(7), v.add(6), v_bit);
    *v.add(8) = *u.add(8);
    *v.add(9) = *u.add(9);
    butterfly_cospi32_0002_neon(cospi, *u.add(10), *u.add(11), v.add(11), v.add(10), v_bit);
    *v.add(12) = *u.add(12);
    *v.add(13) = *u.add(13);
    butterfly_cospi32_0222_neon(cospi, *u.add(15), *u.add(14), v.add(14), v.add(15), v_bit);

    // stage 3
    *u.add(0) = vaddq_s32(*v.add(0), *v.add(2));
    *u.add(1) = vsubq_s32(*v.add(3), *v.add(1));
    *u.add(2) = vsubq_s32(*v.add(0), *v.add(2));
    *u.add(3) = vaddq_s32(*v.add(1), *v.add(3));
    *u.add(4) = vsubq_s32(*v.add(6), *v.add(4));
    *u.add(5) = vaddq_s32(*v.add(5), *v.add(7));
    *u.add(6) = vaddq_s32(*v.add(4), *v.add(6));
    *u.add(7) = vsubq_s32(*v.add(5), *v.add(7));
    *u.add(8) = vsubq_s32(*v.add(10), *v.add(8));
    *u.add(9) = vaddq_s32(*v.add(9), *v.add(11));
    *u.add(10) = vaddq_s32(*v.add(8), *v.add(10));
    *u.add(11) = vsubq_s32(*v.add(9), *v.add(11));
    *u.add(12) = vaddq_s32(*v.add(12), *v.add(14));
    *u.add(13) = vsubq_s32(*v.add(15), *v.add(13));
    *u.add(14) = vsubq_s32(*v.add(12), *v.add(14));
    *u.add(15) = vaddq_s32(*v.add(13), *v.add(15));

    // stage 4
    *v.add(0) = *u.add(0);
    *v.add(1) = *u.add(1);
    *v.add(2) = *u.add(2);
    *v.add(3) = *u.add(3);
    butterfly_0112_neon(cospi, 16, *u.add(4), *u.add(5), v.add(4), v.add(5), v_bit);
    butterfly_0112_neon(cospi, 16, *u.add(7), *u.add(6), v.add(6), v.add(7), v_bit);

    *v.add(8) = *u.add(8);
    *v.add(9) = *u.add(9);
    *v.add(10) = *u.add(10);
    *v.add(11) = *u.add(11);

    butterfly_0112_neon(cospi, 16, *u.add(12), *u.add(13), v.add(12), v.add(13), v_bit);
    butterfly_0332_neon(cospi, 16, *u.add(14), *u.add(15), v.add(15), v.add(14), v_bit);

    // stage 5
    *u.add(0) = vaddq_s32(*v.add(0), *v.add(4));
    *u.add(1) = vaddq_s32(*v.add(1), *v.add(5));
    *u.add(2) = vaddq_s32(*v.add(2), *v.add(6));
    *u.add(3) = vsubq_s32(*v.add(7), *v.add(3));
    *u.add(4) = vsubq_s32(*v.add(0), *v.add(4));
    *u.add(5) = vsubq_s32(*v.add(1), *v.add(5));
    *u.add(6) = vsubq_s32(*v.add(2), *v.add(6));
    *u.add(7) = vaddq_s32(*v.add(3), *v.add(7));
    *u.add(8) = vaddq_s32(*v.add(8), *v.add(12));
    *u.add(9) = vaddq_s32(*v.add(9), *v.add(13));
    *u.add(10) = vsubq_s32(*v.add(14), *v.add(10));
    *u.add(11) = vaddq_s32(*v.add(11), *v.add(15));
    *u.add(12) = vsubq_s32(*v.add(8), *v.add(12));
    *u.add(13) = vsubq_s32(*v.add(9), *v.add(13));
    *u.add(14) = vaddq_s32(*v.add(10), *v.add(14));
    *u.add(15) = vsubq_s32(*v.add(11), *v.add(15));

    // stage 6
    *v.add(0) = *u.add(0);
    *v.add(1) = *u.add(1);
    *v.add(2) = *u.add(2);
    *v.add(3) = *u.add(3);
    *v.add(4) = *u.add(4);
    *v.add(5) = *u.add(5);
    *v.add(6) = *u.add(6);
    *v.add(7) = *u.add(7);

    butterfly_0112_neon(cospi, 8, *u.add(8), *u.add(9), v.add(8), v.add(9), v_bit);
    butterfly_0130_neon(cospi, 8, *u.add(12), *u.add(13), v.add(13), v.add(12), v_bit);
    butterfly_0130_neon(cospi, 24, *u.add(11), *u.add(10), v.add(10), v.add(11), v_bit);
    butterfly_0130_neon(cospi, 24, *u.add(14), *u.add(15), v.add(14), v.add(15), v_bit);

    // stage 7
    *u.add(0) = vaddq_s32(*v.add(0), *v.add(8));
    *u.add(1) = vaddq_s32(*v.add(1), *v.add(9));
    *u.add(2) = vaddq_s32(*v.add(2), *v.add(10));
    *u.add(3) = vaddq_s32(*v.add(3), *v.add(11));
    *u.add(4) = vaddq_s32(*v.add(4), *v.add(12));
    *u.add(5) = vaddq_s32(*v.add(5), *v.add(13));
    *u.add(6) = vaddq_s32(*v.add(6), *v.add(14));
    *u.add(7) = vsubq_s32(*v.add(15), *v.add(7));
    *u.add(8) = vsubq_s32(*v.add(0), *v.add(8));
    *u.add(9) = vsubq_s32(*v.add(1), *v.add(9));
    *u.add(10) = vsubq_s32(*v.add(2), *v.add(10));
    *u.add(11) = vsubq_s32(*v.add(3), *v.add(11));
    *u.add(12) = vsubq_s32(*v.add(4), *v.add(12));
    *u.add(13) = vsubq_s32(*v.add(5), *v.add(13));
    *u.add(14) = vsubq_s32(*v.add(6), *v.add(14));
    *u.add(15) = vaddq_s32(*v.add(7), *v.add(15));

    // stage 8
    butterfly_0112_neon(cospi, 2, *u.add(0), *u.add(1), v.add(0), v.add(1), v_bit);
    butterfly_0112_neon(cospi, 10, *u.add(2), *u.add(3), v.add(2), v.add(3), v_bit);
    butterfly_0112_neon(cospi, 18, *u.add(4), *u.add(5), v.add(4), v.add(5), v_bit);
    butterfly_0112_neon(cospi, 26, *u.add(6), *u.add(7), v.add(6), v.add(7), v_bit);
    butterfly_0130_neon(cospi, 30, *u.add(9), *u.add(8), v.add(8), v.add(9), v_bit);
    butterfly_0130_neon(cospi, 22, *u.add(11), *u.add(10), v.add(10), v.add(11), v_bit);
    butterfly_0130_neon(cospi, 14, *u.add(13), *u.add(12), v.add(12), v.add(13), v_bit);
    butterfly_0112_neon(cospi, 6, *u.add(14), *u.add(15), v.add(15), v.add(14), v_bit);

    // stage 9
    *out.add(0) = *v.add(1);
    *out.add(1) = *v.add(14);
    *out.add(2) = *v.add(3);
    *out.add(3) = *v.add(12);
    *out.add(4) = *v.add(5);
    *out.add(5) = *v.add(10);
    *out.add(6) = *v.add(7);
    *out.add(7) = *v.add(8);
    *out.add(8) = *v.add(9);
    *out.add(9) = *v.add(6);
    *out.add(10) = *v.add(11);
    *out.add(11) = *v.add(4);
    *out.add(12) = *v.add(13);
    *out.add(13) = *v.add(2);
    *out.add(14) = *v.add(15);
    *out.add(15) = *v.add(0);
}

unsafe fn highbd_fidentity16_x4_neon(in_: *const int32x4_t, out: *mut int32x4_t, bit: i32) {
    let _ = bit;
    let fact = vdupq_n_s32(2 * NewSqrt2);
    let offset = vdupq_n_s32(1 << (NewSqrt2Bits - 1));

    for i in 0..16 {
        let mut a = vmulq_s32(*in_.add(i), fact);
        a = vaddq_s32(a, offset);
        *out.add(i) = vshrq_n_s32::<NewSqrt2Bits>(a);
    }
}

unsafe fn highbd_fdct16_xn_neon(in_: *const int32x4_t, out: *mut int32x4_t, bit: i32,
                                howmany: i32) {
    let stride: i32 = 16;
    let mut i: i32 = 0;
    loop {
        highbd_fdct16_x4_neon(in_.offset((i * stride) as isize), out.offset((i * stride) as isize),
                              bit);
        i += 1;
        if !(i < howmany) {
            break;
        }
    }
}

unsafe fn highbd_fadst16_xn_neon(in_: *const int32x4_t, out: *mut int32x4_t, bit: i32,
                                 howmany: i32) {
    let stride: i32 = 16;
    let mut i: i32 = 0;
    loop {
        highbd_fadst16_x4_neon(in_.offset((i * stride) as isize),
                               out.offset((i * stride) as isize), bit);
        i += 1;
        if !(i < howmany) {
            break;
        }
    }
}

unsafe fn highbd_fidentity16_xn_neon(in_: *const int32x4_t, out: *mut int32x4_t, bit: i32,
                                     howmany: i32) {
    let stride: i32 = 16;
    let mut i: i32 = 0;
    loop {
        highbd_fidentity16_x4_neon(in_.offset((i * stride) as isize),
                                   out.offset((i * stride) as isize), bit);
        i += 1;
        if !(i < howmany) {
            break;
        }
    }
}

#[no_mangle]
pub unsafe extern "C-unwind" fn av1_fwd_txfm2d_16x16_neon(mut input: *const i16, coeff: *mut i32,
                                                          mut stride: i32, tx_type: TX_TYPE,
                                                          bd: i32) {
    let _ = bd;
    let mut ud_flip: i32 = 0;
    let mut lr_flip: i32 = 0;
    get_flip_cfg(tx_type, &mut ud_flip, &mut lr_flip);
    ud_adjust_input_and_stride(ud_flip, &mut input, &mut stride, 16);

    // Workspaces for column/row-wise transforms.
    let mut buf0_: [int32x4_t; 64] = core::mem::zeroed();
    let buf0 = buf0_.as_mut_ptr();
    let mut buf1_: [int32x4_t; 64] = core::mem::zeroed();
    let buf1 = buf1_.as_mut_ptr();

    match tx_type {
        DCT_DCT => {
            load_buffer_16x16(input, buf0, stride, 0);
            highbd_fdct16_xn_neon(buf0, buf0, av1_fwd_cos_bit_col[2][2] as i32, 4);
            shift_right_2_round_s32_x4(buf0, buf0, 64);
            transpose_arrays_s32_16x16(buf0, buf1);
            highbd_fdct16_xn_neon(buf1, buf1, av1_fwd_cos_bit_row[2][2] as i32, 4);
            store_buffer_16x16(buf1, coeff, /*stride=*/16);
        }
        ADST_DCT => {
            load_buffer_16x16(input, buf0, stride, 0);
            highbd_fadst16_xn_neon(buf0, buf0, av1_fwd_cos_bit_col[2][2] as i32, 4);
            shift_right_2_round_s32_x4(buf0, buf0, 64);
            transpose_arrays_s32_16x16(buf0, buf1);
            highbd_fdct16_xn_neon(buf1, buf1, av1_fwd_cos_bit_row[2][2] as i32, 4);
            store_buffer_16x16(buf1, coeff, /*stride=*/16);
        }
        DCT_ADST => {
            load_buffer_16x16(input, buf0, stride, 0);
            highbd_fdct16_xn_neon(buf0, buf0, av1_fwd_cos_bit_col[2][2] as i32, 4);
            shift_right_2_round_s32_x4(buf0, buf0, 64);
            transpose_arrays_s32_16x16(buf0, buf1);
            highbd_fadst16_xn_neon(buf1, buf1, av1_fwd_cos_bit_row[2][2] as i32, 4);
            store_buffer_16x16(buf1, coeff, /*stride=*/16);
        }
        ADST_ADST => {
            load_buffer_16x16(input, buf0, stride, 0);
            highbd_fadst16_xn_neon(buf0, buf0, av1_fwd_cos_bit_col[2][2] as i32, 4);
            shift_right_2_round_s32_x4(buf0, buf0, 64);
            transpose_arrays_s32_16x16(buf0, buf1);
            highbd_fadst16_xn_neon(buf1, buf1, av1_fwd_cos_bit_row[2][2] as i32, 4);
            store_buffer_16x16(buf1, coeff, /*stride=*/16);
        }
        FLIPADST_DCT => {
            load_buffer_16x16(input, buf0, stride, 0);
            highbd_fadst16_xn_neon(buf0, buf0, av1_fwd_cos_bit_col[2][2] as i32, 4);
            shift_right_2_round_s32_x4(buf0, buf0, 64);
            transpose_arrays_s32_16x16(buf0, buf1);
            highbd_fdct16_xn_neon(buf1, buf1, av1_fwd_cos_bit_row[2][2] as i32, 4);
            store_buffer_16x16(buf1, coeff, /*stride=*/16);
        }
        DCT_FLIPADST => {
            load_buffer_16x16(input, buf0, stride, 1);
            highbd_fdct16_xn_neon(buf0, buf0, av1_fwd_cos_bit_col[2][2] as i32, 4);
            shift_right_2_round_s32_x4(buf0, buf0, 64);
            transpose_arrays_s32_16x16(buf0, buf1);
            highbd_fadst16_xn_neon(buf1, buf1, av1_fwd_cos_bit_row[2][2] as i32, 4);
            store_buffer_16x16(buf1, coeff, /*stride=*/16);
        }
        FLIPADST_FLIPADST => {
            load_buffer_16x16(input, buf0, stride, 1);
            highbd_fadst16_xn_neon(buf0, buf0, av1_fwd_cos_bit_col[2][2] as i32, 4);
            shift_right_2_round_s32_x4(buf0, buf0, 64);
            transpose_arrays_s32_16x16(buf0, buf1);
            highbd_fadst16_xn_neon(buf1, buf1, av1_fwd_cos_bit_row[2][2] as i32, 4);
            store_buffer_16x16(buf1, coeff, /*stride=*/16);
        }
        ADST_FLIPADST => {
            load_buffer_16x16(input, buf0, stride, 1);
            highbd_fadst16_xn_neon(buf0, buf0, av1_fwd_cos_bit_col[2][2] as i32, 4);
            shift_right_2_round_s32_x4(buf0, buf0, 64);
            transpose_arrays_s32_16x16(buf0, buf1);
            highbd_fadst16_xn_neon(buf1, buf1, av1_fwd_cos_bit_row[2][2] as i32, 4);
            store_buffer_16x16(buf1, coeff, /*stride=*/16);
        }
        FLIPADST_ADST => {
            load_buffer_16x16(input, buf0, stride, 0);
            highbd_fadst16_xn_neon(buf0, buf0, av1_fwd_cos_bit_col[2][2] as i32, 4);
            shift_right_2_round_s32_x4(buf0, buf0, 64);
            transpose_arrays_s32_16x16(buf0, buf1);
            highbd_fadst16_xn_neon(buf1, buf1, av1_fwd_cos_bit_row[2][2] as i32, 4);
            store_buffer_16x16(buf1, coeff, /*stride=*/16);
        }
        IDTX => {
            load_buffer_16x16(input, buf0, stride, 0);
            highbd_fidentity16_xn_neon(buf0, buf0, av1_fwd_cos_bit_col[2][2] as i32, 4);
            shift_right_2_round_s32_x4(buf0, buf0, 64);
            transpose_arrays_s32_16x16(buf0, buf1);
            highbd_fidentity16_xn_neon(buf1, buf1, av1_fwd_cos_bit_row[2][2] as i32, 4);
            store_buffer_16x16(buf1, coeff, /*stride=*/16);
        }
        V_DCT => {
            load_buffer_16x16(input, buf0, stride, 0);
            highbd_fdct16_xn_neon(buf0, buf0, av1_fwd_cos_bit_col[2][2] as i32, 4);
            shift_right_2_round_s32_x4(buf0, buf0, 64);
            transpose_arrays_s32_16x16(buf0, buf1);
            highbd_fidentity16_xn_neon(buf1, buf1, av1_fwd_cos_bit_row[2][2] as i32, 4);
            store_buffer_16x16(buf1, coeff, /*stride=*/16);
        }
        H_DCT => {
            load_buffer_16x16(input, buf0, stride, 0);
            highbd_fidentity16_xn_neon(buf0, buf0, av1_fwd_cos_bit_col[2][2] as i32, 4);
            shift_right_2_round_s32_x4(buf0, buf0, 64);
            transpose_arrays_s32_16x16(buf0, buf1);
            highbd_fdct16_xn_neon(buf1, buf1, av1_fwd_cos_bit_row[2][2] as i32, 4);
            store_buffer_16x16(buf1, coeff, /*stride=*/16);
        }
        V_ADST => {
            load_buffer_16x16(input, buf0, stride, 0);
            highbd_fadst16_xn_neon(buf0, buf0, av1_fwd_cos_bit_col[2][2] as i32, 4);
            shift_right_2_round_s32_x4(buf0, buf0, 64);
            transpose_arrays_s32_16x16(buf0, buf1);
            highbd_fidentity16_xn_neon(buf1, buf1, av1_fwd_cos_bit_row[2][2] as i32, 4);
            store_buffer_16x16(buf1, coeff, /*stride=*/16);
        }
        H_ADST => {
            load_buffer_16x16(input, buf0, stride, 0);
            highbd_fidentity16_xn_neon(buf0, buf0, av1_fwd_cos_bit_col[2][2] as i32, 4);
            shift_right_2_round_s32_x4(buf0, buf0, 64);
            transpose_arrays_s32_16x16(buf0, buf1);
            highbd_fadst16_xn_neon(buf1, buf1, av1_fwd_cos_bit_row[2][2] as i32, 4);
            store_buffer_16x16(buf1, coeff, /*stride=*/16);
        }
        V_FLIPADST => {
            load_buffer_16x16(input, buf0, stride, 0);
            highbd_fadst16_xn_neon(buf0, buf0, av1_fwd_cos_bit_col[2][2] as i32, 4);
            shift_right_2_round_s32_x4(buf0, buf0, 64);
            transpose_arrays_s32_16x16(buf0, buf1);
            highbd_fidentity16_xn_neon(buf1, buf1, av1_fwd_cos_bit_row[2][2] as i32, 4);
            store_buffer_16x16(buf1, coeff, /*stride=*/16);
        }
        H_FLIPADST => {
            load_buffer_16x16(input, buf0, stride, 1);
            highbd_fidentity16_xn_neon(buf0, buf0, av1_fwd_cos_bit_col[2][2] as i32, 4);
            shift_right_2_round_s32_x4(buf0, buf0, 64);
            transpose_arrays_s32_16x16(buf0, buf1);
            highbd_fadst16_xn_neon(buf1, buf1, av1_fwd_cos_bit_row[2][2] as i32, 4);
            store_buffer_16x16(buf1, coeff, /*stride=*/16);
        }
        _ => debug_assert!(false),
    }
}

type fwd_transform_1d_col_neon =
    Option<unsafe fn(in_: *const i16, out: *mut int32x4_t, stride: i32, bit: i32, lr_flip: i32)>;
type fwd_transform_1d_col_many_neon = Option<
    unsafe fn(in_: *const i16, out: *mut int32x4_t, stride: i32, bit: i32, lr_flip: i32,
              howmany: i32, hm_stride: i32),
>;

type fwd_transform_1d_row_neon =
    Option<unsafe fn(in_: *const int32x4_t, out: *mut i32, bit: i32, stride: i32)>;
type fwd_transform_1d_row_many_neon = Option<
    unsafe fn(in_: *const int32x4_t, out: *mut i32, bit: i32, howmany: i32, hm_stride: i32,
              stride: i32),
>;

// Construct component kernels that include the load_buffer and store_buffer
// stages to avoid the need to spill loaded data to the stack between these and
// the txfm kernel calls.
// The TRANSFORM_*_ONE cases are only ever called in situations where the
// howmany parameter would be one, so no need for the loop at all in these
// cases.
//
// The C macros paste `highbd_##name##_...` identifiers; the Rust macros take
// the generated name, the 1-D kernel, and the load/store helper explicitly.

macro_rules! TRANSFORM_COL_ONE {
    ($fn_name:ident, $txfm:ident, $load:ident, $n:literal) => {
        unsafe fn $fn_name(input: *const i16, output: *mut int32x4_t, stride: i32, cos_bit: i32,
                           lr_flip: i32) {
            let mut buf0_: [int32x4_t; $n] = core::mem::zeroed();
            let buf0 = buf0_.as_mut_ptr();
            $load(input, buf0, stride, lr_flip);
            $txfm(buf0, output, cos_bit);
        }
    };
}

macro_rules! TRANSFORM_COL_MANY {
    ($fn_name:ident, $txfm:ident, $load:ident, $n:literal) => {
        unsafe fn $fn_name(input: *const i16, output: *mut int32x4_t, stride: i32, cos_bit: i32,
                           lr_flip: i32, howmany: i32, hm_stride: i32) {
            let mut i: i32 = 0;
            loop {
                let mut buf0_: [int32x4_t; $n] = core::mem::zeroed();
                let buf0 = buf0_.as_mut_ptr();
                $load(input.offset((4 * i) as isize), buf0, stride, lr_flip);
                $txfm(buf0, output.offset((i * hm_stride) as isize), cos_bit);
                i += 1;
                if !(i < howmany) {
                    break;
                }
            }
        }
    };
}

macro_rules! TRANSFORM_ROW_ONE {
    ($fn_name:ident, $txfm:ident, $store:ident, $n:literal) => {
        unsafe fn $fn_name(input: *const int32x4_t, output: *mut i32, cos_bit: i32, stride: i32) {
            let mut buf0_: [int32x4_t; $n] = core::mem::zeroed();
            let buf0 = buf0_.as_mut_ptr();
            $txfm(input, buf0, cos_bit);
            $store(buf0, output, stride);
        }
    };
}

macro_rules! TRANSFORM_ROW_RECT_ONE {
    ($fn_name:ident, $txfm:ident, $store:ident, $n:literal) => {
        unsafe fn $fn_name(input: *const int32x4_t, output: *mut i32, cos_bit: i32, stride: i32) {
            let mut buf0_: [int32x4_t; $n] = core::mem::zeroed();
            let buf0 = buf0_.as_mut_ptr();
            $txfm(input, buf0, cos_bit);
            round_rect_array_s32_neon(buf0, buf0, $n);
            $store(buf0, output, stride);
        }
    };
}

macro_rules! TRANSFORM_ROW_MANY {
    ($fn_name:ident, $txfm:ident, $store:ident, $n:literal) => {
        unsafe fn $fn_name(input: *const int32x4_t, output: *mut i32, cos_bit: i32, howmany: i32,
                           hm_stride: i32, stride: i32) {
            let mut i: i32 = 0;
            loop {
                let mut buf0_: [int32x4_t; $n] = core::mem::zeroed();
                let buf0 = buf0_.as_mut_ptr();
                $txfm(input.offset((hm_stride * i) as isize), buf0, cos_bit);
                $store(buf0, output.offset((4 * i) as isize), stride);
                i += 1;
                if !(i < howmany) {
                    break;
                }
            }
        }
    };
}

macro_rules! TRANSFORM_ROW_RECT_MANY {
    ($fn_name:ident, $txfm:ident, $store:ident, $n:literal) => {
        unsafe fn $fn_name(input: *const int32x4_t, output: *mut i32, cos_bit: i32, howmany: i32,
                           hm_stride: i32, stride: i32) {
            let mut i: i32 = 0;
            loop {
                let mut buf0_: [int32x4_t; $n] = core::mem::zeroed();
                let buf0 = buf0_.as_mut_ptr();
                $txfm(input.offset((hm_stride * i) as isize), buf0, cos_bit);
                round_rect_array_s32_neon(buf0, buf0, $n);
                $store(buf0, output.offset((4 * i) as isize), stride);
                i += 1;
                if !(i < howmany) {
                    break;
                }
            }
        }
    };
}

TRANSFORM_COL_ONE!(highbd_fdct8_col_neon, highbd_fdct8_x4_neon, load_buffer_4x8, 8);
TRANSFORM_COL_ONE!(highbd_fadst8_col_neon, highbd_fadst8_x4_neon, load_buffer_4x8, 8);
TRANSFORM_COL_ONE!(highbd_fidentity8_col_neon, highbd_fidentity8_x4_neon, load_buffer_4x8, 8);

TRANSFORM_COL_MANY!(highbd_fdct4_col_many_neon, highbd_fdct4_x4_neon, load_buffer_4x4, 4);
TRANSFORM_COL_MANY!(highbd_fdct8_col_many_neon, highbd_fdct8_x4_neon, load_buffer_4x8, 8);
TRANSFORM_COL_MANY!(highbd_fdct16_col_many_neon, highbd_fdct16_x4_neon, load_buffer_4x16, 16);
TRANSFORM_COL_MANY!(highbd_fadst4_col_many_neon, highbd_fadst4_x4_neon, load_buffer_4x4, 4);
TRANSFORM_COL_MANY!(highbd_fadst8_col_many_neon, highbd_fadst8_x4_neon, load_buffer_4x8, 8);
TRANSFORM_COL_MANY!(highbd_fadst16_col_many_neon, highbd_fadst16_x4_neon, load_buffer_4x16, 16);
TRANSFORM_COL_MANY!(highbd_fidentity4_col_many_neon, highbd_fidentity4_x4_neon, load_buffer_4x4,
                    4);
TRANSFORM_COL_MANY!(highbd_fidentity8_col_many_neon, highbd_fidentity8_x4_neon, load_buffer_4x8,
                    8);
TRANSFORM_COL_MANY!(highbd_fidentity16_col_many_neon, highbd_fidentity16_x4_neon,
                    load_buffer_4x16, 16);

TRANSFORM_ROW_ONE!(highbd_fdct16_row_neon, highbd_fdct16_x4_neon, store_buffer_16x4, 16);
TRANSFORM_ROW_ONE!(highbd_fadst16_row_neon, highbd_fadst16_x4_neon, store_buffer_16x4, 16);
TRANSFORM_ROW_ONE!(highbd_fidentity16_row_neon, highbd_fidentity16_x4_neon, store_buffer_16x4,
                   16);

TRANSFORM_ROW_RECT_ONE!(highbd_fdct8_row_rect_neon, highbd_fdct8_x4_neon, store_buffer_8x4, 8);
TRANSFORM_ROW_RECT_ONE!(highbd_fadst8_row_rect_neon, highbd_fadst8_x4_neon, store_buffer_8x4, 8);
TRANSFORM_ROW_RECT_ONE!(highbd_fidentity8_row_rect_neon, highbd_fidentity8_x4_neon,
                        store_buffer_8x4, 8);

// !CONFIG_REALTIME_ONLY
TRANSFORM_ROW_MANY!(highbd_fdct4_row_many_neon, highbd_fdct4_x4_neon, store_buffer_4x4, 4);
TRANSFORM_ROW_MANY!(highbd_fdct8_row_many_neon, highbd_fdct8_x4_neon, store_buffer_8x4, 8);
TRANSFORM_ROW_MANY!(highbd_fadst4_row_many_neon, highbd_fadst4_x4_neon, store_buffer_4x4, 4);
TRANSFORM_ROW_MANY!(highbd_fadst8_row_many_neon, highbd_fadst8_x4_neon, store_buffer_8x4, 8);
TRANSFORM_ROW_MANY!(highbd_fidentity4_row_many_neon, highbd_fidentity4_x4_neon, store_buffer_4x4,
                    4);
TRANSFORM_ROW_MANY!(highbd_fidentity8_row_many_neon, highbd_fidentity8_x4_neon, store_buffer_8x4,
                    8);

TRANSFORM_ROW_RECT_MANY!(highbd_fdct4_row_rect_many_neon, highbd_fdct4_x4_neon, store_buffer_4x4,
                         4);
TRANSFORM_ROW_RECT_MANY!(highbd_fdct8_row_rect_many_neon, highbd_fdct8_x4_neon, store_buffer_8x4,
                         8);
TRANSFORM_ROW_RECT_MANY!(highbd_fdct16_row_rect_many_neon, highbd_fdct16_x4_neon,
                         store_buffer_16x4, 16);
TRANSFORM_ROW_RECT_MANY!(highbd_fadst4_row_rect_many_neon, highbd_fadst4_x4_neon,
                         store_buffer_4x4, 4);
TRANSFORM_ROW_RECT_MANY!(highbd_fadst8_row_rect_many_neon, highbd_fadst8_x4_neon,
                         store_buffer_8x4, 8);
TRANSFORM_ROW_RECT_MANY!(highbd_fadst16_row_rect_many_neon, highbd_fadst16_x4_neon,
                         store_buffer_16x4, 16);
TRANSFORM_ROW_RECT_MANY!(highbd_fidentity4_row_rect_many_neon, highbd_fidentity4_x4_neon,
                         store_buffer_4x4, 4);
TRANSFORM_ROW_RECT_MANY!(highbd_fidentity8_row_rect_many_neon, highbd_fidentity8_x4_neon,
                         store_buffer_8x4, 8);
TRANSFORM_ROW_RECT_MANY!(highbd_fidentity16_row_rect_many_neon, highbd_fidentity16_x4_neon,
                         store_buffer_16x4, 16);

static col_highbd_txfm8_xn_arr: [fwd_transform_1d_col_many_neon; TX_TYPES] = [
    Some(highbd_fdct8_col_many_neon),      // DCT_DCT
    Some(highbd_fadst8_col_many_neon),     // ADST_DCT
    Some(highbd_fdct8_col_many_neon),      // DCT_ADST
    Some(highbd_fadst8_col_many_neon),     // ADST_ADST
    Some(highbd_fadst8_col_many_neon),     // FLIPADST_DCT
    Some(highbd_fdct8_col_many_neon),      // DCT_FLIPADST
    Some(highbd_fadst8_col_many_neon),     // FLIPADST_FLIPADST
    Some(highbd_fadst8_col_many_neon),     // ADST_FLIPADST
    Some(highbd_fadst8_col_many_neon),     // FLIPADST_ADST
    Some(highbd_fidentity8_col_many_neon), // IDTX
    Some(highbd_fdct8_col_many_neon),      // V_DCT
    Some(highbd_fidentity8_col_many_neon), // H_DCT
    Some(highbd_fadst8_col_many_neon),     // V_ADST
    Some(highbd_fidentity8_col_many_neon), // H_ADST
    Some(highbd_fadst8_col_many_neon),     // V_FLIPADST
    Some(highbd_fidentity8_col_many_neon), // H_FLIPADST
];

static col_highbd_txfm8_x4_arr: [fwd_transform_1d_col_neon; TX_TYPES] = [
    Some(highbd_fdct8_col_neon),      // DCT_DCT
    Some(highbd_fadst8_col_neon),     // ADST_DCT
    Some(highbd_fdct8_col_neon),      // DCT_ADST
    Some(highbd_fadst8_col_neon),     // ADST_ADST
    Some(highbd_fadst8_col_neon),     // FLIPADST_DCT
    Some(highbd_fdct8_col_neon),      // DCT_FLIPADST
    Some(highbd_fadst8_col_neon),     // FLIPADST_FLIPADST
    Some(highbd_fadst8_col_neon),     // ADST_FLIPADST
    Some(highbd_fadst8_col_neon),     // FLIPADST_ADST
    Some(highbd_fidentity8_col_neon), // IDTX
    Some(highbd_fdct8_col_neon),      // V_DCT
    Some(highbd_fidentity8_col_neon), // H_DCT
    Some(highbd_fadst8_col_neon),     // V_ADST
    Some(highbd_fidentity8_col_neon), // H_ADST
    Some(highbd_fadst8_col_neon),     // V_FLIPADST
    Some(highbd_fidentity8_col_neon), // H_FLIPADST
];

static col_highbd_txfm16_xn_arr: [fwd_transform_1d_col_many_neon; TX_TYPES] = [
    Some(highbd_fdct16_col_many_neon),      // DCT_DCT
    Some(highbd_fadst16_col_many_neon),     // ADST_DCT
    Some(highbd_fdct16_col_many_neon),      // DCT_ADST
    Some(highbd_fadst16_col_many_neon),     // ADST_ADST
    Some(highbd_fadst16_col_many_neon),     // FLIPADST_DCT
    Some(highbd_fdct16_col_many_neon),      // DCT_FLIPADST
    Some(highbd_fadst16_col_many_neon),     // FLIPADST_FLIPADST
    Some(highbd_fadst16_col_many_neon),     // ADST_FLIPADST
    Some(highbd_fadst16_col_many_neon),     // FLIPADST_ADST
    Some(highbd_fidentity16_col_many_neon), // IDTX
    Some(highbd_fdct16_col_many_neon),      // V_DCT
    Some(highbd_fidentity16_col_many_neon), // H_DCT
    Some(highbd_fadst16_col_many_neon),     // V_ADST
    Some(highbd_fidentity16_col_many_neon), // H_ADST
    Some(highbd_fadst16_col_many_neon),     // V_FLIPADST
    Some(highbd_fidentity16_col_many_neon), // H_FLIPADST
];

static col_highbd_txfm4_xn_arr: [fwd_transform_1d_col_many_neon; TX_TYPES] = [
    Some(highbd_fdct4_col_many_neon),      // DCT_DCT
    Some(highbd_fadst4_col_many_neon),     // ADST_DCT
    Some(highbd_fdct4_col_many_neon),      // DCT_ADST
    Some(highbd_fadst4_col_many_neon),     // ADST_ADST
    Some(highbd_fadst4_col_many_neon),     // FLIPADST_DCT
    Some(highbd_fdct4_col_many_neon),      // DCT_FLIPADST
    Some(highbd_fadst4_col_many_neon),     // FLIPADST_FLIPADST
    Some(highbd_fadst4_col_many_neon),     // ADST_FLIPADST
    Some(highbd_fadst4_col_many_neon),     // FLIPADST_ADST
    Some(highbd_fidentity4_col_many_neon), // IDTX
    Some(highbd_fdct4_col_many_neon),      // V_DCT
    Some(highbd_fidentity4_col_many_neon), // H_DCT
    Some(highbd_fadst4_col_many_neon),     // V_ADST
    Some(highbd_fidentity4_col_many_neon), // H_ADST
    Some(highbd_fadst4_col_many_neon),     // V_FLIPADST
    Some(highbd_fidentity4_col_many_neon), // H_FLIPADST
];

static row_highbd_txfm16_xn_arr: [fwd_transform_1d_row_neon; TX_TYPES] = [
    Some(highbd_fdct16_row_neon),      // DCT_DCT
    Some(highbd_fdct16_row_neon),      // ADST_DCT
    Some(highbd_fadst16_row_neon),     // DCT_ADST
    Some(highbd_fadst16_row_neon),     // ADST_ADST
    Some(highbd_fdct16_row_neon),      // FLIPADST_DCT
    Some(highbd_fadst16_row_neon),     // DCT_FLIPADST
    Some(highbd_fadst16_row_neon),     // FLIPADST_FLIPADST
    Some(highbd_fadst16_row_neon),     // ADST_FLIPADST
    Some(highbd_fadst16_row_neon),     // FLIPADST_ADST
    Some(highbd_fidentity16_row_neon), // IDTX
    Some(highbd_fidentity16_row_neon), // V_DCT
    Some(highbd_fdct16_row_neon),      // H_DCT
    Some(highbd_fidentity16_row_neon), // V_ADST
    Some(highbd_fadst16_row_neon),     // H_ADST
    Some(highbd_fidentity16_row_neon), // V_FLIPADST
    Some(highbd_fadst16_row_neon),     // H_FLIPADST
];

static row_rect_highbd_txfm16_xn_arr: [fwd_transform_1d_row_many_neon; TX_TYPES] = [
    Some(highbd_fdct16_row_rect_many_neon),      // DCT_DCT
    Some(highbd_fdct16_row_rect_many_neon),      // ADST_DCT
    Some(highbd_fadst16_row_rect_many_neon),     // DCT_ADST
    Some(highbd_fadst16_row_rect_many_neon),     // ADST_ADST
    Some(highbd_fdct16_row_rect_many_neon),      // FLIPADST_DCT
    Some(highbd_fadst16_row_rect_many_neon),     // DCT_FLIPADST
    Some(highbd_fadst16_row_rect_many_neon),     // FLIPADST_FLIPADST
    Some(highbd_fadst16_row_rect_many_neon),     // ADST_FLIPADST
    Some(highbd_fadst16_row_rect_many_neon),     // FLIPADST_ADST
    Some(highbd_fidentity16_row_rect_many_neon), // IDTX
    Some(highbd_fidentity16_row_rect_many_neon), // V_DCT
    Some(highbd_fdct16_row_rect_many_neon),      // H_DCT
    Some(highbd_fidentity16_row_rect_many_neon), // V_ADST
    Some(highbd_fadst16_row_rect_many_neon),     // H_ADST
    Some(highbd_fidentity16_row_rect_many_neon), // V_FLIPADST
    Some(highbd_fadst16_row_rect_many_neon),     // H_FLIPADST
];

// !CONFIG_REALTIME_ONLY
static row_highbd_txfm8_xn_arr: [fwd_transform_1d_row_many_neon; TX_TYPES] = [
    Some(highbd_fdct8_row_many_neon),      // DCT_DCT
    Some(highbd_fdct8_row_many_neon),      // ADST_DCT
    Some(highbd_fadst8_row_many_neon),     // DCT_ADST
    Some(highbd_fadst8_row_many_neon),     // ADST_ADST
    Some(highbd_fdct8_row_many_neon),      // FLIPADST_DCT
    Some(highbd_fadst8_row_many_neon),     // DCT_FLIPADST
    Some(highbd_fadst8_row_many_neon),     // FLIPADST_FLIPADST
    Some(highbd_fadst8_row_many_neon),     // ADST_FLIPADST
    Some(highbd_fadst8_row_many_neon),     // FLIPADST_ADST
    Some(highbd_fidentity8_row_many_neon), // IDTX
    Some(highbd_fidentity8_row_many_neon), // V_DCT
    Some(highbd_fdct8_row_many_neon),      // H_DCT
    Some(highbd_fidentity8_row_many_neon), // V_ADST
    Some(highbd_fadst8_row_many_neon),     // H_ADST
    Some(highbd_fidentity8_row_many_neon), // V_FLIPADST
    Some(highbd_fadst8_row_many_neon),     // H_FLIPADST
];

static row_rect_highbd_txfm8_xn_arr: [fwd_transform_1d_row_many_neon; TX_TYPES] = [
    Some(highbd_fdct8_row_rect_many_neon),      // DCT_DCT
    Some(highbd_fdct8_row_rect_many_neon),      // ADST_DCT
    Some(highbd_fadst8_row_rect_many_neon),     // DCT_ADST
    Some(highbd_fadst8_row_rect_many_neon),     // ADST_ADST
    Some(highbd_fdct8_row_rect_many_neon),      // FLIPADST_DCT
    Some(highbd_fadst8_row_rect_many_neon),     // DCT_FLIPADST
    Some(highbd_fadst8_row_rect_many_neon),     // FLIPADST_FLIPADST
    Some(highbd_fadst8_row_rect_many_neon),     // ADST_FLIPADST
    Some(highbd_fadst8_row_rect_many_neon),     // FLIPADST_ADST
    Some(highbd_fidentity8_row_rect_many_neon), // IDTX
    Some(highbd_fidentity8_row_rect_many_neon), // V_DCT
    Some(highbd_fdct8_row_rect_many_neon),      // H_DCT
    Some(highbd_fidentity8_row_rect_many_neon), // V_ADST
    Some(highbd_fadst8_row_rect_many_neon),     // H_ADST
    Some(highbd_fidentity8_row_rect_many_neon), // V_FLIPADST
    Some(highbd_fadst8_row_rect_many_neon),     // H_FLIPADST
];

static row_highbd_txfm8_x4_arr: [fwd_transform_1d_row_neon; TX_TYPES] = [
    Some(highbd_fdct8_row_rect_neon),      // DCT_DCT
    Some(highbd_fdct8_row_rect_neon),      // ADST_DCT
    Some(highbd_fadst8_row_rect_neon),     // DCT_ADST
    Some(highbd_fadst8_row_rect_neon),     // ADST_ADST
    Some(highbd_fdct8_row_rect_neon),      // FLIPADST_DCT
    Some(highbd_fadst8_row_rect_neon),     // DCT_FLIPADST
    Some(highbd_fadst8_row_rect_neon),     // FLIPADST_FLIPADST
    Some(highbd_fadst8_row_rect_neon),     // ADST_FLIPADST
    Some(highbd_fadst8_row_rect_neon),     // FLIPADST_ADST
    Some(highbd_fidentity8_row_rect_neon), // IDTX
    Some(highbd_fidentity8_row_rect_neon), // V_DCT
    Some(highbd_fdct8_row_rect_neon),      // H_DCT
    Some(highbd_fidentity8_row_rect_neon), // V_ADST
    Some(highbd_fadst8_row_rect_neon),     // H_ADST
    Some(highbd_fidentity8_row_rect_neon), // V_FLIPADST
    Some(highbd_fadst8_row_rect_neon),     // H_FLIPADST
];

// !CONFIG_REALTIME_ONLY
static row_highbd_txfm4_xn_arr: [fwd_transform_1d_row_many_neon; TX_TYPES] = [
    Some(highbd_fdct4_row_many_neon),      // DCT_DCT
    Some(highbd_fdct4_row_many_neon),      // ADST_DCT
    Some(highbd_fadst4_row_many_neon),     // DCT_ADST
    Some(highbd_fadst4_row_many_neon),     // ADST_ADST
    Some(highbd_fdct4_row_many_neon),      // FLIPADST_DCT
    Some(highbd_fadst4_row_many_neon),     // DCT_FLIPADST
    Some(highbd_fadst4_row_many_neon),     // FLIPADST_FLIPADST
    Some(highbd_fadst4_row_many_neon),     // ADST_FLIPADST
    Some(highbd_fadst4_row_many_neon),     // FLIPADST_ADST
    Some(highbd_fidentity4_row_many_neon), // IDTX
    Some(highbd_fidentity4_row_many_neon), // V_DCT
    Some(highbd_fdct4_row_many_neon),      // H_DCT
    Some(highbd_fidentity4_row_many_neon), // V_ADST
    Some(highbd_fadst4_row_many_neon),     // H_ADST
    Some(highbd_fidentity4_row_many_neon), // V_FLIPADST
    Some(highbd_fadst4_row_many_neon),     // H_FLIPADST
];

static row_rect_highbd_txfm4_xn_arr: [fwd_transform_1d_row_many_neon; TX_TYPES] = [
    Some(highbd_fdct4_row_rect_many_neon),      // DCT_DCT
    Some(highbd_fdct4_row_rect_many_neon),      // ADST_DCT
    Some(highbd_fadst4_row_rect_many_neon),     // DCT_ADST
    Some(highbd_fadst4_row_rect_many_neon),     // ADST_ADST
    Some(highbd_fdct4_row_rect_many_neon),      // FLIPADST_DCT
    Some(highbd_fadst4_row_rect_many_neon),     // DCT_FLIPADST
    Some(highbd_fadst4_row_rect_many_neon),     // FLIPADST_FLIPADST
    Some(highbd_fadst4_row_rect_many_neon),     // ADST_FLIPADST
    Some(highbd_fadst4_row_rect_many_neon),     // FLIPADST_ADST
    Some(highbd_fidentity4_row_rect_many_neon), // IDTX
    Some(highbd_fidentity4_row_rect_many_neon), // V_DCT
    Some(highbd_fdct4_row_rect_many_neon),      // H_DCT
    Some(highbd_fidentity4_row_rect_many_neon), // V_ADST
    Some(highbd_fadst4_row_rect_many_neon),     // H_ADST
    Some(highbd_fidentity4_row_rect_many_neon), // V_FLIPADST
    Some(highbd_fadst4_row_rect_many_neon),     // H_FLIPADST
];

unsafe fn highbd_fdct32_x4_neon(input: *const int32x4_t, output: *mut int32x4_t, cos_bit: i32) {
    let cospi = cospi_arr_s32(cos_bit);
    let v_cos_bit = vdupq_n_s32(-cos_bit);

    // Workspaces for intermediate transform steps.
    let mut buf0_: [int32x4_t; 32] = core::mem::zeroed();
    let buf0 = buf0_.as_mut_ptr();
    let mut buf1_: [int32x4_t; 32] = core::mem::zeroed();
    let buf1 = buf1_.as_mut_ptr();

    // stage 1
    butterfly_dct_pre(input, buf1, 32);

    // stage 2
    butterfly_dct_pre(buf1, buf0, 16);
    *buf0.add(16) = *buf1.add(16);
    *buf0.add(17) = *buf1.add(17);
    *buf0.add(18) = *buf1.add(18);
    *buf0.add(19) = *buf1.add(19);
    butterfly_0112_neon(cospi, 32, *buf1.add(27), *buf1.add(20), buf0.add(27), buf0.add(20), v_cos_bit);
    butterfly_0112_neon(cospi, 32, *buf1.add(26), *buf1.add(21), buf0.add(26), buf0.add(21), v_cos_bit);
    butterfly_0112_neon(cospi, 32, *buf1.add(25), *buf1.add(22), buf0.add(25), buf0.add(22), v_cos_bit);
    butterfly_0112_neon(cospi, 32, *buf1.add(24), *buf1.add(23), buf0.add(24), buf0.add(23), v_cos_bit);
    *buf0.add(28) = *buf1.add(28);
    *buf0.add(29) = *buf1.add(29);
    *buf0.add(30) = *buf1.add(30);
    *buf0.add(31) = *buf1.add(31);

    // stage 3
    butterfly_dct_pre(buf0, buf1, 8);
    *buf1.add(8) = *buf0.add(8);
    *buf1.add(9) = *buf0.add(9);
    butterfly_0112_neon(cospi, 32, *buf0.add(13), *buf0.add(10), buf1.add(13), buf1.add(10), v_cos_bit);
    butterfly_0112_neon(cospi, 32, *buf0.add(12), *buf0.add(11), buf1.add(12), buf1.add(11), v_cos_bit);
    *buf1.add(14) = *buf0.add(14);
    *buf1.add(15) = *buf0.add(15);
    butterfly_dct_post(buf0.add(16), buf0.add(16), buf1.add(16), 16);

    // stage 4
    butterfly_dct_pre(buf1, buf0, 4);
    *buf0.add(4) = *buf1.add(4);
    butterfly_0112_neon(cospi, 32, *buf1.add(6), *buf1.add(5), buf0.add(6), buf0.add(5), v_cos_bit);
    *buf0.add(7) = *buf1.add(7);
    butterfly_dct_post(buf1.add(8), buf1.add(8), buf0.add(8), 8);
    *buf0.add(16) = *buf1.add(16);
    *buf0.add(17) = *buf1.add(17);
    butterfly_0112_neon(cospi, 16, *buf1.add(29), *buf1.add(18), buf0.add(29), buf0.add(18), v_cos_bit);
    butterfly_0112_neon(cospi, 16, *buf1.add(28), *buf1.add(19), buf0.add(28), buf0.add(19), v_cos_bit);
    butterfly_2312_neon(cospi, 16, *buf1.add(27), *buf1.add(20), buf0.add(20), buf0.add(27), v_cos_bit);
    butterfly_2312_neon(cospi, 16, *buf1.add(26), *buf1.add(21), buf0.add(21), buf0.add(26), v_cos_bit);
    *buf0.add(22) = *buf1.add(22);
    *buf0.add(23) = *buf1.add(23);
    *buf0.add(24) = *buf1.add(24);
    *buf0.add(25) = *buf1.add(25);
    *buf0.add(30) = *buf1.add(30);
    *buf0.add(31) = *buf1.add(31);

    // stage 5
    butterfly_0112_neon(cospi, 32, *buf0.add(0), *buf0.add(1), buf1.add(0), buf1.add(1), v_cos_bit);
    butterfly_0112_neon(cospi, 16, *buf0.add(3), *buf0.add(2), buf1.add(2), buf1.add(3), v_cos_bit);
    butterfly_dct_post(buf0.add(4), buf0.add(4), buf1.add(4), 4);
    *buf1.add(8) = *buf0.add(8);
    butterfly_0112_neon(cospi, 16, *buf0.add(14), *buf0.add(9), buf1.add(14), buf1.add(9), v_cos_bit);
    butterfly_2312_neon(cospi, 16, *buf0.add(13), *buf0.add(10), buf1.add(10), buf1.add(13), v_cos_bit);
    *buf1.add(11) = *buf0.add(11);
    *buf1.add(12) = *buf0.add(12);
    *buf1.add(15) = *buf0.add(15);
    butterfly_dct_post(buf0.add(16), buf0.add(16), buf1.add(16), 8);
    butterfly_dct_post(buf0.add(24), buf0.add(24), buf1.add(24), 8);

    // stage 6
    *buf0.add(0) = *buf1.add(0);
    *buf0.add(1) = *buf1.add(1);
    *buf0.add(2) = *buf1.add(2);
    *buf0.add(3) = *buf1.add(3);

    butterfly_0112_neon(cospi, 8, *buf1.add(7), *buf1.add(4), buf0.add(4), buf0.add(7), v_cos_bit);
    butterfly_0112_neon(cospi, 8, *buf1.add(30), *buf1.add(17), buf0.add(30), buf0.add(17), v_cos_bit);
    butterfly_2312_neon(cospi, 8, *buf1.add(29), *buf1.add(18), buf0.add(18), buf0.add(29), v_cos_bit);
    butterfly_dct_post(buf1.add(8), buf1.add(8), buf0.add(8), 4);
    butterfly_dct_post(buf1.add(12), buf1.add(12), buf0.add(12), 4);
    *buf0.add(16) = *buf1.add(16);
    *buf0.add(19) = *buf1.add(19);
    *buf0.add(20) = *buf1.add(20);

    butterfly_0130_neon(cospi, 24, *buf1.add(5), *buf1.add(6), buf0.add(5), buf0.add(6), v_cos_bit);
    butterfly_0130_neon(cospi, 24, *buf1.add(21), *buf1.add(26), buf0.add(26), buf0.add(21), v_cos_bit);
    butterfly_0332_neon(cospi, 24, *buf1.add(25), *buf1.add(22), buf0.add(25), buf0.add(22), v_cos_bit);

    *buf0.add(23) = *buf1.add(23);
    *buf0.add(24) = *buf1.add(24);
    *buf0.add(27) = *buf1.add(27);
    *buf0.add(28) = *buf1.add(28);
    *buf0.add(31) = *buf1.add(31);

    // stage 7
    *buf1.add(0) = *buf0.add(0);
    *buf1.add(1) = *buf0.add(1);
    *buf1.add(2) = *buf0.add(2);
    *buf1.add(3) = *buf0.add(3);
    *buf1.add(4) = *buf0.add(4);
    *buf1.add(5) = *buf0.add(5);
    *buf1.add(6) = *buf0.add(6);
    *buf1.add(7) = *buf0.add(7);
    butterfly_0112_neon(cospi, 4, *buf0.add(15), *buf0.add(8), buf1.add(8), buf1.add(15), v_cos_bit);
    butterfly_0130_neon(cospi, 28, *buf0.add(9), *buf0.add(14), buf1.add(9), buf1.add(14), v_cos_bit);
    butterfly_0112_neon(cospi, 20, *buf0.add(13), *buf0.add(10), buf1.add(10), buf1.add(13), v_cos_bit);
    butterfly_0130_neon(cospi, 12, *buf0.add(11), *buf0.add(12), buf1.add(11), buf1.add(12), v_cos_bit);
    butterfly_dct_post(buf0.add(16), buf0.add(16), buf1.add(16), 4);
    butterfly_dct_post(buf0.add(20), buf0.add(20), buf1.add(20), 4);
    butterfly_dct_post(buf0.add(24), buf0.add(24), buf1.add(24), 4);
    butterfly_dct_post(buf0.add(28), buf0.add(28), buf1.add(28), 4);

    // stage 8
    *buf0.add(0) = *buf1.add(0);
    *buf0.add(1) = *buf1.add(1);
    *buf0.add(2) = *buf1.add(2);
    *buf0.add(3) = *buf1.add(3);
    *buf0.add(4) = *buf1.add(4);
    *buf0.add(5) = *buf1.add(5);
    *buf0.add(6) = *buf1.add(6);
    *buf0.add(7) = *buf1.add(7);
    *buf0.add(8) = *buf1.add(8);
    *buf0.add(9) = *buf1.add(9);
    *buf0.add(10) = *buf1.add(10);
    *buf0.add(11) = *buf1.add(11);
    *buf0.add(12) = *buf1.add(12);
    *buf0.add(13) = *buf1.add(13);
    *buf0.add(14) = *buf1.add(14);
    *buf0.add(15) = *buf1.add(15);
    butterfly_0112_neon(cospi, 2, *buf1.add(31), *buf1.add(16), buf0.add(16), buf0.add(31), v_cos_bit);
    butterfly_0130_neon(cospi, 30, *buf1.add(17), *buf1.add(30), buf0.add(17), buf0.add(30), v_cos_bit);
    butterfly_0112_neon(cospi, 18, *buf1.add(29), *buf1.add(18), buf0.add(18), buf0.add(29), v_cos_bit);
    butterfly_0130_neon(cospi, 14, *buf1.add(19), *buf1.add(28), buf0.add(19), buf0.add(28), v_cos_bit);
    butterfly_0112_neon(cospi, 10, *buf1.add(27), *buf1.add(20), buf0.add(20), buf0.add(27), v_cos_bit);
    butterfly_0130_neon(cospi, 22, *buf1.add(21), *buf1.add(26), buf0.add(21), buf0.add(26), v_cos_bit);
    butterfly_0112_neon(cospi, 26, *buf1.add(25), *buf1.add(22), buf0.add(22), buf0.add(25), v_cos_bit);
    butterfly_0130_neon(cospi, 6, *buf1.add(23), *buf1.add(24), buf0.add(23), buf0.add(24), v_cos_bit);

    // stage 9
    *output.add(0) = *buf0.add(0);
    *output.add(1) = *buf0.add(16);
    *output.add(2) = *buf0.add(8);
    *output.add(3) = *buf0.add(24);
    *output.add(4) = *buf0.add(4);
    *output.add(5) = *buf0.add(20);
    *output.add(6) = *buf0.add(12);
    *output.add(7) = *buf0.add(28);
    *output.add(8) = *buf0.add(2);
    *output.add(9) = *buf0.add(18);
    *output.add(10) = *buf0.add(10);
    *output.add(11) = *buf0.add(26);
    *output.add(12) = *buf0.add(6);
    *output.add(13) = *buf0.add(22);
    *output.add(14) = *buf0.add(14);
    *output.add(15) = *buf0.add(30);
    *output.add(16) = *buf0.add(1);
    *output.add(17) = *buf0.add(17);
    *output.add(18) = *buf0.add(9);
    *output.add(19) = *buf0.add(25);
    *output.add(20) = *buf0.add(5);
    *output.add(21) = *buf0.add(21);
    *output.add(22) = *buf0.add(13);
    *output.add(23) = *buf0.add(29);
    *output.add(24) = *buf0.add(3);
    *output.add(25) = *buf0.add(19);
    *output.add(26) = *buf0.add(11);
    *output.add(27) = *buf0.add(27);
    *output.add(28) = *buf0.add(7);
    *output.add(29) = *buf0.add(23);
    *output.add(30) = *buf0.add(15);
    *output.add(31) = *buf0.add(31);
}

unsafe fn highbd_fdct64_x4_neon(input: *const int32x4_t, output: *mut int32x4_t, cos_bit: i8) {
    let cospi = cospi_arr_s32(cos_bit as i32);
    let v_cos_bit = vdupq_n_s32(-(cos_bit as i32));

    // stage 1
    let mut x1_: [int32x4_t; 64] = core::mem::zeroed();
    let x1 = x1_.as_mut_ptr();
    butterfly_dct_pre(input, x1, 64);

    // stage 2
    let mut x2_: [int32x4_t; 64] = core::mem::zeroed();
    let x2 = x2_.as_mut_ptr();
    butterfly_dct_pre(x1, x2, 32);
    *x2.add(32) = *x1.add(32);
    *x2.add(33) = *x1.add(33);
    *x2.add(34) = *x1.add(34);
    *x2.add(35) = *x1.add(35);
    *x2.add(36) = *x1.add(36);
    *x2.add(37) = *x1.add(37);
    *x2.add(38) = *x1.add(38);
    *x2.add(39) = *x1.add(39);
    butterfly_0112_neon(cospi, 32, *x1.add(55), *x1.add(40), x2.add(55), x2.add(40), v_cos_bit);
    butterfly_0112_neon(cospi, 32, *x1.add(54), *x1.add(41), x2.add(54), x2.add(41), v_cos_bit);
    butterfly_0112_neon(cospi, 32, *x1.add(53), *x1.add(42), x2.add(53), x2.add(42), v_cos_bit);
    butterfly_0112_neon(cospi, 32, *x1.add(52), *x1.add(43), x2.add(52), x2.add(43), v_cos_bit);
    butterfly_0112_neon(cospi, 32, *x1.add(51), *x1.add(44), x2.add(51), x2.add(44), v_cos_bit);
    butterfly_0112_neon(cospi, 32, *x1.add(50), *x1.add(45), x2.add(50), x2.add(45), v_cos_bit);
    butterfly_0112_neon(cospi, 32, *x1.add(49), *x1.add(46), x2.add(49), x2.add(46), v_cos_bit);
    butterfly_0112_neon(cospi, 32, *x1.add(48), *x1.add(47), x2.add(48), x2.add(47), v_cos_bit);
    *x2.add(56) = *x1.add(56);
    *x2.add(57) = *x1.add(57);
    *x2.add(58) = *x1.add(58);
    *x2.add(59) = *x1.add(59);
    *x2.add(60) = *x1.add(60);
    *x2.add(61) = *x1.add(61);
    *x2.add(62) = *x1.add(62);
    *x2.add(63) = *x1.add(63);

    // stage 3
    let mut x3_: [int32x4_t; 64] = core::mem::zeroed();
    let x3 = x3_.as_mut_ptr();
    butterfly_dct_pre(x2, x3, 16);
    *x3.add(16) = *x2.add(16);
    *x3.add(17) = *x2.add(17);
    *x3.add(18) = *x2.add(18);
    *x3.add(19) = *x2.add(19);
    butterfly_0112_neon(cospi, 32, *x2.add(27), *x2.add(20), x3.add(27), x3.add(20), v_cos_bit);
    butterfly_0112_neon(cospi, 32, *x2.add(26), *x2.add(21), x3.add(26), x3.add(21), v_cos_bit);
    butterfly_0112_neon(cospi, 32, *x2.add(25), *x2.add(22), x3.add(25), x3.add(22), v_cos_bit);
    butterfly_0112_neon(cospi, 32, *x2.add(24), *x2.add(23), x3.add(24), x3.add(23), v_cos_bit);
    *x3.add(28) = *x2.add(28);
    *x3.add(29) = *x2.add(29);
    *x3.add(30) = *x2.add(30);
    *x3.add(31) = *x2.add(31);
    butterfly_dct_post(x2.add(32), x2.add(32), x3.add(32), 32);

    // stage 4
    let mut x4_: [int32x4_t; 64] = core::mem::zeroed();
    let x4 = x4_.as_mut_ptr();
    butterfly_dct_pre(x3, x4, 8);
    *x4.add(8) = *x3.add(8);
    *x4.add(9) = *x3.add(9);
    butterfly_0112_neon(cospi, 32, *x3.add(13), *x3.add(10), x4.add(13), x4.add(10), v_cos_bit);
    butterfly_0112_neon(cospi, 32, *x3.add(12), *x3.add(11), x4.add(12), x4.add(11), v_cos_bit);
    *x4.add(14) = *x3.add(14);
    *x4.add(15) = *x3.add(15);
    butterfly_dct_post(x3.add(16), x3.add(16), x4.add(16), 16);
    *x4.add(32) = *x3.add(32);
    *x4.add(33) = *x3.add(33);
    *x4.add(34) = *x3.add(34);
    *x4.add(35) = *x3.add(35);
    butterfly_0112_neon(cospi, 16, *x3.add(59), *x3.add(36), x4.add(59), x4.add(36), v_cos_bit);
    butterfly_0112_neon(cospi, 16, *x3.add(58), *x3.add(37), x4.add(58), x4.add(37), v_cos_bit);
    butterfly_0112_neon(cospi, 16, *x3.add(57), *x3.add(38), x4.add(57), x4.add(38), v_cos_bit);
    butterfly_0112_neon(cospi, 16, *x3.add(56), *x3.add(39), x4.add(56), x4.add(39), v_cos_bit);
    butterfly_2312_neon(cospi, 16, *x3.add(55), *x3.add(40), x4.add(40), x4.add(55), v_cos_bit);
    butterfly_2312_neon(cospi, 16, *x3.add(54), *x3.add(41), x4.add(41), x4.add(54), v_cos_bit);
    butterfly_2312_neon(cospi, 16, *x3.add(53), *x3.add(42), x4.add(42), x4.add(53), v_cos_bit);
    butterfly_2312_neon(cospi, 16, *x3.add(52), *x3.add(43), x4.add(43), x4.add(52), v_cos_bit);
    *x4.add(44) = *x3.add(44);
    *x4.add(45) = *x3.add(45);
    *x4.add(46) = *x3.add(46);
    *x4.add(47) = *x3.add(47);
    *x4.add(48) = *x3.add(48);
    *x4.add(49) = *x3.add(49);
    *x4.add(50) = *x3.add(50);
    *x4.add(51) = *x3.add(51);
    *x4.add(60) = *x3.add(60);
    *x4.add(61) = *x3.add(61);
    *x4.add(62) = *x3.add(62);
    *x4.add(63) = *x3.add(63);

    // stage 5
    let mut x5_: [int32x4_t; 64] = core::mem::zeroed();
    let x5 = x5_.as_mut_ptr();
    butterfly_dct_pre(x4, x5, 4);
    *x5.add(4) = *x4.add(4);
    butterfly_0112_neon(cospi, 32, *x4.add(6), *x4.add(5), x5.add(6), x5.add(5), v_cos_bit);
    *x5.add(7) = *x4.add(7);
    butterfly_dct_post(x4.add(8), x4.add(8), x5.add(8), 8);
    *x5.add(16) = *x4.add(16);
    *x5.add(17) = *x4.add(17);
    butterfly_0112_neon(cospi, 16, *x4.add(29), *x4.add(18), x5.add(29), x5.add(18), v_cos_bit);
    butterfly_0112_neon(cospi, 16, *x4.add(28), *x4.add(19), x5.add(28), x5.add(19), v_cos_bit);
    butterfly_2312_neon(cospi, 16, *x4.add(27), *x4.add(20), x5.add(20), x5.add(27), v_cos_bit);
    butterfly_2312_neon(cospi, 16, *x4.add(26), *x4.add(21), x5.add(21), x5.add(26), v_cos_bit);
    *x5.add(22) = *x4.add(22);
    *x5.add(23) = *x4.add(23);
    *x5.add(24) = *x4.add(24);
    *x5.add(25) = *x4.add(25);
    *x5.add(30) = *x4.add(30);
    *x5.add(31) = *x4.add(31);
    butterfly_dct_post(x4.add(32), x4.add(32), x5.add(32), 16);
    butterfly_dct_post(x4.add(48), x4.add(48), x5.add(48), 16);

    // stage 6
    let mut x6_: [int32x4_t; 64] = core::mem::zeroed();
    let x6 = x6_.as_mut_ptr();
    butterfly_0112_neon(cospi, 32, *x5.add(0), *x5.add(1), x6.add(0), x6.add(1), v_cos_bit);
    butterfly_0112_neon(cospi, 16, *x5.add(3), *x5.add(2), x6.add(2), x6.add(3), v_cos_bit);
    butterfly_dct_post(x5.add(4), x5.add(4), x6.add(4), 4);
    *x6.add(8) = *x5.add(8);
    butterfly_0112_neon(cospi, 16, *x5.add(14), *x5.add(9), x6.add(14), x6.add(9), v_cos_bit);
    butterfly_2312_neon(cospi, 16, *x5.add(13), *x5.add(10), x6.add(10), x6.add(13), v_cos_bit);
    *x6.add(11) = *x5.add(11);
    *x6.add(12) = *x5.add(12);
    *x6.add(15) = *x5.add(15);
    butterfly_dct_post(x5.add(16), x5.add(16), x6.add(16), 8);
    butterfly_dct_post(x5.add(24), x5.add(24), x6.add(24), 8);
    *x6.add(32) = *x5.add(32);
    *x6.add(33) = *x5.add(33);
    butterfly_0112_neon(cospi, 8, *x5.add(61), *x5.add(34), x6.add(61), x6.add(34), v_cos_bit);
    butterfly_0112_neon(cospi, 8, *x5.add(60), *x5.add(35), x6.add(60), x6.add(35), v_cos_bit);
    butterfly_2312_neon(cospi, 8, *x5.add(59), *x5.add(36), x6.add(36), x6.add(59), v_cos_bit);
    butterfly_2312_neon(cospi, 8, *x5.add(58), *x5.add(37), x6.add(37), x6.add(58), v_cos_bit);
    *x6.add(38) = *x5.add(38);
    *x6.add(39) = *x5.add(39);
    *x6.add(40) = *x5.add(40);
    *x6.add(41) = *x5.add(41);
    butterfly_0130_neon(cospi, 24, *x5.add(42), *x5.add(53), x6.add(53), x6.add(42), v_cos_bit);
    butterfly_0130_neon(cospi, 24, *x5.add(43), *x5.add(52), x6.add(52), x6.add(43), v_cos_bit);
    butterfly_0332_neon(cospi, 24, *x5.add(51), *x5.add(44), x6.add(51), x6.add(44), v_cos_bit);
    butterfly_0332_neon(cospi, 24, *x5.add(50), *x5.add(45), x6.add(50), x6.add(45), v_cos_bit);
    *x6.add(46) = *x5.add(46);
    *x6.add(47) = *x5.add(47);
    *x6.add(48) = *x5.add(48);
    *x6.add(49) = *x5.add(49);
    *x6.add(54) = *x5.add(54);
    *x6.add(55) = *x5.add(55);
    *x6.add(56) = *x5.add(56);
    *x6.add(57) = *x5.add(57);
    *x6.add(62) = *x5.add(62);
    *x6.add(63) = *x5.add(63);

    // stage 7
    let mut x7_: [int32x4_t; 64] = core::mem::zeroed();
    let x7 = x7_.as_mut_ptr();
    *x7.add(0) = *x6.add(0);
    *x7.add(1) = *x6.add(1);
    *x7.add(2) = *x6.add(2);
    *x7.add(3) = *x6.add(3);
    butterfly_0112_neon(cospi, 8, *x6.add(7), *x6.add(4), x7.add(4), x7.add(7), v_cos_bit);
    butterfly_0130_neon(cospi, 24, *x6.add(5), *x6.add(6), x7.add(5), x7.add(6), v_cos_bit);
    butterfly_dct_post(x6.add(8), x6.add(8), x7.add(8), 4);
    butterfly_dct_post(x6.add(12), x6.add(12), x7.add(12), 4);
    *x7.add(16) = *x6.add(16);
    butterfly_0112_neon(cospi, 8, *x6.add(30), *x6.add(17), x7.add(30), x7.add(17), v_cos_bit);
    butterfly_2312_neon(cospi, 8, *x6.add(29), *x6.add(18), x7.add(18), x7.add(29), v_cos_bit);
    *x7.add(19) = *x6.add(19);
    *x7.add(20) = *x6.add(20);
    butterfly_0130_neon(cospi, 24, *x6.add(21), *x6.add(26), x7.add(26), x7.add(21), v_cos_bit);
    butterfly_0332_neon(cospi, 24, *x6.add(25), *x6.add(22), x7.add(25), x7.add(22), v_cos_bit);
    *x7.add(23) = *x6.add(23);
    *x7.add(24) = *x6.add(24);
    *x7.add(27) = *x6.add(27);
    *x7.add(28) = *x6.add(28);
    *x7.add(31) = *x6.add(31);
    butterfly_dct_post(x6.add(32), x6.add(32), x7.add(32), 8);
    butterfly_dct_post(x6.add(40), x6.add(40), x7.add(40), 8);
    butterfly_dct_post(x6.add(48), x6.add(48), x7.add(48), 8);
    butterfly_dct_post(x6.add(56), x6.add(56), x7.add(56), 8);

    // stage 8
    let mut x8_: [int32x4_t; 64] = core::mem::zeroed();
    let x8 = x8_.as_mut_ptr();
    *x8.add(0) = *x7.add(0);
    *x8.add(1) = *x7.add(1);
    *x8.add(2) = *x7.add(2);
    *x8.add(3) = *x7.add(3);
    *x8.add(4) = *x7.add(4);
    *x8.add(5) = *x7.add(5);
    *x8.add(6) = *x7.add(6);
    *x8.add(7) = *x7.add(7);

    butterfly_0112_neon(cospi, 4, *x7.add(15), *x7.add(8), x8.add(8), x8.add(15), v_cos_bit);
    butterfly_0130_neon(cospi, 28, *x7.add(9), *x7.add(14), x8.add(9), x8.add(14), v_cos_bit);
    butterfly_0112_neon(cospi, 20, *x7.add(13), *x7.add(10), x8.add(10), x8.add(13), v_cos_bit);
    butterfly_0130_neon(cospi, 12, *x7.add(11), *x7.add(12), x8.add(11), x8.add(12), v_cos_bit);
    butterfly_dct_post(x7.add(16), x7.add(16), x8.add(16), 4);
    butterfly_dct_post(x7.add(20), x7.add(20), x8.add(20), 4);
    butterfly_dct_post(x7.add(24), x7.add(24), x8.add(24), 4);
    butterfly_dct_post(x7.add(28), x7.add(28), x8.add(28), 4);
    *x8.add(32) = *x7.add(32);
    butterfly_0112_neon(cospi, 4, *x7.add(62), *x7.add(33), x8.add(62), x8.add(33), v_cos_bit);
    butterfly_2312_neon(cospi, 4, *x7.add(61), *x7.add(34), x8.add(34), x8.add(61), v_cos_bit);
    *x8.add(35) = *x7.add(35);
    *x8.add(36) = *x7.add(36);
    butterfly_0130_neon(cospi, 28, *x7.add(37), *x7.add(58), x8.add(58), x8.add(37), v_cos_bit);
    butterfly_0332_neon(cospi, 28, *x7.add(57), *x7.add(38), x8.add(57), x8.add(38), v_cos_bit);
    *x8.add(39) = *x7.add(39);
    *x8.add(40) = *x7.add(40);
    butterfly_0112_neon(cospi, 20, *x7.add(54), *x7.add(41), x8.add(54), x8.add(41), v_cos_bit);
    butterfly_2312_neon(cospi, 20, *x7.add(53), *x7.add(42), x8.add(42), x8.add(53), v_cos_bit);
    *x8.add(43) = *x7.add(43);
    *x8.add(44) = *x7.add(44);
    butterfly_0130_neon(cospi, 12, *x7.add(45), *x7.add(50), x8.add(50), x8.add(45), v_cos_bit);
    butterfly_0332_neon(cospi, 12, *x7.add(49), *x7.add(46), x8.add(49), x8.add(46), v_cos_bit);
    *x8.add(47) = *x7.add(47);
    *x8.add(48) = *x7.add(48);
    *x8.add(51) = *x7.add(51);
    *x8.add(52) = *x7.add(52);
    *x8.add(55) = *x7.add(55);
    *x8.add(56) = *x7.add(56);
    *x8.add(59) = *x7.add(59);
    *x8.add(60) = *x7.add(60);
    *x8.add(63) = *x7.add(63);

    // stage 9
    let mut x9_: [int32x4_t; 64] = core::mem::zeroed();
    let x9 = x9_.as_mut_ptr();
    *x9.add(0) = *x8.add(0);
    *x9.add(1) = *x8.add(1);
    *x9.add(2) = *x8.add(2);
    *x9.add(3) = *x8.add(3);
    *x9.add(4) = *x8.add(4);
    *x9.add(5) = *x8.add(5);
    *x9.add(6) = *x8.add(6);
    *x9.add(7) = *x8.add(7);
    *x9.add(8) = *x8.add(8);
    *x9.add(9) = *x8.add(9);
    *x9.add(10) = *x8.add(10);
    *x9.add(11) = *x8.add(11);
    *x9.add(12) = *x8.add(12);
    *x9.add(13) = *x8.add(13);
    *x9.add(14) = *x8.add(14);
    *x9.add(15) = *x8.add(15);
    butterfly_0112_neon(cospi, 2, *x8.add(31), *x8.add(16), x9.add(16), x9.add(31), v_cos_bit);
    butterfly_0130_neon(cospi, 30, *x8.add(17), *x8.add(30), x9.add(17), x9.add(30), v_cos_bit);
    butterfly_0112_neon(cospi, 18, *x8.add(29), *x8.add(18), x9.add(18), x9.add(29), v_cos_bit);
    butterfly_0130_neon(cospi, 14, *x8.add(19), *x8.add(28), x9.add(19), x9.add(28), v_cos_bit);
    butterfly_0112_neon(cospi, 10, *x8.add(27), *x8.add(20), x9.add(20), x9.add(27), v_cos_bit);
    butterfly_0130_neon(cospi, 22, *x8.add(21), *x8.add(26), x9.add(21), x9.add(26), v_cos_bit);
    butterfly_0112_neon(cospi, 26, *x8.add(25), *x8.add(22), x9.add(22), x9.add(25), v_cos_bit);
    butterfly_0130_neon(cospi, 6, *x8.add(23), *x8.add(24), x9.add(23), x9.add(24), v_cos_bit);
    butterfly_dct_post(x8.add(32), x8.add(32), x9.add(32), 4);
    butterfly_dct_post(x8.add(36), x8.add(36), x9.add(36), 4);
    butterfly_dct_post(x8.add(40), x8.add(40), x9.add(40), 4);
    butterfly_dct_post(x8.add(44), x8.add(44), x9.add(44), 4);
    butterfly_dct_post(x8.add(48), x8.add(48), x9.add(48), 4);
    butterfly_dct_post(x8.add(52), x8.add(52), x9.add(52), 4);
    butterfly_dct_post(x8.add(56), x8.add(56), x9.add(56), 4);
    butterfly_dct_post(x8.add(60), x8.add(60), x9.add(60), 4);

    // stage 10
    let mut x10_: [int32x4_t; 64] = core::mem::zeroed();
    let x10 = x10_.as_mut_ptr();
    *x10.add(0) = *x9.add(0);
    *x10.add(1) = *x9.add(1);
    *x10.add(2) = *x9.add(2);
    *x10.add(3) = *x9.add(3);
    *x10.add(4) = *x9.add(4);
    *x10.add(5) = *x9.add(5);
    *x10.add(6) = *x9.add(6);
    *x10.add(7) = *x9.add(7);
    *x10.add(8) = *x9.add(8);
    *x10.add(9) = *x9.add(9);
    *x10.add(10) = *x9.add(10);
    *x10.add(11) = *x9.add(11);
    *x10.add(12) = *x9.add(12);
    *x10.add(13) = *x9.add(13);
    *x10.add(14) = *x9.add(14);
    *x10.add(15) = *x9.add(15);
    *x10.add(16) = *x9.add(16);
    *x10.add(17) = *x9.add(17);
    *x10.add(18) = *x9.add(18);
    *x10.add(19) = *x9.add(19);
    *x10.add(20) = *x9.add(20);
    *x10.add(21) = *x9.add(21);
    *x10.add(22) = *x9.add(22);
    *x10.add(23) = *x9.add(23);
    *x10.add(24) = *x9.add(24);
    *x10.add(25) = *x9.add(25);
    *x10.add(26) = *x9.add(26);
    *x10.add(27) = *x9.add(27);
    *x10.add(28) = *x9.add(28);
    *x10.add(29) = *x9.add(29);
    *x10.add(30) = *x9.add(30);
    *x10.add(31) = *x9.add(31);
    butterfly_0112_neon(cospi, 1, *x9.add(63), *x9.add(32), x10.add(32), x10.add(63), v_cos_bit);
    butterfly_0130_neon(cospi, 31, *x9.add(33), *x9.add(62), x10.add(33), x10.add(62), v_cos_bit);
    butterfly_0112_neon(cospi, 17, *x9.add(61), *x9.add(34), x10.add(34), x10.add(61), v_cos_bit);
    butterfly_0130_neon(cospi, 15, *x9.add(35), *x9.add(60), x10.add(35), x10.add(60), v_cos_bit);
    butterfly_0112_neon(cospi, 9, *x9.add(59), *x9.add(36), x10.add(36), x10.add(59), v_cos_bit);
    butterfly_0130_neon(cospi, 23, *x9.add(37), *x9.add(58), x10.add(37), x10.add(58), v_cos_bit);
    butterfly_0112_neon(cospi, 25, *x9.add(57), *x9.add(38), x10.add(38), x10.add(57), v_cos_bit);
    butterfly_0130_neon(cospi, 7, *x9.add(39), *x9.add(56), x10.add(39), x10.add(56), v_cos_bit);
    butterfly_0112_neon(cospi, 5, *x9.add(55), *x9.add(40), x10.add(40), x10.add(55), v_cos_bit);
    butterfly_0130_neon(cospi, 27, *x9.add(41), *x9.add(54), x10.add(41), x10.add(54), v_cos_bit);
    butterfly_0112_neon(cospi, 21, *x9.add(53), *x9.add(42), x10.add(42), x10.add(53), v_cos_bit);
    butterfly_0130_neon(cospi, 11, *x9.add(43), *x9.add(52), x10.add(43), x10.add(52), v_cos_bit);
    butterfly_0112_neon(cospi, 13, *x9.add(51), *x9.add(44), x10.add(44), x10.add(51), v_cos_bit);
    butterfly_0130_neon(cospi, 19, *x9.add(45), *x9.add(50), x10.add(45), x10.add(50), v_cos_bit);
    butterfly_0112_neon(cospi, 29, *x9.add(49), *x9.add(46), x10.add(46), x10.add(49), v_cos_bit);
    butterfly_0130_neon(cospi, 3, *x9.add(47), *x9.add(48), x10.add(47), x10.add(48), v_cos_bit);

    // stage 11
    *output.add(0) = *x10.add(0);
    *output.add(1) = *x10.add(32);
    *output.add(2) = *x10.add(16);
    *output.add(3) = *x10.add(48);
    *output.add(4) = *x10.add(8);
    *output.add(5) = *x10.add(40);
    *output.add(6) = *x10.add(24);
    *output.add(7) = *x10.add(56);
    *output.add(8) = *x10.add(4);
    *output.add(9) = *x10.add(36);
    *output.add(10) = *x10.add(20);
    *output.add(11) = *x10.add(52);
    *output.add(12) = *x10.add(12);
    *output.add(13) = *x10.add(44);
    *output.add(14) = *x10.add(28);
    *output.add(15) = *x10.add(60);
    *output.add(16) = *x10.add(2);
    *output.add(17) = *x10.add(34);
    *output.add(18) = *x10.add(18);
    *output.add(19) = *x10.add(50);
    *output.add(20) = *x10.add(10);
    *output.add(21) = *x10.add(42);
    *output.add(22) = *x10.add(26);
    *output.add(23) = *x10.add(58);
    *output.add(24) = *x10.add(6);
    *output.add(25) = *x10.add(38);
    *output.add(26) = *x10.add(22);
    *output.add(27) = *x10.add(54);
    *output.add(28) = *x10.add(14);
    *output.add(29) = *x10.add(46);
    *output.add(30) = *x10.add(30);
    *output.add(31) = *x10.add(62);
    *output.add(32) = *x10.add(1);
    *output.add(33) = *x10.add(33);
    *output.add(34) = *x10.add(17);
    *output.add(35) = *x10.add(49);
    *output.add(36) = *x10.add(9);
    *output.add(37) = *x10.add(41);
    *output.add(38) = *x10.add(25);
    *output.add(39) = *x10.add(57);
    *output.add(40) = *x10.add(5);
    *output.add(41) = *x10.add(37);
    *output.add(42) = *x10.add(21);
    *output.add(43) = *x10.add(53);
    *output.add(44) = *x10.add(13);
    *output.add(45) = *x10.add(45);
    *output.add(46) = *x10.add(29);
    *output.add(47) = *x10.add(61);
    *output.add(48) = *x10.add(3);
    *output.add(49) = *x10.add(35);
    *output.add(50) = *x10.add(19);
    *output.add(51) = *x10.add(51);
    *output.add(52) = *x10.add(11);
    *output.add(53) = *x10.add(43);
    *output.add(54) = *x10.add(27);
    *output.add(55) = *x10.add(59);
    *output.add(56) = *x10.add(7);
    *output.add(57) = *x10.add(39);
    *output.add(58) = *x10.add(23);
    *output.add(59) = *x10.add(55);
    *output.add(60) = *x10.add(15);
    *output.add(61) = *x10.add(47);
    *output.add(62) = *x10.add(31);
    *output.add(63) = *x10.add(63);
}

unsafe fn highbd_fidentity32_x4_neon(input: *const int32x4_t, output: *mut int32x4_t,
                                     cos_bit: i32) {
    let _ = cos_bit;
    for i in 0..32 {
        *output.add(i) = vshlq_n_s32::<2>(*input.add(i));
    }
}

TRANSFORM_COL_MANY!(highbd_fdct32_col_many_neon, highbd_fdct32_x4_neon, load_buffer_4x32, 32);
TRANSFORM_COL_MANY!(highbd_fidentity32_col_many_neon, highbd_fidentity32_x4_neon,
                    load_buffer_4x32, 32);

static col_highbd_txfm32_x4_arr: [fwd_transform_1d_col_many_neon; TX_TYPES] = [
    Some(highbd_fdct32_col_many_neon),      // DCT_DCT
    None,                                   // ADST_DCT
    None,                                   // DCT_ADST
    None,                                   // ADST_ADST
    None,                                   // FLIPADST_DCT
    None,                                   // DCT_FLIPADST
    None,                                   // FLIPADST_FLIPADST
    None,                                   // ADST_FLIPADST
    None,                                   // FLIPADST_ADST
    Some(highbd_fidentity32_col_many_neon), // IDTX
    None,                                   // V_DCT
    None,                                   // H_DCT
    None,                                   // V_ADST
    None,                                   // H_ADST
    None,                                   // V_FLIPADST
    None,                                   // H_FLIPADST
];

TRANSFORM_ROW_MANY!(highbd_fdct32_row_many_neon, highbd_fdct32_x4_neon, store_buffer_32x4, 32);
TRANSFORM_ROW_MANY!(highbd_fidentity32_row_many_neon, highbd_fidentity32_x4_neon,
                    store_buffer_32x4, 32);

static row_highbd_txfm32_x4_arr: [fwd_transform_1d_row_many_neon; TX_TYPES] = [
    Some(highbd_fdct32_row_many_neon),      // DCT_DCT
    None,                                   // ADST_DCT
    None,                                   // DCT_ADST
    None,                                   // ADST_ADST
    None,                                   // FLIPADST_DCT
    None,                                   // DCT_FLIPADST
    None,                                   // FLIPADST_FLIPADST
    None,                                   // ADST_FLIPADST
    None,                                   // FLIPADST_ADST
    Some(highbd_fidentity32_row_many_neon), // IDTX
    None,                                   // V_DCT
    None,                                   // H_DCT
    None,                                   // V_ADST
    None,                                   // H_ADST
    None,                                   // V_FLIPADST
    None,                                   // H_FLIPADST
];

TRANSFORM_ROW_RECT_MANY!(highbd_fdct32_row_rect_many_neon, highbd_fdct32_x4_neon,
                         store_buffer_32x4, 32);
TRANSFORM_ROW_RECT_MANY!(highbd_fidentity32_row_rect_many_neon, highbd_fidentity32_x4_neon,
                         store_buffer_32x4, 32);

static row_rect_highbd_txfm32_x4_arr: [fwd_transform_1d_row_many_neon; TX_TYPES] = [
    Some(highbd_fdct32_row_rect_many_neon),      // DCT_DCT
    None,                                        // ADST_DCT
    None,                                        // DCT_ADST
    None,                                        // ADST_ADST
    None,                                        // FLIPADST_DCT
    None,                                        // DCT_FLIPADST
    None,                                        // FLIPADST_FLIPADST
    None,                                        // ADST_FLIPADST
    None,                                        // FLIPADST_ADST
    Some(highbd_fidentity32_row_rect_many_neon), // IDTX
    None,                                        // V_DCT
    None,                                        // H_DCT
    None,                                        // V_ADST
    None,                                        // H_ADST
    None,                                        // V_FLIPADST
    None,                                        // H_FLIPADST
];

// The C code calls the table entries unconditionally; a NULL entry (a
// transform type the size does not support) would crash there and panics here.

#[no_mangle]
pub unsafe extern "C-unwind" fn av1_fwd_txfm2d_16x8_neon(mut input: *const i16, coeff: *mut i32,
                                                         mut stride: i32, tx_type: TX_TYPE,
                                                         bd: i32) {
    let _ = bd;
    let col_txfm = col_highbd_txfm8_xn_arr[tx_type as usize].unwrap();
    let row_txfm = row_rect_highbd_txfm16_xn_arr[tx_type as usize].unwrap();
    let bit = av1_fwd_cos_bit_col[2][1] as i32;

    let mut ud_flip: i32 = 0;
    let mut lr_flip: i32 = 0;
    get_flip_cfg(tx_type, &mut ud_flip, &mut lr_flip);
    ud_adjust_input_and_stride(ud_flip, &mut input, &mut stride, 8);

    // Column-wise transform.
    let mut buf0_: [int32x4_t; 32] = core::mem::zeroed();
    let buf0 = buf0_.as_mut_ptr();
    if lr_flip != 0 {
        col_txfm(input, buf0.add(3 * 8), stride, bit, /*lr_flip=*/ 1, /*howmany=*/ 4,
                 /*hm_stride=*/ -8);
    } else {
        col_txfm(input, buf0, stride, bit, /*lr_flip=*/ 0, /*howmany=*/ 4, /*hm_stride=*/ 8);
    }
    shift_right_2_round_s32_x4(buf0, buf0, 32);

    let mut buf1_: [int32x4_t; 32] = core::mem::zeroed();
    let buf1 = buf1_.as_mut_ptr();
    transpose_arrays_s32_16x8(buf0, buf1);

    // Row-wise transform.
    row_txfm(buf1, coeff, bit, /*howmany=*/ 2, /*hm_stride=*/ 16, /*stride=*/ 8);
}

#[no_mangle]
pub unsafe extern "C-unwind" fn av1_fwd_txfm2d_8x16_neon(mut input: *const i16, coeff: *mut i32,
                                                         mut stride: i32, tx_type: TX_TYPE,
                                                         bd: i32) {
    let _ = bd;
    let col_txfm = col_highbd_txfm16_xn_arr[tx_type as usize].unwrap();
    let row_txfm = row_rect_highbd_txfm8_xn_arr[tx_type as usize].unwrap();
    let bit = av1_fwd_cos_bit_col[1][2] as i32;

    let mut ud_flip: i32 = 0;
    let mut lr_flip: i32 = 0;
    get_flip_cfg(tx_type, &mut ud_flip, &mut lr_flip);
    ud_adjust_input_and_stride(ud_flip, &mut input, &mut stride, 16);

    // Column-wise transform.
    let mut buf0_: [int32x4_t; 32] = core::mem::zeroed();
    let buf0 = buf0_.as_mut_ptr();
    if lr_flip != 0 {
        col_txfm(input, buf0.add(16), stride, bit, /*lr_flip=*/ 1, /*howmany=*/ 2,
                 /*hm_stride=*/ -16);
    } else {
        col_txfm(input, buf0, stride, bit, /*lr_flip=*/ 0, /*howmany=*/ 2, /*hm_stride=*/ 16);
    }
    shift_right_2_round_s32_x4(buf0, buf0, 32);

    let mut buf1_: [int32x4_t; 32] = core::mem::zeroed();
    let buf1 = buf1_.as_mut_ptr();
    transpose_arrays_s32_8x16(buf0, buf1);

    // Row-wise transform.
    row_txfm(buf1, coeff, bit, /*howmany=*/ 4, /*hm_stride=*/ 8, /*stride=*/ 16);
}

// !CONFIG_REALTIME_ONLY
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_fwd_txfm2d_4x16_neon(mut input: *const i16, coeff: *mut i32,
                                                         mut stride: i32, tx_type: TX_TYPE,
                                                         bd: i32) {
    let _ = bd;
    let bitcol = av1_fwd_cos_bit_col[0][2] as i32;
    let bitrow = av1_fwd_cos_bit_row[0][2] as i32;
    let col_txfm = col_highbd_txfm16_xn_arr[tx_type as usize].unwrap();
    let row_txfm = row_highbd_txfm4_xn_arr[tx_type as usize].unwrap();

    let mut ud_flip: i32 = 0;
    let mut lr_flip: i32 = 0;
    get_flip_cfg(tx_type, &mut ud_flip, &mut lr_flip);
    ud_adjust_input_and_stride(ud_flip, &mut input, &mut stride, 16);

    // Column-wise transform.
    let mut buf0_: [int32x4_t; 16] = core::mem::zeroed();
    let buf0 = buf0_.as_mut_ptr();
    if lr_flip != 0 {
        col_txfm(input, buf0, stride, bitcol, /*lr_flip=*/ 1, /*howmany=*/ 1, /*hm_stride=*/ 0);
    } else {
        col_txfm(input, buf0, stride, bitcol, /*lr_flip=*/ 0, /*howmany=*/ 1, /*hm_stride=*/ 0);
    }
    shift_right_1_round_s32_x4(buf0, buf0, 16);

    let mut buf1_: [int32x4_t; 16] = core::mem::zeroed();
    let buf1 = buf1_.as_mut_ptr();
    transpose_arrays_s32_4x16(buf0, buf1);

    // Row-wise transform.
    row_txfm(buf1, coeff, bitrow, /*howmany=*/ 4, /*hm_stride=*/ 4, /*stride=*/ 16);
}

#[no_mangle]
pub unsafe extern "C-unwind" fn av1_fwd_txfm2d_16x4_neon(mut input: *const i16, coeff: *mut i32,
                                                         mut stride: i32, tx_type: TX_TYPE,
                                                         bd: i32) {
    let _ = bd;
    let bitcol = av1_fwd_cos_bit_col[2][0] as i32;
    let bitrow = av1_fwd_cos_bit_row[2][0] as i32;
    let col_txfm = col_highbd_txfm4_xn_arr[tx_type as usize].unwrap();
    let row_txfm = row_highbd_txfm16_xn_arr[tx_type as usize].unwrap();

    let mut ud_flip: i32 = 0;
    let mut lr_flip: i32 = 0;
    get_flip_cfg(tx_type, &mut ud_flip, &mut lr_flip);
    ud_adjust_input_and_stride(ud_flip, &mut input, &mut stride, 4);

    // Column-wise transform.
    let mut buf0_: [int32x4_t; 16] = core::mem::zeroed();
    let buf0 = buf0_.as_mut_ptr();
    if lr_flip != 0 {
        col_txfm(input, buf0.add(3 * 4), stride, bitcol, /*lr_flip=*/ 1, /*howmany=*/ 4,
                 /*hm_stride=*/ -4);
    } else {
        col_txfm(input, buf0, stride, bitcol, /*lr_flip=*/ 0, /*howmany=*/ 4, /*hm_stride=*/ 4);
    }

    shift_right_1_round_s32_x4(buf0, buf0, 16);
    transpose_arrays_s32_4x16(buf0, buf0);

    // Row-wise transform.
    row_txfm(buf0, coeff, bitrow, /*stride=*/ 4);
}

#[no_mangle]
pub unsafe extern "C-unwind" fn av1_fwd_txfm2d_16x32_neon(input: *const i16, coeff: *mut i32,
                                                          stride: i32, tx_type: TX_TYPE,
                                                          bd: i32) {
    let _ = bd;
    let col_txfm = col_highbd_txfm32_x4_arr[tx_type as usize].unwrap();
    let row_txfm = row_rect_highbd_txfm16_xn_arr[tx_type as usize].unwrap();
    let bitcol = av1_fwd_cos_bit_col[2][3] as i32;
    let bitrow = av1_fwd_cos_bit_row[2][3] as i32;

    // Column-wise transform.
    let mut buf0_: [int32x4_t; 128] = core::mem::zeroed();
    let buf0 = buf0_.as_mut_ptr();
    col_txfm(input, buf0, stride, bitcol, /*lr_flip=*/ 0, /*howmany=*/ 4, /*hm_stride=*/ 32);
    shift_right_4_round_s32_x4(buf0, buf0, 128);

    let mut buf1_: [int32x4_t; 128] = core::mem::zeroed();
    let buf1 = buf1_.as_mut_ptr();
    transpose_arrays_s32_16x32(buf0, buf1);

    // Row-wise transform.
    row_txfm(buf1, coeff, bitrow, /*howmany=*/ 8, /*hm_stride=*/ 16, /*stride=*/ 32);
}

#[no_mangle]
pub unsafe extern "C-unwind" fn av1_fwd_txfm2d_32x64_neon(input: *const i16, coeff: *mut i32,
                                                          stride: i32, tx_type: TX_TYPE,
                                                          bd: i32) {
    let _ = bd;
    let _ = tx_type;
    let bitcol = av1_fwd_cos_bit_col[3][4] as i32;
    let bitrow = av1_fwd_cos_bit_row[3][4] as i32;

    // Column-wise transform.
    let mut buf0_: [int32x4_t; 512] = core::mem::zeroed();
    let buf0 = buf0_.as_mut_ptr();
    load_buffer_32x64(input, buf0, stride, 0);
    for i in 0..8 {
        highbd_fdct64_x4_neon(buf0.add(i * 64), buf0.add(i * 64), bitcol as i8);
    }
    shift_right_2_round_s32_x4(buf0, buf0, 512);

    let mut buf1_: [int32x4_t; 512] = core::mem::zeroed();
    let buf1 = buf1_.as_mut_ptr();
    transpose_arrays_s32_32x64(buf0, buf1);

    // Row-wise transform.
    for i in 0..16 {
        highbd_fdct32_x4_neon(buf1.add(i * 32), buf1.add(i * 32), bitrow);
    }
    round_shift2_rect_array_s32_neon(buf1, buf1, 512);
    store_buffer_32x32(buf1, coeff, /*stride=*/ 32);
}

#[no_mangle]
pub unsafe extern "C-unwind" fn av1_fwd_txfm2d_64x32_neon(input: *const i16, coeff: *mut i32,
                                                          stride: i32, tx_type: TX_TYPE,
                                                          bd: i32) {
    let _ = bd;
    let _ = tx_type;
    let bitcol = av1_fwd_cos_bit_col[4][3] as i32;
    let bitrow = av1_fwd_cos_bit_row[4][3] as i32;

    // Column-wise transform.
    let mut buf0_: [int32x4_t; 512] = core::mem::zeroed();
    let buf0 = buf0_.as_mut_ptr();
    load_buffer_64x32(input, buf0, stride, 0);
    for i in 0..16 {
        highbd_fdct32_x4_neon(buf0.add(i * 32), buf0.add(i * 32), bitcol);
    }
    shift_right_4_round_s32_x4(buf0, buf0, 512);

    let mut buf1_: [int32x4_t; 512] = core::mem::zeroed();
    let buf1 = buf1_.as_mut_ptr();
    transpose_arrays_s32_64x32(buf0, buf1);

    // Row-wise transform.
    for i in 0..8 {
        highbd_fdct64_x4_neon(buf1.add(i * 64), buf1.add(i * 64), bitrow as i8);
    }
    round_shift2_rect_array_s32_neon(buf1, buf1, 512);
    store_buffer_64x32(buf1, coeff, /*stride=*/ 32);
}

#[no_mangle]
pub unsafe extern "C-unwind" fn av1_fwd_txfm2d_32x16_neon(input: *const i16, coeff: *mut i32,
                                                          stride: i32, tx_type: TX_TYPE,
                                                          bd: i32) {
    let _ = bd;
    let col_txfm = col_highbd_txfm16_xn_arr[tx_type as usize].unwrap();
    let row_txfm = row_rect_highbd_txfm32_x4_arr[tx_type as usize].unwrap();
    let bitcol = av1_fwd_cos_bit_col[3][2] as i32;
    let bitrow = av1_fwd_cos_bit_row[3][2] as i32;

    // Column-wise transform.
    let mut buf0_: [int32x4_t; 128] = core::mem::zeroed();
    let buf0 = buf0_.as_mut_ptr();
    col_txfm(input, buf0, stride, bitcol, /*lr_flip=*/ 0, /*howmany=*/ 8, /*hm_stride=*/ 16);
    shift_right_4_round_s32_x4(buf0, buf0, 128);

    let mut buf1_: [int32x4_t; 128] = core::mem::zeroed();
    let buf1 = buf1_.as_mut_ptr();
    transpose_arrays_s32_32x16(buf0, buf1);

    // Row-wise transform.
    row_txfm(buf1, coeff, bitrow, /*howmany=*/ 4, /*hm_stride=*/ 32, /*stride=*/ 16);
}

// !CONFIG_REALTIME_ONLY
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_fwd_txfm2d_8x32_neon(input: *const i16, coeff: *mut i32,
                                                         stride: i32, tx_type: TX_TYPE,
                                                         bd: i32) {
    let _ = bd;
    let col_txfm = col_highbd_txfm32_x4_arr[tx_type as usize].unwrap();
    let row_txfm = row_highbd_txfm8_xn_arr[tx_type as usize].unwrap();
    let bitcol = av1_fwd_cos_bit_col[1][3] as i32;
    let bitrow = av1_fwd_cos_bit_row[1][3] as i32;

    // Column-wise transform.
    let mut buf0_: [int32x4_t; 64] = core::mem::zeroed();
    let buf0 = buf0_.as_mut_ptr();
    col_txfm(input, buf0, stride, bitcol, /*lr_flip=*/ 0, /*howmany=*/ 2, /*hm_stride=*/ 32);
    shift_right_2_round_s32_x4(buf0, buf0, 64);

    let mut buf1_: [int32x4_t; 64] = core::mem::zeroed();
    let buf1 = buf1_.as_mut_ptr();
    transpose_arrays_s32_8x32(buf0, buf1);

    // Row-wise transform.
    row_txfm(buf1, coeff, bitrow, /*howmany=*/ 8, /*hm_stride=*/ 8, /*stride=*/ 32);
}

#[no_mangle]
pub unsafe extern "C-unwind" fn av1_fwd_txfm2d_32x8_neon(input: *const i16, coeff: *mut i32,
                                                         stride: i32, tx_type: TX_TYPE,
                                                         bd: i32) {
    let _ = bd;
    let col_txfm = col_highbd_txfm8_xn_arr[tx_type as usize].unwrap();
    let row_txfm = row_highbd_txfm32_x4_arr[tx_type as usize].unwrap();
    let bitcol = av1_fwd_cos_bit_col[3][1] as i32;
    let bitrow = av1_fwd_cos_bit_row[3][1] as i32;

    // Column-wise transform.
    let mut buf0_: [int32x4_t; 64] = core::mem::zeroed();
    let buf0 = buf0_.as_mut_ptr();
    col_txfm(input, buf0, stride, bitcol, /*lr_flip=*/ 0, /*howmany=*/ 8, /*hm_stride=*/ 8);
    shift_right_2_round_s32_x4(buf0, buf0, 64);

    let mut buf1_: [int32x4_t; 64] = core::mem::zeroed();
    let buf1 = buf1_.as_mut_ptr();
    transpose_arrays_s32_32x8(buf0, buf1);

    // Row-wise transform.
    row_txfm(buf1, coeff, bitrow, /*howmany=*/ 2, /*hm_stride=*/ 32, /*stride=*/ 8);
}

#[no_mangle]
pub unsafe extern "C-unwind" fn av1_fwd_txfm2d_4x8_neon(mut input: *const i16, coeff: *mut i32,
                                                        mut stride: i32, tx_type: TX_TYPE,
                                                        bd: i32) {
    let _ = bd;
    let bitcol = av1_fwd_cos_bit_col[0][1] as i32;
    let bitrow = av1_fwd_cos_bit_row[0][1] as i32;
    let col_txfm = col_highbd_txfm8_x4_arr[tx_type as usize].unwrap();
    let row_txfm = row_rect_highbd_txfm4_xn_arr[tx_type as usize].unwrap();

    let mut ud_flip: i32 = 0;
    let mut lr_flip: i32 = 0;
    get_flip_cfg(tx_type, &mut ud_flip, &mut lr_flip);
    ud_adjust_input_and_stride(ud_flip, &mut input, &mut stride, 8);

    // Column-wise transform.
    let mut buf0_: [int32x4_t; 8] = core::mem::zeroed();
    let buf0 = buf0_.as_mut_ptr();
    col_txfm(input, buf0, stride, bitcol, lr_flip);
    shift_right_1_round_s32_x4(buf0, buf0, 8);

    let mut buf1_: [int32x4_t; 8] = core::mem::zeroed();
    let buf1 = buf1_.as_mut_ptr();
    transpose_arrays_s32_4x8(buf0, buf1);

    // Row-wise transform.
    row_txfm(buf1, coeff, bitrow, /*howmany=*/ 2, /*hm_stride=*/ 4, /*stride=*/ 8);
}

#[no_mangle]
pub unsafe extern "C-unwind" fn av1_fwd_txfm2d_8x4_neon(mut input: *const i16, coeff: *mut i32,
                                                        mut stride: i32, tx_type: TX_TYPE,
                                                        bd: i32) {
    let _ = bd;
    let bitcol = av1_fwd_cos_bit_col[1][0] as i32;
    let bitrow = av1_fwd_cos_bit_row[1][0] as i32;
    let col_txfm = col_highbd_txfm4_xn_arr[tx_type as usize].unwrap();
    let row_txfm = row_highbd_txfm8_x4_arr[tx_type as usize].unwrap();

    let mut ud_flip: i32 = 0;
    let mut lr_flip: i32 = 0;
    get_flip_cfg(tx_type, &mut ud_flip, &mut lr_flip);
    ud_adjust_input_and_stride(ud_flip, &mut input, &mut stride, 4);

    // Column-wise transform.
    let mut buf0_: [int32x4_t; 8] = core::mem::zeroed();
    let buf0 = buf0_.as_mut_ptr();
    if lr_flip != 0 {
        col_txfm(input, buf0.add(4), stride, bitcol, /*lr_flip=*/ 1, /*howmany=*/ 2,
                 /*hm_stride=*/ -4);
    } else {
        col_txfm(input, buf0, stride, bitcol, /*lr_flip=*/ 0, /*howmany=*/ 2, /*hm_stride=*/ 4);
    }

    shift_right_1_round_s32_x4(buf0, buf0, 8);

    let mut buf1_: [int32x4_t; 8] = core::mem::zeroed();
    let buf1 = buf1_.as_mut_ptr();
    transpose_arrays_s32_8x4(buf0, buf1);

    // Row-wise transform.
    row_txfm(buf1, coeff, bitrow, /*stride=*/ 4);
}

// !CONFIG_REALTIME_ONLY
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_fwd_txfm2d_16x64_neon(mut input: *const i16, coeff: *mut i32,
                                                          mut stride: i32, tx_type: TX_TYPE,
                                                          bd: i32) {
    let _ = bd;
    let bitcol = av1_fwd_cos_bit_col[2][4] as i32;
    let bitrow = av1_fwd_cos_bit_row[2][4] as i32;

    let mut ud_flip: i32 = 0;
    let mut lr_flip: i32 = 0;
    get_flip_cfg(tx_type, &mut ud_flip, &mut lr_flip);
    ud_adjust_input_and_stride(ud_flip, &mut input, &mut stride, 64);

    // Column-wise transform.
    let mut buf0_: [int32x4_t; 256] = core::mem::zeroed();
    let buf0 = buf0_.as_mut_ptr();
    load_buffer_16x64(input, buf0, stride, lr_flip);
    for i in 0..4 {
        highbd_fdct64_x4_neon(buf0.add(i * 64), buf0.add(i * 64), bitcol as i8);
    }
    shift_right_2_round_s32_x4(buf0, buf0, 256);

    let mut buf1_: [int32x4_t; 256] = core::mem::zeroed();
    let buf1 = buf1_.as_mut_ptr();
    transpose_arrays_s32_16x64(buf0, buf1);

    // Row-wise transform.
    highbd_fdct16_xn_neon(buf1, buf1, bitrow, 8);
    store_buffer_16x32(buf1, coeff, /*stride=*/ 32);
}

#[no_mangle]
pub unsafe extern "C-unwind" fn av1_fwd_txfm2d_64x16_neon(mut input: *const i16, coeff: *mut i32,
                                                          mut stride: i32, tx_type: TX_TYPE,
                                                          bd: i32) {
    let _ = bd;
    let bitcol = av1_fwd_cos_bit_col[4][2] as i32;
    let bitrow = av1_fwd_cos_bit_row[4][2] as i32;

    let mut ud_flip: i32 = 0;
    let mut lr_flip: i32 = 0;
    get_flip_cfg(tx_type, &mut ud_flip, &mut lr_flip);
    ud_adjust_input_and_stride(ud_flip, &mut input, &mut stride, 16);

    // Column-wise transform.
    let mut buf0_: [int32x4_t; 256] = core::mem::zeroed();
    let buf0 = buf0_.as_mut_ptr();
    load_buffer_64x16(input, buf0, stride, lr_flip);
    highbd_fdct16_xn_neon(buf0, buf0, bitcol, 16);
    shift_right_4_round_s32_x4(buf0, buf0, 256);

    let mut buf1_: [int32x4_t; 256] = core::mem::zeroed();
    let buf1 = buf1_.as_mut_ptr();
    transpose_arrays_s32_64x16(buf0, buf1);

    // Row-wise transform.
    for i in 0..4 {
        highbd_fdct64_x4_neon(buf1.add(i * 64), buf1.add(i * 64), bitrow as i8);
    }
    store_buffer_64x16(buf1, coeff, /*stride=*/ 16);
    core::ptr::write_bytes(coeff.add(16 * 32), 0, 16 * 32);
}

#[no_mangle]
pub unsafe extern "C-unwind" fn av1_fwd_txfm2d_32x32_neon(input: *const i16, output: *mut i32,
                                                          stride: i32, tx_type: TX_TYPE,
                                                          bd: i32) {
    let _ = bd;
    let col_txfm = col_highbd_txfm32_x4_arr[tx_type as usize].unwrap();
    let row_txfm = row_highbd_txfm32_x4_arr[tx_type as usize].unwrap();

    // Column-wise transform.
    let mut buf0_: [int32x4_t; 256] = core::mem::zeroed();
    let buf0 = buf0_.as_mut_ptr();
    col_txfm(input, buf0, stride, /*cos_bit=*/ 12, /*lr_flip=*/ 0, /*howmany=*/ 8,
             /*hm_stride=*/ 32);
    shift_right_4_round_s32_x4(buf0, buf0, 256);

    let mut buf1_: [int32x4_t; 256] = core::mem::zeroed();
    let buf1 = buf1_.as_mut_ptr();
    transpose_arrays_s32_32x32(buf0, buf1);

    // Row-wise transform.
    row_txfm(buf1, output, /*cos_bit=*/ 12, /*howmany=*/ 8, /*hm_stride=*/ 32,
             /*stride=*/ 32);
}

#[no_mangle]
pub unsafe extern "C-unwind" fn av1_fwd_txfm2d_64x64_neon(input: *const i16, output: *mut i32,
                                                          stride: i32, tx_type: TX_TYPE,
                                                          bd: i32) {
    let _ = bd;
    let _ = tx_type;

    // Column-wise transform.
    let mut buf0_: [int32x4_t; 1024] = core::mem::zeroed();
    let buf0 = buf0_.as_mut_ptr();
    load_buffer_64x64(input, buf0, stride, 0);
    for col in 0..16 {
        highbd_fdct64_x4_neon(buf0.add(col * 64), buf0.add(col * 64), 13);
    }
    shift_right_2_round_s32_x4(buf0, buf0, 1024);

    let mut buf1_: [int32x4_t; 1024] = core::mem::zeroed();
    let buf1 = buf1_.as_mut_ptr();
    transpose_arrays_s32_64x64(buf0, buf1);

    // Row-wise transform.
    for col in 0..8 {
        highbd_fdct64_x4_neon(buf1.add(col * 64), buf1.add(col * 64), 10);
    }
    shift_right_2_round_s32_x4(buf1, buf1, 512);
    store_buffer_64x32(buf1, output, /*stride=*/ 32);
}
