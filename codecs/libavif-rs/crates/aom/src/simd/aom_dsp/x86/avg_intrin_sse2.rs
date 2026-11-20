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

// Translated from upstream/aom/aom_dsp/x86/avg_intrin_sse2.c (ADR 0056, ADR 0058).
// Upstream compiles it with -msse2. The functions were
// transliterated with a typing-aware C-subset printer (not C2Rust) and
// reviewed against the C function by function: the same sequence of intrinsic
// and helper calls, constants and operators in order (ADR 0058).
// Only the Hadamard kernels of the file are translated (the config's `only`
// list; the averaging, SATD, projection and variance kernels after them are
// not on the still-image path).
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

use crate::simd::aom_dsp__x86__bitdepth_conversion_sse2::*;
use core::arch::x86_64::*;

#[repr(C, align(16))]
struct Aligned16<T>(T);
#[repr(C, align(32))]
struct Aligned32<T>(T);

#[inline]
#[target_feature(enable = "sse2")]
unsafe fn sign_extend_16bit_to_32bit_sse2(
    in_: __m128i,
    zero: __m128i,
    out_lo: *mut __m128i,
    out_hi: *mut __m128i,
) {
    let sign_bits: __m128i = _mm_cmplt_epi16(in_, zero);
    *out_lo = _mm_unpacklo_epi16(in_, sign_bits);
    *out_hi = _mm_unpackhi_epi16(in_, sign_bits);
}

#[inline]
#[target_feature(enable = "sse2")]
unsafe fn hadamard_col4_sse2(in_: *mut __m128i, iter: i32) {
    let a0: __m128i = *in_;
    let a1: __m128i = *in_.offset(1);
    let a2: __m128i = *in_.offset(2);
    let a3: __m128i = *in_.offset(3);
    let b0: __m128i = _mm_srai_epi16::<{ 1 }>(_mm_add_epi16(a0, a1));
    let b1: __m128i = _mm_srai_epi16::<{ 1 }>(_mm_sub_epi16(a0, a1));
    let b2: __m128i = _mm_srai_epi16::<{ 1 }>(_mm_add_epi16(a2, a3));
    let b3: __m128i = _mm_srai_epi16::<{ 1 }>(_mm_sub_epi16(a2, a3));
    *in_ = _mm_add_epi16(b0, b2);
    *in_.offset(1) = _mm_add_epi16(b1, b3);
    *in_.offset(2) = _mm_sub_epi16(b0, b2);
    *in_.offset(3) = _mm_sub_epi16(b1, b3);
    if iter == 0 {
        let ba: __m128i = _mm_unpacklo_epi16(*in_, *in_.offset(1));
        let dc: __m128i = _mm_unpacklo_epi16(*in_.offset(2), *in_.offset(3));
        let dcba_lo: __m128i = _mm_unpacklo_epi32(ba, dc);
        let dcba_hi: __m128i = _mm_unpackhi_epi32(ba, dc);
        *in_ = dcba_lo;
        *in_.offset(1) = _mm_srli_si128::<8>(dcba_lo);
        *in_.offset(2) = dcba_hi;
        *in_.offset(3) = _mm_srli_si128::<8>(dcba_hi);
    }
}

#[no_mangle]
#[target_feature(enable = "sse2")]
pub unsafe extern "C-unwind" fn aom_hadamard_4x4_sse2(
    mut src_diff: *const i16,
    src_stride: isize,
    mut coeff: *mut i32,
) {
    let mut src: [__m128i; 4] = core::mem::zeroed();
    src[0] = _mm_loadl_epi64(src_diff as *const __m128i);
    src[1] = _mm_loadl_epi64({
        src_diff = src_diff.offset(src_stride);
        src_diff
    } as *const __m128i);
    src[2] = _mm_loadl_epi64({
        src_diff = src_diff.offset(src_stride);
        src_diff
    } as *const __m128i);
    src[3] = _mm_loadl_epi64(src_diff.offset(src_stride) as *const __m128i);
    hadamard_col4_sse2(src.as_mut_ptr(), 0);
    hadamard_col4_sse2(src.as_mut_ptr(), 1);
    store_tran_low(_mm_unpacklo_epi64(src[0], src[1]), coeff);
    coeff = coeff.offset(8);
    store_tran_low(_mm_unpacklo_epi64(src[2], src[3]), coeff);
}

#[inline]
#[target_feature(enable = "sse2")]
unsafe fn hadamard_col8_sse2(in_: *mut __m128i, iter: i32) {
    let mut a0: __m128i = *in_;
    let mut a1: __m128i = *in_.offset(1);
    let mut a2: __m128i = *in_.offset(2);
    let mut a3: __m128i = *in_.offset(3);
    let mut a4: __m128i = *in_.offset(4);
    let mut a5: __m128i = *in_.offset(5);
    let mut a6: __m128i = *in_.offset(6);
    let mut a7: __m128i = *in_.offset(7);
    let mut b0: __m128i = _mm_add_epi16(a0, a1);
    let mut b1: __m128i = _mm_sub_epi16(a0, a1);
    let mut b2: __m128i = _mm_add_epi16(a2, a3);
    let mut b3: __m128i = _mm_sub_epi16(a2, a3);
    let mut b4: __m128i = _mm_add_epi16(a4, a5);
    let mut b5: __m128i = _mm_sub_epi16(a4, a5);
    let mut b6: __m128i = _mm_add_epi16(a6, a7);
    let mut b7: __m128i = _mm_sub_epi16(a6, a7);
    a0 = _mm_add_epi16(b0, b2);
    a1 = _mm_add_epi16(b1, b3);
    a2 = _mm_sub_epi16(b0, b2);
    a3 = _mm_sub_epi16(b1, b3);
    a4 = _mm_add_epi16(b4, b6);
    a5 = _mm_add_epi16(b5, b7);
    a6 = _mm_sub_epi16(b4, b6);
    a7 = _mm_sub_epi16(b5, b7);
    if iter == 0 {
        b0 = _mm_add_epi16(a0, a4);
        b7 = _mm_add_epi16(a1, a5);
        b3 = _mm_add_epi16(a2, a6);
        b4 = _mm_add_epi16(a3, a7);
        b2 = _mm_sub_epi16(a0, a4);
        b6 = _mm_sub_epi16(a1, a5);
        b1 = _mm_sub_epi16(a2, a6);
        b5 = _mm_sub_epi16(a3, a7);
        a0 = _mm_unpacklo_epi16(b0, b1);
        a1 = _mm_unpacklo_epi16(b2, b3);
        a2 = _mm_unpackhi_epi16(b0, b1);
        a3 = _mm_unpackhi_epi16(b2, b3);
        a4 = _mm_unpacklo_epi16(b4, b5);
        a5 = _mm_unpacklo_epi16(b6, b7);
        a6 = _mm_unpackhi_epi16(b4, b5);
        a7 = _mm_unpackhi_epi16(b6, b7);
        b0 = _mm_unpacklo_epi32(a0, a1);
        b1 = _mm_unpacklo_epi32(a4, a5);
        b2 = _mm_unpackhi_epi32(a0, a1);
        b3 = _mm_unpackhi_epi32(a4, a5);
        b4 = _mm_unpacklo_epi32(a2, a3);
        b5 = _mm_unpacklo_epi32(a6, a7);
        b6 = _mm_unpackhi_epi32(a2, a3);
        b7 = _mm_unpackhi_epi32(a6, a7);
        *in_ = _mm_unpacklo_epi64(b0, b1);
        *in_.offset(1) = _mm_unpackhi_epi64(b0, b1);
        *in_.offset(2) = _mm_unpacklo_epi64(b2, b3);
        *in_.offset(3) = _mm_unpackhi_epi64(b2, b3);
        *in_.offset(4) = _mm_unpacklo_epi64(b4, b5);
        *in_.offset(5) = _mm_unpackhi_epi64(b4, b5);
        *in_.offset(6) = _mm_unpacklo_epi64(b6, b7);
        *in_.offset(7) = _mm_unpackhi_epi64(b6, b7);
    } else {
        *in_ = _mm_add_epi16(a0, a4);
        *in_.offset(7) = _mm_add_epi16(a1, a5);
        *in_.offset(3) = _mm_add_epi16(a2, a6);
        *in_.offset(4) = _mm_add_epi16(a3, a7);
        *in_.offset(2) = _mm_sub_epi16(a0, a4);
        *in_.offset(6) = _mm_sub_epi16(a1, a5);
        *in_.offset(1) = _mm_sub_epi16(a2, a6);
        *in_.offset(5) = _mm_sub_epi16(a3, a7);
    }
}

#[inline]
#[target_feature(enable = "sse2")]
unsafe fn hadamard_8x8_sse2(
    mut src_diff: *const i16,
    src_stride: isize,
    mut coeff: *mut i32,
    is_final: i32,
) {
    let mut src: [__m128i; 8] = core::mem::zeroed();
    src[0] = _mm_load_si128(src_diff as *const __m128i);
    src[1] = _mm_load_si128({
        src_diff = src_diff.offset(src_stride);
        src_diff
    } as *const __m128i);
    src[2] = _mm_load_si128({
        src_diff = src_diff.offset(src_stride);
        src_diff
    } as *const __m128i);
    src[3] = _mm_load_si128({
        src_diff = src_diff.offset(src_stride);
        src_diff
    } as *const __m128i);
    src[4] = _mm_load_si128({
        src_diff = src_diff.offset(src_stride);
        src_diff
    } as *const __m128i);
    src[5] = _mm_load_si128({
        src_diff = src_diff.offset(src_stride);
        src_diff
    } as *const __m128i);
    src[6] = _mm_load_si128({
        src_diff = src_diff.offset(src_stride);
        src_diff
    } as *const __m128i);
    src[7] = _mm_load_si128(src_diff.offset(src_stride) as *const __m128i);
    hadamard_col8_sse2(src.as_mut_ptr(), 0);
    hadamard_col8_sse2(src.as_mut_ptr(), 1);
    if is_final != 0 {
        store_tran_low(src[0], coeff);
        coeff = coeff.offset(8);
        store_tran_low(src[1], coeff);
        coeff = coeff.offset(8);
        store_tran_low(src[2], coeff);
        coeff = coeff.offset(8);
        store_tran_low(src[3], coeff);
        coeff = coeff.offset(8);
        store_tran_low(src[4], coeff);
        coeff = coeff.offset(8);
        store_tran_low(src[5], coeff);
        coeff = coeff.offset(8);
        store_tran_low(src[6], coeff);
        coeff = coeff.offset(8);
        store_tran_low(src[7], coeff);
    } else {
        let mut coeff16: *mut i16 = coeff as *mut i16;
        _mm_store_si128(coeff16 as *mut __m128i, src[0]);
        coeff16 = coeff16.offset(8);
        _mm_store_si128(coeff16 as *mut __m128i, src[1]);
        coeff16 = coeff16.offset(8);
        _mm_store_si128(coeff16 as *mut __m128i, src[2]);
        coeff16 = coeff16.offset(8);
        _mm_store_si128(coeff16 as *mut __m128i, src[3]);
        coeff16 = coeff16.offset(8);
        _mm_store_si128(coeff16 as *mut __m128i, src[4]);
        coeff16 = coeff16.offset(8);
        _mm_store_si128(coeff16 as *mut __m128i, src[5]);
        coeff16 = coeff16.offset(8);
        _mm_store_si128(coeff16 as *mut __m128i, src[6]);
        coeff16 = coeff16.offset(8);
        _mm_store_si128(coeff16 as *mut __m128i, src[7]);
    }
}

#[no_mangle]
#[target_feature(enable = "sse2")]
pub unsafe extern "C-unwind" fn aom_hadamard_8x8_sse2(
    src_diff: *const i16,
    src_stride: isize,
    coeff: *mut i32,
) {
    hadamard_8x8_sse2(src_diff, src_stride, coeff, 1);
}

#[inline]
#[target_feature(enable = "sse2")]
unsafe fn hadamard_lp_8x8_sse2(mut src_diff: *const i16, src_stride: isize, mut coeff: *mut i16) {
    let mut src: [__m128i; 8] = core::mem::zeroed();
    src[0] = _mm_load_si128(src_diff as *const __m128i);
    src[1] = _mm_load_si128({
        src_diff = src_diff.offset(src_stride);
        src_diff
    } as *const __m128i);
    src[2] = _mm_load_si128({
        src_diff = src_diff.offset(src_stride);
        src_diff
    } as *const __m128i);
    src[3] = _mm_load_si128({
        src_diff = src_diff.offset(src_stride);
        src_diff
    } as *const __m128i);
    src[4] = _mm_load_si128({
        src_diff = src_diff.offset(src_stride);
        src_diff
    } as *const __m128i);
    src[5] = _mm_load_si128({
        src_diff = src_diff.offset(src_stride);
        src_diff
    } as *const __m128i);
    src[6] = _mm_load_si128({
        src_diff = src_diff.offset(src_stride);
        src_diff
    } as *const __m128i);
    src[7] = _mm_load_si128(src_diff.offset(src_stride) as *const __m128i);
    hadamard_col8_sse2(src.as_mut_ptr(), 0);
    hadamard_col8_sse2(src.as_mut_ptr(), 1);
    _mm_store_si128(coeff as *mut __m128i, src[0]);
    coeff = coeff.offset(8);
    _mm_store_si128(coeff as *mut __m128i, src[1]);
    coeff = coeff.offset(8);
    _mm_store_si128(coeff as *mut __m128i, src[2]);
    coeff = coeff.offset(8);
    _mm_store_si128(coeff as *mut __m128i, src[3]);
    coeff = coeff.offset(8);
    _mm_store_si128(coeff as *mut __m128i, src[4]);
    coeff = coeff.offset(8);
    _mm_store_si128(coeff as *mut __m128i, src[5]);
    coeff = coeff.offset(8);
    _mm_store_si128(coeff as *mut __m128i, src[6]);
    coeff = coeff.offset(8);
    _mm_store_si128(coeff as *mut __m128i, src[7]);
}

#[no_mangle]
#[target_feature(enable = "sse2")]
pub unsafe extern "C-unwind" fn aom_hadamard_lp_8x8_sse2(
    src_diff: *const i16,
    src_stride: isize,
    coeff: *mut i16,
) {
    hadamard_lp_8x8_sse2(src_diff, src_stride, coeff);
}

#[no_mangle]
#[target_feature(enable = "sse2")]
pub unsafe extern "C-unwind" fn aom_hadamard_lp_8x8_dual_sse2(
    src_diff: *const i16,
    src_stride: isize,
    coeff: *mut i16,
) {
    for i in 0..2 {
        hadamard_lp_8x8_sse2(
            src_diff.offset((i * 8) as isize),
            src_stride,
            coeff.offset((i * 64) as isize),
        );
    }
}

#[no_mangle]
#[target_feature(enable = "sse2")]
pub unsafe extern "C-unwind" fn aom_hadamard_lp_16x16_sse2(
    src_diff: *const i16,
    src_stride: isize,
    coeff: *mut i16,
) {
    for idx in 0..4 {
        let mut src_ptr: *const i16 = src_diff
            .offset(((idx >> 1) * 8) as isize * src_stride)
            .offset(((idx & 0x01) * 8) as isize);
        hadamard_lp_8x8_sse2(src_ptr, src_stride, coeff.offset((idx * 64) as isize));
    }
    let mut t_coeff: *mut i16 = coeff;
    for idx in (0..64).step_by(8) {
        let mut coeff0: __m128i = _mm_load_si128(t_coeff as *const __m128i);
        let mut coeff1: __m128i = _mm_load_si128(t_coeff.offset(64) as *const __m128i);
        let mut coeff2: __m128i = _mm_load_si128(t_coeff.offset(128) as *const __m128i);
        let mut coeff3: __m128i = _mm_load_si128(t_coeff.offset(192) as *const __m128i);
        let mut b0: __m128i = _mm_add_epi16(coeff0, coeff1);
        let mut b1: __m128i = _mm_sub_epi16(coeff0, coeff1);
        let mut b2: __m128i = _mm_add_epi16(coeff2, coeff3);
        let mut b3: __m128i = _mm_sub_epi16(coeff2, coeff3);
        b0 = _mm_srai_epi16::<{ 1 }>(b0);
        b1 = _mm_srai_epi16::<{ 1 }>(b1);
        b2 = _mm_srai_epi16::<{ 1 }>(b2);
        b3 = _mm_srai_epi16::<{ 1 }>(b3);
        coeff0 = _mm_add_epi16(b0, b2);
        coeff1 = _mm_add_epi16(b1, b3);
        coeff2 = _mm_sub_epi16(b0, b2);
        coeff3 = _mm_sub_epi16(b1, b3);
        _mm_store_si128(t_coeff as *mut __m128i, coeff0);
        _mm_store_si128(t_coeff.offset(64) as *mut __m128i, coeff1);
        _mm_store_si128(t_coeff.offset(128) as *mut __m128i, coeff2);
        _mm_store_si128(t_coeff.offset(192) as *mut __m128i, coeff3);
        t_coeff = t_coeff.offset(8);
    }
}

#[inline]
#[target_feature(enable = "sse2")]
unsafe fn hadamard_16x16_sse2(
    src_diff: *const i16,
    src_stride: isize,
    mut coeff: *mut i32,
    is_final: i32,
) {
    // For high bitdepths, it is unnecessary to store_tran_low
    // (mult/unpack/store), then load_tran_low (load/pack) the same memory in the
    // next stage.  Output to an intermediate buffer first, then store_tran_low()
    // in the final stage.
    let mut temp_coeff: Aligned32<[i16; (16 * 16) as usize]> = core::mem::zeroed();
    let mut t_coeff: *mut i16 = temp_coeff.0.as_mut_ptr();
    let mut coeff16: *mut i16 = coeff as *mut i16;
    let mut idx: i32 = 0;
    {
        idx = 0;
        while idx < 4 {
            let mut src_ptr: *const i16 = src_diff
                .offset(((idx >> 1) * 8) as isize * src_stride)
                .offset(((idx & 0x01) * 8) as isize);
            hadamard_8x8_sse2(
                src_ptr,
                src_stride,
                t_coeff.offset((idx * 64) as isize) as *mut i32,
                0,
            );
            idx += 1;
        }
    }
    {
        idx = 0;
        while idx < 64 {
            let mut coeff0: __m128i = _mm_load_si128(t_coeff as *const __m128i);
            let mut coeff1: __m128i = _mm_load_si128(t_coeff.offset(64) as *const __m128i);
            let mut coeff2: __m128i = _mm_load_si128(t_coeff.offset(128) as *const __m128i);
            let mut coeff3: __m128i = _mm_load_si128(t_coeff.offset(192) as *const __m128i);
            let mut b0: __m128i = _mm_add_epi16(coeff0, coeff1);
            let mut b1: __m128i = _mm_sub_epi16(coeff0, coeff1);
            let mut b2: __m128i = _mm_add_epi16(coeff2, coeff3);
            let mut b3: __m128i = _mm_sub_epi16(coeff2, coeff3);
            b0 = _mm_srai_epi16::<{ 1 }>(b0);
            b1 = _mm_srai_epi16::<{ 1 }>(b1);
            b2 = _mm_srai_epi16::<{ 1 }>(b2);
            b3 = _mm_srai_epi16::<{ 1 }>(b3);
            coeff0 = _mm_add_epi16(b0, b2);
            coeff1 = _mm_add_epi16(b1, b3);
            coeff2 = _mm_sub_epi16(b0, b2);
            coeff3 = _mm_sub_epi16(b1, b3);
            if is_final != 0 {
                store_tran_low_offset_4(coeff0, coeff);
                store_tran_low_offset_4(coeff1, coeff.offset(64));
                store_tran_low_offset_4(coeff2, coeff.offset(128));
                store_tran_low_offset_4(coeff3, coeff.offset(192));
                coeff = coeff.offset(4);
            } else {
                _mm_store_si128(coeff16 as *mut __m128i, coeff0);
                _mm_store_si128(coeff16.offset(64) as *mut __m128i, coeff1);
                _mm_store_si128(coeff16.offset(128) as *mut __m128i, coeff2);
                _mm_store_si128(coeff16.offset(192) as *mut __m128i, coeff3);
                coeff16 = coeff16.offset(8);
            }
            t_coeff = t_coeff.offset(8);
            // Increment the pointer additionally by 0 and 8 in alternate
            // iterations(instead of 8) to ensure the coherency with the implementation
            // of store_tran_low_offset_4()
            coeff = coeff.offset((((idx >> 3) & 1) << 3) as isize);
            idx += 8;
        }
    }
}

#[no_mangle]
#[target_feature(enable = "sse2")]
pub unsafe extern "C-unwind" fn aom_hadamard_16x16_sse2(
    src_diff: *const i16,
    src_stride: isize,
    coeff: *mut i32,
) {
    hadamard_16x16_sse2(src_diff, src_stride, coeff, 1);
}

#[no_mangle]
#[target_feature(enable = "sse2")]
pub unsafe extern "C-unwind" fn aom_hadamard_32x32_sse2(
    src_diff: *const i16,
    src_stride: isize,
    mut coeff: *mut i32,
) {
    // For high bitdepths, it is unnecessary to store_tran_low
    // (mult/unpack/store), then load_tran_low (load/pack) the same memory in the
    // next stage.  Output to an intermediate buffer first, then store_tran_low()
    // in the final stage.
    let mut temp_coeff: Aligned32<[i16; (32 * 32) as usize]> = core::mem::zeroed();
    let mut t_coeff: *mut i16 = temp_coeff.0.as_mut_ptr();
    let mut idx: i32 = 0;
    let mut coeff0_lo: __m128i = core::mem::zeroed();
    let mut coeff1_lo: __m128i = core::mem::zeroed();
    let mut coeff2_lo: __m128i = core::mem::zeroed();
    let mut coeff3_lo: __m128i = core::mem::zeroed();
    let mut b0_lo: __m128i;
    let mut b1_lo: __m128i;
    let mut b2_lo: __m128i;
    let mut b3_lo: __m128i;
    let mut coeff0_hi: __m128i = core::mem::zeroed();
    let mut coeff1_hi: __m128i = core::mem::zeroed();
    let mut coeff2_hi: __m128i = core::mem::zeroed();
    let mut coeff3_hi: __m128i = core::mem::zeroed();
    let mut b0_hi: __m128i;
    let mut b1_hi: __m128i;
    let mut b2_hi: __m128i;
    let mut b3_hi: __m128i;
    let mut b0: __m128i;
    let mut b1: __m128i;
    let mut b2: __m128i;
    let mut b3: __m128i;
    let zero: __m128i = _mm_setzero_si128();
    {
        idx = 0;
        while idx < 4 {
            let mut src_ptr: *const i16 = src_diff
                .offset(((idx >> 1) * 16) as isize * src_stride)
                .offset(((idx & 0x01) * 16) as isize);
            hadamard_16x16_sse2(
                src_ptr,
                src_stride,
                t_coeff.offset((idx * 256) as isize) as *mut i32,
                0,
            );
            idx += 1;
        }
    }
    {
        idx = 0;
        while idx < 256 {
            let mut coeff0: __m128i = _mm_load_si128(t_coeff as *const __m128i);
            let mut coeff1: __m128i = _mm_load_si128(t_coeff.offset(256) as *const __m128i);
            let mut coeff2: __m128i = _mm_load_si128(t_coeff.offset(512) as *const __m128i);
            let mut coeff3: __m128i = _mm_load_si128(t_coeff.offset(768) as *const __m128i);
            // Sign extend 16 bit to 32 bit.
            sign_extend_16bit_to_32bit_sse2(coeff0, zero, &raw mut coeff0_lo, &raw mut coeff0_hi);
            sign_extend_16bit_to_32bit_sse2(coeff1, zero, &raw mut coeff1_lo, &raw mut coeff1_hi);
            sign_extend_16bit_to_32bit_sse2(coeff2, zero, &raw mut coeff2_lo, &raw mut coeff2_hi);
            sign_extend_16bit_to_32bit_sse2(coeff3, zero, &raw mut coeff3_lo, &raw mut coeff3_hi);
            b0_lo = _mm_add_epi32(coeff0_lo, coeff1_lo);
            b0_hi = _mm_add_epi32(coeff0_hi, coeff1_hi);
            b1_lo = _mm_sub_epi32(coeff0_lo, coeff1_lo);
            b1_hi = _mm_sub_epi32(coeff0_hi, coeff1_hi);
            b2_lo = _mm_add_epi32(coeff2_lo, coeff3_lo);
            b2_hi = _mm_add_epi32(coeff2_hi, coeff3_hi);
            b3_lo = _mm_sub_epi32(coeff2_lo, coeff3_lo);
            b3_hi = _mm_sub_epi32(coeff2_hi, coeff3_hi);
            b0_lo = _mm_srai_epi32::<{ 2 }>(b0_lo);
            b1_lo = _mm_srai_epi32::<{ 2 }>(b1_lo);
            b2_lo = _mm_srai_epi32::<{ 2 }>(b2_lo);
            b3_lo = _mm_srai_epi32::<{ 2 }>(b3_lo);
            b0_hi = _mm_srai_epi32::<{ 2 }>(b0_hi);
            b1_hi = _mm_srai_epi32::<{ 2 }>(b1_hi);
            b2_hi = _mm_srai_epi32::<{ 2 }>(b2_hi);
            b3_hi = _mm_srai_epi32::<{ 2 }>(b3_hi);
            b0 = _mm_packs_epi32(b0_lo, b0_hi);
            b1 = _mm_packs_epi32(b1_lo, b1_hi);
            b2 = _mm_packs_epi32(b2_lo, b2_hi);
            b3 = _mm_packs_epi32(b3_lo, b3_hi);
            coeff0 = _mm_add_epi16(b0, b2);
            coeff1 = _mm_add_epi16(b1, b3);
            store_tran_low_offset_4(coeff0, coeff);
            store_tran_low_offset_4(coeff1, coeff.offset(256));
            coeff2 = _mm_sub_epi16(b0, b2);
            coeff3 = _mm_sub_epi16(b1, b3);
            store_tran_low_offset_4(coeff2, coeff.offset(512));
            store_tran_low_offset_4(coeff3, coeff.offset(768));
            // Increment the pointer by 4 and 12 in alternate iterations(instead of 8)
            // to ensure the coherency with the implementation of
            // store_tran_low_offset_4()
            coeff = coeff.offset((4 + (((idx >> 3) & 1) << 3)) as isize);
            t_coeff = t_coeff.offset(8);
            idx += 8;
        }
    }
}

// SIMD implementation assumes width to be multiple of 16.
