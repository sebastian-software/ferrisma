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

// Translated from upstream/aom/aom_dsp/x86/loopfilter_sse2.c (ADR 0056, ADR 0058).
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

use crate::aom_dsp__aom_dsp_rtcd::{
    aom_lpf_horizontal_14_quad, aom_lpf_horizontal_6_quad, aom_lpf_horizontal_8_quad,
};
use crate::simd::aom_dsp__x86__lpf_common_sse2::*;
use crate::simd::aom_dsp__x86__synonyms::*;
use core::arch::x86_64::*;

#[repr(C, align(16))]
struct Aligned16<T>(T);
#[repr(C, align(32))]
struct Aligned32<T>(T);

// SSE2
#[inline]
#[target_feature(enable = "sse2")]
unsafe fn abs_diff(a: __m128i, b: __m128i) -> __m128i {
    return _mm_or_si128(_mm_subs_epu8(a, b), _mm_subs_epu8(b, a));
}

// this function treats its input as 2 parallel 8x4 matrices, transposes each of
// them to 4x8  independently while flipping the second matrix horizontally.
// Used for 14 taps pq pairs creation
#[inline]
#[target_feature(enable = "sse2")]
unsafe fn transpose_pq_14_sse2(
    x0: *mut __m128i,
    x1: *mut __m128i,
    x2: *mut __m128i,
    x3: *mut __m128i,
    q0p0: *mut __m128i,
    q1p1: *mut __m128i,
    q2p2: *mut __m128i,
    q3p3: *mut __m128i,
    q4p4: *mut __m128i,
    q5p5: *mut __m128i,
    q6p6: *mut __m128i,
    q7p7: *mut __m128i,
) {
    let mut w0: __m128i;
    let mut w1: __m128i;
    let mut ww0: __m128i;
    let mut ww1: __m128i;
    let mut w2: __m128i;
    let mut w3: __m128i;
    let mut ww2: __m128i;
    let mut ww3: __m128i;
    w0 = _mm_unpacklo_epi8(*x0, *x1); // 00 10 01 11 02 12 03 13 04 14 05 15 06 16 07 17
    w1 = _mm_unpacklo_epi8(*x2, *x3); // 20 30 21 31 22 32 23 33 24 34 25 35 26 36 27 37
    w2 = _mm_unpackhi_epi8(*x0, *x1); // 08 18 09 19 010 110 011 111 012 112 013 113 014 114 015 115
    w3 = _mm_unpackhi_epi8(*x2, *x3); // 28 38 29 39 210 310 211 311 212 312 213 313 214 314 215 315
    ww0 = _mm_unpacklo_epi16(w0, w1); // 00 10 20 30 01 11 21 31        02 12 22 32 03 13 23 33
    ww1 = _mm_unpackhi_epi16(w0, w1); // 04 14 24 34 05 15 25 35        06 16 26 36 07 17 27 37
    ww2 = _mm_unpacklo_epi16(w2, w3); // 08 18 28 38 09 19 29 39       010 110 210 310 011 111 211 311
    ww3 = _mm_unpackhi_epi16(w2, w3); // 012 112 212 312 013 113 213 313  014 114 214 314 015 115 215 315
    *q7p7 = _mm_unpacklo_epi32(ww0, _mm_srli_si128::<12>(ww3)); // 00 10 20 30  015 115 215 315  xx xx xx xx xx xx xx xx
    *q6p6 = _mm_unpackhi_epi32(_mm_slli_si128::<4>(ww0), ww3); // 01 11 21 31  014 114 214 314  xx xx xx xxxx xx xx xx
    *q5p5 = _mm_unpackhi_epi32(ww0, _mm_slli_si128::<4>(ww3)); // 02 12 22 32  013 113 213 313  xx xx xx x xx xx xx xxx
    *q4p4 = _mm_unpacklo_epi32(_mm_srli_si128::<12>(ww0), ww3); // 03 13 23 33  012 112 212 312 xx xx xx xx xx xx xx xx
    *q3p3 = _mm_unpacklo_epi32(ww1, _mm_srli_si128::<12>(ww2)); // 04 14 24 34  011 111 211 311 xx xx xx xx xx xx xx xx
    *q2p2 = _mm_unpackhi_epi32(_mm_slli_si128::<4>(ww1), ww2); // 05 15 25 35   010 110 210 310 xx xx xx xx xx xx xx xx
    *q1p1 = _mm_unpackhi_epi32(ww1, _mm_slli_si128::<4>(ww2)); // 06 16 26 36   09 19 29 39     xx xx xx xx xx xx xx xx
    *q0p0 = _mm_unpacklo_epi32(_mm_srli_si128::<12>(ww1), ww2);
}

// 07 17 27 37  08 18 28 38     xx xx xx xx xx xx xx xx
// this function treats its input as 2 parallel 8x4 matrices, transposes each of
// them  independently while flipping the second matrix horizontaly  Used for 14
// taps filter pq pairs inverse
#[inline]
#[target_feature(enable = "sse2")]
unsafe fn transpose_pq_14_inv_sse2(
    x0: *mut __m128i,
    x1: *mut __m128i,
    x2: *mut __m128i,
    x3: *mut __m128i,
    x4: *mut __m128i,
    x5: *mut __m128i,
    x6: *mut __m128i,
    x7: *mut __m128i,
    pq0: *mut __m128i,
    pq1: *mut __m128i,
    pq2: *mut __m128i,
    pq3: *mut __m128i,
) {
    let mut w10: __m128i;
    let mut w11: __m128i;
    let mut w12: __m128i;
    let mut w13: __m128i;
    let mut w0: __m128i;
    let mut w1: __m128i;
    let mut w2: __m128i;
    let mut w3: __m128i;
    let mut w4: __m128i;
    let mut w5: __m128i;
    let mut d0: __m128i;
    let mut d1: __m128i;
    let mut d2: __m128i;
    let mut d3: __m128i;
    w0 = _mm_unpacklo_epi8(*x0, *x1); // p 00 10 01 11 02 12 03 13 04 14 05 15 06 16 07 17
    w1 = _mm_unpacklo_epi8(*x2, *x3); // p 20 30 21 31 22 32 23 33 24 34 25 35 26 36 27 37
    w2 = _mm_unpacklo_epi8(*x4, *x5); // p 40 50 41 51 42 52 43 53 44 54 45 55 46 56 47 57
    w3 = _mm_unpacklo_epi8(*x6, *x7); // p 60 70 61 71 62 72 63 73 64 74 65 75 66 76 67 77
    w4 = _mm_unpacklo_epi16(w0, w1); // 00 10 20 30 01 11 21 31 02 12 22 32 03 13 23 33
    w5 = _mm_unpacklo_epi16(w2, w3); // 40 50 60 70 41 51 61 71 42 52 62 72 43 53 63 73
    d0 = _mm_unpacklo_epi32(w4, w5); // 00 10 20 30 40 50 60 70 01 11 21 31 41 51 61 71
    d2 = _mm_unpackhi_epi32(w4, w5); // 02 12 22 32 42 52 62 72 03 13 23 33 43 53 63 73
    w10 = _mm_unpacklo_epi8(*x7, *x6); // q xx xx xx xx xx xx xx xx 00 10 01 11 02 12 03 13
    w11 = _mm_unpacklo_epi8(*x5, *x4); // q  xx xx xx xx xx xx xx xx 20 30 21 31 22 32 23 33
    w12 = _mm_unpacklo_epi8(*x3, *x2); // q  xx xx xx xx xx xx xx xx 40 50 41 51 42 52 43 53
    w13 = _mm_unpacklo_epi8(*x1, *x0); // q  xx xx xx xx xx xx xx xx 60 70 61 71 62 72 63 73
    w4 = _mm_unpackhi_epi16(w10, w11); // 00 10 20 30 01 11 21 31 02 12 22 32 03 13 23 33
    w5 = _mm_unpackhi_epi16(w12, w13); // 40 50 60 70 41 51 61 71 42 52 62 72 43 53 63 73
    d1 = _mm_unpacklo_epi32(w4, w5); // 00 10 20 30 40 50 60 70 01 11 21 31 41 51 61 71
    d3 = _mm_unpackhi_epi32(w4, w5); // 02 12 22 32 42 52 62 72 03 13 23 33 43 53 63 73
    *pq0 = _mm_unpacklo_epi64(d0, d1); // pq
    *pq1 = _mm_unpackhi_epi64(d0, d1); // pq
    *pq2 = _mm_unpacklo_epi64(d2, d3); // pq
    *pq3 = _mm_unpackhi_epi64(d2, d3);
}

// pq
#[inline]
#[target_feature(enable = "sse2")]
unsafe fn filter4_sse2(
    p1p0: *mut __m128i,
    q1q0: *mut __m128i,
    hev: *mut __m128i,
    mask: *mut __m128i,
    qs1qs0: *mut __m128i,
    ps1ps0: *mut __m128i,
) {
    let mut filter: __m128i;
    let mut filter2filter1: __m128i;
    let mut work: __m128i;
    let mut ps1ps0_work: __m128i;
    let mut qs1qs0_work: __m128i;
    let mut hev1: __m128i;
    let t3t4: __m128i = _mm_set_epi8(0, 0, 0, 0, 0, 0, 0, 0, 3, 3, 3, 3, 4, 4, 4, 4);
    let t80: __m128i = _mm_set1_epi8(0x80_i32 as i8);
    let ff: __m128i = _mm_cmpeq_epi8(t80, t80);
    ps1ps0_work = _mm_xor_si128(*p1p0, t80);
    // ^ 0x80
    qs1qs0_work = _mm_xor_si128(*q1q0, t80);
    // int8_t filter = signed_char_clamp(ps1 - qs1) & hev;
    work = _mm_subs_epi8(ps1ps0_work, qs1qs0_work);
    filter = _mm_and_si128(_mm_srli_si128::<4>(work), *hev);
    // filter = signed_char_clamp(filter + 3 * (qs0 - ps0)) & mask;
    filter = _mm_subs_epi8(filter, work);
    filter = _mm_subs_epi8(filter, work);
    filter = _mm_subs_epi8(filter, work);
    // + 3 * (qs0 - ps0)
    filter = _mm_and_si128(filter, *mask);
    // & mask
    filter = _mm_unpacklo_epi32(filter, filter);
    // filter1 = signed_char_clamp(filter + 4) >> 3;
    // filter2 = signed_char_clamp(filter + 3) >> 3;
    filter2filter1 = _mm_adds_epi8(filter, t3t4);
    // signed_char_clamp
    filter2filter1 = _mm_unpacklo_epi8(filter2filter1, filter2filter1); // goto 16 bit
    filter2filter1 = _mm_srai_epi16::<{ 11 }>(filter2filter1);
    // >> 3
    filter2filter1 = _mm_packs_epi16(filter2filter1, filter2filter1);
    // filter = ROUND_POWER_OF_TWO(filter1, 1) & ~hev;
    filter = _mm_subs_epi8(filter2filter1, ff);
    // + 1
    filter = _mm_unpacklo_epi8(filter, filter); // goto 16 bit
    filter = _mm_srai_epi16::<{ 9 }>(filter);
    // round
    filter = _mm_packs_epi16(filter, filter);
    filter = _mm_andnot_si128(*hev, filter);
    filter = _mm_unpacklo_epi32(filter, filter);
    filter2filter1 = _mm_unpacklo_epi32(filter2filter1, filter);
    hev1 = _mm_srli_si128::<8>(filter2filter1);
    // signed_char_clamp(qs1 - filter), signed_char_clamp(qs0 - filter1)
    qs1qs0_work = _mm_subs_epi8(qs1qs0_work, filter2filter1);
    // signed_char_clamp(ps1 + filter), signed_char_clamp(ps0 + filter2)
    ps1ps0_work = _mm_adds_epi8(ps1ps0_work, hev1);
    *qs1qs0 = _mm_xor_si128(qs1qs0_work, t80);
    // ^ 0x80
    *ps1ps0 = _mm_xor_si128(ps1ps0_work, t80);
}

// ^ 0x80
#[inline]
#[target_feature(enable = "sse2")]
unsafe fn filter4_dual_sse2(
    p1p0: *mut __m128i,
    q1q0: *mut __m128i,
    hev: *mut __m128i,
    mask: *mut __m128i,
    qs1qs0: *mut __m128i,
    ps1ps0: *mut __m128i,
) {
    let t3t4: __m128i = _mm_set_epi8(3, 3, 3, 3, 3, 3, 3, 3, 4, 4, 4, 4, 4, 4, 4, 4);
    let t80: __m128i = _mm_set1_epi8(0x80_i32 as i8);
    let mut filter: __m128i;
    let mut filter2filter1: __m128i;
    let mut work: __m128i;
    let mut ps1ps0_work: __m128i;
    let mut qs1qs0_work: __m128i;
    let mut hev1: __m128i;
    let ff: __m128i = _mm_cmpeq_epi8(t80, t80);
    ps1ps0_work = _mm_xor_si128(*p1p0, t80);
    // ^ 0x80
    qs1qs0_work = _mm_xor_si128(*q1q0, t80);
    // int8_t filter = signed_char_clamp(ps1 - qs1) & hev;
    work = _mm_subs_epi8(ps1ps0_work, qs1qs0_work);
    filter = _mm_and_si128(_mm_srli_si128::<8>(work), *hev);
    // filter = signed_char_clamp(filter + 3 * (qs0 - ps0)) & mask;
    filter = _mm_subs_epi8(filter, work);
    filter = _mm_subs_epi8(filter, work);
    filter = _mm_subs_epi8(filter, work);
    // + 3 * (qs0 - ps0)
    filter = _mm_and_si128(filter, *mask);
    // & mask
    filter = _mm_unpacklo_epi64(filter, filter);
    // filter1 = signed_char_clamp(filter + 4) >> 3;
    // filter2 = signed_char_clamp(filter + 3) >> 3;
    filter2filter1 = _mm_adds_epi8(filter, t3t4);
    // signed_char_clamp
    filter = _mm_unpackhi_epi8(filter2filter1, filter2filter1);
    filter2filter1 = _mm_unpacklo_epi8(filter2filter1, filter2filter1);
    filter2filter1 = _mm_srai_epi16::<{ 11 }>(filter2filter1);
    // >> 3
    filter = _mm_srai_epi16::<{ 11 }>(filter);
    // >> 3
    filter2filter1 = _mm_packs_epi16(filter2filter1, filter);
    // filter = ROUND_POWER_OF_TWO(filter1, 1) & ~hev;
    filter = _mm_subs_epi8(filter2filter1, ff);
    // + 1
    filter = _mm_unpacklo_epi8(filter, filter);
    filter = _mm_srai_epi16::<{ 9 }>(filter);
    // round
    filter = _mm_packs_epi16(filter, filter);
    filter = _mm_andnot_si128(*hev, filter);
    hev1 = _mm_unpackhi_epi64(filter2filter1, filter);
    filter2filter1 = _mm_unpacklo_epi64(filter2filter1, filter);
    // signed_char_clamp(qs1 - filter), signed_char_clamp(qs0 - filter1)
    qs1qs0_work = _mm_subs_epi8(qs1qs0_work, filter2filter1);
    // signed_char_clamp(ps1 + filter), signed_char_clamp(ps0 + filter2)
    ps1ps0_work = _mm_adds_epi8(ps1ps0_work, hev1);
    *qs1qs0 = _mm_xor_si128(qs1qs0_work, t80);
    // ^ 0x80
    *ps1ps0 = _mm_xor_si128(ps1ps0_work, t80);
}

// ^ 0x80
#[inline]
#[target_feature(enable = "sse2")]
unsafe fn lpf_internal_4_sse2(
    p1: *mut __m128i,
    p0: *mut __m128i,
    q0: *mut __m128i,
    q1: *mut __m128i,
    limit: *mut __m128i,
    thresh: *mut __m128i,
    q1q0_out: *mut __m128i,
    p1p0_out: *mut __m128i,
) {
    let mut q1p1: __m128i;
    let mut q0p0: __m128i;
    let mut p1p0: __m128i = core::mem::zeroed();
    let mut q1q0: __m128i = core::mem::zeroed();
    let mut abs_p0q0: __m128i;
    let mut abs_p1q1: __m128i;
    let mut mask: __m128i = core::mem::zeroed();
    let mut flat: __m128i;
    let mut hev: __m128i = core::mem::zeroed();
    let zero: __m128i = _mm_setzero_si128();
    q1p1 = _mm_unpacklo_epi32(*p1, *q1);
    q0p0 = _mm_unpacklo_epi32(*p0, *q0);
    p1p0 = _mm_unpacklo_epi32(q0p0, q1p1);
    q1q0 = _mm_srli_si128::<8>(p1p0);
    // (abs(q1 - q0), abs(p1 - p0)
    flat = abs_diff(q1p1, q0p0);
    // abs(p1 - q1), abs(p0 - q0)
    let mut abs_p1q1p0q0: __m128i = abs_diff(p1p0, q1q0);
    // const uint8_t hev = hev_mask(thresh, *op1, *op0, *oq0, *oq1);
    flat = _mm_max_epu8(flat, _mm_srli_si128::<4>(flat));
    hev = _mm_unpacklo_epi8(flat, zero);
    hev = _mm_cmpgt_epi16(hev, *thresh);
    hev = _mm_packs_epi16(hev, hev);
    hev = _mm_unpacklo_epi32(hev, hev);
    abs_p0q0 = _mm_adds_epu8(abs_p1q1p0q0, abs_p1q1p0q0);
    // abs(p0 - q0) * 2
    abs_p1q1 = _mm_srli_si128::<4>(abs_p1q1p0q0);
    // abs(p1 - q1)
    abs_p1q1 = _mm_unpacklo_epi8(abs_p1q1, abs_p1q1);
    abs_p1q1 = _mm_srli_epi16::<{ 9 }>(abs_p1q1);
    abs_p1q1 = _mm_packs_epi16(abs_p1q1, abs_p1q1);
    // abs(p1 - q1) / 2
    // abs(p0 - q0) * 2 + abs(p1 - q1) / 2
    mask = _mm_adds_epu8(abs_p0q0, abs_p1q1);
    mask = _mm_unpacklo_epi32(mask, flat);
    mask = _mm_subs_epu8(mask, *limit);
    mask = _mm_cmpeq_epi8(mask, zero);
    mask = _mm_and_si128(mask, _mm_srli_si128::<4>(mask));
    filter4_sse2(
        &raw mut p1p0,
        &raw mut q1q0,
        &raw mut hev,
        &raw mut mask,
        q1q0_out,
        p1p0_out,
    );
}

#[inline]
#[target_feature(enable = "sse2")]
unsafe fn lpf_internal_4_dual_sse2(
    p1: *mut __m128i,
    p0: *mut __m128i,
    q0: *mut __m128i,
    q1: *mut __m128i,
    limit: *mut __m128i,
    thresh: *mut __m128i,
    q1q0_out: *mut __m128i,
    p1p0_out: *mut __m128i,
) {
    let mut q1p1: __m128i;
    let mut q0p0: __m128i;
    let mut p1p0: __m128i = core::mem::zeroed();
    let mut q1q0: __m128i = core::mem::zeroed();
    let mut abs_p0q0: __m128i;
    let mut abs_p1q1: __m128i;
    let mut mask: __m128i = core::mem::zeroed();
    let mut hev: __m128i = core::mem::zeroed();
    let zero: __m128i = _mm_setzero_si128();
    q1p1 = _mm_unpacklo_epi64(*p1, *q1);
    q0p0 = _mm_unpacklo_epi64(*p0, *q0);
    p1p0 = _mm_unpacklo_epi64(q0p0, q1p1);
    q1q0 = _mm_unpackhi_epi64(q0p0, q1p1);
    // (abs(q1 - q0), abs(p1 - p0)
    let mut flat: __m128i = abs_diff(q1p1, q0p0);
    // abs(p1 - q1), abs(p0 - q0)
    let abs_p1q1p0q0: __m128i = abs_diff(p1p0, q1q0);
    // const uint8_t hev = hev_mask(thresh, *op1, *op0, *oq0, *oq1);
    flat = _mm_max_epu8(flat, _mm_srli_si128::<8>(flat));
    hev = _mm_unpacklo_epi8(flat, zero);
    hev = _mm_cmpgt_epi16(hev, *thresh);
    hev = _mm_packs_epi16(hev, hev);
    // const int8_t mask = filter_mask2(*limit, *blimit,
    // p1, p0, q0, q1);
    abs_p0q0 = _mm_adds_epu8(abs_p1q1p0q0, abs_p1q1p0q0);
    // abs(p0 - q0) * 2
    abs_p1q1 = _mm_unpackhi_epi8(abs_p1q1p0q0, abs_p1q1p0q0);
    // abs(p1 - q1)
    abs_p1q1 = _mm_srli_epi16::<{ 9 }>(abs_p1q1);
    abs_p1q1 = _mm_packs_epi16(abs_p1q1, abs_p1q1);
    // abs(p1 - q1) / 2
    // abs(p0 - q0) * 2 + abs(p1 - q1) / 2
    mask = _mm_adds_epu8(abs_p0q0, abs_p1q1);
    mask = _mm_unpacklo_epi64(mask, flat);
    mask = _mm_subs_epu8(mask, *limit);
    mask = _mm_cmpeq_epi8(mask, zero);
    mask = _mm_and_si128(mask, _mm_srli_si128::<8>(mask));
    filter4_dual_sse2(
        &raw mut p1p0,
        &raw mut q1q0,
        &raw mut hev,
        &raw mut mask,
        q1q0_out,
        p1p0_out,
    );
}

#[no_mangle]
#[target_feature(enable = "sse2")]
pub unsafe extern "C-unwind" fn aom_lpf_horizontal_4_sse2(
    s: *mut u8,
    p: i32,
    _blimit: *const u8,
    _limit: *const u8,
    _thresh: *const u8,
) {
    // pitch
    let zero: __m128i = _mm_setzero_si128();
    let mut limit: __m128i = _mm_unpacklo_epi32(
        _mm_loadl_epi64(_blimit as *const __m128i),
        _mm_loadl_epi64(_limit as *const __m128i),
    );
    let mut thresh: __m128i = _mm_unpacklo_epi8(_mm_loadl_epi64(_thresh as *const __m128i), zero);
    let mut qs1qs0: __m128i = core::mem::zeroed();
    let mut ps1ps0: __m128i = core::mem::zeroed();
    let mut p1: __m128i = core::mem::zeroed();
    let mut p0: __m128i = core::mem::zeroed();
    let mut q0: __m128i = core::mem::zeroed();
    let mut q1: __m128i = core::mem::zeroed();
    p1 = xx_loadl_32(s.offset(-((2 * p) as isize)) as *const core::ffi::c_void);
    p0 = xx_loadl_32(s.offset(-((1 * p) as isize)) as *const core::ffi::c_void);
    q0 = xx_loadl_32(s.offset(-((0 * p) as isize)) as *const core::ffi::c_void);
    q1 = xx_loadl_32(s.offset((1 * p) as isize) as *const core::ffi::c_void);
    lpf_internal_4_sse2(
        &raw mut p1,
        &raw mut p0,
        &raw mut q0,
        &raw mut q1,
        &raw mut limit,
        &raw mut thresh,
        &raw mut qs1qs0,
        &raw mut ps1ps0,
    );
    xx_storel_32(
        s.offset(-((1 * p) as isize)) as *mut core::ffi::c_void,
        ps1ps0,
    );
    xx_storel_32(
        s.offset(-((2 * p) as isize)) as *mut core::ffi::c_void,
        _mm_srli_si128::<4>(ps1ps0),
    );
    xx_storel_32(s.offset((0 * p) as isize) as *mut core::ffi::c_void, qs1qs0);
    xx_storel_32(
        s.offset((1 * p) as isize) as *mut core::ffi::c_void,
        _mm_srli_si128::<4>(qs1qs0),
    );
}

#[no_mangle]
#[target_feature(enable = "sse2")]
pub unsafe extern "C-unwind" fn aom_lpf_vertical_4_sse2(
    s: *mut u8,
    p: i32,
    _blimit: *const u8,
    _limit: *const u8,
    _thresh: *const u8,
) {
    // pitch
    let mut p1p0: __m128i = core::mem::zeroed();
    let mut q1q0: __m128i = core::mem::zeroed();
    let mut p1: __m128i = core::mem::zeroed();
    let mut p0: __m128i = core::mem::zeroed();
    let mut q0: __m128i = core::mem::zeroed();
    let mut q1: __m128i = core::mem::zeroed();
    let zero: __m128i = _mm_setzero_si128();
    let mut limit: __m128i = _mm_unpacklo_epi32(
        _mm_loadl_epi64(_blimit as *const __m128i),
        _mm_loadl_epi64(_limit as *const __m128i),
    );
    let mut thresh: __m128i = _mm_unpacklo_epi8(_mm_loadl_epi64(_thresh as *const __m128i), zero);
    let mut x0: __m128i = core::mem::zeroed();
    let mut x1: __m128i = core::mem::zeroed();
    let mut x2: __m128i = core::mem::zeroed();
    let mut x3: __m128i = core::mem::zeroed();
    let mut d0: __m128i = core::mem::zeroed();
    let mut d1: __m128i = core::mem::zeroed();
    let mut d2: __m128i = core::mem::zeroed();
    let mut d3: __m128i = core::mem::zeroed();
    x0 = _mm_loadl_epi64(s.offset(-2).offset((0 * p) as isize) as *mut __m128i as *const __m128i);
    x1 = _mm_loadl_epi64(s.offset(-2).offset((1 * p) as isize) as *mut __m128i as *const __m128i);
    x2 = _mm_loadl_epi64(s.offset(-2).offset((2 * p) as isize) as *mut __m128i as *const __m128i);
    x3 = _mm_loadl_epi64(s.offset(-2).offset((3 * p) as isize) as *mut __m128i as *const __m128i);
    transpose4x8_8x4_low_sse2(
        &raw mut x0,
        &raw mut x1,
        &raw mut x2,
        &raw mut x3,
        &raw mut p1,
        &raw mut p0,
        &raw mut q0,
        &raw mut q1,
    );
    lpf_internal_4_sse2(
        &raw mut p1,
        &raw mut p0,
        &raw mut q0,
        &raw mut q1,
        &raw mut limit,
        &raw mut thresh,
        &raw mut q1q0,
        &raw mut p1p0,
    );
    // Transpose 8x4 to 4x8
    p1 = _mm_srli_si128::<4>(p1p0);
    q1 = _mm_srli_si128::<4>(q1q0);
    transpose4x8_8x4_low_sse2(
        &raw mut p1,
        &raw mut p1p0,
        &raw mut q1q0,
        &raw mut q1,
        &raw mut d0,
        &raw mut d1,
        &raw mut d2,
        &raw mut d3,
    );
    xx_storel_32(
        s.offset((0 * p) as isize).offset(-2) as *mut core::ffi::c_void,
        d0,
    );
    xx_storel_32(
        s.offset((1 * p) as isize).offset(-2) as *mut core::ffi::c_void,
        d1,
    );
    xx_storel_32(
        s.offset((2 * p) as isize).offset(-2) as *mut core::ffi::c_void,
        d2,
    );
    xx_storel_32(
        s.offset((3 * p) as isize).offset(-2) as *mut core::ffi::c_void,
        d3,
    );
}

#[inline]
#[target_feature(enable = "sse2")]
unsafe fn store_buffer_horz_8(x: __m128i, p: i32, num: i32, s: *mut u8) {
    xx_storel_32(
        s.offset(-(((num + 1) * p) as isize)) as *mut core::ffi::c_void,
        x,
    );
    xx_storel_32(
        s.offset((num * p) as isize) as *mut core::ffi::c_void,
        _mm_srli_si128::<4>(x),
    );
}

#[inline]
#[target_feature(enable = "sse2")]
unsafe fn lpf_internal_14_dual_sse2(
    q6p6: *mut __m128i,
    q5p5: *mut __m128i,
    q4p4: *mut __m128i,
    q3p3: *mut __m128i,
    q2p2: *mut __m128i,
    q1p1: *mut __m128i,
    q0p0: *mut __m128i,
    blimit: *mut __m128i,
    limit: *mut __m128i,
    thresh: *mut __m128i,
) {
    let zero: __m128i = _mm_setzero_si128();
    let one: __m128i = _mm_set1_epi8(1);
    let mut mask: __m128i = core::mem::zeroed();
    let mut hev: __m128i = core::mem::zeroed();
    let mut flat: __m128i;
    let mut flat2: __m128i;
    let mut qs0ps0: __m128i;
    let mut qs1ps1: __m128i;
    let mut p1p0: __m128i = core::mem::zeroed();
    let mut q1q0: __m128i = core::mem::zeroed();
    let mut qs1qs0: __m128i = core::mem::zeroed();
    let mut ps1ps0: __m128i = core::mem::zeroed();
    let mut abs_p1p0: __m128i;
    p1p0 = _mm_unpacklo_epi64(*q0p0, *q1p1);
    q1q0 = _mm_unpackhi_epi64(*q0p0, *q1p1);
    {
        let mut abs_p1q1: __m128i;
        let mut abs_p0q0: __m128i;
        let mut abs_q1q0: __m128i;
        let mut fe: __m128i;
        let mut ff: __m128i;
        let mut work: __m128i;
        abs_p1p0 = abs_diff(*q1p1, *q0p0);
        abs_q1q0 = _mm_srli_si128::<8>(abs_p1p0);
        fe = _mm_set1_epi8(0xfe_i32 as i8);
        ff = _mm_cmpeq_epi8(abs_p1p0, abs_p1p0);
        abs_p0q0 = abs_diff(p1p0, q1q0);
        abs_p1q1 = _mm_srli_si128::<8>(abs_p0q0);
        abs_p0q0 = _mm_unpacklo_epi64(abs_p0q0, zero);
        flat = _mm_max_epu8(abs_p1p0, abs_q1q0);
        hev = _mm_subs_epu8(flat, *thresh);
        hev = _mm_xor_si128(_mm_cmpeq_epi8(hev, zero), ff);
        // replicate for the further "merged variables" usage
        hev = _mm_unpacklo_epi64(hev, hev);
        abs_p0q0 = _mm_adds_epu8(abs_p0q0, abs_p0q0);
        abs_p1q1 = _mm_srli_epi16::<{ 1 }>(_mm_and_si128(abs_p1q1, fe));
        mask = _mm_subs_epu8(_mm_adds_epu8(abs_p0q0, abs_p1q1), *blimit);
        mask = _mm_xor_si128(_mm_cmpeq_epi8(mask, zero), ff);
        // mask |= (abs(p0 - q0) * 2 + abs(p1 - q1) / 2  > blimit) * -1;
        mask = _mm_max_epu8(abs_p1p0, mask);
        // mask |= (abs(p1 - p0) > limit) * -1;
        // mask |= (abs(q1 - q0) > limit) * -1;
        work = _mm_max_epu8(abs_diff(*q2p2, *q1p1), abs_diff(*q3p3, *q2p2));
        mask = _mm_max_epu8(work, mask);
        mask = _mm_max_epu8(mask, _mm_srli_si128::<8>(mask));
        mask = _mm_subs_epu8(mask, *limit);
        mask = _mm_cmpeq_epi8(mask, zero);
    }
    // lp filter - the same for 6, 8 and 14 versions
    filter4_dual_sse2(
        &raw mut p1p0,
        &raw mut q1q0,
        &raw mut hev,
        &raw mut mask,
        &raw mut qs1qs0,
        &raw mut ps1ps0,
    );
    qs0ps0 = _mm_unpacklo_epi64(ps1ps0, qs1qs0);
    qs1ps1 = _mm_unpackhi_epi64(ps1ps0, qs1qs0);
    // loopfilter done
    let mut flat2_q5p5: __m128i;
    let mut flat2_q4p4: __m128i;
    let mut flat2_q3p3: __m128i;
    let mut flat2_q2p2: __m128i;
    let mut flat2_q1p1: __m128i;
    let mut flat2_q0p0: __m128i;
    let mut flat_q2p2: __m128i;
    let mut flat_q1p1: __m128i;
    let mut flat_q0p0: __m128i;
    let mut work: __m128i;
    flat = _mm_max_epu8(abs_diff(*q2p2, *q0p0), abs_diff(*q3p3, *q0p0));
    flat = _mm_max_epu8(abs_p1p0, flat);
    flat = _mm_max_epu8(flat, _mm_srli_si128::<8>(flat));
    flat = _mm_subs_epu8(flat, one);
    flat = _mm_cmpeq_epi8(flat, zero);
    flat = _mm_and_si128(flat, mask);
    // if flat ==0 then flat2 is zero as well and we don't need any calc below
    // sse4.1 if (0==_mm_test_all_zeros(flat,ff))
    if 0xffff != _mm_movemask_epi8(_mm_cmpeq_epi8(flat, zero)) {
        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        // flat and wide flat calculations
        let eight: __m128i = _mm_set1_epi16(8);
        let four: __m128i = _mm_set1_epi16(4);
        let mut p6_16: __m128i;
        let mut p5_16: __m128i;
        let mut p4_16: __m128i;
        let mut p3_16: __m128i;
        let mut p2_16: __m128i;
        let mut p1_16: __m128i;
        let mut p0_16: __m128i;
        let mut q6_16: __m128i;
        let mut q5_16: __m128i;
        let mut q4_16: __m128i;
        let mut q3_16: __m128i;
        let mut q2_16: __m128i;
        let mut q1_16: __m128i;
        let mut q0_16: __m128i;
        let mut pixelFilter_p: __m128i;
        let mut pixelFilter_q: __m128i;
        let mut pixetFilter_p2p1p0: __m128i;
        let mut pixetFilter_q2q1q0: __m128i;
        let mut sum_p6: __m128i;
        let mut sum_q6: __m128i;
        let mut sum_p3: __m128i;
        let mut sum_q3: __m128i;
        let mut res_p: __m128i;
        let mut res_q: __m128i;
        p6_16 = _mm_unpacklo_epi8(*q6p6, zero);
        p5_16 = _mm_unpacklo_epi8(*q5p5, zero);
        p4_16 = _mm_unpacklo_epi8(*q4p4, zero);
        p3_16 = _mm_unpacklo_epi8(*q3p3, zero);
        p2_16 = _mm_unpacklo_epi8(*q2p2, zero);
        p1_16 = _mm_unpacklo_epi8(*q1p1, zero);
        p0_16 = _mm_unpacklo_epi8(*q0p0, zero);
        q0_16 = _mm_unpackhi_epi8(*q0p0, zero);
        q1_16 = _mm_unpackhi_epi8(*q1p1, zero);
        q2_16 = _mm_unpackhi_epi8(*q2p2, zero);
        q3_16 = _mm_unpackhi_epi8(*q3p3, zero);
        q4_16 = _mm_unpackhi_epi8(*q4p4, zero);
        q5_16 = _mm_unpackhi_epi8(*q5p5, zero);
        q6_16 = _mm_unpackhi_epi8(*q6p6, zero);
        pixelFilter_p = _mm_add_epi16(p5_16, _mm_add_epi16(p4_16, p3_16));
        pixelFilter_q = _mm_add_epi16(q5_16, _mm_add_epi16(q4_16, q3_16));
        pixetFilter_p2p1p0 = _mm_add_epi16(p0_16, _mm_add_epi16(p2_16, p1_16));
        pixelFilter_p = _mm_add_epi16(pixelFilter_p, pixetFilter_p2p1p0);
        pixetFilter_q2q1q0 = _mm_add_epi16(q0_16, _mm_add_epi16(q2_16, q1_16));
        pixelFilter_q = _mm_add_epi16(pixelFilter_q, pixetFilter_q2q1q0);
        pixelFilter_p = _mm_add_epi16(eight, _mm_add_epi16(pixelFilter_p, pixelFilter_q));
        pixetFilter_p2p1p0 =
            _mm_add_epi16(four, _mm_add_epi16(pixetFilter_p2p1p0, pixetFilter_q2q1q0));
        res_p = _mm_srli_epi16::<{ 4 }>(_mm_add_epi16(
            pixelFilter_p,
            _mm_add_epi16(_mm_add_epi16(p6_16, p0_16), _mm_add_epi16(p1_16, q0_16)),
        ));
        res_q = _mm_srli_epi16::<{ 4 }>(_mm_add_epi16(
            pixelFilter_p,
            _mm_add_epi16(_mm_add_epi16(q6_16, q0_16), _mm_add_epi16(p0_16, q1_16)),
        ));
        flat2_q0p0 = _mm_packus_epi16(res_p, res_q);
        res_p = _mm_srli_epi16::<{ 3 }>(_mm_add_epi16(
            pixetFilter_p2p1p0,
            _mm_add_epi16(p3_16, p0_16),
        ));
        res_q = _mm_srli_epi16::<{ 3 }>(_mm_add_epi16(
            pixetFilter_p2p1p0,
            _mm_add_epi16(q3_16, q0_16),
        ));
        flat_q0p0 = _mm_packus_epi16(res_p, res_q);
        sum_p6 = _mm_add_epi16(p6_16, p6_16);
        sum_q6 = _mm_add_epi16(q6_16, q6_16);
        sum_p3 = _mm_add_epi16(p3_16, p3_16);
        sum_q3 = _mm_add_epi16(q3_16, q3_16);
        pixelFilter_q = _mm_sub_epi16(pixelFilter_p, p5_16);
        pixelFilter_p = _mm_sub_epi16(pixelFilter_p, q5_16);
        res_p = _mm_srli_epi16::<{ 4 }>(_mm_add_epi16(
            pixelFilter_p,
            _mm_add_epi16(sum_p6, _mm_add_epi16(p1_16, _mm_add_epi16(p2_16, p0_16))),
        ));
        res_q = _mm_srli_epi16::<{ 4 }>(_mm_add_epi16(
            pixelFilter_q,
            _mm_add_epi16(sum_q6, _mm_add_epi16(q1_16, _mm_add_epi16(q0_16, q2_16))),
        ));
        flat2_q1p1 = _mm_packus_epi16(res_p, res_q);
        pixetFilter_q2q1q0 = _mm_sub_epi16(pixetFilter_p2p1p0, p2_16);
        pixetFilter_p2p1p0 = _mm_sub_epi16(pixetFilter_p2p1p0, q2_16);
        res_p = _mm_srli_epi16::<{ 3 }>(_mm_add_epi16(
            pixetFilter_p2p1p0,
            _mm_add_epi16(sum_p3, p1_16),
        ));
        res_q = _mm_srli_epi16::<{ 3 }>(_mm_add_epi16(
            pixetFilter_q2q1q0,
            _mm_add_epi16(sum_q3, q1_16),
        ));
        flat_q1p1 = _mm_packus_epi16(res_p, res_q);
        pixetFilter_p2p1p0 = _mm_sub_epi16(pixetFilter_p2p1p0, q1_16);
        pixetFilter_q2q1q0 = _mm_sub_epi16(pixetFilter_q2q1q0, p1_16);
        sum_p3 = _mm_add_epi16(sum_p3, p3_16);
        sum_q3 = _mm_add_epi16(sum_q3, q3_16);
        res_p = _mm_srli_epi16::<{ 3 }>(_mm_add_epi16(
            pixetFilter_p2p1p0,
            _mm_add_epi16(sum_p3, p2_16),
        ));
        res_q = _mm_srli_epi16::<{ 3 }>(_mm_add_epi16(
            pixetFilter_q2q1q0,
            _mm_add_epi16(sum_q3, q2_16),
        ));
        flat_q2p2 = _mm_packus_epi16(res_p, res_q);
        // work with flat2
        flat2 = _mm_max_epu8(abs_diff(*q4p4, *q0p0), abs_diff(*q5p5, *q0p0));
        work = abs_diff(*q6p6, *q0p0);
        flat2 = _mm_max_epu8(work, flat2);
        flat2 = _mm_max_epu8(flat2, _mm_srli_si128::<8>(flat2));
        flat2 = _mm_subs_epu8(flat2, one);
        flat2 = _mm_cmpeq_epi8(flat2, zero);
        flat2 = _mm_and_si128(flat2, flat); // flat2 & flat & mask
                                            // ~~~~~~~~~~ apply flat ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        flat = _mm_unpacklo_epi64(flat, flat);
        *q2p2 = _mm_andnot_si128(flat, *q2p2);
        flat_q2p2 = _mm_and_si128(flat, flat_q2p2);
        *q2p2 = _mm_or_si128(*q2p2, flat_q2p2);
        qs1ps1 = _mm_andnot_si128(flat, qs1ps1);
        flat_q1p1 = _mm_and_si128(flat, flat_q1p1);
        *q1p1 = _mm_or_si128(qs1ps1, flat_q1p1);
        qs0ps0 = _mm_andnot_si128(flat, qs0ps0);
        flat_q0p0 = _mm_and_si128(flat, flat_q0p0);
        *q0p0 = _mm_or_si128(qs0ps0, flat_q0p0);
        if 0xffff != _mm_movemask_epi8(_mm_cmpeq_epi8(flat2, zero)) {
            pixelFilter_p = _mm_sub_epi16(pixelFilter_p, q4_16);
            pixelFilter_q = _mm_sub_epi16(pixelFilter_q, p4_16);
            sum_p6 = _mm_add_epi16(sum_p6, p6_16);
            sum_q6 = _mm_add_epi16(sum_q6, q6_16);
            res_p = _mm_srli_epi16::<{ 4 }>(_mm_add_epi16(
                pixelFilter_p,
                _mm_add_epi16(sum_p6, _mm_add_epi16(p2_16, _mm_add_epi16(p3_16, p1_16))),
            ));
            res_q = _mm_srli_epi16::<{ 4 }>(_mm_add_epi16(
                pixelFilter_q,
                _mm_add_epi16(sum_q6, _mm_add_epi16(q2_16, _mm_add_epi16(q1_16, q3_16))),
            ));
            flat2_q2p2 = _mm_packus_epi16(res_p, res_q);
            sum_p6 = _mm_add_epi16(sum_p6, p6_16);
            sum_q6 = _mm_add_epi16(sum_q6, q6_16);
            pixelFilter_p = _mm_sub_epi16(pixelFilter_p, q3_16);
            pixelFilter_q = _mm_sub_epi16(pixelFilter_q, p3_16);
            res_p = _mm_srli_epi16::<{ 4 }>(_mm_add_epi16(
                pixelFilter_p,
                _mm_add_epi16(sum_p6, _mm_add_epi16(p3_16, _mm_add_epi16(p4_16, p2_16))),
            ));
            res_q = _mm_srli_epi16::<{ 4 }>(_mm_add_epi16(
                pixelFilter_q,
                _mm_add_epi16(sum_q6, _mm_add_epi16(q3_16, _mm_add_epi16(q2_16, q4_16))),
            ));
            flat2_q3p3 = _mm_packus_epi16(res_p, res_q);
            sum_p6 = _mm_add_epi16(sum_p6, p6_16);
            sum_q6 = _mm_add_epi16(sum_q6, q6_16);
            pixelFilter_p = _mm_sub_epi16(pixelFilter_p, q2_16);
            pixelFilter_q = _mm_sub_epi16(pixelFilter_q, p2_16);
            res_p = _mm_srli_epi16::<{ 4 }>(_mm_add_epi16(
                pixelFilter_p,
                _mm_add_epi16(sum_p6, _mm_add_epi16(p4_16, _mm_add_epi16(p5_16, p3_16))),
            ));
            res_q = _mm_srli_epi16::<{ 4 }>(_mm_add_epi16(
                pixelFilter_q,
                _mm_add_epi16(sum_q6, _mm_add_epi16(q4_16, _mm_add_epi16(q3_16, q5_16))),
            ));
            flat2_q4p4 = _mm_packus_epi16(res_p, res_q);
            sum_p6 = _mm_add_epi16(sum_p6, p6_16);
            sum_q6 = _mm_add_epi16(sum_q6, q6_16);
            pixelFilter_p = _mm_sub_epi16(pixelFilter_p, q1_16);
            pixelFilter_q = _mm_sub_epi16(pixelFilter_q, p1_16);
            res_p = _mm_srli_epi16::<{ 4 }>(_mm_add_epi16(
                pixelFilter_p,
                _mm_add_epi16(sum_p6, _mm_add_epi16(p5_16, _mm_add_epi16(p6_16, p4_16))),
            ));
            res_q = _mm_srli_epi16::<{ 4 }>(_mm_add_epi16(
                pixelFilter_q,
                _mm_add_epi16(sum_q6, _mm_add_epi16(q5_16, _mm_add_epi16(q6_16, q4_16))),
            ));
            flat2_q5p5 = _mm_packus_epi16(res_p, res_q);
            // wide flat
            // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            flat2 = _mm_unpacklo_epi64(flat2, flat2);
            *q5p5 = _mm_andnot_si128(flat2, *q5p5);
            flat2_q5p5 = _mm_and_si128(flat2, flat2_q5p5);
            *q5p5 = _mm_or_si128(*q5p5, flat2_q5p5);
            *q4p4 = _mm_andnot_si128(flat2, *q4p4);
            flat2_q4p4 = _mm_and_si128(flat2, flat2_q4p4);
            *q4p4 = _mm_or_si128(*q4p4, flat2_q4p4);
            *q3p3 = _mm_andnot_si128(flat2, *q3p3);
            flat2_q3p3 = _mm_and_si128(flat2, flat2_q3p3);
            *q3p3 = _mm_or_si128(*q3p3, flat2_q3p3);
            *q2p2 = _mm_andnot_si128(flat2, *q2p2);
            flat2_q2p2 = _mm_and_si128(flat2, flat2_q2p2);
            *q2p2 = _mm_or_si128(*q2p2, flat2_q2p2);
            *q1p1 = _mm_andnot_si128(flat2, *q1p1);
            flat2_q1p1 = _mm_and_si128(flat2, flat2_q1p1);
            *q1p1 = _mm_or_si128(*q1p1, flat2_q1p1);
            *q0p0 = _mm_andnot_si128(flat2, *q0p0);
            flat2_q0p0 = _mm_and_si128(flat2, flat2_q0p0);
            *q0p0 = _mm_or_si128(*q0p0, flat2_q0p0);
        }
    } else {
        *q0p0 = qs0ps0;
        *q1p1 = qs1ps1;
    }
}

#[inline]
#[target_feature(enable = "sse2")]
unsafe fn lpf_internal_14_sse2(
    q6p6: *mut __m128i,
    q5p5: *mut __m128i,
    q4p4: *mut __m128i,
    q3p3: *mut __m128i,
    q2p2: *mut __m128i,
    q1p1: *mut __m128i,
    q0p0: *mut __m128i,
    blimit: *mut __m128i,
    limit: *mut __m128i,
    thresh: *mut __m128i,
) {
    let zero: __m128i = _mm_setzero_si128();
    let one: __m128i = _mm_set1_epi8(1);
    let mut mask: __m128i = core::mem::zeroed();
    let mut hev: __m128i = core::mem::zeroed();
    let mut flat: __m128i;
    let mut flat2: __m128i;
    let mut flat2_pq: [__m128i; 6] = core::mem::zeroed();
    let mut flat_pq: [__m128i; 3] = core::mem::zeroed();
    let mut qs0ps0: __m128i;
    let mut qs1ps1: __m128i;
    let mut p1p0: __m128i = core::mem::zeroed();
    let mut q1q0: __m128i = core::mem::zeroed();
    let mut qs1qs0: __m128i = core::mem::zeroed();
    let mut ps1ps0: __m128i = core::mem::zeroed();
    let mut abs_p1p0: __m128i;
    p1p0 = _mm_unpacklo_epi32(*q0p0, *q1p1);
    q1q0 = _mm_srli_si128::<8>(p1p0);
    let mut fe: __m128i;
    let mut ff: __m128i;
    let mut work: __m128i;
    {
        let mut abs_p1q1: __m128i;
        let mut abs_p0q0: __m128i;
        let mut abs_q1q0: __m128i;
        abs_p1p0 = abs_diff(*q1p1, *q0p0);
        abs_q1q0 = _mm_srli_si128::<4>(abs_p1p0);
        fe = _mm_set1_epi8(0xfe_i32 as i8);
        ff = _mm_cmpeq_epi8(fe, fe);
        abs_p0q0 = abs_diff(p1p0, q1q0);
        abs_p1q1 = _mm_srli_si128::<4>(abs_p0q0);
        flat = _mm_max_epu8(abs_p1p0, abs_q1q0);
        hev = _mm_subs_epu8(flat, *thresh);
        hev = _mm_xor_si128(_mm_cmpeq_epi8(hev, zero), ff);
        // replicate for the further "merged variables" usage
        hev = _mm_unpacklo_epi32(hev, hev);
        abs_p0q0 = _mm_adds_epu8(abs_p0q0, abs_p0q0);
        abs_p1q1 = _mm_srli_epi16::<{ 1 }>(_mm_and_si128(abs_p1q1, fe));
        mask = _mm_subs_epu8(_mm_adds_epu8(abs_p0q0, abs_p1q1), *blimit);
        mask = _mm_unpacklo_epi32(mask, zero);
        mask = _mm_xor_si128(_mm_cmpeq_epi8(mask, zero), ff);
        // mask |= (abs(p0 - q0) * 2 + abs(p1 - q1) / 2  > blimit) * -1;
        mask = _mm_max_epu8(abs_p1p0, mask);
        // mask |= (abs(p1 - p0) > limit) * -1;
        // mask |= (abs(q1 - q0) > limit) * -1;
        work = _mm_max_epu8(abs_diff(*q2p2, *q1p1), abs_diff(*q3p3, *q2p2));
        mask = _mm_max_epu8(work, mask);
        mask = _mm_max_epu8(mask, _mm_srli_si128::<4>(mask));
        mask = _mm_subs_epu8(mask, *limit);
        mask = _mm_cmpeq_epi8(mask, zero);
    }
    // lp filter - the same for 6, 8 and 14 versions
    filter4_sse2(
        &raw mut p1p0,
        &raw mut q1q0,
        &raw mut hev,
        &raw mut mask,
        &raw mut qs1qs0,
        &raw mut ps1ps0,
    );
    qs0ps0 = _mm_unpacklo_epi32(ps1ps0, qs1qs0);
    qs1ps1 = _mm_srli_si128::<8>(qs0ps0);
    // loopfilter done
    flat = _mm_max_epu8(abs_diff(*q2p2, *q0p0), abs_diff(*q3p3, *q0p0));
    flat = _mm_max_epu8(abs_p1p0, flat);
    flat = _mm_max_epu8(flat, _mm_srli_si128::<4>(flat));
    flat = _mm_subs_epu8(flat, one);
    flat = _mm_cmpeq_epi8(flat, zero);
    flat = _mm_and_si128(flat, mask);
    flat = _mm_unpacklo_epi32(flat, flat);
    flat = _mm_unpacklo_epi64(flat, flat);
    // if flat ==0 then flat2 is zero as well and we don't need any calc below
    // sse4.1 if (0==_mm_test_all_zeros(flat,ff))
    if 0xffff != _mm_movemask_epi8(_mm_cmpeq_epi8(flat, zero)) {
        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        // flat and wide flat calculations
        let mut q5_16: __m128i;
        let mut q4_16: __m128i;
        let mut q3_16: __m128i;
        let mut q2_16: __m128i;
        let mut q1_16: __m128i;
        let mut q0_16: __m128i;
        let mut pq_16: [__m128i; 7] = core::mem::zeroed();
        let eight: __m128i = _mm_set1_epi16(8);
        let four: __m128i = _mm_set1_epi16(4);
        let mut sum_p6: __m128i;
        let mut sum_p3: __m128i;
        pq_16[0] = _mm_unpacklo_epi8(*q0p0, zero);
        pq_16[1] = _mm_unpacklo_epi8(*q1p1, zero);
        pq_16[2] = _mm_unpacklo_epi8(*q2p2, zero);
        pq_16[3] = _mm_unpacklo_epi8(*q3p3, zero);
        pq_16[4] = _mm_unpacklo_epi8(*q4p4, zero);
        pq_16[5] = _mm_unpacklo_epi8(*q5p5, zero);
        pq_16[6] = _mm_unpacklo_epi8(*q6p6, zero);
        q0_16 = _mm_srli_si128::<8>(pq_16[0]);
        q1_16 = _mm_srli_si128::<8>(pq_16[1]);
        q2_16 = _mm_srli_si128::<8>(pq_16[2]);
        q3_16 = _mm_srli_si128::<8>(pq_16[3]);
        q4_16 = _mm_srli_si128::<8>(pq_16[4]);
        q5_16 = _mm_srli_si128::<8>(pq_16[5]);
        let mut flat_p: [__m128i; 3] = core::mem::zeroed();
        let mut flat_q: [__m128i; 3] = core::mem::zeroed();
        let mut flat2_p: [__m128i; 6] = core::mem::zeroed();
        let mut flat2_q: [__m128i; 6] = core::mem::zeroed();
        let mut work0: __m128i;
        let mut work0_0: __m128i;
        let mut work0_1: __m128i;
        let mut sum_p_0: __m128i;
        let mut sum_p: __m128i = _mm_add_epi16(pq_16[5], _mm_add_epi16(pq_16[4], pq_16[3]));
        let mut sum_lp: __m128i = _mm_add_epi16(pq_16[0], _mm_add_epi16(pq_16[2], pq_16[1]));
        sum_p = _mm_add_epi16(sum_p, sum_lp);
        let mut sum_lq: __m128i = _mm_srli_si128::<8>(sum_lp);
        let mut sum_q: __m128i = _mm_srli_si128::<8>(sum_p);
        sum_p_0 = _mm_add_epi16(eight, _mm_add_epi16(sum_p, sum_q));
        sum_lp = _mm_add_epi16(four, _mm_add_epi16(sum_lp, sum_lq));
        flat_p[0] = _mm_add_epi16(sum_lp, _mm_add_epi16(pq_16[3], pq_16[0]));
        flat_q[0] = _mm_add_epi16(sum_lp, _mm_add_epi16(q3_16, q0_16));
        sum_p6 = _mm_add_epi16(pq_16[6], pq_16[6]);
        sum_p3 = _mm_add_epi16(pq_16[3], pq_16[3]);
        sum_q = _mm_sub_epi16(sum_p_0, pq_16[5]);
        sum_p = _mm_sub_epi16(sum_p_0, q5_16);
        work0_0 = _mm_add_epi16(_mm_add_epi16(pq_16[6], pq_16[0]), pq_16[1]);
        work0_1 = _mm_add_epi16(
            sum_p6,
            _mm_add_epi16(pq_16[1], _mm_add_epi16(pq_16[2], pq_16[0])),
        );
        sum_lq = _mm_sub_epi16(sum_lp, pq_16[2]);
        sum_lp = _mm_sub_epi16(sum_lp, q2_16);
        work0 = _mm_add_epi16(sum_p3, pq_16[1]);
        flat_p[1] = _mm_add_epi16(sum_lp, work0);
        flat_q[1] = _mm_add_epi16(sum_lq, _mm_srli_si128::<8>(work0));
        flat_pq[0] = _mm_srli_epi16::<{ 3 }>(_mm_unpacklo_epi64(flat_p[0], flat_q[0]));
        flat_pq[1] = _mm_srli_epi16::<{ 3 }>(_mm_unpacklo_epi64(flat_p[1], flat_q[1]));
        flat_pq[0] = _mm_packus_epi16(flat_pq[0], flat_pq[0]);
        flat_pq[1] = _mm_packus_epi16(flat_pq[1], flat_pq[1]);
        sum_lp = _mm_sub_epi16(sum_lp, q1_16);
        sum_lq = _mm_sub_epi16(sum_lq, pq_16[1]);
        sum_p3 = _mm_add_epi16(sum_p3, pq_16[3]);
        work0 = _mm_add_epi16(sum_p3, pq_16[2]);
        flat_p[2] = _mm_add_epi16(sum_lp, work0);
        flat_q[2] = _mm_add_epi16(sum_lq, _mm_srli_si128::<8>(work0));
        flat_pq[2] = _mm_srli_epi16::<{ 3 }>(_mm_unpacklo_epi64(flat_p[2], flat_q[2]));
        flat_pq[2] = _mm_packus_epi16(flat_pq[2], flat_pq[2]);
        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~ flat 2 ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        flat2 = _mm_max_epu8(abs_diff(*q4p4, *q0p0), abs_diff(*q5p5, *q0p0));
        work = abs_diff(*q6p6, *q0p0);
        flat2 = _mm_max_epu8(work, flat2);
        flat2 = _mm_max_epu8(flat2, _mm_srli_si128::<4>(flat2));
        flat2 = _mm_subs_epu8(flat2, one);
        flat2 = _mm_cmpeq_epi8(flat2, zero);
        flat2 = _mm_and_si128(flat2, flat); // flat2 & flat & mask
        flat2 = _mm_unpacklo_epi32(flat2, flat2);
        // ~~~~~~~~~~ apply flat ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        qs0ps0 = _mm_andnot_si128(flat, qs0ps0);
        flat_pq[0] = _mm_and_si128(flat, flat_pq[0]);
        *q0p0 = _mm_or_si128(qs0ps0, flat_pq[0]);
        qs1ps1 = _mm_andnot_si128(flat, qs1ps1);
        flat_pq[1] = _mm_and_si128(flat, flat_pq[1]);
        *q1p1 = _mm_or_si128(qs1ps1, flat_pq[1]);
        *q2p2 = _mm_andnot_si128(flat, *q2p2);
        flat_pq[2] = _mm_and_si128(flat, flat_pq[2]);
        *q2p2 = _mm_or_si128(*q2p2, flat_pq[2]);
        if 0xffff != _mm_movemask_epi8(_mm_cmpeq_epi8(flat2, zero)) {
            flat2_p[0] = _mm_add_epi16(sum_p_0, _mm_add_epi16(work0_0, q0_16));
            flat2_q[0] = _mm_add_epi16(
                sum_p_0,
                _mm_add_epi16(_mm_srli_si128::<8>(work0_0), pq_16[0]),
            );
            flat2_p[1] = _mm_add_epi16(sum_p, work0_1);
            flat2_q[1] = _mm_add_epi16(sum_q, _mm_srli_si128::<8>(work0_1));
            flat2_pq[0] = _mm_srli_epi16::<{ 4 }>(_mm_unpacklo_epi64(flat2_p[0], flat2_q[0]));
            flat2_pq[1] = _mm_srli_epi16::<{ 4 }>(_mm_unpacklo_epi64(flat2_p[1], flat2_q[1]));
            flat2_pq[0] = _mm_packus_epi16(flat2_pq[0], flat2_pq[0]);
            flat2_pq[1] = _mm_packus_epi16(flat2_pq[1], flat2_pq[1]);
            sum_p = _mm_sub_epi16(sum_p, q4_16);
            sum_q = _mm_sub_epi16(sum_q, pq_16[4]);
            sum_p6 = _mm_add_epi16(sum_p6, pq_16[6]);
            work0 = _mm_add_epi16(
                sum_p6,
                _mm_add_epi16(pq_16[2], _mm_add_epi16(pq_16[3], pq_16[1])),
            );
            flat2_p[2] = _mm_add_epi16(sum_p, work0);
            flat2_q[2] = _mm_add_epi16(sum_q, _mm_srli_si128::<8>(work0));
            flat2_pq[2] = _mm_srli_epi16::<{ 4 }>(_mm_unpacklo_epi64(flat2_p[2], flat2_q[2]));
            flat2_pq[2] = _mm_packus_epi16(flat2_pq[2], flat2_pq[2]);
            sum_p6 = _mm_add_epi16(sum_p6, pq_16[6]);
            sum_p = _mm_sub_epi16(sum_p, q3_16);
            sum_q = _mm_sub_epi16(sum_q, pq_16[3]);
            work0 = _mm_add_epi16(
                sum_p6,
                _mm_add_epi16(pq_16[3], _mm_add_epi16(pq_16[4], pq_16[2])),
            );
            flat2_p[3] = _mm_add_epi16(sum_p, work0);
            flat2_q[3] = _mm_add_epi16(sum_q, _mm_srli_si128::<8>(work0));
            flat2_pq[3] = _mm_srli_epi16::<{ 4 }>(_mm_unpacklo_epi64(flat2_p[3], flat2_q[3]));
            flat2_pq[3] = _mm_packus_epi16(flat2_pq[3], flat2_pq[3]);
            sum_p6 = _mm_add_epi16(sum_p6, pq_16[6]);
            sum_p = _mm_sub_epi16(sum_p, q2_16);
            sum_q = _mm_sub_epi16(sum_q, pq_16[2]);
            work0 = _mm_add_epi16(
                sum_p6,
                _mm_add_epi16(pq_16[4], _mm_add_epi16(pq_16[5], pq_16[3])),
            );
            flat2_p[4] = _mm_add_epi16(sum_p, work0);
            flat2_q[4] = _mm_add_epi16(sum_q, _mm_srli_si128::<8>(work0));
            flat2_pq[4] = _mm_srli_epi16::<{ 4 }>(_mm_unpacklo_epi64(flat2_p[4], flat2_q[4]));
            flat2_pq[4] = _mm_packus_epi16(flat2_pq[4], flat2_pq[4]);
            sum_p6 = _mm_add_epi16(sum_p6, pq_16[6]);
            sum_p = _mm_sub_epi16(sum_p, q1_16);
            sum_q = _mm_sub_epi16(sum_q, pq_16[1]);
            work0 = _mm_add_epi16(
                sum_p6,
                _mm_add_epi16(pq_16[5], _mm_add_epi16(pq_16[6], pq_16[4])),
            );
            flat2_p[5] = _mm_add_epi16(sum_p, work0);
            flat2_q[5] = _mm_add_epi16(sum_q, _mm_srli_si128::<8>(work0));
            flat2_pq[5] = _mm_srli_epi16::<{ 4 }>(_mm_unpacklo_epi64(flat2_p[5], flat2_q[5]));
            flat2_pq[5] = _mm_packus_epi16(flat2_pq[5], flat2_pq[5]);
            // wide flat
            // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            *q0p0 = _mm_andnot_si128(flat2, *q0p0);
            flat2_pq[0] = _mm_and_si128(flat2, flat2_pq[0]);
            *q0p0 = _mm_or_si128(*q0p0, flat2_pq[0]);
            *q1p1 = _mm_andnot_si128(flat2, *q1p1);
            flat2_pq[1] = _mm_and_si128(flat2, flat2_pq[1]);
            *q1p1 = _mm_or_si128(*q1p1, flat2_pq[1]);
            *q2p2 = _mm_andnot_si128(flat2, *q2p2);
            flat2_pq[2] = _mm_and_si128(flat2, flat2_pq[2]);
            *q2p2 = _mm_or_si128(*q2p2, flat2_pq[2]);
            *q3p3 = _mm_andnot_si128(flat2, *q3p3);
            flat2_pq[3] = _mm_and_si128(flat2, flat2_pq[3]);
            *q3p3 = _mm_or_si128(*q3p3, flat2_pq[3]);
            *q4p4 = _mm_andnot_si128(flat2, *q4p4);
            flat2_pq[4] = _mm_and_si128(flat2, flat2_pq[4]);
            *q4p4 = _mm_or_si128(*q4p4, flat2_pq[4]);
            *q5p5 = _mm_andnot_si128(flat2, *q5p5);
            flat2_pq[5] = _mm_and_si128(flat2, flat2_pq[5]);
            *q5p5 = _mm_or_si128(*q5p5, flat2_pq[5]);
        }
    } else {
        *q0p0 = qs0ps0;
        *q1p1 = qs1ps1;
    }
}

#[no_mangle]
#[target_feature(enable = "sse2")]
pub unsafe extern "C-unwind" fn aom_lpf_horizontal_14_sse2(
    s: *mut u8,
    p: i32,
    _blimit: *const u8,
    _limit: *const u8,
    _thresh: *const u8,
) {
    let mut q6p6: __m128i = core::mem::zeroed();
    let mut q5p5: __m128i = core::mem::zeroed();
    let mut q4p4: __m128i = core::mem::zeroed();
    let mut q3p3: __m128i = core::mem::zeroed();
    let mut q2p2: __m128i = core::mem::zeroed();
    let mut q1p1: __m128i = core::mem::zeroed();
    let mut q0p0: __m128i = core::mem::zeroed();
    let mut blimit: __m128i = _mm_load_si128(_blimit as *const __m128i);
    let mut limit: __m128i = _mm_load_si128(_limit as *const __m128i);
    let mut thresh: __m128i = _mm_load_si128(_thresh as *const __m128i);
    q4p4 = _mm_unpacklo_epi32(
        xx_loadl_32(s.offset(-((5 * p) as isize)) as *const core::ffi::c_void),
        xx_loadl_32(s.offset((4 * p) as isize) as *const core::ffi::c_void),
    );
    q3p3 = _mm_unpacklo_epi32(
        xx_loadl_32(s.offset(-((4 * p) as isize)) as *const core::ffi::c_void),
        xx_loadl_32(s.offset((3 * p) as isize) as *const core::ffi::c_void),
    );
    q2p2 = _mm_unpacklo_epi32(
        xx_loadl_32(s.offset(-((3 * p) as isize)) as *const core::ffi::c_void),
        xx_loadl_32(s.offset((2 * p) as isize) as *const core::ffi::c_void),
    );
    q1p1 = _mm_unpacklo_epi32(
        xx_loadl_32(s.offset(-((2 * p) as isize)) as *const core::ffi::c_void),
        xx_loadl_32(s.offset((1 * p) as isize) as *const core::ffi::c_void),
    );
    q0p0 = _mm_unpacklo_epi32(
        xx_loadl_32(s.offset(-((1 * p) as isize)) as *const core::ffi::c_void),
        xx_loadl_32(s.offset(-((0 * p) as isize)) as *const core::ffi::c_void),
    );
    q5p5 = _mm_unpacklo_epi32(
        xx_loadl_32(s.offset(-((6 * p) as isize)) as *const core::ffi::c_void),
        xx_loadl_32(s.offset((5 * p) as isize) as *const core::ffi::c_void),
    );
    q6p6 = _mm_unpacklo_epi32(
        xx_loadl_32(s.offset(-((7 * p) as isize)) as *const core::ffi::c_void),
        xx_loadl_32(s.offset((6 * p) as isize) as *const core::ffi::c_void),
    );
    lpf_internal_14_sse2(
        &raw mut q6p6,
        &raw mut q5p5,
        &raw mut q4p4,
        &raw mut q3p3,
        &raw mut q2p2,
        &raw mut q1p1,
        &raw mut q0p0,
        &raw mut blimit,
        &raw mut limit,
        &raw mut thresh,
    );
    store_buffer_horz_8(q0p0, p, 0, s);
    store_buffer_horz_8(q1p1, p, 1, s);
    store_buffer_horz_8(q2p2, p, 2, s);
    store_buffer_horz_8(q3p3, p, 3, s);
    store_buffer_horz_8(q4p4, p, 4, s);
    store_buffer_horz_8(q5p5, p, 5, s);
}

#[inline]
#[target_feature(enable = "sse2")]
unsafe fn lpf_internal_6_dual_sse2(
    p2: *mut __m128i,
    q2: *mut __m128i,
    p1: *mut __m128i,
    q1: *mut __m128i,
    p0: *mut __m128i,
    q0: *mut __m128i,
    q1q0: *mut __m128i,
    p1p0: *mut __m128i,
    blimit: *mut __m128i,
    limit: *mut __m128i,
    thresh: *mut __m128i,
) {
    let zero: __m128i = _mm_setzero_si128();
    let mut mask: __m128i = core::mem::zeroed();
    let mut hev: __m128i = core::mem::zeroed();
    let mut flat: __m128i;
    let mut q2p2: __m128i;
    let mut q1p1: __m128i;
    let mut q0p0: __m128i;
    let mut flat_p1p0: __m128i;
    let mut flat_q0q1: __m128i;
    let mut p2_16: __m128i;
    let mut q2_16: __m128i;
    let mut p1_16: __m128i;
    let mut q1_16: __m128i;
    let mut p0_16: __m128i;
    let mut q0_16: __m128i;
    let mut ps1ps0: __m128i;
    let mut qs1qs0: __m128i;
    q2p2 = _mm_unpacklo_epi64(*p2, *q2);
    q1p1 = _mm_unpacklo_epi64(*p1, *q1);
    q0p0 = _mm_unpacklo_epi64(*p0, *q0);
    *p1p0 = _mm_unpacklo_epi64(q0p0, q1p1);
    *q1q0 = _mm_unpackhi_epi64(q0p0, q1p1);
    let one: __m128i = _mm_set1_epi8(1);
    let fe: __m128i = _mm_set1_epi8(0xfe_i32 as i8);
    let ff: __m128i = _mm_cmpeq_epi8(fe, fe);
    {
        // filter_mask and hev_mask
        let mut abs_p1q1: __m128i;
        let mut abs_p0q0: __m128i;
        let mut abs_q1q0: __m128i;
        let mut abs_p1p0: __m128i;
        let mut work: __m128i;
        abs_p1p0 = abs_diff(q1p1, q0p0);
        abs_q1q0 = _mm_srli_si128::<8>(abs_p1p0);
        abs_p0q0 = abs_diff(*p1p0, *q1q0);
        abs_p1q1 = _mm_srli_si128::<8>(abs_p0q0);
        abs_p0q0 = _mm_unpacklo_epi64(abs_p0q0, zero);
        // considering sse doesn't have unsigned elements comparison the idea is
        // to find at least one case when X > limit, it means the corresponding
        // mask bit is set.
        // to achieve that we find global max value of all inputs of abs(x-y) or
        // (abs(p0 - q0) * 2 + abs(p1 - q1) / 2 If it is > limit the mask is set
        // otherwise - not
        flat = _mm_max_epu8(abs_p1p0, abs_q1q0);
        hev = _mm_subs_epu8(flat, *thresh);
        hev = _mm_xor_si128(_mm_cmpeq_epi8(hev, zero), ff);
        // replicate for the further "merged variables" usage
        hev = _mm_unpacklo_epi64(hev, hev);
        abs_p0q0 = _mm_adds_epu8(abs_p0q0, abs_p0q0);
        abs_p1q1 = _mm_srli_epi16::<{ 1 }>(_mm_and_si128(abs_p1q1, fe));
        mask = _mm_subs_epu8(_mm_adds_epu8(abs_p0q0, abs_p1q1), *blimit);
        mask = _mm_xor_si128(_mm_cmpeq_epi8(mask, zero), ff);
        // mask |= (abs(p0 - q0) * 2 + abs(p1 - q1) / 2  > blimit) * -1;
        mask = _mm_max_epu8(abs_p1p0, mask);
        // mask |= (abs(p1 - p0) > limit) * -1;
        // mask |= (abs(q1 - q0) > limit) * -1;
        work = abs_diff(q2p2, q1p1);
        mask = _mm_max_epu8(work, mask);
        mask = _mm_max_epu8(mask, _mm_srli_si128::<8>(mask));
        mask = _mm_subs_epu8(mask, *limit);
        mask = _mm_cmpeq_epi8(mask, zero);
        // lp filter - the same for 6, 8 and 14 versions
        filter4_dual_sse2(p1p0, q1q0, &raw mut hev, &raw mut mask, q1q0, p1p0);
        // flat_mask
        flat = _mm_max_epu8(abs_diff(q2p2, q0p0), abs_p1p0);
        flat = _mm_max_epu8(flat, _mm_srli_si128::<8>(flat));
        flat = _mm_subs_epu8(flat, one);
        flat = _mm_cmpeq_epi8(flat, zero);
        flat = _mm_and_si128(flat, mask);
        // replicate for the further "merged variables" usage
        flat = _mm_unpacklo_epi64(flat, flat);
    }
    // 5 tap filter
    // need it only if flat !=0
    if 0xffff != _mm_movemask_epi8(_mm_cmpeq_epi8(flat, zero)) {
        let four: __m128i = _mm_set1_epi16(4);
        let mut workp_a: __m128i;
        let mut workp_b: __m128i;
        let mut workp_shft0: __m128i;
        let mut workp_shft1: __m128i;
        p2_16 = _mm_unpacklo_epi8(*p2, zero);
        p1_16 = _mm_unpacklo_epi8(*p1, zero);
        p0_16 = _mm_unpacklo_epi8(*p0, zero);
        q0_16 = _mm_unpacklo_epi8(*q0, zero);
        q1_16 = _mm_unpacklo_epi8(*q1, zero);
        q2_16 = _mm_unpacklo_epi8(*q2, zero);
        // op1
        workp_a = _mm_add_epi16(_mm_add_epi16(p0_16, p0_16), _mm_add_epi16(p1_16, p1_16)); // p0 *2 + p1 * 2
        workp_a = _mm_add_epi16(_mm_add_epi16(workp_a, four), p2_16); // p2 + p0 * 2 + p1 * 2 + 4
        workp_b = _mm_add_epi16(_mm_add_epi16(p2_16, p2_16), q0_16);
        workp_shft0 = _mm_srli_epi16::<{ 3 }>(_mm_add_epi16(workp_a, workp_b)); // p2 * 3 + p1 * 2 + p0 * 2 + q0 + 4
                                                                                // op0
        workp_b = _mm_add_epi16(_mm_add_epi16(q0_16, q0_16), q1_16); // q0 * 2 + q1
        workp_a = _mm_add_epi16(workp_a, workp_b); // p2 + p0 * 2 + p1 * 2 + q0 * 2 + q1 + 4
        workp_shft1 = _mm_srli_epi16::<{ 3 }>(workp_a);
        flat_p1p0 = _mm_packus_epi16(workp_shft1, workp_shft0);
        // oq0
        workp_a = _mm_sub_epi16(_mm_sub_epi16(workp_a, p2_16), p1_16); // p0 * 2 + p1  + q0 * 2 + q1 + 4
        workp_b = _mm_add_epi16(q1_16, q2_16);
        workp_a = _mm_add_epi16(workp_a, workp_b); // p0 * 2 + p1  + q0 * 2 + q1 * 2 + q2 + 4
        workp_shft0 = _mm_srli_epi16::<{ 3 }>(workp_a);
        // oq1
        workp_a = _mm_sub_epi16(_mm_sub_epi16(workp_a, p1_16), p0_16); // p0   + q0 * 2 + q1 * 2 + q2 + 4
        workp_b = _mm_add_epi16(q2_16, q2_16);
        workp_shft1 = _mm_srli_epi16::<{ 3 }>(_mm_add_epi16(workp_a, workp_b)); // p0  + q0 * 2 + q1 * 2 + q2 * 3 + 4
        flat_q0q1 = _mm_packus_epi16(workp_shft0, workp_shft1);
        qs1qs0 = _mm_andnot_si128(flat, *q1q0);
        *q1q0 = _mm_and_si128(flat, flat_q0q1);
        *q1q0 = _mm_or_si128(qs1qs0, *q1q0);
        ps1ps0 = _mm_andnot_si128(flat, *p1p0);
        *p1p0 = _mm_and_si128(flat, flat_p1p0);
        *p1p0 = _mm_or_si128(ps1ps0, *p1p0);
    }
}

#[inline]
#[target_feature(enable = "sse2")]
unsafe fn lpf_internal_6_sse2(
    p2: *mut __m128i,
    q2: *mut __m128i,
    p1: *mut __m128i,
    q1: *mut __m128i,
    p0: *mut __m128i,
    q0: *mut __m128i,
    q1q0: *mut __m128i,
    p1p0: *mut __m128i,
    blimit: *mut __m128i,
    limit: *mut __m128i,
    thresh: *mut __m128i,
) {
    let zero: __m128i = _mm_setzero_si128();
    let mut mask: __m128i = core::mem::zeroed();
    let mut hev: __m128i = core::mem::zeroed();
    let mut flat: __m128i;
    let mut q2p2: __m128i;
    let mut q1p1: __m128i;
    let mut q0p0: __m128i;
    let mut flat_p1p0: __m128i;
    let mut flat_q0q1: __m128i;
    let mut pq2_16: __m128i;
    let mut q2_16: __m128i;
    let mut pq1_16: __m128i;
    let mut pq0_16: __m128i;
    let mut q0_16: __m128i;
    let mut ps1ps0: __m128i;
    let mut qs1qs0: __m128i;
    q2p2 = _mm_unpacklo_epi32(*p2, *q2);
    q1p1 = _mm_unpacklo_epi32(*p1, *q1);
    q0p0 = _mm_unpacklo_epi32(*p0, *q0);
    *p1p0 = _mm_unpacklo_epi32(*p0, *p1);
    *q1q0 = _mm_unpacklo_epi32(*q0, *q1);
    let one: __m128i = _mm_set1_epi8(1);
    let fe: __m128i = _mm_set1_epi8(0xfe_i32 as i8);
    let ff: __m128i = _mm_cmpeq_epi8(fe, fe);
    {
        // filter_mask and hev_mask
        let mut abs_p1q1: __m128i;
        let mut abs_p0q0: __m128i;
        let mut abs_q1q0: __m128i;
        let mut abs_p1p0: __m128i;
        let mut work: __m128i;
        abs_p1p0 = abs_diff(q1p1, q0p0);
        abs_q1q0 = _mm_srli_si128::<4>(abs_p1p0);
        abs_p0q0 = abs_diff(*p1p0, *q1q0);
        abs_p1q1 = _mm_srli_si128::<4>(abs_p0q0);
        // considering sse doesn't have unsigned elements comparison the idea is
        // to find at least one case when X > limit, it means the corresponding
        // mask bit is set.
        // to achieve that we find global max value of all inputs of abs(x-y) or
        // (abs(p0 - q0) * 2 + abs(p1 - q1) / 2 If it is > limit the mask is set
        // otherwise - not
        flat = _mm_max_epu8(abs_p1p0, abs_q1q0);
        hev = _mm_subs_epu8(flat, *thresh);
        hev = _mm_xor_si128(_mm_cmpeq_epi8(hev, zero), ff);
        // replicate for the further "merged variables" usage
        hev = _mm_unpacklo_epi32(hev, hev);
        abs_p0q0 = _mm_adds_epu8(abs_p0q0, abs_p0q0);
        abs_p1q1 = _mm_srli_epi16::<{ 1 }>(_mm_and_si128(abs_p1q1, fe));
        mask = _mm_subs_epu8(_mm_adds_epu8(abs_p0q0, abs_p1q1), *blimit);
        mask = _mm_unpacklo_epi32(mask, zero);
        mask = _mm_xor_si128(_mm_cmpeq_epi8(mask, zero), ff);
        // mask |= (abs(p0 - q0) * 2 + abs(p1 - q1) / 2  > blimit) * -1;
        mask = _mm_max_epu8(abs_p1p0, mask);
        // mask |= (abs(p1 - p0) > limit) * -1;
        // mask |= (abs(q1 - q0) > limit) * -1;
        work = abs_diff(q2p2, q1p1);
        mask = _mm_max_epu8(work, mask);
        mask = _mm_max_epu8(mask, _mm_srli_si128::<4>(mask));
        mask = _mm_subs_epu8(mask, *limit);
        mask = _mm_cmpeq_epi8(mask, zero);
        // lp filter - the same for 6, 8 and 14 versions
        filter4_sse2(p1p0, q1q0, &raw mut hev, &raw mut mask, q1q0, p1p0);
        // flat_mask
        flat = _mm_max_epu8(abs_diff(q2p2, q0p0), abs_p1p0);
        flat = _mm_max_epu8(flat, _mm_srli_si128::<4>(flat));
        flat = _mm_subs_epu8(flat, one);
        flat = _mm_cmpeq_epi8(flat, zero);
        flat = _mm_and_si128(flat, mask);
        // replicate for the further "merged variables" usage
        flat = _mm_unpacklo_epi32(flat, flat);
        flat = _mm_unpacklo_epi64(flat, flat);
    }
    // 5 tap filter
    // need it only if flat !=0
    if 0xffff != _mm_movemask_epi8(_mm_cmpeq_epi8(flat, zero)) {
        let four: __m128i = _mm_set1_epi16(4);
        let mut workp_a: __m128i;
        let mut workp_b: __m128i;
        let mut workp_c: __m128i;
        let mut pq0x2_pq1: __m128i;
        let mut pq1_pq2: __m128i;
        pq2_16 = _mm_unpacklo_epi8(q2p2, zero);
        pq1_16 = _mm_unpacklo_epi8(q1p1, zero);
        pq0_16 = _mm_unpacklo_epi8(q0p0, zero);
        q0_16 = _mm_srli_si128::<8>(pq0_16);
        q2_16 = _mm_srli_si128::<8>(pq2_16);
        // op1
        pq0x2_pq1 = _mm_add_epi16(_mm_add_epi16(pq0_16, pq0_16), pq1_16); // p0 *2 + p1
        pq1_pq2 = _mm_add_epi16(pq1_16, pq2_16); // p1 + p2
        workp_a = _mm_add_epi16(_mm_add_epi16(pq0x2_pq1, four), pq1_pq2); // p2 + p0 * 2 + p1 * 2 + 4
        workp_b = _mm_add_epi16(_mm_add_epi16(pq2_16, pq2_16), q0_16);
        workp_b = _mm_add_epi16(workp_a, workp_b); // p2 * 3 + p1 * 2 + p0 * 2 + q0 + 4
                                                   // op0
        workp_c = _mm_srli_si128::<8>(pq0x2_pq1); // q0 * 2 + q1
        workp_a = _mm_add_epi16(workp_a, workp_c); // p2 + p0 * 2 + p1 * 2 + q0 * 2 + q1 + 4
        workp_b = _mm_unpacklo_epi64(workp_a, workp_b);
        workp_b = _mm_srli_epi16::<{ 3 }>(workp_b);
        flat_p1p0 = _mm_packus_epi16(workp_b, workp_b);
        // oq0
        workp_a = _mm_sub_epi16(_mm_sub_epi16(workp_a, pq2_16), pq1_16); // p0 * 2 + p1  + q0 * 2 + q1 + 4
        workp_b = _mm_srli_si128::<8>(pq1_pq2);
        workp_a = _mm_add_epi16(workp_a, workp_b); // p0 * 2 + p1  + q0 * 2 + q1 * 2 + q2 + 4
                                                   // workp_shft0 = _mm_srli_epi16(workp_a, 3);
                                                   // oq1
        workp_c = _mm_sub_epi16(_mm_sub_epi16(workp_a, pq1_16), pq0_16); // p0   + q0 * 2 + q1 * 2 + q2 + 4
        workp_b = _mm_add_epi16(q2_16, q2_16);
        workp_b = _mm_add_epi16(workp_c, workp_b); // p0  + q0 * 2 + q1 * 2 + q2 * 3 + 4
        workp_a = _mm_unpacklo_epi64(workp_a, workp_b);
        workp_a = _mm_srli_epi16::<{ 3 }>(workp_a);
        flat_q0q1 = _mm_packus_epi16(workp_a, workp_a);
        qs1qs0 = _mm_andnot_si128(flat, *q1q0);
        *q1q0 = _mm_and_si128(flat, flat_q0q1);
        *q1q0 = _mm_or_si128(qs1qs0, *q1q0);
        ps1ps0 = _mm_andnot_si128(flat, *p1p0);
        *p1p0 = _mm_and_si128(flat, flat_p1p0);
        *p1p0 = _mm_or_si128(ps1ps0, *p1p0);
    }
}

#[no_mangle]
#[target_feature(enable = "sse2")]
pub unsafe extern "C-unwind" fn aom_lpf_horizontal_6_sse2(
    s: *mut u8,
    p: i32,
    _blimit: *const u8,
    _limit: *const u8,
    _thresh: *const u8,
) {
    let mut p2: __m128i = core::mem::zeroed();
    let mut p1: __m128i = core::mem::zeroed();
    let mut p0: __m128i = core::mem::zeroed();
    let mut q0: __m128i = core::mem::zeroed();
    let mut q1: __m128i = core::mem::zeroed();
    let mut q2: __m128i = core::mem::zeroed();
    let mut p1p0: __m128i = core::mem::zeroed();
    let mut q1q0: __m128i = core::mem::zeroed();
    let mut blimit: __m128i = _mm_load_si128(_blimit as *mut __m128i as *const __m128i);
    let mut limit: __m128i = _mm_load_si128(_limit as *mut __m128i as *const __m128i);
    let mut thresh: __m128i = _mm_load_si128(_thresh as *mut __m128i as *const __m128i);
    p2 = xx_loadl_32(s.offset(-((3 * p) as isize)) as *const core::ffi::c_void);
    p1 = xx_loadl_32(s.offset(-((2 * p) as isize)) as *const core::ffi::c_void);
    p0 = xx_loadl_32(s.offset(-((1 * p) as isize)) as *const core::ffi::c_void);
    q0 = xx_loadl_32(s.offset(-((0 * p) as isize)) as *const core::ffi::c_void);
    q1 = xx_loadl_32(s.offset((1 * p) as isize) as *const core::ffi::c_void);
    q2 = xx_loadl_32(s.offset((2 * p) as isize) as *const core::ffi::c_void);
    lpf_internal_6_sse2(
        &raw mut p2,
        &raw mut q2,
        &raw mut p1,
        &raw mut q1,
        &raw mut p0,
        &raw mut q0,
        &raw mut q1q0,
        &raw mut p1p0,
        &raw mut blimit,
        &raw mut limit,
        &raw mut thresh,
    );
    xx_storel_32(
        s.offset(-((1 * p) as isize)) as *mut core::ffi::c_void,
        p1p0,
    );
    xx_storel_32(
        s.offset(-((2 * p) as isize)) as *mut core::ffi::c_void,
        _mm_srli_si128::<4>(p1p0),
    );
    xx_storel_32(s.offset((0 * p) as isize) as *mut core::ffi::c_void, q1q0);
    xx_storel_32(
        s.offset((1 * p) as isize) as *mut core::ffi::c_void,
        _mm_srli_si128::<4>(q1q0),
    );
}

#[no_mangle]
#[target_feature(enable = "sse2")]
pub unsafe extern "C-unwind" fn aom_lpf_horizontal_6_dual_sse2(
    s: *mut u8,
    p: i32,
    _blimit0: *const u8,
    _limit0: *const u8,
    _thresh0: *const u8,
    _blimit1: *const u8,
    _limit1: *const u8,
    _thresh1: *const u8,
) {
    let mut blimit: __m128i = _mm_unpacklo_epi32(
        _mm_load_si128(_blimit0 as *mut __m128i as *const __m128i),
        _mm_load_si128(_blimit1 as *mut __m128i as *const __m128i),
    );
    let mut limit: __m128i = _mm_unpacklo_epi32(
        _mm_load_si128(_limit0 as *mut __m128i as *const __m128i),
        _mm_load_si128(_limit1 as *mut __m128i as *const __m128i),
    );
    let mut thresh: __m128i = _mm_unpacklo_epi32(
        _mm_load_si128(_thresh0 as *mut __m128i as *const __m128i),
        _mm_load_si128(_thresh1 as *mut __m128i as *const __m128i),
    );
    let mut p2: __m128i = core::mem::zeroed();
    let mut p1: __m128i = core::mem::zeroed();
    let mut p0: __m128i = core::mem::zeroed();
    let mut q0: __m128i = core::mem::zeroed();
    let mut q1: __m128i = core::mem::zeroed();
    let mut q2: __m128i = core::mem::zeroed();
    let mut p1p0: __m128i = core::mem::zeroed();
    let mut q1q0: __m128i = core::mem::zeroed();
    p2 = _mm_loadl_epi64(s.offset(-((3 * p) as isize)) as *mut __m128i as *const __m128i);
    p1 = _mm_loadl_epi64(s.offset(-((2 * p) as isize)) as *mut __m128i as *const __m128i);
    p0 = _mm_loadl_epi64(s.offset(-((1 * p) as isize)) as *mut __m128i as *const __m128i);
    q0 = _mm_loadl_epi64(s.offset(-((0 * p) as isize)) as *mut __m128i as *const __m128i);
    q1 = _mm_loadl_epi64(s.offset((1 * p) as isize) as *mut __m128i as *const __m128i);
    q2 = _mm_loadl_epi64(s.offset((2 * p) as isize) as *mut __m128i as *const __m128i);
    lpf_internal_6_dual_sse2(
        &raw mut p2,
        &raw mut q2,
        &raw mut p1,
        &raw mut q1,
        &raw mut p0,
        &raw mut q0,
        &raw mut q1q0,
        &raw mut p1p0,
        &raw mut blimit,
        &raw mut limit,
        &raw mut thresh,
    );
    _mm_storel_epi64(s.offset(-((1 * p) as isize)) as *mut __m128i, p1p0);
    _mm_storel_epi64(
        s.offset(-((2 * p) as isize)) as *mut __m128i,
        _mm_srli_si128::<8>(p1p0),
    );
    _mm_storel_epi64(s.offset((0 * p) as isize) as *mut __m128i, q1q0);
    _mm_storel_epi64(
        s.offset((1 * p) as isize) as *mut __m128i,
        _mm_srli_si128::<8>(q1q0),
    );
}

#[inline]
#[target_feature(enable = "sse2")]
unsafe fn lpf_internal_8_sse2(
    p3: *mut __m128i,
    q3: *mut __m128i,
    p2: *mut __m128i,
    q2: *mut __m128i,
    p1: *mut __m128i,
    q1: *mut __m128i,
    p0: *mut __m128i,
    q0: *mut __m128i,
    q1q0_out: *mut __m128i,
    p1p0_out: *mut __m128i,
    blimit: *mut __m128i,
    limit: *mut __m128i,
    thresh: *mut __m128i,
) {
    let zero: __m128i = _mm_setzero_si128();
    let mut mask: __m128i = core::mem::zeroed();
    let mut hev: __m128i = core::mem::zeroed();
    let mut flat: __m128i;
    let mut p2_16: __m128i;
    let mut q2_16: __m128i;
    let mut p1_16: __m128i;
    let mut p0_16: __m128i;
    let mut q0_16: __m128i;
    let mut q1_16: __m128i;
    let mut p3_16: __m128i;
    let mut q3_16: __m128i;
    let mut q3p3: __m128i;
    let mut flat_p1p0: __m128i;
    let mut flat_q0q1: __m128i;
    let mut q2p2: __m128i;
    let mut q1p1: __m128i;
    let mut q0p0: __m128i;
    let mut q1q0: __m128i = core::mem::zeroed();
    let mut p1p0: __m128i = core::mem::zeroed();
    let mut ps1ps0: __m128i;
    let mut qs1qs0: __m128i;
    let mut work_pq: __m128i;
    let mut opq2: __m128i;
    let mut pq2: __m128i;
    q3p3 = _mm_unpacklo_epi32(*p3, *q3);
    q2p2 = _mm_unpacklo_epi32(*p2, *q2);
    q1p1 = _mm_unpacklo_epi32(*p1, *q1);
    q0p0 = _mm_unpacklo_epi32(*p0, *q0);
    p1p0 = _mm_unpacklo_epi32(q0p0, q1p1); // p1p0 q1q0
    q1q0 = _mm_srli_si128::<8>(p1p0);
    // filter_mask and hev_mask
    // considering sse doesn't have unsigned elements comparison the idea is to
    // find at least one case when X > limit, it means the corresponding  mask
    // bit is set.
    // to achieve that we find global max value of all inputs of abs(x-y) or
    // (abs(p0 - q0) * 2 + abs(p1 - q1) / 2 If it is > limit the mask is set
    // otherwise - not
    let one: __m128i = _mm_set1_epi8(1);
    let fe: __m128i = _mm_set1_epi8(0xfe_i32 as i8);
    let ff: __m128i = _mm_cmpeq_epi8(fe, fe);
    let mut abs_p1q1: __m128i;
    let mut abs_p0q0: __m128i;
    let mut abs_q1q0: __m128i;
    let mut abs_p1p0: __m128i;
    let mut work: __m128i;
    abs_p1p0 = abs_diff(q1p1, q0p0);
    abs_q1q0 = _mm_srli_si128::<4>(abs_p1p0);
    abs_p0q0 = abs_diff(p1p0, q1q0);
    abs_p1q1 = _mm_srli_si128::<4>(abs_p0q0);
    flat = _mm_max_epu8(abs_p1p0, abs_q1q0);
    hev = _mm_subs_epu8(flat, *thresh);
    hev = _mm_xor_si128(_mm_cmpeq_epi8(hev, zero), ff);
    // replicate for the further "merged variables" usage
    hev = _mm_unpacklo_epi32(hev, hev);
    abs_p0q0 = _mm_adds_epu8(abs_p0q0, abs_p0q0);
    abs_p1q1 = _mm_srli_epi16::<{ 1 }>(_mm_and_si128(abs_p1q1, fe));
    mask = _mm_subs_epu8(_mm_adds_epu8(abs_p0q0, abs_p1q1), *blimit);
    mask = _mm_unpacklo_epi32(mask, zero);
    mask = _mm_xor_si128(_mm_cmpeq_epi8(mask, zero), ff);
    // mask |= (abs(p0 - q0) * 2 + abs(p1 - q1) / 2  > blimit) * -1;
    mask = _mm_max_epu8(abs_p1p0, mask);
    // mask |= (abs(p1 - p0) > limit) * -1;
    // mask |= (abs(q1 - q0) > limit) * -1;
    work = _mm_max_epu8(abs_diff(q2p2, q1p1), abs_diff(q3p3, q2p2));
    mask = _mm_max_epu8(work, mask);
    mask = _mm_max_epu8(mask, _mm_srli_si128::<4>(mask));
    mask = _mm_subs_epu8(mask, *limit);
    mask = _mm_cmpeq_epi8(mask, zero);
    // lp filter - the same for 6, 8 and 14 versions
    filter4_sse2(
        &raw mut p1p0,
        &raw mut q1q0,
        &raw mut hev,
        &raw mut mask,
        q1q0_out,
        p1p0_out,
    );
    // flat_mask4
    flat = _mm_max_epu8(abs_diff(q2p2, q0p0), abs_diff(q3p3, q0p0));
    flat = _mm_max_epu8(abs_p1p0, flat);
    flat = _mm_max_epu8(flat, _mm_srli_si128::<4>(flat));
    flat = _mm_subs_epu8(flat, one);
    flat = _mm_cmpeq_epi8(flat, zero);
    flat = _mm_and_si128(flat, mask);
    // replicate for the further "merged variables" usage
    flat = _mm_unpacklo_epi32(flat, flat);
    flat = _mm_unpacklo_epi64(flat, flat);
    // filter8 need it only if flat !=0
    if 0xffff != _mm_movemask_epi8(_mm_cmpeq_epi8(flat, zero)) {
        let four: __m128i = _mm_set1_epi16(4);
        let mut workp_a: __m128i;
        let mut workp_b: __m128i;
        let mut workp_c: __m128i;
        let mut workp_d: __m128i;
        let mut workp_shft1: __m128i;
        let mut workp_shft2: __m128i;
        p2_16 = _mm_unpacklo_epi8(*p2, zero);
        p1_16 = _mm_unpacklo_epi8(*p1, zero);
        p0_16 = _mm_unpacklo_epi8(*p0, zero);
        q0_16 = _mm_unpacklo_epi8(*q0, zero);
        q1_16 = _mm_unpacklo_epi8(*q1, zero);
        q2_16 = _mm_unpacklo_epi8(*q2, zero);
        p3_16 = _mm_unpacklo_epi8(*p3, zero);
        q3_16 = _mm_unpacklo_epi8(*q3, zero);
        // op2
        workp_a = _mm_add_epi16(_mm_add_epi16(p3_16, p3_16), _mm_add_epi16(p2_16, p1_16));
        workp_a = _mm_add_epi16(_mm_add_epi16(workp_a, four), p0_16);
        workp_b = _mm_add_epi16(_mm_add_epi16(q0_16, p2_16), p3_16);
        workp_shft2 = _mm_add_epi16(workp_a, workp_b);
        // op1
        workp_b = _mm_add_epi16(_mm_add_epi16(q0_16, q1_16), p1_16);
        workp_c = _mm_add_epi16(workp_a, workp_b);
        // workp_shft0 = _mm_srli_epi16(_mm_add_epi16(workp_a, workp_b), 3);
        // op0
        workp_a = _mm_add_epi16(_mm_sub_epi16(workp_a, p3_16), q2_16);
        workp_b = _mm_add_epi16(_mm_sub_epi16(workp_b, p1_16), p0_16);
        workp_d = _mm_add_epi16(workp_a, workp_b);
        // workp_shft1 = _mm_srli_epi16(_mm_add_epi16(workp_a, workp_b), 3);
        workp_c = _mm_unpacklo_epi64(workp_d, workp_c);
        workp_c = _mm_srli_epi16::<{ 3 }>(workp_c);
        flat_p1p0 = _mm_packus_epi16(workp_c, workp_c);
        // oq0
        workp_a = _mm_add_epi16(_mm_sub_epi16(workp_a, p3_16), q3_16);
        workp_b = _mm_add_epi16(_mm_sub_epi16(workp_b, p0_16), q0_16);
        // workp_shft0 = _mm_srli_epi16(_mm_add_epi16(workp_a, workp_b), 3);
        workp_c = _mm_add_epi16(workp_a, workp_b);
        // oq1
        workp_a = _mm_add_epi16(_mm_sub_epi16(workp_a, p2_16), q3_16);
        workp_b = _mm_add_epi16(_mm_sub_epi16(workp_b, q0_16), q1_16);
        workp_d = _mm_add_epi16(workp_a, workp_b);
        // workp_shft1 = _mm_srli_epi16(_mm_add_epi16(workp_a, workp_b), 3);
        workp_c = _mm_unpacklo_epi64(workp_c, workp_d);
        workp_c = _mm_srli_epi16::<{ 3 }>(workp_c);
        flat_q0q1 = _mm_packus_epi16(workp_c, workp_c);
        // oq2
        workp_a = _mm_add_epi16(_mm_sub_epi16(workp_a, p1_16), q3_16);
        workp_b = _mm_add_epi16(_mm_sub_epi16(workp_b, q1_16), q2_16);
        workp_shft1 = _mm_add_epi16(workp_a, workp_b);
        workp_c = _mm_unpacklo_epi64(workp_shft2, workp_shft1);
        workp_c = _mm_srli_epi16::<{ 3 }>(workp_c);
        opq2 = _mm_packus_epi16(workp_c, workp_c);
        work_pq = _mm_andnot_si128(flat, q2p2);
        pq2 = _mm_and_si128(flat, opq2);
        *p2 = _mm_or_si128(work_pq, pq2);
        *q2 = _mm_srli_si128::<4>(*p2);
        qs1qs0 = _mm_andnot_si128(flat, *q1q0_out);
        q1q0 = _mm_and_si128(flat, flat_q0q1);
        *q1q0_out = _mm_or_si128(qs1qs0, q1q0);
        ps1ps0 = _mm_andnot_si128(flat, *p1p0_out);
        p1p0 = _mm_and_si128(flat, flat_p1p0);
        *p1p0_out = _mm_or_si128(ps1ps0, p1p0);
    }
}

#[inline]
#[target_feature(enable = "sse2")]
unsafe fn lpf_internal_8_dual_sse2(
    p3: *mut __m128i,
    q3: *mut __m128i,
    p2: *mut __m128i,
    q2: *mut __m128i,
    p1: *mut __m128i,
    q1: *mut __m128i,
    p0: *mut __m128i,
    q0: *mut __m128i,
    q1q0_out: *mut __m128i,
    p1p0_out: *mut __m128i,
    blimit: *mut __m128i,
    limit: *mut __m128i,
    thresh: *mut __m128i,
) {
    let zero: __m128i = _mm_setzero_si128();
    let mut mask: __m128i = core::mem::zeroed();
    let mut hev: __m128i = core::mem::zeroed();
    let mut flat: __m128i;
    let mut p2_16: __m128i;
    let mut q2_16: __m128i;
    let mut p1_16: __m128i;
    let mut p0_16: __m128i;
    let mut q0_16: __m128i;
    let mut q1_16: __m128i;
    let mut p3_16: __m128i;
    let mut q3_16: __m128i;
    let mut q3p3: __m128i;
    let mut flat_p1p0: __m128i;
    let mut flat_q0q1: __m128i;
    let mut q2p2: __m128i;
    let mut q1p1: __m128i;
    let mut q0p0: __m128i;
    let mut q1q0: __m128i = core::mem::zeroed();
    let mut p1p0: __m128i = core::mem::zeroed();
    let mut ps1ps0: __m128i;
    let mut qs1qs0: __m128i;
    let mut work_pq: __m128i;
    let mut opq2: __m128i;
    let mut pq2: __m128i;
    q3p3 = _mm_unpacklo_epi64(*p3, *q3);
    q2p2 = _mm_unpacklo_epi64(*p2, *q2);
    q1p1 = _mm_unpacklo_epi64(*p1, *q1);
    q0p0 = _mm_unpacklo_epi64(*p0, *q0);
    p1p0 = _mm_unpacklo_epi64(q0p0, q1p1);
    q1q0 = _mm_unpackhi_epi64(q0p0, q1p1);
    {
        // filter_mask and hev_mask
        // considering sse doesn't have unsigned elements comparison the idea is to
        // find at least one case when X > limit, it means the corresponding  mask
        // bit is set.
        // to achieve that we find global max value of all inputs of abs(x-y) or
        // (abs(p0 - q0) * 2 + abs(p1 - q1) / 2 If it is > limit the mask is set
        // otherwise - not
        let one: __m128i = _mm_set1_epi8(1);
        let fe: __m128i = _mm_set1_epi8(0xfe_i32 as i8);
        let ff: __m128i = _mm_cmpeq_epi8(fe, fe);
        let mut abs_p1q1: __m128i;
        let mut abs_p0q0: __m128i;
        let mut abs_q1q0: __m128i;
        let mut abs_p1p0: __m128i;
        let mut work: __m128i;
        abs_p1p0 = abs_diff(q1p1, q0p0);
        abs_q1q0 = _mm_srli_si128::<8>(abs_p1p0);
        abs_p0q0 = abs_diff(p1p0, q1q0);
        abs_p1q1 = _mm_srli_si128::<8>(abs_p0q0);
        abs_p0q0 = _mm_unpacklo_epi64(abs_p0q0, abs_p0q0);
        flat = _mm_max_epu8(abs_p1p0, abs_q1q0);
        hev = _mm_subs_epu8(flat, *thresh);
        hev = _mm_xor_si128(_mm_cmpeq_epi8(hev, zero), ff);
        // replicate for the further "merged variables" usage
        hev = _mm_unpacklo_epi64(hev, hev);
        abs_p0q0 = _mm_adds_epu8(abs_p0q0, abs_p0q0);
        abs_p1q1 = _mm_srli_epi16::<{ 1 }>(_mm_and_si128(abs_p1q1, fe));
        mask = _mm_subs_epu8(_mm_adds_epu8(abs_p0q0, abs_p1q1), *blimit);
        mask = _mm_xor_si128(_mm_cmpeq_epi8(mask, zero), ff);
        // mask |= (abs(p0 - q0) * 2 + abs(p1 - q1) / 2  > blimit) * -1;
        mask = _mm_max_epu8(abs_p1p0, mask);
        // mask |= (abs(p1 - p0) > limit) * -1;
        // mask |= (abs(q1 - q0) > limit) * -1;
        work = _mm_max_epu8(abs_diff(q2p2, q1p1), abs_diff(q3p3, q2p2));
        mask = _mm_max_epu8(work, mask);
        mask = _mm_max_epu8(mask, _mm_srli_si128::<8>(mask));
        mask = _mm_subs_epu8(mask, *limit);
        mask = _mm_cmpeq_epi8(mask, zero);
        // lp filter - the same for 6, 8 and 14 versions
        filter4_dual_sse2(
            &raw mut p1p0,
            &raw mut q1q0,
            &raw mut hev,
            &raw mut mask,
            q1q0_out,
            p1p0_out,
        );
        // flat_mask4
        flat = _mm_max_epu8(abs_diff(q2p2, q0p0), abs_diff(q3p3, q0p0));
        flat = _mm_max_epu8(abs_p1p0, flat);
        flat = _mm_max_epu8(flat, _mm_srli_si128::<8>(flat));
        flat = _mm_subs_epu8(flat, one);
        flat = _mm_cmpeq_epi8(flat, zero);
        flat = _mm_and_si128(flat, mask);
        // replicate for the further "merged variables" usage
        flat = _mm_unpacklo_epi64(flat, flat);
    }
    // filter8 need it only if flat !=0
    if 0xffff != _mm_movemask_epi8(_mm_cmpeq_epi8(flat, zero)) {
        let four: __m128i = _mm_set1_epi16(4);
        let mut workp_a: __m128i;
        let mut workp_b: __m128i;
        let mut workp_shft0: __m128i;
        let mut workp_shft1: __m128i;
        let mut workp_shft2: __m128i;
        p2_16 = _mm_unpacklo_epi8(*p2, zero);
        p1_16 = _mm_unpacklo_epi8(*p1, zero);
        p0_16 = _mm_unpacklo_epi8(*p0, zero);
        q0_16 = _mm_unpacklo_epi8(*q0, zero);
        q1_16 = _mm_unpacklo_epi8(*q1, zero);
        q2_16 = _mm_unpacklo_epi8(*q2, zero);
        p3_16 = _mm_unpacklo_epi8(*p3, zero);
        q3_16 = _mm_unpacklo_epi8(*q3, zero);
        // op2
        workp_a = _mm_add_epi16(_mm_add_epi16(p3_16, p3_16), _mm_add_epi16(p2_16, p1_16));
        workp_a = _mm_add_epi16(_mm_add_epi16(workp_a, four), p0_16);
        workp_b = _mm_add_epi16(_mm_add_epi16(q0_16, p2_16), p3_16);
        workp_shft2 = _mm_srli_epi16::<{ 3 }>(_mm_add_epi16(workp_a, workp_b));
        // op1
        workp_b = _mm_add_epi16(_mm_add_epi16(q0_16, q1_16), p1_16);
        workp_shft0 = _mm_srli_epi16::<{ 3 }>(_mm_add_epi16(workp_a, workp_b));
        // op0
        workp_a = _mm_add_epi16(_mm_sub_epi16(workp_a, p3_16), q2_16);
        workp_b = _mm_add_epi16(_mm_sub_epi16(workp_b, p1_16), p0_16);
        workp_shft1 = _mm_srli_epi16::<{ 3 }>(_mm_add_epi16(workp_a, workp_b));
        flat_p1p0 = _mm_packus_epi16(workp_shft1, workp_shft0);
        // oq0
        workp_a = _mm_add_epi16(_mm_sub_epi16(workp_a, p3_16), q3_16);
        workp_b = _mm_add_epi16(_mm_sub_epi16(workp_b, p0_16), q0_16);
        workp_shft0 = _mm_srli_epi16::<{ 3 }>(_mm_add_epi16(workp_a, workp_b));
        // oq1
        workp_a = _mm_add_epi16(_mm_sub_epi16(workp_a, p2_16), q3_16);
        workp_b = _mm_add_epi16(_mm_sub_epi16(workp_b, q0_16), q1_16);
        workp_shft1 = _mm_srli_epi16::<{ 3 }>(_mm_add_epi16(workp_a, workp_b));
        flat_q0q1 = _mm_packus_epi16(workp_shft0, workp_shft1);
        // oq2
        workp_a = _mm_add_epi16(_mm_sub_epi16(workp_a, p1_16), q3_16);
        workp_b = _mm_add_epi16(_mm_sub_epi16(workp_b, q1_16), q2_16);
        workp_shft1 = _mm_srli_epi16::<{ 3 }>(_mm_add_epi16(workp_a, workp_b));
        opq2 = _mm_packus_epi16(workp_shft2, workp_shft1);
        work_pq = _mm_andnot_si128(flat, q2p2);
        pq2 = _mm_and_si128(flat, opq2);
        *p2 = _mm_or_si128(work_pq, pq2);
        *q2 = _mm_srli_si128::<8>(*p2);
        qs1qs0 = _mm_andnot_si128(flat, *q1q0_out);
        q1q0 = _mm_and_si128(flat, flat_q0q1);
        *q1q0_out = _mm_or_si128(qs1qs0, q1q0);
        ps1ps0 = _mm_andnot_si128(flat, *p1p0_out);
        p1p0 = _mm_and_si128(flat, flat_p1p0);
        *p1p0_out = _mm_or_si128(ps1ps0, p1p0);
    }
}

#[no_mangle]
#[target_feature(enable = "sse2")]
pub unsafe extern "C-unwind" fn aom_lpf_horizontal_8_sse2(
    s: *mut u8,
    p: i32,
    _blimit: *const u8,
    _limit: *const u8,
    _thresh: *const u8,
) {
    let mut p3: __m128i = core::mem::zeroed();
    let mut p2: __m128i = core::mem::zeroed();
    let mut p1: __m128i = core::mem::zeroed();
    let mut p0: __m128i = core::mem::zeroed();
    let mut q0: __m128i = core::mem::zeroed();
    let mut q1: __m128i = core::mem::zeroed();
    let mut q2: __m128i = core::mem::zeroed();
    let mut q3: __m128i = core::mem::zeroed();
    let mut q1q0: __m128i = core::mem::zeroed();
    let mut p1p0: __m128i = core::mem::zeroed();
    let mut blimit: __m128i = _mm_load_si128(_blimit as *const __m128i);
    let mut limit: __m128i = _mm_load_si128(_limit as *const __m128i);
    let mut thresh: __m128i = _mm_load_si128(_thresh as *const __m128i);
    p3 = xx_loadl_32(s.offset(-((4 * p) as isize)) as *const core::ffi::c_void);
    p2 = xx_loadl_32(s.offset(-((3 * p) as isize)) as *const core::ffi::c_void);
    p1 = xx_loadl_32(s.offset(-((2 * p) as isize)) as *const core::ffi::c_void);
    p0 = xx_loadl_32(s.offset(-((1 * p) as isize)) as *const core::ffi::c_void);
    q0 = xx_loadl_32(s.offset(-((0 * p) as isize)) as *const core::ffi::c_void);
    q1 = xx_loadl_32(s.offset((1 * p) as isize) as *const core::ffi::c_void);
    q2 = xx_loadl_32(s.offset((2 * p) as isize) as *const core::ffi::c_void);
    q3 = xx_loadl_32(s.offset((3 * p) as isize) as *const core::ffi::c_void);
    lpf_internal_8_sse2(
        &raw mut p3,
        &raw mut q3,
        &raw mut p2,
        &raw mut q2,
        &raw mut p1,
        &raw mut q1,
        &raw mut p0,
        &raw mut q0,
        &raw mut q1q0,
        &raw mut p1p0,
        &raw mut blimit,
        &raw mut limit,
        &raw mut thresh,
    );
    xx_storel_32(
        s.offset(-((1 * p) as isize)) as *mut core::ffi::c_void,
        p1p0,
    );
    xx_storel_32(
        s.offset(-((2 * p) as isize)) as *mut core::ffi::c_void,
        _mm_srli_si128::<4>(p1p0),
    );
    xx_storel_32(s.offset((0 * p) as isize) as *mut core::ffi::c_void, q1q0);
    xx_storel_32(
        s.offset((1 * p) as isize) as *mut core::ffi::c_void,
        _mm_srli_si128::<4>(q1q0),
    );
    xx_storel_32(s.offset(-((3 * p) as isize)) as *mut core::ffi::c_void, p2);
    xx_storel_32(s.offset((2 * p) as isize) as *mut core::ffi::c_void, q2);
}

#[no_mangle]
#[target_feature(enable = "sse2")]
pub unsafe extern "C-unwind" fn aom_lpf_horizontal_14_dual_sse2(
    s: *mut u8,
    p: i32,
    _blimit0: *const u8,
    _limit0: *const u8,
    _thresh0: *const u8,
    _blimit1: *const u8,
    _limit1: *const u8,
    _thresh1: *const u8,
) {
    let mut q6p6: __m128i = core::mem::zeroed();
    let mut q5p5: __m128i = core::mem::zeroed();
    let mut q4p4: __m128i = core::mem::zeroed();
    let mut q3p3: __m128i = core::mem::zeroed();
    let mut q2p2: __m128i = core::mem::zeroed();
    let mut q1p1: __m128i = core::mem::zeroed();
    let mut q0p0: __m128i = core::mem::zeroed();
    let mut blimit: __m128i = _mm_unpacklo_epi32(
        _mm_load_si128(_blimit0 as *const __m128i),
        _mm_load_si128(_blimit1 as *const __m128i),
    );
    let mut limit: __m128i = _mm_unpacklo_epi32(
        _mm_load_si128(_limit0 as *const __m128i),
        _mm_load_si128(_limit1 as *const __m128i),
    );
    let mut thresh: __m128i = _mm_unpacklo_epi32(
        _mm_load_si128(_thresh0 as *const __m128i),
        _mm_load_si128(_thresh1 as *const __m128i),
    );
    q4p4 = _mm_unpacklo_epi64(
        _mm_loadl_epi64(s.offset(-((5 * p) as isize)) as *mut __m128i as *const __m128i),
        _mm_loadl_epi64(s.offset((4 * p) as isize) as *mut __m128i as *const __m128i),
    );
    q3p3 = _mm_unpacklo_epi64(
        _mm_loadl_epi64(s.offset(-((4 * p) as isize)) as *mut __m128i as *const __m128i),
        _mm_loadl_epi64(s.offset((3 * p) as isize) as *mut __m128i as *const __m128i),
    );
    q2p2 = _mm_unpacklo_epi64(
        _mm_loadl_epi64(s.offset(-((3 * p) as isize)) as *mut __m128i as *const __m128i),
        _mm_loadl_epi64(s.offset((2 * p) as isize) as *mut __m128i as *const __m128i),
    );
    q1p1 = _mm_unpacklo_epi64(
        _mm_loadl_epi64(s.offset(-((2 * p) as isize)) as *mut __m128i as *const __m128i),
        _mm_loadl_epi64(s.offset((1 * p) as isize) as *mut __m128i as *const __m128i),
    );
    q0p0 = _mm_unpacklo_epi64(
        _mm_loadl_epi64(s.offset(-((1 * p) as isize)) as *mut __m128i as *const __m128i),
        _mm_loadl_epi64(s.offset(-((0 * p) as isize)) as *mut __m128i as *const __m128i),
    );
    q5p5 = _mm_unpacklo_epi64(
        _mm_loadl_epi64(s.offset(-((6 * p) as isize)) as *mut __m128i as *const __m128i),
        _mm_loadl_epi64(s.offset((5 * p) as isize) as *mut __m128i as *const __m128i),
    );
    q6p6 = _mm_unpacklo_epi64(
        _mm_loadl_epi64(s.offset(-((7 * p) as isize)) as *mut __m128i as *const __m128i),
        _mm_loadl_epi64(s.offset((6 * p) as isize) as *mut __m128i as *const __m128i),
    );
    lpf_internal_14_dual_sse2(
        &raw mut q6p6,
        &raw mut q5p5,
        &raw mut q4p4,
        &raw mut q3p3,
        &raw mut q2p2,
        &raw mut q1p1,
        &raw mut q0p0,
        &raw mut blimit,
        &raw mut limit,
        &raw mut thresh,
    );
    _mm_storel_epi64(s.offset(-((1 * p) as isize)) as *mut __m128i, q0p0);
    _mm_storel_epi64(
        s.offset((0 * p) as isize) as *mut __m128i,
        _mm_srli_si128::<8>(q0p0),
    );
    _mm_storel_epi64(s.offset(-((2 * p) as isize)) as *mut __m128i, q1p1);
    _mm_storel_epi64(
        s.offset((1 * p) as isize) as *mut __m128i,
        _mm_srli_si128::<8>(q1p1),
    );
    _mm_storel_epi64(s.offset(-((3 * p) as isize)) as *mut __m128i, q2p2);
    _mm_storel_epi64(
        s.offset((2 * p) as isize) as *mut __m128i,
        _mm_srli_si128::<8>(q2p2),
    );
    _mm_storel_epi64(s.offset(-((4 * p) as isize)) as *mut __m128i, q3p3);
    _mm_storel_epi64(
        s.offset((3 * p) as isize) as *mut __m128i,
        _mm_srli_si128::<8>(q3p3),
    );
    _mm_storel_epi64(s.offset(-((5 * p) as isize)) as *mut __m128i, q4p4);
    _mm_storel_epi64(
        s.offset((4 * p) as isize) as *mut __m128i,
        _mm_srli_si128::<8>(q4p4),
    );
    _mm_storel_epi64(s.offset(-((6 * p) as isize)) as *mut __m128i, q5p5);
    _mm_storel_epi64(
        s.offset((5 * p) as isize) as *mut __m128i,
        _mm_srli_si128::<8>(q5p5),
    );
}

#[no_mangle]
#[target_feature(enable = "sse2")]
pub unsafe extern "C-unwind" fn aom_lpf_horizontal_8_dual_sse2(
    s: *mut u8,
    p: i32,
    _blimit0: *const u8,
    _limit0: *const u8,
    _thresh0: *const u8,
    _blimit1: *const u8,
    _limit1: *const u8,
    _thresh1: *const u8,
) {
    let mut blimit: __m128i = _mm_unpacklo_epi32(
        _mm_load_si128(_blimit0 as *mut __m128i as *const __m128i),
        _mm_load_si128(_blimit1 as *mut __m128i as *const __m128i),
    );
    let mut limit: __m128i = _mm_unpacklo_epi32(
        _mm_load_si128(_limit0 as *mut __m128i as *const __m128i),
        _mm_load_si128(_limit1 as *mut __m128i as *const __m128i),
    );
    let mut thresh: __m128i = _mm_unpacklo_epi32(
        _mm_load_si128(_thresh0 as *mut __m128i as *const __m128i),
        _mm_load_si128(_thresh1 as *mut __m128i as *const __m128i),
    );
    let mut p2: __m128i = core::mem::zeroed();
    let mut p1: __m128i = core::mem::zeroed();
    let mut p0: __m128i = core::mem::zeroed();
    let mut q0: __m128i = core::mem::zeroed();
    let mut q1: __m128i = core::mem::zeroed();
    let mut q2: __m128i = core::mem::zeroed();
    let mut p3: __m128i = core::mem::zeroed();
    let mut q3: __m128i = core::mem::zeroed();
    let mut q1q0: __m128i = core::mem::zeroed();
    let mut p1p0: __m128i = core::mem::zeroed();
    p3 = _mm_loadl_epi64(s.offset(-((4 * p) as isize)) as *mut __m128i as *const __m128i);
    p2 = _mm_loadl_epi64(s.offset(-((3 * p) as isize)) as *mut __m128i as *const __m128i);
    p1 = _mm_loadl_epi64(s.offset(-((2 * p) as isize)) as *mut __m128i as *const __m128i);
    p0 = _mm_loadl_epi64(s.offset(-((1 * p) as isize)) as *mut __m128i as *const __m128i);
    q0 = _mm_loadl_epi64(s.offset(-((0 * p) as isize)) as *mut __m128i as *const __m128i);
    q1 = _mm_loadl_epi64(s.offset((1 * p) as isize) as *mut __m128i as *const __m128i);
    q2 = _mm_loadl_epi64(s.offset((2 * p) as isize) as *mut __m128i as *const __m128i);
    q3 = _mm_loadl_epi64(s.offset((3 * p) as isize) as *mut __m128i as *const __m128i);
    lpf_internal_8_dual_sse2(
        &raw mut p3,
        &raw mut q3,
        &raw mut p2,
        &raw mut q2,
        &raw mut p1,
        &raw mut q1,
        &raw mut p0,
        &raw mut q0,
        &raw mut q1q0,
        &raw mut p1p0,
        &raw mut blimit,
        &raw mut limit,
        &raw mut thresh,
    );
    _mm_storel_epi64(s.offset(-((1 * p) as isize)) as *mut __m128i, p1p0);
    _mm_storel_epi64(
        s.offset(-((2 * p) as isize)) as *mut __m128i,
        _mm_srli_si128::<8>(p1p0),
    );
    _mm_storel_epi64(s.offset((0 * p) as isize) as *mut __m128i, q1q0);
    _mm_storel_epi64(
        s.offset((1 * p) as isize) as *mut __m128i,
        _mm_srli_si128::<8>(q1q0),
    );
    _mm_storel_epi64(s.offset(-((3 * p) as isize)) as *mut __m128i, p2);
    _mm_storel_epi64(s.offset((2 * p) as isize) as *mut __m128i, q2);
}

#[no_mangle]
#[target_feature(enable = "sse2")]
pub unsafe extern "C-unwind" fn aom_lpf_horizontal_4_dual_sse2(
    s: *mut u8,
    p: i32,
    _blimit0: *const u8,
    _limit0: *const u8,
    _thresh0: *const u8,
    _blimit1: *const u8,
    _limit1: *const u8,
    _thresh1: *const u8,
) {
    let mut p1: __m128i = core::mem::zeroed();
    let mut p0: __m128i = core::mem::zeroed();
    let mut q0: __m128i = core::mem::zeroed();
    let mut q1: __m128i = core::mem::zeroed();
    let mut qs1qs0: __m128i = core::mem::zeroed();
    let mut ps1ps0: __m128i = core::mem::zeroed();
    p1 = _mm_loadl_epi64(s.offset(-((2 * p) as isize)) as *mut __m128i as *const __m128i);
    p0 = _mm_loadl_epi64(s.offset(-((1 * p) as isize)) as *mut __m128i as *const __m128i);
    q0 = _mm_loadl_epi64(s.offset(-((0 * p) as isize)) as *mut __m128i as *const __m128i);
    q1 = _mm_loadl_epi64(s.offset((1 * p) as isize) as *mut __m128i as *const __m128i);
    let zero: __m128i = _mm_setzero_si128();
    let blimit: __m128i = _mm_unpacklo_epi32(
        _mm_load_si128(_blimit0 as *const __m128i),
        _mm_load_si128(_blimit1 as *const __m128i),
    );
    let limit: __m128i = _mm_unpacklo_epi32(
        _mm_load_si128(_limit0 as *const __m128i),
        _mm_load_si128(_limit1 as *const __m128i),
    );
    let mut l: __m128i = _mm_unpacklo_epi64(blimit, limit);
    let mut thresh0: __m128i = _mm_unpacklo_epi8(_mm_loadl_epi64(_thresh0 as *const __m128i), zero);
    let mut thresh1: __m128i = _mm_unpacklo_epi8(_mm_loadl_epi64(_thresh1 as *const __m128i), zero);
    let mut t: __m128i = _mm_unpacklo_epi64(thresh0, thresh1);
    lpf_internal_4_dual_sse2(
        &raw mut p1,
        &raw mut p0,
        &raw mut q0,
        &raw mut q1,
        &raw mut l,
        &raw mut t,
        &raw mut qs1qs0,
        &raw mut ps1ps0,
    );
    _mm_storel_epi64(s.offset(-((1 * p) as isize)) as *mut __m128i, ps1ps0);
    _mm_storel_epi64(
        s.offset(-((2 * p) as isize)) as *mut __m128i,
        _mm_srli_si128::<8>(ps1ps0),
    );
    _mm_storel_epi64(s.offset((0 * p) as isize) as *mut __m128i, qs1qs0);
    _mm_storel_epi64(
        s.offset((1 * p) as isize) as *mut __m128i,
        _mm_srli_si128::<8>(qs1qs0),
    );
}

#[no_mangle]
#[target_feature(enable = "sse2")]
pub unsafe extern "C-unwind" fn aom_lpf_vertical_4_dual_sse2(
    s: *mut u8,
    p: i32,
    _blimit0: *const u8,
    _limit0: *const u8,
    _thresh0: *const u8,
    _blimit1: *const u8,
    _limit1: *const u8,
    _thresh1: *const u8,
) {
    let mut p0: __m128i = core::mem::zeroed();
    let mut q0: __m128i = core::mem::zeroed();
    let mut q1: __m128i = core::mem::zeroed();
    let mut p1: __m128i = core::mem::zeroed();
    let mut x0: __m128i = core::mem::zeroed();
    let mut x1: __m128i = core::mem::zeroed();
    let mut x2: __m128i = core::mem::zeroed();
    let mut x3: __m128i = core::mem::zeroed();
    let mut x4: __m128i = core::mem::zeroed();
    let mut x5: __m128i = core::mem::zeroed();
    let mut x6: __m128i = core::mem::zeroed();
    let mut x7: __m128i = core::mem::zeroed();
    let mut d0: __m128i = core::mem::zeroed();
    let mut d1: __m128i = core::mem::zeroed();
    let mut d2: __m128i = core::mem::zeroed();
    let mut d3: __m128i = core::mem::zeroed();
    let mut d4: __m128i = core::mem::zeroed();
    let mut d5: __m128i = core::mem::zeroed();
    let mut d6: __m128i = core::mem::zeroed();
    let mut d7: __m128i = core::mem::zeroed();
    let mut qs1qs0: __m128i = core::mem::zeroed();
    let mut ps1ps0: __m128i = core::mem::zeroed();
    let zero: __m128i = _mm_setzero_si128();
    let blimit: __m128i = _mm_unpacklo_epi32(
        _mm_load_si128(_blimit0 as *const __m128i),
        _mm_load_si128(_blimit1 as *const __m128i),
    );
    let limit: __m128i = _mm_unpacklo_epi32(
        _mm_load_si128(_limit0 as *const __m128i),
        _mm_load_si128(_limit1 as *const __m128i),
    );
    let mut l: __m128i = _mm_unpacklo_epi64(blimit, limit);
    let mut thresh0: __m128i = _mm_unpacklo_epi8(_mm_loadl_epi64(_thresh0 as *const __m128i), zero);
    let mut thresh1: __m128i = _mm_unpacklo_epi8(_mm_loadl_epi64(_thresh1 as *const __m128i), zero);
    let mut t: __m128i = _mm_unpacklo_epi64(thresh0, thresh1);
    x0 = _mm_loadl_epi64(s.offset(-2) as *mut __m128i as *const __m128i);
    x1 = _mm_loadl_epi64(s.offset(-2).offset(p as isize) as *mut __m128i as *const __m128i);
    x2 = _mm_loadl_epi64(s.offset(-2).offset((2 * p) as isize) as *mut __m128i as *const __m128i);
    x3 = _mm_loadl_epi64(s.offset(-2).offset((3 * p) as isize) as *mut __m128i as *const __m128i);
    x4 = _mm_loadl_epi64(s.offset(-2).offset((4 * p) as isize) as *mut __m128i as *const __m128i);
    x5 = _mm_loadl_epi64(s.offset(-2).offset((5 * p) as isize) as *mut __m128i as *const __m128i);
    x6 = _mm_loadl_epi64(s.offset(-2).offset((6 * p) as isize) as *mut __m128i as *const __m128i);
    x7 = _mm_loadl_epi64(s.offset(-2).offset((7 * p) as isize) as *mut __m128i as *const __m128i);
    transpose8x8_low_sse2(
        &raw mut x0,
        &raw mut x1,
        &raw mut x2,
        &raw mut x3,
        &raw mut x4,
        &raw mut x5,
        &raw mut x6,
        &raw mut x7,
        &raw mut p1,
        &raw mut p0,
        &raw mut q0,
        &raw mut q1,
    );
    lpf_internal_4_dual_sse2(
        &raw mut p1,
        &raw mut p0,
        &raw mut q0,
        &raw mut q1,
        &raw mut l,
        &raw mut t,
        &raw mut qs1qs0,
        &raw mut ps1ps0,
    );
    p1 = _mm_srli_si128::<8>(ps1ps0);
    q1 = _mm_srli_si128::<8>(qs1qs0);
    transpose4x8_8x4_sse2(
        &raw mut p1,
        &raw mut ps1ps0,
        &raw mut qs1qs0,
        &raw mut q1,
        &raw mut d0,
        &raw mut d1,
        &raw mut d2,
        &raw mut d3,
        &raw mut d4,
        &raw mut d5,
        &raw mut d6,
        &raw mut d7,
    );
    xx_storel_32(
        s.offset(-2).offset((0 * p) as isize) as *mut core::ffi::c_void,
        d0,
    );
    xx_storel_32(
        s.offset(-2).offset((1 * p) as isize) as *mut core::ffi::c_void,
        d1,
    );
    xx_storel_32(
        s.offset(-2).offset((2 * p) as isize) as *mut core::ffi::c_void,
        d2,
    );
    xx_storel_32(
        s.offset(-2).offset((3 * p) as isize) as *mut core::ffi::c_void,
        d3,
    );
    xx_storel_32(
        s.offset(-2).offset((4 * p) as isize) as *mut core::ffi::c_void,
        d4,
    );
    xx_storel_32(
        s.offset(-2).offset((5 * p) as isize) as *mut core::ffi::c_void,
        d5,
    );
    xx_storel_32(
        s.offset(-2).offset((6 * p) as isize) as *mut core::ffi::c_void,
        d6,
    );
    xx_storel_32(
        s.offset(-2).offset((7 * p) as isize) as *mut core::ffi::c_void,
        d7,
    );
}

#[no_mangle]
#[target_feature(enable = "sse2")]
pub unsafe extern "C-unwind" fn aom_lpf_vertical_6_sse2(
    s: *mut u8,
    p: i32,
    _blimit: *const u8,
    _limit: *const u8,
    _thresh: *const u8,
) {
    let mut d0: __m128i = core::mem::zeroed();
    let mut d1: __m128i = core::mem::zeroed();
    let mut d2: __m128i = core::mem::zeroed();
    let mut d3: __m128i = core::mem::zeroed();
    let mut d4: __m128i = core::mem::zeroed();
    let mut d5: __m128i = core::mem::zeroed();
    let mut d6: __m128i = core::mem::zeroed();
    let mut d7: __m128i = core::mem::zeroed();
    let mut x2: __m128i = core::mem::zeroed();
    let mut x1: __m128i = core::mem::zeroed();
    let mut x0: __m128i = core::mem::zeroed();
    let mut x3: __m128i = core::mem::zeroed();
    let mut p0: __m128i = core::mem::zeroed();
    let mut q0: __m128i = core::mem::zeroed();
    let mut p1p0: __m128i = core::mem::zeroed();
    let mut q1q0: __m128i = core::mem::zeroed();
    let mut blimit: __m128i = _mm_load_si128(_blimit as *mut __m128i as *const __m128i);
    let mut limit: __m128i = _mm_load_si128(_limit as *mut __m128i as *const __m128i);
    let mut thresh: __m128i = _mm_load_si128(_thresh as *mut __m128i as *const __m128i);
    x3 = _mm_loadl_epi64(s.offset(-3).offset((0 * p) as isize) as *mut __m128i as *const __m128i);
    x2 = _mm_loadl_epi64(s.offset(-3).offset((1 * p) as isize) as *mut __m128i as *const __m128i);
    x1 = _mm_loadl_epi64(s.offset(-3).offset((2 * p) as isize) as *mut __m128i as *const __m128i);
    x0 = _mm_loadl_epi64(s.offset(-3).offset((3 * p) as isize) as *mut __m128i as *const __m128i);
    transpose4x8_8x4_sse2(
        &raw mut x3,
        &raw mut x2,
        &raw mut x1,
        &raw mut x0,
        &raw mut d0,
        &raw mut d1,
        &raw mut d2,
        &raw mut d3,
        &raw mut d4,
        &raw mut d5,
        &raw mut d6,
        &raw mut d7,
    );
    lpf_internal_6_sse2(
        &raw mut d0,
        &raw mut d5,
        &raw mut d1,
        &raw mut d4,
        &raw mut d2,
        &raw mut d3,
        &raw mut q1q0,
        &raw mut p1p0,
        &raw mut blimit,
        &raw mut limit,
        &raw mut thresh,
    );
    p0 = _mm_srli_si128::<4>(p1p0);
    q0 = _mm_srli_si128::<4>(q1q0);
    transpose4x8_8x4_low_sse2(
        &raw mut p0,
        &raw mut p1p0,
        &raw mut q1q0,
        &raw mut q0,
        &raw mut d0,
        &raw mut d1,
        &raw mut d2,
        &raw mut d3,
    );
    xx_storel_32(
        s.offset((0 * p) as isize).offset(-2) as *mut core::ffi::c_void,
        d0,
    );
    xx_storel_32(
        s.offset((1 * p) as isize).offset(-2) as *mut core::ffi::c_void,
        d1,
    );
    xx_storel_32(
        s.offset((2 * p) as isize).offset(-2) as *mut core::ffi::c_void,
        d2,
    );
    xx_storel_32(
        s.offset((3 * p) as isize).offset(-2) as *mut core::ffi::c_void,
        d3,
    );
}

#[no_mangle]
#[target_feature(enable = "sse2")]
pub unsafe extern "C-unwind" fn aom_lpf_vertical_6_dual_sse2(
    s: *mut u8,
    p: i32,
    _blimit0: *const u8,
    _limit0: *const u8,
    _thresh0: *const u8,
    _blimit1: *const u8,
    _limit1: *const u8,
    _thresh1: *const u8,
) {
    let mut blimit: __m128i = _mm_unpacklo_epi32(
        _mm_load_si128(_blimit0 as *mut __m128i as *const __m128i),
        _mm_load_si128(_blimit1 as *mut __m128i as *const __m128i),
    );
    let mut limit: __m128i = _mm_unpacklo_epi32(
        _mm_load_si128(_limit0 as *mut __m128i as *const __m128i),
        _mm_load_si128(_limit1 as *mut __m128i as *const __m128i),
    );
    let mut thresh: __m128i = _mm_unpacklo_epi32(
        _mm_load_si128(_thresh0 as *mut __m128i as *const __m128i),
        _mm_load_si128(_thresh1 as *mut __m128i as *const __m128i),
    );
    let mut d0: __m128i = core::mem::zeroed();
    let mut d1: __m128i = core::mem::zeroed();
    let mut d2: __m128i = core::mem::zeroed();
    let mut d3: __m128i = core::mem::zeroed();
    let mut d4: __m128i = core::mem::zeroed();
    let mut d5: __m128i = core::mem::zeroed();
    let mut d6: __m128i = core::mem::zeroed();
    let mut d7: __m128i = core::mem::zeroed();
    let mut x0: __m128i = core::mem::zeroed();
    let mut x1: __m128i = core::mem::zeroed();
    let mut x2: __m128i = core::mem::zeroed();
    let mut x3: __m128i = core::mem::zeroed();
    let mut x4: __m128i = core::mem::zeroed();
    let mut x5: __m128i = core::mem::zeroed();
    let mut x6: __m128i = core::mem::zeroed();
    let mut x7: __m128i = core::mem::zeroed();
    let mut p0: __m128i = core::mem::zeroed();
    let mut q0: __m128i = core::mem::zeroed();
    let mut p1p0: __m128i = core::mem::zeroed();
    let mut q1q0: __m128i = core::mem::zeroed();
    let mut d0d1: __m128i = core::mem::zeroed();
    let mut d2d3: __m128i = core::mem::zeroed();
    let mut d4d5: __m128i = core::mem::zeroed();
    let mut d6d7: __m128i = core::mem::zeroed();
    x0 = _mm_loadl_epi64(s.offset(-3).offset((0 * p) as isize) as *mut __m128i as *const __m128i);
    x1 = _mm_loadl_epi64(s.offset(-3).offset((1 * p) as isize) as *mut __m128i as *const __m128i);
    x2 = _mm_loadl_epi64(s.offset(-3).offset((2 * p) as isize) as *mut __m128i as *const __m128i);
    x3 = _mm_loadl_epi64(s.offset(-3).offset((3 * p) as isize) as *mut __m128i as *const __m128i);
    x4 = _mm_loadl_epi64(s.offset(-3).offset((4 * p) as isize) as *mut __m128i as *const __m128i);
    x5 = _mm_loadl_epi64(s.offset(-3).offset((5 * p) as isize) as *mut __m128i as *const __m128i);
    x6 = _mm_loadl_epi64(s.offset(-3).offset((6 * p) as isize) as *mut __m128i as *const __m128i);
    x7 = _mm_loadl_epi64(s.offset(-3).offset((7 * p) as isize) as *mut __m128i as *const __m128i);
    transpose8x8_sse2(
        &raw mut x0,
        &raw mut x1,
        &raw mut x2,
        &raw mut x3,
        &raw mut x4,
        &raw mut x5,
        &raw mut x6,
        &raw mut x7,
        &raw mut d0d1,
        &raw mut d2d3,
        &raw mut d4d5,
        &raw mut d6d7,
    );
    d1 = _mm_srli_si128::<8>(d0d1);
    d3 = _mm_srli_si128::<8>(d2d3);
    d5 = _mm_srli_si128::<8>(d4d5);
    d7 = _mm_srli_si128::<8>(d6d7);
    lpf_internal_6_dual_sse2(
        &raw mut d0d1,
        &raw mut d5,
        &raw mut d1,
        &raw mut d4d5,
        &raw mut d2d3,
        &raw mut d3,
        &raw mut q1q0,
        &raw mut p1p0,
        &raw mut blimit,
        &raw mut limit,
        &raw mut thresh,
    );
    p0 = _mm_srli_si128::<8>(p1p0);
    q0 = _mm_srli_si128::<8>(q1q0);
    transpose4x8_8x4_sse2(
        &raw mut p0,
        &raw mut p1p0,
        &raw mut q1q0,
        &raw mut q0,
        &raw mut d0,
        &raw mut d1,
        &raw mut d2,
        &raw mut d3,
        &raw mut d4,
        &raw mut d5,
        &raw mut d6,
        &raw mut d7,
    );
    xx_storel_32(
        s.offset(-2).offset((0 * p) as isize) as *mut core::ffi::c_void,
        d0,
    );
    xx_storel_32(
        s.offset(-2).offset((1 * p) as isize) as *mut core::ffi::c_void,
        d1,
    );
    xx_storel_32(
        s.offset(-2).offset((2 * p) as isize) as *mut core::ffi::c_void,
        d2,
    );
    xx_storel_32(
        s.offset(-2).offset((3 * p) as isize) as *mut core::ffi::c_void,
        d3,
    );
    xx_storel_32(
        s.offset(-2).offset((4 * p) as isize) as *mut core::ffi::c_void,
        d4,
    );
    xx_storel_32(
        s.offset(-2).offset((5 * p) as isize) as *mut core::ffi::c_void,
        d5,
    );
    xx_storel_32(
        s.offset(-2).offset((6 * p) as isize) as *mut core::ffi::c_void,
        d6,
    );
    xx_storel_32(
        s.offset(-2).offset((7 * p) as isize) as *mut core::ffi::c_void,
        d7,
    );
}

#[no_mangle]
#[target_feature(enable = "sse2")]
pub unsafe extern "C-unwind" fn aom_lpf_vertical_8_sse2(
    s: *mut u8,
    p: i32,
    _blimit: *const u8,
    _limit: *const u8,
    _thresh: *const u8,
) {
    let mut d0: __m128i = core::mem::zeroed();
    let mut d1: __m128i = core::mem::zeroed();
    let mut d2: __m128i = core::mem::zeroed();
    let mut d3: __m128i = core::mem::zeroed();
    let mut d4: __m128i = core::mem::zeroed();
    let mut d5: __m128i = core::mem::zeroed();
    let mut d6: __m128i = core::mem::zeroed();
    let mut d7: __m128i = core::mem::zeroed();
    let mut p0: __m128i = core::mem::zeroed();
    let mut q0: __m128i = core::mem::zeroed();
    let mut x2: __m128i = core::mem::zeroed();
    let mut x1: __m128i = core::mem::zeroed();
    let mut x0: __m128i = core::mem::zeroed();
    let mut x3: __m128i = core::mem::zeroed();
    let mut q1q0: __m128i = core::mem::zeroed();
    let mut p1p0: __m128i = core::mem::zeroed();
    let mut blimit: __m128i = _mm_load_si128(_blimit as *const __m128i);
    let mut limit: __m128i = _mm_load_si128(_limit as *const __m128i);
    let mut thresh: __m128i = _mm_load_si128(_thresh as *const __m128i);
    x3 = _mm_loadl_epi64(s.offset(-4).offset((0 * p) as isize) as *mut __m128i as *const __m128i);
    x2 = _mm_loadl_epi64(s.offset(-4).offset((1 * p) as isize) as *mut __m128i as *const __m128i);
    x1 = _mm_loadl_epi64(s.offset(-4).offset((2 * p) as isize) as *mut __m128i as *const __m128i);
    x0 = _mm_loadl_epi64(s.offset(-4).offset((3 * p) as isize) as *mut __m128i as *const __m128i);
    transpose4x8_8x4_sse2(
        &raw mut x3,
        &raw mut x2,
        &raw mut x1,
        &raw mut x0,
        &raw mut d0,
        &raw mut d1,
        &raw mut d2,
        &raw mut d3,
        &raw mut d4,
        &raw mut d5,
        &raw mut d6,
        &raw mut d7,
    );
    // Loop filtering
    lpf_internal_8_sse2(
        &raw mut d0,
        &raw mut d7,
        &raw mut d1,
        &raw mut d6,
        &raw mut d2,
        &raw mut d5,
        &raw mut d3,
        &raw mut d4,
        &raw mut q1q0,
        &raw mut p1p0,
        &raw mut blimit,
        &raw mut limit,
        &raw mut thresh,
    );
    p0 = _mm_srli_si128::<4>(p1p0);
    q0 = _mm_srli_si128::<4>(q1q0);
    transpose8x8_low_sse2(
        &raw mut d0,
        &raw mut d1,
        &raw mut p0,
        &raw mut p1p0,
        &raw mut q1q0,
        &raw mut q0,
        &raw mut d6,
        &raw mut d7,
        &raw mut d0,
        &raw mut d1,
        &raw mut d2,
        &raw mut d3,
    );
    _mm_storel_epi64(s.offset(-4).offset((0 * p) as isize) as *mut __m128i, d0);
    _mm_storel_epi64(s.offset(-4).offset((1 * p) as isize) as *mut __m128i, d1);
    _mm_storel_epi64(s.offset(-4).offset((2 * p) as isize) as *mut __m128i, d2);
    _mm_storel_epi64(s.offset(-4).offset((3 * p) as isize) as *mut __m128i, d3);
}

#[no_mangle]
#[target_feature(enable = "sse2")]
pub unsafe extern "C-unwind" fn aom_lpf_vertical_8_dual_sse2(
    s: *mut u8,
    p: i32,
    _blimit0: *const u8,
    _limit0: *const u8,
    _thresh0: *const u8,
    _blimit1: *const u8,
    _limit1: *const u8,
    _thresh1: *const u8,
) {
    let mut blimit: __m128i = _mm_unpacklo_epi32(
        _mm_load_si128(_blimit0 as *mut __m128i as *const __m128i),
        _mm_load_si128(_blimit1 as *mut __m128i as *const __m128i),
    );
    let mut limit: __m128i = _mm_unpacklo_epi32(
        _mm_load_si128(_limit0 as *mut __m128i as *const __m128i),
        _mm_load_si128(_limit1 as *mut __m128i as *const __m128i),
    );
    let mut thresh: __m128i = _mm_unpacklo_epi32(
        _mm_load_si128(_thresh0 as *mut __m128i as *const __m128i),
        _mm_load_si128(_thresh1 as *mut __m128i as *const __m128i),
    );
    let mut x0: __m128i = core::mem::zeroed();
    let mut x1: __m128i = core::mem::zeroed();
    let mut x2: __m128i = core::mem::zeroed();
    let mut x3: __m128i = core::mem::zeroed();
    let mut x4: __m128i = core::mem::zeroed();
    let mut x5: __m128i = core::mem::zeroed();
    let mut x6: __m128i = core::mem::zeroed();
    let mut x7: __m128i = core::mem::zeroed();
    let mut d1: __m128i = core::mem::zeroed();
    let mut d3: __m128i = core::mem::zeroed();
    let mut d5: __m128i = core::mem::zeroed();
    let mut d7: __m128i = core::mem::zeroed();
    let mut q1q0: __m128i = core::mem::zeroed();
    let mut p1p0: __m128i = core::mem::zeroed();
    let mut p1: __m128i = core::mem::zeroed();
    let mut q1: __m128i = core::mem::zeroed();
    let mut d0d1: __m128i = core::mem::zeroed();
    let mut d2d3: __m128i = core::mem::zeroed();
    let mut d4d5: __m128i = core::mem::zeroed();
    let mut d6d7: __m128i = core::mem::zeroed();
    x0 = _mm_loadl_epi64(s.offset(-4).offset((0 * p) as isize) as *mut __m128i as *const __m128i);
    x1 = _mm_loadl_epi64(s.offset(-4).offset((1 * p) as isize) as *mut __m128i as *const __m128i);
    x2 = _mm_loadl_epi64(s.offset(-4).offset((2 * p) as isize) as *mut __m128i as *const __m128i);
    x3 = _mm_loadl_epi64(s.offset(-4).offset((3 * p) as isize) as *mut __m128i as *const __m128i);
    x4 = _mm_loadl_epi64(s.offset(-4).offset((4 * p) as isize) as *mut __m128i as *const __m128i);
    x5 = _mm_loadl_epi64(s.offset(-4).offset((5 * p) as isize) as *mut __m128i as *const __m128i);
    x6 = _mm_loadl_epi64(s.offset(-4).offset((6 * p) as isize) as *mut __m128i as *const __m128i);
    x7 = _mm_loadl_epi64(s.offset(-4).offset((7 * p) as isize) as *mut __m128i as *const __m128i);
    transpose8x8_sse2(
        &raw mut x0,
        &raw mut x1,
        &raw mut x2,
        &raw mut x3,
        &raw mut x4,
        &raw mut x5,
        &raw mut x6,
        &raw mut x7,
        &raw mut d0d1,
        &raw mut d2d3,
        &raw mut d4d5,
        &raw mut d6d7,
    );
    d1 = _mm_srli_si128::<8>(d0d1);
    d3 = _mm_srli_si128::<8>(d2d3);
    d5 = _mm_srli_si128::<8>(d4d5);
    d7 = _mm_srli_si128::<8>(d6d7);
    lpf_internal_8_dual_sse2(
        &raw mut d0d1,
        &raw mut d7,
        &raw mut d1,
        &raw mut d6d7,
        &raw mut d2d3,
        &raw mut d5,
        &raw mut d3,
        &raw mut d4d5,
        &raw mut q1q0,
        &raw mut p1p0,
        &raw mut blimit,
        &raw mut limit,
        &raw mut thresh,
    );
    p1 = _mm_srli_si128::<8>(p1p0);
    q1 = _mm_srli_si128::<8>(q1q0);
    transpose8x8_sse2(
        &raw mut d0d1,
        &raw mut d1,
        &raw mut p1,
        &raw mut p1p0,
        &raw mut q1q0,
        &raw mut q1,
        &raw mut d6d7,
        &raw mut d7,
        &raw mut d0d1,
        &raw mut d2d3,
        &raw mut d4d5,
        &raw mut d6d7,
    );
    _mm_storel_epi64(s.offset(-4).offset((0 * p) as isize) as *mut __m128i, d0d1);
    _mm_storel_epi64(
        s.offset(-4).offset((1 * p) as isize) as *mut __m128i,
        _mm_srli_si128::<8>(d0d1),
    );
    _mm_storel_epi64(s.offset(-4).offset((2 * p) as isize) as *mut __m128i, d2d3);
    _mm_storel_epi64(
        s.offset(-4).offset((3 * p) as isize) as *mut __m128i,
        _mm_srli_si128::<8>(d2d3),
    );
    _mm_storel_epi64(s.offset(-4).offset((4 * p) as isize) as *mut __m128i, d4d5);
    _mm_storel_epi64(
        s.offset(-4).offset((5 * p) as isize) as *mut __m128i,
        _mm_srli_si128::<8>(d4d5),
    );
    _mm_storel_epi64(s.offset(-4).offset((6 * p) as isize) as *mut __m128i, d6d7);
    _mm_storel_epi64(
        s.offset(-4).offset((7 * p) as isize) as *mut __m128i,
        _mm_srli_si128::<8>(d6d7),
    );
}

#[no_mangle]
#[target_feature(enable = "sse2")]
pub unsafe extern "C-unwind" fn aom_lpf_vertical_14_sse2(
    s: *mut u8,
    p: i32,
    _blimit: *const u8,
    _limit: *const u8,
    _thresh: *const u8,
) {
    let mut q7p7: __m128i = core::mem::zeroed();
    let mut q6p6: __m128i = core::mem::zeroed();
    let mut q5p5: __m128i = core::mem::zeroed();
    let mut q4p4: __m128i = core::mem::zeroed();
    let mut q3p3: __m128i = core::mem::zeroed();
    let mut q2p2: __m128i = core::mem::zeroed();
    let mut q1p1: __m128i = core::mem::zeroed();
    let mut q0p0: __m128i = core::mem::zeroed();
    let mut x6: __m128i = core::mem::zeroed();
    let mut x5: __m128i = core::mem::zeroed();
    let mut x4: __m128i = core::mem::zeroed();
    let mut x3: __m128i = core::mem::zeroed();
    let mut pq0: __m128i = core::mem::zeroed();
    let mut pq1: __m128i = core::mem::zeroed();
    let mut pq2: __m128i = core::mem::zeroed();
    let mut pq3: __m128i = core::mem::zeroed();
    let mut blimit: __m128i = _mm_load_si128(_blimit as *mut __m128i as *const __m128i);
    let mut limit: __m128i = _mm_load_si128(_limit as *mut __m128i as *const __m128i);
    let mut thresh: __m128i = _mm_load_si128(_thresh as *mut __m128i as *const __m128i);
    x6 = _mm_loadu_si128(s.offset(-8).offset((0 * p) as isize) as *mut __m128i as *const __m128i);
    x5 = _mm_loadu_si128(s.offset(-8).offset((1 * p) as isize) as *mut __m128i as *const __m128i);
    x4 = _mm_loadu_si128(s.offset(-8).offset((2 * p) as isize) as *mut __m128i as *const __m128i);
    x3 = _mm_loadu_si128(s.offset(-8).offset((3 * p) as isize) as *mut __m128i as *const __m128i);
    transpose_pq_14_sse2(
        &raw mut x6,
        &raw mut x5,
        &raw mut x4,
        &raw mut x3,
        &raw mut q0p0,
        &raw mut q1p1,
        &raw mut q2p2,
        &raw mut q3p3,
        &raw mut q4p4,
        &raw mut q5p5,
        &raw mut q6p6,
        &raw mut q7p7,
    );
    lpf_internal_14_sse2(
        &raw mut q6p6,
        &raw mut q5p5,
        &raw mut q4p4,
        &raw mut q3p3,
        &raw mut q2p2,
        &raw mut q1p1,
        &raw mut q0p0,
        &raw mut blimit,
        &raw mut limit,
        &raw mut thresh,
    );
    transpose_pq_14_inv_sse2(
        &raw mut q7p7,
        &raw mut q6p6,
        &raw mut q5p5,
        &raw mut q4p4,
        &raw mut q3p3,
        &raw mut q2p2,
        &raw mut q1p1,
        &raw mut q0p0,
        &raw mut pq0,
        &raw mut pq1,
        &raw mut pq2,
        &raw mut pq3,
    );
    _mm_storeu_si128(s.offset(-8).offset((0 * p) as isize) as *mut __m128i, pq0);
    _mm_storeu_si128(s.offset(-8).offset((1 * p) as isize) as *mut __m128i, pq1);
    _mm_storeu_si128(s.offset(-8).offset((2 * p) as isize) as *mut __m128i, pq2);
    _mm_storeu_si128(s.offset(-8).offset((3 * p) as isize) as *mut __m128i, pq3);
}

#[no_mangle]
#[target_feature(enable = "sse2")]
pub unsafe extern "C-unwind" fn aom_lpf_vertical_14_dual_sse2(
    s: *mut u8,
    p: i32,
    _blimit0: *const u8,
    _limit0: *const u8,
    _thresh0: *const u8,
    _blimit1: *const u8,
    _limit1: *const u8,
    _thresh1: *const u8,
) {
    let mut q6p6: __m128i = core::mem::zeroed();
    let mut q5p5: __m128i = core::mem::zeroed();
    let mut q4p4: __m128i = core::mem::zeroed();
    let mut q3p3: __m128i = core::mem::zeroed();
    let mut q2p2: __m128i = core::mem::zeroed();
    let mut q1p1: __m128i = core::mem::zeroed();
    let mut q0p0: __m128i = core::mem::zeroed();
    let mut x7: __m128i = core::mem::zeroed();
    let mut x6: __m128i = core::mem::zeroed();
    let mut x5: __m128i = core::mem::zeroed();
    let mut x4: __m128i = core::mem::zeroed();
    let mut x3: __m128i = core::mem::zeroed();
    let mut x2: __m128i = core::mem::zeroed();
    let mut x1: __m128i = core::mem::zeroed();
    let mut x0: __m128i = core::mem::zeroed();
    let mut d0d1: __m128i = core::mem::zeroed();
    let mut d2d3: __m128i = core::mem::zeroed();
    let mut d4d5: __m128i = core::mem::zeroed();
    let mut d6d7: __m128i = core::mem::zeroed();
    let mut d8d9: __m128i = core::mem::zeroed();
    let mut d10d11: __m128i = core::mem::zeroed();
    let mut d12d13: __m128i = core::mem::zeroed();
    let mut d14d15: __m128i = core::mem::zeroed();
    let mut q0: __m128i = core::mem::zeroed();
    let mut q1: __m128i = core::mem::zeroed();
    let mut q2: __m128i = core::mem::zeroed();
    let mut q3: __m128i = core::mem::zeroed();
    let mut q7: __m128i = core::mem::zeroed();
    let mut p0p1: __m128i = core::mem::zeroed();
    let mut p2p3: __m128i = core::mem::zeroed();
    let mut p4p5: __m128i = core::mem::zeroed();
    let mut p6p7: __m128i = core::mem::zeroed();
    let mut blimit: __m128i = _mm_unpacklo_epi32(
        _mm_load_si128(_blimit0 as *const __m128i),
        _mm_load_si128(_blimit1 as *const __m128i),
    );
    let mut limit: __m128i = _mm_unpacklo_epi32(
        _mm_load_si128(_limit0 as *const __m128i),
        _mm_load_si128(_limit1 as *const __m128i),
    );
    let mut thresh: __m128i = _mm_unpacklo_epi32(
        _mm_load_si128(_thresh0 as *const __m128i),
        _mm_load_si128(_thresh1 as *const __m128i),
    );
    x7 = _mm_loadu_si128(s.offset(-8).offset((0 * p) as isize) as *mut __m128i as *const __m128i);
    x6 = _mm_loadu_si128(s.offset(-8).offset((1 * p) as isize) as *mut __m128i as *const __m128i);
    x5 = _mm_loadu_si128(s.offset(-8).offset((2 * p) as isize) as *mut __m128i as *const __m128i);
    x4 = _mm_loadu_si128(s.offset(-8).offset((3 * p) as isize) as *mut __m128i as *const __m128i);
    x3 = _mm_loadu_si128(s.offset(-8).offset((4 * p) as isize) as *mut __m128i as *const __m128i);
    x2 = _mm_loadu_si128(s.offset(-8).offset((5 * p) as isize) as *mut __m128i as *const __m128i);
    x1 = _mm_loadu_si128(s.offset(-8).offset((6 * p) as isize) as *mut __m128i as *const __m128i);
    x0 = _mm_loadu_si128(s.offset(-8).offset((7 * p) as isize) as *mut __m128i as *const __m128i);
    transpose8x16_16x8_sse2(
        &raw mut x7,
        &raw mut x6,
        &raw mut x5,
        &raw mut x4,
        &raw mut x3,
        &raw mut x2,
        &raw mut x1,
        &raw mut x0,
        &raw mut d0d1,
        &raw mut d2d3,
        &raw mut d4d5,
        &raw mut d6d7,
        &raw mut d8d9,
        &raw mut d10d11,
        &raw mut d12d13,
        &raw mut d14d15,
    );
    q6p6 = _mm_unpacklo_epi64(d2d3, _mm_srli_si128::<8>(d12d13));
    q5p5 = _mm_unpacklo_epi64(d4d5, _mm_srli_si128::<8>(d10d11));
    q4p4 = _mm_unpacklo_epi64(d6d7, _mm_srli_si128::<8>(d8d9));
    q3p3 = _mm_unpacklo_epi64(d8d9, _mm_srli_si128::<8>(d6d7));
    q2p2 = _mm_unpacklo_epi64(d10d11, _mm_srli_si128::<8>(d4d5));
    q1p1 = _mm_unpacklo_epi64(d12d13, _mm_srli_si128::<8>(d2d3));
    q0p0 = _mm_unpacklo_epi64(d14d15, _mm_srli_si128::<8>(d0d1));
    q7 = _mm_srli_si128::<8>(d14d15);
    lpf_internal_14_dual_sse2(
        &raw mut q6p6,
        &raw mut q5p5,
        &raw mut q4p4,
        &raw mut q3p3,
        &raw mut q2p2,
        &raw mut q1p1,
        &raw mut q0p0,
        &raw mut blimit,
        &raw mut limit,
        &raw mut thresh,
    );
    x0 = _mm_srli_si128::<8>(q0p0);
    x1 = _mm_srli_si128::<8>(q1p1);
    x2 = _mm_srli_si128::<8>(q2p2);
    x3 = _mm_srli_si128::<8>(q3p3);
    x4 = _mm_srli_si128::<8>(q4p4);
    x5 = _mm_srli_si128::<8>(q5p5);
    x6 = _mm_srli_si128::<8>(q6p6);
    transpose16x8_8x16_sse2(
        &raw mut d0d1,
        &raw mut q6p6,
        &raw mut q5p5,
        &raw mut q4p4,
        &raw mut q3p3,
        &raw mut q2p2,
        &raw mut q1p1,
        &raw mut q0p0,
        &raw mut x0,
        &raw mut x1,
        &raw mut x2,
        &raw mut x3,
        &raw mut x4,
        &raw mut x5,
        &raw mut x6,
        &raw mut q7,
        &raw mut p0p1,
        &raw mut p2p3,
        &raw mut p4p5,
        &raw mut p6p7,
        &raw mut q0,
        &raw mut q1,
        &raw mut q2,
        &raw mut q3,
    );
    _mm_storeu_si128(s.offset(-8).offset((0 * p) as isize) as *mut __m128i, p0p1);
    _mm_storeu_si128(s.offset(-8).offset((1 * p) as isize) as *mut __m128i, p2p3);
    _mm_storeu_si128(s.offset(-8).offset((2 * p) as isize) as *mut __m128i, p4p5);
    _mm_storeu_si128(s.offset(-8).offset((3 * p) as isize) as *mut __m128i, p6p7);
    _mm_storeu_si128(s.offset(-8).offset((4 * p) as isize) as *mut __m128i, q0);
    _mm_storeu_si128(s.offset(-8).offset((5 * p) as isize) as *mut __m128i, q1);
    _mm_storeu_si128(s.offset(-8).offset((6 * p) as isize) as *mut __m128i, q2);
    _mm_storeu_si128(s.offset(-8).offset((7 * p) as isize) as *mut __m128i, q3);
}

#[inline]
#[target_feature(enable = "sse2")]
unsafe fn filter_add2_sub2(
    total: *const __m128i,
    a1: *const __m128i,
    a2: *const __m128i,
    s1: *const __m128i,
    s2: *const __m128i,
) -> __m128i {
    let mut x: __m128i = _mm_add_epi16(*a1, *total);
    x = _mm_add_epi16(_mm_sub_epi16(x, _mm_add_epi16(*s1, *s2)), *a2);
    return x;
}

#[inline]
#[target_feature(enable = "sse2")]
unsafe fn filter8_mask(
    flat: *const __m128i,
    other_filt: *const __m128i,
    f8_lo: *const __m128i,
    f8_hi: *const __m128i,
) -> __m128i {
    let f8: __m128i = _mm_packus_epi16(
        _mm_srli_epi16::<{ 3 }>(*f8_lo),
        _mm_srli_epi16::<{ 3 }>(*f8_hi),
    );
    let result: __m128i = _mm_and_si128(*flat, f8);
    return _mm_or_si128(_mm_andnot_si128(*flat, *other_filt), result);
}

#[inline]
#[target_feature(enable = "sse2")]
unsafe fn filter16_mask(
    flat: *const __m128i,
    other_filt: *const __m128i,
    f_lo: *const __m128i,
    f_hi: *const __m128i,
) -> __m128i {
    let f: __m128i = _mm_packus_epi16(
        _mm_srli_epi16::<{ 4 }>(*f_lo),
        _mm_srli_epi16::<{ 4 }>(*f_hi),
    );
    let result: __m128i = _mm_and_si128(*flat, f);
    return _mm_or_si128(_mm_andnot_si128(*flat, *other_filt), result);
}

#[no_mangle]
#[target_feature(enable = "sse2")]
pub unsafe extern "C-unwind" fn aom_lpf_horizontal_14_quad_sse2(
    s: *mut u8,
    p: i32,
    _blimit0: *const u8,
    _limit0: *const u8,
    _thresh0: *const u8,
) {
    let zero: __m128i = _mm_setzero_si128();
    let one: __m128i = _mm_set1_epi8(1);
    let blimit_v: __m128i = _mm_load_si128(_blimit0 as *const __m128i);
    let limit_v: __m128i = _mm_load_si128(_limit0 as *const __m128i);
    let thresh_v: __m128i = _mm_load_si128(_thresh0 as *const __m128i);
    let mut mask: __m128i;
    let mut hev: __m128i;
    let mut flat: __m128i = core::mem::zeroed();
    let mut flat2: __m128i = core::mem::zeroed();
    let mut p6: __m128i;
    let mut p5: __m128i = core::mem::zeroed();
    let mut p4: __m128i = core::mem::zeroed();
    let mut p3: __m128i = core::mem::zeroed();
    let mut p2: __m128i = core::mem::zeroed();
    let mut p1: __m128i;
    let mut p0: __m128i;
    let mut q0: __m128i;
    let mut q1: __m128i;
    let mut q2: __m128i = core::mem::zeroed();
    let mut q3: __m128i = core::mem::zeroed();
    let mut q4: __m128i = core::mem::zeroed();
    let mut q6: __m128i;
    let mut q5: __m128i = core::mem::zeroed();
    let mut op2: __m128i = core::mem::zeroed();
    let mut op1: __m128i = core::mem::zeroed();
    let mut op0: __m128i = core::mem::zeroed();
    let mut oq0: __m128i = core::mem::zeroed();
    let mut oq1: __m128i = core::mem::zeroed();
    let mut oq2: __m128i = core::mem::zeroed();
    let mut max_abs_p1p0q1q0: __m128i;
    p6 = _mm_loadu_si128(s.offset(-((7 * p) as isize)) as *mut __m128i as *const __m128i);
    p5 = _mm_loadu_si128(s.offset(-((6 * p) as isize)) as *mut __m128i as *const __m128i);
    p4 = _mm_loadu_si128(s.offset(-((5 * p) as isize)) as *mut __m128i as *const __m128i);
    p3 = _mm_loadu_si128(s.offset(-((4 * p) as isize)) as *mut __m128i as *const __m128i);
    p2 = _mm_loadu_si128(s.offset(-((3 * p) as isize)) as *mut __m128i as *const __m128i);
    p1 = _mm_loadu_si128(s.offset(-((2 * p) as isize)) as *mut __m128i as *const __m128i);
    p0 = _mm_loadu_si128(s.offset(-((1 * p) as isize)) as *mut __m128i as *const __m128i);
    q0 = _mm_loadu_si128(s.offset(-((0 * p) as isize)) as *mut __m128i as *const __m128i);
    q1 = _mm_loadu_si128(s.offset((1 * p) as isize) as *mut __m128i as *const __m128i);
    q2 = _mm_loadu_si128(s.offset((2 * p) as isize) as *mut __m128i as *const __m128i);
    q3 = _mm_loadu_si128(s.offset((3 * p) as isize) as *mut __m128i as *const __m128i);
    q4 = _mm_loadu_si128(s.offset((4 * p) as isize) as *mut __m128i as *const __m128i);
    q5 = _mm_loadu_si128(s.offset((5 * p) as isize) as *mut __m128i as *const __m128i);
    q6 = _mm_loadu_si128(s.offset((6 * p) as isize) as *mut __m128i as *const __m128i);
    {
        let abs_p1p0: __m128i = abs_diff(p1, p0);
        let abs_q1q0: __m128i = abs_diff(q1, q0);
        let fe: __m128i = _mm_set1_epi8(0xfe_i32 as i8);
        let ff: __m128i = _mm_cmpeq_epi8(zero, zero);
        let mut abs_p0q0: __m128i = abs_diff(p0, q0);
        let mut abs_p1q1: __m128i = abs_diff(p1, q1);
        let mut work: __m128i;
        max_abs_p1p0q1q0 = _mm_max_epu8(abs_p1p0, abs_q1q0);
        abs_p0q0 = _mm_adds_epu8(abs_p0q0, abs_p0q0);
        abs_p1q1 = _mm_srli_epi16::<{ 1 }>(_mm_and_si128(abs_p1q1, fe));
        mask = _mm_subs_epu8(_mm_adds_epu8(abs_p0q0, abs_p1q1), blimit_v);
        mask = _mm_xor_si128(_mm_cmpeq_epi8(mask, zero), ff);
        // mask |= (abs(p0 - q0) * 2 + abs(p1 - q1) / 2  > blimit) * -1;
        mask = _mm_max_epu8(max_abs_p1p0q1q0, mask);
        // mask |= (abs(p1 - p0) > limit) * -1;
        // mask |= (abs(q1 - q0) > limit) * -1;
        work = _mm_max_epu8(abs_diff(p2, p1), abs_diff(p3, p2));
        mask = _mm_max_epu8(work, mask);
        work = _mm_max_epu8(abs_diff(q2, q1), abs_diff(q3, q2));
        mask = _mm_max_epu8(work, mask);
        mask = _mm_subs_epu8(mask, limit_v);
        mask = _mm_cmpeq_epi8(mask, zero);
    }
    if 0xffff == _mm_movemask_epi8(_mm_cmpeq_epi8(mask, zero)) {
        return;
    }
    {
        let mut work: __m128i;
        work = _mm_max_epu8(abs_diff(p2, p0), abs_diff(q2, q0));
        flat = _mm_max_epu8(work, max_abs_p1p0q1q0);
        work = _mm_max_epu8(abs_diff(p3, p0), abs_diff(q3, q0));
        flat = _mm_max_epu8(work, flat);
        work = _mm_max_epu8(abs_diff(p4, p0), abs_diff(q4, q0));
        flat = _mm_subs_epu8(flat, one);
        flat = _mm_cmpeq_epi8(flat, zero);
        flat = _mm_and_si128(flat, mask);
        flat2 = _mm_max_epu8(abs_diff(p5, p0), abs_diff(q5, q0));
        flat2 = _mm_max_epu8(work, flat2);
        work = _mm_max_epu8(abs_diff(p6, p0), abs_diff(q6, q0));
        flat2 = _mm_max_epu8(work, flat2);
        flat2 = _mm_subs_epu8(flat2, one);
        flat2 = _mm_cmpeq_epi8(flat2, zero);
        flat2 = _mm_and_si128(flat2, flat);
    } // flat2 & flat & mask
      // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
      // filter4
    {
        let t4: __m128i = _mm_set1_epi8(4);
        let t3: __m128i = _mm_set1_epi8(3);
        let t80: __m128i = _mm_set1_epi8(0x80_i32 as i8);
        let te0: __m128i = _mm_set1_epi8(0xe0_i32 as i8);
        let t1f: __m128i = _mm_set1_epi8(0x1f);
        let t1: __m128i = _mm_set1_epi8(0x1);
        let t7f: __m128i = _mm_set1_epi8(0x7f);
        let ff: __m128i = _mm_cmpeq_epi8(t4, t4);
        let mut filt: __m128i;
        let mut work_a: __m128i;
        let mut filter1: __m128i;
        let mut filter2: __m128i;
        op1 = _mm_xor_si128(p1, t80);
        op0 = _mm_xor_si128(p0, t80);
        oq0 = _mm_xor_si128(q0, t80);
        oq1 = _mm_xor_si128(q1, t80);
        hev = _mm_subs_epu8(max_abs_p1p0q1q0, thresh_v);
        hev = _mm_xor_si128(_mm_cmpeq_epi8(hev, zero), ff);
        filt = _mm_and_si128(_mm_subs_epi8(op1, oq1), hev);
        work_a = _mm_subs_epi8(oq0, op0);
        filt = _mm_adds_epi8(filt, work_a);
        filt = _mm_adds_epi8(filt, work_a);
        filt = _mm_adds_epi8(filt, work_a);
        filt = _mm_and_si128(filt, mask);
        filter1 = _mm_adds_epi8(filt, t4);
        filter2 = _mm_adds_epi8(filt, t3);
        work_a = _mm_cmpgt_epi8(zero, filter1);
        filter1 = _mm_srli_epi16::<{ 3 }>(filter1);
        work_a = _mm_and_si128(work_a, te0);
        filter1 = _mm_and_si128(filter1, t1f);
        filter1 = _mm_or_si128(filter1, work_a);
        oq0 = _mm_xor_si128(_mm_subs_epi8(oq0, filter1), t80);
        work_a = _mm_cmpgt_epi8(zero, filter2);
        filter2 = _mm_srli_epi16::<{ 3 }>(filter2);
        work_a = _mm_and_si128(work_a, te0);
        filter2 = _mm_and_si128(filter2, t1f);
        filter2 = _mm_or_si128(filter2, work_a);
        op0 = _mm_xor_si128(_mm_adds_epi8(op0, filter2), t80);
        filt = _mm_adds_epi8(filter1, t1);
        work_a = _mm_cmpgt_epi8(zero, filt);
        filt = _mm_srli_epi16::<{ 1 }>(filt);
        work_a = _mm_and_si128(work_a, t80);
        filt = _mm_and_si128(filt, t7f);
        filt = _mm_or_si128(filt, work_a);
        filt = _mm_andnot_si128(hev, filt);
        op1 = _mm_xor_si128(_mm_adds_epi8(op1, filt), t80);
        oq1 = _mm_xor_si128(_mm_subs_epi8(oq1, filt), t80);
        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        // filter8
        if 0xffff != _mm_movemask_epi8(_mm_cmpeq_epi8(flat, zero)) {
            let four: __m128i = _mm_set1_epi16(4);
            let p3_lo: __m128i = _mm_unpacklo_epi8(p3, zero);
            let p2_lo: __m128i = _mm_unpacklo_epi8(p2, zero);
            let p1_lo: __m128i = _mm_unpacklo_epi8(p1, zero);
            let p0_lo: __m128i = _mm_unpacklo_epi8(p0, zero);
            let q0_lo: __m128i = _mm_unpacklo_epi8(q0, zero);
            let q1_lo: __m128i = _mm_unpacklo_epi8(q1, zero);
            let q2_lo: __m128i = _mm_unpacklo_epi8(q2, zero);
            let q3_lo: __m128i = _mm_unpacklo_epi8(q3, zero);
            let p3_hi: __m128i = _mm_unpackhi_epi8(p3, zero);
            let p2_hi: __m128i = _mm_unpackhi_epi8(p2, zero);
            let p1_hi: __m128i = _mm_unpackhi_epi8(p1, zero);
            let p0_hi: __m128i = _mm_unpackhi_epi8(p0, zero);
            let q0_hi: __m128i = _mm_unpackhi_epi8(q0, zero);
            let q1_hi: __m128i = _mm_unpackhi_epi8(q1, zero);
            let q2_hi: __m128i = _mm_unpackhi_epi8(q2, zero);
            let q3_hi: __m128i = _mm_unpackhi_epi8(q3, zero);
            let mut f8_lo: __m128i = core::mem::zeroed();
            let mut f8_hi: __m128i = core::mem::zeroed();
            f8_lo = _mm_add_epi16(_mm_add_epi16(p3_lo, four), _mm_add_epi16(p3_lo, p2_lo));
            f8_lo = _mm_add_epi16(_mm_add_epi16(p3_lo, f8_lo), _mm_add_epi16(p2_lo, p1_lo));
            f8_lo = _mm_add_epi16(_mm_add_epi16(p0_lo, q0_lo), f8_lo);
            f8_hi = _mm_add_epi16(_mm_add_epi16(p3_hi, four), _mm_add_epi16(p3_hi, p2_hi));
            f8_hi = _mm_add_epi16(_mm_add_epi16(p3_hi, f8_hi), _mm_add_epi16(p2_hi, p1_hi));
            f8_hi = _mm_add_epi16(_mm_add_epi16(p0_hi, q0_hi), f8_hi);
            op2 = filter8_mask(
                &raw mut flat as *const __m128i,
                &raw mut p2 as *const __m128i,
                &raw mut f8_lo as *const __m128i,
                &raw mut f8_hi as *const __m128i,
            );
            f8_lo = filter_add2_sub2(
                &raw mut f8_lo as *const __m128i,
                &raw const q1_lo,
                &raw const p1_lo,
                &raw const p2_lo,
                &raw const p3_lo,
            );
            f8_hi = filter_add2_sub2(
                &raw mut f8_hi as *const __m128i,
                &raw const q1_hi,
                &raw const p1_hi,
                &raw const p2_hi,
                &raw const p3_hi,
            );
            op1 = filter8_mask(
                &raw mut flat as *const __m128i,
                &raw mut op1 as *const __m128i,
                &raw mut f8_lo as *const __m128i,
                &raw mut f8_hi as *const __m128i,
            );
            f8_lo = filter_add2_sub2(
                &raw mut f8_lo as *const __m128i,
                &raw const q2_lo,
                &raw const p0_lo,
                &raw const p1_lo,
                &raw const p3_lo,
            );
            f8_hi = filter_add2_sub2(
                &raw mut f8_hi as *const __m128i,
                &raw const q2_hi,
                &raw const p0_hi,
                &raw const p1_hi,
                &raw const p3_hi,
            );
            op0 = filter8_mask(
                &raw mut flat as *const __m128i,
                &raw mut op0 as *const __m128i,
                &raw mut f8_lo as *const __m128i,
                &raw mut f8_hi as *const __m128i,
            );
            f8_lo = filter_add2_sub2(
                &raw mut f8_lo as *const __m128i,
                &raw const q3_lo,
                &raw const q0_lo,
                &raw const p0_lo,
                &raw const p3_lo,
            );
            f8_hi = filter_add2_sub2(
                &raw mut f8_hi as *const __m128i,
                &raw const q3_hi,
                &raw const q0_hi,
                &raw const p0_hi,
                &raw const p3_hi,
            );
            oq0 = filter8_mask(
                &raw mut flat as *const __m128i,
                &raw mut oq0 as *const __m128i,
                &raw mut f8_lo as *const __m128i,
                &raw mut f8_hi as *const __m128i,
            );
            f8_lo = filter_add2_sub2(
                &raw mut f8_lo as *const __m128i,
                &raw const q3_lo,
                &raw const q1_lo,
                &raw const q0_lo,
                &raw const p2_lo,
            );
            f8_hi = filter_add2_sub2(
                &raw mut f8_hi as *const __m128i,
                &raw const q3_hi,
                &raw const q1_hi,
                &raw const q0_hi,
                &raw const p2_hi,
            );
            oq1 = filter8_mask(
                &raw mut flat as *const __m128i,
                &raw mut oq1 as *const __m128i,
                &raw mut f8_lo as *const __m128i,
                &raw mut f8_hi as *const __m128i,
            );
            f8_lo = filter_add2_sub2(
                &raw mut f8_lo as *const __m128i,
                &raw const q3_lo,
                &raw const q2_lo,
                &raw const q1_lo,
                &raw const p1_lo,
            );
            f8_hi = filter_add2_sub2(
                &raw mut f8_hi as *const __m128i,
                &raw const q3_hi,
                &raw const q2_hi,
                &raw const q1_hi,
                &raw const p1_hi,
            );
            oq2 = filter8_mask(
                &raw mut flat as *const __m128i,
                &raw mut q2 as *const __m128i,
                &raw mut f8_lo as *const __m128i,
                &raw mut f8_hi as *const __m128i,
            );
            // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            // wide flat calculations
            if 0xffff != _mm_movemask_epi8(_mm_cmpeq_epi8(flat2, zero)) {
                let eight: __m128i = _mm_set1_epi16(8);
                let p6_lo: __m128i = _mm_unpacklo_epi8(p6, zero);
                let p5_lo: __m128i = _mm_unpacklo_epi8(p5, zero);
                let p4_lo: __m128i = _mm_unpacklo_epi8(p4, zero);
                let q4_lo: __m128i = _mm_unpacklo_epi8(q4, zero);
                let q5_lo: __m128i = _mm_unpacklo_epi8(q5, zero);
                let q6_lo: __m128i = _mm_unpacklo_epi8(q6, zero);
                let p6_hi: __m128i = _mm_unpackhi_epi8(p6, zero);
                let p5_hi: __m128i = _mm_unpackhi_epi8(p5, zero);
                let p4_hi: __m128i = _mm_unpackhi_epi8(p4, zero);
                let q4_hi: __m128i = _mm_unpackhi_epi8(q4, zero);
                let q5_hi: __m128i = _mm_unpackhi_epi8(q5, zero);
                let q6_hi: __m128i = _mm_unpackhi_epi8(q6, zero);
                let mut f_lo: __m128i = core::mem::zeroed();
                let mut f_hi: __m128i = core::mem::zeroed();
                f_lo = _mm_sub_epi16(_mm_slli_epi16::<{ 3 }>(p6_lo), p6_lo);
                f_lo = _mm_add_epi16(_mm_slli_epi16::<{ 1 }>(p5_lo), f_lo);
                f_lo = _mm_add_epi16(_mm_slli_epi16::<{ 1 }>(p4_lo), f_lo);
                f_lo = _mm_add_epi16(_mm_add_epi16(p3_lo, f_lo), _mm_add_epi16(p2_lo, p1_lo));
                f_lo = _mm_add_epi16(_mm_add_epi16(p0_lo, q0_lo), f_lo);
                f_lo = _mm_add_epi16(f_lo, eight);
                f_hi = _mm_sub_epi16(_mm_slli_epi16::<{ 3 }>(p6_hi), p6_hi);
                f_hi = _mm_add_epi16(_mm_slli_epi16::<{ 1 }>(p5_hi), f_hi);
                f_hi = _mm_add_epi16(_mm_slli_epi16::<{ 1 }>(p4_hi), f_hi);
                f_hi = _mm_add_epi16(_mm_add_epi16(p3_hi, f_hi), _mm_add_epi16(p2_hi, p1_hi));
                f_hi = _mm_add_epi16(_mm_add_epi16(p0_hi, q0_hi), f_hi);
                f_hi = _mm_add_epi16(f_hi, eight);
                p5 = filter16_mask(
                    &raw mut flat2 as *const __m128i,
                    &raw mut p5 as *const __m128i,
                    &raw mut f_lo as *const __m128i,
                    &raw mut f_hi as *const __m128i,
                );
                _mm_storeu_si128(s.offset(-((6 * p) as isize)) as *mut __m128i, p5);
                f_lo = filter_add2_sub2(
                    &raw mut f_lo as *const __m128i,
                    &raw const q1_lo,
                    &raw const p3_lo,
                    &raw const p6_lo,
                    &raw const p6_lo,
                );
                f_hi = filter_add2_sub2(
                    &raw mut f_hi as *const __m128i,
                    &raw const q1_hi,
                    &raw const p3_hi,
                    &raw const p6_hi,
                    &raw const p6_hi,
                );
                p4 = filter16_mask(
                    &raw mut flat2 as *const __m128i,
                    &raw mut p4 as *const __m128i,
                    &raw mut f_lo as *const __m128i,
                    &raw mut f_hi as *const __m128i,
                );
                _mm_storeu_si128(s.offset(-((5 * p) as isize)) as *mut __m128i, p4);
                f_lo = filter_add2_sub2(
                    &raw mut f_lo as *const __m128i,
                    &raw const q2_lo,
                    &raw const p2_lo,
                    &raw const p6_lo,
                    &raw const p5_lo,
                );
                f_hi = filter_add2_sub2(
                    &raw mut f_hi as *const __m128i,
                    &raw const q2_hi,
                    &raw const p2_hi,
                    &raw const p6_hi,
                    &raw const p5_hi,
                );
                p3 = filter16_mask(
                    &raw mut flat2 as *const __m128i,
                    &raw mut p3 as *const __m128i,
                    &raw mut f_lo as *const __m128i,
                    &raw mut f_hi as *const __m128i,
                );
                _mm_storeu_si128(s.offset(-((4 * p) as isize)) as *mut __m128i, p3);
                f_lo = filter_add2_sub2(
                    &raw mut f_lo as *const __m128i,
                    &raw const q3_lo,
                    &raw const p1_lo,
                    &raw const p6_lo,
                    &raw const p4_lo,
                );
                f_hi = filter_add2_sub2(
                    &raw mut f_hi as *const __m128i,
                    &raw const q3_hi,
                    &raw const p1_hi,
                    &raw const p6_hi,
                    &raw const p4_hi,
                );
                op2 = filter16_mask(
                    &raw mut flat2 as *const __m128i,
                    &raw mut op2 as *const __m128i,
                    &raw mut f_lo as *const __m128i,
                    &raw mut f_hi as *const __m128i,
                );
                _mm_storeu_si128(s.offset(-((3 * p) as isize)) as *mut __m128i, op2);
                f_lo = filter_add2_sub2(
                    &raw mut f_lo as *const __m128i,
                    &raw const q4_lo,
                    &raw const p0_lo,
                    &raw const p6_lo,
                    &raw const p3_lo,
                );
                f_hi = filter_add2_sub2(
                    &raw mut f_hi as *const __m128i,
                    &raw const q4_hi,
                    &raw const p0_hi,
                    &raw const p6_hi,
                    &raw const p3_hi,
                );
                op1 = filter16_mask(
                    &raw mut flat2 as *const __m128i,
                    &raw mut op1 as *const __m128i,
                    &raw mut f_lo as *const __m128i,
                    &raw mut f_hi as *const __m128i,
                );
                _mm_storeu_si128(s.offset(-((2 * p) as isize)) as *mut __m128i, op1);
                f_lo = filter_add2_sub2(
                    &raw mut f_lo as *const __m128i,
                    &raw const q5_lo,
                    &raw const q0_lo,
                    &raw const p6_lo,
                    &raw const p2_lo,
                );
                f_hi = filter_add2_sub2(
                    &raw mut f_hi as *const __m128i,
                    &raw const q5_hi,
                    &raw const q0_hi,
                    &raw const p6_hi,
                    &raw const p2_hi,
                );
                op0 = filter16_mask(
                    &raw mut flat2 as *const __m128i,
                    &raw mut op0 as *const __m128i,
                    &raw mut f_lo as *const __m128i,
                    &raw mut f_hi as *const __m128i,
                );
                _mm_storeu_si128(s.offset(-((1 * p) as isize)) as *mut __m128i, op0);
                f_lo = filter_add2_sub2(
                    &raw mut f_lo as *const __m128i,
                    &raw const q6_lo,
                    &raw const q1_lo,
                    &raw const p6_lo,
                    &raw const p1_lo,
                );
                f_hi = filter_add2_sub2(
                    &raw mut f_hi as *const __m128i,
                    &raw const q6_hi,
                    &raw const q1_hi,
                    &raw const p6_hi,
                    &raw const p1_hi,
                );
                oq0 = filter16_mask(
                    &raw mut flat2 as *const __m128i,
                    &raw mut oq0 as *const __m128i,
                    &raw mut f_lo as *const __m128i,
                    &raw mut f_hi as *const __m128i,
                );
                _mm_storeu_si128(s.offset(-((0 * p) as isize)) as *mut __m128i, oq0);
                f_lo = filter_add2_sub2(
                    &raw mut f_lo as *const __m128i,
                    &raw const q6_lo,
                    &raw const q2_lo,
                    &raw const p5_lo,
                    &raw const p0_lo,
                );
                f_hi = filter_add2_sub2(
                    &raw mut f_hi as *const __m128i,
                    &raw const q6_hi,
                    &raw const q2_hi,
                    &raw const p5_hi,
                    &raw const p0_hi,
                );
                oq1 = filter16_mask(
                    &raw mut flat2 as *const __m128i,
                    &raw mut oq1 as *const __m128i,
                    &raw mut f_lo as *const __m128i,
                    &raw mut f_hi as *const __m128i,
                );
                _mm_storeu_si128(s.offset((1 * p) as isize) as *mut __m128i, oq1);
                f_lo = filter_add2_sub2(
                    &raw mut f_lo as *const __m128i,
                    &raw const q6_lo,
                    &raw const q3_lo,
                    &raw const p4_lo,
                    &raw const q0_lo,
                );
                f_hi = filter_add2_sub2(
                    &raw mut f_hi as *const __m128i,
                    &raw const q6_hi,
                    &raw const q3_hi,
                    &raw const p4_hi,
                    &raw const q0_hi,
                );
                oq2 = filter16_mask(
                    &raw mut flat2 as *const __m128i,
                    &raw mut oq2 as *const __m128i,
                    &raw mut f_lo as *const __m128i,
                    &raw mut f_hi as *const __m128i,
                );
                _mm_storeu_si128(s.offset((2 * p) as isize) as *mut __m128i, oq2);
                f_lo = filter_add2_sub2(
                    &raw mut f_lo as *const __m128i,
                    &raw const q6_lo,
                    &raw const q4_lo,
                    &raw const p3_lo,
                    &raw const q1_lo,
                );
                f_hi = filter_add2_sub2(
                    &raw mut f_hi as *const __m128i,
                    &raw const q6_hi,
                    &raw const q4_hi,
                    &raw const p3_hi,
                    &raw const q1_hi,
                );
                q3 = filter16_mask(
                    &raw mut flat2 as *const __m128i,
                    &raw mut q3 as *const __m128i,
                    &raw mut f_lo as *const __m128i,
                    &raw mut f_hi as *const __m128i,
                );
                _mm_storeu_si128(s.offset((3 * p) as isize) as *mut __m128i, q3);
                f_lo = filter_add2_sub2(
                    &raw mut f_lo as *const __m128i,
                    &raw const q6_lo,
                    &raw const q5_lo,
                    &raw const p2_lo,
                    &raw const q2_lo,
                );
                f_hi = filter_add2_sub2(
                    &raw mut f_hi as *const __m128i,
                    &raw const q6_hi,
                    &raw const q5_hi,
                    &raw const p2_hi,
                    &raw const q2_hi,
                );
                q4 = filter16_mask(
                    &raw mut flat2 as *const __m128i,
                    &raw mut q4 as *const __m128i,
                    &raw mut f_lo as *const __m128i,
                    &raw mut f_hi as *const __m128i,
                );
                _mm_storeu_si128(s.offset((4 * p) as isize) as *mut __m128i, q4);
                f_lo = filter_add2_sub2(
                    &raw mut f_lo as *const __m128i,
                    &raw const q6_lo,
                    &raw const q6_lo,
                    &raw const p1_lo,
                    &raw const q3_lo,
                );
                f_hi = filter_add2_sub2(
                    &raw mut f_hi as *const __m128i,
                    &raw const q6_hi,
                    &raw const q6_hi,
                    &raw const p1_hi,
                    &raw const q3_hi,
                );
                q5 = filter16_mask(
                    &raw mut flat2 as *const __m128i,
                    &raw mut q5 as *const __m128i,
                    &raw mut f_lo as *const __m128i,
                    &raw mut f_hi as *const __m128i,
                );
                _mm_storeu_si128(s.offset((5 * p) as isize) as *mut __m128i, q5);
            } else {
                _mm_storeu_si128(s.offset(-((3 * p) as isize)) as *mut __m128i, op2);
                _mm_storeu_si128(s.offset(-((2 * p) as isize)) as *mut __m128i, op1);
                _mm_storeu_si128(s.offset(-((1 * p) as isize)) as *mut __m128i, op0);
                _mm_storeu_si128(s.offset(-((0 * p) as isize)) as *mut __m128i, oq0);
                _mm_storeu_si128(s.offset((1 * p) as isize) as *mut __m128i, oq1);
                _mm_storeu_si128(s.offset((2 * p) as isize) as *mut __m128i, oq2);
            }
        } else {
            _mm_storeu_si128(s.offset(-((2 * p) as isize)) as *mut __m128i, op1);
            _mm_storeu_si128(s.offset(-((1 * p) as isize)) as *mut __m128i, op0);
            _mm_storeu_si128(s.offset(-((0 * p) as isize)) as *mut __m128i, oq0);
            _mm_storeu_si128(s.offset((1 * p) as isize) as *mut __m128i, oq1);
        }
    }
}

#[no_mangle]
#[target_feature(enable = "sse2")]
pub unsafe extern "C-unwind" fn aom_lpf_horizontal_8_quad_sse2(
    s: *mut u8,
    p: i32,
    _blimit0: *const u8,
    _limit0: *const u8,
    _thresh0: *const u8,
) {
    let zero: __m128i = _mm_setzero_si128();
    let one: __m128i = _mm_set1_epi8(1);
    let blimit_v: __m128i = _mm_load_si128(_blimit0 as *const __m128i);
    let limit_v: __m128i = _mm_load_si128(_limit0 as *const __m128i);
    let thresh_v: __m128i = _mm_load_si128(_thresh0 as *const __m128i);
    let mut mask: __m128i;
    let mut hev: __m128i;
    let mut flat: __m128i = core::mem::zeroed();
    let mut p3: __m128i;
    let mut p2: __m128i = core::mem::zeroed();
    let mut p1: __m128i;
    let mut p0: __m128i;
    let mut q0: __m128i;
    let mut q1: __m128i;
    let mut q2: __m128i = core::mem::zeroed();
    let mut q3: __m128i;
    let mut op2: __m128i;
    let mut op1: __m128i = core::mem::zeroed();
    let mut op0: __m128i = core::mem::zeroed();
    let mut oq0: __m128i = core::mem::zeroed();
    let mut oq1: __m128i = core::mem::zeroed();
    let mut oq2: __m128i;
    let mut max_abs_p1p0q1q0: __m128i;
    p3 = _mm_loadu_si128(s.offset(-((4 * p) as isize)) as *mut __m128i as *const __m128i);
    p2 = _mm_loadu_si128(s.offset(-((3 * p) as isize)) as *mut __m128i as *const __m128i);
    p1 = _mm_loadu_si128(s.offset(-((2 * p) as isize)) as *mut __m128i as *const __m128i);
    p0 = _mm_loadu_si128(s.offset(-((1 * p) as isize)) as *mut __m128i as *const __m128i);
    q0 = _mm_loadu_si128(s.offset(-((0 * p) as isize)) as *mut __m128i as *const __m128i);
    q1 = _mm_loadu_si128(s.offset((1 * p) as isize) as *mut __m128i as *const __m128i);
    q2 = _mm_loadu_si128(s.offset((2 * p) as isize) as *mut __m128i as *const __m128i);
    q3 = _mm_loadu_si128(s.offset((3 * p) as isize) as *mut __m128i as *const __m128i);
    {
        let abs_p1p0: __m128i = abs_diff(p1, p0);
        let abs_q1q0: __m128i = abs_diff(q1, q0);
        let fe: __m128i = _mm_set1_epi8(0xfe_i32 as i8);
        let ff: __m128i = _mm_cmpeq_epi8(zero, zero);
        let mut abs_p0q0: __m128i = abs_diff(p0, q0);
        let mut abs_p1q1: __m128i = abs_diff(p1, q1);
        let mut work: __m128i;
        max_abs_p1p0q1q0 = _mm_max_epu8(abs_p1p0, abs_q1q0);
        abs_p0q0 = _mm_adds_epu8(abs_p0q0, abs_p0q0);
        abs_p1q1 = _mm_srli_epi16::<{ 1 }>(_mm_and_si128(abs_p1q1, fe));
        mask = _mm_subs_epu8(_mm_adds_epu8(abs_p0q0, abs_p1q1), blimit_v);
        mask = _mm_xor_si128(_mm_cmpeq_epi8(mask, zero), ff);
        // mask |= (abs(p0 - q0) * 2 + abs(p1 - q1) / 2  > blimit) * -1;
        mask = _mm_max_epu8(max_abs_p1p0q1q0, mask);
        // mask |= (abs(p1 - p0) > limit) * -1;
        // mask |= (abs(q1 - q0) > limit) * -1;
        work = _mm_max_epu8(abs_diff(p2, p1), abs_diff(p3, p2));
        mask = _mm_max_epu8(work, mask);
        work = _mm_max_epu8(abs_diff(q2, q1), abs_diff(q3, q2));
        mask = _mm_max_epu8(work, mask);
        mask = _mm_subs_epu8(mask, limit_v);
        mask = _mm_cmpeq_epi8(mask, zero);
    }
    if 0xffff == _mm_movemask_epi8(_mm_cmpeq_epi8(mask, zero)) {
        return;
    }
    {
        let mut work: __m128i;
        work = _mm_max_epu8(abs_diff(p2, p0), abs_diff(q2, q0));
        flat = _mm_max_epu8(work, max_abs_p1p0q1q0);
        work = _mm_max_epu8(abs_diff(p3, p0), abs_diff(q3, q0));
        flat = _mm_max_epu8(work, flat);
        flat = _mm_subs_epu8(flat, one);
        flat = _mm_cmpeq_epi8(flat, zero);
        flat = _mm_and_si128(flat, mask);
    }
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    // filter4
    {
        let t4: __m128i = _mm_set1_epi8(4);
        let t3: __m128i = _mm_set1_epi8(3);
        let t80: __m128i = _mm_set1_epi8(0x80_i32 as i8);
        let te0: __m128i = _mm_set1_epi8(0xe0_i32 as i8);
        let t1f: __m128i = _mm_set1_epi8(0x1f);
        let t1: __m128i = _mm_set1_epi8(0x1);
        let t7f: __m128i = _mm_set1_epi8(0x7f);
        let ff: __m128i = _mm_cmpeq_epi8(t4, t4);
        let mut filt: __m128i;
        let mut work_a: __m128i;
        let mut filter1: __m128i;
        let mut filter2: __m128i;
        op1 = _mm_xor_si128(p1, t80);
        op0 = _mm_xor_si128(p0, t80);
        oq0 = _mm_xor_si128(q0, t80);
        oq1 = _mm_xor_si128(q1, t80);
        hev = _mm_subs_epu8(max_abs_p1p0q1q0, thresh_v);
        hev = _mm_xor_si128(_mm_cmpeq_epi8(hev, zero), ff);
        filt = _mm_and_si128(_mm_subs_epi8(op1, oq1), hev);
        work_a = _mm_subs_epi8(oq0, op0);
        filt = _mm_adds_epi8(filt, work_a);
        filt = _mm_adds_epi8(filt, work_a);
        filt = _mm_adds_epi8(filt, work_a);
        filt = _mm_and_si128(filt, mask);
        filter1 = _mm_adds_epi8(filt, t4);
        filter2 = _mm_adds_epi8(filt, t3);
        work_a = _mm_cmpgt_epi8(zero, filter1);
        filter1 = _mm_srli_epi16::<{ 3 }>(filter1);
        work_a = _mm_and_si128(work_a, te0);
        filter1 = _mm_and_si128(filter1, t1f);
        filter1 = _mm_or_si128(filter1, work_a);
        oq0 = _mm_xor_si128(_mm_subs_epi8(oq0, filter1), t80);
        work_a = _mm_cmpgt_epi8(zero, filter2);
        filter2 = _mm_srli_epi16::<{ 3 }>(filter2);
        work_a = _mm_and_si128(work_a, te0);
        filter2 = _mm_and_si128(filter2, t1f);
        filter2 = _mm_or_si128(filter2, work_a);
        op0 = _mm_xor_si128(_mm_adds_epi8(op0, filter2), t80);
        filt = _mm_adds_epi8(filter1, t1);
        work_a = _mm_cmpgt_epi8(zero, filt);
        filt = _mm_srli_epi16::<{ 1 }>(filt);
        work_a = _mm_and_si128(work_a, t80);
        filt = _mm_and_si128(filt, t7f);
        filt = _mm_or_si128(filt, work_a);
        filt = _mm_andnot_si128(hev, filt);
        op1 = _mm_xor_si128(_mm_adds_epi8(op1, filt), t80);
        oq1 = _mm_xor_si128(_mm_subs_epi8(oq1, filt), t80);
        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        // filter8
        if 0xffff != _mm_movemask_epi8(_mm_cmpeq_epi8(flat, zero)) {
            let four: __m128i = _mm_set1_epi16(4);
            let p3_lo: __m128i = _mm_unpacklo_epi8(p3, zero);
            let p2_lo: __m128i = _mm_unpacklo_epi8(p2, zero);
            let p1_lo: __m128i = _mm_unpacklo_epi8(p1, zero);
            let p0_lo: __m128i = _mm_unpacklo_epi8(p0, zero);
            let q0_lo: __m128i = _mm_unpacklo_epi8(q0, zero);
            let q1_lo: __m128i = _mm_unpacklo_epi8(q1, zero);
            let q2_lo: __m128i = _mm_unpacklo_epi8(q2, zero);
            let q3_lo: __m128i = _mm_unpacklo_epi8(q3, zero);
            let p3_hi: __m128i = _mm_unpackhi_epi8(p3, zero);
            let p2_hi: __m128i = _mm_unpackhi_epi8(p2, zero);
            let p1_hi: __m128i = _mm_unpackhi_epi8(p1, zero);
            let p0_hi: __m128i = _mm_unpackhi_epi8(p0, zero);
            let q0_hi: __m128i = _mm_unpackhi_epi8(q0, zero);
            let q1_hi: __m128i = _mm_unpackhi_epi8(q1, zero);
            let q2_hi: __m128i = _mm_unpackhi_epi8(q2, zero);
            let q3_hi: __m128i = _mm_unpackhi_epi8(q3, zero);
            let mut f8_lo: __m128i = core::mem::zeroed();
            let mut f8_hi: __m128i = core::mem::zeroed();
            f8_lo = _mm_add_epi16(_mm_add_epi16(p3_lo, four), _mm_add_epi16(p3_lo, p2_lo));
            f8_lo = _mm_add_epi16(_mm_add_epi16(p3_lo, f8_lo), _mm_add_epi16(p2_lo, p1_lo));
            f8_lo = _mm_add_epi16(_mm_add_epi16(p0_lo, q0_lo), f8_lo);
            f8_hi = _mm_add_epi16(_mm_add_epi16(p3_hi, four), _mm_add_epi16(p3_hi, p2_hi));
            f8_hi = _mm_add_epi16(_mm_add_epi16(p3_hi, f8_hi), _mm_add_epi16(p2_hi, p1_hi));
            f8_hi = _mm_add_epi16(_mm_add_epi16(p0_hi, q0_hi), f8_hi);
            op2 = filter8_mask(
                &raw mut flat as *const __m128i,
                &raw mut p2 as *const __m128i,
                &raw mut f8_lo as *const __m128i,
                &raw mut f8_hi as *const __m128i,
            );
            _mm_storeu_si128(s.offset(-((3 * p) as isize)) as *mut __m128i, op2);
            f8_lo = filter_add2_sub2(
                &raw mut f8_lo as *const __m128i,
                &raw const q1_lo,
                &raw const p1_lo,
                &raw const p2_lo,
                &raw const p3_lo,
            );
            f8_hi = filter_add2_sub2(
                &raw mut f8_hi as *const __m128i,
                &raw const q1_hi,
                &raw const p1_hi,
                &raw const p2_hi,
                &raw const p3_hi,
            );
            op1 = filter8_mask(
                &raw mut flat as *const __m128i,
                &raw mut op1 as *const __m128i,
                &raw mut f8_lo as *const __m128i,
                &raw mut f8_hi as *const __m128i,
            );
            _mm_storeu_si128(s.offset(-((2 * p) as isize)) as *mut __m128i, op1);
            f8_lo = filter_add2_sub2(
                &raw mut f8_lo as *const __m128i,
                &raw const q2_lo,
                &raw const p0_lo,
                &raw const p1_lo,
                &raw const p3_lo,
            );
            f8_hi = filter_add2_sub2(
                &raw mut f8_hi as *const __m128i,
                &raw const q2_hi,
                &raw const p0_hi,
                &raw const p1_hi,
                &raw const p3_hi,
            );
            op0 = filter8_mask(
                &raw mut flat as *const __m128i,
                &raw mut op0 as *const __m128i,
                &raw mut f8_lo as *const __m128i,
                &raw mut f8_hi as *const __m128i,
            );
            _mm_storeu_si128(s.offset(-((1 * p) as isize)) as *mut __m128i, op0);
            f8_lo = filter_add2_sub2(
                &raw mut f8_lo as *const __m128i,
                &raw const q3_lo,
                &raw const q0_lo,
                &raw const p0_lo,
                &raw const p3_lo,
            );
            f8_hi = filter_add2_sub2(
                &raw mut f8_hi as *const __m128i,
                &raw const q3_hi,
                &raw const q0_hi,
                &raw const p0_hi,
                &raw const p3_hi,
            );
            oq0 = filter8_mask(
                &raw mut flat as *const __m128i,
                &raw mut oq0 as *const __m128i,
                &raw mut f8_lo as *const __m128i,
                &raw mut f8_hi as *const __m128i,
            );
            _mm_storeu_si128(s.offset(-((0 * p) as isize)) as *mut __m128i, oq0);
            f8_lo = filter_add2_sub2(
                &raw mut f8_lo as *const __m128i,
                &raw const q3_lo,
                &raw const q1_lo,
                &raw const q0_lo,
                &raw const p2_lo,
            );
            f8_hi = filter_add2_sub2(
                &raw mut f8_hi as *const __m128i,
                &raw const q3_hi,
                &raw const q1_hi,
                &raw const q0_hi,
                &raw const p2_hi,
            );
            oq1 = filter8_mask(
                &raw mut flat as *const __m128i,
                &raw mut oq1 as *const __m128i,
                &raw mut f8_lo as *const __m128i,
                &raw mut f8_hi as *const __m128i,
            );
            _mm_storeu_si128(s.offset((1 * p) as isize) as *mut __m128i, oq1);
            f8_lo = filter_add2_sub2(
                &raw mut f8_lo as *const __m128i,
                &raw const q3_lo,
                &raw const q2_lo,
                &raw const q1_lo,
                &raw const p1_lo,
            );
            f8_hi = filter_add2_sub2(
                &raw mut f8_hi as *const __m128i,
                &raw const q3_hi,
                &raw const q2_hi,
                &raw const q1_hi,
                &raw const p1_hi,
            );
            oq2 = filter8_mask(
                &raw mut flat as *const __m128i,
                &raw mut q2 as *const __m128i,
                &raw mut f8_lo as *const __m128i,
                &raw mut f8_hi as *const __m128i,
            );
            _mm_storeu_si128(s.offset((2 * p) as isize) as *mut __m128i, oq2);
        } else {
            _mm_storeu_si128(s.offset(-((2 * p) as isize)) as *mut __m128i, op1);
            _mm_storeu_si128(s.offset(-((1 * p) as isize)) as *mut __m128i, op0);
            _mm_storeu_si128(s.offset(-((0 * p) as isize)) as *mut __m128i, oq0);
            _mm_storeu_si128(s.offset((1 * p) as isize) as *mut __m128i, oq1);
        }
    }
}

#[no_mangle]
#[target_feature(enable = "sse2")]
pub unsafe extern "C-unwind" fn aom_lpf_horizontal_6_quad_sse2(
    s: *mut u8,
    p: i32,
    _blimit0: *const u8,
    _limit0: *const u8,
    _thresh0: *const u8,
) {
    let zero: __m128i = _mm_setzero_si128();
    let one: __m128i = _mm_set1_epi8(1);
    let blimit_v: __m128i = _mm_load_si128(_blimit0 as *const __m128i);
    let limit_v: __m128i = _mm_load_si128(_limit0 as *const __m128i);
    let thresh_v: __m128i = _mm_load_si128(_thresh0 as *const __m128i);
    let mut mask: __m128i;
    let mut hev: __m128i;
    let mut flat: __m128i = core::mem::zeroed();
    let mut p2: __m128i;
    let mut p1: __m128i;
    let mut p0: __m128i;
    let mut q0: __m128i;
    let mut q1: __m128i;
    let mut q2: __m128i;
    let mut op1: __m128i = core::mem::zeroed();
    let mut op0: __m128i = core::mem::zeroed();
    let mut oq0: __m128i = core::mem::zeroed();
    let mut oq1: __m128i = core::mem::zeroed();
    let mut max_abs_p1p0q1q0: __m128i;
    p2 = _mm_loadu_si128(s.offset(-((3 * p) as isize)) as *mut __m128i as *const __m128i);
    p1 = _mm_loadu_si128(s.offset(-((2 * p) as isize)) as *mut __m128i as *const __m128i);
    p0 = _mm_loadu_si128(s.offset(-((1 * p) as isize)) as *mut __m128i as *const __m128i);
    q0 = _mm_loadu_si128(s.offset(-((0 * p) as isize)) as *mut __m128i as *const __m128i);
    q1 = _mm_loadu_si128(s.offset((1 * p) as isize) as *mut __m128i as *const __m128i);
    q2 = _mm_loadu_si128(s.offset((2 * p) as isize) as *mut __m128i as *const __m128i);
    {
        let abs_p1p0: __m128i = abs_diff(p1, p0);
        let abs_q1q0: __m128i = abs_diff(q1, q0);
        let fe: __m128i = _mm_set1_epi8(0xfe_i32 as i8);
        let ff: __m128i = _mm_cmpeq_epi8(zero, zero);
        let mut abs_p0q0: __m128i = abs_diff(p0, q0);
        let mut abs_p1q1: __m128i = abs_diff(p1, q1);
        let mut work: __m128i;
        max_abs_p1p0q1q0 = _mm_max_epu8(abs_p1p0, abs_q1q0);
        abs_p0q0 = _mm_adds_epu8(abs_p0q0, abs_p0q0);
        abs_p1q1 = _mm_srli_epi16::<{ 1 }>(_mm_and_si128(abs_p1q1, fe));
        mask = _mm_subs_epu8(_mm_adds_epu8(abs_p0q0, abs_p1q1), blimit_v);
        mask = _mm_xor_si128(_mm_cmpeq_epi8(mask, zero), ff);
        // mask |= (abs(p0 - q0) * 2 + abs(p1 - q1) / 2  > blimit) * -1;
        mask = _mm_max_epu8(max_abs_p1p0q1q0, mask);
        // mask |= (abs(p1 - p0) > limit) * -1;
        // mask |= (abs(q1 - q0) > limit) * -1;
        work = _mm_max_epu8(abs_diff(p2, p1), abs_diff(q2, q1));
        mask = _mm_max_epu8(work, mask);
        mask = _mm_subs_epu8(mask, limit_v);
        mask = _mm_cmpeq_epi8(mask, zero);
    }
    if 0xffff == _mm_movemask_epi8(_mm_cmpeq_epi8(mask, zero)) {
        return;
    }
    {
        let mut work: __m128i;
        work = _mm_max_epu8(abs_diff(p2, p0), abs_diff(q2, q0));
        flat = _mm_max_epu8(work, max_abs_p1p0q1q0);
        flat = _mm_subs_epu8(flat, one);
        flat = _mm_cmpeq_epi8(flat, zero);
        flat = _mm_and_si128(flat, mask);
    }
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    // filter4
    {
        let t4: __m128i = _mm_set1_epi8(4);
        let t3: __m128i = _mm_set1_epi8(3);
        let t80: __m128i = _mm_set1_epi8(0x80_i32 as i8);
        let te0: __m128i = _mm_set1_epi8(0xe0_i32 as i8);
        let t1f: __m128i = _mm_set1_epi8(0x1f);
        let t1: __m128i = _mm_set1_epi8(0x1);
        let t7f: __m128i = _mm_set1_epi8(0x7f);
        let ff: __m128i = _mm_cmpeq_epi8(t4, t4);
        let mut filt: __m128i;
        let mut work_a: __m128i;
        let mut filter1: __m128i;
        let mut filter2: __m128i;
        op1 = _mm_xor_si128(p1, t80);
        op0 = _mm_xor_si128(p0, t80);
        oq0 = _mm_xor_si128(q0, t80);
        oq1 = _mm_xor_si128(q1, t80);
        hev = _mm_subs_epu8(max_abs_p1p0q1q0, thresh_v);
        hev = _mm_xor_si128(_mm_cmpeq_epi8(hev, zero), ff);
        filt = _mm_and_si128(_mm_subs_epi8(op1, oq1), hev);
        work_a = _mm_subs_epi8(oq0, op0);
        filt = _mm_adds_epi8(filt, work_a);
        filt = _mm_adds_epi8(filt, work_a);
        filt = _mm_adds_epi8(filt, work_a);
        filt = _mm_and_si128(filt, mask);
        filter1 = _mm_adds_epi8(filt, t4);
        filter2 = _mm_adds_epi8(filt, t3);
        work_a = _mm_cmpgt_epi8(zero, filter1);
        filter1 = _mm_srli_epi16::<{ 3 }>(filter1);
        work_a = _mm_and_si128(work_a, te0);
        filter1 = _mm_and_si128(filter1, t1f);
        filter1 = _mm_or_si128(filter1, work_a);
        oq0 = _mm_xor_si128(_mm_subs_epi8(oq0, filter1), t80);
        work_a = _mm_cmpgt_epi8(zero, filter2);
        filter2 = _mm_srli_epi16::<{ 3 }>(filter2);
        work_a = _mm_and_si128(work_a, te0);
        filter2 = _mm_and_si128(filter2, t1f);
        filter2 = _mm_or_si128(filter2, work_a);
        op0 = _mm_xor_si128(_mm_adds_epi8(op0, filter2), t80);
        filt = _mm_adds_epi8(filter1, t1);
        work_a = _mm_cmpgt_epi8(zero, filt);
        filt = _mm_srli_epi16::<{ 1 }>(filt);
        work_a = _mm_and_si128(work_a, t80);
        filt = _mm_and_si128(filt, t7f);
        filt = _mm_or_si128(filt, work_a);
        filt = _mm_andnot_si128(hev, filt);
        op1 = _mm_xor_si128(_mm_adds_epi8(op1, filt), t80);
        oq1 = _mm_xor_si128(_mm_subs_epi8(oq1, filt), t80);
        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        // filter6
        if 0xffff != _mm_movemask_epi8(_mm_cmpeq_epi8(flat, zero)) {
            let four: __m128i = _mm_set1_epi16(4);
            let p2_lo: __m128i = _mm_unpacklo_epi8(p2, zero);
            let p1_lo: __m128i = _mm_unpacklo_epi8(p1, zero);
            let p0_lo: __m128i = _mm_unpacklo_epi8(p0, zero);
            let q0_lo: __m128i = _mm_unpacklo_epi8(q0, zero);
            let q1_lo: __m128i = _mm_unpacklo_epi8(q1, zero);
            let q2_lo: __m128i = _mm_unpacklo_epi8(q2, zero);
            let p2_hi: __m128i = _mm_unpackhi_epi8(p2, zero);
            let p1_hi: __m128i = _mm_unpackhi_epi8(p1, zero);
            let p0_hi: __m128i = _mm_unpackhi_epi8(p0, zero);
            let q0_hi: __m128i = _mm_unpackhi_epi8(q0, zero);
            let q1_hi: __m128i = _mm_unpackhi_epi8(q1, zero);
            let q2_hi: __m128i = _mm_unpackhi_epi8(q2, zero);
            let mut f8_lo: __m128i = core::mem::zeroed();
            let mut f8_hi: __m128i = core::mem::zeroed();
            f8_lo = _mm_add_epi16(_mm_add_epi16(p2_lo, four), _mm_add_epi16(p2_lo, p2_lo));
            f8_lo = _mm_add_epi16(_mm_add_epi16(p1_lo, f8_lo), _mm_add_epi16(p1_lo, p0_lo));
            f8_lo = _mm_add_epi16(_mm_add_epi16(p0_lo, q0_lo), f8_lo);
            f8_hi = _mm_add_epi16(_mm_add_epi16(p2_hi, four), _mm_add_epi16(p2_hi, p2_hi));
            f8_hi = _mm_add_epi16(_mm_add_epi16(p1_hi, f8_hi), _mm_add_epi16(p1_hi, p0_hi));
            f8_hi = _mm_add_epi16(_mm_add_epi16(p0_hi, q0_hi), f8_hi);
            op1 = filter8_mask(
                &raw mut flat as *const __m128i,
                &raw mut op1 as *const __m128i,
                &raw mut f8_lo as *const __m128i,
                &raw mut f8_hi as *const __m128i,
            );
            _mm_storeu_si128(s.offset(-((2 * p) as isize)) as *mut __m128i, op1);
            f8_lo = filter_add2_sub2(
                &raw mut f8_lo as *const __m128i,
                &raw const q0_lo,
                &raw const q1_lo,
                &raw const p2_lo,
                &raw const p2_lo,
            );
            f8_hi = filter_add2_sub2(
                &raw mut f8_hi as *const __m128i,
                &raw const q0_hi,
                &raw const q1_hi,
                &raw const p2_hi,
                &raw const p2_hi,
            );
            op0 = filter8_mask(
                &raw mut flat as *const __m128i,
                &raw mut op0 as *const __m128i,
                &raw mut f8_lo as *const __m128i,
                &raw mut f8_hi as *const __m128i,
            );
            _mm_storeu_si128(s.offset(-((1 * p) as isize)) as *mut __m128i, op0);
            f8_lo = filter_add2_sub2(
                &raw mut f8_lo as *const __m128i,
                &raw const q1_lo,
                &raw const q2_lo,
                &raw const p1_lo,
                &raw const p2_lo,
            );
            f8_hi = filter_add2_sub2(
                &raw mut f8_hi as *const __m128i,
                &raw const q1_hi,
                &raw const q2_hi,
                &raw const p1_hi,
                &raw const p2_hi,
            );
            oq0 = filter8_mask(
                &raw mut flat as *const __m128i,
                &raw mut oq0 as *const __m128i,
                &raw mut f8_lo as *const __m128i,
                &raw mut f8_hi as *const __m128i,
            );
            _mm_storeu_si128(s.offset(-((0 * p) as isize)) as *mut __m128i, oq0);
            f8_lo = filter_add2_sub2(
                &raw mut f8_lo as *const __m128i,
                &raw const q2_lo,
                &raw const q2_lo,
                &raw const p0_lo,
                &raw const p1_lo,
            );
            f8_hi = filter_add2_sub2(
                &raw mut f8_hi as *const __m128i,
                &raw const q2_hi,
                &raw const q2_hi,
                &raw const p0_hi,
                &raw const p1_hi,
            );
            oq1 = filter8_mask(
                &raw mut flat as *const __m128i,
                &raw mut oq1 as *const __m128i,
                &raw mut f8_lo as *const __m128i,
                &raw mut f8_hi as *const __m128i,
            );
            _mm_storeu_si128(s.offset((1 * p) as isize) as *mut __m128i, oq1);
        } else {
            _mm_storeu_si128(s.offset(-((2 * p) as isize)) as *mut __m128i, op1);
            _mm_storeu_si128(s.offset(-((1 * p) as isize)) as *mut __m128i, op0);
            _mm_storeu_si128(s.offset(-((0 * p) as isize)) as *mut __m128i, oq0);
            _mm_storeu_si128(s.offset((1 * p) as isize) as *mut __m128i, oq1);
        }
    }
}

#[no_mangle]
#[target_feature(enable = "sse2")]
pub unsafe extern "C-unwind" fn aom_lpf_horizontal_4_quad_sse2(
    s: *mut u8,
    p: i32,
    _blimit0: *const u8,
    _limit0: *const u8,
    _thresh0: *const u8,
) {
    let zero: __m128i = _mm_setzero_si128();
    let blimit_v: __m128i = _mm_load_si128(_blimit0 as *const __m128i);
    let limit_v: __m128i = _mm_load_si128(_limit0 as *const __m128i);
    let thresh_v: __m128i = _mm_load_si128(_thresh0 as *const __m128i);
    let mut mask: __m128i;
    let mut hev: __m128i;
    let mut p1: __m128i;
    let mut p0: __m128i;
    let mut q0: __m128i;
    let mut q1: __m128i;
    let mut op1: __m128i;
    let mut op0: __m128i;
    let mut oq0: __m128i;
    let mut oq1: __m128i;
    let mut max_abs_p1p0q1q0: __m128i;
    p1 = _mm_loadu_si128(s.offset(-((2 * p) as isize)) as *mut __m128i as *const __m128i);
    p0 = _mm_loadu_si128(s.offset(-((1 * p) as isize)) as *mut __m128i as *const __m128i);
    q0 = _mm_loadu_si128(s.offset(-((0 * p) as isize)) as *mut __m128i as *const __m128i);
    q1 = _mm_loadu_si128(s.offset((1 * p) as isize) as *mut __m128i as *const __m128i);
    {
        let abs_p1p0: __m128i = abs_diff(p1, p0);
        let abs_q1q0: __m128i = abs_diff(q1, q0);
        let fe: __m128i = _mm_set1_epi8(0xfe_i32 as i8);
        let ff: __m128i = _mm_cmpeq_epi8(zero, zero);
        let mut abs_p0q0: __m128i = abs_diff(p0, q0);
        let mut abs_p1q1: __m128i = abs_diff(p1, q1);
        max_abs_p1p0q1q0 = _mm_max_epu8(abs_p1p0, abs_q1q0);
        abs_p0q0 = _mm_adds_epu8(abs_p0q0, abs_p0q0);
        abs_p1q1 = _mm_srli_epi16::<{ 1 }>(_mm_and_si128(abs_p1q1, fe));
        mask = _mm_subs_epu8(_mm_adds_epu8(abs_p0q0, abs_p1q1), blimit_v);
        mask = _mm_xor_si128(_mm_cmpeq_epi8(mask, zero), ff);
        // mask |= (abs(p0 - q0) * 2 + abs(p1 - q1) / 2  > blimit) * -1;
        mask = _mm_max_epu8(max_abs_p1p0q1q0, mask);
        // mask |= (abs(p1 - p0) > limit) * -1;
        // mask |= (abs(q1 - q0) > limit) * -1;
        mask = _mm_subs_epu8(mask, limit_v);
        mask = _mm_cmpeq_epi8(mask, zero);
    }
    if 0xffff == _mm_movemask_epi8(_mm_cmpeq_epi8(mask, zero)) {
        return;
    }
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    // filter4
    {
        let t4: __m128i = _mm_set1_epi8(4);
        let t3: __m128i = _mm_set1_epi8(3);
        let t80: __m128i = _mm_set1_epi8(0x80_i32 as i8);
        let te0: __m128i = _mm_set1_epi8(0xe0_i32 as i8);
        let t1f: __m128i = _mm_set1_epi8(0x1f);
        let t1: __m128i = _mm_set1_epi8(0x1);
        let t7f: __m128i = _mm_set1_epi8(0x7f);
        let ff: __m128i = _mm_cmpeq_epi8(t4, t4);
        let mut filt: __m128i;
        let mut work_a: __m128i;
        let mut filter1: __m128i;
        let mut filter2: __m128i;
        op1 = _mm_xor_si128(p1, t80);
        op0 = _mm_xor_si128(p0, t80);
        oq0 = _mm_xor_si128(q0, t80);
        oq1 = _mm_xor_si128(q1, t80);
        hev = _mm_subs_epu8(max_abs_p1p0q1q0, thresh_v);
        hev = _mm_xor_si128(_mm_cmpeq_epi8(hev, zero), ff);
        filt = _mm_and_si128(_mm_subs_epi8(op1, oq1), hev);
        work_a = _mm_subs_epi8(oq0, op0);
        filt = _mm_adds_epi8(filt, work_a);
        filt = _mm_adds_epi8(filt, work_a);
        filt = _mm_adds_epi8(filt, work_a);
        filt = _mm_and_si128(filt, mask);
        filter1 = _mm_adds_epi8(filt, t4);
        filter2 = _mm_adds_epi8(filt, t3);
        work_a = _mm_cmpgt_epi8(zero, filter1);
        filter1 = _mm_srli_epi16::<{ 3 }>(filter1);
        work_a = _mm_and_si128(work_a, te0);
        filter1 = _mm_and_si128(filter1, t1f);
        filter1 = _mm_or_si128(filter1, work_a);
        oq0 = _mm_xor_si128(_mm_subs_epi8(oq0, filter1), t80);
        work_a = _mm_cmpgt_epi8(zero, filter2);
        filter2 = _mm_srli_epi16::<{ 3 }>(filter2);
        work_a = _mm_and_si128(work_a, te0);
        filter2 = _mm_and_si128(filter2, t1f);
        filter2 = _mm_or_si128(filter2, work_a);
        op0 = _mm_xor_si128(_mm_adds_epi8(op0, filter2), t80);
        filt = _mm_adds_epi8(filter1, t1);
        work_a = _mm_cmpgt_epi8(zero, filt);
        filt = _mm_srli_epi16::<{ 1 }>(filt);
        work_a = _mm_and_si128(work_a, t80);
        filt = _mm_and_si128(filt, t7f);
        filt = _mm_or_si128(filt, work_a);
        filt = _mm_andnot_si128(hev, filt);
        op1 = _mm_xor_si128(_mm_adds_epi8(op1, filt), t80);
        oq1 = _mm_xor_si128(_mm_subs_epi8(oq1, filt), t80);
        _mm_storeu_si128(s.offset(-((2 * p) as isize)) as *mut __m128i, op1);
        _mm_storeu_si128(s.offset(-((1 * p) as isize)) as *mut __m128i, op0);
        _mm_storeu_si128(s.offset(-((0 * p) as isize)) as *mut __m128i, oq0);
        _mm_storeu_si128(s.offset((1 * p) as isize) as *mut __m128i, oq1);
    }
}

#[no_mangle]
#[target_feature(enable = "sse2")]
pub unsafe extern "C-unwind" fn aom_lpf_vertical_14_quad_sse2(
    s: *mut u8,
    pitch: i32,
    _blimit0: *const u8,
    _limit0: *const u8,
    _thresh0: *const u8,
) {
    let mut t_dst: Aligned16<[u8; 256]> = core::mem::zeroed();
    // Transpose 16x16
    transpose_16x8(
        s.offset(-8),
        s.offset(-8).offset((8 * pitch) as isize),
        pitch,
        t_dst.0.as_mut_ptr(),
        16,
    );
    transpose_16x8(
        s,
        s.offset((8 * pitch) as isize),
        pitch,
        t_dst.0.as_mut_ptr().offset((8 * 16) as isize),
        16,
    );
    // Loop filtering
    aom_lpf_horizontal_14_quad.unwrap()(
        t_dst.0.as_mut_ptr().offset((8 * 16) as isize),
        16,
        _blimit0,
        _limit0,
        _thresh0,
    );
    // Transpose back
    transpose_16x8(
        t_dst.0.as_mut_ptr(),
        t_dst.0.as_mut_ptr().offset((8 * 16) as isize),
        16,
        s.offset(-8),
        pitch,
    );
    transpose_16x8(
        t_dst.0.as_mut_ptr().offset(8),
        t_dst.0.as_mut_ptr().offset(8).offset((8 * 16) as isize),
        16,
        s.offset(-8).offset((8 * pitch) as isize),
        pitch,
    );
}

#[no_mangle]
#[target_feature(enable = "sse2")]
pub unsafe extern "C-unwind" fn aom_lpf_vertical_8_quad_sse2(
    s: *mut u8,
    pitch: i32,
    _blimit0: *const u8,
    _limit0: *const u8,
    _thresh0: *const u8,
) {
    let mut t_dst: Aligned16<[u8; (16 * 8) as usize]> = core::mem::zeroed();
    // Transpose 16x8
    transpose_16x8(
        s.offset(-4),
        s.offset(-4).offset((pitch * 8) as isize),
        pitch,
        t_dst.0.as_mut_ptr(),
        16,
    );
    // Loop filtering
    aom_lpf_horizontal_8_quad.unwrap()(
        t_dst.0.as_mut_ptr().offset((4 * 16) as isize),
        16,
        _blimit0,
        _limit0,
        _thresh0,
    );
    // Transpose back
    transpose_16x8_to_8x16(t_dst.0.as_mut_ptr(), 16, s.offset(-4), pitch);
}

#[no_mangle]
#[target_feature(enable = "sse2")]
pub unsafe extern "C-unwind" fn aom_lpf_vertical_6_quad_sse2(
    s: *mut u8,
    pitch: i32,
    _blimit0: *const u8,
    _limit0: *const u8,
    _thresh0: *const u8,
) {
    let mut t_dst: Aligned16<[u8; (16 * 8) as usize]> = core::mem::zeroed();
    // Transpose 16x8:: (wxh) 8x16 to 16x8
    transpose_16x8(
        s.offset(-4),
        s.offset(-4).offset((pitch * 8) as isize),
        pitch,
        t_dst.0.as_mut_ptr(),
        16,
    );
    // Loop filtering
    aom_lpf_horizontal_6_quad.unwrap()(
        t_dst.0.as_mut_ptr().offset((4 * 16) as isize),
        16,
        _blimit0,
        _limit0,
        _thresh0,
    );
    // Transpose back:: (wxh) 16x8 to 8x16
    transpose_16x8_to_8x16(t_dst.0.as_mut_ptr(), 16, s.offset(-4), pitch);
}

#[no_mangle]
#[target_feature(enable = "sse2")]
pub unsafe extern "C-unwind" fn aom_lpf_vertical_4_quad_sse2(
    s: *mut u8,
    pitch: i32,
    _blimit0: *const u8,
    _limit0: *const u8,
    _thresh0: *const u8,
) {
    let mut t_dst: Aligned16<[u8; (16 * 8) as usize]> = core::mem::zeroed();
    // Transpose 16x8
    transpose_16x8(
        s.offset(-4),
        s.offset(-4).offset((pitch * 8) as isize),
        pitch,
        t_dst.0.as_mut_ptr(),
        16,
    );
    // Loop filtering
    aom_lpf_horizontal_4_quad_sse2(
        t_dst.0.as_mut_ptr().offset((4 * 16) as isize),
        16,
        _blimit0,
        _limit0,
        _thresh0,
    );
    // Transpose back
    transpose_16x8_to_8x16(t_dst.0.as_mut_ptr(), 16, s.offset(-4), pitch);
}
