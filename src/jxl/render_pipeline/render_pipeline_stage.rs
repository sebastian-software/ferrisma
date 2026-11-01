// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
//! RenderPipelineStage/settings/default methods from render_pipeline/
//! render_pipeline_stage.{h,cc}. Owner-relative row descriptors replace float
//! pointers. RowBuffers resolves them under one exclusive storage borrow.
use crate::jxl::base::status::{Status, StatusCode};
use crate::jxl::frame_header::FrameOrigin;
use crate::jxl::image::Plane;

#[cfg(any(target_arch = "arm", target_arch = "aarch64"))]
pub const RENDER_PIPELINE_X_OFFSET: usize = 24;
#[cfg(not(any(target_arch = "arm", target_arch = "aarch64")))]
pub const RENDER_PIPELINE_X_OFFSET: usize = 32;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(i32)]
pub enum RenderPipelineChannelMode {
    Ignored = 0,
    InPlace = 1,
    InOut = 2,
    Input = 3,
}
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Settings {
    pub border_x: usize,
    pub border_y: usize,
    pub shift_x: usize,
    pub shift_y: usize,
}
impl Settings {
    pub fn shift_x(shift: usize, border: usize) -> Self {
        Self {
            border_x: border,
            shift_x: shift,
            ..Self::default()
        }
    }
    pub fn shift_y(shift: usize, border: usize) -> Self {
        Self {
            border_y: border,
            shift_y: shift,
            ..Self::default()
        }
    }
    pub fn symmetric(shift: usize, border: usize) -> Self {
        Self {
            border_x: border,
            border_y: border,
            shift_x: shift,
            shift_y: shift,
        }
    }
    pub fn symmetric_border_only(border: usize) -> Self {
        Self::symmetric(0, border)
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RowStorage {
    Input,
    Output,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Row {
    pub storage: RowStorage,
    pub channel: usize,
    pub y: usize,
    pub x: usize,
}
impl Row {
    pub fn offset(self, x: isize) -> Self {
        Self {
            x: self.x.wrapping_add_signed(x),
            ..self
        }
    }
}
pub type RowInfo = Vec<Vec<Row>>;
pub struct RowBuffers<'a> {
    pub input: &'a mut [Plane<f32>],
    pub output: &'a mut [Plane<f32>],
}
impl RowBuffers<'_> {
    pub fn row(&self, row: Row) -> &[f32] {
        let planes = match row.storage {
            RowStorage::Input => &*self.input,
            RowStorage::Output => &*self.output,
        };
        &planes[row.channel].row(row.y)[row.x..]
    }
    pub fn row_mut(&mut self, row: Row) -> &mut [f32] {
        let planes = match row.storage {
            RowStorage::Input => &mut *self.input,
            RowStorage::Output => &mut *self.output,
        };
        &mut planes[row.channel].row_mut(row.y)[row.x..]
    }
}
pub trait RenderPipelineStage {
    fn settings(&self) -> Settings;
    #[expect(
        clippy::too_many_arguments,
        reason = "Retain the original stage row boundary, with explicit borrowed row storage."
    )]
    fn process_row(
        &mut self,
        input_rows: &RowInfo,
        output_rows: &RowInfo,
        buffers: &mut RowBuffers<'_>,
        xextra_left: usize,
        xextra_right: usize,
        xsize: usize,
        xpos: usize,
        ypos: usize,
        thread_id: usize,
    ) -> Status;
    fn get_channel_mode(&self, c: usize) -> RenderPipelineChannelMode;
    fn get_name(&self) -> &str;
    // These are the complete original base implementations, not substitutes
    // for an unported concrete stage. Concrete overrides retain their own work.
    fn is_initialized(&self) -> Status {
        Ok(())
    }
    fn set_input_sizes(&mut self, _input_sizes: &[(usize, usize)]) -> Status {
        Ok(())
    }
    fn prepare_for_threads(&mut self, _num_threads: usize) -> Status {
        Ok(())
    }
    fn switch_to_image_dimensions(&self) -> bool {
        false
    }
    fn get_image_dimensions(
        &self,
        _xsize: &mut usize,
        _ysize: &mut usize,
        _origin: &mut FrameOrigin,
    ) {
    }
    fn process_padding_row(
        &mut self,
        _output_rows: &RowInfo,
        _buffers: &mut RowBuffers<'_>,
        _xsize: usize,
        _xpos: usize,
        _ypos: usize,
    ) {
    }
    fn get_input_row(&self, rows: &RowInfo, c: usize, offset: i32) -> Row {
        assert_ne!(self.get_channel_mode(c), RenderPipelineChannelMode::Ignored);
        let border = self.settings().border_y;
        assert!((offset as i64).abs() <= border as i64);
        rows[c][border.wrapping_add_signed(offset as isize)]
            .offset(RENDER_PIPELINE_X_OFFSET as isize)
    }
    fn get_output_row(&self, rows: &RowInfo, c: usize, offset: usize) -> Row {
        assert_eq!(self.get_channel_mode(c), RenderPipelineChannelMode::InOut);
        assert!(offset <= 1usize << self.settings().shift_y);
        rows[c][offset].offset(RENDER_PIPELINE_X_OFFSET as isize)
    }
}
/// Complete original WriteToImage3FStage and factory from stage_write.cc.
pub struct WriteToImage3FStage<'a> {
    image: &'a mut crate::jxl::image::Image3<f32>,
}
impl<'a> WriteToImage3FStage<'a> {
    pub fn new(image: &'a mut crate::jxl::image::Image3<f32>) -> Self {
        Self { image }
    }
}
impl RenderPipelineStage for WriteToImage3FStage<'_> {
    fn settings(&self) -> Settings {
        Settings::default()
    }
    fn set_input_sizes(&mut self, sizes: &[(usize, usize)]) -> Status {
        if sizes.len() < 3 {
            return Err(StatusCode::GenericError);
        }
        for c in 1..3 {
            if sizes[c].0 != sizes[0].0 || sizes[c].1 != sizes[0].1 {
                return Err(StatusCode::GenericError);
            }
        }
        *self.image = crate::jxl::image::Image3::create(sizes[0].0, sizes[0].1)?;
        Ok(())
    }
    fn process_row(
        &mut self,
        input: &RowInfo,
        _output: &RowInfo,
        buffers: &mut RowBuffers<'_>,
        left: usize,
        right: usize,
        width: usize,
        x: usize,
        y: usize,
        _thread: usize,
    ) -> Status {
        if left != 0 || right != 0 {
            return Err(StatusCode::GenericError);
        }
        for c in 0..3 {
            let from = self.get_input_row(input, c, 0);
            self.image.plane_row_mut(c, y)[x..x + width]
                .copy_from_slice(&buffers.row(from)[..width]);
        }
        Ok(())
    }
    fn get_channel_mode(&self, c: usize) -> RenderPipelineChannelMode {
        if c < 3 {
            RenderPipelineChannelMode::Input
        } else {
            RenderPipelineChannelMode::Ignored
        }
    }
    fn get_name(&self) -> &str {
        "WriteI3F"
    }
}

/// Complete original WriteToImageBundleStage/factory from stage_write.cc.
pub struct WriteToImageBundleStage<'b, 'm> {
    image_bundle: &'b mut crate::jxl::image_bundle::ImageBundle<'m>,
    color_encoding: crate::jxl::color_encoding_internal::ColorEncoding,
}
impl<'b, 'm> WriteToImageBundleStage<'b, 'm> {
    pub fn new(
        image_bundle: &'b mut crate::jxl::image_bundle::ImageBundle<'m>,
        color_encoding: &crate::jxl::color_encoding_internal::ColorEncoding,
    ) -> Self {
        Self {
            image_bundle,
            color_encoding: color_encoding.clone(),
        }
    }
}
impl RenderPipelineStage for WriteToImageBundleStage<'_, '_> {
    fn settings(&self) -> Settings {
        Settings::default()
    }
    fn set_input_sizes(&mut self, sizes: &[(usize, usize)]) -> Status {
        if sizes.len() < 3 {
            return Err(StatusCode::GenericError);
        }
        for c in 1..sizes.len() {
            if sizes[c].0 != sizes[0].0 {
                return Err(StatusCode::GenericError);
            }
            if sizes[c].1 != sizes[0].1 {
                return Err(StatusCode::GenericError);
            }
        }
        let mut tmp = Some(crate::jxl::image::Image3::<f32>::create(
            sizes[0].0, sizes[0].1,
        )?);
        self.image_bundle
            .set_from_image(&mut tmp, &self.color_encoding)?;
        self.image_bundle.extra_channels_mut().clear();
        for &(w, h) in sizes.iter().skip(3) {
            let ch = Plane::create(w, h)?;
            self.image_bundle.extra_channels_mut().push(ch);
        }
        Ok(())
    }
    fn process_row(
        &mut self,
        input: &RowInfo,
        _output: &RowInfo,
        buffers: &mut RowBuffers<'_>,
        left: usize,
        right: usize,
        width: usize,
        x: usize,
        y: usize,
        _thread: usize,
    ) -> Status {
        if left != 0 || right != 0 {
            return Err(StatusCode::GenericError);
        }
        for c in 0..3 {
            let from = self.get_input_row(input, c, 0);
            self.image_bundle.color_mut().plane_row_mut(c, y)[x..x + width]
                .copy_from_slice(&buffers.row(from)[..width]);
        }
        for ec in 0..self.image_bundle.extra_channels().len() {
            if self.image_bundle.extra_channels()[ec].xsize() < x.wrapping_add(width) {
                return Err(StatusCode::GenericError);
            }
            let from = self.get_input_row(input, 3 + ec, 0);
            self.image_bundle.extra_channels_mut()[ec].row_mut(y)
                [x.wrapping_sub(left)..x.wrapping_sub(left).wrapping_add(width)]
                .copy_from_slice(&buffers.row(from)[..width]);
        }
        Ok(())
    }
    fn get_channel_mode(&self, _c: usize) -> RenderPipelineChannelMode {
        RenderPipelineChannelMode::Input
    }
    fn get_name(&self) -> &str {
        "WriteIB"
    }
}
