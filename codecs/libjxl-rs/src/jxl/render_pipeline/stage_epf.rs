// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
//! Complete one-lane EPF0/1/2 stages and factory from stage_epf.{h,cc}.
//! Owner-relative row descriptors replace pointers; the sigma plane remains a
//! borrow of the original frame owner. SIMD specializations are separate.
use super::render_pipeline_stage::{
    PerThread, RenderPipelineChannelMode as Mode, RenderPipelineStage, Row, RowBuffers, RowInfo,
    Settings,
};
use crate::jxl::{
    base::float::{add_first_nan, mul_first_nan, mul_promoted_first_nan},
    base::status::Status,
    dec_cache::SIGMA_PADDING,
    epf::MIN_SIGMA,
    image::Plane,
    loop_filter::LoopFilter,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum EpfStage {
    Zero,
    One,
    Two,
}
impl EpfStage {
    fn simd(self) -> super::stage_epf_simd::EpfPass {
        use super::stage_epf_simd::EpfPass;
        match self {
            Self::Zero => EpfPass::Zero,
            Self::One => EpfPass::One,
            Self::Two => EpfPass::Two,
        }
    }
}

// Copyright 2019 Google LLC
// Copyright (c) The Highway Project Authors. All rights reserved.
// SPDX-License-Identifier: BSD-3-Clause
// Scalar Add, AbsDiff, MulAdd and operator* from hwy/ops/scalar-inl.h.
// Each scalar multiplication explicitly promotes to double, then narrows.
// The weights' ZeroIfNegative observes a NaN's sign, so Add and operator*
// keep the source's first-operand NaN selection (`base::float`); AbsDiff
// clears the sign and needs none.
#[inline]
fn add(a: f32, b: f32) -> f32 {
    add_first_nan(a, b)
}
#[inline]
fn sub(a: f32, b: f32) -> f32 {
    a - b
}
#[inline]
fn mul(a: f32, b: f32) -> f32 {
    mul_promoted_first_nan(a, b)
}
#[inline]
fn ma(a: f32, b: f32, c: f32) -> f32 {
    add(mul(a, b), c)
}
#[inline]
fn diff(a: f32, b: f32) -> f32 {
    sub(a, b).abs()
}
#[inline]
fn weight(sad: f32, inv_sigma: f32) -> f32 {
    let v = ma(sad, inv_sigma, 1.0);
    if v.is_sign_negative() { 0.0 } else { v }
}

enum SigmaOwner<'a> {
    Borrowed(&'a Plane<f32>),
    Shared(crate::jxl::base::sync_cell::SharedSigma),
}
pub struct EPFStage<'a> {
    lf: LoopFilter,
    sigma: SigmaOwner<'a>,
    pass: EpfStage,
    /// Output rows of one call, written back after all samples are computed.
    scratch: PerThread<[Vec<f32>; 3]>,
}
impl EPFStage<'_> {
    fn border(&self) -> usize {
        match self.pass {
            EpfStage::Zero => 3,
            EpfStage::One => 2,
            EpfStage::Two => 1,
        }
    }
}
impl RenderPipelineStage for EPFStage<'_> {
    fn as_concurrent(&self) -> Option<&(dyn RenderPipelineStage + Sync)> {
        Some(self)
    }
    fn settings(&self) -> Settings {
        Settings::symmetric_border_only(self.border())
    }
    #[expect(
        clippy::manual_is_multiple_of,
        reason = "Mirror the source modulo test."
    )]
    #[allow(clippy::needless_range_loop, reason = "Keep the source index loops.")]
    fn process_row(
        &self,
        input: &RowInfo,
        output: &RowInfo,
        buffers: &mut RowBuffers<'_>,
        left: usize,
        right: usize,
        width: usize,
        xpos: usize,
        ypos: usize,
        thread: usize,
    ) -> Status {
        let sigma_borrow = match &self.sigma {
            SigmaOwner::Shared(owner) => Some(owner.borrow()),
            SigmaOwner::Borrowed(_) => None,
        };
        let sigma = match &self.sigma {
            SigmaOwner::Borrowed(plane) => *plane,
            SigmaOwner::Shared(_) => sigma_borrow.as_deref().unwrap(),
        };
        let row_sigma = sigma.row(ypos / 8 + SIGMA_PADDING);
        // The unsuffixed 1.65 in passes 0/2 is evaluated as double in C++.
        let sm = match self.pass {
            EpfStage::Zero => (f64::from(self.lf.epf_pass0_sigma_scale) * 1.65) as f32,
            EpfStage::One => 1.65f32,
            EpfStage::Two => (f64::from(self.lf.epf_pass2_sigma_scale) * 1.65) as f32,
        };
        // float bsm = sm * lf_.epf_border_sad_mul; its NaN sign reaches the weights.
        let bsm = mul_first_nan(sm, self.lf.epf_border_sad_mul);
        let border = self.border() as i32;
        // Keep source row descriptors on the stack. Passes 1/2 use only the
        // first 5/3 slots; safely initialized unused slots are never observed.
        let mut rows = [[self.get_input_row(input, 0, -border); 7]; 3];
        for (c, channel) in rows.iter_mut().enumerate() {
            for (i, row) in channel
                .iter_mut()
                .take((2 * border + 1) as usize)
                .enumerate()
            {
                *row = self.get_input_row(input, c, i as i32 - border);
            }
        }
        let outs: [_; 3] = std::array::from_fn(|c| self.get_output_row(output, c, 0));
        // The original's row pointers: every input row span is resolved once
        // per call. Outputs are staged in scratch rows because input and
        // output rows may share a plane here.
        let spans: [[&[f32]; 7]; 3] = std::array::from_fn(|c| {
            std::array::from_fn(|i| buffers.row(Row { x: 0, ..rows[c][i] }))
        });
        let mut scratch = self.scratch.with(thread, std::mem::take);
        for row in &mut scratch {
            row.clear();
        }
        let x_begin = -(left as isize);
        for x in x_begin..width.wrapping_add(right) as isize {
            let bx = (x as usize)
                .wrapping_add(xpos)
                .wrapping_add(SIGMA_PADDING * 8)
                / 8;
            let ix = (x as usize).wrapping_add(xpos) % 8;
            let get = |c: usize, y: i32, dx: isize| {
                let row = rows[c][(border + y) as usize];
                spans[c][(border + y) as usize][row.x.wrapping_add_signed(x + dx)]
            };
            let center: [f32; 3] = std::array::from_fn(|c| get(c, 0, 0));
            if row_sigma[bx] < MIN_SIGMA {
                for c in 0..3 {
                    scratch[c].push(center[c]);
                }
                continue;
            }
            let inv_sigma = mul(
                row_sigma[bx],
                if ypos % 8 == 0 || ypos % 8 == 7 || ix == 0 || ix == 7 {
                    bsm
                } else {
                    sm
                },
            );
            let mut accum = center;
            let mut w = 1.0;
            match self.pass {
                EpfStage::Zero => {
                    const OFF: [(i32, isize); 12] = [
                        (-2, 0),
                        (-1, -1),
                        (-1, 0),
                        (-1, 1),
                        (0, -2),
                        (0, -1),
                        (0, 1),
                        (0, 2),
                        (1, -1),
                        (1, 0),
                        (1, 1),
                        (2, 0),
                    ];
                    let mut sads = [0.0; 12];
                    for c in 0..3 {
                        for (i, &(dy, dx)) in OFF.iter().enumerate() {
                            let mut sad = 0.0;
                            for (py, px) in [(0, 0), (-1, 0), (0, -1), (1, 0), (0, 1)] {
                                sad = add(sad, diff(get(c, py, px), get(c, dy + py, dx + px)));
                            }
                            sads[i] = ma(sad, self.lf.epf_channel_scale[c], sads[i]);
                        }
                    }
                    for (i, &(dy, dx)) in OFF.iter().enumerate() {
                        let weight = weight(sads[i], inv_sigma);
                        w = add(w, weight);
                        for (c, value) in accum.iter_mut().enumerate() {
                            *value = ma(weight, get(c, dy, dx), *value);
                        }
                    }
                }
                EpfStage::One => {
                    let mut sads = [0.0; 4];
                    for c in 0..3 {
                        let p20 = get(c, -2, 0);
                        let p21 = get(c, -1, 0);
                        let mut sad0c = diff(p20, p21);
                        let p11 = get(c, -1, -1);
                        let mut sad1c = diff(p11, p21);
                        let p31 = get(c, -1, 1);
                        let mut sad2c = diff(p31, p21);
                        let p02 = get(c, 0, -2);
                        let p12 = get(c, 0, -1);
                        sad1c = add(sad1c, diff(p02, p12));
                        sad0c = add(sad0c, diff(p11, p12));
                        let p22 = get(c, 0, 0);
                        let mut t = diff(p12, p22);
                        sad1c = add(sad1c, t);
                        sad2c = add(sad2c, t);
                        t = diff(p22, p21);
                        let mut sad3c = t;
                        sad0c = add(sad0c, t);
                        let p32 = get(c, 0, 1);
                        sad0c = add(sad0c, diff(p31, p32));
                        t = diff(p22, p32);
                        sad1c = add(sad1c, t);
                        sad2c = add(sad2c, t);
                        let p42 = get(c, 0, 2);
                        sad2c = add(sad2c, diff(p42, p32));
                        let p13 = get(c, 1, -1);
                        sad3c = add(sad3c, diff(p13, p12));
                        let p23 = get(c, 1, 0);
                        t = diff(p22, p23);
                        sad0c = add(sad0c, t);
                        sad3c = add(sad3c, t);
                        sad1c = add(sad1c, diff(p13, p23));
                        let p33 = get(c, 1, 1);
                        sad2c = add(sad2c, diff(p33, p23));
                        sad3c = add(sad3c, diff(p33, p32));
                        let p24 = get(c, 2, 0);
                        sad3c = add(sad3c, diff(p24, p23));
                        for (i, sad) in [sad0c, sad1c, sad2c, sad3c].into_iter().enumerate() {
                            sads[i] = ma(sad, self.lf.epf_channel_scale[c], sads[i]);
                        }
                    }
                    for (i, (dy, dx)) in [(-1, 0), (0, -1), (0, 1), (1, 0)].into_iter().enumerate()
                    {
                        let weight = weight(sads[i], inv_sigma);
                        w = add(w, weight);
                        for (c, value) in accum.iter_mut().enumerate() {
                            *value = ma(weight, get(c, dy, dx), *value);
                        }
                    }
                }
                EpfStage::Two => {
                    for (dy, dx) in [(-1, 0), (0, -1), (0, 1), (1, 0)] {
                        let candidate: [f32; 3] = std::array::from_fn(|c| get(c, dy, dx));
                        let mut sad =
                            mul(diff(candidate[0], center[0]), self.lf.epf_channel_scale[0]);
                        sad = ma(
                            diff(candidate[1], center[1]),
                            self.lf.epf_channel_scale[1],
                            sad,
                        );
                        sad = ma(
                            diff(candidate[2], center[2]),
                            self.lf.epf_channel_scale[2],
                            sad,
                        );
                        let weight = weight(sad, inv_sigma);
                        w = add(w, weight);
                        for c in 0..3 {
                            accum[c] = ma(weight, candidate[c], accum[c]);
                        }
                    }
                }
            }
            let inv_w = 1.0 / w; // JXL_HIGH_PRECISION defaults to 1 in the pinned source.
            for c in 0..3 {
                scratch[c].push(mul(accum[c], inv_w));
            }
        }
        for c in 0..3 {
            let out = buffers.row_mut(outs[c].offset(x_begin));
            out[..scratch[c].len()].copy_from_slice(&scratch[c]);
        }
        self.scratch.with(thread, |slot| *slot = scratch);
        Ok(())
    }
    fn get_channel_mode(&self, c: usize) -> Mode {
        if c < 3 { Mode::InOut } else { Mode::Ignored }
    }
    fn get_name(&self) -> &str {
        match self.pass {
            EpfStage::Zero => "EPF0",
            EpfStage::One => "EPF1",
            EpfStage::Two => "EPF2",
        }
    }
}
pub fn get_epf_stage<'a>(
    lf: &LoopFilter,
    sigma: &'a Plane<f32>,
    pass: EpfStage,
) -> Option<Box<dyn RenderPipelineStage + 'a>> {
    if lf.epf_iters == 0 {
        None
    } else if let Some(target) = crate::jxl::hwy::decoder_target() {
        // HWY_DYNAMIC_DISPATCH(GetEPFStage) (port ADR 0004).
        Some(super::stage_epf_simd::get_epf_stage_simd_borrowed(
            lf,
            sigma,
            pass.simd(),
            target,
        ))
    } else {
        Some(Box::new(EPFStage {
            lf: lf.clone(),
            sigma: SigmaOwner::Borrowed(sigma),
            pass,
            scratch: PerThread::new(),
        }))
    }
}

/// Keep the decoder's sigma owner shared with its pipeline, without a self-borrow.
/// The sequential caller releases its mutation borrow before invoking the stage.
pub fn get_epf_stage_shared(
    lf: &LoopFilter,
    sigma: crate::jxl::base::sync_cell::SharedSigma,
    pass: EpfStage,
) -> Option<Box<dyn RenderPipelineStage>> {
    if lf.epf_iters == 0 {
        None
    } else if let Some(target) = crate::jxl::hwy::decoder_target() {
        // HWY_DYNAMIC_DISPATCH(GetEPFStage) (port ADR 0004).
        Some(super::stage_epf_simd::get_epf_stage_simd_shared(
            lf,
            sigma,
            pass.simd(),
            target,
        ))
    } else {
        Some(Box::new(EPFStage {
            lf: lf.clone(),
            sigma: SigmaOwner::Shared(sigma),
            pass,
            scratch: PerThread::new(),
        }))
    }
}
