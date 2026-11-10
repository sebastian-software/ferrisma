// libwebp v1.6.0, src/enc/near_lossless_enc.c:
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
// Near-lossless image preprocessing adjusts pixel values to help
// compressibility with a guarantee of maximum deviation between original and
// resulting pixel values.
//
// Author: Jyrki Alakuijala (jyrki@google.com)
// Converted to C by Aleksander Kramarz (akramarz@google.com)

//! Translation of `src/enc/near_lossless_enc.c` from libwebp v1.6.0.
//!
//! Work package: WP11 (VP8L encoder).
//!
//! Only the `WEBP_NEAR_LOSSLESS == 1` branch is translated: the reference
//! configuration enables near-lossless, and the other branch defines only the
//! empty `VP8LNearLosslessStub`.
//!
//! # Excluded undefined behavior
//!
//! Inputs that are undefined behavior (or assertion failures) in C and are
//! therefore excluded from the oracle comparison (PORTING rule 8):
//!
//! 1. `VP8ApplyNearLossless` with quality 100 (C asserts `limit_bits > 0`; the
//!    encoder calls it only below 100), `VP8LExtraCost` with an odd or short
//!    length, `VP8LResidualImage` and `VP8LColorSpaceTransform` with bits
//!    outside 2 to 9, and the prefix encodings with negative distances.

use crate::dsp::lossless_common::vp8l_near_lossless_bits;
use crate::enc::backward_references_enc::to_index;
use crate::enc::picture_enc::argb_samples;
use crate::utils::utils::webp_safe_malloc;
use crate::webp::encode::WebPPicture;

// upstream: src/enc/near_lossless_enc.c MIN_DIM_FOR_NEAR_LOSSLESS
const MIN_DIM_FOR_NEAR_LOSSLESS: i32 = 64;
// upstream: src/enc/near_lossless_enc.c MAX_LIMIT_BITS
const MAX_LIMIT_BITS: i32 = 5;

// upstream: src/enc/near_lossless_enc.c FindClosestDiscretized
/// Quantizes the value up or down to a multiple of 1<<bits (or to 255),
/// choosing the closer one, resolving ties using bankers' rounding.
fn find_closest_discretized(a: u32, bits: i32) -> u32 {
    let mask = (1_u32 << bits) - 1;
    let biased = a + (mask >> 1) + ((a >> bits) & 1);
    debug_assert!(bits > 0);
    if biased > 0xff {
        return 0xff;
    }
    biased & !mask
}

// upstream: src/enc/near_lossless_enc.c ClosestDiscretizedArgb
/// Applies `FindClosestDiscretized` to all channels of pixel.
fn closest_discretized_argb(a: u32, bits: i32) -> u32 {
    (find_closest_discretized(a >> 24, bits) << 24)
        | (find_closest_discretized((a >> 16) & 0xff, bits) << 16)
        | (find_closest_discretized((a >> 8) & 0xff, bits) << 8)
        | find_closest_discretized(a & 0xff, bits)
}

// upstream: src/enc/near_lossless_enc.c IsNear
/// Checks if distance between corresponding channel values of pixels a and b
/// is within the given limit.
#[expect(clippy::cast_possible_wrap, reason = "the channels are below 256")]
fn is_near(a: u32, b: u32, limit: i32) -> bool {
    for k in 0..4 {
        let delta = ((a >> (k * 8)) & 0xff) as i32 - ((b >> (k * 8)) & 0xff) as i32;
        if delta >= limit || delta <= -limit {
            return false;
        }
    }
    true
}

// upstream: src/enc/near_lossless_enc.c IsSmooth
/// The three rows are positions in `rows`.
fn is_smooth(
    rows: &[u32],
    prev_row: usize,
    curr_row: usize,
    next_row: usize,
    ix: usize,
    limit: i32,
) -> bool {
    // Check that all pixels in 4-connected neighborhood are smooth.
    is_near(rows[curr_row + ix], rows[curr_row + ix - 1], limit)
        && is_near(rows[curr_row + ix], rows[curr_row + ix + 1], limit)
        && is_near(rows[curr_row + ix], rows[prev_row + ix], limit)
        && is_near(rows[curr_row + ix], rows[next_row + ix], limit)
}

/// The source rows: the separate source, or the destination itself.
fn source<'s>(argb_src: Option<&'s [u32]>, argb_dst: &'s [u32]) -> &'s [u32] {
    argb_src.unwrap_or(argb_dst)
}

// upstream: src/enc/near_lossless_enc.c NearLossless
/// Adjusts pixel values of image with given maximum error. `argb_src` is
/// `None` where C passes `argb_dst` as the source too, with the stride `xsize`:
/// the image is then adjusted in place, and the copies of a row onto itself that
/// C makes are skipped.
fn near_lossless(
    xsize: i32,
    ysize: i32,
    argb_src: Option<&[u32]>,
    stride: i32,
    limit_bits: i32,
    copy_buffer: &mut [u32],
    argb_dst: &mut [u32],
) {
    let width = to_index(xsize);
    let stride = to_index(stride);
    let limit = 1 << limit_bits;
    let mut prev_row: usize = 0;
    let mut curr_row: usize = width;
    let mut next_row: usize = 2 * width;
    copy_buffer[curr_row..curr_row + width].copy_from_slice(&source(argb_src, argb_dst)[..width]);
    copy_buffer[next_row..next_row + width]
        .copy_from_slice(&source(argb_src, argb_dst)[stride..stride + width]);

    for y in 0..to_index(ysize) {
        let src = y * stride;
        let dst = y * width;
        if y == 0 || y == to_index(ysize) - 1 {
            if let Some(argb_src) = argb_src {
                argb_dst[dst..dst + width].copy_from_slice(&argb_src[src..src + width]);
            }
        } else {
            let next = src + stride;
            copy_buffer[next_row..next_row + width]
                .copy_from_slice(&source(argb_src, argb_dst)[next..next + width]);
            if let Some(argb_src) = argb_src {
                argb_dst[dst] = argb_src[src];
                argb_dst[dst + width - 1] = argb_src[src + width - 1];
            }
            for x in 1..width - 1 {
                argb_dst[dst + x] =
                    if is_smooth(copy_buffer, prev_row, curr_row, next_row, x, limit) {
                        copy_buffer[curr_row + x]
                    } else {
                        closest_discretized_argb(copy_buffer[curr_row + x], limit_bits)
                    };
            }
        }
        // Three-way swap.
        let temp = prev_row;
        prev_row = curr_row;
        curr_row = next_row;
        next_row = temp;
    }
}

// upstream: src/enc/near_lossless_enc.c VP8ApplyNearLossless
/// Near lossless preprocessing in RGB color-space. Returns `false` when the
/// copy buffer cannot be allocated; C records no error then, the caller does.
///
/// # Panics
///
/// Panics when the picture has no ARGB plane.
#[must_use]
pub fn vp8_apply_near_lossless(
    picture: &WebPPicture<'_>,
    quality: i32,
    argb_dst: &mut [u32],
) -> bool {
    let xsize = picture.width;
    let ysize = picture.height;
    let stride = picture.argb_stride;
    let Some(mut copy_buffer) = webp_safe_malloc::<u32>(i64::from(xsize * 3).cast_unsigned(), 4)
    else {
        return false;
    };
    copy_buffer.resize(to_index(xsize * 3), 0);
    let limit_bits = vp8l_near_lossless_bits(quality);
    debug_assert!(limit_bits > 0);
    debug_assert!(limit_bits <= MAX_LIMIT_BITS);
    let argb = argb_samples(
        &picture.argb_storage,
        picture.argb.expect("the picture has an ARGB plane"),
    );
    // For small icon images, don't attempt to apply near-lossless compression.
    if (xsize < MIN_DIM_FOR_NEAR_LOSSLESS && ysize < MIN_DIM_FOR_NEAR_LOSSLESS) || ysize < 3 {
        let width = to_index(xsize);
        for i in 0..to_index(ysize) {
            let src = i * to_index(picture.argb_stride);
            argb_dst[i * width..(i + 1) * width].copy_from_slice(&argb[src..src + width]);
        }
        return true;
    }

    near_lossless(
        xsize,
        ysize,
        Some(argb),
        stride,
        limit_bits,
        &mut copy_buffer,
        argb_dst,
    );
    let mut i = limit_bits - 1;
    while i != 0 {
        near_lossless(xsize, ysize, None, xsize, i, &mut copy_buffer, argb_dst);
        i -= 1;
    }
    true
}
