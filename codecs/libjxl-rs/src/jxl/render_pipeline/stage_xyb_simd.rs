// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
//! The SIMD XYBStage of render_pipeline/stage_xyb.cc and its `XybToRgb`
//! (dec_xyb-inl.h, the non-`HWY_SCALAR` branch) for the NEON and AVX2
//! targets (ADR 0004), written once against [`SimdF32`]. Compare it only
//! with the original's SIMD build at the same target. The NEON-only
//! `FastXYBStage` is compiled out by `JXL_HIGH_PRECISION` and not ported.
use super::render_pipeline_stage::{
    RenderPipelineChannelMode as Mode, RenderPipelineStage, RowBuffers, RowInfo, Settings, Span,
};
use super::simd_rows::{vector_span, with_spans};
use crate::jxl::base::status::{Status, StatusCode};
use crate::jxl::cms::opsin_params::{SCALED_XYB_OFFSET, SCALED_XYB_SCALE};
use crate::jxl::color_encoding::ColorSpace;
use crate::jxl::dec_xyb::{OpsinParams, OutputEncodingInfo};
use crate::jxl::hwy::{Kernel, SimdF32, Target, dispatch};

/// `XybToRgb(d, opsin_x, opsin_y, opsin_b, opsin_params, ...)` of
/// dec_xyb-inl.h for a SIMD target: the biases come from one `LoadDup128`
/// and `Broadcast`, and each matrix entry is the `LoadDup128` of its four
/// stored copies, so a lane uses copy `lane % 4`.
#[inline(always)]
pub fn xyb_to_rgb<D: SimdF32>(
    d: D,
    opsin_x: D::V,
    opsin_y: D::V,
    opsin_b: D::V,
    opsin_params: &OpsinParams,
) -> [D::V; 3] {
    let neg_bias_rgb = d.load_dup128(&opsin_params.opsin_biases);
    let neg_bias_r = d.broadcast::<0>(neg_bias_rgb);
    let neg_bias_g = d.broadcast::<1>(neg_bias_rgb);
    let neg_bias_b = d.broadcast::<2>(neg_bias_rgb);

    // Color space: XYB -> RGB
    let mut gamma_r = d.add(opsin_y, opsin_x);
    let mut gamma_g = d.sub(opsin_y, opsin_x);
    let mut gamma_b = opsin_b;

    gamma_r = d.sub(gamma_r, d.set(opsin_params.opsin_biases_cbrt[0]));
    gamma_g = d.sub(gamma_g, d.set(opsin_params.opsin_biases_cbrt[1]));
    gamma_b = d.sub(gamma_b, d.set(opsin_params.opsin_biases_cbrt[2]));

    // Undo gamma compression: linear = gamma^3 for efficiency.
    let gamma_r2 = d.mul(gamma_r, gamma_r);
    let gamma_g2 = d.mul(gamma_g, gamma_g);
    let gamma_b2 = d.mul(gamma_b, gamma_b);
    let mixed_r = d.mul_add(gamma_r2, gamma_r, neg_bias_r);
    let mixed_g = d.mul_add(gamma_g2, gamma_g, neg_bias_g);
    let mixed_b = d.mul_add(gamma_b2, gamma_b, neg_bias_b);

    let inverse_matrix = &opsin_params.inverse_opsin_matrix;
    let m = |i: usize| d.load_dup128(&inverse_matrix[i * 4..i * 4 + 4]);

    // Unmix (multiply by 3x3 inverse_matrix)
    let mut linear_r = d.mul(m(0), mixed_r);
    let mut linear_g = d.mul(m(3), mixed_r);
    let mut linear_b = d.mul(m(6), mixed_r);
    linear_r = d.mul_add(m(1), mixed_g, linear_r);
    linear_g = d.mul_add(m(4), mixed_g, linear_g);
    linear_b = d.mul_add(m(7), mixed_g, linear_b);
    linear_r = d.mul_add(m(2), mixed_b, linear_r);
    linear_g = d.mul_add(m(5), mixed_b, linear_g);
    linear_b = d.mul_add(m(8), mixed_b, linear_b);
    [linear_r, linear_g, linear_b]
}

/// One in-place row triple of `iterations` whole vectors.
struct XybRows<'a> {
    rows: [&'a mut [f32]; 3],
    iterations: usize,
    opsin_params: &'a OpsinParams,
    output_is_xyb: bool,
}
impl Kernel for XybRows<'_> {
    type Output = ();
    #[inline(always)]
    fn run<D: SimdF32>(self, d: D) {
        let [row0, row1, row2] = self.rows;
        if self.output_is_xyb {
            let scale_x = d.set(SCALED_XYB_SCALE[0]);
            let scale_y = d.set(SCALED_XYB_SCALE[1]);
            let scale_bmy = d.set(SCALED_XYB_SCALE[2]);
            let offset_x = d.set(SCALED_XYB_OFFSET[0]);
            let offset_y = d.set(SCALED_XYB_OFFSET[1]);
            let offset_bmy = d.set(SCALED_XYB_OFFSET[2]);
            for i in 0..self.iterations {
                let x = i * D::LANES;
                let in_x = d.load(&row0[x..]);
                let in_y = d.load(&row1[x..]);
                let in_b = d.load(&row2[x..]);
                let out_x = d.mul(d.add(in_x, offset_x), scale_x);
                let out_y = d.mul(d.add(in_y, offset_y), scale_y);
                let out_b = d.mul(d.add(d.sub(in_b, in_y), offset_bmy), scale_bmy);
                d.store(out_x, &mut row0[x..]);
                d.store(out_y, &mut row1[x..]);
                d.store(out_b, &mut row2[x..]);
            }
        } else {
            for i in 0..self.iterations {
                let x = i * D::LANES;
                let in_opsin_x = d.load(&row0[x..]);
                let in_opsin_y = d.load(&row1[x..]);
                let in_opsin_b = d.load(&row2[x..]);
                let [r, g, b] =
                    xyb_to_rgb(d, in_opsin_x, in_opsin_y, in_opsin_b, self.opsin_params);
                d.store(r, &mut row0[x..]);
                d.store(g, &mut row1[x..]);
                d.store(b, &mut row2[x..]);
            }
        }
    }
}

pub struct XybStageSimd {
    target: Target,
    opsin_params: OpsinParams,
    output_is_xyb: bool,
}
impl RenderPipelineStage for XybStageSimd {
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
        // `for (x = 0; x < xsize; x += Lanes(d))`: whole vectors up to
        // RoundUpTo(xsize, Lanes(d)), in the row padding.
        let lanes = self.target.lanes();
        let (_, iterations) = vector_span(0, 0, width, lanes);
        let len = iterations * lanes;
        let rows = [0, 1, 2].map(|c| self.get_input_row(input, c, 0));
        // The source's loads and stores through its row pointers, in place.
        let spans = rows.map(|row| Span { row, from: 0, len });
        with_spans(buffers, [], spans, |[], [a, b, c]| {
            dispatch(
                self.target,
                XybRows {
                    rows: [a, b, c],
                    iterations,
                    opsin_params: &self.opsin_params,
                    output_is_xyb: self.output_is_xyb,
                },
            )
        });
        Ok(())
    }
    fn get_channel_mode(&self, c: usize) -> Mode {
        if c < 3 { Mode::InPlace } else { Mode::Ignored }
    }
    fn get_name(&self) -> &str {
        "XYB"
    }
}

/// `GetXYBStage` instantiated for one SIMD target.
pub fn get_xyb_stage_simd(
    info: &OutputEncodingInfo,
    target: Target,
) -> Box<dyn RenderPipelineStage> {
    Box::new(XybStageSimd {
        target,
        opsin_params: info.opsin_params.clone(),
        output_is_xyb: info.color_encoding.get_color_space() == ColorSpace::XYB,
    })
}
