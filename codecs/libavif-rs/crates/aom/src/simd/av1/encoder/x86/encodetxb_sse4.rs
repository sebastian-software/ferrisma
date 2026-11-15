/*
 * Copyright (c) 2017, Alliance for Open Media. All rights reserved.
 *
 * This source code is subject to the terms of the BSD 2 Clause License and
 * the Alliance for Open Media Patent License 1.0. If the BSD 2 Clause License
 * was not distributed with this source code in the LICENSE file, you can
 * obtain it at www.aomedia.org/license/software. If the Alliance for Open
 * Media Patent License 1.0 was not distributed with this source code in the
 * PATENTS file, you can obtain it at www.aomedia.org/license/patent.
 */

// Translated from upstream/aom/av1/encoder/x86/encodetxb_sse4.c (ADR 0056, ADR 0058).
// Upstream compiles it with -msse41. The functions were
// transliterated with a typing-aware C-subset printer (not C2Rust) and
// reviewed against the C function by function: the same sequence of intrinsic
// and helper calls, constants and operators in order (ADR 0058).
//
// Translation notes:
// - Every function carries `sse4.1`, the flag upstream compiles the file with.
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
use core::arch::x86_64::*;

// From av1/common/enums.h.
const TX_PAD_HOR: i32 = 4;
const TX_PAD_BOTTOM: i32 = 4;

// SSE2
// SSE4.1
#[no_mangle]
#[target_feature(enable = "sse4.1")]
pub unsafe extern "C-unwind" fn av1_txb_init_levels_sse4_1(
    coeff: *const i32,
    width: i32,
    height: i32,
    levels: *mut u8,
) {
    let stride: i32 = height + TX_PAD_HOR;
    let zeros: __m128i = _mm_setzero_si128();
    let bottom_len: i32 = (core::mem::size_of::<u8>() * (TX_PAD_BOTTOM * stride) as usize) as i32;
    let mut bottom_buf: *mut u8 = levels.offset((stride * width) as isize);
    let mut bottom_buf_end: *mut u8 = bottom_buf.offset(bottom_len as isize);
    loop {
        _mm_storeu_si128(bottom_buf as *mut __m128i, zeros);
        bottom_buf = bottom_buf.offset(16);
        if !(bottom_buf < bottom_buf_end) {
            break;
        }
    }
    let mut i: i32 = 0;
    let mut ls: *mut u8 = levels;
    let mut cf: *const i32 = coeff;
    if height == 4 {
        loop {
            let coeffA: __m128i = xx_loadu_128(cf as *const core::ffi::c_void);
            let coeffB: __m128i = xx_loadu_128(cf.offset(4) as *const core::ffi::c_void);
            let coeffAB: __m128i = _mm_packs_epi32(coeffA, coeffB);
            let absAB: __m128i = _mm_abs_epi16(coeffAB);
            let absAB8: __m128i = _mm_packs_epi16(absAB, zeros);
            let lsAB: __m128i = _mm_unpacklo_epi32(absAB8, zeros);
            xx_storeu_128(ls as *mut core::ffi::c_void, lsAB);
            ls = ls.offset((stride << 1) as isize);
            cf = cf.offset((height << 1) as isize);
            i += 2;
            if !(i < width) {
                break;
            }
        }
    } else if height == 8 {
        loop {
            let coeffA: __m128i = xx_loadu_128(cf as *const core::ffi::c_void);
            let coeffB: __m128i = xx_loadu_128(cf.offset(4) as *const core::ffi::c_void);
            let coeffAB: __m128i = _mm_packs_epi32(coeffA, coeffB);
            let absAB: __m128i = _mm_abs_epi16(coeffAB);
            let absAB8: __m128i = _mm_packs_epi16(absAB, zeros);
            xx_storeu_128(ls as *mut core::ffi::c_void, absAB8);
            ls = ls.offset(stride as isize);
            cf = cf.offset(height as isize);
            i += 1;
            if !(i < width) {
                break;
            }
        }
    } else {
        loop {
            let mut j: i32 = 0;
            loop {
                let coeffA: __m128i = xx_loadu_128(cf as *const core::ffi::c_void);
                let coeffB: __m128i = xx_loadu_128(cf.offset(4) as *const core::ffi::c_void);
                let coeffC: __m128i = xx_loadu_128(cf.offset(8) as *const core::ffi::c_void);
                let coeffD: __m128i = xx_loadu_128(cf.offset(12) as *const core::ffi::c_void);
                let coeffAB: __m128i = _mm_packs_epi32(coeffA, coeffB);
                let coeffCD: __m128i = _mm_packs_epi32(coeffC, coeffD);
                let absAB: __m128i = _mm_abs_epi16(coeffAB);
                let absCD: __m128i = _mm_abs_epi16(coeffCD);
                let absABCD: __m128i = _mm_packs_epi16(absAB, absCD);
                xx_storeu_128(ls.offset(j as isize) as *mut core::ffi::c_void, absABCD);
                j += 16;
                cf = cf.offset(16);
                if !(j < height) {
                    break;
                }
            }
            core::ptr::write_unaligned(ls.offset(height as isize) as *mut i32, 0);
            ls = ls.offset(stride as isize);
            i += 1;
            if !(i < width) {
                break;
            }
        }
    }
}
