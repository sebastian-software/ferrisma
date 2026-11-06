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

// Translated by hand from upstream/aom/av1/encoder/arm/av1_error_neon.c
// (ADR 0056).
use core::arch::aarch64::*;

use crate::simd::aom_dsp__arm__mem_neon::*;
use crate::simd::aom_dsp__arm__sum_neon::*;
use crate::simd::types::*;

#[no_mangle]
pub unsafe extern "C-unwind" fn av1_block_error_neon(
    mut coeff: *const tran_low_t,
    mut dqcoeff: *const tran_low_t,
    mut block_size: isize,
    ssz: *mut i64,
) -> i64 {
    let mut err_u64 = vdupq_n_u64(0);
    let mut ssz_s64 = vdupq_n_s64(0);

    debug_assert!(block_size >= 16);
    debug_assert!(block_size % 16 == 0);

    loop {
        let c0 = load_tran_low_to_s16q(coeff);
        let c1 = load_tran_low_to_s16q(coeff.add(8));
        let d0 = load_tran_low_to_s16q(dqcoeff);
        let d1 = load_tran_low_to_s16q(dqcoeff.add(8));

        let diff0 = vreinterpretq_u16_s16(vabdq_s16(c0, d0));
        let diff1 = vreinterpretq_u16_s16(vabdq_s16(c1, d1));

        // By operating on unsigned integers we can store up to 4 squared diff in a
        // 32-bit element before having to widen to 64 bits.
        let mut err = vmull_u16(vget_low_u16(diff0), vget_low_u16(diff0));
        err = vmlal_u16(err, vget_high_u16(diff0), vget_high_u16(diff0));
        err = vmlal_u16(err, vget_low_u16(diff1), vget_low_u16(diff1));
        err = vmlal_u16(err, vget_high_u16(diff1), vget_high_u16(diff1));
        err_u64 = vpadalq_u32(err_u64, err);

        // We can't do the same here as we're operating on signed integers, so we
        // can only accumulate 2 squares.
        let mut ssz0 = vmull_s16(vget_low_s16(c0), vget_low_s16(c0));
        ssz0 = vmlal_s16(ssz0, vget_high_s16(c0), vget_high_s16(c0));
        ssz_s64 = vpadalq_s32(ssz_s64, ssz0);

        let mut ssz1 = vmull_s16(vget_low_s16(c1), vget_low_s16(c1));
        ssz1 = vmlal_s16(ssz1, vget_high_s16(c1), vget_high_s16(c1));
        ssz_s64 = vpadalq_s32(ssz_s64, ssz1);

        coeff = coeff.add(16);
        dqcoeff = dqcoeff.add(16);
        block_size -= 16;
        if block_size == 0 {
            break;
        }
    }

    *ssz = horizontal_add_s64x2(ssz_s64);
    horizontal_add_u64x2(err_u64) as i64
}

#[no_mangle]
pub unsafe extern "C-unwind" fn av1_block_error_lp_neon(
    mut coeff: *const i16,
    mut dqcoeff: *const i16,
    mut block_size: isize,
) -> i64 {
    let mut err_u64 = vdupq_n_u64(0);

    debug_assert!(block_size >= 16);
    debug_assert!(block_size % 16 == 0);

    loop {
        let c0 = vld1q_s16(coeff);
        let c1 = vld1q_s16(coeff.add(8));
        let d0 = vld1q_s16(dqcoeff);
        let d1 = vld1q_s16(dqcoeff.add(8));

        let diff0 = vreinterpretq_u16_s16(vabdq_s16(c0, d0));
        let diff1 = vreinterpretq_u16_s16(vabdq_s16(c1, d1));

        // By operating on unsigned integers we can store up to 4 squared diff in a
        // 32-bit element before having to widen to 64 bits.
        let mut err = vmull_u16(vget_low_u16(diff0), vget_low_u16(diff0));
        err = vmlal_u16(err, vget_high_u16(diff0), vget_high_u16(diff0));
        err = vmlal_u16(err, vget_low_u16(diff1), vget_low_u16(diff1));
        err = vmlal_u16(err, vget_high_u16(diff1), vget_high_u16(diff1));
        err_u64 = vpadalq_u32(err_u64, err);

        coeff = coeff.add(16);
        dqcoeff = dqcoeff.add(16);
        block_size -= 16;
        if block_size == 0 {
            break;
        }
    }

    horizontal_add_u64x2(err_u64) as i64
}
