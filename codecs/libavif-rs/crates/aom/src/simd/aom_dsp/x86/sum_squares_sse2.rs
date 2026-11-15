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

// Translated from upstream/aom/aom_dsp/x86/sum_squares_sse2.c (ADR 0056, ADR 0058).
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

use crate::aom_dsp__sum_squares::{
    aom_sum_squares_2d_i16_c, aom_sum_squares_i16_c, aom_sum_sse_2d_i16_c,
};
use crate::simd::aom_dsp__x86__synonyms::*;
use core::arch::x86_64::*;

#[inline]
#[target_feature(enable = "sse2")]
unsafe fn xx_loadh_64(a: __m128i, b: *const core::ffi::c_void) -> __m128i {
    let ad: __m128d = _mm_castsi128_pd(a);
    return _mm_castpd_si128(_mm_loadh_pd(ad, b as *mut f64 as *const f64));
}

#[inline]
#[target_feature(enable = "sse2")]
unsafe fn xx_cvtsi128_si64(a: __m128i) -> u64 {
    return _mm_cvtsi128_si64(a) as u64;
}

#[inline]
#[target_feature(enable = "sse2")]
unsafe fn sum_squares_i16_4x4_sse2(src: *const i16, stride: i32) -> __m128i {
    let v_val_0_w: __m128i =
        xx_loadl_64(src.offset((0 * stride) as isize) as *const core::ffi::c_void);
    let v_val_2_w: __m128i =
        xx_loadl_64(src.offset((2 * stride) as isize) as *const core::ffi::c_void);
    let v_val_01_w: __m128i = xx_loadh_64(
        v_val_0_w,
        src.offset((1 * stride) as isize) as *const core::ffi::c_void,
    );
    let v_val_23_w: __m128i = xx_loadh_64(
        v_val_2_w,
        src.offset((3 * stride) as isize) as *const core::ffi::c_void,
    );
    let v_sq_01_d: __m128i = _mm_madd_epi16(v_val_01_w, v_val_01_w);
    let v_sq_23_d: __m128i = _mm_madd_epi16(v_val_23_w, v_val_23_w);
    return _mm_add_epi32(v_sq_01_d, v_sq_23_d);
}

#[no_mangle]
#[target_feature(enable = "sse2")]
pub unsafe extern "C-unwind" fn aom_sum_squares_2d_i16_4x4_sse2(
    src: *const i16,
    stride: i32,
) -> u64 {
    let v_sum_0123_d: __m128i = sum_squares_i16_4x4_sse2(src, stride);
    let mut v_sum_d: __m128i = _mm_add_epi32(v_sum_0123_d, _mm_srli_epi64::<{ 32 }>(v_sum_0123_d));
    v_sum_d = _mm_add_epi32(v_sum_d, _mm_srli_si128::<8>(v_sum_d));
    return _mm_cvtsi128_si32(v_sum_d) as u64;
}

#[no_mangle]
#[target_feature(enable = "sse2")]
pub unsafe extern "C-unwind" fn aom_sum_sse_2d_i16_4x4_sse2(
    src: *const i16,
    stride: i32,
    sum: *mut i32,
) -> u64 {
    let one_reg: __m128i = _mm_set1_epi16(1);
    let v_val_0_w: __m128i =
        xx_loadl_64(src.offset((0 * stride) as isize) as *const core::ffi::c_void);
    let v_val_2_w: __m128i =
        xx_loadl_64(src.offset((2 * stride) as isize) as *const core::ffi::c_void);
    let mut v_val_01_w: __m128i = xx_loadh_64(
        v_val_0_w,
        src.offset((1 * stride) as isize) as *const core::ffi::c_void,
    );
    let mut v_val_23_w: __m128i = xx_loadh_64(
        v_val_2_w,
        src.offset((3 * stride) as isize) as *const core::ffi::c_void,
    );
    let mut v_sum_0123_d: __m128i = _mm_add_epi16(v_val_01_w, v_val_23_w);
    v_sum_0123_d = _mm_madd_epi16(v_sum_0123_d, one_reg);
    v_sum_0123_d = _mm_add_epi32(v_sum_0123_d, _mm_srli_si128::<8>(v_sum_0123_d));
    v_sum_0123_d = _mm_add_epi32(v_sum_0123_d, _mm_srli_si128::<4>(v_sum_0123_d));
    *sum = _mm_cvtsi128_si32(v_sum_0123_d);
    let v_sq_01_d: __m128i = _mm_madd_epi16(v_val_01_w, v_val_01_w);
    let v_sq_23_d: __m128i = _mm_madd_epi16(v_val_23_w, v_val_23_w);
    let mut v_sq_0123_d: __m128i = _mm_add_epi32(v_sq_01_d, v_sq_23_d);
    v_sq_0123_d = _mm_add_epi32(v_sq_0123_d, _mm_srli_si128::<8>(v_sq_0123_d));
    v_sq_0123_d = _mm_add_epi32(v_sq_0123_d, _mm_srli_si128::<4>(v_sq_0123_d));
    return _mm_cvtsi128_si32(v_sq_0123_d) as u64;
}

#[no_mangle]
#[target_feature(enable = "sse2")]
pub unsafe extern "C-unwind" fn aom_sum_squares_2d_i16_4xn_sse2(
    mut src: *const i16,
    stride: i32,
    height: i32,
) -> u64 {
    let mut r: i32 = 0;
    let mut v_acc_q: __m128i = _mm_setzero_si128();
    loop {
        let v_acc_d: __m128i = sum_squares_i16_4x4_sse2(src, stride);
        v_acc_q = _mm_add_epi32(v_acc_q, v_acc_d);
        src = src.offset((stride << 2) as isize);
        r += 4;
        if !(r < height) {
            break;
        }
    }
    let v_zext_mask_q: __m128i = _mm_set1_epi64x(!0_u32 as i64);
    let mut v_acc_64: __m128i = _mm_add_epi64(
        _mm_srli_epi64::<{ 32 }>(v_acc_q),
        _mm_and_si128(v_acc_q, v_zext_mask_q),
    );
    v_acc_64 = _mm_add_epi64(v_acc_64, _mm_srli_si128::<8>(v_acc_64));
    return xx_cvtsi128_si64(v_acc_64);
}

#[no_mangle]
#[target_feature(enable = "sse2")]
pub unsafe extern "C-unwind" fn aom_sum_sse_2d_i16_4xn_sse2(
    mut src: *const i16,
    stride: i32,
    height: i32,
    sum: *mut i32,
) -> u64 {
    let mut r: i32 = 0;
    let mut sse: u64 = 0;
    loop {
        let mut curr_sum: i32 = 0;
        sse += aom_sum_sse_2d_i16_4x4_sse2(src, stride, &raw mut curr_sum);
        *sum += curr_sum;
        src = src.offset((stride << 2) as isize);
        r += 4;
        if !(r < height) {
            break;
        }
    }
    return sse;
}

// This prevents GCC/Clang from inlining this function into
// aom_sum_squares_2d_i16_sse2, which in turn saves some stack
// maintenance instructions in the common case of 4x4.
#[target_feature(enable = "sse2")]
pub unsafe fn aom_sum_squares_2d_i16_nxn_sse2(
    mut src: *const i16,
    stride: i32,
    width: i32,
    height: i32,
) -> u64 {
    let mut r: i32 = 0;
    let v_zext_mask_q: __m128i = _mm_set1_epi64x(!0_u32 as i64);
    let mut v_acc_q: __m128i = _mm_setzero_si128();
    loop {
        let mut v_acc_d: __m128i = _mm_setzero_si128();
        let mut c: i32 = 0;
        loop {
            let mut b: *const i16 = src.offset(c as isize);
            let v_val_0_w: __m128i =
                xx_load_128(b.offset((0 * stride) as isize) as *const core::ffi::c_void);
            let v_val_1_w: __m128i =
                xx_load_128(b.offset((1 * stride) as isize) as *const core::ffi::c_void);
            let v_val_2_w: __m128i =
                xx_load_128(b.offset((2 * stride) as isize) as *const core::ffi::c_void);
            let v_val_3_w: __m128i =
                xx_load_128(b.offset((3 * stride) as isize) as *const core::ffi::c_void);
            let v_sq_0_d: __m128i = _mm_madd_epi16(v_val_0_w, v_val_0_w);
            let v_sq_1_d: __m128i = _mm_madd_epi16(v_val_1_w, v_val_1_w);
            let v_sq_2_d: __m128i = _mm_madd_epi16(v_val_2_w, v_val_2_w);
            let v_sq_3_d: __m128i = _mm_madd_epi16(v_val_3_w, v_val_3_w);
            let v_sum_01_d: __m128i = _mm_add_epi32(v_sq_0_d, v_sq_1_d);
            let v_sum_23_d: __m128i = _mm_add_epi32(v_sq_2_d, v_sq_3_d);
            let v_sum_0123_d: __m128i = _mm_add_epi32(v_sum_01_d, v_sum_23_d);
            v_acc_d = _mm_add_epi32(v_acc_d, v_sum_0123_d);
            c += 8;
            if !(c < width) {
                break;
            }
        }
        v_acc_q = _mm_add_epi64(v_acc_q, _mm_and_si128(v_acc_d, v_zext_mask_q));
        v_acc_q = _mm_add_epi64(v_acc_q, _mm_srli_epi64::<{ 32 }>(v_acc_d));
        src = src.offset((4 * stride) as isize);
        r += 4;
        if !(r < height) {
            break;
        }
    }
    v_acc_q = _mm_add_epi64(v_acc_q, _mm_srli_si128::<8>(v_acc_q));
    return xx_cvtsi128_si64(v_acc_q);
}

// This prevents GCC/Clang from inlining this function into
// aom_sum_sse_2d_i16_nxn_sse2, which in turn saves some stack
// maintenance instructions in the common case of 4x4.
#[target_feature(enable = "sse2")]
pub unsafe fn aom_sum_sse_2d_i16_nxn_sse2(
    mut src: *const i16,
    stride: i32,
    width: i32,
    height: i32,
    sum: *mut i32,
) -> u64 {
    let mut r: i32 = 0;
    let mut result: u64 = 0;
    let zero_reg: __m128i = _mm_setzero_si128();
    let one_reg: __m128i = _mm_set1_epi16(1);
    let mut v_sse_total: __m128i = zero_reg;
    let mut v_sum_total: __m128i = zero_reg;
    loop {
        let mut c: i32 = 0;
        let mut v_sse_row: __m128i = zero_reg;
        loop {
            let mut b: *const i16 = src.offset(c as isize);
            let mut v_val_0_w: __m128i =
                xx_load_128(b.offset((0 * stride) as isize) as *const core::ffi::c_void);
            let mut v_val_1_w: __m128i =
                xx_load_128(b.offset((1 * stride) as isize) as *const core::ffi::c_void);
            let mut v_val_2_w: __m128i =
                xx_load_128(b.offset((2 * stride) as isize) as *const core::ffi::c_void);
            let mut v_val_3_w: __m128i =
                xx_load_128(b.offset((3 * stride) as isize) as *const core::ffi::c_void);
            let v_sq_0_d: __m128i = _mm_madd_epi16(v_val_0_w, v_val_0_w);
            let v_sq_1_d: __m128i = _mm_madd_epi16(v_val_1_w, v_val_1_w);
            let v_sq_2_d: __m128i = _mm_madd_epi16(v_val_2_w, v_val_2_w);
            let v_sq_3_d: __m128i = _mm_madd_epi16(v_val_3_w, v_val_3_w);
            let v_sq_01_d: __m128i = _mm_add_epi32(v_sq_0_d, v_sq_1_d);
            let v_sq_23_d: __m128i = _mm_add_epi32(v_sq_2_d, v_sq_3_d);
            let v_sq_0123_d: __m128i = _mm_add_epi32(v_sq_01_d, v_sq_23_d);
            v_sse_row = _mm_add_epi32(v_sse_row, v_sq_0123_d);
            let v_sum_01: __m128i = _mm_add_epi16(v_val_0_w, v_val_1_w);
            let v_sum_23: __m128i = _mm_add_epi16(v_val_2_w, v_val_3_w);
            let mut v_sum_0123_d: __m128i = _mm_add_epi16(v_sum_01, v_sum_23);
            v_sum_0123_d = _mm_madd_epi16(v_sum_0123_d, one_reg);
            v_sum_total = _mm_add_epi32(v_sum_total, v_sum_0123_d);
            c += 8;
            if !(c < width) {
                break;
            }
        }
        let v_sse_row_low: __m128i = _mm_unpacklo_epi32(v_sse_row, zero_reg);
        let v_sse_row_hi: __m128i = _mm_unpackhi_epi32(v_sse_row, zero_reg);
        v_sse_row = _mm_add_epi64(v_sse_row_low, v_sse_row_hi);
        v_sse_total = _mm_add_epi64(v_sse_total, v_sse_row);
        src = src.offset((4 * stride) as isize);
        r += 4;
        if !(r < height) {
            break;
        }
    }
    v_sum_total = _mm_add_epi32(v_sum_total, _mm_srli_si128::<8>(v_sum_total));
    v_sum_total = _mm_add_epi32(v_sum_total, _mm_srli_si128::<4>(v_sum_total));
    *sum += _mm_cvtsi128_si32(v_sum_total);
    v_sse_total = _mm_add_epi64(v_sse_total, _mm_srli_si128::<8>(v_sse_total));
    xx_storel_64(&raw mut result as *mut core::ffi::c_void, v_sse_total);
    return result;
}

#[no_mangle]
#[target_feature(enable = "sse2")]
pub unsafe extern "C-unwind" fn aom_sum_squares_2d_i16_sse2(
    src: *const i16,
    stride: i32,
    width: i32,
    height: i32,
) -> u64 {
    // 4 elements per row only requires half an XMM register, so this
    // must be a special case, but also note that over 75% of all calls
    // are with size == 4, so it is also the common case.
    if width == 4 && height == 4 {
        return aom_sum_squares_2d_i16_4x4_sse2(src, stride);
    } else if width == 4 && height & 3 == 0 {
        return aom_sum_squares_2d_i16_4xn_sse2(src, stride, height);
    } else if width & 7 == 0 && height & 3 == 0 {
        // Generic case
        return aom_sum_squares_2d_i16_nxn_sse2(src, stride, width, height);
    } else {
        return aom_sum_squares_2d_i16_c(src, stride, width, height);
    }
}

#[no_mangle]
#[target_feature(enable = "sse2")]
pub unsafe extern "C-unwind" fn aom_sum_sse_2d_i16_sse2(
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
    } else if width & 7 == 0 && height & 3 == 0 {
        // Generic case
        return aom_sum_sse_2d_i16_nxn_sse2(src, src_stride, width, height, sum);
    } else {
        return aom_sum_sse_2d_i16_c(src, src_stride, width, height, sum);
    }
}

//////////////////////////////////////////////////////////////////////////////
// 1D version
//////////////////////////////////////////////////////////////////////////////
#[inline]
#[target_feature(enable = "sse2")]
unsafe fn aom_sum_squares_i16_64n_sse2(mut src: *const i16, n: u32) -> u64 {
    let v_zext_mask_q: __m128i = _mm_set1_epi64x(!0_u32 as i64);
    let mut v_acc0_q: __m128i = _mm_setzero_si128();
    let mut v_acc1_q: __m128i = _mm_setzero_si128();
    let mut end: *const i16 = src.offset(n as isize);
    debug_assert!(n % 64 == 0);
    while src < end {
        let v_val_0_w: __m128i = xx_load_128(src as *const core::ffi::c_void);
        let v_val_1_w: __m128i = xx_load_128(src.offset(8) as *const core::ffi::c_void);
        let v_val_2_w: __m128i = xx_load_128(src.offset(16) as *const core::ffi::c_void);
        let v_val_3_w: __m128i = xx_load_128(src.offset(24) as *const core::ffi::c_void);
        let v_val_4_w: __m128i = xx_load_128(src.offset(32) as *const core::ffi::c_void);
        let v_val_5_w: __m128i = xx_load_128(src.offset(40) as *const core::ffi::c_void);
        let v_val_6_w: __m128i = xx_load_128(src.offset(48) as *const core::ffi::c_void);
        let v_val_7_w: __m128i = xx_load_128(src.offset(56) as *const core::ffi::c_void);
        let v_sq_0_d: __m128i = _mm_madd_epi16(v_val_0_w, v_val_0_w);
        let v_sq_1_d: __m128i = _mm_madd_epi16(v_val_1_w, v_val_1_w);
        let v_sq_2_d: __m128i = _mm_madd_epi16(v_val_2_w, v_val_2_w);
        let v_sq_3_d: __m128i = _mm_madd_epi16(v_val_3_w, v_val_3_w);
        let v_sq_4_d: __m128i = _mm_madd_epi16(v_val_4_w, v_val_4_w);
        let v_sq_5_d: __m128i = _mm_madd_epi16(v_val_5_w, v_val_5_w);
        let v_sq_6_d: __m128i = _mm_madd_epi16(v_val_6_w, v_val_6_w);
        let v_sq_7_d: __m128i = _mm_madd_epi16(v_val_7_w, v_val_7_w);
        let v_sum_01_d: __m128i = _mm_add_epi32(v_sq_0_d, v_sq_1_d);
        let v_sum_23_d: __m128i = _mm_add_epi32(v_sq_2_d, v_sq_3_d);
        let v_sum_45_d: __m128i = _mm_add_epi32(v_sq_4_d, v_sq_5_d);
        let v_sum_67_d: __m128i = _mm_add_epi32(v_sq_6_d, v_sq_7_d);
        let v_sum_0123_d: __m128i = _mm_add_epi32(v_sum_01_d, v_sum_23_d);
        let v_sum_4567_d: __m128i = _mm_add_epi32(v_sum_45_d, v_sum_67_d);
        let v_sum_d: __m128i = _mm_add_epi32(v_sum_0123_d, v_sum_4567_d);
        v_acc0_q = _mm_add_epi64(v_acc0_q, _mm_and_si128(v_sum_d, v_zext_mask_q));
        v_acc1_q = _mm_add_epi64(v_acc1_q, _mm_srli_epi64::<{ 32 }>(v_sum_d));
        src = src.offset(64);
    }
    v_acc0_q = _mm_add_epi64(v_acc0_q, v_acc1_q);
    v_acc0_q = _mm_add_epi64(v_acc0_q, _mm_srli_si128::<8>(v_acc0_q));
    return xx_cvtsi128_si64(v_acc0_q);
}

#[no_mangle]
#[target_feature(enable = "sse2")]
pub unsafe extern "C-unwind" fn aom_sum_squares_i16_sse2(src: *const i16, n: u32) -> u64 {
    if n % 64 == 0 {
        return aom_sum_squares_i16_64n_sse2(src, n);
    } else if n > 64 {
        let k: u32 = n & !63_u32;
        return aom_sum_squares_i16_64n_sse2(src, k)
            + aom_sum_squares_i16_c(src.offset(k as isize), n - k);
    } else {
        return aom_sum_squares_i16_c(src, n);
    }
}

// Accumulate sum of 16-bit elements in the vector
#[inline]
#[target_feature(enable = "sse2")]
unsafe fn mm_accumulate_epi16(mut vec_a: __m128i) -> i32 {
    let mut vtmp: __m128i = _mm_srli_si128::<8>(vec_a);
    vec_a = _mm_add_epi16(vec_a, vtmp);
    vtmp = _mm_srli_si128::<4>(vec_a);
    vec_a = _mm_add_epi16(vec_a, vtmp);
    vtmp = _mm_srli_si128::<2>(vec_a);
    vec_a = _mm_add_epi16(vec_a, vtmp);
    return _mm_extract_epi16::<0>(vec_a);
}

// Accumulate sum of 32-bit elements in the vector
#[inline]
#[target_feature(enable = "sse2")]
unsafe fn mm_accumulate_epi32(mut vec_a: __m128i) -> i32 {
    let mut vtmp: __m128i = _mm_srli_si128::<8>(vec_a);
    vec_a = _mm_add_epi32(vec_a, vtmp);
    vtmp = _mm_srli_si128::<4>(vec_a);
    vec_a = _mm_add_epi32(vec_a, vtmp);
    return _mm_cvtsi128_si32(vec_a);
}

#[no_mangle]
#[target_feature(enable = "sse2")]
pub unsafe extern "C-unwind" fn aom_var_2d_u8_sse2(
    src: *mut u8,
    src_stride: i32,
    width: i32,
    height: i32,
) -> u64 {
    let mut srcp: *mut u8;
    let mut s: u64 = 0;
    let mut ss: u64 = 0;
    let mut vzero: __m128i = _mm_setzero_si128();
    let mut v_acc_sum: __m128i = vzero;
    let mut v_acc_sqs: __m128i = vzero;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    // Process 16 elements in a row
    {
        i = 0;
        while i < width - 15 {
            srcp = src.offset(i as isize);
            // Process 8 columns at a time
            {
                j = 0;
                while j < height - 7 {
                    let mut vsrc: [__m128i; 8] = core::mem::zeroed();
                    for k in 0..8 {
                        vsrc[k as usize] = _mm_loadu_si128(srcp as *mut __m128i as *const __m128i);
                        srcp = srcp.offset(src_stride as isize);
                    }
                    for k in 0..8 {
                        let mut vsrc0: __m128i = _mm_unpacklo_epi8(vsrc[k as usize], vzero);
                        let mut vsrc1: __m128i = _mm_unpackhi_epi8(vsrc[k as usize], vzero);
                        v_acc_sum = _mm_add_epi16(v_acc_sum, vsrc0);
                        v_acc_sum = _mm_add_epi16(v_acc_sum, vsrc1);
                        let mut vsqs0: __m128i = _mm_madd_epi16(vsrc0, vsrc0);
                        let mut vsqs1: __m128i = _mm_madd_epi16(vsrc1, vsrc1);
                        v_acc_sqs = _mm_add_epi32(v_acc_sqs, vsqs0);
                        v_acc_sqs = _mm_add_epi32(v_acc_sqs, vsqs1);
                    }
                    // Update total sum and clear the vectors
                    s += mm_accumulate_epi16(v_acc_sum) as u64;
                    ss += mm_accumulate_epi32(v_acc_sqs) as u64;
                    v_acc_sum = vzero;
                    v_acc_sqs = vzero;
                    j += 8;
                }
            }
            // Process remaining rows (height not a multiple of 8)
            {
                while j < height {
                    let mut vsrc: __m128i = _mm_loadu_si128(srcp as *mut __m128i as *const __m128i);
                    let mut vsrc0: __m128i = _mm_unpacklo_epi8(vsrc, vzero);
                    let mut vsrc1: __m128i = _mm_unpackhi_epi8(vsrc, vzero);
                    v_acc_sum = _mm_add_epi16(v_acc_sum, vsrc0);
                    v_acc_sum = _mm_add_epi16(v_acc_sum, vsrc1);
                    let mut vsqs0: __m128i = _mm_madd_epi16(vsrc0, vsrc0);
                    let mut vsqs1: __m128i = _mm_madd_epi16(vsrc1, vsrc1);
                    v_acc_sqs = _mm_add_epi32(v_acc_sqs, vsqs0);
                    v_acc_sqs = _mm_add_epi32(v_acc_sqs, vsqs1);
                    srcp = srcp.offset(src_stride as isize);
                    j += 1;
                }
            }
            // Update total sum and clear the vectors
            s += mm_accumulate_epi16(v_acc_sum) as u64;
            ss += mm_accumulate_epi32(v_acc_sqs) as u64;
            v_acc_sum = vzero;
            v_acc_sqs = vzero;
            i += 16;
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
#[target_feature(enable = "sse2")]
pub unsafe extern "C-unwind" fn aom_var_2d_u16_sse2(
    src: *mut u8,
    src_stride: i32,
    width: i32,
    height: i32,
) -> u64 {
    let mut srcp1: *mut u16 = ((src as usize) << 1) as *mut u16;
    let mut srcp: *mut u16;
    let mut s: u64 = 0;
    let mut ss: u64 = 0;
    let mut vzero: __m128i = _mm_setzero_si128();
    let mut v_acc_sum: __m128i = vzero;
    let mut v_acc_sqs: __m128i = vzero;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    // Process 8 elements in a row
    {
        i = 0;
        while i < width - 8 {
            srcp = srcp1.offset(i as isize);
            // Process 8 columns at a time
            {
                j = 0;
                while j < height - 8 {
                    let mut vsrc: [__m128i; 8] = core::mem::zeroed();
                    for k in 0..8 {
                        vsrc[k as usize] = _mm_loadu_si128(srcp as *mut __m128i as *const __m128i);
                        srcp = srcp.offset(src_stride as isize);
                    }
                    for k in 0..8 {
                        let mut vsrc0: __m128i = _mm_unpacklo_epi16(vsrc[k as usize], vzero);
                        let mut vsrc1: __m128i = _mm_unpackhi_epi16(vsrc[k as usize], vzero);
                        v_acc_sum = _mm_add_epi32(vsrc0, v_acc_sum);
                        v_acc_sum = _mm_add_epi32(vsrc1, v_acc_sum);
                        let mut vsqs0: __m128i = _mm_madd_epi16(vsrc[k as usize], vsrc[k as usize]);
                        v_acc_sqs = _mm_add_epi32(v_acc_sqs, vsqs0);
                    }
                    // Update total sum and clear the vectors
                    s += mm_accumulate_epi32(v_acc_sum) as u64;
                    ss += mm_accumulate_epi32(v_acc_sqs) as u64;
                    v_acc_sum = vzero;
                    v_acc_sqs = vzero;
                    j += 8;
                }
            }
            // Process remaining rows (height not a multiple of 8)
            {
                while j < height {
                    let mut vsrc: __m128i = _mm_loadu_si128(srcp as *mut __m128i as *const __m128i);
                    let mut vsrc0: __m128i = _mm_unpacklo_epi16(vsrc, vzero);
                    let mut vsrc1: __m128i = _mm_unpackhi_epi16(vsrc, vzero);
                    v_acc_sum = _mm_add_epi32(vsrc0, v_acc_sum);
                    v_acc_sum = _mm_add_epi32(vsrc1, v_acc_sum);
                    let mut vsqs0: __m128i = _mm_madd_epi16(vsrc, vsrc);
                    v_acc_sqs = _mm_add_epi32(v_acc_sqs, vsqs0);
                    srcp = srcp.offset(src_stride as isize);
                    j += 1;
                }
            }
            // Update total sum and clear the vectors
            s += mm_accumulate_epi32(v_acc_sum) as u64;
            ss += mm_accumulate_epi32(v_acc_sqs) as u64;
            v_acc_sum = vzero;
            v_acc_sqs = vzero;
            i += 8;
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
