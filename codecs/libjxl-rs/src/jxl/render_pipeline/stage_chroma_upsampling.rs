// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
//! Complete scalar Horizontal/VerticalChromaUpsamplingStage and factory from
//! stage_chroma_upsampling.{h,cc}. Borrowed row descriptors replace pointers.
use super::render_pipeline_stage::{
    RenderPipelineChannelMode as Mode, RenderPipelineStage, Row, RowBuffers, RowInfo, Settings,
};
use crate::jxl::base::status::Status;
// Copyright 2019 Google LLC
// Copyright (c) The Highway Project Authors. All rights reserved.
// SPDX-License-Identifier: BSD-3-Clause
// Scalar Add/MulAdd/operator* from hwy/ops/scalar-inl.h, in the source
// operand order on every target (NaN payloads are not gated, decision D8).
#[inline]
fn add(a: f32, b: f32) -> f32 {
    a + b
}
#[inline]
fn promote(a: f32) -> f64 {
    f64::from(a)
}
#[inline]
fn product(a: f64, b: f64) -> f32 {
    (a * b) as f32
}
#[inline]
fn mul(a: f32, b: f32) -> f32 {
    product(promote(a), promote(b))
}
#[inline]
fn ma(a: f32, b: f32, c: f32) -> f32 {
    add(mul(a, b), c)
}
pub struct HorizontalChromaUpsamplingStage {
    channel: usize,
    /// Output samples of one call, written back after the loop.
    scratch: Vec<f32>,
}
pub struct VerticalChromaUpsamplingStage {
    channel: usize,
    /// Output samples of one call for both output rows.
    scratch: [Vec<f32>; 2],
}
impl RenderPipelineStage for HorizontalChromaUpsamplingStage {
    fn settings(&self) -> Settings {
        Settings::shift_x(1, 1)
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
        let row = self.get_input_row(input, self.channel, 0);
        let out = self.get_output_row(output, self.channel, 0);
        // The original's row pointer: the input span is resolved once per
        // call. Outputs are staged because input and output rows may share a
        // plane here.
        let span = buffers.row(Row { x: 0, ..row });
        let at = |x: isize| row.x.wrapping_add_signed(x);
        let mut scratch = std::mem::take(&mut self.scratch);
        scratch.clear();
        let x_begin = -(left as isize);
        for x in x_begin..width.wrapping_add(right) as isize {
            let current = mul(span[at(x)], 0.75);
            let prev = span[at(x - 1)];
            let next = span[at(x + 1)];
            // MulAdd(onefour, prev, current), MulAdd(onefour, next, current).
            let l = ma(0.25, prev, current);
            let r = ma(0.25, next, current);
            scratch.push(l);
            scratch.push(r);
        }
        let target = buffers.row_mut(out.offset(x_begin * 2));
        target[..scratch.len()].copy_from_slice(&scratch);
        self.scratch = scratch;
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
        "HChromaUps"
    }
}
impl RenderPipelineStage for VerticalChromaUpsamplingStage {
    fn settings(&self) -> Settings {
        Settings::shift_y(1, 1)
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
        let top = self.get_input_row(input, self.channel, -1);
        let mid = self.get_input_row(input, self.channel, 0);
        let bot = self.get_input_row(input, self.channel, 1);
        let out0 = self.get_output_row(output, self.channel, 0);
        let out1 = self.get_output_row(output, self.channel, 1);
        // Resolve the input spans once per call and stage both output rows.
        let (t_row, m_row, b_row) = (
            buffers.row(Row { x: 0, ..top }),
            buffers.row(Row { x: 0, ..mid }),
            buffers.row(Row { x: 0, ..bot }),
        );
        let [mut s0, mut s1] = std::mem::take(&mut self.scratch);
        s0.clear();
        s1.clear();
        let x_begin = -(left as isize);
        for x in x_begin..width.wrapping_add(right) as isize {
            let it = t_row[top.x.wrapping_add_signed(x)];
            let im = m_row[mid.x.wrapping_add_signed(x)];
            let ib = b_row[bot.x.wrapping_add_signed(x)];
            let scaled = mul(im, 0.75);
            s0.push(ma(it, 0.25, scaled));
            // Original top and bottom stores keep distinct addition operands.
            s1.push(ma(ib, 0.25, scaled));
        }
        let target = buffers.row_mut(out0.offset(x_begin));
        target[..s0.len()].copy_from_slice(&s0);
        let target = buffers.row_mut(out1.offset(x_begin));
        target[..s1.len()].copy_from_slice(&s1);
        self.scratch = [s0, s1];
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
        "VChromaUps"
    }
}
pub fn get_chroma_upsampling_stage(
    channel: usize,
    horizontal: bool,
) -> Box<dyn RenderPipelineStage> {
    // HWY_DYNAMIC_DISPATCH(GetChromaUpsamplingStage) (port ADR 0004).
    if let Some(target) = crate::jxl::hwy::decoder_target() {
        return super::stage_chroma_simd::get_chroma_upsampling_stage_simd(
            channel, horizontal, target,
        );
    }
    if horizontal {
        Box::new(HorizontalChromaUpsamplingStage {
            channel,
            scratch: Vec::new(),
        })
    } else {
        Box::new(VerticalChromaUpsamplingStage {
            channel,
            scratch: Default::default(),
        })
    }
}
