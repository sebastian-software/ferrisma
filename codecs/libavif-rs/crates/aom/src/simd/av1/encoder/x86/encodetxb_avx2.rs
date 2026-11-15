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

// Translated from upstream/aom/av1/encoder/x86/encodetxb_avx2.c (ADR 0056, ADR 0058).
// Upstream compiles it with -mavx2. The functions were
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

// From av1/common/enums.h.
const TX_PAD_HOR: i32 = 4;
const TX_PAD_BOTTOM: i32 = 4;

// SSE2
// SSE4.1
// AVX2
#[no_mangle]
#[target_feature(enable = "avx2")]
pub unsafe extern "C-unwind" fn av1_txb_init_levels_avx2(
    coeff: *const i32,
    width: i32,
    height: i32,
    levels: *mut u8,
) {
    let stride: i32 = height + TX_PAD_HOR;
    let y_zeros: __m256i = _mm256_setzero_si256();
    let bottom_len: i32 = (core::mem::size_of::<u8>() * (TX_PAD_BOTTOM * stride) as usize) as i32;
    let mut bottom_buf_end: *mut u8 = levels.offset(((width + TX_PAD_BOTTOM) * stride) as isize);
    let mut bottom_buf: *mut u8 = bottom_buf_end.offset(-(((bottom_len + 31) & !31) as isize));
    loop {
        yy_storeu_256(bottom_buf as *mut core::ffi::c_void, y_zeros);
        bottom_buf = bottom_buf.offset(32);
        if !(bottom_buf < bottom_buf_end) {
            break;
        }
    }
    let mut i: i32 = 0;
    let mut ls: *mut u8 = levels;
    let mut cf: *const i32 = coeff;
    if height == 4 {
        loop {
            let c0: __m256i = yy_loadu_256(cf as *const core::ffi::c_void);
            let c1: __m256i = yy_loadu_256(cf.offset(8) as *const core::ffi::c_void);
            let abs01: __m256i = _mm256_abs_epi16(_mm256_packs_epi32(c0, c1));
            let abs01_8: __m256i = _mm256_packs_epi16(abs01, y_zeros);
            let res_: __m256i = _mm256_shuffle_epi32::<0xd8>(abs01_8);
            let res: __m256i = _mm256_permute4x64_epi64::<0xd8>(res_);
            yy_storeu_256(ls as *mut core::ffi::c_void, res);
            ls = ls.offset(32);
            cf = cf.offset(16);
            i += 4;
            if !(i < width) {
                break;
            }
        }
    } else if height == 8 {
        loop {
            let coeffA: __m256i = yy_loadu_256(cf as *const core::ffi::c_void);
            let coeffB: __m256i = yy_loadu_256(cf.offset(8) as *const core::ffi::c_void);
            let coeffC: __m256i = yy_loadu_256(cf.offset(16) as *const core::ffi::c_void);
            let coeffD: __m256i = yy_loadu_256(cf.offset(24) as *const core::ffi::c_void);
            let coeffAB: __m256i = _mm256_packs_epi32(coeffA, coeffB);
            let coeffCD: __m256i = _mm256_packs_epi32(coeffC, coeffD);
            let absAB: __m256i = _mm256_abs_epi16(coeffAB);
            let absCD: __m256i = _mm256_abs_epi16(coeffCD);
            let absABCD: __m256i = _mm256_packs_epi16(absAB, absCD);
            let res_: __m256i = _mm256_permute4x64_epi64::<0xd8>(absABCD);
            let res: __m256i = _mm256_shuffle_epi32::<0xd8>(res_);
            let res0: __m128i = _mm256_castsi256_si128(res);
            let res1: __m128i = _mm256_extracti128_si256::<1>(res);
            xx_storel_64(ls as *mut core::ffi::c_void, res0);
            core::ptr::write_unaligned(ls.offset(height as isize) as *mut i32, 0);
            xx_storel_64(
                ls.offset(stride as isize) as *mut core::ffi::c_void,
                _mm_srli_si128::<8>(res0),
            );
            core::ptr::write_unaligned(
                ls.offset(height as isize).offset(stride as isize) as *mut i32,
                0,
            );
            xx_storel_64(
                ls.offset((stride * 2) as isize) as *mut core::ffi::c_void,
                res1,
            );
            core::ptr::write_unaligned(
                ls.offset(height as isize).offset((stride * 2) as isize) as *mut i32,
                0,
            );
            xx_storel_64(
                ls.offset((stride * 3) as isize) as *mut core::ffi::c_void,
                _mm_srli_si128::<8>(res1),
            );
            core::ptr::write_unaligned(
                ls.offset(height as isize).offset((stride * 3) as isize) as *mut i32,
                0,
            );
            cf = cf.offset(32);
            ls = ls.offset((stride << 2) as isize);
            i += 4;
            if !(i < width) {
                break;
            }
        }
    } else if height == 16 {
        loop {
            let coeffA: __m256i = yy_loadu_256(cf as *const core::ffi::c_void);
            let coeffB: __m256i = yy_loadu_256(cf.offset(8) as *const core::ffi::c_void);
            let coeffC: __m256i = yy_loadu_256(cf.offset(16) as *const core::ffi::c_void);
            let coeffD: __m256i = yy_loadu_256(cf.offset(24) as *const core::ffi::c_void);
            let coeffAB: __m256i = _mm256_packs_epi32(coeffA, coeffB);
            let coeffCD: __m256i = _mm256_packs_epi32(coeffC, coeffD);
            let absAB: __m256i = _mm256_abs_epi16(coeffAB);
            let absCD: __m256i = _mm256_abs_epi16(coeffCD);
            let absABCD: __m256i = _mm256_packs_epi16(absAB, absCD);
            let res_: __m256i = _mm256_permute4x64_epi64::<0xd8>(absABCD);
            let res: __m256i = _mm256_shuffle_epi32::<0xd8>(res_);
            xx_storeu_128(ls as *mut core::ffi::c_void, _mm256_castsi256_si128(res));
            xx_storeu_128(
                ls.offset(stride as isize) as *mut core::ffi::c_void,
                _mm256_extracti128_si256::<1>(res),
            );
            cf = cf.offset(32);
            core::ptr::write_unaligned(ls.offset(height as isize) as *mut i32, 0);
            core::ptr::write_unaligned(
                ls.offset(stride as isize).offset(height as isize) as *mut i32,
                0,
            );
            ls = ls.offset((stride << 1) as isize);
            i += 2;
            if !(i < width) {
                break;
            }
        }
    } else {
        loop {
            let coeffA: __m256i = yy_loadu_256(cf as *const core::ffi::c_void);
            let coeffB: __m256i = yy_loadu_256(cf.offset(8) as *const core::ffi::c_void);
            let coeffC: __m256i = yy_loadu_256(cf.offset(16) as *const core::ffi::c_void);
            let coeffD: __m256i = yy_loadu_256(cf.offset(24) as *const core::ffi::c_void);
            let coeffAB: __m256i = _mm256_packs_epi32(coeffA, coeffB);
            let coeffCD: __m256i = _mm256_packs_epi32(coeffC, coeffD);
            let absAB: __m256i = _mm256_abs_epi16(coeffAB);
            let absCD: __m256i = _mm256_abs_epi16(coeffCD);
            let absABCD: __m256i = _mm256_packs_epi16(absAB, absCD);
            let res_: __m256i = _mm256_permute4x64_epi64::<0xd8>(absABCD);
            let res: __m256i = _mm256_shuffle_epi32::<0xd8>(res_);
            yy_storeu_256(ls as *mut core::ffi::c_void, res);
            cf = cf.offset(32);
            core::ptr::write_unaligned(ls.offset(height as isize) as *mut i32, 0);
            ls = ls.offset(stride as isize);
            i += 1;
            if !(i < width) {
                break;
            }
        }
    }
}
