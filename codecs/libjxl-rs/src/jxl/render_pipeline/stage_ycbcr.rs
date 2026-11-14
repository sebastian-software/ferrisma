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
// double promotion/product/narrowing and separate binary32 addition, in the
// source operand order on every target (NaN payloads are not gated, D8).
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
fn ma(a: f32, b: f32, c: f32) -> f32 {
    add(product(promote(a), promote(b)), c)
}
pub struct YCbCrStage;
impl RenderPipelineStage for YCbCrStage {
    fn as_concurrent(&self) -> Option<&(dyn RenderPipelineStage + Sync)> {
        Some(self)
    }
    fn settings(&self) -> Settings {
        Settings::default()
    }
    fn process_row(
        &self,
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
        // In place on three disjoint spans resolved once per row; each sample
        // is read and written in the original order.
        let [r0, r1, r2] = buffers.three_rows_mut(rows, width);
        for x in 0..width {
            let y = add(r1[x], c128);
            let cb = r0[x];
            let cr = r2[x];
            let r = ma(crcr, cr, y);
            let g = ma(cgcr, cr, ma(cgcb, cb, y));
            let b = ma(cbcb, cb, y);
            r0[x] = r;
            r1[x] = g;
            r2[x] = b;
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
    // HWY_DYNAMIC_DISPATCH(GetYCbCrStage) (port ADR 0004).
    if let Some(target) = crate::jxl::hwy::decoder_target() {
        return super::stage_chroma_simd::get_ycbcr_stage_simd(target);
    }
    Box::new(YCbCrStage)
}
