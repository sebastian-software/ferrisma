// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
//! TransformToPixels, AFVIDCT4x4 and LowestFrequenciesFromDC of
//! lib/jxl/dec_transforms-inl.h for the NEON and AVX2 targets (port ADR
//! 0004), written once against [`SimdF32`]. Compare them only with the
//! original's SIMD build at the same target.
//!
//! The scalar statements of these functions lie inside `HWY_NAMESPACE`, so
//! they carry the target attributes (FMA on NEON and AVX2): Clang contracts
//! IDENTITY's `block_dc - residual_sum * (1.0f / 16)` to one fused operation
//! on both targets. No other scalar statement here has a contractible
//! product and sum; they are those of the scalar translation
//! ([`super::dec_transforms`]).
use super::ac_strategy::{AcStrategy, AcStrategyType};
use super::dct_scales::total_resample_scale;
use super::dct_simd::{compute_scaled_dct_simd, compute_scaled_idct_simd};
use super::dec_transforms::AFV_BASIS;
use super::hwy::{Kernel, SimdF32, Target, dispatch};

/// AFVIDCT4x4 with `HWY_CAPPED(float, 16)` (four NEON, eight AVX2 lanes):
/// each pixel accumulates the sixteen fused products in basis order.
#[inline(always)]
fn afv_idct4x4<D: SimdF32>(d: D, coeffs: &[f32], pixels: &mut [f32]) {
    for i in (0..16).step_by(D::LANES) {
        let mut pixel = d.zero();
        for j in 0..16 {
            let cf = d.set(coeffs[j]);
            let basis = d.load(&AFV_BASIS[j][i..]);
            pixel = d.mul_add(cf, basis, pixel);
        }
        d.store(pixel, &mut pixels[i..]);
    }
}

// The original scalar AFV addition, in source operand order.
#[inline(always)]
fn afv_add(first: f32, second: f32) -> f32 {
    first + second
}

/// AFVTransformToPixels<afv_kind>.
#[inline(always)]
fn afv_transform_to_pixels<D: SimdF32>(
    d: D,
    kind: usize,
    coefficients: &[f32],
    pixels: &mut [f32],
    stride: usize,
) {
    let mut scratch = [0.0; 4 * 8 * 4];
    let ax = kind & 1;
    let ay = kind / 2;
    let a = coefficients[0];
    let b = coefficients[1];
    let c = coefficients[8];
    let dc0 = afv_add(b, a + c) * 4.0;
    let dcs = [dc0, a + c - b, a - c];
    let mut coeff = [0.0; 16];
    coeff[0] = dcs[0];
    for y in 0..4 {
        for x in 0..4 {
            if x != 0 || y != 0 {
                coeff[y * 4 + x] = coefficients[y * 16 + x * 2];
            }
        }
    }
    let mut block = [0.0; 32];
    afv_idct4x4(d, &coeff, &mut block);
    for y in 0..4 {
        for x in 0..4 {
            pixels[(y + ay * 4) * stride + ax * 4 + x] =
                block[(if ay == 1 { 3 - y } else { y }) * 4 + if ax == 1 { 3 - x } else { x }];
        }
    }
    block[0] = dcs[1];
    for y in 0..4 {
        for x in 0..4 {
            if x != 0 || y != 0 {
                block[y * 4 + x] = coefficients[y * 16 + x * 2 + 1];
            }
        }
    }
    compute_scaled_idct_simd(
        d,
        4,
        4,
        &mut block,
        &mut pixels[ay * 4 * stride + if ax == 1 { 0 } else { 4 }..],
        stride,
        &mut scratch,
    );
    block[0] = dcs[2];
    for y in 0..4 {
        for x in 0..8 {
            if x != 0 || y != 0 {
                block[y * 8 + x] = coefficients[(1 + y * 2) * 8 + x];
            }
        }
    }
    compute_scaled_idct_simd(
        d,
        4,
        8,
        &mut block,
        &mut pixels[(if ay == 1 { 0 } else { 4 }) * stride..],
        stride,
        &mut scratch,
    );
}

// The original identity pixel addition, coefficient first as in the source.
#[inline(always)]
fn identity_add(coefficient: f32, dc: f32) -> f32 {
    coefficient + dc
}

fn idct2_top_block(s: usize, block: &mut [f32]) {
    let mut temp = [0.0; 64];
    let half = s / 2;
    for y in 0..half {
        for x in 0..half {
            let a = block[y * 8 + x];
            let b = block[y * 8 + half + x];
            let c = block[(y + half) * 8 + x];
            let d = block[(y + half) * 8 + half + x];
            temp[y * 16 + x * 2] = a + b + c + d;
            temp[y * 16 + x * 2 + 1] = a + b - c - d;
            temp[(y * 2 + 1) * 8 + x * 2] = a - b + c - d;
            temp[(y * 2 + 1) * 8 + x * 2 + 1] = a - b - c + d;
        }
    }
    for y in 0..s {
        for x in 0..s {
            block[y * 8 + x] = temp[y * 8 + x];
        }
    }
}

/// TransformToPixels for the descriptor's target; the contract of
/// [`super::dec_transforms::transform_to_pixels`].
#[doc(hidden)]
#[inline(always)]
pub fn transform_to_pixels_d<D: SimdF32>(
    d: D,
    kind: AcStrategyType,
    coefficients: &mut [f32],
    pixels: &mut [f32],
    stride: usize,
    scratch: &mut [f32],
) {
    let acs = AcStrategy::from_raw_strategy(kind);
    assert!(stride >= acs.covered_blocks_x() * 8);
    match kind {
        AcStrategyType::IDENTITY => {
            let a = coefficients[0];
            let b = coefficients[1];
            let c = coefficients[8];
            let e = coefficients[9];
            let dcs = [a + b + c + e, a + b - c - e, a - b + c - e, a - b - c + e];
            for y in 0..2 {
                for x in 0..2 {
                    let dc = dcs[y * 2 + x];
                    let mut sum = 0.0;
                    for iy in 0..4 {
                        for ix in 0..4 {
                            if ix != 0 || iy != 0 {
                                sum += coefficients[(y + iy * 2) * 8 + x + ix * 2];
                            }
                        }
                    }
                    // Contracted on both targets (FMA in HWY_ATTR): one
                    // rounding of block_dc - residual_sum / 16.
                    let base = sum.mul_add(-1.0 / 16.0, dc);
                    pixels[(4 * y + 1) * stride + 4 * x + 1] = base;
                    for iy in 0..4 {
                        for ix in 0..4 {
                            if ix != 1 || iy != 1 {
                                pixels[(y * 4 + iy) * stride + x * 4 + ix] =
                                    identity_add(coefficients[(y + iy * 2) * 8 + x + ix * 2], base);
                            }
                        }
                    }
                    pixels[y * 4 * stride + x * 4] =
                        identity_add(coefficients[(y + 2) * 8 + x + 2], base);
                }
            }
        }
        AcStrategyType::DCT8X4 | AcStrategyType::DCT4X8 => {
            let dcs = [
                coefficients[0] + coefficients[8],
                coefficients[0] - coefficients[8],
            ];
            for (i, &dc) in dcs.iter().enumerate() {
                let mut block = [0.0; 32];
                block[0] = dc;
                for y in 0..4 {
                    for x in 0..8 {
                        if x != 0 || y != 0 {
                            block[y * 8 + x] = coefficients[(i + y * 2) * 8 + x];
                        }
                    }
                }
                if kind == AcStrategyType::DCT8X4 {
                    compute_scaled_idct_simd(
                        d,
                        8,
                        4,
                        &mut block,
                        &mut pixels[i * 4..],
                        stride,
                        scratch,
                    );
                } else {
                    compute_scaled_idct_simd(
                        d,
                        4,
                        8,
                        &mut block,
                        &mut pixels[i * 4 * stride..],
                        stride,
                        scratch,
                    );
                }
            }
        }
        AcStrategyType::DCT4X4 => {
            let a = coefficients[0];
            let b = coefficients[1];
            let c = coefficients[8];
            let e = coefficients[9];
            let dcs = [a + b + c + e, a + b - c - e, a - b + c - e, a - b - c + e];
            for y in 0..2 {
                for x in 0..2 {
                    let mut block = [0.0; 16];
                    block[0] = dcs[y * 2 + x];
                    for iy in 0..4 {
                        for ix in 0..4 {
                            if ix != 0 || iy != 0 {
                                block[iy * 4 + ix] = coefficients[(y + iy * 2) * 8 + x + ix * 2];
                            }
                        }
                    }
                    compute_scaled_idct_simd(
                        d,
                        4,
                        4,
                        &mut block,
                        &mut pixels[y * 4 * stride + x * 4..],
                        stride,
                        scratch,
                    );
                }
            }
        }
        AcStrategyType::DCT2X2 => {
            let mut block = [0.0; 64];
            block.copy_from_slice(&coefficients[..64]);
            idct2_top_block(2, &mut block);
            idct2_top_block(4, &mut block);
            idct2_top_block(8, &mut block);
            for y in 0..8 {
                for x in 0..8 {
                    pixels[y * stride + x] = block[y * 8 + x];
                }
            }
        }
        AcStrategyType::AFV0
        | AcStrategyType::AFV1
        | AcStrategyType::AFV2
        | AcStrategyType::AFV3 => afv_transform_to_pixels(
            d,
            (kind.0 - AcStrategyType::AFV0.0) as usize,
            coefficients,
            pixels,
            stride,
        ),
        _ => compute_scaled_idct_simd(
            d,
            acs.covered_blocks_y() * 8,
            acs.covered_blocks_x() * 8,
            coefficients,
            pixels,
            stride,
            scratch,
        ),
    }
}

/// LowestFrequenciesFromDC for the descriptor's target; the contract of
/// [`super::dec_transforms::lowest_frequencies_from_dc`].
#[doc(hidden)]
#[inline(always)]
pub fn lowest_frequencies_from_dc_d<D: SimdF32>(
    d: D,
    kind: AcStrategyType,
    dc: &[f32],
    dc_stride: usize,
    llf: &mut [f32],
    scratch: &mut [f32],
) {
    let acs = AcStrategy::from_raw_strategy(kind);
    if !acs.is_multiblock() {
        llf[0] = dc[0];
        return;
    }
    let rows = acs.covered_blocks_y();
    let cols = acs.covered_blocks_x();
    let stride = rows.max(cols) * 8;
    let mut warm_block = [0.0; 16];
    let mut warm_scratch = [0.0; 64];
    let (block, tmp) = if rows.max(cols) <= 4 {
        (&mut warm_block[..], &mut warm_scratch[..])
    } else {
        scratch.split_at_mut(rows * cols)
    };
    compute_scaled_dct_simd(d, rows, cols, dc, dc_stride, block, tmp);
    if rows < cols {
        for y in 0..rows {
            for x in 0..cols {
                llf[y * stride + x] = block[y * cols + x]
                    * total_resample_scale(rows, rows * 8, y)
                    * total_resample_scale(cols, cols * 8, x);
            }
        }
    } else {
        for y in 0..cols {
            for x in 0..rows {
                llf[y * stride + x] = block[y * rows + x]
                    * total_resample_scale(cols, cols * 8, y)
                    * total_resample_scale(rows, rows * 8, x);
            }
        }
    }
}

struct ToPixels<'a> {
    kind: AcStrategyType,
    coefficients: &'a mut [f32],
    pixels: &'a mut [f32],
    stride: usize,
    scratch: &'a mut [f32],
}
impl Kernel for ToPixels<'_> {
    type Output = ();
    #[inline(always)]
    fn run<D: SimdF32>(self, d: D) {
        transform_to_pixels_d(
            d,
            self.kind,
            self.coefficients,
            self.pixels,
            self.stride,
            self.scratch,
        )
    }
}

/// `TransformToPixels` instantiated for one SIMD target.
pub fn transform_to_pixels_simd(
    target: Target,
    kind: AcStrategyType,
    coefficients: &mut [f32],
    pixels: &mut [f32],
    stride: usize,
    scratch: &mut [f32],
) {
    dispatch(
        target,
        ToPixels {
            kind,
            coefficients,
            pixels,
            stride,
            scratch,
        },
    )
}

struct Llf<'a> {
    kind: AcStrategyType,
    dc: &'a [f32],
    dc_stride: usize,
    llf: &'a mut [f32],
    scratch: &'a mut [f32],
}
impl Kernel for Llf<'_> {
    type Output = ();
    #[inline(always)]
    fn run<D: SimdF32>(self, d: D) {
        lowest_frequencies_from_dc_d(
            d,
            self.kind,
            self.dc,
            self.dc_stride,
            self.llf,
            self.scratch,
        )
    }
}

/// `LowestFrequenciesFromDC` instantiated for one SIMD target.
pub fn lowest_frequencies_from_dc_simd(
    target: Target,
    kind: AcStrategyType,
    dc: &[f32],
    dc_stride: usize,
    llf: &mut [f32],
    scratch: &mut [f32],
) {
    dispatch(
        target,
        Llf {
            kind,
            dc,
            dc_stride,
            llf,
            scratch,
        },
    )
}

struct ScaledDct<'a> {
    inverse: bool,
    rows: usize,
    cols: usize,
    input: &'a mut [f32],
    stride: usize,
    output: &'a mut [f32],
    scratch: &'a mut [f32],
}
impl Kernel for ScaledDct<'_> {
    type Output = ();
    #[inline(always)]
    fn run<D: SimdF32>(self, d: D) {
        if self.inverse {
            compute_scaled_idct_simd(
                d,
                self.rows,
                self.cols,
                self.input,
                self.output,
                self.stride,
                self.scratch,
            )
        } else {
            compute_scaled_dct_simd(
                d,
                self.rows,
                self.cols,
                self.input,
                self.stride,
                self.output,
                self.scratch,
            )
        }
    }
}

/// ComputeScaledIDCT<rows, cols> (`inverse`, from `input` into `output`
/// rows of `stride`) or ComputeScaledDCT<rows, cols> (from `input` rows of
/// `stride` into `output`) instantiated for one SIMD target.
#[allow(clippy::too_many_arguments)]
pub fn compute_scaled_simd(
    target: Target,
    inverse: bool,
    rows: usize,
    cols: usize,
    input: &mut [f32],
    stride: usize,
    output: &mut [f32],
    scratch: &mut [f32],
) {
    dispatch(
        target,
        ScaledDct {
            inverse,
            rows,
            cols,
            input,
            stride,
            output,
            scratch,
        },
    )
}
