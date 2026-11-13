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

// Translated by hand from upstream/aom/aom_dsp/arm/highbd_sse_neon.c
// (ADR 0056). Intrinsic sequences, helper structure, and names follow the
// original; only the AOM_ARCH_AARCH64 branches are retained. C locals that the
// original leaves uninitialized start zeroed here.
#![allow(unused_mut, unused_parens, unused_variables, unused_braces, clippy::all)]

use core::arch::aarch64::*;

use crate::simd::aom_dsp__arm__sum_neon::*;

// aom_dsp/aom_dsp_common.h
#[inline(always)]
fn CONVERT_TO_SHORTPTR(x: *const u8) -> *mut u16 {
    ((x as usize) << 1) as *mut u16
}

#[inline(always)]
unsafe fn highbd_sse_8x1_init_neon(mut src: *const u16, mut ref_: *const u16, mut sse_acc0: *mut uint32x4_t, mut sse_acc1: *mut uint32x4_t) {
    let mut s: uint16x8_t = vld1q_u16(src);
    let mut r: uint16x8_t = vld1q_u16(ref_);
    let mut abs_diff: uint16x8_t = vabdq_u16(s, r);
    let mut abs_diff_lo: uint16x4_t = vget_low_u16(abs_diff);
    let mut abs_diff_hi: uint16x4_t = vget_high_u16(abs_diff);
    *sse_acc0 = vmull_u16(abs_diff_lo, abs_diff_lo);
    *sse_acc1 = vmull_u16(abs_diff_hi, abs_diff_hi);
}

#[inline(always)]
unsafe fn highbd_sse_8x1_neon(mut src: *const u16, mut ref_: *const u16, mut sse_acc0: *mut uint32x4_t, mut sse_acc1: *mut uint32x4_t) {
    let mut s: uint16x8_t = vld1q_u16(src);
    let mut r: uint16x8_t = vld1q_u16(ref_);
    let mut abs_diff: uint16x8_t = vabdq_u16(s, r);
    let mut abs_diff_lo: uint16x4_t = vget_low_u16(abs_diff);
    let mut abs_diff_hi: uint16x4_t = vget_high_u16(abs_diff);
    *sse_acc0 = vmlal_u16(*sse_acc0, abs_diff_lo, abs_diff_lo);
    *sse_acc1 = vmlal_u16(*sse_acc1, abs_diff_hi, abs_diff_hi);
}

#[inline(always)]
unsafe fn highbd_sse_128xh_neon(mut src: *const u16, mut src_stride: i32, mut ref_: *const u16, mut ref_stride: i32, mut height: i32) -> i64 {
    let mut sse: [uint32x4_t; 16] = core::mem::zeroed();
    highbd_sse_8x1_init_neon(src.offset(((0 * 8)) as isize), ref_.offset(((0 * 8)) as isize), &raw mut sse[0], &raw mut sse[1]);
    highbd_sse_8x1_init_neon(src.offset(((1 * 8)) as isize), ref_.offset(((1 * 8)) as isize), &raw mut sse[2], &raw mut sse[3]);
    highbd_sse_8x1_init_neon(src.offset(((2 * 8)) as isize), ref_.offset(((2 * 8)) as isize), &raw mut sse[4], &raw mut sse[5]);
    highbd_sse_8x1_init_neon(src.offset(((3 * 8)) as isize), ref_.offset(((3 * 8)) as isize), &raw mut sse[6], &raw mut sse[7]);
    highbd_sse_8x1_init_neon(src.offset(((4 * 8)) as isize), ref_.offset(((4 * 8)) as isize), &raw mut sse[8], &raw mut sse[9]);
    highbd_sse_8x1_init_neon(src.offset(((5 * 8)) as isize), ref_.offset(((5 * 8)) as isize), &raw mut sse[10], &raw mut sse[11]);
    highbd_sse_8x1_init_neon(src.offset(((6 * 8)) as isize), ref_.offset(((6 * 8)) as isize), &raw mut sse[12], &raw mut sse[13]);
    highbd_sse_8x1_init_neon(src.offset(((7 * 8)) as isize), ref_.offset(((7 * 8)) as isize), &raw mut sse[14], &raw mut sse[15]);
    highbd_sse_8x1_neon(src.offset(((8 * 8)) as isize), ref_.offset(((8 * 8)) as isize), &raw mut sse[0], &raw mut sse[1]);
    highbd_sse_8x1_neon(src.offset(((9 * 8)) as isize), ref_.offset(((9 * 8)) as isize), &raw mut sse[2], &raw mut sse[3]);
    highbd_sse_8x1_neon(src.offset(((10 * 8)) as isize), ref_.offset(((10 * 8)) as isize), &raw mut sse[4], &raw mut sse[5]);
    highbd_sse_8x1_neon(src.offset(((11 * 8)) as isize), ref_.offset(((11 * 8)) as isize), &raw mut sse[6], &raw mut sse[7]);
    highbd_sse_8x1_neon(src.offset(((12 * 8)) as isize), ref_.offset(((12 * 8)) as isize), &raw mut sse[8], &raw mut sse[9]);
    highbd_sse_8x1_neon(src.offset(((13 * 8)) as isize), ref_.offset(((13 * 8)) as isize), &raw mut sse[10], &raw mut sse[11]);
    highbd_sse_8x1_neon(src.offset(((14 * 8)) as isize), ref_.offset(((14 * 8)) as isize), &raw mut sse[12], &raw mut sse[13]);
    highbd_sse_8x1_neon(src.offset(((15 * 8)) as isize), ref_.offset(((15 * 8)) as isize), &raw mut sse[14], &raw mut sse[15]);
    src = src.offset(((src_stride)) as isize);
    ref_ = ref_.offset(((ref_stride)) as isize);
    while { height -= 1; height } != 0 {
        highbd_sse_8x1_neon(src.offset(((0 * 8)) as isize), ref_.offset(((0 * 8)) as isize), &raw mut sse[0], &raw mut sse[1]);
        highbd_sse_8x1_neon(src.offset(((1 * 8)) as isize), ref_.offset(((1 * 8)) as isize), &raw mut sse[2], &raw mut sse[3]);
        highbd_sse_8x1_neon(src.offset(((2 * 8)) as isize), ref_.offset(((2 * 8)) as isize), &raw mut sse[4], &raw mut sse[5]);
        highbd_sse_8x1_neon(src.offset(((3 * 8)) as isize), ref_.offset(((3 * 8)) as isize), &raw mut sse[6], &raw mut sse[7]);
        highbd_sse_8x1_neon(src.offset(((4 * 8)) as isize), ref_.offset(((4 * 8)) as isize), &raw mut sse[8], &raw mut sse[9]);
        highbd_sse_8x1_neon(src.offset(((5 * 8)) as isize), ref_.offset(((5 * 8)) as isize), &raw mut sse[10], &raw mut sse[11]);
        highbd_sse_8x1_neon(src.offset(((6 * 8)) as isize), ref_.offset(((6 * 8)) as isize), &raw mut sse[12], &raw mut sse[13]);
        highbd_sse_8x1_neon(src.offset(((7 * 8)) as isize), ref_.offset(((7 * 8)) as isize), &raw mut sse[14], &raw mut sse[15]);
        highbd_sse_8x1_neon(src.offset(((8 * 8)) as isize), ref_.offset(((8 * 8)) as isize), &raw mut sse[0], &raw mut sse[1]);
        highbd_sse_8x1_neon(src.offset(((9 * 8)) as isize), ref_.offset(((9 * 8)) as isize), &raw mut sse[2], &raw mut sse[3]);
        highbd_sse_8x1_neon(src.offset(((10 * 8)) as isize), ref_.offset(((10 * 8)) as isize), &raw mut sse[4], &raw mut sse[5]);
        highbd_sse_8x1_neon(src.offset(((11 * 8)) as isize), ref_.offset(((11 * 8)) as isize), &raw mut sse[6], &raw mut sse[7]);
        highbd_sse_8x1_neon(src.offset(((12 * 8)) as isize), ref_.offset(((12 * 8)) as isize), &raw mut sse[8], &raw mut sse[9]);
        highbd_sse_8x1_neon(src.offset(((13 * 8)) as isize), ref_.offset(((13 * 8)) as isize), &raw mut sse[10], &raw mut sse[11]);
        highbd_sse_8x1_neon(src.offset(((14 * 8)) as isize), ref_.offset(((14 * 8)) as isize), &raw mut sse[12], &raw mut sse[13]);
        highbd_sse_8x1_neon(src.offset(((15 * 8)) as isize), ref_.offset(((15 * 8)) as isize), &raw mut sse[14], &raw mut sse[15]);
        src = src.offset(((src_stride)) as isize);
        ref_ = ref_.offset(((ref_stride)) as isize);
    }
    return horizontal_long_add_u32x4_x16(sse.as_mut_ptr()) as i64;
}

#[inline(always)]
unsafe fn highbd_sse_64xh_neon(mut src: *const u16, mut src_stride: i32, mut ref_: *const u16, mut ref_stride: i32, mut height: i32) -> i64 {
    let mut sse: [uint32x4_t; 8] = core::mem::zeroed();
    highbd_sse_8x1_init_neon(src.offset(((0 * 8)) as isize), ref_.offset(((0 * 8)) as isize), &raw mut sse[0], &raw mut sse[1]);
    highbd_sse_8x1_init_neon(src.offset(((1 * 8)) as isize), ref_.offset(((1 * 8)) as isize), &raw mut sse[2], &raw mut sse[3]);
    highbd_sse_8x1_init_neon(src.offset(((2 * 8)) as isize), ref_.offset(((2 * 8)) as isize), &raw mut sse[4], &raw mut sse[5]);
    highbd_sse_8x1_init_neon(src.offset(((3 * 8)) as isize), ref_.offset(((3 * 8)) as isize), &raw mut sse[6], &raw mut sse[7]);
    highbd_sse_8x1_neon(src.offset(((4 * 8)) as isize), ref_.offset(((4 * 8)) as isize), &raw mut sse[0], &raw mut sse[1]);
    highbd_sse_8x1_neon(src.offset(((5 * 8)) as isize), ref_.offset(((5 * 8)) as isize), &raw mut sse[2], &raw mut sse[3]);
    highbd_sse_8x1_neon(src.offset(((6 * 8)) as isize), ref_.offset(((6 * 8)) as isize), &raw mut sse[4], &raw mut sse[5]);
    highbd_sse_8x1_neon(src.offset(((7 * 8)) as isize), ref_.offset(((7 * 8)) as isize), &raw mut sse[6], &raw mut sse[7]);
    src = src.offset(((src_stride)) as isize);
    ref_ = ref_.offset(((ref_stride)) as isize);
    while { height -= 1; height } != 0 {
        highbd_sse_8x1_neon(src.offset(((0 * 8)) as isize), ref_.offset(((0 * 8)) as isize), &raw mut sse[0], &raw mut sse[1]);
        highbd_sse_8x1_neon(src.offset(((1 * 8)) as isize), ref_.offset(((1 * 8)) as isize), &raw mut sse[2], &raw mut sse[3]);
        highbd_sse_8x1_neon(src.offset(((2 * 8)) as isize), ref_.offset(((2 * 8)) as isize), &raw mut sse[4], &raw mut sse[5]);
        highbd_sse_8x1_neon(src.offset(((3 * 8)) as isize), ref_.offset(((3 * 8)) as isize), &raw mut sse[6], &raw mut sse[7]);
        highbd_sse_8x1_neon(src.offset(((4 * 8)) as isize), ref_.offset(((4 * 8)) as isize), &raw mut sse[0], &raw mut sse[1]);
        highbd_sse_8x1_neon(src.offset(((5 * 8)) as isize), ref_.offset(((5 * 8)) as isize), &raw mut sse[2], &raw mut sse[3]);
        highbd_sse_8x1_neon(src.offset(((6 * 8)) as isize), ref_.offset(((6 * 8)) as isize), &raw mut sse[4], &raw mut sse[5]);
        highbd_sse_8x1_neon(src.offset(((7 * 8)) as isize), ref_.offset(((7 * 8)) as isize), &raw mut sse[6], &raw mut sse[7]);
        src = src.offset(((src_stride)) as isize);
        ref_ = ref_.offset(((ref_stride)) as isize);
    }
    return horizontal_long_add_u32x4_x8(sse.as_mut_ptr()) as i64;
}

#[inline(always)]
unsafe fn highbd_sse_32xh_neon(mut src: *const u16, mut src_stride: i32, mut ref_: *const u16, mut ref_stride: i32, mut height: i32) -> i64 {
    let mut sse: [uint32x4_t; 8] = core::mem::zeroed();
    highbd_sse_8x1_init_neon(src.offset(((0 * 8)) as isize), ref_.offset(((0 * 8)) as isize), &raw mut sse[0], &raw mut sse[1]);
    highbd_sse_8x1_init_neon(src.offset(((1 * 8)) as isize), ref_.offset(((1 * 8)) as isize), &raw mut sse[2], &raw mut sse[3]);
    highbd_sse_8x1_init_neon(src.offset(((2 * 8)) as isize), ref_.offset(((2 * 8)) as isize), &raw mut sse[4], &raw mut sse[5]);
    highbd_sse_8x1_init_neon(src.offset(((3 * 8)) as isize), ref_.offset(((3 * 8)) as isize), &raw mut sse[6], &raw mut sse[7]);
    src = src.offset(((src_stride)) as isize);
    ref_ = ref_.offset(((ref_stride)) as isize);
    while { height -= 1; height } != 0 {
        highbd_sse_8x1_neon(src.offset(((0 * 8)) as isize), ref_.offset(((0 * 8)) as isize), &raw mut sse[0], &raw mut sse[1]);
        highbd_sse_8x1_neon(src.offset(((1 * 8)) as isize), ref_.offset(((1 * 8)) as isize), &raw mut sse[2], &raw mut sse[3]);
        highbd_sse_8x1_neon(src.offset(((2 * 8)) as isize), ref_.offset(((2 * 8)) as isize), &raw mut sse[4], &raw mut sse[5]);
        highbd_sse_8x1_neon(src.offset(((3 * 8)) as isize), ref_.offset(((3 * 8)) as isize), &raw mut sse[6], &raw mut sse[7]);
        src = src.offset(((src_stride)) as isize);
        ref_ = ref_.offset(((ref_stride)) as isize);
    }
    return horizontal_long_add_u32x4_x8(sse.as_mut_ptr()) as i64;
}

#[inline(always)]
unsafe fn highbd_sse_16xh_neon(mut src: *const u16, mut src_stride: i32, mut ref_: *const u16, mut ref_stride: i32, mut height: i32) -> i64 {
    let mut sse: [uint32x4_t; 4] = core::mem::zeroed();
    highbd_sse_8x1_init_neon(src.offset(((0 * 8)) as isize), ref_.offset(((0 * 8)) as isize), &raw mut sse[0], &raw mut sse[1]);
    highbd_sse_8x1_init_neon(src.offset(((1 * 8)) as isize), ref_.offset(((1 * 8)) as isize), &raw mut sse[2], &raw mut sse[3]);
    src = src.offset(((src_stride)) as isize);
    ref_ = ref_.offset(((ref_stride)) as isize);
    while { height -= 1; height } != 0 {
        highbd_sse_8x1_neon(src.offset(((0 * 8)) as isize), ref_.offset(((0 * 8)) as isize), &raw mut sse[0], &raw mut sse[1]);
        highbd_sse_8x1_neon(src.offset(((1 * 8)) as isize), ref_.offset(((1 * 8)) as isize), &raw mut sse[2], &raw mut sse[3]);
        src = src.offset(((src_stride)) as isize);
        ref_ = ref_.offset(((ref_stride)) as isize);
    }
    return horizontal_long_add_u32x4_x4(sse.as_mut_ptr()) as i64;
}

#[inline(always)]
unsafe fn highbd_sse_8xh_neon(mut src: *const u16, mut src_stride: i32, mut ref_: *const u16, mut ref_stride: i32, mut height: i32) -> i64 {
    let mut sse: [uint32x4_t; 2] = core::mem::zeroed();
    highbd_sse_8x1_init_neon(src, ref_, &raw mut sse[0], &raw mut sse[1]);
    src = src.offset(((src_stride)) as isize);
    ref_ = ref_.offset(((ref_stride)) as isize);
    while { height -= 1; height } != 0 {
        highbd_sse_8x1_neon(src, ref_, &raw mut sse[0], &raw mut sse[1]);
        src = src.offset(((src_stride)) as isize);
        ref_ = ref_.offset(((ref_stride)) as isize);
    }
    return horizontal_long_add_u32x4_x2(sse.as_mut_ptr()) as i64;
}

#[inline(always)]
unsafe fn highbd_sse_4xh_neon(mut src: *const u16, mut src_stride: i32, mut ref_: *const u16, mut ref_stride: i32, mut height: i32) -> i64 {
    // Peel the first loop iteration.
    let mut s: uint16x4_t = vld1_u16(src);
    let mut r: uint16x4_t = vld1_u16(ref_);
    let mut abs_diff: uint16x4_t = vabd_u16(s, r);
    let mut sse: uint32x4_t = vmull_u16(abs_diff, abs_diff);
    src = src.offset(((src_stride)) as isize);
    ref_ = ref_.offset(((ref_stride)) as isize);
    while { height -= 1; height } != 0 {
        s = vld1_u16(src);
        r = vld1_u16(ref_);
        abs_diff = vabd_u16(s, r);
        sse = vmlal_u16(sse, abs_diff, abs_diff);
        src = src.offset(((src_stride)) as isize);
        ref_ = ref_.offset(((ref_stride)) as isize);
    }
    return horizontal_long_add_u32x4(sse) as i64;
}

#[inline(always)]
unsafe fn highbd_sse_wxh_neon(mut src: *const u16, mut src_stride: i32, mut ref_: *const u16, mut ref_stride: i32, mut width: i32, mut height: i32) -> i64 {
    // { 0, 1, 2, 3, 4, 5, 6, 7 }
    let mut k01234567: uint16x8_t = vmovl_u8(vcreate_u8(0x0706050403020100));
    let mut remainder_mask: uint16x8_t = vcltq_u16(k01234567, vdupq_n_u16((width & 7) as u16));
    let mut sse: u64 = 0;
    loop {
        let mut w: i32 = width;
        let mut offset: i32 = 0;
        loop {
            let mut s: uint16x8_t = vld1q_u16(src.offset(((offset)) as isize));
            let mut r: uint16x8_t = vld1q_u16(ref_.offset(((offset)) as isize));
            if w < 8 {
                // Mask out-of-range elements.
                s = vandq_u16(s, remainder_mask);
                r = vandq_u16(r, remainder_mask);
            }
            let mut abs_diff: uint16x8_t = vabdq_u16(s, r);
            let mut abs_diff_lo: uint16x4_t = vget_low_u16(abs_diff);
            let mut abs_diff_hi: uint16x4_t = vget_high_u16(abs_diff);
            let mut sse_u32: uint32x4_t = vmull_u16(abs_diff_lo, abs_diff_lo);
            sse_u32 = vmlal_u16(sse_u32, abs_diff_hi, abs_diff_hi);
            sse += horizontal_long_add_u32x4(sse_u32);
            offset += 8;
            w -= 8;
            if !(w > 0) { break; }
        }
        src = src.offset(((src_stride)) as isize);
        ref_ = ref_.offset(((ref_stride)) as isize);
        if !({ height -= 1; height } != 0) { break; }
    }
    return sse as i64;
}

#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_sse_neon(mut src8: *const u8, mut src_stride: i32, mut ref8: *const u8, mut ref_stride: i32, mut width: i32, mut height: i32) -> i64 {
    let mut src: *mut u16 = CONVERT_TO_SHORTPTR(src8);
    let mut ref_: *mut u16 = CONVERT_TO_SHORTPTR(ref8);
    match width {
        4 => {
            return highbd_sse_4xh_neon(src, src_stride, ref_, ref_stride, height);
        }
        8 => {
            return highbd_sse_8xh_neon(src, src_stride, ref_, ref_stride, height);
        }
        16 => {
            return highbd_sse_16xh_neon(src, src_stride, ref_, ref_stride, height);
        }
        32 => {
            return highbd_sse_32xh_neon(src, src_stride, ref_, ref_stride, height);
        }
        64 => {
            return highbd_sse_64xh_neon(src, src_stride, ref_, ref_stride, height);
        }
        128 => {
            return highbd_sse_128xh_neon(src, src_stride, ref_, ref_stride, height);
        }
        _ => {
            return highbd_sse_wxh_neon(src, src_stride, ref_, ref_stride, width, height);
        }
    }
}

