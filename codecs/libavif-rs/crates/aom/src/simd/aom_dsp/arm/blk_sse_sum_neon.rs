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

// Translated by hand from upstream/aom/aom_dsp/arm/blk_sse_sum_neon.c
// (ADR 0056). Intrinsic sequences, helper structure, and names follow the
// original.
use core::arch::aarch64::*;

use crate::simd::aom_dsp__arm__sum_neon::{horizontal_add_s32x4, horizontal_add_s64x2,
                                          horizontal_long_add_s32x4};

#[inline(always)]
unsafe fn get_blk_sse_sum_4xh_neon(mut data: *const i16, stride: i32, bh: i32, x_sum: *mut i32,
                                   x2_sum: *mut i64) {
    let mut i: i32 = bh;
    let mut sum: int32x4_t = vdupq_n_s32(0);
    let mut sse: int32x4_t = vdupq_n_s32(0);

    loop {
        let d: int16x8_t = vcombine_s16(vld1_s16(data), vld1_s16(data.offset(stride as isize)));

        sum = vpadalq_s16(sum, d);

        sse = vmlal_s16(sse, vget_low_s16(d), vget_low_s16(d));
        sse = vmlal_s16(sse, vget_high_s16(d), vget_high_s16(d));

        data = data.offset((2 * stride) as isize);
        i -= 2;
        if i == 0 {
            break;
        }
    }

    *x_sum = horizontal_add_s32x4(sum);
    *x2_sum = horizontal_long_add_s32x4(sse);
}

#[inline(always)]
unsafe fn get_blk_sse_sum_8xh_neon(mut data: *const i16, stride: i32, bh: i32, x_sum: *mut i32,
                                   x2_sum: *mut i64) {
    let mut i: i32 = bh;
    let mut sum: int32x4_t = vdupq_n_s32(0);
    let mut sse: int32x4_t = vdupq_n_s32(0);

    // Input is 12-bit wide, so we can add up to 127 squared elements in a signed
    // 32-bits element. Since we're accumulating into an int32x4_t and the maximum
    // value for bh is 32, we don't have to worry about sse overflowing.

    loop {
        let d: int16x8_t = vld1q_s16(data);

        sum = vpadalq_s16(sum, d);

        sse = vmlal_s16(sse, vget_low_s16(d), vget_low_s16(d));
        sse = vmlal_s16(sse, vget_high_s16(d), vget_high_s16(d));

        data = data.offset(stride as isize);
        i -= 1;
        if i == 0 {
            break;
        }
    }

    *x_sum = horizontal_add_s32x4(sum);
    *x2_sum = horizontal_long_add_s32x4(sse);
}

#[inline(always)]
unsafe fn get_blk_sse_sum_large_neon(mut data: *const i16, stride: i32, bw: i32, bh: i32,
                                     x_sum: *mut i32, x2_sum: *mut i64) {
    let mut sum: int32x4_t = vdupq_n_s32(0);
    let mut sse: int64x2_t = vdupq_n_s64(0);

    // Input is 12-bit wide, so we can add up to 127 squared elements in a signed
    // 32-bits element. Since we're accumulating into an int32x4_t vector that
    // means we can process up to (127*4)/bw rows before we need to widen to
    // 64 bits.

    let i_limit: i32 = (127 * 4) / bw;
    let mut i_tmp: i32 = if bh > i_limit { i_limit } else { bh };

    let mut i: i32 = 0;
    loop {
        let mut sse_s32: int32x4_t = vdupq_n_s32(0);
        loop {
            let mut j: i32 = bw;
            let mut data_ptr: *const i16 = data;
            loop {
                let d: int16x8_t = vld1q_s16(data_ptr);

                sum = vpadalq_s16(sum, d);

                sse_s32 = vmlal_s16(sse_s32, vget_low_s16(d), vget_low_s16(d));
                sse_s32 = vmlal_s16(sse_s32, vget_high_s16(d), vget_high_s16(d));

                data_ptr = data_ptr.add(8);
                j -= 8;
                if j == 0 {
                    break;
                }
            }

            data = data.offset(stride as isize);
            i += 1;
            if !(i < i_tmp && i < bh) {
                break;
            }
        }

        sse = vpadalq_s32(sse, sse_s32);
        i_tmp += i_limit;
        if !(i < bh) {
            break;
        }
    }

    *x_sum = horizontal_add_s32x4(sum);
    *x2_sum = horizontal_add_s64x2(sse);
}

#[no_mangle]
pub unsafe extern "C-unwind" fn aom_get_blk_sse_sum_neon(data: *const i16, stride: i32, bw: i32,
                                                         bh: i32, x_sum: *mut i32,
                                                         x2_sum: *mut i64) {
    if bw == 4 {
        get_blk_sse_sum_4xh_neon(data, stride, bh, x_sum, x2_sum);
    } else if bw == 8 {
        get_blk_sse_sum_8xh_neon(data, stride, bh, x_sum, x2_sum);
    } else {
        debug_assert!(bw % 8 == 0);
        get_blk_sse_sum_large_neon(data, stride, bw, bh, x_sum, x2_sum);
    }
}
