// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
//! Complete scalar factory-reached ToLinearStage from stage_to_linear.{h,cc}.
//! Source operation constructors copy their selected scalar parameters. Unused
//! template default constructors are not exposed by the original factory.
use super::render_pipeline_stage::{
    RenderPipelineChannelMode as Mode, RenderPipelineStage, RowBuffers, RowInfo, Settings,
};
use crate::jxl::base::status::{Status, StatusCode};
use crate::jxl::cms::tone_mapping_scalar::HlgOotf;
use crate::jxl::cms::transfer_functions_scalar::{Tf709, TfHlg, TfPq, srgb_display_from_encoded};
use crate::jxl::dec_xyb::OutputEncodingInfo;
enum Op {
    Linear,
    Rgb,
    Pq(TfPq),
    Hlg(HlgOotf),
    Rec709,
    Gamma(f32),
    Invalid,
}
impl Op {
    fn transform(&self, rgb: &mut [f32; 3]) {
        match self {
            Self::Linear | Self::Invalid => {}
            Self::Rgb => {
                for value in rgb {
                    *value = srgb_display_from_encoded(*value);
                }
            }
            Self::Pq(pq) => {
                for value in rgb {
                    *value = pq.display_from_encoded(*value);
                }
            }
            Self::Hlg(ootf) => {
                for value in &mut *rgb {
                    *value = TfHlg::display_from_encoded(*value);
                }
                ootf.apply(rgb);
            }
            Self::Rec709 => {
                for value in rgb {
                    *value = Tf709::display_from_encoded(*value);
                }
            }
            Self::Gamma(gamma) => {
                for value in rgb {
                    let below = *value <= 1e-5f32;
                    let power = crate::jxl::base::fast_math::fast_powf(*value, *gamma);
                    *value = if below { 0.0 } else { power };
                }
            }
        }
    }
}
struct ToLinearStage {
    op: Op,
    valid: bool,
}
impl RenderPipelineStage for ToLinearStage {
    fn settings(&self) -> Settings {
        Settings::default()
    }
    fn is_initialized(&self) -> Status {
        if self.valid {
            Ok(())
        } else {
            Err(StatusCode::GenericError)
        }
    }
    fn get_channel_mode(&self, c: usize) -> Mode {
        if c < 3 { Mode::InPlace } else { Mode::Ignored }
    }
    fn get_name(&self) -> &str {
        "ToLinear"
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
            let mut rgb = std::array::from_fn(|c| buffers.row(rows[c])[x]);
            self.op.transform(&mut rgb);
            for c in 0..3 {
                buffers.row_mut(rows[c])[x] = rgb[c];
            }
        }
        Ok(())
    }
}
/// Invalid target encoding selects the original initialized identity operation.
pub fn get_to_linear_stage(info: &OutputEncodingInfo) -> Box<dyn RenderPipelineStage> {
    let tf = info.color_encoding.tf();
    let op = if tf.is_linear() {
        Op::Linear
    } else if tf.is_srgb() {
        Op::Rgb
    } else if tf.is_pq() {
        Op::Pq(TfPq::new(info.orig_intensity_target))
    } else if tf.is_hlg() {
        Op::Hlg(HlgOotf::from_scene_light(
            info.orig_intensity_target,
            info.luminances,
        ))
    } else if tf.is709() {
        Op::Rec709
    } else if tf.have_gamma || tf.is_dci() {
        Op::Gamma(1.0f32 / info.inverse_gamma)
    } else {
        Op::Invalid
    };
    Box::new(ToLinearStage { op, valid: true })
}
