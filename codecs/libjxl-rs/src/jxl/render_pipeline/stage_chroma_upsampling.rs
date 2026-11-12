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
// Scalar Add/MulAdd/operator* from hwy/ops/scalar-inl.h.
#[inline(never)]
fn add(a: f32, b: f32) -> f32 {
    a + b
}
#[inline(never)]
fn promote(a: f32) -> f64 {
    f64::from(a)
}
#[inline(never)]
fn product(a: f64, b: f64) -> f32 {
    (a * b) as f32
}
fn mul(a: f32, b: f32) -> f32 {
    product(promote(a), promote(b))
}
fn ma(a: f32, b: f32, c: f32) -> f32 {
    add(mul(a, b), c)
}
// The selected original Linux x86_64 keeps the neighbor product first; the
// established ARM mapping keeps the current value first. Retain NaN operand order.
fn neighbor_add(neighbor: f32, current: f32) -> f32 {
    #[cfg(all(target_os = "linux", target_arch = "x86_64"))]
    {
        add(neighbor, current)
    }
    #[cfg(not(all(target_os = "linux", target_arch = "x86_64")))]
    {
        add(current, neighbor)
    }
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
        // Resolve the input span once per call and stage the outputs: input
        // and output rows may share a plane. Values, order and arithmetic are
        // unchanged; only addressing is hoisted.
        let span = buffers.row(Row { x: 0, ..row });
        let at = |x: isize| row.x.wrapping_add_signed(x);
        let mut scratch = std::mem::take(&mut self.scratch);
        scratch.clear();
        let x_begin = -(left as isize);
        for x in x_begin..width.wrapping_add(right) as isize {
            let current = mul(span[at(x)], 0.75);
            let prev = span[at(x - 1)];
            let next = span[at(x + 1)];
            // Keep the selected original target-specific addition operands.
            let l = neighbor_add(mul(0.25, prev), current);
            let r = neighbor_add(mul(0.25, next), current);
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
            s1.push(neighbor_add(mul(ib, 0.25), scaled));
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
