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

// Translated from upstream/aom/av1/common/x86/wiener_convolve_avx2.c (ADR 0056, ADR 0058).
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
// A row of, say, 8-bit pixels with values p0, p1, p2, ..., p30, p31 will be
// loaded and stored as [ p31 ... p17 p16 ][ p15 ... p1 p0 ].
// Exploiting the range of wiener filter coefficients,
// horizontal filtering can be done in 16 bit intermediate precision.
// The details are as follows :
// Consider the horizontal wiener filter coefficients of the following form :
//      [C0, C1, C2, 2^(FILTER_BITS) -2 * (C0 + C1 + C2), C2, C1, C0]
// Subtracting  2^(FILTER_BITS) from the centre tap we get the following  :
//      [C0, C1, C2,     -2 * (C0 + C1 + C2),             C2, C1, C0]
// The sum of the product "C0 * p0 + C1 * p1 + C2 * p2 -2 * (C0 + C1 + C2) * p3
// + C2 * p4 + C1 * p5 + C0 * p6" would be in the range of signed 16 bit
// precision. Finally, after rounding the above result by round_0, we multiply
// the centre pixel by 2^(FILTER_BITS - round_0) and add it to get the
// horizontal filter output.
#[no_mangle]
#[target_feature(enable = "avx2")]
pub unsafe extern "C-unwind" fn av1_wiener_convolve_add_src_avx2(
    src: *const u8,
    src_stride: isize,
    dst: *mut u8,
    dst_stride: isize,
    filter_x: *const i16,
    x_step_q4: i32,
    filter_y: *const i16,
    y_step_q4: i32,
    w: i32,
    h: i32,
    conv_params: *const WienerConvolveParams,
) {
    let bd: i32 = 8;
    debug_assert!(x_step_q4 == 16 && y_step_q4 == 16);
    debug_assert!(!(w & 7 != 0));
    let _ = x_step_q4;
    let _ = y_step_q4;
    let mut im_block: Aligned32<[i16; (((1 << 7) + 8) * 8) as usize]> = core::mem::zeroed();
    let mut im_h: i32 = h + 8 - 2;
    let mut im_stride: i32 = 8;
    core::ptr::write_bytes(
        im_block.0.as_mut_ptr().offset((im_h * im_stride) as isize) as *mut u8,
        0,
        (1 << 7) as usize,
    );
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let center_tap: i32 = (8 - 1) / 2;
    let mut src_ptr: *const u8 = src
        .offset(-(center_tap as isize * src_stride))
        .offset(-(center_tap as isize));
    let mut filt: [__m256i; 4] = core::mem::zeroed();
    let mut coeffs_h: [__m256i; 4] = core::mem::zeroed();
    let mut coeffs_v: [__m256i; 4] = core::mem::zeroed();
    let mut filt_center: __m256i;
    debug_assert!((*conv_params).round_0 > 0);
    filt[0] = _mm256_load_si256(filt1_global_avx2.0.as_ptr() as *const __m256i);
    filt[1] = _mm256_load_si256(filt2_global_avx2.0.as_ptr() as *const __m256i);
    filt[2] = _mm256_load_si256(filt3_global_avx2.0.as_ptr() as *const __m256i);
    filt[3] = _mm256_load_si256(filt4_global_avx2.0.as_ptr() as *const __m256i);
    filt_center = _mm256_load_si256(filt_center_global_avx2.0.as_ptr() as *const __m256i);
    let coeffs_x: __m128i = _mm_loadu_si128(filter_x as *mut __m128i as *const __m128i);
    let filter_coeffs_x: __m256i = _mm256_broadcastsi128_si256(coeffs_x);
    // coeffs 0 1 0 1 0 1 0 1
    coeffs_h[0] = _mm256_shuffle_epi8(filter_coeffs_x, _mm256_set1_epi16(0x0200_u32 as i16));
    // coeffs 2 3 2 3 2 3 2 3
    coeffs_h[1] = _mm256_shuffle_epi8(filter_coeffs_x, _mm256_set1_epi16(0x0604_u32 as i16));
    // coeffs 4 5 4 5 4 5 4 5
    coeffs_h[2] = _mm256_shuffle_epi8(filter_coeffs_x, _mm256_set1_epi16(0x0a08_u32 as i16));
    // coeffs 6 7 6 7 6 7 6 7
    coeffs_h[3] = _mm256_shuffle_epi8(filter_coeffs_x, _mm256_set1_epi16(0x0e0c_u32 as i16));
    let round_const_h: __m256i = _mm256_set1_epi16((1 << ((*conv_params).round_0 - 1)) as i16);
    let round_const_horz: __m256i =
        _mm256_set1_epi16((1 << (bd + 7 - (*conv_params).round_0 - 1)) as i16);
    let clamp_low: __m256i = _mm256_setzero_si256();
    let clamp_high: __m256i =
        _mm256_set1_epi16(((1 << (bd + 1 + 7 - (*conv_params).round_0)) - 1) as i16);
    let round_shift_h: __m128i = _mm_cvtsi32_si128((*conv_params).round_0);
    // Add an offset to account for the "add_src" part of the convolve function.
    let zero_128: __m128i = _mm_setzero_si128();
    let offset_0: __m128i = _mm_insert_epi16::<3>(zero_128, 1 << 7);
    let coeffs_y: __m128i =
        _mm_add_epi16(xx_loadu_128(filter_y as *const core::ffi::c_void), offset_0);
    let filter_coeffs_y: __m256i = _mm256_broadcastsi128_si256(coeffs_y);
    // coeffs 0 1 0 1 0 1 0 1
    coeffs_v[0] = _mm256_shuffle_epi32::<0x00>(filter_coeffs_y);
    // coeffs 2 3 2 3 2 3 2 3
    coeffs_v[1] = _mm256_shuffle_epi32::<0x55>(filter_coeffs_y);
    // coeffs 4 5 4 5 4 5 4 5
    coeffs_v[2] = _mm256_shuffle_epi32::<0xaa>(filter_coeffs_y);
    // coeffs 6 7 6 7 6 7 6 7
    coeffs_v[3] = _mm256_shuffle_epi32::<0xff>(filter_coeffs_y);
    let round_const_v: __m256i = _mm256_set1_epi32(
        (1 << ((*conv_params).round_1 - 1)) - (1 << (bd + (*conv_params).round_1 - 1)),
    );
    let round_shift_v: __m128i = _mm_cvtsi32_si128((*conv_params).round_1);
    {
        j = 0;
        while j < w {
            {
                i = 0;
                while i < im_h {
                    let mut data: __m256i = _mm256_castsi128_si256(_mm_loadu_si128(
                        src_ptr.offset(i as isize * src_stride + j as isize) as *mut __m128i
                            as *const __m128i,
                    ));
                    // Load the next line
                    if i + 1 < im_h {
                        data = _mm256_inserti128_si256::<1>(
                            data,
                            _mm_loadu_si128(
                                src_ptr.offset(i as isize * src_stride + j as isize + src_stride)
                                    as *mut __m128i
                                    as *const __m128i,
                            ),
                        );
                    }
                    let mut res: __m256i = convolve_lowbd_x(
                        data,
                        coeffs_h.as_mut_ptr() as *const __m256i,
                        filt.as_mut_ptr() as *const __m256i,
                    );
                    res = _mm256_sra_epi16(_mm256_add_epi16(res, round_const_h), round_shift_h);
                    let mut data_0: __m256i = _mm256_shuffle_epi8(data, filt_center);
                    // multiply the center pixel by 2^(FILTER_BITS - round_0) and add it to
                    // the result
                    data_0 =
                        _mm256_sll_epi16(data_0, _mm_cvtsi32_si128(7 - (*conv_params).round_0));
                    res = _mm256_add_epi16(res, data_0);
                    res = _mm256_add_epi16(res, round_const_horz);
                    let res_clamped: __m256i =
                        _mm256_min_epi16(_mm256_max_epi16(res, clamp_low), clamp_high);
                    _mm256_store_si256(
                        im_block.0.as_mut_ptr().offset((i * im_stride) as isize) as *mut __m256i,
                        res_clamped,
                    );
                    i += 2;
                }
            }
            // Vertical filter
            {
                let mut src_0: __m256i =
                    _mm256_loadu_si256(im_block.0.as_mut_ptr().offset((0 * im_stride) as isize)
                        as *mut __m256i as *const __m256i);
                let mut src_1: __m256i =
                    _mm256_loadu_si256(im_block.0.as_mut_ptr().offset((1 * im_stride) as isize)
                        as *mut __m256i as *const __m256i);
                let mut src_2: __m256i =
                    _mm256_loadu_si256(im_block.0.as_mut_ptr().offset((2 * im_stride) as isize)
                        as *mut __m256i as *const __m256i);
                let mut src_3: __m256i =
                    _mm256_loadu_si256(im_block.0.as_mut_ptr().offset((3 * im_stride) as isize)
                        as *mut __m256i as *const __m256i);
                let mut src_4: __m256i =
                    _mm256_loadu_si256(im_block.0.as_mut_ptr().offset((4 * im_stride) as isize)
                        as *mut __m256i as *const __m256i);
                let mut src_5: __m256i =
                    _mm256_loadu_si256(im_block.0.as_mut_ptr().offset((5 * im_stride) as isize)
                        as *mut __m256i as *const __m256i);
                let mut s: [__m256i; 8] = core::mem::zeroed();
                s[0] = _mm256_unpacklo_epi16(src_0, src_1);
                s[1] = _mm256_unpacklo_epi16(src_2, src_3);
                s[2] = _mm256_unpacklo_epi16(src_4, src_5);
                s[4] = _mm256_unpackhi_epi16(src_0, src_1);
                s[5] = _mm256_unpackhi_epi16(src_2, src_3);
                s[6] = _mm256_unpackhi_epi16(src_4, src_5);
                {
                    i = 0;
                    while i < h - 1 {
                        let mut data: *const i16 =
                            im_block.0.as_mut_ptr().offset((i * im_stride) as isize) as *const i16;
                        let s6: __m256i = _mm256_loadu_si256(data.offset((6 * im_stride) as isize)
                            as *mut __m256i
                            as *const __m256i);
                        let s7: __m256i = _mm256_loadu_si256(data.offset((7 * im_stride) as isize)
                            as *mut __m256i
                            as *const __m256i);
                        s[3] = _mm256_unpacklo_epi16(s6, s7);
                        s[7] = _mm256_unpackhi_epi16(s6, s7);
                        let mut res_a: __m256i = convolve(
                            s.as_mut_ptr() as *const __m256i,
                            coeffs_v.as_mut_ptr() as *const __m256i,
                        );
                        let mut res_b: __m256i = convolve(
                            s.as_mut_ptr().offset(4) as *const __m256i,
                            coeffs_v.as_mut_ptr() as *const __m256i,
                        );
                        let res_a_round: __m256i =
                            _mm256_sra_epi32(_mm256_add_epi32(res_a, round_const_v), round_shift_v);
                        let res_b_round: __m256i =
                            _mm256_sra_epi32(_mm256_add_epi32(res_b, round_const_v), round_shift_v);
                        // rounding code
                        // 16 bit conversion
                        let res_16bit: __m256i = _mm256_packs_epi32(res_a_round, res_b_round);
                        // 8 bit conversion and saturation to uint8
                        let res_8b: __m256i = _mm256_packus_epi16(res_16bit, res_16bit);
                        let res_0: __m128i = _mm256_castsi256_si128(res_8b);
                        let res_1: __m128i = _mm256_extracti128_si256::<1>(res_8b);
                        // Store values into the destination buffer
                        let mut p_0: *mut __m128i =
                            dst.offset(i as isize * dst_stride + j as isize) as *mut __m128i;
                        let mut p_1: *mut __m128i = dst
                            .offset(i as isize * dst_stride + j as isize + dst_stride)
                            as *mut __m128i;
                        _mm_storel_epi64(p_0, res_0);
                        _mm_storel_epi64(p_1, res_1);
                        s[0] = s[1];
                        s[1] = s[2];
                        s[2] = s[3];
                        s[4] = s[5];
                        s[5] = s[6];
                        s[6] = s[7];
                        i += 2;
                    }
                }
                if h - i != 0 {
                    s[0] = _mm256_permute2x128_si256::<0x20>(s[0], s[4]);
                    s[1] = _mm256_permute2x128_si256::<0x20>(s[1], s[5]);
                    s[2] = _mm256_permute2x128_si256::<0x20>(s[2], s[6]);
                    let mut data: *const i16 =
                        im_block.0.as_mut_ptr().offset((i * im_stride) as isize) as *const i16;
                    let s6_: __m128i = _mm_loadu_si128(data.offset((6 * im_stride) as isize)
                        as *mut __m128i
                        as *const __m128i);
                    let s7_: __m128i = _mm_loadu_si128(data.offset((7 * im_stride) as isize)
                        as *mut __m128i
                        as *const __m128i);
                    let mut s3: __m128i = _mm_unpacklo_epi16(s6_, s7_);
                    let mut s7: __m128i = _mm_unpackhi_epi16(s6_, s7_);
                    s[3] = _mm256_inserti128_si256::<1>(_mm256_castsi128_si256(s3), s7);
                    let mut convolveres: __m256i = convolve(
                        s.as_mut_ptr() as *const __m256i,
                        coeffs_v.as_mut_ptr() as *const __m256i,
                    );
                    let res_round: __m256i = _mm256_sra_epi32(
                        _mm256_add_epi32(convolveres, round_const_v),
                        round_shift_v,
                    );
                    // rounding code
                    // 16 bit conversion
                    let mut reslo: __m128i = _mm256_castsi256_si128(res_round);
                    let mut reshi: __m128i = _mm256_extracti128_si256::<1>(res_round);
                    let res_16bit: __m128i = _mm_packus_epi32(reslo, reshi);
                    // 8 bit conversion and saturation to uint8
                    let res_8b: __m128i = _mm_packus_epi16(res_16bit, res_16bit);
                    let mut p_0: *mut __m128i =
                        dst.offset(i as isize * dst_stride + j as isize) as *mut __m128i;
                    _mm_storel_epi64(p_0, res_8b);
                }
            }
            j += 8;
        }
    }
}
