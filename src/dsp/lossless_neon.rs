// libwebp v1.6.0, src/dsp/lossless_neon.c:
//
// Copyright 2014 Google Inc. All Rights Reserved.
//
// Use of this source code is governed by a BSD-style license
// that can be found in the COPYING file in the root of the source
// tree. An additional intellectual property rights grant can be found
// in the file PATENTS. All contributing project authors may
// be found in the AUTHORS file in the root of the source tree.
// -----------------------------------------------------------------------------
//
// NEON variant of methods for lossless decoder
//
// Author: Skal (pascal.massimino@gmail.com)

//! Translation of `src/dsp/lossless_neon.c` from libwebp v1.6.0.
//!
//! Work package: WP20 (NEON).
//!
//! The NEON variants of the lossless decoder's predictors, inverse
//! transforms and color-space converters of `src/dsp/lossless.c`. Each
//! `_neon` function is the body that upstream's `VP8LDspInitNEON` assigns;
//! the safe entry point below it, named after the function without the
//! suffix, is what the dispatch functions of `src/dsp/lossless.rs` call on
//! 64-bit ARM.
//!
//! `VP8LDspInitNEON` assigns `VP8LPredictors[5]`, `[6]`, `[7]` and `[13]`,
//! `VP8LPredictorsAdd[0]` to `[13]` (entries 14 and 15 keep the C padding
//! sentinels), `VP8LConvertBGRAToRGBA`, `VP8LConvertBGRAToBGR`,
//! `VP8LConvertBGRAToRGB`, `VP8LAddGreenToBlueAndRed` and
//! `VP8LTransformColorInverse`. It leaves `VP8LPredictorsAdd_C` with the C
//! functions, which the NEON functions call for their left-over pixels.
//! `VP8LDspInit` calls it unconditionally on 64-bit ARM
//! (`WEBP_NEON_OMIT_C_CODE`), where it also skips its own assignments of the
//! last five pointers, so the dispatch functions `vp8l_predictors` (for the
//! four modes), `vp8l_predictors_add` (for modes 0 to 13),
//! `vp8l_convert_bgra_to_rgba`, `vp8l_convert_bgra_to_bgr`,
//! `vp8l_convert_bgra_to_rgb`, `vp8l_add_green_to_blue_and_red` and
//! `vp8l_transform_color_inverse` return the NEON entry points there.
//!
//! # Deviations from upstream
//!
//! 1. **Loads, stores and alignment** follow the deviations 1 to 3 of
//!    `src/dsp/neon.rs`. A load or store of the bytes of `uint32_t` pixels
//!    (`vld4q_u8((uint8_t*)src)`, `vld1q_u8((const uint8_t*)src)`) uses the
//!    `_words` helpers there, which read the same bytes.
//! 2. **Pixel buffers.** The predictors and the inverse transforms take the
//!    one pixel buffer `data` with the C pointers `in`, `upper`, `out` (and
//!    `src`, `dst`) as positions in it, the form of `src/dsp/lossless.rs`
//!    (ADR 0003): every read and write happens at the index and in the order
//!    of the C code, whatever the overlap. The one-pixel predictors take
//!    `left` and `top` as a slice and a position each.
//! 3. **Lane stores of one pixel.** `vst1q_lane_u32(&out[i + LANE], v, LANE)`
//!    and `vst1_lane_u32(&out[i + LANE], v, LANE & 1)` store a 32-bit lane
//!    into a `uint32_t` element; the translation assigns
//!    `vgetq_lane_u32`/`vget_lane_u32` of the same lane to that element, which
//!    writes the same value.
//! 4. **Macros.** The function-like macros `LOAD_U32_AS_U8`,
//!    `LOADQ_U32_AS_U8`, `LOADQ_U32P_AS_U8`, `GET_U8_AS_U32`,
//!    `STOREQ_U8_AS_U32P` and `ROTATE32_LEFT` are functions. The statement
//!    macros `DO_PRED5`, `DO_PRED67`, `DO_PRED10`, `DO_PRED11`, `DO_PRED12`
//!    and `DO_PRED13`, which read and assign the locals of their function,
//!    are `macro_rules!` macros defined where those locals are in scope, and
//!    `GENERATE_PREDICTOR_1` and `GENERATE_PREDICTOR_2` are `macro_rules!`
//!    macros that define the functions, with the position of their `IN`
//!    argument written as an expression of the position `upper` and the
//!    index `i`.
//! 5. **Counts.** The loop counters are `usize` indices; the C `int`
//!    arithmetic of the left-over counts (`num_pixels - i`,
//!    `num_pixels & 15`) is kept in `i32`.
//!
//! # Excluded undefined behavior
//!
//! 1. **Negative pixel counts in the converters and in
//!    `AddGreenToBlueAndRed_NEON`.** For `num_pixels < 0`, C forms
//!    `src + (num_pixels & ~15)` (or `& ~3`), a pointer before the start of
//!    the buffer, and then converts `num_pixels & 15` (or `& 3`) pixels with
//!    the C function, where the C function alone converts none. The library
//!    never passes a negative count (the rows of `VP8LConvertFromBGRA` and
//!    the transforms have a positive width); the translation runs no vector
//!    iteration and calls the C function with the same masked count, and the
//!    parity tests compare non-negative counts only.
//!
//! # Checks not applicable
//!
//! 1. **`WORK_AROUND_GCC`** is not defined on 64-bit ARM, so the
//!    `vtbl`-based converters of its `#else` branch and their tables
//!    `kRGBAShuffle`, `kBGRShuffle` and `kRGBShuffle` are not compiled and
//!    not translated (`src/dsp/neon.rs`, checks not applicable 2).
//! 2. **`USE_VTBLQ`** is defined only for Apple clang older than Xcode 6.3
//!    (`__apple_build_version__ < 6020037`), which cannot build this crate's
//!    oracle; the translation takes the `!USE_VTBLQ` branch, the 8-byte
//!    `kGreenShuffle` and `k0g0g` tables with `vtbl1_u8`, on macOS and Linux
//!    alike. The `USE_VTBLQ` branch (`vtbl1q_u8` with the 16-byte tables,
//!    including `kg0g0`) selects the same bytes and is not translated.
//! 3. **`LOAD_U32P_AS_U8` and `GETQ_U8_AS_U32`** are defined and undefined
//!    without a use in the file; a translation would be dead code.
//! 4. **`WEBP_RESTRICT`** has no counterpart: the predictors' one buffer is
//!    the representation of `src/dsp/lossless.rs`, and the converters take
//!    distinct slices.

use core::arch::aarch64::{
    uint8x8_t, uint8x16_t, uint8x16x3_t, uint16x8_t, vabdq_u8, vadd_u8, vaddq_s8, vaddq_s16,
    vaddq_u8, vaddw_s8, vandq_u32, vbslq_u8, vcgt_u8, vcgtq_u8, vcleq_u32, vcombine_u8, vdup_n_u32,
    vdupq_n_u32, vextq_u8, vextq_u16, vget_high_u8, vget_lane_u32, vget_low_u8, vgetq_lane_u32,
    vhadd_u8, vhaddq_u8, vhsub_u8, vhsubq_u8, vmovl_u8, vorrq_u32, vpaddlq_u8, vpaddlq_u16,
    vqdmulhq_s16, vqmovun_s16, vreinterpret_s8_u8, vreinterpret_u8_u32, vreinterpret_u32_u8,
    vreinterpretq_s8_s16, vreinterpretq_s8_u8, vreinterpretq_s8_u32, vreinterpretq_s16_s8,
    vreinterpretq_s16_u8, vreinterpretq_s16_u16, vreinterpretq_u8_u32, vreinterpretq_u16_s8,
    vreinterpretq_u32_s16, vreinterpretq_u32_u8, vreinterpretq_u32_u16, vshlq_n_s16, vshrq_n_u16,
    vshrq_n_u32, vsubl_u8, vtbl1_u8,
};

use crate::dsp::lossless::{
    VP8LMultipliers, vp8l_add_green_to_blue_and_red_c, vp8l_convert_bgra_to_bgr_c,
    vp8l_convert_bgra_to_rgb_c, vp8l_convert_bgra_to_rgba_c, vp8l_predictors_add_c,
    vp8l_transform_color_inverse_c,
};
use crate::dsp::neon::{
    vld1_u8, vld1q_s16, vld1q_u8_words, vld1q_u32, vld4q_u8_words, vst1q_u8_words, vst1q_u32,
    vst3q_u8, vst4q_u8,
};
use crate::webp::format_constants::ARGB_BLACK;

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
// Colorspace conversion functions

// upstream: src/dsp/lossless_neon.c ConvertBGRAToRGBA_NEON
// gcc 4.6.0 had some trouble (NDK-r9) with this code. We only use it for
// gcc-4.8.x at least.
#[target_feature(enable = "neon")]
#[expect(clippy::manual_swap, reason = "the C swap of val[0] and val[2]")]
fn convert_bgra_to_rgba_neon(src: &[u32], num_pixels: i32, dst: &mut [u8]) {
    let end = count(num_pixels & !15);
    let mut s = 0;
    let mut d = 0;
    while s < end {
        let mut pixel = vld4q_u8_words(&src[s..]);
        // swap B and R. (VSWP d0,d2 has no intrinsics equivalent!)
        let tmp = pixel.0;
        pixel.0 = pixel.2;
        pixel.2 = tmp;
        vst4q_u8(&mut dst[d..], pixel);
        d += 64;
        s += 16;
    }
    vp8l_convert_bgra_to_rgba_c(&src[s..], num_pixels & 15, &mut dst[d..]); // left-overs
}

/// Entry point of `ConvertBGRAToRGBA_NEON` for `VP8LConvertBGRAToRGBA`.
pub fn convert_bgra_to_rgba(src: &[u32], num_pixels: i32, dst: &mut [u8]) {
    // SAFETY: NEON is in the AArch64 baseline that this module is compiled for,
    // and upstream selects the NEON variant there unconditionally (ADR 0008).
    unsafe { convert_bgra_to_rgba_neon(src, num_pixels, dst) }
}

// upstream: src/dsp/lossless_neon.c ConvertBGRAToBGR_NEON
#[target_feature(enable = "neon")]
fn convert_bgra_to_bgr_neon(src: &[u32], num_pixels: i32, dst: &mut [u8]) {
    let end = count(num_pixels & !15);
    let mut s = 0;
    let mut d = 0;
    while s < end {
        let pixel = vld4q_u8_words(&src[s..]);
        let tmp = uint8x16x3_t(pixel.0, pixel.1, pixel.2);
        vst3q_u8(&mut dst[d..], tmp);
        d += 48;
        s += 16;
    }
    vp8l_convert_bgra_to_bgr_c(&src[s..], num_pixels & 15, &mut dst[d..]); // left-overs
}

/// Entry point of `ConvertBGRAToBGR_NEON` for `VP8LConvertBGRAToBGR`.
pub fn convert_bgra_to_bgr(src: &[u32], num_pixels: i32, dst: &mut [u8]) {
    // SAFETY: NEON is in the AArch64 baseline that this module is compiled for,
    // and upstream selects the NEON variant there unconditionally (ADR 0008).
    unsafe { convert_bgra_to_bgr_neon(src, num_pixels, dst) }
}

// upstream: src/dsp/lossless_neon.c ConvertBGRAToRGB_NEON
#[target_feature(enable = "neon")]
fn convert_bgra_to_rgb_neon(src: &[u32], num_pixels: i32, dst: &mut [u8]) {
    let end = count(num_pixels & !15);
    let mut s = 0;
    let mut d = 0;
    while s < end {
        let pixel = vld4q_u8_words(&src[s..]);
        let tmp = uint8x16x3_t(pixel.2, pixel.1, pixel.0);
        vst3q_u8(&mut dst[d..], tmp);
        d += 48;
        s += 16;
    }
    vp8l_convert_bgra_to_rgb_c(&src[s..], num_pixels & 15, &mut dst[d..]); // left-overs
}

/// Entry point of `ConvertBGRAToRGB_NEON` for `VP8LConvertBGRAToRGB`.
pub fn convert_bgra_to_rgb(src: &[u32], num_pixels: i32, dst: &mut [u8]) {
    // SAFETY: NEON is in the AArch64 baseline that this module is compiled for,
    // and upstream selects the NEON variant there unconditionally (ADR 0008).
    unsafe { convert_bgra_to_rgb_neon(src, num_pixels, dst) }
}

//------------------------------------------------------------------------------
// Predictor Transform

// upstream: src/dsp/lossless_neon.c LOAD_U32_AS_U8
#[target_feature(enable = "neon")]
#[must_use]
fn load_u32_as_u8(value: u32) -> uint8x8_t {
    vreinterpret_u8_u32(vdup_n_u32(value))
}

// upstream: src/dsp/lossless_neon.c LOADQ_U32_AS_U8
#[target_feature(enable = "neon")]
#[must_use]
fn loadq_u32_as_u8(value: u32) -> uint8x16_t {
    vreinterpretq_u8_u32(vdupq_n_u32(value))
}

// upstream: src/dsp/lossless_neon.c LOADQ_U32P_AS_U8
#[target_feature(enable = "neon")]
#[must_use]
fn loadq_u32p_as_u8(src: &[u32]) -> uint8x16_t {
    vreinterpretq_u8_u32(vld1q_u32(src))
}

// upstream: src/dsp/lossless_neon.c GET_U8_AS_U32
#[target_feature(enable = "neon")]
#[must_use]
fn get_u8_as_u32(value: uint8x8_t) -> u32 {
    vget_lane_u32::<0>(vreinterpret_u32_u8(value))
}

// upstream: src/dsp/lossless_neon.c STOREQ_U8_AS_U32P
#[target_feature(enable = "neon")]
fn storeq_u8_as_u32p(out: &mut [u32], value: uint8x16_t) {
    vst1q_u32(out, vreinterpretq_u32_u8(value));
}

// upstream: src/dsp/lossless_neon.c ROTATE32_LEFT
/// D|C|B|A -> C|B|A|D
#[target_feature(enable = "neon")]
#[must_use]
fn rotate32_left(l: uint8x16_t) -> uint8x16_t {
    vextq_u8::<12>(l, l)
}

// upstream: src/dsp/lossless_neon.c Average2_u8_NEON
#[target_feature(enable = "neon")]
#[must_use]
fn average2_u8_neon(a0: u32, a1: u32) -> uint8x8_t {
    let a0 = load_u32_as_u8(a0);
    let a1 = load_u32_as_u8(a1);
    vhadd_u8(a0, a1)
}

// upstream: src/dsp/lossless_neon.c ClampedAddSubtractHalf_NEON
#[target_feature(enable = "neon")]
#[must_use]
fn clamped_add_subtract_half_neon(c0: u32, c1: u32, c2: u32) -> u32 {
    let avg = average2_u8_neon(c0, c1);
    // Remove one to c2 when bigger than avg.
    let c2 = load_u32_as_u8(c2);
    let cmp = vcgt_u8(c2, avg);
    let c2_1 = vadd_u8(c2, cmp);
    // Compute half of the difference between avg and c2.
    let diff_avg = vreinterpret_s8_u8(vhsub_u8(avg, c2_1));
    // Compute the sum with avg and saturate.
    let avg_16 = vreinterpretq_s16_u16(vmovl_u8(avg));
    let res = vqmovun_s16(vaddw_s8(avg_16, diff_avg));
    get_u8_as_u32(res)
}

// upstream: src/dsp/lossless_neon.c Average2_NEON
#[target_feature(enable = "neon")]
#[must_use]
fn average2_neon(a0: u32, a1: u32) -> u32 {
    let avg_u8x8 = average2_u8_neon(a0, a1);
    get_u8_as_u32(avg_u8x8)
}

// upstream: src/dsp/lossless_neon.c Average3_NEON
#[target_feature(enable = "neon")]
#[must_use]
fn average3_neon(a0: u32, a1: u32, a2: u32) -> u32 {
    let avg0 = average2_u8_neon(a0, a2);
    let a1 = load_u32_as_u8(a1);
    get_u8_as_u32(vhadd_u8(avg0, a1))
}

// upstream: src/dsp/lossless_neon.c Predictor5_NEON
#[target_feature(enable = "neon")]
#[must_use]
fn predictor5_neon(left: &[u32], left_pos: usize, top: &[u32], top_pos: usize) -> u32 {
    average3_neon(left[left_pos], top[top_pos], top[top_pos + 1])
}

/// Entry point of `Predictor5_NEON` for `VP8LPredictors[5]`.
#[must_use]
pub fn predictor5(left: &[u32], left_pos: usize, top: &[u32], top_pos: usize) -> u32 {
    // SAFETY: NEON is in the AArch64 baseline that this module is compiled for,
    // and upstream selects the NEON variant there unconditionally (ADR 0008).
    unsafe { predictor5_neon(left, left_pos, top, top_pos) }
}

// upstream: src/dsp/lossless_neon.c Predictor6_NEON
#[target_feature(enable = "neon")]
#[must_use]
fn predictor6_neon(left: &[u32], left_pos: usize, top: &[u32], top_pos: usize) -> u32 {
    average2_neon(left[left_pos], top[top_pos - 1])
}

/// Entry point of `Predictor6_NEON` for `VP8LPredictors[6]`.
#[must_use]
pub fn predictor6(left: &[u32], left_pos: usize, top: &[u32], top_pos: usize) -> u32 {
    // SAFETY: NEON is in the AArch64 baseline that this module is compiled for,
    // and upstream selects the NEON variant there unconditionally (ADR 0008).
    unsafe { predictor6_neon(left, left_pos, top, top_pos) }
}

// upstream: src/dsp/lossless_neon.c Predictor7_NEON
#[target_feature(enable = "neon")]
#[must_use]
fn predictor7_neon(left: &[u32], left_pos: usize, top: &[u32], top_pos: usize) -> u32 {
    average2_neon(left[left_pos], top[top_pos])
}

/// Entry point of `Predictor7_NEON` for `VP8LPredictors[7]`.
#[must_use]
pub fn predictor7(left: &[u32], left_pos: usize, top: &[u32], top_pos: usize) -> u32 {
    // SAFETY: NEON is in the AArch64 baseline that this module is compiled for,
    // and upstream selects the NEON variant there unconditionally (ADR 0008).
    unsafe { predictor7_neon(left, left_pos, top, top_pos) }
}

// upstream: src/dsp/lossless_neon.c Predictor13_NEON
#[target_feature(enable = "neon")]
#[must_use]
fn predictor13_neon(left: &[u32], left_pos: usize, top: &[u32], top_pos: usize) -> u32 {
    clamped_add_subtract_half_neon(left[left_pos], top[top_pos], top[top_pos - 1])
}

/// Entry point of `Predictor13_NEON` for `VP8LPredictors[13]`.
#[must_use]
pub fn predictor13(left: &[u32], left_pos: usize, top: &[u32], top_pos: usize) -> u32 {
    // SAFETY: NEON is in the AArch64 baseline that this module is compiled for,
    // and upstream selects the NEON variant there unconditionally (ADR 0008).
    unsafe { predictor13_neon(left, left_pos, top, top_pos) }
}

// Batch versions of those functions.

// upstream: src/dsp/lossless_neon.c PredictorAdd0_NEON
/// Predictor0: `ARGB_BLACK`.
#[target_feature(enable = "neon")]
fn predictor_add0_neon(
    data: &mut [u32],
    in_pos: usize,
    upper_pos: usize,
    num_pixels: i32,
    out_pos: usize,
) {
    let n = count(num_pixels);
    let black = vreinterpretq_u8_u32(vdupq_n_u32(ARGB_BLACK));
    let mut i = 0;
    while i + 4 <= n {
        let src = loadq_u32p_as_u8(&data[in_pos + i..]);
        let res = vaddq_u8(src, black);
        storeq_u8_as_u32p(&mut data[out_pos + i..], res);
        i += 4;
    }
    vp8l_predictors_add_c(
        0,
        data,
        in_pos + i,
        upper_pos + i,
        num_pixels - int(i),
        out_pos + i,
    );
}

/// Entry point of `PredictorAdd0_NEON` for `VP8LPredictorsAdd[0]`.
pub fn predictor_add0(
    data: &mut [u32],
    in_pos: usize,
    upper_pos: usize,
    num_pixels: i32,
    out_pos: usize,
) {
    // SAFETY: NEON is in the AArch64 baseline that this module is compiled for,
    // and upstream selects the NEON variant there unconditionally (ADR 0008).
    unsafe { predictor_add0_neon(data, in_pos, upper_pos, num_pixels, out_pos) }
}

// upstream: src/dsp/lossless_neon.c PredictorAdd1_NEON
/// Predictor1: left.
#[target_feature(enable = "neon")]
fn predictor_add1_neon(
    data: &mut [u32],
    in_pos: usize,
    upper_pos: usize,
    num_pixels: i32,
    out_pos: usize,
) {
    let n = count(num_pixels);
    let zero = loadq_u32_as_u8(0);
    let mut i = 0;
    while i + 4 <= n {
        // a | b | c | d
        let src = loadq_u32p_as_u8(&data[in_pos + i..]);
        // 0 | a | b | c
        let shift0 = vextq_u8::<12>(zero, src);
        // a | a + b | b + c | c + d
        let sum0 = vaddq_u8(src, shift0);
        // 0 | 0 | a | a + b
        let shift1 = vextq_u8::<8>(zero, sum0);
        // a | a + b | a + b + c | a + b + c + d
        let sum1 = vaddq_u8(sum0, shift1);
        let prev = loadq_u32_as_u8(data[out_pos + i - 1]);
        let res = vaddq_u8(sum1, prev);
        storeq_u8_as_u32p(&mut data[out_pos + i..], res);
        i += 4;
    }
    vp8l_predictors_add_c(
        1,
        data,
        in_pos + i,
        upper_pos + i,
        num_pixels - int(i),
        out_pos + i,
    );
}

/// Entry point of `PredictorAdd1_NEON` for `VP8LPredictorsAdd[1]`.
pub fn predictor_add1(
    data: &mut [u32],
    in_pos: usize,
    upper_pos: usize,
    num_pixels: i32,
    out_pos: usize,
) {
    // SAFETY: NEON is in the AArch64 baseline that this module is compiled for,
    // and upstream selects the NEON variant there unconditionally (ADR 0008).
    unsafe { predictor_add1_neon(data, in_pos, upper_pos, num_pixels, out_pos) }
}

// upstream: src/dsp/lossless_neon.c GENERATE_PREDICTOR_1
/// Macro that adds 32-bit integers from IN using mod 256 arithmetic
/// per 8 bit channel. `$upper` and `$i` name the position of `upper` and the
/// index in the expression `$other`, the position of the C `IN`.
macro_rules! generate_predictor_1 {
    ($name:ident, $x:literal, $upper:ident, $i:ident => $other:expr) => {
        #[target_feature(enable = "neon")]
        fn $name(data: &mut [u32], in_pos: usize, $upper: usize, num_pixels: i32, out_pos: usize) {
            let n = count(num_pixels);
            let mut $i = 0;
            while $i + 4 <= n {
                let src = loadq_u32p_as_u8(&data[in_pos + $i..]);
                let other = loadq_u32p_as_u8(&data[$other..]);
                let res = vaddq_u8(src, other);
                storeq_u8_as_u32p(&mut data[out_pos + $i..], res);
                $i += 4;
            }
            vp8l_predictors_add_c(
                $x,
                data,
                in_pos + $i,
                $upper + $i,
                num_pixels - int($i),
                out_pos + $i,
            );
        }
    };
}
// upstream: src/dsp/lossless_neon.c PredictorAdd2_NEON
// Predictor2: Top.
generate_predictor_1!(predictor_add2_neon, 2, upper, i => upper + i);
// upstream: src/dsp/lossless_neon.c PredictorAdd3_NEON
// Predictor3: Top-right.
generate_predictor_1!(predictor_add3_neon, 3, upper, i => upper + i + 1);
// upstream: src/dsp/lossless_neon.c PredictorAdd4_NEON
// Predictor4: Top-left.
generate_predictor_1!(predictor_add4_neon, 4, upper, i => upper + i - 1);

/// Entry point of `PredictorAdd2_NEON` for `VP8LPredictorsAdd[2]`.
pub fn predictor_add2(
    data: &mut [u32],
    in_pos: usize,
    upper_pos: usize,
    num_pixels: i32,
    out_pos: usize,
) {
    // SAFETY: NEON is in the AArch64 baseline that this module is compiled for,
    // and upstream selects the NEON variant there unconditionally (ADR 0008).
    unsafe { predictor_add2_neon(data, in_pos, upper_pos, num_pixels, out_pos) }
}

/// Entry point of `PredictorAdd3_NEON` for `VP8LPredictorsAdd[3]`.
pub fn predictor_add3(
    data: &mut [u32],
    in_pos: usize,
    upper_pos: usize,
    num_pixels: i32,
    out_pos: usize,
) {
    // SAFETY: NEON is in the AArch64 baseline that this module is compiled for,
    // and upstream selects the NEON variant there unconditionally (ADR 0008).
    unsafe { predictor_add3_neon(data, in_pos, upper_pos, num_pixels, out_pos) }
}

/// Entry point of `PredictorAdd4_NEON` for `VP8LPredictorsAdd[4]`.
pub fn predictor_add4(
    data: &mut [u32],
    in_pos: usize,
    upper_pos: usize,
    num_pixels: i32,
    out_pos: usize,
) {
    // SAFETY: NEON is in the AArch64 baseline that this module is compiled for,
    // and upstream selects the NEON variant there unconditionally (ADR 0008).
    unsafe { predictor_add4_neon(data, in_pos, upper_pos, num_pixels, out_pos) }
}

// upstream: src/dsp/lossless_neon.c PredictorAdd5_NEON
/// Predictor5: average(average(left, TR), T)
#[target_feature(enable = "neon")]
fn predictor_add5_neon(
    data: &mut [u32],
    in_pos: usize,
    upper_pos: usize,
    num_pixels: i32,
    out_pos: usize,
) {
    let n = count(num_pixels);
    let mut l = loadq_u32_as_u8(data[out_pos - 1]);
    let mut i = 0;
    while i + 4 <= n {
        let src = loadq_u32p_as_u8(&data[in_pos + i..]);
        let t = loadq_u32p_as_u8(&data[upper_pos + i..]);
        let tr = loadq_u32p_as_u8(&data[upper_pos + i + 1..]);
        // upstream: src/dsp/lossless_neon.c DO_PRED5
        macro_rules! do_pred5 {
            ($lane:literal) => {{
                let avg_ltr = vhaddq_u8(l, tr);
                let avg = vhaddq_u8(avg_ltr, t);
                let res = vaddq_u8(avg, src);
                data[out_pos + i + $lane] = vgetq_lane_u32::<$lane>(vreinterpretq_u32_u8(res));
                l = rotate32_left(res);
            }};
        }
        do_pred5!(0);
        do_pred5!(1);
        do_pred5!(2);
        do_pred5!(3);
        i += 4;
    }
    vp8l_predictors_add_c(
        5,
        data,
        in_pos + i,
        upper_pos + i,
        num_pixels - int(i),
        out_pos + i,
    );
}

/// Entry point of `PredictorAdd5_NEON` for `VP8LPredictorsAdd[5]`.
pub fn predictor_add5(
    data: &mut [u32],
    in_pos: usize,
    upper_pos: usize,
    num_pixels: i32,
    out_pos: usize,
) {
    // SAFETY: NEON is in the AArch64 baseline that this module is compiled for,
    // and upstream selects the NEON variant there unconditionally (ADR 0008).
    unsafe { predictor_add5_neon(data, in_pos, upper_pos, num_pixels, out_pos) }
}

// upstream: src/dsp/lossless_neon.c PredictorAdd6_NEON
/// Predictor6: average(left, TL)
#[target_feature(enable = "neon")]
fn predictor_add6_neon(
    data: &mut [u32],
    in_pos: usize,
    upper_pos: usize,
    num_pixels: i32,
    out_pos: usize,
) {
    let n = count(num_pixels);
    let mut l = loadq_u32_as_u8(data[out_pos - 1]);
    let mut i = 0;
    while i + 4 <= n {
        let src = loadq_u32p_as_u8(&data[in_pos + i..]);
        let top = loadq_u32p_as_u8(&data[upper_pos + i - 1..]);
        // upstream: src/dsp/lossless_neon.c DO_PRED67
        macro_rules! do_pred67 {
            ($lane:literal) => {{
                let avg = vhaddq_u8(l, top);
                let res = vaddq_u8(avg, src);
                data[out_pos + i + $lane] = vgetq_lane_u32::<$lane>(vreinterpretq_u32_u8(res));
                l = rotate32_left(res);
            }};
        }
        do_pred67!(0);
        do_pred67!(1);
        do_pred67!(2);
        do_pred67!(3);
        i += 4;
    }
    vp8l_predictors_add_c(
        6,
        data,
        in_pos + i,
        upper_pos + i,
        num_pixels - int(i),
        out_pos + i,
    );
}

/// Entry point of `PredictorAdd6_NEON` for `VP8LPredictorsAdd[6]`.
pub fn predictor_add6(
    data: &mut [u32],
    in_pos: usize,
    upper_pos: usize,
    num_pixels: i32,
    out_pos: usize,
) {
    // SAFETY: NEON is in the AArch64 baseline that this module is compiled for,
    // and upstream selects the NEON variant there unconditionally (ADR 0008).
    unsafe { predictor_add6_neon(data, in_pos, upper_pos, num_pixels, out_pos) }
}

// upstream: src/dsp/lossless_neon.c PredictorAdd7_NEON
/// Predictor7: average(left, T)
#[target_feature(enable = "neon")]
fn predictor_add7_neon(
    data: &mut [u32],
    in_pos: usize,
    upper_pos: usize,
    num_pixels: i32,
    out_pos: usize,
) {
    let n = count(num_pixels);
    let mut l = loadq_u32_as_u8(data[out_pos - 1]);
    let mut i = 0;
    while i + 4 <= n {
        let src = loadq_u32p_as_u8(&data[in_pos + i..]);
        let top = loadq_u32p_as_u8(&data[upper_pos + i..]);
        // upstream: src/dsp/lossless_neon.c DO_PRED67
        macro_rules! do_pred67 {
            ($lane:literal) => {{
                let avg = vhaddq_u8(l, top);
                let res = vaddq_u8(avg, src);
                data[out_pos + i + $lane] = vgetq_lane_u32::<$lane>(vreinterpretq_u32_u8(res));
                l = rotate32_left(res);
            }};
        }
        do_pred67!(0);
        do_pred67!(1);
        do_pred67!(2);
        do_pred67!(3);
        i += 4;
    }
    vp8l_predictors_add_c(
        7,
        data,
        in_pos + i,
        upper_pos + i,
        num_pixels - int(i),
        out_pos + i,
    );
}

/// Entry point of `PredictorAdd7_NEON` for `VP8LPredictorsAdd[7]`.
pub fn predictor_add7(
    data: &mut [u32],
    in_pos: usize,
    upper_pos: usize,
    num_pixels: i32,
    out_pos: usize,
) {
    // SAFETY: NEON is in the AArch64 baseline that this module is compiled for,
    // and upstream selects the NEON variant there unconditionally (ADR 0008).
    unsafe { predictor_add7_neon(data, in_pos, upper_pos, num_pixels, out_pos) }
}

// upstream: src/dsp/lossless_neon.c GENERATE_PREDICTOR_2
/// The average of `upper[i]` and the C `IN`, added to `in`. `$upper` and
/// `$i` name the position of `upper` and the index in the expression
/// `$other`, the position of the C `IN`.
macro_rules! generate_predictor_2 {
    ($name:ident, $x:literal, $upper:ident, $i:ident => $other:expr) => {
        #[target_feature(enable = "neon")]
        fn $name(data: &mut [u32], in_pos: usize, $upper: usize, num_pixels: i32, out_pos: usize) {
            let n = count(num_pixels);
            let mut $i = 0;
            while $i + 4 <= n {
                let src = loadq_u32p_as_u8(&data[in_pos + $i..]);
                let t_other = loadq_u32p_as_u8(&data[$other..]);
                let t = loadq_u32p_as_u8(&data[$upper + $i..]);
                let avg = vhaddq_u8(t, t_other);
                let res = vaddq_u8(avg, src);
                storeq_u8_as_u32p(&mut data[out_pos + $i..], res);
                $i += 4;
            }
            vp8l_predictors_add_c(
                $x,
                data,
                in_pos + $i,
                $upper + $i,
                num_pixels - int($i),
                out_pos + $i,
            );
        }
    };
}
// upstream: src/dsp/lossless_neon.c PredictorAdd8_NEON
// Predictor8: average TL T.
generate_predictor_2!(predictor_add8_neon, 8, upper, i => upper + i - 1);
// upstream: src/dsp/lossless_neon.c PredictorAdd9_NEON
// Predictor9: average T TR.
generate_predictor_2!(predictor_add9_neon, 9, upper, i => upper + i + 1);

/// Entry point of `PredictorAdd8_NEON` for `VP8LPredictorsAdd[8]`.
pub fn predictor_add8(
    data: &mut [u32],
    in_pos: usize,
    upper_pos: usize,
    num_pixels: i32,
    out_pos: usize,
) {
    // SAFETY: NEON is in the AArch64 baseline that this module is compiled for,
    // and upstream selects the NEON variant there unconditionally (ADR 0008).
    unsafe { predictor_add8_neon(data, in_pos, upper_pos, num_pixels, out_pos) }
}

/// Entry point of `PredictorAdd9_NEON` for `VP8LPredictorsAdd[9]`.
pub fn predictor_add9(
    data: &mut [u32],
    in_pos: usize,
    upper_pos: usize,
    num_pixels: i32,
    out_pos: usize,
) {
    // SAFETY: NEON is in the AArch64 baseline that this module is compiled for,
    // and upstream selects the NEON variant there unconditionally (ADR 0008).
    unsafe { predictor_add9_neon(data, in_pos, upper_pos, num_pixels, out_pos) }
}

// upstream: src/dsp/lossless_neon.c PredictorAdd10_NEON
/// Predictor10: average of (average of (L,TL), average of (T, TR)).
#[target_feature(enable = "neon")]
fn predictor_add10_neon(
    data: &mut [u32],
    in_pos: usize,
    upper_pos: usize,
    num_pixels: i32,
    out_pos: usize,
) {
    let n = count(num_pixels);
    let mut l = loadq_u32_as_u8(data[out_pos - 1]);
    let mut i = 0;
    while i + 4 <= n {
        let src = loadq_u32p_as_u8(&data[in_pos + i..]);
        let tl = loadq_u32p_as_u8(&data[upper_pos + i - 1..]);
        let t = loadq_u32p_as_u8(&data[upper_pos + i..]);
        let tr = loadq_u32p_as_u8(&data[upper_pos + i + 1..]);
        let avg_ttr = vhaddq_u8(t, tr);
        // upstream: src/dsp/lossless_neon.c DO_PRED10
        macro_rules! do_pred10 {
            ($lane:literal) => {{
                let avg_ltl = vhaddq_u8(l, tl);
                let avg = vhaddq_u8(avg_ttr, avg_ltl);
                let res = vaddq_u8(avg, src);
                data[out_pos + i + $lane] = vgetq_lane_u32::<$lane>(vreinterpretq_u32_u8(res));
                l = rotate32_left(res);
            }};
        }
        do_pred10!(0);
        do_pred10!(1);
        do_pred10!(2);
        do_pred10!(3);
        i += 4;
    }
    vp8l_predictors_add_c(
        10,
        data,
        in_pos + i,
        upper_pos + i,
        num_pixels - int(i),
        out_pos + i,
    );
}

/// Entry point of `PredictorAdd10_NEON` for `VP8LPredictorsAdd[10]`.
pub fn predictor_add10(
    data: &mut [u32],
    in_pos: usize,
    upper_pos: usize,
    num_pixels: i32,
    out_pos: usize,
) {
    // SAFETY: NEON is in the AArch64 baseline that this module is compiled for,
    // and upstream selects the NEON variant there unconditionally (ADR 0008).
    unsafe { predictor_add10_neon(data, in_pos, upper_pos, num_pixels, out_pos) }
}

// upstream: src/dsp/lossless_neon.c PredictorAdd11_NEON
/// Predictor11: select.
#[target_feature(enable = "neon")]
fn predictor_add11_neon(
    data: &mut [u32],
    in_pos: usize,
    upper_pos: usize,
    num_pixels: i32,
    out_pos: usize,
) {
    let n = count(num_pixels);
    let mut l = loadq_u32_as_u8(data[out_pos - 1]);
    let mut i = 0;
    while i + 4 <= n {
        let t = loadq_u32p_as_u8(&data[upper_pos + i..]);
        let tl = loadq_u32p_as_u8(&data[upper_pos + i - 1..]);
        let p_ttl = vabdq_u8(t, tl); // |T - TL|
        let sum_ttl = vpaddlq_u8(p_ttl);
        let pb = vpaddlq_u16(sum_ttl);
        let src = loadq_u32p_as_u8(&data[in_pos + i..]);
        let sum_tin = vaddq_u8(t, src); // in + T
        // upstream: src/dsp/lossless_neon.c DO_PRED11
        macro_rules! do_pred11 {
            ($lane:literal) => {{
                let sum_lin = vaddq_u8(l, src); // in + L
                let p_ltl = vabdq_u8(l, tl); // |L - TL|
                let sum_ltl = vpaddlq_u8(p_ltl);
                let pa = vpaddlq_u16(sum_ltl);
                let mask = vcleq_u32(pa, pb);
                let res = vbslq_u8(vreinterpretq_u8_u32(mask), sum_tin, sum_lin);
                data[out_pos + i + $lane] = vgetq_lane_u32::<$lane>(vreinterpretq_u32_u8(res));
                l = rotate32_left(res);
            }};
        }
        do_pred11!(0);
        do_pred11!(1);
        do_pred11!(2);
        do_pred11!(3);
        i += 4;
    }
    vp8l_predictors_add_c(
        11,
        data,
        in_pos + i,
        upper_pos + i,
        num_pixels - int(i),
        out_pos + i,
    );
}

/// Entry point of `PredictorAdd11_NEON` for `VP8LPredictorsAdd[11]`.
pub fn predictor_add11(
    data: &mut [u32],
    in_pos: usize,
    upper_pos: usize,
    num_pixels: i32,
    out_pos: usize,
) {
    // SAFETY: NEON is in the AArch64 baseline that this module is compiled for,
    // and upstream selects the NEON variant there unconditionally (ADR 0008).
    unsafe { predictor_add11_neon(data, in_pos, upper_pos, num_pixels, out_pos) }
}

// upstream: src/dsp/lossless_neon.c PredictorAdd12_NEON
/// Predictor12: `ClampedAddSubtractFull`.
#[target_feature(enable = "neon")]
fn predictor_add12_neon(
    data: &mut [u32],
    in_pos: usize,
    upper_pos: usize,
    num_pixels: i32,
    out_pos: usize,
) {
    let n = count(num_pixels);
    let mut l: uint16x8_t = vmovl_u8(load_u32_as_u8(data[out_pos - 1]));
    let mut i = 0;
    while i + 4 <= n {
        // load four pixels of source
        let src = loadq_u32p_as_u8(&data[in_pos + i..]);
        // precompute the difference T - TL once for all, stored as s16
        let tl = loadq_u32p_as_u8(&data[upper_pos + i - 1..]);
        let t = loadq_u32p_as_u8(&data[upper_pos + i..]);
        let diff_lo = vreinterpretq_s16_u16(vsubl_u8(vget_low_u8(t), vget_low_u8(tl)));
        let diff_hi = vreinterpretq_s16_u16(vsubl_u8(vget_high_u8(t), vget_high_u8(tl)));
        // upstream: src/dsp/lossless_neon.c DO_PRED12
        macro_rules! do_pred12 {
            ($diff:ident, $lane:literal) => {{
                let pred = vqmovun_s16(vaddq_s16(vreinterpretq_s16_u16(l), $diff));
                let res = vadd_u8(
                    pred,
                    if $lane <= 1 {
                        vget_low_u8(src)
                    } else {
                        vget_high_u8(src)
                    },
                );
                let res16 = vmovl_u8(res);
                data[out_pos + i + $lane] =
                    vget_lane_u32::<{ $lane & 1 }>(vreinterpret_u32_u8(res));
                // rotate in the left predictor for next iteration
                l = vextq_u16::<4>(res16, res16);
            }};
        }
        // loop over the four reconstructed pixels
        do_pred12!(diff_lo, 0);
        do_pred12!(diff_lo, 1);
        do_pred12!(diff_hi, 2);
        do_pred12!(diff_hi, 3);
        i += 4;
    }
    vp8l_predictors_add_c(
        12,
        data,
        in_pos + i,
        upper_pos + i,
        num_pixels - int(i),
        out_pos + i,
    );
}

/// Entry point of `PredictorAdd12_NEON` for `VP8LPredictorsAdd[12]`.
pub fn predictor_add12(
    data: &mut [u32],
    in_pos: usize,
    upper_pos: usize,
    num_pixels: i32,
    out_pos: usize,
) {
    // SAFETY: NEON is in the AArch64 baseline that this module is compiled for,
    // and upstream selects the NEON variant there unconditionally (ADR 0008).
    unsafe { predictor_add12_neon(data, in_pos, upper_pos, num_pixels, out_pos) }
}

// upstream: src/dsp/lossless_neon.c PredictorAdd13_NEON
/// Predictor13: `ClampedAddSubtractHalf`
#[target_feature(enable = "neon")]
fn predictor_add13_neon(
    data: &mut [u32],
    in_pos: usize,
    upper_pos: usize,
    num_pixels: i32,
    out_pos: usize,
) {
    let n = count(num_pixels);
    let mut l = loadq_u32_as_u8(data[out_pos - 1]);
    let mut i = 0;
    while i + 4 <= n {
        let src = loadq_u32p_as_u8(&data[in_pos + i..]);
        let t = loadq_u32p_as_u8(&data[upper_pos + i..]);
        let tl = loadq_u32p_as_u8(&data[upper_pos + i - 1..]);
        // upstream: src/dsp/lossless_neon.c DO_PRED13
        macro_rules! do_pred13 {
            ($lane:literal, $low_or_hi:ident) => {{
                let avg = vhaddq_u8(l, t);
                let cmp = vcgtq_u8(tl, avg);
                let tl_1 = vaddq_u8(tl, cmp);
                // Compute half of the difference between avg and TL'.
                let diff_avg = vreinterpret_s8_u8($low_or_hi(vhsubq_u8(avg, tl_1)));
                // Compute the sum with avg and saturate.
                let avg_16 = vreinterpretq_s16_u16(vmovl_u8($low_or_hi(avg)));
                let delta = vqmovun_s16(vaddw_s8(avg_16, diff_avg));
                let res = vadd_u8($low_or_hi(src), delta);
                let res2 = vcombine_u8(res, res);
                data[out_pos + i + $lane] =
                    vget_lane_u32::<{ $lane & 1 }>(vreinterpret_u32_u8(res));
                l = rotate32_left(res2);
            }};
        }
        do_pred13!(0, vget_low_u8);
        do_pred13!(1, vget_low_u8);
        do_pred13!(2, vget_high_u8);
        do_pred13!(3, vget_high_u8);
        i += 4;
    }
    vp8l_predictors_add_c(
        13,
        data,
        in_pos + i,
        upper_pos + i,
        num_pixels - int(i),
        out_pos + i,
    );
}

/// Entry point of `PredictorAdd13_NEON` for `VP8LPredictorsAdd[13]`.
pub fn predictor_add13(
    data: &mut [u32],
    in_pos: usize,
    upper_pos: usize,
    num_pixels: i32,
    out_pos: usize,
) {
    // SAFETY: NEON is in the AArch64 baseline that this module is compiled for,
    // and upstream selects the NEON variant there unconditionally (ADR 0008).
    unsafe { predictor_add13_neon(data, in_pos, upper_pos, num_pixels, out_pos) }
}

//------------------------------------------------------------------------------
// Subtract-Green Transform

// upstream: src/dsp/lossless_neon.c kGreenShuffle
/// 255 = byte will be zeroed
const GREEN_SHUFFLE: [u8; 8] = [1, 255, 1, 255, 5, 255, 5, 255];

// upstream: src/dsp/lossless_neon.c DoGreenShuffle_NEON
#[target_feature(enable = "neon")]
#[must_use]
fn do_green_shuffle_neon(argb: uint8x16_t, shuffle: uint8x8_t) -> uint8x16_t {
    vcombine_u8(
        vtbl1_u8(vget_low_u8(argb), shuffle),
        vtbl1_u8(vget_high_u8(argb), shuffle),
    )
}

// upstream: src/dsp/lossless_neon.c AddGreenToBlueAndRed_NEON
#[target_feature(enable = "neon")]
fn add_green_to_blue_and_red_neon(
    data: &mut [u32],
    src_pos: usize,
    num_pixels: i32,
    dst_pos: usize,
) {
    let end = count(num_pixels & !3);
    let shuffle = vld1_u8(&GREEN_SHUFFLE);
    let mut i = 0;
    while i < end {
        let argb = vld1q_u8_words(&data[src_pos + i..]);
        let greens = do_green_shuffle_neon(argb, shuffle);
        vst1q_u8_words(&mut data[dst_pos + i..], vaddq_u8(argb, greens));
        i += 4;
    }
    // fallthrough and finish off with plain-C
    vp8l_add_green_to_blue_and_red_c(data, src_pos + i, num_pixels & 3, dst_pos + i);
}

/// Entry point of `AddGreenToBlueAndRed_NEON` for
/// `VP8LAddGreenToBlueAndRed`.
pub fn add_green_to_blue_and_red(
    data: &mut [u32],
    src_pos: usize,
    num_pixels: i32,
    dst_pos: usize,
) {
    // SAFETY: NEON is in the AArch64 baseline that this module is compiled for,
    // and upstream selects the NEON variant there unconditionally (ADR 0008).
    unsafe { add_green_to_blue_and_red_neon(data, src_pos, num_pixels, dst_pos) }
}

//------------------------------------------------------------------------------
// Color Transform

// upstream: src/dsp/lossless_neon.c CST
/// sign-extended multiplying constants, pre-shifted by 6.
#[must_use]
fn cst(x: u8) -> i16 {
    (u16::from(x) << 8).cast_signed() >> 6
}

// upstream: src/dsp/lossless_neon.c TransformColorInverse_NEON
#[target_feature(enable = "neon")]
#[expect(
    clippy::many_single_char_names,
    reason = "the C names m, A, B, C, D, E, F and G"
)]
#[expect(
    clippy::trivially_copy_pass_by_ref,
    reason = "the C function takes a pointer to the multipliers"
)]
fn transform_color_inverse_neon(
    m: &VP8LMultipliers,
    data: &mut [u32],
    src_pos: usize,
    num_pixels: i32,
    dst_pos: usize,
) {
    // upstream: src/dsp/lossless_neon.c k0g0g
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
    let mask_ag = vdupq_n_u32(0xff00_ff00);
    let n = count(num_pixels);
    let mut i = 0;
    while i + 4 <= n {
        let r#in = vld1q_u8_words(&data[src_pos + i..]);
        let a0g0 = vandq_u32(vreinterpretq_u32_u8(r#in), mask_ag);
        // 0 g 0 g
        let greens = do_green_shuffle_neon(r#in, shuffle);
        // x dr  x db1
        let a = vqdmulhq_s16(vreinterpretq_s16_u8(greens), mults_rb);
        // x r'  x   b'
        let b = vaddq_s8(vreinterpretq_s8_u8(r#in), vreinterpretq_s8_s16(a));
        // r' 0   b' 0
        let c = vshlq_n_s16::<8>(vreinterpretq_s16_s8(b));
        // x db2  0  0
        let d = vqdmulhq_s16(c, mults_b2);
        // 0  x db2  0
        let e = vshrq_n_u32::<8>(vreinterpretq_u32_s16(d));
        // r' x  b'' 0
        let f = vaddq_s8(vreinterpretq_s8_u32(e), vreinterpretq_s8_s16(c));
        // 0  r'  0  b''
        let g = vshrq_n_u16::<8>(vreinterpretq_u16_s8(f));
        let out = vorrq_u32(vreinterpretq_u32_u16(g), a0g0);
        vst1q_u32(&mut data[dst_pos + i..], out);
        i += 4;
    }
    // Fall-back to C-version for left-overs.
    vp8l_transform_color_inverse_c(m, data, src_pos + i, num_pixels - int(i), dst_pos + i);
}

/// Entry point of `TransformColorInverse_NEON` for
/// `VP8LTransformColorInverse`.
pub fn transform_color_inverse(
    m: &VP8LMultipliers,
    data: &mut [u32],
    src_pos: usize,
    num_pixels: i32,
    dst_pos: usize,
) {
    // SAFETY: NEON is in the AArch64 baseline that this module is compiled for,
    // and upstream selects the NEON variant there unconditionally (ADR 0008).
    unsafe { transform_color_inverse_neon(m, data, src_pos, num_pixels, dst_pos) }
}

//------------------------------------------------------------------------------
// Entry point

// upstream: src/dsp/lossless_neon.c VP8LDspInitNEON
// The pointer assignments are the dispatch functions of `src/dsp/lossless.rs`
// named in the module documentation (ADR 0008).
