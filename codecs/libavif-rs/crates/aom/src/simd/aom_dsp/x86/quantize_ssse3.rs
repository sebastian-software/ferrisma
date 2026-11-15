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

// Translated from upstream/aom/aom_dsp/x86/quantize_ssse3.c (ADR 0056, ADR 0058).
// Upstream compiles it with -mssse3. The functions were
// transliterated with a typing-aware C-subset printer (not C2Rust) and
// reviewed against the C function by function: the same sequence of intrinsic
// and helper calls, constants and operators in order (ADR 0058).
//
// Translation notes:
// - Every function carries `ssse3`, the flag upstream compiles the file with.
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

use crate::aom_dsp__quantize::aom_quantize_b_helper_c;
use crate::simd::aom_dsp__x86__quantize_x86::*;
use crate::simd::aom_dsp__x86__synonyms::*;
use core::arch::x86_64::*;

#[inline]
#[target_feature(enable = "ssse3")]
unsafe fn calculate_qcoeff_64x64(
    coeff: *mut __m128i,
    round: __m128i,
    quant: __m128i,
    shift: *const __m128i,
) {
    let mut tmp: __m128i;
    let mut qcoeff: __m128i;
    let mut tmp1: __m128i;
    qcoeff = _mm_adds_epi16(*coeff, round);
    tmp = _mm_mulhi_epi16(qcoeff, quant);
    qcoeff = _mm_add_epi16(tmp, qcoeff);
    tmp = _mm_mullo_epi16(qcoeff, *shift);
    tmp = _mm_srli_epi16::<{ 14 }>(tmp);
    tmp1 = _mm_mulhi_epi16(qcoeff, *shift);
    tmp1 = _mm_slli_epi16::<{ 2 }>(tmp1);
    *coeff = _mm_or_si128(tmp, tmp1);
}

#[inline]
#[target_feature(enable = "ssse3")]
unsafe fn calculate_dqcoeff_and_store_64x64(
    qcoeff: __m128i,
    dequant: __m128i,
    zero: __m128i,
    dqcoeff: *mut i32,
) {
    // Un-sign to bias rounding like C.
    let coeff: __m128i = _mm_abs_epi16(qcoeff);
    let sign_0: __m128i = _mm_unpacklo_epi16(zero, qcoeff);
    let sign_1: __m128i = _mm_unpackhi_epi16(zero, qcoeff);
    let low: __m128i = _mm_mullo_epi16(coeff, dequant);
    let high: __m128i = _mm_mulhi_epi16(coeff, dequant);
    let mut dqcoeff32_0: __m128i = _mm_unpacklo_epi16(low, high);
    let mut dqcoeff32_1: __m128i = _mm_unpackhi_epi16(low, high);
    // "Divide" by 4.
    dqcoeff32_0 = _mm_srli_epi32::<{ 2 }>(dqcoeff32_0);
    dqcoeff32_1 = _mm_srli_epi32::<{ 2 }>(dqcoeff32_1);
    dqcoeff32_0 = _mm_sign_epi32(dqcoeff32_0, sign_0);
    dqcoeff32_1 = _mm_sign_epi32(dqcoeff32_1, sign_1);
    _mm_store_si128(dqcoeff as *mut __m128i, dqcoeff32_0);
    _mm_store_si128(dqcoeff.offset(4) as *mut __m128i, dqcoeff32_1);
}

#[no_mangle]
#[target_feature(enable = "ssse3")]
pub unsafe extern "C-unwind" fn aom_quantize_b_64x64_ssse3(
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
    let zero: __m128i = _mm_setzero_si128();
    let one: __m128i = _mm_set1_epi16(1);
    let two: __m128i = _mm_set1_epi16(2);
    let mut index: i32 = 0;
    let mut zbin: __m128i;
    let mut round: __m128i;
    let mut quant: __m128i;
    let mut dequant: __m128i;
    let mut shift: __m128i = core::mem::zeroed();
    let mut coeff0: __m128i;
    let mut coeff1: __m128i;
    let mut qcoeff0: __m128i = core::mem::zeroed();
    let mut qcoeff1: __m128i = core::mem::zeroed();
    let mut cmp_mask0: __m128i;
    let mut cmp_mask1: __m128i;
    let mut all_zero: __m128i;
    let mut eob: __m128i = zero;
    let mut eob0: __m128i;
    let _ = scan;
    let _ = n_coeffs;
    // Setup global values.
    zbin = _mm_load_si128(zbin_ptr as *const __m128i);
    round = _mm_load_si128(round_ptr as *const __m128i);
    quant = _mm_load_si128(quant_ptr as *const __m128i);
    dequant = _mm_load_si128(dequant_ptr as *const __m128i);
    shift = _mm_load_si128(quant_shift_ptr as *const __m128i);
    // Shift with rounding.
    zbin = _mm_add_epi16(zbin, two);
    round = _mm_add_epi16(round, two);
    zbin = _mm_srli_epi16::<{ 2 }>(zbin);
    round = _mm_srli_epi16::<{ 2 }>(round);
    zbin = _mm_sub_epi16(zbin, one);
    // Do DC and first 15 AC.
    coeff0 = load_coefficients(coeff_ptr);
    coeff1 = load_coefficients(coeff_ptr.offset(8));
    qcoeff0 = _mm_abs_epi16(coeff0);
    qcoeff1 = _mm_abs_epi16(coeff1);
    cmp_mask0 = _mm_cmpgt_epi16(qcoeff0, zbin);
    zbin = _mm_unpackhi_epi64(zbin, zbin);
    cmp_mask1 = _mm_cmpgt_epi16(qcoeff1, zbin);
    all_zero = _mm_or_si128(cmp_mask0, cmp_mask1);
    if _mm_movemask_epi8(all_zero) == 0 {
        _mm_store_si128(qcoeff_ptr as *mut __m128i, zero);
        _mm_store_si128(qcoeff_ptr.offset(4) as *mut __m128i, zero);
        _mm_store_si128(qcoeff_ptr.offset(8) as *mut __m128i, zero);
        _mm_store_si128(qcoeff_ptr.offset(12) as *mut __m128i, zero);
        _mm_store_si128(dqcoeff_ptr as *mut __m128i, zero);
        _mm_store_si128(dqcoeff_ptr.offset(4) as *mut __m128i, zero);
        _mm_store_si128(dqcoeff_ptr.offset(8) as *mut __m128i, zero);
        _mm_store_si128(dqcoeff_ptr.offset(12) as *mut __m128i, zero);
        round = _mm_unpackhi_epi64(round, round);
        quant = _mm_unpackhi_epi64(quant, quant);
        shift = _mm_unpackhi_epi64(shift, shift);
        dequant = _mm_unpackhi_epi64(dequant, dequant);
    } else {
        calculate_qcoeff_64x64(
            &raw mut qcoeff0,
            round,
            quant,
            &raw mut shift as *const __m128i,
        );
        round = _mm_unpackhi_epi64(round, round);
        quant = _mm_unpackhi_epi64(quant, quant);
        shift = _mm_unpackhi_epi64(shift, shift);
        calculate_qcoeff_64x64(
            &raw mut qcoeff1,
            round,
            quant,
            &raw mut shift as *const __m128i,
        );
        // Reinsert signs.
        qcoeff0 = _mm_sign_epi16(qcoeff0, coeff0);
        qcoeff1 = _mm_sign_epi16(qcoeff1, coeff1);
        // Mask out zbin threshold coeffs.
        qcoeff0 = _mm_and_si128(qcoeff0, cmp_mask0);
        qcoeff1 = _mm_and_si128(qcoeff1, cmp_mask1);
        store_coefficients(qcoeff0, qcoeff_ptr);
        store_coefficients(qcoeff1, qcoeff_ptr.offset(8));
        calculate_dqcoeff_and_store_64x64(qcoeff0, dequant, zero, dqcoeff_ptr);
        dequant = _mm_unpackhi_epi64(dequant, dequant);
        calculate_dqcoeff_and_store_64x64(qcoeff1, dequant, zero, dqcoeff_ptr.offset(8));
        eob = scan_for_eob(
            &raw mut qcoeff0,
            &raw mut qcoeff1,
            cmp_mask0,
            cmp_mask1,
            iscan,
            0,
            zero,
        );
    }
    // AC only loop.
    {
        index = 16;
        while index < 1024 {
            coeff0 = load_coefficients(coeff_ptr.offset(index as isize));
            coeff1 = load_coefficients(coeff_ptr.offset(index as isize).offset(8));
            qcoeff0 = _mm_abs_epi16(coeff0);
            qcoeff1 = _mm_abs_epi16(coeff1);
            cmp_mask0 = _mm_cmpgt_epi16(qcoeff0, zbin);
            cmp_mask1 = _mm_cmpgt_epi16(qcoeff1, zbin);
            all_zero = _mm_or_si128(cmp_mask0, cmp_mask1);
            if _mm_movemask_epi8(all_zero) == 0 {
                _mm_store_si128(qcoeff_ptr.offset(index as isize) as *mut __m128i, zero);
                _mm_store_si128(
                    qcoeff_ptr.offset(index as isize).offset(4) as *mut __m128i,
                    zero,
                );
                _mm_store_si128(
                    qcoeff_ptr.offset(index as isize).offset(8) as *mut __m128i,
                    zero,
                );
                _mm_store_si128(
                    qcoeff_ptr.offset(index as isize).offset(12) as *mut __m128i,
                    zero,
                );
                _mm_store_si128(dqcoeff_ptr.offset(index as isize) as *mut __m128i, zero);
                _mm_store_si128(
                    dqcoeff_ptr.offset(index as isize).offset(4) as *mut __m128i,
                    zero,
                );
                _mm_store_si128(
                    dqcoeff_ptr.offset(index as isize).offset(8) as *mut __m128i,
                    zero,
                );
                _mm_store_si128(
                    dqcoeff_ptr.offset(index as isize).offset(12) as *mut __m128i,
                    zero,
                );
                index += 16;
                continue;
            }
            calculate_qcoeff_64x64(
                &raw mut qcoeff0,
                round,
                quant,
                &raw mut shift as *const __m128i,
            );
            calculate_qcoeff_64x64(
                &raw mut qcoeff1,
                round,
                quant,
                &raw mut shift as *const __m128i,
            );
            qcoeff0 = _mm_sign_epi16(qcoeff0, coeff0);
            qcoeff1 = _mm_sign_epi16(qcoeff1, coeff1);
            qcoeff0 = _mm_and_si128(qcoeff0, cmp_mask0);
            qcoeff1 = _mm_and_si128(qcoeff1, cmp_mask1);
            store_coefficients(qcoeff0, qcoeff_ptr.offset(index as isize));
            store_coefficients(qcoeff1, qcoeff_ptr.offset(index as isize).offset(8));
            calculate_dqcoeff_and_store_64x64(
                qcoeff0,
                dequant,
                zero,
                dqcoeff_ptr.offset(index as isize),
            );
            calculate_dqcoeff_and_store_64x64(
                qcoeff1,
                dequant,
                zero,
                dqcoeff_ptr.offset(8).offset(index as isize),
            );
            eob0 = scan_for_eob(
                &raw mut qcoeff0,
                &raw mut qcoeff1,
                cmp_mask0,
                cmp_mask1,
                iscan,
                index,
                zero,
            );
            eob = _mm_max_epi16(eob, eob0);
            index += 16;
        }
    }
    *eob_ptr = accumulate_eob(eob) as u16;
}
