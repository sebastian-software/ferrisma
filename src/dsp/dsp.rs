// libwebp v1.6.0, src/dsp/dsp.h:
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
//   Speed-critical functions.
//
// Author: Skal (pascal.massimino@gmail.com)

//! Translation of `src/dsp/dsp.h` from libwebp v1.6.0.
//!
//! Work package: WP4 (pixel ops).

// upstream: src/dsp/dsp.h BPS
/// The common stride of the encoder and decoder blocks.
pub const BPS: i32 = 32;

// upstream: src/dsp/dsp.h MAX_COEFF_THRESH
/// Size of the histogram that `VP8CollectHistogram` uses.
pub const MAX_COEFF_THRESH: i32 = 31;

// upstream: src/dsp/dsp.h VP8Histogram
/// The statistics of one block that `VP8CollectHistogram` gathers. Only the
/// maximum value and the last non-zero position are kept.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct VP8Histogram {
    pub max_value: i32,
    pub last_non_zero: i32,
}

// upstream: src/dsp/dsp.h VP8DistoStats
/// Accumulated statistical moments of a window, for the SSIM computation.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct VP8DistoStats {
    /// `sum(w_i)`: sum of weights
    pub w: u32,
    /// `sum(w_i * x_i)` and `sum(w_i * y_i)`
    pub xm: u32,
    pub ym: u32,
    /// `sum(w_i * x_i * x_i)`, etc.
    pub xxm: u32,
    pub xym: u32,
    pub yym: u32,
}

// upstream: src/dsp/dsp.h VP8_SSIM_KERNEL
/// Half the size of the SSIM kernel: the kernel is `2 * VP8_SSIM_KERNEL + 1` wide.
pub const VP8_SSIM_KERNEL: i32 = 3;

// upstream: src/dsp/dsp.h VP8_DITHER_DESCALE
/// Descaling of the dithering values.
pub const VP8_DITHER_DESCALE: i32 = 4;
// upstream: src/dsp/dsp.h VP8_DITHER_DESCALE_ROUNDER
/// Rounding term of the dithering descaling.
pub const VP8_DITHER_DESCALE_ROUNDER: i32 = 1 << (VP8_DITHER_DESCALE - 1);
// upstream: src/dsp/dsp.h VP8_DITHER_AMP_BITS
/// Bits of the dithering amplitude.
pub const VP8_DITHER_AMP_BITS: i32 = 7;
// upstream: src/dsp/dsp.h VP8_DITHER_AMP_CENTER
/// Center of the dithering amplitude.
pub const VP8_DITHER_AMP_CENTER: i32 = 1 << VP8_DITHER_AMP_BITS;

// upstream: src/dsp/dsp.h WEBP_TRANSFORM_AC3_C1
/// Fixed-point constant of the AC3 inverse transform (`sqrt(2) * cos(pi/8)`).
pub const WEBP_TRANSFORM_AC3_C1: i32 = 20091;
// upstream: src/dsp/dsp.h WEBP_TRANSFORM_AC3_C2
/// Fixed-point constant of the AC3 inverse transform (`sqrt(2) * sin(pi/8)`).
pub const WEBP_TRANSFORM_AC3_C2: i32 = 35468;

// upstream: src/dsp/dsp.h WEBP_TRANSFORM_AC3_MUL1
/// `a * C1 >> 16` plus `a`, as the AC3 transform computes it.
#[must_use]
pub fn webp_transform_ac3_mul1(a: i32) -> i32 {
    ((a * WEBP_TRANSFORM_AC3_C1) >> 16) + a
}

// upstream: src/dsp/dsp.h WEBP_TRANSFORM_AC3_MUL2
/// `a * C2 >> 16`, as the AC3 transform computes it.
#[must_use]
pub fn webp_transform_ac3_mul2(a: i32) -> i32 {
    (a * WEBP_TRANSFORM_AC3_C2) >> 16
}

// upstream: src/dsp/dsp.h WEBP_FILTER_TYPE
/// A filter type of the lossless and alpha coders. The C enumeration has no
/// negative values, so it wraps `u32` (ADR 0003).
#[must_use]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[expect(non_camel_case_types, reason = "C type name")]
pub struct WEBP_FILTER_TYPE(pub u32);

impl WEBP_FILTER_TYPE {
    // upstream: src/dsp/dsp.h WEBP_FILTER_NONE
    pub const WEBP_FILTER_NONE: Self = Self(0);
    // upstream: src/dsp/dsp.h WEBP_FILTER_HORIZONTAL
    pub const WEBP_FILTER_HORIZONTAL: Self = Self(1);
    // upstream: src/dsp/dsp.h WEBP_FILTER_VERTICAL
    pub const WEBP_FILTER_VERTICAL: Self = Self(2);
    // upstream: src/dsp/dsp.h WEBP_FILTER_GRADIENT
    pub const WEBP_FILTER_GRADIENT: Self = Self(3);
    // upstream: src/dsp/dsp.h WEBP_FILTER_LAST
    /// End marker: the number of filter types.
    pub const WEBP_FILTER_LAST: Self = Self(4);
    // upstream: src/dsp/dsp.h WEBP_FILTER_BEST
    /// Meta-type: the best of the filters.
    pub const WEBP_FILTER_BEST: Self = Self(5);
    // upstream: src/dsp/dsp.h WEBP_FILTER_FAST
    /// Meta-type: a quick choice of a filter.
    pub const WEBP_FILTER_FAST: Self = Self(6);
}
