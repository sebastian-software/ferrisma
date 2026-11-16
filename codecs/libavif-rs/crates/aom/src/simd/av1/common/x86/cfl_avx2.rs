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

// Translated from upstream/aom/av1/common/x86/cfl_avx2.c (ADR 0056, ADR 0058).
// Upstream compiles it with -mavx2. The functions were
// transliterated with a typing-aware C-subset printer (not C2Rust) and
// reviewed against the C function by function: the same sequence of intrinsic
// and helper calls, constants and operators in order (ADR 0058).
// The size wrappers and RTCD getters that the CFL_* macros of av1/common/cfl.h generate are
// expanded with the C preprocessor before printing (tools/x86_c2r/c2r.py `cpp`); the
// wrappers keep the C ABI because the getters return them as C function pointers.
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
use crate::simd::av1__common__x86__cfl_sse2::*;
use crate::simd::av1__common__x86__cfl_ssse3::*;
use core::arch::x86_64::*;

// av1/common/blockd.h
const CFL_BUF_LINE: i32 = 32;
const CFL_BUF_LINE_I128: i32 = CFL_BUF_LINE >> 3;
const CFL_BUF_LINE_I256: i32 = CFL_BUF_LINE >> 4;
// av1/common/enums.h
const TX_SIZES_ALL: i32 = 19;

// av1/common/cfl.h and config/av1_rtcd.h: the function types the RTCD getters return.
type cfl_subsample_lbd_fn = Option<unsafe extern "C-unwind" fn(*const u8, i32, *mut u16)>;
type cfl_subsample_hbd_fn = Option<unsafe extern "C-unwind" fn(*const u16, i32, *mut u16)>;
type cfl_subtract_average_fn = Option<unsafe extern "C-unwind" fn(*const u16, *mut i16)>;
type cfl_predict_lbd_fn = Option<unsafe extern "C-unwind" fn(*const i16, *mut u8, i32, i32)>;
type cfl_predict_hbd_fn = Option<unsafe extern "C-unwind" fn(*const i16, *mut u16, i32, i32, i32)>;

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn cfl_luma_subsampling_420_lbd_avx2(
    mut input: *const u8,
    input_stride: i32,
    pred_buf_q3: *mut u16,
    width: i32,
    height: i32,
) {
    let _ = width; // Forever 32
    let twos: __m256i = _mm256_set1_epi8(2); // Thirty two twos
    let luma_stride: i32 = input_stride << 1;
    let mut row: *mut __m256i = pred_buf_q3 as *mut __m256i;
    let mut row_end: *const __m256i =
        row.offset(((height >> 1) * CFL_BUF_LINE_I256) as isize) as *const __m256i;
    loop {
        let mut top: __m256i = _mm256_loadu_si256(input as *mut __m256i as *const __m256i);
        let mut bot: __m256i = _mm256_loadu_si256(
            input.offset(input_stride as isize) as *mut __m256i as *const __m256i
        );
        let mut top_16x16: __m256i = _mm256_maddubs_epi16(top, twos);
        let mut bot_16x16: __m256i = _mm256_maddubs_epi16(bot, twos);
        let mut sum_16x16: __m256i = _mm256_add_epi16(top_16x16, bot_16x16);
        _mm256_storeu_si256(row, sum_16x16);
        input = input.offset(luma_stride as isize);
        row = row.offset(CFL_BUF_LINE_I256 as isize);
        if !(row < row_end as *mut __m256i) {
            break;
        }
    }
}

#[target_feature(enable = "avx2")]
pub unsafe extern "C-unwind" fn cfl_subsample_lbd_420_32x32_avx2(
    cfl_type: *const u8,
    input_stride: i32,
    output_q3: *mut u16,
) {
    cfl_luma_subsampling_420_lbd_avx2(cfl_type, input_stride, output_q3, 32, 32);
}

#[target_feature(enable = "avx2")]
pub unsafe extern "C-unwind" fn cfl_subsample_lbd_420_32x16_avx2(
    cfl_type: *const u8,
    input_stride: i32,
    output_q3: *mut u16,
) {
    cfl_luma_subsampling_420_lbd_avx2(cfl_type, input_stride, output_q3, 32, 16);
}

#[target_feature(enable = "avx2")]
pub unsafe extern "C-unwind" fn cfl_subsample_lbd_420_32x8_avx2(
    cfl_type: *const u8,
    input_stride: i32,
    output_q3: *mut u16,
) {
    cfl_luma_subsampling_420_lbd_avx2(cfl_type, input_stride, output_q3, 32, 8);
}

#[no_mangle]
#[target_feature(enable = "avx2")]
pub unsafe extern "C-unwind" fn cfl_get_luma_subsampling_420_lbd_avx2(
    tx_size: u8,
) -> cfl_subsample_lbd_fn {
    let mut subfn_420: [cfl_subsample_lbd_fn; (TX_SIZES_ALL) as usize] = [
        Some(cfl_subsample_lbd_420_4x4_ssse3),
        Some(cfl_subsample_lbd_420_8x8_ssse3),
        Some(cfl_subsample_lbd_420_16x16_ssse3),
        Some(cfl_subsample_lbd_420_32x32_avx2),
        None,
        Some(cfl_subsample_lbd_420_4x8_ssse3),
        Some(cfl_subsample_lbd_420_8x4_ssse3),
        Some(cfl_subsample_lbd_420_8x16_ssse3),
        Some(cfl_subsample_lbd_420_16x8_ssse3),
        Some(cfl_subsample_lbd_420_16x32_ssse3),
        Some(cfl_subsample_lbd_420_32x16_avx2),
        None,
        None,
        Some(cfl_subsample_lbd_420_4x16_ssse3),
        Some(cfl_subsample_lbd_420_16x4_ssse3),
        Some(cfl_subsample_lbd_420_8x32_ssse3),
        Some(cfl_subsample_lbd_420_32x8_avx2),
        None,
        None,
    ];
    return subfn_420[tx_size as usize];
}

// Adds 2 pixels (in a 2x1 grid) and multiplies them by 4. Resulting in a more
// precise version of a box filter 4:2:2 pixel subsampling in Q3.
//
// The CfL prediction buffer is always of size CFL_BUF_SQUARE. However, the
// active area is specified using width and height.
//
// Note: We don't need to worry about going over the active area, as long as we
// stay inside the CfL prediction buffer.
#[inline]
#[target_feature(enable = "avx2")]
unsafe fn cfl_luma_subsampling_422_lbd_avx2(
    mut input: *const u8,
    input_stride: i32,
    pred_buf_q3: *mut u16,
    width: i32,
    height: i32,
) {
    let _ = width; // Forever 32
    let fours: __m256i = _mm256_set1_epi8(4); // Thirty two fours
    let mut row: *mut __m256i = pred_buf_q3 as *mut __m256i;
    let mut row_end: *const __m256i =
        row.offset((height * CFL_BUF_LINE_I256) as isize) as *const __m256i;
    loop {
        let mut top: __m256i = _mm256_loadu_si256(input as *mut __m256i as *const __m256i);
        let mut top_16x16: __m256i = _mm256_maddubs_epi16(top, fours);
        _mm256_storeu_si256(row, top_16x16);
        input = input.offset(input_stride as isize);
        row = row.offset(CFL_BUF_LINE_I256 as isize);
        if !(row < row_end as *mut __m256i) {
            break;
        }
    }
}

#[target_feature(enable = "avx2")]
pub unsafe extern "C-unwind" fn cfl_subsample_lbd_422_32x32_avx2(
    cfl_type: *const u8,
    input_stride: i32,
    output_q3: *mut u16,
) {
    cfl_luma_subsampling_422_lbd_avx2(cfl_type, input_stride, output_q3, 32, 32);
}

#[target_feature(enable = "avx2")]
pub unsafe extern "C-unwind" fn cfl_subsample_lbd_422_32x16_avx2(
    cfl_type: *const u8,
    input_stride: i32,
    output_q3: *mut u16,
) {
    cfl_luma_subsampling_422_lbd_avx2(cfl_type, input_stride, output_q3, 32, 16);
}

#[target_feature(enable = "avx2")]
pub unsafe extern "C-unwind" fn cfl_subsample_lbd_422_32x8_avx2(
    cfl_type: *const u8,
    input_stride: i32,
    output_q3: *mut u16,
) {
    cfl_luma_subsampling_422_lbd_avx2(cfl_type, input_stride, output_q3, 32, 8);
}

#[no_mangle]
#[target_feature(enable = "avx2")]
pub unsafe extern "C-unwind" fn cfl_get_luma_subsampling_422_lbd_avx2(
    tx_size: u8,
) -> cfl_subsample_lbd_fn {
    let mut subfn_422: [cfl_subsample_lbd_fn; (TX_SIZES_ALL) as usize] = [
        Some(cfl_subsample_lbd_422_4x4_ssse3),
        Some(cfl_subsample_lbd_422_8x8_ssse3),
        Some(cfl_subsample_lbd_422_16x16_ssse3),
        Some(cfl_subsample_lbd_422_32x32_avx2),
        None,
        Some(cfl_subsample_lbd_422_4x8_ssse3),
        Some(cfl_subsample_lbd_422_8x4_ssse3),
        Some(cfl_subsample_lbd_422_8x16_ssse3),
        Some(cfl_subsample_lbd_422_16x8_ssse3),
        Some(cfl_subsample_lbd_422_16x32_ssse3),
        Some(cfl_subsample_lbd_422_32x16_avx2),
        None,
        None,
        Some(cfl_subsample_lbd_422_4x16_ssse3),
        Some(cfl_subsample_lbd_422_16x4_ssse3),
        Some(cfl_subsample_lbd_422_8x32_ssse3),
        Some(cfl_subsample_lbd_422_32x8_avx2),
        None,
        None,
    ];
    return subfn_422[tx_size as usize];
}

// Multiplies the pixels by 8 (scaling in Q3). The AVX2 subsampling is only
// performed on block of width 32.
//
// The CfL prediction buffer is always of size CFL_BUF_SQUARE. However, the
// active area is specified using width and height.
//
// Note: We don't need to worry about going over the active area, as long as we
// stay inside the CfL prediction buffer.
#[inline]
#[target_feature(enable = "avx2")]
unsafe fn cfl_luma_subsampling_444_lbd_avx2(
    mut input: *const u8,
    input_stride: i32,
    pred_buf_q3: *mut u16,
    width: i32,
    height: i32,
) {
    let _ = width; // Forever 32
    let mut row: *mut __m256i = pred_buf_q3 as *mut __m256i;
    let mut row_end: *const __m256i =
        row.offset((height * CFL_BUF_LINE_I256) as isize) as *const __m256i;
    let zeros: __m256i = _mm256_setzero_si256();
    loop {
        let mut top: __m256i = _mm256_loadu_si256(input as *mut __m256i as *const __m256i);
        top = _mm256_permute4x64_epi64::<{ ((3) << 6) | ((1) << 4) | ((2) << 2) | (0) }>(top);
        let mut row_lo: __m256i = _mm256_unpacklo_epi8(top, zeros);
        row_lo = _mm256_slli_epi16::<{ 3 }>(row_lo);
        let mut row_hi: __m256i = _mm256_unpackhi_epi8(top, zeros);
        row_hi = _mm256_slli_epi16::<{ 3 }>(row_hi);
        _mm256_storeu_si256(row, row_lo);
        _mm256_storeu_si256(row.offset(1), row_hi);
        input = input.offset(input_stride as isize);
        row = row.offset(CFL_BUF_LINE_I256 as isize);
        if !(row < row_end as *mut __m256i) {
            break;
        }
    }
}

#[target_feature(enable = "avx2")]
pub unsafe extern "C-unwind" fn cfl_subsample_lbd_444_32x32_avx2(
    cfl_type: *const u8,
    input_stride: i32,
    output_q3: *mut u16,
) {
    cfl_luma_subsampling_444_lbd_avx2(cfl_type, input_stride, output_q3, 32, 32);
}

#[target_feature(enable = "avx2")]
pub unsafe extern "C-unwind" fn cfl_subsample_lbd_444_32x16_avx2(
    cfl_type: *const u8,
    input_stride: i32,
    output_q3: *mut u16,
) {
    cfl_luma_subsampling_444_lbd_avx2(cfl_type, input_stride, output_q3, 32, 16);
}

#[target_feature(enable = "avx2")]
pub unsafe extern "C-unwind" fn cfl_subsample_lbd_444_32x8_avx2(
    cfl_type: *const u8,
    input_stride: i32,
    output_q3: *mut u16,
) {
    cfl_luma_subsampling_444_lbd_avx2(cfl_type, input_stride, output_q3, 32, 8);
}

#[no_mangle]
#[target_feature(enable = "avx2")]
pub unsafe extern "C-unwind" fn cfl_get_luma_subsampling_444_lbd_avx2(
    tx_size: u8,
) -> cfl_subsample_lbd_fn {
    let mut subfn_444: [cfl_subsample_lbd_fn; (TX_SIZES_ALL) as usize] = [
        Some(cfl_subsample_lbd_444_4x4_ssse3),
        Some(cfl_subsample_lbd_444_8x8_ssse3),
        Some(cfl_subsample_lbd_444_16x16_ssse3),
        Some(cfl_subsample_lbd_444_32x32_avx2),
        None,
        Some(cfl_subsample_lbd_444_4x8_ssse3),
        Some(cfl_subsample_lbd_444_8x4_ssse3),
        Some(cfl_subsample_lbd_444_8x16_ssse3),
        Some(cfl_subsample_lbd_444_16x8_ssse3),
        Some(cfl_subsample_lbd_444_16x32_ssse3),
        Some(cfl_subsample_lbd_444_32x16_avx2),
        None,
        None,
        Some(cfl_subsample_lbd_444_4x16_ssse3),
        Some(cfl_subsample_lbd_444_16x4_ssse3),
        Some(cfl_subsample_lbd_444_8x32_ssse3),
        Some(cfl_subsample_lbd_444_32x8_avx2),
        None,
        None,
    ];
    return subfn_444[tx_size as usize];
}

// Adds 4 pixels (in a 2x2 grid) and multiplies them by 2. Resulting in a more
// precise version of a box filter 4:2:0 pixel subsampling in Q3.
//
// The CfL prediction buffer is always of size CFL_BUF_SQUARE. However, the
// active area is specified using width and height.
//
// Note: We don't need to worry about going over the active area, as long as we
// stay inside the CfL prediction buffer.
//
// Note: For 4:2:0 luma subsampling, the width will never be greater than 16.
#[inline]
#[target_feature(enable = "avx2")]
unsafe fn cfl_luma_subsampling_420_hbd_avx2(
    mut input: *const u16,
    input_stride: i32,
    pred_buf_q3: *mut u16,
    width: i32,
    height: i32,
) {
    let _ = width; // Forever 32
    let luma_stride: i32 = input_stride << 1;
    let mut row: *mut __m256i = pred_buf_q3 as *mut __m256i;
    let mut row_end: *const __m256i =
        row.offset(((height >> 1) * CFL_BUF_LINE_I256) as isize) as *const __m256i;
    loop {
        let mut top: __m256i = _mm256_loadu_si256(input as *mut __m256i as *const __m256i);
        let mut bot: __m256i = _mm256_loadu_si256(
            input.offset(input_stride as isize) as *mut __m256i as *const __m256i
        );
        let mut sum: __m256i = _mm256_add_epi16(top, bot);
        let mut top_1: __m256i =
            _mm256_loadu_si256(input.offset(16) as *mut __m256i as *const __m256i);
        let mut bot_1: __m256i = _mm256_loadu_si256(
            input.offset(16).offset(input_stride as isize) as *mut __m256i as *const __m256i
        );
        let mut sum_1: __m256i = _mm256_add_epi16(top_1, bot_1);
        let mut hsum: __m256i = _mm256_hadd_epi16(sum, sum_1);
        hsum = _mm256_permute4x64_epi64::<{ ((3) << 6) | ((1) << 4) | ((2) << 2) | (0) }>(hsum);
        hsum = _mm256_add_epi16(hsum, hsum);
        _mm256_storeu_si256(row, hsum);
        input = input.offset(luma_stride as isize);
        row = row.offset(CFL_BUF_LINE_I256 as isize);
        if !(row < row_end as *mut __m256i) {
            break;
        }
    }
}

#[target_feature(enable = "avx2")]
pub unsafe extern "C-unwind" fn cfl_subsample_hbd_420_32x32_avx2(
    cfl_type: *const u16,
    input_stride: i32,
    output_q3: *mut u16,
) {
    cfl_luma_subsampling_420_hbd_avx2(cfl_type, input_stride, output_q3, 32, 32);
}

#[target_feature(enable = "avx2")]
pub unsafe extern "C-unwind" fn cfl_subsample_hbd_420_32x16_avx2(
    cfl_type: *const u16,
    input_stride: i32,
    output_q3: *mut u16,
) {
    cfl_luma_subsampling_420_hbd_avx2(cfl_type, input_stride, output_q3, 32, 16);
}

#[target_feature(enable = "avx2")]
pub unsafe extern "C-unwind" fn cfl_subsample_hbd_420_32x8_avx2(
    cfl_type: *const u16,
    input_stride: i32,
    output_q3: *mut u16,
) {
    cfl_luma_subsampling_420_hbd_avx2(cfl_type, input_stride, output_q3, 32, 8);
}

#[no_mangle]
#[target_feature(enable = "avx2")]
pub unsafe extern "C-unwind" fn cfl_get_luma_subsampling_420_hbd_avx2(
    tx_size: u8,
) -> cfl_subsample_hbd_fn {
    let mut subfn_420: [cfl_subsample_hbd_fn; (TX_SIZES_ALL) as usize] = [
        Some(cfl_subsample_hbd_420_4x4_ssse3),
        Some(cfl_subsample_hbd_420_8x8_ssse3),
        Some(cfl_subsample_hbd_420_16x16_ssse3),
        Some(cfl_subsample_hbd_420_32x32_avx2),
        None,
        Some(cfl_subsample_hbd_420_4x8_ssse3),
        Some(cfl_subsample_hbd_420_8x4_ssse3),
        Some(cfl_subsample_hbd_420_8x16_ssse3),
        Some(cfl_subsample_hbd_420_16x8_ssse3),
        Some(cfl_subsample_hbd_420_16x32_ssse3),
        Some(cfl_subsample_hbd_420_32x16_avx2),
        None,
        None,
        Some(cfl_subsample_hbd_420_4x16_ssse3),
        Some(cfl_subsample_hbd_420_16x4_ssse3),
        Some(cfl_subsample_hbd_420_8x32_ssse3),
        Some(cfl_subsample_hbd_420_32x8_avx2),
        None,
        None,
    ];
    return subfn_420[tx_size as usize];
}

// Adds 2 pixels (in a 2x1 grid) and multiplies them by 4. Resulting in a more
// precise version of a box filter 4:2:2 pixel subsampling in Q3.
//
// The CfL prediction buffer is always of size CFL_BUF_SQUARE. However, the
// active area is specified using width and height.
//
// Note: We don't need to worry about going over the active area, as long as we
// stay inside the CfL prediction buffer.
//
#[inline]
#[target_feature(enable = "avx2")]
unsafe fn cfl_luma_subsampling_422_hbd_avx2(
    mut input: *const u16,
    input_stride: i32,
    pred_buf_q3: *mut u16,
    width: i32,
    height: i32,
) {
    let _ = width; // Forever 32
    let mut row: *mut __m256i = pred_buf_q3 as *mut __m256i;
    let mut row_end: *const __m256i =
        row.offset((height * CFL_BUF_LINE_I256) as isize) as *const __m256i;
    loop {
        let mut top: __m256i = _mm256_loadu_si256(input as *mut __m256i as *const __m256i);
        let mut top_1: __m256i =
            _mm256_loadu_si256(input.offset(16) as *mut __m256i as *const __m256i);
        let mut hsum: __m256i = _mm256_hadd_epi16(top, top_1);
        hsum = _mm256_permute4x64_epi64::<{ ((3) << 6) | ((1) << 4) | ((2) << 2) | (0) }>(hsum);
        hsum = _mm256_slli_epi16::<{ 2 }>(hsum);
        _mm256_storeu_si256(row, hsum);
        input = input.offset(input_stride as isize);
        row = row.offset(CFL_BUF_LINE_I256 as isize);
        if !(row < row_end as *mut __m256i) {
            break;
        }
    }
}

#[target_feature(enable = "avx2")]
pub unsafe extern "C-unwind" fn cfl_subsample_hbd_422_32x32_avx2(
    cfl_type: *const u16,
    input_stride: i32,
    output_q3: *mut u16,
) {
    cfl_luma_subsampling_422_hbd_avx2(cfl_type, input_stride, output_q3, 32, 32);
}

#[target_feature(enable = "avx2")]
pub unsafe extern "C-unwind" fn cfl_subsample_hbd_422_32x16_avx2(
    cfl_type: *const u16,
    input_stride: i32,
    output_q3: *mut u16,
) {
    cfl_luma_subsampling_422_hbd_avx2(cfl_type, input_stride, output_q3, 32, 16);
}

#[target_feature(enable = "avx2")]
pub unsafe extern "C-unwind" fn cfl_subsample_hbd_422_32x8_avx2(
    cfl_type: *const u16,
    input_stride: i32,
    output_q3: *mut u16,
) {
    cfl_luma_subsampling_422_hbd_avx2(cfl_type, input_stride, output_q3, 32, 8);
}

#[no_mangle]
#[target_feature(enable = "avx2")]
pub unsafe extern "C-unwind" fn cfl_get_luma_subsampling_422_hbd_avx2(
    tx_size: u8,
) -> cfl_subsample_hbd_fn {
    let mut subfn_422: [cfl_subsample_hbd_fn; (TX_SIZES_ALL) as usize] = [
        Some(cfl_subsample_hbd_422_4x4_ssse3),
        Some(cfl_subsample_hbd_422_8x8_ssse3),
        Some(cfl_subsample_hbd_422_16x16_ssse3),
        Some(cfl_subsample_hbd_422_32x32_avx2),
        None,
        Some(cfl_subsample_hbd_422_4x8_ssse3),
        Some(cfl_subsample_hbd_422_8x4_ssse3),
        Some(cfl_subsample_hbd_422_8x16_ssse3),
        Some(cfl_subsample_hbd_422_16x8_ssse3),
        Some(cfl_subsample_hbd_422_16x32_ssse3),
        Some(cfl_subsample_hbd_422_32x16_avx2),
        None,
        None,
        Some(cfl_subsample_hbd_422_4x16_ssse3),
        Some(cfl_subsample_hbd_422_16x4_ssse3),
        Some(cfl_subsample_hbd_422_8x32_ssse3),
        Some(cfl_subsample_hbd_422_32x8_avx2),
        None,
        None,
    ];
    return subfn_422[tx_size as usize];
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn cfl_luma_subsampling_444_hbd_avx2(
    mut input: *const u16,
    input_stride: i32,
    pred_buf_q3: *mut u16,
    width: i32,
    height: i32,
) {
    let _ = width; // Forever 32
    let mut row: *mut __m256i = pred_buf_q3 as *mut __m256i;
    let mut row_end: *const __m256i =
        row.offset((height * CFL_BUF_LINE_I256) as isize) as *const __m256i;
    loop {
        let mut top: __m256i = _mm256_loadu_si256(input as *mut __m256i as *const __m256i);
        let mut top_1: __m256i =
            _mm256_loadu_si256(input.offset(16) as *mut __m256i as *const __m256i);
        _mm256_storeu_si256(row, _mm256_slli_epi16::<{ 3 }>(top));
        _mm256_storeu_si256(row.offset(1), _mm256_slli_epi16::<{ 3 }>(top_1));
        input = input.offset(input_stride as isize);
        row = row.offset(CFL_BUF_LINE_I256 as isize);
        if !(row < row_end as *mut __m256i) {
            break;
        }
    }
}

#[target_feature(enable = "avx2")]
pub unsafe extern "C-unwind" fn cfl_subsample_hbd_444_32x32_avx2(
    cfl_type: *const u16,
    input_stride: i32,
    output_q3: *mut u16,
) {
    cfl_luma_subsampling_444_hbd_avx2(cfl_type, input_stride, output_q3, 32, 32);
}

#[target_feature(enable = "avx2")]
pub unsafe extern "C-unwind" fn cfl_subsample_hbd_444_32x16_avx2(
    cfl_type: *const u16,
    input_stride: i32,
    output_q3: *mut u16,
) {
    cfl_luma_subsampling_444_hbd_avx2(cfl_type, input_stride, output_q3, 32, 16);
}

#[target_feature(enable = "avx2")]
pub unsafe extern "C-unwind" fn cfl_subsample_hbd_444_32x8_avx2(
    cfl_type: *const u16,
    input_stride: i32,
    output_q3: *mut u16,
) {
    cfl_luma_subsampling_444_hbd_avx2(cfl_type, input_stride, output_q3, 32, 8);
}

#[no_mangle]
#[target_feature(enable = "avx2")]
pub unsafe extern "C-unwind" fn cfl_get_luma_subsampling_444_hbd_avx2(
    tx_size: u8,
) -> cfl_subsample_hbd_fn {
    let mut subfn_444: [cfl_subsample_hbd_fn; (TX_SIZES_ALL) as usize] = [
        Some(cfl_subsample_hbd_444_4x4_ssse3),
        Some(cfl_subsample_hbd_444_8x8_ssse3),
        Some(cfl_subsample_hbd_444_16x16_ssse3),
        Some(cfl_subsample_hbd_444_32x32_avx2),
        None,
        Some(cfl_subsample_hbd_444_4x8_ssse3),
        Some(cfl_subsample_hbd_444_8x4_ssse3),
        Some(cfl_subsample_hbd_444_8x16_ssse3),
        Some(cfl_subsample_hbd_444_16x8_ssse3),
        Some(cfl_subsample_hbd_444_16x32_ssse3),
        Some(cfl_subsample_hbd_444_32x16_avx2),
        None,
        None,
        Some(cfl_subsample_hbd_444_4x16_ssse3),
        Some(cfl_subsample_hbd_444_16x4_ssse3),
        Some(cfl_subsample_hbd_444_8x32_ssse3),
        Some(cfl_subsample_hbd_444_32x8_avx2),
        None,
        None,
    ];
    return subfn_444[tx_size as usize];
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn predict_unclipped(
    input: *const __m256i,
    alpha_q12: __m256i,
    alpha_sign: __m256i,
    dc_q0: __m256i,
) -> __m256i {
    let mut ac_q3: __m256i = _mm256_loadu_si256(input);
    let mut ac_sign: __m256i = _mm256_sign_epi16(alpha_sign, ac_q3);
    let mut scaled_luma_q0: __m256i = _mm256_mulhrs_epi16(_mm256_abs_epi16(ac_q3), alpha_q12);
    scaled_luma_q0 = _mm256_sign_epi16(scaled_luma_q0, ac_sign);
    return _mm256_add_epi16(scaled_luma_q0, dc_q0);
}

#[inline]
#[target_feature(enable = "avx2")]
pub unsafe extern "C-unwind" fn cfl_predict_lbd_avx2(
    pred_buf_q3: *const i16,
    mut dst: *mut u8,
    dst_stride: i32,
    alpha_q3: i32,
    width: i32,
    height: i32,
) {
    let _ = width;
    let alpha_sign: __m256i = _mm256_set1_epi16(alpha_q3 as i16);
    let alpha_q12: __m256i = _mm256_slli_epi16::<{ 9 }>(_mm256_abs_epi16(alpha_sign));
    let dc_q0: __m256i = _mm256_set1_epi16(*dst as i16);
    let mut row: *mut __m256i = pred_buf_q3 as *mut __m256i;
    let mut row_end: *const __m256i =
        row.offset((height * CFL_BUF_LINE_I256) as isize) as *const __m256i;
    loop {
        let mut res: __m256i =
            predict_unclipped(row as *const __m256i, alpha_q12, alpha_sign, dc_q0);
        let mut next: __m256i = predict_unclipped(
            row.offset(1) as *const __m256i,
            alpha_q12,
            alpha_sign,
            dc_q0,
        );
        res = _mm256_packus_epi16(res, next);
        res = _mm256_permute4x64_epi64::<{ ((3) << 6) | ((1) << 4) | ((2) << 2) | (0) }>(res);
        _mm256_storeu_si256(dst as *mut __m256i, res);
        dst = dst.offset(dst_stride as isize);
        row = row.offset(CFL_BUF_LINE_I256 as isize);
        if !(row < row_end as *mut __m256i) {
            break;
        }
    }
}

#[target_feature(enable = "avx2")]
pub unsafe extern "C-unwind" fn cfl_predict_lbd_32x8_avx2(
    pred_buf_q3: *const i16,
    dst: *mut u8,
    dst_stride: i32,
    alpha_q3: i32,
) {
    cfl_predict_lbd_avx2(pred_buf_q3, dst, dst_stride, alpha_q3, 32, 8);
}

#[target_feature(enable = "avx2")]
pub unsafe extern "C-unwind" fn cfl_predict_lbd_32x16_avx2(
    pred_buf_q3: *const i16,
    dst: *mut u8,
    dst_stride: i32,
    alpha_q3: i32,
) {
    cfl_predict_lbd_avx2(pred_buf_q3, dst, dst_stride, alpha_q3, 32, 16);
}

#[target_feature(enable = "avx2")]
pub unsafe extern "C-unwind" fn cfl_predict_lbd_32x32_avx2(
    pred_buf_q3: *const i16,
    dst: *mut u8,
    dst_stride: i32,
    alpha_q3: i32,
) {
    cfl_predict_lbd_avx2(pred_buf_q3, dst, dst_stride, alpha_q3, 32, 32);
}

#[no_mangle]
#[target_feature(enable = "avx2")]
pub unsafe extern "C-unwind" fn cfl_get_predict_lbd_fn_avx2(tx_size: u8) -> cfl_predict_lbd_fn {
    let mut pred: [cfl_predict_lbd_fn; (TX_SIZES_ALL) as usize] = [
        Some(cfl_predict_lbd_4x4_ssse3),
        Some(cfl_predict_lbd_8x8_ssse3),
        Some(cfl_predict_lbd_16x16_ssse3),
        Some(cfl_predict_lbd_32x32_avx2),
        None,
        Some(cfl_predict_lbd_4x8_ssse3),
        Some(cfl_predict_lbd_8x4_ssse3),
        Some(cfl_predict_lbd_8x16_ssse3),
        Some(cfl_predict_lbd_16x8_ssse3),
        Some(cfl_predict_lbd_16x32_ssse3),
        Some(cfl_predict_lbd_32x16_avx2),
        None,
        None,
        Some(cfl_predict_lbd_4x16_ssse3),
        Some(cfl_predict_lbd_16x4_ssse3),
        Some(cfl_predict_lbd_8x32_ssse3),
        Some(cfl_predict_lbd_32x8_avx2),
        None,
        None,
    ];
    // 4x4
    // 8x8
    // 16x16
    // 32x32
    // 64x64 (invalid CFL size)
    // 4x8
    // 8x4
    // 8x16
    // 16x8
    // 16x32
    // 32x16
    // 32x64 (invalid CFL size)
    // 64x32 (invalid CFL size)
    // 4x16
    // 16x4
    // 8x32
    // 32x8
    // 16x64 (invalid CFL size)
    // 64x16 (invalid CFL size)
    // Modulo TX_SIZES_ALL to ensure that an attacker won't be able to index the
    // function pointer array out of bounds.
    return pred[(tx_size as i32 % TX_SIZES_ALL) as usize];
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn highbd_max_epi16(bd: i32) -> __m256i {
    let neg_one: __m256i = _mm256_set1_epi16(-1);
    // (1 << bd) - 1 => -(-1 << bd) -1 => -1 - (-1 << bd) => -1 ^ (-1 << bd)
    return _mm256_xor_si256(_mm256_sll_epi16(neg_one, _mm_cvtsi32_si128(bd)), neg_one);
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn highbd_clamp_epi16(u: __m256i, zero: __m256i, max: __m256i) -> __m256i {
    return _mm256_max_epi16(_mm256_min_epi16(u, max), zero);
}

#[inline]
#[target_feature(enable = "avx2")]
pub unsafe extern "C-unwind" fn cfl_predict_hbd_avx2(
    pred_buf_q3: *const i16,
    mut dst: *mut u16,
    dst_stride: i32,
    alpha_q3: i32,
    bd: i32,
    width: i32,
    height: i32,
) {
    // Use SSSE3 version for smaller widths
    debug_assert!(width == 16 || width == 32);
    let alpha_sign: __m256i = _mm256_set1_epi16(alpha_q3 as i16);
    let alpha_q12: __m256i = _mm256_slli_epi16::<{ 9 }>(_mm256_abs_epi16(alpha_sign));
    let dc_q0: __m256i = _mm256_loadu_si256(dst as *mut __m256i as *const __m256i);
    let max: __m256i = highbd_max_epi16(bd);
    let mut row: *mut __m256i = pred_buf_q3 as *mut __m256i;
    let mut row_end: *const __m256i =
        row.offset((height * CFL_BUF_LINE_I256) as isize) as *const __m256i;
    loop {
        let res: __m256i = predict_unclipped(row as *const __m256i, alpha_q12, alpha_sign, dc_q0);
        _mm256_storeu_si256(
            dst as *mut __m256i,
            highbd_clamp_epi16(res, _mm256_setzero_si256(), max),
        );
        if width == 32 {
            let res_1: __m256i = predict_unclipped(
                row.offset(1) as *const __m256i,
                alpha_q12,
                alpha_sign,
                dc_q0,
            );
            _mm256_storeu_si256(
                dst.offset(16) as *mut __m256i,
                highbd_clamp_epi16(res_1, _mm256_setzero_si256(), max),
            );
        }
        dst = dst.offset(dst_stride as isize);
        row = row.offset(CFL_BUF_LINE_I256 as isize);
        if !(row < row_end as *mut __m256i) {
            break;
        }
    }
}

#[target_feature(enable = "avx2")]
pub unsafe extern "C-unwind" fn cfl_predict_hbd_16x4_avx2(
    pred_buf_q3: *const i16,
    dst: *mut u16,
    dst_stride: i32,
    alpha_q3: i32,
    bd: i32,
) {
    cfl_predict_hbd_avx2(pred_buf_q3, dst, dst_stride, alpha_q3, bd, 16, 4);
}

#[target_feature(enable = "avx2")]
pub unsafe extern "C-unwind" fn cfl_predict_hbd_16x8_avx2(
    pred_buf_q3: *const i16,
    dst: *mut u16,
    dst_stride: i32,
    alpha_q3: i32,
    bd: i32,
) {
    cfl_predict_hbd_avx2(pred_buf_q3, dst, dst_stride, alpha_q3, bd, 16, 8);
}

#[target_feature(enable = "avx2")]
pub unsafe extern "C-unwind" fn cfl_predict_hbd_16x16_avx2(
    pred_buf_q3: *const i16,
    dst: *mut u16,
    dst_stride: i32,
    alpha_q3: i32,
    bd: i32,
) {
    cfl_predict_hbd_avx2(pred_buf_q3, dst, dst_stride, alpha_q3, bd, 16, 16);
}

#[target_feature(enable = "avx2")]
pub unsafe extern "C-unwind" fn cfl_predict_hbd_16x32_avx2(
    pred_buf_q3: *const i16,
    dst: *mut u16,
    dst_stride: i32,
    alpha_q3: i32,
    bd: i32,
) {
    cfl_predict_hbd_avx2(pred_buf_q3, dst, dst_stride, alpha_q3, bd, 16, 32);
}

#[target_feature(enable = "avx2")]
pub unsafe extern "C-unwind" fn cfl_predict_hbd_32x8_avx2(
    pred_buf_q3: *const i16,
    dst: *mut u16,
    dst_stride: i32,
    alpha_q3: i32,
    bd: i32,
) {
    cfl_predict_hbd_avx2(pred_buf_q3, dst, dst_stride, alpha_q3, bd, 32, 8);
}

#[target_feature(enable = "avx2")]
pub unsafe extern "C-unwind" fn cfl_predict_hbd_32x16_avx2(
    pred_buf_q3: *const i16,
    dst: *mut u16,
    dst_stride: i32,
    alpha_q3: i32,
    bd: i32,
) {
    cfl_predict_hbd_avx2(pred_buf_q3, dst, dst_stride, alpha_q3, bd, 32, 16);
}

#[target_feature(enable = "avx2")]
pub unsafe extern "C-unwind" fn cfl_predict_hbd_32x32_avx2(
    pred_buf_q3: *const i16,
    dst: *mut u16,
    dst_stride: i32,
    alpha_q3: i32,
    bd: i32,
) {
    cfl_predict_hbd_avx2(pred_buf_q3, dst, dst_stride, alpha_q3, bd, 32, 32);
}

#[no_mangle]
#[target_feature(enable = "avx2")]
pub unsafe extern "C-unwind" fn cfl_get_predict_hbd_fn_avx2(tx_size: u8) -> cfl_predict_hbd_fn {
    let mut pred: [cfl_predict_hbd_fn; (TX_SIZES_ALL) as usize] = [
        Some(cfl_predict_hbd_4x4_ssse3),
        Some(cfl_predict_hbd_8x8_ssse3),
        Some(cfl_predict_hbd_16x16_avx2),
        Some(cfl_predict_hbd_32x32_avx2),
        None,
        Some(cfl_predict_hbd_4x8_ssse3),
        Some(cfl_predict_hbd_8x4_ssse3),
        Some(cfl_predict_hbd_8x16_ssse3),
        Some(cfl_predict_hbd_16x8_avx2),
        Some(cfl_predict_hbd_16x32_avx2),
        Some(cfl_predict_hbd_32x16_avx2),
        None,
        None,
        Some(cfl_predict_hbd_4x16_ssse3),
        Some(cfl_predict_hbd_16x4_avx2),
        Some(cfl_predict_hbd_8x32_ssse3),
        Some(cfl_predict_hbd_32x8_avx2),
        None,
        None,
    ];
    // 4x4
    // 8x8
    // 16x16
    // 32x32
    // 64x64 (invalid CFL size)
    // 4x8
    // 8x4
    // 8x16
    // 16x8
    // 16x32
    // 32x16
    // 32x64 (invalid CFL size)
    // 64x32 (invalid CFL size)
    // 4x16
    // 16x4
    // 8x32
    // 32x8
    // 16x64 (invalid CFL size)
    // 64x16 (invalid CFL size)
    // Modulo TX_SIZES_ALL to ensure that an attacker won't be able to index the
    // function pointer array out of bounds.
    return pred[(tx_size as i32 % TX_SIZES_ALL) as usize];
}

// Returns a vector where all the (32-bits) elements are the sum of all the
// lanes in a.
#[inline]
#[target_feature(enable = "avx2")]
unsafe fn fill_sum_epi32(mut a: __m256i) -> __m256i {
    // Given that a == [A, B, C, D, E, F, G, H]
    a = _mm256_hadd_epi32(a, a);
    // Given that A' == A + B, C' == C + D, E' == E + F, G' == G + H
    // a == [A', C', A', C', E', G', E', G']
    a = _mm256_permute4x64_epi64::<{ ((3) << 6) | ((1) << 4) | ((2) << 2) | (0) }>(a);
    // a == [A', C', E', G', A', C', E', G']
    a = _mm256_hadd_epi32(a, a);
    // Given that A'' == A' + C' and E'' == E' + G'
    // a == [A'', E'', A'', E'', A'', E'', A'', E'']
    return _mm256_hadd_epi32(a, a);
}

// Given that A''' == A'' + E''
// a == [A''', A''', A''', A''', A''', A''', A''', A''']
#[inline]
#[target_feature(enable = "avx2")]
unsafe fn _mm256_addl_epi16(a: __m256i) -> __m256i {
    return _mm256_add_epi32(
        _mm256_unpacklo_epi16(a, _mm256_setzero_si256()),
        _mm256_unpackhi_epi16(a, _mm256_setzero_si256()),
    );
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn subtract_average_avx2(
    src_ptr: *const u16,
    dst_ptr: *mut i16,
    width: i32,
    height: i32,
    round_offset: i32,
    num_pel_log2: i32,
) {
    // Use SSE2 version for smaller widths
    debug_assert!(width == 16 || width == 32);
    let mut src: *const __m256i = src_ptr as *mut __m256i as *const __m256i;
    let mut end: *const __m256i = src.offset((height * CFL_BUF_LINE_I256) as isize);
    // To maximize usage of the AVX2 registers, we sum two rows per loop
    // iteration
    let step: i32 = 2 * CFL_BUF_LINE_I256;
    let mut sum: __m256i = _mm256_setzero_si256();
    // For width 32, we use a second sum accumulator to reduce accumulator
    // dependencies in the loop.
    let mut sum2: __m256i = core::mem::zeroed();
    if width == 32 {
        sum2 = _mm256_setzero_si256();
    }
    loop {
        // Add top row to the bottom row
        let mut l0: __m256i = _mm256_add_epi16(
            _mm256_loadu_si256(src),
            _mm256_loadu_si256(src.offset(CFL_BUF_LINE_I256 as isize)),
        );
        sum = _mm256_add_epi32(sum, _mm256_addl_epi16(l0));
        if width == 32 {
            // Don't worry, this if it gets optimized out.
            // Add the second part of the top row to the second part of the bottom row
            let mut l1: __m256i = _mm256_add_epi16(
                _mm256_loadu_si256(src.offset(1)),
                _mm256_loadu_si256(src.offset(1).offset(CFL_BUF_LINE_I256 as isize)),
            );
            sum2 = _mm256_add_epi32(sum2, _mm256_addl_epi16(l1));
        }
        src = src.offset(step as isize);
        if !(src < end) {
            break;
        }
    }
    // Combine both sum accumulators
    if width == 32 {
        sum = _mm256_add_epi32(sum, sum2);
    }
    let mut fill: __m256i = fill_sum_epi32(sum);
    let mut avg_epi16: __m256i = _mm256_srl_epi32(
        _mm256_add_epi32(fill, _mm256_set1_epi32(round_offset)),
        _mm_cvtsi32_si128(num_pel_log2),
    );
    avg_epi16 = _mm256_packs_epi32(avg_epi16, avg_epi16);
    // Store and subtract loop
    src = src_ptr as *mut __m256i as *const __m256i;
    let mut dst: *mut __m256i = dst_ptr as *mut __m256i;
    loop {
        _mm256_storeu_si256(dst, _mm256_sub_epi16(_mm256_loadu_si256(src), avg_epi16));
        if width == 32 {
            _mm256_storeu_si256(
                dst.offset(1),
                _mm256_sub_epi16(_mm256_loadu_si256(src.offset(1)), avg_epi16),
            );
        }
        src = src.offset(CFL_BUF_LINE_I256 as isize);
        dst = dst.offset(CFL_BUF_LINE_I256 as isize);
        if !(src < end) {
            break;
        }
    }
}

// Declare wrappers for AVX2 sizes
#[target_feature(enable = "avx2")]
pub unsafe extern "C-unwind" fn cfl_subtract_average_16x4_avx2(src: *const u16, dst: *mut i16) {
    subtract_average_avx2(src, dst, 16, 4, 32, 6);
}

#[target_feature(enable = "avx2")]
pub unsafe extern "C-unwind" fn cfl_subtract_average_16x8_avx2(src: *const u16, dst: *mut i16) {
    subtract_average_avx2(src, dst, 16, 8, 64, 7);
}

#[target_feature(enable = "avx2")]
pub unsafe extern "C-unwind" fn cfl_subtract_average_16x16_avx2(src: *const u16, dst: *mut i16) {
    subtract_average_avx2(src, dst, 16, 16, 128, 8);
}

#[target_feature(enable = "avx2")]
pub unsafe extern "C-unwind" fn cfl_subtract_average_16x32_avx2(src: *const u16, dst: *mut i16) {
    subtract_average_avx2(src, dst, 16, 32, 256, 9);
}

#[target_feature(enable = "avx2")]
pub unsafe extern "C-unwind" fn cfl_subtract_average_32x8_avx2(src: *const u16, dst: *mut i16) {
    subtract_average_avx2(src, dst, 32, 8, 128, 8);
}

#[target_feature(enable = "avx2")]
pub unsafe extern "C-unwind" fn cfl_subtract_average_32x16_avx2(src: *const u16, dst: *mut i16) {
    subtract_average_avx2(src, dst, 32, 16, 256, 9);
}

#[target_feature(enable = "avx2")]
pub unsafe extern "C-unwind" fn cfl_subtract_average_32x32_avx2(src: *const u16, dst: *mut i16) {
    subtract_average_avx2(src, dst, 32, 32, 512, 10);
}

// Based on the observation that for small blocks AVX2 does not outperform
// SSE2, we call the SSE2 code for block widths 4 and 8.
#[no_mangle]
#[target_feature(enable = "avx2")]
pub unsafe extern "C-unwind" fn cfl_get_subtract_average_fn_avx2(
    tx_size: u8,
) -> cfl_subtract_average_fn {
    let mut sub_avg: [cfl_subtract_average_fn; (TX_SIZES_ALL) as usize] = [
        Some(cfl_subtract_average_4x4_sse2),
        Some(cfl_subtract_average_8x8_sse2),
        Some(cfl_subtract_average_16x16_avx2),
        Some(cfl_subtract_average_32x32_avx2),
        None,
        Some(cfl_subtract_average_4x8_sse2),
        Some(cfl_subtract_average_8x4_sse2),
        Some(cfl_subtract_average_8x16_sse2),
        Some(cfl_subtract_average_16x8_avx2),
        Some(cfl_subtract_average_16x32_avx2),
        Some(cfl_subtract_average_32x16_avx2),
        None,
        None,
        Some(cfl_subtract_average_4x16_sse2),
        Some(cfl_subtract_average_16x4_avx2),
        Some(cfl_subtract_average_8x32_sse2),
        Some(cfl_subtract_average_32x8_avx2),
        None,
        None,
    ];
    // 4x4
    // 8x8
    // 16x16
    // 32x32
    // 64x64 (invalid CFL size)
    // 4x8
    // 8x4
    // 8x16
    // 16x8
    // 16x32
    // 32x16
    // 32x64 (invalid CFL size)
    // 64x32 (invalid CFL size)
    // 4x16
    // 16x4
    // 8x32
    // 32x8
    // 16x64 (invalid CFL size)
    // 64x16 (invalid CFL size)
    // Modulo TX_SIZES_ALL to ensure that an attacker won't be able to
    // index the function pointer array out of bounds.
    return sub_avg[(tx_size as i32 % TX_SIZES_ALL) as usize];
}
