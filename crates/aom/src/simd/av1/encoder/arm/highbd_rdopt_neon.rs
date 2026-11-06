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

// Translated by hand from upstream/aom/av1/encoder/arm/highbd_rdopt_neon.c
// (ADR 0056).
use core::arch::aarch64::*;

use crate::simd::aom_dsp__arm__sum_neon::*;
use crate::simd::types::*;

#[no_mangle]
pub unsafe extern "C-unwind" fn av1_highbd_block_error_neon(
    mut coeff: *const tran_low_t,
    mut dqcoeff: *const tran_low_t,
    mut block_size: isize,
    ssz: *mut i64,
    bd: i32,
) -> i64 {
    let mut err_u64 = vdupq_n_u64(0);
    let mut ssz_s64 = vdupq_n_s64(0);

    let shift: i32 = 2 * (bd - 8);
    let rounding: i32 = (1 << shift) >> 1;

    debug_assert!(block_size >= 16);
    debug_assert!(block_size % 16 == 0);

    loop {
        let c = vld1q_s32(coeff);
        let d = vld1q_s32(dqcoeff);

        let diff = vreinterpretq_u32_s32(vabdq_s32(c, d));

        err_u64 = vmlal_u32(err_u64, vget_low_u32(diff), vget_low_u32(diff));
        err_u64 = vmlal_u32(err_u64, vget_high_u32(diff), vget_high_u32(diff));

        ssz_s64 = vmlal_s32(ssz_s64, vget_low_s32(c), vget_low_s32(c));
        ssz_s64 = vmlal_s32(ssz_s64, vget_high_s32(c), vget_high_s32(c));

        coeff = coeff.add(4);
        dqcoeff = dqcoeff.add(4);
        block_size -= 4;
        if block_size == 0 {
            break;
        }
    }

    *ssz = (horizontal_add_s64x2(ssz_s64) + rounding as i64) >> shift;
    ((horizontal_add_u64x2(err_u64) as i64) + rounding as i64) >> shift
}
