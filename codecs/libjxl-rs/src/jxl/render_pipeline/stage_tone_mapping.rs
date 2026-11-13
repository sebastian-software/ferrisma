// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
//! Complete one-lane ToneMappingStage/factory from stage_tone_mapping.cc.
//! Borrowed row descriptors replace restrict pointers; scalar MSan annotations
//! have zero-sized tails. General SIMD specializations remain unported.
use super::render_pipeline_stage::{
    RenderPipelineChannelMode as Mode, RenderPipelineStage, RowBuffers, RowInfo, Settings,
};
use crate::jxl::base::status::{Status, StatusCode};
use crate::jxl::cms::tone_mapping_scalar::{HlgOotf, Rec2408ToneMapper, gamut_map};
use crate::jxl::dec_xyb::OutputEncodingInfo;
pub struct ToneMappingStage {
    info: OutputEncodingInfo,
    tone_mapper: Option<Box<Rec2408ToneMapper>>,
    hlg_ootf: Option<Box<HlgOotf>>,
    to_intensity_target: f32,
    from_desired_intensity_target: f32,
}
impl ToneMappingStage {
    pub fn new(info: OutputEncodingInfo) -> Self {
        let mut stage = Self {
            info,
            tone_mapper: None,
            hlg_ootf: None,
            to_intensity_target: 1.0,
            from_desired_intensity_target: 1.0,
        };
        if stage.info.desired_intensity_target == stage.info.orig_intensity_target {
            return stage;
        }
        let orig = stage.info.orig_color_encoding.tf();
        let dest = stage.info.color_encoding.tf();
        if orig.is_pq() && stage.info.desired_intensity_target < stage.info.orig_intensity_target {
            stage.tone_mapper = Some(Box::new(Rec2408ToneMapper::new(
                [0.0, stage.info.orig_intensity_target],
                [0.0, stage.info.desired_intensity_target],
                stage.info.luminances,
            )));
        } else if orig.is_hlg() && !dest.is_hlg() {
            stage.hlg_ootf = Some(Box::new(HlgOotf::new(
                stage.info.orig_intensity_target,
                stage.info.desired_intensity_target,
                stage.info.luminances,
            )));
        }
        if dest.is_pq() && (stage.tone_mapper.is_some() || stage.hlg_ootf.is_some()) {
            stage.to_intensity_target = 10000.0 / stage.info.orig_intensity_target;
            stage.from_desired_intensity_target = stage.info.desired_intensity_target / 10000.0;
        }
        stage
    }
    pub fn is_needed(&self) -> bool {
        self.tone_mapper.is_some() || self.hlg_ootf.is_some()
    }
}
impl RenderPipelineStage for ToneMappingStage {
    fn settings(&self) -> Settings {
        Settings::default()
    }
    fn get_channel_mode(&self, c: usize) -> Mode {
        if c < 3 { Mode::InPlace } else { Mode::Ignored }
    }
    fn get_name(&self) -> &str {
        "ToneMapping"
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
        if !self.is_needed() {
            return Ok(());
        }
        if left != 0 || right != 0 {
            return Err(StatusCode::GenericError);
        }
        let rows: [_; 3] = std::array::from_fn(|c| self.get_input_row(input, c, 0));
        for x in 0..width {
            let mut rgb = std::array::from_fn(|c| buffers.row(rows[c])[x]);
            for v in &mut rgb {
                *v *= self.to_intensity_target;
            }
            if let Some(t) = &self.tone_mapper {
                t.tone_map(&mut rgb);
            } else {
                self.hlg_ootf.as_ref().unwrap().apply(&mut rgb);
            }
            if self.tone_mapper.is_some()
                || self.hlg_ootf.as_ref().unwrap().warrants_gamut_mapping()
            {
                gamut_map(&mut rgb, self.info.luminances, 0.1);
            }
            for v in &mut rgb {
                *v *= self.from_desired_intensity_target;
            }
            for c in 0..3 {
                buffers.row_mut(rows[c])[x] = rgb[c];
            }
        }
        Ok(())
    }
}
pub fn get_tone_mapping_stage(info: &OutputEncodingInfo) -> Option<Box<dyn RenderPipelineStage>> {
    // HWY_DYNAMIC_DISPATCH(GetToneMappingStage) (port ADR 0004).
    if let Some(target) = crate::jxl::hwy::decoder_target() {
        return super::stage_tone_mapping_simd::get_tone_mapping_stage_simd(info, target);
    }
    let stage = Box::new(ToneMappingStage::new(info.clone()));
    if !stage.is_needed() {
        None
    } else {
        Some(stage)
    }
}
