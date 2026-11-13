// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
// FastCosf and FastErff are translated from lib/jxl/base/fast_math-inl.h.
//! The SIMD `SegmentsFromPoints` (with `ContinuousIDCT`) and `DrawSegments`
//! (with both `DrawSegment` overloads) of lib/jxl/splines.cc for the NEON and
//! AVX2 targets (ADR 0004), written once against [`SimdF32`]. Compare them
//! only with the original's SIMD build at the same target.
//!
//! `ContinuousIDCT` uses `HWY_CAPPED(float, 32)`, the full vector on both
//! targets, so its `SumOfLanes` follows the target's lane count. `DrawSegment`
//! draws whole vectors while they fit in the span and the rest with
//! `HWY_CAPPED(float, 1)`; every operation of that body is lane-wise, so the
//! one-lane vector is computed in lane 0 of a full vector (as for
//! compressed_dc_simd.rs) and only lane 0 is loaded and stored.
//! `ComputeSegments` is scalar code inside `HWY_NAMESPACE` without a
//! contraction candidate; it is shared with the scalar path.
use super::hwy::{Kernel, SimdF32, Target, dispatch};
use super::splines::{
    DESIRED_RENDERING_DISTANCE, Dct32, Spline, SplinePoint, SplineSegment, compute_segments,
};

/// `kPi` of base/common.h, a double.
const K_PI: f64 = std::f64::consts::PI;

/// `FastCosf(df, x)`.
#[inline(always)]
#[expect(
    clippy::excessive_precision,
    clippy::approx_constant,
    reason = "Keep the original polynomial constants directly traceable to fast_math-inl.h."
)]
fn fast_cosf<D: SimdF32>(df: D, x: D::V) -> D::V {
    // Step 1: range reduction to [0, 2pi)
    let pi2 = df.set((K_PI * 2.0) as f32);
    let pi2_inv = df.set((0.5 / K_PI) as f32);
    let npi2 = df.mul(df.floor(df.mul(x, pi2_inv)), pi2);
    let xmodpi2 = df.sub(x, npi2);
    // Step 2: range reduction to [0, pi]
    let x_pi = df.min(xmodpi2, df.sub(pi2, xmodpi2));
    // Step 3: range reduction to [0, pi/2]
    let above_pihalf = df.ge(x_pi, df.set((K_PI / 2.0) as f32));
    let x_pihalf = df.if_then_else(above_pihalf, df.sub(df.set(K_PI as f32), x_pi), x_pi);
    // Step 4: Taylor-like approximation, scaled by 2**0.75 to make angle
    // duplication steps faster, on x/4.
    let xs = df.mul(x_pihalf, df.set(0.25));
    let x2 = df.mul(xs, xs);
    let x4 = df.mul(x2, x2);
    let cosx_prescaling = df.mul_add(
        x4,
        df.set(0.06960438),
        df.mul_add(x2, df.set(-0.84087373), df.set(1.68179268)),
    );
    // Step 5: angle duplication.
    let cosx_scale1 = df.mul_add(cosx_prescaling, cosx_prescaling, df.set(-1.414213562));
    let cosx_scale2 = df.mul_add(cosx_scale1, cosx_scale1, df.set(-1.0));
    // Step 6: change sign if needed.
    let signbit = df.shift_left_i32::<31>(df.bitcast_to_i32(df.vec_from_mask(above_pihalf)));
    df.xor(df.bitcast_to_f32(signbit), cosx_scale2)
}

/// `FastErff(df, x)`.
#[inline(always)]
#[expect(
    clippy::excessive_precision,
    reason = "Keep the original polynomial constants directly traceable to fast_math-inl.h."
)]
fn fast_erff<D: SimdF32>(df: D, x: D::V) -> D::V {
    let xle0 = df.le(x, df.zero());
    let absx = df.abs(x);
    // Compute 1 - 1 / ((((x * a + b) * x + c) * x + d) * x + 1)**4
    let denom1 = df.mul_add(absx, df.set(7.77394369e-02), df.set(2.05260015e-04));
    let denom2 = df.mul_add(denom1, absx, df.set(2.32120216e-01));
    let denom3 = df.mul_add(denom2, absx, df.set(2.77820801e-01));
    let denom4 = df.mul_add(denom3, absx, df.set(1.0));
    let denom5 = df.mul(denom4, denom4);
    let inv_denom5 = df.div(df.set(1.0), denom5);
    let result = df.neg_mul_add(inv_denom5, inv_denom5, df.set(1.0));
    // Change sign if needed.
    let signbit = df.shift_left_i32::<31>(df.bitcast_to_i32(df.vec_from_mask(xle0)));
    df.xor(df.bitcast_to_f32(signbit), result)
}

/// `ContinuousIDCT(dct, t)`.
struct ContinuousIdct<'a> {
    dct: &'a Dct32,
    t: f32,
}
impl Kernel for ContinuousIdct<'_> {
    type Output = f32;
    #[inline(always)]
    fn run<D: SimdF32>(self, df: D) -> f32 {
        // kPi / 32 * i in double, stored as float.
        let multipliers: [f32; 32] = std::array::from_fn(|i| (K_PI / 32.0 * i as f64) as f32);
        let mut result = df.zero();
        let tandhalf = df.set(self.t + 0.5);
        let mut i = 0;
        while i < 32 {
            let cos_arg = df.mul(df.load(&multipliers[i..]), tandhalf);
            let cos = fast_cosf(df, cos_arg);
            let local_res = df.mul(df.load(&self.dct[i..]), cos);
            result = df.mul_add(df.set(std::f32::consts::SQRT_2), local_res, result);
            i += D::LANES;
        }
        df.get_lane(df.sum_of_lanes(result))
    }
}

/// `SegmentsFromPoints` instantiated for one SIMD target.
pub fn segments_from_points_simd(
    target: Target,
    image_ysize: usize,
    spline: &Spline,
    points_to_draw: &[(SplinePoint, f32)],
    arc_length: f32,
    segments: &mut Vec<SplineSegment>,
    segments_spans: &mut Vec<(usize, usize)>,
) {
    let inv_arc_length = 1.0 / arc_length;
    let idct = |dct: &Dct32, t: f32| dispatch(target, ContinuousIdct { dct, t });
    for (k, &(point, multiplier)) in points_to_draw.iter().enumerate() {
        let progress_along_arc = super::base::float::min_f32(
            1.0,
            (k as i32 as f32 * DESIRED_RENDERING_DISTANCE) * inv_arc_length,
        );
        let color: [f32; 3] = std::array::from_fn(|c| {
            idct(&spline.color_dct[c], (32 - 1) as f32 * progress_along_arc)
        });
        let sigma = idct(&spline.sigma_dct, (32 - 1) as f32 * progress_along_arc);
        compute_segments(
            image_ysize,
            point,
            multiplier,
            color,
            sigma,
            segments,
            segments_spans,
        );
    }
}

/// The vector `DrawSegment(df, segment, add, y, x, x0, rows)`; `lanes` is
/// `Lanes(df)` (the full vector) or 1 (`HWY_CAPPED(float, 1)`, computed in
/// lane 0).
#[inline(always)]
#[expect(
    clippy::excessive_precision,
    reason = "Keep the original DrawSegment integration constant."
)]
#[expect(
    clippy::too_many_arguments,
    reason = "Retain the original DrawSegment(df, segment, add, y, x, x0, rows) boundary plus the lane count."
)]
fn draw_segment_vector<D: SimdF32>(
    df: D,
    lanes: usize,
    segment: &SplineSegment,
    add: bool,
    y: usize,
    x: usize,
    x0: usize,
    rows: &mut [&mut [f32]; 3],
) {
    let inv_sigma = df.set(segment.inv_sigma);
    let half = df.set(0.5);
    let one_over_2s2 = df.set(0.353553391);
    let sigma_over_4_times_intensity = df.set(segment.sigma_over_4_times_intensity);
    // Iota(di, x + x0): the int32 conversion of x + x0, plus the lane index.
    let base = x.wrapping_add(x0) as i32;
    let iota: [u32; 16] = std::array::from_fn(|i| base.wrapping_add(i as i32) as u32);
    let dx = df.sub(
        df.convert_to_f32(df.bitcast_to_i32(df.load_bits(&iota))),
        df.set(segment.center_x),
    );
    let dy = df.set(y as f32 - segment.center_y);
    let sqd = df.mul_add(dx, dx, df.mul(dy, dy));
    let distance = df.sqrt(sqd);
    let one_dimensional_factor = df.sub(
        fast_erff(
            df,
            df.mul(df.mul_add(distance, half, one_over_2s2), inv_sigma),
        ),
        fast_erff(
            df,
            df.mul(df.mul_sub(distance, half, one_over_2s2), inv_sigma),
        ),
    );
    let local_intensity = df.mul(
        sigma_over_4_times_intensity,
        df.mul(one_dimensional_factor, one_dimensional_factor),
    );
    for (c, row) in rows.iter_mut().enumerate() {
        let cm = df.set(if add {
            segment.color[c]
        } else {
            -segment.color[c]
        });
        let mut buf = [0.0f32; 16];
        buf[..lanes].copy_from_slice(&row[x..x + lanes]);
        let in_ = df.load(&buf);
        df.store(df.mul_add(cm, local_intensity, in_), &mut buf);
        row[x..x + lanes].copy_from_slice(&buf[..lanes]);
    }
}

struct DrawSegments<'a, 'r> {
    rows: [&'r mut [f32]; 3],
    y: usize,
    x0: usize,
    x1: usize,
    add: bool,
    segments: &'a [SplineSegment],
    segment_indices: &'a [usize],
    segment_y_start: &'a [isize],
}
impl Kernel for DrawSegments<'_, '_> {
    type Output = ();
    #[inline(always)]
    fn run<D: SimdF32>(self, df: D) {
        let mut rows = self.rows;
        let (y, x0, x1) = (self.y, self.x0, self.x1);
        for i in self.segment_y_start[y]..self.segment_y_start[y + 1] {
            let segment = &self.segments[self.segment_indices[i as usize]];
            // DrawSegment(segment, add, y, x0, x1, rows)
            let start = (segment.center_x - segment.maximum_distance).round() as isize;
            let end = (segment.center_x + segment.maximum_distance).round() as isize;
            if end < x0 as isize || start >= x1 as isize {
                continue; // span does not intersect scan
            }
            let span_x0 = ((x0 as isize).max(start) as usize).wrapping_sub(x0);
            let span_x1 = ((x1 as isize).min(end + 1) as usize).wrapping_sub(x0); // exclusive
            let mut x = span_x0;
            while x + D::LANES <= span_x1 {
                draw_segment_vector(df, D::LANES, segment, self.add, y, x, x0, &mut rows);
                x += D::LANES;
            }
            while x < span_x1 {
                draw_segment_vector(df, 1, segment, self.add, y, x, x0, &mut rows);
                x += 1;
            }
        }
    }
}

/// `DrawSegments` instantiated for one SIMD target.
#[allow(
    clippy::too_many_arguments,
    reason = "Retain the original DrawSegments boundary."
)]
pub fn draw_segments_simd(
    target: Target,
    rows: [&mut [f32]; 3],
    y: usize,
    x0: usize,
    x1: usize,
    add: bool,
    segments: &[SplineSegment],
    segment_indices: &[usize],
    segment_y_start: &[isize],
) {
    dispatch(
        target,
        DrawSegments {
            rows,
            y,
            x0,
            x1,
            add,
            segments,
            segment_indices,
            segment_y_start,
        },
    );
}
