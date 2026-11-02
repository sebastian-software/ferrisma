// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
//! Complete scalar kYCbCrStage and factory from stage_ycbcr.{h,cc}.
use super::render_pipeline_stage::{
    RenderPipelineChannelMode as Mode, RenderPipelineStage, RowBuffers, RowInfo, Settings,
};
use crate::jxl::base::status::{Status, StatusCode};
// Copyright 2019 Google LLC
// Copyright (c) The Highway Project Authors. All rights reserved.
// SPDX-License-Identifier: BSD-3-Clause
// Scalar Add/MulAdd/operator* from hwy/ops/scalar-inl.h. Keep the original
// double promotion/product/narrowing and separate binary32 addition.
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
// Both complete native ARM original objects keep the addend first.
fn ma(a: f32, b: f32, c: f32) -> f32 {
    add(c, product(promote(a), promote(b)))
}
pub struct YCbCrStage;
impl RenderPipelineStage for YCbCrStage {
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
        let c128 = 128.0f32 / 255.0;
        let crcr = 1.402f32;
        let cgcb = -0.114f32 * 1.772f32 / 0.587f32;
        let cgcr = -0.299f32 * 1.402f32 / 0.587f32;
        let cbcb = 1.772f32;
        let rows: [_; 3] = std::array::from_fn(|c| self.get_input_row(input, c, 0));
        for x in 0..width {
            let y = add(buffers.row(rows[1])[x], c128);
            let cb = buffers.row(rows[0])[x];
            let cr = buffers.row(rows[2])[x];
            let r = ma(crcr, cr, y);
            let g = ma(cgcr, cr, ma(cgcb, cb, y));
            let b = ma(cbcb, cb, y);
            buffers.row_mut(rows[0])[x] = r;
            buffers.row_mut(rows[1])[x] = g;
            buffers.row_mut(rows[2])[x] = b;
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
pub fn get_ycbcr_stage() -> Box<dyn RenderPipelineStage> {
    Box::new(YCbCrStage)
}
