// libwebp v1.6.0, src/dsp/filters.c:
//
// Copyright 2011 Google Inc. All Rights Reserved.
//
// Use of this source code is governed by a BSD-style license
// that can be found in the COPYING file in the root of the source
// tree. An additional intellectual property rights grant can be found
// in the file PATENTS. All contributing project authors may
// be found in the AUTHORS file in the root of the source tree.
// -----------------------------------------------------------------------------
//
// Spatial prediction using various filters
//
// Author: Urvang (urvang@google.com)

//! Translation of `src/dsp/filters.c` from libwebp v1.6.0.
//!
//! Work package: WP4 (pixel ops).

use crate::dsp::dsp::WEBP_FILTER_TYPE;

// upstream: src/dsp/dsp.h WebPFilterFunc
/// Filters a 2-D pixel array of `stride * height` bytes into `out`.
pub type WebPFilterFunc = fn(&[u8], i32, i32, i32, &mut [u8]);

// upstream: src/dsp/dsp.h WebPUnfilterFunc
/// Reconstructs one row from its filtered form `input`, with the reconstructed
/// previous row `prev` when there is one. The C form may reconstruct in place,
/// with `prev`, `input` and the output aliasing; the Rust form takes distinct
/// slices, and the callers take a copy when they need the in-place form. Each
/// unfilter reads every input element before it writes the same index, so the
/// copy gives the same bytes (ADR 0003).
pub type WebPUnfilterFunc = fn(Option<&[u8]>, &[u8], &mut [u8], i32);

// upstream: src/dsp/filters.c PredictLine_C
/// Writes `src - pred` for `length` bytes, modulo 256.
fn predict_line_c(src: &[u8], pred: &[u8], dst: &mut [u8], length: usize) {
    for i in 0..length {
        dst[i] = src[i].wrapping_sub(pred[i]);
    }
}

// upstream: src/dsp/filters.c GradientPredictor_C
/// The gradient predictor `a + b - c`, clipped to a byte.
#[must_use]
fn gradient_predictor_c(a: u8, b: u8, c: u8) -> i32 {
    let g = i32::from(a) + i32::from(b) - i32::from(c);
    if (g & !0xff) == 0 {
        g
    } else if g < 0 {
        0
    } else {
        255
    }
}

/// Stores a predictor, which `gradient_predictor_c` clips to a byte.
#[must_use]
fn predictor_byte(pred: i32) -> u8 {
    u8::try_from(pred).expect("the gradient predictor is a byte")
}

// upstream: src/dsp/filters.c DoHorizontalFilter_C
/// The horizontal filter: each pixel is predicted from its left neighbor, and
/// the leftmost pixel of each row from the row above.
fn do_horizontal_filter_c(input: &[u8], width: i32, height: i32, stride: i32, out: &mut [u8]) {
    let width = usize::try_from(width).expect("the width is positive");
    let height = usize::try_from(height).expect("the height is positive");
    let stride = usize::try_from(stride).expect("the stride is not negative");
    // Leftmost pixel is the same as input for topmost scanline.
    out[0] = input[0];
    predict_line_c(&input[1..], &input[0..], &mut out[1..], width - 1);
    // Filter line-by-line.
    for row in 1..height {
        let cur = row * stride;
        let above = (row - 1) * stride;
        // Leftmost pixel is predicted from above.
        predict_line_c(&input[cur..], &input[above..], &mut out[cur..], 1);
        predict_line_c(
            &input[cur + 1..],
            &input[cur..],
            &mut out[cur + 1..],
            width - 1,
        );
    }
}

// upstream: src/dsp/filters.c DoVerticalFilter_C
/// The vertical filter: each pixel is predicted from the pixel above it, and
/// the top row from its left neighbor.
fn do_vertical_filter_c(input: &[u8], width: i32, height: i32, stride: i32, out: &mut [u8]) {
    let width = usize::try_from(width).expect("the width is positive");
    let height = usize::try_from(height).expect("the height is positive");
    let stride = usize::try_from(stride).expect("the stride is not negative");
    // Very first top-left pixel is copied.
    out[0] = input[0];
    // Rest of top scan-line is left-predicted.
    predict_line_c(&input[1..], &input[0..], &mut out[1..], width - 1);
    // Filter line-by-line.
    for row in 1..height {
        let cur = row * stride;
        let above = (row - 1) * stride;
        predict_line_c(&input[cur..], &input[above..], &mut out[cur..], width);
    }
}

// upstream: src/dsp/filters.c DoGradientFilter_C
/// The gradient filter: each pixel is predicted with the gradient predictor
/// from its left, upper and upper-left neighbors.
fn do_gradient_filter_c(input: &[u8], width: i32, height: i32, stride: i32, out: &mut [u8]) {
    let width = usize::try_from(width).expect("the width is positive");
    let height = usize::try_from(height).expect("the height is positive");
    let stride = usize::try_from(stride).expect("the stride is not negative");
    // left prediction for top scan-line
    out[0] = input[0];
    predict_line_c(&input[1..], &input[0..], &mut out[1..], width - 1);
    // Filter line-by-line.
    for row in 1..height {
        let cur = row * stride;
        let above = (row - 1) * stride;
        // leftmost pixel: predict from above.
        predict_line_c(&input[cur..], &input[above..], &mut out[cur..], 1);
        for w in 1..width {
            let pred =
                gradient_predictor_c(input[cur + w - 1], input[above + w], input[above + w - 1]);
            out[cur + w] = input[cur + w].wrapping_sub(predictor_byte(pred));
        }
    }
}

// upstream: src/dsp/filters.c HorizontalFilter_C
fn horizontal_filter_c(
    data: &[u8],
    width: i32,
    height: i32,
    stride: i32,
    filtered_data: &mut [u8],
) {
    do_horizontal_filter_c(data, width, height, stride, filtered_data);
}

// upstream: src/dsp/filters.c VerticalFilter_C
fn vertical_filter_c(data: &[u8], width: i32, height: i32, stride: i32, filtered_data: &mut [u8]) {
    do_vertical_filter_c(data, width, height, stride, filtered_data);
}

// upstream: src/dsp/filters.c GradientFilter_C
fn gradient_filter_c(data: &[u8], width: i32, height: i32, stride: i32, filtered_data: &mut [u8]) {
    do_gradient_filter_c(data, width, height, stride, filtered_data);
}

// upstream: src/dsp/filters.c NoneUnfilter_C
fn none_unfilter_c(_prev: Option<&[u8]>, input: &[u8], out: &mut [u8], width: i32) {
    let width = usize::try_from(width).unwrap_or(0);
    out[..width].copy_from_slice(&input[..width]);
}

// upstream: src/dsp/filters.c HorizontalUnfilter_C
#[expect(
    clippy::similar_names,
    reason = "the C names prev and pred of the reconstruction"
)]
fn horizontal_unfilter_c(prev: Option<&[u8]>, input: &[u8], out: &mut [u8], width: i32) {
    let width = usize::try_from(width).unwrap_or(0);
    let mut pred = prev.map_or(0, |prev| prev[0]);
    for i in 0..width {
        out[i] = pred.wrapping_add(input[i]);
        pred = out[i];
    }
}

// upstream: src/dsp/filters.c VerticalUnfilter_C
fn vertical_unfilter_c(prev: Option<&[u8]>, input: &[u8], out: &mut [u8], width: i32) {
    match prev {
        None => horizontal_unfilter_c(None, input, out, width),
        Some(prev) => {
            let width = usize::try_from(width).unwrap_or(0);
            for i in 0..width {
                out[i] = prev[i].wrapping_add(input[i]);
            }
        }
    }
}

// upstream: src/dsp/filters.c GradientUnfilter_C
fn gradient_unfilter_c(prev: Option<&[u8]>, input: &[u8], out: &mut [u8], width: i32) {
    match prev {
        None => horizontal_unfilter_c(None, input, out, width),
        Some(prev) => {
            let width = usize::try_from(width).unwrap_or(0);
            let mut top = prev[0];
            let mut top_left = top;
            let mut left = top;
            for i in 0..width {
                top = prev[i]; // need to read this first, in case prev==out
                left = input[i]
                    .wrapping_add(predictor_byte(gradient_predictor_c(left, top, top_left)));
                top_left = top;
                out[i] = left;
            }
        }
    }
}

// upstream: src/dsp/filters.c WebPFilters
// The table is filled by `VP8FiltersInit`, whose portable effect is to assign
// these pointers (ADR 0003). The NONE entry is NULL in C: no filter is applied
// to a plane filtered with WEBP_FILTER_NONE.
static WEBP_FILTERS: [Option<WebPFilterFunc>; 4] = [
    None,                      // WEBP_FILTER_NONE
    Some(horizontal_filter_c), // WEBP_FILTER_HORIZONTAL
    Some(vertical_filter_c),   // WEBP_FILTER_VERTICAL
    Some(gradient_filter_c),   // WEBP_FILTER_GRADIENT
];

// upstream: src/dsp/filters.c WebPUnfilters
static WEBP_UNFILTERS: [Option<WebPUnfilterFunc>; 4] = [
    Some(none_unfilter_c),       // WEBP_FILTER_NONE
    Some(horizontal_unfilter_c), // WEBP_FILTER_HORIZONTAL
    Some(vertical_unfilter_c),   // WEBP_FILTER_VERTICAL
    Some(gradient_unfilter_c),   // WEBP_FILTER_GRADIENT
];

// upstream: src/dsp/filters.c WebPFilters
/// Filters a plane with the filter `filter`, `WebPFilters[filter]`.
///
/// # Panics
///
/// Panics for `WEBP_FILTER_NONE`, whose entry is NULL in C, and for a value
/// outside the table. Calling a NULL entry is undefined behavior in C.
pub fn webp_filters(
    filter: WEBP_FILTER_TYPE,
    data: &[u8],
    width: i32,
    height: i32,
    stride: i32,
    filtered_data: &mut [u8],
) {
    let filter_fn =
        WEBP_FILTERS[filter.0 as usize].expect("WEBP_FILTER_NONE has no filter function");
    filter_fn(data, width, height, stride, filtered_data);
}

// upstream: src/dsp/filters.c WebPUnfilters
/// Reconstructs one row with the unfilter of `filter`, `WebPUnfilters[filter]`.
///
/// # Panics
///
/// Panics for a value outside the table.
pub fn webp_unfilters(
    filter: WEBP_FILTER_TYPE,
    prev: Option<&[u8]>,
    input: &[u8],
    out: &mut [u8],
    width: i32,
) {
    let unfilter_fn =
        WEBP_UNFILTERS[filter.0 as usize].expect("every filter has an unfilter function");
    unfilter_fn(prev, input, out, width);
}
