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

// Translated from upstream/aom/aom_dsp/x86/variance_sse2.c (ADR 0056, ADR 0058).
// Upstream compiles it with -msse2. The functions were
// transliterated with a typing-aware C-subset printer (not C2Rust) and
// reviewed against the C function by function: the same sequence of intrinsic
// and helper calls, constants and operators in order (ADR 0058).
// Only the still-image functions of this file (aom_mse_wxh_16bit*, aom_mse_16xh_16bit*) and the
// static helpers they call are translated; the inter-prediction variance kernels are not
// (working decision O2, 2026-11-13, delegated, revisable by the owner).
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

use crate::simd::aom_dsp__x86__synonyms::*;
use core::arch::x86_64::*;

// CONFIG_AV1_HIGHBITDEPTH
#[inline]
#[target_feature(enable = "sse2")]
unsafe fn mse_4xh_16bit_sse2(
    dst: *mut u8,
    dstride: i32,
    src: *mut u16,
    sstride: i32,
    h: i32,
) -> u64 {
    let mut sum: u64 = 0;
    let mut dst0_8x8: __m128i;
    let mut dst1_8x8: __m128i;
    let mut dst_16x8: __m128i;
    let mut src0_16x4: __m128i;
    let mut src1_16x4: __m128i;
    let mut src_16x8: __m128i;
    let mut res0_32x4: __m128i;
    let mut res0_64x2: __m128i;
    let mut res1_64x2: __m128i;
    let mut sub_result_16x8: __m128i;
    let zeros: __m128i = _mm_setzero_si128();
    let mut square_result: __m128i = _mm_setzero_si128();
    for i in (0..h).step_by(2) {
        dst0_8x8 = _mm_cvtsi32_si128(core::ptr::read_unaligned(
            dst.offset(((i + 0) * dstride) as isize) as *const i32,
        ));
        dst1_8x8 = _mm_cvtsi32_si128(core::ptr::read_unaligned(
            dst.offset(((i + 1) * dstride) as isize) as *const i32,
        ));
        dst_16x8 = _mm_unpacklo_epi8(_mm_unpacklo_epi32(dst0_8x8, dst1_8x8), zeros);
        src0_16x4 = _mm_loadl_epi64(src.offset(((i + 0) * sstride) as isize) as *const __m128i);
        src1_16x4 = _mm_loadl_epi64(src.offset(((i + 1) * sstride) as isize) as *const __m128i);
        src_16x8 = _mm_unpacklo_epi64(src0_16x4, src1_16x4);
        sub_result_16x8 = _mm_sub_epi16(src_16x8, dst_16x8);
        res0_32x4 = _mm_madd_epi16(sub_result_16x8, sub_result_16x8);
        res0_64x2 = _mm_unpacklo_epi32(res0_32x4, zeros);
        res1_64x2 = _mm_unpackhi_epi32(res0_32x4, zeros);
        square_result = _mm_add_epi64(square_result, _mm_add_epi64(res0_64x2, res1_64x2));
    }
    let sum_64x1: __m128i = _mm_add_epi64(square_result, _mm_srli_si128::<8>(square_result));
    xx_storel_64(&raw mut sum as *mut core::ffi::c_void, sum_64x1);
    return sum;
}

#[inline]
#[target_feature(enable = "sse2")]
unsafe fn mse_8xh_16bit_sse2(
    dst: *mut u8,
    dstride: i32,
    src: *mut u16,
    sstride: i32,
    h: i32,
) -> u64 {
    let mut sum: u64 = 0;
    let mut dst_8x8: __m128i;
    let mut dst_16x8: __m128i;
    let mut src_16x8: __m128i;
    let mut res0_32x4: __m128i;
    let mut res0_64x2: __m128i;
    let mut res1_64x2: __m128i;
    let mut sub_result_16x8: __m128i;
    let zeros: __m128i = _mm_setzero_si128();
    let mut square_result: __m128i = _mm_setzero_si128();
    for i in 0..h {
        dst_8x8 = _mm_loadl_epi64(dst.offset(((i + 0) * dstride) as isize) as *const __m128i);
        dst_16x8 = _mm_unpacklo_epi8(dst_8x8, zeros);
        src_16x8 =
            _mm_loadu_si128(src.offset((i * sstride) as isize) as *mut __m128i as *const __m128i);
        sub_result_16x8 = _mm_sub_epi16(src_16x8, dst_16x8);
        res0_32x4 = _mm_madd_epi16(sub_result_16x8, sub_result_16x8);
        res0_64x2 = _mm_unpacklo_epi32(res0_32x4, zeros);
        res1_64x2 = _mm_unpackhi_epi32(res0_32x4, zeros);
        square_result = _mm_add_epi64(square_result, _mm_add_epi64(res0_64x2, res1_64x2));
    }
    let sum_64x1: __m128i = _mm_add_epi64(square_result, _mm_srli_si128::<8>(square_result));
    xx_storel_64(&raw mut sum as *mut core::ffi::c_void, sum_64x1);
    return sum;
}

#[no_mangle]
#[target_feature(enable = "sse2")]
pub unsafe extern "C-unwind" fn aom_mse_wxh_16bit_sse2(
    dst: *mut u8,
    dstride: i32,
    src: *mut u16,
    sstride: i32,
    w: i32,
    h: i32,
) -> u64 {
    debug_assert!(
        (w == 8 || w == 4) && (h == 8 || h == 4),
        "w=8/4 and h=8/4 must satisfy"
    );
    match w {
        4 => {
            return mse_4xh_16bit_sse2(dst, dstride, src, sstride, h);
        }
        8 => {
            return mse_8xh_16bit_sse2(dst, dstride, src, sstride, h);
        }
        _ => {
            debug_assert!(0 != 0, "unsupported width");
            return (-1_i32) as u64;
        }
    }
}

#[no_mangle]
#[target_feature(enable = "sse2")]
pub unsafe extern "C-unwind" fn aom_mse_16xh_16bit_sse2(
    mut dst: *mut u8,
    dstride: i32,
    mut src: *mut u16,
    w: i32,
    h: i32,
) -> u64 {
    debug_assert!(
        (w == 8 || w == 4) && (h == 8 || h == 4),
        "w=8/4 and h=8/4 must be satisfied"
    );
    let num_blks: i32 = 16 / w;
    let mut sum: u64 = 0;
    for i in 0..num_blks {
        sum += aom_mse_wxh_16bit_sse2(dst, dstride, src, w, w, h);
        dst = dst.offset(w as isize);
        src = src.offset((w * h) as isize);
    }
    return sum;
}
