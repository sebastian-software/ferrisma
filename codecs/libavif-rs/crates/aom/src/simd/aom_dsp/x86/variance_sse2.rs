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

// Translated from upstream/aom/aom_dsp/x86/variance_sse2.c (ADR 0056, ADR 0058).
// Upstream compiles it with -msse2. The functions were
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

use crate::simd::aom_dsp__x86__mem_sse2::loadu_int32;
use crate::simd::aom_dsp__x86__synonyms::*;
use core::arch::x86_64::*;

#[inline]
#[target_feature(enable = "sse2")]
unsafe fn load4x2_sse2(p: *const u8, stride: i32) -> __m128i {
    let p0: __m128i = _mm_cvtsi32_si128(loadu_int32(
        p.offset((0 * stride) as isize) as *const core::ffi::c_void
    ));
    let p1: __m128i = _mm_cvtsi32_si128(loadu_int32(
        p.offset((1 * stride) as isize) as *const core::ffi::c_void
    ));
    return _mm_unpacklo_epi8(_mm_unpacklo_epi32(p0, p1), _mm_setzero_si128());
}

#[inline]
#[target_feature(enable = "sse2")]
unsafe fn load8_8to16_sse2(p: *const u8) -> __m128i {
    let p0: __m128i = _mm_loadl_epi64(p as *const __m128i);
    return _mm_unpacklo_epi8(p0, _mm_setzero_si128());
}

#[inline]
#[target_feature(enable = "sse2")]
unsafe fn load16_8to16_sse2(p: *const u8, out: *mut __m128i) {
    let p0: __m128i = _mm_loadu_si128(p as *const __m128i);
    *out = _mm_unpacklo_epi8(p0, _mm_setzero_si128()); // lower 8 values
    *out.offset(1) = _mm_unpackhi_epi8(p0, _mm_setzero_si128());
}

// upper 8 values
// Accumulate 4 32bit numbers in val to 1 32bit number
#[inline]
#[target_feature(enable = "sse2")]
unsafe fn add32x4_sse2(mut val: __m128i) -> u32 {
    val = _mm_add_epi32(val, _mm_srli_si128::<8>(val));
    val = _mm_add_epi32(val, _mm_srli_si128::<4>(val));
    return _mm_cvtsi128_si32(val) as u32;
}

// Accumulate 8 16bit in sum to 4 32bit number
#[inline]
#[target_feature(enable = "sse2")]
unsafe fn sum_to_32bit_sse2(sum: __m128i) -> __m128i {
    let sum_lo: __m128i = _mm_srai_epi32::<{ 16 }>(_mm_unpacklo_epi16(sum, sum));
    let sum_hi: __m128i = _mm_srai_epi32::<{ 16 }>(_mm_unpackhi_epi16(sum, sum));
    return _mm_add_epi32(sum_lo, sum_hi);
}

#[inline]
#[target_feature(enable = "sse2")]
unsafe fn variance_kernel_sse2(src: __m128i, ref_: __m128i, sse: *mut __m128i, sum: *mut __m128i) {
    let diff: __m128i = _mm_sub_epi16(src, ref_);
    *sse = _mm_add_epi32(*sse, _mm_madd_epi16(diff, diff));
    *sum = _mm_add_epi16(*sum, diff);
}

// Can handle 128 pixels' diff sum (such as 8x16 or 16x8)
// Slightly faster than variance_final_256_pel_sse2()
// diff sum of 128 pixels can still fit in 16bit integer
#[inline]
#[target_feature(enable = "sse2")]
unsafe fn variance_final_128_pel_sse2(
    vsse: __m128i,
    mut vsum: __m128i,
    sse: *mut u32,
    sum: *mut i32,
) {
    *sse = add32x4_sse2(vsse);
    vsum = _mm_add_epi16(vsum, _mm_srli_si128::<8>(vsum));
    vsum = _mm_add_epi16(vsum, _mm_srli_si128::<4>(vsum));
    vsum = _mm_add_epi16(vsum, _mm_srli_si128::<2>(vsum));
    *sum = _mm_extract_epi16::<0>(vsum) as i16 as i32;
}

// Can handle 256 pixels' diff sum (such as 16x16)
#[inline]
#[target_feature(enable = "sse2")]
unsafe fn variance_final_256_pel_sse2(
    vsse: __m128i,
    mut vsum: __m128i,
    sse: *mut u32,
    sum: *mut i32,
) {
    *sse = add32x4_sse2(vsse);
    vsum = _mm_add_epi16(vsum, _mm_srli_si128::<8>(vsum));
    vsum = _mm_add_epi16(vsum, _mm_srli_si128::<4>(vsum));
    *sum = _mm_extract_epi16::<0>(vsum) as i16 as i32;
    *sum += _mm_extract_epi16::<1>(vsum) as i16 as i32;
}

// Can handle 512 pixels' diff sum (such as 16x32 or 32x16)
#[inline]
#[target_feature(enable = "sse2")]
unsafe fn variance_final_512_pel_sse2(
    vsse: __m128i,
    mut vsum: __m128i,
    sse: *mut u32,
    sum: *mut i32,
) {
    *sse = add32x4_sse2(vsse);
    vsum = _mm_add_epi16(vsum, _mm_srli_si128::<8>(vsum));
    vsum = _mm_unpacklo_epi16(vsum, vsum);
    vsum = _mm_srai_epi32::<{ 16 }>(vsum);
    *sum = add32x4_sse2(vsum) as i32;
}

// Can handle 1024 pixels' diff sum (such as 32x32)
#[inline]
#[target_feature(enable = "sse2")]
unsafe fn variance_final_1024_pel_sse2(
    vsse: __m128i,
    mut vsum: __m128i,
    sse: *mut u32,
    sum: *mut i32,
) {
    *sse = add32x4_sse2(vsse);
    vsum = sum_to_32bit_sse2(vsum);
    *sum = add32x4_sse2(vsum) as i32;
}

#[inline]
#[target_feature(enable = "sse2")]
unsafe fn variance4_sse2(
    mut src: *const u8,
    src_stride: i32,
    mut ref_: *const u8,
    ref_stride: i32,
    h: i32,
    sse: *mut __m128i,
    sum: *mut __m128i,
) {
    debug_assert!(h <= 256); // May overflow for larger height.
    *sum = _mm_setzero_si128();
    for i in (0..h).step_by(2) {
        let s: __m128i = load4x2_sse2(src, src_stride);
        let r: __m128i = load4x2_sse2(ref_, ref_stride);
        variance_kernel_sse2(s, r, sse, sum);
        src = src.offset((2 * src_stride) as isize);
        ref_ = ref_.offset((2 * ref_stride) as isize);
    }
}

#[inline]
#[target_feature(enable = "sse2")]
unsafe fn variance8_sse2(
    mut src: *const u8,
    src_stride: i32,
    mut ref_: *const u8,
    ref_stride: i32,
    h: i32,
    sse: *mut __m128i,
    sum: *mut __m128i,
) {
    debug_assert!(h <= 128); // May overflow for larger height.
    *sum = _mm_setzero_si128();
    *sse = _mm_setzero_si128();
    for i in 0..h {
        let s: __m128i = load8_8to16_sse2(src);
        let r: __m128i = load8_8to16_sse2(ref_);
        variance_kernel_sse2(s, r, sse, sum);
        src = src.offset(src_stride as isize);
        ref_ = ref_.offset(ref_stride as isize);
    }
}

#[inline]
#[target_feature(enable = "sse2")]
unsafe fn variance16_kernel_sse2(
    src: *const u8,
    ref_: *const u8,
    sse: *mut __m128i,
    sum: *mut __m128i,
) {
    let zero: __m128i = _mm_setzero_si128();
    let s: __m128i = _mm_loadu_si128(src as *const __m128i);
    let r: __m128i = _mm_loadu_si128(ref_ as *const __m128i);
    let src0: __m128i = _mm_unpacklo_epi8(s, zero);
    let ref0: __m128i = _mm_unpacklo_epi8(r, zero);
    let src1: __m128i = _mm_unpackhi_epi8(s, zero);
    let ref1: __m128i = _mm_unpackhi_epi8(r, zero);
    variance_kernel_sse2(src0, ref0, sse, sum);
    variance_kernel_sse2(src1, ref1, sse, sum);
}

#[inline]
#[target_feature(enable = "sse2")]
unsafe fn variance16_sse2(
    mut src: *const u8,
    src_stride: i32,
    mut ref_: *const u8,
    ref_stride: i32,
    h: i32,
    sse: *mut __m128i,
    sum: *mut __m128i,
) {
    debug_assert!(h <= 64); // May overflow for larger height.
    *sum = _mm_setzero_si128();
    for i in 0..h {
        variance16_kernel_sse2(src, ref_, sse, sum);
        src = src.offset(src_stride as isize);
        ref_ = ref_.offset(ref_stride as isize);
    }
}

#[inline]
#[target_feature(enable = "sse2")]
unsafe fn variance32_sse2(
    mut src: *const u8,
    src_stride: i32,
    mut ref_: *const u8,
    ref_stride: i32,
    h: i32,
    sse: *mut __m128i,
    sum: *mut __m128i,
) {
    debug_assert!(h <= 32); // May overflow for larger height.
                            // Don't initialize sse here since it's an accumulation.
    *sum = _mm_setzero_si128();
    for i in 0..h {
        variance16_kernel_sse2(src, ref_, sse, sum);
        variance16_kernel_sse2(src.offset(16), ref_.offset(16), sse, sum);
        src = src.offset(src_stride as isize);
        ref_ = ref_.offset(ref_stride as isize);
    }
}

#[inline]
#[target_feature(enable = "sse2")]
unsafe fn variance64_sse2(
    mut src: *const u8,
    src_stride: i32,
    mut ref_: *const u8,
    ref_stride: i32,
    h: i32,
    sse: *mut __m128i,
    sum: *mut __m128i,
) {
    debug_assert!(h <= 16); // May overflow for larger height.
    *sum = _mm_setzero_si128();
    for i in 0..h {
        variance16_kernel_sse2(src, ref_, sse, sum);
        variance16_kernel_sse2(src.offset(16), ref_.offset(16), sse, sum);
        variance16_kernel_sse2(src.offset(32), ref_.offset(32), sse, sum);
        variance16_kernel_sse2(src.offset(48), ref_.offset(48), sse, sum);
        src = src.offset(src_stride as isize);
        ref_ = ref_.offset(ref_stride as isize);
    }
}

#[inline]
#[target_feature(enable = "sse2")]
unsafe fn variance128_sse2(
    mut src: *const u8,
    src_stride: i32,
    mut ref_: *const u8,
    ref_stride: i32,
    h: i32,
    sse: *mut __m128i,
    sum: *mut __m128i,
) {
    debug_assert!(h <= 8); // May overflow for larger height.
    *sum = _mm_setzero_si128();
    for i in 0..h {
        for j in 0..4 {
            let offset0: i32 = j << 5;
            let offset1: i32 = offset0 + 16;
            variance16_kernel_sse2(
                src.offset(offset0 as isize),
                ref_.offset(offset0 as isize),
                sse,
                sum,
            );
            variance16_kernel_sse2(
                src.offset(offset1 as isize),
                ref_.offset(offset1 as isize),
                sse,
                sum,
            );
        }
        src = src.offset(src_stride as isize);
        ref_ = ref_.offset(ref_stride as isize);
    }
}

#[no_mangle]
#[target_feature(enable = "sse2")]
pub unsafe extern "C-unwind" fn aom_get_var_sse_sum_8x8_quad_sse2(
    src_ptr: *const u8,
    src_stride: i32,
    ref_ptr: *const u8,
    ref_stride: i32,
    sse8x8: *mut u32,
    sum8x8: *mut i32,
    tot_sse: *mut u32,
    tot_sum: *mut i32,
    var8x8: *mut u32,
) {
    // Loop over 4 8x8 blocks. Process one 8x32 block.
    for k in 0..4 {
        let mut src: *const u8 = src_ptr;
        let mut ref_: *const u8 = ref_ptr;
        let mut vsum: __m128i = _mm_setzero_si128();
        let mut vsse: __m128i = _mm_setzero_si128();
        for i in 0..8 {
            let s: __m128i = load8_8to16_sse2(src.offset((k * 8) as isize));
            let r: __m128i = load8_8to16_sse2(ref_.offset((k * 8) as isize));
            let diff: __m128i = _mm_sub_epi16(s, r);
            vsse = _mm_add_epi32(vsse, _mm_madd_epi16(diff, diff));
            vsum = _mm_add_epi16(vsum, diff);
            src = src.offset(src_stride as isize);
            ref_ = ref_.offset(ref_stride as isize);
        }
        variance_final_128_pel_sse2(
            vsse,
            vsum,
            sse8x8.offset(k as isize),
            sum8x8.offset(k as isize),
        );
    }
    // Calculate variance at 8x8 level and total sse, sum of 8x32 block.
    *tot_sse += *sse8x8 + *sse8x8.offset(1) + *sse8x8.offset(2) + *sse8x8.offset(3);
    *tot_sum += *sum8x8 + *sum8x8.offset(1) + *sum8x8.offset(2) + *sum8x8.offset(3);
    for i in 0..4 {
        *var8x8.offset(i as isize) = *sse8x8.offset(i as isize)
            - ((*sum8x8.offset(i as isize) as i64 * *sum8x8.offset(i as isize) as i64) >> 6) as u32;
    }
}

#[no_mangle]
#[target_feature(enable = "sse2")]
pub unsafe extern "C-unwind" fn aom_get_var_sse_sum_16x16_dual_sse2(
    src_ptr: *const u8,
    src_stride: i32,
    ref_ptr: *const u8,
    ref_stride: i32,
    sse16x16: *mut u32,
    tot_sse: *mut u32,
    tot_sum: *mut i32,
    var16x16: *mut u32,
) {
    let mut sum16x16: [i32; 2] = [0, 0];
    // Loop over 2 16x16 blocks. Process one 16x32 block.
    for k in 0..2 {
        let mut src: *const u8 = src_ptr;
        let mut ref_: *const u8 = ref_ptr;
        let mut vsum: __m128i = _mm_setzero_si128();
        let mut vsse: __m128i = _mm_setzero_si128();
        for i in 0..16 {
            let mut s: [__m128i; 2] = core::mem::zeroed();
            let mut r: [__m128i; 2] = core::mem::zeroed();
            load16_8to16_sse2(src.offset((k * 16) as isize), s.as_mut_ptr());
            load16_8to16_sse2(ref_.offset((k * 16) as isize), r.as_mut_ptr());
            let diff0: __m128i = _mm_sub_epi16(s[0], r[0]);
            let diff1: __m128i = _mm_sub_epi16(s[1], r[1]);
            vsse = _mm_add_epi32(vsse, _mm_madd_epi16(diff0, diff0));
            vsse = _mm_add_epi32(vsse, _mm_madd_epi16(diff1, diff1));
            vsum = _mm_add_epi16(vsum, _mm_add_epi16(diff0, diff1));
            src = src.offset(src_stride as isize);
            ref_ = ref_.offset(ref_stride as isize);
        }
        variance_final_256_pel_sse2(
            vsse,
            vsum,
            sse16x16.offset(k as isize),
            sum16x16.as_mut_ptr().offset(k as isize),
        );
    }
    // Calculate variance at 16x16 level and total sse, sum of 16x32 block.
    *tot_sse += *sse16x16 + *sse16x16.offset(1);
    *tot_sum += sum16x16[0] + sum16x16[1];
    for i in 0..2 {
        *var16x16.offset(i as isize) = *sse16x16.offset(i as isize)
            - ((sum16x16[i as usize] as i64 * sum16x16[i as usize] as i64) >> 8) as u32;
    }
}

#[no_mangle]
#[target_feature(enable = "sse2")]
pub unsafe extern "C-unwind" fn aom_variance4x4_sse2(
    src: *const u8,
    src_stride: i32,
    ref_: *const u8,
    ref_stride: i32,
    sse: *mut u32,
) -> u32 {
    let mut vsse: __m128i = _mm_setzero_si128();
    let mut vsum: __m128i = core::mem::zeroed();
    let mut sum: i32 = 0;
    variance4_sse2(
        src,
        src_stride,
        ref_,
        ref_stride,
        4,
        &raw mut vsse,
        &raw mut vsum,
    );
    variance_final_128_pel_sse2(vsse, vsum, sse, &raw mut sum);
    debug_assert!(sum <= 255 * 4 * 4);
    debug_assert!(sum >= -255 * 4 * 4);
    return *sse - ((sum as i64 * sum as i64) >> 4) as u32;
}

#[no_mangle]
#[target_feature(enable = "sse2")]
pub unsafe extern "C-unwind" fn aom_variance4x8_sse2(
    src: *const u8,
    src_stride: i32,
    ref_: *const u8,
    ref_stride: i32,
    sse: *mut u32,
) -> u32 {
    let mut vsse: __m128i = _mm_setzero_si128();
    let mut vsum: __m128i = core::mem::zeroed();
    let mut sum: i32 = 0;
    variance4_sse2(
        src,
        src_stride,
        ref_,
        ref_stride,
        8,
        &raw mut vsse,
        &raw mut vsum,
    );
    variance_final_128_pel_sse2(vsse, vsum, sse, &raw mut sum);
    debug_assert!(sum <= 255 * 4 * 8);
    debug_assert!(sum >= -255 * 4 * 8);
    return *sse - ((sum as i64 * sum as i64) >> 5) as u32;
}

#[no_mangle]
#[target_feature(enable = "sse2")]
pub unsafe extern "C-unwind" fn aom_variance8x4_sse2(
    src: *const u8,
    src_stride: i32,
    ref_: *const u8,
    ref_stride: i32,
    sse: *mut u32,
) -> u32 {
    let mut vsse: __m128i = _mm_setzero_si128();
    let mut vsum: __m128i = core::mem::zeroed();
    let mut sum: i32 = 0;
    variance8_sse2(
        src,
        src_stride,
        ref_,
        ref_stride,
        4,
        &raw mut vsse,
        &raw mut vsum,
    );
    variance_final_128_pel_sse2(vsse, vsum, sse, &raw mut sum);
    debug_assert!(sum <= 255 * 8 * 4);
    debug_assert!(sum >= -255 * 8 * 4);
    return *sse - ((sum as i64 * sum as i64) >> 5) as u32;
}

#[no_mangle]
#[target_feature(enable = "sse2")]
pub unsafe extern "C-unwind" fn aom_variance8x8_sse2(
    src: *const u8,
    src_stride: i32,
    ref_: *const u8,
    ref_stride: i32,
    sse: *mut u32,
) -> u32 {
    let mut vsse: __m128i = _mm_setzero_si128();
    let mut vsum: __m128i = core::mem::zeroed();
    let mut sum: i32 = 0;
    variance8_sse2(
        src,
        src_stride,
        ref_,
        ref_stride,
        8,
        &raw mut vsse,
        &raw mut vsum,
    );
    variance_final_128_pel_sse2(vsse, vsum, sse, &raw mut sum);
    debug_assert!(sum <= 255 * 8 * 8);
    debug_assert!(sum >= -255 * 8 * 8);
    return *sse - ((sum as i64 * sum as i64) >> 6) as u32;
}

#[no_mangle]
#[target_feature(enable = "sse2")]
pub unsafe extern "C-unwind" fn aom_variance8x16_sse2(
    src: *const u8,
    src_stride: i32,
    ref_: *const u8,
    ref_stride: i32,
    sse: *mut u32,
) -> u32 {
    let mut vsse: __m128i = _mm_setzero_si128();
    let mut vsum: __m128i = core::mem::zeroed();
    let mut sum: i32 = 0;
    variance8_sse2(
        src,
        src_stride,
        ref_,
        ref_stride,
        16,
        &raw mut vsse,
        &raw mut vsum,
    );
    variance_final_128_pel_sse2(vsse, vsum, sse, &raw mut sum);
    debug_assert!(sum <= 255 * 8 * 16);
    debug_assert!(sum >= -255 * 8 * 16);
    return *sse - ((sum as i64 * sum as i64) >> 7) as u32;
}

#[no_mangle]
#[target_feature(enable = "sse2")]
pub unsafe extern "C-unwind" fn aom_variance16x8_sse2(
    src: *const u8,
    src_stride: i32,
    ref_: *const u8,
    ref_stride: i32,
    sse: *mut u32,
) -> u32 {
    let mut vsse: __m128i = _mm_setzero_si128();
    let mut vsum: __m128i = core::mem::zeroed();
    let mut sum: i32 = 0;
    variance16_sse2(
        src,
        src_stride,
        ref_,
        ref_stride,
        8,
        &raw mut vsse,
        &raw mut vsum,
    );
    variance_final_128_pel_sse2(vsse, vsum, sse, &raw mut sum);
    debug_assert!(sum <= 255 * 16 * 8);
    debug_assert!(sum >= -255 * 16 * 8);
    return *sse - ((sum as i64 * sum as i64) >> 7) as u32;
}

#[no_mangle]
#[target_feature(enable = "sse2")]
pub unsafe extern "C-unwind" fn aom_variance16x16_sse2(
    src: *const u8,
    src_stride: i32,
    ref_: *const u8,
    ref_stride: i32,
    sse: *mut u32,
) -> u32 {
    let mut vsse: __m128i = _mm_setzero_si128();
    let mut vsum: __m128i = core::mem::zeroed();
    let mut sum: i32 = 0;
    variance16_sse2(
        src,
        src_stride,
        ref_,
        ref_stride,
        16,
        &raw mut vsse,
        &raw mut vsum,
    );
    variance_final_256_pel_sse2(vsse, vsum, sse, &raw mut sum);
    debug_assert!(sum <= 255 * 16 * 16);
    debug_assert!(sum >= -255 * 16 * 16);
    return *sse - ((sum as i64 * sum as i64) >> 8) as u32;
}

#[no_mangle]
#[target_feature(enable = "sse2")]
pub unsafe extern "C-unwind" fn aom_variance16x32_sse2(
    src: *const u8,
    src_stride: i32,
    ref_: *const u8,
    ref_stride: i32,
    sse: *mut u32,
) -> u32 {
    let mut vsse: __m128i = _mm_setzero_si128();
    let mut vsum: __m128i = core::mem::zeroed();
    let mut sum: i32 = 0;
    variance16_sse2(
        src,
        src_stride,
        ref_,
        ref_stride,
        32,
        &raw mut vsse,
        &raw mut vsum,
    );
    variance_final_512_pel_sse2(vsse, vsum, sse, &raw mut sum);
    debug_assert!(sum <= 255 * 16 * 32);
    debug_assert!(sum >= -255 * 16 * 32);
    return *sse - ((sum as i64 * sum as i64) >> 9) as u32;
}

#[no_mangle]
#[target_feature(enable = "sse2")]
pub unsafe extern "C-unwind" fn aom_variance32x16_sse2(
    src: *const u8,
    src_stride: i32,
    ref_: *const u8,
    ref_stride: i32,
    sse: *mut u32,
) -> u32 {
    let mut vsse: __m128i = _mm_setzero_si128();
    let mut vsum: __m128i = core::mem::zeroed();
    let mut sum: i32 = 0;
    variance32_sse2(
        src,
        src_stride,
        ref_,
        ref_stride,
        16,
        &raw mut vsse,
        &raw mut vsum,
    );
    variance_final_512_pel_sse2(vsse, vsum, sse, &raw mut sum);
    debug_assert!(sum <= 255 * 32 * 16);
    debug_assert!(sum >= -255 * 32 * 16);
    return *sse - ((sum as i64 * sum as i64) >> 9) as u32;
}

#[no_mangle]
#[target_feature(enable = "sse2")]
pub unsafe extern "C-unwind" fn aom_variance32x32_sse2(
    src: *const u8,
    src_stride: i32,
    ref_: *const u8,
    ref_stride: i32,
    sse: *mut u32,
) -> u32 {
    let mut vsse: __m128i = _mm_setzero_si128();
    let mut vsum: __m128i = core::mem::zeroed();
    let mut sum: i32 = 0;
    variance32_sse2(
        src,
        src_stride,
        ref_,
        ref_stride,
        32,
        &raw mut vsse,
        &raw mut vsum,
    );
    variance_final_1024_pel_sse2(vsse, vsum, sse, &raw mut sum);
    debug_assert!(sum <= 255 * 32 * 32);
    debug_assert!(sum >= -255 * 32 * 32);
    return *sse - ((sum as i64 * sum as i64) >> 10) as u32;
}

#[no_mangle]
#[target_feature(enable = "sse2")]
pub unsafe extern "C-unwind" fn aom_variance4x16_sse2(
    src: *const u8,
    src_stride: i32,
    ref_: *const u8,
    ref_stride: i32,
    sse: *mut u32,
) -> u32 {
    let mut vsse: __m128i = _mm_setzero_si128();
    let mut vsum: __m128i = core::mem::zeroed();
    let mut sum: i32 = 0;
    variance4_sse2(
        src,
        src_stride,
        ref_,
        ref_stride,
        16,
        &raw mut vsse,
        &raw mut vsum,
    );
    variance_final_128_pel_sse2(vsse, vsum, sse, &raw mut sum);
    debug_assert!(sum <= 255 * 4 * 16);
    debug_assert!(sum >= -255 * 4 * 16);
    return *sse - ((sum as i64 * sum as i64) >> 6) as u32;
}

#[no_mangle]
#[target_feature(enable = "sse2")]
pub unsafe extern "C-unwind" fn aom_variance16x4_sse2(
    src: *const u8,
    src_stride: i32,
    ref_: *const u8,
    ref_stride: i32,
    sse: *mut u32,
) -> u32 {
    let mut vsse: __m128i = _mm_setzero_si128();
    let mut vsum: __m128i = core::mem::zeroed();
    let mut sum: i32 = 0;
    variance16_sse2(
        src,
        src_stride,
        ref_,
        ref_stride,
        4,
        &raw mut vsse,
        &raw mut vsum,
    );
    variance_final_128_pel_sse2(vsse, vsum, sse, &raw mut sum);
    debug_assert!(sum <= 255 * 16 * 4);
    debug_assert!(sum >= -255 * 16 * 4);
    return *sse - ((sum as i64 * sum as i64) >> 6) as u32;
}

#[no_mangle]
#[target_feature(enable = "sse2")]
pub unsafe extern "C-unwind" fn aom_variance8x32_sse2(
    src: *const u8,
    src_stride: i32,
    ref_: *const u8,
    ref_stride: i32,
    sse: *mut u32,
) -> u32 {
    let mut vsse: __m128i = _mm_setzero_si128();
    let mut vsum: __m128i = core::mem::zeroed();
    let mut sum: i32 = 0;
    variance8_sse2(
        src,
        src_stride,
        ref_,
        ref_stride,
        32,
        &raw mut vsse,
        &raw mut vsum,
    );
    variance_final_256_pel_sse2(vsse, vsum, sse, &raw mut sum);
    debug_assert!(sum <= 255 * 8 * 32);
    debug_assert!(sum >= -255 * 8 * 32);
    return *sse - ((sum as i64 * sum as i64) >> 8) as u32;
}

#[no_mangle]
#[target_feature(enable = "sse2")]
pub unsafe extern "C-unwind" fn aom_variance32x8_sse2(
    src: *const u8,
    src_stride: i32,
    ref_: *const u8,
    ref_stride: i32,
    sse: *mut u32,
) -> u32 {
    let mut vsse: __m128i = _mm_setzero_si128();
    let mut vsum: __m128i = core::mem::zeroed();
    let mut sum: i32 = 0;
    variance32_sse2(
        src,
        src_stride,
        ref_,
        ref_stride,
        8,
        &raw mut vsse,
        &raw mut vsum,
    );
    variance_final_256_pel_sse2(vsse, vsum, sse, &raw mut sum);
    debug_assert!(sum <= 255 * 32 * 8);
    debug_assert!(sum >= -255 * 32 * 8);
    return *sse - ((sum as i64 * sum as i64) >> 8) as u32;
}

#[no_mangle]
#[target_feature(enable = "sse2")]
pub unsafe extern "C-unwind" fn aom_variance16x64_sse2(
    src: *const u8,
    src_stride: i32,
    ref_: *const u8,
    ref_stride: i32,
    sse: *mut u32,
) -> u32 {
    let mut vsse: __m128i = _mm_setzero_si128();
    let mut vsum: __m128i = core::mem::zeroed();
    let mut sum: i32 = 0;
    variance16_sse2(
        src,
        src_stride,
        ref_,
        ref_stride,
        64,
        &raw mut vsse,
        &raw mut vsum,
    );
    variance_final_1024_pel_sse2(vsse, vsum, sse, &raw mut sum);
    debug_assert!(sum <= 255 * 16 * 64);
    debug_assert!(sum >= -255 * 16 * 64);
    return *sse - ((sum as i64 * sum as i64) >> 10) as u32;
}

#[no_mangle]
#[target_feature(enable = "sse2")]
pub unsafe extern "C-unwind" fn aom_variance32x64_sse2(
    mut src: *const u8,
    src_stride: i32,
    mut ref_: *const u8,
    ref_stride: i32,
    sse: *mut u32,
) -> u32 {
    let mut vsse: __m128i = _mm_setzero_si128();
    let mut vsum: __m128i = _mm_setzero_si128();
    for i in 0..64 / 32 {
        let mut vsum16: __m128i = core::mem::zeroed();
        variance32_sse2(
            src,
            src_stride,
            ref_,
            ref_stride,
            32,
            &raw mut vsse,
            &raw mut vsum16,
        );
        vsum = _mm_add_epi32(vsum, sum_to_32bit_sse2(vsum16));
        src = src.offset((src_stride * 32) as isize);
        ref_ = ref_.offset((ref_stride * 32) as isize);
    }
    *sse = add32x4_sse2(vsse);
    let mut sum: i32 = add32x4_sse2(vsum) as i32;
    debug_assert!(sum <= 255 * 32 * 64);
    debug_assert!(sum >= -255 * 32 * 64);
    return *sse - ((sum as i64 * sum as i64) >> 11) as u32;
}

// 32x32 * ( 64/32 )
#[no_mangle]
#[target_feature(enable = "sse2")]
pub unsafe extern "C-unwind" fn aom_variance64x32_sse2(
    mut src: *const u8,
    src_stride: i32,
    mut ref_: *const u8,
    ref_stride: i32,
    sse: *mut u32,
) -> u32 {
    let mut vsse: __m128i = _mm_setzero_si128();
    let mut vsum: __m128i = _mm_setzero_si128();
    for i in 0..32 / 16 {
        let mut vsum16: __m128i = core::mem::zeroed();
        variance64_sse2(
            src,
            src_stride,
            ref_,
            ref_stride,
            16,
            &raw mut vsse,
            &raw mut vsum16,
        );
        vsum = _mm_add_epi32(vsum, sum_to_32bit_sse2(vsum16));
        src = src.offset((src_stride * 16) as isize);
        ref_ = ref_.offset((ref_stride * 16) as isize);
    }
    *sse = add32x4_sse2(vsse);
    let mut sum: i32 = add32x4_sse2(vsum) as i32;
    debug_assert!(sum <= 255 * 64 * 32);
    debug_assert!(sum >= -255 * 64 * 32);
    return *sse - ((sum as i64 * sum as i64) >> 11) as u32;
}

// 64x16 * ( 32/16 )
#[no_mangle]
#[target_feature(enable = "sse2")]
pub unsafe extern "C-unwind" fn aom_variance64x64_sse2(
    mut src: *const u8,
    src_stride: i32,
    mut ref_: *const u8,
    ref_stride: i32,
    sse: *mut u32,
) -> u32 {
    let mut vsse: __m128i = _mm_setzero_si128();
    let mut vsum: __m128i = _mm_setzero_si128();
    for i in 0..64 / 16 {
        let mut vsum16: __m128i = core::mem::zeroed();
        variance64_sse2(
            src,
            src_stride,
            ref_,
            ref_stride,
            16,
            &raw mut vsse,
            &raw mut vsum16,
        );
        vsum = _mm_add_epi32(vsum, sum_to_32bit_sse2(vsum16));
        src = src.offset((src_stride * 16) as isize);
        ref_ = ref_.offset((ref_stride * 16) as isize);
    }
    *sse = add32x4_sse2(vsse);
    let mut sum: i32 = add32x4_sse2(vsum) as i32;
    debug_assert!(sum <= 255 * 64 * 64);
    debug_assert!(sum >= -255 * 64 * 64);
    return *sse - ((sum as i64 * sum as i64) >> 12) as u32;
}

// 64x16 * ( 64/16 )
#[no_mangle]
#[target_feature(enable = "sse2")]
pub unsafe extern "C-unwind" fn aom_variance64x128_sse2(
    mut src: *const u8,
    src_stride: i32,
    mut ref_: *const u8,
    ref_stride: i32,
    sse: *mut u32,
) -> u32 {
    let mut vsse: __m128i = _mm_setzero_si128();
    let mut vsum: __m128i = _mm_setzero_si128();
    for i in 0..128 / 16 {
        let mut vsum16: __m128i = core::mem::zeroed();
        variance64_sse2(
            src,
            src_stride,
            ref_,
            ref_stride,
            16,
            &raw mut vsse,
            &raw mut vsum16,
        );
        vsum = _mm_add_epi32(vsum, sum_to_32bit_sse2(vsum16));
        src = src.offset((src_stride * 16) as isize);
        ref_ = ref_.offset((ref_stride * 16) as isize);
    }
    *sse = add32x4_sse2(vsse);
    let mut sum: i32 = add32x4_sse2(vsum) as i32;
    debug_assert!(sum <= 255 * 64 * 128);
    debug_assert!(sum >= -255 * 64 * 128);
    return *sse - ((sum as i64 * sum as i64) >> 13) as u32;
}

// 64x16 * ( 128/16 )
#[no_mangle]
#[target_feature(enable = "sse2")]
pub unsafe extern "C-unwind" fn aom_variance128x64_sse2(
    mut src: *const u8,
    src_stride: i32,
    mut ref_: *const u8,
    ref_stride: i32,
    sse: *mut u32,
) -> u32 {
    let mut vsse: __m128i = _mm_setzero_si128();
    let mut vsum: __m128i = _mm_setzero_si128();
    for i in 0..64 / 8 {
        let mut vsum16: __m128i = core::mem::zeroed();
        variance128_sse2(
            src,
            src_stride,
            ref_,
            ref_stride,
            8,
            &raw mut vsse,
            &raw mut vsum16,
        );
        vsum = _mm_add_epi32(vsum, sum_to_32bit_sse2(vsum16));
        src = src.offset((src_stride * 8) as isize);
        ref_ = ref_.offset((ref_stride * 8) as isize);
    }
    *sse = add32x4_sse2(vsse);
    let mut sum: i32 = add32x4_sse2(vsum) as i32;
    debug_assert!(sum <= 255 * 128 * 64);
    debug_assert!(sum >= -255 * 128 * 64);
    return *sse - ((sum as i64 * sum as i64) >> 13) as u32;
}

// 128x8 * ( 64/8 )
#[no_mangle]
#[target_feature(enable = "sse2")]
pub unsafe extern "C-unwind" fn aom_variance128x128_sse2(
    mut src: *const u8,
    src_stride: i32,
    mut ref_: *const u8,
    ref_stride: i32,
    sse: *mut u32,
) -> u32 {
    let mut vsse: __m128i = _mm_setzero_si128();
    let mut vsum: __m128i = _mm_setzero_si128();
    for i in 0..128 / 8 {
        let mut vsum16: __m128i = core::mem::zeroed();
        variance128_sse2(
            src,
            src_stride,
            ref_,
            ref_stride,
            8,
            &raw mut vsse,
            &raw mut vsum16,
        );
        vsum = _mm_add_epi32(vsum, sum_to_32bit_sse2(vsum16));
        src = src.offset((src_stride * 8) as isize);
        ref_ = ref_.offset((ref_stride * 8) as isize);
    }
    *sse = add32x4_sse2(vsse);
    let mut sum: i32 = add32x4_sse2(vsum) as i32;
    debug_assert!(sum <= 255 * 128 * 128);
    debug_assert!(sum >= -255 * 128 * 128);
    return *sse - ((sum as i64 * sum as i64) >> 14) as u32;
}

// 128x8 * ( 128/8 )
#[no_mangle]
#[target_feature(enable = "sse2")]
pub unsafe extern "C-unwind" fn aom_variance64x16_sse2(
    src: *const u8,
    src_stride: i32,
    ref_: *const u8,
    ref_stride: i32,
    sse: *mut u32,
) -> u32 {
    let mut vsse: __m128i = _mm_setzero_si128();
    let mut vsum: __m128i = core::mem::zeroed();
    let mut sum: i32 = 0;
    variance64_sse2(
        src,
        src_stride,
        ref_,
        ref_stride,
        16,
        &raw mut vsse,
        &raw mut vsum,
    );
    variance_final_1024_pel_sse2(vsse, vsum, sse, &raw mut sum);
    debug_assert!(sum <= 255 * 64 * 16);
    debug_assert!(sum >= -255 * 64 * 16);
    return *sse - ((sum as i64 * sum as i64) >> 10) as u32;
}

#[no_mangle]
#[target_feature(enable = "sse2")]
pub unsafe extern "C-unwind" fn aom_mse8x8_sse2(
    src: *const u8,
    src_stride: i32,
    ref_: *const u8,
    ref_stride: i32,
    sse: *mut u32,
) -> u32 {
    aom_variance8x8_sse2(src, src_stride, ref_, ref_stride, sse);
    return *sse;
}

#[no_mangle]
#[target_feature(enable = "sse2")]
pub unsafe extern "C-unwind" fn aom_mse8x16_sse2(
    src: *const u8,
    src_stride: i32,
    ref_: *const u8,
    ref_stride: i32,
    sse: *mut u32,
) -> u32 {
    aom_variance8x16_sse2(src, src_stride, ref_, ref_stride, sse);
    return *sse;
}

#[no_mangle]
#[target_feature(enable = "sse2")]
pub unsafe extern "C-unwind" fn aom_mse16x8_sse2(
    src: *const u8,
    src_stride: i32,
    ref_: *const u8,
    ref_stride: i32,
    sse: *mut u32,
) -> u32 {
    aom_variance16x8_sse2(src, src_stride, ref_, ref_stride, sse);
    return *sse;
}

#[no_mangle]
#[target_feature(enable = "sse2")]
pub unsafe extern "C-unwind" fn aom_mse16x16_sse2(
    src: *const u8,
    src_stride: i32,
    ref_: *const u8,
    ref_stride: i32,
    sse: *mut u32,
) -> u32 {
    aom_variance16x16_sse2(src, src_stride, ref_, ref_stride, sse);
    return *sse;
}

#[inline]
#[target_feature(enable = "sse2")]
unsafe fn mse_4xh_16bit_sse2(
    dst: *mut u8,
    dstride: i32,
    src: *mut u16,
    sstride: i32,
    h: i32,
) -> u64 {
    let mut sum: u64 = 0;
    let mut dst0_8x8: __m128i;
    let mut dst1_8x8: __m128i;
    let mut dst_16x8: __m128i;
    let mut src0_16x4: __m128i;
    let mut src1_16x4: __m128i;
    let mut src_16x8: __m128i;
    let mut res0_32x4: __m128i;
    let mut res0_64x2: __m128i;
    let mut res1_64x2: __m128i;
    let mut sub_result_16x8: __m128i;
    let zeros: __m128i = _mm_setzero_si128();
    let mut square_result: __m128i = _mm_setzero_si128();
    for i in (0..h).step_by(2) {
        dst0_8x8 = _mm_cvtsi32_si128(core::ptr::read_unaligned(
            dst.offset(((i + 0) * dstride) as isize) as *const i32,
        ));
        dst1_8x8 = _mm_cvtsi32_si128(core::ptr::read_unaligned(
            dst.offset(((i + 1) * dstride) as isize) as *const i32,
        ));
        dst_16x8 = _mm_unpacklo_epi8(_mm_unpacklo_epi32(dst0_8x8, dst1_8x8), zeros);
        src0_16x4 = _mm_loadl_epi64(src.offset(((i + 0) * sstride) as isize) as *const __m128i);
        src1_16x4 = _mm_loadl_epi64(src.offset(((i + 1) * sstride) as isize) as *const __m128i);
        src_16x8 = _mm_unpacklo_epi64(src0_16x4, src1_16x4);
        sub_result_16x8 = _mm_sub_epi16(src_16x8, dst_16x8);
        res0_32x4 = _mm_madd_epi16(sub_result_16x8, sub_result_16x8);
        res0_64x2 = _mm_unpacklo_epi32(res0_32x4, zeros);
        res1_64x2 = _mm_unpackhi_epi32(res0_32x4, zeros);
        square_result = _mm_add_epi64(square_result, _mm_add_epi64(res0_64x2, res1_64x2));
    }
    let sum_64x1: __m128i = _mm_add_epi64(square_result, _mm_srli_si128::<8>(square_result));
    xx_storel_64(&raw mut sum as *mut core::ffi::c_void, sum_64x1);
    return sum;
}

#[inline]
#[target_feature(enable = "sse2")]
unsafe fn mse_8xh_16bit_sse2(
    dst: *mut u8,
    dstride: i32,
    src: *mut u16,
    sstride: i32,
    h: i32,
) -> u64 {
    let mut sum: u64 = 0;
    let mut dst_8x8: __m128i;
    let mut dst_16x8: __m128i;
    let mut src_16x8: __m128i;
    let mut res0_32x4: __m128i;
    let mut res0_64x2: __m128i;
    let mut res1_64x2: __m128i;
    let mut sub_result_16x8: __m128i;
    let zeros: __m128i = _mm_setzero_si128();
    let mut square_result: __m128i = _mm_setzero_si128();
    for i in 0..h {
        dst_8x8 = _mm_loadl_epi64(dst.offset(((i + 0) * dstride) as isize) as *const __m128i);
        dst_16x8 = _mm_unpacklo_epi8(dst_8x8, zeros);
        src_16x8 =
            _mm_loadu_si128(src.offset((i * sstride) as isize) as *mut __m128i as *const __m128i);
        sub_result_16x8 = _mm_sub_epi16(src_16x8, dst_16x8);
        res0_32x4 = _mm_madd_epi16(sub_result_16x8, sub_result_16x8);
        res0_64x2 = _mm_unpacklo_epi32(res0_32x4, zeros);
        res1_64x2 = _mm_unpackhi_epi32(res0_32x4, zeros);
        square_result = _mm_add_epi64(square_result, _mm_add_epi64(res0_64x2, res1_64x2));
    }
    let sum_64x1: __m128i = _mm_add_epi64(square_result, _mm_srli_si128::<8>(square_result));
    xx_storel_64(&raw mut sum as *mut core::ffi::c_void, sum_64x1);
    return sum;
}

#[no_mangle]
#[target_feature(enable = "sse2")]
pub unsafe extern "C-unwind" fn aom_mse_wxh_16bit_sse2(
    dst: *mut u8,
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
            return mse_4xh_16bit_sse2(dst, dstride, src, sstride, h);
        }
        8 => {
            return mse_8xh_16bit_sse2(dst, dstride, src, sstride, h);
        }
        _ => {
            debug_assert!(0 != 0, "unsupported width");
            return (-1_i32) as u64;
        }
    }
}

#[no_mangle]
#[target_feature(enable = "sse2")]
pub unsafe extern "C-unwind" fn aom_mse_16xh_16bit_sse2(
    mut dst: *mut u8,
    dstride: i32,
    mut src: *mut u16,
    w: i32,
    h: i32,
) -> u64 {
    debug_assert!(
        (w == 8 || w == 4) && (h == 8 || h == 4),
        "w=8/4 and h=8/4 must be satisfied"
    );
    let num_blks: i32 = 16 / w;
    let mut sum: u64 = 0;
    for i in 0..num_blks {
        sum += aom_mse_wxh_16bit_sse2(dst, dstride, src, w, w, h);
        dst = dst.offset(w as isize);
        src = src.offset((w * h) as isize);
    }
    return sum;
}
