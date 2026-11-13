/*
 * Copyright (c) 2018, Alliance for Open Media. All rights reserved.
 *
 * This source code is subject to the terms of the BSD 2 Clause License and
 * the Alliance for Open Media Patent License 1.0. If the BSD 2 Clause License
 * was not distributed with this source code in the LICENSE file, you can
 * obtain it at www.aomedia.org/license/software. If the Alliance for Open
 * Media Patent License 1.0 was not distributed with this source code in the
 * PATENTS file, you can obtain it at www.aomedia.org/license/patent.
 */

// Translated by hand from upstream/aom/av1/common/arm/wiener_convolve_neon.c
// (ADR 0056). Intrinsic sequences, helper structure, and names follow the
// original. C locals that the original leaves uninitialized start zeroed here.
#![allow(non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![allow(unused_variables, dead_code, clippy::all)]

use core::arch::aarch64::*;

use crate::av1__common__restoration::WienerConvolveParams;
use crate::simd::aom_dsp__arm__mem_neon::*;

// aom_dsp/aom_filter.h
const FILTER_BITS: i32 = 7;
// av1/common/convolve.h
const WIENER_ROUND0_BITS: i32 = 3;
// av1/common/enums.h
const MAX_SB_SIZE: i32 = 128;
// av1/common/restoration.h
const WIENER_WIN: i32 = 7;
const WIENER_WIN_REDUCED: i32 = WIENER_WIN - 2;

#[inline(always)]
unsafe fn wiener_convolve5_8_2d_h(
    t0: uint8x8_t,
    t1: uint8x8_t,
    t2: uint8x8_t,
    t3: uint8x8_t,
    t4: uint8x8_t,
    x_filter: int16x4_t,
    round_vec: int32x4_t,
    im_max_val: uint16x8_t,
) -> uint16x8_t {
    // Since the Wiener filter is symmetric about the middle tap (tap 2) add
    // mirrored source elements before multiplying filter coefficients.
    let s04: int16x8_t = vreinterpretq_s16_u16(vaddl_u8(t0, t4));
    let s13: int16x8_t = vreinterpretq_s16_u16(vaddl_u8(t1, t3));
    let s2: int16x8_t = vreinterpretq_s16_u16(vmovl_u8(t2));

    // x_filter[0] = 0. (5-tap filters are 0-padded to 7 taps.)
    let mut sum_lo: int32x4_t = vmlal_lane_s16::<1>(round_vec, vget_low_s16(s04), x_filter);
    sum_lo = vmlal_lane_s16::<2>(sum_lo, vget_low_s16(s13), x_filter);
    sum_lo = vmlal_lane_s16::<3>(sum_lo, vget_low_s16(s2), x_filter);

    let mut sum_hi: int32x4_t = vmlal_lane_s16::<1>(round_vec, vget_high_s16(s04), x_filter);
    sum_hi = vmlal_lane_s16::<2>(sum_hi, vget_high_s16(s13), x_filter);
    sum_hi = vmlal_lane_s16::<3>(sum_hi, vget_high_s16(s2), x_filter);

    let res: uint16x8_t = vcombine_u16(
        vqrshrun_n_s32::<WIENER_ROUND0_BITS>(sum_lo),
        vqrshrun_n_s32::<WIENER_ROUND0_BITS>(sum_hi),
    );

    vminq_u16(res, im_max_val)
}

#[inline(always)]
unsafe fn convolve_add_src_horiz_5tap_neon(
    mut src_ptr: *const u8,
    src_stride: isize,
    mut dst_ptr: *mut u16,
    dst_stride: isize,
    w: i32,
    mut h: i32,
    x_filter: int16x4_t,
    round_vec: int32x4_t,
    im_max_val: uint16x8_t,
) {
    loop {
        let mut s: *const u8 = src_ptr;
        let mut d: *mut u16 = dst_ptr;
        let mut width: i32 = w;

        loop {
            let mut s0: uint8x8_t = core::mem::zeroed();
            let mut s1: uint8x8_t = core::mem::zeroed();
            let mut s2: uint8x8_t = core::mem::zeroed();
            let mut s3: uint8x8_t = core::mem::zeroed();
            let mut s4: uint8x8_t = core::mem::zeroed();
            load_u8_8x5(s, 1, &mut s0, &mut s1, &mut s2, &mut s3, &mut s4);

            let d0: uint16x8_t =
                wiener_convolve5_8_2d_h(s0, s1, s2, s3, s4, x_filter, round_vec, im_max_val);

            vst1q_u16(d, d0);

            s = s.offset(8);
            d = d.offset(8);
            width -= 8;
            if width == 0 {
                break;
            }
        }
        src_ptr = src_ptr.offset(src_stride);
        dst_ptr = dst_ptr.offset(dst_stride);
        h -= 1;
        if h == 0 {
            break;
        }
    }
}

#[inline(always)]
unsafe fn wiener_convolve7_8_2d_h(
    t0: uint8x8_t,
    t1: uint8x8_t,
    t2: uint8x8_t,
    t3: uint8x8_t,
    t4: uint8x8_t,
    t5: uint8x8_t,
    t6: uint8x8_t,
    x_filter: int16x4_t,
    round_vec: int32x4_t,
    im_max_val: uint16x8_t,
) -> uint16x8_t {
    // Since the Wiener filter is symmetric about the middle tap (tap 3) add
    // mirrored source elements before multiplying by filter coefficients.
    let s06: int16x8_t = vreinterpretq_s16_u16(vaddl_u8(t0, t6));
    let s15: int16x8_t = vreinterpretq_s16_u16(vaddl_u8(t1, t5));
    let s24: int16x8_t = vreinterpretq_s16_u16(vaddl_u8(t2, t4));
    let s3: int16x8_t = vreinterpretq_s16_u16(vmovl_u8(t3));

    let mut sum_lo: int32x4_t = vmlal_lane_s16::<0>(round_vec, vget_low_s16(s06), x_filter);
    sum_lo = vmlal_lane_s16::<1>(sum_lo, vget_low_s16(s15), x_filter);
    sum_lo = vmlal_lane_s16::<2>(sum_lo, vget_low_s16(s24), x_filter);
    sum_lo = vmlal_lane_s16::<3>(sum_lo, vget_low_s16(s3), x_filter);

    let mut sum_hi: int32x4_t = vmlal_lane_s16::<0>(round_vec, vget_high_s16(s06), x_filter);
    sum_hi = vmlal_lane_s16::<1>(sum_hi, vget_high_s16(s15), x_filter);
    sum_hi = vmlal_lane_s16::<2>(sum_hi, vget_high_s16(s24), x_filter);
    sum_hi = vmlal_lane_s16::<3>(sum_hi, vget_high_s16(s3), x_filter);

    let res: uint16x8_t = vcombine_u16(
        vqrshrun_n_s32::<WIENER_ROUND0_BITS>(sum_lo),
        vqrshrun_n_s32::<WIENER_ROUND0_BITS>(sum_hi),
    );

    vminq_u16(res, im_max_val)
}

#[inline(always)]
unsafe fn convolve_add_src_horiz_7tap_neon(
    mut src_ptr: *const u8,
    src_stride: isize,
    mut dst_ptr: *mut u16,
    dst_stride: isize,
    w: i32,
    mut h: i32,
    x_filter: int16x4_t,
    round_vec: int32x4_t,
    im_max_val: uint16x8_t,
) {
    loop {
        let mut s: *const u8 = src_ptr;
        let mut d: *mut u16 = dst_ptr;
        let mut width: i32 = w;

        loop {
            let mut s0: uint8x8_t = core::mem::zeroed();
            let mut s1: uint8x8_t = core::mem::zeroed();
            let mut s2: uint8x8_t = core::mem::zeroed();
            let mut s3: uint8x8_t = core::mem::zeroed();
            let mut s4: uint8x8_t = core::mem::zeroed();
            let mut s5: uint8x8_t = core::mem::zeroed();
            let mut s6: uint8x8_t = core::mem::zeroed();
            load_u8_8x7(s, 1, &mut s0, &mut s1, &mut s2, &mut s3, &mut s4, &mut s5, &mut s6);

            let d0: uint16x8_t = wiener_convolve7_8_2d_h(
                s0, s1, s2, s3, s4, s5, s6, x_filter, round_vec, im_max_val,
            );

            vst1q_u16(d, d0);

            s = s.offset(8);
            d = d.offset(8);
            width -= 8;
            if width == 0 {
                break;
            }
        }
        src_ptr = src_ptr.offset(src_stride);
        dst_ptr = dst_ptr.offset(dst_stride);
        h -= 1;
        if h == 0 {
            break;
        }
    }
}

#[inline(always)]
unsafe fn wiener_convolve5_8_2d_v(
    s0: int16x8_t,
    s1: int16x8_t,
    s2: int16x8_t,
    s3: int16x8_t,
    s4: int16x8_t,
    y_filter: int16x4_t,
    round_vec: int32x4_t,
) -> uint8x8_t {
    // Since the Wiener filter is symmetric about the middle tap (tap 2) add
    // mirrored source elements before multiplying by filter coefficients.
    let s04: int16x8_t = vaddq_s16(s0, s4);
    let s13: int16x8_t = vaddq_s16(s1, s3);

    let mut sum_lo: int32x4_t = vmlal_lane_s16::<1>(round_vec, vget_low_s16(s04), y_filter);
    sum_lo = vmlal_lane_s16::<2>(sum_lo, vget_low_s16(s13), y_filter);
    sum_lo = vmlal_lane_s16::<3>(sum_lo, vget_low_s16(s2), y_filter);

    let mut sum_hi: int32x4_t = vmlal_lane_s16::<1>(round_vec, vget_high_s16(s04), y_filter);
    sum_hi = vmlal_lane_s16::<2>(sum_hi, vget_high_s16(s13), y_filter);
    sum_hi = vmlal_lane_s16::<3>(sum_hi, vget_high_s16(s2), y_filter);

    let res_lo: int16x4_t = vshrn_n_s32::<{ 2 * FILTER_BITS - WIENER_ROUND0_BITS }>(sum_lo);
    let res_hi: int16x4_t = vshrn_n_s32::<{ 2 * FILTER_BITS - WIENER_ROUND0_BITS }>(sum_hi);

    vqmovun_s16(vcombine_s16(res_lo, res_hi))
}

#[inline(always)]
unsafe fn convolve_add_src_vert_5tap_neon(
    mut src: *const u16,
    src_stride: isize,
    mut dst: *mut u8,
    dst_stride: isize,
    mut w: i32,
    h: i32,
    y_filter: int16x4_t,
    round_vec: int32x4_t,
) {
    loop {
        let mut s: *const i16 = src as *const i16;
        let mut d: *mut u8 = dst;
        let mut height: i32 = h;

        while height > 3 {
            let mut s0: int16x8_t = core::mem::zeroed();
            let mut s1: int16x8_t = core::mem::zeroed();
            let mut s2: int16x8_t = core::mem::zeroed();
            let mut s3: int16x8_t = core::mem::zeroed();
            let mut s4: int16x8_t = core::mem::zeroed();
            let mut s5: int16x8_t = core::mem::zeroed();
            let mut s6: int16x8_t = core::mem::zeroed();
            let mut s7: int16x8_t = core::mem::zeroed();
            load_s16_8x8(
                s, src_stride, &mut s0, &mut s1, &mut s2, &mut s3, &mut s4, &mut s5, &mut s6,
                &mut s7,
            );

            let d0: uint8x8_t = wiener_convolve5_8_2d_v(s0, s1, s2, s3, s4, y_filter, round_vec);
            let d1: uint8x8_t = wiener_convolve5_8_2d_v(s1, s2, s3, s4, s5, y_filter, round_vec);
            let d2: uint8x8_t = wiener_convolve5_8_2d_v(s2, s3, s4, s5, s6, y_filter, round_vec);
            let d3: uint8x8_t = wiener_convolve5_8_2d_v(s3, s4, s5, s6, s7, y_filter, round_vec);

            store_u8_8x4(d, dst_stride, d0, d1, d2, d3);

            s = s.offset(4 * src_stride);
            d = d.offset(4 * dst_stride);
            height -= 4;
        }

        while height != 0 {
            height -= 1;
            let mut s0: int16x8_t = core::mem::zeroed();
            let mut s1: int16x8_t = core::mem::zeroed();
            let mut s2: int16x8_t = core::mem::zeroed();
            let mut s3: int16x8_t = core::mem::zeroed();
            let mut s4: int16x8_t = core::mem::zeroed();
            load_s16_8x5(s, src_stride, &mut s0, &mut s1, &mut s2, &mut s3, &mut s4);

            let d0: uint8x8_t = wiener_convolve5_8_2d_v(s0, s1, s2, s3, s4, y_filter, round_vec);

            vst1_u8(d, d0);

            d = d.offset(dst_stride);
            s = s.offset(src_stride);
        }

        src = src.offset(8);
        dst = dst.offset(8);
        w -= 8;
        if w == 0 {
            break;
        }
    }
}

#[inline(always)]
unsafe fn wiener_convolve7_8_2d_v(
    s0: int16x8_t,
    s1: int16x8_t,
    s2: int16x8_t,
    s3: int16x8_t,
    s4: int16x8_t,
    s5: int16x8_t,
    s6: int16x8_t,
    y_filter: int16x4_t,
    round_vec: int32x4_t,
) -> uint8x8_t {
    // Since the Wiener filter is symmetric about the middle tap (tap 3) add
    // mirrored source elements before multiplying by filter coefficients.
    let s06: int16x8_t = vaddq_s16(s0, s6);
    let s15: int16x8_t = vaddq_s16(s1, s5);
    let s24: int16x8_t = vaddq_s16(s2, s4);

    let mut sum_lo: int32x4_t = vmlal_lane_s16::<0>(round_vec, vget_low_s16(s06), y_filter);
    sum_lo = vmlal_lane_s16::<1>(sum_lo, vget_low_s16(s15), y_filter);
    sum_lo = vmlal_lane_s16::<2>(sum_lo, vget_low_s16(s24), y_filter);
    sum_lo = vmlal_lane_s16::<3>(sum_lo, vget_low_s16(s3), y_filter);

    let mut sum_hi: int32x4_t = vmlal_lane_s16::<0>(round_vec, vget_high_s16(s06), y_filter);
    sum_hi = vmlal_lane_s16::<1>(sum_hi, vget_high_s16(s15), y_filter);
    sum_hi = vmlal_lane_s16::<2>(sum_hi, vget_high_s16(s24), y_filter);
    sum_hi = vmlal_lane_s16::<3>(sum_hi, vget_high_s16(s3), y_filter);

    let res_lo: int16x4_t = vshrn_n_s32::<{ 2 * FILTER_BITS - WIENER_ROUND0_BITS }>(sum_lo);
    let res_hi: int16x4_t = vshrn_n_s32::<{ 2 * FILTER_BITS - WIENER_ROUND0_BITS }>(sum_hi);

    vqmovun_s16(vcombine_s16(res_lo, res_hi))
}

#[inline(always)]
unsafe fn convolve_add_src_vert_7tap_neon(
    mut src: *const u16,
    src_stride: isize,
    mut dst: *mut u8,
    dst_stride: isize,
    mut w: i32,
    h: i32,
    y_filter: int16x4_t,
    round_vec: int32x4_t,
) {
    loop {
        let mut s: *const i16 = src as *const i16;
        let mut d: *mut u8 = dst;
        let mut height: i32 = h;

        while height > 3 {
            let mut s0: int16x8_t = core::mem::zeroed();
            let mut s1: int16x8_t = core::mem::zeroed();
            let mut s2: int16x8_t = core::mem::zeroed();
            let mut s3: int16x8_t = core::mem::zeroed();
            let mut s4: int16x8_t = core::mem::zeroed();
            let mut s5: int16x8_t = core::mem::zeroed();
            let mut s6: int16x8_t = core::mem::zeroed();
            let mut s7: int16x8_t = core::mem::zeroed();
            let mut s8: int16x8_t = core::mem::zeroed();
            let mut s9: int16x8_t = core::mem::zeroed();
            load_s16_8x10(
                s, src_stride, &mut s0, &mut s1, &mut s2, &mut s3, &mut s4, &mut s5, &mut s6,
                &mut s7, &mut s8, &mut s9,
            );

            let d0: uint8x8_t =
                wiener_convolve7_8_2d_v(s0, s1, s2, s3, s4, s5, s6, y_filter, round_vec);
            let d1: uint8x8_t =
                wiener_convolve7_8_2d_v(s1, s2, s3, s4, s5, s6, s7, y_filter, round_vec);
            let d2: uint8x8_t =
                wiener_convolve7_8_2d_v(s2, s3, s4, s5, s6, s7, s8, y_filter, round_vec);
            let d3: uint8x8_t =
                wiener_convolve7_8_2d_v(s3, s4, s5, s6, s7, s8, s9, y_filter, round_vec);

            store_u8_8x4(d, dst_stride, d0, d1, d2, d3);

            s = s.offset(4 * src_stride);
            d = d.offset(4 * dst_stride);
            height -= 4;
        }

        while height != 0 {
            height -= 1;
            let mut s0: int16x8_t = core::mem::zeroed();
            let mut s1: int16x8_t = core::mem::zeroed();
            let mut s2: int16x8_t = core::mem::zeroed();
            let mut s3: int16x8_t = core::mem::zeroed();
            let mut s4: int16x8_t = core::mem::zeroed();
            let mut s5: int16x8_t = core::mem::zeroed();
            let mut s6: int16x8_t = core::mem::zeroed();
            load_s16_8x7(
                s, src_stride, &mut s0, &mut s1, &mut s2, &mut s3, &mut s4, &mut s5, &mut s6,
            );

            let d0: uint8x8_t =
                wiener_convolve7_8_2d_v(s0, s1, s2, s3, s4, s5, s6, y_filter, round_vec);

            vst1_u8(d, d0);

            d = d.offset(dst_stride);
            s = s.offset(src_stride);
        }

        src = src.offset(8);
        dst = dst.offset(8);
        w -= 8;
        if w == 0 {
            break;
        }
    }
}

#[inline(always)]
unsafe fn get_wiener_filter_taps(filter: *const i16) -> i32 {
    debug_assert!(*filter.offset(7) == 0);
    if *filter.offset(0) == 0 && *filter.offset(6) == 0 {
        return WIENER_WIN_REDUCED;
    }
    WIENER_WIN
}

#[repr(C, align(16))]
struct Aligned16<T>(T);

// Wiener filter 2D
// Apply horizontal filter and store in a temporary buffer. When applying
// vertical filter, overwrite the original pixel values.
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_wiener_convolve_add_src_neon(
    src: *const u8,
    src_stride: isize,
    dst: *mut u8,
    dst_stride: isize,
    x_filter: *const i16,
    x_step_q4: i32,
    y_filter: *const i16,
    y_step_q4: i32,
    w: i32,
    h: i32,
    conv_params: *const WienerConvolveParams,
) {
    debug_assert!(w % 8 == 0);
    debug_assert!(w <= MAX_SB_SIZE && h <= MAX_SB_SIZE);
    debug_assert!(x_step_q4 == 16 && y_step_q4 == 16);
    debug_assert!(*x_filter.offset(7) == 0 && *y_filter.offset(7) == 0);
    // For bd == 8, assert horizontal filtering output will not exceed 15-bit:
    debug_assert!(8 + 1 + FILTER_BITS - (*conv_params).round_0 <= 15);

    let mut im_block: Aligned16<[u16; ((MAX_SB_SIZE + WIENER_WIN - 1) * MAX_SB_SIZE) as usize]> =
        Aligned16([0; ((MAX_SB_SIZE + WIENER_WIN - 1) * MAX_SB_SIZE) as usize]);

    let x_filter_taps: i32 = get_wiener_filter_taps(x_filter);
    let y_filter_taps: i32 = get_wiener_filter_taps(y_filter);
    let mut x_filter_s16: int16x4_t = vld1_s16(x_filter);
    let mut y_filter_s16: int16x4_t = vld1_s16(y_filter);
    // Add 128 to tap 3. (Needed for rounding.)
    x_filter_s16 = vadd_s16(x_filter_s16, vcreate_s16(128u64 << 48));
    y_filter_s16 = vadd_s16(y_filter_s16, vcreate_s16(128u64 << 48));

    let im_stride: i32 = MAX_SB_SIZE;
    let im_h: i32 = h + y_filter_taps - 1;
    let horiz_offset: i32 = x_filter_taps / 2;
    let vert_offset: i32 = (y_filter_taps / 2) * src_stride as i32;

    let bd: i32 = 8;
    let im_max_val: uint16x8_t =
        vdupq_n_u16(((1 << (bd + 1 + FILTER_BITS - WIENER_ROUND0_BITS)) - 1) as u16);
    let horiz_round_vec: int32x4_t = vdupq_n_s32(1 << (bd + FILTER_BITS - 1));

    let vert_round_vec: int32x4_t = vdupq_n_s32(
        (1 << (2 * FILTER_BITS - WIENER_ROUND0_BITS - 1))
            - (1 << (bd + (2 * FILTER_BITS - WIENER_ROUND0_BITS) - 1)),
    );

    if x_filter_taps == WIENER_WIN_REDUCED {
        convolve_add_src_horiz_5tap_neon(
            src.offset(-(horiz_offset as isize) - vert_offset as isize),
            src_stride,
            im_block.0.as_mut_ptr(),
            im_stride as isize,
            w,
            im_h,
            x_filter_s16,
            horiz_round_vec,
            im_max_val,
        );
    } else {
        convolve_add_src_horiz_7tap_neon(
            src.offset(-(horiz_offset as isize) - vert_offset as isize),
            src_stride,
            im_block.0.as_mut_ptr(),
            im_stride as isize,
            w,
            im_h,
            x_filter_s16,
            horiz_round_vec,
            im_max_val,
        );
    }

    if y_filter_taps == WIENER_WIN_REDUCED {
        convolve_add_src_vert_5tap_neon(
            im_block.0.as_ptr(),
            im_stride as isize,
            dst,
            dst_stride,
            w,
            h,
            y_filter_s16,
            vert_round_vec,
        );
    } else {
        convolve_add_src_vert_7tap_neon(
            im_block.0.as_ptr(),
            im_stride as isize,
            dst,
            dst_stride,
            w,
            h,
            y_filter_s16,
            vert_round_vec,
        );
    }
}
