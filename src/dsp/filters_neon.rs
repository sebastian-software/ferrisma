// libwebp v1.6.0, src/dsp/filters_neon.c:
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
// NEON variant of alpha filters
//
// Author: Skal (pascal.massimino@gmail.com)

//! Translation of `src/dsp/filters_neon.c` from libwebp v1.6.0.
//!
//! Work package: WP20 (NEON).
//!
//! The NEON variants of the alpha-plane filters of `src/dsp/filters.c`. Each
//! `_neon` function is the body that upstream's `VP8FiltersInitNEON` stores in
//! `WebPFilters` or `WebPUnfilters`; the safe entry point below it, named
//! after the function without the suffix, has the type `WebPFilterFunc` or
//! `WebPUnfilterFunc`, and [`filters`] and [`unfilters`] give the entry of a
//! filter type, which `webp_filters` and `webp_unfilters` of
//! `src/dsp/filters.rs` use on 64-bit ARM.
//!
//! `VP8FiltersInitNEON` sets `WebPFilters` for the horizontal, vertical and
//! gradient filters and `WebPUnfilters` for the horizontal and vertical
//! unfilters; `VP8FiltersInit` calls it unconditionally on 64-bit ARM
//! (`WEBP_NEON_OMIT_C_CODE`). `WEBP_FILTER_NONE` and the gradient unfilter
//! keep their C functions.
//!
//! # Deviations from upstream
//!
//! 1. **Loads, stores and alignment** follow the deviations 1 to 3 of
//!    `src/dsp/neon.rs`.
//! 2. **Pointers** into the plane are positions: `in - stride`, `src - 1`
//!    and `top[i - 1]` are indices into the input slice, as in
//!    `src/dsp/filters.rs`. The unfilters take the distinct slices of
//!    `WebPUnfilterFunc`, whose callers copy for the in-place form (the
//!    deviation 15 of WP4).
//! 3. **The macros** `U8_TO_S16`, `LOAD_U8_TO_S16`, `SHIFT_RIGHT_N_Q` and
//!    `SHIFT_LEFT_N_Q` are functions; the shift count of `SHIFT_LEFT_N_Q`,
//!    an immediate, selects the `vextq_u8` instance. `DCHECK` is a function
//!    of `debug_assert!`s.
//!
//! # Excluded undefined behavior
//!
//! None.
//!
//! # Checks not applicable
//!
//! 1. **`USE_GRADIENT_UNFILTER`** is 0 in upstream ("correct but slower than
//!    the C-version"), so `GRAD_PROCESS_LANE`, `GradientPredictInverse_NEON`
//!    and `GradientUnfilter_NEON` are not compiled, and neither are their
//!    only users of `ROTATE_LEFT_N` and `ROTATE_RIGHT_N`; none is translated.
//! 2. **`HorizontalUnfilter_NEON` with a width of 0** writes `out[0]`, which
//!    `HorizontalUnfilter_C` does not; the alpha decoder passes the width of
//!    the plane, at least 1, and the parity test covers widths from 1. The
//!    translation keeps upstream's write.
//! 3. **`WEBP_RESTRICT`** has no counterpart: the slices cannot alias.

use core::arch::aarch64::{
    int16x8_t, uint8x8_t, uint8x16_t, vaddl_u8, vaddq_u8, vdupq_n_u8, vextq_u8, vmovl_u8,
    vqmovun_s16, vreinterpretq_s16_u16, vsetq_lane_u8, vsub_u8, vsubq_s16, vsubq_u8,
};

use crate::dsp::dsp::WEBP_FILTER_TYPE;
use crate::dsp::filters::{WebPFilterFunc, WebPUnfilterFunc};
use crate::dsp::neon::{vld1_u8, vld1q_u8, vst1_u8, vst1q_u8};

//------------------------------------------------------------------------------
// Helpful macros.

// upstream: src/dsp/filters_neon.c DCHECK
/// The assertions of the filters on their arguments.
fn dcheck(width: i32, height: i32, stride: i32) {
    debug_assert!(width > 0);
    debug_assert!(height > 0);
    debug_assert!(stride >= width);
}

// upstream: src/dsp/filters_neon.c U8_TO_S16
/// load eight u8 and widen to s16
#[target_feature(enable = "neon")]
#[must_use]
fn u8_to_s16(a: uint8x8_t) -> int16x8_t {
    vreinterpretq_s16_u16(vmovl_u8(a))
}

// upstream: src/dsp/filters_neon.c LOAD_U8_TO_S16
#[target_feature(enable = "neon")]
#[must_use]
fn load_u8_to_s16(a: &[u8]) -> int16x8_t {
    u8_to_s16(vld1_u8(a))
}

// upstream: src/dsp/filters_neon.c SHIFT_RIGHT_N_Q
/// shift right by 15 bytes, inserting zeros: the only use, `N == 15`.
#[target_feature(enable = "neon")]
#[must_use]
fn shift_right_15_q(a: uint8x16_t, zero: uint8x16_t) -> uint8x16_t {
    vextq_u8::<15>(a, zero)
}

// upstream: src/dsp/filters_neon.c SHIFT_LEFT_N_Q
/// shift left by `n` bytes (1, 2, 4 or 8), inserting zeros:
/// `vextq_u8(zero, a, (16 - n) % 16)`.
#[target_feature(enable = "neon")]
#[must_use]
fn shift_left_n_q(a: uint8x16_t, zero: uint8x16_t, n: usize) -> uint8x16_t {
    match n {
        1 => vextq_u8::<15>(zero, a),
        2 => vextq_u8::<14>(zero, a),
        4 => vextq_u8::<12>(zero, a),
        _ => vextq_u8::<8>(zero, a),
    }
}

// upstream: src/dsp/filters_neon.c PredictLine_NEON
#[target_feature(enable = "neon")]
fn predict_line_neon(src: &[u8], pred: &[u8], dst: &mut [u8], length: usize) {
    let mut i = 0;
    while i + 16 <= length {
        let a = vld1q_u8(&src[i..]);
        let b = vld1q_u8(&pred[i..]);
        let c = vsubq_u8(a, b);
        vst1q_u8(&mut dst[i..], c);
        i += 16;
    }
    while i < length {
        dst[i] = src[i].wrapping_sub(pred[i]);
        i += 1;
    }
}

// upstream: src/dsp/filters_neon.c PredictLineLeft_NEON
/// Special case for left-based prediction (when preds==dst-1 or preds==src-1):
/// `src` is `plane[pos..]` and the prediction `plane[pos - 1..]`.
#[target_feature(enable = "neon")]
fn predict_line_left_neon(plane: &[u8], pos: usize, dst: &mut [u8], length: usize) {
    predict_line_neon(&plane[pos..], &plane[pos - 1..], dst, length);
}

/// The width, height and stride of a plane, which the filters take as
/// positive.
fn dimensions(width: i32, height: i32, stride: i32) -> (usize, usize, usize) {
    (
        usize::try_from(width).expect("the width is positive"),
        usize::try_from(height).expect("the height is positive"),
        usize::try_from(stride).expect("the stride is not negative"),
    )
}

//------------------------------------------------------------------------------
// Horizontal filter.

// upstream: src/dsp/filters_neon.c DoHorizontalFilter_NEON
#[target_feature(enable = "neon")]
fn do_horizontal_filter_neon(input: &[u8], width: i32, height: i32, stride: i32, out: &mut [u8]) {
    dcheck(width, height, stride);
    let (width, height, stride) = dimensions(width, height, stride);

    // Leftmost pixel is the same as input for topmost scanline.
    out[0] = input[0];
    predict_line_left_neon(input, 1, &mut out[1..], width - 1);

    // Filter line-by-line.
    for row in 1..height {
        let cur = row * stride;
        // Leftmost pixel is predicted from above.
        out[cur] = input[cur].wrapping_sub(input[cur - stride]);
        predict_line_left_neon(input, cur + 1, &mut out[cur + 1..], width - 1);
    }
}

// upstream: src/dsp/filters_neon.c HorizontalFilter_NEON
#[target_feature(enable = "neon")]
fn horizontal_filter_neon(
    data: &[u8],
    width: i32,
    height: i32,
    stride: i32,
    filtered_data: &mut [u8],
) {
    do_horizontal_filter_neon(data, width, height, stride, filtered_data);
}

/// Entry point of `HorizontalFilter_NEON` for
/// `WebPFilters[WEBP_FILTER_HORIZONTAL]`.
pub(crate) fn horizontal_filter(
    data: &[u8],
    width: i32,
    height: i32,
    stride: i32,
    filtered_data: &mut [u8],
) {
    // SAFETY: NEON is in the AArch64 baseline that this module is compiled for,
    // and upstream selects the NEON variant there unconditionally (ADR 0008).
    unsafe { horizontal_filter_neon(data, width, height, stride, filtered_data) }
}

//------------------------------------------------------------------------------
// Vertical filter.

// upstream: src/dsp/filters_neon.c DoVerticalFilter_NEON
#[target_feature(enable = "neon")]
fn do_vertical_filter_neon(input: &[u8], width: i32, height: i32, stride: i32, out: &mut [u8]) {
    dcheck(width, height, stride);
    let (width, height, stride) = dimensions(width, height, stride);

    // Very first top-left pixel is copied.
    out[0] = input[0];
    // Rest of top scan-line is left-predicted.
    predict_line_left_neon(input, 1, &mut out[1..], width - 1);

    // Filter line-by-line.
    for row in 1..height {
        let cur = row * stride;
        predict_line_neon(
            &input[cur..],
            &input[cur - stride..],
            &mut out[cur..],
            width,
        );
    }
}

// upstream: src/dsp/filters_neon.c VerticalFilter_NEON
#[target_feature(enable = "neon")]
fn vertical_filter_neon(
    data: &[u8],
    width: i32,
    height: i32,
    stride: i32,
    filtered_data: &mut [u8],
) {
    do_vertical_filter_neon(data, width, height, stride, filtered_data);
}

/// Entry point of `VerticalFilter_NEON` for `WebPFilters[WEBP_FILTER_VERTICAL]`.
pub(crate) fn vertical_filter(
    data: &[u8],
    width: i32,
    height: i32,
    stride: i32,
    filtered_data: &mut [u8],
) {
    // SAFETY: NEON is in the AArch64 baseline that this module is compiled for,
    // and upstream selects the NEON variant there unconditionally (ADR 0008).
    unsafe { vertical_filter_neon(data, width, height, stride, filtered_data) }
}

//------------------------------------------------------------------------------
// Gradient filter.

// upstream: src/dsp/filters_neon.c GradientPredictor_C
#[must_use]
fn gradient_predictor_c(a: u8, b: u8, c: u8) -> u8 {
    let g = i32::from(a) + i32::from(b) - i32::from(c);
    // clip to 8bit
    if (g & !0xff) == 0 {
        u8::try_from(g).expect("a value in 0..=255")
    } else if g < 0 {
        0
    } else {
        255
    }
}

// upstream: src/dsp/filters_neon.c GradientPredictDirect_NEON
/// `row` and `top` are the positions of the current and the upper row in
/// `plane`; the predictor reads one sample before each.
#[target_feature(enable = "neon")]
#[expect(
    clippy::many_single_char_names,
    reason = "the short C names of the source"
)]
fn gradient_predict_direct_neon(
    plane: &[u8],
    row: usize,
    top: usize,
    out: &mut [u8],
    length: usize,
) {
    let mut i = 0;
    while i + 8 <= length {
        let a = vld1_u8(&plane[row + i - 1..]);
        let b = vld1_u8(&plane[top + i..]);
        let c = vreinterpretq_s16_u16(vaddl_u8(a, b));
        let d = load_u8_to_s16(&plane[top + i - 1..]);
        let e = vqmovun_s16(vsubq_s16(c, d));
        let f = vld1_u8(&plane[row + i..]);
        vst1_u8(&mut out[i..], vsub_u8(f, e));
        i += 8;
    }
    while i < length {
        out[i] = plane[row + i].wrapping_sub(gradient_predictor_c(
            plane[row + i - 1],
            plane[top + i],
            plane[top + i - 1],
        ));
        i += 1;
    }
}

// upstream: src/dsp/filters_neon.c DoGradientFilter_NEON
#[target_feature(enable = "neon")]
fn do_gradient_filter_neon(input: &[u8], width: i32, height: i32, stride: i32, out: &mut [u8]) {
    dcheck(width, height, stride);
    let (width, height, stride) = dimensions(width, height, stride);

    // left prediction for top scan-line
    out[0] = input[0];
    predict_line_left_neon(input, 1, &mut out[1..], width - 1);

    // Filter line-by-line.
    for row in 1..height {
        let cur = row * stride;
        out[cur] = input[cur].wrapping_sub(input[cur - stride]);
        gradient_predict_direct_neon(
            input,
            cur + 1,
            cur + 1 - stride,
            &mut out[cur + 1..],
            width - 1,
        );
    }
}

// upstream: src/dsp/filters_neon.c GradientFilter_NEON
#[target_feature(enable = "neon")]
fn gradient_filter_neon(
    data: &[u8],
    width: i32,
    height: i32,
    stride: i32,
    filtered_data: &mut [u8],
) {
    do_gradient_filter_neon(data, width, height, stride, filtered_data);
}

/// Entry point of `GradientFilter_NEON` for `WebPFilters[WEBP_FILTER_GRADIENT]`.
pub(crate) fn gradient_filter(
    data: &[u8],
    width: i32,
    height: i32,
    stride: i32,
    filtered_data: &mut [u8],
) {
    // SAFETY: NEON is in the AArch64 baseline that this module is compiled for,
    // and upstream selects the NEON variant there unconditionally (ADR 0008).
    unsafe { gradient_filter_neon(data, width, height, stride, filtered_data) }
}

//------------------------------------------------------------------------------
// Inverse transforms

// upstream: src/dsp/filters_neon.c HorizontalUnfilter_NEON
#[target_feature(enable = "neon")]
fn horizontal_unfilter_neon(prev: Option<&[u8]>, input: &[u8], out: &mut [u8], width: i32) {
    let zero = vdupq_n_u8(0);
    out[0] = input[0].wrapping_add(prev.map_or(0, |prev| prev[0]));
    if width <= 1 {
        return;
    }
    let width = usize::try_from(width).expect("the width is positive");
    let mut last = vsetq_lane_u8::<0>(out[0], zero);
    let mut i = 1;
    while i + 16 <= width {
        let a0 = vld1q_u8(&input[i..]);
        let a1 = vaddq_u8(a0, last);
        let a2 = shift_left_n_q(a1, zero, 1);
        let a3 = vaddq_u8(a1, a2);
        let a4 = shift_left_n_q(a3, zero, 2);
        let a5 = vaddq_u8(a3, a4);
        let a6 = shift_left_n_q(a5, zero, 4);
        let a7 = vaddq_u8(a5, a6);
        let a8 = shift_left_n_q(a7, zero, 8);
        let a9 = vaddq_u8(a7, a8);
        vst1q_u8(&mut out[i..], a9);
        last = shift_right_15_q(a9, zero);
        i += 16;
    }
    while i < width {
        out[i] = input[i].wrapping_add(out[i - 1]);
        i += 1;
    }
}

/// Entry point of `HorizontalUnfilter_NEON` for
/// `WebPUnfilters[WEBP_FILTER_HORIZONTAL]`.
pub(crate) fn horizontal_unfilter(prev: Option<&[u8]>, input: &[u8], out: &mut [u8], width: i32) {
    // SAFETY: NEON is in the AArch64 baseline that this module is compiled for,
    // and upstream selects the NEON variant there unconditionally (ADR 0008).
    unsafe { horizontal_unfilter_neon(prev, input, out, width) }
}

// upstream: src/dsp/filters_neon.c VerticalUnfilter_NEON
#[target_feature(enable = "neon")]
fn vertical_unfilter_neon(prev: Option<&[u8]>, input: &[u8], out: &mut [u8], width: i32) {
    match prev {
        None => horizontal_unfilter_neon(None, input, out, width),
        Some(prev) => {
            debug_assert!(width >= 0);
            let width = usize::try_from(width).unwrap_or(0);
            let mut i = 0;
            while i + 16 <= width {
                let a = vld1q_u8(&input[i..]);
                let b = vld1q_u8(&prev[i..]);
                let c = vaddq_u8(a, b);
                vst1q_u8(&mut out[i..], c);
                i += 16;
            }
            while i < width {
                out[i] = input[i].wrapping_add(prev[i]);
                i += 1;
            }
        }
    }
}

/// Entry point of `VerticalUnfilter_NEON` for
/// `WebPUnfilters[WEBP_FILTER_VERTICAL]`.
pub(crate) fn vertical_unfilter(prev: Option<&[u8]>, input: &[u8], out: &mut [u8], width: i32) {
    // SAFETY: NEON is in the AArch64 baseline that this module is compiled for,
    // and upstream selects the NEON variant there unconditionally (ADR 0008).
    unsafe { vertical_unfilter_neon(prev, input, out, width) }
}

//------------------------------------------------------------------------------
// Entry point

// upstream: src/dsp/filters_neon.c VP8FiltersInitNEON
/// The entry of `WebPFilters` that `VP8FiltersInitNEON` sets for `filter`, or
/// `None` where it keeps the C entry.
#[must_use]
pub(crate) fn filters(filter: WEBP_FILTER_TYPE) -> Option<WebPFilterFunc> {
    let filter_fn: WebPFilterFunc = match filter {
        WEBP_FILTER_TYPE::WEBP_FILTER_HORIZONTAL => horizontal_filter,
        WEBP_FILTER_TYPE::WEBP_FILTER_VERTICAL => vertical_filter,
        WEBP_FILTER_TYPE::WEBP_FILTER_GRADIENT => gradient_filter,
        _ => return None,
    };
    Some(filter_fn)
}

// upstream: src/dsp/filters_neon.c VP8FiltersInitNEON
/// The entry of `WebPUnfilters` that `VP8FiltersInitNEON` sets for `filter`,
/// or `None` where it keeps the C entry (`USE_GRADIENT_UNFILTER` is 0).
#[must_use]
pub(crate) fn unfilters(filter: WEBP_FILTER_TYPE) -> Option<WebPUnfilterFunc> {
    let unfilter_fn: WebPUnfilterFunc = match filter {
        WEBP_FILTER_TYPE::WEBP_FILTER_HORIZONTAL => horizontal_unfilter,
        WEBP_FILTER_TYPE::WEBP_FILTER_VERTICAL => vertical_unfilter,
        _ => return None,
    };
    Some(unfilter_fn)
}
