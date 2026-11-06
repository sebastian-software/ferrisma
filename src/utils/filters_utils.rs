// libwebp v1.6.0, src/utils/filters_utils.c:
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
// filter estimation
//
// Author: Urvang (urvang@google.com)

// libwebp v1.6.0, src/utils/filters_utils.h:
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

//! Translation of `src/utils/filters_utils.c` and `src/utils/filters_utils.h` from libwebp v1.6.0.
//!
//! Work package: WP4 (pixel ops).

use crate::dsp::dsp::WEBP_FILTER_TYPE;

// upstream: src/utils/filters_utils.c SMAX
/// The number of score bins: differences in `[0..SMAX)`.
const SMAX: usize = 16;

// upstream: src/utils/filters_utils.c SDIFF
/// The scoring difference of two samples, in `[0..SMAX)`.
fn sdiff(a: i32, b: i32) -> usize {
    usize::try_from((a - b).abs() >> 4).expect("a scoring difference is below SMAX")
}

// upstream: src/utils/filters_utils.c GradientPredictor
/// The gradient predictor `a + b - c`, clipped to a byte.
#[must_use]
fn gradient_predictor(a: u8, b: u8, c: u8) -> i32 {
    let g = i32::from(a) + i32::from(b) - i32::from(c);
    if (g & !0xff) == 0 {
        g
    } else if g < 0 {
        0
    } else {
        255
    }
}

// upstream: src/utils/filters_utils.c WebPEstimateBestFilter
/// A quick estimate of a potentially good filter for the plane `data`.
///
/// Every other pixel of every other row is sampled. Each filter scores the
/// sampled differences into bins, and the filter whose set of occupied bins
/// has the smallest sum wins; ties keep the earlier filter.
///
/// # Panics
///
/// Panics when `data` holds fewer than `height` rows of `stride` bytes.
pub fn webp_estimate_best_filter(
    data: &[u8],
    width: i32,
    height: i32,
    stride: i32,
) -> WEBP_FILTER_TYPE {
    let mut bins = [[0_i32; SMAX]; 4];
    let stride = isize::try_from(stride).expect("the stride fits in isize");
    let width = isize::try_from(width).expect("the width fits in isize");
    // We only sample every other pixels. That's enough.
    let mut j = 2_isize;
    while j < isize::try_from(height).expect("the height fits in isize") - 1 {
        let row = j * stride;
        let p =
            |index: isize| i32::from(data[usize::try_from(row + index).expect("inside the plane")]);
        let mut mean = p(0);
        let mut i = 2_isize;
        while i < width - 1 {
            let diff0 = sdiff(p(i), mean);
            let diff1 = sdiff(p(i), p(i - 1));
            let diff2 = sdiff(p(i), p(i - width));
            let grad_pred = gradient_predictor(
                u8::try_from(p(i - 1)).expect("a sample is a byte"),
                u8::try_from(p(i - width)).expect("a sample is a byte"),
                u8::try_from(p(i - width - 1)).expect("a sample is a byte"),
            );
            let diff3 = sdiff(p(i), grad_pred);
            bins[WEBP_FILTER_TYPE::WEBP_FILTER_NONE.0 as usize][diff0] = 1;
            bins[WEBP_FILTER_TYPE::WEBP_FILTER_HORIZONTAL.0 as usize][diff1] = 1;
            bins[WEBP_FILTER_TYPE::WEBP_FILTER_VERTICAL.0 as usize][diff2] = 1;
            bins[WEBP_FILTER_TYPE::WEBP_FILTER_GRADIENT.0 as usize][diff3] = 1;
            mean = (3 * mean + p(i) + 2) >> 2;
            i += 2;
        }
        j += 2;
    }
    let mut best_filter = WEBP_FILTER_TYPE::WEBP_FILTER_NONE;
    let mut best_score = 0x7fff_ffff_i32;
    for filter in [
        WEBP_FILTER_TYPE::WEBP_FILTER_NONE,
        WEBP_FILTER_TYPE::WEBP_FILTER_HORIZONTAL,
        WEBP_FILTER_TYPE::WEBP_FILTER_VERTICAL,
        WEBP_FILTER_TYPE::WEBP_FILTER_GRADIENT,
    ] {
        let mut score = 0_i32;
        for (i, &count) in bins[filter.0 as usize].iter().enumerate() {
            if count > 0 {
                score += i32::try_from(i).expect("a bin index fits in int");
            }
        }
        if score < best_score {
            best_score = score;
            best_filter = filter;
        }
    }
    best_filter
}
