// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
//! Complete scalar SplineStage/factory from render_pipeline/stage_splines.{h,cc}.
//! Borrowed or shared ownership retains the source's immutable spline-cache
//! access without copying cache state. Shared ownership removes a decoder
//! self-reference; RefCell enforces exclusive mutation between row calls. Disjoint
//! channel borrows replace its three restricted row pointers.
use super::render_pipeline_stage::{
    RenderPipelineChannelMode as Mode, RenderPipelineStage, RowBuffers, RowInfo, Settings,
};
use crate::jxl::base::status::Status;
use crate::jxl::splines::Splines;
use std::cell::RefCell;
use std::rc::Rc;

enum SplineOwner<'a, 'data> {
    Borrowed(&'a Splines<'data>),
    Shared(Rc<RefCell<Splines<'data>>>),
}
pub struct SplineStage<'a, 'data> {
    splines: SplineOwner<'a, 'data>,
}
impl RenderPipelineStage for SplineStage<'_, '_> {
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
        x: usize,
        y: usize,
        _thread: usize,
    ) -> Status {
        let rows: [_; 3] =
            std::array::from_fn(|c| self.get_input_row(input, c, 0).offset(-(left as isize)));
        let [row_x, row_y, row_b] = buffers.three_rows_mut(rows, left + width + right);
        match &self.splines {
            SplineOwner::Borrowed(splines) => splines.add_to_row(
                row_x,
                row_y,
                row_b,
                y,
                x.wrapping_sub(left),
                x.wrapping_add(width).wrapping_add(right),
            ),
            SplineOwner::Shared(splines) => splines.borrow().add_to_row(
                row_x,
                row_y,
                row_b,
                y,
                x.wrapping_sub(left),
                x.wrapping_add(width).wrapping_add(right),
            ),
        }
        Ok(())
    }
    fn get_channel_mode(&self, c: usize) -> Mode {
        if c < 3 { Mode::InPlace } else { Mode::Ignored }
    }
    fn get_name(&self) -> &str {
        "Splines"
    }
}
pub fn get_spline_stage<'a, 'data>(splines: &'a Splines<'data>) -> SplineStage<'a, 'data> {
    SplineStage {
        splines: SplineOwner::Borrowed(splines),
    }
}

/// The shared owner stays identical to the decoder's owner; the stage holds an
/// immutable runtime borrow only while processing a row.
pub fn get_spline_stage_shared<'data>(
    splines: Rc<RefCell<Splines<'data>>>,
) -> SplineStage<'data, 'data> {
    SplineStage {
        splines: SplineOwner::Shared(splines),
    }
}
