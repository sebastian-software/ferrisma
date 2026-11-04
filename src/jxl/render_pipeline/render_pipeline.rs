// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
//! RenderPipeline, RenderPipeline::Builder and RenderPipelineInput from
//! render_pipeline/render_pipeline.{h,cc}.
//!
//! Language-required representation:
//! - The abstract base class becomes the `RenderPipeline` trait. Its protected
//!   members live in `RenderPipelineBase`, owned by each implementation, and its
//!   non-virtual base methods are provided trait methods. Builder::Finalize
//!   computes the same padding/shift tables and then calls the selected Init.
//! - `PrepareBuffers` returns owner-relative buffer identifiers (buffer set,
//!   channel, rect) instead of `ImageF*`; RenderPipelineInput resolves them
//!   under its exclusive pipeline borrow.
//! - JXL_CHECK_PLANE_INITIALIZED is MSan-only instrumentation and JXL_DASSERT
//!   is absent from the pinned NDEBUG reference graph; neither is translated.
use super::low_memory_render_pipeline::LowMemoryRenderPipeline;
use super::render_pipeline_stage::{RenderPipelineChannelMode as Mode, RenderPipelineStage};
use super::simple_render_pipeline::SimpleRenderPipeline;
use crate::jxl::base::rect::Rect;
use crate::jxl::base::status::{Status, StatusCode, StatusOr};
use crate::jxl::frame_dimensions::FrameDimensions;
use crate::jxl::image::Plane;

fn div_ceil(n: usize, d: usize) -> usize {
    n.wrapping_add(d - 1) / d
}

/// The protected RenderPipeline members shared by both implementations.
#[derive(Default)]
pub struct RenderPipelineBase<'a> {
    pub(crate) stages: Vec<Box<dyn RenderPipelineStage + 'a>>,
    // Shifts for every channel at the input of each stage.
    pub(crate) channel_shifts: Vec<Vec<(usize, usize)>>,
    // Amount of (cumulative) padding required by each stage and channel, in
    // either direction.
    pub(crate) padding: Vec<Vec<(usize, usize)>>,
    pub(crate) frame_dimensions: FrameDimensions,
    pub(crate) group_completed_passes: Vec<u8>,
}

/// One PrepareBuffers entry: the pipeline buffer set (implementation-defined,
/// always 0 for the simple pipeline), the channel within it, and its rect.
pub type InputBuffer = (usize, usize, Rect);

pub trait RenderPipeline<'a> {
    fn base(&self) -> &RenderPipelineBase<'a>;
    fn base_mut(&mut self) -> &mut RenderPipelineBase<'a>;
    /// Upcast used by the base GetInputBuffers to bind the returned input.
    fn as_dyn_mut(&mut self) -> &mut (dyn RenderPipeline<'a> + 'a);
    fn prepare_buffers(&mut self, group_id: usize, thread_id: usize) -> Vec<InputBuffer>;
    fn process_buffers(&mut self, group_id: usize, thread_id: usize) -> Status;
    // Note that this method may be called multiple times with different (or
    // equal) `num`.
    fn prepare_for_threads_internal(&mut self, num: usize, use_group_ids: bool) -> Status;
    // Called once frame dimensions and stages are known.
    fn init(&mut self) -> Status {
        Ok(())
    }
    fn clear_done(&mut self, _i: usize) {}
    /// Resolve a PrepareBuffers identifier.
    fn input_plane(&self, set: usize, channel: usize) -> &Plane<f32>;
    fn input_plane_mut(&mut self, set: usize, channel: usize) -> &mut Plane<f32>;
    /// All channels of one buffer set, in PrepareBuffers channel order.
    fn input_planes_mut(&mut self, set: usize) -> &mut [Plane<f32>];

    fn is_initialized(&self) -> Status {
        for stage in &self.base().stages {
            stage.is_initialized()?;
        }
        Ok(())
    }
    /// Allocates storage to run with `num` threads. If `use_group_ids` is
    /// true, storage is allocated for each group, not each thread.
    fn prepare_for_threads(&mut self, num: usize, use_group_ids: bool) -> Status {
        for stage in &mut self.base_mut().stages {
            stage.prepare_for_threads(num)?;
        }
        self.prepare_for_threads_internal(num, use_group_ids)?;
        Ok(())
    }
    /// Retrieves a buffer where input data should be stored by the callee.
    /// The group bound check keeps the existing native guard for the source's
    /// group-index precondition.
    fn get_input_buffers(
        &mut self,
        group_id: usize,
        thread_id: usize,
    ) -> RenderPipelineInput<'_, 'a> {
        assert!(group_id < self.base().group_completed_passes.len());
        let buffers = self.prepare_buffers(group_id, thread_id);
        RenderPipelineInput {
            pipeline: Some(self.as_dyn_mut()),
            group_id,
            thread_id,
            buffers,
        }
    }
    fn passes_with_all_input(&self) -> usize {
        usize::from(
            *self
                .base()
                .group_completed_passes
                .iter()
                .min()
                .expect("source requires at least one group"),
        )
    }
    fn input_ready(
        &mut self,
        group_id: usize,
        thread_id: usize,
        _buffers: &[InputBuffer],
    ) -> Status {
        let passes = &mut self.base_mut().group_completed_passes;
        if group_id >= passes.len() {
            return Err(StatusCode::GenericError);
        }
        passes[group_id] = passes[group_id].wrapping_add(1);
        self.process_buffers(group_id, thread_id)?;
        Ok(())
    }
}

/// The original Builder. `use_simple_implementation` selects
/// SimpleRenderPipeline; the default selects LowMemoryRenderPipeline.
pub struct RenderPipelineBuilder<'a> {
    stages: Vec<Box<dyn RenderPipelineStage + 'a>>,
    num_c: usize,
    use_simple_implementation: bool,
}
impl<'a> RenderPipelineBuilder<'a> {
    pub fn new(num_c: usize) -> Self {
        assert!(num_c > 0, "source channel-count precondition");
        Self {
            stages: Vec::new(),
            num_c,
            use_simple_implementation: false,
        }
    }
    /// Adds a stage to the pipeline. Must be called at least once; the last
    /// added stage cannot have kInOut channels.
    pub fn add_stage(&mut self, stage: Option<Box<dyn RenderPipelineStage + 'a>>) -> Status {
        self.stages.push(stage.ok_or(StatusCode::GenericError)?);
        Ok(())
    }
    /// Enables using the simple (i.e. non-memory-efficient) implementation of
    /// the pipeline.
    pub fn use_simple_implementation(&mut self) {
        self.use_simple_implementation = true;
    }
    /// Finalizes setup of the pipeline. Shifts for all channels should be 0 at
    /// this point.
    pub fn finalize(
        self,
        frame_dimensions: FrameDimensions,
    ) -> StatusOr<Box<dyn RenderPipeline<'a> + 'a>> {
        if self.use_simple_implementation {
            Ok(self.finalize_simple(frame_dimensions)?)
        } else {
            let base = self.finalize_base(frame_dimensions)?;
            let mut res = Box::new(LowMemoryRenderPipeline::new(base));
            res.init()?;
            Ok(res)
        }
    }
    /// Finalize after UseSimpleImplementation, retaining the concrete type for
    /// callers that observe simple-pipeline storage. SimpleRenderPipeline
    /// inherits the base Init, which returns true.
    pub(crate) fn finalize_simple(
        self,
        frame_dimensions: FrameDimensions,
    ) -> StatusOr<Box<SimpleRenderPipeline<'a>>> {
        debug_assert!(self.use_simple_implementation);
        let base = self.finalize_base(frame_dimensions)?;
        let mut res = Box::new(SimpleRenderPipeline::new(base));
        RenderPipeline::init(&mut *res)?;
        Ok(res)
    }
    fn finalize_base(self, frame_dimensions: FrameDimensions) -> StatusOr<RenderPipelineBase<'a>> {
        // Check that the last stage is not a kInOut stage for any channel, and
        // that there is at least one stage.
        if self.stages.is_empty() {
            return Err(StatusCode::GenericError);
        }
        for c in 0..self.num_c {
            if self.stages.last().unwrap().get_channel_mode(c) == Mode::InOut {
                return Err(StatusCode::GenericError);
            }
        }
        let mut res = RenderPipelineBase::default();
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
        res.frame_dimensions = frame_dimensions;
        res.group_completed_passes
            .resize(frame_dimensions.num_groups, 0);
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
        Ok(res)
    }
}

/// Interface to provide input to the rendering pipeline. The binding borrows
/// its pipeline; buffers are owner-relative identifiers resolved on access.
#[derive(Default)]
pub struct RenderPipelineInput<'p, 'a> {
    pipeline: Option<&'p mut (dyn RenderPipeline<'a> + 'a)>,
    group_id: usize,
    thread_id: usize,
    buffers: Vec<InputBuffer>,
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
            .input_ready(self.group_id, self.thread_id, &self.buffers)?;
        Ok(())
    }
    pub fn get_buffer(&self, c: usize) -> (&Plane<f32>, Rect) {
        let &(set, channel, r) = &self.buffers[c];
        (
            self.pipeline.as_deref().unwrap().input_plane(set, channel),
            r,
        )
    }
    pub fn get_buffer_mut(&mut self, c: usize) -> (&mut Plane<f32>, Rect) {
        let &(set, channel, r) = &self.buffers[c];
        (
            self.pipeline
                .as_deref_mut()
                .unwrap()
                .input_plane_mut(set, channel),
            r,
        )
    }
    pub fn buffer_count(&self) -> usize {
        self.buffers.len()
    }
    /// Disjoint mutable channel views for source kernels that fill RGB together.
    /// Both implementations' PrepareBuffers create channels 0..n of one buffer
    /// set in this same order; the token keeps their storage alive throughout.
    pub fn get_buffers_mut(&mut self) -> impl Iterator<Item = (&mut Plane<f32>, Rect)> {
        let buffers = &self.buffers;
        let set = buffers.first().map_or(0, |b| b.0);
        self.pipeline
            .as_deref_mut()
            .expect("source requires a bound input")
            .input_planes_mut(set)
            .iter_mut()
            .zip(buffers)
            .map(|(plane, &(_, _, rect))| (plane, rect))
    }
}
