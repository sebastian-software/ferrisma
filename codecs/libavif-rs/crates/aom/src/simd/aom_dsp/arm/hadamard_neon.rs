/*
 * Copyright (c) 2019, Alliance for Open Media. All rights reserved.
 *
 * This source code is subject to the terms of the BSD 2 Clause License and
 * the Alliance for Open Media Patent License 1.0. If the BSD 2 Clause License
 * was not distributed with this source code in the LICENSE file, you can
 * obtain it at www.aomedia.org/license/software. If the Alliance for Open
 * Media Patent License 1.0 was not distributed with this source code in the
 * PATENTS file, you can obtain it at www.aomedia.org/license/patent.
 */

// Translated by hand from upstream/aom/aom_dsp/arm/hadamard_neon.c (ADR 0056).
// Function names, helper structure and intrinsic sequences follow the
// original. Pointer advances (`coeff += 8`) are `offset` on the raw pointer.
#![allow(unused_mut, clippy::all)]

use core::arch::aarch64::*;

use crate::simd::aom_dsp__arm__mem_neon::{store_s16_to_tran_low, store_s16q_to_tran_low};
use crate::simd::aom_dsp__arm__transpose_neon::{transpose_elems_inplace_s16_4x4,
                                                transpose_elems_inplace_s16_8x8};
use crate::simd::types::tran_low_t;

#[inline(always)]
unsafe fn hadamard_4x4_one_pass(a0: *mut int16x4_t, a1: *mut int16x4_t, a2: *mut int16x4_t,
                                a3: *mut int16x4_t) {
    let b0: int16x4_t = vhadd_s16(*a0, *a1);
    let b1: int16x4_t = vhsub_s16(*a0, *a1);
    let b2: int16x4_t = vhadd_s16(*a2, *a3);
    let b3: int16x4_t = vhsub_s16(*a2, *a3);

    *a0 = vadd_s16(b0, b2);
    *a1 = vadd_s16(b1, b3);
    *a2 = vsub_s16(b0, b2);
    *a3 = vsub_s16(b1, b3);
}

#[no_mangle]
pub unsafe extern "C-unwind" fn aom_hadamard_4x4_neon(src_diff: *const i16, src_stride: isize,
                                                      coeff: *mut tran_low_t) {
    let mut a0: int16x4_t = vld1_s16(src_diff);
    let mut a1: int16x4_t = vld1_s16(src_diff.offset(src_stride));
    let mut a2: int16x4_t = vld1_s16(src_diff.offset(2 * src_stride));
    let mut a3: int16x4_t = vld1_s16(src_diff.offset(3 * src_stride));

    hadamard_4x4_one_pass(&mut a0, &mut a1, &mut a2, &mut a3);

    transpose_elems_inplace_s16_4x4(&mut a0, &mut a1, &mut a2, &mut a3);

    hadamard_4x4_one_pass(&mut a0, &mut a1, &mut a2, &mut a3);

    store_s16_to_tran_low(coeff, a0);
    store_s16_to_tran_low(coeff.offset(4), a1);
    store_s16_to_tran_low(coeff.offset(8), a2);
    store_s16_to_tran_low(coeff.offset(12), a3);
}

#[inline(always)]
unsafe fn hadamard8x8_one_pass(a0: *mut int16x8_t, a1: *mut int16x8_t, a2: *mut int16x8_t,
                               a3: *mut int16x8_t, a4: *mut int16x8_t, a5: *mut int16x8_t,
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
    let c1: int16x8_t = vaddq_s16(b1, b3);
    let c2: int16x8_t = vsubq_s16(b0, b2);
    let c3: int16x8_t = vsubq_s16(b1, b3);
    let c4: int16x8_t = vaddq_s16(b4, b6);
    let c5: int16x8_t = vaddq_s16(b5, b7);
    let c6: int16x8_t = vsubq_s16(b4, b6);
    let c7: int16x8_t = vsubq_s16(b5, b7);

    *a0 = vaddq_s16(c0, c4);
    *a1 = vsubq_s16(c2, c6);
    *a2 = vsubq_s16(c0, c4);
    *a3 = vaddq_s16(c2, c6);
    *a4 = vaddq_s16(c3, c7);
    *a5 = vsubq_s16(c3, c7);
    *a6 = vsubq_s16(c1, c5);
    *a7 = vaddq_s16(c1, c5);
}

#[no_mangle]
pub unsafe extern "C-unwind" fn aom_hadamard_8x8_neon(src_diff: *const i16, src_stride: isize,
                                                      coeff: *mut tran_low_t) {
    let mut a0: int16x8_t = vld1q_s16(src_diff);
    let mut a1: int16x8_t = vld1q_s16(src_diff.offset(src_stride));
    let mut a2: int16x8_t = vld1q_s16(src_diff.offset(2 * src_stride));
    let mut a3: int16x8_t = vld1q_s16(src_diff.offset(3 * src_stride));
    let mut a4: int16x8_t = vld1q_s16(src_diff.offset(4 * src_stride));
    let mut a5: int16x8_t = vld1q_s16(src_diff.offset(5 * src_stride));
    let mut a6: int16x8_t = vld1q_s16(src_diff.offset(6 * src_stride));
    let mut a7: int16x8_t = vld1q_s16(src_diff.offset(7 * src_stride));

    hadamard8x8_one_pass(&mut a0, &mut a1, &mut a2, &mut a3, &mut a4, &mut a5, &mut a6, &mut a7);

    transpose_elems_inplace_s16_8x8(&mut a0, &mut a1, &mut a2, &mut a3, &mut a4, &mut a5, &mut a6,
                                    &mut a7);

    hadamard8x8_one_pass(&mut a0, &mut a1, &mut a2, &mut a3, &mut a4, &mut a5, &mut a6, &mut a7);

    // Skip the second transpose because it is not required.

    store_s16q_to_tran_low(coeff.offset(0), a0);
    store_s16q_to_tran_low(coeff.offset(8), a1);
    store_s16q_to_tran_low(coeff.offset(16), a2);
    store_s16q_to_tran_low(coeff.offset(24), a3);
    store_s16q_to_tran_low(coeff.offset(32), a4);
    store_s16q_to_tran_low(coeff.offset(40), a5);
    store_s16q_to_tran_low(coeff.offset(48), a6);
    store_s16q_to_tran_low(coeff.offset(56), a7);
}

#[no_mangle]
pub unsafe extern "C-unwind" fn aom_hadamard_lp_8x8_neon(src_diff: *const i16, src_stride: isize,
                                                         coeff: *mut i16) {
    let mut a0: int16x8_t = vld1q_s16(src_diff);
    let mut a1: int16x8_t = vld1q_s16(src_diff.offset(src_stride));
    let mut a2: int16x8_t = vld1q_s16(src_diff.offset(2 * src_stride));
    let mut a3: int16x8_t = vld1q_s16(src_diff.offset(3 * src_stride));
    let mut a4: int16x8_t = vld1q_s16(src_diff.offset(4 * src_stride));
    let mut a5: int16x8_t = vld1q_s16(src_diff.offset(5 * src_stride));
    let mut a6: int16x8_t = vld1q_s16(src_diff.offset(6 * src_stride));
    let mut a7: int16x8_t = vld1q_s16(src_diff.offset(7 * src_stride));

    hadamard8x8_one_pass(&mut a0, &mut a1, &mut a2, &mut a3, &mut a4, &mut a5, &mut a6, &mut a7);

    transpose_elems_inplace_s16_8x8(&mut a0, &mut a1, &mut a2, &mut a3, &mut a4, &mut a5, &mut a6,
                                    &mut a7);

    hadamard8x8_one_pass(&mut a0, &mut a1, &mut a2, &mut a3, &mut a4, &mut a5, &mut a6, &mut a7);

    // Skip the second transpose because it is not required.

    vst1q_s16(coeff.offset(0), a0);
    vst1q_s16(coeff.offset(8), a1);
    vst1q_s16(coeff.offset(16), a2);
    vst1q_s16(coeff.offset(24), a3);
    vst1q_s16(coeff.offset(32), a4);
    vst1q_s16(coeff.offset(40), a5);
    vst1q_s16(coeff.offset(48), a6);
    vst1q_s16(coeff.offset(56), a7);
}

#[no_mangle]
pub unsafe extern "C-unwind" fn aom_hadamard_lp_8x8_dual_neon(src_diff: *const i16,
                                                              src_stride: isize, coeff: *mut i16) {
    for i in 0..2isize {
        aom_hadamard_lp_8x8_neon(src_diff.offset(i * 8), src_stride, coeff.offset(i * 64));
    }
}

#[no_mangle]
pub unsafe extern "C-unwind" fn aom_hadamard_lp_16x16_neon(src_diff: *const i16,
                                                           src_stride: isize,
                                                           mut coeff: *mut i16) {
    /* Rearrange 16x16 to 8x32 and remove stride.
     * Top left first. */
    aom_hadamard_lp_8x8_neon(src_diff.offset(0 + 0 * src_stride), src_stride, coeff.offset(0));
    /* Top right. */
    aom_hadamard_lp_8x8_neon(src_diff.offset(8 + 0 * src_stride), src_stride, coeff.offset(64));
    /* Bottom left. */
    aom_hadamard_lp_8x8_neon(src_diff.offset(0 + 8 * src_stride), src_stride, coeff.offset(128));
    /* Bottom right. */
    aom_hadamard_lp_8x8_neon(src_diff.offset(8 + 8 * src_stride), src_stride, coeff.offset(192));

    let mut i = 0;
    while i < 64 {
        let a0: int16x8_t = vld1q_s16(coeff.offset(0));
        let a1: int16x8_t = vld1q_s16(coeff.offset(64));
        let a2: int16x8_t = vld1q_s16(coeff.offset(128));
        let a3: int16x8_t = vld1q_s16(coeff.offset(192));

        let b0: int16x8_t = vhaddq_s16(a0, a1);
        let b1: int16x8_t = vhsubq_s16(a0, a1);
        let b2: int16x8_t = vhaddq_s16(a2, a3);
        let b3: int16x8_t = vhsubq_s16(a2, a3);

        let c0: int16x8_t = vaddq_s16(b0, b2);
        let c1: int16x8_t = vaddq_s16(b1, b3);
        let c2: int16x8_t = vsubq_s16(b0, b2);
        let c3: int16x8_t = vsubq_s16(b1, b3);

        vst1q_s16(coeff.offset(0), c0);
        vst1q_s16(coeff.offset(64), c1);
        vst1q_s16(coeff.offset(128), c2);
        vst1q_s16(coeff.offset(192), c3);

        coeff = coeff.offset(8);
        i += 8;
    }
}

#[no_mangle]
pub unsafe extern "C-unwind" fn aom_hadamard_16x16_neon(src_diff: *const i16, src_stride: isize,
                                                        mut coeff: *mut tran_low_t) {
    /* Rearrange 16x16 to 8x32 and remove stride.
     * Top left first. */
    aom_hadamard_8x8_neon(src_diff.offset(0 + 0 * src_stride), src_stride, coeff.offset(0));
    /* Top right. */
    aom_hadamard_8x8_neon(src_diff.offset(8 + 0 * src_stride), src_stride, coeff.offset(64));
    /* Bottom left. */
    aom_hadamard_8x8_neon(src_diff.offset(0 + 8 * src_stride), src_stride, coeff.offset(128));
    /* Bottom right. */
    aom_hadamard_8x8_neon(src_diff.offset(8 + 8 * src_stride), src_stride, coeff.offset(192));

    // Each iteration of the loop operates on entire rows (16 samples each)
    // because we need to swap the second and third quarters of every row in the
    // output to match AVX2 output (i.e., aom_hadamard_16x16_avx2). See the for
    // loop at the end of aom_hadamard_16x16_c.
    let mut i = 0;
    while i < 64 {
        let a00: int32x4_t = vld1q_s32(coeff.offset(0));
        let a01: int32x4_t = vld1q_s32(coeff.offset(64));
        let a02: int32x4_t = vld1q_s32(coeff.offset(128));
        let a03: int32x4_t = vld1q_s32(coeff.offset(192));

        let b00: int32x4_t = vhaddq_s32(a00, a01);
        let b01: int32x4_t = vhsubq_s32(a00, a01);
        let b02: int32x4_t = vhaddq_s32(a02, a03);
        let b03: int32x4_t = vhsubq_s32(a02, a03);

        let c00: int32x4_t = vaddq_s32(b00, b02);
        let c01: int32x4_t = vaddq_s32(b01, b03);
        let c02: int32x4_t = vsubq_s32(b00, b02);
        let c03: int32x4_t = vsubq_s32(b01, b03);

        let a10: int32x4_t = vld1q_s32(coeff.offset(4 + 0));
        let a11: int32x4_t = vld1q_s32(coeff.offset(4 + 64));
        let a12: int32x4_t = vld1q_s32(coeff.offset(4 + 128));
        let a13: int32x4_t = vld1q_s32(coeff.offset(4 + 192));

        let b10: int32x4_t = vhaddq_s32(a10, a11);
        let b11: int32x4_t = vhsubq_s32(a10, a11);
        let b12: int32x4_t = vhaddq_s32(a12, a13);
        let b13: int32x4_t = vhsubq_s32(a12, a13);

        let c10: int32x4_t = vaddq_s32(b10, b12);
        let c11: int32x4_t = vaddq_s32(b11, b13);
        let c12: int32x4_t = vsubq_s32(b10, b12);
        let c13: int32x4_t = vsubq_s32(b11, b13);

        let a20: int32x4_t = vld1q_s32(coeff.offset(8 + 0));
        let a21: int32x4_t = vld1q_s32(coeff.offset(8 + 64));
        let a22: int32x4_t = vld1q_s32(coeff.offset(8 + 128));
        let a23: int32x4_t = vld1q_s32(coeff.offset(8 + 192));

        let b20: int32x4_t = vhaddq_s32(a20, a21);
        let b21: int32x4_t = vhsubq_s32(a20, a21);
        let b22: int32x4_t = vhaddq_s32(a22, a23);
        let b23: int32x4_t = vhsubq_s32(a22, a23);

        let c20: int32x4_t = vaddq_s32(b20, b22);
        let c21: int32x4_t = vaddq_s32(b21, b23);
        let c22: int32x4_t = vsubq_s32(b20, b22);
        let c23: int32x4_t = vsubq_s32(b21, b23);

        let a30: int32x4_t = vld1q_s32(coeff.offset(12 + 0));
        let a31: int32x4_t = vld1q_s32(coeff.offset(12 + 64));
        let a32: int32x4_t = vld1q_s32(coeff.offset(12 + 128));
        let a33: int32x4_t = vld1q_s32(coeff.offset(12 + 192));

        let b30: int32x4_t = vhaddq_s32(a30, a31);
        let b31: int32x4_t = vhsubq_s32(a30, a31);
        let b32: int32x4_t = vhaddq_s32(a32, a33);
        let b33: int32x4_t = vhsubq_s32(a32, a33);

        let c30: int32x4_t = vaddq_s32(b30, b32);
        let c31: int32x4_t = vaddq_s32(b31, b33);
        let c32: int32x4_t = vsubq_s32(b30, b32);
        let c33: int32x4_t = vsubq_s32(b31, b33);

        vst1q_s32(coeff.offset(0 + 0), c00);
        vst1q_s32(coeff.offset(0 + 4), c20);
        vst1q_s32(coeff.offset(0 + 8), c10);
        vst1q_s32(coeff.offset(0 + 12), c30);

        vst1q_s32(coeff.offset(64 + 0), c01);
        vst1q_s32(coeff.offset(64 + 4), c21);
        vst1q_s32(coeff.offset(64 + 8), c11);
        vst1q_s32(coeff.offset(64 + 12), c31);

        vst1q_s32(coeff.offset(128 + 0), c02);
        vst1q_s32(coeff.offset(128 + 4), c22);
        vst1q_s32(coeff.offset(128 + 8), c12);
        vst1q_s32(coeff.offset(128 + 12), c32);

        vst1q_s32(coeff.offset(192 + 0), c03);
        vst1q_s32(coeff.offset(192 + 4), c23);
        vst1q_s32(coeff.offset(192 + 8), c13);
        vst1q_s32(coeff.offset(192 + 12), c33);

        coeff = coeff.offset(16);
        i += 16;
    }
}

#[no_mangle]
pub unsafe extern "C-unwind" fn aom_hadamard_32x32_neon(src_diff: *const i16, src_stride: isize,
                                                        mut coeff: *mut tran_low_t) {
    /* Top left first. */
    aom_hadamard_16x16_neon(src_diff.offset(0 + 0 * src_stride), src_stride, coeff.offset(0));
    /* Top right. */
    aom_hadamard_16x16_neon(src_diff.offset(16 + 0 * src_stride), src_stride, coeff.offset(256));
    /* Bottom left. */
    aom_hadamard_16x16_neon(src_diff.offset(0 + 16 * src_stride), src_stride, coeff.offset(512));
    /* Bottom right. */
    aom_hadamard_16x16_neon(src_diff.offset(16 + 16 * src_stride), src_stride, coeff.offset(768));

    let mut i = 0;
    while i < 256 {
        let a0: int32x4_t = vld1q_s32(coeff);
        let a1: int32x4_t = vld1q_s32(coeff.offset(256));
        let a2: int32x4_t = vld1q_s32(coeff.offset(512));
        let a3: int32x4_t = vld1q_s32(coeff.offset(768));

        let b0: int32x4_t = vshrq_n_s32::<2>(vaddq_s32(a0, a1));
        let b1: int32x4_t = vshrq_n_s32::<2>(vsubq_s32(a0, a1));
        let b2: int32x4_t = vshrq_n_s32::<2>(vaddq_s32(a2, a3));
        let b3: int32x4_t = vshrq_n_s32::<2>(vsubq_s32(a2, a3));

        let c0: int32x4_t = vaddq_s32(b0, b2);
        let c1: int32x4_t = vaddq_s32(b1, b3);
        let c2: int32x4_t = vsubq_s32(b0, b2);
        let c3: int32x4_t = vsubq_s32(b1, b3);

        vst1q_s32(coeff.offset(0), c0);
        vst1q_s32(coeff.offset(256), c1);
        vst1q_s32(coeff.offset(512), c2);
        vst1q_s32(coeff.offset(768), c3);

        coeff = coeff.offset(4);
        i += 4;
    }
}
