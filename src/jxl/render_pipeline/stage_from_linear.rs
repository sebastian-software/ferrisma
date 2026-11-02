// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
//! The scalar FromLinearStage<OpRgb> specialization in stage_from_linear.cc,
//! with the original high-precision TF_SRGB from cms/transfer_functions-inl.h.
//! The separate original FastLinearToSRGB helper is translated as well.
//! Other transfer-function specializations are separate port boundaries.
use super::render_pipeline_stage::{
    RenderPipelineChannelMode as Mode, RenderPipelineStage, RowBuffers, RowInfo, Settings,
};
use crate::jxl::base::hwy_scalar::mul_add;
use crate::jxl::base::status::{Status, StatusCode};

// Retain the original binary32 literal spellings.
#[allow(clippy::excessive_precision)]
pub fn fast_linear_to_srgb(v: f32) -> f32 {
    let normalized = f32::from_bits((v.to_bits() | 0x3e80_0000) & 0x3eff_ffff);
    let d1 = mul_add(normalized, 0.059914046, -0.108894556);
    let d2 = mul_add(d1, normalized, 0.107963754);
    let power = mul_add(d2, normalized, 0.018092343);
    const HIGH: [u32; 16] = [
        0x0, 0xa, 0x19, 0x26, 0x32, 0x41, 0x4d, 0x5c, 0x68, 0x75, 0x83, 0x8f, 0xa0, 0xaa, 0xb9,
        0xc6,
    ];
    const LOW: [u32; 16] = [
        0x0, 0xb7, 0x4, 0xd, 0xcb, 0xe7, 0x41, 0x68, 0x51, 0xd1, 0xeb, 0xf2, 0x0, 0xb7, 0x4, 0xd,
    ];
    let exponent = (((v.to_bits() as i32 >> 23).wrapping_sub(118)) as u32 & 15) as usize;
    let multiplier = f32::from_bits((HIGH[exponent] << 18) | (LOW[exponent] << 10) | 0x4000_0000);
    // The source computes both vector alternatives before selecting one.
    let linear = v * 12.92;
    let nonlinear = mul_add(power, multiplier, -0.055);
    if v < 0.0031308 { linear } else { nonlinear }
}

pub struct SrgbFromLinearStage;
impl RenderPipelineStage for SrgbFromLinearStage {
    fn settings(&self) -> Settings {
        Settings::default()
    }
    fn get_channel_mode(&self, c: usize) -> Mode {
        if c < 3 { Mode::InPlace } else { Mode::Ignored }
    }
    fn get_name(&self) -> &str {
        "FromLinear"
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
        if left != 0 || right != 0 {
            return Err(StatusCode::GenericError);
        }
        let rows: [_; 3] = std::array::from_fn(|c| self.get_input_row(input, c, 0));
        for x in 0..width {
            for row in &rows {
                let value = buffers.row(*row)[x];
                buffers.row_mut(*row)[x] =
                    crate::jxl::cms::transfer_functions_scalar::srgb_encoded_from_display(value);
            }
        }
        Ok(())
    }
}

pub fn get_srgb_from_linear_stage() -> Box<dyn RenderPipelineStage> {
    Box::new(SrgbFromLinearStage)
}
