/*
 * Copyright (c) 2019, Alliance for Open Media. All rights reserved.
 *
 * This source code is subject to the terms of the BSD 2 Clause License and
 * the Alliance for Open Media Patent License 1.0. If the BSD 2 Clause License
 * was not distributed with this source code in the LICENSE file, you can
 * obtain it at www.aomedia.org/license/software. If the Alliance for Open
 * Media Patent License 1.0 was not distributed with this source code in the
 * PATENTS file, you can obtain it at www.aomedia.org/license/patent.
 */

// Translated from upstream/aom/aom_dsp/x86/blk_sse_sum_avx2.c (ADR 0056, ADR 0058).
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

use crate::aom_dsp__blk_sse_sum::aom_get_blk_sse_sum_c;
use crate::simd::aom_dsp__x86__synonyms::*;
use crate::simd::aom_dsp__x86__synonyms_avx2::*;
use core::arch::x86_64::*;

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn accumulate_sse_sum(
    mut regx_sum: __m256i,
    mut regx2_sum: __m256i,
    x_sum: *mut i32,
    x2_sum: *mut i64,
) {
    let mut sum_buffer: __m256i;
    let mut sse_buffer: __m256i;
    let mut out_buffer: __m128i;
    // Accumulate the various elements of register into first element.
    sum_buffer = _mm256_permute2f128_si256::<1>(regx_sum, regx_sum);
    regx_sum = _mm256_add_epi32(sum_buffer, regx_sum);
    regx_sum = _mm256_add_epi32(regx_sum, _mm256_srli_si256::<8>(regx_sum));
    regx_sum = _mm256_add_epi32(regx_sum, _mm256_srli_si256::<4>(regx_sum));
    sse_buffer = _mm256_permute2f128_si256::<1>(regx2_sum, regx2_sum);
    regx2_sum = _mm256_add_epi64(sse_buffer, regx2_sum);
    regx2_sum = _mm256_add_epi64(regx2_sum, _mm256_srli_si256::<8>(regx2_sum));
    out_buffer = _mm256_castsi256_si128(regx_sum);
    *x_sum += _mm_cvtsi128_si32(out_buffer);
    out_buffer = _mm256_castsi256_si128(regx2_sum);
    *x2_sum += _mm_cvtsi128_si64(out_buffer);
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn sse_sum_wd4_avx2(
    data: *const i16,
    stride: i32,
    bh: i32,
    x_sum: *mut i32,
    x2_sum: *mut i64,
) {
    let mut row1: __m128i;
    let mut row2: __m128i;
    let mut row3: __m128i;
    let mut regx_sum: __m256i;
    let mut regx2_sum: __m256i;
    let mut load_pixels: __m256i;
    let mut sum_buffer: __m256i;
    let mut sse_buffer: __m256i;
    let mut temp_buffer1: __m256i;
    let mut temp_buffer2: __m256i;
    let mut row_sum_buffer: __m256i;
    let mut row_sse_buffer: __m256i;
    let mut data_tmp: *const i16 = data;
    let mut one: __m256i = _mm256_set1_epi16(1);
    regx_sum = _mm256_setzero_si256();
    regx2_sum = regx_sum;
    sum_buffer = _mm256_setzero_si256();
    sse_buffer = sum_buffer;
    for j in 0..(bh >> 2) {
        // Load 4 rows at a time.
        row1 = _mm_loadl_epi64(data_tmp as *const __m128i);
        row2 = _mm_loadl_epi64(data_tmp.offset(stride as isize) as *const __m128i);
        row1 = _mm_unpacklo_epi64(row1, row2);
        row2 = _mm_loadl_epi64(data_tmp.offset((2 * stride) as isize) as *const __m128i);
        row3 = _mm_loadl_epi64(data_tmp.offset((3 * stride) as isize) as *const __m128i);
        row2 = _mm_unpacklo_epi64(row2, row3);
        load_pixels = _mm256_insertf128_si256::<1>(_mm256_castsi128_si256(row1), row2);
        row_sum_buffer = _mm256_madd_epi16(load_pixels, one);
        row_sse_buffer = _mm256_madd_epi16(load_pixels, load_pixels);
        sum_buffer = _mm256_add_epi32(row_sum_buffer, sum_buffer);
        sse_buffer = _mm256_add_epi32(row_sse_buffer, sse_buffer);
        data_tmp = data_tmp.offset((4 * stride) as isize);
    }
    // To prevent 32-bit variable overflow, unpack the elements to 64-bit.
    temp_buffer1 = _mm256_unpacklo_epi32(sse_buffer, _mm256_setzero_si256());
    temp_buffer2 = _mm256_unpackhi_epi32(sse_buffer, _mm256_setzero_si256());
    sse_buffer = _mm256_add_epi64(temp_buffer1, temp_buffer2);
    regx_sum = _mm256_add_epi32(sum_buffer, regx_sum);
    regx2_sum = _mm256_add_epi64(sse_buffer, regx2_sum);
    accumulate_sse_sum(regx_sum, regx2_sum, x_sum, x2_sum);
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn sse_sum_wd8_avx2(
    data: *const i16,
    stride: i32,
    bh: i32,
    x_sum: *mut i32,
    x2_sum: *mut i64,
) {
    let mut load_128bit: __m128i;
    let mut load_next_128bit: __m128i;
    let mut regx_sum: __m256i;
    let mut regx2_sum: __m256i;
    let mut load_pixels: __m256i;
    let mut sum_buffer: __m256i;
    let mut sse_buffer: __m256i;
    let mut temp_buffer1: __m256i;
    let mut temp_buffer2: __m256i;
    let mut row_sum_buffer: __m256i;
    let mut row_sse_buffer: __m256i;
    let mut data_tmp: *const i16 = data;
    let mut one: __m256i = _mm256_set1_epi16(1);
    regx_sum = _mm256_setzero_si256();
    regx2_sum = regx_sum;
    sum_buffer = _mm256_setzero_si256();
    sse_buffer = sum_buffer;
    for j in 0..(bh >> 1) {
        // Load 2 rows at a time.
        load_128bit = _mm_loadu_si128(data_tmp as *const __m128i);
        load_next_128bit = _mm_loadu_si128(data_tmp.offset(stride as isize) as *const __m128i);
        load_pixels =
            _mm256_insertf128_si256::<1>(_mm256_castsi128_si256(load_128bit), load_next_128bit);
        row_sum_buffer = _mm256_madd_epi16(load_pixels, one);
        row_sse_buffer = _mm256_madd_epi16(load_pixels, load_pixels);
        sum_buffer = _mm256_add_epi32(row_sum_buffer, sum_buffer);
        sse_buffer = _mm256_add_epi32(row_sse_buffer, sse_buffer);
        data_tmp = data_tmp.offset((2 * stride) as isize);
    }
    temp_buffer1 = _mm256_unpacklo_epi32(sse_buffer, _mm256_setzero_si256());
    temp_buffer2 = _mm256_unpackhi_epi32(sse_buffer, _mm256_setzero_si256());
    sse_buffer = _mm256_add_epi64(temp_buffer1, temp_buffer2);
    regx_sum = _mm256_add_epi32(sum_buffer, regx_sum);
    regx2_sum = _mm256_add_epi64(sse_buffer, regx2_sum);
    accumulate_sse_sum(regx_sum, regx2_sum, x_sum, x2_sum);
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn sse_sum_wd16_avx2(
    data: *const i16,
    stride: i32,
    bh: i32,
    x_sum: *mut i32,
    x2_sum: *mut i64,
    loop_count: i32,
) {
    let mut regx_sum: __m256i;
    let mut regx2_sum: __m256i;
    let mut load_pixels: __m256i;
    let mut sum_buffer: __m256i;
    let mut sse_buffer: __m256i;
    let mut temp_buffer1: __m256i;
    let mut temp_buffer2: __m256i;
    let mut row_sum_buffer: __m256i;
    let mut row_sse_buffer: __m256i;
    let mut data_tmp: *const i16 = data;
    let mut one: __m256i = _mm256_set1_epi16(1);
    regx_sum = _mm256_setzero_si256();
    regx2_sum = regx_sum;
    sum_buffer = _mm256_setzero_si256();
    sse_buffer = sum_buffer;
    for i in 0..loop_count {
        data_tmp = data.offset((16 * i) as isize);
        for j in 0..bh {
            load_pixels = _mm256_lddqu_si256(data_tmp as *const __m256i);
            row_sum_buffer = _mm256_madd_epi16(load_pixels, one);
            row_sse_buffer = _mm256_madd_epi16(load_pixels, load_pixels);
            sum_buffer = _mm256_add_epi32(row_sum_buffer, sum_buffer);
            sse_buffer = _mm256_add_epi32(row_sse_buffer, sse_buffer);
            data_tmp = data_tmp.offset(stride as isize);
        }
    }
    temp_buffer1 = _mm256_unpacklo_epi32(sse_buffer, _mm256_setzero_si256());
    temp_buffer2 = _mm256_unpackhi_epi32(sse_buffer, _mm256_setzero_si256());
    sse_buffer = _mm256_add_epi64(temp_buffer1, temp_buffer2);
    regx_sum = _mm256_add_epi32(sum_buffer, regx_sum);
    regx2_sum = _mm256_add_epi64(sse_buffer, regx2_sum);
    accumulate_sse_sum(regx_sum, regx2_sum, x_sum, x2_sum);
}

#[no_mangle]
#[target_feature(enable = "avx2")]
pub unsafe extern "C-unwind" fn aom_get_blk_sse_sum_avx2(
    data: *const i16,
    stride: i32,
    bw: i32,
    bh: i32,
    x_sum: *mut i32,
    x2_sum: *mut i64,
) {
    *x_sum = 0;
    *x2_sum = 0;
    if bh & 3 == 0 {
        match bw {
            4 => {
                // For smaller block widths, compute multiple rows simultaneously.
                sse_sum_wd4_avx2(data, stride, bh, x_sum, x2_sum);
            }
            8 => {
                sse_sum_wd8_avx2(data, stride, bh, x_sum, x2_sum);
            }
            16 | 32 => {
                sse_sum_wd16_avx2(data, stride, bh, x_sum, x2_sum, bw >> 4);
            }
            64 => {
                // 32-bit variables will overflow for 64 rows at a single time, so
                // compute 32 rows at a time.
                if bh <= 32 {
                    sse_sum_wd16_avx2(data, stride, bh, x_sum, x2_sum, bw >> 4);
                } else {
                    sse_sum_wd16_avx2(data, stride, 32, x_sum, x2_sum, bw >> 4);
                    sse_sum_wd16_avx2(
                        data.offset((32 * stride) as isize),
                        stride,
                        32,
                        x_sum,
                        x2_sum,
                        bw >> 4,
                    );
                }
            }
            _ => {
                aom_get_blk_sse_sum_c(data, stride, bw, bh, x_sum, x2_sum);
            }
        }
    } else {
        aom_get_blk_sse_sum_c(data, stride, bw, bh, x_sum, x2_sum);
    }
}
