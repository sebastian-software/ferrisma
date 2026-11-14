// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
//! Complete scalar AddNoiseStage/ConvolveNoiseStage and factories from
//! lib/jxl/render_pipeline/stage_noise.cc. Borrowed parameter owners replace
//! source references; row descriptors resolve under the pipeline's storage borrow.
//! The NEON/AVX2 stages are stage_noise_simd.rs.
use super::render_pipeline_stage::{
    PerThread, RenderPipelineChannelMode as Mode, RenderPipelineStage, Row, RowBuffers, RowInfo,
    Settings,
};
use crate::jxl::base::float::{add_first_nan, mul_promoted_first_nan, sub_first_nan};
use crate::jxl::base::hwy_scalar::{floor, max, min, to_i32, zero_if_negative};
use crate::jxl::base::status::{Status, StatusCode};
use crate::jxl::chroma_from_luma::ColorCorrelation;
use crate::jxl::noise::{NUM_NOISE_POINTS, NoiseParams};

// Copyright 2019 Google LLC
// Copyright (c) The Highway Project Authors. All rights reserved.
// SPDX-License-Identifier: BSD-3-Clause
// Derived scalar Add/Sub/operator* boundaries from hwy/ops/scalar-inl.h,
// under licenses/HIGHWAY-BSD-3-Clause.txt. Multiplication explicitly promotes
// both operands before narrowing, and MulAdd has separate product/add rounding.
// StrengthEvalLut's ZeroIfNegative observes a NaN's sign, so these keep the
// source's first-operand NaN selection while inlining (`base::float`).
#[inline]
fn add(a: f32, b: f32) -> f32 {
    add_first_nan(a, b)
}
#[inline]
fn sub(a: f32, b: f32) -> f32 {
    sub_first_nan(a, b)
}
#[inline]
fn mul(a: f32, b: f32) -> f32 {
    mul_promoted_first_nan(a, b)
}
#[inline]
fn mul_add(a: f32, b: f32, c: f32) -> f32 {
    add(mul(a, b), c)
}

/// Original scalar StrengthEvalLut followed by Clamp0ToMax.
pub fn noise_strength(params: &NoiseParams, x: f32) -> f32 {
    const SCALE: usize = NUM_NOISE_POINTS - 2;
    let scaled_x = max(0.0, mul(x, SCALE as f32));
    let mut floor_x = floor(scaled_x);
    let mut frac_x = sub(scaled_x, floor_x);
    if scaled_x >= (SCALE + 1) as f32 {
        floor_x = SCALE as f32;
        frac_x = 1.0;
    }
    let index = to_i32(floor_x) as usize;
    let low = params.lut[index];
    let hi = params.lut[index + 1];
    zero_if_negative(min(mul_add(sub(hi, low), frac_x, low), 1.0))
}

enum NoiseOwners<'a> {
    Borrowed(&'a NoiseParams, &'a ColorCorrelation),
    Shared(
        std::sync::Arc<crate::jxl::base::sync_cell::SyncRefCell<NoiseParams>>,
        std::sync::Arc<crate::jxl::base::sync_cell::SyncRefCell<ColorCorrelation>>,
    ),
}
pub struct AddNoiseStage<'a> {
    owners: NoiseOwners<'a>,
    first_c: usize,
    /// Copies of the three random rows of one call.
    scratch: PerThread<[Vec<f32>; 3]>,
}
impl RenderPipelineStage for AddNoiseStage<'_> {
    fn as_concurrent(&self) -> Option<&(dyn RenderPipelineStage + Sync)> {
        Some(self)
    }
    fn settings(&self) -> Settings {
        Settings::default()
    }
    fn process_row(
        &self,
        input: &RowInfo,
        _output: &RowInfo,
        buffers: &mut RowBuffers<'_>,
        left: usize,
        right: usize,
        width: usize,
        _x: usize,
        _y: usize,
        thread: usize,
    ) -> Status {
        let shared = match &self.owners {
            NoiseOwners::Borrowed(..) => None,
            NoiseOwners::Shared(params, correlation) => {
                Some((params.borrow(), correlation.borrow()))
            }
        };
        let (noise_params, color_correlation) = match &self.owners {
            NoiseOwners::Borrowed(params, correlation) => (*params, *correlation),
            NoiseOwners::Shared(..) => {
                let (params, correlation) = shared.as_ref().unwrap();
                (&**params, &**correlation)
            }
        };
        if !noise_params.has_any() {
            return Ok(());
        }
        if left != 0 || right != 0 {
            return Err(StatusCode::GenericError);
        }
        let ytox = color_correlation.y_to_x_ratio(0);
        let ytob = color_correlation.y_to_b_ratio(0);
        let rows: [_; 3] = std::array::from_fn(|c| self.get_input_row(input, c, 0));
        let random: [_; 3] =
            std::array::from_fn(|c| self.get_input_row(input, self.first_c + c, 0));
        // Resolve the three in-place spans and the three random spans once
        // per row; values, order and arithmetic are unchanged. The random
        // channels are separate Input channels, copied before the loop
        // because the borrow of the in-place spans is exclusive.
        let mut noise = self.scratch.with(thread, std::mem::take);
        for (c, row) in noise.iter_mut().enumerate() {
            row.clear();
            row.extend_from_slice(&buffers.row(random[c])[..width]);
        }
        let [r0, r1, r2] = buffers.three_rows_mut(rows, width);
        for x in 0..width {
            let vx = r0[x];
            let vy = r1[x];
            let vb = r2[x];
            let in_g = sub(vy, vx);
            let in_r = add(vy, vx);
            let strength_g = noise_strength(noise_params, mul(in_g, 0.5));
            let strength_r = noise_strength(noise_params, mul(in_r, 0.5));
            let rnd_r = mul(noise[0][x], 0.22);
            let rnd_g = mul(noise[1][x], 0.22);
            let rnd_c = mul(noise[2][x], 0.22);
            let red = mul(strength_r, mul_add(0.0078125, rnd_r, mul(0.9921875, rnd_c)));
            let green = mul(strength_g, mul_add(0.0078125, rnd_g, mul(0.9921875, rnd_c)));
            let rg = add(red, green);
            let vx = add(mul_add(ytox, rg, sub(red, green)), vx);
            let vy = add(vy, rg);
            let vb = mul_add(ytob, rg, vb);
            r0[x] = vx;
            r1[x] = vy;
            r2[x] = vb;
        }
        self.scratch.with(thread, |slot| *slot = noise);
        Ok(())
    }
    fn get_channel_mode(&self, c: usize) -> Mode {
        if c >= self.first_c {
            Mode::Input
        } else if c < 3 {
            Mode::InPlace
        } else {
            Mode::Ignored
        }
    }
    fn get_name(&self) -> &str {
        "AddNoise"
    }
}
pub fn get_add_noise_stage<'a>(
    noise_params: &'a NoiseParams,
    color_correlation: &'a ColorCorrelation,
    first_c: usize,
) -> Box<dyn RenderPipelineStage + 'a> {
    // HWY_DYNAMIC_DISPATCH(GetAddNoiseStage) (port ADR 0004).
    if let Some(target) = crate::jxl::hwy::decoder_target() {
        return super::stage_noise_simd::get_add_noise_stage_simd(
            noise_params,
            color_correlation,
            first_c,
            target,
        );
    }
    Box::new(AddNoiseStage {
        owners: NoiseOwners::Borrowed(noise_params, color_correlation),
        first_c,
        scratch: PerThread::new(),
    })
}

/// Share the original parameter owners with decoder state. Each source call
/// borrows them only while processing a row; later assignments remain visible.
pub fn get_add_noise_stage_shared(
    noise_params: std::sync::Arc<crate::jxl::base::sync_cell::SyncRefCell<NoiseParams>>,
    color_correlation: std::sync::Arc<crate::jxl::base::sync_cell::SyncRefCell<ColorCorrelation>>,
    first_c: usize,
) -> Box<dyn RenderPipelineStage> {
    if let Some(target) = crate::jxl::hwy::decoder_target() {
        return super::stage_noise_simd::get_add_noise_stage_shared_simd(
            noise_params,
            color_correlation,
            first_c,
            target,
        );
    }
    Box::new(AddNoiseStage {
        owners: NoiseOwners::Shared(noise_params, color_correlation),
        first_c,
        scratch: PerThread::new(),
    })
}

pub struct ConvolveNoiseStage {
    first_c: usize,
    /// One output row, written back after the row's samples are computed.
    scratch: PerThread<[Vec<f32>; 1]>,
}
impl RenderPipelineStage for ConvolveNoiseStage {
    fn as_concurrent(&self) -> Option<&(dyn RenderPipelineStage + Sync)> {
        Some(self)
    }
    fn settings(&self) -> Settings {
        Settings::symmetric_border_only(2)
    }
    fn process_row(
        &self,
        input: &RowInfo,
        output: &RowInfo,
        buffers: &mut RowBuffers<'_>,
        left: usize,
        right: usize,
        width: usize,
        _x: usize,
        _y: usize,
        thread: usize,
    ) -> Status {
        for c in self.first_c..self.first_c + 3 {
            let rows: [_; 5] = std::array::from_fn(|i| self.get_input_row(input, c, i as i32 - 2));
            let out = self.get_output_row(output, c, 0);
            // Resolve the five input spans once per row and stage the output
            // row, as for Gaborish: input and output rows may share a plane.
            // Values, order and arithmetic are unchanged.
            let spans: [&[f32]; 5] = std::array::from_fn(|i| buffers.row(Row { x: 0, ..rows[i] }));
            let get = |r: usize, x: isize| spans[r][rows[r].x.wrapping_add_signed(x)];
            let x_begin = -(left as isize);
            let [mut scratch0] = self.scratch.with(thread, std::mem::take);
            scratch0.clear();
            for x in x_begin..(width + right) as isize {
                let p00 = get(2, x);
                let mut others = 0.0;
                for i in -2..=2 {
                    others = add(others, get(0, x + i));
                    others = add(others, get(1, x + i));
                    others = add(others, get(3, x + i));
                    others = add(others, get(4, x + i));
                }
                for i in [-2, -1, 1, 2] {
                    others = add(others, get(2, x + i));
                }
                scratch0.push(mul_add(others, 0.16, mul(p00, -3.84)));
            }
            let target = buffers.row_mut(out.offset(x_begin));
            target[..scratch0.len()].copy_from_slice(&scratch0);
            self.scratch.with(thread, |slot| *slot = [scratch0]);
        }
        Ok(())
    }
    fn get_channel_mode(&self, c: usize) -> Mode {
        if c >= self.first_c {
            Mode::InOut
        } else {
            Mode::Ignored
        }
    }
    fn get_name(&self) -> &str {
        "ConvNoise"
    }
}
pub fn get_convolve_noise_stage(first_c: usize) -> Box<dyn RenderPipelineStage> {
    // HWY_DYNAMIC_DISPATCH(GetConvolveNoiseStage) (port ADR 0004).
    if let Some(target) = crate::jxl::hwy::decoder_target() {
        return super::stage_noise_simd::get_convolve_noise_stage_simd(first_c, target);
    }
    Box::new(ConvolveNoiseStage {
        first_c,
        scratch: PerThread::new(),
    })
}
