// libwebp v1.6.0, src/dec/webp_dec.c:
//
// Copyright 2010 Google Inc. All Rights Reserved.
//
// Use of this source code is governed by a BSD-style license
// that can be found in the COPYING file in the root of the source
// tree. An additional intellectual property rights grant can be found
// in the file PATENTS. All contributing project authors may
// be found in the AUTHORS file in the root of the source tree.
// -----------------------------------------------------------------------------
//
// Main decoding functions for WEBP images.
//
// Author: Skal (pascal.massimino@gmail.com)

//! Translation of `src/dec/webp_dec.c` from libwebp v1.6.0.
//!
//! Work package: WP12 (decoder integration).
//!
//! Exception: WP7 translates `WebPIoInitFromOptions`, `WebPCheckCropDimensions` and `WebPCheckCropDimensionsBasic` from `src/dec/webp_dec.c`, because `VP8LDecodeImage` calls `WebPIoInitFromOptions`, which checks the crop with the other two.

// The items below are WP7's exception in this module (`tools/units.json`):
// `WebPIoInitFromOptions` and the two crop checks that it calls.

use crate::dec::vp8_dec::VP8Io;
use crate::utils::rescaler_utils::webp_rescaler_get_scaled_dimensions;
use crate::webp::decode::{WEBP_CSP_MODE, WebPDecoderOptions, webp_is_rgb_mode};

// upstream: src/dec/webp_dec.c WebPCheckCropDimensionsBasic
/// Returns true if the crop offset is not negative and its size is positive.
fn webp_check_crop_dimensions_basic(x: i32, y: i32, w: i32, h: i32) -> bool {
    !(x < 0 || y < 0 || w <= 0 || h <= 0)
}

// upstream: src/dec/webp_dec.c WebPCheckCropDimensions
/// Returns true if crop dimensions are within image bounds.
#[must_use]
pub fn webp_check_crop_dimensions(
    image_width: i32,
    image_height: i32,
    x: i32,
    y: i32,
    w: i32,
    h: i32,
) -> bool {
    webp_check_crop_dimensions_basic(x, y, w, h)
        && !(x >= image_width
            || w > image_width
            || w > image_width - x
            || y >= image_height
            || h > image_height
            || h > image_height - y)
}

// upstream: src/dec/webp_dec.c WebPIoInitFromOptions
/// Sets up the crop fields, `mb_w` and `mb_h` of `io`, and its scaling,
/// filtering and upsampling fields, from `options` (C: `NULL` is `None`).
/// `src_colorspace` refers to the compressed format, not the output one. Returns
/// false for a crop outside the picture or invalid scaled dimensions; the fields
/// set before the failing check keep their new values, as in C.
///
/// # Panics
///
/// Debug builds panic when `io.width * 3` or `io.height * 3` overflows with
/// scaling enabled, where the C `int` arithmetic is undefined.
#[must_use]
pub fn webp_io_init_from_options(
    options: Option<&WebPDecoderOptions>,
    io: &mut VP8Io,
    src_colorspace: WEBP_CSP_MODE,
) -> bool {
    let big_w = io.width;
    let big_h = io.height;
    let mut x = 0;
    let mut y = 0;
    let mut w = big_w;
    let mut h = big_h;

    // Cropping
    io.use_cropping = options.is_some_and(|options| options.use_cropping != 0);
    if let Some(options) = options.filter(|_| io.use_cropping) {
        w = options.crop_width;
        h = options.crop_height;
        x = options.crop_left;
        y = options.crop_top;
        if !webp_is_rgb_mode(src_colorspace) {
            // only snap for YUV420
            x &= !1;
            y &= !1;
        }
        if !webp_check_crop_dimensions(big_w, big_h, x, y, w, h) {
            return false; // out of frame boundary error
        }
    }
    io.crop_left = x;
    io.crop_top = y;
    io.crop_right = x + w;
    io.crop_bottom = y + h;
    io.mb_w = w;
    io.mb_h = h;

    // Scaling
    io.use_scaling = options.is_some_and(|options| options.use_scaling != 0);
    if let Some(options) = options.filter(|_| io.use_scaling) {
        let mut scaled_width = options.scaled_width;
        let mut scaled_height = options.scaled_height;
        if !webp_rescaler_get_scaled_dimensions(w, h, &mut scaled_width, &mut scaled_height) {
            return false;
        }
        io.scaled_width = scaled_width;
        io.scaled_height = scaled_height;
    }

    // Filter
    io.bypass_filtering = options.is_some_and(|options| options.bypass_filtering != 0);

    // Fancy upsampler (FANCY_UPSAMPLING is defined in src/dsp/dsp.h)
    io.fancy_upsampling = options.is_none_or(|options| options.no_fancy_upsampling == 0);

    if io.use_scaling {
        // disable filter (only for large downscaling ratio).
        io.bypass_filtering |=
            (io.scaled_width < big_w * 3 / 4) && (io.scaled_height < big_h * 3 / 4);
        io.fancy_upsampling = false;
    }
    true
}
