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

// Translated from upstream/aom/aom_dsp/x86/intrapred_sse4.c (ADR 0056, ADR 0058).
// Upstream compiles it with -msse41. The functions were
// transliterated with a typing-aware C-subset printer (not C2Rust) and
// reviewed against the C function by function: the same sequence of intrinsic
// and helper calls, constants and operators in order (ADR 0058).
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

use crate::simd::aom_dsp__x86__intrapred_utils::*;
use crate::simd::aom_dsp__x86__intrapred_x86::*;
use crate::simd::aom_dsp__x86__lpf_common_sse2::*;
use crate::simd::aom_dsp__x86__synonyms::*;
use core::arch::x86_64::*;

const SMOOTH_WEIGHT_LOG2_SCALE: i32 = 8;

// SSE2
// SSE4.1
// Low bit depth functions
static Mask: Aligned16<[[[u8; 16]; 33]; 2]> = Aligned16([
    [
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0xff, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0xff, 0xff, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0xff, 0xff, 0xff, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0xff, 0xff, 0xff, 0xff, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [
            0xff, 0xff, 0xff, 0xff, 0xff, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ],
        [
            0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ],
        [
            0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ],
        [
            0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0, 0, 0, 0, 0, 0, 0, 0,
        ],
        [
            0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0, 0, 0, 0, 0, 0, 0,
        ],
        [
            0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0, 0, 0, 0, 0, 0,
        ],
        [
            0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0, 0, 0, 0, 0,
        ],
        [
            0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0, 0, 0, 0,
        ],
        [
            0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0, 0, 0,
        ],
        [
            0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0,
            0,
        ],
        [
            0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
            0xff, 0,
        ],
        [
            0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
            0xff, 0xff,
        ],
        [
            0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
            0xff, 0xff,
        ],
        [
            0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
            0xff, 0xff,
        ],
        [
            0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
            0xff, 0xff,
        ],
        [
            0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
            0xff, 0xff,
        ],
        [
            0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
            0xff, 0xff,
        ],
        [
            0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
            0xff, 0xff,
        ],
        [
            0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
            0xff, 0xff,
        ],
        [
            0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
            0xff, 0xff,
        ],
        [
            0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
            0xff, 0xff,
        ],
        [
            0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
            0xff, 0xff,
        ],
        [
            0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
            0xff, 0xff,
        ],
        [
            0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
            0xff, 0xff,
        ],
        [
            0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
            0xff, 0xff,
        ],
        [
            0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
            0xff, 0xff,
        ],
        [
            0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
            0xff, 0xff,
        ],
        [
            0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
            0xff, 0xff,
        ],
    ],
    [
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0xff, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0xff, 0xff, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0xff, 0xff, 0xff, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0xff, 0xff, 0xff, 0xff, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [
            0xff, 0xff, 0xff, 0xff, 0xff, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ],
        [
            0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ],
        [
            0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ],
        [
            0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0, 0, 0, 0, 0, 0, 0, 0,
        ],
        [
            0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0, 0, 0, 0, 0, 0, 0,
        ],
        [
            0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0, 0, 0, 0, 0, 0,
        ],
        [
            0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0, 0, 0, 0, 0,
        ],
        [
            0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0, 0, 0, 0,
        ],
        [
            0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0, 0, 0,
        ],
        [
            0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0,
            0,
        ],
        [
            0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
            0xff, 0,
        ],
        [
            0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
            0xff, 0xff,
        ],
    ],
]);

// clang-format on
#[inline]
#[target_feature(enable = "sse4.1")]
unsafe fn dr_prediction_z1_HxW_internal_sse4_1(
    H: i32,
    W: i32,
    dst: *mut __m128i,
    above: *const u8,
    upsample_above: i32,
    dx: i32,
) {
    let frac_bits: i32 = 6 - upsample_above;
    let max_base_x: i32 = (W + H - 1) << upsample_above;
    debug_assert!(dx > 0);
    // pre-filter above pixels
    // store in temp buffers:
    //   above[x] * 32 + 16
    //   above[x+1] - above[x]
    // final pixels will be calculated as:
    //   (above[x] * 32 + 16 + (above[x+1] - above[x]) * shift) >> 5
    let mut a0: __m128i;
    let mut a1: __m128i;
    let mut a32: __m128i;
    let mut a16: __m128i;
    let mut diff: __m128i;
    let mut c3f: __m128i;
    let mut a_mbase_x: __m128i;
    a16 = _mm_set1_epi16(16);
    a_mbase_x = _mm_set1_epi8(*above.offset(max_base_x as isize) as i8);
    c3f = _mm_set1_epi16(0x3f);
    let mut x: i32 = dx;
    for r in 0..W {
        let mut b: __m128i;
        let mut res: __m128i;
        let mut res1: __m128i;
        let mut shift: __m128i;
        let mut a0_above: __m128i;
        let mut a1_above: __m128i;
        let mut base: i32 = x >> frac_bits;
        let mut base_max_diff: i32 = (max_base_x - base) >> upsample_above;
        if base_max_diff <= 0 {
            for i in r..W {
                *dst.offset(i as isize) = a_mbase_x;
            } // save 4 values
            return;
        }
        if base_max_diff > H {
            base_max_diff = H;
        }
        a0_above = _mm_loadu_si128(above.offset(base as isize) as *mut __m128i as *const __m128i);
        a1_above = _mm_loadu_si128(
            above.offset(base as isize).offset(1) as *mut __m128i as *const __m128i
        );
        if upsample_above != 0 {
            a0_above = _mm_shuffle_epi8(a0_above, *(EvenOddMaskx.0[0].as_ptr() as *mut __m128i));
            a1_above = _mm_srli_si128::<8>(a0_above);
            shift = _mm_srli_epi16::<{ 1 }>(_mm_and_si128(
                _mm_sll_epi16(_mm_set1_epi16(x as i16), _mm_cvtsi32_si128(upsample_above)),
                c3f,
            ));
        } else {
            shift = _mm_srli_epi16::<{ 1 }>(_mm_and_si128(_mm_set1_epi16(x as i16), c3f));
        }
        // lower half
        a0 = _mm_cvtepu8_epi16(a0_above);
        a1 = _mm_cvtepu8_epi16(a1_above);
        diff = _mm_sub_epi16(a1, a0); // a[x+1] - a[x]
        a32 = _mm_slli_epi16::<{ 5 }>(a0); // a[x] * 32
        a32 = _mm_add_epi16(a32, a16); // a[x] * 32 + 16
        b = _mm_mullo_epi16(diff, shift);
        res = _mm_add_epi16(a32, b);
        res = _mm_srli_epi16::<{ 5 }>(res);
        // uppar half
        a0 = _mm_cvtepu8_epi16(_mm_srli_si128::<8>(a0_above));
        a1 = _mm_cvtepu8_epi16(_mm_srli_si128::<8>(a1_above));
        diff = _mm_sub_epi16(a1, a0); // a[x+1] - a[x]
        a32 = _mm_slli_epi16::<{ 5 }>(a0); // a[x] * 32
        a32 = _mm_add_epi16(a32, a16); // a[x] * 32 + 16
        b = _mm_mullo_epi16(diff, shift);
        res1 = _mm_add_epi16(a32, b);
        res1 = _mm_srli_epi16::<{ 5 }>(res1);
        res = _mm_packus_epi16(res, res1);
        *dst.offset(r as isize) = _mm_blendv_epi8(
            a_mbase_x,
            res,
            *(Mask.0[0][base_max_diff as usize].as_ptr() as *mut __m128i),
        );
        x += dx;
    }
}

#[inline]
#[target_feature(enable = "sse4.1")]
unsafe fn dr_prediction_z1_4xN_sse4_1(
    N: i32,
    dst: *mut u8,
    stride: isize,
    above: *const u8,
    upsample_above: i32,
    dx: i32,
) {
    let mut dstvec: [__m128i; 16] = core::mem::zeroed();
    dr_prediction_z1_HxW_internal_sse4_1(4, N, dstvec.as_mut_ptr(), above, upsample_above, dx);
    for i in 0..N {
        core::ptr::write_unaligned(
            dst.offset(stride * i as isize) as *mut i32,
            _mm_cvtsi128_si32(dstvec[i as usize]),
        );
    }
}

#[inline]
#[target_feature(enable = "sse4.1")]
unsafe fn dr_prediction_z1_8xN_sse4_1(
    N: i32,
    dst: *mut u8,
    stride: isize,
    above: *const u8,
    upsample_above: i32,
    dx: i32,
) {
    let mut dstvec: [__m128i; 32] = core::mem::zeroed();
    dr_prediction_z1_HxW_internal_sse4_1(8, N, dstvec.as_mut_ptr(), above, upsample_above, dx);
    for i in 0..N {
        _mm_storel_epi64(
            dst.offset(stride * i as isize) as *mut __m128i,
            dstvec[i as usize],
        );
    }
}

#[inline]
#[target_feature(enable = "sse4.1")]
unsafe fn dr_prediction_z1_16xN_sse4_1(
    N: i32,
    dst: *mut u8,
    stride: isize,
    above: *const u8,
    upsample_above: i32,
    dx: i32,
) {
    let mut dstvec: [__m128i; 64] = core::mem::zeroed();
    dr_prediction_z1_HxW_internal_sse4_1(16, N, dstvec.as_mut_ptr(), above, upsample_above, dx);
    for i in 0..N {
        _mm_storeu_si128(
            dst.offset(stride * i as isize) as *mut __m128i,
            dstvec[i as usize],
        );
    }
}

#[inline]
#[target_feature(enable = "sse4.1")]
unsafe fn dr_prediction_z1_32xN_internal_sse4_1(
    N: i32,
    dstvec: *mut __m128i,
    dstvec_h: *mut __m128i,
    above: *const u8,
    upsample_above: i32,
    dx: i32,
) {
    // here upsample_above is 0 by design of av1_use_intra_edge_upsample
    let _ = upsample_above;
    let frac_bits: i32 = 6;
    let max_base_x: i32 = 32 + N - 1;
    // pre-filter above pixels
    // store in temp buffers:
    //   above[x] * 32 + 16
    //   above[x+1] - above[x]
    // final pixels will be calculated as:
    //   (above[x] * 32 + 16 + (above[x+1] - above[x]) * shift) >> 5
    let mut a0: __m128i;
    let mut a1: __m128i;
    let mut a32: __m128i;
    let mut a16: __m128i;
    let mut a_mbase_x: __m128i;
    let mut diff: __m128i;
    let mut c3f: __m128i;
    a16 = _mm_set1_epi16(16);
    a_mbase_x = _mm_set1_epi8(*above.offset(max_base_x as isize) as i8);
    c3f = _mm_set1_epi16(0x3f);
    let mut x: i32 = dx;
    for r in 0..N {
        let mut b: __m128i;
        let mut res: __m128i;
        let mut res1: __m128i;
        let mut res16: [__m128i; 2] = core::mem::zeroed();
        let mut a0_above: __m128i;
        let mut a1_above: __m128i;
        let mut base: i32 = x >> frac_bits;
        let mut base_max_diff: i32 = max_base_x - base;
        if base_max_diff <= 0 {
            for i in r..N {
                *dstvec.offset(i as isize) = a_mbase_x; // save 32 values
                *dstvec_h.offset(i as isize) = a_mbase_x;
            }
            return;
        }
        if base_max_diff > 32 {
            base_max_diff = 32;
        }
        let mut shift: __m128i =
            _mm_srli_epi16::<{ 1 }>(_mm_and_si128(_mm_set1_epi16(x as i16), c3f));
        {
            let mut j: i32 = 0;
            let mut jj: i32 = 0;
            while j < 32 {
                let mut mdiff: i32 = base_max_diff - j;
                if mdiff <= 0 {
                    res16[jj as usize] = a_mbase_x;
                } else {
                    a0_above = _mm_loadu_si128(above.offset(base as isize).offset(j as isize)
                        as *mut __m128i
                        as *const __m128i);
                    a1_above =
                        _mm_loadu_si128(above.offset(base as isize).offset(j as isize).offset(1)
                            as *mut __m128i
                            as *const __m128i);
                    // lower half
                    a0 = _mm_cvtepu8_epi16(a0_above);
                    a1 = _mm_cvtepu8_epi16(a1_above);
                    diff = _mm_sub_epi16(a1, a0); // a[x+1] - a[x]
                    a32 = _mm_slli_epi16::<{ 5 }>(a0); // a[x] * 32
                    a32 = _mm_add_epi16(a32, a16); // a[x] * 32 + 16
                    b = _mm_mullo_epi16(diff, shift);
                    res = _mm_add_epi16(a32, b);
                    res = _mm_srli_epi16::<{ 5 }>(res);
                    // uppar half
                    a0 = _mm_cvtepu8_epi16(_mm_srli_si128::<8>(a0_above));
                    a1 = _mm_cvtepu8_epi16(_mm_srli_si128::<8>(a1_above));
                    diff = _mm_sub_epi16(a1, a0); // a[x+1] - a[x]
                    a32 = _mm_slli_epi16::<{ 5 }>(a0); // a[x] * 32
                    a32 = _mm_add_epi16(a32, a16); // a[x] * 32 + 16
                    b = _mm_mullo_epi16(diff, shift);
                    res1 = _mm_add_epi16(a32, b);
                    res1 = _mm_srli_epi16::<{ 5 }>(res1);
                    res16[jj as usize] = _mm_packus_epi16(res, res1);
                }
                j += 16;
                jj += 1;
            }
        } // 16 8bit values
        *dstvec.offset(r as isize) = _mm_blendv_epi8(
            a_mbase_x,
            res16[0],
            *(Mask.0[0][base_max_diff as usize].as_ptr() as *mut __m128i),
        ); // 16 8bit values
        *dstvec_h.offset(r as isize) = _mm_blendv_epi8(
            a_mbase_x,
            res16[1],
            *(Mask.0[1][base_max_diff as usize].as_ptr() as *mut __m128i),
        ); // 16 8bit values
        x += dx;
    }
}

#[inline]
#[target_feature(enable = "sse4.1")]
unsafe fn dr_prediction_z1_32xN_sse4_1(
    N: i32,
    dst: *mut u8,
    stride: isize,
    above: *const u8,
    upsample_above: i32,
    dx: i32,
) {
    let mut dstvec: [__m128i; 64] = core::mem::zeroed();
    let mut dstvec_h: [__m128i; 64] = core::mem::zeroed();
    dr_prediction_z1_32xN_internal_sse4_1(
        N,
        dstvec.as_mut_ptr(),
        dstvec_h.as_mut_ptr(),
        above,
        upsample_above,
        dx,
    );
    for i in 0..N {
        _mm_storeu_si128(
            dst.offset(stride * i as isize) as *mut __m128i,
            dstvec[i as usize],
        );
        _mm_storeu_si128(
            dst.offset(stride * i as isize).offset(16) as *mut __m128i,
            dstvec_h[i as usize],
        );
    }
}

#[inline]
#[target_feature(enable = "sse4.1")]
unsafe fn dr_prediction_z1_64xN_sse4_1(
    N: i32,
    mut dst: *mut u8,
    stride: isize,
    above: *const u8,
    upsample_above: i32,
    dx: i32,
) {
    // here upsample_above is 0 by design of av1_use_intra_edge_upsample
    let _ = upsample_above;
    let frac_bits: i32 = 6;
    let max_base_x: i32 = 64 + N - 1;
    // pre-filter above pixels
    // store in temp buffers:
    //   above[x] * 32 + 16
    //   above[x+1] - above[x]
    // final pixels will be calculated as:
    //   (above[x] * 32 + 16 + (above[x+1] - above[x]) * shift) >> 5
    let mut a0: __m128i;
    let mut a1: __m128i;
    let mut a32: __m128i;
    let mut a16: __m128i;
    let mut a_mbase_x: __m128i;
    let mut diff: __m128i;
    let mut c3f: __m128i;
    let mut max_base: __m128i;
    let mut base_inc: __m128i;
    let mut mask: __m128i;
    a16 = _mm_set1_epi16(16);
    a_mbase_x = _mm_set1_epi8(*above.offset(max_base_x as isize) as i8);
    max_base = _mm_set1_epi8(max_base_x as i8);
    c3f = _mm_set1_epi16(0x3f);
    let mut x: i32 = dx;
    {
        let mut r: i32 = 0;
        while r < N {
            let mut b: __m128i;
            let mut res: __m128i;
            let mut res1: __m128i;
            let mut base: i32 = x >> frac_bits;
            if base >= max_base_x {
                for i in r..N {
                    _mm_storeu_si128(dst as *mut __m128i, a_mbase_x); // save 32 values
                    _mm_storeu_si128(dst.offset(16) as *mut __m128i, a_mbase_x);
                    _mm_storeu_si128(dst.offset(32) as *mut __m128i, a_mbase_x);
                    _mm_storeu_si128(dst.offset(48) as *mut __m128i, a_mbase_x);
                    dst = dst.offset(stride);
                }
                return;
            }
            let mut shift: __m128i =
                _mm_srli_epi16::<{ 1 }>(_mm_and_si128(_mm_set1_epi16(x as i16), c3f)); // 8 element
            let mut a0_above: __m128i;
            let mut a1_above: __m128i;
            let mut res_val: __m128i;
            for j in (0..64).step_by(16) {
                let mut mdif: i32 = max_base_x - (base + j);
                if mdif <= 0 {
                    _mm_storeu_si128(dst.offset(j as isize) as *mut __m128i, a_mbase_x);
                } else {
                    a0_above = _mm_loadu_si128(above.offset(base as isize).offset(j as isize)
                        as *mut __m128i
                        as *const __m128i); // load 16 element
                    a1_above =
                        _mm_loadu_si128(above.offset(base as isize).offset(1).offset(j as isize)
                            as *mut __m128i
                            as *const __m128i);
                    // lower half
                    a0 = _mm_cvtepu8_epi16(a0_above);
                    a1 = _mm_cvtepu8_epi16(a1_above);
                    diff = _mm_sub_epi16(a1, a0); // a[x+1] - a[x]
                    a32 = _mm_slli_epi16::<{ 5 }>(a0); // a[x] * 32
                    a32 = _mm_add_epi16(a32, a16); // a[x] * 32 + 16
                    b = _mm_mullo_epi16(diff, shift);
                    res = _mm_add_epi16(a32, b);
                    res = _mm_srli_epi16::<{ 5 }>(res);
                    // uppar half
                    a0 = _mm_cvtepu8_epi16(_mm_srli_si128::<8>(a0_above));
                    a1 = _mm_cvtepu8_epi16(_mm_srli_si128::<8>(a1_above));
                    diff = _mm_sub_epi16(a1, a0); // a[x+1] - a[x]
                    a32 = _mm_slli_epi16::<{ 5 }>(a0); // a[x] * 32
                    a32 = _mm_add_epi16(a32, a16); // a[x] * 32 + 16
                    b = _mm_mullo_epi16(diff, shift);
                    res1 = _mm_add_epi16(a32, b);
                    res1 = _mm_srli_epi16::<{ 5 }>(res1);
                    res = _mm_packus_epi16(res, res1); // 16 8bit values
                    base_inc = _mm_setr_epi8(
                        (base + j) as i8,
                        (base + j + 1) as i8,
                        (base + j + 2) as i8,
                        (base + j + 3) as i8,
                        (base + j + 4) as i8,
                        (base + j + 5) as i8,
                        (base + j + 6) as i8,
                        (base + j + 7) as i8,
                        (base + j + 8) as i8,
                        (base + j + 9) as i8,
                        (base + j + 10) as i8,
                        (base + j + 11) as i8,
                        (base + j + 12) as i8,
                        (base + j + 13) as i8,
                        (base + j + 14) as i8,
                        (base + j + 15) as i8,
                    );
                    mask = _mm_cmpgt_epi8(_mm_subs_epu8(max_base, base_inc), _mm_setzero_si128());
                    res_val = _mm_blendv_epi8(a_mbase_x, res, mask);
                    _mm_storeu_si128(dst.offset(j as isize) as *mut __m128i, res_val);
                }
            }
            x += dx;
            r += 1;
            dst = dst.offset(stride);
        }
    }
}

// Directional prediction, zone 1: 0 < angle < 90
#[no_mangle]
#[target_feature(enable = "sse4.1")]
pub unsafe extern "C-unwind" fn av1_dr_prediction_z1_sse4_1(
    dst: *mut u8,
    stride: isize,
    bw: i32,
    bh: i32,
    above: *const u8,
    left: *const u8,
    upsample_above: i32,
    dx: i32,
    dy: i32,
) {
    let _ = left;
    let _ = dy;
    match bw {
        4 => {
            dr_prediction_z1_4xN_sse4_1(bh, dst, stride, above, upsample_above, dx);
        }
        8 => {
            dr_prediction_z1_8xN_sse4_1(bh, dst, stride, above, upsample_above, dx);
        }
        16 => {
            dr_prediction_z1_16xN_sse4_1(bh, dst, stride, above, upsample_above, dx);
        }
        32 => {
            dr_prediction_z1_32xN_sse4_1(bh, dst, stride, above, upsample_above, dx);
        }
        64 => {
            dr_prediction_z1_64xN_sse4_1(bh, dst, stride, above, upsample_above, dx);
        }
        _ => {
            debug_assert!(0 != 0, "Invalid block size");
        }
    }
    return;
}

#[inline]
#[target_feature(enable = "sse4.1")]
unsafe fn dr_prediction_z2_Nx4_sse4_1(
    N: i32,
    mut dst: *mut u8,
    stride: isize,
    above: *const u8,
    left: *const u8,
    upsample_above: i32,
    upsample_left: i32,
    dx: i32,
    dy: i32,
) {
    let min_base_x: i32 = -(1 << upsample_above);
    let min_base_y: i32 = -(1 << upsample_left);
    let frac_bits_x: i32 = 6 - upsample_above;
    let frac_bits_y: i32 = 6 - upsample_left;
    debug_assert!(dx > 0);
    // pre-filter above pixels
    // store in temp buffers:
    //   above[x] * 32 + 16
    //   above[x+1] - above[x]
    // final pixels will be calculated as:
    //   (above[x] * 32 + 16 + (above[x+1] - above[x]) * shift) >> 5
    let mut a0_x: __m128i;
    let mut a1_x: __m128i;
    let mut a32: __m128i;
    let mut diff: __m128i;
    let c3f: __m128i = _mm_set1_epi16(0x3f);
    let min_y_base: __m128i = _mm_set1_epi16(min_base_y as i16);
    let c1234: __m128i = _mm_setr_epi16(0, 1, 2, 3, 4, 0, 0, 0);
    let dy_reg: __m128i = _mm_set1_epi16(dy as i16);
    let a16: __m128i = _mm_set1_epi16(16);
    for r in 0..N {
        let mut b: __m128i;
        let mut res: __m128i;
        let mut shift: __m128i;
        let mut r6: __m128i;
        let mut ydx: __m128i;
        let mut resx: __m128i;
        let mut resy: __m128i;
        let mut resxy: __m128i;
        let mut a0_above: __m128i;
        let mut a1_above: __m128i;
        let mut y: i32 = r + 1;
        let mut base_x: i32 = (-y * dx) >> frac_bits_x;
        let mut base_shift: i32 = 0;
        if base_x < min_base_x - 1 {
            base_shift = (min_base_x - base_x - 1) >> upsample_above;
        }
        let mut base_min_diff: i32 = (min_base_x - base_x + upsample_above) >> upsample_above;
        if base_min_diff > 4 {
            base_min_diff = 4;
        } else {
            if base_min_diff < 0 {
                base_min_diff = 0;
            }
        }
        if base_shift > 3 {
            a0_x = _mm_setzero_si128();
            a1_x = _mm_setzero_si128();
            shift = _mm_setzero_si128();
        } else {
            a0_above = _mm_loadu_si128(above.offset(base_x as isize).offset(base_shift as isize)
                as *mut __m128i as *const __m128i);
            ydx = _mm_set1_epi16((y * dx) as i16);
            r6 = _mm_slli_epi16::<{ 6 }>(c1234);
            if upsample_above != 0 {
                a0_above = _mm_shuffle_epi8(
                    a0_above,
                    *(EvenOddMaskx.0[base_shift as usize].as_ptr() as *mut __m128i),
                );
                a1_above = _mm_srli_si128::<8>(a0_above);
                shift = _mm_srli_epi16::<{ 1 }>(_mm_and_si128(
                    _mm_sll_epi16(_mm_sub_epi16(r6, ydx), _mm_cvtsi32_si128(upsample_above)),
                    c3f,
                ));
            } else {
                a0_above = _mm_shuffle_epi8(
                    a0_above,
                    *(LoadMaskx.0[base_shift as usize].as_ptr() as *mut __m128i),
                );
                a1_above = _mm_srli_si128::<1>(a0_above);
                shift = _mm_srli_epi16::<{ 1 }>(_mm_and_si128(_mm_sub_epi16(r6, ydx), c3f));
            }
            a0_x = _mm_cvtepu8_epi16(a0_above);
            a1_x = _mm_cvtepu8_epi16(a1_above);
        }
        // y calc
        let mut a0_y: __m128i;
        let mut a1_y: __m128i;
        let mut shifty: __m128i;
        if base_x < min_base_x {
            let mut base_y_c: Aligned32<[i16; 8]> = core::mem::zeroed();
            let mut y_c: __m128i;
            let mut base_y_c_reg: __m128i;
            let mut mask: __m128i;
            let mut c1234_: __m128i;
            c1234_ = _mm_srli_si128::<2>(c1234);
            r6 = _mm_set1_epi16((r << 6) as i16);
            y_c = _mm_sub_epi16(r6, _mm_mullo_epi16(c1234_, dy_reg));
            base_y_c_reg = _mm_sra_epi16(y_c, _mm_cvtsi32_si128(frac_bits_y));
            mask = _mm_cmpgt_epi16(min_y_base, base_y_c_reg);
            base_y_c_reg = _mm_andnot_si128(mask, base_y_c_reg);
            _mm_store_si128(base_y_c.0.as_mut_ptr() as *mut __m128i, base_y_c_reg);
            a0_y = _mm_setr_epi16(
                *left.offset(base_y_c.0[0] as isize) as i16,
                *left.offset(base_y_c.0[1] as isize) as i16,
                *left.offset(base_y_c.0[2] as isize) as i16,
                *left.offset(base_y_c.0[3] as isize) as i16,
                0,
                0,
                0,
                0,
            );
            base_y_c_reg = _mm_add_epi16(base_y_c_reg, _mm_srli_epi16::<{ 4 }>(a16));
            _mm_store_si128(base_y_c.0.as_mut_ptr() as *mut __m128i, base_y_c_reg);
            a1_y = _mm_setr_epi16(
                *left.offset(base_y_c.0[0] as isize) as i16,
                *left.offset(base_y_c.0[1] as isize) as i16,
                *left.offset(base_y_c.0[2] as isize) as i16,
                *left.offset(base_y_c.0[3] as isize) as i16,
                0,
                0,
                0,
                0,
            );
            if upsample_left != 0 {
                shifty = _mm_srli_epi16::<{ 1 }>(_mm_and_si128(
                    _mm_sll_epi16(y_c, _mm_cvtsi32_si128(upsample_left)),
                    c3f,
                ));
            } else {
                shifty = _mm_srli_epi16::<{ 1 }>(_mm_and_si128(y_c, c3f));
            }
            a0_x = _mm_unpacklo_epi64(a0_x, a0_y);
            a1_x = _mm_unpacklo_epi64(a1_x, a1_y);
            shift = _mm_unpacklo_epi64(shift, shifty);
        }
        diff = _mm_sub_epi16(a1_x, a0_x); // a[x+1] - a[x]
        a32 = _mm_slli_epi16::<{ 5 }>(a0_x); // a[x] * 32
        a32 = _mm_add_epi16(a32, a16); // a[x] * 32 + 16
        b = _mm_mullo_epi16(diff, shift);
        res = _mm_add_epi16(a32, b);
        res = _mm_srli_epi16::<{ 5 }>(res);
        resx = _mm_packus_epi16(res, res);
        resy = _mm_srli_si128::<4>(resx);
        resxy = _mm_blendv_epi8(
            resx,
            resy,
            *(Mask.0[0][base_min_diff as usize].as_ptr() as *mut __m128i),
        );
        core::ptr::write_unaligned(dst as *mut i32, _mm_cvtsi128_si32(resxy));
        dst = dst.offset(stride);
    }
}

#[inline]
#[target_feature(enable = "sse4.1")]
unsafe fn dr_prediction_z2_Nx8_sse4_1(
    N: i32,
    mut dst: *mut u8,
    stride: isize,
    above: *const u8,
    left: *const u8,
    upsample_above: i32,
    upsample_left: i32,
    dx: i32,
    dy: i32,
) {
    let min_base_x: i32 = -(1 << upsample_above);
    let min_base_y: i32 = -(1 << upsample_left);
    let frac_bits_x: i32 = 6 - upsample_above;
    let frac_bits_y: i32 = 6 - upsample_left;
    // pre-filter above pixels
    // store in temp buffers:
    //   above[x] * 32 + 16
    //   above[x+1] - above[x]
    // final pixels will be calculated as:
    //   (above[x] * 32 + 16 + (above[x+1] - above[x]) * shift) >> 5
    let mut diff: __m128i;
    let mut a32: __m128i;
    let mut a0_x: __m128i;
    let mut a1_x: __m128i;
    let mut a0_y: __m128i;
    let mut a1_y: __m128i;
    let mut a0_above: __m128i;
    let mut a1_above: __m128i;
    let a16: __m128i = _mm_set1_epi16(16);
    let c3f: __m128i = _mm_set1_epi16(0x3f);
    let min_y_base: __m128i = _mm_set1_epi16(min_base_y as i16);
    let dy_reg: __m128i = _mm_set1_epi16(dy as i16);
    let c1234: __m128i = _mm_setr_epi16(1, 2, 3, 4, 5, 6, 7, 8);
    for r in 0..N {
        let mut b: __m128i;
        let mut res: __m128i;
        let mut res1: __m128i;
        let mut shift: __m128i;
        let mut resx: __m128i;
        let mut resy: __m128i;
        let mut resxy: __m128i;
        let mut r6: __m128i;
        let mut ydx: __m128i;
        let mut y: i32 = r + 1;
        let mut base_x: i32 = (-y * dx) >> frac_bits_x;
        let mut base_shift: i32 = 0;
        if base_x < min_base_x - 1 {
            base_shift = (min_base_x - base_x - 1) >> upsample_above;
        }
        let mut base_min_diff: i32 = (min_base_x - base_x + upsample_above) >> upsample_above;
        if base_min_diff > 8 {
            base_min_diff = 8;
        } else {
            if base_min_diff < 0 {
                base_min_diff = 0;
            }
        }
        if base_shift > 7 {
            resx = _mm_setzero_si128();
        } else {
            a0_above = _mm_loadu_si128(above.offset(base_x as isize).offset(base_shift as isize)
                as *mut __m128i as *const __m128i);
            ydx = _mm_set1_epi16((y * dx) as i16);
            r6 = _mm_slli_epi16::<{ 6 }>(_mm_srli_si128::<2>(c1234));
            if upsample_above != 0 {
                a0_above = _mm_shuffle_epi8(
                    a0_above,
                    *(EvenOddMaskx.0[base_shift as usize].as_ptr() as *mut __m128i),
                );
                a1_above = _mm_srli_si128::<8>(a0_above);
                shift = _mm_srli_epi16::<{ 1 }>(_mm_and_si128(
                    _mm_sll_epi16(_mm_sub_epi16(r6, ydx), _mm_cvtsi32_si128(upsample_above)),
                    c3f,
                ));
            } else {
                a1_above = _mm_srli_si128::<1>(a0_above);
                a0_above = _mm_shuffle_epi8(
                    a0_above,
                    *(LoadMaskx.0[base_shift as usize].as_ptr() as *mut __m128i),
                );
                a1_above = _mm_shuffle_epi8(
                    a1_above,
                    *(LoadMaskx.0[base_shift as usize].as_ptr() as *mut __m128i),
                );
                shift = _mm_srli_epi16::<{ 1 }>(_mm_and_si128(_mm_sub_epi16(r6, ydx), c3f));
            }
            a0_x = _mm_cvtepu8_epi16(a0_above);
            a1_x = _mm_cvtepu8_epi16(a1_above);
            diff = _mm_sub_epi16(a1_x, a0_x); // a[x+1] - a[x]
            a32 = _mm_slli_epi16::<{ 5 }>(a0_x); // a[x] * 32
            a32 = _mm_add_epi16(a32, a16); // a[x] * 32 + 16
            b = _mm_mullo_epi16(diff, shift);
            res = _mm_add_epi16(a32, b);
            res = _mm_srli_epi16::<{ 5 }>(res);
            resx = _mm_packus_epi16(res, res);
        }
        // y calc
        if base_x < min_base_x {
            let mut base_y_c: Aligned32<[i16; 16]> = core::mem::zeroed();
            let mut y_c: __m128i;
            let mut base_y_c_reg: __m128i;
            let mut mask: __m128i;
            r6 = _mm_set1_epi16((r << 6) as i16);
            y_c = _mm_sub_epi16(r6, _mm_mullo_epi16(c1234, dy_reg));
            base_y_c_reg = _mm_sra_epi16(y_c, _mm_cvtsi32_si128(frac_bits_y));
            mask = _mm_cmpgt_epi16(min_y_base, base_y_c_reg);
            base_y_c_reg = _mm_andnot_si128(mask, base_y_c_reg);
            _mm_store_si128(base_y_c.0.as_mut_ptr() as *mut __m128i, base_y_c_reg);
            a0_y = _mm_setr_epi16(
                *left.offset(base_y_c.0[0] as isize) as i16,
                *left.offset(base_y_c.0[1] as isize) as i16,
                *left.offset(base_y_c.0[2] as isize) as i16,
                *left.offset(base_y_c.0[3] as isize) as i16,
                *left.offset(base_y_c.0[4] as isize) as i16,
                *left.offset(base_y_c.0[5] as isize) as i16,
                *left.offset(base_y_c.0[6] as isize) as i16,
                *left.offset(base_y_c.0[7] as isize) as i16,
            );
            base_y_c_reg = _mm_add_epi16(base_y_c_reg, _mm_srli_epi16::<{ 4 }>(a16));
            _mm_store_si128(base_y_c.0.as_mut_ptr() as *mut __m128i, base_y_c_reg);
            a1_y = _mm_setr_epi16(
                *left.offset(base_y_c.0[0] as isize) as i16,
                *left.offset(base_y_c.0[1] as isize) as i16,
                *left.offset(base_y_c.0[2] as isize) as i16,
                *left.offset(base_y_c.0[3] as isize) as i16,
                *left.offset(base_y_c.0[4] as isize) as i16,
                *left.offset(base_y_c.0[5] as isize) as i16,
                *left.offset(base_y_c.0[6] as isize) as i16,
                *left.offset(base_y_c.0[7] as isize) as i16,
            );
            if upsample_left != 0 {
                shift = _mm_srli_epi16::<{ 1 }>(_mm_and_si128(
                    _mm_sll_epi16(y_c, _mm_cvtsi32_si128(upsample_left)),
                    c3f,
                ));
            } else {
                shift = _mm_srli_epi16::<{ 1 }>(_mm_and_si128(y_c, c3f));
            }
            diff = _mm_sub_epi16(a1_y, a0_y); // a[x+1] - a[x]
            a32 = _mm_slli_epi16::<{ 5 }>(a0_y); // a[x] * 32
            a32 = _mm_add_epi16(a32, a16); // a[x] * 32 + 16
            b = _mm_mullo_epi16(diff, shift);
            res1 = _mm_add_epi16(a32, b);
            res1 = _mm_srli_epi16::<{ 5 }>(res1);
            resy = _mm_packus_epi16(res1, res1);
            resxy = _mm_blendv_epi8(
                resx,
                resy,
                *(Mask.0[0][base_min_diff as usize].as_ptr() as *mut __m128i),
            );
            _mm_storel_epi64(dst as *mut __m128i, resxy);
        } else {
            _mm_storel_epi64(dst as *mut __m128i, resx);
        }
        dst = dst.offset(stride);
    }
}

#[inline]
#[target_feature(enable = "sse4.1")]
unsafe fn dr_prediction_z2_HxW_sse4_1(
    H: i32,
    W: i32,
    mut dst: *mut u8,
    stride: isize,
    above: *const u8,
    left: *const u8,
    upsample_above: i32,
    upsample_left: i32,
    dx: i32,
    dy: i32,
) {
    // here upsample_above and upsample_left are 0 by design of
    // av1_use_intra_edge_upsample
    let min_base_x: i32 = -1;
    let min_base_y: i32 = -1;
    let _ = upsample_above;
    let _ = upsample_left;
    let frac_bits_x: i32 = 6;
    let frac_bits_y: i32 = 6;
    let mut a0_x: __m128i;
    let mut a1_x: __m128i;
    let mut a0_y: __m128i;
    let mut a1_y: __m128i;
    let mut a0_y_h: __m128i;
    let mut a1_y_h: __m128i;
    let mut a32: __m128i;
    let mut diff: __m128i;
    let mut shifty: __m128i;
    let mut shifty_h: __m128i;
    let mut a0_above: __m128i;
    let mut a1_above: __m128i;
    let mut base_y_c: Aligned32<[i16; 16]> = core::mem::zeroed();
    let a16: __m128i = _mm_set1_epi16(16);
    let c1: __m128i = _mm_srli_epi16::<{ 4 }>(a16);
    let min_y_base: __m128i = _mm_set1_epi16(min_base_y as i16);
    let c3f: __m128i = _mm_set1_epi16(0x3f);
    let dy256: __m128i = _mm_set1_epi16(dy as i16);
    let c0123: __m128i = _mm_setr_epi16(0, 1, 2, 3, 4, 5, 6, 7);
    let c0123_h: __m128i = _mm_setr_epi16(8, 9, 10, 11, 12, 13, 14, 15);
    let c1234: __m128i = _mm_add_epi16(c0123, c1);
    let c1234_h: __m128i = _mm_add_epi16(c0123_h, c1);
    for r in 0..H {
        let mut b: __m128i;
        let mut res: __m128i;
        let mut res1: __m128i;
        let mut shift: __m128i;
        let mut reg_j: __m128i;
        let mut r6: __m128i;
        let mut ydx: __m128i;
        let mut resx: __m128i;
        let mut resy: __m128i;
        let mut resxy: __m128i;
        let mut y: i32 = r + 1;
        ydx = _mm_set1_epi16((y * dx) as i16);
        let mut base_x: i32 = (-y * dx) >> frac_bits_x;
        for j in (0..W).step_by(16) {
            reg_j = _mm_set1_epi16(j as i16);
            let mut base_shift: i32 = 0;
            if base_x + j < min_base_x - 1 {
                base_shift = min_base_x - (base_x + j) - 1;
            }
            let mut base_min_diff: i32 = min_base_x - base_x - j;
            if base_min_diff > 16 {
                base_min_diff = 16;
            } else {
                if base_min_diff < 0 {
                    base_min_diff = 0;
                }
            }
            if base_shift < 16 {
                a0_above = _mm_loadu_si128(
                    above
                        .offset(base_x as isize)
                        .offset(base_shift as isize)
                        .offset(j as isize) as *mut __m128i as *const __m128i,
                );
                a1_above = _mm_loadu_si128(
                    above
                        .offset(base_x as isize)
                        .offset(base_shift as isize)
                        .offset(1)
                        .offset(j as isize) as *mut __m128i as *const __m128i,
                );
                a0_above = _mm_shuffle_epi8(
                    a0_above,
                    *(LoadMaskx.0[base_shift as usize].as_ptr() as *mut __m128i),
                );
                a1_above = _mm_shuffle_epi8(
                    a1_above,
                    *(LoadMaskx.0[base_shift as usize].as_ptr() as *mut __m128i),
                );
                a0_x = _mm_cvtepu8_epi16(a0_above);
                a1_x = _mm_cvtepu8_epi16(a1_above);
                r6 = _mm_slli_epi16::<{ 6 }>(_mm_add_epi16(c0123, reg_j));
                shift = _mm_srli_epi16::<{ 1 }>(_mm_and_si128(_mm_sub_epi16(r6, ydx), c3f));
                diff = _mm_sub_epi16(a1_x, a0_x); // a[x+1] - a[x]
                a32 = _mm_slli_epi16::<{ 5 }>(a0_x); // a[x] * 32
                a32 = _mm_add_epi16(a32, a16); // a[x] * 32 + 16
                b = _mm_mullo_epi16(diff, shift);
                res = _mm_add_epi16(a32, b);
                res = _mm_srli_epi16::<{ 5 }>(res); // 16 16-bit values
                a0_x = _mm_cvtepu8_epi16(_mm_srli_si128::<8>(a0_above));
                a1_x = _mm_cvtepu8_epi16(_mm_srli_si128::<8>(a1_above));
                r6 = _mm_slli_epi16::<{ 6 }>(_mm_add_epi16(c0123_h, reg_j));
                shift = _mm_srli_epi16::<{ 1 }>(_mm_and_si128(_mm_sub_epi16(r6, ydx), c3f));
                diff = _mm_sub_epi16(a1_x, a0_x); // a[x+1] - a[x]
                a32 = _mm_slli_epi16::<{ 5 }>(a0_x); // a[x] * 32
                a32 = _mm_add_epi16(a32, a16); // a[x] * 32 + 16
                b = _mm_mullo_epi16(diff, shift);
                res1 = _mm_add_epi16(a32, b);
                res1 = _mm_srli_epi16::<{ 5 }>(res1); // 16 16-bit values
                resx = _mm_packus_epi16(res, res1);
            } else {
                resx = _mm_setzero_si128();
            }
            // y calc
            if base_x < min_base_x {
                let mut c_reg: __m128i;
                let mut c_reg_h: __m128i;
                let mut y_reg: __m128i;
                let mut y_reg_h: __m128i;
                let mut base_y: __m128i;
                let mut base_y_h: __m128i;
                let mut mask: __m128i;
                let mut mask_h: __m128i;
                let mut mul16: __m128i;
                let mut mul16_h: __m128i;
                r6 = _mm_set1_epi16((r << 6) as i16);
                c_reg = _mm_add_epi16(reg_j, c1234);
                c_reg_h = _mm_add_epi16(reg_j, c1234_h);
                mul16 = _mm_min_epu16(
                    _mm_mullo_epi16(c_reg, dy256),
                    _mm_srli_epi16::<{ 1 }>(min_y_base),
                );
                mul16_h = _mm_min_epu16(
                    _mm_mullo_epi16(c_reg_h, dy256),
                    _mm_srli_epi16::<{ 1 }>(min_y_base),
                );
                y_reg = _mm_sub_epi16(r6, mul16);
                y_reg_h = _mm_sub_epi16(r6, mul16_h);
                base_y = _mm_sra_epi16(y_reg, _mm_cvtsi32_si128(frac_bits_y));
                base_y_h = _mm_sra_epi16(y_reg_h, _mm_cvtsi32_si128(frac_bits_y));
                mask = _mm_cmpgt_epi16(min_y_base, base_y);
                mask_h = _mm_cmpgt_epi16(min_y_base, base_y_h);
                base_y = _mm_blendv_epi8(base_y, min_y_base, mask);
                base_y_h = _mm_blendv_epi8(base_y_h, min_y_base, mask_h);
                let mut min_y: i16 = _mm_extract_epi16::<7>(base_y_h) as i16;
                let mut max_y: i16 = _mm_extract_epi16::<0>(base_y) as i16;
                let mut offset_diff: i16 = (max_y as i32 - min_y as i32) as i16;
                if (offset_diff as i32) < 16 {
                    let mut min_y_reg: __m128i = _mm_set1_epi16(min_y);
                    let mut base_y_offset: __m128i = _mm_sub_epi16(base_y, min_y_reg);
                    let mut base_y_offset_h: __m128i = _mm_sub_epi16(base_y_h, min_y_reg);
                    let mut y_offset: __m128i = _mm_packs_epi16(base_y_offset, base_y_offset_h);
                    let mut a0_mask: __m128i = _mm_loadu_si128(left.offset(min_y as isize)
                        as *mut __m128i
                        as *const __m128i);
                    let mut a1_mask: __m128i = _mm_loadu_si128(
                        left.offset(min_y as isize).offset(1) as *mut __m128i as *const __m128i,
                    );
                    let mut LoadMask: __m128i =
                        _mm_loadu_si128(LoadMaskz2.0[(offset_diff as i32 / 4) as usize].as_ptr()
                            as *mut __m128i
                            as *const __m128i);
                    a0_mask = _mm_and_si128(a0_mask, LoadMask);
                    a1_mask = _mm_and_si128(a1_mask, LoadMask);
                    a0_mask = _mm_shuffle_epi8(a0_mask, y_offset);
                    a1_mask = _mm_shuffle_epi8(a1_mask, y_offset);
                    a0_y = _mm_cvtepu8_epi16(a0_mask);
                    a1_y = _mm_cvtepu8_epi16(a1_mask);
                    a0_y_h = _mm_cvtepu8_epi16(_mm_srli_si128::<8>(a0_mask));
                    a1_y_h = _mm_cvtepu8_epi16(_mm_srli_si128::<8>(a1_mask));
                } else {
                    base_y = _mm_andnot_si128(mask, base_y);
                    base_y_h = _mm_andnot_si128(mask_h, base_y_h);
                    _mm_store_si128(base_y_c.0.as_mut_ptr() as *mut __m128i, base_y);
                    _mm_store_si128(base_y_c.0.as_mut_ptr().offset(8) as *mut __m128i, base_y_h);
                    a0_y = _mm_setr_epi16(
                        *left.offset(base_y_c.0[0] as isize) as i16,
                        *left.offset(base_y_c.0[1] as isize) as i16,
                        *left.offset(base_y_c.0[2] as isize) as i16,
                        *left.offset(base_y_c.0[3] as isize) as i16,
                        *left.offset(base_y_c.0[4] as isize) as i16,
                        *left.offset(base_y_c.0[5] as isize) as i16,
                        *left.offset(base_y_c.0[6] as isize) as i16,
                        *left.offset(base_y_c.0[7] as isize) as i16,
                    );
                    a0_y_h = _mm_setr_epi16(
                        *left.offset(base_y_c.0[8] as isize) as i16,
                        *left.offset(base_y_c.0[9] as isize) as i16,
                        *left.offset(base_y_c.0[10] as isize) as i16,
                        *left.offset(base_y_c.0[11] as isize) as i16,
                        *left.offset(base_y_c.0[12] as isize) as i16,
                        *left.offset(base_y_c.0[13] as isize) as i16,
                        *left.offset(base_y_c.0[14] as isize) as i16,
                        *left.offset(base_y_c.0[15] as isize) as i16,
                    );
                    base_y = _mm_add_epi16(base_y, c1);
                    base_y_h = _mm_add_epi16(base_y_h, c1);
                    _mm_store_si128(base_y_c.0.as_mut_ptr() as *mut __m128i, base_y);
                    _mm_store_si128(base_y_c.0.as_mut_ptr().offset(8) as *mut __m128i, base_y_h);
                    a1_y = _mm_setr_epi16(
                        *left.offset(base_y_c.0[0] as isize) as i16,
                        *left.offset(base_y_c.0[1] as isize) as i16,
                        *left.offset(base_y_c.0[2] as isize) as i16,
                        *left.offset(base_y_c.0[3] as isize) as i16,
                        *left.offset(base_y_c.0[4] as isize) as i16,
                        *left.offset(base_y_c.0[5] as isize) as i16,
                        *left.offset(base_y_c.0[6] as isize) as i16,
                        *left.offset(base_y_c.0[7] as isize) as i16,
                    );
                    a1_y_h = _mm_setr_epi16(
                        *left.offset(base_y_c.0[8] as isize) as i16,
                        *left.offset(base_y_c.0[9] as isize) as i16,
                        *left.offset(base_y_c.0[10] as isize) as i16,
                        *left.offset(base_y_c.0[11] as isize) as i16,
                        *left.offset(base_y_c.0[12] as isize) as i16,
                        *left.offset(base_y_c.0[13] as isize) as i16,
                        *left.offset(base_y_c.0[14] as isize) as i16,
                        *left.offset(base_y_c.0[15] as isize) as i16,
                    );
                }
                shifty = _mm_srli_epi16::<{ 1 }>(_mm_and_si128(y_reg, c3f));
                shifty_h = _mm_srli_epi16::<{ 1 }>(_mm_and_si128(y_reg_h, c3f));
                diff = _mm_sub_epi16(a1_y, a0_y); // a[x+1] - a[x]
                a32 = _mm_slli_epi16::<{ 5 }>(a0_y); // a[x] * 32
                a32 = _mm_add_epi16(a32, a16); // a[x] * 32 + 16
                b = _mm_mullo_epi16(diff, shifty);
                res = _mm_add_epi16(a32, b);
                res = _mm_srli_epi16::<{ 5 }>(res); // 16 16-bit values
                diff = _mm_sub_epi16(a1_y_h, a0_y_h); // a[x+1] - a[x]
                a32 = _mm_slli_epi16::<{ 5 }>(a0_y_h); // a[x] * 32
                a32 = _mm_add_epi16(a32, a16); // a[x] * 32 + 16
                b = _mm_mullo_epi16(diff, shifty_h);
                res1 = _mm_add_epi16(a32, b);
                res1 = _mm_srli_epi16::<{ 5 }>(res1); // 16 16-bit values
                resy = _mm_packus_epi16(res, res1);
            } else {
                resy = _mm_setzero_si128();
            }
            resxy = _mm_blendv_epi8(
                resx,
                resy,
                *(Mask.0[0][base_min_diff as usize].as_ptr() as *mut __m128i),
            );
            _mm_storeu_si128(dst.offset(j as isize) as *mut __m128i, resxy);
        } // for j
        dst = dst.offset(stride);
    }
}

// Directional prediction, zone 2: 90 < angle < 180
#[no_mangle]
#[target_feature(enable = "sse4.1")]
pub unsafe extern "C-unwind" fn av1_dr_prediction_z2_sse4_1(
    dst: *mut u8,
    stride: isize,
    bw: i32,
    bh: i32,
    above: *const u8,
    left: *const u8,
    upsample_above: i32,
    upsample_left: i32,
    dx: i32,
    dy: i32,
) {
    debug_assert!(dx > 0);
    debug_assert!(dy > 0);
    match bw {
        4 => {
            dr_prediction_z2_Nx4_sse4_1(
                bh,
                dst,
                stride,
                above,
                left,
                upsample_above,
                upsample_left,
                dx,
                dy,
            );
        }
        8 => {
            dr_prediction_z2_Nx8_sse4_1(
                bh,
                dst,
                stride,
                above,
                left,
                upsample_above,
                upsample_left,
                dx,
                dy,
            );
        }
        _ => {
            dr_prediction_z2_HxW_sse4_1(
                bh,
                bw,
                dst,
                stride,
                above,
                left,
                upsample_above,
                upsample_left,
                dx,
                dy,
            );
        }
    }
    return;
}

// z3 functions
#[inline]
#[target_feature(enable = "sse4.1")]
unsafe fn dr_prediction_z3_4x4_sse4_1(
    dst: *mut u8,
    stride: isize,
    left: *const u8,
    upsample_left: i32,
    dy: i32,
) {
    let mut dstvec: [__m128i; 4] = core::mem::zeroed();
    let mut d: [__m128i; 4] = core::mem::zeroed();
    dr_prediction_z1_HxW_internal_sse4_1(4, 4, dstvec.as_mut_ptr(), left, upsample_left, dy);
    transpose4x8_8x4_low_sse2(
        dstvec.as_mut_ptr(),
        dstvec.as_mut_ptr().offset(1),
        dstvec.as_mut_ptr().offset(2),
        dstvec.as_mut_ptr().offset(3),
        d.as_mut_ptr(),
        d.as_mut_ptr().offset(1),
        d.as_mut_ptr().offset(2),
        d.as_mut_ptr().offset(3),
    );
    core::ptr::write_unaligned(dst.offset(stride * 0) as *mut i32, _mm_cvtsi128_si32(d[0]));
    core::ptr::write_unaligned(dst.offset(stride * 1) as *mut i32, _mm_cvtsi128_si32(d[1]));
    core::ptr::write_unaligned(dst.offset(stride * 2) as *mut i32, _mm_cvtsi128_si32(d[2]));
    core::ptr::write_unaligned(dst.offset(stride * 3) as *mut i32, _mm_cvtsi128_si32(d[3]));
    return;
}

#[inline]
#[target_feature(enable = "sse4.1")]
unsafe fn dr_prediction_z3_8x8_sse4_1(
    dst: *mut u8,
    stride: isize,
    left: *const u8,
    upsample_left: i32,
    dy: i32,
) {
    let mut dstvec: [__m128i; 8] = core::mem::zeroed();
    let mut d: [__m128i; 8] = core::mem::zeroed();
    dr_prediction_z1_HxW_internal_sse4_1(8, 8, dstvec.as_mut_ptr(), left, upsample_left, dy);
    transpose8x8_sse2(
        dstvec.as_mut_ptr(),
        dstvec.as_mut_ptr().offset(1),
        dstvec.as_mut_ptr().offset(2),
        dstvec.as_mut_ptr().offset(3),
        dstvec.as_mut_ptr().offset(4),
        dstvec.as_mut_ptr().offset(5),
        dstvec.as_mut_ptr().offset(6),
        dstvec.as_mut_ptr().offset(7),
        d.as_mut_ptr(),
        d.as_mut_ptr().offset(1),
        d.as_mut_ptr().offset(2),
        d.as_mut_ptr().offset(3),
    );
    _mm_storel_epi64(dst.offset(0 * stride) as *mut __m128i, d[0]);
    _mm_storel_epi64(
        dst.offset(1 * stride) as *mut __m128i,
        _mm_srli_si128::<8>(d[0]),
    );
    _mm_storel_epi64(dst.offset(2 * stride) as *mut __m128i, d[1]);
    _mm_storel_epi64(
        dst.offset(3 * stride) as *mut __m128i,
        _mm_srli_si128::<8>(d[1]),
    );
    _mm_storel_epi64(dst.offset(4 * stride) as *mut __m128i, d[2]);
    _mm_storel_epi64(
        dst.offset(5 * stride) as *mut __m128i,
        _mm_srli_si128::<8>(d[2]),
    );
    _mm_storel_epi64(dst.offset(6 * stride) as *mut __m128i, d[3]);
    _mm_storel_epi64(
        dst.offset(7 * stride) as *mut __m128i,
        _mm_srli_si128::<8>(d[3]),
    );
}

#[inline]
#[target_feature(enable = "sse4.1")]
unsafe fn dr_prediction_z3_4x8_sse4_1(
    dst: *mut u8,
    stride: isize,
    left: *const u8,
    upsample_left: i32,
    dy: i32,
) {
    let mut dstvec: [__m128i; 4] = core::mem::zeroed();
    let mut d: [__m128i; 8] = core::mem::zeroed();
    dr_prediction_z1_HxW_internal_sse4_1(8, 4, dstvec.as_mut_ptr(), left, upsample_left, dy);
    transpose4x8_8x4_sse2(
        dstvec.as_mut_ptr(),
        dstvec.as_mut_ptr().offset(1),
        dstvec.as_mut_ptr().offset(2),
        dstvec.as_mut_ptr().offset(3),
        d.as_mut_ptr(),
        d.as_mut_ptr().offset(1),
        d.as_mut_ptr().offset(2),
        d.as_mut_ptr().offset(3),
        d.as_mut_ptr().offset(4),
        d.as_mut_ptr().offset(5),
        d.as_mut_ptr().offset(6),
        d.as_mut_ptr().offset(7),
    );
    for i in 0..8 {
        core::ptr::write_unaligned(
            dst.offset(stride * i as isize) as *mut i32,
            _mm_cvtsi128_si32(d[i as usize]),
        );
    }
}

#[inline]
#[target_feature(enable = "sse4.1")]
unsafe fn dr_prediction_z3_8x4_sse4_1(
    dst: *mut u8,
    stride: isize,
    left: *const u8,
    upsample_left: i32,
    dy: i32,
) {
    let mut dstvec: [__m128i; 8] = core::mem::zeroed();
    let mut d: [__m128i; 4] = core::mem::zeroed();
    dr_prediction_z1_HxW_internal_sse4_1(4, 8, dstvec.as_mut_ptr(), left, upsample_left, dy);
    transpose8x8_low_sse2(
        dstvec.as_mut_ptr(),
        dstvec.as_mut_ptr().offset(1),
        dstvec.as_mut_ptr().offset(2),
        dstvec.as_mut_ptr().offset(3),
        dstvec.as_mut_ptr().offset(4),
        dstvec.as_mut_ptr().offset(5),
        dstvec.as_mut_ptr().offset(6),
        dstvec.as_mut_ptr().offset(7),
        d.as_mut_ptr(),
        d.as_mut_ptr().offset(1),
        d.as_mut_ptr().offset(2),
        d.as_mut_ptr().offset(3),
    );
    _mm_storel_epi64(dst.offset(0 * stride) as *mut __m128i, d[0]);
    _mm_storel_epi64(dst.offset(1 * stride) as *mut __m128i, d[1]);
    _mm_storel_epi64(dst.offset(2 * stride) as *mut __m128i, d[2]);
    _mm_storel_epi64(dst.offset(3 * stride) as *mut __m128i, d[3]);
}

#[inline]
#[target_feature(enable = "sse4.1")]
unsafe fn dr_prediction_z3_8x16_sse4_1(
    dst: *mut u8,
    stride: isize,
    left: *const u8,
    upsample_left: i32,
    dy: i32,
) {
    let mut dstvec: [__m128i; 8] = core::mem::zeroed();
    let mut d: [__m128i; 8] = core::mem::zeroed();
    dr_prediction_z1_HxW_internal_sse4_1(16, 8, dstvec.as_mut_ptr(), left, upsample_left, dy);
    transpose8x16_16x8_sse2(
        dstvec.as_mut_ptr(),
        dstvec.as_mut_ptr().offset(1),
        dstvec.as_mut_ptr().offset(2),
        dstvec.as_mut_ptr().offset(3),
        dstvec.as_mut_ptr().offset(4),
        dstvec.as_mut_ptr().offset(5),
        dstvec.as_mut_ptr().offset(6),
        dstvec.as_mut_ptr().offset(7),
        d.as_mut_ptr(),
        d.as_mut_ptr().offset(1),
        d.as_mut_ptr().offset(2),
        d.as_mut_ptr().offset(3),
        d.as_mut_ptr().offset(4),
        d.as_mut_ptr().offset(5),
        d.as_mut_ptr().offset(6),
        d.as_mut_ptr().offset(7),
    );
    for i in 0..8 {
        _mm_storel_epi64(
            dst.offset(i as isize * stride) as *mut __m128i,
            d[i as usize],
        );
        _mm_storel_epi64(
            dst.offset((i + 8) as isize * stride) as *mut __m128i,
            _mm_srli_si128::<8>(d[i as usize]),
        );
    }
}

#[inline]
#[target_feature(enable = "sse4.1")]
unsafe fn dr_prediction_z3_16x8_sse4_1(
    dst: *mut u8,
    stride: isize,
    left: *const u8,
    upsample_left: i32,
    dy: i32,
) {
    let mut dstvec: [__m128i; 16] = core::mem::zeroed();
    let mut d: [__m128i; 16] = core::mem::zeroed();
    dr_prediction_z1_HxW_internal_sse4_1(8, 16, dstvec.as_mut_ptr(), left, upsample_left, dy);
    transpose16x8_8x16_sse2(
        dstvec.as_mut_ptr(),
        dstvec.as_mut_ptr().offset(1),
        dstvec.as_mut_ptr().offset(2),
        dstvec.as_mut_ptr().offset(3),
        dstvec.as_mut_ptr().offset(4),
        dstvec.as_mut_ptr().offset(5),
        dstvec.as_mut_ptr().offset(6),
        dstvec.as_mut_ptr().offset(7),
        dstvec.as_mut_ptr().offset(8),
        dstvec.as_mut_ptr().offset(9),
        dstvec.as_mut_ptr().offset(10),
        dstvec.as_mut_ptr().offset(11),
        dstvec.as_mut_ptr().offset(12),
        dstvec.as_mut_ptr().offset(13),
        dstvec.as_mut_ptr().offset(14),
        dstvec.as_mut_ptr().offset(15),
        d.as_mut_ptr(),
        d.as_mut_ptr().offset(1),
        d.as_mut_ptr().offset(2),
        d.as_mut_ptr().offset(3),
        d.as_mut_ptr().offset(4),
        d.as_mut_ptr().offset(5),
        d.as_mut_ptr().offset(6),
        d.as_mut_ptr().offset(7),
    );
    for i in 0..8 {
        _mm_storeu_si128(
            dst.offset(i as isize * stride) as *mut __m128i,
            d[i as usize],
        );
    }
}

#[inline]
#[target_feature(enable = "sse4.1")]
unsafe fn dr_prediction_z3_4x16_sse4_1(
    dst: *mut u8,
    stride: isize,
    left: *const u8,
    upsample_left: i32,
    dy: i32,
) {
    let mut dstvec: [__m128i; 4] = core::mem::zeroed();
    let mut d: [__m128i; 16] = core::mem::zeroed();
    dr_prediction_z1_HxW_internal_sse4_1(16, 4, dstvec.as_mut_ptr(), left, upsample_left, dy);
    transpose4x16_sse2(dstvec.as_mut_ptr(), d.as_mut_ptr());
    for i in 0..16 {
        core::ptr::write_unaligned(
            dst.offset(stride * i as isize) as *mut i32,
            _mm_cvtsi128_si32(d[i as usize]),
        );
    }
}

#[inline]
#[target_feature(enable = "sse4.1")]
unsafe fn dr_prediction_z3_16x4_sse4_1(
    dst: *mut u8,
    stride: isize,
    left: *const u8,
    upsample_left: i32,
    dy: i32,
) {
    let mut dstvec: [__m128i; 16] = core::mem::zeroed();
    let mut d: [__m128i; 8] = core::mem::zeroed();
    dr_prediction_z1_HxW_internal_sse4_1(4, 16, dstvec.as_mut_ptr(), left, upsample_left, dy);
    for i in 4..8 {
        d[i as usize] = _mm_setzero_si128();
    }
    transpose16x8_8x16_sse2(
        dstvec.as_mut_ptr(),
        dstvec.as_mut_ptr().offset(1),
        dstvec.as_mut_ptr().offset(2),
        dstvec.as_mut_ptr().offset(3),
        dstvec.as_mut_ptr().offset(4),
        dstvec.as_mut_ptr().offset(5),
        dstvec.as_mut_ptr().offset(6),
        dstvec.as_mut_ptr().offset(7),
        dstvec.as_mut_ptr().offset(8),
        dstvec.as_mut_ptr().offset(9),
        dstvec.as_mut_ptr().offset(10),
        dstvec.as_mut_ptr().offset(11),
        dstvec.as_mut_ptr().offset(12),
        dstvec.as_mut_ptr().offset(13),
        dstvec.as_mut_ptr().offset(14),
        dstvec.as_mut_ptr().offset(15),
        d.as_mut_ptr(),
        d.as_mut_ptr().offset(1),
        d.as_mut_ptr().offset(2),
        d.as_mut_ptr().offset(3),
        d.as_mut_ptr().offset(4),
        d.as_mut_ptr().offset(5),
        d.as_mut_ptr().offset(6),
        d.as_mut_ptr().offset(7),
    );
    for i in 0..4 {
        _mm_storeu_si128(
            dst.offset(i as isize * stride) as *mut __m128i,
            d[i as usize],
        );
    }
}

#[inline]
#[target_feature(enable = "sse4.1")]
unsafe fn dr_prediction_z3_8x32_sse4_1(
    dst: *mut u8,
    stride: isize,
    left: *const u8,
    upsample_left: i32,
    dy: i32,
) {
    let mut dstvec: [__m128i; 16] = core::mem::zeroed();
    let mut d: [__m128i; 16] = core::mem::zeroed();
    let mut dstvec_h: [__m128i; 16] = core::mem::zeroed();
    let mut d_h: [__m128i; 16] = core::mem::zeroed();
    dr_prediction_z1_32xN_internal_sse4_1(
        8,
        dstvec.as_mut_ptr(),
        dstvec_h.as_mut_ptr(),
        left,
        upsample_left,
        dy,
    );
    for i in 8..16 {
        dstvec[i as usize] = _mm_setzero_si128();
        dstvec_h[i as usize] = _mm_setzero_si128();
    }
    transpose16x16_sse2(dstvec.as_mut_ptr(), d.as_mut_ptr());
    transpose16x16_sse2(dstvec_h.as_mut_ptr(), d_h.as_mut_ptr());
    for i in 0..16 {
        _mm_storel_epi64(
            dst.offset(i as isize * stride) as *mut __m128i,
            d[i as usize],
        );
    }
    for i in 0..16 {
        _mm_storel_epi64(
            dst.offset((i + 16) as isize * stride) as *mut __m128i,
            d_h[i as usize],
        );
    }
}

#[inline]
#[target_feature(enable = "sse4.1")]
unsafe fn dr_prediction_z3_32x8_sse4_1(
    dst: *mut u8,
    stride: isize,
    left: *const u8,
    upsample_left: i32,
    dy: i32,
) {
    let mut dstvec: [__m128i; 32] = core::mem::zeroed();
    let mut d: [__m128i; 16] = core::mem::zeroed();
    dr_prediction_z1_HxW_internal_sse4_1(8, 32, dstvec.as_mut_ptr(), left, upsample_left, dy);
    transpose16x8_8x16_sse2(
        dstvec.as_mut_ptr(),
        dstvec.as_mut_ptr().offset(1),
        dstvec.as_mut_ptr().offset(2),
        dstvec.as_mut_ptr().offset(3),
        dstvec.as_mut_ptr().offset(4),
        dstvec.as_mut_ptr().offset(5),
        dstvec.as_mut_ptr().offset(6),
        dstvec.as_mut_ptr().offset(7),
        dstvec.as_mut_ptr().offset(8),
        dstvec.as_mut_ptr().offset(9),
        dstvec.as_mut_ptr().offset(10),
        dstvec.as_mut_ptr().offset(11),
        dstvec.as_mut_ptr().offset(12),
        dstvec.as_mut_ptr().offset(13),
        dstvec.as_mut_ptr().offset(14),
        dstvec.as_mut_ptr().offset(15),
        d.as_mut_ptr(),
        d.as_mut_ptr().offset(1),
        d.as_mut_ptr().offset(2),
        d.as_mut_ptr().offset(3),
        d.as_mut_ptr().offset(4),
        d.as_mut_ptr().offset(5),
        d.as_mut_ptr().offset(6),
        d.as_mut_ptr().offset(7),
    );
    transpose16x8_8x16_sse2(
        dstvec.as_mut_ptr().offset((0 + 16) as isize),
        dstvec.as_mut_ptr().offset((1 + 16) as isize),
        dstvec.as_mut_ptr().offset((2 + 16) as isize),
        dstvec.as_mut_ptr().offset((3 + 16) as isize),
        dstvec.as_mut_ptr().offset((4 + 16) as isize),
        dstvec.as_mut_ptr().offset((5 + 16) as isize),
        dstvec.as_mut_ptr().offset((6 + 16) as isize),
        dstvec.as_mut_ptr().offset((7 + 16) as isize),
        dstvec.as_mut_ptr().offset((8 + 16) as isize),
        dstvec.as_mut_ptr().offset((9 + 16) as isize),
        dstvec.as_mut_ptr().offset((10 + 16) as isize),
        dstvec.as_mut_ptr().offset((11 + 16) as isize),
        dstvec.as_mut_ptr().offset((12 + 16) as isize),
        dstvec.as_mut_ptr().offset((13 + 16) as isize),
        dstvec.as_mut_ptr().offset((14 + 16) as isize),
        dstvec.as_mut_ptr().offset((15 + 16) as isize),
        d.as_mut_ptr().offset((0 + 8) as isize),
        d.as_mut_ptr().offset((1 + 8) as isize),
        d.as_mut_ptr().offset((2 + 8) as isize),
        d.as_mut_ptr().offset((3 + 8) as isize),
        d.as_mut_ptr().offset((4 + 8) as isize),
        d.as_mut_ptr().offset((5 + 8) as isize),
        d.as_mut_ptr().offset((6 + 8) as isize),
        d.as_mut_ptr().offset((7 + 8) as isize),
    );
    for i in 0..8 {
        _mm_storeu_si128(
            dst.offset(i as isize * stride) as *mut __m128i,
            d[i as usize],
        );
        _mm_storeu_si128(
            dst.offset(i as isize * stride).offset(16) as *mut __m128i,
            d[(i + 8) as usize],
        );
    }
}

// !CONFIG_REALTIME_ONLY || CONFIG_AV1_DECODER
#[inline]
#[target_feature(enable = "sse4.1")]
unsafe fn dr_prediction_z3_16x16_sse4_1(
    dst: *mut u8,
    stride: isize,
    left: *const u8,
    upsample_left: i32,
    dy: i32,
) {
    let mut dstvec: [__m128i; 16] = core::mem::zeroed();
    let mut d: [__m128i; 16] = core::mem::zeroed();
    dr_prediction_z1_HxW_internal_sse4_1(16, 16, dstvec.as_mut_ptr(), left, upsample_left, dy);
    transpose16x16_sse2(dstvec.as_mut_ptr(), d.as_mut_ptr());
    for i in 0..16 {
        _mm_storeu_si128(
            dst.offset(i as isize * stride) as *mut __m128i,
            d[i as usize],
        );
    }
}

#[inline]
#[target_feature(enable = "sse4.1")]
unsafe fn dr_prediction_z3_32x32_sse4_1(
    dst: *mut u8,
    stride: isize,
    left: *const u8,
    upsample_left: i32,
    dy: i32,
) {
    let mut dstvec: [__m128i; 32] = core::mem::zeroed();
    let mut d: [__m128i; 32] = core::mem::zeroed();
    let mut dstvec_h: [__m128i; 32] = core::mem::zeroed();
    let mut d_h: [__m128i; 32] = core::mem::zeroed();
    dr_prediction_z1_32xN_internal_sse4_1(
        32,
        dstvec.as_mut_ptr(),
        dstvec_h.as_mut_ptr(),
        left,
        upsample_left,
        dy,
    );
    transpose16x16_sse2(dstvec.as_mut_ptr(), d.as_mut_ptr());
    transpose16x16_sse2(dstvec_h.as_mut_ptr(), d_h.as_mut_ptr());
    transpose16x16_sse2(dstvec.as_mut_ptr().offset(16), d.as_mut_ptr().offset(16));
    transpose16x16_sse2(
        dstvec_h.as_mut_ptr().offset(16),
        d_h.as_mut_ptr().offset(16),
    );
    for j in 0..16 {
        _mm_storeu_si128(
            dst.offset(j as isize * stride) as *mut __m128i,
            d[j as usize],
        );
        _mm_storeu_si128(
            dst.offset(j as isize * stride).offset(16) as *mut __m128i,
            d[(j + 16) as usize],
        );
    }
    for j in 0..16 {
        _mm_storeu_si128(
            dst.offset((j + 16) as isize * stride) as *mut __m128i,
            d_h[j as usize],
        );
        _mm_storeu_si128(
            dst.offset((j + 16) as isize * stride).offset(16) as *mut __m128i,
            d_h[(j + 16) as usize],
        );
    }
}

#[inline]
#[target_feature(enable = "sse4.1")]
unsafe fn dr_prediction_z3_64x64_sse4_1(
    dst: *mut u8,
    stride: isize,
    left: *const u8,
    upsample_left: i32,
    dy: i32,
) {
    let mut dstT: [u8; (64 * 64) as usize] = core::mem::zeroed();
    dr_prediction_z1_64xN_sse4_1(64, dstT.as_mut_ptr(), 64, left, upsample_left, dy);
    transpose(dstT.as_mut_ptr() as *const u8, 64, dst, stride, 64, 64);
}

#[inline]
#[target_feature(enable = "sse4.1")]
unsafe fn dr_prediction_z3_16x32_sse4_1(
    dst: *mut u8,
    stride: isize,
    left: *const u8,
    upsample_left: i32,
    dy: i32,
) {
    let mut dstvec: [__m128i; 16] = core::mem::zeroed();
    let mut d: [__m128i; 16] = core::mem::zeroed();
    let mut dstvec_h: [__m128i; 16] = core::mem::zeroed();
    let mut d_h: [__m128i; 16] = core::mem::zeroed();
    dr_prediction_z1_32xN_internal_sse4_1(
        16,
        dstvec.as_mut_ptr(),
        dstvec_h.as_mut_ptr(),
        left,
        upsample_left,
        dy,
    );
    transpose16x16_sse2(dstvec.as_mut_ptr(), d.as_mut_ptr());
    transpose16x16_sse2(dstvec_h.as_mut_ptr(), d_h.as_mut_ptr());
    // store
    for j in 0..16 {
        _mm_storeu_si128(
            dst.offset(j as isize * stride) as *mut __m128i,
            d[j as usize],
        );
        _mm_storeu_si128(
            dst.offset((j + 16) as isize * stride) as *mut __m128i,
            d_h[j as usize],
        );
    }
}

#[inline]
#[target_feature(enable = "sse4.1")]
unsafe fn dr_prediction_z3_32x16_sse4_1(
    dst: *mut u8,
    stride: isize,
    left: *const u8,
    upsample_left: i32,
    dy: i32,
) {
    let mut dstvec: [__m128i; 32] = core::mem::zeroed();
    let mut d: [__m128i; 16] = core::mem::zeroed();
    dr_prediction_z1_HxW_internal_sse4_1(16, 32, dstvec.as_mut_ptr(), left, upsample_left, dy);
    for i in (0..32).step_by(16) {
        transpose16x16_sse2(dstvec.as_mut_ptr().offset(i as isize), d.as_mut_ptr());
        for j in 0..16 {
            _mm_storeu_si128(
                dst.offset(j as isize * stride).offset(i as isize) as *mut __m128i,
                d[j as usize],
            );
        }
    }
}

#[inline]
#[target_feature(enable = "sse4.1")]
unsafe fn dr_prediction_z3_32x64_sse4_1(
    dst: *mut u8,
    stride: isize,
    left: *const u8,
    upsample_left: i32,
    dy: i32,
) {
    let mut dstT: [u8; (64 * 32) as usize] = core::mem::zeroed();
    dr_prediction_z1_64xN_sse4_1(32, dstT.as_mut_ptr(), 64, left, upsample_left, dy);
    transpose(dstT.as_mut_ptr() as *const u8, 64, dst, stride, 32, 64);
}

#[inline]
#[target_feature(enable = "sse4.1")]
unsafe fn dr_prediction_z3_64x32_sse4_1(
    dst: *mut u8,
    stride: isize,
    left: *const u8,
    upsample_left: i32,
    dy: i32,
) {
    let mut dstT: [u8; (32 * 64) as usize] = core::mem::zeroed();
    dr_prediction_z1_32xN_sse4_1(64, dstT.as_mut_ptr(), 32, left, upsample_left, dy);
    transpose(dstT.as_mut_ptr() as *const u8, 32, dst, stride, 64, 32);
    return;
}

#[inline]
#[target_feature(enable = "sse4.1")]
unsafe fn dr_prediction_z3_16x64_sse4_1(
    dst: *mut u8,
    stride: isize,
    left: *const u8,
    upsample_left: i32,
    dy: i32,
) {
    let mut dstT: [u8; (64 * 16) as usize] = core::mem::zeroed();
    dr_prediction_z1_64xN_sse4_1(16, dstT.as_mut_ptr(), 64, left, upsample_left, dy);
    transpose(dstT.as_mut_ptr() as *const u8, 64, dst, stride, 16, 64);
}

#[inline]
#[target_feature(enable = "sse4.1")]
unsafe fn dr_prediction_z3_64x16_sse4_1(
    dst: *mut u8,
    stride: isize,
    left: *const u8,
    upsample_left: i32,
    dy: i32,
) {
    let mut dstvec: [__m128i; 64] = core::mem::zeroed();
    let mut d: [__m128i; 16] = core::mem::zeroed();
    dr_prediction_z1_HxW_internal_sse4_1(16, 64, dstvec.as_mut_ptr(), left, upsample_left, dy);
    for i in (0..64).step_by(16) {
        transpose16x16_sse2(dstvec.as_mut_ptr().offset(i as isize), d.as_mut_ptr());
        for j in 0..16 {
            _mm_storeu_si128(
                dst.offset(j as isize * stride).offset(i as isize) as *mut __m128i,
                d[j as usize],
            );
        }
    }
}

// !CONFIG_REALTIME_ONLY || CONFIG_AV1_DECODER
#[no_mangle]
#[target_feature(enable = "sse4.1")]
pub unsafe extern "C-unwind" fn av1_dr_prediction_z3_sse4_1(
    dst: *mut u8,
    stride: isize,
    bw: i32,
    bh: i32,
    above: *const u8,
    left: *const u8,
    upsample_left: i32,
    dx: i32,
    dy: i32,
) {
    let _ = above;
    let _ = dx;
    debug_assert!(dx == 1);
    debug_assert!(dy > 0);
    if bw == bh {
        match bw {
            4 => {
                dr_prediction_z3_4x4_sse4_1(dst, stride, left, upsample_left, dy);
            }
            8 => {
                dr_prediction_z3_8x8_sse4_1(dst, stride, left, upsample_left, dy);
            }
            16 => {
                dr_prediction_z3_16x16_sse4_1(dst, stride, left, upsample_left, dy);
            }
            32 => {
                dr_prediction_z3_32x32_sse4_1(dst, stride, left, upsample_left, dy);
            }
            64 => {
                dr_prediction_z3_64x64_sse4_1(dst, stride, left, upsample_left, dy);
            }
            _ => {
                debug_assert!(0 != 0, "Invalid block size");
            }
        }
    } else {
        if bw < bh {
            if bw + bw == bh {
                match bw {
                    4 => {
                        dr_prediction_z3_4x8_sse4_1(dst, stride, left, upsample_left, dy);
                    }
                    8 => {
                        dr_prediction_z3_8x16_sse4_1(dst, stride, left, upsample_left, dy);
                    }
                    16 => {
                        dr_prediction_z3_16x32_sse4_1(dst, stride, left, upsample_left, dy);
                    }
                    32 => {
                        dr_prediction_z3_32x64_sse4_1(dst, stride, left, upsample_left, dy);
                    }
                    _ => {
                        debug_assert!(0 != 0, "Invalid block size");
                    }
                }
            } else {
                match bw {
                    4 => {
                        dr_prediction_z3_4x16_sse4_1(dst, stride, left, upsample_left, dy);
                    }
                    8 => {
                        dr_prediction_z3_8x32_sse4_1(dst, stride, left, upsample_left, dy);
                    }
                    16 => {
                        dr_prediction_z3_16x64_sse4_1(dst, stride, left, upsample_left, dy);
                    }
                    _ => {
                        debug_assert!(0 != 0, "Invalid block size");
                    }
                }
            }
        } else {
            // !CONFIG_REALTIME_ONLY || CONFIG_AV1_DECODER
            if bh + bh == bw {
                match bh {
                    4 => {
                        dr_prediction_z3_8x4_sse4_1(dst, stride, left, upsample_left, dy);
                    }
                    8 => {
                        dr_prediction_z3_16x8_sse4_1(dst, stride, left, upsample_left, dy);
                    }
                    16 => {
                        dr_prediction_z3_32x16_sse4_1(dst, stride, left, upsample_left, dy);
                    }
                    32 => {
                        dr_prediction_z3_64x32_sse4_1(dst, stride, left, upsample_left, dy);
                    }
                    _ => {
                        debug_assert!(0 != 0, "Invalid block size");
                    }
                }
            } else {
                match bh {
                    4 => {
                        dr_prediction_z3_16x4_sse4_1(dst, stride, left, upsample_left, dy);
                    }
                    8 => {
                        dr_prediction_z3_32x8_sse4_1(dst, stride, left, upsample_left, dy);
                    }
                    16 => {
                        dr_prediction_z3_64x16_sse4_1(dst, stride, left, upsample_left, dy);
                    }
                    _ => {
                        debug_assert!(0 != 0, "Invalid block size");
                    }
                }
            }
        }
    }
}

// !CONFIG_REALTIME_ONLY || CONFIG_AV1_DECODER
