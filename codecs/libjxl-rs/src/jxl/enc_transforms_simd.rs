// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
//! TransformFromPixels (with DCT2TopBlock and AFVTransformFromPixels),
//! DCFromLowestFrequencies (with ReinterpretingIDCT) and AFVDCT4x4 of
//! lib/jxl/enc_transforms-inl.h for the NEON and AVX2 targets (port ADR
//! 0004, "Encoder path"), written once against [`SimdF32`]. The scaled
//! (I)DCTs are the decoder's SIMD dct-inl.h translation (`dct_simd`), not a
//! copy. Compare them only with the original's SIMD build at the same
//! target; the scalar translation stays in `enc_transforms`.
//!
//! The scalar statements of these functions lie inside `HWY_NAMESPACE`, so
//! they carry the target attributes (FMA on NEON and AVX2): Clang contracts
//! AFVTransformFromPixels' final DC mix `block00 + block01 +/- 2 * block10`
//! to one fused operation on both targets (on x86-64 the scalar build fuses
//! nothing, so this differs from the scalar translation there). No other
//! scalar statement here has a contractible product and sum: IDENTITY sums
//! and scales in separate statements, DCT2TopBlock adds before it scales,
//! and ReinterpretingIDCT only multiplies.
use super::ac_strategy::AcStrategyType;
use super::dct_scales::total_resample_scale;
use super::dct_simd::{compute_scaled_dct_simd, compute_scaled_idct_simd};
use super::enc_transforms::{K4X4_AFV_BASIS_TRANSPOSE, dct2_top_block, store_dct2_top_block};
use super::frame_dimensions::BLOCK_DIM;
use super::hwy::{Kernel, SimdF32, Target, dispatch};

const MAX_BLOCKS: usize = 32;

/// AFVDCT4x4 with `HWY_CAPPED(float, 16)` (four NEON, eight AVX2 lanes):
/// each coefficient accumulates the sixteen fused products in pixel order.
#[inline(always)]
pub fn afvdct4x4_d<D: SimdF32>(d: D, pixels: &[f32], coeffs: &mut [f32]) {
    for i in (0..16).step_by(D::LANES) {
        let mut scalar = d.zero();
        for j in 0..16 {
            let px = d.set(pixels[j]);
            let basis = d.load(&K4X4_AFV_BASIS_TRANSPOSE[j][i..]);
            scalar = d.mul_add(px, basis, scalar);
        }
        d.store(scalar, &mut coeffs[i..]);
    }
}

// AFVTransformFromPixels' final DC mix, contracted on both SIMD targets.
#[inline(always)]
fn afv_dc_mix(sum: f32, block10: f32, scale: f32) -> f32 {
    block10.mul_add(scale, sum)
}

/// Inverse of ReinterpretingDCT. `scratch` holds the 32x32 coefficient block
/// followed by the scaled IDCT's own scratch.
#[expect(
    clippy::too_many_arguments,
    reason = "Spell out the source's template arguments."
)]
fn reinterpreting_idct<D: SimdF32>(
    d: D,
    dct_rows: usize,
    dct_cols: usize,
    lf_rows: usize,
    lf_cols: usize,
    rows: usize,
    cols: usize,
    input: &[f32],
    input_stride: usize,
    output: &mut [f32],
    output_stride: usize,
    scratch: &mut [f32],
) {
    assert!(rows <= MAX_BLOCKS, "Unsupported block size");
    assert!(cols <= MAX_BLOCKS, "Unsupported block size");
    let (block, scratch_space) = scratch.split_at_mut(MAX_BLOCKS * MAX_BLOCKS);
    if rows < cols {
        for y in 0..lf_rows {
            for x in 0..lf_cols {
                block[y * cols + x] = input[y * input_stride + x]
                    * total_resample_scale(dct_rows, rows, y)
                    * total_resample_scale(dct_cols, cols, x);
            }
        }
    } else {
        for y in 0..lf_cols {
            for x in 0..lf_rows {
                block[y * rows + x] = input[y * input_stride + x]
                    * total_resample_scale(dct_cols, cols, y)
                    * total_resample_scale(dct_rows, rows, x);
            }
        }
    }

    compute_scaled_idct_simd(
        d,
        rows,
        cols,
        &mut block[..rows * cols],
        output,
        output_stride,
        scratch_space,
    );
}

// Coefficient layout:
//  - (even, even) positions hold AFV coefficients
//  - (odd, even) positions hold DCT4x4 coefficients
//  - (any, odd) positions hold DCT4x8 coefficients
fn afv_transform_from_pixels<D: SimdF32>(
    d: D,
    afv_kind: usize,
    pixels: &[f32],
    pixels_stride: usize,
    coefficients: &mut [f32],
) {
    let mut scratch_space = [0.0f32; 4 * 8 * 5];
    let afv_x = afv_kind & 1;
    let afv_y = afv_kind / 2;
    let mut block = [0.0f32; 4 * 8];
    for iy in 0..4 {
        for ix in 0..4 {
            block[(if afv_y == 1 { 3 - iy } else { iy }) * 4
                + if afv_x == 1 { 3 - ix } else { ix }] =
                pixels[(iy + 4 * afv_y) * pixels_stride + ix + 4 * afv_x];
        }
    }
    // AFV coefficients in (even, even) positions.
    let mut coeff = [0.0f32; 4 * 4];
    afvdct4x4_d(d, &block, &mut coeff);
    for iy in 0..4 {
        for ix in 0..4 {
            coefficients[iy * 2 * 8 + ix * 2] = coeff[iy * 4 + ix];
        }
    }
    // 4x4 DCT of the block with same y and different x.
    compute_scaled_dct_simd(
        d,
        4,
        4,
        &pixels[afv_y * 4 * pixels_stride + if afv_x == 1 { 0 } else { 4 }..],
        pixels_stride,
        &mut block,
        &mut scratch_space,
    );
    // ... in (odd, even) positions.
    for iy in 0..4 {
        for ix in 0..8 {
            coefficients[iy * 2 * 8 + ix * 2 + 1] = block[iy * 4 + ix];
        }
    }
    // 4x8 DCT of the other half of the block.
    compute_scaled_dct_simd(
        d,
        4,
        8,
        &pixels[(if afv_y == 1 { 0 } else { 4 }) * pixels_stride..],
        pixels_stride,
        &mut block,
        &mut scratch_space,
    );
    for iy in 0..4 {
        for ix in 0..8 {
            coefficients[(1 + iy * 2) * 8 + ix] = block[iy * 8 + ix];
        }
    }
    let block00 = coefficients[0] * 0.25;
    let block01 = coefficients[1];
    let block10 = coefficients[8];
    coefficients[0] = afv_dc_mix(block00 + block01, block10, 2.0) * 0.25;
    coefficients[1] = (block00 - block01) * 0.5;
    coefficients[8] = afv_dc_mix(block00 + block01, block10, -2.0) * 0.25;
}

/// Original TransformFromPixels for one descriptor; the contract of
/// [`super::enc_transforms::transform_from_pixels`].
pub fn transform_from_pixels_d<D: SimdF32>(
    d: D,
    strategy: AcStrategyType,
    pixels: &[f32],
    pixels_stride: usize,
    coefficients: &mut [f32],
    scratch_space: &mut [f32],
) {
    type Type = AcStrategyType;
    let scaled = |rows: usize, cols: usize, coefficients: &mut [f32], scratch: &mut [f32]| {
        compute_scaled_dct_simd(d, rows, cols, pixels, pixels_stride, coefficients, scratch);
    };
    match strategy {
        Type::IDENTITY => {
            for y in 0..2 {
                for x in 0..2 {
                    let mut block_dc = 0.0f32;
                    for iy in 0..4 {
                        for ix in 0..4 {
                            block_dc += pixels[(y * 4 + iy) * pixels_stride + x * 4 + ix];
                        }
                    }
                    block_dc *= 1.0 / 16.0;
                    for iy in 0..4 {
                        for ix in 0..4 {
                            if ix == 1 && iy == 1 {
                                continue;
                            }
                            coefficients[(y + iy * 2) * 8 + x + ix * 2] = pixels
                                [(y * 4 + iy) * pixels_stride + x * 4 + ix]
                                - pixels[(y * 4 + 1) * pixels_stride + x * 4 + 1];
                        }
                    }
                    coefficients[(y + 2) * 8 + x + 2] = coefficients[y * 8 + x];
                    coefficients[y * 8 + x] = block_dc;
                }
            }
            let block00 = coefficients[0];
            let block01 = coefficients[1];
            let block10 = coefficients[8];
            let block11 = coefficients[9];
            coefficients[0] = (block00 + block01 + block10 + block11) * 0.25;
            coefficients[1] = (block00 + block01 - block10 - block11) * 0.25;
            coefficients[8] = (block00 - block01 + block10 - block11) * 0.25;
            coefficients[9] = (block00 - block01 - block10 + block11) * 0.25;
        }
        Type::DCT8X4 => {
            for x in 0..2 {
                let mut block = [0.0f32; 4 * 8];
                compute_scaled_dct_simd(
                    d,
                    8,
                    4,
                    &pixels[x * 4..],
                    pixels_stride,
                    &mut block,
                    scratch_space,
                );
                for iy in 0..4 {
                    for ix in 0..8 {
                        // Store transposed.
                        coefficients[(x + iy * 2) * 8 + ix] = block[iy * 8 + ix];
                    }
                }
            }
            let block0 = coefficients[0];
            let block1 = coefficients[8];
            coefficients[0] = (block0 + block1) * 0.5;
            coefficients[8] = (block0 - block1) * 0.5;
        }
        Type::DCT4X8 => {
            for y in 0..2 {
                let mut block = [0.0f32; 4 * 8];
                compute_scaled_dct_simd(
                    d,
                    4,
                    8,
                    &pixels[y * 4 * pixels_stride..],
                    pixels_stride,
                    &mut block,
                    scratch_space,
                );
                for iy in 0..4 {
                    for ix in 0..8 {
                        coefficients[(y + iy * 2) * 8 + ix] = block[iy * 8 + ix];
                    }
                }
            }
            let block0 = coefficients[0];
            let block1 = coefficients[8];
            coefficients[0] = (block0 + block1) * 0.5;
            coefficients[8] = (block0 - block1) * 0.5;
        }
        Type::DCT4X4 => {
            for y in 0..2 {
                for x in 0..2 {
                    let mut block = [0.0f32; 4 * 4];
                    compute_scaled_dct_simd(
                        d,
                        4,
                        4,
                        &pixels[y * 4 * pixels_stride + x * 4..],
                        pixels_stride,
                        &mut block,
                        scratch_space,
                    );
                    for iy in 0..4 {
                        for ix in 0..4 {
                            coefficients[(y + iy * 2) * 8 + x + ix * 2] = block[iy * 4 + ix];
                        }
                    }
                }
            }
            let block00 = coefficients[0];
            let block01 = coefficients[1];
            let block10 = coefficients[8];
            let block11 = coefficients[9];
            coefficients[0] = (block00 + block01 + block10 + block11) * 0.25;
            coefficients[1] = (block00 + block01 - block10 - block11) * 0.25;
            coefficients[8] = (block00 - block01 + block10 - block11) * 0.25;
            coefficients[9] = (block00 - block01 - block10 + block11) * 0.25;
        }
        Type::DCT2X2 => {
            let temp = dct2_top_block(8, pixels, pixels_stride);
            store_dct2_top_block(8, &temp, coefficients);
            let temp = dct2_top_block(4, coefficients, BLOCK_DIM);
            store_dct2_top_block(4, &temp, coefficients);
            let temp = dct2_top_block(2, coefficients, BLOCK_DIM);
            store_dct2_top_block(2, &temp, coefficients);
        }
        Type::DCT16X16 => scaled(16, 16, coefficients, scratch_space),
        Type::DCT16X8 => scaled(16, 8, coefficients, scratch_space),
        Type::DCT8X16 => scaled(8, 16, coefficients, scratch_space),
        Type::DCT32X8 => scaled(32, 8, coefficients, scratch_space),
        Type::DCT8X32 => scaled(8, 32, coefficients, scratch_space),
        Type::DCT32X16 => scaled(32, 16, coefficients, scratch_space),
        Type::DCT16X32 => scaled(16, 32, coefficients, scratch_space),
        Type::DCT32X32 => scaled(32, 32, coefficients, scratch_space),
        Type::DCT => scaled(8, 8, coefficients, scratch_space),
        Type::AFV0 => afv_transform_from_pixels(d, 0, pixels, pixels_stride, coefficients),
        Type::AFV1 => afv_transform_from_pixels(d, 1, pixels, pixels_stride, coefficients),
        Type::AFV2 => afv_transform_from_pixels(d, 2, pixels, pixels_stride, coefficients),
        Type::AFV3 => afv_transform_from_pixels(d, 3, pixels, pixels_stride, coefficients),
        Type::DCT64X64 => scaled(64, 64, coefficients, scratch_space),
        Type::DCT64X32 => scaled(64, 32, coefficients, scratch_space),
        Type::DCT32X64 => scaled(32, 64, coefficients, scratch_space),
        Type::DCT128X128 => scaled(128, 128, coefficients, scratch_space),
        Type::DCT128X64 => scaled(128, 64, coefficients, scratch_space),
        Type::DCT64X128 => scaled(64, 128, coefficients, scratch_space),
        Type::DCT256X256 => scaled(256, 256, coefficients, scratch_space),
        Type::DCT256X128 => scaled(256, 128, coefficients, scratch_space),
        Type::DCT128X256 => scaled(128, 256, coefficients, scratch_space),
        _ => {}
    }
}

/// Original DCFromLowestFrequencies for one descriptor; the contract of
/// [`super::enc_transforms::dc_from_lowest_frequencies`].
pub fn dc_from_lowest_frequencies_d<D: SimdF32>(
    d: D,
    strategy: AcStrategyType,
    block: &[f32],
    dc: &mut [f32],
    dc_stride: usize,
    scratch_space: &mut [f32],
) {
    type Type = AcStrategyType;
    let mut idct = |rows: usize, cols: usize, input_stride: usize| {
        // DCT_ROWS = ROWS * kBlockDim and DCT_COLS = COLS * kBlockDim; the
        // low-frequency extent equals the block counts for every strategy.
        reinterpreting_idct(
            d,
            rows * BLOCK_DIM,
            cols * BLOCK_DIM,
            rows,
            cols,
            rows,
            cols,
            block,
            input_stride,
            dc,
            dc_stride,
            scratch_space,
        );
    };
    match strategy {
        Type::DCT16X8 => idct(2, 1, 2 * BLOCK_DIM),
        Type::DCT8X16 => idct(1, 2, 2 * BLOCK_DIM),
        Type::DCT16X16 => idct(2, 2, 2 * BLOCK_DIM),
        Type::DCT32X8 => idct(4, 1, 4 * BLOCK_DIM),
        Type::DCT8X32 => idct(1, 4, 4 * BLOCK_DIM),
        Type::DCT32X16 => idct(4, 2, 4 * BLOCK_DIM),
        Type::DCT16X32 => idct(2, 4, 4 * BLOCK_DIM),
        Type::DCT32X32 => idct(4, 4, 4 * BLOCK_DIM),
        Type::DCT64X32 => idct(8, 4, 8 * BLOCK_DIM),
        Type::DCT32X64 => idct(4, 8, 8 * BLOCK_DIM),
        Type::DCT64X64 => idct(8, 8, 8 * BLOCK_DIM),
        Type::DCT128X64 => idct(16, 8, 16 * BLOCK_DIM),
        Type::DCT64X128 => idct(8, 16, 16 * BLOCK_DIM),
        Type::DCT128X128 => idct(16, 16, 16 * BLOCK_DIM),
        Type::DCT256X128 => idct(32, 16, 32 * BLOCK_DIM),
        Type::DCT128X256 => idct(16, 32, 32 * BLOCK_DIM),
        Type::DCT256X256 => idct(32, 32, 32 * BLOCK_DIM),
        Type::DCT
        | Type::DCT2X2
        | Type::DCT4X4
        | Type::DCT4X8
        | Type::DCT8X4
        | Type::AFV0
        | Type::AFV1
        | Type::AFV2
        | Type::AFV3
        | Type::IDENTITY => dc[0] = block[0],
        _ => {}
    }
}

struct FromPixels<'a> {
    strategy: AcStrategyType,
    pixels: &'a [f32],
    pixels_stride: usize,
    coefficients: &'a mut [f32],
    scratch_space: &'a mut [f32],
}
impl Kernel for FromPixels<'_> {
    type Output = ();
    #[inline(always)]
    fn run<D: SimdF32>(self, d: D) {
        transform_from_pixels_d(
            d,
            self.strategy,
            self.pixels,
            self.pixels_stride,
            self.coefficients,
            self.scratch_space,
        );
    }
}

/// `TransformFromPixels` instantiated for one SIMD target.
pub fn transform_from_pixels_simd(
    target: Target,
    strategy: AcStrategyType,
    pixels: &[f32],
    pixels_stride: usize,
    coefficients: &mut [f32],
    scratch_space: &mut [f32],
) {
    dispatch(
        target,
        FromPixels {
            strategy,
            pixels,
            pixels_stride,
            coefficients,
            scratch_space,
        },
    );
}

struct FromLowest<'a> {
    strategy: AcStrategyType,
    block: &'a [f32],
    dc: &'a mut [f32],
    dc_stride: usize,
    scratch_space: &'a mut [f32],
}
impl Kernel for FromLowest<'_> {
    type Output = ();
    #[inline(always)]
    fn run<D: SimdF32>(self, d: D) {
        dc_from_lowest_frequencies_d(
            d,
            self.strategy,
            self.block,
            self.dc,
            self.dc_stride,
            self.scratch_space,
        );
    }
}

/// `DCFromLowestFrequencies` instantiated for one SIMD target.
pub fn dc_from_lowest_frequencies_simd(
    target: Target,
    strategy: AcStrategyType,
    block: &[f32],
    dc: &mut [f32],
    dc_stride: usize,
    scratch_space: &mut [f32],
) {
    dispatch(
        target,
        FromLowest {
            strategy,
            block,
            dc,
            dc_stride,
            scratch_space,
        },
    );
}

struct Afv<'a> {
    pixels: &'a [f32],
    coeffs: &'a mut [f32],
}
impl Kernel for Afv<'_> {
    type Output = ();
    #[inline(always)]
    fn run<D: SimdF32>(self, d: D) {
        afvdct4x4_d(d, self.pixels, self.coeffs);
    }
}

/// `AFVDCT4x4` instantiated for one SIMD target.
pub fn afvdct4x4_simd(target: Target, pixels: &[f32], coeffs: &mut [f32]) {
    dispatch(target, Afv { pixels, coeffs });
}
