// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
//! The SIMD AdaptiveQuantizationMap of lib/jxl/enc_adaptive_quantization.cc
//! (AdaptiveQuantizationImpl::PrepareBuffers/ComputeTile, ComputeMask,
//! RatioOfDerivativesOfCubicRootToSimpleGamma in its vector and one-lane
//! forms, GammaModulation, BlueModulation, HfModulation,
//! PerBlockModulations, MaskingSqrt in both forms and FuzzyErosion) for the
//! NEON and AVX2 targets (port ADR 0004, "Encoder path"), written once
//! against [`SimdF32`]. FastLog2f and FastPow2f are `simd_transfer`'s.
//! Compare it only with the original's SIMD build at the same target; the
//! scalar translation stays in `enc_adaptive_quantization`, and the encoder
//! reaches this module only through [`crate::jxl::hwy::encoder_target`] at
//! InitialQuantField (`HWY_DYNAMIC_DISPATCH(AdaptiveQuantizationMap)`).
//!
//! `HWY_CAPPED(float, kBlockDim)` and `HWY_FULL(float)` are the full vector
//! on both targets (four NEON, eight AVX2 lanes). The one-lane forms
//! (`HWY_CAPPED(float, 1)`: the scalar `RatioOfDerivativesOfCubicRootToSimpleGamma`,
//! `MaskingSqrt` and `FastPow2f`) use the target's instructions on one lane;
//! every operation they use is lane-wise, so they are lane 0 of the full
//! vector operation on a broadcast (their MulAdd is fused).
//!
//! The whole export lies inside `HWY_NAMESPACE`, so its scalar statements
//! carry FMA on both targets: the five contraction sites of
//! tools/encoder_heuristics_contraction.json (PerBlockModulations' final
//! scale, FuzzyErosion's weights and its four-product sum) are fused here on
//! x86-64 too. Products of two constants are folded before contraction
//! (`kSGVOffset * kInvLog2e + kEpsilon`, `64 * kLimit - scalar_sum`), as in
//! the scalar translation. Blur1x1Masking is shared with the scalar
//! translation: its only contractible product is exact. It calls the
//! scalar Symmetric5 (convolve_symmetric5 is a separate export without a
//! SIMD translation); the original's SIMD Symmetric5 runs the same
//! roundings (unfused border statements, separate vector Mul and Add, four
//! equal weight copies), which the parity test of `mask1x1` checks.
use super::base::data_parallel::{ThreadPool, run_on_pool_exclusive};
use super::base::rect::Rect;
use super::base::status::{Status, StatusCode, StatusOr};
use super::enc_adaptive_quantization::{
    ENC_TILE_DIM_IN_BLOCKS, INV_LOG2E, SG_MUL, SG_RET_MUL, SG_V_OFFSET, blur1x1_masking,
    compute_mask_for_ac_strategy_use, store_min4,
};
use super::enc_params::ENC_TILE_DIM;
use super::frame_dimensions::BLOCK_DIM;
use super::hwy::{Kernel, SimdF32, Target, dispatch};
use super::image::{Image3F, ImageF};
use super::memory_manager_internal::MemoryManagerPtr;
use super::render_pipeline::simd_transfer::{fast_log2f, fast_pow2f};

/// `ComputeMask(d, out_val)`.
#[inline(always)]
#[expect(clippy::excessive_precision, reason = "the source's decimal literals")]
fn compute_mask_d<D: SimdF32>(d: D, out_val: D::V) -> D::V {
    let k_base = d.set(-0.7647f32);
    let k_mul4 = d.set(9.4708735624378946f32);
    let k_mul2 = d.set(17.35036561631863f32);
    let k_offset2 = d.set(302.59587815579727f32);
    let k_mul3 = d.set(6.7943250517376494f32);
    let k_offset3 = d.set(3.7179635626140772f32);
    let k_offset4 = d.mul(d.set(0.25f32), k_offset3);
    let k_mul0 = d.set(0.80061762862741759f32);
    let k1 = d.set(1.0f32);

    // Avoid division by zero.
    let v1 = d.max(d.mul(out_val, k_mul0), d.set(1e-3f32));
    let v2 = d.div(k1, d.add(v1, k_offset2));
    let v3 = d.div(k1, d.mul_add(v1, v1, k_offset3));
    let v4 = d.div(k1, d.mul_add(v1, v1, k_offset4));
    d.add(
        k_base,
        d.mul_add(k_mul4, v4, d.mul_add(k_mul2, v2, d.mul(k_mul3, v3))),
    )
}

/// `RatioOfDerivativesOfCubicRootToSimpleGamma<invert>(d, v)`.
#[inline(always)]
fn ratio_of_derivatives_d<D: SimdF32>(d: D, invert: bool, v: D::V) -> D::V {
    let epsilon = 1e-2f64 as f32;
    let v = d.zero_if_negative(v);
    // Each constant is a product (and sum) of constants, folded unfused.
    let num_mul = d.set(SG_RET_MUL * 3.0 * SG_MUL);
    let v_offset = d.set(SG_V_OFFSET * INV_LOG2E + epsilon);
    let den_mul = d.set(INV_LOG2E * SG_MUL);

    let v2 = d.mul(v, v);

    let num = d.mul_add(num_mul, v2, d.set(epsilon));
    let den = d.mul_add(d.mul(den_mul, v), v2, v_offset);
    if invert {
        d.div(num, den)
    } else {
        d.div(den, num)
    }
}

/// The one-lane `RatioOfDerivativesOfCubicRootToSimpleGamma<invert>(float)`.
#[inline(always)]
fn ratio_of_derivatives_lane<D: SimdF32>(d: D, invert: bool, v: f32) -> f32 {
    d.get_lane(ratio_of_derivatives_d(d, invert, d.set(v)))
}

/// `Load(d, row + offset)` for a vector some of whose lanes the caller masks
/// to zero: lanes past `row` read +0.0 instead of the row padding.
#[inline(always)]
fn load_masked<D: SimdF32>(d: D, row: &[f32], offset: usize) -> D::V {
    if row.len() >= offset + D::LANES {
        d.load(&row[offset..])
    } else {
        let mut lanes = [0.0f32; 16];
        let available = row.len().saturating_sub(offset).min(D::LANES);
        lanes[..available].copy_from_slice(&row[offset..offset + available]);
        d.load(&lanes)
    }
}

/// `GammaModulation(d, x, y, xyb_x, xyb_y, rect, out_val)`.
#[inline(always)]
#[expect(clippy::excessive_precision, reason = "the source's decimal literals")]
fn gamma_modulation_d<D: SimdF32>(
    d: D,
    x: usize,
    y: usize,
    xyb_x: &ImageF,
    xyb_y: &ImageF,
    rect: Rect,
    out_val: D::V,
) -> D::V {
    let bias_value = 0.16f32;
    let mut overall_ratio = d.zero();
    let bias = d.set(bias_value);
    for dy in 0..8 {
        let row_in_x = &xyb_x.row(rect.y0() + y + dy)[rect.x0()..];
        let row_in_y = &xyb_y.row(rect.y0() + y + dy)[rect.x0()..];
        let mut dx = 0;
        while dx < 8 {
            let iny = d.add(d.load(&row_in_y[x + dx..]), bias);
            let inx = d.load(&row_in_x[x + dx..]);

            let r = d.sub(iny, inx);
            let ratio_r = ratio_of_derivatives_d(d, true, r);
            overall_ratio = d.add(overall_ratio, ratio_r);

            let g = d.add(iny, inx);
            let ratio_g = ratio_of_derivatives_d(d, true, g);
            overall_ratio = d.add(overall_ratio, ratio_g);
            dx += D::LANES;
        }
    }
    overall_ratio = d.mul(d.sum_of_lanes(overall_ratio), d.set(0.5f32 / 64.0));
    // ideally -1.0, but likely optimal correction adds some entropy, so slightly
    // less than that.
    let gamma = d.set(0.1005613337192697f32);
    d.mul_add(gamma, fast_log2f(d, overall_ratio), out_val)
}

/// `BlueModulation(d, x, y, planex, planey, planeb, rect, out_val)`.
#[inline(always)]
#[expect(clippy::too_many_arguments, reason = "mirrors the source signature")]
#[expect(clippy::excessive_precision, reason = "the source's decimal literals")]
fn blue_modulation_d<D: SimdF32>(
    d: D,
    x: usize,
    y: usize,
    planex: &ImageF,
    planey: &ImageF,
    planeb: &ImageF,
    rect: Rect,
    out_val: D::V,
) -> D::V {
    let mut sum = d.zero();
    let limit = 0.010474084867598155f64 as f32;
    let offset = 0.0031994768654636393f64 as f32;
    for dy in 0..8 {
        let row_in_x = &planex.row(rect.y0() + y + dy)[rect.x0() + x..];
        let row_in_y = &planey.row(rect.y0() + y + dy)[rect.x0() + x..];
        let row_in_b = &planeb.row(rect.y0() + y + dy)[rect.x0() + x..];
        let mut dx = 0;
        while dx < 8 {
            let p_x = d.load(&row_in_x[dx..]);
            let p_b = d.load(&row_in_b[dx..]);
            let p_y_raw = d.add(d.load(&row_in_y[dx..]), d.set(offset));
            let p_y_effective = d.add(p_y_raw, d.abs(p_x));
            sum = d.add(
                sum,
                d.if_then_else_zero(
                    d.gt(p_b, p_y_effective),
                    d.min(d.sub(p_b, p_y_effective), d.set(limit)),
                ),
            );
            dx += D::LANES;
        }
    }
    let mul = 0.90590804735610064f64 as f32;
    sum = d.sum_of_lanes(sum);
    let mut scalar_sum = d.get_lane(sum);
    // If it is all blue, don't boost the quantization.
    // All blue likely means low frequency blue. Let's not make the most
    // perfect sky ever.
    if scalar_sum >= 32.0 * limit {
        // The product of two constants is folded before contraction.
        scalar_sum = 64.0 * limit - scalar_sum;
    }
    let max_limit = 15.463398341612438f64 as f32;
    if scalar_sum >= max_limit * limit {
        scalar_sum = max_limit * limit;
    }
    scalar_sum *= mul;
    d.add(d.set(scalar_sum), out_val)
}

/// `HfModulation(d, x, y, xyb_y, rect, out_val)`: the SIMD loop over whole
/// vectors, whose last lane's right neighbor (one sample past the block) is
/// masked out by kMaskRight.
#[inline(always)]
fn hf_modulation_d<D: SimdF32>(
    d: D,
    x: usize,
    y: usize,
    xyb_y: &ImageF,
    rect: Rect,
    out_val: D::V,
) -> D::V {
    // Zero out the invalid differences for the rightmost value per row.
    const MASK_RIGHT: [u32; BLOCK_DIM] = [!0, !0, !0, !0, !0, !0, !0, 0];

    // Sums of deltas of y and x components between (approximate)
    // 4-connected pixels.
    let mut sum_y = d.zero();
    let valmin_y = 0.0206f64 as f32;
    let valminv_y = d.set(valmin_y);
    for dy in 0..8 {
        let row_in_y = &xyb_y.row(rect.y0() + y + dy)[rect.x0() + x..];
        let row_in_y_next = if dy == 7 {
            row_in_y
        } else {
            &xyb_y.row(rect.y0() + y + dy + 1)[rect.x0() + x..]
        };

        let mut dx = 0;
        while dx < 8 {
            let mask = d.load_bits(&MASK_RIGHT[dx..]);
            {
                let p_y = d.load(&row_in_y[dx..]);
                // The masked lane's value does not reach the sum (And with
                // a zero mask is +0.0 whatever the value).
                let pr_y = load_masked(d, row_in_y, dx + 1);
                sum_y = d.add(sum_y, d.and(mask, d.min(valminv_y, d.abs_diff(p_y, pr_y))));
                let pd_y = d.load(&row_in_y_next[dx..]);
                sum_y = d.add(sum_y, d.min(valminv_y, d.abs_diff(p_y, pd_y)));
            }
            dx += D::LANES;
        }
    }
    let mul_y = -0.38f64 as f32;
    sum_y = d.sum_of_lanes(sum_y);

    let mut scalar_sum_y = d.get_lane(sum_y);
    scalar_sum_y *= mul_y;

    // higher value -> more bpp
    let offset = 0.42f64 as f32;
    scalar_sum_y += offset;

    d.add(d.set(scalar_sum_y), out_val)
}

/// `PerBlockModulations`.
#[expect(clippy::too_many_arguments, reason = "mirrors the source signature")]
#[expect(
    clippy::approx_constant,
    clippy::excessive_precision,
    reason = "the source's literal"
)]
#[inline(always)]
fn per_block_modulations_d<D: SimdF32>(
    d: D,
    butteraugli_target: f32,
    xyb_x: &ImageF,
    xyb_y: &ImageF,
    xyb_b: &ImageF,
    rect_in: Rect,
    scale: f32,
    rect_out: Rect,
    out: &mut ImageF,
) {
    let base_level = 0.48f32 * scale;
    let dampen_ramp_start = 2.0f32;
    let dampen_ramp_end = 14.0f32;
    let mut dampen = 1.0f32;
    if butteraugli_target >= dampen_ramp_start {
        dampen = 1.0
            - ((butteraugli_target - dampen_ramp_start) / (dampen_ramp_end - dampen_ramp_start));
        if dampen < 0.0 {
            dampen = 0.0;
        }
    }
    let mul = scale * dampen;
    let add = (1.0f32 - dampen) * base_level;
    for iy in rect_out.y0()..rect_out.y1() {
        let y = iy * 8;
        for ix in rect_out.x0()..rect_out.x1() {
            let x = ix * 8;
            let mut mask_val = compute_mask_d(d, d.set(out.row(iy)[ix]));
            mask_val = gamma_modulation_d(d, x, y, xyb_x, xyb_y, rect_in, mask_val);
            let mut out_val = hf_modulation_d(d, x, y, xyb_y, rect_in, mask_val);
            out_val = d.min(
                out_val,
                blue_modulation_d(d, x, y, xyb_x, xyb_y, xyb_b, rect_in, mask_val),
            );
            // We want multiplicative quantization field, so everything
            // until this point has been modulating the exponent.
            // FastPow2f(float) is the one-lane vector form; `* mul + add`
            // is contracted on both targets.
            let pow = d.get_lane(fast_pow2f(d, d.set(d.get_lane(out_val) * 1.442695041f32)));
            out.row_mut(iy)[ix] = pow.mul_add(mul, add);
        }
    }
}

/// `MaskingSqrt(d, v)`.
#[inline(always)]
#[expect(clippy::excessive_precision, reason = "the source's decimal literals")]
fn masking_sqrt_d<D: SimdF32>(d: D, v: D::V) -> D::V {
    let log_offset = 27.505837037000106f32;
    let mul = 211.66567973503678f32;
    // kMul * 1e8 is a double product.
    let mul_v = d.set((f64::from(mul) * 1e8) as f32);
    let offset_v = d.set(log_offset);
    d.mul(
        d.set(0.25f32),
        d.sqrt(d.mul_add(v, d.sqrt(mul_v), offset_v)),
    )
}

/// The one-lane `MaskingSqrt(float)`.
#[inline(always)]
fn masking_sqrt_lane<D: SimdF32>(d: D, v: f32) -> f32 {
    d.get_lane(masking_sqrt_d(d, d.set(v)))
}

/// `FuzzyErosion`, with its contraction sites fused.
#[expect(clippy::excessive_precision, reason = "the source's decimal literals")]
fn fuzzy_erosion_fused(
    butteraugli_target: f32,
    from_rect: Rect,
    from: &ImageF,
    to_rect: Rect,
    to: &mut ImageF,
) -> Status {
    let xsize = from.xsize();
    let ysize = from.ysize();
    const STEP: usize = 1;
    if to_rect.xsize() * 2 != from_rect.xsize() || to_rect.ysize() * 2 != from_rect.ysize() {
        return Err(StatusCode::GenericError);
    }
    const MUL_BASE: [f32; 4] = [
        0.125f64 as f32,
        0.1f64 as f32,
        0.09f64 as f32,
        0.06f64 as f32,
    ];
    const MUL_ADD: [f32; 4] = [
        0.0f64 as f32,
        -0.1f64 as f32,
        -0.09f64 as f32,
        -0.06f64 as f32,
    ];
    let mut mul = 0.0f32;
    if butteraugli_target < 2.0 {
        mul = (2.0 - butteraugli_target) * (1.0f32 / 2.0);
    }
    let mut k_mul = [0.0f32; 4];
    let mut norm_sum = 0.0f32;
    for ii in 0..4 {
        k_mul[ii] = mul.mul_add(MUL_ADD[ii], MUL_BASE[ii]);
        norm_sum += k_mul[ii];
    }
    let total = 0.29959705784054957f64 as f32;
    for k in &mut k_mul {
        *k *= total / norm_sum;
    }
    for fy in 0..from_rect.ysize() {
        let y = fy + from_rect.y0();
        let ym1 = if y >= STEP { y - STEP } else { y };
        let yp1 = if y + STEP < ysize { y + STEP } else { y };
        let rowt = from.row(ym1);
        let row = from.row(y);
        let rowb = from.row(yp1);
        for fx in 0..from_rect.xsize() {
            let x = fx + from_rect.x0();
            let xm1 = if x >= STEP { x - STEP } else { x };
            let xp1 = if x + STEP < xsize { x + STEP } else { x };
            let mut min = [row[x], row[xm1], row[xp1], rowt[xm1]];
            // Sort the first four values.
            if min[0] > min[1] {
                min.swap(0, 1);
            }
            if min[0] > min[2] {
                min.swap(0, 2);
            }
            if min[0] > min[3] {
                min.swap(0, 3);
            }
            if min[1] > min[2] {
                min.swap(1, 2);
            }
            if min[1] > min[3] {
                min.swap(1, 3);
            }
            if min[2] > min[3] {
                min.swap(2, 3);
            }
            // The remaining five values of a 3x3 neighbourhood.
            store_min4(rowt[x], &mut min);
            store_min4(rowt[xp1], &mut min);
            store_min4(rowb[xm1], &mut min);
            store_min4(rowb[x], &mut min);
            store_min4(rowb[xp1], &mut min);

            // The first product is fused into the second's sum, and each
            // later product into the running sum.
            let v = k_mul[3].mul_add(
                min[3],
                k_mul[2].mul_add(min[2], k_mul[0].mul_add(min[0], k_mul[1] * min[1])),
            );
            let row_out = &mut to.row_mut(to_rect.y0() + fy / 2)[to_rect.x0()..];
            if fx % 2 == 0 && fy % 2 == 0 {
                row_out[fx / 2] = v;
            } else {
                row_out[fx / 2] += v;
            }
        }
    }
    Ok(())
}

#[derive(Default)]
struct AdaptiveQuantizationImpl {
    pre_erosion: Vec<ImageF>,
    aq_map: ImageF,
    diff_buffer: ImageF,
}

impl AdaptiveQuantizationImpl {
    fn prepare_buffers(&mut self, memory_manager: MemoryManagerPtr, num_threads: usize) -> Status {
        self.diff_buffer = ImageF::create_in(ENC_TILE_DIM + 8, num_threads, 0, memory_manager)?;
        for _ in self.pre_erosion.len()..num_threads {
            let tmp = ImageF::create_in(
                ENC_TILE_DIM_IN_BLOCKS * 2 + 2,
                ENC_TILE_DIM_IN_BLOCKS * 2 + 2,
                0,
                memory_manager,
            )?;
            self.pre_erosion.push(tmp);
        }
        Ok(())
    }
}

/// `AdaptiveQuantizationImpl::ComputeTile` for one SIMD target.
#[expect(clippy::too_many_arguments, reason = "mirrors the source signature")]
#[expect(clippy::manual_is_multiple_of, reason = "keeps the source's % tests")]
#[inline(always)]
fn compute_tile_d<D: SimdF32>(
    d: D,
    imp: &mut AdaptiveQuantizationImpl,
    butteraugli_target: f32,
    scale: f32,
    xyb: &Image3F,
    rect_in: Rect,
    rect_out: Rect,
    thread: usize,
    mask: &mut ImageF,
    mask1x1: &mut ImageF,
) -> Status {
    if rect_in.x0() % BLOCK_DIM != 0 || rect_in.y0() % BLOCK_DIM != 0 {
        return Err(StatusCode::GenericError);
    }
    let xsize = xyb.xsize();
    let ysize = xyb.ysize();

    // The XYB gamma is 3.0 to be able to decode faster with two muls.
    // Butteraugli's gamma is matching the gamma of human eye, around 2.6.
    // We approximate the gamma difference by adding one cubic root into
    // the adaptive quantization. This gives us a total gamma of 2.6666
    // for quantization uses.
    let match_gamma_offset = 0.019f64 as f32;

    // HWY_FULL(float) df: `d`.
    let mut y_start_1x1 = rect_in.y0() + rect_out.y0() * 8;
    let mut y_end_1x1 = y_start_1x1 + rect_out.ysize() * 8;

    let mut x_start_1x1 = rect_in.x0() + rect_out.x0() * 8;
    let mut x_end_1x1 = x_start_1x1 + rect_out.xsize() * 8;

    if rect_in.x0() != 0 && rect_out.x0() == 0 {
        x_start_1x1 -= 2;
    }
    if rect_in.x1() < xsize && rect_out.x1() * 8 == rect_in.xsize() {
        x_end_1x1 += 2;
    }
    if rect_in.y0() != 0 && rect_out.y0() == 0 {
        y_start_1x1 -= 2;
    }
    if rect_in.y1() < ysize && rect_out.y1() * 8 == rect_in.ysize() {
        y_end_1x1 += 2;
    }

    // Computes image (padded to multiple of 8x8) of local pixel differences.
    // Subsample both directions by 4.
    // 1x1 Laplacian of intensity.
    for y in y_start_1x1..y_end_1x1 {
        let y2 = if y + 1 < ysize { y + 1 } else { y };
        let y1 = if y > 0 { y - 1 } else { y };
        let row_in = xyb.plane_row(1, y);
        let row_in1 = xyb.plane_row(1, y1);
        let row_in2 = xyb.plane_row(1, y2);
        let mask1x1_out = mask1x1.row_mut(y);
        for x in x_start_1x1..x_end_1x1 {
            let x2 = if x + 1 < xsize { x + 1 } else { x };
            let x1 = if x > 0 { x - 1 } else { x };
            let base = 0.25f32 * (row_in2[x] + row_in1[x] + row_in[x1] + row_in[x2]);
            let gammac = ratio_of_derivatives_lane(d, false, row_in[x] + match_gamma_offset);
            let mut diff = (gammac * (row_in[x] - base)).abs();
            const SCALER: f64 = 1.0;
            diff = (f64::from(diff) * SCALER) as f32;
            diff = diff.ln_1p();
            const MUL: f32 = 1.0;
            const OFFSET: f32 = 0.01f64 as f32;
            mask1x1_out[x] = MUL / (diff + OFFSET);
        }
    }

    let mut y_start = rect_in.y0() + rect_out.y0() * 8;
    let mut y_end = y_start + rect_out.ysize() * 8;

    let mut x_start = rect_in.x0() + rect_out.x0() * 8;
    let mut x_end = x_start + rect_out.xsize() * 8;

    if x_start != 0 {
        x_start -= 4;
    }
    if x_end != xsize {
        x_end += 4;
    }
    if y_start != 0 {
        y_start -= 4;
    }
    if y_end != ysize {
        y_end += 4;
    }
    imp.pre_erosion[thread].shrink_to((x_end - x_start) / 4, (y_end - y_start) / 4)?;

    let limit = 0.2f32;
    for y in y_start..y_end {
        let y2 = if y + 1 < ysize { y + 1 } else { y };
        let y1 = if y > 0 { y - 1 } else { y };

        let row_in = xyb.plane_row(1, y);
        let row_in1 = xyb.plane_row(1, y1);
        let row_in2 = xyb.plane_row(1, y2);
        let row_out = imp.diff_buffer.row_mut(thread);

        let scalar_pixel = |row_out: &mut [f32], x: usize| {
            let x2 = if x + 1 < xsize { x + 1 } else { x };
            let x1 = if x > 0 { x - 1 } else { x };
            let base = 0.25f32 * (row_in2[x] + row_in1[x] + row_in[x1] + row_in[x2]);
            let gammac = ratio_of_derivatives_lane(d, false, row_in[x] + match_gamma_offset);
            let mut diff = gammac * (row_in[x] - base);
            diff *= diff;
            if diff >= limit {
                diff = limit;
            }
            diff = masking_sqrt_lane(d, diff);
            if (y % 4) != 0 {
                row_out[x - x_start] += diff;
            } else {
                row_out[x - x_start] = diff;
            }
        };

        let mut x = x_start;
        // First pixel of the row.
        if x_start == 0 {
            scalar_pixel(row_out, x_start);
            x += 1;
        }
        // SIMD
        let match_gamma_offset_v = d.set(match_gamma_offset);
        let quarter = d.set(0.25f32);
        while x + 1 + D::LANES < x_end {
            let input = d.load(&row_in[x..]);
            let in_r = d.load(&row_in[x + 1..]);
            let in_l = d.load(&row_in[x - 1..]);
            let in_t = d.load(&row_in2[x..]);
            let in_b = d.load(&row_in1[x..]);
            let base = d.mul(quarter, d.add(d.add(in_r, in_l), d.add(in_t, in_b)));
            let gammacv = ratio_of_derivatives_d(d, false, d.add(input, match_gamma_offset_v));
            let mut diff = d.mul(gammacv, d.sub(input, base));
            diff = d.mul(diff, diff);
            diff = d.min(diff, d.set(limit));
            diff = masking_sqrt_d(d, diff);
            if (y & 3) != 0 {
                diff = d.add(diff, d.load(&row_out[x - x_start..]));
            }
            d.store(diff, &mut row_out[x - x_start..]);
            x += D::LANES;
        }
        // Scalar
        while x < x_end {
            scalar_pixel(row_out, x);
            x += 1;
        }
        if y % 4 == 3 {
            let row_d_out = imp.pre_erosion[thread].row_mut((y - y_start) / 4);
            let row_out = imp.diff_buffer.row(thread);
            for qx in 0..(x_end - x_start) / 4 {
                row_d_out[qx] = (row_out[qx * 4]
                    + row_out[qx * 4 + 1]
                    + row_out[qx * 4 + 2]
                    + row_out[qx * 4 + 3])
                    * 0.25f32;
            }
        }
    }
    if x_start % (BLOCK_DIM / 2) != 0 || y_start % (BLOCK_DIM / 2) != 0 {
        return Err(StatusCode::GenericError);
    }
    let from_rect = Rect::new(
        if x_start % 8 == 0 { 0 } else { 1 },
        if y_start % 8 == 0 { 0 } else { 1 },
        rect_out.xsize() * 2,
        rect_out.ysize() * 2,
    );
    fuzzy_erosion_fused(
        butteraugli_target,
        from_rect,
        &imp.pre_erosion[thread],
        rect_out,
        &mut imp.aq_map,
    )?;
    for y in 0..rect_out.ysize() {
        let aq_map_row = &imp.aq_map.row(rect_out.y0() + y)[rect_out.x0()..];
        let mask_row = &mut mask.row_mut(rect_out.y0() + y)[rect_out.x0()..];
        for x in 0..rect_out.xsize() {
            mask_row[x] = compute_mask_for_ac_strategy_use(aq_map_row[x]);
        }
    }
    per_block_modulations_d(
        d,
        butteraugli_target,
        xyb.plane(0),
        xyb.plane(1),
        xyb.plane(2),
        rect_in,
        scale,
        rect_out,
        &mut imp.aq_map,
    );
    Ok(())
}

struct Tile<'a> {
    imp: &'a mut AdaptiveQuantizationImpl,
    butteraugli_target: f32,
    scale: f32,
    xyb: &'a Image3F,
    rect_in: Rect,
    rect_out: Rect,
    thread: usize,
    mask: &'a mut ImageF,
    mask1x1: &'a mut ImageF,
}

impl Kernel for Tile<'_> {
    type Output = Status;
    #[inline(always)]
    fn run<D: SimdF32>(self, d: D) -> Status {
        compute_tile_d(
            d,
            self.imp,
            self.butteraugli_target,
            self.scale,
            self.xyb,
            self.rect_in,
            self.rect_out,
            self.thread,
            self.mask,
            self.mask1x1,
        )
    }
}

/// `AdaptiveQuantizationMap` instantiated for one SIMD target: the tiles of
/// the source's RunOnPool, each tile's body in the target's instantiation,
/// then Blur1x1Masking.
#[expect(clippy::too_many_arguments, reason = "mirrors the source signature")]
pub fn adaptive_quantization_map_simd(
    target: Target,
    butteraugli_target: f32,
    xyb: &Image3F,
    rect: Rect,
    scale: f32,
    pool: Option<&ThreadPool>,
    mask: &mut ImageF,
    mask1x1: &mut ImageF,
) -> StatusOr<ImageF> {
    map_tiles(
        butteraugli_target,
        xyb,
        rect,
        scale,
        pool,
        mask,
        mask1x1,
        |tile| dispatch(target, tile),
    )
}

/// `AdaptiveQuantizationMap` with each tile's body instantiated for the
/// descriptor `d` directly (no target dispatch): for the 8-lane structure
/// test, which runs a descriptor that is not a detected target.
#[doc(hidden)]
#[expect(clippy::too_many_arguments, reason = "mirrors the source signature")]
pub fn adaptive_quantization_map_d<D: SimdF32>(
    d: D,
    butteraugli_target: f32,
    xyb: &Image3F,
    rect: Rect,
    scale: f32,
    pool: Option<&ThreadPool>,
    mask: &mut ImageF,
    mask1x1: &mut ImageF,
) -> StatusOr<ImageF> {
    map_tiles(
        butteraugli_target,
        xyb,
        rect,
        scale,
        pool,
        mask,
        mask1x1,
        |tile| tile.run(d),
    )
}

#[expect(clippy::too_many_arguments, reason = "mirrors the source signature")]
#[expect(clippy::manual_is_multiple_of, reason = "keeps the source's % tests")]
fn map_tiles(
    butteraugli_target: f32,
    xyb: &Image3F,
    rect: Rect,
    scale: f32,
    pool: Option<&ThreadPool>,
    mask: &mut ImageF,
    mask1x1: &mut ImageF,
    run_tile: impl Fn(Tile<'_>) -> Status,
) -> StatusOr<ImageF> {
    if rect.xsize() % BLOCK_DIM != 0 || rect.ysize() % BLOCK_DIM != 0 {
        return Err(StatusCode::GenericError);
    }
    let mut imp = AdaptiveQuantizationImpl::default();
    let xsize_blocks = rect.xsize() / BLOCK_DIM;
    let ysize_blocks = rect.ysize() / BLOCK_DIM;
    let memory_manager = xyb.memory_manager();
    imp.aq_map = ImageF::create_in(xsize_blocks, ysize_blocks, 0, memory_manager)?;
    *mask = ImageF::create_in(xsize_blocks, ysize_blocks, 0, memory_manager)?;
    *mask1x1 = ImageF::create_in(xyb.xsize(), xyb.ysize(), 0, memory_manager)?;
    let prepare = |state: &mut (&mut AdaptiveQuantizationImpl, &mut ImageF, &mut ImageF),
                   num_threads: usize|
     -> Status { state.0.prepare_buffers(memory_manager, num_threads) };
    let process_tile = |state: &mut (&mut AdaptiveQuantizationImpl, &mut ImageF, &mut ImageF),
                        tid: u32,
                        thread: usize|
     -> Status {
        let n_enc_tiles = xsize_blocks.div_ceil(ENC_TILE_DIM_IN_BLOCKS);
        let tx = tid as usize % n_enc_tiles;
        let ty = tid as usize / n_enc_tiles;
        let by0 = ty * ENC_TILE_DIM_IN_BLOCKS;
        let by1 = ((ty + 1) * ENC_TILE_DIM_IN_BLOCKS).min(ysize_blocks);
        let bx0 = tx * ENC_TILE_DIM_IN_BLOCKS;
        let bx1 = ((tx + 1) * ENC_TILE_DIM_IN_BLOCKS).min(xsize_blocks);
        let rect_out = Rect::new(bx0, by0, bx1 - bx0, by1 - by0);
        let (imp, mask, mask1x1) = state;
        run_tile(Tile {
            imp,
            butteraugli_target,
            scale,
            xyb,
            rect_in: rect,
            rect_out,
            thread,
            mask,
            mask1x1,
        })
    };
    let num_tiles = xsize_blocks.div_ceil(ENC_TILE_DIM_IN_BLOCKS)
        * ysize_blocks.div_ceil(ENC_TILE_DIM_IN_BLOCKS);
    let mut state = (&mut imp, &mut *mask, &mut *mask1x1);
    // SAFETY: The tasks mutate only the exclusively borrowed implementation
    // state and output planes and read the shared input image; none of them
    // holds a thread-affine value, and nothing else uses these owners until
    // the call returns.
    unsafe { run_on_pool_exclusive(pool, 0, num_tiles as u32, &mut state, prepare, process_tile) }?;

    blur1x1_masking(memory_manager, pool, mask1x1, rect)?;
    Ok(std::mem::take(&mut imp.aq_map))
}
