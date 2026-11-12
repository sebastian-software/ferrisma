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
// The row kernels of `scalar_rows` keep these Add/promote/product boundaries
// per operation over a whole row.
use super::scalar_rows::{self as k, Broadcast, lanes};

/// The addition of MulAdd(a, b, c) on rows: `out = add(c, product)` on the
/// established ARM mapping, `add(product, c)` on the selected original Linux
/// x86_64 operands. Preserve both rounding sites.
fn ma_add(out: &mut [f32], product: &[f32], c: &[f32]) {
    #[cfg(all(target_os = "linux", target_arch = "x86_64"))]
    {
        k::add(out, product, c)
    }
    #[cfg(not(all(target_os = "linux", target_arch = "x86_64")))]
    {
        k::add(out, c, product)
    }
}

/// Scratch rows of the row kernels.
#[derive(Default)]
struct YCbCrRows {
    f: [Vec<f32>; 5],
    d: [Vec<f64>; 2],
    /// c128, then the promoted crcr, cgcb, cgcr and cbcb.
    scalars: [Broadcast; 5],
}
pub struct YCbCrStage {
    rows: YCbCrRows,
}
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
        // In place on three disjoint spans resolved once per row; each sample
        // is read and written in the original order.
        let [r0, r1, r2] = buffers.three_rows_mut(rows, width);
        // Lane by lane through the row kernels: y = add(Y, c128),
        // r = ma(crcr, cr, y), g = ma(cgcr, cr, ma(cgcb, cb, y)),
        // b = ma(cbcb, cb, y), with the original operands and order.
        let n = width;
        let YCbCrRows { f, d, scalars } = &mut self.rows;
        let [y, t, r, g, p] = f;
        let [wcb, wcr] = d;
        let [s128, s_crcr, s_cgcb, s_cgcr, s_cbcb] = scalars;
        let (y, t, r, g, p) = (
            lanes(y, n),
            lanes(t, n),
            lanes(r, n),
            lanes(g, n),
            lanes(p, n),
        );
        let (wcb, wcr) = (lanes(wcb, n), lanes(wcr, n));
        k::add(y, r1, s128.row(c128, n));
        k::promote(wcb, r0);
        k::promote(wcr, r2);
        k::product(p, s_crcr.promoted(crcr, n), wcr);
        ma_add(r, p, y);
        k::product(p, s_cgcb.promoted(cgcb, n), wcb);
        ma_add(t, p, y);
        k::product(p, s_cgcr.promoted(cgcr, n), wcr);
        ma_add(g, p, t);
        k::product(p, s_cbcb.promoted(cbcb, n), wcb);
        ma_add(t, p, y);
        r0.copy_from_slice(r);
        r1.copy_from_slice(g);
        r2.copy_from_slice(t);
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
    Box::new(YCbCrStage {
        rows: YCbCrRows::default(),
    })
}
