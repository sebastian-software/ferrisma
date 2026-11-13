/*
 * Copyright (c) 2017, Alliance for Open Media. All rights reserved.
 *
 * This source code is subject to the terms of the BSD 2 Clause License and
 * the Alliance for Open Media Patent License 1.0. If the BSD 2 Clause License
 * was not distributed with this source code in the LICENSE file, you can
 * obtain it at www.aomedia.org/license/software. If the Alliance for Open
 * Media Patent License 1.0 was not distributed with this source code in the
 * PATENTS file, you can obtain it at www.aomedia.org/license/patent.
 */

// Translated by hand from upstream/aom/av1/common/arm/cfl_neon.c (ADR 0056).
// Function names, helper structure, and intrinsic sequences follow the
// original; only the AOM_ARCH_AARCH64 branches and the CONFIG_AV1_HIGHBITDEPTH
// configuration are retained. The size-specific wrappers and `cfl_get_*_neon`
// getters that the original generates with the CFL_GET_SUBSAMPLE_FUNCTION,
// CFL_SUB_AVG_FN, and CFL_PREDICT_FN macros of av1/common/cfl.h are generated
// by the `macro_rules!` below with the same names, sizes, and table layout.
// The original's `vld1_dup_u32` / `vst1_lane_u32` on `(uint32_t *)` casts of
// byte or halfword addresses become unaligned equivalents (same LD1R / ST1
// lane semantics without Rust's alignment requirement on `*const u32`), as in
// mem_neon.rs. Pointers that advance past the end of a row loop use wrapping
// arithmetic, as C computes but never dereferences them.
#![allow(non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![allow(clippy::all)]

use core::arch::aarch64::*;
use core::ptr;

use crate::av1__common__cfl::{
    cfl_predict_hbd_fn, cfl_predict_lbd_fn, cfl_subsample_hbd_fn, cfl_subsample_lbd_fn,
    cfl_subtract_average_fn,
};

type TX_SIZE = u8;

// av1/common/enums.h
const TX_SIZES_ALL: usize = 19;

// av1/common/blockd.h (CFL_BUF_LINE)
const CFL_BUF_LINE: isize = 32;

#[inline(always)]
unsafe fn vldsubstq_s16(dst: *mut i16, src: *const u16, offset: isize, sub: int16x8_t) {
    vst1q_s16(
        dst.offset(offset),
        vsubq_s16(vreinterpretq_s16_u16(vld1q_u16(src.offset(offset))), sub),
    );
}

#[inline(always)]
unsafe fn vldaddq_u16(buf: *const u16, offset: isize) -> uint16x8_t {
    vaddq_u16(vld1q_u16(buf), vld1q_u16(buf.offset(offset)))
}

// Load half of a vector and duplicated in other half
#[inline(always)]
unsafe fn vldh_dup_u8(ptr: *const u8) -> uint8x8_t {
    vreinterpret_u8_u32(vdup_n_u32(ptr::read_unaligned(ptr as *const u32)))
}

// Store half of a vector.
#[inline(always)]
unsafe fn vsth_u16(ptr: *mut u16, val: uint16x4_t) {
    ptr::write_unaligned(ptr as *mut u32, vget_lane_u32::<0>(vreinterpret_u32_u16(val)));
}

// Store half of a vector.
#[inline(always)]
unsafe fn vsth_u8(ptr: *mut u8, val: uint8x8_t) {
    ptr::write_unaligned(ptr as *mut u32, vget_lane_u32::<0>(vreinterpret_u32_u8(val)));
}

#[inline(always)]
unsafe fn cfl_luma_subsampling_420_lbd_neon(
    mut input: *const u8,
    input_stride: i32,
    mut pred_buf_q3: *mut u16,
    width: i32,
    height: i32,
) {
    let end: *const u16 = pred_buf_q3.wrapping_offset((height >> 1) as isize * CFL_BUF_LINE);
    let luma_stride: i32 = input_stride << 1;
    loop {
        if width == 4 {
            let top: uint16x4_t = vpaddl_u8(vldh_dup_u8(input));
            let sum: uint16x4_t = vpadal_u8(top, vldh_dup_u8(input.offset(input_stride as isize)));
            vsth_u16(pred_buf_q3, vshl_n_u16::<1>(sum));
        } else if width == 8 {
            let top: uint16x4_t = vpaddl_u8(vld1_u8(input));
            let sum: uint16x4_t = vpadal_u8(top, vld1_u8(input.offset(input_stride as isize)));
            vst1_u16(pred_buf_q3, vshl_n_u16::<1>(sum));
        } else if width == 16 {
            let top: uint16x8_t = vpaddlq_u8(vld1q_u8(input));
            let sum: uint16x8_t = vpadalq_u8(top, vld1q_u8(input.offset(input_stride as isize)));
            vst1q_u16(pred_buf_q3, vshlq_n_u16::<1>(sum));
        } else {
            let top: uint8x8x4_t = vld4_u8(input);
            let bot: uint8x8x4_t = vld4_u8(input.offset(input_stride as isize));
            // equivalent to a vpaddlq_u8 (because vld4q interleaves)
            let top_0: uint16x8_t = vaddl_u8(top.0, top.1);
            // equivalent to a vpaddlq_u8 (because vld4q interleaves)
            let bot_0: uint16x8_t = vaddl_u8(bot.0, bot.1);
            // equivalent to a vpaddlq_u8 (because vld4q interleaves)
            let top_1: uint16x8_t = vaddl_u8(top.2, top.3);
            // equivalent to a vpaddlq_u8 (because vld4q interleaves)
            let bot_1: uint16x8_t = vaddl_u8(bot.2, bot.3);
            let mut sum: uint16x8x2_t = uint16x8x2_t(vdupq_n_u16(0), vdupq_n_u16(0));
            sum.0 = vshlq_n_u16::<1>(vaddq_u16(top_0, bot_0));
            sum.1 = vshlq_n_u16::<1>(vaddq_u16(top_1, bot_1));
            vst2q_u16(pred_buf_q3, sum);
        }
        input = input.wrapping_offset(luma_stride as isize);
        pred_buf_q3 = pred_buf_q3.wrapping_offset(CFL_BUF_LINE);
        if !((pred_buf_q3 as *const u16) < end) {
            break;
        }
    }
}

#[inline(always)]
unsafe fn cfl_luma_subsampling_422_lbd_neon(
    mut input: *const u8,
    input_stride: i32,
    mut pred_buf_q3: *mut u16,
    width: i32,
    height: i32,
) {
    let end: *const u16 = pred_buf_q3.wrapping_offset(height as isize * CFL_BUF_LINE);
    loop {
        if width == 4 {
            let top: uint16x4_t = vpaddl_u8(vldh_dup_u8(input));
            vsth_u16(pred_buf_q3, vshl_n_u16::<2>(top));
        } else if width == 8 {
            let top: uint16x4_t = vpaddl_u8(vld1_u8(input));
            vst1_u16(pred_buf_q3, vshl_n_u16::<2>(top));
        } else if width == 16 {
            let top: uint16x8_t = vpaddlq_u8(vld1q_u8(input));
            vst1q_u16(pred_buf_q3, vshlq_n_u16::<2>(top));
        } else {
            let top: uint8x8x4_t = vld4_u8(input);
            let mut sum: uint16x8x2_t = uint16x8x2_t(vdupq_n_u16(0), vdupq_n_u16(0));
            // vaddl_u8 is equivalent to a vpaddlq_u8 (because vld4q interleaves)
            sum.0 = vshlq_n_u16::<2>(vaddl_u8(top.0, top.1));
            sum.1 = vshlq_n_u16::<2>(vaddl_u8(top.2, top.3));
            vst2q_u16(pred_buf_q3, sum);
        }
        input = input.wrapping_offset(input_stride as isize);
        pred_buf_q3 = pred_buf_q3.wrapping_offset(CFL_BUF_LINE);
        if !((pred_buf_q3 as *const u16) < end) {
            break;
        }
    }
}

#[inline(always)]
unsafe fn cfl_luma_subsampling_444_lbd_neon(
    mut input: *const u8,
    input_stride: i32,
    mut pred_buf_q3: *mut u16,
    width: i32,
    height: i32,
) {
    let end: *const u16 = pred_buf_q3.wrapping_offset(height as isize * CFL_BUF_LINE);
    loop {
        if width == 4 {
            let top: uint16x8_t = vshll_n_u8::<3>(vldh_dup_u8(input));
            vst1_u16(pred_buf_q3, vget_low_u16(top));
        } else if width == 8 {
            let top: uint16x8_t = vshll_n_u8::<3>(vld1_u8(input));
            vst1q_u16(pred_buf_q3, top);
        } else {
            let top: uint8x16_t = vld1q_u8(input);
            vst1q_u16(pred_buf_q3, vshll_n_u8::<3>(vget_low_u8(top)));
            vst1q_u16(pred_buf_q3.offset(8), vshll_n_u8::<3>(vget_high_u8(top)));
            if width == 32 {
                let next_top: uint8x16_t = vld1q_u8(input.offset(16));
                vst1q_u16(pred_buf_q3.offset(16), vshll_n_u8::<3>(vget_low_u8(next_top)));
                vst1q_u16(pred_buf_q3.offset(24), vshll_n_u8::<3>(vget_high_u8(next_top)));
            }
        }
        input = input.wrapping_offset(input_stride as isize);
        pred_buf_q3 = pred_buf_q3.wrapping_offset(CFL_BUF_LINE);
        if !((pred_buf_q3 as *const u16) < end) {
            break;
        }
    }
}

#[inline(always)]
unsafe fn cfl_luma_subsampling_420_hbd_neon(
    mut input: *const u16,
    input_stride: i32,
    mut pred_buf_q3: *mut u16,
    width: i32,
    height: i32,
) {
    let end: *const u16 = pred_buf_q3.wrapping_offset((height >> 1) as isize * CFL_BUF_LINE);
    let luma_stride: i32 = input_stride << 1;
    loop {
        if width == 4 {
            let top: uint16x4_t = vld1_u16(input);
            let bot: uint16x4_t = vld1_u16(input.offset(input_stride as isize));
            let sum: uint16x4_t = vadd_u16(top, bot);
            let hsum: uint16x4_t = vpadd_u16(sum, sum);
            vsth_u16(pred_buf_q3, vshl_n_u16::<1>(hsum));
        } else if width < 32 {
            let top: uint16x8_t = vld1q_u16(input);
            let bot: uint16x8_t = vld1q_u16(input.offset(input_stride as isize));
            let sum: uint16x8_t = vaddq_u16(top, bot);
            if width == 8 {
                let hsum: uint16x4_t = vget_low_u16(vpaddq_u16(sum, sum));
                vst1_u16(pred_buf_q3, vshl_n_u16::<1>(hsum));
            } else {
                let top_1: uint16x8_t = vld1q_u16(input.offset(8));
                let bot_1: uint16x8_t = vld1q_u16(input.offset(8 + input_stride as isize));
                let sum_1: uint16x8_t = vaddq_u16(top_1, bot_1);
                let hsum: uint16x8_t = vpaddq_u16(sum, sum_1);
                vst1q_u16(pred_buf_q3, vshlq_n_u16::<1>(hsum));
            }
        } else {
            let top: uint16x8x4_t = vld4q_u16(input);
            let bot: uint16x8x4_t = vld4q_u16(input.offset(input_stride as isize));
            // equivalent to a vpaddq_u16 (because vld4q interleaves)
            let top_0: uint16x8_t = vaddq_u16(top.0, top.1);
            // equivalent to a vpaddq_u16 (because vld4q interleaves)
            let bot_0: uint16x8_t = vaddq_u16(bot.0, bot.1);
            // equivalent to a vpaddq_u16 (because vld4q interleaves)
            let top_1: uint16x8_t = vaddq_u16(top.2, top.3);
            // equivalent to a vpaddq_u16 (because vld4q interleaves)
            let bot_1: uint16x8_t = vaddq_u16(bot.2, bot.3);
            let mut sum: uint16x8x2_t = uint16x8x2_t(vdupq_n_u16(0), vdupq_n_u16(0));
            sum.0 = vshlq_n_u16::<1>(vaddq_u16(top_0, bot_0));
            sum.1 = vshlq_n_u16::<1>(vaddq_u16(top_1, bot_1));
            vst2q_u16(pred_buf_q3, sum);
        }
        input = input.wrapping_offset(luma_stride as isize);
        pred_buf_q3 = pred_buf_q3.wrapping_offset(CFL_BUF_LINE);
        if !((pred_buf_q3 as *const u16) < end) {
            break;
        }
    }
}

#[inline(always)]
unsafe fn cfl_luma_subsampling_422_hbd_neon(
    mut input: *const u16,
    input_stride: i32,
    mut pred_buf_q3: *mut u16,
    width: i32,
    height: i32,
) {
    let end: *const u16 = pred_buf_q3.wrapping_offset(height as isize * CFL_BUF_LINE);
    loop {
        if width == 4 {
            let top: uint16x4_t = vld1_u16(input);
            let hsum: uint16x4_t = vpadd_u16(top, top);
            vsth_u16(pred_buf_q3, vshl_n_u16::<2>(hsum));
        } else if width == 8 {
            let top: uint16x4x2_t = vld2_u16(input);
            // equivalent to a vpadd_u16 (because vld2 interleaves)
            let hsum: uint16x4_t = vadd_u16(top.0, top.1);
            vst1_u16(pred_buf_q3, vshl_n_u16::<2>(hsum));
        } else if width == 16 {
            let top: uint16x8x2_t = vld2q_u16(input);
            // equivalent to a vpaddq_u16 (because vld2q interleaves)
            let hsum: uint16x8_t = vaddq_u16(top.0, top.1);
            vst1q_u16(pred_buf_q3, vshlq_n_u16::<2>(hsum));
        } else {
            let top: uint16x8x4_t = vld4q_u16(input);
            // equivalent to a vpaddq_u16 (because vld4q interleaves)
            let hsum_0: uint16x8_t = vaddq_u16(top.0, top.1);
            // equivalent to a vpaddq_u16 (because vld4q interleaves)
            let hsum_1: uint16x8_t = vaddq_u16(top.2, top.3);
            let result: uint16x8x2_t =
                uint16x8x2_t(vshlq_n_u16::<2>(hsum_0), vshlq_n_u16::<2>(hsum_1));
            vst2q_u16(pred_buf_q3, result);
        }
        input = input.wrapping_offset(input_stride as isize);
        pred_buf_q3 = pred_buf_q3.wrapping_offset(CFL_BUF_LINE);
        if !((pred_buf_q3 as *const u16) < end) {
            break;
        }
    }
}

#[inline(always)]
unsafe fn cfl_luma_subsampling_444_hbd_neon(
    mut input: *const u16,
    input_stride: i32,
    mut pred_buf_q3: *mut u16,
    width: i32,
    height: i32,
) {
    let end: *const u16 = pred_buf_q3.wrapping_offset(height as isize * CFL_BUF_LINE);
    loop {
        if width == 4 {
            let top: uint16x4_t = vld1_u16(input);
            vst1_u16(pred_buf_q3, vshl_n_u16::<3>(top));
        } else if width == 8 {
            let top: uint16x8_t = vld1q_u16(input);
            vst1q_u16(pred_buf_q3, vshlq_n_u16::<3>(top));
        } else if width == 16 {
            let mut top: uint16x8x2_t = vld2q_u16(input);
            top.0 = vshlq_n_u16::<3>(top.0);
            top.1 = vshlq_n_u16::<3>(top.1);
            vst2q_u16(pred_buf_q3, top);
        } else {
            let mut top: uint16x8x4_t = vld4q_u16(input);
            top.0 = vshlq_n_u16::<3>(top.0);
            top.1 = vshlq_n_u16::<3>(top.1);
            top.2 = vshlq_n_u16::<3>(top.2);
            top.3 = vshlq_n_u16::<3>(top.3);
            vst4q_u16(pred_buf_q3, top);
        }
        input = input.wrapping_offset(input_stride as isize);
        pred_buf_q3 = pred_buf_q3.wrapping_offset(CFL_BUF_LINE);
        if !((pred_buf_q3 as *const u16) < end) {
            break;
        }
    }
}

// CFL_SUBSAMPLE / CFL_SUBSAMPLE_FUNCTIONS / CFL_SUBSAMPLE_FUNCTION_ARRAY
// (av1/common/cfl.h), instantiated by CFL_GET_SUBSAMPLE_FUNCTION(neon).
macro_rules! cfl_subsample {
    ($name:ident, $inner:ident, $ty:ty, $w:literal, $h:literal) => {
        #[no_mangle]
        pub unsafe extern "C-unwind" fn $name(
            cfl_type: *const $ty,
            input_stride: i32,
            output_q3: *mut u16,
        ) {
            $inner(cfl_type, input_stride, output_q3, $w, $h);
        }
    };
}

macro_rules! cfl_subsample_functions {
    ($getter:ident, $fn_ty:ty, $inner:ident, $ty:ty,
     [$n4x4:ident, $n8x8:ident, $n16x16:ident, $n32x32:ident, $n4x8:ident, $n8x4:ident,
      $n8x16:ident, $n16x8:ident, $n16x32:ident, $n32x16:ident, $n4x16:ident, $n16x4:ident,
      $n8x32:ident, $n32x8:ident]) => {
        cfl_subsample!($n4x4, $inner, $ty, 4, 4);
        cfl_subsample!($n8x8, $inner, $ty, 8, 8);
        cfl_subsample!($n16x16, $inner, $ty, 16, 16);
        cfl_subsample!($n32x32, $inner, $ty, 32, 32);
        cfl_subsample!($n4x8, $inner, $ty, 4, 8);
        cfl_subsample!($n8x4, $inner, $ty, 8, 4);
        cfl_subsample!($n8x16, $inner, $ty, 8, 16);
        cfl_subsample!($n16x8, $inner, $ty, 16, 8);
        cfl_subsample!($n16x32, $inner, $ty, 16, 32);
        cfl_subsample!($n32x16, $inner, $ty, 32, 16);
        cfl_subsample!($n4x16, $inner, $ty, 4, 16);
        cfl_subsample!($n16x4, $inner, $ty, 16, 4);
        cfl_subsample!($n8x32, $inner, $ty, 8, 32);
        cfl_subsample!($n32x8, $inner, $ty, 32, 8);

        #[no_mangle]
        pub unsafe extern "C-unwind" fn $getter(tx_size: TX_SIZE) -> $fn_ty {
            static subfn: [$fn_ty; TX_SIZES_ALL] = [
                Some($n4x4),   // 4x4
                Some($n8x8),   // 8x8
                Some($n16x16), // 16x16
                Some($n32x32), // 32x32
                None,          // 64x64 (invalid CFL size)
                Some($n4x8),   // 4x8
                Some($n8x4),   // 8x4
                Some($n8x16),  // 8x16
                Some($n16x8),  // 16x8
                Some($n16x32), // 16x32
                Some($n32x16), // 32x16
                None,          // 32x64 (invalid CFL size)
                None,          // 64x32 (invalid CFL size)
                Some($n4x16),  // 4x16
                Some($n16x4),  // 16x4
                Some($n8x32),  // 8x32
                Some($n32x8),  // 32x8
                None,          // 16x64 (invalid CFL size)
                None,          // 64x16 (invalid CFL size)
            ];
            subfn[tx_size as usize]
        }
    };
}

// CFL_GET_SUBSAMPLE_FUNCTION(neon), CONFIG_AV1_HIGHBITDEPTH
cfl_subsample_functions!(
    cfl_get_luma_subsampling_420_lbd_neon, cfl_subsample_lbd_fn,
    cfl_luma_subsampling_420_lbd_neon, u8,
    [cfl_subsample_lbd_420_4x4_neon, cfl_subsample_lbd_420_8x8_neon,
     cfl_subsample_lbd_420_16x16_neon, cfl_subsample_lbd_420_32x32_neon,
     cfl_subsample_lbd_420_4x8_neon, cfl_subsample_lbd_420_8x4_neon,
     cfl_subsample_lbd_420_8x16_neon, cfl_subsample_lbd_420_16x8_neon,
     cfl_subsample_lbd_420_16x32_neon, cfl_subsample_lbd_420_32x16_neon,
     cfl_subsample_lbd_420_4x16_neon, cfl_subsample_lbd_420_16x4_neon,
     cfl_subsample_lbd_420_8x32_neon, cfl_subsample_lbd_420_32x8_neon]
);
cfl_subsample_functions!(
    cfl_get_luma_subsampling_422_lbd_neon, cfl_subsample_lbd_fn,
    cfl_luma_subsampling_422_lbd_neon, u8,
    [cfl_subsample_lbd_422_4x4_neon, cfl_subsample_lbd_422_8x8_neon,
     cfl_subsample_lbd_422_16x16_neon, cfl_subsample_lbd_422_32x32_neon,
     cfl_subsample_lbd_422_4x8_neon, cfl_subsample_lbd_422_8x4_neon,
     cfl_subsample_lbd_422_8x16_neon, cfl_subsample_lbd_422_16x8_neon,
     cfl_subsample_lbd_422_16x32_neon, cfl_subsample_lbd_422_32x16_neon,
     cfl_subsample_lbd_422_4x16_neon, cfl_subsample_lbd_422_16x4_neon,
     cfl_subsample_lbd_422_8x32_neon, cfl_subsample_lbd_422_32x8_neon]
);
cfl_subsample_functions!(
    cfl_get_luma_subsampling_444_lbd_neon, cfl_subsample_lbd_fn,
    cfl_luma_subsampling_444_lbd_neon, u8,
    [cfl_subsample_lbd_444_4x4_neon, cfl_subsample_lbd_444_8x8_neon,
     cfl_subsample_lbd_444_16x16_neon, cfl_subsample_lbd_444_32x32_neon,
     cfl_subsample_lbd_444_4x8_neon, cfl_subsample_lbd_444_8x4_neon,
     cfl_subsample_lbd_444_8x16_neon, cfl_subsample_lbd_444_16x8_neon,
     cfl_subsample_lbd_444_16x32_neon, cfl_subsample_lbd_444_32x16_neon,
     cfl_subsample_lbd_444_4x16_neon, cfl_subsample_lbd_444_16x4_neon,
     cfl_subsample_lbd_444_8x32_neon, cfl_subsample_lbd_444_32x8_neon]
);
cfl_subsample_functions!(
    cfl_get_luma_subsampling_420_hbd_neon, cfl_subsample_hbd_fn,
    cfl_luma_subsampling_420_hbd_neon, u16,
    [cfl_subsample_hbd_420_4x4_neon, cfl_subsample_hbd_420_8x8_neon,
     cfl_subsample_hbd_420_16x16_neon, cfl_subsample_hbd_420_32x32_neon,
     cfl_subsample_hbd_420_4x8_neon, cfl_subsample_hbd_420_8x4_neon,
     cfl_subsample_hbd_420_8x16_neon, cfl_subsample_hbd_420_16x8_neon,
     cfl_subsample_hbd_420_16x32_neon, cfl_subsample_hbd_420_32x16_neon,
     cfl_subsample_hbd_420_4x16_neon, cfl_subsample_hbd_420_16x4_neon,
     cfl_subsample_hbd_420_8x32_neon, cfl_subsample_hbd_420_32x8_neon]
);
cfl_subsample_functions!(
    cfl_get_luma_subsampling_422_hbd_neon, cfl_subsample_hbd_fn,
    cfl_luma_subsampling_422_hbd_neon, u16,
    [cfl_subsample_hbd_422_4x4_neon, cfl_subsample_hbd_422_8x8_neon,
     cfl_subsample_hbd_422_16x16_neon, cfl_subsample_hbd_422_32x32_neon,
     cfl_subsample_hbd_422_4x8_neon, cfl_subsample_hbd_422_8x4_neon,
     cfl_subsample_hbd_422_8x16_neon, cfl_subsample_hbd_422_16x8_neon,
     cfl_subsample_hbd_422_16x32_neon, cfl_subsample_hbd_422_32x16_neon,
     cfl_subsample_hbd_422_4x16_neon, cfl_subsample_hbd_422_16x4_neon,
     cfl_subsample_hbd_422_8x32_neon, cfl_subsample_hbd_422_32x8_neon]
);
cfl_subsample_functions!(
    cfl_get_luma_subsampling_444_hbd_neon, cfl_subsample_hbd_fn,
    cfl_luma_subsampling_444_hbd_neon, u16,
    [cfl_subsample_hbd_444_4x4_neon, cfl_subsample_hbd_444_8x8_neon,
     cfl_subsample_hbd_444_16x16_neon, cfl_subsample_hbd_444_32x32_neon,
     cfl_subsample_hbd_444_4x8_neon, cfl_subsample_hbd_444_8x4_neon,
     cfl_subsample_hbd_444_8x16_neon, cfl_subsample_hbd_444_16x8_neon,
     cfl_subsample_hbd_444_16x32_neon, cfl_subsample_hbd_444_32x16_neon,
     cfl_subsample_hbd_444_4x16_neon, cfl_subsample_hbd_444_16x4_neon,
     cfl_subsample_hbd_444_8x32_neon, cfl_subsample_hbd_444_32x8_neon]
);

#[inline(always)]
unsafe fn subtract_average_neon(
    mut src: *const u16,
    mut dst: *mut i16,
    width: i32,
    height: i32,
    round_offset: i32,
    num_pel_log2: i32,
) {
    let end: *const u16 = src.wrapping_offset(height as isize * CFL_BUF_LINE);

    // Round offset is not needed, because NEON will handle the rounding.
    let _ = round_offset;

    // To optimize the use of the CPU pipeline, we process 4 rows per iteration
    let step: isize = 4 * CFL_BUF_LINE;

    // At this stage, the prediction buffer contains scaled reconstructed luma
    // pixels, which are positive integer and only require 15 bits. By using
    // unsigned integer for the sum, we can do one addition operation inside 16
    // bits (8 lanes) before having to convert to 32 bits (4 lanes).
    let mut sum_buf: *const u16 = src;
    let mut sum_32x4: uint32x4_t = vdupq_n_u32(0);
    loop {
        // For all widths, we load, add and combine the data so it fits in 4 lanes.
        if width == 4 {
            let a0: uint16x4_t =
                vadd_u16(vld1_u16(sum_buf), vld1_u16(sum_buf.offset(CFL_BUF_LINE)));
            let a1: uint16x4_t = vadd_u16(
                vld1_u16(sum_buf.offset(2 * CFL_BUF_LINE)),
                vld1_u16(sum_buf.offset(3 * CFL_BUF_LINE)),
            );
            sum_32x4 = vaddq_u32(sum_32x4, vaddl_u16(a0, a1));
        } else if width == 8 {
            let a0: uint16x8_t = vldaddq_u16(sum_buf, CFL_BUF_LINE);
            let a1: uint16x8_t = vldaddq_u16(sum_buf.offset(2 * CFL_BUF_LINE), CFL_BUF_LINE);
            sum_32x4 = vpadalq_u16(sum_32x4, a0);
            sum_32x4 = vpadalq_u16(sum_32x4, a1);
        } else {
            let row0: uint16x8_t = vldaddq_u16(sum_buf, 8);
            let row1: uint16x8_t = vldaddq_u16(sum_buf.offset(CFL_BUF_LINE), 8);
            let row2: uint16x8_t = vldaddq_u16(sum_buf.offset(2 * CFL_BUF_LINE), 8);
            let row3: uint16x8_t = vldaddq_u16(sum_buf.offset(3 * CFL_BUF_LINE), 8);
            sum_32x4 = vpadalq_u16(sum_32x4, row0);
            sum_32x4 = vpadalq_u16(sum_32x4, row1);
            sum_32x4 = vpadalq_u16(sum_32x4, row2);
            sum_32x4 = vpadalq_u16(sum_32x4, row3);

            if width == 32 {
                let row0_1: uint16x8_t = vldaddq_u16(sum_buf.offset(16), 8);
                let row1_1: uint16x8_t = vldaddq_u16(sum_buf.offset(CFL_BUF_LINE + 16), 8);
                let row2_1: uint16x8_t = vldaddq_u16(sum_buf.offset(2 * CFL_BUF_LINE + 16), 8);
                let row3_1: uint16x8_t = vldaddq_u16(sum_buf.offset(3 * CFL_BUF_LINE + 16), 8);

                sum_32x4 = vpadalq_u16(sum_32x4, row0_1);
                sum_32x4 = vpadalq_u16(sum_32x4, row1_1);
                sum_32x4 = vpadalq_u16(sum_32x4, row2_1);
                sum_32x4 = vpadalq_u16(sum_32x4, row3_1);
            }
        }
        sum_buf = sum_buf.wrapping_offset(step);
        if !(sum_buf < end) {
            break;
        }
    }

    // Permute and add in such a way that each lane contains the block sum.
    // [A+C+B+D, B+D+A+C, C+A+D+B, D+B+C+A]
    sum_32x4 = vpaddq_u32(sum_32x4, sum_32x4);
    sum_32x4 = vpaddq_u32(sum_32x4, sum_32x4);

    // Computing the average could be done using scalars, but getting off the NEON
    // engine introduces latency, so we use vqrshrn.
    let mut avg_16x4: int16x4_t = vdup_n_s16(0);
    // Constant propagation makes for some ugly code.
    match num_pel_log2 {
        4 => avg_16x4 = vreinterpret_s16_u16(vqrshrn_n_u32::<4>(sum_32x4)),
        5 => avg_16x4 = vreinterpret_s16_u16(vqrshrn_n_u32::<5>(sum_32x4)),
        6 => avg_16x4 = vreinterpret_s16_u16(vqrshrn_n_u32::<6>(sum_32x4)),
        7 => avg_16x4 = vreinterpret_s16_u16(vqrshrn_n_u32::<7>(sum_32x4)),
        8 => avg_16x4 = vreinterpret_s16_u16(vqrshrn_n_u32::<8>(sum_32x4)),
        9 => avg_16x4 = vreinterpret_s16_u16(vqrshrn_n_u32::<9>(sum_32x4)),
        10 => avg_16x4 = vreinterpret_s16_u16(vqrshrn_n_u32::<10>(sum_32x4)),
        _ => debug_assert!(false),
    }

    if width == 4 {
        loop {
            vst1_s16(dst, vsub_s16(vreinterpret_s16_u16(vld1_u16(src)), avg_16x4));
            src = src.wrapping_offset(CFL_BUF_LINE);
            dst = dst.wrapping_offset(CFL_BUF_LINE);
            if !(src < end) {
                break;
            }
        }
    } else {
        let avg_16x8: int16x8_t = vcombine_s16(avg_16x4, avg_16x4);
        loop {
            vldsubstq_s16(dst, src, 0, avg_16x8);
            vldsubstq_s16(dst, src, CFL_BUF_LINE, avg_16x8);
            vldsubstq_s16(dst, src, 2 * CFL_BUF_LINE, avg_16x8);
            vldsubstq_s16(dst, src, 3 * CFL_BUF_LINE, avg_16x8);

            if width > 8 {
                vldsubstq_s16(dst, src, 8, avg_16x8);
                vldsubstq_s16(dst, src, 8 + CFL_BUF_LINE, avg_16x8);
                vldsubstq_s16(dst, src, 8 + 2 * CFL_BUF_LINE, avg_16x8);
                vldsubstq_s16(dst, src, 8 + 3 * CFL_BUF_LINE, avg_16x8);
            }
            if width == 32 {
                vldsubstq_s16(dst, src, 16, avg_16x8);
                vldsubstq_s16(dst, src, 16 + CFL_BUF_LINE, avg_16x8);
                vldsubstq_s16(dst, src, 16 + 2 * CFL_BUF_LINE, avg_16x8);
                vldsubstq_s16(dst, src, 16 + 3 * CFL_BUF_LINE, avg_16x8);
                vldsubstq_s16(dst, src, 24, avg_16x8);
                vldsubstq_s16(dst, src, 24 + CFL_BUF_LINE, avg_16x8);
                vldsubstq_s16(dst, src, 24 + 2 * CFL_BUF_LINE, avg_16x8);
                vldsubstq_s16(dst, src, 24 + 3 * CFL_BUF_LINE, avg_16x8);
            }
            src = src.wrapping_offset(step);
            dst = dst.wrapping_offset(step);
            if !(src < end) {
                break;
            }
        }
    }
}

// CFL_SUB_AVG_X / CFL_SUB_AVG_FN (av1/common/cfl.h)
macro_rules! cfl_sub_avg_x {
    ($name:ident, $w:literal, $h:literal, $round_offset:literal, $num_pel_log2:literal) => {
        #[no_mangle]
        pub unsafe extern "C-unwind" fn $name(src: *const u16, dst: *mut i16) {
            subtract_average_neon(src, dst, $w, $h, $round_offset, $num_pel_log2);
        }
    };
}

cfl_sub_avg_x!(cfl_subtract_average_4x4_neon, 4, 4, 8, 4);
cfl_sub_avg_x!(cfl_subtract_average_4x8_neon, 4, 8, 16, 5);
cfl_sub_avg_x!(cfl_subtract_average_4x16_neon, 4, 16, 32, 6);
cfl_sub_avg_x!(cfl_subtract_average_8x4_neon, 8, 4, 16, 5);
cfl_sub_avg_x!(cfl_subtract_average_8x8_neon, 8, 8, 32, 6);
cfl_sub_avg_x!(cfl_subtract_average_8x16_neon, 8, 16, 64, 7);
cfl_sub_avg_x!(cfl_subtract_average_8x32_neon, 8, 32, 128, 8);
cfl_sub_avg_x!(cfl_subtract_average_16x4_neon, 16, 4, 32, 6);
cfl_sub_avg_x!(cfl_subtract_average_16x8_neon, 16, 8, 64, 7);
cfl_sub_avg_x!(cfl_subtract_average_16x16_neon, 16, 16, 128, 8);
cfl_sub_avg_x!(cfl_subtract_average_16x32_neon, 16, 32, 256, 9);
cfl_sub_avg_x!(cfl_subtract_average_32x8_neon, 32, 8, 128, 8);
cfl_sub_avg_x!(cfl_subtract_average_32x16_neon, 32, 16, 256, 9);
cfl_sub_avg_x!(cfl_subtract_average_32x32_neon, 32, 32, 512, 10);

#[no_mangle]
pub unsafe extern "C-unwind" fn cfl_get_subtract_average_fn_neon(
    tx_size: TX_SIZE,
) -> cfl_subtract_average_fn {
    static sub_avg: [cfl_subtract_average_fn; TX_SIZES_ALL] = [
        Some(cfl_subtract_average_4x4_neon),   // 4x4
        Some(cfl_subtract_average_8x8_neon),   // 8x8
        Some(cfl_subtract_average_16x16_neon), // 16x16
        Some(cfl_subtract_average_32x32_neon), // 32x32
        None,                                  // 64x64 (invalid CFL size)
        Some(cfl_subtract_average_4x8_neon),   // 4x8
        Some(cfl_subtract_average_8x4_neon),   // 8x4
        Some(cfl_subtract_average_8x16_neon),  // 8x16
        Some(cfl_subtract_average_16x8_neon),  // 16x8
        Some(cfl_subtract_average_16x32_neon), // 16x32
        Some(cfl_subtract_average_32x16_neon), // 32x16
        None,                                  // 32x64 (invalid CFL size)
        None,                                  // 64x32 (invalid CFL size)
        Some(cfl_subtract_average_4x16_neon),  // 4x16 (invalid CFL size)
        Some(cfl_subtract_average_16x4_neon),  // 16x4 (invalid CFL size)
        Some(cfl_subtract_average_8x32_neon),  // 8x32 (invalid CFL size)
        Some(cfl_subtract_average_32x8_neon),  // 32x8 (invalid CFL size)
        None,                                  // 16x64 (invalid CFL size)
        None,                                  // 64x16 (invalid CFL size)
    ];
    // Modulo TX_SIZES_ALL to ensure that an attacker won't be able to
    // index the function pointer array out of bounds.
    sub_avg[tx_size as usize % TX_SIZES_ALL]
}

// Saturating negate 16-bit integers in a when the corresponding signed 16-bit
// integer in b is negative.
// Notes:
//   * Negating INT16_MIN results in INT16_MIN. However, this cannot occur in
//   practice, as scaled_luma is the multiplication of two absolute values.
//   * In the Intel equivalent, elements in a are zeroed out when the
//   corresponding elements in b are zero. Because vsign is used twice in a
//   row, with b in the first call becoming a in the second call, there's no
//   impact from not zeroing out.
#[inline(always)]
unsafe fn vsign_s16(a: int16x4_t, b: int16x4_t) -> int16x4_t {
    let mask: int16x4_t = vshr_n_s16::<15>(b);
    veor_s16(vadd_s16(a, mask), mask)
}

// Saturating negate 16-bit integers in a when the corresponding signed 16-bit
// integer in b is negative.
// Notes:
//   * Negating INT16_MIN results in INT16_MIN. However, this cannot occur in
//   practice, as scaled_luma is the multiplication of two absolute values.
//   * In the Intel equivalent, elements in a are zeroed out when the
//   corresponding elements in b are zero. Because vsignq is used twice in a
//   row, with b in the first call becoming a in the second call, there's no
//   impact from not zeroing out.
#[inline(always)]
unsafe fn vsignq_s16(a: int16x8_t, b: int16x8_t) -> int16x8_t {
    let mask: int16x8_t = vshrq_n_s16::<15>(b);
    veorq_s16(vaddq_s16(a, mask), mask)
}

#[inline(always)]
unsafe fn predict_w4(
    pred_buf_q3: *const i16,
    alpha_sign: int16x4_t,
    abs_alpha_q12: i32,
    dc: int16x4_t,
) -> int16x4_t {
    let ac_q3: int16x4_t = vld1_s16(pred_buf_q3);
    let ac_sign: int16x4_t = veor_s16(alpha_sign, ac_q3);
    let scaled_luma: int16x4_t = vqrdmulh_n_s16(vabs_s16(ac_q3), abs_alpha_q12 as i16);
    vadd_s16(vsign_s16(scaled_luma, ac_sign), dc)
}

#[inline(always)]
unsafe fn predict_w8(
    pred_buf_q3: *const i16,
    alpha_sign: int16x8_t,
    abs_alpha_q12: i32,
    dc: int16x8_t,
) -> int16x8_t {
    let ac_q3: int16x8_t = vld1q_s16(pred_buf_q3);
    let ac_sign: int16x8_t = veorq_s16(alpha_sign, ac_q3);
    let scaled_luma: int16x8_t = vqrdmulhq_n_s16(vabsq_s16(ac_q3), abs_alpha_q12 as i16);
    vaddq_s16(vsignq_s16(scaled_luma, ac_sign), dc)
}

#[inline(always)]
unsafe fn predict_w16(
    pred_buf_q3: *const i16,
    alpha_sign: int16x8_t,
    abs_alpha_q12: i32,
    dc: int16x8_t,
) -> int16x8x2_t {
    let ac_q3: int16x8x2_t = vld1q_s16_x2(pred_buf_q3);
    let ac_sign_0: int16x8_t = veorq_s16(alpha_sign, ac_q3.0);
    let ac_sign_1: int16x8_t = veorq_s16(alpha_sign, ac_q3.1);
    let scaled_luma_0: int16x8_t = vqrdmulhq_n_s16(vabsq_s16(ac_q3.0), abs_alpha_q12 as i16);
    let scaled_luma_1: int16x8_t = vqrdmulhq_n_s16(vabsq_s16(ac_q3.1), abs_alpha_q12 as i16);
    let mut result: int16x8x2_t = int16x8x2_t(vdupq_n_s16(0), vdupq_n_s16(0));
    result.0 = vaddq_s16(vsignq_s16(scaled_luma_0, ac_sign_0), dc);
    result.1 = vaddq_s16(vsignq_s16(scaled_luma_1, ac_sign_1), dc);
    result
}

#[inline(always)]
unsafe fn predict_w32(
    pred_buf_q3: *const i16,
    alpha_sign: int16x8_t,
    abs_alpha_q12: i32,
    dc: int16x8_t,
) -> int16x8x4_t {
    let ac_q3: int16x8x4_t = vld1q_s16_x4(pred_buf_q3);
    let ac_sign_0: int16x8_t = veorq_s16(alpha_sign, ac_q3.0);
    let ac_sign_1: int16x8_t = veorq_s16(alpha_sign, ac_q3.1);
    let ac_sign_2: int16x8_t = veorq_s16(alpha_sign, ac_q3.2);
    let ac_sign_3: int16x8_t = veorq_s16(alpha_sign, ac_q3.3);
    let scaled_luma_0: int16x8_t = vqrdmulhq_n_s16(vabsq_s16(ac_q3.0), abs_alpha_q12 as i16);
    let scaled_luma_1: int16x8_t = vqrdmulhq_n_s16(vabsq_s16(ac_q3.1), abs_alpha_q12 as i16);
    let scaled_luma_2: int16x8_t = vqrdmulhq_n_s16(vabsq_s16(ac_q3.2), abs_alpha_q12 as i16);
    let scaled_luma_3: int16x8_t = vqrdmulhq_n_s16(vabsq_s16(ac_q3.3), abs_alpha_q12 as i16);
    let mut result: int16x8x4_t =
        int16x8x4_t(vdupq_n_s16(0), vdupq_n_s16(0), vdupq_n_s16(0), vdupq_n_s16(0));
    result.0 = vaddq_s16(vsignq_s16(scaled_luma_0, ac_sign_0), dc);
    result.1 = vaddq_s16(vsignq_s16(scaled_luma_1, ac_sign_1), dc);
    result.2 = vaddq_s16(vsignq_s16(scaled_luma_2, ac_sign_2), dc);
    result.3 = vaddq_s16(vsignq_s16(scaled_luma_3, ac_sign_3), dc);
    result
}

#[inline(always)]
unsafe fn cfl_predict_lbd_neon(
    mut pred_buf_q3: *const i16,
    mut dst: *mut u8,
    dst_stride: i32,
    alpha_q3: i32,
    width: i32,
    height: i32,
) {
    // `const int16_t abs_alpha_q12 = abs(alpha_q3) << 9;` then promoted back
    // to int at each use.
    let abs_alpha_q12: i32 = ((alpha_q3.wrapping_abs() << 9) as i16) as i32;
    let end: *const i16 = pred_buf_q3.wrapping_offset(height as isize * CFL_BUF_LINE);
    if width == 4 {
        let alpha_sign: int16x4_t = vdup_n_s16(alpha_q3 as i16);
        let dc: int16x4_t = vdup_n_s16(*dst as i16);
        loop {
            let pred: int16x4_t = predict_w4(pred_buf_q3, alpha_sign, abs_alpha_q12, dc);
            vsth_u8(dst, vqmovun_s16(vcombine_s16(pred, pred)));
            dst = dst.wrapping_offset(dst_stride as isize);
            pred_buf_q3 = pred_buf_q3.wrapping_offset(CFL_BUF_LINE);
            if !(pred_buf_q3 < end) {
                break;
            }
        }
    } else {
        let alpha_sign: int16x8_t = vdupq_n_s16(alpha_q3 as i16);
        let dc: int16x8_t = vdupq_n_s16(*dst as i16);
        loop {
            if width == 8 {
                vst1_u8(
                    dst,
                    vqmovun_s16(predict_w8(pred_buf_q3, alpha_sign, abs_alpha_q12, dc)),
                );
            } else if width == 16 {
                let pred: int16x8x2_t = predict_w16(pred_buf_q3, alpha_sign, abs_alpha_q12, dc);
                let predun: uint8x8x2_t = uint8x8x2_t(vqmovun_s16(pred.0), vqmovun_s16(pred.1));
                vst1_u8_x2(dst, predun);
            } else {
                let pred: int16x8x4_t = predict_w32(pred_buf_q3, alpha_sign, abs_alpha_q12, dc);
                let predun: uint8x8x4_t = uint8x8x4_t(
                    vqmovun_s16(pred.0),
                    vqmovun_s16(pred.1),
                    vqmovun_s16(pred.2),
                    vqmovun_s16(pred.3),
                );
                vst1_u8_x4(dst, predun);
            }
            dst = dst.wrapping_offset(dst_stride as isize);
            pred_buf_q3 = pred_buf_q3.wrapping_offset(CFL_BUF_LINE);
            if !(pred_buf_q3 < end) {
                break;
            }
        }
    }
}

// CFL_PREDICT_lbd / CFL_PREDICT_FN(neon, lbd) (av1/common/cfl.h)
macro_rules! cfl_predict_lbd {
    ($name:ident, $w:literal, $h:literal) => {
        #[no_mangle]
        pub unsafe extern "C-unwind" fn $name(
            pred_buf_q3: *const i16,
            dst: *mut u8,
            dst_stride: i32,
            alpha_q3: i32,
        ) {
            cfl_predict_lbd_neon(pred_buf_q3, dst, dst_stride, alpha_q3, $w, $h);
        }
    };
}

cfl_predict_lbd!(cfl_predict_lbd_4x4_neon, 4, 4);
cfl_predict_lbd!(cfl_predict_lbd_4x8_neon, 4, 8);
cfl_predict_lbd!(cfl_predict_lbd_4x16_neon, 4, 16);
cfl_predict_lbd!(cfl_predict_lbd_8x4_neon, 8, 4);
cfl_predict_lbd!(cfl_predict_lbd_8x8_neon, 8, 8);
cfl_predict_lbd!(cfl_predict_lbd_8x16_neon, 8, 16);
cfl_predict_lbd!(cfl_predict_lbd_8x32_neon, 8, 32);
cfl_predict_lbd!(cfl_predict_lbd_16x4_neon, 16, 4);
cfl_predict_lbd!(cfl_predict_lbd_16x8_neon, 16, 8);
cfl_predict_lbd!(cfl_predict_lbd_16x16_neon, 16, 16);
cfl_predict_lbd!(cfl_predict_lbd_16x32_neon, 16, 32);
cfl_predict_lbd!(cfl_predict_lbd_32x8_neon, 32, 8);
cfl_predict_lbd!(cfl_predict_lbd_32x16_neon, 32, 16);
cfl_predict_lbd!(cfl_predict_lbd_32x32_neon, 32, 32);

#[no_mangle]
pub unsafe extern "C-unwind" fn cfl_get_predict_lbd_fn_neon(tx_size: TX_SIZE) -> cfl_predict_lbd_fn {
    static pred: [cfl_predict_lbd_fn; TX_SIZES_ALL] = [
        Some(cfl_predict_lbd_4x4_neon),   // 4x4
        Some(cfl_predict_lbd_8x8_neon),   // 8x8
        Some(cfl_predict_lbd_16x16_neon), // 16x16
        Some(cfl_predict_lbd_32x32_neon), // 32x32
        None,                             // 64x64 (invalid CFL size)
        Some(cfl_predict_lbd_4x8_neon),   // 4x8
        Some(cfl_predict_lbd_8x4_neon),   // 8x4
        Some(cfl_predict_lbd_8x16_neon),  // 8x16
        Some(cfl_predict_lbd_16x8_neon),  // 16x8
        Some(cfl_predict_lbd_16x32_neon), // 16x32
        Some(cfl_predict_lbd_32x16_neon), // 32x16
        None,                             // 32x64 (invalid CFL size)
        None,                             // 64x32 (invalid CFL size)
        Some(cfl_predict_lbd_4x16_neon),  // 4x16
        Some(cfl_predict_lbd_16x4_neon),  // 16x4
        Some(cfl_predict_lbd_8x32_neon),  // 8x32
        Some(cfl_predict_lbd_32x8_neon),  // 32x8
        None,                             // 16x64 (invalid CFL size)
        None,                             // 64x16 (invalid CFL size)
    ];
    // Modulo TX_SIZES_ALL to ensure that an attacker won't be able to
    // index the function pointer array out of bounds.
    pred[tx_size as usize % TX_SIZES_ALL]
}

#[inline(always)]
unsafe fn clamp_s16(a: int16x4_t, max: int16x4_t) -> uint16x4_t {
    vreinterpret_u16_s16(vmax_s16(vmin_s16(a, max), vdup_n_s16(0)))
}

#[inline(always)]
unsafe fn clampq_s16(a: int16x8_t, max: int16x8_t) -> uint16x8_t {
    vreinterpretq_u16_s16(vmaxq_s16(vminq_s16(a, max), vdupq_n_s16(0)))
}

#[inline(always)]
unsafe fn clamp2q_s16(a: int16x8x2_t, max: int16x8_t) -> uint16x8x2_t {
    let mut result: uint16x8x2_t = uint16x8x2_t(vdupq_n_u16(0), vdupq_n_u16(0));
    result.0 = vreinterpretq_u16_s16(vmaxq_s16(vminq_s16(a.0, max), vdupq_n_s16(0)));
    result.1 = vreinterpretq_u16_s16(vmaxq_s16(vminq_s16(a.1, max), vdupq_n_s16(0)));
    result
}

#[inline(always)]
unsafe fn clamp4q_s16(a: int16x8x4_t, max: int16x8_t) -> uint16x8x4_t {
    let mut result: uint16x8x4_t =
        uint16x8x4_t(vdupq_n_u16(0), vdupq_n_u16(0), vdupq_n_u16(0), vdupq_n_u16(0));
    result.0 = vreinterpretq_u16_s16(vmaxq_s16(vminq_s16(a.0, max), vdupq_n_s16(0)));
    result.1 = vreinterpretq_u16_s16(vmaxq_s16(vminq_s16(a.1, max), vdupq_n_s16(0)));
    result.2 = vreinterpretq_u16_s16(vmaxq_s16(vminq_s16(a.2, max), vdupq_n_s16(0)));
    result.3 = vreinterpretq_u16_s16(vmaxq_s16(vminq_s16(a.3, max), vdupq_n_s16(0)));
    result
}

#[inline(always)]
unsafe fn cfl_predict_hbd_neon(
    mut pred_buf_q3: *const i16,
    mut dst: *mut u16,
    dst_stride: i32,
    alpha_q3: i32,
    bd: i32,
    width: i32,
    height: i32,
) {
    let max: i32 = (1 << bd) - 1;
    // `const int16_t abs_alpha_q12 = abs(alpha_q3) << 9;` then promoted back
    // to int at each use.
    let abs_alpha_q12: i32 = ((alpha_q3.wrapping_abs() << 9) as i16) as i32;
    let end: *const i16 = pred_buf_q3.wrapping_offset(height as isize * CFL_BUF_LINE);
    if width == 4 {
        let alpha_sign: int16x4_t = vdup_n_s16(alpha_q3 as i16);
        let dc: int16x4_t = vdup_n_s16(*dst as i16);
        let max_16x4: int16x4_t = vdup_n_s16(max as i16);
        loop {
            let scaled_luma: int16x4_t = predict_w4(pred_buf_q3, alpha_sign, abs_alpha_q12, dc);
            vst1_u16(dst, clamp_s16(scaled_luma, max_16x4));
            dst = dst.wrapping_offset(dst_stride as isize);
            pred_buf_q3 = pred_buf_q3.wrapping_offset(CFL_BUF_LINE);
            if !(pred_buf_q3 < end) {
                break;
            }
        }
    } else {
        let alpha_sign: int16x8_t = vdupq_n_s16(alpha_q3 as i16);
        let dc: int16x8_t = vdupq_n_s16(*dst as i16);
        let max_16x8: int16x8_t = vdupq_n_s16(max as i16);
        loop {
            if width == 8 {
                let pred: int16x8_t = predict_w8(pred_buf_q3, alpha_sign, abs_alpha_q12, dc);
                vst1q_u16(dst, clampq_s16(pred, max_16x8));
            } else if width == 16 {
                let pred: int16x8x2_t = predict_w16(pred_buf_q3, alpha_sign, abs_alpha_q12, dc);
                vst1q_u16_x2(dst, clamp2q_s16(pred, max_16x8));
            } else {
                let pred: int16x8x4_t = predict_w32(pred_buf_q3, alpha_sign, abs_alpha_q12, dc);
                vst1q_u16_x4(dst, clamp4q_s16(pred, max_16x8));
            }
            dst = dst.wrapping_offset(dst_stride as isize);
            pred_buf_q3 = pred_buf_q3.wrapping_offset(CFL_BUF_LINE);
            if !(pred_buf_q3 < end) {
                break;
            }
        }
    }
}

// CFL_PREDICT_hbd / CFL_PREDICT_FN(neon, hbd) (av1/common/cfl.h)
macro_rules! cfl_predict_hbd {
    ($name:ident, $w:literal, $h:literal) => {
        #[no_mangle]
        pub unsafe extern "C-unwind" fn $name(
            pred_buf_q3: *const i16,
            dst: *mut u16,
            dst_stride: i32,
            alpha_q3: i32,
            bd: i32,
        ) {
            cfl_predict_hbd_neon(pred_buf_q3, dst, dst_stride, alpha_q3, bd, $w, $h);
        }
    };
}

cfl_predict_hbd!(cfl_predict_hbd_4x4_neon, 4, 4);
cfl_predict_hbd!(cfl_predict_hbd_4x8_neon, 4, 8);
cfl_predict_hbd!(cfl_predict_hbd_4x16_neon, 4, 16);
cfl_predict_hbd!(cfl_predict_hbd_8x4_neon, 8, 4);
cfl_predict_hbd!(cfl_predict_hbd_8x8_neon, 8, 8);
cfl_predict_hbd!(cfl_predict_hbd_8x16_neon, 8, 16);
cfl_predict_hbd!(cfl_predict_hbd_8x32_neon, 8, 32);
cfl_predict_hbd!(cfl_predict_hbd_16x4_neon, 16, 4);
cfl_predict_hbd!(cfl_predict_hbd_16x8_neon, 16, 8);
cfl_predict_hbd!(cfl_predict_hbd_16x16_neon, 16, 16);
cfl_predict_hbd!(cfl_predict_hbd_16x32_neon, 16, 32);
cfl_predict_hbd!(cfl_predict_hbd_32x8_neon, 32, 8);
cfl_predict_hbd!(cfl_predict_hbd_32x16_neon, 32, 16);
cfl_predict_hbd!(cfl_predict_hbd_32x32_neon, 32, 32);

#[no_mangle]
pub unsafe extern "C-unwind" fn cfl_get_predict_hbd_fn_neon(tx_size: TX_SIZE) -> cfl_predict_hbd_fn {
    static pred: [cfl_predict_hbd_fn; TX_SIZES_ALL] = [
        Some(cfl_predict_hbd_4x4_neon),   // 4x4
        Some(cfl_predict_hbd_8x8_neon),   // 8x8
        Some(cfl_predict_hbd_16x16_neon), // 16x16
        Some(cfl_predict_hbd_32x32_neon), // 32x32
        None,                             // 64x64 (invalid CFL size)
        Some(cfl_predict_hbd_4x8_neon),   // 4x8
        Some(cfl_predict_hbd_8x4_neon),   // 8x4
        Some(cfl_predict_hbd_8x16_neon),  // 8x16
        Some(cfl_predict_hbd_16x8_neon),  // 16x8
        Some(cfl_predict_hbd_16x32_neon), // 16x32
        Some(cfl_predict_hbd_32x16_neon), // 32x16
        None,                             // 32x64 (invalid CFL size)
        None,                             // 64x32 (invalid CFL size)
        Some(cfl_predict_hbd_4x16_neon),  // 4x16
        Some(cfl_predict_hbd_16x4_neon),  // 16x4
        Some(cfl_predict_hbd_8x32_neon),  // 8x32
        Some(cfl_predict_hbd_32x8_neon),  // 32x8
        None,                             // 16x64 (invalid CFL size)
        None,                             // 64x16 (invalid CFL size)
    ];
    // Modulo TX_SIZES_ALL to ensure that an attacker won't be able to
    // index the function pointer array out of bounds.
    pred[tx_size as usize % TX_SIZES_ALL]
}
