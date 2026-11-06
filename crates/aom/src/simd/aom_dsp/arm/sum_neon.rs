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

// Translated by hand from upstream/aom/aom_dsp/arm/sum_neon.h (ADR 0056).
// Function names, order, and intrinsic sequences follow the original; only the
// AOM_ARCH_AARCH64 branches are retained. C array parameters (`const
// uint32x4_t sum[4]`) decay to pointers and stay raw pointers here.
use core::arch::aarch64::*;

#[inline(always)]
pub unsafe fn horizontal_add_u8x8(a: uint8x8_t) -> i32 {
    vaddlv_u8(a) as i32
}

#[inline(always)]
pub unsafe fn horizontal_add_s16x8(a: int16x8_t) -> i32 {
    vaddlvq_s16(a)
}

#[inline(always)]
pub unsafe fn horizontal_add_s32x4(a: int32x4_t) -> i32 {
    vaddvq_s32(a)
}

#[inline(always)]
pub unsafe fn horizontal_add_s64x2(a: int64x2_t) -> i64 {
    vaddvq_s64(a)
}

#[inline(always)]
pub unsafe fn horizontal_add_u64x2(a: uint64x2_t) -> u64 {
    vaddvq_u64(a)
}

#[inline(always)]
pub unsafe fn horizontal_long_add_u32x4(a: uint32x4_t) -> u64 {
    vaddlvq_u32(a)
}

#[inline(always)]
pub unsafe fn horizontal_long_add_s32x4(a: int32x4_t) -> i64 {
    vaddlvq_s32(a)
}

#[inline(always)]
pub unsafe fn horizontal_add_u32x4(a: uint32x4_t) -> u32 {
    vaddvq_u32(a)
}

#[inline(always)]
pub unsafe fn horizontal_add_4d_u32x4(sum: *const uint32x4_t) -> uint32x4_t {
    let res01 = vpaddq_u32(*sum.add(0), *sum.add(1));
    let res23 = vpaddq_u32(*sum.add(2), *sum.add(3));
    vpaddq_u32(res01, res23)
}

#[inline(always)]
pub unsafe fn horizontal_add_4d_s32x4(sum: *const int32x4_t) -> int32x4_t {
    let res01 = vpaddq_s32(*sum.add(0), *sum.add(1));
    let res23 = vpaddq_s32(*sum.add(2), *sum.add(3));
    vpaddq_s32(res01, res23)
}

#[inline(always)]
pub unsafe fn horizontal_long_add_u16x8(vec_lo: uint16x8_t, vec_hi: uint16x8_t) -> u32 {
    vaddlvq_u16(vec_lo).wrapping_add(vaddlvq_u16(vec_hi))
}

#[inline(always)]
pub unsafe fn horizontal_long_add_4d_u16x8(sum_lo: *const uint16x8_t,
                                           sum_hi: *const uint16x8_t) -> uint32x4_t {
    let a0 = vpaddlq_u16(*sum_lo.add(0));
    let a1 = vpaddlq_u16(*sum_lo.add(1));
    let a2 = vpaddlq_u16(*sum_lo.add(2));
    let a3 = vpaddlq_u16(*sum_lo.add(3));
    let b0 = vpadalq_u16(a0, *sum_hi.add(0));
    let b1 = vpadalq_u16(a1, *sum_hi.add(1));
    let b2 = vpadalq_u16(a2, *sum_hi.add(2));
    let b3 = vpadalq_u16(a3, *sum_hi.add(3));
    let c0 = vpaddq_u32(b0, b1);
    let c1 = vpaddq_u32(b2, b3);
    vpaddq_u32(c0, c1)
}

#[inline(always)]
pub unsafe fn horizontal_add_u16x8(a: uint16x8_t) -> u32 {
    vaddlvq_u16(a)
}

#[inline(always)]
pub unsafe fn horizontal_add_4d_u16x8(sum: *const uint16x8_t) -> uint32x4_t {
    let a0 = vpaddq_u16(*sum.add(0), *sum.add(1));
    let a1 = vpaddq_u16(*sum.add(2), *sum.add(3));
    let b0 = vpaddq_u16(a0, a1);
    vpaddlq_u16(b0)
}

#[inline(always)]
pub unsafe fn horizontal_add_4d_s16x8(sum: *const int16x8_t) -> int32x4_t {
    let a0 = vpaddq_s16(*sum.add(0), *sum.add(1));
    let a1 = vpaddq_s16(*sum.add(2), *sum.add(3));
    let b0 = vpaddq_s16(a0, a1);
    vpaddlq_s16(b0)
}

#[inline(always)]
pub unsafe fn horizontal_add_u32x2(a: uint32x2_t) -> u32 {
    vaddv_u32(a)
}

#[inline(always)]
pub unsafe fn horizontal_long_add_u32x2(a: uint32x2_t) -> u64 {
    vaddlv_u32(a)
}

#[inline(always)]
pub unsafe fn horizontal_add_u16x4(a: uint16x4_t) -> u32 {
    vaddlv_u16(a)
}

#[inline(always)]
pub unsafe fn horizontal_add_2d_u32(a: uint32x4_t, b: uint32x4_t) -> uint32x4_t {
    vpaddq_u32(a, b)
}

#[inline(always)]
pub unsafe fn horizontal_add_2d_s32(a: int32x4_t, b: int32x4_t) -> int32x4_t {
    vpaddq_s32(a, b)
}

#[inline(always)]
pub unsafe fn add_pairwise_s32x4(a: int32x4_t) -> int32x2_t {
    vget_low_s32(vpaddq_s32(a, a))
}

#[inline(always)]
pub unsafe fn horizontal_long_add_u32x4_x2(a: *const uint32x4_t) -> u64 {
    horizontal_long_add_u32x4(*a.add(0)).wrapping_add(horizontal_long_add_u32x4(*a.add(1)))
}

#[inline(always)]
pub unsafe fn horizontal_long_add_u32x4_x4(a: *const uint32x4_t) -> u64 {
    let mut sum = vpaddlq_u32(*a.add(0));
    sum = vpadalq_u32(sum, *a.add(1));
    sum = vpadalq_u32(sum, *a.add(2));
    sum = vpadalq_u32(sum, *a.add(3));

    horizontal_add_u64x2(sum)
}

#[inline(always)]
pub unsafe fn horizontal_long_add_u32x4_x8(a: *const uint32x4_t) -> u64 {
    let mut sum: [uint64x2_t; 2] = [vpaddlq_u32(*a.add(0)), vpaddlq_u32(*a.add(1))];
    sum[0] = vpadalq_u32(sum[0], *a.add(2));
    sum[1] = vpadalq_u32(sum[1], *a.add(3));
    sum[0] = vpadalq_u32(sum[0], *a.add(4));
    sum[1] = vpadalq_u32(sum[1], *a.add(5));
    sum[0] = vpadalq_u32(sum[0], *a.add(6));
    sum[1] = vpadalq_u32(sum[1], *a.add(7));

    horizontal_add_u64x2(vaddq_u64(sum[0], sum[1]))
}

#[inline(always)]
pub unsafe fn horizontal_long_add_u32x4_x16(a: *const uint32x4_t) -> u64 {
    let mut sum: [uint64x2_t; 2] = [vpaddlq_u32(*a.add(0)), vpaddlq_u32(*a.add(1))];
    sum[0] = vpadalq_u32(sum[0], *a.add(2));
    sum[1] = vpadalq_u32(sum[1], *a.add(3));
    sum[0] = vpadalq_u32(sum[0], *a.add(4));
    sum[1] = vpadalq_u32(sum[1], *a.add(5));
    sum[0] = vpadalq_u32(sum[0], *a.add(6));
    sum[1] = vpadalq_u32(sum[1], *a.add(7));
    sum[0] = vpadalq_u32(sum[0], *a.add(8));
    sum[1] = vpadalq_u32(sum[1], *a.add(9));
    sum[0] = vpadalq_u32(sum[0], *a.add(10));
    sum[1] = vpadalq_u32(sum[1], *a.add(11));
    sum[0] = vpadalq_u32(sum[0], *a.add(12));
    sum[1] = vpadalq_u32(sum[1], *a.add(13));
    sum[0] = vpadalq_u32(sum[0], *a.add(14));
    sum[1] = vpadalq_u32(sum[1], *a.add(15));

    horizontal_add_u64x2(vaddq_u64(sum[0], sum[1]))
}
