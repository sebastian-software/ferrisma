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

// Translated by hand from upstream/aom/aom_dsp/simd/v256_intrinsics_x86.h
// (ADR 0056, ADR 0058).
//
// This header has no build flag of its own: upstream includes it, through
// aom_dsp/aom_simd.h, into each file that uses libaom's portable SIMD
// abstraction, and compiles it with that file's flag. The only such file in
// the x86 kernel set is av1/common/x86/cdef_block_avx2.c (-mavx2), so this
// translation takes the `defined(__AVX2__)` half of the header (v256 is
// __m256i; the v256_intrinsics_v128.h fallback for builds without AVX2 is not
// used) and, inside it, the branches that are active when __SSSE3__,
// __SSE4_1__ and __AVX2__ are all defined. Every function carries
// #[target_feature(enable = "avx2")], the flag it is compiled with upstream.
// SIMD_INLINE functions become `#[inline]` functions; macros that require an
// immediate become functions with a const generic.
//
// Only the functions that cdef_block_avx2.c and av1/common/cdef_block_simd.h
// use, directly or through other abstraction functions, are translated; they
// keep the order of the header.
use core::arch::x86_64::*;

use crate::simd::aom_dsp__simd__v128_intrinsics_x86::*;
use crate::simd::aom_dsp__simd__v64_intrinsics_x86::*;

// _MM_SHUFFLE(z, y, x, w) from <xmmintrin.h>. core::arch's version is not yet
// usable in a const generic argument, so the macro is translated here; it
// shadows the glob import.
#[allow(non_snake_case)]
const fn _MM_SHUFFLE(z: i32, y: i32, x: i32, w: i32) -> i32 {
    (z << 6) | (y << 4) | (x << 2) | w
}

pub type v256 = __m256i;

#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn v256_low_v128(a: v256) -> v128 {
    _mm256_castsi256_si128(a)
}

#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn v256_high_v128(a: v256) -> v128 {
    _mm256_extracti128_si256::<1>(a)
}

#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn v256_from_v128(a: v128, b: v128) -> v256 {
    // gcc seems to be missing _mm256_set_m128i()
    _mm256_inserti128_si256::<1>(_mm256_castsi128_si256(b), a)
}

#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn v256_from_v64(a: v64, b: v64, c: v64, d: v64) -> v256 {
    v256_from_v128(v128_from_v64(a, b), v128_from_v64(c, d))
}

#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn v256_zero() -> v256 {
    _mm256_setzero_si256()
}

#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn v256_dup_16(x: u16) -> v256 {
    _mm256_set1_epi16(x as i16)
}

#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn v256_add_16(a: v256, b: v256) -> v256 {
    _mm256_add_epi16(a, b)
}

#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn v256_sub_16(a: v256, b: v256) -> v256 {
    _mm256_sub_epi16(a, b)
}

#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn v256_ssub_u16(a: v256, b: v256) -> v256 {
    _mm256_subs_epu16(a, b)
}

#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn v256_abs_s16(a: v256) -> v256 {
    _mm256_abs_epi16(a)
}

#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn v256_pack_s16_u8(a: v256, b: v256) -> v256 {
    _mm256_permute4x64_epi64::<{ _MM_SHUFFLE(3, 1, 2, 0) }>(_mm256_packus_epi16(b, a))
}

#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn v256_xor(a: v256, b: v256) -> v256 {
    _mm256_xor_si256(a, b)
}

#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn v256_and(a: v256, b: v256) -> v256 {
    _mm256_and_si256(a, b)
}

#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn v256_mullo_s16(a: v256, b: v256) -> v256 {
    _mm256_mullo_epi16(a, b)
}

#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn v256_max_u8(a: v256, b: v256) -> v256 {
    _mm256_max_epu8(a, b)
}

#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn v256_min_s16(a: v256, b: v256) -> v256 {
    _mm256_min_epi16(a, b)
}

#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn v256_max_s16(a: v256, b: v256) -> v256 {
    _mm256_max_epi16(a, b)
}

#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn v256_cmplt_s16(a: v256, b: v256) -> v256 {
    _mm256_cmpgt_epi16(b, a)
}

#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn v256_shr_u16(a: v256, c: u32) -> v256 {
    _mm256_srl_epi16(a, _mm_cvtsi32_si128(c as i32))
}

/* These intrinsics require immediate values, so we must use #defines
   to enforce that. */
// #define v256_shr_n_s16(a, c) _mm256_srai_epi16(a, c)
#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn v256_shr_n_s16<const C: i32>(a: v256) -> v256 {
    _mm256_srai_epi16::<C>(a)
}
