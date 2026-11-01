// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
//! Complete SimpleRenderPipeline from render_pipeline/simple_render_pipeline.cc
//! and its RenderPipeline base operations. SimpleRenderPipelineBuilder represents
//! the original Builder + UseSimpleImplementation selection. The default
//! LowMemoryRenderPipeline branch remains absent. Null-pool calls are sequential.
use super::render_pipeline_stage::{
    RENDER_PIPELINE_X_OFFSET as X_OFFSET, RenderPipelineChannelMode as Mode, RenderPipelineStage,
    Row, RowBuffers, RowInfo, RowStorage,
};
use crate::jxl::base::bits::ceil_log2_nonzero;
use crate::jxl::base::rect::Rect;
use crate::jxl::base::status::{Status, StatusCode, StatusOr};
use crate::jxl::frame_dimensions::FrameDimensions;
use crate::jxl::frame_header::FrameOrigin;
use crate::jxl::image::Plane;
use crate::jxl::image_ops::{copy_rect_to, mirror};
// hwy/base.h's architecture constant, independent of the scalar target's
// separate MaxVectorSize() function. RVV intrinsic builds remain unported.
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
const MAX_VECTOR_SIZE: usize = 64;
#[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
const MAX_VECTOR_SIZE: usize = 16;
fn div_ceil(n: usize, d: usize) -> usize {
    n.wrapping_add(d - 1) / d
}
pub struct SimpleRenderPipelineBuilder<'a> {
    stages: Vec<Box<dyn RenderPipelineStage + 'a>>,
    num_c: usize,
}
impl<'a> SimpleRenderPipelineBuilder<'a> {
    pub fn new(num_c: usize) -> Self {
        assert!(num_c > 0, "source channel-count precondition");
        Self {
            stages: Vec::new(),
            num_c,
        }
    }
    pub fn add_stage(&mut self, stage: Option<Box<dyn RenderPipelineStage + 'a>>) -> Status {
        self.stages.push(stage.ok_or(StatusCode::GenericError)?);
        Ok(())
    }
    pub fn finalize(self, dim: FrameDimensions) -> StatusOr<Box<SimpleRenderPipeline<'a>>> {
        if self.stages.is_empty() {
            return Err(StatusCode::GenericError);
        }
        for c in 0..self.num_c {
            if self.stages.last().unwrap().get_channel_mode(c) == Mode::InOut {
                return Err(StatusCode::GenericError);
            }
        }
        let mut res = Box::new(SimpleRenderPipeline {
            stages: Vec::new(),
            channel_shifts: Vec::new(),
            padding: Vec::new(),
            frame_dimensions: FrameDimensions::default(),
            group_completed_passes: Vec::new(),
            channel_data: Vec::new(),
            processed_passes: 0,
        });
        res.padding.resize(self.stages.len(), Vec::new());
        for i in (0..self.stages.len()).rev() {
            let stage = &self.stages[i];
            res.padding[i].resize(self.num_c, (0, 0));
            if i + 1 == self.stages.len() {
                continue;
            }
            for c in 0..self.num_c {
                res.padding[i][c] = if stage.get_channel_mode(c) == Mode::InOut {
                    let s = stage.settings();
                    (
                        div_ceil(res.padding[i + 1][c].0, 1usize << s.shift_x)
                            .wrapping_add(s.border_x),
                        div_ceil(res.padding[i + 1][c].1, 1usize << s.shift_y)
                            .wrapping_add(s.border_y),
                    )
                } else {
                    res.padding[i + 1][c]
                };
            }
        }
        res.frame_dimensions = dim;
        res.group_completed_passes.resize(dim.num_groups, 0);
        res.channel_shifts.resize(self.stages.len(), Vec::new());
        res.channel_shifts[0].resize(self.num_c, (0usize, 0usize));
        for i in 1..self.stages.len() {
            let stage = &self.stages[i - 1];
            for c in 0..self.num_c {
                if stage.get_channel_mode(c) == Mode::InOut {
                    res.channel_shifts[0][c].0 = res.channel_shifts[0][c]
                        .0
                        .wrapping_add(stage.settings().shift_x);
                    res.channel_shifts[0][c].1 = res.channel_shifts[0][c]
                        .1
                        .wrapping_add(stage.settings().shift_y);
                }
            }
        }
        for i in 1..self.stages.len() {
            let stage = &self.stages[i - 1];
            res.channel_shifts[i].resize(self.num_c, (0, 0));
            for c in 0..self.num_c {
                res.channel_shifts[i][c] = if stage.get_channel_mode(c) == Mode::InOut {
                    (
                        res.channel_shifts[i - 1][c]
                            .0
                            .wrapping_sub(stage.settings().shift_x),
                        res.channel_shifts[i - 1][c]
                            .1
                            .wrapping_sub(stage.settings().shift_y),
                    )
                } else {
                    res.channel_shifts[i - 1][c]
                };
            }
        }
        res.stages = self.stages;
        // SimpleRenderPipeline inherits RenderPipeline::Init's original success.
        Ok(res)
    }
}
pub struct SimpleRenderPipeline<'a> {
    stages: Vec<Box<dyn RenderPipelineStage + 'a>>,
    channel_shifts: Vec<Vec<(usize, usize)>>,
    padding: Vec<Vec<(usize, usize)>>,
    frame_dimensions: FrameDimensions,
    group_completed_passes: Vec<u8>,
    channel_data: Vec<Plane<f32>>,
    processed_passes: usize,
}
impl<'a> SimpleRenderPipeline<'a> {
    pub fn is_initialized(&self) -> Status {
        for stage in &self.stages {
            stage.is_initialized()?;
        }
        Ok(())
    }
    pub fn prepare_for_threads(&mut self, num: usize, use_group_ids: bool) -> Status {
        for stage in &mut self.stages {
            stage.prepare_for_threads(num)?;
        }
        self.prepare_for_threads_internal(num, use_group_ids)
    }
    fn prepare_for_threads_internal(&mut self, _num: usize, _use_group_ids: bool) -> Status {
        if !self.channel_data.is_empty() {
            return Ok(());
        }
        for &(xs, ys) in &self.channel_shifts[0] {
            self.channel_data.push(Plane::create(
                div_ceil(self.frame_dimensions.xsize_upsampled, 1usize << xs)
                    .wrapping_add(X_OFFSET * 2),
                div_ceil(self.frame_dimensions.ysize_upsampled, 1usize << ys)
                    .wrapping_add(X_OFFSET * 2),
            )?);
            // MSan poisoning is diagnostic-only. Safe native storage is initialized.
        }
        Ok(())
    }
    fn make_channel_rect(&self, group: usize, c: usize) -> Rect {
        let base = ceil_log2_nonzero(
            self.frame_dimensions.xsize_upsampled_padded / self.frame_dimensions.xsize_padded,
        );
        let gx = group % self.frame_dimensions.xsize_groups;
        let gy = group / self.frame_dimensions.xsize_groups;
        let (xs, ys) = self.channel_shifts[0][c];
        let xdim = (self.frame_dimensions.group_dim << base) >> xs;
        let ydim = (self.frame_dimensions.group_dim << base) >> ys;
        Rect::new_clamped(
            X_OFFSET.wrapping_add(gx.wrapping_mul(xdim)),
            X_OFFSET.wrapping_add(gy.wrapping_mul(ydim)),
            xdim,
            ydim,
            X_OFFSET.wrapping_add(div_ceil(
                self.frame_dimensions.xsize_upsampled,
                1usize << xs,
            )),
            X_OFFSET.wrapping_add(div_ceil(
                self.frame_dimensions.ysize_upsampled,
                1usize << ys,
            )),
        )
    }
    pub fn get_input_buffers(
        &mut self,
        group: usize,
        thread: usize,
    ) -> RenderPipelineInput<'_, 'a> {
        assert!(group < self.group_completed_passes.len());
        let buffers = (0..self.channel_data.len())
            .map(|c| (c, self.make_channel_rect(group, c)))
            .collect();
        RenderPipelineInput {
            pipeline: Some(self),
            group_id: group,
            thread_id: thread,
            buffers,
        }
    }
    pub fn passes_with_all_input(&self) -> usize {
        usize::from(
            *self
                .group_completed_passes
                .iter()
                .min()
                .expect("source requires at least one group"),
        )
    }
    // The original Simple implementation inherits this empty base operation.
    pub fn clear_done(&mut self, _group: usize) {}
    fn input_ready(&mut self, group: usize, thread: usize, _buffers: &[(usize, Rect)]) -> Status {
        if group >= self.group_completed_passes.len() {
            return Err(StatusCode::GenericError);
        }
        self.group_completed_passes[group] = self.group_completed_passes[group].wrapping_add(1);
        self.process_buffers(group, thread)
    }
    fn process_buffers(&mut self, group: usize, thread: usize) -> Status {
        for c in 0..self.channel_data.len() {
            let _r = self.make_channel_rect(group, c);
        }
        if self.passes_with_all_input() <= self.processed_passes {
            return Ok(());
        }
        self.processed_passes = self.processed_passes.wrapping_add(1);
        for stage_id in 0..self.stages.len() {
            let stage = &mut self.stages[stage_id];
            let settings = stage.settings();
            let mut new_channels: Vec<Plane<f32>> = (0..self.channel_data.len())
                .map(|_| Plane::default())
                .collect();
            let mut output_channels = vec![false; self.channel_data.len()];
            let input_sizes: Vec<_> = self
                .channel_data
                .iter()
                .map(|ch| {
                    (
                        ch.xsize().wrapping_sub(X_OFFSET * 2),
                        ch.ysize().wrapping_sub(X_OFFSET * 2),
                    )
                })
                .collect();
            for c in 0..self.channel_data.len() {
                if stage.get_channel_mode(c) != Mode::InOut {
                    continue;
                }
                // Preserve hwy::kMaxVectorSize's architecture-dependent padding.
                new_channels[c] = Plane::create(
                    self.frame_dimensions
                        .xsize_upsampled_padded
                        .wrapping_add(X_OFFSET * 2)
                        .wrapping_add(MAX_VECTOR_SIZE * 8),
                    self.frame_dimensions
                        .ysize_upsampled_padded
                        .wrapping_add(X_OFFSET * 2),
                )?;
                new_channels[c].shrink_to(
                    (input_sizes[c].0 << settings.shift_x).wrapping_add(X_OFFSET * 2),
                    (input_sizes[c].1 << settings.shift_y).wrapping_add(X_OFFSET * 2),
                )?;
                output_channels[c] = true;
            }
            for c in 0..self.channel_data.len() {
                if stage.get_channel_mode(c) != Mode::InOut {
                    continue;
                }
                let ch = &mut self.channel_data[c];
                for y in 0..input_sizes[c].1 {
                    let row = ch.row_mut(X_OFFSET + y);
                    for ix in 0..settings.border_x {
                        row[X_OFFSET - ix - 1] = row
                            [X_OFFSET + mirror(-(ix as i64) - 1, input_sizes[c].0 as i64) as usize];
                    }
                    for ix in 0..settings.border_x {
                        row[X_OFFSET + ix + input_sizes[c].0] = row[X_OFFSET
                            + mirror((ix + input_sizes[c].0) as i64, input_sizes[c].0 as i64)
                                as usize];
                    }
                }
                // Copy byte ranges within the same owner, without a scratch
                // allocation or simultaneous aliased typed mutable views.
                let stride = ch.bytes_per_row();
                let start = (X_OFFSET - settings.border_x) * 4;
                let len = (input_sizes[c].0 + settings.border_x * 2) * 4;
                for y in 0..settings.border_y {
                    let src = X_OFFSET + mirror(-(y as i64) - 1, input_sizes[c].1 as i64) as usize;
                    let dst = X_OFFSET - y - 1;
                    ch.bytes_mut().unwrap().copy_within(
                        src * stride + start..src * stride + start + len,
                        dst * stride + start,
                    );
                }
                for y in 0..settings.border_y {
                    let src = X_OFFSET
                        + mirror((input_sizes[c].1 + y) as i64, input_sizes[c].1 as i64) as usize;
                    let dst = X_OFFSET + input_sizes[c].1 + y;
                    ch.bytes_mut().unwrap().copy_within(
                        src * stride + start..src * stride + start + len,
                        dst * stride + start,
                    );
                }
            }
            let mut ysize = 0usize;
            let mut xsize = 0usize;
            for (c, &(w, h)) in input_sizes.iter().enumerate() {
                if stage.get_channel_mode(c) != Mode::Ignored {
                    ysize = ysize.max(h);
                    xsize = xsize.max(w);
                }
            }
            if ysize == 0 || xsize == 0 {
                return Err(StatusCode::GenericError);
            }
            let mut input_rows: RowInfo = vec![Vec::new(); self.channel_data.len()];
            let mut output_rows: RowInfo = vec![Vec::new(); self.channel_data.len()];
            stage.set_input_sizes(&input_sizes)?;
            // SetInputSizes is a mutable virtual call. Read border_y at the
            // original checkpoint rather than retaining the earlier settings.
            let border_y = stage.settings().border_y as i32;
            for y in 0..ysize {
                for (c, rows) in input_rows.iter_mut().enumerate() {
                    if stage.get_channel_mode(c) == Mode::Ignored {
                        continue;
                    }
                    rows.clear();
                    for iy in -border_y..=border_y {
                        rows.push(Row {
                            storage: RowStorage::Input,
                            channel: c,
                            y: (y + X_OFFSET).wrapping_add_signed(iy as isize),
                            x: 0,
                        });
                    }
                }
                for c in 0..self.channel_data.len() {
                    if output_channels[c] {
                        let shift_y = stage.settings().shift_y;
                        output_rows[c].resize(
                            1usize << shift_y,
                            Row {
                                storage: RowStorage::Output,
                                channel: c,
                                y: 0,
                                x: 0,
                            },
                        );
                        for (iy, row) in output_rows[c].iter_mut().enumerate() {
                            *row = Row {
                                storage: RowStorage::Output,
                                channel: c,
                                y: (y << shift_y) + iy + X_OFFSET,
                                x: 0,
                            };
                        }
                    }
                }
                let mut buffers = RowBuffers {
                    input: &mut self.channel_data,
                    output: &mut new_channels,
                };
                stage.process_row(
                    &input_rows,
                    &output_rows,
                    &mut buffers,
                    0,
                    0,
                    xsize,
                    0,
                    y,
                    thread,
                )?;
            }
            for (c, new_channel) in new_channels.iter_mut().enumerate() {
                if stage.get_channel_mode(c) == Mode::InOut {
                    self.channel_data[c] = std::mem::take(new_channel);
                }
            }
            for c in 0..self.channel_data.len() {
                let next = (stage_id + 1).min(self.channel_shifts.len() - 1);
                let (xs, ys) = self.channel_shifts[next][c];
                self.channel_data[c].shrink_to(
                    div_ceil(self.frame_dimensions.xsize_upsampled, 1usize << xs)
                        .wrapping_add(X_OFFSET * 2),
                    div_ceil(self.frame_dimensions.ysize_upsampled, 1usize << ys)
                        .wrapping_add(X_OFFSET * 2),
                )?;
            }
            if stage.switch_to_image_dimensions() {
                let mut image_xsize = 0;
                let mut image_ysize = 0;
                let mut origin = FrameOrigin::default();
                stage.get_image_dimensions(&mut image_xsize, &mut image_ysize, &mut origin);
                self.frame_dimensions
                    .set(image_xsize, image_ysize, 0, 0, 0, false, 1);
                let old_channels = std::mem::take(&mut self.channel_data);
                self.channel_data.reserve(old_channels.len());
                for _c in 0..old_channels.len() {
                    self.channel_data.push(Plane::create(
                        X_OFFSET * 2 + image_xsize,
                        X_OFFSET * 2 + image_ysize,
                    )?);
                }
                for y in 0..image_ysize {
                    for (c, rows) in output_rows.iter_mut().enumerate() {
                        *rows = vec![Row {
                            storage: RowStorage::Input,
                            channel: c,
                            y: X_OFFSET + y,
                            x: 0,
                        }];
                    }
                    let mut buffers = RowBuffers {
                        input: &mut self.channel_data,
                        output: &mut [],
                    };
                    stage.process_padding_row(&output_rows, &mut buffers, image_xsize, 0, y);
                }
                let mut x0 = origin.x0 as isize;
                let mut y0 = origin.y0 as isize;
                let mut x0_fg = 0usize;
                let mut y0_fg = 0usize;
                if x0 < 0 {
                    xsize = xsize.wrapping_add_signed(x0);
                    x0_fg = x0_fg.wrapping_sub(x0 as usize);
                    x0 = 0;
                }
                if (x0 as usize).wrapping_add(xsize) > image_xsize {
                    xsize = image_xsize.wrapping_sub(x0 as usize);
                }
                if y0 < 0 {
                    ysize = ysize.wrapping_add_signed(y0);
                    y0_fg = y0_fg.wrapping_sub(y0 as usize);
                    y0 = 0;
                }
                if (y0 as usize).wrapping_add(ysize) > image_ysize {
                    ysize = image_ysize.wrapping_sub(y0 as usize);
                }
                let target = Rect::new(x0 as usize, y0 as usize, xsize, ysize)
                    .translate(X_OFFSET as i64, X_OFFSET as i64);
                let source = Rect::new(x0_fg, y0_fg, xsize, ysize)
                    .translate(X_OFFSET as i64, X_OFFSET as i64);
                for (c, old) in old_channels.iter().enumerate() {
                    copy_rect_to(source, old, target, &mut self.channel_data[c])?;
                }
            }
        }
        Ok(())
    }
    pub fn frame_dimensions(&self) -> &FrameDimensions {
        &self.frame_dimensions
    }
    pub fn channel_shifts(&self) -> &[Vec<(usize, usize)>] {
        &self.channel_shifts
    }
    pub fn padding(&self) -> &[Vec<(usize, usize)>] {
        &self.padding
    }
    pub fn group_completed_passes(&self) -> &[u8] {
        &self.group_completed_passes
    }
    pub fn processed_passes(&self) -> usize {
        self.processed_passes
    }
    pub fn channel_data(&self) -> &[Plane<f32>] {
        &self.channel_data
    }
}
#[derive(Default)]
pub struct RenderPipelineInput<'p, 'a> {
    pipeline: Option<&'p mut SimpleRenderPipeline<'a>>,
    group_id: usize,
    thread_id: usize,
    buffers: Vec<(usize, Rect)>,
}
impl RenderPipelineInput<'_, '_> {
    /// Source move construction/assignment leaves the old binding null while
    /// retaining its scalar IDs. Ordinary Rust moves invalidate the old name.
    pub fn move_from(other: &mut Self) -> Self {
        Self {
            pipeline: other.pipeline.take(),
            group_id: other.group_id,
            thread_id: other.thread_id,
            buffers: std::mem::take(&mut other.buffers),
        }
    }
    pub fn done(&mut self) -> Status {
        self.pipeline
            .as_deref_mut()
            .ok_or(StatusCode::GenericError)?
            .input_ready(self.group_id, self.thread_id, &self.buffers)
    }
    pub fn get_buffer(&self, c: usize) -> (&Plane<f32>, Rect) {
        let &(index, r) = &self.buffers[c];
        (&self.pipeline.as_ref().unwrap().channel_data[index], r)
    }
    pub fn get_buffer_mut(&mut self, c: usize) -> (&mut Plane<f32>, Rect) {
        let &(index, r) = &self.buffers[c];
        (
            &mut self.pipeline.as_deref_mut().unwrap().channel_data[index],
            r,
        )
    }
    pub fn buffer_count(&self) -> usize {
        self.buffers.len()
    }
    /// Disjoint mutable channel views for source kernels that fill RGB together.
    /// PrepareBuffers creates indices 0..channel_data.len() in this same order;
    /// the token keeps their pipeline/storage alive throughout the iterator.
    pub fn get_buffers_mut(&mut self) -> impl Iterator<Item = (&mut Plane<f32>, Rect)> {
        let buffers = &self.buffers;
        self.pipeline
            .as_deref_mut()
            .expect("source requires a bound input")
            .channel_data
            .iter_mut()
            .zip(buffers)
            .map(|(plane, &(_, rect))| (plane, rect))
    }
}
