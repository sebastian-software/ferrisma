// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
//! The SIMD ToLinearStage of render_pipeline/stage_to_linear.cc with all
//! operations (OpLinear, OpRgb, OpPq, OpHlg with the inverse HlgOOTF, Op709,
//! OpGamma, OpInvalid) for the NEON and AVX2 targets (ADR 0004), written
//! once against [`SimdF32`]. Compare it only with the original's SIMD build
//! at the same target.
use super::render_pipeline_stage::{
    RenderPipelineChannelMode as Mode, RenderPipelineStage, RowBuffers, RowInfo, Settings, Span,
};
use super::simd_rows::{vector_span, with_spans};
use super::simd_transfer::{
    TfPqSimd, fast_powf, hlg_display_from_encoded, hlg_ootf_apply, rec709_display_from_encoded,
    srgb_display_from_encoded,
};
use crate::jxl::base::status::{Status, StatusCode};
use crate::jxl::cms::tone_mapping_scalar::HlgOotf;
use crate::jxl::dec_xyb::OutputEncodingInfo;
use crate::jxl::hwy::{Kernel, SimdF32, Target, dispatch};

/// The source's `Op` structs; each constructor copies only its own state.
enum Op {
    Linear,
    Rgb,
    Pq(TfPqSimd),
    Hlg(HlgOotf),
    Rec709,
    Gamma(f32),
    Invalid,
}
impl Op {
    /// `op_.Transform(d, &r, &g, &b)`.
    #[inline(always)]
    fn transform<D: SimdF32>(&self, d: D, rgb: &mut [D::V; 3]) {
        match self {
            Self::Linear | Self::Invalid => {}
            Self::Rgb => {
                for val in rgb {
                    *val = srgb_display_from_encoded(d, *val);
                }
            }
            Self::Pq(tf_pq) => {
                for val in rgb {
                    *val = tf_pq.display_from_encoded(d, *val);
                }
            }
            Self::Hlg(hlg_ootf) => {
                for val in &mut *rgb {
                    *val = hlg_display_from_encoded(d, *val);
                }
                hlg_ootf_apply(d, hlg_ootf.base(), rgb);
            }
            Self::Rec709 => {
                for val in rgb {
                    *val = rec709_display_from_encoded(d, *val);
                }
            }
            Self::Gamma(gamma) => {
                for val in rgb {
                    *val = d.if_then_zero_else(
                        d.le(*val, d.set(1e-5f32)),
                        fast_powf(d, *val, d.set(*gamma)),
                    );
                }
            }
        }
    }
}

struct ToLinearRows<'a> {
    rows: [&'a mut [f32]; 3],
    iterations: usize,
    op: &'a Op,
}
impl Kernel for ToLinearRows<'_> {
    type Output = ();
    #[inline(always)]
    fn run<D: SimdF32>(self, d: D) {
        let [row0, row1, row2] = self.rows;
        for i in 0..self.iterations {
            let x = i * D::LANES;
            let mut rgb = [d.load(&row0[x..]), d.load(&row1[x..]), d.load(&row2[x..])];
            self.op.transform(d, &mut rgb);
            d.store(rgb[0], &mut row0[x..]);
            d.store(rgb[1], &mut row1[x..]);
            d.store(rgb[2], &mut row2[x..]);
        }
    }
}

pub struct ToLinearStageSimd {
    target: Target,
    op: Op,
}
impl RenderPipelineStage for ToLinearStageSimd {
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
        _thread: usize,
    ) -> Status {
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
                ToLinearRows {
                    rows: [a, b, c],
                    iterations,
                    op: &self.op,
                },
            )
        });
        Ok(())
    }
    fn get_channel_mode(&self, c: usize) -> Mode {
        if c < 3 { Mode::InPlace } else { Mode::Ignored }
    }
    fn get_name(&self) -> &str {
        "ToLinear"
    }
}

/// `GetToLinearStage` instantiated for one SIMD target. An invalid target
/// encoding selects the original's initialized identity operation
/// (`OpInvalid`).
pub fn get_to_linear_stage_simd(
    info: &OutputEncodingInfo,
    target: Target,
) -> Box<dyn RenderPipelineStage> {
    let tf = info.color_encoding.tf();
    let op = if tf.is_linear() {
        Op::Linear
    } else if tf.is_srgb() {
        Op::Rgb
    } else if tf.is_pq() {
        Op::Pq(TfPqSimd::new(info.orig_intensity_target))
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
    Box::new(ToLinearStageSimd { target, op })
}
