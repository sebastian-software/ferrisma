// libwebp v1.6.0, src/dsp/lossless.c:
//
// Copyright 2012 Google Inc. All Rights Reserved.
//
// Use of this source code is governed by a BSD-style license
// that can be found in the COPYING file in the root of the source
// tree. An additional intellectual property rights grant can be found
// in the file PATENTS. All contributing project authors may
// be found in the AUTHORS file in the root of the source tree.
// -----------------------------------------------------------------------------
//
// Image transforms and color space conversion methods for lossless decoder.
//
// Authors: Vikas Arora (vikaas.arora@gmail.com)
//          Jyrki Alakuijala (jyrki@google.com)
//          Urvang Joshi (urvang@google.com)

// libwebp v1.6.0, src/dsp/lossless.h:
//
// Copyright 2012 Google Inc. All Rights Reserved.
//
// Use of this source code is governed by a BSD-style license
// that can be found in the COPYING file in the root of the source
// tree. An additional intellectual property rights grant can be found
// in the file PATENTS. All contributing project authors may
// be found in the AUTHORS file in the root of the source tree.
// -----------------------------------------------------------------------------
//
// Image transforms and color space conversion methods for lossless decoder.
//
// Authors: Vikas Arora (vikaas.arora@gmail.com)
//          Jyrki Alakuijala (jyrki@google.com)

//! Translation of `src/dsp/lossless.c` and `src/dsp/lossless.h` from libwebp v1.6.0.
//!
//! Work package: WP3 (lossless shared).
//!
//! Representation (ADR 0003, Pointers): the inverse transforms of the decoder
//! read and write one pixel buffer. Their input and output may be the same
//! rows (the transforms run in place), and the predictors read the row above
//! the output and the output pixel to the left. Each such function takes the
//! buffer once, as `data`, with the C pointers as positions in it, so every
//! read and write happens at the index and in the order of the C code,
//! whatever the overlap. The 8-bit alpha path and the color-space converters
//! take distinct buffers, as their callers have them.
//!
//! Not translated here, with the reason:
//! - `VP8LDspInit` only assigns the portable functions (ADR 0003); its calls are
//!   omitted. The SIMD pointers and initializers are wave 4.
//! - The encoder functions that `lossless.h` declares (`VP8LTransformColor_C`,
//!   `VP8LSubtractGreenFromBlueAndRed_C`, the collectors, the entropy and cost
//!   functions, `VP8LBundleColorMap_C`, `VP8LPredictorsSub`, `VP8LEncDspInit`
//!   and their pointers) are defined in `src/dsp/lossless_enc.c` (WP11).

use crate::dec::vp8li_dec::VP8LTransform;
use crate::dsp::alpha_processing::{webp_apply_alpha_multiply, webp_apply_alpha_multiply4444};
use crate::dsp::lossless_common::{
    generate_predictor_add, vp8_get_alpha_index, vp8_get_alpha_value, vp8_get_argb_index,
    vp8_get_argb_value, vp8l_add_pixels, vp8l_sub_sample_size,
};
use crate::utils::endian_inl_utils::b_swap32;
use crate::utils::utils::webp_uint32_to_mem;
use crate::webp::decode::WEBP_CSP_MODE;
use crate::webp::format_constants::{ARGB_BLACK, VP8LImageTransformType};

//------------------------------------------------------------------------------
// Decoding

// upstream: src/dsp/lossless.h VP8LPredictorFunc
/// A one-pixel predictor: `left[left_pos]` is the C `*left`, and
/// `top[top_pos + k]` is the C `top[k]` for `k` in -1..=1.
pub type VP8LPredictorFunc = fn(&[u32], usize, &[u32], usize) -> u32;

// upstream: src/dsp/lossless.h VP8LPredictorAddSubFunc
/// The decoder form of a batch predictor: `(data, in, upper, num_pixels, out)`,
/// with `in`, `upper` and `out` as positions in `data`. Expects `upper - 1` and
/// `out - 1` to be readable, as the C functions expect `upper[-1]` and
/// `out[-1]`.
pub type VP8LPredictorAddSubFunc = fn(&mut [u32], usize, usize, i32, usize);

// upstream: src/dsp/lossless.h VP8LMultipliers
/// Note: the members are `uint8_t`, so that any negative values are
/// automatically converted to "mod 256" values.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct VP8LMultipliers {
    pub green_to_red: u8,
    pub green_to_blue: u8,
    pub red_to_blue: u8,
}

// upstream: src/dsp/lossless.h VP8LStreaks
/// Small struct to hold counters.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct VP8LStreaks {
    /// Index: 0=zero streak, 1=non-zero streak.
    pub counts: [i32; 2],
    /// [zero/non-zero][streak<3 / streak>=3].
    pub streaks: [[i32; 2]; 2],
}

// upstream: src/dsp/lossless.h VP8LBitEntropy
/// Small struct to hold bit entropy results.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct VP8LBitEntropy {
    /// Entropy.
    pub entropy: u64,
    /// Sum of the population.
    pub sum: u32,
    /// Number of non-zero elements in the population.
    pub nonzeros: i32,
    /// Maximum value in the population.
    pub max_val: u32,
    /// Index of the last non-zero in the population.
    pub nonzero_code: u32,
}

/// The pixel count of a C loop `for (i = 0; i < num_pixels; ++i)`, which runs
/// no iteration for a negative count.
#[inline]
fn pixel_count(num_pixels: i32) -> usize {
    usize::try_from(num_pixels).unwrap_or(0)
}

/// Converts a position or a size of the transforms, which is never negative,
/// to a slice index.
#[inline]
fn position(value: i32) -> usize {
    usize::try_from(value).expect("a transform position is not negative")
}

//------------------------------------------------------------------------------
// Image transforms.

// upstream: src/dsp/lossless.c Average2
#[must_use]
#[inline]
fn average2(a0: u32, a1: u32) -> u32 {
    (((a0 ^ a1) & 0xfefe_fefe) >> 1) + (a0 & a1)
}

// upstream: src/dsp/lossless.c Average3
#[must_use]
#[inline]
fn average3(a0: u32, a1: u32, a2: u32) -> u32 {
    average2(average2(a0, a2), a1)
}

// upstream: src/dsp/lossless.c Average4
#[must_use]
#[inline]
fn average4(a0: u32, a1: u32, a2: u32, a3: u32) -> u32 {
    average2(average2(a0, a1), average2(a2, a3))
}

// upstream: src/dsp/lossless.c Clip255
#[must_use]
#[inline]
fn clip255(a: u32) -> u32 {
    if a < 256 {
        return a;
    }
    // return 0, when a is a negative integer.
    // return 255, when a is positive.
    !a >> 24
}

// upstream: src/dsp/lossless.c AddSubtractComponentFull
#[must_use]
#[inline]
fn add_subtract_component_full(a: i32, b: i32, c: i32) -> i32 {
    clip255((a + b - c).cast_unsigned()).cast_signed()
}

/// The C conversion of a `uint32_t` channel to the `int` parameters of the
/// component helpers; a channel is at most 255.
#[inline]
fn channel(value: u32) -> i32 {
    value.cast_signed()
}

// upstream: src/dsp/lossless.c ClampedAddSubtractFull
#[must_use]
#[inline]
fn clamped_add_subtract_full(c0: u32, c1: u32, c2: u32) -> u32 {
    let a = add_subtract_component_full(channel(c0 >> 24), channel(c1 >> 24), channel(c2 >> 24));
    let r = add_subtract_component_full(
        channel((c0 >> 16) & 0xff),
        channel((c1 >> 16) & 0xff),
        channel((c2 >> 16) & 0xff),
    );
    let g = add_subtract_component_full(
        channel((c0 >> 8) & 0xff),
        channel((c1 >> 8) & 0xff),
        channel((c2 >> 8) & 0xff),
    );
    let b = add_subtract_component_full(channel(c0 & 0xff), channel(c1 & 0xff), channel(c2 & 0xff));
    (a.cast_unsigned() << 24) | ((r << 16) | (g << 8) | b).cast_unsigned()
}

// upstream: src/dsp/lossless.c AddSubtractComponentHalf
#[must_use]
#[inline]
fn add_subtract_component_half(a: i32, b: i32) -> i32 {
    clip255((a + (a - b) / 2).cast_unsigned()).cast_signed()
}

// upstream: src/dsp/lossless.c ClampedAddSubtractHalf
#[must_use]
#[inline]
fn clamped_add_subtract_half(c0: u32, c1: u32, c2: u32) -> u32 {
    let ave = average2(c0, c1);
    let a = add_subtract_component_half(channel(ave >> 24), channel(c2 >> 24));
    let r = add_subtract_component_half(channel((ave >> 16) & 0xff), channel((c2 >> 16) & 0xff));
    let g = add_subtract_component_half(channel((ave >> 8) & 0xff), channel((c2 >> 8) & 0xff));
    let b = add_subtract_component_half(channel(ave & 0xff), channel(c2 & 0xff));
    (a.cast_unsigned() << 24) | ((r << 16) | (g << 8) | b).cast_unsigned()
}

// upstream: src/dsp/lossless.c Sub3
#[must_use]
#[inline]
fn sub3(a: i32, b: i32, c: i32) -> i32 {
    let pb = b - c;
    let pa = a - c;
    pb.abs() - pa.abs()
}

// upstream: src/dsp/lossless.c Select
#[must_use]
#[inline]
fn select(a: u32, b: u32, c: u32) -> u32 {
    let pa_minus_pb = sub3(channel(a >> 24), channel(b >> 24), channel(c >> 24))
        + sub3(
            channel((a >> 16) & 0xff),
            channel((b >> 16) & 0xff),
            channel((c >> 16) & 0xff),
        )
        + sub3(
            channel((a >> 8) & 0xff),
            channel((b >> 8) & 0xff),
            channel((c >> 8) & 0xff),
        )
        + sub3(channel(a & 0xff), channel(b & 0xff), channel(c & 0xff));
    if pa_minus_pb <= 0 { a } else { b }
}

//------------------------------------------------------------------------------
// Predictors

// upstream: src/dsp/lossless.c VP8LPredictor0_C
fn vp8l_predictor0_c(_left: &[u32], _left_pos: usize, _top: &[u32], _top_pos: usize) -> u32 {
    ARGB_BLACK
}

// upstream: src/dsp/lossless.c VP8LPredictor1_C
fn vp8l_predictor1_c(left: &[u32], left_pos: usize, _top: &[u32], _top_pos: usize) -> u32 {
    left[left_pos]
}

// upstream: src/dsp/lossless.c VP8LPredictor2_C
#[must_use]
pub fn vp8l_predictor2_c(_left: &[u32], _left_pos: usize, top: &[u32], top_pos: usize) -> u32 {
    top[top_pos]
}

// upstream: src/dsp/lossless.c VP8LPredictor3_C
#[must_use]
pub fn vp8l_predictor3_c(_left: &[u32], _left_pos: usize, top: &[u32], top_pos: usize) -> u32 {
    top[top_pos + 1]
}

// upstream: src/dsp/lossless.c VP8LPredictor4_C
#[must_use]
pub fn vp8l_predictor4_c(_left: &[u32], _left_pos: usize, top: &[u32], top_pos: usize) -> u32 {
    top[top_pos - 1]
}

// upstream: src/dsp/lossless.c VP8LPredictor5_C
#[must_use]
pub fn vp8l_predictor5_c(left: &[u32], left_pos: usize, top: &[u32], top_pos: usize) -> u32 {
    average3(left[left_pos], top[top_pos], top[top_pos + 1])
}

// upstream: src/dsp/lossless.c VP8LPredictor6_C
#[must_use]
pub fn vp8l_predictor6_c(left: &[u32], left_pos: usize, top: &[u32], top_pos: usize) -> u32 {
    average2(left[left_pos], top[top_pos - 1])
}

// upstream: src/dsp/lossless.c VP8LPredictor7_C
#[must_use]
pub fn vp8l_predictor7_c(left: &[u32], left_pos: usize, top: &[u32], top_pos: usize) -> u32 {
    average2(left[left_pos], top[top_pos])
}

// upstream: src/dsp/lossless.c VP8LPredictor8_C
#[must_use]
pub fn vp8l_predictor8_c(_left: &[u32], _left_pos: usize, top: &[u32], top_pos: usize) -> u32 {
    average2(top[top_pos - 1], top[top_pos])
}

// upstream: src/dsp/lossless.c VP8LPredictor9_C
#[must_use]
pub fn vp8l_predictor9_c(_left: &[u32], _left_pos: usize, top: &[u32], top_pos: usize) -> u32 {
    average2(top[top_pos], top[top_pos + 1])
}

// upstream: src/dsp/lossless.c VP8LPredictor10_C
#[must_use]
pub fn vp8l_predictor10_c(left: &[u32], left_pos: usize, top: &[u32], top_pos: usize) -> u32 {
    average4(
        left[left_pos],
        top[top_pos - 1],
        top[top_pos],
        top[top_pos + 1],
    )
}

// upstream: src/dsp/lossless.c VP8LPredictor11_C
#[must_use]
pub fn vp8l_predictor11_c(left: &[u32], left_pos: usize, top: &[u32], top_pos: usize) -> u32 {
    select(top[top_pos], left[left_pos], top[top_pos - 1])
}

// upstream: src/dsp/lossless.c VP8LPredictor12_C
#[must_use]
pub fn vp8l_predictor12_c(left: &[u32], left_pos: usize, top: &[u32], top_pos: usize) -> u32 {
    clamped_add_subtract_full(left[left_pos], top[top_pos], top[top_pos - 1])
}

// upstream: src/dsp/lossless.c VP8LPredictor13_C
#[must_use]
pub fn vp8l_predictor13_c(left: &[u32], left_pos: usize, top: &[u32], top_pos: usize) -> u32 {
    clamped_add_subtract_half(left[left_pos], top[top_pos], top[top_pos - 1])
}

// upstream: src/dsp/lossless.c PredictorAdd0_C
fn predictor_add0_c(
    data: &mut [u32],
    in_pos: usize,
    _upper_pos: usize,
    num_pixels: i32,
    out_pos: usize,
) {
    for x in 0..pixel_count(num_pixels) {
        data[out_pos + x] = vp8l_add_pixels(data[in_pos + x], ARGB_BLACK);
    }
}

// upstream: src/dsp/lossless.c PredictorAdd1_C
fn predictor_add1_c(
    data: &mut [u32],
    in_pos: usize,
    _upper_pos: usize,
    num_pixels: i32,
    out_pos: usize,
) {
    let mut left = data[out_pos - 1];
    for i in 0..pixel_count(num_pixels) {
        left = vp8l_add_pixels(data[in_pos + i], left);
        data[out_pos + i] = left;
    }
}

// upstream: src/dsp/lossless.c PredictorAdd2_C
generate_predictor_add!(vp8l_predictor2_c, predictor_add2_c);
// upstream: src/dsp/lossless.c PredictorAdd3_C
generate_predictor_add!(vp8l_predictor3_c, predictor_add3_c);
// upstream: src/dsp/lossless.c PredictorAdd4_C
generate_predictor_add!(vp8l_predictor4_c, predictor_add4_c);
// upstream: src/dsp/lossless.c PredictorAdd5_C
generate_predictor_add!(vp8l_predictor5_c, predictor_add5_c);
// upstream: src/dsp/lossless.c PredictorAdd6_C
generate_predictor_add!(vp8l_predictor6_c, predictor_add6_c);
// upstream: src/dsp/lossless.c PredictorAdd7_C
generate_predictor_add!(vp8l_predictor7_c, predictor_add7_c);
// upstream: src/dsp/lossless.c PredictorAdd8_C
generate_predictor_add!(vp8l_predictor8_c, predictor_add8_c);
// upstream: src/dsp/lossless.c PredictorAdd9_C
generate_predictor_add!(vp8l_predictor9_c, predictor_add9_c);
// upstream: src/dsp/lossless.c PredictorAdd10_C
generate_predictor_add!(vp8l_predictor10_c, predictor_add10_c);
// upstream: src/dsp/lossless.c PredictorAdd11_C
generate_predictor_add!(vp8l_predictor11_c, predictor_add11_c);
// upstream: src/dsp/lossless.c PredictorAdd12_C
generate_predictor_add!(vp8l_predictor12_c, predictor_add12_c);
// upstream: src/dsp/lossless.c PredictorAdd13_C
generate_predictor_add!(vp8l_predictor13_c, predictor_add13_c);

// upstream: src/dsp/lossless.c VP8LPredictors
/// The table that `VP8LDspInit` fills with `COPY_PREDICTOR_ARRAY`; entries 14
/// and 15 are padding security sentinels.
static VP8L_PREDICTORS: [VP8LPredictorFunc; 16] = [
    vp8l_predictor0_c,
    vp8l_predictor1_c,
    vp8l_predictor2_c,
    vp8l_predictor3_c,
    vp8l_predictor4_c,
    vp8l_predictor5_c,
    vp8l_predictor6_c,
    vp8l_predictor7_c,
    vp8l_predictor8_c,
    vp8l_predictor9_c,
    vp8l_predictor10_c,
    vp8l_predictor11_c,
    vp8l_predictor12_c,
    vp8l_predictor13_c,
    vp8l_predictor0_c, // <- padding security sentinels
    vp8l_predictor0_c,
];

// upstream: src/dsp/lossless.c VP8LPredictorsAdd
/// The table that `VP8LDspInit` fills with `COPY_PREDICTOR_ARRAY`.
static VP8L_PREDICTORS_ADD: [VP8LPredictorAddSubFunc; 16] = [
    predictor_add0_c,
    predictor_add1_c,
    predictor_add2_c,
    predictor_add3_c,
    predictor_add4_c,
    predictor_add5_c,
    predictor_add6_c,
    predictor_add7_c,
    predictor_add8_c,
    predictor_add9_c,
    predictor_add10_c,
    predictor_add11_c,
    predictor_add12_c,
    predictor_add13_c,
    predictor_add0_c, // <- padding security sentinels
    predictor_add0_c,
];

// upstream: src/dsp/lossless.c VP8LPredictorsAdd_C
/// The exposed plain-C implementations, which `VP8LDspInit` fills with
/// `COPY_PREDICTOR_ARRAY` as well.
static VP8L_PREDICTORS_ADD_C: [VP8LPredictorAddSubFunc; 16] = VP8L_PREDICTORS_ADD;

// upstream: src/dsp/lossless.c VP8LPredictors
/// Calls the predictor `VP8LPredictors[mode]`.
///
/// # Panics
///
/// Panics for a mode outside 0..16.
#[must_use]
#[inline]
pub fn vp8l_predictors(
    mode: usize,
    left: &[u32],
    left_pos: usize,
    top: &[u32],
    top_pos: usize,
) -> u32 {
    #[cfg(all(feature = "simd", target_arch = "aarch64"))]
    {
        // `VP8LDspInitNEON` assigns the entries 5, 6, 7 and 13.
        let neon: Option<VP8LPredictorFunc> = match mode {
            5 => Some(crate::dsp::lossless_neon::predictor5),
            6 => Some(crate::dsp::lossless_neon::predictor6),
            7 => Some(crate::dsp::lossless_neon::predictor7),
            13 => Some(crate::dsp::lossless_neon::predictor13),
            _ => None,
        };
        if let Some(predictor) = neon {
            return predictor(left, left_pos, top, top_pos);
        }
    }
    VP8L_PREDICTORS[mode](left, left_pos, top, top_pos)
}

// upstream: src/dsp/lossless.c VP8LPredictorsAdd
/// Calls the batch predictor `VP8LPredictorsAdd[mode]`.
///
/// # Panics
///
/// Panics for a mode outside 0..16.
#[inline]
pub fn vp8l_predictors_add(
    mode: usize,
    data: &mut [u32],
    in_pos: usize,
    upper_pos: usize,
    num_pixels: i32,
    out_pos: usize,
) {
    #[cfg(all(feature = "simd", target_arch = "aarch64"))]
    {
        // `VP8LDspInitNEON` assigns the entries 0 to 13; 14 and 15 keep the
        // C padding sentinels.
        let neon: Option<VP8LPredictorAddSubFunc> = match mode {
            0 => Some(crate::dsp::lossless_neon::predictor_add0),
            1 => Some(crate::dsp::lossless_neon::predictor_add1),
            2 => Some(crate::dsp::lossless_neon::predictor_add2),
            3 => Some(crate::dsp::lossless_neon::predictor_add3),
            4 => Some(crate::dsp::lossless_neon::predictor_add4),
            5 => Some(crate::dsp::lossless_neon::predictor_add5),
            6 => Some(crate::dsp::lossless_neon::predictor_add6),
            7 => Some(crate::dsp::lossless_neon::predictor_add7),
            8 => Some(crate::dsp::lossless_neon::predictor_add8),
            9 => Some(crate::dsp::lossless_neon::predictor_add9),
            10 => Some(crate::dsp::lossless_neon::predictor_add10),
            11 => Some(crate::dsp::lossless_neon::predictor_add11),
            12 => Some(crate::dsp::lossless_neon::predictor_add12),
            13 => Some(crate::dsp::lossless_neon::predictor_add13),
            _ => None,
        };
        if let Some(predictor_add) = neon {
            return predictor_add(data, in_pos, upper_pos, num_pixels, out_pos);
        }
    }
    VP8L_PREDICTORS_ADD[mode](data, in_pos, upper_pos, num_pixels, out_pos);
}

// upstream: src/dsp/lossless.c VP8LPredictorsAdd_C
/// Calls the plain-C batch predictor `VP8LPredictorsAdd_C[mode]`.
///
/// # Panics
///
/// Panics for a mode outside 0..16.
#[inline]
pub fn vp8l_predictors_add_c(
    mode: usize,
    data: &mut [u32],
    in_pos: usize,
    upper_pos: usize,
    num_pixels: i32,
    out_pos: usize,
) {
    VP8L_PREDICTORS_ADD_C[mode](data, in_pos, upper_pos, num_pixels, out_pos);
}

//------------------------------------------------------------------------------

// upstream: src/dsp/lossless.c PredictorInverseTransform_C
/// Inverse prediction of rows `y_start..y_end` from `data[in_pos..]` into
/// `data[out_pos..]`; for `y_start > 0` the row above `out_pos` holds the last
/// row of the previous call.
fn predictor_inverse_transform_c(
    transform: &VP8LTransform,
    mut y_start: i32,
    y_end: i32,
    data: &mut [u32],
    mut in_pos: usize,
    mut out_pos: usize,
) {
    let width = transform.xsize;
    let row = position(width);
    if y_start == 0 {
        // First Row follows the L (mode=1) mode.
        predictor_add0_c(data, in_pos, 0, 1, out_pos);
        predictor_add1_c(data, in_pos + 1, 0, width - 1, out_pos + 1);
        in_pos += row;
        out_pos += row;
        y_start += 1;
    }

    {
        let mut y = y_start;
        let tile_width: i32 = 1 << transform.bits;
        let mask = tile_width - 1;
        let tiles_per_row =
            vp8l_sub_sample_size(width.cast_unsigned(), transform.bits.cast_unsigned());
        let mut pred_mode_base =
            (y >> transform.bits).cast_unsigned() as usize * tiles_per_row as usize;

        while y < y_end {
            let mut pred_mode_src = pred_mode_base;
            let mut x: i32 = 1;
            // First pixel follows the T (mode=2) mode.
            predictor_add2_c(data, in_pos, out_pos - row, 1, out_pos);
            // .. the rest:
            while x < width {
                let mode = ((transform.data[pred_mode_src] >> 8) & 0xf) as usize;
                pred_mode_src += 1;
                let mut x_end = (x & !mask) + tile_width;
                if x_end > width {
                    x_end = width;
                }
                let at = position(x);
                vp8l_predictors_add(
                    mode,
                    data,
                    in_pos + at,
                    out_pos + at - row,
                    x_end - x,
                    out_pos + at,
                );
                x = x_end;
            }
            in_pos += row;
            out_pos += row;
            y += 1;
            if (y & mask) == 0 {
                // Use the same mask, since tiles are squares.
                pred_mode_base += tiles_per_row as usize;
            }
        }
    }
}

// upstream: src/dsp/lossless.c VP8LAddGreenToBlueAndRed_C
/// Add green to blue and red channels (i.e. perform the inverse transform of
/// 'subtract green'), from `data[src_pos..]` into `data[dst_pos..]`.
pub fn vp8l_add_green_to_blue_and_red_c(
    data: &mut [u32],
    src_pos: usize,
    num_pixels: i32,
    dst_pos: usize,
) {
    for i in 0..pixel_count(num_pixels) {
        let argb = data[src_pos + i];
        let green = (argb >> 8) & 0xff;
        let mut red_blue = argb & 0x00ff_00ff;
        red_blue = red_blue.wrapping_add((green << 16) | green);
        red_blue &= 0x00ff_00ff;
        data[dst_pos + i] = (argb & 0xff00_ff00) | red_blue;
    }
}

// upstream: src/dsp/lossless.c VP8LAddGreenToBlueAndRed
/// Calls the portable [`vp8l_add_green_to_blue_and_red_c`].
#[inline]
pub fn vp8l_add_green_to_blue_and_red(
    data: &mut [u32],
    src_pos: usize,
    num_pixels: i32,
    dst_pos: usize,
) {
    #[cfg(all(feature = "simd", target_arch = "aarch64"))]
    {
        return crate::dsp::lossless_neon::add_green_to_blue_and_red(
            data, src_pos, num_pixels, dst_pos,
        );
    }
    #[allow(
        unreachable_code,
        reason = "the C path is reached only without a SIMD variant"
    )]
    vp8l_add_green_to_blue_and_red_c(data, src_pos, num_pixels, dst_pos);
}

// upstream: src/dsp/lossless.c ColorTransformDelta
#[must_use]
#[inline]
fn color_transform_delta(color_pred: i8, color: i8) -> i32 {
    (i32::from(color_pred) * i32::from(color)) >> 5
}

// upstream: src/dsp/lossless.c ColorCodeToMultipliers
#[inline]
fn color_code_to_multipliers(color_code: u32, m: &mut VP8LMultipliers) {
    m.green_to_red = (color_code & 0xff) as u8;
    m.green_to_blue = ((color_code >> 8) & 0xff) as u8;
    m.red_to_blue = ((color_code >> 16) & 0xff) as u8;
}

/// The C `(int8_t)` cast of the low byte of a value.
#[inline]
#[expect(
    clippy::cast_possible_truncation,
    reason = "the C (int8_t) cast keeps the low byte"
)]
fn low_int8(value: u32) -> i8 {
    (value as u8).cast_signed()
}

// upstream: src/dsp/lossless.c VP8LTransformColorInverse_C
/// The inverse color transform of `num_pixels` pixels, from `data[src_pos..]`
/// into `data[dst_pos..]`.
pub fn vp8l_transform_color_inverse_c(
    m: &VP8LMultipliers,
    data: &mut [u32],
    src_pos: usize,
    num_pixels: i32,
    dst_pos: usize,
) {
    for i in 0..pixel_count(num_pixels) {
        let argb = data[src_pos + i];
        let green = low_int8(argb >> 8);
        let red = argb >> 16;
        let mut new_red: i32 = channel(red & 0xff);
        let mut new_blue: i32 = channel(argb & 0xff);
        new_red += color_transform_delta(m.green_to_red.cast_signed(), green);
        new_red &= 0xff;
        new_blue += color_transform_delta(m.green_to_blue.cast_signed(), green);
        new_blue += color_transform_delta(
            m.red_to_blue.cast_signed(),
            low_int8(new_red.cast_unsigned()),
        );
        new_blue &= 0xff;
        data[dst_pos + i] = (argb & 0xff00_ff00) | ((new_red << 16) | new_blue).cast_unsigned();
    }
}

// upstream: src/dsp/lossless.c VP8LTransformColorInverse
/// Calls the portable [`vp8l_transform_color_inverse_c`].
#[inline]
pub fn vp8l_transform_color_inverse(
    m: &VP8LMultipliers,
    data: &mut [u32],
    src_pos: usize,
    num_pixels: i32,
    dst_pos: usize,
) {
    #[cfg(all(feature = "simd", target_arch = "aarch64"))]
    {
        return crate::dsp::lossless_neon::transform_color_inverse(
            m, data, src_pos, num_pixels, dst_pos,
        );
    }
    #[allow(
        unreachable_code,
        reason = "the C path is reached only without a SIMD variant"
    )]
    vp8l_transform_color_inverse_c(m, data, src_pos, num_pixels, dst_pos);
}

// upstream: src/dsp/lossless.c ColorSpaceInverseTransform_C
/// Color space inverse transform of rows `y_start..y_end`, from
/// `data[src_pos..]` into `data[dst_pos..]`.
fn color_space_inverse_transform_c(
    transform: &VP8LTransform,
    y_start: i32,
    y_end: i32,
    data: &mut [u32],
    mut src: usize,
    mut dst: usize,
) {
    let width = transform.xsize;
    let tile_width: i32 = 1 << transform.bits;
    let mask = tile_width - 1;
    let safe_width = width & !mask;
    let remaining_width = width - safe_width;
    let tiles_per_row =
        vp8l_sub_sample_size(width.cast_unsigned(), transform.bits.cast_unsigned()) as usize;
    let mut y = y_start;
    let mut pred_row = (y >> transform.bits).cast_unsigned() as usize * tiles_per_row;

    while y < y_end {
        let mut pred = pred_row;
        let mut m = VP8LMultipliers::default();
        let src_safe_end = src + position(safe_width);
        let src_end = src + position(width);
        while src < src_safe_end {
            color_code_to_multipliers(transform.data[pred], &mut m);
            pred += 1;
            vp8l_transform_color_inverse(&m, data, src, tile_width, dst);
            src += position(tile_width);
            dst += position(tile_width);
        }
        if src < src_end {
            // Left-overs using C-version.
            color_code_to_multipliers(transform.data[pred], &mut m);
            vp8l_transform_color_inverse(&m, data, src, remaining_width, dst);
            src += position(remaining_width);
            dst += position(remaining_width);
        }
        y += 1;
        if (y & mask) == 0 {
            pred_row += tiles_per_row;
        }
    }
}

// Separate out pixels packed together using pixel-bundling.
// We define two methods for ARGB data (uint32_t) and alpha-only data (uint8_t).

// upstream: src/dsp/lossless.c MapARGB_C
/// Maps `width` pixels of each row `y_start..y_end` through `color_map`, from
/// `data[src..]` into `data[dst..]`.
fn map_argb_c(
    data: &mut [u32],
    mut src: usize,
    color_map: &[u32],
    mut dst: usize,
    y_start: i32,
    y_end: i32,
    width: i32,
) {
    for _ in y_start..y_end {
        for _ in 0..pixel_count(width) {
            data[dst] = vp8_get_argb_value(color_map[vp8_get_argb_index(data[src]) as usize]);
            dst += 1;
            src += 1;
        }
    }
}

// upstream: src/dsp/lossless.c MapAlpha_C
/// Maps `width` pixels of each row `y_start..y_end` through `color_map`, from
/// `src` into `dst`.
fn map_alpha_c(
    src: &[u8],
    color_map: &[u32],
    dst: &mut [u8],
    y_start: i32,
    y_end: i32,
    width: i32,
) {
    let mut s = 0;
    let mut d = 0;
    for _ in y_start..y_end {
        for _ in 0..pixel_count(width) {
            dst[d] = vp8_get_alpha_value(color_map[usize::from(vp8_get_alpha_index(src[s]))]);
            d += 1;
            s += 1;
        }
    }
}

// upstream: src/dsp/lossless.c VP8LMapColor32b
/// Calls the portable `MapARGB_C`.
#[inline]
pub fn vp8l_map_color32b(
    data: &mut [u32],
    src: usize,
    color_map: &[u32],
    dst: usize,
    y_start: i32,
    y_end: i32,
    width: i32,
) {
    map_argb_c(data, src, color_map, dst, y_start, y_end, width);
}

// upstream: src/dsp/lossless.c VP8LMapColor8b
/// Calls the portable `MapAlpha_C`.
#[inline]
pub fn vp8l_map_color8b(
    src: &[u8],
    color_map: &[u32],
    dst: &mut [u8],
    y_start: i32,
    y_end: i32,
    width: i32,
) {
    map_alpha_c(src, color_map, dst, y_start, y_end, width);
}

// upstream: src/dsp/lossless.c ColorIndexInverseTransform_C
/// The color-indexing inverse transform of rows `y_start..y_end` for ARGB
/// data, from `data[src..]` into `data[dst..]` (the `COLOR_INDEX_INVERSE`
/// macro with `uint32_t`).
fn color_index_inverse_transform_c(
    transform: &VP8LTransform,
    y_start: i32,
    y_end: i32,
    data: &mut [u32],
    mut src: usize,
    mut dst: usize,
) {
    let bits_per_pixel = 8 >> transform.bits;
    let width = transform.xsize;
    let color_map = &transform.data[..];
    if bits_per_pixel < 8 {
        let pixels_per_byte = 1 << transform.bits;
        let count_mask = pixels_per_byte - 1;
        let bit_mask: u32 = ((1_i32 << bits_per_pixel) - 1).cast_unsigned();
        for _ in y_start..y_end {
            let mut packed_pixels: u32 = 0;
            for x in 0..width {
                // We need to load fresh 'packed_pixels' once every
                // 'pixels_per_byte' increments of x. Fortunately, pixels_per_byte
                // is a power of 2, so can just use a mask for that, instead of
                // decrementing a counter.
                if (x & count_mask) == 0 {
                    packed_pixels = vp8_get_argb_index(data[src]);
                    src += 1;
                }
                data[dst] = vp8_get_argb_value(color_map[(packed_pixels & bit_mask) as usize]);
                dst += 1;
                packed_pixels >>= bits_per_pixel;
            }
        }
    } else {
        vp8l_map_color32b(data, src, color_map, dst, y_start, y_end, width);
    }
}

// upstream: src/dsp/lossless.c VP8LColorIndexInverseTransformAlpha
/// Similar to the static method `ColorIndexInverseTransform()` that is part of
/// lossless.c, but used only for alpha decoding. It takes `uint8_t` (rather
/// than `uint32_t`) arguments for `src` and `dst` (the `COLOR_INDEX_INVERSE`
/// macro with `uint8_t`).
pub fn vp8l_color_index_inverse_transform_alpha(
    transform: &VP8LTransform,
    y_start: i32,
    y_end: i32,
    src: &[u8],
    dst: &mut [u8],
) {
    let bits_per_pixel = 8 >> transform.bits;
    let width = transform.xsize;
    let color_map = &transform.data[..];
    if bits_per_pixel < 8 {
        let pixels_per_byte = 1 << transform.bits;
        let count_mask = pixels_per_byte - 1;
        let bit_mask: u32 = ((1_i32 << bits_per_pixel) - 1).cast_unsigned();
        let mut s = 0;
        let mut d = 0;
        for _ in y_start..y_end {
            let mut packed_pixels: u32 = 0;
            for x in 0..width {
                // We need to load fresh 'packed_pixels' once every
                // 'pixels_per_byte' increments of x. Fortunately, pixels_per_byte
                // is a power of 2, so can just use a mask for that, instead of
                // decrementing a counter.
                if (x & count_mask) == 0 {
                    packed_pixels = u32::from(vp8_get_alpha_index(src[s]));
                    s += 1;
                }
                dst[d] = vp8_get_alpha_value(color_map[(packed_pixels & bit_mask) as usize]);
                d += 1;
                packed_pixels >>= bits_per_pixel;
            }
        }
    } else {
        vp8l_map_color8b(src, color_map, dst, y_start, y_end, width);
    }
}

// upstream: src/dsp/lossless.c VP8LInverseTransform
/// Performs inverse transform of data given transform information, start and
/// end rows. Transform will be applied to rows [`row_start`, `row_end`[.
/// The `in_pos` and `out_pos` positions refer to source and destination data
/// in `data`, corresponding to the intermediate row (`row_start`). They may be
/// equal; the predictor reads the row above `out_pos` when `row_start > 0`.
///
/// # Panics
///
/// Panics when a row lies outside `data` or a transform reads outside its
/// `data`, where C reads or writes outside the buffers.
pub fn vp8l_inverse_transform(
    transform: &VP8LTransform,
    row_start: i32,
    row_end: i32,
    data: &mut [u32],
    in_pos: usize,
    out_pos: usize,
) {
    let width = transform.xsize;
    debug_assert!(row_start < row_end);
    debug_assert!(row_end <= transform.ysize);
    match transform.r#type {
        VP8LImageTransformType::SUBTRACT_GREEN_TRANSFORM => {
            vp8l_add_green_to_blue_and_red(data, in_pos, (row_end - row_start) * width, out_pos);
        }
        VP8LImageTransformType::PREDICTOR_TRANSFORM => {
            predictor_inverse_transform_c(transform, row_start, row_end, data, in_pos, out_pos);
            if row_end != transform.ysize {
                // The last predicted row in this iteration will be the top-pred row
                // for the first row in next iteration.
                let row = position(width);
                let last = out_pos + position((row_end - row_start - 1) * width);
                data.copy_within(last..last + row, out_pos - row);
            }
        }
        VP8LImageTransformType::CROSS_COLOR_TRANSFORM => {
            color_space_inverse_transform_c(transform, row_start, row_end, data, in_pos, out_pos);
        }
        VP8LImageTransformType::COLOR_INDEXING_TRANSFORM => {
            if in_pos == out_pos && transform.bits > 0 {
                // Move packed pixels to the end of unpacked region, so that unpacking
                // can occur seamlessly.
                // Also, note that this is the only transform that applies on
                // the effective width of VP8LSubSampleSize(xsize, bits). All other
                // transforms work on effective width of 'xsize'.
                let out_stride = position((row_end - row_start) * width);
                let in_stride = position(row_end - row_start)
                    * vp8l_sub_sample_size(
                        transform.xsize.cast_unsigned(),
                        transform.bits.cast_unsigned(),
                    ) as usize;
                let src = out_pos + out_stride - in_stride;
                data.copy_within(out_pos..out_pos + in_stride, src);
                color_index_inverse_transform_c(transform, row_start, row_end, data, src, out_pos);
            } else {
                color_index_inverse_transform_c(
                    transform, row_start, row_end, data, in_pos, out_pos,
                );
            }
        }
        _ => {}
    }
}

//------------------------------------------------------------------------------
// Color space conversion.

// upstream: src/dsp/lossless.c is_big_endian
#[must_use]
#[inline]
fn is_big_endian() -> bool {
    cfg!(target_endian = "big")
}

/// The byte `(argb >> shift) & 0xff`.
#[inline]
fn byte_at(argb: u32, shift: u32) -> u8 {
    ((argb >> shift) & 0xff) as u8
}

// upstream: src/dsp/lossless.c VP8LConvertBGRAToRGB_C
pub fn vp8l_convert_bgra_to_rgb_c(src: &[u32], num_pixels: i32, dst: &mut [u8]) {
    for (&argb, out) in src[..pixel_count(num_pixels)]
        .iter()
        .zip(dst.as_chunks_mut::<3>().0)
    {
        out[0] = byte_at(argb, 16);
        out[1] = byte_at(argb, 8);
        out[2] = byte_at(argb, 0);
    }
}

// upstream: src/dsp/lossless.c VP8LConvertBGRAToRGBA_C
pub fn vp8l_convert_bgra_to_rgba_c(src: &[u32], num_pixels: i32, dst: &mut [u8]) {
    for (&argb, out) in src[..pixel_count(num_pixels)]
        .iter()
        .zip(dst.as_chunks_mut::<4>().0)
    {
        out[0] = byte_at(argb, 16);
        out[1] = byte_at(argb, 8);
        out[2] = byte_at(argb, 0);
        out[3] = byte_at(argb, 24);
    }
}

// upstream: src/dsp/lossless.c VP8LConvertBGRAToRGBA4444_C
/// `WEBP_SWAP_16BIT_CSP` is 0 in the reference configuration, so the bytes are
/// stored as `rg`, `ba`.
pub fn vp8l_convert_bgra_to_rgba4444_c(src: &[u32], num_pixels: i32, dst: &mut [u8]) {
    for (&argb, out) in src[..pixel_count(num_pixels)]
        .iter()
        .zip(dst.as_chunks_mut::<2>().0)
    {
        let rg = byte_at(argb, 16) & 0xf0 | byte_at(argb, 12) & 0xf;
        let ba = byte_at(argb, 0) & 0xf0 | byte_at(argb, 28) & 0xf;
        out[0] = rg;
        out[1] = ba;
    }
}

// upstream: src/dsp/lossless.c VP8LConvertBGRAToRGB565_C
/// `WEBP_SWAP_16BIT_CSP` is 0 in the reference configuration, so the bytes are
/// stored as `rg`, `gb`.
pub fn vp8l_convert_bgra_to_rgb565_c(src: &[u32], num_pixels: i32, dst: &mut [u8]) {
    for (&argb, out) in src[..pixel_count(num_pixels)]
        .iter()
        .zip(dst.as_chunks_mut::<2>().0)
    {
        let rg = byte_at(argb, 16) & 0xf8 | byte_at(argb, 13) & 0x7;
        let gb = byte_at(argb, 5) & 0xe0 | byte_at(argb, 3) & 0x1f;
        out[0] = rg;
        out[1] = gb;
    }
}

// upstream: src/dsp/lossless.c VP8LConvertBGRAToBGR_C
pub fn vp8l_convert_bgra_to_bgr_c(src: &[u32], num_pixels: i32, dst: &mut [u8]) {
    for (&argb, out) in src[..pixel_count(num_pixels)]
        .iter()
        .zip(dst.as_chunks_mut::<3>().0)
    {
        out[0] = byte_at(argb, 0);
        out[1] = byte_at(argb, 8);
        out[2] = byte_at(argb, 16);
    }
}

// upstream: src/dsp/lossless.c VP8LConvertBGRAToRGB
/// Calls the portable [`vp8l_convert_bgra_to_rgb_c`].
#[inline]
pub fn vp8l_convert_bgra_to_rgb(src: &[u32], num_pixels: i32, dst: &mut [u8]) {
    #[cfg(all(feature = "simd", target_arch = "aarch64"))]
    {
        return crate::dsp::lossless_neon::convert_bgra_to_rgb(src, num_pixels, dst);
    }
    #[allow(
        unreachable_code,
        reason = "the C path is reached only without a SIMD variant"
    )]
    vp8l_convert_bgra_to_rgb_c(src, num_pixels, dst);
}

// upstream: src/dsp/lossless.c VP8LConvertBGRAToRGBA
/// Calls the portable [`vp8l_convert_bgra_to_rgba_c`].
#[inline]
pub fn vp8l_convert_bgra_to_rgba(src: &[u32], num_pixels: i32, dst: &mut [u8]) {
    #[cfg(all(feature = "simd", target_arch = "aarch64"))]
    {
        return crate::dsp::lossless_neon::convert_bgra_to_rgba(src, num_pixels, dst);
    }
    #[allow(
        unreachable_code,
        reason = "the C path is reached only without a SIMD variant"
    )]
    vp8l_convert_bgra_to_rgba_c(src, num_pixels, dst);
}

// upstream: src/dsp/lossless.c VP8LConvertBGRAToRGBA4444
/// Calls the portable [`vp8l_convert_bgra_to_rgba4444_c`].
#[inline]
pub fn vp8l_convert_bgra_to_rgba4444(src: &[u32], num_pixels: i32, dst: &mut [u8]) {
    vp8l_convert_bgra_to_rgba4444_c(src, num_pixels, dst);
}

// upstream: src/dsp/lossless.c VP8LConvertBGRAToRGB565
/// Calls the portable [`vp8l_convert_bgra_to_rgb565_c`].
#[inline]
pub fn vp8l_convert_bgra_to_rgb565(src: &[u32], num_pixels: i32, dst: &mut [u8]) {
    vp8l_convert_bgra_to_rgb565_c(src, num_pixels, dst);
}

// upstream: src/dsp/lossless.c VP8LConvertBGRAToBGR
/// Calls the portable [`vp8l_convert_bgra_to_bgr_c`].
#[inline]
pub fn vp8l_convert_bgra_to_bgr(src: &[u32], num_pixels: i32, dst: &mut [u8]) {
    #[cfg(all(feature = "simd", target_arch = "aarch64"))]
    {
        return crate::dsp::lossless_neon::convert_bgra_to_bgr(src, num_pixels, dst);
    }
    #[allow(
        unreachable_code,
        reason = "the C path is reached only without a SIMD variant"
    )]
    vp8l_convert_bgra_to_bgr_c(src, num_pixels, dst);
}

// upstream: src/dsp/lossless.c CopyOrSwap
/// Stores each pixel in host byte order, byte-swapped when the host's byte
/// order is `swap_on_big_endian`.
fn copy_or_swap(src: &[u32], num_pixels: i32, dst: &mut [u8], swap_on_big_endian: bool) {
    let src = &src[..pixel_count(num_pixels)];
    if is_big_endian() == swap_on_big_endian {
        for (&argb, out) in src.iter().zip(dst.as_chunks_mut::<4>().0) {
            webp_uint32_to_mem(out, b_swap32(argb));
        }
    } else {
        for (&argb, out) in src.iter().zip(dst.as_chunks_mut::<4>().0) {
            out.copy_from_slice(&argb.to_ne_bytes());
        }
    }
}

// upstream: src/dsp/lossless.c VP8LConvertFromBGRA
/// Converts from BGRA to other color spaces.
///
/// C asserts for a mode without a conversion (`MODE_YUV` and above); this
/// debug-asserts and writes nothing.
pub fn vp8l_convert_from_bgra(
    in_data: &[u32],
    num_pixels: i32,
    out_colorspace: WEBP_CSP_MODE,
    rgba: &mut [u8],
) {
    match out_colorspace {
        WEBP_CSP_MODE::MODE_RGB => vp8l_convert_bgra_to_rgb(in_data, num_pixels, rgba),
        WEBP_CSP_MODE::MODE_RGBA => vp8l_convert_bgra_to_rgba(in_data, num_pixels, rgba),
        WEBP_CSP_MODE::MODE_rgbA => {
            vp8l_convert_bgra_to_rgba(in_data, num_pixels, rgba);
            webp_apply_alpha_multiply(rgba, false, num_pixels, 1, 0);
        }
        WEBP_CSP_MODE::MODE_BGR => vp8l_convert_bgra_to_bgr(in_data, num_pixels, rgba),
        WEBP_CSP_MODE::MODE_BGRA => copy_or_swap(in_data, num_pixels, rgba, true),
        WEBP_CSP_MODE::MODE_bgrA => {
            copy_or_swap(in_data, num_pixels, rgba, true);
            webp_apply_alpha_multiply(rgba, false, num_pixels, 1, 0);
        }
        WEBP_CSP_MODE::MODE_ARGB => copy_or_swap(in_data, num_pixels, rgba, false),
        WEBP_CSP_MODE::MODE_Argb => {
            copy_or_swap(in_data, num_pixels, rgba, false);
            webp_apply_alpha_multiply(rgba, true, num_pixels, 1, 0);
        }
        WEBP_CSP_MODE::MODE_RGBA_4444 => vp8l_convert_bgra_to_rgba4444(in_data, num_pixels, rgba),
        WEBP_CSP_MODE::MODE_rgbA_4444 => {
            vp8l_convert_bgra_to_rgba4444(in_data, num_pixels, rgba);
            webp_apply_alpha_multiply4444(rgba, num_pixels, 1, 0);
        }
        WEBP_CSP_MODE::MODE_RGB_565 => vp8l_convert_bgra_to_rgb565(in_data, num_pixels, rgba),
        _ => debug_assert!(false, "Code flow should not reach here."),
    }
}
