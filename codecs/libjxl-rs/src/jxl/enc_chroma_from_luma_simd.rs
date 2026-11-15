// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
//! InitDCStorage and ComputeTile (with CFLFunction and FindBestMultiplier)
//! of lib/jxl/enc_chroma_from_luma.cc for the NEON and AVX2 targets (port
//! ADR 0004, "Encoder path"), written once against [`SimdF32`] with
//! `HWY_FULL(float)` (four NEON, eight AVX2 lanes). Inside `HWY_NAMESPACE`
//! the transforms are the in-namespace SIMD TransformFromPixels and
//! DCFromLowestFrequencies (`enc_transforms_simd`). Compare them only with
//! the original's SIMD build at the same target; the scalar (one-lane)
//! translation stays in `enc_chroma_from_luma`.
//!
//! The scalar statements lie inside `HWY_NAMESPACE` and carry FMA on both
//! targets. Of the scalar translation's contraction map, the fast search's
//! denominator `GetLane(SumOfLanes(ca)) + num * distance_mul * 0.5f` is the
//! only product-and-sum in one expression; it is fused here on both targets
//! (`vfmadd231ss` in the AVX2 dispatch object's N_AVX2::FindBestMultiplier).
//! The Newton search's `first_derivative + GetLane(...)` sums a product
//! computed in an earlier statement and is not contracted.
use super::ac_strategy::{
    AcStrategy, AcStrategyImage, AcStrategyType, MAX_BLOCK_DIM, MAX_COEFF_AREA, MAX_COEFF_BLOCKS,
};
use super::base::float::{clamp_f32, max_f32, min_f32};
use super::base::rect::Rect;
use super::base::status::{Status, StatusCode};
use super::chroma_from_luma::{COLOR_TILE_DIM, COLOR_TILE_DIM_IN_BLOCKS, DEFAULT_COLOR_FACTOR};
use super::cms::opsin_params::Y_TO_B_RATIO;
use super::coeff_order_fwd::coefficient_layout;
use super::enc_transforms_simd::{dc_from_lowest_frequencies_d, transform_from_pixels_d};
use super::frame_dimensions::BLOCK_DIM;
use super::hwy::{Kernel, SimdF32, Target, dispatch};
use super::image::{Image3F, ImageF, ImageI, ImageSB};
use super::memory_manager_internal::MemoryManagerPtr;
use super::quant_weights::DequantMatrices;
use super::quantizer::Quantizer;

/// InitDCStorage: four rows of `RoundUpTo(num_blocks, Lanes(df))` samples,
/// the last `Lanes(df)` of each zeroed.
#[inline(always)]
pub fn init_dc_storage_d<D: SimdF32>(
    _d: D,
    memory_manager: MemoryManagerPtr,
    num_blocks: usize,
    dc_values: &mut ImageF,
) -> Status {
    // First row: Y channel
    // Second row: X channel
    // Third row: Y channel
    // Fourth row: B channel
    *dc_values = ImageF::create_in(
        num_blocks.div_ceil(D::LANES) * D::LANES,
        4,
        0,
        memory_manager,
    )?;

    if dc_values.xsize() == 0 {
        return Err(StatusCode::GenericError);
    }
    // Zero-fill the last lanes
    let xsize = dc_values.xsize();
    for y in 0..4 {
        for x in xsize - D::LANES..xsize {
            dc_values.row_mut(y)[x] = 0.0;
        }
    }
    Ok(())
}

struct InitDc<'a> {
    memory_manager: MemoryManagerPtr,
    num_blocks: usize,
    dc_values: &'a mut ImageF,
}
impl Kernel for InitDc<'_> {
    type Output = Status;
    #[inline(always)]
    fn run<D: SimdF32>(self, d: D) -> Status {
        init_dc_storage_d(d, self.memory_manager, self.num_blocks, self.dc_values)
    }
}

/// `InitDCStorage` instantiated for one SIMD target.
pub fn init_dc_storage_simd(
    target: Target,
    memory_manager: MemoryManagerPtr,
    num_blocks: usize,
    dc_values: &mut ImageF,
) -> Status {
    dispatch(
        target,
        InitDc {
            memory_manager,
            num_blocks,
            dc_values,
        },
    )
}

/// CFLFunction: the Newton search's objective derivative.
struct CflFunction<'a, D: SimdF32> {
    d: D,
    values_m: &'a [f32],
    values_s: &'a [f32],
    num: usize,
    base: f32,
    distance_mul: f32,
}

impl<D: SimdF32> CflFunction<'_, D> {
    const COEFF: f32 = 1.0f32 / 3.0;
    const THRES: f32 = 100.0;
    const INV_COLOR_FACTOR: f32 = 1.0f32 / DEFAULT_COLOR_FACTOR as f32;

    // Returns f'(x), where f is 1/3 * sum ((|color residual| + 1)^2-1) +
    // distance_mul * x^2 * num.
    #[inline(always)]
    fn compute(&self, x: f32, eps: f32, fpeps: &mut f32, fmeps: &mut f32) -> f32 {
        let d = self.d;
        let num = self.num as f32;
        let first_derivative = 2.0 * self.distance_mul * num * x;
        let first_derivative_peps = 2.0 * self.distance_mul * num * (x + eps);
        let first_derivative_meps = 2.0 * self.distance_mul * num * (x - eps);

        let inv_color_factor = d.set(Self::INV_COLOR_FACTOR);
        let thres = d.set(Self::THRES);
        let coeffx2 = d.set(Self::COEFF * 2.0);
        let one = d.set(1.0);
        let zero = d.set(0.0);
        let base_v = d.set(self.base);
        let x_v = d.set(x);
        let xpe_v = d.set(x + eps);
        let xme_v = d.set(x - eps);
        let mut fd_v = d.zero();
        let mut fdpe_v = d.zero();
        let mut fdme_v = d.zero();

        let mut i = 0;
        while i < self.num {
            // color residual = ax + b
            let a = d.mul(inv_color_factor, d.load(&self.values_m[i..]));
            let b = d.sub(
                d.mul(base_v, d.load(&self.values_m[i..])),
                d.load(&self.values_s[i..]),
            );
            let v = d.mul_add(a, x_v, b);
            let vpe = d.mul_add(a, xpe_v, b);
            let vme = d.mul_add(a, xme_v, b);
            let av = d.abs(v);
            let avpe = d.abs(vpe);
            let avme = d.abs(vme);
            let acoeffx2 = d.mul(coeffx2, a);
            let mut dv = d.mul(acoeffx2, d.add(av, one));
            let mut dpe = d.mul(acoeffx2, d.add(avpe, one));
            let mut dme = d.mul(acoeffx2, d.add(avme, one));
            dv = d.if_then_else(d.lt(v, zero), d.sub(zero, dv), dv);
            dpe = d.if_then_else(d.lt(vpe, zero), d.sub(zero, dpe), dpe);
            dme = d.if_then_else(d.lt(vme, zero), d.sub(zero, dme), dme);
            let above = d.ge(av, thres);
            // TODO(eustas): use IfThenElseZero
            fd_v = d.add(fd_v, d.if_then_else(above, zero, dv));
            fdpe_v = d.add(fdpe_v, d.if_then_else(above, zero, dpe));
            fdme_v = d.add(fdme_v, d.if_then_else(above, zero, dme));
            i += D::LANES;
        }

        *fpeps = first_derivative_peps + d.reduce_sum(fdpe_v);
        *fmeps = first_derivative_meps + d.reduce_sum(fdme_v);
        first_derivative + d.reduce_sum(fd_v)
    }
}

/// FindBestMultiplier: chroma-from-luma search, values_m will have luma --
/// and values_s chroma. `num` is a multiple of `D::LANES`.
#[inline(always)]
pub fn find_best_multiplier_d<D: SimdF32>(
    d: D,
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
        let inv_color_factor_s = 1.0f32 / DEFAULT_COLOR_FACTOR as f32;
        let mut ca = d.zero();
        let mut cb = d.zero();
        let inv_color_factor = d.set(inv_color_factor_s);
        let base_v = d.set(base);
        let mut i = 0;
        while i < num {
            // color residual = ax + b
            let a = d.mul(inv_color_factor, d.load(&values_m[i..]));
            let b = d.sub(
                d.mul(base_v, d.load(&values_m[i..])),
                d.load(&values_s[i..]),
            );
            ca = d.mul_add(a, a, ca);
            cb = d.mul_add(a, b, cb);
            i += D::LANES;
        }
        // + distance_mul * x^2 * num
        // `GetLane(ca) + num * distance_mul * 0.5f` is contracted inside
        // HWY_NAMESPACE on both targets (vfmadd231ss in N_AVX2).
        x = -d.reduce_sum(cb) / (num as f32 * distance_mul).mul_add(0.5, d.reduce_sum(ca));
    } else {
        let eps = 100.0f32;
        let clamp = 20.0f32;
        let function = CflFunction {
            d,
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

/// The words of ComputeTile's per-thread scratch for `lanes` lanes
/// (`CfLHeuristics::ItemsPerThread` with `MaxVectorSize()` of the target).
pub const fn items_per_thread_for_lanes(lanes: usize) -> usize {
    let dct_scratch_size = 3 * lanes * MAX_BLOCK_DIM;
    let dc_scratch_size = 3 * MAX_COEFF_BLOCKS * MAX_COEFF_BLOCKS;
    MAX_COEFF_AREA * 3        // Blocks
        + COLOR_TILE_DIM * COLOR_TILE_DIM * 4  // AC coeff storage
        + MAX_COEFF_AREA * 2  // Scratch space
        + dct_scratch_size
        + dc_scratch_size
}

/// The inputs of ComputeTile, in the original's order.
pub struct TileArgs<'a> {
    pub opsin: &'a Image3F,
    pub opsin_rect: Rect,
    pub dequant: &'a DequantMatrices,
    pub ac_strategy: Option<&'a AcStrategyImage>,
    pub raw_quant_field: Option<&'a ImageI>,
    pub quantizer: Option<&'a Quantizer>,
    pub rect: Rect,
    pub fast: bool,
    pub use_dct8: bool,
}

/// ComputeTile; the contract of the scalar `compute_tile` in
/// `enc_chroma_from_luma`, with the target's `MaxVectorSize()` in the
/// scratch size.
#[expect(
    clippy::manual_is_multiple_of,
    reason = "the source checks num_ac % Lanes(df)"
)]
#[inline(always)]
pub fn compute_tile_d<D: SimdF32>(
    d: D,
    args: TileArgs<'_>,
    map_x: &mut ImageSB,
    map_b: &mut ImageSB,
    dc_values: &mut ImageF,
    mem: &mut [f32],
) -> Status {
    let TileArgs {
        opsin,
        opsin_rect,
        dequant,
        ac_strategy,
        raw_quant_field,
        quantizer,
        rect,
        fast,
        use_dct8,
    } = args;
    let xsize_blocks = opsin_rect.xsize() / BLOCK_DIM;
    const DISTANCE_MULTIPLIER_AC: f32 = 1e-9;
    // MaxVectorSize() / sizeof(float) of the selected target.
    let dct_scratch_size = 3 * D::LANES * MAX_BLOCK_DIM;

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
            transform_from_pixels_d(
                d,
                acs.strategy(),
                &planes[0][offset..],
                stride,
                block_y,
                scratch_space,
            );
            dc_from_lowest_frequencies_d(d, acs.strategy(), block_y, dc_y, xs, scratch_space);
            transform_from_pixels_d(
                d,
                acs.strategy(),
                &planes[1][offset..],
                stride,
                block_x,
                scratch_space,
            );
            dc_from_lowest_frequencies_d(d, acs.strategy(), block_x, dc_x, xs, scratch_space);
            transform_from_pixels_d(
                d,
                acs.strategy(),
                &planes[2][offset..],
                stride,
                block_b,
                scratch_space,
            );
            dc_from_lowest_frequencies_d(d, acs.strategy(), block_b, dc_b, xs, scratch_space);
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
            let qv = d.set(q);
            let mut i = 0;
            while i < cx * cy * 64 {
                let b_y = d.load(&block_y[i..]);
                let b_x = d.load(&block_x[i..]);
                let b_b = d.load(&block_b[i..]);
                let qqm_x = d.mul(qv, d.load(&qm_x[i..]));
                let qqm_b = d.mul(qv, d.load(&qm_b[i..]));
                d.store(d.mul(b_y, qqm_x), &mut coeffs_yx[num_ac..]);
                d.store(d.mul(b_x, qqm_x), &mut coeffs_x[num_ac..]);
                d.store(d.mul(b_y, qqm_b), &mut coeffs_yb[num_ac..]);
                d.store(d.mul(b_b, qqm_b), &mut coeffs_b[num_ac..]);
                num_ac += D::LANES;
                i += D::LANES;
            }
        }
    }
    if num_ac % D::LANES != 0 {
        return Err(StatusCode::GenericError);
    }
    map_x.row_mut(ty)[tx] = find_best_multiplier_d(
        d,
        coeffs_yx,
        coeffs_x,
        num_ac,
        0.0,
        DISTANCE_MULTIPLIER_AC,
        fast,
    ) as i8;
    map_b.row_mut(ty)[tx] = find_best_multiplier_d(
        d,
        coeffs_yb,
        coeffs_b,
        num_ac,
        Y_TO_B_RATIO,
        DISTANCE_MULTIPLIER_AC,
        fast,
    ) as i8;
    Ok(())
}

struct Tile<'a, 'b> {
    args: TileArgs<'a>,
    map_x: &'b mut ImageSB,
    map_b: &'b mut ImageSB,
    dc_values: &'b mut ImageF,
    mem: &'b mut [f32],
}
impl Kernel for Tile<'_, '_> {
    type Output = Status;
    #[inline(always)]
    fn run<D: SimdF32>(self, d: D) -> Status {
        compute_tile_d(
            d,
            self.args,
            self.map_x,
            self.map_b,
            self.dc_values,
            self.mem,
        )
    }
}

/// `ComputeTile` instantiated for one SIMD target.
pub fn compute_tile_simd(
    target: Target,
    args: TileArgs<'_>,
    map_x: &mut ImageSB,
    map_b: &mut ImageSB,
    dc_values: &mut ImageF,
    mem: &mut [f32],
) -> Status {
    dispatch(
        target,
        Tile {
            args,
            map_x,
            map_b,
            dc_values,
            mem,
        },
    )
}

struct BestMultiplier<'a> {
    values_m: &'a [f32],
    values_s: &'a [f32],
    num: usize,
    base: f32,
    distance_mul: f32,
    fast: bool,
}
impl Kernel for BestMultiplier<'_> {
    type Output = i32;
    #[inline(always)]
    fn run<D: SimdF32>(self, d: D) -> i32 {
        find_best_multiplier_d(
            d,
            self.values_m,
            self.values_s,
            self.num,
            self.base,
            self.distance_mul,
            self.fast,
        )
    }
}

/// `FindBestMultiplier` instantiated for one SIMD target (not an export;
/// for the structural tests).
pub fn find_best_multiplier_simd(
    target: Target,
    values_m: &[f32],
    values_s: &[f32],
    num: usize,
    base: f32,
    distance_mul: f32,
    fast: bool,
) -> i32 {
    dispatch(
        target,
        BestMultiplier {
            values_m,
            values_s,
            num,
            base,
            distance_mul,
            fast,
        },
    )
}
