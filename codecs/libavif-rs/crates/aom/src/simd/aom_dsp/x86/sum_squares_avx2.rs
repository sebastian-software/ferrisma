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

// Translated from upstream/aom/aom_dsp/x86/sum_squares_avx2.c (ADR 0056, ADR 0058).
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

use crate::aom_dsp__sum_squares::{
    aom_sum_squares_2d_i16_c, aom_sum_squares_i16_c, aom_sum_sse_2d_i16_c,
};
use crate::simd::aom_dsp__x86__sum_squares_sse2::*;
use crate::simd::aom_dsp__x86__synonyms::*;
use crate::simd::aom_dsp__x86__synonyms_avx2::*;
use core::arch::x86_64::*;

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn aom_sum_squares_2d_i16_nxn_avx2(
    mut src: *const i16,
    stride: i32,
    width: i32,
    height: i32,
) -> u64 {
    let mut result: u64 = 0;
    let mut v_acc_q: __m256i = _mm256_setzero_si256();
    let v_zext_mask_q: __m256i = _mm256_set1_epi64x(!0_u32 as i64);
    for col in (0..height).step_by(4) {
        let mut v_acc_d: __m256i = _mm256_setzero_si256();
        for row in (0..width).step_by(16) {
            let mut tempsrc: *const i16 = src.offset(row as isize);
            let v_val_0_w: __m256i =
                _mm256_loadu_si256(tempsrc.offset((0 * stride) as isize) as *const __m256i);
            let v_val_1_w: __m256i =
                _mm256_loadu_si256(tempsrc.offset((1 * stride) as isize) as *const __m256i);
            let v_val_2_w: __m256i =
                _mm256_loadu_si256(tempsrc.offset((2 * stride) as isize) as *const __m256i);
            let v_val_3_w: __m256i =
                _mm256_loadu_si256(tempsrc.offset((3 * stride) as isize) as *const __m256i);
            let v_sq_0_d: __m256i = _mm256_madd_epi16(v_val_0_w, v_val_0_w);
            let v_sq_1_d: __m256i = _mm256_madd_epi16(v_val_1_w, v_val_1_w);
            let v_sq_2_d: __m256i = _mm256_madd_epi16(v_val_2_w, v_val_2_w);
            let v_sq_3_d: __m256i = _mm256_madd_epi16(v_val_3_w, v_val_3_w);
            let v_sum_01_d: __m256i = _mm256_add_epi32(v_sq_0_d, v_sq_1_d);
            let v_sum_23_d: __m256i = _mm256_add_epi32(v_sq_2_d, v_sq_3_d);
            let v_sum_0123_d: __m256i = _mm256_add_epi32(v_sum_01_d, v_sum_23_d);
            v_acc_d = _mm256_add_epi32(v_acc_d, v_sum_0123_d);
        }
        v_acc_q = _mm256_add_epi64(v_acc_q, _mm256_and_si256(v_acc_d, v_zext_mask_q));
        v_acc_q = _mm256_add_epi64(v_acc_q, _mm256_srli_epi64::<{ 32 }>(v_acc_d));
        src = src.offset((4 * stride) as isize);
    }
    let mut lower_64_2_Value: __m128i = _mm256_castsi256_si128(v_acc_q);
    let mut higher_64_2_Value: __m128i = _mm256_extracti128_si256::<1>(v_acc_q);
    let mut result_64_2_int: __m128i = _mm_add_epi64(lower_64_2_Value, higher_64_2_Value);
    result_64_2_int = _mm_add_epi64(
        result_64_2_int,
        _mm_unpackhi_epi64(result_64_2_int, result_64_2_int),
    );
    xx_storel_64(&raw mut result as *mut core::ffi::c_void, result_64_2_int);
    return result;
}

#[no_mangle]
#[target_feature(enable = "avx2")]
pub unsafe extern "C-unwind" fn aom_sum_squares_2d_i16_avx2(
    src: *const i16,
    stride: i32,
    width: i32,
    height: i32,
) -> u64 {
    if width == 4 && height == 4 {
        return aom_sum_squares_2d_i16_4x4_sse2(src, stride);
    } else if width == 4 && height & 3 == 0 {
        return aom_sum_squares_2d_i16_4xn_sse2(src, stride, height);
    } else if width == 8 && height & 3 == 0 {
        return aom_sum_squares_2d_i16_nxn_sse2(src, stride, width, height);
    } else if width & 15 == 0 && height & 3 == 0 {
        return aom_sum_squares_2d_i16_nxn_avx2(src, stride, width, height);
    } else {
        return aom_sum_squares_2d_i16_c(src, stride, width, height);
    }
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn aom_sum_sse_2d_i16_nxn_avx2(
    mut src: *const i16,
    stride: i32,
    width: i32,
    height: i32,
    sum: *mut i32,
) -> u64 {
    let mut result: u64 = 0;
    let zero_reg: __m256i = _mm256_setzero_si256();
    let one_reg: __m256i = _mm256_set1_epi16(1);
    let mut v_sse_total: __m256i = zero_reg;
    let mut v_sum_total: __m256i = zero_reg;
    for col in (0..height).step_by(4) {
        let mut v_sse_row: __m256i = zero_reg;
        for row in (0..width).step_by(16) {
            let mut tempsrc: *const i16 = src.offset(row as isize);
            let v_val_0_w: __m256i =
                _mm256_loadu_si256(tempsrc.offset((0 * stride) as isize) as *const __m256i);
            let v_val_1_w: __m256i =
                _mm256_loadu_si256(tempsrc.offset((1 * stride) as isize) as *const __m256i);
            let v_val_2_w: __m256i =
                _mm256_loadu_si256(tempsrc.offset((2 * stride) as isize) as *const __m256i);
            let v_val_3_w: __m256i =
                _mm256_loadu_si256(tempsrc.offset((3 * stride) as isize) as *const __m256i);
            let v_sum_01: __m256i = _mm256_add_epi16(v_val_0_w, v_val_1_w);
            let v_sum_23: __m256i = _mm256_add_epi16(v_val_2_w, v_val_3_w);
            let mut v_sum_0123: __m256i = _mm256_add_epi16(v_sum_01, v_sum_23);
            v_sum_0123 = _mm256_madd_epi16(v_sum_0123, one_reg);
            v_sum_total = _mm256_add_epi32(v_sum_total, v_sum_0123);
            let v_sq_0_d: __m256i = _mm256_madd_epi16(v_val_0_w, v_val_0_w);
            let v_sq_1_d: __m256i = _mm256_madd_epi16(v_val_1_w, v_val_1_w);
            let v_sq_2_d: __m256i = _mm256_madd_epi16(v_val_2_w, v_val_2_w);
            let v_sq_3_d: __m256i = _mm256_madd_epi16(v_val_3_w, v_val_3_w);
            let v_sq_01_d: __m256i = _mm256_add_epi32(v_sq_0_d, v_sq_1_d);
            let v_sq_23_d: __m256i = _mm256_add_epi32(v_sq_2_d, v_sq_3_d);
            let v_sq_0123_d: __m256i = _mm256_add_epi32(v_sq_01_d, v_sq_23_d);
            v_sse_row = _mm256_add_epi32(v_sse_row, v_sq_0123_d);
        }
        let v_sse_row_low: __m256i = _mm256_unpacklo_epi32(v_sse_row, zero_reg);
        let v_sse_row_hi: __m256i = _mm256_unpackhi_epi32(v_sse_row, zero_reg);
        v_sse_row = _mm256_add_epi64(v_sse_row_low, v_sse_row_hi);
        v_sse_total = _mm256_add_epi64(v_sse_total, v_sse_row);
        src = src.offset((4 * stride) as isize);
    }
    let v_sum_total_low: __m128i = _mm256_castsi256_si128(v_sum_total);
    let v_sum_total_hi: __m128i = _mm256_extracti128_si256::<1>(v_sum_total);
    let mut sum_128bit: __m128i = _mm_add_epi32(v_sum_total_hi, v_sum_total_low);
    sum_128bit = _mm_add_epi32(sum_128bit, _mm_srli_si128::<8>(sum_128bit));
    sum_128bit = _mm_add_epi32(sum_128bit, _mm_srli_si128::<4>(sum_128bit));
    *sum += _mm_cvtsi128_si32(sum_128bit);
    let mut v_sse_total_lo: __m128i = _mm256_castsi256_si128(v_sse_total);
    let mut v_sse_total_hi: __m128i = _mm256_extracti128_si256::<1>(v_sse_total);
    let mut sse_128bit: __m128i = _mm_add_epi64(v_sse_total_lo, v_sse_total_hi);
    sse_128bit = _mm_add_epi64(sse_128bit, _mm_unpackhi_epi64(sse_128bit, sse_128bit));
    xx_storel_64(&raw mut result as *mut core::ffi::c_void, sse_128bit);
    return result;
}

#[no_mangle]
#[target_feature(enable = "avx2")]
pub unsafe extern "C-unwind" fn aom_sum_sse_2d_i16_avx2(
    src: *const i16,
    src_stride: i32,
    width: i32,
    height: i32,
    sum: *mut i32,
) -> u64 {
    if width == 4 && height == 4 {
        return aom_sum_sse_2d_i16_4x4_sse2(src, src_stride, sum);
    } else if width == 4 && height & 3 == 0 {
        return aom_sum_sse_2d_i16_4xn_sse2(src, src_stride, height, sum);
    } else if width == 8 && height & 3 == 0 {
        return aom_sum_sse_2d_i16_nxn_sse2(src, src_stride, width, height, sum);
    } else if width & 15 == 0 && height & 3 == 0 {
        return aom_sum_sse_2d_i16_nxn_avx2(src, src_stride, width, height, sum);
    } else {
        return aom_sum_sse_2d_i16_c(src, src_stride, width, height, sum);
    }
}

// Accumulate sum of 16-bit elements in the vector
#[inline]
#[target_feature(enable = "avx2")]
unsafe fn mm256_accumulate_epi16(vec_a: __m256i) -> i32 {
    let mut vtmp1: __m128i = _mm256_extracti128_si256::<1>(vec_a);
    let mut vtmp2: __m128i = _mm256_castsi256_si128(vec_a);
    vtmp1 = _mm_add_epi16(vtmp1, vtmp2);
    vtmp2 = _mm_srli_si128::<8>(vtmp1);
    vtmp1 = _mm_add_epi16(vtmp1, vtmp2);
    vtmp2 = _mm_srli_si128::<4>(vtmp1);
    vtmp1 = _mm_add_epi16(vtmp1, vtmp2);
    vtmp2 = _mm_srli_si128::<2>(vtmp1);
    vtmp1 = _mm_add_epi16(vtmp1, vtmp2);
    return _mm_extract_epi16::<0>(vtmp1);
}

// Accumulate sum of 32-bit elements in the vector
#[inline]
#[target_feature(enable = "avx2")]
unsafe fn mm256_accumulate_epi32(vec_a: __m256i) -> i32 {
    let mut vtmp1: __m128i = _mm256_extracti128_si256::<1>(vec_a);
    let mut vtmp2: __m128i = _mm256_castsi256_si128(vec_a);
    vtmp1 = _mm_add_epi32(vtmp1, vtmp2);
    vtmp2 = _mm_srli_si128::<8>(vtmp1);
    vtmp1 = _mm_add_epi32(vtmp1, vtmp2);
    vtmp2 = _mm_srli_si128::<4>(vtmp1);
    vtmp1 = _mm_add_epi32(vtmp1, vtmp2);
    return _mm_cvtsi128_si32(vtmp1);
}

#[no_mangle]
#[target_feature(enable = "avx2")]
pub unsafe extern "C-unwind" fn aom_var_2d_u8_avx2(
    src: *mut u8,
    src_stride: i32,
    width: i32,
    height: i32,
) -> u64 {
    let mut srcp: *mut u8;
    let mut s: u64 = 0;
    let mut ss: u64 = 0;
    let mut vzero: __m256i = _mm256_setzero_si256();
    let mut v_acc_sum: __m256i = vzero;
    let mut v_acc_sqs: __m256i = vzero;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    // Process 32 elements in a row
    {
        i = 0;
        while i < width - 31 {
            srcp = src.offset(i as isize);
            // Process 8 columns at a time
            {
                j = 0;
                while j < height - 7 {
                    let mut vsrc: [__m256i; 8] = core::mem::zeroed();
                    for k in 0..8 {
                        vsrc[k as usize] =
                            _mm256_loadu_si256(srcp as *mut __m256i as *const __m256i);
                        srcp = srcp.offset(src_stride as isize);
                    }
                    for k in 0..8 {
                        let mut vsrc0: __m256i = _mm256_unpacklo_epi8(vsrc[k as usize], vzero);
                        let mut vsrc1: __m256i = _mm256_unpackhi_epi8(vsrc[k as usize], vzero);
                        v_acc_sum = _mm256_add_epi16(v_acc_sum, vsrc0);
                        v_acc_sum = _mm256_add_epi16(v_acc_sum, vsrc1);
                        let mut vsqs0: __m256i = _mm256_madd_epi16(vsrc0, vsrc0);
                        let mut vsqs1: __m256i = _mm256_madd_epi16(vsrc1, vsrc1);
                        v_acc_sqs = _mm256_add_epi32(v_acc_sqs, vsqs0);
                        v_acc_sqs = _mm256_add_epi32(v_acc_sqs, vsqs1);
                    }
                    // Update total sum and clear the vectors
                    s += mm256_accumulate_epi16(v_acc_sum) as u64;
                    ss += mm256_accumulate_epi32(v_acc_sqs) as u64;
                    v_acc_sum = vzero;
                    v_acc_sqs = vzero;
                    j += 8;
                }
            }
            // Process remaining rows (height not a multiple of 8)
            {
                while j < height {
                    let mut vsrc: __m256i =
                        _mm256_loadu_si256(srcp as *mut __m256i as *const __m256i);
                    let mut vsrc0: __m256i = _mm256_unpacklo_epi8(vsrc, vzero);
                    let mut vsrc1: __m256i = _mm256_unpackhi_epi8(vsrc, vzero);
                    v_acc_sum = _mm256_add_epi16(v_acc_sum, vsrc0);
                    v_acc_sum = _mm256_add_epi16(v_acc_sum, vsrc1);
                    let mut vsqs0: __m256i = _mm256_madd_epi16(vsrc0, vsrc0);
                    let mut vsqs1: __m256i = _mm256_madd_epi16(vsrc1, vsrc1);
                    v_acc_sqs = _mm256_add_epi32(v_acc_sqs, vsqs0);
                    v_acc_sqs = _mm256_add_epi32(v_acc_sqs, vsqs1);
                    srcp = srcp.offset(src_stride as isize);
                    j += 1;
                }
            }
            // Update total sum and clear the vectors
            s += mm256_accumulate_epi16(v_acc_sum) as u64;
            ss += mm256_accumulate_epi32(v_acc_sqs) as u64;
            v_acc_sum = vzero;
            v_acc_sqs = vzero;
            i += 32;
        }
    }
    // Process the remaining area using C
    srcp = src;
    for k in 0..height {
        for m in i..width {
            let mut val: u8 = *srcp.offset(m as isize);
            s += val as u64;
            ss += (val as i32 * val as i32) as u64;
        }
        srcp = srcp.offset(src_stride as isize);
    }
    return ss - s * s / (width * height) as u64;
}

#[no_mangle]
#[target_feature(enable = "avx2")]
pub unsafe extern "C-unwind" fn aom_var_2d_u16_avx2(
    src: *mut u8,
    src_stride: i32,
    width: i32,
    height: i32,
) -> u64 {
    let mut srcp1: *mut u16 = ((src as usize) << 1) as *mut u16;
    let mut srcp: *mut u16;
    let mut s: u64 = 0;
    let mut ss: u64 = 0;
    let mut vzero: __m256i = _mm256_setzero_si256();
    let mut v_acc_sum: __m256i = vzero;
    let mut v_acc_sqs: __m256i = vzero;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    // Process 16 elements in a row
    {
        i = 0;
        while i < width - 15 {
            srcp = srcp1.offset(i as isize);
            // Process 8 columns at a time
            {
                j = 0;
                while j < height - 8 {
                    let mut vsrc: [__m256i; 8] = core::mem::zeroed();
                    for k in 0..8 {
                        vsrc[k as usize] =
                            _mm256_loadu_si256(srcp as *mut __m256i as *const __m256i);
                        srcp = srcp.offset(src_stride as isize);
                    }
                    for k in 0..8 {
                        let mut vsrc0: __m256i = _mm256_unpacklo_epi16(vsrc[k as usize], vzero);
                        let mut vsrc1: __m256i = _mm256_unpackhi_epi16(vsrc[k as usize], vzero);
                        v_acc_sum = _mm256_add_epi32(vsrc0, v_acc_sum);
                        v_acc_sum = _mm256_add_epi32(vsrc1, v_acc_sum);
                        let mut vsqs0: __m256i =
                            _mm256_madd_epi16(vsrc[k as usize], vsrc[k as usize]);
                        v_acc_sqs = _mm256_add_epi32(v_acc_sqs, vsqs0);
                    }
                    // Update total sum and clear the vectors
                    s += mm256_accumulate_epi32(v_acc_sum) as u64;
                    ss += mm256_accumulate_epi32(v_acc_sqs) as u64;
                    v_acc_sum = vzero;
                    v_acc_sqs = vzero;
                    j += 8;
                }
            }
            // Process remaining rows (height not a multiple of 8)
            {
                while j < height {
                    let mut vsrc: __m256i =
                        _mm256_loadu_si256(srcp as *mut __m256i as *const __m256i);
                    let mut vsrc0: __m256i = _mm256_unpacklo_epi16(vsrc, vzero);
                    let mut vsrc1: __m256i = _mm256_unpackhi_epi16(vsrc, vzero);
                    v_acc_sum = _mm256_add_epi32(vsrc0, v_acc_sum);
                    v_acc_sum = _mm256_add_epi32(vsrc1, v_acc_sum);
                    let mut vsqs0: __m256i = _mm256_madd_epi16(vsrc, vsrc);
                    v_acc_sqs = _mm256_add_epi32(v_acc_sqs, vsqs0);
                    srcp = srcp.offset(src_stride as isize);
                    j += 1;
                }
            }
            // Update total sum and clear the vectors
            s += mm256_accumulate_epi32(v_acc_sum) as u64;
            ss += mm256_accumulate_epi32(v_acc_sqs) as u64;
            v_acc_sum = vzero;
            v_acc_sqs = vzero;
            i += 16;
        }
    }
    // Process the remaining area using C
    srcp = srcp1;
    for k in 0..height {
        for m in i..width {
            let mut val: u16 = *srcp.offset(m as isize);
            s += val as u64;
            ss += (val as i32 * val as i32) as u64;
        }
        srcp = srcp.offset(src_stride as isize);
    }
    return ss - s * s / (width * height) as u64;
}

// CONFIG_AV1_HIGHBITDEPTH
