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

// Translated from upstream/aom/aom_dsp/x86/convolve_avx2.h (ADR 0056, ADR 0058).
// Only the tables and inline functions the Wiener filter of
// av1/common/x86/wiener_convolve_avx2.c uses are translated (convolve, convolve_lowbd_x and
// the filter tables); the inter-prediction convolutions are not (working decision O2,
// 2026-11-13, delegated, revisable by the owner). They were
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

use crate::simd::aom_dsp__x86__synonyms::*;
use crate::simd::aom_dsp__x86__synonyms_avx2::*;
use core::arch::x86_64::*;

#[repr(C, align(32))]
pub struct Aligned32<T>(pub T);

const SECOND_32_BLK: i32 = (32);
const THIRD_32_BLK: i32 = (32 << 1);
const FOURTH_32_BLK: i32 = (SECOND_32_BLK + THIRD_32_BLK);

// filters for 16
pub static filt_center_global_avx2: Aligned32<[u8; 32]> = Aligned32([
    3, 255, 4, 255, 5, 255, 6, 255, 7, 255, 8, 255, 9, 255, 10, 255, 3, 255, 4, 255, 5, 255, 6,
    255, 7, 255, 8, 255, 9, 255, 10, 255,
]);

pub static filt1_global_avx2: Aligned32<[u8; 32]> = Aligned32([
    0, 1, 1, 2, 2, 3, 3, 4, 4, 5, 5, 6, 6, 7, 7, 8, 0, 1, 1, 2, 2, 3, 3, 4, 4, 5, 5, 6, 6, 7, 7, 8,
]);

pub static filt2_global_avx2: Aligned32<[u8; 32]> = Aligned32([
    2, 3, 3, 4, 4, 5, 5, 6, 6, 7, 7, 8, 8, 9, 9, 10, 2, 3, 3, 4, 4, 5, 5, 6, 6, 7, 7, 8, 8, 9, 9,
    10,
]);

pub static filt3_global_avx2: Aligned32<[u8; 32]> = Aligned32([
    4, 5, 5, 6, 6, 7, 7, 8, 8, 9, 9, 10, 10, 11, 11, 12, 4, 5, 5, 6, 6, 7, 7, 8, 8, 9, 9, 10, 10,
    11, 11, 12,
]);

pub static filt4_global_avx2: Aligned32<[u8; 32]> = Aligned32([
    6, 7, 7, 8, 8, 9, 9, 10, 10, 11, 11, 12, 12, 13, 13, 14, 6, 7, 7, 8, 8, 9, 9, 10, 10, 11, 11,
    12, 12, 13, 13, 14,
]);

#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn convolve_lowbd(s: *const __m256i, coeffs: *const __m256i) -> __m256i {
    let res_01: __m256i = _mm256_maddubs_epi16(*s, *coeffs);
    let res_23: __m256i = _mm256_maddubs_epi16(*s.offset(1), *coeffs.offset(1));
    let res_45: __m256i = _mm256_maddubs_epi16(*s.offset(2), *coeffs.offset(2));
    let res_67: __m256i = _mm256_maddubs_epi16(*s.offset(3), *coeffs.offset(3));
    let res: __m256i = _mm256_add_epi16(
        _mm256_add_epi16(res_01, res_45),
        _mm256_add_epi16(res_23, res_67),
    );
    return res;
}

#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn convolve(s: *const __m256i, coeffs: *const __m256i) -> __m256i {
    let res_0: __m256i = _mm256_madd_epi16(*s, *coeffs);
    let res_1: __m256i = _mm256_madd_epi16(*s.offset(1), *coeffs.offset(1));
    let res_2: __m256i = _mm256_madd_epi16(*s.offset(2), *coeffs.offset(2));
    let res_3: __m256i = _mm256_madd_epi16(*s.offset(3), *coeffs.offset(3));
    let res: __m256i = _mm256_add_epi32(
        _mm256_add_epi32(res_0, res_1),
        _mm256_add_epi32(res_2, res_3),
    );
    return res;
}

#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn convolve_lowbd_x(
    data: __m256i,
    coeffs: *const __m256i,
    filt: *const __m256i,
) -> __m256i {
    let mut s: [__m256i; 4] = core::mem::zeroed();
    s[0] = _mm256_shuffle_epi8(data, *filt);
    s[1] = _mm256_shuffle_epi8(data, *filt.offset(1));
    s[2] = _mm256_shuffle_epi8(data, *filt.offset(2));
    s[3] = _mm256_shuffle_epi8(data, *filt.offset(3));
    return convolve_lowbd(s.as_mut_ptr() as *const __m256i, coeffs);
}

// AOM_AOM_DSP_X86_CONVOLVE_AVX2_H_
