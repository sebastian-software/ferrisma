// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.

//! QuantizeBlockAC, AdjustQuantBlockAC, QuantizeRoundtripYBlockAC,
//! ComputeCoefficients and EncodeGroupTokenizedCoefficients from
//! lib/jxl/enc_group.cc, for the one-lane (HWY_SCALAR) target that the
//! reference configuration builds.
//!
//! Language-required deviations:
//! - The Quantizer keeps no pointer to its DequantMatrices (ADR 0003), so the
//!   functions that read matrices take the shared owner as an argument.
//! - Row pointers become slices with explicit strides: the opsin image is
//!   read through its whole-plane sample slice, and the per-pass coefficient
//!   cursors are offsets into the group rows of the ACImage.
//! - ComputeCoefficients' AlignedMemory scratch owners are vectors of the same
//!   sizes; MaxVectorSize() is that of the scalar target (16 bytes).
//!
//! Contraction map (front-end `llvm.fmuladd` sites, fused on AArch64 and
//! separate on x86-64): AdjustQuantBlockAC's `thresholds[i] += 0.01 *
//! activity` (double) and QuantizeBlockAC's `thresholds[i] -= 0.00744f *
//! xsize * ysize`. AdjustQuantBlockAC's `kMul1 * xsize * ysize * kBlockDim *
//! kBlockDim + kMul2 * sum_of_vals` also forms a site, but its fused factor is
//! the exact multiplication by kBlockDim, so both forms agree. Highway's
//! scalar NegMulAdd/Mul (vector operator functions) are never fused.

use super::ac_strategy::{AcStrategyType, MAX_BLOCK_DIM, MAX_COEFF_AREA};
use super::base::float::{mul_add_f32, mul_add_f64};
use super::base::hwy_scalar;
use super::base::rect::Rect;
use super::base::status::{Status, StatusCode};
use super::chroma_from_luma::COLOR_TILE_DIM_IN_BLOCKS;
use super::coeff_order_fwd::coefficient_layout;
use super::common::{MAX_NUM_PASSES, SpeedTier};
use super::dct_util::{AcPtrMut, AcType};
use super::enc_ans::write_tokens_with_allotment;
use super::enc_aux_out::{AuxOut, LayerType};
use super::enc_bit_writer::BitWriter;
use super::enc_cache::PassesEncoderState;
use super::enc_transforms::{dc_from_lowest_frequencies, transform_from_pixels};
use super::frame_dimensions::{BLOCK_DIM, DCT_BLOCK_SIZE};
use super::image::Image3;
use super::quant_weights::DequantMatrices;
use super::quantizer::{DEFAULT_QUANT_BIAS, Quantizer};
use super::quantizer_inl::adjust_quant_bias;

/// QuantizeBlockAC. The caller extracts `quant` from the raw quant field.
/// `block_in` and `block_out` hold `xsize * ysize * 64` samples.
#[allow(clippy::too_many_arguments)] // Preserve the original function boundary.
pub fn quantize_block_ac(
    quantizer: &Quantizer,
    matrices: &DequantMatrices,
    _error_diffusion: bool,
    c: usize,
    qm_multiplier: f32,
    quant_kind: AcStrategyType,
    xsize: usize,
    ysize: usize,
    thresholds: &mut [f32; 4],
    block_in: &[f32],
    quant: i32,
    block_out: &mut [i32],
) {
    let qm = quantizer.inv_dequant_matrix(matrices, quant_kind, c);
    let qac = quantizer.scale() * quant as f32;
    // Not SIMD-ified for now.
    if c != 1 && xsize * ysize >= 4 {
        for threshold in thresholds.iter_mut() {
            // `thresholds[i] -= 0.00744f * xsize * ysize` is one contracted
            // expression: fmuladd(-(0.00744f * xsize), ysize, thresholds[i]).
            *threshold = mul_add_f32(-(0.00744f32 * xsize as f32), ysize as f32, *threshold);
            if f64::from(*threshold) < 0.5 {
                *threshold = 0.5;
            }
        }
    }
    let quantv = qac * qm_multiplier;
    for y in 0..ysize * BLOCK_DIM {
        let yfix = usize::from(y >= ysize * BLOCK_DIM / 2) * 2;
        let off = y * BLOCK_DIM * xsize;
        // One lane: HWY_CAPPED(float, kBlockDim) has Lanes() == 1.
        for x in 0..xsize * BLOCK_DIM {
            let threshold = if xsize == 1 {
                const MASK: [bool; BLOCK_DIM] =
                    [false, false, false, false, true, true, true, true];
                if MASK[x] {
                    thresholds[yfix + 1]
                } else {
                    thresholds[yfix]
                }
            } else {
                // Same for all lanes in the vector.
                thresholds[yfix + usize::from(x >= xsize * BLOCK_DIM / 2)]
            };
            let q = qm[off + x] * quantv;
            let input = block_in[off + x];
            let val = q * input;
            let nzero_mask = val.abs() >= threshold;
            let v = hwy_scalar::to_i32(if nzero_mask {
                hwy_scalar::round(val)
            } else {
                0.0
            });
            block_out[off + x] = v;
        }
    }
}

/// AdjustQuantBlockAC. `block_in` holds `xsize * ysize * 64` samples.
#[allow(clippy::too_many_arguments)] // Preserve the original function boundary.
pub fn adjust_quant_block_ac(
    quantizer: &Quantizer,
    matrices: &DequantMatrices,
    c: usize,
    qm_multiplier: f32,
    quant_kind: AcStrategyType,
    xsize: usize,
    ysize: usize,
    thresholds: &mut [f32; 4],
    block_in: &[f32],
    quant: &mut i32,
) {
    // No quantization adjusting for these small blocks.
    // Quantization adjusting attempts to fix some known issues
    // with larger blocks and on the 8x8 dct's emerging 8x8 blockiness
    // when there are not many non-zeros.
    const PARTIAL_BLOCK_KINDS: usize = (1 << AcStrategyType::IDENTITY.0)
        | (1 << AcStrategyType::DCT2X2.0)
        | (1 << AcStrategyType::DCT4X4.0)
        | (1 << AcStrategyType::DCT4X8.0)
        | (1 << AcStrategyType::DCT8X4.0)
        | (1 << AcStrategyType::AFV0.0)
        | (1 << AcStrategyType::AFV1.0)
        | (1 << AcStrategyType::AFV2.0)
        | (1 << AcStrategyType::AFV3.0);
    if (1usize << quant_kind.0) & PARTIAL_BLOCK_KINDS != 0 {
        return;
    }

    let qm = quantizer.inv_dequant_matrix(matrices, quant_kind, c);
    let qac = quantizer.scale() * *quant as f32;
    if xsize > 1 || ysize > 1 {
        for threshold in thresholds.iter_mut() {
            // Clamp1 is a function call boundary: no contraction.
            let t = 0.003f32 * xsize as f32 * ysize as f32;
            *threshold -= if t < 0.0 {
                0.0
            } else if t > 0.08 {
                0.08
            } else {
                t
            };
            if f64::from(*threshold) < 0.54 {
                *threshold = 0.54;
            }
        }
    }
    let mut sum_of_highest_freq_row_and_column = 0.0f32;
    let mut sum_of_error = 0.0f32;
    let mut sum_of_vals = 0.0f32;
    let mut hf_non_zeros = [0.0f32; 4];
    let mut hf_max_error = [0.0f32; 4];

    for y in 0..ysize * BLOCK_DIM {
        for x in 0..xsize * BLOCK_DIM {
            let pos = y * BLOCK_DIM * xsize + x;
            if x < xsize && y < ysize {
                continue;
            }
            let hfix = usize::from(y >= ysize * BLOCK_DIM / 2) * 2
                + usize::from(x >= xsize * BLOCK_DIM / 2);
            let val = block_in[pos] * (qm[pos] * qac * qm_multiplier);
            // rintf: round to nearest even in the default rounding mode.
            let v = if val.abs() < thresholds[hfix] {
                0.0
            } else {
                val.round_ties_even()
            };
            let error = (val - v).abs();
            sum_of_error += error;
            sum_of_vals += v.abs();
            if c == 1 && v == 0.0 && hf_max_error[hfix] < error {
                hf_max_error[hfix] = error;
            }
            if v != 0.0 {
                hf_non_zeros[hfix] += v.abs();
                let in_corner = y >= 7 * ysize && x >= 7 * xsize;
                let on_border = y == ysize * BLOCK_DIM - 1 || x == xsize * BLOCK_DIM - 1;
                let in_larger_corner = x >= 4 * xsize && y >= 4 * ysize;
                if in_corner || (on_border && in_larger_corner) {
                    sum_of_highest_freq_row_and_column += val.abs();
                }
            }
        }
    }
    if c == 1 && sum_of_vals * 8.0 < (xsize * ysize) as f32 {
        const LIMIT: [f64; 4] = [0.46, 0.46, 0.46, 0.46];
        const MUL: [f64; 4] = [0.9999, 0.9999, 0.9999, 0.9999];
        let orig_quant = *quant;
        let mut new_quant = *quant;
        for i in 1..4 {
            if f64::from(hf_non_zeros[i]) == 0.0 && f64::from(hf_max_error[i]) > LIMIT[i] {
                new_quant = orig_quant.wrapping_add(1);
                break;
            }
        }
        *quant = new_quant;
        if f64::from(hf_non_zeros[3]) == 0.0 && f64::from(hf_max_error[3]) > LIMIT[3] {
            thresholds[3] = (MUL[3] * f64::from(hf_max_error[3]) * f64::from(new_quant)
                / f64::from(orig_quant)) as f32;
        } else if (f64::from(hf_non_zeros[1]) == 0.0 && f64::from(hf_max_error[1]) > LIMIT[1])
            || (f64::from(hf_non_zeros[2]) == 0.0 && f64::from(hf_max_error[2]) > LIMIT[2])
        {
            // std::max keeps its first argument on equality and NaN.
            let max = if hf_max_error[1] < hf_max_error[2] {
                hf_max_error[2]
            } else {
                hf_max_error[1]
            };
            thresholds[1] =
                (MUL[1] * f64::from(max) * f64::from(new_quant) / f64::from(orig_quant)) as f32;
            thresholds[2] = thresholds[1];
        } else if f64::from(hf_non_zeros[0]) == 0.0 && f64::from(hf_max_error[0]) > LIMIT[0] {
            thresholds[0] = (MUL[0] * f64::from(hf_max_error[0]) * f64::from(new_quant)
                / f64::from(orig_quant)) as f32;
        }
    }
    // Heuristic for improving accuracy of high-frequency patterns
    // occurring in an environment with no medium-frequency masking
    // patterns.
    {
        let all = hf_non_zeros[0] + hf_non_zeros[1] + hf_non_zeros[2] + hf_non_zeros[3] + 1.0;
        let mul = [70.0f32, 30.0, 60.0];
        if mul[c] * sum_of_highest_freq_row_and_column >= all {
            // int += float: the sum is a float narrowed toward zero.
            *quant =
                float_to_int(*quant as f32 + mul[c] * sum_of_highest_freq_row_and_column / all);
            if *quant >= Quantizer::QUANT_MAX {
                *quant = Quantizer::QUANT_MAX - 1;
            }
        }
    }
    if quant_kind == AcStrategyType::DCT {
        // If this 8x8 block is too flat, increase the adaptive quantization level
        // a bit to reduce visible block boundaries and requantize the block.
        if hf_non_zeros[0] + hf_non_zeros[1] + hf_non_zeros[2] + hf_non_zeros[3] < 11.0 {
            *quant = quant.wrapping_add(1);
            if *quant >= Quantizer::QUANT_MAX {
                *quant = Quantizer::QUANT_MAX - 1;
            }
        }
    }
    {
        const MUL1: [[f64; 3]; 4] = [
            [
                0.22080615753848404,
                0.45797479824262011,
                0.29859235095977965,
            ],
            [
                0.70109486510286834,
                0.16185281305512639,
                0.14387691730035473,
            ],
            [
                0.114985964456218638,
                0.44656840441027695,
                0.10587658215149048,
            ],
            [
                0.46849665264409396,
                0.41239077937781954,
                0.088667407767185444,
            ],
        ];
        const MUL2: [[f64; 3]; 4] = [
            [0.27450281941822197, 1.1255766549984996, 0.98950459134128388],
            [0.4652168675598285, 0.40945807983455818, 0.36581899811751367],
            [0.28034972424715715, 0.9182653201929738, 1.5581531543057416],
            [0.26873118114033728, 0.68863712390392484, 1.2082185408666786],
        ];
        const QUANT_NORMALIZER: f64 = 2.2942708343284721;
        sum_of_error = (f64::from(sum_of_error) * QUANT_NORMALIZER) as f32;
        sum_of_vals = (f64::from(sum_of_vals) * QUANT_NORMALIZER) as f32;
        if quant_kind.0 >= AcStrategyType::DCT16X16.0 {
            let ix = if quant_kind == AcStrategyType::DCT32X16
                || quant_kind == AcStrategyType::DCT16X32
            {
                1
            } else if quant_kind == AcStrategyType::DCT16X16 {
                0
            } else if quant_kind == AcStrategyType::DCT32X32 {
                2
            } else {
                3
            };
            // The left product ends with an exact multiplication by
            // kBlockDim, so its contraction does not change the sum.
            let limit = |c: usize| {
                let left = MUL1[ix][c] * xsize as f64 * ysize as f64 * BLOCK_DIM as f64;
                mul_add_f64(left, BLOCK_DIM as f64, MUL2[ix][c] * f64::from(sum_of_vals))
            };
            let mut step = float64_to_int(f64::from(sum_of_error) / limit(c));
            if step >= 2 {
                step = 2;
            }
            if step < 0 {
                step = 0;
            }
            if f64::from(sum_of_error) > limit(c) {
                *quant = quant.wrapping_add(step);
                if *quant >= Quantizer::QUANT_MAX {
                    *quant = Quantizer::QUANT_MAX - 1;
                }
            }
        }
    }
    {
        // Reduce quant in highly active areas.
        let div = (xsize * ysize) as i32;
        let min_hf_non_zeros = std_min4(hf_non_zeros);
        let mut activity = 15i32;
        if min_hf_non_zeros < 15.0f32 * div as f32 {
            activity = (float_to_int(min_hf_non_zeros).wrapping_add(div / 2)) / div;
        }
        let mut qp = quant.wrapping_sub(activity);
        if c == 1 {
            for threshold in thresholds.iter_mut().skip(1) {
                // `thresholds[i] += 0.01 * activity` (double, contracted).
                *threshold = mul_add_f64(0.01, f64::from(activity), f64::from(*threshold)) as f32;
            }
        }
        let orig_qp_limit = 4.max(*quant / 2);
        if qp < orig_qp_limit {
            qp = orig_qp_limit;
        }
        *quant = qp;
    }
}

/// std::min({a, b, c, d}) with libc++'s initializer-list min_element: the
/// first smallest element under `<`.
fn std_min4(v: [f32; 4]) -> f32 {
    let mut result = v[0];
    for &x in &v[1..] {
        if x < result {
            result = x;
        }
    }
    result
}

/// The float-to-int conversion of C++ assignment; the reached values are in
/// range (the source behavior is undefined otherwise).
fn float_to_int(v: f32) -> i32 {
    debug_assert!(v.is_finite() && v > -2147483904.0 && v < 2147483648.0);
    v as i32
}

fn float64_to_int(v: f64) -> i32 {
    debug_assert!(v.is_nan() || (v > -2147483649.0 && v < 2147483648.0));
    v as i32
}

/// QuantizeRoundtripYBlockAC. `inout` and `quantized` hold the three
/// channels, `size` samples apart.
#[allow(clippy::too_many_arguments)] // Preserve the original function boundary.
pub fn quantize_roundtrip_y_block_ac(
    enc_state: &PassesEncoderState<'_>,
    size: usize,
    quantizer: &Quantizer,
    matrices: &DequantMatrices,
    error_diffusion: bool,
    quant_kind: AcStrategyType,
    xsize: usize,
    ysize: usize,
    biases: &[f32; 4],
    quant: &mut i32,
    inout: &mut [f32],
    quantized: &mut [i32],
) {
    let mut thres_y = [0.58f32, 0.64, 0.64, 0.64];
    if enc_state.cparams.speed_tier <= SpeedTier::Hare {
        let mut max_quant = 0i32;
        let quant_orig = *quant;
        let val = [enc_state.x_qm_multiplier, 1.0f32, enc_state.b_qm_multiplier];
        for c in [1usize, 0, 2] {
            let mut thres = [0.58f32, 0.64, 0.64, 0.64];
            *quant = quant_orig;
            adjust_quant_block_ac(
                quantizer,
                matrices,
                c,
                val[c],
                quant_kind,
                xsize,
                ysize,
                &mut thres,
                &inout[c * size..],
                quant,
            );
            // Dead zone adjustment
            if c == 1 {
                thres_y = thres;
            }
            max_quant = (*quant).max(max_quant);
        }
        *quant = max_quant;
    } else {
        thres_y = [0.56, 0.62, 0.62, 0.62];
    }

    {
        let (_, rest) = quantized.split_at_mut(size);
        quantize_block_ac(
            quantizer,
            matrices,
            error_diffusion,
            1,
            1.0,
            quant_kind,
            xsize,
            ysize,
            &mut thres_y,
            &inout[size..],
            *quant,
            rest,
        );
    }

    let dequant_matrix = quantizer.dequant_matrix(matrices, quant_kind, 1);
    let inv_qac = quantizer.inv_quant_ac(*quant);
    for k in 0..DCT_BLOCK_SIZE * xsize * ysize {
        let oquant = quantized[size + k];
        let adj_quant = adjust_quant_bias(1, oquant, biases);
        let dequantm = dequant_matrix[k];
        inout[size + k] = (adj_quant * dequantm) * inv_qac;
    }
}

/// ComputeCoefficients for one group. `opsin` is read in `rect`'s
/// coordinates; `dc` receives the group's DC values.
pub fn compute_coefficients(
    group_idx: usize,
    enc_state: &mut PassesEncoderState<'_>,
    opsin: &Image3<f32>,
    rect: Rect,
    dc: &mut Image3<f32>,
) -> Status {
    let frame_dim = enc_state.shared.frame_dim;
    let block_group_rect = frame_dim.block_group_rect(group_idx);
    let cmap_rect = Rect::new(
        block_group_rect.x0() / COLOR_TILE_DIM_IN_BLOCKS,
        block_group_rect.y0() / COLOR_TILE_DIM_IN_BLOCKS,
        block_group_rect.xsize().div_ceil(COLOR_TILE_DIM_IN_BLOCKS),
        block_group_rect.ysize().div_ceil(COLOR_TILE_DIM_IN_BLOCKS),
    );
    let group_rect = frame_dim
        .group_rect(group_idx)
        .translate(rect.x0() as i64, rect.y0() as i64);

    let xsize_blocks = block_group_rect.xsize();
    let ysize_blocks = block_group_rect.ysize();

    let dc_stride = dc.pixels_per_row() as usize;
    let opsin_stride = opsin.pixels_per_row() as usize;

    // MaxVectorSize() / sizeof(float) of the scalar target is 4.
    let dct_scratch_size = 3 * 4 * MAX_BLOCK_DIM;

    // TODO(veluca): consider strategies to reduce this memory.
    let mut mem = vec![0i32; 3 * MAX_COEFF_AREA];
    let mut fmem = vec![0f32; 5 * MAX_COEFF_AREA + dct_scratch_size];
    let (coeffs_in, scratch_space) = fmem.split_at_mut(3 * MAX_COEFF_AREA);
    let quantized = &mut mem[..];
    {
        // Only use error diffusion in Squirrel mode or slower.
        let error_diffusion = enc_state.cparams.speed_tier <= SpeedTier::Squirrel;

        let num_passes = enc_state.progressive_splitter.get_num_passes();
        if num_passes == 0 {
            return Err(StatusCode::GenericError);
        }
        // Offsets into each pass's group row replace the coeffs[c][i] cursors.
        let mut coeffs = [[0usize; MAX_NUM_PASSES]; 3];
        for i in 0..num_passes {
            // TODO(veluca): 16-bit quantized coeffs are not implemented yet.
            let image = enc_state.coeffs[i]
                .as_deref()
                .ok_or(StatusCode::GenericError)?;
            if image.image_type() != AcType::I32 {
                return Err(StatusCode::GenericError);
            }
            for c in 0..3 {
                coeffs[c][i] = 0;
            }
        }

        for by in 0..ysize_blocks {
            let ty = by / COLOR_TILE_DIM_IN_BLOCKS;
            for tx in 0..xsize_blocks.div_ceil(COLOR_TILE_DIM_IN_BLOCKS) {
                let (x_factor, b_factor) = {
                    let cmap = &enc_state.shared.cmap;
                    let base = cmap.base();
                    (
                        base.y_to_x_ratio(i32::from(
                            cmap.ytox_map.row(cmap_rect.y0() + ty)[cmap_rect.x0() + tx],
                        )),
                        base.y_to_b_ratio(i32::from(
                            cmap.ytob_map.row(cmap_rect.y0() + ty)[cmap_rect.x0() + tx],
                        )),
                    )
                };
                let mut bx = tx * COLOR_TILE_DIM_IN_BLOCKS;
                while bx < xsize_blocks && bx < (tx + 1) * COLOR_TILE_DIM_IN_BLOCKS {
                    let acs = enc_state
                        .shared
                        .ac_strategy
                        .const_rect_row(block_group_rect, by)
                        .get(bx);
                    if !acs.is_first_block() {
                        bx += 1;
                        continue;
                    }

                    let mut xblocks = acs.covered_blocks_x();
                    let mut yblocks = acs.covered_blocks_y();

                    coefficient_layout(&mut yblocks, &mut xblocks);

                    let size = DCT_BLOCK_SIZE * xblocks * yblocks;

                    // DCT Y channel, roundtrip-quantize it and set DC.
                    let mut quant_ac = enc_state
                        .shared
                        .raw_quant_field
                        .row(block_group_rect.y0() + by)[block_group_rect.x0() + bx];
                    for c in 0..3 {
                        let pixels = opsin.plane(c).pixels().ok_or(StatusCode::GenericError)?;
                        let start = (group_rect.y0() + by * BLOCK_DIM) * opsin_stride
                            + group_rect.x0()
                            + bx * BLOCK_DIM;
                        transform_from_pixels(
                            acs.strategy(),
                            &pixels[start..],
                            opsin_stride,
                            &mut coeffs_in[c * size..],
                            scratch_space,
                        );
                    }
                    {
                        let dc_pixels = dc
                            .plane_mut(1)
                            .pixels_mut()
                            .ok_or(StatusCode::GenericError)?;
                        let start =
                            (block_group_rect.y0() + by) * dc_stride + block_group_rect.x0() + bx;
                        dc_from_lowest_frequencies(
                            acs.strategy(),
                            &coeffs_in[size..],
                            &mut dc_pixels[start..],
                            dc_stride,
                            scratch_space,
                        );
                    }

                    quantize_roundtrip_y_block_ac(
                        enc_state,
                        size,
                        &enc_state.shared.quantizer,
                        &enc_state.shared.matrices,
                        error_diffusion,
                        acs.strategy(),
                        xblocks,
                        yblocks,
                        &DEFAULT_QUANT_BIAS,
                        &mut quant_ac,
                        coeffs_in,
                        quantized,
                    );

                    // Unapply color correlation
                    for k in 0..size {
                        let in_x = coeffs_in[k];
                        let in_y = coeffs_in[size + k];
                        let in_b = coeffs_in[2 * size + k];
                        // Highway's scalar NegMulAdd: add - mul * x, unfused.
                        let out_x = in_x - x_factor * in_y;
                        let out_b = in_b - b_factor * in_y;
                        coeffs_in[k] = out_x;
                        coeffs_in[2 * size + k] = out_b;
                    }

                    // Quantize X and B channels and set DC.
                    for c in [0usize, 2] {
                        let mut thres = [0.58f32, 0.62, 0.62, 0.62];
                        quantize_block_ac(
                            &enc_state.shared.quantizer,
                            &enc_state.shared.matrices,
                            error_diffusion,
                            c,
                            if c == 0 {
                                enc_state.x_qm_multiplier
                            } else {
                                enc_state.b_qm_multiplier
                            },
                            acs.strategy(),
                            xblocks,
                            yblocks,
                            &mut thres,
                            &coeffs_in[c * size..],
                            quant_ac,
                            &mut quantized[c * size..],
                        );
                        let dc_pixels = dc
                            .plane_mut(c)
                            .pixels_mut()
                            .ok_or(StatusCode::GenericError)?;
                        let start =
                            (block_group_rect.y0() + by) * dc_stride + block_group_rect.x0() + bx;
                        dc_from_lowest_frequencies(
                            acs.strategy(),
                            &coeffs_in[c * size..],
                            &mut dc_pixels[start..],
                            dc_stride,
                            scratch_space,
                        );
                    }
                    enc_state
                        .shared
                        .raw_quant_field
                        .row_mut(block_group_rect.y0() + by)[block_group_rect.x0() + bx] = quant_ac;
                    for c in 0..3 {
                        let splitter = enc_state.progressive_splitter;
                        let mut outputs: Vec<&mut [i32]> = Vec::with_capacity(num_passes);
                        // Each pass owns a distinct ACImage, so the borrows
                        // are disjoint.
                        for (p, image) in enc_state.coeffs.iter_mut().take(num_passes).enumerate() {
                            let image = image.as_deref_mut().expect("checked above");
                            let AcPtrMut::I32(row) = image.plane_row_mut(c, group_idx, 0) else {
                                unreachable!("checked above");
                            };
                            outputs.push(&mut row[coeffs[c][p]..]);
                        }
                        splitter.split_ac_coefficients(
                            &quantized[c * size..c * size + size],
                            acs,
                            bx,
                            by,
                            &mut outputs,
                        );
                        for p in 0..num_passes {
                            coeffs[c][p] += size;
                        }
                    }
                    bx += 1;
                }
            }
        }
    }
    Ok(())
}

/// EncodeGroupTokenizedCoefficients.
pub fn encode_group_tokenized_coefficients(
    group_idx: usize,
    pass_idx: usize,
    histogram_idx: usize,
    enc_state: &PassesEncoderState<'_>,
    writer: &mut BitWriter,
    mut aux_out: Option<&mut AuxOut>,
) -> Status {
    // Select which histogram to use among those of the current pass.
    let num_histograms = enc_state.shared.num_histograms;
    // num_histograms is 0 only for lossless.
    if !(num_histograms == 0 || histogram_idx < num_histograms) {
        return Err(StatusCode::GenericError);
    }
    let histo_selector_bits = if num_histograms <= 1 {
        0
    } else {
        usize::BITS as usize - (num_histograms - 1).leading_zeros() as usize
    };

    if histo_selector_bits != 0 {
        writer.with_max_bits(
            histo_selector_bits,
            LayerType::Ac,
            aux_out.as_deref_mut(),
            |w, _| {
                w.write(histo_selector_bits, histogram_idx as u64);
                Ok(())
            },
            false,
        )?;
    }
    let context_offset = histogram_idx * enc_state.shared.block_ctx_map.num_ac_contexts() as usize;
    write_tokens_with_allotment(
        &enc_state.passes[pass_idx].ac_tokens[group_idx],
        &enc_state.passes[pass_idx].codes,
        context_offset,
        writer,
        LayerType::AcTokens,
        aux_out,
    )
}
