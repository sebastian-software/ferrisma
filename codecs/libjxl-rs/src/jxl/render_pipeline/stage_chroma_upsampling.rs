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
// The row kernels of `scalar_rows` keep these Add/promote/product boundaries
// per operation over a whole row.
use super::scalar_rows::{self as k, Broadcast, lanes};

// The selected original Linux x86_64 keeps the neighbor product first; the
// established ARM mapping keeps the current value first. Retain NaN operand order.
fn neighbor_add(out: &mut [f32], neighbor: &[f32], current: &[f32]) {
    #[cfg(all(target_os = "linux", target_arch = "x86_64"))]
    {
        k::add(out, neighbor, current)
    }
    #[cfg(not(all(target_os = "linux", target_arch = "x86_64")))]
    {
        k::add(out, current, neighbor)
    }
}

/// Scratch rows of the row kernels.
#[derive(Default)]
struct ChromaRows {
    f: [Vec<f32>; 4],
    d: [Vec<f64>; 3],
    /// The promoted 0.75 and 0.25.
    scalars: [Broadcast; 2],
}
pub struct HorizontalChromaUpsamplingStage {
    channel: usize,
    /// Output samples of one call, written back after the loop.
    scratch: Vec<f32>,
    rows: ChromaRows,
}
pub struct VerticalChromaUpsamplingStage {
    channel: usize,
    /// Output samples of one call for both output rows.
    scratch: [Vec<f32>; 2],
    rows: ChromaRows,
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
        let mut scratch = std::mem::take(&mut self.scratch);
        scratch.clear();
        let x_begin = -(left as isize);
        let n = (width.wrapping_add(right) as isize).wrapping_sub(x_begin) as usize;
        // Lane by lane through the row kernels: current = mul(in[x], 0.75),
        // l = neighbor_add(mul(0.25, in[x - 1]), current) and r likewise with
        // in[x + 1]. The span is promoted once over n + 2 lanes; promotion is
        // exact and deterministic, so shifted views equal per-tap promotion.
        let ChromaRows { f, d, scalars } = &mut self.rows;
        let [current, neighbor, l, r] = f;
        let [wide, _, _] = d;
        let [s075, s025] = scalars;
        let (current, neighbor) = (lanes(current, n), lanes(neighbor, n));
        let (l, r) = (lanes(l, n), lanes(r, n));
        let wide = lanes(wide, n + 2);
        k::promote(wide, &span[row.x.wrapping_add_signed(x_begin - 1)..]);
        k::product(current, &wide[1..], s075.promoted(0.75, n));
        let quarter = s025.promoted(0.25, n);
        k::product(neighbor, quarter, &wide[..n]);
        neighbor_add(l, neighbor, current);
        k::product(neighbor, quarter, &wide[2..]);
        neighbor_add(r, neighbor, current);
        for (&l, &r) in l.iter().zip(&*r) {
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
        let x_begin = -(left as isize);
        let n = (width.wrapping_add(right) as isize).wrapping_sub(x_begin) as usize;
        let at = |row: Row| row.x.wrapping_add_signed(x_begin);
        let (it, im, ib) = (&t_row[at(top)..], &m_row[at(mid)..], &b_row[at(bot)..]);
        // Lane by lane through the row kernels: scaled = mul(im, 0.75),
        // out0 = ma(it, 0.25, scaled) = add(mul(it, 0.25), scaled) and
        // out1 = neighbor_add(mul(ib, 0.25), scaled).
        let ChromaRows { f, d, scalars } = &mut self.rows;
        let [scaled, product, _, _] = f;
        let [wm, wt, wb] = d;
        let [s075, s025] = scalars;
        let (scaled, product) = (lanes(scaled, n), lanes(product, n));
        let (wm, wt, wb) = (lanes(wm, n), lanes(wt, n), lanes(wb, n));
        k::promote(wm, im);
        k::product(scaled, wm, s075.promoted(0.75, n));
        let quarter = s025.promoted(0.25, n);
        k::promote(wt, it);
        k::product(product, wt, quarter);
        k::add(lanes(&mut s0, n), product, scaled);
        k::promote(wb, ib);
        k::product(product, wb, quarter);
        neighbor_add(lanes(&mut s1, n), product, scaled);
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
            rows: ChromaRows::default(),
        })
    } else {
        Box::new(VerticalChromaUpsamplingStage {
            channel,
            scratch: Default::default(),
            rows: ChromaRows::default(),
        })
    }
}
