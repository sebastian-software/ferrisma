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

// Translated by hand from upstream/aom/aom_dsp/x86/synonyms.h (ADR 0056,
// ADR 0058). Function names, order, comments, and intrinsic sequences follow
// the original; every helper carries the smallest target feature its own
// intrinsics need (SSE2), so it inlines into SSE2, SSE4.1, and AVX2 callers.
//
// Translation notes:
// - `const void *` / `void *const` parameters become `*const c_void` /
//   `*mut c_void`.
// - memcpy-based unaligned scalar accesses use ptr::read_unaligned /
//   ptr::write_unaligned.
// - The C helpers pass their run-time `bits` argument to `_mm_srli_epi16`,
//   `_mm_srli_epi32`, `_mm_srai_epi16`, and `_mm_srai_epi32`. In C these take
//   an ordinary `int`, and GCC and Clang lower a count that is not a constant
//   to the count-register form of the same instruction (PSRLW/PSRAD xmm, xmm
//   with the count zero-extended by MOVD). core::arch offers only constant
//   counts under the `*i_*` names, so the run-time count is translated with the
//   count-register intrinsic (`_mm_srl_epi16(v, _mm_cvtsi32_si128(n))`, ...).
//   Both forms saturate counts above the lane width in the same way (zero for
//   logical shifts, sign fill for arithmetic shifts), so the result is
//   bit-identical for every `int` count, and the signatures stay those of C.

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
#[target_feature(enable = "sse2")]
pub unsafe fn xx_loadl_32(a: *const c_void) -> __m128i {
    let val: i32 = ptr::read_unaligned(a as *const i32);
    _mm_cvtsi32_si128(val)
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn xx_loadl_64(a: *const c_void) -> __m128i {
    _mm_loadl_epi64(a as *const __m128i)
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn xx_load_128(a: *const c_void) -> __m128i {
    _mm_load_si128(a as *const __m128i)
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn xx_loadu_128(a: *const c_void) -> __m128i {
    _mm_loadu_si128(a as *const __m128i)
}

// Load 64 bits from each of hi and low, and pack into an SSE register
// Since directly loading as `int64_t`s and using _mm_set_epi64 may violate
// the strict aliasing rule, this takes a different approach
#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn xx_loadu_2x64(hi: *const c_void, lo: *const c_void) -> __m128i {
    _mm_unpacklo_epi64(_mm_loadl_epi64(lo as *const __m128i),
                       _mm_loadl_epi64(hi as *const __m128i))
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn xx_storel_16(a: *mut c_void, v: __m128i) {
    let val: u16 = _mm_cvtsi128_si32(v) as u16;
    ptr::write_unaligned(a as *mut u16, val);
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn xx_storel_32(a: *mut c_void, v: __m128i) {
    let val: i32 = _mm_cvtsi128_si32(v);
    ptr::write_unaligned(a as *mut i32, val);
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn xx_storel_64(a: *mut c_void, v: __m128i) {
    _mm_storel_epi64(a as *mut __m128i, v);
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn xx_store_128(a: *mut c_void, v: __m128i) {
    _mm_store_si128(a as *mut __m128i, v);
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn xx_storeu_128(a: *mut c_void, v: __m128i) {
    _mm_storeu_si128(a as *mut __m128i, v);
}

// Fill an SSE register using an interleaved pair of values, ie. set the
// 8 channels to {a, b, a, b, a, b, a, b}, using the same channel ordering
// as when a register is stored to / loaded from memory.
//
// This is useful for rearranging filter kernels for use with the _mm_madd_epi16
// instruction
#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn xx_set2_epi16(a: i16, b: i16) -> __m128i {
    _mm_setr_epi16(a, b, a, b, a, b, a, b)
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn xx_round_epu16(v_val_w: __m128i) -> __m128i {
    _mm_avg_epu16(v_val_w, _mm_setzero_si128())
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn xx_roundn_epu16(v_val_w: __m128i, bits: i32) -> __m128i {
    // _mm_srli_epi16(v_val_w, bits - 1) with a run-time count.
    let v_s_w = _mm_srl_epi16(v_val_w, _mm_cvtsi32_si128(bits - 1));
    _mm_avg_epu16(v_s_w, _mm_setzero_si128())
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn xx_roundn_epu32(v_val_d: __m128i, bits: i32) -> __m128i {
    let v_bias_d = _mm_set1_epi32((1 << bits) >> 1);
    let v_tmp_d = _mm_add_epi32(v_val_d, v_bias_d);
    // _mm_srli_epi32(v_tmp_d, bits) with a run-time count.
    _mm_srl_epi32(v_tmp_d, _mm_cvtsi32_si128(bits))
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn xx_roundn_epi16_unsigned(v_val_d: __m128i, bits: i32) -> __m128i {
    let v_bias_d = _mm_set1_epi16(((1 << bits) >> 1) as i16);
    let v_tmp_d = _mm_add_epi16(v_val_d, v_bias_d);
    // _mm_srai_epi16(v_tmp_d, bits) with a run-time count.
    _mm_sra_epi16(v_tmp_d, _mm_cvtsi32_si128(bits))
}

// This is equivalent to ROUND_POWER_OF_TWO(v_val_d, bits)
#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn xx_roundn_epi32_unsigned(v_val_d: __m128i, bits: i32) -> __m128i {
    let v_bias_d = _mm_set1_epi32((1 << bits) >> 1);
    let v_tmp_d = _mm_add_epi32(v_val_d, v_bias_d);
    // _mm_srai_epi32(v_tmp_d, bits) with a run-time count.
    _mm_sra_epi32(v_tmp_d, _mm_cvtsi32_si128(bits))
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn xx_roundn_epi16(v_val_d: __m128i, bits: i32) -> __m128i {
    let v_bias_d = _mm_set1_epi16(((1 << bits) >> 1) as i16);
    let v_sign_d = _mm_srai_epi16::<15>(v_val_d);
    let v_tmp_d = _mm_add_epi16(_mm_add_epi16(v_val_d, v_bias_d), v_sign_d);
    // _mm_srai_epi16(v_tmp_d, bits) with a run-time count.
    _mm_sra_epi16(v_tmp_d, _mm_cvtsi32_si128(bits))
}
