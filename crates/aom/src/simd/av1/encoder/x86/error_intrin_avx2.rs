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

// Translated by hand from upstream/aom/av1/encoder/x86/error_intrin_avx2.c
// (ADR 0056, ADR 0058). Upstream compiles this file with -mavx2.
use core::arch::x86_64::*;

use crate::simd::types::*;

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn read_coeff(coeff: *const tran_low_t, offset: isize, c: *mut __m256i) {
    let addr = coeff.offset(offset);

    if core::mem::size_of::<tran_low_t>() == 4 {
        let x0 = _mm256_loadu_si256(addr as *const __m256i);
        let x1 = _mm256_loadu_si256((addr as *const __m256i).add(1));
        let y = _mm256_packs_epi32(x0, x1);
        *c = _mm256_permute4x64_epi64::<0xD8>(y);
    } else {
        *c = _mm256_loadu_si256(addr as *const __m256i);
    }
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn av1_block_error_block_size16_avx2(coeff: *const i16, dqcoeff: *const i16,
                                            sse_256: *mut __m256i) {
    let _coeff = _mm256_loadu_si256(coeff as *const __m256i);
    let _dqcoeff = _mm256_loadu_si256(dqcoeff as *const __m256i);
    // d0 d1 d2 d3 d4 d5 d6 d7 d8 d9 d10 d11 d12 d13 d14 d15
    let diff = _mm256_sub_epi16(_dqcoeff, _coeff);
    // r0 r1 r2 r3 r4 r5 r6 r7
    let error = _mm256_madd_epi16(diff, diff);
    // r0+r1 r2+r3 | r0+r1 r2+r3 | r4+r5 r6+r7 | r4+r5 r6+r7
    let error_hi = _mm256_hadd_epi32(error, error);
    // r0+r1 | r2+r3 | r4+r5 | r6+r7
    *sse_256 = _mm256_unpacklo_epi32(error_hi, _mm256_setzero_si256());
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn av1_block_error_block_size32_avx2(coeff: *const i16, dqcoeff: *const i16,
                                            sse_256: *mut __m256i) {
    let zero = _mm256_setzero_si256();
    let _coeff_0 = _mm256_loadu_si256(coeff as *const __m256i);
    let _dqcoeff_0 = _mm256_loadu_si256(dqcoeff as *const __m256i);
    let _coeff_1 = _mm256_loadu_si256(coeff.add(16) as *const __m256i);
    let _dqcoeff_1 = _mm256_loadu_si256(dqcoeff.add(16) as *const __m256i);

    // d0 d1 d2 d3 d4 d5 d6 d7 d8 d9 d10 d11 d12 d13 d14 d15
    let diff_0 = _mm256_sub_epi16(_dqcoeff_0, _coeff_0);
    let diff_1 = _mm256_sub_epi16(_dqcoeff_1, _coeff_1);

    // r0 r1 r2 r3 r4 r5 r6 r7
    let error_0 = _mm256_madd_epi16(diff_0, diff_0);
    let error_1 = _mm256_madd_epi16(diff_1, diff_1);
    let err_final_0 = _mm256_add_epi32(error_0, error_1);

    // For extreme input values, the accumulation needs to happen in 64 bit
    // precision to avoid any overflow.
    let exp0_error_lo = _mm256_unpacklo_epi32(err_final_0, zero);
    let exp0_error_hi = _mm256_unpackhi_epi32(err_final_0, zero);
    let sum_temp_0 = _mm256_add_epi64(exp0_error_hi, exp0_error_lo);
    *sse_256 = _mm256_add_epi64(*sse_256, sum_temp_0);
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn av1_block_error_block_size64_avx2(mut coeff: *const i16, mut dqcoeff: *const i16,
                                            sse_256: *mut __m256i, block_size: isize) {
    let zero = _mm256_setzero_si256();
    let mut i: isize = 0;
    while i < block_size {
        // Load 64 elements for coeff and dqcoeff.
        let _coeff_0 = _mm256_loadu_si256(coeff as *const __m256i);
        let _dqcoeff_0 = _mm256_loadu_si256(dqcoeff as *const __m256i);
        let _coeff_1 = _mm256_loadu_si256(coeff.add(16) as *const __m256i);
        let _dqcoeff_1 = _mm256_loadu_si256(dqcoeff.add(16) as *const __m256i);
        let _coeff_2 = _mm256_loadu_si256(coeff.add(32) as *const __m256i);
        let _dqcoeff_2 = _mm256_loadu_si256(dqcoeff.add(32) as *const __m256i);
        let _coeff_3 = _mm256_loadu_si256(coeff.add(48) as *const __m256i);
        let _dqcoeff_3 = _mm256_loadu_si256(dqcoeff.add(48) as *const __m256i);

        // d0 d1 d2 d3 d4 d5 d6 d7 d8 d9 d10 d11 d12 d13 d14 d15
        let diff_0 = _mm256_sub_epi16(_dqcoeff_0, _coeff_0);
        let diff_1 = _mm256_sub_epi16(_dqcoeff_1, _coeff_1);
        let diff_2 = _mm256_sub_epi16(_dqcoeff_2, _coeff_2);
        let diff_3 = _mm256_sub_epi16(_dqcoeff_3, _coeff_3);

        // r0 r1 r2 r3 r4 r5 r6 r7
        let error_0 = _mm256_madd_epi16(diff_0, diff_0);
        let error_1 = _mm256_madd_epi16(diff_1, diff_1);
        let error_2 = _mm256_madd_epi16(diff_2, diff_2);
        let error_3 = _mm256_madd_epi16(diff_3, diff_3);
        // r00 r01 r02 r03 r04 r05 r06 r07
        let err_final_0 = _mm256_add_epi32(error_0, error_1);
        // r10 r11 r12 r13 r14 r15 r16 r17
        let err_final_1 = _mm256_add_epi32(error_2, error_3);

        // For extreme input values, the accumulation needs to happen in 64 bit
        // precision to avoid any overflow. r00 r01 r04 r05
        let exp0_error_lo = _mm256_unpacklo_epi32(err_final_0, zero);
        // r02 r03 r06 r07
        let exp0_error_hi = _mm256_unpackhi_epi32(err_final_0, zero);
        // r10 r11 r14 r15
        let exp1_error_lo = _mm256_unpacklo_epi32(err_final_1, zero);
        // r12 r13 r16 r17
        let exp1_error_hi = _mm256_unpackhi_epi32(err_final_1, zero);

        let sum_temp_0 = _mm256_add_epi64(exp0_error_hi, exp0_error_lo);
        let sum_temp_1 = _mm256_add_epi64(exp1_error_hi, exp1_error_lo);
        let sse_256_temp = _mm256_add_epi64(sum_temp_1, sum_temp_0);
        *sse_256 = _mm256_add_epi64(*sse_256, sse_256_temp);
        coeff = coeff.add(64);
        dqcoeff = dqcoeff.add(64);
        i += 64;
    }
}

#[no_mangle]
#[target_feature(enable = "avx2")]
pub unsafe extern "C-unwind" fn av1_block_error_lp_avx2(coeff: *const i16, dqcoeff: *const i16,
                                                         block_size: isize) -> i64 {
    debug_assert!(block_size % 16 == 0);
    let mut sse_256 = _mm256_setzero_si256();
    let mut sse: i64 = 0;

    if block_size == 16 {
        av1_block_error_block_size16_avx2(coeff, dqcoeff, &mut sse_256);
    } else if block_size == 32 {
        av1_block_error_block_size32_avx2(coeff, dqcoeff, &mut sse_256);
    } else {
        av1_block_error_block_size64_avx2(coeff, dqcoeff, &mut sse_256, block_size);
    }

    // Save the higher 64 bit of each 128 bit lane.
    let sse_hi = _mm256_srli_si256::<8>(sse_256);
    // Add the higher 64 bit to the low 64 bit.
    sse_256 = _mm256_add_epi64(sse_256, sse_hi);
    // Accumulate the sse_256 register to get final sse
    let sse_128 = _mm_add_epi64(_mm256_castsi256_si128(sse_256),
                                _mm256_extractf128_si256::<1>(sse_256));

    // Store the results.
    _mm_storel_epi64(&mut sse as *mut i64 as *mut __m128i, sse_128);
    sse
}

#[no_mangle]
#[target_feature(enable = "avx2")]
pub unsafe extern "C-unwind" fn av1_block_error_avx2(coeff: *const tran_low_t,
                                                      dqcoeff: *const tran_low_t,
                                                      block_size: isize, ssz: *mut i64) -> i64 {
    let mut sse_reg: __m256i;
    let mut ssz_reg: __m256i;
    let mut coeff_reg = _mm256_setzero_si256();
    let mut dqcoeff_reg = _mm256_setzero_si256();
    let mut exp_dqcoeff_lo: __m256i;
    let mut exp_dqcoeff_hi: __m256i;
    let mut exp_coeff_lo: __m256i;
    let mut exp_coeff_hi: __m256i;
    let sse_reg_64hi: __m256i;
    let ssz_reg_64hi: __m256i;
    let sse_reg128: __m128i;
    let ssz_reg128: __m128i;
    let mut sse: i64 = 0;
    let mut i: isize;
    let zero_reg = _mm256_setzero_si256();

    // init sse and ssz registerd to zero
    sse_reg = _mm256_setzero_si256();
    ssz_reg = _mm256_setzero_si256();

    i = 0;
    while i < block_size {
        // load 32 bytes from coeff and dqcoeff
        read_coeff(coeff, i, &mut coeff_reg);
        read_coeff(dqcoeff, i, &mut dqcoeff_reg);
        // dqcoeff - coeff
        dqcoeff_reg = _mm256_sub_epi16(dqcoeff_reg, coeff_reg);
        // madd (dqcoeff - coeff)
        dqcoeff_reg = _mm256_madd_epi16(dqcoeff_reg, dqcoeff_reg);
        // madd coeff
        coeff_reg = _mm256_madd_epi16(coeff_reg, coeff_reg);
        // expand each double word of madd (dqcoeff - coeff) to quad word
        exp_dqcoeff_lo = _mm256_unpacklo_epi32(dqcoeff_reg, zero_reg);
        exp_dqcoeff_hi = _mm256_unpackhi_epi32(dqcoeff_reg, zero_reg);
        // expand each double word of madd (coeff) to quad word
        exp_coeff_lo = _mm256_unpacklo_epi32(coeff_reg, zero_reg);
        exp_coeff_hi = _mm256_unpackhi_epi32(coeff_reg, zero_reg);
        // add each quad word of madd (dqcoeff - coeff) and madd (coeff)
        sse_reg = _mm256_add_epi64(sse_reg, exp_dqcoeff_lo);
        ssz_reg = _mm256_add_epi64(ssz_reg, exp_coeff_lo);
        sse_reg = _mm256_add_epi64(sse_reg, exp_dqcoeff_hi);
        ssz_reg = _mm256_add_epi64(ssz_reg, exp_coeff_hi);
        i += 16;
    }
    // save the higher 64 bit of each 128 bit lane
    sse_reg_64hi = _mm256_srli_si256::<8>(sse_reg);
    ssz_reg_64hi = _mm256_srli_si256::<8>(ssz_reg);
    // add the higher 64 bit to the low 64 bit
    sse_reg = _mm256_add_epi64(sse_reg, sse_reg_64hi);
    ssz_reg = _mm256_add_epi64(ssz_reg, ssz_reg_64hi);

    // add each 64 bit from each of the 128 bit lane of the 256 bit
    sse_reg128 = _mm_add_epi64(_mm256_castsi256_si128(sse_reg),
                               _mm256_extractf128_si256::<1>(sse_reg));

    ssz_reg128 = _mm_add_epi64(_mm256_castsi256_si128(ssz_reg),
                               _mm256_extractf128_si256::<1>(ssz_reg));

    // store the results
    _mm_storel_epi64(&mut sse as *mut i64 as *mut __m128i, sse_reg128);

    _mm_storel_epi64(ssz as *mut __m128i, ssz_reg128);
    _mm256_zeroupper();
    sse
}
