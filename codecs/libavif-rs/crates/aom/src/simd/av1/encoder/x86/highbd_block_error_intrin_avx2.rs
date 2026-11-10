/*
 * Copyright (c) 2019, Alliance for Open Media. All rights reserved.
 *
 * This source code is subject to the terms of the BSD 2 Clause License and
 * the Alliance for Open Media Patent License 1.0. If the BSD 2 Clause License
 * was not distributed with this source code in the LICENSE file, you can
 * obtain it at www.aomedia.org/license/software. If the Alliance for Open
 * Media Patent License 1.0 was not distributed with this source code in the
 * PATENTS file, you can obtain it at www.aomedia.org/license/patent.
 */

// Translated by hand from
// upstream/aom/av1/encoder/x86/highbd_block_error_intrin_avx2.c (ADR 0056,
// ADR 0058). Upstream compiles this file with -mavx2.
use core::arch::x86_64::*;

use crate::simd::types::*;

#[no_mangle]
#[target_feature(enable = "avx2")]
pub unsafe extern "C-unwind" fn av1_highbd_block_error_avx2(coeff: *const tran_low_t,
                                                             dqcoeff: *const tran_low_t,
                                                             block_size: isize, ssz: *mut i64,
                                                             bps: i32) -> i64 {
    let mut i: isize;
    let mut temp1 = [0i64; 8];
    let mut error: i64 = 0;
    let mut sqcoeff: i64 = 0;
    let shift = 2 * (bps - 8);
    let rounding = (1i32 << shift) >> 1;

    i = 0;
    while i < block_size {
        let mm256_coeff = _mm256_loadu_si256(coeff.offset(i) as *const __m256i);
        let mm256_coeff2 = _mm256_loadu_si256(coeff.offset(i + 8) as *const __m256i);
        let mm256_dqcoeff = _mm256_loadu_si256(dqcoeff.offset(i) as *const __m256i);
        let mm256_dqcoeff2 = _mm256_loadu_si256(dqcoeff.offset(i + 8) as *const __m256i);

        let diff1 = _mm256_sub_epi32(mm256_coeff, mm256_dqcoeff);
        let diff2 = _mm256_sub_epi32(mm256_coeff2, mm256_dqcoeff2);
        let diff1h = _mm256_srli_epi64::<32>(diff1);
        let diff2h = _mm256_srli_epi64::<32>(diff2);
        let mut res = _mm256_mul_epi32(diff1, diff1);
        let mut res1 = _mm256_mul_epi32(diff1h, diff1h);
        let mut res2 = _mm256_mul_epi32(diff2, diff2);
        let mut res3 = _mm256_mul_epi32(diff2h, diff2h);
        let res_diff = _mm256_add_epi64(_mm256_add_epi64(res, res1),
                                        _mm256_add_epi64(res2, res3));
        let mm256_coeffh = _mm256_srli_epi64::<32>(mm256_coeff);
        let mm256_coeffh2 = _mm256_srli_epi64::<32>(mm256_coeff2);
        res = _mm256_mul_epi32(mm256_coeff, mm256_coeff);
        res1 = _mm256_mul_epi32(mm256_coeffh, mm256_coeffh);
        res2 = _mm256_mul_epi32(mm256_coeff2, mm256_coeff2);
        res3 = _mm256_mul_epi32(mm256_coeffh2, mm256_coeffh2);
        let res_sqcoeff = _mm256_add_epi64(_mm256_add_epi64(res, res1),
                                           _mm256_add_epi64(res2, res3));
        _mm256_storeu_si256(temp1.as_mut_ptr() as *mut __m256i, res_diff);
        _mm256_storeu_si256((temp1.as_mut_ptr() as *mut __m256i).add(1), res_sqcoeff);

        error = error.wrapping_add(temp1[0].wrapping_add(temp1[1]).wrapping_add(temp1[2])
            .wrapping_add(temp1[3]));
        sqcoeff = sqcoeff.wrapping_add(temp1[4].wrapping_add(temp1[5]).wrapping_add(temp1[6])
            .wrapping_add(temp1[7]));
        i += 16;
    }
    error = (error + rounding as i64) >> shift;
    sqcoeff = (sqcoeff + rounding as i64) >> shift;

    *ssz = sqcoeff;
    error
}
