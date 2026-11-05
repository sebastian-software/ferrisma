// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.

//! The kSquirrel quantization field heuristics of
//! lib/jxl/enc_adaptive_quantization.{h,cc}: InitialQuantDC,
//! InitialQuantField with the one-lane (HWY_SCALAR) AdaptiveQuantizationMap
//! (AdaptiveQuantizationImpl::PrepareBuffers/ComputeTile, ComputeMask,
//! GammaModulation, BlueModulation, HfModulation, PerBlockModulations,
//! MaskingSqrt, StoreMin4, FuzzyErosion, ComputeMaskForAcStrategyUse,
//! RatioOfDerivativesOfCubicRootToSimpleGamma) and Blur1x1Masking, and
//! AdjustQuantField. FindBestQuantizer is a no-op at kSquirrel and faster; its
//! butteraugli loops (FindBestQuantization, FindBestQuantizationMaxError,
//! RoundtripImage, TileDistMap) and the debug heat maps are unported.
//!
//! Highway's scalar Mul is a double product narrowed to binary32, which is the
//! correctly rounded binary32 product; MulAdd, Add, Div and Sqrt are separate
//! binary32 operations, and Min/Max return the other operand for a NaN. Plain
//! binary32 expressions keep the source's operand order. The original objects
//! contract five of them (PerBlockModulations' final scale, FuzzyErosion's
//! weights and its four-product sum, AdjustQuantField's mixer and mix):
//! fused on AArch64 and separate on x86-64 through `mul_add_f32`, as recorded
//! in tools/encoder_heuristics_contraction.json. `std::log1p(float)` and
//! `std::pow(float, float)` call the host's `log1pf`/`powf`, as Rust's
//! `ln_1p`/`powf` do. The source tiles run through RunOnPool with one
//! scratch slot per thread; their order and thread slots are preserved.
use super::ac_strategy::AcStrategyImage;
use super::base::data_parallel::{ThreadPool, run_on_pool_exclusive};
use super::base::fast_math::{fast_log2f, fast_pow2f};
use super::base::float::{max_f32, min_f32, mul_add_f32};
use super::base::hwy_scalar::{max, min, zero_if_negative};
use super::base::rect::Rect;
use super::base::status::{Status, StatusCode, StatusOr};
use super::convolve::WeightsSymmetric5;
use super::convolve_symmetric5::symmetric5;
use super::enc_params::ENC_TILE_DIM;
use super::frame_dimensions::BLOCK_DIM;
use super::image::{Image3F, ImageF};
use super::memory_manager_internal::MemoryManagerPtr;

const ENC_TILE_DIM_IN_BLOCKS: usize = ENC_TILE_DIM / BLOCK_DIM;
// base/common.h: constexpr float kInvLog2e = 0.6931471805599453;
#[expect(clippy::approx_constant, reason = "the source's literal")]
const INV_LOG2E: f32 = 0.6931471805599453f64 as f32;

// Hack for mask estimation. Eventually replace this code with butteraugli's
// masking.
fn compute_mask_for_ac_strategy_use(out_val: f32) -> f32 {
    const MUL: f32 = 1.0;
    const OFFSET: f32 = 0.001;
    MUL / (out_val + OFFSET)
}

#[expect(clippy::excessive_precision, reason = "the source's decimal literals")]
fn compute_mask(out_val: f32) -> f32 {
    let base = -0.7647f32;
    let mul4 = 9.4708735624378946f32;
    let mul2 = 17.35036561631863f32;
    let offset2 = 302.59587815579727f32;
    let mul3 = 6.7943250517376494f32;
    let offset3 = 3.7179635626140772f32;
    let offset4 = 0.25f32 * offset3;
    let mul0 = 0.80061762862741759f32;
    let k1 = 1.0f32;

    // Avoid division by zero.
    let v1 = max(out_val * mul0, 1e-3f32);
    let v2 = k1 / (v1 + offset2);
    let v3 = k1 / (v1 * v1 + offset3);
    let v4 = k1 / (v1 * v1 + offset4);
    // TODO(jyrki):
    // A log or two here could make sense. In butteraugli we have effectively
    // log(log(x + C)) for this kind of use, as a single log is used in
    // saturating visual masking and here the modulation values are exponential,
    // another log would counter that.
    base + (mul4 * v4 + (mul2 * v2 + mul3 * v3))
}

// mul and mul2 represent a scaling difference between jxl and butteraugli.
#[expect(clippy::excessive_precision, reason = "the source's decimal literals")]
const SG_MUL: f32 = 226.77216153508914f32;
#[expect(clippy::excessive_precision, reason = "the source's decimal literals")]
const SG_MUL2: f32 = 1.0f32 / 73.377132366608819f32;
// Includes correction factor for std::log -> log2.
#[expect(clippy::excessive_precision, reason = "the source's decimal literals")]
const SG_RET_MUL: f32 = SG_MUL2 * 18.6580932135f32 * INV_LOG2E;
#[expect(clippy::excessive_precision, reason = "the source's decimal literals")]
const SG_V_OFFSET: f32 = 7.7825991679894591f32;

fn ratio_of_derivatives_of_cubic_root_to_simple_gamma(invert: bool, v: f32) -> f32 {
    // The opsin space in jxl is the cubic root of photons, i.e., v * v * v
    // is related to the number of photons.
    //
    // SimpleGamma(v * v * v) is the psychovisual space in butteraugli.
    // This ratio allows quantization to move from jxl's opsin space to
    // butteraugli's log-gamma space.
    let epsilon = 1e-2f64 as f32;
    let v = zero_if_negative(v);
    let num_mul = SG_RET_MUL * 3.0 * SG_MUL;
    let v_offset = SG_V_OFFSET * INV_LOG2E + epsilon;
    let den_mul = INV_LOG2E * SG_MUL;

    let v2 = v * v;

    let num = num_mul * v2 + epsilon;
    let den = (den_mul * v) * v2 + v_offset;
    if invert { num / den } else { den / num }
}

#[expect(clippy::excessive_precision, reason = "the source's decimal literals")]
fn gamma_modulation(
    x: usize,
    y: usize,
    xyb_x: &ImageF,
    xyb_y: &ImageF,
    rect: Rect,
    out_val: f32,
) -> f32 {
    let bias = 0.16f32;
    let mut overall_ratio = 0.0f32;
    for dy in 0..8 {
        let row_in_x = &xyb_x.row(rect.y0() + y + dy)[rect.x0()..];
        let row_in_y = &xyb_y.row(rect.y0() + y + dy)[rect.x0()..];
        for dx in 0..8 {
            let iny = row_in_y[x + dx] + bias;
            let inx = row_in_x[x + dx];

            let r = iny - inx;
            let ratio_r = ratio_of_derivatives_of_cubic_root_to_simple_gamma(true, r);
            overall_ratio += ratio_r;

            let g = iny + inx;
            let ratio_g = ratio_of_derivatives_of_cubic_root_to_simple_gamma(true, g);
            overall_ratio += ratio_g;
        }
    }
    overall_ratio *= 0.5f32 / 64.0;
    // ideally -1.0, but likely optimal correction adds some entropy, so slightly
    // less than that.
    let gamma = 0.1005613337192697f32;
    gamma * fast_log2f(overall_ratio) + out_val
}

// Change precision in 8x8 blocks that have significant amounts of blue
// content (but are not close to solid blue).
// This is based on the idea that M and L cone activations saturate the
// S (blue) receptors, and the S reception becomes more important when
// both M and L levels are low. In that case M and L receptors may be
// observing S-spectra instead and viewing them with higher spatial
// accuracy, justifying spending more bits here.
#[expect(clippy::excessive_precision, reason = "the source's decimal literals")]
fn blue_modulation(
    x: usize,
    y: usize,
    planex: &ImageF,
    planey: &ImageF,
    planeb: &ImageF,
    rect: Rect,
    out_val: f32,
) -> f32 {
    let mut sum = 0.0f32;
    let limit = 0.010474084867598155f64 as f32;
    let offset = 0.0031994768654636393f64 as f32;
    for dy in 0..8 {
        let row_in_x = &planex.row(rect.y0() + y + dy)[rect.x0() + x..];
        let row_in_y = &planey.row(rect.y0() + y + dy)[rect.x0() + x..];
        let row_in_b = &planeb.row(rect.y0() + y + dy)[rect.x0() + x..];
        for dx in 0..8 {
            let p_x = row_in_x[dx];
            let p_b = row_in_b[dx];
            let p_y_raw = row_in_y[dx] + offset;
            let p_y_effective = p_y_raw + p_x.abs();
            sum += if p_b > p_y_effective {
                min(p_b - p_y_effective, limit)
            } else {
                0.0
            };
        }
    }
    let mul = 0.90590804735610064f64 as f32;
    let mut scalar_sum = sum;
    // If it is all blue, don't boost the quantization.
    // All blue likely means low frequency blue. Let's not make the most
    // perfect sky ever.
    if scalar_sum >= 32.0 * limit {
        scalar_sum = 64.0 * limit - scalar_sum;
    }
    let max_limit = 15.463398341612438f64 as f32;
    if scalar_sum >= max_limit * limit {
        scalar_sum = max_limit * limit;
    }
    scalar_sum *= mul;
    scalar_sum + out_val
}

// Change precision in 8x8 blocks that have high frequency content.
fn hf_modulation(x: usize, y: usize, xyb_y: &ImageF, rect: Rect, out_val: f32) -> f32 {
    // Sums of deltas of y and x components between (approximate)
    // 4-connected pixels.
    let mut sum_y = 0.0f32;
    let valmin_y = 0.0206f64 as f32;
    for dy in 0..8 {
        let row_in_y = &xyb_y.row(rect.y0() + y + dy)[rect.x0() + x..];
        let row_in_y_next = if dy == 7 {
            row_in_y
        } else {
            &xyb_y.row(rect.y0() + y + dy + 1)[rect.x0() + x..]
        };

        // In SCALAR, there is no guarantee of having extra row padding.
        // Hence, we need to ensure we don't access pixels outside the row
        // itself. The kMaskRight lanes of the first seven columns are all set.
        for dx in 0..7 {
            let p_y = row_in_y[dx];
            let pr_y = row_in_y[dx + 1];
            sum_y += min(valmin_y, (p_y - pr_y).abs());
            let pd_y = row_in_y_next[dx];
            sum_y += min(valmin_y, (p_y - pd_y).abs());
        }
        let p_y = row_in_y[7];
        let pd_y = row_in_y_next[7];
        sum_y += min(valmin_y, (p_y - pd_y).abs());
    }
    let mul_y = -0.38f64 as f32;
    let mut scalar_sum_y = sum_y;
    scalar_sum_y *= mul_y;

    // higher value -> more bpp
    let offset = 0.42f64 as f32;
    scalar_sum_y += offset;

    scalar_sum_y + out_val
}

#[expect(clippy::too_many_arguments, reason = "mirrors the source signature")]
#[expect(
    clippy::approx_constant,
    clippy::excessive_precision,
    reason = "the source's literal"
)]
fn per_block_modulations(
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
            let mut mask_val = compute_mask(out.row(iy)[ix]);
            mask_val = gamma_modulation(x, y, xyb_x, xyb_y, rect_in, mask_val);
            let mut out_val = hf_modulation(x, y, xyb_y, rect_in, mask_val);
            out_val = min(
                out_val,
                blue_modulation(x, y, xyb_x, xyb_y, xyb_b, rect_in, mask_val),
            );
            // We want multiplicative quantization field, so everything
            // until this point has been modulating the exponent.
            out.row_mut(iy)[ix] = mul_add_f32(fast_pow2f(out_val * 1.442695041f32), mul, add);
        }
    }
}

#[expect(clippy::excessive_precision, reason = "the source's decimal literals")]
fn masking_sqrt(v: f32) -> f32 {
    let log_offset = 27.505837037000106f32;
    let mul = 211.66567973503678f32;
    let mul_v = (f64::from(mul) * 1e8) as f32;
    let offset_v = log_offset;
    0.25f32 * (v * mul_v.sqrt() + offset_v).sqrt()
}

fn store_min4(v: f32, min: &mut [f32; 4]) {
    if v < min[3] {
        if v < min[0] {
            min[3] = min[2];
            min[2] = min[1];
            min[1] = min[0];
            min[0] = v;
        } else if v < min[1] {
            min[3] = min[2];
            min[2] = min[1];
            min[1] = v;
        } else if v < min[2] {
            min[3] = min[2];
            min[2] = v;
        } else {
            min[3] = v;
        }
    }
}

// Look for smooth areas near the area of degradation.
// If the areas are generally smooth, don't do masking.
// Output is downsampled 2x.
#[expect(clippy::excessive_precision, reason = "the source's decimal literals")]
fn fuzzy_erosion(
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
        k_mul[ii] = mul_add_f32(mul, MUL_ADD[ii], MUL_BASE[ii]);
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

            // The original object fuses the first product and adds each later
            // product in a fused operation.
            let v = mul_add_f32(
                k_mul[3],
                min[3],
                mul_add_f32(
                    k_mul[2],
                    min[2],
                    mul_add_f32(k_mul[0], min[0], k_mul[1] * min[1]),
                ),
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

    #[expect(clippy::too_many_arguments, reason = "mirrors the source signature")]
    #[expect(clippy::manual_is_multiple_of, reason = "keeps the source's % tests")]
    fn compute_tile(
        &mut self,
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
                let gammac = ratio_of_derivatives_of_cubic_root_to_simple_gamma(
                    false,
                    row_in[x] + match_gamma_offset,
                );
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
        self.pre_erosion[thread].shrink_to((x_end - x_start) / 4, (y_end - y_start) / 4)?;

        let limit = 0.2f32;
        for y in y_start..y_end {
            let y2 = if y + 1 < ysize { y + 1 } else { y };
            let y1 = if y > 0 { y - 1 } else { y };

            let row_in = xyb.plane_row(1, y);
            let row_in1 = xyb.plane_row(1, y1);
            let row_in2 = xyb.plane_row(1, y2);
            let row_out = self.diff_buffer.row_mut(thread);

            let scalar_pixel = |row_out: &mut [f32], x: usize| {
                let x2 = if x + 1 < xsize { x + 1 } else { x };
                let x1 = if x > 0 { x - 1 } else { x };
                let base = 0.25f32 * (row_in2[x] + row_in1[x] + row_in[x1] + row_in[x2]);
                let gammac = ratio_of_derivatives_of_cubic_root_to_simple_gamma(
                    false,
                    row_in[x] + match_gamma_offset,
                );
                let mut diff = gammac * (row_in[x] - base);
                diff *= diff;
                if diff >= limit {
                    diff = limit;
                }
                diff = masking_sqrt(diff);
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
            // The one-lane "SIMD" loop, with its own operation order.
            let quarter = 0.25f32;
            while x + 1 + 1 < x_end {
                let input = row_in[x];
                let in_r = row_in[x + 1];
                let in_l = row_in[x - 1];
                let in_t = row_in2[x];
                let in_b = row_in1[x];
                let base = quarter * ((in_r + in_l) + (in_t + in_b));
                let gammacv = ratio_of_derivatives_of_cubic_root_to_simple_gamma(
                    false,
                    input + match_gamma_offset,
                );
                let mut diff = gammacv * (input - base);
                diff = diff * diff;
                diff = min(diff, limit);
                diff = masking_sqrt(diff);
                if (y & 3) != 0 {
                    diff += row_out[x - x_start];
                }
                row_out[x - x_start] = diff;
                x += 1;
            }
            // Scalar
            while x < x_end {
                scalar_pixel(row_out, x);
                x += 1;
            }
            if y % 4 == 3 {
                let row_d_out = self.pre_erosion[thread].row_mut((y - y_start) / 4);
                let row_out = self.diff_buffer.row(thread);
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
        fuzzy_erosion(
            butteraugli_target,
            from_rect,
            &self.pre_erosion[thread],
            rect_out,
            &mut self.aq_map,
        )?;
        for y in 0..rect_out.ysize() {
            let aq_map_row = &self.aq_map.row(rect_out.y0() + y)[rect_out.x0()..];
            let mask_row = &mut mask.row_mut(rect_out.y0() + y)[rect_out.x0()..];
            for x in 0..rect_out.xsize() {
                mask_row[x] = compute_mask_for_ac_strategy_use(aq_map_row[x]);
            }
        }
        per_block_modulations(
            butteraugli_target,
            xyb.plane(0),
            xyb.plane(1),
            xyb.plane(2),
            rect_in,
            scale,
            rect_out,
            &mut self.aq_map,
        );
        Ok(())
    }
}

fn blur1x1_masking(
    memory_manager: MemoryManagerPtr,
    pool: Option<&ThreadPool>,
    mask1x1: &mut ImageF,
    rect: Rect,
) -> Status {
    // Blur the mask1x1 to obtain the masking image.
    // Before blurring it contains an image of absolute value of the
    // Laplacian of the intensity channel.
    const FILTER_MASK1X1: [f32; 5] = [
        0.364911248f64 as f32,
        0.05f64 as f32,
        0.1688888021f64 as f32,
        0.221069183f64 as f32,
        0.306563504f64 as f32,
    ];
    // The front end folds the binary32 constant sum; only 1.0 + ... is double.
    let mut sum = 1.0
        + f64::from(
            4.0 * (FILTER_MASK1X1[0]
                + FILTER_MASK1X1[1]
                + FILTER_MASK1X1[2]
                + FILTER_MASK1X1[4]
                + 2.0 * FILTER_MASK1X1[3]),
        );
    if sum < 1e-5 {
        sum = 1e-5;
    }
    let normalize = (1.0 / sum) as f32;
    let weights = WeightsSymmetric5 {
        c: [normalize; 4],
        r: [normalize * FILTER_MASK1X1[0]; 4],
        R: [normalize * FILTER_MASK1X1[2]; 4],
        d: [normalize * FILTER_MASK1X1[1]; 4],
        D: [normalize * FILTER_MASK1X1[4]; 4],
        L: [normalize * FILTER_MASK1X1[3]; 4],
    };
    let mut temp = ImageF::create_in(rect.xsize(), rect.ysize(), 0, memory_manager)?;
    let temp_rect = Rect::new(0, 0, temp.xsize(), temp.ysize());
    symmetric5(mask1x1, rect, &weights, pool, &mut temp, temp_rect)?;
    *mask1x1 = temp;
    Ok(())
}

#[expect(clippy::manual_is_multiple_of, reason = "keeps the source's % tests")]
fn adaptive_quantization_map(
    butteraugli_target: f32,
    xyb: &Image3F,
    rect: Rect,
    scale: f32,
    pool: Option<&ThreadPool>,
    mask: &mut ImageF,
    mask1x1: &mut ImageF,
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
        imp.compute_tile(
            butteraugli_target,
            scale,
            xyb,
            rect,
            rect_out,
            thread,
            mask,
            mask1x1,
        )
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

/// AdjustQuantField: replace the whole quant_field in non-8x8 blocks with
/// the maximum of each 8x8 block, mixed with the mean at larger distances.
pub fn adjust_quant_field(
    ac_strategy: &AcStrategyImage,
    rect: Rect,
    butteraugli_target: f32,
    quant_field: &mut ImageF,
) -> Status {
    // Replace the whole quant_field in non-8x8 blocks with the maximum of each
    // 8x8 block.
    let stride = quant_field.pixels_per_row() as usize;

    // At low distances it is great to use max, but mean works better
    // at high distances. We interpolate between them for a distance
    // range.
    let mut mean_max_mixer = 1.0f32;
    {
        let limit = 1.54138f32;
        let mul = 0.56391f32;
        let min = 0.0f32;
        if butteraugli_target > limit {
            mean_max_mixer = mul_add_f32(-(butteraugli_target - limit), mul, mean_max_mixer);
            if mean_max_mixer < min {
                mean_max_mixer = min;
            }
        }
    }
    let xsize = quant_field.xsize();
    let ysize = quant_field.ysize();
    for y in 0..rect.ysize() {
        let ac_strategy_row = ac_strategy.const_rect_row(rect, y);
        let row_offset = (rect.y0() + y) * stride + rect.x0();
        let pixels = quant_field
            .pixels_mut()
            .expect("an allocated quant field has storage");
        let quant_row = &mut pixels[row_offset..];
        for x in 0..rect.xsize() {
            let acs = ac_strategy_row.get(x);
            if !acs.is_first_block() {
                continue;
            }
            if x + acs.covered_blocks_x() > xsize || y + acs.covered_blocks_y() > ysize {
                return Err(StatusCode::GenericError);
            }
            let mut max = quant_row[x];
            let mut mean = 0.0f32;
            for iy in 0..acs.covered_blocks_y() {
                for ix in 0..acs.covered_blocks_x() {
                    mean += quant_row[x + ix + iy * stride];
                    max = max_f32(quant_row[x + ix + iy * stride], max);
                }
            }
            mean /= (acs.covered_blocks_y() * acs.covered_blocks_x()) as f32;
            if acs.covered_blocks_y() * acs.covered_blocks_x() >= 4 {
                max *= mean_max_mixer;
                max = mul_add_f32(1.0 - mean_max_mixer, mean, max);
            }
            for iy in 0..acs.covered_blocks_y() {
                for ix in 0..acs.covered_blocks_x() {
                    quant_row[x + ix + iy * stride] = max;
                }
            }
        }
    }
    Ok(())
}

const DC_QUANT_POW: f32 = 0.83;
#[expect(clippy::excessive_precision, reason = "the source's decimal literals")]
const DC_QUANT: f32 = 1.095924047623553;
const AC_QUANT: f32 = 0.765;

pub fn initial_quant_dc(butteraugli_target: f32) -> f32 {
    // Butteraugli target where non-linearity kicks in.
    let dc_mul = 0.3f64 as f32;
    let butteraugli_target_dc = max_f32(
        0.5f32 * butteraugli_target,
        min_f32(
            butteraugli_target,
            dc_mul * ((1.0f32 / dc_mul) * butteraugli_target).powf(DC_QUANT_POW),
        ),
    );
    // We want the maximum DC value to be at most 2**15 * kInvDCQuant / quant_dc.
    // The maximum DC value might not be in the kXybRange because of inverse
    // gaborish, so we add some slack to the maximum theoretical quant obtained
    // this way (64).
    min_f32(DC_QUANT / butteraugli_target_dc, 50.0)
}

/// InitialQuantField: the adaptive quantization map with its block masking
/// field `mask` and pixel masking field `mask1x1`.
pub fn initial_quant_field(
    butteraugli_target: f32,
    opsin: &Image3F,
    rect: Rect,
    pool: Option<&ThreadPool>,
    rescale: f32,
    mask: &mut ImageF,
    mask1x1: &mut ImageF,
) -> StatusOr<ImageF> {
    let quant_ac = AC_QUANT / butteraugli_target;
    adaptive_quantization_map(
        butteraugli_target,
        opsin,
        rect,
        quant_ac * rescale,
        pool,
        mask,
        mask1x1,
    )
}
