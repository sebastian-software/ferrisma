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

// Translated by hand from
// upstream/aom/av1/common/arm/highbd_wiener_convolve_neon.c (ADR 0056).
// Intrinsic sequences, helper structure, and names follow the original. The
// function-generating C macros become macro_rules! that receive the generated
// function names explicitly. C locals that the original leaves uninitialized
// start zeroed here.
#![allow(non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![allow(unused_variables, dead_code, clippy::all)]

use core::arch::aarch64::*;

use crate::av1__common__restoration::WienerConvolveParams;
use crate::simd::aom_dsp__arm__mem_neon::*;

// aom_dsp/aom_filter.h
const FILTER_BITS: i32 = 7;
// av1/common/convolve.h
const WIENER_ROUND0_BITS: i32 = 3;
#[inline(always)]
fn WIENER_CLAMP_LIMIT(r0: i32, bd: i32) -> i32 {
    1 << (bd + 1 + FILTER_BITS - r0)
}
// av1/common/enums.h
const MAX_SB_SIZE: i32 = 128;
// av1/common/restoration.h
const WIENER_WIN: i32 = 7;
const WIENER_WIN_REDUCED: i32 = WIENER_WIN - 2;

// aom_ports/mem.h
#[inline(always)]
fn CONVERT_TO_SHORTPTR(x: *const u8) -> *mut u16 {
    ((x as usize) << 1) as *mut u16
}

macro_rules! hbd_wiener_5tap_horiz {
    ($convolve:ident, $add_src:ident, $shift:expr) => {
        #[inline(always)]
        unsafe fn $convolve(
            s0: int16x8_t,
            s1: int16x8_t,
            s2: int16x8_t,
            s3: int16x8_t,
            s4: int16x8_t,
            x_filter: int16x4_t,
            round_vec: int32x4_t,
            im_max_val: uint16x8_t,
        ) -> uint16x8_t {
            /* Wiener filter is symmetric so add mirrored source elements. */
            let s04: int16x8_t = vaddq_s16(s0, s4);
            let s13: int16x8_t = vaddq_s16(s1, s3);

            /* x_filter[0] = 0. (5-tap filters are 0-padded to 7 taps.) */
            let mut sum_lo: int32x4_t =
                vmlal_lane_s16::<1>(round_vec, vget_low_s16(s04), x_filter);
            sum_lo = vmlal_lane_s16::<2>(sum_lo, vget_low_s16(s13), x_filter);
            sum_lo = vmlal_lane_s16::<3>(sum_lo, vget_low_s16(s2), x_filter);

            let mut sum_hi: int32x4_t =
                vmlal_lane_s16::<1>(round_vec, vget_high_s16(s04), x_filter);
            sum_hi = vmlal_lane_s16::<2>(sum_hi, vget_high_s16(s13), x_filter);
            sum_hi = vmlal_lane_s16::<3>(sum_hi, vget_high_s16(s2), x_filter);

            let res_lo: uint16x4_t = vqrshrun_n_s32::<{ $shift }>(sum_lo);
            let res_hi: uint16x4_t = vqrshrun_n_s32::<{ $shift }>(sum_hi);

            vminq_u16(vcombine_u16(res_lo, res_hi), im_max_val)
        }

        #[inline(always)]
        unsafe fn $add_src(
            mut src_ptr: *const u16,
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
                let mut s: *const i16 = src_ptr as *const i16;
                let mut d: *mut u16 = dst_ptr;
                let mut width: i32 = w;

                loop {
                    let mut s0: int16x8_t = core::mem::zeroed();
                    let mut s1: int16x8_t = core::mem::zeroed();
                    let mut s2: int16x8_t = core::mem::zeroed();
                    let mut s3: int16x8_t = core::mem::zeroed();
                    let mut s4: int16x8_t = core::mem::zeroed();
                    load_s16_8x5(s, 1, &mut s0, &mut s1, &mut s2, &mut s3, &mut s4);

                    let d0: uint16x8_t =
                        $convolve(s0, s1, s2, s3, s4, x_filter, round_vec, im_max_val);

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
    };
}

hbd_wiener_5tap_horiz!(
    highbd_wiener_convolve5_8_2d_h,
    highbd_convolve_add_src_5tap_horiz,
    WIENER_ROUND0_BITS
);
hbd_wiener_5tap_horiz!(
    highbd_12_wiener_convolve5_8_2d_h,
    highbd_12_convolve_add_src_5tap_horiz,
    WIENER_ROUND0_BITS + 2
);

macro_rules! hbd_wiener_7tap_horiz {
    ($convolve:ident, $add_src:ident, $shift:expr) => {
        #[inline(always)]
        unsafe fn $convolve(
            s0: int16x8_t,
            s1: int16x8_t,
            s2: int16x8_t,
            s3: int16x8_t,
            s4: int16x8_t,
            s5: int16x8_t,
            s6: int16x8_t,
            x_filter: int16x4_t,
            round_vec: int32x4_t,
            im_max_val: uint16x8_t,
        ) -> uint16x8_t {
            /* Wiener filter is symmetric so add mirrored source elements. */
            let s06: int16x8_t = vaddq_s16(s0, s6);
            let s15: int16x8_t = vaddq_s16(s1, s5);
            let s24: int16x8_t = vaddq_s16(s2, s4);

            let mut sum_lo: int32x4_t =
                vmlal_lane_s16::<0>(round_vec, vget_low_s16(s06), x_filter);
            sum_lo = vmlal_lane_s16::<1>(sum_lo, vget_low_s16(s15), x_filter);
            sum_lo = vmlal_lane_s16::<2>(sum_lo, vget_low_s16(s24), x_filter);
            sum_lo = vmlal_lane_s16::<3>(sum_lo, vget_low_s16(s3), x_filter);

            let mut sum_hi: int32x4_t =
                vmlal_lane_s16::<0>(round_vec, vget_high_s16(s06), x_filter);
            sum_hi = vmlal_lane_s16::<1>(sum_hi, vget_high_s16(s15), x_filter);
            sum_hi = vmlal_lane_s16::<2>(sum_hi, vget_high_s16(s24), x_filter);
            sum_hi = vmlal_lane_s16::<3>(sum_hi, vget_high_s16(s3), x_filter);

            let res_lo: uint16x4_t = vqrshrun_n_s32::<{ $shift }>(sum_lo);
            let res_hi: uint16x4_t = vqrshrun_n_s32::<{ $shift }>(sum_hi);

            vminq_u16(vcombine_u16(res_lo, res_hi), im_max_val)
        }

        #[inline(always)]
        unsafe fn $add_src(
            mut src_ptr: *const u16,
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
                let mut s: *const i16 = src_ptr as *const i16;
                let mut d: *mut u16 = dst_ptr;
                let mut width: i32 = w;

                loop {
                    let mut s0: int16x8_t = core::mem::zeroed();
                    let mut s1: int16x8_t = core::mem::zeroed();
                    let mut s2: int16x8_t = core::mem::zeroed();
                    let mut s3: int16x8_t = core::mem::zeroed();
                    let mut s4: int16x8_t = core::mem::zeroed();
                    let mut s5: int16x8_t = core::mem::zeroed();
                    let mut s6: int16x8_t = core::mem::zeroed();
                    load_s16_8x7(
                        s, 1, &mut s0, &mut s1, &mut s2, &mut s3, &mut s4, &mut s5, &mut s6,
                    );

                    let d0: uint16x8_t = $convolve(
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
    };
}

hbd_wiener_7tap_horiz!(
    highbd_wiener_convolve7_8_2d_h,
    highbd_convolve_add_src_7tap_horiz,
    WIENER_ROUND0_BITS
);
hbd_wiener_7tap_horiz!(
    highbd_12_wiener_convolve7_8_2d_h,
    highbd_12_convolve_add_src_7tap_horiz,
    WIENER_ROUND0_BITS + 2
);

macro_rules! hbd_wiener_5tap_vert {
    ($convolve:ident, $add_src:ident, $shift:expr) => {
        #[inline(always)]
        unsafe fn $convolve(
            s0: int16x8_t,
            s1: int16x8_t,
            s2: int16x8_t,
            s3: int16x8_t,
            s4: int16x8_t,
            y_filter: int16x4_t,
            round_vec: int32x4_t,
            res_max_val: uint16x8_t,
        ) -> uint16x8_t {
            let y_filter_lo: int32x2_t = vget_low_s32(vmovl_s16(y_filter));
            let y_filter_hi: int32x2_t = vget_high_s32(vmovl_s16(y_filter));
            /* Wiener filter is symmetric so add mirrored source elements. */
            let s04_lo: int32x4_t = vaddl_s16(vget_low_s16(s0), vget_low_s16(s4));
            let s13_lo: int32x4_t = vaddl_s16(vget_low_s16(s1), vget_low_s16(s3));

            /* y_filter[0] = 0. (5-tap filters are 0-padded to 7 taps.) */
            let mut sum_lo: int32x4_t = vmlaq_lane_s32::<1>(round_vec, s04_lo, y_filter_lo);
            sum_lo = vmlaq_lane_s32::<0>(sum_lo, s13_lo, y_filter_hi);
            sum_lo = vmlaq_lane_s32::<1>(sum_lo, vmovl_s16(vget_low_s16(s2)), y_filter_hi);

            let s04_hi: int32x4_t = vaddl_s16(vget_high_s16(s0), vget_high_s16(s4));
            let s13_hi: int32x4_t = vaddl_s16(vget_high_s16(s1), vget_high_s16(s3));

            let mut sum_hi: int32x4_t = vmlaq_lane_s32::<1>(round_vec, s04_hi, y_filter_lo);
            sum_hi = vmlaq_lane_s32::<0>(sum_hi, s13_hi, y_filter_hi);
            sum_hi = vmlaq_lane_s32::<1>(sum_hi, vmovl_s16(vget_high_s16(s2)), y_filter_hi);

            let res_lo: uint16x4_t = vqrshrun_n_s32::<{ $shift }>(sum_lo);
            let res_hi: uint16x4_t = vqrshrun_n_s32::<{ $shift }>(sum_hi);

            vminq_u16(vcombine_u16(res_lo, res_hi), res_max_val)
        }

        #[inline(always)]
        unsafe fn $add_src(
            mut src_ptr: *const u16,
            src_stride: isize,
            mut dst_ptr: *mut u16,
            dst_stride: isize,
            mut w: i32,
            h: i32,
            y_filter: int16x4_t,
            round_vec: int32x4_t,
            res_max_val: uint16x8_t,
        ) {
            loop {
                let mut s: *const i16 = src_ptr as *const i16;
                let mut d: *mut u16 = dst_ptr;
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
                        s, src_stride, &mut s0, &mut s1, &mut s2, &mut s3, &mut s4, &mut s5,
                        &mut s6, &mut s7,
                    );

                    let d0: uint16x8_t =
                        $convolve(s0, s1, s2, s3, s4, y_filter, round_vec, res_max_val);
                    let d1: uint16x8_t =
                        $convolve(s1, s2, s3, s4, s5, y_filter, round_vec, res_max_val);
                    let d2: uint16x8_t =
                        $convolve(s2, s3, s4, s5, s6, y_filter, round_vec, res_max_val);
                    let d3: uint16x8_t =
                        $convolve(s3, s4, s5, s6, s7, y_filter, round_vec, res_max_val);

                    store_u16_8x4(d, dst_stride, d0, d1, d2, d3);

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

                    let d0: uint16x8_t =
                        $convolve(s0, s1, s2, s3, s4, y_filter, round_vec, res_max_val);

                    vst1q_u16(d, d0);

                    s = s.offset(src_stride);
                    d = d.offset(dst_stride);
                }

                src_ptr = src_ptr.offset(8);
                dst_ptr = dst_ptr.offset(8);
                w -= 8;
                if w == 0 {
                    break;
                }
            }
        }
    };
}

hbd_wiener_5tap_vert!(
    highbd_wiener_convolve5_8_2d_v,
    highbd_convolve_add_src_5tap_vert,
    2 * FILTER_BITS - WIENER_ROUND0_BITS
);
hbd_wiener_5tap_vert!(
    highbd_12_wiener_convolve5_8_2d_v,
    highbd_12_convolve_add_src_5tap_vert,
    2 * FILTER_BITS - WIENER_ROUND0_BITS - 2
);

macro_rules! hbd_wiener_7tap_vert {
    ($convolve:ident, $add_src:ident, $shift:expr) => {
        #[inline(always)]
        unsafe fn $convolve(
            s0: int16x8_t,
            s1: int16x8_t,
            s2: int16x8_t,
            s3: int16x8_t,
            s4: int16x8_t,
            s5: int16x8_t,
            s6: int16x8_t,
            y_filter: int16x4_t,
            round_vec: int32x4_t,
            res_max_val: uint16x8_t,
        ) -> uint16x8_t {
            let y_filter_lo: int32x2_t = vget_low_s32(vmovl_s16(y_filter));
            let y_filter_hi: int32x2_t = vget_high_s32(vmovl_s16(y_filter));
            /* Wiener filter is symmetric so add mirrored source elements. */
            let s06_lo: int32x4_t = vaddl_s16(vget_low_s16(s0), vget_low_s16(s6));
            let s15_lo: int32x4_t = vaddl_s16(vget_low_s16(s1), vget_low_s16(s5));
            let s24_lo: int32x4_t = vaddl_s16(vget_low_s16(s2), vget_low_s16(s4));

            let mut sum_lo: int32x4_t = vmlaq_lane_s32::<0>(round_vec, s06_lo, y_filter_lo);
            sum_lo = vmlaq_lane_s32::<1>(sum_lo, s15_lo, y_filter_lo);
            sum_lo = vmlaq_lane_s32::<0>(sum_lo, s24_lo, y_filter_hi);
            sum_lo = vmlaq_lane_s32::<1>(sum_lo, vmovl_s16(vget_low_s16(s3)), y_filter_hi);

            let s06_hi: int32x4_t = vaddl_s16(vget_high_s16(s0), vget_high_s16(s6));
            let s15_hi: int32x4_t = vaddl_s16(vget_high_s16(s1), vget_high_s16(s5));
            let s24_hi: int32x4_t = vaddl_s16(vget_high_s16(s2), vget_high_s16(s4));

            let mut sum_hi: int32x4_t = vmlaq_lane_s32::<0>(round_vec, s06_hi, y_filter_lo);
            sum_hi = vmlaq_lane_s32::<1>(sum_hi, s15_hi, y_filter_lo);
            sum_hi = vmlaq_lane_s32::<0>(sum_hi, s24_hi, y_filter_hi);
            sum_hi = vmlaq_lane_s32::<1>(sum_hi, vmovl_s16(vget_high_s16(s3)), y_filter_hi);

            let res_lo: uint16x4_t = vqrshrun_n_s32::<{ $shift }>(sum_lo);
            let res_hi: uint16x4_t = vqrshrun_n_s32::<{ $shift }>(sum_hi);

            vminq_u16(vcombine_u16(res_lo, res_hi), res_max_val)
        }

        #[inline(always)]
        unsafe fn $add_src(
            mut src_ptr: *const u16,
            src_stride: isize,
            mut dst_ptr: *mut u16,
            dst_stride: isize,
            mut w: i32,
            h: i32,
            y_filter: int16x4_t,
            round_vec: int32x4_t,
            res_max_val: uint16x8_t,
        ) {
            loop {
                let mut s: *const i16 = src_ptr as *const i16;
                let mut d: *mut u16 = dst_ptr;
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
                        s, src_stride, &mut s0, &mut s1, &mut s2, &mut s3, &mut s4, &mut s5,
                        &mut s6, &mut s7, &mut s8, &mut s9,
                    );

                    let d0: uint16x8_t =
                        $convolve(s0, s1, s2, s3, s4, s5, s6, y_filter, round_vec, res_max_val);
                    let d1: uint16x8_t =
                        $convolve(s1, s2, s3, s4, s5, s6, s7, y_filter, round_vec, res_max_val);
                    let d2: uint16x8_t =
                        $convolve(s2, s3, s4, s5, s6, s7, s8, y_filter, round_vec, res_max_val);
                    let d3: uint16x8_t =
                        $convolve(s3, s4, s5, s6, s7, s8, s9, y_filter, round_vec, res_max_val);

                    store_u16_8x4(d, dst_stride, d0, d1, d2, d3);

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
                        s, src_stride, &mut s0, &mut s1, &mut s2, &mut s3, &mut s4, &mut s5,
                        &mut s6,
                    );

                    let d0: uint16x8_t =
                        $convolve(s0, s1, s2, s3, s4, s5, s6, y_filter, round_vec, res_max_val);

                    vst1q_u16(d, d0);

                    s = s.offset(src_stride);
                    d = d.offset(dst_stride);
                }

                src_ptr = src_ptr.offset(8);
                dst_ptr = dst_ptr.offset(8);
                w -= 8;
                if w == 0 {
                    break;
                }
            }
        }
    };
}

hbd_wiener_7tap_vert!(
    highbd_wiener_convolve7_8_2d_v,
    highbd_convolve_add_src_7tap_vert,
    2 * FILTER_BITS - WIENER_ROUND0_BITS
);
hbd_wiener_7tap_vert!(
    highbd_12_wiener_convolve7_8_2d_v,
    highbd_12_convolve_add_src_7tap_vert,
    2 * FILTER_BITS - WIENER_ROUND0_BITS - 2
);

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

#[no_mangle]
pub unsafe extern "C-unwind" fn av1_highbd_wiener_convolve_add_src_neon(
    src8: *const u8,
    src_stride: isize,
    dst8: *mut u8,
    dst_stride: isize,
    x_filter: *const i16,
    x_step_q4: i32,
    y_filter: *const i16,
    y_step_q4: i32,
    w: i32,
    h: i32,
    conv_params: *const WienerConvolveParams,
    bd: i32,
) {
    debug_assert!(w % 8 == 0);
    debug_assert!(w <= MAX_SB_SIZE && h <= MAX_SB_SIZE);
    debug_assert!(x_step_q4 == 16 && y_step_q4 == 16);
    debug_assert!(*x_filter.offset(7) == 0 && *y_filter.offset(7) == 0);

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

    let extraprec_clamp_limit: i32 = WIENER_CLAMP_LIMIT((*conv_params).round_0, bd);
    let im_max_val: uint16x8_t = vdupq_n_u16((extraprec_clamp_limit - 1) as u16);
    let horiz_round_vec: int32x4_t = vdupq_n_s32(1 << (bd + FILTER_BITS - 1));

    let res_max_val: uint16x8_t = vdupq_n_u16(((1 << bd) - 1) as u16);
    let vert_round_vec: int32x4_t = vdupq_n_s32(-(1 << (bd + (*conv_params).round_1 - 1)));

    let src: *mut u16 = CONVERT_TO_SHORTPTR(src8);
    let dst: *mut u16 = CONVERT_TO_SHORTPTR(dst8);
    let src_start: *const u16 = src.offset(-(horiz_offset as isize) - vert_offset as isize);
    let im: *mut u16 = im_block.0.as_mut_ptr();

    if bd == 12 {
        if x_filter_taps == WIENER_WIN_REDUCED {
            highbd_12_convolve_add_src_5tap_horiz(
                src_start, src_stride, im, im_stride as isize, w, im_h, x_filter_s16,
                horiz_round_vec, im_max_val,
            );
        } else {
            highbd_12_convolve_add_src_7tap_horiz(
                src_start, src_stride, im, im_stride as isize, w, im_h, x_filter_s16,
                horiz_round_vec, im_max_val,
            );
        }

        if y_filter_taps == WIENER_WIN_REDUCED {
            highbd_12_convolve_add_src_5tap_vert(
                im, im_stride as isize, dst, dst_stride, w, h, y_filter_s16, vert_round_vec,
                res_max_val,
            );
        } else {
            highbd_12_convolve_add_src_7tap_vert(
                im, im_stride as isize, dst, dst_stride, w, h, y_filter_s16, vert_round_vec,
                res_max_val,
            );
        }
    } else {
        if x_filter_taps == WIENER_WIN_REDUCED {
            highbd_convolve_add_src_5tap_horiz(
                src_start, src_stride, im, im_stride as isize, w, im_h, x_filter_s16,
                horiz_round_vec, im_max_val,
            );
        } else {
            highbd_convolve_add_src_7tap_horiz(
                src_start, src_stride, im, im_stride as isize, w, im_h, x_filter_s16,
                horiz_round_vec, im_max_val,
            );
        }

        if y_filter_taps == WIENER_WIN_REDUCED {
            highbd_convolve_add_src_5tap_vert(
                im, im_stride as isize, dst, dst_stride, w, h, y_filter_s16, vert_round_vec,
                res_max_val,
            );
        } else {
            highbd_convolve_add_src_7tap_vert(
                im, im_stride as isize, dst, dst_stride, w, h, y_filter_s16, vert_round_vec,
                res_max_val,
            );
        }
    }
}
