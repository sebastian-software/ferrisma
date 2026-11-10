// libwebp v1.6.0, src/enc/predictor_enc.c:
//
// Copyright 2016 Google Inc. All Rights Reserved.
//
// Use of this source code is governed by a BSD-style license
// that can be found in the COPYING file in the root of the source
// tree. An additional intellectual property rights grant can be found
// in the file PATENTS. All contributing project authors may
// be found in the AUTHORS file in the root of the source tree.
// -----------------------------------------------------------------------------
//
// Image transform methods for lossless encoder.
//
// Authors: Vikas Arora (vikaas.arora@gmail.com)
//          Jyrki Alakuijala (jyrki@google.com)
//          Urvang Joshi (urvang@google.com)
//          Vincent Rabaud (vrabaud@google.com)

//! Translation of `src/enc/predictor_enc.c` from libwebp v1.6.0.
//!
//! Work package: WP11 (VP8L encoder).
//!
//! Only the `WEBP_NEAR_LOSSLESS == 1` branches are translated, as the
//! reference configuration enables near-lossless.
//!
//! # Upstream behavior preserved
//!
//! Behavior of the C code that a reader might take for a defect, which the
//! translation keeps:
//!
//! 1. **`VP8LResidualImage` returns 0 without recording an error** when its
//!    modes cannot be allocated, and `EncodeStreamHook` (`src/enc/vp8l_enc.rs`)
//!    then returns success because it tests only `picture->error_code`;
//!    `VP8LEncodeImage` goes on to write the partial stream. The Rust code
//!    keeps this (not reachable in the tests).
//!
//! # Excluded undefined behavior
//!
//! Inputs that are undefined behavior (or assertion failures) in C and are
//! therefore excluded from the oracle comparison (PORTING rule 8):
//!
//! 1. `VP8LResidualImage` and `VP8LColorSpaceTransform` with bits outside 2 to
//!    9; see `src/enc/near_lossless_enc.rs`, `# Excluded undefined behavior`,
//!    item 1.
//!
//! # Checks not applicable
//!
//! These checks do not apply, or cannot be made, with the reason:
//!
//! 1. **Big-endian hosts.** The `max_diffs` bytes of the predictor scratch are
//!    written by `MaxDiffsForRow` and read by `GetResidual` only, both in
//!    native byte order, so the byte order cannot change a result; the
//!    development host is little-endian.

use crate::dsp::lossless::{VP8LMultipliers, vp8l_predictors};
use crate::dsp::lossless_common::{
    LOG_2_PRECISION_BITS, WEBP_INT64_MAX, div_round, vp8l_add_pixels, vp8l_near_lossless_bits,
    vp8l_sub_pixels, vp8l_sub_sample_size,
};
use crate::dsp::lossless_enc::{
    vp8l_add_vector_eq, vp8l_collect_color_blue_transforms, vp8l_collect_color_red_transforms,
    vp8l_combined_shannon_entropy, vp8l_predictors_sub, vp8l_shannon_entropy, vp8l_transform_color,
};
use crate::enc::backward_references_enc::to_index;
use crate::enc::vp8li_enc::MAX_TRANSFORM_BITS;
use crate::enc::webp_enc::webp_report_progress;
use crate::utils::utils::{webp_safe_calloc, webp_safe_malloc};
use crate::webp::encode::{WebPEncodingError, WebPPicture};
use crate::webp::format_constants::ARGB_BLACK;

// upstream: src/enc/predictor_enc.c HISTO_SIZE
const HISTO_SIZE: i32 = 4 * 256;
// upstream: src/enc/predictor_enc.c kSpatialPredictorBias
const SPATIAL_PREDICTOR_BIAS: i64 = 15 << LOG_2_PRECISION_BITS;
// upstream: src/enc/predictor_enc.c kPredLowEffort
const PRED_LOW_EFFORT: i32 = 11;
// upstream: src/enc/predictor_enc.c kMaskAlpha
const MASK_ALPHA: u32 = 0xff00_0000;
// upstream: src/enc/predictor_enc.c kNumPredModes
const NUM_PRED_MODES: i32 = 14;

// upstream: src/enc/predictor_enc.c GetMin
/// Mostly used to reduce code size + readability
#[inline]
fn get_min(a: i32, b: i32) -> i32 {
    if a > b { b } else { a }
}

// upstream: src/enc/predictor_enc.c GetMax
#[inline]
fn get_max(a: i32, b: i32) -> i32 {
    if a < b { b } else { a }
}

/// The byte `i` of a run of words, as a C `uint8_t*` view of `uint32_t` memory
/// addresses it (native byte order). The scratch rows of the predictor keep
/// their `max_diffs` bytes behind the two pixel rows, in the same words.
#[inline]
fn byte_at(words: &[u32], i: usize) -> u8 {
    words[i / 4].to_ne_bytes()[i % 4]
}

/// Writes the byte `i` of a run of words; see [`byte_at`].
#[inline]
fn set_byte_at(words: &mut [u32], i: usize, value: u8) {
    let mut bytes = words[i / 4].to_ne_bytes();
    bytes[i % 4] = value;
    words[i / 4] = u32::from_ne_bytes(bytes);
}

//------------------------------------------------------------------------------
// Methods to calculate Entropy (Shannon).

// upstream: src/enc/predictor_enc.c PredictionCostBias
/// Compute a bias for prediction entropy using a global heuristic to favor
/// values closer to 0. Hence the final negative sign.
/// `exp_val` has a scaling factor of 1/100. The `uint64_t` arithmetic wraps and
/// converts to the `int64_t` arguments of `DivRound`, as in C.
fn prediction_cost_bias(counts: &[u32], weight_0: u64, mut exp_val: u64) -> i64 {
    const SIGNIFICANT_SYMBOLS: usize = 256 >> 4;
    const EXP_DECAY_FACTOR: u64 = 6; // has a scaling factor of 1/10
    let mut bits: u64 = weight_0.wrapping_mul(u64::from(counts[0])) << LOG_2_PRECISION_BITS;
    exp_val <<= LOG_2_PRECISION_BITS;
    for i in 1..SIGNIFICANT_SYMBOLS {
        let pair = counts[i].wrapping_add(counts[256 - i]);
        bits = bits.wrapping_add(
            div_round(exp_val.wrapping_mul(u64::from(pair)).cast_signed(), 100).cast_unsigned(),
        );
        exp_val =
            div_round(EXP_DECAY_FACTOR.wrapping_mul(exp_val).cast_signed(), 10).cast_unsigned();
    }
    -div_round(bits.cast_signed(), 10)
}

// upstream: src/enc/predictor_enc.c PredictionCostSpatialHistogram
fn prediction_cost_spatial_histogram(
    accumulated: &[u32],
    tile: &[u32],
    mode: i32,
    left_mode: i32,
    above_mode: i32,
) -> i64 {
    let mut retval: i64 = 0;
    for i in 0..4 {
        const EXP_VALUE: u64 = 94;
        let tile_i = &tile[i * 256..];
        retval += prediction_cost_bias(tile_i, 1, EXP_VALUE);
        // Compute the new cost if 'tile' is added to 'accumulate' but also add the
        // cost of the current histogram to guide the spatial predictor selection.
        // Basically, favor low entropy, locally and globally.
        retval += vp8l_combined_shannon_entropy(tile_i, &accumulated[i * 256..]).cast_signed();
    }
    // Favor keeping the areas locally similar.
    if mode == left_mode {
        retval -= SPATIAL_PREDICTOR_BIAS;
    }
    if mode == above_mode {
        retval -= SPATIAL_PREDICTOR_BIAS;
    }
    retval
}

// upstream: src/enc/predictor_enc.c UpdateHisto
#[inline]
fn update_histo(histo_argb: &mut [u32], argb: u32) {
    histo_argb[(argb >> 24) as usize] += 1;
    histo_argb[256 + ((argb >> 16) & 0xff) as usize] += 1;
    histo_argb[2 * 256 + ((argb >> 8) & 0xff) as usize] += 1;
    histo_argb[3 * 256 + (argb & 0xff) as usize] += 1;
}

//------------------------------------------------------------------------------
// Spatial transform functions.

// upstream: src/enc/predictor_enc.c PredictBatch
/// `current` and `upper` are positions of two rows in `rows`; the residuals go
/// to `out`. C passes `NULL` as the row above to the predictors 0 and 1, which
/// do not read it; the position 0 stands for it.
#[inline]
#[expect(
    clippy::too_many_arguments,
    reason = "the C signature of PredictBatch has seven parameters, and its two row pointers are positions in one buffer"
)]
fn predict_batch(
    mode: i32,
    mut x_start: i32,
    y: i32,
    mut num_pixels: i32,
    rows: &[u32],
    current: usize,
    upper: usize,
    out: &mut [u32],
) {
    let mut out = out;
    if x_start == 0 {
        if y == 0 {
            // ARGB_BLACK.
            vp8l_predictors_sub(0, rows, current, 0, 1, out);
        } else {
            // Top one.
            vp8l_predictors_sub(2, rows, current, upper, 1, out);
        }
        x_start += 1;
        out = &mut out[1..];
        num_pixels -= 1;
    }
    let x = to_index(x_start);
    if y == 0 {
        // Left one.
        vp8l_predictors_sub(1, rows, current + x, 0, num_pixels, out);
    } else {
        vp8l_predictors_sub(
            to_index(mode),
            rows,
            current + x,
            upper + x,
            num_pixels,
            out,
        );
    }
}

// upstream: src/enc/predictor_enc.c MaxDiffBetweenPixels
#[expect(clippy::cast_possible_wrap, reason = "the channels are below 256")]
fn max_diff_between_pixels(p1: u32, p2: u32) -> i32 {
    let diff_a = ((p1 >> 24) as i32 - (p2 >> 24) as i32).abs();
    let diff_r = (((p1 >> 16) & 0xff) as i32 - ((p2 >> 16) & 0xff) as i32).abs();
    let diff_g = (((p1 >> 8) & 0xff) as i32 - ((p2 >> 8) & 0xff) as i32).abs();
    let diff_b = ((p1 & 0xff) as i32 - (p2 & 0xff) as i32).abs();
    get_max(get_max(diff_a, diff_r), get_max(diff_g, diff_b))
}

// upstream: src/enc/predictor_enc.c MaxDiffAroundPixel
fn max_diff_around_pixel(current: u32, up: u32, down: u32, left: u32, right: u32) -> i32 {
    let diff_up = max_diff_between_pixels(current, up);
    let diff_down = max_diff_between_pixels(current, down);
    let diff_left = max_diff_between_pixels(current, left);
    let diff_right = max_diff_between_pixels(current, right);
    get_max(get_max(diff_up, diff_down), get_max(diff_left, diff_right))
}

// upstream: src/enc/predictor_enc.c AddGreenToBlueAndRed
fn add_green_to_blue_and_red(argb: u32) -> u32 {
    let green = (argb >> 8) & 0xff;
    let mut red_blue = argb & 0x00ff_00ff;
    red_blue = red_blue.wrapping_add((green << 16) | green);
    red_blue &= 0x00ff_00ff;
    (argb & 0xff00_ff00) | red_blue
}

// upstream: src/enc/predictor_enc.c MaxDiffsForRow
/// `argb[argb_pos]` is the C `argb[0]`, and the rows above and below are read at
/// `argb_pos - stride` and `argb_pos + stride`. The differences are written as
/// bytes from byte `max_diffs_pos` of `max_diffs`.
#[expect(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    reason = "the C conversion of the difference to uint8_t"
)]
fn max_diffs_for_row(
    width: i32,
    stride: i32,
    argb: &[u32],
    argb_pos: usize,
    max_diffs: &mut [u32],
    max_diffs_pos: usize,
    used_subtract_green: bool,
) {
    if width <= 2 {
        return;
    }
    let stride = to_index(stride);
    let mut current = argb[argb_pos];
    let mut right = argb[argb_pos + 1];
    if used_subtract_green {
        current = add_green_to_blue_and_red(current);
        right = add_green_to_blue_and_red(right);
    }
    // max_diffs[0] and max_diffs[width - 1] are never used.
    for x in 1..to_index(width) - 1 {
        let mut up = argb[argb_pos + x - stride];
        let mut down = argb[argb_pos + stride + x];
        let left = current;
        current = right;
        right = argb[argb_pos + x + 1];
        if used_subtract_green {
            up = add_green_to_blue_and_red(up);
            down = add_green_to_blue_and_red(down);
            right = add_green_to_blue_and_red(right);
        }
        set_byte_at(
            max_diffs,
            max_diffs_pos + x,
            max_diff_around_pixel(current, up, down, left, right) as u8,
        );
    }
}

// upstream: src/enc/predictor_enc.c NearLosslessComponent
/// Quantize the difference between the actual component value and its
/// prediction to a multiple of quantization, working modulo 256, taking care
/// not to cross a boundary (inclusive upper limit).
#[expect(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    reason = "the C conversions of the result to uint8_t"
)]
fn near_lossless_component(value: u8, predict: u8, boundary: u8, quantization: i32) -> u8 {
    let (value, predict, boundary) = (i32::from(value), i32::from(predict), i32::from(boundary));
    let residual = (value - predict) & 0xff;
    let boundary_residual = (boundary - predict) & 0xff;
    let lower = residual & !(quantization - 1);
    let upper = lower + quantization;
    // Resolve ties towards a value closer to the prediction (i.e. towards lower
    // if value comes after prediction and towards upper otherwise).
    let bias = i32::from(((boundary - value) & 0xff) < boundary_residual);
    if residual - lower < upper - residual + bias {
        // lower is closer to residual than upper.
        if residual > boundary_residual && lower <= boundary_residual {
            // Halve quantization step to avoid crossing boundary. This midpoint is
            // on the same side of boundary as residual because midpoint >= residual
            // (since lower is closer than upper) and residual is above the boundary.
            return (lower + (quantization >> 1)) as u8;
        }
        lower as u8
    } else {
        // upper is closer to residual than lower.
        if residual <= boundary_residual && upper > boundary_residual {
            // Halve quantization step to avoid crossing boundary. This midpoint is
            // on the same side of boundary as residual because midpoint <= residual
            // (since upper is closer than lower) and residual is below the boundary.
            return (lower + (quantization >> 1)) as u8;
        }
        (upper & 0xff) as u8
    }
}

// upstream: src/enc/predictor_enc.c NearLosslessDiff
#[inline]
#[expect(
    clippy::cast_sign_loss,
    reason = "the C conversion of the masked difference to uint8_t"
)]
fn near_lossless_diff(a: u8, b: u8) -> u8 {
    ((i32::from(a) - i32::from(b)) & 0xff) as u8
}

/// The C conversion of a channel, already below 256, to `uint8_t`.
#[inline]
#[expect(clippy::cast_possible_truncation, reason = "the channel is below 256")]
fn channel(value: u32) -> u8 {
    value as u8
}

// upstream: src/enc/predictor_enc.c NearLossless
/// Quantize every component of the difference between the actual pixel value
/// and its prediction to a multiple of a quantization (a power of 2, not larger
/// than `max_quantization` which is a power of 2, smaller than `max_diff`). Take
/// care if value and predict have undergone subtract green, which means that red
/// and blue are represented as offsets from green.
fn near_lossless(
    value: u32,
    predict: u32,
    max_quantization: i32,
    max_diff: i32,
    used_subtract_green: bool,
) -> u32 {
    let mut new_green: u8 = 0;
    let mut green_diff: u8 = 0;
    if max_diff <= 2 {
        return vp8l_sub_pixels(value, predict);
    }
    let mut quantization = max_quantization;
    while quantization >= max_diff {
        quantization >>= 1;
    }
    let a = if (value >> 24) == 0 || (value >> 24) == 0xff {
        // Preserve transparency of fully transparent or fully opaque pixels.
        near_lossless_diff(
            channel((value >> 24) & 0xff),
            channel((predict >> 24) & 0xff),
        )
    } else {
        near_lossless_component(
            channel(value >> 24),
            channel(predict >> 24),
            0xff,
            quantization,
        )
    };
    let g = near_lossless_component(
        channel((value >> 8) & 0xff),
        channel((predict >> 8) & 0xff),
        0xff,
        quantization,
    );
    if used_subtract_green {
        // The green offset will be added to red and blue components during decoding
        // to obtain the actual red and blue values.
        new_green = channel(((predict >> 8).wrapping_add(u32::from(g))) & 0xff);
        // The amount by which green has been adjusted during quantization. It is
        // subtracted from red and blue for compensation, to avoid accumulating two
        // quantization errors in them.
        green_diff = near_lossless_diff(new_green, channel((value >> 8) & 0xff));
    }
    let r = near_lossless_component(
        near_lossless_diff(channel((value >> 16) & 0xff), green_diff),
        channel((predict >> 16) & 0xff),
        0xff - new_green,
        quantization,
    );
    let b = near_lossless_component(
        near_lossless_diff(channel(value & 0xff), green_diff),
        channel(predict & 0xff),
        0xff - new_green,
        quantization,
    );
    (u32::from(a) << 24) | (u32::from(r) << 16) | (u32::from(g) << 8) | u32::from(b)
}

// upstream: src/enc/predictor_enc.c GetResidual
/// Stores the difference between the pixel and its prediction in `out`.
/// In case of a lossy encoding, updates the source image to avoid propagating
/// the deviation further to pixels which depend on the current pixel for their
/// predictions. `upper_row` and `current_row` are positions in `rows`; the
/// `max_diffs` bytes start at byte `max_diffs_pos` of `max_diffs`.
#[inline]
#[expect(
    clippy::too_many_arguments,
    reason = "the C signature of GetResidual has thirteen parameters"
)]
fn get_residual(
    width: i32,
    height: i32,
    rows: &mut [u32],
    upper_row: usize,
    current_row: usize,
    max_diffs: &[u32],
    max_diffs_pos: usize,
    mode: i32,
    x_start: i32,
    x_end: i32,
    y: i32,
    max_quantization: i32,
    exact: bool,
    used_subtract_green: bool,
    out: &mut [u32],
) {
    if exact {
        predict_batch(
            mode,
            x_start,
            y,
            x_end - x_start,
            rows,
            current_row,
            upper_row,
            out,
        );
    } else {
        for x in x_start..x_end {
            let xi = to_index(x);
            let predict = if y == 0 {
                if x == 0 {
                    ARGB_BLACK
                } else {
                    rows[current_row + xi - 1] // Left.
                }
            } else if x == 0 {
                rows[upper_row + xi] // Top.
            } else {
                vp8l_predictors(
                    to_index(mode),
                    rows,
                    current_row + xi - 1,
                    rows,
                    upper_row + xi,
                )
            };
            let mut residual;
            if max_quantization == 1
                || mode == 0
                || y == 0
                || y == height - 1
                || x == 0
                || x == width - 1
            {
                residual = vp8l_sub_pixels(rows[current_row + xi], predict);
            } else {
                residual = near_lossless(
                    rows[current_row + xi],
                    predict,
                    max_quantization,
                    i32::from(byte_at(max_diffs, max_diffs_pos + xi)),
                    used_subtract_green,
                );
                // Update the source image.
                rows[current_row + xi] = vp8l_add_pixels(predict, residual);
                // x is never 0 here so we do not need to update upper_row like below.
            }
            if (rows[current_row + xi] & MASK_ALPHA) == 0 {
                // If alpha is 0, cleanup RGB. We can choose the RGB values of the
                // residual for best compression. The prediction of alpha itself can be
                // non-zero and must be kept though. We choose RGB of the residual to be
                // 0.
                residual &= MASK_ALPHA;
                // Update the source image.
                rows[current_row + xi] = predict & !MASK_ALPHA;
                // The prediction for the rightmost pixel in a row uses the leftmost
                // pixel
                // in that row as its top-right context pixel. Hence if we change the
                // leftmost pixel of current_row, the corresponding change must be
                // applied
                // to upper_row as well where top-right context is being read from.
                if x == 0 && y != 0 {
                    rows[upper_row + to_index(width)] = rows[current_row];
                }
            }
            out[to_index(x - x_start)] = residual;
        }
    }
}

// upstream: src/enc/predictor_enc.c GetHistoArgb
/// Accessors to residual histograms: the position of the histogram in
/// `all_histos`.
#[inline]
fn get_histo_argb(subsampling_index: u32, mode: i32) -> usize {
    (subsampling_index as usize * to_index(NUM_PRED_MODES) + to_index(mode)) * to_index(HISTO_SIZE)
}

// upstream: src/enc/predictor_enc.c GetHistoArgbConst
#[inline]
fn get_histo_argb_const(subsampling_index: u32, mode: i32) -> usize {
    subsampling_index as usize * to_index(NUM_PRED_MODES) * to_index(HISTO_SIZE)
        + to_index(mode) * to_index(HISTO_SIZE)
}

// upstream: src/enc/predictor_enc.c GetAccumulatedHisto
/// Accessors to accumulated residual histogram: its position.
#[inline]
fn get_accumulated_histo(subsampling_index: u32) -> usize {
    subsampling_index as usize * to_index(HISTO_SIZE)
}

// upstream: src/enc/predictor_enc.c GetBestPredictorForTile
/// Find and store the best predictor for a tile at subsampling
/// `subsampling_index`. `all_modes[i]` is the position of the modes of
/// subsampling `i` in `modes_raw`.
#[expect(
    clippy::too_many_arguments,
    reason = "the C signature of GetBestPredictorForTile has eight parameters"
)]
#[expect(
    clippy::cast_possible_wrap,
    clippy::cast_sign_loss,
    reason = "the C conversions of the stored modes and the best mode"
)]
fn get_best_predictor_for_tile(
    all_argb: &[u32],
    subsampling_index: u32,
    tile_x: i32,
    tile_y: i32,
    tiles_per_row: i32,
    all_accumulated_argb: &mut [u32],
    modes_raw: &mut [u32],
    all_modes: &[usize],
    all_pred_histos: &mut [u32],
) {
    let accumulated_argb = get_accumulated_histo(subsampling_index);
    let modes = all_modes[subsampling_index as usize];
    let pred_histos = subsampling_index as usize * to_index(NUM_PRED_MODES);
    // Prediction modes of the left and above neighbor tiles.
    let left_mode = if tile_x > 0 {
        ((modes_raw[modes + to_index(tile_y * tiles_per_row + tile_x - 1)] >> 8) & 0xff) as i32
    } else {
        0xff
    };
    let above_mode = if tile_y > 0 {
        ((modes_raw[modes + to_index((tile_y - 1) * tiles_per_row + tile_x)] >> 8) & 0xff) as i32
    } else {
        0xff
    };
    let mut best_diff = WEBP_INT64_MAX;
    let mut best_mode: u32 = 0;
    let mut best_histo = get_histo_argb_const(/*subsampling_index=*/ 0, best_mode as i32);
    for mode in 0..NUM_PRED_MODES {
        let histo_argb = get_histo_argb_const(subsampling_index, mode);
        let cur_diff = prediction_cost_spatial_histogram(
            &all_accumulated_argb[accumulated_argb..],
            &all_argb[histo_argb..],
            mode,
            left_mode,
            above_mode,
        );

        if cur_diff < best_diff {
            best_histo = histo_argb;
            best_diff = cur_diff;
            best_mode = mode as u32;
        }
    }
    // Update the accumulated histogram.
    vp8l_add_vector_eq(
        &all_argb[best_histo..],
        &mut all_accumulated_argb[accumulated_argb..],
        HISTO_SIZE,
    );
    modes_raw[modes + to_index(tile_y * tiles_per_row + tile_x)] = ARGB_BLACK | (best_mode << 8);
    all_pred_histos[pred_histos + best_mode as usize] += 1;
}

// upstream: src/enc/predictor_enc.c ComputeResidualsForTile
/// Computes the residuals for the different predictors.
/// If `max_quantization` > 1, assumes that near lossless processing will be
/// applied, quantizing residuals to multiples of quantization levels up to
/// `max_quantization` (the actual quantization level depends on smoothness near
/// the given pixel).
#[expect(
    clippy::too_many_arguments,
    reason = "the C signature of ComputeResidualsForTile has twelve parameters"
)]
fn compute_residuals_for_tile(
    width: i32,
    height: i32,
    tile_x: i32,
    tile_y: i32,
    min_bits: i32,
    update_up_to_index: u32,
    all_argb: &mut [u32],
    argb_scratch: &mut [u32],
    argb: &[u32],
    max_quantization: i32,
    exact: bool,
    used_subtract_green: bool,
) {
    let start_x = tile_x << min_bits;
    let start_y = tile_y << min_bits;
    let tile_size = 1 << min_bits;
    let max_y = get_min(tile_size, height - start_y);
    let max_x = get_min(tile_size, width - start_x);
    // Whether there exist columns just outside the tile.
    let have_left = i32::from(start_x > 0);
    // Position and size of the strip covering the tile and adjacent columns if
    // they exist.
    let context_start_x = start_x - have_left;
    let context_width = max_x + have_left + i32::from(max_x < width - start_x);
    // The width of upper_row and current_row is one pixel larger than image width
    // to allow the top right pixel to point to the leftmost pixel of the next row
    // when at the right edge.
    let row_size = to_index(width) + 1;
    let (rows, max_diffs) = argb_scratch.split_at_mut(2 * row_size);
    let mut upper_row: usize = 0;
    let mut current_row: usize = row_size;
    // Need pointers to be able to swap arrays.
    let mut residuals = [0_u32; 1 << MAX_TRANSFORM_BITS];
    let context = to_index(context_start_x);
    debug_assert!(max_x <= (1 << MAX_TRANSFORM_BITS));
    for mode in 0..NUM_PRED_MODES {
        let histo_argb = get_histo_argb(/*subsampling_index=*/ 0, mode);
        if start_y > 0 {
            // Read the row above the tile which will become the first upper_row.
            // Include a pixel to the left if it exists; include a pixel to the right
            // in all cases (wrapping to the leftmost pixel of the next row if it does
            // not exist).
            let len = to_index(max_x + have_left + 1);
            let src = to_index((start_y - 1) * width + context_start_x);
            rows[current_row + context..current_row + context + len]
                .copy_from_slice(&argb[src..src + len]);
        }
        for relative_y in 0..max_y {
            let y = start_y + relative_y;
            std::mem::swap(&mut upper_row, &mut current_row);
            // Read current_row. Include a pixel to the left if it exists; include a
            // pixel to the right in all cases except at the bottom right corner of
            // the image (wrapping to the leftmost pixel of the next row if it does
            // not exist in the current row).
            let len = to_index(max_x + have_left + i32::from(y + 1 < height));
            let src = to_index(y * width + context_start_x);
            rows[current_row + context..current_row + context + len]
                .copy_from_slice(&argb[src..src + len]);
            if max_quantization > 1 && y >= 1 && y + 1 < height {
                max_diffs_for_row(
                    context_width,
                    width,
                    argb,
                    src,
                    max_diffs,
                    context,
                    used_subtract_green,
                );
            }

            get_residual(
                width,
                height,
                rows,
                upper_row,
                current_row,
                max_diffs,
                0,
                mode,
                start_x,
                start_x + max_x,
                y,
                max_quantization,
                exact,
                used_subtract_green,
                &mut residuals,
            );
            for &residual in &residuals[..to_index(max_x)] {
                update_histo(&mut all_argb[histo_argb..], residual);
            }
            if update_up_to_index > 0 {
                for subsampling_index in 1..=update_up_to_index {
                    let super_histo = get_histo_argb(subsampling_index, mode);
                    for &residual in &residuals[..to_index(max_x)] {
                        update_histo(&mut all_argb[super_histo..], residual);
                    }
                }
            }
        }
    }
}

// upstream: src/enc/predictor_enc.c CopyImageWithPrediction
/// Converts pixels of the image to residuals with respect to predictions.
/// If `max_quantization` > 1, applies near lossless processing, quantizing
/// residuals to multiples of quantization levels up to `max_quantization`
/// (the actual quantization level depends on smoothness near the given pixel).
#[expect(
    clippy::too_many_arguments,
    reason = "the C signature of CopyImageWithPrediction has ten parameters"
)]
#[expect(
    clippy::cast_possible_wrap,
    reason = "the C conversion of the stored mode to int"
)]
fn copy_image_with_prediction(
    width: i32,
    height: i32,
    bits: i32,
    modes: &[u32],
    argb_scratch: &mut [u32],
    argb: &mut [u32],
    low_effort: bool,
    max_quantization: i32,
    exact: bool,
    used_subtract_green: bool,
) {
    let tiles_per_row =
        vp8l_sub_sample_size(width.cast_unsigned(), bits.cast_unsigned()).cast_signed();
    // The width of upper_row and current_row is one pixel larger than image width
    // to allow the top right pixel to point to the leftmost pixel of the next row
    // when at the right edge.
    let w = to_index(width);
    let row_size = w + 1;
    let (rows, max_diffs) = argb_scratch.split_at_mut(2 * row_size);
    let mut upper_row: usize = 0;
    let mut current_row: usize = row_size;
    let mut current_max_diffs: usize = 0;
    let mut lower_max_diffs: usize = current_max_diffs + w;

    for y in 0..height {
        std::mem::swap(&mut upper_row, &mut current_row);
        let row = to_index(y) * w;
        let len = w + usize::from(y + 1 < height);
        rows[current_row..current_row + len].copy_from_slice(&argb[row..row + len]);

        if low_effort {
            predict_batch(
                PRED_LOW_EFFORT,
                0,
                y,
                width,
                rows,
                current_row,
                upper_row,
                &mut argb[row..],
            );
        } else {
            if max_quantization > 1 {
                // Compute max_diffs for the lower row now, because that needs the
                // contents of argb for the current row, which we will overwrite with
                // residuals before proceeding with the next row.
                std::mem::swap(&mut current_max_diffs, &mut lower_max_diffs);
                if y + 2 < height {
                    max_diffs_for_row(
                        width,
                        width,
                        argb,
                        row + w,
                        max_diffs,
                        lower_max_diffs,
                        used_subtract_green,
                    );
                }
            }
            let mut x: i32 = 0;
            while x < width {
                let mode = ((modes[to_index((y >> bits) * tiles_per_row + (x >> bits))] >> 8)
                    & 0xff) as i32;
                let mut x_end = x + (1 << bits);
                if x_end > width {
                    x_end = width;
                }
                get_residual(
                    width,
                    height,
                    rows,
                    upper_row,
                    current_row,
                    max_diffs,
                    current_max_diffs,
                    mode,
                    x,
                    x_end,
                    y,
                    max_quantization,
                    exact,
                    used_subtract_green,
                    &mut argb[row + to_index(x)..],
                );
                x = x_end;
            }
        }
    }
}

// upstream: src/enc/predictor_enc.c VP8LOptimizeSampling
/// Checks whether `image` can be subsampled by finding the biggest power of 2
/// squares (defined by `best_bits`) of uniform value it is made out of.
pub fn vp8l_optimize_sampling(
    image: &mut [u32],
    full_width: i32,
    full_height: i32,
    bits: i32,
    max_bits: i32,
    best_bits_out: &mut i32,
) {
    let mut width = to_index(
        vp8l_sub_sample_size(full_width.cast_unsigned(), bits.cast_unsigned()).cast_signed(),
    );
    let mut height = to_index(
        vp8l_sub_sample_size(full_height.cast_unsigned(), bits.cast_unsigned()).cast_signed(),
    );
    let mut best_bits = bits;
    *best_bits_out = bits;
    // Check rows first.
    while best_bits < max_bits {
        let new_square_size = 1_usize << (best_bits + 1 - bits);
        let mut is_good = true;
        let square_size = 1_usize << (best_bits - bits);
        let mut y = 0;
        while y + square_size < height {
            // Check the first lines of consecutive line groups.
            if image[y * width..(y + 1) * width]
                != image[(y + square_size) * width..(y + square_size + 1) * width]
            {
                is_good = false;
                break;
            }
            y += new_square_size;
        }
        if is_good {
            best_bits += 1;
        } else {
            break;
        }
    }
    if best_bits == bits {
        return;
    }

    // Check columns.
    while best_bits > bits {
        let mut is_good = true;
        let square_size = 1_usize << (best_bits - bits);
        let mut y = 0;
        while is_good && y < height {
            let mut x = 0;
            while is_good && x < width {
                for i in (x + 1)..(x + square_size).min(width) {
                    if image[y * width + i] != image[y * width + x] {
                        is_good = false;
                        break;
                    }
                }
                x += square_size;
            }
            y += 1;
        }
        if is_good {
            break;
        }
        best_bits -= 1;
    }
    if best_bits == bits {
        return;
    }

    // Subsample the image.
    let old_width = width;
    let square_size = 1_usize << (best_bits - bits);
    width = to_index(
        vp8l_sub_sample_size(full_width.cast_unsigned(), best_bits.cast_unsigned()).cast_signed(),
    );
    height = to_index(
        vp8l_sub_sample_size(full_height.cast_unsigned(), best_bits.cast_unsigned()).cast_signed(),
    );
    for y in 0..height {
        for x in 0..width {
            image[y * width + x] = image[square_size * (y * old_width + x)];
        }
    }
    *best_bits_out = best_bits;
}

// upstream: src/enc/predictor_enc.c GetBestPredictorsAndSubSampling
/// Computes the best predictor image.
/// Finds the best predictors per tile. Once done, finds the best predictor
/// image sampling.
/// `best_bits` is set to 0 in case of error.
/// The following requires some glossary:
/// - a tile is a square of side 2^`min_bits` pixels.
/// - a super-tile of a tile is a square of side 2^bits pixels with bits in
///   [`min_bits`+1, `max_bits`].
/// - the max-tile of a tile is the square of 2^`max_bits` pixels containing it.
///   If this max-tile crosses the border of an image, it is cropped.
/// - tile, super-tiles and `max_tile` are aligned on powers of 2 in the original
///   image.
/// - coordinates for tile, super-tile, max-tile are respectively named
///   `tile_x`, `super_tile_x`, `max_tile_x` at their bit scale.
/// - in the max-tile, a tile has local coordinates (`local_tile_x`,
///   `local_tile_y`).
///
/// The tiles are processed in the following zigzag order to complete the
/// super-tiles as soon as possible:
///   1  2|  5  6
///   3  4|  7  8
/// --------------
///   9 10| 13 14
///  11 12| 15 16
///
/// When computing the residuals for a tile, the histogram of the above
/// super-tile is updated. If this super-tile is finished, its histogram is used
/// to update the histogram of the next super-tile and so on up to the max-tile.
///
/// `all_modes[i]` is the position in `modes_raw` of the modes of subsampling
/// `i`; `best_mode` receives the subsampling index whose modes C returns as
/// `*best_mode`, or `None` where C leaves `NULL`.
#[expect(
    clippy::too_many_arguments,
    reason = "the C signature of GetBestPredictorsAndSubSampling has fifteen parameters"
)]
#[expect(
    clippy::cast_possible_wrap,
    clippy::cast_sign_loss,
    reason = "the C conversions between the int and uint32_t tile coordinates"
)]
#[expect(
    clippy::too_many_lines,
    reason = "the C function GetBestPredictorsAndSubSampling is translated as one function"
)]
fn get_best_predictors_and_sub_sampling(
    width: i32,
    height: i32,
    min_bits: i32,
    max_bits: i32,
    argb_scratch: &mut [u32],
    argb: &[u32],
    max_quantization: i32,
    exact: bool,
    used_subtract_green: bool,
    pic: &mut WebPPicture<'_>,
    percent_range: i32,
    percent: &mut i32,
    modes_raw: &mut [u32],
    all_modes: &[usize],
    best_bits: &mut i32,
    best_mode: &mut Option<usize>,
) {
    let tiles_per_row = vp8l_sub_sample_size(width.cast_unsigned(), min_bits.cast_unsigned());
    let tiles_per_col = vp8l_sub_sample_size(height.cast_unsigned(), min_bits.cast_unsigned());
    let max_subsampling_index = (max_bits - min_bits) as u32;
    // Compute the needed memory size for residual histograms, accumulated
    // residual histograms and predictor histograms.
    let num_argb = ((max_subsampling_index + 1) * NUM_PRED_MODES as u32 * HISTO_SIZE as u32) as i32;
    let num_accumulated_rgb = ((max_subsampling_index + 1) * HISTO_SIZE as u32) as i32;
    let num_predictors = ((max_subsampling_index + 1) * NUM_PRED_MODES as u32) as i32;
    let raw_data = webp_safe_calloc::<u32>(
        i64::from(num_argb + num_accumulated_rgb + num_predictors).cast_unsigned(),
        4,
    );
    let max_tile_size: u32 = 1 << max_subsampling_index; // in tile size
    let percent_start = *percent;
    // When using the residuals of a tile for its super-tiles, you can either:
    // - use each residual to update the histogram of the super-tile, with a cost
    //   of 4 * (1<<n)^2 increment operations (4 for the number of channels, and
    //   (1<<n)^2 for the number of pixels in the tile)
    // - use the histogram of the tile to update the histogram of the super-tile,
    //   with a cost of HISTO_SIZE (1024)
    // The first method is therefore faster until n==4. 'update_up_to_index'
    // defines the maximum subsampling_index for which the residuals should be
    // individually added to the super-tile histogram.
    let update_up_to_index = (get_max(get_min(4, max_bits), min_bits) - min_bits) as u32;
    // Coordinates in the max-tile in tile units.
    let mut local_tile_x: u32 = 0;
    let mut local_tile_y: u32 = 0;
    let mut max_tile_x: u32 = 0;
    let mut max_tile_y: u32 = 0;
    let mut tile_x: u32 = 0;
    let mut tile_y: u32 = 0;

    *best_bits = 0;
    *best_mode = None;
    let Some(mut raw_data) = raw_data else {
        return;
    };
    let (all_argb, rest) = raw_data.split_at_mut(to_index(num_argb));
    let (all_accumulated_argb, all_pred_histos) = rest.split_at_mut(to_index(num_accumulated_rgb));

    while tile_y < tiles_per_col {
        compute_residuals_for_tile(
            width,
            height,
            tile_x as i32,
            tile_y as i32,
            min_bits,
            update_up_to_index,
            all_argb,
            argb_scratch,
            argb,
            max_quantization,
            exact,
            used_subtract_green,
        );

        // Update all the super-tiles that are complete.
        let mut subsampling_index: u32 = 0;
        loop {
            let super_tile_x = tile_x >> subsampling_index;
            let super_tile_y = tile_y >> subsampling_index;
            let super_tiles_per_row = vp8l_sub_sample_size(
                width.cast_unsigned(),
                (min_bits + subsampling_index as i32).cast_unsigned(),
            );
            get_best_predictor_for_tile(
                all_argb,
                subsampling_index,
                super_tile_x as i32,
                super_tile_y as i32,
                super_tiles_per_row as i32,
                all_accumulated_argb,
                modes_raw,
                all_modes,
                all_pred_histos,
            );
            if subsampling_index == max_subsampling_index {
                break;
            }

            // Update the following super-tile histogram if it has not been updated
            // yet.
            subsampling_index += 1;
            if subsampling_index > update_up_to_index && subsampling_index <= max_subsampling_index
            {
                let (lower, upper) =
                    all_argb.split_at_mut(get_histo_argb(subsampling_index, /*mode=*/ 0));
                vp8l_add_vector_eq(
                    &lower[get_histo_argb_const(subsampling_index - 1, /*mode=*/ 0)..],
                    upper,
                    HISTO_SIZE * NUM_PRED_MODES,
                );
            }
            // Check whether the super-tile is not complete (if the smallest tile
            // is not at the end of a line/column or at the beginning of a super-tile
            // of size (1 << subsampling_index)).
            if !((tile_x == (tiles_per_row - 1)
                || (local_tile_x + 1).is_multiple_of(1 << subsampling_index))
                && (tile_y == (tiles_per_col - 1)
                    || (local_tile_y + 1).is_multiple_of(1 << subsampling_index)))
            {
                subsampling_index -= 1;
                // subsampling_index now is the index of the last finished super-tile.
                break;
            }
        }
        // Reset all the histograms belonging to finished tiles.
        let finished = to_index(HISTO_SIZE * NUM_PRED_MODES) * (subsampling_index as usize + 1);
        all_argb[..finished].fill(0);

        if subsampling_index == max_subsampling_index {
            // If a new max-tile is started.
            if tile_x == (tiles_per_row - 1) {
                max_tile_x = 0;
                max_tile_y += 1;
            } else {
                max_tile_x += 1;
            }
            local_tile_x = 0;
            local_tile_y = 0;
        } else {
            // Proceed with the Z traversal.
            let mut coord_x = local_tile_x >> subsampling_index;
            let mut coord_y = local_tile_y >> subsampling_index;
            if tile_x == (tiles_per_row - 1) && coord_x.is_multiple_of(2) {
                coord_y += 1;
            } else if coord_x.is_multiple_of(2) {
                coord_x += 1;
            } else {
                // Z traversal.
                coord_y += 1;
                coord_x -= 1;
            }
            local_tile_x = coord_x << subsampling_index;
            local_tile_y = coord_y << subsampling_index;
        }
        tile_x = max_tile_x * max_tile_size + local_tile_x;
        tile_y = max_tile_y * max_tile_size + local_tile_y;

        if tile_x == 0 {
            let progress = (percent_start
                .cast_unsigned()
                .wrapping_add(percent_range.cast_unsigned().wrapping_mul(tile_y) / tiles_per_col))
                as i32;
            if webp_report_progress(pic, progress, Some(&mut *percent)).is_err() {
                return;
            }
        }
    }

    // Figure out the best sampling.
    let mut best_cost = WEBP_INT64_MAX;
    for subsampling_index in 0..=max_subsampling_index {
        let accumulated = get_accumulated_histo(subsampling_index);
        let mut cost = vp8l_shannon_entropy(
            &all_pred_histos[subsampling_index as usize * to_index(NUM_PRED_MODES)..],
            NUM_PRED_MODES,
        )
        .cast_signed();
        for plane in 0..4 {
            cost = cost
                .cast_unsigned()
                .wrapping_add(vp8l_shannon_entropy(
                    &all_accumulated_argb[accumulated + plane * 256..],
                    256,
                ))
                .cast_signed();
        }
        if cost < best_cost {
            best_cost = cost;
            *best_bits = min_bits + subsampling_index as i32;
            *best_mode = Some(subsampling_index as usize);
        }
    }

    drop(raw_data);

    let best = all_modes[best_mode.expect("a best sampling is found")];
    let bits = *best_bits;
    vp8l_optimize_sampling(
        &mut modes_raw[best..],
        width,
        height,
        bits,
        MAX_TRANSFORM_BITS,
        best_bits,
    );
}

// upstream: src/enc/predictor_enc.c VP8LResidualImage
/// Finds the best predictor for each tile, and converts the image to residuals
/// with respect to predictions. If `near_lossless_quality` < 100, applies
/// near lossless processing, shaving off more bits of residuals for lower
/// qualities.
/// `pic` and `percent` are for progress.
/// Returns false in case of error (stored in `pic.error_code`). A failed
/// allocation of the modes returns false without recording an error, as in C.
///
/// # Panics
///
/// Panics when the slices are shorter than the image, the scratch rows or the
/// predictor image that C addresses.
#[must_use]
#[expect(
    clippy::too_many_arguments,
    reason = "the C signature of VP8LResidualImage has fifteen parameters"
)]
pub fn vp8l_residual_image(
    width: i32,
    height: i32,
    min_bits: i32,
    max_bits: i32,
    low_effort: bool,
    argb: &mut [u32],
    argb_scratch: &mut [u32],
    image: &mut [u32],
    near_lossless_quality: i32,
    exact: bool,
    used_subtract_green: bool,
    pic: &mut WebPPicture<'_>,
    percent_range: i32,
    percent: &mut i32,
    best_bits: &mut i32,
) -> bool {
    let percent_start = *percent;
    let max_quantization = 1 << vp8l_near_lossless_bits(near_lossless_quality);
    if low_effort {
        let tiles_per_row =
            vp8l_sub_sample_size(width.cast_unsigned(), max_bits.cast_unsigned()).cast_signed();
        let tiles_per_col =
            vp8l_sub_sample_size(height.cast_unsigned(), max_bits.cast_unsigned()).cast_signed();
        for pixel in &mut image[..to_index(tiles_per_row * tiles_per_col)] {
            *pixel = ARGB_BLACK | ((PRED_LOW_EFFORT as u32) << 8);
        }
        *best_bits = max_bits;
    } else {
        // Allocate data to try all samplings from min_bits to max_bits.
        const SLOTS: usize = MAX_TRANSFORM_BITS as usize + 1;
        let mut sum_num_pixels: u32 = 0;
        let mut modes = [0_usize; SLOTS];
        let mut num_pixels = [0_u32; SLOTS];
        for bits in min_bits..=max_bits {
            let tiles_per_row = vp8l_sub_sample_size(width.cast_unsigned(), bits.cast_unsigned());
            let tiles_per_col = vp8l_sub_sample_size(height.cast_unsigned(), bits.cast_unsigned());
            num_pixels[to_index(bits)] = tiles_per_row.wrapping_mul(tiles_per_col);
            sum_num_pixels = sum_num_pixels.wrapping_add(num_pixels[to_index(bits)]);
        }
        let Some(mut modes_raw) = webp_safe_malloc::<u32>(u64::from(sum_num_pixels), 4) else {
            return false;
        };
        modes_raw.resize(sum_num_pixels as usize, 0);
        // Have modes point to the right global memory modes_raw.
        modes[to_index(min_bits)] = 0;
        for bits in (min_bits + 1)..=max_bits {
            let b = to_index(bits);
            modes[b] = modes[b - 1] + num_pixels[b - 1] as usize;
        }
        // Find the best sampling.
        let mut best_mode: Option<usize> = None;
        get_best_predictors_and_sub_sampling(
            width,
            height,
            min_bits,
            max_bits,
            argb_scratch,
            argb,
            max_quantization,
            exact,
            used_subtract_green,
            pic,
            percent_range,
            percent,
            &mut modes_raw,
            &modes[to_index(min_bits)..],
            best_bits,
            &mut best_mode,
        );
        if *best_bits == 0 {
            return false;
        }
        // Keep the best predictor image.
        let start = modes[to_index(min_bits) + best_mode.expect("a best sampling is found")];
        let count =
            vp8l_sub_sample_size(width.cast_unsigned(), best_bits.cast_unsigned()).wrapping_mul(
                vp8l_sub_sample_size(height.cast_unsigned(), best_bits.cast_unsigned()),
            ) as usize;
        image[..count].copy_from_slice(&modes_raw[start..start + count]);
    }

    copy_image_with_prediction(
        width,
        height,
        *best_bits,
        image,
        argb_scratch,
        argb,
        low_effort,
        max_quantization,
        exact,
        used_subtract_green,
    );
    webp_report_progress(pic, percent_start + percent_range, Some(percent)).is_ok()
}

//------------------------------------------------------------------------------
// Color transform functions.

// upstream: src/enc/predictor_enc.c MultipliersClear
#[inline]
fn multipliers_clear(m: &mut VP8LMultipliers) {
    m.green_to_red = 0;
    m.green_to_blue = 0;
    m.red_to_blue = 0;
}

// upstream: src/enc/predictor_enc.c ColorCodeToMultipliers
#[inline]
fn color_code_to_multipliers(color_code: u32, m: &mut VP8LMultipliers) {
    m.green_to_red = (color_code & 0xff) as u8;
    m.green_to_blue = ((color_code >> 8) & 0xff) as u8;
    m.red_to_blue = ((color_code >> 16) & 0xff) as u8;
}

// upstream: src/enc/predictor_enc.c MultipliersToColorCode
#[inline]
#[expect(
    clippy::trivially_copy_pass_by_ref,
    reason = "C passes the multipliers by pointer"
)]
fn multipliers_to_color_code(m: &VP8LMultipliers) -> u32 {
    0xff00_0000
        | (u32::from(m.red_to_blue) << 16)
        | (u32::from(m.green_to_blue) << 8)
        | u32::from(m.green_to_red)
}

// upstream: src/enc/predictor_enc.c PredictionCostCrossColor
fn prediction_cost_cross_color(accumulated: &[u32], counts: &[u32]) -> i64 {
    // Favor low entropy, locally and globally.
    // Favor small absolute values for PredictionCostSpatial
    const EXP_VALUE: u64 = 240;
    vp8l_combined_shannon_entropy(counts, accumulated).cast_signed()
        + prediction_cost_bias(counts, 3, EXP_VALUE)
}

/// The C `(uint8_t)` cast of a multiplier candidate.
#[inline]
#[expect(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    reason = "the C (uint8_t) cast keeps the low byte"
)]
fn low_byte(value: i32) -> u8 {
    value as u8
}

// upstream: src/enc/predictor_enc.c GetPredictionCostCrossColorRed
#[expect(
    clippy::too_many_arguments,
    reason = "the C signature of GetPredictionCostCrossColorRed has eight parameters"
)]
fn get_prediction_cost_cross_color_red(
    argb: &[u32],
    stride: i32,
    tile_width: i32,
    tile_height: i32,
    prev_x: VP8LMultipliers,
    prev_y: VP8LMultipliers,
    green_to_red: i32,
    accumulated_red_histo: &[u32],
) -> i64 {
    let mut histo = [0_u32; 256];

    vp8l_collect_color_red_transforms(
        argb,
        stride,
        tile_width,
        tile_height,
        green_to_red,
        &mut histo,
    );

    let mut cur_diff = prediction_cost_cross_color(accumulated_red_histo, &histo);
    if low_byte(green_to_red) == prev_x.green_to_red {
        // favor keeping the areas locally similar
        cur_diff -= 3_i64 << LOG_2_PRECISION_BITS;
    }
    if low_byte(green_to_red) == prev_y.green_to_red {
        // favor keeping the areas locally similar
        cur_diff -= 3_i64 << LOG_2_PRECISION_BITS;
    }
    if green_to_red == 0 {
        cur_diff -= 3_i64 << LOG_2_PRECISION_BITS;
    }
    cur_diff
}

// upstream: src/enc/predictor_enc.c GetBestGreenToRed
#[expect(
    clippy::too_many_arguments,
    reason = "the C signature of GetBestGreenToRed has nine parameters"
)]
fn get_best_green_to_red(
    argb: &[u32],
    stride: i32,
    tile_width: i32,
    tile_height: i32,
    prev_x: VP8LMultipliers,
    prev_y: VP8LMultipliers,
    quality: i32,
    accumulated_red_histo: &[u32],
    best_tx: &mut VP8LMultipliers,
) {
    let max_iters = 4 + ((7 * quality) >> 8); // in range [4..6]
    let mut green_to_red_best: i32 = 0;
    let mut best_diff = get_prediction_cost_cross_color_red(
        argb,
        stride,
        tile_width,
        tile_height,
        prev_x,
        prev_y,
        green_to_red_best,
        accumulated_red_histo,
    );
    for iter in 0..max_iters {
        // ColorTransformDelta is a 3.5 bit fixed point, so 32 is equal to
        // one in color computation. Having initial delta here as 1 is sufficient
        // to explore the range of (-2, 2).
        let delta = 32 >> iter;
        // Try a negative and a positive delta from the best known value.
        let mut offset = -delta;
        while offset <= delta {
            let green_to_red_cur = offset + green_to_red_best;
            let cur_diff = get_prediction_cost_cross_color_red(
                argb,
                stride,
                tile_width,
                tile_height,
                prev_x,
                prev_y,
                green_to_red_cur,
                accumulated_red_histo,
            );
            if cur_diff < best_diff {
                best_diff = cur_diff;
                green_to_red_best = green_to_red_cur;
            }
            offset += 2 * delta;
        }
    }
    best_tx.green_to_red = low_byte(green_to_red_best & 0xff);
}

// upstream: src/enc/predictor_enc.c GetPredictionCostCrossColorBlue
#[expect(
    clippy::too_many_arguments,
    reason = "the C signature of GetPredictionCostCrossColorBlue has nine parameters"
)]
fn get_prediction_cost_cross_color_blue(
    argb: &[u32],
    stride: i32,
    tile_width: i32,
    tile_height: i32,
    prev_x: VP8LMultipliers,
    prev_y: VP8LMultipliers,
    green_to_blue: i32,
    red_to_blue: i32,
    accumulated_blue_histo: &[u32],
) -> i64 {
    let mut histo = [0_u32; 256];

    vp8l_collect_color_blue_transforms(
        argb,
        stride,
        tile_width,
        tile_height,
        green_to_blue,
        red_to_blue,
        &mut histo,
    );

    let mut cur_diff = prediction_cost_cross_color(accumulated_blue_histo, &histo);
    if low_byte(green_to_blue) == prev_x.green_to_blue {
        // favor keeping the areas locally similar
        cur_diff -= 3_i64 << LOG_2_PRECISION_BITS;
    }
    if low_byte(green_to_blue) == prev_y.green_to_blue {
        // favor keeping the areas locally similar
        cur_diff -= 3_i64 << LOG_2_PRECISION_BITS;
    }
    if low_byte(red_to_blue) == prev_x.red_to_blue {
        // favor keeping the areas locally similar
        cur_diff -= 3_i64 << LOG_2_PRECISION_BITS;
    }
    if low_byte(red_to_blue) == prev_y.red_to_blue {
        // favor keeping the areas locally similar
        cur_diff -= 3_i64 << LOG_2_PRECISION_BITS;
    }
    if green_to_blue == 0 {
        cur_diff -= 3_i64 << LOG_2_PRECISION_BITS;
    }
    if red_to_blue == 0 {
        cur_diff -= 3_i64 << LOG_2_PRECISION_BITS;
    }
    cur_diff
}

// upstream: src/enc/predictor_enc.c kGreenRedToBlueNumAxis
const GREEN_RED_TO_BLUE_NUM_AXIS: usize = 8;
// upstream: src/enc/predictor_enc.c kGreenRedToBlueMaxIters
const GREEN_RED_TO_BLUE_MAX_ITERS: i32 = 7;

// upstream: src/enc/predictor_enc.c GetBestGreenRedToBlue
#[expect(
    clippy::too_many_arguments,
    reason = "the C signature of GetBestGreenRedToBlue has nine parameters"
)]
fn get_best_green_red_to_blue(
    argb: &[u32],
    stride: i32,
    tile_width: i32,
    tile_height: i32,
    prev_x: VP8LMultipliers,
    prev_y: VP8LMultipliers,
    quality: i32,
    accumulated_blue_histo: &[u32],
    best_tx: &mut VP8LMultipliers,
) {
    const OFFSET: [[i8; 2]; GREEN_RED_TO_BLUE_NUM_AXIS] = [
        [0, -1],
        [0, 1],
        [-1, 0],
        [1, 0],
        [-1, -1],
        [-1, 1],
        [1, -1],
        [1, 1],
    ];
    const DELTA_LUT: [i8; GREEN_RED_TO_BLUE_MAX_ITERS as usize] = [16, 16, 8, 4, 2, 2, 2];
    let iters = if quality < 25 {
        1
    } else if quality > 50 {
        GREEN_RED_TO_BLUE_MAX_ITERS
    } else {
        4
    };
    let mut green_to_blue_best: i32 = 0;
    let mut red_to_blue_best: i32 = 0;
    // Initial value at origin:
    let mut best_diff = get_prediction_cost_cross_color_blue(
        argb,
        stride,
        tile_width,
        tile_height,
        prev_x,
        prev_y,
        green_to_blue_best,
        red_to_blue_best,
        accumulated_blue_histo,
    );
    for iter in 0..iters {
        let delta = i32::from(DELTA_LUT[to_index(iter)]);
        for axis in &OFFSET {
            let green_to_blue_cur = i32::from(axis[0]) * delta + green_to_blue_best;
            let red_to_blue_cur = i32::from(axis[1]) * delta + red_to_blue_best;
            let cur_diff = get_prediction_cost_cross_color_blue(
                argb,
                stride,
                tile_width,
                tile_height,
                prev_x,
                prev_y,
                green_to_blue_cur,
                red_to_blue_cur,
                accumulated_blue_histo,
            );
            if cur_diff < best_diff {
                best_diff = cur_diff;
                green_to_blue_best = green_to_blue_cur;
                red_to_blue_best = red_to_blue_cur;
            }
            if quality < 25 && iter == 4 {
                // Only axis aligned diffs for lower quality.
                break; // next iter.
            }
        }
        if delta == 2 && green_to_blue_best == 0 && red_to_blue_best == 0 {
            // Further iterations would not help.
            break; // out of iter-loop.
        }
    }
    best_tx.green_to_blue = low_byte(green_to_blue_best & 0xff);
    best_tx.red_to_blue = low_byte(red_to_blue_best & 0xff);
}

// upstream: src/enc/predictor_enc.c GetBestColorTransformForTile
#[expect(
    clippy::too_many_arguments,
    reason = "the C signature of GetBestColorTransformForTile has eleven parameters"
)]
#[expect(clippy::similar_names, reason = "the C names all_x_max and all_y_max")]
fn get_best_color_transform_for_tile(
    tile_x: i32,
    tile_y: i32,
    bits: i32,
    prev_x: VP8LMultipliers,
    prev_y: VP8LMultipliers,
    quality: i32,
    xsize: i32,
    ysize: i32,
    accumulated_red_histo: &[u32],
    accumulated_blue_histo: &[u32],
    argb: &[u32],
) -> VP8LMultipliers {
    let max_tile_size = 1 << bits;
    let tile_y_offset = tile_y * max_tile_size;
    let tile_x_offset = tile_x * max_tile_size;
    let all_x_max = get_min(tile_x_offset + max_tile_size, xsize);
    let all_y_max = get_min(tile_y_offset + max_tile_size, ysize);
    let tile_width = all_x_max - tile_x_offset;
    let tile_height = all_y_max - tile_y_offset;
    let tile_argb = &argb[to_index(tile_y_offset * xsize + tile_x_offset)..];
    let mut best_tx = VP8LMultipliers::default();
    multipliers_clear(&mut best_tx);

    get_best_green_to_red(
        tile_argb,
        xsize,
        tile_width,
        tile_height,
        prev_x,
        prev_y,
        quality,
        accumulated_red_histo,
        &mut best_tx,
    );
    get_best_green_red_to_blue(
        tile_argb,
        xsize,
        tile_width,
        tile_height,
        prev_x,
        prev_y,
        quality,
        accumulated_blue_histo,
        &mut best_tx,
    );
    best_tx
}

// upstream: src/enc/predictor_enc.c CopyTileWithColorTransform
fn copy_tile_with_color_transform(
    xsize: i32,
    ysize: i32,
    tile_x: i32,
    tile_y: i32,
    max_tile_size: i32,
    color_transform: VP8LMultipliers,
    argb: &mut [u32],
) {
    let xscan = get_min(max_tile_size, xsize - tile_x);
    let mut yscan = get_min(max_tile_size, ysize - tile_y);
    let mut pos = to_index(tile_y * xsize + tile_x);
    while yscan > 0 {
        yscan -= 1;
        vp8l_transform_color(&color_transform, &mut argb[pos..], xscan);
        pos += to_index(xsize);
    }
}

// upstream: src/enc/predictor_enc.c VP8LColorSpaceTransform
/// `pic` and `percent` are for progress.
///
/// # Errors
///
/// Returns the error that `pic` records after a user abort.
#[expect(
    clippy::too_many_arguments,
    reason = "the C signature of VP8LColorSpaceTransform has ten parameters"
)]
#[expect(
    clippy::similar_names,
    reason = "the C names tile_xsize, tile_ysize, all_x_max and all_y_max"
)]
pub fn vp8l_color_space_transform(
    width: i32,
    height: i32,
    bits: i32,
    quality: i32,
    argb: &mut [u32],
    image: &mut [u32],
    pic: &mut WebPPicture<'_>,
    percent_range: i32,
    percent: &mut i32,
    best_bits: &mut i32,
) -> Result<(), WebPEncodingError> {
    let max_tile_size = 1 << bits;
    let tile_xsize =
        vp8l_sub_sample_size(width.cast_unsigned(), bits.cast_unsigned()).cast_signed();
    let tile_ysize =
        vp8l_sub_sample_size(height.cast_unsigned(), bits.cast_unsigned()).cast_signed();
    let percent_start = *percent;
    let mut accumulated_red_histo = [0_u32; 256];
    let mut accumulated_blue_histo = [0_u32; 256];
    let mut prev_x = VP8LMultipliers::default();
    let mut prev_y = VP8LMultipliers::default();
    multipliers_clear(&mut prev_y);
    multipliers_clear(&mut prev_x);
    for tile_y in 0..tile_ysize {
        for tile_x in 0..tile_xsize {
            let tile_x_offset = tile_x * max_tile_size;
            let tile_y_offset = tile_y * max_tile_size;
            let all_x_max = get_min(tile_x_offset + max_tile_size, width);
            let all_y_max = get_min(tile_y_offset + max_tile_size, height);
            let offset = to_index(tile_y * tile_xsize + tile_x);
            if tile_y != 0 {
                color_code_to_multipliers(image[offset - to_index(tile_xsize)], &mut prev_y);
            }
            prev_x = get_best_color_transform_for_tile(
                tile_x,
                tile_y,
                bits,
                prev_x,
                prev_y,
                quality,
                width,
                height,
                &accumulated_red_histo,
                &accumulated_blue_histo,
                argb,
            );
            image[offset] = multipliers_to_color_code(&prev_x);
            copy_tile_with_color_transform(
                width,
                height,
                tile_x_offset,
                tile_y_offset,
                max_tile_size,
                prev_x,
                argb,
            );

            // Gather accumulated histogram data.
            for y in tile_y_offset..all_y_max {
                let mut ix = to_index(y * width + tile_x_offset);
                let ix_end = ix + to_index(all_x_max - tile_x_offset);
                let w = to_index(width);
                while ix < ix_end {
                    let pix = argb[ix];
                    if ix >= 2 && pix == argb[ix - 2] && pix == argb[ix - 1] {
                        ix += 1;
                        continue; // repeated pixels are handled by backward references
                    }
                    if ix >= w + 2
                        && argb[ix - 2] == argb[ix - w - 2]
                        && argb[ix - 1] == argb[ix - w - 1]
                        && pix == argb[ix - w]
                    {
                        ix += 1;
                        continue; // repeated pixels are handled by backward references
                    }
                    accumulated_red_histo[((pix >> 16) & 0xff) as usize] += 1;
                    accumulated_blue_histo[(pix & 0xff) as usize] += 1;
                    ix += 1;
                }
            }
        }
        let progress = percent_start + percent_range * tile_y / tile_ysize;
        webp_report_progress(pic, progress, Some(&mut *percent))?;
    }
    vp8l_optimize_sampling(image, width, height, bits, MAX_TRANSFORM_BITS, best_bits);
    Ok(())
}
