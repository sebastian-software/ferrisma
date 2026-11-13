// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
//! Complete one-lane GaborishStage and factory from stage_gaborish.{h,cc}.
//! Existing owner-relative row descriptors replace source pointers. Scalar
//! Highway loads/stores have one lane, so rounding the left extra span retains
//! its original value. SIMD specializations remain a separate boundary.
use super::render_pipeline_stage::{
    RenderPipelineChannelMode as Mode, RenderPipelineStage, Row, RowBuffers, RowInfo, Settings,
};
use crate::jxl::base::float::mul_add_f32;
use crate::jxl::base::status::Status;
use crate::jxl::loop_filter::LoopFilter;

// Copyright 2019 Google LLC
// Copyright (c) The Highway Project Authors. All rights reserved.
// SPDX-License-Identifier: BSD-3-Clause
// Use of this derived helper code is governed by Highway's offered BSD
// alternative in licenses/HIGHWAY-BSD-3-Clause.txt.
// Used Add/operator*/MulAdd from hwy/ops/scalar-inl.h. Highway's float
// multiplication promotes both operands to double before narrowing. The
// helpers inline; which NaN payload survives is not part of the gate
// (decision D8; port ADR 0002).
#[inline]
fn add(a: f32, b: f32) -> f32 {
    a + b
}
#[inline]
fn promote(a: f32) -> f64 {
    f64::from(a)
}
#[inline]
fn multiply_and_demote(a: f64, b: f64) -> f32 {
    (a * b) as f32
}
#[inline]
fn mul(a: f32, b: f32) -> f32 {
    multiply_and_demote(promote(a), promote(b))
}

pub struct GaborishStage {
    weights: [f32; 9],
    /// One output row, written back after the row's samples are computed.
    scratch: Vec<f32>,
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
            let div = mul_add_f32(
                4.0,
                add(weights[3 * c + 1], weights[3 * c + 2]),
                weights[3 * c],
            );
            let mul = 1.0 / div;
            weights[3 * c] *= mul;
            weights[3 * c + 1] *= mul;
            weights[3 * c + 2] *= mul;
        }
        Self {
            weights,
            scratch: Vec::new(),
        }
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
            // The original's row pointers: each row span is resolved once per
            // row. Outputs are staged in a scratch row because input and
            // output rows may share one plane here.
            let (tx, mx, bx) = (row_t.x, row_m.x, row_b.x);
            let t_row = buffers.row(Row { x: 0, ..row_t });
            let m_row = buffers.row(Row { x: 0, ..row_m });
            let b_row = buffers.row(Row { x: 0, ..row_b });
            let at = |x0: usize, x: isize| x0.wrapping_add_signed(x);
            self.scratch.clear();
            for x in x_start..x_end {
                let t = t_row[at(tx, x)];
                let tl = t_row[at(tx, x - 1)];
                let tr = t_row[at(tx, x + 1)];
                let m = m_row[at(mx, x)];
                let l = m_row[at(mx, x - 1)];
                let r = m_row[at(mx, x + 1)];
                let b = b_row[at(bx, x)];
                let bl = b_row[at(bx, x - 1)];
                let br = b_row[at(bx, x + 1)];
                let sum0 = m;
                let sum1 = add(add(l, r), add(t, b));
                let sum2 = add(add(tl, tr), add(bl, br));
                // Literal original MulAdd(sum2,w2,MulAdd(sum1,w1,Mul(sum0,w0))).
                // Each scalar multiply and add retains its own rounding.
                let pixels = add(mul(sum2, w2), add(mul(sum1, w1), mul(sum0, w0)));
                self.scratch.push(pixels);
            }
            let out = buffers.row_mut(row_out.offset(x_start));
            out[..self.scratch.len()].copy_from_slice(&self.scratch);
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
    // HWY_DYNAMIC_DISPATCH(GetGaborishStage) (port ADR 0004).
    if let Some(target) = crate::jxl::hwy::decoder_target() {
        return super::stage_gaborish_simd::get_gaborish_stage_simd(lf, target);
    }
    Some(Box::new(GaborishStage::new(lf)))
}
