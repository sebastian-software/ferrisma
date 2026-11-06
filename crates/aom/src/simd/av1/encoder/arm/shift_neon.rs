/*
 * Copyright (c) 2023, Alliance for Open Media. All rights reserved.
 *
 * This source code is subject to the terms of the BSD 2 Clause License and
 * the Alliance for Open Media Patent License 1.0. If the BSD 2 Clause License
 * was not distributed with this source code in the LICENSE file, you can
 * obtain it at www.aomedia.org/license/software. If the Alliance for Open
 * Media Patent License 1.0 was not distributed with this source code in the
 * PATENTS file, you can obtain it at www.aomedia.org/license/patent.
 */

// Translated by hand from upstream/aom/av1/encoder/arm/shift_neon.h (ADR 0056).
// The SHIFT_LOOP_HELPER macro becomes a macro_rules! generating the same named
// functions. Because Rust takes immediate shift counts as const generics, the
// `intrinsic(in[i], arg)` call is written as a one-argument closure-like body
// applied to `in[i]`; the intrinsic and its argument are unchanged.
#![allow(dead_code)]

use core::arch::aarch64::*;

macro_rules! SHIFT_LOOP_HELPER {
    ($name:ident, $type:ty, |$a:ident| $intrinsic_call:expr) => {
        #[inline(always)]
        pub unsafe fn $name(in_: *const $type, out: *mut $type, size: i32) {
            let mut i: i32 = 0;
            loop {
                let $a = *in_.offset(i as isize);
                *out.offset(i as isize) = $intrinsic_call;
                i += 1;
                if !(i < size) {
                    break;
                }
            }
        }
    };
}

SHIFT_LOOP_HELPER!(shift_left_2_s16_x4, int16x4_t, |a| vshl_n_s16::<2>(a));
SHIFT_LOOP_HELPER!(shift_left_2_s16_x8, int16x8_t, |a| vshlq_n_s16::<2>(a));
SHIFT_LOOP_HELPER!(shift_left_2_s32_x4, int32x4_t, |a| vshlq_n_s32::<2>(a));
SHIFT_LOOP_HELPER!(shift_right_2_round_s16_x8, int16x8_t, |a| vrshrq_n_s16::<2>(a));
SHIFT_LOOP_HELPER!(shift_right_2_round_s32_x4, int32x4_t, |a| vrshrq_n_s32::<2>(a));
SHIFT_LOOP_HELPER!(shift_right_4_round_s16_x8, int16x8_t, |a| vrshrq_n_s16::<4>(a));
SHIFT_LOOP_HELPER!(shift_right_4_round_s32_x4, int32x4_t, |a| vrshrq_n_s32::<4>(a));

// Addition instructions have slightly better performance compared to shift
// instructions on some micro-architectures, so use these for shifts by one.

SHIFT_LOOP_HELPER!(shift_left_1_s16_x4, int16x4_t, |a| vadd_s16(a, a));
SHIFT_LOOP_HELPER!(shift_left_1_s16_x8, int16x8_t, |a| vaddq_s16(a, a));
SHIFT_LOOP_HELPER!(shift_right_1_round_s16_x4, int16x4_t, |a| vrhadd_s16(a, vdup_n_s16(0)));
SHIFT_LOOP_HELPER!(shift_right_1_round_s16_x8, int16x8_t, |a| vrhaddq_s16(a, vdupq_n_s16(0)));
SHIFT_LOOP_HELPER!(shift_right_1_round_s32_x4, int32x4_t, |a| vrhaddq_s32(a, vdupq_n_s32(0)));
