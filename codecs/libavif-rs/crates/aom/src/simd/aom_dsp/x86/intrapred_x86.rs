/*
 * Copyright (c) 2020, Alliance for Open Media. All rights reserved.
 *
 * This source code is subject to the terms of the BSD 2 Clause License and
 * the Alliance for Open Media Patent License 1.0. If the BSD 2 Clause License
 * was not distributed with this source code in the LICENSE file, you can
 * obtain it at www.aomedia.org/license/software. If the Alliance for Open
 * Media Patent License 1.0 was not distributed with this source code in the
 * PATENTS file, you can obtain it at www.aomedia.org/license/patent.
 */

// Translated from upstream/aom/aom_dsp/x86/intrapred_x86.h (ADR 0056, ADR 0058).
// The inline functions were
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

// SSE2
#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn dc_sum_16_sse2(ref_: *const u8) -> __m128i {
    let mut x: __m128i = _mm_load_si128(ref_ as *const __m128i);
    let zero: __m128i = _mm_setzero_si128();
    x = _mm_sad_epu8(x, zero);
    let high: __m128i = _mm_unpackhi_epi64(x, x);
    return _mm_add_epi16(x, high);
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn dc_sum_32_sse2(ref_: *const u8) -> __m128i {
    let mut x0: __m128i = _mm_load_si128(ref_ as *const __m128i);
    let mut x1: __m128i = _mm_load_si128(ref_.offset(16) as *const __m128i);
    let zero: __m128i = _mm_setzero_si128();
    x0 = _mm_sad_epu8(x0, zero);
    x1 = _mm_sad_epu8(x1, zero);
    x0 = _mm_add_epi16(x0, x1);
    let high: __m128i = _mm_unpackhi_epi64(x0, x0);
    return _mm_add_epi16(x0, high);
}

// AOM_AOM_DSP_X86_INTRAPRED_X86_H_
