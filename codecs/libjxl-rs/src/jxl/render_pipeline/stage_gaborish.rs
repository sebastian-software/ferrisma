// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
//! Complete one-lane GaborishStage and factory from stage_gaborish.{h,cc}.
//! Existing owner-relative row descriptors replace source pointers. Scalar
//! Highway loads/stores have one lane, so rounding the left extra span retains
//! its original value. SIMD specializations remain a separate boundary.
use super::render_pipeline_stage::{
    RenderPipelineChannelMode as Mode, RenderPipelineStage, Row, RowBuffers, RowInfo, Settings,
};
use super::scalar_rows::{self as k, Broadcast, lanes};
use crate::jxl::base::float::mul_add_f32;
use crate::jxl::base::status::Status;
use crate::jxl::loop_filter::LoopFilter;

// Copyright 2019 Google LLC
// Copyright (c) The Highway Project Authors. All rights reserved.
// SPDX-License-Identifier: BSD-3-Clause
// Use of this derived helper code is governed by Highway's offered BSD
// alternative in licenses/HIGHWAY-BSD-3-Clause.txt.
// Used Add/operator*/MulAdd from hwy/ops/scalar-inl.h. Keep the original
// scalar operator boundaries in the dedicated reference TU.
// Inlining may commute operands with different NaN payloads. Highway's float
// multiplication explicitly promotes both operands to double before narrowing;
// promotion also quiets signaling NaNs before the product chooses its payload.
#[inline(never)]
fn add(a: f32, b: f32) -> f32 {
    a + b
}
// The pixel loop uses the row kernels of `scalar_rows`, which keep these
// add/promote/product boundaries per operation over a whole row.

// The pinned x86 plain constructor object multiplies reciprocal first; its
// instrumented third weight keeps that order too. This is ordinary binary32
// arithmetic, separate from Highway's explicitly widened pixel multiplication.
#[cfg(target_arch = "x86_64")]
#[inline(never)]
fn normalize_mul(reciprocal: f32, weight: f32) -> f32 {
    reciprocal * weight
}

pub struct GaborishStage {
    weights: [f32; 9],
    /// One output row, written back after the row's samples are computed.
    scratch: Vec<f32>,
    /// Intermediate rows of the row kernels.
    sums: [Vec<f32>; 7],
    wide: Vec<f64>,
    /// The promoted weights as rows.
    weight_rows: [Broadcast; 9],
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
            #[cfg(target_arch = "x86_64")]
            {
                weights[3 * c] = normalize_mul(mul, weights[3 * c]);
                weights[3 * c + 1] = normalize_mul(mul, weights[3 * c + 1]);
                weights[3 * c + 2] = normalize_mul(mul, weights[3 * c + 2]);
            }
            #[cfg(not(target_arch = "x86_64"))]
            {
                weights[3 * c] *= mul;
                weights[3 * c + 1] *= mul;
                weights[3 * c + 2] *= mul;
            }
        }
        Self {
            weights,
            scratch: Vec::new(),
            sums: Default::default(),
            wide: Vec::new(),
            weight_rows: Default::default(),
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
            let [w0, w1, w2] = [0, 1, 2].map(|i| self.weights[3 * c + i]);
            // Resolve each row span once per row instead of once per sample,
            // and stage the outputs in a scratch row: input and output rows
            // may share one plane. Each lane performs the original operations
            // on the same operands in the same order; the row kernels keep
            // the out-of-line Add/promote/product boundaries per operation.
            let n = x_end.wrapping_sub(x_start) as usize;
            let tap = |row: Row, dx: isize| {
                let span = buffers.row(Row { x: 0, ..row });
                &span[row.x.wrapping_add_signed(x_start + dx)..]
            };
            let (t, tl, tr) = (tap(row_t, 0), tap(row_t, -1), tap(row_t, 1));
            let (m, l, r) = (tap(row_m, 0), tap(row_m, -1), tap(row_m, 1));
            let (b, bl, br) = (tap(row_b, 0), tap(row_b, -1), tap(row_b, 1));
            let [s0, s1, sum1, sum2, p0, p1, p2] = &mut self.sums;
            let (s0, s1, sum1, sum2) = (lanes(s0, n), lanes(s1, n), lanes(sum1, n), lanes(sum2, n));
            let (p0, p1, p2) = (lanes(p0, n), lanes(p1, n), lanes(p2, n));
            let wide = lanes(&mut self.wide, n);
            let [wr0, wr1, wr2] = &mut self.weight_rows[3 * c..3 * c + 3] else {
                unreachable!()
            };
            // sum1 = add(add(l, r), add(t, b)); sum2 = add(add(tl, tr), add(bl, br))
            k::add(s0, l, r);
            k::add(s1, t, b);
            k::add(sum1, s0, s1);
            k::add(s0, tl, tr);
            k::add(s1, bl, br);
            k::add(sum2, s0, s1);
            // Literal original MulAdd(sum2,w2,MulAdd(sum1,w1,Mul(sum0,w0))).
            // Each scalar multiply and add retains its own rounding.
            k::promote(wide, sum2);
            k::product(p2, wide, wr2.promoted(w2, n));
            k::promote(wide, sum1);
            k::product(p1, wide, wr1.promoted(w1, n));
            k::promote(wide, m);
            k::product(p0, wide, wr0.promoted(w0, n));
            k::add(s0, p1, p0);
            k::add(lanes(&mut self.scratch, n), p2, s0);
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
    Some(Box::new(GaborishStage::new(lf)))
}
