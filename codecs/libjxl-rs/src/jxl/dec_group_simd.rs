// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
//! DequantLane/DequantBlock of lib/jxl/dec_group.cc with AdjustQuantBias of
//! lib/jxl/quantizer-inl.h for the NEON and AVX2 targets (port ADR 0004),
//! written once against [`SimdF32`] with `HWY_FULL(float)`. Compare them only
//! with the original's SIMD build at the same target.
//!
//! AdjustQuantBias uses `ApproximateReciprocal`: `vrecpe` on NEON, which
//! the Arm architecture defines exactly, and the vendor-specific `rcpps` on
//! AVX2. Working decision 2026-11-13 evening (delegated, revisable by the
//! owner): both targets select this instantiation, and the AVX2 path executes
//! the original's estimate (`_mm256_rcp_ps`), so AVX2 output can differ
//! between CPU vendors exactly as the original's AVX2 build does. Identity is
//! claimed per host against the original (ADR-0041), never across hosts. The
//! owner's open question O4 (#250) still covers whether that is acceptable
//! for shipped defaults; the default selection stays off.
use super::ac_strategy::AcStrategyType;
use super::dct_util::ConstAcPtr;
use super::dec_transforms_simd::lowest_frequencies_from_dc_d;
use super::frame_dimensions::DCT_BLOCK_SIZE;
use super::hwy::{Kernel, SimdF32, Target, dispatch};
use super::quant_weights::DequantMatrices;
use super::quantizer::Quantizer;

/// AdjustQuantBias(di, c, quant_i, biases) on full vectors.
#[inline(always)]
pub(crate) fn adjust_quant_bias_d<D: SimdF32>(
    d: D,
    c: usize,
    quant_i: D::I,
    biases: &[f32; 4],
) -> D::V {
    let quant = d.convert_to_f32(quant_i);
    // BitCast(df, Set(di, INT32_MIN)).
    let k_sign = d.set(f32::from_bits(0x8000_0000));
    let sign = d.and(quant, k_sign);
    let abs_quant = d.and_not(k_sign, quant);
    let is_01 = d.lt(abs_quant, d.set(1.125));
    let not_0 = d.gt(abs_quant, d.zero());
    let one_bias = d.if_then_else_zero(not_0, d.xor(d.set(biases[c]), sign));
    let bias = d.neg_mul_add(d.set(biases[3]), d.approximate_reciprocal(quant), quant);
    d.if_then_else(is_01, one_bias, bias)
}

/// `Load(di, ptr32 + k)` or `PromoteTo(di, Load(di16, ptr16 + k))`.
#[inline(always)]
fn load_quantized<D: SimdF32>(d: D, block: &ConstAcPtr<'_>, k: usize) -> D::I {
    let mut bits = [0u32; 16];
    let bits = &mut bits[..D::LANES];
    match block {
        ConstAcPtr::I16(values) => {
            for (b, &v) in bits.iter_mut().zip(&values[k..k + D::LANES]) {
                *b = i32::from(v) as u32;
            }
        }
        ConstAcPtr::I32(values) => {
            for (b, &v) in bits.iter_mut().zip(&values[k..k + D::LANES]) {
                *b = v as u32;
            }
        }
    }
    d.bitcast_to_i32(d.load_bits(bits))
}

/// DequantLane<ac_type> at `k` (a lane multiple).
#[allow(clippy::too_many_arguments)]
#[inline(always)]
fn dequant_lane_d<D: SimdF32>(
    d: D,
    scaled_dequant: [D::V; 3],
    dequant_matrices: [&[f32]; 3],
    size: usize,
    k: usize,
    x_cc_mul: D::V,
    b_cc_mul: D::V,
    biases: &[f32; 4],
    qblock: &[ConstAcPtr<'_>; 3],
    block: &mut [f32],
) {
    let x_mul = d.mul(d.load(&dequant_matrices[0][k..]), scaled_dequant[0]);
    let y_mul = d.mul(d.load(&dequant_matrices[1][k..]), scaled_dequant[1]);
    let b_mul = d.mul(d.load(&dequant_matrices[2][k..]), scaled_dequant[2]);
    let quantized_x_int = load_quantized(d, &qblock[0], k);
    let quantized_y_int = load_quantized(d, &qblock[1], k);
    let quantized_b_int = load_quantized(d, &qblock[2], k);
    let dequant_x_cc = d.mul(adjust_quant_bias_d(d, 0, quantized_x_int, biases), x_mul);
    let dequant_y = d.mul(adjust_quant_bias_d(d, 1, quantized_y_int, biases), y_mul);
    let dequant_b_cc = d.mul(adjust_quant_bias_d(d, 2, quantized_b_int, biases), b_mul);
    let dequant_x = d.mul_add(x_cc_mul, dequant_y, dequant_x_cc);
    let dequant_b = d.mul_add(b_cc_mul, dequant_y, dequant_b_cc);
    d.store(dequant_x, &mut block[k..]);
    d.store(dequant_y, &mut block[size + k..]);
    d.store(dequant_b, &mut block[2 * size + k..]);
}

/// The arguments of DequantBlock, as in [`super::dec_group_render::dequant_block`].
pub struct DequantBlockArgs<'a> {
    pub inv_global_scale: f32,
    pub quant: i32,
    pub x_dm_multiplier: f32,
    pub b_dm_multiplier: f32,
    pub x_cc_mul: f32,
    pub b_cc_mul: f32,
    pub kind: AcStrategyType,
    pub size: usize,
    pub quantizer: &'a Quantizer,
    pub matrices: &'a DequantMatrices,
    pub covered_blocks: usize,
    pub sbx: [usize; 3],
    pub dc_rows: [&'a [f32]; 3],
    pub dc_stride: usize,
    pub biases: &'a [f32; 4],
    pub qblock: &'a [ConstAcPtr<'a>; 3],
    pub block: &'a mut [f32],
    pub scratch: &'a mut [f32],
}

impl Kernel for DequantBlockArgs<'_> {
    type Output = ();
    #[inline(always)]
    fn run<D: SimdF32>(self, d: D) {
        let scaled_dequant_s = self.inv_global_scale / self.quant as f32;
        let scaled_dequant = [
            d.set(scaled_dequant_s * self.x_dm_multiplier),
            d.set(scaled_dequant_s),
            d.set(scaled_dequant_s * self.b_dm_multiplier),
        ];
        let dequant_matrices =
            std::array::from_fn(|c| self.quantizer.dequant_matrix(self.matrices, self.kind, c));
        let x_cc_mul = d.set(self.x_cc_mul);
        let b_cc_mul = d.set(self.b_cc_mul);
        for k in (0..self.covered_blocks * DCT_BLOCK_SIZE).step_by(D::LANES) {
            dequant_lane_d(
                d,
                scaled_dequant,
                dequant_matrices,
                self.size,
                k,
                x_cc_mul,
                b_cc_mul,
                self.biases,
                self.qblock,
                self.block,
            );
        }
        for c in 0..3 {
            lowest_frequencies_from_dc_d(
                d,
                self.kind,
                &self.dc_rows[c][self.sbx[c]..],
                self.dc_stride,
                &mut self.block[c * self.size..],
                self.scratch,
            );
        }
    }
}

/// `DequantBlock` instantiated for one SIMD target.
pub fn dequant_block_simd(target: Target, args: DequantBlockArgs<'_>) {
    dispatch(target, args)
}

struct Bias<'a> {
    c: usize,
    quant: &'a [i32],
    biases: &'a [f32; 4],
}
impl Kernel for Bias<'_> {
    type Output = Vec<f32>;
    #[inline(always)]
    fn run<D: SimdF32>(self, d: D) -> Vec<f32> {
        self.quant
            .iter()
            .map(|&q| {
                let v = d.bitcast_to_i32(d.load_bits(&[q as u32; 16][..D::LANES]));
                d.get_lane(adjust_quant_bias_d(d, self.c, v, self.biases))
            })
            .collect()
    }
}

/// AdjustQuantBias on a broadcast quantized value, lane 0 of the result,
/// for each of `quant` (the oracle's `dq-bias` observation).
pub fn adjust_quant_bias_simd(
    target: Target,
    c: usize,
    quant: &[i32],
    biases: &[f32; 4],
) -> Vec<f32> {
    dispatch(target, Bias { c, quant, biases })
}
