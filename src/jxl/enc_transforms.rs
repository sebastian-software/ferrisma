// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.

//! Scalar forward transforms from lib/jxl/enc_transforms.{h,cc} and
//! lib/jxl/enc_transforms-inl.h: TransformFromPixels for all 27 strategies
//! (with DCT2TopBlock and AFVTransformFromPixels), DCFromLowestFrequencies
//! with ReinterpretingIDCT, and AFVDCT4x4. The scaled DCT/IDCT recursion is
//! the existing one-lane `dct` translation of dct-inl.h.
//!
//! Contraction: the pinned native AArch64 object fuses exactly two sites,
//! both in AFVTransformFromPixels' final DC mix: `block00 + block01 +
//! 2 * block10` becomes FMADD(block10, 2, block00 + block01), and the `- 2 *
//! block10` variant FMADD(block10, -2, block00 + block01). All other float
//! operations here are Highway scalar operations or separate statements.
//! Baseline x86-64 fuses nothing. Highway's scalar MulAdd in AFVDCT4x4 is a
//! `scalar_mul` product followed by a separate binary32 addition.
//!
//! The source's uninitialized local AFV scratch and DCT2TopBlock temporary are
//! zero-initialized here; the original reads none of those values before
//! writing them.
use super::ac_strategy::AcStrategyType;
use super::dct::{compute_scaled_dct, compute_scaled_idct, scalar_mul};
use super::dct_scales::total_resample_scale;
use super::frame_dimensions::{BLOCK_DIM, DCT_BLOCK_SIZE};

const MAX_BLOCKS: usize = 32;

/// Inverse of ReinterpretingDCT. `scratch` holds the 32x32 coefficient block
/// followed by the scaled IDCT's own scratch.
#[expect(
    clippy::too_many_arguments,
    reason = "Spell out the source's template arguments."
)]
fn reinterpreting_idct(
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

    compute_scaled_idct(
        rows,
        cols,
        &mut block[..rows * cols],
        output,
        output_stride,
        scratch_space,
    );
}

/// DCT2TopBlock<S>: reads all S x S inputs before writing `out`, so the
/// source's aliasing calls with `block == out` are the same operation.
fn dct2_top_block(s: usize, block: &[f32], stride: usize) -> [f32; DCT_BLOCK_SIZE] {
    assert!(
        BLOCK_DIM.is_multiple_of(s),
        "S should be a divisor of kBlockDim"
    );
    assert!(s.is_multiple_of(2), "S should be even");
    let mut temp = [0.0f32; DCT_BLOCK_SIZE];
    let num_2x2 = s / 2;
    for y in 0..num_2x2 {
        for x in 0..num_2x2 {
            let c00 = block[y * 2 * stride + x * 2];
            let c01 = block[y * 2 * stride + x * 2 + 1];
            let c10 = block[(y * 2 + 1) * stride + x * 2];
            let c11 = block[(y * 2 + 1) * stride + x * 2 + 1];
            let mut r00 = c00 + c01 + c10 + c11;
            let mut r01 = c00 + c01 - c10 - c11;
            let mut r10 = c00 - c01 + c10 - c11;
            let mut r11 = c00 - c01 - c10 + c11;
            r00 *= 0.25;
            r01 *= 0.25;
            r10 *= 0.25;
            r11 *= 0.25;
            temp[y * BLOCK_DIM + x] = r00;
            temp[y * BLOCK_DIM + num_2x2 + x] = r01;
            temp[(y + num_2x2) * BLOCK_DIM + x] = r10;
            temp[(y + num_2x2) * BLOCK_DIM + num_2x2 + x] = r11;
        }
    }
    temp
}
fn store_dct2_top_block(s: usize, temp: &[f32; DCT_BLOCK_SIZE], out: &mut [f32]) {
    for y in 0..s {
        for x in 0..s {
            out[y * BLOCK_DIM + x] = temp[y * BLOCK_DIM + x];
        }
    }
}

#[expect(
    clippy::excessive_precision,
    clippy::approx_constant,
    reason = "Keep the source's literal digits."
)]
#[rustfmt::skip]
const K4X4_AFV_BASIS_TRANSPOSE: [[f32; 16]; 16] = [
    [
        0.2500000000000000,
        0.8769029297991420,
        0.0000000000000000,
        0.0000000000000000,
        0.0000000000000000,
        -0.4105377591765233,
        0.0000000000000000,
        0.0000000000000000,
        0.0000000000000000,
        0.0000000000000000,
        0.0000000000000000,
        0.0000000000000000,
        0.0000000000000000,
        0.0000000000000000,
        0.0000000000000000,
        0.0000000000000000,
    ],
    [
        0.2500000000000000,
        0.2206518106944235,
        0.0000000000000000,
        0.0000000000000000,
        -0.7071067811865474,
        0.6235485373547691,
        0.0000000000000000,
        0.0000000000000000,
        0.0000000000000000,
        0.0000000000000000,
        0.0000000000000000,
        0.0000000000000000,
        0.0000000000000000,
        0.0000000000000000,
        0.0000000000000000,
        0.0000000000000000,
    ],
    [
        0.2500000000000000,
        -0.1014005039375376,
        0.4067007583026075,
        -0.2125574805828875,
        0.0000000000000000,
        -0.0643507165794627,
        -0.4517556589999482,
        -0.3046847507248690,
        0.3017929516615495,
        0.4082482904638627,
        0.1747866975480809,
        -0.2110560104933578,
        -0.1426608480880726,
        -0.1381354035075859,
        -0.1743760259965107,
        0.1135498731499434,
    ],
    [
        0.2500000000000000,
        -0.1014005039375375,
        0.4444481661973445,
        0.3085497062849767,
        0.0000000000000000,
        -0.0643507165794627,
        0.1585450355184006,
        0.5112616136591823,
        0.2579236279634118,
        0.0000000000000000,
        0.0812611176717539,
        0.1856718091610980,
        -0.3416446842253372,
        0.3302282550303788,
        0.0702790691196284,
        -0.0741750459581035,
    ],
    [
        0.2500000000000000,
        0.2206518106944236,
        0.0000000000000000,
        0.0000000000000000,
        0.7071067811865476,
        0.6235485373547694,
        0.0000000000000000,
        0.0000000000000000,
        0.0000000000000000,
        0.0000000000000000,
        0.0000000000000000,
        0.0000000000000000,
        0.0000000000000000,
        0.0000000000000000,
        0.0000000000000000,
        0.0000000000000000,
    ],
    [
        0.2500000000000000,
        -0.1014005039375378,
        0.0000000000000000,
        0.4706702258572536,
        0.0000000000000000,
        -0.0643507165794628,
        -0.0403851516082220,
        0.0000000000000000,
        0.1627234014286620,
        0.0000000000000000,
        0.0000000000000000,
        0.0000000000000000,
        0.7367497537172237,
        0.0875511500058708,
        -0.2921026642334881,
        0.1940289303259434,
    ],
    [
        0.2500000000000000,
        -0.1014005039375377,
        0.1957439937204294,
        -0.1621205195722993,
        0.0000000000000000,
        -0.0643507165794628,
        0.0074182263792424,
        -0.2904801297289980,
        0.0952002265347504,
        0.0000000000000000,
        -0.3675398009862027,
        0.4921585901373873,
        0.2462710772207515,
        -0.0794670660590957,
        0.3623817333531167,
        -0.4351904965232280,
    ],
    [
        0.2500000000000000,
        -0.1014005039375376,
        0.2929100136981264,
        0.0000000000000000,
        0.0000000000000000,
        -0.0643507165794627,
        0.3935103426921017,
        -0.0657870154914280,
        0.0000000000000000,
        -0.4082482904638628,
        -0.3078822139579090,
        -0.3852501370925192,
        -0.0857401903551931,
        -0.4613374887461511,
        0.0000000000000000,
        0.2191868483885747,
    ],
    [
        0.2500000000000000,
        -0.1014005039375376,
        -0.4067007583026072,
        -0.2125574805828705,
        0.0000000000000000,
        -0.0643507165794627,
        -0.4517556589999464,
        0.3046847507248840,
        0.3017929516615503,
        -0.4082482904638635,
        -0.1747866975480813,
        0.2110560104933581,
        -0.1426608480880734,
        -0.1381354035075829,
        -0.1743760259965108,
        0.1135498731499426,
    ],
    [
        0.2500000000000000,
        -0.1014005039375377,
        -0.1957439937204287,
        -0.1621205195722833,
        0.0000000000000000,
        -0.0643507165794628,
        0.0074182263792444,
        0.2904801297290076,
        0.0952002265347505,
        0.0000000000000000,
        0.3675398009862011,
        -0.4921585901373891,
        0.2462710772207514,
        -0.0794670660591026,
        0.3623817333531165,
        -0.4351904965232251,
    ],
    [
        0.2500000000000000,
        -0.1014005039375375,
        0.0000000000000000,
        -0.4706702258572528,
        0.0000000000000000,
        -0.0643507165794627,
        0.1107416575309343,
        0.0000000000000000,
        -0.1627234014286617,
        0.0000000000000000,
        0.0000000000000000,
        0.0000000000000000,
        0.1488339922711357,
        0.4972464710953509,
        0.2921026642334879,
        0.5550443808910661,
    ],
    [
        0.2500000000000000,
        -0.1014005039375377,
        0.1137907446044809,
        -0.1464291867126764,
        0.0000000000000000,
        -0.0643507165794628,
        0.0829816309488205,
        -0.2388977352334460,
        -0.3531238544981630,
        -0.4082482904638630,
        0.4826689115059883,
        0.1741941265991622,
        -0.0476868035022925,
        0.1253805944856366,
        -0.4326608024727445,
        -0.2546827712406646,
    ],
    [
        0.2500000000000000,
        -0.1014005039375377,
        -0.4444481661973438,
        0.3085497062849487,
        0.0000000000000000,
        -0.0643507165794628,
        0.1585450355183970,
        -0.5112616136592012,
        0.2579236279634129,
        0.0000000000000000,
        -0.0812611176717504,
        -0.1856718091610990,
        -0.3416446842253373,
        0.3302282550303805,
        0.0702790691196282,
        -0.0741750459581023,
    ],
    [
        0.2500000000000000,
        -0.1014005039375376,
        -0.2929100136981264,
        0.0000000000000000,
        0.0000000000000000,
        -0.0643507165794627,
        0.3935103426921022,
        0.0657870154914254,
        0.0000000000000000,
        0.4082482904638634,
        0.3078822139579031,
        0.3852501370925211,
        -0.0857401903551927,
        -0.4613374887461554,
        0.0000000000000000,
        0.2191868483885728,
    ],
    [
        0.2500000000000000,
        -0.1014005039375376,
        -0.1137907446044814,
        -0.1464291867126654,
        0.0000000000000000,
        -0.0643507165794627,
        0.0829816309488214,
        0.2388977352334547,
        -0.3531238544981624,
        0.4082482904638630,
        -0.4826689115059858,
        -0.1741941265991621,
        -0.0476868035022928,
        0.1253805944856431,
        -0.4326608024727457,
        -0.2546827712406641,
    ],
    [
        0.2500000000000000,
        -0.1014005039375374,
        0.0000000000000000,
        0.4251149611657548,
        0.0000000000000000,
        -0.0643507165794626,
        -0.4517556589999480,
        0.0000000000000000,
        -0.6035859033230976,
        0.0000000000000000,
        0.0000000000000000,
        0.0000000000000000,
        -0.1426608480880724,
        -0.1381354035075845,
        0.3487520519930227,
        0.1135498731499429,
    ],
];

// MulAdd(px, basis, scalar)'s binary32 addition. Keep the accumulator as the
// first operand out of line: when it and the product hold different NaNs, the
// stable reference instantiations select the accumulator's on native x86-64
// (SSE keeps the first source) and on AArch64.
#[inline(never)]
fn afv_accumulate(scalar: f32, product: f32) -> f32 {
    scalar + product
}

/// Original AFVDCT4x4 with one Highway lane: each coefficient accumulates
/// MulAdd(pixel, basis, scalar) over the 16 pixels in order.
pub fn afvdct4x4(pixels: &[f32], coeffs: &mut [f32]) {
    // HWY_CAPPED(float, 16) has one lane on the scalar target.
    const LANES: usize = 1;
    let mut i = 0;
    while i < 16 {
        let mut scalar = 0.0f32;
        for j in 0..16 {
            let px = pixels[j];
            let basis = K4X4_AFV_BASIS_TRANSPOSE[j][i];
            scalar = afv_accumulate(scalar, scalar_mul(px, basis));
        }
        coeffs[i] = scalar;
        i += LANES;
    }
}

// AFVTransformFromPixels' final DC mix with the native objects' contraction.
#[cfg(target_arch = "aarch64")]
fn afv_dc_mix(sum: f32, block10: f32, scale: f32) -> f32 {
    block10.mul_add(scale, sum)
}
#[cfg(not(target_arch = "aarch64"))]
fn afv_dc_mix(sum: f32, block10: f32, scale: f32) -> f32 {
    sum + scale * block10
}

// Coefficient layout:
//  - (even, even) positions hold AFV coefficients
//  - (odd, even) positions hold DCT4x4 coefficients
//  - (any, odd) positions hold DCT4x8 coefficients
fn afv_transform_from_pixels(
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
    afvdct4x4(&block, &mut coeff);
    for iy in 0..4 {
        for ix in 0..4 {
            coefficients[iy * 2 * 8 + ix * 2] = coeff[iy * 4 + ix];
        }
    }
    // 4x4 DCT of the block with same y and different x.
    compute_scaled_dct(
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
    compute_scaled_dct(
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

/// Original TransformFromPixels. `pixels` is read with `pixels_stride`;
/// `coefficients` and `scratch_space` are separate owners, as the source's
/// restrict arguments require. An unknown strategy value writes nothing.
pub fn transform_from_pixels(
    strategy: AcStrategyType,
    pixels: &[f32],
    pixels_stride: usize,
    coefficients: &mut [f32],
    scratch_space: &mut [f32],
) {
    type Type = AcStrategyType;
    let scaled = |rows: usize, cols: usize, coefficients: &mut [f32], scratch: &mut [f32]| {
        compute_scaled_dct(rows, cols, pixels, pixels_stride, coefficients, scratch);
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
                compute_scaled_dct(
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
                compute_scaled_dct(
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
                    compute_scaled_dct(
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
        Type::AFV0 => afv_transform_from_pixels(0, pixels, pixels_stride, coefficients),
        Type::AFV1 => afv_transform_from_pixels(1, pixels, pixels_stride, coefficients),
        Type::AFV2 => afv_transform_from_pixels(2, pixels, pixels_stride, coefficients),
        Type::AFV3 => afv_transform_from_pixels(3, pixels, pixels_stride, coefficients),
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

/// Original DCFromLowestFrequencies. `scratch_space` should be at least
/// 4 * kMaxBlocks * kMaxBlocks elements. An unknown strategy value writes
/// nothing.
pub fn dc_from_lowest_frequencies(
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
