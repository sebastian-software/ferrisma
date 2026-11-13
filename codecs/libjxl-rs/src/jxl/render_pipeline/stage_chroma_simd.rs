// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
//! The SIMD kYCbCrStage (render_pipeline/stage_ycbcr.cc) and the horizontal
//! and vertical chroma upsampling stages (stage_chroma_upsampling.cc) for the
//! NEON and AVX2 targets (ADR 0004), written once against [`SimdF32`].
//! Compare them only with the original's SIMD build at the same target.
use super::render_pipeline_stage::{
    RenderPipelineChannelMode as Mode, RenderPipelineStage, RowBuffers, RowInfo, Settings,
};
use super::simd_rows::{read, vector_span, write};
use crate::jxl::base::status::{Status, StatusCode};
use crate::jxl::hwy::{Kernel, SimdF32, Target, dispatch};

struct YCbCrRows<'a> {
    rows: [&'a mut [f32]; 3],
    iterations: usize,
}
impl Kernel for YCbCrRows<'_> {
    type Output = ();
    #[inline(always)]
    fn run<D: SimdF32>(self, df: D) {
        // Full-range BT.601 as defined by JFIF Clause 7; the constants are
        // the source's binary32 expressions.
        let c128 = df.set(128.0f32 / 255.0);
        let crcr = df.set(1.402f32);
        let cgcb = df.set(-0.114f32 * 1.772 / 0.587);
        let cgcr = df.set(-0.299f32 * 1.402 / 0.587);
        let cbcb = df.set(1.772f32);
        let [row0, row1, row2] = self.rows;
        for i in 0..self.iterations {
            let x = i * D::LANES;
            let y_vec = df.add(df.load(&row1[x..]), c128);
            let cb_vec = df.load(&row0[x..]);
            let cr_vec = df.load(&row2[x..]);
            let r_vec = df.mul_add(crcr, cr_vec, y_vec);
            let g_vec = df.mul_add(cgcr, cr_vec, df.mul_add(cgcb, cb_vec, y_vec));
            let b_vec = df.mul_add(cbcb, cb_vec, y_vec);
            df.store(r_vec, &mut row0[x..]);
            df.store(g_vec, &mut row1[x..]);
            df.store(b_vec, &mut row2[x..]);
        }
    }
}

pub struct YCbCrStageSimd {
    target: Target,
}
impl RenderPipelineStage for YCbCrStageSimd {
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
        if left != 0 || right != 0 {
            return Err(StatusCode::GenericError);
        }
        let lanes = self.target.lanes();
        let (_, iterations) = vector_span(0, 0, width, lanes);
        let len = iterations * lanes;
        let rows = [0, 1, 2].map(|c| self.get_input_row(input, c, 0));
        let mut values = rows.map(|row| read(buffers, row, 0, len));
        let [a, b, c] = &mut values;
        dispatch(
            self.target,
            YCbCrRows {
                rows: [a, b, c],
                iterations,
            },
        );
        for (row, values) in rows.into_iter().zip(&values) {
            write(buffers, row, 0, values);
        }
        Ok(())
    }
    fn get_channel_mode(&self, c: usize) -> Mode {
        if c < 3 { Mode::InPlace } else { Mode::Ignored }
    }
    fn get_name(&self) -> &str {
        "YCbCr"
    }
}

/// `GetYCbCrStage` instantiated for one SIMD target.
pub fn get_ycbcr_stage_simd(target: Target) -> Box<dyn RenderPipelineStage> {
    Box::new(YCbCrStageSimd { target })
}

/// Horizontal: `row_in` from `x_start - 1`; `out` receives `2 * len` samples.
struct HorizontalRow<'a> {
    row_in: &'a [f32],
    iterations: usize,
    out: &'a mut [f32],
}
impl Kernel for HorizontalRow<'_> {
    type Output = ();
    #[inline(always)]
    fn run<D: SimdF32>(self, df: D) {
        let threefour = df.set(0.75);
        let onefour = df.set(0.25);
        for i in 0..self.iterations {
            let x = i * D::LANES + 1;
            let current = df.mul(df.load(&self.row_in[x..]), threefour);
            let prev = df.load(&self.row_in[x - 1..]);
            let next = df.load(&self.row_in[x + 1..]);
            let left = df.mul_add(onefour, prev, current);
            let right = df.mul_add(onefour, next, current);
            df.store_interleaved2(left, right, &mut self.out[2 * (x - 1)..]);
        }
    }
}

/// Vertical: rows from `x_start`; two output rows of `len` samples.
struct VerticalRow<'a> {
    rows: [&'a [f32]; 3],
    iterations: usize,
    out: [&'a mut [f32]; 2],
}
impl Kernel for VerticalRow<'_> {
    type Output = ();
    #[inline(always)]
    fn run<D: SimdF32>(self, df: D) {
        let threefour = df.set(0.75);
        let onefour = df.set(0.25);
        let [row_top, row_mid, row_bot] = self.rows;
        let [out0, out1] = self.out;
        for i in 0..self.iterations {
            let x = i * D::LANES;
            let it = df.load(&row_top[x..]);
            let im = df.load(&row_mid[x..]);
            let ib = df.load(&row_bot[x..]);
            let im_scaled = df.mul(im, threefour);
            df.store(df.mul_add(it, onefour, im_scaled), &mut out0[x..]);
            df.store(df.mul_add(ib, onefour, im_scaled), &mut out1[x..]);
        }
    }
}

pub struct ChromaUpsamplingStageSimd {
    target: Target,
    channel: usize,
    horizontal: bool,
}
impl RenderPipelineStage for ChromaUpsamplingStageSimd {
    fn settings(&self) -> Settings {
        if self.horizontal {
            Settings::shift_x(1, 1)
        } else {
            Settings::shift_y(1, 1)
        }
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
        let lanes = self.target.lanes();
        let (x_start, iterations) = vector_span(left, right, width, lanes);
        let len = iterations * lanes;
        let c = self.channel;
        if self.horizontal {
            let row_in = read(
                buffers,
                self.get_input_row(input, c, 0),
                x_start - 1,
                len + 2,
            );
            let mut out = vec![0.0; 2 * len];
            dispatch(
                self.target,
                HorizontalRow {
                    row_in: &row_in,
                    iterations,
                    out: &mut out,
                },
            );
            write(
                buffers,
                self.get_output_row(output, c, 0),
                2 * x_start,
                &out,
            );
        } else {
            let rows =
                [-1, 0, 1].map(|dy| read(buffers, self.get_input_row(input, c, dy), x_start, len));
            let (mut out0, mut out1) = (vec![0.0; len], vec![0.0; len]);
            dispatch(
                self.target,
                VerticalRow {
                    rows: [&rows[0], &rows[1], &rows[2]],
                    iterations,
                    out: [&mut out0, &mut out1],
                },
            );
            write(buffers, self.get_output_row(output, c, 0), x_start, &out0);
            write(buffers, self.get_output_row(output, c, 1), x_start, &out1);
        }
        Ok(())
    }
    fn get_channel_mode(&self, c: usize) -> Mode {
        if c == self.channel {
            Mode::InOut
        } else {
            Mode::Ignored
        }
    }
    fn get_name(&self) -> &str {
        if self.horizontal {
            "HChromaUps"
        } else {
            "VChromaUps"
        }
    }
}

/// `GetChromaUpsamplingStage` instantiated for one SIMD target.
pub fn get_chroma_upsampling_stage_simd(
    channel: usize,
    horizontal: bool,
    target: Target,
) -> Box<dyn RenderPipelineStage> {
    Box::new(ChromaUpsamplingStageSimd {
        target,
        channel,
        horizontal,
    })
}
