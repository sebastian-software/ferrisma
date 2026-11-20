/*
 * Copyright (c) 2020, Alliance for Open Media. All rights reserved.
 *
 * This source code is subject to the terms of the BSD 2 Clause License and
 * the Alliance for Open Media Patent License 1.0. If the BSD 2 Clause License
 * was not distributed with this source code in the LICENSE file, you can
 * obtain it at www.aomedia.org/license/software. If the Alliance for Open
 * Media Patent License 1.0 was not distributed with this source code in the
 * PATENTS file, you can obtain it at www.aomedia.org/license/patent.
 */

// Translated from upstream/aom/aom_dsp/x86/highbd_variance_avx2.c (ADR 0056, ADR 0058).
// Upstream compiles it with -mavx2. The functions were
// transliterated with a typing-aware C-subset printer (not C2Rust) and
// reviewed against the C function by function: the same sequence of intrinsic
// and helper calls, constants and operators in order (ADR 0058).
// Only the functions of this file that the still-image encoder dispatches (the block
// variances and MSEs, aom_get_var_sse_sum_*, aom_mse_wxh_16bit*, aom_mse_16xh_16bit*) and
// the static helpers they call are translated; the macros that define the block
// variances are expanded by the preprocessor. The sub-pixel and averaging kernels are
// not translated (working decision O2, 2026-11-13, delegated, revisable by the owner).
// ROUND_POWER_OF_TWO (aom_ports/mem.h) is written out through the config's `replace`.
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

// typedef void (*high_variance_fn_t)(const uint16_t *src, int src_stride,
//                                    const uint16_t *ref, int ref_stride,
//                                    uint32_t *sse, int *sum);
type high_variance_fn_t = Option<unsafe fn(*const u16, i32, *const u16, i32, *mut u32, *mut i32)>;

// xoffset==0 && yoffset==0
// xoffset==0 && yoffset==4
// xoffset==0 && yoffset==1,2,3,5,6,7
// xoffset==4 && yoffset==0
// xoffset==4 && yoffset==4
// xoffset==4 && yoffset==1,2,3,5,6,7
// xoffset==1,2,3,5,6,7 && yoffset==0
// xoffset==1,2,3,5,6,7 && yoffset==4
// xoffset==1,2,3,5,6,7 && yoffset==1,2,3,5,6,7
#[inline]
#[target_feature(enable = "avx2")]
unsafe fn highbd_calc8x8var_avx2(
    mut src: *const u16,
    src_stride: i32,
    mut ref_: *const u16,
    ref_stride: i32,
    sse: *mut u32,
    sum: *mut i32,
) {
    let mut v_sum_d: __m256i = _mm256_setzero_si256();
    let mut v_sse_d: __m256i = _mm256_setzero_si256();
    for i in (0..8).step_by(2) {
        let v_p_a0: __m128i = _mm_loadu_si128(src as *const __m128i);
        let v_p_a1: __m128i = _mm_loadu_si128(src.offset(src_stride as isize) as *const __m128i);
        let v_p_b0: __m128i = _mm_loadu_si128(ref_ as *const __m128i);
        let v_p_b1: __m128i = _mm_loadu_si128(ref_.offset(ref_stride as isize) as *const __m128i);
        let mut v_p_a: __m256i = _mm256_castsi128_si256(v_p_a0);
        let mut v_p_b: __m256i = _mm256_castsi128_si256(v_p_b0);
        v_p_a = _mm256_inserti128_si256::<1>(v_p_a, v_p_a1);
        v_p_b = _mm256_inserti128_si256::<1>(v_p_b, v_p_b1);
        let v_diff: __m256i = _mm256_sub_epi16(v_p_a, v_p_b);
        let v_sqrdiff: __m256i = _mm256_madd_epi16(v_diff, v_diff);
        v_sum_d = _mm256_add_epi16(v_sum_d, v_diff);
        v_sse_d = _mm256_add_epi32(v_sse_d, v_sqrdiff);
        src = src.offset((src_stride * 2) as isize);
        ref_ = ref_.offset((ref_stride * 2) as isize);
    }
    let mut v_sum00: __m256i = _mm256_cvtepi16_epi32(_mm256_castsi256_si128(v_sum_d));
    let mut v_sum01: __m256i = _mm256_cvtepi16_epi32(_mm256_extracti128_si256::<1>(v_sum_d));
    let mut v_sum0: __m256i = _mm256_add_epi32(v_sum00, v_sum01);
    let mut v_d_l: __m256i = _mm256_unpacklo_epi32(v_sum0, v_sse_d);
    let mut v_d_h: __m256i = _mm256_unpackhi_epi32(v_sum0, v_sse_d);
    let mut v_d_lh: __m256i = _mm256_add_epi32(v_d_l, v_d_h);
    let v_d0_d: __m128i = _mm256_castsi256_si128(v_d_lh);
    let v_d1_d: __m128i = _mm256_extracti128_si256::<1>(v_d_lh);
    let mut v_d: __m128i = _mm_add_epi32(v_d0_d, v_d1_d);
    v_d = _mm_add_epi32(v_d, _mm_srli_si128::<8>(v_d));
    *sum = _mm_extract_epi32::<0>(v_d);
    *sse = _mm_extract_epi32::<1>(v_d) as u32;
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn highbd_calc16x16var_avx2(
    mut src: *const u16,
    src_stride: i32,
    mut ref_: *const u16,
    ref_stride: i32,
    sse: *mut u32,
    sum: *mut i32,
) {
    let mut v_sum_d: __m256i = _mm256_setzero_si256();
    let mut v_sse_d: __m256i = _mm256_setzero_si256();
    let one: __m256i = _mm256_set1_epi16(1);
    for i in 0..16 {
        let v_p_a: __m256i = _mm256_loadu_si256(src as *const __m256i);
        let v_p_b: __m256i = _mm256_loadu_si256(ref_ as *const __m256i);
        let v_diff: __m256i = _mm256_sub_epi16(v_p_a, v_p_b);
        let v_sqrdiff: __m256i = _mm256_madd_epi16(v_diff, v_diff);
        v_sum_d = _mm256_add_epi16(v_sum_d, v_diff);
        v_sse_d = _mm256_add_epi32(v_sse_d, v_sqrdiff);
        src = src.offset(src_stride as isize);
        ref_ = ref_.offset(ref_stride as isize);
    }
    let mut v_sum0: __m256i = _mm256_madd_epi16(v_sum_d, one);
    let mut v_d_l: __m256i = _mm256_unpacklo_epi32(v_sum0, v_sse_d);
    let mut v_d_h: __m256i = _mm256_unpackhi_epi32(v_sum0, v_sse_d);
    let mut v_d_lh: __m256i = _mm256_add_epi32(v_d_l, v_d_h);
    let v_d0_d: __m128i = _mm256_castsi256_si128(v_d_lh);
    let v_d1_d: __m128i = _mm256_extracti128_si256::<1>(v_d_lh);
    let mut v_d: __m128i = _mm_add_epi32(v_d0_d, v_d1_d);
    v_d = _mm_add_epi32(v_d, _mm_srli_si128::<8>(v_d));
    *sum = _mm_extract_epi32::<0>(v_d);
    *sse = _mm_extract_epi32::<1>(v_d) as u32;
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn highbd_10_variance_avx2(
    src: *const u16,
    src_stride: i32,
    ref_: *const u16,
    ref_stride: i32,
    w: i32,
    h: i32,
    sse: *mut u32,
    sum: *mut i32,
    var_fn: high_variance_fn_t,
    block_size: i32,
) {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut sse_long: u64 = 0;
    let mut sum_long: i32 = 0;
    {
        i = 0;
        while i < h {
            {
                j = 0;
                while j < w {
                    let mut sse0: u32 = 0;
                    let mut sum0: i32 = 0;
                    var_fn.unwrap()(
                        src.offset((src_stride * i) as isize).offset(j as isize),
                        src_stride,
                        ref_.offset((ref_stride * i) as isize).offset(j as isize),
                        ref_stride,
                        &raw mut sse0,
                        &raw mut sum0,
                    );
                    sse_long += sse0 as u64;
                    sum_long += sum0;
                    j += block_size;
                }
            }
            i += block_size;
        }
    }
    *sum = (sum_long + ((1 << 2) >> 1)) >> 2;
    *sse = ((sse_long + ((1 << 4) >> 1) as u64) >> 4) as u32;
}

#[no_mangle]
#[target_feature(enable = "avx2")]
pub unsafe extern "C-unwind" fn aom_highbd_10_variance128x128_avx2(
    src8: *const u8,
    src_stride: i32,
    ref8: *const u8,
    ref_stride: i32,
    sse: *mut u32,
) -> u32 {
    let mut sum: i32 = 0;
    let mut var: i64 = 0;
    let mut src: *mut u16 = ((src8 as usize) << 1) as *mut u16;
    let mut ref_: *mut u16 = ((ref8 as usize) << 1) as *mut u16;
    highbd_10_variance_avx2(
        src as *const u16,
        src_stride,
        ref_ as *const u16,
        ref_stride,
        128,
        128,
        sse,
        &raw mut sum,
        Some(highbd_calc16x16var_avx2),
        16,
    );
    var = *sse as i64 - ((sum as i64 * sum as i64) >> 14);
    return if var >= 0 { var as u32 } else { 0 };
}

#[no_mangle]
#[target_feature(enable = "avx2")]
pub unsafe extern "C-unwind" fn aom_highbd_10_variance128x64_avx2(
    src8: *const u8,
    src_stride: i32,
    ref8: *const u8,
    ref_stride: i32,
    sse: *mut u32,
) -> u32 {
    let mut sum: i32 = 0;
    let mut var: i64 = 0;
    let mut src: *mut u16 = ((src8 as usize) << 1) as *mut u16;
    let mut ref_: *mut u16 = ((ref8 as usize) << 1) as *mut u16;
    highbd_10_variance_avx2(
        src as *const u16,
        src_stride,
        ref_ as *const u16,
        ref_stride,
        128,
        64,
        sse,
        &raw mut sum,
        Some(highbd_calc16x16var_avx2),
        16,
    );
    var = *sse as i64 - ((sum as i64 * sum as i64) >> 13);
    return if var >= 0 { var as u32 } else { 0 };
}

#[no_mangle]
#[target_feature(enable = "avx2")]
pub unsafe extern "C-unwind" fn aom_highbd_10_variance64x128_avx2(
    src8: *const u8,
    src_stride: i32,
    ref8: *const u8,
    ref_stride: i32,
    sse: *mut u32,
) -> u32 {
    let mut sum: i32 = 0;
    let mut var: i64 = 0;
    let mut src: *mut u16 = ((src8 as usize) << 1) as *mut u16;
    let mut ref_: *mut u16 = ((ref8 as usize) << 1) as *mut u16;
    highbd_10_variance_avx2(
        src as *const u16,
        src_stride,
        ref_ as *const u16,
        ref_stride,
        64,
        128,
        sse,
        &raw mut sum,
        Some(highbd_calc16x16var_avx2),
        16,
    );
    var = *sse as i64 - ((sum as i64 * sum as i64) >> 13);
    return if var >= 0 { var as u32 } else { 0 };
}

#[no_mangle]
#[target_feature(enable = "avx2")]
pub unsafe extern "C-unwind" fn aom_highbd_10_variance64x64_avx2(
    src8: *const u8,
    src_stride: i32,
    ref8: *const u8,
    ref_stride: i32,
    sse: *mut u32,
) -> u32 {
    let mut sum: i32 = 0;
    let mut var: i64 = 0;
    let mut src: *mut u16 = ((src8 as usize) << 1) as *mut u16;
    let mut ref_: *mut u16 = ((ref8 as usize) << 1) as *mut u16;
    highbd_10_variance_avx2(
        src as *const u16,
        src_stride,
        ref_ as *const u16,
        ref_stride,
        64,
        64,
        sse,
        &raw mut sum,
        Some(highbd_calc16x16var_avx2),
        16,
    );
    var = *sse as i64 - ((sum as i64 * sum as i64) >> 12);
    return if var >= 0 { var as u32 } else { 0 };
}

#[no_mangle]
#[target_feature(enable = "avx2")]
pub unsafe extern "C-unwind" fn aom_highbd_10_variance64x32_avx2(
    src8: *const u8,
    src_stride: i32,
    ref8: *const u8,
    ref_stride: i32,
    sse: *mut u32,
) -> u32 {
    let mut sum: i32 = 0;
    let mut var: i64 = 0;
    let mut src: *mut u16 = ((src8 as usize) << 1) as *mut u16;
    let mut ref_: *mut u16 = ((ref8 as usize) << 1) as *mut u16;
    highbd_10_variance_avx2(
        src as *const u16,
        src_stride,
        ref_ as *const u16,
        ref_stride,
        64,
        32,
        sse,
        &raw mut sum,
        Some(highbd_calc16x16var_avx2),
        16,
    );
    var = *sse as i64 - ((sum as i64 * sum as i64) >> 11);
    return if var >= 0 { var as u32 } else { 0 };
}

#[no_mangle]
#[target_feature(enable = "avx2")]
pub unsafe extern "C-unwind" fn aom_highbd_10_variance32x64_avx2(
    src8: *const u8,
    src_stride: i32,
    ref8: *const u8,
    ref_stride: i32,
    sse: *mut u32,
) -> u32 {
    let mut sum: i32 = 0;
    let mut var: i64 = 0;
    let mut src: *mut u16 = ((src8 as usize) << 1) as *mut u16;
    let mut ref_: *mut u16 = ((ref8 as usize) << 1) as *mut u16;
    highbd_10_variance_avx2(
        src as *const u16,
        src_stride,
        ref_ as *const u16,
        ref_stride,
        32,
        64,
        sse,
        &raw mut sum,
        Some(highbd_calc16x16var_avx2),
        16,
    );
    var = *sse as i64 - ((sum as i64 * sum as i64) >> 11);
    return if var >= 0 { var as u32 } else { 0 };
}

#[no_mangle]
#[target_feature(enable = "avx2")]
pub unsafe extern "C-unwind" fn aom_highbd_10_variance32x32_avx2(
    src8: *const u8,
    src_stride: i32,
    ref8: *const u8,
    ref_stride: i32,
    sse: *mut u32,
) -> u32 {
    let mut sum: i32 = 0;
    let mut var: i64 = 0;
    let mut src: *mut u16 = ((src8 as usize) << 1) as *mut u16;
    let mut ref_: *mut u16 = ((ref8 as usize) << 1) as *mut u16;
    highbd_10_variance_avx2(
        src as *const u16,
        src_stride,
        ref_ as *const u16,
        ref_stride,
        32,
        32,
        sse,
        &raw mut sum,
        Some(highbd_calc16x16var_avx2),
        16,
    );
    var = *sse as i64 - ((sum as i64 * sum as i64) >> 10);
    return if var >= 0 { var as u32 } else { 0 };
}

#[no_mangle]
#[target_feature(enable = "avx2")]
pub unsafe extern "C-unwind" fn aom_highbd_10_variance32x16_avx2(
    src8: *const u8,
    src_stride: i32,
    ref8: *const u8,
    ref_stride: i32,
    sse: *mut u32,
) -> u32 {
    let mut sum: i32 = 0;
    let mut var: i64 = 0;
    let mut src: *mut u16 = ((src8 as usize) << 1) as *mut u16;
    let mut ref_: *mut u16 = ((ref8 as usize) << 1) as *mut u16;
    highbd_10_variance_avx2(
        src as *const u16,
        src_stride,
        ref_ as *const u16,
        ref_stride,
        32,
        16,
        sse,
        &raw mut sum,
        Some(highbd_calc16x16var_avx2),
        16,
    );
    var = *sse as i64 - ((sum as i64 * sum as i64) >> 9);
    return if var >= 0 { var as u32 } else { 0 };
}

#[no_mangle]
#[target_feature(enable = "avx2")]
pub unsafe extern "C-unwind" fn aom_highbd_10_variance16x32_avx2(
    src8: *const u8,
    src_stride: i32,
    ref8: *const u8,
    ref_stride: i32,
    sse: *mut u32,
) -> u32 {
    let mut sum: i32 = 0;
    let mut var: i64 = 0;
    let mut src: *mut u16 = ((src8 as usize) << 1) as *mut u16;
    let mut ref_: *mut u16 = ((ref8 as usize) << 1) as *mut u16;
    highbd_10_variance_avx2(
        src as *const u16,
        src_stride,
        ref_ as *const u16,
        ref_stride,
        16,
        32,
        sse,
        &raw mut sum,
        Some(highbd_calc16x16var_avx2),
        16,
    );
    var = *sse as i64 - ((sum as i64 * sum as i64) >> 9);
    return if var >= 0 { var as u32 } else { 0 };
}

#[no_mangle]
#[target_feature(enable = "avx2")]
pub unsafe extern "C-unwind" fn aom_highbd_10_variance16x16_avx2(
    src8: *const u8,
    src_stride: i32,
    ref8: *const u8,
    ref_stride: i32,
    sse: *mut u32,
) -> u32 {
    let mut sum: i32 = 0;
    let mut var: i64 = 0;
    let mut src: *mut u16 = ((src8 as usize) << 1) as *mut u16;
    let mut ref_: *mut u16 = ((ref8 as usize) << 1) as *mut u16;
    highbd_10_variance_avx2(
        src as *const u16,
        src_stride,
        ref_ as *const u16,
        ref_stride,
        16,
        16,
        sse,
        &raw mut sum,
        Some(highbd_calc16x16var_avx2),
        16,
    );
    var = *sse as i64 - ((sum as i64 * sum as i64) >> 8);
    return if var >= 0 { var as u32 } else { 0 };
}

#[no_mangle]
#[target_feature(enable = "avx2")]
pub unsafe extern "C-unwind" fn aom_highbd_10_variance16x8_avx2(
    src8: *const u8,
    src_stride: i32,
    ref8: *const u8,
    ref_stride: i32,
    sse: *mut u32,
) -> u32 {
    let mut sum: i32 = 0;
    let mut var: i64 = 0;
    let mut src: *mut u16 = ((src8 as usize) << 1) as *mut u16;
    let mut ref_: *mut u16 = ((ref8 as usize) << 1) as *mut u16;
    highbd_10_variance_avx2(
        src as *const u16,
        src_stride,
        ref_ as *const u16,
        ref_stride,
        16,
        8,
        sse,
        &raw mut sum,
        Some(highbd_calc8x8var_avx2),
        8,
    );
    var = *sse as i64 - ((sum as i64 * sum as i64) >> 7);
    return if var >= 0 { var as u32 } else { 0 };
}

#[no_mangle]
#[target_feature(enable = "avx2")]
pub unsafe extern "C-unwind" fn aom_highbd_10_variance8x16_avx2(
    src8: *const u8,
    src_stride: i32,
    ref8: *const u8,
    ref_stride: i32,
    sse: *mut u32,
) -> u32 {
    let mut sum: i32 = 0;
    let mut var: i64 = 0;
    let mut src: *mut u16 = ((src8 as usize) << 1) as *mut u16;
    let mut ref_: *mut u16 = ((ref8 as usize) << 1) as *mut u16;
    highbd_10_variance_avx2(
        src as *const u16,
        src_stride,
        ref_ as *const u16,
        ref_stride,
        8,
        16,
        sse,
        &raw mut sum,
        Some(highbd_calc8x8var_avx2),
        8,
    );
    var = *sse as i64 - ((sum as i64 * sum as i64) >> 7);
    return if var >= 0 { var as u32 } else { 0 };
}

#[no_mangle]
#[target_feature(enable = "avx2")]
pub unsafe extern "C-unwind" fn aom_highbd_10_variance8x8_avx2(
    src8: *const u8,
    src_stride: i32,
    ref8: *const u8,
    ref_stride: i32,
    sse: *mut u32,
) -> u32 {
    let mut sum: i32 = 0;
    let mut var: i64 = 0;
    let mut src: *mut u16 = ((src8 as usize) << 1) as *mut u16;
    let mut ref_: *mut u16 = ((ref8 as usize) << 1) as *mut u16;
    highbd_10_variance_avx2(
        src as *const u16,
        src_stride,
        ref_ as *const u16,
        ref_stride,
        8,
        8,
        sse,
        &raw mut sum,
        Some(highbd_calc8x8var_avx2),
        8,
    );
    var = *sse as i64 - ((sum as i64 * sum as i64) >> 6);
    return if var >= 0 { var as u32 } else { 0 };
}

#[no_mangle]
#[target_feature(enable = "avx2")]
pub unsafe extern "C-unwind" fn aom_highbd_10_variance16x64_avx2(
    src8: *const u8,
    src_stride: i32,
    ref8: *const u8,
    ref_stride: i32,
    sse: *mut u32,
) -> u32 {
    let mut sum: i32 = 0;
    let mut var: i64 = 0;
    let mut src: *mut u16 = ((src8 as usize) << 1) as *mut u16;
    let mut ref_: *mut u16 = ((ref8 as usize) << 1) as *mut u16;
    highbd_10_variance_avx2(
        src as *const u16,
        src_stride,
        ref_ as *const u16,
        ref_stride,
        16,
        64,
        sse,
        &raw mut sum,
        Some(highbd_calc16x16var_avx2),
        16,
    );
    var = *sse as i64 - ((sum as i64 * sum as i64) >> 10);
    return if var >= 0 { var as u32 } else { 0 };
}

#[no_mangle]
#[target_feature(enable = "avx2")]
pub unsafe extern "C-unwind" fn aom_highbd_10_variance32x8_avx2(
    src8: *const u8,
    src_stride: i32,
    ref8: *const u8,
    ref_stride: i32,
    sse: *mut u32,
) -> u32 {
    let mut sum: i32 = 0;
    let mut var: i64 = 0;
    let mut src: *mut u16 = ((src8 as usize) << 1) as *mut u16;
    let mut ref_: *mut u16 = ((ref8 as usize) << 1) as *mut u16;
    highbd_10_variance_avx2(
        src as *const u16,
        src_stride,
        ref_ as *const u16,
        ref_stride,
        32,
        8,
        sse,
        &raw mut sum,
        Some(highbd_calc8x8var_avx2),
        8,
    );
    var = *sse as i64 - ((sum as i64 * sum as i64) >> 8);
    return if var >= 0 { var as u32 } else { 0 };
}

#[no_mangle]
#[target_feature(enable = "avx2")]
pub unsafe extern "C-unwind" fn aom_highbd_10_variance64x16_avx2(
    src8: *const u8,
    src_stride: i32,
    ref8: *const u8,
    ref_stride: i32,
    sse: *mut u32,
) -> u32 {
    let mut sum: i32 = 0;
    let mut var: i64 = 0;
    let mut src: *mut u16 = ((src8 as usize) << 1) as *mut u16;
    let mut ref_: *mut u16 = ((ref8 as usize) << 1) as *mut u16;
    highbd_10_variance_avx2(
        src as *const u16,
        src_stride,
        ref_ as *const u16,
        ref_stride,
        64,
        16,
        sse,
        &raw mut sum,
        Some(highbd_calc16x16var_avx2),
        16,
    );
    var = *sse as i64 - ((sum as i64 * sum as i64) >> 10);
    return if var >= 0 { var as u32 } else { 0 };
}

#[no_mangle]
#[target_feature(enable = "avx2")]
pub unsafe extern "C-unwind" fn aom_highbd_10_variance8x32_avx2(
    src8: *const u8,
    src_stride: i32,
    ref8: *const u8,
    ref_stride: i32,
    sse: *mut u32,
) -> u32 {
    let mut sum: i32 = 0;
    let mut var: i64 = 0;
    let mut src: *mut u16 = ((src8 as usize) << 1) as *mut u16;
    let mut ref_: *mut u16 = ((ref8 as usize) << 1) as *mut u16;
    highbd_10_variance_avx2(
        src as *const u16,
        src_stride,
        ref_ as *const u16,
        ref_stride,
        8,
        32,
        sse,
        &raw mut sum,
        Some(highbd_calc8x8var_avx2),
        8,
    );
    var = *sse as i64 - ((sum as i64 * sum as i64) >> 8);
    return if var >= 0 { var as u32 } else { 0 };
}

#[no_mangle]
#[target_feature(enable = "avx2")]
pub unsafe extern "C-unwind" fn aom_highbd_10_mse16x16_avx2(
    src8: *const u8,
    src_stride: i32,
    ref8: *const u8,
    ref_stride: i32,
    sse: *mut u32,
) -> u32 {
    let mut sum: i32 = 0;
    let mut src: *mut u16 = ((src8 as usize) << 1) as *mut u16;
    let mut ref_: *mut u16 = ((ref8 as usize) << 1) as *mut u16;
    highbd_10_variance_avx2(
        src as *const u16,
        src_stride,
        ref_ as *const u16,
        ref_stride,
        16,
        16,
        sse,
        &raw mut sum,
        Some(highbd_calc16x16var_avx2),
        16,
    );
    return *sse;
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn mse_4xh_16bit_highbd_avx2(
    dst: *mut u16,
    dstride: i32,
    src: *mut u16,
    sstride: i32,
    h: i32,
) -> u64 {
    let mut sum: u64 = 0;
    let mut reg0_4x16: __m128i;
    let mut reg1_4x16: __m128i;
    let mut reg2_4x16: __m128i;
    let mut reg3_4x16: __m128i;
    let mut src0_8x16: __m256i;
    let mut src1_8x16: __m256i;
    let mut src_16x16: __m256i;
    let mut dst0_8x16: __m256i;
    let mut dst1_8x16: __m256i;
    let mut dst_16x16: __m256i;
    let mut res0_4x64: __m256i;
    let mut res1_4x64: __m256i;
    let mut res2_4x64: __m256i;
    let mut res3_4x64: __m256i;
    let mut sub_result: __m256i;
    let zeros: __m256i = _mm256_broadcastsi128_si256(_mm_setzero_si128());
    let mut square_result: __m256i = _mm256_broadcastsi128_si256(_mm_setzero_si128());
    for i in (0..h).step_by(4) {
        reg0_4x16 = _mm_loadl_epi64(dst.offset(((i + 0) * dstride) as isize) as *const __m128i);
        reg1_4x16 = _mm_loadl_epi64(dst.offset(((i + 1) * dstride) as isize) as *const __m128i);
        reg2_4x16 = _mm_loadl_epi64(dst.offset(((i + 2) * dstride) as isize) as *const __m128i);
        reg3_4x16 = _mm_loadl_epi64(dst.offset(((i + 3) * dstride) as isize) as *const __m128i);
        dst0_8x16 = _mm256_castsi128_si256(_mm_unpacklo_epi64(reg0_4x16, reg1_4x16));
        dst1_8x16 = _mm256_castsi128_si256(_mm_unpacklo_epi64(reg2_4x16, reg3_4x16));
        dst_16x16 = _mm256_permute2x128_si256::<0x20>(dst0_8x16, dst1_8x16);
        reg0_4x16 = _mm_loadl_epi64(src.offset(((i + 0) * sstride) as isize) as *const __m128i);
        reg1_4x16 = _mm_loadl_epi64(src.offset(((i + 1) * sstride) as isize) as *const __m128i);
        reg2_4x16 = _mm_loadl_epi64(src.offset(((i + 2) * sstride) as isize) as *const __m128i);
        reg3_4x16 = _mm_loadl_epi64(src.offset(((i + 3) * sstride) as isize) as *const __m128i);
        src0_8x16 = _mm256_castsi128_si256(_mm_unpacklo_epi64(reg0_4x16, reg1_4x16));
        src1_8x16 = _mm256_castsi128_si256(_mm_unpacklo_epi64(reg2_4x16, reg3_4x16));
        src_16x16 = _mm256_permute2x128_si256::<0x20>(src0_8x16, src1_8x16);
        sub_result = _mm256_abs_epi16(_mm256_sub_epi16(src_16x16, dst_16x16));
        src_16x16 = _mm256_unpacklo_epi16(sub_result, zeros);
        dst_16x16 = _mm256_unpackhi_epi16(sub_result, zeros);
        src_16x16 = _mm256_madd_epi16(src_16x16, src_16x16);
        dst_16x16 = _mm256_madd_epi16(dst_16x16, dst_16x16);
        res0_4x64 = _mm256_unpacklo_epi32(src_16x16, zeros);
        res1_4x64 = _mm256_unpackhi_epi32(src_16x16, zeros);
        res2_4x64 = _mm256_unpacklo_epi32(dst_16x16, zeros);
        res3_4x64 = _mm256_unpackhi_epi32(dst_16x16, zeros);
        square_result = _mm256_add_epi64(
            square_result,
            _mm256_add_epi64(
                _mm256_add_epi64(_mm256_add_epi64(res0_4x64, res1_4x64), res2_4x64),
                res3_4x64,
            ),
        );
    }
    let sum_2x64: __m128i = _mm_add_epi64(
        _mm256_castsi256_si128(square_result),
        _mm256_extracti128_si256::<1>(square_result),
    );
    let sum_1x64: __m128i = _mm_add_epi64(sum_2x64, _mm_srli_si128::<8>(sum_2x64));
    xx_storel_64(&raw mut sum as *mut core::ffi::c_void, sum_1x64);
    return sum;
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn mse_8xh_16bit_highbd_avx2(
    dst: *mut u16,
    dstride: i32,
    src: *mut u16,
    sstride: i32,
    h: i32,
) -> u64 {
    let mut sum: u64 = 0;
    let mut src0_8x16: __m256i;
    let mut src1_8x16: __m256i;
    let mut src_16x16: __m256i;
    let mut dst0_8x16: __m256i;
    let mut dst1_8x16: __m256i;
    let mut dst_16x16: __m256i;
    let mut res0_4x64: __m256i;
    let mut res1_4x64: __m256i;
    let mut res2_4x64: __m256i;
    let mut res3_4x64: __m256i;
    let mut sub_result: __m256i;
    let zeros: __m256i = _mm256_broadcastsi128_si256(_mm_setzero_si128());
    let mut square_result: __m256i = _mm256_broadcastsi128_si256(_mm_setzero_si128());
    for i in (0..h).step_by(2) {
        dst0_8x16 = _mm256_castsi128_si256(_mm_loadu_si128(dst.offset((i * dstride) as isize)
            as *mut __m128i
            as *const __m128i));
        dst1_8x16 =
            _mm256_castsi128_si256(_mm_loadu_si128(dst.offset(((i + 1) * dstride) as isize)
                as *mut __m128i
                as *const __m128i));
        dst_16x16 = _mm256_permute2x128_si256::<0x20>(dst0_8x16, dst1_8x16);
        src0_8x16 = _mm256_castsi128_si256(_mm_loadu_si128(src.offset((i * sstride) as isize)
            as *mut __m128i
            as *const __m128i));
        src1_8x16 =
            _mm256_castsi128_si256(_mm_loadu_si128(src.offset(((i + 1) * sstride) as isize)
                as *mut __m128i
                as *const __m128i));
        src_16x16 = _mm256_permute2x128_si256::<0x20>(src0_8x16, src1_8x16);
        sub_result = _mm256_abs_epi16(_mm256_sub_epi16(src_16x16, dst_16x16));
        src_16x16 = _mm256_unpacklo_epi16(sub_result, zeros);
        dst_16x16 = _mm256_unpackhi_epi16(sub_result, zeros);
        src_16x16 = _mm256_madd_epi16(src_16x16, src_16x16);
        dst_16x16 = _mm256_madd_epi16(dst_16x16, dst_16x16);
        res0_4x64 = _mm256_unpacklo_epi32(src_16x16, zeros);
        res1_4x64 = _mm256_unpackhi_epi32(src_16x16, zeros);
        res2_4x64 = _mm256_unpacklo_epi32(dst_16x16, zeros);
        res3_4x64 = _mm256_unpackhi_epi32(dst_16x16, zeros);
        square_result = _mm256_add_epi64(
            square_result,
            _mm256_add_epi64(
                _mm256_add_epi64(_mm256_add_epi64(res0_4x64, res1_4x64), res2_4x64),
                res3_4x64,
            ),
        );
    }
    let sum_2x64: __m128i = _mm_add_epi64(
        _mm256_castsi256_si128(square_result),
        _mm256_extracti128_si256::<1>(square_result),
    );
    let sum_1x64: __m128i = _mm_add_epi64(sum_2x64, _mm_srli_si128::<8>(sum_2x64));
    xx_storel_64(&raw mut sum as *mut core::ffi::c_void, sum_1x64);
    return sum;
}

#[no_mangle]
#[target_feature(enable = "avx2")]
pub unsafe extern "C-unwind" fn aom_mse_wxh_16bit_highbd_avx2(
    dst: *mut u16,
    dstride: i32,
    src: *mut u16,
    sstride: i32,
    w: i32,
    h: i32,
) -> u64 {
    debug_assert!(
        (w == 8 || w == 4) && (h == 8 || h == 4),
        "w=8/4 and h=8/4 must satisfy"
    );
    match w {
        4 => {
            return mse_4xh_16bit_highbd_avx2(dst, dstride, src, sstride, h);
        }
        8 => {
            return mse_8xh_16bit_highbd_avx2(dst, dstride, src, sstride, h);
        }
        _ => {
            debug_assert!(0 != 0, "unsupported width");
            return (-1_i32) as u64;
        }
    }
}
