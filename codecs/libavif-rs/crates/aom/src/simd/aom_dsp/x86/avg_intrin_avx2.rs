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

// Translated from upstream/aom/aom_dsp/x86/avg_intrin_avx2.c (ADR 0056, ADR 0058).
// Upstream compiles it with -mavx2. The functions were
// transliterated with a typing-aware C-subset printer (not C2Rust) and
// reviewed against the C function by function: the same sequence of intrinsic
// and helper calls, constants and operators in order (ADR 0058).
// Only the Hadamard kernels of the file are translated (the config's `only`
// list; the averaging, SATD, projection and variance kernels after them are
// not on the still-image path).
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

use crate::simd::aom_dsp__x86__bitdepth_conversion_avx2::*;
use core::arch::x86_64::*;

#[repr(C, align(16))]
struct Aligned16<T>(T);
#[repr(C, align(32))]
struct Aligned32<T>(T);

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn sign_extend_16bit_to_32bit_avx2(
    in_: __m256i,
    zero: __m256i,
    out_lo: *mut __m256i,
    out_hi: *mut __m256i,
) {
    let sign_bits: __m256i = _mm256_cmpgt_epi16(zero, in_);
    *out_lo = _mm256_unpacklo_epi16(in_, sign_bits);
    *out_hi = _mm256_unpackhi_epi16(in_, sign_bits);
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn hadamard_col8x2_avx2(in_: *mut __m256i, iter: i32) {
    let mut a0: __m256i = *in_;
    let mut a1: __m256i = *in_.offset(1);
    let mut a2: __m256i = *in_.offset(2);
    let mut a3: __m256i = *in_.offset(3);
    let mut a4: __m256i = *in_.offset(4);
    let mut a5: __m256i = *in_.offset(5);
    let mut a6: __m256i = *in_.offset(6);
    let mut a7: __m256i = *in_.offset(7);
    let mut b0: __m256i = _mm256_add_epi16(a0, a1);
    let mut b1: __m256i = _mm256_sub_epi16(a0, a1);
    let mut b2: __m256i = _mm256_add_epi16(a2, a3);
    let mut b3: __m256i = _mm256_sub_epi16(a2, a3);
    let mut b4: __m256i = _mm256_add_epi16(a4, a5);
    let mut b5: __m256i = _mm256_sub_epi16(a4, a5);
    let mut b6: __m256i = _mm256_add_epi16(a6, a7);
    let mut b7: __m256i = _mm256_sub_epi16(a6, a7);
    a0 = _mm256_add_epi16(b0, b2);
    a1 = _mm256_add_epi16(b1, b3);
    a2 = _mm256_sub_epi16(b0, b2);
    a3 = _mm256_sub_epi16(b1, b3);
    a4 = _mm256_add_epi16(b4, b6);
    a5 = _mm256_add_epi16(b5, b7);
    a6 = _mm256_sub_epi16(b4, b6);
    a7 = _mm256_sub_epi16(b5, b7);
    if iter == 0 {
        b0 = _mm256_add_epi16(a0, a4);
        b7 = _mm256_add_epi16(a1, a5);
        b3 = _mm256_add_epi16(a2, a6);
        b4 = _mm256_add_epi16(a3, a7);
        b2 = _mm256_sub_epi16(a0, a4);
        b6 = _mm256_sub_epi16(a1, a5);
        b1 = _mm256_sub_epi16(a2, a6);
        b5 = _mm256_sub_epi16(a3, a7);
        a0 = _mm256_unpacklo_epi16(b0, b1);
        a1 = _mm256_unpacklo_epi16(b2, b3);
        a2 = _mm256_unpackhi_epi16(b0, b1);
        a3 = _mm256_unpackhi_epi16(b2, b3);
        a4 = _mm256_unpacklo_epi16(b4, b5);
        a5 = _mm256_unpacklo_epi16(b6, b7);
        a6 = _mm256_unpackhi_epi16(b4, b5);
        a7 = _mm256_unpackhi_epi16(b6, b7);
        b0 = _mm256_unpacklo_epi32(a0, a1);
        b1 = _mm256_unpacklo_epi32(a4, a5);
        b2 = _mm256_unpackhi_epi32(a0, a1);
        b3 = _mm256_unpackhi_epi32(a4, a5);
        b4 = _mm256_unpacklo_epi32(a2, a3);
        b5 = _mm256_unpacklo_epi32(a6, a7);
        b6 = _mm256_unpackhi_epi32(a2, a3);
        b7 = _mm256_unpackhi_epi32(a6, a7);
        *in_ = _mm256_unpacklo_epi64(b0, b1);
        *in_.offset(1) = _mm256_unpackhi_epi64(b0, b1);
        *in_.offset(2) = _mm256_unpacklo_epi64(b2, b3);
        *in_.offset(3) = _mm256_unpackhi_epi64(b2, b3);
        *in_.offset(4) = _mm256_unpacklo_epi64(b4, b5);
        *in_.offset(5) = _mm256_unpackhi_epi64(b4, b5);
        *in_.offset(6) = _mm256_unpacklo_epi64(b6, b7);
        *in_.offset(7) = _mm256_unpackhi_epi64(b6, b7);
    } else {
        *in_ = _mm256_add_epi16(a0, a4);
        *in_.offset(7) = _mm256_add_epi16(a1, a5);
        *in_.offset(3) = _mm256_add_epi16(a2, a6);
        *in_.offset(4) = _mm256_add_epi16(a3, a7);
        *in_.offset(2) = _mm256_sub_epi16(a0, a4);
        *in_.offset(6) = _mm256_sub_epi16(a1, a5);
        *in_.offset(1) = _mm256_sub_epi16(a2, a6);
        *in_.offset(5) = _mm256_sub_epi16(a3, a7);
    }
}

#[no_mangle]
#[target_feature(enable = "avx2")]
pub unsafe extern "C-unwind" fn aom_hadamard_lp_8x8_dual_avx2(
    mut src_diff: *const i16,
    src_stride: isize,
    mut coeff: *mut i16,
) {
    let mut src: [__m256i; 8] = core::mem::zeroed();
    src[0] = _mm256_loadu_si256(src_diff as *const __m256i);
    src[1] = _mm256_loadu_si256({
        src_diff = src_diff.offset(src_stride);
        src_diff
    } as *const __m256i);
    src[2] = _mm256_loadu_si256({
        src_diff = src_diff.offset(src_stride);
        src_diff
    } as *const __m256i);
    src[3] = _mm256_loadu_si256({
        src_diff = src_diff.offset(src_stride);
        src_diff
    } as *const __m256i);
    src[4] = _mm256_loadu_si256({
        src_diff = src_diff.offset(src_stride);
        src_diff
    } as *const __m256i);
    src[5] = _mm256_loadu_si256({
        src_diff = src_diff.offset(src_stride);
        src_diff
    } as *const __m256i);
    src[6] = _mm256_loadu_si256({
        src_diff = src_diff.offset(src_stride);
        src_diff
    } as *const __m256i);
    src[7] = _mm256_loadu_si256(src_diff.offset(src_stride) as *const __m256i);
    hadamard_col8x2_avx2(src.as_mut_ptr(), 0);
    hadamard_col8x2_avx2(src.as_mut_ptr(), 1);
    _mm256_storeu_si256(
        coeff as *mut __m256i,
        _mm256_permute2x128_si256::<0x20>(src[0], src[1]),
    );
    coeff = coeff.offset(16);
    _mm256_storeu_si256(
        coeff as *mut __m256i,
        _mm256_permute2x128_si256::<0x20>(src[2], src[3]),
    );
    coeff = coeff.offset(16);
    _mm256_storeu_si256(
        coeff as *mut __m256i,
        _mm256_permute2x128_si256::<0x20>(src[4], src[5]),
    );
    coeff = coeff.offset(16);
    _mm256_storeu_si256(
        coeff as *mut __m256i,
        _mm256_permute2x128_si256::<0x20>(src[6], src[7]),
    );
    coeff = coeff.offset(16);
    _mm256_storeu_si256(
        coeff as *mut __m256i,
        _mm256_permute2x128_si256::<0x31>(src[0], src[1]),
    );
    coeff = coeff.offset(16);
    _mm256_storeu_si256(
        coeff as *mut __m256i,
        _mm256_permute2x128_si256::<0x31>(src[2], src[3]),
    );
    coeff = coeff.offset(16);
    _mm256_storeu_si256(
        coeff as *mut __m256i,
        _mm256_permute2x128_si256::<0x31>(src[4], src[5]),
    );
    coeff = coeff.offset(16);
    _mm256_storeu_si256(
        coeff as *mut __m256i,
        _mm256_permute2x128_si256::<0x31>(src[6], src[7]),
    );
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn hadamard_16x16_avx2(
    src_diff: *const i16,
    src_stride: isize,
    mut coeff: *mut i32,
    is_final: i32,
) {
    let mut temp_coeff: Aligned32<[i16; (16 * 16) as usize]> = core::mem::zeroed();
    let mut t_coeff: *mut i16 = temp_coeff.0.as_mut_ptr();
    let mut coeff16: *mut i16 = coeff as *mut i16;
    let mut idx: i32 = 0;
    {
        idx = 0;
        while idx < 2 {
            let mut src_ptr: *const i16 = src_diff.offset((idx * 8) as isize * src_stride);
            aom_hadamard_lp_8x8_dual_avx2(
                src_ptr,
                src_stride,
                t_coeff.offset((idx * 64 * 2) as isize),
            );
            idx += 1;
        }
    }
    {
        idx = 0;
        while idx < 64 {
            let coeff0: __m256i = _mm256_loadu_si256(t_coeff as *const __m256i);
            let coeff1: __m256i = _mm256_loadu_si256(t_coeff.offset(64) as *const __m256i);
            let coeff2: __m256i = _mm256_loadu_si256(t_coeff.offset(128) as *const __m256i);
            let coeff3: __m256i = _mm256_loadu_si256(t_coeff.offset(192) as *const __m256i);
            let mut b0: __m256i = _mm256_add_epi16(coeff0, coeff1);
            let mut b1: __m256i = _mm256_sub_epi16(coeff0, coeff1);
            let mut b2: __m256i = _mm256_add_epi16(coeff2, coeff3);
            let mut b3: __m256i = _mm256_sub_epi16(coeff2, coeff3);
            b0 = _mm256_srai_epi16::<{ 1 }>(b0);
            b1 = _mm256_srai_epi16::<{ 1 }>(b1);
            b2 = _mm256_srai_epi16::<{ 1 }>(b2);
            b3 = _mm256_srai_epi16::<{ 1 }>(b3);
            if is_final != 0 {
                store_tran_low(_mm256_add_epi16(b0, b2), coeff);
                store_tran_low(_mm256_add_epi16(b1, b3), coeff.offset(64));
                store_tran_low(_mm256_sub_epi16(b0, b2), coeff.offset(128));
                store_tran_low(_mm256_sub_epi16(b1, b3), coeff.offset(192));
                coeff = coeff.offset(16);
            } else {
                _mm256_storeu_si256(coeff16 as *mut __m256i, _mm256_add_epi16(b0, b2));
                _mm256_storeu_si256(coeff16.offset(64) as *mut __m256i, _mm256_add_epi16(b1, b3));
                _mm256_storeu_si256(
                    coeff16.offset(128) as *mut __m256i,
                    _mm256_sub_epi16(b0, b2),
                );
                _mm256_storeu_si256(
                    coeff16.offset(192) as *mut __m256i,
                    _mm256_sub_epi16(b1, b3),
                );
                coeff16 = coeff16.offset(16);
            }
            t_coeff = t_coeff.offset(16);
            idx += 16;
        }
    }
}

#[no_mangle]
#[target_feature(enable = "avx2")]
pub unsafe extern "C-unwind" fn aom_hadamard_16x16_avx2(
    src_diff: *const i16,
    src_stride: isize,
    coeff: *mut i32,
) {
    hadamard_16x16_avx2(src_diff, src_stride, coeff, 1);
}

#[no_mangle]
#[target_feature(enable = "avx2")]
pub unsafe extern "C-unwind" fn aom_hadamard_lp_16x16_avx2(
    src_diff: *const i16,
    src_stride: isize,
    mut coeff: *mut i16,
) {
    let mut t_coeff: *mut i16 = coeff;
    for idx in 0..2 {
        let mut src_ptr: *const i16 = src_diff.offset((idx * 8) as isize * src_stride);
        aom_hadamard_lp_8x8_dual_avx2(src_ptr, src_stride, t_coeff.offset((idx * 64 * 2) as isize));
    }
    for idx in (0..64).step_by(16) {
        let coeff0: __m256i = _mm256_loadu_si256(t_coeff as *const __m256i);
        let coeff1: __m256i = _mm256_loadu_si256(t_coeff.offset(64) as *const __m256i);
        let coeff2: __m256i = _mm256_loadu_si256(t_coeff.offset(128) as *const __m256i);
        let coeff3: __m256i = _mm256_loadu_si256(t_coeff.offset(192) as *const __m256i);
        let mut b0: __m256i = _mm256_add_epi16(coeff0, coeff1);
        let mut b1: __m256i = _mm256_sub_epi16(coeff0, coeff1);
        let mut b2: __m256i = _mm256_add_epi16(coeff2, coeff3);
        let mut b3: __m256i = _mm256_sub_epi16(coeff2, coeff3);
        b0 = _mm256_srai_epi16::<{ 1 }>(b0);
        b1 = _mm256_srai_epi16::<{ 1 }>(b1);
        b2 = _mm256_srai_epi16::<{ 1 }>(b2);
        b3 = _mm256_srai_epi16::<{ 1 }>(b3);
        _mm256_storeu_si256(coeff as *mut __m256i, _mm256_add_epi16(b0, b2));
        _mm256_storeu_si256(coeff.offset(64) as *mut __m256i, _mm256_add_epi16(b1, b3));
        _mm256_storeu_si256(coeff.offset(128) as *mut __m256i, _mm256_sub_epi16(b0, b2));
        _mm256_storeu_si256(coeff.offset(192) as *mut __m256i, _mm256_sub_epi16(b1, b3));
        coeff = coeff.offset(16);
        t_coeff = t_coeff.offset(16);
    }
}

#[no_mangle]
#[target_feature(enable = "avx2")]
pub unsafe extern "C-unwind" fn aom_hadamard_32x32_avx2(
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
    let mut coeff0_lo: __m256i = core::mem::zeroed();
    let mut coeff1_lo: __m256i = core::mem::zeroed();
    let mut coeff2_lo: __m256i = core::mem::zeroed();
    let mut coeff3_lo: __m256i = core::mem::zeroed();
    let mut b0_lo: __m256i;
    let mut b1_lo: __m256i;
    let mut b2_lo: __m256i;
    let mut b3_lo: __m256i;
    let mut coeff0_hi: __m256i = core::mem::zeroed();
    let mut coeff1_hi: __m256i = core::mem::zeroed();
    let mut coeff2_hi: __m256i = core::mem::zeroed();
    let mut coeff3_hi: __m256i = core::mem::zeroed();
    let mut b0_hi: __m256i;
    let mut b1_hi: __m256i;
    let mut b2_hi: __m256i;
    let mut b3_hi: __m256i;
    let mut b0: __m256i;
    let mut b1: __m256i;
    let mut b2: __m256i;
    let mut b3: __m256i;
    let zero: __m256i = _mm256_setzero_si256();
    {
        idx = 0;
        while idx < 4 {
            // src_diff: 9 bit, dynamic range [-255, 255]
            let mut src_ptr: *const i16 = src_diff
                .offset(((idx >> 1) * 16) as isize * src_stride)
                .offset(((idx & 0x01) * 16) as isize);
            hadamard_16x16_avx2(
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
            let coeff0: __m256i = _mm256_loadu_si256(t_coeff as *const __m256i);
            let coeff1: __m256i = _mm256_loadu_si256(t_coeff.offset(256) as *const __m256i);
            let coeff2: __m256i = _mm256_loadu_si256(t_coeff.offset(512) as *const __m256i);
            let coeff3: __m256i = _mm256_loadu_si256(t_coeff.offset(768) as *const __m256i);
            // Sign extend 16 bit to 32 bit.
            sign_extend_16bit_to_32bit_avx2(coeff0, zero, &raw mut coeff0_lo, &raw mut coeff0_hi);
            sign_extend_16bit_to_32bit_avx2(coeff1, zero, &raw mut coeff1_lo, &raw mut coeff1_hi);
            sign_extend_16bit_to_32bit_avx2(coeff2, zero, &raw mut coeff2_lo, &raw mut coeff2_hi);
            sign_extend_16bit_to_32bit_avx2(coeff3, zero, &raw mut coeff3_lo, &raw mut coeff3_hi);
            b0_lo = _mm256_add_epi32(coeff0_lo, coeff1_lo);
            b0_hi = _mm256_add_epi32(coeff0_hi, coeff1_hi);
            b1_lo = _mm256_sub_epi32(coeff0_lo, coeff1_lo);
            b1_hi = _mm256_sub_epi32(coeff0_hi, coeff1_hi);
            b2_lo = _mm256_add_epi32(coeff2_lo, coeff3_lo);
            b2_hi = _mm256_add_epi32(coeff2_hi, coeff3_hi);
            b3_lo = _mm256_sub_epi32(coeff2_lo, coeff3_lo);
            b3_hi = _mm256_sub_epi32(coeff2_hi, coeff3_hi);
            b0_lo = _mm256_srai_epi32::<{ 2 }>(b0_lo);
            b1_lo = _mm256_srai_epi32::<{ 2 }>(b1_lo);
            b2_lo = _mm256_srai_epi32::<{ 2 }>(b2_lo);
            b3_lo = _mm256_srai_epi32::<{ 2 }>(b3_lo);
            b0_hi = _mm256_srai_epi32::<{ 2 }>(b0_hi);
            b1_hi = _mm256_srai_epi32::<{ 2 }>(b1_hi);
            b2_hi = _mm256_srai_epi32::<{ 2 }>(b2_hi);
            b3_hi = _mm256_srai_epi32::<{ 2 }>(b3_hi);
            b0 = _mm256_packs_epi32(b0_lo, b0_hi);
            b1 = _mm256_packs_epi32(b1_lo, b1_hi);
            b2 = _mm256_packs_epi32(b2_lo, b2_hi);
            b3 = _mm256_packs_epi32(b3_lo, b3_hi);
            store_tran_low(_mm256_add_epi16(b0, b2), coeff);
            store_tran_low(_mm256_add_epi16(b1, b3), coeff.offset(256));
            store_tran_low(_mm256_sub_epi16(b0, b2), coeff.offset(512));
            store_tran_low(_mm256_sub_epi16(b1, b3), coeff.offset(768));
            coeff = coeff.offset(16);
            t_coeff = t_coeff.offset(16);
            idx += 16;
        }
    }
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn highbd_hadamard_col8_avx2(in_: *mut __m256i, iter: i32) {
    let mut a0: __m256i = *in_;
    let mut a1: __m256i = *in_.offset(1);
    let mut a2: __m256i = *in_.offset(2);
    let mut a3: __m256i = *in_.offset(3);
    let mut a4: __m256i = *in_.offset(4);
    let mut a5: __m256i = *in_.offset(5);
    let mut a6: __m256i = *in_.offset(6);
    let mut a7: __m256i = *in_.offset(7);
    let mut b0: __m256i = _mm256_add_epi32(a0, a1);
    let mut b1: __m256i = _mm256_sub_epi32(a0, a1);
    let mut b2: __m256i = _mm256_add_epi32(a2, a3);
    let mut b3: __m256i = _mm256_sub_epi32(a2, a3);
    let mut b4: __m256i = _mm256_add_epi32(a4, a5);
    let mut b5: __m256i = _mm256_sub_epi32(a4, a5);
    let mut b6: __m256i = _mm256_add_epi32(a6, a7);
    let mut b7: __m256i = _mm256_sub_epi32(a6, a7);
    a0 = _mm256_add_epi32(b0, b2);
    a1 = _mm256_add_epi32(b1, b3);
    a2 = _mm256_sub_epi32(b0, b2);
    a3 = _mm256_sub_epi32(b1, b3);
    a4 = _mm256_add_epi32(b4, b6);
    a5 = _mm256_add_epi32(b5, b7);
    a6 = _mm256_sub_epi32(b4, b6);
    a7 = _mm256_sub_epi32(b5, b7);
    if iter == 0 {
        b0 = _mm256_add_epi32(a0, a4);
        b7 = _mm256_add_epi32(a1, a5);
        b3 = _mm256_add_epi32(a2, a6);
        b4 = _mm256_add_epi32(a3, a7);
        b2 = _mm256_sub_epi32(a0, a4);
        b6 = _mm256_sub_epi32(a1, a5);
        b1 = _mm256_sub_epi32(a2, a6);
        b5 = _mm256_sub_epi32(a3, a7);
        a0 = _mm256_unpacklo_epi32(b0, b1);
        a1 = _mm256_unpacklo_epi32(b2, b3);
        a2 = _mm256_unpackhi_epi32(b0, b1);
        a3 = _mm256_unpackhi_epi32(b2, b3);
        a4 = _mm256_unpacklo_epi32(b4, b5);
        a5 = _mm256_unpacklo_epi32(b6, b7);
        a6 = _mm256_unpackhi_epi32(b4, b5);
        a7 = _mm256_unpackhi_epi32(b6, b7);
        b0 = _mm256_unpacklo_epi64(a0, a1);
        b1 = _mm256_unpacklo_epi64(a4, a5);
        b2 = _mm256_unpackhi_epi64(a0, a1);
        b3 = _mm256_unpackhi_epi64(a4, a5);
        b4 = _mm256_unpacklo_epi64(a2, a3);
        b5 = _mm256_unpacklo_epi64(a6, a7);
        b6 = _mm256_unpackhi_epi64(a2, a3);
        b7 = _mm256_unpackhi_epi64(a6, a7);
        *in_ = _mm256_permute2x128_si256::<0x20>(b0, b1);
        *in_.offset(1) = _mm256_permute2x128_si256::<0x31>(b0, b1);
        *in_.offset(2) = _mm256_permute2x128_si256::<0x20>(b2, b3);
        *in_.offset(3) = _mm256_permute2x128_si256::<0x31>(b2, b3);
        *in_.offset(4) = _mm256_permute2x128_si256::<0x20>(b4, b5);
        *in_.offset(5) = _mm256_permute2x128_si256::<0x31>(b4, b5);
        *in_.offset(6) = _mm256_permute2x128_si256::<0x20>(b6, b7);
        *in_.offset(7) = _mm256_permute2x128_si256::<0x31>(b6, b7);
    } else {
        *in_ = _mm256_add_epi32(a0, a4);
        *in_.offset(7) = _mm256_add_epi32(a1, a5);
        *in_.offset(3) = _mm256_add_epi32(a2, a6);
        *in_.offset(4) = _mm256_add_epi32(a3, a7);
        *in_.offset(2) = _mm256_sub_epi32(a0, a4);
        *in_.offset(6) = _mm256_sub_epi32(a1, a5);
        *in_.offset(1) = _mm256_sub_epi32(a2, a6);
        *in_.offset(5) = _mm256_sub_epi32(a3, a7);
    }
}

#[no_mangle]
#[target_feature(enable = "avx2")]
pub unsafe extern "C-unwind" fn aom_highbd_hadamard_8x8_avx2(
    mut src_diff: *const i16,
    src_stride: isize,
    mut coeff: *mut i32,
) {
    let mut src16: [__m128i; 8] = core::mem::zeroed();
    let mut src32: [__m256i; 8] = core::mem::zeroed();
    src16[0] = _mm_loadu_si128(src_diff as *const __m128i);
    src16[1] = _mm_loadu_si128({
        src_diff = src_diff.offset(src_stride);
        src_diff
    } as *const __m128i);
    src16[2] = _mm_loadu_si128({
        src_diff = src_diff.offset(src_stride);
        src_diff
    } as *const __m128i);
    src16[3] = _mm_loadu_si128({
        src_diff = src_diff.offset(src_stride);
        src_diff
    } as *const __m128i);
    src16[4] = _mm_loadu_si128({
        src_diff = src_diff.offset(src_stride);
        src_diff
    } as *const __m128i);
    src16[5] = _mm_loadu_si128({
        src_diff = src_diff.offset(src_stride);
        src_diff
    } as *const __m128i);
    src16[6] = _mm_loadu_si128({
        src_diff = src_diff.offset(src_stride);
        src_diff
    } as *const __m128i);
    src16[7] = _mm_loadu_si128(src_diff.offset(src_stride) as *const __m128i);
    src32[0] = _mm256_cvtepi16_epi32(src16[0]);
    src32[1] = _mm256_cvtepi16_epi32(src16[1]);
    src32[2] = _mm256_cvtepi16_epi32(src16[2]);
    src32[3] = _mm256_cvtepi16_epi32(src16[3]);
    src32[4] = _mm256_cvtepi16_epi32(src16[4]);
    src32[5] = _mm256_cvtepi16_epi32(src16[5]);
    src32[6] = _mm256_cvtepi16_epi32(src16[6]);
    src32[7] = _mm256_cvtepi16_epi32(src16[7]);
    highbd_hadamard_col8_avx2(src32.as_mut_ptr(), 0);
    highbd_hadamard_col8_avx2(src32.as_mut_ptr(), 1);
    _mm256_storeu_si256(coeff as *mut __m256i, src32[0]);
    coeff = coeff.offset(8);
    _mm256_storeu_si256(coeff as *mut __m256i, src32[1]);
    coeff = coeff.offset(8);
    _mm256_storeu_si256(coeff as *mut __m256i, src32[2]);
    coeff = coeff.offset(8);
    _mm256_storeu_si256(coeff as *mut __m256i, src32[3]);
    coeff = coeff.offset(8);
    _mm256_storeu_si256(coeff as *mut __m256i, src32[4]);
    coeff = coeff.offset(8);
    _mm256_storeu_si256(coeff as *mut __m256i, src32[5]);
    coeff = coeff.offset(8);
    _mm256_storeu_si256(coeff as *mut __m256i, src32[6]);
    coeff = coeff.offset(8);
    _mm256_storeu_si256(coeff as *mut __m256i, src32[7]);
}

#[no_mangle]
#[target_feature(enable = "avx2")]
pub unsafe extern "C-unwind" fn aom_highbd_hadamard_16x16_avx2(
    src_diff: *const i16,
    src_stride: isize,
    mut coeff: *mut i32,
) {
    let mut idx: i32 = 0;
    let mut t_coeff: *mut i32 = coeff;
    {
        idx = 0;
        while idx < 4 {
            let mut src_ptr: *const i16 = src_diff
                .offset(((idx >> 1) * 8) as isize * src_stride)
                .offset(((idx & 0x01) * 8) as isize);
            aom_highbd_hadamard_8x8_avx2(src_ptr, src_stride, t_coeff.offset((idx * 64) as isize));
            idx += 1;
        }
    }
    {
        idx = 0;
        while idx < 64 {
            let mut coeff0: __m256i = _mm256_loadu_si256(t_coeff as *const __m256i);
            let mut coeff1: __m256i = _mm256_loadu_si256(t_coeff.offset(64) as *const __m256i);
            let mut coeff2: __m256i = _mm256_loadu_si256(t_coeff.offset(128) as *const __m256i);
            let mut coeff3: __m256i = _mm256_loadu_si256(t_coeff.offset(192) as *const __m256i);
            let mut b0: __m256i = _mm256_add_epi32(coeff0, coeff1);
            let mut b1: __m256i = _mm256_sub_epi32(coeff0, coeff1);
            let mut b2: __m256i = _mm256_add_epi32(coeff2, coeff3);
            let mut b3: __m256i = _mm256_sub_epi32(coeff2, coeff3);
            b0 = _mm256_srai_epi32::<{ 1 }>(b0);
            b1 = _mm256_srai_epi32::<{ 1 }>(b1);
            b2 = _mm256_srai_epi32::<{ 1 }>(b2);
            b3 = _mm256_srai_epi32::<{ 1 }>(b3);
            coeff0 = _mm256_add_epi32(b0, b2);
            coeff1 = _mm256_add_epi32(b1, b3);
            coeff2 = _mm256_sub_epi32(b0, b2);
            coeff3 = _mm256_sub_epi32(b1, b3);
            _mm256_storeu_si256(coeff as *mut __m256i, coeff0);
            _mm256_storeu_si256(coeff.offset(64) as *mut __m256i, coeff1);
            _mm256_storeu_si256(coeff.offset(128) as *mut __m256i, coeff2);
            _mm256_storeu_si256(coeff.offset(192) as *mut __m256i, coeff3);
            coeff = coeff.offset(8);
            t_coeff = t_coeff.offset(8);
            idx += 8;
        }
    }
}

#[no_mangle]
#[target_feature(enable = "avx2")]
pub unsafe extern "C-unwind" fn aom_highbd_hadamard_32x32_avx2(
    src_diff: *const i16,
    src_stride: isize,
    mut coeff: *mut i32,
) {
    let mut idx: i32 = 0;
    let mut t_coeff: *mut i32 = coeff;
    {
        idx = 0;
        while idx < 4 {
            let mut src_ptr: *const i16 = src_diff
                .offset(((idx >> 1) * 16) as isize * src_stride)
                .offset(((idx & 0x01) * 16) as isize);
            aom_highbd_hadamard_16x16_avx2(
                src_ptr,
                src_stride,
                t_coeff.offset((idx * 256) as isize),
            );
            idx += 1;
        }
    }
    {
        idx = 0;
        while idx < 256 {
            let mut coeff0: __m256i = _mm256_loadu_si256(t_coeff as *const __m256i);
            let mut coeff1: __m256i = _mm256_loadu_si256(t_coeff.offset(256) as *const __m256i);
            let mut coeff2: __m256i = _mm256_loadu_si256(t_coeff.offset(512) as *const __m256i);
            let mut coeff3: __m256i = _mm256_loadu_si256(t_coeff.offset(768) as *const __m256i);
            let mut b0: __m256i = _mm256_add_epi32(coeff0, coeff1);
            let mut b1: __m256i = _mm256_sub_epi32(coeff0, coeff1);
            let mut b2: __m256i = _mm256_add_epi32(coeff2, coeff3);
            let mut b3: __m256i = _mm256_sub_epi32(coeff2, coeff3);
            b0 = _mm256_srai_epi32::<{ 2 }>(b0);
            b1 = _mm256_srai_epi32::<{ 2 }>(b1);
            b2 = _mm256_srai_epi32::<{ 2 }>(b2);
            b3 = _mm256_srai_epi32::<{ 2 }>(b3);
            coeff0 = _mm256_add_epi32(b0, b2);
            coeff1 = _mm256_add_epi32(b1, b3);
            coeff2 = _mm256_sub_epi32(b0, b2);
            coeff3 = _mm256_sub_epi32(b1, b3);
            _mm256_storeu_si256(coeff as *mut __m256i, coeff0);
            _mm256_storeu_si256(coeff.offset(256) as *mut __m256i, coeff1);
            _mm256_storeu_si256(coeff.offset(512) as *mut __m256i, coeff2);
            _mm256_storeu_si256(coeff.offset(768) as *mut __m256i, coeff3);
            coeff = coeff.offset(8);
            t_coeff = t_coeff.offset(8);
            idx += 8;
        }
    }
}

// CONFIG_AV1_HIGHBITDEPTH
// 32 bit horizontal add
// 32 bit horizontal add
// (avg + 32) >> 6
// SIMD implementation assumes width and height to be multiple of 16 and 2
// respectively. For any odd width or height, SIMD support needs to be added.
// r00 r10 x x r01 r11 x x | r02 r12 x x r03 r13 x x
// r00 r10 r20 x r01 r11 r21 x | r02 r12 r22 x r03 r13 r23 x
// r00 r10 r20 r30 r01 r11 r21 r31 | r02 r12 r22 r32 r03 r13 r23 r33
// Post sad operation, the data is present in lower 16-bit of each 64-bit lane
// and higher 16-bits are Zero. Here, we are processing 8 rows at a time to
// utilize the higher 16-bits efficiently.
// s00 x x x s01 x x x | s40 x x x s41 x x x
// s10 x x x s11 x x x | s50 x x x s51 x x x
// s20 x x x s21 x x x | s60 x x x s61 x x x
// s30 x x x s31 x x x | s70 x x x s71 x x x
// s00 s10 x x x x x x | s40 s50 x x x x x x
// s01 s11 x x x x x x | s41 s51 x x x x x x
// s20 s30 x x x x x x | s60 s70 x x x x x x
// s21 s31 x x x x x x | s61 s71 x x x x x x
// s0 s1 x x x x x x | s4 s5 x x x x x x
// s2 s3 x x x x x x | s6 s7 x x x x x x
// s1 s1 s2 s3 s4 s5 s6 s7
// Load source data.
// Row0 output: r00 x x x r01 x x x | r02 x x x r03 x x x
// Row1 output: r10 x x x r11 x x x | r12 x x x r13 x x x
// Row2 output: r20 x x x r21 x x x | r22 x x x r23 x x x
// Row3 output: r30 x x x r31 x x x | r32 x x x r33 x x x
// Load source data.
// Row0 output: r00 x x x r01 x x x | r02 x x x r03 x x x
// Row1 output: r10 x x x r11 x x x | r12 x x x r13 x x x
// Row2 output: r20 x x x r21 x x x | r22 x x x r23 x x x
// Row3 output: r30 x x x r31 x x x | r32 x x x r33 x x x
// Load source data.
// s00 x x x s01 x x x s02 x x x s03 x x x
// s10 x x x s11 x x x s12 x x x s13 x x x
// s20 x x x s21 x x x s22 x x x s23 x x x
// s30 x x x s31 x x x s32 x x x s33 x x x
// (mean * mean): dynamic range 31 bits.
// ref: [0 - 510]
// src: [0 - 510]
// bwl: {2, 3, 4, 5}
// Instead of having a loop over width 16, considered loop unrolling to avoid
// some addition operations.
