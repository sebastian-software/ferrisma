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

// Translated by hand from upstream/aom/av1/encoder/arm/hybrid_fwd_txfm_neon.c
// (ADR 0056). Intrinsic sequence, helper structure, and names follow the
// original. C locals that the original leaves uninitialized start zeroed here.
use core::arch::aarch64::*;

use crate::simd::types::tran_low_t;

// aom_dsp/txfm_common.h
const UNIT_QUANT_SHIFT: i32 = 2;

#[inline(always)]
unsafe fn transpose4x4(in_: *const int16x8_t, out: *mut int16x4_t) {
    let b0: int32x4x2_t = vtrnq_s32(vreinterpretq_s32_s16(*in_), vreinterpretq_s32_s16(*in_.add(1)));
    let c0: int16x4x2_t = vtrn_s16(vreinterpret_s16_s32(vget_low_s32(b0.0)),
                                   vreinterpret_s16_s32(vget_high_s32(b0.0)));
    let c1: int16x4x2_t = vtrn_s16(vreinterpret_s16_s32(vget_low_s32(b0.1)),
                                   vreinterpret_s16_s32(vget_high_s32(b0.1)));
    *out = c0.0;
    *out.add(1) = c0.1;
    *out.add(2) = c1.0;
    *out.add(3) = c1.1;
}

#[no_mangle]
pub unsafe extern "C-unwind" fn av1_fwht4x4_neon(input: *const i16, output: *mut tran_low_t,
                                                 stride: i32) {
    // Load the 4x4 source in transposed form.
    let mut a1: int16x4_t;
    let mut b1: int16x4_t;
    let mut c1: int16x4_t;
    let mut d1: int16x4_t;
    let mut e: int16x4_t;
    a1 = vld1_s16(input);
    b1 = vld1_s16(input.offset((1 * stride) as isize));
    c1 = vld1_s16(input.offset((2 * stride) as isize));
    d1 = vld1_s16(input.offset((3 * stride) as isize));

    // WHT.

    // Row transforms.
    a1 = vadd_s16(a1, b1);
    d1 = vsub_s16(d1, c1);
    e = vhsub_s16(a1, d1);
    b1 = vsub_s16(e, b1);
    c1 = vsub_s16(e, c1);
    a1 = vsub_s16(a1, c1);
    d1 = vadd_s16(d1, b1);

    let mut x: [int16x8_t; 2] = core::mem::zeroed();
    x[0] = vcombine_s16(a1, c1);
    x[1] = vcombine_s16(d1, b1);

    let mut s: [int16x4_t; 4] = core::mem::zeroed();
    transpose4x4(x.as_ptr(), s.as_mut_ptr());

    a1 = s[0];
    b1 = s[1];
    c1 = s[2];
    d1 = s[3];

    // Row transforms.
    a1 = vadd_s16(a1, b1);
    d1 = vsub_s16(d1, c1);
    e = vhsub_s16(a1, d1);
    b1 = vsub_s16(e, b1);
    c1 = vsub_s16(e, c1);
    a1 = vsub_s16(a1, c1);
    d1 = vadd_s16(d1, b1);

    vst1q_s32(output, vshll_n_s16::<UNIT_QUANT_SHIFT>(a1));
    vst1q_s32(output.add(4), vshll_n_s16::<UNIT_QUANT_SHIFT>(c1));
    vst1q_s32(output.add(8), vshll_n_s16::<UNIT_QUANT_SHIFT>(d1));
    vst1q_s32(output.add(12), vshll_n_s16::<UNIT_QUANT_SHIFT>(b1));
}
