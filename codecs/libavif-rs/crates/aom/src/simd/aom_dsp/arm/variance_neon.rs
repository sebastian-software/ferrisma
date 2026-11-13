/*
 * Copyright (c) 2016, Alliance for Open Media. All rights reserved.
 *
 * This source code is subject to the terms of the BSD 2 Clause License and
 * the Alliance for Open Media Patent License 1.0. If the BSD 2 Clause License
 * was not distributed with this source code in the LICENSE file, you can
 * obtain it at www.aomedia.org/license/software. If the Alliance for Open
 * Media Patent License 1.0 was not distributed with this source code in the
 * PATENTS file, you can obtain it at www.aomedia.org/license/patent.
 */

// Translated by hand from upstream/aom/aom_dsp/arm/variance_neon.c (ADR 0056).
// Only the still-image kernels used by the CDEF search (pickcdef.c) are
// translated: mse_accumulate_u16_u8_8x2, mse_wxh_16bit,
// aom_mse_wxh_16bit_neon and aom_mse_16xh_16bit_neon. The variance, MSE and
// sub-pixel kernels of the same file serve inter prediction and motion search
// and are out of scope until open question O2 is decided. Intrinsic
// sequences, helper structure, and names follow the original.
use core::arch::aarch64::*;

use crate::simd::aom_dsp__arm__mem_neon::{load_unaligned_u16_4x2, load_unaligned_u8_4x2};
use crate::simd::aom_dsp__arm__sum_neon::horizontal_add_u64x2;

#[inline(always)]
unsafe fn mse_accumulate_u16_u8_8x2(sum: uint64x2_t, s0: uint16x8_t, s1: uint16x8_t,
                                    d0: uint8x8_t, d1: uint8x8_t) -> uint64x2_t {
    let e0: int16x8_t = vreinterpretq_s16_u16(vsubw_u8(s0, d0));
    let e1: int16x8_t = vreinterpretq_s16_u16(vsubw_u8(s1, d1));

    let mut mse: int32x4_t = vmull_s16(vget_low_s16(e0), vget_low_s16(e0));
    mse = vmlal_s16(mse, vget_high_s16(e0), vget_high_s16(e0));
    mse = vmlal_s16(mse, vget_low_s16(e1), vget_low_s16(e1));
    mse = vmlal_s16(mse, vget_high_s16(e1), vget_high_s16(e1));

    vpadalq_u32(sum, vreinterpretq_u32_s32(mse))
}

unsafe fn mse_wxh_16bit(mut dst: *mut u8, dstride: i32, mut src: *const u16, sstride: i32, w: i32,
                        mut h: i32) -> uint64x2_t {
    debug_assert!((w == 8 || w == 4) && (h == 8 || h == 4));

    let mut sum: uint64x2_t = vdupq_n_u64(0);

    if w == 8 {
        loop {
            let d0: uint8x8_t = vld1_u8(dst.offset((0 * dstride) as isize));
            let d1: uint8x8_t = vld1_u8(dst.offset((1 * dstride) as isize));
            let s0: uint16x8_t = vld1q_u16(src.offset((0 * sstride) as isize));
            let s1: uint16x8_t = vld1q_u16(src.offset((1 * sstride) as isize));

            sum = mse_accumulate_u16_u8_8x2(sum, s0, s1, d0, d1);

            dst = dst.offset((2 * dstride) as isize);
            src = src.offset((2 * sstride) as isize);
            h -= 2;
            if h == 0 {
                break;
            }
        }
    } else {
        loop {
            let d0: uint8x8_t =
                load_unaligned_u8_4x2(dst.offset((0 * dstride) as isize), dstride as isize);
            let d1: uint8x8_t =
                load_unaligned_u8_4x2(dst.offset((2 * dstride) as isize), dstride as isize);
            let s0: uint16x8_t =
                load_unaligned_u16_4x2(src.offset((0 * sstride) as isize), sstride as isize);
            let s1: uint16x8_t =
                load_unaligned_u16_4x2(src.offset((2 * sstride) as isize), sstride as isize);

            sum = mse_accumulate_u16_u8_8x2(sum, s0, s1, d0, d1);

            dst = dst.offset((4 * dstride) as isize);
            src = src.offset((4 * sstride) as isize);
            h -= 4;
            if h == 0 {
                break;
            }
        }
    }

    sum
}

// Computes mse for a given block size. This function gets called for specific
// block sizes, which are 8x8, 8x4, 4x8 and 4x4.
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_mse_wxh_16bit_neon(dst: *mut u8, dstride: i32, src: *mut u16,
                                                       sstride: i32, w: i32, h: i32) -> u64 {
    horizontal_add_u64x2(mse_wxh_16bit(dst, dstride, src, sstride, w, h))
}

#[no_mangle]
pub unsafe extern "C-unwind" fn aom_mse_16xh_16bit_neon(mut dst: *mut u8, dstride: i32,
                                                        mut src: *mut u16, w: i32, h: i32) -> u64 {
    let mut sum: uint64x2_t = vdupq_n_u64(0);

    let mut num_blks: i32 = 16 / w;
    loop {
        sum = vaddq_u64(sum, mse_wxh_16bit(dst, dstride, src, w, w, h));
        dst = dst.offset(w as isize);
        src = src.offset((w * h) as isize);
        num_blks -= 1;
        if num_blks == 0 {
            break;
        }
    }

    horizontal_add_u64x2(sum)
}
