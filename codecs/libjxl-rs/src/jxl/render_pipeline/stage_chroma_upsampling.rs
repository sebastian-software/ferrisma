// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
//! Complete scalar Horizontal/VerticalChromaUpsamplingStage and factory from
//! stage_chroma_upsampling.{h,cc}. Borrowed row descriptors replace pointers.
use super::render_pipeline_stage::{
    RenderPipelineChannelMode as Mode, RenderPipelineStage, RowBuffers, RowInfo, Settings,
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
}
pub struct VerticalChromaUpsamplingStage {
    channel: usize,
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
        for x in -(left as isize)..width.wrapping_add(right) as isize {
            let current = mul(buffers.row(row.offset(x))[0], 0.75);
            let prev = buffers.row(row.offset(x - 1))[0];
            let next = buffers.row(row.offset(x + 1))[0];
            // Keep the selected original target-specific addition operands.
            let l = neighbor_add(mul(0.25, prev), current);
            let r = neighbor_add(mul(0.25, next), current);
            buffers.row_mut(out.offset(x * 2))[0] = l;
            buffers.row_mut(out.offset(x * 2 + 1))[0] = r;
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
        for x in -(left as isize)..width.wrapping_add(right) as isize {
            let it = buffers.row(top.offset(x))[0];
            let im = buffers.row(mid.offset(x))[0];
            let ib = buffers.row(bot.offset(x))[0];
            let scaled = mul(im, 0.75);
            buffers.row_mut(out0.offset(x))[0] = ma(it, 0.25, scaled);
            // Original top and bottom stores keep distinct addition operands.
            buffers.row_mut(out1.offset(x))[0] = neighbor_add(mul(ib, 0.25), scaled);
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
        "VChromaUps"
    }
}
pub fn get_chroma_upsampling_stage(
    channel: usize,
    horizontal: bool,
) -> Box<dyn RenderPipelineStage> {
    if horizontal {
        Box::new(HorizontalChromaUpsamplingStage { channel })
    } else {
        Box::new(VerticalChromaUpsamplingStage { channel })
    }
}
