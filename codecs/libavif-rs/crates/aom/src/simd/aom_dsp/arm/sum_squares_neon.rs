/*
 * Copyright (c) 2019, Alliance for Open Media. All rights reserved.
 *
 * This source code is subject to the terms of the BSD 2 Clause License and
 * the Alliance for Open Media Patent License 1.0. If the BSD 2 Clause License
 * was not distributed with this source code in the LICENSE file, you can
 * obtain it at www.aomedia.org/license/software. If the Alliance for Open
 * Media Patent License 1.0 was not distributed with this source code in the
 * PATENTS file, you can obtain it at www.aomedia.org/license/patent.
 */

// Translated by hand from upstream/aom/aom_dsp/arm/sum_squares_neon.c
// (ADR 0056). Intrinsic sequences, helper structure, and names follow the
// original; only the AOM_ARCH_AARCH64 branches are retained (the selected
// configuration has CONFIG_AV1_HIGHBITDEPTH set). C locals that the original
// leaves uninitialized start zeroed here; LIKELY() is a branch hint only.
// Unsigned C arithmetic wraps, so the variance results use wrapping
// operations.
#![allow(unused_mut, unused_parens, unused_variables, unused_braces, clippy::all)]

use core::arch::aarch64::*;

use crate::simd::aom_dsp__arm__mem_neon::load_unaligned_u8;
use crate::simd::aom_dsp__arm__sum_neon::*;

extern "C-unwind" {
    fn aom_sum_squares_2d_i16_c(src: *const i16, stride: i32, width: i32, height: i32) -> u64;
    fn aom_sum_sse_2d_i16_c(src: *const i16, stride: i32, width: i32, height: i32,
                            sum: *mut i32) -> u64;
    fn aom_sum_squares_i16_c(src: *const i16, n: u32) -> u64;
    fn aom_var_2d_u8_c(src: *mut u8, src_stride: i32, width: i32, height: i32) -> u64;
    fn aom_var_2d_u16_c(src: *mut u8, src_stride: i32, width: i32, height: i32) -> u64;
}

// aom_dsp/aom_dsp_common.h
#[inline(always)]
fn CONVERT_TO_SHORTPTR(x: *const u8) -> *mut u16 {
    ((x as usize) << 1) as *mut u16
}

#[inline(always)]
unsafe fn aom_sum_squares_2d_i16_4x4_neon(mut src: *const i16, mut stride: i32) -> u64 {
    let mut s0: int16x4_t = vld1_s16(src.offset(((0 * stride)) as isize));
    let mut s1: int16x4_t = vld1_s16(src.offset(((1 * stride)) as isize));
    let mut s2: int16x4_t = vld1_s16(src.offset(((2 * stride)) as isize));
    let mut s3: int16x4_t = vld1_s16(src.offset(((3 * stride)) as isize));
    let mut sum_squares: int32x4_t = vmull_s16(s0, s0);
    sum_squares = vmlal_s16(sum_squares, s1, s1);
    sum_squares = vmlal_s16(sum_squares, s2, s2);
    sum_squares = vmlal_s16(sum_squares, s3, s3);
    return horizontal_long_add_u32x4(vreinterpretq_u32_s32(sum_squares));
}

#[inline(always)]
unsafe fn aom_sum_squares_2d_i16_4xn_neon(mut src: *const i16, mut stride: i32, mut height: i32) -> u64 {
    let mut sum_squares: [int32x4_t; 2] = [vdupq_n_s32(0), vdupq_n_s32(0)];
    let mut h: i32 = height;
    loop {
        let mut s0: int16x4_t = vld1_s16(src.offset(((0 * stride)) as isize));
        let mut s1: int16x4_t = vld1_s16(src.offset(((1 * stride)) as isize));
        let mut s2: int16x4_t = vld1_s16(src.offset(((2 * stride)) as isize));
        let mut s3: int16x4_t = vld1_s16(src.offset(((3 * stride)) as isize));
        sum_squares[0] = vmlal_s16(sum_squares[0], s0, s0);
        sum_squares[0] = vmlal_s16(sum_squares[0], s1, s1);
        sum_squares[1] = vmlal_s16(sum_squares[1], s2, s2);
        sum_squares[1] = vmlal_s16(sum_squares[1], s3, s3);
        src = src.offset(((4 * stride)) as isize);
        h -= 4;
        if !(h != 0) { break; }
    }
    return horizontal_long_add_u32x4(vreinterpretq_u32_s32(vaddq_s32(sum_squares[0], sum_squares[1])));
}

#[inline(always)]
unsafe fn aom_sum_squares_2d_i16_nxn_neon(mut src: *const i16, mut stride: i32, mut width: i32, mut height: i32) -> u64 {
    let mut sum_squares: uint64x2_t = vdupq_n_u64(0);
    let mut h: i32 = height;
    loop {
        let mut ss_row: [int32x4_t; 2] = [vdupq_n_s32(0), vdupq_n_s32(0)];
        let mut w: i32 = 0;
        loop {
            let mut s: *const i16 = src.offset(w as isize);
            let mut s0: int16x8_t = vld1q_s16(s.offset(((0 * stride)) as isize));
            let mut s1: int16x8_t = vld1q_s16(s.offset(((1 * stride)) as isize));
            let mut s2: int16x8_t = vld1q_s16(s.offset(((2 * stride)) as isize));
            let mut s3: int16x8_t = vld1q_s16(s.offset(((3 * stride)) as isize));
            ss_row[0] = vmlal_s16(ss_row[0], vget_low_s16(s0), vget_low_s16(s0));
            ss_row[0] = vmlal_s16(ss_row[0], vget_low_s16(s1), vget_low_s16(s1));
            ss_row[0] = vmlal_s16(ss_row[0], vget_low_s16(s2), vget_low_s16(s2));
            ss_row[0] = vmlal_s16(ss_row[0], vget_low_s16(s3), vget_low_s16(s3));
            ss_row[1] = vmlal_s16(ss_row[1], vget_high_s16(s0), vget_high_s16(s0));
            ss_row[1] = vmlal_s16(ss_row[1], vget_high_s16(s1), vget_high_s16(s1));
            ss_row[1] = vmlal_s16(ss_row[1], vget_high_s16(s2), vget_high_s16(s2));
            ss_row[1] = vmlal_s16(ss_row[1], vget_high_s16(s3), vget_high_s16(s3));
            w += 8;
            if !(w < width) { break; }
        }
        sum_squares = vpadalq_u32(sum_squares, vreinterpretq_u32_s32(vaddq_s32(ss_row[0], ss_row[1])));
        src = src.offset(((4 * stride)) as isize);
        h -= 4;
        if !(h != 0) { break; }
    }
    return horizontal_add_u64x2(sum_squares);
}

#[no_mangle]
pub unsafe extern "C-unwind" fn aom_sum_squares_2d_i16_neon(mut src: *const i16, mut stride: i32, mut width: i32, mut height: i32) -> u64 {
    // 4 elements per row only requires half an SIMD register, so this
    // must be a special case, but also note that over 75% of all calls
    // are with size == 4, so it is also the common case.
    if width == 4 && height == 4 {
        return aom_sum_squares_2d_i16_4x4_neon(src, stride);
    } else if width == 4 && (height & 3) == 0 {
        return aom_sum_squares_2d_i16_4xn_neon(src, stride, height);
    } else if (width & 7) == 0 && (height & 3) == 0 {
        // Generic case
        return aom_sum_squares_2d_i16_nxn_neon(src, stride, width, height);
    } else {
        return aom_sum_squares_2d_i16_c(src, stride, width, height);
    }
}

#[inline(always)]
unsafe fn aom_sum_sse_2d_i16_4x4_neon(mut src: *const i16, mut stride: i32, mut sum: *mut i32) -> u64 {
    let mut s0: int16x4_t = vld1_s16(src.offset(((0 * stride)) as isize));
    let mut s1: int16x4_t = vld1_s16(src.offset(((1 * stride)) as isize));
    let mut s2: int16x4_t = vld1_s16(src.offset(((2 * stride)) as isize));
    let mut s3: int16x4_t = vld1_s16(src.offset(((3 * stride)) as isize));
    let mut sse: int32x4_t = vmull_s16(s0, s0);
    sse = vmlal_s16(sse, s1, s1);
    sse = vmlal_s16(sse, s2, s2);
    sse = vmlal_s16(sse, s3, s3);
    let mut sum_01: int32x4_t = vaddl_s16(s0, s1);
    let mut sum_23: int32x4_t = vaddl_s16(s2, s3);
    *sum += horizontal_add_s32x4(vaddq_s32(sum_01, sum_23));
    return horizontal_long_add_u32x4(vreinterpretq_u32_s32(sse));
}

#[inline(always)]
unsafe fn aom_sum_sse_2d_i16_4xn_neon(mut src: *const i16, mut stride: i32, mut height: i32, mut sum: *mut i32) -> u64 {
    let mut sse: [int32x4_t; 2] = [vdupq_n_s32(0), vdupq_n_s32(0)];
    let mut sum_acc: [int32x2_t; 2] = [vdup_n_s32(0), vdup_n_s32(0)];
    let mut h: i32 = height;
    loop {
        let mut s0: int16x4_t = vld1_s16(src.offset(((0 * stride)) as isize));
        let mut s1: int16x4_t = vld1_s16(src.offset(((1 * stride)) as isize));
        let mut s2: int16x4_t = vld1_s16(src.offset(((2 * stride)) as isize));
        let mut s3: int16x4_t = vld1_s16(src.offset(((3 * stride)) as isize));
        sse[0] = vmlal_s16(sse[0], s0, s0);
        sse[0] = vmlal_s16(sse[0], s1, s1);
        sse[1] = vmlal_s16(sse[1], s2, s2);
        sse[1] = vmlal_s16(sse[1], s3, s3);
        sum_acc[0] = vpadal_s16(sum_acc[0], s0);
        sum_acc[0] = vpadal_s16(sum_acc[0], s1);
        sum_acc[1] = vpadal_s16(sum_acc[1], s2);
        sum_acc[1] = vpadal_s16(sum_acc[1], s3);
        src = src.offset(((4 * stride)) as isize);
        h -= 4;
        if !(h != 0) { break; }
    }
    *sum += horizontal_add_s32x4(vcombine_s32(sum_acc[0], sum_acc[1]));
    return horizontal_long_add_u32x4(vreinterpretq_u32_s32(vaddq_s32(sse[0], sse[1])));
}

#[inline(always)]
unsafe fn aom_sum_sse_2d_i16_nxn_neon(mut src: *const i16, mut stride: i32, mut width: i32, mut height: i32, mut sum: *mut i32) -> u64 {
    let mut sse: uint64x2_t = vdupq_n_u64(0);
    let mut sum_acc: int32x4_t = vdupq_n_s32(0);
    let mut h: i32 = height;
    loop {
        let mut sse_row: [int32x4_t; 2] = [vdupq_n_s32(0), vdupq_n_s32(0)];
        let mut w: i32 = 0;
        loop {
            let mut s: *const i16 = src.offset(w as isize);
            let mut s0: int16x8_t = vld1q_s16(s.offset(((0 * stride)) as isize));
            let mut s1: int16x8_t = vld1q_s16(s.offset(((1 * stride)) as isize));
            let mut s2: int16x8_t = vld1q_s16(s.offset(((2 * stride)) as isize));
            let mut s3: int16x8_t = vld1q_s16(s.offset(((3 * stride)) as isize));
            sse_row[0] = vmlal_s16(sse_row[0], vget_low_s16(s0), vget_low_s16(s0));
            sse_row[0] = vmlal_s16(sse_row[0], vget_low_s16(s1), vget_low_s16(s1));
            sse_row[0] = vmlal_s16(sse_row[0], vget_low_s16(s2), vget_low_s16(s2));
            sse_row[0] = vmlal_s16(sse_row[0], vget_low_s16(s3), vget_low_s16(s3));
            sse_row[1] = vmlal_s16(sse_row[1], vget_high_s16(s0), vget_high_s16(s0));
            sse_row[1] = vmlal_s16(sse_row[1], vget_high_s16(s1), vget_high_s16(s1));
            sse_row[1] = vmlal_s16(sse_row[1], vget_high_s16(s2), vget_high_s16(s2));
            sse_row[1] = vmlal_s16(sse_row[1], vget_high_s16(s3), vget_high_s16(s3));
            sum_acc = vpadalq_s16(sum_acc, s0);
            sum_acc = vpadalq_s16(sum_acc, s1);
            sum_acc = vpadalq_s16(sum_acc, s2);
            sum_acc = vpadalq_s16(sum_acc, s3);
            w += 8;
            if !(w < width) { break; }
        }
        sse = vpadalq_u32(sse, vreinterpretq_u32_s32(vaddq_s32(sse_row[0], sse_row[1])));
        src = src.offset(((4 * stride)) as isize);
        h -= 4;
        if !(h != 0) { break; }
    }
    *sum += horizontal_add_s32x4(sum_acc);
    return horizontal_add_u64x2(sse);
}

#[no_mangle]
pub unsafe extern "C-unwind" fn aom_sum_sse_2d_i16_neon(mut src: *const i16, mut stride: i32, mut width: i32, mut height: i32, mut sum: *mut i32) -> u64 {
    let mut sse: u64 = core::mem::zeroed();
    if width == 4 && height == 4 {
        sse = aom_sum_sse_2d_i16_4x4_neon(src, stride, sum);
    } else if width == 4 && (height & 3) == 0 {
        // width = 4, height is a multiple of 4.
        sse = aom_sum_sse_2d_i16_4xn_neon(src, stride, height, sum);
    } else if (width & 7) == 0 && (height & 3) == 0 {
        // Generic case - width is multiple of 8, height is multiple of 4.
        sse = aom_sum_sse_2d_i16_nxn_neon(src, stride, width, height, sum);
    } else {
        sse = aom_sum_sse_2d_i16_c(src, stride, width, height, sum);
    }
    return sse;
}

#[inline(always)]
unsafe fn aom_sum_squares_i16_4xn_neon(mut src: *const i16, mut n: u32) -> u64 {
    let mut sum_u64: uint64x2_t = vdupq_n_u64(0);
    let mut i: i32 = n as i32;
    loop {
        let mut sum: uint32x4_t = core::mem::zeroed();
        let mut s0: int16x4_t = vld1_s16(src);
        sum = vreinterpretq_u32_s32(vmull_s16(s0, s0));
        sum_u64 = vpadalq_u32(sum_u64, sum);
        src = src.offset(((4)) as isize);
        i -= 4;
        if !(i >= 4) { break; }
    }
    if i > 0 {
        return horizontal_add_u64x2(sum_u64) + aom_sum_squares_i16_c(src, i as u32);
    }
    return horizontal_add_u64x2(sum_u64);
}

#[inline(always)]
unsafe fn aom_sum_squares_i16_8xn_neon(mut src: *const i16, mut n: u32) -> u64 {
    let mut sum_u64: [uint64x2_t; 2] = [vdupq_n_u64(0), vdupq_n_u64(0)];
    let mut i: i32 = n as i32;
    loop {
        let mut sum: [uint32x4_t; 2] = core::mem::zeroed();
        let mut s0: int16x8_t = vld1q_s16(src);
        sum[0] = vreinterpretq_u32_s32(vmull_s16(vget_low_s16(s0), vget_low_s16(s0)));
        sum[1] = vreinterpretq_u32_s32(vmull_s16(vget_high_s16(s0), vget_high_s16(s0)));
        sum_u64[0] = vpadalq_u32(sum_u64[0], sum[0]);
        sum_u64[1] = vpadalq_u32(sum_u64[1], sum[1]);
        src = src.offset(((8)) as isize);
        i -= 8;
        if !(i >= 8) { break; }
    }
    if i > 0 {
        return horizontal_add_u64x2(vaddq_u64(sum_u64[0], sum_u64[1])) + aom_sum_squares_i16_c(src, i as u32);
    }
    return horizontal_add_u64x2(vaddq_u64(sum_u64[0], sum_u64[1]));
}

#[no_mangle]
pub unsafe extern "C-unwind" fn aom_sum_squares_i16_neon(mut src: *const i16, mut n: u32) -> u64 {
    // This function seems to be called only for values of N >= 64. See
    // av1/encoder/compound_type.c.
    if n >= 8 {
        return aom_sum_squares_i16_8xn_neon(src, n);
    }
    if n >= 4 {
        return aom_sum_squares_i16_4xn_neon(src, n);
    }
    return aom_sum_squares_i16_c(src, n);
}

#[inline(always)]
unsafe fn aom_var_2d_u8_4xh_neon(mut src: *mut u8, mut src_stride: i32, mut width: i32, mut height: i32) -> u64 {
    let mut sum: u64 = 0;
    let mut sse: u64 = 0;
    let mut sum_u32: uint32x2_t = vdup_n_u32(0);
    let mut sse_u32: uint32x4_t = vdupq_n_u32(0);
    // 255*256 = 65280, so we can accumulate up to 256 8-bit elements in a 16-bit
    // element before we need to accumulate to 32-bit elements. Since we're
    // accumulating in uint16x4_t vectors, this means we can accumulate up to 4
    // rows of 256 elements. Therefore the limit can be computed as: h_limit = (4
    // * 256) / width.
    let mut h_limit: i32 = (4 * 256) / width;
    let mut h_tmp: i32 = if height > h_limit { h_limit } else { height };
    let mut h: i32 = 0;
    loop {
        let mut sum_u16: uint16x4_t = vdup_n_u16(0);
        loop {
            let mut src_ptr: *mut u8 = src;
            let mut w: i32 = width;
            loop {
                let mut s0: uint8x8_t = load_unaligned_u8(src_ptr, src_stride as isize);
                sum_u16 = vpadal_u8(sum_u16, s0);
                let mut sse_u16: uint16x8_t = vmull_u8(s0, s0);
                sse_u32 = vpadalq_u16(sse_u32, sse_u16);
                src_ptr = src_ptr.offset(((8)) as isize);
                w -= 8;
                if !(w >= 8) { break; }
            }
            // Process remaining columns in the row using C.
            while w > 0 {
                let mut idx: i32 = width - w;
                let mut v: u8 = *src.offset(idx as isize);
                sum += v as u64;
                sse += (v as i32 * v as i32) as u64;
                w -= 1;
            }
            src = src.offset(((2 * src_stride)) as isize);
            h += 2;
            if !(h < h_tmp && h < height) { break; }
        }
        sum_u32 = vpadal_u16(sum_u32, sum_u16);
        h_tmp += h_limit;
        if !(h < height) { break; }
    }
    sum += horizontal_long_add_u32x2(sum_u32);
    sse += horizontal_long_add_u32x4(sse_u32);
    return sse.wrapping_sub(sum.wrapping_mul(sum) / (width * height) as u64);
}

#[inline(always)]
unsafe fn aom_var_2d_u8_8xh_neon(mut src: *mut u8, mut src_stride: i32, mut width: i32, mut height: i32) -> u64 {
    let mut sum: u64 = 0;
    let mut sse: u64 = 0;
    let mut sum_u32: uint32x2_t = vdup_n_u32(0);
    let mut sse_u32: uint32x4_t = vdupq_n_u32(0);
    // 255*256 = 65280, so we can accumulate up to 256 8-bit elements in a 16-bit
    // element before we need to accumulate to 32-bit elements. Since we're
    // accumulating in uint16x4_t vectors, this means we can accumulate up to 4
    // rows of 256 elements. Therefore the limit can be computed as: h_limit = (4
    // * 256) / width.
    let mut h_limit: i32 = (4 * 256) / width;
    let mut h_tmp: i32 = if height > h_limit { h_limit } else { height };
    let mut h: i32 = 0;
    loop {
        let mut sum_u16: uint16x4_t = vdup_n_u16(0);
        loop {
            let mut src_ptr: *mut u8 = src;
            let mut w: i32 = width;
            loop {
                let mut s0: uint8x8_t = vld1_u8(src_ptr);
                sum_u16 = vpadal_u8(sum_u16, s0);
                let mut sse_u16: uint16x8_t = vmull_u8(s0, s0);
                sse_u32 = vpadalq_u16(sse_u32, sse_u16);
                src_ptr = src_ptr.offset(((8)) as isize);
                w -= 8;
                if !(w >= 8) { break; }
            }
            // Process remaining columns in the row using C.
            while w > 0 {
                let mut idx: i32 = width - w;
                let mut v: u8 = *src.offset(idx as isize);
                sum += v as u64;
                sse += (v as i32 * v as i32) as u64;
                w -= 1;
            }
            src = src.offset(((src_stride)) as isize);
            h += 1;
            if !(h < h_tmp && h < height) { break; }
        }
        sum_u32 = vpadal_u16(sum_u32, sum_u16);
        h_tmp += h_limit;
        if !(h < height) { break; }
    }
    sum += horizontal_long_add_u32x2(sum_u32);
    sse += horizontal_long_add_u32x4(sse_u32);
    return sse.wrapping_sub(sum.wrapping_mul(sum) / (width * height) as u64);
}

#[inline(always)]
unsafe fn aom_var_2d_u8_16xh_neon(mut src: *mut u8, mut src_stride: i32, mut width: i32, mut height: i32) -> u64 {
    let mut sum: u64 = 0;
    let mut sse: u64 = 0;
    let mut sum_u32: uint32x4_t = vdupq_n_u32(0);
    let mut sse_u32: [uint32x4_t; 2] = [vdupq_n_u32(0), vdupq_n_u32(0)];
    // 255*256 = 65280, so we can accumulate up to 256 8-bit elements in a 16-bit
    // element before we need to accumulate to 32-bit elements. Since we're
    // accumulating in uint16x8_t vectors, this means we can accumulate up to 8
    // rows of 256 elements. Therefore the limit can be computed as: h_limit = (8
    // * 256) / width.
    let mut h_limit: i32 = (8 * 256) / width;
    let mut h_tmp: i32 = if height > h_limit { h_limit } else { height };
    let mut h: i32 = 0;
    loop {
        let mut sum_u16: uint16x8_t = vdupq_n_u16(0);
        loop {
            let mut w: i32 = width;
            let mut src_ptr: *mut u8 = src;
            loop {
                let mut s0: uint8x16_t = vld1q_u8(src_ptr);
                sum_u16 = vpadalq_u8(sum_u16, s0);
                let mut sse_u16_lo: uint16x8_t = vmull_u8(vget_low_u8(s0), vget_low_u8(s0));
                let mut sse_u16_hi: uint16x8_t = vmull_u8(vget_high_u8(s0), vget_high_u8(s0));
                sse_u32[0] = vpadalq_u16(sse_u32[0], sse_u16_lo);
                sse_u32[1] = vpadalq_u16(sse_u32[1], sse_u16_hi);
                src_ptr = src_ptr.offset(((16)) as isize);
                w -= 16;
                if !(w >= 16) { break; }
            }
            // Process remaining columns in the row using C.
            while w > 0 {
                let mut idx: i32 = width - w;
                let mut v: u8 = *src.offset(idx as isize);
                sum += v as u64;
                sse += (v as i32 * v as i32) as u64;
                w -= 1;
            }
            src = src.offset(((src_stride)) as isize);
            h += 1;
            if !(h < h_tmp && h < height) { break; }
        }
        sum_u32 = vpadalq_u16(sum_u32, sum_u16);
        h_tmp += h_limit;
        if !(h < height) { break; }
    }
    sum += horizontal_long_add_u32x4(sum_u32);
    sse += horizontal_long_add_u32x4(vaddq_u32(sse_u32[0], sse_u32[1]));
    return sse.wrapping_sub(sum.wrapping_mul(sum) / (width * height) as u64);
}

#[no_mangle]
pub unsafe extern "C-unwind" fn aom_var_2d_u8_neon(mut src: *mut u8, mut src_stride: i32, mut width: i32, mut height: i32) -> u64 {
    if width >= 16 {
        return aom_var_2d_u8_16xh_neon(src, src_stride, width, height);
    }
    if width >= 8 {
        return aom_var_2d_u8_8xh_neon(src, src_stride, width, height);
    }
    if width >= 4 && height % 2 == 0 {
        return aom_var_2d_u8_4xh_neon(src, src_stride, width, height);
    }
    return aom_var_2d_u8_c(src, src_stride, width, height);
}

// #if CONFIG_AV1_HIGHBITDEPTH
#[inline(always)]
unsafe fn aom_var_2d_u16_4xh_neon(mut src: *mut u8, mut src_stride: i32, mut width: i32, mut height: i32) -> u64 {
    let mut src_u16: *mut u16 = CONVERT_TO_SHORTPTR(src);
    let mut sum: u64 = 0;
    let mut sse: u64 = 0;
    let mut sum_u32: uint32x2_t = vdup_n_u32(0);
    let mut sse_u64: uint64x2_t = vdupq_n_u64(0);
    let mut h: i32 = height;
    loop {
        let mut w: i32 = width;
        let mut src_ptr: *mut u16 = src_u16;
        loop {
            let mut s0: uint16x4_t = vld1_u16(src_ptr);
            sum_u32 = vpadal_u16(sum_u32, s0);
            let mut sse_u32: uint32x4_t = vmull_u16(s0, s0);
            sse_u64 = vpadalq_u32(sse_u64, sse_u32);
            src_ptr = src_ptr.offset(((4)) as isize);
            w -= 4;
            if !(w >= 4) { break; }
        }
        // Process remaining columns in the row using C.
        while w > 0 {
            let mut idx: i32 = width - w;
            let mut v: u16 = *src_u16.offset(idx as isize);
            sum += v as u64;
            sse += (v as i32 * v as i32) as u64;
            w -= 1;
        }
        src_u16 = src_u16.offset(((src_stride)) as isize);
        if !({ h -= 1; h } != 0) { break; }
    }
    sum += horizontal_long_add_u32x2(sum_u32);
    sse += horizontal_add_u64x2(sse_u64);
    return sse.wrapping_sub(sum.wrapping_mul(sum) / (width * height) as u64);
}

#[inline(always)]
unsafe fn aom_var_2d_u16_8xh_neon(mut src: *mut u8, mut src_stride: i32, mut width: i32, mut height: i32) -> u64 {
    let mut src_u16: *mut u16 = CONVERT_TO_SHORTPTR(src);
    let mut sum: u64 = 0;
    let mut sse: u64 = 0;
    let mut sum_u32: uint32x4_t = vdupq_n_u32(0);
    let mut sse_u64: [uint64x2_t; 2] = [vdupq_n_u64(0), vdupq_n_u64(0)];
    let mut h: i32 = height;
    loop {
        let mut w: i32 = width;
        let mut src_ptr: *mut u16 = src_u16;
        loop {
            let mut s0: uint16x8_t = vld1q_u16(src_ptr);
            sum_u32 = vpadalq_u16(sum_u32, s0);
            let mut sse_u32_lo: uint32x4_t = vmull_u16(vget_low_u16(s0), vget_low_u16(s0));
            let mut sse_u32_hi: uint32x4_t = vmull_u16(vget_high_u16(s0), vget_high_u16(s0));
            sse_u64[0] = vpadalq_u32(sse_u64[0], sse_u32_lo);
            sse_u64[1] = vpadalq_u32(sse_u64[1], sse_u32_hi);
            src_ptr = src_ptr.offset(((8)) as isize);
            w -= 8;
            if !(w >= 8) { break; }
        }
        // Process remaining columns in the row using C.
        while w > 0 {
            let mut idx: i32 = width - w;
            let mut v: u16 = *src_u16.offset(idx as isize);
            sum += v as u64;
            sse += (v as i32 * v as i32) as u64;
            w -= 1;
        }
        src_u16 = src_u16.offset(((src_stride)) as isize);
        if !({ h -= 1; h } != 0) { break; }
    }
    sum += horizontal_long_add_u32x4(sum_u32);
    sse += horizontal_add_u64x2(vaddq_u64(sse_u64[0], sse_u64[1]));
    return sse.wrapping_sub(sum.wrapping_mul(sum) / (width * height) as u64);
}

#[no_mangle]
pub unsafe extern "C-unwind" fn aom_var_2d_u16_neon(mut src: *mut u8, mut src_stride: i32, mut width: i32, mut height: i32) -> u64 {
    if width >= 8 {
        return aom_var_2d_u16_8xh_neon(src, src_stride, width, height);
    }
    if width >= 4 {
        return aom_var_2d_u16_4xh_neon(src, src_stride, width, height);
    }
    return aom_var_2d_u16_c(src, src_stride, width, height);
}

