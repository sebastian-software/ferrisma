/*
 * Copyright (c) 2017, Alliance for Open Media. All rights reserved.
 *
 * This source code is subject to the terms of the BSD 2 Clause License and
 * the Alliance for Open Media Patent License 1.0. If the BSD 2 Clause License
 * was not distributed with this source code in the LICENSE file, you can
 * obtain it at www.aomedia.org/license/software. If the Alliance for Open
 * Media Patent License 1.0 was not distributed with this source code in the
 * PATENTS file, you can obtain it at www.aomedia.org/license/patent.
 */

// Translated from upstream/aom/aom_dsp/x86/intrapred_ssse3.c (ADR 0056, ADR 0058).
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

use crate::simd::aom_dsp__x86__intrapred_utils::*;
use crate::simd::aom_dsp__x86__intrapred_x86::*;
use crate::simd::aom_dsp__x86__lpf_common_sse2::*;
use crate::simd::aom_dsp__x86__synonyms::*;
use core::arch::x86_64::*;

// From aom_dsp/intrapred_common.h.
const SMOOTH_WEIGHT_LOG2_SCALE: i32 = 8;

// Weights are quadratic from '1' to '1 / block_size', scaled by
// 2^SMOOTH_WEIGHT_LOG2_SCALE.
// Note these arrays are aligned to ensure NEON loads using a cast to uint32_t*
// have sufficient alignment. Using 8 preserves the potential for an alignment
// hint in load_weight_w8(). For that case, this could be increased to 16 to
// allow an aligned load in x86.
static smooth_weights: Aligned8<[u8; 124]> = Aligned8([
    255, 149, 85, 64, 255, 197, 146, 105, 73, 50, 37, 32, 255, 225, 196, 170, 145, 123, 102, 84,
    68, 54, 43, 33, 26, 20, 17, 16, 255, 240, 225, 210, 196, 182, 169, 157, 145, 133, 122, 111,
    101, 92, 83, 74, 66, 59, 52, 45, 39, 34, 29, 25, 21, 17, 14, 12, 10, 9, 8, 8, 255, 248, 240,
    233, 225, 218, 210, 203, 196, 189, 182, 176, 169, 163, 156, 150, 144, 138, 133, 127, 121, 116,
    111, 106, 101, 96, 91, 86, 82, 77, 73, 69, 65, 61, 57, 54, 50, 47, 44, 41, 38, 35, 32, 29, 27,
    25, 22, 20, 18, 16, 15, 13, 12, 10, 9, 8, 7, 6, 6, 5, 5, 4, 4, 4,
]);

// bs = 4
// bs = 8
// bs = 16
// bs = 32
// bs = 64
static smooth_weights_u16: Aligned8<[u16; 124]> = Aligned8([
    255, 149, 85, 64, 255, 197, 146, 105, 73, 50, 37, 32, 255, 225, 196, 170, 145, 123, 102, 84,
    68, 54, 43, 33, 26, 20, 17, 16, 255, 240, 225, 210, 196, 182, 169, 157, 145, 133, 122, 111,
    101, 92, 83, 74, 66, 59, 52, 45, 39, 34, 29, 25, 21, 17, 14, 12, 10, 9, 8, 8, 255, 248, 240,
    233, 225, 218, 210, 203, 196, 189, 182, 176, 169, 163, 156, 150, 144, 138, 133, 127, 121, 116,
    111, 106, 101, 96, 91, 86, 82, 77, 73, 69, 65, 61, 57, 54, 50, 47, 44, 41, 38, 35, 32, 29, 27,
    25, 22, 20, 18, 16, 15, 13, 12, 10, 9, 8, 7, 6, 6, 5, 5, 4, 4, 4,
]);

// block dimension = 4
// block dimension = 8
// block dimension = 16
// block dimension = 32
// block dimension = 64
// AOM_AOM_DSP_INTRAPRED_COMMON_H_

// -----------------------------------------------------------------------------
// PAETH_PRED
// Return 8 16-bit pixels in one row
#[inline]
#[target_feature(enable = "ssse3")]
unsafe fn paeth_8x1_pred(
    left: *const __m128i,
    top: *const __m128i,
    topleft: *const __m128i,
) -> __m128i {
    let base: __m128i = _mm_sub_epi16(_mm_add_epi16(*top, *left), *topleft);
    let mut pl: __m128i = _mm_abs_epi16(_mm_sub_epi16(base, *left));
    let mut pt: __m128i = _mm_abs_epi16(_mm_sub_epi16(base, *top));
    let mut ptl: __m128i = _mm_abs_epi16(_mm_sub_epi16(base, *topleft));
    let mut mask1: __m128i = _mm_cmpgt_epi16(pl, pt);
    mask1 = _mm_or_si128(mask1, _mm_cmpgt_epi16(pl, ptl));
    let mut mask2: __m128i = _mm_cmpgt_epi16(pt, ptl);
    pl = _mm_andnot_si128(mask1, *left);
    ptl = _mm_and_si128(mask2, *topleft);
    pt = _mm_andnot_si128(mask2, *top);
    pt = _mm_or_si128(pt, ptl);
    pt = _mm_and_si128(mask1, pt);
    return _mm_or_si128(pl, pt);
}

#[no_mangle]
#[target_feature(enable = "ssse3")]
pub unsafe extern "C-unwind" fn aom_paeth_predictor_4x4_ssse3(
    mut dst: *mut u8,
    stride: isize,
    above: *const u8,
    left: *const u8,
) {
    let mut l: __m128i = _mm_loadl_epi64(left as *const __m128i);
    let t: __m128i = _mm_loadl_epi64(above as *const __m128i);
    let zero: __m128i = _mm_setzero_si128();
    let t16: __m128i = _mm_unpacklo_epi8(t, zero);
    let tl16: __m128i = _mm_set1_epi16(*above.offset(-1) as i16);
    let mut rep: __m128i = _mm_set1_epi16(0x8000_i32 as i16);
    let one: __m128i = _mm_set1_epi16(1);
    let mut i: i32 = 0;
    {
        i = 0;
        while i < 4 {
            let l16: __m128i = _mm_shuffle_epi8(l, rep);
            let row: __m128i = paeth_8x1_pred(&raw const l16, &raw const t16, &raw const tl16);
            core::ptr::write_unaligned(
                dst as *mut i32,
                _mm_cvtsi128_si32(_mm_packus_epi16(row, row)),
            );
            dst = dst.offset(stride);
            rep = _mm_add_epi16(rep, one);
            i += 1;
        }
    }
}

#[no_mangle]
#[target_feature(enable = "ssse3")]
pub unsafe extern "C-unwind" fn aom_paeth_predictor_4x8_ssse3(
    mut dst: *mut u8,
    stride: isize,
    above: *const u8,
    left: *const u8,
) {
    let mut l: __m128i = _mm_loadl_epi64(left as *const __m128i);
    let t: __m128i = _mm_loadl_epi64(above as *const __m128i);
    let zero: __m128i = _mm_setzero_si128();
    let t16: __m128i = _mm_unpacklo_epi8(t, zero);
    let tl16: __m128i = _mm_set1_epi16(*above.offset(-1) as i16);
    let mut rep: __m128i = _mm_set1_epi16(0x8000_i32 as i16);
    let one: __m128i = _mm_set1_epi16(1);
    let mut i: i32 = 0;
    {
        i = 0;
        while i < 8 {
            let l16: __m128i = _mm_shuffle_epi8(l, rep);
            let row: __m128i = paeth_8x1_pred(&raw const l16, &raw const t16, &raw const tl16);
            core::ptr::write_unaligned(
                dst as *mut i32,
                _mm_cvtsi128_si32(_mm_packus_epi16(row, row)),
            );
            dst = dst.offset(stride);
            rep = _mm_add_epi16(rep, one);
            i += 1;
        }
    }
}

#[no_mangle]
#[target_feature(enable = "ssse3")]
pub unsafe extern "C-unwind" fn aom_paeth_predictor_4x16_ssse3(
    mut dst: *mut u8,
    stride: isize,
    above: *const u8,
    left: *const u8,
) {
    let mut l: __m128i = _mm_load_si128(left as *const __m128i);
    let t: __m128i = _mm_cvtsi32_si128(core::ptr::read_unaligned(above as *const i32));
    let zero: __m128i = _mm_setzero_si128();
    let t16: __m128i = _mm_unpacklo_epi8(t, zero);
    let tl16: __m128i = _mm_set1_epi16(*above.offset(-1) as i16);
    let mut rep: __m128i = _mm_set1_epi16(0x8000_i32 as i16);
    let one: __m128i = _mm_set1_epi16(1);
    for i in 0..16 {
        let l16: __m128i = _mm_shuffle_epi8(l, rep);
        let row: __m128i = paeth_8x1_pred(&raw const l16, &raw const t16, &raw const tl16);
        core::ptr::write_unaligned(
            dst as *mut i32,
            _mm_cvtsi128_si32(_mm_packus_epi16(row, row)),
        );
        dst = dst.offset(stride);
        rep = _mm_add_epi16(rep, one);
    }
}

// !CONFIG_REALTIME_ONLY || CONFIG_AV1_DECODER
#[no_mangle]
#[target_feature(enable = "ssse3")]
pub unsafe extern "C-unwind" fn aom_paeth_predictor_8x4_ssse3(
    mut dst: *mut u8,
    stride: isize,
    above: *const u8,
    left: *const u8,
) {
    let mut l: __m128i = _mm_loadl_epi64(left as *const __m128i);
    let t: __m128i = _mm_loadl_epi64(above as *const __m128i);
    let zero: __m128i = _mm_setzero_si128();
    let t16: __m128i = _mm_unpacklo_epi8(t, zero);
    let tl16: __m128i = _mm_set1_epi16(*above.offset(-1) as i16);
    let mut rep: __m128i = _mm_set1_epi16(0x8000_i32 as i16);
    let one: __m128i = _mm_set1_epi16(1);
    let mut i: i32 = 0;
    {
        i = 0;
        while i < 4 {
            let l16: __m128i = _mm_shuffle_epi8(l, rep);
            let row: __m128i = paeth_8x1_pred(&raw const l16, &raw const t16, &raw const tl16);
            _mm_storel_epi64(dst as *mut __m128i, _mm_packus_epi16(row, row));
            dst = dst.offset(stride);
            rep = _mm_add_epi16(rep, one);
            i += 1;
        }
    }
}

#[no_mangle]
#[target_feature(enable = "ssse3")]
pub unsafe extern "C-unwind" fn aom_paeth_predictor_8x8_ssse3(
    mut dst: *mut u8,
    stride: isize,
    above: *const u8,
    left: *const u8,
) {
    let mut l: __m128i = _mm_loadl_epi64(left as *const __m128i);
    let t: __m128i = _mm_loadl_epi64(above as *const __m128i);
    let zero: __m128i = _mm_setzero_si128();
    let t16: __m128i = _mm_unpacklo_epi8(t, zero);
    let tl16: __m128i = _mm_set1_epi16(*above.offset(-1) as i16);
    let mut rep: __m128i = _mm_set1_epi16(0x8000_i32 as i16);
    let one: __m128i = _mm_set1_epi16(1);
    let mut i: i32 = 0;
    {
        i = 0;
        while i < 8 {
            let l16: __m128i = _mm_shuffle_epi8(l, rep);
            let row: __m128i = paeth_8x1_pred(&raw const l16, &raw const t16, &raw const tl16);
            _mm_storel_epi64(dst as *mut __m128i, _mm_packus_epi16(row, row));
            dst = dst.offset(stride);
            rep = _mm_add_epi16(rep, one);
            i += 1;
        }
    }
}

#[no_mangle]
#[target_feature(enable = "ssse3")]
pub unsafe extern "C-unwind" fn aom_paeth_predictor_8x16_ssse3(
    mut dst: *mut u8,
    stride: isize,
    above: *const u8,
    left: *const u8,
) {
    let mut l: __m128i = _mm_load_si128(left as *const __m128i);
    let t: __m128i = _mm_loadl_epi64(above as *const __m128i);
    let zero: __m128i = _mm_setzero_si128();
    let t16: __m128i = _mm_unpacklo_epi8(t, zero);
    let tl16: __m128i = _mm_set1_epi16(*above.offset(-1) as i16);
    let mut rep: __m128i = _mm_set1_epi16(0x8000_i32 as i16);
    let one: __m128i = _mm_set1_epi16(1);
    let mut i: i32 = 0;
    {
        i = 0;
        while i < 16 {
            let l16: __m128i = _mm_shuffle_epi8(l, rep);
            let row: __m128i = paeth_8x1_pred(&raw const l16, &raw const t16, &raw const tl16);
            _mm_storel_epi64(dst as *mut __m128i, _mm_packus_epi16(row, row));
            dst = dst.offset(stride);
            rep = _mm_add_epi16(rep, one);
            i += 1;
        }
    }
}

#[no_mangle]
#[target_feature(enable = "ssse3")]
pub unsafe extern "C-unwind" fn aom_paeth_predictor_8x32_ssse3(
    mut dst: *mut u8,
    stride: isize,
    above: *const u8,
    left: *const u8,
) {
    let t: __m128i = _mm_loadl_epi64(above as *const __m128i);
    let zero: __m128i = _mm_setzero_si128();
    let t16: __m128i = _mm_unpacklo_epi8(t, zero);
    let tl16: __m128i = _mm_set1_epi16(*above.offset(-1) as i16);
    let one: __m128i = _mm_set1_epi16(1);
    for j in 0..2 {
        let l: __m128i = _mm_load_si128(left.offset((j * 16) as isize) as *const __m128i);
        let mut rep: __m128i = _mm_set1_epi16(0x8000_i32 as i16);
        for i in 0..16 {
            let l16: __m128i = _mm_shuffle_epi8(l, rep);
            let row: __m128i = paeth_8x1_pred(&raw const l16, &raw const t16, &raw const tl16);
            _mm_storel_epi64(dst as *mut __m128i, _mm_packus_epi16(row, row));
            dst = dst.offset(stride);
            rep = _mm_add_epi16(rep, one);
        }
    }
}

// !CONFIG_REALTIME_ONLY || CONFIG_AV1_DECODER
// Return 16 8-bit pixels in one row
#[inline]
#[target_feature(enable = "ssse3")]
unsafe fn paeth_16x1_pred(
    left: *const __m128i,
    top0: *const __m128i,
    top1: *const __m128i,
    topleft: *const __m128i,
) -> __m128i {
    let p0: __m128i = paeth_8x1_pred(left, top0, topleft);
    let p1: __m128i = paeth_8x1_pred(left, top1, topleft);
    return _mm_packus_epi16(p0, p1);
}

#[no_mangle]
#[target_feature(enable = "ssse3")]
pub unsafe extern "C-unwind" fn aom_paeth_predictor_16x4_ssse3(
    mut dst: *mut u8,
    stride: isize,
    above: *const u8,
    left: *const u8,
) {
    let mut l: __m128i = _mm_cvtsi32_si128(core::ptr::read_unaligned(left as *const i32));
    let t: __m128i = _mm_load_si128(above as *const __m128i);
    let zero: __m128i = _mm_setzero_si128();
    let top0: __m128i = _mm_unpacklo_epi8(t, zero);
    let top1: __m128i = _mm_unpackhi_epi8(t, zero);
    let tl16: __m128i = _mm_set1_epi16(*above.offset(-1) as i16);
    let mut rep: __m128i = _mm_set1_epi16(0x8000_i32 as i16);
    let one: __m128i = _mm_set1_epi16(1);
    for i in 0..4 {
        let l16: __m128i = _mm_shuffle_epi8(l, rep);
        let row: __m128i = paeth_16x1_pred(
            &raw const l16,
            &raw const top0,
            &raw const top1,
            &raw const tl16,
        );
        _mm_store_si128(dst as *mut __m128i, row);
        dst = dst.offset(stride);
        rep = _mm_add_epi16(rep, one);
    }
}

// !CONFIG_REALTIME_ONLY || CONFIG_AV1_DECODER
#[no_mangle]
#[target_feature(enable = "ssse3")]
pub unsafe extern "C-unwind" fn aom_paeth_predictor_16x8_ssse3(
    mut dst: *mut u8,
    stride: isize,
    above: *const u8,
    left: *const u8,
) {
    let mut l: __m128i = _mm_loadl_epi64(left as *const __m128i);
    let t: __m128i = _mm_load_si128(above as *const __m128i);
    let zero: __m128i = _mm_setzero_si128();
    let top0: __m128i = _mm_unpacklo_epi8(t, zero);
    let top1: __m128i = _mm_unpackhi_epi8(t, zero);
    let tl16: __m128i = _mm_set1_epi16(*above.offset(-1) as i16);
    let mut rep: __m128i = _mm_set1_epi16(0x8000_i32 as i16);
    let one: __m128i = _mm_set1_epi16(1);
    let mut i: i32 = 0;
    {
        i = 0;
        while i < 8 {
            let l16: __m128i = _mm_shuffle_epi8(l, rep);
            let row: __m128i = paeth_16x1_pred(
                &raw const l16,
                &raw const top0,
                &raw const top1,
                &raw const tl16,
            );
            _mm_store_si128(dst as *mut __m128i, row);
            dst = dst.offset(stride);
            rep = _mm_add_epi16(rep, one);
            i += 1;
        }
    }
}

#[no_mangle]
#[target_feature(enable = "ssse3")]
pub unsafe extern "C-unwind" fn aom_paeth_predictor_16x16_ssse3(
    mut dst: *mut u8,
    stride: isize,
    above: *const u8,
    left: *const u8,
) {
    let mut l: __m128i = _mm_load_si128(left as *const __m128i);
    let t: __m128i = _mm_load_si128(above as *const __m128i);
    let zero: __m128i = _mm_setzero_si128();
    let top0: __m128i = _mm_unpacklo_epi8(t, zero);
    let top1: __m128i = _mm_unpackhi_epi8(t, zero);
    let tl16: __m128i = _mm_set1_epi16(*above.offset(-1) as i16);
    let mut rep: __m128i = _mm_set1_epi16(0x8000_i32 as i16);
    let one: __m128i = _mm_set1_epi16(1);
    let mut i: i32 = 0;
    {
        i = 0;
        while i < 16 {
            let l16: __m128i = _mm_shuffle_epi8(l, rep);
            let row: __m128i = paeth_16x1_pred(
                &raw const l16,
                &raw const top0,
                &raw const top1,
                &raw const tl16,
            );
            _mm_store_si128(dst as *mut __m128i, row);
            dst = dst.offset(stride);
            rep = _mm_add_epi16(rep, one);
            i += 1;
        }
    }
}

#[no_mangle]
#[target_feature(enable = "ssse3")]
pub unsafe extern "C-unwind" fn aom_paeth_predictor_16x32_ssse3(
    mut dst: *mut u8,
    stride: isize,
    above: *const u8,
    left: *const u8,
) {
    let mut l: __m128i = _mm_load_si128(left as *const __m128i);
    let t: __m128i = _mm_load_si128(above as *const __m128i);
    let zero: __m128i = _mm_setzero_si128();
    let top0: __m128i = _mm_unpacklo_epi8(t, zero);
    let top1: __m128i = _mm_unpackhi_epi8(t, zero);
    let tl16: __m128i = _mm_set1_epi16(*above.offset(-1) as i16);
    let mut rep: __m128i = _mm_set1_epi16(0x8000_i32 as i16);
    let one: __m128i = _mm_set1_epi16(1);
    let mut l16: __m128i = core::mem::zeroed();
    let mut i: i32 = 0;
    {
        i = 0;
        while i < 16 {
            l16 = _mm_shuffle_epi8(l, rep);
            let row: __m128i = paeth_16x1_pred(
                &raw mut l16 as *const __m128i,
                &raw const top0,
                &raw const top1,
                &raw const tl16,
            );
            _mm_store_si128(dst as *mut __m128i, row);
            dst = dst.offset(stride);
            rep = _mm_add_epi16(rep, one);
            i += 1;
        }
    }
    l = _mm_load_si128(left.offset(16) as *const __m128i);
    rep = _mm_set1_epi16(0x8000_i32 as i16);
    {
        i = 0;
        while i < 16 {
            l16 = _mm_shuffle_epi8(l, rep);
            let row: __m128i = paeth_16x1_pred(
                &raw mut l16 as *const __m128i,
                &raw const top0,
                &raw const top1,
                &raw const tl16,
            );
            _mm_store_si128(dst as *mut __m128i, row);
            dst = dst.offset(stride);
            rep = _mm_add_epi16(rep, one);
            i += 1;
        }
    }
}

#[no_mangle]
#[target_feature(enable = "ssse3")]
pub unsafe extern "C-unwind" fn aom_paeth_predictor_16x64_ssse3(
    mut dst: *mut u8,
    stride: isize,
    above: *const u8,
    left: *const u8,
) {
    let t: __m128i = _mm_load_si128(above as *const __m128i);
    let zero: __m128i = _mm_setzero_si128();
    let top0: __m128i = _mm_unpacklo_epi8(t, zero);
    let top1: __m128i = _mm_unpackhi_epi8(t, zero);
    let tl16: __m128i = _mm_set1_epi16(*above.offset(-1) as i16);
    let one: __m128i = _mm_set1_epi16(1);
    for j in 0..4 {
        let l: __m128i = _mm_load_si128(left.offset((j * 16) as isize) as *const __m128i);
        let mut rep: __m128i = _mm_set1_epi16(0x8000_i32 as i16);
        for i in 0..16 {
            let l16: __m128i = _mm_shuffle_epi8(l, rep);
            let row: __m128i = paeth_16x1_pred(
                &raw const l16,
                &raw const top0,
                &raw const top1,
                &raw const tl16,
            );
            _mm_store_si128(dst as *mut __m128i, row);
            dst = dst.offset(stride);
            rep = _mm_add_epi16(rep, one);
        }
    }
}

#[no_mangle]
#[target_feature(enable = "ssse3")]
pub unsafe extern "C-unwind" fn aom_paeth_predictor_32x8_ssse3(
    mut dst: *mut u8,
    stride: isize,
    above: *const u8,
    left: *const u8,
) {
    let a: __m128i = _mm_load_si128(above as *const __m128i);
    let b: __m128i = _mm_load_si128(above.offset(16) as *const __m128i);
    let zero: __m128i = _mm_setzero_si128();
    let al: __m128i = _mm_unpacklo_epi8(a, zero);
    let ah: __m128i = _mm_unpackhi_epi8(a, zero);
    let bl: __m128i = _mm_unpacklo_epi8(b, zero);
    let bh: __m128i = _mm_unpackhi_epi8(b, zero);
    let tl16: __m128i = _mm_set1_epi16(*above.offset(-1) as i16);
    let mut rep: __m128i = _mm_set1_epi16(0x8000_i32 as i16);
    let one: __m128i = _mm_set1_epi16(1);
    let l: __m128i = _mm_loadl_epi64(left as *const __m128i);
    let mut l16: __m128i = core::mem::zeroed();
    for i in 0..8 {
        l16 = _mm_shuffle_epi8(l, rep);
        let r32l: __m128i = paeth_16x1_pred(
            &raw mut l16 as *const __m128i,
            &raw const al,
            &raw const ah,
            &raw const tl16,
        );
        let r32h: __m128i = paeth_16x1_pred(
            &raw mut l16 as *const __m128i,
            &raw const bl,
            &raw const bh,
            &raw const tl16,
        );
        _mm_store_si128(dst as *mut __m128i, r32l);
        _mm_store_si128(dst.offset(16) as *mut __m128i, r32h);
        dst = dst.offset(stride);
        rep = _mm_add_epi16(rep, one);
    }
}

// !CONFIG_REALTIME_ONLY || CONFIG_AV1_DECODER
#[no_mangle]
#[target_feature(enable = "ssse3")]
pub unsafe extern "C-unwind" fn aom_paeth_predictor_32x16_ssse3(
    mut dst: *mut u8,
    stride: isize,
    above: *const u8,
    left: *const u8,
) {
    let a: __m128i = _mm_load_si128(above as *const __m128i);
    let b: __m128i = _mm_load_si128(above.offset(16) as *const __m128i);
    let zero: __m128i = _mm_setzero_si128();
    let al: __m128i = _mm_unpacklo_epi8(a, zero);
    let ah: __m128i = _mm_unpackhi_epi8(a, zero);
    let bl: __m128i = _mm_unpacklo_epi8(b, zero);
    let bh: __m128i = _mm_unpackhi_epi8(b, zero);
    let tl16: __m128i = _mm_set1_epi16(*above.offset(-1) as i16);
    let mut rep: __m128i = _mm_set1_epi16(0x8000_i32 as i16);
    let one: __m128i = _mm_set1_epi16(1);
    let mut l: __m128i = _mm_load_si128(left as *const __m128i);
    let mut l16: __m128i = core::mem::zeroed();
    let mut i: i32 = 0;
    {
        i = 0;
        while i < 16 {
            l16 = _mm_shuffle_epi8(l, rep);
            let r32l: __m128i = paeth_16x1_pred(
                &raw mut l16 as *const __m128i,
                &raw const al,
                &raw const ah,
                &raw const tl16,
            );
            let r32h: __m128i = paeth_16x1_pred(
                &raw mut l16 as *const __m128i,
                &raw const bl,
                &raw const bh,
                &raw const tl16,
            );
            _mm_store_si128(dst as *mut __m128i, r32l);
            _mm_store_si128(dst.offset(16) as *mut __m128i, r32h);
            dst = dst.offset(stride);
            rep = _mm_add_epi16(rep, one);
            i += 1;
        }
    }
}

#[no_mangle]
#[target_feature(enable = "ssse3")]
pub unsafe extern "C-unwind" fn aom_paeth_predictor_32x32_ssse3(
    mut dst: *mut u8,
    stride: isize,
    above: *const u8,
    left: *const u8,
) {
    let a: __m128i = _mm_load_si128(above as *const __m128i);
    let b: __m128i = _mm_load_si128(above.offset(16) as *const __m128i);
    let zero: __m128i = _mm_setzero_si128();
    let al: __m128i = _mm_unpacklo_epi8(a, zero);
    let ah: __m128i = _mm_unpackhi_epi8(a, zero);
    let bl: __m128i = _mm_unpacklo_epi8(b, zero);
    let bh: __m128i = _mm_unpackhi_epi8(b, zero);
    let tl16: __m128i = _mm_set1_epi16(*above.offset(-1) as i16);
    let mut rep: __m128i = _mm_set1_epi16(0x8000_i32 as i16);
    let one: __m128i = _mm_set1_epi16(1);
    let mut l: __m128i = _mm_load_si128(left as *const __m128i);
    let mut l16: __m128i = core::mem::zeroed();
    let mut i: i32 = 0;
    {
        i = 0;
        while i < 16 {
            l16 = _mm_shuffle_epi8(l, rep);
            let r32l: __m128i = paeth_16x1_pred(
                &raw mut l16 as *const __m128i,
                &raw const al,
                &raw const ah,
                &raw const tl16,
            );
            let r32h: __m128i = paeth_16x1_pred(
                &raw mut l16 as *const __m128i,
                &raw const bl,
                &raw const bh,
                &raw const tl16,
            );
            _mm_store_si128(dst as *mut __m128i, r32l);
            _mm_store_si128(dst.offset(16) as *mut __m128i, r32h);
            dst = dst.offset(stride);
            rep = _mm_add_epi16(rep, one);
            i += 1;
        }
    }
    rep = _mm_set1_epi16(0x8000_i32 as i16);
    l = _mm_load_si128(left.offset(16) as *const __m128i);
    {
        i = 0;
        while i < 16 {
            l16 = _mm_shuffle_epi8(l, rep);
            let r32l: __m128i = paeth_16x1_pred(
                &raw mut l16 as *const __m128i,
                &raw const al,
                &raw const ah,
                &raw const tl16,
            );
            let r32h: __m128i = paeth_16x1_pred(
                &raw mut l16 as *const __m128i,
                &raw const bl,
                &raw const bh,
                &raw const tl16,
            );
            _mm_store_si128(dst as *mut __m128i, r32l);
            _mm_store_si128(dst.offset(16) as *mut __m128i, r32h);
            dst = dst.offset(stride);
            rep = _mm_add_epi16(rep, one);
            i += 1;
        }
    }
}

#[no_mangle]
#[target_feature(enable = "ssse3")]
pub unsafe extern "C-unwind" fn aom_paeth_predictor_32x64_ssse3(
    mut dst: *mut u8,
    stride: isize,
    above: *const u8,
    left: *const u8,
) {
    let a: __m128i = _mm_load_si128(above as *const __m128i);
    let b: __m128i = _mm_load_si128(above.offset(16) as *const __m128i);
    let zero: __m128i = _mm_setzero_si128();
    let al: __m128i = _mm_unpacklo_epi8(a, zero);
    let ah: __m128i = _mm_unpackhi_epi8(a, zero);
    let bl: __m128i = _mm_unpacklo_epi8(b, zero);
    let bh: __m128i = _mm_unpackhi_epi8(b, zero);
    let tl16: __m128i = _mm_set1_epi16(*above.offset(-1) as i16);
    let one: __m128i = _mm_set1_epi16(1);
    let mut l16: __m128i = core::mem::zeroed();
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    {
        j = 0;
        while j < 4 {
            let l: __m128i = _mm_load_si128(left.offset((j * 16) as isize) as *const __m128i);
            let mut rep: __m128i = _mm_set1_epi16(0x8000_i32 as i16);
            {
                i = 0;
                while i < 16 {
                    l16 = _mm_shuffle_epi8(l, rep);
                    let r32l: __m128i = paeth_16x1_pred(
                        &raw mut l16 as *const __m128i,
                        &raw const al,
                        &raw const ah,
                        &raw const tl16,
                    );
                    let r32h: __m128i = paeth_16x1_pred(
                        &raw mut l16 as *const __m128i,
                        &raw const bl,
                        &raw const bh,
                        &raw const tl16,
                    );
                    _mm_store_si128(dst as *mut __m128i, r32l);
                    _mm_store_si128(dst.offset(16) as *mut __m128i, r32h);
                    dst = dst.offset(stride);
                    rep = _mm_add_epi16(rep, one);
                    i += 1;
                }
            }
            j += 1;
        }
    }
}

#[no_mangle]
#[target_feature(enable = "ssse3")]
pub unsafe extern "C-unwind" fn aom_paeth_predictor_64x32_ssse3(
    mut dst: *mut u8,
    stride: isize,
    above: *const u8,
    left: *const u8,
) {
    let a: __m128i = _mm_load_si128(above as *const __m128i);
    let b: __m128i = _mm_load_si128(above.offset(16) as *const __m128i);
    let c: __m128i = _mm_load_si128(above.offset(32) as *const __m128i);
    let d: __m128i = _mm_load_si128(above.offset(48) as *const __m128i);
    let zero: __m128i = _mm_setzero_si128();
    let al: __m128i = _mm_unpacklo_epi8(a, zero);
    let ah: __m128i = _mm_unpackhi_epi8(a, zero);
    let bl: __m128i = _mm_unpacklo_epi8(b, zero);
    let bh: __m128i = _mm_unpackhi_epi8(b, zero);
    let cl: __m128i = _mm_unpacklo_epi8(c, zero);
    let ch: __m128i = _mm_unpackhi_epi8(c, zero);
    let dl: __m128i = _mm_unpacklo_epi8(d, zero);
    let dh: __m128i = _mm_unpackhi_epi8(d, zero);
    let tl16: __m128i = _mm_set1_epi16(*above.offset(-1) as i16);
    let one: __m128i = _mm_set1_epi16(1);
    let mut l16: __m128i = core::mem::zeroed();
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    {
        j = 0;
        while j < 2 {
            let l: __m128i = _mm_load_si128(left.offset((j * 16) as isize) as *const __m128i);
            let mut rep: __m128i = _mm_set1_epi16(0x8000_i32 as i16);
            {
                i = 0;
                while i < 16 {
                    l16 = _mm_shuffle_epi8(l, rep);
                    let r0: __m128i = paeth_16x1_pred(
                        &raw mut l16 as *const __m128i,
                        &raw const al,
                        &raw const ah,
                        &raw const tl16,
                    );
                    let r1: __m128i = paeth_16x1_pred(
                        &raw mut l16 as *const __m128i,
                        &raw const bl,
                        &raw const bh,
                        &raw const tl16,
                    );
                    let r2: __m128i = paeth_16x1_pred(
                        &raw mut l16 as *const __m128i,
                        &raw const cl,
                        &raw const ch,
                        &raw const tl16,
                    );
                    let r3: __m128i = paeth_16x1_pred(
                        &raw mut l16 as *const __m128i,
                        &raw const dl,
                        &raw const dh,
                        &raw const tl16,
                    );
                    _mm_store_si128(dst as *mut __m128i, r0);
                    _mm_store_si128(dst.offset(16) as *mut __m128i, r1);
                    _mm_store_si128(dst.offset(32) as *mut __m128i, r2);
                    _mm_store_si128(dst.offset(48) as *mut __m128i, r3);
                    dst = dst.offset(stride);
                    rep = _mm_add_epi16(rep, one);
                    i += 1;
                }
            }
            j += 1;
        }
    }
}

#[no_mangle]
#[target_feature(enable = "ssse3")]
pub unsafe extern "C-unwind" fn aom_paeth_predictor_64x64_ssse3(
    mut dst: *mut u8,
    stride: isize,
    above: *const u8,
    left: *const u8,
) {
    let a: __m128i = _mm_load_si128(above as *const __m128i);
    let b: __m128i = _mm_load_si128(above.offset(16) as *const __m128i);
    let c: __m128i = _mm_load_si128(above.offset(32) as *const __m128i);
    let d: __m128i = _mm_load_si128(above.offset(48) as *const __m128i);
    let zero: __m128i = _mm_setzero_si128();
    let al: __m128i = _mm_unpacklo_epi8(a, zero);
    let ah: __m128i = _mm_unpackhi_epi8(a, zero);
    let bl: __m128i = _mm_unpacklo_epi8(b, zero);
    let bh: __m128i = _mm_unpackhi_epi8(b, zero);
    let cl: __m128i = _mm_unpacklo_epi8(c, zero);
    let ch: __m128i = _mm_unpackhi_epi8(c, zero);
    let dl: __m128i = _mm_unpacklo_epi8(d, zero);
    let dh: __m128i = _mm_unpackhi_epi8(d, zero);
    let tl16: __m128i = _mm_set1_epi16(*above.offset(-1) as i16);
    let one: __m128i = _mm_set1_epi16(1);
    let mut l16: __m128i = core::mem::zeroed();
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    {
        j = 0;
        while j < 4 {
            let l: __m128i = _mm_load_si128(left.offset((j * 16) as isize) as *const __m128i);
            let mut rep: __m128i = _mm_set1_epi16(0x8000_i32 as i16);
            {
                i = 0;
                while i < 16 {
                    l16 = _mm_shuffle_epi8(l, rep);
                    let r0: __m128i = paeth_16x1_pred(
                        &raw mut l16 as *const __m128i,
                        &raw const al,
                        &raw const ah,
                        &raw const tl16,
                    );
                    let r1: __m128i = paeth_16x1_pred(
                        &raw mut l16 as *const __m128i,
                        &raw const bl,
                        &raw const bh,
                        &raw const tl16,
                    );
                    let r2: __m128i = paeth_16x1_pred(
                        &raw mut l16 as *const __m128i,
                        &raw const cl,
                        &raw const ch,
                        &raw const tl16,
                    );
                    let r3: __m128i = paeth_16x1_pred(
                        &raw mut l16 as *const __m128i,
                        &raw const dl,
                        &raw const dh,
                        &raw const tl16,
                    );
                    _mm_store_si128(dst as *mut __m128i, r0);
                    _mm_store_si128(dst.offset(16) as *mut __m128i, r1);
                    _mm_store_si128(dst.offset(32) as *mut __m128i, r2);
                    _mm_store_si128(dst.offset(48) as *mut __m128i, r3);
                    dst = dst.offset(stride);
                    rep = _mm_add_epi16(rep, one);
                    i += 1;
                }
            }
            j += 1;
        }
    }
}

#[no_mangle]
#[target_feature(enable = "ssse3")]
pub unsafe extern "C-unwind" fn aom_paeth_predictor_64x16_ssse3(
    mut dst: *mut u8,
    stride: isize,
    above: *const u8,
    left: *const u8,
) {
    let a: __m128i = _mm_load_si128(above as *const __m128i);
    let b: __m128i = _mm_load_si128(above.offset(16) as *const __m128i);
    let c: __m128i = _mm_load_si128(above.offset(32) as *const __m128i);
    let d: __m128i = _mm_load_si128(above.offset(48) as *const __m128i);
    let zero: __m128i = _mm_setzero_si128();
    let al: __m128i = _mm_unpacklo_epi8(a, zero);
    let ah: __m128i = _mm_unpackhi_epi8(a, zero);
    let bl: __m128i = _mm_unpacklo_epi8(b, zero);
    let bh: __m128i = _mm_unpackhi_epi8(b, zero);
    let cl: __m128i = _mm_unpacklo_epi8(c, zero);
    let ch: __m128i = _mm_unpackhi_epi8(c, zero);
    let dl: __m128i = _mm_unpacklo_epi8(d, zero);
    let dh: __m128i = _mm_unpackhi_epi8(d, zero);
    let tl16: __m128i = _mm_set1_epi16(*above.offset(-1) as i16);
    let one: __m128i = _mm_set1_epi16(1);
    let mut l16: __m128i = core::mem::zeroed();
    let mut i: i32 = 0;
    let l: __m128i = _mm_load_si128(left as *const __m128i);
    let mut rep: __m128i = _mm_set1_epi16(0x8000_i32 as i16);
    {
        i = 0;
        while i < 16 {
            l16 = _mm_shuffle_epi8(l, rep);
            let r0: __m128i = paeth_16x1_pred(
                &raw mut l16 as *const __m128i,
                &raw const al,
                &raw const ah,
                &raw const tl16,
            );
            let r1: __m128i = paeth_16x1_pred(
                &raw mut l16 as *const __m128i,
                &raw const bl,
                &raw const bh,
                &raw const tl16,
            );
            let r2: __m128i = paeth_16x1_pred(
                &raw mut l16 as *const __m128i,
                &raw const cl,
                &raw const ch,
                &raw const tl16,
            );
            let r3: __m128i = paeth_16x1_pred(
                &raw mut l16 as *const __m128i,
                &raw const dl,
                &raw const dh,
                &raw const tl16,
            );
            _mm_store_si128(dst as *mut __m128i, r0);
            _mm_store_si128(dst.offset(16) as *mut __m128i, r1);
            _mm_store_si128(dst.offset(32) as *mut __m128i, r2);
            _mm_store_si128(dst.offset(48) as *mut __m128i, r3);
            dst = dst.offset(stride);
            rep = _mm_add_epi16(rep, one);
            i += 1;
        }
    }
}

// !CONFIG_REALTIME_ONLY || CONFIG_AV1_DECODER
// -----------------------------------------------------------------------------
// SMOOTH_PRED
// pixels[0]: above and below_pred interleave vector
// pixels[1]: left vector
// pixels[2]: right_pred vector
#[inline]
#[target_feature(enable = "ssse3")]
unsafe fn load_pixel_w4(above: *const u8, left: *const u8, height: i32, pixels: *mut __m128i) {
    let mut d: __m128i = _mm_cvtsi32_si128(core::ptr::read_unaligned(above as *const i32));
    if height == 4 {
        *pixels.offset(1) = _mm_cvtsi32_si128(core::ptr::read_unaligned(left as *const i32));
    } else if height == 8 {
        *pixels.offset(1) = _mm_loadl_epi64(left as *const __m128i);
    } else {
        *pixels.offset(1) = _mm_loadu_si128(left as *const __m128i);
    }
    *pixels.offset(2) = _mm_set1_epi16(*above.offset(3) as i16);
    let bp: __m128i = _mm_set1_epi16(*left.offset((height - 1) as isize) as i16);
    let zero: __m128i = _mm_setzero_si128();
    d = _mm_unpacklo_epi8(d, zero);
    *pixels = _mm_unpacklo_epi16(d, bp);
}

// weight_h[0]: weight_h vector
// weight_h[1]: scale - weight_h vector
// weight_h[2]: same as [0], second half for height = 16 only
// weight_h[3]: same as [1], second half for height = 16 only
// weight_w[0]: weights_w and scale - weights_w interleave vector
#[inline]
#[target_feature(enable = "ssse3")]
unsafe fn load_weight_w4(height: i32, weight_h: *mut __m128i, weight_w: *mut __m128i) {
    let zero: __m128i = _mm_setzero_si128();
    let d: __m128i = _mm_set1_epi16((1 << SMOOTH_WEIGHT_LOG2_SCALE) as i16);
    let t: __m128i = _mm_cvtsi32_si128(core::ptr::read_unaligned(
        smooth_weights.0.as_ptr() as *const i32
    ));
    *weight_h = _mm_unpacklo_epi8(t, zero);
    *weight_h.offset(1) = _mm_sub_epi16(d, *weight_h);
    *weight_w = _mm_unpacklo_epi16(*weight_h, *weight_h.offset(1));
    if height == 8 {
        let weight: __m128i =
            _mm_loadl_epi64(smooth_weights.0.as_ptr().offset(4) as *const __m128i);
        *weight_h = _mm_unpacklo_epi8(weight, zero);
        *weight_h.offset(1) = _mm_sub_epi16(d, *weight_h);
    } else if height == 16 {
        let weight: __m128i =
            _mm_loadu_si128(smooth_weights.0.as_ptr().offset(12) as *const __m128i);
        *weight_h = _mm_unpacklo_epi8(weight, zero);
        *weight_h.offset(1) = _mm_sub_epi16(d, *weight_h);
        *weight_h.offset(2) = _mm_unpackhi_epi8(weight, zero);
        *weight_h.offset(3) = _mm_sub_epi16(d, *weight_h.offset(2));
    }
}

#[inline]
#[target_feature(enable = "ssse3")]
unsafe fn smooth_pred_4xh(
    pixel: *const __m128i,
    wh: *const __m128i,
    ww: *const __m128i,
    h: i32,
    mut dst: *mut u8,
    stride: isize,
    second_half: i32,
) {
    let round: __m128i = _mm_set1_epi32(1 << SMOOTH_WEIGHT_LOG2_SCALE);
    let one: __m128i = _mm_set1_epi16(1);
    let inc: __m128i = _mm_set1_epi16(0x202);
    let gat: __m128i = _mm_set1_epi32(0xc080400);
    let mut rep: __m128i = if second_half != 0 {
        _mm_set1_epi16(0x8008_i32 as i16)
    } else {
        _mm_set1_epi16(0x8000_i32 as i16)
    };
    let mut d: __m128i = _mm_set1_epi16(0x100);
    for i in 0..h {
        let wg_wg: __m128i = _mm_shuffle_epi8(*wh, d);
        let sc_sc: __m128i = _mm_shuffle_epi8(*wh.offset(1), d);
        let wh_sc: __m128i = _mm_unpacklo_epi16(wg_wg, sc_sc);
        let mut s: __m128i = _mm_madd_epi16(*pixel, wh_sc);
        let mut b: __m128i = _mm_shuffle_epi8(*pixel.offset(1), rep);
        b = _mm_unpacklo_epi16(b, *pixel.offset(2));
        let mut sum: __m128i = _mm_madd_epi16(b, *ww);
        sum = _mm_add_epi32(s, sum);
        sum = _mm_add_epi32(sum, round);
        sum = _mm_srai_epi32::<{ 1 + SMOOTH_WEIGHT_LOG2_SCALE }>(sum);
        sum = _mm_shuffle_epi8(sum, gat);
        core::ptr::write_unaligned(dst as *mut i32, _mm_cvtsi128_si32(sum));
        dst = dst.offset(stride);
        rep = _mm_add_epi16(rep, one);
        d = _mm_add_epi16(d, inc);
    }
}

#[no_mangle]
#[target_feature(enable = "ssse3")]
pub unsafe extern "C-unwind" fn aom_smooth_predictor_4x4_ssse3(
    dst: *mut u8,
    stride: isize,
    above: *const u8,
    left: *const u8,
) {
    let mut pixels: [__m128i; 3] = core::mem::zeroed();
    load_pixel_w4(above, left, 4, pixels.as_mut_ptr());
    let mut wh: [__m128i; 4] = core::mem::zeroed();
    let mut ww: [__m128i; 2] = core::mem::zeroed();
    load_weight_w4(4, wh.as_mut_ptr(), ww.as_mut_ptr());
    smooth_pred_4xh(
        pixels.as_mut_ptr() as *const __m128i,
        wh.as_mut_ptr() as *const __m128i,
        ww.as_mut_ptr() as *const __m128i,
        4,
        dst,
        stride,
        0,
    );
}

#[no_mangle]
#[target_feature(enable = "ssse3")]
pub unsafe extern "C-unwind" fn aom_smooth_predictor_4x8_ssse3(
    dst: *mut u8,
    stride: isize,
    above: *const u8,
    left: *const u8,
) {
    let mut pixels: [__m128i; 3] = core::mem::zeroed();
    load_pixel_w4(above, left, 8, pixels.as_mut_ptr());
    let mut wh: [__m128i; 4] = core::mem::zeroed();
    let mut ww: [__m128i; 2] = core::mem::zeroed();
    load_weight_w4(8, wh.as_mut_ptr(), ww.as_mut_ptr());
    smooth_pred_4xh(
        pixels.as_mut_ptr() as *const __m128i,
        wh.as_mut_ptr() as *const __m128i,
        ww.as_mut_ptr() as *const __m128i,
        8,
        dst,
        stride,
        0,
    );
}

#[no_mangle]
#[target_feature(enable = "ssse3")]
pub unsafe extern "C-unwind" fn aom_smooth_predictor_4x16_ssse3(
    mut dst: *mut u8,
    stride: isize,
    above: *const u8,
    left: *const u8,
) {
    let mut pixels: [__m128i; 3] = core::mem::zeroed();
    load_pixel_w4(above, left, 16, pixels.as_mut_ptr());
    let mut wh: [__m128i; 4] = core::mem::zeroed();
    let mut ww: [__m128i; 2] = core::mem::zeroed();
    load_weight_w4(16, wh.as_mut_ptr(), ww.as_mut_ptr());
    smooth_pred_4xh(
        pixels.as_mut_ptr() as *const __m128i,
        wh.as_mut_ptr() as *const __m128i,
        ww.as_mut_ptr() as *const __m128i,
        8,
        dst,
        stride,
        0,
    );
    dst = dst.offset(stride << 3);
    smooth_pred_4xh(
        pixels.as_mut_ptr() as *const __m128i,
        wh.as_mut_ptr().offset(2) as *const __m128i,
        ww.as_mut_ptr() as *const __m128i,
        8,
        dst,
        stride,
        1,
    );
}

// !CONFIG_REALTIME_ONLY || CONFIG_AV1_DECODER
// pixels[0]: above and below_pred interleave vector, first half
// pixels[1]: above and below_pred interleave vector, second half
// pixels[2]: left vector
// pixels[3]: right_pred vector
// pixels[4]: above and below_pred interleave vector, first half
// pixels[5]: above and below_pred interleave vector, second half
// pixels[6]: left vector + 16
// pixels[7]: right_pred vector
#[inline]
#[target_feature(enable = "ssse3")]
unsafe fn load_pixel_w8(above: *const u8, left: *const u8, height: i32, pixels: *mut __m128i) {
    let zero: __m128i = _mm_setzero_si128();
    let bp: __m128i = _mm_set1_epi16(*left.offset((height - 1) as isize) as i16);
    let mut d: __m128i = _mm_loadl_epi64(above as *const __m128i);
    d = _mm_unpacklo_epi8(d, zero);
    *pixels = _mm_unpacklo_epi16(d, bp);
    *pixels.offset(1) = _mm_unpackhi_epi16(d, bp);
    *pixels.offset(3) = _mm_set1_epi16(*above.offset(7) as i16);
    if height == 4 {
        *pixels.offset(2) = _mm_cvtsi32_si128(core::ptr::read_unaligned(left as *const i32));
    } else if height == 8 {
        *pixels.offset(2) = _mm_loadl_epi64(left as *const __m128i);
    } else if height == 16 {
        *pixels.offset(2) = _mm_load_si128(left as *const __m128i);
    } else {
        *pixels.offset(2) = _mm_load_si128(left as *const __m128i);
        *pixels.offset(4) = *pixels;
        *pixels.offset(5) = *pixels.offset(1);
        *pixels.offset(6) = _mm_load_si128(left.offset(16) as *const __m128i);
        *pixels.offset(7) = *pixels.offset(3);
    }
}

// weight_h[0]: weight_h vector
// weight_h[1]: scale - weight_h vector
// weight_h[2]: same as [0], offset 8
// weight_h[3]: same as [1], offset 8
// weight_h[4]: same as [0], offset 16
// weight_h[5]: same as [1], offset 16
// weight_h[6]: same as [0], offset 24
// weight_h[7]: same as [1], offset 24
// weight_w[0]: weights_w and scale - weights_w interleave vector, first half
// weight_w[1]: weights_w and scale - weights_w interleave vector, second half
#[inline]
#[target_feature(enable = "ssse3")]
unsafe fn load_weight_w8(height: i32, weight_h: *mut __m128i, weight_w: *mut __m128i) {
    let zero: __m128i = _mm_setzero_si128();
    let we_offset: i32 = if height < 8 { 0 } else { 4 };
    let mut we: __m128i =
        _mm_loadu_si128(smooth_weights.0.as_ptr().offset(we_offset as isize) as *const __m128i);
    *weight_h = _mm_unpacklo_epi8(we, zero);
    let d: __m128i = _mm_set1_epi16((1 << SMOOTH_WEIGHT_LOG2_SCALE) as i16);
    *weight_h.offset(1) = _mm_sub_epi16(d, *weight_h);
    if height == 4 {
        we = _mm_srli_si128::<4>(we);
        let mut tmp1: __m128i = _mm_unpacklo_epi8(we, zero);
        let mut tmp2: __m128i = _mm_sub_epi16(d, tmp1);
        *weight_w = _mm_unpacklo_epi16(tmp1, tmp2);
        *weight_w.offset(1) = _mm_unpackhi_epi16(tmp1, tmp2);
    } else {
        *weight_w = _mm_unpacklo_epi16(*weight_h, *weight_h.offset(1));
        *weight_w.offset(1) = _mm_unpackhi_epi16(*weight_h, *weight_h.offset(1));
    }
    if height == 16 {
        we = _mm_loadu_si128(smooth_weights.0.as_ptr().offset(12) as *const __m128i);
        *weight_h = _mm_unpacklo_epi8(we, zero);
        *weight_h.offset(1) = _mm_sub_epi16(d, *weight_h);
        *weight_h.offset(2) = _mm_unpackhi_epi8(we, zero);
        *weight_h.offset(3) = _mm_sub_epi16(d, *weight_h.offset(2));
    } else if height == 32 {
        let weight_lo: __m128i =
            _mm_loadu_si128(smooth_weights.0.as_ptr().offset(28) as *const __m128i);
        *weight_h = _mm_unpacklo_epi8(weight_lo, zero);
        *weight_h.offset(1) = _mm_sub_epi16(d, *weight_h);
        *weight_h.offset(2) = _mm_unpackhi_epi8(weight_lo, zero);
        *weight_h.offset(3) = _mm_sub_epi16(d, *weight_h.offset(2));
        let weight_hi: __m128i =
            _mm_loadu_si128(smooth_weights.0.as_ptr().offset((28 + 16) as isize) as *const __m128i);
        *weight_h.offset(4) = _mm_unpacklo_epi8(weight_hi, zero);
        *weight_h.offset(5) = _mm_sub_epi16(d, *weight_h.offset(4));
        *weight_h.offset(6) = _mm_unpackhi_epi8(weight_hi, zero);
        *weight_h.offset(7) = _mm_sub_epi16(d, *weight_h.offset(6));
    }
}

#[inline]
#[target_feature(enable = "ssse3")]
unsafe fn smooth_pred_8xh(
    pixels: *const __m128i,
    wh: *const __m128i,
    ww: *const __m128i,
    h: i32,
    mut dst: *mut u8,
    stride: isize,
    second_half: i32,
) {
    let round: __m128i = _mm_set1_epi32(1 << SMOOTH_WEIGHT_LOG2_SCALE);
    let one: __m128i = _mm_set1_epi16(1);
    let inc: __m128i = _mm_set1_epi16(0x202);
    let gat: __m128i = _mm_set_epi32(0, 0, 0xe0c0a08, 0x6040200);
    let mut rep: __m128i = if second_half != 0 {
        _mm_set1_epi16(0x8008_i32 as i16)
    } else {
        _mm_set1_epi16(0x8000_i32 as i16)
    };
    let mut d: __m128i = _mm_set1_epi16(0x100);
    let mut i: i32 = 0;
    {
        i = 0;
        while i < h {
            let wg_wg: __m128i = _mm_shuffle_epi8(*wh, d);
            let sc_sc: __m128i = _mm_shuffle_epi8(*wh.offset(1), d);
            let wh_sc: __m128i = _mm_unpacklo_epi16(wg_wg, sc_sc);
            let mut s0: __m128i = _mm_madd_epi16(*pixels, wh_sc);
            let mut s1: __m128i = _mm_madd_epi16(*pixels.offset(1), wh_sc);
            let mut b: __m128i = _mm_shuffle_epi8(*pixels.offset(2), rep);
            b = _mm_unpacklo_epi16(b, *pixels.offset(3));
            let mut sum0: __m128i = _mm_madd_epi16(b, *ww);
            let mut sum1: __m128i = _mm_madd_epi16(b, *ww.offset(1));
            s0 = _mm_add_epi32(s0, sum0);
            s0 = _mm_add_epi32(s0, round);
            s0 = _mm_srai_epi32::<{ 1 + SMOOTH_WEIGHT_LOG2_SCALE }>(s0);
            s1 = _mm_add_epi32(s1, sum1);
            s1 = _mm_add_epi32(s1, round);
            s1 = _mm_srai_epi32::<{ 1 + SMOOTH_WEIGHT_LOG2_SCALE }>(s1);
            sum0 = _mm_packus_epi16(s0, s1);
            sum0 = _mm_shuffle_epi8(sum0, gat);
            _mm_storel_epi64(dst as *mut __m128i, sum0);
            dst = dst.offset(stride);
            rep = _mm_add_epi16(rep, one);
            d = _mm_add_epi16(d, inc);
            i += 1;
        }
    }
}

#[no_mangle]
#[target_feature(enable = "ssse3")]
pub unsafe extern "C-unwind" fn aom_smooth_predictor_8x4_ssse3(
    dst: *mut u8,
    stride: isize,
    above: *const u8,
    left: *const u8,
) {
    let mut pixels: [__m128i; 4] = core::mem::zeroed();
    load_pixel_w8(above, left, 4, pixels.as_mut_ptr());
    let mut wh: [__m128i; 4] = core::mem::zeroed();
    let mut ww: [__m128i; 2] = core::mem::zeroed();
    load_weight_w8(4, wh.as_mut_ptr(), ww.as_mut_ptr());
    smooth_pred_8xh(
        pixels.as_mut_ptr() as *const __m128i,
        wh.as_mut_ptr() as *const __m128i,
        ww.as_mut_ptr() as *const __m128i,
        4,
        dst,
        stride,
        0,
    );
}

#[no_mangle]
#[target_feature(enable = "ssse3")]
pub unsafe extern "C-unwind" fn aom_smooth_predictor_8x8_ssse3(
    dst: *mut u8,
    stride: isize,
    above: *const u8,
    left: *const u8,
) {
    let mut pixels: [__m128i; 4] = core::mem::zeroed();
    load_pixel_w8(above, left, 8, pixels.as_mut_ptr());
    let mut wh: [__m128i; 4] = core::mem::zeroed();
    let mut ww: [__m128i; 2] = core::mem::zeroed();
    load_weight_w8(8, wh.as_mut_ptr(), ww.as_mut_ptr());
    smooth_pred_8xh(
        pixels.as_mut_ptr() as *const __m128i,
        wh.as_mut_ptr() as *const __m128i,
        ww.as_mut_ptr() as *const __m128i,
        8,
        dst,
        stride,
        0,
    );
}

#[no_mangle]
#[target_feature(enable = "ssse3")]
pub unsafe extern "C-unwind" fn aom_smooth_predictor_8x16_ssse3(
    mut dst: *mut u8,
    stride: isize,
    above: *const u8,
    left: *const u8,
) {
    let mut pixels: [__m128i; 4] = core::mem::zeroed();
    load_pixel_w8(above, left, 16, pixels.as_mut_ptr());
    let mut wh: [__m128i; 4] = core::mem::zeroed();
    let mut ww: [__m128i; 2] = core::mem::zeroed();
    load_weight_w8(16, wh.as_mut_ptr(), ww.as_mut_ptr());
    smooth_pred_8xh(
        pixels.as_mut_ptr() as *const __m128i,
        wh.as_mut_ptr() as *const __m128i,
        ww.as_mut_ptr() as *const __m128i,
        8,
        dst,
        stride,
        0,
    );
    dst = dst.offset(stride << 3);
    smooth_pred_8xh(
        pixels.as_mut_ptr() as *const __m128i,
        wh.as_mut_ptr().offset(2) as *const __m128i,
        ww.as_mut_ptr() as *const __m128i,
        8,
        dst,
        stride,
        1,
    );
}

#[no_mangle]
#[target_feature(enable = "ssse3")]
pub unsafe extern "C-unwind" fn aom_smooth_predictor_8x32_ssse3(
    mut dst: *mut u8,
    stride: isize,
    above: *const u8,
    left: *const u8,
) {
    let mut pixels: [__m128i; 8] = core::mem::zeroed();
    load_pixel_w8(above, left, 32, pixels.as_mut_ptr());
    let mut wh: [__m128i; 8] = core::mem::zeroed();
    let mut ww: [__m128i; 2] = core::mem::zeroed();
    load_weight_w8(32, wh.as_mut_ptr(), ww.as_mut_ptr());
    smooth_pred_8xh(
        pixels.as_mut_ptr() as *const __m128i,
        wh.as_mut_ptr() as *const __m128i,
        ww.as_mut_ptr() as *const __m128i,
        8,
        dst,
        stride,
        0,
    );
    dst = dst.offset(stride << 3);
    smooth_pred_8xh(
        pixels.as_mut_ptr() as *const __m128i,
        wh.as_mut_ptr().offset(2) as *const __m128i,
        ww.as_mut_ptr() as *const __m128i,
        8,
        dst,
        stride,
        1,
    );
    dst = dst.offset(stride << 3);
    smooth_pred_8xh(
        pixels.as_mut_ptr().offset(4) as *const __m128i,
        wh.as_mut_ptr().offset(4) as *const __m128i,
        ww.as_mut_ptr() as *const __m128i,
        8,
        dst,
        stride,
        0,
    );
    dst = dst.offset(stride << 3);
    smooth_pred_8xh(
        pixels.as_mut_ptr().offset(4) as *const __m128i,
        wh.as_mut_ptr().offset(6) as *const __m128i,
        ww.as_mut_ptr() as *const __m128i,
        8,
        dst,
        stride,
        1,
    );
}

// !CONFIG_REALTIME_ONLY || CONFIG_AV1_DECODER
// TODO(slavarnway): Visual Studio only supports restrict when /std:c11
// (available in 2019+) or greater is specified; __restrict can be used in that
// case. This should be moved to rtcd and used consistently between the
// function declarations and definitions to avoid warnings in Visual Studio
// when defining LIBAOM_RESTRICT to restrict or __restrict.
#[inline]
#[target_feature(enable = "ssse3")]
unsafe fn Load4(src: *const core::ffi::c_void) -> __m128i {
    // With new compilers such as clang 8.0.0 we can use the new _mm_loadu_si32
    // intrinsic. Both _mm_loadu_si32(src) and the code here are compiled into a
    // movss instruction.
    //
    // Until compiler support of _mm_loadu_si32 is widespread, use of
    // _mm_loadu_si32 is banned.
    let mut val: i32 = 0;
    core::ptr::copy_nonoverlapping(
        src as *const u8,
        &raw mut val as *mut u8,
        core::mem::size_of::<i32>(),
    );
    return _mm_cvtsi32_si128(val);
}

#[inline]
#[target_feature(enable = "ssse3")]
unsafe fn LoadLo8(a: *const core::ffi::c_void) -> __m128i {
    return _mm_loadl_epi64(a as *const __m128i);
}

#[inline]
#[target_feature(enable = "ssse3")]
unsafe fn LoadUnaligned16(a: *const core::ffi::c_void) -> __m128i {
    return _mm_loadu_si128(a as *const __m128i);
}

#[inline]
#[target_feature(enable = "ssse3")]
unsafe fn Store4(dst: *mut core::ffi::c_void, x: __m128i) {
    let val: i32 = _mm_cvtsi128_si32(x);
    core::ptr::copy_nonoverlapping(
        &raw const val as *const u8,
        dst as *mut u8,
        core::mem::size_of::<i32>(),
    );
}

#[inline]
#[target_feature(enable = "ssse3")]
unsafe fn StoreLo8(a: *mut core::ffi::c_void, v: __m128i) {
    _mm_storel_epi64(a as *mut __m128i, v);
}

#[inline]
#[target_feature(enable = "ssse3")]
unsafe fn StoreUnaligned16(a: *mut core::ffi::c_void, v: __m128i) {
    _mm_storeu_si128(a as *mut __m128i, v);
}

#[inline]
#[target_feature(enable = "ssse3")]
unsafe fn cvtepu8_epi16(x: __m128i) -> __m128i {
    return _mm_unpacklo_epi8(x, _mm_setzero_si128());
}

#[inline]
#[target_feature(enable = "ssse3")]
unsafe fn cvtepu8_epi32(x: __m128i) -> __m128i {
    let tmp: __m128i = _mm_unpacklo_epi8(x, _mm_setzero_si128());
    return _mm_unpacklo_epi16(tmp, _mm_setzero_si128());
}

#[inline]
#[target_feature(enable = "ssse3")]
unsafe fn cvtepu16_epi32(x: __m128i) -> __m128i {
    return _mm_unpacklo_epi16(x, _mm_setzero_si128());
}

#[inline]
#[target_feature(enable = "ssse3")]
unsafe fn smooth_predictor_wxh(
    mut dst: *mut u8,
    stride: isize,
    top_row: *const u8,
    left_column: *const u8,
    width: i32,
    height: i32,
) {
    let mut sm_weights_h: *const u8 = smooth_weights.0.as_ptr().offset(height as isize).offset(-4);
    let mut sm_weights_w: *const u8 = smooth_weights.0.as_ptr().offset(width as isize).offset(-4);
    let zero: __m128i = _mm_setzero_si128();
    let scale_value: __m128i = _mm_set1_epi16((1 << SMOOTH_WEIGHT_LOG2_SCALE) as i16);
    let bottom_left: __m128i = _mm_cvtsi32_si128(*left_column.offset((height - 1) as isize) as i32);
    let top_right: __m128i = _mm_set1_epi16(*top_row.offset((width - 1) as isize) as i16);
    let round: __m128i = _mm_set1_epi32(1 << SMOOTH_WEIGHT_LOG2_SCALE);
    for y in 0..height {
        let weights_y: __m128i = _mm_cvtsi32_si128(*sm_weights_h.offset(y as isize) as i32);
        let left_y: __m128i = _mm_cvtsi32_si128(*left_column.offset(y as isize) as i32);
        let scale_m_weights_y: __m128i = _mm_sub_epi16(scale_value, weights_y);
        let mut scaled_bottom_left: __m128i = _mm_mullo_epi16(scale_m_weights_y, bottom_left);
        let weight_left_y: __m128i = _mm_shuffle_epi32::<0>(_mm_unpacklo_epi16(weights_y, left_y));
        scaled_bottom_left = _mm_add_epi32(scaled_bottom_left, round);
        scaled_bottom_left = _mm_shuffle_epi32::<0>(scaled_bottom_left);
        for x in (0..width).step_by(8) {
            let top_x: __m128i = LoadLo8(top_row.offset(x as isize) as *const core::ffi::c_void);
            let weights_x: __m128i =
                LoadLo8(sm_weights_w.offset(x as isize) as *const core::ffi::c_void);
            let top_weights_x: __m128i = _mm_unpacklo_epi8(top_x, weights_x);
            let top_weights_x_lo: __m128i = cvtepu8_epi16(top_weights_x);
            let top_weights_x_hi: __m128i = _mm_unpackhi_epi8(top_weights_x, zero);
            // Here opposite weights and pixels are multiplied, where the order of
            // interleaving is indicated in the names.
            let mut pred_lo: __m128i = _mm_madd_epi16(top_weights_x_lo, weight_left_y);
            let mut pred_hi: __m128i = _mm_madd_epi16(top_weights_x_hi, weight_left_y);
            // |scaled_bottom_left| is always scaled by the same weight each row, so
            // we only derive |scaled_top_right| values here.
            let inverted_weights_x: __m128i = _mm_sub_epi16(scale_value, cvtepu8_epi16(weights_x));
            let scaled_top_right: __m128i = _mm_mullo_epi16(inverted_weights_x, top_right);
            let scaled_top_right_lo: __m128i = cvtepu16_epi32(scaled_top_right);
            let scaled_top_right_hi: __m128i = _mm_unpackhi_epi16(scaled_top_right, zero);
            pred_lo = _mm_add_epi32(pred_lo, scaled_bottom_left);
            pred_hi = _mm_add_epi32(pred_hi, scaled_bottom_left);
            pred_lo = _mm_add_epi32(pred_lo, scaled_top_right_lo);
            pred_hi = _mm_add_epi32(pred_hi, scaled_top_right_hi);
            // The round value for RightShiftWithRounding was added with
            // |scaled_bottom_left|.
            pred_lo = _mm_srli_epi32::<{ 1 + SMOOTH_WEIGHT_LOG2_SCALE }>(pred_lo);
            pred_hi = _mm_srli_epi32::<{ 1 + SMOOTH_WEIGHT_LOG2_SCALE }>(pred_hi);
            let pred: __m128i = _mm_packus_epi16(pred_lo, pred_hi);
            StoreLo8(
                dst.offset(x as isize) as *mut core::ffi::c_void,
                _mm_packus_epi16(pred, pred),
            );
        }
        dst = dst.offset(stride);
    }
}

#[no_mangle]
#[target_feature(enable = "ssse3")]
pub unsafe extern "C-unwind" fn aom_smooth_predictor_16x4_ssse3(
    dst: *mut u8,
    stride: isize,
    above: *const u8,
    left: *const u8,
) {
    smooth_predictor_wxh(dst, stride, above, left, 16, 4);
}

// !CONFIG_REALTIME_ONLY || CONFIG_AV1_DECODER
#[no_mangle]
#[target_feature(enable = "ssse3")]
pub unsafe extern "C-unwind" fn aom_smooth_predictor_16x8_ssse3(
    dst: *mut u8,
    stride: isize,
    above: *const u8,
    left: *const u8,
) {
    smooth_predictor_wxh(dst, stride, above, left, 16, 8);
}

#[no_mangle]
#[target_feature(enable = "ssse3")]
pub unsafe extern "C-unwind" fn aom_smooth_predictor_16x16_ssse3(
    dst: *mut u8,
    stride: isize,
    above: *const u8,
    left: *const u8,
) {
    smooth_predictor_wxh(dst, stride, above, left, 16, 16);
}

#[no_mangle]
#[target_feature(enable = "ssse3")]
pub unsafe extern "C-unwind" fn aom_smooth_predictor_16x32_ssse3(
    dst: *mut u8,
    stride: isize,
    above: *const u8,
    left: *const u8,
) {
    smooth_predictor_wxh(dst, stride, above, left, 16, 32);
}

#[no_mangle]
#[target_feature(enable = "ssse3")]
pub unsafe extern "C-unwind" fn aom_smooth_predictor_16x64_ssse3(
    dst: *mut u8,
    stride: isize,
    above: *const u8,
    left: *const u8,
) {
    smooth_predictor_wxh(dst, stride, above, left, 16, 64);
}

#[no_mangle]
#[target_feature(enable = "ssse3")]
pub unsafe extern "C-unwind" fn aom_smooth_predictor_32x8_ssse3(
    dst: *mut u8,
    stride: isize,
    above: *const u8,
    left: *const u8,
) {
    smooth_predictor_wxh(dst, stride, above, left, 32, 8);
}

// !CONFIG_REALTIME_ONLY || CONFIG_AV1_DECODER
#[no_mangle]
#[target_feature(enable = "ssse3")]
pub unsafe extern "C-unwind" fn aom_smooth_predictor_32x16_ssse3(
    dst: *mut u8,
    stride: isize,
    above: *const u8,
    left: *const u8,
) {
    smooth_predictor_wxh(dst, stride, above, left, 32, 16);
}

#[no_mangle]
#[target_feature(enable = "ssse3")]
pub unsafe extern "C-unwind" fn aom_smooth_predictor_32x32_ssse3(
    dst: *mut u8,
    stride: isize,
    above: *const u8,
    left: *const u8,
) {
    smooth_predictor_wxh(dst, stride, above, left, 32, 32);
}

#[no_mangle]
#[target_feature(enable = "ssse3")]
pub unsafe extern "C-unwind" fn aom_smooth_predictor_32x64_ssse3(
    dst: *mut u8,
    stride: isize,
    above: *const u8,
    left: *const u8,
) {
    smooth_predictor_wxh(dst, stride, above, left, 32, 64);
}

#[no_mangle]
#[target_feature(enable = "ssse3")]
pub unsafe extern "C-unwind" fn aom_smooth_predictor_64x16_ssse3(
    dst: *mut u8,
    stride: isize,
    above: *const u8,
    left: *const u8,
) {
    smooth_predictor_wxh(dst, stride, above, left, 64, 16);
}

// !CONFIG_REALTIME_ONLY || CONFIG_AV1_DECODER
#[no_mangle]
#[target_feature(enable = "ssse3")]
pub unsafe extern "C-unwind" fn aom_smooth_predictor_64x32_ssse3(
    dst: *mut u8,
    stride: isize,
    above: *const u8,
    left: *const u8,
) {
    smooth_predictor_wxh(dst, stride, above, left, 64, 32);
}

#[no_mangle]
#[target_feature(enable = "ssse3")]
pub unsafe extern "C-unwind" fn aom_smooth_predictor_64x64_ssse3(
    dst: *mut u8,
    stride: isize,
    above: *const u8,
    left: *const u8,
) {
    smooth_predictor_wxh(dst, stride, above, left, 64, 64);
}

// -----------------------------------------------------------------------------
// Smooth horizontal/vertical helper functions.
// For Horizontal, pixels1 and pixels2 are the same repeated value. For
// Vertical, weights1 and weights2 are the same, and scaled_corner1 and
// scaled_corner2 are the same.
#[inline]
#[target_feature(enable = "ssse3")]
unsafe fn write_smooth_directional_sum16(
    dst: *mut u8,
    pixels1: __m128i,
    pixels2: __m128i,
    weights1: __m128i,
    weights2: __m128i,
    scaled_corner1: __m128i,
    scaled_corner2: __m128i,
    round: __m128i,
) {
    let weighted_px1: __m128i = _mm_mullo_epi16(pixels1, weights1);
    let weighted_px2: __m128i = _mm_mullo_epi16(pixels2, weights2);
    let pred_sum1: __m128i = _mm_add_epi16(scaled_corner1, weighted_px1);
    let pred_sum2: __m128i = _mm_add_epi16(scaled_corner2, weighted_px2);
    // Equivalent to RightShiftWithRounding(pred[x][y], 8).
    let pred1: __m128i = _mm_srli_epi16::<{ 8 }>(_mm_add_epi16(pred_sum1, round));
    let pred2: __m128i = _mm_srli_epi16::<{ 8 }>(_mm_add_epi16(pred_sum2, round));
    StoreUnaligned16(
        dst as *mut core::ffi::c_void,
        _mm_packus_epi16(pred1, pred2),
    );
}

#[inline]
#[target_feature(enable = "ssse3")]
unsafe fn smooth_directional_sum8(
    pixels: __m128i,
    weights: __m128i,
    scaled_corner: __m128i,
) -> __m128i {
    let weighted_px: __m128i = _mm_mullo_epi16(pixels, weights);
    return _mm_add_epi16(scaled_corner, weighted_px);
}

#[inline]
#[target_feature(enable = "ssse3")]
unsafe fn write_smooth_directional_sum8(
    dst: *mut u8,
    pixels: *const __m128i,
    weights: *const __m128i,
    scaled_corner: *const __m128i,
    round: *const __m128i,
) {
    let pred_sum: __m128i = smooth_directional_sum8(*pixels, *weights, *scaled_corner);
    // Equivalent to RightShiftWithRounding(pred[x][y], 8).
    let pred: __m128i = _mm_srli_epi16::<{ 8 }>(_mm_add_epi16(pred_sum, *round));
    StoreLo8(dst as *mut core::ffi::c_void, _mm_packus_epi16(pred, pred));
}

// -----------------------------------------------------------------------------
// SMOOTH_V_PRED
#[inline]
#[target_feature(enable = "ssse3")]
unsafe fn load_smooth_vertical_pixels4(
    above: *const u8,
    left: *const u8,
    height: i32,
    pixels: *mut __m128i,
) {
    let mut top: __m128i = Load4(above as *const core::ffi::c_void);
    let bottom_left: __m128i = _mm_set1_epi16(*left.offset((height - 1) as isize) as i16);
    top = cvtepu8_epi16(top);
    *pixels = _mm_unpacklo_epi16(top, bottom_left);
}

// |weight_array| alternates weight vectors from the table with their inverted
// (256-w) counterparts. This is precomputed by the compiler when the weights
// table is visible to this module. Removing this visibility can cut speed by up
// to half in both 4xH and 8xH transforms.
#[inline]
#[target_feature(enable = "ssse3")]
unsafe fn load_smooth_vertical_weights4(
    weight_array: *const u8,
    height: i32,
    weights: *mut __m128i,
) {
    let inverter: __m128i = _mm_set1_epi16(256);
    if height == 4 {
        let weight: __m128i = Load4(weight_array as *const core::ffi::c_void);
        *weights = cvtepu8_epi16(weight);
        *weights.offset(1) = _mm_sub_epi16(inverter, *weights);
    } else if height == 8 {
        let weight: __m128i = LoadLo8(weight_array.offset(4) as *const core::ffi::c_void);
        *weights = cvtepu8_epi16(weight);
        *weights.offset(1) = _mm_sub_epi16(inverter, *weights);
    } else {
        let weight: __m128i = LoadUnaligned16(weight_array.offset(12) as *const core::ffi::c_void);
        let zero: __m128i = _mm_setzero_si128();
        *weights = cvtepu8_epi16(weight);
        *weights.offset(1) = _mm_sub_epi16(inverter, *weights);
        *weights.offset(2) = _mm_unpackhi_epi8(weight, zero);
        *weights.offset(3) = _mm_sub_epi16(inverter, *weights.offset(2));
    }
}

#[inline]
#[target_feature(enable = "ssse3")]
unsafe fn write_smooth_vertical4xh(
    pixel: *const __m128i,
    weight: *const __m128i,
    height: i32,
    mut dst: *mut u8,
    stride: isize,
) {
    let pred_round: __m128i = _mm_set1_epi32(128);
    let mask_increment: __m128i = _mm_set1_epi16(0x0202);
    let cvtepu8_epi32: __m128i = _mm_set1_epi32(0xC080400);
    let mut y_select: __m128i = _mm_set1_epi16(0x0100);
    for y in 0..height {
        let weight_y: __m128i = _mm_shuffle_epi8(*weight, y_select);
        let inverted_weight_y: __m128i = _mm_shuffle_epi8(*weight.offset(1), y_select);
        let alternate_weights: __m128i = _mm_unpacklo_epi16(weight_y, inverted_weight_y);
        // Here the pixel vector is top_row[0], corner, top_row[1], corner, ...
        // The madd instruction yields four results of the form:
        // (top_row[x] * weight[y] + corner * inverted_weight[y])
        let mut sum: __m128i = _mm_madd_epi16(*pixel, alternate_weights);
        sum = _mm_add_epi32(sum, pred_round);
        sum = _mm_srai_epi32::<{ 8 }>(sum);
        sum = _mm_shuffle_epi8(sum, cvtepu8_epi32);
        Store4(dst as *mut core::ffi::c_void, sum);
        dst = dst.offset(stride);
        y_select = _mm_add_epi16(y_select, mask_increment);
    }
}

#[no_mangle]
#[target_feature(enable = "ssse3")]
pub unsafe extern "C-unwind" fn aom_smooth_v_predictor_4x4_ssse3(
    dst: *mut u8,
    stride: isize,
    top_row: *const u8,
    left_column: *const u8,
) {
    let mut pixels: __m128i = core::mem::zeroed();
    load_smooth_vertical_pixels4(top_row, left_column, 4, &raw mut pixels);
    let mut weights: [__m128i; 2] = core::mem::zeroed();
    load_smooth_vertical_weights4(smooth_weights.0.as_ptr(), 4, weights.as_mut_ptr());
    write_smooth_vertical4xh(
        &raw mut pixels as *const __m128i,
        weights.as_mut_ptr() as *const __m128i,
        4,
        dst,
        stride,
    );
}

#[no_mangle]
#[target_feature(enable = "ssse3")]
pub unsafe extern "C-unwind" fn aom_smooth_v_predictor_4x8_ssse3(
    dst: *mut u8,
    stride: isize,
    top_row: *const u8,
    left_column: *const u8,
) {
    let mut pixels: __m128i = core::mem::zeroed();
    load_smooth_vertical_pixels4(top_row, left_column, 8, &raw mut pixels);
    let mut weights: [__m128i; 2] = core::mem::zeroed();
    load_smooth_vertical_weights4(smooth_weights.0.as_ptr(), 8, weights.as_mut_ptr());
    write_smooth_vertical4xh(
        &raw mut pixels as *const __m128i,
        weights.as_mut_ptr() as *const __m128i,
        8,
        dst,
        stride,
    );
}

#[no_mangle]
#[target_feature(enable = "ssse3")]
pub unsafe extern "C-unwind" fn aom_smooth_v_predictor_4x16_ssse3(
    mut dst: *mut u8,
    stride: isize,
    top_row: *const u8,
    left_column: *const u8,
) {
    let mut pixels: __m128i = core::mem::zeroed();
    load_smooth_vertical_pixels4(top_row, left_column, 16, &raw mut pixels);
    let mut weights: [__m128i; 4] = core::mem::zeroed();
    load_smooth_vertical_weights4(smooth_weights.0.as_ptr(), 16, weights.as_mut_ptr());
    write_smooth_vertical4xh(
        &raw mut pixels as *const __m128i,
        weights.as_mut_ptr() as *const __m128i,
        8,
        dst,
        stride,
    );
    dst = dst.offset(stride << 3);
    write_smooth_vertical4xh(
        &raw mut pixels as *const __m128i,
        weights.as_mut_ptr().offset(2) as *const __m128i,
        8,
        dst,
        stride,
    );
}

// !CONFIG_REALTIME_ONLY || CONFIG_AV1_DECODER
#[no_mangle]
#[target_feature(enable = "ssse3")]
pub unsafe extern "C-unwind" fn aom_smooth_v_predictor_8x4_ssse3(
    mut dst: *mut u8,
    stride: isize,
    top_row: *const u8,
    left_column: *const u8,
) {
    let bottom_left: __m128i = _mm_set1_epi16(*left_column.offset(3) as i16);
    let weights: __m128i =
        cvtepu8_epi16(Load4(smooth_weights.0.as_ptr() as *const core::ffi::c_void));
    let scale: __m128i = _mm_set1_epi16((1 << SMOOTH_WEIGHT_LOG2_SCALE) as i16);
    let inverted_weights: __m128i = _mm_sub_epi16(scale, weights);
    let scaled_bottom_left: __m128i = _mm_mullo_epi16(inverted_weights, bottom_left);
    let round: __m128i = _mm_set1_epi16((1 << (SMOOTH_WEIGHT_LOG2_SCALE - 1)) as i16);
    let mut y_select: __m128i = _mm_set1_epi32(0x01000100);
    let top: __m128i = cvtepu8_epi16(LoadLo8(top_row as *const core::ffi::c_void));
    let mut weights_y: __m128i = _mm_shuffle_epi8(weights, y_select);
    let mut scaled_bottom_left_y: __m128i = _mm_shuffle_epi8(scaled_bottom_left, y_select);
    write_smooth_directional_sum8(
        dst,
        &raw const top,
        &raw mut weights_y as *const __m128i,
        &raw mut scaled_bottom_left_y as *const __m128i,
        &raw const round,
    );
    dst = dst.offset(stride);
    y_select = _mm_set1_epi32(0x03020302);
    weights_y = _mm_shuffle_epi8(weights, y_select);
    scaled_bottom_left_y = _mm_shuffle_epi8(scaled_bottom_left, y_select);
    write_smooth_directional_sum8(
        dst,
        &raw const top,
        &raw mut weights_y as *const __m128i,
        &raw mut scaled_bottom_left_y as *const __m128i,
        &raw const round,
    );
    dst = dst.offset(stride);
    y_select = _mm_set1_epi32(0x05040504);
    weights_y = _mm_shuffle_epi8(weights, y_select);
    scaled_bottom_left_y = _mm_shuffle_epi8(scaled_bottom_left, y_select);
    write_smooth_directional_sum8(
        dst,
        &raw const top,
        &raw mut weights_y as *const __m128i,
        &raw mut scaled_bottom_left_y as *const __m128i,
        &raw const round,
    );
    dst = dst.offset(stride);
    y_select = _mm_set1_epi32(0x07060706);
    weights_y = _mm_shuffle_epi8(weights, y_select);
    scaled_bottom_left_y = _mm_shuffle_epi8(scaled_bottom_left, y_select);
    write_smooth_directional_sum8(
        dst,
        &raw const top,
        &raw mut weights_y as *const __m128i,
        &raw mut scaled_bottom_left_y as *const __m128i,
        &raw const round,
    );
}

#[no_mangle]
#[target_feature(enable = "ssse3")]
pub unsafe extern "C-unwind" fn aom_smooth_v_predictor_8x8_ssse3(
    mut dst: *mut u8,
    stride: isize,
    top_row: *const u8,
    left_column: *const u8,
) {
    let bottom_left: __m128i = _mm_set1_epi16(*left_column.offset(7) as i16);
    let weights: __m128i = cvtepu8_epi16(LoadLo8(
        smooth_weights.0.as_ptr().offset(4) as *const core::ffi::c_void
    ));
    let scale: __m128i = _mm_set1_epi16((1 << SMOOTH_WEIGHT_LOG2_SCALE) as i16);
    let inverted_weights: __m128i = _mm_sub_epi16(scale, weights);
    let scaled_bottom_left: __m128i = _mm_mullo_epi16(inverted_weights, bottom_left);
    let round: __m128i = _mm_set1_epi16((1 << (SMOOTH_WEIGHT_LOG2_SCALE - 1)) as i16);
    let top: __m128i = cvtepu8_epi16(LoadLo8(top_row as *const core::ffi::c_void));
    for y_mask in (0x01000100..0x0F0E0F0F).step_by(0x02020202) {
        let y_select: __m128i = _mm_set1_epi32(y_mask);
        let weights_y: __m128i = _mm_shuffle_epi8(weights, y_select);
        let scaled_bottom_left_y: __m128i = _mm_shuffle_epi8(scaled_bottom_left, y_select);
        write_smooth_directional_sum8(
            dst,
            &raw const top,
            &raw const weights_y,
            &raw const scaled_bottom_left_y,
            &raw const round,
        );
        dst = dst.offset(stride);
    }
}

#[no_mangle]
#[target_feature(enable = "ssse3")]
pub unsafe extern "C-unwind" fn aom_smooth_v_predictor_8x16_ssse3(
    mut dst: *mut u8,
    stride: isize,
    top_row: *const u8,
    left_column: *const u8,
) {
    let bottom_left: __m128i = _mm_set1_epi16(*left_column.offset(15) as i16);
    let weights: __m128i =
        LoadUnaligned16(smooth_weights.0.as_ptr().offset(12) as *const core::ffi::c_void);
    let weights1: __m128i = cvtepu8_epi16(weights);
    let weights2: __m128i = cvtepu8_epi16(_mm_srli_si128::<8>(weights));
    let scale: __m128i = _mm_set1_epi16((1 << SMOOTH_WEIGHT_LOG2_SCALE) as i16);
    let inverted_weights1: __m128i = _mm_sub_epi16(scale, weights1);
    let inverted_weights2: __m128i = _mm_sub_epi16(scale, weights2);
    let scaled_bottom_left1: __m128i = _mm_mullo_epi16(inverted_weights1, bottom_left);
    let scaled_bottom_left2: __m128i = _mm_mullo_epi16(inverted_weights2, bottom_left);
    let round: __m128i = _mm_set1_epi16((1 << (SMOOTH_WEIGHT_LOG2_SCALE - 1)) as i16);
    let top: __m128i = cvtepu8_epi16(LoadLo8(top_row as *const core::ffi::c_void));
    for y_mask in (0x01000100..0x0F0E0F0F).step_by(0x02020202) {
        let y_select: __m128i = _mm_set1_epi32(y_mask);
        let weights_y: __m128i = _mm_shuffle_epi8(weights1, y_select);
        let scaled_bottom_left_y: __m128i = _mm_shuffle_epi8(scaled_bottom_left1, y_select);
        write_smooth_directional_sum8(
            dst,
            &raw const top,
            &raw const weights_y,
            &raw const scaled_bottom_left_y,
            &raw const round,
        );
        dst = dst.offset(stride);
    }
    for y_mask in (0x01000100..0x0F0E0F0F).step_by(0x02020202) {
        let y_select: __m128i = _mm_set1_epi32(y_mask);
        let weights_y: __m128i = _mm_shuffle_epi8(weights2, y_select);
        let scaled_bottom_left_y: __m128i = _mm_shuffle_epi8(scaled_bottom_left2, y_select);
        write_smooth_directional_sum8(
            dst,
            &raw const top,
            &raw const weights_y,
            &raw const scaled_bottom_left_y,
            &raw const round,
        );
        dst = dst.offset(stride);
    }
}

#[no_mangle]
#[target_feature(enable = "ssse3")]
pub unsafe extern "C-unwind" fn aom_smooth_v_predictor_8x32_ssse3(
    mut dst: *mut u8,
    stride: isize,
    top_row: *const u8,
    left_column: *const u8,
) {
    let zero: __m128i = _mm_setzero_si128();
    let bottom_left: __m128i = _mm_set1_epi16(*left_column.offset(31) as i16);
    let weights_lo: __m128i =
        LoadUnaligned16(smooth_weights.0.as_ptr().offset(28) as *const core::ffi::c_void);
    let weights_hi: __m128i =
        LoadUnaligned16(smooth_weights.0.as_ptr().offset(44) as *const core::ffi::c_void);
    let weights1: __m128i = cvtepu8_epi16(weights_lo);
    let weights2: __m128i = _mm_unpackhi_epi8(weights_lo, zero);
    let weights3: __m128i = cvtepu8_epi16(weights_hi);
    let weights4: __m128i = _mm_unpackhi_epi8(weights_hi, zero);
    let scale: __m128i = _mm_set1_epi16((1 << SMOOTH_WEIGHT_LOG2_SCALE) as i16);
    let inverted_weights1: __m128i = _mm_sub_epi16(scale, weights1);
    let inverted_weights2: __m128i = _mm_sub_epi16(scale, weights2);
    let inverted_weights3: __m128i = _mm_sub_epi16(scale, weights3);
    let inverted_weights4: __m128i = _mm_sub_epi16(scale, weights4);
    let scaled_bottom_left1: __m128i = _mm_mullo_epi16(inverted_weights1, bottom_left);
    let scaled_bottom_left2: __m128i = _mm_mullo_epi16(inverted_weights2, bottom_left);
    let scaled_bottom_left3: __m128i = _mm_mullo_epi16(inverted_weights3, bottom_left);
    let scaled_bottom_left4: __m128i = _mm_mullo_epi16(inverted_weights4, bottom_left);
    let round: __m128i = _mm_set1_epi16((1 << (SMOOTH_WEIGHT_LOG2_SCALE - 1)) as i16);
    let top: __m128i = cvtepu8_epi16(LoadLo8(top_row as *const core::ffi::c_void));
    for y_mask in (0x01000100..0x0F0E0F0F).step_by(0x02020202) {
        let y_select: __m128i = _mm_set1_epi32(y_mask);
        let weights_y: __m128i = _mm_shuffle_epi8(weights1, y_select);
        let scaled_bottom_left_y: __m128i = _mm_shuffle_epi8(scaled_bottom_left1, y_select);
        write_smooth_directional_sum8(
            dst,
            &raw const top,
            &raw const weights_y,
            &raw const scaled_bottom_left_y,
            &raw const round,
        );
        dst = dst.offset(stride);
    }
    for y_mask in (0x01000100..0x0F0E0F0F).step_by(0x02020202) {
        let y_select: __m128i = _mm_set1_epi32(y_mask);
        let weights_y: __m128i = _mm_shuffle_epi8(weights2, y_select);
        let scaled_bottom_left_y: __m128i = _mm_shuffle_epi8(scaled_bottom_left2, y_select);
        write_smooth_directional_sum8(
            dst,
            &raw const top,
            &raw const weights_y,
            &raw const scaled_bottom_left_y,
            &raw const round,
        );
        dst = dst.offset(stride);
    }
    for y_mask in (0x01000100..0x0F0E0F0F).step_by(0x02020202) {
        let y_select: __m128i = _mm_set1_epi32(y_mask);
        let weights_y: __m128i = _mm_shuffle_epi8(weights3, y_select);
        let scaled_bottom_left_y: __m128i = _mm_shuffle_epi8(scaled_bottom_left3, y_select);
        write_smooth_directional_sum8(
            dst,
            &raw const top,
            &raw const weights_y,
            &raw const scaled_bottom_left_y,
            &raw const round,
        );
        dst = dst.offset(stride);
    }
    for y_mask in (0x01000100..0x0F0E0F0F).step_by(0x02020202) {
        let y_select: __m128i = _mm_set1_epi32(y_mask);
        let weights_y: __m128i = _mm_shuffle_epi8(weights4, y_select);
        let scaled_bottom_left_y: __m128i = _mm_shuffle_epi8(scaled_bottom_left4, y_select);
        write_smooth_directional_sum8(
            dst,
            &raw const top,
            &raw const weights_y,
            &raw const scaled_bottom_left_y,
            &raw const round,
        );
        dst = dst.offset(stride);
    }
}

#[no_mangle]
#[target_feature(enable = "ssse3")]
pub unsafe extern "C-unwind" fn aom_smooth_v_predictor_16x4_ssse3(
    mut dst: *mut u8,
    stride: isize,
    top_row: *const u8,
    left_column: *const u8,
) {
    let bottom_left: __m128i = _mm_set1_epi16(*left_column.offset(3) as i16);
    let weights: __m128i =
        cvtepu8_epi16(Load4(smooth_weights.0.as_ptr() as *const core::ffi::c_void));
    let scale: __m128i = _mm_set1_epi16((1 << SMOOTH_WEIGHT_LOG2_SCALE) as i16);
    let inverted_weights: __m128i = _mm_sub_epi16(scale, weights);
    let scaled_bottom_left: __m128i = _mm_mullo_epi16(inverted_weights, bottom_left);
    let round: __m128i = _mm_set1_epi16(128);
    let top: __m128i = LoadUnaligned16(top_row as *const core::ffi::c_void);
    let top_lo: __m128i = cvtepu8_epi16(top);
    let top_hi: __m128i = cvtepu8_epi16(_mm_srli_si128::<8>(top));
    let mut y_select: __m128i = _mm_set1_epi32(0x01000100);
    let mut weights_y: __m128i = _mm_shuffle_epi8(weights, y_select);
    let mut scaled_bottom_left_y: __m128i = _mm_shuffle_epi8(scaled_bottom_left, y_select);
    write_smooth_directional_sum16(
        dst,
        top_lo,
        top_hi,
        weights_y,
        weights_y,
        scaled_bottom_left_y,
        scaled_bottom_left_y,
        round,
    );
    dst = dst.offset(stride);
    y_select = _mm_set1_epi32(0x03020302);
    weights_y = _mm_shuffle_epi8(weights, y_select);
    scaled_bottom_left_y = _mm_shuffle_epi8(scaled_bottom_left, y_select);
    write_smooth_directional_sum16(
        dst,
        top_lo,
        top_hi,
        weights_y,
        weights_y,
        scaled_bottom_left_y,
        scaled_bottom_left_y,
        round,
    );
    dst = dst.offset(stride);
    y_select = _mm_set1_epi32(0x05040504);
    weights_y = _mm_shuffle_epi8(weights, y_select);
    scaled_bottom_left_y = _mm_shuffle_epi8(scaled_bottom_left, y_select);
    write_smooth_directional_sum16(
        dst,
        top_lo,
        top_hi,
        weights_y,
        weights_y,
        scaled_bottom_left_y,
        scaled_bottom_left_y,
        round,
    );
    dst = dst.offset(stride);
    y_select = _mm_set1_epi32(0x07060706);
    weights_y = _mm_shuffle_epi8(weights, y_select);
    scaled_bottom_left_y = _mm_shuffle_epi8(scaled_bottom_left, y_select);
    write_smooth_directional_sum16(
        dst,
        top_lo,
        top_hi,
        weights_y,
        weights_y,
        scaled_bottom_left_y,
        scaled_bottom_left_y,
        round,
    );
}

// !CONFIG_REALTIME_ONLY || CONFIG_AV1_DECODER
#[no_mangle]
#[target_feature(enable = "ssse3")]
pub unsafe extern "C-unwind" fn aom_smooth_v_predictor_16x8_ssse3(
    mut dst: *mut u8,
    stride: isize,
    top_row: *const u8,
    left_column: *const u8,
) {
    let bottom_left: __m128i = _mm_set1_epi16(*left_column.offset(7) as i16);
    let weights: __m128i = cvtepu8_epi16(LoadLo8(
        smooth_weights.0.as_ptr().offset(4) as *const core::ffi::c_void
    ));
    let scale: __m128i = _mm_set1_epi16((1 << SMOOTH_WEIGHT_LOG2_SCALE) as i16);
    let inverted_weights: __m128i = _mm_sub_epi16(scale, weights);
    let scaled_bottom_left: __m128i = _mm_mullo_epi16(inverted_weights, bottom_left);
    let round: __m128i = _mm_set1_epi16(128);
    let top: __m128i = LoadUnaligned16(top_row as *const core::ffi::c_void);
    let top_lo: __m128i = cvtepu8_epi16(top);
    let top_hi: __m128i = cvtepu8_epi16(_mm_srli_si128::<8>(top));
    for y_mask in (0x01000100..0x0F0E0F0F).step_by(0x02020202) {
        let y_select: __m128i = _mm_set1_epi32(y_mask);
        let weights_y: __m128i = _mm_shuffle_epi8(weights, y_select);
        let scaled_bottom_left_y: __m128i = _mm_shuffle_epi8(scaled_bottom_left, y_select);
        write_smooth_directional_sum16(
            dst,
            top_lo,
            top_hi,
            weights_y,
            weights_y,
            scaled_bottom_left_y,
            scaled_bottom_left_y,
            round,
        );
        dst = dst.offset(stride);
    }
}

#[no_mangle]
#[target_feature(enable = "ssse3")]
pub unsafe extern "C-unwind" fn aom_smooth_v_predictor_16x16_ssse3(
    mut dst: *mut u8,
    stride: isize,
    top_row: *const u8,
    left_column: *const u8,
) {
    let bottom_left: __m128i = _mm_set1_epi16(*left_column.offset(15) as i16);
    let zero: __m128i = _mm_setzero_si128();
    let scale: __m128i = _mm_set1_epi16((1 << SMOOTH_WEIGHT_LOG2_SCALE) as i16);
    let weights: __m128i =
        LoadUnaligned16(smooth_weights.0.as_ptr().offset(12) as *const core::ffi::c_void);
    let weights_lo: __m128i = cvtepu8_epi16(weights);
    let weights_hi: __m128i = _mm_unpackhi_epi8(weights, zero);
    let inverted_weights_lo: __m128i = _mm_sub_epi16(scale, weights_lo);
    let inverted_weights_hi: __m128i = _mm_sub_epi16(scale, weights_hi);
    let scaled_bottom_left_lo: __m128i = _mm_mullo_epi16(inverted_weights_lo, bottom_left);
    let scaled_bottom_left_hi: __m128i = _mm_mullo_epi16(inverted_weights_hi, bottom_left);
    let round: __m128i = _mm_set1_epi16(128);
    let top: __m128i = LoadUnaligned16(top_row as *const core::ffi::c_void);
    let top_lo: __m128i = cvtepu8_epi16(top);
    let top_hi: __m128i = _mm_unpackhi_epi8(top, zero);
    for y_mask in (0x01000100..0x0F0E0F0F).step_by(0x02020202) {
        let y_select: __m128i = _mm_set1_epi32(y_mask);
        let weights_y: __m128i = _mm_shuffle_epi8(weights_lo, y_select);
        let scaled_bottom_left_y: __m128i = _mm_shuffle_epi8(scaled_bottom_left_lo, y_select);
        write_smooth_directional_sum16(
            dst,
            top_lo,
            top_hi,
            weights_y,
            weights_y,
            scaled_bottom_left_y,
            scaled_bottom_left_y,
            round,
        );
        dst = dst.offset(stride);
    }
    for y_mask in (0x01000100..0x0F0E0F0F).step_by(0x02020202) {
        let y_select: __m128i = _mm_set1_epi32(y_mask);
        let weights_y: __m128i = _mm_shuffle_epi8(weights_hi, y_select);
        let scaled_bottom_left_y: __m128i = _mm_shuffle_epi8(scaled_bottom_left_hi, y_select);
        write_smooth_directional_sum16(
            dst,
            top_lo,
            top_hi,
            weights_y,
            weights_y,
            scaled_bottom_left_y,
            scaled_bottom_left_y,
            round,
        );
        dst = dst.offset(stride);
    }
}

#[no_mangle]
#[target_feature(enable = "ssse3")]
pub unsafe extern "C-unwind" fn aom_smooth_v_predictor_16x32_ssse3(
    mut dst: *mut u8,
    stride: isize,
    top_row: *const u8,
    left_column: *const u8,
) {
    let bottom_left: __m128i = _mm_set1_epi16(*left_column.offset(31) as i16);
    let weights_lo: __m128i =
        LoadUnaligned16(smooth_weights.0.as_ptr().offset(28) as *const core::ffi::c_void);
    let weights_hi: __m128i =
        LoadUnaligned16(smooth_weights.0.as_ptr().offset(44) as *const core::ffi::c_void);
    let scale: __m128i = _mm_set1_epi16((1 << SMOOTH_WEIGHT_LOG2_SCALE) as i16);
    let zero: __m128i = _mm_setzero_si128();
    let weights1: __m128i = cvtepu8_epi16(weights_lo);
    let weights2: __m128i = _mm_unpackhi_epi8(weights_lo, zero);
    let weights3: __m128i = cvtepu8_epi16(weights_hi);
    let weights4: __m128i = _mm_unpackhi_epi8(weights_hi, zero);
    let inverted_weights1: __m128i = _mm_sub_epi16(scale, weights1);
    let inverted_weights2: __m128i = _mm_sub_epi16(scale, weights2);
    let inverted_weights3: __m128i = _mm_sub_epi16(scale, weights3);
    let inverted_weights4: __m128i = _mm_sub_epi16(scale, weights4);
    let scaled_bottom_left1: __m128i = _mm_mullo_epi16(inverted_weights1, bottom_left);
    let scaled_bottom_left2: __m128i = _mm_mullo_epi16(inverted_weights2, bottom_left);
    let scaled_bottom_left3: __m128i = _mm_mullo_epi16(inverted_weights3, bottom_left);
    let scaled_bottom_left4: __m128i = _mm_mullo_epi16(inverted_weights4, bottom_left);
    let round: __m128i = _mm_set1_epi16(128);
    let top: __m128i = LoadUnaligned16(top_row as *const core::ffi::c_void);
    let top_lo: __m128i = cvtepu8_epi16(top);
    let top_hi: __m128i = _mm_unpackhi_epi8(top, zero);
    for y_mask in (0x01000100..0x0F0E0F0F).step_by(0x02020202) {
        let y_select: __m128i = _mm_set1_epi32(y_mask);
        let weights_y: __m128i = _mm_shuffle_epi8(weights1, y_select);
        let scaled_bottom_left_y: __m128i = _mm_shuffle_epi8(scaled_bottom_left1, y_select);
        write_smooth_directional_sum16(
            dst,
            top_lo,
            top_hi,
            weights_y,
            weights_y,
            scaled_bottom_left_y,
            scaled_bottom_left_y,
            round,
        );
        dst = dst.offset(stride);
    }
    for y_mask in (0x01000100..0x0F0E0F0F).step_by(0x02020202) {
        let y_select: __m128i = _mm_set1_epi32(y_mask);
        let weights_y: __m128i = _mm_shuffle_epi8(weights2, y_select);
        let scaled_bottom_left_y: __m128i = _mm_shuffle_epi8(scaled_bottom_left2, y_select);
        write_smooth_directional_sum16(
            dst,
            top_lo,
            top_hi,
            weights_y,
            weights_y,
            scaled_bottom_left_y,
            scaled_bottom_left_y,
            round,
        );
        dst = dst.offset(stride);
    }
    for y_mask in (0x01000100..0x0F0E0F0F).step_by(0x02020202) {
        let y_select: __m128i = _mm_set1_epi32(y_mask);
        let weights_y: __m128i = _mm_shuffle_epi8(weights3, y_select);
        let scaled_bottom_left_y: __m128i = _mm_shuffle_epi8(scaled_bottom_left3, y_select);
        write_smooth_directional_sum16(
            dst,
            top_lo,
            top_hi,
            weights_y,
            weights_y,
            scaled_bottom_left_y,
            scaled_bottom_left_y,
            round,
        );
        dst = dst.offset(stride);
    }
    for y_mask in (0x01000100..0x0F0E0F0F).step_by(0x02020202) {
        let y_select: __m128i = _mm_set1_epi32(y_mask);
        let weights_y: __m128i = _mm_shuffle_epi8(weights4, y_select);
        let scaled_bottom_left_y: __m128i = _mm_shuffle_epi8(scaled_bottom_left4, y_select);
        write_smooth_directional_sum16(
            dst,
            top_lo,
            top_hi,
            weights_y,
            weights_y,
            scaled_bottom_left_y,
            scaled_bottom_left_y,
            round,
        );
        dst = dst.offset(stride);
    }
}

#[no_mangle]
#[target_feature(enable = "ssse3")]
pub unsafe extern "C-unwind" fn aom_smooth_v_predictor_16x64_ssse3(
    mut dst: *mut u8,
    stride: isize,
    top_row: *const u8,
    left_column: *const u8,
) {
    let bottom_left: __m128i = _mm_set1_epi16(*left_column.offset(63) as i16);
    let scale: __m128i = _mm_set1_epi16((1 << SMOOTH_WEIGHT_LOG2_SCALE) as i16);
    let round: __m128i = _mm_set1_epi16(128);
    let zero: __m128i = _mm_setzero_si128();
    let top: __m128i = LoadUnaligned16(top_row as *const core::ffi::c_void);
    let top_lo: __m128i = cvtepu8_epi16(top);
    let top_hi: __m128i = _mm_unpackhi_epi8(top, zero);
    let mut weights_base_ptr: *const u8 = smooth_weights.0.as_ptr().offset(60);
    for left_offset in (0..64).step_by(16) {
        let weights: __m128i = LoadUnaligned16(
            weights_base_ptr.offset(left_offset as isize) as *const core::ffi::c_void
        );
        let weights_lo: __m128i = cvtepu8_epi16(weights);
        let weights_hi: __m128i = _mm_unpackhi_epi8(weights, zero);
        let inverted_weights_lo: __m128i = _mm_sub_epi16(scale, weights_lo);
        let inverted_weights_hi: __m128i = _mm_sub_epi16(scale, weights_hi);
        let scaled_bottom_left_lo: __m128i = _mm_mullo_epi16(inverted_weights_lo, bottom_left);
        let scaled_bottom_left_hi: __m128i = _mm_mullo_epi16(inverted_weights_hi, bottom_left);
        for y_mask in (0x01000100..0x0F0E0F0F).step_by(0x02020202) {
            let y_select: __m128i = _mm_set1_epi32(y_mask);
            let weights_y: __m128i = _mm_shuffle_epi8(weights_lo, y_select);
            let scaled_bottom_left_y: __m128i = _mm_shuffle_epi8(scaled_bottom_left_lo, y_select);
            write_smooth_directional_sum16(
                dst,
                top_lo,
                top_hi,
                weights_y,
                weights_y,
                scaled_bottom_left_y,
                scaled_bottom_left_y,
                round,
            );
            dst = dst.offset(stride);
        }
        for y_mask in (0x01000100..0x0F0E0F0F).step_by(0x02020202) {
            let y_select: __m128i = _mm_set1_epi32(y_mask);
            let weights_y: __m128i = _mm_shuffle_epi8(weights_hi, y_select);
            let scaled_bottom_left_y: __m128i = _mm_shuffle_epi8(scaled_bottom_left_hi, y_select);
            write_smooth_directional_sum16(
                dst,
                top_lo,
                top_hi,
                weights_y,
                weights_y,
                scaled_bottom_left_y,
                scaled_bottom_left_y,
                round,
            );
            dst = dst.offset(stride);
        }
    }
}

#[no_mangle]
#[target_feature(enable = "ssse3")]
pub unsafe extern "C-unwind" fn aom_smooth_v_predictor_32x8_ssse3(
    mut dst: *mut u8,
    stride: isize,
    top_row: *const u8,
    left_column: *const u8,
) {
    let zero: __m128i = _mm_setzero_si128();
    let bottom_left: __m128i = _mm_set1_epi16(*left_column.offset(7) as i16);
    let top_lo: __m128i = LoadUnaligned16(top_row as *const core::ffi::c_void);
    let top_hi: __m128i = LoadUnaligned16(top_row.offset(16) as *const core::ffi::c_void);
    let top1: __m128i = cvtepu8_epi16(top_lo);
    let top2: __m128i = _mm_unpackhi_epi8(top_lo, zero);
    let top3: __m128i = cvtepu8_epi16(top_hi);
    let top4: __m128i = _mm_unpackhi_epi8(top_hi, zero);
    let mut scale: __m128i = _mm_set1_epi16(256);
    let weights: __m128i = cvtepu8_epi16(LoadLo8(
        smooth_weights.0.as_ptr().offset(4) as *const core::ffi::c_void
    ));
    let inverted_weights: __m128i = _mm_sub_epi16(scale, weights);
    let scaled_bottom_left: __m128i = _mm_mullo_epi16(inverted_weights, bottom_left);
    let round: __m128i = _mm_set1_epi16((1 << (SMOOTH_WEIGHT_LOG2_SCALE - 1)) as i16);
    for y_mask in (0x01000100..0x0F0E0F0F).step_by(0x02020202) {
        let mut y_select: __m128i = _mm_set1_epi32(y_mask);
        let weights_y: __m128i = _mm_shuffle_epi8(weights, y_select);
        let scaled_bottom_left_y: __m128i = _mm_shuffle_epi8(scaled_bottom_left, y_select);
        write_smooth_directional_sum16(
            dst,
            top1,
            top2,
            weights_y,
            weights_y,
            scaled_bottom_left_y,
            scaled_bottom_left_y,
            round,
        );
        write_smooth_directional_sum16(
            dst.offset(16),
            top3,
            top4,
            weights_y,
            weights_y,
            scaled_bottom_left_y,
            scaled_bottom_left_y,
            round,
        );
        dst = dst.offset(stride);
    }
}

// !CONFIG_REALTIME_ONLY || CONFIG_AV1_DECODER
#[no_mangle]
#[target_feature(enable = "ssse3")]
pub unsafe extern "C-unwind" fn aom_smooth_v_predictor_32x16_ssse3(
    mut dst: *mut u8,
    stride: isize,
    top_row: *const u8,
    left_column: *const u8,
) {
    let zero: __m128i = _mm_setzero_si128();
    let bottom_left: __m128i = _mm_set1_epi16(*left_column.offset(15) as i16);
    let top_lo: __m128i = LoadUnaligned16(top_row as *const core::ffi::c_void);
    let top_hi: __m128i = LoadUnaligned16(top_row.offset(16) as *const core::ffi::c_void);
    let top1: __m128i = cvtepu8_epi16(top_lo);
    let top2: __m128i = _mm_unpackhi_epi8(top_lo, zero);
    let top3: __m128i = cvtepu8_epi16(top_hi);
    let top4: __m128i = _mm_unpackhi_epi8(top_hi, zero);
    let weights: __m128i =
        LoadUnaligned16(smooth_weights.0.as_ptr().offset(12) as *const core::ffi::c_void);
    let weights1: __m128i = cvtepu8_epi16(weights);
    let weights2: __m128i = _mm_unpackhi_epi8(weights, zero);
    let scale: __m128i = _mm_set1_epi16((1 << SMOOTH_WEIGHT_LOG2_SCALE) as i16);
    let inverted_weights1: __m128i = _mm_sub_epi16(scale, weights1);
    let inverted_weights2: __m128i = _mm_sub_epi16(scale, weights2);
    let scaled_bottom_left1: __m128i = _mm_mullo_epi16(inverted_weights1, bottom_left);
    let scaled_bottom_left2: __m128i = _mm_mullo_epi16(inverted_weights2, bottom_left);
    let round: __m128i = _mm_set1_epi16((1 << (SMOOTH_WEIGHT_LOG2_SCALE - 1)) as i16);
    for y_mask in (0x01000100..0x0F0E0F0F).step_by(0x02020202) {
        let mut y_select: __m128i = _mm_set1_epi32(y_mask);
        let weights_y: __m128i = _mm_shuffle_epi8(weights1, y_select);
        let scaled_bottom_left_y: __m128i = _mm_shuffle_epi8(scaled_bottom_left1, y_select);
        write_smooth_directional_sum16(
            dst,
            top1,
            top2,
            weights_y,
            weights_y,
            scaled_bottom_left_y,
            scaled_bottom_left_y,
            round,
        );
        write_smooth_directional_sum16(
            dst.offset(16),
            top3,
            top4,
            weights_y,
            weights_y,
            scaled_bottom_left_y,
            scaled_bottom_left_y,
            round,
        );
        dst = dst.offset(stride);
    }
    for y_mask in (0x01000100..0x0F0E0F0F).step_by(0x02020202) {
        let mut y_select: __m128i = _mm_set1_epi32(y_mask);
        let weights_y: __m128i = _mm_shuffle_epi8(weights2, y_select);
        let scaled_bottom_left_y: __m128i = _mm_shuffle_epi8(scaled_bottom_left2, y_select);
        write_smooth_directional_sum16(
            dst,
            top1,
            top2,
            weights_y,
            weights_y,
            scaled_bottom_left_y,
            scaled_bottom_left_y,
            round,
        );
        write_smooth_directional_sum16(
            dst.offset(16),
            top3,
            top4,
            weights_y,
            weights_y,
            scaled_bottom_left_y,
            scaled_bottom_left_y,
            round,
        );
        dst = dst.offset(stride);
    }
}

#[no_mangle]
#[target_feature(enable = "ssse3")]
pub unsafe extern "C-unwind" fn aom_smooth_v_predictor_32x32_ssse3(
    mut dst: *mut u8,
    stride: isize,
    top_row: *const u8,
    left_column: *const u8,
) {
    let bottom_left: __m128i = _mm_set1_epi16(*left_column.offset(31) as i16);
    let weights_lo: __m128i =
        LoadUnaligned16(smooth_weights.0.as_ptr().offset(28) as *const core::ffi::c_void);
    let weights_hi: __m128i =
        LoadUnaligned16(smooth_weights.0.as_ptr().offset(44) as *const core::ffi::c_void);
    let zero: __m128i = _mm_setzero_si128();
    let scale: __m128i = _mm_set1_epi16((1 << SMOOTH_WEIGHT_LOG2_SCALE) as i16);
    let top_lo: __m128i = LoadUnaligned16(top_row as *const core::ffi::c_void);
    let top_hi: __m128i = LoadUnaligned16(top_row.offset(16) as *const core::ffi::c_void);
    let top1: __m128i = cvtepu8_epi16(top_lo);
    let top2: __m128i = _mm_unpackhi_epi8(top_lo, zero);
    let top3: __m128i = cvtepu8_epi16(top_hi);
    let top4: __m128i = _mm_unpackhi_epi8(top_hi, zero);
    let weights1: __m128i = cvtepu8_epi16(weights_lo);
    let weights2: __m128i = _mm_unpackhi_epi8(weights_lo, zero);
    let weights3: __m128i = cvtepu8_epi16(weights_hi);
    let weights4: __m128i = _mm_unpackhi_epi8(weights_hi, zero);
    let inverted_weights1: __m128i = _mm_sub_epi16(scale, weights1);
    let inverted_weights2: __m128i = _mm_sub_epi16(scale, weights2);
    let inverted_weights3: __m128i = _mm_sub_epi16(scale, weights3);
    let inverted_weights4: __m128i = _mm_sub_epi16(scale, weights4);
    let scaled_bottom_left1: __m128i = _mm_mullo_epi16(inverted_weights1, bottom_left);
    let scaled_bottom_left2: __m128i = _mm_mullo_epi16(inverted_weights2, bottom_left);
    let scaled_bottom_left3: __m128i = _mm_mullo_epi16(inverted_weights3, bottom_left);
    let scaled_bottom_left4: __m128i = _mm_mullo_epi16(inverted_weights4, bottom_left);
    let round: __m128i = _mm_set1_epi16((1 << (SMOOTH_WEIGHT_LOG2_SCALE - 1)) as i16);
    for y_mask in (0x01000100..0x0F0E0F0F).step_by(0x02020202) {
        let y_select: __m128i = _mm_set1_epi32(y_mask);
        let weights_y: __m128i = _mm_shuffle_epi8(weights1, y_select);
        let scaled_bottom_left_y: __m128i = _mm_shuffle_epi8(scaled_bottom_left1, y_select);
        write_smooth_directional_sum16(
            dst,
            top1,
            top2,
            weights_y,
            weights_y,
            scaled_bottom_left_y,
            scaled_bottom_left_y,
            round,
        );
        write_smooth_directional_sum16(
            dst.offset(16),
            top3,
            top4,
            weights_y,
            weights_y,
            scaled_bottom_left_y,
            scaled_bottom_left_y,
            round,
        );
        dst = dst.offset(stride);
    }
    for y_mask in (0x01000100..0x0F0E0F0F).step_by(0x02020202) {
        let y_select: __m128i = _mm_set1_epi32(y_mask);
        let weights_y: __m128i = _mm_shuffle_epi8(weights2, y_select);
        let scaled_bottom_left_y: __m128i = _mm_shuffle_epi8(scaled_bottom_left2, y_select);
        write_smooth_directional_sum16(
            dst,
            top1,
            top2,
            weights_y,
            weights_y,
            scaled_bottom_left_y,
            scaled_bottom_left_y,
            round,
        );
        write_smooth_directional_sum16(
            dst.offset(16),
            top3,
            top4,
            weights_y,
            weights_y,
            scaled_bottom_left_y,
            scaled_bottom_left_y,
            round,
        );
        dst = dst.offset(stride);
    }
    for y_mask in (0x01000100..0x0F0E0F0F).step_by(0x02020202) {
        let y_select: __m128i = _mm_set1_epi32(y_mask);
        let weights_y: __m128i = _mm_shuffle_epi8(weights3, y_select);
        let scaled_bottom_left_y: __m128i = _mm_shuffle_epi8(scaled_bottom_left3, y_select);
        write_smooth_directional_sum16(
            dst,
            top1,
            top2,
            weights_y,
            weights_y,
            scaled_bottom_left_y,
            scaled_bottom_left_y,
            round,
        );
        write_smooth_directional_sum16(
            dst.offset(16),
            top3,
            top4,
            weights_y,
            weights_y,
            scaled_bottom_left_y,
            scaled_bottom_left_y,
            round,
        );
        dst = dst.offset(stride);
    }
    for y_mask in (0x01000100..0x0F0E0F0F).step_by(0x02020202) {
        let y_select: __m128i = _mm_set1_epi32(y_mask);
        let weights_y: __m128i = _mm_shuffle_epi8(weights4, y_select);
        let scaled_bottom_left_y: __m128i = _mm_shuffle_epi8(scaled_bottom_left4, y_select);
        write_smooth_directional_sum16(
            dst,
            top1,
            top2,
            weights_y,
            weights_y,
            scaled_bottom_left_y,
            scaled_bottom_left_y,
            round,
        );
        write_smooth_directional_sum16(
            dst.offset(16),
            top3,
            top4,
            weights_y,
            weights_y,
            scaled_bottom_left_y,
            scaled_bottom_left_y,
            round,
        );
        dst = dst.offset(stride);
    }
}

#[no_mangle]
#[target_feature(enable = "ssse3")]
pub unsafe extern "C-unwind" fn aom_smooth_v_predictor_32x64_ssse3(
    mut dst: *mut u8,
    stride: isize,
    top_row: *const u8,
    left_column: *const u8,
) {
    let zero: __m128i = _mm_setzero_si128();
    let bottom_left: __m128i = _mm_set1_epi16(*left_column.offset(63) as i16);
    let top_lo: __m128i = LoadUnaligned16(top_row as *const core::ffi::c_void);
    let top_hi: __m128i = LoadUnaligned16(top_row.offset(16) as *const core::ffi::c_void);
    let top1: __m128i = cvtepu8_epi16(top_lo);
    let top2: __m128i = _mm_unpackhi_epi8(top_lo, zero);
    let top3: __m128i = cvtepu8_epi16(top_hi);
    let top4: __m128i = _mm_unpackhi_epi8(top_hi, zero);
    let scale: __m128i = _mm_set1_epi16((1 << SMOOTH_WEIGHT_LOG2_SCALE) as i16);
    let round: __m128i = _mm_set1_epi16((1 << (SMOOTH_WEIGHT_LOG2_SCALE - 1)) as i16);
    let mut weights_base_ptr: *const u8 = smooth_weights.0.as_ptr().offset(60);
    for left_offset in (0..64).step_by(16) {
        let weights: __m128i = LoadUnaligned16(
            weights_base_ptr.offset(left_offset as isize) as *const core::ffi::c_void
        );
        let weights_lo: __m128i = cvtepu8_epi16(weights);
        let weights_hi: __m128i = _mm_unpackhi_epi8(weights, zero);
        let inverted_weights_lo: __m128i = _mm_sub_epi16(scale, weights_lo);
        let inverted_weights_hi: __m128i = _mm_sub_epi16(scale, weights_hi);
        let scaled_bottom_left_lo: __m128i = _mm_mullo_epi16(inverted_weights_lo, bottom_left);
        let scaled_bottom_left_hi: __m128i = _mm_mullo_epi16(inverted_weights_hi, bottom_left);
        for y_mask in (0x01000100..0x0F0E0F0F).step_by(0x02020202) {
            let y_select: __m128i = _mm_set1_epi32(y_mask);
            let weights_y: __m128i = _mm_shuffle_epi8(weights_lo, y_select);
            let scaled_bottom_left_y: __m128i = _mm_shuffle_epi8(scaled_bottom_left_lo, y_select);
            write_smooth_directional_sum16(
                dst,
                top1,
                top2,
                weights_y,
                weights_y,
                scaled_bottom_left_y,
                scaled_bottom_left_y,
                round,
            );
            write_smooth_directional_sum16(
                dst.offset(16),
                top3,
                top4,
                weights_y,
                weights_y,
                scaled_bottom_left_y,
                scaled_bottom_left_y,
                round,
            );
            dst = dst.offset(stride);
        }
        for y_mask in (0x01000100..0x0F0E0F0F).step_by(0x02020202) {
            let y_select: __m128i = _mm_set1_epi32(y_mask);
            let weights_y: __m128i = _mm_shuffle_epi8(weights_hi, y_select);
            let scaled_bottom_left_y: __m128i = _mm_shuffle_epi8(scaled_bottom_left_hi, y_select);
            write_smooth_directional_sum16(
                dst,
                top1,
                top2,
                weights_y,
                weights_y,
                scaled_bottom_left_y,
                scaled_bottom_left_y,
                round,
            );
            write_smooth_directional_sum16(
                dst.offset(16),
                top3,
                top4,
                weights_y,
                weights_y,
                scaled_bottom_left_y,
                scaled_bottom_left_y,
                round,
            );
            dst = dst.offset(stride);
        }
    }
}

#[no_mangle]
#[target_feature(enable = "ssse3")]
pub unsafe extern "C-unwind" fn aom_smooth_v_predictor_64x16_ssse3(
    mut dst: *mut u8,
    stride: isize,
    top_row: *const u8,
    left_column: *const u8,
) {
    let bottom_left: __m128i = _mm_set1_epi16(*left_column.offset(15) as i16);
    let scale: __m128i = _mm_set1_epi16((1 << SMOOTH_WEIGHT_LOG2_SCALE) as i16);
    let zero: __m128i = _mm_setzero_si128();
    let top_lolo: __m128i = LoadUnaligned16(top_row as *const core::ffi::c_void);
    let top_lohi: __m128i = LoadUnaligned16(top_row.offset(16) as *const core::ffi::c_void);
    let top1: __m128i = cvtepu8_epi16(top_lolo);
    let top2: __m128i = _mm_unpackhi_epi8(top_lolo, zero);
    let top3: __m128i = cvtepu8_epi16(top_lohi);
    let top4: __m128i = _mm_unpackhi_epi8(top_lohi, zero);
    let weights: __m128i =
        LoadUnaligned16(smooth_weights.0.as_ptr().offset(12) as *const core::ffi::c_void);
    let weights1: __m128i = cvtepu8_epi16(weights);
    let weights2: __m128i = _mm_unpackhi_epi8(weights, zero);
    let inverted_weights1: __m128i = _mm_sub_epi16(scale, weights1);
    let inverted_weights2: __m128i = _mm_sub_epi16(scale, weights2);
    let top_hilo: __m128i = LoadUnaligned16(top_row.offset(32) as *const core::ffi::c_void);
    let top_hihi: __m128i = LoadUnaligned16(top_row.offset(48) as *const core::ffi::c_void);
    let top5: __m128i = cvtepu8_epi16(top_hilo);
    let top6: __m128i = _mm_unpackhi_epi8(top_hilo, zero);
    let top7: __m128i = cvtepu8_epi16(top_hihi);
    let top8: __m128i = _mm_unpackhi_epi8(top_hihi, zero);
    let scaled_bottom_left1: __m128i = _mm_mullo_epi16(inverted_weights1, bottom_left);
    let scaled_bottom_left2: __m128i = _mm_mullo_epi16(inverted_weights2, bottom_left);
    let round: __m128i = _mm_set1_epi16((1 << (SMOOTH_WEIGHT_LOG2_SCALE - 1)) as i16);
    for y_mask in (0x01000100..0x0F0E0F0F).step_by(0x02020202) {
        let y_select: __m128i = _mm_set1_epi32(y_mask);
        let weights_y: __m128i = _mm_shuffle_epi8(weights1, y_select);
        let scaled_bottom_left_y: __m128i = _mm_shuffle_epi8(scaled_bottom_left1, y_select);
        write_smooth_directional_sum16(
            dst,
            top1,
            top2,
            weights_y,
            weights_y,
            scaled_bottom_left_y,
            scaled_bottom_left_y,
            round,
        );
        write_smooth_directional_sum16(
            dst.offset(16),
            top3,
            top4,
            weights_y,
            weights_y,
            scaled_bottom_left_y,
            scaled_bottom_left_y,
            round,
        );
        write_smooth_directional_sum16(
            dst.offset(32),
            top5,
            top6,
            weights_y,
            weights_y,
            scaled_bottom_left_y,
            scaled_bottom_left_y,
            round,
        );
        write_smooth_directional_sum16(
            dst.offset(48),
            top7,
            top8,
            weights_y,
            weights_y,
            scaled_bottom_left_y,
            scaled_bottom_left_y,
            round,
        );
        dst = dst.offset(stride);
    }
    for y_mask in (0x01000100..0x0F0E0F0F).step_by(0x02020202) {
        let y_select: __m128i = _mm_set1_epi32(y_mask);
        let weights_y: __m128i = _mm_shuffle_epi8(weights2, y_select);
        let scaled_bottom_left_y: __m128i = _mm_shuffle_epi8(scaled_bottom_left2, y_select);
        write_smooth_directional_sum16(
            dst,
            top1,
            top2,
            weights_y,
            weights_y,
            scaled_bottom_left_y,
            scaled_bottom_left_y,
            round,
        );
        write_smooth_directional_sum16(
            dst.offset(16),
            top3,
            top4,
            weights_y,
            weights_y,
            scaled_bottom_left_y,
            scaled_bottom_left_y,
            round,
        );
        write_smooth_directional_sum16(
            dst.offset(32),
            top5,
            top6,
            weights_y,
            weights_y,
            scaled_bottom_left_y,
            scaled_bottom_left_y,
            round,
        );
        write_smooth_directional_sum16(
            dst.offset(48),
            top7,
            top8,
            weights_y,
            weights_y,
            scaled_bottom_left_y,
            scaled_bottom_left_y,
            round,
        );
        dst = dst.offset(stride);
    }
}

// !CONFIG_REALTIME_ONLY || CONFIG_AV1_DECODER
#[no_mangle]
#[target_feature(enable = "ssse3")]
pub unsafe extern "C-unwind" fn aom_smooth_v_predictor_64x32_ssse3(
    mut dst: *mut u8,
    stride: isize,
    top_row: *const u8,
    left_column: *const u8,
) {
    let zero: __m128i = _mm_setzero_si128();
    let bottom_left: __m128i = _mm_set1_epi16(*left_column.offset(31) as i16);
    let top_lolo: __m128i = LoadUnaligned16(top_row as *const core::ffi::c_void);
    let top_lohi: __m128i = LoadUnaligned16(top_row.offset(16) as *const core::ffi::c_void);
    let top1: __m128i = cvtepu8_epi16(top_lolo);
    let top2: __m128i = _mm_unpackhi_epi8(top_lolo, zero);
    let top3: __m128i = cvtepu8_epi16(top_lohi);
    let top4: __m128i = _mm_unpackhi_epi8(top_lohi, zero);
    let top_hilo: __m128i = LoadUnaligned16(top_row.offset(32) as *const core::ffi::c_void);
    let top_hihi: __m128i = LoadUnaligned16(top_row.offset(48) as *const core::ffi::c_void);
    let top5: __m128i = cvtepu8_epi16(top_hilo);
    let top6: __m128i = _mm_unpackhi_epi8(top_hilo, zero);
    let top7: __m128i = cvtepu8_epi16(top_hihi);
    let top8: __m128i = _mm_unpackhi_epi8(top_hihi, zero);
    let weights_lo: __m128i =
        LoadUnaligned16(smooth_weights.0.as_ptr().offset(28) as *const core::ffi::c_void);
    let weights_hi: __m128i =
        LoadUnaligned16(smooth_weights.0.as_ptr().offset(44) as *const core::ffi::c_void);
    let weights1: __m128i = cvtepu8_epi16(weights_lo);
    let weights2: __m128i = _mm_unpackhi_epi8(weights_lo, zero);
    let weights3: __m128i = cvtepu8_epi16(weights_hi);
    let weights4: __m128i = _mm_unpackhi_epi8(weights_hi, zero);
    let scale: __m128i = _mm_set1_epi16((1 << SMOOTH_WEIGHT_LOG2_SCALE) as i16);
    let inverted_weights1: __m128i = _mm_sub_epi16(scale, weights1);
    let inverted_weights2: __m128i = _mm_sub_epi16(scale, weights2);
    let inverted_weights3: __m128i = _mm_sub_epi16(scale, weights3);
    let inverted_weights4: __m128i = _mm_sub_epi16(scale, weights4);
    let scaled_bottom_left1: __m128i = _mm_mullo_epi16(inverted_weights1, bottom_left);
    let scaled_bottom_left2: __m128i = _mm_mullo_epi16(inverted_weights2, bottom_left);
    let scaled_bottom_left3: __m128i = _mm_mullo_epi16(inverted_weights3, bottom_left);
    let scaled_bottom_left4: __m128i = _mm_mullo_epi16(inverted_weights4, bottom_left);
    let round: __m128i = _mm_set1_epi16((1 << (SMOOTH_WEIGHT_LOG2_SCALE - 1)) as i16);
    for y_mask in (0x01000100..0x0F0E0F0F).step_by(0x02020202) {
        let y_select: __m128i = _mm_set1_epi32(y_mask);
        let weights_y: __m128i = _mm_shuffle_epi8(weights1, y_select);
        let scaled_bottom_left_y: __m128i = _mm_shuffle_epi8(scaled_bottom_left1, y_select);
        write_smooth_directional_sum16(
            dst,
            top1,
            top2,
            weights_y,
            weights_y,
            scaled_bottom_left_y,
            scaled_bottom_left_y,
            round,
        );
        write_smooth_directional_sum16(
            dst.offset(16),
            top3,
            top4,
            weights_y,
            weights_y,
            scaled_bottom_left_y,
            scaled_bottom_left_y,
            round,
        );
        write_smooth_directional_sum16(
            dst.offset(32),
            top5,
            top6,
            weights_y,
            weights_y,
            scaled_bottom_left_y,
            scaled_bottom_left_y,
            round,
        );
        write_smooth_directional_sum16(
            dst.offset(48),
            top7,
            top8,
            weights_y,
            weights_y,
            scaled_bottom_left_y,
            scaled_bottom_left_y,
            round,
        );
        dst = dst.offset(stride);
    }
    for y_mask in (0x01000100..0x0F0E0F0F).step_by(0x02020202) {
        let y_select: __m128i = _mm_set1_epi32(y_mask);
        let weights_y: __m128i = _mm_shuffle_epi8(weights2, y_select);
        let scaled_bottom_left_y: __m128i = _mm_shuffle_epi8(scaled_bottom_left2, y_select);
        write_smooth_directional_sum16(
            dst,
            top1,
            top2,
            weights_y,
            weights_y,
            scaled_bottom_left_y,
            scaled_bottom_left_y,
            round,
        );
        write_smooth_directional_sum16(
            dst.offset(16),
            top3,
            top4,
            weights_y,
            weights_y,
            scaled_bottom_left_y,
            scaled_bottom_left_y,
            round,
        );
        write_smooth_directional_sum16(
            dst.offset(32),
            top5,
            top6,
            weights_y,
            weights_y,
            scaled_bottom_left_y,
            scaled_bottom_left_y,
            round,
        );
        write_smooth_directional_sum16(
            dst.offset(48),
            top7,
            top8,
            weights_y,
            weights_y,
            scaled_bottom_left_y,
            scaled_bottom_left_y,
            round,
        );
        dst = dst.offset(stride);
    }
    for y_mask in (0x01000100..0x0F0E0F0F).step_by(0x02020202) {
        let y_select: __m128i = _mm_set1_epi32(y_mask);
        let weights_y: __m128i = _mm_shuffle_epi8(weights3, y_select);
        let scaled_bottom_left_y: __m128i = _mm_shuffle_epi8(scaled_bottom_left3, y_select);
        write_smooth_directional_sum16(
            dst,
            top1,
            top2,
            weights_y,
            weights_y,
            scaled_bottom_left_y,
            scaled_bottom_left_y,
            round,
        );
        write_smooth_directional_sum16(
            dst.offset(16),
            top3,
            top4,
            weights_y,
            weights_y,
            scaled_bottom_left_y,
            scaled_bottom_left_y,
            round,
        );
        write_smooth_directional_sum16(
            dst.offset(32),
            top5,
            top6,
            weights_y,
            weights_y,
            scaled_bottom_left_y,
            scaled_bottom_left_y,
            round,
        );
        write_smooth_directional_sum16(
            dst.offset(48),
            top7,
            top8,
            weights_y,
            weights_y,
            scaled_bottom_left_y,
            scaled_bottom_left_y,
            round,
        );
        dst = dst.offset(stride);
    }
    for y_mask in (0x01000100..0x0F0E0F0F).step_by(0x02020202) {
        let y_select: __m128i = _mm_set1_epi32(y_mask);
        let weights_y: __m128i = _mm_shuffle_epi8(weights4, y_select);
        let scaled_bottom_left_y: __m128i = _mm_shuffle_epi8(scaled_bottom_left4, y_select);
        write_smooth_directional_sum16(
            dst,
            top1,
            top2,
            weights_y,
            weights_y,
            scaled_bottom_left_y,
            scaled_bottom_left_y,
            round,
        );
        write_smooth_directional_sum16(
            dst.offset(16),
            top3,
            top4,
            weights_y,
            weights_y,
            scaled_bottom_left_y,
            scaled_bottom_left_y,
            round,
        );
        write_smooth_directional_sum16(
            dst.offset(32),
            top5,
            top6,
            weights_y,
            weights_y,
            scaled_bottom_left_y,
            scaled_bottom_left_y,
            round,
        );
        write_smooth_directional_sum16(
            dst.offset(48),
            top7,
            top8,
            weights_y,
            weights_y,
            scaled_bottom_left_y,
            scaled_bottom_left_y,
            round,
        );
        dst = dst.offset(stride);
    }
}

#[no_mangle]
#[target_feature(enable = "ssse3")]
pub unsafe extern "C-unwind" fn aom_smooth_v_predictor_64x64_ssse3(
    mut dst: *mut u8,
    stride: isize,
    top_row: *const u8,
    left_column: *const u8,
) {
    let zero: __m128i = _mm_setzero_si128();
    let bottom_left: __m128i = _mm_set1_epi16(*left_column.offset(63) as i16);
    let top_lolo: __m128i = LoadUnaligned16(top_row as *const core::ffi::c_void);
    let top_lohi: __m128i = LoadUnaligned16(top_row.offset(16) as *const core::ffi::c_void);
    let top1: __m128i = cvtepu8_epi16(top_lolo);
    let top2: __m128i = _mm_unpackhi_epi8(top_lolo, zero);
    let top3: __m128i = cvtepu8_epi16(top_lohi);
    let top4: __m128i = _mm_unpackhi_epi8(top_lohi, zero);
    let top_hilo: __m128i = LoadUnaligned16(top_row.offset(32) as *const core::ffi::c_void);
    let top_hihi: __m128i = LoadUnaligned16(top_row.offset(48) as *const core::ffi::c_void);
    let top5: __m128i = cvtepu8_epi16(top_hilo);
    let top6: __m128i = _mm_unpackhi_epi8(top_hilo, zero);
    let top7: __m128i = cvtepu8_epi16(top_hihi);
    let top8: __m128i = _mm_unpackhi_epi8(top_hihi, zero);
    let scale: __m128i = _mm_set1_epi16((1 << SMOOTH_WEIGHT_LOG2_SCALE) as i16);
    let round: __m128i = _mm_set1_epi16(128);
    let mut weights_base_ptr: *const u8 = smooth_weights.0.as_ptr().offset(60);
    for left_offset in (0..64).step_by(16) {
        let weights: __m128i = LoadUnaligned16(
            weights_base_ptr.offset(left_offset as isize) as *const core::ffi::c_void
        );
        let weights_lo: __m128i = cvtepu8_epi16(weights);
        let weights_hi: __m128i = _mm_unpackhi_epi8(weights, zero);
        let inverted_weights_lo: __m128i = _mm_sub_epi16(scale, weights_lo);
        let inverted_weights_hi: __m128i = _mm_sub_epi16(scale, weights_hi);
        let scaled_bottom_left_lo: __m128i = _mm_mullo_epi16(inverted_weights_lo, bottom_left);
        let scaled_bottom_left_hi: __m128i = _mm_mullo_epi16(inverted_weights_hi, bottom_left);
        for y_mask in (0x01000100..0x0F0E0F0F).step_by(0x02020202) {
            let y_select: __m128i = _mm_set1_epi32(y_mask);
            let weights_y: __m128i = _mm_shuffle_epi8(weights_lo, y_select);
            let scaled_bottom_left_y: __m128i = _mm_shuffle_epi8(scaled_bottom_left_lo, y_select);
            write_smooth_directional_sum16(
                dst,
                top1,
                top2,
                weights_y,
                weights_y,
                scaled_bottom_left_y,
                scaled_bottom_left_y,
                round,
            );
            write_smooth_directional_sum16(
                dst.offset(16),
                top3,
                top4,
                weights_y,
                weights_y,
                scaled_bottom_left_y,
                scaled_bottom_left_y,
                round,
            );
            write_smooth_directional_sum16(
                dst.offset(32),
                top5,
                top6,
                weights_y,
                weights_y,
                scaled_bottom_left_y,
                scaled_bottom_left_y,
                round,
            );
            write_smooth_directional_sum16(
                dst.offset(48),
                top7,
                top8,
                weights_y,
                weights_y,
                scaled_bottom_left_y,
                scaled_bottom_left_y,
                round,
            );
            dst = dst.offset(stride);
        }
        for y_mask in (0x01000100..0x0F0E0F0F).step_by(0x02020202) {
            let y_select: __m128i = _mm_set1_epi32(y_mask);
            let weights_y: __m128i = _mm_shuffle_epi8(weights_hi, y_select);
            let scaled_bottom_left_y: __m128i = _mm_shuffle_epi8(scaled_bottom_left_hi, y_select);
            write_smooth_directional_sum16(
                dst,
                top1,
                top2,
                weights_y,
                weights_y,
                scaled_bottom_left_y,
                scaled_bottom_left_y,
                round,
            );
            write_smooth_directional_sum16(
                dst.offset(16),
                top3,
                top4,
                weights_y,
                weights_y,
                scaled_bottom_left_y,
                scaled_bottom_left_y,
                round,
            );
            write_smooth_directional_sum16(
                dst.offset(32),
                top5,
                top6,
                weights_y,
                weights_y,
                scaled_bottom_left_y,
                scaled_bottom_left_y,
                round,
            );
            write_smooth_directional_sum16(
                dst.offset(48),
                top7,
                top8,
                weights_y,
                weights_y,
                scaled_bottom_left_y,
                scaled_bottom_left_y,
                round,
            );
            dst = dst.offset(stride);
        }
    }
}

// -----------------------------------------------------------------------------
// SMOOTH_H_PRED
#[inline]
#[target_feature(enable = "ssse3")]
unsafe fn write_smooth_horizontal_sum4(
    dst: *mut u8,
    left_y: *const __m128i,
    weights: *const __m128i,
    scaled_top_right: *const __m128i,
    round: *const __m128i,
) {
    let weighted_left_y: __m128i = _mm_mullo_epi16(*left_y, *weights);
    let pred_sum: __m128i = _mm_add_epi32(*scaled_top_right, weighted_left_y);
    // Equivalent to RightShiftWithRounding(pred[x][y], 8).
    let pred: __m128i = _mm_srli_epi32::<{ 8 }>(_mm_add_epi32(pred_sum, *round));
    let cvtepi32_epi8: __m128i = _mm_set1_epi32(0x0C080400);
    Store4(
        dst as *mut core::ffi::c_void,
        _mm_shuffle_epi8(pred, cvtepi32_epi8),
    );
}

#[no_mangle]
#[target_feature(enable = "ssse3")]
pub unsafe extern "C-unwind" fn aom_smooth_h_predictor_4x4_ssse3(
    mut dst: *mut u8,
    stride: isize,
    top_row: *const u8,
    left_column: *const u8,
) {
    let top_right: __m128i = _mm_set1_epi32(*top_row.offset(3) as i32);
    let left: __m128i = cvtepu8_epi32(Load4(left_column as *const core::ffi::c_void));
    let weights: __m128i =
        cvtepu8_epi32(Load4(smooth_weights.0.as_ptr() as *const core::ffi::c_void));
    let scale: __m128i = _mm_set1_epi16((1 << SMOOTH_WEIGHT_LOG2_SCALE) as i16);
    let inverted_weights: __m128i = _mm_sub_epi32(scale, weights);
    let scaled_top_right: __m128i = _mm_mullo_epi16(inverted_weights, top_right);
    let round: __m128i = _mm_set1_epi16((1 << (SMOOTH_WEIGHT_LOG2_SCALE - 1)) as i16);
    let mut left_y: __m128i = _mm_shuffle_epi32::<0>(left);
    write_smooth_horizontal_sum4(
        dst,
        &raw mut left_y as *const __m128i,
        &raw const weights,
        &raw const scaled_top_right,
        &raw const round,
    );
    dst = dst.offset(stride);
    left_y = _mm_shuffle_epi32::<0x55>(left);
    write_smooth_horizontal_sum4(
        dst,
        &raw mut left_y as *const __m128i,
        &raw const weights,
        &raw const scaled_top_right,
        &raw const round,
    );
    dst = dst.offset(stride);
    left_y = _mm_shuffle_epi32::<0xaa>(left);
    write_smooth_horizontal_sum4(
        dst,
        &raw mut left_y as *const __m128i,
        &raw const weights,
        &raw const scaled_top_right,
        &raw const round,
    );
    dst = dst.offset(stride);
    left_y = _mm_shuffle_epi32::<0xff>(left);
    write_smooth_horizontal_sum4(
        dst,
        &raw mut left_y as *const __m128i,
        &raw const weights,
        &raw const scaled_top_right,
        &raw const round,
    );
}

#[no_mangle]
#[target_feature(enable = "ssse3")]
pub unsafe extern "C-unwind" fn aom_smooth_h_predictor_4x8_ssse3(
    mut dst: *mut u8,
    stride: isize,
    top_row: *const u8,
    left_column: *const u8,
) {
    let top_right: __m128i = _mm_set1_epi32(*top_row.offset(3) as i32);
    let weights: __m128i =
        cvtepu8_epi32(Load4(smooth_weights.0.as_ptr() as *const core::ffi::c_void));
    let scale: __m128i = _mm_set1_epi16((1 << SMOOTH_WEIGHT_LOG2_SCALE) as i16);
    let inverted_weights: __m128i = _mm_sub_epi32(scale, weights);
    let scaled_top_right: __m128i = _mm_mullo_epi16(inverted_weights, top_right);
    let round: __m128i = _mm_set1_epi16((1 << (SMOOTH_WEIGHT_LOG2_SCALE - 1)) as i16);
    let mut left: __m128i = cvtepu8_epi32(Load4(left_column as *const core::ffi::c_void));
    let mut left_y: __m128i = _mm_shuffle_epi32::<0>(left);
    write_smooth_horizontal_sum4(
        dst,
        &raw mut left_y as *const __m128i,
        &raw const weights,
        &raw const scaled_top_right,
        &raw const round,
    );
    dst = dst.offset(stride);
    left_y = _mm_shuffle_epi32::<0x55>(left);
    write_smooth_horizontal_sum4(
        dst,
        &raw mut left_y as *const __m128i,
        &raw const weights,
        &raw const scaled_top_right,
        &raw const round,
    );
    dst = dst.offset(stride);
    left_y = _mm_shuffle_epi32::<0xaa>(left);
    write_smooth_horizontal_sum4(
        dst,
        &raw mut left_y as *const __m128i,
        &raw const weights,
        &raw const scaled_top_right,
        &raw const round,
    );
    dst = dst.offset(stride);
    left_y = _mm_shuffle_epi32::<0xff>(left);
    write_smooth_horizontal_sum4(
        dst,
        &raw mut left_y as *const __m128i,
        &raw const weights,
        &raw const scaled_top_right,
        &raw const round,
    );
    dst = dst.offset(stride);
    left = cvtepu8_epi32(Load4(left_column.offset(4) as *const core::ffi::c_void));
    left_y = _mm_shuffle_epi32::<0>(left);
    write_smooth_horizontal_sum4(
        dst,
        &raw mut left_y as *const __m128i,
        &raw const weights,
        &raw const scaled_top_right,
        &raw const round,
    );
    dst = dst.offset(stride);
    left_y = _mm_shuffle_epi32::<0x55>(left);
    write_smooth_horizontal_sum4(
        dst,
        &raw mut left_y as *const __m128i,
        &raw const weights,
        &raw const scaled_top_right,
        &raw const round,
    );
    dst = dst.offset(stride);
    left_y = _mm_shuffle_epi32::<0xaa>(left);
    write_smooth_horizontal_sum4(
        dst,
        &raw mut left_y as *const __m128i,
        &raw const weights,
        &raw const scaled_top_right,
        &raw const round,
    );
    dst = dst.offset(stride);
    left_y = _mm_shuffle_epi32::<0xff>(left);
    write_smooth_horizontal_sum4(
        dst,
        &raw mut left_y as *const __m128i,
        &raw const weights,
        &raw const scaled_top_right,
        &raw const round,
    );
}

#[no_mangle]
#[target_feature(enable = "ssse3")]
pub unsafe extern "C-unwind" fn aom_smooth_h_predictor_4x16_ssse3(
    mut dst: *mut u8,
    stride: isize,
    top_row: *const u8,
    left_column: *const u8,
) {
    let top_right: __m128i = _mm_set1_epi32(*top_row.offset(3) as i32);
    let weights: __m128i =
        cvtepu8_epi32(Load4(smooth_weights.0.as_ptr() as *const core::ffi::c_void));
    let scale: __m128i = _mm_set1_epi16((1 << SMOOTH_WEIGHT_LOG2_SCALE) as i16);
    let inverted_weights: __m128i = _mm_sub_epi32(scale, weights);
    let scaled_top_right: __m128i = _mm_mullo_epi16(inverted_weights, top_right);
    let round: __m128i = _mm_set1_epi16((1 << (SMOOTH_WEIGHT_LOG2_SCALE - 1)) as i16);
    let mut left: __m128i = cvtepu8_epi32(Load4(left_column as *const core::ffi::c_void));
    let mut left_y: __m128i = _mm_shuffle_epi32::<0>(left);
    write_smooth_horizontal_sum4(
        dst,
        &raw mut left_y as *const __m128i,
        &raw const weights,
        &raw const scaled_top_right,
        &raw const round,
    );
    dst = dst.offset(stride);
    left_y = _mm_shuffle_epi32::<0x55>(left);
    write_smooth_horizontal_sum4(
        dst,
        &raw mut left_y as *const __m128i,
        &raw const weights,
        &raw const scaled_top_right,
        &raw const round,
    );
    dst = dst.offset(stride);
    left_y = _mm_shuffle_epi32::<0xaa>(left);
    write_smooth_horizontal_sum4(
        dst,
        &raw mut left_y as *const __m128i,
        &raw const weights,
        &raw const scaled_top_right,
        &raw const round,
    );
    dst = dst.offset(stride);
    left_y = _mm_shuffle_epi32::<0xff>(left);
    write_smooth_horizontal_sum4(
        dst,
        &raw mut left_y as *const __m128i,
        &raw const weights,
        &raw const scaled_top_right,
        &raw const round,
    );
    dst = dst.offset(stride);
    left = cvtepu8_epi32(Load4(left_column.offset(4) as *const core::ffi::c_void));
    left_y = _mm_shuffle_epi32::<0>(left);
    write_smooth_horizontal_sum4(
        dst,
        &raw mut left_y as *const __m128i,
        &raw const weights,
        &raw const scaled_top_right,
        &raw const round,
    );
    dst = dst.offset(stride);
    left_y = _mm_shuffle_epi32::<0x55>(left);
    write_smooth_horizontal_sum4(
        dst,
        &raw mut left_y as *const __m128i,
        &raw const weights,
        &raw const scaled_top_right,
        &raw const round,
    );
    dst = dst.offset(stride);
    left_y = _mm_shuffle_epi32::<0xaa>(left);
    write_smooth_horizontal_sum4(
        dst,
        &raw mut left_y as *const __m128i,
        &raw const weights,
        &raw const scaled_top_right,
        &raw const round,
    );
    dst = dst.offset(stride);
    left_y = _mm_shuffle_epi32::<0xff>(left);
    write_smooth_horizontal_sum4(
        dst,
        &raw mut left_y as *const __m128i,
        &raw const weights,
        &raw const scaled_top_right,
        &raw const round,
    );
    dst = dst.offset(stride);
    left = cvtepu8_epi32(Load4(left_column.offset(8) as *const core::ffi::c_void));
    left_y = _mm_shuffle_epi32::<0>(left);
    write_smooth_horizontal_sum4(
        dst,
        &raw mut left_y as *const __m128i,
        &raw const weights,
        &raw const scaled_top_right,
        &raw const round,
    );
    dst = dst.offset(stride);
    left_y = _mm_shuffle_epi32::<0x55>(left);
    write_smooth_horizontal_sum4(
        dst,
        &raw mut left_y as *const __m128i,
        &raw const weights,
        &raw const scaled_top_right,
        &raw const round,
    );
    dst = dst.offset(stride);
    left_y = _mm_shuffle_epi32::<0xaa>(left);
    write_smooth_horizontal_sum4(
        dst,
        &raw mut left_y as *const __m128i,
        &raw const weights,
        &raw const scaled_top_right,
        &raw const round,
    );
    dst = dst.offset(stride);
    left_y = _mm_shuffle_epi32::<0xff>(left);
    write_smooth_horizontal_sum4(
        dst,
        &raw mut left_y as *const __m128i,
        &raw const weights,
        &raw const scaled_top_right,
        &raw const round,
    );
    dst = dst.offset(stride);
    left = cvtepu8_epi32(Load4(left_column.offset(12) as *const core::ffi::c_void));
    left_y = _mm_shuffle_epi32::<0>(left);
    write_smooth_horizontal_sum4(
        dst,
        &raw mut left_y as *const __m128i,
        &raw const weights,
        &raw const scaled_top_right,
        &raw const round,
    );
    dst = dst.offset(stride);
    left_y = _mm_shuffle_epi32::<0x55>(left);
    write_smooth_horizontal_sum4(
        dst,
        &raw mut left_y as *const __m128i,
        &raw const weights,
        &raw const scaled_top_right,
        &raw const round,
    );
    dst = dst.offset(stride);
    left_y = _mm_shuffle_epi32::<0xaa>(left);
    write_smooth_horizontal_sum4(
        dst,
        &raw mut left_y as *const __m128i,
        &raw const weights,
        &raw const scaled_top_right,
        &raw const round,
    );
    dst = dst.offset(stride);
    left_y = _mm_shuffle_epi32::<0xff>(left);
    write_smooth_horizontal_sum4(
        dst,
        &raw mut left_y as *const __m128i,
        &raw const weights,
        &raw const scaled_top_right,
        &raw const round,
    );
}

// !CONFIG_REALTIME_ONLY || CONFIG_AV1_DECODER
// For SMOOTH_H, |pixels| is the repeated left value for the row. For SMOOTH_V,
// |pixels| is a segment of the top row or the whole top row, and |weights| is
// repeated.
#[no_mangle]
#[target_feature(enable = "ssse3")]
pub unsafe extern "C-unwind" fn aom_smooth_h_predictor_8x4_ssse3(
    mut dst: *mut u8,
    stride: isize,
    top_row: *const u8,
    left_column: *const u8,
) {
    let top_right: __m128i = _mm_set1_epi16(*top_row.offset(7) as i16);
    let left: __m128i = cvtepu8_epi16(Load4(left_column as *const core::ffi::c_void));
    let weights: __m128i = cvtepu8_epi16(LoadLo8(
        smooth_weights.0.as_ptr().offset(4) as *const core::ffi::c_void
    ));
    let scale: __m128i = _mm_set1_epi16((1 << SMOOTH_WEIGHT_LOG2_SCALE) as i16);
    let inverted_weights: __m128i = _mm_sub_epi16(scale, weights);
    let scaled_top_right: __m128i = _mm_mullo_epi16(inverted_weights, top_right);
    let round: __m128i = _mm_set1_epi16((1 << (SMOOTH_WEIGHT_LOG2_SCALE - 1)) as i16);
    let mut y_select: __m128i = _mm_set1_epi32(0x01000100);
    let mut left_y: __m128i = _mm_shuffle_epi8(left, y_select);
    write_smooth_directional_sum8(
        dst,
        &raw mut left_y as *const __m128i,
        &raw const weights,
        &raw const scaled_top_right,
        &raw const round,
    );
    dst = dst.offset(stride);
    y_select = _mm_set1_epi32(0x03020302);
    left_y = _mm_shuffle_epi8(left, y_select);
    write_smooth_directional_sum8(
        dst,
        &raw mut left_y as *const __m128i,
        &raw const weights,
        &raw const scaled_top_right,
        &raw const round,
    );
    dst = dst.offset(stride);
    y_select = _mm_set1_epi32(0x05040504);
    left_y = _mm_shuffle_epi8(left, y_select);
    write_smooth_directional_sum8(
        dst,
        &raw mut left_y as *const __m128i,
        &raw const weights,
        &raw const scaled_top_right,
        &raw const round,
    );
    dst = dst.offset(stride);
    y_select = _mm_set1_epi32(0x07060706);
    left_y = _mm_shuffle_epi8(left, y_select);
    write_smooth_directional_sum8(
        dst,
        &raw mut left_y as *const __m128i,
        &raw const weights,
        &raw const scaled_top_right,
        &raw const round,
    );
}

#[no_mangle]
#[target_feature(enable = "ssse3")]
pub unsafe extern "C-unwind" fn aom_smooth_h_predictor_8x8_ssse3(
    mut dst: *mut u8,
    stride: isize,
    top_row: *const u8,
    left_column: *const u8,
) {
    let top_right: __m128i = _mm_set1_epi16(*top_row.offset(7) as i16);
    let left: __m128i = cvtepu8_epi16(LoadLo8(left_column as *const core::ffi::c_void));
    let weights: __m128i = cvtepu8_epi16(LoadLo8(
        smooth_weights.0.as_ptr().offset(4) as *const core::ffi::c_void
    ));
    let scale: __m128i = _mm_set1_epi16((1 << SMOOTH_WEIGHT_LOG2_SCALE) as i16);
    let inverted_weights: __m128i = _mm_sub_epi16(scale, weights);
    let scaled_top_right: __m128i = _mm_mullo_epi16(inverted_weights, top_right);
    let round: __m128i = _mm_set1_epi16((1 << (SMOOTH_WEIGHT_LOG2_SCALE - 1)) as i16);
    for y_mask in (0x01000100..0x0F0E0F0F).step_by(0x02020202) {
        let y_select: __m128i = _mm_set1_epi32(y_mask);
        let left_y: __m128i = _mm_shuffle_epi8(left, y_select);
        write_smooth_directional_sum8(
            dst,
            &raw const left_y,
            &raw const weights,
            &raw const scaled_top_right,
            &raw const round,
        );
        dst = dst.offset(stride);
    }
}

#[no_mangle]
#[target_feature(enable = "ssse3")]
pub unsafe extern "C-unwind" fn aom_smooth_h_predictor_8x16_ssse3(
    mut dst: *mut u8,
    stride: isize,
    top_row: *const u8,
    left_column: *const u8,
) {
    let top_right: __m128i = _mm_set1_epi16(*top_row.offset(7) as i16);
    let weights: __m128i = cvtepu8_epi16(LoadLo8(
        smooth_weights.0.as_ptr().offset(4) as *const core::ffi::c_void
    ));
    let scale: __m128i = _mm_set1_epi16((1 << SMOOTH_WEIGHT_LOG2_SCALE) as i16);
    let inverted_weights: __m128i = _mm_sub_epi16(scale, weights);
    let scaled_top_right: __m128i = _mm_mullo_epi16(inverted_weights, top_right);
    let round: __m128i = _mm_set1_epi16((1 << (SMOOTH_WEIGHT_LOG2_SCALE - 1)) as i16);
    let mut left: __m128i = cvtepu8_epi16(LoadLo8(left_column as *const core::ffi::c_void));
    for y_mask in (0x01000100..0x0F0E0F0F).step_by(0x02020202) {
        let y_select: __m128i = _mm_set1_epi32(y_mask);
        let left_y: __m128i = _mm_shuffle_epi8(left, y_select);
        write_smooth_directional_sum8(
            dst,
            &raw const left_y,
            &raw const weights,
            &raw const scaled_top_right,
            &raw const round,
        );
        dst = dst.offset(stride);
    }
    left = cvtepu8_epi16(LoadLo8(left_column.offset(8) as *const core::ffi::c_void));
    for y_mask in (0x01000100..0x0F0E0F0F).step_by(0x02020202) {
        let y_select: __m128i = _mm_set1_epi32(y_mask);
        let left_y: __m128i = _mm_shuffle_epi8(left, y_select);
        write_smooth_directional_sum8(
            dst,
            &raw const left_y,
            &raw const weights,
            &raw const scaled_top_right,
            &raw const round,
        );
        dst = dst.offset(stride);
    }
}

#[no_mangle]
#[target_feature(enable = "ssse3")]
pub unsafe extern "C-unwind" fn aom_smooth_h_predictor_8x32_ssse3(
    mut dst: *mut u8,
    stride: isize,
    top_row: *const u8,
    left_column: *const u8,
) {
    let top_right: __m128i = _mm_set1_epi16(*top_row.offset(7) as i16);
    let weights: __m128i = cvtepu8_epi16(LoadLo8(
        smooth_weights.0.as_ptr().offset(4) as *const core::ffi::c_void
    ));
    let scale: __m128i = _mm_set1_epi16((1 << SMOOTH_WEIGHT_LOG2_SCALE) as i16);
    let inverted_weights: __m128i = _mm_sub_epi16(scale, weights);
    let scaled_top_right: __m128i = _mm_mullo_epi16(inverted_weights, top_right);
    let round: __m128i = _mm_set1_epi16((1 << (SMOOTH_WEIGHT_LOG2_SCALE - 1)) as i16);
    let mut left: __m128i = cvtepu8_epi16(LoadLo8(left_column as *const core::ffi::c_void));
    for y_mask in (0x01000100..0x0F0E0F0F).step_by(0x02020202) {
        let y_select: __m128i = _mm_set1_epi32(y_mask);
        let left_y: __m128i = _mm_shuffle_epi8(left, y_select);
        write_smooth_directional_sum8(
            dst,
            &raw const left_y,
            &raw const weights,
            &raw const scaled_top_right,
            &raw const round,
        );
        dst = dst.offset(stride);
    }
    left = cvtepu8_epi16(LoadLo8(left_column.offset(8) as *const core::ffi::c_void));
    for y_mask in (0x01000100..0x0F0E0F0F).step_by(0x02020202) {
        let y_select: __m128i = _mm_set1_epi32(y_mask);
        let left_y: __m128i = _mm_shuffle_epi8(left, y_select);
        write_smooth_directional_sum8(
            dst,
            &raw const left_y,
            &raw const weights,
            &raw const scaled_top_right,
            &raw const round,
        );
        dst = dst.offset(stride);
    }
    left = cvtepu8_epi16(LoadLo8(left_column.offset(16) as *const core::ffi::c_void));
    for y_mask in (0x01000100..0x0F0E0F0F).step_by(0x02020202) {
        let y_select: __m128i = _mm_set1_epi32(y_mask);
        let left_y: __m128i = _mm_shuffle_epi8(left, y_select);
        write_smooth_directional_sum8(
            dst,
            &raw const left_y,
            &raw const weights,
            &raw const scaled_top_right,
            &raw const round,
        );
        dst = dst.offset(stride);
    }
    left = cvtepu8_epi16(LoadLo8(left_column.offset(24) as *const core::ffi::c_void));
    for y_mask in (0x01000100..0x0F0E0F0F).step_by(0x02020202) {
        let y_select: __m128i = _mm_set1_epi32(y_mask);
        let left_y: __m128i = _mm_shuffle_epi8(left, y_select);
        write_smooth_directional_sum8(
            dst,
            &raw const left_y,
            &raw const weights,
            &raw const scaled_top_right,
            &raw const round,
        );
        dst = dst.offset(stride);
    }
}

#[no_mangle]
#[target_feature(enable = "ssse3")]
pub unsafe extern "C-unwind" fn aom_smooth_h_predictor_16x4_ssse3(
    mut dst: *mut u8,
    stride: isize,
    top_row: *const u8,
    left_column: *const u8,
) {
    let top_right: __m128i = _mm_set1_epi16(*top_row.offset(15) as i16);
    let left: __m128i = cvtepu8_epi16(Load4(left_column as *const core::ffi::c_void));
    let weights: __m128i =
        LoadUnaligned16(smooth_weights.0.as_ptr().offset(12) as *const core::ffi::c_void);
    let scale: __m128i = _mm_set1_epi16((1 << SMOOTH_WEIGHT_LOG2_SCALE) as i16);
    let weights1: __m128i = cvtepu8_epi16(weights);
    let weights2: __m128i = cvtepu8_epi16(_mm_srli_si128::<8>(weights));
    let inverted_weights1: __m128i = _mm_sub_epi16(scale, weights1);
    let inverted_weights2: __m128i = _mm_sub_epi16(scale, weights2);
    let scaled_top_right1: __m128i = _mm_mullo_epi16(inverted_weights1, top_right);
    let scaled_top_right2: __m128i = _mm_mullo_epi16(inverted_weights2, top_right);
    let round: __m128i = _mm_set1_epi16((1 << (SMOOTH_WEIGHT_LOG2_SCALE - 1)) as i16);
    let mut y_mask: __m128i = _mm_set1_epi32(0x01000100);
    let mut left_y: __m128i = _mm_shuffle_epi8(left, y_mask);
    write_smooth_directional_sum16(
        dst,
        left_y,
        left_y,
        weights1,
        weights2,
        scaled_top_right1,
        scaled_top_right2,
        round,
    );
    dst = dst.offset(stride);
    y_mask = _mm_set1_epi32(0x03020302);
    left_y = _mm_shuffle_epi8(left, y_mask);
    write_smooth_directional_sum16(
        dst,
        left_y,
        left_y,
        weights1,
        weights2,
        scaled_top_right1,
        scaled_top_right2,
        round,
    );
    dst = dst.offset(stride);
    y_mask = _mm_set1_epi32(0x05040504);
    left_y = _mm_shuffle_epi8(left, y_mask);
    write_smooth_directional_sum16(
        dst,
        left_y,
        left_y,
        weights1,
        weights2,
        scaled_top_right1,
        scaled_top_right2,
        round,
    );
    dst = dst.offset(stride);
    y_mask = _mm_set1_epi32(0x07060706);
    left_y = _mm_shuffle_epi8(left, y_mask);
    write_smooth_directional_sum16(
        dst,
        left_y,
        left_y,
        weights1,
        weights2,
        scaled_top_right1,
        scaled_top_right2,
        round,
    );
}

// !CONFIG_REALTIME_ONLY || CONFIG_AV1_DECODER
#[no_mangle]
#[target_feature(enable = "ssse3")]
pub unsafe extern "C-unwind" fn aom_smooth_h_predictor_16x8_ssse3(
    mut dst: *mut u8,
    stride: isize,
    top_row: *const u8,
    left_column: *const u8,
) {
    let top_right: __m128i = _mm_set1_epi16(*top_row.offset(15) as i16);
    let left: __m128i = cvtepu8_epi16(LoadLo8(left_column as *const core::ffi::c_void));
    let weights: __m128i =
        LoadUnaligned16(smooth_weights.0.as_ptr().offset(12) as *const core::ffi::c_void);
    let scale: __m128i = _mm_set1_epi16((1 << SMOOTH_WEIGHT_LOG2_SCALE) as i16);
    let weights1: __m128i = cvtepu8_epi16(weights);
    let weights2: __m128i = cvtepu8_epi16(_mm_srli_si128::<8>(weights));
    let inverted_weights1: __m128i = _mm_sub_epi16(scale, weights1);
    let inverted_weights2: __m128i = _mm_sub_epi16(scale, weights2);
    let scaled_top_right1: __m128i = _mm_mullo_epi16(inverted_weights1, top_right);
    let scaled_top_right2: __m128i = _mm_mullo_epi16(inverted_weights2, top_right);
    let round: __m128i = _mm_set1_epi16((1 << (SMOOTH_WEIGHT_LOG2_SCALE - 1)) as i16);
    for y_mask in (0x01000100..0x0F0E0F0F).step_by(0x02020202) {
        let y_select: __m128i = _mm_set1_epi32(y_mask);
        let left_y: __m128i = _mm_shuffle_epi8(left, y_select);
        write_smooth_directional_sum16(
            dst,
            left_y,
            left_y,
            weights1,
            weights2,
            scaled_top_right1,
            scaled_top_right2,
            round,
        );
        dst = dst.offset(stride);
    }
}

#[no_mangle]
#[target_feature(enable = "ssse3")]
pub unsafe extern "C-unwind" fn aom_smooth_h_predictor_16x16_ssse3(
    mut dst: *mut u8,
    stride: isize,
    top_row: *const u8,
    left_column: *const u8,
) {
    let top_right: __m128i = _mm_set1_epi16(*top_row.offset(15) as i16);
    let weights: __m128i =
        LoadUnaligned16(smooth_weights.0.as_ptr().offset(12) as *const core::ffi::c_void);
    let scale: __m128i = _mm_set1_epi16((1 << SMOOTH_WEIGHT_LOG2_SCALE) as i16);
    let weights1: __m128i = cvtepu8_epi16(weights);
    let weights2: __m128i = cvtepu8_epi16(_mm_srli_si128::<8>(weights));
    let inverted_weights1: __m128i = _mm_sub_epi16(scale, weights1);
    let inverted_weights2: __m128i = _mm_sub_epi16(scale, weights2);
    let scaled_top_right1: __m128i = _mm_mullo_epi16(inverted_weights1, top_right);
    let scaled_top_right2: __m128i = _mm_mullo_epi16(inverted_weights2, top_right);
    let round: __m128i = _mm_set1_epi16((1 << (SMOOTH_WEIGHT_LOG2_SCALE - 1)) as i16);
    let mut left: __m128i = cvtepu8_epi16(LoadLo8(left_column as *const core::ffi::c_void));
    for y_mask in (0x01000100..0x0F0E0F0F).step_by(0x02020202) {
        let y_select: __m128i = _mm_set1_epi32(y_mask);
        let left_y: __m128i = _mm_shuffle_epi8(left, y_select);
        write_smooth_directional_sum16(
            dst,
            left_y,
            left_y,
            weights1,
            weights2,
            scaled_top_right1,
            scaled_top_right2,
            round,
        );
        dst = dst.offset(stride);
    }
    left = cvtepu8_epi16(LoadLo8(left_column.offset(8) as *const core::ffi::c_void));
    for y_mask in (0x01000100..0x0F0E0F0F).step_by(0x02020202) {
        let y_select: __m128i = _mm_set1_epi32(y_mask);
        let left_y: __m128i = _mm_shuffle_epi8(left, y_select);
        write_smooth_directional_sum16(
            dst,
            left_y,
            left_y,
            weights1,
            weights2,
            scaled_top_right1,
            scaled_top_right2,
            round,
        );
        dst = dst.offset(stride);
    }
}

#[no_mangle]
#[target_feature(enable = "ssse3")]
pub unsafe extern "C-unwind" fn aom_smooth_h_predictor_16x32_ssse3(
    mut dst: *mut u8,
    stride: isize,
    top_row: *const u8,
    left_column: *const u8,
) {
    let top_right: __m128i = _mm_set1_epi16(*top_row.offset(15) as i16);
    let weights: __m128i =
        LoadUnaligned16(smooth_weights.0.as_ptr().offset(12) as *const core::ffi::c_void);
    let scale: __m128i = _mm_set1_epi16((1 << SMOOTH_WEIGHT_LOG2_SCALE) as i16);
    let weights1: __m128i = cvtepu8_epi16(weights);
    let weights2: __m128i = cvtepu8_epi16(_mm_srli_si128::<8>(weights));
    let inverted_weights1: __m128i = _mm_sub_epi16(scale, weights1);
    let inverted_weights2: __m128i = _mm_sub_epi16(scale, weights2);
    let scaled_top_right1: __m128i = _mm_mullo_epi16(inverted_weights1, top_right);
    let scaled_top_right2: __m128i = _mm_mullo_epi16(inverted_weights2, top_right);
    let round: __m128i = _mm_set1_epi16((1 << (SMOOTH_WEIGHT_LOG2_SCALE - 1)) as i16);
    let mut left: __m128i = cvtepu8_epi16(LoadLo8(left_column as *const core::ffi::c_void));
    for y_mask in (0x01000100..0x0F0E0F0F).step_by(0x02020202) {
        let y_select: __m128i = _mm_set1_epi32(y_mask);
        let left_y: __m128i = _mm_shuffle_epi8(left, y_select);
        write_smooth_directional_sum16(
            dst,
            left_y,
            left_y,
            weights1,
            weights2,
            scaled_top_right1,
            scaled_top_right2,
            round,
        );
        dst = dst.offset(stride);
    }
    left = cvtepu8_epi16(LoadLo8(left_column.offset(8) as *const core::ffi::c_void));
    for y_mask in (0x01000100..0x0F0E0F0F).step_by(0x02020202) {
        let y_select: __m128i = _mm_set1_epi32(y_mask);
        let left_y: __m128i = _mm_shuffle_epi8(left, y_select);
        write_smooth_directional_sum16(
            dst,
            left_y,
            left_y,
            weights1,
            weights2,
            scaled_top_right1,
            scaled_top_right2,
            round,
        );
        dst = dst.offset(stride);
    }
    left = cvtepu8_epi16(LoadLo8(left_column.offset(16) as *const core::ffi::c_void));
    for y_mask in (0x01000100..0x0F0E0F0F).step_by(0x02020202) {
        let y_select: __m128i = _mm_set1_epi32(y_mask);
        let left_y: __m128i = _mm_shuffle_epi8(left, y_select);
        write_smooth_directional_sum16(
            dst,
            left_y,
            left_y,
            weights1,
            weights2,
            scaled_top_right1,
            scaled_top_right2,
            round,
        );
        dst = dst.offset(stride);
    }
    left = cvtepu8_epi16(LoadLo8(left_column.offset(24) as *const core::ffi::c_void));
    for y_mask in (0x01000100..0x0F0E0F0F).step_by(0x02020202) {
        let y_select: __m128i = _mm_set1_epi32(y_mask);
        let left_y: __m128i = _mm_shuffle_epi8(left, y_select);
        write_smooth_directional_sum16(
            dst,
            left_y,
            left_y,
            weights1,
            weights2,
            scaled_top_right1,
            scaled_top_right2,
            round,
        );
        dst = dst.offset(stride);
    }
}

#[no_mangle]
#[target_feature(enable = "ssse3")]
pub unsafe extern "C-unwind" fn aom_smooth_h_predictor_16x64_ssse3(
    mut dst: *mut u8,
    stride: isize,
    top_row: *const u8,
    left_column: *const u8,
) {
    let top_right: __m128i = _mm_set1_epi16(*top_row.offset(15) as i16);
    let weights: __m128i =
        LoadUnaligned16(smooth_weights.0.as_ptr().offset(12) as *const core::ffi::c_void);
    let scale: __m128i = _mm_set1_epi16((1 << SMOOTH_WEIGHT_LOG2_SCALE) as i16);
    let weights1: __m128i = cvtepu8_epi16(weights);
    let weights2: __m128i = cvtepu8_epi16(_mm_srli_si128::<8>(weights));
    let inverted_weights1: __m128i = _mm_sub_epi16(scale, weights1);
    let inverted_weights2: __m128i = _mm_sub_epi16(scale, weights2);
    let scaled_top_right1: __m128i = _mm_mullo_epi16(inverted_weights1, top_right);
    let scaled_top_right2: __m128i = _mm_mullo_epi16(inverted_weights2, top_right);
    let round: __m128i = _mm_set1_epi16((1 << (SMOOTH_WEIGHT_LOG2_SCALE - 1)) as i16);
    for left_offset in (0..64).step_by(8) {
        let left: __m128i = cvtepu8_epi16(LoadLo8(
            left_column.offset(left_offset as isize) as *const core::ffi::c_void
        ));
        for y_mask in (0x01000100..0x0F0E0F0F).step_by(0x02020202) {
            let y_select: __m128i = _mm_set1_epi32(y_mask);
            let left_y: __m128i = _mm_shuffle_epi8(left, y_select);
            write_smooth_directional_sum16(
                dst,
                left_y,
                left_y,
                weights1,
                weights2,
                scaled_top_right1,
                scaled_top_right2,
                round,
            );
            dst = dst.offset(stride);
        }
    }
}

#[no_mangle]
#[target_feature(enable = "ssse3")]
pub unsafe extern "C-unwind" fn aom_smooth_h_predictor_32x8_ssse3(
    mut dst: *mut u8,
    stride: isize,
    top_row: *const u8,
    left_column: *const u8,
) {
    let top_right: __m128i = _mm_set1_epi16(*top_row.offset(31) as i16);
    let left: __m128i = cvtepu8_epi16(LoadLo8(left_column as *const core::ffi::c_void));
    let weights_lo: __m128i =
        LoadUnaligned16(smooth_weights.0.as_ptr().offset(28) as *const core::ffi::c_void);
    let weights_hi: __m128i =
        LoadUnaligned16(smooth_weights.0.as_ptr().offset(44) as *const core::ffi::c_void);
    let scale: __m128i = _mm_set1_epi16((1 << SMOOTH_WEIGHT_LOG2_SCALE) as i16);
    let weights1: __m128i = cvtepu8_epi16(weights_lo);
    let weights2: __m128i = cvtepu8_epi16(_mm_srli_si128::<8>(weights_lo));
    let weights3: __m128i = cvtepu8_epi16(weights_hi);
    let weights4: __m128i = cvtepu8_epi16(_mm_srli_si128::<8>(weights_hi));
    let inverted_weights1: __m128i = _mm_sub_epi16(scale, weights1);
    let inverted_weights2: __m128i = _mm_sub_epi16(scale, weights2);
    let inverted_weights3: __m128i = _mm_sub_epi16(scale, weights3);
    let inverted_weights4: __m128i = _mm_sub_epi16(scale, weights4);
    let scaled_top_right1: __m128i = _mm_mullo_epi16(inverted_weights1, top_right);
    let scaled_top_right2: __m128i = _mm_mullo_epi16(inverted_weights2, top_right);
    let scaled_top_right3: __m128i = _mm_mullo_epi16(inverted_weights3, top_right);
    let scaled_top_right4: __m128i = _mm_mullo_epi16(inverted_weights4, top_right);
    let round: __m128i = _mm_set1_epi16((1 << (SMOOTH_WEIGHT_LOG2_SCALE - 1)) as i16);
    for y_mask in (0x01000100..0x0F0E0F0F).step_by(0x02020202) {
        let mut y_select: __m128i = _mm_set1_epi32(y_mask);
        let mut left_y: __m128i = _mm_shuffle_epi8(left, y_select);
        write_smooth_directional_sum16(
            dst,
            left_y,
            left_y,
            weights1,
            weights2,
            scaled_top_right1,
            scaled_top_right2,
            round,
        );
        write_smooth_directional_sum16(
            dst.offset(16),
            left_y,
            left_y,
            weights3,
            weights4,
            scaled_top_right3,
            scaled_top_right4,
            round,
        );
        dst = dst.offset(stride);
    }
}

// !CONFIG_REALTIME_ONLY || CONFIG_AV1_DECODER
#[no_mangle]
#[target_feature(enable = "ssse3")]
pub unsafe extern "C-unwind" fn aom_smooth_h_predictor_32x16_ssse3(
    mut dst: *mut u8,
    stride: isize,
    top_row: *const u8,
    left_column: *const u8,
) {
    let top_right: __m128i = _mm_set1_epi16(*top_row.offset(31) as i16);
    let left1: __m128i = cvtepu8_epi16(LoadLo8(left_column as *const core::ffi::c_void));
    let weights_lo: __m128i =
        LoadUnaligned16(smooth_weights.0.as_ptr().offset(28) as *const core::ffi::c_void);
    let weights_hi: __m128i =
        LoadUnaligned16(smooth_weights.0.as_ptr().offset(44) as *const core::ffi::c_void);
    let scale: __m128i = _mm_set1_epi16((1 << SMOOTH_WEIGHT_LOG2_SCALE) as i16);
    let weights1: __m128i = cvtepu8_epi16(weights_lo);
    let weights2: __m128i = cvtepu8_epi16(_mm_srli_si128::<8>(weights_lo));
    let weights3: __m128i = cvtepu8_epi16(weights_hi);
    let weights4: __m128i = cvtepu8_epi16(_mm_srli_si128::<8>(weights_hi));
    let inverted_weights1: __m128i = _mm_sub_epi16(scale, weights1);
    let inverted_weights2: __m128i = _mm_sub_epi16(scale, weights2);
    let inverted_weights3: __m128i = _mm_sub_epi16(scale, weights3);
    let inverted_weights4: __m128i = _mm_sub_epi16(scale, weights4);
    let scaled_top_right1: __m128i = _mm_mullo_epi16(inverted_weights1, top_right);
    let scaled_top_right2: __m128i = _mm_mullo_epi16(inverted_weights2, top_right);
    let scaled_top_right3: __m128i = _mm_mullo_epi16(inverted_weights3, top_right);
    let scaled_top_right4: __m128i = _mm_mullo_epi16(inverted_weights4, top_right);
    let round: __m128i = _mm_set1_epi16((1 << (SMOOTH_WEIGHT_LOG2_SCALE - 1)) as i16);
    for y_mask in (0x01000100..0x0F0E0F0F).step_by(0x02020202) {
        let mut y_select: __m128i = _mm_set1_epi32(y_mask);
        let mut left_y: __m128i = _mm_shuffle_epi8(left1, y_select);
        write_smooth_directional_sum16(
            dst,
            left_y,
            left_y,
            weights1,
            weights2,
            scaled_top_right1,
            scaled_top_right2,
            round,
        );
        write_smooth_directional_sum16(
            dst.offset(16),
            left_y,
            left_y,
            weights3,
            weights4,
            scaled_top_right3,
            scaled_top_right4,
            round,
        );
        dst = dst.offset(stride);
    }
    let left2: __m128i = cvtepu8_epi16(LoadLo8(
        (left_column as *const u8).offset(8) as *const core::ffi::c_void
    ));
    for y_mask in (0x01000100..0x0F0E0F0F).step_by(0x02020202) {
        let mut y_select: __m128i = _mm_set1_epi32(y_mask);
        let mut left_y: __m128i = _mm_shuffle_epi8(left2, y_select);
        write_smooth_directional_sum16(
            dst,
            left_y,
            left_y,
            weights1,
            weights2,
            scaled_top_right1,
            scaled_top_right2,
            round,
        );
        write_smooth_directional_sum16(
            dst.offset(16),
            left_y,
            left_y,
            weights3,
            weights4,
            scaled_top_right3,
            scaled_top_right4,
            round,
        );
        dst = dst.offset(stride);
    }
}

#[no_mangle]
#[target_feature(enable = "ssse3")]
pub unsafe extern "C-unwind" fn aom_smooth_h_predictor_32x32_ssse3(
    mut dst: *mut u8,
    stride: isize,
    top_row: *const u8,
    left_column: *const u8,
) {
    let top_right: __m128i = _mm_set1_epi16(*top_row.offset(31) as i16);
    let weights_lo: __m128i =
        LoadUnaligned16(smooth_weights.0.as_ptr().offset(28) as *const core::ffi::c_void);
    let weights_hi: __m128i =
        LoadUnaligned16(smooth_weights.0.as_ptr().offset(44) as *const core::ffi::c_void);
    let scale: __m128i = _mm_set1_epi16((1 << SMOOTH_WEIGHT_LOG2_SCALE) as i16);
    let weights1: __m128i = cvtepu8_epi16(weights_lo);
    let weights2: __m128i = cvtepu8_epi16(_mm_srli_si128::<8>(weights_lo));
    let weights3: __m128i = cvtepu8_epi16(weights_hi);
    let weights4: __m128i = cvtepu8_epi16(_mm_srli_si128::<8>(weights_hi));
    let inverted_weights1: __m128i = _mm_sub_epi16(scale, weights1);
    let inverted_weights2: __m128i = _mm_sub_epi16(scale, weights2);
    let inverted_weights3: __m128i = _mm_sub_epi16(scale, weights3);
    let inverted_weights4: __m128i = _mm_sub_epi16(scale, weights4);
    let scaled_top_right1: __m128i = _mm_mullo_epi16(inverted_weights1, top_right);
    let scaled_top_right2: __m128i = _mm_mullo_epi16(inverted_weights2, top_right);
    let scaled_top_right3: __m128i = _mm_mullo_epi16(inverted_weights3, top_right);
    let scaled_top_right4: __m128i = _mm_mullo_epi16(inverted_weights4, top_right);
    let round: __m128i = _mm_set1_epi16((1 << (SMOOTH_WEIGHT_LOG2_SCALE - 1)) as i16);
    let mut left: __m128i = cvtepu8_epi16(LoadLo8(left_column as *const core::ffi::c_void));
    for y_mask in (0x01000100..0x0F0E0F0F).step_by(0x02020202) {
        let mut y_select: __m128i = _mm_set1_epi32(y_mask);
        let mut left_y: __m128i = _mm_shuffle_epi8(left, y_select);
        write_smooth_directional_sum16(
            dst,
            left_y,
            left_y,
            weights1,
            weights2,
            scaled_top_right1,
            scaled_top_right2,
            round,
        );
        write_smooth_directional_sum16(
            dst.offset(16),
            left_y,
            left_y,
            weights3,
            weights4,
            scaled_top_right3,
            scaled_top_right4,
            round,
        );
        dst = dst.offset(stride);
    }
    left = cvtepu8_epi16(LoadLo8(left_column.offset(8) as *const core::ffi::c_void));
    for y_mask in (0x01000100..0x0F0E0F0F).step_by(0x02020202) {
        let mut y_select: __m128i = _mm_set1_epi32(y_mask);
        let mut left_y: __m128i = _mm_shuffle_epi8(left, y_select);
        write_smooth_directional_sum16(
            dst,
            left_y,
            left_y,
            weights1,
            weights2,
            scaled_top_right1,
            scaled_top_right2,
            round,
        );
        write_smooth_directional_sum16(
            dst.offset(16),
            left_y,
            left_y,
            weights3,
            weights4,
            scaled_top_right3,
            scaled_top_right4,
            round,
        );
        dst = dst.offset(stride);
    }
    left = cvtepu8_epi16(LoadLo8(left_column.offset(16) as *const core::ffi::c_void));
    for y_mask in (0x01000100..0x0F0E0F0F).step_by(0x02020202) {
        let mut y_select: __m128i = _mm_set1_epi32(y_mask);
        let mut left_y: __m128i = _mm_shuffle_epi8(left, y_select);
        write_smooth_directional_sum16(
            dst,
            left_y,
            left_y,
            weights1,
            weights2,
            scaled_top_right1,
            scaled_top_right2,
            round,
        );
        write_smooth_directional_sum16(
            dst.offset(16),
            left_y,
            left_y,
            weights3,
            weights4,
            scaled_top_right3,
            scaled_top_right4,
            round,
        );
        dst = dst.offset(stride);
    }
    left = cvtepu8_epi16(LoadLo8(left_column.offset(24) as *const core::ffi::c_void));
    for y_mask in (0x01000100..0x0F0E0F0F).step_by(0x02020202) {
        let mut y_select: __m128i = _mm_set1_epi32(y_mask);
        let mut left_y: __m128i = _mm_shuffle_epi8(left, y_select);
        write_smooth_directional_sum16(
            dst,
            left_y,
            left_y,
            weights1,
            weights2,
            scaled_top_right1,
            scaled_top_right2,
            round,
        );
        write_smooth_directional_sum16(
            dst.offset(16),
            left_y,
            left_y,
            weights3,
            weights4,
            scaled_top_right3,
            scaled_top_right4,
            round,
        );
        dst = dst.offset(stride);
    }
}

#[no_mangle]
#[target_feature(enable = "ssse3")]
pub unsafe extern "C-unwind" fn aom_smooth_h_predictor_32x64_ssse3(
    mut dst: *mut u8,
    stride: isize,
    top_row: *const u8,
    left_column: *const u8,
) {
    let top_right: __m128i = _mm_set1_epi16(*top_row.offset(31) as i16);
    let weights_lo: __m128i =
        LoadUnaligned16(smooth_weights.0.as_ptr().offset(28) as *const core::ffi::c_void);
    let weights_hi: __m128i =
        LoadUnaligned16(smooth_weights.0.as_ptr().offset(44) as *const core::ffi::c_void);
    let scale: __m128i = _mm_set1_epi16((1 << SMOOTH_WEIGHT_LOG2_SCALE) as i16);
    let weights1: __m128i = cvtepu8_epi16(weights_lo);
    let weights2: __m128i = cvtepu8_epi16(_mm_srli_si128::<8>(weights_lo));
    let weights3: __m128i = cvtepu8_epi16(weights_hi);
    let weights4: __m128i = cvtepu8_epi16(_mm_srli_si128::<8>(weights_hi));
    let inverted_weights1: __m128i = _mm_sub_epi16(scale, weights1);
    let inverted_weights2: __m128i = _mm_sub_epi16(scale, weights2);
    let inverted_weights3: __m128i = _mm_sub_epi16(scale, weights3);
    let inverted_weights4: __m128i = _mm_sub_epi16(scale, weights4);
    let scaled_top_right1: __m128i = _mm_mullo_epi16(inverted_weights1, top_right);
    let scaled_top_right2: __m128i = _mm_mullo_epi16(inverted_weights2, top_right);
    let scaled_top_right3: __m128i = _mm_mullo_epi16(inverted_weights3, top_right);
    let scaled_top_right4: __m128i = _mm_mullo_epi16(inverted_weights4, top_right);
    let round: __m128i = _mm_set1_epi16((1 << (SMOOTH_WEIGHT_LOG2_SCALE - 1)) as i16);
    for left_offset in (0..64).step_by(8) {
        let left: __m128i = cvtepu8_epi16(LoadLo8(
            left_column.offset(left_offset as isize) as *const core::ffi::c_void
        ));
        for y_mask in (0x01000100..0x0F0E0F0F).step_by(0x02020202) {
            let y_select: __m128i = _mm_set1_epi32(y_mask);
            let left_y: __m128i = _mm_shuffle_epi8(left, y_select);
            write_smooth_directional_sum16(
                dst,
                left_y,
                left_y,
                weights1,
                weights2,
                scaled_top_right1,
                scaled_top_right2,
                round,
            );
            write_smooth_directional_sum16(
                dst.offset(16),
                left_y,
                left_y,
                weights3,
                weights4,
                scaled_top_right3,
                scaled_top_right4,
                round,
            );
            dst = dst.offset(stride);
        }
    }
}

#[no_mangle]
#[target_feature(enable = "ssse3")]
pub unsafe extern "C-unwind" fn aom_smooth_h_predictor_64x16_ssse3(
    mut dst: *mut u8,
    stride: isize,
    top_row: *const u8,
    left_column: *const u8,
) {
    let top_right: __m128i = _mm_set1_epi16(*top_row.offset(63) as i16);
    let left1: __m128i = cvtepu8_epi16(LoadLo8(left_column as *const core::ffi::c_void));
    let weights_lolo: __m128i =
        LoadUnaligned16(smooth_weights.0.as_ptr().offset(60) as *const core::ffi::c_void);
    let weights_lohi: __m128i =
        LoadUnaligned16(smooth_weights.0.as_ptr().offset(76) as *const core::ffi::c_void);
    let scale: __m128i = _mm_set1_epi16((1 << SMOOTH_WEIGHT_LOG2_SCALE) as i16);
    let weights1: __m128i = cvtepu8_epi16(weights_lolo);
    let weights2: __m128i = cvtepu8_epi16(_mm_srli_si128::<8>(weights_lolo));
    let weights3: __m128i = cvtepu8_epi16(weights_lohi);
    let weights4: __m128i = cvtepu8_epi16(_mm_srli_si128::<8>(weights_lohi));
    let inverted_weights1: __m128i = _mm_sub_epi16(scale, weights1);
    let inverted_weights2: __m128i = _mm_sub_epi16(scale, weights2);
    let inverted_weights3: __m128i = _mm_sub_epi16(scale, weights3);
    let inverted_weights4: __m128i = _mm_sub_epi16(scale, weights4);
    let scaled_top_right1: __m128i = _mm_mullo_epi16(inverted_weights1, top_right);
    let scaled_top_right2: __m128i = _mm_mullo_epi16(inverted_weights2, top_right);
    let scaled_top_right3: __m128i = _mm_mullo_epi16(inverted_weights3, top_right);
    let scaled_top_right4: __m128i = _mm_mullo_epi16(inverted_weights4, top_right);
    let weights_hilo: __m128i =
        LoadUnaligned16(smooth_weights.0.as_ptr().offset(92) as *const core::ffi::c_void);
    let weights_hihi: __m128i =
        LoadUnaligned16(smooth_weights.0.as_ptr().offset(108) as *const core::ffi::c_void);
    let weights5: __m128i = cvtepu8_epi16(weights_hilo);
    let weights6: __m128i = cvtepu8_epi16(_mm_srli_si128::<8>(weights_hilo));
    let weights7: __m128i = cvtepu8_epi16(weights_hihi);
    let weights8: __m128i = cvtepu8_epi16(_mm_srli_si128::<8>(weights_hihi));
    let inverted_weights5: __m128i = _mm_sub_epi16(scale, weights5);
    let inverted_weights6: __m128i = _mm_sub_epi16(scale, weights6);
    let inverted_weights7: __m128i = _mm_sub_epi16(scale, weights7);
    let inverted_weights8: __m128i = _mm_sub_epi16(scale, weights8);
    let scaled_top_right5: __m128i = _mm_mullo_epi16(inverted_weights5, top_right);
    let scaled_top_right6: __m128i = _mm_mullo_epi16(inverted_weights6, top_right);
    let scaled_top_right7: __m128i = _mm_mullo_epi16(inverted_weights7, top_right);
    let scaled_top_right8: __m128i = _mm_mullo_epi16(inverted_weights8, top_right);
    let round: __m128i = _mm_set1_epi16((1 << (SMOOTH_WEIGHT_LOG2_SCALE - 1)) as i16);
    for y_mask in (0x01000100..0x0F0E0F0F).step_by(0x02020202) {
        let mut y_select: __m128i = _mm_set1_epi32(y_mask);
        let mut left_y: __m128i = _mm_shuffle_epi8(left1, y_select);
        write_smooth_directional_sum16(
            dst,
            left_y,
            left_y,
            weights1,
            weights2,
            scaled_top_right1,
            scaled_top_right2,
            round,
        );
        write_smooth_directional_sum16(
            dst.offset(16),
            left_y,
            left_y,
            weights3,
            weights4,
            scaled_top_right3,
            scaled_top_right4,
            round,
        );
        write_smooth_directional_sum16(
            dst.offset(32),
            left_y,
            left_y,
            weights5,
            weights6,
            scaled_top_right5,
            scaled_top_right6,
            round,
        );
        write_smooth_directional_sum16(
            dst.offset(48),
            left_y,
            left_y,
            weights7,
            weights8,
            scaled_top_right7,
            scaled_top_right8,
            round,
        );
        dst = dst.offset(stride);
    }
    let left2: __m128i = cvtepu8_epi16(LoadLo8(left_column.offset(8) as *const core::ffi::c_void));
    for y_mask in (0x01000100..0x0F0E0F0F).step_by(0x02020202) {
        let mut y_select: __m128i = _mm_set1_epi32(y_mask);
        let mut left_y: __m128i = _mm_shuffle_epi8(left2, y_select);
        write_smooth_directional_sum16(
            dst,
            left_y,
            left_y,
            weights1,
            weights2,
            scaled_top_right1,
            scaled_top_right2,
            round,
        );
        write_smooth_directional_sum16(
            dst.offset(16),
            left_y,
            left_y,
            weights3,
            weights4,
            scaled_top_right3,
            scaled_top_right4,
            round,
        );
        write_smooth_directional_sum16(
            dst.offset(32),
            left_y,
            left_y,
            weights5,
            weights6,
            scaled_top_right5,
            scaled_top_right6,
            round,
        );
        write_smooth_directional_sum16(
            dst.offset(48),
            left_y,
            left_y,
            weights7,
            weights8,
            scaled_top_right7,
            scaled_top_right8,
            round,
        );
        dst = dst.offset(stride);
    }
}

// !CONFIG_REALTIME_ONLY || CONFIG_AV1_DECODER
#[no_mangle]
#[target_feature(enable = "ssse3")]
pub unsafe extern "C-unwind" fn aom_smooth_h_predictor_64x32_ssse3(
    mut dst: *mut u8,
    stride: isize,
    top_row: *const u8,
    left_column: *const u8,
) {
    let top_right: __m128i = _mm_set1_epi16(*top_row.offset(63) as i16);
    let left1: __m128i = cvtepu8_epi16(LoadLo8(left_column as *const core::ffi::c_void));
    let weights_lolo: __m128i =
        LoadUnaligned16(smooth_weights.0.as_ptr().offset(60) as *const core::ffi::c_void);
    let weights_lohi: __m128i =
        LoadUnaligned16(smooth_weights.0.as_ptr().offset(76) as *const core::ffi::c_void);
    let scale: __m128i = _mm_set1_epi16((1 << SMOOTH_WEIGHT_LOG2_SCALE) as i16);
    let weights1: __m128i = cvtepu8_epi16(weights_lolo);
    let weights2: __m128i = cvtepu8_epi16(_mm_srli_si128::<8>(weights_lolo));
    let weights3: __m128i = cvtepu8_epi16(weights_lohi);
    let weights4: __m128i = cvtepu8_epi16(_mm_srli_si128::<8>(weights_lohi));
    let inverted_weights1: __m128i = _mm_sub_epi16(scale, weights1);
    let inverted_weights2: __m128i = _mm_sub_epi16(scale, weights2);
    let inverted_weights3: __m128i = _mm_sub_epi16(scale, weights3);
    let inverted_weights4: __m128i = _mm_sub_epi16(scale, weights4);
    let scaled_top_right1: __m128i = _mm_mullo_epi16(inverted_weights1, top_right);
    let scaled_top_right2: __m128i = _mm_mullo_epi16(inverted_weights2, top_right);
    let scaled_top_right3: __m128i = _mm_mullo_epi16(inverted_weights3, top_right);
    let scaled_top_right4: __m128i = _mm_mullo_epi16(inverted_weights4, top_right);
    let weights_hilo: __m128i =
        LoadUnaligned16(smooth_weights.0.as_ptr().offset(92) as *const core::ffi::c_void);
    let weights_hihi: __m128i =
        LoadUnaligned16(smooth_weights.0.as_ptr().offset(108) as *const core::ffi::c_void);
    let weights5: __m128i = cvtepu8_epi16(weights_hilo);
    let weights6: __m128i = cvtepu8_epi16(_mm_srli_si128::<8>(weights_hilo));
    let weights7: __m128i = cvtepu8_epi16(weights_hihi);
    let weights8: __m128i = cvtepu8_epi16(_mm_srli_si128::<8>(weights_hihi));
    let inverted_weights5: __m128i = _mm_sub_epi16(scale, weights5);
    let inverted_weights6: __m128i = _mm_sub_epi16(scale, weights6);
    let inverted_weights7: __m128i = _mm_sub_epi16(scale, weights7);
    let inverted_weights8: __m128i = _mm_sub_epi16(scale, weights8);
    let scaled_top_right5: __m128i = _mm_mullo_epi16(inverted_weights5, top_right);
    let scaled_top_right6: __m128i = _mm_mullo_epi16(inverted_weights6, top_right);
    let scaled_top_right7: __m128i = _mm_mullo_epi16(inverted_weights7, top_right);
    let scaled_top_right8: __m128i = _mm_mullo_epi16(inverted_weights8, top_right);
    let round: __m128i = _mm_set1_epi16((1 << (SMOOTH_WEIGHT_LOG2_SCALE - 1)) as i16);
    for y_mask in (0x01000100..0x0F0E0F0F).step_by(0x02020202) {
        let y_select: __m128i = _mm_set1_epi32(y_mask);
        let left_y: __m128i = _mm_shuffle_epi8(left1, y_select);
        write_smooth_directional_sum16(
            dst,
            left_y,
            left_y,
            weights1,
            weights2,
            scaled_top_right1,
            scaled_top_right2,
            round,
        );
        write_smooth_directional_sum16(
            dst.offset(16),
            left_y,
            left_y,
            weights3,
            weights4,
            scaled_top_right3,
            scaled_top_right4,
            round,
        );
        write_smooth_directional_sum16(
            dst.offset(32),
            left_y,
            left_y,
            weights5,
            weights6,
            scaled_top_right5,
            scaled_top_right6,
            round,
        );
        write_smooth_directional_sum16(
            dst.offset(48),
            left_y,
            left_y,
            weights7,
            weights8,
            scaled_top_right7,
            scaled_top_right8,
            round,
        );
        dst = dst.offset(stride);
    }
    let left2: __m128i = cvtepu8_epi16(LoadLo8(left_column.offset(8) as *const core::ffi::c_void));
    for y_mask in (0x01000100..0x0F0E0F0F).step_by(0x02020202) {
        let y_select: __m128i = _mm_set1_epi32(y_mask);
        let left_y: __m128i = _mm_shuffle_epi8(left2, y_select);
        write_smooth_directional_sum16(
            dst,
            left_y,
            left_y,
            weights1,
            weights2,
            scaled_top_right1,
            scaled_top_right2,
            round,
        );
        write_smooth_directional_sum16(
            dst.offset(16),
            left_y,
            left_y,
            weights3,
            weights4,
            scaled_top_right3,
            scaled_top_right4,
            round,
        );
        write_smooth_directional_sum16(
            dst.offset(32),
            left_y,
            left_y,
            weights5,
            weights6,
            scaled_top_right5,
            scaled_top_right6,
            round,
        );
        write_smooth_directional_sum16(
            dst.offset(48),
            left_y,
            left_y,
            weights7,
            weights8,
            scaled_top_right7,
            scaled_top_right8,
            round,
        );
        dst = dst.offset(stride);
    }
    let left3: __m128i = cvtepu8_epi16(LoadLo8(left_column.offset(16) as *const core::ffi::c_void));
    for y_mask in (0x01000100..0x0F0E0F0F).step_by(0x02020202) {
        let y_select: __m128i = _mm_set1_epi32(y_mask);
        let left_y: __m128i = _mm_shuffle_epi8(left3, y_select);
        write_smooth_directional_sum16(
            dst,
            left_y,
            left_y,
            weights1,
            weights2,
            scaled_top_right1,
            scaled_top_right2,
            round,
        );
        write_smooth_directional_sum16(
            dst.offset(16),
            left_y,
            left_y,
            weights3,
            weights4,
            scaled_top_right3,
            scaled_top_right4,
            round,
        );
        write_smooth_directional_sum16(
            dst.offset(32),
            left_y,
            left_y,
            weights5,
            weights6,
            scaled_top_right5,
            scaled_top_right6,
            round,
        );
        write_smooth_directional_sum16(
            dst.offset(48),
            left_y,
            left_y,
            weights7,
            weights8,
            scaled_top_right7,
            scaled_top_right8,
            round,
        );
        dst = dst.offset(stride);
    }
    let left4: __m128i = cvtepu8_epi16(LoadLo8(left_column.offset(24) as *const core::ffi::c_void));
    for y_mask in (0x01000100..0x0F0E0F0F).step_by(0x02020202) {
        let y_select: __m128i = _mm_set1_epi32(y_mask);
        let left_y: __m128i = _mm_shuffle_epi8(left4, y_select);
        write_smooth_directional_sum16(
            dst,
            left_y,
            left_y,
            weights1,
            weights2,
            scaled_top_right1,
            scaled_top_right2,
            round,
        );
        write_smooth_directional_sum16(
            dst.offset(16),
            left_y,
            left_y,
            weights3,
            weights4,
            scaled_top_right3,
            scaled_top_right4,
            round,
        );
        write_smooth_directional_sum16(
            dst.offset(32),
            left_y,
            left_y,
            weights5,
            weights6,
            scaled_top_right5,
            scaled_top_right6,
            round,
        );
        write_smooth_directional_sum16(
            dst.offset(48),
            left_y,
            left_y,
            weights7,
            weights8,
            scaled_top_right7,
            scaled_top_right8,
            round,
        );
        dst = dst.offset(stride);
    }
}

#[no_mangle]
#[target_feature(enable = "ssse3")]
pub unsafe extern "C-unwind" fn aom_smooth_h_predictor_64x64_ssse3(
    mut dst: *mut u8,
    stride: isize,
    top_row: *const u8,
    left_column: *const u8,
) {
    let top_right: __m128i = _mm_set1_epi16(*top_row.offset(63) as i16);
    let weights_lolo: __m128i =
        LoadUnaligned16(smooth_weights.0.as_ptr().offset(60) as *const core::ffi::c_void);
    let weights_lohi: __m128i =
        LoadUnaligned16(smooth_weights.0.as_ptr().offset(76) as *const core::ffi::c_void);
    let scale: __m128i = _mm_set1_epi16((1 << SMOOTH_WEIGHT_LOG2_SCALE) as i16);
    let weights1: __m128i = cvtepu8_epi16(weights_lolo);
    let weights2: __m128i = cvtepu8_epi16(_mm_srli_si128::<8>(weights_lolo));
    let weights3: __m128i = cvtepu8_epi16(weights_lohi);
    let weights4: __m128i = cvtepu8_epi16(_mm_srli_si128::<8>(weights_lohi));
    let inverted_weights1: __m128i = _mm_sub_epi16(scale, weights1);
    let inverted_weights2: __m128i = _mm_sub_epi16(scale, weights2);
    let inverted_weights3: __m128i = _mm_sub_epi16(scale, weights3);
    let inverted_weights4: __m128i = _mm_sub_epi16(scale, weights4);
    let scaled_top_right1: __m128i = _mm_mullo_epi16(inverted_weights1, top_right);
    let scaled_top_right2: __m128i = _mm_mullo_epi16(inverted_weights2, top_right);
    let scaled_top_right3: __m128i = _mm_mullo_epi16(inverted_weights3, top_right);
    let scaled_top_right4: __m128i = _mm_mullo_epi16(inverted_weights4, top_right);
    let weights_hilo: __m128i =
        LoadUnaligned16(smooth_weights.0.as_ptr().offset(92) as *const core::ffi::c_void);
    let weights_hihi: __m128i =
        LoadUnaligned16(smooth_weights.0.as_ptr().offset(108) as *const core::ffi::c_void);
    let weights5: __m128i = cvtepu8_epi16(weights_hilo);
    let weights6: __m128i = cvtepu8_epi16(_mm_srli_si128::<8>(weights_hilo));
    let weights7: __m128i = cvtepu8_epi16(weights_hihi);
    let weights8: __m128i = cvtepu8_epi16(_mm_srli_si128::<8>(weights_hihi));
    let inverted_weights5: __m128i = _mm_sub_epi16(scale, weights5);
    let inverted_weights6: __m128i = _mm_sub_epi16(scale, weights6);
    let inverted_weights7: __m128i = _mm_sub_epi16(scale, weights7);
    let inverted_weights8: __m128i = _mm_sub_epi16(scale, weights8);
    let scaled_top_right5: __m128i = _mm_mullo_epi16(inverted_weights5, top_right);
    let scaled_top_right6: __m128i = _mm_mullo_epi16(inverted_weights6, top_right);
    let scaled_top_right7: __m128i = _mm_mullo_epi16(inverted_weights7, top_right);
    let scaled_top_right8: __m128i = _mm_mullo_epi16(inverted_weights8, top_right);
    let round: __m128i = _mm_set1_epi16((1 << (SMOOTH_WEIGHT_LOG2_SCALE - 1)) as i16);
    for left_offset in (0..64).step_by(8) {
        let left: __m128i = cvtepu8_epi16(LoadLo8(
            left_column.offset(left_offset as isize) as *const core::ffi::c_void
        ));
        for y_mask in (0x01000100..0x0F0E0F0F).step_by(0x02020202) {
            let y_select: __m128i = _mm_set1_epi32(y_mask);
            let left_y: __m128i = _mm_shuffle_epi8(left, y_select);
            write_smooth_directional_sum16(
                dst,
                left_y,
                left_y,
                weights1,
                weights2,
                scaled_top_right1,
                scaled_top_right2,
                round,
            );
            write_smooth_directional_sum16(
                dst.offset(16),
                left_y,
                left_y,
                weights3,
                weights4,
                scaled_top_right3,
                scaled_top_right4,
                round,
            );
            write_smooth_directional_sum16(
                dst.offset(32),
                left_y,
                left_y,
                weights5,
                weights6,
                scaled_top_right5,
                scaled_top_right6,
                round,
            );
            write_smooth_directional_sum16(
                dst.offset(48),
                left_y,
                left_y,
                weights7,
                weights8,
                scaled_top_right7,
                scaled_top_right8,
                round,
            );
            dst = dst.offset(stride);
        }
    }
}
