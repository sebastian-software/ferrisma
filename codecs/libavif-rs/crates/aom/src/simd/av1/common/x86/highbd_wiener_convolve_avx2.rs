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

// Translated from upstream/aom/av1/common/x86/highbd_wiener_convolve_avx2.c (ADR 0056, ADR 0058).
// Upstream compiles it with -mavx2. The functions were
// transliterated with a typing-aware C-subset printer (not C2Rust) and
// reviewed against the C function by function: the same sequence of intrinsic
// and helper calls, constants and operators in order (ADR 0058).
// The restoration and filter macros of av1/common/restoration.h, aom_dsp/aom_filter.h,
// av1/common/enums.h, aom_dsp/aom_dsp_common.h, aom_ports/mem.h and av1/common/convolve.h
// are expanded with the C preprocessor before printing (tools/x86_c2r/c2r.py `cpp`).
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

use crate::av1__common__convolve::{
    av1_highbd_wiener_convolve_add_src_c, av1_wiener_convolve_add_src_c, WienerConvolveParams,
};
use crate::simd::aom_dsp__x86__convolve_avx2::*;
use crate::simd::aom_dsp__x86__synonyms::*;
use crate::simd::aom_dsp__x86__synonyms_avx2::*;
use core::arch::x86_64::*;

// 128-bit xmmwords are written as [ ... ] with the MSB on the left.
// 256-bit ymmwords are written as two xmmwords, [ ... ][ ... ] with the MSB
// on the left.
// A row of, say, 16-bit pixels with values p0, p1, p2, ..., p14, p15 will be
// loaded and stored as [ p15 ... p9 p8 ][ p7 ... p1 p0 ].
#[no_mangle]
#[target_feature(enable = "avx2")]
pub unsafe extern "C-unwind" fn av1_highbd_wiener_convolve_add_src_avx2(
    src8: *const u8,
    src_stride: isize,
    dst8: *mut u8,
    dst_stride: isize,
    filter_x: *const i16,
    x_step_q4: i32,
    filter_y: *const i16,
    y_step_q4: i32,
    w: i32,
    h: i32,
    conv_params: *const WienerConvolveParams,
    bd: i32,
) {
    debug_assert!(x_step_q4 == 16 && y_step_q4 == 16);
    debug_assert!(!(w & 7 != 0));
    debug_assert!(bd + 7 - (*conv_params).round_0 + 2 <= 16);
    let _ = x_step_q4;
    let _ = y_step_q4;
    let mut src: *const u16 = ((src8 as usize) << 1) as *mut u16 as *const u16;
    let mut dst: *mut u16 = ((dst8 as usize) << 1) as *mut u16;
    let mut temp: Aligned32<[u16; (((1 << 7) + 8 - 1) * (1 << 7)) as usize]> = core::mem::zeroed();
    let mut intermediate_height: i32 = h + 8 - 1;
    let center_tap: i32 = (8 - 1) / 2;
    let mut src_ptr: *const u16 = src
        .offset(-(center_tap as isize * src_stride))
        .offset(-(center_tap as isize));
    let zero_128: __m128i = _mm_setzero_si128();
    let zero_256: __m256i = _mm256_setzero_si256();
    let offset: __m128i = _mm_insert_epi16::<3>(zero_128, 1 << 7);
    // Add an offset to account for the "add_src" part of the convolve function.
    let clamp_low: __m256i = zero_256;
    {
        // Horizontal filter
        let clamp_high_ep: __m256i =
            _mm256_set1_epi16(((1 << (bd + 1 + 7 - (*conv_params).round_0)) - 1) as i16);
        let coeffs_x: __m128i =
            _mm_add_epi16(xx_loadu_128(filter_x as *const core::ffi::c_void), offset);
        // coeffs [ f7 f6 f5 f4 f3 f2 f1 f0 ]
        let coeffs_0123: __m128i = _mm_unpacklo_epi32(coeffs_x, coeffs_x);
        // coeffs [ f3 f2 f3 f2 f1 f0 f1 f0 ]
        let coeffs_4567: __m128i = _mm_unpackhi_epi32(coeffs_x, coeffs_x);
        // coeffs [ f7 f6 f7 f6 f5 f4 f5 f4 ]
        let coeffs_01_128: __m128i = _mm_unpacklo_epi64(coeffs_0123, coeffs_0123);
        // coeffs [ f1 f0 f1 f0 f1 f0 f1 f0 ]
        let coeffs_23_128: __m128i = _mm_unpackhi_epi64(coeffs_0123, coeffs_0123);
        // coeffs [ f3 f2 f3 f2 f3 f2 f3 f2 ]
        let coeffs_45_128: __m128i = _mm_unpacklo_epi64(coeffs_4567, coeffs_4567);
        // coeffs [ f5 f4 f5 f4 f5 f4 f5 f4 ]
        let coeffs_67_128: __m128i = _mm_unpackhi_epi64(coeffs_4567, coeffs_4567);
        // coeffs [ f7 f6 f7 f6 f7 f6 f7 f6 ]
        let coeffs_01: __m256i = yy_set_m128i(coeffs_01_128, coeffs_01_128);
        // coeffs [ f1 f0 f1 f0 f1 f0 f1 f0 ][ f1 f0 f1 f0 f1 f0 f1 f0 ]
        let coeffs_23: __m256i = yy_set_m128i(coeffs_23_128, coeffs_23_128);
        // coeffs [ f3 f2 f3 f2 f3 f2 f3 f2 ][ f3 f2 f3 f2 f3 f2 f3 f2 ]
        let coeffs_45: __m256i = yy_set_m128i(coeffs_45_128, coeffs_45_128);
        // coeffs [ f5 f4 f5 f4 f5 f4 f5 f4 ][ f5 f4 f5 f4 f5 f4 f5 f4 ]
        let coeffs_67: __m256i = yy_set_m128i(coeffs_67_128, coeffs_67_128);
        // coeffs [ f7 f6 f7 f6 f7 f6 f7 f6 ][ f7 f6 f7 f6 f7 f6 f7 f6 ]
        let round_const: __m256i =
            _mm256_set1_epi32((1 << ((*conv_params).round_0 - 1)) + (1 << (bd + 7 - 1)));
        for i in 0..intermediate_height {
            for j in (0..w).step_by(16) {
                let mut src_ij: *const u16 =
                    src_ptr.offset(i as isize * src_stride).offset(j as isize);
                let src_0: __m256i = yy_loadu_256(src_ij as *const core::ffi::c_void);
                // Load 16-bit src data
                let src_1: __m256i = yy_loadu_256(src_ij.offset(1) as *const core::ffi::c_void);
                let src_2: __m256i = yy_loadu_256(src_ij.offset(2) as *const core::ffi::c_void);
                let src_3: __m256i = yy_loadu_256(src_ij.offset(3) as *const core::ffi::c_void);
                let src_4: __m256i = yy_loadu_256(src_ij.offset(4) as *const core::ffi::c_void);
                let src_5: __m256i = yy_loadu_256(src_ij.offset(5) as *const core::ffi::c_void);
                let src_6: __m256i = yy_loadu_256(src_ij.offset(6) as *const core::ffi::c_void);
                let src_7: __m256i = yy_loadu_256(src_ij.offset(7) as *const core::ffi::c_void);
                let res_0: __m256i = _mm256_madd_epi16(src_0, coeffs_01);
                // Multiply src data by filter coeffs and sum pairs
                let res_1: __m256i = _mm256_madd_epi16(src_1, coeffs_01);
                let res_2: __m256i = _mm256_madd_epi16(src_2, coeffs_23);
                let res_3: __m256i = _mm256_madd_epi16(src_3, coeffs_23);
                let res_4: __m256i = _mm256_madd_epi16(src_4, coeffs_45);
                let res_5: __m256i = _mm256_madd_epi16(src_5, coeffs_45);
                let res_6: __m256i = _mm256_madd_epi16(src_6, coeffs_67);
                let res_7: __m256i = _mm256_madd_epi16(src_7, coeffs_67);
                // Calculate scalar product for even- and odd-indices separately,
                let res_even_sum: __m256i = _mm256_add_epi32(
                    _mm256_add_epi32(res_0, res_4),
                    _mm256_add_epi32(res_2, res_6),
                );
                // increasing to 32-bit precision
                let res_even: __m256i = _mm256_sra_epi32(
                    _mm256_add_epi32(res_even_sum, round_const),
                    _mm_cvtsi32_si128((*conv_params).round_0),
                );
                let res_odd_sum: __m256i = _mm256_add_epi32(
                    _mm256_add_epi32(res_1, res_5),
                    _mm256_add_epi32(res_3, res_7),
                );
                let res_odd: __m256i = _mm256_sra_epi32(
                    _mm256_add_epi32(res_odd_sum, round_const),
                    _mm_cvtsi32_si128((*conv_params).round_0),
                );
                // Reduce to 16-bit precision and pack even- and odd-index results
                // back into one register. The _mm256_packs_epi32 intrinsic returns
                // a register with the pixels ordered as follows:
                let res: __m256i = _mm256_packs_epi32(res_even, res_odd);
                // [ 15 13 11 9 14 12 10 8 ] [ 7 5 3 1 6 4 2 0 ]
                let res_clamped: __m256i =
                    _mm256_min_epi16(_mm256_max_epi16(res, clamp_low), clamp_high_ep);
                yy_storeu_256(
                    temp.0
                        .as_mut_ptr()
                        .offset((i * (1 << 7)) as isize)
                        .offset(j as isize) as *mut core::ffi::c_void,
                    res_clamped,
                );
            }
        }
    }
    // Store in a temporary array
    {
        // Vertical filter
        let clamp_high: __m256i = _mm256_set1_epi16(((1 << bd) - 1) as i16);
        let coeffs_y: __m128i =
            _mm_add_epi16(xx_loadu_128(filter_y as *const core::ffi::c_void), offset);
        // coeffs [ f7 f6 f5 f4 f3 f2 f1 f0 ]
        let coeffs_0123: __m128i = _mm_unpacklo_epi32(coeffs_y, coeffs_y);
        // coeffs [ f3 f2 f3 f2 f1 f0 f1 f0 ]
        let coeffs_4567: __m128i = _mm_unpackhi_epi32(coeffs_y, coeffs_y);
        // coeffs [ f7 f6 f7 f6 f5 f4 f5 f4 ]
        let coeffs_01_128: __m128i = _mm_unpacklo_epi64(coeffs_0123, coeffs_0123);
        // coeffs [ f1 f0 f1 f0 f1 f0 f1 f0 ]
        let coeffs_23_128: __m128i = _mm_unpackhi_epi64(coeffs_0123, coeffs_0123);
        // coeffs [ f3 f2 f3 f2 f3 f2 f3 f2 ]
        let coeffs_45_128: __m128i = _mm_unpacklo_epi64(coeffs_4567, coeffs_4567);
        // coeffs [ f5 f4 f5 f4 f5 f4 f5 f4 ]
        let coeffs_67_128: __m128i = _mm_unpackhi_epi64(coeffs_4567, coeffs_4567);
        // coeffs [ f7 f6 f7 f6 f7 f6 f7 f6 ]
        let coeffs_01: __m256i = yy_set_m128i(coeffs_01_128, coeffs_01_128);
        // coeffs [ f1 f0 f1 f0 f1 f0 f1 f0 ][ f1 f0 f1 f0 f1 f0 f1 f0 ]
        let coeffs_23: __m256i = yy_set_m128i(coeffs_23_128, coeffs_23_128);
        // coeffs [ f3 f2 f3 f2 f3 f2 f3 f2 ][ f3 f2 f3 f2 f3 f2 f3 f2 ]
        let coeffs_45: __m256i = yy_set_m128i(coeffs_45_128, coeffs_45_128);
        // coeffs [ f5 f4 f5 f4 f5 f4 f5 f4 ][ f5 f4 f5 f4 f5 f4 f5 f4 ]
        let coeffs_67: __m256i = yy_set_m128i(coeffs_67_128, coeffs_67_128);
        // coeffs [ f7 f6 f7 f6 f7 f6 f7 f6 ][ f7 f6 f7 f6 f7 f6 f7 f6 ]
        let round_const: __m256i = _mm256_set1_epi32(
            (1 << ((*conv_params).round_1 - 1)) - (1 << (bd + (*conv_params).round_1 - 1)),
        );
        for i in 0..h {
            for j in (0..w).step_by(16) {
                let mut temp_ij: *const u16 = temp
                    .0
                    .as_mut_ptr()
                    .offset((i * (1 << 7)) as isize)
                    .offset(j as isize) as *const u16;
                // Load 16-bit data from the output of the horizontal filter in
                // which the pixels are ordered as follows:
                let data_0: __m256i = yy_loadu_256(
                    temp_ij.offset((0 * (1 << 7)) as isize) as *const core::ffi::c_void
                );
                // [ 15 13 11 9 14 12 10 8 ] [ 7 5 3 1 6 4 2 0 ]
                let data_1: __m256i = yy_loadu_256(
                    temp_ij.offset((1 * (1 << 7)) as isize) as *const core::ffi::c_void
                );
                let data_2: __m256i = yy_loadu_256(
                    temp_ij.offset((2 * (1 << 7)) as isize) as *const core::ffi::c_void
                );
                let data_3: __m256i = yy_loadu_256(
                    temp_ij.offset((3 * (1 << 7)) as isize) as *const core::ffi::c_void
                );
                let data_4: __m256i = yy_loadu_256(
                    temp_ij.offset((4 * (1 << 7)) as isize) as *const core::ffi::c_void
                );
                let data_5: __m256i = yy_loadu_256(
                    temp_ij.offset((5 * (1 << 7)) as isize) as *const core::ffi::c_void
                );
                let data_6: __m256i = yy_loadu_256(
                    temp_ij.offset((6 * (1 << 7)) as isize) as *const core::ffi::c_void
                );
                let data_7: __m256i = yy_loadu_256(
                    temp_ij.offset((7 * (1 << 7)) as isize) as *const core::ffi::c_void
                );
                let src_0: __m256i = _mm256_unpacklo_epi16(data_0, data_1);
                // Filter the even-indices, increasing to 32-bit precision
                let src_2: __m256i = _mm256_unpacklo_epi16(data_2, data_3);
                let src_4: __m256i = _mm256_unpacklo_epi16(data_4, data_5);
                let src_6: __m256i = _mm256_unpacklo_epi16(data_6, data_7);
                let res_0: __m256i = _mm256_madd_epi16(src_0, coeffs_01);
                let res_2: __m256i = _mm256_madd_epi16(src_2, coeffs_23);
                let res_4: __m256i = _mm256_madd_epi16(src_4, coeffs_45);
                let res_6: __m256i = _mm256_madd_epi16(src_6, coeffs_67);
                let res_even: __m256i = _mm256_add_epi32(
                    _mm256_add_epi32(res_0, res_2),
                    _mm256_add_epi32(res_4, res_6),
                );
                let src_1: __m256i = _mm256_unpackhi_epi16(data_0, data_1);
                // Filter the odd-indices, increasing to 32-bit precision
                let src_3: __m256i = _mm256_unpackhi_epi16(data_2, data_3);
                let src_5: __m256i = _mm256_unpackhi_epi16(data_4, data_5);
                let src_7: __m256i = _mm256_unpackhi_epi16(data_6, data_7);
                let res_1: __m256i = _mm256_madd_epi16(src_1, coeffs_01);
                let res_3: __m256i = _mm256_madd_epi16(src_3, coeffs_23);
                let res_5: __m256i = _mm256_madd_epi16(src_5, coeffs_45);
                let res_7: __m256i = _mm256_madd_epi16(src_7, coeffs_67);
                let res_odd: __m256i = _mm256_add_epi32(
                    _mm256_add_epi32(res_1, res_3),
                    _mm256_add_epi32(res_5, res_7),
                );
                // Pixels are currently in the following order:
                // res_even order: [ 14 12 10 8 ] [ 6 4 2 0 ]
                // res_odd order:  [ 15 13 11 9 ] [ 7 5 3 1 ]
                //
                // Rearrange the pixels into the following order:
                // res_lo order: [ 11 10  9  8 ] [ 3 2 1 0 ]
                let res_lo: __m256i = _mm256_unpacklo_epi32(res_even, res_odd);
                // res_hi order: [ 15 14 13 12 ] [ 7 6 5 4 ]
                let res_hi: __m256i = _mm256_unpackhi_epi32(res_even, res_odd);
                let res_lo_round: __m256i = _mm256_sra_epi32(
                    _mm256_add_epi32(res_lo, round_const),
                    _mm_cvtsi32_si128((*conv_params).round_1),
                );
                let res_hi_round: __m256i = _mm256_sra_epi32(
                    _mm256_add_epi32(res_hi, round_const),
                    _mm_cvtsi32_si128((*conv_params).round_1),
                );
                // Reduce to 16-bit precision and pack into the correct order:
                let res_16bit: __m256i = _mm256_packs_epi32(res_lo_round, res_hi_round);
                // [ 15 14 13 12 11 10 9 8 ][ 7 6 5 4 3 2 1 0 ]
                let res_16bit_clamped: __m256i =
                    _mm256_min_epi16(_mm256_max_epi16(res_16bit, clamp_low), clamp_high);
                yy_storeu_256(
                    dst.offset(i as isize * dst_stride).offset(j as isize)
                        as *mut core::ffi::c_void,
                    res_16bit_clamped,
                );
            }
        }
    }
}

// Store in the dst array
