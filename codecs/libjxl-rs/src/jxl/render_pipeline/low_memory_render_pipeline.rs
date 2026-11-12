// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
//! Complete LowMemoryRenderPipeline from
//! render_pipeline/low_memory_render_pipeline.{h,cc}, including its private
//! Rows helper, GetMirroredY and ApplyXMirroring.
//!
//! Language-required representation:
//! - Row pointers become `Row` descriptors. While RenderRect runs, the group
//!   input planes and this thread's ring buffers are moved (headers only, no
//!   pixel copy) into one ordered slice, so the stages' existing RowBuffers view
//!   resolves every input and output row of a call. The planes return to their
//!   owners before RenderRect returns, on success and on error. Each descriptor
//!   keeps the source pointer's absolute row and its signed column offset as a
//!   wrapping usize; only the final, in-row index is dereferenced.
//! - RenderPadding views the per-thread out-of-frame image directly, one row
//!   per channel, as in the source.
//! - JXL_DASSERTs and the JXL_IS_DEBUG_BUILD ring-slot tracking are absent from
//!   the pinned NDEBUG reference graph and are not translated; neither is the
//!   MSan-only image poisoning. JXL_DEBUG_V is disabled at level 0.
//! - Null-pool execution is sequential; thread ids select per-thread storage.
use super::render_pipeline::{InputBuffer, RenderPipeline, RenderPipelineBase};
use super::render_pipeline_stage::{
    RENDER_PIPELINE_X_OFFSET as X_OFFSET, RenderPipelineChannelMode as Mode, Row, RowBuffers,
    RowInfo, RowStorage,
};
use crate::jxl::base::bits::ceil_log2_nonzero;
use crate::jxl::base::rect::{Rect, SignedRect};
use crate::jxl::base::status::{Status, StatusCode, StatusOr};
use crate::jxl::dec_group_border::GroupBorderAssigner;
use crate::jxl::frame_header::FrameOrigin;
use crate::jxl::image::Plane;
use crate::jxl::image_ops::{copy_rect_to, mirror};

type ImageF = Plane<f32>;

fn div_ceil(a: usize, b: usize) -> usize {
    a.wrapping_add(b).wrapping_sub(1) / b
}
fn round_up_to(what: usize, align: usize) -> usize {
    div_ceil(what, align).wrapping_mul(align)
}
fn same_size(r: Rect, image: &ImageF) -> bool {
    r.xsize() == image.xsize() && r.ysize() == image.ysize()
}

// Ensure that the X border on color channels is a multiple of kBlockDim or
// the vector size (required for EPF stages). Vectors on ARM NEON are never
// wider than 4 floats, so rounding to multiples of 4 is enough.
#[cfg(any(target_arch = "arm", target_arch = "aarch64"))]
const GROUP_X_ALIGN: usize = 4;
#[cfg(not(any(target_arch = "arm", target_arch = "aarch64")))]
const GROUP_X_ALIGN: usize = 16;

/// A multithreaded, low-memory rendering pipeline that only allocates a
/// minimal amount of buffers.
pub struct LowMemoryRenderPipeline<'a> {
    base: RenderPipelineBase<'a>,
    use_group_ids: bool,
    // Storage for borders between groups. Borders of adjacent groups are
    // stacked together, e.g. bottom border of current group is followed by top
    // border of next group.
    borders_horizontal: Vec<ImageF>,
    borders_vertical: Vec<ImageF>,
    // Manages the status of borders.
    group_border_assigner: GroupBorderAssigner,
    // Size (in color-channel-pixels) of the border around each group that
    // might be assigned to that group.
    group_border: (usize, usize),
    // base_color_shift defines the size of groups in terms of final image
    // pixels.
    base_color_shift: usize,
    // Buffer for decoded pixel data for a group, indexed by [thread][channel]
    // or [group][channel] depending on `use_group_ids`.
    group_data: Vec<Vec<ImageF>>,
    // Borders for storing group data.
    group_data_x_border: usize,
    group_data_y_border: usize,
    // Buffers for intermediate rows for the various stages, indexed by
    // [thread][channel][stage].
    stage_data: Vec<Vec<Vec<ImageF>>>,
    // Buffers for out-of-frame data, indexed by [thread]; every row is a
    // different channel.
    out_of_frame_data: Vec<ImageF>,
    // For each stage, a non-kIgnored channel.
    anyc: Vec<i32>,
    // Size of the image at each stage.
    image_rect: Vec<Rect>,
    // For each stage, for each channel, keep track of the kInOut stage that
    // produced the input to that stage (which corresponds to the buffer index
    // containing the data). -1 if data comes from the original input.
    stage_input_for_channel: Vec<Vec<i32>>,
    // Number of (virtual) extra rows that must be processed at each stage to
    // produce sufficient output for future stages.
    virtual_ypadding_for_output: Vec<i32>,
    // Same thing for columns, except these are real columns and not virtual
    // ones.
    xpadding_for_output: Vec<i32>,
    // First stage that doesn't have any kInOut channel.
    first_trailing_stage: usize,
    // Origin and size of the frame after switching to image dimensions.
    frame_origin: FrameOrigin,
    full_image_xsize: usize,
    full_image_ysize: usize,
    first_image_dim_stage: usize,
}

impl Drop for LowMemoryRenderPipeline<'_> {
    /// The implicit destructors: the derived buffers in reverse member order,
    /// each vector last first, then the base's stages.
    fn drop(&mut self) {
        use crate::jxl::base::libcpp_vector::{clear, clear_nested};
        clear(&mut self.out_of_frame_data);
        while let Some(mut per_thread) = self.stage_data.pop() {
            clear_nested(&mut per_thread);
        }
        clear_nested(&mut self.group_data);
        clear(&mut self.borders_vertical);
        clear(&mut self.borders_horizontal);
        clear(&mut self.base.stages);
    }
}
impl<'a> LowMemoryRenderPipeline<'a> {
    /// Members without a source initializer are assigned by Init or
    /// PrepareForThreadsInternal before their first read.
    pub(crate) fn new(base: RenderPipelineBase<'a>) -> Self {
        Self {
            base,
            use_group_ids: false,
            borders_horizontal: Vec::new(),
            borders_vertical: Vec::new(),
            group_border_assigner: GroupBorderAssigner::default(),
            group_border: (0, 0),
            base_color_shift: 0,
            group_data: Vec::new(),
            group_data_x_border: 0,
            group_data_y_border: 0,
            stage_data: Vec::new(),
            out_of_frame_data: Vec::new(),
            anyc: Vec::new(),
            image_rect: Vec::new(),
            stage_input_for_channel: Vec::new(),
            virtual_ypadding_for_output: Vec::new(),
            xpadding_for_output: Vec::new(),
            first_trailing_stage: 0,
            frame_origin: FrameOrigin { x0: 0, y0: 0 },
            full_image_xsize: 0,
            full_image_ysize: 0,
            first_image_dim_stage: 0,
        }
    }

    fn color_dimensions_to_channel_dimensions(
        &self,
        input: (usize, usize),
        c: usize,
        stage: usize,
    ) -> (usize, usize) {
        let shift = self.base.channel_shifts[stage][c];
        (
            (input.0 << self.base_color_shift)
                .wrapping_add(1usize << shift.0)
                .wrapping_sub(1)
                >> shift.0,
            (input.1 << self.base_color_shift)
                .wrapping_add(1usize << shift.1)
                .wrapping_sub(1)
                >> shift.1,
        )
    }

    fn border_to_store(&self, c: usize) -> (usize, usize) {
        let mut ret = self.color_dimensions_to_channel_dimensions(self.group_border, c, 0);
        ret.0 = ret.0.wrapping_add(self.base.padding[0][c].0);
        ret.1 = ret.1.wrapping_add(self.base.padding[0][c].1);
        ret
    }

    /// `input_set` selects `group_data[input_set]`, the source's `in`.
    fn save_borders(&mut self, group_id: usize, c: usize, input_set: usize) -> Status {
        let fd = &self.base.frame_dimensions;
        let gy = group_id / fd.xsize_groups;
        let gx = group_id % fd.xsize_groups;
        let hshift = self.base.channel_shifts[0][c].0;
        let vshift = self.base.channel_shifts[0][c].1;
        let x0 = gx.wrapping_mul(self.group_input_x_size(c));
        let x1 = (gx + 1)
            .wrapping_mul(self.group_input_x_size(c))
            .min(div_ceil(fd.xsize_upsampled, 1 << hshift));
        let y0 = gy.wrapping_mul(self.group_input_y_size(c));
        let y1 = (gy + 1)
            .wrapping_mul(self.group_input_y_size(c))
            .min(div_ceil(fd.ysize_upsampled, 1 << vshift));
        let xsize_groups = fd.xsize_groups;
        let ysize_groups = fd.ysize_groups;

        let borders = self.border_to_store(c);
        let borderx_write = borders.0;
        let bordery_write = borders.1;
        let gdxb = self.group_data_x_border;
        let gdyb = self.group_data_y_border;
        let input = &self.group_data[input_set][c];

        if gy > 0 {
            let from = Rect::new(gdxb, gdyb, x1.wrapping_sub(x0), bordery_write);
            let to = Rect::new(
                x0,
                (gy * 2 - 1).wrapping_mul(bordery_write),
                x1.wrapping_sub(x0),
                bordery_write,
            );
            copy_rect_to(from, input, to, &mut self.borders_horizontal[c])?;
        }
        if gy + 1 < ysize_groups {
            let from = Rect::new(
                gdxb,
                gdyb.wrapping_add(y1)
                    .wrapping_sub(y0)
                    .wrapping_sub(bordery_write),
                x1.wrapping_sub(x0),
                bordery_write,
            );
            let to = Rect::new(
                x0,
                (gy * 2).wrapping_mul(bordery_write),
                x1.wrapping_sub(x0),
                bordery_write,
            );
            copy_rect_to(from, input, to, &mut self.borders_horizontal[c])?;
        }
        if gx > 0 {
            let from = Rect::new(gdxb, gdyb, borderx_write, y1.wrapping_sub(y0));
            let to = Rect::new(
                (gx * 2 - 1).wrapping_mul(borderx_write),
                y0,
                borderx_write,
                y1.wrapping_sub(y0),
            );
            copy_rect_to(from, input, to, &mut self.borders_vertical[c])?;
        }
        if gx + 1 < xsize_groups {
            let from = Rect::new(
                gdxb.wrapping_add(x1)
                    .wrapping_sub(x0)
                    .wrapping_sub(borderx_write),
                gdyb,
                borderx_write,
                y1.wrapping_sub(y0),
            );
            let to = Rect::new(
                (gx * 2).wrapping_mul(borderx_write),
                y0,
                borderx_write,
                y1.wrapping_sub(y0),
            );
            copy_rect_to(from, input, to, &mut self.borders_vertical[c])?;
        }
        Ok(())
    }

    /// `output_set` selects `group_data[output_set]`, the source's `out`.
    fn load_borders(&mut self, group_id: usize, c: usize, r: Rect, output_set: usize) -> Status {
        let fd = &self.base.frame_dimensions;
        let gy = group_id / fd.xsize_groups;
        let gx = group_id % fd.xsize_groups;
        let hshift = self.base.channel_shifts[0][c].0;
        let vshift = self.base.channel_shifts[0][c].1;
        // Coordinates of the group in the image.
        let x0 = gx.wrapping_mul(self.group_input_x_size(c));
        let x1 = (gx + 1)
            .wrapping_mul(self.group_input_x_size(c))
            .min(div_ceil(fd.xsize_upsampled, 1 << hshift));
        let y0 = gy.wrapping_mul(self.group_input_y_size(c));
        let y1 = (gy + 1)
            .wrapping_mul(self.group_input_y_size(c))
            .min(div_ceil(fd.ysize_upsampled, 1 << vshift));
        let xsize_groups = fd.xsize_groups;
        let ysize_groups = fd.ysize_groups;
        let xsize_upsampled = fd.xsize_upsampled;
        let ysize_upsampled = fd.ysize_upsampled;

        let paddingx = self.base.padding[0][c].0;
        let paddingy = self.base.padding[0][c].1;

        let borders = self.border_to_store(c);
        let borderx_write = borders.0;
        let bordery_write = borders.1;
        let gdxb = self.group_data_x_border;
        let gdyb = self.group_data_y_border;
        let bcs = self.base_color_shift;

        // Limits of the area to copy from, in image coordinates.
        if !(r.x0() == 0 || (r.x0() << bcs) >= paddingx) {
            return Err(StatusCode::GenericError);
        }
        let mut x0src = div_ceil(r.x0() << bcs, 1 << hshift);
        if x0src != 0 {
            x0src = x0src.wrapping_sub(paddingx);
        }
        // r may be such that r.x1 (namely x0() + xsize()) is within paddingx of
        // the right side of the image, so we use min() here.
        let mut x1src = div_ceil(r.x0().wrapping_add(r.xsize()) << bcs, 1 << hshift);
        x1src = x1src
            .wrapping_add(paddingx)
            .min(div_ceil(xsize_upsampled, 1 << hshift));

        // Similar computation for y.
        if !(r.y0() == 0 || (r.y0() << bcs) >= paddingy) {
            return Err(StatusCode::GenericError);
        }
        let mut y0src = div_ceil(r.y0() << bcs, 1 << vshift);
        if y0src != 0 {
            y0src = y0src.wrapping_sub(paddingy);
        }
        let mut y1src = div_ceil(r.y0().wrapping_add(r.ysize()) << bcs, 1 << vshift);
        y1src = y1src
            .wrapping_add(paddingy)
            .min(div_ceil(ysize_upsampled, 1 << vshift));

        let out = &mut self.group_data[output_set][c];
        // Copy other groups' borders from the border storage.
        if y0src < y0 {
            if gy == 0 {
                return Err(StatusCode::GenericError);
            }
            copy_rect_to(
                Rect::new(
                    x0src,
                    (gy * 2 - 2).wrapping_mul(bordery_write),
                    x1src.wrapping_sub(x0src),
                    bordery_write,
                ),
                &self.borders_horizontal[c],
                Rect::new(
                    gdxb.wrapping_add(x0src).wrapping_sub(x0),
                    gdyb.wrapping_sub(bordery_write),
                    x1src.wrapping_sub(x0src),
                    bordery_write,
                ),
                out,
            )?;
        }
        if y1src > y1 {
            // When copying the bottom border we must not be on the bottom groups.
            if gy + 1 >= ysize_groups {
                return Err(StatusCode::GenericError);
            }
            copy_rect_to(
                Rect::new(
                    x0src,
                    (gy * 2 + 1).wrapping_mul(bordery_write),
                    x1src.wrapping_sub(x0src),
                    bordery_write,
                ),
                &self.borders_horizontal[c],
                Rect::new(
                    gdxb.wrapping_add(x0src).wrapping_sub(x0),
                    gdyb.wrapping_add(y1).wrapping_sub(y0),
                    x1src.wrapping_sub(x0src),
                    bordery_write,
                ),
                out,
            )?;
        }
        if x0src < x0 {
            if gx == 0 {
                return Err(StatusCode::GenericError);
            }
            copy_rect_to(
                Rect::new(
                    (gx * 2 - 2).wrapping_mul(borderx_write),
                    y0src,
                    borderx_write,
                    y1src.wrapping_sub(y0src),
                ),
                &self.borders_vertical[c],
                Rect::new(
                    gdxb.wrapping_sub(borderx_write),
                    gdyb.wrapping_add(y0src).wrapping_sub(y0),
                    borderx_write,
                    y1src.wrapping_sub(y0src),
                ),
                out,
            )?;
        }
        if x1src > x1 {
            // When copying the right border we must not be on the rightmost
            // groups.
            if gx + 1 >= xsize_groups {
                return Err(StatusCode::GenericError);
            }
            copy_rect_to(
                Rect::new(
                    (gx * 2 + 1).wrapping_mul(borderx_write),
                    y0src,
                    borderx_write,
                    y1src.wrapping_sub(y0src),
                ),
                &self.borders_vertical[c],
                Rect::new(
                    gdxb.wrapping_add(x1).wrapping_sub(x0),
                    gdyb.wrapping_add(y0src).wrapping_sub(y0),
                    borderx_write,
                    y1src.wrapping_sub(y0src),
                ),
                out,
            )?;
        }
        Ok(())
    }

    fn group_input_x_size(&self, c: usize) -> usize {
        (self.base.frame_dimensions.group_dim << self.base_color_shift)
            >> self.base.channel_shifts[0][c].0
    }

    fn group_input_y_size(&self, c: usize) -> usize {
        (self.base.frame_dimensions.group_dim << self.base_color_shift)
            >> self.base.channel_shifts[0][c].1
    }

    fn ensure_borders_storage(&mut self) -> Status {
        let num_shifts = self.base.channel_shifts[0].len();
        if self.borders_horizontal.len() < num_shifts {
            self.borders_horizontal
                .resize_with(num_shifts, ImageF::default);
            self.borders_vertical
                .resize_with(num_shifts, ImageF::default);
        }
        for c in 0..num_shifts {
            let shifts = self.base.channel_shifts[0][c];
            let borders = self.border_to_store(c);
            let borderx = borders.0;
            let bordery = borders.1;
            let fd = &self.base.frame_dimensions;
            if fd.xsize_groups == 0 {
                return Err(StatusCode::GenericError);
            }
            let num_xborders = (fd.xsize_groups - 1).wrapping_mul(2);
            if fd.ysize_groups == 0 {
                return Err(StatusCode::GenericError);
            }
            let num_yborders = (fd.ysize_groups - 1).wrapping_mul(2);
            let downsampled_xsize = div_ceil(fd.xsize_upsampled_padded, 1 << shifts.0);
            let downsampled_ysize = div_ceil(fd.ysize_upsampled_padded, 1 << shifts.1);
            let horizontal = Rect::new(0, 0, downsampled_xsize, bordery.wrapping_mul(num_yborders));
            if !same_size(horizontal, &self.borders_horizontal[c]) {
                self.borders_horizontal[c] = ImageF::create_in(
                    horizontal.xsize(),
                    horizontal.ysize(),
                    0,
                    self.base.memory_manager,
                )?;
            }
            let vertical = Rect::new(0, 0, borderx.wrapping_mul(num_xborders), downsampled_ysize);
            if !same_size(vertical, &self.borders_vertical[c]) {
                self.borders_vertical[c] = ImageF::create_in(
                    vertical.xsize(),
                    vertical.ysize(),
                    0,
                    self.base.memory_manager,
                )?;
            }
        }
        Ok(())
    }

    fn init_impl(&mut self) -> Status {
        self.group_border = (0, 0);
        self.base_color_shift = ceil_log2_nonzero(
            self.base.frame_dimensions.xsize_upsampled_padded
                / self.base.frame_dimensions.xsize_padded,
        );

        let num_shifts = self.base.channel_shifts[0].len();
        let bcs = self.base_color_shift;

        // Ensure that each channel has enough many border pixels.
        for c in 0..num_shifts {
            let shift = self.base.channel_shifts[0][c];
            let padding = self.base.padding[0][c];
            self.group_border.0 = self
                .group_border
                .0
                .max(div_ceil(padding.0 << shift.0, 1 << bcs));
            self.group_border.1 = self
                .group_border
                .1
                .max(div_ceil(padding.1 << shift.1, 1 << bcs));
        }

        // Ensure that all channels have an integer number of border pixels in
        // the input.
        for c in 0..num_shifts {
            let shift = self.base.channel_shifts[0][c];
            if shift.0 >= bcs {
                self.group_border.0 = round_up_to(self.group_border.0, 1 << (shift.0 - bcs));
            }
            if shift.1 >= bcs {
                self.group_border.1 = round_up_to(self.group_border.1, 1 << (shift.1 - bcs));
            }
        }
        self.group_border.0 = round_up_to(self.group_border.0, GROUP_X_ALIGN);
        // Allocate borders in group images that are just enough for storing the
        // borders to be copied in, plus any rounding to ensure alignment.
        let mut max_border = (0usize, 0usize);
        for c in 0..num_shifts {
            max_border.0 = self.border_to_store(c).0.max(max_border.0);
            max_border.1 = self.border_to_store(c).1.max(max_border.1);
        }
        self.group_data_x_border = round_up_to(max_border.0, GROUP_X_ALIGN);
        self.group_data_y_border = max_border.1;

        self.ensure_borders_storage()?;
        self.group_border_assigner.init(&self.base.frame_dimensions);

        let num_stages = self.base.stages.len();
        self.first_trailing_stage = num_stages;
        while self.first_trailing_stage > 0 {
            let mut has_inout_c = false;
            for c in 0..num_shifts {
                if self.base.stages[self.first_trailing_stage - 1].get_channel_mode(c)
                    == Mode::InOut
                {
                    has_inout_c = true;
                }
            }
            if has_inout_c {
                break;
            }
            self.first_trailing_stage -= 1;
        }

        self.first_image_dim_stage = num_stages;
        for i in 0..num_stages {
            let mut input_sizes = vec![(0usize, 0usize); num_shifts];
            for (c, size) in input_sizes.iter_mut().enumerate() {
                let shift = self.base.channel_shifts[i][c];
                *size = (
                    div_ceil(self.base.frame_dimensions.xsize_upsampled, 1 << shift.0),
                    div_ceil(self.base.frame_dimensions.ysize_upsampled, 1 << shift.1),
                );
            }
            self.base.stages[i].set_input_sizes(&input_sizes)?;
            if self.base.stages[i].switch_to_image_dimensions() {
                // We don't allow kInOut after switching to image dimensions.
                if i < self.first_trailing_stage {
                    return Err(StatusCode::GenericError);
                }
                self.first_image_dim_stage = i + 1;
                self.base.stages[i].get_image_dimensions(
                    &mut self.full_image_xsize,
                    &mut self.full_image_ysize,
                    &mut self.frame_origin,
                );
                break;
            }
        }
        for i in self.first_image_dim_stage..num_stages {
            if self.base.stages[i].switch_to_image_dimensions() {
                // JXL_UNREACHABLE is JXL_FAILURE in the NDEBUG reference.
                return Err(StatusCode::GenericError);
            }
            let input_sizes = vec![(self.full_image_xsize, self.full_image_ysize); num_shifts];
            self.base.stages[i].set_input_sizes(&input_sizes)?;
        }

        self.anyc.resize(num_stages, 0);
        for i in 0..num_stages {
            for c in 0..num_shifts {
                if self.base.stages[i].get_channel_mode(c) != Mode::Ignored {
                    self.anyc[i] = c as i32;
                }
            }
        }

        self.stage_input_for_channel = vec![vec![0i32; num_shifts]; num_stages];
        for c in 0..num_shifts {
            let mut input = -1i32;
            for i in 0..num_stages {
                self.stage_input_for_channel[i][c] = input;
                if self.base.stages[i].get_channel_mode(c) == Mode::InOut {
                    input = i as i32;
                }
            }
        }

        self.image_rect.resize(num_stages, Rect::default());
        for i in 0..num_stages {
            let shift = self.base.channel_shifts[i][self.anyc[i] as usize];
            let x1 = div_ceil(self.base.frame_dimensions.xsize_upsampled, 1 << shift.0);
            let y1 = div_ceil(self.base.frame_dimensions.ysize_upsampled, 1 << shift.1);
            self.image_rect[i] = Rect::new(0, 0, x1, y1);
        }

        self.virtual_ypadding_for_output.resize(num_stages, 0);
        // How many "xextra" pixels to be processed by the stage to have enough
        // "xextra" plus "border_x" pixels initialized for the next stage(s).
        self.xpadding_for_output.resize(num_stages, 0);
        for c in 0..num_shifts {
            let mut ypad: i32 = 0;
            let mut xpad: i32 = 0;
            for i in (0..num_stages).rev() {
                let y_sampling = 1usize << self.base.channel_shifts[i][c].1;
                let aligned_y_pad = round_up_to(ypad as usize, y_sampling);
                let mode = self.base.stages[i].get_channel_mode(c);
                if mode != Mode::Ignored {
                    self.virtual_ypadding_for_output[i] =
                        self.virtual_ypadding_for_output[i].max(aligned_y_pad as i32);
                    self.xpadding_for_output[i] = xpad.max(self.xpadding_for_output[i]);
                }
                if mode == Mode::InOut {
                    let settings = self.base.stages[i].settings();
                    ypad = aligned_y_pad.wrapping_add(settings.border_y.wrapping_mul(y_sampling))
                        as i32;
                    let d = 1i32 << settings.shift_x;
                    xpad = (((xpad + d - 1) / d) as usize).wrapping_add(settings.border_x) as i32;
                }
            }
        }
        Ok(())
    }

    /// Number of rows of output of `stage` for channel `c` that must be kept
    /// alive simultaneously, i.e. the size of the ring buffer that stores them.
    #[expect(
        clippy::needless_range_loop,
        reason = "Retain the source's indexed loops over parallel stage/channel tables."
    )]
    fn stage_buffer_y_size(&self, c: usize, stage: usize) -> usize {
        // RenderRect walks "virtual" rows (rows of the final image) and runs
        // each stage `virtual_ypadding_for_output` virtual rows ahead of the end
        // of the image, so that the stages consuming its output find all the
        // rows they need (including their vertical border) already produced.
        // That padding is rounded up to the row sampling factor of each stage
        // (see Init), which can put a producer further ahead of its consumers
        // than `border_y` rows. The ring buffer holding the output of `stage`
        // must therefore hold every row from the oldest one a consumer still
        // reads to the newest one the producer has written. Instead of deriving
        // a closed form for that distance, replay the schedule of RenderRect
        // for enough virtual rows to cover the start-up transient plus several
        // periods of all sampling factors, and measure it.
        let stages = &self.base.stages;
        let input_shift =
            |i: usize| -> usize { self.base.channel_shifts[i][self.anyc[i] as usize].1 };
        let bundle: isize = 1isize << stages[stage].settings().shift_y;

        // The consumers of this output are the stages after `stage` that do not
        // ignore channel `c`, up to and including the next kInOut stage. If
        // there is no such stage, the trailing stages read the rows one by one.
        let mut consumers = Vec::new();
        let mut read_by_trailing_stages = true;
        for k in stage + 1..self.first_trailing_stage {
            let mode = stages[k].get_channel_mode(c);
            if mode == Mode::Ignored {
                continue;
            }
            consumers.push(k);
            if mode == Mode::InOut {
                read_by_trailing_stages = false;
                break;
            }
        }

        let mut max_shift: i32 = 0;
        let mut max_vpad: i32 = 0;
        let mut max_border: i32 = 0;
        for k in 0..self.first_trailing_stage {
            max_shift = max_shift.max(input_shift(k) as i32);
            max_vpad = max_vpad.max(self.virtual_ypadding_for_output[k]);
            max_border = max_border.max(stages[k].settings().border_y as i32);
        }
        let num_virtual_rows: i32 = 2 * max_vpad + (4 + 2 * max_border) * (1 << max_shift);

        // Same conditions as in RenderRect, with `vy` relative to the first row.
        let runs_at = |k: usize, vy: i32, y: &mut i32| -> bool {
            let stage_vy = vy + self.virtual_ypadding_for_output[k];
            if stage_vy % (1i32 << input_shift(k)) != 0 {
                return false;
            }
            *y = stage_vy >> input_shift(k);
            true
        };

        let mut newest_produced: isize = -1;
        let mut required: isize = 1;
        for vy in 0..num_virtual_rows {
            let mut y = 0i32;
            if runs_at(stage, vy, &mut y) {
                newest_produced = newest_produced.max(y as isize * bundle + bundle - 1);
            }
            for &k in &consumers {
                if !runs_at(k, vy, &mut y) {
                    continue;
                }
                let oldest_needed: isize =
                    0isize.max(y as isize - stages[k].settings().border_y as isize);
                required = required.max(newest_produced - oldest_needed + 1);
            }
            if read_by_trailing_stages {
                required = required.max(newest_produced - vy as isize + 1);
            }
        }
        1usize << ceil_log2_nonzero(required as usize)
    }

    #[allow(
        clippy::needless_range_loop,
        reason = "Retain the source's indexed loops over parallel stage/channel tables."
    )]
    fn prepare_for_threads_impl(&mut self, num: usize, use_group_ids: bool) -> Status {
        let num_shifts = self.base.channel_shifts[0].len();
        let num_stages = self.base.stages.len();
        self.use_group_ids = use_group_ids;
        let num_buffers = if self.use_group_ids {
            self.base.frame_dimensions.num_groups
        } else {
            num
        };
        for t in self.group_data.len()..num_buffers {
            self.group_data.push(Vec::new());
            self.group_data[t].resize_with(num_shifts, ImageF::default);
            for c in 0..num_shifts {
                self.group_data[t][c] = ImageF::create_in(
                    self.group_input_x_size(c)
                        .wrapping_add(self.group_data_x_border * 2),
                    self.group_input_y_size(c)
                        .wrapping_add(self.group_data_y_border * 2),
                    X_OFFSET,
                    self.base.memory_manager,
                )?;
            }
        }
        // TODO(veluca): avoid reallocating buffers if not needed.
        self.stage_data.resize_with(num, Vec::new);
        let upsampling = 1usize << self.base_color_shift;
        let group_dim = self.base.frame_dimensions.group_dim * upsampling;
        let padding = 2 * self.group_data_x_border * upsampling // maximum size of a rect
            + 2 * X_OFFSET; // extra padding for processing
        let stage_buffer_xsize = group_dim + padding;
        let mut stage_buffer_ysize = vec![vec![0usize; num_stages]; num_shifts];
        for (c, sizes) in stage_buffer_ysize.iter_mut().enumerate() {
            for i in (0..num_stages).rev() {
                if self.base.stages[i].get_channel_mode(c) != Mode::InOut {
                    continue;
                }
                sizes[i] = self.stage_buffer_y_size(c, i);
                // The legacy-size JXL_DASSERT is absent from the NDEBUG graph.
            }
        }
        for t in 0..num {
            self.stage_data[t].resize_with(num_shifts, Vec::new);
            for c in 0..num_shifts {
                self.stage_data[t][c].resize_with(num_stages, ImageF::default);
                for i in (0..num_stages).rev() {
                    if self.base.stages[i].get_channel_mode(c) == Mode::InOut {
                        self.stage_data[t][c][i] = ImageF::create_in(
                            stage_buffer_xsize,
                            stage_buffer_ysize[c][i],
                            0,
                            self.base.memory_manager,
                        )?;
                    }
                }
            }
        }
        if self.first_image_dim_stage != num_stages {
            let fd = &self.base.frame_dimensions;
            let mut image_rect = SignedRect::new(0, 0, fd.xsize_upsampled, fd.ysize_upsampled);
            let full_image_rect =
                SignedRect::new(0, 0, self.full_image_xsize, self.full_image_ysize);
            image_rect = image_rect.translate(
                i64::from(self.frame_origin.x0),
                i64::from(self.frame_origin.y0),
            );
            image_rect = image_rect.intersection(full_image_rect);
            if image_rect.xsize() == 0 || image_rect.ysize() == 0 {
                image_rect = SignedRect::new(0, 0, 0, 0);
            }
            let left_padding = image_rect.x0() as usize;
            let middle_padding = group_dim;
            let right_padding = self.full_image_xsize.wrapping_sub(image_rect.x1() as usize);
            let out_of_frame_xsize =
                padding.wrapping_add(left_padding.max(middle_padding.max(right_padding)));
            self.out_of_frame_data.resize_with(num, ImageF::default);
            for t in 0..num {
                self.out_of_frame_data[t] =
                    ImageF::create_in(out_of_frame_xsize, num_shifts, 0, self.base.memory_manager)?;
            }
        }
        Ok(())
    }

    fn prepare_buffers_impl(&mut self, group_id: usize, thread_id: usize) -> Vec<InputBuffer> {
        let fd = &self.base.frame_dimensions;
        let num_shifts = self.base.channel_shifts[0].len();
        let gx = group_id % fd.xsize_groups;
        let gy = group_id / fd.xsize_groups;
        let set = if self.use_group_ids {
            group_id
        } else {
            thread_id
        };
        (0..num_shifts)
            .map(|c| {
                let shift = self.base.channel_shifts[0][c];
                let rect = Rect::new_clamped(
                    self.group_data_x_border,
                    self.group_data_y_border,
                    self.group_input_x_size(c),
                    self.group_input_y_size(c),
                    div_ceil(fd.xsize_upsampled, 1 << shift.0)
                        .wrapping_sub(gx.wrapping_mul(self.group_input_x_size(c)))
                        .wrapping_add(self.group_data_x_border),
                    div_ceil(fd.ysize_upsampled, 1 << shift.1)
                        .wrapping_sub(gy.wrapping_mul(self.group_input_y_size(c)))
                        .wrapping_add(self.group_data_y_border),
                );
                (set, c, rect)
            })
            .collect()
    }

    /// RenderRect for `group_data[input_set]`. The input planes and this
    /// thread's stage buffers are lent to one row-resolution slice for the
    /// duration of the call and always returned before propagating its status.
    #[allow(
        clippy::needless_range_loop,
        reason = "Retain the source's indexed loops over parallel stage/channel tables."
    )]
    fn render_rect(
        &mut self,
        thread_id: usize,
        input_set: usize,
        data_max_color_channel_rect: Rect,
        image_max_color_channel_rect: Rect,
    ) -> Status {
        let num_stages = self.base.stages.len();
        // For each stage, the rect corresponding to the image area currently
        // being processed, in the coordinates of that stage (i.e. with the
        // scaling factor that that stage has).
        let mut group_rect = vec![Rect::default(); num_stages];
        let image_area_rect = image_max_color_channel_rect
            .shift_left(self.base_color_shift, self.base_color_shift)
            .crop(
                self.base.frame_dimensions.xsize_upsampled,
                self.base.frame_dimensions.ysize_upsampled,
            );
        for i in 0..num_stages {
            group_rect[i] = image_area_rect
                .ceil_shift_right(self.base.channel_shifts[i][self.anyc[i] as usize])?;
        }

        // Compute actual x-axis bounds for the current image area in the
        // context of the full image this frame is part of. As the left boundary
        // may be negative, we also create the x_pixels_skip value, defined as
        // follows:
        // - both x_pixels_skip and full_image_x0 are >= 0, and at least one is 0;
        // - full_image_x0 - x_pixels_skip is the position of the current frame
        //   area in the full image.
        let mut full_image_x0: isize =
            (self.frame_origin.x0 as isize).wrapping_add(image_area_rect.x0() as isize);
        let mut x_pixels_skip: isize = 0;
        if full_image_x0 < 0 {
            x_pixels_skip = -full_image_x0;
            full_image_x0 = 0;
        }
        let full_image_x1: isize =
            (self.frame_origin.x0 as isize).wrapping_add(image_area_rect.x1() as isize);

        let mut span = vec![Rect::default(); num_stages];
        for i in 0..num_stages {
            if i < self.first_image_dim_stage {
                span[i] = Rect::new(
                    group_rect[i].x0(),
                    0,
                    group_rect[i].xsize(),
                    self.image_rect[i].ysize(),
                );
            } else {
                let x0 = full_image_x0 as usize;
                let x1 = full_image_x1 as usize;
                let x_max = self.full_image_xsize;
                let cropped_x1 = (x1 as isize).min(x_max as isize) as usize;
                span[i] = Rect::new(
                    x0,
                    0,
                    0isize.max(cropped_x1.wrapping_sub(x0) as isize) as usize,
                    self.full_image_ysize,
                );
            }
        }

        // Data structures to hold information about input/output rows and
        // their buffers (Rows::Create's checks precede any buffer use).
        let num_channels = self.group_data[input_set].len();
        {
            let thread_data = &self.stage_data[thread_id];
            if thread_data.len() != num_channels
                || self.base.channel_shifts[0].len() != num_channels
            {
                return Err(StatusCode::GenericError);
            }
            for td in thread_data {
                if td.len() != num_stages {
                    return Err(StatusCode::GenericError);
                }
            }
        }
        let mut arena: Vec<ImageF> = Vec::with_capacity(num_channels * (num_stages + 1));
        for plane in self.group_data[input_set].iter_mut() {
            arena.push(std::mem::take(plane));
        }
        for per_channel in self.stage_data[thread_id].iter_mut() {
            for plane in per_channel.iter_mut() {
                arena.push(std::mem::take(plane));
            }
        }
        let status = self.render_rect_rows(
            thread_id,
            &mut arena,
            data_max_color_channel_rect,
            image_area_rect,
            &group_rect,
            &span,
            x_pixels_skip,
        );
        let mut planes = arena.into_iter();
        for plane in self.group_data[input_set].iter_mut() {
            *plane = planes.next().unwrap();
        }
        for per_channel in self.stage_data[thread_id].iter_mut() {
            for plane in per_channel.iter_mut() {
                *plane = planes.next().unwrap();
            }
        }
        status
    }

    #[expect(
        clippy::needless_range_loop,
        reason = "Retain the source's indexed loops over parallel stage/channel tables."
    )]
    #[expect(
        clippy::too_many_arguments,
        reason = "Retain RenderRect's locals across the lent-storage boundary."
    )]
    fn render_rect_rows(
        &mut self,
        thread_id: usize,
        arena: &mut [ImageF],
        data_max_color_channel_rect: Rect,
        image_area_rect: Rect,
        group_rect: &[Rect],
        span: &[Rect],
        x_pixels_skip: isize,
    ) -> Status {
        let num_stages = self.base.stages.len();
        let num_channels = self.base.channel_shifts[0].len();
        let rows = Rows::create(
            &self.base.stages,
            data_max_color_channel_rect,
            self.group_data_x_border as i32,
            self.group_data_y_border as i32,
            &self.base.channel_shifts[0],
            self.base_color_shift,
            arena,
            num_channels,
        )?;

        let first_trailing_stage = self.first_trailing_stage;
        let mut input_rows: Vec<RowInfo> = vec![Vec::new(); first_trailing_stage + 1];
        for rows_i in input_rows.iter_mut().take(first_trailing_stage) {
            rows_i.resize(num_channels, Vec::new());
        }
        input_rows[first_trailing_stage].resize(num_channels, vec![NULL_ROW; 1]);

        // Maximum possible shift is 3.
        let mut output_rows: RowInfo = vec![vec![NULL_ROW; 8]; num_channels];

        // We pretend that every stage has a vertical shift of 0, i.e. it is as
        // tall as the final image.
        // We call each such row a "virtual" row, because it may or may not
        // correspond to an actual row of the current processing stage; actual
        // processing happens when vy % (1<<vshift) == 0.

        let num_extra_rows: i32 = *self.virtual_ypadding_for_output.iter().max().unwrap();

        for vy in -num_extra_rows..image_area_rect.ysize() as i32 + num_extra_rows {
            for i in 0..first_trailing_stage {
                let stage_vy = vy - num_extra_rows + self.virtual_ypadding_for_output[i];
                let shift_y = self.base.channel_shifts[i][self.anyc[i] as usize].1;

                if stage_vy % (1i32 << shift_y) != 0 {
                    continue;
                }

                if stage_vy < -self.virtual_ypadding_for_output[i] {
                    continue;
                }

                let y = stage_vy >> shift_y;

                let image_y = (group_rect[i].y0() as isize).wrapping_add(y as isize);
                // Do not produce rows in out-of-bounds areas.
                if image_y < 0 {
                    continue;
                }
                if image_y >= span[i].y1() as isize {
                    continue;
                }

                // Get the input/output rows and potentially apply mirroring to
                // the input.
                self.prepare_io_rows(
                    y,
                    i,
                    arena,
                    &rows,
                    group_rect,
                    &mut input_rows[i],
                    &mut output_rows,
                );

                // Produce output rows.
                if span[i].xsize() == 0 {
                    continue;
                }
                let xpos = span[i].x0();
                let xsize = span[i].xsize();
                let xdata_right = self.image_rect[i]
                    .xsize()
                    .wrapping_sub(xpos)
                    .wrapping_sub(xsize);
                let xextra_left = xpos.min(self.xpadding_for_output[i] as usize);
                let xextra_right = xdata_right.min(self.xpadding_for_output[i] as usize);
                let mut buffers = RowBuffers {
                    input: arena,
                    output: &mut [],
                };
                self.base.stages[i].process_row(
                    &input_rows[i],
                    &output_rows,
                    &mut buffers,
                    xextra_left,
                    xextra_right,
                    xsize,
                    xpos,
                    image_y as usize,
                    thread_id,
                )?;
            }

            // Process trailing stages, i.e. the final set of non-kInOut
            // stages; they all have the same input buffer and no need to use
            // any mirroring.

            let y = vy - num_extra_rows;

            // Check that we are not outside of the bounds for the current
            // rendering rect. Not doing so might result in overwriting some
            // rows that have been written (or will be written) by other
            // threads.
            if y < 0 || y as isize >= image_area_rect.ysize() as isize {
                continue;
            }

            for c in 0..num_channels {
                input_rows[first_trailing_stage][c][0] = rows.get_input_buffer(
                    self.stage_input_for_channel[first_trailing_stage][c],
                    y,
                    c,
                );
            }

            for i in first_trailing_stage..self.first_image_dim_stage {
                if span[i].xsize() == 0 {
                    continue;
                }
                let y0 = image_area_rect.y0().wrapping_add(y as usize);
                if y0 >= span[i].y1() {
                    continue;
                }
                let mut buffers = RowBuffers {
                    input: arena,
                    output: &mut [],
                };
                self.base.stages[i].process_row(
                    &input_rows[first_trailing_stage],
                    &output_rows,
                    &mut buffers,
                    0,
                    0,
                    span[i].xsize(),
                    span[i].x0(),
                    y0,
                    thread_id,
                )?;
            }

            if self.first_image_dim_stage == num_stages {
                continue;
            }

            // Skip pixels that are not part of the actual final image area.
            for c in 0..num_channels {
                let row = &mut input_rows[first_trailing_stage][c][0];
                *row = row.offset(x_pixels_skip);
            }
            // Avoid running pipeline stages on pixels that are outside the full
            // image area. As trailing stages have no borders, this is a free
            // optimization (and may be necessary for correctness, as some
            // stages assume coordinates are within bounds).
            let full_image_y: isize = (self.frame_origin.y0 as isize)
                .wrapping_add(image_area_rect.y0() as isize)
                .wrapping_add(y as isize);
            if full_image_y < 0 {
                continue;
            }

            for i in self.first_image_dim_stage..num_stages {
                if span[i].xsize() == 0 {
                    continue;
                }
                if full_image_y >= span[i].y1() as isize {
                    continue;
                }
                let mut buffers = RowBuffers {
                    input: arena,
                    output: &mut [],
                };
                self.base.stages[i].process_row(
                    &input_rows[first_trailing_stage],
                    &output_rows,
                    &mut buffers,
                    0,
                    0,
                    span[i].xsize(),
                    span[i].x0(),
                    full_image_y as usize,
                    thread_id,
                )?;
            }
        }
        Ok(())
    }

    /// Fills in input_rows and output_rows for a given y value (relative to the
    /// start of the group, measured in actual pixels at the appropriate
    /// vertical scaling factor) and a given stage, applying mirroring if
    /// necessary. This function is somewhat inefficient for trailing kInOut or
    /// kInput stages, where just filling the input row once ought to be
    /// sufficient.
    #[expect(
        clippy::needless_range_loop,
        reason = "Retain the source's indexed loops over parallel stage/channel tables."
    )]
    #[expect(
        clippy::too_many_arguments,
        reason = "The source lambda captures these RenderRect locals by reference."
    )]
    fn prepare_io_rows(
        &self,
        y: i32,
        i: usize,
        arena: &mut [ImageF],
        rows: &Rows,
        group_rect: &[Rect],
        input_rows_i: &mut RowInfo,
        output_rows: &mut RowInfo,
    ) {
        let stage = &self.base.stages[i];
        let settings = stage.settings();
        let bordery = settings.border_y as isize;
        let shifty = settings.shift_y;
        let make_row = |arena: &mut [ImageF], c: usize, iy: isize| -> Row {
            let mirrored_y = get_mirrored_y(
                (y as isize + iy - bordery) as i32,
                group_rect[i].y0() as isize,
                self.image_rect[i].ysize() as isize,
            ) as usize;
            let row =
                rows.get_input_buffer(self.stage_input_for_channel[i][c], mirrored_y as i32, c);
            apply_x_mirroring(
                arena,
                row,
                stage.settings().border_x as isize,
                group_rect[i].x0() as isize,
                group_rect[i].xsize() as isize,
                self.image_rect[i].xsize() as isize,
            );
            row
        };
        for (c, channel_rows) in input_rows_i.iter_mut().enumerate() {
            let mode = stage.get_channel_mode(c);
            if mode == Mode::Ignored {
                continue;
            }
            // If we already have rows from a previous iteration, we can just
            // shift the rows by 1 and insert the new one.
            if channel_rows.len() == 2 * bordery as usize + 1 {
                for iy in 0..2 * bordery as usize {
                    channel_rows[iy] = channel_rows[iy + 1];
                }
                channel_rows[bordery as usize * 2] = make_row(arena, c, bordery * 2);
            } else {
                channel_rows.resize(2 * bordery as usize + 1, NULL_ROW);
                for iy in 0..2 * bordery + 1 {
                    channel_rows[iy as usize] = make_row(arena, c, iy);
                }
            }

            // If necessary, get the output buffers.
            if mode == Mode::InOut {
                let bundle = 1usize << shifty;
                for iy in 0..bundle {
                    output_rows[c][iy] = rows.get_output_buffer(
                        i as i32,
                        ((y as isize as usize).wrapping_mul(bundle).wrapping_add(iy)) as i32,
                        c,
                    );
                }
            }
        }
    }

    #[allow(clippy::needless_range_loop, reason = "Keep the source index loops.")]
    fn render_padding(&mut self, thread_id: usize, rect: Rect) -> Status {
        if rect.xsize() == 0 {
            return Ok(());
        }
        let numc = self.base.channel_shifts[0].len();
        let input_rows: RowInfo = (0..numc)
            .map(|c| {
                vec![Row {
                    storage: RowStorage::Input,
                    channel: 0,
                    y: c,
                    x: 0,
                }]
            })
            .collect();
        let output_rows: RowInfo = Vec::new();
        let first_image_dim_stage = self.first_image_dim_stage;
        let num_stages = self.base.stages.len();
        let input = std::slice::from_mut(&mut self.out_of_frame_data[thread_id]);

        for y in 0..rect.ysize() {
            let mut buffers = RowBuffers {
                input: &mut *input,
                output: &mut [],
            };
            self.base.stages[first_image_dim_stage - 1].process_padding_row(
                &input_rows,
                &mut buffers,
                rect.xsize(),
                rect.x0(),
                rect.y0().wrapping_add(y),
            );
            for i in first_image_dim_stage..num_stages {
                let mut buffers = RowBuffers {
                    input: &mut *input,
                    output: &mut [],
                };
                self.base.stages[i].process_row(
                    &input_rows,
                    &output_rows,
                    &mut buffers,
                    0,
                    0,
                    rect.xsize(),
                    rect.x0(),
                    rect.y0().wrapping_add(y),
                    thread_id,
                )?;
            }
        }
        Ok(())
    }

    fn process_buffers_impl(&mut self, group_id: usize, thread_id: usize) -> Status {
        let input_set = if self.use_group_ids {
            group_id
        } else {
            thread_id
        };
        let num_channels = self.group_data[input_set].len();

        // Copy the group borders to the border storage.
        for c in 0..num_channels {
            self.save_borders(group_id, c, input_set)?;
        }

        let fd = self.base.frame_dimensions;
        let gy = group_id / fd.xsize_groups;
        let gx = group_id % fd.xsize_groups;

        if self.first_image_dim_stage != self.base.stages.len() {
            let group_dim = fd.group_dim << self.base_color_shift;
            let mut group_rect = SignedRect::new(
                gx.wrapping_mul(group_dim) as isize,
                gy.wrapping_mul(group_dim) as isize,
                group_dim,
                group_dim,
            );
            let mut image_rect = SignedRect::new(0, 0, fd.xsize_upsampled, fd.ysize_upsampled);
            let full_image_rect =
                SignedRect::new(0, 0, self.full_image_xsize, self.full_image_ysize);
            let ox = i64::from(self.frame_origin.x0);
            let oy = i64::from(self.frame_origin.y0);
            group_rect = group_rect.translate(ox, oy);
            image_rect = image_rect.translate(ox, oy);
            image_rect = image_rect.intersection(full_image_rect);
            group_rect = group_rect.intersection(image_rect);
            let x0 = group_rect.x0() as usize;
            let y0 = group_rect.y0() as usize;
            let x1 = group_rect.x1() as usize;
            let y1 = group_rect.y1() as usize;
            let full_x = self.full_image_xsize;
            let full_y = self.full_image_ysize;

            if group_id == 0 && (image_rect.xsize() == 0 || image_rect.ysize() == 0) {
                // If this frame does not intersect with the full image, we have
                // to initialize the whole image area with RenderPadding.
                self.render_padding(thread_id, Rect::new(0, 0, full_x, full_y))?;
            }

            // Render padding for groups that intersect with the full image. The
            // case where no groups intersect was handled above.
            if group_rect.xsize() > 0 && group_rect.ysize() > 0 {
                let last_x = gx + 1 == fd.xsize_groups;
                let last_y = gy + 1 == fd.ysize_groups;
                if gx == 0 && gy == 0 {
                    self.render_padding(thread_id, Rect::new(0, 0, x0, y0))?;
                }
                if gy == 0 {
                    self.render_padding(thread_id, Rect::new(x0, 0, x1.wrapping_sub(x0), y0))?;
                }
                if gx == 0 {
                    self.render_padding(thread_id, Rect::new(0, y0, x0, y1.wrapping_sub(y0)))?;
                }
                if gx == 0 && last_y {
                    self.render_padding(thread_id, Rect::new(0, y1, x0, full_y.wrapping_sub(y1)))?;
                }
                if last_y {
                    self.render_padding(
                        thread_id,
                        Rect::new(x0, y1, x1.wrapping_sub(x0), full_y.wrapping_sub(y1)),
                    )?;
                }
                if gy == 0 && last_x {
                    self.render_padding(thread_id, Rect::new(x1, 0, full_x.wrapping_sub(x1), y0))?;
                }
                if last_x {
                    self.render_padding(
                        thread_id,
                        Rect::new(x1, y0, full_x.wrapping_sub(x1), y1.wrapping_sub(y0)),
                    )?;
                }
                if last_y && last_x {
                    self.render_padding(
                        thread_id,
                        Rect::new(x1, y1, full_x.wrapping_sub(x1), full_y.wrapping_sub(y1)),
                    )?;
                }
            }
        }

        let mut ready_rects = [Rect::default(); GroupBorderAssigner::MAX_TO_FINALIZE];
        let mut num_ready_rects = 0;
        self.group_border_assigner.group_done(
            group_id,
            self.group_border.0,
            self.group_border.1,
            &mut ready_rects,
            &mut num_ready_rects,
        );
        for &image_max_color_channel_rect in ready_rects.iter().take(num_ready_rects) {
            for c in 0..num_channels {
                self.load_borders(group_id, c, image_max_color_channel_rect, input_set)?;
            }
            let data_max_color_channel_rect = Rect::new(
                self.group_data_x_border
                    .wrapping_add(image_max_color_channel_rect.x0())
                    .wrapping_sub(gx.wrapping_mul(fd.group_dim)),
                self.group_data_y_border
                    .wrapping_add(image_max_color_channel_rect.y0())
                    .wrapping_sub(gy.wrapping_mul(fd.group_dim)),
                image_max_color_channel_rect.xsize(),
                image_max_color_channel_rect.ysize(),
            );
            self.render_rect(
                thread_id,
                input_set,
                data_max_color_channel_rect,
                image_max_color_channel_rect,
            )?;
        }
        Ok(())
    }
}

impl<'a> RenderPipeline<'a> for LowMemoryRenderPipeline<'a> {
    fn base(&self) -> &RenderPipelineBase<'a> {
        &self.base
    }
    fn base_mut(&mut self) -> &mut RenderPipelineBase<'a> {
        &mut self.base
    }
    fn as_dyn_mut(&mut self) -> &mut (dyn RenderPipeline<'a> + 'a) {
        self
    }
    fn prepare_buffers(&mut self, group_id: usize, thread_id: usize) -> Vec<InputBuffer> {
        self.prepare_buffers_impl(group_id, thread_id)
    }
    fn process_buffers(&mut self, group_id: usize, thread_id: usize) -> Status {
        self.process_buffers_impl(group_id, thread_id)
    }
    fn prepare_for_threads_internal(&mut self, num: usize, use_group_ids: bool) -> Status {
        self.prepare_for_threads_impl(num, use_group_ids)
    }
    fn init(&mut self) -> Status {
        self.init_impl()
    }
    fn clear_done(&mut self, i: usize) {
        self.group_border_assigner.clear_done(i);
    }
    fn input_plane(&self, set: usize, channel: usize) -> &Plane<f32> {
        &self.group_data[set][channel]
    }
    fn input_plane_mut(&mut self, set: usize, channel: usize) -> &mut Plane<f32> {
        &mut self.group_data[set][channel]
    }
    fn input_planes_mut(&mut self, set: usize) -> &mut [Plane<f32>] {
        &mut self.group_data[set]
    }
}

// A descriptor for the source's value-initialized (null) row pointers. The
// stages' RowBuffers in this pipeline expose no Output storage, so resolving
// it fails like a null dereference instead of aliasing a live row.
const NULL_ROW: Row = Row {
    storage: RowStorage::Output,
    channel: 0,
    y: 0,
    x: 0,
};

fn get_mirrored_y(y: i32, group_y0: isize, image_ysize: isize) -> i32 {
    if group_y0 == 0 && (y < 0 || y as isize + group_y0 >= image_ysize) {
        return mirror(i64::from(y), image_ysize as i64) as i32;
    }
    if y as isize + group_y0 >= image_ysize {
        // Here we know that the one mirroring step is sufficient.
        return (2 * image_ysize - (y as isize + group_y0) - 1 - group_y0) as i32;
    }
    y
}

/// The source's `row[k]` for a row pointer descriptor: the pointer column plus
/// a signed in-row offset.
fn row_index(row: Row, k: isize) -> usize {
    row.x.wrapping_add_signed(k)
}

fn apply_x_mirroring(
    arena: &mut [ImageF],
    row: Row,
    borderx: isize,
    group_x0: isize,
    group_xsize: isize,
    image_xsize: isize,
) {
    let off = X_OFFSET as isize;
    let data = arena[row.channel].row_mut(row.y);
    let mut copy = |to: isize, from: isize| {
        data[row_index(row, to)] = data[row_index(row, from)];
    };
    if image_xsize <= borderx {
        if group_x0 == 0 {
            for ix in 0..borderx {
                copy(
                    off - ix - 1,
                    off + mirror((-ix - 1) as i64, image_xsize as i64) as isize,
                );
            }
        }
        if group_xsize + borderx + group_x0 >= image_xsize {
            for ix in 0..borderx {
                copy(
                    off + image_xsize + ix - group_x0,
                    off + mirror((image_xsize + ix) as i64, image_xsize as i64) as isize - group_x0,
                );
            }
        }
    } else {
        // Here we know that the one mirroring step is sufficient.
        if group_x0 == 0 {
            for ix in 0..borderx {
                copy(off - ix - 1, off + ix);
            }
        }
        if group_xsize + borderx + group_x0 >= image_xsize {
            for ix in 0..borderx {
                copy(
                    off + image_xsize - group_x0 + ix,
                    off + image_xsize - group_x0 - ix - 1,
                );
            }
        }
    }
}

// Information about where the *output* of each stage is stored.
struct RowsInfo {
    // Index of the lent plane holding the first row, and the source pointer's
    // row and signed column within it.
    plane: usize,
    y0: isize,
    x0: isize,
    // Modulo value for the y axis minus 1 (ymod is guaranteed to be a power of
    // 2, which allows efficient mod computation by masking).
    ymod_minus_1: i32,
}
const DEFAULT_ROWS_INFO: RowsInfo = RowsInfo {
    plane: 0,
    y0: 0,
    x0: 0,
    ymod_minus_1: 0,
};

struct Rows {
    rows: Vec<Vec<RowsInfo>>,
}
impl Rows {
    /// `arena` holds the group input planes (channel order) followed by this
    /// thread's stage buffers in [channel][stage] order.
    #[expect(
        clippy::needless_range_loop,
        reason = "Retain the source's indexed loops over parallel stage/channel tables."
    )]
    #[expect(
        clippy::too_many_arguments,
        reason = "Retain the original Rows::Create parameter list."
    )]
    fn create(
        stages: &[Box<dyn super::render_pipeline_stage::RenderPipelineStage + '_>],
        data_max_color_channel_rect: Rect,
        group_data_x_border: i32,
        group_data_y_border: i32,
        group_data_shift: &[(usize, usize)],
        base_color_shift: usize,
        arena: &[ImageF],
        num_channels: usize,
    ) -> StatusOr<Self> {
        let num_stages = stages.len();
        if group_data_shift.len() != num_channels {
            return Err(StatusCode::GenericError);
        }

        let mut rows: Vec<Vec<RowsInfo>> = (0..num_stages + 1)
            .map(|_| (0..num_channels).map(|_| DEFAULT_ROWS_INFO).collect())
            .collect();

        for i in 0..num_stages {
            for c in 0..num_channels {
                if stages[i].get_channel_mode(c) == Mode::InOut {
                    let plane = num_channels + c * num_stages + i;
                    let buffer = &arena[plane];
                    rows[i + 1][c] = RowsInfo {
                        plane,
                        y0: 0,
                        x0: 0,
                        ymod_minus_1: (buffer.ysize() as i32).wrapping_sub(1),
                    };
                }
            }
        }

        for c in 0..num_channels {
            let tmp = SignedRect::from_rect(data_max_color_channel_rect)
                .translate(
                    -i64::from(group_data_x_border),
                    -i64::from(group_data_y_border),
                )
                .shift_left(base_color_shift, base_color_shift);
            let tmp = tmp.ceil_shift_right(group_data_shift[c])?;
            let channel_group_data_rect = tmp.translate(
                (group_data_x_border as isize - X_OFFSET as isize) as i64,
                i64::from(group_data_y_border),
            );
            rows[0][c] = RowsInfo {
                plane: c,
                y0: channel_group_data_rect.y0(),
                x0: channel_group_data_rect.x0(),
                ymod_minus_1: -1,
            };
        }
        Ok(Self { rows })
    }

    // Stage -1 refers to the input data; all other values must be nonnegative
    // and refer to the data for the output of that stage.
    //
    // Row `y` of the output of a stage lives in slot `y mod ring size` of a
    // ring buffer, and is overwritten once the stage produces row
    // `y + ring size`.
    fn get_input_buffer(&self, stage: i32, y: i32, c: usize) -> Row {
        self.get_buffer(stage, y, c)
    }

    fn get_output_buffer(&self, stage: i32, y: i32, c: usize) -> Row {
        self.get_buffer(stage, y, c)
    }

    fn get_buffer(&self, stage: i32, y: i32, c: usize) -> Row {
        let info = &self.rows[(stage + 1) as usize][c];
        Row {
            storage: RowStorage::Input,
            channel: info.plane,
            y: info.y0.wrapping_add((y & info.ymod_minus_1) as isize) as usize,
            x: info.x0 as usize,
        }
    }
}
