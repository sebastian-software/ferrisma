// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
//! Complete scalar AddNoiseStage/ConvolveNoiseStage and factories from
//! lib/jxl/render_pipeline/stage_noise.cc. Borrowed parameter owners replace
//! source references; row descriptors resolve under the pipeline's storage borrow.
use super::render_pipeline_stage::{
    RenderPipelineChannelMode as Mode, RenderPipelineStage, RowBuffers, RowInfo, Settings,
};
use crate::jxl::base::hwy_scalar::{floor, max, min, to_i32, zero_if_negative};
use crate::jxl::base::status::{Status, StatusCode};
use crate::jxl::chroma_from_luma::ColorCorrelation;
use crate::jxl::noise::{NUM_NOISE_POINTS, NoiseParams};
use std::cell::RefCell;
use std::rc::Rc;

// Copyright 2019 Google LLC
// Copyright (c) The Highway Project Authors. All rights reserved.
// SPDX-License-Identifier: BSD-3-Clause
// Derived scalar Add/Sub/operator* boundaries from hwy/ops/scalar-inl.h,
// under licenses/HIGHWAY-BSD-3-Clause.txt. Multiplication explicitly promotes
// both operands before narrowing, and MulAdd has separate product/add rounding.
#[inline(never)]
fn add(a: f32, b: f32) -> f32 {
    a + b
}
#[inline(never)]
fn sub(a: f32, b: f32) -> f32 {
    a - b
}
#[inline(never)]
fn promote(a: f32) -> f64 {
    f64::from(a)
}
#[inline(never)]
fn multiply_and_demote(a: f64, b: f64) -> f32 {
    (a * b) as f32
}
fn mul(a: f32, b: f32) -> f32 {
    multiply_and_demote(promote(a), promote(b))
}
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
    Shared(Rc<RefCell<NoiseParams>>, Rc<RefCell<ColorCorrelation>>),
}
pub struct AddNoiseStage<'a> {
    owners: NoiseOwners<'a>,
    first_c: usize,
}
impl RenderPipelineStage for AddNoiseStage<'_> {
    fn settings(&self) -> Settings {
        Settings::default()
    }
    fn process_row(
        &mut self,
        input: &RowInfo,
        _output: &RowInfo,
        buffers: &mut RowBuffers<'_>,
        left: usize,
        right: usize,
        width: usize,
        _x: usize,
        _y: usize,
        _thread: usize,
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
        for x in 0..width {
            let vx = buffers.row(rows[0])[x];
            let vy = buffers.row(rows[1])[x];
            let vb = buffers.row(rows[2])[x];
            let in_g = sub(vy, vx);
            let in_r = add(vy, vx);
            let strength_g = noise_strength(noise_params, mul(in_g, 0.5));
            let strength_r = noise_strength(noise_params, mul(in_r, 0.5));
            let rnd_r = mul(buffers.row(random[0])[x], 0.22);
            let rnd_g = mul(buffers.row(random[1])[x], 0.22);
            let rnd_c = mul(buffers.row(random[2])[x], 0.22);
            let red = mul(strength_r, mul_add(0.0078125, rnd_r, mul(0.9921875, rnd_c)));
            let green = mul(strength_g, mul_add(0.0078125, rnd_g, mul(0.9921875, rnd_c)));
            let rg = add(red, green);
            let vx = add(mul_add(ytox, rg, sub(red, green)), vx);
            let vy = add(vy, rg);
            let vb = mul_add(ytob, rg, vb);
            buffers.row_mut(rows[0])[x] = vx;
            buffers.row_mut(rows[1])[x] = vy;
            buffers.row_mut(rows[2])[x] = vb;
        }
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
    Box::new(AddNoiseStage {
        owners: NoiseOwners::Borrowed(noise_params, color_correlation),
        first_c,
    })
}

/// Share the original parameter owners with decoder state. Each source call
/// borrows them only while processing a row; later assignments remain visible.
pub fn get_add_noise_stage_shared(
    noise_params: Rc<RefCell<NoiseParams>>,
    color_correlation: Rc<RefCell<ColorCorrelation>>,
    first_c: usize,
) -> Box<dyn RenderPipelineStage> {
    Box::new(AddNoiseStage {
        owners: NoiseOwners::Shared(noise_params, color_correlation),
        first_c,
    })
}

pub struct ConvolveNoiseStage {
    first_c: usize,
}
impl RenderPipelineStage for ConvolveNoiseStage {
    fn settings(&self) -> Settings {
        Settings::symmetric_border_only(2)
    }
    fn process_row(
        &mut self,
        input: &RowInfo,
        output: &RowInfo,
        buffers: &mut RowBuffers<'_>,
        left: usize,
        right: usize,
        width: usize,
        _x: usize,
        _y: usize,
        _thread: usize,
    ) -> Status {
        for c in self.first_c..self.first_c + 3 {
            let rows: [_; 5] = std::array::from_fn(|i| self.get_input_row(input, c, i as i32 - 2));
            let out = self.get_output_row(output, c, 0);
            for x in -(left as isize)..(width + right) as isize {
                let p00 = buffers.row(rows[2].offset(x))[0];
                let mut others = 0.0;
                for i in -2..=2 {
                    others = add(others, buffers.row(rows[0].offset(x + i))[0]);
                    others = add(others, buffers.row(rows[1].offset(x + i))[0]);
                    others = add(others, buffers.row(rows[3].offset(x + i))[0]);
                    others = add(others, buffers.row(rows[4].offset(x + i))[0]);
                }
                for i in [-2, -1, 1, 2] {
                    others = add(others, buffers.row(rows[2].offset(x + i))[0]);
                }
                buffers.row_mut(out.offset(x))[0] = mul_add(others, 0.16, mul(p00, -3.84));
            }
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
    Box::new(ConvolveNoiseStage { first_c })
}
