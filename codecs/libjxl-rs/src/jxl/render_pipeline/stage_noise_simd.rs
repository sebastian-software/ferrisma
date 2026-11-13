// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
//! The SIMD AddNoiseStage and ConvolveNoiseStage of
//! render_pipeline/stage_noise.cc (with the non-`HWY_SCALAR` StrengthEvalLut,
//! whose two 16-bit lookup tables are read with `TableLookupBytes`) for the
//! NEON and AVX2 targets (ADR 0004), written once against [`SimdF32`].
//! Compare them only with the original's SIMD build at the same target.
//!
//! AddNoise uses `HWY_CAPPED(float, kBlockDim)`, which is the full vector on
//! both targets (4 and 8 lanes). Both stages process whole vectors: AddNoise
//! reads and writes up to `RoundUpTo(xsize, Lanes)`, ConvolveNoise from
//! `-xextra_left` while `x < xsize + xextra_right`, in the row padding.
use super::render_pipeline_stage::{
    RenderPipelineChannelMode as Mode, RenderPipelineStage, RowBuffers, RowInfo, Settings,
};
use super::simd_rows::{read, vector_span, write};
use crate::jxl::base::status::{Status, StatusCode};
use crate::jxl::chroma_from_luma::ColorCorrelation;
use crate::jxl::hwy::{Kernel, SimdF32, Target, dispatch};
use crate::jxl::noise::{NUM_NOISE_POINTS, NoiseParams};
use std::cell::RefCell;
use std::rc::Rc;

/// A vector of the 32-bit word `w` in every lane (`Set(DI(), w)`).
#[inline(always)]
fn set_i32<D: SimdF32>(d: D, w: u32) -> D::I {
    d.bitcast_to_i32(d.load_bits(&[w; 16]))
}

/// The non-`HWY_SCALAR` StrengthEvalLut: `noise_params.lut` as two 16-bit
/// byte tables (low and high halves of each binary32 word, little endian).
struct StrengthEvalLut {
    low16_lut: [u8; 16],
    high16_lut: [u8; 16],
}
impl StrengthEvalLut {
    fn new(noise_params: &NoiseParams) -> Self {
        let mut low16_lut = [0u8; 16];
        let mut high16_lut = [0u8; 16];
        for i in 0..NUM_NOISE_POINTS {
            let lut = noise_params.lut[i].to_bits();
            low16_lut[2 * i] = (lut & 0xFF) as u8;
            low16_lut[2 * i + 1] = ((lut >> 8) & 0xFF) as u8;
            high16_lut[2 * i] = ((lut >> 16) & 0xFF) as u8;
            high16_lut[2 * i + 1] = ((lut >> 24) & 0xFF) as u8;
        }
        Self {
            low16_lut,
            high16_lut,
        }
    }
    /// `LoadDup128(DI8(), table)` as a 32-bit integer vector.
    #[inline(always)]
    fn table<D: SimdF32>(d: D, bytes: &[u8; 16]) -> D::I {
        let words: [f32; 4] = std::array::from_fn(|i| {
            f32::from_bits(u32::from_le_bytes(
                bytes[4 * i..4 * i + 4].try_into().unwrap(),
            ))
        });
        d.bitcast_to_i32(d.load_dup128(&words))
    }
    #[inline(always)]
    fn eval<D: SimdF32>(&self, d: D, vx: D::V) -> D::V {
        const SCALE: usize = NUM_NOISE_POINTS - 2;
        let scaled_vx = d.max(d.zero(), d.mul(vx, d.set(SCALE as f32)));
        let mut floor_x = d.floor(scaled_vx);
        let mut frac_x = d.sub(scaled_vx, floor_x);
        floor_x = d.if_then_else(
            d.ge(scaled_vx, d.set((SCALE + 1) as f32)),
            d.set(SCALE as f32),
            floor_x,
        );
        frac_x = d.if_then_else(
            d.ge(scaled_vx, d.set((SCALE + 1) as f32)),
            d.set(1.0),
            frac_x,
        );
        let floor_x_int = d.convert_to_i32(floor_x);
        // Set each lane's bytes to {0, 0, 2x+1, 2x}.
        let mut floorx_indices_low = d.add_i32(
            d.mul_i32(floor_x_int, set_i32(d, 0x0202)),
            set_i32(d, 0x0100),
        );
        // Set each lane's bytes to {2x+1, 2x, 0, 0}.
        let mut floorx_indices_hi = d.add_i32(
            d.mul_i32(floor_x_int, set_i32(d, 0x0202_0000)),
            set_i32(d, 0x0100_0000),
        );
        // load LUT
        let low16 = Self::table(d, &self.low16_lut);
        let lowm = set_i32(d, 0xFFFF);
        let hi16 = Self::table(d, &self.high16_lut);
        let him = set_i32(d, 0xFFFF_0000);
        let and = |a: D::I, b: D::I| d.and(d.bitcast_to_f32(a), d.bitcast_to_f32(b));
        // low = noise_params.lut[floor_x]
        let low = d.or(
            and(d.table_lookup_bytes(low16, floorx_indices_low), lowm),
            and(d.table_lookup_bytes(hi16, floorx_indices_hi), him),
        );
        // hi = noise_params.lut[floor_x+1]
        floorx_indices_low = d.add_i32(floorx_indices_low, set_i32(d, 0x0202));
        floorx_indices_hi = d.add_i32(floorx_indices_hi, set_i32(d, 0x0202_0000));
        let hi = d.or(
            and(d.table_lookup_bytes(low16, floorx_indices_low), lowm),
            and(d.table_lookup_bytes(hi16, floorx_indices_hi), him),
        );
        d.mul_add(d.sub(hi, low), frac_x, low)
    }
}

/// `NoiseStrength(eval, x)`: `Clamp0ToMax(eval(x), 1)`, i.e.
/// `ZeroIfNegative(Min(eval(x), 1))`.
#[inline(always)]
fn noise_strength<D: SimdF32>(d: D, eval: &StrengthEvalLut, x: D::V) -> D::V {
    let clamped = d.min(eval.eval(d, x), d.set(1.0));
    d.zero_if_negative(clamped)
}

/// AddNoise over `iterations` whole vectors of the in-place `rows` and the
/// random `rnd` rows.
struct AddNoiseRows<'a> {
    rows: [&'a mut [f32]; 3],
    rnd: [&'a [f32]; 3],
    iterations: usize,
    eval: &'a StrengthEvalLut,
    ytox: f32,
    ytob: f32,
}
impl Kernel for AddNoiseRows<'_> {
    type Output = ();
    #[inline(always)]
    fn run<D: SimdF32>(self, d: D) {
        let half = d.set(0.5);
        let norm_const = d.set(0.22);
        let [row_x, row_y, row_b] = self.rows;
        let [row_rnd_r, row_rnd_g, row_rnd_c] = self.rnd;
        for i in 0..self.iterations {
            let x = i * D::LANES;
            let vx = d.load(&row_x[x..]);
            let vy = d.load(&row_y[x..]);
            let in_g = d.sub(vy, vx);
            let in_r = d.add(vy, vx);
            let noise_strength_g = noise_strength(d, self.eval, d.mul(in_g, half));
            let noise_strength_r = noise_strength(d, self.eval, d.mul(in_r, half));
            let addit_rnd_noise_red = d.mul(d.load(&row_rnd_r[x..]), norm_const);
            let addit_rnd_noise_green = d.mul(d.load(&row_rnd_g[x..]), norm_const);
            let addit_rnd_noise_correlated = d.mul(d.load(&row_rnd_c[x..]), norm_const);
            // AddNoiseToRGB
            let k_rg_corr = d.set(0.9921875); // 127/128
            let k_rgn_corr = d.set(0.0078125); // 1/128
            let red_noise = d.mul(
                noise_strength_r,
                d.mul_add(
                    k_rgn_corr,
                    addit_rnd_noise_red,
                    d.mul(k_rg_corr, addit_rnd_noise_correlated),
                ),
            );
            let green_noise = d.mul(
                noise_strength_g,
                d.mul_add(
                    k_rgn_corr,
                    addit_rnd_noise_green,
                    d.mul(k_rg_corr, addit_rnd_noise_correlated),
                ),
            );
            let mut vx = d.load(&row_x[x..]);
            let mut vy = d.load(&row_y[x..]);
            let mut vb = d.load(&row_b[x..]);
            let rg_noise = d.add(red_noise, green_noise);
            vx = d.add(
                d.mul_add(d.set(self.ytox), rg_noise, d.sub(red_noise, green_noise)),
                vx,
            );
            vy = d.add(vy, rg_noise);
            vb = d.mul_add(d.set(self.ytob), rg_noise, vb);
            d.store(vx, &mut row_x[x..]);
            d.store(vy, &mut row_y[x..]);
            d.store(vb, &mut row_b[x..]);
        }
    }
}

enum NoiseOwners<'a> {
    Borrowed(&'a NoiseParams, &'a ColorCorrelation),
    Shared(Rc<RefCell<NoiseParams>>, Rc<RefCell<ColorCorrelation>>),
}
pub struct AddNoiseStageSimd<'a> {
    target: Target,
    owners: NoiseOwners<'a>,
    first_c: usize,
}
impl RenderPipelineStage for AddNoiseStageSimd<'_> {
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
        let shared = match &self.owners {
            NoiseOwners::Borrowed(..) => None,
            NoiseOwners::Shared(params, correlation) => {
                Some((params.borrow(), correlation.borrow()))
            }
        };
        let (noise_params, color_correlation) = match &self.owners {
            NoiseOwners::Borrowed(params, correlation) => (*params, *correlation),
            NoiseOwners::Shared(..) => {
                let (params, correlation) = shared.as_ref().unwrap();
                (&**params, &**correlation)
            }
        };
        if !noise_params.has_any() {
            return Ok(());
        }
        if left != 0 || right != 0 {
            return Err(StatusCode::GenericError);
        }
        let noise_model = StrengthEvalLut::new(noise_params);
        let ytox = color_correlation.y_to_x_ratio(0);
        let ytob = color_correlation.y_to_b_ratio(0);
        let lanes = self.target.lanes();
        let (_, iterations) = vector_span(0, 0, width, lanes);
        let len = iterations * lanes;
        let rows: [_; 3] = std::array::from_fn(|c| self.get_input_row(input, c, 0));
        let rnd: [_; 3] = std::array::from_fn(|c| {
            read(
                buffers,
                self.get_input_row(input, self.first_c + c, 0),
                0,
                len,
            )
        });
        let mut values = rows.map(|row| read(buffers, row, 0, len));
        let [a, b, c] = &mut values;
        dispatch(
            self.target,
            AddNoiseRows {
                rows: [a, b, c],
                rnd: [&rnd[0], &rnd[1], &rnd[2]],
                iterations,
                eval: &noise_model,
                ytox,
                ytob,
            },
        );
        for (row, values) in rows.into_iter().zip(&values) {
            write(buffers, row, 0, values);
        }
        Ok(())
    }
    fn get_channel_mode(&self, c: usize) -> Mode {
        if c >= self.first_c {
            Mode::Input
        } else if c < 3 {
            Mode::InPlace
        } else {
            Mode::Ignored
        }
    }
    fn get_name(&self) -> &str {
        "AddNoise"
    }
}

/// `GetAddNoiseStage` instantiated for one SIMD target.
pub fn get_add_noise_stage_simd<'a>(
    noise_params: &'a NoiseParams,
    color_correlation: &'a ColorCorrelation,
    first_c: usize,
    target: Target,
) -> Box<dyn RenderPipelineStage + 'a> {
    Box::new(AddNoiseStageSimd {
        target,
        owners: NoiseOwners::Borrowed(noise_params, color_correlation),
        first_c,
    })
}

/// `GetAddNoiseStage` with the decoder's shared parameter owners.
pub fn get_add_noise_stage_shared_simd(
    noise_params: Rc<RefCell<NoiseParams>>,
    color_correlation: Rc<RefCell<ColorCorrelation>>,
    first_c: usize,
    target: Target,
) -> Box<dyn RenderPipelineStage> {
    Box::new(AddNoiseStageSimd {
        target,
        owners: NoiseOwners::Shared(noise_params, color_correlation),
        first_c,
    })
}

/// ConvolveNoise of one channel: `rows` are the five input rows from
/// `x_start - 2`; `out` receives `iterations` whole vectors from `x_start`.
struct ConvolveRows<'a> {
    rows: [&'a [f32]; 5],
    iterations: usize,
    out: &'a mut [f32],
}
impl Kernel for ConvolveRows<'_> {
    type Output = ();
    #[inline(always)]
    fn run<D: SimdF32>(self, d: D) {
        let rows = self.rows;
        // `rows[r] + x + i` of the original is `rows[r][2 + x + i]` here.
        let load =
            |r: usize, x: usize, i: isize| d.load(&rows[r][(2 + x).wrapping_add_signed(i)..]);
        for it in 0..self.iterations {
            let x = it * D::LANES;
            let p00 = load(2, x, 0);
            let mut others = d.zero();
            for i in -2..=2 {
                others = d.add(others, load(0, x, i));
                others = d.add(others, load(1, x, i));
                others = d.add(others, load(3, x, i));
                others = d.add(others, load(4, x, i));
            }
            others = d.add(others, load(2, x, -2));
            others = d.add(others, load(2, x, -1));
            others = d.add(others, load(2, x, 1));
            others = d.add(others, load(2, x, 2));
            // 4 * (1 - box kernel)
            let pixels = d.mul_add(others, d.set(0.16), d.mul(p00, d.set(-3.84)));
            d.store(pixels, &mut self.out[x..]);
        }
    }
}

pub struct ConvolveNoiseStageSimd {
    target: Target,
    first_c: usize,
}
impl RenderPipelineStage for ConvolveNoiseStageSimd {
    fn settings(&self) -> Settings {
        Settings::symmetric_border_only(2)
    }
    fn process_row(
        &mut self,
        input: &RowInfo,
        output: &RowInfo,
        buffers: &mut RowBuffers<'_>,
        left: usize,
        right: usize,
        width: usize,
        _x: usize,
        _y: usize,
        _thread: usize,
    ) -> Status {
        // x from -xextra_left in whole vectors while x < xsize + xextra_right
        // (no rounding of the start, unlike the shared vector_span).
        let lanes = self.target.lanes();
        let x_start = -(left as isize);
        let x_end = (width + right) as isize;
        let iterations = ((x_end - x_start).max(0) as usize).div_ceil(lanes);
        let len = iterations * lanes;
        for c in self.first_c..self.first_c + 3 {
            let rows: [Vec<f32>; 5] = std::array::from_fn(|i| {
                read(
                    buffers,
                    self.get_input_row(input, c, i as i32 - 2),
                    x_start - 2,
                    len + 4,
                )
            });
            let mut out = vec![0.0f32; len];
            dispatch(
                self.target,
                ConvolveRows {
                    rows: [&rows[0], &rows[1], &rows[2], &rows[3], &rows[4]],
                    iterations,
                    out: &mut out,
                },
            );
            write(buffers, self.get_output_row(output, c, 0), x_start, &out);
        }
        Ok(())
    }
    fn get_channel_mode(&self, c: usize) -> Mode {
        if c >= self.first_c {
            Mode::InOut
        } else {
            Mode::Ignored
        }
    }
    fn get_name(&self) -> &str {
        "ConvNoise"
    }
}

/// `GetConvolveNoiseStage` instantiated for one SIMD target.
pub fn get_convolve_noise_stage_simd(
    first_c: usize,
    target: Target,
) -> Box<dyn RenderPipelineStage> {
    Box::new(ConvolveNoiseStageSimd { target, first_c })
}
