// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
//! The SIMD GaborishStage of render_pipeline/stage_gaborish.cc for the NEON
//! and AVX2 targets (ADR 0004), written once against [`SimdF32`]. Compare it
//! only with the original's SIMD build at the same target.
use super::render_pipeline_stage::{
    RenderPipelineChannelMode as Mode, RenderPipelineStage, RowBuffers, RowInfo, Settings, Span,
};
use super::simd_rows::{vector_span, with_spans};
use crate::jxl::base::status::Status;
use crate::jxl::hwy::{Kernel, SimdF32, Target, dispatch};
use crate::jxl::loop_filter::LoopFilter;

pub struct GaborishStageSimd {
    target: Target,
    weights: [f32; 9],
}
impl GaborishStageSimd {
    fn new(lf: &LoopFilter, target: Target) -> Self {
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
            // The constructor lies inside HWY_NAMESPACE, so it carries the
            // target attributes: FMA is available on NEON and AVX2 and Clang
            // contracts `w0 + 4 * (w1 + w2)` on both targets.
            let div = 4.0f32.mul_add(weights[3 * c + 1] + weights[3 * c + 2], weights[3 * c]);
            let mul = 1.0 / div;
            weights[3 * c] *= mul;
            weights[3 * c + 1] *= mul;
            weights[3 * c + 2] *= mul;
        }
        Self { target, weights }
    }
}

/// One channel row: `rows` are the top/middle/bottom input samples from
/// `x_start - 1` on; `out` receives the samples from `x_start` on.
struct GabRow<'a> {
    rows: [&'a [f32]; 3],
    w: [f32; 3],
    iterations: usize,
    out: &'a mut [f32],
}
impl Kernel for GabRow<'_> {
    type Output = ();
    #[inline(always)]
    fn run<D: SimdF32>(self, d: D) {
        let [row_t, row_m, row_b] = self.rows;
        let w0 = d.set(self.w[0]);
        let w1 = d.set(self.w[1]);
        let w2 = d.set(self.w[2]);
        // Each vector reads one window of `Lanes + 2` samples per row (the
        // source's row + x - 1 .. row + x + Lanes + 1) and writes one output
        // vector, so the nine loads and the store need no per-load checks.
        let n = self.iterations * D::LANES;
        for (i, out) in self.out[..n].chunks_exact_mut(D::LANES).enumerate() {
            let o = i * D::LANES;
            let wt = &row_t[o..o + D::LANES + 2];
            let wm = &row_m[o..o + D::LANES + 2];
            let wb = &row_b[o..o + D::LANES + 2];
            let t = d.load(&wt[1..]);
            let tl = d.load(wt);
            let tr = d.load(&wt[2..]);
            let m = d.load(&wm[1..]);
            let l = d.load(wm);
            let r = d.load(&wm[2..]);
            let b = d.load(&wb[1..]);
            let bl = d.load(wb);
            let br = d.load(&wb[2..]);
            let sum0 = m;
            let sum1 = d.add(d.add(l, r), d.add(t, b));
            let sum2 = d.add(d.add(tl, tr), d.add(bl, br));
            let pixels = d.mul_add(sum2, w2, d.mul_add(sum1, w1, d.mul(sum0, w0)));
            d.store(pixels, out);
        }
    }
}

impl RenderPipelineStage for GaborishStageSimd {
    fn as_concurrent(&self) -> Option<&(dyn RenderPipelineStage + Sync)> {
        Some(self)
    }
    fn settings(&self) -> Settings {
        Settings::symmetric_border_only(1)
    }
    fn process_row(
        &self,
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
        let lanes = self.target.lanes();
        let (x_start, iterations) = vector_span(left, right, width, lanes);
        let len = iterations * lanes;
        // stage_gaborish.cc:65-97: row_t/row_m/row_b and row_out are the
        // pipeline's rows, loaded and stored in place.
        for c in 0..3 {
            let reads = [-1, 0, 1].map(|dy| Span {
                row: self.get_input_row(input, c, dy),
                from: x_start - 1,
                len: len + 2,
            });
            let writes = [Span {
                row: self.get_output_row(output, c, 0),
                from: x_start,
                len,
            }];
            let w = [0, 1, 2].map(|i| self.weights[3 * c + i]);
            with_spans(buffers, reads, writes, |rows, [out]| {
                dispatch(
                    self.target,
                    GabRow {
                        rows,
                        w,
                        iterations,
                        out,
                    },
                )
            });
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

/// `GetGaborishStage` instantiated for one SIMD target.
pub fn get_gaborish_stage_simd(
    lf: &LoopFilter,
    target: Target,
) -> Option<Box<dyn RenderPipelineStage>> {
    if !lf.gab {
        return None;
    }
    Some(Box::new(GaborishStageSimd::new(lf, target)))
}
