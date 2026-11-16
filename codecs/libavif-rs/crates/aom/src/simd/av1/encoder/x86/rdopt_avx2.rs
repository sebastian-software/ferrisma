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

// Translated from upstream/aom/av1/encoder/x86/rdopt_avx2.c (ADR 0056, ADR 0058).
// Upstream compiles it with -mavx2. The functions were
// transliterated with a typing-aware C-subset printer (not C2Rust) and
// reviewed against the C function by function: the same sequence of intrinsic
// and helper calls, constants and operators in order (ADR 0058).
// The final floating-point block is the same sequence of single-precision
// conversions and operations as the original, with no contraction, and calls
// the C library's sqrtf as the original does.
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

use crate::simd::aom_dsp__x86__mem_sse2::loadu_int64;
use crate::simd::aom_dsp__x86__synonyms::*;
use crate::simd::aom_dsp__x86__synonyms_avx2::*;
use core::arch::x86_64::*;

extern "C" {
    fn sqrtf(_: f32) -> f32;
}

// Process horizontal and vertical correlations in a 4x4 block of pixels.
// We actually use the 4x4 pixels to calculate correlations corresponding to
// the top-left 3x3 pixels, so this function must be called with 1x1 overlap,
// moving the window along/down by 3 pixels at a time.
#[inline]
#[target_feature(enable = "avx2")]
unsafe fn horver_correlation_4x4(
    diff: *const i16,
    stride: i32,
    xy_sum_32: *mut __m256i,
    xz_sum_32: *mut __m256i,
    x_sum_32: *mut __m256i,
    x2_sum_32: *mut __m256i,
) {
    // Pixels in this 4x4   [ a b c d ]
    // are referred to as:  [ e f g h ]
    //                      [ i j k l ]
    //                      [ m n o p ]
    let pixels: __m256i = _mm256_set_epi64x(
        loadu_int64(diff.offset((0 * stride) as isize) as *const core::ffi::c_void),
        loadu_int64(diff.offset((1 * stride) as isize) as *const core::ffi::c_void),
        loadu_int64(diff.offset((2 * stride) as isize) as *const core::ffi::c_void),
        loadu_int64(diff.offset((3 * stride) as isize) as *const core::ffi::c_void),
    );
    // pixels = [d c b a h g f e] [l k j i p o n m] as i16
    let slli: __m256i = _mm256_slli_epi64::<{ 16 }>(pixels);
    // slli = [c b a 0 g f e 0] [k j i 0 o n m 0] as i16
    let madd_xy: __m256i = _mm256_madd_epi16(pixels, slli);
    // madd_xy = [bc+cd ab fg+gh ef] [jk+kl ij no+op mn] as i32
    *xy_sum_32 = _mm256_add_epi32(*xy_sum_32, madd_xy);
    // Permute control [3 2] [1 0] => [2 1] [0 0], 0b10010000 = 0x90
    let perm: __m256i = _mm256_permute4x64_epi64::<0x90>(slli);
    // perm = [g f e 0 k j i 0] [o n m 0 o n m 0] as i16
    let madd_xz: __m256i = _mm256_madd_epi16(slli, perm);
    // madd_xz = [cg+bf ae gk+fj ei] [ko+jn im oo+nn mm] as i32
    *xz_sum_32 = _mm256_add_epi32(*xz_sum_32, madd_xz);
    // Sum every element in slli (and then also their squares)
    let madd1_slli: __m256i = _mm256_madd_epi16(slli, _mm256_set1_epi16(1));
    // madd1_slli = [c+b a g+f e] [k+j i o+n m] as i32
    *x_sum_32 = _mm256_add_epi32(*x_sum_32, madd1_slli);
    let madd_slli: __m256i = _mm256_madd_epi16(slli, slli);
    // madd_slli = [cc+bb aa gg+ff ee] [kk+jj ii oo+nn mm] as i32
    *x2_sum_32 = _mm256_add_epi32(*x2_sum_32, madd_slli);
}

#[no_mangle]
#[target_feature(enable = "avx2")]
pub unsafe extern "C-unwind" fn av1_get_horver_correlation_full_avx2(
    diff: *const i16,
    stride: i32,
    width: i32,
    height: i32,
    hcorr: *mut f32,
    vcorr: *mut f32,
) {
    // The following notation is used:
    // x - current pixel
    // y - right neighbour pixel
    // z - below neighbour pixel
    // w - down-right neighbour pixel
    let mut xy_sum: i64 = 0;
    let mut xz_sum: i64 = 0;
    let mut x_sum: i64 = 0;
    let mut x2_sum: i64 = 0;
    // Process horizontal and vertical correlations through the body in 4x4
    // blocks.  This excludes the final row and column and possibly one extra
    // column depending how 3 divides into width and height
    let mut xy_xz_tmp: [i32; 8] = [0, 0, 0, 0, 0, 0, 0, 0];
    let mut x_x2_tmp: [i32; 8] = [0, 0, 0, 0, 0, 0, 0, 0];
    let mut xy_sum_32: __m256i = _mm256_setzero_si256();
    let mut xz_sum_32: __m256i = _mm256_setzero_si256();
    let mut x_sum_32: __m256i = _mm256_setzero_si256();
    let mut x2_sum_32: __m256i = _mm256_setzero_si256();
    for i in (0..=height - 4).step_by(3) {
        for j in (0..=width - 4).step_by(3) {
            horver_correlation_4x4(
                diff.offset((i * stride + j) as isize),
                stride,
                &raw mut xy_sum_32,
                &raw mut xz_sum_32,
                &raw mut x_sum_32,
                &raw mut x2_sum_32,
            );
        }
        let hadd_xy_xz: __m256i = _mm256_hadd_epi32(xy_sum_32, xz_sum_32);
        // hadd_xy_xz = [ae+bf+cg ei+fj+gk ab+bc+cd ef+fg+gh]
        //              [im+jn+ko mm+nn+oo ij+jk+kl mn+no+op] as i32
        yy_storeu_256(xy_xz_tmp.as_mut_ptr() as *mut core::ffi::c_void, hadd_xy_xz);
        xy_sum += xy_xz_tmp[5] as i64 + xy_xz_tmp[4] as i64 + xy_xz_tmp[1] as i64;
        xz_sum += xy_xz_tmp[7] as i64 + xy_xz_tmp[6] as i64 + xy_xz_tmp[3] as i64;
        let hadd_x_x2: __m256i = _mm256_hadd_epi32(x_sum_32, x2_sum_32);
        // hadd_x_x2 = [aa+bb+cc ee+ff+gg a+b+c e+f+g]
        //             [ii+jj+kk mm+nn+oo i+j+k m+n+o] as i32
        yy_storeu_256(x_x2_tmp.as_mut_ptr() as *mut core::ffi::c_void, hadd_x_x2);
        x_sum += x_x2_tmp[5] as i64 + x_x2_tmp[4] as i64 + x_x2_tmp[1] as i64;
        x2_sum += x_x2_tmp[7] as i64 + x_x2_tmp[6] as i64 + x_x2_tmp[3] as i64;
        xy_sum_32 = _mm256_setzero_si256();
        xz_sum_32 = _mm256_setzero_si256();
        x_sum_32 = _mm256_setzero_si256();
        x2_sum_32 = _mm256_setzero_si256();
    }
    // x_sum now covers every pixel except the final 1-2 rows and 1-2 cols
    let mut x_finalrow: i64 = 0;
    let mut x_finalcol: i64 = 0;
    let mut x2_finalrow: i64 = 0;
    let mut x2_finalcol: i64 = 0;
    // Do we have 2 rows remaining or just the one?  Note that width and height
    // are powers of 2, so each modulo 3 must be 1 or 2.
    if height % 3 == 1 {
        // Just horiz corrs on the final row
        let x0: i16 = *diff.offset(((height - 1) * stride) as isize);
        x_sum += x0 as i64;
        x_finalrow += x0 as i64;
        x2_sum += (x0 as i32 * x0 as i32) as i64;
        x2_finalrow += (x0 as i32 * x0 as i32) as i64;
        for j in 0..width - 1 {
            let x: i16 = *diff.offset(((height - 1) * stride + j) as isize);
            let y: i16 = *diff.offset(((height - 1) * stride + j + 1) as isize);
            xy_sum += (x as i32 * y as i32) as i64;
            x_sum += y as i64;
            x2_sum += (y as i32 * y as i32) as i64;
            x_finalrow += y as i64;
            x2_finalrow += (y as i32 * y as i32) as i64;
        }
    } else {
        // Two rows remaining to do
        let x0: i16 = *diff.offset(((height - 2) * stride) as isize);
        let z0: i16 = *diff.offset(((height - 1) * stride) as isize);
        x_sum += (x0 as i32 + z0 as i32) as i64;
        x2_sum += (x0 as i32 * x0 as i32 + z0 as i32 * z0 as i32) as i64;
        x_finalrow += z0 as i64;
        x2_finalrow += (z0 as i32 * z0 as i32) as i64;
        for j in 0..width - 1 {
            let x: i16 = *diff.offset(((height - 2) * stride + j) as isize);
            let y: i16 = *diff.offset(((height - 2) * stride + j + 1) as isize);
            let z: i16 = *diff.offset(((height - 1) * stride + j) as isize);
            let w: i16 = *diff.offset(((height - 1) * stride + j + 1) as isize);
            // Horizontal and vertical correlations for the penultimate row:
            xy_sum += (x as i32 * y as i32) as i64;
            xz_sum += (x as i32 * z as i32) as i64;
            // Now just horizontal correlations for the final row:
            xy_sum += (z as i32 * w as i32) as i64;
            x_sum += (y as i32 + w as i32) as i64;
            x2_sum += (y as i32 * y as i32 + w as i32 * w as i32) as i64;
            x_finalrow += w as i64;
            x2_finalrow += (w as i32 * w as i32) as i64;
        }
    }
    // Do we have 2 columns remaining or just the one?
    if width % 3 == 1 {
        // Just vert corrs on the final col
        let x0: i16 = *diff.offset((width - 1) as isize);
        x_sum += x0 as i64;
        x_finalcol += x0 as i64;
        x2_sum += (x0 as i32 * x0 as i32) as i64;
        x2_finalcol += (x0 as i32 * x0 as i32) as i64;
        for i in 0..height - 1 {
            let x: i16 = *diff.offset((i * stride + width - 1) as isize);
            let z: i16 = *diff.offset(((i + 1) * stride + width - 1) as isize);
            xz_sum += (x as i32 * z as i32) as i64;
            x_finalcol += z as i64;
            x2_finalcol += (z as i32 * z as i32) as i64;
            // So the bottom-right elements don't get counted twice:
            if i < height - (if height % 3 == 1 { 2 } else { 3 }) {
                x_sum += z as i64;
                x2_sum += (z as i32 * z as i32) as i64;
            }
        }
    } else {
        // Two cols remaining
        let x0: i16 = *diff.offset((width - 2) as isize);
        let y0: i16 = *diff.offset((width - 1) as isize);
        x_sum += (x0 as i32 + y0 as i32) as i64;
        x2_sum += (x0 as i32 * x0 as i32 + y0 as i32 * y0 as i32) as i64;
        x_finalcol += y0 as i64;
        x2_finalcol += (y0 as i32 * y0 as i32) as i64;
        for i in 0..height - 1 {
            let x: i16 = *diff.offset((i * stride + width - 2) as isize);
            let y: i16 = *diff.offset((i * stride + width - 1) as isize);
            let z: i16 = *diff.offset(((i + 1) * stride + width - 2) as isize);
            let w: i16 = *diff.offset(((i + 1) * stride + width - 1) as isize);
            // Horizontal and vertical correlations for the penultimate col:
            // Skip these on the last iteration of this loop if we also had two
            // rows remaining, otherwise the final horizontal and vertical correlation
            // get erroneously processed twice
            if i < height - 2 || height % 3 == 1 {
                xy_sum += (x as i32 * y as i32) as i64;
                xz_sum += (x as i32 * z as i32) as i64;
            }
            x_finalcol += w as i64;
            x2_finalcol += (w as i32 * w as i32) as i64;
            // So the bottom-right elements don't get counted twice:
            if i < height - (if height % 3 == 1 { 2 } else { 3 }) {
                x_sum += (z as i32 + w as i32) as i64;
                x2_sum += (z as i32 * z as i32 + w as i32 * w as i32) as i64;
            }
            // Now just vertical correlations for the final column:
            xz_sum += (y as i32 * w as i32) as i64;
        }
    }
    // Calculate the simple sums and squared-sums
    let mut x_firstrow: i64 = 0;
    let mut x_firstcol: i64 = 0;
    let mut x2_firstrow: i64 = 0;
    let mut x2_firstcol: i64 = 0;
    for j in 0..width {
        x_firstrow += *diff.offset(j as isize) as i64;
        x2_firstrow += (*diff.offset(j as isize) as i32 * *diff.offset(j as isize) as i32) as i64;
    }
    for i in 0..height {
        x_firstcol += *diff.offset((i * stride) as isize) as i64;
        x2_firstcol += (*diff.offset((i * stride) as isize) as i32
            * *diff.offset((i * stride) as isize) as i32) as i64;
    }
    let mut xhor_sum: i64 = x_sum - x_finalcol;
    let mut xver_sum: i64 = x_sum - x_finalrow;
    let mut y_sum: i64 = x_sum - x_firstcol;
    let mut z_sum: i64 = x_sum - x_firstrow;
    let mut x2hor_sum: i64 = x2_sum - x2_finalcol;
    let mut x2ver_sum: i64 = x2_sum - x2_finalrow;
    let mut y2_sum: i64 = x2_sum - x2_firstcol;
    let mut z2_sum: i64 = x2_sum - x2_firstrow;
    let num_hor: f32 = (height * (width - 1)) as f32;
    let num_ver: f32 = ((height - 1) * width) as f32;
    let xhor_var_n: f32 = x2hor_sum as f32 - (xhor_sum * xhor_sum) as f32 / num_hor;
    let xver_var_n: f32 = x2ver_sum as f32 - (xver_sum * xver_sum) as f32 / num_ver;
    let y_var_n: f32 = y2_sum as f32 - (y_sum * y_sum) as f32 / num_hor;
    let z_var_n: f32 = z2_sum as f32 - (z_sum * z_sum) as f32 / num_ver;
    let xy_var_n: f32 = xy_sum as f32 - (xhor_sum * y_sum) as f32 / num_hor;
    let xz_var_n: f32 = xz_sum as f32 - (xver_sum * z_sum) as f32 / num_ver;
    if xhor_var_n > 0 as f32 && y_var_n > 0 as f32 {
        *hcorr = xy_var_n / sqrtf(xhor_var_n * y_var_n);
        *hcorr = if *hcorr < 0 as f32 { 0 as f32 } else { *hcorr };
    } else {
        *hcorr = 1.0;
    }
    if xver_var_n > 0 as f32 && z_var_n > 0 as f32 {
        *vcorr = xz_var_n / sqrtf(xver_var_n * z_var_n);
        *vcorr = if *vcorr < 0 as f32 { 0 as f32 } else { *vcorr };
    } else {
        *vcorr = 1.0;
    }
}
