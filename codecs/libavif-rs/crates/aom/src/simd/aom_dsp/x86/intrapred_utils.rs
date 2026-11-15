/*
 * Copyright (c) 2021, Alliance for Open Media. All rights reserved.
 *
 * This source code is subject to the terms of the BSD 2 Clause License and
 * the Alliance for Open Media Patent License 1.0. If the BSD 2 Clause License
 * was not distributed with this source code in the LICENSE file, you can
 * obtain it at www.aomedia.org/license/software. If the Alliance for Open
 * Media Patent License 1.0 was not distributed with this source code in the
 * PATENTS file, you can obtain it at www.aomedia.org/license/patent.
 */

// Translated from upstream/aom/aom_dsp/x86/intrapred_utils.h (ADR 0056, ADR 0058).
// The tables and inline functions were
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

use core::arch::x86_64::*;

#[repr(C, align(8))]
pub struct Aligned8<T>(pub T);
#[repr(C, align(16))]
pub struct Aligned16<T>(pub T);
#[repr(C, align(32))]
pub struct Aligned32<T>(pub T);

// SSE2
pub static EvenOddMaskx: Aligned16<[[u8; 16]; 8]> = Aligned16([
    [0, 2, 4, 6, 8, 10, 12, 14, 1, 3, 5, 7, 9, 11, 13, 15],
    [0, 1, 3, 5, 7, 9, 11, 13, 0, 2, 4, 6, 8, 10, 12, 14],
    [0, 0, 2, 4, 6, 8, 10, 12, 0, 0, 3, 5, 7, 9, 11, 13],
    [0, 0, 0, 3, 5, 7, 9, 11, 0, 0, 0, 4, 6, 8, 10, 12],
    [0, 0, 0, 0, 4, 6, 8, 10, 0, 0, 0, 0, 5, 7, 9, 11],
    [0, 0, 0, 0, 0, 5, 7, 9, 0, 0, 0, 0, 0, 6, 8, 10],
    [0, 0, 0, 0, 0, 0, 6, 8, 0, 0, 0, 0, 0, 0, 7, 9],
    [0, 0, 0, 0, 0, 0, 0, 7, 0, 0, 0, 0, 0, 0, 0, 8],
]);

pub static LoadMaskx: Aligned16<[[u8; 16]; 16]> = Aligned16([
    [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15],
    [0, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14],
    [0, 0, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13],
    [0, 0, 0, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
    [0, 0, 0, 0, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11],
    [0, 0, 0, 0, 0, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
    [0, 0, 0, 0, 0, 0, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9],
    [0, 0, 0, 0, 0, 0, 0, 0, 1, 2, 3, 4, 5, 6, 7, 8],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 2, 3, 4, 5, 6, 7],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 2, 3, 4, 5, 6],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 2, 3, 4, 5],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 2, 3, 4],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 2, 3],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 2],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
]);

pub static LoadMaskz2: Aligned32<[[i32; 8]; 8]> = Aligned32([
    [-1, 0, 0, 0, 0, 0, 0, 0],
    [-1, -1, 0, 0, 0, 0, 0, 0],
    [-1, -1, -1, 0, 0, 0, 0, 0],
    [-1, -1, -1, -1, 0, 0, 0, 0],
    [-1, -1, -1, -1, -1, 0, 0, 0],
    [-1, -1, -1, -1, -1, -1, 0, 0],
    [-1, -1, -1, -1, -1, -1, -1, 0],
    [-1, -1, -1, -1, -1, -1, -1, -1],
]);

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn transpose4x16_sse2(x: *mut __m128i, d: *mut __m128i) {
    let mut w0: __m128i;
    let mut w1: __m128i;
    let mut w2: __m128i;
    let mut w3: __m128i;
    let mut ww0: __m128i;
    let mut ww1: __m128i;
    let mut ww2: __m128i;
    let mut ww3: __m128i;
    w0 = _mm_unpacklo_epi8(*x, *x.offset(1));
    w1 = _mm_unpacklo_epi8(*x.offset(2), *x.offset(3));
    w2 = _mm_unpackhi_epi8(*x, *x.offset(1));
    w3 = _mm_unpackhi_epi8(*x.offset(2), *x.offset(3));
    ww0 = _mm_unpacklo_epi16(w0, w1);
    ww1 = _mm_unpacklo_epi16(w2, w3);
    ww2 = _mm_unpackhi_epi16(w0, w1);
    ww3 = _mm_unpackhi_epi16(w2, w3);
    w0 = _mm_unpacklo_epi32(ww0, ww1);
    w2 = _mm_unpacklo_epi32(ww2, ww3);
    w1 = _mm_unpackhi_epi32(ww0, ww1);
    w3 = _mm_unpackhi_epi32(ww2, ww3);
    *d = _mm_unpacklo_epi64(w0, w2);
    *d.offset(1) = _mm_unpackhi_epi64(w0, w2);
    *d.offset(2) = _mm_unpacklo_epi64(w1, w3);
    *d.offset(3) = _mm_unpackhi_epi64(w1, w3);
    *d.offset(4) = _mm_srli_si128::<8>(*d);
    *d.offset(5) = _mm_srli_si128::<8>(*d.offset(1));
    *d.offset(6) = _mm_srli_si128::<8>(*d.offset(2));
    *d.offset(7) = _mm_srli_si128::<8>(*d.offset(3));
    *d.offset(8) = _mm_srli_si128::<4>(*d);
    *d.offset(9) = _mm_srli_si128::<4>(*d.offset(1));
    *d.offset(10) = _mm_srli_si128::<4>(*d.offset(2));
    *d.offset(11) = _mm_srli_si128::<4>(*d.offset(3));
    *d.offset(12) = _mm_srli_si128::<12>(*d);
    *d.offset(13) = _mm_srli_si128::<12>(*d.offset(1));
    *d.offset(14) = _mm_srli_si128::<12>(*d.offset(2));
    *d.offset(15) = _mm_srli_si128::<12>(*d.offset(3));
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn transpose16x16_sse2(x: *mut __m128i, d: *mut __m128i) {
    let mut w0: __m128i;
    let mut w1: __m128i;
    let mut w2: __m128i;
    let mut w3: __m128i;
    let mut w4: __m128i;
    let mut w5: __m128i;
    let mut w6: __m128i;
    let mut w7: __m128i;
    let mut w8: __m128i;
    let mut w9: __m128i;
    let mut w10: __m128i;
    let mut w11: __m128i;
    let mut w12: __m128i;
    let mut w13: __m128i;
    let mut w14: __m128i;
    let mut w15: __m128i;
    w0 = _mm_unpacklo_epi8(*x, *x.offset(1));
    w1 = _mm_unpacklo_epi8(*x.offset(2), *x.offset(3));
    w2 = _mm_unpacklo_epi8(*x.offset(4), *x.offset(5));
    w3 = _mm_unpacklo_epi8(*x.offset(6), *x.offset(7));
    w8 = _mm_unpacklo_epi8(*x.offset(8), *x.offset(9));
    w9 = _mm_unpacklo_epi8(*x.offset(10), *x.offset(11));
    w10 = _mm_unpacklo_epi8(*x.offset(12), *x.offset(13));
    w11 = _mm_unpacklo_epi8(*x.offset(14), *x.offset(15));
    w4 = _mm_unpacklo_epi16(w0, w1);
    w5 = _mm_unpacklo_epi16(w2, w3);
    w12 = _mm_unpacklo_epi16(w8, w9);
    w13 = _mm_unpacklo_epi16(w10, w11);
    w6 = _mm_unpacklo_epi32(w4, w5);
    w7 = _mm_unpackhi_epi32(w4, w5);
    w14 = _mm_unpacklo_epi32(w12, w13);
    w15 = _mm_unpackhi_epi32(w12, w13);
    // Store first 4-line result
    *d = _mm_unpacklo_epi64(w6, w14);
    *d.offset(1) = _mm_unpackhi_epi64(w6, w14);
    *d.offset(2) = _mm_unpacklo_epi64(w7, w15);
    *d.offset(3) = _mm_unpackhi_epi64(w7, w15);
    w4 = _mm_unpackhi_epi16(w0, w1);
    w5 = _mm_unpackhi_epi16(w2, w3);
    w12 = _mm_unpackhi_epi16(w8, w9);
    w13 = _mm_unpackhi_epi16(w10, w11);
    w6 = _mm_unpacklo_epi32(w4, w5);
    w7 = _mm_unpackhi_epi32(w4, w5);
    w14 = _mm_unpacklo_epi32(w12, w13);
    w15 = _mm_unpackhi_epi32(w12, w13);
    // Store second 4-line result
    *d.offset(4) = _mm_unpacklo_epi64(w6, w14);
    *d.offset(5) = _mm_unpackhi_epi64(w6, w14);
    *d.offset(6) = _mm_unpacklo_epi64(w7, w15);
    *d.offset(7) = _mm_unpackhi_epi64(w7, w15);
    // upper half
    w0 = _mm_unpackhi_epi8(*x, *x.offset(1));
    w1 = _mm_unpackhi_epi8(*x.offset(2), *x.offset(3));
    w2 = _mm_unpackhi_epi8(*x.offset(4), *x.offset(5));
    w3 = _mm_unpackhi_epi8(*x.offset(6), *x.offset(7));
    w8 = _mm_unpackhi_epi8(*x.offset(8), *x.offset(9));
    w9 = _mm_unpackhi_epi8(*x.offset(10), *x.offset(11));
    w10 = _mm_unpackhi_epi8(*x.offset(12), *x.offset(13));
    w11 = _mm_unpackhi_epi8(*x.offset(14), *x.offset(15));
    w4 = _mm_unpacklo_epi16(w0, w1);
    w5 = _mm_unpacklo_epi16(w2, w3);
    w12 = _mm_unpacklo_epi16(w8, w9);
    w13 = _mm_unpacklo_epi16(w10, w11);
    w6 = _mm_unpacklo_epi32(w4, w5);
    w7 = _mm_unpackhi_epi32(w4, w5);
    w14 = _mm_unpacklo_epi32(w12, w13);
    w15 = _mm_unpackhi_epi32(w12, w13);
    // Store first 4-line result
    *d.offset(8) = _mm_unpacklo_epi64(w6, w14);
    *d.offset(9) = _mm_unpackhi_epi64(w6, w14);
    *d.offset(10) = _mm_unpacklo_epi64(w7, w15);
    *d.offset(11) = _mm_unpackhi_epi64(w7, w15);
    w4 = _mm_unpackhi_epi16(w0, w1);
    w5 = _mm_unpackhi_epi16(w2, w3);
    w12 = _mm_unpackhi_epi16(w8, w9);
    w13 = _mm_unpackhi_epi16(w10, w11);
    w6 = _mm_unpacklo_epi32(w4, w5);
    w7 = _mm_unpackhi_epi32(w4, w5);
    w14 = _mm_unpacklo_epi32(w12, w13);
    w15 = _mm_unpackhi_epi32(w12, w13);
    // Store second 4-line result
    *d.offset(12) = _mm_unpacklo_epi64(w6, w14);
    *d.offset(13) = _mm_unpackhi_epi64(w6, w14);
    *d.offset(14) = _mm_unpacklo_epi64(w7, w15);
    *d.offset(15) = _mm_unpackhi_epi64(w7, w15);
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn transpose_TX_16X16(src: *const u8, pitchSrc: isize, dst: *mut u8, pitchDst: isize) {
    let mut r: [__m128i; 16] = core::mem::zeroed();
    let mut d: [__m128i; 16] = core::mem::zeroed();
    for j in 0..16 {
        r[j as usize] =
            _mm_loadu_si128(src.offset(j as isize * pitchSrc) as *mut __m128i as *const __m128i);
    }
    transpose16x16_sse2(r.as_mut_ptr(), d.as_mut_ptr());
    for j in 0..16 {
        _mm_storeu_si128(
            dst.offset(j as isize * pitchDst) as *mut __m128i,
            d[j as usize],
        );
    }
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn transpose(
    src: *const u8,
    pitchSrc: isize,
    dst: *mut u8,
    pitchDst: isize,
    width: i32,
    height: i32,
) {
    for j in (0..height).step_by(16) {
        for i in (0..width).step_by(16) {
            transpose_TX_16X16(
                src.offset(i as isize * pitchSrc).offset(j as isize),
                pitchSrc,
                dst.offset(j as isize * pitchDst).offset(i as isize),
                pitchDst,
            );
        }
    }
}

// AOM_AOM_DSP_X86_INTRAPRED_UTILS_H_
