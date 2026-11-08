/*
 * Copyright (c) 2018, Alliance for Open Media. All rights reserved.
 *
 * This source code is subject to the terms of the BSD 2 Clause License and
 * the Alliance for Open Media Patent License 1.0. If the BSD 2 Clause License
 * was not distributed with this source code in the LICENSE file, you can
 * obtain it at www.aomedia.org/license/software. If the Alliance for Open
 * Media Patent License 1.0 was not distributed with this source code in the
 * PATENTS file, you can obtain it at www.aomedia.org/license/patent.
 */

// Translated by hand from upstream/aom/aom_dsp/x86/synonyms_avx2.h (ADR 0056,
// ADR 0058). Function names, order, comments, and intrinsic sequences follow
// the original; every helper carries the smallest target feature its own
// intrinsics need (`avx` for the 256-bit loads, stores, and lane inserts,
// `avx2` where an AVX2 integer intrinsic is used).
//
// Translation notes:
// - `const void *` / `void *` parameters become `*const c_void` /
//   `*mut c_void`.
// - core::arch's `_mm_loadh_pd` dereferences its pointer as an aligned
//   `*const f64`, while MOVHPD and the C helper accept any address. The high
//   halves in yy_loadu_4x64 are therefore read with ptr::read_unaligned into a
//   local, and `_mm_loadh_pd` loads from that local: the same bits reach the
//   same lane.
// - yy_roundn_epu16 passes its run-time `bits - 1` to `_mm256_srli_epi16`;
//   as in synonyms.rs, the run-time count uses the count-register intrinsic
//   `_mm256_srl_epi16(v, _mm_cvtsi32_si128(n))`, which GCC and Clang emit for
//   a non-constant count and which is bit-identical for every `int` count.

use core::arch::x86_64::*;
use core::ffi::c_void;
use core::ptr;

/*
 * Various reusable shorthands for x86 SIMD intrinsics.
 *
 * Intrinsics prefixed with xx_ operate on or return 128bit XMM registers.
 * Intrinsics prefixed with yy_ operate on or return 256bit YMM registers.
 */

// Loads and stores to do away with the tedium of casting the address
// to the right type.
#[inline]
#[target_feature(enable = "avx")]
pub unsafe fn yy_load_256(a: *const c_void) -> __m256i {
    _mm256_load_si256(a as *const __m256i)
}

#[inline]
#[target_feature(enable = "avx")]
pub unsafe fn yy_loadu_256(a: *const c_void) -> __m256i {
    _mm256_loadu_si256(a as *const __m256i)
}

#[inline]
#[target_feature(enable = "avx")]
pub unsafe fn yy_store_256(a: *mut c_void, v: __m256i) {
    _mm256_store_si256(a as *mut __m256i, v);
}

#[inline]
#[target_feature(enable = "avx")]
pub unsafe fn yy_storeu_256(a: *mut c_void, v: __m256i) {
    _mm256_storeu_si256(a as *mut __m256i, v);
}

// Fill an AVX register using an interleaved pair of values, ie. set the
// 16 channels to {a, b} repeated 8 times, using the same channel ordering
// as when a register is stored to / loaded from memory.
//
// This is useful for rearranging filter kernels for use with the _mm_madd_epi16
// instruction
#[inline]
#[target_feature(enable = "avx")]
pub unsafe fn yy_set2_epi16(a: i16, b: i16) -> __m256i {
    _mm256_setr_epi16(a, b, a, b, a, b, a, b, a, b, a, b, a, b, a, b)
}

// Some compilers don't have _mm256_set_m128i defined in immintrin.h. We
// therefore define an equivalent function using a different intrinsic.
// ([ hi ], [ lo ]) -> [ hi ][ lo ]
#[inline]
#[target_feature(enable = "avx")]
pub unsafe fn yy_set_m128i(hi: __m128i, lo: __m128i) -> __m256i {
    _mm256_insertf128_si256::<1>(_mm256_castsi128_si256(lo), hi)
}

// This behaves similarly to _mm256_set_epi64x(), but avoids undefined
// sanitizer warnings when loading values from unaligned buffers using
// `*(int64_t *)val`.
#[inline]
#[target_feature(enable = "avx")]
pub unsafe fn yy_loadu_4x64(e3: *const c_void, e2: *const c_void, e1: *const c_void,
                            e0: *const c_void) -> __m256i {
    let v0: __m128d = _mm_castsi128_pd(_mm_loadl_epi64(e0 as *const __m128i));
    let e1_val: f64 = ptr::read_unaligned(e1 as *const f64);
    let v01: __m128d = _mm_loadh_pd(v0, &e1_val);
    let v2: __m128d = _mm_castsi128_pd(_mm_loadl_epi64(e2 as *const __m128i));
    let e3_val: f64 = ptr::read_unaligned(e3 as *const f64);
    let v23: __m128d = _mm_loadh_pd(v2, &e3_val);
    // Note this can be replaced with
    // `_mm256_castpd_si256(_mm256_set_m128d(v23, v01))` if immintrin.h contains
    // _mm256_set_m128d() with all supported compilers. This version is used to
    // match the behavior with yy_set_m128i().
    yy_set_m128i(_mm_castpd_si128(v23), _mm_castpd_si128(v01))
}

#[inline]
#[target_feature(enable = "avx")]
pub unsafe fn yy_loadu2_128(hi: *const c_void, lo: *const c_void) -> __m256i {
    let mhi = _mm_loadu_si128(hi as *const __m128i);
    let mlo = _mm_loadu_si128(lo as *const __m128i);
    yy_set_m128i(mhi, mlo)
}

#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn yy_storeu2_128(hi: *mut c_void, lo: *mut c_void, a: __m256i) {
    _mm_storeu_si128(hi as *mut __m128i, _mm256_extracti128_si256::<1>(a));
    _mm_storeu_si128(lo as *mut __m128i, _mm256_castsi256_si128(a));
}

#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn yy_roundn_epu16(v_val_w: __m256i, bits: i32) -> __m256i {
    // _mm256_srli_epi16(v_val_w, bits - 1) with a run-time count.
    let v_s_w = _mm256_srl_epi16(v_val_w, _mm_cvtsi32_si128(bits - 1));
    _mm256_avg_epu16(v_s_w, _mm256_setzero_si256())
}
