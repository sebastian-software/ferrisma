// libwebp v1.6.0, sharpyuv/sharpyuv_neon.c:
//
// Copyright 2022 Google Inc. All Rights Reserved.
//
// Use of this source code is governed by a BSD-style license
// that can be found in the COPYING file in the root of the source
// tree. An additional intellectual property rights grant can be found
// in the file PATENTS. All contributing project authors may
// be found in the AUTHORS file in the root of the source tree.
// -----------------------------------------------------------------------------
//
// Speed-critical functions for Sharp YUV.
//
// Author: Skal (pascal.massimino@gmail.com)

//! Translation of `sharpyuv/sharpyuv_neon.c` from libwebp v1.6.0.
//!
//! Work package: WP20 (NEON).
//!
//! The NEON variants of the `SharpYUV` kernels of `sharpyuv/sharpyuv_dsp.c`.
//! Each `_neon` function is the body that upstream's `InitSharpYuvNEON`
//! assigns; the safe entry point below it, named after the function without
//! the suffix, is what the dispatch function of `src/sharpyuv/sharpyuv_dsp.rs`
//! calls on 64-bit ARM.
//!
//! `InitSharpYuvNEON` assigns `SharpYuvUpdateY`, `SharpYuvUpdateRGB` and
//! `SharpYuvFilterRow`; `SharpYuvInitDsp` calls it unconditionally on 64-bit
//! ARM (`WEBP_NEON_OMIT_C_CODE`, without consulting `SharpYuvGetCPUInfo`), so
//! the three dispatch functions return the NEON entry points there.
//!
//! # Deviations from upstream
//!
//! 1. **Loads, stores and alignment** follow the deviations 1 to 3 of
//!    `src/dsp/neon.rs`.
//! 2. **Pointers** are slices from the first element, as in the C functions of
//!    `src/sharpyuv/sharpyuv_dsp.rs`; `A + i + 1` and `best_y + 2 * i` are
//!    offsets into them.
//!
//! # Excluded undefined behavior
//!
//! None.
//!
//! # Checks not applicable
//!
//! 1. **Parity outside the converter's range.** The kernels reinterpret the
//!    16-bit samples as signed lanes and add them in 16 bits (32 bits in
//!    `SharpYuvFilterRow32_NEON`). `SharpYuvConvert` calls them with a bit
//!    depth of at most `kMaxBitDepth`, 14, luma samples below `1 << 14` and
//!    chroma differences of the same range, and uses the 16-bit filter only up
//!    to a bit depth of 10, where no lane overflows; there the kernels equal
//!    the C functions, which `tests/wp20_sharpyuv.rs` checks for the bit depths
//!    8 to 14. Larger values, which the converter never produces, can wrap in
//!    a lane where C computes in `int`: `tests/wp5_dsp.rs` draws chroma from
//!    `-16384..16384` also at the bit depth 10, where
//!    `SharpYuvFilterRow16_NEON` overflows its 16-bit sums; against the
//!    `release-simd` oracle the Rust kernel gave upstream's NEON bytes for
//!    them, and since the dispatch selects the kernel, that test verifies the
//!    C translation directly (the pattern decided on issue #28).

use core::arch::aarch64::{
    vabsq_s16, vaddl_s16, vaddq_s16, vaddq_s32, vaddw_s16, vcombine_u16, vdupq_n_s16, vdupq_n_u16,
    vdupq_n_u64, vget_high_s16, vget_low_s16, vgetq_lane_u64, vmaxq_s16, vminq_s16, vminq_u16,
    vmovl_s16, vpadalq_u32, vpaddlq_u16, vqmovun_s32, vreinterpretq_s16_u16, vreinterpretq_u16_s16,
    vrhaddq_s16, vrhaddq_s32, vshrq_n_s16, vshrq_n_s32, vsubq_s16, vzipq_s16, vzipq_s32,
};

use crate::dsp::neon::{vld1_s16, vld1q_s16, vld1q_u16, vst1q_s16, vst1q_u16};

// upstream: sharpyuv/sharpyuv_neon.c clip_NEON
#[expect(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    reason = "the C conversions to uint16_t of a value in range"
)]
#[must_use]
fn clip_neon(v: i32, max: i32) -> u16 {
    if v < 0 {
        0
    } else if v > max {
        max as u16
    } else {
        v as u16
    }
}

/// A C length, which the kernels take as not negative.
#[must_use]
fn length(len: i32) -> usize {
    usize::try_from(len).unwrap_or(0)
}

/// `(1 << bit_depth) - 1` as the 16-bit lane value of `vdupq_n_s16(max_y)`.
#[expect(
    clippy::cast_possible_truncation,
    reason = "the C conversion of max_y to the int16_t lane"
)]
#[must_use]
fn lane(max_y: i32) -> i16 {
    max_y as i16
}

// upstream: sharpyuv/sharpyuv_neon.c SharpYuvUpdateY_NEON
#[target_feature(enable = "neon")]
#[expect(
    clippy::many_single_char_names,
    reason = "the short C names of the source"
)]
fn sharp_yuv_update_y_neon(
    r#ref: &[u16],
    src: &[u16],
    dst: &mut [u16],
    len: i32,
    bit_depth: i32,
) -> u64 {
    let max_y = (1 << bit_depth) - 1;
    let zero = vdupq_n_s16(0);
    let max = vdupq_n_s16(lane(max_y));
    let mut sum = vdupq_n_u64(0);
    let len = length(len);

    let mut i = 0;
    while i + 8 <= len {
        let a = vreinterpretq_s16_u16(vld1q_u16(&r#ref[i..]));
        let b = vreinterpretq_s16_u16(vld1q_u16(&src[i..]));
        let c = vreinterpretq_s16_u16(vld1q_u16(&dst[i..]));
        let d = vsubq_s16(a, b); // diff_y
        let f = vaddq_s16(c, d); // new_y
        let h = vreinterpretq_u16_s16(vmaxq_s16(vminq_s16(f, max), zero));
        let i_ = vabsq_s16(d); // abs(diff_y)
        vst1q_u16(&mut dst[i..], h);
        sum = vpadalq_u32(sum, vpaddlq_u16(vreinterpretq_u16_s16(i_)));
        i += 8;
    }
    let mut diff = vgetq_lane_u64::<0>(sum).wrapping_add(vgetq_lane_u64::<1>(sum));
    while i < len {
        let diff_y = i32::from(r#ref[i]) - i32::from(src[i]);
        let new_y = i32::from(dst[i]) + diff_y;
        dst[i] = clip_neon(new_y, max_y);
        diff = diff.wrapping_add(u64::from(diff_y.unsigned_abs()));
        i += 1;
    }
    diff
}

/// Entry point of `SharpYuvUpdateY_NEON` for `SharpYuvUpdateY`.
pub fn sharp_yuv_update_y(
    r#ref: &[u16],
    src: &[u16],
    dst: &mut [u16],
    len: i32,
    bit_depth: i32,
) -> u64 {
    // SAFETY: NEON is in the AArch64 baseline that this module is compiled for,
    // and upstream selects the NEON variant there unconditionally (ADR 0008).
    unsafe { sharp_yuv_update_y_neon(r#ref, src, dst, len, bit_depth) }
}

// upstream: sharpyuv/sharpyuv_neon.c SharpYuvUpdateRGB_NEON
#[target_feature(enable = "neon")]
#[expect(
    clippy::cast_possible_truncation,
    reason = "the C assignment of an int to the int16_t element"
)]
#[expect(
    clippy::many_single_char_names,
    reason = "the short C names of the source"
)]
fn sharp_yuv_update_rgb_neon(r#ref: &[i16], src: &[i16], dst: &mut [i16], len: i32) {
    let len = length(len);
    let mut i = 0;
    while i + 8 <= len {
        let a = vld1q_s16(&r#ref[i..]);
        let b = vld1q_s16(&src[i..]);
        let c = vld1q_s16(&dst[i..]);
        let d = vsubq_s16(a, b); // diff_uv
        let e = vaddq_s16(c, d); // new_uv
        vst1q_s16(&mut dst[i..], e);
        i += 8;
    }
    while i < len {
        let diff_uv = i32::from(r#ref[i]) - i32::from(src[i]);
        dst[i] = (i32::from(dst[i]) + diff_uv) as i16;
        i += 1;
    }
}

/// Entry point of `SharpYuvUpdateRGB_NEON` for `SharpYuvUpdateRGB`.
pub fn sharp_yuv_update_rgb(r#ref: &[i16], src: &[i16], dst: &mut [i16], len: i32) {
    // SAFETY: NEON is in the AArch64 baseline that this module is compiled for,
    // and upstream selects the NEON variant there unconditionally (ADR 0008).
    unsafe { sharp_yuv_update_rgb_neon(r#ref, src, dst, len) }
}

/// The scalar tail of `SharpYuvFilterRow16_NEON` and
/// `SharpYuvFilterRow32_NEON` for the pair `i`, which upstream repeats in
/// both functions.
fn filter_row_tail(a: &[i16], b: &[i16], i: usize, best_y: &[u16], out: &mut [u16], max_y: i32) {
    let a0b1 = i32::from(a[i]) + i32::from(b[i + 1]);
    let a1b0 = i32::from(a[i + 1]) + i32::from(b[i]);
    let a0a1b0b1 = a0b1 + a1b0 + 8;
    let v0 = (8 * i32::from(a[i]) + 2 * a1b0 + a0a1b0b1) >> 4;
    let v1 = (8 * i32::from(a[i + 1]) + 2 * a0b1 + a0a1b0b1) >> 4;
    out[2 * i] = clip_neon(i32::from(best_y[2 * i]) + v0, max_y);
    out[2 * i + 1] = clip_neon(i32::from(best_y[2 * i + 1]) + v1, max_y);
}

// upstream: sharpyuv/sharpyuv_neon.c SharpYuvFilterRow16_NEON
#[target_feature(enable = "neon")]
fn sharp_yuv_filter_row16_neon(
    a: &[i16],
    b: &[i16],
    len: i32,
    best_y: &[u16],
    out: &mut [u16],
    bit_depth: i32,
) {
    let max_y = (1 << bit_depth) - 1;
    let max = vdupq_n_s16(lane(max_y));
    let zero = vdupq_n_s16(0);
    let len = length(len);
    let mut i = 0;
    while i + 8 <= len {
        let a0 = vld1q_s16(&a[i..]);
        let a1 = vld1q_s16(&a[i + 1..]);
        let b0 = vld1q_s16(&b[i..]);
        let b1 = vld1q_s16(&b[i + 1..]);
        let a0b1 = vaddq_s16(a0, b1);
        let a1b0 = vaddq_s16(a1, b0);
        let a0a1b0b1 = vaddq_s16(a0b1, a1b0); // A0+A1+B0+B1
        let a0b1_2 = vaddq_s16(a0b1, a0b1); // 2*(A0+B1)
        let a1b0_2 = vaddq_s16(a1b0, a1b0); // 2*(A1+B0)
        let c0 = vshrq_n_s16::<3>(vaddq_s16(a0b1_2, a0a1b0b1));
        let c1 = vshrq_n_s16::<3>(vaddq_s16(a1b0_2, a0a1b0b1));
        let e0 = vrhaddq_s16(c1, a0);
        let e1 = vrhaddq_s16(c0, a1);
        let f = vzipq_s16(e0, e1);
        let g0 = vreinterpretq_s16_u16(vld1q_u16(&best_y[2 * i..]));
        let g1 = vreinterpretq_s16_u16(vld1q_u16(&best_y[2 * i + 8..]));
        let h0 = vaddq_s16(g0, f.0);
        let h1 = vaddq_s16(g1, f.1);
        let i0 = vmaxq_s16(vminq_s16(h0, max), zero);
        let i1 = vmaxq_s16(vminq_s16(h1, max), zero);
        vst1q_u16(&mut out[2 * i..], vreinterpretq_u16_s16(i0));
        vst1q_u16(&mut out[2 * i + 8..], vreinterpretq_u16_s16(i1));
        i += 8;
    }
    while i < len {
        filter_row_tail(a, b, i, best_y, out, max_y);
        i += 1;
    }
}

// upstream: sharpyuv/sharpyuv_neon.c SharpYuvFilterRow32_NEON
#[target_feature(enable = "neon")]
#[expect(
    clippy::cast_sign_loss,
    clippy::cast_possible_truncation,
    reason = "the C conversion of max_y to the uint16_t lane"
)]
#[expect(
    clippy::many_single_char_names,
    reason = "the short C names of the source"
)]
fn sharp_yuv_filter_row32_neon(
    a: &[i16],
    b: &[i16],
    len: i32,
    best_y: &[u16],
    out: &mut [u16],
    bit_depth: i32,
) {
    let max_y = (1 << bit_depth) - 1;
    let max = vdupq_n_u16(max_y as u16);
    let len = length(len);
    let mut i = 0;
    while i + 4 <= len {
        let a0 = vld1_s16(&a[i..]);
        let a1 = vld1_s16(&a[i + 1..]);
        let b0 = vld1_s16(&b[i..]);
        let b1 = vld1_s16(&b[i + 1..]);
        let a0b1 = vaddl_s16(a0, b1);
        let a1b0 = vaddl_s16(a1, b0);
        let a0a1b0b1 = vaddq_s32(a0b1, a1b0); // A0+A1+B0+B1
        let a0b1_2 = vaddq_s32(a0b1, a0b1); // 2*(A0+B1)
        let a1b0_2 = vaddq_s32(a1b0, a1b0); // 2*(A1+B0)
        let c0 = vshrq_n_s32::<3>(vaddq_s32(a0b1_2, a0a1b0b1));
        let c1 = vshrq_n_s32::<3>(vaddq_s32(a1b0_2, a0a1b0b1));
        let e0 = vrhaddq_s32(c1, vmovl_s16(a0));
        let e1 = vrhaddq_s32(c0, vmovl_s16(a1));
        let f = vzipq_s32(e0, e1);

        let g = vreinterpretq_s16_u16(vld1q_u16(&best_y[2 * i..]));
        let h0 = vaddw_s16(f.0, vget_low_s16(g));
        let h1 = vaddw_s16(f.1, vget_high_s16(g));
        let i_16 = vcombine_u16(vqmovun_s32(h0), vqmovun_s32(h1));
        let i_clamped = vminq_u16(i_16, max);
        vst1q_u16(&mut out[2 * i..], i_clamped);
        i += 4;
    }
    while i < len {
        filter_row_tail(a, b, i, best_y, out, max_y);
        i += 1;
    }
}

// upstream: sharpyuv/sharpyuv_neon.c SharpYuvFilterRow_NEON
#[target_feature(enable = "neon")]
fn sharp_yuv_filter_row_neon(
    a: &[i16],
    b: &[i16],
    len: i32,
    best_y: &[u16],
    out: &mut [u16],
    bit_depth: i32,
) {
    if bit_depth <= 10 {
        sharp_yuv_filter_row16_neon(a, b, len, best_y, out, bit_depth);
    } else {
        sharp_yuv_filter_row32_neon(a, b, len, best_y, out, bit_depth);
    }
}

/// Entry point of `SharpYuvFilterRow_NEON` for `SharpYuvFilterRow`.
pub fn sharp_yuv_filter_row(
    a: &[i16],
    b: &[i16],
    len: i32,
    best_y: &[u16],
    out: &mut [u16],
    bit_depth: i32,
) {
    // SAFETY: NEON is in the AArch64 baseline that this module is compiled for,
    // and upstream selects the NEON variant there unconditionally (ADR 0008).
    unsafe { sharp_yuv_filter_row_neon(a, b, len, best_y, out, bit_depth) }
}

//------------------------------------------------------------------------------

// upstream: sharpyuv/sharpyuv_neon.c InitSharpYuvNEON
// The pointer assignments are the dispatch functions of
// `src/sharpyuv/sharpyuv_dsp.rs` named in the module documentation (ADR 0008).
