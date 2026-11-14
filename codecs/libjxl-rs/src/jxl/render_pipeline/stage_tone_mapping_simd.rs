// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
//! The SIMD ToneMappingStage of render_pipeline/stage_tone_mapping.cc
//! (Rec2408ToneMapper, HlgOOTF and GamutMap of cms/tone_mapping-inl.h) for
//! the NEON and AVX2 targets (ADR 0004), written once against [`SimdF32`].
//! Compare it only with the original's SIMD build at the same target.
use super::render_pipeline_stage::{
    RenderPipelineChannelMode as Mode, RenderPipelineStage, RowBuffers, RowInfo, Settings, Span,
};
use super::simd_rows::{vector_span, with_spans};
use super::simd_transfer::{Rec2408ToneMapperSimd, gamut_map, hlg_ootf_apply};
use crate::jxl::base::status::{Status, StatusCode};
use crate::jxl::cms::tone_mapping_scalar::HlgOotf;
use crate::jxl::dec_xyb::OutputEncodingInfo;
use crate::jxl::hwy::{Kernel, SimdF32, Target, dispatch};

pub struct ToneMappingStageSimd {
    target: Target,
    info: OutputEncodingInfo,
    tone_mapper: Option<Box<Rec2408ToneMapperSimd>>,
    hlg_ootf: Option<Box<HlgOotf>>,
    // When the target colorspace is PQ, 1 represents 10000 nits instead of
    // orig_intensity_target. This temporarily changes this if the tone
    // mappers require it.
    to_intensity_target: f32,
    from_desired_intensity_target: f32,
}
impl ToneMappingStageSimd {
    pub fn new(info: OutputEncodingInfo, target: Target) -> Self {
        let mut stage = Self {
            target,
            info,
            tone_mapper: None,
            hlg_ootf: None,
            to_intensity_target: 1.0,
            from_desired_intensity_target: 1.0,
        };
        if stage.info.desired_intensity_target == stage.info.orig_intensity_target {
            // No tone mapping requested.
            return stage;
        }
        let orig_tf = stage.info.orig_color_encoding.tf();
        let dest_tf = stage.info.color_encoding.tf();
        if orig_tf.is_pq() && stage.info.desired_intensity_target < stage.info.orig_intensity_target
        {
            stage.tone_mapper = Some(Box::new(Rec2408ToneMapperSimd::new(
                [0.0, stage.info.orig_intensity_target],
                [0.0, stage.info.desired_intensity_target],
                stage.info.luminances,
            )));
        } else if orig_tf.is_hlg() && !dest_tf.is_hlg() {
            stage.hlg_ootf = Some(Box::new(HlgOotf::new(
                stage.info.orig_intensity_target,
                stage.info.desired_intensity_target,
                stage.info.luminances,
            )));
        }
        if dest_tf.is_pq() && (stage.tone_mapper.is_some() || stage.hlg_ootf.is_some()) {
            stage.to_intensity_target = 10000.0 / stage.info.orig_intensity_target;
            stage.from_desired_intensity_target = stage.info.desired_intensity_target / 10000.0;
        }
        stage
    }
    pub fn is_needed(&self) -> bool {
        self.tone_mapper.is_some() || self.hlg_ootf.is_some()
    }
}

struct ToneMappingRows<'a> {
    rows: [&'a mut [f32]; 3],
    iterations: usize,
    stage: &'a ToneMappingStageSimd,
}
impl Kernel for ToneMappingRows<'_> {
    type Output = ();
    #[inline(always)]
    fn run<D: SimdF32>(self, d: D) {
        let s = self.stage;
        let [row0, row1, row2] = self.rows;
        for i in 0..self.iterations {
            let x = i * D::LANES;
            let mut rgb = [d.load(&row0[x..]), d.load(&row1[x..]), d.load(&row2[x..])];
            for val in &mut rgb {
                *val = d.mul(*val, d.set(s.to_intensity_target));
            }
            let hlg_ootf = s.hlg_ootf.as_deref();
            if let Some(tone_mapper) = &s.tone_mapper {
                tone_mapper.tone_map(d, &mut rgb);
            } else if let Some(hlg_ootf) = hlg_ootf {
                hlg_ootf_apply(d, hlg_ootf.base(), &mut rgb);
            }
            if s.tone_mapper.is_some() || hlg_ootf.is_some_and(HlgOotf::warrants_gamut_mapping) {
                gamut_map(d, &mut rgb, &s.info.luminances, 0.1);
            }
            for val in &mut rgb {
                *val = d.mul(*val, d.set(s.from_desired_intensity_target));
            }
            d.store(rgb[0], &mut row0[x..]);
            d.store(rgb[1], &mut row1[x..]);
            d.store(rgb[2], &mut row2[x..]);
        }
    }
}

impl RenderPipelineStage for ToneMappingStageSimd {
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
        if !self.is_needed() {
            return Ok(());
        }
        if left != 0 || right != 0 {
            return Err(StatusCode::GenericError);
        }
        // `for (x = 0; x < xsize; x += Lanes(d))` with LoadU/StoreU: whole
        // vectors up to RoundUpTo(xsize, Lanes(d)), in the row padding.
        let lanes = self.target.lanes();
        let (_, iterations) = vector_span(0, 0, width, lanes);
        let len = iterations * lanes;
        let rows = [0, 1, 2].map(|c| self.get_input_row(input, c, 0));
        // The source's loads and stores through its row pointers, in place.
        let spans = rows.map(|row| Span { row, from: 0, len });
        with_spans(buffers, [], spans, |[], [a, b, c]| {
            dispatch(
                self.target,
                ToneMappingRows {
                    rows: [a, b, c],
                    iterations,
                    stage: self,
                },
            )
        });
        Ok(())
    }
    fn get_channel_mode(&self, c: usize) -> Mode {
        if c < 3 { Mode::InPlace } else { Mode::Ignored }
    }
    fn get_name(&self) -> &str {
        "ToneMapping"
    }
}

/// `GetToneMappingStage` instantiated for one SIMD target: null when no tone
/// mapping is needed.
pub fn get_tone_mapping_stage_simd(
    info: &OutputEncodingInfo,
    target: Target,
) -> Option<Box<dyn RenderPipelineStage>> {
    let stage = Box::new(ToneMappingStageSimd::new(info.clone(), target));
    if !stage.is_needed() {
        None
    } else {
        Some(stage)
    }
}
