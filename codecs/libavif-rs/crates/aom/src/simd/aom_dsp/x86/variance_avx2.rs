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

// Translated from upstream/aom/aom_dsp/x86/variance_avx2.c (ADR 0056, ADR 0058).
// Upstream compiles it with -mavx2. The functions were
// transliterated with a typing-aware C-subset printer (not C2Rust) and
// reviewed against the C function by function: the same sequence of intrinsic
// and helper calls, constants and operators in order (ADR 0058).
// Only the functions of this file that the still-image encoder dispatches (the block
// variances and MSEs, aom_get_var_sse_sum_*, aom_mse_wxh_16bit*, aom_mse_16xh_16bit*) and
// the static helpers they call are translated; the macros that define the block
// variances are expanded by the preprocessor. The sub-pixel and averaging kernels are
// not translated (working decision O2, 2026-11-13, delegated, revisable by the owner).
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
unsafe fn mm256_add_hi_lo_epi16(val: __m256i) -> __m128i {
    return _mm_add_epi16(
        _mm256_castsi256_si128(val),
        _mm256_extractf128_si256::<1>(val),
    );
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn mm256_add_hi_lo_epi32(val: __m256i) -> __m128i {
    return _mm_add_epi32(
        _mm256_castsi256_si128(val),
        _mm256_extractf128_si256::<1>(val),
    );
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn variance_kernel_avx2(src: __m256i, ref_: __m256i, sse: *mut __m256i, sum: *mut __m256i) {
    let adj_sub: __m256i = _mm256_set1_epi16(0xff01_i32 as i16); // (1,-1)
                                                                 // unpack into pairs of source and reference values
    let src_ref0: __m256i = _mm256_unpacklo_epi8(src, ref_);
    let src_ref1: __m256i = _mm256_unpackhi_epi8(src, ref_);
    // subtract adjacent elements using src*1 + ref*-1
    let diff0: __m256i = _mm256_maddubs_epi16(src_ref0, adj_sub);
    let diff1: __m256i = _mm256_maddubs_epi16(src_ref1, adj_sub);
    let madd0: __m256i = _mm256_madd_epi16(diff0, diff0);
    let madd1: __m256i = _mm256_madd_epi16(diff1, diff1);
    // add to the running totals
    *sum = _mm256_add_epi16(*sum, _mm256_add_epi16(diff0, diff1));
    *sse = _mm256_add_epi32(*sse, _mm256_add_epi32(madd0, madd1));
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn variance_final_from_32bit_sum_avx2(vsse: __m256i, vsum: __m128i, sse: *mut u32) -> i32 {
    // extract the low lane and add it to the high lane
    let sse_reg_128: __m128i = mm256_add_hi_lo_epi32(vsse);
    // unpack sse and sum registers and add
    let sse_sum_lo: __m128i = _mm_unpacklo_epi32(sse_reg_128, vsum);
    let sse_sum_hi: __m128i = _mm_unpackhi_epi32(sse_reg_128, vsum);
    let sse_sum: __m128i = _mm_add_epi32(sse_sum_lo, sse_sum_hi);
    // perform the final summation and extract the results
    let res: __m128i = _mm_add_epi32(sse_sum, _mm_srli_si128::<8>(sse_sum));
    core::ptr::write_unaligned(sse as *mut i32, _mm_cvtsi128_si32(res));
    return _mm_extract_epi32::<1>(res);
}

// handle pixels (<= 512)
#[inline]
#[target_feature(enable = "avx2")]
unsafe fn variance_final_512_avx2(vsse: __m256i, vsum: __m256i, sse: *mut u32) -> i32 {
    // extract the low lane and add it to the high lane
    let vsum_128: __m128i = mm256_add_hi_lo_epi16(vsum);
    let vsum_64: __m128i = _mm_add_epi16(vsum_128, _mm_srli_si128::<8>(vsum_128));
    let sum_int32: __m128i = _mm_cvtepi16_epi32(vsum_64);
    return variance_final_from_32bit_sum_avx2(vsse, sum_int32, sse);
}

// handle 1024 pixels (32x32, 16x64, 64x16)
#[inline]
#[target_feature(enable = "avx2")]
unsafe fn variance_final_1024_avx2(vsse: __m256i, vsum: __m256i, sse: *mut u32) -> i32 {
    // extract the low lane and add it to the high lane
    let vsum_128: __m128i = mm256_add_hi_lo_epi16(vsum);
    let vsum_64: __m128i = _mm_add_epi32(
        _mm_cvtepi16_epi32(vsum_128),
        _mm_cvtepi16_epi32(_mm_srli_si128::<8>(vsum_128)),
    );
    return variance_final_from_32bit_sum_avx2(vsse, vsum_64, sse);
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn sum_to_32bit_avx2(sum: __m256i) -> __m256i {
    let sum_lo: __m256i = _mm256_cvtepi16_epi32(_mm256_castsi256_si128(sum));
    let sum_hi: __m256i = _mm256_cvtepi16_epi32(_mm256_extractf128_si256::<1>(sum));
    return _mm256_add_epi32(sum_lo, sum_hi);
}

// handle 2048 pixels (32x64, 64x32)
#[inline]
#[target_feature(enable = "avx2")]
unsafe fn variance_final_2048_avx2(vsse: __m256i, mut vsum: __m256i, sse: *mut u32) -> i32 {
    vsum = sum_to_32bit_avx2(vsum);
    let vsum_128: __m128i = mm256_add_hi_lo_epi32(vsum);
    return variance_final_from_32bit_sum_avx2(vsse, vsum_128, sse);
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn variance16_kernel_avx2(
    src: *const u8,
    src_stride: i32,
    ref_: *const u8,
    ref_stride: i32,
    sse: *mut __m256i,
    sum: *mut __m256i,
) {
    let s0: __m128i = _mm_loadu_si128(src.offset((0 * src_stride) as isize) as *const __m128i);
    let s1: __m128i = _mm_loadu_si128(src.offset((1 * src_stride) as isize) as *const __m128i);
    let r0: __m128i = _mm_loadu_si128(ref_.offset((0 * ref_stride) as isize) as *const __m128i);
    let r1: __m128i = _mm_loadu_si128(ref_.offset((1 * ref_stride) as isize) as *const __m128i);
    let s: __m256i = _mm256_inserti128_si256::<1>(_mm256_castsi128_si256(s0), s1);
    let r: __m256i = _mm256_inserti128_si256::<1>(_mm256_castsi128_si256(r0), r1);
    variance_kernel_avx2(s, r, sse, sum);
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn variance32_kernel_avx2(
    src: *const u8,
    ref_: *const u8,
    sse: *mut __m256i,
    sum: *mut __m256i,
) {
    let s: __m256i = _mm256_loadu_si256(src as *const __m256i);
    let r: __m256i = _mm256_loadu_si256(ref_ as *const __m256i);
    variance_kernel_avx2(s, r, sse, sum);
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn variance16_avx2(
    mut src: *const u8,
    src_stride: i32,
    mut ref_: *const u8,
    ref_stride: i32,
    h: i32,
    vsse: *mut __m256i,
    vsum: *mut __m256i,
) {
    *vsum = _mm256_setzero_si256();
    for i in (0..h).step_by(2) {
        variance16_kernel_avx2(src, src_stride, ref_, ref_stride, vsse, vsum);
        src = src.offset((2 * src_stride) as isize);
        ref_ = ref_.offset((2 * ref_stride) as isize);
    }
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn variance32_avx2(
    mut src: *const u8,
    src_stride: i32,
    mut ref_: *const u8,
    ref_stride: i32,
    h: i32,
    vsse: *mut __m256i,
    vsum: *mut __m256i,
) {
    *vsum = _mm256_setzero_si256();
    for i in 0..h {
        variance32_kernel_avx2(src, ref_, vsse, vsum);
        src = src.offset(src_stride as isize);
        ref_ = ref_.offset(ref_stride as isize);
    }
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn variance64_avx2(
    mut src: *const u8,
    src_stride: i32,
    mut ref_: *const u8,
    ref_stride: i32,
    h: i32,
    vsse: *mut __m256i,
    vsum: *mut __m256i,
) {
    *vsum = _mm256_setzero_si256();
    for i in 0..h {
        variance32_kernel_avx2(src, ref_, vsse, vsum);
        variance32_kernel_avx2(src.offset(32), ref_.offset(32), vsse, vsum);
        src = src.offset(src_stride as isize);
        ref_ = ref_.offset(ref_stride as isize);
    }
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn variance128_avx2(
    mut src: *const u8,
    src_stride: i32,
    mut ref_: *const u8,
    ref_stride: i32,
    h: i32,
    vsse: *mut __m256i,
    vsum: *mut __m256i,
) {
    *vsum = _mm256_setzero_si256();
    for i in 0..h {
        variance32_kernel_avx2(src, ref_, vsse, vsum);
        variance32_kernel_avx2(src.offset(32), ref_.offset(32), vsse, vsum);
        variance32_kernel_avx2(src.offset(64), ref_.offset(64), vsse, vsum);
        variance32_kernel_avx2(src.offset(96), ref_.offset(96), vsse, vsum);
        src = src.offset(src_stride as isize);
        ref_ = ref_.offset(ref_stride as isize);
    }
}

#[no_mangle]
#[target_feature(enable = "avx2")]
pub unsafe extern "C-unwind" fn aom_variance16x8_avx2(
    src: *const u8,
    src_stride: i32,
    ref_: *const u8,
    ref_stride: i32,
    sse: *mut u32,
) -> u32 {
    let mut vsse: __m256i = _mm256_setzero_si256();
    let mut vsum: __m256i = core::mem::zeroed();
    variance16_avx2(
        src,
        src_stride,
        ref_,
        ref_stride,
        8,
        &raw mut vsse,
        &raw mut vsum,
    );
    let sum: i32 = variance_final_512_avx2(vsse, vsum, sse);
    return *sse - ((sum as i64 * sum as i64) >> 7) as u32;
}

#[no_mangle]
#[target_feature(enable = "avx2")]
pub unsafe extern "C-unwind" fn aom_variance16x16_avx2(
    src: *const u8,
    src_stride: i32,
    ref_: *const u8,
    ref_stride: i32,
    sse: *mut u32,
) -> u32 {
    let mut vsse: __m256i = _mm256_setzero_si256();
    let mut vsum: __m256i = core::mem::zeroed();
    variance16_avx2(
        src,
        src_stride,
        ref_,
        ref_stride,
        16,
        &raw mut vsse,
        &raw mut vsum,
    );
    let sum: i32 = variance_final_512_avx2(vsse, vsum, sse);
    return *sse - ((sum as i64 * sum as i64) >> 8) as u32;
}

#[no_mangle]
#[target_feature(enable = "avx2")]
pub unsafe extern "C-unwind" fn aom_variance16x32_avx2(
    src: *const u8,
    src_stride: i32,
    ref_: *const u8,
    ref_stride: i32,
    sse: *mut u32,
) -> u32 {
    let mut vsse: __m256i = _mm256_setzero_si256();
    let mut vsum: __m256i = core::mem::zeroed();
    variance16_avx2(
        src,
        src_stride,
        ref_,
        ref_stride,
        32,
        &raw mut vsse,
        &raw mut vsum,
    );
    let sum: i32 = variance_final_512_avx2(vsse, vsum, sse);
    return *sse - ((sum as i64 * sum as i64) >> 9) as u32;
}

#[no_mangle]
#[target_feature(enable = "avx2")]
pub unsafe extern "C-unwind" fn aom_variance32x16_avx2(
    src: *const u8,
    src_stride: i32,
    ref_: *const u8,
    ref_stride: i32,
    sse: *mut u32,
) -> u32 {
    let mut vsse: __m256i = _mm256_setzero_si256();
    let mut vsum: __m256i = core::mem::zeroed();
    variance32_avx2(
        src,
        src_stride,
        ref_,
        ref_stride,
        16,
        &raw mut vsse,
        &raw mut vsum,
    );
    let sum: i32 = variance_final_512_avx2(vsse, vsum, sse);
    return *sse - ((sum as i64 * sum as i64) >> 9) as u32;
}

#[no_mangle]
#[target_feature(enable = "avx2")]
pub unsafe extern "C-unwind" fn aom_variance32x32_avx2(
    src: *const u8,
    src_stride: i32,
    ref_: *const u8,
    ref_stride: i32,
    sse: *mut u32,
) -> u32 {
    let mut vsse: __m256i = _mm256_setzero_si256();
    let mut vsum: __m256i = core::mem::zeroed();
    variance32_avx2(
        src,
        src_stride,
        ref_,
        ref_stride,
        32,
        &raw mut vsse,
        &raw mut vsum,
    );
    let sum: i32 = variance_final_1024_avx2(vsse, vsum, sse);
    return *sse - ((sum as i64 * sum as i64) >> 10) as u32;
}

#[no_mangle]
#[target_feature(enable = "avx2")]
pub unsafe extern "C-unwind" fn aom_variance32x64_avx2(
    src: *const u8,
    src_stride: i32,
    ref_: *const u8,
    ref_stride: i32,
    sse: *mut u32,
) -> u32 {
    let mut vsse: __m256i = _mm256_setzero_si256();
    let mut vsum: __m256i = core::mem::zeroed();
    variance32_avx2(
        src,
        src_stride,
        ref_,
        ref_stride,
        64,
        &raw mut vsse,
        &raw mut vsum,
    );
    let sum: i32 = variance_final_2048_avx2(vsse, vsum, sse);
    return *sse - ((sum as i64 * sum as i64) >> 11) as u32;
}

#[no_mangle]
#[target_feature(enable = "avx2")]
pub unsafe extern "C-unwind" fn aom_variance64x32_avx2(
    src: *const u8,
    src_stride: i32,
    ref_: *const u8,
    ref_stride: i32,
    sse: *mut u32,
) -> u32 {
    let mut vsse: __m256i = _mm256_setzero_si256();
    let mut vsum: __m256i = core::mem::zeroed();
    variance64_avx2(
        src,
        src_stride,
        ref_,
        ref_stride,
        32,
        &raw mut vsse,
        &raw mut vsum,
    );
    let sum: i32 = variance_final_2048_avx2(vsse, vsum, sse);
    return *sse - ((sum as i64 * sum as i64) >> 11) as u32;
}

#[no_mangle]
#[target_feature(enable = "avx2")]
pub unsafe extern "C-unwind" fn aom_variance64x16_avx2(
    src: *const u8,
    src_stride: i32,
    ref_: *const u8,
    ref_stride: i32,
    sse: *mut u32,
) -> u32 {
    let mut vsse: __m256i = _mm256_setzero_si256();
    let mut vsum: __m256i = core::mem::zeroed();
    variance64_avx2(
        src,
        src_stride,
        ref_,
        ref_stride,
        16,
        &raw mut vsse,
        &raw mut vsum,
    );
    let sum: i32 = variance_final_1024_avx2(vsse, vsum, sse);
    return *sse - ((sum as i64 * sum as i64) >> 10) as u32;
}

#[no_mangle]
#[target_feature(enable = "avx2")]
pub unsafe extern "C-unwind" fn aom_variance32x8_avx2(
    src: *const u8,
    src_stride: i32,
    ref_: *const u8,
    ref_stride: i32,
    sse: *mut u32,
) -> u32 {
    let mut vsse: __m256i = _mm256_setzero_si256();
    let mut vsum: __m256i = core::mem::zeroed();
    variance32_avx2(
        src,
        src_stride,
        ref_,
        ref_stride,
        8,
        &raw mut vsse,
        &raw mut vsum,
    );
    let sum: i32 = variance_final_512_avx2(vsse, vsum, sse);
    return *sse - ((sum as i64 * sum as i64) >> 8) as u32;
}

#[no_mangle]
#[target_feature(enable = "avx2")]
pub unsafe extern "C-unwind" fn aom_variance16x64_avx2(
    src: *const u8,
    src_stride: i32,
    ref_: *const u8,
    ref_stride: i32,
    sse: *mut u32,
) -> u32 {
    let mut vsse: __m256i = _mm256_setzero_si256();
    let mut vsum: __m256i = core::mem::zeroed();
    variance16_avx2(
        src,
        src_stride,
        ref_,
        ref_stride,
        64,
        &raw mut vsse,
        &raw mut vsum,
    );
    let sum: i32 = variance_final_1024_avx2(vsse, vsum, sse);
    return *sse - ((sum as i64 * sum as i64) >> 10) as u32;
}

#[no_mangle]
#[target_feature(enable = "avx2")]
pub unsafe extern "C-unwind" fn aom_variance16x4_avx2(
    src: *const u8,
    src_stride: i32,
    ref_: *const u8,
    ref_stride: i32,
    sse: *mut u32,
) -> u32 {
    let mut vsse: __m256i = _mm256_setzero_si256();
    let mut vsum: __m256i = core::mem::zeroed();
    variance16_avx2(
        src,
        src_stride,
        ref_,
        ref_stride,
        4,
        &raw mut vsse,
        &raw mut vsum,
    );
    let sum: i32 = variance_final_512_avx2(vsse, vsum, sse);
    return *sse - ((sum as i64 * sum as i64) >> 6) as u32;
}

#[no_mangle]
#[target_feature(enable = "avx2")]
pub unsafe extern "C-unwind" fn aom_variance64x64_avx2(
    mut src: *const u8,
    src_stride: i32,
    mut ref_: *const u8,
    ref_stride: i32,
    sse: *mut u32,
) -> u32 {
    let mut vsse: __m256i = _mm256_setzero_si256();
    let mut vsum: __m256i = _mm256_setzero_si256();
    for i in 0..64 / 32 {
        let mut vsum16: __m256i = core::mem::zeroed();
        variance64_avx2(
            src,
            src_stride,
            ref_,
            ref_stride,
            32,
            &raw mut vsse,
            &raw mut vsum16,
        );
        vsum = _mm256_add_epi32(vsum, sum_to_32bit_avx2(vsum16));
        src = src.offset((32 * src_stride) as isize);
        ref_ = ref_.offset((32 * ref_stride) as isize);
    }
    let vsum_128: __m128i = mm256_add_hi_lo_epi32(vsum);
    let sum: i32 = variance_final_from_32bit_sum_avx2(vsse, vsum_128, sse);
    return *sse - ((sum as i64 * sum as i64) >> 12) as u32;
}

// 64x32 * ( 64/32)
#[no_mangle]
#[target_feature(enable = "avx2")]
pub unsafe extern "C-unwind" fn aom_variance64x128_avx2(
    mut src: *const u8,
    src_stride: i32,
    mut ref_: *const u8,
    ref_stride: i32,
    sse: *mut u32,
) -> u32 {
    let mut vsse: __m256i = _mm256_setzero_si256();
    let mut vsum: __m256i = _mm256_setzero_si256();
    for i in 0..128 / 32 {
        let mut vsum16: __m256i = core::mem::zeroed();
        variance64_avx2(
            src,
            src_stride,
            ref_,
            ref_stride,
            32,
            &raw mut vsse,
            &raw mut vsum16,
        );
        vsum = _mm256_add_epi32(vsum, sum_to_32bit_avx2(vsum16));
        src = src.offset((32 * src_stride) as isize);
        ref_ = ref_.offset((32 * ref_stride) as isize);
    }
    let vsum_128: __m128i = mm256_add_hi_lo_epi32(vsum);
    let sum: i32 = variance_final_from_32bit_sum_avx2(vsse, vsum_128, sse);
    return *sse - ((sum as i64 * sum as i64) >> 13) as u32;
}

// 64x32 * (128/32)
#[no_mangle]
#[target_feature(enable = "avx2")]
pub unsafe extern "C-unwind" fn aom_variance128x64_avx2(
    mut src: *const u8,
    src_stride: i32,
    mut ref_: *const u8,
    ref_stride: i32,
    sse: *mut u32,
) -> u32 {
    let mut vsse: __m256i = _mm256_setzero_si256();
    let mut vsum: __m256i = _mm256_setzero_si256();
    for i in 0..64 / 16 {
        let mut vsum16: __m256i = core::mem::zeroed();
        variance128_avx2(
            src,
            src_stride,
            ref_,
            ref_stride,
            16,
            &raw mut vsse,
            &raw mut vsum16,
        );
        vsum = _mm256_add_epi32(vsum, sum_to_32bit_avx2(vsum16));
        src = src.offset((16 * src_stride) as isize);
        ref_ = ref_.offset((16 * ref_stride) as isize);
    }
    let vsum_128: __m128i = mm256_add_hi_lo_epi32(vsum);
    let sum: i32 = variance_final_from_32bit_sum_avx2(vsse, vsum_128, sse);
    return *sse - ((sum as i64 * sum as i64) >> 13) as u32;
}

// 128x16 * ( 64/16)
#[no_mangle]
#[target_feature(enable = "avx2")]
pub unsafe extern "C-unwind" fn aom_variance128x128_avx2(
    mut src: *const u8,
    src_stride: i32,
    mut ref_: *const u8,
    ref_stride: i32,
    sse: *mut u32,
) -> u32 {
    let mut vsse: __m256i = _mm256_setzero_si256();
    let mut vsum: __m256i = _mm256_setzero_si256();
    for i in 0..128 / 16 {
        let mut vsum16: __m256i = core::mem::zeroed();
        variance128_avx2(
            src,
            src_stride,
            ref_,
            ref_stride,
            16,
            &raw mut vsse,
            &raw mut vsum16,
        );
        vsum = _mm256_add_epi32(vsum, sum_to_32bit_avx2(vsum16));
        src = src.offset((16 * src_stride) as isize);
        ref_ = ref_.offset((16 * ref_stride) as isize);
    }
    let vsum_128: __m128i = mm256_add_hi_lo_epi32(vsum);
    let sum: i32 = variance_final_from_32bit_sum_avx2(vsse, vsum_128, sse);
    return *sse - ((sum as i64 * sum as i64) >> 14) as u32;
}

// 128x16 * (128/16)
#[no_mangle]
#[target_feature(enable = "avx2")]
pub unsafe extern "C-unwind" fn aom_mse16x16_avx2(
    src: *const u8,
    src_stride: i32,
    ref_: *const u8,
    ref_stride: i32,
    sse: *mut u32,
) -> u32 {
    aom_variance16x16_avx2(src, src_stride, ref_, ref_stride, sse);
    return *sse;
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn mse_4xh_16bit_avx2(
    dst: *mut u8,
    dstride: i32,
    src: *mut u16,
    sstride: i32,
    h: i32,
) -> u64 {
    let mut sum: u64 = 0;
    let mut dst0_4x8: __m128i;
    let mut dst1_4x8: __m128i;
    let mut dst2_4x8: __m128i;
    let mut dst3_4x8: __m128i;
    let mut dst_16x8: __m128i;
    let mut src0_4x16: __m128i;
    let mut src1_4x16: __m128i;
    let mut src2_4x16: __m128i;
    let mut src3_4x16: __m128i;
    let mut src0_8x16: __m256i;
    let mut src1_8x16: __m256i;
    let mut dst_16x16: __m256i;
    let mut src_16x16: __m256i;
    let mut res0_4x64: __m256i;
    let mut res1_4x64: __m256i;
    let mut sub_result: __m256i;
    let zeros: __m256i = _mm256_broadcastsi128_si256(_mm_setzero_si128());
    let mut square_result: __m256i = _mm256_broadcastsi128_si256(_mm_setzero_si128());
    for i in (0..h).step_by(4) {
        dst0_4x8 = _mm_cvtsi32_si128(core::ptr::read_unaligned(
            dst.offset(((i + 0) * dstride) as isize) as *const i32,
        ));
        dst1_4x8 = _mm_cvtsi32_si128(core::ptr::read_unaligned(
            dst.offset(((i + 1) * dstride) as isize) as *const i32,
        ));
        dst2_4x8 = _mm_cvtsi32_si128(core::ptr::read_unaligned(
            dst.offset(((i + 2) * dstride) as isize) as *const i32,
        ));
        dst3_4x8 = _mm_cvtsi32_si128(core::ptr::read_unaligned(
            dst.offset(((i + 3) * dstride) as isize) as *const i32,
        ));
        dst_16x8 = _mm_unpacklo_epi64(
            _mm_unpacklo_epi32(dst0_4x8, dst1_4x8),
            _mm_unpacklo_epi32(dst2_4x8, dst3_4x8),
        );
        dst_16x16 = _mm256_cvtepu8_epi16(dst_16x8);
        src0_4x16 = _mm_loadl_epi64(src.offset(((i + 0) * sstride) as isize) as *const __m128i);
        src1_4x16 = _mm_loadl_epi64(src.offset(((i + 1) * sstride) as isize) as *const __m128i);
        src2_4x16 = _mm_loadl_epi64(src.offset(((i + 2) * sstride) as isize) as *const __m128i);
        src3_4x16 = _mm_loadl_epi64(src.offset(((i + 3) * sstride) as isize) as *const __m128i);
        src0_8x16 = _mm256_castsi128_si256(_mm_unpacklo_epi64(src0_4x16, src1_4x16));
        src1_8x16 = _mm256_castsi128_si256(_mm_unpacklo_epi64(src2_4x16, src3_4x16));
        src_16x16 = _mm256_permute2x128_si256::<0x20>(src0_8x16, src1_8x16);
        // r15 r14 r13------------r1 r0  - 16 bit
        sub_result = _mm256_abs_epi16(_mm256_sub_epi16(src_16x16, dst_16x16));
        // s7 s6 s5 s4 s3 s2 s1 s0 - 32bit
        src_16x16 = _mm256_madd_epi16(sub_result, sub_result);
        // accumulation of result
        square_result = _mm256_add_epi32(square_result, src_16x16);
    }
    // s5 s4 s1 s0  - 64bit
    res0_4x64 = _mm256_unpacklo_epi32(square_result, zeros);
    // s7 s6 s3 s2 - 64bit
    res1_4x64 = _mm256_unpackhi_epi32(square_result, zeros);
    // r3 r2 r1 r0 - 64bit
    res0_4x64 = _mm256_add_epi64(res0_4x64, res1_4x64);
    // r1+r3 r2+r0 - 64bit
    let sum_1x64: __m128i = _mm_add_epi64(
        _mm256_castsi256_si128(res0_4x64),
        _mm256_extracti128_si256::<1>(res0_4x64),
    );
    xx_storel_64(
        &raw mut sum as *mut core::ffi::c_void,
        _mm_add_epi64(sum_1x64, _mm_srli_si128::<8>(sum_1x64)),
    );
    return sum;
}

// Compute mse of four consecutive 4x4 blocks.
// In src buffer, each 4x4 block in a 32x32 filter block is stored sequentially.
// Hence src_blk_stride is same as block width. Whereas dst buffer is a frame
// buffer, thus dstride is a frame level stride.
#[inline]
#[target_feature(enable = "avx2")]
unsafe fn mse_4xh_quad_16bit_avx2(
    dst: *mut u8,
    dstride: i32,
    src: *mut u16,
    src_blk_stride: i32,
    h: i32,
) -> u64 {
    let mut sum: u64 = 0;
    let mut dst0_16x8: __m128i;
    let mut dst1_16x8: __m128i;
    let mut dst2_16x8: __m128i;
    let mut dst3_16x8: __m128i;
    let mut dst0_16x16: __m256i;
    let mut dst1_16x16: __m256i;
    let mut dst2_16x16: __m256i;
    let mut dst3_16x16: __m256i;
    let mut res0_4x64: __m256i;
    let mut res1_4x64: __m256i;
    let mut sub_result_0: __m256i;
    let mut sub_result_1: __m256i;
    let mut sub_result_2: __m256i;
    let mut sub_result_3: __m256i;
    let zeros: __m256i = _mm256_broadcastsi128_si256(_mm_setzero_si128());
    let mut square_result: __m256i = zeros;
    let mut src_temp: *mut u16 = src;
    for i in (0..h).step_by(4) {
        dst0_16x8 = _mm_loadu_si128(
            dst.offset(((i + 0) * dstride) as isize) as *mut __m128i as *const __m128i
        );
        dst1_16x8 = _mm_loadu_si128(
            dst.offset(((i + 1) * dstride) as isize) as *mut __m128i as *const __m128i
        );
        dst2_16x8 = _mm_loadu_si128(
            dst.offset(((i + 2) * dstride) as isize) as *mut __m128i as *const __m128i
        );
        dst3_16x8 = _mm_loadu_si128(
            dst.offset(((i + 3) * dstride) as isize) as *mut __m128i as *const __m128i
        );
        // row0 of 1st,2nd, 3rd and 4th 4x4 blocks- d00 d10 d20 d30
        dst0_16x16 = _mm256_cvtepu8_epi16(dst0_16x8);
        // row1 of 1st,2nd, 3rd and 4th 4x4 blocks - d01 d11 d21 d31
        dst1_16x16 = _mm256_cvtepu8_epi16(dst1_16x8);
        // row2 of 1st,2nd, 3rd and 4th 4x4 blocks - d02 d12 d22 d32
        dst2_16x16 = _mm256_cvtepu8_epi16(dst2_16x8);
        // row3 of 1st,2nd, 3rd and 4th 4x4 blocks - d03 d13 d23 d33
        dst3_16x16 = _mm256_cvtepu8_epi16(dst3_16x8);
        // All rows of 1st 4x4 block - r00 r01 r02 r03
        let mut src0_16x16: __m256i = _mm256_loadu_si256(src_temp as *const __m256i);
        // All rows of 2nd 4x4 block - r10 r11 r12 r13
        let mut src1_16x16: __m256i =
            _mm256_loadu_si256(src_temp.offset(src_blk_stride as isize) as *const __m256i);
        // All rows of 3rd 4x4 block - r20 r21 r22 r23
        let mut src2_16x16: __m256i =
            _mm256_loadu_si256(src_temp.offset((2 * src_blk_stride) as isize) as *const __m256i);
        // All rows of 4th 4x4 block - r30 r31 r32 r33
        let mut src3_16x16: __m256i =
            _mm256_loadu_si256(src_temp.offset((3 * src_blk_stride) as isize) as *const __m256i);
        // r00 r10 r02 r12
        let mut tmp0_16x16: __m256i = _mm256_unpacklo_epi64(src0_16x16, src1_16x16);
        // r01 r11 r03 r13
        let mut tmp1_16x16: __m256i = _mm256_unpackhi_epi64(src0_16x16, src1_16x16);
        // r20 r30 r22 r32
        let mut tmp2_16x16: __m256i = _mm256_unpacklo_epi64(src2_16x16, src3_16x16);
        // r21 r31 r23 r33
        let mut tmp3_16x16: __m256i = _mm256_unpackhi_epi64(src2_16x16, src3_16x16);
        // r00 r10 r20 r30
        src0_16x16 = _mm256_permute2f128_si256::<0x20>(tmp0_16x16, tmp2_16x16);
        // r01 r11 r21 r31
        src1_16x16 = _mm256_permute2f128_si256::<0x20>(tmp1_16x16, tmp3_16x16);
        // r02 r12 r22 r32
        src2_16x16 = _mm256_permute2f128_si256::<0x31>(tmp0_16x16, tmp2_16x16);
        // r03 r13 r23 r33
        src3_16x16 = _mm256_permute2f128_si256::<0x31>(tmp1_16x16, tmp3_16x16);
        // r15 r14 r13------------r1 r0  - 16 bit
        sub_result_0 = _mm256_abs_epi16(_mm256_sub_epi16(src0_16x16, dst0_16x16));
        sub_result_1 = _mm256_abs_epi16(_mm256_sub_epi16(src1_16x16, dst1_16x16));
        sub_result_2 = _mm256_abs_epi16(_mm256_sub_epi16(src2_16x16, dst2_16x16));
        sub_result_3 = _mm256_abs_epi16(_mm256_sub_epi16(src3_16x16, dst3_16x16));
        // s7 s6 s5 s4 s3 s2 s1 s0    - 32bit
        src0_16x16 = _mm256_madd_epi16(sub_result_0, sub_result_0);
        src1_16x16 = _mm256_madd_epi16(sub_result_1, sub_result_1);
        src2_16x16 = _mm256_madd_epi16(sub_result_2, sub_result_2);
        src3_16x16 = _mm256_madd_epi16(sub_result_3, sub_result_3);
        // accumulation of result
        src0_16x16 = _mm256_add_epi32(src0_16x16, src1_16x16);
        src2_16x16 = _mm256_add_epi32(src2_16x16, src3_16x16);
        let square_result_0: __m256i = _mm256_add_epi32(src0_16x16, src2_16x16);
        square_result = _mm256_add_epi32(square_result, square_result_0);
        src_temp = src_temp.offset(16);
    }
    // s5 s4 s1 s0  - 64bit
    res0_4x64 = _mm256_unpacklo_epi32(square_result, zeros);
    // s7  s6  s3  s2 - 64bit
    res1_4x64 = _mm256_unpackhi_epi32(square_result, zeros);
    // r3 r2 r1 r0 - 64bit
    res0_4x64 = _mm256_add_epi64(res0_4x64, res1_4x64);
    // r1+r3 r2+r0 - 64bit
    let sum_1x64: __m128i = _mm_add_epi64(
        _mm256_castsi256_si128(res0_4x64),
        _mm256_extracti128_si256::<1>(res0_4x64),
    );
    xx_storel_64(
        &raw mut sum as *mut core::ffi::c_void,
        _mm_add_epi64(sum_1x64, _mm_srli_si128::<8>(sum_1x64)),
    );
    return sum;
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn mse_8xh_16bit_avx2(
    dst: *mut u8,
    dstride: i32,
    src: *mut u16,
    sstride: i32,
    h: i32,
) -> u64 {
    let mut sum: u64 = 0;
    let mut dst0_8x8: __m128i;
    let mut dst1_8x8: __m128i;
    let mut dst3_16x8: __m128i;
    let mut src0_8x16: __m256i;
    let mut src1_8x16: __m256i;
    let mut src_16x16: __m256i;
    let mut dst_16x16: __m256i;
    let mut res0_4x64: __m256i;
    let mut res1_4x64: __m256i;
    let mut sub_result: __m256i;
    let zeros: __m256i = _mm256_broadcastsi128_si256(_mm_setzero_si128());
    let mut square_result: __m256i = _mm256_broadcastsi128_si256(_mm_setzero_si128());
    for i in (0..h).step_by(2) {
        dst0_8x8 = _mm_loadl_epi64(dst.offset(((i + 0) * dstride) as isize) as *const __m128i);
        dst1_8x8 = _mm_loadl_epi64(dst.offset(((i + 1) * dstride) as isize) as *const __m128i);
        dst3_16x8 = _mm_unpacklo_epi64(dst0_8x8, dst1_8x8);
        dst_16x16 = _mm256_cvtepu8_epi16(dst3_16x8);
        src0_8x16 = _mm256_castsi128_si256(_mm_loadu_si128(src.offset((i * sstride) as isize)
            as *mut __m128i
            as *const __m128i));
        src1_8x16 =
            _mm256_castsi128_si256(_mm_loadu_si128(src.offset(((i + 1) * sstride) as isize)
                as *mut __m128i
                as *const __m128i));
        src_16x16 = _mm256_permute2x128_si256::<0x20>(src0_8x16, src1_8x16);
        // r15 r14 r13 - - - r1 r0 - 16 bit
        sub_result = _mm256_abs_epi16(_mm256_sub_epi16(src_16x16, dst_16x16));
        // s7 s6 s5 s4 s3 s2 s1 s0 - 32bit
        src_16x16 = _mm256_madd_epi16(sub_result, sub_result);
        // accumulation of result
        square_result = _mm256_add_epi32(square_result, src_16x16);
    }
    // s5 s4 s1 s0  - 64bit
    res0_4x64 = _mm256_unpacklo_epi32(square_result, zeros);
    // s7 s6 s3 s2 - 64bit
    res1_4x64 = _mm256_unpackhi_epi32(square_result, zeros);
    // r3 r2 r1 r0 - 64bit
    res0_4x64 = _mm256_add_epi64(res0_4x64, res1_4x64);
    // r1+r3 r2+r0 - 64bit
    let sum_1x64: __m128i = _mm_add_epi64(
        _mm256_castsi256_si128(res0_4x64),
        _mm256_extracti128_si256::<1>(res0_4x64),
    );
    xx_storel_64(
        &raw mut sum as *mut core::ffi::c_void,
        _mm_add_epi64(sum_1x64, _mm_srli_si128::<8>(sum_1x64)),
    );
    return sum;
}

// Compute mse of two consecutive 8x8 blocks.
// In src buffer, each 8x8 block in a 64x64 filter block is stored sequentially.
// Hence src_blk_stride is same as block width. Whereas dst buffer is a frame
// buffer, thus dstride is a frame level stride.
#[inline]
#[target_feature(enable = "avx2")]
unsafe fn mse_8xh_dual_16bit_avx2(
    dst: *mut u8,
    dstride: i32,
    src: *mut u16,
    src_blk_stride: i32,
    h: i32,
) -> u64 {
    let mut sum: u64 = 0;
    let mut dst0_16x8: __m128i;
    let mut dst1_16x8: __m128i;
    let mut dst0_16x16: __m256i;
    let mut dst1_16x16: __m256i;
    let mut res0_4x64: __m256i;
    let mut res1_4x64: __m256i;
    let mut sub_result_0: __m256i;
    let mut sub_result_1: __m256i;
    let zeros: __m256i = _mm256_broadcastsi128_si256(_mm_setzero_si128());
    let mut square_result: __m256i = zeros;
    let mut src_temp: *mut u16 = src;
    for i in (0..h).step_by(2) {
        dst0_16x8 = _mm_loadu_si128(
            dst.offset(((i + 0) * dstride) as isize) as *mut __m128i as *const __m128i
        );
        dst1_16x8 = _mm_loadu_si128(
            dst.offset(((i + 1) * dstride) as isize) as *mut __m128i as *const __m128i
        );
        // row0 of 1st and 2nd 8x8 block - d00 d10
        dst0_16x16 = _mm256_cvtepu8_epi16(dst0_16x8);
        // row1 of 1st and 2nd 8x8 block - d01 d11
        dst1_16x16 = _mm256_cvtepu8_epi16(dst1_16x8);
        // 2 rows of 1st 8x8 block - r00 r01
        let mut src0_16x16: __m256i = _mm256_loadu_si256(src_temp as *const __m256i);
        // 2 rows of 2nd 8x8 block - r10 r11
        let mut src1_16x16: __m256i =
            _mm256_loadu_si256(src_temp.offset(src_blk_stride as isize) as *const __m256i);
        // r00 r10 - 128bit
        let mut tmp0_16x16: __m256i = _mm256_permute2f128_si256::<0x20>(src0_16x16, src1_16x16);
        // r01 r11 - 128bit
        let mut tmp1_16x16: __m256i = _mm256_permute2f128_si256::<0x31>(src0_16x16, src1_16x16);
        // r15 r14 r13------------r1 r0 - 16 bit
        sub_result_0 = _mm256_abs_epi16(_mm256_sub_epi16(tmp0_16x16, dst0_16x16));
        sub_result_1 = _mm256_abs_epi16(_mm256_sub_epi16(tmp1_16x16, dst1_16x16));
        // s7 s6 s5 s4 s3 s2 s1 s0 - 32bit each
        src0_16x16 = _mm256_madd_epi16(sub_result_0, sub_result_0);
        src1_16x16 = _mm256_madd_epi16(sub_result_1, sub_result_1);
        // accumulation of result
        src0_16x16 = _mm256_add_epi32(src0_16x16, src1_16x16);
        square_result = _mm256_add_epi32(square_result, src0_16x16);
        src_temp = src_temp.offset(16);
    }
    // s5 s4 s1 s0  - 64bit
    res0_4x64 = _mm256_unpacklo_epi32(square_result, zeros);
    // s7 s6 s3 s2 - 64bit
    res1_4x64 = _mm256_unpackhi_epi32(square_result, zeros);
    // r3 r2 r1 r0 - 64bit
    res0_4x64 = _mm256_add_epi64(res0_4x64, res1_4x64);
    // r1+r3 r2+r0 - 64bit
    let sum_1x64: __m128i = _mm_add_epi64(
        _mm256_castsi256_si128(res0_4x64),
        _mm256_extracti128_si256::<1>(res0_4x64),
    );
    xx_storel_64(
        &raw mut sum as *mut core::ffi::c_void,
        _mm_add_epi64(sum_1x64, _mm_srli_si128::<8>(sum_1x64)),
    );
    return sum;
}

#[no_mangle]
#[target_feature(enable = "avx2")]
pub unsafe extern "C-unwind" fn aom_mse_wxh_16bit_avx2(
    dst: *mut u8,
    dstride: i32,
    src: *mut u16,
    sstride: i32,
    w: i32,
    h: i32,
) -> u64 {
    debug_assert!(
        (w == 8 || w == 4) && (h == 8 || h == 4),
        "w=8/4 and h=8/4 must be satisfied"
    );
    match w {
        4 => {
            return mse_4xh_16bit_avx2(dst, dstride, src, sstride, h);
        }
        8 => {
            return mse_8xh_16bit_avx2(dst, dstride, src, sstride, h);
        }
        _ => {
            debug_assert!(0 != 0, "unsupported width");
            return (-1_i32) as u64;
        }
    }
}

// Computes mse of two 8x8 or four 4x4 consecutive blocks. Luma plane uses 8x8
// block and Chroma uses 4x4 block. In src buffer, each block in a filter block
// is stored sequentially. Hence src_blk_stride is same as block width. Whereas
// dst buffer is a frame buffer, thus dstride is a frame level stride.
#[no_mangle]
#[target_feature(enable = "avx2")]
pub unsafe extern "C-unwind" fn aom_mse_16xh_16bit_avx2(
    dst: *mut u8,
    dstride: i32,
    src: *mut u16,
    w: i32,
    h: i32,
) -> u64 {
    debug_assert!(
        (w == 8 || w == 4) && (h == 8 || h == 4),
        "w=8/4 and h=8/4 must be satisfied"
    );
    match w {
        4 => {
            return mse_4xh_quad_16bit_avx2(dst, dstride, src, w * h, h);
        }
        8 => {
            return mse_8xh_dual_16bit_avx2(dst, dstride, src, w * h, h);
        }
        _ => {
            debug_assert!(0 != 0, "unsupported width");
            return (-1_i32) as u64;
        }
    }
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn calc_sum_sse_wd32_avx2(
    src: *const u8,
    ref_: *const u8,
    set_one_minusone: __m256i,
    sse_8x16: *mut __m256i,
    sum_8x16: *mut __m256i,
) {
    let s00_256: __m256i = _mm256_loadu_si256(src as *const __m256i);
    let r00_256: __m256i = _mm256_loadu_si256(ref_ as *const __m256i);
    let u_low_256: __m256i = _mm256_unpacklo_epi8(s00_256, r00_256);
    let u_high_256: __m256i = _mm256_unpackhi_epi8(s00_256, r00_256);
    let diff0: __m256i = _mm256_maddubs_epi16(u_low_256, set_one_minusone);
    let diff1: __m256i = _mm256_maddubs_epi16(u_high_256, set_one_minusone);
    *sse_8x16 = _mm256_add_epi32(*sse_8x16, _mm256_madd_epi16(diff0, diff0));
    *sse_8x16.offset(1) = _mm256_add_epi32(*sse_8x16.offset(1), _mm256_madd_epi16(diff1, diff1));
    *sum_8x16 = _mm256_add_epi16(*sum_8x16, diff0);
    *sum_8x16.offset(1) = _mm256_add_epi16(*sum_8x16.offset(1), diff1);
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn calc_sum_sse_order(
    sse_hx16: *mut __m256i,
    sum_hx16: *mut __m256i,
    tot_sse: *mut u32,
    tot_sum: *mut i32,
) -> __m256i {
    // s00 s01 s10 s11 s20 s21 s30 s31
    let sse_results: __m256i = _mm256_hadd_epi32(*sse_hx16, *sse_hx16.offset(1));
    // d00 d01 d02 d03 | d10 d11 d12 d13 | d20 d21 d22 d23 | d30 d31 d32 d33
    let sum_result_r0: __m256i = _mm256_hadd_epi16(*sum_hx16, *sum_hx16.offset(1));
    // d00 d01 d10 d11 | d00 d02 d10 d11 | d20 d21 d30 d31 | d20 d21 d30 d31
    let sum_result_1: __m256i = _mm256_hadd_epi16(sum_result_r0, sum_result_r0);
    // d00 d01 d10 d11 d20 d21 d30 d31 | X
    let sum_result_3: __m256i = _mm256_permute4x64_epi64::<0x08>(sum_result_1);
    // d00 d01 d10 d11 d20 d21 d30 d31
    let sum_results: __m256i = _mm256_cvtepi16_epi32(_mm256_castsi256_si128(sum_result_3));
    // Add sum & sse registers appropriately to get total sum & sse separately.
    // s0 s1 d0 d1 s2 s3 d2 d3
    let sum_sse_add: __m256i = _mm256_hadd_epi32(sse_results, sum_results);
    // s0 s1 s2 s3 d0 d1 d2 d3
    let sum_sse_order_add: __m256i = _mm256_permute4x64_epi64::<0xd8>(sum_sse_add);
    // s0+s1 s2+s3 s0+s1 s2+s3 d0+d1 d2+d3 d0+d1 d2+d3
    let sum_sse_order_add_1: __m256i = _mm256_hadd_epi32(sum_sse_order_add, sum_sse_order_add);
    // s0 x x x | d0 x x x
    let sum_sse_order_add_final: __m256i =
        _mm256_hadd_epi32(sum_sse_order_add_1, sum_sse_order_add_1);
    // s0
    let first_value: u32 = _mm256_extract_epi32::<0>(sum_sse_order_add_final) as u32;
    *tot_sse += first_value;
    // d0
    let second_value: i32 = _mm256_extract_epi32::<4>(sum_sse_order_add_final);
    *tot_sum += second_value;
    return sum_sse_order_add;
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn get_var_sse_sum_8x8_quad_avx2(
    mut src: *const u8,
    src_stride: i32,
    mut ref_: *const u8,
    ref_stride: i32,
    h: i32,
    sse8x8: *mut u32,
    sum8x8: *mut i32,
    tot_sse: *mut u32,
    tot_sum: *mut i32,
    var8x8: *mut u32,
) {
    debug_assert!(h <= 128); // May overflow for larger height.
    let mut sse_8x16: [__m256i; 2] = core::mem::zeroed();
    let mut sum_8x16: [__m256i; 2] = core::mem::zeroed();
    sum_8x16[0] = _mm256_setzero_si256();
    sse_8x16[0] = _mm256_setzero_si256();
    sum_8x16[1] = sum_8x16[0];
    sse_8x16[1] = sse_8x16[0];
    let set_one_minusone: __m256i = _mm256_set1_epi16(0xff01_i32 as i16);
    for i in 0..h {
        // Process 8x32 block of one row.
        calc_sum_sse_wd32_avx2(
            src,
            ref_,
            set_one_minusone,
            sse_8x16.as_mut_ptr(),
            sum_8x16.as_mut_ptr(),
        );
        src = src.offset(src_stride as isize);
        ref_ = ref_.offset(ref_stride as isize);
    }
    let sum_sse_order_add: __m256i = calc_sum_sse_order(
        sse_8x16.as_mut_ptr(),
        sum_8x16.as_mut_ptr(),
        tot_sse,
        tot_sum,
    );
    // s0 s1 s2 s3
    _mm_storeu_si128(
        sse8x8 as *mut __m128i,
        _mm256_castsi256_si128(sum_sse_order_add),
    );
    // d0 d1 d2 d3
    let sum_temp8x8: __m128i = _mm256_extractf128_si256::<1>(sum_sse_order_add);
    _mm_storeu_si128(sum8x8 as *mut __m128i, sum_temp8x8);
    // (d0xd0 >> 6)=f0 (d1xd1 >> 6)=f1 (d2xd2 >> 6)=f2 (d3xd3 >> 6)=f3
    let mull_results: __m128i = _mm_srli_epi32::<{ 6 }>(_mm_mullo_epi32(sum_temp8x8, sum_temp8x8));
    // s0-f0=v0 s1-f1=v1 s2-f2=v2 s3-f3=v3
    let variance_8x8: __m128i =
        _mm_sub_epi32(_mm256_castsi256_si128(sum_sse_order_add), mull_results);
    // v0 v1 v2 v3
    _mm_storeu_si128(var8x8 as *mut __m128i, variance_8x8);
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn get_var_sse_sum_16x16_dual_avx2(
    mut src: *const u8,
    src_stride: i32,
    mut ref_: *const u8,
    ref_stride: i32,
    h: i32,
    sse16x16: *mut u32,
    tot_sse: *mut u32,
    tot_sum: *mut i32,
    var16x16: *mut u32,
) {
    debug_assert!(h <= 128); // May overflow for larger height.
    let mut sse_16x16: [__m256i; 2] = core::mem::zeroed();
    let mut sum_16x16: [__m256i; 2] = core::mem::zeroed();
    sum_16x16[0] = _mm256_setzero_si256();
    sse_16x16[0] = _mm256_setzero_si256();
    sum_16x16[1] = sum_16x16[0];
    sse_16x16[1] = sse_16x16[0];
    let set_one_minusone: __m256i = _mm256_set1_epi16(0xff01_i32 as i16);
    for i in 0..h {
        // Process 16x32 block of one row.
        calc_sum_sse_wd32_avx2(
            src,
            ref_,
            set_one_minusone,
            sse_16x16.as_mut_ptr(),
            sum_16x16.as_mut_ptr(),
        );
        src = src.offset(src_stride as isize);
        ref_ = ref_.offset(ref_stride as isize);
    }
    let sum_sse_order_add: __m256i = calc_sum_sse_order(
        sse_16x16.as_mut_ptr(),
        sum_16x16.as_mut_ptr(),
        tot_sse,
        tot_sum,
    );
    let sum_sse_order_add_1: __m256i = _mm256_hadd_epi32(sum_sse_order_add, sum_sse_order_add);
    // s0+s1 s2+s3 x x
    _mm_storel_epi64(
        sse16x16 as *mut __m128i,
        _mm256_castsi256_si128(sum_sse_order_add_1),
    );
    // d0+d1 d2+d3 x x
    let sum_temp16x16: __m128i = _mm256_extractf128_si256::<1>(sum_sse_order_add_1);
    // (d0xd0 >> 6)=f0 (d1xd1 >> 6)=f1 (d2xd2 >> 6)=f2 (d3xd3 >> 6)=f3
    let mull_results: __m128i =
        _mm_srli_epi32::<{ 8 }>(_mm_mullo_epi32(sum_temp16x16, sum_temp16x16));
    // s0-f0=v0 s1-f1=v1 s2-f2=v2 s3-f3=v3
    let variance_16x16: __m128i =
        _mm_sub_epi32(_mm256_castsi256_si128(sum_sse_order_add_1), mull_results);
    // v0 v1 v2 v3
    _mm_storel_epi64(var16x16 as *mut __m128i, variance_16x16);
}

// Temporary buffer to store horizontal filter results H[y][x]
// Step 1: Compute Horizontal 1D Filter H[y][x] = P(y, x-1) + 2*P(y, x) + P(y,
// x + 1)
// bw == 4
// Step 2: Compute Vertical Filter V[y][x] = H(y-1, x) + 2*H(y, x) + H(y + 1,
// x), smooth = V >> 4, diff = P - smooth, and accum (diff^2)
// bw == 4
#[no_mangle]
#[target_feature(enable = "avx2")]
pub unsafe extern "C-unwind" fn aom_get_var_sse_sum_8x8_quad_avx2(
    src_ptr: *const u8,
    source_stride: i32,
    ref_ptr: *const u8,
    ref_stride: i32,
    sse8x8: *mut u32,
    sum8x8: *mut i32,
    tot_sse: *mut u32,
    tot_sum: *mut i32,
    var8x8: *mut u32,
) {
    get_var_sse_sum_8x8_quad_avx2(
        src_ptr,
        source_stride,
        ref_ptr,
        ref_stride,
        8,
        sse8x8,
        sum8x8,
        tot_sse,
        tot_sum,
        var8x8,
    );
}

#[no_mangle]
#[target_feature(enable = "avx2")]
pub unsafe extern "C-unwind" fn aom_get_var_sse_sum_16x16_dual_avx2(
    src_ptr: *const u8,
    source_stride: i32,
    ref_ptr: *const u8,
    ref_stride: i32,
    sse16x16: *mut u32,
    tot_sse: *mut u32,
    tot_sum: *mut i32,
    var16x16: *mut u32,
) {
    get_var_sse_sum_16x16_dual_avx2(
        src_ptr,
        source_stride,
        ref_ptr,
        ref_stride,
        16,
        sse16x16,
        tot_sse,
        tot_sum,
        var16x16,
    );
}
