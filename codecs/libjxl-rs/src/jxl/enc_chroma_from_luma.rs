// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.

//! CfLHeuristics from lib/jxl/enc_chroma_from_luma.{h,cc}: Init with
//! InitDCStorage, PrepareForThreads, ComputeTile, and the one-lane
//! (HWY_SCALAR) CFLFunction and FindBestMultiplier search.
//! ColorCorrelationEncodeDC remains unported.
//!
//! Highway's scalar Mul is the correctly rounded binary32 product, MulAdd
//! adds it in a separate operation, and SumOfLanes of one lane is the lane.
//! The fast search's denominator `GetLane(ca) + num * distance_mul * 0.5f` is
//! the unit's only llvm.fmuladd site: fused on AArch64 and separate on x86-64
//! (tools/encoder_heuristics_contraction.json). `std::round(float)` is
//! `roundf`, as Rust's `f32::round`. The per-thread scratch is the source's
//! AlignedMemory from the caller's memory manager, split as in ComputeTile.
use super::ac_strategy::{
    AcStrategy, AcStrategyImage, AcStrategyType, MAX_BLOCK_DIM, MAX_COEFF_AREA, MAX_COEFF_BLOCKS,
};
use super::base::float::{clamp_f32, max_f32, min_f32, mul_add_f32};
use super::base::rect::Rect;
use super::base::status::{Status, StatusCode};
use super::chroma_from_luma::{
    COLOR_TILE_DIM, COLOR_TILE_DIM_IN_BLOCKS, ColorCorrelationMap, DEFAULT_COLOR_FACTOR,
};
use super::cms::opsin_params::Y_TO_B_RATIO;
use super::coeff_order_fwd::coefficient_layout;
use super::enc_transforms::{dc_from_lowest_frequencies, transform_from_pixels};
use super::frame_dimensions::BLOCK_DIM;
use super::image::{Image3F, ImageF, ImageI, ImageSB};
use super::memory_manager_internal::{AlignedMemory, MemoryManagerPtr};
use super::quant_weights::DequantMatrices;
use super::quantizer::Quantizer;

/// `MaxVectorSize()` of the one-lane HWY_SCALAR target, in bytes.
pub(crate) const MAX_VECTOR_SIZE: usize = 4;
const LANES: usize = MAX_VECTOR_SIZE / size_of::<f32>();

struct CflFunction<'a> {
    values_m: &'a [f32],
    values_s: &'a [f32],
    num: usize,
    base: f32,
    distance_mul: f32,
}

impl CflFunction<'_> {
    const COEFF: f32 = 1.0f32 / 3.0;
    const THRES: f32 = 100.0;
    const INV_COLOR_FACTOR: f32 = 1.0f32 / DEFAULT_COLOR_FACTOR as f32;

    // Returns f'(x), where f is 1/3 * sum ((|color residual| + 1)^2-1) +
    // distance_mul * x^2 * num.
    fn compute(&self, x: f32, eps: f32, fpeps: &mut f32, fmeps: &mut f32) -> f32 {
        let num = self.num as f32;
        let first_derivative = 2.0 * self.distance_mul * num * x;
        let first_derivative_peps = 2.0 * self.distance_mul * num * (x + eps);
        let first_derivative_meps = 2.0 * self.distance_mul * num * (x - eps);

        let inv_color_factor = Self::INV_COLOR_FACTOR;
        let thres = Self::THRES;
        let coeffx2 = Self::COEFF * 2.0;
        let one = 1.0f32;
        let zero = 0.0f32;
        let base_v = self.base;
        let x_v = x;
        let xpe_v = x + eps;
        let xme_v = x - eps;
        let mut fd_v = 0.0f32;
        let mut fdpe_v = 0.0f32;
        let mut fdme_v = 0.0f32;

        for i in (0..self.num).step_by(LANES) {
            // color residual = ax + b
            let a = inv_color_factor * self.values_m[i];
            let b = base_v * self.values_m[i] - self.values_s[i];
            let v = a * x_v + b;
            let vpe = a * xpe_v + b;
            let vme = a * xme_v + b;
            let av = v.abs();
            let avpe = vpe.abs();
            let avme = vme.abs();
            let acoeffx2 = coeffx2 * a;
            let mut d = acoeffx2 * (av + one);
            let mut dpe = acoeffx2 * (avpe + one);
            let mut dme = acoeffx2 * (avme + one);
            d = if v < zero { zero - d } else { d };
            dpe = if vpe < zero { zero - dpe } else { dpe };
            dme = if vme < zero { zero - dme } else { dme };
            let above = av >= thres;
            // TODO(eustas): use IfThenElseZero
            fd_v += if above { zero } else { d };
            fdpe_v += if above { zero } else { dpe };
            fdme_v += if above { zero } else { dme };
        }

        *fpeps = first_derivative_peps + fdpe_v;
        *fmeps = first_derivative_meps + fdme_v;
        first_derivative + fd_v
    }
}

/// Chroma-from-luma search, values_m will have luma -- and values_s chroma.
#[expect(
    clippy::assign_op_pattern,
    reason = "keeps Highway's MulAdd(a, a, ca) form"
)]
pub fn find_best_multiplier(
    values_m: &[f32],
    values_s: &[f32],
    num: usize,
    base: f32,
    distance_mul: f32,
    fast: bool,
) -> i32 {
    if num == 0 {
        return 0;
    }
    let mut x: f32;
    if fast {
        let inv_color_factor = 1.0f32 / DEFAULT_COLOR_FACTOR as f32;
        let mut ca = 0.0f32;
        let mut cb = 0.0f32;
        let base_v = base;
        for i in (0..num).step_by(LANES) {
            // color residual = ax + b
            let a = inv_color_factor * values_m[i];
            let b = base_v * values_m[i] - values_s[i];
            ca = a * a + ca;
            cb = a * b + cb;
        }
        // + distance_mul * x^2 * num
        x = -cb / mul_add_f32(num as f32 * distance_mul, 0.5, ca);
    } else {
        let eps = 100.0f32;
        let clamp = 20.0f32;
        let function = CflFunction {
            values_m,
            values_s,
            num,
            base,
            distance_mul,
        };
        x = 0.0;
        // Up to 20 Newton iterations, with approximate derivatives.
        // Derivatives are approximate due to the high amount of noise in the
        // exact derivatives.
        for _ in 0..20 {
            let mut dfpeps = 0.0f32;
            let mut dfmeps = 0.0f32;
            let d_f = function.compute(x, eps, &mut dfpeps, &mut dfmeps);
            let ddf = (dfpeps - dfmeps) / (2.0 * eps);
            let experimental_insignificant_stabilizer = 0.85f64 as f32;
            let step = d_f / (ddf + experimental_insignificant_stabilizer);
            x -= min_f32(clamp, max_f32(-clamp, step));
            if f64::from(step.abs()) < 3e-3 {
                break;
            }
        }
    }
    // CFL seems to be tricky for larger transforms for HF components
    // close to zero. This heuristic brings the solutions closer to zero
    // and reduces red-green oscillations. A better approach would
    // look into variance of the multiplier within separate (e.g. 8x8)
    // areas and only apply this heuristic where there is a high variance.
    // This would give about 1 % more compression density.
    let towards_zero = 2.6f64 as f32;
    if x >= towards_zero {
        x -= towards_zero;
    } else if x <= -towards_zero {
        x += towards_zero;
    } else {
        x = 0.0;
    }
    clamp_f32(x.round(), -128.0, 127.0) as i32
}

fn init_dc_storage(
    memory_manager: MemoryManagerPtr,
    num_blocks: usize,
    dc_values: &mut ImageF,
) -> Status {
    // First row: Y channel
    // Second row: X channel
    // Third row: Y channel
    // Fourth row: B channel
    *dc_values = ImageF::create_in(num_blocks.div_ceil(LANES) * LANES, 4, 0, memory_manager)?;

    if dc_values.xsize() == 0 {
        return Err(StatusCode::GenericError);
    }
    // Zero-fill the last lanes
    let xsize = dc_values.xsize();
    for y in 0..4 {
        for x in xsize - LANES..xsize {
            dc_values.row_mut(y)[x] = 0.0;
        }
    }
    Ok(())
}

#[expect(clippy::too_many_arguments, reason = "mirrors the source signature")]
#[expect(
    clippy::modulo_one,
    clippy::manual_is_multiple_of,
    reason = "the source checks num_ac % Lanes(df) for one lane"
)]
fn compute_tile(
    opsin: &Image3F,
    opsin_rect: Rect,
    dequant: &DequantMatrices,
    ac_strategy: Option<&AcStrategyImage>,
    raw_quant_field: Option<&ImageI>,
    quantizer: Option<&Quantizer>,
    rect: Rect,
    fast: bool,
    use_dct8: bool,
    map_x: &mut ImageSB,
    map_b: &mut ImageSB,
    dc_values: &mut ImageF,
    mem: &mut [f32],
) -> Status {
    let xsize_blocks = opsin_rect.xsize() / BLOCK_DIM;
    const DISTANCE_MULTIPLIER_AC: f32 = 1e-9;
    let dct_scratch_size = 3 * (MAX_VECTOR_SIZE / size_of::<f32>()) * MAX_BLOCK_DIM;

    let y0 = rect.y0();
    let x0 = rect.x0();
    let x1 = rect.x0() + rect.xsize();
    let y1 = rect.y0() + rect.ysize();

    let ty = y0 / COLOR_TILE_DIM_IN_BLOCKS;
    let tx = x0 / COLOR_TILE_DIM_IN_BLOCKS;

    // All are aligned.
    let (block_y, mem) = mem.split_at_mut(MAX_COEFF_AREA);
    let (block_x, mem) = mem.split_at_mut(MAX_COEFF_AREA);
    let (block_b, mem) = mem.split_at_mut(MAX_COEFF_AREA);
    const TILE_AREA: usize = COLOR_TILE_DIM * COLOR_TILE_DIM;
    let (coeffs_yx, mem) = mem.split_at_mut(TILE_AREA);
    let (coeffs_x, mem) = mem.split_at_mut(TILE_AREA);
    let (coeffs_yb, mem) = mem.split_at_mut(TILE_AREA);
    let (coeffs_b, mem) = mem.split_at_mut(TILE_AREA);
    const DC_SIZE: usize = MAX_COEFF_BLOCKS * MAX_COEFF_BLOCKS;
    let (dc_y, mem) = mem.split_at_mut(DC_SIZE);
    let (dc_x, mem) = mem.split_at_mut(DC_SIZE);
    let (dc_b, scratch_space) = mem.split_at_mut(DC_SIZE);
    if scratch_space.len() != 2 * MAX_COEFF_AREA + dct_scratch_size {
        return Err(StatusCode::GenericError);
    }

    let mut num_ac = 0usize;

    let stride = opsin.plane(0).pixels_per_row() as usize;
    let planes = [
        opsin.plane(1).pixels().unwrap_or(&[]),
        opsin.plane(0).pixels().unwrap_or(&[]),
        opsin.plane(2).pixels().unwrap_or(&[]),
    ];
    for y in y0..y1 {
        let row_offset = (opsin_rect.y0() + y * BLOCK_DIM) * stride + opsin_rect.x0();

        for x in x0..x1 {
            let acs = if use_dct8 {
                AcStrategy::from_raw_strategy(AcStrategyType::DCT)
            } else {
                ac_strategy
                    .expect("a non-DCT8 tile has a strategy image")
                    .const_row(y, 0)
                    .get(x)
            };
            if !acs.is_first_block() {
                continue;
            }
            let xs = acs.covered_blocks_x();
            let offset = row_offset + x * BLOCK_DIM;
            transform_from_pixels(
                acs.strategy(),
                &planes[0][offset..],
                stride,
                block_y,
                scratch_space,
            );
            dc_from_lowest_frequencies(acs.strategy(), block_y, dc_y, xs, scratch_space);
            transform_from_pixels(
                acs.strategy(),
                &planes[1][offset..],
                stride,
                block_x,
                scratch_space,
            );
            dc_from_lowest_frequencies(acs.strategy(), block_x, dc_x, xs, scratch_space);
            transform_from_pixels(
                acs.strategy(),
                &planes[2][offset..],
                stride,
                block_b,
                scratch_space,
            );
            dc_from_lowest_frequencies(acs.strategy(), block_b, dc_b, xs, scratch_space);
            let qm_x = dequant.inv_matrix(acs.strategy(), 0);
            let qm_b = dequant.inv_matrix(acs.strategy(), 2);
            let q_dc_x = if use_dct8 {
                1.0
            } else {
                1.0f32 / quantizer.expect("a quantizer").inv_dc_step(dequant, 0)
            };
            let q_dc_b = if use_dct8 {
                1.0
            } else {
                1.0f32 / quantizer.expect("a quantizer").inv_dc_step(dequant, 2)
            };

            // Copy DCs in dc_values.
            for iy in 0..acs.covered_blocks_y() {
                for ix in 0..xs {
                    let index = (iy + y) * xsize_blocks + ix + x;
                    dc_values.row_mut(0)[index] = dc_y[iy * xs + ix] * q_dc_x;
                    dc_values.row_mut(1)[index] = dc_x[iy * xs + ix] * q_dc_x;
                    dc_values.row_mut(2)[index] = dc_y[iy * xs + ix] * q_dc_b;
                    dc_values.row_mut(3)[index] = dc_b[iy * xs + ix] * q_dc_b;
                }
            }

            // Do not use this block for computing AC CfL.
            if acs.covered_blocks_x() + x0 > x1 || acs.covered_blocks_y() + y0 > y1 {
                continue;
            }

            // Copy AC coefficients in the local block. The order in which
            // coefficients get stored does not matter.
            let mut cx = acs.covered_blocks_x();
            let mut cy = acs.covered_blocks_y();
            coefficient_layout(&mut cy, &mut cx);
            // Zero out LFs. This introduces terms in the optimization loop that
            // don't affect the result, as they are all 0, but allow for simpler
            // SIMDfication.
            for iy in 0..cy {
                for ix in 0..cx {
                    block_y[cx * BLOCK_DIM * iy + ix] = 0.0;
                    block_x[cx * BLOCK_DIM * iy + ix] = 0.0;
                    block_b[cx * BLOCK_DIM * iy + ix] = 0.0;
                }
            }
            // Unclear why this is like it is. (This works slightly better
            // than the previous approach which was also a hack.)
            let qq = match raw_quant_field {
                None => 1.0f32,
                Some(field) => field.row(y)[x] as f32,
            };
            // Experimentally values 128-130 seem best -- I don't know why we
            // need this multiplier.
            let strange_multiplier = 128.0f32;
            let q = if use_dct8 {
                1.0
            } else {
                quantizer.expect("a quantizer").scale() * strange_multiplier * qq
            };
            let qv = q;
            for i in (0..cx * cy * 64).step_by(LANES) {
                let b_y = block_y[i];
                let b_x = block_x[i];
                let b_b = block_b[i];
                let qqm_x = qv * qm_x[i];
                let qqm_b = qv * qm_b[i];
                coeffs_yx[num_ac] = b_y * qqm_x;
                coeffs_x[num_ac] = b_x * qqm_x;
                coeffs_yb[num_ac] = b_y * qqm_b;
                coeffs_b[num_ac] = b_b * qqm_b;
                num_ac += LANES;
            }
        }
    }
    if num_ac % LANES != 0 {
        return Err(StatusCode::GenericError);
    }
    map_x.row_mut(ty)[tx] = find_best_multiplier(
        coeffs_yx,
        coeffs_x,
        num_ac,
        0.0,
        DISTANCE_MULTIPLIER_AC,
        fast,
    ) as i8;
    map_b.row_mut(ty)[tx] = find_best_multiplier(
        coeffs_yb,
        coeffs_b,
        num_ac,
        Y_TO_B_RATIO,
        DISTANCE_MULTIPLIER_AC,
        fast,
    ) as i8;
    Ok(())
}

pub struct CfLHeuristics {
    pub memory_manager: MemoryManagerPtr,
    pub dc_values: ImageF,
    pub(crate) mem: Option<AlignedMemory>,
}

impl CfLHeuristics {
    pub fn new(memory_manager: MemoryManagerPtr) -> Self {
        Self {
            memory_manager,
            dc_values: ImageF::default(),
            mem: None,
        }
    }
    pub fn init(&mut self, rect: Rect) -> Status {
        let xsize_blocks = rect.xsize() / BLOCK_DIM;
        let ysize_blocks = rect.ysize() / BLOCK_DIM;
        init_dc_storage(
            self.memory_manager,
            xsize_blocks * ysize_blocks,
            &mut self.dc_values,
        )
    }
    pub fn prepare_for_threads(&mut self, num_threads: usize) -> Status {
        let mem_bytes = num_threads * Self::items_per_thread() * size_of::<f32>();
        self.mem = Some(AlignedMemory::create(self.memory_manager, mem_bytes, 0)?);
        Ok(())
    }
    /// A null `ac_strategy` selects the DCT8 pass; the second pass supplies
    /// the strategy image, raw quant field and quantizer together.
    #[expect(clippy::too_many_arguments, reason = "mirrors the source signature")]
    pub fn compute_tile(
        &mut self,
        r: Rect,
        opsin: &Image3F,
        opsin_rect: Rect,
        dequant: &DequantMatrices,
        ac_strategy: Option<&AcStrategyImage>,
        raw_quant_field: Option<&ImageI>,
        quantizer: Option<&Quantizer>,
        fast: bool,
        thread: usize,
        cmap: &mut ColorCorrelationMap,
    ) -> Status {
        let use_dct8 = ac_strategy.is_none();
        let items = Self::items_per_thread();
        let mem = self
            .mem
            .as_mut()
            .ok_or(StatusCode::GenericError)?
            .f32s_mut();
        let scratch = &mut mem[thread * items..(thread + 1) * items];
        compute_tile(
            opsin,
            opsin_rect,
            dequant,
            ac_strategy,
            raw_quant_field,
            quantizer,
            r,
            fast,
            use_dct8,
            &mut cmap.ytox_map,
            &mut cmap.ytob_map,
            &mut self.dc_values,
            scratch,
        )
    }
    // Working set is too large for stack; allocate dynamically.
    pub fn items_per_thread() -> usize {
        let dct_scratch_size = 3 * (MAX_VECTOR_SIZE / size_of::<f32>()) * MAX_BLOCK_DIM;
        let dc_scratch_size = 3 * MAX_COEFF_BLOCKS * MAX_COEFF_BLOCKS;
        MAX_COEFF_AREA * 3        // Blocks
            + COLOR_TILE_DIM * COLOR_TILE_DIM * 4  // AC coeff storage
            + MAX_COEFF_AREA * 2  // Scratch space
            + dct_scratch_size
            + dc_scratch_size
    }
}
