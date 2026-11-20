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

// Translated from upstream/aom/av1/encoder/x86/av1_highbd_quantize_sse4.c (ADR 0056, ADR 0058).
// Upstream compiles it with -msse41. The functions were
// transliterated with a typing-aware C-subset printer (not C2Rust) and
// reviewed against the C function by function: the same sequence of intrinsic
// and helper calls, constants and operators in order (ADR 0058).
// ROUND_POWER_OF_TWO (aom_ports/mem.h) is written out through the config's `replace`.
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

use core::arch::x86_64::*;

// Coefficient quantization phase 1
// param[0-2] : rounding/quan/dequan constants
#[inline]
#[target_feature(enable = "sse4.1")]
unsafe fn quantize_coeff_phase1(
    coeff: *mut __m128i,
    param: *const __m128i,
    shift: i32,
    scale: i32,
    qcoeff: *mut __m128i,
    dquan: *mut __m128i,
    sign: *mut __m128i,
) {
    let zero: __m128i = _mm_setzero_si128();
    let one: __m128i = _mm_set1_epi32(1);
    *sign = _mm_cmplt_epi32(*coeff, zero);
    *sign = _mm_or_si128(*sign, one);
    *coeff = _mm_abs_epi32(*coeff);
    *qcoeff = _mm_add_epi32(*coeff, *param);
    *qcoeff.offset(1) = _mm_unpackhi_epi32(*qcoeff, zero);
    *qcoeff = _mm_unpacklo_epi32(*qcoeff, zero);
    *qcoeff = _mm_mul_epi32(*qcoeff, *param.offset(1));
    *qcoeff = _mm_srl_epi64(*qcoeff, _mm_cvtsi32_si128(shift));
    *dquan = _mm_mul_epi32(*qcoeff, *param.offset(2));
    *dquan = _mm_srl_epi64(*dquan, _mm_cvtsi32_si128(scale));
    let abs_s: __m128i = _mm_sll_epi32(*coeff, _mm_cvtsi32_si128(1 + scale));
    *qcoeff.offset(2) = _mm_cmplt_epi32(abs_s, *param.offset(3));
}

// Coefficient quantization phase 2
#[inline]
#[target_feature(enable = "sse4.1")]
unsafe fn quantize_coeff_phase2(
    qcoeff: *mut __m128i,
    dquan: *mut __m128i,
    sign: *const __m128i,
    param: *const __m128i,
    shift: i32,
    scale: i32,
    qAddr: *mut i32,
    dqAddr: *mut i32,
) {
    let mut mask0L: __m128i = _mm_set_epi32(-1, -1, 0, 0);
    let mut mask0H: __m128i = _mm_set_epi32(0, 0, -1, -1);
    *qcoeff.offset(1) = _mm_mul_epi32(*qcoeff.offset(1), *param.offset(1));
    *qcoeff.offset(1) = _mm_srl_epi64(*qcoeff.offset(1), _mm_cvtsi32_si128(shift));
    *dquan.offset(1) = _mm_mul_epi32(*qcoeff.offset(1), *param.offset(2));
    *dquan.offset(1) = _mm_srl_epi64(*dquan.offset(1), _mm_cvtsi32_si128(scale));
    // combine L&H
    *qcoeff = _mm_shuffle_epi32::<0xd8>(*qcoeff);
    *qcoeff.offset(1) = _mm_shuffle_epi32::<0x8d>(*qcoeff.offset(1));
    *qcoeff = _mm_and_si128(*qcoeff, mask0H);
    *qcoeff.offset(1) = _mm_and_si128(*qcoeff.offset(1), mask0L);
    *dquan = _mm_shuffle_epi32::<0xd8>(*dquan);
    *dquan.offset(1) = _mm_shuffle_epi32::<0x8d>(*dquan.offset(1));
    *dquan = _mm_and_si128(*dquan, mask0H);
    *dquan.offset(1) = _mm_and_si128(*dquan.offset(1), mask0L);
    *qcoeff = _mm_or_si128(*qcoeff, *qcoeff.offset(1));
    *dquan = _mm_or_si128(*dquan, *dquan.offset(1));
    *qcoeff = _mm_sign_epi32(*qcoeff, *sign);
    *dquan = _mm_sign_epi32(*dquan, *sign);
    *qcoeff = _mm_andnot_si128(*qcoeff.offset(2), *qcoeff);
    *dquan = _mm_andnot_si128(*qcoeff.offset(2), *dquan);
    _mm_storeu_si128(qAddr as *mut __m128i, *qcoeff);
    _mm_storeu_si128(dqAddr as *mut __m128i, *dquan);
}

#[inline]
#[target_feature(enable = "sse4.1")]
unsafe fn find_eob(qcoeff_ptr: *mut i32, iscan: *const i16, eob: *mut __m128i) {
    let zero: __m128i = _mm_setzero_si128();
    let mut mask: __m128i;
    let mut iscanIdx: __m128i;
    let q0: __m128i = _mm_loadu_si128(qcoeff_ptr as *const __m128i);
    let q1: __m128i = _mm_loadu_si128(qcoeff_ptr.offset(4) as *const __m128i);
    let mut nz_flag0: __m128i = _mm_cmpeq_epi32(q0, zero);
    let mut nz_flag1: __m128i = _mm_cmpeq_epi32(q1, zero);
    nz_flag0 = _mm_cmpeq_epi32(nz_flag0, zero);
    nz_flag1 = _mm_cmpeq_epi32(nz_flag1, zero);
    mask = _mm_packs_epi32(nz_flag0, nz_flag1);
    iscanIdx = _mm_loadu_si128(iscan as *const __m128i);
    iscanIdx = _mm_sub_epi16(iscanIdx, mask);
    iscanIdx = _mm_and_si128(iscanIdx, mask);
    *eob = _mm_max_epi16(*eob, iscanIdx);
}

#[inline]
#[target_feature(enable = "sse4.1")]
unsafe fn get_accumulated_eob(eob: *mut __m128i) -> u16 {
    let mut eob_shuffled: __m128i;
    let mut eobValue: u16 = 0;
    eob_shuffled = _mm_shuffle_epi32::<0xe>(*eob);
    *eob = _mm_max_epi16(*eob, eob_shuffled);
    eob_shuffled = _mm_shufflelo_epi16::<0xe>(*eob);
    *eob = _mm_max_epi16(*eob, eob_shuffled);
    eob_shuffled = _mm_shufflelo_epi16::<0x1>(*eob);
    *eob = _mm_max_epi16(*eob, eob_shuffled);
    eobValue = _mm_extract_epi16::<0>(*eob) as u16;
    return eobValue;
}

#[no_mangle]
#[target_feature(enable = "sse4.1")]
pub unsafe extern "C-unwind" fn av1_highbd_quantize_fp_sse4_1(
    coeff_ptr: *const i32,
    mut count: isize,
    zbin_ptr: *const i16,
    round_ptr: *const i16,
    quant_ptr: *const i16,
    quant_shift_ptr: *const i16,
    qcoeff_ptr: *mut i32,
    dqcoeff_ptr: *mut i32,
    dequant_ptr: *const i16,
    eob_ptr: *mut u16,
    scan: *const i16,
    mut iscan: *const i16,
    log_scale: i32,
) {
    let mut coeff: [__m128i; 2] = core::mem::zeroed();
    let mut qcoeff: [__m128i; 3] = core::mem::zeroed();
    let mut dequant: [__m128i; 2] = core::mem::zeroed();
    let mut qparam: [__m128i; 4] = core::mem::zeroed();
    let mut coeff_sign: __m128i = core::mem::zeroed();
    let mut eob: __m128i = _mm_setzero_si128();
    let mut src: *const i32 = coeff_ptr;
    let mut quanAddr: *mut i32 = qcoeff_ptr;
    let mut dquanAddr: *mut i32 = dqcoeff_ptr;
    let shift: i32 = 16 - log_scale;
    let coeff_stride: i32 = 4;
    let quan_stride: i32 = coeff_stride;
    let _ = zbin_ptr;
    let _ = quant_shift_ptr;
    let _ = scan;
    core::ptr::write_bytes(
        quanAddr as *mut u8,
        0,
        count as usize * core::mem::size_of::<i32>(),
    );
    core::ptr::write_bytes(
        dquanAddr as *mut u8,
        0,
        count as usize * core::mem::size_of::<i32>(),
    );
    coeff[0] = _mm_loadu_si128(src as *const __m128i);
    let round1: i32 = (*round_ptr.offset(1) as i32 + ((1 << log_scale) >> 1)) >> log_scale;
    let round0: i32 = (*round_ptr as i32 + ((1 << log_scale) >> 1)) >> log_scale;
    qparam[0] = _mm_set_epi32(round1, round1, round1, round0);
    qparam[1] = _mm_set_epi64x(*quant_ptr.offset(1) as u32 as i64, *quant_ptr as u32 as i64);
    qparam[2] = _mm_set_epi64x(
        *dequant_ptr.offset(1) as u32 as i64,
        *dequant_ptr as u32 as i64,
    );
    qparam[3] = _mm_set_epi32(
        *dequant_ptr.offset(1) as i32,
        *dequant_ptr.offset(1) as i32,
        *dequant_ptr.offset(1) as i32,
        *dequant_ptr as i32,
    );
    // DC and first 3 AC
    quantize_coeff_phase1(
        coeff.as_mut_ptr(),
        qparam.as_mut_ptr() as *const __m128i,
        shift,
        log_scale,
        qcoeff.as_mut_ptr(),
        dequant.as_mut_ptr(),
        &raw mut coeff_sign,
    );
    // update round/quan/dquan for AC
    qparam[0] = _mm_unpackhi_epi64(qparam[0], qparam[0]);
    qparam[1] = _mm_set1_epi64x(*quant_ptr.offset(1) as u32 as i64);
    qparam[2] = _mm_set1_epi64x(*dequant_ptr.offset(1) as u32 as i64);
    qparam[3] = _mm_set1_epi32(*dequant_ptr.offset(1) as i32);
    quantize_coeff_phase2(
        qcoeff.as_mut_ptr(),
        dequant.as_mut_ptr(),
        &raw mut coeff_sign as *const __m128i,
        qparam.as_mut_ptr() as *const __m128i,
        shift,
        log_scale,
        quanAddr,
        dquanAddr,
    );
    // next 4 AC
    coeff[1] = _mm_loadu_si128(src.offset(coeff_stride as isize) as *const __m128i);
    quantize_coeff_phase1(
        coeff.as_mut_ptr().offset(1),
        qparam.as_mut_ptr() as *const __m128i,
        shift,
        log_scale,
        qcoeff.as_mut_ptr(),
        dequant.as_mut_ptr(),
        &raw mut coeff_sign,
    );
    quantize_coeff_phase2(
        qcoeff.as_mut_ptr(),
        dequant.as_mut_ptr(),
        &raw mut coeff_sign as *const __m128i,
        qparam.as_mut_ptr() as *const __m128i,
        shift,
        log_scale,
        quanAddr.offset(quan_stride as isize),
        dquanAddr.offset(quan_stride as isize),
    );
    find_eob(quanAddr, iscan, &raw mut eob);
    count -= 8;
    // loop for the rest of AC
    while count > 0 {
        src = src.offset((coeff_stride << 1) as isize);
        quanAddr = quanAddr.offset((quan_stride << 1) as isize);
        dquanAddr = dquanAddr.offset((quan_stride << 1) as isize);
        iscan = iscan.offset((quan_stride << 1) as isize);
        coeff[0] = _mm_loadu_si128(src as *const __m128i);
        coeff[1] = _mm_loadu_si128(src.offset(coeff_stride as isize) as *const __m128i);
        quantize_coeff_phase1(
            coeff.as_mut_ptr(),
            qparam.as_mut_ptr() as *const __m128i,
            shift,
            log_scale,
            qcoeff.as_mut_ptr(),
            dequant.as_mut_ptr(),
            &raw mut coeff_sign,
        );
        quantize_coeff_phase2(
            qcoeff.as_mut_ptr(),
            dequant.as_mut_ptr(),
            &raw mut coeff_sign as *const __m128i,
            qparam.as_mut_ptr() as *const __m128i,
            shift,
            log_scale,
            quanAddr,
            dquanAddr,
        );
        quantize_coeff_phase1(
            coeff.as_mut_ptr().offset(1),
            qparam.as_mut_ptr() as *const __m128i,
            shift,
            log_scale,
            qcoeff.as_mut_ptr(),
            dequant.as_mut_ptr(),
            &raw mut coeff_sign,
        );
        quantize_coeff_phase2(
            qcoeff.as_mut_ptr(),
            dequant.as_mut_ptr(),
            &raw mut coeff_sign as *const __m128i,
            qparam.as_mut_ptr() as *const __m128i,
            shift,
            log_scale,
            quanAddr.offset(quan_stride as isize),
            dquanAddr.offset(quan_stride as isize),
        );
        find_eob(quanAddr, iscan, &raw mut eob);
        count -= 8;
    }
    *eob_ptr = get_accumulated_eob(&raw mut eob);
}
