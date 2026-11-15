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

// Translated from upstream/aom/av1/common/x86/highbd_wiener_convolve_ssse3.c (ADR 0056, ADR 0058).
// Upstream compiles it with -mssse3. The functions were
// transliterated with a typing-aware C-subset printer (not C2Rust) and
// reviewed against the C function by function: the same sequence of intrinsic
// and helper calls, constants and operators in order (ADR 0058).
// The restoration and filter macros of av1/common/restoration.h, aom_dsp/aom_filter.h,
// av1/common/enums.h, aom_dsp/aom_dsp_common.h, aom_ports/mem.h and av1/common/convolve.h
// are expanded with the C preprocessor before printing (tools/x86_c2r/c2r.py `cpp`).
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

use crate::av1__common__convolve::{
    av1_highbd_wiener_convolve_add_src_c, av1_wiener_convolve_add_src_c, WienerConvolveParams,
};
use crate::simd::aom_dsp__x86__synonyms::*;
use core::arch::x86_64::*;

#[repr(C, align(16))]
struct Aligned16<T>(T);

#[no_mangle]
#[target_feature(enable = "ssse3")]
pub unsafe extern "C-unwind" fn av1_highbd_wiener_convolve_add_src_ssse3(
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
    let mut temp: Aligned16<[u16; (((1 << 7) + 8 - 1) * (1 << 7)) as usize]> = core::mem::zeroed();
    let mut intermediate_height: i32 = h + 8 - 1;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let center_tap: i32 = (8 - 1) / 2;
    let mut src_ptr: *const u16 = src
        .offset(-(center_tap as isize * src_stride))
        .offset(-(center_tap as isize));
    let zero: __m128i = _mm_setzero_si128();
    let offset: __m128i = _mm_insert_epi16::<3>(zero, 1 << 7);
    // Add an offset to account for the "add_src" part of the convolve function.
    {
        // Horizontal filter
        let coeffs_x: __m128i = _mm_add_epi16(
            _mm_loadu_si128(filter_x as *mut __m128i as *const __m128i),
            offset,
        );
        let tmp_0: __m128i = _mm_unpacklo_epi32(coeffs_x, coeffs_x);
        // coeffs 0 1 0 1 2 3 2 3
        let tmp_1: __m128i = _mm_unpackhi_epi32(coeffs_x, coeffs_x);
        // coeffs 4 5 4 5 6 7 6 7
        let coeff_01: __m128i = _mm_unpacklo_epi64(tmp_0, tmp_0);
        // coeffs 0 1 0 1 0 1 0 1
        let coeff_23: __m128i = _mm_unpackhi_epi64(tmp_0, tmp_0);
        // coeffs 2 3 2 3 2 3 2 3
        let coeff_45: __m128i = _mm_unpacklo_epi64(tmp_1, tmp_1);
        // coeffs 4 5 4 5 4 5 4 5
        let coeff_67: __m128i = _mm_unpackhi_epi64(tmp_1, tmp_1);
        // coeffs 6 7 6 7 6 7 6 7
        let round_const: __m128i =
            _mm_set1_epi32((1 << ((*conv_params).round_0 - 1)) + (1 << (bd + 7 - 1)));
        {
            i = 0;
            while i < intermediate_height {
                {
                    j = 0;
                    while j < w {
                        let data: __m128i =
                            _mm_loadu_si128(src_ptr.offset(i as isize * src_stride + j as isize)
                                as *mut __m128i
                                as *const __m128i);
                        let data2: __m128i = _mm_loadu_si128(
                            src_ptr.offset(i as isize * src_stride + j as isize + 8) as *mut __m128i
                                as *const __m128i,
                        );
                        let res_0: __m128i = _mm_madd_epi16(data, coeff_01);
                        // Filter even-index pixels
                        let res_2: __m128i =
                            _mm_madd_epi16(_mm_alignr_epi8::<4>(data2, data), coeff_23);
                        let res_4: __m128i =
                            _mm_madd_epi16(_mm_alignr_epi8::<8>(data2, data), coeff_45);
                        let res_6: __m128i =
                            _mm_madd_epi16(_mm_alignr_epi8::<12>(data2, data), coeff_67);
                        let mut res_even: __m128i =
                            _mm_add_epi32(_mm_add_epi32(res_0, res_4), _mm_add_epi32(res_2, res_6));
                        res_even = _mm_sra_epi32(
                            _mm_add_epi32(res_even, round_const),
                            _mm_cvtsi32_si128((*conv_params).round_0),
                        );
                        let res_1: __m128i =
                            _mm_madd_epi16(_mm_alignr_epi8::<2>(data2, data), coeff_01);
                        // Filter odd-index pixels
                        let res_3: __m128i =
                            _mm_madd_epi16(_mm_alignr_epi8::<6>(data2, data), coeff_23);
                        let res_5: __m128i =
                            _mm_madd_epi16(_mm_alignr_epi8::<10>(data2, data), coeff_45);
                        let res_7: __m128i =
                            _mm_madd_epi16(_mm_alignr_epi8::<14>(data2, data), coeff_67);
                        let mut res_odd: __m128i =
                            _mm_add_epi32(_mm_add_epi32(res_1, res_5), _mm_add_epi32(res_3, res_7));
                        res_odd = _mm_sra_epi32(
                            _mm_add_epi32(res_odd, round_const),
                            _mm_cvtsi32_si128((*conv_params).round_0),
                        );
                        let maxval: __m128i = _mm_set1_epi16(
                            ((1 << (bd + 1 + 7 - (*conv_params).round_0)) - 1) as i16,
                        );
                        // Pack in the column order 0, 2, 4, 6, 1, 3, 5, 7
                        let mut res: __m128i = _mm_packs_epi32(res_even, res_odd);
                        res = _mm_min_epi16(_mm_max_epi16(res, zero), maxval);
                        _mm_storeu_si128(
                            temp.0.as_mut_ptr().offset((i * (1 << 7) + j) as isize) as *mut __m128i,
                            res,
                        );
                        j += 8;
                    }
                }
                i += 1;
            }
        }
    }
    {
        // Vertical filter
        let coeffs_y: __m128i = _mm_add_epi16(
            _mm_loadu_si128(filter_y as *mut __m128i as *const __m128i),
            offset,
        );
        let tmp_0: __m128i = _mm_unpacklo_epi32(coeffs_y, coeffs_y);
        // coeffs 0 1 0 1 2 3 2 3
        let tmp_1: __m128i = _mm_unpackhi_epi32(coeffs_y, coeffs_y);
        // coeffs 4 5 4 5 6 7 6 7
        let coeff_01: __m128i = _mm_unpacklo_epi64(tmp_0, tmp_0);
        // coeffs 0 1 0 1 0 1 0 1
        let coeff_23: __m128i = _mm_unpackhi_epi64(tmp_0, tmp_0);
        // coeffs 2 3 2 3 2 3 2 3
        let coeff_45: __m128i = _mm_unpacklo_epi64(tmp_1, tmp_1);
        // coeffs 4 5 4 5 4 5 4 5
        let coeff_67: __m128i = _mm_unpackhi_epi64(tmp_1, tmp_1);
        // coeffs 6 7 6 7 6 7 6 7
        let round_const: __m128i = _mm_set1_epi32(
            (1 << ((*conv_params).round_1 - 1)) - (1 << (bd + (*conv_params).round_1 - 1)),
        );
        {
            i = 0;
            while i < h {
                {
                    j = 0;
                    while j < w {
                        let mut data: *const u16 =
                            temp.0.as_mut_ptr().offset((i * (1 << 7) + j) as isize) as *const u16;
                        // Filter even-index pixels
                        let src_0: __m128i = _mm_unpacklo_epi16(
                            *(data.offset((0 * (1 << 7)) as isize) as *mut __m128i),
                            *(data.offset((1 * (1 << 7)) as isize) as *mut __m128i),
                        );
                        let src_2: __m128i = _mm_unpacklo_epi16(
                            *(data.offset((2 * (1 << 7)) as isize) as *mut __m128i),
                            *(data.offset((3 * (1 << 7)) as isize) as *mut __m128i),
                        );
                        let src_4: __m128i = _mm_unpacklo_epi16(
                            *(data.offset((4 * (1 << 7)) as isize) as *mut __m128i),
                            *(data.offset((5 * (1 << 7)) as isize) as *mut __m128i),
                        );
                        let src_6: __m128i = _mm_unpacklo_epi16(
                            *(data.offset((6 * (1 << 7)) as isize) as *mut __m128i),
                            *(data.offset((7 * (1 << 7)) as isize) as *mut __m128i),
                        );
                        let res_0: __m128i = _mm_madd_epi16(src_0, coeff_01);
                        let res_2: __m128i = _mm_madd_epi16(src_2, coeff_23);
                        let res_4: __m128i = _mm_madd_epi16(src_4, coeff_45);
                        let res_6: __m128i = _mm_madd_epi16(src_6, coeff_67);
                        let res_even: __m128i =
                            _mm_add_epi32(_mm_add_epi32(res_0, res_2), _mm_add_epi32(res_4, res_6));
                        let src_1: __m128i = _mm_unpackhi_epi16(
                            *(data.offset((0 * (1 << 7)) as isize) as *mut __m128i),
                            *(data.offset((1 * (1 << 7)) as isize) as *mut __m128i),
                        );
                        // Filter odd-index pixels
                        let src_3: __m128i = _mm_unpackhi_epi16(
                            *(data.offset((2 * (1 << 7)) as isize) as *mut __m128i),
                            *(data.offset((3 * (1 << 7)) as isize) as *mut __m128i),
                        );
                        let src_5: __m128i = _mm_unpackhi_epi16(
                            *(data.offset((4 * (1 << 7)) as isize) as *mut __m128i),
                            *(data.offset((5 * (1 << 7)) as isize) as *mut __m128i),
                        );
                        let src_7: __m128i = _mm_unpackhi_epi16(
                            *(data.offset((6 * (1 << 7)) as isize) as *mut __m128i),
                            *(data.offset((7 * (1 << 7)) as isize) as *mut __m128i),
                        );
                        let res_1: __m128i = _mm_madd_epi16(src_1, coeff_01);
                        let res_3: __m128i = _mm_madd_epi16(src_3, coeff_23);
                        let res_5: __m128i = _mm_madd_epi16(src_5, coeff_45);
                        let res_7: __m128i = _mm_madd_epi16(src_7, coeff_67);
                        let res_odd: __m128i =
                            _mm_add_epi32(_mm_add_epi32(res_1, res_3), _mm_add_epi32(res_5, res_7));
                        let res_lo: __m128i = _mm_unpacklo_epi32(res_even, res_odd);
                        // Rearrange pixels back into the order 0 ... 7
                        let res_hi: __m128i = _mm_unpackhi_epi32(res_even, res_odd);
                        let res_lo_round: __m128i = _mm_sra_epi32(
                            _mm_add_epi32(res_lo, round_const),
                            _mm_cvtsi32_si128((*conv_params).round_1),
                        );
                        let res_hi_round: __m128i = _mm_sra_epi32(
                            _mm_add_epi32(res_hi, round_const),
                            _mm_cvtsi32_si128((*conv_params).round_1),
                        );
                        let maxval: __m128i = _mm_set1_epi16(((1 << bd) - 1) as i16);
                        let mut res_16bit: __m128i = _mm_packs_epi32(res_lo_round, res_hi_round);
                        res_16bit = _mm_min_epi16(_mm_max_epi16(res_16bit, zero), maxval);
                        let mut p: *mut __m128i =
                            dst.offset(i as isize * dst_stride + j as isize) as *mut __m128i;
                        _mm_storeu_si128(p, res_16bit);
                        j += 8;
                    }
                }
                i += 1;
            }
        }
    }
}
