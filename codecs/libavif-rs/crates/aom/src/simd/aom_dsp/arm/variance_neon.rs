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
// The kernels the still-image encoder dispatches are translated: the block
// variances (aom_variance{W}x{H}_neon), aom_get_var_sse_sum_8x8_quad_neon,
// aom_get_var_sse_sum_16x16_dual_neon, the block MSEs (aom_mse{W}x{H}_neon),
// aom_mse_wxh_16bit_neon and aom_mse_16xh_16bit_neon, with the static helpers
// they call. The sub-pixel and averaging kernels of the same file serve
// motion search and are out of scope until open question O2 is decided.
// Intrinsic sequences, helper structure, and names follow the original; the
// function-generating C macros (VARIANCE_WXH_NEON, MSE_WXH_NEON) become
// macro_rules! that receive the generated function and helper names
// explicitly, invoked in the original order.
use core::arch::aarch64::*;

use crate::simd::aom_dsp__arm__mem_neon::{load_unaligned_u16_4x2, load_unaligned_u8, load_unaligned_u8_4x2};
use crate::simd::aom_dsp__arm__sum_neon::{horizontal_add_s16x8, horizontal_add_s32x4, horizontal_add_u32x4,
                                          horizontal_add_u64x2};

#[inline(always)]
unsafe fn variance_4xh_neon(mut src: *const u8, src_stride: i32, mut ref_: *const u8, ref_stride: i32,
                            mut h: i32, sse: *mut u32, sum: *mut i32) {
    let mut sum_s16: int16x8_t = vdupq_n_s16(0);
    let mut sse_s32: int32x4_t = vdupq_n_s32(0);

    // Number of rows we can process before 'sum_s16' overflows:
    // 32767 / 255 ~= 128, but we use an 8-wide accumulator; so 256 4-wide rows.
    debug_assert!(h <= 256);

    loop {
        let s: uint8x8_t = load_unaligned_u8(src, src_stride as isize);
        let r: uint8x8_t = load_unaligned_u8(ref_, ref_stride as isize);
        let diff: int16x8_t = vreinterpretq_s16_u16(vsubl_u8(s, r));

        sum_s16 = vaddq_s16(sum_s16, diff);

        sse_s32 = vmlal_s16(sse_s32, vget_low_s16(diff), vget_low_s16(diff));
        sse_s32 = vmlal_s16(sse_s32, vget_high_s16(diff), vget_high_s16(diff));

        src = src.wrapping_offset((2 * src_stride) as isize);
        ref_ = ref_.wrapping_offset((2 * ref_stride) as isize);
        h -= 2;
        if h == 0 {
            break;
        }
    }

    *sum = horizontal_add_s16x8(sum_s16);
    *sse = horizontal_add_s32x4(sse_s32) as u32;
}

#[inline(always)]
unsafe fn variance_8xh_neon(mut src: *const u8, src_stride: i32, mut ref_: *const u8, ref_stride: i32,
                            mut h: i32, sse: *mut u32, sum: *mut i32) {
    let mut sum_s16: int16x8_t = vdupq_n_s16(0);
    let mut sse_s32: [int32x4_t; 2] = [vdupq_n_s32(0), vdupq_n_s32(0)];

    // Number of rows we can process before 'sum_s16' overflows:
    // 32767 / 255 ~= 128
    debug_assert!(h <= 128);

    loop {
        let s: uint8x8_t = vld1_u8(src);
        let r: uint8x8_t = vld1_u8(ref_);
        let diff: int16x8_t = vreinterpretq_s16_u16(vsubl_u8(s, r));

        sum_s16 = vaddq_s16(sum_s16, diff);

        sse_s32[0] = vmlal_s16(sse_s32[0], vget_low_s16(diff), vget_low_s16(diff));
        sse_s32[1] = vmlal_s16(sse_s32[1], vget_high_s16(diff), vget_high_s16(diff));

        src = src.wrapping_offset(src_stride as isize);
        ref_ = ref_.wrapping_offset(ref_stride as isize);
        h -= 1;
        if h == 0 {
            break;
        }
    }

    *sum = horizontal_add_s16x8(sum_s16);
    *sse = horizontal_add_s32x4(vaddq_s32(sse_s32[0], sse_s32[1])) as u32;
}

#[inline(always)]
unsafe fn variance_16xh_neon(mut src: *const u8, src_stride: i32, mut ref_: *const u8, ref_stride: i32,
                             mut h: i32, sse: *mut u32, sum: *mut i32) {
    let mut sum_s16: [int16x8_t; 2] = [vdupq_n_s16(0), vdupq_n_s16(0)];
    let mut sse_s32: [int32x4_t; 2] = [vdupq_n_s32(0), vdupq_n_s32(0)];

    // Number of rows we can process before 'sum_s16' accumulators overflow:
    // 32767 / 255 ~= 128, so 128 16-wide rows.
    debug_assert!(h <= 128);

    loop {
        let s: uint8x16_t = vld1q_u8(src);
        let r: uint8x16_t = vld1q_u8(ref_);

        let diff_l: int16x8_t = vreinterpretq_s16_u16(vsubl_u8(vget_low_u8(s), vget_low_u8(r)));
        let diff_h: int16x8_t = vreinterpretq_s16_u16(vsubl_u8(vget_high_u8(s), vget_high_u8(r)));

        sum_s16[0] = vaddq_s16(sum_s16[0], diff_l);
        sum_s16[1] = vaddq_s16(sum_s16[1], diff_h);

        sse_s32[0] = vmlal_s16(sse_s32[0], vget_low_s16(diff_l), vget_low_s16(diff_l));
        sse_s32[1] = vmlal_s16(sse_s32[1], vget_high_s16(diff_l), vget_high_s16(diff_l));
        sse_s32[0] = vmlal_s16(sse_s32[0], vget_low_s16(diff_h), vget_low_s16(diff_h));
        sse_s32[1] = vmlal_s16(sse_s32[1], vget_high_s16(diff_h), vget_high_s16(diff_h));

        src = src.wrapping_offset(src_stride as isize);
        ref_ = ref_.wrapping_offset(ref_stride as isize);
        h -= 1;
        if h == 0 {
            break;
        }
    }

    *sum = horizontal_add_s16x8(vaddq_s16(sum_s16[0], sum_s16[1]));
    *sse = horizontal_add_s32x4(vaddq_s32(sse_s32[0], sse_s32[1])) as u32;
}

#[inline(always)]
unsafe fn variance_large_neon(mut src: *const u8, src_stride: i32, mut ref_: *const u8, ref_stride: i32,
                              w: i32, h: i32, h_limit: i32, sse: *mut u32, sum: *mut i32) {
    let mut sum_s32: int32x4_t = vdupq_n_s32(0);
    let mut sse_s32: [int32x4_t; 2] = [vdupq_n_s32(0), vdupq_n_s32(0)];

    // 'h_limit' is the number of 'w'-width rows we can process before our 16-bit
    // accumulator overflows. After hitting this limit we accumulate into 32-bit
    // elements.
    let mut h_tmp: i32 = if h > h_limit { h_limit } else { h };

    let mut i: i32 = 0;
    loop {
        let mut sum_s16: [int16x8_t; 2] = [vdupq_n_s16(0), vdupq_n_s16(0)];
        loop {
            let mut j: i32 = 0;
            loop {
                let s: uint8x16_t = vld1q_u8(src.offset(j as isize));
                let r: uint8x16_t = vld1q_u8(ref_.offset(j as isize));

                let diff_l: int16x8_t = vreinterpretq_s16_u16(vsubl_u8(vget_low_u8(s), vget_low_u8(r)));
                let diff_h: int16x8_t = vreinterpretq_s16_u16(vsubl_u8(vget_high_u8(s), vget_high_u8(r)));

                sum_s16[0] = vaddq_s16(sum_s16[0], diff_l);
                sum_s16[1] = vaddq_s16(sum_s16[1], diff_h);

                sse_s32[0] = vmlal_s16(sse_s32[0], vget_low_s16(diff_l), vget_low_s16(diff_l));
                sse_s32[1] = vmlal_s16(sse_s32[1], vget_high_s16(diff_l), vget_high_s16(diff_l));
                sse_s32[0] = vmlal_s16(sse_s32[0], vget_low_s16(diff_h), vget_low_s16(diff_h));
                sse_s32[1] = vmlal_s16(sse_s32[1], vget_high_s16(diff_h), vget_high_s16(diff_h));

                j += 16;
                if j >= w {
                    break;
                }
            }

            src = src.wrapping_offset(src_stride as isize);
            ref_ = ref_.wrapping_offset(ref_stride as isize);
            i += 1;
            if i >= h_tmp {
                break;
            }
        }

        sum_s32 = vpadalq_s16(sum_s32, sum_s16[0]);
        sum_s32 = vpadalq_s16(sum_s32, sum_s16[1]);

        h_tmp += h_limit;
        if i >= h {
            break;
        }
    }

    *sum = horizontal_add_s32x4(sum_s32);
    *sse = horizontal_add_s32x4(vaddq_s32(sse_s32[0], sse_s32[1])) as u32;
}

#[inline(always)]
unsafe fn variance_32xh_neon(src: *const u8, src_stride: i32, ref_: *const u8, ref_stride: i32, h: i32,
                             sse: *mut u32, sum: *mut i32) {
    variance_large_neon(src, src_stride, ref_, ref_stride, 32, h, 64, sse, sum);
}

#[inline(always)]
unsafe fn variance_64xh_neon(src: *const u8, src_stride: i32, ref_: *const u8, ref_stride: i32, h: i32,
                             sse: *mut u32, sum: *mut i32) {
    variance_large_neon(src, src_stride, ref_, ref_stride, 64, h, 32, sse, sum);
}

#[inline(always)]
unsafe fn variance_128xh_neon(src: *const u8, src_stride: i32, ref_: *const u8, ref_stride: i32, h: i32,
                              sse: *mut u32, sum: *mut i32) {
    variance_large_neon(src, src_stride, ref_, ref_stride, 128, h, 16, sse, sum);
}

// #define VARIANCE_WXH_NEON(w, h, shift)
macro_rules! variance_wxh_neon {
    ($name:ident, $helper:ident, $h:literal, $shift:literal) => {
        #[no_mangle]
        pub unsafe extern "C-unwind" fn $name(src: *const u8, src_stride: i32, ref_: *const u8,
                                              ref_stride: i32, sse: *mut u32) -> u32 {
            let mut sum: i32 = 0;
            $helper(src, src_stride, ref_, ref_stride, $h, sse, &mut sum);
            *sse - ((sum as i64 * sum as i64) >> $shift) as u32
        }
    };
}

variance_wxh_neon!(aom_variance4x4_neon, variance_4xh_neon, 4, 4);
variance_wxh_neon!(aom_variance4x8_neon, variance_4xh_neon, 8, 5);

variance_wxh_neon!(aom_variance8x4_neon, variance_8xh_neon, 4, 5);
variance_wxh_neon!(aom_variance8x8_neon, variance_8xh_neon, 8, 6);
variance_wxh_neon!(aom_variance8x16_neon, variance_8xh_neon, 16, 7);

variance_wxh_neon!(aom_variance16x8_neon, variance_16xh_neon, 8, 7);
variance_wxh_neon!(aom_variance16x16_neon, variance_16xh_neon, 16, 8);
variance_wxh_neon!(aom_variance16x32_neon, variance_16xh_neon, 32, 9);

variance_wxh_neon!(aom_variance32x16_neon, variance_32xh_neon, 16, 9);
variance_wxh_neon!(aom_variance32x32_neon, variance_32xh_neon, 32, 10);
variance_wxh_neon!(aom_variance32x64_neon, variance_32xh_neon, 64, 11);

variance_wxh_neon!(aom_variance64x32_neon, variance_64xh_neon, 32, 11);
variance_wxh_neon!(aom_variance64x64_neon, variance_64xh_neon, 64, 12);
variance_wxh_neon!(aom_variance64x128_neon, variance_64xh_neon, 128, 13);

variance_wxh_neon!(aom_variance128x64_neon, variance_128xh_neon, 64, 13);
variance_wxh_neon!(aom_variance128x128_neon, variance_128xh_neon, 128, 14);

// #if !CONFIG_REALTIME_ONLY (both translated profiles)
variance_wxh_neon!(aom_variance4x16_neon, variance_4xh_neon, 16, 6);
variance_wxh_neon!(aom_variance8x32_neon, variance_8xh_neon, 32, 8);
variance_wxh_neon!(aom_variance16x4_neon, variance_16xh_neon, 4, 6);
variance_wxh_neon!(aom_variance16x64_neon, variance_16xh_neon, 64, 10);
variance_wxh_neon!(aom_variance32x8_neon, variance_32xh_neon, 8, 8);
variance_wxh_neon!(aom_variance64x16_neon, variance_64xh_neon, 16, 10);

// TODO(yunqingwang): Perform variance of two/four 8x8 blocks similar to that of
// AVX2. Also, implement the NEON for variance computation present in this
// function.
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_get_var_sse_sum_8x8_quad_neon(src: *const u8, src_stride: i32,
                                                                  ref_: *const u8, ref_stride: i32,
                                                                  sse8x8: *mut u32, sum8x8: *mut i32,
                                                                  tot_sse: *mut u32, tot_sum: *mut i32,
                                                                  var8x8: *mut u32) {
    // Loop over four 8x8 blocks. Process one 8x32 block.
    for k in 0..4 {
        variance_8xh_neon(src.offset((k * 8) as isize), src_stride, ref_.offset((k * 8) as isize),
                          ref_stride, 8, sse8x8.offset(k as isize), sum8x8.offset(k as isize));
    }

    *tot_sse += *sse8x8.add(0) + *sse8x8.add(1) + *sse8x8.add(2) + *sse8x8.add(3);
    *tot_sum += *sum8x8.add(0) + *sum8x8.add(1) + *sum8x8.add(2) + *sum8x8.add(3);
    for i in 0..4 {
        *var8x8.add(i) =
            *sse8x8.add(i) - ((*sum8x8.add(i) as i64 * *sum8x8.add(i) as i64) >> 6) as u32;
    }
}

#[no_mangle]
pub unsafe extern "C-unwind" fn aom_get_var_sse_sum_16x16_dual_neon(src: *const u8, src_stride: i32,
                                                                    ref_: *const u8, ref_stride: i32,
                                                                    sse16x16: *mut u32,
                                                                    tot_sse: *mut u32, tot_sum: *mut i32,
                                                                    var16x16: *mut u32) {
    let mut sum16x16: [i32; 2] = [0; 2];
    // Loop over two 16x16 blocks. Process one 16x32 block.
    for k in 0..2 {
        variance_16xh_neon(src.offset((k * 16) as isize), src_stride, ref_.offset((k * 16) as isize),
                           ref_stride, 16, sse16x16.offset(k as isize), &mut sum16x16[k as usize]);
    }

    *tot_sse += *sse16x16.add(0) + *sse16x16.add(1);
    *tot_sum += sum16x16[0] + sum16x16[1];
    for i in 0..2 {
        *var16x16.add(i) =
            *sse16x16.add(i) - ((sum16x16[i] as i64 * sum16x16[i] as i64) >> 8) as u32;
    }
}

#[inline(always)]
unsafe fn mse8xh_neon(mut src: *const u8, src_stride: i32, mut ref_: *const u8, ref_stride: i32,
                      sse: *mut u32, mut h: i32) -> u32 {
    let mut s: [uint8x8_t; 2] = [vdup_n_u8(0); 2];
    let mut r: [uint8x8_t; 2] = [vdup_n_u8(0); 2];
    let mut diff_lo: [int16x4_t; 2] = [vdup_n_s16(0); 2];
    let mut diff_hi: [int16x4_t; 2] = [vdup_n_s16(0); 2];
    let mut diff: [uint16x8_t; 2] = [vdupq_n_u16(0); 2];
    let mut sse_s32: [int32x4_t; 2] = [vdupq_n_s32(0), vdupq_n_s32(0)];

    loop {
        s[0] = vld1_u8(src);
        src = src.wrapping_offset(src_stride as isize);
        s[1] = vld1_u8(src);
        src = src.wrapping_offset(src_stride as isize);
        r[0] = vld1_u8(ref_);
        ref_ = ref_.wrapping_offset(ref_stride as isize);
        r[1] = vld1_u8(ref_);
        ref_ = ref_.wrapping_offset(ref_stride as isize);

        diff[0] = vsubl_u8(s[0], r[0]);
        diff[1] = vsubl_u8(s[1], r[1]);

        diff_lo[0] = vreinterpret_s16_u16(vget_low_u16(diff[0]));
        diff_lo[1] = vreinterpret_s16_u16(vget_low_u16(diff[1]));
        sse_s32[0] = vmlal_s16(sse_s32[0], diff_lo[0], diff_lo[0]);
        sse_s32[1] = vmlal_s16(sse_s32[1], diff_lo[1], diff_lo[1]);

        diff_hi[0] = vreinterpret_s16_u16(vget_high_u16(diff[0]));
        diff_hi[1] = vreinterpret_s16_u16(vget_high_u16(diff[1]));
        sse_s32[0] = vmlal_s16(sse_s32[0], diff_hi[0], diff_hi[0]);
        sse_s32[1] = vmlal_s16(sse_s32[1], diff_hi[1], diff_hi[1]);

        h -= 2;
        if h == 0 {
            break;
        }
    }

    sse_s32[0] = vaddq_s32(sse_s32[0], sse_s32[1]);

    *sse = horizontal_add_u32x4(vreinterpretq_u32_s32(sse_s32[0]));
    horizontal_add_u32x4(vreinterpretq_u32_s32(sse_s32[0]))
}

#[inline(always)]
unsafe fn mse16xh_neon(mut src: *const u8, src_stride: i32, mut ref_: *const u8, ref_stride: i32,
                       sse: *mut u32, mut h: i32) -> u32 {
    let mut s: [uint8x16_t; 2] = [vdupq_n_u8(0); 2];
    let mut r: [uint8x16_t; 2] = [vdupq_n_u8(0); 2];
    let mut diff_lo: [int16x4_t; 4] = [vdup_n_s16(0); 4];
    let mut diff_hi: [int16x4_t; 4] = [vdup_n_s16(0); 4];
    let mut diff: [uint16x8_t; 4] = [vdupq_n_u16(0); 4];
    let mut sse_s32: [int32x4_t; 4] = [vdupq_n_s32(0), vdupq_n_s32(0), vdupq_n_s32(0), vdupq_n_s32(0)];

    loop {
        s[0] = vld1q_u8(src);
        src = src.wrapping_offset(src_stride as isize);
        s[1] = vld1q_u8(src);
        src = src.wrapping_offset(src_stride as isize);
        r[0] = vld1q_u8(ref_);
        ref_ = ref_.wrapping_offset(ref_stride as isize);
        r[1] = vld1q_u8(ref_);
        ref_ = ref_.wrapping_offset(ref_stride as isize);

        diff[0] = vsubl_u8(vget_low_u8(s[0]), vget_low_u8(r[0]));
        diff[1] = vsubl_u8(vget_high_u8(s[0]), vget_high_u8(r[0]));
        diff[2] = vsubl_u8(vget_low_u8(s[1]), vget_low_u8(r[1]));
        diff[3] = vsubl_u8(vget_high_u8(s[1]), vget_high_u8(r[1]));

        diff_lo[0] = vreinterpret_s16_u16(vget_low_u16(diff[0]));
        diff_lo[1] = vreinterpret_s16_u16(vget_low_u16(diff[1]));
        sse_s32[0] = vmlal_s16(sse_s32[0], diff_lo[0], diff_lo[0]);
        sse_s32[1] = vmlal_s16(sse_s32[1], diff_lo[1], diff_lo[1]);

        diff_lo[2] = vreinterpret_s16_u16(vget_low_u16(diff[2]));
        diff_lo[3] = vreinterpret_s16_u16(vget_low_u16(diff[3]));
        sse_s32[2] = vmlal_s16(sse_s32[2], diff_lo[2], diff_lo[2]);
        sse_s32[3] = vmlal_s16(sse_s32[3], diff_lo[3], diff_lo[3]);

        diff_hi[0] = vreinterpret_s16_u16(vget_high_u16(diff[0]));
        diff_hi[1] = vreinterpret_s16_u16(vget_high_u16(diff[1]));
        sse_s32[0] = vmlal_s16(sse_s32[0], diff_hi[0], diff_hi[0]);
        sse_s32[1] = vmlal_s16(sse_s32[1], diff_hi[1], diff_hi[1]);

        diff_hi[2] = vreinterpret_s16_u16(vget_high_u16(diff[2]));
        diff_hi[3] = vreinterpret_s16_u16(vget_high_u16(diff[3]));
        sse_s32[2] = vmlal_s16(sse_s32[2], diff_hi[2], diff_hi[2]);
        sse_s32[3] = vmlal_s16(sse_s32[3], diff_hi[3], diff_hi[3]);

        h -= 2;
        if h == 0 {
            break;
        }
    }

    sse_s32[0] = vaddq_s32(sse_s32[0], sse_s32[1]);
    sse_s32[2] = vaddq_s32(sse_s32[2], sse_s32[3]);
    sse_s32[0] = vaddq_s32(sse_s32[0], sse_s32[2]);

    *sse = horizontal_add_u32x4(vreinterpretq_u32_s32(sse_s32[0]));
    horizontal_add_u32x4(vreinterpretq_u32_s32(sse_s32[0]))
}

// #define MSE_WXH_NEON(w, h)
macro_rules! mse_wxh_neon {
    ($name:ident, $helper:ident, $h:literal) => {
        #[no_mangle]
        pub unsafe extern "C-unwind" fn $name(src: *const u8, src_stride: i32, ref_: *const u8,
                                              ref_stride: i32, sse: *mut u32) -> u32 {
            $helper(src, src_stride, ref_, ref_stride, sse, $h)
        }
    };
}

mse_wxh_neon!(aom_mse8x8_neon, mse8xh_neon, 8);
mse_wxh_neon!(aom_mse8x16_neon, mse8xh_neon, 16);

mse_wxh_neon!(aom_mse16x8_neon, mse16xh_neon, 8);
mse_wxh_neon!(aom_mse16x16_neon, mse16xh_neon, 16);

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
