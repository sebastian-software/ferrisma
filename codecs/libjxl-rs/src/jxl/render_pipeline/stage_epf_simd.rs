// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
//! The SIMD EPF0Stage/EPF1Stage/EPF2Stage of render_pipeline/stage_epf.cc
//! for the NEON and AVX2 targets (ADR 0004), written once against
//! [`SimdF32`] with the source's `DF = HWY_CAPPED(float, 8)` (four NEON
//! lanes, eight AVX2 lanes). `JXL_HIGH_PRECISION` is 1 in the pinned build,
//! so `inv_w` is `Div(Set(1), w)`, not the vendor-specific
//! `ApproximateReciprocal`. Compare only with the original's SIMD build at
//! the same target.
use super::render_pipeline_stage::{
    RenderPipelineChannelMode as Mode, RenderPipelineStage, RowBuffers, RowInfo, Settings, Span,
};
use super::simd_rows::{vector_span, with_spans};
use crate::jxl::base::status::Status;
use crate::jxl::dec_cache::SIGMA_PADDING;
use crate::jxl::epf::MIN_SIGMA;
use crate::jxl::hwy::{Kernel, SimdF32, Target, dispatch};
use crate::jxl::image::Plane;
use crate::jxl::loop_filter::LoopFilter;

const BLOCK_DIM: usize = 8;

/// The source's three EPF passes.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EpfPass {
    Zero,
    One,
    Two,
}
impl EpfPass {
    fn border(self) -> usize {
        match self {
            Self::Zero => 3,
            Self::One => 2,
            Self::Two => 1,
        }
    }
}

/// The sigma plane of a SIMD EPF stage: its own copy, the caller's plane,
/// or the decoder's shared owner (the source keeps a pointer).
enum SigmaOwner<'a> {
    Owned(Plane<f32>),
    Borrowed(&'a Plane<f32>),
    Shared(std::rc::Rc<std::cell::RefCell<Plane<f32>>>),
}

pub struct EpfStageSimd<'a> {
    target: Target,
    pass: EpfPass,
    lf: LoopFilter,
    sigma: SigmaOwner<'a>,
}

/// Rows of one channel, each starting at sample `x_start - border`.
struct EpfRow<'a> {
    pass: EpfPass,
    lf: &'a LoopFilter,
    /// `rows[c][i]` is the source's `rows[c][i]` (row offset `i - border`);
    /// entries from `2 * border + 1` on are unused and empty.
    rows: [[&'a [f32]; 7]; 3],
    row_sigma: &'a [f32],
    x_start: isize,
    xpos: usize,
    ypos: usize,
    iterations: usize,
    out: [&'a mut [f32]; 3],
}

#[inline(always)]
fn weight<D: SimdF32>(d: D, sad: D::V, inv_sigma: D::V) -> D::V {
    let v = d.mul_add(sad, inv_sigma, d.set(1.0));
    d.zero_if_negative(v)
}

impl<'a> Kernel for EpfRow<'a> {
    type Output = ();
    #[inline(always)]
    fn run<D: SimdF32>(self, df: D) {
        let b = self.pass.border();
        let lanes = D::LANES;
        // Sample `x` (relative to the row) of a row, as a slice from there.
        let origin = self.x_start - b as isize;
        let at = |row: &'a [f32], x: isize| -> &'a [f32] { &row[(x - origin) as usize..] };
        let lf = self.lf;
        let sm = match self.pass {
            EpfPass::Zero => (f64::from(lf.epf_pass0_sigma_scale) * 1.65) as f32,
            EpfPass::One => 1.65f32,
            EpfPass::Two => (f64::from(lf.epf_pass2_sigma_scale) * 1.65) as f32,
        };
        let bsm = sm * lf.epf_border_sad_mul;
        let sad_mul_center = [
            bsm, sm, sm, sm, sm, sm, sm, bsm, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
        ];
        let sad_mul_border = [bsm; 15];
        let sad_mul =
            if self.ypos.is_multiple_of(BLOCK_DIM) || self.ypos % BLOCK_DIM == BLOCK_DIM - 1 {
                &sad_mul_border
            } else {
                &sad_mul_center
            };
        let rows = &self.rows;
        let [out0, out1, out2] = self.out;
        let outs = [out0, out1, out2];
        for i in 0..self.iterations {
            let x = self.x_start + (i * lanes) as isize;
            let o = i * lanes;
            let bx = (x + self.xpos as isize + (SIGMA_PADDING * BLOCK_DIM) as isize) as usize
                / BLOCK_DIM;
            let ix = (x + self.xpos as isize) as usize % BLOCK_DIM;
            if self.row_sigma[bx] < MIN_SIGMA {
                for c in 0..3 {
                    let px = df.load(at(rows[c][b], x));
                    df.store(px, &mut outs[c][o..]);
                }
                continue;
            }
            // Load(df, sad_mul + ix): lanes past the 8-entry source array are
            // never used by an output lane of this block; see the ADR note.
            let vsm = df.load(&sad_mul[ix..]);
            let inv_sigma = df.mul(df.set(self.row_sigma[bx]), vsm);
            let (mut x_acc, mut y_acc, mut b_acc);
            let mut w = df.set(1.0);
            let cc = [0, 1, 2].map(|c| df.load(at(rows[c][b], x)));
            x_acc = cc[0];
            y_acc = cc[1];
            b_acc = cc[2];
            let mut add_pixel = |row: isize, xx: isize, sad: D::V| {
                let r = (b as isize + row) as usize;
                let cx = df.load(at(rows[0][r], xx));
                let cy = df.load(at(rows[1][r], xx));
                let cb = df.load(at(rows[2][r], xx));
                let weight = weight(df, sad, inv_sigma);
                w = df.add(w, weight);
                x_acc = df.mul_add(weight, cx, x_acc);
                y_acc = df.mul_add(weight, cy, y_acc);
                b_acc = df.mul_add(weight, cb, b_acc);
            };
            match self.pass {
                EpfPass::Zero => {
                    const SADS_OFF: [[isize; 2]; 12] = [
                        [-2, 0],
                        [-1, -1],
                        [-1, 0],
                        [-1, 1],
                        [0, -2],
                        [0, -1],
                        [0, 1],
                        [0, 2],
                        [1, -1],
                        [1, 0],
                        [1, 1],
                        [2, 0],
                    ];
                    const PLUS_OFF: [[isize; 2]; 5] = [[0, 0], [-1, 0], [0, -1], [1, 0], [0, 1]];
                    let mut sads = [df.zero(); 12];
                    for (c, rows_c) in rows.iter().enumerate() {
                        let scale = df.set(lf.epf_channel_scale[c]);
                        for (i, sad_off) in SADS_OFF.iter().enumerate() {
                            let mut sad = df.zero();
                            for off in PLUS_OFF {
                                let r11 = df.load(at(rows_c[(3 + off[0]) as usize], x + off[1]));
                                let c11 = df.load(at(
                                    rows_c[(3 + sad_off[0] + off[0]) as usize],
                                    x + sad_off[1] + off[1],
                                ));
                                sad = df.add(sad, df.abs_diff(r11, c11));
                            }
                            sads[i] = df.mul_add(sad, scale, sads[i]);
                        }
                    }
                    for (i, sad_off) in SADS_OFF.iter().enumerate() {
                        add_pixel(sad_off[0], x + sad_off[1], sads[i]);
                    }
                }
                EpfPass::One => {
                    let (mut sad0, mut sad1, mut sad2, mut sad3) =
                        (df.zero(), df.zero(), df.zero(), df.zero());
                    for (c, rows_c) in rows.iter().enumerate() {
                        let p =
                            |dy: isize, dx: isize| df.load(at(rows_c[(2 + dy) as usize], x + dx));
                        let p20 = p(-2, 0);
                        let p21 = p(-1, 0);
                        let mut sad0c = df.abs_diff(p20, p21);
                        let p11 = p(-1, -1);
                        let mut sad1c = df.abs_diff(p11, p21);
                        let p31 = p(-1, 1);
                        let mut sad2c = df.abs_diff(p31, p21);
                        let p02 = p(0, -2);
                        let p12 = p(0, -1);
                        sad1c = df.add(sad1c, df.abs_diff(p02, p12));
                        sad0c = df.add(sad0c, df.abs_diff(p11, p12));
                        let p22 = p(0, 0);
                        let mut t = df.abs_diff(p12, p22);
                        sad1c = df.add(sad1c, t);
                        sad2c = df.add(sad2c, t);
                        t = df.abs_diff(p22, p21);
                        let mut sad3c = t;
                        sad0c = df.add(sad0c, t);
                        let p32 = p(0, 1);
                        sad0c = df.add(sad0c, df.abs_diff(p31, p32));
                        t = df.abs_diff(p22, p32);
                        sad1c = df.add(sad1c, t);
                        sad2c = df.add(sad2c, t);
                        let p42 = p(0, 2);
                        sad2c = df.add(sad2c, df.abs_diff(p42, p32));
                        let p13 = p(1, -1);
                        sad3c = df.add(sad3c, df.abs_diff(p13, p12));
                        let p23 = p(1, 0);
                        t = df.abs_diff(p22, p23);
                        sad0c = df.add(sad0c, t);
                        sad3c = df.add(sad3c, t);
                        sad1c = df.add(sad1c, df.abs_diff(p13, p23));
                        let p33 = p(1, 1);
                        sad2c = df.add(sad2c, df.abs_diff(p33, p23));
                        sad3c = df.add(sad3c, df.abs_diff(p33, p32));
                        let p24 = p(2, 0);
                        sad3c = df.add(sad3c, df.abs_diff(p24, p23));
                        let scale = df.set(lf.epf_channel_scale[c]);
                        sad0 = df.mul_add(sad0c, scale, sad0);
                        sad1 = df.mul_add(sad1c, scale, sad1);
                        sad2 = df.mul_add(sad2c, scale, sad2);
                        sad3 = df.mul_add(sad3c, scale, sad3);
                    }
                    add_pixel(-1, x, sad0);
                    add_pixel(0, x - 1, sad1);
                    add_pixel(0, x + 1, sad2);
                    add_pixel(1, x, sad3);
                }
                EpfPass::Two => {
                    let scales = lf.epf_channel_scale;
                    let mut add_pixel2 = |row: isize, xx: isize| {
                        let r = (1 + row) as usize;
                        let cx = df.load(at(rows[0][r], xx));
                        let cy = df.load(at(rows[1][r], xx));
                        let cb = df.load(at(rows[2][r], xx));
                        let mut sad = df.mul(df.abs_diff(cx, cc[0]), df.set(scales[0]));
                        sad = df.mul_add(df.abs_diff(cy, cc[1]), df.set(scales[1]), sad);
                        sad = df.mul_add(df.abs_diff(cb, cc[2]), df.set(scales[2]), sad);
                        let weight = weight(df, sad, inv_sigma);
                        w = df.add(w, weight);
                        x_acc = df.mul_add(weight, cx, x_acc);
                        y_acc = df.mul_add(weight, cy, y_acc);
                        b_acc = df.mul_add(weight, cb, b_acc);
                    };
                    add_pixel2(-1, x);
                    add_pixel2(0, x - 1);
                    add_pixel2(0, x + 1);
                    add_pixel2(1, x);
                }
            }
            let inv_w = df.div(df.set(1.0), w);
            df.store(df.mul(x_acc, inv_w), &mut outs[0][o..]);
            df.store(df.mul(y_acc, inv_w), &mut outs[1][o..]);
            df.store(df.mul(b_acc, inv_w), &mut outs[2][o..]);
        }
    }
}

impl EpfStageSimd<'_> {
    /// One row with `R = 3 * (2 * border + 1)` input rows. As the source
    /// (stage_epf.cc:105-110 and the loads and stores of the x loop), the
    /// stage loads from the pipeline's input rows and stores to its output
    /// rows in place.
    #[expect(
        clippy::too_many_arguments,
        reason = "The source's ProcessRow locals, passed on to the per-pass row count."
    )]
    fn process<const R: usize>(
        &self,
        input: &RowInfo,
        output: &RowInfo,
        buffers: &mut RowBuffers<'_>,
        x_start: isize,
        iterations: usize,
        xpos: usize,
        ypos: usize,
    ) -> Status {
        let lanes = self.target.lanes();
        let len = iterations * lanes;
        let b = self.pass.border();
        let k = 2 * b + 1;
        debug_assert_eq!(R, 3 * k);
        let reads: [Span; R] = std::array::from_fn(|i| Span {
            row: self.get_input_row(input, i / k, (i % k) as i32 - b as i32),
            from: x_start - b as isize,
            len: len + 2 * b,
        });
        let writes = [0, 1, 2].map(|c| Span {
            row: self.get_output_row(output, c, 0),
            from: x_start,
            len,
        });
        let shared = match &self.sigma {
            SigmaOwner::Shared(owner) => Some(owner.borrow()),
            _ => None,
        };
        let sigma = match &self.sigma {
            SigmaOwner::Owned(plane) => plane,
            SigmaOwner::Borrowed(plane) => *plane,
            SigmaOwner::Shared(_) => shared.as_deref().unwrap(),
        };
        let row_sigma = sigma.row(ypos / BLOCK_DIM + SIGMA_PADDING);
        with_spans(buffers, reads, writes, |spans, out| {
            let rows: [[&[f32]; 7]; 3] = std::array::from_fn(|c| {
                std::array::from_fn(|i| if i < k { spans[c * k + i] } else { &[] })
            });
            dispatch(
                self.target,
                EpfRow {
                    pass: self.pass,
                    lf: &self.lf,
                    rows,
                    row_sigma,
                    x_start,
                    xpos,
                    ypos,
                    iterations,
                    out,
                },
            );
        });
        Ok(())
    }
}

impl RenderPipelineStage for EpfStageSimd<'_> {
    fn settings(&self) -> Settings {
        Settings::symmetric_border_only(self.pass.border())
    }
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
        // DF = HWY_CAPPED(float, 8): the full vector on both targets.
        let lanes = self.target.lanes();
        let (x_start, iterations) = vector_span(left, right, width, lanes);
        match self.pass {
            EpfPass::Zero => {
                self.process::<21>(input, output, buffers, x_start, iterations, xpos, ypos)
            }
            EpfPass::One => {
                self.process::<15>(input, output, buffers, x_start, iterations, xpos, ypos)
            }
            EpfPass::Two => {
                self.process::<9>(input, output, buffers, x_start, iterations, xpos, ypos)
            }
        }
    }
    fn get_channel_mode(&self, c: usize) -> Mode {
        if c < 3 { Mode::InOut } else { Mode::Ignored }
    }
    fn get_name(&self) -> &str {
        match self.pass {
            EpfPass::Zero => "EPF0",
            EpfPass::One => "EPF1",
            EpfPass::Two => "EPF2",
        }
    }
}

/// `GetEPFStage{0,1,2}` instantiated for one SIMD target. The stage keeps
/// its own copy of the sigma plane (the source borrows it).
pub fn get_epf_stage_simd(
    lf: &LoopFilter,
    sigma: Plane<f32>,
    pass: EpfPass,
    target: Target,
) -> Box<dyn RenderPipelineStage> {
    Box::new(EpfStageSimd {
        target,
        pass,
        lf: lf.clone(),
        sigma: SigmaOwner::Owned(sigma),
    })
}

/// [`get_epf_stage_simd`] borrowing the caller's sigma plane, as the source.
pub fn get_epf_stage_simd_borrowed<'a>(
    lf: &LoopFilter,
    sigma: &'a Plane<f32>,
    pass: EpfPass,
    target: Target,
) -> Box<dyn RenderPipelineStage + 'a> {
    Box::new(EpfStageSimd {
        target,
        pass,
        lf: lf.clone(),
        sigma: SigmaOwner::Borrowed(sigma),
    })
}

/// [`get_epf_stage_simd`] sharing the decoder's sigma owner with its
/// pipeline, like the scalar `get_epf_stage_shared`.
pub fn get_epf_stage_simd_shared(
    lf: &LoopFilter,
    sigma: std::rc::Rc<std::cell::RefCell<Plane<f32>>>,
    pass: EpfPass,
    target: Target,
) -> Box<dyn RenderPipelineStage> {
    Box::new(EpfStageSimd {
        target,
        pass,
        lf: lf.clone(),
        sigma: SigmaOwner::Shared(sigma),
    })
}
