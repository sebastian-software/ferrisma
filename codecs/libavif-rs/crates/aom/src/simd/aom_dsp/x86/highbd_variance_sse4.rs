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

// Translated from upstream/aom/aom_dsp/x86/highbd_variance_sse4.c (ADR 0056, ADR 0058).
// Upstream compiles it with -msse41. The functions were
// transliterated with a typing-aware C-subset printer (not C2Rust) and
// reviewed against the C function by function: the same sequence of intrinsic
// and helper calls, constants and operators in order (ADR 0058).
// Only the block variances are translated; the sub-pixel kernels are not.
// ROUND_POWER_OF_TWO (aom_ports/mem.h) is written out through the config's `replace`.
//
// Translation notes:
// - Every function carries `sse4.1`, the flag upstream compiles the file with.
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

use core::arch::x86_64::*;

// SSE4.1
#[inline]
#[target_feature(enable = "sse4.1")]
unsafe fn variance4x4_64_sse4_1(
    a8: *const u8,
    a_stride: i32,
    b8: *const u8,
    b_stride: i32,
    sse: *mut u64,
    sum: *mut i64,
) {
    let mut u0: __m128i;
    let mut u1: __m128i;
    let mut u2: __m128i;
    let mut u3: __m128i;
    let mut s0: __m128i;
    let mut s1: __m128i;
    let mut s2: __m128i;
    let mut s3: __m128i;
    let mut t0: __m128i;
    let mut t1: __m128i;
    let mut x0: __m128i;
    let mut y0: __m128i;
    let mut a0: __m128i;
    let mut a1: __m128i;
    let mut a2: __m128i;
    let mut a3: __m128i;
    let mut b0: __m128i;
    let mut b1: __m128i;
    let mut b2: __m128i;
    let mut b3: __m128i;
    let mut k_one_epi16: __m128i = _mm_set1_epi16(1);
    let mut a: *mut u16 = ((a8 as usize) << 1) as *mut u16;
    let mut b: *mut u16 = ((b8 as usize) << 1) as *mut u16;
    a0 = _mm_loadl_epi64(a.offset((0 * a_stride) as isize) as *const __m128i);
    a1 = _mm_loadl_epi64(a.offset((1 * a_stride) as isize) as *const __m128i);
    a2 = _mm_loadl_epi64(a.offset((2 * a_stride) as isize) as *const __m128i);
    a3 = _mm_loadl_epi64(a.offset((3 * a_stride) as isize) as *const __m128i);
    b0 = _mm_loadl_epi64(b.offset((0 * b_stride) as isize) as *const __m128i);
    b1 = _mm_loadl_epi64(b.offset((1 * b_stride) as isize) as *const __m128i);
    b2 = _mm_loadl_epi64(b.offset((2 * b_stride) as isize) as *const __m128i);
    b3 = _mm_loadl_epi64(b.offset((3 * b_stride) as isize) as *const __m128i);
    u0 = _mm_unpacklo_epi16(a0, a1);
    u1 = _mm_unpacklo_epi16(a2, a3);
    u2 = _mm_unpacklo_epi16(b0, b1);
    u3 = _mm_unpacklo_epi16(b2, b3);
    s0 = _mm_sub_epi16(u0, u2);
    s1 = _mm_sub_epi16(u1, u3);
    t0 = _mm_madd_epi16(s0, k_one_epi16);
    t1 = _mm_madd_epi16(s1, k_one_epi16);
    s2 = _mm_hadd_epi32(t0, t1);
    s3 = _mm_hadd_epi32(s2, s2);
    y0 = _mm_hadd_epi32(s3, s3);
    t0 = _mm_madd_epi16(s0, s0);
    t1 = _mm_madd_epi16(s1, s1);
    s2 = _mm_hadd_epi32(t0, t1);
    s3 = _mm_hadd_epi32(s2, s2);
    x0 = _mm_hadd_epi32(s3, s3);
    *sse = _mm_extract_epi32::<0>(x0) as u64;
    *sum = _mm_extract_epi32::<0>(y0) as i64;
}

#[no_mangle]
#[target_feature(enable = "sse4.1")]
pub unsafe extern "C-unwind" fn aom_highbd_8_variance4x4_sse4_1(
    a: *const u8,
    a_stride: i32,
    b: *const u8,
    b_stride: i32,
    sse: *mut u32,
) -> u32 {
    let mut sum: i64 = 0;
    let mut diff: i64 = 0;
    let mut local_sse: u64 = 0;
    variance4x4_64_sse4_1(a, a_stride, b, b_stride, &raw mut local_sse, &raw mut sum);
    *sse = local_sse as u32;
    diff = *sse as i64 - ((sum * sum) >> 4);
    return if diff >= 0 { diff as u32 } else { 0 };
}

#[no_mangle]
#[target_feature(enable = "sse4.1")]
pub unsafe extern "C-unwind" fn aom_highbd_10_variance4x4_sse4_1(
    a: *const u8,
    a_stride: i32,
    b: *const u8,
    b_stride: i32,
    sse: *mut u32,
) -> u32 {
    let mut sum: i64 = 0;
    let mut diff: i64 = 0;
    let mut local_sse: u64 = 0;
    variance4x4_64_sse4_1(a, a_stride, b, b_stride, &raw mut local_sse, &raw mut sum);
    *sse = ((local_sse + ((1 << 4) >> 1) as u64) >> 4) as u32;
    sum = (sum + ((1 << 2) >> 1) as i64) >> 2;
    diff = *sse as i64 - ((sum * sum) >> 4);
    return if diff >= 0 { diff as u32 } else { 0 };
}

#[no_mangle]
#[target_feature(enable = "sse4.1")]
pub unsafe extern "C-unwind" fn aom_highbd_12_variance4x4_sse4_1(
    a: *const u8,
    a_stride: i32,
    b: *const u8,
    b_stride: i32,
    sse: *mut u32,
) -> u32 {
    let mut sum: i64 = 0;
    let mut diff: i64 = 0;
    let mut local_sse: u64 = 0;
    variance4x4_64_sse4_1(a, a_stride, b, b_stride, &raw mut local_sse, &raw mut sum);
    *sse = ((local_sse + ((1 << 8) >> 1) as u64) >> 8) as u32;
    sum = (sum + ((1 << 4) >> 1) as i64) >> 4;
    diff = *sse as i64 - ((sum * sum) >> 4);
    return if diff >= 0 { diff as u32 } else { 0 };
}
