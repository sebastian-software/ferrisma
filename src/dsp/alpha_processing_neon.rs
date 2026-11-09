// libwebp v1.6.0, src/dsp/alpha_processing_neon.c:
//
// Copyright 2017 Google Inc. All Rights Reserved.
//
// Use of this source code is governed by a BSD-style license
// that can be found in the COPYING file in the root of the source
// tree. An additional intellectual property rights grant can be found
// in the file PATENTS. All contributing project authors may
// be found in the AUTHORS file in the root of the source tree.
// -----------------------------------------------------------------------------
//
// Utilities for processing transparent channel, NEON version.
//
// Author: Skal (pascal.massimino@gmail.com)

//! Translation of `src/dsp/alpha_processing_neon.c` from libwebp v1.6.0.
//!
//! Work package: WP20 (NEON).
//!
//! The NEON variants of the alpha dispatch, extraction and premultiplication
//! of `src/dsp/alpha_processing.c`. Each `_neon` function is the body that
//! upstream's `WebPInitAlphaProcessingNEON` assigns; the safe entry point
//! below it, named after the function without the suffix, is what the
//! dispatch function of `src/dsp/alpha_processing.rs` calls on 64-bit ARM.
//!
//! `WebPInitAlphaProcessingNEON` assigns `WebPApplyAlphaMultiply`,
//! `WebPDispatchAlpha`, `WebPDispatchAlphaToGreen`, `WebPExtractAlpha` and
//! `WebPExtractGreen`; `WebPInitAlphaProcessing` calls it unconditionally on
//! 64-bit ARM (`WEBP_NEON_OMIT_C_CODE`), so the dispatch functions of those five
//! pointers return the NEON entry points there.
//!
//! # Deviations from upstream
//!
//! 1. **Loads, stores and alignment** follow the deviations 1 to 3 of
//!    `src/dsp/neon.rs`. `uint32_t tmp[2]`, which `vst1_u8` fills with the
//!    mask, is an `[u8; 8]` read as two native-endian words.
//! 2. **Pointers and strides** are slices and offsets as in the C functions
//!    of `src/dsp/alpha_processing.rs` (its deviation 6): a row starts at
//!    `j * stride`, and the strides are not negative.
//! 3. **`MULTIPLY_BY_ALPHA`** is a function over the four channels as an
//!    array, indexed by the macro's `ALPHA` and `OTHER` arguments.
//!
//! # Excluded undefined behavior
//!
//! None.
//!
//! # Checks not applicable
//!
//! 1. **`WEBP_RESTRICT`** has no counterpart: the slices cannot alias.

use core::arch::aarch64::{
    uint8x8_t, uint8x8x4_t, uint16x8_t, vaddq_u16, vand_u8, vdup_n_u8, vdupq_n_u16, vmull_u8,
    vshrn_n_u16, vsraq_n_u16,
};

use crate::dsp::neon::{
    vld1_u8, vld4_u8, vld4q_u8_words, vst1_u8, vst1q_u8, vst4_u8, vst4_u8_words,
};

/// The low byte of a value, as a C conversion to `uint8_t` keeps it.
#[expect(
    clippy::cast_possible_truncation,
    reason = "a C conversion to uint8_t keeps the low eight bits"
)]
#[must_use]
fn low_byte(value: u32) -> u8 {
    value as u8
}

/// A non-negative C `int` as an index; negative values count as zero, as
/// the C loops over them do not run.
#[must_use]
fn count(value: i32) -> usize {
    usize::try_from(value).unwrap_or(0)
}

/// A C stride, which these functions take as not negative.
#[must_use]
fn row_stride(value: i32) -> usize {
    usize::try_from(value).expect("the stride is not negative")
}

//------------------------------------------------------------------------------

// upstream: src/dsp/alpha_processing_neon.c MULTIPLIER
#[must_use]
const fn multiplier(a: u32) -> u32 {
    a * 0x8081
}

// upstream: src/dsp/alpha_processing_neon.c PREMULTIPLY
#[must_use]
const fn premultiply(x: u32, m: u32) -> u32 {
    (x * m) >> 23
}

// upstream: src/dsp/alpha_processing_neon.c MULTIPLY_BY_ALPHA
/// Multiplies the channels 1, 2 and `other` of `v` by the channel `alpha`,
/// divided by 255 with `(v + 1 + (v >> 8)) >> 8`.
#[target_feature(enable = "neon")]
#[must_use]
fn multiply_by_alpha(v: uint8x8x4_t, alpha: usize, other: usize, k_one: uint16x8_t) -> uint8x8x4_t {
    let mut channels: [uint8x8_t; 4] = [v.0, v.1, v.2, v.3];
    let alpha = channels[alpha];
    let r1 = vmull_u8(channels[1], alpha);
    let g1 = vmull_u8(channels[2], alpha);
    let b1 = vmull_u8(channels[other], alpha);
    // we use: v / 255 = (v + 1 + (v >> 8)) >> 8
    let r2 = vsraq_n_u16::<8>(r1, r1);
    let g2 = vsraq_n_u16::<8>(g1, g1);
    let b2 = vsraq_n_u16::<8>(b1, b1);
    let r3 = vaddq_u16(r2, k_one);
    let g3 = vaddq_u16(g2, k_one);
    let b3 = vaddq_u16(b2, k_one);
    channels[1] = vshrn_n_u16::<8>(r3);
    channels[2] = vshrn_n_u16::<8>(g3);
    channels[other] = vshrn_n_u16::<8>(b3);
    uint8x8x4_t(channels[0], channels[1], channels[2], channels[3])
}

// upstream: src/dsp/alpha_processing_neon.c ApplyAlphaMultiply_NEON
#[target_feature(enable = "neon")]
#[expect(
    clippy::similar_names,
    clippy::many_single_char_names,
    reason = "the C names rgba, rgbx, w, h, i, a and x"
)]
fn apply_alpha_multiply_neon(rgba: &mut [u8], alpha_first: bool, w: i32, h: i32, stride: i32) {
    let k_one = vdupq_n_u16(1);
    let w = count(w);
    let stride = row_stride(stride);
    for row in 0..count(h) {
        let rgbx = row * stride;
        let mut i = 0;
        if alpha_first {
            while i + 8 <= w {
                // load aaaa...|rrrr...|gggg...|bbbb...
                let mut rgbx_v = vld4_u8(&rgba[rgbx + 4 * i..]);
                rgbx_v = multiply_by_alpha(rgbx_v, 0, 3, k_one);
                vst4_u8(&mut rgba[rgbx + 4 * i..], rgbx_v);
                i += 8;
            }
        } else {
            while i + 8 <= w {
                let mut rgbx_v = vld4_u8(&rgba[rgbx + 4 * i..]);
                rgbx_v = multiply_by_alpha(rgbx_v, 3, 0, k_one);
                vst4_u8(&mut rgba[rgbx + 4 * i..], rgbx_v);
                i += 8;
            }
        }
        // Finish with left-overs.
        while i < w {
            let rgb = rgbx + usize::from(alpha_first);
            let alpha = rgbx + if alpha_first { 0 } else { 3 };
            let a = u32::from(rgba[alpha + 4 * i]);
            if a != 0xff {
                let mult = multiplier(a);
                for k in 0..3 {
                    let x = u32::from(rgba[rgb + 4 * i + k]);
                    rgba[rgb + 4 * i + k] = low_byte(premultiply(x, mult));
                }
            }
            i += 1;
        }
    }
}

/// Entry point of `ApplyAlphaMultiply_NEON` for `WebPApplyAlphaMultiply`.
pub fn apply_alpha_multiply(rgba: &mut [u8], alpha_first: bool, w: i32, h: i32, stride: i32) {
    // SAFETY: NEON is in the AArch64 baseline that this module is compiled for,
    // and upstream selects the NEON variant there unconditionally (ADR 0008).
    unsafe { apply_alpha_multiply_neon(rgba, alpha_first, w, h, stride) }
}

//------------------------------------------------------------------------------

/// The C epilogue of `DispatchAlpha_NEON` and `ExtractAlpha_NEON`: whether
/// every byte of `alpha_mask` repeated and of `mask8` is 0xff.
#[target_feature(enable = "neon")]
#[must_use]
fn all_ones(alpha_mask: u32, mask8: uint8x8_t) -> bool {
    let mut tmp = [0_u8; 8];
    vst1_u8(&mut tmp, mask8);
    let mut alpha_mask = alpha_mask.wrapping_mul(0x0101_0101);
    alpha_mask &= u32::from_ne_bytes([tmp[0], tmp[1], tmp[2], tmp[3]]);
    alpha_mask &= u32::from_ne_bytes([tmp[4], tmp[5], tmp[6], tmp[7]]);
    alpha_mask == 0xffff_ffff
}

// upstream: src/dsp/alpha_processing_neon.c DispatchAlpha_NEON
#[target_feature(enable = "neon")]
fn dispatch_alpha_neon(
    alpha: &[u8],
    alpha_stride: i32,
    width: i32,
    height: i32,
    dst: &mut [u8],
    dst_stride: i32,
) -> bool {
    let mut alpha_mask = 0xff_u32;
    let mut mask8 = vdup_n_u8(0xff);
    let width = count(width);
    let alpha_stride = row_stride(alpha_stride);
    let dst_stride = row_stride(dst_stride);
    for j in 0..count(height) {
        let a = j * alpha_stride;
        let d = j * dst_stride;
        let mut i = 0;
        // We don't know if alpha is first or last in dst[] (depending on rgbA/Argb
        // mode). So we must be sure dst[4*i + 8 - 1] is writable for the store.
        // Hence the test with 'width - 1' instead of just 'width'.
        while i + 8 < width {
            let mut rgb_x = vld4_u8(&dst[d + 4 * i..]);
            let alphas = vld1_u8(&alpha[a + i..]);
            rgb_x.0 = alphas;
            vst4_u8(&mut dst[d + 4 * i..], rgb_x);
            mask8 = vand_u8(mask8, alphas);
            i += 8;
        }
        while i < width {
            let alpha_value = alpha[a + i];
            dst[d + 4 * i] = alpha_value;
            alpha_mask &= u32::from(alpha_value);
            i += 1;
        }
    }
    !all_ones(alpha_mask, mask8)
}

/// Entry point of `DispatchAlpha_NEON` for `WebPDispatchAlpha`.
pub fn dispatch_alpha(
    alpha: &[u8],
    alpha_stride: i32,
    width: i32,
    height: i32,
    dst: &mut [u8],
    dst_stride: i32,
) -> bool {
    // SAFETY: NEON is in the AArch64 baseline that this module is compiled for,
    // and upstream selects the NEON variant there unconditionally (ADR 0008).
    unsafe { dispatch_alpha_neon(alpha, alpha_stride, width, height, dst, dst_stride) }
}

// upstream: src/dsp/alpha_processing_neon.c DispatchAlphaToGreen_NEON
#[target_feature(enable = "neon")]
fn dispatch_alpha_to_green_neon(
    alpha: &[u8],
    alpha_stride: i32,
    width: i32,
    height: i32,
    dst: &mut [u32],
    dst_stride: i32,
) {
    let zero = vdup_n_u8(0);
    // leave A/R/B channels zero'd.
    let mut greens = uint8x8x4_t(zero, zero, zero, zero);
    let width = count(width);
    let alpha_stride = row_stride(alpha_stride);
    let dst_stride = row_stride(dst_stride);
    for j in 0..count(height) {
        let a = j * alpha_stride;
        let d = j * dst_stride;
        let mut i = 0;
        while i + 8 <= width {
            greens.1 = vld1_u8(&alpha[a + i..]);
            vst4_u8_words(&mut dst[d + i..], greens);
            i += 8;
        }
        while i < width {
            dst[d + i] = u32::from(alpha[a + i]) << 8;
            i += 1;
        }
    }
}

/// Entry point of `DispatchAlphaToGreen_NEON` for `WebPDispatchAlphaToGreen`.
pub fn dispatch_alpha_to_green(
    alpha: &[u8],
    alpha_stride: i32,
    width: i32,
    height: i32,
    dst: &mut [u32],
    dst_stride: i32,
) {
    // SAFETY: NEON is in the AArch64 baseline that this module is compiled for,
    // and upstream selects the NEON variant there unconditionally (ADR 0008).
    unsafe { dispatch_alpha_to_green_neon(alpha, alpha_stride, width, height, dst, dst_stride) }
}

// upstream: src/dsp/alpha_processing_neon.c ExtractAlpha_NEON
#[target_feature(enable = "neon")]
fn extract_alpha_neon(
    argb: &[u8],
    argb_stride: i32,
    width: i32,
    height: i32,
    alpha: &mut [u8],
    alpha_stride: i32,
) -> bool {
    let mut alpha_mask = 0xff_u32;
    let mut mask8 = vdup_n_u8(0xff);
    let width = count(width);
    let argb_stride = row_stride(argb_stride);
    let alpha_stride = row_stride(alpha_stride);
    for j in 0..count(height) {
        let s = j * argb_stride;
        let a = j * alpha_stride;
        let mut i = 0;
        // We don't know if alpha is first or last in dst[] (depending on rgbA/Argb
        // mode). So we must be sure dst[4*i + 8 - 1] is writable for the store.
        // Hence the test with 'width - 1' instead of just 'width'.
        while i + 8 < width {
            let rgb_x = vld4_u8(&argb[s + 4 * i..]);
            let alphas = rgb_x.0;
            vst1_u8(&mut alpha[a + i..], alphas);
            mask8 = vand_u8(mask8, alphas);
            i += 8;
        }
        while i < width {
            alpha[a + i] = argb[s + 4 * i];
            alpha_mask &= u32::from(alpha[a + i]);
            i += 1;
        }
    }
    all_ones(alpha_mask, mask8)
}

/// Entry point of `ExtractAlpha_NEON` for `WebPExtractAlpha`.
pub fn extract_alpha(
    argb: &[u8],
    argb_stride: i32,
    width: i32,
    height: i32,
    alpha: &mut [u8],
    alpha_stride: i32,
) -> bool {
    // SAFETY: NEON is in the AArch64 baseline that this module is compiled for,
    // and upstream selects the NEON variant there unconditionally (ADR 0008).
    unsafe { extract_alpha_neon(argb, argb_stride, width, height, alpha, alpha_stride) }
}

// upstream: src/dsp/alpha_processing_neon.c ExtractGreen_NEON
#[target_feature(enable = "neon")]
fn extract_green_neon(argb: &[u32], alpha: &mut [u8], size: i32) {
    let size = count(size);
    let mut i = 0;
    while i + 16 <= size {
        let rgb_x = vld4q_u8_words(&argb[i..]);
        let greens = rgb_x.1;
        vst1q_u8(&mut alpha[i..], greens);
        i += 16;
    }
    while i < size {
        alpha[i] = low_byte(argb[i] >> 8);
        i += 1;
    }
}

/// Entry point of `ExtractGreen_NEON` for `WebPExtractGreen`.
pub fn extract_green(argb: &[u32], alpha: &mut [u8], size: i32) {
    // SAFETY: NEON is in the AArch64 baseline that this module is compiled for,
    // and upstream selects the NEON variant there unconditionally (ADR 0008).
    unsafe { extract_green_neon(argb, alpha, size) }
}

//------------------------------------------------------------------------------

// upstream: src/dsp/alpha_processing_neon.c WebPInitAlphaProcessingNEON
// The pointer assignments are the dispatch functions of
// `src/dsp/alpha_processing.rs` named in the module documentation (ADR 0008).
