// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
//! Complete one-lane EPF0/1/2 stages and factory from stage_epf.{h,cc}.
//! Owner-relative row descriptors replace pointers; the sigma plane remains a
//! borrow of the original frame owner. SIMD specializations are separate.
use super::render_pipeline_stage::{
    RenderPipelineChannelMode as Mode, RenderPipelineStage, Row, RowBuffers, RowInfo, Settings,
};
use super::scalar_rows::{self as k, Broadcast, lanes};
use crate::jxl::{
    base::status::Status, dec_cache::SIGMA_PADDING, epf::MIN_SIGMA, image::Plane,
    loop_filter::LoopFilter,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum EpfStage {
    Zero,
    One,
    Two,
}

// Copyright 2019 Google LLC
// Copyright (c) The Highway Project Authors. All rights reserved.
// SPDX-License-Identifier: BSD-3-Clause
// Scalar Add, AbsDiff, MulAdd and operator* from hwy/ops/scalar-inl.h.
// Each scalar multiplication explicitly promotes to double, then narrows.
#[inline(never)]
fn add(a: f32, b: f32) -> f32 {
    a + b
}
#[inline(never)]
fn sub(a: f32, b: f32) -> f32 {
    a - b
}
#[inline(never)]
fn promote(a: f32) -> f64 {
    f64::from(a)
}
#[inline(never)]
fn product(a: f64, b: f64) -> f32 {
    (a * b) as f32
}
fn mul(a: f32, b: f32) -> f32 {
    product(promote(a), promote(b))
}
fn ma(a: f32, b: f32, c: f32) -> f32 {
    add(mul(a, b), c)
}
fn diff(a: f32, b: f32) -> f32 {
    sub(a, b).abs()
}
fn weight(sad: f32, inv_sigma: f32) -> f32 {
    let v = ma(sad, inv_sigma, 1.0);
    if v.is_sign_negative() { 0.0 } else { v }
}

enum SigmaOwner<'a> {
    Borrowed(&'a Plane<f32>),
    Shared(std::rc::Rc<std::cell::RefCell<Plane<f32>>>),
}
pub struct EPFStage<'a> {
    lf: LoopFilter,
    sigma: SigmaOwner<'a>,
    pass: EpfStage,
    /// Output rows of one call, written back after all samples are computed.
    scratch: [Vec<f32>; 3],
    /// Intermediate rows of the row kernels (passes 1 and 2).
    rows: EpfRows,
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
    fn settings(&self) -> Settings {
        Settings::symmetric_border_only(self.border())
    }
    #[expect(
        clippy::manual_is_multiple_of,
        reason = "Mirror the source modulo test."
    )]
    fn process_row(
        &mut self,
        input: &RowInfo,
        output: &RowInfo,
        buffers: &mut RowBuffers<'_>,
        left: usize,
        right: usize,
        width: usize,
        xpos: usize,
        ypos: usize,
        _thread: usize,
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
        let bsm = sm * self.lf.epf_border_sad_mul;
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
        // Resolve every input row span once per call instead of once per tap,
        // and stage outputs in scratch rows. Loaded values, their order and
        // all arithmetic are unchanged; only addressing is hoisted.
        let spans: [[&[f32]; 7]; 3] = std::array::from_fn(|c| {
            std::array::from_fn(|i| buffers.row(Row { x: 0, ..rows[c][i] }))
        });
        let mut scratch = std::mem::take(&mut self.scratch);
        for row in &mut scratch {
            row.clear();
        }
        let x_begin = -(left as isize);
        let x_end = width.wrapping_add(right) as isize;
        if self.pass != EpfStage::Zero {
            let n = x_end.wrapping_sub(x_begin) as usize;
            let tap = |c: usize, y: i32, dx: isize| -> &[f32] {
                let row = rows[c][(border + y) as usize];
                &spans[c][(border + y) as usize][row.x.wrapping_add_signed(x_begin + dx)..]
            };
            let r = &mut self.rows;
            // Per-lane copies of the sigma sample and its scale; no arithmetic.
            let (sigma, scale) = (lanes(&mut r.sigma, n), lanes(&mut r.scale, n));
            r.skip.clear();
            for (i, x) in (x_begin..x_end).enumerate() {
                let bx = (x as usize)
                    .wrapping_add(xpos)
                    .wrapping_add(SIGMA_PADDING * 8)
                    / 8;
                let ix = (x as usize).wrapping_add(xpos) % 8;
                sigma[i] = row_sigma[bx];
                scale[i] = if ypos % 8 == 0 || ypos % 8 == 7 || ix == 0 || ix == 7 {
                    bsm
                } else {
                    sm
                };
                r.skip.push(row_sigma[bx] < MIN_SIGMA);
            }
            let scales = self.lf.epf_channel_scale;
            match self.pass {
                EpfStage::One => r.pass1(n, &tap, scales),
                _ => r.pass2(n, &tap, scales),
            }
            for (c, out) in scratch.iter_mut().enumerate() {
                let center = &tap(c, 0, 0)[..n];
                let computed = &r.accum[c][..n];
                out.extend(
                    r.skip
                        .iter()
                        .zip(center.iter().zip(computed))
                        .map(|(&skip, (&center, &value))| if skip { center } else { value }),
                );
            }
        }
        let per_sample_end = if self.pass == EpfStage::Zero {
            x_end
        } else {
            x_begin
        };
        for x in x_begin..per_sample_end {
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
        self.scratch = scratch;
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
    } else {
        Some(Box::new(EPFStage {
            lf: lf.clone(),
            sigma: SigmaOwner::Borrowed(sigma),
            pass,
            scratch: Default::default(),
            rows: EpfRows::default(),
        }))
    }
}

/// Keep the decoder's sigma owner shared with its pipeline, without a self-borrow.
/// The sequential caller releases its mutation borrow before invoking the stage.
pub fn get_epf_stage_shared(
    lf: &LoopFilter,
    sigma: std::rc::Rc<std::cell::RefCell<Plane<f32>>>,
    pass: EpfStage,
) -> Option<Box<dyn RenderPipelineStage>> {
    if lf.epf_iters == 0 {
        None
    } else {
        Some(Box::new(EPFStage {
            lf: lf.clone(),
            sigma: SigmaOwner::Shared(sigma),
            pass,
            scratch: Default::default(),
            rows: EpfRows::default(),
        }))
    }
}

/// Row form of EPF passes 1 and 2. Every lane performs the per-sample
/// operations of `process_row` (Sub, Abs, Add, promotion and product, in the
/// same order and with the same operands) through the out-of-line kernels of
/// `scalar_rows`; lanes whose sigma is below `MIN_SIGMA` are computed too but
/// replaced by their center sample, as the per-sample loop skips them.
#[derive(Default)]
struct EpfRows {
    sigma: Vec<f32>,
    scale: Vec<f32>,
    skip: Vec<bool>,
    /// Promoted inv_sigma = mul(sigma, scale).
    inv_sigma: Vec<f64>,
    sads: [Vec<f32>; 4],
    sad_c: [Vec<f32>; 4],
    t: Vec<f32>,
    d: Vec<f32>,
    tmp: Vec<f32>,
    prod: Vec<f32>,
    wa: Vec<f64>,
    wb: Vec<f64>,
    weight: Vec<f32>,
    w: Vec<f32>,
    accum: [Vec<f32>; 3],
    /// 0.0, 1.0 and the three channel scales.
    zero: Broadcast,
    one: Broadcast,
    channel_scale: [Broadcast; 3],
}

type Tap<'a> = dyn Fn(usize, i32, isize) -> &'a [f32] + 'a;

/// `out = |a - b|`: the Sub kernel, then the sign-bit clear of Abs.
fn diff_row(out: &mut Vec<f32>, n: usize, a: &[f32], b: &[f32]) {
    let out = lanes(out, n);
    k::sub(out, a, b);
    for v in out {
        *v = v.abs();
    }
}
/// `acc = add(acc, b)`.
fn accumulate(acc: &mut Vec<f32>, tmp: &mut Vec<f32>, n: usize, b: &[f32]) {
    k::add(lanes(tmp, n), &acc[..n], b);
    std::mem::swap(acc, tmp);
}
/// `acc = add(product, acc)`, the addition of `ma(a, b, acc)`.
fn accumulate_after(acc: &mut Vec<f32>, tmp: &mut Vec<f32>, n: usize, product: &[f32]) {
    k::add(lanes(tmp, n), product, &acc[..n]);
    std::mem::swap(acc, tmp);
}
/// `out = mul(a, b)` with `b` already promoted.
fn mul_wide(out: &mut Vec<f32>, wa: &mut Vec<f64>, n: usize, a: &[f32], b: &[f64]) {
    let wa = lanes(wa, n);
    k::promote(wa, a);
    k::product(lanes(out, n), wa, b);
}

impl EpfRows {
    fn begin(&mut self, n: usize, tap: &Tap<'_>) {
        // inv_sigma = mul(sigma, scale), kept promoted for weight().
        let wa = lanes(&mut self.wa, n);
        k::promote(wa, &self.sigma);
        let wb = lanes(&mut self.wb, n);
        k::promote(wb, &self.scale);
        let inv_sigma = lanes(&mut self.prod, n);
        k::product(inv_sigma, wa, wb);
        k::promote(lanes(&mut self.inv_sigma, n), inv_sigma);
        lanes(&mut self.w, n).copy_from_slice(self.one.row(1.0, n));
        for c in 0..3 {
            lanes(&mut self.accum[c], n).copy_from_slice(&tap(c, 0, 0)[..n]);
        }
    }
    /// weight = weight(sad, inv_sigma); w = add(w, weight);
    /// accum[c] = ma(weight, candidate[c], accum[c]).
    fn apply(&mut self, n: usize, sad: &[f32], candidate: [&[f32]; 3]) {
        mul_wide(&mut self.t, &mut self.wa, n, sad, &self.inv_sigma);
        let weight = lanes(&mut self.weight, n);
        k::add(weight, &self.t, self.one.row(1.0, n));
        for v in weight.iter_mut() {
            if v.is_sign_negative() {
                *v = 0.0;
            }
        }
        accumulate(&mut self.w, &mut self.tmp, n, &self.weight);
        k::promote(lanes(&mut self.wa, n), &self.weight);
        for (c, candidate) in candidate.into_iter().enumerate() {
            let wb = lanes(&mut self.wb, n);
            k::promote(wb, candidate);
            k::product(lanes(&mut self.prod, n), &self.wa, wb);
            accumulate_after(&mut self.accum[c], &mut self.tmp, n, &self.prod);
        }
    }
    /// accum[c] = mul(accum[c], 1.0 / w).
    fn finish(&mut self, n: usize) {
        let inv_w = lanes(&mut self.t, n);
        for (inv, &w) in inv_w.iter_mut().zip(&self.w) {
            *inv = 1.0 / w; // JXL_HIGH_PRECISION defaults to 1 in the pinned source.
        }
        let wb = lanes(&mut self.wb, n);
        k::promote(wb, inv_w);
        for c in 0..3 {
            let wa = lanes(&mut self.wa, n);
            k::promote(wa, &self.accum[c]);
            k::product(&mut self.accum[c][..n], wa, wb);
        }
    }
    fn pass1(&mut self, n: usize, tap: &Tap<'_>, scales: [f32; 3]) {
        self.begin(n, tap);
        for sad in &mut self.sads {
            lanes(sad, n).copy_from_slice(self.zero.row(0.0, n));
        }
        for c in 0..3 {
            let p = |dy: i32, dx: isize| &tap(c, dy, dx)[..n];
            let (p20, p21, p11, p31) = (p(-2, 0), p(-1, 0), p(-1, -1), p(-1, 1));
            let (p02, p12, p22, p32, p42) = (p(0, -2), p(0, -1), p(0, 0), p(0, 1), p(0, 2));
            let (p13, p23, p33, p24) = (p(1, -1), p(1, 0), p(1, 1), p(2, 0));
            let [s0, s1, s2, s3] = &mut self.sad_c;
            let (t, d, tmp) = (&mut self.t, &mut self.d, &mut self.tmp);
            diff_row(s0, n, p20, p21);
            diff_row(s1, n, p11, p21);
            diff_row(s2, n, p31, p21);
            diff_row(d, n, p02, p12);
            accumulate(s1, tmp, n, d);
            diff_row(d, n, p11, p12);
            accumulate(s0, tmp, n, d);
            diff_row(t, n, p12, p22);
            accumulate(s1, tmp, n, t);
            accumulate(s2, tmp, n, t);
            diff_row(t, n, p22, p21);
            lanes(s3, n).copy_from_slice(&t[..n]);
            accumulate(s0, tmp, n, t);
            diff_row(d, n, p31, p32);
            accumulate(s0, tmp, n, d);
            diff_row(t, n, p22, p32);
            accumulate(s1, tmp, n, t);
            accumulate(s2, tmp, n, t);
            diff_row(d, n, p42, p32);
            accumulate(s2, tmp, n, d);
            diff_row(d, n, p13, p12);
            accumulate(s3, tmp, n, d);
            diff_row(t, n, p22, p23);
            accumulate(s0, tmp, n, t);
            accumulate(s3, tmp, n, t);
            diff_row(d, n, p13, p23);
            accumulate(s1, tmp, n, d);
            diff_row(d, n, p33, p23);
            accumulate(s2, tmp, n, d);
            diff_row(d, n, p33, p32);
            accumulate(s3, tmp, n, d);
            diff_row(d, n, p24, p23);
            accumulate(s3, tmp, n, d);
            // sads[i] = ma(sad_ic, channel_scale[c], sads[i])
            let scale = self.channel_scale[c].promoted(scales[c], n);
            for i in 0..4 {
                mul_wide(&mut self.prod, &mut self.wa, n, &self.sad_c[i], scale);
                accumulate_after(&mut self.sads[i], &mut self.tmp, n, &self.prod);
            }
        }
        for (i, (dy, dx)) in [(-1, 0), (0, -1), (0, 1), (1, 0)].into_iter().enumerate() {
            let sad = std::mem::take(&mut self.sads[i]);
            self.apply(n, &sad, std::array::from_fn(|c| &tap(c, dy, dx)[..n]));
            self.sads[i] = sad;
        }
        self.finish(n);
    }
    fn pass2(&mut self, n: usize, tap: &Tap<'_>, scales: [f32; 3]) {
        self.begin(n, tap);
        let center: [&[f32]; 3] = std::array::from_fn(|c| &tap(c, 0, 0)[..n]);
        for (dy, dx) in [(-1, 0), (0, -1), (0, 1), (1, 0)] {
            let candidate: [&[f32]; 3] = std::array::from_fn(|c| &tap(c, dy, dx)[..n]);
            // sad = mul(diff(cand0, center0), scale0), then
            // sad = ma(diff(cand_c, center_c), scale_c, sad) for c = 1, 2.
            let mut sad = std::mem::take(&mut self.sads[0]);
            diff_row(&mut self.d, n, candidate[0], center[0]);
            let scale = self.channel_scale[0].promoted(scales[0], n);
            mul_wide(&mut sad, &mut self.wa, n, &self.d, scale);
            for c in 1..3 {
                diff_row(&mut self.d, n, candidate[c], center[c]);
                let scale = self.channel_scale[c].promoted(scales[c], n);
                mul_wide(&mut self.prod, &mut self.wa, n, &self.d, scale);
                accumulate_after(&mut sad, &mut self.tmp, n, &self.prod);
            }
            self.apply(n, &sad, candidate);
            self.sads[0] = sad;
        }
        self.finish(n);
    }
}
