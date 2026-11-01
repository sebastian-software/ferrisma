// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.

//! Point, quantized storage/copy, and SplineDataView from lib/jxl/splines.h.
//! Spline::Point is named SplinePoint because Rust has no nested struct types.
//! Borrowed slices replace non-owning spans. Quantization, entropy decoding,
//! rendering, and draw-cache operations are absent until their algorithms land.

#[derive(Clone, Copy, Debug, Default)]
pub struct SplinePoint {
    pub x: f32,
    pub y: f32,
}
impl SplinePoint {
    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}
impl PartialEq for SplinePoint {
    fn eq(&self, other: &Self) -> bool {
        (self.x - other.x).abs() < 1e-3f32 && (self.y - other.y).abs() < 1e-3f32
    }
}
#[derive(Default)]
pub struct QuantizedSpline {
    control_points: Vec<(i64, i64)>,
    color_dct: [[i32; 32]; 3],
    sigma_dct: [i32; 32],
}
impl Clone for QuantizedSpline {
    fn clone(&self) -> Self {
        Self {
            control_points: self.control_points.clone(),
            color_dct: self.color_dct,
            sigma_dct: self.sigma_dct,
        }
    }
}
#[derive(Clone, Copy, Default)]
pub struct SplineDataView<'a> {
    pub splines: &'a [QuantizedSpline],
    pub starting_points: &'a [SplinePoint],
}
impl SplineDataView<'_> {
    pub fn has_any(&self) -> bool {
        !self.splines.is_empty()
    }
}
