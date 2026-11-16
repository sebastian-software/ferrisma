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

// Translated from upstream/aom/av1/common/x86/cfl_ssse3.c (ADR 0056, ADR 0058).
// Upstream compiles it with -mssse3. The functions were
// transliterated with a typing-aware C-subset printer (not C2Rust) and
// reviewed against the C function by function: the same sequence of intrinsic
// and helper calls, constants and operators in order (ADR 0058).
// The size wrappers and RTCD getters that the CFL_* macros of av1/common/cfl.h generate are
// expanded with the C preprocessor before printing (tools/x86_c2r/c2r.py `cpp`); the
// wrappers keep the C ABI because the getters return them as C function pointers.
//
// Translation notes:
// - Every function carries `ssse3`, the flag upstream compiles the file with.
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
#[target_feature(enable = "ssse3")]
unsafe fn _mm_loadh_epi32(mem_addr: *const __m128i) -> __m128i {
    return _mm_cvtsi32_si128(core::ptr::read_unaligned(mem_addr as *mut i32));
}

// Store 32-bit integer from the first element of a into memory.
#[inline]
#[target_feature(enable = "ssse3")]
unsafe fn _mm_storeh_epi32(mem_addr: *const __m128i, a: __m128i) {
    core::ptr::write_unaligned(mem_addr as *mut i32, _mm_cvtsi128_si32(a));
}

// Adds 4 pixels (in a 2x2 grid) and multiplies them by 2. Resulting in a more
// precise version of a box filter 4:2:0 pixel subsampling in Q3.
//
// The CfL prediction buffer is always of size CFL_BUF_SQUARE. However, the
// active area is specified using width and height.
//
// Note: We don't need to worry about going over the active area, as long as we
// stay inside the CfL prediction buffer.
#[inline]
#[target_feature(enable = "ssse3")]
unsafe fn cfl_luma_subsampling_420_lbd_ssse3(
    mut input: *const u8,
    input_stride: i32,
    pred_buf_q3: *mut u16,
    width: i32,
    height: i32,
) {
    let twos: __m128i = _mm_set1_epi8(2);
    let mut pred_buf_m128i: *mut __m128i = pred_buf_q3 as *mut __m128i;
    let mut end: *const __m128i =
        pred_buf_m128i.offset(((height >> 1) * CFL_BUF_LINE_I128) as isize) as *const __m128i;
    let luma_stride: i32 = input_stride << 1;
    loop {
        if width == 4 {
            let mut top: __m128i = _mm_loadh_epi32(input as *mut __m128i as *const __m128i);
            top = _mm_maddubs_epi16(top, twos);
            let mut bot: __m128i = _mm_loadh_epi32(input.offset(input_stride as isize)
                as *mut __m128i
                as *const __m128i);
            bot = _mm_maddubs_epi16(bot, twos);
            let sum: __m128i = _mm_add_epi16(top, bot);
            _mm_storeh_epi32(pred_buf_m128i as *const __m128i, sum);
        } else if width == 8 {
            let mut top: __m128i = _mm_loadl_epi64(input as *mut __m128i as *const __m128i);
            top = _mm_maddubs_epi16(top, twos);
            let mut bot: __m128i = _mm_loadl_epi64(input.offset(input_stride as isize)
                as *mut __m128i
                as *const __m128i);
            bot = _mm_maddubs_epi16(bot, twos);
            let sum: __m128i = _mm_add_epi16(top, bot);
            _mm_storel_epi64(pred_buf_m128i, sum);
        } else {
            let mut top: __m128i = _mm_loadu_si128(input as *mut __m128i as *const __m128i);
            top = _mm_maddubs_epi16(top, twos);
            let mut bot: __m128i = _mm_loadu_si128(input.offset(input_stride as isize)
                as *mut __m128i
                as *const __m128i);
            bot = _mm_maddubs_epi16(bot, twos);
            let sum: __m128i = _mm_add_epi16(top, bot);
            _mm_storeu_si128(pred_buf_m128i, sum);
            if width == 32 {
                let mut top_1: __m128i =
                    _mm_loadu_si128((input as *mut __m128i).offset(1) as *const __m128i);
                let mut bot_1: __m128i = _mm_loadu_si128(
                    (input.offset(input_stride as isize) as *mut __m128i).offset(1)
                        as *const __m128i,
                );
                top_1 = _mm_maddubs_epi16(top_1, twos);
                bot_1 = _mm_maddubs_epi16(bot_1, twos);
                let mut sum_1: __m128i = _mm_add_epi16(top_1, bot_1);
                _mm_storeu_si128(pred_buf_m128i.offset(1), sum_1);
            }
        }
        input = input.offset(luma_stride as isize);
        pred_buf_m128i = pred_buf_m128i.offset(CFL_BUF_LINE_I128 as isize);
        if !(pred_buf_m128i < end as *mut __m128i) {
            break;
        }
    }
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
#[target_feature(enable = "ssse3")]
unsafe fn cfl_luma_subsampling_422_lbd_ssse3(
    mut input: *const u8,
    input_stride: i32,
    pred_buf_q3: *mut u16,
    width: i32,
    height: i32,
) {
    let fours: __m128i = _mm_set1_epi8(4);
    let mut pred_buf_m128i: *mut __m128i = pred_buf_q3 as *mut __m128i;
    let mut end: *const __m128i =
        pred_buf_m128i.offset((height * CFL_BUF_LINE_I128) as isize) as *const __m128i;
    loop {
        if width == 4 {
            let mut top: __m128i = _mm_loadh_epi32(input as *mut __m128i as *const __m128i);
            top = _mm_maddubs_epi16(top, fours);
            _mm_storeh_epi32(pred_buf_m128i as *const __m128i, top);
        } else if width == 8 {
            let mut top: __m128i = _mm_loadl_epi64(input as *mut __m128i as *const __m128i);
            top = _mm_maddubs_epi16(top, fours);
            _mm_storel_epi64(pred_buf_m128i, top);
        } else {
            let mut top: __m128i = _mm_loadu_si128(input as *mut __m128i as *const __m128i);
            top = _mm_maddubs_epi16(top, fours);
            _mm_storeu_si128(pred_buf_m128i, top);
            if width == 32 {
                let mut top_1: __m128i =
                    _mm_loadu_si128((input as *mut __m128i).offset(1) as *const __m128i);
                top_1 = _mm_maddubs_epi16(top_1, fours);
                _mm_storeu_si128(pred_buf_m128i.offset(1), top_1);
            }
        }
        input = input.offset(input_stride as isize);
        pred_buf_m128i = pred_buf_m128i.offset(CFL_BUF_LINE_I128 as isize);
        if !(pred_buf_m128i < end as *mut __m128i) {
            break;
        }
    }
}

// Multiplies the pixels by 8 (scaling in Q3).
//
// The CfL prediction buffer is always of size CFL_BUF_SQUARE. However, the
// active area is specified using width and height.
//
// Note: We don't need to worry about going over the active area, as long as we
// stay inside the CfL prediction buffer.
#[inline]
#[target_feature(enable = "ssse3")]
unsafe fn cfl_luma_subsampling_444_lbd_ssse3(
    mut input: *const u8,
    input_stride: i32,
    pred_buf_q3: *mut u16,
    width: i32,
    height: i32,
) {
    let zeros: __m128i = _mm_setzero_si128();
    let luma_stride: i32 = input_stride;
    let mut pred_buf_m128i: *mut __m128i = pred_buf_q3 as *mut __m128i;
    let mut end: *const __m128i =
        pred_buf_m128i.offset((height * CFL_BUF_LINE_I128) as isize) as *const __m128i;
    loop {
        if width == 4 {
            let mut row: __m128i = _mm_loadh_epi32(input as *mut __m128i as *const __m128i);
            row = _mm_unpacklo_epi8(row, zeros);
            _mm_storel_epi64(pred_buf_m128i, _mm_slli_epi16::<{ 3 }>(row));
        } else if width == 8 {
            let mut row: __m128i = _mm_loadl_epi64(input as *mut __m128i as *const __m128i);
            row = _mm_unpacklo_epi8(row, zeros);
            _mm_storeu_si128(pred_buf_m128i, _mm_slli_epi16::<{ 3 }>(row));
        } else {
            let mut row: __m128i = _mm_loadu_si128(input as *mut __m128i as *const __m128i);
            let row_lo: __m128i = _mm_unpacklo_epi8(row, zeros);
            let row_hi: __m128i = _mm_unpackhi_epi8(row, zeros);
            _mm_storeu_si128(pred_buf_m128i, _mm_slli_epi16::<{ 3 }>(row_lo));
            _mm_storeu_si128(pred_buf_m128i.offset(1), _mm_slli_epi16::<{ 3 }>(row_hi));
            if width == 32 {
                let mut row_1: __m128i =
                    _mm_loadu_si128((input as *mut __m128i).offset(1) as *const __m128i);
                let row_1_lo: __m128i = _mm_unpacklo_epi8(row_1, zeros);
                let row_1_hi: __m128i = _mm_unpackhi_epi8(row_1, zeros);
                _mm_storeu_si128(pred_buf_m128i.offset(2), _mm_slli_epi16::<{ 3 }>(row_1_lo));
                _mm_storeu_si128(pred_buf_m128i.offset(3), _mm_slli_epi16::<{ 3 }>(row_1_hi));
            }
        }
        input = input.offset(luma_stride as isize);
        pred_buf_m128i = pred_buf_m128i.offset(CFL_BUF_LINE_I128 as isize);
        if !(pred_buf_m128i < end as *mut __m128i) {
            break;
        }
    }
}

// Adds 4 pixels (in a 2x2 grid) and multiplies them by 2. Resulting in a more
// precise version of a box filter 4:2:0 pixel subsampling in Q3.
//
// The CfL prediction buffer is always of size CFL_BUF_SQUARE. However, the
// active area is specified using width and height.
//
// Note: We don't need to worry about going over the active area, as long as we
// stay inside the CfL prediction buffer.
#[inline]
#[target_feature(enable = "ssse3")]
unsafe fn cfl_luma_subsampling_420_hbd_ssse3(
    mut input: *const u16,
    input_stride: i32,
    mut pred_buf_q3: *mut u16,
    width: i32,
    height: i32,
) {
    let mut end: *const u16 =
        pred_buf_q3.offset(((height >> 1) * CFL_BUF_LINE) as isize) as *const u16;
    let luma_stride: i32 = input_stride << 1;
    loop {
        if width == 4 {
            let top: __m128i = _mm_loadl_epi64(input as *mut __m128i as *const __m128i);
            let bot: __m128i = _mm_loadl_epi64(
                input.offset(input_stride as isize) as *mut __m128i as *const __m128i
            );
            let mut sum: __m128i = _mm_add_epi16(top, bot);
            sum = _mm_hadd_epi16(sum, sum);
            core::ptr::write_unaligned(
                pred_buf_q3 as *mut i32,
                _mm_cvtsi128_si32(_mm_add_epi16(sum, sum)),
            );
        } else {
            let top: __m128i = _mm_loadu_si128(input as *mut __m128i as *const __m128i);
            let bot: __m128i = _mm_loadu_si128(
                input.offset(input_stride as isize) as *mut __m128i as *const __m128i
            );
            let mut sum: __m128i = _mm_add_epi16(top, bot);
            if width == 8 {
                sum = _mm_hadd_epi16(sum, sum);
                _mm_storel_epi64(pred_buf_q3 as *mut __m128i, _mm_add_epi16(sum, sum));
            } else {
                let top_1: __m128i =
                    _mm_loadu_si128((input as *mut __m128i).offset(1) as *const __m128i);
                let bot_1: __m128i = _mm_loadu_si128(
                    (input.offset(input_stride as isize) as *mut __m128i).offset(1)
                        as *const __m128i,
                );
                sum = _mm_hadd_epi16(sum, _mm_add_epi16(top_1, bot_1));
                _mm_storeu_si128(pred_buf_q3 as *mut __m128i, _mm_add_epi16(sum, sum));
                if width == 32 {
                    let top_2: __m128i =
                        _mm_loadu_si128((input as *mut __m128i).offset(2) as *const __m128i);
                    let bot_2: __m128i = _mm_loadu_si128(
                        (input.offset(input_stride as isize) as *mut __m128i).offset(2)
                            as *const __m128i,
                    );
                    let top_3: __m128i =
                        _mm_loadu_si128((input as *mut __m128i).offset(3) as *const __m128i);
                    let bot_3: __m128i = _mm_loadu_si128(
                        (input.offset(input_stride as isize) as *mut __m128i).offset(3)
                            as *const __m128i,
                    );
                    let sum_2: __m128i = _mm_add_epi16(top_2, bot_2);
                    let sum_3: __m128i = _mm_add_epi16(top_3, bot_3);
                    let mut next_sum: __m128i = _mm_hadd_epi16(sum_2, sum_3);
                    _mm_storeu_si128(
                        (pred_buf_q3 as *mut __m128i).offset(1),
                        _mm_add_epi16(next_sum, next_sum),
                    );
                }
            }
        }
        input = input.offset(luma_stride as isize);
        pred_buf_q3 = pred_buf_q3.offset(CFL_BUF_LINE as isize);
        if !(pred_buf_q3 < end as *mut u16) {
            break;
        }
    }
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
#[target_feature(enable = "ssse3")]
unsafe fn cfl_luma_subsampling_422_hbd_ssse3(
    mut input: *const u16,
    input_stride: i32,
    pred_buf_q3: *mut u16,
    width: i32,
    height: i32,
) {
    let mut pred_buf_m128i: *mut __m128i = pred_buf_q3 as *mut __m128i;
    let mut end: *const __m128i =
        pred_buf_m128i.offset((height * CFL_BUF_LINE_I128) as isize) as *const __m128i;
    loop {
        if width == 4 {
            let top: __m128i = _mm_loadl_epi64(input as *mut __m128i as *const __m128i);
            let sum: __m128i = _mm_slli_epi16::<{ 2 }>(_mm_hadd_epi16(top, top));
            _mm_storeh_epi32(pred_buf_m128i as *const __m128i, sum);
        } else {
            let top: __m128i = _mm_loadu_si128(input as *mut __m128i as *const __m128i);
            if width == 8 {
                let sum: __m128i = _mm_slli_epi16::<{ 2 }>(_mm_hadd_epi16(top, top));
                _mm_storel_epi64(pred_buf_m128i, sum);
            } else {
                let top_1: __m128i =
                    _mm_loadu_si128((input as *mut __m128i).offset(1) as *const __m128i);
                let sum: __m128i = _mm_slli_epi16::<{ 2 }>(_mm_hadd_epi16(top, top_1));
                _mm_storeu_si128(pred_buf_m128i, sum);
                if width == 32 {
                    let top_2: __m128i =
                        _mm_loadu_si128((input as *mut __m128i).offset(2) as *const __m128i);
                    let top_3: __m128i =
                        _mm_loadu_si128((input as *mut __m128i).offset(3) as *const __m128i);
                    let sum_1: __m128i = _mm_slli_epi16::<{ 2 }>(_mm_hadd_epi16(top_2, top_3));
                    _mm_storeu_si128(pred_buf_m128i.offset(1), sum_1);
                }
            }
        }
        pred_buf_m128i = pred_buf_m128i.offset(CFL_BUF_LINE_I128 as isize);
        input = input.offset(input_stride as isize);
        if !(pred_buf_m128i < end as *mut __m128i) {
            break;
        }
    }
}

#[inline]
#[target_feature(enable = "ssse3")]
unsafe fn cfl_luma_subsampling_444_hbd_ssse3(
    mut input: *const u16,
    input_stride: i32,
    mut pred_buf_q3: *mut u16,
    width: i32,
    height: i32,
) {
    let mut end: *const u16 = pred_buf_q3.offset((height * CFL_BUF_LINE) as isize) as *const u16;
    loop {
        if width == 4 {
            let row: __m128i =
                _mm_slli_epi16::<{ 3 }>(_mm_loadl_epi64(input as *mut __m128i as *const __m128i));
            _mm_storel_epi64(pred_buf_q3 as *mut __m128i, row);
        } else {
            let row: __m128i =
                _mm_slli_epi16::<{ 3 }>(_mm_loadu_si128(input as *mut __m128i as *const __m128i));
            _mm_storeu_si128(pred_buf_q3 as *mut __m128i, row);
            if width >= 16 {
                let mut row_1: __m128i =
                    _mm_loadu_si128((input as *mut __m128i).offset(1) as *const __m128i);
                row_1 = _mm_slli_epi16::<{ 3 }>(row_1);
                _mm_storeu_si128((pred_buf_q3 as *mut __m128i).offset(1), row_1);
                if width == 32 {
                    let mut row_2: __m128i =
                        _mm_loadu_si128((input as *mut __m128i).offset(2) as *const __m128i);
                    row_2 = _mm_slli_epi16::<{ 3 }>(row_2);
                    _mm_storeu_si128((pred_buf_q3 as *mut __m128i).offset(2), row_2);
                    let mut row_3: __m128i =
                        _mm_loadu_si128((input as *mut __m128i).offset(3) as *const __m128i);
                    row_3 = _mm_slli_epi16::<{ 3 }>(row_3);
                    _mm_storeu_si128((pred_buf_q3 as *mut __m128i).offset(3), row_3);
                }
            }
        }
        input = input.offset(input_stride as isize);
        pred_buf_q3 = pred_buf_q3.offset(CFL_BUF_LINE as isize);
        if !(pred_buf_q3 < end as *mut u16) {
            break;
        }
    }
}

#[target_feature(enable = "ssse3")]
pub unsafe extern "C-unwind" fn cfl_subsample_lbd_420_4x4_ssse3(
    cfl_type: *const u8,
    input_stride: i32,
    output_q3: *mut u16,
) {
    cfl_luma_subsampling_420_lbd_ssse3(cfl_type, input_stride, output_q3, 4, 4);
}

#[target_feature(enable = "ssse3")]
pub unsafe extern "C-unwind" fn cfl_subsample_lbd_420_8x8_ssse3(
    cfl_type: *const u8,
    input_stride: i32,
    output_q3: *mut u16,
) {
    cfl_luma_subsampling_420_lbd_ssse3(cfl_type, input_stride, output_q3, 8, 8);
}

#[target_feature(enable = "ssse3")]
pub unsafe extern "C-unwind" fn cfl_subsample_lbd_420_16x16_ssse3(
    cfl_type: *const u8,
    input_stride: i32,
    output_q3: *mut u16,
) {
    cfl_luma_subsampling_420_lbd_ssse3(cfl_type, input_stride, output_q3, 16, 16);
}

#[target_feature(enable = "ssse3")]
pub unsafe extern "C-unwind" fn cfl_subsample_lbd_420_32x32_ssse3(
    cfl_type: *const u8,
    input_stride: i32,
    output_q3: *mut u16,
) {
    cfl_luma_subsampling_420_lbd_ssse3(cfl_type, input_stride, output_q3, 32, 32);
}

#[target_feature(enable = "ssse3")]
pub unsafe extern "C-unwind" fn cfl_subsample_lbd_420_4x8_ssse3(
    cfl_type: *const u8,
    input_stride: i32,
    output_q3: *mut u16,
) {
    cfl_luma_subsampling_420_lbd_ssse3(cfl_type, input_stride, output_q3, 4, 8);
}

#[target_feature(enable = "ssse3")]
pub unsafe extern "C-unwind" fn cfl_subsample_lbd_420_8x4_ssse3(
    cfl_type: *const u8,
    input_stride: i32,
    output_q3: *mut u16,
) {
    cfl_luma_subsampling_420_lbd_ssse3(cfl_type, input_stride, output_q3, 8, 4);
}

#[target_feature(enable = "ssse3")]
pub unsafe extern "C-unwind" fn cfl_subsample_lbd_420_8x16_ssse3(
    cfl_type: *const u8,
    input_stride: i32,
    output_q3: *mut u16,
) {
    cfl_luma_subsampling_420_lbd_ssse3(cfl_type, input_stride, output_q3, 8, 16);
}

#[target_feature(enable = "ssse3")]
pub unsafe extern "C-unwind" fn cfl_subsample_lbd_420_16x8_ssse3(
    cfl_type: *const u8,
    input_stride: i32,
    output_q3: *mut u16,
) {
    cfl_luma_subsampling_420_lbd_ssse3(cfl_type, input_stride, output_q3, 16, 8);
}

#[target_feature(enable = "ssse3")]
pub unsafe extern "C-unwind" fn cfl_subsample_lbd_420_16x32_ssse3(
    cfl_type: *const u8,
    input_stride: i32,
    output_q3: *mut u16,
) {
    cfl_luma_subsampling_420_lbd_ssse3(cfl_type, input_stride, output_q3, 16, 32);
}

#[target_feature(enable = "ssse3")]
pub unsafe extern "C-unwind" fn cfl_subsample_lbd_420_32x16_ssse3(
    cfl_type: *const u8,
    input_stride: i32,
    output_q3: *mut u16,
) {
    cfl_luma_subsampling_420_lbd_ssse3(cfl_type, input_stride, output_q3, 32, 16);
}

#[target_feature(enable = "ssse3")]
pub unsafe extern "C-unwind" fn cfl_subsample_lbd_420_4x16_ssse3(
    cfl_type: *const u8,
    input_stride: i32,
    output_q3: *mut u16,
) {
    cfl_luma_subsampling_420_lbd_ssse3(cfl_type, input_stride, output_q3, 4, 16);
}

#[target_feature(enable = "ssse3")]
pub unsafe extern "C-unwind" fn cfl_subsample_lbd_420_16x4_ssse3(
    cfl_type: *const u8,
    input_stride: i32,
    output_q3: *mut u16,
) {
    cfl_luma_subsampling_420_lbd_ssse3(cfl_type, input_stride, output_q3, 16, 4);
}

#[target_feature(enable = "ssse3")]
pub unsafe extern "C-unwind" fn cfl_subsample_lbd_420_8x32_ssse3(
    cfl_type: *const u8,
    input_stride: i32,
    output_q3: *mut u16,
) {
    cfl_luma_subsampling_420_lbd_ssse3(cfl_type, input_stride, output_q3, 8, 32);
}

#[target_feature(enable = "ssse3")]
pub unsafe extern "C-unwind" fn cfl_subsample_lbd_420_32x8_ssse3(
    cfl_type: *const u8,
    input_stride: i32,
    output_q3: *mut u16,
) {
    cfl_luma_subsampling_420_lbd_ssse3(cfl_type, input_stride, output_q3, 32, 8);
}

#[no_mangle]
#[target_feature(enable = "ssse3")]
pub unsafe extern "C-unwind" fn cfl_get_luma_subsampling_420_lbd_ssse3(
    tx_size: u8,
) -> cfl_subsample_lbd_fn {
    let mut subfn_420: [cfl_subsample_lbd_fn; (TX_SIZES_ALL) as usize] = [
        Some(cfl_subsample_lbd_420_4x4_ssse3),
        Some(cfl_subsample_lbd_420_8x8_ssse3),
        Some(cfl_subsample_lbd_420_16x16_ssse3),
        Some(cfl_subsample_lbd_420_32x32_ssse3),
        None,
        Some(cfl_subsample_lbd_420_4x8_ssse3),
        Some(cfl_subsample_lbd_420_8x4_ssse3),
        Some(cfl_subsample_lbd_420_8x16_ssse3),
        Some(cfl_subsample_lbd_420_16x8_ssse3),
        Some(cfl_subsample_lbd_420_16x32_ssse3),
        Some(cfl_subsample_lbd_420_32x16_ssse3),
        None,
        None,
        Some(cfl_subsample_lbd_420_4x16_ssse3),
        Some(cfl_subsample_lbd_420_16x4_ssse3),
        Some(cfl_subsample_lbd_420_8x32_ssse3),
        Some(cfl_subsample_lbd_420_32x8_ssse3),
        None,
        None,
    ];
    return subfn_420[tx_size as usize];
}

#[target_feature(enable = "ssse3")]
pub unsafe extern "C-unwind" fn cfl_subsample_lbd_422_4x4_ssse3(
    cfl_type: *const u8,
    input_stride: i32,
    output_q3: *mut u16,
) {
    cfl_luma_subsampling_422_lbd_ssse3(cfl_type, input_stride, output_q3, 4, 4);
}

#[target_feature(enable = "ssse3")]
pub unsafe extern "C-unwind" fn cfl_subsample_lbd_422_8x8_ssse3(
    cfl_type: *const u8,
    input_stride: i32,
    output_q3: *mut u16,
) {
    cfl_luma_subsampling_422_lbd_ssse3(cfl_type, input_stride, output_q3, 8, 8);
}

#[target_feature(enable = "ssse3")]
pub unsafe extern "C-unwind" fn cfl_subsample_lbd_422_16x16_ssse3(
    cfl_type: *const u8,
    input_stride: i32,
    output_q3: *mut u16,
) {
    cfl_luma_subsampling_422_lbd_ssse3(cfl_type, input_stride, output_q3, 16, 16);
}

#[target_feature(enable = "ssse3")]
pub unsafe extern "C-unwind" fn cfl_subsample_lbd_422_32x32_ssse3(
    cfl_type: *const u8,
    input_stride: i32,
    output_q3: *mut u16,
) {
    cfl_luma_subsampling_422_lbd_ssse3(cfl_type, input_stride, output_q3, 32, 32);
}

#[target_feature(enable = "ssse3")]
pub unsafe extern "C-unwind" fn cfl_subsample_lbd_422_4x8_ssse3(
    cfl_type: *const u8,
    input_stride: i32,
    output_q3: *mut u16,
) {
    cfl_luma_subsampling_422_lbd_ssse3(cfl_type, input_stride, output_q3, 4, 8);
}

#[target_feature(enable = "ssse3")]
pub unsafe extern "C-unwind" fn cfl_subsample_lbd_422_8x4_ssse3(
    cfl_type: *const u8,
    input_stride: i32,
    output_q3: *mut u16,
) {
    cfl_luma_subsampling_422_lbd_ssse3(cfl_type, input_stride, output_q3, 8, 4);
}

#[target_feature(enable = "ssse3")]
pub unsafe extern "C-unwind" fn cfl_subsample_lbd_422_8x16_ssse3(
    cfl_type: *const u8,
    input_stride: i32,
    output_q3: *mut u16,
) {
    cfl_luma_subsampling_422_lbd_ssse3(cfl_type, input_stride, output_q3, 8, 16);
}

#[target_feature(enable = "ssse3")]
pub unsafe extern "C-unwind" fn cfl_subsample_lbd_422_16x8_ssse3(
    cfl_type: *const u8,
    input_stride: i32,
    output_q3: *mut u16,
) {
    cfl_luma_subsampling_422_lbd_ssse3(cfl_type, input_stride, output_q3, 16, 8);
}

#[target_feature(enable = "ssse3")]
pub unsafe extern "C-unwind" fn cfl_subsample_lbd_422_16x32_ssse3(
    cfl_type: *const u8,
    input_stride: i32,
    output_q3: *mut u16,
) {
    cfl_luma_subsampling_422_lbd_ssse3(cfl_type, input_stride, output_q3, 16, 32);
}

#[target_feature(enable = "ssse3")]
pub unsafe extern "C-unwind" fn cfl_subsample_lbd_422_32x16_ssse3(
    cfl_type: *const u8,
    input_stride: i32,
    output_q3: *mut u16,
) {
    cfl_luma_subsampling_422_lbd_ssse3(cfl_type, input_stride, output_q3, 32, 16);
}

#[target_feature(enable = "ssse3")]
pub unsafe extern "C-unwind" fn cfl_subsample_lbd_422_4x16_ssse3(
    cfl_type: *const u8,
    input_stride: i32,
    output_q3: *mut u16,
) {
    cfl_luma_subsampling_422_lbd_ssse3(cfl_type, input_stride, output_q3, 4, 16);
}

#[target_feature(enable = "ssse3")]
pub unsafe extern "C-unwind" fn cfl_subsample_lbd_422_16x4_ssse3(
    cfl_type: *const u8,
    input_stride: i32,
    output_q3: *mut u16,
) {
    cfl_luma_subsampling_422_lbd_ssse3(cfl_type, input_stride, output_q3, 16, 4);
}

#[target_feature(enable = "ssse3")]
pub unsafe extern "C-unwind" fn cfl_subsample_lbd_422_8x32_ssse3(
    cfl_type: *const u8,
    input_stride: i32,
    output_q3: *mut u16,
) {
    cfl_luma_subsampling_422_lbd_ssse3(cfl_type, input_stride, output_q3, 8, 32);
}

#[target_feature(enable = "ssse3")]
pub unsafe extern "C-unwind" fn cfl_subsample_lbd_422_32x8_ssse3(
    cfl_type: *const u8,
    input_stride: i32,
    output_q3: *mut u16,
) {
    cfl_luma_subsampling_422_lbd_ssse3(cfl_type, input_stride, output_q3, 32, 8);
}

#[no_mangle]
#[target_feature(enable = "ssse3")]
pub unsafe extern "C-unwind" fn cfl_get_luma_subsampling_422_lbd_ssse3(
    tx_size: u8,
) -> cfl_subsample_lbd_fn {
    let mut subfn_422: [cfl_subsample_lbd_fn; (TX_SIZES_ALL) as usize] = [
        Some(cfl_subsample_lbd_422_4x4_ssse3),
        Some(cfl_subsample_lbd_422_8x8_ssse3),
        Some(cfl_subsample_lbd_422_16x16_ssse3),
        Some(cfl_subsample_lbd_422_32x32_ssse3),
        None,
        Some(cfl_subsample_lbd_422_4x8_ssse3),
        Some(cfl_subsample_lbd_422_8x4_ssse3),
        Some(cfl_subsample_lbd_422_8x16_ssse3),
        Some(cfl_subsample_lbd_422_16x8_ssse3),
        Some(cfl_subsample_lbd_422_16x32_ssse3),
        Some(cfl_subsample_lbd_422_32x16_ssse3),
        None,
        None,
        Some(cfl_subsample_lbd_422_4x16_ssse3),
        Some(cfl_subsample_lbd_422_16x4_ssse3),
        Some(cfl_subsample_lbd_422_8x32_ssse3),
        Some(cfl_subsample_lbd_422_32x8_ssse3),
        None,
        None,
    ];
    return subfn_422[tx_size as usize];
}

#[target_feature(enable = "ssse3")]
pub unsafe extern "C-unwind" fn cfl_subsample_lbd_444_4x4_ssse3(
    cfl_type: *const u8,
    input_stride: i32,
    output_q3: *mut u16,
) {
    cfl_luma_subsampling_444_lbd_ssse3(cfl_type, input_stride, output_q3, 4, 4);
}

#[target_feature(enable = "ssse3")]
pub unsafe extern "C-unwind" fn cfl_subsample_lbd_444_8x8_ssse3(
    cfl_type: *const u8,
    input_stride: i32,
    output_q3: *mut u16,
) {
    cfl_luma_subsampling_444_lbd_ssse3(cfl_type, input_stride, output_q3, 8, 8);
}

#[target_feature(enable = "ssse3")]
pub unsafe extern "C-unwind" fn cfl_subsample_lbd_444_16x16_ssse3(
    cfl_type: *const u8,
    input_stride: i32,
    output_q3: *mut u16,
) {
    cfl_luma_subsampling_444_lbd_ssse3(cfl_type, input_stride, output_q3, 16, 16);
}

#[target_feature(enable = "ssse3")]
pub unsafe extern "C-unwind" fn cfl_subsample_lbd_444_32x32_ssse3(
    cfl_type: *const u8,
    input_stride: i32,
    output_q3: *mut u16,
) {
    cfl_luma_subsampling_444_lbd_ssse3(cfl_type, input_stride, output_q3, 32, 32);
}

#[target_feature(enable = "ssse3")]
pub unsafe extern "C-unwind" fn cfl_subsample_lbd_444_4x8_ssse3(
    cfl_type: *const u8,
    input_stride: i32,
    output_q3: *mut u16,
) {
    cfl_luma_subsampling_444_lbd_ssse3(cfl_type, input_stride, output_q3, 4, 8);
}

#[target_feature(enable = "ssse3")]
pub unsafe extern "C-unwind" fn cfl_subsample_lbd_444_8x4_ssse3(
    cfl_type: *const u8,
    input_stride: i32,
    output_q3: *mut u16,
) {
    cfl_luma_subsampling_444_lbd_ssse3(cfl_type, input_stride, output_q3, 8, 4);
}

#[target_feature(enable = "ssse3")]
pub unsafe extern "C-unwind" fn cfl_subsample_lbd_444_8x16_ssse3(
    cfl_type: *const u8,
    input_stride: i32,
    output_q3: *mut u16,
) {
    cfl_luma_subsampling_444_lbd_ssse3(cfl_type, input_stride, output_q3, 8, 16);
}

#[target_feature(enable = "ssse3")]
pub unsafe extern "C-unwind" fn cfl_subsample_lbd_444_16x8_ssse3(
    cfl_type: *const u8,
    input_stride: i32,
    output_q3: *mut u16,
) {
    cfl_luma_subsampling_444_lbd_ssse3(cfl_type, input_stride, output_q3, 16, 8);
}

#[target_feature(enable = "ssse3")]
pub unsafe extern "C-unwind" fn cfl_subsample_lbd_444_16x32_ssse3(
    cfl_type: *const u8,
    input_stride: i32,
    output_q3: *mut u16,
) {
    cfl_luma_subsampling_444_lbd_ssse3(cfl_type, input_stride, output_q3, 16, 32);
}

#[target_feature(enable = "ssse3")]
pub unsafe extern "C-unwind" fn cfl_subsample_lbd_444_32x16_ssse3(
    cfl_type: *const u8,
    input_stride: i32,
    output_q3: *mut u16,
) {
    cfl_luma_subsampling_444_lbd_ssse3(cfl_type, input_stride, output_q3, 32, 16);
}

#[target_feature(enable = "ssse3")]
pub unsafe extern "C-unwind" fn cfl_subsample_lbd_444_4x16_ssse3(
    cfl_type: *const u8,
    input_stride: i32,
    output_q3: *mut u16,
) {
    cfl_luma_subsampling_444_lbd_ssse3(cfl_type, input_stride, output_q3, 4, 16);
}

#[target_feature(enable = "ssse3")]
pub unsafe extern "C-unwind" fn cfl_subsample_lbd_444_16x4_ssse3(
    cfl_type: *const u8,
    input_stride: i32,
    output_q3: *mut u16,
) {
    cfl_luma_subsampling_444_lbd_ssse3(cfl_type, input_stride, output_q3, 16, 4);
}

#[target_feature(enable = "ssse3")]
pub unsafe extern "C-unwind" fn cfl_subsample_lbd_444_8x32_ssse3(
    cfl_type: *const u8,
    input_stride: i32,
    output_q3: *mut u16,
) {
    cfl_luma_subsampling_444_lbd_ssse3(cfl_type, input_stride, output_q3, 8, 32);
}

#[target_feature(enable = "ssse3")]
pub unsafe extern "C-unwind" fn cfl_subsample_lbd_444_32x8_ssse3(
    cfl_type: *const u8,
    input_stride: i32,
    output_q3: *mut u16,
) {
    cfl_luma_subsampling_444_lbd_ssse3(cfl_type, input_stride, output_q3, 32, 8);
}

#[no_mangle]
#[target_feature(enable = "ssse3")]
pub unsafe extern "C-unwind" fn cfl_get_luma_subsampling_444_lbd_ssse3(
    tx_size: u8,
) -> cfl_subsample_lbd_fn {
    let mut subfn_444: [cfl_subsample_lbd_fn; (TX_SIZES_ALL) as usize] = [
        Some(cfl_subsample_lbd_444_4x4_ssse3),
        Some(cfl_subsample_lbd_444_8x8_ssse3),
        Some(cfl_subsample_lbd_444_16x16_ssse3),
        Some(cfl_subsample_lbd_444_32x32_ssse3),
        None,
        Some(cfl_subsample_lbd_444_4x8_ssse3),
        Some(cfl_subsample_lbd_444_8x4_ssse3),
        Some(cfl_subsample_lbd_444_8x16_ssse3),
        Some(cfl_subsample_lbd_444_16x8_ssse3),
        Some(cfl_subsample_lbd_444_16x32_ssse3),
        Some(cfl_subsample_lbd_444_32x16_ssse3),
        None,
        None,
        Some(cfl_subsample_lbd_444_4x16_ssse3),
        Some(cfl_subsample_lbd_444_16x4_ssse3),
        Some(cfl_subsample_lbd_444_8x32_ssse3),
        Some(cfl_subsample_lbd_444_32x8_ssse3),
        None,
        None,
    ];
    return subfn_444[tx_size as usize];
}

#[target_feature(enable = "ssse3")]
pub unsafe extern "C-unwind" fn cfl_subsample_hbd_420_4x4_ssse3(
    cfl_type: *const u16,
    input_stride: i32,
    output_q3: *mut u16,
) {
    cfl_luma_subsampling_420_hbd_ssse3(cfl_type, input_stride, output_q3, 4, 4);
}

#[target_feature(enable = "ssse3")]
pub unsafe extern "C-unwind" fn cfl_subsample_hbd_420_8x8_ssse3(
    cfl_type: *const u16,
    input_stride: i32,
    output_q3: *mut u16,
) {
    cfl_luma_subsampling_420_hbd_ssse3(cfl_type, input_stride, output_q3, 8, 8);
}

#[target_feature(enable = "ssse3")]
pub unsafe extern "C-unwind" fn cfl_subsample_hbd_420_16x16_ssse3(
    cfl_type: *const u16,
    input_stride: i32,
    output_q3: *mut u16,
) {
    cfl_luma_subsampling_420_hbd_ssse3(cfl_type, input_stride, output_q3, 16, 16);
}

#[target_feature(enable = "ssse3")]
pub unsafe extern "C-unwind" fn cfl_subsample_hbd_420_32x32_ssse3(
    cfl_type: *const u16,
    input_stride: i32,
    output_q3: *mut u16,
) {
    cfl_luma_subsampling_420_hbd_ssse3(cfl_type, input_stride, output_q3, 32, 32);
}

#[target_feature(enable = "ssse3")]
pub unsafe extern "C-unwind" fn cfl_subsample_hbd_420_4x8_ssse3(
    cfl_type: *const u16,
    input_stride: i32,
    output_q3: *mut u16,
) {
    cfl_luma_subsampling_420_hbd_ssse3(cfl_type, input_stride, output_q3, 4, 8);
}

#[target_feature(enable = "ssse3")]
pub unsafe extern "C-unwind" fn cfl_subsample_hbd_420_8x4_ssse3(
    cfl_type: *const u16,
    input_stride: i32,
    output_q3: *mut u16,
) {
    cfl_luma_subsampling_420_hbd_ssse3(cfl_type, input_stride, output_q3, 8, 4);
}

#[target_feature(enable = "ssse3")]
pub unsafe extern "C-unwind" fn cfl_subsample_hbd_420_8x16_ssse3(
    cfl_type: *const u16,
    input_stride: i32,
    output_q3: *mut u16,
) {
    cfl_luma_subsampling_420_hbd_ssse3(cfl_type, input_stride, output_q3, 8, 16);
}

#[target_feature(enable = "ssse3")]
pub unsafe extern "C-unwind" fn cfl_subsample_hbd_420_16x8_ssse3(
    cfl_type: *const u16,
    input_stride: i32,
    output_q3: *mut u16,
) {
    cfl_luma_subsampling_420_hbd_ssse3(cfl_type, input_stride, output_q3, 16, 8);
}

#[target_feature(enable = "ssse3")]
pub unsafe extern "C-unwind" fn cfl_subsample_hbd_420_16x32_ssse3(
    cfl_type: *const u16,
    input_stride: i32,
    output_q3: *mut u16,
) {
    cfl_luma_subsampling_420_hbd_ssse3(cfl_type, input_stride, output_q3, 16, 32);
}

#[target_feature(enable = "ssse3")]
pub unsafe extern "C-unwind" fn cfl_subsample_hbd_420_32x16_ssse3(
    cfl_type: *const u16,
    input_stride: i32,
    output_q3: *mut u16,
) {
    cfl_luma_subsampling_420_hbd_ssse3(cfl_type, input_stride, output_q3, 32, 16);
}

#[target_feature(enable = "ssse3")]
pub unsafe extern "C-unwind" fn cfl_subsample_hbd_420_4x16_ssse3(
    cfl_type: *const u16,
    input_stride: i32,
    output_q3: *mut u16,
) {
    cfl_luma_subsampling_420_hbd_ssse3(cfl_type, input_stride, output_q3, 4, 16);
}

#[target_feature(enable = "ssse3")]
pub unsafe extern "C-unwind" fn cfl_subsample_hbd_420_16x4_ssse3(
    cfl_type: *const u16,
    input_stride: i32,
    output_q3: *mut u16,
) {
    cfl_luma_subsampling_420_hbd_ssse3(cfl_type, input_stride, output_q3, 16, 4);
}

#[target_feature(enable = "ssse3")]
pub unsafe extern "C-unwind" fn cfl_subsample_hbd_420_8x32_ssse3(
    cfl_type: *const u16,
    input_stride: i32,
    output_q3: *mut u16,
) {
    cfl_luma_subsampling_420_hbd_ssse3(cfl_type, input_stride, output_q3, 8, 32);
}

#[target_feature(enable = "ssse3")]
pub unsafe extern "C-unwind" fn cfl_subsample_hbd_420_32x8_ssse3(
    cfl_type: *const u16,
    input_stride: i32,
    output_q3: *mut u16,
) {
    cfl_luma_subsampling_420_hbd_ssse3(cfl_type, input_stride, output_q3, 32, 8);
}

#[no_mangle]
#[target_feature(enable = "ssse3")]
pub unsafe extern "C-unwind" fn cfl_get_luma_subsampling_420_hbd_ssse3(
    tx_size: u8,
) -> cfl_subsample_hbd_fn {
    let mut subfn_420: [cfl_subsample_hbd_fn; (TX_SIZES_ALL) as usize] = [
        Some(cfl_subsample_hbd_420_4x4_ssse3),
        Some(cfl_subsample_hbd_420_8x8_ssse3),
        Some(cfl_subsample_hbd_420_16x16_ssse3),
        Some(cfl_subsample_hbd_420_32x32_ssse3),
        None,
        Some(cfl_subsample_hbd_420_4x8_ssse3),
        Some(cfl_subsample_hbd_420_8x4_ssse3),
        Some(cfl_subsample_hbd_420_8x16_ssse3),
        Some(cfl_subsample_hbd_420_16x8_ssse3),
        Some(cfl_subsample_hbd_420_16x32_ssse3),
        Some(cfl_subsample_hbd_420_32x16_ssse3),
        None,
        None,
        Some(cfl_subsample_hbd_420_4x16_ssse3),
        Some(cfl_subsample_hbd_420_16x4_ssse3),
        Some(cfl_subsample_hbd_420_8x32_ssse3),
        Some(cfl_subsample_hbd_420_32x8_ssse3),
        None,
        None,
    ];
    return subfn_420[tx_size as usize];
}

#[target_feature(enable = "ssse3")]
pub unsafe extern "C-unwind" fn cfl_subsample_hbd_422_4x4_ssse3(
    cfl_type: *const u16,
    input_stride: i32,
    output_q3: *mut u16,
) {
    cfl_luma_subsampling_422_hbd_ssse3(cfl_type, input_stride, output_q3, 4, 4);
}

#[target_feature(enable = "ssse3")]
pub unsafe extern "C-unwind" fn cfl_subsample_hbd_422_8x8_ssse3(
    cfl_type: *const u16,
    input_stride: i32,
    output_q3: *mut u16,
) {
    cfl_luma_subsampling_422_hbd_ssse3(cfl_type, input_stride, output_q3, 8, 8);
}

#[target_feature(enable = "ssse3")]
pub unsafe extern "C-unwind" fn cfl_subsample_hbd_422_16x16_ssse3(
    cfl_type: *const u16,
    input_stride: i32,
    output_q3: *mut u16,
) {
    cfl_luma_subsampling_422_hbd_ssse3(cfl_type, input_stride, output_q3, 16, 16);
}

#[target_feature(enable = "ssse3")]
pub unsafe extern "C-unwind" fn cfl_subsample_hbd_422_32x32_ssse3(
    cfl_type: *const u16,
    input_stride: i32,
    output_q3: *mut u16,
) {
    cfl_luma_subsampling_422_hbd_ssse3(cfl_type, input_stride, output_q3, 32, 32);
}

#[target_feature(enable = "ssse3")]
pub unsafe extern "C-unwind" fn cfl_subsample_hbd_422_4x8_ssse3(
    cfl_type: *const u16,
    input_stride: i32,
    output_q3: *mut u16,
) {
    cfl_luma_subsampling_422_hbd_ssse3(cfl_type, input_stride, output_q3, 4, 8);
}

#[target_feature(enable = "ssse3")]
pub unsafe extern "C-unwind" fn cfl_subsample_hbd_422_8x4_ssse3(
    cfl_type: *const u16,
    input_stride: i32,
    output_q3: *mut u16,
) {
    cfl_luma_subsampling_422_hbd_ssse3(cfl_type, input_stride, output_q3, 8, 4);
}

#[target_feature(enable = "ssse3")]
pub unsafe extern "C-unwind" fn cfl_subsample_hbd_422_8x16_ssse3(
    cfl_type: *const u16,
    input_stride: i32,
    output_q3: *mut u16,
) {
    cfl_luma_subsampling_422_hbd_ssse3(cfl_type, input_stride, output_q3, 8, 16);
}

#[target_feature(enable = "ssse3")]
pub unsafe extern "C-unwind" fn cfl_subsample_hbd_422_16x8_ssse3(
    cfl_type: *const u16,
    input_stride: i32,
    output_q3: *mut u16,
) {
    cfl_luma_subsampling_422_hbd_ssse3(cfl_type, input_stride, output_q3, 16, 8);
}

#[target_feature(enable = "ssse3")]
pub unsafe extern "C-unwind" fn cfl_subsample_hbd_422_16x32_ssse3(
    cfl_type: *const u16,
    input_stride: i32,
    output_q3: *mut u16,
) {
    cfl_luma_subsampling_422_hbd_ssse3(cfl_type, input_stride, output_q3, 16, 32);
}

#[target_feature(enable = "ssse3")]
pub unsafe extern "C-unwind" fn cfl_subsample_hbd_422_32x16_ssse3(
    cfl_type: *const u16,
    input_stride: i32,
    output_q3: *mut u16,
) {
    cfl_luma_subsampling_422_hbd_ssse3(cfl_type, input_stride, output_q3, 32, 16);
}

#[target_feature(enable = "ssse3")]
pub unsafe extern "C-unwind" fn cfl_subsample_hbd_422_4x16_ssse3(
    cfl_type: *const u16,
    input_stride: i32,
    output_q3: *mut u16,
) {
    cfl_luma_subsampling_422_hbd_ssse3(cfl_type, input_stride, output_q3, 4, 16);
}

#[target_feature(enable = "ssse3")]
pub unsafe extern "C-unwind" fn cfl_subsample_hbd_422_16x4_ssse3(
    cfl_type: *const u16,
    input_stride: i32,
    output_q3: *mut u16,
) {
    cfl_luma_subsampling_422_hbd_ssse3(cfl_type, input_stride, output_q3, 16, 4);
}

#[target_feature(enable = "ssse3")]
pub unsafe extern "C-unwind" fn cfl_subsample_hbd_422_8x32_ssse3(
    cfl_type: *const u16,
    input_stride: i32,
    output_q3: *mut u16,
) {
    cfl_luma_subsampling_422_hbd_ssse3(cfl_type, input_stride, output_q3, 8, 32);
}

#[target_feature(enable = "ssse3")]
pub unsafe extern "C-unwind" fn cfl_subsample_hbd_422_32x8_ssse3(
    cfl_type: *const u16,
    input_stride: i32,
    output_q3: *mut u16,
) {
    cfl_luma_subsampling_422_hbd_ssse3(cfl_type, input_stride, output_q3, 32, 8);
}

#[no_mangle]
#[target_feature(enable = "ssse3")]
pub unsafe extern "C-unwind" fn cfl_get_luma_subsampling_422_hbd_ssse3(
    tx_size: u8,
) -> cfl_subsample_hbd_fn {
    let mut subfn_422: [cfl_subsample_hbd_fn; (TX_SIZES_ALL) as usize] = [
        Some(cfl_subsample_hbd_422_4x4_ssse3),
        Some(cfl_subsample_hbd_422_8x8_ssse3),
        Some(cfl_subsample_hbd_422_16x16_ssse3),
        Some(cfl_subsample_hbd_422_32x32_ssse3),
        None,
        Some(cfl_subsample_hbd_422_4x8_ssse3),
        Some(cfl_subsample_hbd_422_8x4_ssse3),
        Some(cfl_subsample_hbd_422_8x16_ssse3),
        Some(cfl_subsample_hbd_422_16x8_ssse3),
        Some(cfl_subsample_hbd_422_16x32_ssse3),
        Some(cfl_subsample_hbd_422_32x16_ssse3),
        None,
        None,
        Some(cfl_subsample_hbd_422_4x16_ssse3),
        Some(cfl_subsample_hbd_422_16x4_ssse3),
        Some(cfl_subsample_hbd_422_8x32_ssse3),
        Some(cfl_subsample_hbd_422_32x8_ssse3),
        None,
        None,
    ];
    return subfn_422[tx_size as usize];
}

#[target_feature(enable = "ssse3")]
pub unsafe extern "C-unwind" fn cfl_subsample_hbd_444_4x4_ssse3(
    cfl_type: *const u16,
    input_stride: i32,
    output_q3: *mut u16,
) {
    cfl_luma_subsampling_444_hbd_ssse3(cfl_type, input_stride, output_q3, 4, 4);
}

#[target_feature(enable = "ssse3")]
pub unsafe extern "C-unwind" fn cfl_subsample_hbd_444_8x8_ssse3(
    cfl_type: *const u16,
    input_stride: i32,
    output_q3: *mut u16,
) {
    cfl_luma_subsampling_444_hbd_ssse3(cfl_type, input_stride, output_q3, 8, 8);
}

#[target_feature(enable = "ssse3")]
pub unsafe extern "C-unwind" fn cfl_subsample_hbd_444_16x16_ssse3(
    cfl_type: *const u16,
    input_stride: i32,
    output_q3: *mut u16,
) {
    cfl_luma_subsampling_444_hbd_ssse3(cfl_type, input_stride, output_q3, 16, 16);
}

#[target_feature(enable = "ssse3")]
pub unsafe extern "C-unwind" fn cfl_subsample_hbd_444_32x32_ssse3(
    cfl_type: *const u16,
    input_stride: i32,
    output_q3: *mut u16,
) {
    cfl_luma_subsampling_444_hbd_ssse3(cfl_type, input_stride, output_q3, 32, 32);
}

#[target_feature(enable = "ssse3")]
pub unsafe extern "C-unwind" fn cfl_subsample_hbd_444_4x8_ssse3(
    cfl_type: *const u16,
    input_stride: i32,
    output_q3: *mut u16,
) {
    cfl_luma_subsampling_444_hbd_ssse3(cfl_type, input_stride, output_q3, 4, 8);
}

#[target_feature(enable = "ssse3")]
pub unsafe extern "C-unwind" fn cfl_subsample_hbd_444_8x4_ssse3(
    cfl_type: *const u16,
    input_stride: i32,
    output_q3: *mut u16,
) {
    cfl_luma_subsampling_444_hbd_ssse3(cfl_type, input_stride, output_q3, 8, 4);
}

#[target_feature(enable = "ssse3")]
pub unsafe extern "C-unwind" fn cfl_subsample_hbd_444_8x16_ssse3(
    cfl_type: *const u16,
    input_stride: i32,
    output_q3: *mut u16,
) {
    cfl_luma_subsampling_444_hbd_ssse3(cfl_type, input_stride, output_q3, 8, 16);
}

#[target_feature(enable = "ssse3")]
pub unsafe extern "C-unwind" fn cfl_subsample_hbd_444_16x8_ssse3(
    cfl_type: *const u16,
    input_stride: i32,
    output_q3: *mut u16,
) {
    cfl_luma_subsampling_444_hbd_ssse3(cfl_type, input_stride, output_q3, 16, 8);
}

#[target_feature(enable = "ssse3")]
pub unsafe extern "C-unwind" fn cfl_subsample_hbd_444_16x32_ssse3(
    cfl_type: *const u16,
    input_stride: i32,
    output_q3: *mut u16,
) {
    cfl_luma_subsampling_444_hbd_ssse3(cfl_type, input_stride, output_q3, 16, 32);
}

#[target_feature(enable = "ssse3")]
pub unsafe extern "C-unwind" fn cfl_subsample_hbd_444_32x16_ssse3(
    cfl_type: *const u16,
    input_stride: i32,
    output_q3: *mut u16,
) {
    cfl_luma_subsampling_444_hbd_ssse3(cfl_type, input_stride, output_q3, 32, 16);
}

#[target_feature(enable = "ssse3")]
pub unsafe extern "C-unwind" fn cfl_subsample_hbd_444_4x16_ssse3(
    cfl_type: *const u16,
    input_stride: i32,
    output_q3: *mut u16,
) {
    cfl_luma_subsampling_444_hbd_ssse3(cfl_type, input_stride, output_q3, 4, 16);
}

#[target_feature(enable = "ssse3")]
pub unsafe extern "C-unwind" fn cfl_subsample_hbd_444_16x4_ssse3(
    cfl_type: *const u16,
    input_stride: i32,
    output_q3: *mut u16,
) {
    cfl_luma_subsampling_444_hbd_ssse3(cfl_type, input_stride, output_q3, 16, 4);
}

#[target_feature(enable = "ssse3")]
pub unsafe extern "C-unwind" fn cfl_subsample_hbd_444_8x32_ssse3(
    cfl_type: *const u16,
    input_stride: i32,
    output_q3: *mut u16,
) {
    cfl_luma_subsampling_444_hbd_ssse3(cfl_type, input_stride, output_q3, 8, 32);
}

#[target_feature(enable = "ssse3")]
pub unsafe extern "C-unwind" fn cfl_subsample_hbd_444_32x8_ssse3(
    cfl_type: *const u16,
    input_stride: i32,
    output_q3: *mut u16,
) {
    cfl_luma_subsampling_444_hbd_ssse3(cfl_type, input_stride, output_q3, 32, 8);
}

#[no_mangle]
#[target_feature(enable = "ssse3")]
pub unsafe extern "C-unwind" fn cfl_get_luma_subsampling_444_hbd_ssse3(
    tx_size: u8,
) -> cfl_subsample_hbd_fn {
    let mut subfn_444: [cfl_subsample_hbd_fn; (TX_SIZES_ALL) as usize] = [
        Some(cfl_subsample_hbd_444_4x4_ssse3),
        Some(cfl_subsample_hbd_444_8x8_ssse3),
        Some(cfl_subsample_hbd_444_16x16_ssse3),
        Some(cfl_subsample_hbd_444_32x32_ssse3),
        None,
        Some(cfl_subsample_hbd_444_4x8_ssse3),
        Some(cfl_subsample_hbd_444_8x4_ssse3),
        Some(cfl_subsample_hbd_444_8x16_ssse3),
        Some(cfl_subsample_hbd_444_16x8_ssse3),
        Some(cfl_subsample_hbd_444_16x32_ssse3),
        Some(cfl_subsample_hbd_444_32x16_ssse3),
        None,
        None,
        Some(cfl_subsample_hbd_444_4x16_ssse3),
        Some(cfl_subsample_hbd_444_16x4_ssse3),
        Some(cfl_subsample_hbd_444_8x32_ssse3),
        Some(cfl_subsample_hbd_444_32x8_ssse3),
        None,
        None,
    ];
    return subfn_444[tx_size as usize];
}

#[inline]
#[target_feature(enable = "ssse3")]
unsafe fn predict_unclipped(
    input: *const __m128i,
    alpha_q12: __m128i,
    alpha_sign: __m128i,
    dc_q0: __m128i,
) -> __m128i {
    let mut ac_q3: __m128i = _mm_loadu_si128(input);
    let mut ac_sign: __m128i = _mm_sign_epi16(alpha_sign, ac_q3);
    let mut scaled_luma_q0: __m128i = _mm_mulhrs_epi16(_mm_abs_epi16(ac_q3), alpha_q12);
    scaled_luma_q0 = _mm_sign_epi16(scaled_luma_q0, ac_sign);
    return _mm_add_epi16(scaled_luma_q0, dc_q0);
}

#[inline]
#[target_feature(enable = "ssse3")]
pub unsafe extern "C-unwind" fn cfl_predict_lbd_ssse3(
    pred_buf_q3: *const i16,
    mut dst: *mut u8,
    dst_stride: i32,
    alpha_q3: i32,
    width: i32,
    height: i32,
) {
    let alpha_sign: __m128i = _mm_set1_epi16(alpha_q3 as i16);
    let alpha_q12: __m128i = _mm_slli_epi16::<{ 9 }>(_mm_abs_epi16(alpha_sign));
    let dc_q0: __m128i = _mm_set1_epi16(*dst as i16);
    let mut row: *mut __m128i = pred_buf_q3 as *mut __m128i;
    let mut row_end: *const __m128i =
        row.offset((height * CFL_BUF_LINE_I128) as isize) as *const __m128i;
    loop {
        let mut res: __m128i =
            predict_unclipped(row as *const __m128i, alpha_q12, alpha_sign, dc_q0);
        if width < 16 {
            res = _mm_packus_epi16(res, res);
            if width == 4 {
                _mm_storeh_epi32(dst as *mut __m128i as *const __m128i, res);
            } else {
                _mm_storel_epi64(dst as *mut __m128i, res);
            }
        } else {
            let mut next: __m128i = predict_unclipped(
                row.offset(1) as *const __m128i,
                alpha_q12,
                alpha_sign,
                dc_q0,
            );
            res = _mm_packus_epi16(res, next);
            _mm_storeu_si128(dst as *mut __m128i, res);
            if width == 32 {
                res = predict_unclipped(
                    row.offset(2) as *const __m128i,
                    alpha_q12,
                    alpha_sign,
                    dc_q0,
                );
                next = predict_unclipped(
                    row.offset(3) as *const __m128i,
                    alpha_q12,
                    alpha_sign,
                    dc_q0,
                );
                res = _mm_packus_epi16(res, next);
                _mm_storeu_si128(dst.offset(16) as *mut __m128i, res);
            }
        }
        dst = dst.offset(dst_stride as isize);
        row = row.offset(CFL_BUF_LINE_I128 as isize);
        if !(row < row_end as *mut __m128i) {
            break;
        }
    }
}

#[target_feature(enable = "ssse3")]
pub unsafe extern "C-unwind" fn cfl_predict_lbd_4x4_ssse3(
    pred_buf_q3: *const i16,
    dst: *mut u8,
    dst_stride: i32,
    alpha_q3: i32,
) {
    cfl_predict_lbd_ssse3(pred_buf_q3, dst, dst_stride, alpha_q3, 4, 4);
}

#[target_feature(enable = "ssse3")]
pub unsafe extern "C-unwind" fn cfl_predict_lbd_4x8_ssse3(
    pred_buf_q3: *const i16,
    dst: *mut u8,
    dst_stride: i32,
    alpha_q3: i32,
) {
    cfl_predict_lbd_ssse3(pred_buf_q3, dst, dst_stride, alpha_q3, 4, 8);
}

#[target_feature(enable = "ssse3")]
pub unsafe extern "C-unwind" fn cfl_predict_lbd_4x16_ssse3(
    pred_buf_q3: *const i16,
    dst: *mut u8,
    dst_stride: i32,
    alpha_q3: i32,
) {
    cfl_predict_lbd_ssse3(pred_buf_q3, dst, dst_stride, alpha_q3, 4, 16);
}

#[target_feature(enable = "ssse3")]
pub unsafe extern "C-unwind" fn cfl_predict_lbd_8x4_ssse3(
    pred_buf_q3: *const i16,
    dst: *mut u8,
    dst_stride: i32,
    alpha_q3: i32,
) {
    cfl_predict_lbd_ssse3(pred_buf_q3, dst, dst_stride, alpha_q3, 8, 4);
}

#[target_feature(enable = "ssse3")]
pub unsafe extern "C-unwind" fn cfl_predict_lbd_8x8_ssse3(
    pred_buf_q3: *const i16,
    dst: *mut u8,
    dst_stride: i32,
    alpha_q3: i32,
) {
    cfl_predict_lbd_ssse3(pred_buf_q3, dst, dst_stride, alpha_q3, 8, 8);
}

#[target_feature(enable = "ssse3")]
pub unsafe extern "C-unwind" fn cfl_predict_lbd_8x16_ssse3(
    pred_buf_q3: *const i16,
    dst: *mut u8,
    dst_stride: i32,
    alpha_q3: i32,
) {
    cfl_predict_lbd_ssse3(pred_buf_q3, dst, dst_stride, alpha_q3, 8, 16);
}

#[target_feature(enable = "ssse3")]
pub unsafe extern "C-unwind" fn cfl_predict_lbd_8x32_ssse3(
    pred_buf_q3: *const i16,
    dst: *mut u8,
    dst_stride: i32,
    alpha_q3: i32,
) {
    cfl_predict_lbd_ssse3(pred_buf_q3, dst, dst_stride, alpha_q3, 8, 32);
}

#[target_feature(enable = "ssse3")]
pub unsafe extern "C-unwind" fn cfl_predict_lbd_16x4_ssse3(
    pred_buf_q3: *const i16,
    dst: *mut u8,
    dst_stride: i32,
    alpha_q3: i32,
) {
    cfl_predict_lbd_ssse3(pred_buf_q3, dst, dst_stride, alpha_q3, 16, 4);
}

#[target_feature(enable = "ssse3")]
pub unsafe extern "C-unwind" fn cfl_predict_lbd_16x8_ssse3(
    pred_buf_q3: *const i16,
    dst: *mut u8,
    dst_stride: i32,
    alpha_q3: i32,
) {
    cfl_predict_lbd_ssse3(pred_buf_q3, dst, dst_stride, alpha_q3, 16, 8);
}

#[target_feature(enable = "ssse3")]
pub unsafe extern "C-unwind" fn cfl_predict_lbd_16x16_ssse3(
    pred_buf_q3: *const i16,
    dst: *mut u8,
    dst_stride: i32,
    alpha_q3: i32,
) {
    cfl_predict_lbd_ssse3(pred_buf_q3, dst, dst_stride, alpha_q3, 16, 16);
}

#[target_feature(enable = "ssse3")]
pub unsafe extern "C-unwind" fn cfl_predict_lbd_16x32_ssse3(
    pred_buf_q3: *const i16,
    dst: *mut u8,
    dst_stride: i32,
    alpha_q3: i32,
) {
    cfl_predict_lbd_ssse3(pred_buf_q3, dst, dst_stride, alpha_q3, 16, 32);
}

#[target_feature(enable = "ssse3")]
pub unsafe extern "C-unwind" fn cfl_predict_lbd_32x8_ssse3(
    pred_buf_q3: *const i16,
    dst: *mut u8,
    dst_stride: i32,
    alpha_q3: i32,
) {
    cfl_predict_lbd_ssse3(pred_buf_q3, dst, dst_stride, alpha_q3, 32, 8);
}

#[target_feature(enable = "ssse3")]
pub unsafe extern "C-unwind" fn cfl_predict_lbd_32x16_ssse3(
    pred_buf_q3: *const i16,
    dst: *mut u8,
    dst_stride: i32,
    alpha_q3: i32,
) {
    cfl_predict_lbd_ssse3(pred_buf_q3, dst, dst_stride, alpha_q3, 32, 16);
}

#[target_feature(enable = "ssse3")]
pub unsafe extern "C-unwind" fn cfl_predict_lbd_32x32_ssse3(
    pred_buf_q3: *const i16,
    dst: *mut u8,
    dst_stride: i32,
    alpha_q3: i32,
) {
    cfl_predict_lbd_ssse3(pred_buf_q3, dst, dst_stride, alpha_q3, 32, 32);
}

#[no_mangle]
#[target_feature(enable = "ssse3")]
pub unsafe extern "C-unwind" fn cfl_get_predict_lbd_fn_ssse3(tx_size: u8) -> cfl_predict_lbd_fn {
    let mut pred: [cfl_predict_lbd_fn; (TX_SIZES_ALL) as usize] = [
        Some(cfl_predict_lbd_4x4_ssse3),
        Some(cfl_predict_lbd_8x8_ssse3),
        Some(cfl_predict_lbd_16x16_ssse3),
        Some(cfl_predict_lbd_32x32_ssse3),
        None,
        Some(cfl_predict_lbd_4x8_ssse3),
        Some(cfl_predict_lbd_8x4_ssse3),
        Some(cfl_predict_lbd_8x16_ssse3),
        Some(cfl_predict_lbd_16x8_ssse3),
        Some(cfl_predict_lbd_16x32_ssse3),
        Some(cfl_predict_lbd_32x16_ssse3),
        None,
        None,
        Some(cfl_predict_lbd_4x16_ssse3),
        Some(cfl_predict_lbd_16x4_ssse3),
        Some(cfl_predict_lbd_8x32_ssse3),
        Some(cfl_predict_lbd_32x8_ssse3),
        None,
        None,
    ];
    return pred[(tx_size as i32 % TX_SIZES_ALL) as usize];
}

#[inline]
#[target_feature(enable = "ssse3")]
unsafe fn highbd_max_epi16(bd: i32) -> __m128i {
    let neg_one: __m128i = _mm_set1_epi16(-1);
    // (1 << bd) - 1 => -(-1 << bd) -1 => -1 - (-1 << bd) => -1 ^ (-1 << bd)
    return _mm_xor_si128(_mm_sll_epi16(neg_one, _mm_cvtsi32_si128(bd)), neg_one);
}

#[inline]
#[target_feature(enable = "ssse3")]
unsafe fn highbd_clamp_epi16(u: __m128i, zero: __m128i, max: __m128i) -> __m128i {
    return _mm_max_epi16(_mm_min_epi16(u, max), zero);
}

#[inline]
#[target_feature(enable = "ssse3")]
pub unsafe extern "C-unwind" fn cfl_predict_hbd_ssse3(
    pred_buf_q3: *const i16,
    mut dst: *mut u16,
    dst_stride: i32,
    alpha_q3: i32,
    bd: i32,
    width: i32,
    height: i32,
) {
    let alpha_sign: __m128i = _mm_set1_epi16(alpha_q3 as i16);
    let alpha_q12: __m128i = _mm_slli_epi16::<{ 9 }>(_mm_abs_epi16(alpha_sign));
    let dc_q0: __m128i = _mm_set1_epi16(*dst as i16);
    let max: __m128i = highbd_max_epi16(bd);
    let zeros: __m128i = _mm_setzero_si128();
    let mut row: *mut __m128i = pred_buf_q3 as *mut __m128i;
    let mut row_end: *const __m128i =
        row.offset((height * CFL_BUF_LINE_I128) as isize) as *const __m128i;
    loop {
        let mut res: __m128i =
            predict_unclipped(row as *const __m128i, alpha_q12, alpha_sign, dc_q0);
        res = highbd_clamp_epi16(res, zeros, max);
        if width == 4 {
            _mm_storel_epi64(dst as *mut __m128i, res);
        } else {
            _mm_storeu_si128(dst as *mut __m128i, res);
        }
        if width >= 16 {
            let res_1: __m128i = predict_unclipped(
                row.offset(1) as *const __m128i,
                alpha_q12,
                alpha_sign,
                dc_q0,
            );
            _mm_storeu_si128(
                (dst as *mut __m128i).offset(1),
                highbd_clamp_epi16(res_1, zeros, max),
            );
        }
        if width == 32 {
            let res_2: __m128i = predict_unclipped(
                row.offset(2) as *const __m128i,
                alpha_q12,
                alpha_sign,
                dc_q0,
            );
            _mm_storeu_si128(
                dst.offset(16) as *mut __m128i,
                highbd_clamp_epi16(res_2, zeros, max),
            );
            let res_3: __m128i = predict_unclipped(
                row.offset(3) as *const __m128i,
                alpha_q12,
                alpha_sign,
                dc_q0,
            );
            _mm_storeu_si128(
                dst.offset(24) as *mut __m128i,
                highbd_clamp_epi16(res_3, zeros, max),
            );
        }
        dst = dst.offset(dst_stride as isize);
        row = row.offset(CFL_BUF_LINE_I128 as isize);
        if !(row < row_end as *mut __m128i) {
            break;
        }
    }
}

#[target_feature(enable = "ssse3")]
pub unsafe extern "C-unwind" fn cfl_predict_hbd_4x4_ssse3(
    pred_buf_q3: *const i16,
    dst: *mut u16,
    dst_stride: i32,
    alpha_q3: i32,
    bd: i32,
) {
    cfl_predict_hbd_ssse3(pred_buf_q3, dst, dst_stride, alpha_q3, bd, 4, 4);
}

#[target_feature(enable = "ssse3")]
pub unsafe extern "C-unwind" fn cfl_predict_hbd_4x8_ssse3(
    pred_buf_q3: *const i16,
    dst: *mut u16,
    dst_stride: i32,
    alpha_q3: i32,
    bd: i32,
) {
    cfl_predict_hbd_ssse3(pred_buf_q3, dst, dst_stride, alpha_q3, bd, 4, 8);
}

#[target_feature(enable = "ssse3")]
pub unsafe extern "C-unwind" fn cfl_predict_hbd_4x16_ssse3(
    pred_buf_q3: *const i16,
    dst: *mut u16,
    dst_stride: i32,
    alpha_q3: i32,
    bd: i32,
) {
    cfl_predict_hbd_ssse3(pred_buf_q3, dst, dst_stride, alpha_q3, bd, 4, 16);
}

#[target_feature(enable = "ssse3")]
pub unsafe extern "C-unwind" fn cfl_predict_hbd_8x4_ssse3(
    pred_buf_q3: *const i16,
    dst: *mut u16,
    dst_stride: i32,
    alpha_q3: i32,
    bd: i32,
) {
    cfl_predict_hbd_ssse3(pred_buf_q3, dst, dst_stride, alpha_q3, bd, 8, 4);
}

#[target_feature(enable = "ssse3")]
pub unsafe extern "C-unwind" fn cfl_predict_hbd_8x8_ssse3(
    pred_buf_q3: *const i16,
    dst: *mut u16,
    dst_stride: i32,
    alpha_q3: i32,
    bd: i32,
) {
    cfl_predict_hbd_ssse3(pred_buf_q3, dst, dst_stride, alpha_q3, bd, 8, 8);
}

#[target_feature(enable = "ssse3")]
pub unsafe extern "C-unwind" fn cfl_predict_hbd_8x16_ssse3(
    pred_buf_q3: *const i16,
    dst: *mut u16,
    dst_stride: i32,
    alpha_q3: i32,
    bd: i32,
) {
    cfl_predict_hbd_ssse3(pred_buf_q3, dst, dst_stride, alpha_q3, bd, 8, 16);
}

#[target_feature(enable = "ssse3")]
pub unsafe extern "C-unwind" fn cfl_predict_hbd_8x32_ssse3(
    pred_buf_q3: *const i16,
    dst: *mut u16,
    dst_stride: i32,
    alpha_q3: i32,
    bd: i32,
) {
    cfl_predict_hbd_ssse3(pred_buf_q3, dst, dst_stride, alpha_q3, bd, 8, 32);
}

#[target_feature(enable = "ssse3")]
pub unsafe extern "C-unwind" fn cfl_predict_hbd_16x4_ssse3(
    pred_buf_q3: *const i16,
    dst: *mut u16,
    dst_stride: i32,
    alpha_q3: i32,
    bd: i32,
) {
    cfl_predict_hbd_ssse3(pred_buf_q3, dst, dst_stride, alpha_q3, bd, 16, 4);
}

#[target_feature(enable = "ssse3")]
pub unsafe extern "C-unwind" fn cfl_predict_hbd_16x8_ssse3(
    pred_buf_q3: *const i16,
    dst: *mut u16,
    dst_stride: i32,
    alpha_q3: i32,
    bd: i32,
) {
    cfl_predict_hbd_ssse3(pred_buf_q3, dst, dst_stride, alpha_q3, bd, 16, 8);
}

#[target_feature(enable = "ssse3")]
pub unsafe extern "C-unwind" fn cfl_predict_hbd_16x16_ssse3(
    pred_buf_q3: *const i16,
    dst: *mut u16,
    dst_stride: i32,
    alpha_q3: i32,
    bd: i32,
) {
    cfl_predict_hbd_ssse3(pred_buf_q3, dst, dst_stride, alpha_q3, bd, 16, 16);
}

#[target_feature(enable = "ssse3")]
pub unsafe extern "C-unwind" fn cfl_predict_hbd_16x32_ssse3(
    pred_buf_q3: *const i16,
    dst: *mut u16,
    dst_stride: i32,
    alpha_q3: i32,
    bd: i32,
) {
    cfl_predict_hbd_ssse3(pred_buf_q3, dst, dst_stride, alpha_q3, bd, 16, 32);
}

#[target_feature(enable = "ssse3")]
pub unsafe extern "C-unwind" fn cfl_predict_hbd_32x8_ssse3(
    pred_buf_q3: *const i16,
    dst: *mut u16,
    dst_stride: i32,
    alpha_q3: i32,
    bd: i32,
) {
    cfl_predict_hbd_ssse3(pred_buf_q3, dst, dst_stride, alpha_q3, bd, 32, 8);
}

#[target_feature(enable = "ssse3")]
pub unsafe extern "C-unwind" fn cfl_predict_hbd_32x16_ssse3(
    pred_buf_q3: *const i16,
    dst: *mut u16,
    dst_stride: i32,
    alpha_q3: i32,
    bd: i32,
) {
    cfl_predict_hbd_ssse3(pred_buf_q3, dst, dst_stride, alpha_q3, bd, 32, 16);
}

#[target_feature(enable = "ssse3")]
pub unsafe extern "C-unwind" fn cfl_predict_hbd_32x32_ssse3(
    pred_buf_q3: *const i16,
    dst: *mut u16,
    dst_stride: i32,
    alpha_q3: i32,
    bd: i32,
) {
    cfl_predict_hbd_ssse3(pred_buf_q3, dst, dst_stride, alpha_q3, bd, 32, 32);
}

#[no_mangle]
#[target_feature(enable = "ssse3")]
pub unsafe extern "C-unwind" fn cfl_get_predict_hbd_fn_ssse3(tx_size: u8) -> cfl_predict_hbd_fn {
    let mut pred: [cfl_predict_hbd_fn; (TX_SIZES_ALL) as usize] = [
        Some(cfl_predict_hbd_4x4_ssse3),
        Some(cfl_predict_hbd_8x8_ssse3),
        Some(cfl_predict_hbd_16x16_ssse3),
        Some(cfl_predict_hbd_32x32_ssse3),
        None,
        Some(cfl_predict_hbd_4x8_ssse3),
        Some(cfl_predict_hbd_8x4_ssse3),
        Some(cfl_predict_hbd_8x16_ssse3),
        Some(cfl_predict_hbd_16x8_ssse3),
        Some(cfl_predict_hbd_16x32_ssse3),
        Some(cfl_predict_hbd_32x16_ssse3),
        None,
        None,
        Some(cfl_predict_hbd_4x16_ssse3),
        Some(cfl_predict_hbd_16x4_ssse3),
        Some(cfl_predict_hbd_8x32_ssse3),
        Some(cfl_predict_hbd_32x8_ssse3),
        None,
        None,
    ];
    return pred[(tx_size as i32 % TX_SIZES_ALL) as usize];
}
