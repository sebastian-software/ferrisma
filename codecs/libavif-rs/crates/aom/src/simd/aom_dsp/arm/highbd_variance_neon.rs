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

// Translated by hand from upstream/aom/aom_dsp/arm/highbd_variance_neon.c
// (ADR 0056). Only the still-image kernel used by the CDEF search
// (pickcdef.c) is translated: mse_accumulate_u16_8x2 and
// aom_mse_wxh_16bit_highbd_neon. The variance, MSE and sub-pixel kernels of
// the same file serve inter prediction and motion search and are out of scope
// until open question O2 is decided. Intrinsic sequences, helper structure,
// and names follow the original.
use core::arch::aarch64::*;

use crate::simd::aom_dsp__arm__mem_neon::load_unaligned_u16_4x2;
use crate::simd::aom_dsp__arm__sum_neon::horizontal_add_u64x2;

#[inline(always)]
unsafe fn mse_accumulate_u16_8x2(sum: uint64x2_t, s0: uint16x8_t, s1: uint16x8_t, d0: uint16x8_t,
                                 d1: uint16x8_t) -> uint64x2_t {
    let e0: uint16x8_t = vabdq_u16(s0, d0);
    let e1: uint16x8_t = vabdq_u16(s1, d1);

    let mut mse: uint32x4_t = vmull_u16(vget_low_u16(e0), vget_low_u16(e0));
    mse = vmlal_u16(mse, vget_high_u16(e0), vget_high_u16(e0));
    mse = vmlal_u16(mse, vget_low_u16(e1), vget_low_u16(e1));
    mse = vmlal_u16(mse, vget_high_u16(e1), vget_high_u16(e1));

    vpadalq_u32(sum, mse)
}

#[no_mangle]
pub unsafe extern "C-unwind" fn aom_mse_wxh_16bit_highbd_neon(mut dst: *mut u16, dstride: i32,
                                                              mut src: *mut u16, sstride: i32,
                                                              w: i32, mut h: i32) -> u64 {
    debug_assert!((w == 8 || w == 4) && (h == 8 || h == 4));

    let mut sum: uint64x2_t = vdupq_n_u64(0);

    if w == 8 {
        loop {
            let d0: uint16x8_t = vld1q_u16(dst.offset((0 * dstride) as isize));
            let d1: uint16x8_t = vld1q_u16(dst.offset((1 * dstride) as isize));
            let s0: uint16x8_t = vld1q_u16(src.offset((0 * sstride) as isize));
            let s1: uint16x8_t = vld1q_u16(src.offset((1 * sstride) as isize));

            sum = mse_accumulate_u16_8x2(sum, s0, s1, d0, d1);

            dst = dst.offset((2 * dstride) as isize);
            src = src.offset((2 * sstride) as isize);
            h -= 2;
            if h == 0 {
                break;
            }
        }
    } else {
        // w == 4
        loop {
            let d0: uint16x8_t =
                load_unaligned_u16_4x2(dst.offset((0 * dstride) as isize), dstride as isize);
            let d1: uint16x8_t =
                load_unaligned_u16_4x2(dst.offset((2 * dstride) as isize), dstride as isize);
            let s0: uint16x8_t =
                load_unaligned_u16_4x2(src.offset((0 * sstride) as isize), sstride as isize);
            let s1: uint16x8_t =
                load_unaligned_u16_4x2(src.offset((2 * sstride) as isize), sstride as isize);

            sum = mse_accumulate_u16_8x2(sum, s0, s1, d0, d1);

            dst = dst.offset((4 * dstride) as isize);
            src = src.offset((4 * sstride) as isize);
            h -= 4;
            if h == 0 {
                break;
            }
        }
    }

    horizontal_add_u64x2(sum)
}
