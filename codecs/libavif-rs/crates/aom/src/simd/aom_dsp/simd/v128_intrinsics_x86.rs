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

// Translated by hand from upstream/aom/aom_dsp/simd/v128_intrinsics_x86.h
// (ADR 0056, ADR 0058).
//
// This header has no build flag of its own: upstream includes it, through
// aom_dsp/aom_simd.h and v256_intrinsics_x86.h, into each file that uses
// libaom's portable SIMD abstraction, and compiles it with that file's flag.
// Two such files are in the x86 kernel set: av1/common/x86/cdef_block_avx2.c
// (-mavx2) and av1/common/x86/cdef_block_sse4.c (-msse4.1). The header tests
// only __SSSE3__ and __SSE4_1__, which both flags define, so both files get
// the same branches (for example _mm_lddqu_si128 in v128_load_unaligned,
// _mm_alignr_epi8 in v128_align, _mm_mullo_epi32 and _mm_max_epi32), and that
// is what is translated. Every function carries
// #[target_feature(enable = "sse4.1")], the smaller of the two flags, so the
// SSE4.1 kernels can call it; the AVX2 kernels inline it unchanged.
// SIMD_INLINE functions become `#[inline]` functions; macros that require an
// immediate become functions with a const generic.
//
// v128_align() has two spellings upstream (an inline function for optimizing
// GCC, a macro otherwise); with __SSSE3__ both are `c ? _mm_alignr_epi8(a, b,
// c) : b`, which is what is translated.
//
// Only the functions that cdef_block_avx2.c, cdef_block_sse4.c and
// av1/common/cdef_block_simd.h use, directly or through other abstraction
// functions, are translated; they keep the order of the header.
use core::arch::x86_64::*;

use crate::simd::aom_dsp__simd__v64_intrinsics_x86::*;

pub type v128 = __m128i;

#[inline]
#[target_feature(enable = "sse4.1")]
pub unsafe fn v128_low_u32(a: v128) -> u32 {
    _mm_cvtsi128_si32(a) as u32
}

#[inline]
#[target_feature(enable = "sse4.1")]
pub unsafe fn v128_low_v64(a: v128) -> v64 {
    _mm_unpacklo_epi64(a, v64_zero())
}

#[inline]
#[target_feature(enable = "sse4.1")]
pub unsafe fn v128_high_v64(a: v128) -> v64 {
    _mm_srli_si128::<8>(a)
}

#[inline]
#[target_feature(enable = "sse4.1")]
pub unsafe fn v128_from_v64(a: v64, b: v64) -> v128 {
    _mm_unpacklo_epi64(b, a)
}

#[inline]
#[target_feature(enable = "sse4.1")]
pub unsafe fn v128_from_32(a: u32, b: u32, c: u32, d: u32) -> v128 {
    _mm_set_epi32(a as i32, b as i32, c as i32, d as i32)
}

#[inline]
#[target_feature(enable = "sse4.1")]
pub unsafe fn v128_load_aligned(p: *const core::ffi::c_void) -> v128 {
    _mm_load_si128(p as *const __m128i)
}

#[inline]
#[target_feature(enable = "sse4.1")]
pub unsafe fn v128_load_unaligned(p: *const core::ffi::c_void) -> v128 {
    // #if defined(__SSSE3__)
    _mm_lddqu_si128(p as *const __m128i)
}

#[inline]
#[target_feature(enable = "sse4.1")]
pub unsafe fn v128_store_unaligned(p: *mut core::ffi::c_void, a: v128) {
    _mm_storeu_si128(p as *mut __m128i, a);
}

// The following function requires an immediate.
#[inline]
#[target_feature(enable = "sse4.1")]
pub unsafe fn v128_align<const C: i32>(a: v128, b: v128) -> v128 {
    if C != 0 { _mm_alignr_epi8::<C>(a, b) } else { b }
}

#[inline]
#[target_feature(enable = "sse4.1")]
pub unsafe fn v128_zero() -> v128 {
    _mm_setzero_si128()
}

#[inline]
#[target_feature(enable = "sse4.1")]
pub unsafe fn v128_dup_16(x: u16) -> v128 {
    _mm_set1_epi16(x as i16)
}

#[inline]
#[target_feature(enable = "sse4.1")]
pub unsafe fn v128_dup_32(x: u32) -> v128 {
    _mm_set1_epi32(x as i32)
}

#[inline]
#[target_feature(enable = "sse4.1")]
pub unsafe fn v128_add_16(a: v128, b: v128) -> v128 {
    _mm_add_epi16(a, b)
}

#[inline]
#[target_feature(enable = "sse4.1")]
pub unsafe fn v128_add_32(a: v128, b: v128) -> v128 {
    _mm_add_epi32(a, b)
}

#[inline]
#[target_feature(enable = "sse4.1")]
pub unsafe fn v128_sub_16(a: v128, b: v128) -> v128 {
    _mm_sub_epi16(a, b)
}

#[inline]
#[target_feature(enable = "sse4.1")]
pub unsafe fn v128_ssub_u16(a: v128, b: v128) -> v128 {
    _mm_subs_epu16(a, b)
}

#[inline]
#[target_feature(enable = "sse4.1")]
pub unsafe fn v128_abs_s16(a: v128) -> v128 {
    // #if defined(__SSSE3__)
    _mm_abs_epi16(a)
}

#[inline]
#[target_feature(enable = "sse4.1")]
pub unsafe fn v128_ziplo_16(a: v128, b: v128) -> v128 {
    _mm_unpacklo_epi16(b, a)
}

#[inline]
#[target_feature(enable = "sse4.1")]
pub unsafe fn v128_ziphi_16(a: v128, b: v128) -> v128 {
    _mm_unpackhi_epi16(b, a)
}

#[inline]
#[target_feature(enable = "sse4.1")]
pub unsafe fn v128_ziplo_32(a: v128, b: v128) -> v128 {
    _mm_unpacklo_epi32(b, a)
}

#[inline]
#[target_feature(enable = "sse4.1")]
pub unsafe fn v128_ziphi_32(a: v128, b: v128) -> v128 {
    _mm_unpackhi_epi32(b, a)
}

#[inline]
#[target_feature(enable = "sse4.1")]
pub unsafe fn v128_ziplo_64(a: v128, b: v128) -> v128 {
    _mm_unpacklo_epi64(b, a)
}

#[inline]
#[target_feature(enable = "sse4.1")]
pub unsafe fn v128_ziphi_64(a: v128, b: v128) -> v128 {
    _mm_unpackhi_epi64(b, a)
}

#[inline]
#[target_feature(enable = "sse4.1")]
pub unsafe fn v128_unpack_u8_s16(a: v64) -> v128 {
    _mm_unpacklo_epi8(a, _mm_setzero_si128())
}

#[inline]
#[target_feature(enable = "sse4.1")]
pub unsafe fn v128_pack_s32_s16(a: v128, b: v128) -> v128 {
    _mm_packs_epi32(b, a)
}

#[inline]
#[target_feature(enable = "sse4.1")]
pub unsafe fn v128_pack_s16_u8(a: v128, b: v128) -> v128 {
    _mm_packus_epi16(b, a)
}

#[inline]
#[target_feature(enable = "sse4.1")]
pub unsafe fn v128_pack_s16_s8(a: v128, b: v128) -> v128 {
    _mm_packs_epi16(b, a)
}

#[inline]
#[target_feature(enable = "sse4.1")]
pub unsafe fn v128_shuffle_8(x: v128, pattern: v128) -> v128 {
    // #if defined(__SSSE3__)
    _mm_shuffle_epi8(x, pattern)
}

#[inline]
#[target_feature(enable = "sse4.1")]
pub unsafe fn v128_xor(a: v128, b: v128) -> v128 {
    _mm_xor_si128(a, b)
}

#[inline]
#[target_feature(enable = "sse4.1")]
pub unsafe fn v128_and(a: v128, b: v128) -> v128 {
    _mm_and_si128(a, b)
}

#[inline]
#[target_feature(enable = "sse4.1")]
pub unsafe fn v128_mullo_s16(a: v128, b: v128) -> v128 {
    _mm_mullo_epi16(a, b)
}

#[inline]
#[target_feature(enable = "sse4.1")]
pub unsafe fn v128_mullo_s32(a: v128, b: v128) -> v128 {
    // #if defined(__SSE4_1__)
    _mm_mullo_epi32(a, b)
}

#[inline]
#[target_feature(enable = "sse4.1")]
pub unsafe fn v128_madd_s16(a: v128, b: v128) -> v128 {
    _mm_madd_epi16(a, b)
}

#[inline]
#[target_feature(enable = "sse4.1")]
pub unsafe fn v128_max_u8(a: v128, b: v128) -> v128 {
    _mm_max_epu8(a, b)
}

#[inline]
#[target_feature(enable = "sse4.1")]
pub unsafe fn v128_movemask_8(a: v128) -> u32 {
    _mm_movemask_epi8(a) as u32
}

#[inline]
#[target_feature(enable = "sse4.1")]
pub unsafe fn v128_min_s16(a: v128, b: v128) -> v128 {
    _mm_min_epi16(a, b)
}

#[inline]
#[target_feature(enable = "sse4.1")]
pub unsafe fn v128_max_s16(a: v128, b: v128) -> v128 {
    _mm_max_epi16(a, b)
}

#[inline]
#[target_feature(enable = "sse4.1")]
pub unsafe fn v128_max_s32(a: v128, b: v128) -> v128 {
    // #if defined(__SSE4_1__)
    _mm_max_epi32(a, b)
}

#[inline]
#[target_feature(enable = "sse4.1")]
pub unsafe fn v128_cmplt_s16(a: v128, b: v128) -> v128 {
    _mm_cmplt_epi16(a, b)
}

#[inline]
#[target_feature(enable = "sse4.1")]
pub unsafe fn v128_cmpeq_32(a: v128, b: v128) -> v128 {
    _mm_cmpeq_epi32(a, b)
}

#[inline]
#[target_feature(enable = "sse4.1")]
pub unsafe fn v128_shr_u16(a: v128, c: u32) -> v128 {
    _mm_srl_epi16(a, _mm_cvtsi32_si128(c as i32))
}

#[inline]
#[target_feature(enable = "sse4.1")]
pub unsafe fn v128_shr_s16(a: v128, c: u32) -> v128 {
    _mm_sra_epi16(a, _mm_cvtsi32_si128(c as i32))
}

/* These intrinsics require immediate values, so we must use #defines
   to enforce that. */
// #define v128_shl_n_byte(a, c) _mm_slli_si128(a, (c) & 127)
// The `& 127` cannot be applied to a const generic argument on stable Rust;
// the assertion checks at compile time that it leaves C unchanged, which holds
// for every byte count the callers pass.
#[inline]
#[target_feature(enable = "sse4.1")]
pub unsafe fn v128_shl_n_byte<const C: i32>(a: v128) -> v128 {
    const { assert!(C & 127 == C) };
    _mm_slli_si128::<C>(a)
}

// #define v128_shr_n_byte(a, c) _mm_srli_si128(a, (c) & 127)
#[inline]
#[target_feature(enable = "sse4.1")]
pub unsafe fn v128_shr_n_byte<const C: i32>(a: v128) -> v128 {
    const { assert!(C & 127 == C) };
    _mm_srli_si128::<C>(a)
}

// #define v128_shr_n_s16(a, c) _mm_srai_epi16(a, c)
#[inline]
#[target_feature(enable = "sse4.1")]
pub unsafe fn v128_shr_n_s16<const C: i32>(a: v128) -> v128 {
    _mm_srai_epi16::<C>(a)
}
