// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
//! Normal scalar XYBStage and GetXYBStage from stage_xyb.cc. The NEON-specific
//! FastXYBStage and SIMD conversions remain separate work. One-lane tails have
//! zero length, so the source MSan unpoison/poison annotations touch no storage.
use super::render_pipeline_stage::{
    RenderPipelineChannelMode as Mode, RenderPipelineStage, RowBuffers, RowInfo, Settings,
};
use crate::jxl::base::status::{Status, StatusCode};
use crate::jxl::cms::opsin_params::{SCALED_XYB_OFFSET, SCALED_XYB_SCALE};
use crate::jxl::color_encoding::ColorSpace;
use crate::jxl::dec_xyb::{OpsinParams, OutputEncodingInfo};
use crate::jxl::dec_xyb_scalar::{mul, xyb_to_rgb};

pub struct XybStage {
    opsin_params: OpsinParams,
    output_is_xyb: bool,
}
impl XybStage {
    pub fn new(info: &OutputEncodingInfo) -> Self {
        Self {
            opsin_params: info.opsin_params.clone(),
            output_is_xyb: info.color_encoding.get_color_space() == ColorSpace::XYB,
        }
    }
}
impl RenderPipelineStage for XybStage {
    fn settings(&self) -> Settings {
        Settings::default()
    }
    fn get_channel_mode(&self, c: usize) -> Mode {
        if c < 3 { Mode::InPlace } else { Mode::Ignored }
    }
    fn get_name(&self) -> &str {
        "XYB"
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
        if self.output_is_xyb {
            for x in 0..width {
                let in_x = buffers.row(rows[0])[x];
                let in_y = buffers.row(rows[1])[x];
                let in_b = buffers.row(rows[2])[x];
                let out_x = mul(in_x + SCALED_XYB_OFFSET[0], SCALED_XYB_SCALE[0]);
                let out_y = mul(in_y + SCALED_XYB_OFFSET[1], SCALED_XYB_SCALE[1]);
                let out_b = mul((in_b - in_y) + SCALED_XYB_OFFSET[2], SCALED_XYB_SCALE[2]);
                buffers.row_mut(rows[0])[x] = out_x;
                buffers.row_mut(rows[1])[x] = out_y;
                buffers.row_mut(rows[2])[x] = out_b;
            }
        } else {
            for x in 0..width {
                let rgb = xyb_to_rgb(
                    buffers.row(rows[0])[x],
                    buffers.row(rows[1])[x],
                    buffers.row(rows[2])[x],
                    &self.opsin_params,
                );
                for c in 0..3 {
                    buffers.row_mut(rows[c])[x] = rgb[c];
                }
            }
        }
        Ok(())
    }
}
pub fn get_xyb_stage(info: &OutputEncodingInfo) -> Box<dyn RenderPipelineStage> {
    Box::new(XybStage::new(info))
}
