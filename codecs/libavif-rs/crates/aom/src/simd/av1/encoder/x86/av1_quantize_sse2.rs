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

// Translated from upstream/aom/av1/encoder/x86/av1_quantize_sse2.c (ADR 0056, ADR 0058).
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

#[inline]
#[target_feature(enable = "sse2")]
unsafe fn read_coeff(coeff: *const i32, offset: isize, c0: *mut __m128i, c1: *mut __m128i) {
    let mut addr: *const i32 = coeff.offset(offset);
    if core::mem::size_of::<i32>() == 4 {
        let x0: __m128i = _mm_load_si128(addr as *const __m128i);
        let x1: __m128i = _mm_load_si128((addr as *const __m128i).offset(1));
        let x2: __m128i = _mm_load_si128((addr as *const __m128i).offset(2));
        let x3: __m128i = _mm_load_si128((addr as *const __m128i).offset(3));
        *c0 = _mm_packs_epi32(x0, x1);
        *c1 = _mm_packs_epi32(x2, x3);
    } else {
        *c0 = _mm_load_si128(addr as *const __m128i);
        *c1 = _mm_load_si128((addr as *const __m128i).offset(1));
    }
}

#[inline]
#[target_feature(enable = "sse2")]
unsafe fn write_qcoeff(qc0: *const __m128i, qc1: *const __m128i, qcoeff: *mut i32, offset: isize) {
    let mut addr: *mut i32 = qcoeff.offset(offset);
    if core::mem::size_of::<i32>() == 4 {
        let zero: __m128i = _mm_setzero_si128();
        let mut sign_bits: __m128i = _mm_cmplt_epi16(*qc0, zero);
        let mut y0: __m128i = _mm_unpacklo_epi16(*qc0, sign_bits);
        let mut y1: __m128i = _mm_unpackhi_epi16(*qc0, sign_bits);
        _mm_store_si128(addr as *mut __m128i, y0);
        _mm_store_si128((addr as *mut __m128i).offset(1), y1);
        sign_bits = _mm_cmplt_epi16(*qc1, zero);
        y0 = _mm_unpacklo_epi16(*qc1, sign_bits);
        y1 = _mm_unpackhi_epi16(*qc1, sign_bits);
        _mm_store_si128((addr as *mut __m128i).offset(2), y0);
        _mm_store_si128((addr as *mut __m128i).offset(3), y1);
    } else {
        _mm_store_si128(addr as *mut __m128i, *qc0);
        _mm_store_si128((addr as *mut __m128i).offset(1), *qc1);
    }
}

#[inline]
#[target_feature(enable = "sse2")]
unsafe fn write_zero(qcoeff: *mut i32, offset: isize) {
    let zero: __m128i = _mm_setzero_si128();
    let mut addr: *mut i32 = qcoeff.offset(offset);
    if core::mem::size_of::<i32>() == 4 {
        _mm_store_si128(addr as *mut __m128i, zero);
        _mm_store_si128((addr as *mut __m128i).offset(1), zero);
        _mm_store_si128((addr as *mut __m128i).offset(2), zero);
        _mm_store_si128((addr as *mut __m128i).offset(3), zero);
    } else {
        _mm_store_si128(addr as *mut __m128i, zero);
        _mm_store_si128((addr as *mut __m128i).offset(1), zero);
    }
}

#[inline]
#[target_feature(enable = "sse2")]
unsafe fn quantize(
    iscan_ptr: *const i16,
    coeff_ptr: *const i32,
    n_coeffs: isize,
    qcoeff_ptr: *mut i32,
    dqcoeff_ptr: *mut i32,
    round0: *const __m128i,
    round1: *const __m128i,
    quant0: *const __m128i,
    quant1: *const __m128i,
    dequant0: *const __m128i,
    dequant1: *const __m128i,
    thr0: *const __m128i,
    thr1: *const __m128i,
    eob: *mut __m128i,
) {
    let mut coeff0: __m128i = core::mem::zeroed();
    let mut coeff1: __m128i = core::mem::zeroed();
    // Do DC and first 15 AC
    read_coeff(coeff_ptr, n_coeffs, &raw mut coeff0, &raw mut coeff1);
    // Poor man's sign extract
    let coeff0_sign: __m128i = _mm_srai_epi16::<{ 15 }>(coeff0);
    let coeff1_sign: __m128i = _mm_srai_epi16::<{ 15 }>(coeff1);
    let mut qcoeff0: __m128i = _mm_xor_si128(coeff0, coeff0_sign);
    let mut qcoeff1: __m128i = _mm_xor_si128(coeff1, coeff1_sign);
    qcoeff0 = _mm_sub_epi16(qcoeff0, coeff0_sign);
    qcoeff1 = _mm_sub_epi16(qcoeff1, coeff1_sign);
    let mask0: __m128i = _mm_or_si128(
        _mm_cmpgt_epi16(qcoeff0, *thr0),
        _mm_cmpeq_epi16(qcoeff0, *thr0),
    );
    let mask1: __m128i = _mm_or_si128(
        _mm_cmpgt_epi16(qcoeff1, *thr1),
        _mm_cmpeq_epi16(qcoeff1, *thr1),
    );
    let nzflag: i32 = _mm_movemask_epi8(mask0) | _mm_movemask_epi8(mask1);
    if nzflag != 0 {
        qcoeff0 = _mm_adds_epi16(qcoeff0, *round0);
        qcoeff1 = _mm_adds_epi16(qcoeff1, *round1);
        let qtmp0: __m128i = _mm_mulhi_epi16(qcoeff0, *quant0);
        let qtmp1: __m128i = _mm_mulhi_epi16(qcoeff1, *quant1);
        // Reinsert signs
        qcoeff0 = _mm_xor_si128(qtmp0, coeff0_sign);
        qcoeff1 = _mm_xor_si128(qtmp1, coeff1_sign);
        qcoeff0 = _mm_sub_epi16(qcoeff0, coeff0_sign);
        qcoeff1 = _mm_sub_epi16(qcoeff1, coeff1_sign);
        write_qcoeff(
            &raw mut qcoeff0 as *const __m128i,
            &raw mut qcoeff1 as *const __m128i,
            qcoeff_ptr,
            n_coeffs,
        );
        coeff0 = _mm_mullo_epi16(qcoeff0, *dequant0);
        coeff1 = _mm_mullo_epi16(qcoeff1, *dequant1);
        write_qcoeff(
            &raw mut coeff0 as *const __m128i,
            &raw mut coeff1 as *const __m128i,
            dqcoeff_ptr,
            n_coeffs,
        );
        let zero: __m128i = _mm_setzero_si128();
        // Scan for eob
        let zero_coeff0: __m128i = _mm_cmpeq_epi16(coeff0, zero);
        let zero_coeff1: __m128i = _mm_cmpeq_epi16(coeff1, zero);
        let nzero_coeff0: __m128i = _mm_cmpeq_epi16(zero_coeff0, zero);
        let nzero_coeff1: __m128i = _mm_cmpeq_epi16(zero_coeff1, zero);
        let iscan0: __m128i = _mm_load_si128(iscan_ptr.offset(n_coeffs) as *const __m128i);
        let iscan1: __m128i =
            _mm_load_si128((iscan_ptr.offset(n_coeffs) as *const __m128i).offset(1));
        // Add one to convert from indices to counts
        let iscan0_nz: __m128i = _mm_sub_epi16(iscan0, nzero_coeff0);
        let iscan1_nz: __m128i = _mm_sub_epi16(iscan1, nzero_coeff1);
        let eob0: __m128i = _mm_and_si128(iscan0_nz, nzero_coeff0);
        let eob1: __m128i = _mm_and_si128(iscan1_nz, nzero_coeff1);
        let eob2: __m128i = _mm_max_epi16(eob0, eob1);
        *eob = _mm_max_epi16(*eob, eob2);
    } else {
        write_zero(qcoeff_ptr, n_coeffs);
        write_zero(dqcoeff_ptr, n_coeffs);
    }
}

#[no_mangle]
#[target_feature(enable = "sse2")]
pub unsafe extern "C-unwind" fn av1_quantize_fp_sse2(
    mut coeff_ptr: *const i32,
    mut n_coeffs: isize,
    zbin_ptr: *const i16,
    round_ptr: *const i16,
    quant_ptr: *const i16,
    quant_shift_ptr: *const i16,
    mut qcoeff_ptr: *mut i32,
    mut dqcoeff_ptr: *mut i32,
    dequant_ptr: *const i16,
    eob_ptr: *mut u16,
    scan_ptr: *const i16,
    mut iscan_ptr: *const i16,
) {
    let _ = scan_ptr;
    let _ = zbin_ptr;
    let _ = quant_shift_ptr;
    coeff_ptr = coeff_ptr.offset(n_coeffs);
    iscan_ptr = iscan_ptr.offset(n_coeffs);
    qcoeff_ptr = qcoeff_ptr.offset(n_coeffs);
    dqcoeff_ptr = dqcoeff_ptr.offset(n_coeffs);
    n_coeffs = -n_coeffs;
    let round0: __m128i = _mm_load_si128(round_ptr as *const __m128i);
    let round1: __m128i = _mm_unpackhi_epi64(round0, round0);
    let quant0: __m128i = _mm_load_si128(quant_ptr as *const __m128i);
    let quant1: __m128i = _mm_unpackhi_epi64(quant0, quant0);
    let dequant0: __m128i = _mm_load_si128(dequant_ptr as *const __m128i);
    let dequant1: __m128i = _mm_unpackhi_epi64(dequant0, dequant0);
    let thr0: __m128i = _mm_srai_epi16::<{ 1 }>(dequant0);
    let thr1: __m128i = _mm_srai_epi16::<{ 1 }>(dequant1);
    let mut eob: __m128i = _mm_setzero_si128();
    quantize(
        iscan_ptr,
        coeff_ptr,
        n_coeffs,
        qcoeff_ptr,
        dqcoeff_ptr,
        &raw const round0,
        &raw const round1,
        &raw const quant0,
        &raw const quant1,
        &raw const dequant0,
        &raw const dequant1,
        &raw const thr0,
        &raw const thr1,
        &raw mut eob,
    );
    n_coeffs += (8 * 2) as isize;
    // AC only loop
    while n_coeffs < 0 {
        quantize(
            iscan_ptr,
            coeff_ptr,
            n_coeffs,
            qcoeff_ptr,
            dqcoeff_ptr,
            &raw const round1,
            &raw const round1,
            &raw const quant1,
            &raw const quant1,
            &raw const dequant1,
            &raw const dequant1,
            &raw const thr1,
            &raw const thr1,
            &raw mut eob,
        );
        n_coeffs += (8 * 2) as isize;
    }
    // Accumulate EOB
    {
        let mut eob_shuffled: __m128i;
        eob_shuffled = _mm_shuffle_epi32::<0xe>(eob);
        eob = _mm_max_epi16(eob, eob_shuffled);
        eob_shuffled = _mm_shufflelo_epi16::<0xe>(eob);
        eob = _mm_max_epi16(eob, eob_shuffled);
        eob_shuffled = _mm_shufflelo_epi16::<0x1>(eob);
        eob = _mm_max_epi16(eob, eob_shuffled);
        *eob_ptr = _mm_extract_epi16::<1>(eob) as u16;
    }
}

#[inline]
#[target_feature(enable = "sse2")]
unsafe fn quantize_lp(
    iscan_ptr: *const i16,
    coeff_ptr: *const i16,
    n_coeffs: isize,
    qcoeff_ptr: *mut i16,
    dqcoeff_ptr: *mut i16,
    round0: *const __m128i,
    round1: *const __m128i,
    quant0: *const __m128i,
    quant1: *const __m128i,
    dequant0: *const __m128i,
    dequant1: *const __m128i,
    eob: *mut __m128i,
) {
    let mut read: *const i16 = coeff_ptr.offset(n_coeffs);
    let mut coeff0: __m128i = _mm_load_si128(read as *const __m128i);
    let mut coeff1: __m128i = _mm_load_si128((read as *const __m128i).offset(1));
    // Poor man's sign extract
    let coeff0_sign: __m128i = _mm_srai_epi16::<{ 15 }>(coeff0);
    let coeff1_sign: __m128i = _mm_srai_epi16::<{ 15 }>(coeff1);
    let mut qcoeff0: __m128i = _mm_xor_si128(coeff0, coeff0_sign);
    let mut qcoeff1: __m128i = _mm_xor_si128(coeff1, coeff1_sign);
    qcoeff0 = _mm_sub_epi16(qcoeff0, coeff0_sign);
    qcoeff1 = _mm_sub_epi16(qcoeff1, coeff1_sign);
    qcoeff0 = _mm_adds_epi16(qcoeff0, *round0);
    qcoeff1 = _mm_adds_epi16(qcoeff1, *round1);
    let qtmp0: __m128i = _mm_mulhi_epi16(qcoeff0, *quant0);
    let qtmp1: __m128i = _mm_mulhi_epi16(qcoeff1, *quant1);
    // Reinsert signs
    qcoeff0 = _mm_xor_si128(qtmp0, coeff0_sign);
    qcoeff1 = _mm_xor_si128(qtmp1, coeff1_sign);
    qcoeff0 = _mm_sub_epi16(qcoeff0, coeff0_sign);
    qcoeff1 = _mm_sub_epi16(qcoeff1, coeff1_sign);
    let mut addr: *mut i16 = qcoeff_ptr.offset(n_coeffs);
    _mm_store_si128(addr as *mut __m128i, qcoeff0);
    _mm_store_si128((addr as *mut __m128i).offset(1), qcoeff1);
    coeff0 = _mm_mullo_epi16(qcoeff0, *dequant0);
    coeff1 = _mm_mullo_epi16(qcoeff1, *dequant1);
    addr = dqcoeff_ptr.offset(n_coeffs);
    _mm_store_si128(addr as *mut __m128i, coeff0);
    _mm_store_si128((addr as *mut __m128i).offset(1), coeff1);
    let zero: __m128i = _mm_setzero_si128();
    // Scan for eob
    let zero_coeff0: __m128i = _mm_cmpeq_epi16(coeff0, zero);
    let zero_coeff1: __m128i = _mm_cmpeq_epi16(coeff1, zero);
    let nzero_coeff0: __m128i = _mm_cmpeq_epi16(zero_coeff0, zero);
    let nzero_coeff1: __m128i = _mm_cmpeq_epi16(zero_coeff1, zero);
    let iscan0: __m128i = _mm_load_si128(iscan_ptr.offset(n_coeffs) as *const __m128i);
    let iscan1: __m128i = _mm_load_si128((iscan_ptr.offset(n_coeffs) as *const __m128i).offset(1));
    // Add one to convert from indices to counts
    let iscan0_nz: __m128i = _mm_sub_epi16(iscan0, nzero_coeff0);
    let iscan1_nz: __m128i = _mm_sub_epi16(iscan1, nzero_coeff1);
    let eob0: __m128i = _mm_and_si128(iscan0_nz, nzero_coeff0);
    let eob1: __m128i = _mm_and_si128(iscan1_nz, nzero_coeff1);
    let eob2: __m128i = _mm_max_epi16(eob0, eob1);
    *eob = _mm_max_epi16(*eob, eob2);
}

#[no_mangle]
#[target_feature(enable = "sse2")]
pub unsafe extern "C-unwind" fn av1_quantize_lp_sse2(
    mut coeff_ptr: *const i16,
    mut n_coeffs: isize,
    round_ptr: *const i16,
    quant_ptr: *const i16,
    mut qcoeff_ptr: *mut i16,
    mut dqcoeff_ptr: *mut i16,
    dequant_ptr: *const i16,
    eob_ptr: *mut u16,
    scan: *const i16,
    mut iscan: *const i16,
) {
    let _ = scan;
    coeff_ptr = coeff_ptr.offset(n_coeffs);
    iscan = iscan.offset(n_coeffs);
    qcoeff_ptr = qcoeff_ptr.offset(n_coeffs);
    dqcoeff_ptr = dqcoeff_ptr.offset(n_coeffs);
    n_coeffs = -n_coeffs;
    // Setup global values
    let round0: __m128i = _mm_load_si128(round_ptr as *const __m128i);
    let round1: __m128i = _mm_unpackhi_epi64(round0, round0);
    let quant0: __m128i = _mm_load_si128(quant_ptr as *const __m128i);
    let quant1: __m128i = _mm_unpackhi_epi64(quant0, quant0);
    let dequant0: __m128i = _mm_load_si128(dequant_ptr as *const __m128i);
    let dequant1: __m128i = _mm_unpackhi_epi64(dequant0, dequant0);
    let mut eob: __m128i = _mm_setzero_si128();
    // DC and first 15 AC
    quantize_lp(
        iscan,
        coeff_ptr,
        n_coeffs,
        qcoeff_ptr,
        dqcoeff_ptr,
        &raw const round0,
        &raw const round1,
        &raw const quant0,
        &raw const quant1,
        &raw const dequant0,
        &raw const dequant1,
        &raw mut eob,
    );
    n_coeffs += (8 * 2) as isize;
    // AC only loop
    while n_coeffs < 0 {
        quantize_lp(
            iscan,
            coeff_ptr,
            n_coeffs,
            qcoeff_ptr,
            dqcoeff_ptr,
            &raw const round1,
            &raw const round1,
            &raw const quant1,
            &raw const quant1,
            &raw const dequant1,
            &raw const dequant1,
            &raw mut eob,
        );
        n_coeffs += (8 * 2) as isize;
    }
    // Accumulate EOB
    *eob_ptr = accumulate_eob(eob) as u16;
}
