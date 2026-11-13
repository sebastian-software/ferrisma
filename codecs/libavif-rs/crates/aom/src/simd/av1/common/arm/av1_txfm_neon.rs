/*
 *
 * Copyright (c) 2018, Alliance for Open Media. All rights reserved.
 *
 * This source code is subject to the terms of the BSD 2 Clause License and
 * the Alliance for Open Media Patent License 1.0. If the BSD 2 Clause License
 * was not distributed with this source code in the LICENSE file, you can
 * obtain it at www.aomedia.org/license/software. If the Alliance for Open
 * Media Patent License 1.0 was not distributed with this source code in the
 * PATENTS file, you can obtain it at www.aomedia.org/license/patent.
 */

// Translated by hand from upstream/aom/av1/common/arm/av1_txfm_neon.c
// (ADR 0056). Intrinsic sequence and names follow the original. For a
// negative `bit` (left shift), av1_round_shift_array_c saturates to the int32
// range, while VRSHL wraps; the two agree whenever the shifted value fits in
// int32, which holds for the original test's inputs.
use core::arch::aarch64::*;

#[no_mangle]
pub unsafe extern "C-unwind" fn av1_round_shift_array_neon(mut arr: *mut i32, size: i32, bit: i32) {
    debug_assert!(size % 4 == 0);
    if bit == 0 {
        return;
    }
    let dup_bits_n_32x4: int32x4_t = vdupq_n_s32(-bit);
    let mut i: i32 = 0;
    while i < size {
        let mut tmp_q_s32: int32x4_t = vld1q_s32(arr);
        tmp_q_s32 = vrshlq_s32(tmp_q_s32, dup_bits_n_32x4);
        vst1q_s32(arr, tmp_q_s32);
        arr = arr.add(4);
        i += 4;
    }
}
