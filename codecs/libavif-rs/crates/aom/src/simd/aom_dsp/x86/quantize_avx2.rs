/*
 * Copyright (c) 2022, Alliance for Open Media. All rights reserved.
 *
 * This source code is subject to the terms of the BSD 2 Clause License and
 * the Alliance for Open Media Patent License 1.0. If the BSD 2 Clause License
 * was not distributed with this source code in the LICENSE file, you can
 * obtain it at www.aomedia.org/license/software. If the Alliance for Open
 * Media Patent License 1.0 was not distributed with this source code in the
 * PATENTS file, you can obtain it at www.aomedia.org/license/patent.
 */

// Translated from upstream/aom/aom_dsp/x86/quantize_avx2.c (ADR 0056, ADR 0058).
// Upstream compiles it with -mavx2. The functions were
// transliterated with a typing-aware C-subset printer (not C2Rust) and
// reviewed against the C function by function: the same sequence of intrinsic
// and helper calls, constants and operators in order (ADR 0058).
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

use crate::simd::aom_dsp__x86__quantize_x86::*;
use crate::simd::aom_dsp__x86__synonyms::*;
use crate::simd::aom_dsp__x86__synonyms_avx2::*;
use core::arch::x86_64::*;

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn load_b_values_avx2(
    zbin_ptr: *const i16,
    zbin: *mut __m256i,
    round_ptr: *const i16,
    round: *mut __m256i,
    quant_ptr: *const i16,
    quant: *mut __m256i,
    dequant_ptr: *const i16,
    dequant: *mut __m256i,
    shift_ptr: *const i16,
    shift: *mut __m256i,
    log_scale: i32,
) {
    *zbin = _mm256_castsi128_si256(_mm_load_si128(zbin_ptr as *const __m128i));
    *zbin = _mm256_permute4x64_epi64::<0x54>(*zbin);
    if log_scale > 0 {
        let rnd: __m256i = _mm256_set1_epi16((1 << (log_scale - 1)) as i16);
        *zbin = _mm256_add_epi16(*zbin, rnd);
        *zbin = _mm256_sra_epi16(*zbin, _mm_cvtsi32_si128(log_scale));
    }
    // Subtracting 1 here eliminates a _mm256_cmpeq_epi16() instruction when
    // calculating the zbin mask. (See quantize_b_logscale{0,1,2}_16)
    *zbin = _mm256_sub_epi16(*zbin, _mm256_set1_epi16(1));
    *round = _mm256_castsi128_si256(_mm_load_si128(round_ptr as *const __m128i));
    *round = _mm256_permute4x64_epi64::<0x54>(*round);
    if log_scale > 0 {
        let rnd: __m256i = _mm256_set1_epi16((1 << (log_scale - 1)) as i16);
        *round = _mm256_add_epi16(*round, rnd);
        *round = _mm256_sra_epi16(*round, _mm_cvtsi32_si128(log_scale));
    }
    *quant = _mm256_castsi128_si256(_mm_load_si128(quant_ptr as *const __m128i));
    *quant = _mm256_permute4x64_epi64::<0x54>(*quant);
    *dequant = _mm256_castsi128_si256(_mm_load_si128(dequant_ptr as *const __m128i));
    *dequant = _mm256_permute4x64_epi64::<0x54>(*dequant);
    *shift = _mm256_castsi128_si256(_mm_load_si128(shift_ptr as *const __m128i));
    *shift = _mm256_permute4x64_epi64::<0x54>(*shift);
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn load_coefficients_avx2(coeff_ptr: *const i32) -> __m256i {
    let coeff1: __m256i = _mm256_load_si256(coeff_ptr as *mut __m256i as *const __m256i);
    let coeff2: __m256i = _mm256_load_si256(coeff_ptr.offset(8) as *mut __m256i as *const __m256i);
    return _mm256_packs_epi32(coeff1, coeff2);
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn store_coefficients_avx2(coeff_vals: __m256i, coeff_ptr: *mut i32) {
    let mut coeff_sign: __m256i = _mm256_srai_epi16::<{ 15 }>(coeff_vals);
    let mut coeff_vals_lo: __m256i = _mm256_unpacklo_epi16(coeff_vals, coeff_sign);
    let mut coeff_vals_hi: __m256i = _mm256_unpackhi_epi16(coeff_vals, coeff_sign);
    _mm256_store_si256(coeff_ptr as *mut __m256i, coeff_vals_lo);
    _mm256_store_si256(coeff_ptr.offset(8) as *mut __m256i, coeff_vals_hi);
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn quantize_b_logscale0_16(
    coeff_ptr: *const i32,
    qcoeff_ptr: *mut i32,
    dqcoeff_ptr: *mut i32,
    v_quant: *mut __m256i,
    v_dequant: *mut __m256i,
    v_round: *mut __m256i,
    v_zbin: *mut __m256i,
    v_quant_shift: *mut __m256i,
) -> __m256i {
    let v_coeff: __m256i = load_coefficients_avx2(coeff_ptr);
    let v_abs_coeff: __m256i = _mm256_abs_epi16(v_coeff);
    let v_zbin_mask: __m256i = _mm256_cmpgt_epi16(v_abs_coeff, *v_zbin);
    if _mm256_movemask_epi8(v_zbin_mask) == 0 {
        _mm256_store_si256(qcoeff_ptr as *mut __m256i, _mm256_setzero_si256());
        _mm256_store_si256(dqcoeff_ptr as *mut __m256i, _mm256_setzero_si256());
        _mm256_store_si256(qcoeff_ptr.offset(8) as *mut __m256i, _mm256_setzero_si256());
        _mm256_store_si256(
            dqcoeff_ptr.offset(8) as *mut __m256i,
            _mm256_setzero_si256(),
        );
        return _mm256_setzero_si256();
    }
    // tmp = v_zbin_mask ? (int64_t)abs_coeff + log_scaled_round : 0
    let v_tmp_rnd: __m256i =
        _mm256_and_si256(_mm256_adds_epi16(v_abs_coeff, *v_round), v_zbin_mask);
    //  tmp32 = (int)(((((tmp * quant_ptr[rc != 0]) >> 16) + tmp) *
    //                 quant_shift_ptr[rc != 0]) >>
    //                (16 - log_scale + AOM_QM_BITS));
    let v_tmp32_a: __m256i = _mm256_mulhi_epi16(v_tmp_rnd, *v_quant);
    let v_tmp32_b: __m256i = _mm256_add_epi16(v_tmp32_a, v_tmp_rnd);
    let v_tmp32: __m256i = _mm256_mulhi_epi16(v_tmp32_b, *v_quant_shift);
    let v_nz_mask: __m256i = _mm256_cmpgt_epi16(v_tmp32, _mm256_setzero_si256());
    let v_qcoeff: __m256i = _mm256_sign_epi16(v_tmp32, v_coeff);
    let v_dqcoeff: __m256i = _mm256_mullo_epi16(v_qcoeff, *v_dequant);
    store_coefficients_avx2(v_qcoeff, qcoeff_ptr);
    store_coefficients_avx2(v_dqcoeff, dqcoeff_ptr);
    return v_nz_mask;
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn get_max_lane_eob(iscan: *const i16, v_eobmax: __m256i, v_mask: __m256i) -> __m256i {
    let v_iscan: __m256i = _mm256_loadu_si256(iscan as *const __m256i);
    let v_iscan_perm: __m256i = _mm256_permute4x64_epi64::<0xD8>(v_iscan);
    let v_iscan_plus1: __m256i = _mm256_sub_epi16(v_iscan_perm, v_mask);
    let v_nz_iscan: __m256i = _mm256_and_si256(v_iscan_plus1, v_mask);
    return _mm256_max_epi16(v_eobmax, v_nz_iscan);
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn accumulate_eob256(eob256: __m256i) -> i16 {
    let eob_lo: __m128i = _mm256_castsi256_si128(eob256);
    let eob_hi: __m128i = _mm256_extractf128_si256::<1>(eob256);
    let mut eob: __m128i = _mm_max_epi16(eob_lo, eob_hi);
    let mut eob_shuffled: __m128i = _mm_shuffle_epi32::<0xe>(eob);
    eob = _mm_max_epi16(eob, eob_shuffled);
    eob_shuffled = _mm_shufflelo_epi16::<0xe>(eob);
    eob = _mm_max_epi16(eob, eob_shuffled);
    eob_shuffled = _mm_shufflelo_epi16::<0x1>(eob);
    eob = _mm_max_epi16(eob, eob_shuffled);
    return _mm_extract_epi16::<1>(eob) as i16;
}

#[no_mangle]
#[target_feature(enable = "avx2")]
pub unsafe extern "C-unwind" fn aom_quantize_b_avx2(
    mut coeff_ptr: *const i32,
    n_coeffs: isize,
    zbin_ptr: *const i16,
    round_ptr: *const i16,
    quant_ptr: *const i16,
    quant_shift_ptr: *const i16,
    mut qcoeff_ptr: *mut i32,
    mut dqcoeff_ptr: *mut i32,
    dequant_ptr: *const i16,
    eob_ptr: *mut u16,
    scan: *const i16,
    mut iscan: *const i16,
) {
    let _ = scan;
    let mut v_zbin: __m256i = core::mem::zeroed();
    let mut v_round: __m256i = core::mem::zeroed();
    let mut v_quant: __m256i = core::mem::zeroed();
    let mut v_dequant: __m256i = core::mem::zeroed();
    let mut v_quant_shift: __m256i = core::mem::zeroed();
    let mut v_eobmax: __m256i = _mm256_setzero_si256();
    load_b_values_avx2(
        zbin_ptr,
        &raw mut v_zbin,
        round_ptr,
        &raw mut v_round,
        quant_ptr,
        &raw mut v_quant,
        dequant_ptr,
        &raw mut v_dequant,
        quant_shift_ptr,
        &raw mut v_quant_shift,
        0,
    );
    // Do DC and first 15 AC.
    let mut v_nz_mask: __m256i = quantize_b_logscale0_16(
        coeff_ptr,
        qcoeff_ptr,
        dqcoeff_ptr,
        &raw mut v_quant,
        &raw mut v_dequant,
        &raw mut v_round,
        &raw mut v_zbin,
        &raw mut v_quant_shift,
    );
    v_eobmax = get_max_lane_eob(iscan, v_eobmax, v_nz_mask);
    v_round = _mm256_unpackhi_epi64(v_round, v_round);
    v_quant = _mm256_unpackhi_epi64(v_quant, v_quant);
    v_dequant = _mm256_unpackhi_epi64(v_dequant, v_dequant);
    v_quant_shift = _mm256_unpackhi_epi64(v_quant_shift, v_quant_shift);
    v_zbin = _mm256_unpackhi_epi64(v_zbin, v_zbin);
    {
        let mut count: isize = n_coeffs - 16;
        while count > 0 {
            coeff_ptr = coeff_ptr.offset(16);
            qcoeff_ptr = qcoeff_ptr.offset(16);
            dqcoeff_ptr = dqcoeff_ptr.offset(16);
            iscan = iscan.offset(16);
            v_nz_mask = quantize_b_logscale0_16(
                coeff_ptr,
                qcoeff_ptr,
                dqcoeff_ptr,
                &raw mut v_quant,
                &raw mut v_dequant,
                &raw mut v_round,
                &raw mut v_zbin,
                &raw mut v_quant_shift,
            );
            v_eobmax = get_max_lane_eob(iscan, v_eobmax, v_nz_mask);
            count -= 16;
        }
    }
    *eob_ptr = accumulate_eob256(v_eobmax) as u16;
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn quantize_b_logscale_16(
    coeff_ptr: *const i32,
    qcoeff_ptr: *mut i32,
    dqcoeff_ptr: *mut i32,
    v_quant: *mut __m256i,
    v_dequant: *mut __m256i,
    v_round: *mut __m256i,
    v_zbin: *mut __m256i,
    v_quant_shift: *mut __m256i,
    log_scale: i32,
) -> __m256i {
    let v_coeff: __m256i = load_coefficients_avx2(coeff_ptr);
    let v_abs_coeff: __m256i = _mm256_abs_epi16(v_coeff);
    let v_zbin_mask: __m256i = _mm256_cmpgt_epi16(v_abs_coeff, *v_zbin);
    if _mm256_movemask_epi8(v_zbin_mask) == 0 {
        _mm256_store_si256(qcoeff_ptr as *mut __m256i, _mm256_setzero_si256());
        _mm256_store_si256(dqcoeff_ptr as *mut __m256i, _mm256_setzero_si256());
        _mm256_store_si256(qcoeff_ptr.offset(8) as *mut __m256i, _mm256_setzero_si256());
        _mm256_store_si256(
            dqcoeff_ptr.offset(8) as *mut __m256i,
            _mm256_setzero_si256(),
        );
        return _mm256_setzero_si256();
    }
    // tmp = v_zbin_mask ? (int64_t)abs_coeff + log_scaled_round : 0
    let v_tmp_rnd: __m256i =
        _mm256_and_si256(_mm256_adds_epi16(v_abs_coeff, *v_round), v_zbin_mask);
    //  tmp32 = (int)(((((tmp * quant_ptr[rc != 0]) >> 16) + tmp) *
    //                 quant_shift_ptr[rc != 0]) >>
    //                (16 - log_scale + AOM_QM_BITS));
    let v_tmp32_a: __m256i = _mm256_mulhi_epi16(v_tmp_rnd, *v_quant);
    let v_tmp32_b: __m256i = _mm256_add_epi16(v_tmp32_a, v_tmp_rnd);
    let v_tmp32_hi: __m256i = _mm256_sll_epi16(
        _mm256_mulhi_epi16(v_tmp32_b, *v_quant_shift),
        _mm_cvtsi32_si128(log_scale),
    );
    let v_tmp32_lo: __m256i = _mm256_srl_epi16(
        _mm256_mullo_epi16(v_tmp32_b, *v_quant_shift),
        _mm_cvtsi32_si128(16 - log_scale),
    );
    let v_tmp32: __m256i = _mm256_or_si256(v_tmp32_hi, v_tmp32_lo);
    let v_dqcoeff_hi: __m256i = _mm256_sll_epi16(
        _mm256_mulhi_epi16(v_tmp32, *v_dequant),
        _mm_cvtsi32_si128(16 - log_scale),
    );
    let v_dqcoeff_lo: __m256i = _mm256_srl_epi16(
        _mm256_mullo_epi16(v_tmp32, *v_dequant),
        _mm_cvtsi32_si128(log_scale),
    );
    let v_dqcoeff: __m256i =
        _mm256_sign_epi16(_mm256_or_si256(v_dqcoeff_hi, v_dqcoeff_lo), v_coeff);
    let v_qcoeff: __m256i = _mm256_sign_epi16(v_tmp32, v_coeff);
    let v_nz_mask: __m256i = _mm256_cmpgt_epi16(v_tmp32, _mm256_setzero_si256());
    store_coefficients_avx2(v_qcoeff, qcoeff_ptr);
    store_coefficients_avx2(v_dqcoeff, dqcoeff_ptr);
    return v_nz_mask;
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn quantize_b_no_qmatrix_avx2(
    mut coeff_ptr: *const i32,
    n_coeffs: isize,
    zbin_ptr: *const i16,
    round_ptr: *const i16,
    quant_ptr: *const i16,
    quant_shift_ptr: *const i16,
    mut qcoeff_ptr: *mut i32,
    mut dqcoeff_ptr: *mut i32,
    dequant_ptr: *const i16,
    eob_ptr: *mut u16,
    mut iscan: *const i16,
    log_scale: i32,
) {
    let mut v_zbin: __m256i = core::mem::zeroed();
    let mut v_round: __m256i = core::mem::zeroed();
    let mut v_quant: __m256i = core::mem::zeroed();
    let mut v_dequant: __m256i = core::mem::zeroed();
    let mut v_quant_shift: __m256i = core::mem::zeroed();
    let mut v_eobmax: __m256i = _mm256_setzero_si256();
    load_b_values_avx2(
        zbin_ptr,
        &raw mut v_zbin,
        round_ptr,
        &raw mut v_round,
        quant_ptr,
        &raw mut v_quant,
        dequant_ptr,
        &raw mut v_dequant,
        quant_shift_ptr,
        &raw mut v_quant_shift,
        log_scale,
    );
    // Do DC and first 15 AC.
    let mut v_nz_mask: __m256i = quantize_b_logscale_16(
        coeff_ptr,
        qcoeff_ptr,
        dqcoeff_ptr,
        &raw mut v_quant,
        &raw mut v_dequant,
        &raw mut v_round,
        &raw mut v_zbin,
        &raw mut v_quant_shift,
        log_scale,
    );
    v_eobmax = get_max_lane_eob(iscan, v_eobmax, v_nz_mask);
    v_round = _mm256_unpackhi_epi64(v_round, v_round);
    v_quant = _mm256_unpackhi_epi64(v_quant, v_quant);
    v_dequant = _mm256_unpackhi_epi64(v_dequant, v_dequant);
    v_quant_shift = _mm256_unpackhi_epi64(v_quant_shift, v_quant_shift);
    v_zbin = _mm256_unpackhi_epi64(v_zbin, v_zbin);
    {
        let mut count: isize = n_coeffs - 16;
        while count > 0 {
            coeff_ptr = coeff_ptr.offset(16);
            qcoeff_ptr = qcoeff_ptr.offset(16);
            dqcoeff_ptr = dqcoeff_ptr.offset(16);
            iscan = iscan.offset(16);
            v_nz_mask = quantize_b_logscale_16(
                coeff_ptr,
                qcoeff_ptr,
                dqcoeff_ptr,
                &raw mut v_quant,
                &raw mut v_dequant,
                &raw mut v_round,
                &raw mut v_zbin,
                &raw mut v_quant_shift,
                log_scale,
            );
            v_eobmax = get_max_lane_eob(iscan, v_eobmax, v_nz_mask);
            count -= 16;
        }
    }
    *eob_ptr = accumulate_eob256(v_eobmax) as u16;
}

#[no_mangle]
#[target_feature(enable = "avx2")]
pub unsafe extern "C-unwind" fn aom_quantize_b_32x32_avx2(
    coeff_ptr: *const i32,
    n_coeffs: isize,
    zbin_ptr: *const i16,
    round_ptr: *const i16,
    quant_ptr: *const i16,
    quant_shift_ptr: *const i16,
    qcoeff_ptr: *mut i32,
    dqcoeff_ptr: *mut i32,
    dequant_ptr: *const i16,
    eob_ptr: *mut u16,
    scan: *const i16,
    iscan: *const i16,
) {
    let _ = scan;
    quantize_b_no_qmatrix_avx2(
        coeff_ptr,
        n_coeffs,
        zbin_ptr,
        round_ptr,
        quant_ptr,
        quant_shift_ptr,
        qcoeff_ptr,
        dqcoeff_ptr,
        dequant_ptr,
        eob_ptr,
        iscan,
        1,
    );
}

#[no_mangle]
#[target_feature(enable = "avx2")]
pub unsafe extern "C-unwind" fn aom_quantize_b_64x64_avx2(
    coeff_ptr: *const i32,
    n_coeffs: isize,
    zbin_ptr: *const i16,
    round_ptr: *const i16,
    quant_ptr: *const i16,
    quant_shift_ptr: *const i16,
    qcoeff_ptr: *mut i32,
    dqcoeff_ptr: *mut i32,
    dequant_ptr: *const i16,
    eob_ptr: *mut u16,
    scan: *const i16,
    iscan: *const i16,
) {
    let _ = scan;
    quantize_b_no_qmatrix_avx2(
        coeff_ptr,
        n_coeffs,
        zbin_ptr,
        round_ptr,
        quant_ptr,
        quant_shift_ptr,
        qcoeff_ptr,
        dqcoeff_ptr,
        dequant_ptr,
        eob_ptr,
        iscan,
        2,
    );
}
