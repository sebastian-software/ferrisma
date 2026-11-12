// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.

//! Complete scalar translation of lib/jxl/splines.{h,cc}. Borrowed external
//! SplineDataView and an owned-data selector replace self-referential spans.
//! AlignedMemory owners from the spline memory manager hold the source's index
//! arrays, viewed as typed slices. The public getters expose only
//! initialized source state. SIMD targets and enc_splines.cc are separate units.

use super::base::bits::ceil_log2_nonzero;
use super::base::float::{clamp_f32, max_f32, min_f32, mul_add_f32};
use super::base::rect::Rect;
use super::base::status::{Status, StatusCode, StatusOr};
use super::chroma_from_luma::ColorCorrelation;
use super::dec_ans::{ANSCode, ANSSymbolReader, decode_histograms};
use super::dec_bit_reader::BitReader;
use super::image::Image3F;
use super::memory_manager_internal::{AlignedMemory, MemoryManagerPtr};
use super::pack_signed::unpack_signed;

pub const DESIRED_RENDERING_DISTANCE: f32 = 1.0;
pub type Dct32 = [f32; 32];
const CHANNEL_WEIGHT: [f32; 4] = [0.0042, 0.075, 0.07, 0.3333];
const SQRT_2: f32 = std::f32::consts::SQRT_2;
const SQRT_0_5: f32 = std::f32::consts::FRAC_1_SQRT_2;
const PI: f32 = std::f32::consts::PI;
const NUM_SPLINE_CONTEXTS: usize = 6;
const QUANTIZATION_ADJUSTMENT_CONTEXT: usize = 0;
const STARTING_POSITION_CONTEXT: usize = 1;
const NUM_SPLINES_CONTEXT: usize = 2;
const NUM_CONTROL_POINTS_CONTEXT: usize = 3;
const CONTROL_POINTS_CONTEXT: usize = 4;
const DCT_CONTEXT: usize = 5;

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
#[derive(Clone, Debug, Default)]
pub struct Spline {
    pub control_points: Vec<SplinePoint>,
    pub color_dct: [Dct32; 3],
    pub sigma_dct: Dct32,
}
#[derive(Clone, Debug, Default)]
pub struct QuantizedSpline {
    control_points: Vec<(i64, i64)>,
    color_dct: [[i32; 32]; 3],
    sigma_dct: [i32; 32],
}
fn validate_pos(x: f32, y: f32) -> Status {
    if x >= 8388608.0 || x <= -8388608.0 || y >= 8388608.0 || y <= -8388608.0 {
        Err(StatusCode::GenericError)
    } else {
        Ok(())
    }
}
fn validate_pos_int(x: i64, y: i64) -> Status {
    if x >= 1 << 23 || x <= -(1 << 23) || y >= 1 << 23 || y <= -(1 << 23) {
        Err(StatusCode::GenericError)
    } else {
        Ok(())
    }
}
fn adjusted_quant(a: i32) -> f32 {
    if a >= 0 {
        mul_add_f32(0.125, a as f32, 1.0)
    } else {
        1.0 / mul_add_f32(-0.125, a as f32, 1.0)
    }
}
fn inv_adjusted_quant(a: i32) -> f32 {
    if a >= 0 {
        1.0 / mul_add_f32(0.125, a as f32, 1.0)
    } else {
        mul_add_f32(-0.125, a as f32, 1.0)
    }
}
impl QuantizedSpline {
    pub fn control_points(&self) -> &[(i64, i64)] {
        &self.control_points
    }
    pub fn color_dct(&self) -> &[[i32; 32]; 3] {
        &self.color_dct
    }
    pub fn sigma_dct(&self) -> &[i32; 32] {
        &self.sigma_dct
    }
    pub fn create(original: &Spline, adjustment: i32, y_to_x: f32, y_to_b: f32) -> StatusOr<Self> {
        if original.control_points.is_empty() {
            return Err(StatusCode::GenericError);
        }
        let mut result = Self::default();
        result
            .control_points
            .reserve(original.control_points.len() - 1);
        // As in the source, caller-provided points must round into int32.
        let mut previous_x = original.control_points[0].x.round() as i32;
        let mut previous_y = original.control_points[0].y.round() as i32;
        let (mut previous_delta_x, mut previous_delta_y) = (0, 0);
        for p in &original.control_points[1..] {
            let new_x = p.x.round() as i32;
            let new_y = p.y.round() as i32;
            let dx = new_x - previous_x;
            let dy = new_y - previous_y;
            result.control_points.push((
                i64::from(dx - previous_delta_x),
                i64::from(dy - previous_delta_y),
            ));
            previous_delta_x = dx;
            previous_delta_y = dy;
            previous_x = new_x;
            previous_y = new_y;
        }
        let to_int = |v| clamp_f32(v, -2147483520.0, 2147483520.0).round() as i32;
        let quant = adjusted_quant(adjustment);
        let inv_quant = inv_adjusted_quant(adjustment);
        for c in [1, 0, 2] {
            let factor = if c == 0 {
                y_to_x
            } else if c == 1 {
                0.0
            } else {
                y_to_b
            };
            for i in 0..32 {
                let dct_factor = if i == 0 { SQRT_2 } else { 1.0 };
                let inv_dct_factor = if i == 0 { SQRT_0_5 } else { 1.0 };
                let restored_y =
                    result.color_dct[1][i] as f32 * inv_dct_factor * CHANNEL_WEIGHT[1] * inv_quant;
                let decorrelated = mul_add_f32(-factor, restored_y, original.color_dct[c][i]);
                result.color_dct[c][i] =
                    to_int(decorrelated * dct_factor * quant / CHANNEL_WEIGHT[c]);
            }
        }
        for i in 0..32 {
            let dct_factor = if i == 0 { SQRT_2 } else { 1.0 };
            result.sigma_dct[i] =
                to_int(original.sigma_dct[i] * dct_factor * quant / CHANNEL_WEIGHT[3]);
        }
        Ok(result)
    }
    #[expect(
        clippy::too_many_arguments,
        reason = "Retain the original dequantization and area-accounting boundary."
    )]
    #[allow(clippy::needless_range_loop, reason = "Keep the source index loops.")]
    pub fn dequantize(
        &self,
        start: SplinePoint,
        adjustment: i32,
        y_to_x: f32,
        y_to_b: f32,
        image_size: u64,
        total_area: &mut u64,
        result: &mut Spline,
    ) -> Status {
        let area_limit =
            (1024u64.wrapping_mul(image_size).wrapping_add(1u64 << 32)).min(1u64 << 42);
        result.control_points.clear();
        result.control_points.reserve(self.control_points.len() + 1);
        let px = start.x.round();
        let py = start.y.round();
        validate_pos(px, py)?;
        let (mut x, mut y) = (px as i32, py as i32);
        result
            .control_points
            .push(SplinePoint::new(x as f32, y as f32));
        let (mut dx, mut dy) = (0i32, 0i32);
        let mut distance = 0u64;
        for &(ddx, ddy) in &self.control_points {
            dx = (i64::from(dx) + ddx) as i32;
            dy = (i64::from(dy) + ddy) as i32;
            // Defined source inputs exclude abs(INT_MIN) and signed overflow.
            distance = distance.wrapping_add((dx.abs() + dy.abs()) as u64);
            if distance > area_limit {
                return Err(StatusCode::GenericError);
            }
            validate_pos_int(i64::from(dx), i64::from(dy))?;
            x += dx;
            y += dy;
            validate_pos_int(i64::from(x), i64::from(y))?;
            result
                .control_points
                .push(SplinePoint::new(x as f32, y as f32));
        }
        let inv_quant = inv_adjusted_quant(adjustment);
        for (c, &weight) in CHANNEL_WEIGHT[..3].iter().enumerate() {
            for i in 0..32 {
                let inv_dct_factor = if i == 0 { SQRT_0_5 } else { 1.0 };
                result.color_dct[c][i] =
                    self.color_dct[c][i] as f32 * inv_dct_factor * weight * inv_quant;
            }
        }
        for i in 0..32 {
            result.color_dct[0][i] =
                mul_add_f32(y_to_x, result.color_dct[1][i], result.color_dct[0][i]);
            result.color_dct[2][i] =
                mul_add_f32(y_to_b, result.color_dct[1][i], result.color_dct[2][i]);
        }
        let mut color = [0u64; 3];
        for (c, value) in color.iter_mut().enumerate() {
            for i in 0..32 {
                *value = value
                    .wrapping_add((inv_quant * self.color_dct[c][i].abs() as f32).ceil() as u64);
            }
        }
        color[0] = color[0].wrapping_add((y_to_x.abs().ceil() as u64).wrapping_mul(color[1]));
        color[2] = color[2].wrapping_add((y_to_b.abs().ceil() as u64).wrapping_mul(color[1]));
        let max_color = color[1].max(color[0]).max(color[2]);
        let logcolor = (ceil_log2_nonzero(1u64.wrapping_add(max_color)) as u64).max(1);
        let weight_limit = ((area_limit as f32 / logcolor as f32) / distance.max(1) as f32)
            .sqrt()
            .ceil();
        let mut width = 0u64;
        for i in 0..32 {
            let inv_dct_factor = if i == 0 { SQRT_0_5 } else { 1.0 };
            result.sigma_dct[i] =
                self.sigma_dct[i] as f32 * inv_dct_factor * CHANNEL_WEIGHT[3] * inv_quant;
            let weight_f = (inv_quant * self.sigma_dct[i].abs() as f32).ceil();
            let weight = min_f32(weight_limit, max_f32(1.0, weight_f)) as u64;
            width = width.wrapping_add(weight.wrapping_mul(weight).wrapping_mul(logcolor));
        }
        *total_area = total_area.wrapping_add(width.wrapping_mul(distance));
        if *total_area > area_limit {
            return Err(StatusCode::GenericError);
        }
        Ok(())
    }
    pub fn decode(
        &mut self,
        context_map: &[u8],
        decoder: &mut ANSSymbolReader<'_>,
        br: &mut BitReader<'_>,
        max: usize,
        total: &mut usize,
    ) -> Status {
        let n = decoder.read_hybrid_uint(NUM_CONTROL_POINTS_CONTEXT, br, context_map);
        if n > max {
            return Err(StatusCode::GenericError);
        }
        *total += n;
        if *total > max {
            return Err(StatusCode::GenericError);
        }
        self.control_points.resize(n, (0, 0));
        for point in &mut self.control_points {
            point.0 = i64::from(unpack_signed(decoder.read_hybrid_uint(
                CONTROL_POINTS_CONTEXT,
                br,
                context_map,
            )));
            point.1 = i64::from(unpack_signed(decoder.read_hybrid_uint(
                CONTROL_POINTS_CONTEXT,
                br,
                context_map,
            )));
            if point.0 >= 1 << 30
                || point.0 <= -(1 << 30)
                || point.1 >= 1 << 30
                || point.1 <= -(1 << 30)
            {
                return Err(StatusCode::GenericError);
            }
        }
        for dct in self
            .color_dct
            .iter_mut()
            .chain(std::iter::once(&mut self.sigma_dct))
        {
            for v in dct {
                *v = unpack_signed(decoder.read_hybrid_uint(DCT_CONTEXT, br, context_map));
                if *v == i32::MIN {
                    return Err(StatusCode::GenericError);
                }
            }
        }
        Ok(())
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
#[derive(Clone, Debug)]
pub struct SplineSegment {
    pub center_x: f32,
    pub center_y: f32,
    pub maximum_distance: f32,
    pub inv_sigma: f32,
    pub sigma_over_4_times_intensity: f32,
    pub color: [f32; 3],
}
pub struct Splines<'a> {
    memory_manager: MemoryManagerPtr,
    quantization_adjustment: i32,
    splines_storage: Vec<QuantizedSpline>,
    starting_points_storage: Vec<SplinePoint>,
    data: SplineDataView<'a>,
    owned_data: bool,
    owned_data_sizes: [usize; 2],
    segments: Vec<SplineSegment>,
    segment_indices: Option<AlignedMemory>,
    segment_y_start: Option<AlignedMemory>,
}
impl Default for Splines<'_> {
    fn default() -> Self {
        Self::new()
    }
}
impl<'a> Splines<'a> {
    pub fn new() -> Self {
        Self::new_in(MemoryManagerPtr::NATIVE)
    }
    /// The source Splines(JxlMemoryManager*) constructor.
    pub fn new_in(memory_manager: MemoryManagerPtr) -> Self {
        Self {
            memory_manager,
            quantization_adjustment: 0,
            splines_storage: Vec::new(),
            starting_points_storage: Vec::new(),
            data: SplineDataView::default(),
            owned_data: false,
            owned_data_sizes: [0; 2],
            segments: Vec::new(),
            segment_indices: None,
            segment_y_start: None,
        }
    }
    pub fn clear(&mut self) {
        self.quantization_adjustment = 0;
        self.splines_storage.clear();
        self.starting_points_storage.clear();
        self.data = SplineDataView::default();
        self.owned_data = false;
        self.owned_data_sizes = [0; 2];
        self.segments.clear();
        self.segment_indices = None;
        self.segment_y_start = None;
    }
    pub fn set_data(&mut self, data: SplineDataView<'a>) {
        self.clear();
        self.data = data;
    }
    pub fn has_any(&self) -> bool {
        self.data_sizes()[0] != 0
    }
    /// Source span lengths remain observable after a failed repeated decode,
    /// even when the old source spans can no longer be dereferenced safely.
    pub fn data_sizes(&self) -> [usize; 2] {
        if self.owned_data {
            self.owned_data_sizes
        } else {
            [self.data.splines.len(), self.data.starting_points.len()]
        }
    }
    pub fn quantization_adjustment(&self) -> i32 {
        self.quantization_adjustment
    }
    pub fn quantized_splines(&self) -> &[QuantizedSpline] {
        if self.owned_data {
            &self.splines_storage[..self.owned_data_sizes[0]]
        } else {
            self.data.splines
        }
    }
    pub fn starting_points(&self) -> &[SplinePoint] {
        if self.owned_data {
            &self.starting_points_storage[..self.owned_data_sizes[1]]
        } else {
            self.data.starting_points
        }
    }
    pub fn have_memory_manager(&self) -> bool {
        self.memory_manager.is_some()
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
    pub fn segments(&self) -> &[SplineSegment] {
        &self.segments
    }
    pub fn segment_indices(&self) -> Option<&[usize]> {
        self.segment_indices.as_ref().map(AlignedMemory::usizes)
    }
    pub fn segment_y_start(&self) -> Option<&[isize]> {
        self.segment_y_start.as_ref().map(AlignedMemory::isizes)
    }
    pub fn stored_splines(&self) -> &[QuantizedSpline] {
        &self.splines_storage
    }
    pub fn stored_starting_points(&self) -> &[SplinePoint] {
        &self.starting_points_storage
    }
    pub fn decode(&mut self, br: &mut BitReader<'_>, num_pixels: usize) -> Status {
        let mut context_map = Vec::new();
        let mut code = ANSCode::default();
        decode_histograms(
            self.memory_manager,
            br,
            NUM_SPLINE_CONTEXTS,
            &mut code,
            &mut context_map,
            false,
        )?;
        let mut decoder = ANSSymbolReader::create(&code, br, 0)?;
        let mut n = decoder.read_hybrid_uint(NUM_SPLINES_CONTEXT, br, &context_map);
        let max = (1usize << 20).min(num_pixels / 2);
        if n > max || n + 1 > max {
            return Err(StatusCode::GenericError);
        }
        n += 1;
        self.starting_points_storage.clear();
        self.starting_points_storage.reserve(n);
        let (mut last_x, mut last_y) = (0i64, 0i64);
        for i in 0..n {
            let dx = decoder.read_hybrid_uint(STARTING_POSITION_CONTEXT, br, &context_map);
            let dy = decoder.read_hybrid_uint(STARTING_POSITION_CONTEXT, br, &context_map);
            let x = if i != 0 {
                i64::from(unpack_signed(dx)) + last_x
            } else {
                dx as i64
            };
            let y = if i != 0 {
                i64::from(unpack_signed(dy)) + last_y
            } else {
                dy as i64
            };
            validate_pos_int(x, y)?;
            self.starting_points_storage
                .push(SplinePoint::new(x as f32, y as f32));
            last_x = x;
            last_y = y;
        }
        self.quantization_adjustment = unpack_signed(decoder.read_hybrid_uint(
            QUANTIZATION_ADJUSTMENT_CONTEXT,
            br,
            &context_map,
        ));
        self.splines_storage.clear();
        self.splines_storage.reserve(n);
        let mut total = n;
        for _ in 0..n {
            let mut spline = QuantizedSpline::default();
            spline.decode(&context_map, &mut decoder, br, max, &mut total)?;
            self.splines_storage.push(spline);
        }
        if !decoder.check_ans_final_state() {
            return Err(StatusCode::GenericError);
        }
        self.owned_data = true;
        self.owned_data_sizes = [
            self.splines_storage.len(),
            self.starting_points_storage.len(),
        ];
        if !self.has_any() {
            return Err(StatusCode::GenericError);
        }
        Ok(())
    }
    pub fn initialize_draw_cache(
        &mut self,
        width: usize,
        height: usize,
        correlation: &ColorCorrelation,
    ) -> Status {
        self.segments.clear();
        self.segment_indices = None;
        self.segment_y_start = None;
        let mut spans = Vec::new();
        let mut intermediate_points = Vec::new();
        let mut total_area = 0u64;
        let mut splines = Vec::new();
        for i in 0..self.quantized_splines().len() {
            let mut spline = Spline::default();
            self.quantized_splines()[i].dequantize(
                self.starting_points()[i],
                self.quantization_adjustment,
                correlation.y_to_x_ratio(0),
                correlation.y_to_b_ratio(0),
                width.wrapping_mul(height) as u64,
                &mut total_area,
                &mut spline,
            )?;
            if spline.control_points.windows(2).any(|p| p[0] == p[1]) {
                return Err(StatusCode::GenericError);
            }
            splines.push(spline);
        }
        // Source's large-area warning does not change non-fuzzing status.
        for spline in &splines {
            let mut points_to_draw = Vec::new();
            intermediate_points.clear();
            draw_centripetal_catmull_rom_spline(
                spline.control_points.clone(),
                &mut intermediate_points,
            );
            for_each_equally_spaced_point(&intermediate_points, |p, m| {
                points_to_draw.push((p, m))
            })?;
            let arc_length = mul_add_f32(
                points_to_draw.len().wrapping_sub(2) as f32,
                DESIRED_RENDERING_DISTANCE,
                points_to_draw.last().unwrap().1,
            );
            if arc_length <= 0.0 {
                continue;
            }
            segments_from_points(
                height,
                spline,
                &points_to_draw,
                arc_length,
                &mut self.segments,
                &mut spans,
            );
        }
        // AlignedMemory::Create zero-initializes the source's memset range.
        let segment_y_start_num_bytes = height
            .wrapping_add(2)
            .wrapping_mul(std::mem::size_of::<isize>());
        self.segment_y_start = Some(AlignedMemory::create(
            self.memory_manager,
            segment_y_start_num_bytes,
            0,
        )?);
        let starts = self.segment_y_start.as_mut().unwrap().isizes_mut();
        for &(start, end) in &spans {
            starts[start + 1] += 1;
            starts[end + 1] -= 1;
        }
        let (mut total, mut coverage) = (0usize, 0isize);
        for y in 0..height {
            if starts[y + 1] < 0 && coverage < -starts[y + 1] {
                return Err(StatusCode::GenericError);
            }
            coverage += starts[y + 1];
            starts[y + 1] = total as isize;
            total += coverage as usize;
        }
        self.segment_indices = Some(AlignedMemory::create(
            self.memory_manager,
            total.wrapping_mul(std::mem::size_of::<usize>()),
            0,
        )?);
        let starts = self.segment_y_start.as_mut().unwrap().isizes_mut();
        let indices = self.segment_indices.as_mut().unwrap().usizes_mut();
        for (i, &(start, end)) in spans.iter().enumerate() {
            for y in start..end {
                indices[starts[y + 1] as usize] = i;
                starts[y + 1] += 1;
            }
        }
        Ok(())
    }
    pub fn add_to_row(
        &self,
        row_x: &mut [f32],
        row_y: &mut [f32],
        row_b: &mut [f32],
        y: usize,
        x0: usize,
        x1: usize,
    ) {
        self.apply_to_row::<true>([row_x, row_y, row_b], y, x0, x1);
    }
    pub fn apply_to_row<const ADD: bool>(
        &self,
        mut rows: [&mut [f32]; 3],
        y: usize,
        x0: usize,
        x1: usize,
    ) {
        if self.segments.is_empty() {
            return;
        }
        let starts = self.segment_y_start.as_ref().unwrap().isizes();
        let indices = self.segment_indices.as_ref().unwrap().usizes();
        for i in starts[y]..starts[y + 1] {
            draw_segment(
                &self.segments[indices[i as usize]],
                ADD,
                y,
                x0,
                x1,
                &mut rows,
            );
        }
    }
    pub fn add_to(&self, opsin: &mut Image3F, rect: Rect) {
        self.apply::<true>(opsin, rect);
    }
    pub fn subtract_from(&self, opsin: &mut Image3F) {
        self.apply::<false>(opsin, Rect::new(0, 0, opsin.xsize(), opsin.ysize()));
    }
    fn apply<const ADD: bool>(&self, opsin: &mut Image3F, rect: Rect) {
        if self.segments.is_empty() {
            return;
        }
        for y in 0..rect.ysize() {
            let [xplane, yplane, bplane] = opsin.planes_mut();
            self.apply_to_row::<ADD>(
                [
                    &mut xplane.row_mut(rect.y0() + y)[rect.x0()..],
                    &mut yplane.row_mut(rect.y0() + y)[rect.x0()..],
                    &mut bplane.row_mut(rect.y0() + y)[rect.x0()..],
                ],
                rect.y0() + y,
                rect.x0(),
                rect.x1(),
            );
        }
    }
}

#[derive(Clone, Copy)]
struct Vector {
    x: f32,
    y: f32,
}
fn delta(a: SplinePoint, b: SplinePoint) -> Vector {
    Vector {
        x: a.x - b.x,
        y: a.y - b.y,
    }
}
fn add_scaled(p: SplinePoint, k: f32, v: Vector) -> SplinePoint {
    // Source operator* and operator+ are distinct calls: no contraction.
    SplinePoint::new(p.x + k * v.x, p.y + k * v.y)
}
fn draw_centripetal_catmull_rom_spline(
    mut points: Vec<SplinePoint>,
    result: &mut Vec<SplinePoint>,
) {
    if points.is_empty() {
        return;
    }
    if points.len() == 1 {
        result.push(points[0]);
        return;
    }
    result.reserve((points.len() - 1) * 16 + 1);
    points.insert(0, add_scaled(points[0], 1.0, delta(points[0], points[1])));
    let n = points.len();
    points.push(add_scaled(
        points[n - 1],
        1.0,
        delta(points[n - 1], points[n - 2]),
    ));
    for start in 0..points.len() - 3 {
        let p = &points[start..];
        result.push(p[1]);
        let mut d = [0f32; 3];
        let mut t = [0f32; 4];
        for k in 0..3 {
            d[k] = (p[k + 1].x - p[k].x).hypot(p[k + 1].y - p[k].y).sqrt();
            t[k + 1] = t[k] + d[k];
        }
        for i in 1..16 {
            let tt = mul_add_f32(i as f32 / 16.0, d[1], d[0]);
            let mut a = [SplinePoint::default(); 3];
            for k in 0..3 {
                a[k] = add_scaled(p[k], (tt - t[k]) / d[k], delta(p[k + 1], p[k]));
            }
            let mut b = [SplinePoint::default(); 2];
            for k in 0..2 {
                b[k] = add_scaled(a[k], (tt - t[k]) / (d[k] + d[k + 1]), delta(a[k + 1], a[k]));
            }
            result.push(add_scaled(b[0], (tt - t[1]) / d[1], delta(b[1], b[0])));
        }
    }
    result.push(points[points.len() - 2]);
}
fn for_each_equally_spaced_point(
    points: &[SplinePoint],
    mut functor: impl FnMut(SplinePoint, f32),
) -> Status {
    if points.is_empty() {
        return Err(StatusCode::GenericError);
    }
    let mut current = points[0];
    functor(current, DESIRED_RENDERING_DISTANCE);
    let mut next = 0;
    while next != points.len() {
        let mut previous = current;
        let mut arclength = 0f32;
        loop {
            if next == points.len() {
                functor(previous, arclength);
                return Ok(());
            }
            let v = delta(points[next], previous);
            let length = mul_add_f32(v.x, v.x, v.y * v.y).sqrt();
            if arclength + length >= DESIRED_RENDERING_DISTANCE {
                current = add_scaled(
                    previous,
                    (DESIRED_RENDERING_DISTANCE - arclength) / length,
                    delta(points[next], previous),
                );
                functor(current, DESIRED_RENDERING_DISTANCE);
                break;
            }
            arclength += length;
            previous = points[next];
            next += 1;
        }
    }
    Ok(())
}
// Copyright 2019 Google LLC and the Highway Project Authors. BSD-3-Clause;
// see licenses/HIGHWAY-BSD-3-Clause.txt. These helpers preserve scalar-inl.h's
// double-promoted multiplication and unfused MulAdd/MulSub/NegMulAdd.
fn hmul(a: f32, b: f32) -> f32 {
    (f64::from(a) * f64::from(b)) as f32
}
fn hma(a: f32, b: f32, c: f32) -> f32 {
    hmul(a, b) + c
}
// Used FastCosf/FastErff from lib/jxl/base/fast_math-inl.h.
#[expect(
    clippy::excessive_precision,
    reason = "Keep the original polynomial constants directly traceable to fast_math-inl.h."
)]
fn fast_cos(x: f32) -> f32 {
    let pi2 = PI * 2.0;
    let pi2_inv = 0.5 / PI;
    let npi2 = hmul(super::base::hwy_scalar::floor(hmul(x, pi2_inv)), pi2);
    let xmodpi2 = x - npi2;
    let x_pi = super::base::hwy_scalar::min(xmodpi2, pi2 - xmodpi2);
    let above = x_pi >= PI / 2.0;
    let x_pihalf = if above { PI - x_pi } else { x_pi };
    let xs = hmul(x_pihalf, 0.25);
    let x2 = hmul(xs, xs);
    let x4 = hmul(x2, x2);
    let c0 = hma(x4, 0.06960438, hma(x2, -0.84087373, 1.68179268));
    let c1 = hma(c0, c0, -SQRT_2);
    let c2 = hma(c1, c1, -1.0);
    f32::from_bits(c2.to_bits() ^ if above { 1u32 << 31 } else { 0 })
}
#[expect(
    clippy::excessive_precision,
    reason = "Keep the original polynomial constants directly traceable to fast_math-inl.h."
)]
fn fast_erf(x: f32) -> f32 {
    let abs = x.abs();
    let d1 = hma(abs, 7.77394369e-2, 2.05260015e-4);
    let d2 = hma(d1, abs, 2.32120216e-1);
    let d3 = hma(d2, abs, 2.77820801e-1);
    let d4 = hma(d3, abs, 1.0);
    let d5 = hmul(d4, d4);
    let inv = 1.0 / d5;
    let result = 1.0 - hmul(inv, inv);
    f32::from_bits(result.to_bits() ^ if x <= 0.0 { 1u32 << 31 } else { 0 })
}
fn continuous_idct(dct: &Dct32, t: f32) -> f32 {
    let mut result = 0.0;
    for (i, &v) in dct.iter().enumerate() {
        let arg = hmul((std::f64::consts::PI / 32.0 * i as f64) as f32, t + 0.5);
        let local = hmul(v, fast_cos(arg));
        result = hma(SQRT_2, local, result);
    }
    result
}
fn segments_from_points(
    height: usize,
    spline: &Spline,
    points: &[(SplinePoint, f32)],
    arc: f32,
    segments: &mut Vec<SplineSegment>,
    spans: &mut Vec<(usize, usize)>,
) {
    let inv_arc = 1.0 / arc;
    for (k, &(point, multiplier)) in points.iter().enumerate() {
        let progress = min_f32(
            1.0,
            (k as i32 as f32 * DESIRED_RENDERING_DISTANCE) * inv_arc,
        );
        let color = std::array::from_fn(|c| continuous_idct(&spline.color_dct[c], 31.0 * progress));
        let sigma = continuous_idct(&spline.sigma_dct, 31.0 * progress);
        compute_segments(height, point, multiplier, color, sigma, segments, spans);
    }
}
fn compute_segments(
    height: usize,
    center: SplinePoint,
    intensity: f32,
    color: [f32; 3],
    sigma: f32,
    segments: &mut Vec<SplineSegment>,
    spans: &mut Vec<(usize, usize)>,
) {
    if !(sigma.is_finite() && sigma != 0.0 && (1.0 / sigma).is_finite() && intensity.is_finite()) {
        return;
    }
    let mut max_color = 0.01f32;
    for c in color {
        max_color = max_f32(max_color, (c * intensity).abs());
    }
    let maximum_distance = (-2.0 * sigma * sigma * (0.1f32.ln() * 5.0 - max_color.ln())).sqrt();
    let y0 = ((center.y - maximum_distance).round() as isize).max(0);
    let y1 = ((center.y + maximum_distance).round() as isize + 1).min(height as isize);
    if y1 <= y0 {
        return;
    }
    segments.push(SplineSegment {
        center_x: center.x,
        center_y: center.y,
        maximum_distance,
        inv_sigma: 1.0 / sigma,
        sigma_over_4_times_intensity: 0.25 * sigma * intensity,
        color,
    });
    spans.push((y0 as usize, y1 as usize));
}
#[expect(
    clippy::excessive_precision,
    reason = "Keep the original DrawSegment integration constant."
)]
fn draw_segment(
    segment: &SplineSegment,
    add: bool,
    y: usize,
    x0: usize,
    x1: usize,
    rows: &mut [&mut [f32]; 3],
) {
    let start = (segment.center_x - segment.maximum_distance).round() as isize;
    let end = (segment.center_x + segment.maximum_distance).round() as isize;
    if end < x0 as isize || start >= x1 as isize {
        return;
    }
    let span_x0 = ((x0 as isize).max(start) as usize).wrapping_sub(x0);
    let span_x1 = ((x1 as isize).min(end + 1) as usize).wrapping_sub(x0);
    for x in span_x0..span_x1 {
        let dx = x.wrapping_add(x0) as i32 as f32 - segment.center_x;
        let dy = y as f32 - segment.center_y;
        let distance = hma(dx, dx, hmul(dy, dy)).sqrt();
        let factor = fast_erf(hmul(hma(distance, 0.5, 0.353553391), segment.inv_sigma))
            - fast_erf(hmul(hmul(distance, 0.5) - 0.353553391, segment.inv_sigma));
        let intensity = hmul(segment.sigma_over_4_times_intensity, hmul(factor, factor));
        for (c, row) in rows.iter_mut().enumerate() {
            let cm = if add {
                segment.color[c]
            } else {
                -segment.color[c]
            };
            row[x] = hma(cm, intensity, row[x]);
        }
    }
}
