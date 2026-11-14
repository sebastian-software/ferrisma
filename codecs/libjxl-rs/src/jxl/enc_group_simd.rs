// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
//! ComputeCoefficients with QuantizeBlockAC, AdjustQuantBlockAC and
//! QuantizeRoundtripYBlockAC of lib/jxl/enc_group.cc for the NEON and AVX2
//! targets (port ADR 0004, "Encoder path"), written once against
//! [`SimdF32`]. Inside `HWY_NAMESPACE` the transforms are the in-namespace
//! SIMD TransformFromPixels and DCFromLowestFrequencies
//! (`enc_transforms_simd`), and the dequantization bias is quantizer-inl.h's
//! vector AdjustQuantBias with its `ApproximateReciprocal` (`dec_group_simd`).
//! Compare them only with the original's SIMD build at the same target; the
//! scalar translation stays in `enc_group`.
//!
//! The scalar statements inside `HWY_NAMESPACE` carry FMA on both targets, so
//! the contraction sites of the scalar translation's map are fused here on
//! x86-64 too: QuantizeBlockAC's `thresholds[i] -= 0.00744f * xsize * ysize`
//! and AdjustQuantBlockAC's two double sites (`adjust_quant_block_ac_with`
//! with `FUSED`).
use super::ac_strategy::{AcStrategyType, MAX_BLOCK_DIM, MAX_COEFF_AREA};
use super::base::rect::Rect;
use super::base::status::{Status, StatusCode};
use super::chroma_from_luma::COLOR_TILE_DIM_IN_BLOCKS;
use super::coeff_order_fwd::coefficient_layout;
use super::common::{MAX_NUM_PASSES, SpeedTier};
use super::dct_util::{AcPtrMut, AcType};
use super::dec_group_simd::adjust_quant_bias_d;
use super::enc_cache::PassesEncoderState;
use super::enc_group::adjust_quant_block_ac_with;
use super::enc_transforms_simd::{dc_from_lowest_frequencies_d, transform_from_pixels_d};
use super::frame_dimensions::{BLOCK_DIM, DCT_BLOCK_SIZE};
use super::hwy::{Kernel, SimdF32, Target, dispatch};
use super::image::Image3;
use super::quant_weights::DequantMatrices;
use super::quantizer::{DEFAULT_QUANT_BIAS, Quantizer};

/// QuantizeBlockAC. The caller extracts `quant` from the raw quant field.
/// `block_in` and `block_out` hold `xsize * ysize * 64` samples.
#[allow(clippy::too_many_arguments)] // Preserve the original function boundary.
pub fn quantize_block_ac_d<D: SimdF32>(
    d: D,
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
            // expression: fmuladd(-(0.00744f * xsize), ysize, thresholds[i]),
            // fused on both SIMD targets.
            *threshold = (-(0.00744f32 * xsize as f32)).mul_add(ysize as f32, *threshold);
            if f64::from(*threshold) < 0.5 {
                *threshold = 0.5;
            }
        }
    }
    let quantv = d.set(qac * qm_multiplier);
    for y in 0..ysize * BLOCK_DIM {
        let yfix = usize::from(y >= ysize * BLOCK_DIM / 2) * 2;
        let off = y * BLOCK_DIM * xsize;
        let mut x = 0;
        // HWY_CAPPED(float, kBlockDim): four NEON, eight AVX2 lanes.
        while x < xsize * BLOCK_DIM {
            let threshold = if xsize == 1 {
                const MASK: [u32; BLOCK_DIM] = [0, 0, 0, 0, !0, !0, !0, !0];
                // MaskFromVec(BitCast(df, Load(du, kMask + x))): the lanes
                // are all ones or all zeros.
                let mask = d.bitcast_to_i32(d.load_bits(&MASK[x..]));
                d.bitcast_to_f32(d.if_then_else_i32(
                    mask,
                    d.bitcast_to_i32(d.set(thresholds[yfix + 1])),
                    d.bitcast_to_i32(d.set(thresholds[yfix])),
                ))
            } else {
                // Same for all lanes in the vector.
                d.set(thresholds[yfix + usize::from(x >= xsize * BLOCK_DIM / 2)])
            };
            let q = d.mul(d.load(&qm[off + x..]), quantv);
            let input = d.load(&block_in[off + x..]);
            let val = d.mul(q, input);
            let nzero_mask = d.ge(d.abs(val), threshold);
            let v = d.convert_to_i32(d.if_then_else_zero(nzero_mask, d.round(val)));
            d.store_i32(v, &mut block_out[off + x..]);
            x += D::LANES;
        }
    }
}

/// QuantizeRoundtripYBlockAC. `inout` and `quantized` hold the three
/// channels, `size` samples apart.
#[allow(clippy::too_many_arguments)] // Preserve the original function boundary.
pub fn quantize_roundtrip_y_block_ac_d<D: SimdF32>(
    d: D,
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
            adjust_quant_block_ac_with::<true>(
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
        quantize_block_ac_d(
            d,
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
    // HWY_CAPPED(float, kDCTBlockSize): full vectors on both targets.
    let inv_qac = d.set(quantizer.inv_quant_ac(*quant));
    let mut k = 0;
    while k < DCT_BLOCK_SIZE * xsize * ysize {
        let oquant = d.load_i32(&quantized[size + k..]);
        let adj_quant = adjust_quant_bias_d(d, 1, oquant, biases);
        let dequantm = d.load(&dequant_matrix[k..]);
        d.store(
            d.mul(d.mul(adj_quant, dequantm), inv_qac),
            &mut inout[size + k..],
        );
        k += D::LANES;
    }
}

/// ComputeCoefficients for one group and one descriptor; the contract of
/// [`super::enc_group::compute_coefficients`].
#[expect(
    clippy::needless_range_loop,
    reason = "Keep the source pass and channel index loops."
)]
pub fn compute_coefficients_d<D: SimdF32>(
    d: D,
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

    // MaxVectorSize() / sizeof(float) of the selected target.
    let dct_scratch_size = 3 * D::LANES * MAX_BLOCK_DIM;

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
                        transform_from_pixels_d(
                            d,
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
                        dc_from_lowest_frequencies_d(
                            d,
                            acs.strategy(),
                            &coeffs_in[size..],
                            &mut dc_pixels[start..],
                            dc_stride,
                            scratch_space,
                        );
                    }

                    quantize_roundtrip_y_block_ac_d(
                        d,
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
                    {
                        let x_factor = d.set(x_factor);
                        let b_factor = d.set(b_factor);
                        let mut k = 0;
                        while k < size {
                            let in_x = d.load(&coeffs_in[k..]);
                            let in_y = d.load(&coeffs_in[size + k..]);
                            let in_b = d.load(&coeffs_in[2 * size + k..]);
                            let out_x = d.neg_mul_add(x_factor, in_y, in_x);
                            let out_b = d.neg_mul_add(b_factor, in_y, in_b);
                            d.store(out_x, &mut coeffs_in[k..]);
                            d.store(out_b, &mut coeffs_in[2 * size + k..]);
                            k += D::LANES;
                        }
                    }

                    // Quantize X and B channels and set DC.
                    for c in [0usize, 2] {
                        let mut thres = [0.58f32, 0.62, 0.62, 0.62];
                        quantize_block_ac_d(
                            d,
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
                        dc_from_lowest_frequencies_d(
                            d,
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

struct Coefficients<'a, 'b> {
    group_idx: usize,
    enc_state: &'a mut PassesEncoderState<'b>,
    opsin: &'a Image3<f32>,
    rect: Rect,
    dc: &'a mut Image3<f32>,
}
impl Kernel for Coefficients<'_, '_> {
    type Output = Status;
    #[inline(always)]
    fn run<D: SimdF32>(self, d: D) -> Status {
        compute_coefficients_d(
            d,
            self.group_idx,
            self.enc_state,
            self.opsin,
            self.rect,
            self.dc,
        )
    }
}

/// `ComputeCoefficients` instantiated for one SIMD target.
pub fn compute_coefficients_simd(
    target: Target,
    group_idx: usize,
    enc_state: &mut PassesEncoderState<'_>,
    opsin: &Image3<f32>,
    rect: Rect,
    dc: &mut Image3<f32>,
) -> Status {
    dispatch(
        target,
        Coefficients {
            group_idx,
            enc_state,
            opsin,
            rect,
            dc,
        },
    )
}

/// The arguments of QuantizeBlockAC after the quantizer and matrices.
pub struct QuantizeBlockArgs<'a> {
    pub c: usize,
    pub qm_multiplier: f32,
    pub quant_kind: AcStrategyType,
    pub xsize: usize,
    pub ysize: usize,
    pub thresholds: &'a mut [f32; 4],
    pub block_in: &'a [f32],
    pub quant: i32,
    pub block_out: &'a mut [i32],
}
struct QuantizeBlock<'a> {
    quantizer: &'a Quantizer,
    matrices: &'a DequantMatrices,
    args: QuantizeBlockArgs<'a>,
}
impl Kernel for QuantizeBlock<'_> {
    type Output = ();
    #[inline(always)]
    fn run<D: SimdF32>(self, d: D) {
        let a = self.args;
        quantize_block_ac_d(
            d,
            self.quantizer,
            self.matrices,
            false,
            a.c,
            a.qm_multiplier,
            a.quant_kind,
            a.xsize,
            a.ysize,
            a.thresholds,
            a.block_in,
            a.quant,
            a.block_out,
        );
    }
}

/// `QuantizeBlockAC` instantiated for one SIMD target (its error-diffusion
/// flag is unused in the original).
pub fn quantize_block_ac_simd(
    target: Target,
    quantizer: &Quantizer,
    matrices: &DequantMatrices,
    args: QuantizeBlockArgs<'_>,
) {
    dispatch(
        target,
        QuantizeBlock {
            quantizer,
            matrices,
            args,
        },
    );
}

/// The arguments of QuantizeRoundtripYBlockAC after the encoder state and
/// the quantizer.
pub struct RoundtripArgs<'a> {
    pub size: usize,
    pub quant_kind: AcStrategyType,
    pub xsize: usize,
    pub ysize: usize,
    pub quant: &'a mut i32,
    pub inout: &'a mut [f32],
    pub quantized: &'a mut [i32],
}
struct Roundtrip<'a, 'b> {
    enc_state: &'a PassesEncoderState<'b>,
    quantizer: &'a Quantizer,
    matrices: &'a DequantMatrices,
    args: RoundtripArgs<'a>,
}
impl Kernel for Roundtrip<'_, '_> {
    type Output = ();
    #[inline(always)]
    fn run<D: SimdF32>(self, d: D) {
        let a = self.args;
        quantize_roundtrip_y_block_ac_d(
            d,
            self.enc_state,
            a.size,
            self.quantizer,
            self.matrices,
            false,
            a.quant_kind,
            a.xsize,
            a.ysize,
            &DEFAULT_QUANT_BIAS,
            a.quant,
            a.inout,
            a.quantized,
        );
    }
}

/// `QuantizeRoundtripYBlockAC` with kDefaultQuantBias instantiated for one
/// SIMD target.
pub fn quantize_roundtrip_y_block_ac_simd(
    target: Target,
    enc_state: &PassesEncoderState<'_>,
    quantizer: &Quantizer,
    matrices: &DequantMatrices,
    args: RoundtripArgs<'_>,
) {
    dispatch(
        target,
        Roundtrip {
            enc_state,
            quantizer,
            matrices,
            args,
        },
    );
}
