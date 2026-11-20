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
// (ADR 0056). The kernels the still-image encoder dispatches are translated:
// the block variances (aom_highbd_{8,10,12}_variance{W}x{H}_neon), the block
// MSEs (aom_highbd_{8,10,12}_mse{W}x{H}_neon) and
// aom_mse_wxh_16bit_highbd_neon, with the static helpers they call. The
// sub-pixel, averaging and OBMC kernels of the same file serve motion search
// and are out of scope until open question O2 is decided. Intrinsic
// sequences, helper structure, and names follow the original; the
// function-generating C macros become macro_rules! that receive the generated
// function and helper names explicitly, invoked in the original order.
// ROUND_POWER_OF_TWO (aom_ports/mem.h) is written out:
// `((value) + ((1 << (n)) >> 1)) >> (n)` in the operand's type.
use core::arch::aarch64::*;

use crate::simd::aom_dsp__arm__mem_neon::load_unaligned_u16_4x2;
use crate::simd::aom_dsp__arm__sum_neon::{horizontal_add_s16x8, horizontal_add_s32x4, horizontal_add_s64x2,
                                          horizontal_add_u32x4, horizontal_add_u64x2,
                                          horizontal_long_add_u32x4};

// aom_dsp/aom_dsp_common.h: CONVERT_TO_SHORTPTR(x) ((uint16_t *)(((uintptr_t)(x)) << 1))
#[allow(non_snake_case)]
#[inline(always)]
fn CONVERT_TO_SHORTPTR(x: *const u8) -> *mut u16 {
    ((x as usize) << 1) as *mut u16
}

// Process a block of width 4 two rows at a time.
#[inline(always)]
unsafe fn highbd_variance_4xh_neon(mut src_ptr: *const u16, src_stride: i32, mut ref_ptr: *const u16,
                                   ref_stride: i32, h: i32, sse: *mut u64, sum: *mut i64) {
    let mut sum_s16: int16x8_t = vdupq_n_s16(0);
    let mut sse_s32: int32x4_t = vdupq_n_s32(0);

    let mut i: i32 = h;
    loop {
        let s: uint16x8_t = load_unaligned_u16_4x2(src_ptr, src_stride as isize);
        let r: uint16x8_t = load_unaligned_u16_4x2(ref_ptr, ref_stride as isize);

        let diff: int16x8_t = vreinterpretq_s16_u16(vsubq_u16(s, r));
        sum_s16 = vaddq_s16(sum_s16, diff);

        sse_s32 = vmlal_s16(sse_s32, vget_low_s16(diff), vget_low_s16(diff));
        sse_s32 = vmlal_s16(sse_s32, vget_high_s16(diff), vget_high_s16(diff));

        src_ptr = src_ptr.wrapping_offset((2 * src_stride) as isize);
        ref_ptr = ref_ptr.wrapping_offset((2 * ref_stride) as isize);
        i -= 2;
        if i == 0 {
            break;
        }
    }

    *sum = horizontal_add_s16x8(sum_s16) as i64;
    *sse = horizontal_add_s32x4(sse_s32) as u64;
}

// For 8-bit and 10-bit data, since we're using two int32x4 accumulators, all
// block sizes can be processed in 32-bit elements (1023*1023*128*32 =
// 4286582784 for a 128x128 block).
#[inline(always)]
unsafe fn highbd_variance_large_neon(mut src_ptr: *const u16, src_stride: i32, mut ref_ptr: *const u16,
                                     ref_stride: i32, w: i32, h: i32, sse: *mut u64, sum: *mut i64) {
    let mut sum_s32: int32x4_t = vdupq_n_s32(0);
    let mut sse_s32: [int32x4_t; 2] = [vdupq_n_s32(0), vdupq_n_s32(0)];

    let mut i: i32 = h;
    loop {
        let mut j: i32 = 0;
        loop {
            let s: uint16x8_t = vld1q_u16(src_ptr.offset(j as isize));
            let r: uint16x8_t = vld1q_u16(ref_ptr.offset(j as isize));

            let diff: int16x8_t = vreinterpretq_s16_u16(vsubq_u16(s, r));
            sum_s32 = vpadalq_s16(sum_s32, diff);

            sse_s32[0] = vmlal_s16(sse_s32[0], vget_low_s16(diff), vget_low_s16(diff));
            sse_s32[1] = vmlal_s16(sse_s32[1], vget_high_s16(diff), vget_high_s16(diff));

            j += 8;
            if j >= w {
                break;
            }
        }
        src_ptr = src_ptr.wrapping_offset(src_stride as isize);
        ref_ptr = ref_ptr.wrapping_offset(ref_stride as isize);
        i -= 1;
        if i == 0 {
            break;
        }
    }

    *sum = horizontal_add_s32x4(sum_s32) as i64;
    *sse = horizontal_long_add_u32x4(vaddq_u32(vreinterpretq_u32_s32(sse_s32[0]),
                                               vreinterpretq_u32_s32(sse_s32[1])));
}

#[inline(always)]
unsafe fn highbd_variance_8xh_neon(src: *const u16, src_stride: i32, ref_: *const u16, ref_stride: i32, h: i32,
                             sse: *mut u64, sum: *mut i64) {
    highbd_variance_large_neon(src, src_stride, ref_, ref_stride, 8, h, sse, sum);
}

#[inline(always)]
unsafe fn highbd_variance_16xh_neon(src: *const u16, src_stride: i32, ref_: *const u16, ref_stride: i32, h: i32,
                             sse: *mut u64, sum: *mut i64) {
    highbd_variance_large_neon(src, src_stride, ref_, ref_stride, 16, h, sse, sum);
}

#[inline(always)]
unsafe fn highbd_variance_32xh_neon(src: *const u16, src_stride: i32, ref_: *const u16, ref_stride: i32, h: i32,
                             sse: *mut u64, sum: *mut i64) {
    highbd_variance_large_neon(src, src_stride, ref_, ref_stride, 32, h, sse, sum);
}

#[inline(always)]
unsafe fn highbd_variance_64xh_neon(src: *const u16, src_stride: i32, ref_: *const u16, ref_stride: i32, h: i32,
                             sse: *mut u64, sum: *mut i64) {
    highbd_variance_large_neon(src, src_stride, ref_, ref_stride, 64, h, sse, sum);
}

#[inline(always)]
unsafe fn highbd_variance_128xh_neon(src: *const u16, src_stride: i32, ref_: *const u16, ref_stride: i32, h: i32,
                             sse: *mut u64, sum: *mut i64) {
    highbd_variance_large_neon(src, src_stride, ref_, ref_stride, 128, h, sse, sum);
}

// For 12-bit data, we can only accumulate up to 128 elements in the sum of
// squares (4095*4095*128 = 2146435200), and because we're using two int32x4
// accumulators, we can only process up to 32 32-element rows (32*32/8 = 128)
// or 16 64-element rows before we have to accumulate into 64-bit elements.
// Therefore blocks of size 32x64, 64x32, 64x64, 64x128, 128x64, 128x128 are
// processed in a different helper function.

// Process a block of any size where the width is divisible by 8, with
// accumulation into 64-bit elements.
#[inline(always)]
unsafe fn highbd_variance_xlarge_neon(mut src_ptr: *const u16, src_stride: i32, mut ref_ptr: *const u16,
                                      ref_stride: i32, w: i32, h: i32, h_limit: i32, sse: *mut u64,
                                      sum: *mut i64) {
    let mut sum_s32: int32x4_t = vdupq_n_s32(0);
    let mut sse_s64: int64x2_t = vdupq_n_s64(0);

    // 'h_limit' is the number of 'w'-width rows we can process before our 32-bit
    // accumulator overflows. After hitting this limit we accumulate into 64-bit
    // elements.
    let mut h_tmp: i32 = if h > h_limit { h_limit } else { h };

    let mut i: i32 = 0;
    loop {
        let mut sse_s32: [int32x4_t; 2] = [vdupq_n_s32(0), vdupq_n_s32(0)];
        loop {
            let mut j: i32 = 0;
            loop {
                let s0: uint16x8_t = vld1q_u16(src_ptr.offset(j as isize));
                let r0: uint16x8_t = vld1q_u16(ref_ptr.offset(j as isize));

                let diff: int16x8_t = vreinterpretq_s16_u16(vsubq_u16(s0, r0));
                sum_s32 = vpadalq_s16(sum_s32, diff);

                sse_s32[0] = vmlal_s16(sse_s32[0], vget_low_s16(diff), vget_low_s16(diff));
                sse_s32[1] = vmlal_s16(sse_s32[1], vget_high_s16(diff), vget_high_s16(diff));

                j += 8;
                if j >= w {
                    break;
                }
            }
            src_ptr = src_ptr.wrapping_offset(src_stride as isize);
            ref_ptr = ref_ptr.wrapping_offset(ref_stride as isize);
            i += 1;
            if i >= h_tmp {
                break;
            }
        }

        sse_s64 = vpadalq_s32(sse_s64, sse_s32[0]);
        sse_s64 = vpadalq_s32(sse_s64, sse_s32[1]);
        h_tmp += h_limit;
        if i >= h {
            break;
        }
    }

    *sum = horizontal_add_s32x4(sum_s32) as i64;
    *sse = horizontal_add_s64x2(sse_s64) as u64;
}

#[inline(always)]
unsafe fn highbd_variance_32xh_xlarge_neon(src: *const u16, src_stride: i32, ref_: *const u16, ref_stride: i32,
                                    h: i32, sse: *mut u64, sum: *mut i64) {
    highbd_variance_xlarge_neon(src, src_stride, ref_, ref_stride, 32, h, 32, sse, sum);
}

#[inline(always)]
unsafe fn highbd_variance_64xh_xlarge_neon(src: *const u16, src_stride: i32, ref_: *const u16, ref_stride: i32,
                                    h: i32, sse: *mut u64, sum: *mut i64) {
    highbd_variance_xlarge_neon(src, src_stride, ref_, ref_stride, 64, h, 16, sse, sum);
}

#[inline(always)]
unsafe fn highbd_variance_128xh_xlarge_neon(src: *const u16, src_stride: i32, ref_: *const u16, ref_stride: i32,
                                    h: i32, sse: *mut u64, sum: *mut i64) {
    highbd_variance_xlarge_neon(src, src_stride, ref_, ref_stride, 128, h, 8, sse, sum);
}

// #define HBD_VARIANCE_WXH_8_NEON(w, h)
macro_rules! hbd_variance_wxh_8_neon {
    ($name:ident, $helper:ident, $w:literal, $h:literal) => {
        #[no_mangle]
        pub unsafe extern "C-unwind" fn $name(src_ptr: *const u8, src_stride: i32, ref_ptr: *const u8,
                                              ref_stride: i32, sse: *mut u32) -> u32 {
            let sum: i32;
            let mut sse_long: u64 = 0;
            let mut sum_long: i64 = 0;
            let src: *mut u16 = CONVERT_TO_SHORTPTR(src_ptr);
            let ref_: *mut u16 = CONVERT_TO_SHORTPTR(ref_ptr);
            $helper(src, src_stride, ref_, ref_stride, $h, &mut sse_long, &mut sum_long);
            *sse = sse_long as u32;
            sum = sum_long as i32;
            *sse - ((sum as i64 * sum as i64) / ($w * $h)) as u32
        }
    };
}

// #define HBD_VARIANCE_WXH_10_NEON(w, h)
macro_rules! hbd_variance_wxh_10_neon {
    ($name:ident, $helper:ident, $w:literal, $h:literal) => {
        #[no_mangle]
        pub unsafe extern "C-unwind" fn $name(src_ptr: *const u8, src_stride: i32, ref_ptr: *const u8,
                                              ref_stride: i32, sse: *mut u32) -> u32 {
            let sum: i32;
            let var: i64;
            let mut sse_long: u64 = 0;
            let mut sum_long: i64 = 0;
            let src: *mut u16 = CONVERT_TO_SHORTPTR(src_ptr);
            let ref_: *mut u16 = CONVERT_TO_SHORTPTR(ref_ptr);
            $helper(src, src_stride, ref_, ref_stride, $h, &mut sse_long, &mut sum_long);
            *sse = ((sse_long + ((1 << 4) >> 1)) >> 4) as u32;
            sum = ((sum_long + ((1 << 2) >> 1)) >> 2) as i32;
            var = *sse as i64 - ((sum as i64 * sum as i64) / ($w * $h));
            if var >= 0 { var as u32 } else { 0 }
        }
    };
}

// #define HBD_VARIANCE_WXH_12_NEON(w, h) and HBD_VARIANCE_WXH_12_XLARGE_NEON(w, h),
// which differ only in the helper they call.
macro_rules! hbd_variance_wxh_12_neon {
    ($name:ident, $helper:ident, $w:literal, $h:literal) => {
        #[no_mangle]
        pub unsafe extern "C-unwind" fn $name(src_ptr: *const u8, src_stride: i32, ref_ptr: *const u8,
                                              ref_stride: i32, sse: *mut u32) -> u32 {
            let sum: i32;
            let var: i64;
            let mut sse_long: u64 = 0;
            let mut sum_long: i64 = 0;
            let src: *mut u16 = CONVERT_TO_SHORTPTR(src_ptr);
            let ref_: *mut u16 = CONVERT_TO_SHORTPTR(ref_ptr);
            $helper(src, src_stride, ref_, ref_stride, $h, &mut sse_long, &mut sum_long);
            *sse = ((sse_long + ((1 << 8) >> 1)) >> 8) as u32;
            sum = ((sum_long + ((1 << 4) >> 1)) >> 4) as i32;
            var = *sse as i64 - ((sum as i64 * sum as i64) / ($w * $h));
            if var >= 0 { var as u32 } else { 0 }
        }
    };
}

// 8-bit
hbd_variance_wxh_8_neon!(aom_highbd_8_variance4x4_neon, highbd_variance_4xh_neon, 4, 4);
hbd_variance_wxh_8_neon!(aom_highbd_8_variance4x8_neon, highbd_variance_4xh_neon, 4, 8);
hbd_variance_wxh_8_neon!(aom_highbd_8_variance8x4_neon, highbd_variance_8xh_neon, 8, 4);
hbd_variance_wxh_8_neon!(aom_highbd_8_variance8x8_neon, highbd_variance_8xh_neon, 8, 8);
hbd_variance_wxh_8_neon!(aom_highbd_8_variance8x16_neon, highbd_variance_8xh_neon, 8, 16);
hbd_variance_wxh_8_neon!(aom_highbd_8_variance16x8_neon, highbd_variance_16xh_neon, 16, 8);
hbd_variance_wxh_8_neon!(aom_highbd_8_variance16x16_neon, highbd_variance_16xh_neon, 16, 16);
hbd_variance_wxh_8_neon!(aom_highbd_8_variance16x32_neon, highbd_variance_16xh_neon, 16, 32);
hbd_variance_wxh_8_neon!(aom_highbd_8_variance32x16_neon, highbd_variance_32xh_neon, 32, 16);
hbd_variance_wxh_8_neon!(aom_highbd_8_variance32x32_neon, highbd_variance_32xh_neon, 32, 32);
hbd_variance_wxh_8_neon!(aom_highbd_8_variance32x64_neon, highbd_variance_32xh_neon, 32, 64);
hbd_variance_wxh_8_neon!(aom_highbd_8_variance64x32_neon, highbd_variance_64xh_neon, 64, 32);
hbd_variance_wxh_8_neon!(aom_highbd_8_variance64x64_neon, highbd_variance_64xh_neon, 64, 64);
hbd_variance_wxh_8_neon!(aom_highbd_8_variance64x128_neon, highbd_variance_64xh_neon, 64, 128);
hbd_variance_wxh_8_neon!(aom_highbd_8_variance128x64_neon, highbd_variance_128xh_neon, 128, 64);
hbd_variance_wxh_8_neon!(aom_highbd_8_variance128x128_neon, highbd_variance_128xh_neon, 128, 128);

// 10-bit
hbd_variance_wxh_10_neon!(aom_highbd_10_variance4x4_neon, highbd_variance_4xh_neon, 4, 4);
hbd_variance_wxh_10_neon!(aom_highbd_10_variance4x8_neon, highbd_variance_4xh_neon, 4, 8);
hbd_variance_wxh_10_neon!(aom_highbd_10_variance8x4_neon, highbd_variance_8xh_neon, 8, 4);
hbd_variance_wxh_10_neon!(aom_highbd_10_variance8x8_neon, highbd_variance_8xh_neon, 8, 8);
hbd_variance_wxh_10_neon!(aom_highbd_10_variance8x16_neon, highbd_variance_8xh_neon, 8, 16);
hbd_variance_wxh_10_neon!(aom_highbd_10_variance16x8_neon, highbd_variance_16xh_neon, 16, 8);
hbd_variance_wxh_10_neon!(aom_highbd_10_variance16x16_neon, highbd_variance_16xh_neon, 16, 16);
hbd_variance_wxh_10_neon!(aom_highbd_10_variance16x32_neon, highbd_variance_16xh_neon, 16, 32);
hbd_variance_wxh_10_neon!(aom_highbd_10_variance32x16_neon, highbd_variance_32xh_neon, 32, 16);
hbd_variance_wxh_10_neon!(aom_highbd_10_variance32x32_neon, highbd_variance_32xh_neon, 32, 32);
hbd_variance_wxh_10_neon!(aom_highbd_10_variance32x64_neon, highbd_variance_32xh_neon, 32, 64);
hbd_variance_wxh_10_neon!(aom_highbd_10_variance64x32_neon, highbd_variance_64xh_neon, 64, 32);
hbd_variance_wxh_10_neon!(aom_highbd_10_variance64x64_neon, highbd_variance_64xh_neon, 64, 64);
hbd_variance_wxh_10_neon!(aom_highbd_10_variance64x128_neon, highbd_variance_64xh_neon, 64, 128);
hbd_variance_wxh_10_neon!(aom_highbd_10_variance128x64_neon, highbd_variance_128xh_neon, 128, 64);
hbd_variance_wxh_10_neon!(aom_highbd_10_variance128x128_neon, highbd_variance_128xh_neon, 128, 128);

// 12-bit
hbd_variance_wxh_12_neon!(aom_highbd_12_variance4x4_neon, highbd_variance_4xh_neon, 4, 4);
hbd_variance_wxh_12_neon!(aom_highbd_12_variance4x8_neon, highbd_variance_4xh_neon, 4, 8);
hbd_variance_wxh_12_neon!(aom_highbd_12_variance8x4_neon, highbd_variance_8xh_neon, 8, 4);
hbd_variance_wxh_12_neon!(aom_highbd_12_variance8x8_neon, highbd_variance_8xh_neon, 8, 8);
hbd_variance_wxh_12_neon!(aom_highbd_12_variance8x16_neon, highbd_variance_8xh_neon, 8, 16);
hbd_variance_wxh_12_neon!(aom_highbd_12_variance16x8_neon, highbd_variance_16xh_neon, 16, 8);
hbd_variance_wxh_12_neon!(aom_highbd_12_variance16x16_neon, highbd_variance_16xh_neon, 16, 16);
hbd_variance_wxh_12_neon!(aom_highbd_12_variance16x32_neon, highbd_variance_16xh_neon, 16, 32);
hbd_variance_wxh_12_neon!(aom_highbd_12_variance32x16_neon, highbd_variance_32xh_neon, 32, 16);
hbd_variance_wxh_12_neon!(aom_highbd_12_variance32x32_neon, highbd_variance_32xh_neon, 32, 32);
hbd_variance_wxh_12_neon!(aom_highbd_12_variance32x64_neon, highbd_variance_32xh_xlarge_neon, 32, 64);
hbd_variance_wxh_12_neon!(aom_highbd_12_variance64x32_neon, highbd_variance_64xh_xlarge_neon, 64, 32);
hbd_variance_wxh_12_neon!(aom_highbd_12_variance64x64_neon, highbd_variance_64xh_xlarge_neon, 64, 64);
hbd_variance_wxh_12_neon!(aom_highbd_12_variance64x128_neon, highbd_variance_64xh_xlarge_neon, 64, 128);
hbd_variance_wxh_12_neon!(aom_highbd_12_variance128x64_neon, highbd_variance_128xh_xlarge_neon, 128, 64);
hbd_variance_wxh_12_neon!(aom_highbd_12_variance128x128_neon, highbd_variance_128xh_xlarge_neon, 128, 128);

// #if !CONFIG_REALTIME_ONLY (both translated profiles)
// 8-bit
hbd_variance_wxh_8_neon!(aom_highbd_8_variance4x16_neon, highbd_variance_4xh_neon, 4, 16);
hbd_variance_wxh_8_neon!(aom_highbd_8_variance8x32_neon, highbd_variance_8xh_neon, 8, 32);
hbd_variance_wxh_8_neon!(aom_highbd_8_variance16x4_neon, highbd_variance_16xh_neon, 16, 4);
hbd_variance_wxh_8_neon!(aom_highbd_8_variance16x64_neon, highbd_variance_16xh_neon, 16, 64);
hbd_variance_wxh_8_neon!(aom_highbd_8_variance32x8_neon, highbd_variance_32xh_neon, 32, 8);
hbd_variance_wxh_8_neon!(aom_highbd_8_variance64x16_neon, highbd_variance_64xh_neon, 64, 16);

// 10-bit
hbd_variance_wxh_10_neon!(aom_highbd_10_variance4x16_neon, highbd_variance_4xh_neon, 4, 16);
hbd_variance_wxh_10_neon!(aom_highbd_10_variance8x32_neon, highbd_variance_8xh_neon, 8, 32);
hbd_variance_wxh_10_neon!(aom_highbd_10_variance16x4_neon, highbd_variance_16xh_neon, 16, 4);
hbd_variance_wxh_10_neon!(aom_highbd_10_variance16x64_neon, highbd_variance_16xh_neon, 16, 64);
hbd_variance_wxh_10_neon!(aom_highbd_10_variance32x8_neon, highbd_variance_32xh_neon, 32, 8);
hbd_variance_wxh_10_neon!(aom_highbd_10_variance64x16_neon, highbd_variance_64xh_neon, 64, 16);

// 12-bit
hbd_variance_wxh_12_neon!(aom_highbd_12_variance4x16_neon, highbd_variance_4xh_neon, 4, 16);
hbd_variance_wxh_12_neon!(aom_highbd_12_variance8x32_neon, highbd_variance_8xh_neon, 8, 32);
hbd_variance_wxh_12_neon!(aom_highbd_12_variance16x4_neon, highbd_variance_16xh_neon, 16, 4);
hbd_variance_wxh_12_neon!(aom_highbd_12_variance16x64_neon, highbd_variance_16xh_neon, 16, 64);
hbd_variance_wxh_12_neon!(aom_highbd_12_variance32x8_neon, highbd_variance_32xh_neon, 32, 8);
hbd_variance_wxh_12_neon!(aom_highbd_12_variance64x16_neon, highbd_variance_64xh_neon, 64, 16);

#[inline(always)]
unsafe fn highbd_mse_wxh_neon(mut src_ptr: *const u16, src_stride: i32, mut ref_ptr: *const u16,
                              ref_stride: i32, w: i32, h: i32, sse: *mut u32) -> u32 {
    let mut sse_u32: [uint32x4_t; 2] = [vdupq_n_u32(0), vdupq_n_u32(0)];

    let mut i: i32 = h;
    loop {
        let mut j: i32 = 0;
        loop {
            let s: uint16x8_t = vld1q_u16(src_ptr.offset(j as isize));
            let r: uint16x8_t = vld1q_u16(ref_ptr.offset(j as isize));

            let diff: uint16x8_t = vabdq_u16(s, r);

            sse_u32[0] = vmlal_u16(sse_u32[0], vget_low_u16(diff), vget_low_u16(diff));
            sse_u32[1] = vmlal_u16(sse_u32[1], vget_high_u16(diff), vget_high_u16(diff));

            j += 8;
            if j >= w {
                break;
            }
        }

        src_ptr = src_ptr.wrapping_offset(src_stride as isize);
        ref_ptr = ref_ptr.wrapping_offset(ref_stride as isize);
        i -= 1;
        if i == 0 {
            break;
        }
    }

    *sse = horizontal_add_u32x4(vaddq_u32(sse_u32[0], sse_u32[1]));
    *sse
}

// #define HIGHBD_MSE_WXH_NEON(w, h): the 8-, 10- and 12-bit functions of one size.
macro_rules! highbd_mse_wxh_neon {
    ($name8:ident, $name10:ident, $name12:ident, $w:literal, $h:literal) => {
        #[no_mangle]
        pub unsafe extern "C-unwind" fn $name8(src_ptr: *const u8, src_stride: i32, ref_ptr: *const u8,
                                               ref_stride: i32, sse: *mut u32) -> u32 {
            let src: *mut u16 = CONVERT_TO_SHORTPTR(src_ptr);
            let ref_: *mut u16 = CONVERT_TO_SHORTPTR(ref_ptr);
            highbd_mse_wxh_neon(src, src_stride, ref_, ref_stride, $w, $h, sse);
            *sse
        }

        #[no_mangle]
        pub unsafe extern "C-unwind" fn $name10(src_ptr: *const u8, src_stride: i32, ref_ptr: *const u8,
                                                ref_stride: i32, sse: *mut u32) -> u32 {
            let src: *mut u16 = CONVERT_TO_SHORTPTR(src_ptr);
            let ref_: *mut u16 = CONVERT_TO_SHORTPTR(ref_ptr);
            highbd_mse_wxh_neon(src, src_stride, ref_, ref_stride, $w, $h, sse);
            *sse = (*sse + ((1 << 4) >> 1)) >> 4;
            *sse
        }

        #[no_mangle]
        pub unsafe extern "C-unwind" fn $name12(src_ptr: *const u8, src_stride: i32, ref_ptr: *const u8,
                                                ref_stride: i32, sse: *mut u32) -> u32 {
            let src: *mut u16 = CONVERT_TO_SHORTPTR(src_ptr);
            let ref_: *mut u16 = CONVERT_TO_SHORTPTR(ref_ptr);
            highbd_mse_wxh_neon(src, src_stride, ref_, ref_stride, $w, $h, sse);
            *sse = (*sse + ((1 << 8) >> 1)) >> 8;
            *sse
        }
    };
}

highbd_mse_wxh_neon!(aom_highbd_8_mse16x16_neon, aom_highbd_10_mse16x16_neon,
                     aom_highbd_12_mse16x16_neon, 16, 16);
highbd_mse_wxh_neon!(aom_highbd_8_mse16x8_neon, aom_highbd_10_mse16x8_neon,
                     aom_highbd_12_mse16x8_neon, 16, 8);
highbd_mse_wxh_neon!(aom_highbd_8_mse8x16_neon, aom_highbd_10_mse8x16_neon,
                     aom_highbd_12_mse8x16_neon, 8, 16);
highbd_mse_wxh_neon!(aom_highbd_8_mse8x8_neon, aom_highbd_10_mse8x8_neon,
                     aom_highbd_12_mse8x8_neon, 8, 8);

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
