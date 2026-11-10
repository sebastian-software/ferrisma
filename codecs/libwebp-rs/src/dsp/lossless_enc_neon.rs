// libwebp v1.6.0, src/dsp/lossless_enc_neon.c:
//
// Copyright 2015 Google Inc. All Rights Reserved.
//
// Use of this source code is governed by a BSD-style license
// that can be found in the COPYING file in the root of the source
// tree. An additional intellectual property rights grant can be found
// in the file PATENTS. All contributing project authors may
// be found in the AUTHORS file in the root of the source tree.
// -----------------------------------------------------------------------------
//
// NEON variant of methods for lossless encoder
//
// Author: Skal (pascal.massimino@gmail.com)

//! Translation of `src/dsp/lossless_enc_neon.c` from libwebp v1.6.0.
//!
//! Work package: WP20 (NEON).
//!
//! The NEON variants of the lossless encoder's subtract-green and color
//! transforms of `src/dsp/lossless_enc.c`. Each `_neon` function is the body
//! that upstream's `VP8LEncDspInitNEON` assigns; the safe entry point below
//! it, named after the function without the suffix, is what the dispatch
//! function of `src/dsp/lossless_enc.rs` calls on 64-bit ARM.
//!
//! `VP8LEncDspInitNEON` assigns `VP8LSubtractGreenFromBlueAndRed` and
//! `VP8LTransformColor`. `VP8LEncDspInit` calls it unconditionally on 64-bit
//! ARM (`WEBP_NEON_OMIT_C_CODE`), where it also skips its own assignments of
//! these two pointers, so the dispatch functions
//! `vp8l_subtract_green_from_blue_and_red` and `vp8l_transform_color` return
//! the NEON entry points there.
//!
//! # Deviations from upstream
//!
//! 1. **Loads, stores and alignment** follow the deviations 1 to 3 of
//!    `src/dsp/neon.rs`. The loads and stores of the bytes of `uint32_t`
//!    pixels (`vld1q_u8((uint8_t*)argb_data)`, `vst1q_s8((int8_t*)...)`) use
//!    the `_words` helpers there, which read and write the same bytes.
//! 2. **Pointers** are slices as in `src/dsp/lossless_enc.rs`: `argb_data`
//!    is the slice from the first pixel, and the C pointer increments are
//!    indices into it. The loop counters are `usize`; the C `int` arithmetic
//!    of the left-over counts (`num_pixels - i`, `num_pixels & 3`) is kept in
//!    `i32`.
//!
//! # Excluded undefined behavior
//!
//! 1. **Negative pixel counts in `SubtractGreenFromBlueAndRed_NEON`.** For
//!    `num_pixels < 0`, C forms `argb_data + (num_pixels & ~3)`, a pointer
//!    before the start of the buffer, and then transforms `num_pixels & 3`
//!    pixels with the C function, where the C function alone transforms
//!    none. The library never passes a negative count (the encoder passes
//!    the picture width times its height); the translation runs no vector
//!    iteration and calls the C function with the same masked count, and the
//!    parity tests compare non-negative counts only.
//!
//! # Checks not applicable
//!
//! 1. **`USE_VTBLQ`** is defined only for Apple clang older than Xcode 6.3
//!    (`__apple_build_version__ < 6020037`), which cannot build this crate's
//!    oracle; the translation takes the `!USE_VTBLQ` branch, the 8-byte
//!    `kGreenShuffle` and `k0g0g` tables with `vtbl1_u8`, on macOS and Linux
//!    alike. The `USE_VTBLQ` branch (`vtbl1q_u8` with the 16-byte tables,
//!    including `kg0g0`) selects the same bytes and is not translated.
//! 2. **`WEBP_RESTRICT`** has no counterpart: the multipliers are a shared
//!    reference and the pixels an exclusive slice.

use core::arch::aarch64::{
    uint8x8_t, uint8x16_t, vaddq_s8, vandq_u32, vcombine_u8, vdupq_n_u32, vget_high_u8,
    vget_low_u8, vqdmulhq_s16, vreinterpretq_s8_s16, vreinterpretq_s8_u8, vreinterpretq_s8_u32,
    vreinterpretq_s16_u8, vreinterpretq_u32_s8, vreinterpretq_u32_s16, vshlq_n_s16, vshrq_n_u32,
    vsubq_s8, vsubq_u8, vtbl1_u8,
};

use crate::dsp::lossless::VP8LMultipliers;
use crate::dsp::lossless_enc::{vp8l_subtract_green_from_blue_and_red_c, vp8l_transform_color_c};
use crate::dsp::neon::{vld1_u8, vld1q_s16, vld1q_u8_words, vst1q_s8_words, vst1q_u8_words};

/// The count of a C loop `for (i = 0; i < num_pixels; ...)`, which runs no
/// iteration for a negative count.
#[must_use]
fn count(num_pixels: i32) -> usize {
    usize::try_from(num_pixels).unwrap_or(0)
}

/// An index below a pixel count, as the C `int` it is.
#[must_use]
fn int(i: usize) -> i32 {
    i32::try_from(i).expect("an index below a pixel count fits an int")
}

//------------------------------------------------------------------------------
// Subtract-Green Transform

// upstream: src/dsp/lossless_enc_neon.c kGreenShuffle
/// 255 = byte will be zeroed
const GREEN_SHUFFLE: [u8; 8] = [1, 255, 1, 255, 5, 255, 5, 255];

// upstream: src/dsp/lossless_enc_neon.c DoGreenShuffle_NEON
#[target_feature(enable = "neon")]
#[must_use]
fn do_green_shuffle_neon(argb: uint8x16_t, shuffle: uint8x8_t) -> uint8x16_t {
    vcombine_u8(
        vtbl1_u8(vget_low_u8(argb), shuffle),
        vtbl1_u8(vget_high_u8(argb), shuffle),
    )
}

// upstream: src/dsp/lossless_enc_neon.c SubtractGreenFromBlueAndRed_NEON
#[target_feature(enable = "neon")]
fn subtract_green_from_blue_and_red_neon(argb_data: &mut [u32], num_pixels: i32) {
    let end = count(num_pixels & !3);
    let shuffle = vld1_u8(&GREEN_SHUFFLE);
    let mut i = 0;
    while i < end {
        let argb = vld1q_u8_words(&argb_data[i..]);
        let greens = do_green_shuffle_neon(argb, shuffle);
        vst1q_u8_words(&mut argb_data[i..], vsubq_u8(argb, greens));
        i += 4;
    }
    // fallthrough and finish off with plain-C
    vp8l_subtract_green_from_blue_and_red_c(&mut argb_data[i..], num_pixels & 3);
}

/// Entry point of `SubtractGreenFromBlueAndRed_NEON` for
/// `VP8LSubtractGreenFromBlueAndRed`.
pub(crate) fn subtract_green_from_blue_and_red(argb_data: &mut [u32], num_pixels: i32) {
    // SAFETY: NEON is in the AArch64 baseline that this module is compiled for,
    // and upstream selects the NEON variant there unconditionally (ADR 0008).
    unsafe { subtract_green_from_blue_and_red_neon(argb_data, num_pixels) }
}

//------------------------------------------------------------------------------
// Color Transform

// upstream: src/dsp/lossless_enc_neon.c CST
/// sign-extended multiplying constants, pre-shifted by 6.
#[must_use]
fn cst(x: u8) -> i16 {
    (u16::from(x) << 8).cast_signed() >> 6
}

// upstream: src/dsp/lossless_enc_neon.c TransformColor_NEON
#[target_feature(enable = "neon")]
#[expect(
    clippy::many_single_char_names,
    reason = "the C names m, A, B, C, D, E and F"
)]
#[expect(
    clippy::trivially_copy_pass_by_ref,
    reason = "the C function takes a pointer to the multipliers"
)]
fn transform_color_neon(m: &VP8LMultipliers, argb_data: &mut [u32], num_pixels: i32) {
    // upstream: src/dsp/lossless_enc_neon.c k0g0g
    const K0G0G: [u8; 8] = [255, 1, 255, 1, 255, 5, 255, 5];
    let rb: [i16; 8] = [
        cst(m.green_to_blue),
        cst(m.green_to_red),
        cst(m.green_to_blue),
        cst(m.green_to_red),
        cst(m.green_to_blue),
        cst(m.green_to_red),
        cst(m.green_to_blue),
        cst(m.green_to_red),
    ];
    let mults_rb = vld1q_s16(&rb);
    let b2: [i16; 8] = [
        0,
        cst(m.red_to_blue),
        0,
        cst(m.red_to_blue),
        0,
        cst(m.red_to_blue),
        0,
        cst(m.red_to_blue),
    ];
    let mults_b2 = vld1q_s16(&b2);
    let shuffle = vld1_u8(&K0G0G);
    let mask_rb = vdupq_n_u32(0x00ff_00ff); // red-blue masks
    let n = count(num_pixels);
    let mut i = 0;
    while i + 4 <= n {
        let r#in = vld1q_u8_words(&argb_data[i..]);
        // 0 g 0 g
        let greens = do_green_shuffle_neon(r#in, shuffle);
        // x dr  x db1
        let a = vqdmulhq_s16(vreinterpretq_s16_u8(greens), mults_rb);
        // r 0   b   0
        let b = vshlq_n_s16::<8>(vreinterpretq_s16_u8(r#in));
        // x db2 0   0
        let c = vqdmulhq_s16(b, mults_b2);
        // 0 0   x db2
        let d = vshrq_n_u32::<16>(vreinterpretq_u32_s16(c));
        // x dr  x  db
        let e = vaddq_s8(vreinterpretq_s8_u32(d), vreinterpretq_s8_s16(a));
        // 0 dr  0  db
        let f = vandq_u32(vreinterpretq_u32_s8(e), mask_rb);
        let out = vsubq_s8(vreinterpretq_s8_u8(r#in), vreinterpretq_s8_u32(f));
        vst1q_s8_words(&mut argb_data[i..], out);
        i += 4;
    }
    // fallthrough and finish off with plain-C
    vp8l_transform_color_c(m, &mut argb_data[i..], num_pixels - int(i));
}

/// Entry point of `TransformColor_NEON` for `VP8LTransformColor`.
#[expect(
    clippy::trivially_copy_pass_by_ref,
    reason = "the signature of the dispatch function; C passes the multipliers by pointer"
)]
pub(crate) fn transform_color(m: &VP8LMultipliers, argb_data: &mut [u32], num_pixels: i32) {
    // SAFETY: NEON is in the AArch64 baseline that this module is compiled for,
    // and upstream selects the NEON variant there unconditionally (ADR 0008).
    unsafe { transform_color_neon(m, argb_data, num_pixels) }
}

//------------------------------------------------------------------------------
// Entry point

// upstream: src/dsp/lossless_enc_neon.c VP8LEncDspInitNEON
// The pointer assignments are the dispatch functions of
// `src/dsp/lossless_enc.rs` named in the module documentation (ADR 0008).
