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

// Translated by hand from upstream/aom/av1/encoder/arm/av1_fwd_txfm2d_neon.c
// (ADR 0056). Intrinsic sequences, helper structure, and names follow the
// original; only the AOM_ARCH_AARCH64 branches are retained. C locals that the
// original leaves uninitialized start zeroed here. The function-generating
// macros (ROUND_SHIFT_SQRT_LOOP_HELPER, TRANSFORM_COL, TRANSFORM_ROW,
// TRANSFORM_ROW_RECT) become macro_rules! that emit the same named functions,
// and the function-pointer tables hold Option values so that their NULL
// entries keep the original's checks.
#![allow(non_snake_case, non_upper_case_globals, non_camel_case_types, unused_assignments)]
#![allow(unused_mut, unused_parens, unused_variables, dead_code, clippy::all)]

use core::arch::aarch64::*;

use crate::av1__common__idct::TxfmParam;
use crate::simd::aom_dsp__arm__transpose_neon::*;
use crate::simd::av1__common__av1_txfm::{cospi_arr_q13, sinpi_arr_q13};
use crate::simd::av1__encoder__arm__shift_neon::*;
use crate::simd::av1__encoder__arm__txfm_neon::ud_adjust_input_and_stride;
use crate::simd::types::tran_low_t;

extern "C-unwind" {
    fn av1_fwd_txfm2d_16x32_c(input: *const i16, output: *mut i32, stride: i32, tx_type: TX_TYPE,
                              bd: i32);
    fn av1_fwd_txfm2d_32x16_c(input: *const i16, output: *mut i32, stride: i32, tx_type: TX_TYPE,
                              bd: i32);
    fn av1_fwd_txfm2d_32x32_c(input: *const i16, output: *mut i32, stride: i32, tx_type: TX_TYPE,
                              bd: i32);
    fn av1_lowbd_fwd_txfm_c(src_diff: *const i16, coeff: *mut tran_low_t, diff_stride: i32,
                            txfm_param: *mut TxfmParam);
}

// ---------------------------------------------------------------------------
// Header items used by the original translation unit.

// av1/common/enums.h
type TX_TYPE = u8;
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
const TX_4X4: u8 = 0;
const TX_SIZES_ALL: usize = 19;

// av1/common/av1_txfm.h
const NewSqrt2Bits: i32 = 12;
// 2^12 * sqrt(2)
const NewSqrt2: i32 = 5793;

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

// av1/encoder/av1_fwd_txfm2d.c (FwdTxfm2dFunc, av1/common/av1_txfm.h)
type FwdTxfm2dFunc = Option<unsafe fn(*const i16, *mut i32, i32, TX_TYPE, i32)>;

// ---------------------------------------------------------------------------

const TXFM_COS_BIT_MAX: i32 = 13;

// A note on butterfly helper naming:
//
// butterfly_[input_ty]_[acc_ty]_[input_num]_[weight_num]_[weight_neg]_neon
// e.g. butterfly_s32_s32_x4_0231_neon
//                |   |   |  ^ Weights are applied as indices 0, 2, 3, 1
//                |   |   |    (see more detail below)
//                |   |   ^ (int32)x4 input/output parameters
//                |   ^ 32-bit accumulators internally
//                ^ 32-bit input/output parameters
//
// Weights are stored as 4-tuples in Q2.13 format as (w0, 1-w0, -w0, w0-1) to
// avoid needing separate negation instructions. This is represented in the
// helper naming by referring to the lane index in the loaded tuple that each
// multiply is performed with:
//
//        in0  in1
//      /----------
// out0 |  w0   w1   ==>  out0 = in0 * w0 + in1 * w1
// out1 |  w2   w3   ==>  out1 = in0 * w2 + in1 * w3
//
// So for indices 0331 from the earlier example, we end up with:
//
//          in0       in1
//      /------------------
// out0 | (lane 0) (lane 2)   ==>  out0 = in0 *   w0   + in1 *  -w0
// out1 | (lane 3) (lane 1)   ==>  out1 = in0 * (w0-1) + in1 * (1-w0)

#[inline(always)]
unsafe fn butterfly_s32_s32_x4_0112_neon(w0101_s16: int16x4_t, in0: int32x4_t, in1: int32x4_t,
                                         out0: *mut int32x4_t, out1: *mut int32x4_t) {
    let w0101: int32x4_t = vmovl_s16(w0101_s16);
    let mut o0: int32x4_t = vmulq_lane_s32::<0>(in0, vget_low_s32(w0101));
    o0 = vmlaq_lane_s32::<1>(o0, in1, vget_low_s32(w0101));
    let mut o1: int32x4_t = vmulq_lane_s32::<1>(in0, vget_low_s32(w0101));
    o1 = vmlaq_lane_s32::<0>(o1, in1, vget_high_s32(w0101));
    *out0 = vrshrq_n_s32::<TXFM_COS_BIT_MAX>(o0);
    *out1 = vrshrq_n_s32::<TXFM_COS_BIT_MAX>(o1);
}

#[inline(always)]
unsafe fn butterfly_s32_s32_x4_0332_neon(w0101_s16: int16x4_t, in0: int32x4_t, in1: int32x4_t,
                                         out0: *mut int32x4_t, out1: *mut int32x4_t) {
    let w0101: int32x4_t = vmovl_s16(w0101_s16);
    let mut o0: int32x4_t = vmulq_lane_s32::<0>(in0, vget_low_s32(w0101));
    o0 = vmlaq_lane_s32::<1>(o0, in1, vget_high_s32(w0101));
    let mut o1: int32x4_t = vmulq_lane_s32::<1>(in0, vget_high_s32(w0101));
    o1 = vmlaq_lane_s32::<0>(o1, in1, vget_high_s32(w0101));
    *out0 = vrshrq_n_s32::<TXFM_COS_BIT_MAX>(o0);
    *out1 = vrshrq_n_s32::<TXFM_COS_BIT_MAX>(o1);
}

#[inline(always)]
unsafe fn butterfly_s32_s32_x4_1003_neon(w0101_s16: int16x4_t, in0: int32x4_t, in1: int32x4_t,
                                         out0: *mut int32x4_t, out1: *mut int32x4_t) {
    let w0101: int32x4_t = vmovl_s16(w0101_s16);
    let mut o0: int32x4_t = vmulq_lane_s32::<1>(in0, vget_low_s32(w0101));
    o0 = vmlaq_lane_s32::<0>(o0, in1, vget_low_s32(w0101));
    let mut o1: int32x4_t = vmulq_lane_s32::<0>(in0, vget_low_s32(w0101));
    o1 = vmlaq_lane_s32::<1>(o1, in1, vget_high_s32(w0101));
    *out0 = vrshrq_n_s32::<TXFM_COS_BIT_MAX>(o0);
    *out1 = vrshrq_n_s32::<TXFM_COS_BIT_MAX>(o1);
}

#[inline(always)]
unsafe fn butterfly_s32_s32_x4_1223_neon(w0101_s16: int16x4_t, in0: int32x4_t, in1: int32x4_t,
                                         out0: *mut int32x4_t, out1: *mut int32x4_t) {
    let w0101: int32x4_t = vmovl_s16(w0101_s16);
    let mut o0: int32x4_t = vmulq_lane_s32::<1>(in0, vget_low_s32(w0101));
    o0 = vmlaq_lane_s32::<0>(o0, in1, vget_high_s32(w0101));
    let mut o1: int32x4_t = vmulq_lane_s32::<0>(in0, vget_high_s32(w0101));
    o1 = vmlaq_lane_s32::<1>(o1, in1, vget_high_s32(w0101));
    *out0 = vrshrq_n_s32::<TXFM_COS_BIT_MAX>(o0);
    *out1 = vrshrq_n_s32::<TXFM_COS_BIT_MAX>(o1);
}

macro_rules! butterfly_s16_s32_x4_neon {
    ($wvec:expr, $lane0:literal, $lane1:literal, $lane2:literal, $lane3:literal, $in0:expr,
     $in1:expr, $out0:expr, $out1:expr) => {{
        let mut u0: int32x4_t = vmull_lane_s16::<$lane0>($in0, $wvec);
        u0 = vmlal_lane_s16::<$lane1>(u0, $in1, $wvec);
        let mut v0: int32x4_t = vmull_lane_s16::<$lane2>($in0, $wvec);
        v0 = vmlal_lane_s16::<$lane3>(v0, $in1, $wvec);
        *$out0 = vqrshrn_n_s32::<TXFM_COS_BIT_MAX>(u0);
        *$out1 = vqrshrn_n_s32::<TXFM_COS_BIT_MAX>(v0);
    }};
}

#[inline(always)]
unsafe fn butterfly_s16_s32_x4_0112_neon(w0101: int16x4_t, in0: int16x4_t, in1: int16x4_t,
                                         out0: *mut int16x4_t, out1: *mut int16x4_t) {
    butterfly_s16_s32_x4_neon!(w0101, 0, 1, 1, 2, in0, in1, out0, out1);
}

#[inline(always)]
unsafe fn butterfly_s16_s32_x4_0332_neon(w0101: int16x4_t, in0: int16x4_t, in1: int16x4_t,
                                         out0: *mut int16x4_t, out1: *mut int16x4_t) {
    butterfly_s16_s32_x4_neon!(w0101, 0, 3, 3, 2, in0, in1, out0, out1);
}

#[inline(always)]
unsafe fn butterfly_s16_s32_x4_1003_neon(w0101: int16x4_t, in0: int16x4_t, in1: int16x4_t,
                                         out0: *mut int16x4_t, out1: *mut int16x4_t) {
    butterfly_s16_s32_x4_neon!(w0101, 1, 0, 0, 3, in0, in1, out0, out1);
}

#[inline(always)]
unsafe fn butterfly_s16_s32_x4_1223_neon(w0101: int16x4_t, in0: int16x4_t, in1: int16x4_t,
                                         out0: *mut int16x4_t, out1: *mut int16x4_t) {
    butterfly_s16_s32_x4_neon!(w0101, 1, 2, 2, 3, in0, in1, out0, out1);
}

macro_rules! butterfly_s16_s32_x8_neon {
    ($wvec:expr, $lane0:literal, $lane1:literal, $lane2:literal, $lane3:literal, $in0:expr,
     $in1:expr, $out0:expr, $out1:expr) => {{
        let mut u0: int32x4_t = vmull_lane_s16::<$lane0>(vget_low_s16($in0), $wvec);
        u0 = vmlal_lane_s16::<$lane1>(u0, vget_low_s16($in1), $wvec);
        let mut u1: int32x4_t = vmull_lane_s16::<$lane0>(vget_high_s16($in0), $wvec);
        u1 = vmlal_lane_s16::<$lane1>(u1, vget_high_s16($in1), $wvec);
        let mut v0: int32x4_t = vmull_lane_s16::<$lane2>(vget_low_s16($in0), $wvec);
        v0 = vmlal_lane_s16::<$lane3>(v0, vget_low_s16($in1), $wvec);
        let mut v1: int32x4_t = vmull_lane_s16::<$lane2>(vget_high_s16($in0), $wvec);
        v1 = vmlal_lane_s16::<$lane3>(v1, vget_high_s16($in1), $wvec);
        let c0: int16x4_t = vrshrn_n_s32::<TXFM_COS_BIT_MAX>(u0);
        let c1: int16x4_t = vrshrn_n_s32::<TXFM_COS_BIT_MAX>(u1);
        let d0: int16x4_t = vrshrn_n_s32::<TXFM_COS_BIT_MAX>(v0);
        let d1: int16x4_t = vrshrn_n_s32::<TXFM_COS_BIT_MAX>(v1);
        *$out0 = vcombine_s16(c0, c1);
        *$out1 = vcombine_s16(d0, d1);
    }};
}

#[inline(always)]
unsafe fn butterfly_s16_s32_x8_0112_neon(w0101: int16x4_t, in0: int16x8_t, in1: int16x8_t,
                                         out0: *mut int16x8_t, out1: *mut int16x8_t) {
    butterfly_s16_s32_x8_neon!(w0101, 0, 1, 1, 2, in0, in1, out0, out1);
}

#[inline(always)]
unsafe fn butterfly_s16_s32_x8_0332_neon(w0101: int16x4_t, in0: int16x8_t, in1: int16x8_t,
                                         out0: *mut int16x8_t, out1: *mut int16x8_t) {
    butterfly_s16_s32_x8_neon!(w0101, 0, 3, 3, 2, in0, in1, out0, out1);
}

#[inline(always)]
unsafe fn butterfly_s16_s32_x8_1003_neon(w0101: int16x4_t, in0: int16x8_t, in1: int16x8_t,
                                         out0: *mut int16x8_t, out1: *mut int16x8_t) {
    butterfly_s16_s32_x8_neon!(w0101, 1, 0, 0, 3, in0, in1, out0, out1);
}

#[inline(always)]
unsafe fn butterfly_s16_s32_x8_1223_neon(w0101: int16x4_t, in0: int16x8_t, in1: int16x8_t,
                                         out0: *mut int16x8_t, out1: *mut int16x8_t) {
    butterfly_s16_s32_x8_neon!(w0101, 1, 2, 2, 3, in0, in1, out0, out1);
}

#[inline(always)]
unsafe fn flip_buf_4_neon(in_: *mut int16x4_t, out: *mut int16x4_t, size: i32) {
    for i in 0..size {
        *out.offset((size - i - 1) as isize) = *in_.offset(i as isize);
    }
}

#[inline(always)]
unsafe fn flip_buf_8_neon(in_: *mut int16x8_t, out: *mut int16x8_t, size: i32) {
    for i in 0..size {
        *out.offset((size - i - 1) as isize) = *in_.offset(i as isize);
    }
}

#[inline(always)]
unsafe fn store_buffer_interleaved_s32_x8(out: *mut i32, in1: *const int32x4_t,
                                          in2: *const int32x4_t, stride: i32, out_size: i32) {
    for i in 0..out_size {
        vst1q_s32(out.offset((stride * i) as isize), *in1.offset(i as isize));
        vst1q_s32(out.offset((stride * i + 4) as isize), *in2.offset(i as isize));
    }
}

#[inline(always)]
unsafe fn load_buffer_s16_x4(mut in_: *const i16, stride: i32, out: *mut int16x4_t,
                             out_size: i32) {
    for i in 0..out_size {
        *out.offset(i as isize) = vld1_s16(in_);
        in_ = in_.offset(stride as isize);
    }
}

#[inline(always)]
unsafe fn load_buffer_s16_x8(in_: *const i16, stride: i32, out: *mut int16x8_t, out_size: i32) {
    for i in 0..out_size {
        *out.offset(i as isize) = vld1q_s16(in_.offset((i * stride) as isize));
    }
}

#[inline(always)]
unsafe fn store_buffer_s16_x4(in_: *const int16x4_t, out: *mut i32, stride: i32, out_size: i32) {
    for i in 0..out_size {
        vst1q_s32(out.offset((i * stride) as isize), vmovl_s16(*in_.offset(i as isize)));
    }
}

#[inline(always)]
unsafe fn store_buffer_s16_x8(in_: *const int16x8_t, out: *mut i32, stride: i32, out_size: i32) {
    for i in 0..out_size {
        vst1q_s32(out.offset((i * stride + 0) as isize),
                  vmovl_s16(vget_low_s16(*in_.offset(i as isize))));
        vst1q_s32(out.offset((i * stride + 4) as isize),
                  vmovl_s16(vget_high_s16(*in_.offset(i as isize))));
    }
}

// A note on naming:
//   round_shift_[sqrt2]_s16_s32_4x1_neon(...)
//                |      |   |     ^ 1 => a single vector
//                |      |   |       n => an array of vectors
//                |      |   |   ^ input/output vector element count
//                |      |   ^ output type
//                |      ^ input type
//                ^ multiplicand and shift identifier

#[inline(always)]
unsafe fn round_shift_sqrt2_s16_s16_4x1_neon(a: int16x4_t) -> int16x4_t {
    vqrshrn_n_s32::<NewSqrt2Bits>(vmull_n_s16(a, NewSqrt2 as i16))
}

#[inline(always)]
unsafe fn round_shift_sqrt2_s16_s16_8x1_neon(a: int16x8_t) -> int16x8_t {
    vcombine_s16(round_shift_sqrt2_s16_s16_4x1_neon(vget_low_s16(a)),
                 round_shift_sqrt2_s16_s16_4x1_neon(vget_high_s16(a)))
}

#[inline(always)]
unsafe fn round_shift_2sqrt2_s16_s16_4x1_neon(a: int16x4_t) -> int16x4_t {
    vqrshrn_n_s32::<NewSqrt2Bits>(vmull_n_s16(a, (2 * NewSqrt2) as i16))
}

#[inline(always)]
unsafe fn round_shift_2sqrt2_s16_s16_8x1_neon(a: int16x8_t) -> int16x8_t {
    vcombine_s16(round_shift_2sqrt2_s16_s16_4x1_neon(vget_low_s16(a)),
                 round_shift_2sqrt2_s16_s16_4x1_neon(vget_high_s16(a)))
}

#[inline(always)]
unsafe fn round_shift_sqrt2_s16_s32_4x1_neon(a: int16x4_t) -> int32x4_t {
    vrshrq_n_s32::<NewSqrt2Bits>(vmull_n_s16(a, NewSqrt2 as i16))
}

#[inline(always)]
unsafe fn round_shift_sqrt2_s32_s32_4x1_neon(a: int32x4_t) -> int32x4_t {
    vrshrq_n_s32::<NewSqrt2Bits>(vmulq_n_s32(a, NewSqrt2))
}

macro_rules! ROUND_SHIFT_SQRT_LOOP_HELPER {
    ($name:ident, $type0:ty, $type1:ty, $fn_:ident) => {
        #[inline(always)]
        unsafe fn $name(in_: *const $type0, out: *mut $type1, size: i32) {
            for i in 0..size {
                *out.offset(i as isize) = $fn_(*in_.offset(i as isize));
            }
        }
    };
}

ROUND_SHIFT_SQRT_LOOP_HELPER!(round_shift_sqrt2_s32_s32_4xn_neon, int32x4_t, int32x4_t,
                              round_shift_sqrt2_s32_s32_4x1_neon);
ROUND_SHIFT_SQRT_LOOP_HELPER!(round_shift_sqrt2_s16_s16_4xn_neon, int16x4_t, int16x4_t,
                              round_shift_sqrt2_s16_s16_4x1_neon);
ROUND_SHIFT_SQRT_LOOP_HELPER!(round_shift_sqrt2_s16_s16_8xn_neon, int16x8_t, int16x8_t,
                              round_shift_sqrt2_s16_s16_8x1_neon);
ROUND_SHIFT_SQRT_LOOP_HELPER!(round_shift_2sqrt2_s16_s16_4xn_neon, int16x4_t, int16x4_t,
                              round_shift_2sqrt2_s16_s16_4x1_neon);
ROUND_SHIFT_SQRT_LOOP_HELPER!(round_shift_2sqrt2_s16_s16_8xn_neon, int16x8_t, int16x8_t,
                              round_shift_2sqrt2_s16_s16_8x1_neon);

#[inline(always)]
unsafe fn store_rect_buffer_s16_x4(in_: *const int16x4_t, out: *mut i32, stride: i32,
                                   out_size: i32) {
    for i in 0..out_size {
        vst1q_s32(out.offset((i * stride) as isize),
                  round_shift_sqrt2_s16_s32_4x1_neon(*in_.offset(i as isize)));
    }
}

#[inline(always)]
unsafe fn store_rect_buffer_s16_x8(in_: *const int16x8_t, out: *mut i32, stride: i32,
                                   out_size: i32) {
    for i in 0..out_size {
        vst1q_s32(out.offset((i * stride + 0) as isize),
                  round_shift_sqrt2_s16_s32_4x1_neon(vget_low_s16(*in_.offset(i as isize))));
        vst1q_s32(out.offset((i * stride + 4) as isize),
                  round_shift_sqrt2_s16_s32_4x1_neon(vget_high_s16(*in_.offset(i as isize))));
    }
}

#[inline(always)]
unsafe fn fadst4x4_neon(mut input: *const int16x4_t, mut output: *mut int16x4_t, mut cos_bit: i32) {
    let mut u: [int32x4_t; 6] = core::mem::zeroed();
    let mut v: [int32x4_t; 6] = core::mem::zeroed();
    let mut sinpi: int16x4_t = vld1_s16(sinpi_arr_q13(cos_bit));
    let mut u01: int16x4_t = vqadd_s16(*input.add(0), *input.add(1));
    v[5] = vmull_lane_s16::<2>(*input.add(2), sinpi);
    v[0] = vmull_lane_s16::<1>(*input.add(1), sinpi);
    v[0] = vmlal_lane_s16::<0>(v[0], *input.add(0), sinpi);
    v[1] = vmlal_lane_s16::<3>(v[5], *input.add(3), sinpi);
    v[2] = vmull_lane_s16::<2>(u01, sinpi);
    v[3] = vmull_lane_s16::<3>(*input.add(0), sinpi);
    v[3] = vmlsl_lane_s16::<0>(v[3], *input.add(1), sinpi);
    v[4] = vmlsl_lane_s16::<1>(v[5], *input.add(3), sinpi);
    u[0] = vaddq_s32(v[0], v[1]);
    u[1] = vmlsl_lane_s16::<2>(v[2], *input.add(3), sinpi);
    u[2] = vsubq_s32(v[3], v[4]);
    u[3] = vsubq_s32(u[2], u[0]);
    u[3] = vmlaq_n_s32(u[3], v[5], 3);
    *output.add(0) = vrshrn_n_s32::<{ TXFM_COS_BIT_MAX }>(u[0]);
    *output.add(1) = vrshrn_n_s32::<{ TXFM_COS_BIT_MAX }>(u[1]);
    *output.add(2) = vrshrn_n_s32::<{ TXFM_COS_BIT_MAX }>(u[2]);
    *output.add(3) = vrshrn_n_s32::<{ TXFM_COS_BIT_MAX }>(u[3]);
}

#[inline(always)]
unsafe fn fadst4x8_neon(mut input: *const int16x4_t, mut output: *mut int16x4_t, mut cos_bit: i32) {
    let mut cospi: *const i16 = cospi_arr_q13(cos_bit);
    let mut cospi32_16: int16x8_t = vld1q_s16(cospi.offset((4 * 0) as isize));
    let mut cospi4_12: int16x8_t = vld1q_s16(cospi.offset((4 * 4) as isize));
    let mut cospi20_28: int16x8_t = vld1q_s16(cospi.offset((4 * 6) as isize));
    let mut cospi32: int16x4_t = vget_low_s16(cospi32_16);
    let mut cospi16: int16x4_t = vget_high_s16(cospi32_16);
    let mut cospi4: int16x4_t = vget_low_s16(cospi4_12);
    let mut cospi12: int16x4_t = vget_high_s16(cospi4_12);
    let mut cospi20: int16x4_t = vget_low_s16(cospi20_28);
    let mut cospi28: int16x4_t = vget_high_s16(cospi20_28);
    // stage 1-2
    let mut x2: [int16x4_t; 8] = core::mem::zeroed();
    butterfly_s16_s32_x4_0332_neon(cospi32, *input.add(4), *input.add(3), &raw mut x2[2], &raw mut x2[3]);
    butterfly_s16_s32_x4_0112_neon(cospi32, *input.add(2), *input.add(5), &raw mut x2[7], &raw mut x2[6]);
    // stage 3
    let mut x3: [int16x4_t; 8] = core::mem::zeroed();
    x3[0] = vqadd_s16(*input.add(0), x2[2]);
    x3[1] = vqsub_s16(x2[3], *input.add(7));
    x3[2] = vqsub_s16(*input.add(0), x2[2]);
    x3[3] = vqadd_s16(*input.add(7), x2[3]);
    x3[4] = vqsub_s16(x2[6], *input.add(1));
    x3[5] = vqadd_s16(*input.add(6), x2[7]);
    x3[6] = vqadd_s16(*input.add(1), x2[6]);
    x3[7] = vqsub_s16(*input.add(6), x2[7]);
    // stage 4
    let mut x4: [int16x4_t; 8] = core::mem::zeroed();
    butterfly_s16_s32_x4_0112_neon(cospi16, x3[4], x3[5], &raw mut x4[4], &raw mut x4[5]);
    butterfly_s16_s32_x4_0112_neon(cospi16, x3[7], x3[6], &raw mut x4[6], &raw mut x4[7]);
    // stage 5
    let mut x5: [int16x4_t; 8] = core::mem::zeroed();
    x5[0] = vqadd_s16(x3[0], x4[4]);
    x5[1] = vqadd_s16(x3[1], x4[5]);
    x5[2] = vqadd_s16(x3[2], x4[6]);
    x5[3] = vqsub_s16(x4[7], x3[3]);
    x5[4] = vqsub_s16(x3[0], x4[4]);
    x5[5] = vqsub_s16(x3[1], x4[5]);
    x5[6] = vqsub_s16(x3[2], x4[6]);
    x5[7] = vqadd_s16(x3[3], x4[7]);
    // stage 6-7
    butterfly_s16_s32_x4_0112_neon(cospi4, x5[0], x5[1], output.add(7), output.add(0));
    butterfly_s16_s32_x4_0112_neon(cospi20, x5[2], x5[3], output.add(5), output.add(2));
    butterfly_s16_s32_x4_1003_neon(cospi28, x5[4], x5[5], output.add(3), output.add(4));
    butterfly_s16_s32_x4_0112_neon(cospi12, x5[6], x5[7], output.add(6), output.add(1));
}

#[inline(always)]
unsafe fn fadst8x4_neon(mut input: *const int16x8_t, mut output: *mut int16x8_t, mut cos_bit: i32) {
    let mut u_lo: [int32x4_t; 4] = core::mem::zeroed();
    let mut u_hi: [int32x4_t; 4] = core::mem::zeroed();
    let mut sinpi: int16x4_t = vld1_s16(sinpi_arr_q13(cos_bit));
    let mut u01: int16x8_t = vqaddq_s16(*input.add(0), *input.add(1));
    u_lo[0] = vmull_lane_s16::<1>(vget_low_s16(*input.add(1)), sinpi);
    u_hi[0] = vmull_lane_s16::<1>(vget_high_s16(*input.add(1)), sinpi);
    u_lo[0] = vmlal_lane_s16::<0>(u_lo[0], vget_low_s16(*input.add(0)), sinpi);
    u_hi[0] = vmlal_lane_s16::<0>(u_hi[0], vget_high_s16(*input.add(0)), sinpi);
    u_lo[0] = vmlal_lane_s16::<3>(u_lo[0], vget_low_s16(*input.add(3)), sinpi);
    u_hi[0] = vmlal_lane_s16::<3>(u_hi[0], vget_high_s16(*input.add(3)), sinpi);
    u_lo[0] = vmlal_lane_s16::<2>(u_lo[0], vget_low_s16(*input.add(2)), sinpi);
    u_hi[0] = vmlal_lane_s16::<2>(u_hi[0], vget_high_s16(*input.add(2)), sinpi);
    u_lo[1] = vmull_lane_s16::<2>(vget_low_s16(u01), sinpi);
    u_hi[1] = vmull_lane_s16::<2>(vget_high_s16(u01), sinpi);
    u_lo[2] = vmull_lane_s16::<3>(vget_low_s16(*input.add(0)), sinpi);
    u_hi[2] = vmull_lane_s16::<3>(vget_high_s16(*input.add(0)), sinpi);
    u_lo[2] = vmlsl_lane_s16::<0>(u_lo[2], vget_low_s16(*input.add(1)), sinpi);
    u_hi[2] = vmlsl_lane_s16::<0>(u_hi[2], vget_high_s16(*input.add(1)), sinpi);
    u_lo[2] = vmlal_lane_s16::<1>(u_lo[2], vget_low_s16(*input.add(3)), sinpi);
    u_hi[2] = vmlal_lane_s16::<1>(u_hi[2], vget_high_s16(*input.add(3)), sinpi);
    u_lo[2] = vmlsl_lane_s16::<2>(u_lo[2], vget_low_s16(*input.add(2)), sinpi);
    u_hi[2] = vmlsl_lane_s16::<2>(u_hi[2], vget_high_s16(*input.add(2)), sinpi);
    u_lo[1] = vmlsl_lane_s16::<2>(u_lo[1], vget_low_s16(*input.add(3)), sinpi);
    u_hi[1] = vmlsl_lane_s16::<2>(u_hi[1], vget_high_s16(*input.add(3)), sinpi);
    u_lo[3] = vsubq_s32(u_lo[2], u_lo[0]);
    u_hi[3] = vsubq_s32(u_hi[2], u_hi[0]);
    let mut sinpix3: int16x4_t = vmul_n_s16(sinpi, 3);
    u_lo[3] = vmlal_lane_s16::<2>(u_lo[3], vget_low_s16(*input.add(2)), sinpix3);
    u_hi[3] = vmlal_lane_s16::<2>(u_hi[3], vget_high_s16(*input.add(2)), sinpix3);
    *output.add(0) = vcombine_s16(vrshrn_n_s32::<{ TXFM_COS_BIT_MAX }>(u_lo[0]), vrshrn_n_s32::<{ TXFM_COS_BIT_MAX }>(u_hi[0]));
    *output.add(1) = vcombine_s16(vrshrn_n_s32::<{ TXFM_COS_BIT_MAX }>(u_lo[1]), vrshrn_n_s32::<{ TXFM_COS_BIT_MAX }>(u_hi[1]));
    *output.add(2) = vcombine_s16(vrshrn_n_s32::<{ TXFM_COS_BIT_MAX }>(u_lo[2]), vrshrn_n_s32::<{ TXFM_COS_BIT_MAX }>(u_hi[2]));
    *output.add(3) = vcombine_s16(vrshrn_n_s32::<{ TXFM_COS_BIT_MAX }>(u_lo[3]), vrshrn_n_s32::<{ TXFM_COS_BIT_MAX }>(u_hi[3]));
}

#[inline(always)]
unsafe fn fdct4x4_neon(mut input: *const int16x4_t, mut output: *mut int16x4_t, mut cos_bit: i32) {
    let mut cospi: *const i16 = cospi_arr_q13(cos_bit);
    let mut cospi16: int16x4_t = vld1_s16(cospi.offset((4 * 1) as isize));
    let mut in12a: int16x4_t = vadd_s16(*input.add(1), *input.add(2));
    let mut in12s: int16x4_t = vsub_s16(*input.add(1), *input.add(2));
    let mut in03a: int16x4_t = vadd_s16(*input.add(0), *input.add(3));
    let mut in03s: int16x4_t = vsub_s16(*input.add(0), *input.add(3));
    let mut u0ad1: int32x4_t = vmull_n_s16(in12a, (*cospi.offset((4 * 0) as isize)));
    let mut u0ad2: int32x4_t = vmull_n_s16(in03a, (*cospi.offset((4 * 0) as isize)));
    let mut u: [int32x4_t; 4] = core::mem::zeroed();
    u[0] = vaddq_s32(u0ad1, u0ad2);
    u[1] = vsubq_s32(u0ad2, u0ad1);
    u[2] = vmull_lane_s16::<1>(in12s, cospi16);
    u[2] = vmlal_lane_s16::<0>(u[2], in03s, cospi16);
    u[3] = vmull_lane_s16::<1>(in03s, cospi16);
    u[3] = vmlsl_lane_s16::<0>(u[3], in12s, cospi16);
    *output.add(0) = vrshrn_n_s32::<{ TXFM_COS_BIT_MAX }>(u[0]);
    *output.add(1) = vrshrn_n_s32::<{ TXFM_COS_BIT_MAX }>(u[2]);
    *output.add(2) = vrshrn_n_s32::<{ TXFM_COS_BIT_MAX }>(u[1]);
    *output.add(3) = vrshrn_n_s32::<{ TXFM_COS_BIT_MAX }>(u[3]);
}

// Butterfly pre-processing:
// e.g. n=4:
//   out[0] = in[0] + in[3]
//   out[1] = in[1] + in[2]
//   out[2] = in[1] - in[2]
//   out[3] = in[0] - in[3]
#[inline(always)]
unsafe fn butterfly_dct_pre_s16_x4(mut input: *const int16x4_t, mut output: *mut int16x4_t, mut n: i32) {
    for i in 0..(n / 2) {
        *output.offset(i as isize) = vqadd_s16(*input.offset(i as isize), (*input.offset((n - i - 1) as isize)));
    }
    for i in 0..(n / 2) {
        (*output.offset((n / 2 + i) as isize)) = vqsub_s16((*input.offset((n / 2 - i - 1) as isize)), (*input.offset((n / 2 + i) as isize)));
    }
}

#[inline(always)]
unsafe fn butterfly_dct_pre_s16_x8(mut input: *const int16x8_t, mut output: *mut int16x8_t, mut n: i32) {
    for i in 0..(n / 2) {
        *output.offset(i as isize) = vqaddq_s16(*input.offset(i as isize), (*input.offset((n - i - 1) as isize)));
    }
    for i in 0..(n / 2) {
        (*output.offset((n / 2 + i) as isize)) = vqsubq_s16((*input.offset((n / 2 - i - 1) as isize)), (*input.offset((n / 2 + i) as isize)));
    }
}

#[inline(always)]
unsafe fn butterfly_dct_pre_s32_x4(mut input: *const int32x4_t, mut output: *mut int32x4_t, mut n: i32) {
    for i in 0..(n / 2) {
        *output.offset(i as isize) = vqaddq_s32(*input.offset(i as isize), (*input.offset((n - i - 1) as isize)));
    }
    for i in 0..(n / 2) {
        (*output.offset((n / 2 + i) as isize)) = vqsubq_s32((*input.offset((n / 2 - i - 1) as isize)), (*input.offset((n / 2 + i) as isize)));
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
unsafe fn butterfly_dct_post_s16_x4(mut in0: *const int16x4_t, mut in1: *const int16x4_t, mut output: *mut int16x4_t, mut n: i32) {
    for i in 0..(n / 4) {
        *output.offset(i as isize) = vqadd_s16(*in0.offset(i as isize), (*in1.offset((n / 2 - i - 1) as isize)));
    }
    for i in 0..(n / 4) {
        (*output.offset((n / 4 + i) as isize)) = vqsub_s16((*in0.offset((n / 4 - i - 1) as isize)), (*in1.offset((n / 4 + i) as isize)));
    }
    for i in 0..(n / 4) {
        (*output.offset((n / 2 + i) as isize)) = vqsub_s16((*in0.offset((n - i - 1) as isize)), (*in1.offset((n / 2 + i) as isize)));
    }
    for i in 0..(n / 4) {
        (*output.offset(((3 * n) / 4 + i) as isize)) = vqadd_s16((*in0.offset(((3 * n) / 4 + i) as isize)), (*in1.offset(((3 * n) / 4 - i - 1) as isize)));
    }
}

#[inline(always)]
unsafe fn butterfly_dct_post_s16_x8(mut in0: *const int16x8_t, mut in1: *const int16x8_t, mut output: *mut int16x8_t, mut n: i32) {
    for i in 0..(n / 4) {
        *output.offset(i as isize) = vqaddq_s16(*in0.offset(i as isize), (*in1.offset((n / 2 - i - 1) as isize)));
    }
    for i in 0..(n / 4) {
        (*output.offset((n / 4 + i) as isize)) = vqsubq_s16((*in0.offset((n / 4 - i - 1) as isize)), (*in1.offset((n / 4 + i) as isize)));
    }
    for i in 0..(n / 4) {
        (*output.offset((n / 2 + i) as isize)) = vqsubq_s16((*in0.offset((n - i - 1) as isize)), (*in1.offset((n / 2 + i) as isize)));
    }
    for i in 0..(n / 4) {
        (*output.offset(((3 * n) / 4 + i) as isize)) = vqaddq_s16((*in0.offset(((3 * n) / 4 + i) as isize)), (*in1.offset(((3 * n) / 4 - i - 1) as isize)));
    }
}

#[inline(always)]
unsafe fn butterfly_dct_post_s32_x4(mut in0: *const int32x4_t, mut in1: *const int32x4_t, mut output: *mut int32x4_t, mut n: i32) {
    for i in 0..(n / 4) {
        *output.offset(i as isize) = vqaddq_s32(*in0.offset(i as isize), (*in1.offset((n / 2 - i - 1) as isize)));
    }
    for i in 0..(n / 4) {
        (*output.offset((n / 4 + i) as isize)) = vqsubq_s32((*in0.offset((n / 4 - i - 1) as isize)), (*in1.offset((n / 4 + i) as isize)));
    }
    for i in 0..(n / 4) {
        (*output.offset((n / 2 + i) as isize)) = vqsubq_s32((*in0.offset((n - i - 1) as isize)), (*in1.offset((n / 2 + i) as isize)));
    }
    for i in 0..(n / 4) {
        (*output.offset(((3 * n) / 4 + i) as isize)) = vqaddq_s32((*in0.offset(((3 * n) / 4 + i) as isize)), (*in1.offset(((3 * n) / 4 - i - 1) as isize)));
    }
}

#[inline(always)]
unsafe fn fdct8x4_neon(mut input: *const int16x8_t, mut output: *mut int16x8_t, mut cos_bit: i32) {
    let mut cospi: *const i16 = cospi_arr_q13(cos_bit);
    let mut cospi32_16: int16x8_t = vld1q_s16(cospi.offset((4 * 0) as isize));
    let mut cospi32: int16x4_t = vget_low_s16(cospi32_16);
    let mut cospi16: int16x4_t = vget_high_s16(cospi32_16);
    // stage 1
    let mut x1: [int16x8_t; 4] = core::mem::zeroed();
    butterfly_dct_pre_s16_x8(input, x1.as_mut_ptr(), 4);
    // stage 2
    let mut x2: [int16x8_t; 4] = core::mem::zeroed();
    butterfly_s16_s32_x8_0112_neon(cospi32, x1[0], x1[1], &raw mut x2[0], &raw mut x2[1]);
    butterfly_s16_s32_x8_0112_neon(cospi16, x1[3], x1[2], &raw mut x2[2], &raw mut x2[3]);
    // stage 3
    *output.add(0) = x2[0];
    *output.add(1) = x2[2];
    *output.add(2) = x2[1];
    *output.add(3) = x2[3];
}

#[inline(always)]
unsafe fn fdct4x8_neon(mut input: *const int16x4_t, mut output: *mut int16x4_t, mut cos_bit: i32) {
    let mut cospi: *const i16 = cospi_arr_q13(cos_bit);
    let mut cospi32_16: int16x8_t = vld1q_s16(cospi.offset((4 * 0) as isize));
    let mut cospi8_24: int16x8_t = vld1q_s16(cospi.offset((4 * 2) as isize));
    let mut cospi32: int16x4_t = vget_low_s16(cospi32_16);
    let mut cospi16: int16x4_t = vget_high_s16(cospi32_16);
    let mut cospi8: int16x4_t = vget_low_s16(cospi8_24);
    let mut cospi24: int16x4_t = vget_high_s16(cospi8_24);
    // stage 1
    let mut x1: [int16x4_t; 8] = core::mem::zeroed();
    butterfly_dct_pre_s16_x4(input, x1.as_mut_ptr(), 8);
    // stage 2
    let mut x2: [int16x4_t; 8] = core::mem::zeroed();
    butterfly_dct_pre_s16_x4(x1.as_mut_ptr(), x2.as_mut_ptr(), 4);
    butterfly_s16_s32_x4_0112_neon(cospi32, x1[6], x1[5], &raw mut x2[6], &raw mut x2[5]);
    // stage 3
    let mut x3: [int16x4_t; 8] = core::mem::zeroed();
    butterfly_s16_s32_x4_0112_neon(cospi32, x2[0], x2[1], output.add(0), output.add(4));
    butterfly_s16_s32_x4_0112_neon(cospi16, x2[3], x2[2], output.add(2), output.add(6));
    butterfly_dct_post_s16_x4(x1.as_mut_ptr().offset(((4)) as isize), x2.as_mut_ptr().offset(((4)) as isize), x3.as_mut_ptr().offset(((4)) as isize), 4);
    // stage 4-5
    butterfly_s16_s32_x4_0112_neon(cospi8, x3[7], x3[4], output.add(1), output.add(7));
    butterfly_s16_s32_x4_1003_neon(cospi24, x3[6], x3[5], output.add(5), output.add(3));
}

#[inline(always)]
unsafe fn fdct8x8_neon(mut input: *const int16x8_t, mut output: *mut int16x8_t, mut cos_bit: i32) {
    let mut cospi: *const i16 = cospi_arr_q13(cos_bit);
    let mut cospi32_16: int16x8_t = vld1q_s16(cospi.offset((4 * 0) as isize));
    let mut cospi8_24: int16x8_t = vld1q_s16(cospi.offset((4 * 2) as isize));
    let mut cospi32: int16x4_t = vget_low_s16(cospi32_16);
    let mut cospi16: int16x4_t = vget_high_s16(cospi32_16);
    let mut cospi8: int16x4_t = vget_low_s16(cospi8_24);
    let mut cospi24: int16x4_t = vget_high_s16(cospi8_24);
    // stage 1
    let mut x1: [int16x8_t; 8] = core::mem::zeroed();
    butterfly_dct_pre_s16_x8(input, x1.as_mut_ptr(), 8);
    // stage 2
    let mut x2: [int16x8_t; 8] = core::mem::zeroed();
    butterfly_dct_pre_s16_x8(x1.as_mut_ptr(), x2.as_mut_ptr(), 4);
    butterfly_s16_s32_x8_0112_neon(cospi32, x1[6], x1[5], &raw mut x2[6], &raw mut x2[5]);
    // stage 3
    let mut x3: [int16x8_t; 8] = core::mem::zeroed();
    butterfly_s16_s32_x8_0112_neon(cospi32, x2[0], x2[1], output.add(0), output.add(4));
    butterfly_s16_s32_x8_0112_neon(cospi16, x2[3], x2[2], output.add(2), output.add(6));
    butterfly_dct_post_s16_x8(x1.as_mut_ptr().offset(((4)) as isize), x2.as_mut_ptr().offset(((4)) as isize), x3.as_mut_ptr().offset(((4)) as isize), 4);
    // stage 4-5
    butterfly_s16_s32_x8_0112_neon(cospi8, x3[7], x3[4], output.add(1), output.add(7));
    butterfly_s16_s32_x8_1003_neon(cospi24, x3[6], x3[5], output.add(5), output.add(3));
}

#[inline(always)]
unsafe fn fdct4x16_neon(mut input: *const int16x4_t, mut output: *mut int16x4_t, mut cos_bit: i32) {
    let mut cospi: *const i16 = cospi_arr_q13(cos_bit);
    let mut cospi32_16: int16x8_t = vld1q_s16(cospi.offset((4 * 0) as isize));
    let mut cospi8_24: int16x8_t = vld1q_s16(cospi.offset((4 * 2) as isize));
    let mut cospi4_12: int16x8_t = vld1q_s16(cospi.offset((4 * 4) as isize));
    let mut cospi20_28: int16x8_t = vld1q_s16(cospi.offset((4 * 6) as isize));
    let mut cospi32: int16x4_t = vget_low_s16(cospi32_16);
    let mut cospi16: int16x4_t = vget_high_s16(cospi32_16);
    let mut cospi8: int16x4_t = vget_low_s16(cospi8_24);
    let mut cospi24: int16x4_t = vget_high_s16(cospi8_24);
    let mut cospi4: int16x4_t = vget_low_s16(cospi4_12);
    let mut cospi12: int16x4_t = vget_high_s16(cospi4_12);
    let mut cospi20: int16x4_t = vget_low_s16(cospi20_28);
    let mut cospi28: int16x4_t = vget_high_s16(cospi20_28);
    // stage 1
    let mut x1: [int16x4_t; 16] = core::mem::zeroed();
    butterfly_dct_pre_s16_x4(input, x1.as_mut_ptr(), 16);
    // stage 2
    let mut x2: [int16x4_t; 16] = core::mem::zeroed();
    butterfly_dct_pre_s16_x4(x1.as_mut_ptr(), x2.as_mut_ptr(), 8);
    butterfly_s16_s32_x4_0112_neon(cospi32, x1[13], x1[10], &raw mut x2[13], &raw mut x2[10]);
    butterfly_s16_s32_x4_0112_neon(cospi32, x1[12], x1[11], &raw mut x2[12], &raw mut x2[11]);
    // stage 3
    let mut x3: [int16x4_t; 16] = core::mem::zeroed();
    butterfly_dct_pre_s16_x4(x2.as_mut_ptr(), x3.as_mut_ptr(), 4);
    butterfly_s16_s32_x4_0112_neon(cospi32, x2[6], x2[5], &raw mut x3[6], &raw mut x3[5]);
    butterfly_dct_post_s16_x4(x1.as_mut_ptr().offset(((8)) as isize), x2.as_mut_ptr().offset(((8)) as isize), x3.as_mut_ptr().offset(((8)) as isize), 8);
    // stage 4
    let mut x4: [int16x4_t; 16] = core::mem::zeroed();
    butterfly_s16_s32_x4_0112_neon(cospi32, x3[0], x3[1], output.add(0), output.add(8));
    butterfly_s16_s32_x4_0112_neon(cospi16, x3[3], x3[2], output.add(4), output.add(12));
    butterfly_dct_post_s16_x4(x2.as_mut_ptr().offset(((4)) as isize), x3.as_mut_ptr().offset(((4)) as isize), x4.as_mut_ptr().offset(((4)) as isize), 4);
    butterfly_s16_s32_x4_0112_neon(cospi16, x3[14], x3[9], &raw mut x4[14], &raw mut x4[9]);
    butterfly_s16_s32_x4_1223_neon(cospi16, x3[13], x3[10], &raw mut x4[13], &raw mut x4[10]);
    // stage 5
    let mut x5: [int16x4_t; 16] = core::mem::zeroed();
    butterfly_s16_s32_x4_0112_neon(cospi8, x4[7], x4[4], output.add(2), output.add(14));
    butterfly_s16_s32_x4_1003_neon(cospi24, x4[6], x4[5], output.add(10), output.add(6));
    butterfly_dct_post_s16_x4(x3.as_mut_ptr().offset(((8)) as isize), x4.as_mut_ptr().offset(((8)) as isize), x5.as_mut_ptr().offset(((8)) as isize), 4);
    butterfly_dct_post_s16_x4(x3.as_mut_ptr().offset(((12)) as isize), x4.as_mut_ptr().offset(((12)) as isize), x5.as_mut_ptr().offset(((12)) as isize), 4);
    // stage 6-7
    butterfly_s16_s32_x4_0112_neon(cospi4, x5[15], x5[8], output.add(1), output.add(15));
    butterfly_s16_s32_x4_1003_neon(cospi28, x5[14], x5[9], output.add(9), output.add(7));
    butterfly_s16_s32_x4_0112_neon(cospi20, x5[13], x5[10], output.add(5), output.add(11));
    butterfly_s16_s32_x4_1003_neon(cospi12, x5[12], x5[11], output.add(13), output.add(3));
}

#[inline(always)]
unsafe fn fdct8x16_neon(mut input: *const int16x8_t, mut output: *mut int16x8_t, mut cos_bit: i32) {
    let mut cospi: *const i16 = cospi_arr_q13(cos_bit);
    let mut cospi32_16: int16x8_t = vld1q_s16(cospi.offset((4 * 0) as isize));
    let mut cospi8_24: int16x8_t = vld1q_s16(cospi.offset((4 * 2) as isize));
    let mut cospi4_12: int16x8_t = vld1q_s16(cospi.offset((4 * 4) as isize));
    let mut cospi20_28: int16x8_t = vld1q_s16(cospi.offset((4 * 6) as isize));
    let mut cospi32: int16x4_t = vget_low_s16(cospi32_16);
    let mut cospi16: int16x4_t = vget_high_s16(cospi32_16);
    let mut cospi8: int16x4_t = vget_low_s16(cospi8_24);
    let mut cospi24: int16x4_t = vget_high_s16(cospi8_24);
    let mut cospi4: int16x4_t = vget_low_s16(cospi4_12);
    let mut cospi12: int16x4_t = vget_high_s16(cospi4_12);
    let mut cospi20: int16x4_t = vget_low_s16(cospi20_28);
    let mut cospi28: int16x4_t = vget_high_s16(cospi20_28);
    // stage 1
    let mut x1: [int16x8_t; 16] = core::mem::zeroed();
    butterfly_dct_pre_s16_x8(input, x1.as_mut_ptr(), 16);
    // stage 2
    let mut x2: [int16x8_t; 16] = core::mem::zeroed();
    butterfly_dct_pre_s16_x8(x1.as_mut_ptr(), x2.as_mut_ptr(), 8);
    butterfly_s16_s32_x8_0112_neon(cospi32, x1[13], x1[10], &raw mut x2[13], &raw mut x2[10]);
    butterfly_s16_s32_x8_0112_neon(cospi32, x1[12], x1[11], &raw mut x2[12], &raw mut x2[11]);
    // stage 3
    let mut x3: [int16x8_t; 16] = core::mem::zeroed();
    butterfly_dct_pre_s16_x8(x2.as_mut_ptr(), x3.as_mut_ptr(), 4);
    butterfly_s16_s32_x8_0112_neon(cospi32, x2[6], x2[5], &raw mut x3[6], &raw mut x3[5]);
    butterfly_dct_post_s16_x8(x1.as_mut_ptr().offset(((8)) as isize), x2.as_mut_ptr().offset(((8)) as isize), x3.as_mut_ptr().offset(((8)) as isize), 8);
    // stage 4
    let mut x4: [int16x8_t; 16] = core::mem::zeroed();
    butterfly_s16_s32_x8_0112_neon(cospi32, x3[0], x3[1], output.add(0), output.add(8));
    butterfly_s16_s32_x8_0112_neon(cospi16, x3[3], x3[2], output.add(4), output.add(12));
    butterfly_dct_post_s16_x8(x2.as_mut_ptr().offset(((4)) as isize), x3.as_mut_ptr().offset(((4)) as isize), x4.as_mut_ptr().offset(((4)) as isize), 4);
    butterfly_s16_s32_x8_0112_neon(cospi16, x3[14], x3[9], &raw mut x4[14], &raw mut x4[9]);
    butterfly_s16_s32_x8_1223_neon(cospi16, x3[13], x3[10], &raw mut x4[13], &raw mut x4[10]);
    // stage 5
    let mut x5: [int16x8_t; 16] = core::mem::zeroed();
    butterfly_s16_s32_x8_0112_neon(cospi8, x4[7], x4[4], output.add(2), output.add(14));
    butterfly_s16_s32_x8_1003_neon(cospi24, x4[6], x4[5], output.add(10), output.add(6));
    butterfly_dct_post_s16_x8(x3.as_mut_ptr().offset(((8)) as isize), x4.as_mut_ptr().offset(((8)) as isize), x5.as_mut_ptr().offset(((8)) as isize), 4);
    butterfly_dct_post_s16_x8(x3.as_mut_ptr().offset(((12)) as isize), x4.as_mut_ptr().offset(((12)) as isize), x5.as_mut_ptr().offset(((12)) as isize), 4);
    // stage 6-7
    butterfly_s16_s32_x8_0112_neon(cospi4, x5[15], x5[8], output.add(1), output.add(15));
    butterfly_s16_s32_x8_1003_neon(cospi28, x5[14], x5[9], output.add(9), output.add(7));
    butterfly_s16_s32_x8_0112_neon(cospi20, x5[13], x5[10], output.add(5), output.add(11));
    butterfly_s16_s32_x8_1003_neon(cospi12, x5[12], x5[11], output.add(13), output.add(3));
}

#[inline(always)]
unsafe fn fdct8x32_neon(mut input: *const int16x8_t, mut output: *mut int16x8_t, mut cos_bit: i32) {
    let mut cospi: *const i16 = cospi_arr_q13(cos_bit);
    let mut cospi32_16: int16x8_t = vld1q_s16(cospi.offset((4 * 0) as isize));
    let mut cospi8_24: int16x8_t = vld1q_s16(cospi.offset((4 * 2) as isize));
    let mut cospi4_12: int16x8_t = vld1q_s16(cospi.offset((4 * 4) as isize));
    let mut cospi20_28: int16x8_t = vld1q_s16(cospi.offset((4 * 6) as isize));
    let mut cospi2_6: int16x8_t = vld1q_s16(cospi.offset((4 * 8) as isize));
    let mut cospi10_14: int16x8_t = vld1q_s16(cospi.offset((4 * 10) as isize));
    let mut cospi18_22: int16x8_t = vld1q_s16(cospi.offset((4 * 12) as isize));
    let mut cospi26_30: int16x8_t = vld1q_s16(cospi.offset((4 * 14) as isize));
    let mut cospi32: int16x4_t = vget_low_s16(cospi32_16);
    let mut cospi16: int16x4_t = vget_high_s16(cospi32_16);
    let mut cospi8: int16x4_t = vget_low_s16(cospi8_24);
    let mut cospi24: int16x4_t = vget_high_s16(cospi8_24);
    let mut cospi4: int16x4_t = vget_low_s16(cospi4_12);
    let mut cospi12: int16x4_t = vget_high_s16(cospi4_12);
    let mut cospi20: int16x4_t = vget_low_s16(cospi20_28);
    let mut cospi28: int16x4_t = vget_high_s16(cospi20_28);
    let mut cospi2: int16x4_t = vget_low_s16(cospi2_6);
    let mut cospi6: int16x4_t = vget_high_s16(cospi2_6);
    let mut cospi10: int16x4_t = vget_low_s16(cospi10_14);
    let mut cospi14: int16x4_t = vget_high_s16(cospi10_14);
    let mut cospi18: int16x4_t = vget_low_s16(cospi18_22);
    let mut cospi22: int16x4_t = vget_high_s16(cospi18_22);
    let mut cospi26: int16x4_t = vget_low_s16(cospi26_30);
    let mut cospi30: int16x4_t = vget_high_s16(cospi26_30);
    // stage 1
    let mut x1: [int16x8_t; 32] = core::mem::zeroed();
    butterfly_dct_pre_s16_x8(input, x1.as_mut_ptr(), 32);
    // stage 2
    let mut x2: [int16x8_t; 32] = core::mem::zeroed();
    butterfly_dct_pre_s16_x8(x1.as_mut_ptr(), x2.as_mut_ptr(), 16);
    butterfly_s16_s32_x8_0112_neon(cospi32, x1[27], x1[20], &raw mut x2[27], &raw mut x2[20]);
    butterfly_s16_s32_x8_0112_neon(cospi32, x1[26], x1[21], &raw mut x2[26], &raw mut x2[21]);
    butterfly_s16_s32_x8_0112_neon(cospi32, x1[25], x1[22], &raw mut x2[25], &raw mut x2[22]);
    butterfly_s16_s32_x8_0112_neon(cospi32, x1[24], x1[23], &raw mut x2[24], &raw mut x2[23]);
    // stage 3
    let mut x3: [int16x8_t; 32] = core::mem::zeroed();
    butterfly_dct_pre_s16_x8(x2.as_mut_ptr(), x3.as_mut_ptr(), 8);
    butterfly_s16_s32_x8_0112_neon(cospi32, x2[13], x2[10], &raw mut x3[13], &raw mut x3[10]);
    butterfly_s16_s32_x8_0112_neon(cospi32, x2[12], x2[11], &raw mut x3[12], &raw mut x3[11]);
    butterfly_dct_post_s16_x8(x1.as_mut_ptr().offset(((16)) as isize), x2.as_mut_ptr().offset(((16)) as isize), x3.as_mut_ptr().offset(((16)) as isize), 16);
    // stage 4
    let mut x4: [int16x8_t; 32] = core::mem::zeroed();
    butterfly_dct_pre_s16_x8(x3.as_mut_ptr(), x4.as_mut_ptr(), 4);
    butterfly_s16_s32_x8_0112_neon(cospi32, x3[6], x3[5], &raw mut x4[6], &raw mut x4[5]);
    butterfly_dct_post_s16_x8(x2.as_mut_ptr().offset(((8)) as isize), x3.as_mut_ptr().offset(((8)) as isize), x4.as_mut_ptr().offset(((8)) as isize), 8);
    butterfly_s16_s32_x8_0112_neon(cospi16, x3[29], x3[18], &raw mut x4[29], &raw mut x4[18]);
    butterfly_s16_s32_x8_0112_neon(cospi16, x3[28], x3[19], &raw mut x4[28], &raw mut x4[19]);
    butterfly_s16_s32_x8_1223_neon(cospi16, x3[27], x3[20], &raw mut x4[27], &raw mut x4[20]);
    butterfly_s16_s32_x8_1223_neon(cospi16, x3[26], x3[21], &raw mut x4[26], &raw mut x4[21]);
    // stage 5
    let mut x5: [int16x8_t; 32] = core::mem::zeroed();
    butterfly_s16_s32_x8_0112_neon(cospi32, x4[0], x4[1], output.add(0), output.add(16));
    butterfly_s16_s32_x8_0112_neon(cospi16, x4[3], x4[2], output.add(8), output.add(24));
    butterfly_dct_post_s16_x8(x3.as_mut_ptr().offset(((4)) as isize), x4.as_mut_ptr().offset(((4)) as isize), x5.as_mut_ptr().offset(((4)) as isize), 4);
    butterfly_s16_s32_x8_0112_neon(cospi16, x4[14], x4[9], &raw mut x5[14], &raw mut x5[9]);
    butterfly_s16_s32_x8_1223_neon(cospi16, x4[13], x4[10], &raw mut x5[13], &raw mut x5[10]);
    butterfly_dct_post_s16_x8(x3.as_mut_ptr().offset(((16)) as isize), x4.as_mut_ptr().offset(((16)) as isize), x5.as_mut_ptr().offset(((16)) as isize), 8);
    butterfly_dct_post_s16_x8(x3.as_mut_ptr().offset(((24)) as isize), x4.as_mut_ptr().offset(((24)) as isize), x5.as_mut_ptr().offset(((24)) as isize), 8);
    // stage 6
    let mut x6: [int16x8_t; 32] = core::mem::zeroed();
    butterfly_s16_s32_x8_0112_neon(cospi8, x5[7], x5[4], output.add(4), output.add(28));
    butterfly_s16_s32_x8_1003_neon(cospi24, x5[6], x5[5], output.add(20), output.add(12));
    butterfly_dct_post_s16_x8(x4.as_mut_ptr().offset(((8)) as isize), x5.as_mut_ptr().offset(((8)) as isize), x6.as_mut_ptr().offset(((8)) as isize), 4);
    butterfly_dct_post_s16_x8(x4.as_mut_ptr().offset(((12)) as isize), x5.as_mut_ptr().offset(((12)) as isize), x6.as_mut_ptr().offset(((12)) as isize), 4);
    butterfly_s16_s32_x8_0112_neon(cospi8, x5[30], x5[17], &raw mut x6[30], &raw mut x6[17]);
    butterfly_s16_s32_x8_1223_neon(cospi8, x5[29], x5[18], &raw mut x6[29], &raw mut x6[18]);
    butterfly_s16_s32_x8_1003_neon(cospi24, x5[26], x5[21], &raw mut x6[26], &raw mut x6[21]);
    butterfly_s16_s32_x8_0332_neon(cospi24, x5[25], x5[22], &raw mut x6[25], &raw mut x6[22]);
    // stage 7
    let mut x7: [int16x8_t; 32] = core::mem::zeroed();
    butterfly_s16_s32_x8_0112_neon(cospi4, x6[15], x6[8], output.add(2), output.add(30));
    butterfly_s16_s32_x8_1003_neon(cospi28, x6[14], x6[9], output.add(18), output.add(14));
    butterfly_s16_s32_x8_0112_neon(cospi20, x6[13], x6[10], output.add(10), output.add(22));
    butterfly_s16_s32_x8_1003_neon(cospi12, x6[12], x6[11], output.add(26), output.add(6));
    butterfly_dct_post_s16_x8(x5.as_mut_ptr().offset(((16)) as isize), x6.as_mut_ptr().offset(((16)) as isize), x7.as_mut_ptr().offset(((16)) as isize), 4);
    butterfly_dct_post_s16_x8(x5.as_mut_ptr().offset(((20)) as isize), x6.as_mut_ptr().offset(((20)) as isize), x7.as_mut_ptr().offset(((20)) as isize), 4);
    butterfly_dct_post_s16_x8(x5.as_mut_ptr().offset(((24)) as isize), x6.as_mut_ptr().offset(((24)) as isize), x7.as_mut_ptr().offset(((24)) as isize), 4);
    butterfly_dct_post_s16_x8(x5.as_mut_ptr().offset(((28)) as isize), x6.as_mut_ptr().offset(((28)) as isize), x7.as_mut_ptr().offset(((28)) as isize), 4);
    butterfly_s16_s32_x8_0112_neon(cospi2, x7[31], x7[16], output.add(1), output.add(31));
    butterfly_s16_s32_x8_1003_neon(cospi30, x7[30], x7[17], output.add(17), output.add(15));
    butterfly_s16_s32_x8_0112_neon(cospi18, x7[29], x7[18], output.add(9), output.add(23));
    butterfly_s16_s32_x8_1003_neon(cospi14, x7[28], x7[19], output.add(25), output.add(7));
    butterfly_s16_s32_x8_0112_neon(cospi10, x7[27], x7[20], output.add(5), output.add(27));
    butterfly_s16_s32_x8_1003_neon(cospi22, x7[26], x7[21], output.add(21), output.add(11));
    butterfly_s16_s32_x8_0112_neon(cospi26, x7[25], x7[22], output.add(13), output.add(19));
    butterfly_s16_s32_x8_1003_neon(cospi6, x7[24], x7[23], output.add(29), output.add(3));
}

#[inline(always)]
unsafe fn fdct8x64_neon(mut input: *const int16x8_t, mut output: *mut int16x8_t, mut cos_bit: i32) {
    let mut cospi: *const i16 = cospi_arr_q13(cos_bit);
    let mut cospi32_16: int16x8_t = vld1q_s16(cospi.offset((4 * 0) as isize));
    let mut cospi8_24: int16x8_t = vld1q_s16(cospi.offset((4 * 2) as isize));
    let mut cospi4_12: int16x8_t = vld1q_s16(cospi.offset((4 * 4) as isize));
    let mut cospi20_28: int16x8_t = vld1q_s16(cospi.offset((4 * 6) as isize));
    let mut cospi2_6: int16x8_t = vld1q_s16(cospi.offset((4 * 8) as isize));
    let mut cospi10_14: int16x8_t = vld1q_s16(cospi.offset((4 * 10) as isize));
    let mut cospi18_22: int16x8_t = vld1q_s16(cospi.offset((4 * 12) as isize));
    let mut cospi26_30: int16x8_t = vld1q_s16(cospi.offset((4 * 14) as isize));
    let mut cospi1_3: int16x8_t = vld1q_s16(cospi.offset((4 * 16) as isize));
    let mut cospi5_7: int16x8_t = vld1q_s16(cospi.offset((4 * 18) as isize));
    let mut cospi9_11: int16x8_t = vld1q_s16(cospi.offset((4 * 20) as isize));
    let mut cospi13_15: int16x8_t = vld1q_s16(cospi.offset((4 * 22) as isize));
    let mut cospi17_19: int16x8_t = vld1q_s16(cospi.offset((4 * 24) as isize));
    let mut cospi21_23: int16x8_t = vld1q_s16(cospi.offset((4 * 26) as isize));
    let mut cospi25_27: int16x8_t = vld1q_s16(cospi.offset((4 * 28) as isize));
    let mut cospi29_31: int16x8_t = vld1q_s16(cospi.offset((4 * 30) as isize));
    let mut cospi32: int16x4_t = vget_low_s16(cospi32_16);
    let mut cospi16: int16x4_t = vget_high_s16(cospi32_16);
    let mut cospi8: int16x4_t = vget_low_s16(cospi8_24);
    let mut cospi24: int16x4_t = vget_high_s16(cospi8_24);
    let mut cospi4: int16x4_t = vget_low_s16(cospi4_12);
    let mut cospi12: int16x4_t = vget_high_s16(cospi4_12);
    let mut cospi20: int16x4_t = vget_low_s16(cospi20_28);
    let mut cospi28: int16x4_t = vget_high_s16(cospi20_28);
    let mut cospi2: int16x4_t = vget_low_s16(cospi2_6);
    let mut cospi6: int16x4_t = vget_high_s16(cospi2_6);
    let mut cospi10: int16x4_t = vget_low_s16(cospi10_14);
    let mut cospi14: int16x4_t = vget_high_s16(cospi10_14);
    let mut cospi18: int16x4_t = vget_low_s16(cospi18_22);
    let mut cospi22: int16x4_t = vget_high_s16(cospi18_22);
    let mut cospi26: int16x4_t = vget_low_s16(cospi26_30);
    let mut cospi30: int16x4_t = vget_high_s16(cospi26_30);
    let mut cospi1: int16x4_t = vget_low_s16(cospi1_3);
    let mut cospi3: int16x4_t = vget_high_s16(cospi1_3);
    let mut cospi5: int16x4_t = vget_low_s16(cospi5_7);
    let mut cospi7: int16x4_t = vget_high_s16(cospi5_7);
    let mut cospi9: int16x4_t = vget_low_s16(cospi9_11);
    let mut cospi11: int16x4_t = vget_high_s16(cospi9_11);
    let mut cospi13: int16x4_t = vget_low_s16(cospi13_15);
    let mut cospi15: int16x4_t = vget_high_s16(cospi13_15);
    let mut cospi17: int16x4_t = vget_low_s16(cospi17_19);
    let mut cospi19: int16x4_t = vget_high_s16(cospi17_19);
    let mut cospi21: int16x4_t = vget_low_s16(cospi21_23);
    let mut cospi23: int16x4_t = vget_high_s16(cospi21_23);
    let mut cospi25: int16x4_t = vget_low_s16(cospi25_27);
    let mut cospi27: int16x4_t = vget_high_s16(cospi25_27);
    let mut cospi29: int16x4_t = vget_low_s16(cospi29_31);
    let mut cospi31: int16x4_t = vget_high_s16(cospi29_31);
    // stage 1
    let mut x1: [int16x8_t; 64] = core::mem::zeroed();
    butterfly_dct_pre_s16_x8(input, x1.as_mut_ptr(), 64);
    // stage 2
    let mut x2: [int16x8_t; 64] = core::mem::zeroed();
    butterfly_dct_pre_s16_x8(x1.as_mut_ptr(), x2.as_mut_ptr(), 32);
    butterfly_s16_s32_x8_0112_neon(cospi32, x1[55], x1[40], &raw mut x2[55], &raw mut x2[40]);
    butterfly_s16_s32_x8_0112_neon(cospi32, x1[54], x1[41], &raw mut x2[54], &raw mut x2[41]);
    butterfly_s16_s32_x8_0112_neon(cospi32, x1[53], x1[42], &raw mut x2[53], &raw mut x2[42]);
    butterfly_s16_s32_x8_0112_neon(cospi32, x1[52], x1[43], &raw mut x2[52], &raw mut x2[43]);
    butterfly_s16_s32_x8_0112_neon(cospi32, x1[51], x1[44], &raw mut x2[51], &raw mut x2[44]);
    butterfly_s16_s32_x8_0112_neon(cospi32, x1[50], x1[45], &raw mut x2[50], &raw mut x2[45]);
    butterfly_s16_s32_x8_0112_neon(cospi32, x1[49], x1[46], &raw mut x2[49], &raw mut x2[46]);
    butterfly_s16_s32_x8_0112_neon(cospi32, x1[48], x1[47], &raw mut x2[48], &raw mut x2[47]);
    // stage 3
    let mut x3: [int16x8_t; 64] = core::mem::zeroed();
    butterfly_dct_pre_s16_x8(x2.as_mut_ptr(), x3.as_mut_ptr(), 16);
    x3[16] = x2[16];
    x3[17] = x2[17];
    x3[18] = x2[18];
    x3[19] = x2[19];
    butterfly_s16_s32_x8_0112_neon(cospi32, x2[27], x2[20], &raw mut x3[27], &raw mut x3[20]);
    butterfly_s16_s32_x8_0112_neon(cospi32, x2[26], x2[21], &raw mut x3[26], &raw mut x3[21]);
    butterfly_s16_s32_x8_0112_neon(cospi32, x2[25], x2[22], &raw mut x3[25], &raw mut x3[22]);
    butterfly_s16_s32_x8_0112_neon(cospi32, x2[24], x2[23], &raw mut x3[24], &raw mut x3[23]);
    x3[28] = x2[28];
    x3[29] = x2[29];
    x3[30] = x2[30];
    x3[31] = x2[31];
    butterfly_dct_post_s16_x8(x1.as_mut_ptr().offset(((32)) as isize), x2.as_mut_ptr().offset(((32)) as isize), x3.as_mut_ptr().offset(((32)) as isize), 32);
    // stage 4
    let mut x4: [int16x8_t; 64] = core::mem::zeroed();
    butterfly_dct_pre_s16_x8(x3.as_mut_ptr(), x4.as_mut_ptr(), 8);
    butterfly_s16_s32_x8_0112_neon(cospi32, x3[13], x3[10], &raw mut x4[13], &raw mut x4[10]);
    butterfly_s16_s32_x8_0112_neon(cospi32, x3[12], x3[11], &raw mut x4[12], &raw mut x4[11]);
    butterfly_dct_post_s16_x8(x3.as_mut_ptr().offset(((16)) as isize), x3.as_mut_ptr().offset(((16)) as isize), x4.as_mut_ptr().offset(((16)) as isize), 16);
    butterfly_s16_s32_x8_0112_neon(cospi16, x3[59], x3[36], &raw mut x4[59], &raw mut x4[36]);
    butterfly_s16_s32_x8_0112_neon(cospi16, x3[58], x3[37], &raw mut x4[58], &raw mut x4[37]);
    butterfly_s16_s32_x8_0112_neon(cospi16, x3[57], x3[38], &raw mut x4[57], &raw mut x4[38]);
    butterfly_s16_s32_x8_0112_neon(cospi16, x3[56], x3[39], &raw mut x4[56], &raw mut x4[39]);
    butterfly_s16_s32_x8_1223_neon(cospi16, x3[55], x3[40], &raw mut x4[55], &raw mut x4[40]);
    butterfly_s16_s32_x8_1223_neon(cospi16, x3[54], x3[41], &raw mut x4[54], &raw mut x4[41]);
    butterfly_s16_s32_x8_1223_neon(cospi16, x3[53], x3[42], &raw mut x4[53], &raw mut x4[42]);
    butterfly_s16_s32_x8_1223_neon(cospi16, x3[52], x3[43], &raw mut x4[52], &raw mut x4[43]);
    // stage 5
    let mut x5: [int16x8_t; 64] = core::mem::zeroed();
    butterfly_dct_pre_s16_x8(x4.as_mut_ptr(), x5.as_mut_ptr(), 4);
    butterfly_s16_s32_x8_0112_neon(cospi32, x4[6], x4[5], &raw mut x5[6], &raw mut x5[5]);
    butterfly_dct_post_s16_x8(x3.as_mut_ptr().offset(((8)) as isize), x4.as_mut_ptr().offset(((8)) as isize), x5.as_mut_ptr().offset(((8)) as isize), 8);
    butterfly_s16_s32_x8_0112_neon(cospi16, x4[29], x4[18], &raw mut x5[29], &raw mut x5[18]);
    butterfly_s16_s32_x8_0112_neon(cospi16, x4[28], x4[19], &raw mut x5[28], &raw mut x5[19]);
    butterfly_s16_s32_x8_1223_neon(cospi16, x4[27], x4[20], &raw mut x5[27], &raw mut x5[20]);
    butterfly_s16_s32_x8_1223_neon(cospi16, x4[26], x4[21], &raw mut x5[26], &raw mut x5[21]);
    butterfly_dct_post_s16_x8(x3.as_mut_ptr().offset(((32)) as isize), x4.as_mut_ptr().offset(((32)) as isize), x5.as_mut_ptr().offset(((32)) as isize), 16);
    butterfly_dct_post_s16_x8(x3.as_mut_ptr().offset(((48)) as isize), x4.as_mut_ptr().offset(((48)) as isize), x5.as_mut_ptr().offset(((48)) as isize), 16);
    // stage 6
    let mut x6: [int16x8_t; 64] = core::mem::zeroed();
    butterfly_s16_s32_x8_0112_neon(cospi32, x5[1], x5[0], &raw mut x6[0], &raw mut x6[1]);
    butterfly_s16_s32_x8_0112_neon(cospi16, x5[3], x5[2], &raw mut x6[2], &raw mut x6[3]);
    butterfly_dct_post_s16_x8(x4.as_mut_ptr().offset(((4)) as isize), x5.as_mut_ptr().offset(((4)) as isize), x6.as_mut_ptr().offset(((4)) as isize), 4);
    butterfly_s16_s32_x8_0112_neon(cospi16, x5[14], x5[9], &raw mut x6[14], &raw mut x6[9]);
    butterfly_s16_s32_x8_1223_neon(cospi16, x5[13], x5[10], &raw mut x6[13], &raw mut x6[10]);
    butterfly_dct_post_s16_x8(x4.as_mut_ptr().offset(((16)) as isize), x5.as_mut_ptr().offset(((16)) as isize), x6.as_mut_ptr().offset(((16)) as isize), 8);
    butterfly_dct_post_s16_x8(x4.as_mut_ptr().offset(((24)) as isize), x5.as_mut_ptr().offset(((24)) as isize), x6.as_mut_ptr().offset(((24)) as isize), 8);
    butterfly_s16_s32_x8_0112_neon(cospi8, x5[61], x5[34], &raw mut x6[61], &raw mut x6[34]);
    butterfly_s16_s32_x8_0112_neon(cospi8, x5[60], x5[35], &raw mut x6[60], &raw mut x6[35]);
    butterfly_s16_s32_x8_1223_neon(cospi8, x5[59], x5[36], &raw mut x6[59], &raw mut x6[36]);
    butterfly_s16_s32_x8_1223_neon(cospi8, x5[58], x5[37], &raw mut x6[58], &raw mut x6[37]);
    butterfly_s16_s32_x8_1003_neon(cospi24, x5[53], x5[42], &raw mut x6[53], &raw mut x6[42]);
    butterfly_s16_s32_x8_1003_neon(cospi24, x5[52], x5[43], &raw mut x6[52], &raw mut x6[43]);
    butterfly_s16_s32_x8_0332_neon(cospi24, x5[51], x5[44], &raw mut x6[51], &raw mut x6[44]);
    butterfly_s16_s32_x8_0332_neon(cospi24, x5[50], x5[45], &raw mut x6[50], &raw mut x6[45]);
    // stage 7
    let mut x7: [int16x8_t; 64] = core::mem::zeroed();
    butterfly_s16_s32_x8_0112_neon(cospi8, x6[7], x6[4], &raw mut x7[4], &raw mut x7[7]);
    butterfly_s16_s32_x8_1003_neon(cospi24, x6[6], x6[5], &raw mut x7[5], &raw mut x7[6]);
    butterfly_dct_post_s16_x8(x5.as_mut_ptr().offset(((8)) as isize), x6.as_mut_ptr().offset(((8)) as isize), x7.as_mut_ptr().offset(((8)) as isize), 4);
    butterfly_dct_post_s16_x8(x5.as_mut_ptr().offset(((12)) as isize), x6.as_mut_ptr().offset(((12)) as isize), x7.as_mut_ptr().offset(((12)) as isize), 4);
    butterfly_s16_s32_x8_0112_neon(cospi8, x6[30], x6[17], &raw mut x7[30], &raw mut x7[17]);
    butterfly_s16_s32_x8_1223_neon(cospi8, x6[29], x6[18], &raw mut x7[29], &raw mut x7[18]);
    butterfly_s16_s32_x8_1003_neon(cospi24, x6[26], x6[21], &raw mut x7[26], &raw mut x7[21]);
    butterfly_s16_s32_x8_0332_neon(cospi24, x6[25], x6[22], &raw mut x7[25], &raw mut x7[22]);
    butterfly_dct_post_s16_x8(x5.as_mut_ptr().offset(((32)) as isize), x6.as_mut_ptr().offset(((32)) as isize), x7.as_mut_ptr().offset(((32)) as isize), 8);
    butterfly_dct_post_s16_x8(x5.as_mut_ptr().offset(((40)) as isize), x6.as_mut_ptr().offset(((40)) as isize), x7.as_mut_ptr().offset(((40)) as isize), 8);
    butterfly_dct_post_s16_x8(x5.as_mut_ptr().offset(((48)) as isize), x6.as_mut_ptr().offset(((48)) as isize), x7.as_mut_ptr().offset(((48)) as isize), 8);
    butterfly_dct_post_s16_x8(x5.as_mut_ptr().offset(((56)) as isize), x6.as_mut_ptr().offset(((56)) as isize), x7.as_mut_ptr().offset(((56)) as isize), 8);
    // stage 8
    let mut x8: [int16x8_t; 64] = core::mem::zeroed();
    butterfly_s16_s32_x8_0112_neon(cospi4, x7[15], x7[8], &raw mut x8[8], &raw mut x8[15]);
    butterfly_s16_s32_x8_1003_neon(cospi28, x7[14], x7[9], &raw mut x8[9], &raw mut x8[14]);
    butterfly_s16_s32_x8_0112_neon(cospi20, x7[13], x7[10], &raw mut x8[10], &raw mut x8[13]);
    butterfly_s16_s32_x8_1003_neon(cospi12, x7[12], x7[11], &raw mut x8[11], &raw mut x8[12]);
    butterfly_dct_post_s16_x8(x6.as_mut_ptr().offset(((16)) as isize), x7.as_mut_ptr().offset(((16)) as isize), x8.as_mut_ptr().offset(((16)) as isize), 4);
    butterfly_dct_post_s16_x8(x6.as_mut_ptr().offset(((20)) as isize), x7.as_mut_ptr().offset(((20)) as isize), x8.as_mut_ptr().offset(((20)) as isize), 4);
    butterfly_dct_post_s16_x8(x6.as_mut_ptr().offset(((24)) as isize), x7.as_mut_ptr().offset(((24)) as isize), x8.as_mut_ptr().offset(((24)) as isize), 4);
    butterfly_dct_post_s16_x8(x6.as_mut_ptr().offset(((28)) as isize), x7.as_mut_ptr().offset(((28)) as isize), x8.as_mut_ptr().offset(((28)) as isize), 4);
    butterfly_s16_s32_x8_0112_neon(cospi4, x7[62], x7[33], &raw mut x8[62], &raw mut x8[33]);
    butterfly_s16_s32_x8_1223_neon(cospi4, x7[61], x7[34], &raw mut x8[61], &raw mut x8[34]);
    butterfly_s16_s32_x8_1003_neon(cospi28, x7[58], x7[37], &raw mut x8[58], &raw mut x8[37]);
    butterfly_s16_s32_x8_0332_neon(cospi28, x7[57], x7[38], &raw mut x8[57], &raw mut x8[38]);
    butterfly_s16_s32_x8_0112_neon(cospi20, x7[54], x7[41], &raw mut x8[54], &raw mut x8[41]);
    butterfly_s16_s32_x8_1223_neon(cospi20, x7[53], x7[42], &raw mut x8[53], &raw mut x8[42]);
    butterfly_s16_s32_x8_1003_neon(cospi12, x7[50], x7[45], &raw mut x8[50], &raw mut x8[45]);
    butterfly_s16_s32_x8_0332_neon(cospi12, x7[49], x7[46], &raw mut x8[49], &raw mut x8[46]);
    // stage 9
    let mut x9: [int16x8_t; 64] = core::mem::zeroed();
    butterfly_s16_s32_x8_0112_neon(cospi2, x8[31], x8[16], &raw mut x9[16], &raw mut x9[31]);
    butterfly_s16_s32_x8_1003_neon(cospi30, x8[30], x8[17], &raw mut x9[17], &raw mut x9[30]);
    butterfly_s16_s32_x8_0112_neon(cospi18, x8[29], x8[18], &raw mut x9[18], &raw mut x9[29]);
    butterfly_s16_s32_x8_1003_neon(cospi14, x8[28], x8[19], &raw mut x9[19], &raw mut x9[28]);
    butterfly_s16_s32_x8_0112_neon(cospi10, x8[27], x8[20], &raw mut x9[20], &raw mut x9[27]);
    butterfly_s16_s32_x8_1003_neon(cospi22, x8[26], x8[21], &raw mut x9[21], &raw mut x9[26]);
    butterfly_s16_s32_x8_0112_neon(cospi26, x8[25], x8[22], &raw mut x9[22], &raw mut x9[25]);
    butterfly_s16_s32_x8_1003_neon(cospi6, x8[24], x8[23], &raw mut x9[23], &raw mut x9[24]);
    butterfly_dct_post_s16_x8(x7.as_mut_ptr().offset(((32)) as isize), x8.as_mut_ptr().offset(((32)) as isize), x9.as_mut_ptr().offset(((32)) as isize), 4);
    butterfly_dct_post_s16_x8(x7.as_mut_ptr().offset(((36)) as isize), x8.as_mut_ptr().offset(((36)) as isize), x9.as_mut_ptr().offset(((36)) as isize), 4);
    butterfly_dct_post_s16_x8(x7.as_mut_ptr().offset(((40)) as isize), x8.as_mut_ptr().offset(((40)) as isize), x9.as_mut_ptr().offset(((40)) as isize), 4);
    butterfly_dct_post_s16_x8(x7.as_mut_ptr().offset(((44)) as isize), x8.as_mut_ptr().offset(((44)) as isize), x9.as_mut_ptr().offset(((44)) as isize), 4);
    butterfly_dct_post_s16_x8(x7.as_mut_ptr().offset(((48)) as isize), x8.as_mut_ptr().offset(((48)) as isize), x9.as_mut_ptr().offset(((48)) as isize), 4);
    butterfly_dct_post_s16_x8(x7.as_mut_ptr().offset(((52)) as isize), x8.as_mut_ptr().offset(((52)) as isize), x9.as_mut_ptr().offset(((52)) as isize), 4);
    butterfly_dct_post_s16_x8(x7.as_mut_ptr().offset(((56)) as isize), x8.as_mut_ptr().offset(((56)) as isize), x9.as_mut_ptr().offset(((56)) as isize), 4);
    butterfly_dct_post_s16_x8(x7.as_mut_ptr().offset(((60)) as isize), x8.as_mut_ptr().offset(((60)) as isize), x9.as_mut_ptr().offset(((60)) as isize), 4);
    // stage 10
    butterfly_s16_s32_x8_0112_neon(cospi1, x9[63], x9[32], output.add(1), output.add(63));
    butterfly_s16_s32_x8_1003_neon(cospi31, x9[62], x9[33], output.add(33), output.add(31));
    butterfly_s16_s32_x8_0112_neon(cospi17, x9[61], x9[34], output.add(17), output.add(47));
    butterfly_s16_s32_x8_1003_neon(cospi15, x9[60], x9[35], output.add(49), output.add(15));
    butterfly_s16_s32_x8_0112_neon(cospi9, x9[59], x9[36], output.add(9), output.add(55));
    butterfly_s16_s32_x8_1003_neon(cospi23, x9[58], x9[37], output.add(41), output.add(23));
    butterfly_s16_s32_x8_0112_neon(cospi25, x9[57], x9[38], output.add(25), output.add(39));
    butterfly_s16_s32_x8_1003_neon(cospi7, x9[56], x9[39], output.add(57), output.add(7));
    butterfly_s16_s32_x8_0112_neon(cospi5, x9[55], x9[40], output.add(5), output.add(59));
    butterfly_s16_s32_x8_1003_neon(cospi27, x9[54], x9[41], output.add(37), output.add(27));
    butterfly_s16_s32_x8_0112_neon(cospi21, x9[53], x9[42], output.add(21), output.add(43));
    butterfly_s16_s32_x8_1003_neon(cospi11, x9[52], x9[43], output.add(53), output.add(11));
    butterfly_s16_s32_x8_0112_neon(cospi13, x9[51], x9[44], output.add(13), output.add(51));
    butterfly_s16_s32_x8_1003_neon(cospi19, x9[50], x9[45], output.add(45), output.add(19));
    butterfly_s16_s32_x8_0112_neon(cospi29, x9[49], x9[46], output.add(29), output.add(35));
    butterfly_s16_s32_x8_1003_neon(cospi3, x9[48], x9[47], output.add(61), output.add(3));
    // stage 11
    *output.add(0) = x6[0];
    *output.add(2) = x9[16];
    *output.add(4) = x8[8];
    *output.add(6) = x9[24];
    *output.add(8) = x7[4];
    *output.add(10) = x9[20];
    *output.add(12) = x8[12];
    *output.add(14) = x9[28];
    *output.add(16) = x6[2];
    *output.add(18) = x9[18];
    *output.add(20) = x8[10];
    *output.add(22) = x9[26];
    *output.add(24) = x7[6];
    *output.add(26) = x9[22];
    *output.add(28) = x8[14];
    *output.add(30) = x9[30];
    *output.add(32) = x6[1];
    *output.add(34) = x9[17];
    *output.add(36) = x8[9];
    *output.add(38) = x9[25];
    *output.add(40) = x7[5];
    *output.add(42) = x9[21];
    *output.add(44) = x8[13];
    *output.add(46) = x9[29];
    *output.add(48) = x6[3];
    *output.add(52) = x8[11];
    *output.add(54) = x9[27];
    *output.add(56) = x7[7];
    *output.add(58) = x9[23];
    *output.add(60) = x8[15];
    *output.add(62) = x9[31];
}

#[inline(always)]
unsafe fn fadst8x8_neon(mut input: *const int16x8_t, mut output: *mut int16x8_t, mut cos_bit: i32) {
    let mut cospi: *const i16 = cospi_arr_q13(cos_bit);
    let mut cospi32_16: int16x8_t = vld1q_s16(cospi.offset((4 * 0) as isize));
    let mut cospi4_12: int16x8_t = vld1q_s16(cospi.offset((4 * 4) as isize));
    let mut cospi20_28: int16x8_t = vld1q_s16(cospi.offset((4 * 6) as isize));
    let mut cospi32: int16x4_t = vget_low_s16(cospi32_16);
    let mut cospi16: int16x4_t = vget_high_s16(cospi32_16);
    let mut cospi4: int16x4_t = vget_low_s16(cospi4_12);
    let mut cospi12: int16x4_t = vget_high_s16(cospi4_12);
    let mut cospi20: int16x4_t = vget_low_s16(cospi20_28);
    let mut cospi28: int16x4_t = vget_high_s16(cospi20_28);
    // stage 2
    let mut x2: [int16x8_t; 8] = core::mem::zeroed();
    butterfly_s16_s32_x8_0332_neon(cospi32, *input.add(4), *input.add(3), &raw mut x2[2], &raw mut x2[3]);
    butterfly_s16_s32_x8_0112_neon(cospi32, *input.add(2), *input.add(5), &raw mut x2[7], &raw mut x2[6]);
    // stage 3
    let mut x3: [int16x8_t; 8] = core::mem::zeroed();
    x3[0] = vqaddq_s16(*input.add(0), x2[2]);
    x3[1] = vqsubq_s16(x2[3], *input.add(7));
    x3[2] = vqsubq_s16(*input.add(0), x2[2]);
    x3[3] = vqaddq_s16(*input.add(7), x2[3]);
    x3[4] = vqsubq_s16(x2[6], *input.add(1));
    x3[5] = vqaddq_s16(*input.add(6), x2[7]);
    x3[6] = vqaddq_s16(*input.add(1), x2[6]);
    x3[7] = vqsubq_s16(*input.add(6), x2[7]);
    // stage 4
    butterfly_s16_s32_x8_0112_neon(cospi16, x3[4], x3[5], &raw mut x3[4], &raw mut x3[5]);
    butterfly_s16_s32_x8_0112_neon(cospi16, x3[7], x3[6], &raw mut x3[6], &raw mut x3[7]);
    // stage 5
    let mut x5: [int16x8_t; 8] = core::mem::zeroed();
    x5[0] = vqaddq_s16(x3[0], x3[4]);
    x5[1] = vqaddq_s16(x3[1], x3[5]);
    x5[2] = vqaddq_s16(x3[2], x3[6]);
    x5[3] = vqsubq_s16(x3[7], x3[3]);
    x5[4] = vqsubq_s16(x3[0], x3[4]);
    x5[5] = vqsubq_s16(x3[1], x3[5]);
    x5[6] = vqsubq_s16(x3[2], x3[6]);
    x5[7] = vqaddq_s16(x3[3], x3[7]);
    // stage 6
    butterfly_s16_s32_x8_0112_neon(cospi4, x5[0], x5[1], output.add(7), output.add(0));
    butterfly_s16_s32_x8_0112_neon(cospi20, x5[2], x5[3], output.add(5), output.add(2));
    butterfly_s16_s32_x8_1003_neon(cospi28, x5[4], x5[5], output.add(3), output.add(4));
    butterfly_s16_s32_x8_0112_neon(cospi12, x5[6], x5[7], output.add(6), output.add(1));
}

#[inline(always)]
unsafe fn fadst4x16_neon(mut input: *const int16x4_t, mut output: *mut int16x4_t, mut cos_bit: i32) {
    let mut cospi: *const i16 = cospi_arr_q13(cos_bit);
    let mut cospi32_16: int16x8_t = vld1q_s16(cospi.offset((4 * 0) as isize));
    let mut cospi8_24: int16x8_t = vld1q_s16(cospi.offset((4 * 2) as isize));
    let mut cospi2_6: int16x8_t = vld1q_s16(cospi.offset((4 * 8) as isize));
    let mut cospi10_14: int16x8_t = vld1q_s16(cospi.offset((4 * 10) as isize));
    let mut cospi18_22: int16x8_t = vld1q_s16(cospi.offset((4 * 12) as isize));
    let mut cospi26_30: int16x8_t = vld1q_s16(cospi.offset((4 * 14) as isize));
    let mut cospi32: int16x4_t = vget_low_s16(cospi32_16);
    let mut cospi16: int16x4_t = vget_high_s16(cospi32_16);
    let mut cospi8: int16x4_t = vget_low_s16(cospi8_24);
    let mut cospi24: int16x4_t = vget_high_s16(cospi8_24);
    let mut cospi2: int16x4_t = vget_low_s16(cospi2_6);
    let mut cospi6: int16x4_t = vget_high_s16(cospi2_6);
    let mut cospi10: int16x4_t = vget_low_s16(cospi10_14);
    let mut cospi14: int16x4_t = vget_high_s16(cospi10_14);
    let mut cospi18: int16x4_t = vget_low_s16(cospi18_22);
    let mut cospi22: int16x4_t = vget_high_s16(cospi18_22);
    let mut cospi26: int16x4_t = vget_low_s16(cospi26_30);
    let mut cospi30: int16x4_t = vget_high_s16(cospi26_30);
    // stage 2
    let mut x2: [int16x4_t; 8] = core::mem::zeroed();
    butterfly_s16_s32_x4_0332_neon(cospi32, *input.add(8), *input.add(7), &raw mut x2[0], &raw mut x2[1]);
    butterfly_s16_s32_x4_0112_neon(cospi32, *input.add(4), *input.add(11), &raw mut x2[3], &raw mut x2[2]);
    butterfly_s16_s32_x4_0112_neon(cospi32, *input.add(6), *input.add(9), &raw mut x2[5], &raw mut x2[4]);
    butterfly_s16_s32_x4_0332_neon(cospi32, *input.add(10), *input.add(5), &raw mut x2[6], &raw mut x2[7]);
    // stage 3
    let mut x3: [int16x4_t; 16] = core::mem::zeroed();
    x3[0] = vqadd_s16(*input.add(0), x2[0]);
    x3[1] = vqsub_s16(x2[1], *input.add(15));
    x3[2] = vqsub_s16(*input.add(0), x2[0]);
    x3[3] = vqadd_s16(*input.add(15), x2[1]);
    x3[4] = vqsub_s16(x2[2], *input.add(3));
    x3[5] = vqadd_s16(*input.add(12), x2[3]);
    x3[6] = vqadd_s16(*input.add(3), x2[2]);
    x3[7] = vqsub_s16(*input.add(12), x2[3]);
    x3[8] = vqsub_s16(x2[4], *input.add(1));
    x3[9] = vqadd_s16(*input.add(14), x2[5]);
    x3[10] = vqadd_s16(*input.add(1), x2[4]);
    x3[11] = vqsub_s16(*input.add(14), x2[5]);
    x3[12] = vqadd_s16(*input.add(2), x2[6]);
    x3[13] = vqsub_s16(x2[7], *input.add(13));
    x3[14] = vqsub_s16(*input.add(2), x2[6]);
    x3[15] = vqadd_s16(*input.add(13), x2[7]);
    // stage 4
    butterfly_s16_s32_x4_0112_neon(cospi16, x3[4], x3[5], &raw mut x3[4], &raw mut x3[5]);
    butterfly_s16_s32_x4_0112_neon(cospi16, x3[7], x3[6], &raw mut x3[6], &raw mut x3[7]);
    butterfly_s16_s32_x4_0112_neon(cospi16, x3[12], x3[13], &raw mut x3[12], &raw mut x3[13]);
    butterfly_s16_s32_x4_0332_neon(cospi16, x3[14], x3[15], &raw mut x3[15], &raw mut x3[14]);
    // stage 5
    let mut x5: [int16x4_t; 16] = core::mem::zeroed();
    x5[0] = vqadd_s16(x3[0], x3[4]);
    x5[1] = vqadd_s16(x3[1], x3[5]);
    x5[2] = vqadd_s16(x3[2], x3[6]);
    x5[3] = vqsub_s16(x3[7], x3[3]);
    x5[4] = vqsub_s16(x3[0], x3[4]);
    x5[5] = vqsub_s16(x3[1], x3[5]);
    x5[6] = vqsub_s16(x3[2], x3[6]);
    x5[7] = vqadd_s16(x3[3], x3[7]);
    x5[8] = vqadd_s16(x3[8], x3[12]);
    x5[9] = vqadd_s16(x3[9], x3[13]);
    x5[10] = vqsub_s16(x3[14], x3[10]);
    x5[11] = vqadd_s16(x3[11], x3[15]);
    x5[12] = vqsub_s16(x3[8], x3[12]);
    x5[13] = vqsub_s16(x3[9], x3[13]);
    x5[14] = vqadd_s16(x3[10], x3[14]);
    x5[15] = vqsub_s16(x3[11], x3[15]);
    // stage 6
    butterfly_s16_s32_x4_0112_neon(cospi8, x5[8], x5[9], &raw mut x5[8], &raw mut x5[9]);
    butterfly_s16_s32_x4_1003_neon(cospi24, x5[10], x5[11], &raw mut x5[10], &raw mut x5[11]);
    butterfly_s16_s32_x4_1003_neon(cospi8, x5[13], x5[12], &raw mut x5[13], &raw mut x5[12]);
    butterfly_s16_s32_x4_1003_neon(cospi24, x5[15], x5[14], &raw mut x5[14], &raw mut x5[15]);
    // stage 7
    let mut x7: [int16x4_t; 16] = core::mem::zeroed();
    x7[0] = vqadd_s16(x5[0], x5[8]);
    x7[1] = vqadd_s16(x5[1], x5[9]);
    x7[2] = vqadd_s16(x5[2], x5[10]);
    x7[3] = vqadd_s16(x5[3], x5[11]);
    x7[4] = vqadd_s16(x5[4], x5[12]);
    x7[5] = vqadd_s16(x5[5], x5[13]);
    x7[6] = vqadd_s16(x5[6], x5[14]);
    x7[7] = vqsub_s16(x5[15], x5[7]);
    x7[8] = vqsub_s16(x5[0], x5[8]);
    x7[9] = vqsub_s16(x5[1], x5[9]);
    x7[10] = vqsub_s16(x5[2], x5[10]);
    x7[11] = vqsub_s16(x5[3], x5[11]);
    x7[12] = vqsub_s16(x5[4], x5[12]);
    x7[13] = vqsub_s16(x5[5], x5[13]);
    x7[14] = vqsub_s16(x5[6], x5[14]);
    x7[15] = vqadd_s16(x5[7], x5[15]);
    // stage 8
    butterfly_s16_s32_x4_0112_neon(cospi2, x7[0], x7[1], output.add(15), output.add(0));
    butterfly_s16_s32_x4_0112_neon(cospi10, x7[2], x7[3], output.add(13), output.add(2));
    butterfly_s16_s32_x4_0112_neon(cospi18, x7[4], x7[5], output.add(11), output.add(4));
    butterfly_s16_s32_x4_0112_neon(cospi26, x7[6], x7[7], output.add(9), output.add(6));
    butterfly_s16_s32_x4_1003_neon(cospi30, x7[8], x7[9], output.add(7), output.add(8));
    butterfly_s16_s32_x4_1003_neon(cospi22, x7[10], x7[11], output.add(5), output.add(10));
    butterfly_s16_s32_x4_1003_neon(cospi14, x7[12], x7[13], output.add(3), output.add(12));
    butterfly_s16_s32_x4_0112_neon(cospi6, x7[14], x7[15], output.add(14), output.add(1));
}

#[inline(always)]
unsafe fn fadst8x16_neon(mut input: *const int16x8_t, mut output: *mut int16x8_t, mut cos_bit: i32) {
    let mut cospi: *const i16 = cospi_arr_q13(cos_bit);
    let mut cospi32_16: int16x8_t = vld1q_s16(cospi.offset((4 * 0) as isize));
    let mut cospi8_24: int16x8_t = vld1q_s16(cospi.offset((4 * 2) as isize));
    let mut cospi2_6: int16x8_t = vld1q_s16(cospi.offset((4 * 8) as isize));
    let mut cospi10_14: int16x8_t = vld1q_s16(cospi.offset((4 * 10) as isize));
    let mut cospi18_22: int16x8_t = vld1q_s16(cospi.offset((4 * 12) as isize));
    let mut cospi26_30: int16x8_t = vld1q_s16(cospi.offset((4 * 14) as isize));
    let mut cospi32: int16x4_t = vget_low_s16(cospi32_16);
    let mut cospi16: int16x4_t = vget_high_s16(cospi32_16);
    let mut cospi8: int16x4_t = vget_low_s16(cospi8_24);
    let mut cospi24: int16x4_t = vget_high_s16(cospi8_24);
    let mut cospi2: int16x4_t = vget_low_s16(cospi2_6);
    let mut cospi6: int16x4_t = vget_high_s16(cospi2_6);
    let mut cospi10: int16x4_t = vget_low_s16(cospi10_14);
    let mut cospi14: int16x4_t = vget_high_s16(cospi10_14);
    let mut cospi18: int16x4_t = vget_low_s16(cospi18_22);
    let mut cospi22: int16x4_t = vget_high_s16(cospi18_22);
    let mut cospi26: int16x4_t = vget_low_s16(cospi26_30);
    let mut cospi30: int16x4_t = vget_high_s16(cospi26_30);
    // stage 2
    let mut x2: [int16x8_t; 8] = core::mem::zeroed();
    butterfly_s16_s32_x8_0332_neon(cospi32, *input.add(8), *input.add(7), &raw mut x2[0], &raw mut x2[1]);
    butterfly_s16_s32_x8_0112_neon(cospi32, *input.add(4), *input.add(11), &raw mut x2[3], &raw mut x2[2]);
    butterfly_s16_s32_x8_0112_neon(cospi32, *input.add(6), *input.add(9), &raw mut x2[5], &raw mut x2[4]);
    butterfly_s16_s32_x8_0332_neon(cospi32, *input.add(10), *input.add(5), &raw mut x2[6], &raw mut x2[7]);
    // stage 3
    let mut x3: [int16x8_t; 16] = core::mem::zeroed();
    x3[0] = vqaddq_s16(*input.add(0), x2[0]);
    x3[1] = vqsubq_s16(x2[1], *input.add(15));
    x3[2] = vqsubq_s16(*input.add(0), x2[0]);
    x3[3] = vqaddq_s16(*input.add(15), x2[1]);
    x3[4] = vqsubq_s16(x2[2], *input.add(3));
    x3[5] = vqaddq_s16(*input.add(12), x2[3]);
    x3[6] = vqaddq_s16(*input.add(3), x2[2]);
    x3[7] = vqsubq_s16(*input.add(12), x2[3]);
    x3[8] = vqsubq_s16(x2[4], *input.add(1));
    x3[9] = vqaddq_s16(*input.add(14), x2[5]);
    x3[10] = vqaddq_s16(*input.add(1), x2[4]);
    x3[11] = vqsubq_s16(*input.add(14), x2[5]);
    x3[12] = vqaddq_s16(*input.add(2), x2[6]);
    x3[13] = vqsubq_s16(x2[7], *input.add(13));
    x3[14] = vqsubq_s16(*input.add(2), x2[6]);
    x3[15] = vqaddq_s16(*input.add(13), x2[7]);
    // stage 4
    butterfly_s16_s32_x8_0112_neon(cospi16, x3[4], x3[5], &raw mut x3[4], &raw mut x3[5]);
    butterfly_s16_s32_x8_0112_neon(cospi16, x3[7], x3[6], &raw mut x3[6], &raw mut x3[7]);
    butterfly_s16_s32_x8_0112_neon(cospi16, x3[12], x3[13], &raw mut x3[12], &raw mut x3[13]);
    butterfly_s16_s32_x8_0332_neon(cospi16, x3[14], x3[15], &raw mut x3[15], &raw mut x3[14]);
    // stage 5
    let mut x5: [int16x8_t; 16] = core::mem::zeroed();
    x5[0] = vqaddq_s16(x3[0], x3[4]);
    x5[1] = vqaddq_s16(x3[1], x3[5]);
    x5[2] = vqaddq_s16(x3[2], x3[6]);
    x5[3] = vqsubq_s16(x3[7], x3[3]);
    x5[4] = vqsubq_s16(x3[0], x3[4]);
    x5[5] = vqsubq_s16(x3[1], x3[5]);
    x5[6] = vqsubq_s16(x3[2], x3[6]);
    x5[7] = vqaddq_s16(x3[3], x3[7]);
    x5[8] = vqaddq_s16(x3[8], x3[12]);
    x5[9] = vqaddq_s16(x3[9], x3[13]);
    x5[10] = vqsubq_s16(x3[14], x3[10]);
    x5[11] = vqaddq_s16(x3[11], x3[15]);
    x5[12] = vqsubq_s16(x3[8], x3[12]);
    x5[13] = vqsubq_s16(x3[9], x3[13]);
    x5[14] = vqaddq_s16(x3[10], x3[14]);
    x5[15] = vqsubq_s16(x3[11], x3[15]);
    // stage 6
    butterfly_s16_s32_x8_0112_neon(cospi8, x5[8], x5[9], &raw mut x5[8], &raw mut x5[9]);
    butterfly_s16_s32_x8_1003_neon(cospi24, x5[10], x5[11], &raw mut x5[10], &raw mut x5[11]);
    butterfly_s16_s32_x8_1003_neon(cospi8, x5[13], x5[12], &raw mut x5[13], &raw mut x5[12]);
    butterfly_s16_s32_x8_1003_neon(cospi24, x5[15], x5[14], &raw mut x5[14], &raw mut x5[15]);
    // stage 7
    let mut x7: [int16x8_t; 16] = core::mem::zeroed();
    x7[0] = vqaddq_s16(x5[0], x5[8]);
    x7[1] = vqaddq_s16(x5[1], x5[9]);
    x7[2] = vqaddq_s16(x5[2], x5[10]);
    x7[3] = vqaddq_s16(x5[3], x5[11]);
    x7[4] = vqaddq_s16(x5[4], x5[12]);
    x7[5] = vqaddq_s16(x5[5], x5[13]);
    x7[6] = vqaddq_s16(x5[6], x5[14]);
    x7[7] = vqsubq_s16(x5[15], x5[7]);
    x7[8] = vqsubq_s16(x5[0], x5[8]);
    x7[9] = vqsubq_s16(x5[1], x5[9]);
    x7[10] = vqsubq_s16(x5[2], x5[10]);
    x7[11] = vqsubq_s16(x5[3], x5[11]);
    x7[12] = vqsubq_s16(x5[4], x5[12]);
    x7[13] = vqsubq_s16(x5[5], x5[13]);
    x7[14] = vqsubq_s16(x5[6], x5[14]);
    x7[15] = vqaddq_s16(x5[7], x5[15]);
    // stage 8
    butterfly_s16_s32_x8_0112_neon(cospi2, x7[0], x7[1], output.add(15), output.add(0));
    butterfly_s16_s32_x8_0112_neon(cospi10, x7[2], x7[3], output.add(13), output.add(2));
    butterfly_s16_s32_x8_0112_neon(cospi18, x7[4], x7[5], output.add(11), output.add(4));
    butterfly_s16_s32_x8_0112_neon(cospi26, x7[6], x7[7], output.add(9), output.add(6));
    butterfly_s16_s32_x8_1003_neon(cospi30, x7[8], x7[9], output.add(7), output.add(8));
    butterfly_s16_s32_x8_1003_neon(cospi22, x7[10], x7[11], output.add(5), output.add(10));
    butterfly_s16_s32_x8_1003_neon(cospi14, x7[12], x7[13], output.add(3), output.add(12));
    butterfly_s16_s32_x8_0112_neon(cospi6, x7[14], x7[15], output.add(14), output.add(1));
}

#[inline(always)]
unsafe fn fidentity4x4_neon(mut input: *const int16x4_t, mut output: *mut int16x4_t, mut cos_bit: i32) {
    round_shift_sqrt2_s16_s16_4xn_neon(input, output, 4);
}

#[inline(always)]
unsafe fn fidentity8x4_neon(mut input: *const int16x8_t, mut output: *mut int16x8_t, mut cos_bit: i32) {
    round_shift_sqrt2_s16_s16_8xn_neon(input, output, 4);
}

#[inline(always)]
unsafe fn fidentity4x8_neon(mut input: *const int16x4_t, mut output: *mut int16x4_t, mut cos_bit: i32) {
    shift_left_1_s16_x4(input, output, 8);
}

#[inline(always)]
unsafe fn fidentity8x8_neon(mut input: *const int16x8_t, mut output: *mut int16x8_t, mut cos_bit: i32) {
    shift_left_1_s16_x8(input, output, 8);
}

#[inline(always)]
unsafe fn fidentity4x16_neon(mut input: *const int16x4_t, mut output: *mut int16x4_t, mut cos_bit: i32) {
    round_shift_2sqrt2_s16_s16_4xn_neon(input, output, 16);
}

#[inline(always)]
unsafe fn fidentity8x16_neon(mut input: *const int16x8_t, mut output: *mut int16x8_t, mut cos_bit: i32) {
    round_shift_2sqrt2_s16_s16_8xn_neon(input, output, 16);
}

#[inline(always)]
unsafe fn fidentity8x32_neon(mut input: *const int16x8_t, mut output: *mut int16x8_t, mut cos_bit: i32) {
    shift_left_2_s16_x8(input, output, 32);
}


macro_rules! TRANSFORM_COL {
    ($col:ident, $name:ident, $tw:ident, $load:ident, $shift:ident, $n:literal) => {
        unsafe fn $col(input: *const i16, output: *mut $tw, stride: i32, cos_bit: i32) {
            let mut buf0: [$tw; $n] = core::mem::zeroed();
            $load(input, stride, buf0.as_mut_ptr(), $n);
            $shift(buf0.as_mut_ptr(), buf0.as_mut_ptr(), $n);
            $name(buf0.as_mut_ptr(), output, cos_bit);
        }
    };
}

TRANSFORM_COL!(fadst4x4_col_neon, fadst4x4_neon, int16x4_t, load_buffer_s16_x4, shift_left_2_s16_x4, 4);
TRANSFORM_COL!(fadst4x8_col_neon, fadst4x8_neon, int16x4_t, load_buffer_s16_x4, shift_left_2_s16_x4, 8);
TRANSFORM_COL!(fadst4x16_col_neon, fadst4x16_neon, int16x4_t, load_buffer_s16_x4, shift_left_2_s16_x4, 16);
TRANSFORM_COL!(fadst8x4_col_neon, fadst8x4_neon, int16x8_t, load_buffer_s16_x8, shift_left_2_s16_x8, 4);
TRANSFORM_COL!(fadst8x8_col_neon, fadst8x8_neon, int16x8_t, load_buffer_s16_x8, shift_left_2_s16_x8, 8);
TRANSFORM_COL!(fadst8x16_col_neon, fadst8x16_neon, int16x8_t, load_buffer_s16_x8, shift_left_2_s16_x8, 16);
TRANSFORM_COL!(fdct4x4_col_neon, fdct4x4_neon, int16x4_t, load_buffer_s16_x4, shift_left_2_s16_x4, 4);
TRANSFORM_COL!(fdct4x8_col_neon, fdct4x8_neon, int16x4_t, load_buffer_s16_x4, shift_left_2_s16_x4, 8);
TRANSFORM_COL!(fdct4x16_col_neon, fdct4x16_neon, int16x4_t, load_buffer_s16_x4, shift_left_2_s16_x4, 16);
TRANSFORM_COL!(fdct8x4_col_neon, fdct8x4_neon, int16x8_t, load_buffer_s16_x8, shift_left_2_s16_x8, 4);
TRANSFORM_COL!(fdct8x8_col_neon, fdct8x8_neon, int16x8_t, load_buffer_s16_x8, shift_left_2_s16_x8, 8);
TRANSFORM_COL!(fdct8x16_col_neon, fdct8x16_neon, int16x8_t, load_buffer_s16_x8, shift_left_2_s16_x8, 16);
TRANSFORM_COL!(fdct8x32_col_neon, fdct8x32_neon, int16x8_t, load_buffer_s16_x8, shift_left_2_s16_x8, 32);
TRANSFORM_COL!(fidentity4x4_col_neon, fidentity4x4_neon, int16x4_t, load_buffer_s16_x4, shift_left_2_s16_x4, 4);
TRANSFORM_COL!(fidentity4x8_col_neon, fidentity4x8_neon, int16x4_t, load_buffer_s16_x4, shift_left_2_s16_x4, 8);
TRANSFORM_COL!(fidentity4x16_col_neon, fidentity4x16_neon, int16x4_t, load_buffer_s16_x4, shift_left_2_s16_x4, 16);
TRANSFORM_COL!(fidentity8x4_col_neon, fidentity8x4_neon, int16x8_t, load_buffer_s16_x8, shift_left_2_s16_x8, 4);
TRANSFORM_COL!(fidentity8x8_col_neon, fidentity8x8_neon, int16x8_t, load_buffer_s16_x8, shift_left_2_s16_x8, 8);
TRANSFORM_COL!(fidentity8x16_col_neon, fidentity8x16_neon, int16x8_t, load_buffer_s16_x8, shift_left_2_s16_x8, 16);
TRANSFORM_COL!(fidentity8x32_col_neon, fidentity8x32_neon, int16x8_t, load_buffer_s16_x8, shift_left_2_s16_x8, 32);

macro_rules! TRANSFORM_ROW {
    ($row:ident, $name:ident, $tw:ident, $store:ident, $n:literal) => {
        unsafe fn $row(input: *const $tw, output: *mut i32, stride: i32, cos_bit: i32) {
            let mut buf0: [$tw; $n] = core::mem::zeroed();
            $name(input, buf0.as_mut_ptr(), cos_bit);
            $store(buf0.as_mut_ptr(), output, stride, $n);
        }
    };
}

TRANSFORM_ROW!(fadst4x4_row_neon, fadst4x4_neon, int16x4_t, store_buffer_s16_x4, 4);
TRANSFORM_ROW!(fadst4x16_row_neon, fadst4x16_neon, int16x4_t, store_buffer_s16_x4, 16);
TRANSFORM_ROW!(fadst8x4_row_neon, fadst8x4_neon, int16x8_t, store_buffer_s16_x8, 4);
TRANSFORM_ROW!(fadst8x8_row_neon, fadst8x8_neon, int16x8_t, store_buffer_s16_x8, 8);
TRANSFORM_ROW!(fadst8x16_row_neon, fadst8x16_neon, int16x8_t, store_buffer_s16_x8, 16);
TRANSFORM_ROW!(fdct4x4_row_neon, fdct4x4_neon, int16x4_t, store_buffer_s16_x4, 4);
TRANSFORM_ROW!(fdct4x16_row_neon, fdct4x16_neon, int16x4_t, store_buffer_s16_x4, 16);
TRANSFORM_ROW!(fdct8x4_row_neon, fdct8x4_neon, int16x8_t, store_buffer_s16_x8, 4);
TRANSFORM_ROW!(fdct8x8_row_neon, fdct8x8_neon, int16x8_t, store_buffer_s16_x8, 8);
TRANSFORM_ROW!(fdct8x16_row_neon, fdct8x16_neon, int16x8_t, store_buffer_s16_x8, 16);
TRANSFORM_ROW!(fdct8x32_row_neon, fdct8x32_neon, int16x8_t, store_buffer_s16_x8, 32);
TRANSFORM_ROW!(fidentity4x4_row_neon, fidentity4x4_neon, int16x4_t, store_buffer_s16_x4, 4);
TRANSFORM_ROW!(fidentity4x16_row_neon, fidentity4x16_neon, int16x4_t, store_buffer_s16_x4, 16);
TRANSFORM_ROW!(fidentity8x4_row_neon, fidentity8x4_neon, int16x8_t, store_buffer_s16_x8, 4);
TRANSFORM_ROW!(fidentity8x8_row_neon, fidentity8x8_neon, int16x8_t, store_buffer_s16_x8, 8);
TRANSFORM_ROW!(fidentity8x16_row_neon, fidentity8x16_neon, int16x8_t, store_buffer_s16_x8, 16);
TRANSFORM_ROW!(fidentity8x32_row_neon, fidentity8x32_neon, int16x8_t, store_buffer_s16_x8, 32);

TRANSFORM_ROW!(fadst4x8_row_rect_neon, fadst4x8_neon, int16x4_t, store_rect_buffer_s16_x4, 8);
TRANSFORM_ROW!(fadst8x4_row_rect_neon, fadst8x4_neon, int16x8_t, store_rect_buffer_s16_x8, 4);
TRANSFORM_ROW!(fadst8x8_row_rect_neon, fadst8x8_neon, int16x8_t, store_rect_buffer_s16_x8, 8);
TRANSFORM_ROW!(fadst8x16_row_rect_neon, fadst8x16_neon, int16x8_t, store_rect_buffer_s16_x8, 16);
TRANSFORM_ROW!(fdct4x8_row_rect_neon, fdct4x8_neon, int16x4_t, store_rect_buffer_s16_x4, 8);
TRANSFORM_ROW!(fdct8x4_row_rect_neon, fdct8x4_neon, int16x8_t, store_rect_buffer_s16_x8, 4);
TRANSFORM_ROW!(fdct8x8_row_rect_neon, fdct8x8_neon, int16x8_t, store_rect_buffer_s16_x8, 8);
TRANSFORM_ROW!(fdct8x16_row_rect_neon, fdct8x16_neon, int16x8_t, store_rect_buffer_s16_x8, 16);
TRANSFORM_ROW!(fdct8x32_row_rect_neon, fdct8x32_neon, int16x8_t, store_rect_buffer_s16_x8, 32);
TRANSFORM_ROW!(fidentity4x8_row_rect_neon, fidentity4x8_neon, int16x4_t, store_rect_buffer_s16_x4, 8);
TRANSFORM_ROW!(fidentity8x4_row_rect_neon, fidentity8x4_neon, int16x8_t, store_rect_buffer_s16_x8, 4);
TRANSFORM_ROW!(fidentity8x8_row_rect_neon, fidentity8x8_neon, int16x8_t, store_rect_buffer_s16_x8, 8);
TRANSFORM_ROW!(fidentity8x16_row_rect_neon, fidentity8x16_neon, int16x8_t, store_rect_buffer_s16_x8, 16);
TRANSFORM_ROW!(fidentity8x32_row_rect_neon, fidentity8x32_neon, int16x8_t, store_rect_buffer_s16_x8, 32);

type transform_1d_lbd_4_neon = Option<unsafe fn(*const int16x4_t, *mut int16x4_t, i32)>;
type transform_1d_lbd_8_neon = Option<unsafe fn(*const int16x8_t, *mut int16x8_t, i32)>;

type col_transform_1d_lbd_4_neon = Option<unsafe fn(*const i16, *mut int16x4_t, i32, i32)>;
type col_transform_1d_lbd_8_neon = Option<unsafe fn(*const i16, *mut int16x8_t, i32, i32)>;

type row_transform_1d_lbd_4_neon = Option<unsafe fn(*const int16x4_t, *mut i32, i32, i32)>;
type row_transform_1d_lbd_8_neon = Option<unsafe fn(*const int16x8_t, *mut i32, i32, i32)>;

static col_txfm4x8_arr: [col_transform_1d_lbd_4_neon; TX_TYPES] = [
    Some(fdct4x8_col_neon), // DCT_DCT
    Some(fadst4x8_col_neon), // ADST_DCT
    Some(fdct4x8_col_neon), // DCT_ADST
    Some(fadst4x8_col_neon), // ADST_ADST
    Some(fadst4x8_col_neon), // FLIPADST_DCT
    Some(fdct4x8_col_neon), // DCT_FLIPADST
    Some(fadst4x8_col_neon), // FLIPADST_FLIPADST
    Some(fadst4x8_col_neon), // ADST_FLIPADST
    Some(fadst4x8_col_neon), // FLIPADST_ADST
    Some(fidentity4x8_col_neon), // IDTX
    Some(fdct4x8_col_neon), // V_DCT
    Some(fidentity4x8_col_neon), // H_DCT
    Some(fadst4x8_col_neon), // V_ADST
    Some(fidentity4x8_col_neon), // H_ADST
    Some(fadst4x8_col_neon), // V_FLIPADST
    Some(fidentity4x8_col_neon), // H_FLIPADST
];

static row_txfm8x4_arr: [row_transform_1d_lbd_8_neon; TX_TYPES] = [
    Some(fdct8x4_row_neon), // DCT_DCT
    Some(fdct8x4_row_neon), // ADST_DCT
    Some(fadst8x4_row_neon), // DCT_ADST
    Some(fadst8x4_row_neon), // ADST_ADST
    Some(fdct8x4_row_neon), // FLIPADST_DCT
    Some(fadst8x4_row_neon), // DCT_FLIPADST
    Some(fadst8x4_row_neon), // FLIPADST_FLIPADST
    Some(fadst8x4_row_neon), // ADST_FLIPADST
    Some(fadst8x4_row_neon), // FLIPADST_ADST
    Some(fidentity8x4_row_neon), // IDTX
    Some(fidentity8x4_row_neon), // V_DCT
    Some(fdct8x4_row_neon), // H_DCT
    Some(fidentity8x4_row_neon), // V_ADST
    Some(fadst8x4_row_neon), // H_ADST
    Some(fidentity8x4_row_neon), // V_FLIPADST
    Some(fadst8x4_row_neon), // H_FLIPADST
];

static row_rect_txfm8x4_arr: [row_transform_1d_lbd_8_neon; TX_TYPES] = [
    Some(fdct8x4_row_rect_neon), // DCT_DCT
    Some(fdct8x4_row_rect_neon), // ADST_DCT
    Some(fadst8x4_row_rect_neon), // DCT_ADST
    Some(fadst8x4_row_rect_neon), // ADST_ADST
    Some(fdct8x4_row_rect_neon), // FLIPADST_DCT
    Some(fadst8x4_row_rect_neon), // DCT_FLIPADST
    Some(fadst8x4_row_rect_neon), // FLIPADST_FLIPADST
    Some(fadst8x4_row_rect_neon), // ADST_FLIPADST
    Some(fadst8x4_row_rect_neon), // FLIPADST_ADST
    Some(fidentity8x4_row_rect_neon), // IDTX
    Some(fidentity8x4_row_rect_neon), // V_DCT
    Some(fdct8x4_row_rect_neon), // H_DCT
    Some(fidentity8x4_row_rect_neon), // V_ADST
    Some(fadst8x4_row_rect_neon), // H_ADST
    Some(fidentity8x4_row_rect_neon), // V_FLIPADST
    Some(fadst8x4_row_rect_neon), // H_FLIPADST
];

static col_txfm8x4_arr: [col_transform_1d_lbd_8_neon; TX_TYPES] = [
    Some(fdct8x4_col_neon), // DCT_DCT
    Some(fadst8x4_col_neon), // ADST_DCT
    Some(fdct8x4_col_neon), // DCT_ADST
    Some(fadst8x4_col_neon), // ADST_ADST
    Some(fadst8x4_col_neon), // FLIPADST_DCT
    Some(fdct8x4_col_neon), // DCT_FLIPADST
    Some(fadst8x4_col_neon), // FLIPADST_FLIPADST
    Some(fadst8x4_col_neon), // ADST_FLIPADST
    Some(fadst8x4_col_neon), // FLIPADST_ADST
    Some(fidentity8x4_col_neon), // IDTX
    Some(fdct8x4_col_neon), // V_DCT
    Some(fidentity8x4_col_neon), // H_DCT
    Some(fadst8x4_col_neon), // V_ADST
    Some(fidentity8x4_col_neon), // H_ADST
    Some(fadst8x4_col_neon), // V_FLIPADST
    Some(fidentity8x4_col_neon), // H_FLIPADST
];

static row_rect_txfm4x8_arr: [row_transform_1d_lbd_4_neon; TX_TYPES] = [
    Some(fdct4x8_row_rect_neon), // DCT_DCT
    Some(fdct4x8_row_rect_neon), // ADST_DCT
    Some(fadst4x8_row_rect_neon), // DCT_ADST
    Some(fadst4x8_row_rect_neon), // ADST_ADST
    Some(fdct4x8_row_rect_neon), // FLIPADST_DCT
    Some(fadst4x8_row_rect_neon), // DCT_FLIPADST
    Some(fadst4x8_row_rect_neon), // FLIPADST_FLIPADST
    Some(fadst4x8_row_rect_neon), // ADST_FLIPADST
    Some(fadst4x8_row_rect_neon), // FLIPADST_ADST
    Some(fidentity4x8_row_rect_neon), // IDTX
    Some(fidentity4x8_row_rect_neon), // V_DCT
    Some(fdct4x8_row_rect_neon), // H_DCT
    Some(fidentity4x8_row_rect_neon), // V_ADST
    Some(fadst4x8_row_rect_neon), // H_ADST
    Some(fidentity4x8_row_rect_neon), // V_FLIPADST
    Some(fadst4x8_row_rect_neon), // H_FLIPADST
];

static col_txfm8x8_arr: [col_transform_1d_lbd_8_neon; TX_TYPES] = [
    Some(fdct8x8_col_neon), // DCT_DCT
    Some(fadst8x8_col_neon), // ADST_DCT
    Some(fdct8x8_col_neon), // DCT_ADST
    Some(fadst8x8_col_neon), // ADST_ADST
    Some(fadst8x8_col_neon), // FLIPADST_DCT
    Some(fdct8x8_col_neon), // DCT_FLIPADST
    Some(fadst8x8_col_neon), // FLIPADST_FLIPADST
    Some(fadst8x8_col_neon), // ADST_FLIPADST
    Some(fadst8x8_col_neon), // FLIPADST_ADST
    Some(fidentity8x8_col_neon), // IDTX
    Some(fdct8x8_col_neon), // V_DCT
    Some(fidentity8x8_col_neon), // H_DCT
    Some(fadst8x8_col_neon), // V_ADST
    Some(fidentity8x8_col_neon), // H_ADST
    Some(fadst8x8_col_neon), // V_FLIPADST
    Some(fidentity8x8_col_neon), // H_FLIPADST
];

static row_txfm8x8_arr: [row_transform_1d_lbd_8_neon; TX_TYPES] = [
    Some(fdct8x8_row_neon), // DCT_DCT
    Some(fdct8x8_row_neon), // ADST_DCT
    Some(fadst8x8_row_neon), // DCT_ADST
    Some(fadst8x8_row_neon), // ADST_ADST
    Some(fdct8x8_row_neon), // FLIPADST_DCT
    Some(fadst8x8_row_neon), // DCT_FLIPADST
    Some(fadst8x8_row_neon), // FLIPADST_FLIPADST
    Some(fadst8x8_row_neon), // ADST_FLIPADST
    Some(fadst8x8_row_neon), // FLIPADST_ADST
    Some(fidentity8x8_row_neon), // IDTX
    Some(fidentity8x8_row_neon), // V_DCT
    Some(fdct8x8_row_neon), // H_DCT
    Some(fidentity8x8_row_neon), // V_ADST
    Some(fadst8x8_row_neon), // H_ADST
    Some(fidentity8x8_row_neon), // V_FLIPADST
    Some(fadst8x8_row_neon), // H_FLIPADST
];

static row_rect_txfm8x8_arr: [row_transform_1d_lbd_8_neon; TX_TYPES] = [
    Some(fdct8x8_row_rect_neon), // DCT_DCT
    Some(fdct8x8_row_rect_neon), // ADST_DCT
    Some(fadst8x8_row_rect_neon), // DCT_ADST
    Some(fadst8x8_row_rect_neon), // ADST_ADST
    Some(fdct8x8_row_rect_neon), // FLIPADST_DCT
    Some(fadst8x8_row_rect_neon), // DCT_FLIPADST
    Some(fadst8x8_row_rect_neon), // FLIPADST_FLIPADST
    Some(fadst8x8_row_rect_neon), // ADST_FLIPADST
    Some(fadst8x8_row_rect_neon), // FLIPADST_ADST
    Some(fidentity8x8_row_rect_neon), // IDTX
    Some(fidentity8x8_row_rect_neon), // V_DCT
    Some(fdct8x8_row_rect_neon), // H_DCT
    Some(fidentity8x8_row_rect_neon), // V_ADST
    Some(fadst8x8_row_rect_neon), // H_ADST
    Some(fidentity8x8_row_rect_neon), // V_FLIPADST
    Some(fadst8x8_row_rect_neon), // H_FLIPADST
];

static col_txfm4x16_arr: [col_transform_1d_lbd_4_neon; TX_TYPES] = [
    Some(fdct4x16_col_neon), // DCT_DCT
    Some(fadst4x16_col_neon), // ADST_DCT
    Some(fdct4x16_col_neon), // DCT_ADST
    Some(fadst4x16_col_neon), // ADST_ADST
    Some(fadst4x16_col_neon), // FLIPADST_DCT
    Some(fdct4x16_col_neon), // DCT_FLIPADST
    Some(fadst4x16_col_neon), // FLIPADST_FLIPADST
    Some(fadst4x16_col_neon), // ADST_FLIPADST
    Some(fadst4x16_col_neon), // FLIPADST_ADST
    Some(fidentity4x16_col_neon), // IDTX
    Some(fdct4x16_col_neon), // V_DCT
    Some(fidentity4x16_col_neon), // H_DCT
    Some(fadst4x16_col_neon), // V_ADST
    Some(fidentity4x16_col_neon), // H_ADST
    Some(fadst4x16_col_neon), // V_FLIPADST
    Some(fidentity4x16_col_neon), // H_FLIPADST
];

static row_txfm4x16_arr: [row_transform_1d_lbd_4_neon; TX_TYPES] = [
    Some(fdct4x16_row_neon), // DCT_DCT
    Some(fdct4x16_row_neon), // ADST_DCT
    Some(fadst4x16_row_neon), // DCT_ADST
    Some(fadst4x16_row_neon), // ADST_ADST
    Some(fdct4x16_row_neon), // FLIPADST_DCT
    Some(fadst4x16_row_neon), // DCT_FLIPADST
    Some(fadst4x16_row_neon), // FLIPADST_FLIPADST
    Some(fadst4x16_row_neon), // ADST_FLIPADST
    Some(fadst4x16_row_neon), // FLIPADST_ADST
    Some(fidentity4x16_row_neon), // IDTX
    Some(fidentity4x16_row_neon), // V_DCT
    Some(fdct4x16_row_neon), // H_DCT
    Some(fidentity4x16_row_neon), // V_ADST
    Some(fadst4x16_row_neon), // H_ADST
    Some(fidentity4x16_row_neon), // V_FLIPADST
    Some(fadst4x16_row_neon), // H_FLIPADST
];

static col_txfm8x16_arr: [col_transform_1d_lbd_8_neon; TX_TYPES] = [
    Some(fdct8x16_col_neon), // DCT_DCT
    Some(fadst8x16_col_neon), // ADST_DCT
    Some(fdct8x16_col_neon), // DCT_ADST
    Some(fadst8x16_col_neon), // ADST_ADST
    Some(fadst8x16_col_neon), // FLIPADST_DCT
    Some(fdct8x16_col_neon), // DCT_FLIPADST
    Some(fadst8x16_col_neon), // FLIPADST_FLIPADST
    Some(fadst8x16_col_neon), // ADST_FLIPADST
    Some(fadst8x16_col_neon), // FLIPADST_ADST
    Some(fidentity8x16_col_neon), // IDTX
    Some(fdct8x16_col_neon), // V_DCT
    Some(fidentity8x16_col_neon), // H_DCT
    Some(fadst8x16_col_neon), // V_ADST
    Some(fidentity8x16_col_neon), // H_ADST
    Some(fadst8x16_col_neon), // V_FLIPADST
    Some(fidentity8x16_col_neon), // H_FLIPADST
];

static row_txfm8x16_arr: [row_transform_1d_lbd_8_neon; TX_TYPES] = [
    Some(fdct8x16_row_neon), // DCT_DCT
    Some(fdct8x16_row_neon), // ADST_DCT
    Some(fadst8x16_row_neon), // DCT_ADST
    Some(fadst8x16_row_neon), // ADST_ADST
    Some(fdct8x16_row_neon), // FLIPADST_DCT
    Some(fadst8x16_row_neon), // DCT_FLIPADST
    Some(fadst8x16_row_neon), // FLIPADST_FLIPADST
    Some(fadst8x16_row_neon), // ADST_FLIPADST
    Some(fadst8x16_row_neon), // FLIPADST_ADST
    Some(fidentity8x16_row_neon), // IDTX
    Some(fidentity8x16_row_neon), // V_DCT
    Some(fdct8x16_row_neon), // H_DCT
    Some(fidentity8x16_row_neon), // V_ADST
    Some(fadst8x16_row_neon), // H_ADST
    Some(fidentity8x16_row_neon), // V_FLIPADST
    Some(fadst8x16_row_neon), // H_FLIPADST
];

static row_rect_txfm8x16_arr: [row_transform_1d_lbd_8_neon; TX_TYPES] = [
    Some(fdct8x16_row_rect_neon), // DCT_DCT
    Some(fdct8x16_row_rect_neon), // ADST_DCT
    Some(fadst8x16_row_rect_neon), // DCT_ADST
    Some(fadst8x16_row_rect_neon), // ADST_ADST
    Some(fdct8x16_row_rect_neon), // FLIPADST_DCT
    Some(fadst8x16_row_rect_neon), // DCT_FLIPADST
    Some(fadst8x16_row_rect_neon), // FLIPADST_FLIPADST
    Some(fadst8x16_row_rect_neon), // ADST_FLIPADST
    Some(fadst8x16_row_rect_neon), // FLIPADST_ADST
    Some(fidentity8x16_row_rect_neon), // IDTX
    Some(fidentity8x16_row_rect_neon), // V_DCT
    Some(fdct8x16_row_rect_neon), // H_DCT
    Some(fidentity8x16_row_rect_neon), // V_ADST
    Some(fadst8x16_row_rect_neon), // H_ADST
    Some(fidentity8x16_row_rect_neon), // V_FLIPADST
    Some(fadst8x16_row_rect_neon), // H_FLIPADST
];

static row_txfm8x32_arr: [row_transform_1d_lbd_8_neon; TX_TYPES] = [
    Some(fdct8x32_row_neon), // DCT_DCT
    None, // ADST_DCT
    None, // DCT_ADST
    None, // ADST_ADST
    None, // FLIPADST_DCT
    None, // DCT_FLIPADST
    None, // FLIPADST_FLIPADST
    None, // ADST_FLIPADST
    None, // FLIPADST_ADST
    Some(fidentity8x32_row_neon), // IDTX
    Some(fidentity8x32_row_neon), // V_DCT
    Some(fdct8x32_row_neon), // H_DCT
    None, // V_ADST
    None, // H_ADST
    None, // V_FLIPADST
    None, // H_FLIPADST
];

static row_rect_txfm8x32_arr: [row_transform_1d_lbd_8_neon; TX_TYPES] = [
    Some(fdct8x32_row_rect_neon), // DCT_DCT
    None, // ADST_DCT
    None, // DCT_ADST
    None, // ADST_ADST
    None, // FLIPADST_DCT
    None, // DCT_FLIPADST
    None, // FLIPADST_FLIPADST
    None, // ADST_FLIPADST
    None, // FLIPADST_ADST
    Some(fidentity8x32_row_rect_neon), // IDTX
    Some(fidentity8x32_row_rect_neon), // V_DCT
    Some(fdct8x32_row_rect_neon), // H_DCT
    None, // V_ADST
    None, // H_ADST
    None, // V_FLIPADST
    None, // H_FLIPADST
];

static col_txfm8x32_arr: [col_transform_1d_lbd_8_neon; TX_TYPES] = [
    Some(fdct8x32_col_neon), // DCT_DCT
    None, // ADST_DCT
    None, // DCT_ADST
    None, // ADST_ADST
    None, // FLIPADST_DCT
    None, // DCT_FLIPADST
    None, // FLIPADST_FLIPADST
    None, // ADST_FLIPADST
    None, // FLIPADST_ADST
    Some(fidentity8x32_col_neon), // IDTX
    Some(fdct8x32_col_neon), // V_DCT
    Some(fidentity8x32_col_neon), // H_DCT
    None, // V_ADST
    None, // H_ADST
    None, // V_FLIPADST
    None, // H_FLIPADST
];


unsafe fn lowbd_fwd_txfm2d_4x4_neon(mut input: *const i16, mut output: *mut i32, mut stride: i32, mut tx_type: TX_TYPE, mut bd: i32) {
    let mut ud_flip: i32 = core::mem::zeroed();
    let mut lr_flip: i32 = core::mem::zeroed();
    get_flip_cfg(tx_type, &raw mut ud_flip, &raw mut lr_flip);
    ud_adjust_input_and_stride(ud_flip, &raw mut input, &raw mut stride, 4);
    let mut buf0: [int16x4_t; 4] = core::mem::zeroed();
    let mut buf1: [int16x4_t; 4] = core::mem::zeroed();
    match tx_type {
        DCT_DCT => {
            fdct4x4_col_neon(input, buf0.as_mut_ptr(), stride, 13);
            transpose_arrays_s16_4x4(buf0.as_mut_ptr(), buf1.as_mut_ptr());
            fdct4x4_row_neon(buf1.as_mut_ptr(), output, 4, 13);
        }
        ADST_DCT => {
            fadst4x4_col_neon(input, buf0.as_mut_ptr(), stride, 13);
            transpose_arrays_s16_4x4(buf0.as_mut_ptr(), buf1.as_mut_ptr());
            fdct4x4_row_neon(buf1.as_mut_ptr(), output, 4, 13);
        }
        DCT_ADST => {
            fdct4x4_col_neon(input, buf0.as_mut_ptr(), stride, 13);
            transpose_arrays_s16_4x4(buf0.as_mut_ptr(), buf1.as_mut_ptr());
            fadst4x4_row_neon(buf1.as_mut_ptr(), output, 4, 13);
        }
        ADST_ADST => {
            fadst4x4_col_neon(input, buf0.as_mut_ptr(), stride, 13);
            transpose_arrays_s16_4x4(buf0.as_mut_ptr(), buf1.as_mut_ptr());
            fadst4x4_row_neon(buf1.as_mut_ptr(), output, 4, 13);
        }
        FLIPADST_DCT => {
            fadst4x4_col_neon(input, buf0.as_mut_ptr(), stride, 13);
            transpose_arrays_s16_4x4(buf0.as_mut_ptr(), buf1.as_mut_ptr());
            fdct4x4_row_neon(buf1.as_mut_ptr(), output, 4, 13);
        }
        DCT_FLIPADST => {
            fdct4x4_col_neon(input, buf0.as_mut_ptr(), stride, 13);
            transpose_arrays_s16_4x4(buf0.as_mut_ptr(), buf1.as_mut_ptr());
            flip_buf_4_neon(buf1.as_mut_ptr(), buf0.as_mut_ptr(), 4);
            fadst4x4_row_neon(buf0.as_mut_ptr(), output, 4, 13);
        }
        FLIPADST_FLIPADST => {
            fadst4x4_col_neon(input, buf0.as_mut_ptr(), stride, 13);
            transpose_arrays_s16_4x4(buf0.as_mut_ptr(), buf1.as_mut_ptr());
            flip_buf_4_neon(buf1.as_mut_ptr(), buf0.as_mut_ptr(), 4);
            fadst4x4_row_neon(buf0.as_mut_ptr(), output, 4, 13);
        }
        ADST_FLIPADST => {
            fadst4x4_col_neon(input, buf0.as_mut_ptr(), stride, 13);
            transpose_arrays_s16_4x4(buf0.as_mut_ptr(), buf1.as_mut_ptr());
            flip_buf_4_neon(buf1.as_mut_ptr(), buf0.as_mut_ptr(), 4);
            fadst4x4_row_neon(buf0.as_mut_ptr(), output, 4, 13);
        }
        FLIPADST_ADST => {
            fadst4x4_col_neon(input, buf0.as_mut_ptr(), stride, 13);
            transpose_arrays_s16_4x4(buf0.as_mut_ptr(), buf1.as_mut_ptr());
            fadst4x4_row_neon(buf1.as_mut_ptr(), output, 4, 13);
        }
        IDTX => {
            fidentity4x4_col_neon(input, buf0.as_mut_ptr(), stride, 13);
            transpose_arrays_s16_4x4(buf0.as_mut_ptr(), buf1.as_mut_ptr());
            fidentity4x4_row_neon(buf1.as_mut_ptr(), output, 4, 13);
        }
        V_DCT => {
            fdct4x4_col_neon(input, buf0.as_mut_ptr(), stride, 13);
            transpose_arrays_s16_4x4(buf0.as_mut_ptr(), buf1.as_mut_ptr());
            fidentity4x4_row_neon(buf1.as_mut_ptr(), output, 4, 13);
        }
        H_DCT => {
            fidentity4x4_col_neon(input, buf0.as_mut_ptr(), stride, 13);
            transpose_arrays_s16_4x4(buf0.as_mut_ptr(), buf1.as_mut_ptr());
            fdct4x4_row_neon(buf1.as_mut_ptr(), output, 4, 13);
        }
        V_ADST => {
            fadst4x4_col_neon(input, buf0.as_mut_ptr(), stride, 13);
            transpose_arrays_s16_4x4(buf0.as_mut_ptr(), buf1.as_mut_ptr());
            fidentity4x4_row_neon(buf1.as_mut_ptr(), output, 4, 13);
        }
        H_ADST => {
            fidentity4x4_col_neon(input, buf0.as_mut_ptr(), stride, 13);
            transpose_arrays_s16_4x4(buf0.as_mut_ptr(), buf1.as_mut_ptr());
            fadst4x4_row_neon(buf1.as_mut_ptr(), output, 4, 13);
        }
        V_FLIPADST => {
            fadst4x4_col_neon(input, buf0.as_mut_ptr(), stride, 13);
            transpose_arrays_s16_4x4(buf0.as_mut_ptr(), buf1.as_mut_ptr());
            fidentity4x4_row_neon(buf1.as_mut_ptr(), output, 4, 13);
        }
        H_FLIPADST => {
            fidentity4x4_col_neon(input, buf0.as_mut_ptr(), stride, 13);
            transpose_arrays_s16_4x4(buf0.as_mut_ptr(), buf1.as_mut_ptr());
            flip_buf_4_neon(buf1.as_mut_ptr(), buf0.as_mut_ptr(), 4);
            fadst4x4_row_neon(buf0.as_mut_ptr(), output, 4, 13);
        }
        _ => {}
    }
}

unsafe fn lowbd_fwd_txfm2d_4x8_neon(mut input: *const i16, mut output: *mut i32, mut stride: i32, mut tx_type: TX_TYPE, mut bd: i32) {
    let mut buf0: [int16x4_t; 8] = core::mem::zeroed();
    let mut buf1: [int16x8_t; 8] = core::mem::zeroed();
    let mut col_txfm: col_transform_1d_lbd_4_neon = col_txfm4x8_arr[tx_type as usize];
    let mut row_txfm: row_transform_1d_lbd_8_neon = row_rect_txfm8x4_arr[tx_type as usize];
    let mut ud_flip: i32 = core::mem::zeroed();
    let mut lr_flip: i32 = core::mem::zeroed();
    get_flip_cfg(tx_type, &raw mut ud_flip, &raw mut lr_flip);
    ud_adjust_input_and_stride(ud_flip, &raw mut input, &raw mut stride, 8);
    col_txfm.unwrap()(input, buf0.as_mut_ptr(), stride, 13);
    shift_right_1_round_s16_x4(buf0.as_mut_ptr(), buf0.as_mut_ptr(), 8);
    transpose_arrays_s16_4x8(buf0.as_mut_ptr(), buf1.as_mut_ptr());
    if lr_flip != 0 {
        let mut buf2: [int16x8_t; 8] = core::mem::zeroed();
        flip_buf_8_neon(buf1.as_mut_ptr(), buf2.as_mut_ptr(), 4);
        row_txfm.unwrap()(buf2.as_mut_ptr(), output, 8, 13);
    } else {
        row_txfm.unwrap()(buf1.as_mut_ptr(), output, 8, 13);
    }
}

unsafe fn lowbd_fwd_txfm2d_4x16_neon(mut input: *const i16, mut output: *mut i32, mut stride: i32, mut tx_type: TX_TYPE, mut bd: i32) {
    let mut buf0: [int16x4_t; 16] = core::mem::zeroed();
    let mut buf1: [int16x8_t; 16] = core::mem::zeroed();
    let mut col_txfm: col_transform_1d_lbd_4_neon = col_txfm4x16_arr[tx_type as usize];
    let mut row_txfm: row_transform_1d_lbd_8_neon = row_txfm8x4_arr[tx_type as usize];
    let mut ud_flip: i32 = core::mem::zeroed();
    let mut lr_flip: i32 = core::mem::zeroed();
    get_flip_cfg(tx_type, &raw mut ud_flip, &raw mut lr_flip);
    ud_adjust_input_and_stride(ud_flip, &raw mut input, &raw mut stride, 16);
    col_txfm.unwrap()(input, buf0.as_mut_ptr(), stride, 13);
    shift_right_1_round_s16_x4(buf0.as_mut_ptr(), buf0.as_mut_ptr(), 16);
    transpose_arrays_s16_4x8(buf0.as_mut_ptr(), buf1.as_mut_ptr());
    transpose_arrays_s16_4x8(buf0.as_mut_ptr().offset(((8)) as isize), buf1.as_mut_ptr().offset(((8)) as isize));
    for i in 0..2 {
        if lr_flip != 0 {
            let mut buf2: [int16x8_t; 16] = core::mem::zeroed();
            flip_buf_8_neon(buf1.as_mut_ptr().offset((8 * i) as isize), buf2.as_mut_ptr(), 4);
            row_txfm.unwrap()(buf2.as_mut_ptr(), output.offset((8 * i) as isize), 16, 12);
        } else {
            let mut buf: *mut int16x8_t = buf1.as_mut_ptr().offset((8 * i) as isize);
            row_txfm.unwrap()(buf, output.offset((8 * i) as isize), 16, 12);
        }
    }
}

unsafe fn lowbd_fwd_txfm2d_8x4_neon(mut input: *const i16, mut output: *mut i32, mut stride: i32, mut tx_type: TX_TYPE, mut bd: i32) {
    let mut buf0: [int16x8_t; 8] = core::mem::zeroed();
    let mut buf1: [int16x4_t; 8] = core::mem::zeroed();
    let mut col_txfm: col_transform_1d_lbd_8_neon = col_txfm8x4_arr[tx_type as usize];
    let mut row_txfm: row_transform_1d_lbd_4_neon = row_rect_txfm4x8_arr[tx_type as usize];
    let mut ud_flip: i32 = core::mem::zeroed();
    let mut lr_flip: i32 = core::mem::zeroed();
    get_flip_cfg(tx_type, &raw mut ud_flip, &raw mut lr_flip);
    ud_adjust_input_and_stride(ud_flip, &raw mut input, &raw mut stride, 4);
    col_txfm.unwrap()(input, buf0.as_mut_ptr(), stride, 13);
    shift_right_1_round_s16_x8(buf0.as_mut_ptr(), buf0.as_mut_ptr(), 4);
    transpose_arrays_s16_8x4(buf0.as_mut_ptr(), buf1.as_mut_ptr());
    if lr_flip != 0 {
        let mut buf2: [int16x4_t; 8] = core::mem::zeroed();
        flip_buf_4_neon(buf1.as_mut_ptr(), buf2.as_mut_ptr(), 8);
        row_txfm.unwrap()(buf2.as_mut_ptr(), output, 4, 13);
    } else {
        row_txfm.unwrap()(buf1.as_mut_ptr(), output, 4, 13);
    }
}

unsafe fn lowbd_fwd_txfm2d_8x8_neon(mut input: *const i16, mut output: *mut i32, mut stride: i32, mut tx_type: TX_TYPE, mut bd: i32) {
    let mut ud_flip: i32 = core::mem::zeroed();
    let mut lr_flip: i32 = core::mem::zeroed();
    get_flip_cfg(tx_type, &raw mut ud_flip, &raw mut lr_flip);
    ud_adjust_input_and_stride(ud_flip, &raw mut input, &raw mut stride, 8);
    let mut buf0: [int16x8_t; 8] = core::mem::zeroed();
    let mut buf1: [int16x8_t; 8] = core::mem::zeroed();
    match tx_type {
        DCT_DCT => {
            fdct8x8_col_neon(input, buf0.as_mut_ptr(), stride, 13);
            shift_right_1_round_s16_x8(buf0.as_mut_ptr(), buf0.as_mut_ptr(), 8);
            transpose_arrays_s16_8x8(buf0.as_mut_ptr(), buf1.as_mut_ptr());
            fdct8x8_row_neon(buf1.as_mut_ptr(), output, 8, 13);
        }
        ADST_DCT => {
            fadst8x8_col_neon(input, buf0.as_mut_ptr(), stride, 13);
            shift_right_1_round_s16_x8(buf0.as_mut_ptr(), buf0.as_mut_ptr(), 8);
            transpose_arrays_s16_8x8(buf0.as_mut_ptr(), buf1.as_mut_ptr());
            fdct8x8_row_neon(buf1.as_mut_ptr(), output, 8, 13);
        }
        DCT_ADST => {
            fdct8x8_col_neon(input, buf0.as_mut_ptr(), stride, 13);
            shift_right_1_round_s16_x8(buf0.as_mut_ptr(), buf0.as_mut_ptr(), 8);
            transpose_arrays_s16_8x8(buf0.as_mut_ptr(), buf1.as_mut_ptr());
            fadst8x8_row_neon(buf1.as_mut_ptr(), output, 8, 13);
        }
        ADST_ADST => {
            fadst8x8_col_neon(input, buf0.as_mut_ptr(), stride, 13);
            shift_right_1_round_s16_x8(buf0.as_mut_ptr(), buf0.as_mut_ptr(), 8);
            transpose_arrays_s16_8x8(buf0.as_mut_ptr(), buf1.as_mut_ptr());
            fadst8x8_row_neon(buf1.as_mut_ptr(), output, 8, 13);
        }
        FLIPADST_DCT => {
            fadst8x8_col_neon(input, buf0.as_mut_ptr(), stride, 13);
            shift_right_1_round_s16_x8(buf0.as_mut_ptr(), buf0.as_mut_ptr(), 8);
            transpose_arrays_s16_8x8(buf0.as_mut_ptr(), buf1.as_mut_ptr());
            fdct8x8_row_neon(buf1.as_mut_ptr(), output, 8, 13);
        }
        DCT_FLIPADST => {
            fdct8x8_col_neon(input, buf0.as_mut_ptr(), stride, 13);
            shift_right_1_round_s16_x8(buf0.as_mut_ptr(), buf0.as_mut_ptr(), 8);
            transpose_arrays_s16_8x8(buf0.as_mut_ptr(), buf1.as_mut_ptr());
            flip_buf_8_neon(buf1.as_mut_ptr(), buf0.as_mut_ptr(), 8);
            fadst8x8_row_neon(buf0.as_mut_ptr(), output, 8, 13);
        }
        FLIPADST_FLIPADST => {
            fadst8x8_col_neon(input, buf0.as_mut_ptr(), stride, 13);
            shift_right_1_round_s16_x8(buf0.as_mut_ptr(), buf0.as_mut_ptr(), 8);
            transpose_arrays_s16_8x8(buf0.as_mut_ptr(), buf1.as_mut_ptr());
            flip_buf_8_neon(buf1.as_mut_ptr(), buf0.as_mut_ptr(), 8);
            fadst8x8_row_neon(buf0.as_mut_ptr(), output, 8, 13);
        }
        ADST_FLIPADST => {
            fadst8x8_col_neon(input, buf0.as_mut_ptr(), stride, 13);
            shift_right_1_round_s16_x8(buf0.as_mut_ptr(), buf0.as_mut_ptr(), 8);
            transpose_arrays_s16_8x8(buf0.as_mut_ptr(), buf1.as_mut_ptr());
            flip_buf_8_neon(buf1.as_mut_ptr(), buf0.as_mut_ptr(), 8);
            fadst8x8_row_neon(buf0.as_mut_ptr(), output, 8, 13);
        }
        FLIPADST_ADST => {
            fadst8x8_col_neon(input, buf0.as_mut_ptr(), stride, 13);
            shift_right_1_round_s16_x8(buf0.as_mut_ptr(), buf0.as_mut_ptr(), 8);
            transpose_arrays_s16_8x8(buf0.as_mut_ptr(), buf1.as_mut_ptr());
            fadst8x8_row_neon(buf1.as_mut_ptr(), output, 8, 13);
        }
        IDTX => {
            fidentity8x8_col_neon(input, buf0.as_mut_ptr(), stride, 13);
            shift_right_1_round_s16_x8(buf0.as_mut_ptr(), buf0.as_mut_ptr(), 8);
            transpose_arrays_s16_8x8(buf0.as_mut_ptr(), buf1.as_mut_ptr());
            fidentity8x8_row_neon(buf1.as_mut_ptr(), output, 8, 13);
        }
        V_DCT => {
            fdct8x8_col_neon(input, buf0.as_mut_ptr(), stride, 13);
            shift_right_1_round_s16_x8(buf0.as_mut_ptr(), buf0.as_mut_ptr(), 8);
            transpose_arrays_s16_8x8(buf0.as_mut_ptr(), buf1.as_mut_ptr());
            fidentity8x8_row_neon(buf1.as_mut_ptr(), output, 8, 13);
        }
        H_DCT => {
            fidentity8x8_col_neon(input, buf0.as_mut_ptr(), stride, 13);
            shift_right_1_round_s16_x8(buf0.as_mut_ptr(), buf0.as_mut_ptr(), 8);
            transpose_arrays_s16_8x8(buf0.as_mut_ptr(), buf1.as_mut_ptr());
            fdct8x8_row_neon(buf1.as_mut_ptr(), output, 8, 13);
        }
        V_ADST => {
            fadst8x8_col_neon(input, buf0.as_mut_ptr(), stride, 13);
            shift_right_1_round_s16_x8(buf0.as_mut_ptr(), buf0.as_mut_ptr(), 8);
            transpose_arrays_s16_8x8(buf0.as_mut_ptr(), buf1.as_mut_ptr());
            fidentity8x8_row_neon(buf1.as_mut_ptr(), output, 8, 13);
        }
        H_ADST => {
            fidentity8x8_col_neon(input, buf0.as_mut_ptr(), stride, 13);
            shift_right_1_round_s16_x8(buf0.as_mut_ptr(), buf0.as_mut_ptr(), 8);
            transpose_arrays_s16_8x8(buf0.as_mut_ptr(), buf1.as_mut_ptr());
            fadst8x8_row_neon(buf1.as_mut_ptr(), output, 8, 13);
        }
        V_FLIPADST => {
            fadst8x8_col_neon(input, buf0.as_mut_ptr(), stride, 13);
            shift_right_1_round_s16_x8(buf0.as_mut_ptr(), buf0.as_mut_ptr(), 8);
            transpose_arrays_s16_8x8(buf0.as_mut_ptr(), buf1.as_mut_ptr());
            fidentity8x8_row_neon(buf1.as_mut_ptr(), output, 8, 13);
        }
        H_FLIPADST => {
            fidentity8x8_col_neon(input, buf0.as_mut_ptr(), stride, 13);
            shift_right_1_round_s16_x8(buf0.as_mut_ptr(), buf0.as_mut_ptr(), 8);
            transpose_arrays_s16_8x8(buf0.as_mut_ptr(), buf1.as_mut_ptr());
            flip_buf_8_neon(buf1.as_mut_ptr(), buf0.as_mut_ptr(), 8);
            fadst8x8_row_neon(buf0.as_mut_ptr(), output, 8, 13);
        }
        _ => {}
    }
}

unsafe fn lowbd_fwd_txfm2d_8x16_neon(mut input: *const i16, mut output: *mut i32, mut stride: i32, mut tx_type: TX_TYPE, mut bd: i32) {
    let mut buf0: [int16x8_t; 16] = core::mem::zeroed();
    let mut buf1: [int16x8_t; 16] = core::mem::zeroed();
    let mut col_txfm: col_transform_1d_lbd_8_neon = col_txfm8x16_arr[tx_type as usize];
    let mut row_txfm: row_transform_1d_lbd_8_neon = row_rect_txfm8x8_arr[tx_type as usize];
    let mut ud_flip: i32 = core::mem::zeroed();
    let mut lr_flip: i32 = core::mem::zeroed();
    get_flip_cfg(tx_type, &raw mut ud_flip, &raw mut lr_flip);
    ud_adjust_input_and_stride(ud_flip, &raw mut input, &raw mut stride, 16);
    col_txfm.unwrap()(input, buf0.as_mut_ptr(), stride, 13);
    shift_right_2_round_s16_x8(buf0.as_mut_ptr(), buf0.as_mut_ptr(), 16);
    transpose_arrays_s16_8x8(buf0.as_mut_ptr(), buf1.as_mut_ptr());
    transpose_arrays_s16_8x8(buf0.as_mut_ptr().offset(((8)) as isize), buf1.as_mut_ptr().offset(((8)) as isize));
    for i in 0..2 {
        if lr_flip != 0 {
            flip_buf_8_neon(buf1.as_mut_ptr().offset((8 * i) as isize), buf0.as_mut_ptr(), 8);
            row_txfm.unwrap()(buf0.as_mut_ptr(), output.offset((8 * i) as isize), 16, 13);
        } else {
            let mut buf: *mut int16x8_t = buf1.as_mut_ptr().offset((8 * i) as isize);
            row_txfm.unwrap()(buf, output.offset((8 * i) as isize), 16, 13);
        }
    }
}

unsafe fn lowbd_fwd_txfm2d_8x32_neon(mut input: *const i16, mut output: *mut i32, mut stride: i32, mut tx_type: TX_TYPE, mut bd: i32) {
    let mut buf0: [int16x8_t; 32] = core::mem::zeroed();
    let mut buf1: [int16x8_t; 32] = core::mem::zeroed();
    let mut col_txfm: col_transform_1d_lbd_8_neon = col_txfm8x32_arr[tx_type as usize];
    let mut row_txfm: row_transform_1d_lbd_8_neon = row_txfm8x8_arr[tx_type as usize];
    let mut ud_flip: i32 = core::mem::zeroed();
    let mut lr_flip: i32 = core::mem::zeroed();
    get_flip_cfg(tx_type, &raw mut ud_flip, &raw mut lr_flip);
    ud_adjust_input_and_stride(ud_flip, &raw mut input, &raw mut stride, 32);
    col_txfm.unwrap()(input, buf0.as_mut_ptr(), stride, 12);
    shift_right_2_round_s16_x8(buf0.as_mut_ptr(), buf0.as_mut_ptr(), 32);
    transpose_arrays_s16_8x8(buf0.as_mut_ptr(), buf1.as_mut_ptr());
    transpose_arrays_s16_8x8(buf0.as_mut_ptr().offset(((8)) as isize), buf1.as_mut_ptr().offset(((8)) as isize));
    transpose_arrays_s16_8x8(buf0.as_mut_ptr().offset(((16)) as isize), buf1.as_mut_ptr().offset(((16)) as isize));
    transpose_arrays_s16_8x8(buf0.as_mut_ptr().offset(((24)) as isize), buf1.as_mut_ptr().offset(((24)) as isize));
    for i in 0..4 {
        if lr_flip != 0 {
            flip_buf_8_neon(buf1.as_mut_ptr().offset((8 * i) as isize), buf0.as_mut_ptr(), 8);
            row_txfm.unwrap()(buf0.as_mut_ptr(), output.offset((8 * i) as isize), 32, 12);
        } else {
            let mut buf: *mut int16x8_t = buf1.as_mut_ptr().offset((8 * i) as isize);
            row_txfm.unwrap()(buf, output.offset((8 * i) as isize), 32, 12);
        }
    }
}

unsafe fn lowbd_fwd_txfm2d_16x4_neon(mut input: *const i16, mut output: *mut i32, mut stride: i32, mut tx_type: TX_TYPE, mut bd: i32) {
    let mut buf0: [int16x8_t; 16] = core::mem::zeroed();
    let mut buf1: [int16x4_t; 16] = core::mem::zeroed();
    let mut buf2: [int16x4_t; 16] = core::mem::zeroed();
    let mut col_txfm: col_transform_1d_lbd_8_neon = col_txfm8x4_arr[tx_type as usize];
    let mut row_txfm: row_transform_1d_lbd_4_neon = row_txfm4x16_arr[tx_type as usize];
    let mut ud_flip: i32 = core::mem::zeroed();
    let mut lr_flip: i32 = core::mem::zeroed();
    get_flip_cfg(tx_type, &raw mut ud_flip, &raw mut lr_flip);
    ud_adjust_input_and_stride(ud_flip, &raw mut input, &raw mut stride, 4);
    for i in 0..2 {
        col_txfm.unwrap()(input.offset((8 * i) as isize), buf0.as_mut_ptr(), stride, 13);
        shift_right_1_round_s16_x8(buf0.as_mut_ptr(), buf0.as_mut_ptr(), 4);
        transpose_arrays_s16_8x4(buf0.as_mut_ptr(), buf1.as_mut_ptr().offset((8 * i) as isize));
    }
    if lr_flip != 0 {
        flip_buf_4_neon(buf1.as_mut_ptr(), buf2.as_mut_ptr(), 16);
        row_txfm.unwrap()(buf2.as_mut_ptr(), output, 4, 13);
    } else {
        row_txfm.unwrap()(buf1.as_mut_ptr(), output, 4, 13);
    }
}

unsafe fn lowbd_fwd_txfm2d_16x8_neon(mut input: *const i16, mut output: *mut i32, mut stride: i32, mut tx_type: TX_TYPE, mut bd: i32) {
    let mut buf0: [int16x8_t; 16] = core::mem::zeroed();
    let mut buf1: [int16x8_t; 16] = core::mem::zeroed();
    let mut col_txfm: col_transform_1d_lbd_8_neon = col_txfm8x8_arr[tx_type as usize];
    let mut row_txfm: row_transform_1d_lbd_8_neon = row_rect_txfm8x16_arr[tx_type as usize];
    let mut ud_flip: i32 = core::mem::zeroed();
    let mut lr_flip: i32 = core::mem::zeroed();
    get_flip_cfg(tx_type, &raw mut ud_flip, &raw mut lr_flip);
    ud_adjust_input_and_stride(ud_flip, &raw mut input, &raw mut stride, 8);
    for i in 0..2 {
        col_txfm.unwrap()(input.offset((8 * i) as isize), buf0.as_mut_ptr(), stride, 13);
        shift_right_2_round_s16_x8(buf0.as_mut_ptr(), buf0.as_mut_ptr(), 8);
        transpose_arrays_s16_8x8(buf0.as_mut_ptr(), buf1.as_mut_ptr().offset((8 * i) as isize));
    }
    if lr_flip != 0 {
        flip_buf_8_neon(buf1.as_mut_ptr(), buf0.as_mut_ptr(), 16);
        row_txfm.unwrap()(buf0.as_mut_ptr(), output, 8, 13);
    } else {
        row_txfm.unwrap()(buf1.as_mut_ptr(), output, 8, 13);
    }
}

unsafe fn lowbd_fwd_txfm2d_16x16_neon(mut input: *const i16, mut output: *mut i32, mut stride: i32, mut tx_type: TX_TYPE, mut bd: i32) {
    let mut buf0: [int16x8_t; 16] = core::mem::zeroed();
    let mut buf1: [int16x8_t; 32] = core::mem::zeroed();
    let mut col_txfm: col_transform_1d_lbd_8_neon = col_txfm8x16_arr[tx_type as usize];
    let mut row_txfm: row_transform_1d_lbd_8_neon = row_txfm8x16_arr[tx_type as usize];
    let mut ud_flip: i32 = core::mem::zeroed();
    let mut lr_flip: i32 = core::mem::zeroed();
    get_flip_cfg(tx_type, &raw mut ud_flip, &raw mut lr_flip);
    ud_adjust_input_and_stride(ud_flip, &raw mut input, &raw mut stride, 16);
    for i in 0..2 {
        col_txfm.unwrap()(input.offset((8 * i) as isize), buf0.as_mut_ptr(), stride, 13);
        shift_right_2_round_s16_x8(buf0.as_mut_ptr(), buf0.as_mut_ptr(), 16);
        transpose_arrays_s16_8x8(buf0.as_mut_ptr(), buf1.as_mut_ptr().offset(((0 * 16 + 8 * i)) as isize));
        transpose_arrays_s16_8x8(buf0.as_mut_ptr().offset(((8)) as isize), buf1.as_mut_ptr().offset(((1 * 16 + 8 * i)) as isize));
    }
    for i in 0..2 {
        if lr_flip != 0 {
            flip_buf_8_neon(buf1.as_mut_ptr().offset((16 * i) as isize), buf0.as_mut_ptr(), 16);
            row_txfm.unwrap()(buf0.as_mut_ptr(), output.offset((8 * i) as isize), 16, 12);
        } else {
            let mut buf: *mut int16x8_t = buf1.as_mut_ptr().offset((16 * i) as isize);
            row_txfm.unwrap()(buf, output.offset((8 * i) as isize), 16, 12);
        }
    }
}

unsafe fn lowbd_fwd_txfm2d_16x32_neon(mut input: *const i16, mut output: *mut i32, mut stride: i32, mut tx_type: TX_TYPE, mut bd: i32) {
    let mut buf0: [int16x8_t; 32] = core::mem::zeroed();
    let mut buf1: [int16x8_t; 64] = core::mem::zeroed();
    let mut col_txfm: col_transform_1d_lbd_8_neon = col_txfm8x32_arr[tx_type as usize];
    let mut row_txfm: row_transform_1d_lbd_8_neon = row_rect_txfm8x16_arr[tx_type as usize];
    if col_txfm.is_none() || row_txfm.is_none() {
        av1_fwd_txfm2d_16x32_c(input, output, stride, tx_type, bd);
        return;
    }
    let mut ud_flip: i32 = core::mem::zeroed();
    let mut lr_flip: i32 = core::mem::zeroed();
    get_flip_cfg(tx_type, &raw mut ud_flip, &raw mut lr_flip);
    ud_adjust_input_and_stride(ud_flip, &raw mut input, &raw mut stride, 32);
    for i in 0..2 {
        col_txfm.unwrap()(input.offset((8 * i) as isize), buf0.as_mut_ptr(), stride, 12);
        shift_right_4_round_s16_x8(buf0.as_mut_ptr(), buf0.as_mut_ptr(), 32);
        transpose_arrays_s16_8x8(buf0.as_mut_ptr().offset((0 * 8) as isize), buf1.as_mut_ptr().offset(((0 * 16 + 8 * i)) as isize));
        transpose_arrays_s16_8x8(buf0.as_mut_ptr().offset((1 * 8) as isize), buf1.as_mut_ptr().offset(((1 * 16 + 8 * i)) as isize));
        transpose_arrays_s16_8x8(buf0.as_mut_ptr().offset((2 * 8) as isize), buf1.as_mut_ptr().offset(((2 * 16 + 8 * i)) as isize));
        transpose_arrays_s16_8x8(buf0.as_mut_ptr().offset((3 * 8) as isize), buf1.as_mut_ptr().offset(((3 * 16 + 8 * i)) as isize));
    }
    for i in 0..4 {
        if lr_flip != 0 {
            flip_buf_8_neon(buf1.as_mut_ptr().offset((16 * i) as isize), buf0.as_mut_ptr(), 16);
            row_txfm.unwrap()(buf0.as_mut_ptr(), output.offset((8 * i) as isize), 32, 13);
        } else {
            let mut buf: *mut int16x8_t = buf1.as_mut_ptr().offset((16 * i) as isize);
            row_txfm.unwrap()(buf, output.offset((8 * i) as isize), 32, 13);
        }
    }
}

unsafe fn lowbd_fwd_txfm2d_32x8_neon(mut input: *const i16, mut output: *mut i32, mut stride: i32, mut tx_type: TX_TYPE, mut bd: i32) {
    let mut buf0: [int16x8_t; 32] = core::mem::zeroed();
    let mut buf1: [int16x8_t; 32] = core::mem::zeroed();
    let mut col_txfm: col_transform_1d_lbd_8_neon = col_txfm8x8_arr[tx_type as usize];
    let mut row_txfm: row_transform_1d_lbd_8_neon = row_txfm8x32_arr[tx_type as usize];
    if col_txfm.is_none() || row_txfm.is_none() {
        av1_fwd_txfm2d_32x16_c(input, output, stride, tx_type, bd);
        return;
    }
    let mut ud_flip: i32 = core::mem::zeroed();
    let mut lr_flip: i32 = core::mem::zeroed();
    get_flip_cfg(tx_type, &raw mut ud_flip, &raw mut lr_flip);
    ud_adjust_input_and_stride(ud_flip, &raw mut input, &raw mut stride, 8);
    for i in 0..4 {
        col_txfm.unwrap()(input.offset((8 * i) as isize), buf0.as_mut_ptr(), stride, 13);
        shift_right_2_round_s16_x8(buf0.as_mut_ptr(), buf0.as_mut_ptr(), 8);
        transpose_arrays_s16_8x8(buf0.as_mut_ptr(), buf1.as_mut_ptr().offset(((0 * 32 + 8 * i)) as isize));
    }
    if lr_flip != 0 {
        flip_buf_8_neon(buf1.as_mut_ptr(), buf0.as_mut_ptr(), 32);
        row_txfm.unwrap()(buf0.as_mut_ptr(), output, 8, 12);
    } else {
        row_txfm.unwrap()(buf1.as_mut_ptr(), output, 8, 12);
    }
}

unsafe fn lowbd_fwd_txfm2d_32x16_neon(mut input: *const i16, mut output: *mut i32, mut stride: i32, mut tx_type: TX_TYPE, mut bd: i32) {
    let mut buf0: [int16x8_t; 32] = core::mem::zeroed();
    let mut buf1: [int16x8_t; 64] = core::mem::zeroed();
    let mut col_txfm: col_transform_1d_lbd_8_neon = col_txfm8x16_arr[tx_type as usize];
    let mut row_txfm: row_transform_1d_lbd_8_neon = row_rect_txfm8x32_arr[tx_type as usize];
    if col_txfm.is_none() || row_txfm.is_none() {
        av1_fwd_txfm2d_32x16_c(input, output, stride, tx_type, bd);
        return;
    }
    let mut ud_flip: i32 = core::mem::zeroed();
    let mut lr_flip: i32 = core::mem::zeroed();
    get_flip_cfg(tx_type, &raw mut ud_flip, &raw mut lr_flip);
    ud_adjust_input_and_stride(ud_flip, &raw mut input, &raw mut stride, 16);
    for i in 0..4 {
        col_txfm.unwrap()(input.offset((8 * i) as isize), buf0.as_mut_ptr(), stride, 13);
        shift_right_4_round_s16_x8(buf0.as_mut_ptr(), buf0.as_mut_ptr(), 16);
        transpose_arrays_s16_8x8(buf0.as_mut_ptr(), buf1.as_mut_ptr().offset(((0 * 32 + 8 * i)) as isize));
        transpose_arrays_s16_8x8(buf0.as_mut_ptr().offset(((8)) as isize), buf1.as_mut_ptr().offset(((1 * 32 + 8 * i)) as isize));
    }
    for i in 0..2 {
        if lr_flip != 0 {
            flip_buf_8_neon(buf1.as_mut_ptr().offset((32 * i) as isize), buf0.as_mut_ptr(), 32);
            row_txfm.unwrap()(buf0.as_mut_ptr(), output.offset((8 * i) as isize), 16, 13);
        } else {
            let mut buf: *mut int16x8_t = buf1.as_mut_ptr().offset((32 * i) as isize);
            row_txfm.unwrap()(buf, output.offset((8 * i) as isize), 16, 13);
        }
    }
}

unsafe fn lowbd_fwd_txfm2d_32x32_neon(mut input: *const i16, mut output: *mut i32, mut stride: i32, mut tx_type: TX_TYPE, mut bd: i32) {
    let mut buf0: [int16x8_t; 32] = core::mem::zeroed();
    let mut buf1: [int16x8_t; 128] = core::mem::zeroed();
    let mut col_txfm: col_transform_1d_lbd_8_neon = col_txfm8x32_arr[tx_type as usize];
    let mut row_txfm: row_transform_1d_lbd_8_neon = row_txfm8x32_arr[tx_type as usize];
    if col_txfm.is_none() || row_txfm.is_none() {
        av1_fwd_txfm2d_32x32_c(input, output, stride, tx_type, bd);
        return;
    }
    let mut ud_flip: i32 = core::mem::zeroed();
    let mut lr_flip: i32 = core::mem::zeroed();
    get_flip_cfg(tx_type, &raw mut ud_flip, &raw mut lr_flip);
    ud_adjust_input_and_stride(ud_flip, &raw mut input, &raw mut stride, 32);
    for i in 0..4 {
        col_txfm.unwrap()(input.offset((8 * i) as isize), buf0.as_mut_ptr(), stride, 12);
        shift_right_4_round_s16_x8(buf0.as_mut_ptr(), buf0.as_mut_ptr(), 32);
        transpose_arrays_s16_8x8(buf0.as_mut_ptr().offset((0 * 8) as isize), buf1.as_mut_ptr().offset(((0 * 32 + 8 * i)) as isize));
        transpose_arrays_s16_8x8(buf0.as_mut_ptr().offset((1 * 8) as isize), buf1.as_mut_ptr().offset(((1 * 32 + 8 * i)) as isize));
        transpose_arrays_s16_8x8(buf0.as_mut_ptr().offset((2 * 8) as isize), buf1.as_mut_ptr().offset(((2 * 32 + 8 * i)) as isize));
        transpose_arrays_s16_8x8(buf0.as_mut_ptr().offset((3 * 8) as isize), buf1.as_mut_ptr().offset(((3 * 32 + 8 * i)) as isize));
    }
    for i in 0..4 {
        if lr_flip != 0 {
            flip_buf_8_neon(buf1.as_mut_ptr().offset((32 * i) as isize), buf0.as_mut_ptr(), 32);
            row_txfm.unwrap()(buf0.as_mut_ptr(), output.offset((8 * i) as isize), 32, 12);
        } else {
            let mut buf: *mut int16x8_t = buf1.as_mut_ptr().offset((32 * i) as isize);
            row_txfm.unwrap()(buf, output.offset((8 * i) as isize), 32, 12);
        }
    }
}

unsafe fn lowbd_fwd_txfm2d_64x16_neon(mut input: *const i16, mut output: *mut i32, mut stride: i32, mut tx_type: TX_TYPE, mut bd: i32) {
    debug_assert!(tx_type == DCT_DCT);
    let mut buf0: [int16x8_t; 64] = core::mem::zeroed();
    let mut buf1: [int16x8_t; 128] = core::mem::zeroed();
    let mut col_txfm: transform_1d_lbd_8_neon = Some(fdct8x16_neon);
    let mut row_txfm: transform_1d_lbd_8_neon = Some(fdct8x64_neon);
    for i in 0..8 {
        load_buffer_s16_x8(input.offset((8 * i) as isize), stride, buf0.as_mut_ptr(), 16);
        shift_left_2_s16_x8(buf0.as_mut_ptr(), buf0.as_mut_ptr(), 16);
        col_txfm.unwrap()(buf0.as_mut_ptr(), buf0.as_mut_ptr(), 13);
        shift_right_4_round_s16_x8(buf0.as_mut_ptr(), buf0.as_mut_ptr(), 16);
        for j in 0..2 {
            transpose_arrays_s16_8x8(buf0.as_mut_ptr().offset((j * 8) as isize), buf1.as_mut_ptr().offset(((j * 64 + 8 * i)) as isize));
        }
    }
    for i in 0..2 {
        let mut buf: *mut int16x8_t = buf1.as_mut_ptr().offset((64 * i) as isize);
        row_txfm.unwrap()(buf, buf, 12);
        store_buffer_s16_x8(buf, output.offset((8 * i) as isize), 16, 32);
    }
    // Zero out the bottom 16x32 area.
    core::ptr::write_bytes(output.offset((16 * 32) as isize), 0, 16 * 32);
}

unsafe fn lowbd_fwd_txfm2d_16x64_neon(mut input: *const i16, mut output: *mut i32, mut stride: i32, mut tx_type: TX_TYPE, mut bd: i32) {
    debug_assert!(tx_type == DCT_DCT);
    let mut buf0: [int16x8_t; 64] = core::mem::zeroed();
    let mut buf1: [int16x8_t; 128] = core::mem::zeroed();
    let mut col_txfm: transform_1d_lbd_8_neon = Some(fdct8x64_neon);
    let mut row_txfm: transform_1d_lbd_8_neon = Some(fdct8x16_neon);
    for i in 0..2 {
        load_buffer_s16_x8(input.offset((8 * i) as isize), stride, buf0.as_mut_ptr(), 64);
        col_txfm.unwrap()(buf0.as_mut_ptr(), buf0.as_mut_ptr(), 13);
        shift_right_2_round_s16_x8(buf0.as_mut_ptr(), buf0.as_mut_ptr(), 64);
        for j in 0..8 {
            transpose_arrays_s16_8x8(buf0.as_mut_ptr().offset((j * 8) as isize), buf1.as_mut_ptr().offset(((j * 16 + 8 * i)) as isize));
        }
    }
    for i in 0..4 {
        let mut buf: *mut int16x8_t = buf1.as_mut_ptr().offset((16 * i) as isize);
        row_txfm.unwrap()(buf, buf, 12);
        store_buffer_s16_x8(buf, output.offset((8 * i) as isize), 32, 16);
    }
}

unsafe fn fdct32_neon(mut input: *const int32x4_t, mut output: *mut int32x4_t, mut cos_bit: i32) {
    let mut cospi: *const i16 = cospi_arr_q13(cos_bit);
    let mut cospi32_16: int16x8_t = vld1q_s16(cospi.offset((4 * 0) as isize));
    let mut cospi8_24: int16x8_t = vld1q_s16(cospi.offset((4 * 2) as isize));
    let mut cospi4_12: int16x8_t = vld1q_s16(cospi.offset((4 * 4) as isize));
    let mut cospi20_28: int16x8_t = vld1q_s16(cospi.offset((4 * 6) as isize));
    let mut cospi2_6: int16x8_t = vld1q_s16(cospi.offset((4 * 8) as isize));
    let mut cospi10_14: int16x8_t = vld1q_s16(cospi.offset((4 * 10) as isize));
    let mut cospi18_22: int16x8_t = vld1q_s16(cospi.offset((4 * 12) as isize));
    let mut cospi26_30: int16x8_t = vld1q_s16(cospi.offset((4 * 14) as isize));
    let mut cospi32: int16x4_t = vget_low_s16(cospi32_16);
    let mut cospi16: int16x4_t = vget_high_s16(cospi32_16);
    let mut cospi8: int16x4_t = vget_low_s16(cospi8_24);
    let mut cospi24: int16x4_t = vget_high_s16(cospi8_24);
    let mut cospi4: int16x4_t = vget_low_s16(cospi4_12);
    let mut cospi12: int16x4_t = vget_high_s16(cospi4_12);
    let mut cospi20: int16x4_t = vget_low_s16(cospi20_28);
    let mut cospi28: int16x4_t = vget_high_s16(cospi20_28);
    let mut cospi2: int16x4_t = vget_low_s16(cospi2_6);
    let mut cospi6: int16x4_t = vget_high_s16(cospi2_6);
    let mut cospi10: int16x4_t = vget_low_s16(cospi10_14);
    let mut cospi14: int16x4_t = vget_high_s16(cospi10_14);
    let mut cospi18: int16x4_t = vget_low_s16(cospi18_22);
    let mut cospi22: int16x4_t = vget_high_s16(cospi18_22);
    let mut cospi26: int16x4_t = vget_low_s16(cospi26_30);
    let mut cospi30: int16x4_t = vget_high_s16(cospi26_30);
    let mut buf0: [int32x4_t; 32] = core::mem::zeroed();
    let mut buf1: [int32x4_t; 32] = core::mem::zeroed();
    // stage 1
    butterfly_dct_pre_s32_x4(input, buf1.as_mut_ptr(), 32);
    // stage 2
    butterfly_dct_pre_s32_x4(buf1.as_mut_ptr(), buf0.as_mut_ptr(), 16);
    buf0[16] = buf1[16];
    buf0[17] = buf1[17];
    buf0[18] = buf1[18];
    buf0[19] = buf1[19];
    butterfly_s32_s32_x4_0112_neon(cospi32, buf1[27], buf1[20], &raw mut buf0[27], &raw mut buf0[20]);
    butterfly_s32_s32_x4_0112_neon(cospi32, buf1[26], buf1[21], &raw mut buf0[26], &raw mut buf0[21]);
    butterfly_s32_s32_x4_0112_neon(cospi32, buf1[25], buf1[22], &raw mut buf0[25], &raw mut buf0[22]);
    butterfly_s32_s32_x4_0112_neon(cospi32, buf1[24], buf1[23], &raw mut buf0[24], &raw mut buf0[23]);
    buf0[28] = buf1[28];
    buf0[29] = buf1[29];
    buf0[30] = buf1[30];
    buf0[31] = buf1[31];
    // stage 3
    butterfly_dct_pre_s32_x4(buf0.as_mut_ptr(), buf1.as_mut_ptr(), 8);
    buf1[8] = buf0[8];
    buf1[9] = buf0[9];
    butterfly_s32_s32_x4_0112_neon(cospi32, buf0[13], buf0[10], &raw mut buf1[13], &raw mut buf1[10]);
    butterfly_s32_s32_x4_0112_neon(cospi32, buf0[12], buf0[11], &raw mut buf1[12], &raw mut buf1[11]);
    buf1[14] = buf0[14];
    buf1[15] = buf0[15];
    butterfly_dct_post_s32_x4(buf0.as_mut_ptr().offset(((16)) as isize), buf0.as_mut_ptr().offset(((16)) as isize), buf1.as_mut_ptr().offset(((16)) as isize), 16);
    // stage 4
    butterfly_dct_pre_s32_x4(buf1.as_mut_ptr(), buf0.as_mut_ptr(), 4);
    buf0[4] = buf1[4];
    butterfly_s32_s32_x4_0112_neon(cospi32, buf1[6], buf1[5], &raw mut buf0[6], &raw mut buf0[5]);
    buf0[7] = buf1[7];
    butterfly_dct_post_s32_x4(buf1.as_mut_ptr().offset(((8)) as isize), buf1.as_mut_ptr().offset(((8)) as isize), buf0.as_mut_ptr().offset(((8)) as isize), 8);
    buf0[16] = buf1[16];
    buf0[17] = buf1[17];
    butterfly_s32_s32_x4_0112_neon(cospi16, buf1[29], buf1[18], &raw mut buf0[29], &raw mut buf0[18]);
    butterfly_s32_s32_x4_0112_neon(cospi16, buf1[28], buf1[19], &raw mut buf0[28], &raw mut buf0[19]);
    butterfly_s32_s32_x4_1223_neon(cospi16, buf1[27], buf1[20], &raw mut buf0[27], &raw mut buf0[20]);
    butterfly_s32_s32_x4_1223_neon(cospi16, buf1[26], buf1[21], &raw mut buf0[26], &raw mut buf0[21]);
    buf0[22] = buf1[22];
    buf0[23] = buf1[23];
    buf0[24] = buf1[24];
    buf0[25] = buf1[25];
    buf0[30] = buf1[30];
    buf0[31] = buf1[31];
    // stage 5
    butterfly_s32_s32_x4_0112_neon(cospi32, buf0[0], buf0[1], &raw mut buf1[0], &raw mut buf1[1]);
    butterfly_s32_s32_x4_0112_neon(cospi16, buf0[3], buf0[2], &raw mut buf1[2], &raw mut buf1[3]);
    butterfly_dct_post_s32_x4(buf0.as_mut_ptr().offset(((4)) as isize), buf0.as_mut_ptr().offset(((4)) as isize), buf1.as_mut_ptr().offset(((4)) as isize), 4);
    buf1[8] = buf0[8];
    butterfly_s32_s32_x4_0112_neon(cospi16, buf0[14], buf0[9], &raw mut buf1[14], &raw mut buf1[9]);
    butterfly_s32_s32_x4_1223_neon(cospi16, buf0[13], buf0[10], &raw mut buf1[13], &raw mut buf1[10]);
    buf1[11] = buf0[11];
    buf1[12] = buf0[12];
    buf1[15] = buf0[15];
    butterfly_dct_post_s32_x4(buf0.as_mut_ptr().offset(((16)) as isize), buf0.as_mut_ptr().offset(((16)) as isize), buf1.as_mut_ptr().offset(((16)) as isize), 8);
    butterfly_dct_post_s32_x4(buf0.as_mut_ptr().offset(((24)) as isize), buf0.as_mut_ptr().offset(((24)) as isize), buf1.as_mut_ptr().offset(((24)) as isize), 8);
    // stage 6
    buf0[0] = buf1[0];
    buf0[1] = buf1[1];
    buf0[2] = buf1[2];
    buf0[3] = buf1[3];
    butterfly_s32_s32_x4_0112_neon(cospi8, buf1[7], buf1[4], &raw mut buf0[4], &raw mut buf0[7]);
    butterfly_s32_s32_x4_1003_neon(cospi24, buf1[6], buf1[5], &raw mut buf0[5], &raw mut buf0[6]);
    butterfly_dct_post_s32_x4(buf1.as_mut_ptr().offset(((8)) as isize), buf1.as_mut_ptr().offset(((8)) as isize), buf0.as_mut_ptr().offset(((8)) as isize), 4);
    butterfly_dct_post_s32_x4(buf1.as_mut_ptr().offset(((12)) as isize), buf1.as_mut_ptr().offset(((12)) as isize), buf0.as_mut_ptr().offset(((12)) as isize), 4);
    buf0[16] = buf1[16];
    butterfly_s32_s32_x4_0112_neon(cospi8, buf1[30], buf1[17], &raw mut buf0[30], &raw mut buf0[17]);
    butterfly_s32_s32_x4_1223_neon(cospi8, buf1[29], buf1[18], &raw mut buf0[29], &raw mut buf0[18]);
    buf0[19] = buf1[19];
    buf0[20] = buf1[20];
    butterfly_s32_s32_x4_1003_neon(cospi24, buf1[26], buf1[21], &raw mut buf0[26], &raw mut buf0[21]);
    butterfly_s32_s32_x4_0332_neon(cospi24, buf1[25], buf1[22], &raw mut buf0[25], &raw mut buf0[22]);
    buf0[23] = buf1[23];
    buf0[24] = buf1[24];
    buf0[27] = buf1[27];
    buf0[28] = buf1[28];
    buf0[31] = buf1[31];
    // stage 7
    buf1[0] = buf0[0];
    buf1[1] = buf0[1];
    buf1[2] = buf0[2];
    buf1[3] = buf0[3];
    buf1[4] = buf0[4];
    buf1[5] = buf0[5];
    buf1[6] = buf0[6];
    buf1[7] = buf0[7];
    butterfly_s32_s32_x4_0112_neon(cospi4, buf0[15], buf0[8], &raw mut buf1[8], &raw mut buf1[15]);
    butterfly_s32_s32_x4_1003_neon(cospi28, buf0[14], buf0[9], &raw mut buf1[9], &raw mut buf1[14]);
    butterfly_s32_s32_x4_0112_neon(cospi20, buf0[13], buf0[10], &raw mut buf1[10], &raw mut buf1[13]);
    butterfly_s32_s32_x4_1003_neon(cospi12, buf0[12], buf0[11], &raw mut buf1[11], &raw mut buf1[12]);
    butterfly_dct_post_s32_x4(buf0.as_mut_ptr().offset(((16)) as isize), buf0.as_mut_ptr().offset(((16)) as isize), buf1.as_mut_ptr().offset(((16)) as isize), 4);
    butterfly_dct_post_s32_x4(buf0.as_mut_ptr().offset(((20)) as isize), buf0.as_mut_ptr().offset(((20)) as isize), buf1.as_mut_ptr().offset(((20)) as isize), 4);
    butterfly_dct_post_s32_x4(buf0.as_mut_ptr().offset(((24)) as isize), buf0.as_mut_ptr().offset(((24)) as isize), buf1.as_mut_ptr().offset(((24)) as isize), 4);
    butterfly_dct_post_s32_x4(buf0.as_mut_ptr().offset(((28)) as isize), buf0.as_mut_ptr().offset(((28)) as isize), buf1.as_mut_ptr().offset(((28)) as isize), 4);
    // stage 8
    buf0[0] = buf1[0];
    buf0[1] = buf1[1];
    buf0[2] = buf1[2];
    buf0[3] = buf1[3];
    buf0[4] = buf1[4];
    buf0[5] = buf1[5];
    buf0[6] = buf1[6];
    buf0[7] = buf1[7];
    buf0[8] = buf1[8];
    buf0[9] = buf1[9];
    buf0[10] = buf1[10];
    buf0[11] = buf1[11];
    buf0[12] = buf1[12];
    buf0[13] = buf1[13];
    buf0[14] = buf1[14];
    buf0[15] = buf1[15];
    butterfly_s32_s32_x4_0112_neon(cospi2, buf1[31], buf1[16], &raw mut buf0[16], &raw mut buf0[31]);
    butterfly_s32_s32_x4_1003_neon(cospi30, buf1[30], buf1[17], &raw mut buf0[17], &raw mut buf0[30]);
    butterfly_s32_s32_x4_0112_neon(cospi18, buf1[29], buf1[18], &raw mut buf0[18], &raw mut buf0[29]);
    butterfly_s32_s32_x4_1003_neon(cospi14, buf1[28], buf1[19], &raw mut buf0[19], &raw mut buf0[28]);
    butterfly_s32_s32_x4_0112_neon(cospi10, buf1[27], buf1[20], &raw mut buf0[20], &raw mut buf0[27]);
    butterfly_s32_s32_x4_1003_neon(cospi22, buf1[26], buf1[21], &raw mut buf0[21], &raw mut buf0[26]);
    butterfly_s32_s32_x4_0112_neon(cospi26, buf1[25], buf1[22], &raw mut buf0[22], &raw mut buf0[25]);
    butterfly_s32_s32_x4_1003_neon(cospi6, buf1[24], buf1[23], &raw mut buf0[23], &raw mut buf0[24]);
    // stage 9
    *output.add(0) = buf0[0];
    *output.add(1) = buf0[16];
    *output.add(2) = buf0[8];
    *output.add(3) = buf0[24];
    *output.add(4) = buf0[4];
    *output.add(5) = buf0[20];
    *output.add(6) = buf0[12];
    *output.add(7) = buf0[28];
    *output.add(8) = buf0[2];
    *output.add(9) = buf0[18];
    *output.add(10) = buf0[10];
    *output.add(11) = buf0[26];
    *output.add(12) = buf0[6];
    *output.add(13) = buf0[22];
    *output.add(14) = buf0[14];
    *output.add(15) = buf0[30];
    *output.add(16) = buf0[1];
    *output.add(17) = buf0[17];
    *output.add(18) = buf0[9];
    *output.add(19) = buf0[25];
    *output.add(20) = buf0[5];
    *output.add(21) = buf0[21];
    *output.add(22) = buf0[13];
    *output.add(23) = buf0[29];
    *output.add(24) = buf0[3];
    *output.add(25) = buf0[19];
    *output.add(26) = buf0[11];
    *output.add(27) = buf0[27];
    *output.add(28) = buf0[7];
    *output.add(29) = buf0[23];
    *output.add(30) = buf0[15];
    *output.add(31) = buf0[31];
}

unsafe fn fdct64_neon(mut input: *const int32x4_t, mut output: *mut int32x4_t, mut cos_bit: i32) {
    let mut cospi: *const i16 = cospi_arr_q13(cos_bit);
    let mut cospi32_16: int16x8_t = vld1q_s16(cospi.offset((4 * 0) as isize));
    let mut cospi8_24: int16x8_t = vld1q_s16(cospi.offset((4 * 2) as isize));
    let mut cospi4_12: int16x8_t = vld1q_s16(cospi.offset((4 * 4) as isize));
    let mut cospi20_28: int16x8_t = vld1q_s16(cospi.offset((4 * 6) as isize));
    let mut cospi2_6: int16x8_t = vld1q_s16(cospi.offset((4 * 8) as isize));
    let mut cospi10_14: int16x8_t = vld1q_s16(cospi.offset((4 * 10) as isize));
    let mut cospi18_22: int16x8_t = vld1q_s16(cospi.offset((4 * 12) as isize));
    let mut cospi26_30: int16x8_t = vld1q_s16(cospi.offset((4 * 14) as isize));
    let mut cospi1_3: int16x8_t = vld1q_s16(cospi.offset((4 * 16) as isize));
    let mut cospi5_7: int16x8_t = vld1q_s16(cospi.offset((4 * 18) as isize));
    let mut cospi9_11: int16x8_t = vld1q_s16(cospi.offset((4 * 20) as isize));
    let mut cospi13_15: int16x8_t = vld1q_s16(cospi.offset((4 * 22) as isize));
    let mut cospi17_19: int16x8_t = vld1q_s16(cospi.offset((4 * 24) as isize));
    let mut cospi21_23: int16x8_t = vld1q_s16(cospi.offset((4 * 26) as isize));
    let mut cospi25_27: int16x8_t = vld1q_s16(cospi.offset((4 * 28) as isize));
    let mut cospi29_31: int16x8_t = vld1q_s16(cospi.offset((4 * 30) as isize));
    let mut cospi32: int16x4_t = vget_low_s16(cospi32_16);
    let mut cospi16: int16x4_t = vget_high_s16(cospi32_16);
    let mut cospi8: int16x4_t = vget_low_s16(cospi8_24);
    let mut cospi24: int16x4_t = vget_high_s16(cospi8_24);
    let mut cospi4: int16x4_t = vget_low_s16(cospi4_12);
    let mut cospi12: int16x4_t = vget_high_s16(cospi4_12);
    let mut cospi20: int16x4_t = vget_low_s16(cospi20_28);
    let mut cospi28: int16x4_t = vget_high_s16(cospi20_28);
    let mut cospi2: int16x4_t = vget_low_s16(cospi2_6);
    let mut cospi6: int16x4_t = vget_high_s16(cospi2_6);
    let mut cospi10: int16x4_t = vget_low_s16(cospi10_14);
    let mut cospi14: int16x4_t = vget_high_s16(cospi10_14);
    let mut cospi18: int16x4_t = vget_low_s16(cospi18_22);
    let mut cospi22: int16x4_t = vget_high_s16(cospi18_22);
    let mut cospi26: int16x4_t = vget_low_s16(cospi26_30);
    let mut cospi30: int16x4_t = vget_high_s16(cospi26_30);
    let mut cospi1: int16x4_t = vget_low_s16(cospi1_3);
    let mut cospi3: int16x4_t = vget_high_s16(cospi1_3);
    let mut cospi5: int16x4_t = vget_low_s16(cospi5_7);
    let mut cospi7: int16x4_t = vget_high_s16(cospi5_7);
    let mut cospi9: int16x4_t = vget_low_s16(cospi9_11);
    let mut cospi11: int16x4_t = vget_high_s16(cospi9_11);
    let mut cospi13: int16x4_t = vget_low_s16(cospi13_15);
    let mut cospi15: int16x4_t = vget_high_s16(cospi13_15);
    let mut cospi17: int16x4_t = vget_low_s16(cospi17_19);
    let mut cospi19: int16x4_t = vget_high_s16(cospi17_19);
    let mut cospi21: int16x4_t = vget_low_s16(cospi21_23);
    let mut cospi23: int16x4_t = vget_high_s16(cospi21_23);
    let mut cospi25: int16x4_t = vget_low_s16(cospi25_27);
    let mut cospi27: int16x4_t = vget_high_s16(cospi25_27);
    let mut cospi29: int16x4_t = vget_low_s16(cospi29_31);
    let mut cospi31: int16x4_t = vget_high_s16(cospi29_31);
    // stage 1
    let mut x1: [int32x4_t; 64] = core::mem::zeroed();
    butterfly_dct_pre_s32_x4(input, x1.as_mut_ptr(), 64);
    // stage 2
    let mut x2: [int32x4_t; 64] = core::mem::zeroed();
    butterfly_dct_pre_s32_x4(x1.as_mut_ptr(), x2.as_mut_ptr(), 32);
    butterfly_s32_s32_x4_0112_neon(cospi32, x1[55], x1[40], &raw mut x2[55], &raw mut x2[40]);
    butterfly_s32_s32_x4_0112_neon(cospi32, x1[54], x1[41], &raw mut x2[54], &raw mut x2[41]);
    butterfly_s32_s32_x4_0112_neon(cospi32, x1[53], x1[42], &raw mut x2[53], &raw mut x2[42]);
    butterfly_s32_s32_x4_0112_neon(cospi32, x1[52], x1[43], &raw mut x2[52], &raw mut x2[43]);
    butterfly_s32_s32_x4_0112_neon(cospi32, x1[51], x1[44], &raw mut x2[51], &raw mut x2[44]);
    butterfly_s32_s32_x4_0112_neon(cospi32, x1[50], x1[45], &raw mut x2[50], &raw mut x2[45]);
    butterfly_s32_s32_x4_0112_neon(cospi32, x1[49], x1[46], &raw mut x2[49], &raw mut x2[46]);
    butterfly_s32_s32_x4_0112_neon(cospi32, x1[48], x1[47], &raw mut x2[48], &raw mut x2[47]);
    // stage 3
    let mut x3: [int32x4_t; 64] = core::mem::zeroed();
    butterfly_dct_pre_s32_x4(x2.as_mut_ptr(), x3.as_mut_ptr(), 16);
    butterfly_s32_s32_x4_0112_neon(cospi32, x2[27], x2[20], &raw mut x3[27], &raw mut x3[20]);
    butterfly_s32_s32_x4_0112_neon(cospi32, x2[26], x2[21], &raw mut x3[26], &raw mut x3[21]);
    butterfly_s32_s32_x4_0112_neon(cospi32, x2[25], x2[22], &raw mut x3[25], &raw mut x3[22]);
    butterfly_s32_s32_x4_0112_neon(cospi32, x2[24], x2[23], &raw mut x3[24], &raw mut x3[23]);
    butterfly_dct_post_s32_x4(x1.as_mut_ptr().offset(((32)) as isize), x2.as_mut_ptr().offset(((32)) as isize), x3.as_mut_ptr().offset(((32)) as isize), 32);
    // stage 4
    let mut x4: [int32x4_t; 64] = core::mem::zeroed();
    butterfly_dct_pre_s32_x4(x3.as_mut_ptr(), x4.as_mut_ptr(), 8);
    butterfly_s32_s32_x4_0112_neon(cospi32, x3[13], x3[10], &raw mut x4[13], &raw mut x4[10]);
    butterfly_s32_s32_x4_0112_neon(cospi32, x3[12], x3[11], &raw mut x4[12], &raw mut x4[11]);
    butterfly_dct_post_s32_x4(x2.as_mut_ptr().offset(((16)) as isize), x3.as_mut_ptr().offset(((16)) as isize), x4.as_mut_ptr().offset(((16)) as isize), 16);
    butterfly_s32_s32_x4_0112_neon(cospi16, x3[59], x3[36], &raw mut x4[59], &raw mut x4[36]);
    butterfly_s32_s32_x4_0112_neon(cospi16, x3[58], x3[37], &raw mut x4[58], &raw mut x4[37]);
    butterfly_s32_s32_x4_0112_neon(cospi16, x3[57], x3[38], &raw mut x4[57], &raw mut x4[38]);
    butterfly_s32_s32_x4_0112_neon(cospi16, x3[56], x3[39], &raw mut x4[56], &raw mut x4[39]);
    butterfly_s32_s32_x4_1223_neon(cospi16, x3[55], x3[40], &raw mut x4[55], &raw mut x4[40]);
    butterfly_s32_s32_x4_1223_neon(cospi16, x3[54], x3[41], &raw mut x4[54], &raw mut x4[41]);
    butterfly_s32_s32_x4_1223_neon(cospi16, x3[53], x3[42], &raw mut x4[53], &raw mut x4[42]);
    butterfly_s32_s32_x4_1223_neon(cospi16, x3[52], x3[43], &raw mut x4[52], &raw mut x4[43]);
    // stage 5
    let mut x5: [int32x4_t; 64] = core::mem::zeroed();
    butterfly_dct_pre_s32_x4(x4.as_mut_ptr(), x5.as_mut_ptr(), 4);
    butterfly_s32_s32_x4_0112_neon(cospi32, x4[6], x4[5], &raw mut x5[6], &raw mut x5[5]);
    butterfly_dct_post_s32_x4(x3.as_mut_ptr().offset(((8)) as isize), x4.as_mut_ptr().offset(((8)) as isize), x5.as_mut_ptr().offset(((8)) as isize), 8);
    butterfly_s32_s32_x4_0112_neon(cospi16, x4[29], x4[18], &raw mut x5[29], &raw mut x5[18]);
    butterfly_s32_s32_x4_0112_neon(cospi16, x4[28], x4[19], &raw mut x5[28], &raw mut x5[19]);
    butterfly_s32_s32_x4_1223_neon(cospi16, x4[27], x4[20], &raw mut x5[27], &raw mut x5[20]);
    butterfly_s32_s32_x4_1223_neon(cospi16, x4[26], x4[21], &raw mut x5[26], &raw mut x5[21]);
    butterfly_dct_post_s32_x4(x3.as_mut_ptr().offset(((32)) as isize), x4.as_mut_ptr().offset(((32)) as isize), x5.as_mut_ptr().offset(((32)) as isize), 16);
    butterfly_dct_post_s32_x4(x3.as_mut_ptr().offset(((48)) as isize), x4.as_mut_ptr().offset(((48)) as isize), x5.as_mut_ptr().offset(((48)) as isize), 16);
    // stage 6
    let mut x6: [int32x4_t; 64] = core::mem::zeroed();
    butterfly_s32_s32_x4_0112_neon(cospi32, x5[0], x5[1], &raw mut x6[0], &raw mut x6[1]);
    butterfly_s32_s32_x4_0112_neon(cospi16, x5[3], x5[2], &raw mut x6[2], &raw mut x6[3]);
    butterfly_dct_post_s32_x4(x4.as_mut_ptr().offset(((4)) as isize), x5.as_mut_ptr().offset(((4)) as isize), x6.as_mut_ptr().offset(((4)) as isize), 4);
    butterfly_s32_s32_x4_0112_neon(cospi16, x5[14], x5[9], &raw mut x6[14], &raw mut x6[9]);
    butterfly_s32_s32_x4_1223_neon(cospi16, x5[13], x5[10], &raw mut x6[13], &raw mut x6[10]);
    butterfly_dct_post_s32_x4(x4.as_mut_ptr().offset(((16)) as isize), x5.as_mut_ptr().offset(((16)) as isize), x6.as_mut_ptr().offset(((16)) as isize), 8);
    butterfly_dct_post_s32_x4(x4.as_mut_ptr().offset(((24)) as isize), x5.as_mut_ptr().offset(((24)) as isize), x6.as_mut_ptr().offset(((24)) as isize), 8);
    butterfly_s32_s32_x4_0112_neon(cospi8, x5[61], x5[34], &raw mut x6[61], &raw mut x6[34]);
    butterfly_s32_s32_x4_0112_neon(cospi8, x5[60], x5[35], &raw mut x6[60], &raw mut x6[35]);
    butterfly_s32_s32_x4_1223_neon(cospi8, x5[59], x5[36], &raw mut x6[59], &raw mut x6[36]);
    butterfly_s32_s32_x4_1223_neon(cospi8, x5[58], x5[37], &raw mut x6[58], &raw mut x6[37]);
    butterfly_s32_s32_x4_1003_neon(cospi24, x5[53], x5[42], &raw mut x6[53], &raw mut x6[42]);
    butterfly_s32_s32_x4_1003_neon(cospi24, x5[52], x5[43], &raw mut x6[52], &raw mut x6[43]);
    butterfly_s32_s32_x4_0332_neon(cospi24, x5[51], x5[44], &raw mut x6[51], &raw mut x6[44]);
    butterfly_s32_s32_x4_0332_neon(cospi24, x5[50], x5[45], &raw mut x6[50], &raw mut x6[45]);
    // stage 7
    let mut x7: [int32x4_t; 64] = core::mem::zeroed();
    butterfly_s32_s32_x4_0112_neon(cospi8, x6[7], x6[4], &raw mut x7[4], &raw mut x7[7]);
    butterfly_s32_s32_x4_1003_neon(cospi24, x6[6], x6[5], &raw mut x7[5], &raw mut x7[6]);
    butterfly_dct_post_s32_x4(x5.as_mut_ptr().offset(((8)) as isize), x6.as_mut_ptr().offset(((8)) as isize), x7.as_mut_ptr().offset(((8)) as isize), 4);
    butterfly_dct_post_s32_x4(x5.as_mut_ptr().offset(((12)) as isize), x6.as_mut_ptr().offset(((12)) as isize), x7.as_mut_ptr().offset(((12)) as isize), 4);
    butterfly_s32_s32_x4_0112_neon(cospi8, x6[30], x6[17], &raw mut x7[30], &raw mut x7[17]);
    butterfly_s32_s32_x4_1223_neon(cospi8, x6[29], x6[18], &raw mut x7[29], &raw mut x7[18]);
    butterfly_s32_s32_x4_1003_neon(cospi24, x6[26], x6[21], &raw mut x7[26], &raw mut x7[21]);
    butterfly_s32_s32_x4_0332_neon(cospi24, x6[25], x6[22], &raw mut x7[25], &raw mut x7[22]);
    butterfly_dct_post_s32_x4(x5.as_mut_ptr().offset(((32)) as isize), x6.as_mut_ptr().offset(((32)) as isize), x7.as_mut_ptr().offset(((32)) as isize), 8);
    butterfly_dct_post_s32_x4(x5.as_mut_ptr().offset(((40)) as isize), x6.as_mut_ptr().offset(((40)) as isize), x7.as_mut_ptr().offset(((40)) as isize), 8);
    butterfly_dct_post_s32_x4(x5.as_mut_ptr().offset(((48)) as isize), x6.as_mut_ptr().offset(((48)) as isize), x7.as_mut_ptr().offset(((48)) as isize), 8);
    butterfly_dct_post_s32_x4(x5.as_mut_ptr().offset(((56)) as isize), x6.as_mut_ptr().offset(((56)) as isize), x7.as_mut_ptr().offset(((56)) as isize), 8);
    // stage 8
    let mut x8: [int32x4_t; 64] = core::mem::zeroed();
    butterfly_s32_s32_x4_0112_neon(cospi4, x7[15], x7[8], &raw mut x8[8], &raw mut x8[15]);
    butterfly_s32_s32_x4_1003_neon(cospi28, x7[14], x7[9], &raw mut x8[9], &raw mut x8[14]);
    butterfly_s32_s32_x4_0112_neon(cospi20, x7[13], x7[10], &raw mut x8[10], &raw mut x8[13]);
    butterfly_s32_s32_x4_1003_neon(cospi12, x7[12], x7[11], &raw mut x8[11], &raw mut x8[12]);
    butterfly_dct_post_s32_x4(x6.as_mut_ptr().offset(((16)) as isize), x7.as_mut_ptr().offset(((16)) as isize), x8.as_mut_ptr().offset(((16)) as isize), 4);
    butterfly_dct_post_s32_x4(x6.as_mut_ptr().offset(((20)) as isize), x7.as_mut_ptr().offset(((20)) as isize), x8.as_mut_ptr().offset(((20)) as isize), 4);
    butterfly_dct_post_s32_x4(x6.as_mut_ptr().offset(((24)) as isize), x7.as_mut_ptr().offset(((24)) as isize), x8.as_mut_ptr().offset(((24)) as isize), 4);
    butterfly_dct_post_s32_x4(x6.as_mut_ptr().offset(((28)) as isize), x7.as_mut_ptr().offset(((28)) as isize), x8.as_mut_ptr().offset(((28)) as isize), 4);
    butterfly_s32_s32_x4_0112_neon(cospi4, x7[62], x7[33], &raw mut x8[62], &raw mut x8[33]);
    butterfly_s32_s32_x4_1223_neon(cospi4, x7[61], x7[34], &raw mut x8[61], &raw mut x8[34]);
    butterfly_s32_s32_x4_1003_neon(cospi28, x7[58], x7[37], &raw mut x8[58], &raw mut x8[37]);
    butterfly_s32_s32_x4_0332_neon(cospi28, x7[57], x7[38], &raw mut x8[57], &raw mut x8[38]);
    butterfly_s32_s32_x4_0112_neon(cospi20, x7[54], x7[41], &raw mut x8[54], &raw mut x8[41]);
    butterfly_s32_s32_x4_1223_neon(cospi20, x7[53], x7[42], &raw mut x8[53], &raw mut x8[42]);
    butterfly_s32_s32_x4_1003_neon(cospi12, x7[50], x7[45], &raw mut x8[50], &raw mut x8[45]);
    butterfly_s32_s32_x4_0332_neon(cospi12, x7[49], x7[46], &raw mut x8[49], &raw mut x8[46]);
    // stage 9
    let mut x9: [int32x4_t; 64] = core::mem::zeroed();
    butterfly_s32_s32_x4_0112_neon(cospi2, x8[31], x8[16], &raw mut x9[16], &raw mut x9[31]);
    butterfly_s32_s32_x4_1003_neon(cospi30, x8[30], x8[17], &raw mut x9[17], &raw mut x9[30]);
    butterfly_s32_s32_x4_0112_neon(cospi18, x8[29], x8[18], &raw mut x9[18], &raw mut x9[29]);
    butterfly_s32_s32_x4_1003_neon(cospi14, x8[28], x8[19], &raw mut x9[19], &raw mut x9[28]);
    butterfly_s32_s32_x4_0112_neon(cospi10, x8[27], x8[20], &raw mut x9[20], &raw mut x9[27]);
    butterfly_s32_s32_x4_1003_neon(cospi22, x8[26], x8[21], &raw mut x9[21], &raw mut x9[26]);
    butterfly_s32_s32_x4_0112_neon(cospi26, x8[25], x8[22], &raw mut x9[22], &raw mut x9[25]);
    butterfly_s32_s32_x4_1003_neon(cospi6, x8[24], x8[23], &raw mut x9[23], &raw mut x9[24]);
    butterfly_dct_post_s32_x4(x7.as_mut_ptr().offset(((32)) as isize), x8.as_mut_ptr().offset(((32)) as isize), x9.as_mut_ptr().offset(((32)) as isize), 4);
    butterfly_dct_post_s32_x4(x7.as_mut_ptr().offset(((36)) as isize), x8.as_mut_ptr().offset(((36)) as isize), x9.as_mut_ptr().offset(((36)) as isize), 4);
    butterfly_dct_post_s32_x4(x7.as_mut_ptr().offset(((40)) as isize), x8.as_mut_ptr().offset(((40)) as isize), x9.as_mut_ptr().offset(((40)) as isize), 4);
    butterfly_dct_post_s32_x4(x7.as_mut_ptr().offset(((44)) as isize), x8.as_mut_ptr().offset(((44)) as isize), x9.as_mut_ptr().offset(((44)) as isize), 4);
    butterfly_dct_post_s32_x4(x7.as_mut_ptr().offset(((48)) as isize), x8.as_mut_ptr().offset(((48)) as isize), x9.as_mut_ptr().offset(((48)) as isize), 4);
    butterfly_dct_post_s32_x4(x7.as_mut_ptr().offset(((52)) as isize), x8.as_mut_ptr().offset(((52)) as isize), x9.as_mut_ptr().offset(((52)) as isize), 4);
    butterfly_dct_post_s32_x4(x7.as_mut_ptr().offset(((56)) as isize), x8.as_mut_ptr().offset(((56)) as isize), x9.as_mut_ptr().offset(((56)) as isize), 4);
    butterfly_dct_post_s32_x4(x7.as_mut_ptr().offset(((60)) as isize), x8.as_mut_ptr().offset(((60)) as isize), x9.as_mut_ptr().offset(((60)) as isize), 4);
    // stage 10
    let mut x10: [int32x4_t; 64] = core::mem::zeroed();
    butterfly_s32_s32_x4_0112_neon(cospi1, x9[63], x9[32], &raw mut x10[32], &raw mut x10[63]);
    butterfly_s32_s32_x4_1003_neon(cospi31, x9[62], x9[33], &raw mut x10[33], &raw mut x10[62]);
    butterfly_s32_s32_x4_0112_neon(cospi17, x9[61], x9[34], &raw mut x10[34], &raw mut x10[61]);
    butterfly_s32_s32_x4_1003_neon(cospi15, x9[60], x9[35], &raw mut x10[35], &raw mut x10[60]);
    butterfly_s32_s32_x4_0112_neon(cospi9, x9[59], x9[36], &raw mut x10[36], &raw mut x10[59]);
    butterfly_s32_s32_x4_1003_neon(cospi23, x9[58], x9[37], &raw mut x10[37], &raw mut x10[58]);
    butterfly_s32_s32_x4_0112_neon(cospi25, x9[57], x9[38], &raw mut x10[38], &raw mut x10[57]);
    butterfly_s32_s32_x4_1003_neon(cospi7, x9[56], x9[39], &raw mut x10[39], &raw mut x10[56]);
    butterfly_s32_s32_x4_0112_neon(cospi5, x9[55], x9[40], &raw mut x10[40], &raw mut x10[55]);
    butterfly_s32_s32_x4_1003_neon(cospi27, x9[54], x9[41], &raw mut x10[41], &raw mut x10[54]);
    butterfly_s32_s32_x4_0112_neon(cospi21, x9[53], x9[42], &raw mut x10[42], &raw mut x10[53]);
    butterfly_s32_s32_x4_1003_neon(cospi11, x9[52], x9[43], &raw mut x10[43], &raw mut x10[52]);
    butterfly_s32_s32_x4_0112_neon(cospi13, x9[51], x9[44], &raw mut x10[44], &raw mut x10[51]);
    butterfly_s32_s32_x4_1003_neon(cospi19, x9[50], x9[45], &raw mut x10[45], &raw mut x10[50]);
    butterfly_s32_s32_x4_0112_neon(cospi29, x9[49], x9[46], &raw mut x10[46], &raw mut x10[49]);
    butterfly_s32_s32_x4_1003_neon(cospi3, x9[48], x9[47], &raw mut x10[47], &raw mut x10[48]);
    // stage 11, only store into the low 32 output indices.
    *output.add(0) = x6[0];
    *output.add(1) = x10[32];
    *output.add(2) = x9[16];
    *output.add(3) = x10[48];
    *output.add(4) = x8[8];
    *output.add(5) = x10[40];
    *output.add(6) = x9[24];
    *output.add(7) = x10[56];
    *output.add(8) = x7[4];
    *output.add(9) = x10[36];
    *output.add(10) = x9[20];
    *output.add(11) = x10[52];
    *output.add(12) = x8[12];
    *output.add(13) = x10[44];
    *output.add(14) = x9[28];
    *output.add(15) = x10[60];
    *output.add(16) = x6[2];
    *output.add(17) = x10[34];
    *output.add(18) = x9[18];
    *output.add(19) = x10[50];
    *output.add(20) = x8[10];
    *output.add(21) = x10[42];
    *output.add(22) = x9[26];
    *output.add(23) = x10[58];
    *output.add(24) = x7[6];
    *output.add(25) = x10[38];
    *output.add(26) = x9[22];
    *output.add(27) = x10[54];
    *output.add(28) = x8[14];
    *output.add(29) = x10[46];
    *output.add(30) = x9[30];
    *output.add(31) = x10[62];
}

unsafe fn lowbd_fwd_txfm2d_64x64_neon(mut input: *const i16, mut output: *mut i32, mut stride: i32, mut tx_type: TX_TYPE, mut bd: i32) {
    debug_assert!(tx_type == DCT_DCT);
    let mut buf0: [int16x8_t; 64] = core::mem::zeroed();
    let mut buf1: [int16x8_t; 512] = core::mem::zeroed();
    let mut col_txfm: transform_1d_lbd_8_neon = Some(fdct8x64_neon);
    for i in 0..8 {
        load_buffer_s16_x8(input.offset((8 * i) as isize), stride, buf0.as_mut_ptr(), 64);
        col_txfm.unwrap()(buf0.as_mut_ptr(), buf0.as_mut_ptr(), 13);
        shift_right_2_round_s16_x8(buf0.as_mut_ptr(), buf0.as_mut_ptr(), 64);
        for j in 0..4 {
            transpose_arrays_s16_8x8(buf0.as_mut_ptr().offset((j * 8) as isize), buf1.as_mut_ptr().offset(((j * 64 + 8 * i)) as isize));
        }
    }
    for i in 0..4 {
        let mut bufA: [int32x4_t; 64] = core::mem::zeroed();
        let mut bufB: [int32x4_t; 64] = core::mem::zeroed();
        let mut buf: *mut int16x8_t = buf1.as_mut_ptr().offset((64 * i) as isize);
        for j in 0..64 {
            bufA[j as usize] = vmovl_s16(vget_low_s16(*buf.offset(j as isize)));
            bufB[j as usize] = vmovl_s16(vget_high_s16(*buf.offset(j as isize)));
        }
        fdct64_neon(bufA.as_mut_ptr(), bufA.as_mut_ptr(), 10);
        fdct64_neon(bufB.as_mut_ptr(), bufB.as_mut_ptr(), 10);
        shift_right_2_round_s32_x4(bufA.as_mut_ptr(), bufA.as_mut_ptr(), 32);
        shift_right_2_round_s32_x4(bufB.as_mut_ptr(), bufB.as_mut_ptr(), 32);
        store_buffer_interleaved_s32_x8(output.offset((i * 8) as isize), bufA.as_mut_ptr(), bufB.as_mut_ptr(), 32, 32);
    }
}

unsafe fn lowbd_fwd_txfm2d_64x32_neon(mut input: *const i16, mut output: *mut i32, mut stride: i32, mut tx_type: TX_TYPE, mut bd: i32) {
    let mut buf0: [int16x8_t; 64] = core::mem::zeroed();
    let mut buf1: [int16x8_t; 256] = core::mem::zeroed();
    let mut col_txfm: col_transform_1d_lbd_8_neon = col_txfm8x32_arr[tx_type as usize];
    for i in 0..8 {
        col_txfm.unwrap()(input.offset((8 * i) as isize), buf0.as_mut_ptr(), stride, 12);
        shift_right_4_round_s16_x8(buf0.as_mut_ptr(), buf0.as_mut_ptr(), 32);
        for j in 0..4 {
            transpose_arrays_s16_8x8(buf0.as_mut_ptr().offset((j * 8) as isize), buf1.as_mut_ptr().offset(((j * 64 + 8 * i)) as isize));
        }
    }
    debug_assert!(tx_type == DCT_DCT);
    for i in 0..4 {
        let mut bufA: [int32x4_t; 64] = core::mem::zeroed();
        let mut bufB: [int32x4_t; 64] = core::mem::zeroed();
        let mut buf: *mut int16x8_t = buf1.as_mut_ptr().offset((64 * i) as isize);
        for j in 0..64 {
            bufA[j as usize] = vmovl_s16(vget_low_s16(*buf.offset(j as isize)));
            bufB[j as usize] = vmovl_s16(vget_high_s16(*buf.offset(j as isize)));
        }
        fdct64_neon(bufA.as_mut_ptr(), bufA.as_mut_ptr(), 11);
        fdct64_neon(bufB.as_mut_ptr(), bufB.as_mut_ptr(), 11);
        shift_right_2_round_s32_x4(bufA.as_mut_ptr(), bufA.as_mut_ptr(), 32);
        shift_right_2_round_s32_x4(bufB.as_mut_ptr(), bufB.as_mut_ptr(), 32);
        round_shift_sqrt2_s32_s32_4xn_neon(bufA.as_mut_ptr(), bufA.as_mut_ptr(), 32);
        round_shift_sqrt2_s32_s32_4xn_neon(bufB.as_mut_ptr(), bufB.as_mut_ptr(), 32);
        store_buffer_interleaved_s32_x8(output.offset((i * 8) as isize), bufA.as_mut_ptr(), bufB.as_mut_ptr(), 32, 32);
    }
}

unsafe fn lowbd_fwd_txfm2d_32x64_neon(mut input: *const i16, mut output: *mut i32, mut stride: i32, mut tx_type: TX_TYPE, mut bd: i32) {
    debug_assert!(tx_type == DCT_DCT);
    let mut buf0: [int16x8_t; 64] = core::mem::zeroed();
    let mut buf1: [int16x8_t; 256] = core::mem::zeroed();
    let mut col_txfm: transform_1d_lbd_8_neon = Some(fdct8x64_neon);
    for i in 0..4 {
        load_buffer_s16_x8(input.offset((8 * i) as isize), stride, buf0.as_mut_ptr(), 64);
        col_txfm.unwrap()(buf0.as_mut_ptr(), buf0.as_mut_ptr(), 13);
        shift_right_2_round_s16_x8(buf0.as_mut_ptr(), buf0.as_mut_ptr(), 64);
        for j in 0..4 {
            transpose_arrays_s16_8x8(buf0.as_mut_ptr().offset((j * 8) as isize), buf1.as_mut_ptr().offset(((j * 32 + 8 * i)) as isize));
        }
    }
    for i in 0..4 {
        let mut bufA: [int32x4_t; 32] = core::mem::zeroed();
        let mut bufB: [int32x4_t; 32] = core::mem::zeroed();
        let mut buf: *mut int16x8_t = buf1.as_mut_ptr().offset((32 * i) as isize);
        for j in 0..32 {
            bufA[j as usize] = vmovl_s16(vget_low_s16(*buf.offset(j as isize)));
            bufB[j as usize] = vmovl_s16(vget_high_s16(*buf.offset(j as isize)));
        }
        fdct32_neon(bufA.as_mut_ptr(), bufA.as_mut_ptr(), 11);
        fdct32_neon(bufB.as_mut_ptr(), bufB.as_mut_ptr(), 11);
        shift_right_2_round_s32_x4(bufA.as_mut_ptr(), bufA.as_mut_ptr(), 32);
        shift_right_2_round_s32_x4(bufB.as_mut_ptr(), bufB.as_mut_ptr(), 32);
        round_shift_sqrt2_s32_s32_4xn_neon(bufA.as_mut_ptr(), bufA.as_mut_ptr(), 32);
        round_shift_sqrt2_s32_s32_4xn_neon(bufB.as_mut_ptr(), bufB.as_mut_ptr(), 32);
        store_buffer_interleaved_s32_x8(output.offset((i * 8) as isize), bufA.as_mut_ptr(), bufB.as_mut_ptr(), 32, 32);
    }
}

static lowbd_fwd_txfm_func_ls: [FwdTxfm2dFunc; TX_SIZES_ALL] = [
    Some(lowbd_fwd_txfm2d_4x4_neon),   // 4x4 transform
    Some(lowbd_fwd_txfm2d_8x8_neon),   // 8x8 transform
    Some(lowbd_fwd_txfm2d_16x16_neon), // 16x16 transform
    Some(lowbd_fwd_txfm2d_32x32_neon), // 32x32 transform
    Some(lowbd_fwd_txfm2d_64x64_neon), // 64x64 transform
    Some(lowbd_fwd_txfm2d_4x8_neon),   // 4x8 transform
    Some(lowbd_fwd_txfm2d_8x4_neon),   // 8x4 transform
    Some(lowbd_fwd_txfm2d_8x16_neon),  // 8x16 transform
    Some(lowbd_fwd_txfm2d_16x8_neon),  // 16x8 transform
    Some(lowbd_fwd_txfm2d_16x32_neon), // 16x32 transform
    Some(lowbd_fwd_txfm2d_32x16_neon), // 32x16 transform
    Some(lowbd_fwd_txfm2d_32x64_neon), // 32x64 transform
    Some(lowbd_fwd_txfm2d_64x32_neon), // 64x32 transform
    Some(lowbd_fwd_txfm2d_4x16_neon),  // 4x16 transform
    Some(lowbd_fwd_txfm2d_16x4_neon),  // 16x4 transform
    Some(lowbd_fwd_txfm2d_8x32_neon),  // 8x32 transform
    Some(lowbd_fwd_txfm2d_32x8_neon),  // 32x8 transform
    Some(lowbd_fwd_txfm2d_16x64_neon), // 16x64 transform
    Some(lowbd_fwd_txfm2d_64x16_neon), // 64x16 transform
];

#[no_mangle]
pub unsafe extern "C-unwind" fn av1_lowbd_fwd_txfm_neon(src_diff: *const i16, coeff: *mut tran_low_t,
                                                       diff_stride: i32, txfm_param: *mut TxfmParam) {
    let fwd_txfm2d_func: FwdTxfm2dFunc = lowbd_fwd_txfm_func_ls[(*txfm_param).tx_size as usize];
    if (*txfm_param).lossless != 0 && (*txfm_param).tx_size == TX_4X4 {
        av1_lowbd_fwd_txfm_c(src_diff, coeff, diff_stride, txfm_param);
    } else {
        fwd_txfm2d_func.unwrap()(src_diff, coeff, diff_stride, (*txfm_param).tx_type,
                                 (*txfm_param).bd);
    }
}
