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

// Translated from upstream/aom/aom_dsp/x86/bitdepth_conversion_avx2.h (ADR 0056, ADR 0058).
// The functions were
// transliterated with a typing-aware C-subset printer (not C2Rust) and
// reviewed against the C function by function: the same sequence of intrinsic
// and helper calls, constants and operators in order (ADR 0058).
//
// Translation notes:
// - Every function carries `avx2`, the flag upstream compiles the file with.
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

#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn load_tran_low(a: *const i32) -> __m256i {
    let a_low: __m256i = _mm256_loadu_si256(a as *const __m256i);
    let a_high: __m256i = _mm256_loadu_si256(a.offset(8) as *const __m256i);
    return _mm256_packs_epi32(a_low, a_high);
}

#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn store_tran_low(a: __m256i, b: *mut i32) {
    let one: __m256i = _mm256_set1_epi16(1);
    let a_hi: __m256i = _mm256_mulhi_epi16(a, one);
    let a_lo: __m256i = _mm256_mullo_epi16(a, one);
    let a_1: __m256i = _mm256_unpacklo_epi16(a_lo, a_hi);
    let a_2: __m256i = _mm256_unpackhi_epi16(a_lo, a_hi);
    _mm256_storeu_si256(b as *mut __m256i, a_1);
    _mm256_storeu_si256(b.offset(8) as *mut __m256i, a_2);
}
