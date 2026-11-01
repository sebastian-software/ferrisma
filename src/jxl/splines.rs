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

pub struct SplineSegment {
    pub center_x: f32,
    pub center_y: f32,
    pub maximum_distance: f32,
    pub inv_sigma: f32,
    pub sigma_over_4_times_intensity: f32,
    pub color: [f32; 3],
}
/// Storage, constructor, Clear, HasAny, and scalar/span getters from Splines.
/// Data import, decoding, rendering, and cache initialization remain absent.
pub struct Splines<'a> {
    memory_manager_present: bool,
    quantization_adjustment: i32,
    splines_storage: Vec<QuantizedSpline>,
    starting_points_storage: Vec<SplinePoint>,
    data: SplineDataView<'a>,
    segments: Vec<SplineSegment>,
    segment_indices: Option<super::memory_manager_internal::AlignedMemory>,
    segment_y_start: Option<super::memory_manager_internal::AlignedMemory>,
}
impl Default for Splines<'_> {
    fn default() -> Self {
        Self::new()
    }
}
impl<'a> Splines<'a> {
    pub fn new() -> Self {
        let mut value = Self {
            memory_manager_present: true,
            quantization_adjustment: 0,
            splines_storage: Vec::new(),
            starting_points_storage: Vec::new(),
            data: SplineDataView::default(),
            segments: Vec::new(),
            segment_indices: None,
            segment_y_start: None,
        };
        value.clear();
        value
    }
    pub fn clear(&mut self) {
        self.quantization_adjustment = 0;
        self.splines_storage.clear();
        self.starting_points_storage.clear();
        self.data = SplineDataView::default();
        self.segments.clear();
        self.segment_indices = None;
        self.segment_y_start = None;
    }
    pub fn has_any(&self) -> bool {
        self.data.has_any()
    }
    pub fn quantization_adjustment(&self) -> i32 {
        self.quantization_adjustment
    }
    pub fn quantized_splines(&self) -> &[QuantizedSpline] {
        self.data.splines
    }
    pub fn starting_points(&self) -> &[SplinePoint] {
        self.data.starting_points
    }
    pub fn have_memory_manager(&self) -> bool {
        self.memory_manager_present
    }
    pub fn storage_sizes(&self) -> [usize; 3] {
        [
            self.splines_storage.len(),
            self.starting_points_storage.len(),
            self.segments.len(),
        ]
    }
    pub fn have_segment_indices(&self) -> bool {
        self.segment_indices.is_some()
    }
    pub fn have_segment_y_start(&self) -> bool {
        self.segment_y_start.is_some()
    }
}
