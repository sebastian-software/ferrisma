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

// Translated from upstream/aom/aom_dsp/x86/quantize_sse2.c (ADR 0056, ADR 0058).
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

use crate::simd::aom_dsp__x86__quantize_x86::*;
use crate::simd::aom_dsp__x86__synonyms::*;
use core::arch::x86_64::*;

#[no_mangle]
#[target_feature(enable = "sse2")]
pub unsafe extern "C-unwind" fn aom_quantize_b_sse2(
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
    scan_ptr: *const i16,
    iscan_ptr: *const i16,
) {
    let zero: __m128i = _mm_setzero_si128();
    let mut index: i32 = 16;
    let mut zbin: __m128i = core::mem::zeroed();
    let mut round: __m128i = core::mem::zeroed();
    let mut quant: __m128i = core::mem::zeroed();
    let mut dequant: __m128i = core::mem::zeroed();
    let mut shift: __m128i = core::mem::zeroed();
    let mut coeff0: __m128i = core::mem::zeroed();
    let mut coeff1: __m128i = core::mem::zeroed();
    let mut coeff0_sign: __m128i;
    let mut coeff1_sign: __m128i;
    let mut qcoeff0: __m128i = core::mem::zeroed();
    let mut qcoeff1: __m128i = core::mem::zeroed();
    let mut cmp_mask0: __m128i;
    let mut cmp_mask1: __m128i;
    let mut eob: __m128i;
    let mut eob0: __m128i;
    let _ = scan_ptr;
    // Setup global values.
    load_b_values(
        zbin_ptr,
        &raw mut zbin,
        round_ptr,
        &raw mut round,
        quant_ptr,
        &raw mut quant,
        dequant_ptr,
        &raw mut dequant,
        quant_shift_ptr,
        &raw mut shift,
    );
    // Do DC and first 15 AC.
    coeff0 = load_coefficients(coeff_ptr);
    coeff1 = load_coefficients(coeff_ptr.offset(8));
    // Poor man's abs().
    coeff0_sign = _mm_srai_epi16::<{ 15 }>(coeff0);
    coeff1_sign = _mm_srai_epi16::<{ 15 }>(coeff1);
    qcoeff0 = invert_sign_sse2(coeff0, coeff0_sign);
    qcoeff1 = invert_sign_sse2(coeff1, coeff1_sign);
    cmp_mask0 = _mm_cmpgt_epi16(qcoeff0, zbin);
    zbin = _mm_unpackhi_epi64(zbin, zbin); // Switch DC to AC
    cmp_mask1 = _mm_cmpgt_epi16(qcoeff1, zbin);
    calculate_qcoeff(&raw mut qcoeff0, round, quant, shift);
    round = _mm_unpackhi_epi64(round, round);
    quant = _mm_unpackhi_epi64(quant, quant);
    shift = _mm_unpackhi_epi64(shift, shift);
    calculate_qcoeff(&raw mut qcoeff1, round, quant, shift);
    // Reinsert signs
    qcoeff0 = invert_sign_sse2(qcoeff0, coeff0_sign);
    qcoeff1 = invert_sign_sse2(qcoeff1, coeff1_sign);
    // Mask out zbin threshold coeffs
    qcoeff0 = _mm_and_si128(qcoeff0, cmp_mask0);
    qcoeff1 = _mm_and_si128(qcoeff1, cmp_mask1);
    store_coefficients(qcoeff0, qcoeff_ptr);
    store_coefficients(qcoeff1, qcoeff_ptr.offset(8));
    coeff0 = calculate_dqcoeff(qcoeff0, dequant);
    dequant = _mm_unpackhi_epi64(dequant, dequant);
    coeff1 = calculate_dqcoeff(qcoeff1, dequant);
    store_coefficients(coeff0, dqcoeff_ptr);
    store_coefficients(coeff1, dqcoeff_ptr.offset(8));
    eob = scan_for_eob(
        &raw mut coeff0,
        &raw mut coeff1,
        cmp_mask0,
        cmp_mask1,
        iscan_ptr,
        0,
        zero,
    );
    // AC only loop.
    while (index as isize) < n_coeffs {
        coeff0 = load_coefficients(coeff_ptr.offset(index as isize));
        coeff1 = load_coefficients(coeff_ptr.offset(index as isize).offset(8));
        coeff0_sign = _mm_srai_epi16::<{ 15 }>(coeff0);
        coeff1_sign = _mm_srai_epi16::<{ 15 }>(coeff1);
        qcoeff0 = invert_sign_sse2(coeff0, coeff0_sign);
        qcoeff1 = invert_sign_sse2(coeff1, coeff1_sign);
        cmp_mask0 = _mm_cmpgt_epi16(qcoeff0, zbin);
        cmp_mask1 = _mm_cmpgt_epi16(qcoeff1, zbin);
        calculate_qcoeff(&raw mut qcoeff0, round, quant, shift);
        calculate_qcoeff(&raw mut qcoeff1, round, quant, shift);
        qcoeff0 = invert_sign_sse2(qcoeff0, coeff0_sign);
        qcoeff1 = invert_sign_sse2(qcoeff1, coeff1_sign);
        qcoeff0 = _mm_and_si128(qcoeff0, cmp_mask0);
        qcoeff1 = _mm_and_si128(qcoeff1, cmp_mask1);
        store_coefficients(qcoeff0, qcoeff_ptr.offset(index as isize));
        store_coefficients(qcoeff1, qcoeff_ptr.offset(index as isize).offset(8));
        coeff0 = calculate_dqcoeff(qcoeff0, dequant);
        coeff1 = calculate_dqcoeff(qcoeff1, dequant);
        store_coefficients(coeff0, dqcoeff_ptr.offset(index as isize));
        store_coefficients(coeff1, dqcoeff_ptr.offset(index as isize).offset(8));
        eob0 = scan_for_eob(
            &raw mut coeff0,
            &raw mut coeff1,
            cmp_mask0,
            cmp_mask1,
            iscan_ptr,
            index,
            zero,
        );
        eob = _mm_max_epi16(eob, eob0);
        index += 16;
    }
    *eob_ptr = accumulate_eob(eob) as u16;
}
