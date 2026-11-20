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

// Translated from upstream/aom/av1/common/x86/selfguided_sse4.c (ADR 0056, ADR 0058).
// Upstream compiles it with -msse41. The functions were
// transliterated with a typing-aware C-subset printer (not C2Rust) and
// reviewed against the C function by function: the same sequence of intrinsic
// and helper calls, constants and operators in order (ADR 0058).
// The restoration and filter macros of av1/common/restoration.h, aom_dsp/aom_filter.h,
// av1/common/enums.h, aom_dsp/aom_dsp_common.h, aom_ports/mem.h and av1/common/convolve.h
// are expanded with the C preprocessor before printing (tools/x86_c2r/c2r.py `cpp`).
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

use crate::aom_mem__aom_mem::{aom_free, aom_memalign};
use crate::av1__common__restoration::{
    av1_decode_xq, av1_one_by_x, av1_sgr_params, av1_x_by_xplus1, sgr_params_type,
};
use crate::simd::aom_dsp__x86__synonyms::*;
use core::arch::x86_64::*;

// Load 4 bytes from the possibly-misaligned pointer p, extend each byte to
// 32-bit precision and return them in an SSE register.
#[inline]
#[target_feature(enable = "sse4.1")]
unsafe fn xx_load_extend_8_32(p: *const core::ffi::c_void) -> __m128i {
    return _mm_cvtepu8_epi32(xx_loadl_32(p));
}

// Load 4 halfwords from the possibly-misaligned pointer p, extend each
// halfword to 32-bit precision and return them in an SSE register.
#[inline]
#[target_feature(enable = "sse4.1")]
unsafe fn xx_load_extend_16_32(p: *const core::ffi::c_void) -> __m128i {
    return _mm_cvtepu16_epi32(xx_loadl_64(p));
}

// Compute the scan of an SSE register holding 4 32-bit integers. If the
// register holds x0..x3 then the scan will hold x0, x0+x1, x0+x1+x2,
// x0+x1+x2+x3
#[inline]
#[target_feature(enable = "sse4.1")]
unsafe fn scan_32(x: __m128i) -> __m128i {
    let x01: __m128i = _mm_add_epi32(x, _mm_slli_si128::<4>(x));
    return _mm_add_epi32(x01, _mm_slli_si128::<8>(x01));
}

// Compute two integral images from src. B sums elements; A sums their
// squares. The images are offset by one pixel, so will have width and height
// equal to width + 1, height + 1 and the first row and column will be zero.
//
// A+1 and B+1 should be aligned to 16 bytes. buf_stride should be a multiple
// of 4.
#[inline]
#[target_feature(enable = "sse4.1")]
unsafe fn integral_images(
    src: *const u8,
    src_stride: i32,
    width: i32,
    height: i32,
    A: *mut i32,
    B: *mut i32,
    buf_stride: i32,
) {
    // Write out the zero top row
    core::ptr::write_bytes(
        A as *mut u8,
        0,
        core::mem::size_of::<i32>() * (width + 1) as usize,
    );
    core::ptr::write_bytes(
        B as *mut u8,
        0,
        core::mem::size_of::<i32>() * (width + 1) as usize,
    );
    let zero: __m128i = _mm_setzero_si128();
    for i in 0..height {
        // Zero the left column.
        *B.offset(((i + 1) * buf_stride) as isize) = 0;
        *A.offset(((i + 1) * buf_stride) as isize) = *B.offset(((i + 1) * buf_stride) as isize);
        // ldiff is the difference H - D where H is the output sample immediately
        // to the left and D is the output sample above it. These are scalars,
        // replicated across the four lanes.
        let mut ldiff1: __m128i = zero;
        let mut ldiff2: __m128i = zero;
        for j in (0..width).step_by(4) {
            let ABj: i32 = 1 + j;
            let above1: __m128i =
                xx_load_128(B.offset(ABj as isize).offset((i * buf_stride) as isize)
                    as *const core::ffi::c_void);
            let above2: __m128i =
                xx_load_128(A.offset(ABj as isize).offset((i * buf_stride) as isize)
                    as *const core::ffi::c_void);
            let x1: __m128i =
                xx_load_extend_8_32(src.offset(j as isize).offset((i * src_stride) as isize)
                    as *const core::ffi::c_void);
            let x2: __m128i = _mm_madd_epi16(x1, x1);
            let sc1: __m128i = scan_32(x1);
            let sc2: __m128i = scan_32(x2);
            let row1: __m128i = _mm_add_epi32(_mm_add_epi32(sc1, above1), ldiff1);
            let row2: __m128i = _mm_add_epi32(_mm_add_epi32(sc2, above2), ldiff2);
            xx_store_128(
                B.offset(ABj as isize)
                    .offset(((i + 1) * buf_stride) as isize)
                    as *mut core::ffi::c_void,
                row1,
            );
            xx_store_128(
                A.offset(ABj as isize)
                    .offset(((i + 1) * buf_stride) as isize)
                    as *mut core::ffi::c_void,
                row2,
            );
            // Calculate the new H - D.
            ldiff1 = _mm_shuffle_epi32::<0xff>(_mm_sub_epi32(row1, above1));
            ldiff2 = _mm_shuffle_epi32::<0xff>(_mm_sub_epi32(row2, above2));
        }
    }
}

// Compute two integral images from src. B sums elements; A sums their squares
//
// A and B should be aligned to 16 bytes. buf_stride should be a multiple of 4.
#[inline]
#[target_feature(enable = "sse4.1")]
unsafe fn integral_images_highbd(
    src: *const u16,
    src_stride: i32,
    width: i32,
    height: i32,
    A: *mut i32,
    B: *mut i32,
    buf_stride: i32,
) {
    // Write out the zero top row
    core::ptr::write_bytes(
        A as *mut u8,
        0,
        core::mem::size_of::<i32>() * (width + 1) as usize,
    );
    core::ptr::write_bytes(
        B as *mut u8,
        0,
        core::mem::size_of::<i32>() * (width + 1) as usize,
    );
    let zero: __m128i = _mm_setzero_si128();
    for i in 0..height {
        // Zero the left column.
        *B.offset(((i + 1) * buf_stride) as isize) = 0;
        *A.offset(((i + 1) * buf_stride) as isize) = *B.offset(((i + 1) * buf_stride) as isize);
        // ldiff is the difference H - D where H is the output sample immediately
        // to the left and D is the output sample above it. These are scalars,
        // replicated across the four lanes.
        let mut ldiff1: __m128i = zero;
        let mut ldiff2: __m128i = zero;
        for j in (0..width).step_by(4) {
            let ABj: i32 = 1 + j;
            let above1: __m128i =
                xx_load_128(B.offset(ABj as isize).offset((i * buf_stride) as isize)
                    as *const core::ffi::c_void);
            let above2: __m128i =
                xx_load_128(A.offset(ABj as isize).offset((i * buf_stride) as isize)
                    as *const core::ffi::c_void);
            let x1: __m128i =
                xx_load_extend_16_32(src.offset(j as isize).offset((i * src_stride) as isize)
                    as *const core::ffi::c_void);
            let x2: __m128i = _mm_madd_epi16(x1, x1);
            let sc1: __m128i = scan_32(x1);
            let sc2: __m128i = scan_32(x2);
            let row1: __m128i = _mm_add_epi32(_mm_add_epi32(sc1, above1), ldiff1);
            let row2: __m128i = _mm_add_epi32(_mm_add_epi32(sc2, above2), ldiff2);
            xx_store_128(
                B.offset(ABj as isize)
                    .offset(((i + 1) * buf_stride) as isize)
                    as *mut core::ffi::c_void,
                row1,
            );
            xx_store_128(
                A.offset(ABj as isize)
                    .offset(((i + 1) * buf_stride) as isize)
                    as *mut core::ffi::c_void,
                row2,
            );
            // Calculate the new H - D.
            ldiff1 = _mm_shuffle_epi32::<0xff>(_mm_sub_epi32(row1, above1));
            ldiff2 = _mm_shuffle_epi32::<0xff>(_mm_sub_epi32(row2, above2));
        }
    }
}

// Compute 4 values of boxsum from the given integral image. ii should point
// at the middle of the box (for the first value). r is the box radius.
#[inline]
#[target_feature(enable = "sse4.1")]
unsafe fn boxsum_from_ii(ii: *const i32, stride: i32, r: i32) -> __m128i {
    let tl: __m128i = xx_loadu_128(
        ii.offset(-((r + 1) as isize))
            .offset(-(((r + 1) * stride) as isize)) as *const core::ffi::c_void,
    );
    let tr: __m128i = xx_loadu_128(
        ii.offset((r + 0) as isize)
            .offset(-(((r + 1) * stride) as isize)) as *const core::ffi::c_void,
    );
    let bl: __m128i =
        xx_loadu_128(ii.offset(-((r + 1) as isize)).offset((r * stride) as isize)
            as *const core::ffi::c_void);
    let br: __m128i = xx_loadu_128(
        ii.offset((r + 0) as isize).offset((r * stride) as isize) as *const core::ffi::c_void
    );
    let u: __m128i = _mm_sub_epi32(tr, tl);
    let v: __m128i = _mm_sub_epi32(br, bl);
    return _mm_sub_epi32(v, u);
}

#[inline]
#[target_feature(enable = "sse4.1")]
unsafe fn round_for_shift(shift: u32) -> __m128i {
    return _mm_set1_epi32((1 << shift) >> 1);
}

#[inline]
#[target_feature(enable = "sse4.1")]
unsafe fn compute_p(sum1: __m128i, sum2: __m128i, bit_depth: i32, n: i32) -> __m128i {
    let mut an: __m128i;
    let mut bb: __m128i;
    if bit_depth > 8 {
        let rounding_a: __m128i = round_for_shift((2 * (bit_depth - 8)) as u32);
        let rounding_b: __m128i = round_for_shift((bit_depth - 8) as u32);
        let shift_a: __m128i = _mm_cvtsi32_si128(2 * (bit_depth - 8));
        let shift_b: __m128i = _mm_cvtsi32_si128(bit_depth - 8);
        let a: __m128i = _mm_srl_epi32(_mm_add_epi32(sum2, rounding_a), shift_a);
        let b: __m128i = _mm_srl_epi32(_mm_add_epi32(sum1, rounding_b), shift_b);
        // b < 2^14, so we can use a 16-bit madd rather than a 32-bit
        // mullo to square it
        bb = _mm_madd_epi16(b, b);
        an = _mm_max_epi32(_mm_mullo_epi32(a, _mm_set1_epi32(n)), bb);
    } else {
        bb = _mm_madd_epi16(sum1, sum1);
        an = _mm_mullo_epi32(sum2, _mm_set1_epi32(n));
    }
    return _mm_sub_epi32(an, bb);
}

// Assumes that C, D are integral images for the original buffer which has been
// extended to have a padding of SGRPROJ_BORDER_VERT/SGRPROJ_BORDER_HORZ pixels
// on the sides. A, B, C, D point at logical position (0, 0).
#[inline]
#[target_feature(enable = "sse4.1")]
unsafe fn calc_ab(
    A: *mut i32,
    B: *mut i32,
    C: *const i32,
    D: *const i32,
    width: i32,
    height: i32,
    buf_stride: i32,
    bit_depth: i32,
    sgr_params_idx: i32,
    radius_idx: i32,
) {
    let mut params: *const sgr_params_type =
        av1_sgr_params.as_ptr().offset(sgr_params_idx as isize);
    let r: i32 = (*params).r[radius_idx as usize];
    let n: i32 = (2 * r + 1) * (2 * r + 1);
    let s: __m128i = _mm_set1_epi32((*params).s[radius_idx as usize]);
    // one_over_n[n-1] is 2^12/n, so easily fits in an int16
    let one_over_n: __m128i = _mm_set1_epi32(av1_one_by_x[(n - 1) as usize]);
    let rnd_z: __m128i = round_for_shift(20);
    let rnd_res: __m128i = round_for_shift(12);
    // Set up masks
    let ones32: __m128i = _mm_set_epi32(0, 0, !0, !0);
    let mut mask: [__m128i; 4] = core::mem::zeroed();
    for idx in 0..4 {
        let shift: __m128i = _mm_cvtsi32_si128(8 * (4 - idx));
        mask[idx as usize] = _mm_cvtepi8_epi32(_mm_srl_epi64(ones32, shift));
    }
    for i in -1..height + 1 {
        for j in (-1..width + 1).step_by(4) {
            let mut Cij: *const i32 = C.offset((i * buf_stride) as isize).offset(j as isize);
            let mut Dij: *const i32 = D.offset((i * buf_stride) as isize).offset(j as isize);
            let mut sum1: __m128i = boxsum_from_ii(Dij, buf_stride, r);
            let mut sum2: __m128i = boxsum_from_ii(Cij, buf_stride, r);
            // When width + 2 isn't a multiple of 4, sum1 and sum2 will contain
            // some uninitialised data in their upper words. We use a mask to
            // ensure that these bits are set to 0.
            let mut idx: i32 = if 4 < width + 1 - j { 4 } else { width + 1 - j };
            debug_assert!(idx >= 1);
            if idx < 4 {
                sum1 = _mm_and_si128(mask[idx as usize], sum1);
                sum2 = _mm_and_si128(mask[idx as usize], sum2);
            }
            let p: __m128i = compute_p(sum1, sum2, bit_depth, n);
            let z: __m128i = _mm_min_epi32(
                _mm_srli_epi32::<{ 20 }>(_mm_add_epi32(_mm_mullo_epi32(p, s), rnd_z)),
                _mm_set1_epi32(255),
            );
            // 'Gather' type instructions are not available pre-AVX2, so synthesize a
            // gather using scalar loads.
            let a_res: __m128i = _mm_set_epi32(
                av1_x_by_xplus1[_mm_extract_epi32::<3>(z) as usize],
                av1_x_by_xplus1[_mm_extract_epi32::<2>(z) as usize],
                av1_x_by_xplus1[_mm_extract_epi32::<1>(z) as usize],
                av1_x_by_xplus1[_mm_extract_epi32::<0>(z) as usize],
            );
            xx_storeu_128(
                A.offset((i * buf_stride) as isize).offset(j as isize) as *mut core::ffi::c_void,
                a_res,
            );
            let a_complement: __m128i = _mm_sub_epi32(_mm_set1_epi32(1 << 8), a_res);
            // sum1 might have lanes greater than 2^15, so we can't use madd to do
            // multiplication involving sum1. However, a_complement and one_over_n
            // are both less than 256, so we can multiply them first.
            let a_comp_over_n: __m128i = _mm_madd_epi16(a_complement, one_over_n);
            let b_int: __m128i = _mm_mullo_epi32(a_comp_over_n, sum1);
            let b_res: __m128i = _mm_srli_epi32::<{ 12 }>(_mm_add_epi32(b_int, rnd_res));
            xx_storeu_128(
                B.offset((i * buf_stride) as isize).offset(j as isize) as *mut core::ffi::c_void,
                b_res,
            );
        }
    }
}

// Calculate 4 values of the "cross sum" starting at buf. This is a 3x3 filter
// where the outer four corners have weight 3 and all other pixels have weight
// 4.
//
// Pixels are indexed like this:
// xtl  xt   xtr
// xl    x   xr
// xbl  xb   xbr
//
// buf points to x
//
// fours = xl + xt + xr + xb + x
// threes = xtl + xtr + xbr + xbl
// cross_sum = 4 * fours + 3 * threes
//           = 4 * (fours + threes) - threes
//           = (fours + threes) << 2 - threes
#[inline]
#[target_feature(enable = "sse4.1")]
unsafe fn cross_sum(buf: *const i32, stride: i32) -> __m128i {
    let xtl: __m128i =
        xx_loadu_128(buf.offset(-1).offset(-(stride as isize)) as *const core::ffi::c_void);
    let xt: __m128i = xx_loadu_128(buf.offset(-(stride as isize)) as *const core::ffi::c_void);
    let xtr: __m128i =
        xx_loadu_128(buf.offset(1).offset(-(stride as isize)) as *const core::ffi::c_void);
    let xl: __m128i = xx_loadu_128(buf.offset(-1) as *const core::ffi::c_void);
    let x: __m128i = xx_loadu_128(buf as *const core::ffi::c_void);
    let xr: __m128i = xx_loadu_128(buf.offset(1) as *const core::ffi::c_void);
    let xbl: __m128i =
        xx_loadu_128(buf.offset(-1).offset(stride as isize) as *const core::ffi::c_void);
    let xb: __m128i = xx_loadu_128(buf.offset(stride as isize) as *const core::ffi::c_void);
    let xbr: __m128i =
        xx_loadu_128(buf.offset(1).offset(stride as isize) as *const core::ffi::c_void);
    let fours: __m128i = _mm_add_epi32(
        xl,
        _mm_add_epi32(xt, _mm_add_epi32(xr, _mm_add_epi32(xb, x))),
    );
    let threes: __m128i = _mm_add_epi32(xtl, _mm_add_epi32(xtr, _mm_add_epi32(xbr, xbl)));
    return _mm_sub_epi32(
        _mm_slli_epi32::<{ 2 }>(_mm_add_epi32(fours, threes)),
        threes,
    );
}

// The final filter for self-guided restoration. Computes a weighted average
// across A, B with "cross sums" (see cross_sum implementation above).
#[inline]
#[target_feature(enable = "sse4.1")]
unsafe fn final_filter(
    dst: *mut i32,
    dst_stride: i32,
    A: *const i32,
    B: *const i32,
    buf_stride: i32,
    dgd8: *const core::ffi::c_void,
    dgd_stride: i32,
    width: i32,
    height: i32,
    highbd: i32,
) {
    let nb: i32 = 5;
    let rounding: __m128i = round_for_shift((8 + nb - 4) as u32);
    let mut dgd_real: *const u8 = if highbd != 0 {
        ((dgd8 as usize) << 1) as *mut u16 as *const u8
    } else {
        dgd8 as *const u8
    };
    for i in 0..height {
        for j in (0..width).step_by(4) {
            let a: __m128i = cross_sum(
                A.offset((i * buf_stride) as isize).offset(j as isize),
                buf_stride,
            );
            let b: __m128i = cross_sum(
                B.offset((i * buf_stride) as isize).offset(j as isize),
                buf_stride,
            );
            let raw: __m128i =
                xx_loadl_64(dgd_real.offset(((i * dgd_stride + j) << highbd) as isize)
                    as *const core::ffi::c_void);
            let src: __m128i = if highbd != 0 {
                _mm_cvtepu16_epi32(raw)
            } else {
                _mm_cvtepu8_epi32(raw)
            };
            let mut v: __m128i = _mm_add_epi32(_mm_madd_epi16(a, src), b);
            let mut w: __m128i =
                _mm_sra_epi32(_mm_add_epi32(v, rounding), _mm_cvtsi32_si128(8 + nb - 4));
            xx_storeu_128(
                dst.offset((i * dst_stride) as isize).offset(j as isize) as *mut core::ffi::c_void,
                w,
            );
        }
    }
}

// Assumes that C, D are integral images for the original buffer which has been
// extended to have a padding of SGRPROJ_BORDER_VERT/SGRPROJ_BORDER_HORZ pixels
// on the sides. A, B, C, D point at logical position (0, 0).
#[inline]
#[target_feature(enable = "sse4.1")]
unsafe fn calc_ab_fast(
    A: *mut i32,
    B: *mut i32,
    C: *const i32,
    D: *const i32,
    width: i32,
    height: i32,
    buf_stride: i32,
    bit_depth: i32,
    sgr_params_idx: i32,
    radius_idx: i32,
) {
    let mut params: *const sgr_params_type =
        av1_sgr_params.as_ptr().offset(sgr_params_idx as isize);
    let r: i32 = (*params).r[radius_idx as usize];
    let n: i32 = (2 * r + 1) * (2 * r + 1);
    let s: __m128i = _mm_set1_epi32((*params).s[radius_idx as usize]);
    // one_over_n[n-1] is 2^12/n, so easily fits in an int16
    let one_over_n: __m128i = _mm_set1_epi32(av1_one_by_x[(n - 1) as usize]);
    let rnd_z: __m128i = round_for_shift(20);
    let rnd_res: __m128i = round_for_shift(12);
    // Set up masks
    let ones32: __m128i = _mm_set_epi32(0, 0, !0, !0);
    let mut mask: [__m128i; 4] = core::mem::zeroed();
    for idx in 0..4 {
        let shift: __m128i = _mm_cvtsi32_si128(8 * (4 - idx));
        mask[idx as usize] = _mm_cvtepi8_epi32(_mm_srl_epi64(ones32, shift));
    }
    for i in (-1..height + 1).step_by(2) {
        for j in (-1..width + 1).step_by(4) {
            let mut Cij: *const i32 = C.offset((i * buf_stride) as isize).offset(j as isize);
            let mut Dij: *const i32 = D.offset((i * buf_stride) as isize).offset(j as isize);
            let mut sum1: __m128i = boxsum_from_ii(Dij, buf_stride, r);
            let mut sum2: __m128i = boxsum_from_ii(Cij, buf_stride, r);
            // When width + 2 isn't a multiple of 4, sum1 and sum2 will contain
            // some uninitialised data in their upper words. We use a mask to
            // ensure that these bits are set to 0.
            let mut idx: i32 = if 4 < width + 1 - j { 4 } else { width + 1 - j };
            debug_assert!(idx >= 1);
            if idx < 4 {
                sum1 = _mm_and_si128(mask[idx as usize], sum1);
                sum2 = _mm_and_si128(mask[idx as usize], sum2);
            }
            let p: __m128i = compute_p(sum1, sum2, bit_depth, n);
            let z: __m128i = _mm_min_epi32(
                _mm_srli_epi32::<{ 20 }>(_mm_add_epi32(_mm_mullo_epi32(p, s), rnd_z)),
                _mm_set1_epi32(255),
            );
            // 'Gather' type instructions are not available pre-AVX2, so synthesize a
            // gather using scalar loads.
            let a_res: __m128i = _mm_set_epi32(
                av1_x_by_xplus1[_mm_extract_epi32::<3>(z) as usize],
                av1_x_by_xplus1[_mm_extract_epi32::<2>(z) as usize],
                av1_x_by_xplus1[_mm_extract_epi32::<1>(z) as usize],
                av1_x_by_xplus1[_mm_extract_epi32::<0>(z) as usize],
            );
            xx_storeu_128(
                A.offset((i * buf_stride) as isize).offset(j as isize) as *mut core::ffi::c_void,
                a_res,
            );
            let a_complement: __m128i = _mm_sub_epi32(_mm_set1_epi32(1 << 8), a_res);
            // sum1 might have lanes greater than 2^15, so we can't use madd to do
            // multiplication involving sum1. However, a_complement and one_over_n
            // are both less than 256, so we can multiply them first.
            let a_comp_over_n: __m128i = _mm_madd_epi16(a_complement, one_over_n);
            let b_int: __m128i = _mm_mullo_epi32(a_comp_over_n, sum1);
            let b_res: __m128i = _mm_srli_epi32::<{ 12 }>(_mm_add_epi32(b_int, rnd_res));
            xx_storeu_128(
                B.offset((i * buf_stride) as isize).offset(j as isize) as *mut core::ffi::c_void,
                b_res,
            );
        }
    }
}

// Calculate 4 values of the "cross sum" starting at buf.
//
// Pixels are indexed like this:
// xtl  xt   xtr
//  -   buf   -
// xbl  xb   xbr
//
// Pixels are weighted like this:
//  5    6    5
//  0    0    0
//  5    6    5
//
// fives = xtl + xtr + xbl + xbr
// sixes = xt + xb
// cross_sum = 6 * sixes + 5 * fives
//           = 5 * (fives + sixes) - sixes
//           = (fives + sixes) << 2 + (fives + sixes) + sixes
#[inline]
#[target_feature(enable = "sse4.1")]
unsafe fn cross_sum_fast_even_row(buf: *const i32, stride: i32) -> __m128i {
    let xtl: __m128i =
        xx_loadu_128(buf.offset(-1).offset(-(stride as isize)) as *const core::ffi::c_void);
    let xt: __m128i = xx_loadu_128(buf.offset(-(stride as isize)) as *const core::ffi::c_void);
    let xtr: __m128i =
        xx_loadu_128(buf.offset(1).offset(-(stride as isize)) as *const core::ffi::c_void);
    let xbl: __m128i =
        xx_loadu_128(buf.offset(-1).offset(stride as isize) as *const core::ffi::c_void);
    let xb: __m128i = xx_loadu_128(buf.offset(stride as isize) as *const core::ffi::c_void);
    let xbr: __m128i =
        xx_loadu_128(buf.offset(1).offset(stride as isize) as *const core::ffi::c_void);
    let fives: __m128i = _mm_add_epi32(xtl, _mm_add_epi32(xtr, _mm_add_epi32(xbr, xbl)));
    let sixes: __m128i = _mm_add_epi32(xt, xb);
    let fives_plus_sixes: __m128i = _mm_add_epi32(fives, sixes);
    return _mm_add_epi32(
        _mm_add_epi32(_mm_slli_epi32::<{ 2 }>(fives_plus_sixes), fives_plus_sixes),
        sixes,
    );
}

// Calculate 4 values of the "cross sum" starting at buf.
//
// Pixels are indexed like this:
// xl    x   xr
//
// Pixels are weighted like this:
//  5    6    5
//
// buf points to x
//
// fives = xl + xr
// sixes = x
// cross_sum = 5 * fives + 6 * sixes
//           = 4 * (fives + sixes) + (fives + sixes) + sixes
//           = (fives + sixes) << 2 + (fives + sixes) + sixes
#[inline]
#[target_feature(enable = "sse4.1")]
unsafe fn cross_sum_fast_odd_row(buf: *const i32) -> __m128i {
    let xl: __m128i = xx_loadu_128(buf.offset(-1) as *const core::ffi::c_void);
    let x: __m128i = xx_loadu_128(buf as *const core::ffi::c_void);
    let xr: __m128i = xx_loadu_128(buf.offset(1) as *const core::ffi::c_void);
    let fives: __m128i = _mm_add_epi32(xl, xr);
    let sixes: __m128i = x;
    let fives_plus_sixes: __m128i = _mm_add_epi32(fives, sixes);
    return _mm_add_epi32(
        _mm_add_epi32(_mm_slli_epi32::<{ 2 }>(fives_plus_sixes), fives_plus_sixes),
        sixes,
    );
}

// The final filter for the self-guided restoration. Computes a
// weighted average across A, B with "cross sums" (see cross_sum_...
// implementations above).
#[inline]
#[target_feature(enable = "sse4.1")]
unsafe fn final_filter_fast(
    dst: *mut i32,
    dst_stride: i32,
    A: *const i32,
    B: *const i32,
    buf_stride: i32,
    dgd8: *const core::ffi::c_void,
    dgd_stride: i32,
    width: i32,
    height: i32,
    highbd: i32,
) {
    let nb0: i32 = 5;
    let nb1: i32 = 4;
    let rounding0: __m128i = round_for_shift((8 + nb0 - 4) as u32);
    let rounding1: __m128i = round_for_shift((8 + nb1 - 4) as u32);
    let mut dgd_real: *const u8 = if highbd != 0 {
        ((dgd8 as usize) << 1) as *mut u16 as *const u8
    } else {
        dgd8 as *const u8
    };
    for i in 0..height {
        if !(i & 1 != 0) {
            // even row
            for j in (0..width).step_by(4) {
                let a: __m128i = cross_sum_fast_even_row(
                    A.offset((i * buf_stride) as isize).offset(j as isize),
                    buf_stride,
                );
                let b: __m128i = cross_sum_fast_even_row(
                    B.offset((i * buf_stride) as isize).offset(j as isize),
                    buf_stride,
                );
                let raw: __m128i =
                    xx_loadl_64(dgd_real.offset(((i * dgd_stride + j) << highbd) as isize)
                        as *const core::ffi::c_void);
                let src: __m128i = if highbd != 0 {
                    _mm_cvtepu16_epi32(raw)
                } else {
                    _mm_cvtepu8_epi32(raw)
                };
                let mut v: __m128i = _mm_add_epi32(_mm_madd_epi16(a, src), b);
                let mut w: __m128i =
                    _mm_sra_epi32(_mm_add_epi32(v, rounding0), _mm_cvtsi32_si128(8 + nb0 - 4));
                xx_storeu_128(
                    dst.offset((i * dst_stride) as isize).offset(j as isize)
                        as *mut core::ffi::c_void,
                    w,
                );
            }
        } else {
            // odd row
            for j in (0..width).step_by(4) {
                let a: __m128i =
                    cross_sum_fast_odd_row(A.offset((i * buf_stride) as isize).offset(j as isize));
                let b: __m128i =
                    cross_sum_fast_odd_row(B.offset((i * buf_stride) as isize).offset(j as isize));
                let raw: __m128i =
                    xx_loadl_64(dgd_real.offset(((i * dgd_stride + j) << highbd) as isize)
                        as *const core::ffi::c_void);
                let src: __m128i = if highbd != 0 {
                    _mm_cvtepu16_epi32(raw)
                } else {
                    _mm_cvtepu8_epi32(raw)
                };
                let mut v: __m128i = _mm_add_epi32(_mm_madd_epi16(a, src), b);
                let mut w: __m128i =
                    _mm_sra_epi32(_mm_add_epi32(v, rounding1), _mm_cvtsi32_si128(8 + nb1 - 4));
                xx_storeu_128(
                    dst.offset((i * dst_stride) as isize).offset(j as isize)
                        as *mut core::ffi::c_void,
                    w,
                );
            }
        }
    }
}

#[no_mangle]
#[target_feature(enable = "sse4.1")]
pub unsafe extern "C-unwind" fn av1_selfguided_restoration_sse4_1(
    dgd8: *const u8,
    width: i32,
    height: i32,
    dgd_stride: i32,
    flt0: *mut i32,
    flt1: *mut i32,
    flt_stride: i32,
    sgr_params_idx: i32,
    bit_depth: i32,
    highbd: i32,
) -> i32 {
    let mut buf: *mut i32 = aom_memalign(
        16,
        4 * core::mem::size_of::<i32>() * ((64 + 3 * 2 + 20) * (64 + 3 * 2 + 20)) as usize,
    ) as *mut i32;
    if !!buf.is_null() {
        return -1;
    }
    core::ptr::write_bytes(
        buf as *mut u8,
        0,
        4 * core::mem::size_of::<i32>() * ((64 + 3 * 2 + 20) * (64 + 3 * 2 + 20)) as usize,
    );
    let width_ext: i32 = width + 2 * 3;
    let height_ext: i32 = height + 2 * 3;
    // Adjusting the stride of A and B here appears to avoid bad cache effects,
    // leading to a significant speed improvement.
    // We also align the stride to a multiple of 16 bytes for efficiency.
    let mut buf_stride: i32 = ((width_ext + 3) & !3) + 16;
    // The "tl" pointers point at the top-left of the initialised data for the
    // array. Adding 3 here ensures that column 1 is 16-byte aligned.
    let mut Atl: *mut i32 = buf
        .offset((0 * ((64 + 3 * 2 + 20) * (64 + 3 * 2 + 20))) as isize)
        .offset(3);
    let mut Btl: *mut i32 = buf
        .offset((1 * ((64 + 3 * 2 + 20) * (64 + 3 * 2 + 20))) as isize)
        .offset(3);
    let mut Ctl: *mut i32 = buf
        .offset((2 * ((64 + 3 * 2 + 20) * (64 + 3 * 2 + 20))) as isize)
        .offset(3);
    let mut Dtl: *mut i32 = buf
        .offset((3 * ((64 + 3 * 2 + 20) * (64 + 3 * 2 + 20))) as isize)
        .offset(3);
    // The "0" pointers are (- SGRPROJ_BORDER_VERT, -SGRPROJ_BORDER_HORZ). Note
    // there's a zero row and column in A, B (integral images), so we move down
    // and right one for them.
    let buf_diag_border: i32 = 3 + buf_stride * 3;
    let mut A0: *mut i32 = Atl.offset(1).offset(buf_stride as isize);
    let mut B0: *mut i32 = Btl.offset(1).offset(buf_stride as isize);
    let mut C0: *mut i32 = Ctl.offset(1).offset(buf_stride as isize);
    let mut D0: *mut i32 = Dtl.offset(1).offset(buf_stride as isize);
    // Finally, A, B, C, D point at position (0, 0).
    let mut A: *mut i32 = A0.offset(buf_diag_border as isize);
    let mut B: *mut i32 = B0.offset(buf_diag_border as isize);
    let mut C: *mut i32 = C0.offset(buf_diag_border as isize);
    let mut D: *mut i32 = D0.offset(buf_diag_border as isize);
    let dgd_diag_border: i32 = 3 + dgd_stride * 3;
    let mut dgd0: *const u8 = dgd8.offset(-(dgd_diag_border as isize));
    // Generate integral images from the input. C will contain sums of squares; D
    // will contain just sums
    if highbd != 0 {
        integral_images_highbd(
            ((dgd0 as usize) << 1) as *mut u16 as *const u16,
            dgd_stride,
            width_ext,
            height_ext,
            Ctl,
            Dtl,
            buf_stride,
        );
    } else {
        integral_images(
            dgd0, dgd_stride, width_ext, height_ext, Ctl, Dtl, buf_stride,
        );
    }
    let mut params: *const sgr_params_type =
        av1_sgr_params.as_ptr().offset(sgr_params_idx as isize);
    // Write to flt0 and flt1
    // If params->r == 0 we skip the corresponding filter. We only allow one of
    // the radii to be 0, as having both equal to 0 would be equivalent to
    // skipping SGR entirely.
    debug_assert!(!((*params).r[0] == 0 && (*params).r[1] == 0));
    debug_assert!((*params).r[0] < (if 3 < 3 { 3 } else { 3 }));
    debug_assert!((*params).r[1] < (if 3 < 3 { 3 } else { 3 }));
    if (*params).r[0] > 0 {
        calc_ab_fast(
            A,
            B,
            C as *const i32,
            D as *const i32,
            width,
            height,
            buf_stride,
            bit_depth,
            sgr_params_idx,
            0,
        );
        final_filter_fast(
            flt0,
            flt_stride,
            A as *const i32,
            B as *const i32,
            buf_stride,
            dgd8 as *const core::ffi::c_void,
            dgd_stride,
            width,
            height,
            highbd,
        );
    }
    if (*params).r[1] > 0 {
        calc_ab(
            A,
            B,
            C as *const i32,
            D as *const i32,
            width,
            height,
            buf_stride,
            bit_depth,
            sgr_params_idx,
            1,
        );
        final_filter(
            flt1,
            flt_stride,
            A as *const i32,
            B as *const i32,
            buf_stride,
            dgd8 as *const core::ffi::c_void,
            dgd_stride,
            width,
            height,
            highbd,
        );
    }
    aom_free(buf as *mut core::ffi::c_void);
    return 0;
}

#[no_mangle]
#[target_feature(enable = "sse4.1")]
pub unsafe extern "C-unwind" fn av1_apply_selfguided_restoration_sse4_1(
    dat8: *const u8,
    width: i32,
    height: i32,
    stride: i32,
    eps: i32,
    xqd: *const i32,
    dst8: *mut u8,
    dst_stride: i32,
    tmpbuf: *mut i32,
    bit_depth: i32,
    highbd: i32,
) -> i32 {
    let mut flt0: *mut i32 = tmpbuf;
    let mut flt1: *mut i32 =
        flt0.offset(((256 * 3 / 2 + 2 * 3 + 16) * (256 * 3 / 2 + 2 * 3 + 8)) as isize);
    debug_assert!(width * height <= (256 * 3 / 2 + 2 * 3 + 16) * (256 * 3 / 2 + 2 * 3 + 8));
    let ret: i32 = av1_selfguided_restoration_sse4_1(
        dat8, width, height, stride, flt0, flt1, width, eps, bit_depth, highbd,
    );
    if ret != 0 {
        return ret;
    }
    let mut params: *const sgr_params_type = av1_sgr_params.as_ptr().offset(eps as isize);
    let mut xq: [i32; 2] = core::mem::zeroed();
    av1_decode_xq(xqd, xq.as_mut_ptr(), params);
    let mut xq0: __m128i = _mm_set1_epi32(xq[0]);
    let mut xq1: __m128i = _mm_set1_epi32(xq[1]);
    for i in 0..height {
        // Calculate output in batches of 8 pixels
        for j in (0..width).step_by(8) {
            let k: i32 = i * width + j;
            let m: i32 = i * dst_stride + j;
            let mut dat8ij: *const u8 = dat8.offset((i * stride) as isize).offset(j as isize);
            let mut src: __m128i;
            if highbd != 0 {
                src =
                    xx_loadu_128(((dat8ij as usize) << 1) as *mut u16 as *const core::ffi::c_void);
            } else {
                src = _mm_cvtepu8_epi16(xx_loadl_64(dat8ij as *const core::ffi::c_void));
            }
            let u: __m128i = _mm_slli_epi16::<{ 4 }>(src);
            let u_0: __m128i = _mm_cvtepu16_epi32(u);
            let u_1: __m128i = _mm_cvtepu16_epi32(_mm_srli_si128::<8>(u));
            let mut v_0: __m128i = _mm_slli_epi32::<{ 7 }>(u_0);
            let mut v_1: __m128i = _mm_slli_epi32::<{ 7 }>(u_1);
            if (*params).r[0] > 0 {
                let f1_0: __m128i = _mm_sub_epi32(
                    xx_loadu_128(flt0.offset(k as isize) as *const core::ffi::c_void),
                    u_0,
                );
                v_0 = _mm_add_epi32(v_0, _mm_mullo_epi32(xq0, f1_0));
                let f1_1: __m128i = _mm_sub_epi32(
                    xx_loadu_128(flt0.offset((k + 4) as isize) as *const core::ffi::c_void),
                    u_1,
                );
                v_1 = _mm_add_epi32(v_1, _mm_mullo_epi32(xq0, f1_1));
            }
            if (*params).r[1] > 0 {
                let f2_0: __m128i = _mm_sub_epi32(
                    xx_loadu_128(flt1.offset(k as isize) as *const core::ffi::c_void),
                    u_0,
                );
                v_0 = _mm_add_epi32(v_0, _mm_mullo_epi32(xq1, f2_0));
                let f2_1: __m128i = _mm_sub_epi32(
                    xx_loadu_128(flt1.offset((k + 4) as isize) as *const core::ffi::c_void),
                    u_1,
                );
                v_1 = _mm_add_epi32(v_1, _mm_mullo_epi32(xq1, f2_1));
            }
            let rounding: __m128i = round_for_shift((7 + 4) as u32);
            let w_0: __m128i = _mm_srai_epi32::<{ 7 + 4 }>(_mm_add_epi32(v_0, rounding));
            let w_1: __m128i = _mm_srai_epi32::<{ 7 + 4 }>(_mm_add_epi32(v_1, rounding));
            if highbd != 0 {
                // Pack into 16 bits and clamp to [0, 2^bit_depth)
                let tmp: __m128i = _mm_packus_epi32(w_0, w_1);
                let max: __m128i = _mm_set1_epi16(((1 << bit_depth) - 1) as i16);
                let res: __m128i = _mm_min_epi16(tmp, max);
                xx_storeu_128(
                    ((dst8.offset(m as isize) as usize) << 1) as *mut u16 as *mut core::ffi::c_void,
                    res,
                );
            } else {
                // Pack into 8 bits and clamp to [0, 256)
                let tmp: __m128i = _mm_packs_epi32(w_0, w_1);
                let res: __m128i = _mm_packus_epi16(tmp, tmp);
                // "don't care" value
                xx_storel_64(dst8.offset(m as isize) as *mut core::ffi::c_void, res);
            }
        }
    }
    return 0;
}
