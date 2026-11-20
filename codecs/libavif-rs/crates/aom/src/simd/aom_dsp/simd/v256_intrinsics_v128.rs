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

// Translated by hand from upstream/aom/aom_dsp/simd/v256_intrinsics_v128.h
// (ADR 0056, ADR 0058), the x86 (HAVE_SSE2) half.
//
// aom_dsp/simd/v256_intrinsics_x86.h includes this header instead of its own
// definitions when __AVX2__ is not defined: v256 is then a pair of v128
// (val[0] the low half, val[1] the high half) and every operation runs on
// both halves with the v128 functions of v128_intrinsics_x86.h. The only file
// in the x86 kernel set that includes it so is av1/common/x86/cdef_block_sse4.c
// (-msse4.1), so every function carries #[target_feature(enable = "sse4.1")].
// SIMD_INLINE functions become `#[inline]` functions; the v256_shr_n_s16
// macro, which requires an immediate, becomes a function with a const generic.
//
// Only the functions that av1/common/cdef_block_simd.h uses are translated;
// they keep the order of the header.
use crate::simd::aom_dsp__simd__v128_intrinsics_x86::*;
use crate::simd::aom_dsp__simd__v64_intrinsics_x86::*;

// typedef struct { v128 val[2]; } v256;
#[derive(Clone, Copy)]
#[repr(C)]
pub struct v256 {
    pub val: [v128; 2],
}

#[inline]
#[target_feature(enable = "sse4.1")]
pub unsafe fn v256_low_v128(a: v256) -> v128 {
    a.val[0]
}

#[inline]
#[target_feature(enable = "sse4.1")]
pub unsafe fn v256_high_v128(a: v256) -> v128 {
    a.val[1]
}

#[inline]
#[target_feature(enable = "sse4.1")]
pub unsafe fn v256_from_v128(hi: v128, lo: v128) -> v256 {
    v256 { val: [lo, hi] }
}

#[inline]
#[target_feature(enable = "sse4.1")]
pub unsafe fn v256_from_v64(a: v64, b: v64, c: v64, d: v64) -> v256 {
    v256_from_v128(v128_from_v64(a, b), v128_from_v64(c, d))
}

#[inline]
#[target_feature(enable = "sse4.1")]
pub unsafe fn v256_zero() -> v256 {
    v256_from_v128(v128_zero(), v128_zero())
}

#[inline]
#[target_feature(enable = "sse4.1")]
pub unsafe fn v256_dup_16(x: u16) -> v256 {
    let t = v128_dup_16(x);
    v256_from_v128(t, t)
}

#[inline]
#[target_feature(enable = "sse4.1")]
pub unsafe fn v256_xor(a: v256, b: v256) -> v256 {
    v256_from_v128(v128_xor(a.val[1], b.val[1]), v128_xor(a.val[0], b.val[0]))
}

#[inline]
#[target_feature(enable = "sse4.1")]
pub unsafe fn v256_and(a: v256, b: v256) -> v256 {
    v256_from_v128(v128_and(a.val[1], b.val[1]), v128_and(a.val[0], b.val[0]))
}

#[inline]
#[target_feature(enable = "sse4.1")]
pub unsafe fn v256_add_16(a: v256, b: v256) -> v256 {
    v256_from_v128(v128_add_16(a.val[1], b.val[1]), v128_add_16(a.val[0], b.val[0]))
}

#[inline]
#[target_feature(enable = "sse4.1")]
pub unsafe fn v256_sub_16(a: v256, b: v256) -> v256 {
    v256_from_v128(v128_sub_16(a.val[1], b.val[1]), v128_sub_16(a.val[0], b.val[0]))
}

#[inline]
#[target_feature(enable = "sse4.1")]
pub unsafe fn v256_ssub_u16(a: v256, b: v256) -> v256 {
    v256_from_v128(v128_ssub_u16(a.val[1], b.val[1]), v128_ssub_u16(a.val[0], b.val[0]))
}

#[inline]
#[target_feature(enable = "sse4.1")]
pub unsafe fn v256_abs_s16(a: v256) -> v256 {
    v256_from_v128(v128_abs_s16(a.val[1]), v128_abs_s16(a.val[0]))
}

#[inline]
#[target_feature(enable = "sse4.1")]
pub unsafe fn v256_mullo_s16(a: v256, b: v256) -> v256 {
    v256_from_v128(v128_mullo_s16(a.val[1], b.val[1]), v128_mullo_s16(a.val[0], b.val[0]))
}

#[inline]
#[target_feature(enable = "sse4.1")]
pub unsafe fn v256_max_u8(a: v256, b: v256) -> v256 {
    v256_from_v128(v128_max_u8(a.val[1], b.val[1]), v128_max_u8(a.val[0], b.val[0]))
}

#[inline]
#[target_feature(enable = "sse4.1")]
pub unsafe fn v256_min_s16(a: v256, b: v256) -> v256 {
    v256_from_v128(v128_min_s16(a.val[1], b.val[1]), v128_min_s16(a.val[0], b.val[0]))
}

#[inline]
#[target_feature(enable = "sse4.1")]
pub unsafe fn v256_max_s16(a: v256, b: v256) -> v256 {
    v256_from_v128(v128_max_s16(a.val[1], b.val[1]), v128_max_s16(a.val[0], b.val[0]))
}

#[inline]
#[target_feature(enable = "sse4.1")]
pub unsafe fn v256_pack_s16_u8(a: v256, b: v256) -> v256 {
    v256_from_v128(v128_pack_s16_u8(a.val[1], a.val[0]),
                   v128_pack_s16_u8(b.val[1], b.val[0]))
}

#[inline]
#[target_feature(enable = "sse4.1")]
pub unsafe fn v256_cmplt_s16(a: v256, b: v256) -> v256 {
    v256_from_v128(v128_cmplt_s16(a.val[1], b.val[1]), v128_cmplt_s16(a.val[0], b.val[0]))
}

#[inline]
#[target_feature(enable = "sse4.1")]
pub unsafe fn v256_shr_u16(a: v256, c: u32) -> v256 {
    v256_from_v128(v128_shr_u16(a.val[1], c), v128_shr_u16(a.val[0], c))
}

// #define v256_shr_n_s16(a, n) \
//   v256_from_v128(v128_shr_n_s16(a.val[1], n), v128_shr_n_s16(a.val[0], n))
#[inline]
#[target_feature(enable = "sse4.1")]
pub unsafe fn v256_shr_n_s16<const N: i32>(a: v256) -> v256 {
    v256_from_v128(v128_shr_n_s16::<N>(a.val[1]), v128_shr_n_s16::<N>(a.val[0]))
}
