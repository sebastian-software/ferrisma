// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.

//! Scalar inverse transform dispatch and DC reinterpretation from
//! lib/jxl/dec_transforms-inl.h. All 27 original strategies retain their
//! coefficient placement and the source's mutable-coefficient contract.
use super::ac_strategy::{AcStrategy, AcStrategyType};
use super::dct::{compute_scaled_dct, compute_scaled_idct, scalar_mul};
use super::dct_scales::total_resample_scale;

#[allow(clippy::excessive_precision, clippy::approx_constant)]
const AFV_BASIS: [[f32; 16]; 16] = [
    [
        0.25, 0.25, 0.25, 0.25, 0.25, 0.25, 0.25, 0.25, 0.25, 0.25, 0.25, 0.25, 0.25, 0.25, 0.25,
        0.25,
    ],
    [
        0.876902929799142,
        0.2206518106944235,
        -0.10140050393753763,
        -0.1014005039375375,
        0.2206518106944236,
        -0.10140050393753777,
        -0.10140050393753772,
        -0.10140050393753763,
        -0.10140050393753758,
        -0.10140050393753769,
        -0.1014005039375375,
        -0.10140050393753768,
        -0.10140050393753768,
        -0.10140050393753759,
        -0.10140050393753763,
        -0.10140050393753741,
    ],
    [
        0.0,
        0.0,
        0.40670075830260755,
        0.44444816619734445,
        0.0,
        0.0,
        0.19574399372042936,
        0.2929100136981264,
        -0.40670075830260716,
        -0.19574399372042872,
        0.0,
        0.11379074460448091,
        -0.44444816619734384,
        -0.29291001369812636,
        -0.1137907446044814,
        0.0,
    ],
    [
        0.0,
        0.0,
        -0.21255748058288748,
        0.3085497062849767,
        0.0,
        0.4706702258572536,
        -0.1621205195722993,
        0.0,
        -0.21255748058287047,
        -0.16212051957228327,
        -0.47067022585725277,
        -0.1464291867126764,
        0.3085497062849487,
        0.0,
        -0.14642918671266536,
        0.4251149611657548,
    ],
    [
        0.0,
        -0.7071067811865474,
        0.0,
        0.0,
        0.7071067811865476,
        0.0,
        0.0,
        0.0,
        0.0,
        0.0,
        0.0,
        0.0,
        0.0,
        0.0,
        0.0,
        0.0,
    ],
    [
        -0.4105377591765233,
        0.6235485373547691,
        -0.06435071657946274,
        -0.06435071657946266,
        0.6235485373547694,
        -0.06435071657946284,
        -0.0643507165794628,
        -0.06435071657946274,
        -0.06435071657946272,
        -0.06435071657946279,
        -0.06435071657946266,
        -0.06435071657946277,
        -0.06435071657946277,
        -0.06435071657946273,
        -0.06435071657946274,
        -0.0643507165794626,
    ],
    [
        0.0,
        0.0,
        -0.4517556589999482,
        0.15854503551840063,
        0.0,
        -0.04038515160822202,
        0.0074182263792423875,
        0.39351034269210167,
        -0.45175565899994635,
        0.007418226379244351,
        0.1107416575309343,
        0.08298163094882051,
        0.15854503551839705,
        0.3935103426921022,
        0.0829816309488214,
        -0.45175565899994796,
    ],
    [
        0.0,
        0.0,
        -0.304684750724869,
        0.5112616136591823,
        0.0,
        0.0,
        -0.290480129728998,
        -0.06578701549142804,
        0.304684750724884,
        0.2904801297290076,
        0.0,
        -0.23889773523344604,
        -0.5112616136592012,
        0.06578701549142545,
        0.23889773523345467,
        0.0,
    ],
    [
        0.0,
        0.0,
        0.3017929516615495,
        0.25792362796341184,
        0.0,
        0.16272340142866204,
        0.09520022653475037,
        0.0,
        0.3017929516615503,
        0.09520022653475055,
        -0.16272340142866173,
        -0.35312385449816297,
        0.25792362796341295,
        0.0,
        -0.3531238544981624,
        -0.6035859033230976,
    ],
    [
        0.0,
        0.0,
        0.40824829046386274,
        0.0,
        0.0,
        0.0,
        0.0,
        -0.4082482904638628,
        -0.4082482904638635,
        0.0,
        0.0,
        -0.40824829046386296,
        0.0,
        0.4082482904638634,
        0.408248290463863,
        0.0,
    ],
    [
        0.0,
        0.0,
        0.1747866975480809,
        0.0812611176717539,
        0.0,
        0.0,
        -0.3675398009862027,
        -0.307882213957909,
        -0.17478669754808135,
        0.3675398009862011,
        0.0,
        0.4826689115059883,
        -0.08126111767175039,
        0.30788221395790305,
        -0.48266891150598584,
        0.0,
    ],
    [
        0.0,
        0.0,
        -0.21105601049335784,
        0.18567180916109802,
        0.0,
        0.0,
        0.49215859013738733,
        -0.38525013709251915,
        0.21105601049335806,
        -0.49215859013738905,
        0.0,
        0.17419412659916217,
        -0.18567180916109904,
        0.3852501370925211,
        -0.1741941265991621,
        0.0,
    ],
    [
        0.0,
        0.0,
        -0.14266084808807264,
        -0.3416446842253372,
        0.0,
        0.7367497537172237,
        0.24627107722075148,
        -0.08574019035519306,
        -0.14266084808807344,
        0.24627107722075137,
        0.14883399227113567,
        -0.04768680350229251,
        -0.3416446842253373,
        -0.08574019035519267,
        -0.047686803502292804,
        -0.14266084808807242,
    ],
    [
        0.0,
        0.0,
        -0.13813540350758585,
        0.3302282550303788,
        0.0,
        0.08755115000587084,
        -0.07946706605909573,
        -0.4613374887461511,
        -0.13813540350758294,
        -0.07946706605910261,
        0.49724647109535086,
        0.12538059448563663,
        0.3302282550303805,
        -0.4613374887461554,
        0.12538059448564315,
        -0.13813540350758452,
    ],
    [
        0.0,
        0.0,
        -0.17437602599651067,
        0.0702790691196284,
        0.0,
        -0.2921026642334881,
        0.3623817333531167,
        0.0,
        -0.1743760259965108,
        0.36238173335311646,
        0.29210266423348785,
        -0.4326608024727445,
        0.07027906911962818,
        0.0,
        -0.4326608024727457,
        0.34875205199302267,
    ],
    [
        0.0,
        0.0,
        0.11354987314994337,
        -0.07417504595810355,
        0.0,
        0.19402893032594343,
        -0.435190496523228,
        0.21918684838857466,
        0.11354987314994257,
        -0.4351904965232251,
        0.5550443808910661,
        -0.25468277124066463,
        -0.07417504595810233,
        0.2191868483885728,
        -0.25468277124066413,
        0.1135498731499429,
    ],
];
// Preserve the selected original scalar AFV object's addition operands.
#[inline(never)]
fn afv_add(first: f32, second: f32) -> f32 {
    first + second
}
fn afv_idct4x4(coeffs: &[f32], pixels: &mut [f32]) {
    for i in 0..16 {
        let mut pixel = 0.0f32;
        for j in 0..16 {
            let product = scalar_mul(coeffs[j], AFV_BASIS[j][i]);
            #[cfg(target_arch = "aarch64")]
            {
                pixel = afv_add(pixel, product);
            }
            #[cfg(not(target_arch = "aarch64"))]
            {
                pixel = afv_add(product, pixel);
            }
        }
        pixels[i] = pixel;
    }
}
fn afv_transform_to_pixels(kind: usize, coefficients: &[f32], pixels: &mut [f32], stride: usize) {
    let mut scratch = [0.0; 4 * 8 * 4];
    let ax = kind & 1;
    let ay = kind / 2;
    let a = coefficients[0];
    let b = coefficients[1];
    let c = coefficients[8];
    #[cfg(target_arch = "aarch64")]
    let dc0 = afv_add(b, a + c) * 4.0;
    #[cfg(not(target_arch = "aarch64"))]
    let dc0 = (a + c + b) * 4.0;
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
    afv_idct4x4(&coeff, &mut block);
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
    compute_scaled_idct(
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
    compute_scaled_idct(
        4,
        8,
        &mut block,
        &mut pixels[(if ay == 1 { 0 } else { 4 }) * stride..],
        stride,
        &mut scratch,
    );
}
// Retain the original identity pixel addition's coefficient-first order when
// both it and the DC residual contain different NaN payloads.
#[inline(never)]
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

/// Original TransformToPixels. The output and scratch are separate owners;
/// coefficients are mutated by the DCT branches exactly as in the source.
pub fn transform_to_pixels(
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
            let d = coefficients[9];
            let dcs = [a + b + c + d, a + b - c - d, a - b + c - d, a - b - c + d];
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
                    // The fixed original AArch64 loop contracts to FMADD
                    // with a negative finite scale, retaining fused rounding
                    // without negating the residual's NaN sign.
                    #[cfg(target_arch = "aarch64")]
                    let base = sum.mul_add(-1.0 / 16.0, dc);
                    #[cfg(not(target_arch = "aarch64"))]
                    let base = dc - sum * (1.0 / 16.0);
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
                    compute_scaled_idct(8, 4, &mut block, &mut pixels[i * 4..], stride, scratch);
                } else {
                    compute_scaled_idct(
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
            let d = coefficients[9];
            let dcs = [a + b + c + d, a + b - c - d, a - b + c - d, a - b - c + d];
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
                    compute_scaled_idct(
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
            (kind.0 - AcStrategyType::AFV0.0) as usize,
            coefficients,
            pixels,
            stride,
        ),
        _ => compute_scaled_idct(
            acs.covered_blocks_y() * 8,
            acs.covered_blocks_x() * 8,
            coefficients,
            pixels,
            stride,
            scratch,
        ),
    }
}

/// Original LowestFrequenciesFromDC, including its local warm-block branch and
/// sparse writes into the caller's full coefficient block. Other entries remain
/// unchanged; scratch belongs to the caller for transforms larger than 32x32.
pub fn lowest_frequencies_from_dc(
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
    compute_scaled_dct(rows, cols, dc, dc_stride, block, tmp);
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
