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

// Translated from upstream/aom/aom_dsp/x86/highbd_variance_avx2.c (ADR 0056, ADR 0058).
// Upstream compiles it with -mavx2. The functions were
// transliterated with a typing-aware C-subset printer (not C2Rust) and
// reviewed against the C function by function: the same sequence of intrinsic
// and helper calls, constants and operators in order (ADR 0058).
// Only the still-image functions of this file (aom_mse_wxh_16bit*, aom_mse_16xh_16bit*) and the
// static helpers they call are translated; the inter-prediction variance kernels are not
// (working decision O2, 2026-11-13, delegated, revisable by the owner).
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

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn mse_4xh_16bit_highbd_avx2(
    dst: *mut u16,
    dstride: i32,
    src: *mut u16,
    sstride: i32,
    h: i32,
) -> u64 {
    let mut sum: u64 = 0;
    let mut reg0_4x16: __m128i;
    let mut reg1_4x16: __m128i;
    let mut reg2_4x16: __m128i;
    let mut reg3_4x16: __m128i;
    let mut src0_8x16: __m256i;
    let mut src1_8x16: __m256i;
    let mut src_16x16: __m256i;
    let mut dst0_8x16: __m256i;
    let mut dst1_8x16: __m256i;
    let mut dst_16x16: __m256i;
    let mut res0_4x64: __m256i;
    let mut res1_4x64: __m256i;
    let mut res2_4x64: __m256i;
    let mut res3_4x64: __m256i;
    let mut sub_result: __m256i;
    let zeros: __m256i = _mm256_broadcastsi128_si256(_mm_setzero_si128());
    let mut square_result: __m256i = _mm256_broadcastsi128_si256(_mm_setzero_si128());
    for i in (0..h).step_by(4) {
        reg0_4x16 = _mm_loadl_epi64(dst.offset(((i + 0) * dstride) as isize) as *const __m128i);
        reg1_4x16 = _mm_loadl_epi64(dst.offset(((i + 1) * dstride) as isize) as *const __m128i);
        reg2_4x16 = _mm_loadl_epi64(dst.offset(((i + 2) * dstride) as isize) as *const __m128i);
        reg3_4x16 = _mm_loadl_epi64(dst.offset(((i + 3) * dstride) as isize) as *const __m128i);
        dst0_8x16 = _mm256_castsi128_si256(_mm_unpacklo_epi64(reg0_4x16, reg1_4x16));
        dst1_8x16 = _mm256_castsi128_si256(_mm_unpacklo_epi64(reg2_4x16, reg3_4x16));
        dst_16x16 = _mm256_permute2x128_si256::<0x20>(dst0_8x16, dst1_8x16);
        reg0_4x16 = _mm_loadl_epi64(src.offset(((i + 0) * sstride) as isize) as *const __m128i);
        reg1_4x16 = _mm_loadl_epi64(src.offset(((i + 1) * sstride) as isize) as *const __m128i);
        reg2_4x16 = _mm_loadl_epi64(src.offset(((i + 2) * sstride) as isize) as *const __m128i);
        reg3_4x16 = _mm_loadl_epi64(src.offset(((i + 3) * sstride) as isize) as *const __m128i);
        src0_8x16 = _mm256_castsi128_si256(_mm_unpacklo_epi64(reg0_4x16, reg1_4x16));
        src1_8x16 = _mm256_castsi128_si256(_mm_unpacklo_epi64(reg2_4x16, reg3_4x16));
        src_16x16 = _mm256_permute2x128_si256::<0x20>(src0_8x16, src1_8x16);
        sub_result = _mm256_abs_epi16(_mm256_sub_epi16(src_16x16, dst_16x16));
        src_16x16 = _mm256_unpacklo_epi16(sub_result, zeros);
        dst_16x16 = _mm256_unpackhi_epi16(sub_result, zeros);
        src_16x16 = _mm256_madd_epi16(src_16x16, src_16x16);
        dst_16x16 = _mm256_madd_epi16(dst_16x16, dst_16x16);
        res0_4x64 = _mm256_unpacklo_epi32(src_16x16, zeros);
        res1_4x64 = _mm256_unpackhi_epi32(src_16x16, zeros);
        res2_4x64 = _mm256_unpacklo_epi32(dst_16x16, zeros);
        res3_4x64 = _mm256_unpackhi_epi32(dst_16x16, zeros);
        square_result = _mm256_add_epi64(
            square_result,
            _mm256_add_epi64(
                _mm256_add_epi64(_mm256_add_epi64(res0_4x64, res1_4x64), res2_4x64),
                res3_4x64,
            ),
        );
    }
    let sum_2x64: __m128i = _mm_add_epi64(
        _mm256_castsi256_si128(square_result),
        _mm256_extracti128_si256::<1>(square_result),
    );
    let sum_1x64: __m128i = _mm_add_epi64(sum_2x64, _mm_srli_si128::<8>(sum_2x64));
    xx_storel_64(&raw mut sum as *mut core::ffi::c_void, sum_1x64);
    return sum;
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn mse_8xh_16bit_highbd_avx2(
    dst: *mut u16,
    dstride: i32,
    src: *mut u16,
    sstride: i32,
    h: i32,
) -> u64 {
    let mut sum: u64 = 0;
    let mut src0_8x16: __m256i;
    let mut src1_8x16: __m256i;
    let mut src_16x16: __m256i;
    let mut dst0_8x16: __m256i;
    let mut dst1_8x16: __m256i;
    let mut dst_16x16: __m256i;
    let mut res0_4x64: __m256i;
    let mut res1_4x64: __m256i;
    let mut res2_4x64: __m256i;
    let mut res3_4x64: __m256i;
    let mut sub_result: __m256i;
    let zeros: __m256i = _mm256_broadcastsi128_si256(_mm_setzero_si128());
    let mut square_result: __m256i = _mm256_broadcastsi128_si256(_mm_setzero_si128());
    for i in (0..h).step_by(2) {
        dst0_8x16 = _mm256_castsi128_si256(_mm_loadu_si128(dst.offset((i * dstride) as isize)
            as *mut __m128i
            as *const __m128i));
        dst1_8x16 =
            _mm256_castsi128_si256(_mm_loadu_si128(dst.offset(((i + 1) * dstride) as isize)
                as *mut __m128i
                as *const __m128i));
        dst_16x16 = _mm256_permute2x128_si256::<0x20>(dst0_8x16, dst1_8x16);
        src0_8x16 = _mm256_castsi128_si256(_mm_loadu_si128(src.offset((i * sstride) as isize)
            as *mut __m128i
            as *const __m128i));
        src1_8x16 =
            _mm256_castsi128_si256(_mm_loadu_si128(src.offset(((i + 1) * sstride) as isize)
                as *mut __m128i
                as *const __m128i));
        src_16x16 = _mm256_permute2x128_si256::<0x20>(src0_8x16, src1_8x16);
        sub_result = _mm256_abs_epi16(_mm256_sub_epi16(src_16x16, dst_16x16));
        src_16x16 = _mm256_unpacklo_epi16(sub_result, zeros);
        dst_16x16 = _mm256_unpackhi_epi16(sub_result, zeros);
        src_16x16 = _mm256_madd_epi16(src_16x16, src_16x16);
        dst_16x16 = _mm256_madd_epi16(dst_16x16, dst_16x16);
        res0_4x64 = _mm256_unpacklo_epi32(src_16x16, zeros);
        res1_4x64 = _mm256_unpackhi_epi32(src_16x16, zeros);
        res2_4x64 = _mm256_unpacklo_epi32(dst_16x16, zeros);
        res3_4x64 = _mm256_unpackhi_epi32(dst_16x16, zeros);
        square_result = _mm256_add_epi64(
            square_result,
            _mm256_add_epi64(
                _mm256_add_epi64(_mm256_add_epi64(res0_4x64, res1_4x64), res2_4x64),
                res3_4x64,
            ),
        );
    }
    let sum_2x64: __m128i = _mm_add_epi64(
        _mm256_castsi256_si128(square_result),
        _mm256_extracti128_si256::<1>(square_result),
    );
    let sum_1x64: __m128i = _mm_add_epi64(sum_2x64, _mm_srli_si128::<8>(sum_2x64));
    xx_storel_64(&raw mut sum as *mut core::ffi::c_void, sum_1x64);
    return sum;
}

#[no_mangle]
#[target_feature(enable = "avx2")]
pub unsafe extern "C-unwind" fn aom_mse_wxh_16bit_highbd_avx2(
    dst: *mut u16,
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
            return mse_4xh_16bit_highbd_avx2(dst, dstride, src, sstride, h);
        }
        8 => {
            return mse_8xh_16bit_highbd_avx2(dst, dstride, src, sstride, h);
        }
        _ => {
            debug_assert!(0 != 0, "unsupported width");
            return (-1_i32) as u64;
        }
    }
}
