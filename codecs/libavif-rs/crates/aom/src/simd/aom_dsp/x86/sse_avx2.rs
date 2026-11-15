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

// Translated from upstream/aom/aom_dsp/x86/sse_avx2.c (ADR 0056, ADR 0058).
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

use crate::simd::aom_dsp__x86__synonyms::*;
use crate::simd::aom_dsp__x86__synonyms_avx2::*;
use core::arch::x86_64::*;

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn sse_w32_avx2(sum: *mut __m256i, a: *const u8, b: *const u8) {
    let v_a0: __m256i = yy_loadu_256(a as *const core::ffi::c_void);
    let v_b0: __m256i = yy_loadu_256(b as *const core::ffi::c_void);
    let zero: __m256i = _mm256_setzero_si256();
    let v_a00_w: __m256i = _mm256_unpacklo_epi8(v_a0, zero);
    let v_a01_w: __m256i = _mm256_unpackhi_epi8(v_a0, zero);
    let v_b00_w: __m256i = _mm256_unpacklo_epi8(v_b0, zero);
    let v_b01_w: __m256i = _mm256_unpackhi_epi8(v_b0, zero);
    let v_d00_w: __m256i = _mm256_sub_epi16(v_a00_w, v_b00_w);
    let v_d01_w: __m256i = _mm256_sub_epi16(v_a01_w, v_b01_w);
    *sum = _mm256_add_epi32(*sum, _mm256_madd_epi16(v_d00_w, v_d00_w));
    *sum = _mm256_add_epi32(*sum, _mm256_madd_epi16(v_d01_w, v_d01_w));
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn summary_all_avx2(sum_all: *const __m256i) -> i64 {
    let mut sum: i64 = 0;
    let mut zero: __m256i = _mm256_setzero_si256();
    let sum0_4x64: __m256i = _mm256_unpacklo_epi32(*sum_all, zero);
    let sum1_4x64: __m256i = _mm256_unpackhi_epi32(*sum_all, zero);
    let sum_4x64: __m256i = _mm256_add_epi64(sum0_4x64, sum1_4x64);
    let sum_2x64: __m128i = _mm_add_epi64(
        _mm256_castsi256_si128(sum_4x64),
        _mm256_extracti128_si256::<1>(sum_4x64),
    );
    let sum_1x64: __m128i = _mm_add_epi64(sum_2x64, _mm_srli_si128::<8>(sum_2x64));
    xx_storel_64(&raw mut sum as *mut core::ffi::c_void, sum_1x64);
    return sum;
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn summary_32_avx2(sum32: *const __m256i, sum: *mut __m256i) {
    let sum0_4x64: __m256i = _mm256_cvtepu32_epi64(_mm256_castsi256_si128(*sum32));
    let sum1_4x64: __m256i = _mm256_cvtepu32_epi64(_mm256_extracti128_si256::<1>(*sum32));
    let sum_4x64: __m256i = _mm256_add_epi64(sum0_4x64, sum1_4x64);
    *sum = _mm256_add_epi64(*sum, sum_4x64);
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn summary_4x64_avx2(sum_4x64: __m256i) -> i64 {
    let mut sum: i64 = 0;
    let sum_2x64: __m128i = _mm_add_epi64(
        _mm256_castsi256_si128(sum_4x64),
        _mm256_extracti128_si256::<1>(sum_4x64),
    );
    let sum_1x64: __m128i = _mm_add_epi64(sum_2x64, _mm_srli_si128::<8>(sum_2x64));
    xx_storel_64(&raw mut sum as *mut core::ffi::c_void, sum_1x64);
    return sum;
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn sse_w4x4_avx2(
    a: *const u8,
    a_stride: i32,
    b: *const u8,
    b_stride: i32,
    sum: *mut __m256i,
) {
    let v_a0: __m128i = xx_loadl_32(a as *const core::ffi::c_void);
    let v_a1: __m128i = xx_loadl_32(a.offset(a_stride as isize) as *const core::ffi::c_void);
    let v_a2: __m128i = xx_loadl_32(a.offset((a_stride * 2) as isize) as *const core::ffi::c_void);
    let v_a3: __m128i = xx_loadl_32(a.offset((a_stride * 3) as isize) as *const core::ffi::c_void);
    let v_b0: __m128i = xx_loadl_32(b as *const core::ffi::c_void);
    let v_b1: __m128i = xx_loadl_32(b.offset(b_stride as isize) as *const core::ffi::c_void);
    let v_b2: __m128i = xx_loadl_32(b.offset((b_stride * 2) as isize) as *const core::ffi::c_void);
    let v_b3: __m128i = xx_loadl_32(b.offset((b_stride * 3) as isize) as *const core::ffi::c_void);
    let v_a0123: __m128i = _mm_unpacklo_epi64(
        _mm_unpacklo_epi32(v_a0, v_a1),
        _mm_unpacklo_epi32(v_a2, v_a3),
    );
    let v_b0123: __m128i = _mm_unpacklo_epi64(
        _mm_unpacklo_epi32(v_b0, v_b1),
        _mm_unpacklo_epi32(v_b2, v_b3),
    );
    let v_a_w: __m256i = _mm256_cvtepu8_epi16(v_a0123);
    let v_b_w: __m256i = _mm256_cvtepu8_epi16(v_b0123);
    let v_d_w: __m256i = _mm256_sub_epi16(v_a_w, v_b_w);
    *sum = _mm256_add_epi32(*sum, _mm256_madd_epi16(v_d_w, v_d_w));
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn sse_w8x2_avx2(
    a: *const u8,
    a_stride: i32,
    b: *const u8,
    b_stride: i32,
    sum: *mut __m256i,
) {
    let v_a0: __m128i = xx_loadl_64(a as *const core::ffi::c_void);
    let v_a1: __m128i = xx_loadl_64(a.offset(a_stride as isize) as *const core::ffi::c_void);
    let v_b0: __m128i = xx_loadl_64(b as *const core::ffi::c_void);
    let v_b1: __m128i = xx_loadl_64(b.offset(b_stride as isize) as *const core::ffi::c_void);
    let v_a_w: __m256i = _mm256_cvtepu8_epi16(_mm_unpacklo_epi64(v_a0, v_a1));
    let v_b_w: __m256i = _mm256_cvtepu8_epi16(_mm_unpacklo_epi64(v_b0, v_b1));
    let v_d_w: __m256i = _mm256_sub_epi16(v_a_w, v_b_w);
    *sum = _mm256_add_epi32(*sum, _mm256_madd_epi16(v_d_w, v_d_w));
}

#[no_mangle]
#[target_feature(enable = "avx2")]
pub unsafe extern "C-unwind" fn aom_sse_avx2(
    mut a: *const u8,
    a_stride: i32,
    mut b: *const u8,
    b_stride: i32,
    width: i32,
    height: i32,
) -> i64 {
    let mut y: i32 = 0;
    let mut sse: i64 = 0;
    let mut sum: __m256i = _mm256_setzero_si256();
    let mut zero: __m256i = _mm256_setzero_si256();
    match width {
        4 => {
            loop {
                sse_w4x4_avx2(a, a_stride, b, b_stride, &raw mut sum);
                a = a.offset((a_stride << 2) as isize);
                b = b.offset((b_stride << 2) as isize);
                y += 4;
                if !(y < height) {
                    break;
                }
            }
            sse = summary_all_avx2(&raw mut sum as *const __m256i);
        }
        8 => {
            loop {
                sse_w8x2_avx2(a, a_stride, b, b_stride, &raw mut sum);
                a = a.offset((a_stride << 1) as isize);
                b = b.offset((b_stride << 1) as isize);
                y += 2;
                if !(y < height) {
                    break;
                }
            }
            sse = summary_all_avx2(&raw mut sum as *const __m256i);
        }
        16 => {
            loop {
                let v_a0: __m128i = xx_loadu_128(a as *const core::ffi::c_void);
                let v_a1: __m128i =
                    xx_loadu_128(a.offset(a_stride as isize) as *const core::ffi::c_void);
                let v_b0: __m128i = xx_loadu_128(b as *const core::ffi::c_void);
                let v_b1: __m128i =
                    xx_loadu_128(b.offset(b_stride as isize) as *const core::ffi::c_void);
                let v_a: __m256i =
                    _mm256_insertf128_si256::<0x01>(_mm256_castsi128_si256(v_a0), v_a1);
                let v_b: __m256i =
                    _mm256_insertf128_si256::<0x01>(_mm256_castsi128_si256(v_b0), v_b1);
                let v_al: __m256i = _mm256_unpacklo_epi8(v_a, zero);
                let v_au: __m256i = _mm256_unpackhi_epi8(v_a, zero);
                let v_bl: __m256i = _mm256_unpacklo_epi8(v_b, zero);
                let v_bu: __m256i = _mm256_unpackhi_epi8(v_b, zero);
                let v_asub: __m256i = _mm256_sub_epi16(v_al, v_bl);
                let v_bsub: __m256i = _mm256_sub_epi16(v_au, v_bu);
                let temp: __m256i = _mm256_add_epi32(
                    _mm256_madd_epi16(v_asub, v_asub),
                    _mm256_madd_epi16(v_bsub, v_bsub),
                );
                sum = _mm256_add_epi32(sum, temp);
                a = a.offset((a_stride << 1) as isize);
                b = b.offset((b_stride << 1) as isize);
                y += 2;
                if !(y < height) {
                    break;
                }
            }
            sse = summary_all_avx2(&raw mut sum as *const __m256i);
        }
        32 => {
            loop {
                sse_w32_avx2(&raw mut sum, a, b);
                a = a.offset(a_stride as isize);
                b = b.offset(b_stride as isize);
                y += 1;
                if !(y < height) {
                    break;
                }
            }
            sse = summary_all_avx2(&raw mut sum as *const __m256i);
        }
        64 => {
            loop {
                sse_w32_avx2(&raw mut sum, a, b);
                sse_w32_avx2(&raw mut sum, a.offset(32), b.offset(32));
                a = a.offset(a_stride as isize);
                b = b.offset(b_stride as isize);
                y += 1;
                if !(y < height) {
                    break;
                }
            }
            sse = summary_all_avx2(&raw mut sum as *const __m256i);
        }
        128 => {
            loop {
                sse_w32_avx2(&raw mut sum, a, b);
                sse_w32_avx2(&raw mut sum, a.offset(32), b.offset(32));
                sse_w32_avx2(&raw mut sum, a.offset(64), b.offset(64));
                sse_w32_avx2(&raw mut sum, a.offset(96), b.offset(96));
                a = a.offset(a_stride as isize);
                b = b.offset(b_stride as isize);
                y += 1;
                if !(y < height) {
                    break;
                }
            }
            sse = summary_all_avx2(&raw mut sum as *const __m256i);
        }
        _ => {
            if width & 0x07 == 0 {
                loop {
                    let mut i: i32 = 0;
                    loop {
                        sse_w8x2_avx2(
                            a.offset(i as isize),
                            a_stride,
                            b.offset(i as isize),
                            b_stride,
                            &raw mut sum,
                        );
                        i += 8;
                        if !(i < width) {
                            break;
                        }
                    }
                    a = a.offset((a_stride << 1) as isize);
                    b = b.offset((b_stride << 1) as isize);
                    y += 2;
                    if !(y < height) {
                        break;
                    }
                }
            } else {
                loop {
                    let mut i: i32 = 0;
                    loop {
                        sse_w8x2_avx2(
                            a.offset(i as isize),
                            a_stride,
                            b.offset(i as isize),
                            b_stride,
                            &raw mut sum,
                        );
                        let mut a2: *const u8 =
                            a.offset(i as isize).offset((a_stride << 1) as isize);
                        let mut b2: *const u8 =
                            b.offset(i as isize).offset((b_stride << 1) as isize);
                        sse_w8x2_avx2(a2, a_stride, b2, b_stride, &raw mut sum);
                        i += 8;
                        if !(i + 4 < width) {
                            break;
                        }
                    }
                    sse_w4x4_avx2(
                        a.offset(i as isize),
                        a_stride,
                        b.offset(i as isize),
                        b_stride,
                        &raw mut sum,
                    );
                    a = a.offset((a_stride << 2) as isize);
                    b = b.offset((b_stride << 2) as isize);
                    y += 4;
                    if !(y < height) {
                        break;
                    }
                }
            }
            sse = summary_all_avx2(&raw mut sum as *const __m256i);
        }
    }
    return sse;
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn highbd_sse_w16_avx2(sum: *mut __m256i, a: *const u16, b: *const u16) {
    let v_a_w: __m256i = yy_loadu_256(a as *const core::ffi::c_void);
    let v_b_w: __m256i = yy_loadu_256(b as *const core::ffi::c_void);
    let v_d_w: __m256i = _mm256_sub_epi16(v_a_w, v_b_w);
    *sum = _mm256_add_epi32(*sum, _mm256_madd_epi16(v_d_w, v_d_w));
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn highbd_sse_w4x4_avx2(
    sum: *mut __m256i,
    a: *const u16,
    a_stride: i32,
    b: *const u16,
    b_stride: i32,
) {
    let v_a0: __m128i = xx_loadl_64(a as *const core::ffi::c_void);
    let v_a1: __m128i = xx_loadl_64(a.offset(a_stride as isize) as *const core::ffi::c_void);
    let v_a2: __m128i = xx_loadl_64(a.offset((a_stride * 2) as isize) as *const core::ffi::c_void);
    let v_a3: __m128i = xx_loadl_64(a.offset((a_stride * 3) as isize) as *const core::ffi::c_void);
    let v_b0: __m128i = xx_loadl_64(b as *const core::ffi::c_void);
    let v_b1: __m128i = xx_loadl_64(b.offset(b_stride as isize) as *const core::ffi::c_void);
    let v_b2: __m128i = xx_loadl_64(b.offset((b_stride * 2) as isize) as *const core::ffi::c_void);
    let v_b3: __m128i = xx_loadl_64(b.offset((b_stride * 3) as isize) as *const core::ffi::c_void);
    let v_a_w: __m256i = yy_set_m128i(
        _mm_unpacklo_epi64(v_a0, v_a1),
        _mm_unpacklo_epi64(v_a2, v_a3),
    );
    let v_b_w: __m256i = yy_set_m128i(
        _mm_unpacklo_epi64(v_b0, v_b1),
        _mm_unpacklo_epi64(v_b2, v_b3),
    );
    let v_d_w: __m256i = _mm256_sub_epi16(v_a_w, v_b_w);
    *sum = _mm256_add_epi32(*sum, _mm256_madd_epi16(v_d_w, v_d_w));
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn highbd_sse_w8x2_avx2(
    sum: *mut __m256i,
    a: *const u16,
    a_stride: i32,
    b: *const u16,
    b_stride: i32,
) {
    let v_a_w: __m256i = yy_loadu2_128(
        a.offset(a_stride as isize) as *const core::ffi::c_void,
        a as *const core::ffi::c_void,
    );
    let v_b_w: __m256i = yy_loadu2_128(
        b.offset(b_stride as isize) as *const core::ffi::c_void,
        b as *const core::ffi::c_void,
    );
    let v_d_w: __m256i = _mm256_sub_epi16(v_a_w, v_b_w);
    *sum = _mm256_add_epi32(*sum, _mm256_madd_epi16(v_d_w, v_d_w));
}

#[no_mangle]
#[target_feature(enable = "avx2")]
pub unsafe extern "C-unwind" fn aom_highbd_sse_avx2(
    a8: *const u8,
    a_stride: i32,
    b8: *const u8,
    b_stride: i32,
    width: i32,
    height: i32,
) -> i64 {
    let mut y: i32 = 0;
    let mut sse: i64 = 0;
    let mut a: *mut u16 = ((a8 as usize) << 1) as *mut u16;
    let mut b: *mut u16 = ((b8 as usize) << 1) as *mut u16;
    let mut sum: __m256i = _mm256_setzero_si256();
    match width {
        4 => {
            loop {
                highbd_sse_w4x4_avx2(
                    &raw mut sum,
                    a as *const u16,
                    a_stride,
                    b as *const u16,
                    b_stride,
                );
                a = a.offset((a_stride << 2) as isize);
                b = b.offset((b_stride << 2) as isize);
                y += 4;
                if !(y < height) {
                    break;
                }
            }
            sse = summary_all_avx2(&raw mut sum as *const __m256i);
        }
        8 => {
            loop {
                highbd_sse_w8x2_avx2(
                    &raw mut sum,
                    a as *const u16,
                    a_stride,
                    b as *const u16,
                    b_stride,
                );
                a = a.offset((a_stride << 1) as isize);
                b = b.offset((b_stride << 1) as isize);
                y += 2;
                if !(y < height) {
                    break;
                }
            }
            sse = summary_all_avx2(&raw mut sum as *const __m256i);
        }
        16 => {
            loop {
                highbd_sse_w16_avx2(&raw mut sum, a as *const u16, b as *const u16);
                a = a.offset(a_stride as isize);
                b = b.offset(b_stride as isize);
                y += 1;
                if !(y < height) {
                    break;
                }
            }
            sse = summary_all_avx2(&raw mut sum as *const __m256i);
        }
        32 => {
            loop {
                let mut l: i32 = 0;
                let mut sum32: __m256i = _mm256_setzero_si256();
                loop {
                    highbd_sse_w16_avx2(&raw mut sum32, a as *const u16, b as *const u16);
                    highbd_sse_w16_avx2(
                        &raw mut sum32,
                        a.offset(16) as *const u16,
                        b.offset(16) as *const u16,
                    );
                    a = a.offset(a_stride as isize);
                    b = b.offset(b_stride as isize);
                    l += 1;
                    if !(l < 64 && l < height - y) {
                        break;
                    }
                }
                summary_32_avx2(&raw mut sum32 as *const __m256i, &raw mut sum);
                y += 64;
                if !(y < height) {
                    break;
                }
            }
            sse = summary_4x64_avx2(sum);
        }
        64 => {
            loop {
                let mut l: i32 = 0;
                let mut sum32: __m256i = _mm256_setzero_si256();
                loop {
                    highbd_sse_w16_avx2(&raw mut sum32, a as *const u16, b as *const u16);
                    highbd_sse_w16_avx2(
                        &raw mut sum32,
                        a.offset((16 * 1) as isize) as *const u16,
                        b.offset((16 * 1) as isize) as *const u16,
                    );
                    highbd_sse_w16_avx2(
                        &raw mut sum32,
                        a.offset((16 * 2) as isize) as *const u16,
                        b.offset((16 * 2) as isize) as *const u16,
                    );
                    highbd_sse_w16_avx2(
                        &raw mut sum32,
                        a.offset((16 * 3) as isize) as *const u16,
                        b.offset((16 * 3) as isize) as *const u16,
                    );
                    a = a.offset(a_stride as isize);
                    b = b.offset(b_stride as isize);
                    l += 1;
                    if !(l < 32 && l < height - y) {
                        break;
                    }
                }
                summary_32_avx2(&raw mut sum32 as *const __m256i, &raw mut sum);
                y += 32;
                if !(y < height) {
                    break;
                }
            }
            sse = summary_4x64_avx2(sum);
        }
        128 => {
            loop {
                let mut l: i32 = 0;
                let mut sum32: __m256i = _mm256_setzero_si256();
                loop {
                    highbd_sse_w16_avx2(&raw mut sum32, a as *const u16, b as *const u16);
                    highbd_sse_w16_avx2(
                        &raw mut sum32,
                        a.offset((16 * 1) as isize) as *const u16,
                        b.offset((16 * 1) as isize) as *const u16,
                    );
                    highbd_sse_w16_avx2(
                        &raw mut sum32,
                        a.offset((16 * 2) as isize) as *const u16,
                        b.offset((16 * 2) as isize) as *const u16,
                    );
                    highbd_sse_w16_avx2(
                        &raw mut sum32,
                        a.offset((16 * 3) as isize) as *const u16,
                        b.offset((16 * 3) as isize) as *const u16,
                    );
                    highbd_sse_w16_avx2(
                        &raw mut sum32,
                        a.offset((16 * 4) as isize) as *const u16,
                        b.offset((16 * 4) as isize) as *const u16,
                    );
                    highbd_sse_w16_avx2(
                        &raw mut sum32,
                        a.offset((16 * 5) as isize) as *const u16,
                        b.offset((16 * 5) as isize) as *const u16,
                    );
                    highbd_sse_w16_avx2(
                        &raw mut sum32,
                        a.offset((16 * 6) as isize) as *const u16,
                        b.offset((16 * 6) as isize) as *const u16,
                    );
                    highbd_sse_w16_avx2(
                        &raw mut sum32,
                        a.offset((16 * 7) as isize) as *const u16,
                        b.offset((16 * 7) as isize) as *const u16,
                    );
                    a = a.offset(a_stride as isize);
                    b = b.offset(b_stride as isize);
                    l += 1;
                    if !(l < 16 && l < height - y) {
                        break;
                    }
                }
                summary_32_avx2(&raw mut sum32 as *const __m256i, &raw mut sum);
                y += 16;
                if !(y < height) {
                    break;
                }
            }
            sse = summary_4x64_avx2(sum);
        }
        _ => {
            if width & 0x7 != 0 {
                loop {
                    let mut i: i32 = 0;
                    let mut sum32: __m256i = _mm256_setzero_si256();
                    loop {
                        highbd_sse_w8x2_avx2(
                            &raw mut sum32,
                            a.offset(i as isize) as *const u16,
                            a_stride,
                            b.offset(i as isize) as *const u16,
                            b_stride,
                        );
                        let mut a2: *const u16 =
                            a.offset(i as isize).offset((a_stride << 1) as isize) as *const u16;
                        let mut b2: *const u16 =
                            b.offset(i as isize).offset((b_stride << 1) as isize) as *const u16;
                        highbd_sse_w8x2_avx2(&raw mut sum32, a2, a_stride, b2, b_stride);
                        i += 8;
                        if !(i + 4 < width) {
                            break;
                        }
                    }
                    highbd_sse_w4x4_avx2(
                        &raw mut sum32,
                        a.offset(i as isize) as *const u16,
                        a_stride,
                        b.offset(i as isize) as *const u16,
                        b_stride,
                    );
                    summary_32_avx2(&raw mut sum32 as *const __m256i, &raw mut sum);
                    a = a.offset((a_stride << 2) as isize);
                    b = b.offset((b_stride << 2) as isize);
                    y += 4;
                    if !(y < height) {
                        break;
                    }
                }
            } else {
                loop {
                    let mut l: i32 = 0;
                    let mut sum32: __m256i = _mm256_setzero_si256();
                    loop {
                        let mut i: i32 = 0;
                        loop {
                            highbd_sse_w8x2_avx2(
                                &raw mut sum32,
                                a.offset(i as isize) as *const u16,
                                a_stride,
                                b.offset(i as isize) as *const u16,
                                b_stride,
                            );
                            i += 8;
                            if !(i < width) {
                                break;
                            }
                        }
                        a = a.offset((a_stride << 1) as isize);
                        b = b.offset((b_stride << 1) as isize);
                        l += 2;
                        if !(l < 8 && l < height - y) {
                            break;
                        }
                    }
                    summary_32_avx2(&raw mut sum32 as *const __m256i, &raw mut sum);
                    y += 8;
                    if !(y < height) {
                        break;
                    }
                }
            }
            sse = summary_4x64_avx2(sum);
        }
    }
    return sse;
}

// CONFIG_AV1_HIGHBITDEPTH
