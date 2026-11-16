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

// Translated from upstream/aom/av1/encoder/x86/encodetxb_sse2.c (ADR 0056, ADR 0058).
// Upstream compiles it with -msse2. The functions were
// transliterated with a typing-aware C-subset printer (not C2Rust) and
// reviewed against the C function by function: the same sequence of intrinsic
// and helper calls, constants and operators in order (ADR 0058).
// The TX_PAD_HOR, SIG_COEF_CONTEXTS_2D and TX_CLASS constants and get_txb_bhl,
// get_txb_wide and get_txb_high (with av1_get_adjusted_tx_size and
// tx_size_high_log2) of av1/common/txb_common.h, blockd.h and common_data.h
// were transcribed by hand (tools/x86_c2r/parts/txb_common_sse2.hand.rs).
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

use crate::simd::aom_dsp__x86__mem_sse2::{
    load_8bit_4x4_to_1_reg_sse2, load_8bit_8x2_to_1_reg_sse2,
};
use crate::simd::av1__common__x86__av1_inv_txfm_ssse3::{tx_size_high, tx_size_wide};
use core::arch::x86_64::*;

// From av1/common/enums.h and av1/common/entropy.h.
const TX_PAD_HOR: i32 = 4;
const SIG_COEF_CONTEXTS_2D: i32 = 26;
const TX_CLASS_2D: u8 = 0;
const TX_CLASS_HORIZ: u8 = 1;
const TX_CLASS_VERT: u8 = 2;
const TX_32X32: u8 = 3;
const TX_64X64: u8 = 4;
const TX_32X16: u8 = 10;
const TX_16X32: u8 = 9;
const TX_32X64: u8 = 11;
const TX_64X32: u8 = 12;
const TX_16X64: u8 = 17;
const TX_64X16: u8 = 18;

// From av1/common/common_data.h (tx_size_wide and tx_size_high come from
// crate::simd::av1__common__x86__av1_inv_txfm_ssse3).
static tx_size_high_log2: [i32; 19] = [2, 3, 4, 5, 6, 3, 2, 4, 3, 5, 4, 6, 5, 4, 2, 5, 3, 6, 4];

// From av1/common/blockd.h.
#[inline]
fn av1_get_adjusted_tx_size(tx_size: u8) -> u8 {
    match tx_size {
        TX_64X64 | TX_64X32 | TX_32X64 => TX_32X32,
        TX_64X16 => TX_32X16,
        TX_16X64 => TX_16X32,
        _ => tx_size,
    }
}

// From av1/common/txb_common.h.
#[inline]
fn get_txb_bhl(tx_size: u8) -> i32 {
    tx_size_high_log2[av1_get_adjusted_tx_size(tx_size) as usize]
}

#[inline]
fn get_txb_wide(tx_size: u8) -> i32 {
    tx_size_wide[av1_get_adjusted_tx_size(tx_size) as usize]
}

#[inline]
fn get_txb_high(tx_size: u8) -> i32 {
    tx_size_high[av1_get_adjusted_tx_size(tx_size) as usize]
}

// SSE2
#[inline]
#[target_feature(enable = "sse2")]
unsafe fn load_levels_4x4x5_sse2(
    src: *const u8,
    stride: i32,
    offsets: *const isize,
    level: *mut __m128i,
) {
    *level = load_8bit_4x4_to_1_reg_sse2(src.offset(1) as *const core::ffi::c_void, stride);
    *level.offset(1) = load_8bit_4x4_to_1_reg_sse2(
        src.offset(stride as isize) as *const core::ffi::c_void,
        stride,
    );
    *level.offset(2) =
        load_8bit_4x4_to_1_reg_sse2(src.offset(*offsets) as *const core::ffi::c_void, stride);
    *level.offset(3) = load_8bit_4x4_to_1_reg_sse2(
        src.offset(*offsets.offset(1)) as *const core::ffi::c_void,
        stride,
    );
    *level.offset(4) = load_8bit_4x4_to_1_reg_sse2(
        src.offset(*offsets.offset(2)) as *const core::ffi::c_void,
        stride,
    );
}

#[inline]
#[target_feature(enable = "sse2")]
unsafe fn load_levels_8x2x5_sse2(
    src: *const u8,
    stride: i32,
    offsets: *const isize,
    level: *mut __m128i,
) {
    *level = load_8bit_8x2_to_1_reg_sse2(src.offset(1) as *const core::ffi::c_void, stride);
    *level.offset(1) = load_8bit_8x2_to_1_reg_sse2(
        src.offset(stride as isize) as *const core::ffi::c_void,
        stride,
    );
    *level.offset(2) =
        load_8bit_8x2_to_1_reg_sse2(src.offset(*offsets) as *const core::ffi::c_void, stride);
    *level.offset(3) = load_8bit_8x2_to_1_reg_sse2(
        src.offset(*offsets.offset(1)) as *const core::ffi::c_void,
        stride,
    );
    *level.offset(4) = load_8bit_8x2_to_1_reg_sse2(
        src.offset(*offsets.offset(2)) as *const core::ffi::c_void,
        stride,
    );
}

#[inline]
#[target_feature(enable = "sse2")]
unsafe fn load_levels_16x1x5_sse2(
    src: *const u8,
    stride: i32,
    offsets: *const isize,
    level: *mut __m128i,
) {
    *level = _mm_loadu_si128(src.offset(1) as *mut __m128i as *const __m128i);
    *level.offset(1) =
        _mm_loadu_si128(src.offset(stride as isize) as *mut __m128i as *const __m128i);
    *level.offset(2) = _mm_loadu_si128(src.offset(*offsets) as *mut __m128i as *const __m128i);
    *level.offset(3) =
        _mm_loadu_si128(src.offset(*offsets.offset(1)) as *mut __m128i as *const __m128i);
    *level.offset(4) =
        _mm_loadu_si128(src.offset(*offsets.offset(2)) as *mut __m128i as *const __m128i);
}

#[inline]
#[target_feature(enable = "sse2")]
unsafe fn get_coeff_contexts_kernel_sse2(level: *mut __m128i) -> __m128i {
    let const_3: __m128i = _mm_set1_epi8(3);
    let const_4: __m128i = _mm_set1_epi8(4);
    let mut count: __m128i;
    count = _mm_min_epu8(*level, const_3);
    *level.offset(1) = _mm_min_epu8(*level.offset(1), const_3);
    *level.offset(2) = _mm_min_epu8(*level.offset(2), const_3);
    *level.offset(3) = _mm_min_epu8(*level.offset(3), const_3);
    *level.offset(4) = _mm_min_epu8(*level.offset(4), const_3);
    count = _mm_add_epi8(count, *level.offset(1));
    count = _mm_add_epi8(count, *level.offset(2));
    count = _mm_add_epi8(count, *level.offset(3));
    count = _mm_add_epi8(count, *level.offset(4));
    count = _mm_avg_epu8(count, _mm_setzero_si128());
    count = _mm_min_epu8(count, const_4);
    return count;
}

#[inline]
#[target_feature(enable = "sse2")]
unsafe fn get_4_nz_map_contexts_2d(
    mut levels: *const u8,
    width: i32,
    offsets: *const isize,
    coeff_contexts: *mut i8,
) {
    let stride: i32 = 4 + TX_PAD_HOR;
    let pos_to_offset_large: __m128i = _mm_set1_epi8(21);
    let mut pos_to_offset: __m128i = if width == 4 {
        _mm_setr_epi8(0, 1, 6, 6, 1, 6, 6, 21, 6, 6, 21, 21, 6, 21, 21, 21)
    } else {
        _mm_setr_epi8(0, 16, 16, 16, 16, 16, 16, 16, 6, 6, 21, 21, 6, 21, 21, 21)
    };
    let mut count: __m128i;
    let mut level: [__m128i; 5] = core::mem::zeroed();
    let mut cc: *mut i8 = coeff_contexts;
    let mut col: i32 = width;
    debug_assert!(!(width % 4 != 0));
    loop {
        load_levels_4x4x5_sse2(levels, stride, offsets, level.as_mut_ptr());
        count = get_coeff_contexts_kernel_sse2(level.as_mut_ptr());
        count = _mm_add_epi8(count, pos_to_offset);
        _mm_store_si128(cc as *mut __m128i, count);
        pos_to_offset = pos_to_offset_large;
        levels = levels.offset((4 * stride) as isize);
        cc = cc.offset(16);
        col -= 4;
        if !(col != 0) {
            break;
        }
    }
    *coeff_contexts = 0;
}

#[inline]
#[target_feature(enable = "sse2")]
unsafe fn get_4_nz_map_contexts_ver(
    mut levels: *const u8,
    width: i32,
    offsets: *const isize,
    mut coeff_contexts: *mut i8,
) {
    let stride: i32 = 4 + TX_PAD_HOR;
    let pos_to_offset: __m128i = _mm_setr_epi8(
        (SIG_COEF_CONTEXTS_2D + 0) as i8,
        (SIG_COEF_CONTEXTS_2D + 5) as i8,
        (SIG_COEF_CONTEXTS_2D + 10) as i8,
        (SIG_COEF_CONTEXTS_2D + 10) as i8,
        (SIG_COEF_CONTEXTS_2D + 0) as i8,
        (SIG_COEF_CONTEXTS_2D + 5) as i8,
        (SIG_COEF_CONTEXTS_2D + 10) as i8,
        (SIG_COEF_CONTEXTS_2D + 10) as i8,
        (SIG_COEF_CONTEXTS_2D + 0) as i8,
        (SIG_COEF_CONTEXTS_2D + 5) as i8,
        (SIG_COEF_CONTEXTS_2D + 10) as i8,
        (SIG_COEF_CONTEXTS_2D + 10) as i8,
        (SIG_COEF_CONTEXTS_2D + 0) as i8,
        (SIG_COEF_CONTEXTS_2D + 5) as i8,
        (SIG_COEF_CONTEXTS_2D + 10) as i8,
        (SIG_COEF_CONTEXTS_2D + 10) as i8,
    );
    let mut count: __m128i;
    let mut level: [__m128i; 5] = core::mem::zeroed();
    let mut col: i32 = width;
    debug_assert!(!(width % 4 != 0));
    loop {
        load_levels_4x4x5_sse2(levels, stride, offsets, level.as_mut_ptr());
        count = get_coeff_contexts_kernel_sse2(level.as_mut_ptr());
        count = _mm_add_epi8(count, pos_to_offset);
        _mm_store_si128(coeff_contexts as *mut __m128i, count);
        levels = levels.offset((4 * stride) as isize);
        coeff_contexts = coeff_contexts.offset(16);
        col -= 4;
        if !(col != 0) {
            break;
        }
    }
}

#[inline]
#[target_feature(enable = "sse2")]
unsafe fn get_4_nz_map_contexts_hor(
    mut levels: *const u8,
    width: i32,
    offsets: *const isize,
    mut coeff_contexts: *mut i8,
) {
    let stride: i32 = 4 + TX_PAD_HOR;
    let pos_to_offset_large: __m128i = _mm_set1_epi8((SIG_COEF_CONTEXTS_2D + 10) as i8);
    let mut pos_to_offset: __m128i = _mm_setr_epi8(
        (SIG_COEF_CONTEXTS_2D + 0) as i8,
        (SIG_COEF_CONTEXTS_2D + 0) as i8,
        (SIG_COEF_CONTEXTS_2D + 0) as i8,
        (SIG_COEF_CONTEXTS_2D + 0) as i8,
        (SIG_COEF_CONTEXTS_2D + 5) as i8,
        (SIG_COEF_CONTEXTS_2D + 5) as i8,
        (SIG_COEF_CONTEXTS_2D + 5) as i8,
        (SIG_COEF_CONTEXTS_2D + 5) as i8,
        (SIG_COEF_CONTEXTS_2D + 10) as i8,
        (SIG_COEF_CONTEXTS_2D + 10) as i8,
        (SIG_COEF_CONTEXTS_2D + 10) as i8,
        (SIG_COEF_CONTEXTS_2D + 10) as i8,
        (SIG_COEF_CONTEXTS_2D + 10) as i8,
        (SIG_COEF_CONTEXTS_2D + 10) as i8,
        (SIG_COEF_CONTEXTS_2D + 10) as i8,
        (SIG_COEF_CONTEXTS_2D + 10) as i8,
    );
    let mut count: __m128i;
    let mut level: [__m128i; 5] = core::mem::zeroed();
    let mut col: i32 = width;
    debug_assert!(!(width % 4 != 0));
    loop {
        load_levels_4x4x5_sse2(levels, stride, offsets, level.as_mut_ptr());
        count = get_coeff_contexts_kernel_sse2(level.as_mut_ptr());
        count = _mm_add_epi8(count, pos_to_offset);
        _mm_store_si128(coeff_contexts as *mut __m128i, count);
        pos_to_offset = pos_to_offset_large;
        levels = levels.offset((4 * stride) as isize);
        coeff_contexts = coeff_contexts.offset(16);
        col -= 4;
        if !(col != 0) {
            break;
        }
    }
}

#[inline]
#[target_feature(enable = "sse2")]
unsafe fn get_8_coeff_contexts_2d(
    mut levels: *const u8,
    width: i32,
    offsets: *const isize,
    coeff_contexts: *mut i8,
) {
    let stride: i32 = 8 + TX_PAD_HOR;
    let mut cc: *mut i8 = coeff_contexts;
    let mut col: i32 = width;
    let mut count: __m128i;
    let mut level: [__m128i; 5] = core::mem::zeroed();
    let mut pos_to_offset: [__m128i; 3] = core::mem::zeroed();
    debug_assert!(!(width % 2 != 0));
    if width == 8 {
        pos_to_offset[0] = _mm_setr_epi8(0, 1, 6, 6, 21, 21, 21, 21, 1, 6, 6, 21, 21, 21, 21, 21);
        pos_to_offset[1] =
            _mm_setr_epi8(6, 6, 21, 21, 21, 21, 21, 21, 6, 21, 21, 21, 21, 21, 21, 21);
    } else if width < 8 {
        pos_to_offset[0] =
            _mm_setr_epi8(0, 11, 6, 6, 21, 21, 21, 21, 11, 11, 6, 21, 21, 21, 21, 21);
        pos_to_offset[1] = _mm_setr_epi8(
            11, 11, 21, 21, 21, 21, 21, 21, 11, 11, 21, 21, 21, 21, 21, 21,
        );
    } else {
        pos_to_offset[0] = _mm_setr_epi8(
            0, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16,
        );
        pos_to_offset[1] =
            _mm_setr_epi8(6, 6, 21, 21, 21, 21, 21, 21, 6, 21, 21, 21, 21, 21, 21, 21);
    }
    pos_to_offset[2] = _mm_set1_epi8(21);
    loop {
        load_levels_8x2x5_sse2(levels, stride, offsets, level.as_mut_ptr());
        count = get_coeff_contexts_kernel_sse2(level.as_mut_ptr());
        count = _mm_add_epi8(count, pos_to_offset[0]);
        _mm_store_si128(cc as *mut __m128i, count);
        pos_to_offset[0] = pos_to_offset[1];
        pos_to_offset[1] = pos_to_offset[2];
        levels = levels.offset((2 * stride) as isize);
        cc = cc.offset(16);
        col -= 2;
        if !(col != 0) {
            break;
        }
    }
    *coeff_contexts = 0;
}

#[inline]
#[target_feature(enable = "sse2")]
unsafe fn get_8_coeff_contexts_ver(
    mut levels: *const u8,
    width: i32,
    offsets: *const isize,
    mut coeff_contexts: *mut i8,
) {
    let stride: i32 = 8 + TX_PAD_HOR;
    let pos_to_offset: __m128i = _mm_setr_epi8(
        (SIG_COEF_CONTEXTS_2D + 0) as i8,
        (SIG_COEF_CONTEXTS_2D + 5) as i8,
        (SIG_COEF_CONTEXTS_2D + 10) as i8,
        (SIG_COEF_CONTEXTS_2D + 10) as i8,
        (SIG_COEF_CONTEXTS_2D + 10) as i8,
        (SIG_COEF_CONTEXTS_2D + 10) as i8,
        (SIG_COEF_CONTEXTS_2D + 10) as i8,
        (SIG_COEF_CONTEXTS_2D + 10) as i8,
        (SIG_COEF_CONTEXTS_2D + 0) as i8,
        (SIG_COEF_CONTEXTS_2D + 5) as i8,
        (SIG_COEF_CONTEXTS_2D + 10) as i8,
        (SIG_COEF_CONTEXTS_2D + 10) as i8,
        (SIG_COEF_CONTEXTS_2D + 10) as i8,
        (SIG_COEF_CONTEXTS_2D + 10) as i8,
        (SIG_COEF_CONTEXTS_2D + 10) as i8,
        (SIG_COEF_CONTEXTS_2D + 10) as i8,
    );
    let mut col: i32 = width;
    let mut count: __m128i;
    let mut level: [__m128i; 5] = core::mem::zeroed();
    debug_assert!(!(width % 2 != 0));
    loop {
        load_levels_8x2x5_sse2(levels, stride, offsets, level.as_mut_ptr());
        count = get_coeff_contexts_kernel_sse2(level.as_mut_ptr());
        count = _mm_add_epi8(count, pos_to_offset);
        _mm_store_si128(coeff_contexts as *mut __m128i, count);
        levels = levels.offset((2 * stride) as isize);
        coeff_contexts = coeff_contexts.offset(16);
        col -= 2;
        if !(col != 0) {
            break;
        }
    }
}

#[inline]
#[target_feature(enable = "sse2")]
unsafe fn get_8_coeff_contexts_hor(
    mut levels: *const u8,
    width: i32,
    offsets: *const isize,
    mut coeff_contexts: *mut i8,
) {
    let stride: i32 = 8 + TX_PAD_HOR;
    let pos_to_offset_large: __m128i = _mm_set1_epi8((SIG_COEF_CONTEXTS_2D + 10) as i8);
    let mut pos_to_offset: __m128i = _mm_setr_epi8(
        (SIG_COEF_CONTEXTS_2D + 0) as i8,
        (SIG_COEF_CONTEXTS_2D + 0) as i8,
        (SIG_COEF_CONTEXTS_2D + 0) as i8,
        (SIG_COEF_CONTEXTS_2D + 0) as i8,
        (SIG_COEF_CONTEXTS_2D + 0) as i8,
        (SIG_COEF_CONTEXTS_2D + 0) as i8,
        (SIG_COEF_CONTEXTS_2D + 0) as i8,
        (SIG_COEF_CONTEXTS_2D + 0) as i8,
        (SIG_COEF_CONTEXTS_2D + 5) as i8,
        (SIG_COEF_CONTEXTS_2D + 5) as i8,
        (SIG_COEF_CONTEXTS_2D + 5) as i8,
        (SIG_COEF_CONTEXTS_2D + 5) as i8,
        (SIG_COEF_CONTEXTS_2D + 5) as i8,
        (SIG_COEF_CONTEXTS_2D + 5) as i8,
        (SIG_COEF_CONTEXTS_2D + 5) as i8,
        (SIG_COEF_CONTEXTS_2D + 5) as i8,
    );
    let mut col: i32 = width;
    let mut count: __m128i;
    let mut level: [__m128i; 5] = core::mem::zeroed();
    debug_assert!(!(width % 2 != 0));
    loop {
        load_levels_8x2x5_sse2(levels, stride, offsets, level.as_mut_ptr());
        count = get_coeff_contexts_kernel_sse2(level.as_mut_ptr());
        count = _mm_add_epi8(count, pos_to_offset);
        _mm_store_si128(coeff_contexts as *mut __m128i, count);
        pos_to_offset = pos_to_offset_large;
        levels = levels.offset((2 * stride) as isize);
        coeff_contexts = coeff_contexts.offset(16);
        col -= 2;
        if !(col != 0) {
            break;
        }
    }
}

#[inline]
#[target_feature(enable = "sse2")]
unsafe fn get_16n_coeff_contexts_2d(
    mut levels: *const u8,
    real_width: i32,
    real_height: i32,
    width: i32,
    height: i32,
    offsets: *const isize,
    coeff_contexts: *mut i8,
) {
    let stride: i32 = height + TX_PAD_HOR;
    let mut cc: *mut i8 = coeff_contexts;
    let mut col: i32 = width;
    let mut pos_to_offset: [__m128i; 5] = core::mem::zeroed();
    let mut pos_to_offset_large: [__m128i; 3] = core::mem::zeroed();
    let mut count: __m128i;
    let mut level: [__m128i; 5] = core::mem::zeroed();
    debug_assert!(!(height % 16 != 0));
    pos_to_offset_large[2] = _mm_set1_epi8(21);
    if real_width == real_height {
        pos_to_offset[0] =
            _mm_setr_epi8(0, 1, 6, 6, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21);
        pos_to_offset[1] =
            _mm_setr_epi8(1, 6, 6, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21);
        pos_to_offset[2] =
            _mm_setr_epi8(6, 6, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21);
        pos_to_offset[3] = _mm_setr_epi8(
            6, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21,
        );
        pos_to_offset_large[1] = pos_to_offset_large[2];
        pos_to_offset_large[0] = pos_to_offset_large[1];
        pos_to_offset[4] = pos_to_offset_large[0];
    } else if real_width < real_height {
        pos_to_offset[0] =
            _mm_setr_epi8(0, 11, 6, 6, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21);
        pos_to_offset[1] = _mm_setr_epi8(
            11, 11, 6, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21,
        );
        pos_to_offset[4] = _mm_setr_epi8(
            11, 11, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21,
        );
        pos_to_offset[3] = pos_to_offset[4];
        pos_to_offset[2] = pos_to_offset[3];
        pos_to_offset_large[1] = pos_to_offset_large[2];
        pos_to_offset_large[0] = pos_to_offset_large[1];
    } else {
        // real_width > real_height
        pos_to_offset[1] = _mm_setr_epi8(
            16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16,
        );
        pos_to_offset[0] = pos_to_offset[1];
        pos_to_offset[2] =
            _mm_setr_epi8(6, 6, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21);
        pos_to_offset[3] = _mm_setr_epi8(
            6, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21,
        );
        pos_to_offset[4] = pos_to_offset_large[2];
        pos_to_offset_large[1] = _mm_set1_epi8(16);
        pos_to_offset_large[0] = pos_to_offset_large[1];
    }
    loop {
        let mut h: i32 = height;
        loop {
            load_levels_16x1x5_sse2(levels, stride, offsets, level.as_mut_ptr());
            count = get_coeff_contexts_kernel_sse2(level.as_mut_ptr());
            count = _mm_add_epi8(count, pos_to_offset[0]);
            _mm_store_si128(cc as *mut __m128i, count);
            levels = levels.offset(16);
            cc = cc.offset(16);
            h -= 16;
            pos_to_offset[0] = pos_to_offset_large[0];
            if !(h != 0) {
                break;
            }
        }
        pos_to_offset[0] = pos_to_offset[1];
        pos_to_offset[1] = pos_to_offset[2];
        pos_to_offset[2] = pos_to_offset[3];
        pos_to_offset[3] = pos_to_offset[4];
        pos_to_offset_large[0] = pos_to_offset_large[1];
        pos_to_offset_large[1] = pos_to_offset_large[2];
        levels = levels.offset(TX_PAD_HOR as isize);
        col -= 1;
        if !(col != 0) {
            break;
        }
    }
    *coeff_contexts = 0;
}

#[inline]
#[target_feature(enable = "sse2")]
unsafe fn get_16n_coeff_contexts_ver(
    mut levels: *const u8,
    width: i32,
    height: i32,
    offsets: *const isize,
    mut coeff_contexts: *mut i8,
) {
    let stride: i32 = height + TX_PAD_HOR;
    let pos_to_offset_large: __m128i = _mm_setr_epi8(
        (SIG_COEF_CONTEXTS_2D + 10) as i8,
        (SIG_COEF_CONTEXTS_2D + 10) as i8,
        (SIG_COEF_CONTEXTS_2D + 10) as i8,
        (SIG_COEF_CONTEXTS_2D + 10) as i8,
        (SIG_COEF_CONTEXTS_2D + 10) as i8,
        (SIG_COEF_CONTEXTS_2D + 10) as i8,
        (SIG_COEF_CONTEXTS_2D + 10) as i8,
        (SIG_COEF_CONTEXTS_2D + 10) as i8,
        (SIG_COEF_CONTEXTS_2D + 10) as i8,
        (SIG_COEF_CONTEXTS_2D + 10) as i8,
        (SIG_COEF_CONTEXTS_2D + 10) as i8,
        (SIG_COEF_CONTEXTS_2D + 10) as i8,
        (SIG_COEF_CONTEXTS_2D + 10) as i8,
        (SIG_COEF_CONTEXTS_2D + 10) as i8,
        (SIG_COEF_CONTEXTS_2D + 10) as i8,
        (SIG_COEF_CONTEXTS_2D + 10) as i8,
    );
    let mut count: __m128i;
    let mut level: [__m128i; 5] = core::mem::zeroed();
    let mut col: i32 = width;
    debug_assert!(!(height % 16 != 0));
    loop {
        let mut pos_to_offset: __m128i = _mm_setr_epi8(
            (SIG_COEF_CONTEXTS_2D + 0) as i8,
            (SIG_COEF_CONTEXTS_2D + 5) as i8,
            (SIG_COEF_CONTEXTS_2D + 10) as i8,
            (SIG_COEF_CONTEXTS_2D + 10) as i8,
            (SIG_COEF_CONTEXTS_2D + 10) as i8,
            (SIG_COEF_CONTEXTS_2D + 10) as i8,
            (SIG_COEF_CONTEXTS_2D + 10) as i8,
            (SIG_COEF_CONTEXTS_2D + 10) as i8,
            (SIG_COEF_CONTEXTS_2D + 10) as i8,
            (SIG_COEF_CONTEXTS_2D + 10) as i8,
            (SIG_COEF_CONTEXTS_2D + 10) as i8,
            (SIG_COEF_CONTEXTS_2D + 10) as i8,
            (SIG_COEF_CONTEXTS_2D + 10) as i8,
            (SIG_COEF_CONTEXTS_2D + 10) as i8,
            (SIG_COEF_CONTEXTS_2D + 10) as i8,
            (SIG_COEF_CONTEXTS_2D + 10) as i8,
        );
        let mut h: i32 = height;
        loop {
            load_levels_16x1x5_sse2(levels, stride, offsets, level.as_mut_ptr());
            count = get_coeff_contexts_kernel_sse2(level.as_mut_ptr());
            count = _mm_add_epi8(count, pos_to_offset);
            _mm_store_si128(coeff_contexts as *mut __m128i, count);
            pos_to_offset = pos_to_offset_large;
            levels = levels.offset(16);
            coeff_contexts = coeff_contexts.offset(16);
            h -= 16;
            if !(h != 0) {
                break;
            }
        }
        levels = levels.offset(TX_PAD_HOR as isize);
        col -= 1;
        if !(col != 0) {
            break;
        }
    }
}

#[inline]
#[target_feature(enable = "sse2")]
unsafe fn get_16n_coeff_contexts_hor(
    mut levels: *const u8,
    width: i32,
    height: i32,
    offsets: *const isize,
    mut coeff_contexts: *mut i8,
) {
    let stride: i32 = height + TX_PAD_HOR;
    let mut pos_to_offset: [__m128i; 3] = core::mem::zeroed();
    let mut count: __m128i;
    let mut level: [__m128i; 5] = core::mem::zeroed();
    let mut col: i32 = width;
    debug_assert!(!(height % 16 != 0));
    pos_to_offset[0] = _mm_set1_epi8((SIG_COEF_CONTEXTS_2D + 0) as i8);
    pos_to_offset[1] = _mm_set1_epi8((SIG_COEF_CONTEXTS_2D + 5) as i8);
    pos_to_offset[2] = _mm_set1_epi8((SIG_COEF_CONTEXTS_2D + 10) as i8);
    loop {
        let mut h: i32 = height;
        loop {
            load_levels_16x1x5_sse2(levels, stride, offsets, level.as_mut_ptr());
            count = get_coeff_contexts_kernel_sse2(level.as_mut_ptr());
            count = _mm_add_epi8(count, pos_to_offset[0]);
            _mm_store_si128(coeff_contexts as *mut __m128i, count);
            levels = levels.offset(16);
            coeff_contexts = coeff_contexts.offset(16);
            h -= 16;
            if !(h != 0) {
                break;
            }
        }
        pos_to_offset[0] = pos_to_offset[1];
        pos_to_offset[1] = pos_to_offset[2];
        levels = levels.offset(TX_PAD_HOR as isize);
        col -= 1;
        if !(col != 0) {
            break;
        }
    }
}

// Note: levels[] must be in the range [0, 127], inclusive.
#[no_mangle]
#[target_feature(enable = "sse2")]
pub unsafe extern "C-unwind" fn av1_get_nz_map_contexts_sse2(
    levels: *const u8,
    scan: *const i16,
    eob: i32,
    tx_size: u8,
    tx_class: u8,
    coeff_contexts: *mut i8,
) {
    let last_idx: i32 = eob - 1;
    if !(last_idx != 0) {
        *coeff_contexts = 0;
        return;
    }
    let real_width: i32 = tx_size_wide[tx_size as usize];
    let real_height: i32 = tx_size_high[tx_size as usize];
    let width: i32 = get_txb_wide(tx_size);
    let height: i32 = get_txb_high(tx_size);
    let stride: i32 = height + TX_PAD_HOR;
    let mut offsets: [isize; 3] = core::mem::zeroed();
    // coeff_contexts must be 16 byte aligned.
    debug_assert!(!(coeff_contexts as isize & 0xf != 0));
    if (tx_class as i32) == TX_CLASS_2D as i32 {
        offsets[0] = (0 * stride + 2) as isize;
        offsets[1] = (1 * stride + 1) as isize;
        offsets[2] = (2 * stride + 0) as isize;
        if height == 4 {
            get_4_nz_map_contexts_2d(
                levels,
                width,
                offsets.as_mut_ptr() as *const isize,
                coeff_contexts,
            );
        } else if height == 8 {
            get_8_coeff_contexts_2d(
                levels,
                width,
                offsets.as_mut_ptr() as *const isize,
                coeff_contexts,
            );
        } else if height == 16 {
            get_16n_coeff_contexts_2d(
                levels,
                real_width,
                real_height,
                width,
                height,
                offsets.as_mut_ptr() as *const isize,
                coeff_contexts,
            );
        } else {
            get_16n_coeff_contexts_2d(
                levels,
                real_width,
                real_height,
                width,
                height,
                offsets.as_mut_ptr() as *const isize,
                coeff_contexts,
            );
        }
    } else if (tx_class as i32) == TX_CLASS_HORIZ as i32 {
        offsets[0] = (2 * stride) as isize;
        offsets[1] = (3 * stride) as isize;
        offsets[2] = (4 * stride) as isize;
        if height == 4 {
            get_4_nz_map_contexts_hor(
                levels,
                width,
                offsets.as_mut_ptr() as *const isize,
                coeff_contexts,
            );
        } else if height == 8 {
            get_8_coeff_contexts_hor(
                levels,
                width,
                offsets.as_mut_ptr() as *const isize,
                coeff_contexts,
            );
        } else {
            get_16n_coeff_contexts_hor(
                levels,
                width,
                height,
                offsets.as_mut_ptr() as *const isize,
                coeff_contexts,
            );
        }
    } else {
        // TX_CLASS_VERT
        offsets[0] = 2;
        offsets[1] = 3;
        offsets[2] = 4;
        if height == 4 {
            get_4_nz_map_contexts_ver(
                levels,
                width,
                offsets.as_mut_ptr() as *const isize,
                coeff_contexts,
            );
        } else if height == 8 {
            get_8_coeff_contexts_ver(
                levels,
                width,
                offsets.as_mut_ptr() as *const isize,
                coeff_contexts,
            );
        } else {
            get_16n_coeff_contexts_ver(
                levels,
                width,
                height,
                offsets.as_mut_ptr() as *const isize,
                coeff_contexts,
            );
        }
    }
    let bhl: i32 = get_txb_bhl(tx_size);
    let pos: i32 = *scan.offset(last_idx as isize) as i32;
    if last_idx <= (width << bhl) / 8 {
        *coeff_contexts.offset(pos as isize) = 1;
    } else if last_idx <= (width << bhl) / 4 {
        *coeff_contexts.offset(pos as isize) = 2;
    } else {
        *coeff_contexts.offset(pos as isize) = 3;
    }
}
