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

// Translated from upstream/aom/aom_dsp/x86/blk_sse_sum_sse2.c (ADR 0056, ADR 0058).
// Upstream compiles it with -msse2. The functions were
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

use crate::aom_dsp__blk_sse_sum::aom_get_blk_sse_sum_c;
use crate::simd::aom_dsp__x86__synonyms::*;
use core::arch::x86_64::*;

#[inline]
#[target_feature(enable = "sse2")]
unsafe fn sse_sum_wd4_sse2(
    data: *const i16,
    stride: i32,
    bh: i32,
    x_sum: *mut i32,
    x2_sum: *mut i64,
) {
    let mut data_tmp: *const i16 = data;
    let mut temp_buffer1: __m128i;
    let mut temp_buffer2: __m128i;
    let mut load_pixels_low: __m128i;
    let mut load_pixels_hi: __m128i;
    let mut sum_buffer: __m128i;
    let mut sse_buffer: __m128i;
    let mut one: __m128i = _mm_set1_epi16(1);
    let mut regx_sum: __m128i = _mm_setzero_si128();
    let mut regx2_sum: __m128i = regx_sum;
    for j in 0..(bh >> 1) {
        // Load 2 rows (8 pixels) at a time.
        load_pixels_low = _mm_loadl_epi64(data_tmp as *const __m128i);
        load_pixels_hi = _mm_loadl_epi64(data_tmp.offset(stride as isize) as *const __m128i);
        load_pixels_low = _mm_unpacklo_epi64(load_pixels_low, load_pixels_hi);
        sum_buffer = _mm_madd_epi16(load_pixels_low, one);
        sse_buffer = _mm_madd_epi16(load_pixels_low, load_pixels_low);
        regx_sum = _mm_add_epi32(sum_buffer, regx_sum);
        regx2_sum = _mm_add_epi32(sse_buffer, regx2_sum);
        data_tmp = data_tmp.offset((2 * stride) as isize);
    }
    regx_sum = _mm_add_epi32(regx_sum, _mm_srli_si128::<8>(regx_sum));
    regx_sum = _mm_add_epi32(regx_sum, _mm_srli_si128::<4>(regx_sum));
    *x_sum = _mm_cvtsi128_si32(regx_sum);
    temp_buffer1 = _mm_unpacklo_epi32(regx2_sum, _mm_setzero_si128());
    temp_buffer2 = _mm_unpackhi_epi32(regx2_sum, _mm_setzero_si128());
    regx2_sum = _mm_add_epi64(temp_buffer1, temp_buffer2);
    regx2_sum = _mm_add_epi64(regx2_sum, _mm_srli_si128::<8>(regx2_sum));
    *x2_sum += _mm_cvtsi128_si64(regx2_sum);
}

#[inline]
#[target_feature(enable = "sse2")]
unsafe fn sse_sum_wd8_sse2(
    data: *const i16,
    stride: i32,
    bh: i32,
    x_sum: *mut i32,
    x2_sum: *mut i64,
    loop_cycles: i32,
) {
    let mut data_tmp: *const i16;
    let mut temp_buffer1: __m128i;
    let mut temp_buffer2: __m128i;
    let mut one: __m128i = _mm_set1_epi16(1);
    let mut regx_sum: __m128i = _mm_setzero_si128();
    let mut regx2_sum: __m128i = regx_sum;
    let mut load_pixels: __m128i;
    let mut sum_buffer: __m128i;
    let mut sse_buffer: __m128i;
    for i in 0..loop_cycles {
        data_tmp = data.offset((8 * i) as isize);
        for j in 0..bh {
            // Load 1 row (8-pixels) at a time.
            load_pixels = _mm_loadu_si128(data_tmp as *const __m128i);
            sum_buffer = _mm_madd_epi16(load_pixels, one);
            sse_buffer = _mm_madd_epi16(load_pixels, load_pixels);
            regx_sum = _mm_add_epi32(sum_buffer, regx_sum);
            regx2_sum = _mm_add_epi32(sse_buffer, regx2_sum);
            data_tmp = data_tmp.offset(stride as isize);
        }
    }
    regx_sum = _mm_add_epi32(regx_sum, _mm_srli_si128::<8>(regx_sum));
    regx_sum = _mm_add_epi32(regx_sum, _mm_srli_si128::<4>(regx_sum));
    *x_sum += _mm_cvtsi128_si32(regx_sum);
    temp_buffer1 = _mm_unpacklo_epi32(regx2_sum, _mm_setzero_si128());
    temp_buffer2 = _mm_unpackhi_epi32(regx2_sum, _mm_setzero_si128());
    regx2_sum = _mm_add_epi64(temp_buffer1, temp_buffer2);
    regx2_sum = _mm_add_epi64(regx2_sum, _mm_srli_si128::<8>(regx2_sum));
    *x2_sum += _mm_cvtsi128_si64(regx2_sum);
}

// This functions adds SSE2 Support for the functions 'get_blk_sse_sum_c'
#[no_mangle]
#[target_feature(enable = "sse2")]
pub unsafe extern "C-unwind" fn aom_get_blk_sse_sum_sse2(
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
                sse_sum_wd4_sse2(data, stride, bh, x_sum, x2_sum);
            }
            8 | 16 => {
                sse_sum_wd8_sse2(data, stride, bh, x_sum, x2_sum, bw >> 3);
            }
            32 => {
                // For widths 32 and 64, the registers may overflow. So compute
                // partial widths at a time.
                if bh <= 32 {
                    sse_sum_wd8_sse2(data, stride, bh, x_sum, x2_sum, bw >> 3);
                } else {
                    sse_sum_wd8_sse2(data, stride, 32, x_sum, x2_sum, bw >> 3);
                    sse_sum_wd8_sse2(
                        data.offset((32 * stride) as isize),
                        stride,
                        32,
                        x_sum,
                        x2_sum,
                        bw >> 3,
                    );
                }
            }
            64 => {
                if bh <= 16 {
                    sse_sum_wd8_sse2(data, stride, bh, x_sum, x2_sum, bw >> 3);
                } else {
                    for i in (0..bh).step_by(16) {
                        sse_sum_wd8_sse2(
                            data.offset((i * stride) as isize),
                            stride,
                            16,
                            x_sum,
                            x2_sum,
                            bw >> 3,
                        );
                    }
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
