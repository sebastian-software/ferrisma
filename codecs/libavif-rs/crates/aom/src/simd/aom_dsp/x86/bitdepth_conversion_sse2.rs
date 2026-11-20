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

// Translated from upstream/aom/aom_dsp/x86/bitdepth_conversion_sse2.h (ADR 0056, ADR 0058).
// The functions were
// transliterated with a typing-aware C-subset printer (not C2Rust) and
// reviewed against the C function by function: the same sequence of intrinsic
// and helper calls, constants and operators in order (ADR 0058).
//
// Translation notes:
// - Every function carries `sse2`, the flag upstream compiles the file with.
//   The RTCD specializations are exported under their upstream names.
// - Pointer and array parameters stay raw pointers; local arrays are Rust
//   arrays and start zeroed. DECLARE_ALIGNED arrays keep their alignment
//   through `Aligned16`/`Aligned32`.
// - C integer conversions are explicit (`as`); `assert` becomes
//   `debug_assert!`. Immediate operands become const generics; shift counts
//   that are run-time `int` values use the count-register intrinsics, which is
//   what GCC and Clang emit for a non-constant count.
// - Scalar accesses through cast pointers (`*(int *)p`) are unaligned reads
//   and writes, as x86 compilers emit them.
#![allow(
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut,
    unused_parens
)]
#![allow(unused_variables, dead_code, clippy::all)]

use core::arch::x86_64::*;

// Load 8 16 bit values. If the source is 32 bits then pack down with
// saturation.
#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn load_tran_low(a: *const i32) -> __m128i {
    let a_low: __m128i = _mm_load_si128(a as *const __m128i);
    return _mm_packs_epi32(a_low, *(a.offset(4) as *const __m128i));
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn unpack_trans(a: __m128i, a_1: *mut __m128i, a_2: *mut __m128i) {
    let one: __m128i = _mm_set1_epi16(1);
    let a_hi: __m128i = _mm_mulhi_epi16(a, one);
    let a_lo: __m128i = _mm_mullo_epi16(a, one);
    *a_1 = _mm_unpacklo_epi16(a_lo, a_hi);
    *a_2 = _mm_unpackhi_epi16(a_lo, a_hi);
}

// Store 8 16 bit values. If the destination is 32 bits then sign extend the
// values by multiplying by 1.
#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn store_tran_low(a: __m128i, b: *mut i32) {
    let mut a_1: __m128i = core::mem::zeroed();
    let mut a_2: __m128i = core::mem::zeroed();
    unpack_trans(a, &raw mut a_1, &raw mut a_2);
    _mm_store_si128(b as *mut __m128i, a_1);
    _mm_store_si128(b.offset(4) as *mut __m128i, a_2);
}

// Stores the second result at an offset of 8 (instead of 4) to match the output
// with that of AVX2 implementation and the function is similar to
// store_tran_low().
#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn store_tran_low_offset_4(a: __m128i, b: *mut i32) {
    let mut a_1: __m128i = core::mem::zeroed();
    let mut a_2: __m128i = core::mem::zeroed();
    unpack_trans(a, &raw mut a_1, &raw mut a_2);
    _mm_store_si128(b as *mut __m128i, a_1);
    _mm_store_si128(b.offset(8) as *mut __m128i, a_2);
}
