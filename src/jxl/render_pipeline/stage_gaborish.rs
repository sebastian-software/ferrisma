// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
//! Complete one-lane GaborishStage and factory from stage_gaborish.{h,cc}.
//! Existing owner-relative row descriptors replace source pointers. Scalar
//! Highway loads/stores have one lane, so rounding the left extra span retains
//! its original value. SIMD specializations remain a separate boundary.
use super::render_pipeline_stage::{
    RenderPipelineChannelMode as Mode, RenderPipelineStage, RowBuffers, RowInfo, Settings,
};
use crate::jxl::base::float::mul_add_f32;
use crate::jxl::base::status::Status;
use crate::jxl::loop_filter::LoopFilter;

pub struct GaborishStage {
    weights: [f32; 9],
}
impl GaborishStage {
    fn new(lf: &LoopFilter) -> Self {
        let mut weights = [
            1.0,
            lf.gab_x_weight1,
            lf.gab_x_weight2,
            1.0,
            lf.gab_y_weight1,
            lf.gab_y_weight2,
            1.0,
            lf.gab_b_weight1,
            lf.gab_b_weight2,
        ];
        for c in 0..3 {
            // Ordinary source arithmetic contracts at this site on AArch64.
            // This is distinct from Highway's unfused pixel MulAdd below.
            let div = mul_add_f32(4.0, weights[3 * c + 1] + weights[3 * c + 2], weights[3 * c]);
            let mul = 1.0 / div;
            weights[3 * c] *= mul;
            weights[3 * c + 1] *= mul;
            weights[3 * c + 2] *= mul;
        }
        Self { weights }
    }
}
impl RenderPipelineStage for GaborishStage {
    fn settings(&self) -> Settings {
        Settings::symmetric_border_only(1)
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
        let x_start = -(left as isize);
        let x_end = width.wrapping_add(right) as isize;
        for c in 0..3 {
            let row_t = self.get_input_row(input, c, -1);
            let row_m = self.get_input_row(input, c, 0);
            let row_b = self.get_input_row(input, c, 1);
            let row_out = self.get_output_row(output, c, 0);
            let w0 = self.weights[3 * c];
            let w1 = self.weights[3 * c + 1];
            let w2 = self.weights[3 * c + 2];
            for x in x_start..x_end {
                let t = buffers.row(row_t.offset(x))[0];
                let tl = buffers.row(row_t.offset(x - 1))[0];
                let tr = buffers.row(row_t.offset(x + 1))[0];
                let m = buffers.row(row_m.offset(x))[0];
                let l = buffers.row(row_m.offset(x - 1))[0];
                let r = buffers.row(row_m.offset(x + 1))[0];
                let b = buffers.row(row_b.offset(x))[0];
                let bl = buffers.row(row_b.offset(x - 1))[0];
                let br = buffers.row(row_b.offset(x + 1))[0];
                let sum0 = m;
                let sum1 = (l + r) + (t + b);
                let sum2 = (tl + tr) + (bl + br);
                // The reference scalar object uses weights as the first FMUL
                // operand and the accumulated result as the first FADD operand.
                // Preserve that order for NaN payloads, and Highway MulAdd's
                // separate multiplication/addition rounding even on AArch64.
                let pixels = (w0 * sum0 + w1 * sum1) + w2 * sum2;
                buffers.row_mut(row_out.offset(x))[0] = pixels;
            }
        }
        Ok(())
    }
    fn get_channel_mode(&self, c: usize) -> Mode {
        if c < 3 { Mode::InOut } else { Mode::Ignored }
    }
    fn get_name(&self) -> &str {
        "Gab"
    }
}
pub fn get_gaborish_stage(lf: &LoopFilter) -> Option<Box<dyn RenderPipelineStage>> {
    if !lf.gab {
        return None;
    }
    Some(Box::new(GaborishStage::new(lf)))
}
