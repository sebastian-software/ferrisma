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

// Translated by hand from upstream/aom/aom_dsp/simd/v64_intrinsics_x86.h
// (ADR 0056, ADR 0058).
//
// This header has no build flag of its own: upstream includes it, through
// aom_dsp/aom_simd.h and v256_intrinsics_x86.h, into each file that uses
// libaom's portable SIMD abstraction, and compiles it with that file's flag.
// The only such file in the x86 kernel set is av1/common/x86/cdef_block_avx2.c
// (-mavx2), so this translation takes the branches that are active when
// __SSSE3__, __SSE4_1__ and __AVX2__ are all defined, and every function
// carries #[target_feature(enable = "avx2")], the flag it is compiled with
// upstream. SIMD_INLINE functions become `#[inline]` functions; macros that
// require an immediate become functions with a const generic.
//
// Only the functions that cdef_block_avx2.c and av1/common/cdef_block_simd.h
// use, directly or through other abstraction functions, are translated; they
// keep the order of the header.
//
// u32_store_aligned() stores through a plain `uint32_t *` in C. x86 scalar
// stores have no alignment requirement and the C compilers emit a plain MOV,
// so the translation uses write_unaligned(), which emits the same store and
// stays defined in Rust for every pointer the C accepts.
use core::arch::x86_64::*;

pub type v64 = __m128i;

#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn v64_low_u32(a: v64) -> u32 {
    _mm_cvtsi128_si32(a) as u32
}

#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn v64_high_u32(a: v64) -> u32 {
    _mm_cvtsi128_si32(_mm_srli_si128::<4>(a)) as u32
}

#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn u32_store_aligned(p: *mut core::ffi::c_void, a: u32) {
    (p as *mut u32).write_unaligned(a);
}

#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn v64_load_aligned(p: *const core::ffi::c_void) -> v64 {
    _mm_loadl_epi64(p as *const __m128i)
}

#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn v64_load_unaligned(p: *const core::ffi::c_void) -> v64 {
    _mm_loadl_epi64(p as *const __m128i)
}

#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn v64_store_aligned(p: *mut core::ffi::c_void, a: v64) {
    _mm_storel_epi64(p as *mut __m128i, a);
}

#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn v64_zero() -> v64 {
    _mm_setzero_si128()
}
