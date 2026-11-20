/*
 * Copyright (c) 2023, Alliance for Open Media. All rights reserved.
 *
 * This source code is subject to the terms of the BSD 2 Clause License and
 * the Alliance for Open Media Patent License 1.0. If the BSD 2 Clause License
 * was not distributed with this source code in the LICENSE file, you can
 * obtain it at www.aomedia.org/license/software. If the Alliance for Open
 * Media Patent License 1.0 was not distributed with this source code in the
 * PATENTS file, you can obtain it at www.aomedia.org/license/patent.
 */

// Translated by hand from upstream/aom/aom_dsp/arm/highbd_hadamard_neon.c
// (ADR 0056). Function names, helper structure and intrinsic sequences follow
// the original.
#![allow(unused_mut, clippy::all)]

use core::arch::aarch64::*;

use crate::simd::aom_dsp__arm__transpose_neon::transpose_elems_inplace_s16_8x8;
use crate::simd::types::tran_low_t;

#[inline(always)]
unsafe fn hadamard_highbd_col8_first_pass(a0: *mut int16x8_t, a1: *mut int16x8_t,
                                          a2: *mut int16x8_t, a3: *mut int16x8_t,
                                          a4: *mut int16x8_t, a5: *mut int16x8_t,
                                          a6: *mut int16x8_t, a7: *mut int16x8_t) {
    let b0: int16x8_t = vaddq_s16(*a0, *a1);
    let b1: int16x8_t = vsubq_s16(*a0, *a1);
    let b2: int16x8_t = vaddq_s16(*a2, *a3);
    let b3: int16x8_t = vsubq_s16(*a2, *a3);
    let b4: int16x8_t = vaddq_s16(*a4, *a5);
    let b5: int16x8_t = vsubq_s16(*a4, *a5);
    let b6: int16x8_t = vaddq_s16(*a6, *a7);
    let b7: int16x8_t = vsubq_s16(*a6, *a7);

    let c0: int16x8_t = vaddq_s16(b0, b2);
    let c2: int16x8_t = vsubq_s16(b0, b2);
    let c1: int16x8_t = vaddq_s16(b1, b3);
    let c3: int16x8_t = vsubq_s16(b1, b3);
    let c4: int16x8_t = vaddq_s16(b4, b6);
    let c6: int16x8_t = vsubq_s16(b4, b6);
    let c5: int16x8_t = vaddq_s16(b5, b7);
    let c7: int16x8_t = vsubq_s16(b5, b7);

    *a0 = vaddq_s16(c0, c4);
    *a2 = vsubq_s16(c0, c4);
    *a7 = vaddq_s16(c1, c5);
    *a6 = vsubq_s16(c1, c5);
    *a3 = vaddq_s16(c2, c6);
    *a1 = vsubq_s16(c2, c6);
    *a4 = vaddq_s16(c3, c7);
    *a5 = vsubq_s16(c3, c7);
}

#[inline(always)]
unsafe fn hadamard_highbd_col4_second_pass(a0: int16x4_t, a1: int16x4_t, a2: int16x4_t,
                                           a3: int16x4_t, a4: int16x4_t, a5: int16x4_t,
                                           a6: int16x4_t, a7: int16x4_t,
                                           coeff: *mut tran_low_t) {
    let b0: int32x4_t = vaddl_s16(a0, a1);
    let b1: int32x4_t = vsubl_s16(a0, a1);
    let b2: int32x4_t = vaddl_s16(a2, a3);
    let b3: int32x4_t = vsubl_s16(a2, a3);
    let b4: int32x4_t = vaddl_s16(a4, a5);
    let b5: int32x4_t = vsubl_s16(a4, a5);
    let b6: int32x4_t = vaddl_s16(a6, a7);
    let b7: int32x4_t = vsubl_s16(a6, a7);

    let c0: int32x4_t = vaddq_s32(b0, b2);
    let c2: int32x4_t = vsubq_s32(b0, b2);
    let c1: int32x4_t = vaddq_s32(b1, b3);
    let c3: int32x4_t = vsubq_s32(b1, b3);
    let c4: int32x4_t = vaddq_s32(b4, b6);
    let c6: int32x4_t = vsubq_s32(b4, b6);
    let c5: int32x4_t = vaddq_s32(b5, b7);
    let c7: int32x4_t = vsubq_s32(b5, b7);

    let d0: int32x4_t = vaddq_s32(c0, c4);
    let d2: int32x4_t = vsubq_s32(c0, c4);
    let d7: int32x4_t = vaddq_s32(c1, c5);
    let d6: int32x4_t = vsubq_s32(c1, c5);
    let d3: int32x4_t = vaddq_s32(c2, c6);
    let d1: int32x4_t = vsubq_s32(c2, c6);
    let d4: int32x4_t = vaddq_s32(c3, c7);
    let d5: int32x4_t = vsubq_s32(c3, c7);

    vst1q_s32(coeff.offset(0), d0);
    vst1q_s32(coeff.offset(4), d1);
    vst1q_s32(coeff.offset(8), d2);
    vst1q_s32(coeff.offset(12), d3);
    vst1q_s32(coeff.offset(16), d4);
    vst1q_s32(coeff.offset(20), d5);
    vst1q_s32(coeff.offset(24), d6);
    vst1q_s32(coeff.offset(28), d7);
}

#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_hadamard_8x8_neon(src_diff: *const i16,
                                                             src_stride: isize,
                                                             coeff: *mut tran_low_t) {
    let mut b0: int16x4_t;
    let mut b1: int16x4_t;
    let mut b2: int16x4_t;
    let mut b3: int16x4_t;
    let mut b4: int16x4_t;
    let mut b5: int16x4_t;
    let mut b6: int16x4_t;
    let mut b7: int16x4_t;

    let mut s0: int16x8_t = vld1q_s16(src_diff.offset(0 * src_stride));
    let mut s1: int16x8_t = vld1q_s16(src_diff.offset(1 * src_stride));
    let mut s2: int16x8_t = vld1q_s16(src_diff.offset(2 * src_stride));
    let mut s3: int16x8_t = vld1q_s16(src_diff.offset(3 * src_stride));
    let mut s4: int16x8_t = vld1q_s16(src_diff.offset(4 * src_stride));
    let mut s5: int16x8_t = vld1q_s16(src_diff.offset(5 * src_stride));
    let mut s6: int16x8_t = vld1q_s16(src_diff.offset(6 * src_stride));
    let mut s7: int16x8_t = vld1q_s16(src_diff.offset(7 * src_stride));

    // For the first pass we can stay in 16-bit elements (4095*8 = 32760).
    hadamard_highbd_col8_first_pass(&mut s0, &mut s1, &mut s2, &mut s3, &mut s4, &mut s5, &mut s6,
                                    &mut s7);

    transpose_elems_inplace_s16_8x8(&mut s0, &mut s1, &mut s2, &mut s3, &mut s4, &mut s5, &mut s6,
                                    &mut s7);

    // For the second pass we need to widen to 32-bit elements, so we're
    // processing 4 columns at a time.
    // Skip the second transpose because it is not required.

    b0 = vget_low_s16(s0);
    b1 = vget_low_s16(s1);
    b2 = vget_low_s16(s2);
    b3 = vget_low_s16(s3);
    b4 = vget_low_s16(s4);
    b5 = vget_low_s16(s5);
    b6 = vget_low_s16(s6);
    b7 = vget_low_s16(s7);

    hadamard_highbd_col4_second_pass(b0, b1, b2, b3, b4, b5, b6, b7, coeff);

    b0 = vget_high_s16(s0);
    b1 = vget_high_s16(s1);
    b2 = vget_high_s16(s2);
    b3 = vget_high_s16(s3);
    b4 = vget_high_s16(s4);
    b5 = vget_high_s16(s5);
    b6 = vget_high_s16(s6);
    b7 = vget_high_s16(s7);

    hadamard_highbd_col4_second_pass(b0, b1, b2, b3, b4, b5, b6, b7, coeff.offset(32));
}

#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_hadamard_16x16_neon(src_diff: *const i16,
                                                               src_stride: isize,
                                                               coeff: *mut tran_low_t) {
    // Rearrange 16x16 to 8x32 and remove stride.
    // Top left first.
    aom_highbd_hadamard_8x8_neon(src_diff, src_stride, coeff);
    // Top right.
    aom_highbd_hadamard_8x8_neon(src_diff.offset(8), src_stride, coeff.offset(64));
    // Bottom left.
    aom_highbd_hadamard_8x8_neon(src_diff.offset(8 * src_stride), src_stride, coeff.offset(128));
    // Bottom right.
    aom_highbd_hadamard_8x8_neon(src_diff.offset(8 * src_stride + 8), src_stride,
                                 coeff.offset(192));

    for i in 0..16isize {
        let a0: int32x4_t = vld1q_s32(coeff.offset(4 * i));
        let a1: int32x4_t = vld1q_s32(coeff.offset(4 * i + 64));
        let a2: int32x4_t = vld1q_s32(coeff.offset(4 * i + 128));
        let a3: int32x4_t = vld1q_s32(coeff.offset(4 * i + 192));

        let b0: int32x4_t = vhaddq_s32(a0, a1);
        let b1: int32x4_t = vhsubq_s32(a0, a1);
        let b2: int32x4_t = vhaddq_s32(a2, a3);
        let b3: int32x4_t = vhsubq_s32(a2, a3);

        let c0: int32x4_t = vaddq_s32(b0, b2);
        let c1: int32x4_t = vaddq_s32(b1, b3);
        let c2: int32x4_t = vsubq_s32(b0, b2);
        let c3: int32x4_t = vsubq_s32(b1, b3);

        vst1q_s32(coeff.offset(4 * i), c0);
        vst1q_s32(coeff.offset(4 * i + 64), c1);
        vst1q_s32(coeff.offset(4 * i + 128), c2);
        vst1q_s32(coeff.offset(4 * i + 192), c3);
    }
}

#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_hadamard_32x32_neon(src_diff: *const i16,
                                                               src_stride: isize,
                                                               coeff: *mut tran_low_t) {
    // Rearrange 32x32 to 16x64 and remove stride.
    // Top left first.
    aom_highbd_hadamard_16x16_neon(src_diff, src_stride, coeff);
    // Top right.
    aom_highbd_hadamard_16x16_neon(src_diff.offset(16), src_stride, coeff.offset(256));
    // Bottom left.
    aom_highbd_hadamard_16x16_neon(src_diff.offset(16 * src_stride), src_stride,
                                   coeff.offset(512));
    // Bottom right.
    aom_highbd_hadamard_16x16_neon(src_diff.offset(16 * src_stride + 16), src_stride,
                                   coeff.offset(768));

    for i in 0..64isize {
        let a0: int32x4_t = vld1q_s32(coeff.offset(4 * i));
        let a1: int32x4_t = vld1q_s32(coeff.offset(4 * i + 256));
        let a2: int32x4_t = vld1q_s32(coeff.offset(4 * i + 512));
        let a3: int32x4_t = vld1q_s32(coeff.offset(4 * i + 768));

        let b0: int32x4_t = vshrq_n_s32::<2>(vaddq_s32(a0, a1));
        let b1: int32x4_t = vshrq_n_s32::<2>(vsubq_s32(a0, a1));
        let b2: int32x4_t = vshrq_n_s32::<2>(vaddq_s32(a2, a3));
        let b3: int32x4_t = vshrq_n_s32::<2>(vsubq_s32(a2, a3));

        let c0: int32x4_t = vaddq_s32(b0, b2);
        let c1: int32x4_t = vaddq_s32(b1, b3);
        let c2: int32x4_t = vsubq_s32(b0, b2);
        let c3: int32x4_t = vsubq_s32(b1, b3);

        vst1q_s32(coeff.offset(4 * i), c0);
        vst1q_s32(coeff.offset(4 * i + 256), c1);
        vst1q_s32(coeff.offset(4 * i + 512), c2);
        vst1q_s32(coeff.offset(4 * i + 768), c3);
    }
}
