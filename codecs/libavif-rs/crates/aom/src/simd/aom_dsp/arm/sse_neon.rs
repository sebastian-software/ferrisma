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

// Translated by hand from upstream/aom/aom_dsp/arm/sse_neon.c
// (ADR 0056). Intrinsic sequences, helper structure, and names follow the
// original; only the AOM_ARCH_AARCH64 branches are retained. C locals that the
// original leaves uninitialized start zeroed here.
#![allow(unused_mut, unused_parens, unused_variables, unused_braces, clippy::all)]

use core::arch::aarch64::*;

use crate::simd::aom_dsp__arm__mem_neon::load_unaligned_u8;
use crate::simd::aom_dsp__arm__sum_neon::horizontal_add_u32x4;

#[inline(always)]
unsafe fn sse_16x1_neon(mut src: *const u8, mut ref_: *const u8, mut sse: *mut uint32x4_t) {
    let mut s: uint8x16_t = vld1q_u8(src);
    let mut r: uint8x16_t = vld1q_u8(ref_);
    let mut abs_diff: uint8x16_t = vabdq_u8(s, r);
    let mut abs_diff_lo: uint8x8_t = vget_low_u8(abs_diff);
    let mut abs_diff_hi: uint8x8_t = vget_high_u8(abs_diff);
    *sse = vpadalq_u16(*sse, vmull_u8(abs_diff_lo, abs_diff_lo));
    *sse = vpadalq_u16(*sse, vmull_u8(abs_diff_hi, abs_diff_hi));
}

#[inline(always)]
unsafe fn sse_8x1_neon(mut src: *const u8, mut ref_: *const u8, mut sse: *mut uint32x4_t) {
    let mut s: uint8x8_t = vld1_u8(src);
    let mut r: uint8x8_t = vld1_u8(ref_);
    let mut abs_diff: uint8x8_t = vabd_u8(s, r);
    *sse = vpadalq_u16(*sse, vmull_u8(abs_diff, abs_diff));
}

#[inline(always)]
unsafe fn sse_4x2_neon(mut src: *const u8, mut src_stride: i32, mut ref_: *const u8, mut ref_stride: i32, mut sse: *mut uint32x4_t) {
    let mut s: uint8x8_t = load_unaligned_u8(src, src_stride as isize);
    let mut r: uint8x8_t = load_unaligned_u8(ref_, ref_stride as isize);
    let mut abs_diff: uint8x8_t = vabd_u8(s, r);
    *sse = vpadalq_u16(*sse, vmull_u8(abs_diff, abs_diff));
}

#[inline(always)]
unsafe fn sse_wxh_neon(mut src: *const u8, mut src_stride: i32, mut ref_: *const u8, mut ref_stride: i32, mut width: i32, mut height: i32) -> u32 {
    let mut sse: uint32x4_t = vdupq_n_u32(0);
    if (width & 0x07) != 0 && ((width & 0x07) < 5) {
        let mut i: i32 = height;
        loop {
            let mut j: i32 = 0;
            loop {
                sse_8x1_neon(src.offset(((j)) as isize), ref_.offset(((j)) as isize), &raw mut sse);
                sse_8x1_neon(src.offset(((j + src_stride)) as isize), ref_.offset(((j + ref_stride)) as isize), &raw mut sse);
                j += 8;
                if !(j + 4 < width) { break; }
            }
            sse_4x2_neon(src.offset(((j)) as isize), src_stride, ref_.offset(((j)) as isize), ref_stride, &raw mut sse);
            src = src.offset(((2 * src_stride)) as isize);
            ref_ = ref_.offset(((2 * ref_stride)) as isize);
            i -= 2;
            if !(i != 0) { break; }
        }
    } else {
        let mut i: i32 = height;
        loop {
            let mut j: i32 = 0;
            loop {
                sse_8x1_neon(src.offset(((j)) as isize), ref_.offset(((j)) as isize), &raw mut sse);
                j += 8;
                if !(j < width) { break; }
            }
            src = src.offset(((src_stride)) as isize);
            ref_ = ref_.offset(((ref_stride)) as isize);
            if !({ i -= 1; i } != 0) { break; }
        }
    }
    return horizontal_add_u32x4(sse);
}

#[inline(always)]
unsafe fn sse_128xh_neon(mut src: *const u8, mut src_stride: i32, mut ref_: *const u8, mut ref_stride: i32, mut height: i32) -> u32 {
    let mut sse: [uint32x4_t; 2] = [vdupq_n_u32(0), vdupq_n_u32(0)];
    let mut i: i32 = height;
    loop {
        sse_16x1_neon(src, ref_, &raw mut sse[0]);
        sse_16x1_neon(src.offset(((16)) as isize), ref_.offset(((16)) as isize), &raw mut sse[1]);
        sse_16x1_neon(src.offset(((32)) as isize), ref_.offset(((32)) as isize), &raw mut sse[0]);
        sse_16x1_neon(src.offset(((48)) as isize), ref_.offset(((48)) as isize), &raw mut sse[1]);
        sse_16x1_neon(src.offset(((64)) as isize), ref_.offset(((64)) as isize), &raw mut sse[0]);
        sse_16x1_neon(src.offset(((80)) as isize), ref_.offset(((80)) as isize), &raw mut sse[1]);
        sse_16x1_neon(src.offset(((96)) as isize), ref_.offset(((96)) as isize), &raw mut sse[0]);
        sse_16x1_neon(src.offset(((112)) as isize), ref_.offset(((112)) as isize), &raw mut sse[1]);
        src = src.offset(((src_stride)) as isize);
        ref_ = ref_.offset(((ref_stride)) as isize);
        if !({ i -= 1; i } != 0) { break; }
    }
    return horizontal_add_u32x4(vaddq_u32(sse[0], sse[1]));
}

#[inline(always)]
unsafe fn sse_64xh_neon(mut src: *const u8, mut src_stride: i32, mut ref_: *const u8, mut ref_stride: i32, mut height: i32) -> u32 {
    let mut sse: [uint32x4_t; 2] = [vdupq_n_u32(0), vdupq_n_u32(0)];
    let mut i: i32 = height;
    loop {
        sse_16x1_neon(src, ref_, &raw mut sse[0]);
        sse_16x1_neon(src.offset(((16)) as isize), ref_.offset(((16)) as isize), &raw mut sse[1]);
        sse_16x1_neon(src.offset(((32)) as isize), ref_.offset(((32)) as isize), &raw mut sse[0]);
        sse_16x1_neon(src.offset(((48)) as isize), ref_.offset(((48)) as isize), &raw mut sse[1]);
        src = src.offset(((src_stride)) as isize);
        ref_ = ref_.offset(((ref_stride)) as isize);
        if !({ i -= 1; i } != 0) { break; }
    }
    return horizontal_add_u32x4(vaddq_u32(sse[0], sse[1]));
}

#[inline(always)]
unsafe fn sse_32xh_neon(mut src: *const u8, mut src_stride: i32, mut ref_: *const u8, mut ref_stride: i32, mut height: i32) -> u32 {
    let mut sse: [uint32x4_t; 2] = [vdupq_n_u32(0), vdupq_n_u32(0)];
    let mut i: i32 = height;
    loop {
        sse_16x1_neon(src, ref_, &raw mut sse[0]);
        sse_16x1_neon(src.offset(((16)) as isize), ref_.offset(((16)) as isize), &raw mut sse[1]);
        src = src.offset(((src_stride)) as isize);
        ref_ = ref_.offset(((ref_stride)) as isize);
        if !({ i -= 1; i } != 0) { break; }
    }
    return horizontal_add_u32x4(vaddq_u32(sse[0], sse[1]));
}

#[inline(always)]
unsafe fn sse_16xh_neon(mut src: *const u8, mut src_stride: i32, mut ref_: *const u8, mut ref_stride: i32, mut height: i32) -> u32 {
    let mut sse: [uint32x4_t; 2] = [vdupq_n_u32(0), vdupq_n_u32(0)];
    let mut i: i32 = height;
    loop {
        sse_16x1_neon(src, ref_, &raw mut sse[0]);
        src = src.offset(((src_stride)) as isize);
        ref_ = ref_.offset(((ref_stride)) as isize);
        sse_16x1_neon(src, ref_, &raw mut sse[1]);
        src = src.offset(((src_stride)) as isize);
        ref_ = ref_.offset(((ref_stride)) as isize);
        i -= 2;
        if !(i != 0) { break; }
    }
    return horizontal_add_u32x4(vaddq_u32(sse[0], sse[1]));
}

#[inline(always)]
unsafe fn sse_8xh_neon(mut src: *const u8, mut src_stride: i32, mut ref_: *const u8, mut ref_stride: i32, mut height: i32) -> u32 {
    let mut sse: uint32x4_t = vdupq_n_u32(0);
    let mut i: i32 = height;
    loop {
        sse_8x1_neon(src, ref_, &raw mut sse);
        src = src.offset(((src_stride)) as isize);
        ref_ = ref_.offset(((ref_stride)) as isize);
        if !({ i -= 1; i } != 0) { break; }
    }
    return horizontal_add_u32x4(sse);
}

#[inline(always)]
unsafe fn sse_4xh_neon(mut src: *const u8, mut src_stride: i32, mut ref_: *const u8, mut ref_stride: i32, mut height: i32) -> u32 {
    let mut sse: uint32x4_t = vdupq_n_u32(0);
    let mut i: i32 = height;
    loop {
        sse_4x2_neon(src, src_stride, ref_, ref_stride, &raw mut sse);
        src = src.offset(((2 * src_stride)) as isize);
        ref_ = ref_.offset(((2 * ref_stride)) as isize);
        i -= 2;
        if !(i != 0) { break; }
    }
    return horizontal_add_u32x4(sse);
}

#[no_mangle]
pub unsafe extern "C-unwind" fn aom_sse_neon(mut src: *const u8, mut src_stride: i32, mut ref_: *const u8, mut ref_stride: i32, mut width: i32, mut height: i32) -> i64 {
    match width {
        4 => {
            return sse_4xh_neon(src, src_stride, ref_, ref_stride, height) as i64;
        }
        8 => {
            return sse_8xh_neon(src, src_stride, ref_, ref_stride, height) as i64;
        }
        16 => {
            return sse_16xh_neon(src, src_stride, ref_, ref_stride, height) as i64;
        }
        32 => {
            return sse_32xh_neon(src, src_stride, ref_, ref_stride, height) as i64;
        }
        64 => {
            return sse_64xh_neon(src, src_stride, ref_, ref_stride, height) as i64;
        }
        128 => {
            return sse_128xh_neon(src, src_stride, ref_, ref_stride, height) as i64;
        }
        _ => {
            return sse_wxh_neon(src, src_stride, ref_, ref_stride, width, height) as i64;
        }
    }
}

